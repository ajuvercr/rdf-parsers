use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::ops::Range;
use std::pin::Pin;

use rowan::SyntaxNode;

use crate::Spanned;
use crate::model::*;

use super::parser::{Lang, SyntaxKind};

type Node = SyntaxNode<Lang>;

// RDF/XSD constants
const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const RDF_FIRST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#first";
const RDF_REST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#rest";
const RDF_NIL: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#nil";
const RDF_JSON: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#JSON";
const XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";
const XSD_INTEGER: &str = "http://www.w3.org/2001/XMLSchema#integer";
const XSD_DOUBLE: &str = "http://www.w3.org/2001/XMLSchema#double";
const XSD_BOOLEAN: &str = "http://www.w3.org/2001/XMLSchema#boolean";

// ── Intermediate JSON-LD value representation ─────────────────────────────────

#[derive(Debug, Clone)]
pub enum JsonLdVal {
    /// Object carries its span so we can pass it as the triple span for nodes.
    Object(Vec<(String, JsonLdVal)>, Range<usize>),
    Array(Vec<JsonLdVal>),
    Str(String),
    Number(String),
    Bool(bool),
    Null,
}

// ── Context types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
struct TermDefinition {
    iri: Option<String>,
    type_mapping: Option<String>,
    language: Option<String>,
    container: Option<String>,
    reverse: bool,
}

#[derive(Debug, Clone, Default)]
struct ActiveContext {
    base: Option<String>,
    vocab: Option<String>,
    language: Option<String>,
    terms: HashMap<String, TermDefinition>,
}

// ── Context loader hook ───────────────────────────────────────────────────────

/// Hook for fetching remote JSON-LD context documents asynchronously.
///
/// Implement this trait to support `@context` values that are URLs.  The
/// `load` method receives the URL and should return the raw JSON-LD document
/// content as a string, or `None` if the document cannot be fetched.
///
/// # Example (sync wrapper)
/// ```rust,ignore
/// struct MyLoader;
/// impl ContextLoader for MyLoader {
///     fn load<'a>(&'a self, url: &'a str) -> Pin<Box<dyn Future<Output = Option<String>> + 'a>> {
///         Box::pin(async move { Some(fetch_remote(url).await) })
///     }
/// }
/// ```
pub trait ContextLoader {
    fn load<'a>(&'a mut self, url: &'a str) -> Pin<Box<dyn Future<Output = Option<String>> + 'a>>;
}

/// A no-op `ContextLoader` that never resolves remote contexts.
pub struct NoopContextLoader;

impl ContextLoader for NoopContextLoader {
    fn load<'a>(&'a mut self, _url: &'a str) -> Pin<Box<dyn Future<Output = Option<String>> + 'a>> {
        Box::pin(std::future::ready(None))
    }
}

// ── Convert state ─────────────────────────────────────────────────────────────

struct ConvertState {
    blank_counter: usize,
    triples: Vec<Spanned<Triple>>,
}

impl ConvertState {
    fn new() -> Self {
        Self {
            blank_counter: 0,
            triples: Vec::new(),
        }
    }

    fn fresh_blank(&mut self, offset: usize) -> Term {
        let id = self.blank_counter;
        self.blank_counter += 1;
        Term::BlankNode(BlankNode::Named(format!("b{}", id), offset))
    }

    fn add_triple(
        &mut self,
        subject: Term,
        predicate: Term,
        object: Term,
        graph: Option<Term>,
        span: Range<usize>,
    ) {
        let s = span.clone();
        self.triples.push(Spanned(
            Triple {
                subject: Spanned(subject, s.clone()),
                po: vec![Spanned(
                    PO {
                        predicate: Spanned(predicate, s.clone()),
                        object: vec![Spanned(object, s.clone())],
                    },
                    s.clone(),
                )],
                graph: graph.map(|g| Spanned(g, s.clone())),
            },
            s,
        ));
    }

    fn named(iri: &str) -> Term {
        Term::NamedNode(NamedNode::Full(iri.to_string(), 0))
    }

    fn into_turtle(self) -> Turtle {
        Turtle::new(None, Vec::new(), self.triples)
    }
}

// ── CST helpers ───────────────────────────────────────────────────────────────

fn text_range(node: &Node) -> Range<usize> {
    let r = node.text_range();
    r.start().into()..r.end().into()
}

fn child(node: &Node, kind: SyntaxKind) -> Option<Node> {
    node.children().find(|c| c.kind() == kind)
}

fn children(node: &Node, kind: SyntaxKind) -> impl Iterator<Item = Node> {
    node.children().filter(move |c| c.kind() == kind)
}

/// Extract the unquoted, unescaped string content from a `JsonString` node.
///
/// The A* parser wraps every matched terminal in a terminal wrapper node of the
/// same `SyntaxKind`.  So a `JsonString` producing-rule node contains one child
/// node (e.g., `AtId` or `StringToken`), which in turn contains the raw token.
/// This function peeks through that extra level to retrieve the token text.
fn extract_json_string(node: &Node) -> Option<String> {
    // `JsonString` has one child node: the terminal wrapper (AtId, StringToken, …).
    let terminal_wrapper = node.children().next()?;
    let tok = terminal_wrapper
        .children_with_tokens()
        .filter_map(|e| e.into_token())
        .find(|t| !matches!(t.kind(), SyntaxKind::WhiteSpace | SyntaxKind::Comment))?;
    let text = tok.text();
    if text.starts_with('"') && text.ends_with('"') && text.len() >= 2 {
        Some(unescape_json_string(&text[1..text.len() - 1]))
    } else {
        None
    }
}

/// Extract the text of a token from inside a terminal wrapper node.
fn terminal_text(wrapper: &Node) -> Option<String> {
    wrapper
        .children_with_tokens()
        .filter_map(|e| e.into_token())
        .find(|t| !matches!(t.kind(), SyntaxKind::WhiteSpace | SyntaxKind::Comment))
        .map(|t| t.text().to_string())
}

fn unescape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some('/') => result.push('/'),
                Some('b') => result.push('\x08'),
                Some('f') => result.push('\x0C'),
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('u') => {
                    let hex: String = chars.by_ref().take(4).collect();
                    if let Ok(n) = u32::from_str_radix(&hex, 16) {
                        if let Some(c) = char::from_u32(n) {
                            result.push(c);
                        }
                    }
                }
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    result
}

// ── CST → JsonLdVal ───────────────────────────────────────────────────────────

fn cst_to_value(node: &Node) -> Option<JsonLdVal> {
    let span = text_range(node);
    match node.kind() {
        SyntaxKind::JsonValue => {
            // Non-terminal alternatives: JsonObject, JsonArray, JsonString.
            if let Some(obj) = child(node, SyntaxKind::JsonObject) {
                return cst_to_value(&obj);
            }
            if let Some(arr) = child(node, SyntaxKind::JsonArray) {
                return cst_to_value(&arr);
            }
            if let Some(js) = child(node, SyntaxKind::JsonString) {
                return cst_to_value(&js);
            }
            // Terminal alternatives: JSON_NUMBER, 'true', 'false', 'null'.
            // The A* wraps each matched terminal in a terminal wrapper node of the
            // same SyntaxKind, so they appear as child *nodes* of JsonValue.
            if let Some(num_node) = child(node, SyntaxKind::JsonNumber) {
                if let Some(text) = terminal_text(&num_node) {
                    return Some(JsonLdVal::Number(text));
                }
            }
            if child(node, SyntaxKind::TrueLit).is_some() {
                return Some(JsonLdVal::Bool(true));
            }
            if child(node, SyntaxKind::FalseLit).is_some() {
                return Some(JsonLdVal::Bool(false));
            }
            if child(node, SyntaxKind::NullLit).is_some() {
                return Some(JsonLdVal::Null);
            }
            None
        }
        SyntaxKind::JsonObject => {
            let mut members = Vec::new();
            // JsonObject ::= '{' memberList? '}'
            // Members live inside a MemberList child, not directly in JsonObject.
            let member_nodes: Vec<Node> = if let Some(ml) = child(node, SyntaxKind::MemberList) {
                children(&ml, SyntaxKind::Member).collect()
            } else {
                Vec::new()
            };
            for member in member_nodes {
                let Some(key_node) = child(&member, SyntaxKind::JsonString) else {
                    continue;
                };
                let Some(key) = extract_json_string(&key_node) else {
                    continue;
                };
                let Some(val_node) = child(&member, SyntaxKind::JsonValue) else {
                    continue;
                };
                let Some(val) = cst_to_value(&val_node) else {
                    continue;
                };
                members.push((key, val));
            }
            Some(JsonLdVal::Object(members, span))
        }
        SyntaxKind::JsonArray => {
            let items = if let Some(vl) = child(node, SyntaxKind::ValueList) {
                children(&vl, SyntaxKind::JsonValue)
                    .filter_map(|v| cst_to_value(&v))
                    .collect()
            } else {
                Vec::new()
            };
            Some(JsonLdVal::Array(items))
        }
        SyntaxKind::JsonString => {
            let s = extract_json_string(node)?;
            Some(JsonLdVal::Str(s))
        }
        _ => None,
    }
}

// ── Context processing ────────────────────────────────────────────────────────

thread_local! {
    /// Cache for parsed remote JSON-LD context documents (keyed by URL).
    /// Avoids re-parsing large contexts (e.g. schema.org, ~300KB) on every
    /// incremental re-parse.  Persists for the thread/session lifetime.
    static PARSED_CTX_CACHE: RefCell<HashMap<String, JsonLdVal>> = RefCell::new(HashMap::new());
}

/// Clear the parsed-context cache.  Call this if a remote context document
/// has been updated and the stale cached version should be discarded.
pub fn clear_parsed_context_cache() {
    PARSED_CTX_CACHE.with(|c| c.borrow_mut().clear());
}

/// Process a JSON-LD `@context` value and return the updated active context.
///
/// This is a boxed future because it is recursively called for arrays of
/// contexts and for remote contexts loaded via the `loader`.
fn process_context<'a>(
    mut active: ActiveContext,
    ctx_val: JsonLdVal,
    loader: &'a mut dyn ContextLoader,
) -> Pin<Box<dyn Future<Output = ActiveContext> + 'a>> {
    Box::pin(async move {
        match ctx_val {
            JsonLdVal::Null => {
                active = ActiveContext::default();
            }
            JsonLdVal::Str(ref url) => {
                // Check thread-local parsed-context cache first.
                let cached = PARSED_CTX_CACHE.with(|c| c.borrow().get(url).cloned());
                if let Some(remote_ctx) = cached {
                    active = process_context(active, remote_ctx, loader).await;
                } else if let Some(content) = loader.load(url).await {
                    if let Some(remote_ctx) = parse_jsonld_for_context(&content) {
                        PARSED_CTX_CACHE.with(|c| {
                            c.borrow_mut().insert(url.clone(), remote_ctx.clone())
                        });
                        active = process_context(active, remote_ctx, loader).await;
                    }
                }
            }
            JsonLdVal::Array(items) => {
                for item in items {
                    active = process_context(active, item, loader).await;
                }
            }
            JsonLdVal::Object(members, _) => {
                for (key, val) in members {
                    match key.as_str() {
                        "@base" => match &val {
                            JsonLdVal::Str(s) if s.is_empty() => active.base = None,
                            JsonLdVal::Str(s) => active.base = Some(resolve_iri(&active.base, s)),
                            JsonLdVal::Null => active.base = None,
                            _ => {}
                        },
                        "@vocab" => match val {
                            JsonLdVal::Str(s) => active.vocab = Some(s),
                            JsonLdVal::Null => active.vocab = None,
                            _ => {}
                        },
                        "@language" => match val {
                            JsonLdVal::Str(s) => active.language = Some(s),
                            JsonLdVal::Null => active.language = None,
                            _ => {}
                        },
                        // TODO: @version, @import, @propagate, @protected, @direction
                        k if !k.starts_with('@') => {
                            if let Some(def) = process_term_definition(&active, k, &val) {
                                active.terms.insert(k.to_string(), def);
                            } else {
                                active.terms.remove(k);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        active
    })
}

fn process_term_definition(
    active: &ActiveContext,
    term: &str,
    val: &JsonLdVal,
) -> Option<TermDefinition> {
    match val {
        JsonLdVal::Null => None,
        JsonLdVal::Str(s) => {
            let iri = expand_iri(active, s, true, false);
            Some(TermDefinition {
                iri,
                ..Default::default()
            })
        }
        JsonLdVal::Object(members, _) => {
            let mut def = TermDefinition::default();
            let map: HashMap<&str, &JsonLdVal> =
                members.iter().map(|(k, v)| (k.as_str(), v)).collect();

            // @reverse takes priority over @id for the IRI
            if let Some(rev_val) = map.get("@reverse") {
                if let JsonLdVal::Str(s) = rev_val {
                    def.iri = expand_iri(active, s, true, false);
                    def.reverse = true;
                }
            } else if let Some(id_val) = map.get("@id") {
                match id_val {
                    JsonLdVal::Str(s) => def.iri = expand_iri(active, s, true, false),
                    JsonLdVal::Null => def.iri = None,
                    _ => {}
                }
            } else {
                def.iri = expand_iri(active, term, true, false);
            }

            if let Some(JsonLdVal::Str(s)) = map.get("@type") {
                def.type_mapping = Some(s.clone());
            }
            if let Some(lang_val) = map.get("@language") {
                match lang_val {
                    JsonLdVal::Str(s) => def.language = Some(s.clone()),
                    JsonLdVal::Null => def.language = None,
                    _ => {}
                }
            }
            if let Some(JsonLdVal::Str(s)) = map.get("@container") {
                def.container = Some(s.clone());
            }

            Some(def)
        }
        _ => Some(TermDefinition {
            iri: None,
            ..Default::default()
        }),
    }
}

/// Parse a JSON-LD string (fetched from a remote URL) and return the value
/// that should be used as a context.  If the document has a top-level
/// `@context`, return that; otherwise return the whole document.
fn parse_jsonld_for_context(content: &str) -> Option<JsonLdVal> {
    use super::parser::{Rule, SyntaxKind as SK};
    let rule = Rule::new(SK::JsonldDoc);
    let (parse, _) = crate::parse(rule, content);
    let root = parse.syntax::<Lang>();
    // jsonldDoc ::= jsonValue; the JsonValue is a direct child of ROOT.
    let json_val_node = root.children().find(|c| c.kind() == SK::JsonValue)?;
    let val = cst_to_value(&json_val_node)?;
    // If the document contains @context at the top level, return that.
    if let JsonLdVal::Object(members, _) = &val {
        if let Some((_, ctx)) = members.iter().find(|(k, _)| k == "@context") {
            return Some(ctx.clone());
        }
    }
    Some(val)
}

// ── IRI expansion ─────────────────────────────────────────────────────────────

fn expand_iri(
    active: &ActiveContext,
    value: &str,
    vocab: bool,
    document_relative: bool,
) -> Option<String> {
    if value.is_empty() {
        if document_relative {
            return active.base.clone();
        }
        return None;
    }

    // Keywords pass through as-is
    if value.starts_with('@') {
        return Some(value.to_string());
    }

    // Term definition lookup (vocab=true: term names expand via active context)
    if vocab {
        if let Some(def) = active.terms.get(value) {
            if let Some(iri) = &def.iri {
                return Some(iri.clone());
            }
        }
    }

    // Compact IRI: contains ':' → split into prefix:suffix
    if let Some(pos) = value.find(':') {
        if pos > 0 {
            let prefix = &value[..pos];
            let suffix = &value[pos + 1..];

            // Already absolute (has scheme like http://) or blank node
            if suffix.starts_with("//") || prefix == "_" {
                return Some(value.to_string());
            }

            // Look up prefix in the active context
            if let Some(def) = active.terms.get(prefix) {
                if let Some(iri) = &def.iri {
                    return Some(format!("{}{}", iri, suffix));
                }
            }

            // Unknown prefix — treat as an absolute IRI
            return Some(value.to_string());
        }
    }

    // Vocab-relative expansion
    if vocab {
        if let Some(vocab_iri) = &active.vocab {
            return Some(format!("{}{}", vocab_iri, value));
        }
    }

    // Document-relative resolution
    if document_relative {
        if let Some(base) = &active.base {
            return Some(resolve_iri(&Some(base.clone()), value));
        }
    }

    Some(value.to_string())
}

fn resolve_iri(base: &Option<String>, reference: &str) -> String {
    if reference.contains("://") || reference.starts_with("urn:") {
        return reference.to_string();
    }
    match base {
        None => reference.to_string(),
        Some(base_iri) => {
            if reference.starts_with('/') {
                // Absolute path — combine with scheme+authority
                if let Some(auth_end) = authority_end(base_iri) {
                    format!("{}{}", &base_iri[..auth_end], reference)
                } else {
                    reference.to_string()
                }
            } else if reference.is_empty() {
                base_iri.clone()
            } else {
                // Relative — append to the directory part of base
                let dir = base_iri
                    .rfind('/')
                    .map(|i| &base_iri[..=i])
                    .unwrap_or(base_iri);
                format!("{}{}", dir, reference)
            }
        }
    }
}

fn authority_end(iri: &str) -> Option<usize> {
    let after = iri.find("://")?;
    let start = after + 3;
    let path_start = iri[start..]
        .find('/')
        .map(|i| start + i)
        .unwrap_or(iri.len());
    Some(path_start)
}

// ── Document processing ───────────────────────────────────────────────────────

/// Top-level: process one JSON-LD value (document or item in a top-level array).
///
/// Boxed because it is directly self-recursive for top-level arrays.
fn process_document<'a>(
    state: &'a mut ConvertState,
    active: &'a ActiveContext,
    loader: &'a mut dyn ContextLoader,
    val: &'a JsonLdVal,
) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
    Box::pin(async move {
        match val {
            JsonLdVal::Object(members, span) => {
                process_node(state, active, loader, members, span, None).await;
            }
            JsonLdVal::Array(items) => {
                for item in items {
                    process_document(state, active, loader, item).await;
                }
            }
            _ => {}
        }
    })
}

/// Process a JSON-LD value in the context of a named graph.
///
/// Boxed because it is directly self-recursive for arrays and mutually
/// recursive with [`process_node`].
fn process_document_with_graph<'a>(
    state: &'a mut ConvertState,
    active: &'a ActiveContext,
    loader: &'a mut dyn ContextLoader,
    val: &'a JsonLdVal,
    graph: Option<&'a Term>,
) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
    Box::pin(async move {
        match val {
            JsonLdVal::Object(members, span) => {
                process_node(state, active, loader, members, span, graph).await;
            }
            JsonLdVal::Array(items) => {
                for item in items {
                    process_document_with_graph(state, active, loader, item, graph).await;
                }
            }
            _ => {}
        }
    })
}

/// Process a JSON-LD node object.  Returns the subject term if the node can
/// serve as an object in another triple (e.g., a nested node).
///
/// Boxed because it is mutually recursive with `process_document_with_graph`
/// and transitively recursive through `collect_objects` → `value_to_rdf`.
fn process_node<'a>(
    state: &'a mut ConvertState,
    active: &'a ActiveContext,
    loader: &'a mut dyn ContextLoader,
    members: &'a [(String, JsonLdVal)],
    span: &'a Range<usize>,
    graph: Option<&'a Term>,
) -> Pin<Box<dyn Future<Output = Option<Term>> + 'a>> {
    Box::pin(async move {
        let map: HashMap<&str, &JsonLdVal> = members.iter().map(|(k, v)| (k.as_str(), v)).collect();

        // Value object — return as a literal, generate no subject-based triples
        if map.contains_key("@value") {
            return process_value_object(active, &map, span);
        }

        // List object
        if let Some(list_val) = map.get("@list") {
            let items = match list_val {
                JsonLdVal::Array(items) => items.as_slice(),
                other => std::slice::from_ref(*other),
            };
            return Some(process_list(state, active, loader, items, span, graph).await);
        }

        // Set object — process contents but return nothing as a subject
        if let Some(set_val) = map.get("@set") {
            if let JsonLdVal::Array(items) = set_val {
                for item in items {
                    process_document_with_graph(state, active, loader, item, graph).await;
                }
            }
            return None;
        }

        // Update active context if @context present in this node
        let updated;
        let active: &ActiveContext = if let Some(ctx_val) = map.get("@context") {
            updated = process_context(active.clone(), (*ctx_val).clone(), loader).await;
            &updated
        } else {
            active
        };

        // Determine the subject
        let subject: Term = if let Some(id_val) = map.get("@id") {
            match id_val {
                JsonLdVal::Str(s) => match expand_iri(active, s, false, true) {
                    Some(iri) if iri.starts_with("_:") => {
                        Term::BlankNode(BlankNode::Named(iri[2..].to_string(), span.start))
                    }
                    Some(iri) => Term::NamedNode(NamedNode::Full(iri, span.start)),
                    None => state.fresh_blank(span.start),
                },
                _ => state.fresh_blank(span.start),
            }
        } else {
            state.fresh_blank(span.start)
        };

        // @type → rdf:type triples
        if let Some(type_val) = map.get("@type") {
            for type_str in collect_strings(type_val) {
                if let Some(iri) = expand_iri(active, &type_str, true, true) {
                    let obj = iri_or_blank(iri, span.start);
                    state.add_triple(
                        subject.clone(),
                        ConvertState::named(RDF_TYPE),
                        obj,
                        graph.cloned(),
                        span.clone(),
                    );
                }
            }
        }

        // @graph → named graph containing nested nodes
        if let Some(graph_val) = map.get("@graph") {
            let named_graph = subject.clone();
            match graph_val {
                JsonLdVal::Array(items) => {
                    for item in items {
                        process_document_with_graph(
                            state,
                            active,
                            loader,
                            item,
                            Some(&named_graph),
                        )
                        .await;
                    }
                }
                JsonLdVal::Object(m, s) => {
                    process_node(state, active, loader, m, s, Some(&named_graph)).await;
                }
                _ => {}
            }
        }

        // @reverse → reverse-property triples (object becomes subject)
        if let Some(rev_val) = map.get("@reverse") {
            if let JsonLdVal::Object(rev_members, _) = rev_val {
                for (prop, val) in rev_members {
                    if prop.starts_with('@') {
                        continue;
                    }
                    if let Some(pred_iri) = expand_iri(active, prop, true, false) {
                        let pred = ConvertState::named(&pred_iri);
                        let objects =
                            collect_objects(state, active, loader, val, span, graph, None).await;
                        for obj in objects {
                            // Subject and object are swapped for reverse properties
                            state.add_triple(
                                obj,
                                pred.clone(),
                                subject.clone(),
                                graph.cloned(),
                                span.clone(),
                            );
                        }
                    }
                }
            }
        }

        // @included → additional node objects (processed in default graph)
        if let Some(included_val) = map.get("@included") {
            match included_val {
                JsonLdVal::Array(items) => {
                    for item in items {
                        process_document(state, active, loader, item).await;
                    }
                }
                JsonLdVal::Object(m, s) => {
                    process_node(state, active, loader, m, s, None).await;
                }
                _ => {}
            }
        }

        // Regular properties
        for (key, val) in members {
            match key.as_str() {
                "@context" | "@id" | "@type" | "@graph" | "@reverse" | "@included" | "@nest" => {
                    continue;
                }
                k if k.starts_with('@') => continue,
                _ => {}
            }

            // Handle @nest: the value must be an object whose properties are promoted
            // TODO: full @nest support — for now we skip nested containers

            let pred_iri = match expand_iri(active, key, true, false) {
                Some(iri) if !iri.starts_with('@') => iri,
                _ => continue,
            };

            let term_def = active.terms.get(key.as_str());
            let objects = collect_objects(state, active, loader, val, span, graph, term_def).await;
            if objects.is_empty() {
                continue;
            }
            let s = span.clone();
            state.triples.push(Spanned(
                Triple {
                    subject: Spanned(subject.clone(), s.clone()),
                    po: vec![Spanned(
                        PO {
                            predicate: Spanned(ConvertState::named(&pred_iri), s.clone()),
                            object: objects.into_iter().map(|o| Spanned(o, s.clone())).collect(),
                        },
                        s.clone(),
                    )],
                    graph: graph.map(|g| Spanned(g.clone(), s.clone())),
                },
                s,
            ));
        }

        Some(subject)
    })
}

// ── Value helpers ─────────────────────────────────────────────────────────────

fn collect_strings(val: &JsonLdVal) -> Vec<String> {
    match val {
        JsonLdVal::Str(s) => vec![s.clone()],
        JsonLdVal::Array(items) => items
            .iter()
            .filter_map(|i| {
                if let JsonLdVal::Str(s) = i {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect(),
        _ => vec![],
    }
}

/// Collect zero or more RDF terms from a JSON-LD value.
///
/// Boxed because it is directly self-recursive for arrays.
fn collect_objects<'a>(
    state: &'a mut ConvertState,
    active: &'a ActiveContext,
    loader: &'a mut dyn ContextLoader,
    val: &'a JsonLdVal,
    span: &'a Range<usize>,
    graph: Option<&'a Term>,
    term_def: Option<&'a TermDefinition>,
) -> Pin<Box<dyn Future<Output = Vec<Term>> + 'a>> {
    Box::pin(async move {
        match val {
            JsonLdVal::Array(items) => {
                let mut result = Vec::new();
                for item in items {
                    let mut sub =
                        collect_objects(state, active, loader, item, span, graph, term_def).await;
                    result.append(&mut sub);
                }
                result
            }
            _ => value_to_rdf(state, active, loader, val, span, graph, term_def)
                .await
                .into_iter()
                .collect(),
        }
    })
}

/// Map a single JSON-LD value to zero or one RDF term.
///
/// This is a plain `async fn` (not boxed) because it is not directly
/// self-recursive — the only cycle is through the boxed `process_node` and
/// `process_list` calls.
async fn value_to_rdf<'a>(
    state: &'a mut ConvertState,
    active: &'a ActiveContext,
    loader: &'a mut dyn ContextLoader,
    val: &'a JsonLdVal,
    span: &'a Range<usize>,
    graph: Option<&'a Term>,
    term_def: Option<&'a TermDefinition>,
) -> Option<Term> {
    match val {
        JsonLdVal::Object(members, obj_span) => {
            let map: HashMap<&str, &JsonLdVal> =
                members.iter().map(|(k, v)| (k.as_str(), v)).collect();
            if map.contains_key("@value") {
                return process_value_object(active, &map, obj_span);
            }
            if let Some(list_val) = map.get("@list") {
                let items = match list_val {
                    JsonLdVal::Array(items) => items.as_slice(),
                    other => std::slice::from_ref(*other),
                };
                return Some(process_list(state, active, loader, items, obj_span, graph).await);
            }
            process_node(state, active, loader, members, obj_span, graph).await
        }

        JsonLdVal::Str(s) => {
            if let Some(def) = term_def {
                match def.type_mapping.as_deref() {
                    Some("@id") => {
                        return expand_iri(active, s, false, true)
                            .map(|iri| iri_or_blank(iri, span.start));
                    }
                    Some("@vocab") => {
                        return expand_iri(active, s, true, false)
                            .map(|iri| Term::NamedNode(NamedNode::Full(iri, span.start)));
                    }
                    Some(ty) if !ty.starts_with('@') => {
                        let ty_owned = ty.to_string();
                        if let Some(ty_iri) = expand_iri(active, &ty_owned, true, false) {
                            return Some(Term::Literal(Literal::RDF(RDFLiteral {
                                value: s.clone(),
                                quote_style: StringStyle::Double,
                                lang: None,
                                ty: Some(NamedNode::Full(ty_iri, 0)),
                                idx: span.start,
                                len: span.len(),
                            })));
                        }
                    }
                    _ => {}
                }
                if let Some(lang) = &def.language {
                    return Some(Term::Literal(Literal::RDF(RDFLiteral {
                        value: s.clone(),
                        quote_style: StringStyle::Double,
                        lang: Some(lang.clone()),
                        ty: None,
                        idx: span.start,
                        len: span.len(),
                    })));
                }
            }
            // Default: plain string, inheriting the context default language if set
            let lang = active.language.clone();
            let ty = if lang.is_none() {
                Some(NamedNode::Full(XSD_STRING.to_string(), 0))
            } else {
                None
            };
            Some(Term::Literal(Literal::RDF(RDFLiteral {
                value: s.clone(),
                quote_style: StringStyle::Double,
                lang,
                ty,
                idx: span.start,
                len: span.len(),
            })))
        }

        JsonLdVal::Number(n) => {
            let type_iri = if n.contains('.') || n.to_lowercase().contains('e') {
                XSD_DOUBLE
            } else {
                XSD_INTEGER
            };
            Some(Term::Literal(Literal::RDF(RDFLiteral {
                value: n.clone(),
                quote_style: StringStyle::Double,
                lang: None,
                ty: Some(NamedNode::Full(type_iri.to_string(), 0)),
                idx: span.start,
                len: span.len(),
            })))
        }

        JsonLdVal::Bool(b) => Some(Term::Literal(Literal::RDF(RDFLiteral {
            value: if *b { "true" } else { "false" }.to_string(),
            quote_style: StringStyle::Double,
            lang: None,
            ty: Some(NamedNode::Full(XSD_BOOLEAN.to_string(), 0)),
            idx: span.start,
            len: span.len(),
        }))),

        JsonLdVal::Null => None,

        // Arrays must be handled by collect_objects, not here
        JsonLdVal::Array(_) => None,
    }
}

fn process_value_object(
    active: &ActiveContext,
    map: &HashMap<&str, &JsonLdVal>,
    span: &Range<usize>,
) -> Option<Term> {
    let value = map.get("@value")?;
    let lang_val = map.get("@language");
    let type_val = map.get("@type");

    let (value_str, is_complex) = match value {
        JsonLdVal::Str(s) => (s.clone(), false),
        JsonLdVal::Bool(b) => (if *b { "true" } else { "false" }.to_string(), false),
        JsonLdVal::Number(n) => (n.clone(), false),
        other => (json_serialize(other), true),
    };

    let type_str = type_val.and_then(|t| {
        if let JsonLdVal::Str(s) = t {
            Some(s.as_str())
        } else {
            None
        }
    });

    // @type: @json or complex value → rdf:JSON literal
    if type_str == Some("@json") || is_complex {
        return Some(Term::Literal(Literal::RDF(RDFLiteral {
            value: value_str,
            quote_style: StringStyle::Double,
            lang: None,
            ty: Some(NamedNode::Full(RDF_JSON.to_string(), 0)),
            idx: span.start,
            len: span.len(),
        })));
    }

    // Language-tagged string
    let lang_tag = lang_val.and_then(|l| {
        if let JsonLdVal::Str(s) = l {
            Some(s.as_str())
        } else {
            None
        }
    });
    if let Some(lt) = lang_tag {
        return Some(Term::Literal(Literal::RDF(RDFLiteral {
            value: value_str,
            quote_style: StringStyle::Double,
            lang: Some(lt.to_string()),
            ty: None,
            idx: span.start,
            len: span.len(),
        })));
    }

    // Explicit @type
    if let Some(ty) = type_str {
        let ty_owned = ty.to_string();
        let ty_node = expand_iri(active, &ty_owned, true, false).map(|iri| NamedNode::Full(iri, 0));
        return Some(Term::Literal(Literal::RDF(RDFLiteral {
            value: value_str,
            quote_style: StringStyle::Double,
            lang: None,
            ty: ty_node,
            idx: span.start,
            len: span.len(),
        })));
    }

    // Infer type from the JSON value kind
    let (inferred_ty, inferred_lang) = match value {
        JsonLdVal::Bool(_) => (Some(XSD_BOOLEAN), None),
        JsonLdVal::Number(n) => {
            if n.contains('.') || n.to_lowercase().contains('e') {
                (Some(XSD_DOUBLE), None)
            } else {
                (Some(XSD_INTEGER), None)
            }
        }
        _ => {
            if let Some(lang) = &active.language {
                (None, Some(lang.clone()))
            } else {
                (Some(XSD_STRING), None)
            }
        }
    };

    Some(Term::Literal(Literal::RDF(RDFLiteral {
        value: value_str,
        quote_style: StringStyle::Double,
        lang: inferred_lang,
        ty: inferred_ty.map(|s| NamedNode::Full(s.to_string(), 0)),
        idx: span.start,
        len: span.len(),
    })))
}

/// Build an `rdf:first`/`rdf:rest` list and return the head node.
///
/// Boxed because it is mutually recursive with `value_to_rdf` (which may
/// call `process_list` again for nested `@list` values).
fn process_list<'a>(
    state: &'a mut ConvertState,
    active: &'a ActiveContext,
    loader: &'a mut dyn ContextLoader,
    items: &'a [JsonLdVal],
    span: &'a Range<usize>,
    graph: Option<&'a Term>,
) -> Pin<Box<dyn Future<Output = Term> + 'a>> {
    Box::pin(async move {
        if items.is_empty() {
            return ConvertState::named(RDF_NIL);
        }
        // Build the linked list in reverse
        let mut rest = ConvertState::named(RDF_NIL);
        for item in items.iter().rev() {
            let first_term = value_to_rdf(state, active, loader, item, span, graph, None)
                .await
                .unwrap_or(Term::Invalid);
            let node = state.fresh_blank(span.start);
            state.add_triple(
                node.clone(),
                ConvertState::named(RDF_FIRST),
                first_term,
                graph.cloned(),
                span.clone(),
            );
            state.add_triple(
                node.clone(),
                ConvertState::named(RDF_REST),
                rest,
                graph.cloned(),
                span.clone(),
            );
            rest = node;
        }
        rest
    })
}

fn iri_or_blank(iri: String, offset: usize) -> Term {
    if iri.starts_with("_:") {
        Term::BlankNode(BlankNode::Named(iri[2..].to_string(), offset))
    } else {
        Term::NamedNode(NamedNode::Full(iri, offset))
    }
}

fn json_serialize(val: &JsonLdVal) -> String {
    match val {
        JsonLdVal::Object(members, _) => {
            let pairs: Vec<String> = members
                .iter()
                .map(|(k, v)| format!("\"{}\":{}", json_escape(k), json_serialize(v)))
                .collect();
            format!("{{{}}}", pairs.join(","))
        }
        JsonLdVal::Array(items) => {
            let elems: Vec<String> = items.iter().map(json_serialize).collect();
            format!("[{}]", elems.join(","))
        }
        JsonLdVal::Str(s) => format!("\"{}\"", json_escape(s)),
        JsonLdVal::Number(n) => n.clone(),
        JsonLdVal::Bool(b) => {
            if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        JsonLdVal::Null => "null".to_string(),
    }
}

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Convert a parsed JSON-LD CST to RDF triples.
///
/// Remote `@context` references are not resolved.  Use
/// [`convert_with_loader`] to supply an async context-loading hook.
pub fn convert(root: &Node) -> Turtle {
    // NoopContextLoader always resolves immediately (std::future::ready),
    // so polling once is sufficient and safe without a real async runtime.
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    fn noop_clone(p: *const ()) -> RawWaker {
        RawWaker::new(p, &NOOP_VTABLE)
    }
    fn noop(_: *const ()) {}
    static NOOP_VTABLE: RawWakerVTable = RawWakerVTable::new(noop_clone, noop, noop, noop);

    // SAFETY: The waker never actually wakes anything; it is only used to drive
    // the future to completion in a single synchronous poll.  This is safe
    // because NoopContextLoader::load returns std::future::ready(None), which
    // is always Poll::Ready on the first poll.
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &NOOP_VTABLE)) };
    let mut cx = Context::from_waker(&waker);
    let mut loader = NoopContextLoader;
    let mut fut = std::pin::pin!(convert_with_loader(root, &mut loader));
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => unreachable!("NoopContextLoader always resolves immediately"),
    }
}

/// Convert a parsed JSON-LD CST to RDF triples, using `loader` to fetch
/// remote `@context` documents asynchronously.
pub async fn convert_with_loader(root: &Node, loader: &mut dyn ContextLoader) -> Turtle {
    let mut state = ConvertState::new();
    let active = ActiveContext::default();

    // jsonldDoc ::= jsonValue — the JsonValue is a direct child of ROOT.
    if let Some(json_val_node) = root.children().find(|c| c.kind() == SyntaxKind::JsonValue) {
        if let Some(val) = cst_to_value(&json_val_node) {
            process_document(&mut state, &active, loader, &val).await;
        }
    }

    state.into_turtle()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::super::parser::{Rule, SyntaxKind as SK};
    use super::*;

    fn parse_jsonld(input: &str) -> Turtle {
        use crate::parse as crate_parse;
        let rule = Rule::new(SK::JsonldDoc);
        let (result, _) = crate_parse(rule, input);
        let root = result.syntax::<Lang>();
        convert(&root)
    }

    fn triples_of(t: &Turtle) -> &[Spanned<Triple>] {
        &t.triples
    }

    fn subject_iri(triple: &Triple) -> Option<&str> {
        match &triple.subject.0 {
            Term::NamedNode(NamedNode::Full(s, _)) => Some(s),
            _ => None,
        }
    }

    fn predicate_iri(triple: &Triple) -> Option<&str> {
        triple.po.first().and_then(|po| {
            if let Term::NamedNode(NamedNode::Full(s, _)) = &po.predicate.0 {
                Some(s.as_str())
            } else {
                None
            }
        })
    }

    fn object_iri(triple: &Triple) -> Option<&str> {
        triple
            .po
            .first()
            .and_then(|po| po.object.first())
            .and_then(|o| {
                if let Term::NamedNode(NamedNode::Full(s, _)) = &o.0 {
                    Some(s.as_str())
                } else {
                    None
                }
            })
    }

    fn object_literal(triple: &Triple) -> Option<(&str, Option<&str>, Option<&str>)> {
        triple
            .po
            .first()
            .and_then(|po| po.object.first())
            .and_then(|o| {
                if let Term::Literal(Literal::RDF(lit)) = &o.0 {
                    let lang = lit.lang.as_deref();
                    let ty = if let Some(NamedNode::Full(s, _)) = &lit.ty {
                        Some(s.as_str())
                    } else {
                        None
                    };
                    Some((lit.value.as_str(), lang, ty))
                } else {
                    None
                }
            })
    }

    // ── 1. Basic node with @id and one property ───────────────────────────────

    #[test]
    fn test_basic_triple() {
        let t = parse_jsonld(
            r#"
            {
              "@id": "http://example.org/s",
              "http://example.org/p": "hello"
            }
        "#,
        );
        let triples = triples_of(&t);
        assert_eq!(triples.len(), 1);
        assert_eq!(subject_iri(&triples[0]), Some("http://example.org/s"));
        assert_eq!(predicate_iri(&triples[0]), Some("http://example.org/p"));
        assert_eq!(
            object_literal(&triples[0]),
            Some(("hello", None, Some(XSD_STRING)))
        );
    }

    // ── 2. Multiple properties ────────────────────────────────────────────────

    #[test]
    fn test_multiple_properties() {
        let t = parse_jsonld(
            r#"
            {
              "@id": "http://example.org/s",
              "http://example.org/p1": "a",
              "http://example.org/p2": "b"
            }
        "#,
        );
        assert_eq!(triples_of(&t).len(), 2);
    }

    // ── 3. Anonymous nested object → blank node ───────────────────────────────

    #[test]
    fn test_nested_blank_node() {
        let t = parse_jsonld(
            r#"
            {
              "@id": "http://example.org/s",
              "http://example.org/p": {
                "http://example.org/q": "inner"
              }
            }
        "#,
        );
        // 2 triples: outer s→blank, inner blank→inner
        assert_eq!(triples_of(&t).len(), 2);
        let outer = &triples_of(&t)[1]; // outer triple
        assert_eq!(subject_iri(outer), Some("http://example.org/s"));
        // Object should be a blank node
        if let Some(po) = outer.po.first() {
            if let Some(obj) = po.object.first() {
                assert!(matches!(obj.0, Term::BlankNode(_)));
            } else {
                panic!("no object");
            }
        }
    }

    // ── 4. Inline @context with term definitions ──────────────────────────────

    #[test]
    fn test_inline_context_term() {
        let t = parse_jsonld(
            r#"
            {
              "@context": { "name": "http://schema.org/name" },
              "@id": "http://example.org/alice",
              "name": "Alice"
            }
        "#,
        );
        let triples = triples_of(&t);
        assert_eq!(triples.len(), 1);
        assert_eq!(predicate_iri(&triples[0]), Some("http://schema.org/name"));
        assert_eq!(
            object_literal(&triples[0]),
            Some(("Alice", None, Some(XSD_STRING)))
        );
    }

    // ── 5. @vocab expansion ───────────────────────────────────────────────────

    #[test]
    fn test_vocab_expansion() {
        let t = parse_jsonld(
            r#"
            {
              "@context": { "@vocab": "http://schema.org/" },
              "@id": "http://example.org/alice",
              "name": "Alice"
            }
        "#,
        );
        let triples = triples_of(&t);
        assert_eq!(triples.len(), 1);
        assert_eq!(predicate_iri(&triples[0]), Some("http://schema.org/name"));
    }

    // ── 6. @type generates rdf:type triple ───────────────────────────────────

    #[test]
    fn test_type_triple() {
        let t = parse_jsonld(
            r#"
            {
              "@id": "http://example.org/alice",
              "@type": "http://schema.org/Person"
            }
        "#,
        );
        let triples = triples_of(&t);
        assert_eq!(triples.len(), 1);
        assert_eq!(predicate_iri(&triples[0]), Some(RDF_TYPE));
        assert_eq!(object_iri(&triples[0]), Some("http://schema.org/Person"));
    }

    // ── 7. @type array ────────────────────────────────────────────────────────

    #[test]
    fn test_type_array() {
        let t = parse_jsonld(
            r#"
            {
              "@id": "http://example.org/x",
              "@type": ["http://example.org/A", "http://example.org/B"]
            }
        "#,
        );
        assert_eq!(triples_of(&t).len(), 2);
    }

    // ── 8. Value object with @language ────────────────────────────────────────

    #[test]
    fn test_value_object_language() {
        let t = parse_jsonld(
            r#"
            {
              "@id": "http://example.org/s",
              "http://example.org/p": { "@value": "hello", "@language": "en" }
            }
        "#,
        );
        let triples = triples_of(&t);
        assert_eq!(triples.len(), 1);
        assert_eq!(
            object_literal(&triples[0]),
            Some(("hello", Some("en"), None))
        );
    }

    // ── 9. Value object with explicit @type ───────────────────────────────────

    #[test]
    fn test_value_object_typed() {
        let t = parse_jsonld(
            r#"
            {
              "@id": "http://example.org/s",
              "http://example.org/p": { "@value": "42", "@type": "http://www.w3.org/2001/XMLSchema#integer" }
            }
        "#,
        );
        let triples = triples_of(&t);
        assert_eq!(triples.len(), 1);
        assert_eq!(
            object_literal(&triples[0]),
            Some(("42", None, Some(XSD_INTEGER)))
        );
    }

    // ── 10. Native number → xsd:integer / xsd:double ─────────────────────────

    #[test]
    fn test_native_integer() {
        let t = parse_jsonld(
            r#"
            { "@id": "http://example.org/s", "http://example.org/p": 42 }
        "#,
        );
        assert_eq!(
            object_literal(&triples_of(&t)[0]),
            Some(("42", None, Some(XSD_INTEGER)))
        );
    }

    #[test]
    fn test_native_double() {
        let t = parse_jsonld(
            r#"
            { "@id": "http://example.org/s", "http://example.org/p": 3.14 }
        "#,
        );
        assert_eq!(
            object_literal(&triples_of(&t)[0]),
            Some(("3.14", None, Some(XSD_DOUBLE)))
        );
    }

    // ── 11. Boolean → xsd:boolean ────────────────────────────────────────────

    #[test]
    fn test_boolean() {
        let t = parse_jsonld(
            r#"
            { "@id": "http://example.org/s", "http://example.org/p": true }
        "#,
        );
        assert_eq!(
            object_literal(&triples_of(&t)[0]),
            Some(("true", None, Some(XSD_BOOLEAN)))
        );
    }

    // ── 12. Null value is skipped ─────────────────────────────────────────────

    #[test]
    fn test_null_skipped() {
        let t = parse_jsonld(
            r#"
            { "@id": "http://example.org/s", "http://example.org/p": null }
        "#,
        );
        assert_eq!(triples_of(&t).len(), 0);
    }

    // ── 13. @list → rdf:first/rdf:rest chain ─────────────────────────────────

    #[test]
    fn test_list() {
        let t = parse_jsonld(
            r#"
            {
              "@id": "http://example.org/s",
              "http://example.org/p": { "@list": ["a", "b"] }
            }
        "#,
        );
        // 1 triple for the property + 2*2 triples for the two list nodes
        let triples = triples_of(&t);
        assert!(triples.len() >= 3);
        // The property triple's object should be a blank node (list head)
        let prop_triple = triples.last().unwrap();
        if let Some(po) = prop_triple.po.first() {
            if let Some(obj) = po.object.first() {
                assert!(matches!(obj.0, Term::BlankNode(_)));
            }
        }
        // rdf:first and rdf:rest triples should be present
        let has_first = triples.iter().any(|t| predicate_iri(t) == Some(RDF_FIRST));
        let has_rest = triples.iter().any(|t| predicate_iri(t) == Some(RDF_REST));
        assert!(has_first);
        assert!(has_rest);
    }

    // ── 14. Empty @list → rdf:nil ─────────────────────────────────────────────

    #[test]
    fn test_empty_list() {
        let t = parse_jsonld(
            r#"
            {
              "@id": "http://example.org/s",
              "http://example.org/p": { "@list": [] }
            }
        "#,
        );
        let triples = triples_of(&t);
        assert_eq!(triples.len(), 1);
        assert_eq!(object_iri(&triples[0]), Some(RDF_NIL));
    }

    // ── 15. @graph → named graph triples ─────────────────────────────────────

    #[test]
    fn test_named_graph() {
        let t = parse_jsonld(
            r#"
            {
              "@id": "http://example.org/g",
              "@graph": [
                { "@id": "http://example.org/s", "http://example.org/p": "v" }
              ]
            }
        "#,
        );
        let triples = triples_of(&t);
        // One triple inside the named graph
        assert_eq!(triples.len(), 1);
        let triple = &triples[0];
        if let Some(g) = &triple.graph {
            if let Term::NamedNode(NamedNode::Full(g_iri, _)) = &g.0 {
                assert_eq!(g_iri, "http://example.org/g");
            } else {
                panic!("graph should be a named node");
            }
        } else {
            panic!("triple should be in a named graph");
        }
    }

    // ── 16. @reverse property ────────────────────────────────────────────────

    #[test]
    fn test_reverse() {
        let t = parse_jsonld(
            r#"
            {
              "@id": "http://example.org/child",
              "@reverse": {
                "http://example.org/parent": { "@id": "http://example.org/dad" }
              }
            }
        "#,
        );
        // Triple: dad → parent → child
        let triples = triples_of(&t);
        assert_eq!(triples.len(), 1);
        assert_eq!(subject_iri(&triples[0]), Some("http://example.org/dad"));
        assert_eq!(object_iri(&triples[0]), Some("http://example.org/child"));
    }

    // ── 17. Top-level array ───────────────────────────────────────────────────

    #[test]
    fn test_top_level_array() {
        let t = parse_jsonld(
            r#"
            [
              { "@id": "http://example.org/a", "http://example.org/p": "x" },
              { "@id": "http://example.org/b", "http://example.org/p": "y" }
            ]
        "#,
        );
        assert_eq!(triples_of(&t).len(), 2);
    }

    // ── 18. Compact IRI in context ────────────────────────────────────────────

    #[test]
    fn test_compact_iri() {
        let t = parse_jsonld(
            r#"
            {
              "@context": {
                "schema": "http://schema.org/",
                "name": "schema:name"
              },
              "@id": "http://example.org/alice",
              "name": "Alice"
            }
        "#,
        );
        let triples = triples_of(&t);
        assert_eq!(triples.len(), 1);
        assert_eq!(predicate_iri(&triples[0]), Some("http://schema.org/name"));
    }

    // ── 19. @type coercion to @id ─────────────────────────────────────────────

    #[test]
    fn test_type_coercion_id() {
        let t = parse_jsonld(
            r#"
            {
              "@context": {
                "knows": { "@id": "http://schema.org/knows", "@type": "@id" }
              },
              "@id": "http://example.org/alice",
              "knows": "http://example.org/bob"
            }
        "#,
        );
        let triples = triples_of(&t);
        assert_eq!(triples.len(), 1);
        assert_eq!(object_iri(&triples[0]), Some("http://example.org/bob"));
    }

    // ── 20. Default language from context ─────────────────────────────────────

    #[test]
    fn test_default_language() {
        let t = parse_jsonld(
            r#"
            {
              "@context": { "@language": "fr" },
              "@id": "http://example.org/s",
              "http://example.org/p": "Bonjour"
            }
        "#,
        );
        let triples = triples_of(&t);
        assert_eq!(triples.len(), 1);
        assert_eq!(
            object_literal(&triples[0]),
            Some(("Bonjour", Some("fr"), None))
        );
    }

    // ── 21. Error recovery — malformed JSON-LD still produces some triples ────

    #[test]
    fn test_error_recovery() {
        // Missing closing brace — parser should still extract the one valid property
        let t = parse_jsonld(r#"{ "@id": "http://example.org/s", "http://example.org/p": "v" "#);
        // We may or may not get the triple depending on how error recovery works,
        // but the converter should not panic.
        let _ = triples_of(&t);
    }

    // ── 22. Remote @context via a custom ContextLoader ────────────────────────

    /// Drive an async future to completion using a no-op waker.
    ///
    /// Safe as long as the future only awaits immediately-ready sub-futures
    /// (i.e. no real I/O suspension).
    fn block_on_ready<T>(fut: impl std::future::Future<Output = T>) -> T {
        use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
        fn clone(p: *const ()) -> RawWaker {
            RawWaker::new(p, &VTABLE)
        }
        fn noop(_: *const ()) {}
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
        let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
        let mut cx = Context::from_waker(&waker);
        let mut pinned = std::pin::pin!(fut);
        match pinned.as_mut().poll(&mut cx) {
            Poll::Ready(v) => v,
            Poll::Pending => panic!("future did not complete synchronously"),
        }
    }

    #[test]
    fn test_remote_context_loader() {
        // A mock loader that returns a hardcoded context document for one URL.
        struct MockLoader;
        impl ContextLoader for MockLoader {
            fn load<'a>(
                &'a mut self,
                url: &'a str,
            ) -> Pin<Box<dyn Future<Output = Option<String>> + 'a>> {
                let result = if url == "https://example.org/context.jsonld" {
                    Some(
                        r#"{
                          "@context": {
                            "name":   "http://schema.org/name",
                            "email":  "http://schema.org/email",
                            "Person": "http://schema.org/Person"
                          }
                        }"#
                        .to_string(),
                    )
                } else {
                    None
                };
                Box::pin(std::future::ready(result))
            }
        }

        let input = r#"
        {
          "@context": "https://example.org/context.jsonld",
          "@id":   "http://example.org/alice",
          "@type": "Person",
          "name":  "Alice",
          "email": "alice@example.org"
        }"#;

        let rule = Rule::new(SK::JsonldDoc);
        let (result, _) = crate::parse(rule, input);
        let root = result.syntax::<Lang>();

        let turtle = block_on_ready(convert_with_loader(&root, &mut MockLoader));
        let triples = triples_of(&turtle);

        // @type → rdf:type + 2 data properties = 3 triples
        assert_eq!(triples.len(), 3);

        // rdf:type triple
        let type_triple = triples
            .iter()
            .find(|t| predicate_iri(t) == Some(RDF_TYPE))
            .expect("missing rdf:type triple");
        assert_eq!(subject_iri(type_triple), Some("http://example.org/alice"));
        assert_eq!(object_iri(type_triple), Some("http://schema.org/Person"));

        // name triple — expanded via the remote context
        let name_triple = triples
            .iter()
            .find(|t| predicate_iri(t) == Some("http://schema.org/name"))
            .expect("missing schema:name triple");
        assert_eq!(
            object_literal(name_triple),
            Some(("Alice", None, Some(XSD_STRING)))
        );

        // email triple
        assert!(
            triples
                .iter()
                .any(|t| predicate_iri(t) == Some("http://schema.org/email")),
            "missing schema:email triple"
        );
    }
}
