//! The runs, and the project, the browser kept for the session.
//!
//! The browser keeps its session in local storage, as eframe's store, and
//! the runs it holds apart from it, in its durable records
//! (`rspice_app_services::workbench::app_state::session::browser_session`).
//! A session kept while the project's autosave held the work names that
//! record in place of the project. Records answer later, so both are read
//! before the application starts, and the start puts them back under the
//! session that names them.

use std::cell::RefCell;

use rspice_app_services::io::ProjectSnapshot;
use rspice_app_services::workbench::app_state::session::browser_session::{self, RestoredRuns};
use rspice_app_services::workbench::lifecycle::project_checkpoint;
use rspice_app_state::diagnostics::{LogSeverity, LogSource};

use crate::workbench::app_state::AppState;

thread_local! {
    /// The runs record the browser read before the application started.
    static READ_AT_START: RefCell<Option<String>> = const { RefCell::new(None) };
    /// The autosave the session names in place of its project, by its
    /// record's key, read before the application started.
    static AUTOSAVE_AT_START: RefCell<Option<(String, uuid::Uuid, ProjectSnapshot)>> =
        const { RefCell::new(None) };
}

/// Read the runs the browser kept for the session, and the autosave record
/// `named_autosave` its session names in place of its project, before the
/// application starts.
#[cfg(any(target_arch = "wasm32", test))]
pub async fn read_browser_session(named_autosave: Option<String>) {
    let Some(records) =
        rspice_app_services::workbench::lifecycle::durable_records::browser_records()
    else {
        return;
    };
    let runs = browser_session::read_runs(&*records.session).await;
    READ_AT_START.with(|read| *read.borrow_mut() = runs);
    let mut autosave = None;
    if let Some(key) = named_autosave {
        match project_checkpoint::load_named_record(&*records.checkpoints, &key).await {
            Ok(found) => {
                autosave =
                    found.map(|(summary, snapshot)| (key, summary.checkpoint_id(), snapshot));
            }
            Err(error) => log::warn!("the autosave the session names was not read: {error}"),
        }
    }
    AUTOSAVE_AT_START.with(|read| *read.borrow_mut() = autosave);
}

/// The autosave record the session in this browser's local storage names in
/// place of its project.
#[cfg(target_arch = "wasm32")]
pub fn autosave_named_by_session() -> Option<String> {
    web_sys::window()?
        .local_storage()
        .ok()??
        .get_item(browser_session::PROJECT_AUTOSAVE_KEY)
        .ok()?
}

impl AppState {
    /// Put back the project the restored session names in place of its own
    /// part: the autosave that held it as the session left it, read before
    /// the start. When that autosave is gone, the older project part the
    /// session kept beside it stays: the work it holds is not given up, and
    /// a newer autosave is still found for it.
    pub(crate) fn restore_browser_project(&mut self, storage: Option<&dyn eframe::Storage>) {
        let read = AUTOSAVE_AT_START.with(|read| read.borrow_mut().take());
        let Some(named) =
            storage.and_then(|storage| storage.get_string(browser_session::PROJECT_AUTOSAVE_KEY))
        else {
            return;
        };
        if !self.platform.project_lifecycle.is_open() {
            return;
        }
        let restored = match read {
            Some((key, autosave, snapshot)) if key == named => {
                if snapshot.file.workspace.project.id() == self.model.workspace.content.project.id()
                {
                    // The autosave holds the project as the session left it,
                    // who it is included; where its file is, it does not keep.
                    let path = self.model.workspace.content.project.path.clone();
                    let mut project = snapshot.file.workspace.project.clone();
                    project.path = path;
                    self.model.workspace.content.project = project;
                    crate::workbench::app_state::services_access::restore_checkpoint_in_place(
                        self, snapshot, None,
                    )
                    .map(|()| self.platform.project_autosave.restart_from = Some(autosave))
                } else {
                    Err("it belongs to another project".to_owned())
                }
            }
            _ => Err("it is no longer kept".to_owned()),
        };
        if let Err(error) = restored {
            log::warn!("the session's project was not read from its autosave: {error}");
        }
    }

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
