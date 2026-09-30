//! Compiler findings independent of editor state and transport metadata.

use super::VerilogASourceOperationToken;
use rspice_app_types::diagnostics::{diagnostic_range, stable_diagnostic_id};
use rspice_design::project_sources::ProjectSourceBundle;
use rspice_veriloga::{CompileDiagnosticPhase, CompileDiagnosticSeverity, RuntimeCompileReport};
use std::sync::Arc;

pub type CompileResult = Result<Box<RuntimeCompileReport>, Vec<ProjectCompileDiagnostic>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectDiagnosticSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct ProjectCompileDiagnostic {
    pub producer: String,
    pub code: String,
    pub severity: ProjectDiagnosticSeverity,
    pub message: Arc<str>,
    pub detail: Arc<str>,
    pub source_path: Option<String>,
    pub source: Option<String>,
    pub byte_range: Option<std::ops::Range<usize>>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

impl ProjectCompileDiagnostic {
    pub(super) fn new(
        producer: impl Into<String>,
        code: impl Into<String>,
        severity: ProjectDiagnosticSeverity,
        message: impl Into<Arc<str>>,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            producer: producer.into(),
            code: code.into(),
            severity,
            message: message.into(),
            detail: detail.into(),
            source_path: None,
            source: None,
            byte_range: None,
            line: None,
            column: None,
        }
    }

    fn with_source(
        mut self,
        source_path: Option<String>,
        source: Option<String>,
        byte_range: Option<std::ops::Range<usize>>,
        line: Option<usize>,
        column: Option<usize>,
    ) -> Self {
        self.source_path = source_path;
        self.source = source;
        self.byte_range = byte_range;
        self.line = line;
        self.column = column;
        self
    }

    pub(super) fn identity(&self, token: VerilogASourceOperationToken) -> uuid::Uuid {
        let document_id = self
            .source_path
            .clone()
            .unwrap_or_else(|| token.bundle_id.to_string());
        stable_diagnostic_id(
            &self.producer,
            &self.code,
            &document_id,
            diagnostic_range(self.byte_range.as_ref(), self.line, self.column),
            token.revision,
            &token.closure_digest.to_string(),
            &self.message,
        )
    }
}

pub fn project_compile_outcome(
    bundle: &ProjectSourceBundle,
    outcome: Result<Box<RuntimeCompileReport>, super::ProjectVerilogACompileError>,
) -> CompileResult {
    use super::ProjectVerilogACompileError;

    match outcome {
        Ok(report) => Ok(report),
        Err(ProjectVerilogACompileError::BuildProfile(error)) => build_profile_error_outcome(error),
        Err(ProjectVerilogACompileError::SourceClosure(error)) => {
            Err(vec![ProjectCompileDiagnostic::new(
                "rspice.veriloga.bundle",
                "VA-SOURCE-CLOSURE",
                ProjectDiagnosticSeverity::Error,
                error,
                "sealed project source closure",
            )])
        }
        Err(ProjectVerilogACompileError::RootModuleRequired) => {
            Err(vec![ProjectCompileDiagnostic::new(
                "rspice.veriloga.bundle",
                "VA-ROOT-MODULE-REQUIRED",
                ProjectDiagnosticSeverity::Error,
                "Select the root module before compiling this multi-file Verilog-A bundle.",
                "Enter the exact module identifier in the Model project navigator.",
            )])
        }
        Err(ProjectVerilogACompileError::Compile(error)) => {
            compile_error_outcome(bundle.root().content(), &error)
        }
        Err(ProjectVerilogACompileError::Virtual(failure)) => {
            virtual_compile_error_outcome(*failure)
        }
    }
}

fn build_profile_error_outcome(error: String) -> CompileResult {
    Err(vec![ProjectCompileDiagnostic::new(
        "rspice.veriloga.build-profile",
        "VA-BUILD-PROFILE",
        ProjectDiagnosticSeverity::Error,
        "Verilog-A build profile is invalid",
        error,
    )])
}

fn compile_error_outcome(source: &str, error: &rspice_veriloga::CompileError) -> CompileResult {
    let diagnostics = rspice_veriloga::compile_diagnostics(source, error)
        .into_iter()
        .map(|diagnostic| {
            let byte_range = diagnostic.span.as_ref().and_then(|span| {
                let start = usize::try_from(span.byte_start).ok()?;
                let end = usize::try_from(span.byte_end).ok()?;
                (start <= end && end <= source.len()).then_some(start..end)
            });
            let position = diagnostic.span.as_ref().and_then(|span| span.start);
            ProjectCompileDiagnostic::new(
                "rspice.veriloga.compiler",
                diagnostic.code,
                editor_severity(diagnostic.severity),
                diagnostic.message,
                diagnostic_phase_label(diagnostic.phase),
            )
            .with_source(
                None,
                None,
                byte_range,
                position.and_then(|position| usize::try_from(position.line).ok()),
                position.and_then(|position| usize::try_from(position.column).ok()),
            )
        })
        .collect();
    Err(diagnostics)
}

fn virtual_compile_error_outcome(
    failure: rspice_veriloga::VirtualRuntimeCompileFailure,
) -> CompileResult {
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
            ProjectCompileDiagnostic::new(
                "rspice.veriloga.compiler",
                diagnostic.code,
                ProjectDiagnosticSeverity::Error,
                diagnostic.message,
                source_label,
            )
            .with_source(
                diagnostic.logical_path,
                diagnostic.source,
                byte_range,
                diagnostic.line,
                diagnostic.column,
            )
        })
        .collect();
    Err(diagnostics)
}

pub(super) fn compiler_diagnostics(report: &RuntimeCompileReport) -> Vec<ProjectCompileDiagnostic> {
    report
        .diagnostics
        .iter()
        .map(|diagnostic| {
            ProjectCompileDiagnostic::new(
                "rspice.veriloga.compiler",
                diagnostic.code.as_str(),
                editor_severity(diagnostic.severity),
                diagnostic.message.as_str(),
                diagnostic_phase_label(diagnostic.phase),
            )
        })
        .collect()
}

const fn editor_severity(severity: CompileDiagnosticSeverity) -> ProjectDiagnosticSeverity {
    match severity {
        CompileDiagnosticSeverity::Error => ProjectDiagnosticSeverity::Error,
        CompileDiagnosticSeverity::Warning => ProjectDiagnosticSeverity::Warning,
    }
}

pub(super) fn specialist_diagnostics(
    report: &RuntimeCompileReport,
    profile: &super::build_profile::VerilogABuildProfile,
) -> Vec<ProjectCompileDiagnostic> {
    report
        .specialist
        .findings
        .iter()
        .filter(|finding| specialist_check_enabled(profile, finding.check))
        .map(|finding| {
            let severity = match finding.severity {
                rspice_veriloga::SpecialistFindingSeverity::Information => {
                    ProjectDiagnosticSeverity::Info
                }
                rspice_veriloga::SpecialistFindingSeverity::Warning => {
                    ProjectDiagnosticSeverity::Warning
                }
                rspice_veriloga::SpecialistFindingSeverity::Error => {
                    ProjectDiagnosticSeverity::Error
                }
            };
            let mut detail = finding.detail.clone();
            if let Some(action) = &finding.action {
                detail.push_str(" Suggested review: ");
                detail.push_str(&action.title);
                detail.push_str(" — ");
                detail.push_str(&action.replacement_hint);
            }
            ProjectCompileDiagnostic::new(
                "rspice.veriloga.specialist",
                &finding.code,
                severity,
                finding.summary.as_str(),
                detail,
            )
        })
        .collect()
}

const fn specialist_check_enabled(
    profile: &super::build_profile::VerilogABuildProfile,
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
    use rspice_veriloga::{VerilogACompiler, VirtualSourceBundle};
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

        let Err(diagnostics) = virtual_compile_error_outcome(failure) else {
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
}
