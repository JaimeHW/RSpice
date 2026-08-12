//! Governed in-memory Verilog-A compilation for the Code workspace.

use std::sync::{Arc, Mutex, mpsc};

use rspice_veriloga::{
    CompileDiagnosticPhase, RuntimeCompileReport, RuntimeTarget, RuntimeTargetMaturity,
    RuntimeTargetQualification, RuntimeTargetReadiness, VerilogACompiler,
    VirtualRuntimeCompilation, VirtualSourceBundle, VirtualSourceFile,
};
use sha2::Digest as _;

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    ProjectSourceBundle, ProjectSourceFile, ProjectSourceId, ProjectSourceLanguage,
    ProjectSourceOwner, ProjectSourceRole, ViewType,
};
use crate::workbench::RSpiceApp;

use crate::simulation::veriloga::VerilogASourceOperationToken;

use super::{
    CodeDiagnosticCollection, CodeEditorDiagnostic, CodeEditorSeverity, PendingVerilogACompile,
    TargetQualification, VerilogACompileDialogState, VerilogACompileOutcome,
    VerilogACompileReceipt, VerilogAFileSelection, VerilogAQualificationHistoryRow,
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

/// The exact project-owned Verilog-A closure selected by the current editor.
/// Cell-view ownership takes precedence only while that Verilog-A view is the
/// active design document; every other context retains the original Code
/// Workspace singleton.
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
        VerilogASourceOperationToken {
            project_id,
            bundle_id: self.bundle.id(),
            revision: self.bundle.revision().get(),
            closure_digest: self.bundle.closure_digest(),
            requested_module_digest: self
                .selected_module
                .as_deref()
                .map(crate::simulation::veriloga::veriloga_selected_module_digest),
        }
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
    let (owner, selected_module) = if app.state.workspace.active_view_type() == ViewType::VerilogA {
        let reference = app.state.workspace.active_view.clone();
        let module = crate::state::workspace::project_veriloga_binding_for_view(
            &app.state.workspace,
            &app.state.library_manager,
            &reference,
        )?
        .selected_module()
        .to_owned();
        (ProjectSourceOwner::cell_view(reference), Some(module))
    } else {
        let selected_module = app
            .state
            .ui
            .code_workspace
            .veriloga
            .selected_module
            .trim()
            .to_owned();
        (
            ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
            (!selected_module.is_empty()).then_some(selected_module),
        )
    };
    let bundle = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&owner)
        .ok_or_else(|| match owner {
            ProjectSourceOwner::CellView { reference } => format!(
                "{} has no project-owned Verilog-A source bundle.",
                reference.display_path()
            ),
            ProjectSourceOwner::CodeWorkspace { .. } => {
                "This project has no Code Workspace Verilog-A source bundle.".to_owned()
            }
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
        let extensions: &[&str] = if dependency_import {
            &["va", "vams"]
        } else {
            &["va"]
        };
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Verilog-A", extensions)
            .set_title(if dependency_import {
                "Import Verilog-A dependency"
            } else {
                "Import Verilog-A source"
            })
            .pick_file()
        else {
            app.state.ui.code_workspace.veriloga.import_target = None;
            app.state.ui.code_workspace.veriloga.root_import_target = None;
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
                app.state.ui.code_workspace.veriloga.root_import_target = None;
                app.state.push_user_message(ConsoleMessage::error(format!(
                    "Verilog-A import failed: {error}"
                )));
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let dependency_import = app.state.ui.code_workspace.veriloga.import_target.is_some();
        let token = match crate::workbench::browser::file_import::try_begin_text_import(
            crate::workbench::browser::file_import::BrowserTextImportKind::VerilogA,
        ) {
            Ok(token) => token,
            Err(error) => {
                app.state.ui.code_workspace.veriloga.import_target = None;
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
            app.state.ui.code_workspace.veriloga.root_import_target = None;
        }
        Err(error) => {
            app.state.ui.code_workspace.veriloga.import_target = None;
            app.state.ui.code_workspace.veriloga.root_import_target = None;
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
        app.state.ui.code_workspace.veriloga.root_import_target = None;
        let target_is_current = app.state.workspace.project.id() == target.project_id
            && app
                .state
                .workspace
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
    let root_target = app
        .state
        .ui
        .code_workspace
        .veriloga
        .root_import_target
        .take()
        .unwrap_or_else(|| current_root_import_target(app));
    if root_target.project_id != app.state.workspace.project.id() {
        app.state.push_user_message(ConsoleMessage::error(
            "Verilog-A import was discarded because the active project changed while the picker was open.",
        ));
        return;
    }
    match root_target.bundle_identity {
        Some((bundle_id, revision, digest)) => {
            let current = app
                .state
                .workspace
                .project_sources
                .get_bundle(bundle_id)
                .is_some_and(|bundle| {
                    bundle.revision().get() == revision && bundle.closure_digest() == digest
                });
            if !current {
                app.state.push_user_message(ConsoleMessage::error(
                    "Verilog-A import was discarded because the exact source bundle changed while the picker was open.",
                ));
                return;
            }
        }
        None => {
            let owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA);
            if app
                .state
                .workspace
                .project_sources
                .bundle_for_owner(&owner)
                .is_some()
            {
                app.state.push_user_message(ConsoleMessage::error(
                    "Verilog-A import was discarded because a source workspace was created while the picker was open.",
                ));
                return;
            }
            let result =
                super::source_files::new_imported_veriloga_workspace_bundle(&file_name, contents)
                    .and_then(|bundle| {
                        app.state
                            .workspace
                            .insert_project_source_bundle(bundle)
                            .map_err(|error| error.to_string())
                    });
            match result {
                Ok(_) => {
                    app.state.ui.code_workspace.veriloga = Default::default();
                    if let Some(bundle) =
                        app.state.workspace.project_sources.bundle_for_owner(&owner)
                    {
                        app.state.ui.code_workspace.veriloga.selected_file =
                            Some(super::VerilogAFileSelection {
                                bundle_id: bundle.id(),
                                logical_path: bundle.root().logical_path().to_owned(),
                            });
                    }
                    app.state.push_user_message(ConsoleMessage::info(format!(
                        "Imported project-owned Verilog-A source {file_name}; compile is required."
                    )));
                }
                Err(error) => app.state.push_user_message(ConsoleMessage::error(format!(
                    "Verilog-A import failed: {error}"
                ))),
            }
            return;
        }
    }
    let selected = match selected_veriloga_source(app) {
        Ok(selected) => selected,
        Err(error) => {
            app.state.push_user_message(ConsoleMessage::error(format!(
                "Verilog-A import failed: {error}"
            )));
            return;
        }
    };
    let cell_view_owned = matches!(
        selected.bundle().owner(),
        ProjectSourceOwner::CellView { .. }
    );
    let result = if cell_view_owned {
        replace_selected_veriloga_source(app, &selected, contents)
    } else {
        app.state
            .workspace
            .replace_imported_project_source(
                ProjectSourceLanguage::VerilogA,
                file_name.clone(),
                contents,
            )
            .map_err(|error| error.to_string())
    };
    match result {
        Ok(true) => {
            app.state.ui.code_workspace.veriloga = Default::default();
            app.state.push_user_message(ConsoleMessage::info(format!(
                "Imported project-owned Verilog-A source {file_name}{}; compile is required.",
                if cell_view_owned {
                    format!(" into {}", selected.document().logical_path())
                } else {
                    String::new()
                }
            )));
        }
        Ok(false) => app.state.push_user_message(ConsoleMessage::info(format!(
            "Verilog-A source {file_name} is already current."
        ))),
        Err(error) => app.state.push_user_message(ConsoleMessage::error(format!(
            "Verilog-A import failed: {error}"
        ))),
    }
}

fn current_root_import_target(app: &RSpiceApp) -> super::VerilogARootImportTarget {
    let bundle_identity = selected_veriloga_source(app).ok().map(|selected| {
        (
            selected.bundle().id(),
            selected.bundle().revision().get(),
            selected.bundle().closure_digest(),
        )
    });
    super::VerilogARootImportTarget {
        project_id: app.state.workspace.project.id(),
        bundle_identity,
    }
}

pub(crate) fn request_veriloga_root_import(app: &mut RSpiceApp) -> Result<(), String> {
    if let Some(reason) =
        super::source_file_mutation_block_reason(app, ProjectSourceLanguage::VerilogA)
    {
        return Err(reason.to_owned());
    }
    if app.state.ui.code_workspace.veriloga.import_requested
        || app.state.ui.code_workspace.veriloga.pending.is_some()
    {
        return Err("A Verilog-A import or compile transaction is already active.".to_owned());
    }
    if app.state.workspace.active_view_type() == ViewType::VerilogA
        && selected_veriloga_source(app).is_err()
    {
        return Err(
            "The active Verilog-A cell view has no source owner. Create or repair that cell view from its library owner before importing into it."
                .to_owned(),
        );
    }
    app.state.ui.code_workspace.veriloga.root_import_target = Some(current_root_import_target(app));
    app.state.ui.code_workspace.veriloga.import_target = None;
    app.state.ui.code_workspace.veriloga.import_requested = true;
    Ok(())
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
    app.state.ui.code_workspace.veriloga.root_import_target = Some(current_root_import_target(app));
    apply_import(app, file_name, contents);
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
        .project_sources
        .add_bundle_file(bundle_id, importer_path, file)
        .map_err(|error| error.to_string())?;
    if changed {
        app.state.ui.code_workspace.veriloga.selected_file = Some(VerilogAFileSelection {
            bundle_id,
            logical_path,
        });
        invalidate_veriloga_evidence(app);
    }
    Ok(changed)
}

pub(crate) fn invalidate_veriloga_evidence(app: &mut RSpiceApp) {
    app.state.workspace.project_sources_dirty = true;
    cancel_veriloga_compile(app);
    app.state.ui.code_workspace.veriloga.receipt = None;
    Arc::make_mut(&mut app.state.ui.code_workspace.veriloga.last_failure).clear();
    app.state.ui.code_workspace.veriloga.last_failure_token = None;
}

/// Replace the selected root's exact UTF-8 bytes only when the same bundle
/// identity is still active. The registry performs the edit transaction on a
/// clone, increments the bundle revision, invalidates validation evidence, and
/// commits only after the complete closure remains valid.
pub(crate) fn replace_selected_veriloga_source(
    app: &mut RSpiceApp,
    expected: &SelectedVerilogASource,
    contents: String,
) -> Result<bool, String> {
    replace_selected_veriloga_file(app, expected, expected.document().logical_path(), contents)
}

pub(crate) fn replace_selected_veriloga_file(
    app: &mut RSpiceApp,
    expected: &SelectedVerilogASource,
    logical_path: &str,
    contents: String,
) -> Result<bool, String> {
    let current = selected_veriloga_source(app)?;
    let project_id = app.state.workspace.project.id();
    if !current.matches_token(project_id, expected.token(project_id)) {
        return Err(
            "The active Verilog-A source changed before this edit could be committed.".to_owned(),
        );
    }
    let changed = app
        .state
        .workspace
        .project_sources
        .replace_bundle_file_content(current.bundle().id(), logical_path, contents)
        .map_err(|error| error.to_string())?;
    if changed {
        invalidate_veriloga_evidence(app);
    }
    Ok(changed)
}

pub(crate) fn open_veriloga_compile_dialog(app: &mut RSpiceApp) -> Result<(), String> {
    if app.state.ui.code_workspace.veriloga.pending.is_some() {
        return Err("A Verilog-A compile transaction is already active.".to_owned());
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
    let selected_module = selected
        .selected_module()
        .map(str::to_owned)
        .or_else(|| resolved.profile.entry_modules.first().cloned())
        .unwrap_or_default();
    if !selected_module.is_empty()
        && !resolved.profile.entry_modules.is_empty()
        && !resolved
            .profile
            .entry_modules
            .iter()
            .any(|module| module == &selected_module)
    {
        return Err(format!(
            "Selected module '{selected_module}' is not declared by the Verilog-A build profile."
        ));
    }
    let checks = [
        ("hidden-state", resolved.profile.checks.hidden_state),
        ("discontinuities", resolved.profile.checks.discontinuities),
        ("units-and-ranges", resolved.profile.checks.units_and_ranges),
        ("convergence", resolved.profile.checks.convergence),
        ("portability", resolved.profile.checks.portability),
    ]
    .into_iter()
    .map(|(name, enabled)| (name.to_owned(), enabled))
    .collect();
    let dialog = VerilogACompileDialogState {
        project_id: app.state.workspace.project.id(),
        bundle_id: selected.bundle().id(),
        bundle_revision: selected.bundle().revision().get(),
        closure_digest: selected.bundle().closure_digest(),
        profile_digest: resolved.digest,
        profile_path: resolved
            .logical_path
            .unwrap_or_else(|| "<built-in migration profile>".to_owned()),
        package_name: resolved.profile.package.name.clone(),
        package_version: resolved.profile.package.version.clone(),
        selected_module,
        compile_order: veriloga_compile_order(selected.bundle()),
        include_paths: resolved.profile.include_paths.clone(),
        definitions: resolved
            .profile
            .preprocessor
            .defines
            .iter()
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect(),
        undefinitions: resolved.profile.preprocessor.undefines.clone(),
        generated_rust: resolved.profile.targets.generated_rust,
        native_x64_jit: resolved.profile.targets.native_x64_jit,
        reject_fallback: matches!(
            resolved.profile.targets.fallback,
            super::veriloga_profile::VerilogAFallbackPolicy::Reject
        ),
        checks,
        cell_bindings: resolved
            .profile
            .cell_bindings
            .iter()
            .map(|(cell, module)| (cell.clone(), module.clone()))
            .collect(),
        qualification_attempts: selected.bundle().qualifications().len(),
        recent_qualifications: selected
            .bundle()
            .qualifications()
            .iter()
            .rev()
            .take(20)
            .map(|record| VerilogAQualificationHistoryRow {
                sequence: record.sequence(),
                recorded_at_unix_ms: record.recorded_at_unix_ms,
                disposition: record.disposition,
                selected_module: record.selected_module.clone(),
                report_digest: record.report_digest,
            })
            .collect(),
        error: None,
    };
    app.state.ui.code_workspace.veriloga.compile_dialog = Some(dialog);
    Ok(())
}

pub(crate) fn commit_veriloga_compile_dialog(
    app: &mut RSpiceApp,
    repaint: egui::Context,
) -> Result<(), String> {
    let dialog = app
        .state
        .ui
        .code_workspace
        .veriloga
        .compile_dialog
        .clone()
        .ok_or_else(|| "No Verilog-A compile transaction is open.".to_owned())?;
    let selected = selected_veriloga_source(app)?;
    let current_project = app.state.workspace.project.id();
    if dialog.project_id != current_project
        || dialog.bundle_id != selected.bundle().id()
        || dialog.bundle_revision != selected.bundle().revision().get()
        || dialog.closure_digest != selected.bundle().closure_digest()
    {
        return Err(
            "The project source changed after this compile review opened. Reopen it to review the current build inputs."
                .to_owned(),
        );
    }
    let resolved = super::veriloga_profile::resolve_veriloga_build_profile(selected.bundle())?;
    if resolved.digest != dialog.profile_digest {
        return Err(
            "The Verilog-A build profile changed after this review opened. Reopen it before compiling."
                .to_owned(),
        );
    }
    let current_module = selected
        .selected_module()
        .map(str::to_owned)
        .or_else(|| resolved.profile.entry_modules.first().cloned())
        .unwrap_or_default();
    if current_module != dialog.selected_module {
        return Err(
            "The selected Verilog-A entry module changed after this review opened.".to_owned(),
        );
    }
    app.state.ui.code_workspace.veriloga.compile_dialog = None;
    start_veriloga_compile(app, repaint);
    Ok(())
}

fn persist_legacy_build_profile(
    app: &mut RSpiceApp,
    selected: &SelectedVerilogASource,
    profile: &super::veriloga_profile::VerilogABuildProfile,
) -> Result<(), String> {
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
        .add_project_source_bundle_file_with_role(
            selected.bundle().id(),
            selected.bundle().root().logical_path(),
            file,
            ProjectSourceRole::VerilogABuildProfile,
        )
        .map_err(|error| error.to_string())?;
    invalidate_veriloga_evidence(app);
    app.state.push_user_message(ConsoleMessage::info(format!(
        "Persisted the legacy Verilog-A compiler defaults as project build profile '{path}'."
    )));
    Ok(())
}

fn veriloga_compile_order(bundle: &ProjectSourceBundle) -> Vec<String> {
    fn visit(bundle: &ProjectSourceBundle, path: &str, order: &mut Vec<String>) {
        if order
            .iter()
            .any(|existing| existing.eq_ignore_ascii_case(path))
        {
            return;
        }
        if bundle.role_for_path(path) == Some(ProjectSourceRole::VerilogABuildProfile) {
            return;
        }
        order.push(path.to_owned());
        for edge in bundle
            .dependencies()
            .iter()
            .filter(|edge| edge.importer().eq_ignore_ascii_case(path))
        {
            visit(bundle, edge.imported(), order);
        }
    }

    let mut order = Vec::new();
    visit(bundle, bundle.root().logical_path(), &mut order);
    order
}

/// Compile the exact current project source. Native builds use an operating-
/// system thread; browser builds dispatch the same sealed bundle to a dedicated
/// module worker so compilation never stalls egui's event/rendering thread.
pub(crate) fn start_veriloga_compile(app: &mut RSpiceApp, repaint: egui::Context) {
    if app.state.ui.code_workspace.veriloga.pending.is_some() {
        return;
    }
    let selected = match selected_veriloga_source(app) {
        Ok(selected) => selected,
        Err(error) => {
            app.state.push_user_message(ConsoleMessage::error(error));
            return;
        }
    };
    let token = selected.token(app.state.workspace.project.id());
    let (sender, receiver) = mpsc::channel();
    Arc::make_mut(&mut app.state.ui.code_workspace.veriloga.last_failure).clear();
    app.state.ui.code_workspace.veriloga.last_failure_token = None;
    app.state.ui.code_workspace.veriloga.pending = Some(PendingVerilogACompile {
        token,
        receiver: Arc::new(Mutex::new(receiver)),
    });

    #[cfg(not(target_arch = "wasm32"))]
    std::thread::spawn(move || {
        let outcome = compile_selected_source(&selected);
        let _ = sender.send(outcome);
        repaint.request_repaint();
    });

    #[cfg(target_arch = "wasm32")]
    {
        if let Err(error) =
            super::veriloga_worker::start(&selected, sender.clone(), repaint.clone())
        {
            let _ = sender.send(super::veriloga_worker::transport_failure_outcome(error));
            repaint.request_repaint();
        }
    }
}

/// Cancel transient compile work before source/module identity is replaced.
/// Native work is detached exactly as before; browser work is also terminated
/// so abandoned compilation cannot consume CPU or block the next request.
pub(crate) fn cancel_veriloga_compile(app: &mut RSpiceApp) {
    #[cfg(target_arch = "wasm32")]
    super::veriloga_worker::cancel();
    app.state.ui.code_workspace.veriloga.pending = None;
}

/// Poll a pending compile and publish it only when its revision and digest
/// still identify the exact visible project source.
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
        selected.matches_token(app.state.workspace.project.id(), pending.token)
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
                .project_sources
                .get_bundle(pending.token.bundle_id)
                .and_then(ProjectSourceBundle::validated_identity);
            match app
                .state
                .workspace
                .project_sources
                .mark_bundle_validated(pending.token.bundle_id)
            {
                Ok(_) => {
                    if previous_identity
                        != app
                            .state
                            .workspace
                            .project_sources
                            .get_bundle(pending.token.bundle_id)
                            .and_then(ProjectSourceBundle::validated_identity)
                    {
                        app.state.workspace.project_sources_dirty = true;
                    }
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
    if !selected.matches_token(app.state.workspace.project.id(), token) {
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
    let record = crate::state::ProjectSourceQualificationRecord {
        sequence: 0,
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
    app.state
        .workspace
        .append_project_source_qualification(token.bundle_id, record)
        .map_err(|error| error.to_string())
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

fn compile_selected_source(selected: &SelectedVerilogASource) -> VerilogACompileOutcome {
    compile_project_bundle_source(selected.bundle(), selected.selected_module())
}

pub(super) fn compile_project_bundle_source(
    bundle: &ProjectSourceBundle,
    selected_module: Option<&str>,
) -> VerilogACompileOutcome {
    let resolved = match super::veriloga_profile::resolve_veriloga_build_profile(bundle) {
        Ok(resolved) => resolved,
        Err(error) => return build_profile_error_outcome(error),
    };
    let compiler = VerilogACompiler::new(resolved.profile.compiler_options());
    let qualifications = resolved.profile.qualification_options();
    let source = bundle.root().content();
    let selected_module =
        selected_module.or_else(|| match resolved.profile.entry_modules.as_slice() {
            [module] => Some(module.as_str()),
            _ => None,
        });
    let has_source_dependencies = bundle.files().iter().any(|file| {
        bundle.role_for_path(file.logical_path()) != Some(ProjectSourceRole::VerilogABuildProfile)
    });
    if let Some(module_name) = selected_module {
        let bundle = match project_bundle_as_virtual_with_profile(bundle, &resolved.profile) {
            Ok(bundle) => bundle,
            Err(error) => {
                return VerilogACompileOutcome::Failure(vec![CodeEditorDiagnostic::current(
                    "rspice.veriloga.bundle",
                    "VA-SOURCE-CLOSURE",
                    CodeEditorSeverity::Error,
                    error,
                    "sealed project source closure",
                    None,
                    None,
                    None,
                    None,
                    None,
                )]);
            }
        };
        return match compiler.compile_virtual_runtime_diagnosed_with_qualifications(
            &bundle,
            module_name,
            crate::simulation::veriloga::project_virtual_compile_limits(),
            qualifications,
        ) {
            Ok(compilation) => successful_compile_outcome(compilation.runtime, &resolved.profile),
            Err(failure) => virtual_compile_error_outcome(failure),
        };
    }
    if has_source_dependencies || resolved.profile.entry_modules.len() > 1 {
        return VerilogACompileOutcome::Failure(vec![CodeEditorDiagnostic::current(
            "rspice.veriloga.bundle",
            "VA-ROOT-MODULE-REQUIRED",
            CodeEditorSeverity::Error,
            "Select the root module before compiling this multi-file Verilog-A bundle.",
            "Enter the exact module identifier in the Model project navigator.",
            None,
            None,
            None,
            None,
            None,
        )]);
    }
    match compiler.compile_runtime_with_qualifications(source, None, qualifications) {
        Ok(report) => successful_compile_outcome(report, &resolved.profile),
        Err(error) => compile_error_outcome(source, &error),
    }
}

fn successful_compile_outcome(
    report: RuntimeCompileReport,
    profile: &super::veriloga_profile::VerilogABuildProfile,
) -> VerilogACompileOutcome {
    match validate_profile_cell_bindings(&report, profile) {
        Ok(()) => VerilogACompileOutcome::Success(Box::new(report)),
        Err(error) => build_profile_error_outcome(error),
    }
}

fn validate_profile_cell_bindings(
    report: &RuntimeCompileReport,
    profile: &super::veriloga_profile::VerilogABuildProfile,
) -> Result<(), String> {
    for (instance_path, expected_module) in &profile.cell_bindings {
        let Some(actual) = report
            .specialist
            .evidence
            .instance_bindings
            .iter()
            .find(|binding| binding.instance_path == *instance_path)
        else {
            return Err(format!(
                "Cell-model binding '{instance_path}' references an instance path that is not present in the elaboration graph rooted at selected module '{}'.",
                report.abi.module_name
            ));
        };
        if actual.module_name != *expected_module {
            return Err(format!(
                "Cell-model binding '{instance_path}' expects module '{expected_module}', but the analyzed instance binds '{}'.",
                actual.module_name
            ));
        }
    }
    Ok(())
}

fn build_profile_error_outcome(error: String) -> VerilogACompileOutcome {
    VerilogACompileOutcome::Failure(vec![CodeEditorDiagnostic::current(
        "rspice.veriloga.build-profile",
        "VA-BUILD-PROFILE",
        CodeEditorSeverity::Error,
        "Verilog-A build profile is invalid",
        error,
        None,
        None,
        None,
        None,
        None,
    )])
}

fn compile_error_outcome(
    source: &str,
    error: &rspice_veriloga::CompileError,
) -> VerilogACompileOutcome {
    let diagnostics = rspice_veriloga::compile_diagnostics(source, error)
        .into_iter()
        .map(|diagnostic| {
            let byte_range = diagnostic.span.as_ref().and_then(|span| {
                let start = usize::try_from(span.byte_start).ok()?;
                let end = usize::try_from(span.byte_end).ok()?;
                (start <= end && end <= source.len()).then_some(start..end)
            });
            let position = diagnostic.span.as_ref().and_then(|span| span.start);
            CodeEditorDiagnostic::current(
                "rspice.veriloga.compiler",
                format!(
                    "VA-{}",
                    diagnostic_phase_label(diagnostic.phase).to_ascii_uppercase()
                ),
                CodeEditorSeverity::Error,
                diagnostic.message,
                diagnostic_phase_label(diagnostic.phase),
                None,
                None,
                byte_range,
                position.and_then(|position| usize::try_from(position.line).ok()),
                position.and_then(|position| usize::try_from(position.column).ok()),
            )
        })
        .collect();
    VerilogACompileOutcome::Failure(diagnostics)
}

fn virtual_compile_error_outcome(
    failure: rspice_veriloga::VirtualRuntimeCompileFailure,
) -> VerilogACompileOutcome {
    let diagnostics = failure
        .diagnostics
        .into_iter()
        .map(|diagnostic| {
            let byte_range = diagnostic
                .byte_start
                .zip(diagnostic.byte_end)
                .and_then(|(start, end)| (start <= end).then_some(start..end));
            let source_label = diagnostic.logical_path.as_deref().map_or_else(
                || diagnostic_phase_label(diagnostic.phase).to_owned(),
                |path| match (diagnostic.line, diagnostic.column) {
                    (Some(line), Some(column)) => format!(
                        "{} · {path}:{line}:{column}",
                        diagnostic_phase_label(diagnostic.phase)
                    ),
                    _ => format!("{} · {path}", diagnostic_phase_label(diagnostic.phase)),
                },
            );
            CodeEditorDiagnostic::current(
                "rspice.veriloga.compiler",
                format!(
                    "VA-{}",
                    diagnostic_phase_label(diagnostic.phase).to_ascii_uppercase()
                ),
                CodeEditorSeverity::Error,
                diagnostic.message,
                source_label,
                diagnostic.logical_path,
                diagnostic.source,
                byte_range,
                diagnostic.line,
                diagnostic.column,
            )
        })
        .collect();
    VerilogACompileOutcome::Failure(diagnostics)
}

fn project_bundle_as_virtual_with_profile(
    bundle: &ProjectSourceBundle,
    profile: &super::veriloga_profile::VerilogABuildProfile,
) -> Result<VirtualSourceBundle, String> {
    let files = std::iter::once(VirtualSourceFile::new(
        bundle.root().logical_path(),
        bundle.root().content(),
    ))
    .chain(
        bundle
            .files()
            .iter()
            .filter(|file| {
                bundle.role_for_path(file.logical_path())
                    != Some(ProjectSourceRole::VerilogABuildProfile)
            })
            .map(|file| VirtualSourceFile::new(file.logical_path(), file.content())),
    );
    VirtualSourceBundle::new_with_include_paths(
        bundle.root().logical_path(),
        files,
        profile.include_paths.iter().cloned(),
    )
    .map_err(|error| error.to_string())
}

pub(crate) fn compile_project_bundle_virtual_for_provenance(
    bundle: &ProjectSourceBundle,
    selected_module: &str,
) -> Result<VirtualRuntimeCompilation, String> {
    let resolved = super::veriloga_profile::resolve_veriloga_build_profile(bundle)?;
    if !resolved.profile.entry_modules.is_empty()
        && !resolved
            .profile
            .entry_modules
            .iter()
            .any(|module| module == selected_module)
    {
        return Err(format!(
            "Selected module '{selected_module}' is not declared by the Verilog-A build profile."
        ));
    }
    let virtual_bundle = project_bundle_as_virtual_with_profile(bundle, &resolved.profile)?;
    let compilation = VerilogACompiler::new(resolved.profile.compiler_options())
        .compile_virtual_runtime_diagnosed_with_qualifications(
            &virtual_bundle,
            selected_module,
            crate::simulation::veriloga::project_virtual_compile_limits(),
            resolved.profile.qualification_options(),
        )
        .map_err(|failure| failure.to_string())?;
    validate_profile_cell_bindings(&compilation.runtime, &resolved.profile)?;
    Ok(compilation)
}

pub(crate) fn compile_project_bundle_receipt(
    project_id: crate::product::ProjectId,
    bundle: &ProjectSourceBundle,
    selected_module: Option<&str>,
) -> Result<VerilogACompileReceipt, Vec<CodeEditorDiagnostic>> {
    let selected = SelectedVerilogASource {
        bundle: bundle.clone(),
        selected_module: selected_module.map(str::to_owned),
    };
    let token = selected.token(project_id);
    match compile_selected_source(&selected) {
        VerilogACompileOutcome::Success(report) => {
            receipt_from_report(token, &report, Some(bundle))
                .map_err(|error| vec![diagnostic_capacity_failure(token, error)])
        }
        VerilogACompileOutcome::Failure(diagnostics) => Err(diagnostics
            .into_iter()
            .map(|diagnostic| {
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
    let diagnostics = bundle
        .and_then(|bundle| {
            super::veriloga_profile::resolve_veriloga_build_profile(bundle)
                .ok()
                .map(|resolved| specialist_diagnostics(report, &resolved.profile, token))
        })
        .unwrap_or_default();
    let diagnostics = Arc::new(CodeDiagnosticCollection::try_new(diagnostics)?);
    Ok(VerilogACompileReceipt {
        token,
        module_name: report.abi.module_name.to_string(),
        analog_ports: report.abi.analog_port_count(),
        noise_sources: report.abi.noise_source_count,
        state_variables: report.abi.state_variable_count,
        bytecode_available: report.targets.is_available(RuntimeTarget::BytecodeVm),
        native_jit: target_qualification(report.targets.get(RuntimeTarget::NativeJit)),
        wasm_interpreter: target_qualification(report.targets.get(RuntimeTarget::WasmInterpreter)),
        generated_rust: target_qualification(report.targets.get(RuntimeTarget::GeneratedRust)),
        diagnostics,
        report: Arc::new(report.clone()),
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

fn specialist_diagnostics(
    report: &RuntimeCompileReport,
    profile: &super::veriloga_profile::VerilogABuildProfile,
    token: VerilogASourceOperationToken,
) -> Vec<CodeEditorDiagnostic> {
    report
        .specialist
        .findings
        .iter()
        .filter(|finding| specialist_check_enabled(profile, finding.check))
        .map(|finding| {
            let severity = match finding.severity {
                rspice_veriloga::SpecialistFindingSeverity::Information => CodeEditorSeverity::Info,
                rspice_veriloga::SpecialistFindingSeverity::Warning => CodeEditorSeverity::Warning,
                rspice_veriloga::SpecialistFindingSeverity::Error => CodeEditorSeverity::Error,
            };
            let mut detail = finding.detail.clone();
            if let Some(action) = &finding.action {
                detail.push_str(" Suggested review: ");
                detail.push_str(&action.title);
                detail.push_str(" — ");
                detail.push_str(&action.replacement_hint);
            }
            CodeEditorDiagnostic::current(
                "rspice.veriloga.specialist",
                &finding.code,
                severity,
                finding.summary.as_str(),
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
        })
        .collect()
}

const fn specialist_check_enabled(
    profile: &super::veriloga_profile::VerilogABuildProfile,
    check: rspice_veriloga::SpecialistCheckKind,
) -> bool {
    match check {
        rspice_veriloga::SpecialistCheckKind::HiddenState => profile.checks.hidden_state,
        rspice_veriloga::SpecialistCheckKind::Discontinuity => profile.checks.discontinuities,
        rspice_veriloga::SpecialistCheckKind::UnitsAndRanges => profile.checks.units_and_ranges,
        rspice_veriloga::SpecialistCheckKind::Convergence => profile.checks.convergence,
        rspice_veriloga::SpecialistCheckKind::Portability => profile.checks.portability,
    }
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

const fn diagnostic_phase_label(phase: CompileDiagnosticPhase) -> &'static str {
    match phase {
        CompileDiagnosticPhase::Input => "input",
        CompileDiagnosticPhase::Lexer => "lexer",
        CompileDiagnosticPhase::Parser => "parser",
        CompileDiagnosticPhase::Semantic => "semantic analysis",
        CompileDiagnosticPhase::CodeGeneration => "code generation",
        CompileDiagnosticPhase::BackendQualification => "backend qualification",
        CompileDiagnosticPhase::PerformanceBudget => "performance budget",
        CompileDiagnosticPhase::ModuleSelection => "module selection",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Cell, CellViewRef, Library, ProjectSourceDocument, View};

    #[test]
    fn editor_and_execution_share_the_project_macro_expansion_contract() {
        let limits = crate::simulation::veriloga::project_virtual_compile_limits();
        assert_eq!(
            limits.max_expanded_bytes,
            crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES.saturating_mul(2)
        );
    }

    #[test]
    fn included_virtual_diagnostic_keeps_the_included_document_identity() {
        let child = "module selected(p, n);\n  inout p, n;\n  electrical p, n;\n  analog I(p, n) <+ @;\nendmodule\n";
        let bundle = VirtualSourceBundle::from_sources(
            "root.va",
            [("root.va", "`include \"child.va\"\n"), ("child.va", child)],
        )
        .expect("valid virtual diagnostic fixture");
        let failure = VerilogACompiler::default()
            .compile_virtual_runtime_diagnosed(
                &bundle,
                "selected",
                rspice_veriloga::VirtualCompileLimits::default(),
            )
            .expect_err("included syntax error must fail");

        let VerilogACompileOutcome::Failure(diagnostics) = virtual_compile_error_outcome(failure)
        else {
            panic!("diagnosed compile failure cannot publish success");
        };
        let diagnostic = diagnostics
            .iter()
            .find(|diagnostic| diagnostic.source_path.as_deref() == Some("child.va"))
            .expect("included diagnostic");

        assert_eq!(diagnostic.source.as_deref(), Some(child));
        assert_eq!(diagnostic.line, Some(4));
        assert!(diagnostic.detail.contains("child.va:4"));
        let range = diagnostic.byte_range.clone().expect("included byte range");
        assert_eq!(&child[range], "@");
    }

    fn ensure_legacy_source(app: &mut RSpiceApp) {
        let owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA);
        if app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .is_none()
        {
            app.state
                .workspace
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
        }
    }

    #[test]
    fn compile_review_migrates_legacy_defaults_to_a_persisted_role() {
        let mut app = RSpiceApp::test_instance();
        ensure_legacy_source(&mut app);

        open_veriloga_compile_dialog(&mut app).unwrap();

        let owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA);
        let bundle = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .unwrap();
        let profile_path = bundle
            .paths_for_role(ProjectSourceRole::VerilogABuildProfile)
            .next()
            .unwrap();
        assert_eq!(profile_path, ".rspice/veriloga-build.toml");
        assert!(!bundle.root().content().contains(profile_path));
        assert!(app.state.workspace.project_sources_dirty);
        let dialog = app
            .state
            .ui
            .code_workspace
            .veriloga
            .compile_dialog
            .as_ref()
            .unwrap();
        assert_eq!(dialog.profile_path, profile_path);
        assert_eq!(dialog.bundle_revision, bundle.revision().get());
        assert_eq!(dialog.closure_digest, bundle.closure_digest());
    }

    #[test]
    fn compile_review_rejects_a_profile_changed_after_review() {
        let mut app = RSpiceApp::test_instance();
        ensure_legacy_source(&mut app);
        open_veriloga_compile_dialog(&mut app).unwrap();
        let dialog = app
            .state
            .ui
            .code_workspace
            .veriloga
            .compile_dialog
            .clone()
            .unwrap();
        let source = app
            .state
            .workspace
            .project_sources
            .get_bundle(dialog.bundle_id)
            .unwrap()
            .file_content(&dialog.profile_path)
            .unwrap()
            .to_owned();
        app.state
            .workspace
            .replace_project_source_bundle_file(
                dialog.bundle_id,
                &dialog.profile_path,
                format!("{source}\n# changed after review\n"),
            )
            .unwrap();

        let error = commit_veriloga_compile_dialog(&mut app, egui::Context::default())
            .expect_err("stale review must not launch compilation");
        assert!(error.contains("source changed"));
        assert!(app.state.ui.code_workspace.veriloga.pending.is_none());
    }

    #[test]
    fn qualification_history_is_append_only_without_changing_source_identity() {
        let mut app = RSpiceApp::test_instance();
        ensure_legacy_source(&mut app);
        open_veriloga_compile_dialog(&mut app).unwrap();
        let selected = selected_veriloga_source(&app).unwrap();
        let token = selected.token(app.state.workspace.project.id());
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
            .project_sources
            .get_bundle(token.bundle_id)
            .unwrap();
        assert_eq!((first, second), (1, 2));
        assert_eq!(bundle.qualifications().len(), 2);
        assert_eq!(bundle.revision(), revision);
        assert_eq!(bundle.closure_digest(), closure_digest);
        bundle.validate().unwrap();
    }

    fn cell_binding_report() -> RuntimeCompileReport {
        VerilogACompiler::default()
            .compile_runtime(
                r#"
module leaf(p, n);
  inout p, n; electrical p, n;
endmodule
module child(p, n);
  inout p, n; electrical p, n;
  leaf inner (.p(p), .n(n));
endmodule
module top(p, n);
  inout p, n; electrical p, n;
  child outer (.p(p), .n(n));
endmodule
"#,
                Some("top"),
            )
            .unwrap()
    }

    #[test]
    fn build_profile_cell_bindings_match_exact_elaboration_paths() {
        let report = cell_binding_report();
        let mut profile = crate::workbench::documents::code_workspace::veriloga_profile::VerilogABuildProfile::starter("top");
        profile
            .cell_bindings
            .insert("outer".to_owned(), "child".to_owned());
        profile
            .cell_bindings
            .insert("outer/inner".to_owned(), "leaf".to_owned());

        validate_profile_cell_bindings(&report, &profile).unwrap();
    }

    #[test]
    fn build_profile_cell_bindings_fail_closed_on_path_or_module_mismatch() {
        let report = cell_binding_report();
        let mut profile = crate::workbench::documents::code_workspace::veriloga_profile::VerilogABuildProfile::starter("top");
        profile
            .cell_bindings
            .insert("outer/inner".to_owned(), "wrong".to_owned());
        let mismatch = validate_profile_cell_bindings(&report, &profile).unwrap_err();
        assert!(mismatch.contains("expects module 'wrong'"));

        profile.cell_bindings.clear();
        profile
            .cell_bindings
            .insert("inner".to_owned(), "leaf".to_owned());
        let missing = validate_profile_cell_bindings(&report, &profile).unwrap_err();
        assert!(missing.contains("not present in the elaboration graph"));
    }

    fn add_cell_bundle(app: &mut RSpiceApp, cell_name: &str) -> CellViewRef {
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
            .insert("veriloga.module".to_owned(), "shared_model".to_owned());
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
            "shared.va",
            "module shared_model(p, n); inout p, n; electrical p, n; endmodule\n",
            [],
            [],
        )
        .unwrap();
        app.state
            .workspace
            .project_sources
            .insert_bundle(bundle)
            .unwrap();
        reference
    }

    #[test]
    fn non_veriloga_workspace_view_preserves_legacy_singleton_selection() {
        let mut app = RSpiceApp::test_instance();
        ensure_legacy_source(&mut app);

        let selected = selected_veriloga_source(&app).unwrap();

        assert_eq!(
            selected.bundle().owner(),
            &ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA)
        );
        assert!(selected.selected_module().is_none());
    }

    #[test]
    fn dropped_veriloga_source_reuses_the_exact_import_transition() {
        let mut app = RSpiceApp::test_instance();
        ensure_legacy_source(&mut app);
        let selected = selected_veriloga_source(&app).unwrap();
        let bundle_id = selected.bundle().id();
        let before = selected.bundle().revision().get();
        let source = "module dropped(p, n); inout p, n; electrical p, n; endmodule\n";

        import_dropped_veriloga_source(&mut app, "dropped.va".to_owned(), source.to_owned())
            .expect("drop import");

        let bundle = app
            .state
            .workspace
            .project_sources
            .get_bundle(bundle_id)
            .unwrap();
        assert!(bundle.revision().get() > before);
        assert_eq!(bundle.root().content(), source);
    }

    #[test]
    fn root_import_initializes_an_empty_code_workspace_without_demo_identity() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .workspace
            .remove_project_source(ProjectSourceLanguage::VerilogA);
        let source = "module actual_model(p, n); inout p, n; electrical p, n; endmodule\n";

        request_veriloga_root_import(&mut app).unwrap();
        apply_import(
            &mut app,
            "behavioral/presentation_name.va".to_owned(),
            source.to_owned(),
        );

        let owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA);
        let bundle = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .expect("root import creates the missing source workspace");
        assert_eq!(
            bundle.root().logical_path(),
            "behavioral/presentation_name.va"
        );
        assert_eq!(bundle.root().content(), source);
        let resolved = crate::workbench::documents::code_workspace::veriloga_profile::resolve_veriloga_build_profile(bundle).unwrap();
        assert!(resolved.profile.entry_modules.is_empty());
        compile_project_bundle_receipt(app.state.workspace.project.id(), bundle, None)
            .expect("a single imported module compiles without filename inference");
        assert!(app.state.workspace.project_sources_dirty);
        assert!(
            app.state
                .ui
                .code_workspace
                .veriloga
                .root_import_target
                .is_none()
        );
    }

    #[test]
    fn root_import_rejects_a_bundle_changed_while_the_picker_is_open() {
        let mut app = RSpiceApp::test_instance();
        ensure_legacy_source(&mut app);
        let selected = selected_veriloga_source(&app).unwrap();
        let bundle_id = selected.bundle().id();
        let root = selected.document().logical_path().to_owned();

        request_veriloga_root_import(&mut app).unwrap();
        app.state
            .workspace
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
                .root_import_target
                .is_none()
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
            .project_sources
            .mark_bundle_validated(selected.bundle().id())
            .unwrap();
        let receipt = compile_project_bundle_receipt(
            app.state.workspace.project.id(),
            selected.bundle(),
            selected.selected_module(),
        )
        .unwrap();
        app.state.ui.code_workspace.veriloga.receipt = Some(receipt);
        let before = selected.token(app.state.workspace.project.id());

        assert!(
            replace_selected_veriloga_source(
                &mut app,
                &selected,
                "module shared_model(p, n); inout p, n; electrical p, n; real changed; endmodule\n"
                    .to_owned(),
            )
            .unwrap()
        );

        let edited = selected_veriloga_source(&app).unwrap();
        let after = edited.token(app.state.workspace.project.id());
        assert_eq!(after.bundle_id, before.bundle_id);
        assert!(after.revision > before.revision);
        assert_ne!(after.closure_digest, before.closure_digest);
        assert!(!edited.bundle().validation_is_current());
        assert!(app.state.ui.code_workspace.veriloga.receipt.is_none());
        assert!(app.state.workspace.project_sources_dirty);
    }

    #[test]
    fn code_workspace_multifile_bundle_compiles_with_explicit_root_module() {
        let mut app = RSpiceApp::test_instance();
        ensure_legacy_source(&mut app);
        let selected = selected_veriloga_source(&app).unwrap();
        let bundle_id = selected.bundle().id();
        let root_path = selected.document().logical_path().to_owned();
        app.state
            .workspace
            .project_sources
            .replace_bundle_file_content(
                bundle_id,
                &root_path,
                "module legacy(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n"
                    .to_owned(),
            )
            .unwrap();
        add_bundle_file(
            &mut app,
            bundle_id,
            &root_path,
            "support/constants.vams".to_owned(),
            "`define SCALE 1\n".to_owned(),
        )
        .unwrap();
        app.state.ui.code_workspace.veriloga.selected_module = "legacy".to_owned();

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
    fn completed_async_compile_cannot_cross_publish_between_cell_views() {
        let mut app = RSpiceApp::test_instance();
        let first = add_cell_bundle(&mut app, "first");
        let second = add_cell_bundle(&mut app, "second");
        app.state.workspace.open_view(first, ViewType::VerilogA);
        let first_selected = selected_veriloga_source(&app).unwrap();
        let receipt = compile_project_bundle_receipt(
            app.state.workspace.project.id(),
            first_selected.bundle(),
            first_selected.selected_module(),
        )
        .unwrap();
        let (sender, receiver) = mpsc::channel();
        sender
            .send(VerilogACompileOutcome::Success(Box::new(
                (*receipt.report).clone(),
            )))
            .unwrap();
        app.state.ui.code_workspace.veriloga.receipt = None;
        app.state.ui.code_workspace.veriloga.pending = Some(PendingVerilogACompile {
            token: receipt.token,
            receiver: Arc::new(Mutex::new(receiver)),
        });
        app.state.workspace.open_view(second, ViewType::VerilogA);

        poll_veriloga_compile(&mut app);

        assert!(app.state.ui.code_workspace.veriloga.pending.is_none());
        assert!(app.state.ui.code_workspace.veriloga.receipt.is_none());
        let second_selected = selected_veriloga_source(&app).unwrap();
        assert!(!second_selected.matches_token(app.state.workspace.project.id(), receipt.token));
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
            app.state.workspace.project.id(),
            selected.bundle(),
            selected.selected_module(),
        )
        .unwrap();
        let (sender, receiver) = mpsc::channel();
        sender
            .send(VerilogACompileOutcome::Success(Box::new(
                (*receipt.report).clone(),
            )))
            .unwrap();
        app.state.ui.code_workspace.veriloga.pending = Some(PendingVerilogACompile {
            token: receipt.token,
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
        assert!(!changed.matches_token(app.state.workspace.project.id(), receipt.token));
        assert!(!changed.bundle().validation_is_current());
    }
}
