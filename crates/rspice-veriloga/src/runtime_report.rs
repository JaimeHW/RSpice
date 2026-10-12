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

use crate::ast::PortDirection;
use crate::canonical_ir::{
    CanonicalIrArtifact, CanonicalValueType, HirExprKind, HirExprRef, HirModel, HirParamRange,
    HirParameter,
};
use crate::codegen::CompiledModel;
use crate::disciplines::Domain;
use crate::error::CompileError;
use crate::generated_source::GeneratedRustDevice;
use crate::metrics::PipelineMetrics;
#[cfg(feature = "rust-codegen")]
use crate::rust_backend::{RustTranspileOptions, RustTranspiler};
use crate::semantic::VectorBounds;
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
        if self.model.parameters.len() != self.canonical_ir.hir.parameters.len()
            || self
                .model
                .parameters
                .iter()
                .zip(&self.canonical_ir.hir.parameters)
                .any(|(model, hir)| {
                    model.elaboration_value.map(f64::to_bits)
                        != hir.elaboration_value.map(f64::to_bits)
                        || model.elaboration_given != hir.elaboration_given
                        || model
                            .elaboration_value
                            .is_some_and(|value| !value.is_finite())
                })
        {
            return Err(RuntimeArtifactIntegrityError::AbiSurfaceMismatch);
        }
        if self.abi != RuntimeAbiSummary::from_artifact(&self.canonical_ir)
            || self.abi.noise_source_count != self.model.noise_sources.len()
            || self.abi.internal_node_count != self.model.internal_nodes
            || !self
                .canonical_ir
                .hir
                .internal_nodes
                .iter()
                .enumerate()
                .filter_map(|(index, node)| node.is_state.then_some(index))
                .eq(self.model.internal_state_nodes.iter().copied())
        {
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
    /// Scalar terminals in binding order. A vector port contributes one
    /// terminal per lane, most significant first; the runtime binds an
    /// instance's nodes to this list.
    pub analog_ports: Vec<RuntimeAbiPort>,
    /// The module header's ports as declared, in declaration order, each
    /// naming its lanes in [`Self::analog_ports`].
    pub formal_ports: Vec<RuntimeAbiFormalPort>,
    pub parameters: Vec<RuntimeAbiParameter>,
    /// Exact packed values fixed by elaboration; recompile to change these.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub elaboration_parameters: Vec<crate::canonical_ir::digital::DigitalElaborationParameter>,
    /// Distinct structural processes, including assigned and filtered sources.
    /// Reusing one process at multiple injections does not increase this count.
    pub noise_source_count: usize,
    pub state_variable_count: usize,
    pub internal_node_count: usize,
    pub branch_unknown_count: usize,
    pub equation_count: usize,
}

impl RuntimeAbiSummary {
    fn from_artifact(artifact: &CanonicalIrArtifact) -> Self {
        let hir = &artifact.hir;
        Self {
            module_name: hir.module_name.clone(),
            analog_ports: hir
                .ports
                .iter()
                .map(|port| RuntimeAbiPort {
                    name: port.name.clone(),
                    direction: port.direction,
                    discipline: port.discipline.clone(),
                    potential_nature: port.nature_potential.clone(),
                    flow_nature: port.nature_flow.clone(),
                })
                .collect(),
            // HIR validation gives every formal port a first lane that names
            // a terminal, so nothing is dropped from a validated artifact.
            formal_ports: hir
                .formal_ports
                .iter()
                .filter_map(|port| {
                    let first = hir.ports.get(usize::from(*port.lanes.first()?))?;
                    Some(RuntimeAbiFormalPort {
                        name: port.name.clone(),
                        direction: first.direction,
                        discipline: first.discipline.clone(),
                        domain: port.domain,
                        range: port.range,
                        lanes: port.lanes.iter().copied().map(usize::from).collect(),
                    })
                })
                .collect(),
            parameters: hir
                .parameters
                .iter()
                .filter(|parameter| parameter.is_public)
                .map(|parameter| RuntimeAbiParameter {
                    name: parameter.name.clone(),
                    scope: parameter.scope,
                    value_type: parameter.value_type,
                    default: RuntimeAbiParameterValue::constant_default(hir, parameter),
                    range: parameter
                        .range
                        .as_ref()
                        .and_then(RuntimeAbiParameterRange::from_hir),
                    units: parameter.units.clone(),
                    description: parameter.description.clone(),
                    elaboration_value: parameter.elaboration_value,
                    elaboration_given: parameter.elaboration_given,
                    aliases: parameter.aliases.clone(),
                })
                .collect(),
            elaboration_parameters: artifact
                .digital
                .elaboration_parameters
                .iter()
                .filter(|parameter| parameter.is_public)
                .cloned()
                .collect(),
            noise_source_count: artifact
                .hir
                .expressions
                .iter()
                .filter_map(|expression| match expression.kind {
                    HirExprKind::NoiseSource { process_id, .. } => Some(process_id),
                    _ => None,
                })
                .collect::<BTreeSet<_>>()
                .len(),
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

/// One scalar terminal the runtime binds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAbiPort {
    pub name: SmolStr,
    pub direction: PortDirection,
    pub discipline: SmolStr,
    pub potential_nature: Option<SmolStr>,
    pub flow_nature: Option<SmolStr>,
}

/// One port of the module header, as declared.
///
/// `inout [7:0] d` is one formal port of width eight whose lanes are the
/// terminals `d[7]` through `d[0]`. A discrete-discipline vector is the
/// exception: its bits travel as one digital signal, so it has one lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAbiFormalPort {
    pub name: SmolStr,
    pub direction: PortDirection,
    /// Name of the port's discipline, such as `electrical` or `logic`.
    pub discipline: SmolStr,
    pub domain: Domain,
    /// Declared `[msb:lsb]`, exactly as written (`[0:7]` stays ascending);
    /// `None` for a scalar.
    pub range: Option<VectorBounds>,
    /// Indices into [`RuntimeAbiSummary::analog_ports`], most significant
    /// first.
    pub lanes: Vec<usize>,
}

/// One instance- or model-settable parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeAbiParameter {
    pub name: SmolStr,
    pub scope: crate::semantic::ParameterScope,
    pub value_type: CanonicalValueType,
    /// The declared default when it is a compile-time constant. `None` when
    /// the default is computed from other parameters, per instance, or the
    /// parameter is an array.
    pub default: Option<RuntimeAbiParameterValue>,
    /// The values the model accepts; `None` when it accepts any value.
    pub range: Option<RuntimeAbiParameterRange>,
    /// The declaration's `units` attribute.
    pub units: Option<SmolStr>,
    /// The declaration's `desc` attribute.
    pub description: Option<SmolStr>,
    #[serde(default)]
    pub elaboration_value: Option<f64>,
    #[serde(default)]
    pub elaboration_given: Option<bool>,
    pub aliases: Vec<SmolStr>,
}

/// A constant parameter value, typed as the parameter is declared.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuntimeAbiParameterValue {
    Real(#[serde(with = "crate::json_float")] f64),
    Integer(i64),
    String(SmolStr),
}

impl RuntimeAbiParameterValue {
    /// The value of a real or integer default; `None` for a string.
    pub fn numeric(&self) -> Option<f64> {
        match self {
            Self::Real(value) => Some(*value),
            Self::Integer(value) => Some(*value as f64),
            Self::String(_) => None,
        }
    }

    fn constant_default(hir: &HirModel, parameter: &HirParameter) -> Option<Self> {
        match parameter.value_type {
            // A string default never folds to a number; a literal is the
            // only constant form it has.
            CanonicalValueType::String => {
                let default = parameter.default_expr.as_ref()?;
                match &hir.expressions.get(usize::from(default.id))?.kind {
                    HirExprKind::StringLiteral { value } => Some(Self::String(value.clone())),
                    _ => None,
                }
            }
            // Integer defaults are folded to integral values before lowering.
            CanonicalValueType::Integer => parameter.default.map(|value| {
                if value.fract() == 0.0 && value.abs() <= i64::MAX as f64 {
                    Self::Integer(value as i64)
                } else {
                    Self::Real(value)
                }
            }),
            _ => parameter.default.map(Self::Real),
        }
    }
}

/// The values a parameter accepts: one `from` interval and its `exclude`
/// clauses. Bounds that name other parameters are reported as they read in
/// the source; the compiled model checks them once an instance's values are
/// known.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeAbiParameterRange {
    pub lower: RuntimeAbiRangeBound,
    pub upper: RuntimeAbiRangeBound,
    /// Values the parameter may not take: the constants first, then those
    /// that depend on other parameters.
    pub exclude: Vec<RuntimeAbiRangeValue>,
}

/// One end of a `from` interval.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuntimeAbiRangeBound {
    /// `-inf` or `inf`: no limit on this side.
    Unbounded,
    /// `[` or `]`: the bound itself is accepted.
    Inclusive(RuntimeAbiRangeValue),
    /// `(` or `)`: the bound itself is refused.
    Exclusive(RuntimeAbiRangeValue),
}

/// A range bound or excluded value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuntimeAbiRangeValue {
    /// Folded at compile time; a property editor can enforce it.
    Constant(#[serde(with = "crate::json_float")] f64),
    /// Depends on other parameters, given as its source text; known only
    /// once an instance's parameters are.
    Dependent(SmolStr),
}

impl RuntimeAbiParameterRange {
    fn from_hir(range: &HirParamRange) -> Option<Self> {
        // A dependent bound's text is recorded at compile time; one lowered
        // without its source still has to read as dependent, not as absent.
        let bound = |constant: Option<f64>,
                     parameter: Option<&SmolStr>,
                     expression: Option<&HirExprRef>,
                     text: Option<&SmolStr>,
                     exclusive: bool| {
            let value = match constant {
                Some(value) => RuntimeAbiRangeValue::Constant(value),
                None => match text
                    .or(parameter)
                    .or(expression.map(|expression| &expression.kind))
                {
                    Some(text) => RuntimeAbiRangeValue::Dependent(text.clone()),
                    None => return RuntimeAbiRangeBound::Unbounded,
                },
            };
            if exclusive {
                RuntimeAbiRangeBound::Exclusive(value)
            } else {
                RuntimeAbiRangeBound::Inclusive(value)
            }
        };
        let lower = bound(
            range.min,
            range.min_parameter.as_ref(),
            range.min_expression.as_ref(),
            range.min_text.as_ref(),
            range.min_exclusive,
        );
        let upper = bound(
            range.max,
            range.max_parameter.as_ref(),
            range.max_expression.as_ref(),
            range.max_text.as_ref(),
            range.max_exclusive,
        );
        let exclude: Vec<_> = range
            .exclude
            .iter()
            .copied()
            .map(RuntimeAbiRangeValue::Constant)
            .chain(
                range
                    .exclude_parameters
                    .iter()
                    .cloned()
                    .map(RuntimeAbiRangeValue::Dependent),
            )
            .chain(
                range
                    .exclude_expressions
                    .iter()
                    .enumerate()
                    .map(|(index, expression)| {
                        RuntimeAbiRangeValue::Dependent(
                            range
                                .exclude_texts
                                .get(index)
                                .unwrap_or(&expression.kind)
                                .clone(),
                        )
                    }),
            )
            .collect();
        let unconstrained = lower == RuntimeAbiRangeBound::Unbounded
            && upper == RuntimeAbiRangeBound::Unbounded
            && exclude.is_empty();
        (!unconstrained).then_some(Self {
            lower,
            upper,
            exclude,
        })
    }
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
        qualify_generated_rust(canonical_ir)
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

#[cfg(feature = "rust-codegen")]
fn qualify_generated_rust(
    canonical_ir: &CanonicalIrArtifact,
) -> (RuntimeTargetQualification, Option<GeneratedRustDevice>) {
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
}

#[cfg(not(feature = "rust-codegen"))]
fn qualify_generated_rust(
    _canonical_ir: &CanonicalIrArtifact,
) -> (RuntimeTargetQualification, Option<GeneratedRustDevice>) {
    (
        qualification(
            RuntimeTarget::GeneratedRust,
            RuntimeTargetReadiness::Unavailable,
            RuntimeTargetMaturity::QualificationOnly,
            "generated Rust backend is disabled; enable the rust-codegen feature",
        ),
        None,
    )
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
/// Messages omit offsets so callers can attach remapped source locations
/// without also displaying a contradictory expanded-input offset.
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
            format!("Lexer error: {}", inner.kind),
            Some(inner.span),
        )),
        CompileError::Parser(inner) => diagnostics.push(diagnostic(
            source,
            CompileDiagnosticSeverity::Error,
            CompileDiagnosticPhase::Parser,
            code,
            format!("Parse error: {}", inner.kind),
            Some(inner.span),
        )),
        CompileError::Semantic(inner) => diagnostics.push(diagnostic(
            source,
            CompileDiagnosticSeverity::Error,
            CompileDiagnosticPhase::Semantic,
            code,
            format!("Semantic error: {}", inner.kind),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic::ParameterScope;

    /// One module carrying every port shape and range shape the ABI reports.
    const ABI_SURFACE: &str = r#"
module abi_surface(sense, out, d, a, mode);
  input sense;
  output out;
  inout [3:0] d;
  input [0:1] a;
  input [1:0] mode;
  electrical sense, out;
  electrical [3:0] d;
  electrical [0:1] a;
  logic mode;
  (* desc="Transconductance", units="S", type="instance" *)
  parameter real gain = 2.5 from (0:inf);
  parameter real r = 1000 from [1:1e6];
  parameter real vt = 0.5 from (0:1);
  parameter real tc = 0 from [-1:1);
  parameter real offs = 0 from (-inf:0];
  parameter real k = 2 from [0:inf) exclude 1;
  parameter real wmin = 1e-6 from (0:inf);
  parameter real w = 2e-6 from [wmin:inf) exclude 2*wmin;
  parameter real l = 1e-6 from (0:2*w];
  parameter real lmax = 4*w;
  parameter integer stages = 3 from [1:8];
  analog begin
    I(out) <+ gain * V(sense) + (mode[0] ? vt : offs) + tc + k * l * lmax * stages;
    I(d[3], d[0]) <+ V(d[3], d[0]) / r;
    I(a[0], a[1]) <+ V(a[0], a[1]) / r;
  end
endmodule
"#;

    fn compile() -> RuntimeCompileReport {
        crate::VerilogACompiler::new(crate::CompilerOptions {
            enable_ams: true,
            ..crate::CompilerOptions::default()
        })
        .compile_runtime(ABI_SURFACE, Some("abi_surface"))
        .expect("the ABI surface module compiles")
    }

    fn parameter<'a>(abi: &'a RuntimeAbiSummary, name: &str) -> &'a RuntimeAbiParameter {
        abi.parameters
            .iter()
            .find(|parameter| parameter.name == name)
            .unwrap_or_else(|| panic!("parameter '{name}' is in the ABI"))
    }

    fn constant(value: f64) -> RuntimeAbiRangeValue {
        RuntimeAbiRangeValue::Constant(value)
    }

    fn dependent(text: &str) -> RuntimeAbiRangeValue {
        RuntimeAbiRangeValue::Dependent(text.into())
    }

    fn range(
        lower: RuntimeAbiRangeBound,
        upper: RuntimeAbiRangeBound,
        exclude: Vec<RuntimeAbiRangeValue>,
    ) -> Option<RuntimeAbiParameterRange> {
        Some(RuntimeAbiParameterRange {
            lower,
            upper,
            exclude,
        })
    }

    #[test]
    fn abi_groups_lanes_into_the_declared_formal_ports() {
        let report = compile();
        let abi = &report.abi;
        let lanes: Vec<_> = abi
            .analog_ports
            .iter()
            .map(|port| (port.name.as_str(), port.direction))
            .collect();
        assert_eq!(
            lanes,
            [
                ("sense", PortDirection::Input),
                ("out", PortDirection::Output),
                ("d[3]", PortDirection::Inout),
                ("d[2]", PortDirection::Inout),
                ("d[1]", PortDirection::Inout),
                ("d[0]", PortDirection::Inout),
                ("a[0]", PortDirection::Input),
                ("a[1]", PortDirection::Input),
                ("mode", PortDirection::Input),
            ]
        );

        let formal = |name: &str,
                      direction: PortDirection,
                      discipline: &str,
                      domain: Domain,
                      range: Option<(i64, i64)>,
                      lanes: &[usize]| RuntimeAbiFormalPort {
            name: name.into(),
            direction,
            discipline: discipline.into(),
            domain,
            range: range.map(|(msb, lsb)| VectorBounds { msb, lsb }),
            lanes: lanes.to_vec(),
        };
        assert_eq!(
            abi.formal_ports,
            [
                formal(
                    "sense",
                    PortDirection::Input,
                    "electrical",
                    Domain::Continuous,
                    None,
                    &[0]
                ),
                formal(
                    "out",
                    PortDirection::Output,
                    "electrical",
                    Domain::Continuous,
                    None,
                    &[1]
                ),
                formal(
                    "d",
                    PortDirection::Inout,
                    "electrical",
                    Domain::Continuous,
                    Some((3, 0)),
                    &[2, 3, 4, 5]
                ),
                // Ascending as written; lanes still run from the left bound.
                formal(
                    "a",
                    PortDirection::Input,
                    "electrical",
                    Domain::Continuous,
                    Some((0, 1)),
                    &[6, 7]
                ),
                // A discrete vector is one terminal carrying the whole signal.
                formal(
                    "mode",
                    PortDirection::Input,
                    "logic",
                    Domain::Discrete,
                    Some((1, 0)),
                    &[8]
                ),
            ]
        );
        report.validate_integrity().expect("the report is coherent");
    }

    #[test]
    fn abi_reports_parameter_defaults_ranges_and_attributes() {
        use RuntimeAbiRangeBound::{Exclusive, Inclusive, Unbounded};
        let report = compile();
        let abi = &report.abi;

        let gain = parameter(abi, "gain");
        assert_eq!(gain.scope, ParameterScope::Instance);
        assert_eq!(gain.value_type, CanonicalValueType::Real);
        assert_eq!(gain.default, Some(RuntimeAbiParameterValue::Real(2.5)));
        assert_eq!(gain.units.as_deref(), Some("S"));
        assert_eq!(gain.description.as_deref(), Some("Transconductance"));
        assert_eq!(
            gain.range,
            range(Exclusive(constant(0.0)), Unbounded, vec![])
        );

        let r = parameter(abi, "r");
        assert_eq!(r.scope, ParameterScope::Model);
        assert_eq!((r.units.as_deref(), r.description.as_deref()), (None, None));
        assert_eq!(
            r.range,
            range(Inclusive(constant(1.0)), Inclusive(constant(1e6)), vec![])
        );
        assert_eq!(
            parameter(abi, "vt").range,
            range(Exclusive(constant(0.0)), Exclusive(constant(1.0)), vec![])
        );
        assert_eq!(
            parameter(abi, "tc").range,
            range(Inclusive(constant(-1.0)), Exclusive(constant(1.0)), vec![])
        );
        assert_eq!(
            parameter(abi, "offs").range,
            range(Unbounded, Inclusive(constant(0.0)), vec![])
        );
        assert_eq!(
            parameter(abi, "k").range,
            range(Inclusive(constant(0.0)), Unbounded, vec![constant(1.0)])
        );

        // Bounds that name other parameters read as written and are marked
        // dependent: a property editor cannot check them on its own.
        let w = parameter(abi, "w");
        assert_eq!(w.default, Some(RuntimeAbiParameterValue::Real(2e-6)));
        assert_eq!(
            w.range,
            range(
                Inclusive(dependent("wmin")),
                Unbounded,
                vec![dependent("2*wmin")]
            )
        );
        assert_eq!(
            parameter(abi, "l").range,
            range(
                Exclusive(constant(0.0)),
                Inclusive(dependent("2*w")),
                vec![]
            )
        );
        let lmax = parameter(abi, "lmax");
        assert_eq!((&lmax.default, &lmax.range), (&None, &None));

        let stages = parameter(abi, "stages");
        assert_eq!(stages.value_type, CanonicalValueType::Integer);
        assert_eq!(stages.default, Some(RuntimeAbiParameterValue::Integer(3)));
        assert_eq!(
            stages.range,
            range(Inclusive(constant(1.0)), Inclusive(constant(8.0)), vec![])
        );
    }

    /// The bytecode generator cannot lower a string literal ("String literals
    /// in expressions"), and a string parameter's default is one, so a module
    /// that declares one never reaches a runtime report yet. The ABI is
    /// derived from the canonical artifact, where the default survives.
    #[test]
    fn abi_types_a_string_default_from_the_canonical_artifact() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir_module(
                r#"
module labelled(p, n);
  inout p, n;
  electrical p, n;
  (* desc="Process corner" *) parameter string corner = "typ";
  analog I(p, n) <+ V(p, n);
endmodule
"#,
                Some("labelled"),
            )
            .expect("a string parameter lowers to canonical IR");
        let abi = RuntimeAbiSummary::from_artifact(&artifact);
        let corner = parameter(&abi, "corner");
        assert_eq!(corner.value_type, CanonicalValueType::String);
        assert_eq!(
            corner.default,
            Some(RuntimeAbiParameterValue::String("typ".into()))
        );
        assert_eq!(corner.description.as_deref(), Some("Process corner"));
        assert_eq!(corner.range, None);
    }

    #[test]
    fn abi_survives_transport_and_is_rechecked_against_its_artifact() {
        let report = compile();
        let encoded = serde_json::to_string(&report.abi).expect("encode the ABI");
        let decoded: RuntimeAbiSummary = serde_json::from_str(&encoded).expect("decode the ABI");
        assert_eq!(decoded, report.abi);

        // The ABI is derived from the canonical artifact, so a doctored ABI
        // and a doctored formal-port table are both refused.
        let mut forged = report.clone();
        forged.abi.formal_ports[2].lanes.pop();
        assert_eq!(
            forged.validate_integrity(),
            Err(RuntimeArtifactIntegrityError::AbiSurfaceMismatch)
        );
        let mut orphaned = report.clone();
        orphaned.canonical_ir.hir.formal_ports[2].lanes.pop();
        let Err(RuntimeArtifactIntegrityError::InvalidCanonicalIr { diagnostics }) =
            orphaned.validate_integrity()
        else {
            panic!("a terminal outside every formal port must be refused");
        };
        assert!(
            diagnostics
                .iter()
                .any(|message| message.contains("'d[0]' belongs to no formal port")),
            "{diagnostics:?}"
        );
    }
}
