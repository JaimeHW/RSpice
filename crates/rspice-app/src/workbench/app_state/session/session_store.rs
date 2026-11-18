//! Keeping the session for the next start.
//!
//! The desktop keeps the session in eframe's store when the application
//! closes. The browser keeps it in its durable records, the session in one
//! record and its runs in another
//! (`rspice_app_services::workbench::app_state::session::browser_session`).
//! A record store answers later, so the browser reads them before the
//! application starts, and the start restores them here.

use std::cell::RefCell;

use rspice_app_services::workbench::app_state::session::browser_session::StoredSession;
use rspice_app_state::diagnostics::{LogSeverity, LogSource};

use crate::workbench::app_state::AppState;

thread_local! {
    /// The session the browser read before the application started.
    static READ_AT_START: RefCell<Option<StoredSession>> = const { RefCell::new(None) };
}

/// Read the session the browser kept, before the application starts.
#[cfg(any(target_arch = "wasm32", test))]
pub async fn read_browser_session() {
    let Some(records) =
        rspice_app_services::workbench::lifecycle::durable_records::browser_records()
    else {
        return;
    };
    let stored = rspice_app_services::workbench::app_state::session::browser_session::read_session(
        &*records.session,
    )
    .await;
    READ_AT_START.with(|read| *read.borrow_mut() = Some(stored));
}

/// The session the browser read before the application started, once.
pub(in crate::workbench) fn take_browser_session() -> Option<StoredSession> {
    READ_AT_START.with(|read| read.borrow_mut().take())
}

impl AppState {
    /// Restore a session the browser kept: the session, then the runs kept
    /// beside it. `None` only when nothing was kept.
    pub(crate) fn restore_browser_session(stored: StoredSession) -> Option<Self> {
        let mut state = Self::restore_session_text(&stored.session?);
        if let Some(runs) = stored.runs {
            match serde_json::from_str(&runs) {
                Ok(results) => {
                    if let Some(warning) = state.restore_session_results(results) {
                        state.record(LogSeverity::Warning, LogSource::Studio, warning);
                    }
                }
                Err(error) => {
                    log::warn!("the stored runs did not decode: {error}");
                    state.record(
                        LogSeverity::Warning,
                        LogSource::Studio,
                        "The runs of the previous session could not be restored. Saved projects \
                         are unchanged.",
                    );
                }
            }
            // What the store holds is these runs: the next save of the
            // session does not encode them again.
            state.platform.session_runs_stored =
                Some(state.model.simulation.retained.runs.revision());
        }
        Some(state)
    }
}
