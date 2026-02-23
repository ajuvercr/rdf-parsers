use crate::{Expr, Mark, Rule, parser::LiteralType};
use regex::{Captures, Regex};
use std::collections::{BTreeSet, HashMap};

fn xml_range_to_logos_class(input: &str) -> Result<String, String> {
    let item_re = Regex::new(r"#x([0-9A-Fa-f]{1,6})").map_err(|e| e.to_string())?;

    let input = input.replace('\\', "\\\\");
    // Replace each `#x...` with `\x{...}`.
    // Note: we return a String so we can include backslashes safely.
    let out = item_re.replace_all(&input, |caps: &Captures| {
        let hex = &caps[1];
        format!(r"\x{{{hex}}}")
    });

    Ok(out.into_owned())
}

fn char_class(chars: &str) -> String {
    let mut out = String::from("");
    for c in chars.chars() {
        match c {
            // Escape common metacharacters anyway (safe and readable)
            '.' | '*' | '+' | '?' | '(' | ')' | '{' | '}' | '|' | '$' | '\\' | '[' | ']' => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

pub fn to_regex(e: &Expr) -> String {
    match e {
        Expr::Marked(expr, Mark::Option) => format!("({})?", to_regex(expr)),
        Expr::Marked(expr, Mark::Star) => format!("({})*", to_regex(expr)),
        Expr::Marked(expr, Mark::Plus) => format!("({})+", to_regex(expr)),
        Expr::Either(exprs) => {
            let mut o = to_regex(&exprs[0]);
            for e in &exprs[1..] {
                o += "|";
                o += &to_regex(e);
            }

            format!("({})", o)
        }
        Expr::Seq(exprs) => {
            let mut o = String::new();
            for e in exprs {
                o += &to_regex(e);
            }

            o
        }
        Expr::Literal(LiteralType::Hex, e) => match e.len() {
            x if x > 6 => format!("\\U{:0>8}", e),
            x if x > 4 => format!("\\U{:0>6}", e),
            _ => format!("\\u{:0>4}", e),
        },
        Expr::Literal(LiteralType::Regex, e) => {
            format!("[{}]", xml_range_to_logos_class(&e).unwrap())
        }
        Expr::Literal(_, e) => char_class(&e),
        // Expr::Literal(_, e) => e.to_string(),
        Expr::Reference(e) => format!("(?&{})", e),
    }
}

/// Collect all referenced rule-names inside an Expr.
fn referenced_names(expr: &Expr, out: &mut BTreeSet<String>) {
    match expr {
        Expr::Marked(inner, _) => referenced_names(inner, out),
        Expr::Either(xs) | Expr::Seq(xs) => {
            for x in xs {
                referenced_names(x, out);
            }
        }
        Expr::Literal(_, _) => {}
        Expr::Reference(name) => {
            out.insert(name.clone());
        }
    }
}

/// Return rules in an order where dependencies come first.
/// - If a rule references a name not present in `rules`, we *ignore* it here (optionally error).
/// - If there is a cycle, returns an error with the cycle path.
pub fn order_rules_by_references(rules: &[Rule]) -> Result<Vec<Rule>, String> {
    // name -> index (assuming names are unique)
    let mut by_name: HashMap<&str, usize> = HashMap::new();
    for (i, r) in rules.iter().enumerate() {
        if by_name.insert(r.name.as_str(), i).is_some() {
            return Err(format!("Duplicate rule name: {}", r.name));
        }
    }

    // Build dependency list: rule i depends on set of indices it references.
    let mut deps: Vec<Vec<usize>> = vec![Vec::new(); rules.len()];
    for (i, r) in rules.iter().enumerate() {
        let mut refs = BTreeSet::new();
        referenced_names(&r.expression, &mut refs);

        let mut list = Vec::new();
        for name in refs {
            if let Some(&j) = by_name.get(name.as_str()) {
                list.push(j);
            } else {
                // If you want unknown references to be an error, flip this on:
                // return Err(format!("Rule '{}' references unknown rule '{}'", r.name, name));
            }
        }
        deps[i] = list;
    }

    // DFS topo sort with cycle detection.
    #[derive(Copy, Clone, Eq, PartialEq)]
    enum State {
        NotVisited,
        Visiting,
        Done,
    }

    let mut state = vec![State::NotVisited; rules.len()];
    let mut stack: Vec<usize> = Vec::new();
    let mut output: Vec<usize> = Vec::with_capacity(rules.len());

    fn visit(
        v: usize,
        rules: &[Rule],
        deps: &[Vec<usize>],
        state: &mut [State],
        stack: &mut Vec<usize>,
        output: &mut Vec<usize>,
    ) -> Result<(), String> {
        match state[v] {
            State::Done => return Ok(()),
            State::Visiting => {
                // cycle: extract it from stack
                let pos = stack.iter().rposition(|&x| x == v).unwrap_or(0);
                let cycle = stack[pos..]
                    .iter()
                    .map(|&i| rules[i].name.clone())
                    .collect::<Vec<_>>();
                return Err(format!("Cycle detected: {}", cycle.join(" -> ")));
            }
            State::NotVisited => {}
        }

        state[v] = State::Visiting;
        stack.push(v);

        for &d in &deps[v] {
            visit(d, rules, deps, state, stack, output)?;
        }

        stack.pop();
        state[v] = State::Done;
        output.push(v);
        Ok(())
    }

    for i in 0..rules.len() {
        if state[i] == State::NotVisited {
            visit(i, rules, &deps, &mut state, &mut stack, &mut output)?;
        }
    }

    // `output` is in dependency-first order already because we push after visiting deps.
    // But it may contain duplicates if we weren't careful — here it won't.
    Ok(output.into_iter().map(|i| rules[i].clone()).collect())
}

/// Transform Rules into “statements” after ordering.
/// You provide `generate_statement(rule) -> String`.
pub fn rules_to_statements(
    rules: &[Rule],
    generate_statement: impl Fn(&Rule) -> String,
) -> Result<Vec<String>, String> {
    let ordered = order_rules_by_references(rules)?;
    Ok(ordered.iter().map(generate_statement).collect())
}
