//! The application half of the platform services.
//!
//! The services read and write the shared model and their own state. What a
//! service reads of a feature, and what a project change resets in one, is
//! supplied here by the state that holds them all, in the order the service
//! did it when it held them itself.

use rspice_design::drc::DrcResult;

use crate::workbench::app_state::AppState;
use crate::workbench::app_state::session::serialization::WithoutRuns;
use crate::workbench::app_state::session::session_store::{Offer, offer, settle};
use rspice_app_design::workbench::app::actions::workspace::symbol_document::revert_active_symbol_with;
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_project::workbench::workflows::file_workflow;
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_services::io::FileWorkflowIo;
use rspice_app_services::io::ProjectSnapshot;
use rspice_app_services::services::session::SessionKept;
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_services::workbench::app_state::session::autosave::{self, AutosaveRestore};
use rspice_app_services::workbench::lifecycle::project_lifecycle::{
    self, CellViewRevert, LifecycleHost, LifecycleParts, LifecycleSource, LifecycleSourceMut,
    LifecycleView, PersistenceBinding, ProjectDocumentId, ProjectLifecycleError, RevertFollowUp,
    RevertReviewToken,
};
use rspice_app_services::workbench::lifecycle::recovery::{self, RecoveryCandidate};
use rspice_app_state::diagnostics::{LogSeverity, LogSource};
use rspice_app_state::state::{CellViewRef, LibraryManager, ViewType};

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

/// Test setters for whether a project is open, for every test in the crate.
#[cfg(test)]
impl AppState {
    pub(crate) fn open_project_session(&mut self) {
        project_lifecycle::open_project_session_for_test(self);
    }

    pub(crate) fn close_project_session(&mut self) {
        project_lifecycle::close_project_session_for_test(self);
    }
}

/// Restore the session's project. A session saved with no project open
/// restores onto the no-project landing: its workspace is the placeholder
/// Close Project left behind, and it is never bound or opened.
///
/// The session keeps no unsaved project work: it is written when the
/// application closes, after the reader saved or gave up that work. A
/// project it left with changes its file does not hold opens as saved, and
/// the autosave another session left newer than the file is looked for and
/// offered once.
pub(crate) fn initialize_from_session(state: &mut AppState) {
    if !state.platform.project_lifecycle.is_open() {
        state
            .workbench
            .activate(rspice_app_state::workbench::workspace::Workspace::Project);
        return;
    }
    project_lifecycle::bind_restored_project(state);
    #[cfg(not(target_arch = "wasm32"))]
    {
        // The session keeps no unsaved project work: a closed window left it
        // in an autosave, which the open offers back, and File > Exit saved
        // it or gave it up.
        let saved = project_lifecycle::canonical_native_path(&state.platform);
        match saved {
            Some(path) if project_lifecycle::has_unsaved_changes(state) => {
                state.load_project_from_path(&path);
            }
            _ => autosave::start_recovered_work_search(&state.model, &mut state.platform, None),
        }
    }
}

/// What the Project workspace shows of the project going away: the
/// feature's selections, and its navigator's filter, which names that
/// project's libraries, configurations and checkpoints.
fn forget_project_presentation(state: &mut AppState) {
    // The project going away had its unsaved work saved or given up.
    #[cfg(not(target_arch = "wasm32"))]
    autosave::project_left(&mut state.platform);
    state.project.on_project_changed();
    state
        .workbench
        .navigator_trees
        .filter_mut(rspice_app_state::workbench::workspace::Workspace::Project)
        .clear();
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
    forget_project_presentation(state);
    project_lifecycle::accept_loaded_project(state, baseline, binding);
    // An autosave another session left of the project just opened.
    #[cfg(not(target_arch = "wasm32"))]
    autosave::start_recovered_work_search(&state.model, &mut state.platform, None);
}

/// Start a new, unsaved project, after resetting what the features kept of
/// the project it replaces.
pub(crate) fn reset_for_new_project(state: &mut AppState) {
    state.models.on_project_changed();
    state.model.clear_project_design_history();
    forget_project_presentation(state);
    project_lifecycle::reset_for_new_project(state);
}

/// Take the project another window holds for writing all the same.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn open_anyway(state: &mut AppState) -> bool {
    project_lifecycle::open_anyway(state)
}

/// Close the open project, after resetting what the features kept of it.
pub(crate) fn mark_project_closed(state: &mut AppState) {
    state.models.on_project_changed();
    state.design.on_project_changed();
    state.model.clear_project_design_history();
    forget_project_presentation(state);
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
            state.restore_result_presentation(presentation);
            state.results.analysis.clear_specialized_viewer_data();
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
    // A symbol in front is reverted as a symbol edit is made: the wires its
    // placed instances hold follow the pins back to where they were saved.
    let symbol = state.model.workspace.content.active_view == *reference
        && state.model.workspace.content.active_view_type() == Some(ViewType::Symbol);
    let reverted = if symbol {
        revert_active_symbol_with(&mut state.design.symbol, &mut state.model, |model| {
            project_lifecycle::revert_cell_view(model, baseline, reference)
        })
    } else {
        project_lifecycle::revert_cell_view(&mut state.model, baseline, reference)
    };
    match reverted? {
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
            // The symbol's editor reads the restored drawing anew; the sheet
            // behind it is not the document that was reverted.
            if symbol {
                state.bump_active_schematic_epoch();
            } else if active {
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
/// work; the saved project and the checkpoint stay as they are. The
/// comparison is a new project that belongs to no file: its first save asks
/// where to write, so it can never land on the project it replaced.
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
    reset_for_new_project(state);
    state
        .workbench
        .activate(rspice_app_state::workbench::workspace::Workspace::Design);
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

/// Put the checkpoint the Recovery page read back into the open project, and
/// say so on the status bar. The page lists its checkpoints again, the
/// "Before restore" one among them.
pub(crate) fn apply_restored_checkpoint(state: &mut AppState) {
    let Some(restored) = state.project.project_checkpoint_recovery.restored.take() else {
        return;
    };
    state.project.project_checkpoint_recovery.invalidate();
    let when = rspice_app_state::time_compat::utc_stamp(restored.checkpoint.created_unix_ms());
    match restore_checkpoint_in_place(state, restored.project) {
        Ok(()) => {
            state.record(
                LogSeverity::Info,
                LogSource::Studio,
                format!(
                    "Restored checkpoint \u{201c}{}\u{201d} from {when}; the project as it was is checkpoint {}.",
                    restored.checkpoint.title(),
                    restored.before.checkpoint_id()
                ),
            );
            state.prompt(format!(
                "Restored \u{201c}{}\u{201d} from {when} as unsaved changes. The project as it was is kept as \u{201c}{}\u{201d}.",
                restored.checkpoint.title(),
                restored.before.title()
            ));
        }
        Err(error) => state.report_failure("Checkpoint not restored", error),
    }
}

/// Put a checkpoint's project back in the open project as unsaved changes:
/// its libraries, documents, simulation plan, models and results, its top
/// cell and its description. The project keeps who it is: its file, its
/// revision, its technology and every audit and publication record. A
/// checkpoint taken under another technology is refused: a technology is
/// changed on Dependencies, where the change is recorded.
pub(crate) fn restore_checkpoint_in_place(
    state: &mut AppState,
    mut checkpoint: ProjectSnapshot,
) -> Result<(), String> {
    if state.model.safe_mode.project_read_only() {
        return Err("The project is open read-only.".to_owned());
    }
    if state.model.simulation.execution.has_active_execution() {
        return Err("A simulation is running.".to_owned());
    }
    let current = &state.model.workspace.content.project;
    if checkpoint.file.workspace.project.id() != current.id() {
        return Err("The checkpoint belongs to another project.".to_owned());
    }
    if checkpoint.file.workspace.project.technology_binding() != current.technology_binding() {
        return Err(
            "The checkpoint has another technology. Change the technology on Dependencies first, or save the checkpoint as a new project."
                .to_owned(),
        );
    }
    let (simulation_plan, model_library_manager, warnings) =
        match checkpoint.file.execution_context.take() {
            Some(context) => rspice_app_services::io::restore_execution_context(context)
                .map_err(|error| format!("The checkpoint's simulation setup cannot be read: {error}"))?,
            None => (
                rspice_app_state::workbench::app_state::sim_setup::SimSetupState::new_with_user_preferences(
                    &state.model.preferences,
                ),
                rspice_app_state::workbench::workspace::default_model_library_manager(),
                vec!["The checkpoint has no simulation setup; the default setup was restored".to_owned()],
            ),
        };
    let mut project = current.clone();
    let restored = &checkpoint.file.workspace.project;
    project.root_library = restored.root_library.clone();
    project.top_cell = restored.top_cell.clone();
    project.description = restored.description.clone();
    project
        .set_include_search_paths(restored.include_search_paths().to_vec())
        .map_err(|error| format!("The checkpoint's include search paths are not valid: {error}"))?;
    checkpoint.file.workspace.project = project;

    let mut candidate = state.clone();
    candidate.cancel_schematic_drag();
    candidate.clear_design_execution_context();
    candidate
        .model
        .library_manager
        .replace_catalog_from_snapshot(&checkpoint.file.libraries)?;
    candidate.model.workspace = rspice_app_state::state::ProjectWorkspace::from_parts(
        checkpoint.file.workspace,
        checkpoint.workspace_session,
    );
    candidate.model.sim_setup = simulation_plan;
    candidate.model.model_library_manager = model_library_manager;
    candidate.restore_active_schematic_from_workspace();
    candidate.bump_active_schematic_epoch();
    candidate.model.simulation = rspice_app_state::state::SimulationState::default();
    rspice_app_services::io::restore_simulation_results(
        checkpoint.file.simulation_results,
        &mut candidate.model.simulation,
    )
    .map_err(|error| format!("The checkpoint's results cannot be read: {error}"))?;
    candidate
        .model
        .workspace
        .content
        .project
        .validate()
        .map_err(|error| format!("The restored project is not valid: {error}"))?;
    // What the features kept of the project before, and its edit history,
    // describe documents the restore replaced.
    candidate.models.on_project_changed();
    candidate.design.on_project_changed();
    candidate.code.on_project_changed();
    candidate.model.clear_project_design_history();
    project_lifecycle::refresh_registry(&mut candidate).map_err(|error| error.to_string())?;
    state.commit_candidate(candidate);
    for warning in warnings {
        state.record(LogSeverity::Warning, LogSource::Studio, warning);
    }
    Ok(())
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
        // A stored run's results are kept in the session's own folder first: the session names
        // them, and this process's store is gone by the next launch. Without them the session
        // is kept without its runs.
        #[cfg(not(target_arch = "wasm32"))]
        let documents = &self.model.workspace.content.visualization_documents;
        #[cfg(not(target_arch = "wasm32"))]
        let segments = rspice_app_services::io::result_segments::file_segments(
            &self
                .platform
                .project_lifecycle
                .result_cache
                .capture(&self.model.simulation),
            documents,
        );
        #[cfg(not(target_arch = "wasm32"))]
        let results_kept =
            rspice_app_services::io::result_segments::keep_session_segments(segments.clone())
                .inspect_err(|error| log::warn!("the session keeps no runs: {error}"))
                .is_ok();
        #[cfg(target_arch = "wasm32")]
        let results_kept = true;
        if results_kept && self.platform.session_kept != SessionKept::WithoutRuns(history.clone()) {
            match offer(storage, serde_json::to_string(&*self), read_back) {
                Offer::Kept => {
                    settle(&mut self.model, &mut self.platform, SessionKept::Whole);
                    // Safe mode stores the session it replaced, whose runs are not these.
                    #[cfg(not(target_arch = "wasm32"))]
                    if self.state_safe_mode_session_snapshot().is_none() {
                        rspice_app_services::io::result_segments::release_unread_session_segments(
                            &segments,
                        );
                    }
                    return;
                }
                Offer::Refused => {}
                Offer::NotEncoded => return,
            }
        }
        let kept = match offer(
            storage,
            serde_json::to_string(&WithoutRuns(self)),
            read_back,
        ) {
            Offer::Kept => SessionKept::WithoutRuns(history),
            Offer::Refused => SessionKept::Nothing,
            Offer::NotEncoded => return,
        };
        settle(&mut self.model, &mut self.platform, kept);
        // A session kept without its runs reads only what its documents archived.
        #[cfg(not(target_arch = "wasm32"))]
        if self.state_safe_mode_session_snapshot().is_none() {
            rspice_app_services::io::result_segments::release_unread_session_segments(
                &rspice_app_services::io::result_segments::file_segments(
                    &Default::default(),
                    &self.model.workspace.content.visualization_documents,
                ),
            );
        }
    }
}

/// The recovered-work question an open or a start raised: Keep puts the
/// autosave back in the open project as unsaved changes, Open the saved
/// project leaves it on the Recovery page, Compare shows it there with what
/// it changes listed.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn process_recovered_work_dialog(state: &mut AppState, ctx: &egui::Context) {
    use autosave::RecoveredWorkAnswer;
    let Some(answer) = autosave::recovered_work_dialog(&mut state.platform, ctx) else {
        return;
    };
    match answer {
        RecoveredWorkAnswer::Keep(found) => {
            let when = rspice_app_state::time_compat::utc_stamp(found.autosave.created_unix_ms());
            match restore_checkpoint_in_place(state, *found.project) {
                Ok(()) => {
                    state.project.project_checkpoint_recovery.invalidate();
                    state.prompt(format!(
                        "Kept the changes recovered from {when}; save to keep them in the project file."
                    ));
                }
                Err(error) => state.report_failure("Recovered changes not kept", error),
            }
        }
        RecoveredWorkAnswer::OpenSaved(found) => state.prompt(format!(
            "Opened the saved project. The recovered changes are on the Recovery page as \u{201c}{}\u{201d} until you save.",
            found.autosave.title()
        )),
        RecoveredWorkAnswer::Compare(found) => {
            let id = found.autosave.checkpoint_id();
            let recovery = &mut state.project.project_checkpoint_recovery;
            recovery.project_id = Some(found.project_id);
            recovery.invalidate();
            recovery.set_comparison(id, found.differences);
            state.project.project_checkpoint_selection = Some(id.to_string());
            state.project.project_page =
                rspice_app_project::workbench::project::workspace_state::ProjectPage::Recovery;
            state
                .workbench
                .activate(rspice_app_state::workbench::workspace::Workspace::Project);
        }
    }
}

/// Ask what to do with a save that met its file changed since this window
/// opened it, and do it: keep the work as a new project, replace the saved
/// file, open the saved file in its place, or write nothing.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn process_save_conflict_dialog(state: &mut AppState, ctx: &egui::Context) {
    use rspice_app_project::workbench::app::dialogs::project::save_conflict::{
        SaveConflictAnswer, save_conflict_dialog,
    };
    let Some(conflict) = state.platform.project_lifecycle.save_conflict.as_ref() else {
        return;
    };
    let Some(answer) = save_conflict_dialog(conflict, ctx) else {
        return;
    };
    let Some(conflict) = state.platform.project_lifecycle.save_conflict.take() else {
        return;
    };
    match answer {
        SaveConflictAnswer::SaveCopy => {
            state.save_project_copy();
        }
        SaveConflictAnswer::SaveOver => {
            project_lifecycle::accept_saved_file_for_overwrite(state, &conflict);
            let _ = state.save_project_scope(project_lifecycle::SaveScope::AllDocuments);
        }
        SaveConflictAnswer::OpenSaved => {
            state.load_project_from_path(&conflict.path);
        }
        SaveConflictAnswer::Keep => {
            state.prompt("Nothing was saved; the changes here are still unsaved.");
        }
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
            state.apply_loaded_schematic(
                *schematic,
                file_workflow::SchematicLoadOrigin::PersistentPath(&path),
            );
            state.record(LogSeverity::Info, LogSource::Studio, note);
        }
        AutosaveRestore::SavedFile { path, note } => {
            state.load_schematic_bypassing_autosave(&path, io);
            if let Some(note) = note {
                state.record(LogSeverity::Info, LogSource::Studio, note);
            }
        }
    }
    state.platform.pending_autosave_restore = None;
}
