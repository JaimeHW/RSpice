//! Transactional project/document lifecycle.
//!
//! The accepted project baseline is intentionally separate from the mutable
//! workbench state. `Save` overlays one stable document onto that baseline;
//! `Save all` replaces it with the complete working set. This prevents saving
//! one tab from accidentally committing unrelated drafts.

mod accepted_project;
mod persistence;
mod result_cache;

use accepted_project::AcceptedProject;
#[cfg(target_arch = "wasm32")]
pub(crate) use persistence::{
    start_browser_checkpoint_list, start_browser_checkpoint_publish, start_browser_checkpoint_read,
};
mod registry;

#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use std::path::PathBuf;

pub(crate) use crate::product::TransactionId;
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) use persistence::BrowserBindingBackend;
#[cfg(target_arch = "wasm32")]
pub(crate) use persistence::BrowserWriteTarget;
pub(crate) use persistence::{BrowserBindingReceipt, NativeBindingReceipt, PersistenceBinding};
pub(crate) use registry::ProjectDocumentId;
#[cfg(target_arch = "wasm32")]
pub(crate) use rspice_project::lifecycle::BrowserOperationContext;
use rspice_project::lifecycle::ProjectLifecycle;
pub(crate) use rspice_project::lifecycle::{ProjectLifecycleError, RevertReviewToken, SaveScope};

use crate::diagnostics::{ConsoleMessage, LogSeverity, LogSource};
use crate::io::{ProjectSimulationResults, ProjectSnapshot};
#[cfg(target_arch = "wasm32")]
use crate::product::ContentDigest;
use crate::state::{CellViewRef, ViewType};
use crate::workbench::app_state::AppState;

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_BINDING_RESTORE_RESULTS: std::cell::RefCell<std::collections::VecDeque<BrowserRestoreCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

#[cfg(target_arch = "wasm32")]
struct BrowserRestoreCompletion {
    context: BrowserOperationContext,
    result: persistence::BrowserRestoreResult,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone)]
struct BrowserConflict {
    binding: PersistenceBinding,
    observed_digest: ContentDigest,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ProjectLifecycleState {
    pub(crate) authority: ProjectLifecycle,
    accepted: Option<AcceptedProject>,
    registry: registry::DocumentRegistry,
    unreadable_native_binding: Option<persistence::UnreadableNativeBinding>,
    result_cache: result_cache::ResultCache,
    result_fingerprints: registry::ResultFingerprintCache,
    #[cfg(target_arch = "wasm32")]
    browser_reconnect_binding: Option<PersistenceBinding>,
    #[cfg(target_arch = "wasm32")]
    browser_conflict: Option<BrowserConflict>,
}

impl ProjectLifecycleState {
    pub(crate) const fn is_open(&self) -> bool {
        self.authority.is_open()
    }

    pub(crate) fn operation_in_progress(&self) -> bool {
        self.authority.operation_in_progress()
    }

    pub(crate) fn accepted(&self) -> Option<&AcceptedProject> {
        self.accepted.as_ref()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn canonical_native_path(&self) -> Option<&Path> {
        self.accepted
            .as_ref()
            .and_then(|accepted| accepted.binding.as_ref())
            .and_then(PersistenceBinding::canonical_path)
    }
}

/// Resolve the actual publication scope before validation starts. The first
/// canonical save has no accepted baseline that can safely receive a
/// document-only overlay, so it necessarily publishes the complete project.
/// Workflows such as Check and save use this same decision before presenting
/// their document scope and before freezing validation evidence.
pub(crate) fn effective_save_scope(state: &AppState, requested: SaveScope) -> SaveScope {
    let first_save = state.project_lifecycle.accepted.is_none()
        || state
            .project_lifecycle
            .accepted
            .as_ref()
            .and_then(|accepted| accepted.binding.as_ref())
            .is_none();
    if first_save {
        SaveScope::AllDocuments
    } else {
        requested
    }
}

/// Exact active schematic from the currently accepted canonical project.
/// This is used only to seed the validated-save journal when an older project
/// predates that journal. A missing canonical binding or a newly created view
/// has no predecessor and therefore returns `None`.
pub(crate) fn accepted_active_schematic(state: &AppState) -> Option<crate::state::SchematicState> {
    let accepted = state.project_lifecycle.accepted.as_ref()?;
    accepted.binding.as_ref()?;
    accepted.clone_schematic_editor(&state.workspace.content.active_view.key())
}

/// Monotonic identity of the accepted canonical baseline. Validation receipts
/// bind this value so a save opened against one baseline cannot publish after
/// a different save, import, or binding restoration has completed.
pub(crate) const fn accepted_generation(state: &AppState) -> u64 {
    state.project_lifecycle.authority.accepted_generation()
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DestinationAuthority {
    /// Ordinary Save to an already accepted canonical binding.
    Canonical,
    /// A fresh native Save/Save As picker explicitly selected this path and
    /// supplied the platform overwrite decision.
    UserSelected,
}

pub(crate) fn snapshot(state: &AppState) -> Result<ProjectSnapshot, ProjectLifecycleError> {
    capture_snapshot(state, SnapshotContent::Committed)
}

#[derive(Clone, Copy)]
enum SnapshotContent {
    Committed,
    Current,
}

fn capture_snapshot(
    state: &AppState,
    content: SnapshotContent,
) -> Result<ProjectSnapshot, ProjectLifecycleError> {
    let mut workspace = state.workspace.clone();
    if matches!(
        workspace.content.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    ) {
        workspace.insert_schematic_editor(workspace.content.active_key(), state.schematic.clone());
    }
    // A save/checkpoint retains committed content even when a native window
    // currently holds a live pointer preview in one of the runtime buffers.
    if matches!(content, SnapshotContent::Committed) {
        workspace.for_each_schematic_editor_mut(|_, schematic| {
            schematic.cancel_operation();
        });
    }
    workspace.mark_all_clean();
    workspace.for_each_schematic_editor_mut(|_, schematic| {
        strip_schematic_runtime_state(schematic);
    });

    let mut libraries = state.library_manager.clone();
    sanitize_library_view_runtime_state(&mut libraries);
    let simulation_results = state
        .project_lifecycle
        .result_cache
        .capture(&state.simulation);
    let execution_context =
        crate::io::capture_execution_context(&state.sim_setup, &state.model_library_manager)
            .map_err(ProjectLifecycleError::InvalidState)?;
    let project = ProjectSnapshot::new_with_execution_context(
        workspace,
        libraries,
        simulation_results,
        execution_context,
    )
    .with_result_presentation(state.ui.results.project_presentation(&state.simulation));
    project
        .file
        .validate()
        .map_err(|error| ProjectLifecycleError::InvalidState(error.to_string()))?;
    project
        .file
        .simulation_results
        .validate()
        .map_err(ProjectLifecycleError::InvalidState)?;
    Ok(project)
}

/// Canonical identity of every authoritative input consumed by schematic
/// netlist generation. Result history and independently owned source decks are
/// removed because neither is a generator input; design, hierarchy, project
/// configuration, simulation-plan payloads, project-owned behavioral sources,
/// model bindings, and libraries remain covered by the ordinary document
/// digests. Project sources must stay authenticated because their virtual
/// directive identities and exact bytes affect generated output and execution.
pub(crate) fn generated_netlist_input_digest(
    state: &AppState,
) -> Result<crate::product::ContentDigest, ProjectLifecycleError> {
    // Execution provenance must describe the exact live input consumed by
    // generation, including an edit currently previewed in another window.
    let mut project = capture_snapshot(state, SnapshotContent::Current)?;
    project.file.simulation_results = ProjectSimulationResults::default();
    // A receipt records validation of the existing inputs. Provider decisions
    // still participate because they select the source emitted to the engine.
    if let Some(context) = project.file.execution_context.as_mut() {
        context.model_validation_receipt = None;
    }
    // Annotating a plot must never change what the netlist generator is
    // asked to produce.
    project.file.result_presentation = Default::default();
    project.file.workspace.netlist_source = None;
    project.file.workspace.netlist_source_path = None;
    project.file.workspace.netlist_document = None;
    project.file.workspace.netlist_descriptor = None;
    project.file.workspace.retained_netlist_decks.clear();
    registry::content_digest(&project.file).map_err(ProjectLifecycleError::InvalidState)
}

pub(crate) fn has_unsaved_changes(state: &AppState) -> bool {
    if !state.project_lifecycle.is_open() {
        return false;
    }
    let Some(accepted) = state.project_lifecycle.accepted.as_ref() else {
        return true;
    };
    match working_fingerprints(state).map(|current| current.content_digest()) {
        Ok(current) => accepted
            .fingerprints()
            .map(|baseline| current != baseline.content_digest())
            .unwrap_or(true),
        Err(_) => true,
    }
}

pub(crate) fn operation_in_progress(state: &AppState) -> bool {
    state.project_lifecycle.operation_in_progress()
}

pub(crate) fn active_document(state: &AppState) -> ProjectDocumentId {
    if state.workbench.workspace == crate::workbench::state::Workspace::Netlist
        && state.workspace.content.active_view_type() == ViewType::VerilogA
        && state
            .workspace
            .content
            .project_sources
            .bundle_for_owner(&crate::state::ProjectSourceOwner::cell_view(
                state.workspace.content.active_view.clone(),
            ))
            .is_some()
    {
        return ProjectDocumentId::CellView(state.workspace.content.active_view.clone());
    }
    registry::active_document(
        state.workbench.workspace,
        &state.workspace.content.active_view,
    )
}

pub(crate) fn active_document_is_dirty(state: &AppState) -> bool {
    if state.project_lifecycle.accepted.is_none() {
        return state.project_lifecycle.is_open();
    }
    current_registry(state)
        .map(|registry| registry.is_dirty(&active_document(state)))
        .unwrap_or(true)
}

pub(crate) fn refresh_registry(state: &mut AppState) -> Result<(), ProjectLifecycleError> {
    if !state.project_lifecycle.is_open() {
        state.project_lifecycle.registry = registry::DocumentRegistry::default();
        return Ok(());
    }
    match current_registry(state) {
        Ok(registry) => state.project_lifecycle.registry = registry,
        Err(error) => {
            state.project_lifecycle.registry.invalidate();
            apply_registry_dirty_flags(state);
            return Err(error);
        }
    }
    apply_registry_dirty_flags(state);
    Ok(())
}

fn working_fingerprints(
    state: &AppState,
) -> Result<registry::DocumentFingerprints, ProjectLifecycleError> {
    let current = capture_snapshot(state, SnapshotContent::Current)?;
    registry::document_fingerprints_with_results_cache(
        &current.file,
        &state.project_lifecycle.result_fingerprints,
    )
    .map_err(ProjectLifecycleError::InvalidState)
}

fn current_registry(state: &AppState) -> Result<registry::DocumentRegistry, ProjectLifecycleError> {
    let current = working_fingerprints(state)?;
    let accepted = state
        .project_lifecycle
        .accepted
        .as_ref()
        .map(AcceptedProject::fingerprints)
        .transpose()
        .map_err(ProjectLifecycleError::InvalidState)?;
    let mut registry = registry::DocumentRegistry::default();
    registry.rebuild_from_fingerprints(&current, accepted);
    Ok(registry)
}

fn apply_registry_dirty_flags(state: &mut AppState) {
    if state.project_lifecycle.registry.comparison_failed() {
        // The working draft could not be compared with accepted content.
        // Preserve its authorship and show pending changes until a successful
        // refresh can establish which documents are actually clean.
        state.schematic.session.is_dirty = true;
        state.workspace.content.project_metadata_dirty = true;
        state.workspace.content.netlist_source_dirty = true;
        state.workspace.content.project_sources_dirty = true;
        state
            .workspace
            .for_each_schematic_editor_mut(|_, schematic| {
                schematic.session.is_dirty = true;
            });
        for view in &mut state.workspace.content.open_views {
            view.dirty = true;
        }
        state.library_manager.set_all_views_modified_runtime(true);
        return;
    }
    let cell_dirty = state
        .project_lifecycle
        .registry
        .records()
        .iter()
        .filter_map(|record| match &record.id {
            ProjectDocumentId::CellView(reference) => Some((reference.clone(), record.dirty)),
            _ => None,
        })
        .collect::<Vec<_>>();
    for (reference, dirty) in &cell_dirty {
        if let Some(mut buffer) = state.workspace.schematic_editor_mut(&reference.key()) {
            buffer.editor.session.is_dirty = *dirty;
        }
        if let Some(open) = state
            .workspace
            .content
            .open_views
            .iter_mut()
            .find(|open| open.reference == *reference)
        {
            open.dirty = *dirty;
        }
        let _ = state.library_manager.set_view_modified_runtime(
            &reference.library,
            &reference.cell,
            &reference.view,
            *dirty,
        );
    }
    if let Some((_, dirty)) = cell_dirty
        .iter()
        .find(|(reference, _)| *reference == state.workspace.content.active_view)
    {
        state.schematic.session.is_dirty = *dirty;
    }
    // Project setup is dirty when it differs from the accepted baseline and at
    // no other time. A flag that only ever turned on left the unsaved marker
    // lit for the rest of the session, including after the save that published
    // the very edit that lit it.
    state.workspace.content.project_metadata_dirty = state
        .project_lifecycle
        .registry
        .is_dirty(&ProjectDocumentId::ProjectConfiguration);
    if let Some(accepted) = state.project_lifecycle.accepted.as_ref() {
        let baseline = &accepted.baseline().workspace;
        state.workspace.content.netlist_source_dirty = state.workspace.content.netlist_source
            != baseline.netlist_source
            || state.workspace.content.netlist_source_path != baseline.netlist_source_path
            || state.workspace.content.netlist_document != baseline.netlist_document
            || state.workspace.content.netlist_descriptor != baseline.netlist_descriptor
            || state.workspace.content.retained_netlist_decks != baseline.retained_netlist_decks;
        state.workspace.content.project_sources_dirty =
            state.workspace.content.project_sources != baseline.project_sources;
    }
}

pub(crate) fn initialize_from_session(state: &mut AppState) {
    state.project_lifecycle.authority.open_session();
    #[cfg(not(target_arch = "wasm32"))]
    {
        let restored_path = state.workspace.content.project.path.clone();
        let receipt = state.native_project_binding_receipt.clone();
        match (restored_path, receipt) {
            (Some(path), Some(receipt)) => {
                let session_project_id = state.workspace.content.project.id().to_string();
                match persistence::restore_native_binding(&path, &session_project_id, &receipt) {
                    Ok((baseline, binding)) => {
                        state.project_lifecycle.accepted =
                            Some(AcceptedProject::new(baseline, Some(binding)));
                        state.project_lifecycle.authority.restore_accepted_content();
                        state.browser_project_binding_receipt = None;
                    }
                    Err(error) => {
                        if path.exists() {
                            let canonical_path = persistence::normalize_native_path(&path)
                                .unwrap_or_else(|_| path.clone());
                            state.project_lifecycle.unreadable_native_binding =
                                Some(persistence::UnreadableNativeBinding {
                                    canonical_path,
                                    reason: error.to_string(),
                                });
                        }
                        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                            format!(
                                "Canonical native project was not restored: {error}. The remembered file was left untouched; open it explicitly or save an independent project copy"
                            ),
                        ));
                    }
                }
            }
            (Some(path), None) if path.exists() => {
                let canonical_path =
                    persistence::normalize_native_path(&path).unwrap_or_else(|_| path.clone());
                let reason = "the legacy session has no exact native binding receipt".to_owned();
                state.project_lifecycle.unreadable_native_binding =
                    Some(persistence::UnreadableNativeBinding {
                        canonical_path,
                        reason: reason.clone(),
                    });
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                    "Canonical native project was not restored because {reason}; open it explicitly to accept its current bytes"
                )));
            }
            (None, Some(_)) => {
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                    "Ignored a native binding receipt without its exact restored pathname",
                ));
            }
            (Some(_), None) | (None, None) => {}
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(receipt) = state.browser_project_binding_receipt.clone() {
            let project_id = state.workspace.content.project.id().to_string();
            if receipt.project_id != project_id {
                state.browser_project_binding_receipt = None;
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                    "Ignored a stale browser binding receipt for a different project identity",
                ));
            } else {
                state.project_lifecycle.authority.begin_browser_restore();
                let context = browser_operation_context(state);
                persistence::start_browser_binding_restore(receipt, move |result| {
                    BROWSER_BINDING_RESTORE_RESULTS.with(|queue| {
                        queue
                            .borrow_mut()
                            .push_back(BrowserRestoreCompletion { context, result });
                    });
                    crate::workbench::browser::file_import::request_browser_import_repaint();
                });
            }
        }
    }
    let _ = refresh_registry(state);
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn poll_browser_binding_restore(state: &mut AppState) {
    let Some(completion) =
        BROWSER_BINDING_RESTORE_RESULTS.with(|queue| queue.borrow_mut().pop_front())
    else {
        return;
    };
    if !browser_operation_context_is_current(state, &completion.context) {
        release_restore_result_handle(completion.result);
        return;
    }
    state.project_lifecycle.authority.finish_browser_restore();
    match completion.result {
        persistence::BrowserRestoreResult::Missing => {
            state.browser_project_binding_receipt = None;
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                "The browser canonical-binding receipt has no matching restoration record; choose Save to establish a new canonical binding",
            ));
        }
        persistence::BrowserRestoreResult::Restored { baseline, binding } => {
            if baseline.file.workspace.project.id() != state.workspace.content.project.id() {
                release_browser_binding_handle(&binding);
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                    "Ignored a stale browser project binding for a different project identity",
                ));
                return;
            }
            state.project_lifecycle.accepted = Some(AcceptedProject::new(*baseline, Some(binding)));
            state.native_project_binding_receipt = None;
            state.project_lifecycle.authority.accept_content();
            let _ = refresh_registry(state);
        }
        persistence::BrowserRestoreResult::ReconnectRequired { binding } => {
            state.project_lifecycle.browser_reconnect_binding = Some(binding);
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                "The canonical browser project needs permission again. Choose Save to reconnect under browser user activation; no bytes will be overwritten unless the accepted digest still matches.",
            ));
        }
        persistence::BrowserRestoreResult::Conflict {
            binding,
            observed_digest,
            reason,
        } => {
            state.project_lifecycle.browser_conflict = Some(BrowserConflict {
                binding,
                observed_digest,
            });
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "Canonical browser project conflict: {reason}. Ordinary Save is blocked; reopen it or save an independent project copy"
            )));
        }
        persistence::BrowserRestoreResult::Evicted(reason) => {
            state.browser_project_binding_receipt = None;
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                "Canonical browser project binding was removed: {reason}. Choose Save to select a canonical file again; download fallback remains copy-only."
            )));
        }
        persistence::BrowserRestoreResult::Retryable(reason) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                "Canonical browser project could not be restored yet: {reason}. Its restoration record was retained"
            )));
        }
        persistence::BrowserRestoreResult::Unsupported(reason) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                "Canonical browser saves are unavailable: {reason}"
            )));
        }
    }
}

pub(crate) fn accept_loaded_project(
    state: &mut AppState,
    baseline: ProjectSnapshot,
    binding: Option<PersistenceBinding>,
) {
    state.workbench.clear_project_model_editor();
    state.clear_project_design_history();
    state.dialogs.check_and_save.close();
    #[cfg(not(target_arch = "wasm32"))]
    let native_receipt = binding
        .as_ref()
        .map(|binding| binding.native_receipt(&baseline.file.workspace.project.id().to_string()));
    #[cfg(target_arch = "wasm32")]
    release_replaced_browser_bindings(&state.project_lifecycle, binding.as_ref());
    state.project_lifecycle.authority.open_session();
    state.project_lifecycle.accepted = Some(AcceptedProject::new(baseline, binding));
    state.project_lifecycle.authority.accept_content();
    state.project_lifecycle.unreadable_native_binding = None;
    #[cfg(not(target_arch = "wasm32"))]
    {
        state.native_project_binding_receipt = native_receipt;
        state.browser_project_binding_receipt = None;
    }
    #[cfg(target_arch = "wasm32")]
    {
        state.native_project_binding_receipt = None;
        state.browser_project_binding_receipt = state
            .project_lifecycle
            .accepted
            .as_ref()
            .and_then(|accepted| accepted.binding.as_ref())
            .and_then(PersistenceBinding::durable_browser_receipt);
        state.project_lifecycle.browser_reconnect_binding = None;
        state.project_lifecycle.browser_conflict = None;
        state.project_lifecycle.authority.clear_browser_pending();
    }
    state.project_lifecycle.authority.cancel_transaction();
    let _ = refresh_registry(state);
}

pub(crate) fn reset_for_new_project(state: &mut AppState) {
    state.workbench.clear_project_model_editor();
    state.clear_project_design_history();
    state.dialogs.check_and_save.close();
    state.native_project_binding_receipt = None;
    state.browser_project_binding_receipt = None;
    #[cfg(target_arch = "wasm32")]
    {
        clear_browser_handles();
    }
    let mut authority = std::mem::take(&mut state.project_lifecycle.authority);
    authority.reset_for_new_project();
    *state.project_lifecycle = ProjectLifecycleState {
        authority,
        ..ProjectLifecycleState::default()
    };
    let _ = refresh_registry(state);
}

pub(crate) fn mark_project_closed(state: &mut AppState) {
    state.workbench.clear_project_model_editor();
    // Stimulus drafts belong to the library of the project that is going
    // away. The close guard has already named every unapplied one, so what is
    // left here is bookkeeping: carrying them into the next project would
    // offer to publish a revision of a definition it does not hold.
    state.workbench.selected_stimulus_definition = None;
    state.workbench.stimulus_editor.clear();
    state.workbench.stimulus_browser.clear();
    state.clear_project_design_history();
    state.dialogs.check_and_save.close();
    state.native_project_binding_receipt = None;
    state.browser_project_binding_receipt = None;
    #[cfg(target_arch = "wasm32")]
    {
        clear_browser_handles();
    }
    let mut authority = std::mem::take(&mut state.project_lifecycle.authority);
    authority.close_project();
    *state.project_lifecycle = ProjectLifecycleState {
        authority,
        ..ProjectLifecycleState::default()
    };
}

#[cfg(target_arch = "wasm32")]
fn release_replaced_browser_bindings(
    lifecycle: &ProjectLifecycleState,
    retained: Option<&PersistenceBinding>,
) {
    let retained = browser_binding_handle_id(retained);
    let accepted = lifecycle
        .accepted
        .as_ref()
        .and_then(|accepted| browser_binding_handle_id(accepted.binding.as_ref()));
    let reconnect = browser_binding_handle_id(lifecycle.browser_reconnect_binding.as_ref());
    let conflict = lifecycle
        .browser_conflict
        .as_ref()
        .and_then(|conflict| browser_binding_handle_id(Some(&conflict.binding)));
    for handle_id in [accepted, reconnect, conflict].into_iter().flatten() {
        if Some(handle_id) != retained {
            persistence::release_browser_handle(handle_id);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn browser_binding_handle_id(binding: Option<&PersistenceBinding>) -> Option<u64> {
    binding.map(|binding| match binding {
        PersistenceBinding::Browser { handle_id, .. } => *handle_id,
    })
}

#[cfg(target_arch = "wasm32")]
fn release_browser_binding_handle(binding: &PersistenceBinding) {
    if let Some(handle_id) = browser_binding_handle_id(Some(binding)) {
        persistence::release_browser_handle(handle_id);
    }
}

#[cfg(target_arch = "wasm32")]
fn release_restore_result_handle(result: persistence::BrowserRestoreResult) {
    match result {
        persistence::BrowserRestoreResult::Restored { binding, .. }
        | persistence::BrowserRestoreResult::ReconnectRequired { binding }
        | persistence::BrowserRestoreResult::Conflict { binding, .. } => {
            release_browser_binding_handle(&binding);
        }
        persistence::BrowserRestoreResult::Missing
        | persistence::BrowserRestoreResult::Evicted(_)
        | persistence::BrowserRestoreResult::Retryable(_)
        | persistence::BrowserRestoreResult::Unsupported(_) => {}
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_operation_context(state: &AppState) -> BrowserOperationContext {
    state.project_lifecycle.authority.browser_operation_context(
        &state.workspace.content.project.id().to_string(),
        state.browser_project_binding_receipt.as_ref(),
    )
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_operation_context_is_current(
    state: &AppState,
    context: &BrowserOperationContext,
) -> bool {
    state
        .project_lifecycle
        .authority
        .browser_operation_context_is_current(
            context,
            &state.workspace.content.project.id().to_string(),
            state.browser_project_binding_receipt.as_ref(),
        )
}

/// Relinquish app-side authority for a browser promise that the platform may
/// not be able to abort. The operation generation makes every eventual late
/// completion stale, while accepted project/file authority remains intact.
#[cfg(target_arch = "wasm32")]
pub(crate) fn cancel_pending_browser_operation(state: &mut AppState) -> bool {
    let Some(restore_was_pending) = state.project_lifecycle.authority.cancel_browser_operation()
    else {
        return false;
    };
    if restore_was_pending {
        // A cancelled restore established no restart authority.
        state.browser_project_binding_receipt = None;
    }
    true
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn canonical_native_path(state: &AppState) -> Option<PathBuf> {
    state
        .project_lifecycle
        .canonical_native_path()
        .map(Path::to_path_buf)
}

#[cfg(all(not(target_arch = "wasm32"), test))]
pub(crate) fn normalize_native_path(path: &Path) -> Result<PathBuf, ProjectLifecycleError> {
    persistence::normalize_native_path(path).map_err(ProjectLifecycleError::from)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn read_native_binding(
    path: &Path,
) -> Result<(ProjectSnapshot, PersistenceBinding), ProjectLifecycleError> {
    persistence::read_native_binding(path).map_err(ProjectLifecycleError::from)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn save_native(
    state: &mut AppState,
    requested_scope: SaveScope,
    path: &Path,
    authority: DestinationAuthority,
) -> Result<(), ProjectLifecycleError> {
    require_open_project(state)?;
    require_project_writable(state)?;
    let path = persistence::normalize_native_path(path)?;
    if let Some(unreadable) = state.project_lifecycle.unreadable_native_binding.as_ref()
        && unreadable.canonical_path == path
    {
        return Err(ProjectLifecycleError::UnreadableCanonical(
            unreadable.reason.clone(),
        ));
    }
    let expected = match authority {
        DestinationAuthority::Canonical => state
            .project_lifecycle
            .accepted
            .as_ref()
            .and_then(|accepted| accepted.binding.as_ref())
            .filter(|binding| binding.canonical_path() == Some(path.as_path()))
            .map(PersistenceBinding::accepted_digest)
            .map(|digest| crate::io::durable_file::ExpectedContent::Digest(*digest.as_bytes()))
            .ok_or_else(|| {
                ProjectLifecycleError::UnreadableCanonical(
                    "no exact accepted byte baseline exists for this pathname".to_owned(),
                )
            })?,
        DestinationAuthority::UserSelected => persistence::observe_native_destination(&path)?,
    };
    let scope = effective_save_scope(state, requested_scope);
    state.project_lifecycle.authority.begin_save()?;

    let result = (|| {
        let working = snapshot(state)?;
        let mut candidate = match scope {
            SaveScope::AllDocuments => working.clone(),
            SaveScope::ActiveDocument => state
                .project_lifecycle
                .accepted
                .as_ref()
                .ok_or(ProjectLifecycleError::NoAcceptedBaseline)?
                .document_candidate(&working, &active_document(state))?,
        };
        candidate.file.workspace.project.set_path(path.clone());
        // Build every fallible post-save document digest before publishing.
        // Once the durable file replacement succeeds, adoption below is an
        // in-memory, infallible state transition.
        let post_save_registry =
            prepare_post_save_registry(state, &candidate, scope, SnapshotContent::Current)?;
        let (bytes, _) = persistence::serialized_project(&candidate.file)?;
        let digest = persistence::publish_canonical_native(&path, expected, &bytes)?;
        let binding = PersistenceBinding::Native {
            canonical_path: path.clone(),
            accepted_digest: digest,
        };
        adopt_successful_save(state, candidate, binding, scope, post_save_registry);
        Ok(())
    })();
    state.project_lifecycle.authority.cancel_transaction();
    result
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn save_project_copy_native(
    state: &mut AppState,
    path: &Path,
) -> Result<(), ProjectLifecycleError> {
    require_open_project(state)?;
    require_project_writable(state)?;
    let path = persistence::normalize_native_path(path)?;
    let canonical_source = state.project_lifecycle.canonical_native_path();
    let unreadable_source = state
        .project_lifecycle
        .unreadable_native_binding
        .as_ref()
        .map(|unreadable| unreadable.canonical_path.as_path());
    for source in canonical_source.into_iter().chain(unreadable_source) {
        if source == path || persistence::native_paths_refer_to_same_file(source, &path)? {
            return Err(ProjectLifecycleError::CopyDestinationIsCanonical);
        }
    }
    let expected = persistence::observe_native_destination(&path)?;
    state.project_lifecycle.authority.begin_save()?;
    let result = (|| {
        let mut copy = snapshot(state)?;
        copy.file.workspace.project = copy.file.workspace.project.fork_copy_at(path.clone());
        let (bytes, _) = persistence::serialized_project(&copy.file)?;
        // The picker authorizes this destination, while the captured exact
        // state still prevents a late create/edit from being overwritten.
        // The source project's accepted baseline and binding never change.
        let _ = persistence::publish_canonical_native(&path, expected, &bytes)?;
        Ok(())
    })();
    state.project_lifecycle.authority.cancel_transaction();
    result
}

#[cfg(target_arch = "wasm32")]
pub(crate) struct BrowserPreparedSave {
    pub(crate) transaction: TransactionId,
    pub(crate) context: BrowserOperationContext,
    pub(crate) candidate: ProjectSnapshot,
    pub(crate) scope: SaveScope,
    /// Stable identity of the active document captured with the serialized
    /// snapshot. Active-document continuations must not act on a different tab
    /// that became active while the browser picker or permission prompt waited.
    pub(crate) saved_document: ProjectDocumentId,
    pub(crate) project_copy: bool,
    pub(crate) suggested_name: String,
    pub(crate) bytes: Vec<u8>,
    pub(crate) staged_digest: ContentDigest,
    pub(crate) target: BrowserWriteTarget,
    pub(crate) source_handle_id: Option<u64>,
}

#[cfg(target_arch = "wasm32")]
pub(crate) struct BrowserSavePublication {
    pub(crate) handle_id: u64,
    pub(crate) binding_id: uuid::Uuid,
    pub(crate) backend: BrowserBindingBackend,
    pub(crate) project_id: String,
    pub(crate) generation: u64,
    pub(crate) display_name: String,
    pub(crate) digest: ContentDigest,
    pub(crate) durable: bool,
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn prepare_browser_save(
    state: &mut AppState,
    requested_scope: SaveScope,
    project_copy: bool,
    suggested_name: String,
) -> Result<BrowserPreparedSave, ProjectLifecycleError> {
    require_open_project(state)?;
    require_project_writable(state)?;
    state
        .project_lifecycle
        .authority
        .require_browser_binding_ready()?;
    if !project_copy && let Some(conflict) = state.project_lifecycle.browser_conflict.as_ref() {
        let _ = conflict.observed_digest;
        return Err(ProjectLifecycleError::BrowserExternalChange);
    }
    let scope = effective_save_scope(state, requested_scope);
    let transaction = state.project_lifecycle.authority.begin_save()?;
    let context = browser_operation_context(state);
    let saved_document = active_document(state);
    let result = (|| {
        let working = snapshot(state)?;
        let mut candidate = if project_copy || scope == SaveScope::AllDocuments {
            working
        } else {
            state
                .project_lifecycle
                .accepted
                .as_ref()
                .ok_or(ProjectLifecycleError::NoAcceptedBaseline)?
                .document_candidate(&working, &saved_document)?
        };
        if project_copy {
            candidate.file.workspace.project = candidate
                .file
                .workspace
                .project
                .fork_copy_at(PathBuf::from(&suggested_name));
        } else {
            candidate.file.workspace.project.path = None;
        }
        let (bytes, staged_digest) = persistence::serialized_project(&candidate.file)?;
        let existing_binding = (!project_copy)
            .then(|| {
                state
                    .project_lifecycle
                    .accepted
                    .as_ref()
                    .and_then(|accepted| accepted.binding.as_ref())
                    .or(state.project_lifecycle.browser_reconnect_binding.as_ref())
            })
            .flatten();
        let source_handle_id = state
            .project_lifecycle
            .accepted
            .as_ref()
            .and_then(|accepted| accepted.binding.as_ref())
            .or(state.project_lifecycle.browser_reconnect_binding.as_ref())
            .or(state
                .project_lifecycle
                .browser_conflict
                .as_ref()
                .map(|conflict| &conflict.binding))
            .map(|binding| match binding {
                PersistenceBinding::Browser { handle_id, .. } => *handle_id,
            });
        let project_id = candidate.file.workspace.project.id().to_string();
        let target = if let Some(PersistenceBinding::Browser {
            handle_id,
            binding_id,
            backend,
            project_id: binding_project_id,
            accepted_generation,
            accepted_digest,
            persisted_generation,
            ..
        }) = existing_binding
        {
            if *binding_project_id != project_id {
                return Err(ProjectLifecycleError::InvalidState(
                    "browser binding belongs to a different logical project".to_owned(),
                ));
            }
            persistence::BrowserWriteTarget {
                handle_id: Some(*handle_id),
                binding_id: *binding_id,
                backend: *backend,
                project_id,
                accepted_generation: accepted_generation.saturating_add(1).max(1),
                expected_digest: Some(*accepted_digest),
                persisted_generation: *persisted_generation,
            }
        } else {
            let backend = if project_copy || persistence::browser_external_canonical_supported() {
                BrowserBindingBackend::ExternalFile
            } else if persistence::browser_opfs_supported() {
                BrowserBindingBackend::Opfs
            } else {
                BrowserBindingBackend::ExternalFile
            };
            persistence::BrowserWriteTarget {
                handle_id: None,
                binding_id: uuid::Uuid::new_v4(),
                backend,
                project_id,
                accepted_generation: 1,
                expected_digest: None,
                persisted_generation: None,
            }
        };
        Ok(BrowserPreparedSave {
            transaction,
            context,
            candidate,
            scope,
            saved_document,
            project_copy,
            suggested_name,
            bytes,
            staged_digest,
            target,
            source_handle_id,
        })
    })();
    if result.is_err() {
        state.project_lifecycle.authority.cancel_transaction();
    }
    result
}

/// Decide whether a successfully persisted snapshot still authorizes the
/// destructive action that requested it. Saving and continuing are separate:
/// edits made while a browser surface is pending remain dirty and force a new
/// review instead of being discarded.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn saved_snapshot_authorizes_continuation(
    state: &AppState,
    scope: SaveScope,
    saved_document: &ProjectDocumentId,
) -> bool {
    match scope {
        SaveScope::AllDocuments => !has_unsaved_changes(state),
        SaveScope::ActiveDocument => {
            active_document(state) == *saved_document && !active_document_is_dirty(state)
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_file_picker_supported() -> bool {
    persistence::browser_file_picker_supported()
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_canonical_save_supported() -> bool {
    // IndexedDB restoration is a durable convenience, not a prerequisite for
    // a verified live-session canonical binding. A storage failure is reported
    // explicitly as session-only after the file bytes themselves are saved.
    persistence::browser_external_canonical_supported() || persistence::browser_opfs_supported()
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_open_file_picker_supported() -> bool {
    persistence::browser_open_file_picker_supported()
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn start_browser_write(
    target: BrowserWriteTarget,
    persist_binding: bool,
    suggested_name: &str,
    bytes: Vec<u8>,
    complete: impl FnOnce(persistence::BrowserWriteResult) + 'static,
) -> Result<(), String> {
    persistence::start_browser_write(target, persist_binding, suggested_name, bytes, complete)
}

#[cfg(target_arch = "wasm32")]
pub(crate) use persistence::BrowserWriteResult;

#[cfg(target_arch = "wasm32")]
pub(crate) use persistence::BrowserOpenResult;

#[cfg(target_arch = "wasm32")]
pub(crate) fn start_browser_open(
    complete: impl FnOnce(persistence::BrowserOpenResult) + 'static,
) -> Result<(), String> {
    persistence::start_browser_open(complete)
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn start_browser_binding_persist(
    binding: PersistenceBinding,
    complete: impl FnOnce(Result<(), String>) + 'static,
) {
    persistence::start_browser_binding_persist(binding, complete);
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn release_browser_handle(handle_id: u64) {
    persistence::release_browser_handle(handle_id);
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn release_browser_handle_if_unowned(state: &AppState, handle_id: u64) {
    if !browser_handle_is_current(state, handle_id) {
        persistence::release_browser_handle(handle_id);
    }
}

#[cfg(target_arch = "wasm32")]
fn clear_browser_handles() {
    persistence::clear_browser_handles();
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn complete_browser_save(
    state: &mut AppState,
    prepared: BrowserPreparedSave,
    publication: BrowserSavePublication,
) -> Result<(), ProjectLifecycleError> {
    let BrowserSavePublication {
        handle_id,
        binding_id,
        backend,
        project_id,
        generation,
        display_name,
        digest,
        durable,
    } = publication;
    let current = state
        .project_lifecycle
        .authority
        .is_current_transaction(prepared.transaction);
    if !current || !browser_operation_context_is_current(state, &prepared.context) {
        persistence::release_browser_handle(handle_id);
        return Err(ProjectLifecycleError::TransactionInProgress);
    }
    if prepared.project_copy {
        persistence::release_browser_handle(handle_id);
        state.project_lifecycle.authority.cancel_transaction();
        return Ok(());
    }
    if binding_id != prepared.target.binding_id
        || backend != prepared.target.backend
        || project_id != prepared.target.project_id
        || generation != prepared.target.accepted_generation
        || digest != prepared.staged_digest
    {
        persistence::release_browser_handle(handle_id);
        state.project_lifecycle.authority.cancel_transaction();
        return Err(ProjectLifecycleError::InvalidState(
            "browser binding identity changed during save completion".to_owned(),
        ));
    }
    let binding = PersistenceBinding::Browser {
        handle_id,
        binding_id,
        backend,
        project_id,
        accepted_generation: generation,
        display_name,
        accepted_digest: digest,
        // Keep the last generation that is known to exist in IndexedDB when
        // this publication is session-only. The new file bytes are canonical
        // for this live tab, but the next retry must CAS from durable storage.
        persisted_generation: persistence::persisted_generation_after_browser_write(
            durable,
            generation,
            prepared.target.persisted_generation,
        ),
    };
    finish_successful_save(state, prepared.candidate, binding, prepared.scope);
    state.project_lifecycle.authority.cancel_transaction();
    Ok(())
}

#[cfg(any(test, target_arch = "wasm32"))]
fn finish_successful_save(
    state: &mut AppState,
    candidate: ProjectSnapshot,
    binding: PersistenceBinding,
    scope: SaveScope,
) {
    // The browser has already verified publication of this exact candidate.
    // A newer working draft can be invalid without revoking that publication.
    // Adopt its binding even when comparison fails, so the next save expects
    // the bytes actually on disk and recovery retains the written baseline.
    let post_save_registry = match prepare_post_save_registry(
        state,
        &candidate,
        scope,
        SnapshotContent::Current,
    ) {
        Ok(registry) => registry,
        Err(error) => {
            let mut registry = state.project_lifecycle.registry.clone();
            registry.invalidate();
            state.push_user_message(ConsoleMessage::warning(format!(
                "The saved snapshot was accepted, but the current draft could not be compared with it: {error}. Current edits remain pending; resolve the draft error before saving or closing."
            )));
            registry
        }
    };
    adopt_successful_save(state, candidate, binding, scope, post_save_registry);
}

fn prepare_post_save_registry(
    state: &AppState,
    candidate: &ProjectSnapshot,
    scope: SaveScope,
    content: SnapshotContent,
) -> Result<registry::DocumentRegistry, ProjectLifecycleError> {
    #[cfg(not(target_arch = "wasm32"))]
    let mut current = capture_snapshot(state, content)?;
    #[cfg(target_arch = "wasm32")]
    let current = capture_snapshot(state, content)?;
    #[cfg(not(target_arch = "wasm32"))]
    {
        if scope == SaveScope::AllDocuments
            || active_document(state) == ProjectDocumentId::ProjectConfiguration
        {
            current.file.workspace.project = candidate.file.workspace.project.clone();
        } else {
            current.file.workspace.project.path = candidate.file.workspace.project.path.clone();
        }
    }
    #[cfg(target_arch = "wasm32")]
    let _ = scope;
    let mut post_save_registry = registry::DocumentRegistry::default();
    let cache = &state.project_lifecycle.result_fingerprints;
    let candidate_fingerprints =
        registry::document_fingerprints_with_results_cache(&candidate.file, cache)
            .map_err(ProjectLifecycleError::InvalidState)?;
    let current_fingerprints =
        registry::document_fingerprints_with_results_cache(&current.file, cache)
            .map_err(ProjectLifecycleError::InvalidState)?;
    post_save_registry
        .rebuild_from_fingerprints(&current_fingerprints, Some(&candidate_fingerprints));
    Ok(post_save_registry)
}

fn rebase_pending_operation_dirty_state(
    state: &mut AppState,
    candidate: &ProjectSnapshot,
    scope: SaveScope,
) {
    if !state.schematic.has_pending_operation()
        && !state
            .workspace
            .content
            .schematic_buffers
            .values()
            .any(|schematic| schematic.pending_operation_id().is_some())
    {
        return;
    }
    // Compare cancellation baselines through the same complete document
    // projection used by dirty indicators. A delayed save may acknowledge an
    // older baseline, and a newer invalid draft cannot be declared clean.
    let comparison =
        prepare_post_save_registry(state, candidate, scope, SnapshotContent::Committed);
    let dirty = |key: &str| {
        comparison
            .as_ref()
            .ok()
            .and_then(|registry| {
                registry
                    .records()
                    .iter()
                    .find_map(|record| match &record.id {
                        ProjectDocumentId::CellView(reference) if reference.key() == key => {
                            Some(record.dirty)
                        }
                        _ => None,
                    })
            })
            .unwrap_or(true)
    };
    if state.schematic.has_pending_operation() {
        let was_dirty = dirty(&state.workspace.content.active_schematic_reference().key());
        state.schematic.set_pending_was_dirty(was_dirty);
    }
    state
        .workspace
        .for_each_schematic_editor_mut(|key, schematic| {
            if schematic.has_pending_operation() {
                schematic.set_pending_was_dirty(dirty(key));
            }
        });
}

fn adopt_successful_save(
    state: &mut AppState,
    candidate: ProjectSnapshot,
    binding: PersistenceBinding,
    scope: SaveScope,
    post_save_registry: registry::DocumentRegistry,
) {
    rebase_pending_operation_dirty_state(state, &candidate, scope);
    #[cfg(not(target_arch = "wasm32"))]
    let native_receipt = binding.native_receipt(&candidate.file.workspace.project.id().to_string());
    #[cfg(target_arch = "wasm32")]
    let browser_receipt = binding.durable_browser_receipt();
    #[cfg(target_arch = "wasm32")]
    release_replaced_browser_bindings(&state.project_lifecycle, Some(&binding));
    #[cfg(not(target_arch = "wasm32"))]
    {
        if scope == SaveScope::AllDocuments
            || active_document(state) == ProjectDocumentId::ProjectConfiguration
        {
            state
                .retain_annotation_history_after_save_descriptor(&candidate.file.workspace.project);
            state.workspace.content.project = candidate.file.workspace.project.clone();
        } else {
            // Saving one non-configuration document intentionally publishes a
            // candidate built on the accepted project descriptor. Preserve a
            // concurrent/unrelated live descriptor draft and update only the
            // canonical pathname established by this successful save.
            state.workspace.content.project.path = candidate.file.workspace.project.path.clone();
        }
    }
    state.project_lifecycle.accepted = Some(AcceptedProject::new(candidate, Some(binding)));
    #[cfg(not(target_arch = "wasm32"))]
    {
        state.native_project_binding_receipt = Some(native_receipt);
        state.browser_project_binding_receipt = None;
    }
    #[cfg(target_arch = "wasm32")]
    {
        state.native_project_binding_receipt = None;
        state.browser_project_binding_receipt = browser_receipt;
        state.project_lifecycle.browser_reconnect_binding = None;
        state.project_lifecycle.browser_conflict = None;
    }
    state.project_lifecycle.authority.accept_content();
    #[cfg(not(target_arch = "wasm32"))]
    match scope {
        SaveScope::AllDocuments => {
            state.workspace.mark_all_clean();
            state.schematic.session.is_dirty = false;
            mark_all_library_views_clean(&mut state.library_manager);
        }
        SaveScope::ActiveDocument => mark_active_document_clean(state),
    }
    #[cfg(target_arch = "wasm32")]
    let _ = scope;
    // Native publication requires a fully built comparison. A browser write
    // may finish while the newer draft is invalid; its registry then marks
    // comparisons unverified. Neither case can erase pending authored edits.
    state.project_lifecycle.registry = post_save_registry;
    apply_registry_dirty_flags(state);
    report_design_checks_after_save(state);
}

/// Report what the design checks find, when a completed save was asked to run
/// them.
///
/// The order is the contract. Publication has already happened above, so a
/// finding raised here cannot refuse the save: a check is advice about the
/// drawing, and advice that could veto a save the author explicitly asked for
/// would be a gate wearing a preference's clothes.
///
/// The explicit check command owns the full per-finding listing. A save that
/// nobody asked to review states the outcome and offers the worst finding as
/// the way into the evidence this run just published — the canvas badges, the
/// check pill, and the finding-navigation commands all read it.
fn report_design_checks_after_save(state: &mut AppState) {
    if !state
        .ui
        .preferences
        .toggle(crate::workbench::TogglePreference::RunDesignChecksOnSave)
    {
        return;
    }
    let result = match state.run_active_design_checks() {
        Ok(result) => result,
        Err(error) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "Design checks on save could not create current evidence: {error}"
            )));
            return;
        }
    };
    let summary = result.summary();
    let message = format!(
        "Design checks on save: {} critical, {} errors, {} warnings \u{00b7} the saved revision is already published",
        summary.critical, summary.errors, summary.warnings
    );
    state.push_user_message(if result.passed() {
        ConsoleMessage::info(message)
    } else {
        ConsoleMessage::warning(message)
    });
    if let Some(worst) = result
        .violations()
        .iter()
        .max_by_key(|violation| violation.severity)
    {
        let severity = if result.passed() {
            LogSeverity::Warning
        } else {
            LogSeverity::Error
        };
        let anchor = crate::schematic::view::violations::finding_anchor(state, worst);
        state.log_buffer.log_anchored(
            severity,
            LogSource::Drc,
            worst.message.clone(),
            None,
            anchor,
        );
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn record_browser_save_conflict(
    state: &mut AppState,
    prepared: &BrowserPreparedSave,
    observed_digest: ContentDigest,
) {
    if !browser_operation_context_is_current(state, &prepared.context) {
        return;
    }
    let binding = state
        .project_lifecycle
        .accepted
        .as_ref()
        .and_then(|accepted| accepted.binding.clone())
        .or_else(|| state.project_lifecycle.browser_reconnect_binding.clone());
    if let Some(binding) = binding {
        state.project_lifecycle.browser_conflict = Some(BrowserConflict {
            binding,
            observed_digest,
        });
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn complete_browser_binding_promotion(
    state: &mut AppState,
    context: &BrowserOperationContext,
    handle_id: u64,
    result: &Result<(), String>,
) -> bool {
    if !browser_operation_context_is_current(state, context) {
        if !browser_handle_is_current(state, handle_id) {
            persistence::release_browser_handle(handle_id);
        }
        return false;
    }
    state.project_lifecycle.authority.finish_browser_promotion();
    if result.is_ok()
        && let Some(PersistenceBinding::Browser {
            persisted_generation,
            accepted_generation,
            ..
        }) = state
            .project_lifecycle
            .accepted
            .as_mut()
            .and_then(|accepted| accepted.binding.as_mut())
    {
        *persisted_generation = Some(*accepted_generation);
    }
    state.browser_project_binding_receipt = state
        .project_lifecycle
        .accepted
        .as_ref()
        .and_then(|accepted| accepted.binding.as_ref())
        .and_then(PersistenceBinding::durable_browser_receipt);
    true
}

#[cfg(target_arch = "wasm32")]
fn browser_handle_is_current(state: &AppState, handle_id: u64) -> bool {
    let lifecycle = &state.project_lifecycle;
    lifecycle
        .accepted
        .as_ref()
        .and_then(|accepted| browser_binding_handle_id(accepted.binding.as_ref()))
        .into_iter()
        .chain(browser_binding_handle_id(
            lifecycle.browser_reconnect_binding.as_ref(),
        ))
        .chain(
            lifecycle
                .browser_conflict
                .as_ref()
                .and_then(|conflict| browser_binding_handle_id(Some(&conflict.binding))),
        )
        .any(|current| current == handle_id)
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn begin_browser_binding_promotion(state: &mut AppState) -> BrowserOperationContext {
    state.project_lifecycle.authority.begin_browser_promotion();
    browser_operation_context(state)
}

#[cfg(not(target_arch = "wasm32"))]
fn mark_active_document_clean(state: &mut AppState) {
    match active_document(state) {
        ProjectDocumentId::CellView(reference) => {
            if let Some(mut buffer) = state.workspace.schematic_editor_mut(&reference.key()) {
                buffer.editor.session.is_dirty = false;
            }
            if reference == state.workspace.content.active_view {
                state.schematic.session.is_dirty = false;
            }
            if let Some(open) = state
                .workspace
                .content
                .open_views
                .iter_mut()
                .find(|open| open.reference == reference)
            {
                open.dirty = false;
            }
            let _ = state.library_manager.mark_view_clean_runtime(
                &reference.library,
                &reference.cell,
                &reference.view,
            );
        }
        ProjectDocumentId::NetlistSource => {
            state.workspace.content.netlist_source_dirty = false;
            state.workspace.content.project_sources_dirty = false;
        }
        _ => {}
    }
}

pub(crate) fn prepare_revert_active_document(
    state: &AppState,
) -> Result<RevertReviewToken, ProjectLifecycleError> {
    require_open_project(state)?;
    let id = active_document(state);
    if state.simulation.has_active_execution()
        && matches!(
            id,
            ProjectDocumentId::SimulationPlan
                | ProjectDocumentId::ResultHistory
                | ProjectDocumentId::ModelCatalog
        )
    {
        return Err(ProjectLifecycleError::ActiveRun);
    }
    state.project_lifecycle.authority.prepare_revert(
        id,
        state
            .project_lifecycle
            .accepted
            .as_ref()
            .map(AcceptedProject::content),
    )
}

pub(crate) fn confirm_revert_active_document(
    state: &mut AppState,
    token: &RevertReviewToken,
) -> Result<(), ProjectLifecycleError> {
    require_open_project(state)?;
    state
        .project_lifecycle
        .authority
        .validate_revert(token, &active_document(state))?;
    let current = prepare_revert_active_document(state)?;
    if current != *token {
        return Err(ProjectLifecycleError::RevertReviewStale);
    }
    revert_document(state, token.document().clone())
}

fn revert_document(
    state: &mut AppState,
    id: ProjectDocumentId,
) -> Result<(), ProjectLifecycleError> {
    // Revert can cross document boundaries (for example configuration roots,
    // cell catalogs, source ownership, and editor focus). Build and validate
    // the complete result away from live state so a failed registry rebuild
    // can never leave a partially reverted project behind.
    let mut candidate = state.clone();
    revert_document_in_place(&mut candidate, id)?;
    refresh_registry(&mut candidate)?;
    *state = candidate;
    Ok(())
}

fn revert_document_in_place(
    state: &mut AppState,
    id: ProjectDocumentId,
) -> Result<(), ProjectLifecycleError> {
    let baseline = state
        .project_lifecycle
        .accepted
        .as_ref()
        .ok_or(ProjectLifecycleError::NoAcceptedBaseline)?
        .clone_snapshot();
    let baseline_project_id = baseline.file.workspace.project.id();

    match id {
        ProjectDocumentId::ProjectConfiguration => {
            let callback_receipts = baseline.file.workspace.pdk_callback_receipts().to_vec();
            state.workspace.content.project = baseline.file.workspace.project;
            state.workspace.content.configuration_sets = baseline.file.workspace.configuration_sets;
            state.workspace.content.design_management = baseline.file.workspace.design_management;
            state
                .workspace
                .content
                .replace_pdk_callback_receipts_for_lifecycle(callback_receipts)
                .map_err(ProjectLifecycleError::InvalidState)?;
            restore_project_structure_preserving_documents(state, baseline.file.libraries);
        }
        ProjectDocumentId::CellView(reference) => revert_cell_view(state, &baseline, &reference)?,
        ProjectDocumentId::SimulationPlan => {
            let context = baseline.file.execution_context.ok_or_else(|| {
                ProjectLifecycleError::InvalidState(
                    "accepted project has no simulation plan".to_owned(),
                )
            })?;
            state.sim_setup = crate::workbench::app_state::SimSetupState {
                setup: context.simulation_plan,
                session: Default::default(),
            };
            state.sim_setup.prepare_after_restore();
            state.workspace.content.simulation_plan_payloads =
                baseline.file.workspace.simulation_plan_payloads;
            if let Some(plan_id) = state
                .sim_setup
                .analysis_plan
                .as_ref()
                .map(crate::simulation::plan::SimulationPlan::id)
            {
                state
                    .workspace
                    .content
                    .sync_legacy_specs_projection(plan_id);
            }
        }
        ProjectDocumentId::ModelCatalog => {
            let context = baseline.file.execution_context.ok_or_else(|| {
                ProjectLifecycleError::InvalidState(
                    "accepted project has no model catalog".to_owned(),
                )
            })?;
            let (_, manager, warnings) =
                crate::io::restore_execution_context(context, baseline_project_id)
                    .map_err(ProjectLifecycleError::InvalidState)?;
            state.model_library_manager = manager;
            for warning in warnings {
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(warning));
            }
        }
        ProjectDocumentId::ResultHistory => {
            let mut simulation = crate::state::SimulationState::default();
            crate::io::restore_simulation_results(
                baseline.file.simulation_results,
                &mut simulation,
            )
            .map_err(ProjectLifecycleError::InvalidState)?;
            state.simulation = simulation;
            state.workspace.content.report_documents = baseline.file.workspace.report_documents;
            state.workspace.content.report_documents_dirty = false;
            state.workspace.content.visualization_documents =
                baseline.file.workspace.visualization_documents;
            state.workspace.content.visualization_documents_dirty = false;
            crate::workbench::documents::result_document::restore_presentation(
                state,
                baseline.file.result_presentation,
            );
            state.clear_specialized_viewer_data();
        }
        ProjectDocumentId::VerificationSpecifications => {
            state.workspace.content.specs = baseline.file.workspace.specs;
        }
        ProjectDocumentId::StimulusLibrary => {
            state.workspace.content.stimulus_library = baseline.file.workspace.stimulus_library;
            // The reverted definitions may no longer hold the one the library
            // browser was reading, and a selection that resolves to nothing
            // leaves the workspace on an empty stage with a name in the dock.
            state.workbench.selected_stimulus_definition = None;
            // The drafts belonged to the revisions that have just been thrown
            // away. Keeping one would leave the instrument editing a record
            // whose saved side no longer exists, and Apply would publish it as
            // a revision of a definition the revert removed.
            state.workbench.stimulus_editor.clear();
            state.workbench.stimulus_browser.clear();
        }
        ProjectDocumentId::NetlistSource => {
            state.workspace.content.netlist_source = baseline.file.workspace.netlist_source;
            state.workspace.content.netlist_source_path =
                baseline.file.workspace.netlist_source_path;
            state.workspace.content.netlist_document = baseline.file.workspace.netlist_document;
            state.workspace.content.netlist_descriptor = baseline.file.workspace.netlist_descriptor;
            state.workspace.content.retained_netlist_decks =
                baseline.file.workspace.retained_netlist_decks;
            state
                .workspace
                .content
                .project_sources
                .synchronize_code_workspace_bundles_from(&baseline.file.workspace.project_sources)
                .map_err(|error| ProjectLifecycleError::InvalidState(error.to_string()))?;
            state.workspace.content.netlist_source_dirty = false;
            state.workspace.content.project_sources_dirty = false;
            state.ui.netlist = Default::default();
            state.workbench.netlist_open_documents.clear();
            state.simulation.netlist_content = state
                .workspace
                .content
                .netlist_source
                .clone()
                .unwrap_or_default();
            state.simulation.trigger_simulation = false;
            state.ui.netlist.rerun_queued = false;
            state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
        }
    }
    Ok(())
}

/// The project documents with unsaved changes, in registry order.
///
/// A project with no accepted baseline, or one whose comparison fails, has
/// nothing to diff against: its configuration stands for the whole unsaved
/// project, as one document.
pub(crate) fn dirty_documents(state: &AppState) -> Vec<ProjectDocumentId> {
    if state.project_lifecycle.accepted.is_none() {
        return if state.project_lifecycle.is_open() {
            vec![ProjectDocumentId::ProjectConfiguration]
        } else {
            Vec::new()
        };
    }
    let Ok(registry) = current_registry(state) else {
        return vec![ProjectDocumentId::ProjectConfiguration];
    };
    registry
        .records()
        .iter()
        .filter(|record| record.dirty)
        .map(|record| record.id.clone())
        .collect()
}

pub(crate) fn dirty_document_count(state: &AppState) -> usize {
    dirty_documents(state).len()
}

pub(crate) fn can_close_active_document(state: &AppState) -> bool {
    state.project_lifecycle.is_open()
        && state.workbench.workspace == crate::workbench::state::Workspace::Design
        && state.workspace.content.open_views.len() > 1
}

pub(crate) fn close_active_document(state: &mut AppState) -> Result<(), ProjectLifecycleError> {
    require_open_project(state)?;
    if !can_close_active_document(state) {
        return Err(ProjectLifecycleError::LastPresentedDocument);
    }
    // Capture the live editor before removing only its presentation record.
    state.cancel_schematic_drag();
    state.sync_active_schematic_to_workspace();
    let closing = state.workspace.content.active_view.clone();
    state.workspace.content.close_view(&closing);
    state.restore_active_schematic_from_workspace();
    refresh_registry(state)
}

pub(crate) fn begin_project_replacement(
    state: &mut AppState,
) -> Result<TransactionId, ProjectLifecycleError> {
    if state.simulation.has_active_execution() {
        return Err(ProjectLifecycleError::ActiveRun);
    }
    state
        .project_lifecycle
        .authority
        .ensure_replacement_available()?;
    let content =
        registry::content_digest(&capture_snapshot(state, SnapshotContent::Current)?.file)
            .map_err(ProjectLifecycleError::InvalidState)?;
    state.project_lifecycle.authority.begin_replacement(content)
}

pub(crate) fn cancel_transaction(state: &mut AppState) {
    state.project_lifecycle.authority.cancel_transaction();
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn cancel_transaction_if(state: &mut AppState, id: TransactionId) -> bool {
    state.project_lifecycle.authority.cancel_transaction_if(id)
}

pub(crate) fn validate_project_replacement(
    state: &AppState,
    id: TransactionId,
) -> Result<(), ProjectLifecycleError> {
    if state.simulation.has_active_execution() {
        return Err(ProjectLifecycleError::ActiveRun);
    }
    state
        .project_lifecycle
        .authority
        .validate_replacement(id, || {
            registry::content_digest(&capture_snapshot(state, SnapshotContent::Current)?.file)
                .map_err(ProjectLifecycleError::InvalidState)
        })
}

fn require_open_project(state: &AppState) -> Result<(), ProjectLifecycleError> {
    state.project_lifecycle.authority.require_open_project()
}

fn require_project_writable(state: &AppState) -> Result<(), ProjectLifecycleError> {
    (!state.workbench.safe_mode.project_read_only())
        .then_some(())
        .ok_or(ProjectLifecycleError::SafeModeReadOnly)
}

fn revert_cell_view(
    state: &mut AppState,
    baseline: &ProjectSnapshot,
    reference: &CellViewRef,
) -> Result<(), ProjectLifecycleError> {
    let baseline_view = baseline
        .file
        .libraries
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .cloned();
    let Some(baseline_view) = baseline_view else {
        let removed = state
            .library_manager
            .edit_library(&reference.library)
            .and_then(|mut library| library.remove_view(&reference.cell, &reference.view))
            .unwrap_or(false);
        if !removed {
            return Err(ProjectLifecycleError::NoAcceptedBaseline);
        }
        let source_changed = state
            .workspace
            .content
            .project_sources
            .synchronize_cell_view_bundle_from(reference, &baseline.file.workspace.project_sources)
            .map_err(|error| ProjectLifecycleError::InvalidState(error.to_string()))?;
        state.prune_workspace_after_view_deleted(
            &reference.library,
            &reference.cell,
            &reference.view,
        );
        if source_changed {
            state.ui.code_workspace.veriloga = Default::default();
        }
        return Ok(());
    };
    state
        .library_manager
        .edit_library(&reference.library)
        .and_then(|mut library| library.add_view(&reference.cell, baseline_view))
        .ok_or_else(|| {
            ProjectLifecycleError::InvalidState("active cell no longer exists".to_owned())
        })?;
    let key = reference.key();
    match baseline.clone_schematic_editor(&key) {
        Some(buffer) => {
            state.workspace.insert_schematic_editor(key, buffer);
        }
        None => {
            state.workspace.remove_schematic_editor(&key);
        }
    }
    state
        .workspace
        .content
        .synchronize_physical_layout_document_from(reference, &baseline.file.workspace)
        .map_err(|error| ProjectLifecycleError::InvalidState(error.to_string()))?;
    if reference == &state.workspace.content.active_view {
        state.restore_active_schematic_from_workspace();
    }
    let source_changed = state
        .workspace
        .content
        .project_sources
        .synchronize_cell_view_bundle_from(reference, &baseline.file.workspace.project_sources)
        .map_err(|error| ProjectLifecycleError::InvalidState(error.to_string()))?;
    if source_changed {
        state.ui.code_workspace.veriloga = Default::default();
    }
    Ok(())
}

fn restore_project_structure_preserving_documents(
    state: &mut AppState,
    baseline: crate::state::LibraryManager,
) {
    let previous_references = state
        .library_manager
        .libraries_by_key()
        .flat_map(|(library_key, library)| {
            library.cells.iter().flat_map(move |(cell_key, cell)| {
                cell.views
                    .keys()
                    .map(move |view_key| CellViewRef::new(library_key, cell_key, view_key))
            })
        })
        .collect::<Vec<_>>();
    state.library_manager = baseline.with_document_content_from(&state.library_manager);
    let removed_references = previous_references
        .into_iter()
        .filter(|reference| {
            state
                .library_manager
                .get_library(&reference.library)
                .and_then(|library| library.get_cell(&reference.cell))
                .and_then(|cell| cell.get_view(&reference.view))
                .is_none()
        })
        .collect::<Vec<_>>();
    for reference in removed_references {
        state.prune_workspace_after_view_deleted(
            &reference.library,
            &reference.cell,
            &reference.view,
        );
    }
    let retained = state
        .library_manager
        .libraries_by_key()
        .flat_map(|(library_key, library)| {
            library.cells.iter().flat_map(move |(cell_key, cell)| {
                cell.views
                    .keys()
                    .map(move |view_key| CellViewRef::new(library_key, cell_key, view_key))
            })
        })
        .collect::<Vec<_>>();
    let removed = state
        .workspace
        .content
        .project_sources
        .retain_cell_view_bundles_for(retained);
    if state
        .ui
        .code_workspace
        .veriloga
        .receipt
        .as_ref()
        .is_some_and(|receipt| removed.contains(&receipt.token.bundle_id))
        || state
            .ui
            .code_workspace
            .veriloga
            .pending
            .as_ref()
            .is_some_and(|pending| removed.contains(&pending.token.bundle_id))
    {
        state.ui.code_workspace.veriloga = Default::default();
    }
    state
        .workspace
        .ensure_library_model(&mut state.library_manager);
}

#[cfg(not(target_arch = "wasm32"))]
fn mark_all_library_views_clean(libraries: &mut crate::state::LibraryManager) {
    libraries.mark_all_views_clean_runtime();
}

/// Remove presentation-only state from the serialized clone. This must never
/// be used on the live library manager: saving engineering content does not
/// close tabs or alter the user's presentation state.
fn sanitize_library_view_runtime_state(libraries: &mut crate::state::LibraryManager) {
    libraries.sanitize_views_for_persistence();
}

fn strip_schematic_runtime_state(schematic: &mut crate::state::SchematicState) {
    schematic.strip_runtime_for_project_save();
}

#[cfg(test)]
mod tests;
