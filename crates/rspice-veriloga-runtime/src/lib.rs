//! Stable, engine-neutral runtime ABI for precompiled Verilog-A models.
//!
//! Generated model crates call these concrete types directly.
//!
//! **This crate must not depend on `rspice-core`.** The dependency edges run
//! `rspice-core` -> `rspice-veriloga-models` -> the 43 generated model crates,
//! and each of those leaves depends on exactly this crate for its ABI. Moving
//! these types into `rspice-core`, or adding a `rspice-core` dependency here,
//! closes that path into a cycle.
//!
//! That is also what buys the build headroom: because the leaves stop at this
//! crate, Cargo compiles and caches all 43 independently instead of folding
//! them into one `rspice-core` translation unit. Measured 2026-08-01, the
//! split cut peak single-`rustc` memory for the full corpus from 9.96 GB to
//! 2.59 GB, back under the 3 GB build gate.

// Fixed-arity stamp entry points deliberately keep small derivative sets in
// scalar arguments so generated call sites inline without temporary arrays.
#![allow(clippy::too_many_arguments)]

pub mod absdelta;
mod analog_effects;
mod analog_lifecycle;
pub mod arithmetic;
pub mod array_index;
mod compatibility_catalog;
mod descriptor;
mod event_control;
mod ground_reference;
pub mod integer;
mod integration;
mod integration_state;
mod lanes;
mod limexp;
mod noise_frequency;
pub mod polynomial;
mod simparam;
pub mod transport_delay;

pub use ground_reference::{
    GroundGuardOp, GroundJoin, GroundReference, GroundReferenceGraph, GroundReferenceInputs,
    evaluate_guard as evaluate_ground_guard,
};
pub use simparam::{GeneratedSimulationParameters, SimulationParameter};

pub use analog_effects::{
    AnalogEffectError, AnalogEffectJournal, AnalogEffectLimits, AnalogTaskArgument,
    AnalogTaskEvent, AnalogTaskInvocation, AnalogTaskKind, AnalogTaskSource,
};
pub use analog_lifecycle::{
    ANALYSIS_QUERY_COUNT, AnalogAnalysisPhase, AnalogEvaluationPhase, active_analysis_query_names,
    analysis_query_id, analysis_query_mask, analysis_query_name,
};

pub use compatibility_catalog::{
    GENERATED_VERILOGA_COMPATIBILITY_CATALOG, GENERATED_VERILOGA_V27_COMBINED_IDENTITY_ALIASES,
    GeneratedVerilogACompatibilityCatalogEntry,
    generated_veriloga_accepted_state_shape_is_compatible,
    generated_veriloga_checkpoint_compatibility_entry, generated_veriloga_compatibility_entry,
    generated_veriloga_v26_compatibility_entry, generated_veriloga_wire_compatibility_entry,
    validate_generated_veriloga_compatibility_catalog,
};

pub use descriptor::*;
pub use integration::{
    GeneratedDdtAcceptedHistory, GeneratedDdtCandidate, GeneratedDdtCandidateError,
    GeneratedIdtAcceptedHistory, GeneratedIdtCandidate, GeneratedIdtCandidateError,
    GeneratedIdtModBranch, GeneratedIdtModCandidate, GeneratedIdtModCandidateError,
    evaluate_generated_ddt_candidate, evaluate_generated_ddt_derivative,
    evaluate_generated_idt_candidate, evaluate_generated_idt_derivative,
    evaluate_generated_idtmod_branch_derivative, evaluate_generated_idtmod_candidate,
    evaluate_generated_idtmod_derivative, evaluate_generated_idtmod_initial, idtmod_wrapped_value,
    rspice_eval_ddt, rspice_eval_idt,
};
pub use integration_state::{GeneratedIdtModPersistentState, GeneratedIdtModState};
pub use lanes::*;
pub use limexp::*;

pub use event_control::{
    GeneratedCheckpointLaneDescriptor, GeneratedCheckpointLaneType, GeneratedCrossEvaluation,
    GeneratedCrossState, GeneratedEventControlError, evaluate_generated_above,
    evaluate_generated_cross, evaluate_generated_last_crossing, evaluate_generated_timer,
    validate_generated_cross_state,
};

pub type Value = f64;

/// Thermal voltage per kelvin (V/K), rounded once from the exact SI ratio k/q.
/// The reduced integer operands are exactly representable in binary64. Using
/// them avoids rounding k and q separately before division. Multiply temperature
/// by this ratio to preserve subnormal results without an intermediate k*T.
pub const THERMAL_VOLTAGE_PER_K: Value = 1_380_649.0 / 16_021_766_340.0;

/// Numerical minimum with the compiler's explicit operand-selection contract:
/// a number wins over NaN, equal numbers retain the left operand (including its
/// zero sign), and two NaNs retain the right operand.
#[inline]
pub fn rspice_min(left: Value, right: Value) -> Value {
    if !left.is_nan() && (left <= right || right.is_nan()) {
        left
    } else {
        right
    }
}

/// Numerical maximum with the same selection contract as [`rspice_min`].
#[inline]
pub fn rspice_max(left: Value, right: Value) -> Value {
    if !left.is_nan() && (left >= right || right.is_nan()) {
        left
    } else {
        right
    }
}

/// Validate a symbolic derivative at its original expression's operating point.
/// Both values must survive simplification even when the derivative is zero.
#[inline]
pub fn checked_derivative_value(primal: Value, derivative: Value) -> Result<Value, &'static str> {
    if !primal.is_finite() {
        Err("ddx operand is not finite")
    } else if !derivative.is_finite() {
        Err("ddx derivative is not finite")
    } else {
        Ok(derivative)
    }
}

pub use array_index::checked_discrete_value;

use rspice_matrix::{ComplexMatrix, CscIndex, StaticMatrix};

const DEFAULT_GMIN: Value = 1.0e-12;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedDdtCoefficients {
    pub active: bool,
    pub derivative_scale: Value,
    pub previous_value_scale: Value,
    pub older_value_scale: Value,
    pub previous_derivative_scale: Value,
}

/// Smallest supported transient interval magnitude for Verilog-A companion rules.
///
/// The boundary is inclusive. Smaller nonzero intervals are rejected rather
/// than returning inactive coefficients, which would drop reactive terms and
/// evaluate a different circuit. Generated, runtime-compiled, and mixed models
/// share this constructor and limit.
pub const GENERATED_DDT_TIMESTEP_FLOOR: Value = 1.0e-20;

/// A transient interval a generated `ddt` companion rule cannot integrate.
///
/// Zero is not one of these: it is the static-evaluation convention, and it
/// yields inactive coefficients. This names a step the caller intended to
/// integrate over and the rule cannot represent.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedDdtTimestepError {
    /// The interval the caller asked for.
    pub timestep: Value,
    /// [`GENERATED_DDT_TIMESTEP_FLOOR`], repeated so the message is complete
    /// wherever it is rendered.
    pub floor: Value,
}

impl std::fmt::Display for GeneratedDdtTimestepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "generated ddt cannot integrate a transient timestep of {:.16e}s: the companion rule needs at least {:.16e}s",
            self.timestep, self.floor
        )
    }
}

impl std::error::Error for GeneratedDdtTimestepError {}

impl GeneratedDdtCoefficients {
    #[inline]
    pub const fn inactive() -> Self {
        Self {
            active: false,
            derivative_scale: 0.0,
            previous_value_scale: 0.0,
            older_value_scale: 0.0,
            previous_derivative_scale: 0.0,
        }
    }

    #[inline]
    pub fn from_companion_values(
        coeff_g: Value,
        coeff_v_n: Value,
        coeff_v_n_minus_1: Value,
        needs_two_history: bool,
        needs_current_history: bool,
        timestep: Value,
    ) -> Result<Self, GeneratedDdtTimestepError> {
        Self::from_companion_values_with_derivative_scale(
            coeff_g,
            coeff_v_n,
            coeff_v_n_minus_1,
            needs_two_history,
            if needs_current_history { 1.0 } else { 0.0 },
            timestep,
        )
    }

    /// Construct generalized integration coefficients whose previous
    /// derivative term may have a fractional weight.
    ///
    /// A zero timestep is the static-evaluation convention and yields inactive
    /// coefficients. Any other interval the rule cannot represent —
    /// non-finite, or below [`GENERATED_DDT_TIMESTEP_FLOOR`] — is refused, so
    /// the caller decides what an unintegrable step means instead of receiving
    /// a module silently evaluated without its `ddt` terms.
    #[inline]
    pub fn from_companion_values_with_derivative_scale(
        coeff_g: Value,
        coeff_v_n: Value,
        coeff_v_n_minus_1: Value,
        needs_two_history: bool,
        previous_derivative_scale: Value,
        timestep: Value,
    ) -> Result<Self, GeneratedDdtTimestepError> {
        if timestep == 0.0 {
            return Ok(Self::inactive());
        }
        if !timestep.is_finite() || timestep.abs() < GENERATED_DDT_TIMESTEP_FLOOR {
            return Err(GeneratedDdtTimestepError {
                timestep,
                floor: GENERATED_DDT_TIMESTEP_FLOOR,
            });
        }

        let inverse_timestep = 1.0 / timestep;
        Ok(Self {
            active: true,
            derivative_scale: coeff_g * inverse_timestep,
            previous_value_scale: coeff_v_n * inverse_timestep,
            older_value_scale: if needs_two_history {
                coeff_v_n_minus_1 * inverse_timestep
            } else {
                0.0
            },
            previous_derivative_scale,
        })
    }

    /// Scale the complete dynamic residual without changing its history
    /// representation. Xyce OneStep order two halves the enclosing static
    /// device residual, so generated `ddt` terms are doubled before that
    /// common scaling to retain `(Q_n - Q_previous) / h` at unit weight.
    #[inline]
    pub fn scaled(self, factor: Value) -> Self {
        debug_assert!(factor.is_finite() && factor >= 0.0);
        if !self.active || !factor.is_finite() || factor < 0.0 {
            return Self::inactive();
        }
        Self {
            active: true,
            derivative_scale: self.derivative_scale * factor,
            previous_value_scale: self.previous_value_scale * factor,
            older_value_scale: self.older_value_scale * factor,
            previous_derivative_scale: self.previous_derivative_scale * factor,
        }
    }
}

impl Default for GeneratedDdtCoefficients {
    #[inline]
    fn default() -> Self {
        Self::inactive()
    }
}

/// Accepted dynamic history needed to resume a generated Verilog-A instance.
///
/// Parameters, topology, static caches, and scratch buffers are deliberately
/// excluded: a resumed circuit rebuilds those from the canonical model and
/// validates their source digest before this state is restored.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GeneratedVerilogAPersistentState {
    pub ddt_previous: Vec<Value>,
    pub ddt_older: Vec<Value>,
    pub ddt_derivative_previous: Vec<Value>,
    pub ddt_initialized: Vec<bool>,
    pub idt_previous: Vec<Value>,
    pub idt_older: Vec<Value>,
    pub idt_input_previous: Vec<Value>,
    pub idt_initialized: Vec<bool>,
    /// Circular integrators in generated dense-site order. These retain their
    /// exact origins separately from ordinary integral and event state.
    pub idtmod: Vec<GeneratedIdtModPersistentState>,
    /// Accepted event-controlled procedural variables in generated dense-slot
    /// order. Speculative candidates are intentionally not persistent.
    pub event_variables: Vec<Value>,
    pub limiter_anchor: Vec<Value>,
    pub limiter_initialized: Vec<bool>,
}

/// Mutable evaluation state captured around rejected nonlinear trial points.
///
/// Parameter values, topology, static caches, and linked matrix locations are
/// deliberately excluded. Generated models pack DDT/IDT history, accepted and
/// candidate event variables, and limiter state into these contiguous buffers.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GeneratedVerilogARollbackState {
    pub values: Vec<Value>,
    pub flags: Vec<bool>,
    pub idtmod: Vec<GeneratedIdtModState>,
    /// Task models retain the exact delivery and candidate state during solver
    /// rollback. Numerical-only models leave this unallocated.
    pub analog_effects: Option<Box<AnalogEffectJournal>>,
}

pub const GENERATED_PERSISTENT_STATE_VERSION: u32 = 5;

/// Persistent state plus exact generated-model and instance provenance.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedVerilogAInstanceCheckpoint {
    pub instance_name: String,
    pub model_name: String,
    pub descriptor_abi_version: u32,
    pub source_identity: String,
    pub model_identity: String,
    pub accepted_state_shape_identity: GeneratedVerilogAAcceptedStateShapeIdentity,
    pub state_version: u32,
    pub state: GeneratedVerilogAPersistentState,
    /// Exact currents entering external module terminals at the accepted
    /// checkpoint boundary, in canonical declaration order.
    pub terminal_currents: Vec<Value>,
}

/// Which unknown a lane of a packed derivative array belongs to.
///
/// A packed array is sized to the whole device, so a model with more lanes than
/// any one equation uses carries `Unused` entries rather than resizing per
/// equation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedStampLane {
    Node(usize),
    Branch(usize),
    Unused,
}

/// A recoverable failure reported while evaluating generated device code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedEvaluationError {
    SimulationParameter {
        name: &'static str,
    },
    DiscreteValue {
        reason: &'static str,
    },
    Derivative {
        reason: &'static str,
    },
    ArrayIndex {
        reason: &'static str,
        non_finite: bool,
    },
    Integer {
        reason: &'static str,
        /// Whether the refusal was caused by a non-finite operand.
        ///
        /// The interpreter and the native backend split the same
        /// [`integer::IntegerRuntimeError`] the same way: only
        /// `NonFiniteOperand` is a property of the point the solver handed the
        /// device. A finite operand that rounds outside the signed 32-bit
        /// range, a division by zero, a modulus by zero and zero raised to a
        /// negative power all fail identically at every iterate.
        non_finite: bool,
    },
    /// A contribution reached the matrix or the right-hand side without being
    /// finite.
    ///
    /// Generated code stamps whatever its arithmetic produced, so an
    /// expression that leaves its domain at an overshooting iterate — a
    /// logarithm of a negative, an exponential that overflows — writes NaN or
    /// infinity into the Newton system instead of refusing the point. The
    /// stamper observes every value it writes and reports this once for the
    /// whole evaluation; `target` names which of the two sinks saw it.
    NonFiniteContribution {
        target: &'static str,
    },
    DiscontinuityDegree,
    SmallSignal {
        reason: &'static str,
    },
    Initialization {
        slot: usize,
    },
    AnalogTask {
        site: u32,
        source: AnalogEffectError,
    },
    AnalogLoopLimit {
        phase: &'static str,
        iterations: usize,
        limit: usize,
    },
    DdtCandidate {
        slot: usize,
        source: GeneratedDdtCandidateError,
    },
    IdtCandidate {
        slot: usize,
        source: GeneratedIdtCandidateError,
    },
    IdtModCandidate {
        slot: Option<usize>,
        source: GeneratedIdtModCandidateError,
    },
    EventControl {
        operator: &'static str,
        slot: usize,
        source: GeneratedEventControlError,
    },
}

impl std::fmt::Display for GeneratedEvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SimulationParameter { name } => write!(
                f,
                "simulation parameter '{name}' is unavailable and has no fallback"
            ),
            Self::DiscreteValue { reason } => write!(f, "{reason}"),
            Self::Derivative { reason } => {
                write!(
                    f,
                    "generated Verilog-A derivative evaluation failed: {reason}"
                )
            }
            Self::ArrayIndex { reason, .. } => {
                write!(f, "generated Verilog-A array index failed: {reason}")
            }
            Self::Integer { reason, .. } => {
                write!(f, "generated Verilog-A integer evaluation failed: {reason}")
            }
            Self::NonFiniteContribution { target } => write!(
                f,
                "generated Verilog-A device stamped a non-finite {target}"
            ),
            Self::DiscontinuityDegree => {
                f.write_str("$discontinuity degree must have a finite integer value >= -1")
            }
            Self::SmallSignal { reason } => write!(
                f,
                "generated Verilog-A small-signal evaluation failed: {reason}"
            ),
            Self::Initialization { slot } => write!(
                f,
                "generated Verilog-A initialization has an invalid numeric value at slot {slot}"
            ),
            Self::AnalogTask { site, source } => {
                write!(
                    f,
                    "generated Verilog-A system task at site {site} failed: {source}"
                )
            }
            Self::AnalogLoopLimit {
                phase,
                iterations,
                limit,
            } => write!(
                f,
                "generated Verilog-A {phase} exceeded its analog-loop limit: {iterations} iterations (limit {limit})"
            ),
            Self::DdtCandidate { slot, source } => {
                write!(f, "generated Verilog-A ddt slot {slot} failed: {source}")
            }
            Self::IdtCandidate { slot, source } => {
                write!(f, "generated Verilog-A idt slot {slot} failed: {source}")
            }
            Self::IdtModCandidate { slot, source } => {
                if let Some(slot) = slot {
                    write!(f, "generated Verilog-A idtmod slot {slot} failed: {source}")
                } else {
                    write!(f, "generated Verilog-A idtmod evaluation failed: {source}")
                }
            }
            Self::EventControl {
                operator,
                slot,
                source,
            } => write!(
                f,
                "generated Verilog-A {operator} event slot {slot} failed: {source}"
            ),
        }
    }
}

impl std::error::Error for GeneratedEvaluationError {}

/// Generated evaluation failure with concrete model and instance provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedVerilogAEvaluationError {
    pub instance_name: String,
    pub model_name: &'static str,
    pub source: GeneratedEvaluationError,
}

impl std::fmt::Display for GeneratedVerilogAEvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "generated Verilog-A instance '{}' (model '{}') failed: {}",
            self.instance_name, self.model_name, self.source
        )
    }
}

impl std::error::Error for GeneratedVerilogAEvaluationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

/// Canonical Verilog-A noise primitive represented by generated device code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedNoiseKind {
    White,
    Flicker,
    Table,
}

/// One endpoint of a generated noise contribution in device-local topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedNoiseEndpoint {
    /// Index into the generated instance's complete node array. `None` is ground.
    pub local_node: Option<usize>,
    pub name: &'static str,
    pub is_internal: bool,
}

/// Immutable metadata emitted from a canonical Verilog-A noise contribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedNoiseDescriptor {
    pub mechanism: &'static str,
    pub label: Option<&'static str>,
    pub kind: GeneratedNoiseKind,
    pub equation: usize,
    pub is_current: bool,
    pub branch_ordinal: Option<usize>,
    pub pos: GeneratedNoiseEndpoint,
    pub neg: GeneratedNoiseEndpoint,
    pub table_len: usize,
    pub table_log_interp: bool,
}

/// One semantic noise process. Unlike [`GeneratedNoiseDescriptor`], this does
/// not identify an injection: a process can drive several equations
/// coherently and each injection is described separately below.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedNoiseProcessDescriptor {
    pub process_id: usize,
    pub label: Option<&'static str>,
    pub kind: GeneratedNoiseKind,
    pub table_len: usize,
    pub table_log_interp: bool,
}

/// One circuit injection of a generated semantic noise process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedNoiseInjectionDescriptor {
    pub process_id: usize,
    pub equation: usize,
    pub is_current: bool,
    pub branch_ordinal: Option<usize>,
    pub pos: GeneratedNoiseEndpoint,
    pub neg: GeneratedNoiseEndpoint,
}

/// ABI-stable complex number used by generated noise code without adding a
/// complex-number crate to every generated model package.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GeneratedNoiseComplex {
    pub re: Value,
    pub im: Value,
}

impl GeneratedNoiseComplex {
    /// Compose `(real + j*2*pi*frequency_hz*reactive) * scale`, converting
    /// to binary64 only after all factors can restore an intermediate range.
    #[inline]
    pub fn scaled_transfer(
        real: Value,
        reactive: Value,
        frequency_hz: Value,
        scale: Value,
    ) -> Self {
        let im = if reactive == 0.0 || frequency_hz == 0.0 {
            // A finite frequency needs no angular-frequency conversion for
            // a structural zero. Invalid operands still propagate as NaN.
            (frequency_hz * reactive) * scale
        } else {
            let omega = core::f64::consts::TAU * frequency_hz;
            let partial = omega * reactive;
            let value = partial * scale;
            if omega.is_normal() && partial.is_normal() {
                // The scale product is the final conversion boundary.
                value
            } else {
                Self::wide_reactive_transfer(reactive, frequency_hz, scale)
            }
        };
        Self {
            re: real * scale,
            im,
        }
    }

    #[cold]
    #[inline(never)]
    fn wide_reactive_transfer(reactive: Value, frequency_hz: Value, scale: Value) -> Value {
        use arithmetic::ScaledValue;
        ScaledValue::new(frequency_hz)
            .multiply(ScaledValue::new(core::f64::consts::TAU))
            .multiply(ScaledValue::new(reactive))
            .multiply_binary64(scale)
    }

    #[inline]
    pub fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedNoiseInjectionEvaluation {
    /// Index into the generated module's `GROUPED_NOISE_INJECTIONS` table.
    pub descriptor: usize,
    pub gain: GeneratedNoiseComplex,
}

/// Allocation-free grouped process result valid for the visitor call.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedNoiseProcessEvaluationRef<'a> {
    pub active: bool,
    pub psd: Value,
    pub exponent: Option<Value>,
    pub table_operands: &'a [Value],
    pub injections: &'a [GeneratedNoiseInjectionEvaluation],
}

pub trait GeneratedNoiseProcessVisitor {
    /// Return `false` to stop after this process.
    fn visit_process(
        &mut self,
        index: usize,
        evaluation: GeneratedNoiseProcessEvaluationRef<'_>,
    ) -> bool;
}

impl<F> GeneratedNoiseProcessVisitor for F
where
    F: for<'a> FnMut(usize, GeneratedNoiseProcessEvaluationRef<'a>) -> bool,
{
    #[inline]
    fn visit_process(
        &mut self,
        index: usize,
        evaluation: GeneratedNoiseProcessEvaluationRef<'_>,
    ) -> bool {
        self(index, evaluation)
    }
}

/// Evaluated, frequency-independent data for one generated noise primitive.
///
/// Table operands retain their canonical flat ordering. Interpretation and
/// interpolation belong to the analysis layer, not the generated-device ABI.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedNoiseEvaluation {
    pub active: bool,
    pub psd: Value,
    pub exponent: Option<Value>,
    pub table_operands: Vec<Value>,
}

/// Allocation-free view produced by a generated one-pass noise evaluator.
///
/// Table operands borrow a generated stack buffer for the duration of the
/// visitor call. Consumers that retain an evaluation can materialize the
/// existing owned representation with [`GeneratedNoiseEvaluationRef::to_owned`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedNoiseEvaluationRef<'a> {
    pub active: bool,
    pub psd: Value,
    pub exponent: Option<Value>,
    pub table_operands: &'a [Value],
}

impl GeneratedNoiseEvaluationRef<'_> {
    #[inline]
    pub fn to_owned(self) -> GeneratedNoiseEvaluation {
        GeneratedNoiseEvaluation {
            active: self.active,
            psd: self.psd,
            exponent: self.exponent,
            table_operands: self.table_operands.to_vec(),
        }
    }
}

/// Type-erased sink used by generated devices to report every noise source in
/// one dependency-schedule traversal.
pub trait GeneratedNoiseVisitor {
    /// Return `false` to stop evaluation after the current source.
    fn visit(&mut self, index: usize, evaluation: GeneratedNoiseEvaluationRef<'_>) -> bool;
}

impl<F> GeneratedNoiseVisitor for F
where
    F: for<'a> FnMut(usize, GeneratedNoiseEvaluationRef<'a>) -> bool,
{
    #[inline]
    fn visit(&mut self, index: usize, evaluation: GeneratedNoiseEvaluationRef<'_>) -> bool {
        self(index, evaluation)
    }
}

/// A generated noise evaluator rejected invalid model state or output.
#[derive(Debug, Clone, PartialEq)]
pub enum GeneratedNoiseEvaluationError {
    Evaluation {
        source: GeneratedEvaluationError,
    },
    UninitializedAnalogState,
    SourceIndexOutOfRange {
        index: usize,
        count: usize,
    },
    ProcessIndexOutOfRange {
        index: usize,
        count: usize,
    },
    ProcessIdMismatch {
        index: usize,
        process_id: usize,
    },
    InjectionIndexOutOfRange {
        process: usize,
        injection: usize,
        count: usize,
    },
    InjectionProcessMismatch {
        process: usize,
        injection: usize,
        owner: usize,
    },
    NonFinite {
        index: usize,
        quantity: &'static str,
        value: Value,
    },
    NegativePower {
        index: usize,
        value: Value,
    },
    InvalidMultiplicity {
        value: Value,
    },
    InvalidFrequency {
        value: Value,
    },
    NonFiniteGain {
        process: usize,
        injection: usize,
        re: Value,
        im: Value,
    },
    AnalogLoopLimit {
        iterations: usize,
        limit: usize,
    },
}

impl std::fmt::Display for GeneratedNoiseEvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Evaluation { source } => {
                write!(f, "generated Verilog-A noise evaluation failed: {source}")
            }
            Self::UninitializedAnalogState => {
                f.write_str("analog initialization must complete before generated noise evaluation")
            }
            Self::SourceIndexOutOfRange { index, count } => write!(
                f,
                "generated Verilog-A noise source index {index} is outside the {count}-source catalog"
            ),
            Self::ProcessIndexOutOfRange { index, count } => write!(
                f,
                "generated Verilog-A noise process index {index} is outside the {count}-process catalog"
            ),
            Self::ProcessIdMismatch { index, process_id } => write!(
                f,
                "generated Verilog-A noise process descriptor {index} declares noncanonical process ID {process_id}"
            ),
            Self::InjectionIndexOutOfRange {
                process,
                injection,
                count,
            } => write!(
                f,
                "generated Verilog-A noise process {process} references injection {injection} outside the {count}-injection catalog"
            ),
            Self::InjectionProcessMismatch {
                process,
                injection,
                owner,
            } => write!(
                f,
                "generated Verilog-A noise process {process} references injection {injection} owned by process {owner}"
            ),
            Self::NonFinite {
                index,
                quantity,
                value,
            } => write!(
                f,
                "generated Verilog-A noise source {index} produced non-finite {quantity} {value}"
            ),
            Self::NegativePower { index, value } => write!(
                f,
                "generated Verilog-A noise source {index} produced negative power {value}"
            ),
            Self::InvalidMultiplicity { value } => write!(
                f,
                "generated Verilog-A noise evaluation requires a finite positive multiplicity, found {value}"
            ),
            Self::InvalidFrequency { value } => write!(
                f,
                "generated Verilog-A grouped noise evaluation requires a finite nonnegative frequency, found {value}"
            ),
            Self::NonFiniteGain {
                process,
                injection,
                re,
                im,
            } => write!(
                f,
                "generated Verilog-A noise process {process} injection {injection} produced non-finite gain ({re}, {im})"
            ),
            Self::AnalogLoopLimit { iterations, limit } => write!(
                f,
                "generated Verilog-A noise evaluation exceeded its analog-loop limit: {iterations} iterations (limit {limit})"
            ),
        }
    }
}

impl std::error::Error for GeneratedNoiseEvaluationError {}

/// Engine-neutral location at which a generated noise source is applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedNoiseInjection {
    Current {
        node_pos: usize,
        node_neg: usize,
    },
    /// A potential-noise EMF applied to the equation for this concrete 1-based
    /// circuit branch ordinal returned by `CircuitData::allocate_branch`. This
    /// is not the descriptor's device-local branch ordinal; the analysis layer
    /// owns its conversion to a matrix axis.
    Potential {
        branch: usize,
    },
}

/// Canonical descriptor paired with topology mapped to a concrete instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedMappedNoiseDescriptor {
    pub descriptor: GeneratedNoiseDescriptor,
    pub injection: GeneratedNoiseInjection,
}

/// Grouped-process injection metadata paired with concrete instance topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedMappedNoiseInjectionDescriptor {
    pub descriptor: GeneratedNoiseInjectionDescriptor,
    pub injection: GeneratedNoiseInjection,
}

/// Generated noise topology was inconsistent with its concrete instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratedNoiseTopologyError {
    LocalNodeOutOfRange {
        endpoint: &'static str,
        local_node: usize,
        node_count: usize,
    },
    CurrentSourceHasBranch {
        branch_ordinal: usize,
    },
    PotentialSourceMissingBranch,
    BranchOrdinalOutOfRange {
        branch_ordinal: usize,
        branch_count: usize,
    },
}

impl std::fmt::Display for GeneratedNoiseTopologyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocalNodeOutOfRange {
                endpoint,
                local_node,
                node_count,
            } => write!(
                f,
                "generated Verilog-A noise {endpoint} endpoint references local node {local_node}, but the instance has {node_count} nodes"
            ),
            Self::CurrentSourceHasBranch { branch_ordinal } => write!(
                f,
                "generated Verilog-A current-noise source unexpectedly references branch ordinal {branch_ordinal}"
            ),
            Self::PotentialSourceMissingBranch => write!(
                f,
                "generated Verilog-A potential-noise source has no branch equation"
            ),
            Self::BranchOrdinalOutOfRange {
                branch_ordinal,
                branch_count,
            } => write!(
                f,
                "generated Verilog-A potential-noise source references branch ordinal {branch_ordinal}, but the instance has {branch_count} branches"
            ),
        }
    }
}

impl std::error::Error for GeneratedNoiseTopologyError {}

impl GeneratedNoiseDescriptor {
    /// Validate and map device-local topology without assuming an engine matrix
    /// layout. Ground is represented by circuit node zero.
    pub fn map_topology(
        self,
        nodes: &[usize],
        branches: &[usize],
    ) -> Result<GeneratedMappedNoiseDescriptor, GeneratedNoiseTopologyError> {
        // Validate both canonical endpoints even when a potential contribution
        // is ultimately injected on its branch-equation row. This keeps corrupt
        // generated metadata from being silently masked by injection topology.
        let node_pos = map_generated_noise_endpoint(self.pos, "positive", nodes)?;
        let node_neg = map_generated_noise_endpoint(self.neg, "negative", nodes)?;
        let injection = if self.is_current {
            if let Some(branch_ordinal) = self.branch_ordinal {
                return Err(GeneratedNoiseTopologyError::CurrentSourceHasBranch { branch_ordinal });
            }
            GeneratedNoiseInjection::Current { node_pos, node_neg }
        } else {
            let branch_ordinal = self
                .branch_ordinal
                .ok_or(GeneratedNoiseTopologyError::PotentialSourceMissingBranch)?;
            let branch = branches.get(branch_ordinal).copied().ok_or(
                GeneratedNoiseTopologyError::BranchOrdinalOutOfRange {
                    branch_ordinal,
                    branch_count: branches.len(),
                },
            )?;
            GeneratedNoiseInjection::Potential { branch }
        };
        Ok(GeneratedMappedNoiseDescriptor {
            descriptor: self,
            injection,
        })
    }
}

impl GeneratedNoiseInjectionDescriptor {
    /// Validate and map one grouped-process injection without assuming an
    /// engine matrix layout. Ground is represented by circuit node zero.
    pub fn map_topology(
        self,
        nodes: &[usize],
        branches: &[usize],
    ) -> Result<GeneratedMappedNoiseInjectionDescriptor, GeneratedNoiseTopologyError> {
        let node_pos = map_generated_noise_endpoint(self.pos, "positive", nodes)?;
        let node_neg = map_generated_noise_endpoint(self.neg, "negative", nodes)?;
        let injection = if self.is_current {
            if let Some(branch_ordinal) = self.branch_ordinal {
                return Err(GeneratedNoiseTopologyError::CurrentSourceHasBranch { branch_ordinal });
            }
            GeneratedNoiseInjection::Current { node_pos, node_neg }
        } else {
            let branch_ordinal = self
                .branch_ordinal
                .ok_or(GeneratedNoiseTopologyError::PotentialSourceMissingBranch)?;
            let branch = branches.get(branch_ordinal).copied().ok_or(
                GeneratedNoiseTopologyError::BranchOrdinalOutOfRange {
                    branch_ordinal,
                    branch_count: branches.len(),
                },
            )?;
            GeneratedNoiseInjection::Potential { branch }
        };
        Ok(GeneratedMappedNoiseInjectionDescriptor {
            descriptor: self,
            injection,
        })
    }
}

fn map_generated_noise_endpoint(
    endpoint: GeneratedNoiseEndpoint,
    endpoint_role: &'static str,
    nodes: &[usize],
) -> Result<usize, GeneratedNoiseTopologyError> {
    endpoint.local_node.map_or(Ok(0), |local_node| {
        nodes
            .get(local_node)
            .copied()
            .ok_or(GeneratedNoiseTopologyError::LocalNodeOutOfRange {
                endpoint: endpoint_role,
                local_node,
                node_count: nodes.len(),
            })
    })
}

#[derive(Debug, Clone, Copy)]
pub enum GeneratedAnalysisKind {
    Dc,
    Ac,
    Tran,
    Noise,
    Ic,
}

/// Controls whether a generated evaluation may apply Newton limiting and
/// advance per-instance limiter history. This is intentionally orthogonal to
/// the physical analysis queried through Verilog-A `$analysis(...)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedEvaluationMode {
    NewtonLimited,
    StaticProbe,
    /// Evaluate the transient model's physical static `F-B` contribution
    /// without advancing or contributing `ddt`/`idt` operators. This is used
    /// only to capture Xyce OneStep's accepted static-history vector.
    StaticDaeProbe,
    SmallSignal,
}

impl GeneratedEvaluationMode {
    #[inline]
    pub const fn default_for_analysis(analysis: GeneratedAnalysisKind) -> Self {
        match analysis {
            GeneratedAnalysisKind::Ac | GeneratedAnalysisKind::Noise => Self::SmallSignal,
            GeneratedAnalysisKind::Dc | GeneratedAnalysisKind::Tran | GeneratedAnalysisKind::Ic => {
                Self::NewtonLimited
            }
        }
    }
}

impl GeneratedAnalysisKind {
    #[inline]
    pub const fn code(self) -> u8 {
        match self {
            Self::Dc => 0,
            Self::Ac => 1,
            Self::Tran => 2,
            Self::Noise => 3,
            Self::Ic => 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GeneratedEvalContext<'a> {
    voltages: &'a [Value],
    temperature: Value,
    num_nodes: usize,
    analysis: GeneratedAnalysisKind,
    analysis_phase: AnalogAnalysisPhase,
    analysis_initial_step: bool,
    analysis_final_step: bool,
    simparams: GeneratedSimulationParameters,
    evaluation_mode: GeneratedEvaluationMode,
    record_analog_tasks: bool,
    evaluation_error: std::cell::Cell<Option<GeneratedEvaluationError>>,
}

impl<'a> GeneratedEvalContext<'a> {
    #[inline]
    pub fn new(voltages: &'a [Value], temperature: Value, num_nodes: usize) -> Self {
        Self::with_analysis(voltages, temperature, num_nodes, GeneratedAnalysisKind::Dc)
    }

    #[inline]
    pub fn with_analysis(
        voltages: &'a [Value],
        temperature: Value,
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
    ) -> Self {
        Self::with_analysis_step(voltages, temperature, num_nodes, analysis, false, false)
    }

    #[inline]
    pub fn with_analysis_step(
        voltages: &'a [Value],
        temperature: Value,
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
        initial: bool,
        final_step: bool,
    ) -> Self {
        Self::with_analysis_step_and_simparams(
            voltages,
            temperature,
            num_nodes,
            analysis,
            initial,
            final_step,
            GeneratedSimulationParameters::default(),
        )
    }

    #[inline]
    pub fn with_analysis_step_and_simparams(
        voltages: &'a [Value],
        temperature: Value,
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
        initial: bool,
        final_step: bool,
        simparams: GeneratedSimulationParameters,
    ) -> Self {
        let evaluation_mode = GeneratedEvaluationMode::default_for_analysis(analysis);
        Self::with_analysis_step_simparams_and_mode(
            voltages,
            temperature,
            num_nodes,
            analysis,
            initial,
            final_step,
            simparams,
            evaluation_mode,
        )
    }

    #[inline]
    pub fn with_analysis_step_simparams_and_mode(
        voltages: &'a [Value],
        temperature: Value,
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
        initial: bool,
        final_step: bool,
        simparams: GeneratedSimulationParameters,
        evaluation_mode: GeneratedEvaluationMode,
    ) -> Self {
        Self {
            voltages,
            temperature,
            num_nodes,
            analysis,
            analysis_initial_step: initial,
            analysis_phase: AnalogAnalysisPhase::Point,
            analysis_final_step: final_step,
            simparams,
            evaluation_mode,
            record_analog_tasks: matches!(evaluation_mode, GeneratedEvaluationMode::NewtonLimited),
            evaluation_error: std::cell::Cell::new(None),
        }
    }

    /// Set the solver phase without starting a new physical analysis.
    #[inline]
    pub fn with_analysis_phase(mut self, phase: AnalogAnalysisPhase) -> Self {
        self.analysis_phase = phase;
        self
    }

    #[inline]
    pub fn analysis_code(&self) -> u8 {
        self.analysis.code()
    }

    #[inline]
    pub fn limiting_enabled(&self) -> bool {
        matches!(self.evaluation_mode, GeneratedEvaluationMode::NewtonLimited)
    }

    /// Whether stateful model evaluation, including event controls, is enabled.
    /// Integration operators additionally check [`Self::integration_operators_enabled`].
    #[inline]
    pub fn dynamic_operators_enabled(&self) -> bool {
        !matches!(
            self.evaluation_mode,
            GeneratedEvaluationMode::StaticDaeProbe
        )
    }

    /// Whether `ddt` and `idt` may evaluate a companion and update trial history.
    /// Small-signal points freeze integration while still allowing their
    /// initial/final-step event bodies. Their preceding equilibrium solve uses
    /// the ordinary DC value and Jacobian.
    #[inline]
    pub fn integration_operators_enabled(&self) -> bool {
        self.dynamic_operators_enabled()
            && (!self.analysis_smallsig() || self.analysis_phase.is_equilibrium())
    }

    #[inline]
    pub fn report_analog_loop_limit(&self, phase: &'static str, iterations: usize, limit: usize) {
        if self.evaluation_error.get().is_none() {
            self.evaluation_error
                .set(Some(GeneratedEvaluationError::AnalogLoopLimit {
                    phase,
                    iterations,
                    limit,
                }));
        }
    }

    #[inline]
    pub fn report_ddt_candidate_error(&self, slot: usize, source: GeneratedDdtCandidateError) {
        if self.evaluation_error.get().is_none() {
            self.evaluation_error
                .set(Some(GeneratedEvaluationError::DdtCandidate {
                    slot,
                    source,
                }));
        }
    }

    #[inline]
    pub fn report_idt_candidate_error(&self, slot: usize, source: GeneratedIdtCandidateError) {
        if self.evaluation_error.get().is_none() {
            self.evaluation_error
                .set(Some(GeneratedEvaluationError::IdtCandidate {
                    slot,
                    source,
                }));
        }
    }

    #[inline]
    pub fn report_idtmod_candidate_error(
        &self,
        slot: Option<usize>,
        source: GeneratedIdtModCandidateError,
    ) {
        if self.evaluation_error.get().is_none() {
            self.evaluation_error
                .set(Some(GeneratedEvaluationError::IdtModCandidate {
                    slot,
                    source,
                }));
        }
    }

    #[inline]
    pub fn report_event_control_error(
        &self,
        operator: &'static str,
        slot: usize,
        source: GeneratedEventControlError,
    ) {
        if self.evaluation_error.get().is_none() {
            self.evaluation_error
                .set(Some(GeneratedEvaluationError::EventControl {
                    operator,
                    slot,
                    source,
                }));
        }
    }

    #[inline]
    pub fn take_evaluation_error(&self) -> Option<GeneratedEvaluationError> {
        self.evaluation_error.take()
    }

    /// Noise callbacks must not consume a pending evaluator failure or hide it
    /// behind a finite PSD selected by an invalid condition.
    #[inline]
    pub fn check_noise_evaluation(&self) -> Result<(), GeneratedNoiseEvaluationError> {
        self.evaluation_error.get().map_or(Ok(()), |source| {
            Err(GeneratedNoiseEvaluationError::Evaluation { source })
        })
    }

    /// Ordinary numerical probes do not replace or publish system-task calls.
    #[inline]
    pub fn analog_tasks_enabled(&self) -> bool {
        self.record_analog_tasks
    }

    /// Record the primal body's candidate effects at an accepted-point
    /// observation without enabling Newton limiting. Derivative and noise
    /// probes retain their default disabled policy.
    #[inline]
    pub fn with_analog_task_recording(mut self, enabled: bool) -> Self {
        self.record_analog_tasks = enabled;
        self
    }

    #[inline]
    pub fn evaluation_failed(&self) -> bool {
        self.evaluation_error.get().is_some()
    }

    #[inline]
    pub fn report_analog_task_error(&self, site: u32, source: AnalogEffectError) {
        if self.evaluation_error.get().is_none() {
            self.evaluation_error
                .set(Some(GeneratedEvaluationError::AnalogTask { site, source }));
        }
    }

    /// Retain an invalid instance-dependent `$discontinuity` operand even
    /// when the surrounding contribution remains numerically finite.
    pub fn report_discontinuity_degree_error(&self) {
        if self.evaluation_error.get().is_none() {
            self.evaluation_error
                .set(Some(GeneratedEvaluationError::DiscontinuityDegree));
        }
    }

    pub fn report_initialization_error(&self, slot: usize) {
        if self.evaluation_error.get().is_none() {
            self.evaluation_error
                .set(Some(GeneratedEvaluationError::Initialization { slot }));
        }
    }

    /// Retain derivative failures even when the result only controls a branch.
    #[inline]
    pub fn checked_derivative_value(&self, primal: Value, derivative: Value) -> Value {
        checked_derivative_value(primal, derivative).unwrap_or_else(|reason| {
            if self.evaluation_error.get().is_none() {
                self.evaluation_error
                    .set(Some(GeneratedEvaluationError::Derivative { reason }));
            }
            Value::NAN
        })
    }

    /// Validate a selected discrete input and retain failure through comparisons.
    #[inline]
    pub fn checked_discrete_value(&self, validity: Value, value: Value) -> Value {
        checked_discrete_value(validity, value).unwrap_or_else(|reason| {
            if self.evaluation_error.get().is_none() {
                self.evaluation_error
                    .set(Some(GeneratedEvaluationError::DiscreteValue { reason }));
            }
            Value::NAN
        })
    }

    /// Validate a zero-based SSA array selection before any source publication.
    pub fn checked_array_index(&self, raw: f64, len: usize, lower: i64) -> Value {
        match array_index::checked_array_slot(raw, 0, len, lower) {
            Ok(offset) => offset as f64,
            Err(error) => {
                use array_index::ArrayIndexError;
                let non_finite = matches!(error, ArrayIndexError::NonFinite { .. });
                let reason = match error {
                    ArrayIndexError::NonFinite { .. } => "index must be finite",
                    ArrayIndexError::RoundedOutOfRange { .. } => {
                        "index rounds outside the signed 64-bit range"
                    }
                    ArrayIndexError::Empty => "array is empty",
                    ArrayIndexError::OutOfBounds { .. } => "index is outside declared array bounds",
                    ArrayIndexError::SlotOverflow => "array slot arithmetic overflows",
                };
                if self.evaluation_error.get().is_none() {
                    self.evaluation_error
                        .set(Some(GeneratedEvaluationError::ArrayIndex {
                            reason,
                            non_finite,
                        }));
                }
                Value::NAN
            }
        }
    }

    /// Read a projected array from the immutable dense evaluation-input snapshot.
    /// Keep an unavailable input as an evaluation error even if its value is masked.
    pub fn checked_evaluation_input_indexed(
        &self,
        inputs: &[Value],
        index: Value,
        base: usize,
        len: usize,
    ) -> Value {
        let offset = self.checked_array_index(index, len, 0);
        if !offset.is_finite() {
            return Value::NAN;
        }
        if let Some(value) = base
            .checked_add(offset as usize)
            .and_then(|slot| inputs.get(slot))
        {
            return *value;
        }
        if self.evaluation_error.get().is_none() {
            self.evaluation_error
                .set(Some(GeneratedEvaluationError::ArrayIndex {
                    reason: "procedural evaluation-state input is unavailable",
                    non_finite: false,
                }));
        }
        Value::NAN
    }

    /// Retain integer failures even when the result only controls a branch.
    #[inline]
    pub fn integer_result(&self, result: Result<Value, integer::IntegerRuntimeError>) -> Value {
        result.unwrap_or_else(|source| {
            use integer::IntegerRuntimeError;
            let non_finite = source.is_non_finite_operand();
            let reason = match source {
                IntegerRuntimeError::NonFiniteOperand { .. } => {
                    "conversion requires a finite value"
                }
                IntegerRuntimeError::OperandOutOfRange { .. } => {
                    "conversion rounds outside the signed 32-bit range"
                }
                IntegerRuntimeError::DivisionByZero => "division by zero",
                IntegerRuntimeError::ModulusByZero => "modulus by zero",
                IntegerRuntimeError::ZeroToNegativePower { .. } => {
                    "zero raised to a negative integer exponent"
                }
            };
            if self.evaluation_error.get().is_none() {
                self.evaluation_error
                    .set(Some(GeneratedEvaluationError::Integer {
                        reason,
                        non_finite,
                    }));
            }
            Value::NAN
        })
    }

    #[inline]
    pub fn report_small_signal_error(&self, reason: &'static str) {
        if self.evaluation_error.get().is_none() {
            self.evaluation_error
                .set(Some(GeneratedEvaluationError::SmallSignal { reason }));
        }
    }

    #[inline]
    pub fn simparam_required(&self, name: &'static str) -> Value {
        self.simparams.get(name).unwrap_or_else(|| {
            if self.evaluation_error.get().is_none() {
                self.evaluation_error
                    .set(Some(GeneratedEvaluationError::SimulationParameter { name }));
            }
            Value::NAN
        })
    }

    #[inline]
    pub fn simparam_or(&self, name: &str, fallback: Value) -> Value {
        self.simparams.get(name).unwrap_or(fallback)
    }

    #[inline]
    pub fn has_simparam(&self, name: &str) -> bool {
        self.simparams.get(name).is_some()
    }

    #[inline]
    pub fn node_voltage(&self, node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            self.voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    #[inline]
    pub fn temperature(&self) -> Value {
        self.temperature
    }

    #[inline]
    pub fn thermal_voltage(&self) -> Value {
        self.temperature * THERMAL_VOLTAGE_PER_K
    }

    #[inline]
    pub fn branch_current(&self, branch_ordinal: usize) -> Value {
        if branch_ordinal == 0 {
            0.0
        } else {
            self.voltages
                .get(self.num_nodes + branch_ordinal - 1)
                .copied()
                .unwrap_or(0.0)
        }
    }

    #[inline]
    pub fn analysis(&self, query: &str) -> bool {
        analysis_query_id(query).is_some_and(|id| {
            analysis_query_mask(
                self.analysis.code(),
                self.analysis_phase,
                self.analysis_initial_step(),
                self.analysis_final_step(),
            ) & (1 << id)
                != 0
        })
    }

    #[inline]
    pub fn analysis_dc(&self) -> bool {
        matches!(self.analysis, GeneratedAnalysisKind::Dc)
    }

    #[inline]
    pub fn analysis_ac(&self) -> bool {
        matches!(self.analysis, GeneratedAnalysisKind::Ac)
    }

    #[inline]
    pub fn analysis_tran(&self) -> bool {
        matches!(self.analysis, GeneratedAnalysisKind::Tran)
    }

    #[inline]
    pub fn analysis_noise(&self) -> bool {
        matches!(self.analysis, GeneratedAnalysisKind::Noise)
    }

    #[inline]
    pub fn analysis_ic(&self) -> bool {
        matches!(self.analysis, GeneratedAnalysisKind::Ic)
            || (self.analysis_tran() && self.analysis_phase.is_equilibrium())
    }

    #[inline]
    pub fn analysis_static(&self) -> bool {
        matches!(
            self.analysis,
            GeneratedAnalysisKind::Dc | GeneratedAnalysisKind::Ic
        ) || self.analysis_phase.is_equilibrium()
    }

    #[inline]
    pub fn analysis_nodeset(&self) -> bool {
        matches!(self.analysis_phase, AnalogAnalysisPhase::Nodeset)
    }

    #[inline]
    pub fn analysis_smallsig(&self) -> bool {
        matches!(
            self.analysis,
            GeneratedAnalysisKind::Ac | GeneratedAnalysisKind::Noise
        )
    }

    #[inline]
    pub fn analysis_initial_step(&self) -> bool {
        self.dynamic_operators_enabled() && self.analysis_initial_step
    }

    #[inline]
    pub fn analysis_final_step(&self) -> bool {
        self.dynamic_operators_enabled() && self.analysis_final_step
    }
}

#[derive(Debug, Clone, Default)]
pub struct GeneratedStaticStampCache {
    node_count: usize,
    branch_count: usize,
    node_axes: Vec<Option<usize>>,
    branch_axes: Vec<Option<usize>>,
    axis_matrix_indices: Vec<Option<usize>>,
    matrix_axis_lookup: Vec<(usize, usize)>,
    slots: GeneratedStaticStampSlots,
}

#[derive(Debug, Clone, Default)]
struct GeneratedStaticStampSlots {
    logical_len: usize,
    linked_len: usize,
    dense: Vec<Option<CscIndex>>,
    entries: Vec<(usize, CscIndex)>,
}

impl GeneratedStaticStampSlots {
    const MAX_DENSE_LOGICAL_LEN: usize = 1_024;

    #[inline]
    fn clear(&mut self) {
        self.logical_len = 0;
        self.linked_len = 0;
        self.dense.clear();
        self.entries.clear();
    }

    #[inline]
    fn reset(&mut self, logical_len: usize) {
        self.logical_len = logical_len;
        self.linked_len = 0;
        self.dense.clear();
        self.entries.clear();
    }

    #[inline]
    fn push(&mut self, key: usize, index: CscIndex) {
        debug_assert!(
            self.entries
                .last()
                .is_none_or(|&(previous, _)| previous < key),
            "generated static stamp slots must be linked in key order"
        );
        self.linked_len += 1;
        self.entries.push((key, index));
    }

    fn finish(&mut self) {
        let use_dense = self.logical_len <= Self::MAX_DENSE_LOGICAL_LEN
            || self.entries.len().saturating_mul(3) >= self.logical_len;
        if !use_dense {
            return;
        }

        self.dense.resize(self.logical_len, None);
        for (key, index) in self.entries.drain(..) {
            self.dense[key] = Some(index);
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.logical_len
    }

    #[inline]
    fn get(&self, key: usize) -> Option<CscIndex> {
        if !self.dense.is_empty() {
            return self.dense.get(key).copied().flatten();
        }
        self.entries
            .binary_search_by_key(&key, |&(entry_key, _)| entry_key)
            .ok()
            .map(|position| self.entries[position].1)
    }

    #[inline]
    fn linked_len(&self) -> usize {
        self.linked_len
    }
}

impl GeneratedStaticStampCache {
    #[inline]
    pub fn link(
        &mut self,
        matrix: &StaticMatrix,
        nodes: &[usize],
        branches: &[usize],
        num_nodes: usize,
    ) {
        self.rebuild_axis_indices(nodes, branches, num_nodes);
        let width = self.axis_count();
        self.slots.reset(width * width);
        for row_axis in 0..width {
            let Some(row) = self.axis_matrix_indices[row_axis] else {
                continue;
            };
            for col_axis in 0..width {
                let Some(col) = self.axis_matrix_indices[col_axis] else {
                    continue;
                };
                if let Some(index) = matrix.get_index(row, col) {
                    self.slots.push(row_axis * width + col_axis, index);
                }
            }
        }
        self.slots.finish();
    }

    /// Number of CSC locations retained after linking.
    #[inline]
    pub fn linked_slot_count(&self) -> usize {
        self.slots.linked_len()
    }

    #[inline]
    pub fn axis_indices_match(
        &self,
        nodes: &[usize],
        branches: &[usize],
        num_nodes: usize,
    ) -> bool {
        let axis_count = nodes.len() + branches.len();
        let expected_first_branch = branches
            .first()
            .copied()
            .map(|branch| num_nodes + branch - 1);
        let first_branch_axis = nodes.len();
        let branch_index_matches = match expected_first_branch {
            Some(expected) => {
                self.axis_matrix_indices
                    .get(first_branch_axis)
                    .copied()
                    .flatten()
                    == Some(expected)
            }
            None => true,
        };
        self.axis_matrix_indices.len() == axis_count && branch_index_matches
    }

    /// Discard linked matrix locations without reallocating the surrounding
    /// cache object.
    #[inline]
    pub fn clear_linked_slots(&mut self) {
        self.slots.clear();
    }

    /// Rebuild the device-local-to-global axis mapping without linking matrix
    /// locations. The engine uses this for matrix-free evaluation paths.
    #[inline]
    pub fn rebuild_axis_indices(&mut self, nodes: &[usize], branches: &[usize], num_nodes: usize) {
        self.node_count = nodes.len();
        self.branch_count = branches.len();
        self.axis_matrix_indices.clear();
        self.axis_matrix_indices
            .reserve(self.node_count + self.branch_count);
        self.axis_matrix_indices
            .extend(nodes.iter().copied().map(Self::node_matrix_index));
        self.axis_matrix_indices.extend(
            branches
                .iter()
                .copied()
                .map(|branch| Self::branch_matrix_index(num_nodes, branch)),
        );

        self.node_axes.clear();
        self.node_axes.reserve(self.node_count);
        self.node_axes.extend(
            self.axis_matrix_indices
                .iter()
                .take(self.node_count)
                .enumerate()
                .map(|(axis, matrix_index)| matrix_index.is_some().then_some(axis)),
        );

        self.branch_axes.clear();
        self.branch_axes.reserve(self.branch_count);
        self.branch_axes.extend(
            self.axis_matrix_indices
                .iter()
                .skip(self.node_count)
                .take(self.branch_count)
                .enumerate()
                .map(|(branch, matrix_index)| {
                    matrix_index.is_some().then_some(self.node_count + branch)
                }),
        );

        self.matrix_axis_lookup.clear();
        self.matrix_axis_lookup.extend(
            self.axis_matrix_indices
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(axis, matrix_index)| matrix_index.map(|index| (index, axis))),
        );
        self.matrix_axis_lookup
            .sort_unstable_by_key(|&(matrix_index, axis)| (matrix_index, axis));
    }

    #[inline]
    fn axis_count(&self) -> usize {
        self.node_count + self.branch_count
    }

    #[inline]
    fn node_axis(&self, node_index: usize) -> Option<usize> {
        self.node_axes.get(node_index).copied().flatten()
    }

    #[inline]
    fn branch_axis(&self, branch_index: usize) -> Option<usize> {
        self.branch_axes.get(branch_index).copied().flatten()
    }

    #[inline]
    fn axis_matrix_index(&self, axis: usize) -> Option<usize> {
        self.axis_matrix_indices.get(axis).copied().flatten()
    }

    #[inline]
    fn slot_for_axes(&self, row_axis: usize, col_axis: usize) -> Option<CscIndex> {
        let width = self.axis_count();
        if width == 0 || self.slots.len() != width * width {
            return None;
        }
        self.slots
            .get(row_axis.checked_mul(width)?.checked_add(col_axis)?)
    }

    #[inline]
    fn slot_for_matrix_indices(&self, row: usize, col: usize) -> Option<CscIndex> {
        let row_axis = self.axis_for_matrix_index(row)?;
        let col_axis = self.axis_for_matrix_index(col)?;
        self.slot_for_axes(row_axis, col_axis)
    }

    #[inline]
    fn axis_for_matrix_index(&self, matrix_index: usize) -> Option<usize> {
        self.matrix_axis_lookup
            .binary_search_by_key(&matrix_index, |&(index, _axis)| index)
            .ok()
            .map(|idx| self.matrix_axis_lookup[idx].1)
    }

    #[inline]
    fn node_matrix_index(node: usize) -> Option<usize> {
        if node > 0 { Some(node - 1) } else { None }
    }

    #[inline]
    fn branch_matrix_index(num_nodes: usize, branch_ordinal: usize) -> Option<usize> {
        if branch_ordinal > 0 {
            Some(num_nodes + branch_ordinal - 1)
        } else {
            None
        }
    }
}

enum GeneratedMatrixTarget<'a> {
    Static { matrix: &'a mut StaticMatrix },
    AcReal { matrix: &'a mut ComplexMatrix },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedDerivative {
    axis: GeneratedDerivativeAxis,
    value: Value,
}

impl GeneratedDerivative {
    #[inline]
    pub const fn node(node: usize, value: Value) -> Self {
        Self {
            axis: GeneratedDerivativeAxis::Node(node),
            value,
        }
    }

    #[inline]
    pub const fn branch(branch_ordinal: usize, value: Value) -> Self {
        Self {
            axis: GeneratedDerivativeAxis::Branch(branch_ordinal),
            value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GeneratedDerivativeAxis {
    Node(usize),
    Branch(usize),
}

pub struct GeneratedStamper<'a> {
    matrix: GeneratedMatrixTarget<'a>,
    cache: &'a GeneratedStaticStampCache,
    rhs: Option<&'a mut [Value]>,
    /// Device-local terminal currents accumulated from the exact flow
    /// contributions emitted by generated code. The slice may cover only the
    /// external-node prefix; contributions involving later internal nodes are
    /// deliberately ignored while their external endpoint is still recorded.
    terminal_currents: Option<&'a mut [Value]>,
    voltages: &'a [Value],
    num_nodes: usize,
    /// A non-finite value reached the Jacobian.
    ///
    /// Accumulated rather than raised: an early exit would leave the rest of
    /// the device's contributions out of a system the caller is about to throw
    /// away anyway, and would make the observation depend on the order the
    /// generated code happens to emit its equations in. One `is_finite` and
    /// one `|=` per written value is the same audit the runtime route applies
    /// in `finite_stamp_value`, and it is not feature-gated there either.
    saw_non_finite_matrix: bool,
    /// A non-finite value reached the right-hand side. Separate from the
    /// Jacobian flag because the two sinks fail for different reasons: an
    /// equivalent source carries `value - J*x`, so it can be non-finite from
    /// an unknown the solver handed back as well as from the device's own
    /// arithmetic.
    saw_non_finite_rhs: bool,
}

impl<'a> GeneratedStamper<'a> {
    /// Which sink, if any, received a value that was not finite.
    ///
    /// `None` means every value this stamper wrote was finite.
    #[inline]
    pub fn non_finite_contribution(&self) -> Option<&'static str> {
        match (self.saw_non_finite_matrix, self.saw_non_finite_rhs) {
            (false, false) => None,
            (true, false) => Some("Jacobian entry"),
            (false, true) => Some("right-hand-side contribution"),
            (true, true) => Some("Jacobian entry and right-hand-side contribution"),
        }
    }

    #[inline]
    fn observe_matrix_value(&mut self, value: Value) {
        self.saw_non_finite_matrix |= !value.is_finite();
    }

    #[inline]
    fn observe_rhs_value(&mut self, value: Value) {
        self.saw_non_finite_rhs |= !value.is_finite();
    }

    #[inline]
    pub fn new_with_static_cache(
        matrix: &'a mut StaticMatrix,
        rhs: &'a mut [Value],
        voltages: &'a [Value],
        num_nodes: usize,
        cache: &'a GeneratedStaticStampCache,
    ) -> Self {
        Self {
            matrix: GeneratedMatrixTarget::Static { matrix },
            cache,
            rhs: Some(rhs),
            terminal_currents: None,
            voltages,
            num_nodes,
            saw_non_finite_matrix: false,
            saw_non_finite_rhs: false,
        }
    }

    /// Construct a static stamper that also observes exact device-local
    /// terminal currents.
    ///
    /// Generated contributions use local node ordinals, which preserves the
    /// identity of shorted terminals and avoids trying to infer per-terminal
    /// current from an assembled nodal row. `terminal_currents` is reset by
    /// the caller and accumulated in the same sign convention as Verilog-A
    /// flow: positive current enters the contribution's positive endpoint.
    #[inline]
    pub fn new_with_static_cache_and_terminal_currents(
        matrix: &'a mut StaticMatrix,
        rhs: &'a mut [Value],
        voltages: &'a [Value],
        num_nodes: usize,
        cache: &'a GeneratedStaticStampCache,
        terminal_currents: &'a mut [Value],
    ) -> Self {
        Self {
            matrix: GeneratedMatrixTarget::Static { matrix },
            cache,
            rhs: Some(rhs),
            terminal_currents: Some(terminal_currents),
            voltages,
            num_nodes,
            saw_non_finite_matrix: false,
            saw_non_finite_rhs: false,
        }
    }

    #[inline]
    pub fn new_ac_real_with_static_cache(
        matrix: &'a mut ComplexMatrix,
        voltages: &'a [Value],
        num_nodes: usize,
        cache: &'a GeneratedStaticStampCache,
    ) -> Self {
        Self {
            matrix: GeneratedMatrixTarget::AcReal { matrix },
            cache,
            rhs: None,
            terminal_currents: None,
            voltages,
            num_nodes,
            saw_non_finite_matrix: false,
            saw_non_finite_rhs: false,
        }
    }

    #[inline(always)]
    pub fn stamp_current_sparse_local<const NODE_COUNT: usize, const BRANCH_COUNT: usize>(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node_derivative_indices: [usize; NODE_COUNT],
        node_derivatives: [Value; NODE_COUNT],
        branch_derivative_indices: [usize; BRANCH_COUNT],
        branch_derivatives: [Value; BRANCH_COUNT],
        derivative_scale: Value,
    ) {
        self.observe_terminal_current_pair(pos, neg, value);
        let cache = self.cache;
        let pos_axis = pos.and_then(|node| cache.node_axis(node));
        let neg_axis = neg.and_then(|node| cache.node_axis(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        let width = cache.axis_count();
        let slots_ready = width != 0 && cache.slots.len() == width * width;
        for (node, derivative) in node_derivative_indices
            .iter()
            .copied()
            .zip(node_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.node_axis(node) else {
                continue;
            };
            if needs_rhs {
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            }
            self.add_current_derivative_axis_pair_cached(
                cache,
                slots_ready,
                width,
                pos_axis,
                neg_axis,
                col_axis,
                derivative,
            );
        }
        for (branch, derivative) in branch_derivative_indices
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.branch_axis(branch) else {
                continue;
            };
            if needs_rhs {
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            }
            self.add_current_derivative_axis_pair_cached(
                cache,
                slots_ready,
                width,
                pos_axis,
                neg_axis,
                col_axis,
                derivative,
            );
        }

        if needs_rhs {
            self.add_current_rhs_axis_pair_cached(cache, pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    fn add_current_rhs_pair(
        &mut self,
        pos_row: Option<usize>,
        neg_row: Option<usize>,
        equivalent: Value,
    ) {
        self.observe_rhs_value(equivalent);
        if let Some(rhs) = &mut self.rhs {
            if let Some(row) = pos_row
                && let Some(slot) = rhs.get_mut(row)
            {
                *slot -= equivalent;
            }
            if let Some(row) = neg_row
                && let Some(slot) = rhs.get_mut(row)
            {
                *slot += equivalent;
            }
        }
    }

    #[inline]
    pub fn stamp_potential_branch_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch_index: usize,
        multiplicity: Value,
    ) {
        self.stamp_branch_current_local(pos, neg, branch_index, multiplicity);
        let Some(branch_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        if let Some(pos_axis) = pos.and_then(|node| self.node_axis_local(node)) {
            self.add_real_axis(branch_axis, pos_axis, 1.0);
        }
        if let Some(neg_axis) = neg.and_then(|node| self.node_axis_local(node)) {
            self.add_real_axis(branch_axis, neg_axis, -1.0);
        }
    }

    /// Couple a solver-owned source current into KCL without imposing a
    /// voltage relation on its constraint row. Used by indirect contributions.
    #[inline]
    pub fn stamp_branch_current_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch_index: usize,
        multiplicity: Value,
    ) {
        let Some(branch_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let branch_current = multiplicity * self.branch_value_local(branch_index);
        self.observe_terminal_current_pair(pos, neg, branch_current);
        if let Some(pos_axis) = pos.and_then(|node| self.node_axis_local(node)) {
            self.add_real_axis(pos_axis, branch_axis, multiplicity);
        }
        if let Some(neg_axis) = neg.and_then(|node| self.node_axis_local(node)) {
            self.add_real_axis(neg_axis, branch_axis, -multiplicity);
        }
    }

    /// Keep an allocated but inactive potential-source unknown nonsingular
    /// without coupling it to either endpoint. The unit diagonal constrains the
    /// unused branch current to zero. It is intentionally not scaled by device
    /// multiplicity: this is a solver identity row, not a physical contribution.
    #[inline]
    pub fn stamp_inactive_potential_branch(&mut self, branch_ordinal: usize) {
        if let Some(branch) = self.branch_matrix_index(branch_ordinal) {
            self.add_real(branch, branch, 1.0);
        }
    }

    /// Local-index counterpart of [`Self::stamp_inactive_potential_branch`]
    /// for generated devices linked through a static stamp cache.
    #[inline]
    pub fn stamp_inactive_potential_branch_local(&mut self, branch_index: usize) {
        if let Some(branch_axis) = self.branch_axis_local(branch_index) {
            self.add_real_axis(branch_axis, branch_axis, 1.0);
        }
    }

    #[inline(always)]
    pub fn stamp_potential_sparse_local<const NODE_COUNT: usize, const BRANCH_COUNT: usize>(
        &mut self,
        branch_index: usize,
        value: Value,
        node_derivative_indices: [usize; NODE_COUNT],
        node_derivatives: [Value; NODE_COUNT],
        branch_derivative_indices: [usize; BRANCH_COUNT],
        branch_derivatives: [Value; BRANCH_COUNT],
    ) {
        let cache = self.cache;
        let Some(row_axis) = cache.branch_axis(branch_index) else {
            return;
        };
        let mut equivalent = value;
        let width = cache.axis_count();
        let slots_ready = width != 0 && cache.slots.len() == width * width;
        for (node, derivative) in node_derivative_indices
            .iter()
            .copied()
            .zip(node_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.node_axis(node) else {
                continue;
            };
            equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, -derivative);
        }
        for (branch, derivative) in branch_derivative_indices
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.branch_axis(branch) else {
                continue;
            };
            equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, -derivative);
        }
        self.add_potential_rhs_axis_cached(cache, row_axis, equivalent);
    }

    #[inline]
    fn add_potential_rhs(&mut self, row: usize, equivalent: Value) {
        self.observe_rhs_value(equivalent);
        if equivalent == 0.0 {
            return;
        }
        if let Some(rhs) = &mut self.rhs
            && let Some(slot) = rhs.get_mut(row)
        {
            *slot += equivalent;
        }
    }

    #[inline]
    fn observe_terminal_current_pair(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
    ) {
        let Some(currents) = &mut self.terminal_currents else {
            return;
        };
        if let Some(slot) = pos.and_then(|node| currents.get_mut(node)) {
            *slot += value;
        }
        if let Some(slot) = neg.and_then(|node| currents.get_mut(node)) {
            *slot -= value;
        }
    }

    #[inline]
    fn branch_value_local(&self, branch_index: usize) -> Value {
        self.branch_axis_local(branch_index)
            .and_then(|axis| self.axis_matrix_index_local(axis))
            .and_then(|index| self.voltages.get(index).copied())
            .unwrap_or(0.0)
    }

    #[inline]
    fn branch_matrix_index(&self, branch_ordinal: usize) -> Option<usize> {
        if branch_ordinal > 0 {
            Some(self.num_nodes + branch_ordinal - 1)
        } else {
            None
        }
    }

    #[inline]
    fn node_axis_local(&self, node_index: usize) -> Option<usize> {
        self.cache.node_axis(node_index)
    }

    #[inline]
    fn branch_axis_local(&self, branch_index: usize) -> Option<usize> {
        self.cache.branch_axis(branch_index)
    }

    #[inline]
    fn axis_matrix_index_local(&self, axis: usize) -> Option<usize> {
        self.cache.axis_matrix_index(axis)
    }

    #[inline]
    fn axis_value_cached(&self, cache: &GeneratedStaticStampCache, axis: usize) -> Value {
        cache
            .axis_matrix_index(axis)
            .and_then(|index| self.voltages.get(index).copied())
            .unwrap_or(0.0)
    }

    #[inline]
    fn add_current_derivative_axis_pair_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        slots_ready: bool,
        width: usize,
        pos_axis: Option<usize>,
        neg_axis: Option<usize>,
        col_axis: usize,
        derivative: Value,
    ) {
        if derivative == 0.0 {
            return;
        }
        if let Some(row_axis) = pos_axis {
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, derivative);
        }
        if let Some(row_axis) = neg_axis {
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, -derivative);
        }
    }

    #[inline]
    fn add_current_rhs_axis_pair_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        pos_axis: Option<usize>,
        neg_axis: Option<usize>,
        equivalent: Value,
    ) {
        let pos_row = pos_axis.and_then(|axis| cache.axis_matrix_index(axis));
        let neg_row = neg_axis.and_then(|axis| cache.axis_matrix_index(axis));
        self.add_current_rhs_pair(pos_row, neg_row, equivalent);
    }

    #[inline]
    fn add_potential_rhs_axis_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        row_axis: usize,
        equivalent: Value,
    ) {
        if let Some(row) = cache.axis_matrix_index(row_axis) {
            self.add_potential_rhs(row, equivalent);
        }
    }

    #[inline]
    fn add_real_axis_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        slots_ready: bool,
        width: usize,
        row_axis: usize,
        col_axis: usize,
        value: Value,
    ) {
        self.observe_matrix_value(value);
        if value == 0.0 {
            return;
        }
        let slot = if slots_ready {
            debug_assert!(row_axis < width);
            debug_assert!(col_axis < width);
            cache.slots.get(row_axis * width + col_axis)
        } else {
            None
        };
        if let Some(index) = slot {
            match &mut self.matrix {
                GeneratedMatrixTarget::Static { matrix } => matrix.stamp_direct(index, value),
                GeneratedMatrixTarget::AcReal { matrix } => matrix.stamp_direct_real(index, value),
            }
            return;
        }

        let Some(row) = cache.axis_matrix_index(row_axis) else {
            return;
        };
        let Some(col) = cache.axis_matrix_index(col_axis) else {
            return;
        };
        match &mut self.matrix {
            GeneratedMatrixTarget::Static { matrix } => matrix.add(row, col, value),
            GeneratedMatrixTarget::AcReal { matrix } => matrix.add_real(row, col, value),
        }
    }

    #[inline]
    fn add_real_axis(&mut self, row_axis: usize, col_axis: usize, value: Value) {
        self.observe_matrix_value(value);
        if value == 0.0 {
            return;
        }
        let cache = self.cache;
        let width = cache.axis_count();
        let slots_ready = width != 0 && cache.slots.len() == width * width;
        self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, value);
    }

    #[inline]
    fn add_real(&mut self, row: usize, col: usize, value: Value) {
        self.observe_matrix_value(value);
        if value == 0.0 {
            return;
        }
        match &mut self.matrix {
            GeneratedMatrixTarget::Static { matrix } => {
                if let Some(index) = self.cache.slot_for_matrix_indices(row, col) {
                    matrix.stamp_direct(index, value);
                } else {
                    matrix.add(row, col, value);
                }
            }
            GeneratedMatrixTarget::AcReal { matrix } => matrix.add_real(row, col, value),
        }
    }
}

pub struct GeneratedReactiveStamper<'a> {
    matrix: &'a mut ComplexMatrix,
    cache: &'a GeneratedStaticStampCache,
    nodes: &'a [usize],
    branches: &'a [usize],
    num_nodes: usize,
    omega: Value,
}

impl<'a> GeneratedReactiveStamper<'a> {
    /// The real or imaginary component of
    /// coefficient * scale * (jω)^ddt * (1/jω)^idt. Operator orders are
    /// retained separately so an integrator at zero frequency is diagnosed
    /// even when a surrounding derivative cancels it.
    ///
    /// Compose the frequency action and instance scale before the final
    /// binary64 conversion. Neither partial product is a publication boundary.
    #[inline]
    pub fn scaled_frequency_coefficient(
        &self,
        ctx: &GeneratedEvalContext<'_>,
        coefficient: Value,
        scale: Value,
        ddt: u32,
        idt: u32,
    ) -> Option<Value> {
        if ctx.evaluation_failed() {
            return None;
        }
        let error = if !self.omega.is_finite() || self.omega < 0.0 {
            Some("angular frequency must be finite and nonnegative")
        } else if self.omega == 0.0 && idt > 0 {
            Some("idt small-signal transfer is singular at zero frequency")
        } else if !coefficient.is_finite() {
            Some("non-finite frequency coefficient")
        } else if !scale.is_finite() {
            Some("non-finite frequency scaling factor")
        } else {
            None
        };
        if let Some(error) = error {
            ctx.report_small_signal_error(error);
            return None;
        }
        let exponent = i64::from(ddt) - i64::from(idt);
        let mut value = Self::scale_frequency_coefficient(coefficient, scale, self.omega, exponent);
        if !value.is_finite() {
            ctx.report_small_signal_error("frequency response overflows");
            return None;
        }
        if exponent.rem_euclid(4) >= 2 {
            value = -value;
        }
        Some(value)
    }

    #[inline]
    fn scale_frequency_coefficient(
        value: Value,
        scale: Value,
        omega: Value,
        exponent: i64,
    ) -> Value {
        if exponent == 0 || value == 0.0 || scale == 0.0 {
            return value * scale;
        }
        if omega == 0.0 {
            // Negative powers were rejected before reaching this helper.
            return 0.0_f64.copysign(value * scale);
        }
        let partial = match exponent {
            1 => value * omega,
            -1 => value / omega,
            _ => {
                let Ok(power) = i32::try_from(exponent) else {
                    return Self::wide_frequency_coefficient(value, scale, omega, exponent);
                };
                let factor = omega.powi(power);
                if !factor.is_normal() {
                    return Self::wide_frequency_coefficient(value, scale, omega, exponent);
                }
                value * factor
            }
        };
        let result = partial * scale;
        if partial.is_normal() && result.is_normal() {
            result
        } else {
            Self::wide_frequency_coefficient(value, scale, omega, exponent)
        }
    }

    #[cold]
    #[inline(never)]
    fn wide_frequency_coefficient(
        value: Value,
        scale: Value,
        omega: Value,
        exponent: i64,
    ) -> Value {
        use arithmetic::ScaledValue;
        // A difference of two u32 orders always has magnitude at most u32::MAX.
        let power = ScaledValue::new(omega).powu(exponent.unsigned_abs() as u32);
        let coefficient = ScaledValue::new(value);
        let value = if exponent < 0 {
            coefficient.divide(power)
        } else {
            coefficient.multiply(power)
        };
        value.multiply_binary64(scale)
    }

    /// One dynamic current Jacobian entry. REAL selects the matrix component;
    /// the caller has already composed the frequency action and multiplicity.
    #[inline]
    pub fn stamp_current_frequency_local<const REAL: bool>(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        derivative: GeneratedDerivative,
    ) {
        if derivative.value == 0.0 {
            return;
        }
        let pos = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos == neg {
            return;
        }
        let Some(col) = self.axis_matrix_index_local(derivative.axis) else {
            return;
        };
        if let Some(row) = pos {
            self.add_frequency::<REAL>(row, col, derivative.value);
        }
        if let Some(row) = neg {
            self.add_frequency::<REAL>(row, col, -derivative.value);
        }
    }

    /// Dynamic potential-equation derivative, excluding the structural branch
    /// incidence that the real operating-point stamp has already installed.
    #[inline]
    pub fn stamp_potential_frequency_local<const REAL: bool>(
        &mut self,
        branch: usize,
        derivative: GeneratedDerivative,
    ) {
        if derivative.value == 0.0 {
            return;
        }
        if let (Some(row), Some(col)) = (
            self.branch_matrix_index_local(branch),
            self.axis_matrix_index_local(derivative.axis),
        ) {
            self.add_frequency::<REAL>(row, col, -derivative.value);
        }
    }

    #[inline]
    fn add_frequency<const REAL: bool>(&mut self, row: usize, col: usize, value: Value) {
        if let Some(index) = self.cache.slot_for_matrix_indices(row, col) {
            if REAL {
                self.matrix.stamp_direct_real(index, value);
            } else {
                self.matrix.stamp_direct_imag(index, value);
            }
        } else if REAL {
            self.matrix.add_real(row, col, value);
        } else {
            self.matrix.add_imag(row, col, value);
        }
    }

    #[inline]
    pub fn new_with_local_maps_and_static_cache(
        matrix: &'a mut ComplexMatrix,
        nodes: &'a [usize],
        branches: &'a [usize],
        num_nodes: usize,
        omega: Value,
        cache: &'a GeneratedStaticStampCache,
    ) -> Self {
        Self {
            matrix,
            cache,
            nodes,
            branches,
            num_nodes,
            omega,
        }
    }

    #[inline]
    pub fn stamp_current_reactive_indexed_dense_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        let cache = self.cache;
        let pos_axis = pos.and_then(|node| cache.node_axis(node));
        let neg_axis = neg.and_then(|node| cache.node_axis(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }

        let derivative_scale = arithmetic::ScaledValue::new(self.omega)
            .multiply(arithmetic::ScaledValue::new(derivative_scale));
        let width = cache.axis_count();
        let slots_ready = width != 0 && cache.slots.len() == width * width;
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            let derivative = derivative_scale.multiply_binary64(derivative);
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.node_axis(node) else {
                continue;
            };
            self.add_current_reactive_derivative_axis_pair_cached(
                cache,
                slots_ready,
                width,
                pos_axis,
                neg_axis,
                col_axis,
                derivative,
            );
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            let derivative = derivative_scale.multiply_binary64(derivative);
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.branch_axis(branch) else {
                continue;
            };
            self.add_current_reactive_derivative_axis_pair_cached(
                cache,
                slots_ready,
                width,
                pos_axis,
                neg_axis,
                col_axis,
                derivative,
            );
        }
    }

    #[inline]
    fn add_current_reactive_derivative_axis_pair_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        slots_ready: bool,
        width: usize,
        pos_axis: Option<usize>,
        neg_axis: Option<usize>,
        col_axis: usize,
        derivative: Value,
    ) {
        if derivative == 0.0 {
            return;
        }
        if let Some(row_axis) = pos_axis {
            self.add_imag_axis_cached(cache, slots_ready, width, row_axis, col_axis, derivative);
        }
        if let Some(row_axis) = neg_axis {
            self.add_imag_axis_cached(cache, slots_ready, width, row_axis, col_axis, -derivative);
        }
    }

    #[inline]
    fn add_imag_axis_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        slots_ready: bool,
        width: usize,
        row_axis: usize,
        col_axis: usize,
        value: Value,
    ) {
        if value == 0.0 {
            return;
        }
        let slot = if slots_ready {
            debug_assert!(row_axis < width);
            debug_assert!(col_axis < width);
            cache.slots.get(row_axis * width + col_axis)
        } else {
            None
        };
        if let Some(index) = slot {
            self.matrix.stamp_direct_imag(index, value);
            return;
        }

        let Some(row) = cache.axis_matrix_index(row_axis) else {
            return;
        };
        let Some(col) = cache.axis_matrix_index(col_axis) else {
            return;
        };
        self.matrix.add_imag(row, col, value);
    }

    #[inline]
    pub fn stamp_potential_reactive_indexed_dense_local(
        &mut self,
        branch: usize,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
    ) {
        let cache = self.cache;
        let Some(row_axis) = cache.branch_axis(branch) else {
            return;
        };
        let width = cache.axis_count();
        let slots_ready = width != 0 && cache.slots.len() == width * width;
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.node_axis(node) else {
                continue;
            };
            self.add_imag_axis_cached(
                cache,
                slots_ready,
                width,
                row_axis,
                col_axis,
                -self.omega * derivative,
            );
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.branch_axis(branch) else {
                continue;
            };
            self.add_imag_axis_cached(
                cache,
                slots_ready,
                width,
                row_axis,
                col_axis,
                -self.omega * derivative,
            );
        }
    }

    #[inline]
    fn branch_matrix_index(&self, branch_ordinal: usize) -> Option<usize> {
        if branch_ordinal > 0 {
            Some(self.num_nodes + branch_ordinal - 1)
        } else {
            None
        }
    }

    #[inline]
    fn node_matrix_index(node: usize) -> Option<usize> {
        if node > 0 { Some(node - 1) } else { None }
    }

    #[inline]
    fn node_matrix_index_local(&self, node: usize) -> Option<usize> {
        self.nodes
            .get(node)
            .copied()
            .and_then(Self::node_matrix_index)
    }

    #[inline]
    fn branch_matrix_index_local(&self, branch: usize) -> Option<usize> {
        self.branches
            .get(branch)
            .copied()
            .and_then(|branch| self.branch_matrix_index(branch))
    }

    #[inline]
    fn axis_matrix_index_local(&self, axis: GeneratedDerivativeAxis) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => self.node_matrix_index_local(node),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_matrix_index_local(branch),
        }
    }
}

#[cfg(test)]
mod fixed_lane_tests {
    use super::*;

    #[test]
    fn noise_transfers_preserve_intermediate_range_and_zero_axes() {
        let tau = core::f64::consts::TAU;
        for (reactive, frequency, scale, expected) in [
            (1e200, 1e200, 1e-100, tau * 1e300),
            (1e-200, 1e-200, 1e100, tau * 1e-300),
            (1e200, 1e-200, 1e200, tau * 1e200),
            (1e-200, 1e200, 1e-200, tau * 1e-200),
            (0.1, 1e308, 1.0, tau * 1e307),
            (
                1e308,
                f64::from_bits(1),
                1.0,
                f64::from_bits(0x3ceb_f60a_d470_bc60),
            ),
        ] {
            for sign in [-1.0, 1.0] {
                let actual =
                    GeneratedNoiseComplex::scaled_transfer(0.25, reactive, frequency, sign * scale);
                assert_eq!(actual.re, 0.25 * sign * scale);
                assert!(
                    (actual.im / (sign * expected) - 1.0).abs() <= 8.0 * f64::EPSILON,
                    "{reactive}, {frequency}, {scale}, {sign}: {actual:?}, expected {expected}"
                );
            }
        }
        for reactive in [0.0_f64, -0.0] {
            for scale in [1.0_f64, -1.0] {
                let actual = GeneratedNoiseComplex::scaled_transfer(2.0, reactive, f64::MAX, scale);
                assert_eq!(actual.re, 2.0 * scale);
                assert_eq!(actual.im, 0.0);
                assert_eq!(
                    actual.im.is_sign_negative(),
                    reactive.is_sign_negative() ^ scale.is_sign_negative()
                );
            }
        }
        for frequency in [0.0_f64, -0.0] {
            let actual = GeneratedNoiseComplex::scaled_transfer(1.0, -2.0, frequency, 3.0);
            assert_eq!(actual.im.to_bits(), (-frequency).to_bits());
        }
        for (real, reactive, frequency, scale) in [
            (f64::NAN, 1.0, 1.0, 1.0),
            (0.0, 0.0, f64::INFINITY, 1.0),
            (0.0, 1.0, 1.0, f64::NAN),
            (0.0, 1e200, 1e200, 1.0),
        ] {
            assert!(
                !GeneratedNoiseComplex::scaled_transfer(real, reactive, frequency, scale)
                    .is_finite()
            );
        }
    }

    #[test]
    fn extrema_keep_selected_operand_bits() {
        let nan = f64::from_bits(0x7ff8_0000_0000_0011);
        for (left, right, minimum, maximum) in [
            (1.0, 2.0, 1.0, 2.0),
            (2.0, 1.0, 1.0, 2.0),
            (nan, 2.0, 2.0, 2.0),
            (2.0, nan, 2.0, 2.0),
            (f64::NAN, nan, nan, nan),
            (-0.0, 0.0, -0.0, -0.0),
            (0.0, -0.0, 0.0, 0.0),
            (
                f64::NEG_INFINITY,
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::INFINITY,
            ),
        ] {
            assert_eq!(rspice_min(left, right).to_bits(), minimum.to_bits());
            assert_eq!(rspice_max(left, right).to_bits(), maximum.to_bits());
        }
    }

    #[test]
    fn frequency_coefficients_keep_multiplicity_inside_the_range_boundary() {
        let structure = StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).unwrap();
        let mut matrix = ComplexMatrix::from_real_structure(&structure);
        let cache = GeneratedStaticStampCache::default();
        for (coefficient, scale, omega, ddt, idt, expected) in [
            (1e200, 1e-200, 1e200, 1, 0, 1e200),
            (1e-200, 1e200, 1e-200, 1, 0, 1e-200),
            (1e200, 1e200, 1e-200, 1, 0, 1e200),
            (1e-200, 1e-200, 1e200, 1, 0, 1e-200),
            (1e200, 1e-300, 1e200, 2, 0, -1e300),
            (1e-200, 1e300, 1e-200, 2, 0, -1e-300),
            (1e200, 1e-200, 1e-200, 0, 1, -1e200),
            (1e-200, 1e200, 1e200, 0, 1, -1e-200),
            (2.0, 3.0, 1.0, u32::MAX, 0, -6.0),
            (2.0, 3.0, 1.0, 0, u32::MAX, 6.0),
        ] {
            let ctx =
                GeneratedEvalContext::with_analysis(&[0.0], 300.15, 1, GeneratedAnalysisKind::Ac);
            let stamper = GeneratedReactiveStamper::new_with_local_maps_and_static_cache(
                &mut matrix,
                &[],
                &[],
                1,
                omega,
                &cache,
            );
            let actual = stamper
                .scaled_frequency_coefficient(&ctx, coefficient, scale, ddt, idt)
                .expect("the fully scaled coefficient is finite");
            assert!(
                (actual / expected - 1.0).abs() <= 8.0 * f64::EPSILON,
                "{coefficient} * {scale} * (j{omega})^({ddt}-{idt}): {actual} != {expected}"
            );
            assert!(!ctx.evaluation_failed());
        }
        for (scale, omega, ddt, idt) in [
            (f64::NAN, 1.0, 1, 0),
            (f64::INFINITY, 1.0, 1, 0),
            (0.0, 0.0, 1, 1),
            (1e200, 1e200, 2, 0),
        ] {
            let ctx =
                GeneratedEvalContext::with_analysis(&[0.0], 300.15, 1, GeneratedAnalysisKind::Ac);
            assert!(
                GeneratedReactiveStamper::new_with_local_maps_and_static_cache(
                    &mut matrix,
                    &[],
                    &[],
                    1,
                    omega,
                    &cache
                )
                .scaled_frequency_coefficient(&ctx, 1.0, scale, ddt, idt)
                .is_none()
            );
            assert!(matches!(
                ctx.take_evaluation_error(),
                Some(GeneratedEvaluationError::SmallSignal { .. })
            ));
        }
    }

    #[test]
    fn reactive_dense_stamps_preserve_frequency_scale_range_for_every_mapping() {
        let entries = (0..3)
            .flat_map(|row| (0..3).map(move |col| (row, col, 0.0)))
            .collect::<Vec<_>>();
        let structure = StaticMatrix::from_triplets(3, 3, &entries).unwrap();
        let nodes = [1, 2];
        let branches = [1];
        let mut cache = GeneratedStaticStampCache::default();
        cache.link(&structure, &nodes, &branches, 2);
        for (omega, scale, derivative, expected) in [
            (1e200, 1e200, 1e-200, 1e200),
            (1e-200, 1e-200, 1e200, 1e-200),
            (1e200, 1e-200, 1e200, 1e200),
            (1e-200, 1e200, 1e-200, 1e-200),
            (1e200, 1e200, 0.0, 0.0),
        ] {
            let mut matrix = ComplexMatrix::from_real_structure(&structure);
            let mut stamper = GeneratedReactiveStamper::new_with_local_maps_and_static_cache(
                &mut matrix,
                &nodes,
                &branches,
                2,
                omega,
                &cache,
            );
            stamper.stamp_current_reactive_indexed_dense_local(
                Some(0),
                Some(1),
                &[0],
                &[derivative],
                &[0],
                &[-0.5 * derivative],
                scale,
            );
            let values = matrix.to_dense_imag();
            for (row, col, sign) in [(0, 0, 1.0), (1, 0, -1.0), (0, 2, -0.5), (1, 2, 0.5)] {
                let actual = values[row][col];
                if expected == 0.0 {
                    assert_eq!(actual, 0.0);
                } else {
                    assert!(
                        (actual / (sign * expected) - 1.0).abs() <= 8.0 * f64::EPSILON,
                        "{values:?}, expected={expected}"
                    );
                }
            }
        }
    }

    #[test]
    fn frequency_coefficients_preserve_phase_and_extreme_scaled_results() {
        let structure = StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).unwrap();
        let mut matrix = ComplexMatrix::from_real_structure(&structure);
        let cache = GeneratedStaticStampCache::default();
        let ctx = GeneratedEvalContext::with_analysis(&[0.0], 300.15, 1, GeneratedAnalysisKind::Ac);
        for (coefficient, omega, ddt, idt, expected) in [
            (2.0, 4.0, 1, 0, 8.0),
            (2.0, 4.0, 2, 0, -32.0),
            (2.0, 4.0, 3, 0, -128.0),
            (2.0, 4.0, 4, 0, 512.0),
            (2.0, 4.0, 0, 1, -0.5),
            (2.0, 4.0, 0, 2, -0.125),
            (2.0, 4.0, 0, 3, 0.03125),
            (2.0, 4.0, 1, 1, 2.0),
            (1e-300, 1e200, 2, 0, -1e100),
            (1e300, 1e-200, 2, 0, -1e-100),
            (1e-300, 1e-200, 0, 2, -1e100),
            (1e300, 1e200, 0, 2, -1e-100),
            (0.0, 1e200, 2, 0, 0.0),
            (2.0, 0.0, 2, 0, 0.0),
        ] {
            let stamper = GeneratedReactiveStamper::new_with_local_maps_and_static_cache(
                &mut matrix,
                &[],
                &[],
                1,
                omega,
                &cache,
            );
            let actual = stamper
                .scaled_frequency_coefficient(&ctx, coefficient, 1.0, ddt, idt)
                .unwrap();
            assert!(
                (actual - expected).abs() <= 2e-15 * expected.abs(),
                "{coefficient} * (j{omega})^{ddt} / (j{omega})^{idt}: {actual} != {expected}"
            );
            assert!(!ctx.evaluation_failed());
        }
    }

    #[test]
    fn frequency_coefficients_report_singular_or_invalid_inputs() {
        let structure = StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).unwrap();
        let mut matrix = ComplexMatrix::from_real_structure(&structure);
        let cache = GeneratedStaticStampCache::default();
        for (coefficient, omega, ddt, idt) in [
            (1.0, 0.0, 0, 1),
            (1.0, 0.0, 1, 1),
            (0.0, -1.0, 1, 0),
            (1.0, f64::NAN, 1, 0),
            (1.0, f64::INFINITY, 1, 0),
            (f64::NAN, 1.0, 1, 0),
            (1.0, 1e200, 2, 0),
            (1.0, 1.000001, 1_000_000_000, 0),
        ] {
            let ctx =
                GeneratedEvalContext::with_analysis(&[0.0], 300.15, 1, GeneratedAnalysisKind::Ac);
            let stamper = GeneratedReactiveStamper::new_with_local_maps_and_static_cache(
                &mut matrix,
                &[],
                &[],
                1,
                omega,
                &cache,
            );
            assert!(
                stamper
                    .scaled_frequency_coefficient(&ctx, coefficient, 1.0, ddt, idt)
                    .is_none()
            );
            assert!(matches!(
                ctx.take_evaluation_error(),
                Some(GeneratedEvaluationError::SmallSignal { .. })
            ));
        }
    }

    #[test]
    fn generated_array_errors_preserve_the_first_failure_and_trial_class() {
        for (raw, non_finite) in [
            (f64::NAN, true),
            (f64::INFINITY, true),
            (2.0, false),
            (2.0_f64.powi(63), false),
        ] {
            let ctx = GeneratedEvalContext::new(&[0.0], 300.15, 1);
            assert_eq!(ctx.checked_array_index(-0.5, 3, -1), 0.0);
            assert!(ctx.checked_array_index(raw, 3, -1).is_nan());
            ctx.report_initialization_error(0);
            assert!(
                matches!(ctx.take_evaluation_error(),Some(GeneratedEvaluationError::ArrayIndex{non_finite:actual,..}) if actual==non_finite)
            );
        }
    }

    #[test]
    fn generated_derivative_errors_survive_predicates_and_preserve_the_first_failure() {
        let ctx = GeneratedEvalContext::new(&[0.0], 300.15, 1);
        assert_eq!(ctx.checked_derivative_value(5.0, 0.0), 0.0);
        assert!(!ctx.evaluation_failed());
        for (primal, derivative) in [(f64::NAN, 1.0), (5.0, f64::INFINITY)] {
            let positive = ctx.checked_derivative_value(primal, derivative) > 0.0;
            assert!(!positive);
            assert!(ctx.evaluation_failed());
            ctx.report_initialization_error(0);
            assert!(matches!(
                ctx.take_evaluation_error(),
                Some(GeneratedEvaluationError::Derivative { .. })
            ));
        }
        ctx.report_initialization_error(1);
        ctx.checked_derivative_value(f64::INFINITY, 0.0);
        assert_eq!(
            ctx.take_evaluation_error(),
            Some(GeneratedEvaluationError::Initialization { slot: 1 })
        );
    }

    #[test]
    fn generated_discontinuity_degree_errors_preserve_the_first_failure() {
        let ctx = GeneratedEvalContext::new(&[0.0], 300.15, 1);
        ctx.report_discontinuity_degree_error();
        ctx.report_initialization_error(0);
        assert!(ctx.evaluation_failed());
        let error = ctx.take_evaluation_error().unwrap();
        assert_eq!(error, GeneratedEvaluationError::DiscontinuityDegree);
        assert!(error.to_string().contains("$discontinuity"));
        ctx.report_initialization_error(1);
        ctx.report_discontinuity_degree_error();
        assert_eq!(
            ctx.take_evaluation_error(),
            Some(GeneratedEvaluationError::Initialization { slot: 1 })
        );
    }

    #[test]
    fn generated_integer_errors_are_retained_until_reported() {
        let ctx = GeneratedEvalContext::new(&[0.0], 300.15, 1);
        assert_eq!(
            ctx.integer_result(crate::integer::real_to_integer(-1.5).map(f64::from)),
            -2.0
        );
        assert!(!ctx.evaluation_failed());
        assert!(
            ctx.integer_result(crate::integer::real_to_integer(2147483647.5).map(f64::from))
                .is_nan()
        );
        ctx.report_initialization_error(0);
        assert!(matches!(
            ctx.check_noise_evaluation(),
            Err(crate::GeneratedNoiseEvaluationError::Evaluation {
                source: GeneratedEvaluationError::Integer { .. }
            })
        ));
        assert!(matches!(
            ctx.take_evaluation_error(),
            Some(GeneratedEvaluationError::Integer { .. })
        ));
        assert!(!ctx.evaluation_failed());
        ctx.report_initialization_error(1);
        assert!(
            ctx.integer_result(crate::integer::real_to_integer(f64::NAN).map(f64::from))
                .is_nan()
        );
        assert_eq!(
            ctx.take_evaluation_error(),
            Some(GeneratedEvaluationError::Initialization { slot: 1 })
        );
    }

    #[test]
    fn frequency_stamping_preserves_local_maps_ground_aliases_and_branch_signs() {
        let entries = (0..4)
            .flat_map(|row| (0..4).map(move |col| (row, col, 0.0)))
            .collect::<Vec<_>>();
        let structure = StaticMatrix::from_triplets(4, 4, &entries).unwrap();
        let nodes = [2, 0, 1, 2];
        let branches = [2, 1];
        let mut matrix = ComplexMatrix::from_real_structure(&structure);
        matrix.add_real(1, 1, 1e-30);
        let mut cache = GeneratedStaticStampCache::default();
        cache.link(&structure, &nodes, &branches, 2);
        let mut stamper = GeneratedReactiveStamper::new_with_local_maps_and_static_cache(
            &mut matrix,
            &nodes,
            &branches,
            2,
            1.0,
            &cache,
        );
        stamper.stamp_current_frequency_local::<true>(
            Some(0),
            Some(2),
            GeneratedDerivative::node(2, 3.0),
        );
        stamper.stamp_current_frequency_local::<false>(
            Some(0),
            Some(2),
            GeneratedDerivative::branch(0, 5.0),
        );
        stamper.stamp_potential_frequency_local::<true>(1, GeneratedDerivative::node(0, 7.0));
        stamper.stamp_potential_frequency_local::<false>(0, GeneratedDerivative::branch(1, 11.0));
        stamper.stamp_current_frequency_local::<true>(
            Some(0),
            Some(3),
            GeneratedDerivative::node(0, 13.0),
        );
        stamper.stamp_current_frequency_local::<false>(
            Some(0),
            Some(2),
            GeneratedDerivative::node(1, 17.0),
        );
        assert_eq!(
            matrix.to_dense_real(),
            vec![
                vec![-3.0, 0.0, 0.0, 0.0],
                vec![3.0, 1e-30, 0.0, 0.0],
                vec![0.0, -7.0, 0.0, 0.0],
                vec![0.0; 4]
            ]
        );
        assert_eq!(
            matrix.to_dense_imag(),
            vec![
                vec![0.0, 0.0, 0.0, -5.0],
                vec![0.0, 0.0, 0.0, 5.0],
                vec![0.0; 4],
                vec![0.0, 0.0, -11.0, 0.0]
            ]
        );
    }

    #[test]
    fn generated_cross_checkpoint_lanes_round_trip_and_fail_closed() {
        let state = GeneratedCrossState {
            value: -2.5,
            time: 4.0,
            side: -1,
            last_event_time: 3.0,
            last_crossing_time: 3.0,
            initialized: true,
        };
        let mut lanes = Vec::new();
        state.append_checkpoint_lanes(&mut lanes);
        assert_eq!(lanes.len(), GeneratedCrossState::CHECKPOINT_LANES);
        assert_eq!(
            GeneratedCrossState::from_checkpoint_lanes(&lanes),
            Ok(state)
        );

        assert!(
            GeneratedCrossState::from_checkpoint_lanes(&lanes[..5])
                .expect_err("truncated checkpoint must fail")
                .contains("requires 6 lanes")
        );

        let mut malformed = lanes.clone();
        malformed[2] = 2.0;
        assert!(
            GeneratedCrossState::from_checkpoint_lanes(&malformed)
                .expect_err("invalid crossing side must fail")
                .contains("side must be -1, 0, or 1")
        );

        malformed = lanes.clone();
        malformed[5] = 0.5;
        let error = GeneratedCrossState::from_checkpoint_lanes(&malformed)
            .expect_err("fractional initialized flag must fail");
        assert!(error.contains("crossing initialized flag"));
        assert!(error.contains("finite integer"));

        malformed = lanes.clone();
        malformed[5] = 2.0;
        assert!(
            GeneratedCrossState::from_checkpoint_lanes(&malformed)
                .expect_err("out-of-domain initialized flag must fail")
                .contains("initialized flag must be 0 or 1")
        );

        malformed = lanes;
        malformed[0] = Value::NAN;
        assert!(
            GeneratedCrossState::from_checkpoint_lanes(&malformed)
                .expect_err("non-finite crossing state must fail")
                .contains("malformed")
        );
    }

    #[test]
    fn generated_cross_is_pure_against_accepted_state_and_refines_the_root() {
        let initial = evaluate_generated_cross(
            GeneratedCrossState::INITIAL,
            -1.0,
            0.0,
            1.0,
            0.0,
            0.0,
            1.0,
            true,
        )
        .expect("initialize crossing state");
        assert!(!initial.fired);
        let accepted = initial.candidate;

        let crossing = evaluate_generated_cross(accepted, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, true)
            .expect("evaluate crossing");
        assert!(crossing.fired);
        assert_eq!(crossing.candidate.value, 1.0);
        assert_eq!(crossing.candidate.time, 1.0);
        assert_eq!(crossing.candidate.side, 1);
        assert_eq!(crossing.candidate.last_crossing_time, 0.5);
        assert_eq!(
            crossing.refinement_time,
            Some(f64::from_bits(0.5f64.to_bits() + 1))
        );

        let repeated = evaluate_generated_cross(accepted, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, true)
            .expect("repeat the same Newton candidate");
        assert_eq!(repeated, crossing);
        assert_eq!(accepted, initial.candidate, "candidate evaluation is pure");

        let rolled_back = evaluate_generated_cross(accepted, -0.5, 0.75, 1.0, 0.0, 0.0, 1.0, true)
            .expect("evaluate from the unmodified accepted endpoint");
        assert!(!rolled_back.fired);
        assert_eq!(rolled_back.candidate.side, -1);
    }

    #[test]
    fn generated_event_operands_are_exact_and_invalid_directions_are_inactive() {
        let initialized = evaluate_generated_cross(
            GeneratedCrossState::INITIAL,
            -1.0,
            0.0,
            2.0,
            0.0,
            0.0,
            1.0,
            true,
        )
        .expect("integer direction outside the event domain remains legal");
        let inactive =
            evaluate_generated_cross(initialized.candidate, 1.0, 1.0, 2.0, 0.0, 0.0, 1.0, true)
                .expect("out-of-domain integer direction is inactive");
        assert!(!inactive.fired);

        assert_eq!(
            evaluate_generated_cross(initialized.candidate, 1.0, 1.0, 0.5, 0.0, 0.0, 1.0, true,),
            Err(GeneratedEventControlError::NonIntegerDirection)
        );
        assert_eq!(
            evaluate_generated_above(initialized.candidate, 1.0, 1.0, 0.0, 0.0, 0.5, false,),
            Err(GeneratedEventControlError::NonIntegerEnable)
        );

        // A non-finite operand is a different fact from a finite non-integer
        // one: `0.5` is the module's source and no other point will fix it,
        // NaN is the iterate the solver offered. The consumer rejects the
        // point for one and refuses the run for the other, so the producer
        // must not collapse them — the interpreter and the native route have
        // always split them.
        assert_eq!(
            evaluate_generated_cross(
                initialized.candidate,
                1.0,
                1.0,
                Value::NAN,
                0.0,
                0.0,
                1.0,
                true,
            ),
            Err(GeneratedEventControlError::NonFiniteOperand)
        );
        assert_eq!(
            evaluate_generated_above(
                initialized.candidate,
                1.0,
                1.0,
                0.0,
                0.0,
                Value::INFINITY,
                false,
            ),
            Err(GeneratedEventControlError::NonFiniteOperand)
        );
    }

    #[test]
    fn generated_above_fires_at_a_positive_static_initial_point() {
        let above =
            evaluate_generated_above(GeneratedCrossState::INITIAL, 0.25, 0.0, 0.0, 0.0, 1.0, true)
                .expect("static above evaluation");
        assert!(above.fired);
        assert_eq!(above.candidate.side, 1);
        assert_eq!(above.candidate.last_event_time, 0.0);
        assert_eq!(above.candidate.last_crossing_time, 0.0);
        assert_eq!(above.refinement_time, None);

        let disabled =
            evaluate_generated_above(GeneratedCrossState::INITIAL, 0.25, 0.0, 0.0, 0.0, 0.0, true)
                .expect("disabled static above evaluation");
        assert!(!disabled.fired);
    }

    #[test]
    fn generated_timer_reports_one_shot_and_periodic_event_bounds() {
        assert_eq!(
            evaluate_generated_timer(1.0, 0.0, 0.0, 1.0, 0.0, 0.0),
            (false, Some(1.0))
        );
        assert_eq!(
            evaluate_generated_timer(1.0, 0.0, 0.0, 1.0, 1.0, 1.0),
            (true, None)
        );
        assert_eq!(
            evaluate_generated_timer(1.0, 0.0, 0.0, 1.0, 1.0, 1.0),
            (true, None),
            "repeated Newton evaluations keep a timer event level true"
        );
        assert_eq!(
            evaluate_generated_timer(1.0, 0.0, 0.0, 1.0, 2.0, 1.0),
            (false, None),
            "an accepted one-shot endpoint is not replayed"
        );

        assert_eq!(
            evaluate_generated_timer(1.0, 2.0, 0.0, 1.0, 0.0, 0.0),
            (false, Some(1.0))
        );
        assert_eq!(
            evaluate_generated_timer(1.0, 2.0, 0.0, 1.0, 1.0, 1.0),
            (true, Some(3.0))
        );
        assert_eq!(
            evaluate_generated_timer(1.0, 2.0, 0.0, 1.0, 2.0, 1.0),
            (false, Some(3.0))
        );
        assert_eq!(
            evaluate_generated_timer(1.0, 2.0, 0.0, 1.0, 3.0, 1.0),
            (true, Some(5.0))
        );
        assert_eq!(
            evaluate_generated_timer(1.0, -1.0, 0.0, 1.0, 0.0, 0.0),
            (false, None)
        );
    }

    #[test]
    fn descriptor_v3_unifies_identity_terminal_current_and_dual_scope_metadata() {
        const TERMINALS: [GeneratedVerilogATerminalDescriptor; 1] =
            [GeneratedVerilogATerminalDescriptor {
                name: "FG",
                direction: GeneratedVerilogATerminalDirection::InOut,
                discipline: "electrical",
                current_parameter: "ifg",
            }];
        const PARAMETER: GeneratedVerilogAParameterDescriptor =
            GeneratedVerilogAParameterDescriptor::dual("rth0", Some(0.0))
                .integer()
                .aliases(&["rth"])
                .minimum(GeneratedVerilogAParameterBound::inclusive(0.0))
                .maximum(GeneratedVerilogAParameterBound::exclusive(100.0))
                .excluded_values(&[50.0])
                .dynamic_constraints();
        const EXPECTED_PARAMETER: GeneratedVerilogAParameterDescriptor =
            GeneratedVerilogAParameterDescriptor {
                name: "rth0",
                aliases: &["rth"],
                scope: GeneratedVerilogAParameterScope::Dual,
                is_integer: true,
                is_array: false,
                default: Some(0.0),
                minimum: Some(GeneratedVerilogAParameterBound {
                    value: 0.0,
                    exclusive: false,
                }),
                maximum: Some(GeneratedVerilogAParameterBound {
                    value: 100.0,
                    exclusive: true,
                }),
                excluded_values: &[50.0],
                has_dynamic_constraints: true,
            };
        const MODEL_PARAMETER: GeneratedVerilogAParameterDescriptor =
            GeneratedVerilogAParameterDescriptor::model("model_only", None);
        const INSTANCE_PARAMETER: GeneratedVerilogAParameterDescriptor =
            GeneratedVerilogAParameterDescriptor::instance("instance_only", Some(1.0));

        assert_eq!(GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION, 5);
        const ARRAY: GeneratedVerilogAParameterDescriptor =
            GeneratedVerilogAParameterDescriptor::instance("coefficients", Some(0.0)).array();
        const {
            assert!(ARRAY.is_array);
            assert!(!PARAMETER.is_array);
        }
        assert_eq!(ARRAY.default, None);
        assert_eq!(TERMINALS[0].name, "FG");
        assert_eq!(TERMINALS[0].current_parameter, "ifg");
        assert_eq!(PARAMETER, EXPECTED_PARAMETER);
        assert_eq!(
            MODEL_PARAMETER.scope,
            GeneratedVerilogAParameterScope::Model
        );
        assert_eq!(MODEL_PARAMETER.default, None);
        assert_eq!(
            INSTANCE_PARAMETER.scope,
            GeneratedVerilogAParameterScope::Instance
        );
        assert_eq!(INSTANCE_PARAMETER.default, Some(1.0));
        assert!(!GeneratedVerilogAParameterBound::inclusive(1.0).exclusive);
        assert!(GeneratedVerilogAParameterBound::exclusive(1.0).exclusive);
        assert_eq!(
            GeneratedParameterAssignment::new("rth", 1.0, GeneratedParameterOrigin::ModelCard)
                .origin,
            GeneratedParameterOrigin::ModelCard
        );
    }

    #[test]
    fn compatibility_catalog_requires_an_exact_strong_source_key() {
        validate_generated_veriloga_compatibility_catalog().expect("catalog is globally valid");
        let retained = &GENERATED_VERILOGA_COMPATIBILITY_CATALOG[0];
        assert_eq!(
            generated_veriloga_compatibility_entry(retained.module_name, retained.source_identity,),
            Ok(Some(retained)),
        );
        let previous_shape = retained.accepted_state_shape_identity_aliases[0];
        assert!(
            generated_veriloga_accepted_state_shape_is_compatible(
                retained.public_model_name,
                retained.source_identity,
                retained.semantic_identity,
                previous_shape,
                retained.accepted_state_shape_identity,
            )
            .expect("valid shape alias"),
            "the exact authenticated previous shape must migrate"
        );
        assert!(
            !generated_veriloga_accepted_state_shape_is_compatible(
                retained.public_model_name,
                retained.source_identity,
                retained.semantic_identity,
                retained.accepted_state_shape_identity,
                retained.accepted_state_shape_identity,
            )
            .expect("valid catalog"),
            "the current identity is not a migration alias"
        );
        assert!(
            generated_veriloga_compatibility_entry("VBIC13", retained.source_identity,)
                .expect("valid catalog")
                .is_none(),
            "catalog matching must not infer case-insensitive equivalence"
        );
        let mut changed_source = retained.source_identity.to_string();
        changed_source.replace_range(..1, "0");
        assert!(
            generated_veriloga_compatibility_entry(retained.module_name, &changed_source,)
                .expect("valid catalog")
                .is_none(),
            "a source near-match must fail closed"
        );
        assert_eq!(
            generated_veriloga_v26_compatibility_entry(
                retained.public_model_name,
                retained
                    .wire_v26_combined_identity_alias
                    .expect("v26 alias"),
            ),
            Ok(Some(retained)),
        );
    }

    #[test]
    fn accepted_state_shape_identity_hex_is_strict_and_round_trips() {
        let value = GENERATED_VERILOGA_COMPATIBILITY_CATALOG[0].accepted_state_shape_identity;
        let identity = GeneratedVerilogAAcceptedStateShapeIdentity::from_hex(value)
            .expect("catalog shape is valid");
        assert_eq!(identity.to_string(), value);
        assert!(
            GeneratedVerilogAAcceptedStateShapeIdentity::from_hex(&value.to_uppercase()).is_err()
        );
        assert!(GeneratedVerilogAAcceptedStateShapeIdentity::from_hex(&value[..62]).is_err());
    }

    #[test]
    fn generated_ddt_coefficients_scale_the_complete_dynamic_residual() {
        let coefficients = GeneratedDdtCoefficients {
            active: true,
            derivative_scale: 2.0,
            previous_value_scale: -3.0,
            older_value_scale: 5.0,
            previous_derivative_scale: -7.0,
        };

        assert_eq!(
            coefficients.scaled(0.5),
            GeneratedDdtCoefficients {
                active: true,
                derivative_scale: 1.0,
                previous_value_scale: -1.5,
                older_value_scale: 2.5,
                previous_derivative_scale: -3.5,
            }
        );
        assert_eq!(
            GeneratedDdtCoefficients::inactive().scaled(2.0),
            GeneratedDdtCoefficients::inactive()
        );
    }

    #[test]
    fn generated_ddt_coefficients_accept_fractional_derivative_history() {
        let generalized = GeneratedDdtCoefficients::from_companion_values_with_derivative_scale(
            1.0 / 0.51,
            1.0 / 0.51,
            0.0,
            false,
            0.49 / 0.51,
            0.25,
        )
        .expect("a quarter-second step is integrable");
        assert_eq!(generalized.derivative_scale, (1.0 / 0.51) / 0.25);
        assert_eq!(generalized.previous_value_scale, (1.0 / 0.51) / 0.25);
        assert_eq!(generalized.older_value_scale, 0.0);
        assert_eq!(generalized.previous_derivative_scale, 0.49 / 0.51);

        let canonical =
            GeneratedDdtCoefficients::from_companion_values(2.0, 2.0, 0.0, false, true, 0.25)
                .expect("a quarter-second step is integrable");
        assert_eq!(canonical.previous_derivative_scale, 1.0);
    }

    #[test]
    fn generated_ddt_coefficients_refuse_an_unintegrable_timestep() {
        // Zero is the static-evaluation convention, not an unintegrable step.
        assert_eq!(
            GeneratedDdtCoefficients::from_companion_values(1.0, 1.0, 0.0, false, false, 0.0),
            Ok(GeneratedDdtCoefficients::inactive())
        );

        for timestep in [
            1.0e-21,
            -1.0e-21,
            GENERATED_DDT_TIMESTEP_FLOOR.next_down(),
            Value::NAN,
            Value::INFINITY,
        ] {
            let error = GeneratedDdtCoefficients::from_companion_values(
                1.0, 1.0, 0.0, false, false, timestep,
            )
            .expect_err("a step the companion rule cannot represent must be refused");
            assert_eq!(error.floor, GENERATED_DDT_TIMESTEP_FLOOR);
            assert_eq!(error.timestep.to_bits(), timestep.to_bits());
            let rendered = error.to_string();
            assert!(
                rendered.contains("generated ddt cannot integrate")
                    && rendered.contains(&format!(
                        "needs at least {GENERATED_DDT_TIMESTEP_FLOOR:.16e}"
                    )),
                "the refusal must name the timestep and the floor, got: {rendered}"
            );
        }

        for timestep in [
            GENERATED_DDT_TIMESTEP_FLOOR,
            GENERATED_DDT_TIMESTEP_FLOOR.next_up(),
        ] {
            let coefficients = GeneratedDdtCoefficients::from_companion_values(
                1.0, 1.0, 0.0, false, false, timestep,
            )
            .expect("the documented minimum interval is integrable");
            let mut current = [0.0];
            let mut derivative = [0.0];
            let mut valid = [false];
            let result = rspice_eval_ddt(
                &mut current,
                &[0.0],
                &[0.0],
                &[true],
                &mut derivative,
                &[0.0],
                &mut valid,
                coefficients,
                0,
                timestep,
            )
            .expect("a unit ramp has a finite derivative at the minimum interval");
            assert!((result - 1.0).abs() <= 2.0 * f64::EPSILON);
            assert_eq!(current, [timestep]);
            assert_eq!(valid, [true]);
        }
    }

    #[test]
    fn generated_ddt_helper_publishes_only_finite_candidates() {
        let coefficients = GeneratedDdtCoefficients {
            active: true,
            derivative_scale: 2.0,
            previous_value_scale: 2.0,
            older_value_scale: 0.0,
            previous_derivative_scale: 0.0,
        };
        let mut current = [1.0];
        let previous = [3.0];
        let older = [2.0];
        let initialized = [true];
        let mut derivative_current = [0.5];
        let derivative_previous = [1.0];
        let mut candidate_valid = [false];

        assert_eq!(
            rspice_eval_ddt(
                &mut current,
                &previous,
                &older,
                &initialized,
                &mut derivative_current,
                &derivative_previous,
                &mut candidate_valid,
                coefficients,
                0,
                5.0,
            ),
            Ok(4.0)
        );
        assert_eq!(current, [5.0]);
        assert_eq!(derivative_current, [4.0]);
        assert_eq!(candidate_valid, [true]);

        assert!(
            rspice_eval_ddt(
                &mut current,
                &previous,
                &older,
                &initialized,
                &mut derivative_current,
                &derivative_previous,
                &mut candidate_valid,
                coefficients,
                0,
                f64::NAN,
            )
            .is_err()
        );
        assert_eq!(current, [5.0], "failed evaluation must not publish payload");
        assert_eq!(derivative_current, [4.0]);
        assert_eq!(candidate_valid, [false]);
    }

    #[test]
    fn generated_idt_helper_keeps_accepted_history_immutable() {
        let coefficients = GeneratedDdtCoefficients {
            active: true,
            derivative_scale: 4.0,
            previous_value_scale: 4.0,
            older_value_scale: 0.0,
            previous_derivative_scale: 1.0,
        };
        let mut current = [0.0];
        let mut candidate_previous = [0.0];
        let mut input_current = [0.0];
        let previous = [2.0];
        let older = [1.0];
        let input_previous = [3.0];
        let initialized = [true];
        let mut candidate_valid = [false];

        let candidate = rspice_eval_idt(
            &mut current,
            &mut candidate_previous,
            &mut input_current,
            &previous,
            &older,
            &input_previous,
            &initialized,
            &mut candidate_valid,
            coefficients,
            0,
            1.0,
            0.25,
        )
        .expect("finite trapezoidal candidate");
        assert_eq!(candidate.value, 3.0);
        assert_eq!(candidate.jacobian_scale, 0.25);
        assert_eq!(current, [3.0]);
        assert_eq!(candidate_previous, previous);
        assert_eq!(input_current, [1.0]);
        assert_eq!(candidate_valid, [true]);
        assert_eq!(previous, [2.0]);
        assert_eq!(older, [1.0]);
        assert_eq!(input_previous, [3.0]);
    }

    #[test]
    fn generalized_idt_candidate_matches_backward_euler() {
        let candidate = evaluate_generated_idt_candidate(
            GeneratedDdtCoefficients {
                active: true,
                derivative_scale: 2.0,
                previous_value_scale: 2.0,
                older_value_scale: 0.0,
                previous_derivative_scale: 0.0,
            },
            4.0,
            0.25,
            GeneratedIdtAcceptedHistory {
                initialized: true,
                integral_previous: 10.0,
                integral_older: 8.0,
                input_previous: 3.0,
            },
        )
        .expect("backward-Euler candidate");

        assert_eq!(candidate.value, 12.0);
        assert_eq!(candidate.jacobian_scale, 0.5);
    }

    #[test]
    fn generalized_idt_candidate_matches_trapezoidal_and_gear() {
        let trapezoidal = evaluate_generated_idt_candidate(
            GeneratedDdtCoefficients {
                active: true,
                derivative_scale: 4.0,
                previous_value_scale: 4.0,
                older_value_scale: 0.0,
                previous_derivative_scale: 1.0,
            },
            1.0,
            0.0,
            GeneratedIdtAcceptedHistory {
                initialized: true,
                integral_previous: 0.0,
                integral_older: 0.0,
                input_previous: 0.0,
            },
        )
        .expect("trapezoidal candidate");
        assert_eq!(trapezoidal.value, 0.25);
        assert_eq!(trapezoidal.jacobian_scale, 0.25);

        let gear = evaluate_generated_idt_candidate(
            GeneratedDdtCoefficients {
                active: true,
                derivative_scale: 3.0,
                previous_value_scale: 4.0,
                older_value_scale: -1.0,
                previous_derivative_scale: 0.0,
            },
            2.0,
            0.0,
            GeneratedIdtAcceptedHistory {
                initialized: true,
                integral_previous: trapezoidal.value,
                integral_older: 0.0,
                input_previous: 1.0,
            },
        )
        .expect("Gear-2 candidate");
        assert_eq!(gear.value, 1.0);
        assert_eq!(gear.jacobian_scale, 1.0 / 3.0);
    }

    #[test]
    fn generalized_idt_candidate_handles_inactive_and_uninitialized_state() {
        let inactive = evaluate_generated_idt_candidate(
            GeneratedDdtCoefficients::inactive(),
            7.0,
            1.25,
            GeneratedIdtAcceptedHistory {
                initialized: false,
                integral_previous: 0.0,
                integral_older: 0.0,
                input_previous: 0.0,
            },
        )
        .expect("inactive candidate");
        assert_eq!(inactive.value, 1.25);
        assert_eq!(inactive.jacobian_scale, 0.0);

        let first_backward_euler = evaluate_generated_idt_candidate(
            GeneratedDdtCoefficients {
                active: true,
                derivative_scale: 2.0,
                previous_value_scale: 2.0,
                older_value_scale: 0.0,
                previous_derivative_scale: 0.0,
            },
            4.0,
            10.0,
            GeneratedIdtAcceptedHistory {
                initialized: false,
                integral_previous: 0.0,
                integral_older: 0.0,
                input_previous: 0.0,
            },
        )
        .expect("first backward-Euler candidate");
        assert_eq!(first_backward_euler.value, 12.0);
        assert_eq!(first_backward_euler.jacobian_scale, 0.5);
    }

    #[test]
    fn generalized_idt_candidate_rejects_malformed_numeric_state() {
        let coefficients = GeneratedDdtCoefficients {
            active: true,
            derivative_scale: 2.0,
            previous_value_scale: 2.0,
            older_value_scale: 0.0,
            previous_derivative_scale: 0.0,
        };
        let history = GeneratedIdtAcceptedHistory {
            initialized: true,
            integral_previous: 1.0,
            integral_older: 0.5,
            input_previous: 2.0,
        };

        assert!(matches!(
            evaluate_generated_idt_candidate(coefficients, f64::NAN, 0.0, history),
            Err(GeneratedIdtCandidateError::NonFiniteInput { field: "input" })
        ));
        assert!(matches!(
            evaluate_generated_idt_candidate(
                coefficients,
                1.0,
                0.0,
                GeneratedIdtAcceptedHistory {
                    initialized: false,
                    integral_previous: f64::INFINITY,
                    ..history
                },
            ),
            Err(GeneratedIdtCandidateError::NonFiniteInput {
                field: "accepted previous integral"
            })
        ));

        let mut malformed_coefficients = coefficients;
        malformed_coefficients.previous_value_scale = f64::NAN;
        assert!(matches!(
            evaluate_generated_idt_candidate(malformed_coefficients, 1.0, 0.0, history),
            Err(GeneratedIdtCandidateError::NonFiniteInput {
                field: "previous-value scale"
            })
        ));

        let mut zero_scale = coefficients;
        zero_scale.derivative_scale = -0.0;
        assert_eq!(
            evaluate_generated_idt_candidate(zero_scale, 1.0, 0.0, history),
            Err(GeneratedIdtCandidateError::ZeroDerivativeScale)
        );

        let mut reciprocal_overflow = coefficients;
        reciprocal_overflow.derivative_scale = f64::from_bits(1);
        reciprocal_overflow.previous_value_scale = 0.0;
        assert!(matches!(
            evaluate_generated_idt_candidate(reciprocal_overflow, 0.0, 0.0, history),
            Err(GeneratedIdtCandidateError::NonFiniteResult {
                field: "Jacobian scale"
            })
        ));

        let mut rescued_numerator = coefficients;
        rescued_numerator.previous_value_scale = f64::MAX;
        assert_eq!(
            evaluate_generated_idt_candidate(
                rescued_numerator,
                1.0,
                0.0,
                GeneratedIdtAcceptedHistory {
                    integral_previous: 2.0,
                    ..history
                },
            )
            .unwrap()
            .value,
            f64::MAX,
        );
        rescued_numerator.derivative_scale = 1.0;
        assert!(matches!(
            evaluate_generated_idt_candidate(
                rescued_numerator,
                1.0,
                0.0,
                GeneratedIdtAcceptedHistory {
                    integral_previous: 2.0,
                    ..history
                },
            ),
            Err(GeneratedIdtCandidateError::NonFiniteResult {
                field: "candidate value"
            })
        ));
    }

    #[test]
    fn accepted_point_tasks_do_not_enable_newton_limiting() {
        for (analysis, mode) in [
            (
                GeneratedAnalysisKind::Ac,
                GeneratedEvaluationMode::SmallSignal,
            ),
            (
                GeneratedAnalysisKind::Noise,
                GeneratedEvaluationMode::SmallSignal,
            ),
            (
                GeneratedAnalysisKind::Dc,
                GeneratedEvaluationMode::StaticProbe,
            ),
            (
                GeneratedAnalysisKind::Tran,
                GeneratedEvaluationMode::StaticProbe,
            ),
        ] {
            let probe = GeneratedEvalContext::with_analysis_step_simparams_and_mode(
                &[0.0],
                300.15,
                1,
                analysis,
                false,
                true,
                GeneratedSimulationParameters::default(),
                mode,
            );
            assert!(!probe.analog_tasks_enabled());
            assert!(!probe.limiting_enabled());
            let candidate = probe.clone().with_analog_task_recording(true);
            assert!(candidate.analog_tasks_enabled());
            assert!(!candidate.limiting_enabled());
            assert_eq!(
                candidate.dynamic_operators_enabled(),
                probe.dynamic_operators_enabled()
            );
            assert!(
                !candidate
                    .with_analog_task_recording(false)
                    .analog_tasks_enabled()
            );
            assert!(!probe.analog_tasks_enabled());
        }
        let newton = GeneratedEvalContext::new(&[0.0], 300.15, 1);
        assert!(newton.analog_tasks_enabled() && newton.limiting_enabled());
        let numerical_probe = newton.with_analog_task_recording(false);
        assert!(!numerical_probe.analog_tasks_enabled());
        assert!(numerical_probe.limiting_enabled());
    }

    #[test]
    fn small_signal_integration_freezes_without_disabling_analysis_events() {
        for analysis in [GeneratedAnalysisKind::Ac, GeneratedAnalysisKind::Noise] {
            for (phase, integrates) in [
                (AnalogAnalysisPhase::Point, false),
                (AnalogAnalysisPhase::Equilibrium, true),
                (AnalogAnalysisPhase::Nodeset, true),
            ] {
                let context = GeneratedEvalContext::with_analysis_step(
                    &[0.0],
                    300.15,
                    1,
                    analysis,
                    true,
                    true,
                )
                .with_analysis_phase(phase);
                assert_eq!(context.integration_operators_enabled(), integrates);
                assert!(context.dynamic_operators_enabled());
                assert!(context.analysis_initial_step());
                assert!(context.analysis_final_step());
            }
        }
    }

    #[test]
    fn static_dae_probe_is_the_only_mode_that_disables_dynamic_operators() {
        let voltages = [0.0];
        let dynamic = GeneratedEvalContext::with_analysis_step_simparams_and_mode(
            &voltages,
            300.15,
            1,
            GeneratedAnalysisKind::Tran,
            true,
            true,
            GeneratedSimulationParameters::default(),
            GeneratedEvaluationMode::NewtonLimited,
        );
        let static_probe = GeneratedEvalContext::with_analysis_step_simparams_and_mode(
            &voltages,
            300.15,
            1,
            GeneratedAnalysisKind::Tran,
            true,
            true,
            GeneratedSimulationParameters::default(),
            GeneratedEvaluationMode::StaticProbe,
        );
        let static_dae_probe = GeneratedEvalContext::with_analysis_step_simparams_and_mode(
            &voltages,
            300.15,
            1,
            GeneratedAnalysisKind::Tran,
            true,
            true,
            GeneratedSimulationParameters::default(),
            GeneratedEvaluationMode::StaticDaeProbe,
        );

        assert!(dynamic.dynamic_operators_enabled());
        assert!(static_probe.dynamic_operators_enabled());
        assert!(!static_dae_probe.dynamic_operators_enabled());
        assert!(dynamic.integration_operators_enabled());
        assert!(static_probe.integration_operators_enabled());
        assert!(!static_dae_probe.integration_operators_enabled());
        for context in [&dynamic, &static_probe, &static_dae_probe] {
            assert!(context.analysis("tran"));
            assert!(context.analysis("__rspice_scope_tran"));
            let events = context.dynamic_operators_enabled();
            assert_eq!(context.analysis_initial_step(), events);
            assert_eq!(context.analysis_final_step(), events);
            assert_eq!(context.analysis("__rspice_initial_step"), events);
            assert_eq!(context.analysis("__rspice_final_step"), events);
        }
    }

    #[test]
    fn inactive_potential_branch_pins_exact_unit_diagonal() {
        for cached in [false, true] {
            let mut matrix =
                StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).expect("one-entry branch matrix");
            let mut stamp_rhs = [0.0];
            let voltages = [17.0];
            let mut cache = GeneratedStaticStampCache::default();
            cache.link(&matrix, &[], &[1], 0);
            if cached {
                let mut stamper = GeneratedStamper::new_with_static_cache(
                    &mut matrix,
                    &mut stamp_rhs,
                    &voltages,
                    0,
                    &cache,
                );
                stamper.stamp_inactive_potential_branch_local(0);
            } else {
                // An unlinked cache has no slots, so the stamp takes the
                // matrix-index fallback.
                let unlinked = GeneratedStaticStampCache::default();
                let mut stamper = GeneratedStamper::new_with_static_cache(
                    &mut matrix,
                    &mut stamp_rhs,
                    &voltages,
                    0,
                    &unlinked,
                );
                stamper.stamp_inactive_potential_branch(1);
            }
            let solution = matrix.solve(&[2.5]).expect("unit diagonal solves");
            assert_eq!(solution, vec![2.5], "cached={cached}");
            assert_eq!(stamp_rhs, [0.0], "cached={cached}");
        }
    }

    #[test]
    fn indirect_source_coupling_preserves_constraint_rows_and_terminal_maps() {
        let entries = (0..4)
            .flat_map(|row| (0..4).map(move |col| (row, col, 0.0)))
            .collect::<Vec<_>>();
        let nodes = [2, 0, 1, 2];
        let branches = [2, 1];
        let voltages = [2.0, 4.0, -0.25, 0.5];
        for linked_slots in [false, true] {
            let mut matrix = StaticMatrix::from_triplets(4, 4, &entries).unwrap();
            let mut cache = GeneratedStaticStampCache::default();
            cache.link(&matrix, &nodes, &branches, 2);
            if !linked_slots {
                cache.slots = GeneratedStaticStampSlots::default();
            }
            let mut rhs = [0.0; 4];
            let mut currents = [0.0; 4];
            {
                let mut stamper = GeneratedStamper::new_with_static_cache_and_terminal_currents(
                    &mut matrix,
                    &mut rhs,
                    &voltages,
                    2,
                    &cache,
                    &mut currents,
                );
                stamper.stamp_branch_current_local(Some(0), Some(2), 0, 4.0);
                stamper.stamp_branch_current_local(Some(0), Some(3), 1, 5.0);
                stamper.stamp_branch_current_local(Some(1), Some(2), 1, 2.0);
                // f = 1e-20*V(0) - V(2) + 3*I(1) - 7. The shared
                // potential-row helper takes -f and -J for an indirect source.
                stamper.stamp_potential_sparse_local(
                    0,
                    9.75,
                    [0, 1, 2],
                    [-1e-20, -999.0, 1.0],
                    [1],
                    [-3.0],
                );
            }
            assert_eq!(rhs, [0.0, 0.0, 0.0, 7.0]);
            assert_eq!(currents, [0.75, -0.5, -1.5, 1.25]);
            let expected = [
                [0.0, 0.0, -2.0, -4.0],
                [0.0, 0.0, 0.0, 4.0],
                [0.0; 4],
                [-1.0, 1e-20, 3.0, 0.0],
            ];
            // The complete structure is column-major CSC.
            for (row, values) in expected.iter().enumerate() {
                for (col, expected) in values.iter().enumerate() {
                    assert_eq!(
                        matrix.values_mut()[col * 4 + row],
                        *expected,
                        "({row},{col}), linked_slots={linked_slots}"
                    );
                }
            }
        }
    }

    #[test]
    fn terminal_current_observer_preserves_local_lead_identity_and_sign() {
        let mut matrix = StaticMatrix::from_triplets(
            3,
            3,
            &[
                (0, 2, 0.0),
                (1, 2, 0.0),
                (2, 0, 0.0),
                (2, 1, 0.0),
                (2, 2, 0.0),
            ],
        )
        .expect("potential-source topology");
        let nodes = [1, 1, 2];
        let branches = [1];
        let voltages = [0.0, 0.0, -0.25];
        let mut rhs = [0.0; 3];
        let mut currents = [0.0; 3];
        let mut cache = GeneratedStaticStampCache::default();
        cache.link(&matrix, &nodes, &branches, 2);

        {
            let mut stamper = GeneratedStamper::new_with_static_cache_and_terminal_currents(
                &mut matrix,
                &mut rhs,
                &voltages,
                2,
                &cache,
                &mut currents,
            );
            stamper.stamp_current_sparse_local::<0, 0>(Some(0), Some(2), 2.0, [], [], [], [], 1.0);
            stamper.stamp_current_sparse_local::<0, 0>(Some(1), Some(2), 3.0, [], [], [], [], 1.0);
            stamper.stamp_potential_branch_local(Some(0), Some(2), 0, 4.0);
        }

        assert_eq!(
            currents,
            [1.0, 3.0, -4.0],
            "shorted external terminals retain distinct local lead currents"
        );
        assert_eq!(rhs, [-5.0, 5.0, 0.0]);
        assert_eq!(currents.iter().sum::<Value>(), 0.0);
    }

    fn left<const N: usize>() -> [f64; N] {
        let values = [
            0.0,
            -0.0,
            f64::MIN_POSITIVE,
            f64::from_bits(1),
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::from_bits(0x7ff8_0000_0000_0042),
            -3.5,
        ];
        core::array::from_fn(|index| values[index % values.len()])
    }

    fn right<const N: usize>() -> [f64; N] {
        let values = [
            -0.0,
            0.0,
            -f64::MIN_POSITIVE,
            f64::from_bits(2),
            2.0,
            -7.25,
            f64::from_bits(0x7ff8_0000_0000_00a5),
            f64::INFINITY,
        ];
        core::array::from_fn(|index| values[(index + 3) % values.len()])
    }

    fn assert_same_bits<const N: usize>(actual: [f64; N], expected: [f64; N]) {
        for index in 0..N {
            assert_eq!(
                actual[index].to_bits(),
                expected[index].to_bits(),
                "lane {index} differs: actual={:?}, expected={:?}",
                actual[index],
                expected[index]
            );
        }
    }

    macro_rules! assert_fixed_width {
        ($name:ident, $width:literal) => {{
            let lhs = left::<$width>();
            let rhs = right::<$width>();
            assert_same_bits(($name(lhs) + $name(rhs)).0, (Lanes(lhs) + Lanes(rhs)).0);
            assert_same_bits(($name(lhs) - $name(rhs)).0, (Lanes(lhs) - Lanes(rhs)).0);
            assert_same_bits(($name(lhs) * -3.25).0, (Lanes(lhs) * -3.25).0);
            assert_same_bits(($name(lhs) / -3.25).0, (Lanes(lhs) / -3.25).0);
            for (left_scalar, right_scalar) in [(2.0, -3.0), (1e-200, 1e-200), (1e200, -1e200)] {
                for divisor in [1e-200, 1.0, 1e200] {
                    let one = $name(lhs).product_div(left_scalar, divisor).0;
                    let two = $name(lhs)
                        .product_sum_div(left_scalar, $name(rhs), right_scalar, divisor)
                        .0;
                    let expected_one = arithmetic::sum_products_div_lanes::<$width>(
                        &[(&lhs, left_scalar)],
                        divisor,
                    );
                    let expected_two = arithmetic::sum_products_div_lanes::<$width>(
                        &[(&lhs, left_scalar), (&rhs, right_scalar)],
                        divisor,
                    );
                    for (actual, expected) in one
                        .into_iter()
                        .zip(expected_one)
                        .chain(two.into_iter().zip(expected_two))
                    {
                        if expected.is_nan() {
                            assert!(actual.is_nan());
                        } else {
                            assert_eq!(actual.to_bits(), expected.to_bits());
                        }
                    }
                }
            }
            for index in 0..$width {
                assert_eq!($name(lhs)[index].to_bits(), lhs[index].to_bits());
            }
            assert_eq!(
                core::mem::size_of::<$name>(),
                core::mem::size_of::<[f64; $width]>()
            );
            assert_eq!(
                core::mem::align_of::<$name>(),
                core::mem::align_of::<[f64; $width]>()
            );
        }};
    }

    #[test]
    fn every_fixed_lane_width_matches_the_generic_fallback_bit_for_bit() {
        assert_fixed_width!(L2, 2);
        assert_fixed_width!(L3, 3);
        assert_fixed_width!(L4, 4);
        assert_fixed_width!(L5, 5);
        assert_fixed_width!(L6, 6);
        assert_fixed_width!(L7, 7);
        assert_fixed_width!(L8, 8);
        assert_fixed_width!(L9, 9);
        assert_fixed_width!(L10, 10);
        assert_fixed_width!(L11, 11);
        assert_fixed_width!(L12, 12);
        assert_fixed_width!(L13, 13);
        assert_fixed_width!(L14, 14);
        assert_fixed_width!(L15, 15);
        assert_fixed_width!(L16, 16);
        assert_fixed_width!(L17, 17);
        assert_fixed_width!(L18, 18);
        assert_fixed_width!(L19, 19);
        assert_fixed_width!(L20, 20);
        assert_fixed_width!(L21, 21);
        assert_fixed_width!(L22, 22);
        assert_fixed_width!(L23, 23);
        assert_fixed_width!(L24, 24);
        assert_fixed_width!(L25, 25);
        assert_fixed_width!(L26, 26);
        assert_fixed_width!(L27, 27);
        assert_fixed_width!(L28, 28);
        assert_fixed_width!(L29, 29);
        assert_fixed_width!(L30, 30);
        assert_fixed_width!(L31, 31);
        assert_fixed_width!(L32, 32);
    }

    #[test]
    fn generated_stage_installation_handles_empty_contiguous_and_sparse_maps() {
        let mut destination = [10.0, 11.0, 12.0, 13.0, 14.0];
        install_generated_stage_values(&mut destination, &[], &[]);
        assert_eq!(destination, [10.0, 11.0, 12.0, 13.0, 14.0]);

        install_generated_stage_values(&mut destination, &[1.0, 2.0], &[1, 2]);
        assert_eq!(destination, [10.0, 1.0, 2.0, 13.0, 14.0]);

        install_generated_stage_values(&mut destination, &[7.0, 8.0], &[4, 0]);
        assert_eq!(destination, [8.0, 1.0, 2.0, 13.0, 7.0]);
    }

    #[test]
    #[should_panic(expected = "generated stage value/slot count mismatch")]
    fn generated_stage_installation_rejects_length_mismatch() {
        install_generated_stage_values(&mut [0.0; 2], &[1.0], &[]);
    }

    #[test]
    #[should_panic(expected = "generated stage slot is outside canonical storage")]
    fn generated_stage_installation_rejects_out_of_range_slots() {
        install_generated_stage_values(&mut [0.0; 2], &[1.0], &[2]);
    }

    #[test]
    fn generated_parameter_lookup_is_sorted_deterministic_and_compact() {
        let names = ["alpha", "alpha", "beta", "zeta"];
        let indices = [7, 9, 2, 4];
        assert_eq!(
            find_generated_parameter_index(&names, &indices, "alpha"),
            Some(7)
        );
        assert_eq!(
            find_generated_parameter_index(&names, &indices, "beta"),
            Some(2)
        );
        assert_eq!(
            find_generated_parameter_index(&names, &indices, "gamma"),
            None
        );
        assert_eq!(
            find_generated_parameter_index(&names, &indices, "Alpha"),
            None
        );
        assert_eq!(core::mem::size_of_val(&indices[0]), 2);
    }

    #[test]
    #[should_panic(expected = "generated parameter lookup table length mismatch")]
    fn generated_parameter_lookup_rejects_mismatched_tables() {
        find_generated_parameter_index(&["alpha"], &[], "alpha");
    }

    #[test]
    fn generated_parameter_bound_indices_preserve_sentinel_values_and_diagnostics() {
        const POOL: [GeneratedParameterBound; 2] = [
            GeneratedParameterBound {
                value: -1.0,
                label: "-1.0",
            },
            GeneratedParameterBound {
                value: 2.0,
                label: "2.0",
            },
        ];

        assert_eq!(GENERATED_PARAMETER_BOUND_NONE, 0);
        assert!(resolve_generated_parameter_bound(&POOL, 0).is_none());
        let resolved = resolve_generated_parameter_bound(&POOL, 2).expect("bound exists");
        assert_eq!(resolved.value.to_bits(), 2.0f64.to_bits());
        assert_eq!(resolved.label, "2.0");
        assert_eq!(core::mem::size_of_val(&2u16), core::mem::size_of::<u16>());

        assert_eq!(
            validate_generated_parameter_bound_indices("gain", -2.0, 0, &POOL, 1, 0, &[]),
            Err("parameter 'gain' must be >= -1.0, got -2".to_string())
        );
        assert_eq!(
            validate_generated_parameter_bound_indices("gain", 2.0, 0, &POOL, 0, 0, &[2]),
            Err("parameter 'gain' must not equal 2.0, got 2".to_string())
        );
    }

    #[test]
    #[should_panic(expected = "generated parameter-bound index is outside its pool")]
    fn generated_parameter_bound_indices_reject_out_of_range_values() {
        resolve_generated_parameter_bound(&[], 1);
    }

    #[test]
    #[should_panic(expected = "generated parameter exclusion uses the absence sentinel")]
    fn generated_parameter_bound_indices_reject_none_as_an_exclusion() {
        validate_generated_parameter_bound_indices("gain", 0.0, 0, &[], 0, 0, &[0])
            .expect("validation should panic before returning");
    }
}
