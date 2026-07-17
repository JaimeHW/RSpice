//! In-memory runtime compilation reports and user-facing diagnostics.
//!
//! This module deliberately contains no file-system operations. It describes
//! artifacts produced from source text, their simulator ABI, and the runtime
//! targets that have actually qualified for those artifacts.

use std::collections::BTreeSet;
use std::path::{Component, Path};

use smol_str::SmolStr;
use thiserror::Error;

use crate::canonical_ir::{CanonicalIrArtifact, CanonicalValueType};
use crate::codegen::CompiledModel;
use crate::error::CompileError;
use crate::rust_backend::{
    GeneratedRustDeviceReport, RustBackendSelection, RustTranspileOptions, RustTranspiler,
};
use crate::source::Span;

/// The complete, mutually consistent output of an in-memory runtime compile.
#[derive(Debug, Clone)]
pub struct RuntimeCompileReport {
    /// Bytecode-era model consumed by the simulator and portable interpreter.
    pub model: CompiledModel,
    /// Canonical HIR/MIR/OptIR artifact consumed by qualified backends.
    pub canonical_ir: CanonicalIrArtifact,
    /// Stable public simulator ABI derived from the canonical artifact.
    pub abi: RuntimeAbiSummary,
    /// Qualification result for every target advertised by the workbench.
    pub targets: RuntimeTargetQualifications,
    /// Generated Rust source, present only when in-memory transpilation passed.
    pub generated_rust: Option<GeneratedRustDeviceReport>,
}

impl RuntimeCompileReport {
    pub(crate) fn from_artifacts(model: CompiledModel, canonical_ir: CanonicalIrArtifact) -> Self {
        let abi = RuntimeAbiSummary::from_artifact(&canonical_ir);
        let (targets, generated_rust) = qualify_runtime_targets(&model, &canonical_ir);
        Self {
            model,
            canonical_ir,
            abi,
            targets,
            generated_rust,
        }
    }

    /// Revalidate all cross-artifact identities and generated-source contents.
    ///
    /// A successful compile calls this before publishing the report. Keeping
    /// the method public lets caches and transport boundaries recheck an
    /// artifact after deserialization or mutation in tests.
    pub fn validate_integrity(&self) -> Result<(), RuntimeArtifactIntegrityError> {
        let model_module = self.model.name.as_str();
        let canonical_module = self.canonical_ir.hir.module_name.as_str();
        if model_module != canonical_module {
            return Err(RuntimeArtifactIntegrityError::ModuleMismatch {
                model: model_module.to_owned(),
                canonical: canonical_module.to_owned(),
            });
        }

        let model_digest = self.model.source_digest.as_str();
        let canonical_digest = self.canonical_ir.metadata.source_digest.as_str();
        if model_digest != canonical_digest {
            return Err(RuntimeArtifactIntegrityError::SourceDigestMismatch {
                artifact: "compiled model",
                expected: canonical_digest.to_owned(),
                actual: model_digest.to_owned(),
            });
        }

        if let Err(diagnostics) = self.canonical_ir.validate() {
            return Err(RuntimeArtifactIntegrityError::InvalidCanonicalIr {
                diagnostics: diagnostics
                    .into_iter()
                    .map(|item| item.to_string())
                    .collect(),
            });
        }

        if self.abi.module_name.as_str() != canonical_module {
            return Err(RuntimeArtifactIntegrityError::AbiModuleMismatch {
                abi: self.abi.module_name.to_string(),
                canonical: canonical_module.to_owned(),
            });
        }
        if self.abi != RuntimeAbiSummary::from_artifact(&self.canonical_ir) {
            return Err(RuntimeArtifactIntegrityError::AbiSurfaceMismatch);
        }

        let rust_is_qualified = self.targets.is_available(RuntimeTarget::GeneratedRust);
        if rust_is_qualified != self.generated_rust.is_some() {
            return Err(
                RuntimeArtifactIntegrityError::GeneratedRustQualificationMismatch {
                    qualified: rust_is_qualified,
                    artifact_present: self.generated_rust.is_some(),
                },
            );
        }

        if let Some(generated) = &self.generated_rust {
            validate_generated_rust(generated, canonical_module, canonical_digest)?;
        }

        Ok(())
    }
}

/// Public ABI summary for a compiled behavioral model.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeAbiSummary {
    pub module_name: SmolStr,
    pub analog_ports: Vec<RuntimeAbiPort>,
    pub parameters: Vec<RuntimeAbiParameter>,
    pub noise_source_count: usize,
    pub state_variable_count: usize,
    pub internal_node_count: usize,
    pub branch_unknown_count: usize,
    pub equation_count: usize,
}

impl RuntimeAbiSummary {
    fn from_artifact(artifact: &CanonicalIrArtifact) -> Self {
        Self {
            module_name: artifact.hir.module_name.clone(),
            analog_ports: artifact
                .hir
                .ports
                .iter()
                .map(|port| RuntimeAbiPort {
                    name: port.name.clone(),
                    direction: port.direction.clone(),
                    discipline: port.discipline.clone(),
                    potential_nature: port.nature_potential.clone(),
                    flow_nature: port.nature_flow.clone(),
                })
                .collect(),
            parameters: artifact
                .hir
                .parameters
                .iter()
                .map(|parameter| RuntimeAbiParameter {
                    name: parameter.name.clone(),
                    value_type: parameter.value_type,
                    default: parameter.default,
                    aliases: parameter.aliases.clone(),
                })
                .collect(),
            noise_source_count: artifact.noise_sources.sources.len(),
            state_variable_count: artifact
                .hir
                .variables
                .iter()
                .filter(|variable| variable.is_state)
                .count(),
            internal_node_count: artifact.hir.internal_nodes.len(),
            branch_unknown_count: artifact.mir.branch_unknowns.len(),
            equation_count: artifact.mir.equations.len(),
        }
    }

    /// Number of external analog terminals in declaration order.
    pub fn analog_port_count(&self) -> usize {
        self.analog_ports.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeAbiPort {
    pub name: SmolStr,
    pub direction: SmolStr,
    pub discipline: SmolStr,
    pub potential_nature: Option<SmolStr>,
    pub flow_nature: Option<SmolStr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeAbiParameter {
    pub name: SmolStr,
    pub value_type: CanonicalValueType,
    pub default: Option<f64>,
    pub aliases: Vec<SmolStr>,
}

/// Runtime/compiler targets represented by the behavioral-model workbench.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeTarget {
    SemanticIr,
    BytecodeVm,
    NativeX64Jit,
    WasmInterpreter,
    GeneratedRust,
}

/// Whether a target can consume this exact compiled artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeTargetReadiness {
    Available,
    Unavailable,
    Rejected,
}

/// Product maturity of a target, independent of artifact readiness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeTargetMaturity {
    Production,
    Preview,
    QualificationOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeTargetQualification {
    pub target: RuntimeTarget,
    pub readiness: RuntimeTargetReadiness,
    pub maturity: RuntimeTargetMaturity,
    pub detail: String,
}

impl RuntimeTargetQualification {
    pub fn is_available(&self) -> bool {
        self.readiness == RuntimeTargetReadiness::Available
    }
}

/// Complete target matrix. Every [`RuntimeTarget`] occurs exactly once.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeTargetQualifications {
    entries: Vec<RuntimeTargetQualification>,
}

impl RuntimeTargetQualifications {
    fn new(entries: Vec<RuntimeTargetQualification>) -> Self {
        debug_assert_eq!(entries.len(), 5);
        Self { entries }
    }

    pub fn all(&self) -> &[RuntimeTargetQualification] {
        &self.entries
    }

    pub fn get(&self, target: RuntimeTarget) -> &RuntimeTargetQualification {
        self.entries
            .iter()
            .find(|entry| entry.target == target)
            .expect("runtime target matrix is exhaustive")
    }

    pub fn is_available(&self, target: RuntimeTarget) -> bool {
        self.get(target).is_available()
    }
}

fn qualification(
    target: RuntimeTarget,
    readiness: RuntimeTargetReadiness,
    maturity: RuntimeTargetMaturity,
    detail: impl Into<String>,
) -> RuntimeTargetQualification {
    RuntimeTargetQualification {
        target,
        readiness,
        maturity,
        detail: detail.into(),
    }
}

fn qualify_runtime_targets(
    model: &CompiledModel,
    canonical_ir: &CanonicalIrArtifact,
) -> (
    RuntimeTargetQualifications,
    Option<GeneratedRustDeviceReport>,
) {
    let generated_result =
        RustTranspiler::new(RustTranspileOptions::default()).transpile_with_report(canonical_ir);
    let (generated_qualification, generated_rust) = match generated_result {
        Ok(report) => {
            let backend = rust_backend_name(report.backend);
            (
                qualification(
                    RuntimeTarget::GeneratedRust,
                    RuntimeTargetReadiness::Available,
                    RuntimeTargetMaturity::QualificationOnly,
                    format!("qualified with {backend}"),
                ),
                Some(report),
            )
        }
        Err(error) => (
            qualification(
                RuntimeTarget::GeneratedRust,
                RuntimeTargetReadiness::Rejected,
                RuntimeTargetMaturity::QualificationOnly,
                error.to_string(),
            ),
            None,
        ),
    };

    let entries = vec![
        qualification(
            RuntimeTarget::SemanticIr,
            RuntimeTargetReadiness::Available,
            RuntimeTargetMaturity::Production,
            "canonical HIR/MIR/OptIR validated",
        ),
        qualification(
            RuntimeTarget::BytecodeVm,
            RuntimeTargetReadiness::Available,
            RuntimeTargetMaturity::Production,
            "compiled bytecode model available",
        ),
        qualify_native_x64(model, canonical_ir),
        qualification(
            RuntimeTarget::WasmInterpreter,
            RuntimeTargetReadiness::Available,
            RuntimeTargetMaturity::Production,
            "portable bytecode interpreter available for wasm32 builds",
        ),
        generated_qualification,
    ];

    (RuntimeTargetQualifications::new(entries), generated_rust)
}

fn rust_backend_name(backend: RustBackendSelection) -> &'static str {
    match backend {
        RustBackendSelection::ScalarOptIr => "scalar OptIR backend",
        RustBackendSelection::SparseLocalKernel => "sparse local kernel backend",
        RustBackendSelection::StructuredKernel => "structured kernel backend",
        RustBackendSelection::ScalarHybrid => "scalar hybrid backend",
        RustBackendSelection::LegacyDevice => "legacy device backend",
    }
}

#[cfg(all(feature = "native", target_arch = "x86_64"))]
fn qualify_native_x64(
    model: &CompiledModel,
    canonical_ir: &CanonicalIrArtifact,
) -> RuntimeTargetQualification {
    match crate::native::compile_native_with_canonical_ir(model, canonical_ir) {
        Ok(_) => qualification(
            RuntimeTarget::NativeX64Jit,
            RuntimeTargetReadiness::Available,
            RuntimeTargetMaturity::Preview,
            "native x64 JIT compiled successfully",
        ),
        Err(error) => qualification(
            RuntimeTarget::NativeX64Jit,
            RuntimeTargetReadiness::Rejected,
            RuntimeTargetMaturity::Preview,
            error.to_string(),
        ),
    }
}

#[cfg(all(feature = "native", not(target_arch = "x86_64")))]
fn qualify_native_x64(
    _model: &CompiledModel,
    _canonical_ir: &CanonicalIrArtifact,
) -> RuntimeTargetQualification {
    qualification(
        RuntimeTarget::NativeX64Jit,
        RuntimeTargetReadiness::Unavailable,
        RuntimeTargetMaturity::Preview,
        "native x64 JIT requires an x86-64 host",
    )
}

#[cfg(not(feature = "native"))]
fn qualify_native_x64(
    _model: &CompiledModel,
    _canonical_ir: &CanonicalIrArtifact,
) -> RuntimeTargetQualification {
    qualification(
        RuntimeTarget::NativeX64Jit,
        RuntimeTargetReadiness::Unavailable,
        RuntimeTargetMaturity::Preview,
        "native compiler feature is disabled in this build",
    )
}

fn validate_generated_rust(
    generated: &GeneratedRustDeviceReport,
    canonical_module: &str,
    canonical_digest: &str,
) -> Result<(), RuntimeArtifactIntegrityError> {
    if generated.device.module_name != canonical_module {
        return Err(RuntimeArtifactIntegrityError::GeneratedRustModuleMismatch {
            generated: generated.device.module_name.clone(),
            canonical: canonical_module.to_owned(),
        });
    }
    if generated.device.source_digest != canonical_digest {
        return Err(RuntimeArtifactIntegrityError::SourceDigestMismatch {
            artifact: "generated Rust",
            expected: canonical_digest.to_owned(),
            actual: generated.device.source_digest.clone(),
        });
    }
    if generated.device.files.is_empty() {
        return Err(RuntimeArtifactIntegrityError::GeneratedRustHasNoFiles);
    }

    let mut paths = BTreeSet::new();
    for file in &generated.device.files {
        let path = Path::new(&file.relative_path);
        let safe = !file.relative_path.is_empty()
            && !path.is_absolute()
            && path
                .components()
                .all(|component| matches!(component, Component::Normal(_)));
        if !safe {
            return Err(RuntimeArtifactIntegrityError::UnsafeGeneratedRustPath(
                file.relative_path.clone(),
            ));
        }
        if !paths.insert(file.relative_path.clone()) {
            return Err(RuntimeArtifactIntegrityError::DuplicateGeneratedRustPath(
                file.relative_path.clone(),
            ));
        }
        if file.contents.is_empty() {
            return Err(RuntimeArtifactIntegrityError::EmptyGeneratedRustFile(
                file.relative_path.clone(),
            ));
        }
    }
    Ok(())
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum RuntimeArtifactIntegrityError {
    #[error("compiled model module '{model}' does not match canonical module '{canonical}'")]
    ModuleMismatch { model: String, canonical: String },
    #[error("{artifact} source digest '{actual}' does not match canonical digest '{expected}'")]
    SourceDigestMismatch {
        artifact: &'static str,
        expected: String,
        actual: String,
    },
    #[error("canonical IR validation failed: {}", .diagnostics.join("; "))]
    InvalidCanonicalIr { diagnostics: Vec<String> },
    #[error("ABI module '{abi}' does not match canonical module '{canonical}'")]
    AbiModuleMismatch { abi: String, canonical: String },
    #[error("ABI surface does not match the canonical model")]
    AbiSurfaceMismatch,
    #[error(
        "generated Rust qualification/artifact mismatch: qualified={qualified}, artifact_present={artifact_present}"
    )]
    GeneratedRustQualificationMismatch {
        qualified: bool,
        artifact_present: bool,
    },
    #[error("generated Rust module '{generated}' does not match canonical module '{canonical}'")]
    GeneratedRustModuleMismatch {
        generated: String,
        canonical: String,
    },
    #[error("generated Rust qualification produced no source files")]
    GeneratedRustHasNoFiles,
    #[error("generated Rust path is not a safe relative path: '{0}'")]
    UnsafeGeneratedRustPath(String),
    #[error("generated Rust path occurs more than once: '{0}'")]
    DuplicateGeneratedRustPath(String),
    #[error("generated Rust file is empty: '{0}'")]
    EmptyGeneratedRustFile(String),
}

/// Compiler phase associated with a typed source diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompileDiagnosticPhase {
    Input,
    Lexer,
    Parser,
    Semantic,
    CodeGeneration,
    ModuleSelection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompileDiagnosticSeverity {
    Error,
}

/// One-based source position. `column` counts Unicode scalar values, not bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompileSourcePosition {
    pub line: u32,
    pub column: u32,
}

/// Exact compiler byte span plus display-ready source positions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileDiagnosticSpan {
    pub source_id: u32,
    pub byte_start: u32,
    pub byte_end: u32,
    pub start: Option<CompileSourcePosition>,
    pub end: Option<CompileSourcePosition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileDiagnostic {
    pub severity: CompileDiagnosticSeverity,
    pub phase: CompileDiagnosticPhase,
    pub message: String,
    pub span: Option<CompileDiagnosticSpan>,
}

/// Flatten a compiler error into stable, typed diagnostics for an editor.
///
/// Span offsets are copied verbatim from [`CompileError`]. Line and column
/// values are populated only when those offsets are valid UTF-8 boundaries in
/// `source`; this prevents a preprocessor-expanded span from being presented as
/// a false original-source location.
pub fn compile_diagnostics(source: &str, error: &CompileError) -> Vec<CompileDiagnostic> {
    let mut diagnostics = Vec::new();
    collect_compile_diagnostics(source, error, &mut diagnostics);
    diagnostics
}

fn collect_compile_diagnostics(
    source: &str,
    error: &CompileError,
    diagnostics: &mut Vec<CompileDiagnostic>,
) {
    match error {
        CompileError::Multiple(errors) => {
            for error in errors {
                collect_compile_diagnostics(source, error, diagnostics);
            }
        }
        CompileError::Lexer(error) => diagnostics.push(diagnostic(
            source,
            CompileDiagnosticPhase::Lexer,
            error.to_string(),
            Some(error.span),
        )),
        CompileError::Parser(error) => diagnostics.push(diagnostic(
            source,
            CompileDiagnosticPhase::Parser,
            error.to_string(),
            Some(error.span),
        )),
        CompileError::Semantic(error) => diagnostics.push(diagnostic(
            source,
            CompileDiagnosticPhase::Semantic,
            error.to_string(),
            Some(error.span),
        )),
        CompileError::CodeGen(error) => diagnostics.push(diagnostic(
            source,
            CompileDiagnosticPhase::CodeGeneration,
            error.to_string(),
            error.span,
        )),
        CompileError::ModuleSelection(_) => diagnostics.push(diagnostic(
            source,
            CompileDiagnosticPhase::ModuleSelection,
            error.to_string(),
            None,
        )),
        CompileError::IoError { .. } => diagnostics.push(diagnostic(
            source,
            CompileDiagnosticPhase::Input,
            error.to_string(),
            None,
        )),
    }
}

fn diagnostic(
    source: &str,
    phase: CompileDiagnosticPhase,
    message: String,
    span: Option<Span>,
) -> CompileDiagnostic {
    CompileDiagnostic {
        severity: CompileDiagnosticSeverity::Error,
        phase,
        message,
        span: span.map(|span| CompileDiagnosticSpan {
            source_id: span.source.raw(),
            byte_start: span.start,
            byte_end: span.end,
            start: source_position(source, span.start),
            end: source_position(source, span.end),
        }),
    }
}

fn source_position(source: &str, offset: u32) -> Option<CompileSourcePosition> {
    let offset = usize::try_from(offset).ok()?;
    if offset > source.len() || !source.is_char_boundary(offset) {
        return None;
    }

    let prefix = &source[..offset];
    let line = u32::try_from(prefix.bytes().filter(|byte| *byte == b'\n').count())
        .ok()?
        .checked_add(1)?;
    let line_start = prefix.rfind('\n').map_or(0, |index| index + 1);
    let column = u32::try_from(source[line_start..offset].chars().count())
        .ok()?
        .checked_add(1)?;
    Some(CompileSourcePosition { line, column })
}
