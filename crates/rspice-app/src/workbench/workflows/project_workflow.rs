//! Project file workflows.
//!
//! New, open, save, save-as, and close for a project, including the
//! checkpoint and binding-receipt bookkeeping that makes a reopened project
//! resume where it left off.

#[cfg(target_arch = "wasm32")]
use rspice_project::lifecycle::SaveContinuationEvent;
use std::path::Path;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::io::ProjectSnapshot;
use crate::workbench::app_state::SlotFacts;
use crate::workbench::app_state::{AppState, Model, PlatformState};
use crate::workbench::framework::feature::ModelsPort;
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::lifecycle::project_lifecycle::DestinationAuthority;
#[cfg(target_arch = "wasm32")]
use crate::workbench::lifecycle::project_lifecycle::PersistenceBinding;
use crate::workbench::lifecycle::project_lifecycle::ProjectCloseDestination;
use crate::workbench::lifecycle::project_lifecycle::{
    LifecycleHost, LifecycleParts, LifecycleSource, LifecycleSourceMut, LifecycleView,
    ProjectLifecycleError, SaveScope,
};
use crate::workbench::state::WorkbenchSession;
use crate::workbench::workflows::file_workflow::file_name_string;
use rspice_app_services::workbench::slots::platform::remember_recent_file;
#[cfg(not(target_arch = "wasm32"))]
use rspice_project::ProjectIoError;

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

/// Replace the session with a blank default project.
#[cfg(test)]
pub(crate) fn create_new_project(state: &mut AppState) {
    let _ = state.create_new_project_with(&NewProjectParams::default());
}

/// How a new project under `params` states itself when it replaces the open
/// one.
pub(crate) fn new_project_announcement(params: &NewProjectParams) -> String {
    format!(
        "Created project {} (top cell {}/{}).",
        params.name, params.root_library, params.top_cell
    )
}

/// The project a netlist being opened as one is created as, with how the
/// replacement states itself.
///
/// The deck's file stem names the project when it is a valid project name.
/// The replacement states itself as the netlist that was opened, which is
/// what the reader asked for; a "Created project" line would describe a
/// project they never asked for.
pub(crate) fn netlist_project(display_name: &str) -> (NewProjectParams, String) {
    const FALLBACK_NAME: &str = "Netlist Project";
    let mut params = NewProjectParams {
        name: std::path::Path::new(display_name)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .filter(|stem| !stem.trim().is_empty())
            .unwrap_or(FALLBACK_NAME)
            .to_owned(),
        ..NewProjectParams::default()
    };
    if params.validate().is_err() {
        params.name = FALLBACK_NAME.to_owned();
    }
    let announcement = format!("Opened netlist project {} ({display_name}).", params.name);
    (params, announcement)
}

/// Refuse a new project while a run is in progress or `params` do not name a
/// valid one, answering the gesture with the refusal.
pub(crate) fn refuse_new_project(
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    params: &NewProjectParams,
) -> Result<(), String> {
    if model.simulation.execution.has_active_execution() {
        report_lifecycle_error(
            model,
            workbench,
            ProjectLifecycleError::ActiveRun,
            "New project blocked",
        );
        return Err(ProjectLifecycleError::ActiveRun.to_string());
    }
    if let Err(error) = params.validate() {
        workbench.prompt(format!("New project not created: {error}"));
        return Err(error);
    }
    Ok(())
}

/// A new project, built off to the side before it replaces the open one.
pub(crate) struct NewProject {
    pub(crate) library_manager: crate::state::LibraryManager,
    pub(crate) workspace: crate::state::ProjectWorkspace,
    pub(crate) model_library_manager: crate::state::model_library::ModelLibraryManager,
    /// Why the session's PDK model catalog could not be restored, reported
    /// once the replacement has stated itself.
    pub(crate) model_restore_errors: Vec<String>,
}

/// Build the empty project `params` names: the session's PDK model catalog
/// restored for it, its libraries and workspace, and its drawing-sheet
/// default seeded from the reader's personal one.
pub(crate) fn build_new_project(model: &mut Model, params: &NewProjectParams) -> NewProject {
    let restored_model_catalog =
        crate::workbench::app_state::restore_session_model_library_manager(&mut model.pdk_config);
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
    seed_new_project_drawing_sheet_default(model, &mut workspace);
    NewProject {
        library_manager,
        workspace,
        model_library_manager,
        model_restore_errors,
    }
}

/// Make a replacement project's libraries and workspace the open ones, with
/// `schematic` as the drawing in front, saved into the workspace, under
/// fresh library edit locks.
pub(crate) fn install_project_drawing(
    model: &mut Model,
    library_manager: crate::state::LibraryManager,
    mut workspace: crate::state::ProjectWorkspace,
    schematic: crate::state::SchematicState,
) {
    workspace.save_active_schematic(&schematic);
    model.library_manager = library_manager;
    model.library_edit_locks = crate::state::ProjectLibraryLockAuthority::default();
    model.workspace = workspace;
    model.schematic = schematic;
}

/// Give a new project the session's first plan, with the data that plan
/// owns, and the model catalog it was built with. A new project has never
/// been saved, so it has no browser save name.
pub(crate) fn settle_new_project(
    model: &mut Model,
    platform: &mut PlatformState,
    model_library_manager: crate::state::model_library::ModelLibraryManager,
) {
    model.sim_setup =
        crate::workbench::app_state::SimSetupState::new_with_user_preferences(&model.preferences);
    // The new plan owns its variables, outputs and specifications, as the
    // session's first plan does. Without the payload every Run plan in a new
    // project was refused before it netlisted anything.
    if let Ok(plan) = model.sim_setup.stable_analysis_plan() {
        let plan_id = plan.id();
        model.workspace.content.migrate_active_plan_data(plan_id);
    }
    model.model_library_manager = model_library_manager;
    platform.browser_project_save_name = None;
}

/// Record why the session's PDK model catalog could not be restored for
/// `operation`.
pub(crate) fn emit_session_model_restore_errors(
    model: &mut Model,
    operation: &str,
    errors: Vec<String>,
) {
    for error in errors {
        model.record(
            LogSeverity::Warning,
            LogSource::Studio,
            format!(
                "The PDK model catalog was not restored for the {operation}; built-in models \
                 remain available: {error}"
            ),
        );
    }
}

fn seed_new_project_drawing_sheet_default(
    model: &Model,
    workspace: &mut crate::state::ProjectWorkspace,
) {
    let personal = model.preferences.drawing_sheet_personal_preferences();
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
    let mut settings = workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .clone();
    settings.default_format = project_default;
    if settings != *workspace.content.design_management.drawing_sheet_settings() {
        let revision = workspace.content.design_management.revision();
        workspace
            .content
            .design_management
            .update_drawing_sheet_settings(revision, settings)
            .expect("a new project accepts its validated personal drawing-sheet default");
    }
}

#[cfg(all(not(target_arch = "wasm32"), test))]
pub(crate) fn save_project_to_path(state: &mut AppState, path: &Path) -> bool {
    state
        .save_project_scope_to(
            SaveScope::AllDocuments,
            path,
            DestinationAuthority::UserSelected,
        )
        .is_ok()
}

fn project_save_dialog_default_name(model: &Model, platform: &PlatformState) -> String {
    model
        .workspace
        .content
        .project
        .path
        .as_deref()
        .and_then(file_name_string)
        .or_else(|| platform.browser_project_save_name.clone())
        .unwrap_or_else(|| "untitled.rspiceproj".to_string())
}

pub(crate) fn save_project(state: &mut AppState) -> bool {
    state
        .save_project_scope(SaveScope::ActiveDocument)
        .request_started()
}

pub(crate) fn save_all(state: &mut AppState) -> bool {
    state
        .save_project_scope(SaveScope::AllDocuments)
        .request_started()
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
    state.save_project_scope(SaveScope::AllDocuments)
}

pub(crate) fn save_active_for_continuation(state: &mut AppState) -> SaveRequestOutcome {
    state.save_project_scope(SaveScope::ActiveDocument)
}

/// A streamed mirror never acquires a canonical local binding implicitly.
/// Policy may authorize an explicit independent copy through Save As; it
/// never authorizes ordinary Save/Save All to retain or overwrite the mirror.
fn live_mirror_save_block(model: &Model, project_copy: bool) -> Option<&'static str> {
    let locks = &model.live_write_locks;
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

/// What a project save reads and writes: the lifecycle's parts, the status
/// bar it answers on, and the features' facts the design checks a save runs
/// read, taken when the save starts. A save changes none of those facts.
pub(crate) struct SaveParts<'a> {
    pub(crate) lifecycle: LifecycleParts<'a>,
    pub(crate) workbench: &'a mut WorkbenchSession,
    pub(crate) facts: SlotFacts,
}

impl LifecycleSource for SaveParts<'_> {
    fn lifecycle_view(&self) -> LifecycleView<'_> {
        self.lifecycle.lifecycle_view()
    }
}

impl LifecycleSourceMut for SaveParts<'_> {
    fn lifecycle_parts(&mut self) -> LifecycleParts<'_> {
        self.lifecycle.lifecycle_parts()
    }
}

/// A save the preference asks to be checked runs the model's design checks,
/// stating their prompt on the status bar.
impl LifecycleHost for SaveParts<'_> {
    fn run_active_design_checks(&mut self) -> Result<rspice_design::drc::DrcResult, String> {
        let (outcome, prompt) = self.lifecycle.model.run_active_design_checks(self.facts);
        if let Some(prompt) = prompt {
            self.workbench.prompt(prompt);
        }
        outcome
    }
}

/// Where a save writes on the desktop: refused for a live mirror, the open
/// project's own file, or the file the reader picks.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn native_save_destination(
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    platform: &PlatformState,
) -> Result<(std::path::PathBuf, DestinationAuthority), SaveRequestOutcome> {
    if let Some(message) = live_mirror_save_block(model, false) {
        workbench.prompt(message);
        return Err(SaveRequestOutcome::Failed(message.to_owned()));
    }
    if let Some(path) =
        crate::workbench::lifecycle::project_lifecycle::canonical_native_path(platform)
    {
        return Ok((path, DestinationAuthority::Canonical));
    }
    let default_name = project_save_dialog_default_name(model, platform);
    match crate::io::show_save_project_dialog(Some(&default_name)) {
        Ok(path) => Ok((path, DestinationAuthority::UserSelected)),
        Err(ProjectIoError::Cancelled) => Err(SaveRequestOutcome::Cancelled),
        Err(error) => {
            let message = error.to_string();
            model.report_failure("Project save failed", message.clone());
            Err(SaveRequestOutcome::Failed(message))
        }
    }
}

/// Save `scope` to `path` under `authority`, then remember the file and
/// record the save, or answer why it failed.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn save_native_scope(
    mut parts: SaveParts<'_>,
    scope: SaveScope,
    path: &Path,
    authority: DestinationAuthority,
) -> Result<(), String> {
    match crate::workbench::lifecycle::project_lifecycle::save_native(
        &mut parts, scope, path, authority,
    ) {
        Ok(()) => {
            let LifecycleParts {
                model, platform, ..
            } = parts.lifecycle;
            let canonical =
                crate::workbench::lifecycle::project_lifecycle::canonical_native_path(platform)
                    .unwrap_or_else(|| path.to_path_buf());
            platform.browser_project_save_name = None;
            remember_recent_file(
                model,
                platform,
                crate::services::recent_files::RecentKind::Project,
                &canonical,
            );
            model.record(
                LogSeverity::Info,
                LogSource::Studio,
                format!("Saved project: {}", canonical.display()),
            );
            Ok(())
        }
        Err(error) => {
            let message = error.to_string();
            report_lifecycle_error(
                parts.lifecycle.model,
                parts.workbench,
                error,
                "Project save failed",
            );
            Err(message)
        }
    }
}

/// Save `scope` through the browser, refused for a live mirror.
#[cfg(target_arch = "wasm32")]
pub(crate) fn save_scope_in_browser(parts: SaveParts<'_>, scope: SaveScope) -> SaveRequestOutcome {
    if let Some(message) = live_mirror_save_block(parts.lifecycle.model, false) {
        parts.workbench.prompt(message);
        return SaveRequestOutcome::Failed(message.to_owned());
    }
    start_browser_project_save(parts, scope, false)
}

/// Save a copy of the project through the browser. Returns whether the save
/// started.
#[cfg(target_arch = "wasm32")]
pub(crate) fn save_project_copy_in_browser(parts: SaveParts<'_>) -> bool {
    start_browser_project_save(parts, SaveScope::AllDocuments, true).request_started()
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
    mut parts: SaveParts<'_>,
    scope: SaveScope,
    project_copy: bool,
) -> SaveRequestOutcome {
    let suggested_name =
        project_save_dialog_default_name(parts.lifecycle.model, parts.lifecycle.platform);
    let mut prepared = match crate::workbench::lifecycle::project_lifecycle::prepare_browser_save(
        &mut parts,
        scope,
        project_copy,
        suggested_name,
    ) {
        Ok(prepared) => prepared,
        Err(error) => {
            let message = error.to_string();
            report_lifecycle_error(
                parts.lifecycle.model,
                parts.workbench,
                error,
                "Project save failed",
            );
            return SaveRequestOutcome::Failed(message);
        }
    };
    let LifecycleParts {
        model, platform, ..
    } = parts.lifecycle;

    let save_surface_supported = browser_save_picker_is_safe(
        project_copy,
        prepared.source_handle_id,
        if project_copy {
            crate::workbench::lifecycle::project_lifecycle::browser_file_picker_supported()
        } else {
            crate::workbench::lifecycle::project_lifecycle::browser_canonical_save_supported()
        },
    );
    if prepared.handle_id.is_none()
        && prepared.save.intent().backend
            == crate::workbench::lifecycle::project_lifecycle::BrowserBindingBackend::ExternalFile
        && !save_surface_supported
    {
        let text = match String::from_utf8(prepared.save.take_bytes()) {
            Ok(text) => text,
            Err(error) => {
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                    platform,
                    prepared.save.transaction(),
                );
                model.report_failure(
                    "Project copy failed",
                    format!("the saved project was not UTF-8: {error}"),
                );
                return SaveRequestOutcome::Failed(format!(
                    "Serialized project was not UTF-8: {error}"
                ));
            }
        };
        let path = std::path::PathBuf::from(&prepared.suggested_name);
        match crate::workbench::browser::download::download_text_file(&path, &text) {
            Ok(()) => {
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                    platform,
                    prepared.save.transaction(),
                );
                if project_copy {
                    model.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        "Downloaded a copy of the project; the open project is unchanged.",
                    );
                } else {
                    model.report_warning(
                        "Saved as a download instead",
                        "The project was downloaded as a copy, not saved; it still has unsaved \
                         changes.",
                    );
                }
                return SaveRequestOutcome::CopyOnly;
            }
            Err(error) => {
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                    platform,
                    prepared.save.transaction(),
                );
                model.report_failure("Project download failed", error.clone());
                return SaveRequestOutcome::Failed(error);
            }
        }
    }

    let target = crate::workbench::lifecycle::project_lifecycle::BrowserWriteTarget {
        handle_id: prepared.handle_id,
        intent: prepared.save.intent().clone(),
    };
    let name = prepared.suggested_name.clone();
    let bytes = prepared.save.take_bytes();
    let transaction = prepared.save.transaction();
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
                platform,
                transaction,
            );
            model.report_failure("Project save failed", error);
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
thread_local! {
    static BROWSER_PROJECT_SAVE_RESULTS: std::cell::RefCell<std::collections::VecDeque<BrowserProjectSaveCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn poll_browser_project_save(state: &mut AppState) -> Option<SaveContinuationEvent> {
    let completion = BROWSER_PROJECT_SAVE_RESULTS.with(|queue| queue.borrow_mut().pop_front())?;
    let project_copy = completion.prepared.save.is_project_copy();
    let transaction = completion.prepared.save.transaction();
    let saved_scope = completion.prepared.save.scope();
    let saved_document = completion.prepared.save.saved_document().clone();
    if !crate::workbench::lifecycle::project_lifecycle::browser_operation_context_is_current(
        &state.model,
        &state.platform,
        completion.prepared.save.context(),
    ) {
        let terminal = match &completion.result {
            crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::Saved { .. }
            | crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::SavedSessionOnly { .. } => {
                SaveContinuationEvent::PublishedButNotAdopted(
                    transaction,
                    "The project was saved after this tab lost control of it, so this tab did not take that save as its current version. Reopen the project before continuing."
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
                SaveContinuationEvent::Failed(transaction, error.to_string())
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
                &state.platform,
                *handle_id,
            ),
            crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::Cancelled
            | crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::ExternalChange { .. }
            | crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::Failed(_) => {}
        }
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
            &mut state.platform,
            transaction,
        );
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
            handle_id,
            rspice_project::lifecycle::BrowserSavePublication {
                receipt: rspice_project::persistence::BrowserBindingReceipt {
                    binding_id,
                    backend,
                    project_id,
                    accepted_generation: generation,
                    accepted_digest: digest,
                },
                display_name: display_name.clone(),
                durable: true,
            },
        ) {
            Ok(()) => {
                accept_browser_canonical_display_name(
                    &mut state.platform.browser_project_save_name,
                    project_copy,
                    &display_name,
                );
                state.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    if project_copy {
                        format!(
                            "Saved a copy of the project: {display_name}; the open project is \
                             unchanged."
                        )
                    } else {
                        format!("Saved project: {display_name}")
                    },
                );
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
            handle_id,
            rspice_project::lifecycle::BrowserSavePublication {
                receipt: rspice_project::persistence::BrowserBindingReceipt {
                    binding_id,
                    backend,
                    project_id,
                    accepted_generation: generation,
                    accepted_digest: digest,
                },
                display_name: display_name.clone(),
                durable: false,
            },
        ) {
            Ok(()) => {
                accept_browser_canonical_display_name(
                    &mut state.platform.browser_project_save_name,
                    project_copy,
                    &display_name,
                );
                if project_copy {
                    state.record(
                        LogSeverity::Warning,
                        LogSource::Studio,
                        format!(
                            "Saved a copy of the project: {display_name}; browser storage was \
                             unavailable: {persistence_error}"
                        ),
                    );
                } else {
                    state.model.report_warning(
                        "Project will not reopen on restart",
                        format!(
                            "Saved {display_name}, but browser storage failed: \
                             {persistence_error}"
                        ),
                    );
                }
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
                &mut state.platform,
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
                &state.model,
                &mut state.platform,
                &completion.prepared,
                observed_digest,
            );
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                &mut state.platform,
                transaction,
            );
            state.report_failure(
                "Project save failed",
                "the project changed outside RSpice; reopen it or save a copy.",
            );
            if !project_copy {
                continuation = Some(SaveContinuationEvent::Conflict(transaction));
            }
        }
        crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult::Failed(error) => {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                &mut state.platform,
                transaction,
            );
            state.report_failure("Project save failed", error.to_string());
            if !project_copy {
                continuation = Some(SaveContinuationEvent::Failed(
                    transaction,
                    error.to_string(),
                ));
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
    if !state.commit_pending_inspector_edit() {
        return false;
    }
    if let Some(message) = live_mirror_save_block(&state.model, true) {
        state.prompt(message);
        return false;
    }
    let default_name = project_save_dialog_default_name(&state.model, &state.platform);

    #[cfg(not(target_arch = "wasm32"))]
    match crate::io::show_save_project_dialog(Some(default_name.as_str())) {
        Ok(path) => save_project_as_to_path(state, &path),
        Err(ProjectIoError::Cancelled) => false,
        Err(error) => {
            state.report_failure("Project Save As failed", error.to_string());
            false
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let _ = default_name;
        state.save_project_copy_in_browser()
    }
}

/// Save the project at `path` and carry on working there.
///
/// The file is written under the project's copy rules (its own project
/// identity, never over the open file), then opened in place of the project
/// it came from, so the project location, the recent list, the window title
/// and the saved session all name the new file. The file it came from keeps
/// what was last saved to it.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn save_project_as_to_path(state: &mut AppState, path: &Path) -> bool {
    if let Err(error) =
        crate::workbench::lifecycle::project_lifecycle::save_project_copy_native(state, path)
    {
        lifecycle_error(state, error, "Project Save As failed");
        return false;
    }
    if !load_project_from_path(state, path) {
        return false;
    }
    state.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!("Saved the project as {}.", path.display()),
    );
    true
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_file_operation_label(state: &AppState) -> Option<String> {
    if let Some(kind) = crate::workbench::browser::file_import::active_text_import_kind() {
        return Some(format!(
            "Waiting for the browser {} picker or permission request to finish.",
            kind.label()
        ));
    }
    crate::workbench::lifecycle::project_lifecycle::operation_in_progress(&state.platform)
        .then(|| "Waiting for the browser project file or permission request to finish.".to_owned())
}

/// Cancel app-side authority for an unresolved browser file operation. The
/// underlying JavaScript promise may still resolve, but transaction and import
/// lease generations make that completion unable to mutate current state.
#[cfg(target_arch = "wasm32")]
pub(crate) fn cancel_pending_browser_file_operation(
    platform: &mut PlatformState,
    workbench: &mut WorkbenchSession,
) -> bool {
    let import_cancelled =
        crate::workbench::browser::file_import::cancel_active_text_import().is_some();
    let lifecycle_cancelled =
        crate::workbench::lifecycle::project_lifecycle::cancel_pending_browser_operation(platform);
    let continuation_cancelled = workbench
        .confirmation_dialog
        .cancel_awaiting_canonical_save();
    let cancelled = import_cancelled || lifecycle_cancelled || continuation_cancelled;
    if cancelled {
        workbench.prompt("Cancelled the browser file operation.");
    }
    cancelled
}

pub(crate) fn request_revert_active_document(state: &mut AppState) -> bool {
    match crate::workbench::lifecycle::project_lifecycle::prepare_revert_active_document(state) {
        Ok(token) => {
            state.workbench.project_review_dialog.show_revert(token);
            true
        }
        Err(error) => {
            lifecycle_error(state, error, "Revert review unavailable");
            false
        }
    }
}

/// Say how the reverted document's revert went: record it, or answer the
/// refusal or failure. Returns whether it was reverted.
pub(crate) fn report_revert(
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    token: &crate::workbench::lifecycle::project_lifecycle::RevertReviewToken,
    reverted: Result<(), ProjectLifecycleError>,
) -> bool {
    match reverted {
        Ok(()) => {
            model.record(
                LogSeverity::Info,
                LogSource::Studio,
                format!("Reverted {}.", token.document_label()),
            );
            true
        }
        Err(error) => {
            report_lifecycle_error(model, workbench, error, "Revert failed");
            false
        }
    }
}

pub(crate) fn close_active_document(state: &mut AppState) -> bool {
    match crate::workbench::app_state::services_access::close_active_document(state) {
        Ok(()) => {
            state.prompt("Closed the document; the project keeps its data.");
            true
        }
        Err(error) => {
            lifecycle_error(state, error, "Close document failed");
            false
        }
    }
}

/// Close the project, asking first only when something would be lost: an
/// unsaved document or model candidate, or a local run that has to stop. A
/// clean, idle project closes at once. Either way the close leaves the
/// application on the no-project landing, not in the project launcher.
pub(crate) fn request_close_project(state: &mut AppState) -> bool {
    if !state.platform.project_lifecycle.is_open() {
        lifecycle_error(
            state,
            ProjectLifecycleError::NoProject,
            "Close project unavailable",
        );
        return false;
    }
    let destination = ProjectCloseDestination::EmptyWorkbench;
    if crate::workbench::app::dialogs::project::configuration_dialogs::hold_for_configuration_edits(
        state,
        crate::workbench::app::ConfirmationAction::CloseProject(destination),
        None,
    ) {
        return true;
    }
    if closing_loses_nothing(state) {
        return state.close_project_discard(destination);
    }
    state
        .workbench
        .project_review_dialog
        .show_close_project(destination);
    true
}

/// Whether closing the open project loses nothing: no unsaved document or
/// model candidate, no configuration edits left unapplied, and no local run
/// that would have to stop. A project never saved to a file always has
/// something to lose.
pub(crate) fn closing_loses_nothing(state: &AppState) -> bool {
    !state.model.simulation.execution.has_active_execution()
        && !ModelsPort::model_editor_has_unsaved_changes(&*state.models)
        && !state
            .project
            .configuration
            .is_dirty(&state.model.workspace.content.configuration_sets)
        && crate::workbench::lifecycle::project_lifecycle::dirty_documents(state).is_empty()
}

/// Whether Projects and files ▸ On startup keeps the project the restored
/// session reopened: it does when the reader reopens the last project, and
/// there is nothing to close when no project was reopened.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn startup_keeps_project(model: &Model, platform: &PlatformState) -> bool {
    model.preferences.reopen_last_project() || !platform.project_lifecycle.is_open()
}

/// Record why "Start with no project" left the reopened project open: it has
/// unsaved changes.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn report_startup_project_kept(model: &mut Model, platform: &PlatformState) {
    if platform.project_lifecycle.accepted().is_some() {
        model.record(
            LogSeverity::Info,
            LogSource::Studio,
            "The last project was reopened because it has unsaved changes.",
        );
    }
}

/// Refuse to close the open project while a run is in progress, answering
/// the gesture with the refusal. Returns whether it refused.
pub(crate) fn refuse_project_close(model: &mut Model, workbench: &mut WorkbenchSession) -> bool {
    if model.simulation.execution.has_active_execution() {
        report_lifecycle_error(
            model,
            workbench,
            ProjectLifecycleError::ActiveRun,
            "Close project blocked",
        );
        return true;
    }
    false
}

/// The empty project a close leaves open, built off to the side, with the
/// announcement that retires the closed project's execution context.
pub(crate) struct ClosedProject {
    pub(crate) library_manager: crate::state::LibraryManager,
    pub(crate) workspace: crate::state::ProjectWorkspace,
    pub(crate) announcement: String,
}

/// Build the empty project a close of the open one leaves behind.
pub(crate) fn build_closed_project(model: &Model) -> ClosedProject {
    let announcement = format!(
        "Closed project {}.",
        model.workspace.content.project.display_name()
    );
    let mut library_manager = crate::state::LibraryManager::with_primitives();
    let workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut library_manager);
    ClosedProject {
        library_manager,
        workspace,
        announcement,
    }
}

/// Give the empty project a close leaves open the session's first plan and
/// the session's PDK model catalog, restored again. It has never been
/// saved, so it has no browser save name. Returns why the catalog could not
/// be restored, reported once the close has stated itself.
pub(crate) fn settle_closed_project(
    model: &mut Model,
    platform: &mut PlatformState,
) -> Vec<String> {
    model.sim_setup =
        crate::workbench::app_state::SimSetupState::new_with_user_preferences(&model.preferences);
    let restored_model_catalog =
        crate::workbench::app_state::restore_session_model_library_manager(&mut model.pdk_config);
    let (model_library_manager, model_restore_errors) = match restored_model_catalog {
        Ok((manager, _loaded)) => (manager, Vec::new()),
        Err(errors) => (
            crate::workbench::app_state::default_model_library_manager(),
            errors,
        ),
    };
    model.model_library_manager = model_library_manager;
    platform.browser_project_save_name = None;
    model_restore_errors
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
            let canonical_path = binding.canonical_path.clone();
            state.apply_loaded_project_authorized(
                project,
                ProjectLoadOrigin::PersistentPath(&canonical_path),
                Some(binding),
                transaction,
            )
        }
        Err(error) => {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction(&mut state.platform);
            state.report_failure("Project open failed", error.to_string());
            false
        }
    }
}

#[cfg(test)]
pub(crate) fn apply_loaded_project(
    state: &mut AppState,
    project: ProjectSnapshot,
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
    state.apply_loaded_project_authorized(project, origin, None, transaction)
}

/// A project read for opening, validated and taken apart before it replaces
/// the open one.
pub(crate) struct LoadedProject {
    /// Retires the replaced project's execution context.
    pub(crate) announcement: String,
    pub(crate) install: LoadedProjectInstall,
    pub(crate) restore: LoadedProjectRestore,
}

/// What a loaded project installs as the open one.
pub(crate) struct LoadedProjectInstall {
    library_manager: crate::state::LibraryManager,
    workspace: crate::state::ProjectWorkspace,
    sim_setup: crate::workbench::app_state::SimSetupState,
    model_library_manager: crate::state::model_library::ModelLibraryManager,
}

/// What a loaded project brings back once it is installed.
pub(crate) struct LoadedProjectRestore {
    simulation_results: crate::io::ProjectSimulationResults,
    result_presentation: crate::state::result_presentation::ResultPresentation,
    accepted_baseline: ProjectSnapshot,
    simulation_results_warning: Option<String>,
    workspace_migration_warning: Option<String>,
    execution_warnings: Vec<String>,
}

/// Validate a project read for opening against the replacement
/// `transaction` reserved for it, restore the execution context it was
/// saved with, and settle where it saves next. A refusal ends the
/// transaction and answers the open, and nothing is installed.
pub(crate) fn prepare_loaded_project(
    lifecycle: crate::workbench::lifecycle::project_lifecycle::LifecycleParts<'_>,
    workbench: &mut WorkbenchSession,
    mut project: ProjectSnapshot,
    origin: ProjectLoadOrigin<'_>,
    transaction: crate::product::TransactionId,
) -> Option<LoadedProject> {
    if let Err(error) = crate::workbench::lifecycle::project_lifecycle::validate_project_replacement(
        &lifecycle,
        transaction,
    ) {
        #[cfg(target_arch = "wasm32")]
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
            lifecycle.platform,
            transaction,
        );
        #[cfg(not(target_arch = "wasm32"))]
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction(lifecycle.platform);
        report_lifecycle_error(lifecycle.model, workbench, error, "Project open blocked");
        return None;
    }
    let crate::workbench::lifecycle::project_lifecycle::LifecycleParts {
        model, platform, ..
    } = lifecycle;
    let accepted_execution_context = project.file.execution_context.clone();
    let project_id = project.file.workspace.project.id();
    let (simulation_plan, model_library_manager, execution_warnings) = match project
        .file
        .execution_context
        .take()
    {
        Some(context) => match crate::io::restore_execution_context(context, project_id) {
            Ok(restored) => restored,
            Err(error) => {
                model.report_failure(
                    "Project open failed",
                    format!("its saved simulation setup is invalid: {error}"),
                );
                #[cfg(target_arch = "wasm32")]
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                    platform,
                    transaction,
                );
                #[cfg(not(target_arch = "wasm32"))]
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction(platform);
                return None;
            }
        },
        None => {
            let (manager, mut warnings) =
                match crate::workbench::app_state::restore_session_model_library_manager(
                    &mut model.pdk_config,
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
                    &model.preferences,
                ),
                manager,
                warnings,
            )
        }
    };
    project.ensure_library_model();
    match origin {
        #[cfg(not(target_arch = "wasm32"))]
        ProjectLoadOrigin::PersistentPath(_) => {
            platform.browser_project_save_name = None;
        }
        ProjectLoadOrigin::LiveSession(_) => {
            // The host's on-disk location is meaningless on this machine and
            // must never become a save target here.
            project.file.workspace.project.path = None;
            platform.browser_project_save_name = None;
        }
        #[cfg(any(test, target_arch = "wasm32"))]
        ProjectLoadOrigin::BrowserImport(name) | ProjectLoadOrigin::BrowserCanonical(name) => {
            project.file.workspace.project.path = None;
            platform.browser_project_save_name = Some(name.to_string());
        }
    }
    let mut accepted_baseline = project.clone();
    accepted_baseline.file.execution_context = accepted_execution_context;
    let announcement = format!(
        "{} {} ({}).",
        origin.success_prefix(),
        project.file.workspace.project.display_name(),
        origin.display_label()
    );
    Some(LoadedProject {
        announcement,
        install: LoadedProjectInstall {
            library_manager: project.file.libraries,
            workspace: crate::state::ProjectWorkspace::from_parts(
                project.file.workspace,
                project.workspace_session,
            ),
            sim_setup: simulation_plan,
            model_library_manager,
        },
        restore: LoadedProjectRestore {
            simulation_results: project.file.simulation_results,
            result_presentation: project.file.result_presentation,
            accepted_baseline,
            simulation_results_warning: project.file.simulation_results_warning,
            workspace_migration_warning: project.file.workspace_migration_warning,
            execution_warnings,
        },
    })
}

/// Make a loaded project's libraries, workspace, plan and model catalog the
/// open ones, under fresh library edit locks.
pub(crate) fn install_loaded_project(model: &mut Model, install: LoadedProjectInstall) {
    model.library_manager = install.library_manager;
    model.library_edit_locks = crate::state::ProjectLibraryLockAuthority::default();
    model.workspace = install.workspace;
    model.sim_setup = install.sim_setup;
    model.model_library_manager = install.model_library_manager;
}

/// What an installed project's restore leaves for the shell and the report.
pub(crate) struct OpenedProject {
    pub(crate) result_presentation: crate::state::result_presentation::ResultPresentation,
    pub(crate) accepted_baseline: ProjectSnapshot,
    pub(crate) report: OpenedProjectReport,
}

/// What an opened project has to say once it is open.
pub(crate) struct OpenedProjectReport {
    technology_block: Option<String>,
    workspace_migration_warning: Option<String>,
    simulation_results_warning: Option<String>,
    execution_warnings: Vec<String>,
}

/// Restore an installed project's simulation: a fresh simulation state, the
/// technology gate asked of the installed project, then the results it was
/// saved with.
pub(crate) fn restore_loaded_simulation(
    model: &mut Model,
    restore: LoadedProjectRestore,
) -> OpenedProject {
    let LoadedProjectRestore {
        simulation_results,
        result_presentation,
        accepted_baseline,
        mut simulation_results_warning,
        workspace_migration_warning,
        execution_warnings,
    } = restore;
    model.simulation = crate::state::SimulationState::default();
    // Asked of the one gate a run asks: a project with no technology runs
    // whatever its plan does not need one for. Asking whether it had a
    // binding at all told every such project its runs were blocked, and they
    // were not.
    let technology_block = model
        .technology_inputs()
        .technology_gate_block_reason()
        .err();
    if let Err(error) =
        crate::io::restore_simulation_results(simulation_results, &mut model.simulation)
        && simulation_results_warning.is_none()
    {
        simulation_results_warning = Some(format!(
            "Simulation results were not restored because their persisted data is invalid: {error}"
        ));
    }
    OpenedProject {
        result_presentation,
        accepted_baseline,
        report: OpenedProjectReport {
            technology_block,
            workspace_migration_warning,
            simulation_results_warning,
            execution_warnings,
        },
    }
}

/// Report an opened project: remember the file it came from, then say why
/// its runs are blocked, what its load lost, and what its plan's migration
/// dropped.
pub(crate) fn report_opened_project(
    model: &mut Model,
    platform: &mut PlatformState,
    origin: ProjectLoadOrigin<'_>,
    report: OpenedProjectReport,
) {
    if let Some(path) = origin.recent_path() {
        remember_recent_file(
            model,
            platform,
            crate::services::recent_files::RecentKind::Project,
            path,
        );
    }
    if let Some(error) = report.technology_block {
        model.record(
            LogSeverity::Warning,
            LogSource::Studio,
            format!(
                "Simulation is blocked for this project: {error}. Attach a trusted, signed PDK \
                 revision under Project \u{00b7} Dependencies."
            ),
        );
    }
    for warning in report
        .workspace_migration_warning
        .into_iter()
        .chain(report.simulation_results_warning)
        .chain(report.execution_warnings)
    {
        model.record(LogSeverity::Warning, LogSource::Studio, warning);
    }
    report_legacy_run_set_notes(model);
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
fn report_legacy_run_set_notes(model: &mut Model) {
    for note in std::mem::take(&mut model.sim_setup.legacy_run_set_notes) {
        model.record(LogSeverity::Warning, LogSource::Studio, note);
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
    if state.apply_loaded_project_authorized(
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
            state.report_failure("Project open failed", error.to_string());
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
    let context = crate::workbench::lifecycle::project_lifecycle::browser_operation_context(
        &state.model,
        &state.platform,
    );
    match start_browser_project_import(transaction, context) {
        Ok(()) => {
            state.prompt("Choose an RSpice project file to open.");
            true
        }
        Err(error) => {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction(&mut state.platform);
            state.report_failure("Project open failed", error);
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

/// A project the browser read for opening, for the shell to apply.
#[cfg(target_arch = "wasm32")]
pub(crate) enum BrowserProjectOpen {
    /// A copy imported from a file the reader picked.
    Import {
        project: ProjectSnapshot,
        name: String,
        transaction: crate::product::TransactionId,
    },
    /// The project a browser binding opened, saved back through it.
    Canonical(BrowserCanonicalOpen),
}

/// A project a browser binding opened, with the binding it saves through.
#[cfg(target_arch = "wasm32")]
pub(crate) struct BrowserCanonicalOpen {
    pub(crate) project: ProjectSnapshot,
    pub(crate) display_name: String,
    pub(crate) handle_id: u64,
    pub(crate) binding: PersistenceBinding,
    pub(crate) transaction: crate::product::TransactionId,
}

/// Take the next project the browser finished reading or promoting, if
/// any. A read that is stale, cancelled or failed ends its transaction and
/// opens nothing; a promotion records whether the project will reopen.
#[cfg(target_arch = "wasm32")]
pub(crate) fn poll_browser_project_import(
    model: &mut Model,
    platform: &mut PlatformState,
) -> Option<BrowserProjectOpen> {
    let Some(event) = BROWSER_PROJECT_IMPORT_RESULTS.with(|queue| queue.borrow_mut().pop_front())
    else {
        return None;
    };
    match event {
        BrowserProjectImportResult::CanonicalPromoted {
            context,
            handle_id,
            display_name,
            result,
        } => {
            if crate::workbench::lifecycle::project_lifecycle::complete_browser_binding_promotion(
                model, platform, &context, handle_id, &result,
            ) {
                match result {
                    Ok(()) => {
                        model.record(
                            LogSeverity::Info,
                            LogSource::Studio,
                            format!("{display_name} will reopen when the browser restarts."),
                        );
                    }
                    Err(error) => model.report_warning(
                        "Project will not reopen on restart",
                        format!(
                            "{display_name} will not reopen when the browser restarts: browser \
                             storage failed: {error}"
                        ),
                    ),
                };
            }
            None
        }
        BrowserProjectImportResult::Transaction(completion) => {
            let transaction = completion.transaction;
            if !crate::workbench::lifecycle::project_lifecycle::browser_operation_context_is_current(
                model,
                platform,
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
                    platform,
                    transaction,
                );
                return None;
            }
            match completion.payload {
                BrowserProjectImportPayload::Cancelled => {
                    finish_browser_project_import(completion.import_token);
                    crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                        platform,
                        transaction,
                    );
                    None
                }
                BrowserProjectImportPayload::Failed(error) => {
                    finish_browser_project_import(completion.import_token);
                    crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                        platform,
                        transaction,
                    );
                    model.report_failure("Project open failed", error);
                    None
                }
                BrowserProjectImportPayload::Loaded(file) => {
                    finish_browser_project_import(completion.import_token);
                    match crate::io::project_io::load_project_text(&file.contents, None) {
                        Ok(project) => Some(BrowserProjectOpen::Import {
                            project,
                            name: file.name,
                            transaction,
                        }),
                        Err(error) => {
                            crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                                platform,
                                transaction,
                            );
                            model.report_failure("Project open failed", error.to_string());
                            None
                        }
                    }
                }
                BrowserProjectImportPayload::Canonical(result) => {
                    finish_browser_project_import(completion.import_token);
                    prepare_browser_canonical_open(model, platform, transaction, result)
                        .map(BrowserProjectOpen::Canonical)
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn finish_browser_project_import(token: crate::workbench::browser::file_import::TextImportToken) {
    let _ = crate::workbench::browser::file_import::finish_text_import(token);
}

/// Open the project the browser's binding read, ending the transaction and
/// answering the open when it cannot be read.
#[cfg(target_arch = "wasm32")]
fn prepare_browser_canonical_open(
    model: &mut Model,
    platform: &mut PlatformState,
    transaction: crate::product::TransactionId,
    result: crate::workbench::lifecycle::project_lifecycle::BrowserOpenResult,
) -> Option<BrowserCanonicalOpen> {
    let crate::workbench::lifecycle::project_lifecycle::BrowserOpenResult::Opened {
        handle_id,
        display_name,
        bytes,
    } = result
    else {
        match result {
            crate::workbench::lifecycle::project_lifecycle::BrowserOpenResult::Cancelled => {
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                    platform,
                    transaction,
                );
            }
            crate::workbench::lifecycle::project_lifecycle::BrowserOpenResult::Failed(error) => {
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                    platform,
                    transaction,
                );
                model.report_failure("Project open failed", error.to_string());
            }
            crate::workbench::lifecycle::project_lifecycle::BrowserOpenResult::Opened {
                ..
            } => unreachable!(),
        }
        return None;
    };
    let (project, binding) = match rspice_project::persistence::browser::BrowserBinding::open(
        bytes,
        display_name.clone(),
        crate::state::workspace::WorkspaceSourceFiles,
    ) {
        Ok((project, binding)) => (ProjectSnapshot::from_decoded(project), binding),
        Err(error) => {
            crate::workbench::lifecycle::project_lifecycle::release_browser_handle(handle_id);
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                platform,
                transaction,
            );
            model.report_failure("Project open failed", error.to_string());
            return None;
        }
    };
    Some(BrowserCanonicalOpen {
        project,
        display_name,
        handle_id,
        binding: PersistenceBinding::Browser { handle_id, binding },
        transaction,
    })
}

/// Settle a browser binding's open once the shell has applied it: an open
/// that failed lets the browser's handle go, and an accepted one promotes
/// its binding record so it reopens when the browser restarts. Returns
/// whether the project opened.
#[cfg(target_arch = "wasm32")]
pub(crate) fn settle_browser_canonical_open(
    model: &Model,
    platform: &mut PlatformState,
    opened: bool,
    handle_id: u64,
    display_name: String,
    binding: PersistenceBinding,
) -> bool {
    if !opened {
        crate::workbench::lifecycle::project_lifecycle::release_browser_handle(handle_id);
        return false;
    }
    // The project replacement is accepted before its binding record is
    // promoted. Promotion is separately context/CAS guarded, so IndexedDB can
    // never authorize an open that failed to apply or a project that has since
    // been replaced.
    let context = crate::workbench::lifecycle::project_lifecycle::begin_browser_binding_promotion(
        model, platform,
    );
    crate::workbench::lifecycle::project_lifecycle::start_browser_binding_persist(
        binding,
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

/// Answer a lifecycle command that did not take effect.
///
/// A guard that refused the command (no project, a run in progress, another
/// operation pending) answers the gesture; a project that could not be read
/// or written is a failure the session records and tells the user about.
fn lifecycle_error(state: &mut AppState, error: ProjectLifecycleError, context: &str) {
    report_lifecycle_error(&mut state.model, &mut state.workbench, error, context);
}

/// [`lifecycle_error`] over the two parts it answers through: the model's
/// log and notices, and the status bar.
fn report_lifecycle_error(
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    error: ProjectLifecycleError,
    context: &str,
) {
    match error {
        ProjectLifecycleError::InvalidState(_) | ProjectLifecycleError::Persistence(_) => {
            model.report_failure(context, error.to_string());
        }
        _ => workbench.prompt(format!("{context}: {error}")),
    }
}

#[cfg(test)]
mod tests;
