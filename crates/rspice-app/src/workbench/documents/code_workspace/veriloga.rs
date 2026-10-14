//! Governed in-memory Verilog-A compilation for the Code workspace.

use std::sync::{Arc, Mutex, mpsc};

use rspice_veriloga::{
    RuntimeCompileReport, RuntimeTarget, RuntimeTargetMaturity, RuntimeTargetQualification,
    RuntimeTargetReadiness, VirtualRuntimeCompilation,
};
use sha2::Digest as _;

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    ProjectSourceBundle, ProjectSourceFile, ProjectSourceId, ProjectSourceLanguage,
    ProjectSourceOwner, ProjectSourceRole, ViewType,
};
use crate::workbench::RSpiceApp;

use rspice_simulation::project_veriloga::VerilogASourceOperationToken;

use super::{
    CodeDiagnosticCollection, CodeEditorDiagnostic, CodeEditorSeverity, PendingVerilogACompile,
    TargetQualification, VerilogACompileOutcome, VerilogACompileReceipt, VerilogAFileSelection,
    VerilogAImportFormState, VerilogAReplaceReview, VerilogAReplaceTarget,
};

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
struct BrowserImportCompletion {
    token: crate::workbench::browser::file_import::TextImportToken,
    result: Result<Option<crate::workbench::browser::file_import::PickedTextFile>, String>,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_IMPORT_COMPLETION: std::cell::RefCell<Option<BrowserImportCompletion>> =
        const { std::cell::RefCell::new(None) };
}

/// The exact project-owned Verilog-A closure selected by the current editor:
/// the bundle the active Verilog-A cell view owns, and the module that view
/// binds.
#[derive(Debug, Clone)]
pub(crate) struct SelectedVerilogASource {
    pub(super) bundle: ProjectSourceBundle,
    pub(super) selected_module: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct SelectedVerilogAEditorSnapshot {
    selected: SelectedVerilogASource,
    active_path: String,
    selection_was_invalid: bool,
}

impl SelectedVerilogAEditorSnapshot {
    pub(crate) fn bundle_id(&self) -> ProjectSourceId {
        self.selected.bundle().id()
    }

    pub(crate) fn active_path(&self) -> &str {
        &self.active_path
    }

    pub(crate) const fn selection_was_invalid(&self) -> bool {
        self.selection_was_invalid
    }
}

impl SelectedVerilogASource {
    pub(crate) fn bundle(&self) -> &ProjectSourceBundle {
        &self.bundle
    }

    pub(crate) fn document(&self) -> &crate::state::ProjectSourceDocument {
        self.bundle.root()
    }

    pub(crate) fn selected_module(&self) -> Option<&str> {
        self.selected_module.as_deref()
    }

    pub(crate) fn token(
        &self,
        project_id: crate::product::ProjectId,
    ) -> VerilogASourceOperationToken {
        VerilogASourceOperationToken::capture(project_id, self.bundle(), self.selected_module())
    }

    pub(crate) fn matches_token(
        &self,
        project_id: crate::product::ProjectId,
        token: VerilogASourceOperationToken,
    ) -> bool {
        self.token(project_id) == token
    }
}

pub(crate) fn selected_veriloga_source(app: &RSpiceApp) -> Result<SelectedVerilogASource, String> {
    let no_view = || "Open a Verilog-A cell view to work on its source.".to_owned();
    let owner = super::active_source_owner(&app.state.workspace, ProjectSourceLanguage::VerilogA)
        .ok_or_else(no_view)?;
    let ProjectSourceOwner::CellView { reference } = &owner else {
        return Err(no_view());
    };
    // A view that names no module yet is still its author's to edit: a source
    // migrated before it parsed has no module to name until it does.
    let selected_module = match crate::state::workspace::project_veriloga_binding_for_view(
        &app.state.workspace,
        &app.state.library_manager,
        reference,
    ) {
        Ok(binding) => Some(binding.selected_module().to_owned()),
        Err(_) if view_names_no_module(app, reference) => None,
        Err(error) => return Err(error),
    };
    let bundle = app
        .state
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .ok_or_else(|| {
            format!(
                "{} has no project-owned Verilog-A source bundle.",
                reference.display_path()
            )
        })?
        .clone();
    bundle
        .validate()
        .map_err(|error| format!("The selected Verilog-A source bundle is invalid: {error}"))?;
    Ok(SelectedVerilogASource {
        bundle,
        selected_module,
    })
}

/// Whether the Verilog-A view at `reference` exists and states no module.
fn view_names_no_module(app: &RSpiceApp, reference: &crate::state::CellViewRef) -> bool {
    app.state
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| Some((cell, cell.get_view(&reference.view)?)))
        .is_some_and(|(cell, view)| {
            view.view_type == ViewType::VerilogA
                && crate::state::workspace::metadata_value(
                    [&view.metadata, &cell.metadata],
                    &["veriloga.module", "netlist.module"],
                )
                .is_none()
        })
}

pub(crate) fn selected_veriloga_editor_snapshot(
    app: &RSpiceApp,
) -> Result<SelectedVerilogAEditorSnapshot, String> {
    let selected = selected_veriloga_source(app)?;
    let explicit = app.state.ui.code_workspace.veriloga.selected_file.as_ref();
    let selection_was_invalid = explicit.is_some_and(|selection| {
        selection.bundle_id != selected.bundle().id()
            || !selected.bundle().contains_file(&selection.logical_path)
    });
    let active_path = explicit
        .filter(|selection| {
            selection.bundle_id == selected.bundle().id()
                && selected.bundle().contains_file(&selection.logical_path)
        })
        .map(|selection| selection.logical_path.clone())
        .unwrap_or_else(|| selected.document().logical_path().to_owned());
    Ok(SelectedVerilogAEditorSnapshot {
        selected,
        active_path,
        selection_was_invalid,
    })
}

/// Consume an import request raised by the File menu. Native and browser
/// pickers converge on the same exact UTF-8 project-source transaction.
pub fn poll_veriloga_import(app: &mut RSpiceApp) {
    #[cfg(target_arch = "wasm32")]
    poll_browser_import_completion(app);

    if !app.state.ui.code_workspace.veriloga.import_requested {
        return;
    }
    app.state.ui.code_workspace.veriloga.import_requested = false;

    #[cfg(not(target_arch = "wasm32"))]
    {
        let dependency_import = app.state.ui.code_workspace.veriloga.import_target.is_some();
        let replacement = app
            .state
            .ui
            .code_workspace
            .veriloga
            .replace_target
            .is_some();
        let extensions: &[&str] = if dependency_import || replacement {
            &["va", "vams"]
        } else {
            &["va"]
        };
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Verilog-A", extensions)
            .set_title(if dependency_import {
                "Import Verilog-A dependency"
            } else if replacement {
                "Replace Verilog-A source from file"
            } else {
                "Import Verilog-A source"
            })
            .pick_file()
        else {
            app.state.ui.code_workspace.veriloga.import_target = None;
            app.state.ui.code_workspace.veriloga.replace_target = None;
            return;
        };
        let result = (|| {
            let metadata = std::fs::metadata(&path).map_err(|error| error.to_string())?;
            if metadata.len() > crate::state::MAX_PROJECT_CODE_SOURCE_BYTES as u64 {
                return Err(format!(
                    "Selected source exceeds the supported {}-byte limit",
                    crate::state::MAX_PROJECT_CODE_SOURCE_BYTES
                ));
            }
            let bytes = std::fs::read(&path).map_err(|error| error.to_string())?;
            if bytes.len() > crate::state::MAX_PROJECT_CODE_SOURCE_BYTES {
                return Err(format!(
                    "Selected source exceeds the supported {}-byte limit",
                    crate::state::MAX_PROJECT_CODE_SOURCE_BYTES
                ));
            }
            let contents = String::from_utf8(bytes)
                .map_err(|error| format!("Selected source is not valid UTF-8: {error}"))?;
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .filter(|name| !name.trim().is_empty())
                .ok_or_else(|| "Selected source has no valid UTF-8 file name".to_owned())?;
            Ok((file_name.to_owned(), contents))
        })();
        match result {
            Ok((file_name, contents)) => apply_import(app, file_name, contents),
            Err(error) => {
                app.state.ui.code_workspace.veriloga.import_target = None;
                app.state.ui.code_workspace.veriloga.replace_target = None;
                app.state.push_user_message(ConsoleMessage::error(format!(
                    "Verilog-A import failed: {error}"
                )));
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let dependency_import = app.state.ui.code_workspace.veriloga.import_target.is_some()
            || app
                .state
                .ui
                .code_workspace
                .veriloga
                .replace_target
                .is_some();
        let token = match crate::workbench::browser::file_import::try_begin_text_import(
            crate::workbench::browser::file_import::BrowserTextImportKind::VerilogA,
        ) {
            Ok(token) => token,
            Err(error) => {
                app.state.ui.code_workspace.veriloga.import_target = None;
                app.state.ui.code_workspace.veriloga.replace_target = None;
                app.state.push_user_message(ConsoleMessage::error(format!(
                    "Verilog-A import failed: {error}"
                )));
                return;
            }
        };
        let extensions: &[&str] = if dependency_import {
            &["va", "vams"]
        } else {
            &["va"]
        };
        crate::workbench::browser::file_import::pick_text_file(
            "Verilog-A",
            extensions,
            move |result| {
                if crate::workbench::browser::file_import::text_import_is_current(token) {
                    BROWSER_IMPORT_COMPLETION.with(|slot| {
                        *slot.borrow_mut() = Some(BrowserImportCompletion { token, result });
                    });
                }
            },
        );
    }
}

#[cfg(target_arch = "wasm32")]
fn poll_browser_import_completion(app: &mut RSpiceApp) {
    let Some(completion) = BROWSER_IMPORT_COMPLETION.with(|slot| slot.borrow_mut().take()) else {
        return;
    };
    if !crate::workbench::browser::file_import::finish_text_import(completion.token) {
        return;
    }
    match completion.result {
        Ok(Some(file)) => apply_import(app, file.name, file.contents),
        Ok(None) => {
            app.state.ui.code_workspace.veriloga.import_target = None;
            app.state.ui.code_workspace.veriloga.replace_target = None;
        }
        Err(error) => {
            app.state.ui.code_workspace.veriloga.import_target = None;
            app.state.ui.code_workspace.veriloga.replace_target = None;
            app.state.push_user_message(ConsoleMessage::error(format!(
                "Verilog-A import failed: {error}"
            )));
        }
    }
}

fn apply_import(app: &mut RSpiceApp, file_name: String, contents: String) {
    // `apply_import` is also the single completion point used by deterministic
    // tests and future non-picker import surfaces. Consume the request here as
    // well as in the polling adapter so no successful, failed, or stale
    // completion can leave the workspace permanently locked in "pending".
    app.state.ui.code_workspace.veriloga.import_requested = false;
    if let Some(target) = app.state.ui.code_workspace.veriloga.import_target.take() {
        app.state.ui.code_workspace.veriloga.replace_target = None;
        let target_is_current = app.state.workspace.content.project.id() == target.project_id
            && app
                .state
                .workspace
                .content
                .project_sources
                .get_bundle(target.bundle_id)
                .is_some_and(|bundle| {
                    bundle.revision().get() == target.bundle_revision
                        && bundle.closure_digest() == target.closure_digest
                        && bundle.contains_file(&target.importer_path)
                });
        if !target_is_current {
            app.state.push_user_message(ConsoleMessage::error(
                "Verilog-A dependency import was discarded because its project source changed while the picker was open.",
            ));
            return;
        }
        let logical_path = suggested_import_path(&target.importer_path, &file_name);
        let result = add_bundle_file(
            app,
            target.bundle_id,
            &target.importer_path,
            logical_path.clone(),
            contents,
        );
        match result {
            Ok(true) => app.state.push_user_message(ConsoleMessage::info(format!(
                "Imported project-owned Verilog-A dependency {logical_path}."
            ))),
            Ok(false) => {}
            Err(error) => app.state.push_user_message(ConsoleMessage::error(format!(
                "Verilog-A dependency import failed: {error}"
            ))),
        }
        return;
    }
    let Some(target) = app.state.ui.code_workspace.veriloga.replace_target.take() else {
        // File > Import never touches an open view: the file becomes cell
        // views of its own, and the form says which, and where, before any is
        // made. That holds whether or not a Verilog-A view is in front.
        open_veriloga_import_form(app, file_name, contents);
        return;
    };
    // "Replace source from file…" replaces nothing here either: the picked
    // file waits for the reader to confirm which file it replaces.
    let Some(bundle) = current_replace_bundle(app, &target) else {
        app.state.push_user_message(ConsoleMessage::error(format!(
            "Replacing {} from {file_name} was discarded because the source changed while the picker was open.",
            target.logical_path
        )));
        return;
    };
    let restorable = replacement_keeps_previous_text(bundle, &target.logical_path, &contents);
    app.state.ui.code_workspace.veriloga.replace_review = Some(VerilogAReplaceReview {
        target,
        file_name,
        contents,
        restorable,
    });
}

/// What the picker delivers when a file is chosen, for tests that cannot
/// open a native picker.
#[cfg(test)]
pub(crate) fn complete_veriloga_file_pick(app: &mut RSpiceApp, file_name: &str, contents: &str) {
    apply_import(app, file_name.to_owned(), contents.to_owned());
}

/// Read `contents` into File > Import's form.
fn open_veriloga_import_form(app: &mut RSpiceApp, file_name: String, contents: String) {
    app.state.ui.code_workspace.veriloga.import_form =
        Some(VerilogAImportFormState::read(app, file_name, contents));
}

/// The bundle a replacement was asked for, while it is exactly the revision
/// the editor showed and still holds the file.
fn current_replace_bundle<'a>(
    app: &'a RSpiceApp,
    target: &VerilogAReplaceTarget,
) -> Option<&'a ProjectSourceBundle> {
    (target.project_id == app.state.workspace.content.project.id())
        .then(|| {
            app.state
                .workspace
                .content
                .project_sources
                .get_bundle(target.bundle_id)
        })
        .flatten()
        .filter(|bundle| {
            bundle.revision().get() == target.bundle_revision
                && bundle.closure_digest() == target.closure_digest
                && bundle.contains_file(&target.logical_path)
        })
}

/// Whether replacing `logical_path` with `contents` leaves the text it holds
/// now in the bundle's revision history, where the source history restores
/// it. Measured on a copy, by doing the replacement.
fn replacement_keeps_previous_text(
    bundle: &ProjectSourceBundle,
    logical_path: &str,
    contents: &str,
) -> bool {
    let Some(previous) = bundle.file_content(logical_path) else {
        return false;
    };
    let mut replaced = bundle.clone();
    replaced
        .replace_file_content(logical_path, contents.to_owned())
        .is_ok_and(|changed| changed)
        && replaced
            .revision_snapshot(bundle.revision())
            .and_then(|snapshot| snapshot.file_content(logical_path))
            == Some(previous)
}

/// Ask for a file to replace the one the editor is showing.
///
/// Opens the picker bound to that exact file; the replacement itself waits
/// for the confirmation the picked file opens.
pub(crate) fn request_veriloga_source_replacement(
    app: &mut RSpiceApp,
    logical_path: &str,
) -> Result<(), String> {
    if let Some(reason) =
        super::source_file_mutation_block_reason(app, ProjectSourceLanguage::VerilogA)
    {
        return Err(reason.to_owned());
    }
    if app.state.ui.code_workspace.veriloga.import_requested {
        return Err("A Verilog-A file is already being chosen.".to_owned());
    }
    let selected = selected_veriloga_source(app)?;
    if !super::source_bundle_document_is_editable(
        app,
        ProjectSourceLanguage::VerilogA,
        selected.bundle().id(),
        logical_path,
    ) {
        return Err(format!("{logical_path} cannot be edited."));
    }
    app.state.ui.code_workspace.veriloga.replace_target = Some(VerilogAReplaceTarget {
        project_id: app.state.workspace.content.project.id(),
        view: app.state.workspace.content.active_view.clone(),
        bundle_id: selected.bundle().id(),
        bundle_revision: selected.bundle().revision().get(),
        closure_digest: selected.bundle().closure_digest(),
        logical_path: logical_path.to_owned(),
    });
    app.state.ui.code_workspace.veriloga.import_target = None;
    app.state.ui.code_workspace.veriloga.import_requested = true;
    Ok(())
}

/// Replace the file a confirmed review names with the picked file, when that
/// file is still exactly what the editor showed.
pub(crate) fn commit_veriloga_source_replacement(app: &mut RSpiceApp) -> Result<(), String> {
    let review = app
        .state
        .ui
        .code_workspace
        .veriloga
        .replace_review
        .take()
        .ok_or_else(|| "No source replacement is waiting for confirmation.".to_owned())?;
    if current_replace_bundle(app, &review.target).is_none() {
        return Err(format!(
            "{} changed after the replacement was chosen; nothing was replaced.",
            review.target.logical_path
        ));
    }
    let changed = app
        .state
        .workspace
        .content
        .project_sources
        .replace_bundle_file_content(
            review.target.bundle_id,
            &review.target.logical_path,
            review.contents,
        )
        .map_err(|error| error.to_string())?;
    if changed {
        invalidate_veriloga_evidence(app, review.target.bundle_id);
    }
    app.state
        .push_user_message(ConsoleMessage::info(if changed {
            format!(
                "Replaced {} with {}; compile is required.",
                review.target.logical_path, review.file_name
            )
        } else {
            format!(
                "{} already holds the text of {}.",
                review.target.logical_path, review.file_name
            )
        }));
    Ok(())
}

impl VerilogAImportFormState {
    /// The form for `contents`, read once: the top modules the file declares,
    /// and the library the active design lives in, when it may be written.
    fn read(app: &RSpiceApp, file_name: String, contents: String) -> Self {
        let modules = crate::state::workspace::survey_veriloga_file(&file_name, &contents)
            .map(|survey| survey.tops);
        let writable = writable_libraries(&app.state.library_manager);
        let library = [
            &app.state.workspace.content.active_view.library,
            &app.state.workspace.content.project.root_library,
        ]
        .into_iter()
        .find(|library| writable.contains(*library))
        .or_else(|| writable.first())
        .cloned()
        .unwrap_or_default();
        Self {
            project_id: app.state.workspace.content.project.id(),
            file_name,
            contents,
            library,
            modules,
            error: None,
        }
    }
}

/// The libraries a cell can be created in, by name.
pub(crate) fn writable_libraries(libraries: &crate::state::LibraryManager) -> Vec<String> {
    libraries
        .libraries_sorted()
        .into_iter()
        .filter(|library| !library.read_only)
        .map(|library| library.name.clone())
        .collect()
}

pub(crate) fn import_dropped_veriloga_source(
    app: &mut RSpiceApp,
    file_name: String,
    contents: String,
) -> Result<(), String> {
    if app.state.workbench.safe_mode.project_read_only() {
        return Err("The active project is read-only.".to_owned());
    }
    if app.state.ui.code_workspace.veriloga.import_requested
        || app.state.ui.code_workspace.veriloga.pending.is_some()
    {
        return Err("A Verilog-A import or compile transaction is already active.".to_owned());
    }
    let extension = std::path::Path::new(&file_name)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or_default();
    if !matches!(extension.to_ascii_lowercase().as_str(), "va" | "vams") {
        return Err("Dropped Verilog-A source must use a .va or .vams extension.".to_owned());
    }
    if contents.len() > crate::state::MAX_PROJECT_CODE_SOURCE_BYTES {
        return Err(format!(
            "Dropped source exceeds the supported {}-byte limit.",
            crate::state::MAX_PROJECT_CODE_SOURCE_BYTES
        ));
    }
    // A file dropped on an editor is a file to import, not a replacement for
    // the source under the pointer: it goes to the import form like any other.
    open_veriloga_import_form(app, file_name, contents);
    Ok(())
}

fn suggested_import_path(importer_path: &str, file_name: &str) -> String {
    importer_path.rsplit_once('/').map_or_else(
        || file_name.to_owned(),
        |(parent, _)| format!("{parent}/{file_name}"),
    )
}

pub(crate) fn active_veriloga_file_path(
    app: &RSpiceApp,
    selected: &SelectedVerilogASource,
) -> String {
    app.state
        .ui
        .code_workspace
        .veriloga
        .selected_file
        .as_ref()
        .filter(|selection| selection.bundle_id == selected.bundle().id())
        .and_then(|selection| {
            selected
                .bundle()
                .contains_file(&selection.logical_path)
                .then(|| selection.logical_path.clone())
        })
        .unwrap_or_else(|| selected.document().logical_path().to_owned())
}

/// The built-in header the editor well shows in place of `active_path` of
/// `bundle_id`, by its canonical identity and exact text, when one is open.
pub(crate) fn viewed_veriloga_header(
    state: &super::page::VerilogAWorkbenchState,
    bundle_id: ProjectSourceId,
    active_path: &str,
) -> Option<(&'static str, &'static str)> {
    state
        .viewed_header
        .as_ref()
        .filter(|header| {
            header.bundle_id == bundle_id
                && crate::state::project_source_paths_equal(&header.over, active_path)
        })
        .and_then(|header| super::builtin_header(header.identity))
}

fn add_bundle_file(
    app: &mut RSpiceApp,
    bundle_id: ProjectSourceId,
    importer_path: &str,
    logical_path: String,
    content: String,
) -> Result<bool, String> {
    let file = ProjectSourceFile::try_new(logical_path.clone(), content)
        .map_err(|error| error.to_string())?;
    let changed = app
        .state
        .workspace
        .content
        .project_sources
        .add_bundle_file(bundle_id, importer_path, file)
        .map_err(|error| error.to_string())?;
    if changed {
        app.state.ui.code_workspace.veriloga.selected_file = Some(VerilogAFileSelection {
            bundle_id,
            logical_path,
        });
        invalidate_veriloga_evidence(app, bundle_id);
    }
    Ok(changed)
}

/// Record that `bundle_id` changed: compile evidence for it is void, the
/// project has unsaved source, and the cell view that owns the bundle — whose
/// document the bundle is part of — says so on its tab.
///
/// A save clears the tab the way it clears a schematic's: the lifecycle
/// registry fingerprints a cell view's bundle as part of that view's document,
/// so publishing the bundle publishes the view and the registry marks it
/// clean.
pub(crate) fn invalidate_veriloga_evidence(app: &mut RSpiceApp, bundle_id: ProjectSourceId) {
    mark_bundle_unsaved(app, bundle_id);
    cancel_veriloga_compile(app);
    app.state.ui.code_workspace.veriloga.receipt = None;
    Arc::make_mut(&mut app.state.ui.code_workspace.veriloga.last_failure).clear();
    app.state.ui.code_workspace.veriloga.last_failure_token = None;
}

/// The persisted bundle changed: the project has unsaved source, and so does
/// the cell view that owns the bundle, whose tab says so.
fn mark_bundle_unsaved(app: &mut RSpiceApp, bundle_id: ProjectSourceId) {
    app.state.workspace.content.project_sources_dirty = true;
    let owner = app
        .state
        .workspace
        .content
        .project_sources
        .get_bundle(bundle_id)
        .map(|bundle| bundle.owner().clone());
    if let Some(ProjectSourceOwner::CellView { reference }) = owner
        && let Some(open) = app
            .state
            .workspace
            .content
            .open_views
            .iter_mut()
            .find(|open| open.reference == reference)
    {
        open.dirty = true;
    }
}

/// Replace one file's exact UTF-8 bytes only when the same bundle identity is
/// still active. The registry performs the edit transaction on a clone,
/// increments the bundle revision, invalidates validation evidence, and
/// commits only after the complete closure remains valid.
pub(crate) fn replace_selected_veriloga_file(
    app: &mut RSpiceApp,
    expected: &SelectedVerilogASource,
    logical_path: &str,
    contents: String,
) -> Result<bool, String> {
    let current = selected_veriloga_source(app)?;
    let project_id = app.state.workspace.content.project.id();
    if !current.matches_token(project_id, expected.token(project_id)) {
        return Err(
            "The active Verilog-A source changed before this edit could be committed.".to_owned(),
        );
    }
    let changed = app
        .state
        .workspace
        .content
        .project_sources
        .replace_bundle_file_content(current.bundle().id(), logical_path, contents)
        .map_err(|error| error.to_string())?;
    if changed {
        invalidate_veriloga_evidence(app, current.bundle().id());
    }
    Ok(changed)
}

/// Ask for a compile of the source in front of the reader, as it stands now.
///
/// Compile asks nothing: the closure and its build profile are taken here, at
/// the click, and the frame loop starts exactly that on its next pass. A
/// source still on the compiler's built-in defaults has them written into the
/// bundle as its build profile first, so what compiles is always a profile
/// the project holds.
pub(crate) fn request_veriloga_compile(app: &mut RSpiceApp) -> Result<(), String> {
    if app.state.ui.code_workspace.veriloga.pending.is_some() {
        return Err("A Verilog-A compile is already running.".to_owned());
    }
    let mut selected = selected_veriloga_source(app)?;
    if selected.bundle().qualifications().len()
        >= crate::state::MAX_PROJECT_SOURCE_QUALIFICATION_RECORDS
    {
        return Err(
            "This source bundle has reached the immutable qualification-history limit. Archive the project as a new governed revision before compiling again."
                .to_owned(),
        );
    }
    let mut resolved = super::veriloga_profile::resolve_veriloga_build_profile(selected.bundle())?;
    if resolved.legacy_builtin {
        persist_legacy_build_profile(app, &selected, &resolved.profile)?;
        selected = selected_veriloga_source(app)?;
        resolved = super::veriloga_profile::resolve_veriloga_build_profile(selected.bundle())?;
    }
    let entry_modules = &resolved.profile.entry_modules;
    if let Some(module) = selected.selected_module()
        && !entry_modules.is_empty()
        && !entry_modules.iter().any(|entry| entry == module)
    {
        return Err(format!(
            "Selected module '{module}' is not declared by the Verilog-A build profile."
        ));
    }
    app.state.ui.code_workspace.veriloga.compile_request = Some(selected);
    Ok(())
}

fn persist_legacy_build_profile(
    app: &mut RSpiceApp,
    selected: &SelectedVerilogASource,
    profile: &super::veriloga_profile::VerilogABuildProfile,
) -> Result<(), String> {
    let path = add_build_profile_document(app, selected, profile)?;
    app.state.push_user_message(ConsoleMessage::info(format!(
        "Persisted the legacy Verilog-A compiler defaults as project build profile '{path}'."
    )));
    Ok(())
}

/// Add `profile` to `selected`'s bundle as its build-profile document, at the
/// first free path under `.rspice/`. Returns the path it was written at.
fn add_build_profile_document(
    app: &mut RSpiceApp,
    selected: &SelectedVerilogASource,
    profile: &super::veriloga_profile::VerilogABuildProfile,
) -> Result<String, String> {
    let base = ".rspice/veriloga-build";
    let path = (1_u32..=10_000)
        .map(|index| {
            if index == 1 {
                format!("{base}.toml")
            } else {
                format!("{base}-{index}.toml")
            }
        })
        .find(|path| !selected.bundle().contains_file(path))
        .ok_or_else(|| "Could not allocate a portable Verilog-A build-profile path.".to_owned())?;
    let file =
        ProjectSourceFile::try_new(&path, profile.to_toml()?).map_err(|error| error.to_string())?;
    app.state
        .workspace
        .content
        .add_project_source_bundle_file_with_role(
            selected.bundle().id(),
            selected.bundle().root().logical_path(),
            file,
            ProjectSourceRole::VerilogABuildProfile,
        )
        .map_err(|error| error.to_string())?;
    invalidate_veriloga_evidence(app, selected.bundle().id());
    Ok(path)
}

/// Compile `selected`, the closure a Compile request took at its click.
/// Native builds use an operating-system thread; browser builds dispatch the
/// same sealed bundle to a dedicated module worker so compilation never stalls
/// egui's event/rendering thread.
fn start_veriloga_compile(
    app: &mut RSpiceApp,
    selected: SelectedVerilogASource,
    repaint: &egui::Context,
) {
    if app.state.ui.code_workspace.veriloga.pending.is_some() {
        return;
    }
    let token = selected.token(app.state.workspace.content.project.id());
    let (sender, receiver) = mpsc::channel();
    Arc::make_mut(&mut app.state.ui.code_workspace.veriloga.last_failure).clear();
    app.state.ui.code_workspace.veriloga.last_failure_token = None;
    app.state.ui.code_workspace.veriloga.pending = Some(PendingVerilogACompile {
        token,
        receiver: Arc::new(Mutex::new(receiver)),
    });

    let wake = repaint.clone();
    if let Err(error) = rspice_simulation::project_veriloga::compile_service::start_compile(
        selected.bundle,
        selected.selected_module,
        sender.clone(),
        move || wake.request_repaint(),
    ) {
        let _ = sender.send(
            rspice_simulation::project_veriloga::compile_service::transport_failure_outcome(error),
        );
        repaint.request_repaint();
    }
}

/// Cancel transient compile work before source/module identity is replaced.
/// Native work is detached exactly as before; browser work is also terminated
/// so abandoned compilation cannot consume CPU or block the next request.
pub(crate) fn cancel_veriloga_compile(app: &mut RSpiceApp) {
    #[cfg(target_arch = "wasm32")]
    rspice_simulation::project_veriloga::compile_service::cancel_compile();
    app.state.ui.code_workspace.veriloga.pending = None;
}

/// Start the compile a Compile command asked for.
///
/// The command took the closure at the click; the frame loop starts it here,
/// before any surface draws, with the context the worker wakes when it
/// answers. Its result lands wherever the reader is by then.
pub fn consume_veriloga_compile_request(app: &mut RSpiceApp, repaint: &egui::Context) {
    if let Some(selected) = app.state.ui.code_workspace.veriloga.compile_request.take() {
        start_veriloga_compile(app, selected, repaint);
    }
}

/// Poll a pending compile and publish it only when its revision and digest
/// still identify the exact visible project source.
///
/// The frame loop runs this every frame, so a result is published whether or
/// not an editor host is on screen when it lands.
pub fn poll_veriloga_compile(app: &mut RSpiceApp) {
    let Some(pending) = app.state.ui.code_workspace.veriloga.pending.clone() else {
        return;
    };
    let received = pending
        .receiver
        .lock()
        .ok()
        .and_then(|receiver| match receiver.try_recv() {
            Ok(outcome) => Some(Ok(outcome)),
            Err(mpsc::TryRecvError::Disconnected) => Some(Err(())),
            Err(mpsc::TryRecvError::Empty) => None,
        });
    let Some(received) = received else {
        return;
    };
    cancel_veriloga_compile(app);

    let is_current = selected_veriloga_source(app).is_ok_and(|selected| {
        selected.matches_token(app.state.workspace.content.project.id(), pending.token)
    });
    if !is_current {
        // The editor exposes one active compile receipt. Once an in-flight
        // result is rejected because its source/module contract changed, an
        // older receipt must not remain as apparently current evidence.
        app.state.ui.code_workspace.veriloga.receipt = None;
        return;
    }

    match received {
        Ok(VerilogACompileOutcome::Success(report)) => {
            let receipt_bundle = app
                .state
                .workspace
                .content
                .project_sources
                .get_bundle(pending.token.bundle_id)
                .cloned();
            let receipt = match receipt_from_report(pending.token, &report, receipt_bundle.as_ref())
            {
                Ok(receipt) => receipt,
                Err(error) => {
                    let diagnostic = diagnostic_capacity_failure(pending.token, error);
                    let diagnostics = [diagnostic.clone()];
                    let _ = record_veriloga_qualification(app, pending.token, None, &diagnostics);
                    app.state.ui.code_workspace.veriloga.receipt = None;
                    app.state.ui.code_workspace.veriloga.last_failure = Arc::new(
                        CodeDiagnosticCollection::try_new(vec![diagnostic]).unwrap_or_default(),
                    );
                    app.state.ui.code_workspace.veriloga.last_failure_token = Some(pending.token);
                    app.state.push_user_message(ConsoleMessage::error(
                        "Verilog-A diagnostics exceeded the supported release-scale collection; no validation receipt was published.",
                    ));
                    return;
                }
            };
            if let Err(error) = record_veriloga_qualification(
                app,
                pending.token,
                Some(&report),
                receipt.diagnostics.as_slice(),
            ) {
                app.state.push_user_message(ConsoleMessage::error(format!(
                    "The compiler succeeded, but immutable qualification evidence could not be retained: {error}"
                )));
                app.state.ui.code_workspace.veriloga.receipt = None;
                return;
            }
            let previous_identity = app
                .state
                .workspace
                .content
                .project_sources
                .get_bundle(pending.token.bundle_id)
                .and_then(ProjectSourceBundle::validated_identity);
            match app
                .state
                .workspace
                .content.project_sources
                .mark_bundle_validated(pending.token.bundle_id)
            {
                Ok(_) => {
                    if previous_identity
                        != app
                            .state
                            .workspace
                            .content.project_sources
                            .get_bundle(pending.token.bundle_id)
                            .and_then(ProjectSourceBundle::validated_identity)
                    {
                        app.state.workspace.content.project_sources_dirty = true;
                    }
                    let receipt = match receipt_bundle.as_ref().map(ProjectSourceBundle::owner) {
                        Some(ProjectSourceOwner::CellView { reference }) => {
                            let applied = super::veriloga_interface::apply_compiled_interface(
                                &mut app.state.library_manager,
                                &app.state.workspace.content.schematic_buffers,
                                reference,
                                &report.abi,
                            );
                            if applied.wrote {
                                app.state.workspace.content.project_metadata_dirty = true;
                            }
                            if let super::veriloga_interface::VerilogAInterfaceOutcome::Refused(
                                reason,
                            ) = &applied.outcome
                            {
                                app.state.push_user_message(ConsoleMessage::warning(format!(
                                    "The module compiled, but its ports did not become the cell's interface: {reason}."
                                )));
                            }
                            let receipt = super::veriloga_interface::with_interface_outcome(
                                receipt,
                                reference,
                                applied.outcome,
                            );
                            super::veriloga_interface::offer_placement(
                                &mut app.state.ui.code_workspace.veriloga,
                                receipt,
                                reference,
                                report.abi.formal_ports.len(),
                            )
                        }
                        _ => receipt,
                    };
                    Arc::make_mut(&mut app.state.ui.code_workspace.veriloga.last_failure).clear();
                    app.state.ui.code_workspace.veriloga.last_failure_token = None;
                    app.state.ui.code_workspace.veriloga.receipt = Some(receipt);
                    app.state.push_user_message(ConsoleMessage::info(format!(
                        "Compiled Verilog-A module {} for every qualified runtime target.",
                        report.abi.module_name
                    )));
                }
                Err(error) => app.state.push_user_message(ConsoleMessage::error(format!(
                    "The compiler succeeded, but its validation receipt could not be retained: {error}"
                ))),
            }
        }
        Ok(VerilogACompileOutcome::Failure(diagnostics)) => {
            if let Err(error) =
                record_veriloga_qualification(app, pending.token, None, &diagnostics)
            {
                app.state.push_user_message(ConsoleMessage::error(format!(
                    "Verilog-A qualification failure evidence could not be retained: {error}"
                )));
            }
            app.state.ui.code_workspace.veriloga.receipt = None;
            let diagnostics = diagnostics
                .into_iter()
                .map(|diagnostic| {
                    let document_id = diagnostic
                        .source_path
                        .clone()
                        .unwrap_or_else(|| pending.token.bundle_id.to_string().into());
                    diagnostic.bind_validation(
                        document_id,
                        pending.token.revision,
                        pending.token.closure_digest.to_string(),
                    )
                })
                .collect();
            app.state.ui.code_workspace.veriloga.last_failure = Arc::new(
                CodeDiagnosticCollection::try_new(diagnostics).unwrap_or_else(|error| {
                    CodeDiagnosticCollection::try_new(vec![diagnostic_capacity_failure(
                        pending.token,
                        error,
                    )])
                    .unwrap_or_default()
                }),
            );
            app.state.ui.code_workspace.veriloga.last_failure_token = Some(pending.token);
            app.state.push_user_message(ConsoleMessage::error(
                "Verilog-A compilation failed. Review the source diagnostics.",
            ));
        }
        Err(()) => {
            app.state.ui.code_workspace.veriloga.receipt = None;
            app.state.ui.code_workspace.veriloga.last_failure = Arc::new(
                CodeDiagnosticCollection::try_new(vec![
                    CodeEditorDiagnostic::current(
                        "rspice.veriloga.compiler-worker",
                        "VA-WORKER-STOPPED",
                        CodeEditorSeverity::Error,
                        "Compiler worker stopped unexpectedly",
                        "No compiler report was published; the source was not validated.",
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .bind_validation(
                        pending.token.bundle_id.to_string(),
                        pending.token.revision,
                        pending.token.closure_digest.to_string(),
                    ),
                ])
                .unwrap_or_default(),
            );
            let worker_diagnostics = app.state.ui.code_workspace.veriloga.last_failure.clone();
            if let Err(error) = record_veriloga_qualification(
                app,
                pending.token,
                None,
                worker_diagnostics.as_slice(),
            ) {
                app.state.push_user_message(ConsoleMessage::error(format!(
                    "Verilog-A worker failure evidence could not be retained: {error}"
                )));
            }
            app.state.ui.code_workspace.veriloga.last_failure_token = Some(pending.token);
            app.state.push_user_message(ConsoleMessage::error(
                "The Verilog-A compiler worker stopped before publishing a report.",
            ));
        }
    }
}

fn record_veriloga_qualification(
    app: &mut RSpiceApp,
    token: VerilogASourceOperationToken,
    report: Option<&RuntimeCompileReport>,
    diagnostics: &[CodeEditorDiagnostic],
) -> Result<u64, String> {
    let selected = selected_veriloga_source(app)?;
    if !selected.matches_token(app.state.workspace.content.project.id(), token) {
        return Err("qualification result no longer matches the current source bundle".to_owned());
    }
    let resolved = super::veriloga_profile::resolve_veriloga_build_profile(selected.bundle())?;
    let selected_module = report
        .map(|report| report.abi.module_name.to_string())
        .or_else(|| selected.selected_module().map(str::to_owned))
        .or_else(|| resolved.profile.entry_modules.first().cloned())
        .unwrap_or_else(|| {
            super::source_files::verilog_a_module_name_for_profile(
                selected.bundle().root().logical_path(),
            )
        });
    let report_bytes = match report {
        Some(report) => serde_json::to_vec(report),
        None => serde_json::to_vec(diagnostics),
    }
    .map_err(|error| format!("could not encode qualification evidence: {error}"))?;
    let report_digest =
        crate::product::ContentDigest::from_bytes(sha2::Sha256::digest(&report_bytes).into());
    let targets = report
        .map(|report| {
            report
                .targets
                .all()
                .iter()
                .map(|target| crate::state::ProjectSourceQualificationTarget {
                    name: target.target.label().to_owned(),
                    readiness: runtime_readiness_label(target.readiness).to_owned(),
                    maturity: runtime_maturity_label(target.maturity).to_owned(),
                    detail: target.detail.clone(),
                })
                .collect()
        })
        .unwrap_or_default();
    let checks = report
        .map(|report| {
            report
                .specialist
                .checks
                .iter()
                .map(|check| crate::state::ProjectSourceQualificationCheck {
                    name: specialist_check_label(check.kind).to_owned(),
                    disposition: specialist_disposition_label(check.disposition).to_owned(),
                    findings: check.findings,
                    detail: check.detail.clone(),
                })
                .collect()
        })
        .unwrap_or_default();
    let error_count = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == CodeEditorSeverity::Error)
        .count();
    let warning_count = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == CodeEditorSeverity::Warning)
        .count();
    let recorded_at_unix_ms = crate::time_compat::unix_epoch()
        .as_millis()
        .try_into()
        .map_err(|_| "qualification timestamp exceeds the persisted range".to_owned())?;
    let record = crate::state::ProjectSourceQualificationAttempt {
        attempt_id: uuid::Uuid::new_v4(),
        recorded_at_unix_ms,
        source_revision: token.revision,
        source_closure_digest: token.closure_digest,
        profile_digest: resolved.digest,
        package_name: resolved.profile.package.name,
        package_version: resolved.profile.package.version,
        selected_module,
        compiler_version: env!("CARGO_PKG_VERSION").to_owned(),
        disposition: if report.is_some() {
            crate::state::ProjectSourceQualificationDisposition::Passed
        } else {
            crate::state::ProjectSourceQualificationDisposition::Failed
        },
        report_digest,
        targets,
        checks,
        error_count,
        warning_count,
    };
    let sequence = app
        .state
        .workspace
        .content
        .append_project_source_qualification(token.bundle_id, record)
        .map_err(|error| error.to_string())?;
    // The qualification history is persisted with the bundle, so recording
    // an attempt is a change to the owning cell view's document.
    mark_bundle_unsaved(app, token.bundle_id);
    Ok(sequence)
}

const fn runtime_readiness_label(readiness: RuntimeTargetReadiness) -> &'static str {
    match readiness {
        RuntimeTargetReadiness::Available => "available",
        RuntimeTargetReadiness::Unavailable => "unavailable",
        RuntimeTargetReadiness::Rejected => "rejected",
    }
}

const fn runtime_maturity_label(maturity: RuntimeTargetMaturity) -> &'static str {
    match maturity {
        RuntimeTargetMaturity::Production => "production",
        RuntimeTargetMaturity::Preview => "preview",
        RuntimeTargetMaturity::QualificationOnly => "qualification-only",
    }
}

const fn specialist_check_label(check: rspice_veriloga::SpecialistCheckKind) -> &'static str {
    match check {
        rspice_veriloga::SpecialistCheckKind::HiddenState => "hidden-state",
        rspice_veriloga::SpecialistCheckKind::Discontinuity => "discontinuity",
        rspice_veriloga::SpecialistCheckKind::UnitsAndRanges => "units-and-ranges",
        rspice_veriloga::SpecialistCheckKind::Convergence => "convergence",
        rspice_veriloga::SpecialistCheckKind::Portability => "portability",
    }
}

const fn specialist_disposition_label(
    disposition: rspice_veriloga::SpecialistCheckDisposition,
) -> &'static str {
    match disposition {
        rspice_veriloga::SpecialistCheckDisposition::Evaluated => "evaluated",
        rspice_veriloga::SpecialistCheckDisposition::Findings => "findings",
        rspice_veriloga::SpecialistCheckDisposition::EvidenceOnly => "evidence-only",
        rspice_veriloga::SpecialistCheckDisposition::NotEvaluated => "not-evaluated",
    }
}

#[cfg(test)]
fn compile_selected_source(selected: &SelectedVerilogASource) -> VerilogACompileOutcome {
    rspice_simulation::project_veriloga::compile_service::compile_source(
        selected.bundle(),
        selected.selected_module(),
    )
}

pub(crate) fn compile_project_bundle_virtual_for_provenance(
    bundle: &ProjectSourceBundle,
    selected_module: &str,
) -> Result<VirtualRuntimeCompilation, String> {
    rspice_simulation::project_veriloga::compile_project_virtual_runtime(bundle, selected_module)
        .map_err(|error| error.to_string())
}

pub(crate) fn compile_project_bundle_receipt(
    project_id: crate::product::ProjectId,
    bundle: &ProjectSourceBundle,
    selected_module: Option<&str>,
) -> Result<VerilogACompileReceipt, Vec<CodeEditorDiagnostic>> {
    let token = VerilogASourceOperationToken::capture(project_id, bundle, selected_module);
    match rspice_simulation::project_veriloga::receipt::compile_project_bundle_receipt(
        project_id,
        bundle,
        selected_module,
    ) {
        Ok((compilation, diagnostics)) => receipt_projection(compilation, diagnostics)
            .map_err(|error| vec![diagnostic_capacity_failure(token, error)]),
        Err(diagnostics) => Err(diagnostics
            .into_iter()
            .map(|diagnostic| {
                let diagnostic = CodeEditorDiagnostic::from(diagnostic);
                let document_id = diagnostic
                    .source_path
                    .clone()
                    .unwrap_or_else(|| token.bundle_id.to_string().into());
                diagnostic.bind_validation(
                    document_id,
                    token.revision,
                    token.closure_digest.to_string(),
                )
            })
            .collect()),
    }
}

fn receipt_from_report(
    token: VerilogASourceOperationToken,
    report: &RuntimeCompileReport,
    bundle: Option<&ProjectSourceBundle>,
) -> Result<VerilogACompileReceipt, String> {
    let (compilation, diagnostics) =
        rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt::from_report(
            token, report, bundle,
        )?;
    receipt_projection(compilation, diagnostics)
}

fn receipt_projection(
    compilation: rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt,
    diagnostics: Vec<rspice_simulation::project_veriloga::diagnostics::ProjectCompileDiagnostic>,
) -> Result<VerilogACompileReceipt, String> {
    let token = compilation.token();
    let report = compilation.report();
    let diagnostics = diagnostics
        .into_iter()
        .map(|diagnostic| {
            CodeEditorDiagnostic::from(diagnostic).bind_validation(
                token.bundle_id.to_string(),
                token.revision,
                token.closure_digest.to_string(),
            )
        })
        .collect();
    let diagnostics = Arc::new(CodeDiagnosticCollection::try_new(diagnostics)?);
    Ok(VerilogACompileReceipt {
        analog_ports: report.abi.analog_port_count(),
        noise_sources: report.abi.noise_source_count,
        state_variables: report.abi.state_variable_count,
        bytecode_available: report.targets.is_available(RuntimeTarget::BytecodeVm),
        native_jit: target_qualification(report.targets.get(RuntimeTarget::NativeJit)),
        wasm_interpreter: target_qualification(report.targets.get(RuntimeTarget::WasmInterpreter)),
        generated_rust: target_qualification(report.targets.get(RuntimeTarget::GeneratedRust)),
        diagnostics,
        compilation,
        interface: None,
        placement: None,
    })
}

fn diagnostic_capacity_failure(
    token: VerilogASourceOperationToken,
    detail: String,
) -> CodeEditorDiagnostic {
    CodeEditorDiagnostic::current(
        "rspice.diagnostics",
        "DIAGNOSTIC-CAPACITY",
        CodeEditorSeverity::Error,
        "Diagnostic collection exceeded the supported maximum",
        detail,
        None,
        None,
        None,
        None,
        None,
    )
    .bind_validation(
        token.bundle_id.to_string(),
        token.revision,
        token.closure_digest.to_string(),
    )
}

fn target_qualification(target: &RuntimeTargetQualification) -> TargetQualification {
    match (target.readiness, target.maturity) {
        (RuntimeTargetReadiness::Available, RuntimeTargetMaturity::Production) => {
            TargetQualification::Available
        }
        (RuntimeTargetReadiness::Available, RuntimeTargetMaturity::Preview) => {
            TargetQualification::Preview
        }
        (RuntimeTargetReadiness::Available, RuntimeTargetMaturity::QualificationOnly) => {
            TargetQualification::QualificationOnly
        }
        (RuntimeTargetReadiness::Unavailable, _) => {
            TargetQualification::Unsupported(target.detail.clone())
        }
        (RuntimeTargetReadiness::Rejected, _) => TargetQualification::Failed(target.detail.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Cell, CellViewRef, Library, ProjectSourceDocument, View};

    #[test]
    fn editor_and_execution_share_the_project_macro_expansion_contract() {
        let limits = rspice_simulation::project_veriloga::project_virtual_compile_limits();
        assert_eq!(
            limits.max_expanded_bytes,
            crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES.saturating_mul(2)
        );
    }

    /// A `behavioral/<cell>/veriloga` view binding `module`, owning a
    /// one-file bundle at `root` that holds `source`.
    fn add_module_view(
        app: &mut RSpiceApp,
        cell_name: &str,
        root: &str,
        module: &str,
        source: &str,
    ) -> CellViewRef {
        if app
            .state
            .library_manager
            .get_library("behavioral")
            .is_none()
        {
            app.state
                .library_manager
                .add_library(Library::new("behavioral"));
        }
        let reference = CellViewRef::new("behavioral", cell_name, "veriloga");
        let mut view = View::new("veriloga", ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), module.to_owned());
        view.metadata
            .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
        let library = app
            .state
            .library_manager
            .get_library_mut("behavioral")
            .unwrap();
        let mut cell = Cell::new(cell_name);
        cell.add_view(view);
        library.add_cell(cell);
        let bundle = ProjectSourceBundle::try_new(
            ProjectSourceOwner::cell_view(reference.clone()),
            ProjectSourceLanguage::VerilogA,
            root,
            source,
            [],
            [],
        )
        .unwrap();
        app.state
            .workspace
            .content
            .project_sources
            .insert_bundle(bundle)
            .unwrap();
        reference
    }

    /// A test app whose open Verilog-A view binds `module` over exactly
    /// `source`.
    fn module_source_with(module: &str, source: &str) -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.authority.open_session();
        let reference = add_module_view(&mut app, "legacy", "legacy.va", module, source);
        app.state.workspace.open_view(reference, ViewType::VerilogA);
        app
    }

    fn portless_module() -> RSpiceApp {
        module_source_with("legacy", "module legacy; endmodule\n")
    }

    /// A compile of a source still on the compiler's built-in defaults writes
    /// them into the bundle as its build profile, and compiles the closure
    /// holding it: the request is taken after the write, not before.
    #[test]
    fn a_compile_request_writes_legacy_defaults_as_the_build_profile_it_compiles() {
        let mut app = portless_module();

        request_veriloga_compile(&mut app).unwrap();

        let request = app
            .state
            .ui
            .code_workspace
            .veriloga
            .compile_request
            .clone()
            .expect("the click took the closure");
        let bundle = app
            .state
            .workspace
            .content
            .project_sources
            .get_bundle(request.bundle().id())
            .unwrap();
        let profile_path = bundle
            .paths_for_role(ProjectSourceRole::VerilogABuildProfile)
            .next()
            .unwrap();
        assert_eq!(profile_path, ".rspice/veriloga-build.toml");
        assert!(!bundle.root().content().contains(profile_path));
        assert!(app.state.workspace.content.project_sources_dirty);
        assert_eq!(request.bundle().revision(), bundle.revision());
        assert_eq!(request.bundle().closure_digest(), bundle.closure_digest());
        assert!(request.bundle().contains_file(profile_path));
    }

    #[test]
    fn qualification_history_is_append_only_without_changing_source_identity() {
        let mut app = portless_module();
        request_veriloga_compile(&mut app).unwrap();
        let selected = selected_veriloga_source(&app).unwrap();
        let token = selected.token(app.state.workspace.content.project.id());
        let revision = selected.bundle().revision();
        let closure_digest = selected.bundle().closure_digest();
        let VerilogACompileOutcome::Success(report) = compile_selected_source(&selected) else {
            panic!("legacy source must compile")
        };
        let receipt = receipt_from_report(token, &report, Some(selected.bundle())).unwrap();

        let first = record_veriloga_qualification(
            &mut app,
            token,
            Some(&report),
            receipt.diagnostics.as_slice(),
        )
        .unwrap();
        let second = record_veriloga_qualification(
            &mut app,
            token,
            Some(&report),
            receipt.diagnostics.as_slice(),
        )
        .unwrap();

        let bundle = app
            .state
            .workspace
            .content
            .project_sources
            .get_bundle(token.bundle_id)
            .unwrap();
        assert_eq!((first, second), (1, 2));
        assert_eq!(bundle.qualifications().len(), 2);
        assert_eq!(bundle.revision(), revision);
        assert_eq!(bundle.closure_digest(), closure_digest);
        bundle.validate().unwrap();
    }

    fn add_cell_bundle(app: &mut RSpiceApp, cell_name: &str) -> CellViewRef {
        add_module_view(
            app,
            cell_name,
            "shared.va",
            "shared_model",
            "module shared_model(p, n); inout p, n; electrical p, n; endmodule\n",
        )
    }

    /// Every Verilog-A source is a cell view's. A project-wide bundle still in
    /// a registry is never the source in front of the reader: with no
    /// Verilog-A view open there is none, and with one open it is that view's.
    #[test]
    fn only_the_open_veriloga_view_selects_a_source() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .workspace
            .content
            .project_sources
            .insert(
                ProjectSourceDocument::try_new(
                    "legacy.va",
                    ProjectSourceLanguage::VerilogA,
                    "module legacy; endmodule\n",
                )
                .unwrap(),
            )
            .unwrap();

        let error =
            selected_veriloga_source(&app).expect_err("a project-wide bundle is not a selection");
        assert!(error.contains("Open a Verilog-A cell view"), "{error}");

        let reference = add_cell_bundle(&mut app, "owned");
        app.state
            .workspace
            .open_view(reference.clone(), ViewType::VerilogA);
        let selected = selected_veriloga_source(&app).unwrap();
        assert_eq!(
            selected.bundle().owner(),
            &ProjectSourceOwner::cell_view(reference)
        );
        assert_eq!(selected.selected_module(), Some("shared_model"));
    }

    /// A file dropped while a Verilog-A view is open is a file to import: it
    /// goes to the import form and the open view's source is left as it was.
    #[test]
    fn a_dropped_source_goes_to_the_import_form_and_never_replaces_the_open_view() {
        let mut app = portless_module();
        let sources_before = app.state.workspace.content.project_sources.clone();
        let source = "module dropped(p, n); inout p, n; electrical p, n; endmodule\n";

        import_dropped_veriloga_source(&mut app, "dropped.va".to_owned(), source.to_owned())
            .expect("drop import");

        assert_eq!(app.state.workspace.content.project_sources, sources_before);
        let form = app
            .state
            .ui
            .code_workspace
            .veriloga
            .import_form
            .as_ref()
            .expect("the drop opened the import form");
        assert_eq!(form.file_name, "dropped.va");
        assert!(
            app.state
                .ui
                .code_workspace
                .veriloga
                .replace_review
                .is_none()
        );
    }

    /// "Replace source from file…" replaces nothing when the file arrives: it
    /// opens the review, and only committing that review rewrites the file,
    /// whose previous text stays in the revision history.
    #[test]
    fn a_source_replacement_waits_for_its_confirmation() {
        let mut app = portless_module();
        let selected = selected_veriloga_source(&app).unwrap();
        let bundle_id = selected.bundle().id();
        let root = selected.document().logical_path().to_owned();
        let original = selected.document().content().to_owned();
        let replacement = "module legacy(p, n); inout p, n; electrical p, n; endmodule\n";

        request_veriloga_source_replacement(&mut app, &root).unwrap();
        assert!(app.state.ui.code_workspace.veriloga.import_requested);
        apply_import(&mut app, "next.va".to_owned(), replacement.to_owned());

        let review = app
            .state
            .ui
            .code_workspace
            .veriloga
            .replace_review
            .clone()
            .expect("the picked file waits for confirmation");
        assert_eq!(review.target.logical_path, root);
        assert!(review.restorable);
        let content = |app: &RSpiceApp| {
            app.state
                .workspace
                .content
                .project_sources
                .get_bundle(bundle_id)
                .unwrap()
                .root()
                .content()
                .to_owned()
        };
        assert_eq!(
            content(&app),
            original,
            "nothing is replaced before confirming"
        );
        assert!(app.state.ui.code_workspace.veriloga.import_form.is_none());

        commit_veriloga_source_replacement(&mut app).unwrap();

        assert_eq!(content(&app), replacement);
        let bundle = app
            .state
            .workspace
            .content
            .project_sources
            .get_bundle(bundle_id)
            .unwrap();
        assert_eq!(
            bundle
                .revision_history()
                .last()
                .and_then(|snapshot| snapshot.file_content(&root)),
            Some(original.as_str()),
            "the replaced text stays restorable"
        );
    }

    /// With no Verilog-A view in front, an imported file is read into the
    /// import form, which names its modules; nothing is created until the form
    /// is committed, and there is no project-wide bundle to fill any more.
    #[test]
    fn a_root_import_with_no_veriloga_view_reads_the_file_into_the_import_form() {
        let mut app = RSpiceApp::test_instance();
        let sources_before = app.state.workspace.content.project_sources.clone();
        let source = "module actual_model(p, n); inout p, n; electrical p, n; endmodule\n";

        app.state.ui.code_workspace.veriloga.import_requested = true;
        apply_import(
            &mut app,
            "behavioral/presentation_name.va".to_owned(),
            source.to_owned(),
        );

        let form = app
            .state
            .ui
            .code_workspace
            .veriloga
            .import_form
            .as_ref()
            .expect("the file is read into the import form");
        assert_eq!(form.file_name, "behavioral/presentation_name.va");
        assert_eq!(
            form.modules,
            Ok(vec![crate::state::workspace::DeclaredModule {
                name: "actual_model".to_owned(),
                ports: vec!["p".to_owned(), "n".to_owned()],
            }])
        );
        assert_eq!(app.state.workspace.content.project_sources, sources_before);
        assert!(!app.state.ui.code_workspace.veriloga.import_requested);
    }

    #[test]
    fn a_source_replacement_rejects_a_bundle_changed_while_the_picker_is_open() {
        let mut app = portless_module();
        let selected = selected_veriloga_source(&app).unwrap();
        let bundle_id = selected.bundle().id();
        let root = selected.document().logical_path().to_owned();

        request_veriloga_source_replacement(&mut app, &root).unwrap();
        app.state
            .workspace
            .content
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "module retained_after_picker_open; endmodule\n".to_owned(),
            )
            .unwrap();
        apply_import(
            &mut app,
            "stale_picker_result.va".to_owned(),
            "module stale_picker_result; endmodule\n".to_owned(),
        );

        let bundle = app
            .state
            .workspace
            .content
            .project_sources
            .get_bundle(bundle_id)
            .unwrap();
        assert_eq!(
            bundle.root().content(),
            "module retained_after_picker_open; endmodule\n"
        );
        assert!(
            app.state
                .ui
                .code_workspace
                .veriloga
                .replace_target
                .is_none()
        );
        assert!(
            app.state
                .ui
                .code_workspace
                .veriloga
                .replace_review
                .is_none(),
            "a stale replacement is discarded, not offered"
        );
    }

    #[test]
    fn root_edit_invalidates_bundle_validation_and_compile_evidence() {
        let mut app = RSpiceApp::test_instance();
        let reference = add_cell_bundle(&mut app, "editable");
        app.state.workspace.open_view(reference, ViewType::VerilogA);
        let selected = selected_veriloga_source(&app).unwrap();
        app.state
            .workspace
            .content
            .project_sources
            .mark_bundle_validated(selected.bundle().id())
            .unwrap();
        let receipt = compile_project_bundle_receipt(
            app.state.workspace.content.project.id(),
            selected.bundle(),
            selected.selected_module(),
        )
        .unwrap();
        app.state.ui.code_workspace.veriloga.receipt = Some(receipt);
        let before = selected.token(app.state.workspace.content.project.id());

        assert!(
            replace_selected_veriloga_file(
                &mut app,
                &selected,
                "shared.va",
                "module shared_model(p, n); inout p, n; electrical p, n; real changed; endmodule\n"
                    .to_owned(),
            )
            .unwrap()
        );

        let edited = selected_veriloga_source(&app).unwrap();
        let after = edited.token(app.state.workspace.content.project.id());
        assert_eq!(after.bundle_id, before.bundle_id);
        assert!(after.revision > before.revision);
        assert_ne!(after.closure_digest, before.closure_digest);
        assert!(!edited.bundle().validation_is_current());
        assert!(app.state.ui.code_workspace.veriloga.receipt.is_none());
        assert!(app.state.workspace.content.project_sources_dirty);
    }

    #[test]
    fn a_multifile_bundle_compiles_with_its_views_module() {
        let mut app = module_source_with(
            "legacy",
            "module legacy(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n",
        );
        let selected = selected_veriloga_source(&app).unwrap();
        let bundle_id = selected.bundle().id();
        let root_path = selected.document().logical_path().to_owned();
        add_bundle_file(
            &mut app,
            bundle_id,
            &root_path,
            "support/constants.vams".to_owned(),
            "`define SCALE 1\n".to_owned(),
        )
        .unwrap();

        let selected = selected_veriloga_source(&app).unwrap();
        assert_eq!(selected.selected_module(), Some("legacy"));
        match compile_selected_source(&selected) {
            VerilogACompileOutcome::Success(_) => {}
            VerilogACompileOutcome::Failure(diagnostics) => {
                panic!("multi-file compile failed: {diagnostics:#?}")
            }
        }
    }

    #[test]
    fn a_compile_carrying_warnings_publishes_its_receipt_and_never_reads_as_failed() {
        let mut app = module_source_with(
            "legacy",
            "module legacy(p, n);\n  inout p, n; electrical p, n;\n  analog begin\n    $display(\"trace\");\n    I(p, n) <+ V(p, n);\n  end\nendmodule\n",
        );

        let selected = selected_veriloga_source(&app).unwrap();
        let token = selected.token(app.state.workspace.content.project.id());
        let VerilogACompileOutcome::Success(report) = compile_selected_source(&selected) else {
            panic!("a discarded system task must not fail the compile")
        };
        assert_eq!(report.diagnostics.len(), 1);

        let receipt = receipt_from_report(token, &report, Some(selected.bundle())).unwrap();
        let warning = receipt
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code() == "VA-SEM-NO-EFFECT-SYSTEM-TASK")
            .expect("the compiler's own warning reaches the editor with its code");
        assert_eq!(warning.severity, CodeEditorSeverity::Warning);
        assert!(warning.message.contains("$display"));
        let summary = receipt.diagnostics.summary();
        assert_eq!(summary.errors, 0);
        assert!(summary.warnings >= 1);

        let (sender, receiver) = mpsc::channel();
        sender
            .send(VerilogACompileOutcome::Success(Box::new(
                (**receipt.compilation.report()).clone(),
            )))
            .unwrap();
        app.state.ui.code_workspace.veriloga.pending = Some(PendingVerilogACompile {
            token: receipt.compilation.token(),
            receiver: Arc::new(Mutex::new(receiver)),
        });

        poll_veriloga_compile(&mut app);

        let published = app
            .state
            .ui
            .code_workspace
            .veriloga
            .receipt
            .as_ref()
            .expect("a pass carrying advisories still publishes a receipt");
        assert_eq!(published.diagnostics.summary().errors, 0);
        assert!(
            published
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code() == "VA-SEM-NO-EFFECT-SYSTEM-TASK")
        );
        assert!(app.state.ui.code_workspace.veriloga.last_failure.is_empty());
        assert!(
            app.state
                .ui
                .code_workspace
                .veriloga
                .last_failure_token
                .is_none()
        );
    }

    #[test]
    fn an_unsupported_generate_region_reaches_the_editor_with_its_compiler_code() {
        // `generate` and `genvar` are elaborated (IEEE 1364-2005 section 12.4),
        // so what still carries this code is an `endgenerate` with no region
        // open. What the test is about is unchanged: a parse refusal reaches
        // the editor with its compiler code, severity, and line.
        let app = module_source_with(
            "legacy",
            "module legacy(p, n);\n  inout p, n; electrical p, n;\n  endgenerate\n  analog I(p, n) <+ V(p, n);\nendmodule\n",
        );

        let selected = selected_veriloga_source(&app).unwrap();
        let VerilogACompileOutcome::Failure(diagnostics) = compile_selected_source(&selected)
        else {
            panic!("a region terminator with nothing to terminate")
        };

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code(), "VA-PARSE-UNSUPPORTED-GENERATE");
        assert_eq!(diagnostics[0].severity, CodeEditorSeverity::Error);
        assert!(diagnostics[0].message.contains("no open generate region"));
        assert_eq!(diagnostics[0].line, Some(3));
    }

    #[test]
    fn completed_async_compile_cannot_cross_publish_between_cell_views() {
        let mut app = RSpiceApp::test_instance();
        let first = add_cell_bundle(&mut app, "first");
        let second = add_cell_bundle(&mut app, "second");
        app.state.workspace.open_view(first, ViewType::VerilogA);
        let first_selected = selected_veriloga_source(&app).unwrap();
        let receipt = compile_project_bundle_receipt(
            app.state.workspace.content.project.id(),
            first_selected.bundle(),
            first_selected.selected_module(),
        )
        .unwrap();
        let (sender, receiver) = mpsc::channel();
        sender
            .send(VerilogACompileOutcome::Success(Box::new(
                (**receipt.compilation.report()).clone(),
            )))
            .unwrap();
        app.state.ui.code_workspace.veriloga.receipt = None;
        app.state.ui.code_workspace.veriloga.pending = Some(PendingVerilogACompile {
            token: receipt.compilation.token(),
            receiver: Arc::new(Mutex::new(receiver)),
        });
        app.state.workspace.open_view(second, ViewType::VerilogA);

        poll_veriloga_compile(&mut app);

        assert!(app.state.ui.code_workspace.veriloga.pending.is_none());
        assert!(app.state.ui.code_workspace.veriloga.receipt.is_none());
        let second_selected = selected_veriloga_source(&app).unwrap();
        assert!(!second_selected.matches_token(
            app.state.workspace.content.project.id(),
            receipt.compilation.token()
        ));
    }

    #[test]
    fn completed_async_compile_cannot_publish_after_module_contract_changes() {
        let mut app = RSpiceApp::test_instance();
        let reference = add_cell_bundle(&mut app, "module_change");
        app.state
            .workspace
            .open_view(reference.clone(), ViewType::VerilogA);
        let selected = selected_veriloga_source(&app).unwrap();
        let receipt = compile_project_bundle_receipt(
            app.state.workspace.content.project.id(),
            selected.bundle(),
            selected.selected_module(),
        )
        .unwrap();
        let (sender, receiver) = mpsc::channel();
        sender
            .send(VerilogACompileOutcome::Success(Box::new(
                (**receipt.compilation.report()).clone(),
            )))
            .unwrap();
        app.state.ui.code_workspace.veriloga.pending = Some(PendingVerilogACompile {
            token: receipt.compilation.token(),
            receiver: Arc::new(Mutex::new(receiver)),
        });
        app.state
            .library_manager
            .get_library_mut(&reference.library)
            .and_then(|library| library.get_cell_mut(&reference.cell))
            .and_then(|cell| cell.get_view_mut(&reference.view))
            .unwrap()
            .metadata
            .insert("veriloga.module".to_owned(), "different_model".to_owned());

        poll_veriloga_compile(&mut app);

        assert!(app.state.ui.code_workspace.veriloga.pending.is_none());
        assert!(app.state.ui.code_workspace.veriloga.receipt.is_none());
        let changed = selected_veriloga_source(&app).unwrap();
        assert!(!changed.matches_token(
            app.state.workspace.content.project.id(),
            receipt.compilation.token()
        ));
        assert!(!changed.bundle().validation_is_current());
    }
}
