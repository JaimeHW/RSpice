//! Project file workflows.
//!
//! New, open, save, save-as, and close for a project, including the
//! checkpoint and binding-receipt bookkeeping that makes a reopened project
//! resume where it left off.

use std::path::Path;

use crate::diagnostics::ConsoleMessage;
use crate::io::ProjectFile;
#[cfg(not(target_arch = "wasm32"))]
use crate::io::ProjectIoError;
use crate::workbench::app_state::AppState;
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::lifecycle::project_lifecycle::DestinationAuthority;
use crate::workbench::lifecycle::project_lifecycle::{
    PersistenceBinding, ProjectLifecycleError, SaveScope,
};
use crate::workbench::state::ProjectCloseDestination;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ProjectLoadOrigin<'a> {
    #[cfg(not(target_arch = "wasm32"))]
    PersistentPath(&'a Path),
    /// A live-session host's project snapshot; carries the host's display
    /// name. Never touches recent files or persistence bindings.
    LiveSession(&'a str),
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
            Self::LiveSession(host) => host.to_string(),
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::BrowserImport(name) | Self::BrowserCanonical(name) => name.to_string(),
        }
    }

    fn recent_path(self) -> Option<&'a Path> {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::PersistentPath(path) => Some(path),
            Self::LiveSession(_) => None,
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::BrowserImport(_) | Self::BrowserCanonical(_) => None,
        }
    }

    fn success_prefix(self) -> &'static str {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::PersistentPath(_) => "Opened project",
            Self::LiveSession(_) => "Synchronized live session project",
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
    seed_new_project_drawing_sheet_default(state, &mut workspace);
    let schematic = state.new_schematic_document();
    workspace.save_active_schematic(&schematic);

    state.library_manager = library_manager;
    state.library_edit_locks = crate::state::ProjectLibraryLockAuthority::default();
    state.workspace = workspace;
    state.schematic = schematic;
    state.ui.schematic_snap = state.schematic.snap_engine.clone();
    state.bump_active_schematic_epoch();
    state.clear_design_execution_context();
    state.sim_setup = crate::workbench::app_state::SimSetupState::new_with_user_preferences(
        &state.ui.preferences,
    );
    state.model_library_manager = crate::workbench::app_state::default_model_library_manager();
    state.browser_project_save_name = None;
    crate::workbench::lifecycle::project_lifecycle::reset_for_new_project(state);
    state.push_user_message(ConsoleMessage::info("Created new project"));
}

fn seed_new_project_drawing_sheet_default(
    state: &AppState,
    workspace: &mut crate::state::ProjectWorkspace,
) {
    let personal = state.ui.preferences.drawing_sheet_personal_preferences();
    let project_default = personal
        .default_format
        .try_update(|draft| {
            draft.inheritance = crate::state::DrawingSheetInheritance::ProjectDefault;
            if let crate::state::AuthoredDrawingSheetSize::Custom { snapshot } =
                &mut draft.authored_size
            {
                // A project default is self-contained. Personal preset
                // identity is captured only when a sheet explicitly uses that
                // preset; the project's starting format retains exact
                // dimensions without depending on device-local authority.
                snapshot.preset_id = None;
                snapshot.source_preset_unavailable = false;
            }
        })
        .expect("validated personal drawing-sheet defaults can seed a project")
        .as_drawing_sheet_default();
    let mut settings = workspace.design_management.drawing_sheet_settings().clone();
    settings.default_format = project_default;
    if settings != *workspace.design_management.drawing_sheet_settings() {
        workspace
            .design_management
            .update_drawing_sheet_settings(workspace.design_management.revision(), settings)
            .expect("a new project accepts its validated personal drawing-sheet default");
    }
}

#[cfg(all(not(target_arch = "wasm32"), test))]
pub(crate) fn save_project_to_path(state: &mut AppState, path: &Path) -> bool {
    save_native_scope(
        state,
        SaveScope::AllDocuments,
        path,
        DestinationAuthority::UserSelected,
    )
    .is_ok()
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SaveRequestOutcome {
    CanonicalComplete,
    CanonicalPending(crate::workbench::lifecycle::project_lifecycle::TransactionId),
    CopyOnly,
    CopyPending,
    Cancelled,
    Failed(String),
}

impl SaveRequestOutcome {
    fn request_started(&self) -> bool {
        !matches!(self, Self::Cancelled | Self::Failed(_))
    }

    pub(crate) fn authorizes_immediate_destructive_action(&self) -> bool {
        matches!(self, Self::CanonicalComplete)
    }
}

pub(crate) fn save_all_for_continuation(state: &mut AppState) -> SaveRequestOutcome {
    save_scope_outcome(state, SaveScope::AllDocuments)
}

pub(crate) fn save_active_for_continuation(state: &mut AppState) -> SaveRequestOutcome {
    save_scope_outcome(state, SaveScope::ActiveDocument)
}

/// A streamed mirror never acquires a canonical local binding implicitly.
/// Policy may authorize an explicit independent copy through Save As; it
/// never authorizes ordinary Save/Save All to retain or overwrite the mirror.
fn live_mirror_save_block(state: &AppState, project_copy: bool) -> Option<&'static str> {
    let locks = &state.workbench.live_write_locks;
    if !locks.mirror {
        None
    } else if !locks.mirror_save_copy_allowed {
        Some("The live session's policy does not allow saving a copy of the host's project.")
    } else if !project_copy {
        Some("A live mirror cannot be saved in place. Use Save As to create an independent copy.")
    } else {
        None
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_scope_outcome(state: &mut AppState, scope: SaveScope) -> SaveRequestOutcome {
    if let Some(message) = live_mirror_save_block(state, false) {
        state.push_user_message(ConsoleMessage::warning(message));
        return SaveRequestOutcome::Failed(message.to_owned());
    }
    if let Some(path) = crate::workbench::lifecycle::project_lifecycle::canonical_native_path(state)
    {
        return save_native_scope(state, scope, &path, DestinationAuthority::Canonical)
            .map_or_else(SaveRequestOutcome::Failed, |()| {
                SaveRequestOutcome::CanonicalComplete
            });
    }
    let default_name = project_save_dialog_default_name(state);
    match crate::io::show_save_project_dialog(Some(&default_name)) {
        Ok(path) => save_native_scope(state, scope, &path, DestinationAuthority::UserSelected)
            .map_or_else(SaveRequestOutcome::Failed, |()| {
                SaveRequestOutcome::CanonicalComplete
            }),
        Err(ProjectIoError::Cancelled) => SaveRequestOutcome::Cancelled,
        Err(error) => {
            let message = error.to_string();
            state.push_user_message(ConsoleMessage::error(format!(
                "Project save failed: {error}"
            )));
            SaveRequestOutcome::Failed(message)
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_native_scope(
    state: &mut AppState,
    scope: SaveScope,
    path: &Path,
    authority: DestinationAuthority,
) -> Result<(), String> {
    match crate::workbench::lifecycle::project_lifecycle::save_native(state, scope, path, authority)
    {
        Ok(()) => {
            let canonical =
                crate::workbench::lifecycle::project_lifecycle::canonical_native_path(state)
                    .unwrap_or_else(|| path.to_path_buf());
            state.browser_project_save_name = None;
            state
                .remember_recent_file(crate::workbench::app_state::RecentKind::Project, &canonical);
            state.push_user_message(ConsoleMessage::info(format!(
                "Saved project: {}",
                canonical.display()
            )));
            Ok(())
        }
        Err(error) => {
            let message = error.to_string();
            lifecycle_error(state, error, "Project save failed");
            Err(message)
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn save_scope_outcome(state: &mut AppState, scope: SaveScope) -> SaveRequestOutcome {
    if let Some(message) = live_mirror_save_block(state, false) {
        state.push_user_message(ConsoleMessage::warning(message));
        return SaveRequestOutcome::Failed(message.to_owned());
    }
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
    let mut prepared = match crate::workbench::lifecycle::project_lifecycle::prepare_browser_save(
        state,
        scope,
        project_copy,
        suggested_name,
    ) {
        Ok(prepared) => prepared,
        Err(error) => {
            let message = error.to_string();
            lifecycle_error(state, error, "Project save failed");
            return SaveRequestOutcome::Failed(message);
        }
    };

    let save_surface_supported = browser_save_picker_is_safe(
        project_copy,
        prepared.source_handle_id,
        if project_copy {
            crate::workbench::lifecycle::project_lifecycle::browser_file_picker_supported()
        } else {
            crate::workbench::lifecycle::project_lifecycle::browser_canonical_save_supported()
        },
    );
    if prepared.target.handle_id.is_none()
        && prepared.target.backend
            == crate::workbench::lifecycle::project_lifecycle::BrowserBindingBackend::ExternalFile
        && !save_surface_supported
    {
        let text = match String::from_utf8(std::mem::take(&mut prepared.bytes)) {
            Ok(text) => text,
            Err(error) => {
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                    state,
                    prepared.transaction,
                );
                state.push_user_message(ConsoleMessage::error(format!(
                    "Project copy failed: serialized project was not UTF-8: {error}"
                )));
                return SaveRequestOutcome::Failed(format!(
                    "Serialized project was not UTF-8: {error}"
                ));
            }
        };
        let path = std::path::PathBuf::from(&prepared.suggested_name);
        match crate::workbench::browser::download::download_text_file(&path, &text) {
            Ok(()) => {
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
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
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                    state,
                    prepared.transaction,
                );
                state.push_user_message(ConsoleMessage::error(format!(
                    "Project download failed: {error}"
                )));
                return SaveRequestOutcome::Failed(error);
            }
        }
    }

    let target = prepared.target.clone();
    let name = prepared.suggested_name.clone();
    let bytes = std::mem::take(&mut prepared.bytes);
    let transaction = prepared.transaction;
    match crate::workbench::lifecycle::project_lifecycle::start_browser_write(
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
            crate::workbench::browser::file_import::request_browser_import_repaint();
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
            let message = error.clone();
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                state,
                transaction,
            );
            state.push_user_message(ConsoleMessage::error(format!(
                "Project save failed: {error}"
            )));
            SaveRequestOutcome::Failed(message)
        }
    }
}

#[cfg(target_arch = "wasm32")]
struct BrowserProjectSaveCompletion {
    prepared: crate::workbench::lifecycle::project_lifecycle::BrowserPreparedSave,
    result: crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SaveContinuationEvent {
    Saved(crate::workbench::lifecycle::project_lifecycle::TransactionId),
    SavedWithNewerChanges(crate::workbench::lifecycle::project_lifecycle::TransactionId),
    Cancelled(crate::workbench::lifecycle::project_lifecycle::TransactionId),
    Conflict(crate::workbench::lifecycle::project_lifecycle::TransactionId),
    Failed(
        crate::workbench::lifecycle::project_lifecycle::TransactionId,
        String,
    ),
    PublishedButNotAdopted(
        crate::workbench::lifecycle::project_lifecycle::TransactionId,
        String,
    ),
}

#[cfg(target_arch = "wasm32")]
impl SaveContinuationEvent {
    pub(crate) fn transaction(
        &self,
    ) -> crate::workbench::lifecycle::project_lifecycle::TransactionId {
        match self {
            Self::Saved(transaction)
            | Self::SavedWithNewerChanges(transaction)
            | Self::Cancelled(transaction)
            | Self::Conflict(transaction)
            | Self::Failed(transaction, _)
            | Self::PublishedButNotAdopted(transaction, _) => *transaction,
        }
    }

    pub(crate) fn authorizes_destructive_action(&self) -> bool {
        matches!(self, Self::Saved(_))
    }

    pub(crate) fn needs_another_save(&self) -> bool {
        matches!(self, Self::SavedWithNewerChanges(_))
    }

    pub(crate) fn failure_message(&self) -> Option<&str> {
        match self {
            Self::Cancelled(_) => Some("The canonical save was cancelled."),
            Self::Conflict(_) => Some(
                "The canonical project changed outside RSpice; reopen it or save an independent project copy.",
            ),
            Self::Failed(_, message) | Self::PublishedButNotAdopted(_, message) => Some(message),
            Self::Saved(_) | Self::SavedWithNewerChanges(_) => None,
        }
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
    if !crate::workbench::lifecycle::project_lifecycle::browser_operation_context_is_current(
        state,
        &completion.prepared.context,
    ) {
        let terminal = match &completion.result {
            crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::Saved { .. }
            | crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::SavedSessionOnly { .. } => {
                SaveContinuationEvent::PublishedButNotAdopted(
                    transaction,
                    "The project bytes were published after this tab lost authority, so RSpice did not adopt them as the canonical live baseline. Reopen the project before continuing."
                        .to_owned(),
                )
            }
            crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::Cancelled => {
                SaveContinuationEvent::Cancelled(transaction)
            }
            crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::ExternalChange { .. } => {
                SaveContinuationEvent::Conflict(transaction)
            }
            crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::Failed(error) => {
                SaveContinuationEvent::Failed(transaction, error.clone())
            }
        };
        match &completion.result {
            crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::Saved {
                handle_id, ..
            }
            | crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::SavedSessionOnly {
                handle_id,
                ..
            } => crate::workbench::lifecycle::project_lifecycle::release_browser_handle_if_unowned(
                state, *handle_id,
            ),
            crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::Cancelled
            | crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::ExternalChange { .. }
            | crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::Failed(_) => {}
        }
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(state, transaction);
        return (!project_copy).then_some(terminal);
    }
    let mut continuation = None;
    match completion.result {
        crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::Saved {
            handle_id,
            binding_id,
            backend,
            project_id,
            generation,
            display_name,
            digest,
        } => match crate::workbench::lifecycle::project_lifecycle::complete_browser_save(
            state,
            completion.prepared,
            crate::workbench::lifecycle::project_lifecycle::BrowserSavePublication {
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
                let message = error.to_string();
                lifecycle_error(state, error, "Browser save completion failed");
                if !project_copy {
                    continuation = Some(SaveContinuationEvent::PublishedButNotAdopted(
                        transaction,
                        format!(
                            "The project bytes were written, but RSpice could not adopt the saved baseline: {message}"
                        ),
                    ));
                }
            }
        },
        crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::SavedSessionOnly {
            handle_id,
            binding_id,
            backend,
            project_id,
            generation,
            display_name,
            digest,
            persistence_error,
        } => match crate::workbench::lifecycle::project_lifecycle::complete_browser_save(
            state,
            completion.prepared,
            crate::workbench::lifecycle::project_lifecycle::BrowserSavePublication {
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
                let message = error.to_string();
                lifecycle_error(state, error, "Browser save completion failed");
                if !project_copy {
                    continuation = Some(SaveContinuationEvent::PublishedButNotAdopted(
                        transaction,
                        format!(
                            "The project bytes were written, but RSpice could not adopt the saved baseline: {message}"
                        ),
                    ));
                }
            }
        },
        crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::Cancelled => {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                state,
                transaction,
            );
            if !project_copy {
                continuation = Some(SaveContinuationEvent::Cancelled(transaction));
            }
        }
        crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::ExternalChange {
            observed_digest,
        } => {
            crate::workbench::lifecycle::project_lifecycle::record_browser_save_conflict(
                state,
                &completion.prepared,
                observed_digest,
            );
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                state,
                transaction,
            );
            state.push_user_message(ConsoleMessage::error(
                "Browser project changed outside RSpice; reopen it or save an independent project copy",
            ));
            if !project_copy {
                continuation = Some(SaveContinuationEvent::Conflict(transaction));
            }
        }
        crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::Failed(error) => {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                state,
                transaction,
            );
            state.push_user_message(ConsoleMessage::error(format!(
                "Browser project save failed: {error}"
            )));
            if !project_copy {
                continuation = Some(SaveContinuationEvent::Failed(transaction, error));
            }
        }
    }
    continuation
}

#[cfg(target_arch = "wasm32")]
fn canonical_save_continuation_event(
    state: &AppState,
    transaction: crate::workbench::lifecycle::project_lifecycle::TransactionId,
    scope: SaveScope,
    saved_document: &crate::workbench::lifecycle::project_lifecycle::ProjectDocumentId,
) -> SaveContinuationEvent {
    if crate::workbench::lifecycle::project_lifecycle::saved_snapshot_authorizes_continuation(
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
    if let Some(message) = live_mirror_save_block(state, true) {
        state.push_user_message(ConsoleMessage::warning(message));
        return false;
    }
    let default_name = project_save_dialog_default_name(state);

    #[cfg(not(target_arch = "wasm32"))]
    match crate::io::show_save_project_dialog(Some(default_name.as_str())) {
        Ok(path) => {
            match crate::workbench::lifecycle::project_lifecycle::save_project_copy_native(
                state, &path,
            ) {
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
    if let Some(kind) = crate::workbench::browser::file_import::active_text_import_kind() {
        return Some(format!(
            "Waiting for the browser {} picker or permission request to finish.",
            kind.label()
        ));
    }
    crate::workbench::lifecycle::project_lifecycle::operation_in_progress(state)
        .then(|| "Waiting for the browser project file or permission request to finish.".to_owned())
}

/// Cancel app-side authority for an unresolved browser file operation. The
/// underlying JavaScript promise may still resolve, but transaction and import
/// lease generations make that completion unable to mutate current state.
#[cfg(target_arch = "wasm32")]
pub(crate) fn cancel_pending_browser_file_operation(state: &mut AppState) -> bool {
    let import_cancelled =
        crate::workbench::browser::file_import::cancel_active_text_import().is_some();
    let lifecycle_cancelled =
        crate::workbench::lifecycle::project_lifecycle::cancel_pending_browser_operation(state);
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
    match crate::workbench::lifecycle::project_lifecycle::prepare_revert_active_document(state) {
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
    token: &crate::workbench::lifecycle::project_lifecycle::RevertReviewToken,
) -> bool {
    match crate::workbench::lifecycle::project_lifecycle::confirm_revert_active_document(
        state, token,
    ) {
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
    match crate::workbench::lifecycle::project_lifecycle::close_active_document(state) {
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
    state.library_edit_locks = crate::state::ProjectLibraryLockAuthority::default();
    state.workspace = workspace;
    state.schematic = schematic;
    state.ui.schematic_snap = state.schematic.snap_engine.clone();
    state.bump_active_schematic_epoch();
    state.sim_setup = crate::workbench::app_state::SimSetupState::new_with_user_preferences(
        &state.ui.preferences,
    );
    state.model_library_manager = crate::workbench::app_state::default_model_library_manager();
    state.browser_project_save_name = None;
    crate::workbench::lifecycle::project_lifecycle::mark_project_closed(state);
    match state.workbench.take_project_close_destination() {
        ProjectCloseDestination::Launcher => state.workbench.open_project_launcher(),
        ProjectCloseDestination::EmptyWorkbench => {
            state.workbench.project_launcher_open = false;
            state
                .workbench
                .activate(crate::workbench::state::Workspace::Project);
        }
        ProjectCloseDestination::LiveMirror => {
            state.workbench.project_launcher_open = false;
            state
                .workbench
                .activate(crate::workbench::state::Workspace::Project);
            state.workbench.request_live_mirror_entry();
        }
    }
    state.push_user_message(ConsoleMessage::info("Closed project"));
    true
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn load_project_from_path(state: &mut AppState, path: &Path) -> bool {
    let transaction =
        match crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(state) {
            Ok(transaction) => transaction,
            Err(error) => {
                lifecycle_error(state, error, "Project open blocked");
                return false;
            }
        };
    match crate::workbench::lifecycle::project_lifecycle::read_native_binding(path) {
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
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
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
    let transaction =
        match crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(state) {
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
    transaction: crate::workbench::lifecycle::project_lifecycle::TransactionId,
) -> bool {
    if let Err(error) = crate::workbench::lifecycle::project_lifecycle::validate_project_replacement(
        state,
        transaction,
    ) {
        #[cfg(target_arch = "wasm32")]
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(state, transaction);
        #[cfg(not(target_arch = "wasm32"))]
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
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
                    crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(state, transaction);
                    #[cfg(not(target_arch = "wasm32"))]
                    crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
                    return false;
                }
            },
            None => (
                crate::workbench::app_state::SimSetupState::new_with_user_preferences(
                    &state.ui.preferences,
                ),
                crate::workbench::app_state::default_model_library_manager(),
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
        ProjectLoadOrigin::LiveSession(_) => {
            // The host's on-disk location is meaningless on this machine and
            // must never become a save target here.
            project.workspace.project.path = None;
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
    let result_markers = project.result_markers;
    let mut simulation_results_warning = project.simulation_results_warning;
    state.clear_design_execution_context();
    state.library_manager = project.libraries;
    state.library_edit_locks = crate::state::ProjectLibraryLockAuthority::default();
    state.workspace = project.workspace;
    state.sim_setup = simulation_plan;
    state.model_library_manager = model_library_manager;
    state.restore_active_schematic_from_workspace();
    state.simulation = crate::state::SimulationState::default();
    if let Err(error) = state.validate_project_technology_contract() {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Project opened with simulation execution blocked: {error}. Review Project \u{00b7} Dependencies and attach an exact trusted signed PDK revision before running or governed saving"
        )));
    }
    if let Err(error) = simulation_results.apply_to_state(&mut state.simulation)
        && simulation_results_warning.is_none()
    {
        simulation_results_warning = Some(format!(
            "Simulation results were not restored because their persisted data is invalid: {error}"
        ));
    }
    // Markers are restored after the datasets they annotate, and only those
    // that still find their analysis: a marker pointing at a dataset this
    // project no longer retains would draw on nothing.
    crate::workbench::documents::result_document::restore_markers(state, result_markers);
    if let Some(path) = origin.recent_path() {
        state.remember_recent_file(crate::workbench::app_state::RecentKind::Project, path);
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
    crate::workbench::lifecycle::project_lifecycle::accept_loaded_project(
        state,
        accepted_baseline,
        binding,
    );
    true
}

/// Outcome of applying a live-session host's project snapshot.
pub(crate) enum LiveProjectApply {
    Applied,
    /// A local run or lifecycle transaction owns the project right now;
    /// the caller re-applies once it clears.
    RetryLater,
    Rejected,
}

/// Replace the whole workbench project with a live-session host's snapshot.
/// The join flow already confirmed closing local work and the session
/// arbitrates write authority, so no dialog or recent-file bookkeeping runs.
pub(crate) fn apply_live_project_snapshot(
    state: &mut AppState,
    bytes: &[u8],
    host_label: &str,
) -> LiveProjectApply {
    let Ok(text) = std::str::from_utf8(bytes) else {
        return LiveProjectApply::Rejected;
    };
    let project = match crate::io::project_io::load_project_text(text, None) {
        Ok(project) => project,
        Err(error) => {
            log::warn!("live session project snapshot rejected: {error}");
            return LiveProjectApply::Rejected;
        }
    };
    let transaction =
        match crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(state) {
            Ok(transaction) => transaction,
            Err(
                ProjectLifecycleError::ActiveRun | ProjectLifecycleError::TransactionInProgress,
            ) => {
                return LiveProjectApply::RetryLater;
            }
            Err(error) => {
                lifecycle_error(state, error, "Live session mirror blocked");
                return LiveProjectApply::Rejected;
            }
        };
    if apply_loaded_project_authorized(
        state,
        project,
        ProjectLoadOrigin::LiveSession(host_label),
        None,
        transaction,
    ) {
        LiveProjectApply::Applied
    } else {
        LiveProjectApply::Rejected
    }
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
    let transaction =
        match crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(state) {
            Ok(transaction) => transaction,
            Err(error) => {
                lifecycle_error(state, error, "Project open blocked");
                return false;
            }
        };
    let context = crate::workbench::lifecycle::project_lifecycle::browser_operation_context(state);
    match start_browser_project_import(transaction, context) {
        Ok(()) => {
            state.push_user_message(ConsoleMessage::info(
                "Choose an RSpice project file to open",
            ));
            true
        }
        Err(error) => {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
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
        context: crate::workbench::lifecycle::project_lifecycle::BrowserOperationContext,
        handle_id: u64,
        display_name: String,
        result: Result<(), String>,
    },
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
struct BrowserProjectImportCompletion {
    transaction: crate::workbench::lifecycle::project_lifecycle::TransactionId,
    context: crate::workbench::lifecycle::project_lifecycle::BrowserOperationContext,
    import_token: crate::workbench::browser::file_import::TextImportToken,
    payload: BrowserProjectImportPayload,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
enum BrowserProjectImportPayload {
    Cancelled,
    Failed(String),
    Canonical(crate::workbench::lifecycle::project_lifecycle::BrowserOpenResult),
    Loaded(crate::workbench::browser::file_import::PickedTextFile),
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_PROJECT_IMPORT_RESULTS: std::cell::RefCell<std::collections::VecDeque<BrowserProjectImportResult>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

#[cfg(target_arch = "wasm32")]
fn start_browser_project_import(
    transaction: crate::workbench::lifecycle::project_lifecycle::TransactionId,
    context: crate::workbench::lifecycle::project_lifecycle::BrowserOperationContext,
) -> Result<(), String> {
    let import_token = crate::workbench::browser::file_import::try_begin_text_import(
        crate::workbench::browser::file_import::BrowserTextImportKind::Project,
    )?;

    if crate::workbench::lifecycle::project_lifecycle::browser_open_file_picker_supported() {
        let canonical_context = context.clone();
        let started =
            crate::workbench::lifecycle::project_lifecycle::start_browser_open(move |result| {
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
                crate::workbench::browser::file_import::request_browser_import_repaint();
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

    crate::workbench::browser::file_import::pick_text_file(
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
            if crate::workbench::lifecycle::project_lifecycle::complete_browser_binding_promotion(
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
            if !crate::workbench::lifecycle::project_lifecycle::browser_operation_context_is_current(
                state,
                &completion.context,
            ) {
                if let BrowserProjectImportPayload::Canonical(
                    crate::workbench::lifecycle::project_lifecycle::BrowserOpenResult::Opened {
                        handle_id,
                        ..
                    },
                ) = &completion.payload
                {
                    crate::workbench::lifecycle::project_lifecycle::release_browser_handle(
                        *handle_id,
                    );
                }
                finish_browser_project_import(completion.import_token);
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                    state,
                    transaction,
                );
                return false;
            }
            match completion.payload {
                BrowserProjectImportPayload::Cancelled => {
                    finish_browser_project_import(completion.import_token);
                    crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                        state,
                        transaction,
                    );
                    false
                }
                BrowserProjectImportPayload::Failed(error) => {
                    finish_browser_project_import(completion.import_token);
                    crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                        state,
                        transaction,
                    );
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
                            crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
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
fn finish_browser_project_import(token: crate::workbench::browser::file_import::TextImportToken) {
    let _ = crate::workbench::browser::file_import::finish_text_import(token);
}

#[cfg(target_arch = "wasm32")]
fn finish_browser_canonical_open(
    state: &mut AppState,
    transaction: crate::workbench::lifecycle::project_lifecycle::TransactionId,
    result: crate::workbench::lifecycle::project_lifecycle::BrowserOpenResult,
) -> bool {
    let crate::workbench::lifecycle::project_lifecycle::BrowserOpenResult::Opened {
        handle_id,
        display_name,
        bytes,
        digest,
    } = result
    else {
        match result {
            crate::workbench::lifecycle::project_lifecycle::BrowserOpenResult::Cancelled => {
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                    state,
                    transaction,
                );
            }
            crate::workbench::lifecycle::project_lifecycle::BrowserOpenResult::Failed(error) => {
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                    state,
                    transaction,
                );
                state.push_user_message(ConsoleMessage::error(format!(
                    "Project open failed: {error}"
                )));
            }
            crate::workbench::lifecycle::project_lifecycle::BrowserOpenResult::Opened {
                ..
            } => unreachable!(),
        }
        return false;
    };
    let text = match String::from_utf8(bytes) {
        Ok(text) => text,
        Err(error) => {
            crate::workbench::lifecycle::project_lifecycle::release_browser_handle(handle_id);
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                state,
                transaction,
            );
            state.push_user_message(ConsoleMessage::error(format!(
                "Project open failed: selected project is not valid UTF-8: {error}"
            )));
            return false;
        }
    };
    let project = match crate::io::project_io::load_project_text(&text, None) {
        Ok(project) => project,
        Err(error) => {
            crate::workbench::lifecycle::project_lifecycle::release_browser_handle(handle_id);
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                state,
                transaction,
            );
            state.push_user_message(ConsoleMessage::error(format!(
                "Project open failed: {error}"
            )));
            return false;
        }
    };
    let binding = PersistenceBinding::Browser {
        handle_id,
        binding_id: uuid::Uuid::new_v4(),
        backend:
            crate::workbench::lifecycle::project_lifecycle::BrowserBindingBackend::ExternalFile,
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
        crate::workbench::lifecycle::project_lifecycle::release_browser_handle(handle_id);
        return false;
    }
    // The project replacement is accepted before its binding record is
    // promoted. Promotion is separately context/CAS guarded, so IndexedDB can
    // never authorize an open that failed to apply or a project that has since
    // been replaced.
    let context =
        crate::workbench::lifecycle::project_lifecycle::begin_browser_binding_promotion(state);
    crate::workbench::lifecycle::project_lifecycle::start_browser_binding_persist(
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
            crate::workbench::browser::file_import::request_browser_import_repaint();
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
    use crate::analysis::bode::BodeData;
    use crate::analysis::eye_diagram::{EyeData, EyeTrace};
    use crate::analysis::fft::{FftData, window::WindowFunction};
    use crate::analysis::histogram::HistogramBuilder;
    use crate::analysis::nyquist::NyquistData;
    use crate::analysis::pole_zero::PoleZeroData;
    use crate::io::{ProjectExecutionContext, ProjectSimulationResults};
    use crate::workbench::app_state::ActiveViewer;

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

    fn assert_active_grid_pitch_contract(
        state: &AppState,
        pitch: crate::state::SchematicGridPitch,
    ) {
        let expected = pitch.canvas_grid_size();
        assert_eq!(state.schematic.document_policy.grid_pitch, pitch);
        assert_eq!(state.schematic.grid_size, expected);
        assert_eq!(state.schematic.snap_engine.grid_size, expected);
        assert_eq!(state.ui.schematic_snap.grid_size, expected);
    }

    #[test]
    fn new_and_closed_project_installs_reconcile_every_grid_pitch_owner() {
        use crate::workbench::ChoicePreference;

        let mut state = AppState::default();
        state.ui.schematic_snap.snap_radius = 8;
        state.ui.schematic_snap.snap_to_grid = false;
        state.ui.schematic_snap.grid_size = 555;
        state
            .ui
            .preferences
            .set_choice(ChoicePreference::SchematicGrid, 1)
            .unwrap();

        create_new_project(&mut state);

        assert_active_grid_pitch_contract(&state, crate::state::SchematicGridPitch::Mil25);
        assert_eq!(state.schematic.snap_engine.snap_radius, 8);
        assert!(!state.schematic.snap_engine.snap_to_grid);

        state
            .ui
            .preferences
            .set_choice(ChoicePreference::SchematicGrid, 2)
            .unwrap();
        state.ui.schematic_snap.grid_size = 777;
        state
            .workbench
            .begin_project_close(ProjectCloseDestination::EmptyWorkbench);

        assert!(close_project_discard(&mut state));
        assert_active_grid_pitch_contract(&state, crate::state::SchematicGridPitch::Metric);
        assert_eq!(state.schematic.snap_engine.snap_radius, 8);
        assert!(!state.schematic.snap_engine.snap_to_grid);
    }

    #[test]
    fn only_verified_canonical_completion_authorizes_an_immediate_destructive_action() {
        assert!(SaveRequestOutcome::CanonicalComplete.authorizes_immediate_destructive_action());
        assert!(
            !SaveRequestOutcome::CanonicalPending(
                crate::workbench::lifecycle::project_lifecycle::TransactionId::new()
            )
            .authorizes_immediate_destructive_action()
        );
        assert!(!SaveRequestOutcome::CopyOnly.authorizes_immediate_destructive_action());
        assert!(!SaveRequestOutcome::CopyPending.authorizes_immediate_destructive_action());
        assert!(!SaveRequestOutcome::Cancelled.authorizes_immediate_destructive_action());
        assert!(
            !SaveRequestOutcome::Failed("disk unavailable".to_owned())
                .authorizes_immediate_destructive_action()
        );
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
            Some(crate::workbench::app::ProjectReviewRequest::CloseProject)
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

    #[test]
    fn close_to_live_mirror_raises_the_one_shot_engine_entry_request() {
        let mut state = AppState::default();
        state
            .workbench
            .begin_project_close(ProjectCloseDestination::LiveMirror);

        assert!(close_project_discard(&mut state));
        assert!(!state.project_lifecycle.project_open);
        assert!(!state.workbench.project_launcher_open);
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Project
        );
        assert!(state.workbench.take_live_mirror_entry());
        assert!(!state.workbench.take_live_mirror_entry());
    }

    #[test]
    fn live_project_snapshot_applies_wholesale_and_never_keeps_the_host_path() {
        let mut host = AppState::default();
        host.workspace
            .project
            .set_path(std::path::PathBuf::from("C:/host-only/design.rspiceproj"));
        let snapshot = crate::workbench::lifecycle::project_lifecycle::snapshot(&host)
            .expect("host state snapshots");
        let text =
            crate::io::project_io::serialize_project_file(&snapshot).expect("snapshot serializes");

        let mut guest = AppState::default();
        assert!(matches!(
            apply_live_project_snapshot(&mut guest, text.as_bytes(), "Jaime"),
            LiveProjectApply::Applied
        ));
        assert!(guest.project_lifecycle.project_open);
        assert_eq!(guest.workspace.project.id(), host.workspace.project.id());
        // The host's on-disk location must never become a guest save target.
        assert!(guest.workspace.project.path.is_none());
        assert!(
            crate::workbench::lifecycle::project_lifecycle::canonical_native_path(&guest).is_none()
        );
    }

    #[test]
    fn live_project_snapshot_waits_out_a_local_run_and_rejects_garbage() {
        let host = AppState::default();
        let snapshot = crate::workbench::lifecycle::project_lifecycle::snapshot(&host)
            .expect("host state snapshots");
        let text =
            crate::io::project_io::serialize_project_file(&snapshot).expect("snapshot serializes");

        let mut guest = AppState::default();
        guest.simulation.is_running = true;
        assert!(matches!(
            apply_live_project_snapshot(&mut guest, text.as_bytes(), "Jaime"),
            LiveProjectApply::RetryLater
        ));

        guest.simulation.is_running = false;
        assert!(matches!(
            apply_live_project_snapshot(&mut guest, b"not a project", "Jaime"),
            LiveProjectApply::Rejected
        ));
    }

    #[test]
    fn mirror_save_copy_policy_gates_every_project_persistence_path() {
        let mut state = AppState::default();
        state.workbench.live_write_locks.mirror = true;
        state.workbench.live_write_locks.mirror_save_copy_allowed = false;

        assert!(!save_project(&mut state));
        assert!(!save_all(&mut state));
        assert!(!save_project_as(&mut state));
        assert!(matches!(
            save_all_for_continuation(&mut state),
            SaveRequestOutcome::Failed(_)
        ));
    }

    #[test]
    fn save_copy_permission_never_turns_a_live_mirror_into_a_canonical_project() {
        let mut state = AppState::default();
        state.workbench.live_write_locks.mirror = true;
        state.workbench.live_write_locks.mirror_save_copy_allowed = true;

        assert_eq!(
            live_mirror_save_block(&state, false),
            Some(
                "A live mirror cannot be saved in place. Use Save As to create an independent copy."
            )
        );
        assert_eq!(live_mirror_save_block(&state, true), None);
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
        let waveform = crate::state::WaveformData::new(
            "V(out)",
            vec![0.0, 1.0, 2.0],
            vec![0.0, 0.5, 1.0],
            "#00aaff",
        );
        let mut run = crate::state::SimulationRun::new(1);
        run.add_analysis(
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
                .with_waveforms(vec![waveform]),
        );
        seal_legacy_unattributed(&mut run);
        state.simulation.runs = vec![run];
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        state.simulation.next_run_id = 2;

        state
            .analysis
            .histogram_state
            .load_histogram(HistogramBuilder::new().build(&[1.0, 2.0, 3.0]));

        let mut bode = BodeData::new();
        bode.add_response();
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

        let provenance = state
            .active_specialized_viewer_cache_provenance()
            .expect("default test project has an active retained analysis");
        for viewer in [
            ActiveViewer::BodePlot,
            ActiveViewer::Nyquist,
            ActiveViewer::SmithChart,
            ActiveViewer::Histogram,
            ActiveViewer::Fft,
            ActiveViewer::EyeDiagram,
        ] {
            state.bind_specialized_viewer_cache(viewer, provenance);
        }

        for viewer in [
            ActiveViewer::SmithChart,
            ActiveViewer::EyeDiagram,
            ActiveViewer::Histogram,
            ActiveViewer::BodePlot,
            ActiveViewer::Nyquist,
            ActiveViewer::Fft,
        ] {
            assert!(
                state.viewer_is_available(viewer),
                "{} should be available before project switch",
                viewer.name()
            );
        }
    }

    fn assert_specialized_viewer_caches_cleared(state: &AppState) {
        assert!(
            state.analysis.pole_zero_state.is_empty(),
            "legacy pole-zero presentation cache should be cleared"
        );
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
    fn create_new_project_captures_the_personal_drawing_sheet_default() {
        use crate::state::{
            DrawingSheetInheritance, DrawingSheetStandard, SchematicPageOrientation,
            SchematicSheetFormat,
        };

        let mut state = AppState::default();
        let mut personal = state.ui.preferences.drawing_sheet_personal_preferences();
        personal.default_format = SchematicSheetFormat::from_standard(
            DrawingSheetStandard::AnsiC,
            SchematicPageOrientation::Portrait,
        )
        .try_update(|draft| {
            draft.inheritance = DrawingSheetInheritance::UserDefault;
        })
        .expect("the personal default is valid");
        state
            .ui
            .preferences
            .set_drawing_sheet_personal_preferences(personal)
            .expect("the personal default persists");

        create_new_project(&mut state);

        let project_default = state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .default_format
            .clone();
        assert!(
            matches!(
                &project_default.authored_size,
                crate::state::AuthoredDrawingSheetSize::Standard {
                    standard: DrawingSheetStandard::AnsiC
                }
            ),
            "the personal physical format becomes the new project's default"
        );
        assert_eq!(
            project_default.orientation,
            SchematicPageOrientation::Portrait
        );
        assert_eq!(
            project_default.inheritance,
            DrawingSheetInheritance::ProjectDefault
        );
        assert!(crate::workbench::app::open_drawing_sheet_setup_for_state(
            &mut state
        ));
        let page_setup_format = state
            .dialogs
            .drawing_sheet_setup
            .draft
            .validate()
            .expect("the initial Page Setup draft is valid")
            .page_format;
        assert_eq!(
            page_setup_format.as_drawing_sheet_default(),
            project_default,
            "the initial Page Setup draft resolves the project default without storing a concrete sheet title in the reusable template"
        );
        assert_eq!(
            page_setup_format.title_block.fields
                [&crate::state::DrawingSheetTitleFieldId::SheetTitle]
                .value,
            "top",
            "the governed sheet keeps its own title while inheriting the project format"
        );
    }

    #[test]
    fn new_project_custom_default_is_exact_and_has_no_personal_preset_dependency() {
        use crate::state::{
            AuthoredDrawingSheetSize, DrawingSheetInheritance, SchematicPageOrientation,
            SchematicSheetFormat,
        };

        let mut state = AppState::default();
        let mut personal = state.ui.preferences.drawing_sheet_personal_preferences();
        personal.default_format = SchematicSheetFormat::try_custom(
            "Personal lab panel",
            250_001,
            400_003,
            SchematicPageOrientation::Portrait,
        )
        .unwrap()
        .try_update(|draft| {
            draft.inheritance = DrawingSheetInheritance::UserDefault;
            let AuthoredDrawingSheetSize::Custom { snapshot } = &mut draft.authored_size else {
                unreachable!("the test starts with a custom size");
            };
            snapshot.preset_id = Some("personal-lab-panel".to_owned());
        })
        .unwrap();
        state
            .ui
            .preferences
            .set_drawing_sheet_personal_preferences(personal.clone())
            .unwrap();
        let retained_personal = state.ui.preferences.drawing_sheet_personal_preferences();

        create_new_project(&mut state);

        let project_settings = state.workspace.design_management.drawing_sheet_settings();
        assert_eq!(
            project_settings.default_format.portrait_dimensions_um(),
            (250_001, 400_003)
        );
        let AuthoredDrawingSheetSize::Custom { snapshot } =
            &project_settings.default_format.authored_size
        else {
            panic!("the exact custom physical format must be retained");
        };
        assert!(snapshot.preset_id.is_none());
        assert!(!snapshot.source_preset_unavailable);
        assert!(project_settings.presets.is_empty());
        assert_eq!(
            state.ui.preferences.drawing_sheet_personal_preferences(),
            retained_personal,
            "seeding a project must not mutate personal preferences"
        );
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
            &crate::workbench::app_state::SimSetupState::new(),
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

    /// Markers are the reader's own annotation of a result. Losing them on
    /// close is data loss, so they are written beside the retained datasets
    /// and re-attach to the analysis they named.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn save_project_to_path_round_trips_result_markers() {
        use crate::workbench::documents::result_document::AnalysisPresentationKey;

        let mut state = AppState::default();
        let waveform = crate::state::WaveformData::new(
            "V(out)",
            vec![0.0, 1.0, 2.0],
            vec![0.0, 1.5, 3.0],
            "#00aaff",
        );
        let mut run = crate::state::SimulationRun::new(11);
        run.add_analysis(
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
                .with_waveforms(vec![waveform]),
        );
        seal_legacy_unattributed(&mut run);
        state.simulation.runs = vec![run];
        state.simulation.next_run_id = 11;
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);

        let active = state.simulation.active_run().expect("active retained run");
        let analysis = AnalysisPresentationKey::new(active.dataset_id, &active.analyses[0]);
        let anchor = state
            .ui
            .results
            .markers
            .first()
            .map(|marker| marker.anchor.clone());
        assert!(anchor.is_none(), "a fresh workspace carries no markers");
        let id = {
            let waveform_anchor =
                crate::workbench::documents::result_document::marker_anchor_for(analysis, "V(out)");
            state
                .ui
                .results
                .add_marker(analysis, waveform_anchor, "V(out)".to_owned(), 1.0)
        };
        if let Some(marker) = state.ui.results.marker_mut(id) {
            marker.note = "settling point".to_owned();
        }

        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "rspice-save-project-markers-{}-{unique}.rspiceproj",
            std::process::id()
        ));

        let saved = save_project_to_path(&mut state, &path);
        let loaded = crate::io::load_project_file(&path).expect("saved project reloads");
        let _ = std::fs::remove_file(&path);

        assert!(saved);
        assert_eq!(loaded.result_markers.len(), 1);
        assert_eq!(loaded.result_markers[0].note, "settling point");

        let mut reopened = AppState::default();
        reopened.simulation = state.simulation.clone();
        crate::workbench::documents::result_document::restore_markers(
            &mut reopened,
            loaded.result_markers,
        );
        assert_eq!(reopened.ui.results.markers.len(), 1);
        assert_eq!(reopened.ui.results.markers[0].note, "settling point");
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
