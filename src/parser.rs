#[allow(unused)]
#[derive(Debug)]
pub struct FatToken<T: TokenTrait> {
    pub kind: T,
    text: String,
    pub range: Range<usize>,
    old_kind: Option<crate::Fingerprint>,
}
impl<T: TokenTrait> FatToken<T> {
    pub fn new(kind: T, range: Range<usize>, text: String) -> Self {
        FatToken {
            kind,
            range,
            text,
            old_kind: None,
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

    pub fn to_prev_token(&self) -> crate::PrevToken {
        crate::PrevToken {
            text: self.text.clone(),
            fingerprint: self.old_kind,
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
        let mut fingerprint_assignments: Vec<(usize, crate::Fingerprint)> = Vec::new();
        let skip_white_with_builder = |builder: &mut GreenNodeBuilder<'_>, at: &mut usize| {
            while let Some(t) = tokens.get(*at)
                && t.kind.skips()
            {
                builder.token(t.kind.clone().into(), &t.text);
                *at += 1;
            }
        };

        let mut errors = List::default();
        let mut builder = GreenNodeBuilder::new();
        builder.start_node(T::ROOT.into());
        let step_count = steps.len();
        let steps: Vec<_> = {
            let mut v = Vec::with_capacity(step_count);
            v.extend(steps.iter().cloned());
            v
        };
        let steps = coalesce_empty_rules(steps.into_iter().rev().collect());
        for step in steps.into_iter() {
            match step {
                Step::Start(kind) => {
                    skip_white_with_builder(&mut builder, &mut at);
                    builder.start_node(kind.into());
                }
                Step::End => builder.finish_node(),
                Step::Error(e) => {
                    builder.start_node(T::ERROR.into());
                    errors = errors.prepend(format!("{:?}", e));
                    builder.finish_node();
                }
                Step::Bump(fp) => {
                    skip_white_with_builder(&mut builder, &mut at);
                    if let Some(i) = tokens.get(at) {
                        fingerprint_assignments.push((at, fp));
                        builder.token(i.kind.clone().into(), &i.text);
                        at += 1;
                    }
                }
            }
        }
        skip_white_with_builder(&mut builder, &mut at);

        for (idx, fp) in fingerprint_assignments {
            if let Some(tok) = tokens.get_mut(idx) {
                tok.set_old_kind(Some(fp));
            }
        }

        builder.finish_node();

        Parse {
            green_node: builder.finish(),
            errors: errors,
            suggestions: HashSet::new(),
        }
    }
}

impl Parse {
    pub fn syntax<L: Language>(&self) -> rowan::SyntaxNode<L> {
        rowan::SyntaxNode::new_root(self.green_node.clone())
    }
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
