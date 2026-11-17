//! The project transactions over the whole application that the
//! application root's own code calls: the File menu's New project opener
//! and its hold for unapplied configuration edits, Check and save's
//! whole-state validation, and the tests' project apply. They sit in the
//! root's layer, below the workflows; the rest sit in
//! `session::project_transactions`, above them.

use crate::workbench::app_state::AppState;
use rspice_app_project::workbench::app::dialogs::check_and_save_validation::{
    CheckAndSaveValidationReport, ValidationReads, run_design_checks,
};
use rspice_app_project::workbench::app::dialogs::project::configuration_dialogs::Leaving;
use rspice_app_project::workbench::app::dialogs::project::new_project::NewProjectDialogState;
use rspice_app_project::workbench::surfaces::project::attachable_technologies;
#[cfg(test)]
use rspice_app_project::workbench::workflows::project_workflow::ProjectLoadOrigin;
#[cfg(test)]
use rspice_app_project::workbench::workflows::project_workflow::report_lifecycle_error;
#[cfg(test)]
use rspice_app_services::io::ProjectSnapshot;
use rspice_app_services::services::recent_files::{RecentFile, RecentKind};
use rspice_app_services::workbench::lifecycle::project_lifecycle::LifecycleSource;
use rspice_app_state::state::workspace::{DEFAULT_PROJECT_LIBRARY, DEFAULT_TOP_CELL};
use rspice_app_workbench::workbench::app::dialogs::confirmation_state::ConfirmationAction;

/// Open the transaction with the default location, name, library and top
/// cell, and the technologies this session can attach. The name is the
/// default project name, numbered past any folder of it in the location, and
/// the dialog opens on it selected, so typing a name replaces it and Enter
/// takes it as it is.
pub(crate) fn open_new_project_dialog(state: &mut AppState) {
    state.project.new_project = NewProjectDialogState {
        open: true,
        location: default_project_location(&state.platform.recent_files),
        library: DEFAULT_PROJECT_LIBRARY.to_owned(),
        top_cell: DEFAULT_TOP_CELL.to_owned(),
        technologies: attachable_technologies(&state.model),
        ..NewProjectDialogState::default()
    };
    state.project.new_project.offer_free_name();
}

/// Where New project offers to make the project's folder: where the most
/// recently opened project's folder is, else the reader's documents folder.
/// A project kept in a folder of its own name sits in its own folder, so the
/// offer is that folder's parent; otherwise it is the file's own folder.
#[cfg(not(target_arch = "wasm32"))]
fn default_project_location(recent_files: &[RecentFile]) -> String {
    recent_files
        .iter()
        .filter(|recent| recent.kind == RecentKind::Project)
        .max_by_key(|recent| recent.opened_at_unix_ms)
        .and_then(|recent| {
            let folder = recent.path.parent()?;
            let own_folder = folder.file_name().is_some_and(|name| {
                recent
                    .path
                    .file_stem()
                    .is_some_and(|stem| stem.eq_ignore_ascii_case(name))
            });
            if own_folder {
                folder.parent()
            } else {
                Some(folder)
            }
            .map(std::path::Path::to_path_buf)
        })
        .or_else(documents_folder)
        .map(|folder| folder.display().to_string())
        .unwrap_or_default()
}

/// The reader's documents folder, else their home folder.
#[cfg(all(not(target_arch = "wasm32"), not(test)))]
fn documents_folder() -> Option<std::path::PathBuf> {
    dirs::document_dir().or_else(dirs::home_dir)
}

/// A test's documents folder is a fresh one of its own: a project created
/// in a test would otherwise be written among the developer's documents,
/// and the next run's would be refused the folder the last one left.
#[cfg(all(not(target_arch = "wasm32"), test))]
fn documents_folder() -> Option<std::path::PathBuf> {
    Some(
        rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir(
            "new-project-documents",
        ),
    )
}

/// The browser writes a project where its first save is asked to.
#[cfg(target_arch = "wasm32")]
fn default_project_location(_recent_files: &[RecentFile]) -> String {
    String::new()
}

/// Hold `action`, which takes the open project away, while the Configuration
/// page has edits that are not applied: the page asks to apply or discard
/// them first, then goes on with it. Returns whether it held the action.
pub(crate) fn hold_for_configuration_edits(
    state: &mut AppState,
    action: ConfirmationAction,
    recent: Option<(std::path::PathBuf, RecentKind)>,
) -> bool {
    if !state
        .project
        .configuration
        .is_dirty(&state.model.workspace.content.configuration_sets)
    {
        return false;
    }
    state.project.configuration_dialog.leaving = Some(Leaving { action, recent });
    true
}

#[cfg(test)]
pub(crate) fn apply_loaded_project(
    state: &mut AppState,
    project: ProjectSnapshot,
    origin: ProjectLoadOrigin<'_>,
) -> bool {
    let transaction =
        match rspice_app_services::workbench::lifecycle::project_lifecycle::begin_project_replacement(state) {
            Ok(transaction) => transaction,
            Err(error) => {
                report_lifecycle_error(
                    &mut state.model,
                    &mut state.workbench,
                    error,
                    "Project open blocked",
                );
                return false;
            }
        };
    state.apply_loaded_project_authorized(project, origin, None, transaction)
}

/// [`run_design_checks`] in the whole application state.
pub(crate) fn run_design_checks_in(state: &mut AppState) {
    let facts = state.slot_facts();
    run_design_checks(
        &mut state.model,
        &state.platform,
        facts,
        &mut state.workbench,
    );
}

/// Check and save's report, collected or validated over the whole
/// application state. It lives in the shell because it reads AppState.
pub(crate) trait WholeStateValidation: Sized {
    fn collect_in(state: &AppState) -> Result<Self, String>;
    #[cfg(test)]
    fn validate(state: &AppState) -> Result<Self, String>;
}

impl WholeStateValidation for CheckAndSaveValidationReport {
    /// [`Self::collect`] in the whole application state.
    fn collect_in(state: &AppState) -> Result<Self, String> {
        Self::collect(&ValidationReads::of(state))
    }

    /// The report the dialog opens on, its checks run first in a copy of
    /// `state`, so the state a test built is left as it was.
    #[cfg(test)]
    fn validate(state: &AppState) -> Result<Self, String> {
        let mut state = state.clone();
        run_design_checks_in(&mut state);
        Self::collect_in(&state)
    }
}

/// What a Check and save report reads, taken from the whole application
/// state. It lives in the shell because it reads AppState.
pub(crate) trait WholeStateValidationReads<'a> {
    fn of(state: &'a AppState) -> Self;
}

impl<'a> WholeStateValidationReads<'a> for ValidationReads<'a> {
    /// What a report reads, from the whole application state.
    fn of(state: &'a AppState) -> Self {
        let lifecycle = state.lifecycle_view();
        Self {
            model: lifecycle.model,
            platform: lifecycle.platform,
            facts: state.slot_facts(),
            workspace: lifecycle.workspace,
            presentation: lifecycle.presentation,
            design: &*state.design,
            code: &*state.code,
        }
    }
}
