//! The application half of the platform services.
//!
//! The services read and write the shared model and their own state. What a
//! service reads of a feature, and what a project change resets in one, is
//! supplied here by the state that holds them all, in the order the service
//! did it when it held them itself.

use rspice_design::drc::DrcResult;

use crate::workbench::app_state::AppState;
use crate::workbench::app_state::session::serialization::WithoutRuns;
use rspice_app_design::workbench::app::actions::workspace::symbol_document::revert_active_symbol_with;
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_project::workbench::workflows::file_workflow;
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_services::io::FileWorkflowIo;
use rspice_app_services::io::ProjectSnapshot;
use rspice_app_services::workbench::app_state::session::autosave;
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_services::workbench::app_state::session::autosave::AutosaveRestore;
use rspice_app_services::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary;
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
            presentation: &*self.results.results,
        }
    }
}

impl LifecycleSourceMut for AppState {
    fn lifecycle_parts(&mut self) -> LifecycleParts<'_> {
        LifecycleParts {
            model: &mut self.model,
            platform: &mut self.platform,
            workspace: self.workbench.workspace,
            presentation: &*self.results.results,
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
    // The browser has no exit to ask about unsaved work at: its session keeps
    // that work, and the autosave the last session left is looked for once
    // the project is bound again (in the browser, when its binding restore
    // lands).
    if rspice_app_services::workbench::lifecycle::durable_records::browser_records().is_some() {
        #[cfg(not(target_arch = "wasm32"))]
        if state.platform.project_lifecycle.binding().is_some() {
            autosave::start_restart_search(&mut state.lifecycle_parts());
        }
        return;
    }
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

/// Take the project another window, or another tab of this browser, holds
/// for writing all the same.
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
        RevertFollowUp::ResultHistory(signal_sets) => {
            state.restore_result_signal_sets(signal_sets);
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
    let when = rspice_app_state::time_compat::local_stamp(restored.checkpoint.created_unix_ms());
    let authority = RestoreAuthority {
        restored: &restored.checkpoint,
        before: &restored.before,
        actor: &restored.actor,
    };
    match restore_checkpoint_in_place(state, restored.project, Some(authority)) {
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

/// Who restores a checkpoint, and the checkpoint that keeps the project as
/// it was before: what a restore that brings back another technology records
/// that change under.
pub(crate) struct RestoreAuthority<'a> {
    pub restored: &'a ProjectCheckpointSummary,
    pub before: &'a ProjectCheckpointSummary,
    pub actor: &'a str,
}

/// Put a checkpoint's project back in the open project as unsaved changes:
/// its libraries, documents, simulation plan, models and results, its top
/// cell, its description and its technology. The project keeps who it is:
/// its file, its revision history and every audit and publication record. A
/// technology the checkpoint brings back, or one it detaches, is recorded in
/// the project's technology history under `authority`, the "Before restore"
/// checkpoint its receipt names; without one, a checkpoint taken under
/// another technology is refused.
pub(crate) fn restore_checkpoint_in_place(
    state: &mut AppState,
    mut checkpoint: ProjectSnapshot,
    authority: Option<RestoreAuthority<'_>>,
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
    let restored_technology = checkpoint
        .file
        .workspace
        .project
        .technology_binding()
        .cloned();
    let technology_changes = restored_technology.as_ref() != current.technology_binding();
    if technology_changes && authority.is_none() {
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
    if technology_changes && let Some(authority) = &authority {
        restore_technology(&state.model, &mut project, restored_technology, authority)?;
    }
    let restored = &checkpoint.file.workspace.project;
    project.root_library = restored.root_library.clone();
    project.top_cell = restored.top_cell.clone();
    project.description = restored.description.clone();
    project
        .set_include_search_paths(restored.include_search_paths().to_vec())
        .map_err(|error| format!("The checkpoint's include search paths are not valid: {error}"))?;
    checkpoint.file.workspace.project = project;
    // Libraries kept in folders of their own are read as they are now, and
    // the checkpoint's unsaved edits to them go back over them, unsaved.
    let unsaved_library_files = std::mem::take(&mut checkpoint.file.unsaved_library_files);
    let mut library_files = state.model.read_referenced_libraries(&mut checkpoint.file);
    state.model.restore_unsaved_library_files(
        &mut checkpoint.file,
        &mut library_files,
        unsaved_library_files,
    );

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
    candidate.model.library_files = library_files;
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
    if technology_changes
        && candidate
            .model
            .workspace
            .content
            .project
            .technology_binding()
            .is_some()
    {
        candidate
            .model
            .validate_project_technology_contract()
            .map_err(|error| {
                format!("The checkpoint's technology does not resolve here: {error}")
            })?;
    }
    // What the features kept of the project before, and its edit history,
    // describe documents the restore replaced.
    candidate.models.on_project_changed();
    candidate.design.on_project_changed();
    candidate.code.on_project_changed();
    candidate.model.clear_project_design_history();
    project_lifecycle::refresh_registry(&mut candidate).map_err(|error| error.to_string())?;
    state.model.release_library_locks();
    state.commit_candidate(candidate);
    for warning in warnings {
        state.record(LogSeverity::Warning, LogSource::Studio, warning);
    }
    Ok(())
}

/// Bring `project`'s technology back to `restored`, the one the checkpoint
/// was taken under, as one audited change whose receipt names the "Before
/// restore" checkpoint and the person restoring it. A technology that is not
/// installed here, or a PDK revision change, which is reviewed on
/// Dependencies, is refused.
fn restore_technology(
    model: &rspice_app_state::workbench::app_state::model::Model,
    project: &mut rspice_app_state::state::ProjectDescriptor,
    restored: Option<rspice_app_state::state::ProjectTechnologyBinding>,
    authority: &RestoreAuthority<'_>,
) -> Result<(), String> {
    use rspice_app_services::workbench::lifecycle::project_checkpoint::technology_restore;
    let verdict = technology_restore(model, restored.as_ref());
    if let Some(refusal) = verdict.refusal(authority.restored.title()) {
        return Err(refusal);
    }
    let before = authority.before;
    let context = rspice_app_state::state::ProjectTechnologyChangeContext::new(
        rspice_app_state::state::ProjectTechnologyChangeAuthority::new(
            authority.actor.trim(),
            format!(
                "Restore of checkpoint {}",
                authority.restored.checkpoint_id()
            ),
            format!(
                "Restored \u{201c}{}\u{201d} from {}",
                authority.restored.title(),
                rspice_app_state::time_compat::utc_stamp(authority.restored.created_unix_ms())
            ),
        )
        .map_err(|error| format!("Who restored it cannot be recorded: {error}"))?,
        before.checkpoint_id(),
        rspice_app_state::product::ObjectRevision::new(before.project_revision()).map_err(
            |error| format!("The \u{201c}Before restore\u{201d} checkpoint is not valid: {error}"),
        )?,
        before.created_unix_ms(),
        before.snapshot_digest(),
        before.snapshot_byte_len(),
    )
    .map_err(|error| {
        format!("The \u{201c}Before restore\u{201d} checkpoint is not valid: {error}")
    })?;
    match restored {
        Some(binding) => project
            .attach_technology_audited(binding, context)
            .map(|_| ())
            .map_err(|error| format!("The checkpoint's technology was not brought back: {error}")),
        None => project
            .detach_technology_audited(context)
            .map(|_| ())
            .map_err(|error| format!("The technology was not detached: {error}")),
    }
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
    /// Save the session under eframe's application key, as the desktop keeps
    /// it when the application closes.
    pub(crate) fn store_session(&mut self, storage: &mut dyn eframe::Storage) {
        // A stored run's results are kept in the session's own folder first: the session names
        // them, and this process's store is gone by the next launch. Without them the session
        // is kept without its runs.
        #[cfg(not(target_arch = "wasm32"))]
        let segments = rspice_app_services::io::result_segments::file_segments(
            &self
                .platform
                .project_lifecycle
                .result_cache
                .capture(&self.model.simulation),
            &self.model.workspace.content.visualization_documents,
        );
        #[cfg(not(target_arch = "wasm32"))]
        let results_kept =
            rspice_app_services::io::result_segments::keep_session_segments(segments.clone())
                .inspect_err(|error| log::warn!("the session keeps no runs: {error}"))
                .is_ok();
        #[cfg(target_arch = "wasm32")]
        let results_kept = true;
        let encoded = if results_kept {
            serde_json::to_string(&*self)
        } else {
            serde_json::to_string(&WithoutRuns(self))
        };
        match encoded {
            Ok(text) => storage.set_string(eframe::APP_KEY, text),
            Err(error) => {
                // What eframe's own encoder does: the stored session stays.
                log::error!("The session could not be encoded: {error}");
                return;
            }
        }
        // Safe mode stores the session it replaced, whose runs are not these.
        #[cfg(not(target_arch = "wasm32"))]
        if self.state_safe_mode_session_snapshot().is_none() {
            let read = if results_kept {
                segments
            } else {
                // A session kept without its runs reads only what its documents archived.
                rspice_app_services::io::result_segments::file_segments(
                    &Default::default(),
                    &self.model.workspace.content.visualization_documents,
                )
            };
            rspice_app_services::io::result_segments::release_unread_session_segments(&read);
        }
    }

    /// Keep the session as the browser does when its page is left: the
    /// project's unsaved work as an autosave, and the runs when they changed,
    /// each written in a browser task; then the session itself, at once, in
    /// local storage, because the page may be gone before a task runs. The
    /// session names the runs record it goes with.
    pub(crate) fn keep_browser_session(&mut self, storage: &mut dyn eframe::Storage) {
        use rspice_app_services::workbench::app_state::session::browser_session;
        if rspice_app_services::workbench::lifecycle::durable_records::browser_records().is_none() {
            return;
        }
        autosave::write_now(self, None);
        browser_session::keep_runs(&self.model, &mut self.platform, None);
        let session = match serde_json::to_string(&WithoutRuns(self)) {
            Ok(session) => session,
            Err(error) => {
                log::error!("The session could not be encoded: {error}");
                return;
            }
        };
        // Local storage refuses a write it has no room for and keeps what it
        // held, saying nothing: the session is read back to know.
        storage.set_string(eframe::APP_KEY, session.clone());
        let kept = storage
            .get_string(eframe::APP_KEY)
            .is_some_and(|stored| stored == session);
        if kept {
            match self.platform.session_runs.record {
                Some(record) => {
                    storage.set_string(browser_session::RUNS_RECORD_KEY, record.to_string());
                }
                None => storage.remove_string(browser_session::RUNS_RECORD_KEY),
            }
        }
        browser_session::session_written(&mut self.model, &mut self.platform, kept);
    }
}

/// A browser start reopened the project with the unsaved work its session
/// kept, and recovered work was found for it. When the session brought back
/// just what the autosave holds, the project is put back as saved and the
/// autosave offered, as the desktop offers it at a start. When the reader has
/// worked on since, or the session kept work the autosave does not hold, that
/// work stays and nothing is asked: the autosave stays on the Recovery page.
pub(crate) fn settle_restart_offer(state: &mut AppState) {
    let autosave = &mut state.platform.project_autosave;
    if autosave.offer.is_none() {
        return;
    }
    let Some(stamp) = autosave.restart_stamp.take() else {
        return;
    };
    if !project_lifecycle::has_unsaved_changes(state) {
        return;
    }
    let Some(offer) = state.platform.project_autosave.offer.take() else {
        return;
    };
    // Runs the session could not keep, when storage was full, are still in
    // the autosave: their absence is not newer work.
    let runs_left_out = state.model.simulation.retained.runs.is_empty();
    let session_holds_the_autosave = stamp == project_lifecycle::working_stamp(state)
        && project_lifecycle::snapshot(state)
            .ok()
            .and_then(|working| {
                rspice_app_services::workbench::lifecycle::project_checkpoint::differences_between(
                    &working,
                    &offer.project,
                )
                .ok()
            })
            .is_some_and(|differences| {
                differences.iter().all(|difference| {
                    runs_left_out
                        && difference.document
                            == project_lifecycle::ProjectDocumentId::ResultHistory
                })
            });
    let saved = state.platform.project_lifecycle.accepted_snapshot();
    match saved {
        Ok(saved) if session_holds_the_autosave => {
            match restore_checkpoint_in_place(state, saved, None) {
                Ok(()) => state.platform.project_autosave.offer = Some(offer),
                Err(error) => {
                    log::warn!("the saved project was not put back before the offer: {error}");
                }
            }
        }
        _ => {
            state.record(
                LogSeverity::Info,
                LogSource::Studio,
                "The unsaved changes this session brought back stay open. The autosave the last \
                 session left is on the History page.",
            );
        }
    }
}

/// The recovered-work question an open or a start raised: Keep puts the
/// autosave back in the open project as unsaved changes, Open the saved
/// project leaves it on the Recovery page, Compare shows it there with what
/// it changes listed.
pub(crate) fn process_recovered_work_dialog(state: &mut AppState, ctx: &egui::Context) {
    use autosave::RecoveredWorkAnswer;
    let Some(answer) = autosave::recovered_work_dialog(&mut state.platform, ctx) else {
        return;
    };
    match answer {
        RecoveredWorkAnswer::Keep(found) => {
            let when = rspice_app_state::time_compat::local_stamp(found.autosave.created_unix_ms());
            match restore_checkpoint_in_place(state, *found.project, None) {
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
            "Opened the saved project. The recovered changes are on the History page as \u{201c}{}\u{201d} until you save.",
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
                rspice_app_project::workbench::project::workspace_state::ProjectPage::History;
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
