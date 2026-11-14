//! The project transactions over the whole application that the
//! application root's own code calls: the File menu's New project opener
//! and its hold for unapplied configuration edits, Check and save's
//! whole-state validation, and the tests' project apply. They sit in the
//! root's layer, below the workflows; the rest sit in
//! `session::project_transactions`, above them.

#[cfg(test)]
use crate::io::ProjectSnapshot;
use crate::services::recent_files::RecentKind;
use crate::state::workspace::{DEFAULT_PROJECT_LIBRARY, DEFAULT_TOP_CELL};
use crate::workbench::app::ConfirmationAction;
use crate::workbench::app::dialogs::check_and_save_validation::{
    CheckAndSaveValidationReport, ValidationReads, run_design_checks,
};
use crate::workbench::app::dialogs::project::configuration_dialogs::Leaving;
use crate::workbench::app::dialogs::project::new_project::NewProjectDialogState;
use crate::workbench::app_state::AppState;
use crate::workbench::lifecycle::project_lifecycle::LifecycleSource;
#[cfg(test)]
use crate::workbench::workflows::project_workflow::ProjectLoadOrigin;
#[cfg(test)]
use crate::workbench::workflows::project_workflow::report_lifecycle_error;

/// Open the transaction with the default library and top cell. The project
/// name starts empty because it is the one field with no defensible default:
/// requiring it is the point of the dialog.
pub(crate) fn open_new_project_dialog(state: &mut AppState) {
    state.project.new_project = NewProjectDialogState {
        open: true,
        library: DEFAULT_PROJECT_LIBRARY.to_owned(),
        top_cell: DEFAULT_TOP_CELL.to_owned(),
        ..NewProjectDialogState::default()
    };
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
        match crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(state) {
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
