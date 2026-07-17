//! Governed in-memory Verilog-A compilation for the Code workspace.

use std::sync::{Arc, Mutex, mpsc};

use rspice_veriloga::{
    CompileDiagnosticPhase, CompilerOptions, RuntimeCompileReport, RuntimeTarget,
    RuntimeTargetMaturity, RuntimeTargetQualification, RuntimeTargetReadiness, VerilogACompiler,
};

use crate::common::{ConsoleMessage, RSpiceApp};
use crate::state::ProjectSourceLanguage;

use super::{
    CodeEditorDiagnostic, CodeEditorSeverity, PendingVerilogACompile, SourceOperationToken,
    TargetQualification, VerilogACompileOutcome, VerilogACompileReceipt,
};

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
struct BrowserImportCompletion {
    token: crate::common::browser_file_import::TextImportToken,
    result: Result<Option<crate::common::browser_file_import::PickedTextFile>, String>,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_IMPORT_COMPLETION: std::cell::RefCell<Option<BrowserImportCompletion>> =
        const { std::cell::RefCell::new(None) };
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
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Verilog-A", &["va"])
            .set_title("Import Verilog-A source")
            .pick_file()
        else {
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
            Err(error) => app.state.push_user_message(ConsoleMessage::error(format!(
                "Verilog-A import failed: {error}"
            ))),
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let token = match crate::common::browser_file_import::try_begin_text_import(
            crate::common::browser_file_import::BrowserTextImportKind::VerilogA,
        ) {
            Ok(token) => token,
            Err(error) => {
                app.state.push_user_message(ConsoleMessage::error(format!(
                    "Verilog-A import failed: {error}"
                )));
                return;
            }
        };
        app.state.ui.code_workspace.veriloga.import_in_progress = true;
        crate::common::browser_file_import::pick_text_file("Verilog-A", &["va"], move |result| {
            if crate::common::browser_file_import::text_import_is_current(token) {
                BROWSER_IMPORT_COMPLETION.with(|slot| {
                    *slot.borrow_mut() = Some(BrowserImportCompletion { token, result });
                });
            }
        });
    }
}

#[cfg(target_arch = "wasm32")]
fn poll_browser_import_completion(app: &mut RSpiceApp) {
    let Some(completion) = BROWSER_IMPORT_COMPLETION.with(|slot| slot.borrow_mut().take()) else {
        return;
    };
    app.state.ui.code_workspace.veriloga.import_in_progress = false;
    if !crate::common::browser_file_import::finish_text_import(completion.token) {
        return;
    }
    match completion.result {
        Ok(Some(file)) => apply_import(app, file.name, file.contents),
        Ok(None) => {}
        Err(error) => app.state.push_user_message(ConsoleMessage::error(format!(
            "Verilog-A import failed: {error}"
        ))),
    }
}

fn apply_import(app: &mut RSpiceApp, file_name: String, contents: String) {
    match app.state.workspace.replace_imported_project_source(
        ProjectSourceLanguage::VerilogA,
        file_name.clone(),
        contents,
    ) {
        Ok(true) => {
            app.state.ui.code_workspace.veriloga = Default::default();
            app.state.push_user_message(ConsoleMessage::info(format!(
                "Imported project-owned Verilog-A source {file_name}; compile is required."
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

/// Compile the exact current project source. Native builds run the compiler off
/// the UI thread; wasm builds execute in-place because browser builds do not
/// have a shared-memory worker runtime.
pub fn start_veriloga_compile(app: &mut RSpiceApp, repaint: egui::Context) {
    if app.state.ui.code_workspace.veriloga.pending.is_some() {
        return;
    }
    let Some(document) = app
        .state
        .workspace
        .project_sources
        .get(ProjectSourceLanguage::VerilogA)
        .cloned()
    else {
        app.state.push_user_message(ConsoleMessage::error(
            "This project has no Verilog-A source document.",
        ));
        return;
    };
    let token = SourceOperationToken {
        project_id: app.state.workspace.project.id(),
        revision: document.revision().get(),
        content_digest: document.content_digest(),
    };
    let source = document.content().to_owned();
    let (sender, receiver) = mpsc::channel();
    app.state.ui.code_workspace.veriloga.last_failure.clear();
    app.state.ui.code_workspace.veriloga.last_failure_token = None;
    app.state.ui.code_workspace.veriloga.pending = Some(PendingVerilogACompile {
        token,
        receiver: Arc::new(Mutex::new(receiver)),
    });

    #[cfg(not(target_arch = "wasm32"))]
    std::thread::spawn(move || {
        let outcome = compile_source(&source);
        let _ = sender.send(outcome);
        repaint.request_repaint();
    });

    #[cfg(target_arch = "wasm32")]
    {
        let outcome = compile_source(&source);
        let _ = sender.send(outcome);
        repaint.request_repaint();
    }
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
    app.state.ui.code_workspace.veriloga.pending = None;

    let is_current = app
        .state
        .workspace
        .project_sources
        .get(ProjectSourceLanguage::VerilogA)
        .is_some_and(|document| {
            app.state.workspace.project.id() == pending.token.project_id
                && document.revision().get() == pending.token.revision
                && document.content_digest() == pending.token.content_digest
        });
    if !is_current {
        return;
    }

    match received {
        Ok(VerilogACompileOutcome::Success(report)) => {
            let receipt = receipt_from_report(pending.token, &report);
            match app
                .state
                .workspace
                .mark_project_source_validated(ProjectSourceLanguage::VerilogA)
            {
                Ok(_) => {
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

fn compile_source(source: &str) -> VerilogACompileOutcome {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    match compiler.compile_runtime(source, None) {
        Ok(report) => VerilogACompileOutcome::Success(Box::new(report)),
        Err(error) => {
            let diagnostics = rspice_veriloga::compile_diagnostics(source, &error)
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
                        byte_range,
                        line: position.and_then(|position| usize::try_from(position.line).ok()),
                        column: position.and_then(|position| usize::try_from(position.column).ok()),
                    }
                })
                .collect();
            VerilogACompileOutcome::Failure(diagnostics)
        }
    }
}

/// Rebuild the transient compiler receipt for an exact project document.
/// Durable validation stores only source identity; executable artifacts are
/// intentionally reconstructed in-process and never trusted from disk.
pub(crate) fn compile_project_source_receipt(
    project_id: crate::product::ProjectId,
    document: &crate::state::ProjectSourceDocument,
) -> Result<VerilogACompileReceipt, Vec<CodeEditorDiagnostic>> {
    let token = SourceOperationToken {
        project_id,
        revision: document.revision().get(),
        content_digest: document.content_digest(),
    };
    match compile_source(document.content()) {
        VerilogACompileOutcome::Success(report) => Ok(receipt_from_report(token, &report)),
        VerilogACompileOutcome::Failure(diagnostics) => Err(diagnostics),
    }
}

fn receipt_from_report(
    token: SourceOperationToken,
    report: &RuntimeCompileReport,
) -> VerilogACompileReceipt {
    VerilogACompileReceipt {
        token,
        module_name: report.abi.module_name.to_string(),
        analog_ports: report.abi.analog_port_count(),
        noise_sources: report.abi.noise_source_count,
        state_variables: report.abi.state_variable_count,
        semantic_ir_digest: report.canonical_ir.hir_digest.to_string(),
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
        CompileDiagnosticPhase::ModuleSelection => "module selection",
    }
}
