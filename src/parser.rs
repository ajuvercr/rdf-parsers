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
/// Only the outermost failing rule under a successfully-parsed ancestor is collapsed:
/// if a parent rule also has no bumps, it defers to the grandparent, and so on.
/// This prevents showing dozens of low-level "Expected(Iriref)" errors when an
/// entire `PredicateObjectList` is simply absent.
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
            Step::Bump(_) => {
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
                // Coalesce only when:
                // - this rule produced no real tokens (has_bump = false)
                // - it contains at least one error (has_error = true)
                // - its parent has real tokens (parent.has_bump = true), meaning
                //   the parent will NOT itself be coalesced at a higher level.
                let parent_has_bump = stack.last().map(|e| e.2).unwrap_or(false);
                if !has_bump && has_error && parent_has_bump {
                    out.truncate(start_pos);
                    out.push(Step::Start(kind.clone()));
                    out.push(Step::Error(Error::Expected(kind)));
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
                    error_msgs.push(format!("{:?}", e));
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
            }
        }
        skip_white_with_builder(&mut builder, &mut at);

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
/// widens that point to include the run of skippable tokens (whitespace /
/// trivia) that immediately precede it — the "dead zone" between the last real
/// token and where the missing one was expected.  This gives callers a
/// non-empty range suitable for highlighting in an editor.
///
/// # Example
///
/// For input `"  <b> <c> }"` with a missing `{`, the error node sits at byte
/// 2 (after the two leading spaces are consumed by the parser).  The effective
/// span is `0..2`, covering those two whitespace bytes.
///
/// If there is no preceding whitespace the function returns a zero-width range
/// at the error position, identical to the node's own `text_range()`.
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
    let span_end: usize = error_node.text_range().end().into();
    let mut iter = root.descendants_with_tokens();
    for elem in &mut iter {
        let NodeOrToken::Token(tok) = elem else {
            continue;
        };
        let tok_start: usize = tok.text_range().start().into();
        if tok_start >= error_start {
            break;
        }
        if !tok.kind().skips() || tok.text_range().is_empty() {
            let s: usize = tok.text_range().end().into();
            span_start = s + 1;
        }
    }

    span_start..span_end
}

use std::{collections::HashSet, ops::Range};

use rowan::{GreenNode, GreenNodeBuilder, Language};

use crate::{TokenTrait, list::List};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error<T> {
    Expected(T),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Step<T> {
    Start(T),
    Error(Error<T>),
    End,
    Bump(crate::Fingerprint),
}
impl<T: TokenTrait> Step<T> {
    pub fn start(kind: T) -> Self {
        Step::Start(kind)
    }
    pub fn error(error: Error<T>) -> Self {
        Self::Error(error)
    }

    pub fn end() -> Self {
        Step::End
    }
}
