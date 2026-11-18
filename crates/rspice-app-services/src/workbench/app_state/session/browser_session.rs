//! The browser's session. The session itself (its layout and selections,
//! and the workspace they are over) is kept in local storage when the page
//! is left: written at once, while the page goes, so a reload finds it. The
//! runs it holds are kept apart from it, in the browser's durable records,
//! written in a browser task when they change, never on a frame and never
//! inside the session's text.
//!
//! The session names the runs record it was kept with, and is restored with
//! that one only: runs a page left before their write landed are said to be
//! lost, never put under a session they do not belong to. When the browser's
//! storage is full the project's autosave comes first: the runs give way to
//! it, and what was not kept is said once, when it starts.

use egui::Context;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::io::ProjectSimulationResults;
use crate::services::session::SessionKept;
use crate::state::RunHistoryRevision;
use crate::workbench::lifecycle::durable_records::{self, DurableRecords, Record, RecordError};
use crate::workbench::lifecycle::project_lifecycle::PlatformState;
use rspice_app_state::workbench::app_state::model::Model;

/// The record that holds the session's runs.
pub const RUNS_KEY: &str = "runs";
/// The local-storage key, beside the session's, naming the runs record the
/// session was kept with.
pub const RUNS_RECORD_KEY: &str = "rspice.session.runs";

/// The runs the browser keeps for the session.
#[derive(Debug, Clone, Default)]
pub struct KeptRuns {
    /// The run history the last write was taken at.
    pub history: Option<RunHistoryRevision>,
    /// The write the runs record holds, or is being written with. `None`
    /// when it holds none.
    pub record: Option<u64>,
    /// The last write started: each write is numbered.
    pub generation: u64,
    /// A write has not finished.
    pub writing: bool,
}

/// The runs record: the runs, tagged with the write that kept them.
fn runs_record(generation: u64, results: &str) -> String {
    format!("{{\"generation\":{generation},\"results\":{results}}}")
}

#[derive(serde::Deserialize)]
struct StoredRuns {
    generation: u64,
    results: ProjectSimulationResults,
}

/// Read the runs record. A store that cannot be read keeps none, said in
/// the log.
pub async fn read_runs(records: &dyn DurableRecords) -> Option<String> {
    match records.get(RUNS_KEY).await {
        Ok(record) => record.and_then(Record::into_text),
        Err(error) => {
            log::warn!("the stored runs could not be read: {error}");
            None
        }
    }
}

/// What a start found of the session's runs.
#[derive(Debug)]
pub enum RestoredRuns {
    /// The session was kept with none.
    None,
    /// The runs the session was kept with.
    Kept(ProjectSimulationResults),
    /// The session was kept with runs the store does not hold: their write
    /// had not landed when the page was left, or they do not read.
    Lost,
}

/// Match the runs record to the session: `expected` is the write the session
/// names, `stored` the runs record as read. The kept runs are set to what the
/// store holds, so a record no session names is removed by the next write.
pub fn restore_runs(
    expected: Option<u64>,
    stored: Option<&str>,
    kept: &mut KeptRuns,
) -> RestoredRuns {
    let stored = stored.map(|text| {
        serde_json::from_str::<StoredRuns>(text).map_err(|error| {
            log::warn!("the stored runs did not decode: {error}");
        })
    });
    let stored_generation = match &stored {
        Some(Ok(runs)) => Some(runs.generation),
        // A record that does not read is removed by the next write.
        Some(Err(())) => Some(u64::MAX),
        None => None,
    };
    kept.record = stored_generation;
    kept.generation = expected
        .max(stored_generation.filter(|generation| *generation != u64::MAX))
        .unwrap_or(0);
    match (expected, stored) {
        (None, _) => RestoredRuns::None,
        (Some(expected), Some(Ok(runs))) if runs.generation == expected => {
            RestoredRuns::Kept(runs.results)
        }
        (Some(_), _) => RestoredRuns::Lost,
    }
}

/// What becomes of the stored runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunsDecision {
    /// The stored runs are the session's: nothing to write.
    Unchanged,
    /// No runs are kept: the session has none, or they gave way.
    Removed,
    /// The session's runs, to be encoded and written.
    Encode,
}

/// What the runs at `history` need: nothing when they are what the last
/// write was taken at, removing when there are none or they gave way at
/// this very history, or writing.
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

/// What a runs write kept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunsWritten {
    Kept,
    Removed,
    /// The store refused them; the runs it held before are gone too, so
    /// they never come back with a later session.
    Refused,
    /// The store failed, for the reason given; it holds no runs.
    Failed(String),
}

/// Write the runs record, or remove it when `runs` is `None`.
async fn write_runs(
    records: &dyn DurableRecords,
    generation: u64,
    runs: Option<String>,
) -> RunsWritten {
    let Some(runs) = runs else {
        return match records.delete(RUNS_KEY).await {
            Ok(()) => RunsWritten::Removed,
            Err(error) => RunsWritten::Failed(error.to_string()),
        };
    };
    let refused = match records
        .put(RUNS_KEY, Record::Text(runs_record(generation, &runs)))
        .await
    {
        Ok(()) => return RunsWritten::Kept,
        Err(refused) => refused,
    };
    let removed = records.delete(RUNS_KEY).await;
    match (refused, removed) {
        (RecordError::Full, Ok(())) => RunsWritten::Refused,
        (refused, Ok(())) => RunsWritten::Failed(refused.to_string()),
        (_, Err(error)) => RunsWritten::Failed(error.to_string()),
    }
}

/// A runs write the browser finished, for a frame to settle.
#[derive(Debug, Clone)]
struct FinishedRuns {
    written: RunsWritten,
    history: RunHistoryRevision,
    generation: u64,
}

thread_local! {
    static FINISHED: std::cell::RefCell<Vec<FinishedRuns>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// Keep the session's runs when they changed: read off the model here,
/// encoded and written in a browser task. Not while a run is still being
/// produced, and one write at a time.
pub fn keep_runs(model: &Model, platform: &mut PlatformState, ctx: Option<&Context>) {
    let Some(records) = durable_records::browser_records() else {
        return;
    };
    let kept = &mut platform.session_runs;
    if kept.writing || model.simulation.execution.has_active_execution() {
        return;
    }
    let history = model.simulation.retained.runs.revision();
    let decision = runs_decision(
        &history,
        !model.simulation.retained.runs.is_empty(),
        &platform.session_kept,
        kept.history.as_ref(),
    );
    match decision {
        RunsDecision::Unchanged => return,
        RunsDecision::Removed if kept.record.is_none() => {
            kept.history = Some(history);
            return;
        }
        RunsDecision::Removed | RunsDecision::Encode => {}
    }
    let results = (decision == RunsDecision::Encode)
        .then(|| crate::io::capture_simulation_results(&model.simulation));
    kept.generation += 1;
    let generation = kept.generation;
    kept.history = Some(history.clone());
    kept.record = results.is_some().then_some(generation);
    kept.writing = true;
    durable_records::spawn_task(ctx, async move {
        let runs = results.and_then(|results| {
            serde_json::to_string(&results)
                .inspect_err(|error| log::warn!("the session's runs could not be encoded: {error}"))
                .ok()
        });
        let written = write_runs(&*records.session, generation, runs).await;
        FINISHED.with(|finished| {
            finished.borrow_mut().push(FinishedRuns {
                written,
                history,
                generation,
            });
        });
    });
}

/// Settle every runs write the browser finished, and say what was not kept
/// when that starts.
pub fn settle_finished(model: &mut Model, platform: &mut PlatformState) {
    let writes = FINISHED.with(|finished| std::mem::take(&mut *finished.borrow_mut()));
    for write in writes {
        platform.session_runs.writing = false;
        match write.written {
            RunsWritten::Kept | RunsWritten::Removed => {
                if platform.session_kept != SessionKept::Nothing {
                    settle(model, platform, SessionKept::Whole);
                }
            }
            RunsWritten::Refused | RunsWritten::Failed(_) => {
                if let RunsWritten::Failed(error) = &write.written {
                    log::warn!("the session's runs were not kept: {error}");
                }
                if platform.session_runs.record == Some(write.generation) {
                    platform.session_runs.record = None;
                }
                settle(model, platform, SessionKept::WithoutRuns(write.history));
            }
        }
    }
}

/// The session's runs gave way to the project's autosave: the store holds
/// none, and none are written again until the runs change.
pub fn runs_gave_way(model: &mut Model, platform: &mut PlatformState) {
    platform.session_runs.record = None;
    let history = model.simulation.retained.runs.revision();
    platform.session_runs.history = Some(history.clone());
    settle(model, platform, SessionKept::WithoutRuns(history));
}

/// The session was written to local storage (`kept`) or refused.
pub fn session_written(model: &mut Model, platform: &mut PlatformState, kept: bool) {
    if !kept {
        settle(model, platform, SessionKept::Nothing);
    } else if platform.session_kept == SessionKept::Nothing {
        let runs_left_out =
            platform.session_runs.record.is_none() && !model.simulation.retained.runs.is_empty();
        let now = if runs_left_out {
            SessionKept::WithoutRuns(model.simulation.retained.runs.revision())
        } else {
            SessionKept::Whole
        };
        settle(model, platform, now);
    }
}

/// Remember what the session keeps, and say so when that changes.
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

    fn results() -> String {
        serde_json::to_string(&ProjectSimulationResults::default()).expect("results encode")
    }

    /// The runs are kept tagged with their write, and a session is restored
    /// with the runs it names and no others.
    #[test]
    fn a_session_is_restored_with_the_runs_it_was_kept_with_and_no_others() {
        let origin = MemoryOrigin::new(None);
        let records = origin.store();
        assert_eq!(
            block_on(write_runs(&*records, 7, Some(results()))),
            RunsWritten::Kept
        );
        let stored = block_on(read_runs(&*records));

        let mut kept = KeptRuns::default();
        let restored = restore_runs(Some(7), stored.as_deref(), &mut kept);
        assert!(matches!(restored, RestoredRuns::Kept(_)), "{restored:?}");
        assert_eq!((kept.record, kept.generation), (Some(7), 7));

        // A session kept while a later write was still going.
        let mut kept = KeptRuns::default();
        let restored = restore_runs(Some(8), stored.as_deref(), &mut kept);
        assert!(matches!(restored, RestoredRuns::Lost), "{restored:?}");
        assert_eq!((kept.record, kept.generation), (Some(7), 8));

        // A session kept with none leaves the record to be removed.
        let mut kept = KeptRuns::default();
        let restored = restore_runs(None, stored.as_deref(), &mut kept);
        assert!(matches!(restored, RestoredRuns::None), "{restored:?}");
        assert_eq!(kept.record, Some(7));
    }

    /// Runs the store refuses go, the ones it held before them too: a reload
    /// brings back the session without runs, not with runs from another time.
    #[test]
    fn runs_the_store_refuses_leave_none_kept() {
        let origin = MemoryOrigin::new(None);
        let records = origin.store();
        block_on(write_runs(&*records, 1, Some(results())));
        origin.set_quota(Some(origin.used() + 16));
        let written = block_on(write_runs(&*records, 2, Some("x".repeat(1_000))));
        assert_eq!(written, RunsWritten::Refused);
        assert_eq!(block_on(read_runs(&*records)), None);
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
