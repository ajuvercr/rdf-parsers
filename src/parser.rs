#[allow(unused)]
#[derive(Debug)]
pub struct FatToken<T: TokenTrait> {
    pub kind: T,
    text: String,
    pub range: Range<usize>,
    old_kind: Option<crate::Fingerprint>,
    /// Bracket nesting depth from the previous parse at which this token was
    /// parsed.  `None` for new (inserted) tokens.
    old_depth: Option<u8>,
}
impl<T: TokenTrait> FatToken<T> {
    pub fn new(kind: T, range: Range<usize>, text: String) -> Self {
        FatToken {
            kind,
            range,
            text,
            old_kind: None,
            old_depth: None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn old_kind(&self) -> Option<crate::Fingerprint> {
        self.old_kind
    }

    pub fn set_old_kind(&mut self, kind: Option<crate::Fingerprint>) {
        self.old_kind = kind;
    }

    pub fn old_depth(&self) -> Option<u8> {
        self.old_depth
    }

    pub fn set_old_depth(&mut self, depth: Option<u8>) {
        self.old_depth = depth;
    }

    pub fn to_prev_token(&self, current_depth: u8) -> crate::PrevToken {
        crate::PrevToken {
            text: self.text.clone(),
            fingerprint: self.old_kind,
            depth: current_depth,
        }
    }
}

/// Collapse rule nodes that consumed no tokens into a single `Expected(RuleName)` error.
///
/// When a grammar rule fires but consumes no tokens and has errors, and at least one
/// ancestor rule has already consumed something, the whole rule is replaced by a
/// single `Expected(rule_kind)` error.  Collapsing is allowed to cascade outward:
/// each successive `out.truncate` erases the inner collapse so only the outermost
/// still-empty rule survives.  This naturally surfaces the most specific grammar
/// rule that the user should care about — e.g. `Expected(Verb)` instead of
/// `Expected(Alit)` when the verb is absent, or `Expected(PredicateObjectList)`
/// when both verb and object are absent.
fn coalesce_empty_rules<T: crate::TokenTrait>(steps: Vec<Step<T>>) -> Vec<Step<T>> {
    let mut out: Vec<Step<T>> = Vec::with_capacity(steps.len());
    // Stack entry: (kind, start_idx_in_out, has_bump, has_error)
    let mut stack: Vec<(T, usize, bool, bool)> = Vec::new();

    for step in steps {
        match step {
            Step::Start(ref kind) => {
                stack.push((kind.clone(), out.len(), false, false));
                out.push(step);
            }
            Step::Bump(_) | Step::Delete => {
                for entry in &mut stack {
                    entry.2 = true;
                }
                out.push(step);
            }
            Step::Error(_) => {
                for entry in &mut stack {
                    entry.3 = true;
                }
                out.push(step);
            }
            Step::End => {
                let (kind, start_pos, has_bump, has_error) = stack.pop().expect("unmatched End");
                let any_ancestor_has_bump = stack.iter().any(|e| e.2);

                if !has_bump && has_error && any_ancestor_has_bump {
                    // Rule consumed no tokens: collapse into a single Expected(rule_kind)
                    // error.  The truncate erases any inner collapses, so this rule's name
                    // takes precedence over more specific (but noisier) inner names.
                    out.truncate(start_pos);
                    out.push(Step::Start(kind.clone()));
                    out.push(Step::Error(Error::Expected(kind.into())));
                    out.push(Step::End);
                } else {
                    out.push(Step::End);
                }
            }
        }
    }
    out
}

pub struct Parse {
    pub green_node: GreenNode,
    pub errors: List<String>,
    pub suggestions: HashSet<(String, Range<usize>)>,
}
impl Parse {
    pub fn expected_at<L>(
        &self,
        byte_offset: usize,
        first_tokens: fn(L::Kind) -> &'static [L::Kind],
    ) -> Vec<L::Kind>
    where
        L: rowan::Language,
        L::Kind: Eq + std::hash::Hash + Copy,
    {
        use rowan::{TextSize, TokenAtOffset};
        let root = self.syntax::<L>();
        let offset = TextSize::new(byte_offset as u32);

        let (current_kind, start_node) = match root.token_at_offset(offset) {
            TokenAtOffset::None => (None, None),
            TokenAtOffset::Single(t) => (Some(t.kind()), t.parent()),
            TokenAtOffset::Between(_, right) => (Some(right.kind()), right.parent()),
        };

        let mut result = HashSet::new();
        let mut node = start_node;
        while let Some(n) = node {
            result.extend(first_tokens(n.kind()).iter().copied());
            node = n.parent();
        }

        if let Some(k) = current_kind {
            result.remove(&k);
        }

        result.into_iter().collect()
    }

    pub fn from_steps<T: crate::TokenTrait>(
        tokens: &mut [FatToken<T>],
        steps: List<Step<T>>,
    ) -> Self {
        let mut at = 0;
        // (token_index, fingerprint, bracket_depth_at_bump)
        let mut fingerprint_assignments: Vec<(usize, crate::Fingerprint, u8)> = Vec::new();
        let skip_white_with_builder = |builder: &mut GreenNodeBuilder<'_>, at: &mut usize| {
            while let Some(t) = tokens.get(*at)
                && t.kind.skips()
            {
                builder.token(t.kind.clone().into(), &t.text);
                *at += 1;
            }
        };

        let mut error_msgs: Vec<String> = Vec::new();
        let mut builder = GreenNodeBuilder::new();
        builder.start_node(T::ROOT.into());
        let step_count = steps.len();
        let steps_vec: Vec<_> = {
            let mut v = Vec::with_capacity(step_count);
            v.extend(steps.iter().cloned());
            // Drop the list iteratively to avoid stack overflow on long lists.
            crate::list::drop_list(steps);
            v
        };
        let steps = coalesce_empty_rules(steps_vec.into_iter().rev().collect());
        // Running bracket nesting depth: incremented on openers, decremented on closers.
        let mut current_depth: i32 = 0;
        for step in steps.into_iter() {
            match step {
                Step::Start(kind) => {
                    skip_white_with_builder(&mut builder, &mut at);
                    builder.start_node(kind.into());
                }
                Step::End => builder.finish_node(),
                Step::Error(e) => {
                    // Skip whitespace so the zero-width error node is positioned at
                    // the first non-whitespace byte where the missing token was expected,
                    // not at the end of the previously-consumed token.
                    skip_white_with_builder(&mut builder, &mut at);
                    builder.start_node(T::ERROR.into());
                    // Convert the rowan::SyntaxKind back to the language-specific T
                    // for a human-readable debug name (e.g. "Expected(Verb)" rather
                    // than "Expected(SyntaxKind(42))").
                    let Error::Expected(raw_kind) = e;
                    let lang_kind: T = raw_kind.into();
                    error_msgs.push(format!("Expected({:?})", lang_kind));
                    builder.finish_node();
                }
                Step::Bump(fp) => {
                    skip_white_with_builder(&mut builder, &mut at);
                    if let Some(i) = tokens.get(at) {
                        // Record depth at the token's position (before adjusting for
                        // this token's own bracket_delta, so openers record the outer
                        // depth and closers record the inner depth).
                        let depth_at_token = current_depth.clamp(0, 255) as u8;
                        fingerprint_assignments.push((at, fp, depth_at_token));
                        current_depth += i.kind.bracket_delta() as i32;
                        builder.token(i.kind.clone().into(), &i.text);
                        at += 1;
                    }
                }
                Step::Delete => {
                    skip_white_with_builder(&mut builder, &mut at);
                    if let Some(i) = tokens.get(at) {
                        builder.start_node(T::ERROR.into());
                        builder.token(i.kind.clone().into(), &i.text);
                        builder.finish_node();
                        error_msgs.push(format!("Unexpected({:?})", i.kind));
                        // Do NOT adjust current_depth — the deleted token is
                        // being discarded, not consumed into the grammar.
                        at += 1;
                    }
                }
            }
        }
        skip_white_with_builder(&mut builder, &mut at);

        // Wrap any remaining unconsumed non-whitespace tokens in Error nodes.
        // This happens when the A* search timed out and returned a partial
        // parse that didn't consume the full input.
        while at < tokens.len() {
            let tok = &tokens[at];
            if tok.kind.skips() {
                builder.token(tok.kind.clone().into(), &tok.text);
                at += 1;
            } else {
                builder.start_node(T::ERROR.into());
                builder.token(tok.kind.clone().into(), &tok.text);
                builder.finish_node();
                error_msgs.push(format!("Unparsed({:?})", tok.kind));
                at += 1;
            }
        }

        for (idx, fp, depth) in fingerprint_assignments {
            if let Some(tok) = tokens.get_mut(idx) {
                tok.set_old_kind(Some(fp));
                tok.set_old_depth(Some(depth));
            }
        }

        builder.finish_node();

        // Build errors in forward document order: error_msgs was collected left-to-right
        // (first error in document first), so fold in reverse to build a prepend-only List
        // whose head is the first error.
        let errors = error_msgs
            .into_iter()
            .rfold(List::default(), |acc, msg| acc.prepend(msg));

        Parse {
            green_node: builder.finish(),
            errors,
            suggestions: HashSet::new(),
        }
    }
}

impl Parse {
    pub fn syntax<L: Language>(&self) -> rowan::SyntaxNode<L> {
        rowan::SyntaxNode::new_root(self.green_node.clone())
    }
}

/// Returns the effective byte span of an error node.
///
/// Error nodes in the CST are zero-width: they sit at the byte position of the
/// missing token but span no characters themselves.  The *effective* span
/// widens that point in both directions to include skippable tokens (whitespace /
/// trivia) that immediately surround it — the "dead zone" between the adjacent
/// real tokens.  This gives callers a non-empty range suitable for highlighting.
///
/// Specifically the span is `leading_gap_start..trailing_gap_end`, where:
/// - `leading_gap_start` is the end of the last non-skippable token *before* the
///   error (or `0` if the error is at the start of the file).
/// - `trailing_gap_end` is the start of the first non-skippable token at or
///   after the error position (extended through any immediately following
///   skippable tokens).
///
/// # Example
///
/// For input `"  <b> <c> }"` with a missing `{`, the error node sits at byte 2
/// (after the two leading spaces are consumed as trivia by the `Start` step
/// preceding the error).  The effective span is `0..2`, covering those two bytes.
///
/// For input `"<a> <b> <c>"` missing a trailing `.`, where the error appears at
/// the end (byte 11) with no trailing whitespace, the span is `11..11` (zero-width).
pub fn effective_error_span<L>(error_node: &rowan::SyntaxNode<L>) -> Range<usize>
where
    L: Language,
    L::Kind: TokenTrait,
{
    use rowan::NodeOrToken;

    let error_start: usize = error_node.text_range().start().into();

    let root = error_node
        .ancestors()
        .last()
        .unwrap_or_else(|| error_node.clone());

    let mut span_start = 0usize;
    let mut span_end = error_start;
    let mut past_error = false;

    for elem in root.descendants_with_tokens() {
        let NodeOrToken::Token(tok) = elem else {
            continue;
        };
        let tok_start: usize = tok.text_range().start().into();

        if !past_error {
            if tok_start >= error_start {
                // Transition to trailing side — handle this token below without
                // skipping it so span_end is computed from the right position.
                past_error = true;
            } else {
                if !tok.kind().skips() {
                    span_start = tok.text_range().end().into();
                }
                continue;
            }
        }

        // Trailing side: stop at the first non-skippable token and record its
        // start as span_end; extend through consecutive skippable tokens.
        if !tok.kind().skips() {
            span_end = tok_start;
            break;
        }
        span_end = tok.text_range().end().into();
    }

    span_start..span_end
}

use std::{collections::HashSet, ops::Range};

use rowan::{GreenNode, GreenNodeBuilder, Language};

use crate::{TokenTrait, list::List};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Expected(rowan::SyntaxKind),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Step<T> {
    Start(T),
    Error(Error),
    End,
    Bump(crate::Fingerprint),
    /// Delete (skip) the current token. The token still appears in the CST
    /// wrapped in an Error node, but no grammar rule consumes it.
    Delete,
}
impl<T: TokenTrait> Step<T> {
    pub fn start(kind: T) -> Self {
        Step::Start(kind)
    }
    pub fn error(error: Error) -> Self {
        Self::Error(error)
    }

    pub fn end() -> Self {
        Step::End
    }
}
