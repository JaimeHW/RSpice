use std::path::Path;

use crate::common::app::{AppState, ConsoleMessage};
#[cfg(not(target_arch = "wasm32"))]
use crate::common::project_lifecycle::DestinationAuthority;
use crate::common::project_lifecycle::{PersistenceBinding, ProjectLifecycleError, SaveScope};
use crate::io::ProjectFile;
#[cfg(not(target_arch = "wasm32"))]
use crate::io::ProjectIoError;
use crate::workbench::state::ProjectCloseDestination;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ProjectLoadOrigin<'a> {
    #[cfg(not(target_arch = "wasm32"))]
    PersistentPath(&'a Path),
    #[cfg(any(test, target_arch = "wasm32"))]
    BrowserImport(&'a str),
    #[cfg(any(test, target_arch = "wasm32"))]
    BrowserCanonical(&'a str),
}

impl<'a> ProjectLoadOrigin<'a> {
    fn display_label(self) -> String {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::PersistentPath(path) => path.display().to_string(),
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::BrowserImport(name) | Self::BrowserCanonical(name) => name.to_string(),
        }
    }

    fn recent_path(self) -> Option<&'a Path> {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::PersistentPath(path) => Some(path),
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::BrowserImport(_) | Self::BrowserCanonical(_) => None,
        }
    }

    fn success_prefix(self) -> &'static str {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::PersistentPath(_) => "Opened project",
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::BrowserImport(_) => "Imported project",
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::BrowserCanonical(_) => "Opened project",
        }
    }
}

pub(crate) fn create_new_project(state: &mut AppState) {
    if state.simulation.is_running {
        lifecycle_error(
            state,
            ProjectLifecycleError::ActiveRun,
            "New project blocked",
        );
        return;
    }
    let mut library_manager = crate::state::LibraryManager::with_primitives();
    let mut workspace =
        crate::state::ProjectWorkspace::new_empty_bootstrapped(&mut library_manager);
    let schematic = state.new_schematic_document();
    workspace.save_active_schematic(&schematic);

    state.library_manager = library_manager;
    state.workspace = workspace;
    state.schematic = schematic;
    state.clear_design_execution_context();
    state.sim_setup =
        crate::common::app::SimSetupState::new_with_user_preferences(&state.ui.preferences);
    state.model_library_manager = crate::common::app::default_model_library_manager();
    state.browser_project_save_name = None;
    crate::common::project_lifecycle::reset_for_new_project(state);
    state.push_user_message(ConsoleMessage::info("Created new project"));
}

#[cfg(all(not(target_arch = "wasm32"), test))]
pub(crate) fn save_project_to_path(state: &mut AppState, path: &Path) -> bool {
    save_native_scope(
        state,
        SaveScope::AllDocuments,
        path,
        DestinationAuthority::UserSelected,
    )
}

fn file_name_string(path: &Path) -> Option<String> {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .filter(|name| !name.trim().is_empty())
}

fn project_save_dialog_default_name(state: &AppState) -> String {
    state
        .workspace
        .project
        .path
        .as_deref()
        .and_then(file_name_string)
        .or_else(|| state.browser_project_save_name.clone())
        .unwrap_or_else(|| "untitled.rspiceproj".to_string())
}

pub(crate) fn save_project(state: &mut AppState) -> bool {
    save_scope_outcome(state, SaveScope::ActiveDocument).request_started()
}

pub(crate) fn save_all(state: &mut AppState) -> bool {
    save_scope_outcome(state, SaveScope::AllDocuments).request_started()
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SaveRequestOutcome {
    CanonicalComplete,
    CanonicalPending(crate::common::project_lifecycle::TransactionId),
    CopyOnly,
    CopyPending,
    CancelledOrFailed,
}

impl SaveRequestOutcome {
    fn request_started(self) -> bool {
        !matches!(self, Self::CancelledOrFailed)
    }

    pub(crate) fn authorizes_immediate_destructive_action(self) -> bool {
        matches!(self, Self::CanonicalComplete)
    }
}

pub(crate) fn save_all_for_continuation(state: &mut AppState) -> SaveRequestOutcome {
    save_scope_outcome(state, SaveScope::AllDocuments)
}

pub(crate) fn save_active_for_continuation(state: &mut AppState) -> SaveRequestOutcome {
    save_scope_outcome(state, SaveScope::ActiveDocument)
}

#[cfg(not(target_arch = "wasm32"))]
fn save_scope_outcome(state: &mut AppState, scope: SaveScope) -> SaveRequestOutcome {
    if let Some(path) = crate::common::project_lifecycle::canonical_native_path(state) {
        return if save_native_scope(state, scope, &path, DestinationAuthority::Canonical) {
            SaveRequestOutcome::CanonicalComplete
        } else {
            SaveRequestOutcome::CancelledOrFailed
        };
    }
    let default_name = project_save_dialog_default_name(state);
    match crate::io::show_save_project_dialog(Some(&default_name)) {
        Ok(path) => {
            if save_native_scope(state, scope, &path, DestinationAuthority::UserSelected) {
                SaveRequestOutcome::CanonicalComplete
            } else {
                SaveRequestOutcome::CancelledOrFailed
            }
        }
        Err(ProjectIoError::Cancelled) => SaveRequestOutcome::CancelledOrFailed,
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Project save failed: {error}"
            )));
            SaveRequestOutcome::CancelledOrFailed
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_native_scope(
    state: &mut AppState,
    scope: SaveScope,
    path: &Path,
    authority: DestinationAuthority,
) -> bool {
    match crate::common::project_lifecycle::save_native(state, scope, path, authority) {
        Ok(()) => {
            let canonical = crate::common::project_lifecycle::canonical_native_path(state)
                .unwrap_or_else(|| path.to_path_buf());
            state.browser_project_save_name = None;
            state.remember_recent_file(crate::common::app::RecentKind::Project, &canonical);
            state.push_user_message(ConsoleMessage::info(format!(
                "Saved project: {}",
                canonical.display()
            )));
            true
        }
        Err(error) => {
            lifecycle_error(state, error, "Project save failed");
            false
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn save_scope_outcome(state: &mut AppState, scope: SaveScope) -> SaveRequestOutcome {
    save_project_in_browser(state, scope)
}

#[cfg(target_arch = "wasm32")]
fn save_project_in_browser(state: &mut AppState, scope: SaveScope) -> SaveRequestOutcome {
    start_browser_project_save(state, scope, false)
}

#[cfg(target_arch = "wasm32")]
fn save_project_copy_in_browser(state: &mut AppState) -> bool {
    start_browser_project_save(state, SaveScope::AllDocuments, true).request_started()
}

#[cfg(any(test, target_arch = "wasm32"))]
const fn browser_save_picker_is_safe(
    project_copy: bool,
    source_handle_id: Option<u64>,
    picker_supported: bool,
) -> bool {
    // A save picker is allowed to create or truncate the selected entry before
    // returning its handle. Consequently, `isSameEntry` is only a diagnostic
    // after selection; it cannot protect the active canonical file from a
    // project-copy picker. Use the copy-only download path whenever a live
    // canonical source handle exists.
    picker_supported && (!project_copy || source_handle_id.is_none())
}

#[cfg(any(test, target_arch = "wasm32"))]
fn accept_browser_canonical_display_name(
    current: &mut Option<String>,
    project_copy: bool,
    display_name: &str,
) {
    if !project_copy {
        *current = Some(display_name.to_owned());
    }
}

#[cfg(target_arch = "wasm32")]
fn start_browser_project_save(
    state: &mut AppState,
    scope: SaveScope,
    project_copy: bool,
) -> SaveRequestOutcome {
    let suggested_name = project_save_dialog_default_name(state);
    let mut prepared = match crate::common::project_lifecycle::prepare_browser_save(
        state,
        scope,
        project_copy,
        suggested_name,
    ) {
        Ok(prepared) => prepared,
        Err(error) => {
            lifecycle_error(state, error, "Project save failed");
            return SaveRequestOutcome::CancelledOrFailed;
        }
    };

    let save_surface_supported = browser_save_picker_is_safe(
        project_copy,
        prepared.source_handle_id,
        if project_copy {
            crate::common::project_lifecycle::browser_file_picker_supported()
        } else {
            crate::common::project_lifecycle::browser_canonical_save_supported()
        },
    );
    if prepared.target.handle_id.is_none()
        && prepared.target.backend
            == crate::common::project_lifecycle::BrowserBindingBackend::ExternalFile
        && !save_surface_supported
    {
        let text = match String::from_utf8(std::mem::take(&mut prepared.bytes)) {
            Ok(text) => text,
            Err(error) => {
                crate::common::project_lifecycle::cancel_transaction_if(
                    state,
                    prepared.transaction,
                );
                state.push_user_message(ConsoleMessage::error(format!(
                    "Project copy failed: serialized project was not UTF-8: {error}"
                )));
                return SaveRequestOutcome::CancelledOrFailed;
            }
        };
        let path = std::path::PathBuf::from(&prepared.suggested_name);
        match crate::common::browser_download::download_text_file(&path, &text) {
            Ok(()) => {
                crate::common::project_lifecycle::cancel_transaction_if(
                    state,
                    prepared.transaction,
                );
                state.push_user_message(ConsoleMessage::warning(if project_copy {
                    "Downloaded an independent project copy; the active project binding and dirty state are unchanged"
                } else {
                    "Download fallback created a project copy only, not a canonical save; unsaved changes remain"
                }));
                return SaveRequestOutcome::CopyOnly;
            }
            Err(error) => {
                crate::common::project_lifecycle::cancel_transaction_if(
                    state,
                    prepared.transaction,
                );
                state.push_user_message(ConsoleMessage::error(format!(
                    "Project download failed: {error}"
                )));
                return SaveRequestOutcome::CancelledOrFailed;
            }
        }
    }

    let target = prepared.target.clone();
    let name = prepared.suggested_name.clone();
    let bytes = std::mem::take(&mut prepared.bytes);
    let transaction = prepared.transaction;
    match crate::common::project_lifecycle::start_browser_write(
        target,
        !project_copy,
        &name,
        bytes,
        move |result| {
            BROWSER_PROJECT_SAVE_RESULTS.with(|queue| {
                queue
                    .borrow_mut()
                    .push_back(BrowserProjectSaveCompletion { prepared, result });
            });
            crate::common::browser_file_import::request_browser_import_repaint();
        },
    ) {
        Ok(()) => {
            if project_copy {
                SaveRequestOutcome::CopyPending
            } else {
                SaveRequestOutcome::CanonicalPending(transaction)
            }
        }
        Err(error) => {
            crate::common::project_lifecycle::cancel_transaction_if(state, transaction);
            state.push_user_message(ConsoleMessage::error(format!(
                "Project save failed: {error}"
            )));
            SaveRequestOutcome::CancelledOrFailed
        }
    }
}

#[cfg(target_arch = "wasm32")]
struct BrowserProjectSaveCompletion {
    prepared: crate::common::project_lifecycle::BrowserPreparedSave,
    result: crate::common::project_lifecycle::BrowserWriteResult,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SaveContinuationEvent {
    Saved(crate::common::project_lifecycle::TransactionId),
    SavedWithNewerChanges(crate::common::project_lifecycle::TransactionId),
    NotSaved(crate::common::project_lifecycle::TransactionId),
}

#[cfg(target_arch = "wasm32")]
impl SaveContinuationEvent {
    pub(crate) fn transaction(self) -> crate::common::project_lifecycle::TransactionId {
        match self {
            Self::Saved(transaction)
            | Self::SavedWithNewerChanges(transaction)
            | Self::NotSaved(transaction) => transaction,
        }
    }

    pub(crate) fn authorizes_destructive_action(self) -> bool {
        matches!(self, Self::Saved(_))
    }

    pub(crate) fn needs_another_save(self) -> bool {
        matches!(self, Self::SavedWithNewerChanges(_))
    }
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_PROJECT_SAVE_RESULTS: std::cell::RefCell<std::collections::VecDeque<BrowserProjectSaveCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn poll_browser_project_save(state: &mut AppState) -> Option<SaveContinuationEvent> {
    let completion = BROWSER_PROJECT_SAVE_RESULTS.with(|queue| queue.borrow_mut().pop_front())?;
    let project_copy = completion.prepared.project_copy;
    let transaction = completion.prepared.transaction;
    let saved_scope = completion.prepared.scope;
    let saved_document = completion.prepared.saved_document.clone();
    if !crate::common::project_lifecycle::browser_operation_context_is_current(
        state,
        &completion.prepared.context,
    ) {
        match &completion.result {
            crate::common::project_lifecycle::BrowserWriteResult::Saved { handle_id, .. }
            | crate::common::project_lifecycle::BrowserWriteResult::SavedSessionOnly {
                handle_id,
                ..
            } => crate::common::project_lifecycle::release_browser_handle_if_unowned(
                state, *handle_id,
            ),
            crate::common::project_lifecycle::BrowserWriteResult::Cancelled
            | crate::common::project_lifecycle::BrowserWriteResult::ExternalChange { .. }
            | crate::common::project_lifecycle::BrowserWriteResult::Failed(_) => {}
        }
        crate::common::project_lifecycle::cancel_transaction_if(state, transaction);
        return (!project_copy).then_some(SaveContinuationEvent::NotSaved(transaction));
    }
    let mut continuation = None;
    match completion.result {
        crate::common::project_lifecycle::BrowserWriteResult::Saved {
            handle_id,
            binding_id,
            backend,
            project_id,
            generation,
            display_name,
            digest,
        } => match crate::common::project_lifecycle::complete_browser_save(
            state,
            completion.prepared,
            crate::common::project_lifecycle::BrowserSavePublication {
                handle_id,
                binding_id,
                backend,
                project_id,
                generation,
                display_name: display_name.clone(),
                digest,
                durable: true,
            },
        ) {
            Ok(()) => {
                accept_browser_canonical_display_name(
                    &mut state.browser_project_save_name,
                    project_copy,
                    &display_name,
                );
                state.push_user_message(ConsoleMessage::info(if project_copy {
                    format!(
                        "Saved independent project copy: {display_name}; the active project remains unchanged"
                    )
                } else {
                    format!("Saved canonical browser project: {display_name}")
                }));
                if !project_copy {
                    continuation = Some(canonical_save_continuation_event(
                        state,
                        transaction,
                        saved_scope,
                        &saved_document,
                    ));
                }
            }
            Err(error) => {
                lifecycle_error(state, error, "Browser save completion failed");
                if !project_copy {
                    continuation = Some(SaveContinuationEvent::NotSaved(transaction));
                }
            }
        },
        crate::common::project_lifecycle::BrowserWriteResult::SavedSessionOnly {
            handle_id,
            binding_id,
            backend,
            project_id,
            generation,
            display_name,
            digest,
            persistence_error,
        } => match crate::common::project_lifecycle::complete_browser_save(
            state,
            completion.prepared,
            crate::common::project_lifecycle::BrowserSavePublication {
                handle_id,
                binding_id,
                backend,
                project_id,
                generation,
                display_name: display_name.clone(),
                digest,
                durable: false,
            },
        ) {
            Ok(()) => {
                accept_browser_canonical_display_name(
                    &mut state.browser_project_save_name,
                    project_copy,
                    &display_name,
                );
                state.push_user_message(ConsoleMessage::warning(if project_copy {
                    format!(
                        "Saved independent project copy: {display_name}; browser binding storage was unavailable: {persistence_error}"
                    )
                } else {
                    format!(
                        "Saved and verified canonical browser project: {display_name}. The live session remains bound, but restart restoration is unavailable because browser binding storage failed: {persistence_error}"
                    )
                }));
                if !project_copy {
                    continuation = Some(canonical_save_continuation_event(
                        state,
                        transaction,
                        saved_scope,
                        &saved_document,
                    ));
                }
            }
            Err(error) => {
                lifecycle_error(state, error, "Browser save completion failed");
                if !project_copy {
                    continuation = Some(SaveContinuationEvent::NotSaved(transaction));
                }
            }
        },
        crate::common::project_lifecycle::BrowserWriteResult::Cancelled => {
            crate::common::project_lifecycle::cancel_transaction_if(state, transaction);
            if !project_copy {
                continuation = Some(SaveContinuationEvent::NotSaved(transaction));
            }
        }
        crate::common::project_lifecycle::BrowserWriteResult::ExternalChange {
            observed_digest,
        } => {
            crate::common::project_lifecycle::record_browser_save_conflict(
                state,
                &completion.prepared,
                observed_digest,
            );
            crate::common::project_lifecycle::cancel_transaction_if(state, transaction);
            state.push_user_message(ConsoleMessage::error(
                "Browser project changed outside RSpice; reopen it or save an independent project copy",
            ));
            if !project_copy {
                continuation = Some(SaveContinuationEvent::NotSaved(transaction));
            }
        }
        crate::common::project_lifecycle::BrowserWriteResult::Failed(error) => {
            crate::common::project_lifecycle::cancel_transaction_if(state, transaction);
            state.push_user_message(ConsoleMessage::error(format!(
                "Browser project save failed: {error}"
            )));
            if !project_copy {
                continuation = Some(SaveContinuationEvent::NotSaved(transaction));
            }
        }
    }
    continuation
}

#[cfg(target_arch = "wasm32")]
fn canonical_save_continuation_event(
    state: &AppState,
    transaction: crate::common::project_lifecycle::TransactionId,
    scope: SaveScope,
    saved_document: &crate::common::project_lifecycle::ProjectDocumentId,
) -> SaveContinuationEvent {
    if crate::common::project_lifecycle::saved_snapshot_authorizes_continuation(
        state,
        scope,
        saved_document,
    ) {
        SaveContinuationEvent::Saved(transaction)
    } else {
        SaveContinuationEvent::SavedWithNewerChanges(transaction)
    }
}

pub(crate) fn save_project_as(state: &mut AppState) -> bool {
    let default_name = project_save_dialog_default_name(state);

    #[cfg(not(target_arch = "wasm32"))]
    match crate::io::show_save_project_dialog(Some(default_name.as_str())) {
        Ok(path) => {
            match crate::common::project_lifecycle::save_project_copy_native(state, &path) {
                Ok(()) => {
                    state.push_user_message(ConsoleMessage::info(format!(
                    "Saved independent project copy: {} (the active project remains bound to its original location)",
                    path.display()
                )));
                    true
                }
                Err(error) => {
                    lifecycle_error(state, error, "Project copy failed");
                    false
                }
            }
        }
        Err(ProjectIoError::Cancelled) => false,
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Project Save As failed: {}",
                error
            )));
            false
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let _ = default_name;
        save_project_copy_in_browser(state)
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_file_operation_label(state: &AppState) -> Option<String> {
    if let Some(kind) = crate::common::browser_file_import::active_text_import_kind() {
        return Some(format!(
            "Waiting for the browser {} picker or permission request to finish.",
            kind.label()
        ));
    }
    crate::common::project_lifecycle::operation_in_progress(state)
        .then(|| "Waiting for the browser project file or permission request to finish.".to_owned())
}

/// Cancel app-side authority for an unresolved browser file operation. The
/// underlying JavaScript promise may still resolve, but transaction and import
/// lease generations make that completion unable to mutate current state.
#[cfg(target_arch = "wasm32")]
pub(crate) fn cancel_pending_browser_file_operation(state: &mut AppState) -> bool {
    let import_cancelled =
        crate::common::browser_file_import::cancel_active_text_import().is_some();
    let lifecycle_cancelled =
        crate::common::project_lifecycle::cancel_pending_browser_operation(state);
    let continuation_cancelled = state
        .dialogs
        .confirmation_dialog
        .cancel_awaiting_canonical_save();
    let cancelled = import_cancelled || lifecycle_cancelled || continuation_cancelled;
    if cancelled {
        state.push_user_message(ConsoleMessage::info(
            "Cancelled the pending browser file operation; any eventual late browser completion will be ignored",
        ));
    }
    cancelled
}

pub(crate) fn request_revert_active_document(state: &mut AppState) -> bool {
    match crate::common::project_lifecycle::prepare_revert_active_document(state) {
        Ok(token) => {
            state.dialogs.project_review_dialog.show_revert(token);
            true
        }
        Err(error) => {
            lifecycle_error(state, error, "Revert review unavailable");
            false
        }
    }
}

pub(crate) fn confirm_revert_active_document(
    state: &mut AppState,
    token: &crate::common::project_lifecycle::RevertReviewToken,
) -> bool {
    match crate::common::project_lifecycle::confirm_revert_active_document(state, token) {
        Ok(()) => {
            state.push_user_message(ConsoleMessage::info("Reverted active document"));
            true
        }
        Err(error) => {
            lifecycle_error(state, error, "Revert failed");
            false
        }
    }
}

pub(crate) fn close_active_document(state: &mut AppState) -> bool {
    match crate::common::project_lifecycle::close_active_document(state) {
        Ok(()) => {
            state.push_user_message(ConsoleMessage::info(
                "Closed active document presentation; project data was retained",
            ));
            true
        }
        Err(error) => {
            lifecycle_error(state, error, "Close document failed");
            false
        }
    }
}

pub(crate) fn request_close_project(state: &mut AppState) -> bool {
    request_close_project_for(state, ProjectCloseDestination::Launcher)
}

/// Request the same data-safe close review as File > Close Project, but leave
/// the application in the mockup's explicit no-project workbench after the
/// validated close transaction completes.
pub(crate) fn request_close_project_to_empty_workbench(state: &mut AppState) -> bool {
    request_close_project_for(state, ProjectCloseDestination::EmptyWorkbench)
}

fn request_close_project_for(state: &mut AppState, destination: ProjectCloseDestination) -> bool {
    if !state.project_lifecycle.project_open {
        lifecycle_error(
            state,
            ProjectLifecycleError::NoProject,
            "Close project unavailable",
        );
        return false;
    }
    state.workbench.begin_project_close(destination);
    state.dialogs.project_review_dialog.show_close_project();
    true
}

pub(crate) fn close_project_discard(state: &mut AppState) -> bool {
    if state.simulation.is_running {
        lifecycle_error(
            state,
            ProjectLifecycleError::ActiveRun,
            "Close project blocked",
        );
        return false;
    }
    let mut libraries = crate::state::LibraryManager::with_primitives();
    let mut workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut libraries);
    let schematic = state.new_schematic_document();
    workspace.save_active_schematic(&schematic);
    state.clear_design_execution_context();
    state.library_manager = libraries;
    state.workspace = workspace;
    state.schematic = schematic;
    state.sim_setup =
        crate::common::app::SimSetupState::new_with_user_preferences(&state.ui.preferences);
    state.model_library_manager = crate::common::app::default_model_library_manager();
    state.browser_project_save_name = None;
    crate::common::project_lifecycle::mark_project_closed(state);
    match state.workbench.take_project_close_destination() {
        ProjectCloseDestination::Launcher => state.workbench.open_project_launcher(),
        ProjectCloseDestination::EmptyWorkbench => {
            state.workbench.project_launcher_open = false;
            state
                .workbench
                .activate(crate::workbench::state::Workspace::Project);
        }
    }
    state.push_user_message(ConsoleMessage::info("Closed project"));
    true
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn load_project_from_path(state: &mut AppState, path: &Path) -> bool {
    let transaction = match crate::common::project_lifecycle::begin_project_replacement(state) {
        Ok(transaction) => transaction,
        Err(error) => {
            lifecycle_error(state, error, "Project open blocked");
            return false;
        }
    };
    match crate::common::project_lifecycle::read_native_binding(path) {
        Ok((project, binding)) => {
            let canonical_path = match &binding {
                PersistenceBinding::Native { canonical_path, .. } => canonical_path.clone(),
            };
            apply_loaded_project_authorized(
                state,
                project,
                ProjectLoadOrigin::PersistentPath(&canonical_path),
                Some(binding),
                transaction,
            )
        }
        Err(error) => {
            crate::common::project_lifecycle::cancel_transaction(state);
            state.push_user_message(ConsoleMessage::error(format!(
                "Project open failed: {}",
                error
            )));
            false
        }
    }
}

#[cfg(test)]
pub(crate) fn apply_loaded_project(
    state: &mut AppState,
    project: ProjectFile,
    origin: ProjectLoadOrigin<'_>,
) -> bool {
    let transaction = match crate::common::project_lifecycle::begin_project_replacement(state) {
        Ok(transaction) => transaction,
        Err(error) => {
            lifecycle_error(state, error, "Project open blocked");
            return false;
        }
    };
    apply_loaded_project_authorized(state, project, origin, None, transaction)
}

fn apply_loaded_project_authorized(
    state: &mut AppState,
    mut project: ProjectFile,
    origin: ProjectLoadOrigin<'_>,
    binding: Option<PersistenceBinding>,
    transaction: crate::common::project_lifecycle::TransactionId,
) -> bool {
    if let Err(error) =
        crate::common::project_lifecycle::validate_project_replacement(state, transaction)
    {
        #[cfg(target_arch = "wasm32")]
        crate::common::project_lifecycle::cancel_transaction_if(state, transaction);
        #[cfg(not(target_arch = "wasm32"))]
        crate::common::project_lifecycle::cancel_transaction(state);
        lifecycle_error(state, error, "Project open blocked");
        return false;
    }
    let accepted_execution_context = project.execution_context.clone();
    let project_id = project.workspace.project.id();
    let (simulation_plan, model_library_manager, execution_warnings) =
        match project.execution_context.take() {
            Some(context) => match context.into_state(project_id) {
                Ok(restored) => restored,
                Err(error) => {
                    state.push_user_message(ConsoleMessage::error(format!(
                        "Project open failed: persisted execution context is invalid: {error}"
                    )));
                    #[cfg(target_arch = "wasm32")]
                    crate::common::project_lifecycle::cancel_transaction_if(state, transaction);
                    #[cfg(not(target_arch = "wasm32"))]
                    crate::common::project_lifecycle::cancel_transaction(state);
                    return false;
                }
            },
            None => (
                crate::common::app::SimSetupState::new_with_user_preferences(
                    &state.ui.preferences,
                ),
                crate::common::app::default_model_library_manager(),
                vec![
                    "This legacy project predates durable simulation plans; RSpice initialized the documented default Transient plan and built-in model catalog"
                        .to_owned(),
                ],
            ),
        };
    project
        .workspace
        .ensure_library_model(&mut project.libraries);
    match origin {
        #[cfg(not(target_arch = "wasm32"))]
        ProjectLoadOrigin::PersistentPath(_) => {
            state.browser_project_save_name = None;
        }
        #[cfg(any(test, target_arch = "wasm32"))]
        ProjectLoadOrigin::BrowserImport(name) | ProjectLoadOrigin::BrowserCanonical(name) => {
            project.workspace.project.path = None;
            state.browser_project_save_name = Some(name.to_string());
        }
    }
    let mut accepted_baseline = project.clone();
    accepted_baseline.execution_context = accepted_execution_context;
    let simulation_results = project.simulation_results;
    let mut simulation_results_warning = project.simulation_results_warning;
    state.clear_design_execution_context();
    state.library_manager = project.libraries;
    state.workspace = project.workspace;
    state.sim_setup = simulation_plan;
    state.model_library_manager = model_library_manager;
    state.restore_active_schematic_from_workspace();
    state.simulation = crate::state::SimulationState::default();
    if let Err(error) = simulation_results.apply_to_state(&mut state.simulation)
        && simulation_results_warning.is_none()
    {
        simulation_results_warning = Some(format!(
            "Simulation results were not restored because their persisted data is invalid: {error}"
        ));
    }
    if let Some(path) = origin.recent_path() {
        state.remember_recent_file(crate::common::app::RecentKind::Project, path);
    }
    state.push_user_message(ConsoleMessage::info(format!(
        "{}: {}",
        origin.success_prefix(),
        origin.display_label()
    )));
    if let Some(warning) = simulation_results_warning {
        state.push_user_message(ConsoleMessage::warning(warning));
    }
    for warning in execution_warnings {
        state.push_user_message(ConsoleMessage::warning(warning));
    }
    crate::common::project_lifecycle::accept_loaded_project(state, accepted_baseline, binding);
    true
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn open_project(state: &mut AppState) -> bool {
    match crate::io::show_open_project_dialog() {
        Ok(path) => load_project_from_path(state, &path),
        Err(ProjectIoError::Cancelled) => false,
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Project open failed: {}",
                error
            )));
            false
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn open_project(state: &mut AppState) -> bool {
    let transaction = match crate::common::project_lifecycle::begin_project_replacement(state) {
        Ok(transaction) => transaction,
        Err(error) => {
            lifecycle_error(state, error, "Project open blocked");
            return false;
        }
    };
    let context = crate::common::project_lifecycle::browser_operation_context(state);
    match start_browser_project_import(transaction, context) {
        Ok(()) => {
            state.push_user_message(ConsoleMessage::info(
                "Choose an RSpice project file to open",
            ));
            true
        }
        Err(error) => {
            crate::common::project_lifecycle::cancel_transaction(state);
            state.push_user_message(ConsoleMessage::error(format!(
                "Project open failed: {}",
                error
            )));
            false
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
enum BrowserProjectImportResult {
    Transaction(BrowserProjectImportCompletion),
    CanonicalPromoted {
        context: crate::common::project_lifecycle::BrowserOperationContext,
        handle_id: u64,
        display_name: String,
        result: Result<(), String>,
    },
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
struct BrowserProjectImportCompletion {
    transaction: crate::common::project_lifecycle::TransactionId,
    context: crate::common::project_lifecycle::BrowserOperationContext,
    import_token: crate::common::browser_file_import::TextImportToken,
    payload: BrowserProjectImportPayload,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
enum BrowserProjectImportPayload {
    Cancelled,
    Failed(String),
    Canonical(crate::common::project_lifecycle::BrowserOpenResult),
    Loaded(crate::common::browser_file_import::PickedTextFile),
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_PROJECT_IMPORT_RESULTS: std::cell::RefCell<std::collections::VecDeque<BrowserProjectImportResult>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

#[cfg(target_arch = "wasm32")]
fn start_browser_project_import(
    transaction: crate::common::project_lifecycle::TransactionId,
    context: crate::common::project_lifecycle::BrowserOperationContext,
) -> Result<(), String> {
    let import_token = crate::common::browser_file_import::try_begin_text_import(
        crate::common::browser_file_import::BrowserTextImportKind::Project,
    )?;

    if crate::common::project_lifecycle::browser_open_file_picker_supported() {
        let canonical_context = context.clone();
        let started = crate::common::project_lifecycle::start_browser_open(move |result| {
            BROWSER_PROJECT_IMPORT_RESULTS.with(|queue| {
                queue
                    .borrow_mut()
                    .push_back(BrowserProjectImportResult::Transaction(
                        BrowserProjectImportCompletion {
                            transaction,
                            context: canonical_context,
                            import_token,
                            payload: BrowserProjectImportPayload::Canonical(result),
                        },
                    ));
            });
            crate::common::browser_file_import::request_browser_import_repaint();
        });
        match started {
            Ok(()) => return Ok(()),
            Err(error) => {
                log::warn!(
                    "File System Access project open could not start; using the import-only picker fallback: {error}"
                );
            }
        }
    }

    crate::common::browser_file_import::pick_text_file(
        crate::io::project_io::PROJECT_FILTER.0,
        crate::io::project_io::PROJECT_FILTER.1,
        move |result| {
            let payload = match result {
                Ok(Some(file)) => BrowserProjectImportPayload::Loaded(file),
                Ok(None) => BrowserProjectImportPayload::Cancelled,
                Err(error) => BrowserProjectImportPayload::Failed(error),
            };
            BROWSER_PROJECT_IMPORT_RESULTS.with(|queue| {
                queue
                    .borrow_mut()
                    .push_back(BrowserProjectImportResult::Transaction(
                        BrowserProjectImportCompletion {
                            transaction,
                            context,
                            import_token,
                            payload,
                        },
                    ));
            });
        },
    );
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn poll_browser_project_import(state: &mut AppState) -> bool {
    let Some(event) = BROWSER_PROJECT_IMPORT_RESULTS.with(|queue| queue.borrow_mut().pop_front())
    else {
        return false;
    };
    match event {
        BrowserProjectImportResult::CanonicalPromoted {
            context,
            handle_id,
            display_name,
            result,
        } => {
            if crate::common::project_lifecycle::complete_browser_binding_promotion(
                state, &context, handle_id, &result,
            ) {
                match result {
                    Ok(()) => state.push_user_message(ConsoleMessage::info(format!(
                        "Enabled restart restoration for canonical browser project: {display_name}"
                    ))),
                    Err(error) => state.push_user_message(ConsoleMessage::warning(format!(
                        "Opened and verified canonical browser project: {display_name}. The live session remains bound, but restart restoration is unavailable because browser binding storage failed: {error}"
                    ))),
                }
            }
            false
        }
        BrowserProjectImportResult::Transaction(completion) => {
            let transaction = completion.transaction;
            if !crate::common::project_lifecycle::browser_operation_context_is_current(
                state,
                &completion.context,
            ) {
                if let BrowserProjectImportPayload::Canonical(
                    crate::common::project_lifecycle::BrowserOpenResult::Opened {
                        handle_id, ..
                    },
                ) = &completion.payload
                {
                    crate::common::project_lifecycle::release_browser_handle(*handle_id);
                }
                finish_browser_project_import(completion.import_token);
                crate::common::project_lifecycle::cancel_transaction_if(state, transaction);
                return false;
            }
            match completion.payload {
                BrowserProjectImportPayload::Cancelled => {
                    finish_browser_project_import(completion.import_token);
                    crate::common::project_lifecycle::cancel_transaction_if(state, transaction);
                    false
                }
                BrowserProjectImportPayload::Failed(error) => {
                    finish_browser_project_import(completion.import_token);
                    crate::common::project_lifecycle::cancel_transaction_if(state, transaction);
                    state.push_user_message(ConsoleMessage::error(format!(
                        "Project open failed: {error}"
                    )));
                    false
                }
                BrowserProjectImportPayload::Loaded(file) => {
                    finish_browser_project_import(completion.import_token);
                    match crate::io::project_io::load_project_text(&file.contents, None) {
                        Ok(project) => apply_loaded_project_authorized(
                            state,
                            project,
                            ProjectLoadOrigin::BrowserImport(&file.name),
                            None,
                            transaction,
                        ),
                        Err(error) => {
                            crate::common::project_lifecycle::cancel_transaction_if(
                                state,
                                transaction,
                            );
                            state.push_user_message(ConsoleMessage::error(format!(
                                "Project open failed: {error}"
                            )));
                            false
                        }
                    }
                }
                BrowserProjectImportPayload::Canonical(result) => {
                    finish_browser_project_import(completion.import_token);
                    finish_browser_canonical_open(state, transaction, result)
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn finish_browser_project_import(token: crate::common::browser_file_import::TextImportToken) {
    let _ = crate::common::browser_file_import::finish_text_import(token);
}

#[cfg(target_arch = "wasm32")]
fn finish_browser_canonical_open(
    state: &mut AppState,
    transaction: crate::common::project_lifecycle::TransactionId,
    result: crate::common::project_lifecycle::BrowserOpenResult,
) -> bool {
    let crate::common::project_lifecycle::BrowserOpenResult::Opened {
        handle_id,
        display_name,
        bytes,
        digest,
    } = result
    else {
        match result {
            crate::common::project_lifecycle::BrowserOpenResult::Cancelled => {
                crate::common::project_lifecycle::cancel_transaction_if(state, transaction);
            }
            crate::common::project_lifecycle::BrowserOpenResult::Failed(error) => {
                crate::common::project_lifecycle::cancel_transaction_if(state, transaction);
                state.push_user_message(ConsoleMessage::error(format!(
                    "Project open failed: {error}"
                )));
            }
            crate::common::project_lifecycle::BrowserOpenResult::Opened { .. } => unreachable!(),
        }
        return false;
    };
    let text = match String::from_utf8(bytes) {
        Ok(text) => text,
        Err(error) => {
            crate::common::project_lifecycle::release_browser_handle(handle_id);
            crate::common::project_lifecycle::cancel_transaction_if(state, transaction);
            state.push_user_message(ConsoleMessage::error(format!(
                "Project open failed: selected project is not valid UTF-8: {error}"
            )));
            return false;
        }
    };
    let project = match crate::io::project_io::load_project_text(&text, None) {
        Ok(project) => project,
        Err(error) => {
            crate::common::project_lifecycle::release_browser_handle(handle_id);
            crate::common::project_lifecycle::cancel_transaction_if(state, transaction);
            state.push_user_message(ConsoleMessage::error(format!(
                "Project open failed: {error}"
            )));
            return false;
        }
    };
    let binding = PersistenceBinding::Browser {
        handle_id,
        binding_id: uuid::Uuid::new_v4(),
        backend: crate::common::project_lifecycle::BrowserBindingBackend::ExternalFile,
        project_id: project.workspace.project.id().to_string(),
        accepted_generation: 1,
        display_name: display_name.clone(),
        accepted_digest: digest,
        persisted_generation: None,
    };
    let binding_for_persist = binding.clone();
    let opened = apply_loaded_project_authorized(
        state,
        project,
        ProjectLoadOrigin::BrowserCanonical(&display_name),
        Some(binding),
        transaction,
    );
    if !opened {
        crate::common::project_lifecycle::release_browser_handle(handle_id);
        return false;
    }
    // The project replacement is accepted before its binding record is
    // promoted. Promotion is separately context/CAS guarded, so IndexedDB can
    // never authorize an open that failed to apply or a project that has since
    // been replaced.
    let context = crate::common::project_lifecycle::begin_browser_binding_promotion(state);
    crate::common::project_lifecycle::start_browser_binding_persist(
        binding_for_persist,
        move |result| {
            BROWSER_PROJECT_IMPORT_RESULTS.with(|queue| {
                queue
                    .borrow_mut()
                    .push_back(BrowserProjectImportResult::CanonicalPromoted {
                        context,
                        handle_id,
                        display_name,
                        result,
                    });
            });
            crate::common::browser_file_import::request_browser_import_repaint();
        },
    );
    true
}

fn lifecycle_error(state: &mut AppState, error: ProjectLifecycleError, context: &str) {
    state.push_user_message(ConsoleMessage::error(format!("{context}: {error}")));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::bode::{BodeData, FrequencyResponse};
    use crate::analysis::eye_diagram::{EyeData, EyeTrace};
    use crate::analysis::fft::{FftData, window::WindowFunction};
    use crate::analysis::histogram::HistogramBuilder;
    use crate::analysis::nyquist::NyquistData;
    use crate::analysis::pole_zero::PoleZeroData;
    use crate::common::app::ActiveViewer;
    use crate::io::{ProjectExecutionContext, ProjectSimulationResults};

    fn seal_legacy_unattributed(run: &mut crate::state::SimulationRun) {
        run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
            .expect("synthetic historical run has valid unattributed legacy provenance");
    }

    fn project_named(path: &str) -> ProjectFile {
        let mut libraries = crate::state::LibraryManager::with_primitives();
        let mut workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut libraries);
        workspace.project.set_path(std::path::PathBuf::from(path));
        ProjectFile::new(workspace, libraries)
    }

    #[test]
    fn only_verified_canonical_completion_authorizes_an_immediate_destructive_action() {
        assert!(SaveRequestOutcome::CanonicalComplete.authorizes_immediate_destructive_action());
        assert!(
            !SaveRequestOutcome::CanonicalPending(
                crate::common::project_lifecycle::TransactionId::new()
            )
            .authorizes_immediate_destructive_action()
        );
        assert!(!SaveRequestOutcome::CopyOnly.authorizes_immediate_destructive_action());
        assert!(!SaveRequestOutcome::CopyPending.authorizes_immediate_destructive_action());
        assert!(!SaveRequestOutcome::CancelledOrFailed.authorizes_immediate_destructive_action());
    }

    #[test]
    fn browser_canonical_origin_is_distinct_from_copy_import_semantics() {
        let canonical = ProjectLoadOrigin::BrowserCanonical("sensor-afe.rspiceproj");
        let imported = ProjectLoadOrigin::BrowserImport("sensor-afe.rspiceproj");

        assert_eq!(canonical.display_label(), "sensor-afe.rspiceproj");
        assert_eq!(canonical.success_prefix(), "Opened project");
        assert_eq!(imported.success_prefix(), "Imported project");
        assert!(canonical.recent_path().is_none());
    }

    #[test]
    fn browser_project_copy_never_opens_a_save_picker_over_a_canonical_handle() {
        assert!(browser_save_picker_is_safe(true, None, true));
        assert!(!browser_save_picker_is_safe(true, Some(7), true));
        assert!(!browser_save_picker_is_safe(true, Some(7), false));
        assert!(browser_save_picker_is_safe(false, Some(7), true));
    }

    #[test]
    fn browser_project_copy_never_changes_canonical_save_name() {
        let mut canonical_name = Some("source-project.rspiceproj".to_owned());
        accept_browser_canonical_display_name(
            &mut canonical_name,
            true,
            "independent-copy.rspiceproj",
        );
        assert_eq!(canonical_name.as_deref(), Some("source-project.rspiceproj"));

        accept_browser_canonical_display_name(
            &mut canonical_name,
            false,
            "canonical-project.rspiceproj",
        );
        assert_eq!(
            canonical_name.as_deref(),
            Some("canonical-project.rspiceproj")
        );
    }

    #[test]
    fn launcher_continue_closes_through_the_reviewed_lifecycle_into_empty_workbench() {
        let mut state = AppState::default();
        state.workbench.open_project_launcher();

        assert!(request_close_project_to_empty_workbench(&mut state));
        assert!(matches!(
            state.dialogs.project_review_dialog.request,
            Some(crate::common::app::ProjectReviewRequest::CloseProject)
        ));

        state.dialogs.project_review_dialog.close();
        assert!(close_project_discard(&mut state));
        assert!(!state.project_lifecycle.project_open);
        assert!(!state.workbench.project_launcher_open);
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Project
        );
    }

    fn project_named_with_results(path: &str) -> ProjectFile {
        let mut libraries = crate::state::LibraryManager::with_primitives();
        let mut workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut libraries);
        workspace.project.set_path(std::path::PathBuf::from(path));

        let waveform = crate::state::WaveformData::new(
            "V(out)",
            vec![0.0, 1.0, 2.0],
            vec![0.0, 0.5, 1.0],
            "#00aaff",
        );
        let mut run = crate::state::SimulationRun::new(4);
        run.label = "Run 4 (import fixture)".to_string();
        run.add_analysis(
            crate::state::AnalysisResult::new(2, crate::state::AnalysisType::Transient, "TRAN")
                .with_waveforms(vec![waveform]),
        );
        seal_legacy_unattributed(&mut run);
        let mut simulation = crate::state::SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 4;
        simulation.active_run_idx = Some(0);
        simulation.active_analysis_idx = Some(0);

        ProjectFile::new_with_simulation_results(
            workspace,
            libraries,
            crate::io::project_io::ProjectSimulationResults::from_state(&simulation),
        )
    }

    fn seed_specialized_viewer_caches(state: &mut AppState) {
        state
            .analysis
            .histogram_state
            .load_histogram(HistogramBuilder::new().build(&[1.0, 2.0, 3.0]));

        let mut bode = BodeData::new();
        bode.add_response(FrequencyResponse::from_arrays(
            "old bode",
            &[1.0, 10.0],
            &[1.0, 0.1],
            &[0.0, -1.0],
        ));
        state.analysis.bode_plot_state.load_data(bode);

        state
            .analysis
            .nyquist_state
            .load_data(NyquistData::from_arrays(
                "old nyquist",
                &[1.0, 10.0],
                &[1.0, -0.5],
                &[0.0, 0.25],
            ));

        state
            .analysis
            .smith_chart_state
            .load_sparam_data("S11", &[1.0], &[0.25], &[0.0]);

        let mut pz = PoleZeroData::new("old pz");
        pz.add_real_pole(-1.0);
        state.analysis.pole_zero_state.load_data(pz);

        let mut eye = EyeData::new(1e-9, 2);
        eye.add_trace(EyeTrace::new(vec![0.0, 0.5, 1.0], vec![0.0, 1.0, 0.0]));
        state.analysis.eye_diagram_state.load_data(eye);

        state
            .analysis
            .fft_state
            .load_data(FftData::from_time_domain(
                "old fft",
                &[0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0],
                8.0,
                WindowFunction::Rectangular,
            ));

        for viewer in [
            ActiveViewer::SmithChart,
            ActiveViewer::EyeDiagram,
            ActiveViewer::Histogram,
            ActiveViewer::BodePlot,
            ActiveViewer::Nyquist,
            ActiveViewer::Fft,
            ActiveViewer::PoleZero,
        ] {
            assert!(
                state.viewer_is_available(viewer),
                "{} should be available before project switch",
                viewer.name()
            );
        }
    }

    fn assert_specialized_viewer_caches_cleared(state: &AppState) {
        for viewer in [
            ActiveViewer::SmithChart,
            ActiveViewer::EyeDiagram,
            ActiveViewer::Histogram,
            ActiveViewer::BodePlot,
            ActiveViewer::Nyquist,
            ActiveViewer::Fft,
            ActiveViewer::PoleZero,
        ] {
            assert!(
                !state.viewer_is_available(viewer),
                "{} should be unavailable after project switch",
                viewer.name()
            );
        }
    }

    #[test]
    fn browser_import_applies_project_clears_runs_and_skips_recents() {
        let mut state = AppState::default();
        seal_legacy_unattributed(state.simulation.start_run());
        assert!(state.simulation.has_results());

        let mut project = project_named("browser-import.rspiceproj");
        project.workspace.project.path = None;

        let imported = apply_loaded_project(
            &mut state,
            project,
            ProjectLoadOrigin::BrowserImport("browser-import.rspiceproj"),
        );

        assert!(imported);
        assert_eq!(state.workspace.project.display_name(), "browser-import");
        assert!(state.workspace.project.path.is_none());
        assert!(!state.simulation.has_results());
        assert!(state.recent_files.is_empty());
        assert!(state.log_buffer.entries().any(|entry| {
            entry
                .message
                .contains("Imported project: browser-import.rspiceproj")
        }));
    }

    #[test]
    fn browser_import_keeps_project_filename_as_save_suggestion_without_native_path() {
        let mut state = AppState::default();
        let project = project_named("stale-native-path.rspiceproj");

        let imported = apply_loaded_project(
            &mut state,
            project,
            ProjectLoadOrigin::BrowserImport("browser-import.rspiceproj"),
        );

        assert!(imported);
        assert!(state.workspace.project.path.is_none());
        assert_eq!(
            state.browser_project_save_name.as_deref(),
            Some("browser-import.rspiceproj")
        );
        assert!(state.recent_files.is_empty());
    }

    #[test]
    fn browser_import_filename_is_used_for_next_project_save_as_dialog() {
        let mut state = AppState::default();
        let project = project_named("stale-native-path.rspiceproj");

        assert!(apply_loaded_project(
            &mut state,
            project,
            ProjectLoadOrigin::BrowserImport("browser-import.rspiceproj"),
        ));

        assert_eq!(
            project_save_dialog_default_name(&state),
            "browser-import.rspiceproj"
        );
    }

    #[test]
    fn project_import_clears_stale_specialized_viewer_caches_without_results() {
        let mut state = AppState::default();
        seed_specialized_viewer_caches(&mut state);

        let project = project_named("browser-import.rspiceproj");

        assert!(apply_loaded_project(
            &mut state,
            project,
            ProjectLoadOrigin::BrowserImport("browser-import.rspiceproj"),
        ));

        assert_specialized_viewer_caches_cleared(&state);
    }

    #[test]
    fn create_new_project_clears_stale_specialized_viewer_caches() {
        let mut state = AppState::default();
        seed_specialized_viewer_caches(&mut state);
        state.browser_project_save_name = Some("previous-project.rspiceproj".to_owned());

        create_new_project(&mut state);

        assert_specialized_viewer_caches_cleared(&state);
        assert!(
            state.browser_project_save_name.is_none(),
            "a new browser project must not inherit the previous canonical suggestion"
        );
    }

    #[test]
    fn create_new_project_resets_project_owned_plan_and_model_context() {
        use crate::simulation::plan::AnalysisKind;
        use crate::state::model_library::{ModelLibrary, ModelLibraryManager};

        let mut state = AppState::default();
        let stale_plan = state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("current project owns a stable plan");
        let transient_id = stale_plan.instances()[0].id();
        stale_plan
            .remove(transient_id, Vec::new())
            .expect("default transient removes");
        stale_plan
            .insert(AnalysisKind::Noise)
            .expect("stale noise analysis inserts");
        let mut stale_models = ModelLibraryManager::new();
        stale_models.add_library(ModelLibrary::new("stale_project_models"));
        state.model_library_manager = stale_models;

        create_new_project(&mut state);

        let reset_plan = state
            .sim_setup
            .stable_analysis_plan()
            .expect("new project owns a stable plan");
        assert_eq!(reset_plan.instances().len(), 1);
        assert_eq!(reset_plan.instances()[0].kind(), AnalysisKind::Transient);
        assert!(reset_plan.instances()[0].enabled());
        assert!(
            state
                .model_library_manager
                .get_library("stale_project_models")
                .is_none()
        );
        assert!(state.model_library_manager.library_count() > 0);
    }

    #[test]
    fn create_new_project_copies_the_retained_solver_default_into_the_plan() {
        use crate::simulation::dialog::IntegrationMethod;
        use crate::workbench::ChoicePreference;

        let mut state = AppState::default();
        state
            .ui
            .preferences
            .set_choice(ChoicePreference::DefaultSolverPreset, 3)
            .expect("Robust is a valid solver preset");

        create_new_project(&mut state);

        assert_eq!(state.sim_setup.options.itl1, 200);
        assert_eq!(state.sim_setup.options.itl4, 20);
        assert!(state.sim_setup.options.arc_length);
        assert_eq!(state.sim_setup.options.method, IntegrationMethod::Gear2Only);
        assert_eq!(state.sim_setup.options.temp, 27.0);
    }

    #[test]
    fn project_import_restores_plan_order_solver_options_and_model_catalog() {
        use crate::simulation::dialog::{IntegrationMethod, MatrixSolver};
        use crate::simulation::plan::{AnalysisKind, AnalysisLifecycleState};
        use crate::state::model_library::{ModelLibrary, ModelLibraryManager};

        let mut source = AppState::default();
        let source_plan = source
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("current project owns a stable plan");
        let transient_id = source_plan.instances()[0].id();
        let (op_id, _) = source_plan
            .insert_at(AnalysisKind::OperatingPoint, 0)
            .expect("OP inserts first");
        let (ac_id, _) = source_plan
            .insert_at(AnalysisKind::Ac, 1)
            .expect("AC inserts second");
        source_plan
            .bind_dependency(ac_id, AnalysisKind::OperatingPoint, op_id)
            .expect("AC binds exact OP");
        let (noise_id, _) = source_plan
            .insert_at(AnalysisKind::Noise, 2)
            .expect("noise inserts third");
        source_plan
            .bind_dependency(noise_id, AnalysisKind::OperatingPoint, op_id)
            .expect("noise binds exact OP");
        source.sim_setup.options.reltol = 2e-4;
        source.sim_setup.options.method = IntegrationMethod::Gear2Only;
        source.sim_setup.options.solver = MatrixSolver::SparseLu;
        source.sim_setup.options.verbose = true;
        let mut project_models = ModelLibraryManager::new();
        project_models.add_library(ModelLibrary::new("project_exact_models"));
        source.model_library_manager = project_models;

        let context = ProjectExecutionContext::from_state(
            source.workspace.project.id(),
            &source.sim_setup,
            &source.model_library_manager,
        )
        .expect("source context validates");
        let expected_plan =
            serde_json::to_value(&context.simulation_plan).expect("expected plan serializes");
        let mut design_libraries = crate::state::LibraryManager::with_primitives();
        let mut workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut design_libraries);
        workspace
            .project
            .set_path(std::path::PathBuf::from("context.rspiceproj"));
        let project = ProjectFile::new_with_execution_context(
            workspace,
            design_libraries,
            ProjectSimulationResults::default(),
            context,
        );

        let mut target = AppState::default();
        target
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("target owns a stable plan")
            .insert(AnalysisKind::DcSweep)
            .expect("target stale analysis inserts");
        target.model_library_manager.clear();
        let imported = apply_loaded_project(
            &mut target,
            project,
            ProjectLoadOrigin::BrowserImport("context.rspiceproj"),
        );

        assert!(imported);
        assert_eq!(
            serde_json::to_value(&target.sim_setup).expect("restored plan serializes"),
            expected_plan
        );
        let restored = target
            .sim_setup
            .stable_analysis_plan()
            .expect("import restores stable plan");
        assert_eq!(
            restored
                .instances()
                .iter()
                .map(|instance| instance.id())
                .collect::<Vec<_>>(),
            vec![op_id, ac_id, noise_id, transient_id]
        );
        assert_eq!(
            restored
                .instance(ac_id)
                .expect("AC restored")
                .dependencies()[0]
                .target(),
            op_id
        );
        assert_eq!(
            restored
                .instance(noise_id)
                .expect("noise restored")
                .dependencies()[0]
                .target(),
            op_id
        );
        assert!(restored.instances().iter().all(|instance| {
            instance.enabled() && instance.lifecycle() == AnalysisLifecycleState::Draft
        }));
        assert_eq!(
            target.sim_setup.options_draft.reltol,
            crate::simulation::dialog::options::format_si_value(2e-4)
        );
        assert!(!target.sim_setup.options_open);
        assert!(target.sim_setup.options_errors.is_empty());
        assert_eq!(target.model_library_manager.library_count(), 1);
        assert!(
            target
                .model_library_manager
                .get_library("project_exact_models")
                .is_some()
        );
    }

    #[test]
    fn invalid_execution_context_does_not_partially_replace_open_project() {
        let mut project = project_named("invalid-context.rspiceproj");
        let context = ProjectExecutionContext::from_state(
            project.workspace.project.id(),
            &crate::common::app::SimSetupState::new(),
            &crate::state::model_library::ModelLibraryManager::new(),
        )
        .expect("baseline context validates");
        let mut value = serde_json::to_value(context).expect("context serializes");
        let instances = value["simulation_plan"]["analysis_plan"]["instances"]
            .as_array_mut()
            .expect("v4 instances are an array");
        let duplicate = instances[0].clone();
        instances.push(duplicate);
        let context: ProjectExecutionContext =
            serde_json::from_value(value).expect("corrupt structure deserializes for validation");
        project.execution_context = Some(context);

        let mut state = AppState::default();
        let original_project_name = state.workspace.project.display_name().to_owned();
        let original_active_view = state.workspace.active_view.clone();

        let imported = apply_loaded_project(
            &mut state,
            project,
            ProjectLoadOrigin::BrowserImport("invalid-context.rspiceproj"),
        );

        assert!(!imported);
        assert_eq!(
            state.workspace.project.display_name(),
            original_project_name
        );
        assert_eq!(state.workspace.active_view, original_active_view);
        assert!(state.log_buffer.entries().any(|entry| {
            entry
                .message
                .contains("persisted execution context is invalid")
                && entry.message.contains("appears more than once")
        }));
    }

    #[test]
    fn browser_import_restores_project_simulation_results_and_skips_recents() {
        let mut state = AppState::default();
        seal_legacy_unattributed(state.simulation.start_run());
        assert!(state.simulation.has_results());

        let project = project_named_with_results("browser-import.rspiceproj");

        let imported = apply_loaded_project(
            &mut state,
            project,
            ProjectLoadOrigin::BrowserImport("browser-import.rspiceproj"),
        );

        assert!(imported);
        assert_eq!(state.workspace.project.display_name(), "browser-import");
        assert_eq!(state.simulation.run_count(), 1);
        assert_eq!(
            state
                .simulation
                .active_run()
                .expect("active imported run")
                .label,
            "Run 4 (import fixture)"
        );
        assert_eq!(state.simulation.waveforms.len(), 1);
        assert_eq!(state.simulation.waveforms[0].name, "V(out)");
        assert!(state.recent_files.is_empty());
    }

    #[test]
    fn project_import_resets_non_persisted_simulation_runtime_state() {
        let mut state = AppState::default();
        // An actually running local simulation deliberately blocks project
        // replacement. These are stale, non-persisted controls from an
        // already-finished run and must be reset by the accepted import.
        state.simulation.is_running = false;
        state.simulation.trigger_simulation = true;
        state.simulation.trigger_abort = true;
        state.simulation.progress = 0.75;
        state.simulation.status = "Running stale project".to_string();
        state.simulation.netlist_content = "stale netlist".to_string();
        state
            .simulation
            .node_to_waveform
            .insert("stale".to_string(), 99);
        state.simulation.ground_node = Some("OLD_GND".to_string());

        let project = project_named_with_results("browser-import.rspiceproj");

        assert!(apply_loaded_project(
            &mut state,
            project,
            ProjectLoadOrigin::BrowserImport("browser-import.rspiceproj"),
        ));

        assert!(!state.simulation.is_running);
        assert!(!state.simulation.trigger_simulation);
        assert!(!state.simulation.trigger_abort);
        assert_eq!(state.simulation.progress, 0.0);
        assert!(state.simulation.status.is_empty());
        assert!(state.simulation.netlist_content.is_empty());
        assert_eq!(state.simulation.node_to_waveform.get("stale"), None);
        assert_eq!(state.simulation.ground_node, None);
        assert_eq!(state.simulation.waveforms[0].name, "V(out)");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn save_project_to_path_writes_simulation_results() {
        let mut state = AppState::default();
        let waveform = crate::state::WaveformData::new(
            "V(out)",
            vec![0.0, 1.0, 2.0],
            vec![0.0, 1.5, 3.0],
            "#00aaff",
        );
        let mut run = crate::state::SimulationRun::new(9);
        run.add_analysis(
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
                .with_waveforms(vec![waveform]),
        );
        seal_legacy_unattributed(&mut run);
        state.simulation.runs = vec![run];
        state.simulation.next_run_id = 9;
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);

        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "rspice-save-project-results-{}-{unique}.rspiceproj",
            std::process::id()
        ));

        let saved = save_project_to_path(&mut state, &path);
        let loaded = crate::io::load_project_file(&path).expect("saved project reloads");
        let _ = std::fs::remove_file(&path);

        assert!(saved);
        assert!(loaded.execution_context.is_some());
        assert_eq!(loaded.simulation_results.runs.len(), 1);
        assert_eq!(
            loaded.simulation_results.runs[0].analyses[0].waveforms[0].name,
            "V(out)"
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn save_project_to_path_rejects_invalid_simulation_results_without_publishing() {
        let mut state = AppState::default();
        let waveform = crate::state::WaveformData::new(
            "V(out)",
            vec![0.0, 1.0, 2.0],
            vec![0.0, f64::NAN, 3.0],
            "#00aaff",
        );
        let mut run = crate::state::SimulationRun::new(10);
        run.add_analysis(
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
                .with_waveforms(vec![waveform]),
        );
        state.simulation.runs = vec![run];
        state.simulation.next_run_id = 10;
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);

        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "rspice-save-project-invalid-results-{}-{unique}.rspiceproj",
            std::process::id()
        ));

        let saved = save_project_to_path(&mut state, &path);
        let published = path.exists();
        let _ = std::fs::remove_file(&path);

        assert!(!saved);
        assert!(!published);
        assert!(
            state
                .log_buffer
                .entries()
                .any(|entry| { entry.message.contains("Project save failed") })
        );
    }
}
