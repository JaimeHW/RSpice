//! The application half of the platform services.
//!
//! The services read and write the shared model and their own state. What a
//! service reads of a feature, and what a project change resets in one, is
//! supplied here by the state that holds them all, in the order the service
//! did it when it held them itself.

use rspice_design::drc::DrcResult;

#[cfg(not(target_arch = "wasm32"))]
use crate::diagnostics::{LogSeverity, LogSource};
#[cfg(not(target_arch = "wasm32"))]
use crate::io::FileWorkflowIo;
use crate::io::ProjectSnapshot;
use crate::services::session::SessionKept;
use crate::state::result_presentation::ResultPresentation;
use crate::state::{CellViewRef, LibraryManager, SimulationState};
use crate::workbench::app_state::AppState;
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::app_state::session::autosave::{self, AutosaveRestore};
use crate::workbench::app_state::session::serialization::WithoutRuns;
use crate::workbench::app_state::session::session_store::{Offer, offer, settle};
use crate::workbench::lifecycle::project_lifecycle::{
    self, CellViewRevert, LifecycleHost, LifecycleParts, LifecycleSource, LifecycleSourceMut,
    LifecycleView, PersistenceBinding, ProjectDocumentId, ProjectLifecycleError,
    ResultPresentationSource, RevertFollowUp, RevertReviewToken,
};
use crate::workbench::lifecycle::recovery::{self, RecoveryCandidate};
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::workflows::file_workflow;

impl ResultPresentationSource for crate::workbench::documents::result_document::ResultsState {
    fn project_presentation(&self, simulation: &SimulationState) -> ResultPresentation {
        Self::project_presentation(self, simulation)
    }
}

impl LifecycleSource for AppState {
    fn lifecycle_view(&self) -> LifecycleView<'_> {
        LifecycleView {
            model: &self.model,
            platform: &self.platform,
            workspace: self.workbench.workspace,
            presentation: &self.results.results,
        }
    }
}

impl LifecycleSourceMut for AppState {
    fn lifecycle_parts(&mut self) -> LifecycleParts<'_> {
        LifecycleParts {
            model: &mut self.model,
            platform: &mut self.platform,
            workspace: self.workbench.workspace,
            presentation: &self.results.results,
        }
    }
}

impl LifecycleHost for AppState {
    fn run_active_design_checks(&mut self) -> Result<DrcResult, String> {
        Self::run_active_design_checks(self)
    }
}

/// Restore the session's project. A session saved with no project open
/// restores onto the no-project landing: its workspace is the placeholder
/// Close Project left behind, and it is never bound or opened.
pub(crate) fn initialize_from_session(state: &mut AppState) {
    if !state.platform.project_lifecycle.is_open() {
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Project);
        return;
    }
    project_lifecycle::bind_restored_project(state);
}

/// Accept a loaded project as the open one, after resetting what the
/// features kept of the project it replaces.
pub(crate) fn accept_loaded_project(
    state: &mut AppState,
    baseline: ProjectSnapshot,
    binding: Option<PersistenceBinding>,
) {
    state.models.on_project_changed();
    state.model.clear_project_design_history();
    state.project.on_project_changed();
    project_lifecycle::accept_loaded_project(state, baseline, binding);
}

/// Start a new, unsaved project, after resetting what the features kept of
/// the project it replaces.
pub(crate) fn reset_for_new_project(state: &mut AppState) {
    state.models.on_project_changed();
    state.model.clear_project_design_history();
    state.project.on_project_changed();
    project_lifecycle::reset_for_new_project(state);
}

/// Close the open project, after resetting what the features kept of it.
pub(crate) fn mark_project_closed(state: &mut AppState) {
    state.models.on_project_changed();
    state.design.on_project_changed();
    state.model.clear_project_design_history();
    state.project.on_project_changed();
    project_lifecycle::mark_project_closed(state);
}

/// Close the active cell view's presentation, leaving its design in the
/// project.
pub(crate) fn close_active_document(state: &mut AppState) -> Result<(), ProjectLifecycleError> {
    project_lifecycle::require_open_project(&state.platform)?;
    if !project_lifecycle::can_close_active_document(state) {
        return Err(ProjectLifecycleError::LastPresentedDocument);
    }
    // Capture the live editor before removing only its presentation record.
    state.cancel_schematic_drag();
    state.sync_active_schematic_to_workspace();
    let closing = state.model.workspace.content.active_view.clone();
    state.model.workspace.content.close_view(&closing);
    state.restore_active_schematic_from_workspace();
    project_lifecycle::refresh_registry(state)
}

/// Revert the active document to the accepted project, as the confirmed
/// review describes it.
pub(crate) fn confirm_revert_active_document(
    state: &mut AppState,
    token: &RevertReviewToken,
) -> Result<(), ProjectLifecycleError> {
    project_lifecycle::validate_revert_review(state, token)?;
    revert_document(state, token.document().clone())
}

pub(crate) fn revert_document(
    state: &mut AppState,
    id: ProjectDocumentId,
) -> Result<(), ProjectLifecycleError> {
    // Revert can cross document boundaries (for example configuration roots,
    // cell catalogs, source ownership, and editor focus). Build and validate
    // the complete result away from live state so a failed registry rebuild
    // can never leave a partially reverted project behind.
    let mut candidate = state.clone();
    revert_document_in_place(&mut candidate, id)?;
    project_lifecycle::refresh_registry(&mut candidate)?;
    state.commit_candidate(candidate);
    Ok(())
}

fn revert_document_in_place(
    state: &mut AppState,
    id: ProjectDocumentId,
) -> Result<(), ProjectLifecycleError> {
    let follow_up = project_lifecycle::revert_document_model(&mut state.lifecycle_parts(), id)?;
    match follow_up {
        RevertFollowUp::None => {}
        RevertFollowUp::ProjectStructure(libraries) => {
            restore_project_structure_preserving_documents(state, libraries);
        }
        RevertFollowUp::CellView(baseline, reference) => {
            revert_cell_view(state, &baseline, &reference)?;
        }
        RevertFollowUp::ResultHistory(presentation) => {
            crate::workbench::documents::result_document::restore_presentation(state, presentation);
            state.clear_specialized_viewer_data();
        }
        RevertFollowUp::StimulusLibrary => state.design.on_project_changed(),
        RevertFollowUp::NetlistSource => {
            state.code.on_project_changed();
            *state.model.netlist_artifacts = Default::default();
            *state.runs.manual_run = Default::default();
            project_lifecycle::finish_netlist_source_revert(&mut state.model);
        }
    }
    Ok(())
}

fn revert_cell_view(
    state: &mut AppState,
    baseline: &ProjectSnapshot,
    reference: &CellViewRef,
) -> Result<(), ProjectLifecycleError> {
    match project_lifecycle::revert_cell_view(&mut state.model, baseline, reference)? {
        CellViewRevert::Removed { source_changed } => {
            state.prune_workspace_after_view_deleted(
                &reference.library,
                &reference.cell,
                &reference.view,
            );
            if source_changed {
                state.code.code_workspace.veriloga = Default::default();
            }
        }
        CellViewRevert::Restored { active } => {
            if active {
                state.restore_active_schematic_from_workspace();
            }
            if project_lifecycle::revert_cell_view_sources(&mut state.model, baseline, reference)? {
                state.code.code_workspace.veriloga = Default::default();
            }
        }
    }
    Ok(())
}

/// Open a recovery checkpoint as a comparison workspace in place of the open
/// work; the saved project and the checkpoint stay as they are.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn open_comparison(
    state: &mut AppState,
    candidate: &RecoveryCandidate,
) -> Result<(), String> {
    let comparison = recovery::prepare_comparison(&state.model, candidate)?;
    state.replace_project_execution_context(format!(
        "Opened recovery comparison for '{}'; the saved project and checkpoint are unchanged.",
        candidate.display_name
    ));
    let baseline_notice = recovery::install_comparison_design(&mut state.model, comparison);
    state.bump_active_schematic_epoch();
    recovery::reset_comparison_execution(&mut state.model);
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    if let Some(notice) = baseline_notice {
        state.record(LogSeverity::Warning, LogSource::Studio, notice);
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn open_comparison(
    _state: &mut AppState,
    _candidate: &RecoveryCandidate,
) -> Result<(), String> {
    Err(recovery::comparison_unavailable())
}

fn restore_project_structure_preserving_documents(state: &mut AppState, baseline: LibraryManager) {
    for reference in project_lifecycle::replace_project_structure(&mut state.model, baseline) {
        state.prune_workspace_after_view_deleted(
            &reference.library,
            &reference.cell,
            &reference.view,
        );
    }
    let removed = project_lifecycle::retain_project_view_sources(&mut state.model);
    if state
        .code
        .code_workspace
        .veriloga
        .receipt
        .as_ref()
        .is_some_and(|receipt| removed.contains(&receipt.compilation.token().bundle_id))
        || state
            .code
            .code_workspace
            .veriloga
            .pending
            .as_ref()
            .is_some_and(|pending| removed.contains(&pending.token.bundle_id))
    {
        state.code.code_workspace.veriloga = Default::default();
    }
    state
        .model
        .workspace
        .ensure_library_model(&mut state.model.library_manager);
}

impl AppState {
    /// Save the session under eframe's application key.
    pub(crate) fn store_session(&mut self, storage: &mut dyn eframe::Storage) {
        // Only the browser's store refuses without saying so. A desktop store
        // holds what it is given until it writes its file.
        self.store_session_checked(storage, cfg!(target_arch = "wasm32"));
    }

    pub(super) fn store_session_checked(
        &mut self,
        storage: &mut dyn eframe::Storage,
        read_back: bool,
    ) {
        let history = self.model.simulation.retained.runs.revision();
        if self.platform.session_kept != SessionKept::WithoutRuns(history.clone()) {
            match offer(storage, ron::ser::to_string(&*self), read_back) {
                Offer::Kept => {
                    settle(&mut self.model, &mut self.platform, SessionKept::Whole);
                    return;
                }
                Offer::Refused => {}
                Offer::NotEncoded => return,
            }
        }
        let kept = match offer(storage, ron::ser::to_string(&WithoutRuns(self)), read_back) {
            Offer::Kept => SessionKept::WithoutRuns(history),
            Offer::Refused => SessionKept::Nothing,
            Offer::NotEncoded => return,
        };
        settle(&mut self.model, &mut self.platform, kept);
    }
}

/// The restore decision an open deferred: Restore loads the checkpoint as
/// unsaved changes over the file's identity; Discard deletes it and opens the
/// file; Esc opens the file and keeps the checkpoint.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn process_autosave_restore_dialog(
    state: &mut AppState,
    io: &dyn FileWorkflowIo,
    ctx: &egui::Context,
) {
    let Some(choice) = autosave::restore_dialog(&mut state.model, &state.platform, ctx) else {
        return;
    };
    match choice {
        AutosaveRestore::Checkpoint {
            path,
            schematic,
            note,
        } => {
            file_workflow::apply_loaded_schematic(
                state,
                *schematic,
                file_workflow::SchematicLoadOrigin::PersistentPath(&path),
            );
            state.record(LogSeverity::Info, LogSource::Studio, note);
        }
        AutosaveRestore::SavedFile { path, note } => {
            file_workflow::load_schematic_bypassing_autosave(state, &path, io);
            if let Some(note) = note {
                state.record(LogSeverity::Info, LogSource::Studio, note);
            }
        }
    }
    state.platform.pending_autosave_restore = None;
}
