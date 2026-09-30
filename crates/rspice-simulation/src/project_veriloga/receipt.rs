//! Project-bound compilation receipts used by preparation and application projections.

use super::diagnostics::{
    ProjectCompileDiagnostic, ProjectDiagnosticSeverity, compiler_diagnostics,
    project_compile_outcome, specialist_diagnostics,
};
use super::{
    VerilogASourceOperationToken, build_profile, compile_project_bundle_source,
    prepare_project_runtime,
};
use crate::veriloga::{PreparedRuntimeError, PreparedVerilogARuntime};
use rspice_app_types::diagnostics::{insert_diagnostic_id, validate_diagnostic_count};
use rspice_app_types::product::ProjectId;
use rspice_design::project_sources::ProjectSourceBundle;
use rspice_veriloga::RuntimeCompileReport;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ProjectCompileReceipt {
    token: VerilogASourceOperationToken,
    report: Arc<RuntimeCompileReport>,
}

impl ProjectCompileReceipt {
    pub fn from_report(
        token: VerilogASourceOperationToken,
        report: &RuntimeCompileReport,
        bundle: Option<&ProjectSourceBundle>,
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
        )
    }
}

pub fn compile_project_bundle_receipt(
    project_id: ProjectId,
    bundle: &ProjectSourceBundle,
    selected_module: Option<&str>,
) -> Result<(ProjectCompileReceipt, Vec<ProjectCompileDiagnostic>), Vec<ProjectCompileDiagnostic>> {
    let token = VerilogASourceOperationToken::capture(project_id, bundle, selected_module);
    match project_compile_outcome(
        bundle,
        compile_project_bundle_source(bundle, selected_module),
    ) {
        Ok(report) => {
            ProjectCompileReceipt::from_report(token, &report, Some(bundle)).map_err(|detail| {
                vec![ProjectCompileDiagnostic::new(
                    "rspice.diagnostics",
                    "DIAGNOSTIC-CAPACITY",
                    ProjectDiagnosticSeverity::Error,
                    "Diagnostic collection exceeded the supported maximum",
                    detail,
                )]
            })
        }
        Err(diagnostics) => Err(diagnostics),
    }
}
