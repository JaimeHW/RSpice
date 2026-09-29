//! Validated project compilation shared by worker hosts, without editor state.

use rspice_design::project_sources::{
    MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES, ProjectSourceBundle, ProjectSourceError,
    ProjectSourceLanguage,
};
use rspice_veriloga::RuntimeCompileReport;

use super::{ProjectVerilogACompileError, build_profile, compile_project_bundle_source};

/// Source admission failures, distinct from diagnosed compiler failures.
#[derive(Debug)]
pub enum WorkerCompileRequestError {
    Language,
    SourceBundle(ProjectSourceError),
    BuildProfile(String),
    ModuleSelection,
}

impl std::fmt::Display for WorkerCompileRequestError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Language => {
                formatter.write_str("Verilog-A worker received a non-Verilog-A source bundle.")
            }
            Self::SourceBundle(error) => {
                write!(
                    formatter,
                    "Verilog-A worker source bundle is invalid: {error}"
                )
            }
            Self::BuildProfile(error) => {
                write!(
                    formatter,
                    "Verilog-A worker build profile is invalid: {error}"
                )
            }
            Self::ModuleSelection => {
                formatter.write_str("Verilog-A worker module selection is invalid.")
            }
        }
    }
}

impl std::error::Error for WorkerCompileRequestError {}

/// A host explicitly selects whether to attempt the optional browser backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerCompileTarget {
    Bytecode,
    #[cfg(feature = "wasm-jit")]
    WasmJit,
}

/// A successful compiler report and the optional secondary backend outcome.
/// A JIT refusal retains the report and its bytecode fallback.
#[derive(Debug)]
pub struct CompiledWorkerSource {
    pub report: Box<RuntimeCompileReport>,
    #[cfg(feature = "wasm-jit")]
    pub wasm_jit: Option<
        Result<
            rspice_veriloga::wasm_jit::WasmJitModelArtifact,
            rspice_veriloga::wasm_jit::WasmJitError,
        >,
    >,
}

/// Borrows one validated source closure and its exact module selection.
/// Transport identity and publication currentness remain host responsibilities.
#[derive(Debug)]
pub struct WorkerCompileRequest<'a> {
    bundle: &'a ProjectSourceBundle,
    selected_module: Option<&'a str>,
}

impl<'a> WorkerCompileRequest<'a> {
    pub fn try_new(
        bundle: &'a ProjectSourceBundle,
        selected_module: Option<&'a str>,
    ) -> Result<Self, WorkerCompileRequestError> {
        if bundle.language() != ProjectSourceLanguage::VerilogA {
            return Err(WorkerCompileRequestError::Language);
        }
        bundle
            .validate()
            .map_err(WorkerCompileRequestError::SourceBundle)?;
        build_profile::resolve_veriloga_build_profile(bundle)
            .map_err(WorkerCompileRequestError::BuildProfile)?;
        if selected_module.is_some_and(|module| {
            module.is_empty()
                || module.len() > MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES
                || module.chars().any(char::is_control)
        }) {
            return Err(WorkerCompileRequestError::ModuleSelection);
        }
        Ok(Self {
            bundle,
            selected_module,
        })
    }

    pub fn compile(
        self,
        target: WorkerCompileTarget,
    ) -> Result<CompiledWorkerSource, ProjectVerilogACompileError> {
        let report = compile_project_bundle_source(self.bundle, self.selected_module)?;
        Ok(match target {
            WorkerCompileTarget::Bytecode => CompiledWorkerSource {
                report,
                #[cfg(feature = "wasm-jit")]
                wasm_jit: None,
            },
            #[cfg(feature = "wasm-jit")]
            WorkerCompileTarget::WasmJit => {
                let wasm_jit = rspice_veriloga::wasm_jit::compile_model_value_module(
                    &report.model,
                    &report.canonical_ir,
                );
                CompiledWorkerSource {
                    report,
                    wasm_jit: Some(wasm_jit),
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::project_sources::ProjectSourceOwner;

    const SOURCE: &str =
        "module worker(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n";

    fn bundle(language: ProjectSourceLanguage, source: &str) -> ProjectSourceBundle {
        ProjectSourceBundle::try_new(
            ProjectSourceOwner::code_workspace(language),
            language,
            format!("worker{}", language.required_extension()),
            source,
            [],
            [],
        )
        .unwrap()
    }

    #[test]
    fn source_admission_precedes_compilation_and_bytecode_needs_no_jit() {
        let foreign = bundle(ProjectSourceLanguage::RSpiceAutomation, "workflow: test");
        assert!(matches!(
            WorkerCompileRequest::try_new(&foreign, Some("worker")),
            Err(WorkerCompileRequestError::Language)
        ));
        let source = bundle(ProjectSourceLanguage::VerilogA, SOURCE);
        for module in [
            "",
            "worker\n",
            &"x".repeat(MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES + 1),
        ] {
            assert!(matches!(
                WorkerCompileRequest::try_new(&source, Some(module)),
                Err(WorkerCompileRequestError::ModuleSelection)
            ));
        }
        let compiled = WorkerCompileRequest::try_new(&source, Some("worker"))
            .unwrap()
            .compile(WorkerCompileTarget::Bytecode)
            .unwrap();
        assert_eq!(compiled.report.abi.module_name.as_str(), "worker");
        compiled.report.validate_integrity().unwrap();
        #[cfg(feature = "wasm-jit")]
        assert!(compiled.wasm_jit.is_none());
    }

    #[cfg(feature = "wasm-jit")]
    #[test]
    fn jit_compilation_retains_the_report_and_source_failures_stay_diagnosed() {
        let source = bundle(ProjectSourceLanguage::VerilogA, SOURCE);
        let compiled = WorkerCompileRequest::try_new(&source, Some("worker"))
            .unwrap()
            .compile(WorkerCompileTarget::WasmJit)
            .unwrap();
        compiled.report.validate_integrity().unwrap();
        let artifact = compiled.wasm_jit.unwrap().unwrap();
        assert!(!artifact.module().bytes().is_empty());

        let invalid = bundle(
            ProjectSourceLanguage::VerilogA,
            "module worker(p, n); inout p, n; electrical p, n; analog I(p,n) <+ @; endmodule\n",
        );
        let failure = WorkerCompileRequest::try_new(&invalid, Some("worker"))
            .unwrap()
            .compile(WorkerCompileTarget::WasmJit)
            .unwrap_err();
        let ProjectVerilogACompileError::Virtual(failure) = failure else {
            panic!("selected-module compilation must retain virtual source diagnostics");
        };
        assert!(failure.diagnostics.iter().any(|diagnostic| {
            diagnostic.logical_path.as_deref() == Some("worker.va")
                && diagnostic.byte_start.is_some()
                && diagnostic.byte_end.is_some()
        }));
    }
}
