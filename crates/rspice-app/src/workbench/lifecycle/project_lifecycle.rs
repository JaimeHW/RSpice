//! Transactional project/document lifecycle.
//!
//! The accepted project baseline is intentionally separate from the mutable
//! workbench state. `Save` overlays one stable document onto that baseline;
//! `Save all` replaces it with the complete working set. This prevents saving
//! one tab from accidentally committing unrelated drafts.

mod accepted_project;
mod persistence;
mod result_cache;

#[cfg(target_arch = "wasm32")]
pub(crate) use persistence::{
    start_browser_checkpoint_list, start_browser_checkpoint_publish, start_browser_checkpoint_read,
};
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use rspice_project::persistence::native::DestinationAuthority;
#[cfg(not(target_arch = "wasm32"))]
use rspice_project::persistence::native::{NativeCopyDestination, NativeSaveDestination};
use std::rc::Rc;
mod registry;
mod source;

#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
#[cfg(any(test, not(target_arch = "wasm32")))]
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
#[cfg(target_arch = "wasm32")]
use rspice_project::lifecycle::{PreparedBrowserSave, StagedBrowserSave};
pub(crate) use rspice_project::lifecycle::{ProjectLifecycleError, RevertReviewToken, SaveScope};
pub(crate) use source::{
    LifecycleHost, LifecycleParts, LifecycleSource, LifecycleSourceMut, LifecycleView,
    ResultPresentationSource,
};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::io::ProjectSnapshot;
#[cfg(target_arch = "wasm32")]
use crate::product::ContentDigest;
use crate::state::project_snapshot::SnapshotSessionCapture;
use crate::state::{CellViewRef, ProjectOpenFact};
// The lower-tier state the lifecycle and its sibling services work on.
pub(in crate::workbench) use crate::workbench::app_state::{Model, SimSetupState};
pub(in crate::workbench) use crate::workbench::slots::platform::PlatformState;
use rspice_project::{ProjectWorkingSet, SnapshotContent};

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

/// Where the validated close-project transaction should leave the user.
///
/// This is runtime-only intent. It travels with the close itself, through the
/// review dialog's request and a browser canonical-save continuation, so the
/// close completes the exact originally requested transition without
/// inferring intent from whichever surface is visible later.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProjectCloseDestination {
    /// The no-project landing: `Workspace::Project` with nothing open.
    EmptyWorkbench,
    /// Joining a live session: the same landing, where this install then
    /// waits for the session host's project snapshot.
    LiveMirror,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ProjectLifecycleState {
    pub(crate) authority: ProjectLifecycle,
    accepted_session: Rc<crate::state::workspace::WorkspaceSession>,
    accepted_binding: Option<PersistenceBinding>,
    #[cfg(not(target_arch = "wasm32"))]
    unreadable_native_binding: Option<persistence::UnreadableNativeBinding>,
    result_cache: result_cache::ResultCache,
    #[cfg(target_arch = "wasm32")]
    browser_reconnect_binding: Option<PersistenceBinding>,
    #[cfg(target_arch = "wasm32")]
    browser_conflict: Option<BrowserConflict>,
}

impl ProjectLifecycleState {
    pub(crate) const fn is_open(&self) -> bool {
        self.authority.is_open()
    }

    /// The one writer of the model's copy of whether a project is open.
    ///
    /// Every transition that changes [`Self::is_open`] calls this in the same
    /// call, so model code reading the copy never sees an answer the lifecycle
    /// no longer gives.
    fn publish_open(&self, fact: &mut ProjectOpenFact) {
        fact.set_from_lifecycle(self.is_open());
    }

    /// A restored session that recorded no open project starts with its
    /// project closed. A restore builds a fresh lifecycle, so there is nothing
    /// else to clear.
    pub(crate) fn close_restored_session(&mut self, fact: &mut ProjectOpenFact) {
        self.authority.close_project();
        self.publish_open(fact);
    }

    pub(crate) fn operation_in_progress(&self) -> bool {
        self.authority.operation_in_progress()
    }

    pub(crate) fn accepted(&self) -> Option<&rspice_project::AcceptedProject> {
        self.authority.accepted()
    }

    pub(crate) fn binding(&self) -> Option<&PersistenceBinding> {
        self.accepted_binding.as_ref()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn canonical_native_path(&self) -> Option<&Path> {
        self.binding()
            .map(|binding| binding.canonical_path.as_path())
    }
}

/// Resolve the actual publication scope before validation starts. The first
/// canonical save has no accepted baseline that can safely receive a
/// document-only overlay, so it necessarily publishes the complete project.
/// Workflows such as Check and save use this same decision before presenting
/// their document scope and before freezing validation evidence.
pub(crate) fn effective_save_scope(platform: &PlatformState, requested: SaveScope) -> SaveScope {
    platform
        .project_lifecycle
        .authority
        .effective_save_scope(requested, platform.project_lifecycle.binding().is_some())
}

/// Exact active schematic from the currently accepted canonical project.
/// This is used only to seed the validated-save journal when an older project
/// predates that journal. A missing canonical binding or a newly created view
/// has no predecessor and therefore returns `None`.
pub(crate) fn accepted_active_schematic(
    model: &Model,
    platform: &PlatformState,
) -> Option<crate::state::SchematicState> {
    platform.project_lifecycle.binding()?;
    platform
        .project_lifecycle
        .accepted_schematic_editor(&model.workspace.content.active_view.key())
}

/// Monotonic identity of the accepted canonical baseline. Validation receipts
/// bind this value so a save opened against one baseline cannot publish after
/// a different save, import, or binding restoration has completed.
pub(crate) const fn accepted_generation(platform: &PlatformState) -> u64 {
    platform.project_lifecycle.authority.accepted_generation()
}

pub(crate) fn snapshot(
    state: &dyn LifecycleSource,
) -> Result<ProjectSnapshot, ProjectLifecycleError> {
    capture_snapshot(state, SnapshotContent::Committed)
}

fn capture_snapshot(
    state: &dyn LifecycleSource,
    content: SnapshotContent,
) -> Result<ProjectSnapshot, ProjectLifecycleError> {
    let state = &state.lifecycle_view();
    let mut sessions = SnapshotSessionCapture {
        workspace: state.model.workspace.session.clone(),
        active: &state.model.schematic.session,
    };
    let file = ProjectWorkingSet {
        workspace: &state.model.workspace.content,
        active_schematic: state.model.schematic.editor_ref().design,
        libraries: &state.model.library_manager,
    }
    .capture(
        content,
        &mut sessions,
        || {
            state
                .platform
                .project_lifecycle
                .result_cache
                .capture(&state.model.simulation)
        },
        || {
            crate::io::capture_execution_context(
                &state.model.sim_setup,
                &state.model.model_library_manager,
            )
            .map_err(ProjectLifecycleError::InvalidState)
        },
        || {
            state
                .presentation
                .project_presentation(&state.model.simulation)
        },
    )?;
    Ok(ProjectSnapshot {
        file,
        workspace_session: sessions.workspace,
    })
}

/// Canonical identity of every authoritative input consumed by schematic
/// netlist generation. Result history and independently owned source decks are
/// removed because neither is a generator input; design, hierarchy, project
/// configuration, simulation-plan payloads, project-owned behavioral sources,
/// model bindings, and libraries remain covered by the ordinary document
/// digests. Project sources must stay authenticated because their virtual
/// directive identities and exact bytes affect generated output and execution.
pub(crate) fn generated_netlist_input_digest(
    state: &dyn LifecycleSource,
) -> Result<crate::product::ContentDigest, ProjectLifecycleError> {
    // Execution provenance must describe the exact live input consumed by
    // generation, including an edit currently previewed in another window.
    capture_snapshot(state, SnapshotContent::Current)?
        .file
        .generated_netlist_input_digest()
}

pub(crate) fn has_unsaved_changes(state: &dyn LifecycleSource) -> bool {
    let state = &state.lifecycle_view();
    state
        .platform
        .project_lifecycle
        .authority
        .has_unsaved_changes(|| {
            capture_snapshot(state, SnapshotContent::Current).map(|snapshot| snapshot.file)
        })
}

pub(crate) fn operation_in_progress(platform: &PlatformState) -> bool {
    platform.project_lifecycle.operation_in_progress()
}

pub(crate) fn active_document(state: &dyn LifecycleSource) -> ProjectDocumentId {
    let state = state.lifecycle_view();
    registry::active_document(state.workspace, &state.model.workspace.content.active_view)
}

pub(crate) fn active_document_is_dirty(state: &dyn LifecycleSource) -> bool {
    let state = &state.lifecycle_view();
    state
        .platform
        .project_lifecycle
        .authority
        .document_is_dirty(
            || active_document(state),
            || capture_snapshot(state, SnapshotContent::Current).map(|snapshot| snapshot.file),
        )
}

pub(crate) fn refresh_registry(
    state: &mut dyn LifecycleSourceMut,
) -> Result<(), ProjectLifecycleError> {
    let state = &mut state.lifecycle_parts();
    let comparison = state
        .platform
        .project_lifecycle
        .authority
        .prepare_registry_refresh(|| {
            capture_snapshot(state, SnapshotContent::Current).map(|snapshot| snapshot.file)
        });
    let result = state
        .platform
        .project_lifecycle
        .authority
        .finish_registry_refresh(comparison);
    if state.platform.project_lifecycle.is_open() {
        apply_registry_dirty_flags(state);
    }
    result
}

fn apply_registry_dirty_flags(state: &mut LifecycleParts<'_>) {
    if state
        .platform
        .project_lifecycle
        .authority
        .registry()
        .comparison_failed()
    {
        // The working draft could not be compared with accepted content.
        // Preserve its authorship and show pending changes until a successful
        // refresh can establish which documents are actually clean.
        state.model.schematic.session.is_dirty = true;
        state.model.workspace.content.project_metadata_dirty = true;
        state.model.workspace.content.netlist_source_dirty = true;
        state.model.workspace.content.project_sources_dirty = true;
        state
            .model
            .workspace
            .for_each_schematic_editor_mut(|_, schematic| {
                schematic.session.is_dirty = true;
            });
        for view in &mut state.model.workspace.content.open_views {
            view.dirty = true;
        }
        state
            .model
            .library_manager
            .set_all_views_modified_runtime(true);
        return;
    }
    let cell_dirty = state
        .platform
        .project_lifecycle
        .authority
        .registry()
        .records()
        .iter()
        .filter_map(|record| match &record.id {
            ProjectDocumentId::CellView(reference) => Some((reference.clone(), record.dirty)),
            _ => None,
        })
        .collect::<Vec<_>>();
    for (reference, dirty) in &cell_dirty {
        if let Some(mut buffer) = state.model.workspace.schematic_editor_mut(&reference.key()) {
            buffer.editor.session.is_dirty = *dirty;
        }
        if let Some(open) = state
            .model
            .workspace
            .content
            .open_views
            .iter_mut()
            .find(|open| open.reference == *reference)
        {
            open.dirty = *dirty;
        }
        let _ = state.model.library_manager.set_view_modified_runtime(
            &reference.library,
            &reference.cell,
            &reference.view,
            *dirty,
        );
    }
    if let Some((_, dirty)) = cell_dirty
        .iter()
        .find(|(reference, _)| *reference == state.model.workspace.content.active_view)
    {
        state.model.schematic.session.is_dirty = *dirty;
    }
    // Project setup is dirty when it differs from the accepted baseline and at
    // no other time. A flag that only ever turned on left the unsaved marker
    // lit for the rest of the session, including after the save that published
    // the very edit that lit it.
    state.model.workspace.content.project_metadata_dirty = state
        .platform
        .project_lifecycle
        .authority
        .registry()
        .is_dirty(&ProjectDocumentId::ProjectConfiguration);
    if let Some(accepted) = state.platform.project_lifecycle.accepted() {
        accepted.apply_source_dirty_flags(&mut state.model.workspace.content);
    }
}

/// Bind the open project the restored session holds to its file. A session
/// saved with no project open is never bound or opened; whoever restores it
/// lands it on the no-project landing instead of calling this.
pub(crate) fn bind_restored_project(state: &mut dyn LifecycleSourceMut) {
    let state = &mut state.lifecycle_parts();
    #[cfg(not(target_arch = "wasm32"))]
    {
        let restored_path = state.model.workspace.content.project.path.clone();
        let receipt = state.platform.native_project_binding_receipt.clone();
        match (restored_path, receipt) {
            (Some(path), Some(receipt)) => {
                let session_project_id = state.model.workspace.content.project.id().to_string();
                match persistence::restore_native_binding(&path, &session_project_id, &receipt) {
                    Ok((baseline, binding)) => {
                        state
                            .platform
                            .project_lifecycle
                            .restore_project(baseline, binding);
                        state.platform.browser_project_binding_receipt = None;
                    }
                    Err(error) => {
                        if path.exists() {
                            let canonical_path = persistence::normalize_native_path(&path)
                                .unwrap_or_else(|_| path.clone());
                            state.platform.project_lifecycle.unreadable_native_binding =
                                Some(persistence::UnreadableNativeBinding {
                                    canonical_path,
                                    reason: error.to_string(),
                                });
                        }
                        state.model.record(
                            LogSeverity::Warning,
                            LogSource::Studio,
                            format!(
                                "The project file from the last session was not reopened: {error}. The file was left untouched; open it, or save a copy of the project."
                            ),
                        );
                    }
                }
            }
            (Some(path), None) if path.exists() => {
                let canonical_path =
                    persistence::normalize_native_path(&path).unwrap_or_else(|_| path.clone());
                let reason = "the saved session does not record the file's binding".to_owned();
                state.platform.project_lifecycle.unreadable_native_binding =
                    Some(persistence::UnreadableNativeBinding {
                        canonical_path,
                        reason: reason.clone(),
                    });
                state.model.record(
                    LogSeverity::Warning,
                    LogSource::Studio,
                    format!(
                        "The project file from the last session was not reopened because {reason}; open it to accept its current contents."
                    ),
                );
            }
            (None, Some(_)) => {
                state.model.record(
                    LogSeverity::Warning,
                    LogSource::Studio,
                    "Ignored a saved project-file binding that has no path.",
                );
            }
            (Some(_), None) | (None, None) => {}
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(receipt) = state.platform.browser_project_binding_receipt.clone() {
            let project_id = state.model.workspace.content.project.id().to_string();
            if receipt.project_id != project_id {
                state.platform.browser_project_binding_receipt = None;
                state.model.record(
                    LogSeverity::Warning,
                    LogSource::Studio,
                    "Ignored a saved browser file binding that belongs to a different project.",
                );
            } else {
                state
                    .platform
                    .project_lifecycle
                    .authority
                    .begin_browser_restore();
                let context = browser_operation_context(state.model, state.platform);
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
pub(crate) fn poll_browser_binding_restore(state: &mut dyn LifecycleSourceMut) {
    let state = &mut state.lifecycle_parts();
    let Some(completion) =
        BROWSER_BINDING_RESTORE_RESULTS.with(|queue| queue.borrow_mut().pop_front())
    else {
        return;
    };
    if !browser_operation_context_is_current(state.model, state.platform, &completion.context) {
        release_restore_result_handle(completion.result);
        return;
    }
    state
        .platform
        .project_lifecycle
        .authority
        .finish_browser_restore();
    match completion.result {
        persistence::BrowserRestoreResult::Missing => {
            state.platform.browser_project_binding_receipt = None;
            state.model.record(
                LogSeverity::Warning,
                LogSource::Studio,
                "The browser has no record of the project's file; choose Save to pick a file again.",
            );
        }
        persistence::BrowserRestoreResult::Restored { baseline, binding } => {
            if baseline.file.workspace.project.id() != state.model.workspace.content.project.id() {
                release_browser_binding_handle(&binding);
                state.model.record(
                    LogSeverity::Warning,
                    LogSource::Studio,
                    "Ignored a saved browser file binding that belongs to a different project.",
                );
                return;
            }
            state
                .platform
                .project_lifecycle
                .accept_project(*baseline, Some(binding));
            state.platform.native_project_binding_receipt = None;
            let _ = refresh_registry(state);
        }
        persistence::BrowserRestoreResult::ReconnectRequired { binding } => {
            state.platform.project_lifecycle.browser_reconnect_binding = Some(binding);
            state.model.record(
                LogSeverity::Warning,
                LogSource::Studio,
                "The browser needs permission to write the project file again. Choose Save to reconnect; nothing is overwritten unless the file is unchanged.",
            );
        }
        persistence::BrowserRestoreResult::Conflict {
            binding,
            observed_digest,
            reason,
        } => {
            state.platform.project_lifecycle.browser_conflict = Some(BrowserConflict {
                binding,
                observed_digest,
            });
            state.model.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!(
                    "The project file changed outside RSpice: {reason}. Save is blocked; reopen the project or save a copy."
                ),
            );
        }
        persistence::BrowserRestoreResult::Evicted(reason) => {
            state.platform.browser_project_binding_receipt = None;
            state.model.record(
                LogSeverity::Warning,
                LogSource::Studio,
                format!(
                    "The browser removed the project's file binding: {reason}. Choose Save to pick a file again; downloads remain copies only."
                ),
            );
        }
        persistence::BrowserRestoreResult::Retryable(reason) => {
            state.model.record(
                LogSeverity::Warning,
                LogSource::Studio,
                format!(
                    "The project file could not be reconnected yet: {reason}. Its binding was kept."
                ),
            );
        }
        persistence::BrowserRestoreResult::Unsupported(reason) => {
            state.model.record(
                LogSeverity::Warning,
                LogSource::Studio,
                format!("Saving to a file is unavailable in this browser: {reason}"),
            );
        }
    }
}

/// Accept a loaded project as the open project. The features' own state for
/// the project being replaced is reset first, by whoever holds them.
pub(crate) fn accept_loaded_project(
    state: &mut dyn LifecycleSourceMut,
    baseline: ProjectSnapshot,
    binding: Option<PersistenceBinding>,
) {
    let state = &mut state.lifecycle_parts();
    #[cfg(not(target_arch = "wasm32"))]
    let native_receipt = binding
        .as_ref()
        .map(|binding| binding.native_receipt(&baseline.file.workspace.project.id().to_string()));
    #[cfg(target_arch = "wasm32")]
    release_replaced_browser_bindings(&state.platform.project_lifecycle, binding.as_ref());
    state.platform.project_lifecycle.authority.open_session();
    state
        .platform
        .project_lifecycle
        .publish_open(&mut state.model.project_open);
    state
        .platform
        .project_lifecycle
        .accept_project(baseline, binding);
    #[cfg(not(target_arch = "wasm32"))]
    {
        state.platform.project_lifecycle.unreadable_native_binding = None;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        state.platform.native_project_binding_receipt = native_receipt;
        state.platform.browser_project_binding_receipt = None;
    }
    #[cfg(target_arch = "wasm32")]
    {
        state.platform.native_project_binding_receipt = None;
        state.platform.browser_project_binding_receipt = state
            .platform
            .project_lifecycle
            .binding()
            .and_then(PersistenceBinding::durable_browser_receipt);
        state.platform.project_lifecycle.browser_reconnect_binding = None;
        state.platform.project_lifecycle.browser_conflict = None;
        state
            .platform
            .project_lifecycle
            .authority
            .clear_browser_pending();
    }
    state
        .platform
        .project_lifecycle
        .authority
        .cancel_transaction();
    let _ = refresh_registry(state);
}

/// Reset the lifecycle for a new, unsaved project. The features' own state
/// for the project being replaced is reset first, by whoever holds them.
pub(crate) fn reset_for_new_project(state: &mut dyn LifecycleSourceMut) {
    let state = &mut state.lifecycle_parts();
    state.platform.native_project_binding_receipt = None;
    state.platform.browser_project_binding_receipt = None;
    #[cfg(target_arch = "wasm32")]
    {
        clear_browser_handles();
    }
    let mut authority = std::mem::take(&mut state.platform.project_lifecycle.authority);
    authority.reset_for_new_project();
    *state.platform.project_lifecycle = ProjectLifecycleState {
        authority,
        ..ProjectLifecycleState::default()
    };
    state
        .platform
        .project_lifecycle
        .publish_open(&mut state.model.project_open);
    let _ = refresh_registry(state);
}

/// Close the open project's lifecycle. The features' own state for the
/// project going away is reset first, by whoever holds them.
pub(crate) fn mark_project_closed(state: &mut dyn LifecycleSourceMut) {
    let state = &mut state.lifecycle_parts();
    state.platform.native_project_binding_receipt = None;
    state.platform.browser_project_binding_receipt = None;
    #[cfg(target_arch = "wasm32")]
    {
        clear_browser_handles();
    }
    let mut authority = std::mem::take(&mut state.platform.project_lifecycle.authority);
    authority.close_project();
    *state.platform.project_lifecycle = ProjectLifecycleState {
        authority,
        ..ProjectLifecycleState::default()
    };
    state
        .platform
        .project_lifecycle
        .publish_open(&mut state.model.project_open);
}

#[cfg(target_arch = "wasm32")]
fn release_replaced_browser_bindings(
    lifecycle: &ProjectLifecycleState,
    retained: Option<&PersistenceBinding>,
) {
    let retained = browser_binding_handle_id(retained);
    let accepted = lifecycle
        .binding()
        .and_then(|binding| browser_binding_handle_id(Some(binding)));
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
pub(crate) fn browser_operation_context(
    model: &Model,
    platform: &PlatformState,
) -> BrowserOperationContext {
    platform
        .project_lifecycle
        .authority
        .browser_operation_context(
            &model.workspace.content.project.id().to_string(),
            platform.browser_project_binding_receipt.as_ref(),
        )
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_operation_context_is_current(
    model: &Model,
    platform: &PlatformState,
    context: &BrowserOperationContext,
) -> bool {
    platform
        .project_lifecycle
        .authority
        .browser_operation_context_is_current(
            context,
            &model.workspace.content.project.id().to_string(),
            platform.browser_project_binding_receipt.as_ref(),
        )
}

/// Relinquish app-side authority for a browser promise that the platform may
/// not be able to abort. The operation generation makes every eventual late
/// completion stale, while accepted project/file authority remains intact.
#[cfg(target_arch = "wasm32")]
pub(crate) fn cancel_pending_browser_operation(platform: &mut PlatformState) -> bool {
    let Some(restore_was_pending) = platform
        .project_lifecycle
        .authority
        .cancel_browser_operation()
    else {
        return false;
    };
    if restore_was_pending {
        // A cancelled restore established no restart authority.
        platform.browser_project_binding_receipt = None;
    }
    true
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn canonical_native_path(platform: &PlatformState) -> Option<PathBuf> {
    platform
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
    state: &mut dyn LifecycleHost,
    requested_scope: SaveScope,
    path: &Path,
    authority: DestinationAuthority,
) -> Result<(), ProjectLifecycleError> {
    let (destination, scope) = {
        let state = &mut state.lifecycle_parts();
        require_open_project(state.platform)?;
        require_project_writable(state.model)?;
        let destination = NativeSaveDestination::new(
            &persistence::NativeStorage,
            path,
            authority,
            state.platform.project_lifecycle.binding(),
            state
                .platform
                .project_lifecycle
                .unreadable_native_binding
                .as_ref(),
        )?;
        let scope = effective_save_scope(state.platform, requested_scope);
        state.platform.project_lifecycle.authority.begin_save()?;
        (destination, scope)
    };

    let result = (|| {
        let working = snapshot(state)?;
        let candidate = match scope {
            SaveScope::AllDocuments => working,
            SaveScope::ActiveDocument => state
                .lifecycle_view()
                .platform
                .project_lifecycle
                .accepted_document_candidate(&working, &active_document(state))?,
        };
        let ProjectSnapshot {
            file,
            workspace_session,
        } = candidate;
        let published = destination.publish(&persistence::NativeStorage, file, |candidate| {
            prepare_post_save_registry(state, candidate, scope, SnapshotContent::Current)
        })?;
        let candidate = ProjectSnapshot {
            file: published.candidate,
            workspace_session,
        };
        adopt_successful_save(
            state,
            candidate,
            published.binding,
            scope,
            published.registry,
        );
        Ok(())
    })();
    state
        .lifecycle_parts()
        .platform
        .project_lifecycle
        .authority
        .cancel_transaction();
    result
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn save_project_copy_native(
    state: &mut dyn LifecycleSourceMut,
    path: &Path,
) -> Result<(), ProjectLifecycleError> {
    let state = &mut state.lifecycle_parts();
    require_open_project(state.platform)?;
    require_project_writable(state.model)?;
    let destination = NativeCopyDestination::new(
        &persistence::NativeStorage,
        path,
        state.platform.project_lifecycle.canonical_native_path(),
        state
            .platform
            .project_lifecycle
            .unreadable_native_binding
            .as_ref(),
    )?;
    state.platform.project_lifecycle.authority.begin_save()?;
    let result = (|| {
        let copy = snapshot(state)?;
        destination.publish(&persistence::NativeStorage, copy.file)
    })();
    state
        .platform
        .project_lifecycle
        .authority
        .cancel_transaction();
    result
}

#[cfg(target_arch = "wasm32")]
pub(crate) struct BrowserPreparedSave {
    pub(crate) save: PreparedBrowserSave,
    workspace_session: crate::state::workspace::WorkspaceSession,
    pub(crate) suggested_name: String,
    pub(crate) handle_id: Option<u64>,
    pub(crate) source_handle_id: Option<u64>,
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn prepare_browser_save(
    state: &mut dyn LifecycleSourceMut,
    requested_scope: SaveScope,
    project_copy: bool,
    suggested_name: String,
) -> Result<BrowserPreparedSave, ProjectLifecycleError> {
    let state = &mut state.lifecycle_parts();
    require_open_project(state.platform)?;
    require_project_writable(state.model)?;
    state
        .platform
        .project_lifecycle
        .authority
        .require_browser_binding_ready()?;
    if !project_copy
        && let Some(conflict) = state.platform.project_lifecycle.browser_conflict.as_ref()
    {
        let _ = conflict.observed_digest;
        return Err(ProjectLifecycleError::BrowserExternalChange);
    }
    let scope = effective_save_scope(state.platform, requested_scope);
    let transaction = state.platform.project_lifecycle.authority.begin_save()?;
    let context = browser_operation_context(state.model, state.platform);
    let saved_document = active_document(state);
    let result = (|| {
        let working = snapshot(state)?;
        let candidate = if project_copy || scope == SaveScope::AllDocuments {
            working
        } else {
            state
                .platform
                .project_lifecycle
                .accepted_document_candidate(&working, &saved_document)?
        };
        let ProjectSnapshot {
            file,
            workspace_session,
        } = candidate;
        let staged = StagedBrowserSave::new(
            transaction,
            context,
            file,
            scope,
            saved_document,
            project_copy,
            &suggested_name,
        )?;
        let existing_binding = (!project_copy)
            .then(|| {
                state.platform.project_lifecycle.binding().or(state
                    .platform
                    .project_lifecycle
                    .browser_reconnect_binding
                    .as_ref())
            })
            .flatten();
        let source_handle_id = state
            .platform
            .project_lifecycle
            .binding()
            .or(state
                .platform
                .project_lifecycle
                .browser_reconnect_binding
                .as_ref())
            .or(state
                .platform
                .project_lifecycle
                .browser_conflict
                .as_ref()
                .map(|conflict| &conflict.binding))
            .map(|binding| match binding {
                PersistenceBinding::Browser { handle_id, .. } => *handle_id,
            });
        let handle_id = existing_binding.map(|binding| match binding {
            PersistenceBinding::Browser { handle_id, .. } => *handle_id,
        });
        let binding = existing_binding.map(|binding| match binding {
            PersistenceBinding::Browser { binding, .. } => binding,
        });
        let save = staged.bind(binding, || {
            if project_copy || persistence::browser_external_canonical_supported() {
                BrowserBindingBackend::ExternalFile
            } else if persistence::browser_opfs_supported() {
                BrowserBindingBackend::Opfs
            } else {
                BrowserBindingBackend::ExternalFile
            }
        })?;
        Ok(BrowserPreparedSave {
            save,
            workspace_session,
            suggested_name,
            handle_id,
            source_handle_id,
        })
    })();
    if result.is_err() {
        state
            .platform
            .project_lifecycle
            .authority
            .cancel_transaction();
    }
    result
}

/// Decide whether a successfully persisted snapshot still authorizes the
/// destructive action that requested it. Saving and continuing are separate:
/// edits made while a browser surface is pending remain dirty and force a new
/// review instead of being discarded.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn saved_snapshot_authorizes_continuation(
    state: &dyn LifecycleSource,
    scope: SaveScope,
    saved_document: &ProjectDocumentId,
) -> bool {
    scope.authorizes_continuation(
        saved_document,
        || active_document(state),
        || has_unsaved_changes(state),
        || active_document_is_dirty(state),
    )
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
pub(crate) fn release_browser_handle_if_unowned(platform: &PlatformState, handle_id: u64) {
    if !browser_handle_is_current(platform, handle_id) {
        persistence::release_browser_handle(handle_id);
    }
}

#[cfg(target_arch = "wasm32")]
fn clear_browser_handles() {
    persistence::clear_browser_handles();
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn complete_browser_save(
    state: &mut dyn LifecycleHost,
    prepared: BrowserPreparedSave,
    handle_id: u64,
    publication: rspice_project::lifecycle::BrowserSavePublication,
) -> Result<(), ProjectLifecycleError> {
    let BrowserPreparedSave {
        save,
        workspace_session,
        ..
    } = prepared;
    let completion = {
        let state = &mut state.lifecycle_parts();
        state
            .platform
            .project_lifecycle
            .authority
            .complete_browser_save(
                save,
                &state.model.workspace.content.project.id().to_string(),
                state.platform.browser_project_binding_receipt.as_ref(),
                publication,
                || persistence::release_browser_handle(handle_id),
            )?
    };
    if let Some(completion) = completion {
        let candidate = ProjectSnapshot {
            file: completion.candidate,
            workspace_session,
        };
        let binding = PersistenceBinding::Browser {
            handle_id,
            binding: completion.binding,
        };
        finish_successful_save(state, candidate, binding, completion.scope);
        state
            .lifecycle_parts()
            .platform
            .project_lifecycle
            .authority
            .cancel_transaction();
    }
    Ok(())
}

#[cfg(any(test, target_arch = "wasm32"))]
fn finish_successful_save(
    state: &mut dyn LifecycleHost,
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
        &candidate.file,
        scope,
        SnapshotContent::Current,
    ) {
        Ok(registry) => registry,
        Err(error) => {
            let state = &mut state.lifecycle_parts();
            let registry = state
                .platform
                .project_lifecycle
                .authority
                .unverified_registry();
            state.model.record(
                LogSeverity::Warning,
                LogSource::Studio,
                format!(
                    "The project was saved, but the current draft could not be compared with it: {error}. Current edits remain unsaved; resolve the draft error before saving or closing."
                ),
            );
            registry
        }
    };
    adopt_successful_save(state, candidate, binding, scope, post_save_registry);
}

fn prepare_post_save_registry(
    state: &dyn LifecycleSource,
    candidate: &rspice_project::ProjectFile,
    scope: SaveScope,
    content: SnapshotContent,
) -> Result<registry::DocumentRegistry, ProjectLifecycleError> {
    let current = capture_snapshot(state, content)?;
    state
        .lifecycle_view()
        .platform
        .project_lifecycle
        .authority
        .prepare_post_save_registry(current.file, candidate, scope, || active_document(state))
}

fn rebase_pending_operation_dirty_state(
    state: &mut LifecycleParts<'_>,
    candidate: &ProjectSnapshot,
    scope: SaveScope,
) {
    if !state.model.schematic.has_pending_operation()
        && !state
            .model
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
        prepare_post_save_registry(state, &candidate.file, scope, SnapshotContent::Committed);
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
    if state.model.schematic.has_pending_operation() {
        let was_dirty = dirty(
            &state
                .model
                .workspace
                .content
                .active_schematic_reference()
                .key(),
        );
        state.model.schematic.set_pending_was_dirty(was_dirty);
    }
    state
        .model
        .workspace
        .for_each_schematic_editor_mut(|key, schematic| {
            if schematic.has_pending_operation() {
                schematic.set_pending_was_dirty(dirty(key));
            }
        });
}

fn adopt_successful_save(
    host: &mut dyn LifecycleHost,
    candidate: ProjectSnapshot,
    binding: PersistenceBinding,
    scope: SaveScope,
    post_save_registry: registry::DocumentRegistry,
) {
    let state = &mut host.lifecycle_parts();
    rebase_pending_operation_dirty_state(state, &candidate, scope);
    #[cfg(not(target_arch = "wasm32"))]
    let native_receipt = binding.native_receipt(&candidate.file.workspace.project.id().to_string());
    #[cfg(target_arch = "wasm32")]
    let browser_receipt = binding.durable_browser_receipt();
    #[cfg(target_arch = "wasm32")]
    release_replaced_browser_bindings(&state.platform.project_lifecycle, Some(&binding));
    #[cfg(not(target_arch = "wasm32"))]
    {
        if scope == SaveScope::AllDocuments
            || active_document(state) == ProjectDocumentId::ProjectConfiguration
        {
            state
                .model
                .retain_annotation_history_after_save_descriptor(&candidate.file.workspace.project);
            state.model.workspace.content.project = candidate.file.workspace.project.clone();
        } else {
            // Saving one non-configuration document intentionally publishes a
            // candidate built on the accepted project descriptor. Preserve a
            // concurrent/unrelated live descriptor draft and update only the
            // canonical pathname established by this successful save.
            state.model.workspace.content.project.path =
                candidate.file.workspace.project.path.clone();
        }
    }
    state
        .platform
        .project_lifecycle
        .accept_project(candidate, Some(binding));
    #[cfg(not(target_arch = "wasm32"))]
    {
        state.platform.native_project_binding_receipt = Some(native_receipt);
        state.platform.browser_project_binding_receipt = None;
    }
    #[cfg(target_arch = "wasm32")]
    {
        state.platform.native_project_binding_receipt = None;
        state.platform.browser_project_binding_receipt = browser_receipt;
        state.platform.project_lifecycle.browser_reconnect_binding = None;
        state.platform.project_lifecycle.browser_conflict = None;
    }
    #[cfg(not(target_arch = "wasm32"))]
    match scope {
        SaveScope::AllDocuments => {
            state.model.workspace.mark_all_clean();
            state.model.schematic.session.is_dirty = false;
            mark_all_library_views_clean(&mut state.model.library_manager);
        }
        SaveScope::ActiveDocument => mark_active_document_clean(state),
    }
    #[cfg(target_arch = "wasm32")]
    let _ = scope;
    // Native publication requires a fully built comparison. A browser write
    // may finish while the newer draft is invalid; its registry then marks
    // comparisons unverified. Neither case can erase pending authored edits.
    state
        .platform
        .project_lifecycle
        .authority
        .adopt_registry(post_save_registry);
    apply_registry_dirty_flags(state);
    report_design_checks_after_save(host);
}

/// Report what the design checks find, when a completed save was asked to run
/// them.
///
/// The order is the contract. Publication has already happened above, so a
/// finding raised here cannot refuse the save: a check is advice about the
/// drawing, and advice that could veto a save the author explicitly asked for
/// would be a gate wearing a preference's clothes.
///
/// The findings themselves are canonical records, published by the checks
/// this runs: the Problems page lists them, and the canvas badges, the check
/// pill and the finding tour read them. The Console gets one line, because a
/// check having run is a session event. It does not copy a finding as well —
/// a finding restated in the log stands on the page a second time.
fn report_design_checks_after_save(state: &mut dyn LifecycleHost) {
    if !state
        .lifecycle_view()
        .model
        .preferences
        .toggle(crate::workbench::TogglePreference::RunDesignChecksOnSave)
    {
        return;
    }
    let result = match state.run_active_design_checks() {
        Ok(result) => result,
        Err(error) => {
            state.lifecycle_parts().model.record(
                LogSeverity::Warning,
                LogSource::Studio,
                format!("Design checks on save could not run: {error}"),
            );
            return;
        }
    };
    let summary = result.summary();
    let counts = format!(
        "Design checks on save: {} critical, {} errors, {} warnings.",
        summary.critical, summary.errors, summary.warnings
    );
    let line = if result.passed() {
        counts
    } else {
        format!("{counts} See Problems.")
    };
    // The rule Check Design states its outcome by, so a check reads the same
    // whether the reader ran it or a save did.
    state
        .lifecycle_parts()
        .model
        .record_check_outcome(&result, line);
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn record_browser_save_conflict(
    model: &Model,
    platform: &mut PlatformState,
    prepared: &BrowserPreparedSave,
    observed_digest: ContentDigest,
) {
    if !browser_operation_context_is_current(model, platform, prepared.save.context()) {
        return;
    }
    let binding = platform
        .project_lifecycle
        .binding()
        .cloned()
        .or_else(|| platform.project_lifecycle.browser_reconnect_binding.clone());
    if let Some(binding) = binding {
        platform.project_lifecycle.browser_conflict = Some(BrowserConflict {
            binding,
            observed_digest,
        });
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn complete_browser_binding_promotion(
    model: &Model,
    platform: &mut PlatformState,
    context: &BrowserOperationContext,
    handle_id: u64,
    result: &Result<(), String>,
) -> bool {
    if !browser_operation_context_is_current(model, platform, context) {
        if !browser_handle_is_current(platform, handle_id) {
            persistence::release_browser_handle(handle_id);
        }
        return false;
    }
    platform
        .project_lifecycle
        .authority
        .finish_browser_promotion();
    if result.is_ok()
        && let Some(PersistenceBinding::Browser { binding, .. }) =
            platform.project_lifecycle.accepted_binding.as_mut()
    {
        binding.persisted_generation = Some(binding.receipt.accepted_generation);
    }
    platform.browser_project_binding_receipt = platform
        .project_lifecycle
        .binding()
        .and_then(PersistenceBinding::durable_browser_receipt);
    true
}

#[cfg(target_arch = "wasm32")]
fn browser_handle_is_current(platform: &PlatformState, handle_id: u64) -> bool {
    let lifecycle = &platform.project_lifecycle;
    lifecycle
        .binding()
        .and_then(|binding| browser_binding_handle_id(Some(binding)))
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
pub(crate) fn begin_browser_binding_promotion(
    model: &Model,
    platform: &mut PlatformState,
) -> BrowserOperationContext {
    platform
        .project_lifecycle
        .authority
        .begin_browser_promotion();
    browser_operation_context(model, platform)
}

#[cfg(not(target_arch = "wasm32"))]
fn mark_active_document_clean(state: &mut LifecycleParts<'_>) {
    match active_document(state) {
        ProjectDocumentId::CellView(reference) => {
            if let Some(mut buffer) = state.model.workspace.schematic_editor_mut(&reference.key()) {
                buffer.editor.session.is_dirty = false;
            }
            if reference == state.model.workspace.content.active_view {
                state.model.schematic.session.is_dirty = false;
            }
            if let Some(open) = state
                .model
                .workspace
                .content
                .open_views
                .iter_mut()
                .find(|open| open.reference == reference)
            {
                open.dirty = false;
            }
            let _ = state.model.library_manager.mark_view_clean_runtime(
                &reference.library,
                &reference.cell,
                &reference.view,
            );
        }
        ProjectDocumentId::NetlistSource => {
            state.model.workspace.content.netlist_source_dirty = false;
            state.model.workspace.content.project_sources_dirty = false;
        }
        _ => {}
    }
}

pub(crate) fn prepare_revert_active_document(
    state: &dyn LifecycleSource,
) -> Result<RevertReviewToken, ProjectLifecycleError> {
    let state = &state.lifecycle_view();
    require_open_project(state.platform)?;
    let id = active_document(state);
    if state.model.simulation.execution.has_active_execution()
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
        .platform
        .project_lifecycle
        .authority
        .prepare_revert(id)
}

/// Check a confirmed revert review against the project as it stands: the
/// review must name the active document, and still be the review the project
/// would give now.
pub(crate) fn validate_revert_review(
    state: &dyn LifecycleSource,
    token: &RevertReviewToken,
) -> Result<(), ProjectLifecycleError> {
    let view = &state.lifecycle_view();
    require_open_project(view.platform)?;
    view.platform
        .project_lifecycle
        .authority
        .validate_revert(token, &active_document(view))?;
    let current = prepare_revert_active_document(state)?;
    if current != *token {
        return Err(ProjectLifecycleError::RevertReviewStale);
    }
    Ok(())
}

/// What reverting one document leaves to the features above the lifecycle,
/// to be done before the reverted project is checked and committed.
pub(crate) enum RevertFollowUp {
    /// Nothing above the model holds the reverted document.
    None,
    /// The accepted configuration's libraries replace the project structure;
    /// [`replace_project_structure`] and [`retain_project_view_sources`] do the
    /// model's part around the views it prunes.
    ProjectStructure(crate::state::LibraryManager),
    /// One cell view goes back to the accepted one; [`revert_cell_view`] does
    /// the model's part.
    CellView(Box<ProjectSnapshot>, CellViewRef),
    /// The result history is the accepted one; its saved presentation goes
    /// back to the results, and their derived viewers are cleared.
    ResultHistory(crate::state::result_presentation::ResultPresentation),
    /// The stimulus library is the accepted one.
    StimulusLibrary,
    /// The netlist source is the accepted one; [`finish_netlist_source_revert`]
    /// does the rest of the model's part after the features reset.
    NetlistSource,
}

/// Put the model's part of one document back to the accepted project.
pub(crate) fn revert_document_model(
    state: &mut LifecycleParts<'_>,
    id: ProjectDocumentId,
) -> Result<RevertFollowUp, ProjectLifecycleError> {
    let baseline = state.platform.project_lifecycle.accepted_snapshot()?;
    let baseline_project_id = baseline.file.workspace.project.id();

    let follow_up = match id {
        ProjectDocumentId::ProjectConfiguration => {
            let callback_receipts = baseline.file.workspace.pdk_callback_receipts().to_vec();
            state.model.workspace.content.project = baseline.file.workspace.project;
            state.model.workspace.content.configuration_sets =
                baseline.file.workspace.configuration_sets;
            state.model.workspace.content.design_management =
                baseline.file.workspace.design_management;
            state
                .model
                .workspace
                .content
                .replace_pdk_callback_receipts_for_lifecycle(callback_receipts)
                .map_err(ProjectLifecycleError::InvalidState)?;
            RevertFollowUp::ProjectStructure(baseline.file.libraries)
        }
        ProjectDocumentId::CellView(reference) => {
            RevertFollowUp::CellView(Box::new(baseline), reference)
        }
        ProjectDocumentId::SimulationPlan => {
            let context = baseline.file.execution_context.ok_or_else(|| {
                ProjectLifecycleError::InvalidState(
                    "accepted project has no simulation plan".to_owned(),
                )
            })?;
            state.model.sim_setup = SimSetupState {
                setup: context.simulation_plan,
                session: Default::default(),
            };
            state.model.sim_setup.prepare_after_restore();
            state.model.workspace.content.simulation_plan_payloads =
                baseline.file.workspace.simulation_plan_payloads;
            if let Some(plan_id) = state
                .model
                .sim_setup
                .analysis_plan
                .as_ref()
                .map(crate::simulation::plan::SimulationPlan::id)
            {
                state
                    .model
                    .workspace
                    .content
                    .sync_legacy_specs_projection(plan_id);
            }
            RevertFollowUp::None
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
            state.model.model_library_manager = manager;
            for warning in warnings {
                state
                    .model
                    .record(LogSeverity::Warning, LogSource::Studio, warning);
            }
            RevertFollowUp::None
        }
        ProjectDocumentId::ResultHistory => {
            let mut simulation = crate::state::SimulationState::default();
            crate::io::restore_simulation_results(
                baseline.file.simulation_results,
                &mut simulation,
            )
            .map_err(ProjectLifecycleError::InvalidState)?;
            state.model.simulation = simulation;
            state.model.workspace.content.report_documents =
                baseline.file.workspace.report_documents;
            state.model.workspace.content.report_documents_dirty = false;
            state.model.workspace.content.visualization_documents =
                baseline.file.workspace.visualization_documents;
            state.model.workspace.content.visualization_documents_dirty = false;
            RevertFollowUp::ResultHistory(baseline.file.result_presentation)
        }
        ProjectDocumentId::VerificationSpecifications => {
            state.model.workspace.content.specs = baseline.file.workspace.specs;
            RevertFollowUp::None
        }
        ProjectDocumentId::StimulusLibrary => {
            state.model.workspace.content.stimulus_library =
                baseline.file.workspace.stimulus_library;
            RevertFollowUp::StimulusLibrary
        }
        ProjectDocumentId::NetlistSource => {
            state.model.workspace.content.netlist_source = baseline.file.workspace.netlist_source;
            state.model.workspace.content.netlist_source_path =
                baseline.file.workspace.netlist_source_path;
            state.model.workspace.content.netlist_document =
                baseline.file.workspace.netlist_document;
            state.model.workspace.content.netlist_descriptor =
                baseline.file.workspace.netlist_descriptor;
            state.model.workspace.content.retained_netlist_decks =
                baseline.file.workspace.retained_netlist_decks;
            state
                .model
                .workspace
                .content
                .project_sources
                .synchronize_code_workspace_bundles_from(&baseline.file.workspace.project_sources)
                .map_err(|error| ProjectLifecycleError::InvalidState(error.to_string()))?;
            state.model.workspace.content.netlist_source_dirty = false;
            state.model.workspace.content.project_sources_dirty = false;
            RevertFollowUp::NetlistSource
        }
    };
    Ok(follow_up)
}

/// The model's part of a netlist-source revert that follows the features'
/// reset: the deck the run reads is the reverted source, no run is pending
/// on the old one, and execution sees a new design.
pub(crate) fn finish_netlist_source_revert(model: &mut Model) {
    model.simulation.source.netlist_content = model
        .workspace
        .content
        .netlist_source
        .clone()
        .unwrap_or_default();
    model.simulation.execution.trigger_simulation = false;
    model.design_execution_epoch = model.design_execution_epoch.wrapping_add(1);
}

/// The project documents with unsaved changes, in registry order.
///
/// A project with no accepted baseline, or one whose comparison fails, has
/// nothing to diff against: its configuration stands for the whole unsaved
/// project, as one document.
pub(crate) fn dirty_documents(state: &dyn LifecycleSource) -> Vec<ProjectDocumentId> {
    let state = &state.lifecycle_view();
    state
        .platform
        .project_lifecycle
        .authority
        .dirty_documents(|| {
            capture_snapshot(state, SnapshotContent::Current).map(|snapshot| snapshot.file)
        })
}

pub(crate) fn dirty_document_count(state: &dyn LifecycleSource) -> usize {
    dirty_documents(state).len()
}

pub(crate) fn can_close_active_document(state: &dyn LifecycleSource) -> bool {
    let state = state.lifecycle_view();
    state.platform.project_lifecycle.is_open()
        && state.workspace == crate::workbench::state::Workspace::Design
        && state.model.workspace.content.open_views.len() > 1
}

pub(crate) fn begin_project_replacement(
    state: &mut dyn LifecycleSourceMut,
) -> Result<TransactionId, ProjectLifecycleError> {
    let state = &mut state.lifecycle_parts();
    if state.model.simulation.execution.has_active_execution() {
        return Err(ProjectLifecycleError::ActiveRun);
    }
    state
        .platform
        .project_lifecycle
        .authority
        .ensure_replacement_available()?;
    let content =
        registry::content_digest(&capture_snapshot(state, SnapshotContent::Current)?.file)
            .map_err(ProjectLifecycleError::InvalidState)?;
    state
        .platform
        .project_lifecycle
        .authority
        .begin_replacement(content)
}

pub(crate) fn cancel_transaction(platform: &mut PlatformState) {
    platform.project_lifecycle.authority.cancel_transaction();
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn cancel_transaction_if(platform: &mut PlatformState, id: TransactionId) -> bool {
    platform
        .project_lifecycle
        .authority
        .cancel_transaction_if(id)
}

pub(crate) fn validate_project_replacement(
    state: &dyn LifecycleSource,
    id: TransactionId,
) -> Result<(), ProjectLifecycleError> {
    let state = &state.lifecycle_view();
    if state.model.simulation.execution.has_active_execution() {
        return Err(ProjectLifecycleError::ActiveRun);
    }
    state
        .platform
        .project_lifecycle
        .authority
        .validate_replacement(id, || {
            registry::content_digest(&capture_snapshot(state, SnapshotContent::Current)?.file)
                .map_err(ProjectLifecycleError::InvalidState)
        })
}

pub(crate) fn require_open_project(platform: &PlatformState) -> Result<(), ProjectLifecycleError> {
    platform.project_lifecycle.authority.require_open_project()
}

fn require_project_writable(model: &Model) -> Result<(), ProjectLifecycleError> {
    (!model.safe_mode.project_read_only())
        .then_some(())
        .ok_or(ProjectLifecycleError::SafeModeReadOnly)
}

/// How the model's part of a cell-view revert left the view.
pub(crate) enum CellViewRevert {
    /// The accepted project has no such view, so it was removed; the
    /// workspace still names it until it is pruned.
    Removed {
        /// The view's sources changed, so their compiled form is stale.
        source_changed: bool,
    },
    /// The accepted view is back in the library; its sources follow once the
    /// active schematic is restored, by [`revert_cell_view_sources`].
    Restored {
        /// The reverted view is the active one.
        active: bool,
    },
}

/// Put one cell view's design back to the accepted project.
pub(crate) fn revert_cell_view(
    model: &mut Model,
    baseline: &ProjectSnapshot,
    reference: &CellViewRef,
) -> Result<CellViewRevert, ProjectLifecycleError> {
    let baseline_view = baseline
        .file
        .libraries
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .cloned();
    let Some(baseline_view) = baseline_view else {
        let removed = model
            .library_manager
            .edit_library(&reference.library)
            .and_then(|mut library| library.remove_view(&reference.cell, &reference.view))
            .unwrap_or(false);
        if !removed {
            return Err(ProjectLifecycleError::NoAcceptedBaseline);
        }
        let source_changed = revert_cell_view_sources(model, baseline, reference)?;
        return Ok(CellViewRevert::Removed { source_changed });
    };
    model
        .library_manager
        .edit_library(&reference.library)
        .and_then(|mut library| library.add_view(&reference.cell, baseline_view))
        .ok_or_else(|| {
            ProjectLifecycleError::InvalidState("active cell no longer exists".to_owned())
        })?;
    let key = reference.key();
    match baseline.clone_schematic_editor(&key) {
        Some(buffer) => {
            model.workspace.insert_schematic_editor(key, buffer);
        }
        None => {
            model.workspace.remove_schematic_editor(&key);
        }
    }
    model
        .workspace
        .content
        .synchronize_physical_layout_document_from(reference, &baseline.file.workspace)
        .map_err(|error| ProjectLifecycleError::InvalidState(error.to_string()))?;
    Ok(CellViewRevert::Restored {
        active: reference == &model.workspace.content.active_view,
    })
}

/// Put one cell view's source bundle back to the accepted project, and say
/// whether that changed it.
pub(crate) fn revert_cell_view_sources(
    model: &mut Model,
    baseline: &ProjectSnapshot,
    reference: &CellViewRef,
) -> Result<bool, ProjectLifecycleError> {
    model
        .workspace
        .content
        .project_sources
        .synchronize_cell_view_bundle_from(reference, &baseline.file.workspace.project_sources)
        .map_err(|error| ProjectLifecycleError::InvalidState(error.to_string()))
}

/// Replace the project's library structure with the accepted one, keeping
/// each retained view's document content, and return the views it removed:
/// the workspace still names them until they are pruned.
pub(crate) fn replace_project_structure(
    model: &mut Model,
    baseline: crate::state::LibraryManager,
) -> Vec<CellViewRef> {
    let previous_references = library_view_references(&model.library_manager);
    model.library_manager = baseline.with_document_content_from(&model.library_manager);
    previous_references
        .into_iter()
        .filter(|reference| {
            model
                .library_manager
                .get_library(&reference.library)
                .and_then(|library| library.get_cell(&reference.cell))
                .and_then(|cell| cell.get_view(&reference.view))
                .is_none()
        })
        .collect()
}

/// Keep only the source bundles of views the project still holds, and return
/// the bundles dropped.
pub(crate) fn retain_project_view_sources(model: &mut Model) -> Vec<crate::state::ProjectSourceId> {
    let retained = library_view_references(&model.library_manager);
    model
        .workspace
        .content
        .project_sources
        .retain_cell_view_bundles_for(retained)
}

fn library_view_references(libraries: &crate::state::LibraryManager) -> Vec<CellViewRef> {
    libraries
        .libraries_by_key()
        .flat_map(|(library_key, library)| {
            library.cells.iter().flat_map(move |(cell_key, cell)| {
                cell.views
                    .keys()
                    .map(move |view_key| CellViewRef::new(library_key, cell_key, view_key))
            })
        })
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn mark_all_library_views_clean(libraries: &mut crate::state::LibraryManager) {
    libraries.mark_all_views_clean_runtime();
}

#[cfg(test)]
mod tests;
