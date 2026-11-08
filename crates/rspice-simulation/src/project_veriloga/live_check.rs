//! The check a Verilog-A source gets while it is being written.
//!
//! It reads the source as its compile does, through the preprocessor, the
//! parser and semantic analysis, and stops there: no code is generated, so a
//! check costs a fraction of a compile and can follow the writer's pauses.

use std::sync::Arc;

use rspice_design::project_sources::ProjectSourceBundle;
use rspice_veriloga::{CompileDiagnosticSeverity, VerilogACompiler};
use rspice_vocabulary::diagnostics::SourceDiagnostic;

use super::ProjectVerilogACompileError;
use super::diagnostics::{ProjectCompileDiagnostic, ProjectDiagnosticSeverity};

/// What reading `bundle` through semantic analysis finds: the errors that
/// stop the reading, or the warnings it reads past. Empty when the source
/// reads clean. `selected_module` is the module its compile builds, whose
/// name the build profile is checked for.
pub fn check_source(
    bundle: &ProjectSourceBundle,
    selected_module: Option<&str>,
) -> Vec<SourceDiagnostic> {
    let findings = match read_source(bundle, selected_module) {
        Ok(warnings) => warnings,
        Err(error) => super::diagnostics::project_compile_outcome(bundle, Err(error))
            .err()
            .unwrap_or_default(),
    };
    findings.into_iter().map(SourceDiagnostic::from).collect()
}

fn read_source(
    bundle: &ProjectSourceBundle,
    selected_module: Option<&str>,
) -> Result<Vec<ProjectCompileDiagnostic>, ProjectVerilogACompileError> {
    let resolved = super::build_profile::resolve_veriloga_build_profile(bundle)
        .map_err(ProjectVerilogACompileError::BuildProfile)?;
    let selected_module = selected_module.or(match resolved.profile.entry_modules.as_slice() {
        [module] => Some(module.as_str()),
        _ => None,
    });
    if let Some(module) = selected_module {
        resolved
            .profile
            .validate_selected_module(module)
            .map_err(ProjectVerilogACompileError::BuildProfile)?;
    }
    let sources =
        super::build_profile::project_bundle_as_virtual_with_profile(bundle, &resolved.profile)
            .map_err(ProjectVerilogACompileError::SourceClosure)?;
    let prepared = VerilogACompiler::new(resolved.profile.compiler_options())
        .prepare_virtual_runtime_source(&sources, super::project_virtual_compile_limits())
        .map_err(|failure| ProjectVerilogACompileError::Virtual(Box::new(failure)))?;
    Ok(prepared
        .diagnostics()
        .iter()
        .map(|diagnostic| ProjectCompileDiagnostic {
            producer: "rspice.veriloga.compiler".to_owned(),
            code: diagnostic.code.clone(),
            severity: match diagnostic.severity {
                CompileDiagnosticSeverity::Error => ProjectDiagnosticSeverity::Error,
                CompileDiagnosticSeverity::Warning => ProjectDiagnosticSeverity::Warning,
            },
            message: Arc::from(diagnostic.message.as_str()),
            detail: Arc::from(""),
            source_path: diagnostic.path.clone(),
            source: None,
            byte_range: diagnostic
                .byte_start
                .zip(diagnostic.byte_end)
                .and_then(|(start, end)| (start <= end).then_some(start..end)),
            line: diagnostic.line,
            column: diagnostic.column,
        })
        .collect())
}

/// Run [`check_source`] on a thread of its own, send what it finds, and
/// call `wake` so the editor shows it.
#[cfg(not(target_arch = "wasm32"))]
pub fn start_check(
    bundle: ProjectSourceBundle,
    selected_module: Option<String>,
    sender: std::sync::mpsc::Sender<Vec<SourceDiagnostic>>,
    wake: impl Fn() + Send + 'static,
) {
    std::thread::spawn(move || {
        let findings = check_source(&bundle, selected_module.as_deref());
        let _ = sender.send(findings);
        wake();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_vocabulary::diagnostics::DiagnosticSeverity;

    const PROBE: &str = "`include \"disciplines.vams\"\n\nmodule probe(p, n);\n  inout p, n;\n  electrical p, n;\n  real g;\n  analog begin\n    g = 2;\n    I(p, n) <+ g * V(p, n);\n  end\nendmodule\n";

    fn bundle(source: &str) -> ProjectSourceBundle {
        use rspice_design::project_sources::{ProjectSourceLanguage, ProjectSourceOwner};
        ProjectSourceBundle::try_new(
            ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
            ProjectSourceLanguage::VerilogA,
            "probe.va",
            source,
            [],
            [],
        )
        .expect("a valid bundle")
    }

    #[test]
    fn a_clean_source_reads_clean_and_an_undeclared_name_is_placed_where_it_was_written() {
        for module in [None, Some("probe")] {
            assert!(
                check_source(&bundle(PROBE), module).is_empty(),
                "{:#?}",
                check_source(&bundle(PROBE), module)
            );
        }
        let broken = PROBE.replace("<+ g *", "<+ gain *");
        let findings = check_source(&bundle(&broken), Some("probe"));
        let [finding] = findings.as_slice() else {
            panic!("one error: {findings:#?}");
        };
        assert_eq!(finding.severity, DiagnosticSeverity::Error);
        assert_eq!(finding.source_path.as_deref(), Some("probe.va"));
        assert_eq!((finding.line, finding.column), (Some(9), Some(16)));
        assert!(finding.message.contains("gain"), "{}", finding.message);
    }

    #[test]
    fn a_syntax_error_is_a_finding_of_its_own() {
        let broken = PROBE.replace("g = 2;", "g = 2");
        let findings = check_source(&bundle(&broken), None);
        assert!(
            findings
                .iter()
                .any(|finding| finding.severity == DiagnosticSeverity::Error
                    && finding.line.is_some()),
            "{findings:#?}"
        );
    }
}
