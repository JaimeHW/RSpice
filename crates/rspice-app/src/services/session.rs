//! What the session keeps between starts: the outcome of its last save and
//! the autosave interval a session starts with.

/// What the last save of the session kept for the next start. Transient.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) enum SessionKept {
    /// All of it, or nothing has been saved yet.
    #[default]
    Whole,
    /// All but the runs, which the store refused. Holds the run history they
    /// were refused for, so later saves do not encode the same runs again
    /// only to have them refused.
    WithoutRuns(crate::state::RunHistoryRevision),
    /// None of it: the store refused even the session without its runs.
    Nothing,
}

/// The autosave interval a session starts with when nothing chose one.
pub(crate) const DEFAULT_AUTOSAVE_MINUTES: u8 = 5;
