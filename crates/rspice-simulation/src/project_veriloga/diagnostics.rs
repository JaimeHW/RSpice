//! Compiler findings independent of editor state and transport metadata.

use super::VerilogASourceOperationToken;
use rspice_design::project_sources::ProjectSourceBundle;
use rspice_veriloga::{CompileDiagnosticPhase, CompileDiagnosticSeverity, RuntimeCompileReport};
use rspice_vocabulary::diagnostics::{
    DiagnosticSeverity, SourceDiagnostic, diagnostic_range, stable_diagnostic_id,
};
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

/// The specialist review's findings, each placed where it was written. A
/// finding in text the compiler read without a document name is in the
/// bundle's root document, `root_path`.
pub(super) fn specialist_diagnostics(
    report: &RuntimeCompileReport,
    profile: &super::build_profile::VerilogABuildProfile,
    root_path: &str,
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
            let diagnostic = ProjectCompileDiagnostic::new(
                "rspice.veriloga.specialist",
                &finding.code,
                severity,
                finding.summary.as_str(),
                detail,
            );
            match &finding.location {
                Some(location) => diagnostic.with_source(
                    Some(
                        location
                            .path
                            .clone()
                            .unwrap_or_else(|| root_path.to_owned()),
                    ),
                    None,
                    Some(location.byte_start..location.byte_end),
                    Some(location.line),
                    Some(location.column),
                ),
                None => diagnostic,
            }
        })
        .collect()
}

/// One row per distinct finding.
///
/// Findings with one identity — the same words about the same place — are
/// one finding reported more than once, and the row says how many times. A
/// repeat is bookkeeping, never a reason to refuse a compile.
pub(super) fn merge_repeated(
    diagnostics: Vec<ProjectCompileDiagnostic>,
    token: VerilogASourceOperationToken,
) -> Vec<ProjectCompileDiagnostic> {
    let mut rows: Vec<(ProjectCompileDiagnostic, usize)> = Vec::with_capacity(diagnostics.len());
    let mut rows_by_identity =
        std::collections::HashMap::<_, usize>::with_capacity(diagnostics.len());
    for diagnostic in diagnostics {
        match rows_by_identity.entry(diagnostic.identity(token)) {
            std::collections::hash_map::Entry::Occupied(row) => {
                if cfg!(debug_assertions) {
                    log::warn!(
                        "Verilog-A finding {} reported more than once: {}",
                        diagnostic.code,
                        diagnostic.message
                    );
                }
                rows[*row.get()].1 += 1;
            }
            std::collections::hash_map::Entry::Vacant(row) => {
                row.insert(rows.len());
                rows.push((diagnostic, 1));
            }
        }
    }
    rows.into_iter()
        .map(|(mut diagnostic, count)| {
            if count > 1 {
                diagnostic.detail = format!("{} Reported {count} times.", diagnostic.detail).into();
            }
            diagnostic
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

impl From<ProjectCompileDiagnostic> for SourceDiagnostic {
    fn from(diagnostic: ProjectCompileDiagnostic) -> Self {
        let severity = match diagnostic.severity {
            ProjectDiagnosticSeverity::Info => DiagnosticSeverity::Info,
            ProjectDiagnosticSeverity::Warning => DiagnosticSeverity::Warning,
            ProjectDiagnosticSeverity::Error => DiagnosticSeverity::Error,
        };
        Self::current(
            diagnostic.producer,
            diagnostic.code,
            severity,
            diagnostic.message,
            diagnostic.detail,
            diagnostic.source_path,
            diagnostic.source,
            diagnostic.byte_range,
            diagnostic.line,
            diagnostic.column,
        )
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
            .compile_virtual_runtime_diagnosed_with_qualifications(
                &bundle,
                "selected",
                rspice_veriloga::VirtualCompileLimits::default(),
                rspice_veriloga::RuntimeQualificationOptions::NONE,
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

    /// The probe divides by a branch voltage on line 6, column 31.
    const PROBE: &str = "`include \"disciplines.vams\"\n\nmodule probe(p, n, c);\n  inout p, n, c;\n  electrical p, n, c;\n  analog I(p, n) <+ V(p, n) / V(c);\nendmodule\n";

    fn bundle(path: &str, source: &str) -> ProjectSourceBundle {
        use rspice_design::project_sources::{ProjectSourceLanguage, ProjectSourceOwner};
        ProjectSourceBundle::try_new(
            ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
            ProjectSourceLanguage::VerilogA,
            path,
            source,
            [],
            [],
        )
        .expect("a valid bundle")
    }

    fn compiled(
        bundle: &ProjectSourceBundle,
        module: Option<&str>,
    ) -> (
        super::super::receipt::ProjectCompileReceipt,
        Vec<ProjectCompileDiagnostic>,
    ) {
        super::super::receipt::compile_project_bundle_receipt(
            rspice_vocabulary::product::ProjectId::new(),
            bundle,
            module,
        )
        .unwrap_or_else(|diagnostics| panic!("the module compiles: {diagnostics:#?}"))
    }

    #[test]
    fn a_lint_finding_is_placed_where_it_was_written_on_either_compile_route() {
        let bundle = bundle("probe.va", PROBE);
        for module in [None, Some("probe")] {
            let (_, diagnostics) = compiled(&bundle, module);
            let divisions = diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.code == "VA-CONVERGENCE-DIVISION")
                .collect::<Vec<_>>();
            let [division] = divisions.as_slice() else {
                panic!("one finding for one division: {diagnostics:#?}");
            };
            assert_eq!(division.severity, ProjectDiagnosticSeverity::Info);
            assert_eq!(division.source_path.as_deref(), Some("probe.va"));
            assert_eq!((division.line, division.column), (Some(6), Some(31)));
            let range = division.byte_range.clone().expect("a byte range");
            assert_eq!(&PROBE[range], "V(c)");
        }
    }

    #[test]
    fn repeated_findings_are_one_row_and_never_fail_the_compile() {
        let bundle = bundle("probe.va", PROBE);
        let (receipt, _) = compiled(&bundle, Some("probe"));
        let mut report = (**receipt.report()).clone();
        let mut unplaced = report
            .specialist
            .findings
            .iter()
            .find(|finding| finding.code == "VA-CONVERGENCE-DIVISION")
            .expect("the division finding")
            .clone();
        unplaced.location = None;
        report
            .specialist
            .findings
            .extend([unplaced.clone(), unplaced.clone(), unplaced]);

        let (_, diagnostics) = super::super::receipt::ProjectCompileReceipt::from_report_against(
            receipt.token(),
            &report,
            Some(&bundle),
            None,
            rspice_veriloga::canonical_ir::digital::DigitalDelaySelect::Typ,
        )
        .expect("a repeated finding never fails a compile");
        let divisions = diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "VA-CONVERGENCE-DIVISION")
            .collect::<Vec<_>>();
        assert_eq!(divisions.len(), 2, "the placed finding and one repeated row");
        assert!(
            divisions
                .iter()
                .any(|diagnostic| diagnostic.line.is_none()
                    && diagnostic.detail.ends_with("Reported 3 times.")),
            "{divisions:#?}"
        );
        assert!(
            diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != "DIAGNOSTIC-CAPACITY")
        );
    }
}
