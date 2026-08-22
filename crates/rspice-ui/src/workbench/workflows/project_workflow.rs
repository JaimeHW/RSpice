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

/// Identity a new project is created under.
///
/// [`Default`] is the seed a route with no dialog gets: the untitled project
/// with the crate's default editable design library and top cell. The New
/// project dialog builds the same value from reviewed fields, so both routes
/// reach one creation path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NewProjectParams {
    pub(crate) name: String,
    pub(crate) root_library: String,
    pub(crate) top_cell: String,
}

impl Default for NewProjectParams {
    fn default() -> Self {
        Self {
            name: crate::state::ProjectDescriptor::default().name().to_owned(),
            root_library: crate::state::workspace::DEFAULT_PROJECT_LIBRARY.to_owned(),
            top_cell: crate::state::workspace::DEFAULT_TOP_CELL.to_owned(),
        }
    }
}

impl NewProjectParams {
    /// Reject an identity the project file could not persist, naming the field
    /// that failed. This is the only validation authority for the three names:
    /// the dialog calls it on every keystroke to gate its primary action, and
    /// creation calls it again so a route that skipped the dialog cannot seed
    /// a project whose own `validate` would refuse it.
    pub(crate) fn validate(&self) -> Result<(), String> {
        crate::state::ProjectDescriptor::validate_name(&self.name)
            .map_err(|error| format!("Project name {error}"))?;
        validate_identity_segment("Library", &self.root_library)?;
        validate_identity_segment("Top cell", &self.top_cell)
    }
}

fn validate_identity_segment(field: &str, value: &str) -> Result<(), String> {
    crate::state::workspace::validate_cell_view_name_segment(value)
        .map_err(|error| format!("{field} {error}"))
}

pub(crate) fn create_new_project(state: &mut AppState) {
    let _ = create_new_project_with(state, &NewProjectParams::default());
}

/// Replace the session with a new project under `params`.
///
/// Returns the exact refusal so a dialog can hold itself open beside the
/// console message rather than closing over a project that was never created.
pub(crate) fn create_new_project_with(
    state: &mut AppState,
    params: &NewProjectParams,
) -> Result<(), String> {
    if state.simulation.has_active_execution() {
        lifecycle_error(
            state,
            ProjectLifecycleError::ActiveRun,
            "New project blocked",
        );
        return Err(ProjectLifecycleError::ActiveRun.to_string());
    }
    if let Err(error) = params.validate() {
        state.push_user_message(ConsoleMessage::error(format!(
            "New project blocked: {error}"
        )));
        return Err(error);
    }
    let restored_model_catalog =
        crate::workbench::app_state::restore_session_model_library_manager(&mut state.pdk_config);
    let (model_library_manager, model_restore_errors) = match restored_model_catalog {
        Ok((manager, _loaded)) => (manager, Vec::new()),
        Err(errors) => (
            crate::workbench::app_state::default_model_library_manager(),
            errors,
        ),
    };
    let mut library_manager = crate::state::LibraryManager::with_primitives();
    let mut workspace = crate::state::ProjectWorkspace::new_empty_bootstrapped(
        &mut library_manager,
        &params.name,
        &params.root_library,
        &params.top_cell,
    );
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
    state.model_library_manager = model_library_manager;
    state.browser_project_save_name = None;
    crate::workbench::lifecycle::project_lifecycle::reset_for_new_project(state);
    state.push_user_message(ConsoleMessage::info(format!(
        "Created new project '{}' with top cell {}/{}/{}",
        params.name,
        params.root_library,
        params.top_cell,
        crate::state::workspace::DEFAULT_SCHEMATIC_VIEW
    )));
    emit_session_model_restore_errors(state, "new project", model_restore_errors);
    Ok(())
}

fn emit_session_model_restore_errors(state: &mut AppState, operation: &str, errors: Vec<String>) {
    for error in errors {
        state.push_user_message(ConsoleMessage::error(format!(
            "Configured PDK model catalog could not be restored for {operation}; built-in models remain available: {error}"
        )));
    }
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
    CanonicalPending(crate::product::TransactionId),
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
    Saved(crate::product::TransactionId),
    SavedWithNewerChanges(crate::product::TransactionId),
    Cancelled(crate::product::TransactionId),
    Conflict(crate::product::TransactionId),
    Failed(crate::product::TransactionId, String),
    PublishedButNotAdopted(crate::product::TransactionId, String),
}

#[cfg(target_arch = "wasm32")]
impl SaveContinuationEvent {
    pub(crate) fn transaction(&self) -> crate::product::TransactionId {
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
    transaction: crate::product::TransactionId,
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
    if state.simulation.has_active_execution() {
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
    let restored_model_catalog =
        crate::workbench::app_state::restore_session_model_library_manager(&mut state.pdk_config);
    let (model_library_manager, model_restore_errors) = match restored_model_catalog {
        Ok((manager, _loaded)) => (manager, Vec::new()),
        Err(errors) => (
            crate::workbench::app_state::default_model_library_manager(),
            errors,
        ),
    };
    state.model_library_manager = model_library_manager;
    state.browser_project_save_name = None;
    crate::workbench::lifecycle::project_lifecycle::mark_project_closed(state);
    emit_session_model_restore_errors(state, "project close", model_restore_errors);
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
    transaction: crate::product::TransactionId,
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
    let (simulation_plan, model_library_manager, execution_warnings) = match project
        .execution_context
        .take()
    {
        Some(context) => match context.into_state(project_id) {
            Ok(restored) => restored,
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "Project open failed: persisted execution context is invalid: {error}"
                )));
                #[cfg(target_arch = "wasm32")]
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                    state,
                    transaction,
                );
                #[cfg(not(target_arch = "wasm32"))]
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
                return false;
            }
        },
        None => {
            let (manager, mut warnings) =
                match crate::workbench::app_state::restore_session_model_library_manager(
                    &mut state.pdk_config,
                ) {
                    Ok((manager, loaded)) => (
                        manager,
                        vec![format!(
                            "This legacy project predates durable simulation plans; RSpice initialized the documented default Transient plan and restored {loaded} configured PDK model source{}",
                            if loaded == 1 { "" } else { "s" }
                        )],
                    ),
                    Err(errors) => {
                        let mut warnings = vec![
                                "This legacy project predates durable simulation plans; RSpice initialized the documented default Transient plan and built-in model catalog"
                                    .to_owned(),
                            ];
                        warnings.extend(errors.into_iter().map(|error| {
                                format!(
                                    "Configured PDK model catalog could not be restored while opening the legacy project: {error}"
                                )
                            }));
                        (
                            crate::workbench::app_state::default_model_library_manager(),
                            warnings,
                        )
                    }
                };
            if manager.library_count() == 0 {
                warnings.push(
                    "The restored model catalog is empty; simulation model binding is unavailable"
                        .to_owned(),
                );
            }
            (
                crate::workbench::app_state::SimSetupState::new_with_user_preferences(
                    &state.ui.preferences,
                ),
                manager,
                warnings,
            )
        }
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
    let result_log_y_panes = project.result_log_y_panes;
    let result_expression_groups = project.result_expression_groups;
    let mut simulation_results_warning = project.simulation_results_warning;
    let workspace_migration_warning = project.workspace_migration_warning;
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
    crate::workbench::documents::result_document::restore_log_y_panes(state, result_log_y_panes);
    crate::workbench::documents::result_document::restore_expression_groups(
        state,
        result_expression_groups,
    );
    if let Some(path) = origin.recent_path() {
        state.remember_recent_file(crate::workbench::app_state::RecentKind::Project, path);
    }
    state.push_user_message(ConsoleMessage::info(format!(
        "{}: {}",
        origin.success_prefix(),
        origin.display_label()
    )));
    if let Some(warning) = workspace_migration_warning {
        state.push_user_message(ConsoleMessage::warning(warning));
    }
    if let Some(warning) = simulation_results_warning {
        state.push_user_message(ConsoleMessage::warning(warning));
    }
    for warning in execution_warnings {
        state.push_user_message(ConsoleMessage::warning(warning));
    }
    report_legacy_run_set_notes(state);
    crate::workbench::lifecycle::project_lifecycle::accept_loaded_project(
        state,
        accepted_baseline,
        binding,
    );
    true
}

/// Report what folding a pre-unification Corner run space into the plan-global
/// one had to drop.
///
/// The migration records those losses while the disappearing axes can still be
/// named, and nothing read them: a project opened with its Corner analysis
/// silently running a different space than the one it was saved with. They are
/// reported through the console, beside the load's other losses, because they
/// are a fact about this load rather than about the project — a save carries
/// one declaration and has nothing left to report.
///
/// Drained rather than read, so a second load of the same session cannot repeat
/// a loss that was already announced.
fn report_legacy_run_set_notes(state: &mut AppState) {
    for note in std::mem::take(&mut state.sim_setup.legacy_run_set_notes) {
        state.push_user_message(ConsoleMessage::warning(note));
    }
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
    transaction: crate::product::TransactionId,
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
    transaction: crate::product::TransactionId,
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
    transaction: crate::product::TransactionId,
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
mod tests;
