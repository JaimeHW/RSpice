//! The runs the browser kept for the session.
//!
//! The browser keeps its session in local storage, as eframe's store, and
//! the runs it holds apart from it, in its durable records
//! (`rspice_app_services::workbench::app_state::session::browser_session`).
//! Those answer later, so the runs are read before the application starts,
//! and the start puts them back under the session that names them.

use std::cell::RefCell;

use rspice_app_services::workbench::app_state::session::browser_session::{self, RestoredRuns};
use rspice_app_state::diagnostics::{LogSeverity, LogSource};

use crate::workbench::app_state::AppState;

thread_local! {
    /// The runs record the browser read before the application started.
    static READ_AT_START: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Read the runs the browser kept for the session, before the application
/// starts.
#[cfg(any(target_arch = "wasm32", test))]
pub async fn read_browser_session() {
    let Some(records) =
        rspice_app_services::workbench::lifecycle::durable_records::browser_records()
    else {
        return;
    };
    let runs = browser_session::read_runs(&*records.session).await;
    READ_AT_START.with(|read| *read.borrow_mut() = runs);
}

impl AppState {
    /// Put back the runs the restored session was kept with, read from the
    /// browser's records before the start. Runs the page was left before
    /// keeping are said to be lost.
    pub(crate) fn restore_browser_runs(&mut self, storage: Option<&dyn eframe::Storage>) {
        let expected = storage
            .and_then(|storage| storage.get_string(browser_session::RUNS_RECORD_KEY))
            .and_then(|record| record.parse::<u64>().ok());
        let stored = READ_AT_START.with(|read| read.borrow_mut().take());
        match browser_session::restore_runs(
            expected,
            stored.as_deref(),
            &mut self.platform.session_runs,
        ) {
            RestoredRuns::None => {}
            RestoredRuns::Kept(results) => {
                if let Some(warning) = self.restore_session_results(results) {
                    self.record(LogSeverity::Warning, LogSource::Studio, warning);
                }
                // What the store holds is these runs: they are not written
                // again until they change.
                self.platform.session_runs.history =
                    Some(self.model.simulation.retained.runs.revision());
            }
            RestoredRuns::Lost => {
                self.record(
                    LogSeverity::Warning,
                    LogSource::Studio,
                    "The run results of the previous session were not restored: the page was left \
                     before the browser kept them. Saved projects are unchanged.",
                );
            }
        }
    }
}
