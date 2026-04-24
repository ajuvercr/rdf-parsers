use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::future::Future;
use std::io::Cursor;
use std::ops::Range;
use std::pin::Pin;

use ariadne::{ColorGenerator, Config, Label, Report, ReportBuilder, ReportKind, Source};
use rowan::{NodeOrToken, SyntaxElement};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use rdf_parsers::{
    IncrementalBias, Parse, ParserTrait, PrevParseInfo, TokenTrait, effective_error_span,
    jsonld::parser::{Lang as JsonLdLang, Rule as JsonLdRule, SyntaxKind as JsonLdSyntaxKind},
    model::Turtle,
    n3::parser::{Lang as N3Lang, Rule as N3Rule, SyntaxKind as N3SyntaxKind},
    ntriples::parser::{
        Lang as NTriplesLang, Rule as NTriplesRule, SyntaxKind as NTriplesSyntaxKind,
    },
    parse_incremental,
    sparql::parser::{Lang as SparqlLang, Rule as SparqlRule, SyntaxKind as SparqlSyntaxKind},
    trig::parser::{Lang as TrigLang, Rule as TrigRule, SyntaxKind as TrigSyntaxKind},
    turtle::parser::{Lang, Rule, SyntaxKind},
};

thread_local! {
    static PREV_TURTLE:   RefCell<Option<PrevParseInfo>> = RefCell::new(None);
    static PREV_SPARQL:   RefCell<Option<PrevParseInfo>> = RefCell::new(None);
    static PREV_TRIG:     RefCell<Option<PrevParseInfo>> = RefCell::new(None);
    static PREV_NTRIPLES: RefCell<Option<PrevParseInfo>> = RefCell::new(None);
    static PREV_N3:       RefCell<Option<PrevParseInfo>> = RefCell::new(None);
    static PREV_JSONLD:   RefCell<Option<PrevParseInfo>> = RefCell::new(None);
    static JSONLD_CACHE:   RefCell<WasmFetchLoader> = RefCell::new(WasmFetchLoader::default());
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Walk a finished A* `Parse` and return `(byte_range, message)` pairs for
/// every error indicator in source order: both Error *nodes* (grammar errors)
/// and Error *tokens* (lexer failures).
fn get_error_range_pairs<L>(parse: &Parse) -> Vec<(Range<usize>, String)>
where
    L: rowan::Language,
    L::Kind: TokenTrait,
{
    let msgs: Vec<String> = parse.errors.iter().cloned().collect();

    let root = parse.syntax::<L>();

    // Collect error ranges from both Error nodes and Error tokens, in source
    // order.  Error nodes come from grammar-level recovery (Step::Error,
    // Step::Delete, Unparsed); Error tokens come from lexer failures.
    let mut ranges: Vec<Range<usize>> = Vec::new();
    for elem in root.descendants_with_tokens() {
        match elem {
            NodeOrToken::Node(n) if n.kind() == L::Kind::ERROR => {
                ranges.push(effective_error_span::<L>(&n));
            }
            NodeOrToken::Token(t) if t.kind() == L::Kind::ERROR => {
                // Skip Error tokens that are children of Error nodes — those
                // are already covered by the parent node's range.
                let parent_is_error = t.parent().map_or(false, |p| p.kind() == L::Kind::ERROR);
                if !parent_is_error {
                    let r = t.text_range();
                    ranges.push(usize::from(r.start())..usize::from(r.end()));
                }
            }
            _ => {}
        }
    }

    ranges.into_iter().zip(msgs).collect()
}

fn render_ariadne(errors: &[(Range<usize>, String)], source: &str, loc: &str) -> String {
    if errors.is_empty() {
        return String::from("No errors \u{2713}");
    }

    // ariadne with auto-color calls concolor::get(Stream::Stderr).color() to decide
    // whether to emit ANSI codes — not Config::with_color and not yansi's global state.
    // Force it on via concolor's api feature.
    concolor::set(concolor::ColorChoice::Always);

    let s = errors.iter().map(|x| x.0.start).min().unwrap_or_default();
    let e = errors.iter().map(|x| x.0.end).max().unwrap_or_default();

    let mut colors = ColorGenerator::from_state([10000, 15000, 15000], 0.8);

    let report = errors.iter().fold(
        Report::build(ReportKind::Error, (loc, s..e))
            .with_config(Config::default().with_color(true)),
        |report: ReportBuilder<(&str, Range<usize>)>, (span, msg)| {
            report.with_label(
                Label::new((loc, span.clone()))
                    .with_message(msg)
                    .with_color(colors.next()),
            )
        },
    );

    let mut out = Cursor::new(Vec::<u8>::new());
    let _ = report.finish().write((loc, Source::from(source)), &mut out);
    String::from_utf8(out.into_inner()).unwrap_or_default()
}

/// Recursively format a syntax node/token into `out`, annotating Error nodes
/// with their error message.  `error_msgs` maps a node's tree-order index
/// among all Error nodes to its message string.
fn format_element<L>(
    elem: SyntaxElement<L>,
    error_msgs: &HashMap<usize, String>,
    error_idx: &mut usize,
    depth: usize,
    out: &mut String,
) where
    L: rowan::Language,
    L::Kind: TokenTrait + std::fmt::Debug,
{
    let indent = "  ".repeat(depth);
    match elem {
        NodeOrToken::Node(n) => {
            let range = n.text_range();
            let is_error = n.kind() == L::Kind::ERROR;
            let _ = write!(
                out,
                "{}{:?}@{}..{}",
                indent,
                n.kind(),
                u32::from(range.start()),
                u32::from(range.end()),
            );
            if is_error {
                if let Some(msg) = error_msgs.get(error_idx) {
                    let _ = write!(out, "  // {}", msg);
                }
                *error_idx += 1;
            }
            out.push('\n');
            for child in n.children_with_tokens() {
                format_element(child, error_msgs, error_idx, depth + 1, out);
            }
        }
        NodeOrToken::Token(t) => {
            let range = t.text_range();
            let text = t.text();
            let is_error_token = t.kind() == L::Kind::ERROR
                && t.parent().map_or(true, |p| p.kind() != L::Kind::ERROR);
            if text.len() > 40 {
                let _ = write!(
                    out,
                    "{}{:?}@{}..{}  \"{} ...\"",
                    indent,
                    t.kind(),
                    u32::from(range.start()),
                    u32::from(range.end()),
                    &text[..40],
                );
            } else {
                let _ = write!(
                    out,
                    "{}{:?}@{}..{}  {:?}",
                    indent,
                    t.kind(),
                    u32::from(range.start()),
                    u32::from(range.end()),
                    text,
                );
            }
            if is_error_token {
                if let Some(msg) = error_msgs.get(error_idx) {
                    let _ = write!(out, "  // {}", msg);
                }
                *error_idx += 1;
            }
            out.push('\n');
        }
    }
}

fn render_ast<L>(parse: &Parse, pairs: &[(Range<usize>, String)]) -> String
where
    L: rowan::Language,
    L::Kind: TokenTrait + std::fmt::Debug,
{
    // pairs[i] corresponds to the i-th Error node in pre-order traversal.
    let error_msgs: HashMap<usize, String> = pairs
        .iter()
        .enumerate()
        .map(|(i, (_, msg))| (i, msg.clone()))
        .collect();

    let root = parse.syntax::<L>();
    let mut out = String::new();
    format_element(NodeOrToken::Node(root), &error_msgs, &mut 0, 0, &mut out);
    out
}

fn render_triples(turtle: &Turtle) -> String {
    let mut out = String::new();

    if let Some(base) = &turtle.base {
        let _ = writeln!(out, "{}", base.value());
    }
    for prefix in &turtle.prefixes {
        let _ = writeln!(out, "{}", prefix.value());
    }
    if turtle.base.is_some() || !turtle.prefixes.is_empty() {
        out.push('\n');
    }

    if turtle.triples.is_empty() {
        out.push_str("(no triples)");
    } else {
        for triple in &turtle.triples {
            let _ = writeln!(out, "{}", triple.value());
        }
    }

    out
}

#[wasm_bindgen]
pub struct ParseResult {
    ariadne: String,
    ast: String,
    triples: String,
    formatted: String,
}

#[wasm_bindgen]
impl ParseResult {
    #[wasm_bindgen(getter)]
    pub fn ast(&self) -> String {
        self.ast.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn ariadne(&self) -> String {
        self.ariadne.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn triples(&self) -> String {
        self.triples.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn formatted(&self) -> String {
        self.formatted.clone()
    }
}

/// Parse `text` incrementally against `prev`, render the ariadne error report,
/// AST, triples, and formatted output.
fn parse_language<T, L>(
    root: T,
    text: &str,
    prev: &RefCell<Option<PrevParseInfo>>,
    convert_fn: impl FnOnce(&rowan::SyntaxNode<L>) -> Turtle,
    format_fn: impl FnOnce(&rowan::SyntaxNode<L>) -> String,
) -> ParseResult
where
    T: ParserTrait + 'static + Clone,
    for<'a> T::Kind: logos::Logos<'a, Source = str>,
    for<'a> <<T as ParserTrait>::Kind as logos::Logos<'a>>::Extras: Default,
    L: rowan::Language,
    L::Kind: TokenTrait + std::fmt::Debug,
{
    let (parse, new_prev) = {
        let p = prev.borrow();
        parse_incremental(root, text, p.as_ref(), IncrementalBias::default())
    };
    let pairs = get_error_range_pairs::<L>(&parse);
    let syntax_root = parse.syntax::<L>();
    let turtle = convert_fn(&syntax_root);
    let formatted = format_fn(&syntax_root);
    *prev.borrow_mut() = Some(new_prev);
    ParseResult {
        ariadne: render_ariadne(&pairs, text, "input"),
        ast: render_ast::<L>(&parse, &pairs),
        triples: render_triples(&turtle),
        formatted,
    }
}

/// A `ContextLoader` that fetches remote JSON-LD context documents using the
/// browser `fetch()` API.

#[derive(Default)]
struct WasmFetchLoader {
    cache: HashMap<String, Option<String>>,
    val_cache: HashMap<String, rdf_parsers::jsonld::convert::JsonLdVal>,
}

impl rdf_parsers::jsonld::convert::ContextLoader for WasmFetchLoader {
    fn load<'a>(&'a mut self, url: &'a str) -> Pin<Box<dyn Future<Output = Option<String>> + 'a>> {
        let url = url.to_string();
        Box::pin(async move {
            if let Some(o) = self.cache.get(&url) {
                return o.clone();
            }
            let window = web_sys::window()?;
            let resp_val = JsFuture::from(window.fetch_with_str(&url)).await.ok()?;
            let resp: web_sys::Response = resp_val.dyn_into().ok()?;
            if !resp.ok() {
                return None;
            }
            let text_val = JsFuture::from(resp.text().ok()?).await.ok()?;
            let v = text_val.as_string();
            self.cache.insert(url, v.clone());
            v
        })
    }

    fn load_val<'a>(
        &'a mut self,
        url: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<rdf_parsers::jsonld::convert::JsonLdVal>> + 'a>> {
        let url = url.to_string();
        Box::pin(async move {
            if let Some(v) = self.val_cache.get(&url) {
                return Some(v.clone());
            }
            let content = self.load(&url).await?;
            let val = rdf_parsers::jsonld::convert::parse_jsonld_for_context(&content)?;
            self.val_cache.insert(url, val.clone());
            Some(val)
        })
    }
}

#[wasm_bindgen]
pub async fn parse(language: &str, text: &str) -> Result<ParseResult, JsValue> {
    Ok(match language {
        "turtle" => PREV_TURTLE.with(|prev| {
            parse_language::<_, Lang>(
                Rule::new(SyntaxKind::TurtleDoc),
                text,
                prev,
                |n| rdf_parsers::turtle::convert::convert(n),
                |n| rdf_parsers::turtle::parser::format::format(n, 40),
            )
        }),
        "sparql" => PREV_SPARQL.with(|prev| {
            parse_language::<_, SparqlLang>(
                SparqlRule::new(SparqlSyntaxKind::QueryUnit),
                text,
                prev,
                |n| rdf_parsers::sparql::convert::convert(n),
                |n| rdf_parsers::sparql::parser::format::format(n, 80),
            )
        }),
        "trig" => PREV_TRIG.with(|prev| {
            parse_language::<_, TrigLang>(
                TrigRule::new(TrigSyntaxKind::TrigDoc),
                text,
                prev,
                |n| rdf_parsers::trig::convert::convert(n),
                |n| rdf_parsers::trig::parser::format::format(n, 80),
            )
        }),
        "ntriples" => PREV_NTRIPLES.with(|prev| {
            parse_language::<_, NTriplesLang>(
                NTriplesRule::new(NTriplesSyntaxKind::NtriplesDoc),
                text,
                prev,
                |n| rdf_parsers::ntriples::convert::convert(n),
                |n| rdf_parsers::ntriples::parser::format::format(n, 80),
            )
        }),
        "n3" => PREV_N3.with(|prev| {
            parse_language::<_, N3Lang>(
                N3Rule::new(N3SyntaxKind::N3Doc),
                text,
                prev,
                |n| rdf_parsers::n3::convert::convert(n),
                |n| rdf_parsers::n3::parser::format::format(n, 80),
            )
        }),
        "jsonld" => {
            // Parse the CST synchronously (updates the incremental cache),
            // then convert asynchronously so the WasmFetchLoader can await
            // fetch() calls for any remote @context URLs.
            let (parse, pairs) = PREV_JSONLD.with(|prev| {
                let (parse, new_prev) = {
                    let p = prev.borrow();
                    parse_incremental(
                        JsonLdRule::new(JsonLdSyntaxKind::JsonldDoc),
                        text,
                        p.as_ref(),
                        IncrementalBias::default(),
                    )
                };
                let pairs = get_error_range_pairs::<JsonLdLang>(&parse);
                *prev.borrow_mut() = Some(new_prev);
                (parse, pairs)
            });
            let root = parse.syntax::<JsonLdLang>();

            let mut cache = JSONLD_CACHE.take();
            let (turtle, _) =
                rdf_parsers::jsonld::convert::convert_with_loader(&root, &mut cache, None).await;
            JSONLD_CACHE.set(cache);

            let formatted = rdf_parsers::jsonld::parser::format::format(&root, 80);
            ParseResult {
                ariadne: render_ariadne(&pairs, text, "input"),
                ast: render_ast::<JsonLdLang>(&parse, &pairs),
                triples: render_triples(&turtle),
                formatted,
            }
        }
        _ => return Err("Unknown language".into()),
    })
}
