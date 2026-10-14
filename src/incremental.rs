//! What a parse leaves behind for the next one: its steps and checkpoints of
//! the parser state along the way, as plain data (no language types, no
//! `Rc`), so `PrevParseInfo` stays `Send + Sync` and language-agnostic.
//!
//! The next parse uses them to skip work: it resumes from a checkpoint before
//! the edit instead of starting at the root (prefix reuse), and when it
//! reaches the same parser state as the old parse at a token after the edit,
//! it completes with the old steps instead of parsing the rest (suffix reuse).

use std::collections::HashMap;

use crate::{Error, Fingerprint, Step, TokenTrait, list::List};

/// A `Step` without the language's kind type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RawStep {
    Start(rowan::SyntaxKind),
    End,
    Bump(Fingerprint),
    Error(rowan::SyntaxKind),
    Delete,
}

impl RawStep {
    pub(crate) fn from_step<T: TokenTrait>(step: &Step<T>) -> Self {
        match step {
            Step::Start(k) => RawStep::Start(k.clone().into()),
            Step::End => RawStep::End,
            Step::Bump(fp) => RawStep::Bump(*fp),
            Step::Error(Error::Expected(k)) => RawStep::Error(*k),
            Step::Delete => RawStep::Delete,
        }
    }

    pub(crate) fn to_step<T: TokenTrait>(self) -> Step<T> {
        match self {
            RawStep::Start(k) => Step::Start(T::from(k)),
            RawStep::End => Step::End,
            RawStep::Bump(fp) => Step::Bump(fp),
            RawStep::Error(k) => Step::Error(Error::Expected(k)),
            RawStep::Delete => Step::Delete,
        }
    }

    fn is_error(self) -> bool {
        matches!(self, RawStep::Error(_) | RawStep::Delete)
    }
}

/// The parser state at a token on the chosen parse path.
#[derive(Clone, Debug)]
pub(crate) struct Checkpoint {
    /// Index into the token list (trivia included) of the next token.
    pub token: usize,
    /// Number of steps taken before this point.
    pub steps: usize,
    /// Path cost at this point.
    pub cost: isize,
    pub fp: Fingerprint,
    pub depth: i8,
    /// Rule stack, innermost first: (rule kind, rule state, saved state
    /// fingerprint, role fingerprint).
    pub stack: Vec<(rowan::SyntaxKind, u32, Fingerprint, Fingerprint)>,
    /// No error steps before / after this point.
    pub clean_before: bool,
    pub clean_after: bool,
}

/// Steps and checkpoints of a finished parse.
#[derive(Clone, Debug, Default)]
pub(crate) struct Memo {
    pub steps: Vec<RawStep>,
    pub checkpoints: Vec<Checkpoint>,
    pub cost: isize,
}

impl Memo {
    /// Builds a memo from the final steps (in order) and the checkpoints
    /// taken along the way; fills in the `clean_*` flags.
    pub(crate) fn new(steps: Vec<RawStep>, mut checkpoints: Vec<Checkpoint>, cost: isize) -> Self {
        let first_error = steps.iter().position(|s| s.is_error()).unwrap_or(usize::MAX);
        let last_error = steps.iter().rposition(|s| s.is_error());
        for cp in &mut checkpoints {
            cp.clean_before = cp.steps <= first_error;
            cp.clean_after = last_error.is_none_or(|e| e < cp.steps);
        }
        Memo {
            steps,
            checkpoints,
            cost,
        }
    }
}

/// How close (in token indices, trivia included) a resume checkpoint may be
/// to the first edited token.  The old parse may have looked a little ahead
/// to choose its path, so the last few tokens before an edit are reparsed.
const RESUME_MARGIN: usize = 16;

/// Which parts of the previous parse a new parse can reuse.
pub(crate) struct Reuse<'p> {
    pub memo: &'p Memo,
    /// Index of the checkpoint to resume from.
    pub resume: Option<usize>,
    /// New token index → checkpoints whose old suffix can be spliced in
    /// there, if the parser is in the same state.
    pub splice: HashMap<usize, Vec<usize>>,
    /// New token index minus old token index after the edit.
    pub delta: isize,
}

impl<'p> Reuse<'p> {
    /// Plans reuse from the token texts of the old and new document.
    pub(crate) fn plan(memo: &'p Memo, old: &[&str], new: &[&str]) -> Option<Self> {
        if memo.checkpoints.is_empty() {
            return None;
        }
        let prefix = old.iter().zip(new).take_while(|(a, b)| a == b).count();
        let max_suffix = old.len().min(new.len()) - prefix;
        let suffix = old
            .iter()
            .rev()
            .zip(new.iter().rev())
            .take(max_suffix)
            .take_while(|(a, b)| a == b)
            .count();
        let delta = new.len() as isize - old.len() as isize;
        let old_tail = old.len() - suffix;

        let resume = memo
            .checkpoints
            .iter()
            .rposition(|cp| cp.clean_before && cp.token + RESUME_MARGIN <= prefix);
        let resume_token = resume.map_or(0, |i| memo.checkpoints[i].token);

        let mut splice: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, cp) in memo.checkpoints.iter().enumerate() {
            let at = cp.token as isize + delta;
            if cp.token >= old_tail && at > resume_token as isize {
                splice.entry(at as usize).or_default().push(i);
            }
        }

        if resume.is_none() && splice.is_empty() {
            return None;
        }
        Some(Reuse {
            memo,
            resume,
            splice,
            delta,
        })
    }

    /// The old steps up to checkpoint `cp`, as a step list.
    pub(crate) fn prefix<T: TokenTrait>(&self, cp: usize) -> List<Step<T>> {
        self.memo.steps[..self.memo.checkpoints[cp].steps]
            .iter()
            .fold(List::default(), |list, s| list.prepend(s.to_step()))
    }
}
