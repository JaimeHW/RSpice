//! Governed in-memory Verilog-A compilation for the Code workspace.

use std::sync::{Arc, Mutex, mpsc};

use rspice_veriloga::{
    CompileDiagnosticPhase, CompilerOptions, RuntimeCompileReport, RuntimeTarget,
    RuntimeTargetMaturity, RuntimeTargetQualification, RuntimeTargetReadiness, VerilogACompiler,
    VirtualSourceBundle, VirtualSourceFile,
};

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    ProjectSourceBundle, ProjectSourceFile, ProjectSourceId, ProjectSourceLanguage,
    ProjectSourceOwner, ViewType,
};
use crate::workbench::RSpiceApp;

use crate::simulation::veriloga::VerilogASourceOperationToken;

use super::{
    CodeEditorDiagnostic, CodeEditorSeverity, PendingVerilogACompile, TargetQualification,
    VerilogACompileOutcome, VerilogACompileReceipt, VerilogAFileSelection,
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
        Ok(None) => app.state.ui.code_workspace.veriloga.import_target = None,
        Err(error) => {
            app.state.ui.code_workspace.veriloga.import_target = None;
            app.state.push_user_message(ConsoleMessage::error(format!(
                "Verilog-A import failed: {error}"
            )));
        }
    }
}

fn apply_import(app: &mut RSpiceApp, file_name: String, contents: String) {
    if let Some(target) = app.state.ui.code_workspace.veriloga.import_target.take() {
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

fn invalidate_veriloga_evidence(app: &mut RSpiceApp) {
    app.state.workspace.project_sources_dirty = true;
    cancel_veriloga_compile(app);
    app.state.ui.code_workspace.veriloga.receipt = None;
    app.state.ui.code_workspace.veriloga.last_failure.clear();
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

/// Compile the exact current project source. Native builds use an operating-
/// system thread; browser builds dispatch the same sealed bundle to a dedicated
/// module worker so compilation never stalls egui's event/rendering thread.
pub fn start_veriloga_compile(app: &mut RSpiceApp, repaint: egui::Context) {
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
    app.state.ui.code_workspace.veriloga.last_failure.clear();
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
            let receipt = receipt_from_report(pending.token, &report);
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
                    app.state.ui.code_workspace.veriloga.last_failure.clear();
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
            app.state.ui.code_workspace.veriloga.receipt = None;
            app.state.ui.code_workspace.veriloga.last_failure = diagnostics;
            app.state.ui.code_workspace.veriloga.last_failure_token = Some(pending.token);
            app.state.push_user_message(ConsoleMessage::error(
                "Verilog-A compilation failed. Review the source diagnostics.",
            ));
        }
        Err(()) => {
            app.state.ui.code_workspace.veriloga.receipt = None;
            app.state.ui.code_workspace.veriloga.last_failure = vec![CodeEditorDiagnostic {
                severity: CodeEditorSeverity::Error,
                message: "Compiler worker stopped unexpectedly".to_owned(),
                detail: "No compiler report was published; the source was not validated."
                    .to_owned(),
                source_path: None,
                source: None,
                byte_range: None,
                line: None,
                column: None,
            }];
            app.state.ui.code_workspace.veriloga.last_failure_token = Some(pending.token);
            app.state.push_user_message(ConsoleMessage::error(
                "The Verilog-A compiler worker stopped before publishing a report.",
            ));
        }
    }
}

fn compile_selected_source(selected: &SelectedVerilogASource) -> VerilogACompileOutcome {
    compile_project_bundle_source(selected.bundle(), selected.selected_module())
}

pub(super) fn compile_project_bundle_source(
    bundle: &ProjectSourceBundle,
    selected_module: Option<&str>,
) -> VerilogACompileOutcome {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let source = bundle.root().content();
    if let Some(module_name) = selected_module {
        let bundle = match project_bundle_as_virtual(bundle) {
            Ok(bundle) => bundle,
            Err(error) => {
                return VerilogACompileOutcome::Failure(vec![CodeEditorDiagnostic {
                    severity: CodeEditorSeverity::Error,
                    message: error,
                    detail: "sealed project source closure".to_owned(),
                    source_path: None,
                    source: None,
                    byte_range: None,
                    line: None,
                    column: None,
                }]);
            }
        };
        return match compiler.compile_virtual_runtime_diagnosed(
            &bundle,
            module_name,
            crate::simulation::veriloga::project_virtual_compile_limits(),
        ) {
            Ok(compilation) => VerilogACompileOutcome::Success(Box::new(compilation.runtime)),
            Err(failure) => virtual_compile_error_outcome(failure),
        };
    }
    if !bundle.files().is_empty() {
        return VerilogACompileOutcome::Failure(vec![CodeEditorDiagnostic {
            severity: CodeEditorSeverity::Error,
            message: "Select the root module before compiling this multi-file Verilog-A bundle."
                .to_owned(),
            detail: "Enter the exact module identifier in the Model project navigator.".to_owned(),
            source_path: None,
            source: None,
            byte_range: None,
            line: None,
            column: None,
        }]);
    }
    match compiler.compile_runtime(source, None) {
        Ok(report) => VerilogACompileOutcome::Success(Box::new(report)),
        Err(error) => compile_error_outcome(source, &error),
    }
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
            CodeEditorDiagnostic {
                severity: CodeEditorSeverity::Error,
                message: diagnostic.message,
                detail: diagnostic_phase_label(diagnostic.phase).to_owned(),
                source_path: None,
                source: None,
                byte_range,
                line: position.and_then(|position| usize::try_from(position.line).ok()),
                column: position.and_then(|position| usize::try_from(position.column).ok()),
            }
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
            CodeEditorDiagnostic {
                severity: CodeEditorSeverity::Error,
                message: diagnostic.message,
                detail: source_label,
                source_path: diagnostic.logical_path,
                source: diagnostic.source,
                byte_range,
                line: diagnostic.line,
                column: diagnostic.column,
            }
        })
        .collect();
    VerilogACompileOutcome::Failure(diagnostics)
}

pub(crate) fn project_bundle_as_virtual(
    bundle: &ProjectSourceBundle,
) -> Result<VirtualSourceBundle, String> {
    let files = std::iter::once(VirtualSourceFile::new(
        bundle.root().logical_path(),
        bundle.root().content(),
    ))
    .chain(
        bundle
            .files()
            .iter()
            .map(|file| VirtualSourceFile::new(file.logical_path(), file.content())),
    );
    VirtualSourceBundle::new(bundle.root().logical_path(), files).map_err(|error| error.to_string())
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
        VerilogACompileOutcome::Success(report) => Ok(receipt_from_report(token, &report)),
        VerilogACompileOutcome::Failure(diagnostics) => Err(diagnostics),
    }
}

fn receipt_from_report(
    token: VerilogASourceOperationToken,
    report: &RuntimeCompileReport,
) -> VerilogACompileReceipt {
    VerilogACompileReceipt {
        token,
        module_name: report.abi.module_name.to_string(),
        analog_ports: report.abi.analog_port_count(),
        noise_sources: report.abi.noise_source_count,
        state_variables: report.abi.state_variable_count,
        bytecode_available: report.targets.is_available(RuntimeTarget::BytecodeVm),
        native_jit: target_qualification(report.targets.get(RuntimeTarget::NativeX64Jit)),
        wasm_interpreter: target_qualification(report.targets.get(RuntimeTarget::WasmInterpreter)),
        generated_rust: target_qualification(report.targets.get(RuntimeTarget::GeneratedRust)),
        diagnostics: Vec::new(),
        report: Arc::new(report.clone()),
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
