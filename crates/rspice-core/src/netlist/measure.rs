//! The `.MEAS` statement as parsed from a deck.
//!
//! These types are deck syntax, not results. A `.MEAS` card names a
//! measurement type, an operand, trigger and target specifications, and a
//! print policy; all of that is decided while reading the deck, and none of
//! it depends on having run an analysis.
//!
//! They lived in `analysis::measure` beside the engine that evaluates them,
//! which meant the parser reached five layers up into an analysis module for
//! the vocabulary of the card it was parsing — the single largest inverted
//! edge in the crate at 77 references. The evaluator (`MeasureEngine`,
//! `MeasureResult`, and the continuous-measure machinery) stays where it
//! belongs and now reads these types downward.
//!
//! `analysis::measure` re-exports everything here, so `rspice_core::analysis::MeasureType`
//! and friends keep resolving for callers outside the crate.

use super::{GroundPolicy, apply_ground_policy_to_probe_references};
use crate::Value;

/// Edge type for trigger/target detection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EdgeType {
    /// Rising edge (signal crosses threshold going up)
    #[default]
    Rise,
    /// Falling edge (signal crosses threshold going down)
    Fall,
    /// Either edge
    Cross,
}

/// Selects whether an extrema measurement reports the dependent value or the
/// independent-axis location where that value first occurs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExtremaOutput {
    #[default]
    Value,
    IndependentAxis,
}

/// Norm used by Xyce's waveform-relative error functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorFunctionNorm {
    /// `ERR`/`ERR1`: root mean square relative error.
    RootMeanSquare,
    /// `ERR2`: mean absolute relative error.
    MeanAbsolute,
}

/// Norm applied to the absolute difference vector by Xyce's file-backed
/// `ERROR` measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileErrorNorm {
    /// Maximum absolute difference.
    Infinity,
    /// Sum of absolute differences.
    L1,
    /// Euclidean (Frobenius) norm.
    L2,
}

/// Right-hand operand of a `WHEN left=right` measurement condition.
///
/// Numeric values are retained as scalars. Signal references and braced
/// expressions are retained as waveform names and materialized against the
/// accepted analysis-point stream before evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum MeasureOperand {
    Constant(Value),
    Waveform(String),
}

/// Xyce's default absolute equality tolerance for conditional measurements.
pub const XYCE_DEFAULT_MEASURE_MINVAL: Value = 1.0e-12;

/// Selects a particular conditional crossing in accepted-point order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventOccurrence {
    pub edge: EdgeType,
    /// Positive values select from the start; negative values select from the
    /// end (`-1` is Xyce's `LAST`). Zero is invalid and never matches.
    pub number: isize,
}

impl Default for EventOccurrence {
    fn default() -> Self {
        Self {
            edge: EdgeType::Cross,
            number: 1,
        }
    }
}

/// A typed conditional event used by FIND and DERIV measurements.
#[derive(Debug, Clone, PartialEq)]
pub struct WhenCondition {
    pub left: String,
    pub right: MeasureOperand,
    pub occurrence: EventOccurrence,
}

/// Event form for one side of a trigger/target delay measurement.
#[derive(Debug, Clone, PartialEq)]
pub enum TriggerEvent {
    /// Exact independent-axis location (`TRIG AT=...`).
    At(Value),
    /// Conditional waveform intersection (`TRIG lhs=rhs`).
    When(WhenCondition),
}

/// Trigger/Target specification for delay measurements.
#[derive(Debug, Clone, PartialEq)]
pub struct TrigSpec {
    pub event: TriggerEvent,
    /// Optional numeric-axis lower bound. Xyce copies an explicitly supplied
    /// TRIG TD to TARG when TARG omits TD.
    pub td: Option<Value>,
}

impl TrigSpec {
    pub fn new(signal: &str, value: Value) -> Self {
        Self {
            event: TriggerEvent::When(WhenCondition {
                left: signal.to_string(),
                right: MeasureOperand::Constant(value),
                occurrence: EventOccurrence {
                    edge: EdgeType::Cross,
                    number: 1,
                },
            }),
            td: None,
        }
    }

    pub fn with_edge(mut self, edge: EdgeType) -> Self {
        if let TriggerEvent::When(condition) = &mut self.event {
            condition.occurrence.edge = edge;
        }
        self
    }

    pub fn with_number(mut self, n: usize) -> Self {
        if let TriggerEvent::When(condition) = &mut self.event {
            // Preserve the infallible builder while failing closed for values
            // that the signed Xyce occurrence domain cannot represent.
            condition.occurrence.number = isize::try_from(n).unwrap_or(0);
        }
        self
    }
}

/// Type of measurement to perform
#[derive(Debug, Clone)]
pub enum MeasureType {
    /// Delay measurement: time between trigger and target events
    /// .MEAS TRAN name TRIG ... TARG ...
    Delay {
        trig: TrigSpec,
        targ: TrigSpec,
        minval: Value,
    },

    /// Find value at specific time or when condition is met
    /// .MEAS TRAN name FIND V(out) AT=time
    /// .MEAS TRAN name FIND V(out) WHEN V(in)=0.5
    Find {
        signal: String,
        at: Option<Value>,
        when: Option<WhenCondition>,
        from: Option<Value>,
        to: Option<Value>,
        td: Option<Value>,
        minval: Value,
    },

    /// Independent-axis value where a conditional event is first met.
    /// .MEAS DC name WHEN left=right
    When {
        condition: WhenCondition,
        from: Option<Value>,
        to: Option<Value>,
        td: Option<Value>,
        minval: Value,
    },

    /// Time-derivative of a signal at a point
    /// .MEAS TRAN name DERIV V(out) AT=time | WHEN sig=value
    Derivative {
        signal: String,
        at: Option<Value>,
        when: Option<WhenCondition>,
        from: Option<Value>,
        to: Option<Value>,
        td: Option<Value>,
        minval: Value,
    },

    /// Expression over previously evaluated measurement results
    /// .MEAS TRAN name PARAM='expr'
    Param { expression: String },

    /// Xyce continuous equation measure. `PARAM` and `EQN` are aliases in
    /// Xyce mode: the expression is evaluated at every accepted analysis
    /// point and its current value can be consumed by later equation measures
    /// and output probes.
    Equation {
        expression: String,
        from: Option<Value>,
        to: Option<Value>,
        td: Option<Value>,
    },

    /// Pointwise relative error between two accepted-point waveforms.
    ErrorFunction {
        measured: String,
        comparison: String,
        norm: ErrorFunctionNorm,
        from: Option<Value>,
        to: Option<Value>,
        minval: Value,
        ymin: Value,
        ymax: Value,
        /// Xyce parses WEIGHT but intentionally does not apply it here.
        weight: Option<Value>,
    },

    /// Difference norm between an accepted-point waveform and one column of
    /// an external Xyce PRN, CSV, or CSDF table.
    FileError {
        signal: String,
        file: String,
        norm: FileErrorNorm,
        /// Retained for non-DC interpolation support. Xyce deliberately
        /// ignores this option for DC measurements.
        independent_column: Option<isize>,
        dependent_column: usize,
    },

    /// Minimum value over range
    Min {
        signal: String,
        from: Option<Value>,
        to: Option<Value>,
        output: ExtremaOutput,
    },

    /// Maximum value over range
    Max {
        signal: String,
        from: Option<Value>,
        to: Option<Value>,
        output: ExtremaOutput,
    },

    /// Peak-to-peak (max - min) over range
    PeakToPeak {
        signal: String,
        from: Option<Value>,
        to: Option<Value>,
    },

    /// Average value over range
    Avg {
        signal: String,
        from: Option<Value>,
        to: Option<Value>,
    },

    /// RMS value over range
    Rms {
        signal: String,
        from: Option<Value>,
        to: Option<Value>,
    },

    /// Rise time (10% to 90% by default)
    RiseTime {
        signal: String,
        from_pct: Value, // e.g., 0.1 for 10%
        to_pct: Value,   // e.g., 0.9 for 90%
        number: usize,   // Which transition
    },

    /// Fall time (90% to 10% by default)
    FallTime {
        signal: String,
        from_pct: Value,
        to_pct: Value,
        number: usize,
    },

    /// Integral of signal over range
    Integ {
        signal: String,
        from: Option<Value>,
        to: Option<Value>,
    },
}

/// Per-statement Xyce measurement-output routing.
///
/// This is independent of whether the measurement is evaluated. It controls
/// whether a successful or failed result is emitted to the aggregate
/// measurement file, standard output, both, or neither.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MeasurePrintPolicy {
    /// Emit to both the aggregate measurement file and standard output.
    #[default]
    All,
    /// Emit only to standard output.
    Stdout,
    /// Suppress both aggregate-file and standard-output emission.
    None,
}

/// A complete measurement statement
#[derive(Debug, Clone)]
pub struct MeasureStatement {
    /// Measurement name (result variable)
    pub name: String,
    /// Type of measurement
    pub measure_type: MeasureType,
    /// Analysis type (TRAN, AC, DC)
    pub analysis: String,
    /// Expected value (`GOAL=`): a computed value that misses the goal
    /// fails the measurement.
    pub goal: Option<Value>,
    /// Allowed deviation from the goal (`TOL=`). Defaults to
    /// max(1% of |goal|, 1e-12) when a goal is given without a tolerance.
    pub tolerance: Option<Value>,
    /// Per-statement Xyce `DEFAULT_VAL`. The global
    /// `.OPTIONS MEASURE DEFAULT_VAL` setting takes precedence when present.
    pub default_value: Option<Value>,
    /// Per-statement Xyce `PRINT=ALL|STDOUT|NONE` output policy.
    pub print_policy: MeasurePrintPolicy,
}

impl MeasureStatement {
    /// Normalize dialect-specific node-zero aliases in execution-facing
    /// waveform and expression fields. Authored spelling remains available in
    /// the netlist source and output-provenance sidecar.
    pub(crate) fn apply_ground_policy(&mut self, policy: GroundPolicy) {
        fn rewrite(value: &mut String, policy: GroundPolicy) {
            *value = apply_ground_policy_to_probe_references(value, policy);
        }

        fn rewrite_operand(operand: &mut MeasureOperand, policy: GroundPolicy) {
            if let MeasureOperand::Waveform(value) = operand {
                rewrite(value, policy);
            }
        }

        fn rewrite_condition(condition: &mut WhenCondition, policy: GroundPolicy) {
            rewrite(&mut condition.left, policy);
            rewrite_operand(&mut condition.right, policy);
        }

        fn rewrite_trigger(trigger: &mut TrigSpec, policy: GroundPolicy) {
            if let TriggerEvent::When(condition) = &mut trigger.event {
                rewrite_condition(condition, policy);
            }
        }

        match &mut self.measure_type {
            MeasureType::Delay { trig, targ, .. } => {
                rewrite_trigger(trig, policy);
                rewrite_trigger(targ, policy);
            }
            MeasureType::Find { signal, when, .. }
            | MeasureType::Derivative { signal, when, .. } => {
                rewrite(signal, policy);
                if let Some(condition) = when {
                    rewrite_condition(condition, policy);
                }
            }
            MeasureType::When { condition, .. } => rewrite_condition(condition, policy),
            MeasureType::Param { expression } | MeasureType::Equation { expression, .. } => {
                rewrite(expression, policy);
            }
            MeasureType::ErrorFunction {
                measured,
                comparison,
                ..
            } => {
                rewrite(measured, policy);
                rewrite(comparison, policy);
            }
            MeasureType::FileError { signal, .. }
            | MeasureType::Min { signal, .. }
            | MeasureType::Max { signal, .. }
            | MeasureType::PeakToPeak { signal, .. }
            | MeasureType::Avg { signal, .. }
            | MeasureType::Rms { signal, .. }
            | MeasureType::RiseTime { signal, .. }
            | MeasureType::FallTime { signal, .. }
            | MeasureType::Integ { signal, .. } => rewrite(signal, policy),
        }
    }
}

