//! A Verilog-A source's compile, as the receipt the code workspace keeps.
//!
//! A receipt speaks for one exact revision of a source bundle: its findings
//! are bound to that revision, so an edit since makes them history. A compile
//! the reader starts is published from the report its worker returns; a
//! starter module, which no reader could compile before its view exists, is
//! compiled here in place.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use std::sync::Arc;

use rspice_simulation::project_veriloga::VerilogASourceOperationToken;
use rspice_veriloga::{RuntimeCompileReport, RuntimeTarget};
use rspice_vocabulary::diagnostics::{
    DiagnosticSeverity as CodeEditorSeverity, SourceDiagnostic as CodeEditorDiagnostic,
};

use crate::state::code_diagnostics::CodeDiagnosticCollection;
use crate::state::{ProjectSourceBundle, VerilogACompileReceipt};

/// Compile `bundle`'s `selected_module` in place, as the receipt a published
/// compile carries. Refused with the compiler's findings, bound to the
/// bundle's revision.
pub fn compile_project_bundle_receipt(
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
            .map(|diagnostic| bind_to_compile(CodeEditorDiagnostic::from(diagnostic), token))
            .collect()),
    }
}

/// `diagnostic` bound to the compile `token` names, in the document it was
/// found in: two findings in different documents are two findings.
fn bind_to_compile(
    diagnostic: CodeEditorDiagnostic,
    token: VerilogASourceOperationToken,
) -> CodeEditorDiagnostic {
    let document_id = diagnostic
        .source_path
        .clone()
        .unwrap_or_else(|| token.bundle_id.to_string().into());
    diagnostic.bind_validation(
        document_id,
        token.revision,
        token.closure_digest.to_string(),
    )
}

/// The receipt of the compile `token` names, from the report its worker
/// returned for `bundle`.
pub fn receipt_from_report(
    token: VerilogASourceOperationToken,
    report: &RuntimeCompileReport,
    bundle: Option<&ProjectSourceBundle>,
    cells: Option<&[rspice_veriloga::ExternalCell]>,
    delay_select: rspice_veriloga::canonical_ir::digital::DigitalDelaySelect,
) -> Result<VerilogACompileReceipt, String> {
    let (compilation, diagnostics) =
        rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt::from_report_against(
            token,
            report,
            bundle,
            cells,
            delay_select,
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
        .map(|diagnostic| bind_to_compile(CodeEditorDiagnostic::from(diagnostic), token))
        .collect();
    let diagnostics = Arc::new(CodeDiagnosticCollection::try_new(diagnostics)?);
    Ok(VerilogACompileReceipt {
        bytecode_available: report.targets.is_available(RuntimeTarget::BytecodeVm),
        diagnostics,
        compilation,
        interface: None,
        placement: None,
    })
}

/// The one finding that stands for a compile whose findings exceed what a
/// diagnostic collection holds.
pub fn diagnostic_capacity_failure(
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
