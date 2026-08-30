//! In-memory runtime compilation reports and user-facing diagnostics.
//!
//! This module deliberately contains no file-system operations. It describes
//! artifacts produced from source text, their simulator ABI, and the runtime
//! targets that have actually qualified for those artifacts.

use std::collections::BTreeSet;
use std::path::{Component, Path};

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use thiserror::Error;

use crate::canonical_ir::{CanonicalIrArtifact, CanonicalValueType};
use crate::codegen::CompiledModel;
use crate::error::CompileError;
use crate::metrics::PipelineMetrics;
use crate::rust_backend::{GeneratedRustDevice, RustTranspileOptions, RustTranspiler};
use crate::source::Span;

/// The complete, mutually consistent output of an in-memory runtime compile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeCompileReport {
    /// Bytecode-era model consumed by the simulator and portable interpreter.
    pub model: CompiledModel,
    /// Canonical HIR/MIR artifact consumed by qualified backends.
    pub canonical_ir: CanonicalIrArtifact,
    /// Stable public simulator ABI derived from the canonical artifact.
    pub abi: RuntimeAbiSummary,
    /// Qualification result for every target advertised by the workbench.
    pub targets: RuntimeTargetQualifications,
    /// Generated Rust source, present only when in-memory transpilation passed.
    pub generated_rust: Option<GeneratedRustDevice>,
    /// Source-authentic specialist review retained with this exact artifact.
    #[serde(default)]
    pub specialist: crate::specialist::SpecialistReport,
    /// Non-fatal compiler diagnostics raised while producing this artifact.
    ///
    /// This is the success path's diagnostic channel: a compile that succeeds
    /// still has things to tell its author, such as a system task the analyzer
    /// parsed and then discarded. A failed compile publishes nothing here —
    /// its errors arrive as [`CompileError`] and are flattened by
    /// [`compile_diagnostics`] — so this vector never holds
    /// [`CompileDiagnosticSeverity::Error`].
    #[serde(default)]
    pub diagnostics: Vec<CompileDiagnostic>,
    /// Operational timings and work-size counters. These are not artifact
    /// identity and are excluded from runtime-contract digests.
    #[serde(default)]
    pub metrics: PipelineMetrics,
}

/// Expensive backend qualifications to perform while constructing a runtime
/// compile report.
///
/// Runtime compilation always produces the semantic and bytecode artifacts
/// needed by the portable simulator. Generated Rust and native machine code
/// are separate products: constructing either can dominate compilation for a
/// large compact model, so callers must request them explicitly.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeQualificationOptions {
    /// Transpile the canonical CFG to generated Rust and retain the source.
    pub generated_rust: bool,
    /// Compile the host-native qualification artifact when supported.
    ///
    /// The field name is retained for serialized and source compatibility with
    /// the original x86-64-only backend. It now selects either x86-64 or
    /// AArch64 machine code according to the host target.
    pub native_x64_jit: bool,
    /// Compile and verify a standard-WebAssembly model artifact.
    ///
    /// Browser instantiation is a separate worker capability check; this flag
    /// qualifies the deterministic compiler product for the exact model.
    #[serde(default)]
    pub wasm_jit: bool,
    /// Whether failure of a requested optimized backend may fall back to the
    /// portable bytecode interpreter.
    #[serde(default)]
    pub interpreter_fallback: InterpreterFallbackPolicy,
}

/// Policy for a requested optimized backend that cannot accept the model.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterpreterFallbackPolicy {
    /// Preserve the portable runtime and report backend readiness in the
    /// qualification matrix. Appropriate for editors and exploratory tools.
    #[default]
    Allow,
    /// Fail compilation when any explicitly requested optimized backend is
    /// unavailable or rejects the model. Appropriate for release pipelines
    /// and performance-qualified simulation products.
    Reject,
}

impl RuntimeQualificationOptions {
    /// No expensive qualification products. This is the runtime/editor
    /// default.
    pub const NONE: Self = Self {
        generated_rust: false,
        native_x64_jit: false,
        wasm_jit: false,
        interpreter_fallback: InterpreterFallbackPolicy::Allow,
    };

    /// Qualify every optional backend available in this build.
    pub const ALL: Self = Self {
        generated_rust: true,
        native_x64_jit: true,
        wasm_jit: true,
        interpreter_fallback: InterpreterFallbackPolicy::Allow,
    };

    /// Require portable generated Rust; never silently accept the interpreter.
    pub const GENERATED_RUST_REQUIRED: Self = Self {
        generated_rust: true,
        native_x64_jit: false,
        wasm_jit: false,
        interpreter_fallback: InterpreterFallbackPolicy::Reject,
    };

    /// Require the native x86-64 JIT; never silently accept the interpreter.
    pub const NATIVE_X64_REQUIRED: Self = Self {
        generated_rust: false,
        native_x64_jit: true,
        wasm_jit: false,
        interpreter_fallback: InterpreterFallbackPolicy::Reject,
    };

    /// Require the host-native JIT; never silently accept the interpreter.
    ///
    /// This architecture-neutral name is preferred for new callers.
    pub const NATIVE_REQUIRED: Self = Self::NATIVE_X64_REQUIRED;

    /// Require a verified browser WebAssembly JIT artifact; never silently
    /// accept the portable interpreter.
    pub const WASM_JIT_REQUIRED: Self = Self {
        generated_rust: false,
        native_x64_jit: false,
        wasm_jit: true,
        interpreter_fallback: InterpreterFallbackPolicy::Reject,
    };

    /// Require every requested backend to qualify.
    pub const fn rejecting_interpreter_fallback(mut self) -> Self {
        self.interpreter_fallback = InterpreterFallbackPolicy::Reject;
        self
    }
}

impl RuntimeCompileReport {
    pub(crate) fn from_artifacts(
        model: CompiledModel,
        canonical_ir: CanonicalIrArtifact,
        qualifications: RuntimeQualificationOptions,
    ) -> Self {
        let abi = RuntimeAbiSummary::from_artifact(&canonical_ir);
        let (targets, generated_rust) =
            qualify_runtime_targets(&model, &canonical_ir, qualifications);
        Self {
            model,
            canonical_ir,
            abi,
            targets,
            generated_rust,
            specialist: crate::specialist::SpecialistReport::default(),
            diagnostics: Vec::new(),
            metrics: PipelineMetrics::default(),
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

        for (index, filter) in self.model.zi_filters.iter().enumerate() {
            filter.validate_integrity().map_err(|error| {
                RuntimeArtifactIntegrityError::InvalidZiFilterState {
                    index,
                    detail: error.to_string(),
                }
            })?;
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

        if !self.targets.is_exhaustive() {
            return Err(RuntimeArtifactIntegrityError::InvalidTargetMatrix);
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

        self.specialist
            .validate(canonical_module)
            .map_err(RuntimeArtifactIntegrityError::InvalidSpecialistReport)?;

        for diagnostic in &self.diagnostics {
            if diagnostic.severity == CompileDiagnosticSeverity::Error {
                return Err(RuntimeArtifactIntegrityError::InvalidCompileDiagnostic(
                    format!(
                        "a published report cannot carry the error '{}'",
                        diagnostic.code
                    ),
                ));
            }
            if diagnostic.code.trim().is_empty() || diagnostic.message.trim().is_empty() {
                return Err(RuntimeArtifactIntegrityError::InvalidCompileDiagnostic(
                    "a compile diagnostic requires both a code and a message".to_owned(),
                ));
            }
        }

        Ok(())
    }

    /// Enforce the caller's optimized-backend fallback policy.
    ///
    /// Qualification remains report-only by default. Release tooling can use a
    /// `*_REQUIRED` option or [`RuntimeQualificationOptions::rejecting_interpreter_fallback`]
    /// to make a rejected or unavailable requested backend a typed error.
    pub fn enforce_fallback_policy(
        &self,
        options: RuntimeQualificationOptions,
    ) -> Result<(), BackendQualificationError> {
        if options.interpreter_fallback == InterpreterFallbackPolicy::Allow {
            return Ok(());
        }
        for (requested, target) in [
            (options.generated_rust, RuntimeTarget::GeneratedRust),
            (options.native_x64_jit, RuntimeTarget::NativeX64Jit),
            (options.wasm_jit, RuntimeTarget::WasmJit),
        ] {
            let qualification = self.targets.get(target);
            if requested && !qualification.is_available() {
                return Err(BackendQualificationError {
                    target,
                    readiness: qualification.readiness,
                    detail: qualification.detail.clone(),
                });
            }
        }
        Ok(())
    }
}

/// Public ABI summary for a compiled behavioral model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
                .filter(|parameter| parameter.is_public)
                .map(|parameter| RuntimeAbiParameter {
                    name: parameter.name.clone(),
                    scope: parameter.scope,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAbiPort {
    pub name: SmolStr,
    pub direction: SmolStr,
    pub discipline: SmolStr,
    pub potential_nature: Option<SmolStr>,
    pub flow_nature: Option<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeAbiParameter {
    pub name: SmolStr,
    pub scope: crate::semantic::ParameterScope,
    pub value_type: CanonicalValueType,
    pub default: Option<f64>,
    pub aliases: Vec<SmolStr>,
}

/// Runtime/compiler targets represented by the behavioral-model workbench.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuntimeTarget {
    SemanticIr,
    BytecodeVm,
    NativeX64Jit,
    WasmJit,
    WasmInterpreter,
    GeneratedRust,
}

impl RuntimeTarget {
    /// The JIT for the current native host architecture.
    ///
    /// The enum variant retains its original spelling for API and serialized
    /// artifact compatibility. New code should use this neutral alias.
    #[allow(non_upper_case_globals)]
    pub const NativeJit: Self = Self::NativeX64Jit;

    pub const fn label(self) -> &'static str {
        match self {
            Self::SemanticIr => "semantic IR",
            Self::BytecodeVm => "bytecode VM",
            Self::NativeX64Jit => "host-native JIT",
            Self::WasmJit => "browser WebAssembly JIT",
            Self::WasmInterpreter => "WebAssembly interpreter",
            Self::GeneratedRust => "generated Rust",
        }
    }
}

/// Whether a target can consume this exact compiled artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeTargetReadiness {
    Available,
    Unavailable,
    Rejected,
}

impl RuntimeTargetReadiness {
    const fn label(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Unavailable => "unavailable",
            Self::Rejected => "rejected",
        }
    }
}

/// Product maturity of a target, independent of artifact readiness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeTargetMaturity {
    Production,
    Preview,
    QualificationOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeTargetQualifications {
    entries: Vec<RuntimeTargetQualification>,
}

impl RuntimeTargetQualifications {
    fn new(entries: Vec<RuntimeTargetQualification>) -> Self {
        debug_assert_eq!(entries.len(), 6);
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

    fn is_exhaustive(&self) -> bool {
        const TARGETS: [RuntimeTarget; 6] = [
            RuntimeTarget::SemanticIr,
            RuntimeTarget::BytecodeVm,
            RuntimeTarget::NativeX64Jit,
            RuntimeTarget::WasmJit,
            RuntimeTarget::WasmInterpreter,
            RuntimeTarget::GeneratedRust,
        ];
        self.entries.len() == TARGETS.len()
            && TARGETS.iter().all(|target| {
                self.entries
                    .iter()
                    .filter(|entry| entry.target == *target)
                    .count()
                    == 1
            })
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
    options: RuntimeQualificationOptions,
) -> (RuntimeTargetQualifications, Option<GeneratedRustDevice>) {
    let (generated_qualification, generated_rust) = if options.generated_rust {
        match RustTranspiler::new(RustTranspileOptions::default()).transpile(canonical_ir) {
            Ok(device) => (
                qualification(
                    RuntimeTarget::GeneratedRust,
                    RuntimeTargetReadiness::Available,
                    RuntimeTargetMaturity::QualificationOnly,
                    "qualified with the canonical CFG backend",
                ),
                Some(device),
            ),
            Err(error) => (
                qualification(
                    RuntimeTarget::GeneratedRust,
                    RuntimeTargetReadiness::Rejected,
                    RuntimeTargetMaturity::QualificationOnly,
                    error.to_string(),
                ),
                None,
            ),
        }
    } else {
        (
            qualification(
                RuntimeTarget::GeneratedRust,
                RuntimeTargetReadiness::Unavailable,
                RuntimeTargetMaturity::QualificationOnly,
                "generated Rust qualification was not requested",
            ),
            None,
        )
    };

    let entries = vec![
        qualification(
            RuntimeTarget::SemanticIr,
            RuntimeTargetReadiness::Available,
            RuntimeTargetMaturity::Production,
            "canonical HIR/MIR validated",
        ),
        qualification(
            RuntimeTarget::BytecodeVm,
            RuntimeTargetReadiness::Available,
            RuntimeTargetMaturity::Production,
            "compiled bytecode model available",
        ),
        qualify_native_if_requested(model, canonical_ir, options.native_x64_jit),
        qualify_wasm_jit_if_requested(model, canonical_ir, options.wasm_jit),
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

fn qualify_wasm_jit_if_requested(
    model: &CompiledModel,
    canonical_ir: &CanonicalIrArtifact,
    requested: bool,
) -> RuntimeTargetQualification {
    if requested {
        qualify_wasm_jit(model, canonical_ir)
    } else {
        qualification(
            RuntimeTarget::WasmJit,
            RuntimeTargetReadiness::Unavailable,
            RuntimeTargetMaturity::QualificationOnly,
            "browser WebAssembly JIT qualification was not requested",
        )
    }
}

#[cfg(feature = "wasm-jit")]
fn qualify_wasm_jit(
    model: &CompiledModel,
    canonical_ir: &CanonicalIrArtifact,
) -> RuntimeTargetQualification {
    match (
        crate::wasm_jit::emit_architecture_probe(),
        crate::wasm_jit::compile_model_value_module(model, canonical_ir),
    ) {
        (Ok(probe), Ok(model_artifact)) => qualification(
            RuntimeTarget::WasmJit,
            RuntimeTargetReadiness::Rejected,
            RuntimeTargetMaturity::QualificationOnly,
            format!(
                "secondary-module architecture qualified with {} probe bytes and {} deterministic scalar entries plus assignment kernels in a {}-byte verified model module; pure and stateful helpers, owned re-entrant sessions, and the fail-closed solver dispatch bridge are installed, with browser startup solver/stamp qualification required before use; broader differential, performance, and release gates remain pending",
                probe.bytes().len(),
                model_artifact.entries().len(),
                model_artifact.module().bytes().len(),
            ),
        ),
        (Err(error), _) | (_, Err(error)) => qualification(
            RuntimeTarget::WasmJit,
            RuntimeTargetReadiness::Rejected,
            RuntimeTargetMaturity::QualificationOnly,
            error.to_string(),
        ),
    }
}

#[cfg(not(feature = "wasm-jit"))]
fn qualify_wasm_jit(
    _model: &CompiledModel,
    _canonical_ir: &CanonicalIrArtifact,
) -> RuntimeTargetQualification {
    qualification(
        RuntimeTarget::WasmJit,
        RuntimeTargetReadiness::Unavailable,
        RuntimeTargetMaturity::QualificationOnly,
        "browser WebAssembly JIT compiler feature is disabled in this build",
    )
}

fn qualify_native_if_requested(
    model: &CompiledModel,
    canonical_ir: &CanonicalIrArtifact,
    requested: bool,
) -> RuntimeTargetQualification {
    if requested {
        qualify_native(model, canonical_ir)
    } else {
        qualification(
            RuntimeTarget::NativeX64Jit,
            RuntimeTargetReadiness::Unavailable,
            RuntimeTargetMaturity::Preview,
            "host-native JIT qualification was not requested",
        )
    }
}

#[cfg(all(
    feature = "native",
    all(
        any(target_arch = "x86_64", target_arch = "aarch64"),
        any(target_os = "macos", target_os = "linux", windows)
    )
))]
fn qualify_native(
    model: &CompiledModel,
    canonical_ir: &CanonicalIrArtifact,
) -> RuntimeTargetQualification {
    match crate::native::compile_native_with_canonical_ir(model, canonical_ir) {
        Ok(_) => qualification(
            RuntimeTarget::NativeX64Jit,
            RuntimeTargetReadiness::Available,
            RuntimeTargetMaturity::Preview,
            format!(
                "{} JIT compiled successfully",
                crate::native::TargetSpec::host()
                    .map(|target| target.display_name())
                    .unwrap_or_else(|| "host-native".to_owned())
            ),
        ),
        Err(error) => qualification(
            RuntimeTarget::NativeX64Jit,
            RuntimeTargetReadiness::Rejected,
            RuntimeTargetMaturity::Preview,
            error.to_string(),
        ),
    }
}

#[cfg(all(
    feature = "native",
    not(all(
        any(target_arch = "x86_64", target_arch = "aarch64"),
        any(target_os = "macos", target_os = "linux", windows)
    ))
))]
fn qualify_native(
    _model: &CompiledModel,
    _canonical_ir: &CanonicalIrArtifact,
) -> RuntimeTargetQualification {
    qualification(
        RuntimeTarget::NativeX64Jit,
        RuntimeTargetReadiness::Unavailable,
        RuntimeTargetMaturity::Preview,
        "host architecture or operating system is not yet qualified for the native JIT",
    )
}

#[cfg(not(feature = "native"))]
fn qualify_native(
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
    generated: &GeneratedRustDevice,
    canonical_module: &str,
    canonical_digest: &str,
) -> Result<(), RuntimeArtifactIntegrityError> {
    if generated.module_name != canonical_module {
        return Err(RuntimeArtifactIntegrityError::GeneratedRustModuleMismatch {
            generated: generated.module_name.clone(),
            canonical: canonical_module.to_owned(),
        });
    }
    if generated.source_digest != canonical_digest {
        return Err(RuntimeArtifactIntegrityError::SourceDigestMismatch {
            artifact: "generated Rust",
            expected: canonical_digest.to_owned(),
            actual: generated.source_digest.clone(),
        });
    }
    if generated.files.is_empty() {
        return Err(RuntimeArtifactIntegrityError::GeneratedRustHasNoFiles);
    }

    let mut paths = BTreeSet::new();
    for file in &generated.files {
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

/// A required optimized backend was not available for the compiled model.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error(
    "required {} backend is {}: {}; interpreter fallback is disabled",
    .target.label(),
    .readiness.label(),
    .detail
)]
pub struct BackendQualificationError {
    pub target: RuntimeTarget,
    pub readiness: RuntimeTargetReadiness,
    pub detail: String,
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
    #[error("compiled model Zi filter {index} failed integrity validation: {detail}")]
    InvalidZiFilterState { index: usize, detail: String },
    #[error("ABI module '{abi}' does not match canonical module '{canonical}'")]
    AbiModuleMismatch { abi: String, canonical: String },
    #[error("ABI surface does not match the canonical model")]
    AbiSurfaceMismatch,
    #[error("runtime target qualification matrix is not exhaustive")]
    InvalidTargetMatrix,
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
    #[error("invalid Verilog-A specialist report: {0}")]
    InvalidSpecialistReport(String),
    #[error("invalid compile diagnostic: {0}")]
    InvalidCompileDiagnostic(String),
}

/// Compiler phase associated with a typed source diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompileDiagnosticPhase {
    Input,
    Lexer,
    Parser,
    Semantic,
    CodeGeneration,
    BackendQualification,
    PerformanceBudget,
    ModuleSelection,
}

/// How a source diagnostic bears on the compilation that produced it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompileDiagnosticSeverity {
    /// The compilation failed. Errors only ever arrive through
    /// [`compile_diagnostics`]; a report never carries one.
    Error,
    /// The compilation succeeded and the compiler accepted the source with a
    /// caveat the author must read — a construct it parsed and then dropped,
    /// for example. Warnings ride on [`RuntimeCompileReport::diagnostics`].
    Warning,
}

/// One-based source position. `column` counts Unicode scalar values, not bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileSourcePosition {
    pub line: u32,
    pub column: u32,
}

/// Exact compiler byte span plus display-ready source positions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileDiagnosticSpan {
    pub source_id: u32,
    pub byte_start: u32,
    pub byte_end: u32,
    pub start: Option<CompileSourcePosition>,
    pub end: Option<CompileSourcePosition>,
}

/// One typed, display-ready compiler diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileDiagnostic {
    pub severity: CompileDiagnosticSeverity,
    pub phase: CompileDiagnosticPhase,
    /// Stable compiler-owned diagnostic code, for example
    /// `VA-SEM-NO-EFFECT-SYSTEM-TASK`. Editors group, filter and document a
    /// diagnostic by this code; presentation layers never synthesize one.
    pub code: String,
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
    let code = error.diagnostic_code();
    match error {
        CompileError::Multiple(errors) => {
            for error in errors {
                collect_compile_diagnostics(source, error, diagnostics);
            }
        }
        CompileError::Lexer(inner) => diagnostics.push(diagnostic(
            source,
            CompileDiagnosticSeverity::Error,
            CompileDiagnosticPhase::Lexer,
            code,
            error.to_string(),
            Some(inner.span),
        )),
        CompileError::Parser(inner) => diagnostics.push(diagnostic(
            source,
            CompileDiagnosticSeverity::Error,
            CompileDiagnosticPhase::Parser,
            code,
            error.to_string(),
            Some(inner.span),
        )),
        CompileError::Semantic(inner) => diagnostics.push(diagnostic(
            source,
            CompileDiagnosticSeverity::Error,
            CompileDiagnosticPhase::Semantic,
            code,
            error.to_string(),
            Some(inner.span),
        )),
        CompileError::CodeGen(inner) => diagnostics.push(diagnostic(
            source,
            CompileDiagnosticSeverity::Error,
            CompileDiagnosticPhase::CodeGeneration,
            code,
            error.to_string(),
            inner.span,
        )),
        CompileError::BackendQualification(_) => diagnostics.push(diagnostic(
            source,
            CompileDiagnosticSeverity::Error,
            CompileDiagnosticPhase::BackendQualification,
            code,
            error.to_string(),
            None,
        )),
        CompileError::PerformanceBudget(_) => diagnostics.push(diagnostic(
            source,
            CompileDiagnosticSeverity::Error,
            CompileDiagnosticPhase::PerformanceBudget,
            code,
            error.to_string(),
            None,
        )),
        CompileError::Cancelled(_) => diagnostics.push(diagnostic(
            source,
            CompileDiagnosticSeverity::Error,
            CompileDiagnosticPhase::Input,
            code,
            error.to_string(),
            None,
        )),
        CompileError::ModuleSelection(_) => diagnostics.push(diagnostic(
            source,
            CompileDiagnosticSeverity::Error,
            CompileDiagnosticPhase::ModuleSelection,
            code,
            error.to_string(),
            None,
        )),
        CompileError::IoError { .. } | CompileError::VirtualSource(_) => {
            diagnostics.push(diagnostic(
                source,
                CompileDiagnosticSeverity::Error,
                CompileDiagnosticPhase::Input,
                code,
                error.to_string(),
                None,
            ))
        }
    }
}

/// Present the semantic analyzer's non-fatal findings as editor diagnostics.
///
/// `source` is the preprocessed text the analyzer saw, so line and column
/// resolution follows exactly the same rule as [`compile_diagnostics`].
pub(crate) fn semantic_warning_diagnostics(
    source: &str,
    warnings: &[crate::semantic::SemanticWarning],
) -> Vec<CompileDiagnostic> {
    warnings
        .iter()
        .map(|warning| {
            diagnostic(
                source,
                CompileDiagnosticSeverity::Warning,
                CompileDiagnosticPhase::Semantic,
                warning.code,
                warning.message.clone(),
                Some(warning.span),
            )
        })
        .collect()
}

fn diagnostic(
    source: &str,
    severity: CompileDiagnosticSeverity,
    phase: CompileDiagnosticPhase,
    code: &'static str,
    message: String,
    span: Option<Span>,
) -> CompileDiagnostic {
    CompileDiagnostic {
        severity,
        phase,
        code: code.to_owned(),
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
