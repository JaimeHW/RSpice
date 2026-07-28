//! Transactional project/document lifecycle.
//!
//! The accepted project baseline is intentionally separate from the mutable
//! workbench state. `Save` overlays one stable document onto that baseline;
//! `Save all` replaces it with the complete working set. This prevents saving
//! one tab from accidentally committing unrelated drafts.

mod persistence;
#[cfg(target_arch = "wasm32")]
pub(crate) use persistence::{
    start_browser_checkpoint_list, start_browser_checkpoint_publish, start_browser_checkpoint_read,
};
mod registry;
mod transaction;

#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use std::path::PathBuf;

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) use persistence::BrowserBindingBackend;
#[cfg(target_arch = "wasm32")]
pub(crate) use persistence::BrowserWriteTarget;
pub(crate) use persistence::{BrowserBindingReceipt, NativeBindingReceipt, PersistenceBinding};
pub(crate) use registry::ProjectDocumentId;
pub(crate) use transaction::TransactionId;
use transaction::{LifecycleTransaction, TransactionKind};

use crate::io::{ProjectExecutionContext, ProjectFile, ProjectSimulationResults};
#[cfg(target_arch = "wasm32")]
use crate::product::ContentDigest;
use crate::state::{CellViewRef, ViewType};
use crate::workbench::app_state::AppState;

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_BINDING_RESTORE_RESULTS: std::cell::RefCell<std::collections::VecDeque<BrowserRestoreCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BrowserOperationContext {
    epoch: u64,
    operation_generation: u64,
    project_id: String,
    binding_receipt: Option<BrowserBindingReceipt>,
    accepted_generation: u64,
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

#[derive(Debug, Clone)]
pub(crate) struct AcceptedProject {
    pub(crate) baseline: ProjectFile,
    pub(crate) binding: Option<PersistenceBinding>,
}

#[derive(Debug, Clone)]
pub(crate) struct ProjectLifecycleState {
    /// Runtime-only project incarnation. It never moves backwards, including
    /// across New/Close, so a completion from an older project cannot acquire
    /// authority in a newer one.
    epoch: u64,
    pub(crate) project_open: bool,
    accepted: Option<AcceptedProject>,
    registry: registry::DocumentRegistry,
    transaction: Option<LifecycleTransaction>,
    unreadable_native_binding: Option<persistence::UnreadableNativeBinding>,
    accepted_generation: u64,
    /// Monotonic authority for browser promises. Explicit cancellation bumps
    /// this value so a promise that JavaScript cannot abort is harmless when
    /// it eventually resolves.
    #[cfg(target_arch = "wasm32")]
    browser_operation_generation: u64,
    #[cfg(target_arch = "wasm32")]
    browser_restore_pending: bool,
    #[cfg(target_arch = "wasm32")]
    browser_promotion_pending: bool,
    #[cfg(target_arch = "wasm32")]
    browser_reconnect_binding: Option<PersistenceBinding>,
    #[cfg(target_arch = "wasm32")]
    browser_conflict: Option<BrowserConflict>,
}

impl Default for ProjectLifecycleState {
    fn default() -> Self {
        Self {
            epoch: 1,
            project_open: true,
            accepted: None,
            registry: registry::DocumentRegistry::default(),
            transaction: None,
            unreadable_native_binding: None,
            accepted_generation: 0,
            #[cfg(target_arch = "wasm32")]
            browser_operation_generation: 1,
            #[cfg(target_arch = "wasm32")]
            browser_restore_pending: false,
            #[cfg(target_arch = "wasm32")]
            browser_promotion_pending: false,
            #[cfg(target_arch = "wasm32")]
            browser_reconnect_binding: None,
            #[cfg(target_arch = "wasm32")]
            browser_conflict: None,
        }
    }
}

impl ProjectLifecycleState {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SaveScope {
    ActiveDocument,
    AllDocuments,
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
    accepted
        .baseline
        .workspace
        .schematic_buffers
        .get(&state.workspace.active_view.key())
        .cloned()
}

/// Monotonic identity of the accepted canonical baseline. Validation receipts
/// bind this value so a save opened against one baseline cannot publish after
/// a different save, import, or binding restoration has completed.
pub(crate) const fn accepted_generation(state: &AppState) -> u64 {
    state.project_lifecycle.accepted_generation
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

#[derive(Debug, thiserror::Error)]
pub(crate) enum ProjectLifecycleError {
    #[error("no project is open")]
    NoProject,
    #[error("safe mode opened this project read-only; project writes are blocked for this launch")]
    SafeModeReadOnly,
    #[error(
        "a local simulation is running; stop it before replacing project-owned plan, model, or result state"
    )]
    ActiveRun,
    #[error("the active document has no accepted project baseline to restore")]
    NoAcceptedBaseline,
    #[error(
        "the active document cannot be closed because it is the only presented design document"
    )]
    LastPresentedDocument,
    #[error(
        "the project changed while the file picker was open; the replacement was cancelled to preserve newer edits"
    )]
    ReplacementChanged,
    #[error("another project lifecycle operation is already in progress")]
    TransactionInProgress,
    #[cfg(not(target_arch = "wasm32"))]
    #[error(
        "the canonical project could not be accepted at startup ({0}); use Save as project copy to preserve it"
    )]
    UnreadableCanonical(String),
    #[cfg(not(target_arch = "wasm32"))]
    #[error(
        "a project copy cannot replace the active project's canonical file; choose a different destination"
    )]
    CopyDestinationIsCanonical,
    #[cfg(target_arch = "wasm32")]
    #[error(
        "browser canonical-binding restoration or promotion is still in progress; wait for it to finish before saving"
    )]
    BrowserBindingRestorePending,
    #[cfg(target_arch = "wasm32")]
    #[error(
        "the canonical browser project changed outside RSpice; reopen it or save an independent project copy"
    )]
    BrowserExternalChange,
    #[error(
        "the active document or accepted project changed while the revert review was open; nothing was reverted"
    )]
    RevertReviewStale,
    #[error("project state is invalid: {0}")]
    InvalidState(String),
    #[error(transparent)]
    Persistence(#[from] persistence::PersistenceError),
}

pub(crate) fn snapshot(state: &AppState) -> Result<ProjectFile, ProjectLifecycleError> {
    let mut workspace = state.workspace.clone();
    if matches!(
        workspace.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    ) {
        workspace
            .schematic_buffers
            .insert(workspace.active_key(), state.schematic.clone());
    }
    workspace.mark_all_clean();
    for schematic in workspace.schematic_buffers.values_mut() {
        strip_schematic_runtime_state(schematic);
    }

    let mut libraries = state.library_manager.clone();
    sanitize_library_view_runtime_state(&mut libraries);
    let simulation_results = ProjectSimulationResults::from_state(&state.simulation);
    let execution_context = ProjectExecutionContext::from_state(
        workspace.project.id(),
        &state.sim_setup,
        &state.model_library_manager,
    )
    .map_err(ProjectLifecycleError::InvalidState)?;
    let project = ProjectFile::new_with_execution_context(
        workspace,
        libraries,
        simulation_results,
        execution_context,
    );
    project
        .validate()
        .map_err(|error| ProjectLifecycleError::InvalidState(error.to_string()))?;
    project
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
    let mut project = snapshot(state)?;
    project.simulation_results = ProjectSimulationResults::default();
    project.workspace.netlist_source = None;
    project.workspace.netlist_source_path = None;
    project.workspace.netlist_document = None;
    project.workspace.netlist_descriptor = None;
    registry::content_digest(&project).map_err(ProjectLifecycleError::InvalidState)
}

pub(crate) fn has_unsaved_changes(state: &AppState) -> bool {
    if !state.project_lifecycle.project_open {
        return false;
    }
    let Some(accepted) = state.project_lifecycle.accepted.as_ref() else {
        return true;
    };
    match snapshot(state).and_then(|current| {
        registry::content_digest(&current)
            .map_err(ProjectLifecycleError::InvalidState)
            .map(|digest| (digest, current))
    }) {
        Ok((current, _)) => registry::content_digest(&accepted.baseline)
            .map(|baseline| current != baseline)
            .unwrap_or(true),
        Err(_) => true,
    }
}

pub(crate) fn operation_in_progress(state: &AppState) -> bool {
    state.project_lifecycle.transaction.is_some() || {
        #[cfg(target_arch = "wasm32")]
        {
            state.project_lifecycle.browser_restore_pending
                || state.project_lifecycle.browser_promotion_pending
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            false
        }
    }
}

pub(crate) fn active_document(state: &AppState) -> ProjectDocumentId {
    if state.workbench.workspace == crate::workbench::state::Workspace::Netlist
        && state.workspace.active_view_type() == ViewType::VerilogA
        && state
            .workspace
            .project_sources
            .bundle_for_owner(&crate::state::ProjectSourceOwner::cell_view(
                state.workspace.active_view.clone(),
            ))
            .is_some()
    {
        return ProjectDocumentId::CellView(state.workspace.active_view.clone());
    }
    registry::active_document(state.workbench.workspace, &state.workspace.active_view)
}

pub(crate) fn active_document_is_dirty(state: &AppState) -> bool {
    let Some(accepted) = state.project_lifecycle.accepted.as_ref() else {
        return state.project_lifecycle.project_open;
    };
    let Ok(current) = snapshot(state) else {
        return true;
    };
    let mut registry = registry::DocumentRegistry::default();
    if registry
        .rebuild(&current, Some(&accepted.baseline))
        .is_err()
    {
        return true;
    }
    registry.is_dirty(&active_document(state))
}

pub(crate) fn refresh_registry(state: &mut AppState) -> Result<(), ProjectLifecycleError> {
    if !state.project_lifecycle.project_open {
        state.project_lifecycle.registry = registry::DocumentRegistry::default();
        return Ok(());
    }
    let current = snapshot(state)?;
    let accepted = state
        .project_lifecycle
        .accepted
        .as_ref()
        .map(|accepted| accepted.baseline.clone());
    let mut registry = registry::DocumentRegistry::default();
    registry
        .rebuild(&current, accepted.as_ref())
        .map_err(ProjectLifecycleError::InvalidState)?;
    state.project_lifecycle.registry = registry;
    apply_registry_dirty_flags(state);
    Ok(())
}

fn apply_registry_dirty_flags(state: &mut AppState) {
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
        if let Some(buffer) = state.workspace.schematic_buffers.get_mut(&reference.key()) {
            buffer.is_dirty = *dirty;
        }
        if let Some(open) = state
            .workspace
            .open_views
            .iter_mut()
            .find(|open| open.reference == *reference)
        {
            open.dirty = *dirty;
        }
        if let Some(view) = state
            .library_manager
            .get_library_mut(&reference.library)
            .and_then(|library| library.get_cell_mut(&reference.cell))
            .and_then(|cell| cell.get_view_mut(&reference.view))
        {
            view.modified = *dirty;
        }
    }
    if let Some((_, dirty)) = cell_dirty
        .iter()
        .find(|(reference, _)| *reference == state.workspace.active_view)
    {
        state.schematic.is_dirty = *dirty;
    }
    if let Some(accepted) = state.project_lifecycle.accepted.as_ref() {
        let baseline = &accepted.baseline.workspace;
        state.workspace.netlist_source_dirty = state.workspace.netlist_source
            != baseline.netlist_source
            || state.workspace.netlist_source_path != baseline.netlist_source_path
            || state.workspace.netlist_document != baseline.netlist_document
            || state.workspace.netlist_descriptor != baseline.netlist_descriptor;
        state.workspace.project_sources_dirty =
            state.workspace.project_sources != baseline.project_sources;
    }
}

pub(crate) fn initialize_from_session(state: &mut AppState) {
    state.project_lifecycle.project_open = true;
    #[cfg(not(target_arch = "wasm32"))]
    {
        let restored_path = state.workspace.project.path.clone();
        let receipt = state.native_project_binding_receipt.clone();
        match (restored_path, receipt) {
            (Some(path), Some(receipt)) => {
                let session_project_id = state.workspace.project.id().to_string();
                match persistence::restore_native_binding(&path, &session_project_id, &receipt) {
                    Ok((baseline, binding)) => {
                        state.project_lifecycle.accepted = Some(AcceptedProject {
                            baseline,
                            binding: Some(binding),
                        });
                        state.project_lifecycle.accepted_generation = 1;
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
            let project_id = state.workspace.project.id().to_string();
            if receipt.project_id != project_id {
                state.browser_project_binding_receipt = None;
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                    "Ignored a stale browser binding receipt for a different project identity",
                ));
            } else {
                state.project_lifecycle.browser_restore_pending = true;
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
    state.project_lifecycle.browser_restore_pending = false;
    match completion.result {
        persistence::BrowserRestoreResult::Missing => {
            state.browser_project_binding_receipt = None;
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                "The browser canonical-binding receipt has no matching restoration record; choose Save to establish a new canonical binding",
            ));
        }
        persistence::BrowserRestoreResult::Restored { baseline, binding } => {
            if baseline.workspace.project.id() != state.workspace.project.id() {
                release_browser_binding_handle(&binding);
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                    "Ignored a stale browser project binding for a different project identity",
                ));
                return;
            }
            state.project_lifecycle.accepted = Some(AcceptedProject {
                baseline: *baseline,
                binding: Some(binding),
            });
            state.native_project_binding_receipt = None;
            advance_accepted_generation(&mut state.project_lifecycle);
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
    baseline: ProjectFile,
    binding: Option<PersistenceBinding>,
) {
    state.clear_project_design_history();
    state.dialogs.check_and_save.close();
    #[cfg(not(target_arch = "wasm32"))]
    let native_receipt = binding
        .as_ref()
        .map(|binding| binding.native_receipt(&baseline.workspace.project.id().to_string()));
    #[cfg(target_arch = "wasm32")]
    release_replaced_browser_bindings(&state.project_lifecycle, binding.as_ref());
    state.project_lifecycle.project_open = true;
    state.project_lifecycle.accepted = Some(AcceptedProject { baseline, binding });
    advance_accepted_generation(&mut state.project_lifecycle);
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
        state.project_lifecycle.browser_restore_pending = false;
        state.project_lifecycle.browser_promotion_pending = false;
    }
    state.project_lifecycle.transaction = None;
    let _ = refresh_registry(state);
}

pub(crate) fn reset_for_new_project(state: &mut AppState) {
    state.clear_project_design_history();
    state.dialogs.check_and_save.close();
    state.native_project_binding_receipt = None;
    state.browser_project_binding_receipt = None;
    #[cfg(target_arch = "wasm32")]
    {
        clear_browser_handles();
    }
    let next_epoch = state.project_lifecycle.epoch.wrapping_add(1).max(1);
    state.project_lifecycle = ProjectLifecycleState {
        epoch: next_epoch,
        ..ProjectLifecycleState::default()
    };
    let _ = refresh_registry(state);
}

pub(crate) fn mark_project_closed(state: &mut AppState) {
    state.clear_project_design_history();
    state.dialogs.check_and_save.close();
    state.native_project_binding_receipt = None;
    state.browser_project_binding_receipt = None;
    #[cfg(target_arch = "wasm32")]
    {
        clear_browser_handles();
    }
    let next_epoch = state.project_lifecycle.epoch.wrapping_add(1).max(1);
    state.project_lifecycle = ProjectLifecycleState {
        epoch: next_epoch,
        project_open: false,
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

#[cfg(any(test, target_arch = "wasm32"))]
fn operation_context_matches(
    context: &BrowserOperationContext,
    epoch: u64,
    operation_generation: u64,
    project_id: &str,
    receipt: Option<&BrowserBindingReceipt>,
    accepted_generation: u64,
) -> bool {
    context.epoch == epoch
        && context.operation_generation == operation_generation
        && context.project_id == project_id
        && context.binding_receipt.as_ref() == receipt
        && context.accepted_generation == accepted_generation
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_operation_context(state: &AppState) -> BrowserOperationContext {
    BrowserOperationContext {
        epoch: state.project_lifecycle.epoch,
        operation_generation: state.project_lifecycle.browser_operation_generation,
        project_id: state.workspace.project.id().to_string(),
        binding_receipt: state.browser_project_binding_receipt.clone(),
        accepted_generation: state.project_lifecycle.accepted_generation,
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_operation_context_is_current(
    state: &AppState,
    context: &BrowserOperationContext,
) -> bool {
    operation_context_matches(
        context,
        state.project_lifecycle.epoch,
        state.project_lifecycle.browser_operation_generation,
        &state.workspace.project.id().to_string(),
        state.browser_project_binding_receipt.as_ref(),
        state.project_lifecycle.accepted_generation,
    )
}

#[cfg(target_arch = "wasm32")]
fn advance_browser_operation_generation(lifecycle: &mut ProjectLifecycleState) {
    lifecycle.browser_operation_generation = lifecycle
        .browser_operation_generation
        .wrapping_add(1)
        .max(1);
}

/// Relinquish app-side authority for a browser promise that the platform may
/// not be able to abort. The operation generation makes every eventual late
/// completion stale, while accepted project/file authority remains intact.
#[cfg(target_arch = "wasm32")]
pub(crate) fn cancel_pending_browser_operation(state: &mut AppState) -> bool {
    let restore_was_pending = state.project_lifecycle.browser_restore_pending;
    let pending = state.project_lifecycle.transaction.is_some()
        || restore_was_pending
        || state.project_lifecycle.browser_promotion_pending;
    if !pending {
        return false;
    }

    state.project_lifecycle.transaction = None;
    state.project_lifecycle.browser_restore_pending = false;
    state.project_lifecycle.browser_promotion_pending = false;
    if restore_was_pending {
        // A canceled restore did not establish authority. Keeping its receipt
        // would repeatedly imply a restart binding which this session chose
        // not to verify.
        state.browser_project_binding_receipt = None;
    }
    advance_browser_operation_generation(&mut state.project_lifecycle);
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
) -> Result<(ProjectFile, PersistenceBinding), ProjectLifecycleError> {
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
    let kind = match scope {
        SaveScope::ActiveDocument => TransactionKind::SaveActive,
        SaveScope::AllDocuments => TransactionKind::SaveAll,
    };
    begin_save_transaction(state, kind)?;

    let result = (|| {
        let working = snapshot(state)?;
        let mut candidate = match scope {
            SaveScope::AllDocuments => working.clone(),
            SaveScope::ActiveDocument => {
                let mut baseline = state
                    .project_lifecycle
                    .accepted
                    .as_ref()
                    .ok_or(ProjectLifecycleError::NoAcceptedBaseline)?
                    .baseline
                    .clone();
                overlay_document(&mut baseline, &working, &active_document(state))?;
                baseline
            }
        };
        candidate.workspace.project.set_path(path.clone());
        // Build every fallible post-save document digest before publishing.
        // Once the durable file replacement succeeds, adoption below is an
        // in-memory, infallible state transition.
        let post_save_registry = prepare_post_save_registry(state, &candidate, scope)?;
        let (bytes, _) = persistence::serialized_project(&candidate)?;
        let digest = persistence::publish_canonical_native(&path, expected, &bytes)?;
        let binding = PersistenceBinding::Native {
            canonical_path: path.clone(),
            accepted_digest: digest,
        };
        adopt_successful_save(state, candidate, binding, scope, post_save_registry);
        Ok(())
    })();
    state.project_lifecycle.transaction = None;
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
    begin_save_transaction(state, TransactionKind::SaveProjectCopy)?;
    let result = (|| {
        let mut copy = snapshot(state)?;
        copy.workspace.project = copy.workspace.project.fork_copy_at(path.clone());
        let (bytes, _) = persistence::serialized_project(&copy)?;
        // The picker authorizes this destination, while the captured exact
        // state still prevents a late create/edit from being overwritten.
        // The source project's accepted baseline and binding never change.
        let _ = persistence::publish_canonical_native(&path, expected, &bytes)?;
        Ok(())
    })();
    state.project_lifecycle.transaction = None;
    result
}

#[cfg(target_arch = "wasm32")]
pub(crate) struct BrowserPreparedSave {
    pub(crate) transaction: TransactionId,
    pub(crate) context: BrowserOperationContext,
    pub(crate) candidate: ProjectFile,
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
    if state.project_lifecycle.browser_restore_pending
        || state.project_lifecycle.browser_promotion_pending
    {
        return Err(ProjectLifecycleError::BrowserBindingRestorePending);
    }
    if !project_copy && let Some(conflict) = state.project_lifecycle.browser_conflict.as_ref() {
        let _ = conflict.observed_digest;
        return Err(ProjectLifecycleError::BrowserExternalChange);
    }
    let scope = effective_save_scope(state, requested_scope);
    let kind = if project_copy {
        TransactionKind::SaveProjectCopy
    } else {
        match scope {
            SaveScope::ActiveDocument => TransactionKind::SaveActive,
            SaveScope::AllDocuments => TransactionKind::SaveAll,
        }
    };
    let transaction = begin_save_transaction(state, kind)?;
    let context = browser_operation_context(state);
    let saved_document = active_document(state);
    let result = (|| {
        let working = snapshot(state)?;
        let mut candidate = if project_copy || scope == SaveScope::AllDocuments {
            working
        } else {
            let mut baseline = state
                .project_lifecycle
                .accepted
                .as_ref()
                .ok_or(ProjectLifecycleError::NoAcceptedBaseline)?
                .baseline
                .clone();
            overlay_document(&mut baseline, &working, &saved_document)?;
            baseline
        };
        if project_copy {
            candidate.workspace.project = candidate
                .workspace
                .project
                .fork_copy_at(PathBuf::from(&suggested_name));
        } else {
            candidate.workspace.project.path = None;
        }
        let (bytes, staged_digest) = persistence::serialized_project(&candidate)?;
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
        let project_id = candidate.workspace.project.id().to_string();
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
        state.project_lifecycle.transaction = None;
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
        .transaction
        .as_ref()
        .is_some_and(|transaction| transaction.id == prepared.transaction);
    if !current || !browser_operation_context_is_current(state, &prepared.context) {
        persistence::release_browser_handle(handle_id);
        return Err(ProjectLifecycleError::TransactionInProgress);
    }
    if prepared.project_copy {
        persistence::release_browser_handle(handle_id);
        state.project_lifecycle.transaction = None;
        return Ok(());
    }
    if binding_id != prepared.target.binding_id
        || backend != prepared.target.backend
        || project_id != prepared.target.project_id
        || generation != prepared.target.accepted_generation
        || digest != prepared.staged_digest
    {
        persistence::release_browser_handle(handle_id);
        state.project_lifecycle.transaction = None;
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
    let result = finish_successful_save(state, prepared.candidate, binding, prepared.scope);
    state.project_lifecycle.transaction = None;
    result
}

#[cfg(target_arch = "wasm32")]
fn finish_successful_save(
    state: &mut AppState,
    candidate: ProjectFile,
    binding: PersistenceBinding,
    scope: SaveScope,
) -> Result<(), ProjectLifecycleError> {
    let post_save_registry = prepare_post_save_registry(state, &candidate, scope)?;
    adopt_successful_save(state, candidate, binding, scope, post_save_registry);
    Ok(())
}

fn prepare_post_save_registry(
    state: &AppState,
    candidate: &ProjectFile,
    scope: SaveScope,
) -> Result<registry::DocumentRegistry, ProjectLifecycleError> {
    #[cfg(not(target_arch = "wasm32"))]
    let mut current = snapshot(state)?;
    #[cfg(target_arch = "wasm32")]
    let current = snapshot(state)?;
    #[cfg(not(target_arch = "wasm32"))]
    {
        if scope == SaveScope::AllDocuments
            || active_document(state) == ProjectDocumentId::ProjectConfiguration
        {
            current.workspace.project = candidate.workspace.project.clone();
        } else {
            current.workspace.project.path = candidate.workspace.project.path.clone();
        }
    }
    #[cfg(target_arch = "wasm32")]
    let _ = scope;
    let mut post_save_registry = registry::DocumentRegistry::default();
    post_save_registry
        .rebuild(&current, Some(candidate))
        .map_err(ProjectLifecycleError::InvalidState)?;
    Ok(post_save_registry)
}

fn adopt_successful_save(
    state: &mut AppState,
    candidate: ProjectFile,
    binding: PersistenceBinding,
    scope: SaveScope,
    post_save_registry: registry::DocumentRegistry,
) {
    #[cfg(not(target_arch = "wasm32"))]
    let native_receipt = binding.native_receipt(&candidate.workspace.project.id().to_string());
    #[cfg(target_arch = "wasm32")]
    let browser_receipt = binding.durable_browser_receipt();
    #[cfg(target_arch = "wasm32")]
    release_replaced_browser_bindings(&state.project_lifecycle, Some(&binding));
    #[cfg(not(target_arch = "wasm32"))]
    {
        if scope == SaveScope::AllDocuments
            || active_document(state) == ProjectDocumentId::ProjectConfiguration
        {
            state.workspace.project = candidate.workspace.project.clone();
        } else {
            // Saving one non-configuration document intentionally publishes a
            // candidate built on the accepted project descriptor. Preserve a
            // concurrent/unrelated live descriptor draft and update only the
            // canonical pathname established by this successful save.
            state.workspace.project.path = candidate.workspace.project.path.clone();
        }
    }
    state.project_lifecycle.accepted = Some(AcceptedProject {
        baseline: candidate,
        binding: Some(binding),
    });
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
    advance_accepted_generation(&mut state.project_lifecycle);
    #[cfg(not(target_arch = "wasm32"))]
    match scope {
        SaveScope::AllDocuments => {
            state.workspace.mark_all_clean();
            state.schematic.is_dirty = false;
            mark_all_library_views_clean(&mut state.library_manager);
        }
        SaveScope::ActiveDocument => mark_active_document_clean(state),
    }
    #[cfg(target_arch = "wasm32")]
    let _ = scope;
    // This registry was fully built before native publication and before any
    // browser adoption mutation. Installing it cannot fail and preserves
    // edits made while an asynchronous browser write was pending.
    state.project_lifecycle.registry = post_save_registry;
    apply_registry_dirty_flags(state);
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
    state.project_lifecycle.browser_promotion_pending = false;
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
    state.project_lifecycle.browser_promotion_pending = true;
    browser_operation_context(state)
}

#[cfg(not(target_arch = "wasm32"))]
fn mark_active_document_clean(state: &mut AppState) {
    match active_document(state) {
        ProjectDocumentId::CellView(reference) => {
            if let Some(buffer) = state.workspace.schematic_buffers.get_mut(&reference.key()) {
                buffer.is_dirty = false;
            }
            if reference == state.workspace.active_view {
                state.schematic.is_dirty = false;
            }
            if let Some(open) = state
                .workspace
                .open_views
                .iter_mut()
                .find(|open| open.reference == reference)
            {
                open.dirty = false;
            }
            if let Some(view) = state
                .library_manager
                .get_library_mut(&reference.library)
                .and_then(|library| library.get_cell_mut(&reference.cell))
                .and_then(|cell| cell.get_view_mut(&reference.view))
            {
                view.modified = false;
            }
        }
        ProjectDocumentId::NetlistSource => {
            state.workspace.netlist_source_dirty = false;
            state.workspace.project_sources_dirty = false;
        }
        _ => {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RevertReviewToken {
    document: ProjectDocumentId,
    accepted_generation: u64,
}

impl RevertReviewToken {
    pub(crate) fn document_label(&self) -> String {
        match &self.document {
            ProjectDocumentId::ProjectConfiguration => "Project configuration".to_owned(),
            ProjectDocumentId::CellView(reference) => reference.key(),
            ProjectDocumentId::SimulationPlan => "Simulation plan".to_owned(),
            ProjectDocumentId::ResultHistory => "Result history".to_owned(),
            ProjectDocumentId::VerificationSpecifications => {
                "Verification specifications".to_owned()
            }
            ProjectDocumentId::ModelCatalog => "Model catalog".to_owned(),
            ProjectDocumentId::NetlistSource => "Netlist source".to_owned(),
        }
    }
}

pub(crate) fn prepare_revert_active_document(
    state: &AppState,
) -> Result<RevertReviewToken, ProjectLifecycleError> {
    require_open_project(state)?;
    let id = active_document(state);
    if state.simulation.is_running
        && matches!(
            id,
            ProjectDocumentId::SimulationPlan
                | ProjectDocumentId::ResultHistory
                | ProjectDocumentId::ModelCatalog
        )
    {
        return Err(ProjectLifecycleError::ActiveRun);
    }
    state
        .project_lifecycle
        .accepted
        .as_ref()
        .ok_or(ProjectLifecycleError::NoAcceptedBaseline)?;
    Ok(RevertReviewToken {
        document: id,
        accepted_generation: state.project_lifecycle.accepted_generation,
    })
}

pub(crate) fn confirm_revert_active_document(
    state: &mut AppState,
    token: &RevertReviewToken,
) -> Result<(), ProjectLifecycleError> {
    require_open_project(state)?;
    if active_document(state) != token.document
        || state.project_lifecycle.accepted_generation != token.accepted_generation
    {
        return Err(ProjectLifecycleError::RevertReviewStale);
    }
    let current = prepare_revert_active_document(state)?;
    if current != *token {
        return Err(ProjectLifecycleError::RevertReviewStale);
    }
    revert_document(state, token.document.clone())
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
        .baseline
        .clone();
    let baseline_project_id = baseline.workspace.project.id();

    match id {
        ProjectDocumentId::ProjectConfiguration => {
            state.workspace.project = baseline.workspace.project;
            state.workspace.configuration_sets = baseline.workspace.configuration_sets;
            state.workspace.design_management = baseline.workspace.design_management;
            restore_project_structure_preserving_documents(state, baseline.libraries);
        }
        ProjectDocumentId::CellView(reference) => revert_cell_view(state, &baseline, &reference)?,
        ProjectDocumentId::SimulationPlan => {
            let context = baseline.execution_context.ok_or_else(|| {
                ProjectLifecycleError::InvalidState(
                    "accepted project has no simulation plan".to_owned(),
                )
            })?;
            state.sim_setup = context.simulation_plan;
            state.sim_setup.prepare_after_restore();
            state.workspace.simulation_plan_payloads = baseline.workspace.simulation_plan_payloads;
            if let Some(plan_id) = state
                .sim_setup
                .analysis_plan
                .as_ref()
                .map(crate::simulation::plan::SimulationPlan::id)
            {
                state.workspace.sync_legacy_specs_projection(plan_id);
            }
        }
        ProjectDocumentId::ModelCatalog => {
            let context = baseline.execution_context.ok_or_else(|| {
                ProjectLifecycleError::InvalidState(
                    "accepted project has no model catalog".to_owned(),
                )
            })?;
            let (_, manager, warnings) = context
                .into_state(baseline_project_id)
                .map_err(ProjectLifecycleError::InvalidState)?;
            state.model_library_manager = manager;
            for warning in warnings {
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(warning));
            }
        }
        ProjectDocumentId::ResultHistory => {
            let mut simulation = crate::state::SimulationState::default();
            baseline
                .simulation_results
                .apply_to_state(&mut simulation)
                .map_err(ProjectLifecycleError::InvalidState)?;
            state.simulation = simulation;
            state.workspace.report_documents = baseline.workspace.report_documents;
            state.workspace.report_documents_dirty = false;
            state.clear_specialized_viewer_data();
        }
        ProjectDocumentId::VerificationSpecifications => {
            state.workspace.specs = baseline.workspace.specs;
        }
        ProjectDocumentId::NetlistSource => {
            state.workspace.netlist_source = baseline.workspace.netlist_source;
            state.workspace.netlist_source_path = baseline.workspace.netlist_source_path;
            state.workspace.netlist_document = baseline.workspace.netlist_document;
            state.workspace.netlist_descriptor = baseline.workspace.netlist_descriptor;
            state
                .workspace
                .project_sources
                .synchronize_code_workspace_bundles_from(&baseline.workspace.project_sources)
                .map_err(|error| ProjectLifecycleError::InvalidState(error.to_string()))?;
            state.workspace.netlist_source_dirty = false;
            state.workspace.project_sources_dirty = false;
            state.ui.netlist = Default::default();
            state.simulation.netlist_content =
                state.workspace.netlist_source.clone().unwrap_or_default();
            state.simulation.trigger_simulation = false;
            state.ui.netlist.rerun_queued = false;
            state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
        }
    }
    Ok(())
}

pub(crate) fn dirty_document_count(state: &AppState) -> usize {
    let Some(accepted) = state.project_lifecycle.accepted.as_ref() else {
        return if state.project_lifecycle.project_open {
            1
        } else {
            0
        };
    };
    let Ok(current) = snapshot(state) else {
        return 1;
    };
    let mut registry = registry::DocumentRegistry::default();
    if registry
        .rebuild(&current, Some(&accepted.baseline))
        .is_err()
    {
        return 1;
    }
    registry
        .records()
        .iter()
        .filter(|record| record.dirty)
        .count()
}

pub(crate) fn can_close_active_document(state: &AppState) -> bool {
    state.project_lifecycle.project_open
        && state.workbench.workspace == crate::workbench::state::Workspace::Design
        && state.workspace.open_views.len() > 1
}

pub(crate) fn close_active_document(state: &mut AppState) -> Result<(), ProjectLifecycleError> {
    require_open_project(state)?;
    if !can_close_active_document(state) {
        return Err(ProjectLifecycleError::LastPresentedDocument);
    }
    // Capture the live editor before removing only its presentation record.
    state.sync_active_schematic_to_workspace();
    let closing = state.workspace.active_view.clone();
    state.workspace.close_view(&closing);
    state.restore_active_schematic_from_workspace();
    refresh_registry(state)
}

pub(crate) fn begin_project_replacement(
    state: &mut AppState,
) -> Result<TransactionId, ProjectLifecycleError> {
    if state.simulation.is_running {
        return Err(ProjectLifecycleError::ActiveRun);
    }
    if state.project_lifecycle.transaction.is_some() {
        return Err(ProjectLifecycleError::TransactionInProgress);
    }
    #[cfg(target_arch = "wasm32")]
    if state.project_lifecycle.browser_restore_pending
        || state.project_lifecycle.browser_promotion_pending
    {
        return Err(ProjectLifecycleError::TransactionInProgress);
    }
    let content =
        registry::content_digest(&snapshot(state)?).map_err(ProjectLifecycleError::InvalidState)?;
    let transaction = LifecycleTransaction::replacement(content);
    let id = transaction.id;
    state.project_lifecycle.transaction = Some(transaction);
    Ok(id)
}

pub(crate) fn cancel_transaction(state: &mut AppState) {
    state.project_lifecycle.transaction = None;
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn cancel_transaction_if(state: &mut AppState, id: TransactionId) -> bool {
    if state
        .project_lifecycle
        .transaction
        .as_ref()
        .is_some_and(|transaction| transaction.id == id)
    {
        state.project_lifecycle.transaction = None;
        true
    } else {
        false
    }
}

pub(crate) fn validate_project_replacement(
    state: &AppState,
    id: TransactionId,
) -> Result<(), ProjectLifecycleError> {
    if state.simulation.is_running {
        return Err(ProjectLifecycleError::ActiveRun);
    }
    let transaction = state
        .project_lifecycle
        .transaction
        .as_ref()
        .filter(|transaction| {
            transaction.id == id && transaction.kind == TransactionKind::OpenProject
        })
        .ok_or(ProjectLifecycleError::ReplacementChanged)?;
    let expected = transaction
        .replacement_guard
        .ok_or(ProjectLifecycleError::ReplacementChanged)?;
    let actual =
        registry::content_digest(&snapshot(state)?).map_err(ProjectLifecycleError::InvalidState)?;
    if actual != expected {
        return Err(ProjectLifecycleError::ReplacementChanged);
    }
    Ok(())
}

fn begin_save_transaction(
    state: &mut AppState,
    kind: TransactionKind,
) -> Result<TransactionId, ProjectLifecycleError> {
    if state.project_lifecycle.transaction.is_some() {
        return Err(ProjectLifecycleError::TransactionInProgress);
    }
    let transaction = LifecycleTransaction::save(kind);
    let id = transaction.id;
    state.project_lifecycle.transaction = Some(transaction);
    Ok(id)
}

fn require_open_project(state: &AppState) -> Result<(), ProjectLifecycleError> {
    state
        .project_lifecycle
        .project_open
        .then_some(())
        .ok_or(ProjectLifecycleError::NoProject)
}

fn require_project_writable(state: &AppState) -> Result<(), ProjectLifecycleError> {
    (!state.workbench.safe_mode.project_read_only())
        .then_some(())
        .ok_or(ProjectLifecycleError::SafeModeReadOnly)
}

fn advance_accepted_generation(lifecycle: &mut ProjectLifecycleState) {
    lifecycle.accepted_generation = lifecycle.accepted_generation.wrapping_add(1).max(1);
}

fn overlay_document(
    target: &mut ProjectFile,
    working: &ProjectFile,
    id: &ProjectDocumentId,
) -> Result<(), ProjectLifecycleError> {
    match id {
        ProjectDocumentId::ProjectConfiguration => {
            target.workspace.project = working.workspace.project.clone();
            target.workspace.configuration_sets = working.workspace.configuration_sets.clone();
            target.workspace.design_management = working.workspace.design_management.clone();
            target.libraries = merge_project_structure_with_document_content(
                &working.libraries,
                &target.libraries,
            );
        }
        ProjectDocumentId::CellView(reference) => overlay_cell_view(target, working, reference)?,
        ProjectDocumentId::SimulationPlan => {
            ensure_execution_context(target, working)?.simulation_plan = working
                .execution_context
                .as_ref()
                .ok_or_else(|| {
                    ProjectLifecycleError::InvalidState(
                        "working project has no simulation plan".to_owned(),
                    )
                })?
                .simulation_plan
                .clone();
            target.workspace.simulation_plan_payloads =
                working.workspace.simulation_plan_payloads.clone();
            if let Some(plan_id) = target
                .execution_context
                .as_ref()
                .and_then(|context| context.simulation_plan.analysis_plan.as_ref())
                .map(crate::simulation::plan::SimulationPlan::id)
            {
                target.workspace.sync_legacy_specs_projection(plan_id);
            }
        }
        ProjectDocumentId::ModelCatalog => {
            ensure_execution_context(target, working)?.model_libraries = working
                .execution_context
                .as_ref()
                .ok_or_else(|| {
                    ProjectLifecycleError::InvalidState(
                        "working project has no model catalog".to_owned(),
                    )
                })?
                .model_libraries
                .clone();
        }
        ProjectDocumentId::ResultHistory => {
            target.simulation_results = working.simulation_results.clone();
            target.workspace.report_documents = working.workspace.report_documents.clone();
        }
        ProjectDocumentId::VerificationSpecifications => {
            target.workspace.specs = working.workspace.specs.clone();
        }
        ProjectDocumentId::NetlistSource => {
            target.workspace.netlist_source = working.workspace.netlist_source.clone();
            target.workspace.netlist_source_path = working.workspace.netlist_source_path.clone();
            target.workspace.netlist_document = working.workspace.netlist_document.clone();
            target.workspace.netlist_descriptor = working.workspace.netlist_descriptor.clone();
            target
                .workspace
                .project_sources
                .synchronize_code_workspace_bundles_from(&working.workspace.project_sources)
                .map_err(|error| ProjectLifecycleError::InvalidState(error.to_string()))?;
        }
    }
    target
        .validate()
        .map_err(|error| ProjectLifecycleError::InvalidState(error.to_string()))
}

fn ensure_execution_context<'a>(
    target: &'a mut ProjectFile,
    working: &ProjectFile,
) -> Result<&'a mut ProjectExecutionContext, ProjectLifecycleError> {
    if target.execution_context.is_none() {
        target.execution_context = working.execution_context.clone();
    }
    target.execution_context.as_mut().ok_or_else(|| {
        ProjectLifecycleError::InvalidState("project has no execution context".to_owned())
    })
}

fn overlay_cell_view(
    target: &mut ProjectFile,
    working: &ProjectFile,
    reference: &CellViewRef,
) -> Result<(), ProjectLifecycleError> {
    let working_library = working
        .libraries
        .get_library(&reference.library)
        .ok_or_else(|| {
            ProjectLifecycleError::InvalidState(format!("missing library '{}'", reference.library))
        })?;
    let working_cell = working_library.get_cell(&reference.cell).ok_or_else(|| {
        ProjectLifecycleError::InvalidState(format!("missing cell '{}'", reference.cell))
    })?;
    let working_view = working_cell
        .get_view(&reference.view)
        .ok_or_else(|| {
            ProjectLifecycleError::InvalidState(format!("missing view '{}'", reference.view))
        })?
        .clone();

    if target.libraries.get_library(&reference.library).is_none() {
        let mut library = working_library.clone();
        library.cells.clear();
        target.libraries.add_library(library);
    }
    let target_library = target
        .libraries
        .get_library_mut(&reference.library)
        .expect("library inserted above");
    if target_library.get_cell(&reference.cell).is_none() {
        let mut cell = working_cell.clone();
        cell.views.clear();
        target_library.add_cell(cell);
    }
    target_library
        .get_cell_mut(&reference.cell)
        .expect("cell inserted above")
        .add_view(working_view);

    let key = reference.key();
    match working.workspace.schematic_buffers.get(&key) {
        Some(buffer) => {
            target
                .workspace
                .schematic_buffers
                .insert(key, buffer.clone());
        }
        None => {
            target.workspace.schematic_buffers.remove(&key);
        }
    }
    overlay_generated_symbol(target, working, reference);
    target
        .workspace
        .project_sources
        .synchronize_cell_view_bundle_from(reference, &working.workspace.project_sources)
        .map_err(|error| ProjectLifecycleError::InvalidState(error.to_string()))?;
    Ok(())
}

fn overlay_generated_symbol(
    target: &mut ProjectFile,
    working: &ProjectFile,
    reference: &CellViewRef,
) {
    if !reference.view.eq_ignore_ascii_case("schematic") {
        return;
    }
    let working_symbol = working
        .libraries
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view("symbol"))
        .filter(|view| view.metadata.contains_key("generated"))
        .cloned();
    let Some(cell) = target
        .libraries
        .get_library_mut(&reference.library)
        .and_then(|library| library.get_cell_mut(&reference.cell))
    else {
        return;
    };
    match working_symbol {
        Some(symbol) => cell.add_view(symbol),
        None if cell
            .get_view("symbol")
            .is_some_and(|view| view.metadata.contains_key("generated")) =>
        {
            cell.remove_view("symbol");
        }
        None => {}
    }
}

fn revert_cell_view(
    state: &mut AppState,
    baseline: &ProjectFile,
    reference: &CellViewRef,
) -> Result<(), ProjectLifecycleError> {
    let baseline_view = baseline
        .libraries
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .cloned();
    let Some(baseline_view) = baseline_view else {
        let removed = state
            .library_manager
            .get_library_mut(&reference.library)
            .and_then(|library| library.get_cell_mut(&reference.cell))
            .is_some_and(|cell| cell.remove_view(&reference.view));
        if !removed {
            return Err(ProjectLifecycleError::NoAcceptedBaseline);
        }
        let source_changed = state
            .workspace
            .project_sources
            .synchronize_cell_view_bundle_from(reference, &baseline.workspace.project_sources)
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
    let target_cell = state
        .library_manager
        .get_library_mut(&reference.library)
        .and_then(|library| library.get_cell_mut(&reference.cell))
        .ok_or_else(|| {
            ProjectLifecycleError::InvalidState("active cell no longer exists".to_owned())
        })?;
    target_cell.add_view(baseline_view);
    let key = reference.key();
    match baseline.workspace.schematic_buffers.get(&key) {
        Some(buffer) => {
            state
                .workspace
                .schematic_buffers
                .insert(key, buffer.clone());
        }
        None => {
            state.workspace.schematic_buffers.remove(&key);
        }
    }
    if reference == &state.workspace.active_view {
        state.restore_active_schematic_from_workspace();
    }
    let source_changed = state
        .workspace
        .project_sources
        .synchronize_cell_view_bundle_from(reference, &baseline.workspace.project_sources)
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
    state.library_manager =
        merge_project_structure_with_document_content(&baseline, &state.library_manager);
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

fn merge_project_structure_with_document_content(
    structure: &crate::state::LibraryManager,
    content: &crate::state::LibraryManager,
) -> crate::state::LibraryManager {
    let mut merged = structure.clone();
    let cells = merged
        .libraries_by_key()
        .flat_map(|(library_key, library)| {
            library
                .cells
                .keys()
                .map(move |cell_key| (library_key.to_owned(), cell_key.to_owned()))
        })
        .collect::<Vec<_>>();
    for (library, cell) in cells {
        if let Some(cell) = merged
            .get_library_mut(&library)
            .and_then(|library| library.get_cell_mut(&cell))
        {
            cell.views.clear();
        }
    }
    let references = content
        .libraries_by_key()
        .flat_map(|(library_key, library)| {
            library.cells.iter().flat_map(move |(cell_key, cell)| {
                cell.views
                    .keys()
                    .map(move |view_key| CellViewRef::new(library_key, cell_key, view_key))
            })
        })
        .collect::<Vec<_>>();
    for reference in references {
        let Some(view) = content
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .cloned()
        else {
            continue;
        };
        if let Some(cell) = merged
            .get_library_mut(&reference.library)
            .and_then(|library| library.get_cell_mut(&reference.cell))
        {
            cell.add_view(view);
        }
    }
    merged
}

fn mark_all_library_views_clean(libraries: &mut crate::state::LibraryManager) {
    let references = libraries
        .libraries_by_key()
        .flat_map(|(library_key, library)| {
            library.cells.iter().flat_map(move |(cell_key, cell)| {
                cell.views
                    .keys()
                    .map(move |view_key| CellViewRef::new(library_key, cell_key, view_key))
            })
        })
        .collect::<Vec<_>>();
    for reference in references {
        if let Some(view) = libraries
            .get_library_mut(&reference.library)
            .and_then(|library| library.get_cell_mut(&reference.cell))
            .and_then(|cell| cell.get_view_mut(&reference.view))
        {
            view.modified = false;
        }
    }
}

/// Remove presentation-only state from the serialized clone. This must never
/// be used on the live library manager: saving engineering content does not
/// close tabs or alter the user's presentation state.
fn sanitize_library_view_runtime_state(libraries: &mut crate::state::LibraryManager) {
    mark_all_library_views_clean(libraries);
    let references = libraries
        .libraries_by_key()
        .flat_map(|(library_key, library)| {
            library.cells.iter().flat_map(move |(cell_key, cell)| {
                cell.views
                    .keys()
                    .map(move |view_key| CellViewRef::new(library_key, cell_key, view_key))
            })
        })
        .collect::<Vec<_>>();
    for reference in references {
        if let Some(view) = libraries
            .get_library_mut(&reference.library)
            .and_then(|library| library.get_cell_mut(&reference.cell))
            .and_then(|cell| cell.get_view_mut(&reference.view))
        {
            view.is_open = false;
            view.file_path = None;
            view.modified_time = None;
        }
    }
}

fn strip_schematic_runtime_state(schematic: &mut crate::state::SchematicState) {
    schematic.selection = Default::default();
    schematic.wire_drawing = Default::default();
    schematic.clipboard = Default::default();
    schematic.preview_rotation = Default::default();
    schematic.preview_mirror_h = false;
    schematic.connections.clear();
    schematic.is_dirty = false;
}


#[cfg(test)]
mod tests;
