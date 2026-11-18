//! The browser's session, kept in its durable records rather than in one
//! local-storage string: the session itself (its layout and selections, and
//! the workspace they are over) in one record, and the runs it holds in
//! another, written only when they changed.
//!
//! The session is written when the page is left, never between frames. When
//! the browser's storage is full the project's autosave comes first: the runs
//! give way to it, and what was not kept is said once, when it starts.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::services::session::SessionKept;
use crate::state::RunHistoryRevision;
use crate::workbench::lifecycle::durable_records::{DurableRecords, Record};
use crate::workbench::lifecycle::project_lifecycle::PlatformState;
use rspice_app_state::workbench::app_state::model::Model;

/// The record that holds the session.
pub const SESSION_KEY: &str = "session";
/// The record that holds the session's runs.
pub const RUNS_KEY: &str = "runs";

/// What a start reads back.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StoredSession {
    pub session: Option<String>,
    pub runs: Option<String>,
}

/// Read the session and its runs back. A store that cannot be read is a
/// session started afresh, said in the log.
pub async fn read_session(records: &dyn DurableRecords) -> StoredSession {
    let read = |key: &'static str| async move {
        match records.get(key).await {
            Ok(record) => record.and_then(Record::into_text),
            Err(error) => {
                log::warn!("the stored {key} could not be read: {error}");
                None
            }
        }
    };
    let session = read(SESSION_KEY).await;
    // Runs without the session they belong to are never restored.
    let runs = match session {
        Some(_) => read(RUNS_KEY).await,
        None => None,
    };
    StoredSession { session, runs }
}

/// What becomes of the stored runs when the session is written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunsToKeep {
    /// The stored runs are the session's: nothing to write.
    Unchanged,
    /// No runs are kept: the session has none, or they gave way.
    Removed,
    /// The session's runs, encoded.
    Write(String),
}

/// What the runs are, for the session about to be written: unchanged since
/// the last write that kept them, left out since they gave way at this very
/// history, or to be encoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunsDecision {
    Unchanged,
    Removed,
    Encode,
}

/// Decide what the session write does with the runs at `history`.
pub fn runs_decision(
    history: &RunHistoryRevision,
    has_runs: bool,
    kept: &SessionKept,
    stored: Option<&RunHistoryRevision>,
) -> RunsDecision {
    if stored == Some(history) {
        RunsDecision::Unchanged
    } else if !has_runs || *kept == SessionKept::WithoutRuns(history.clone()) {
        RunsDecision::Removed
    } else {
        RunsDecision::Encode
    }
}

/// What a session write kept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionWritten {
    Whole,
    /// The session without its runs, which the store refused; the runs it
    /// held before are gone too, so they never come back with a later
    /// session.
    WithoutRuns,
    /// Nothing: the store refused the session, for the reason given.
    Nothing(String),
}

/// Write the session, then its runs. A session the store refuses keeps
/// the one stored before, with its runs; runs it refuses leave the new
/// session without any.
pub async fn keep_session(
    records: &dyn DurableRecords,
    session: String,
    runs: RunsToKeep,
) -> SessionWritten {
    if let Err(error) = records.put(SESSION_KEY, Record::Text(session)).await {
        return SessionWritten::Nothing(error.to_string());
    }
    let removed = match runs {
        RunsToKeep::Unchanged => return SessionWritten::Whole,
        RunsToKeep::Removed => records.delete(RUNS_KEY).await,
        RunsToKeep::Write(text) => match records.put(RUNS_KEY, Record::Text(text)).await {
            Ok(()) => return SessionWritten::Whole,
            Err(refused) => {
                log::warn!("the session's runs were not kept: {refused}");
                return match records.delete(RUNS_KEY).await {
                    Ok(()) => SessionWritten::WithoutRuns,
                    Err(error) => SessionWritten::Nothing(error.to_string()),
                };
            }
        },
    };
    match removed {
        Ok(()) => SessionWritten::Whole,
        Err(error) => SessionWritten::Nothing(format!(
            "the runs it no longer holds could not be removed: {error}"
        )),
    }
}

/// A session write the browser finished, for the frame to settle.
#[derive(Debug, Clone)]
pub struct FinishedWrite {
    pub written: SessionWritten,
    /// The run history the write was taken at.
    pub history: RunHistoryRevision,
    pub runs: RunsDecision,
}

thread_local! {
    static FINISHED: std::cell::RefCell<Vec<FinishedWrite>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// Leave a finished write for the next frame.
pub fn finished(write: FinishedWrite) {
    FINISHED.with(|finished| finished.borrow_mut().push(write));
}

/// Settle every session write the browser finished: remember which runs
/// the store holds, and say what was not kept when that starts.
pub fn settle_finished(model: &mut Model, platform: &mut PlatformState) {
    let writes = FINISHED.with(|finished| std::mem::take(&mut *finished.borrow_mut()));
    for write in writes {
        let kept = match write.written {
            SessionWritten::Whole => {
                platform.session_runs_stored = match write.runs {
                    RunsDecision::Unchanged => platform.session_runs_stored.take(),
                    RunsDecision::Removed => None,
                    RunsDecision::Encode => Some(write.history.clone()),
                };
                if write.runs == RunsDecision::Removed
                    && platform.session_kept == SessionKept::WithoutRuns(write.history.clone())
                {
                    // Still left out for the history they gave way at.
                    continue;
                }
                SessionKept::Whole
            }
            SessionWritten::WithoutRuns => {
                platform.session_runs_stored = None;
                SessionKept::WithoutRuns(write.history)
            }
            SessionWritten::Nothing(reason) => {
                log::warn!("the session was not kept: {reason}");
                SessionKept::Nothing
            }
        };
        settle(model, platform, kept);
    }
}

/// The session's runs gave way to the project's autosave: the store holds
/// none, and none are written again until the runs change.
pub fn runs_gave_way(model: &mut Model, platform: &mut PlatformState) {
    platform.session_runs_stored = None;
    let history = model.simulation.retained.runs.revision();
    settle(model, platform, SessionKept::WithoutRuns(history));
}

/// Remember what this save kept, and say so when that changes.
pub fn settle(model: &mut Model, platform: &mut PlatformState, kept: SessionKept) {
    let before = std::mem::replace(&mut platform.session_kept, kept);
    match (&before, &platform.session_kept) {
        (SessionKept::Whole, SessionKept::Whole)
        | (SessionKept::WithoutRuns(_), SessionKept::WithoutRuns(_))
        | (SessionKept::Nothing, SessionKept::Nothing) => {}
        (_, SessionKept::Whole) => {
            model.record(
                LogSeverity::Info,
                LogSource::Studio,
                "This browser keeps the whole session again, run results included.",
            );
        }
        (_, SessionKept::WithoutRuns(_)) => model.report_warning(
            "Run results are not kept for the next session",
            "This browser's storage for RSpice is full, and the project and its autosave come \
             first, so a reload brings back the project without them. Save the project to keep \
             them, or free space in the browser's site data.",
        ),
        (_, SessionKept::Nothing) => model.report_failure(
            "The session could not be kept",
            "This browser's storage refused the layout and selections, so a reload returns to \
             the last session it kept. The project's autosave is kept apart from it; save the \
             project to keep its work.",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::lifecycle::durable_records::{MemoryOrigin, block_on};

    #[test]
    fn a_session_and_its_runs_are_kept_as_two_records() {
        let origin = MemoryOrigin::new(None);
        let records = origin.store();
        let written = block_on(keep_session(
            &*records,
            "{\"layout\":1}".to_owned(),
            RunsToKeep::Write("[1,2,3]".to_owned()),
        ));
        assert_eq!(written, SessionWritten::Whole);
        assert_eq!(
            block_on(read_session(&*records)),
            StoredSession {
                session: Some("{\"layout\":1}".to_owned()),
                runs: Some("[1,2,3]".to_owned()),
            }
        );
    }

    /// Runs the store refuses go, the ones it held before them too, and the
    /// session is kept: a reload brings back the session without runs, not
    /// with runs from another time.
    #[test]
    fn runs_the_store_refuses_leave_the_session_kept_without_any() {
        let origin = MemoryOrigin::new(None);
        let records = origin.store();
        block_on(keep_session(
            &*records,
            "old".to_owned(),
            RunsToKeep::Write("old runs".to_owned()),
        ));
        origin.set_quota(Some(origin.used() + 16));
        let written = block_on(keep_session(
            &*records,
            "new".to_owned(),
            RunsToKeep::Write("x".repeat(1_000)),
        ));
        assert_eq!(written, SessionWritten::WithoutRuns);
        assert_eq!(
            block_on(read_session(&*records)),
            StoredSession {
                session: Some("new".to_owned()),
                runs: None,
            }
        );
    }

    /// A session the store refuses leaves the one it kept before, with that
    /// session's runs.
    #[test]
    fn a_session_the_store_refuses_leaves_the_last_one_kept() {
        let origin = MemoryOrigin::new(None);
        let records = origin.store();
        block_on(keep_session(
            &*records,
            "old".to_owned(),
            RunsToKeep::Write("old runs".to_owned()),
        ));
        origin.set_quota(Some(origin.used()));
        let written = block_on(keep_session(
            &*records,
            "a much longer session".to_owned(),
            RunsToKeep::Removed,
        ));
        assert!(matches!(written, SessionWritten::Nothing(_)), "{written:?}");
        assert_eq!(
            block_on(read_session(&*records)),
            StoredSession {
                session: Some("old".to_owned()),
                runs: Some("old runs".to_owned()),
            }
        );
    }

    #[test]
    fn unchanged_runs_are_not_encoded_again_and_runs_that_gave_way_are_not_retried() {
        let history = RunHistoryRevision::default();
        assert_eq!(
            runs_decision(&history, true, &SessionKept::Whole, Some(&history)),
            RunsDecision::Unchanged
        );
        assert_eq!(
            runs_decision(
                &history,
                true,
                &SessionKept::WithoutRuns(history.clone()),
                None
            ),
            RunsDecision::Removed
        );
        // Runs that changed since they gave way are tried again.
        let mut changed = history.clone();
        changed.advance();
        assert_eq!(
            runs_decision(
                &changed,
                true,
                &SessionKept::WithoutRuns(history.clone()),
                None
            ),
            RunsDecision::Encode
        );
        assert_eq!(
            runs_decision(&history, false, &SessionKept::Whole, None),
            RunsDecision::Removed
        );
        assert_eq!(
            runs_decision(&history, true, &SessionKept::Whole, None),
            RunsDecision::Encode
        );
    }
}
