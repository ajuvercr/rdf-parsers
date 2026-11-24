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
    DEFAULT_LOCAL_WINDOW, Engine, Parse, ParseOptions, ParserTrait, PrevParseInfo, TokenTrait,
    effective_error_span, last_commits, last_iterations,
    jsonld::parser::{Lang as JsonLdLang, Rule as JsonLdRule, SyntaxKind as JsonLdSyntaxKind},
    model::Turtle,
    n3::parser::{Lang as N3Lang, Rule as N3Rule, SyntaxKind as N3SyntaxKind},
    ntriples::parser::{
        Lang as NTriplesLang, Rule as NTriplesRule, SyntaxKind as NTriplesSyntaxKind,
    },
    parse_incremental_with,
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
    /// The engine of `parse` (see `set_engine`).
    static ENGINE: std::cell::Cell<Engine> = const { std::cell::Cell::new(Engine::Global) };
    /// The editing history of every engine in `compare`, per language.
    static COMPARE_PREV: RefCell<HashMap<(String, usize), PrevParseInfo>> = RefCell::new(HashMap::new());
}

/// The engines the page offers, by the names it uses.
const ENGINES: [(&str, Engine); 4] = [
    ("global", Engine::Global),
    ("exact", Engine::Exact),
    ("local", Engine::Local { window: DEFAULT_LOCAL_WINDOW }),
    ("local:4", Engine::Local { window: 4 }),
];

/// Sets the engine `parse` uses: `global`, `exact`, `local` or `local:4`.
#[wasm_bindgen]
pub fn set_engine(name: &str) -> Result<(), JsValue> {
    let (_, engine) = ENGINES
        .iter()
        .find(|(n, _)| *n == name)
        .ok_or_else(|| JsValue::from_str("unknown engine"))?;
    ENGINE.set(*engine);
    Ok(())
}

fn options() -> ParseOptions {
    ParseOptions {
        engine: ENGINE.get(),
        ..ParseOptions::default()
    }
}

fn now_ms() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map_or(0.0, |p| p.now())
}

fn json_str(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Parses `text` with every engine, each with its own editing history (so
/// they see the same sequence of edits), and returns a JSON array with one
/// object per engine: `engine`, `iterations`, `commits`, `ms`, `errors`.
fn compare_engines<T>(root: T, language: &str, text: &str) -> String
where
    T: ParserTrait + 'static + Clone,
    for<'a> T::Kind: logos::Logos<'a, Source = str>,
    for<'a> <<T as ParserTrait>::Kind as logos::Logos<'a>>::Extras: Default,
{
    let mut rows = Vec::new();
    COMPARE_PREV.with(|prev| {
        let mut prev = prev.borrow_mut();
        for (i, (name, engine)) in ENGINES.iter().enumerate() {
            let key = (language.to_string(), i);
            let options = ParseOptions {
                engine: *engine,
                ..ParseOptions::default()
            };
            let t0 = now_ms();
            let (result, info) = parse_incremental_with(root.clone(), text, prev.get(&key), options);
            let ms = now_ms() - t0;
            let errors: Vec<String> = result.errors.iter().map(|e| json_str(e)).collect();
            rows.push(format!(
                "{{\"engine\":{},\"iterations\":{},\"commits\":{},\"ms\":{:.2},\"errors\":[{}]}}",
                json_str(name),
                last_iterations(),
                last_commits(),
                ms,
                errors.join(",")
            ));
            prev.insert(key, info);
        }
    });
    format!("[{}]", rows.join(","))
}

/// `compare_engines` for `language`.
#[wasm_bindgen]
pub fn compare(language: &str, text: &str) -> Result<String, JsValue> {
    Ok(match language {
        "turtle" => compare_engines(Rule::new(SyntaxKind::TurtleDoc), language, text),
        "sparql" => compare_engines(SparqlRule::new(SparqlSyntaxKind::QueryUnit), language, text),
        "trig" => compare_engines(TrigRule::new(TrigSyntaxKind::TrigDoc), language, text),
        "ntriples" => {
            compare_engines(NTriplesRule::new(NTriplesSyntaxKind::NtriplesDoc), language, text)
        }
        "n3" => compare_engines(N3Rule::new(N3SyntaxKind::N3Doc), language, text),
        "jsonld" => compare_engines(JsonLdRule::new(JsonLdSyntaxKind::JsonldDoc), language, text),
        _ => return Err("Unknown language".into()),
    })
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
    iterations: u32,
    commits: u32,
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

    /// Search iterations of this parse.
    #[wasm_bindgen(getter)]
    pub fn iterations(&self) -> u32 {
        self.iterations
    }

    /// Commits of this parse (local engine only).
    #[wasm_bindgen(getter)]
    pub fn commits(&self) -> u32 {
        self.commits
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
        parse_incremental_with(root, text, p.as_ref(), options())
    };
    let (iterations, commits) = (last_iterations() as u32, last_commits() as u32);
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
        iterations,
        commits,
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
                    parse_incremental_with(
                        JsonLdRule::new(JsonLdSyntaxKind::JsonldDoc),
                        text,
                        p.as_ref(),
                        options(),
                    )
                };
                let pairs = get_error_range_pairs::<JsonLdLang>(&parse);
                *prev.borrow_mut() = Some(new_prev);
                (parse, pairs)
            });
            let (iterations, commits) = (last_iterations() as u32, last_commits() as u32);
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
                iterations,
                commits,
            }
        }
        _ => return Err("Unknown language".into()),
    })
}
