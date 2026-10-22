//! Project-bound compilation receipts used by preparation and application projections.

use super::diagnostics::{
    ProjectCompileDiagnostic, ProjectDiagnosticSeverity, compiler_diagnostics,
    project_compile_outcome, specialist_diagnostics,
};
use super::{
    VerilogASourceOperationToken, build_profile, compile_project_bundle_source_against,
    prepare_project_runtime,
};
use crate::veriloga::{PreparedRuntimeError, PreparedVerilogARuntime};
use rspice_design::project_sources::ProjectSourceBundle;
use rspice_veriloga::{ExternalCell, RuntimeCompileReport};
use rspice_vocabulary::diagnostics::{insert_diagnostic_id, validate_diagnostic_count};
use rspice_vocabulary::product::ProjectId;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ProjectCompileReceipt {
    token: VerilogASourceOperationToken,
    report: Arc<RuntimeCompileReport>,
    /// The cells the report was compiled against, if any.
    cells: Option<Arc<[ExternalCell]>>,
}

impl ProjectCompileReceipt {
    pub fn from_report(
        token: VerilogASourceOperationToken,
        report: &RuntimeCompileReport,
        bundle: Option<&ProjectSourceBundle>,
    ) -> Result<(Self, Vec<ProjectCompileDiagnostic>), String> {
        Self::from_report_against(token, report, bundle, None)
    }

    /// [`Self::from_report`] for a report compiled against `cells`.
    pub fn from_report_against(
        token: VerilogASourceOperationToken,
        report: &RuntimeCompileReport,
        bundle: Option<&ProjectSourceBundle>,
        cells: Option<&[ExternalCell]>,
    ) -> Result<(Self, Vec<ProjectCompileDiagnostic>), String> {
        let mut diagnostics = compiler_diagnostics(report);
        diagnostics.extend(
            bundle
                .and_then(|bundle| {
                    build_profile::resolve_veriloga_build_profile(bundle)
                        .ok()
                        .map(|resolved| specialist_diagnostics(report, &resolved.profile))
                })
                .unwrap_or_default(),
        );
        validate_diagnostic_count(diagnostics.len())?;
        let mut ids = HashSet::new();
        for diagnostic in &diagnostics {
            insert_diagnostic_id(&mut ids, diagnostic.identity(token))?;
        }
        Ok((
            Self {
                token,
                report: Arc::new(report.clone()),
                cells: cells.map(Arc::from),
            },
            diagnostics,
        ))
    }

    pub const fn token(&self) -> VerilogASourceOperationToken {
        self.token
    }
    pub fn report(&self) -> &Arc<RuntimeCompileReport> {
        &self.report
    }

    /// Whether a deck offering `cells` builds this report's design as it
    /// was compiled: always for a design with no leaf, else only for the
    /// cells it was compiled against.
    pub fn builds_for(&self, cells: &[ExternalCell]) -> bool {
        self.report.canonical_ir.hir.external_leaves.is_empty()
            || self.cells.as_deref() == Some(cells)
    }

    pub fn prepare_runtime(
        &self,
        project_id: ProjectId,
        bundle: &ProjectSourceBundle,
    ) -> Result<PreparedVerilogARuntime, PreparedRuntimeError> {
        self.prepare_runtime_with_alias(project_id, bundle, self.report.abi.module_name.to_string())
    }

    pub fn prepare_runtime_with_alias(
        &self,
        project_id: ProjectId,
        bundle: &ProjectSourceBundle,
        netlist_alias: impl Into<String>,
    ) -> Result<PreparedVerilogARuntime, PreparedRuntimeError> {
        prepare_project_runtime(
            project_id,
            bundle,
            &self.token,
            self.report.abi.module_name.as_str(),
            &self.report,
            netlist_alias,
            self.cells.as_deref(),
        )
    }
}

pub fn compile_project_bundle_receipt(
    project_id: ProjectId,
    bundle: &ProjectSourceBundle,
    selected_module: Option<&str>,
) -> Result<(ProjectCompileReceipt, Vec<ProjectCompileDiagnostic>), Vec<ProjectCompileDiagnostic>> {
    compile_project_bundle_receipt_against(project_id, bundle, selected_module, None)
}

/// [`compile_project_bundle_receipt`] offering the design `cells` as leaves.
pub fn compile_project_bundle_receipt_against(
    project_id: ProjectId,
    bundle: &ProjectSourceBundle,
    selected_module: Option<&str>,
    cells: Option<&[ExternalCell]>,
) -> Result<(ProjectCompileReceipt, Vec<ProjectCompileDiagnostic>), Vec<ProjectCompileDiagnostic>> {
    let token = VerilogASourceOperationToken::capture(project_id, bundle, selected_module);
    match project_compile_outcome(
        bundle,
        compile_project_bundle_source_against(bundle, selected_module, cells),
    ) {
        Ok(report) => {
            ProjectCompileReceipt::from_report_against(token, &report, Some(bundle), cells).map_err(
                |detail| {
                    vec![ProjectCompileDiagnostic::new(
                        "rspice.diagnostics",
                        "DIAGNOSTIC-CAPACITY",
                        ProjectDiagnosticSeverity::Error,
                        "Diagnostic collection exceeded the supported maximum",
                        detail,
                    )]
                },
            )
        }
        Err(diagnostics) => Err(diagnostics),
    }
}
