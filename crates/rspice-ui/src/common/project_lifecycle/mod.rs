//! Transactional project/document lifecycle.
//!
//! The accepted project baseline is intentionally separate from the mutable
//! workbench state. `Save` overlays one stable document onto that baseline;
//! `Save all` replaces it with the complete working set. This prevents saving
//! one tab from accidentally committing unrelated drafts.

mod persistence;
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

use crate::common::app::AppState;
use crate::io::{ProjectExecutionContext, ProjectFile, ProjectSimulationResults};
#[cfg(target_arch = "wasm32")]
use crate::product::ContentDigest;
use crate::state::{CellViewRef, ViewType};

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
    state.workspace.netlist_source_dirty = state
        .project_lifecycle
        .registry
        .is_dirty(&ProjectDocumentId::NetlistSource);
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
                        state.push_user_message(crate::common::app::ConsoleMessage::warning(
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
                state.push_user_message(crate::common::app::ConsoleMessage::warning(format!(
                    "Canonical native project was not restored because {reason}; open it explicitly to accept its current bytes"
                )));
            }
            (None, Some(_)) => {
                state.push_user_message(crate::common::app::ConsoleMessage::warning(
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
                state.push_user_message(crate::common::app::ConsoleMessage::warning(
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
                    crate::common::browser_file_import::request_browser_import_repaint();
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
            state.push_user_message(crate::common::app::ConsoleMessage::warning(
                "The browser canonical-binding receipt has no matching restoration record; choose Save to establish a new canonical binding",
            ));
        }
        persistence::BrowserRestoreResult::Restored { baseline, binding } => {
            if baseline.workspace.project.id() != state.workspace.project.id() {
                release_browser_binding_handle(&binding);
                state.push_user_message(crate::common::app::ConsoleMessage::warning(
                    "Ignored a stale browser project binding for a different project identity",
                ));
                return;
            }
            state.project_lifecycle.accepted = Some(AcceptedProject {
                baseline,
                binding: Some(binding),
            });
            state.native_project_binding_receipt = None;
            advance_accepted_generation(&mut state.project_lifecycle);
            let _ = refresh_registry(state);
        }
        persistence::BrowserRestoreResult::ReconnectRequired { binding } => {
            state.project_lifecycle.browser_reconnect_binding = Some(binding);
            state.push_user_message(crate::common::app::ConsoleMessage::warning(
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
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "Canonical browser project conflict: {reason}. Ordinary Save is blocked; reopen it or save an independent project copy"
            )));
        }
        persistence::BrowserRestoreResult::Evicted(reason) => {
            state.browser_project_binding_receipt = None;
            state.push_user_message(crate::common::app::ConsoleMessage::warning(format!(
                "Canonical browser project binding was removed: {reason}. Choose Save to select a canonical file again; download fallback remains copy-only."
            )));
        }
        persistence::BrowserRestoreResult::Retryable(reason) => {
            state.push_user_message(crate::common::app::ConsoleMessage::warning(format!(
                "Canonical browser project could not be restored yet: {reason}. Its restoration record was retained"
            )));
        }
        persistence::BrowserRestoreResult::Unsupported(reason) => {
            state.push_user_message(crate::common::app::ConsoleMessage::warning(format!(
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
    let first_save = state.project_lifecycle.accepted.is_none()
        || state
            .project_lifecycle
            .accepted
            .as_ref()
            .and_then(|accepted| accepted.binding.as_ref())
            .is_none();
    let scope = if first_save {
        SaveScope::AllDocuments
    } else {
        requested_scope
    };
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
        let (bytes, _) = persistence::serialized_project(&candidate)?;
        let digest = persistence::publish_canonical_native(&path, expected, &bytes)?;
        let binding = PersistenceBinding::Native {
            canonical_path: path.clone(),
            accepted_digest: digest,
        };
        finish_successful_save(state, candidate, binding, scope)
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
    let first_save = state.project_lifecycle.accepted.is_none()
        || state
            .project_lifecycle
            .accepted
            .as_ref()
            .and_then(|accepted| accepted.binding.as_ref())
            .is_none();
    let scope = if first_save {
        SaveScope::AllDocuments
    } else {
        requested_scope
    };
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
            .and_then(|binding| match binding {
                PersistenceBinding::Browser { handle_id, .. } => Some(*handle_id),
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
    handle_id: u64,
    binding_id: uuid::Uuid,
    backend: BrowserBindingBackend,
    project_id: String,
    generation: u64,
    display_name: String,
    digest: ContentDigest,
    durable: bool,
) -> Result<(), ProjectLifecycleError> {
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

fn finish_successful_save(
    state: &mut AppState,
    candidate: ProjectFile,
    binding: PersistenceBinding,
    scope: SaveScope,
) -> Result<(), ProjectLifecycleError> {
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
    #[cfg(target_arch = "wasm32")]
    {
        let _ = scope;
        // The picker/write is asynchronous. Rebuilding dirtiness against the
        // newly accepted bytes preserves any edits made while it was pending,
        // including project-descriptor edits, instead of overwriting them or
        // marking them clean.
        return refresh_registry(state);
    }
    #[cfg(not(target_arch = "wasm32"))]
    match scope {
        SaveScope::AllDocuments => {
            state.workspace.mark_all_clean();
            state.schematic.is_dirty = false;
            mark_all_library_views_clean(&mut state.library_manager);
        }
        SaveScope::ActiveDocument => mark_active_document_clean(state),
    }
    #[cfg(not(target_arch = "wasm32"))]
    refresh_registry(state)
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
        ProjectDocumentId::NetlistSource => state.workspace.netlist_source_dirty = false,
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
                state.push_user_message(crate::common::app::ConsoleMessage::warning(warning));
            }
        }
        ProjectDocumentId::ResultHistory => {
            let mut simulation = crate::state::SimulationState::default();
            baseline
                .simulation_results
                .apply_to_state(&mut simulation)
                .map_err(ProjectLifecycleError::InvalidState)?;
            state.simulation = simulation;
            state.clear_specialized_viewer_data();
        }
        ProjectDocumentId::VerificationSpecifications => {
            state.workspace.specs = baseline.workspace.specs;
        }
        ProjectDocumentId::NetlistSource => {
            state.workspace.netlist_source = baseline.workspace.netlist_source;
            state.workspace.netlist_source_path = baseline.workspace.netlist_source_path;
            state.workspace.netlist_source_dirty = false;
            state.ui.netlist = Default::default();
        }
    }
    refresh_registry(state)
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
        }
        ProjectDocumentId::VerificationSpecifications => {
            target.workspace.specs = working.workspace.specs.clone();
        }
        ProjectDocumentId::NetlistSource => {
            target.workspace.netlist_source = working.workspace.netlist_source.clone();
            target.workspace.netlist_source_path = working.workspace.netlist_source_path.clone();
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
        return Err(ProjectLifecycleError::NoAcceptedBaseline);
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
    Ok(())
}

fn restore_project_structure_preserving_documents(
    state: &mut AppState,
    baseline: crate::state::LibraryManager,
) {
    state.library_manager =
        merge_project_structure_with_document_content(&baseline, &state.library_manager);
    state
        .workspace
        .ensure_library_model(&mut state.library_manager);
}

fn merge_project_structure_with_document_content(
    structure: &crate::state::LibraryManager,
    content: &crate::state::LibraryManager,
) -> crate::state::LibraryManager {
    let mut merged = structure.clone();
    let references = merged
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
    schematic.connections.clear();
    schematic.is_dirty = false;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(target_arch = "wasm32"))]
    use crate::simulation::plan::AnalysisKind;
    use crate::state::{ComponentType, Point};

    #[cfg(not(target_arch = "wasm32"))]
    fn insert_ac_analysis(state: &mut AppState) -> crate::product::AnalysisInstanceId {
        state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("current project owns a stable plan")
            .insert(AnalysisKind::Ac)
            .expect("AC analysis inserts")
            .0
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn has_ac_analysis(setup: &crate::common::app::SimSetupState) -> bool {
        setup
            .stable_analysis_plan()
            .expect("current project owns a stable plan")
            .instances()
            .iter()
            .any(|instance| instance.kind() == AnalysisKind::Ac)
    }

    #[test]
    fn browser_completion_context_rejects_every_authority_change() {
        let digest = persistence::digest_bytes(b"accepted browser bytes");
        let receipt = BrowserBindingReceipt {
            binding_id: uuid::Uuid::from_u128(0xf20c_f308_17a1_4fc4_8b0d_8f09_eab7_35c2),
            project_id: "logical-project".to_owned(),
            accepted_generation: 4,
            accepted_digest: digest,
            backend: persistence::BrowserBindingBackend::Opfs,
        };
        let context = BrowserOperationContext {
            epoch: 11,
            operation_generation: 3,
            project_id: receipt.project_id.clone(),
            binding_receipt: Some(receipt.clone()),
            accepted_generation: 9,
        };

        assert!(operation_context_matches(
            &context,
            11,
            3,
            "logical-project",
            Some(&receipt),
            9,
        ));
        assert!(!operation_context_matches(
            &context,
            12,
            3,
            "logical-project",
            Some(&receipt),
            9,
        ));
        assert!(!operation_context_matches(
            &context,
            11,
            3,
            "replacement-project",
            Some(&receipt),
            9,
        ));
        assert!(!operation_context_matches(
            &context,
            11,
            3,
            "logical-project",
            None,
            9,
        ));
        assert!(!operation_context_matches(
            &context,
            11,
            3,
            "logical-project",
            Some(&receipt),
            10,
        ));
        assert!(!operation_context_matches(
            &context,
            11,
            4,
            "logical-project",
            Some(&receipt),
            9,
        ));
    }

    #[test]
    fn lifecycle_epoch_advances_across_new_and_close() {
        let mut state = AppState::default();
        let receipt = NativeBindingReceipt {
            canonical_path: PathBuf::from("accepted.rspiceproj"),
            project_id: state.workspace.project.id().to_string(),
            accepted_digest: crate::product::ContentDigest::from_bytes([0x44; 32]),
        };
        state.native_project_binding_receipt = Some(receipt.clone());
        let initial = state.project_lifecycle.epoch;
        reset_for_new_project(&mut state);
        let after_new = state.project_lifecycle.epoch;
        assert!(state.native_project_binding_receipt.is_none());
        state.native_project_binding_receipt = Some(receipt);
        mark_project_closed(&mut state);
        let after_close = state.project_lifecycle.epoch;

        assert!(after_new > initial);
        assert!(after_close > after_new);
        assert!(state.native_project_binding_receipt.is_none());
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn unique_path(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "rspice-lifecycle-{label}-{}-{}.rspiceproj",
            std::process::id(),
            uuid::Uuid::new_v4()
        ))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn remove_project_artifacts(path: &Path) {
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
        let mut lock = path.as_os_str().to_os_string();
        lock.push(".rspice.lock");
        let _ = std::fs::remove_file(PathBuf::from(lock));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn safe_mode_read_only_policy_blocks_native_project_writes_before_publication() {
        let path = unique_path("safe-mode-read-only");
        let mut state = AppState::default();
        state.workbench.safe_mode.activate(
            crate::workbench::state::LocalSafeModeOptions {
                disable_third_party_extensions: false,
                disable_gpu_acceleration: false,
                isolate_prior_documents: false,
                reset_layout: false,
                open_project_read_only: true,
            },
            "protected session".to_owned(),
        );

        let error = save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .expect_err("read-only safe mode must reject project writes");

        assert!(matches!(error, ProjectLifecycleError::SafeModeReadOnly));
        assert!(!path.exists());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_session_restore_requires_exact_path_project_and_digest_receipt() {
        let path = unique_path("session-receipt");
        let mut state = AppState::default();
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .expect("establish canonical binding");
        let canonical = normalize_native_path(&path).expect("normalize fixture");
        let receipt = state
            .native_project_binding_receipt
            .clone()
            .expect("successful save records receipt");
        let accepted_bytes = std::fs::read(&canonical).expect("read accepted bytes");
        let session = serde_json::to_string(&state).expect("serialize accepted session");

        assert_eq!(receipt.canonical_path, canonical);
        assert_eq!(receipt.project_id, state.workspace.project.id().to_string());

        let mut exact: AppState = serde_json::from_str(&session).expect("restore exact session");
        initialize_from_session(&mut exact);
        assert_eq!(canonical_native_path(&exact), Some(canonical.clone()));
        assert!(exact.project_lifecycle.accepted.is_some());

        let mut same_project = crate::io::load_project_file(&canonical).expect("load fixture");
        same_project
            .workspace
            .project
            .rename("Externally renamed project")
            .expect("valid project name");
        let changed = crate::io::project_io::serialize_project_file(&same_project)
            .expect("serialize same-UUID replacement");
        std::fs::write(&canonical, changed).expect("write same-UUID replacement");
        let changed_bytes = std::fs::read(&canonical).expect("capture replacement bytes");

        let mut digest_conflict: AppState =
            serde_json::from_str(&session).expect("restore conflicted session");
        initialize_from_session(&mut digest_conflict);
        assert!(digest_conflict.project_lifecycle.accepted.is_none());
        assert!(canonical_native_path(&digest_conflict).is_none());
        assert_eq!(
            digest_conflict.native_project_binding_receipt,
            Some(receipt.clone()),
            "conflict evidence must be retained"
        );
        assert_eq!(
            std::fs::read(&canonical).expect("read preserved replacement"),
            changed_bytes,
            "startup conflict handling must not rewrite the target"
        );

        let mut different = snapshot(&AppState::default()).expect("snapshot different project");
        different.workspace.project.set_path(canonical.clone());
        let different_bytes = crate::io::project_io::serialize_project_file(&different)
            .expect("serialize different-UUID replacement");
        std::fs::write(&canonical, different_bytes.as_bytes())
            .expect("write different-UUID replacement");
        let mut identity_conflict: AppState =
            serde_json::from_str(&session).expect("restore identity-conflict session");
        initialize_from_session(&mut identity_conflict);
        assert!(identity_conflict.project_lifecycle.accepted.is_none());
        assert!(canonical_native_path(&identity_conflict).is_none());

        std::fs::write(&canonical, &accepted_bytes).expect("restore exact fixture bytes");
        let mut legacy_value =
            serde_json::from_str::<serde_json::Value>(&session).expect("parse session JSON");
        legacy_value
            .as_object_mut()
            .expect("session object")
            .remove("native_project_binding_receipt");
        let mut legacy: AppState =
            serde_json::from_value(legacy_value).expect("restore legacy session");
        initialize_from_session(&mut legacy);
        assert!(legacy.project_lifecycle.accepted.is_none());
        assert!(canonical_native_path(&legacy).is_none());

        std::fs::remove_file(&canonical).expect("remove canonical fixture");
        let mut missing: AppState =
            serde_json::from_str(&session).expect("restore missing session");
        initialize_from_session(&mut missing);
        assert!(missing.project_lifecycle.accepted.is_none());
        assert!(canonical_native_path(&missing).is_none());
        remove_project_artifacts(&path);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn user_selected_native_publication_rejects_late_create_and_edit() {
        let path = unique_path("picker-cas");
        let missing = persistence::observe_native_destination(&path)
            .expect("observe picker destination as missing");
        std::fs::write(&path, b"created after picker").expect("simulate late create");

        let create_conflict =
            persistence::publish_canonical_native(&path, missing, b"local project bytes")
                .expect_err("late create must block publication");
        assert!(matches!(
            create_conflict,
            persistence::PersistenceError::ExternalChange
        ));
        assert_eq!(
            std::fs::read(&path).expect("read late create"),
            b"created after picker"
        );

        let accepted = persistence::observe_native_destination(&path)
            .expect("capture exact picker-time bytes");
        std::fs::write(&path, b"edited after picker").expect("simulate late edit");
        let edit_conflict =
            persistence::publish_canonical_native(&path, accepted, b"local project bytes")
                .expect_err("late edit must block publication");
        assert!(matches!(
            edit_conflict,
            persistence::PersistenceError::ExternalChange
        ));
        assert_eq!(
            std::fs::read(&path).expect("read late edit"),
            b"edited after picker"
        );
        remove_project_artifacts(&path);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn save_active_overlays_only_active_document_on_accepted_baseline() {
        let path = unique_path("active-overlay");
        let mut state = AppState::default();
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .expect("first full save");

        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(4, 8));
        let ac_id = insert_ac_analysis(&mut state);
        assert!(
            save_native(
                &mut state,
                SaveScope::ActiveDocument,
                &path,
                DestinationAuthority::Canonical,
            )
            .is_ok()
        );

        let persisted = crate::io::load_project_file(&path).expect("reload saved project");
        let persisted_context = persisted.execution_context.expect("execution context");
        assert_eq!(
            persisted
                .workspace
                .schematic_buffers
                .get(&state.workspace.active_key())
                .expect("active buffer")
                .components
                .len(),
            1
        );
        assert!(
            !has_ac_analysis(&persisted_context.simulation_plan),
            "unrelated plan draft must remain outside an active-design save"
        );
        assert_eq!(
            state
                .sim_setup
                .stable_analysis_plan()
                .expect("live project owns a stable plan")
                .instance(ac_id)
                .expect("active-design save retains the exact live AC identity")
                .kind(),
            AnalysisKind::Ac
        );
        assert!(has_unsaved_changes(&state));
        assert!(!active_document_is_dirty(&state));
        remove_project_artifacts(&path);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn save_active_design_preserves_unpublished_live_project_descriptor() {
        let path = unique_path("active-preserves-project-draft");
        let mut state = AppState::default();
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .expect("establish baseline");
        let accepted_name = state.workspace.project.name().to_owned();

        state
            .workspace
            .project
            .rename("Unpublished descriptor draft")
            .expect("valid draft name");
        state.workspace.project.description = "not part of design Save".to_owned();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(9, 4));

        save_native(
            &mut state,
            SaveScope::ActiveDocument,
            &path,
            DestinationAuthority::Canonical,
        )
        .expect("save active design only");

        assert_eq!(
            state.workspace.project.name(),
            "Unpublished descriptor draft",
            "successful design save must not roll back a different document's draft"
        );
        assert_eq!(
            state.workspace.project.description,
            "not part of design Save"
        );
        assert!(
            state
                .project_lifecycle
                .registry
                .is_dirty(&ProjectDocumentId::ProjectConfiguration)
        );
        assert!(!active_document_is_dirty(&state));

        let persisted = crate::io::load_project_file(&path).expect("reload active save");
        assert_eq!(persisted.workspace.project.name(), accepted_name);
        assert!(persisted.workspace.project.description.is_empty());
        remove_project_artifacts(&path);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn newer_edits_after_saved_snapshot_revoke_destructive_continuation() {
        let path = unique_path("continuation-guard");
        let mut state = AppState::default();
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .expect("establish accepted snapshot");
        let saved_document = active_document(&state);

        assert!(saved_snapshot_authorizes_continuation(
            &state,
            SaveScope::AllDocuments,
            &saved_document
        ));
        state
            .schematic
            .add_component(ComponentType::Capacitor, Point::new(6, 2));
        assert!(
            !saved_snapshot_authorizes_continuation(
                &state,
                SaveScope::AllDocuments,
                &saved_document
            ),
            "post-snapshot project edits require another Save All before replacement"
        );
        assert!(
            !saved_snapshot_authorizes_continuation(
                &state,
                SaveScope::ActiveDocument,
                &saved_document
            ),
            "post-snapshot active-document edits require another save"
        );

        save_native(
            &mut state,
            SaveScope::ActiveDocument,
            &path,
            DestinationAuthority::Canonical,
        )
        .expect("save active document");
        assert!(saved_snapshot_authorizes_continuation(
            &state,
            SaveScope::ActiveDocument,
            &saved_document
        ));
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Project);
        assert!(
            !saved_snapshot_authorizes_continuation(
                &state,
                SaveScope::ActiveDocument,
                &saved_document
            ),
            "an active-tab change cannot redirect an old continuation"
        );
        remove_project_artifacts(&path);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn save_all_commits_complete_working_set() {
        let path = unique_path("save-all");
        let mut state = AppState::default();
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .expect("first save");
        let ac_id = insert_ac_analysis(&mut state);

        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::Canonical,
        )
        .expect("save all");

        let persisted = crate::io::load_project_file(&path).expect("reload");
        let persisted_context = persisted.execution_context.expect("context");
        assert_eq!(
            persisted_context
                .simulation_plan
                .stable_analysis_plan()
                .expect("saved project owns a stable plan")
                .instance(ac_id)
                .expect("saved plan retains the exact AC identity")
                .kind(),
            AnalysisKind::Ac
        );
        assert!(!has_unsaved_changes(&state));
        remove_project_artifacts(&path);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn project_copy_does_not_rebind_or_clean_source_project() {
        let source = unique_path("copy-source");
        let copy = unique_path("copy-target");
        let mut state = AppState::default();
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &source,
            DestinationAuthority::UserSelected,
        )
        .expect("save source");
        let source_id = state.workspace.project.id();
        let source_receipt = state.native_project_binding_receipt.clone();
        state
            .schematic
            .add_component(ComponentType::Capacitor, Point::new(1, 2));

        save_project_copy_native(&mut state, &copy).expect("save independent copy");

        assert_eq!(state.workspace.project.id(), source_id);
        assert_eq!(state.native_project_binding_receipt, source_receipt);
        assert_eq!(
            canonical_native_path(&state),
            Some(normalize_native_path(&source).expect("normalize source"))
        );
        assert!(has_unsaved_changes(&state));
        let copied = crate::io::load_project_file(&copy).expect("load copy");
        assert_ne!(copied.workspace.project.id(), source_id);
        remove_project_artifacts(&source);
        remove_project_artifacts(&copy);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn project_copy_rejects_direct_normalized_symlink_and_hardlink_aliases() {
        let source = unique_path("copy-alias-source");
        let hardlink = unique_path("copy-alias-hardlink");
        let symlink = unique_path("copy-alias-symlink");
        let mut state = AppState::default();
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &source,
            DestinationAuthority::UserSelected,
        )
        .expect("save canonical source");
        state
            .schematic
            .add_component(ComponentType::Diode, Point::new(7, 9));

        let direct = save_project_copy_native(&mut state, &source)
            .expect_err("direct canonical destination must be rejected");
        assert!(matches!(
            direct,
            ProjectLifecycleError::CopyDestinationIsCanonical
        ));

        let normalized_alias = source
            .parent()
            .expect("temporary parent")
            .join(".")
            .join(source.file_name().expect("source filename"));
        let normalized = save_project_copy_native(&mut state, &normalized_alias)
            .expect_err("normalized canonical alias must be rejected");
        assert!(matches!(
            normalized,
            ProjectLifecycleError::CopyDestinationIsCanonical
        ));

        std::fs::hard_link(&source, &hardlink).expect("create hardlink alias");
        let hardlinked = save_project_copy_native(&mut state, &hardlink)
            .expect_err("hardlink alias must be rejected");
        assert!(matches!(
            hardlinked,
            ProjectLifecycleError::CopyDestinationIsCanonical
        ));

        #[cfg(unix)]
        std::os::unix::fs::symlink(&source, &symlink).expect("create symlink alias");
        #[cfg(windows)]
        let symlink_created = std::os::windows::fs::symlink_file(&source, &symlink).is_ok();
        #[cfg(unix)]
        let symlink_created = true;
        if symlink_created {
            let symlinked = save_project_copy_native(&mut state, &symlink)
                .expect_err("symlink alias must be rejected");
            assert!(matches!(
                symlinked,
                ProjectLifecycleError::CopyDestinationIsCanonical
            ));
        }

        assert!(has_unsaved_changes(&state));
        remove_project_artifacts(&hardlink);
        remove_project_artifacts(&symlink);
        remove_project_artifacts(&source);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn deleted_source_allows_recovery_copy_but_unreadable_canonical_path_is_rejected() {
        let source = unique_path("deleted-copy-source");
        let recovery = unique_path("deleted-copy-recovery");
        let mut state = AppState::default();
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &source,
            DestinationAuthority::UserSelected,
        )
        .expect("establish canonical source");
        std::fs::remove_file(&source).expect("simulate external source deletion");
        std::fs::write(&recovery, b"picker-observed prior recovery bytes")
            .expect("create existing recovery destination");
        state
            .schematic
            .add_component(ComponentType::Diode, Point::new(12, 7));

        save_project_copy_native(&mut state, &recovery)
            .expect("missing source cannot alias an independent recovery destination");
        let recovered = crate::io::load_project_file(&recovery).expect("load recovery copy");
        assert_ne!(
            recovered.workspace.project.id(),
            state.workspace.project.id()
        );

        let unreadable = unique_path("remembered-unreadable-canonical");
        std::fs::write(&unreadable, b"not a project").expect("create unreadable canonical");
        let unreadable = normalize_native_path(&unreadable).expect("normalize unreadable path");
        let mut unreadable_state = AppState::default();
        unreadable_state.project_lifecycle.unreadable_native_binding =
            Some(persistence::UnreadableNativeBinding {
                canonical_path: unreadable.clone(),
                reason: "invalid project bytes".to_owned(),
            });
        let before = std::fs::read(&unreadable).expect("capture unreadable bytes");
        let error = save_project_copy_native(&mut unreadable_state, &unreadable)
            .expect_err("Save Copy cannot overwrite remembered unreadable authority");
        assert!(matches!(
            error,
            ProjectLifecycleError::CopyDestinationIsCanonical
        ));
        assert_eq!(std::fs::read(&unreadable).unwrap(), before);

        remove_project_artifacts(&source);
        remove_project_artifacts(&recovery);
        remove_project_artifacts(&unreadable);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn save_all_preserves_live_document_presentation_while_sanitizing_persisted_copy() {
        let path = unique_path("presentation-preservation");
        let view_path = unique_path("view-presentation");
        let mut state = AppState::default();
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .expect("save baseline");
        let active = state.workspace.active_view.clone();
        let open_count = state.workspace.open_views.len();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(14, 3));
        let view = state
            .library_manager
            .get_library_mut(&active.library)
            .and_then(|library| library.get_cell_mut(&active.cell))
            .and_then(|cell| cell.get_view_mut(&active.view))
            .expect("active library view");
        view.is_open = true;
        view.modified = true;
        view.file_path = Some(view_path.clone());
        view.modified_time = Some(8_675_309);

        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::Canonical,
        )
        .expect("save all");

        assert_eq!(state.workspace.active_view, active);
        assert_eq!(state.workspace.open_views.len(), open_count);
        let live = state
            .library_manager
            .get_library(&active.library)
            .and_then(|library| library.get_cell(&active.cell))
            .and_then(|cell| cell.get_view(&active.view))
            .expect("live active view");
        assert!(live.is_open);
        assert!(!live.modified);
        assert_eq!(live.file_path.as_deref(), Some(view_path.as_path()));
        assert_eq!(live.modified_time, Some(8_675_309));

        let persisted = crate::io::load_project_file(&path).expect("reload persisted project");
        let persisted_view = persisted
            .libraries
            .get_library(&active.library)
            .and_then(|library| library.get_cell(&active.cell))
            .and_then(|cell| cell.get_view(&active.view))
            .expect("persisted active view");
        assert!(!persisted_view.is_open);
        assert!(!persisted_view.modified);
        assert!(persisted_view.file_path.is_none());
        assert!(persisted_view.modified_time.is_none());
        remove_project_artifacts(&path);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn saving_active_cell_never_dirties_project_configuration() {
        let path = unique_path("active-cell-config-boundary");
        let mut state = AppState::default();
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .expect("save baseline");
        let active = state.workspace.active_view.clone();
        state
            .schematic
            .add_component(ComponentType::Capacitor, Point::new(5, 11));
        state
            .library_manager
            .get_library_mut(&active.library)
            .and_then(|library| library.get_cell_mut(&active.cell))
            .and_then(|cell| cell.get_view_mut(&active.view))
            .expect("active view")
            .metadata
            .insert(
                "document-setting".to_owned(),
                "engineering-value".to_owned(),
            );

        save_native(
            &mut state,
            SaveScope::ActiveDocument,
            &path,
            DestinationAuthority::Canonical,
        )
        .expect("save active cell");

        assert!(
            !state
                .project_lifecycle
                .registry
                .is_dirty(&ProjectDocumentId::CellView(active))
        );
        assert!(
            !state
                .project_lifecycle
                .registry
                .is_dirty(&ProjectDocumentId::ProjectConfiguration)
        );
        assert!(!has_unsaved_changes(&state));
        remove_project_artifacts(&path);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn revert_is_document_scoped_and_rejects_active_document_and_baseline_races() {
        let path = unique_path("revert-races");
        let mut state = AppState::default();
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .expect("save baseline");

        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(2, 2));
        let ac_id = insert_ac_analysis(&mut state);
        let scoped = prepare_revert_active_document(&state).expect("prepare scoped revert");
        confirm_revert_active_document(&mut state, &scoped).expect("confirm scoped revert");
        assert!(state.schematic.components.is_empty());
        assert_eq!(
            state
                .sim_setup
                .stable_analysis_plan()
                .expect("live plan retained")
                .instance(ac_id)
                .expect("AC identity retained")
                .kind(),
            AnalysisKind::Ac
        );
        assert!(has_unsaved_changes(&state));

        state
            .schematic
            .add_component(ComponentType::Inductor, Point::new(4, 7));
        let active_race = prepare_revert_active_document(&state).expect("prepare active race");
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Project);
        let active_race_error = confirm_revert_active_document(&mut state, &active_race)
            .expect_err("changed active document invalidates review");
        assert!(matches!(
            active_race_error,
            ProjectLifecycleError::RevertReviewStale
        ));
        assert_eq!(state.schematic.components.len(), 1);

        state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);
        let baseline_race = prepare_revert_active_document(&state).expect("prepare baseline race");
        save_native(
            &mut state,
            SaveScope::ActiveDocument,
            &path,
            DestinationAuthority::Canonical,
        )
        .expect("advance accepted baseline");
        state
            .schematic
            .add_component(ComponentType::Diode, Point::new(8, 12));
        let baseline_race_error = confirm_revert_active_document(&mut state, &baseline_race)
            .expect_err("changed accepted baseline invalidates review");
        assert!(matches!(
            baseline_race_error,
            ProjectLifecycleError::RevertReviewStale
        ));
        assert_eq!(state.schematic.components.len(), 2);
        remove_project_artifacts(&path);
    }

    #[test]
    fn close_active_document_is_presentation_only() {
        let mut state = AppState::default();
        let second = CellViewRef::new("user", "second", "schematic");
        assert!(state.library_manager.create_cell("user", "second"));
        assert!(state.library_manager.create_view(
            "user",
            "second",
            "schematic",
            ViewType::Schematic
        ));
        state
            .workspace
            .open_view(second.clone(), ViewType::Schematic);
        state.restore_active_schematic_from_workspace();
        state
            .schematic
            .add_component(ComponentType::Inductor, Point::new(3, 5));
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);

        close_active_document(&mut state).expect("close presentation");

        assert!(
            !state
                .workspace
                .open_views
                .iter()
                .any(|open| open.reference == second)
        );
        assert_eq!(
            state
                .workspace
                .schematic_buffers
                .get(&second.key())
                .expect("closed document data retained")
                .components
                .len(),
            1
        );
    }
}
