//! Differential check: print errors for mutated real files, one line per case.
//!     cargo run --release --example mutate <lang> <files...>
use rdf_parsers::{TokenTrait, tokenize};

fn rng(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

macro_rules! run {
    ($lang:ident, $unit:ident, $files:expr) => {{
        use rdf_parsers::$lang::{Rule, SyntaxKind};
        for f in $files {
            let Ok(src) = std::fs::read_to_string(&f) else { continue };
            let toks = tokenize::<SyntaxKind>(&src);
            let real: Vec<_> = toks.iter().filter(|t| !t.kind.skips()).collect();
            if real.len() < 5 || real.len() > 400 {
                continue;
            }
            let mut seed = 0x9E3779B97F4A7C15u64 ^ src.len() as u64;
            for m in 0..4 {
                let t = real[rng(&mut seed) as usize % real.len()];
                let r = t.range.clone();
                let text = if m % 2 == 0 {
                    format!("{}{}", &src[..r.start], &src[r.end..])
                } else {
                    format!("{}{} {}", &src[..r.end], &src[r.start..r.end], &src[r.end..])
                };
                let start = std::time::Instant::now();
                let (p, _) = rdf_parsers::parse(Rule::new(SyntaxKind::$unit), &text);
                let el = start.elapsed().as_millis();
                let errs: Vec<_> = p.errors.iter().cloned().collect();
                println!("{f}#{m}\t{}\t{el}\t{}", errs.len(), errs.join(","));
            }
        }
    }};
}

fn main() {
    let mut args = std::env::args().skip(1);
    let lang = args.next().unwrap();
    let files: Vec<String> = args.collect();
    match lang.as_str() {
        "turtle" => run!(turtle, TurtleDoc, files),
        "sparql" => run!(sparql, QueryUnit, files),
        _ => panic!("lang"),
    }
}
