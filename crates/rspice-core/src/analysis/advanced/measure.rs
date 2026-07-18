//! Measurement Engine for SPICE .MEAS statements
//!
//! Provides automated measurement extraction from simulation results including:
//! - Rise/fall time measurements
//! - Delay measurements (TRIG/TARG)
//! - Min/Max/Peak-to-peak values
//! - Average and RMS calculations
//! - FIND...WHEN conditional measurements
//!
//! # Example
//! ```ignore
//! .MEAS TRAN delay TRIG V(in) VAL=0.5 RISE=1 TARG V(out) VAL=0.5 RISE=1
//! .MEAS TRAN rise_time TRIG V(out) VAL=0.1 RISE=1 TARG V(out) VAL=0.9 RISE=1
//! .MEAS TRAN vmax MAX V(out)
//! .MEAS TRAN vpp PP V(out)
//! .MEAS TRAN vavg AVG V(out)
//! ```

#![allow(clippy::too_many_arguments)]
use crate::Value;
use std::collections::HashMap;

//=============================================================================
// Measurement Types
//=============================================================================

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
    /// Absolute equality tolerance used when detecting the conditional event.
    pub minval: Value,
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
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
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
    Delay { trig: TrigSpec, targ: TrigSpec },

    /// Find value at specific time or when condition is met
    /// .MEAS TRAN name FIND V(out) AT=time
    /// .MEAS TRAN name FIND V(out) WHEN V(in)=0.5
    Find {
        signal: String,
        at: Option<Value>,
        when: Option<WhenCondition>,
        from: Option<Value>,
        to: Option<Value>,
    },

    /// Independent-axis value where a conditional event is first met.
    /// .MEAS DC name WHEN left=right
    When {
        condition: WhenCondition,
        from: Option<Value>,
        to: Option<Value>,
    },

    /// Time-derivative of a signal at a point
    /// .MEAS TRAN name DERIV V(out) AT=time | WHEN sig=value
    Derivative {
        signal: String,
        at: Option<Value>,
        when: Option<WhenCondition>,
        from: Option<Value>,
        to: Option<Value>,
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
    pub(crate) fn apply_ground_policy(&mut self, policy: crate::netlist::GroundPolicy) {
        fn rewrite(value: &mut String, policy: crate::netlist::GroundPolicy) {
            *value = crate::netlist::apply_ground_policy_to_probe_references(value, policy);
        }

        fn rewrite_operand(operand: &mut MeasureOperand, policy: crate::netlist::GroundPolicy) {
            if let MeasureOperand::Waveform(value) = operand {
                rewrite(value, policy);
            }
        }

        fn rewrite_condition(condition: &mut WhenCondition, policy: crate::netlist::GroundPolicy) {
            rewrite(&mut condition.left, policy);
            rewrite_operand(&mut condition.right, policy);
        }

        fn rewrite_trigger(trigger: &mut TrigSpec, policy: crate::netlist::GroundPolicy) {
            if let TriggerEvent::When(condition) = &mut trigger.event {
                rewrite_condition(condition, policy);
            }
        }

        match &mut self.measure_type {
            MeasureType::Delay { trig, targ } => {
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

/// Result of a measurement
#[derive(Debug, Clone)]
pub struct MeasureResult {
    /// Measurement name
    pub name: String,
    /// Computed value (None if measurement failed)
    pub value: Option<Value>,
    /// Error message if failed
    pub error: Option<String>,
    /// Whether the measurement passed: a value was computed and, when a
    /// GOAL was declared, it landed within tolerance.
    pub passed: bool,
    /// The declared goal, when the statement carried one.
    pub expected: Option<Value>,
    /// The effective tolerance applied to the goal check.
    pub tolerance: Option<Value>,
    /// Independent-axis location associated with a point or extrema result.
    ///
    /// Xyce reports this metadata alongside scalar `AT`, `WHEN`, `MIN`, and
    /// `MAX` measurements. Keeping it typed prevents output adapters and
    /// regression oracles from reverse-engineering the event from a value.
    pub event_axis: Option<Value>,
}

impl MeasureResult {
    pub fn success(name: &str, value: Value) -> Self {
        if !value.is_finite() {
            return Self::failed(name, &format!("measurement value is non-finite ({value})"));
        }
        Self {
            name: name.to_string(),
            value: Some(value),
            error: None,
            passed: true,
            expected: None,
            tolerance: None,
            event_axis: None,
        }
    }

    pub fn failed(name: &str, error: &str) -> Self {
        Self {
            name: name.to_string(),
            value: None,
            error: Some(error.to_string()),
            passed: false,
            expected: None,
            tolerance: None,
            event_axis: None,
        }
    }

    fn with_event_axis(mut self, event_axis: Value) -> Self {
        if !event_axis.is_finite() {
            return Self::failed(
                &self.name,
                &format!("measurement event axis is non-finite ({event_axis})"),
            );
        }
        self.event_axis = Some(event_axis);
        self
    }

    /// Apply a statement's GOAL/TOL contract to a computed result.
    pub(super) fn check_goal(mut self, statement: &MeasureStatement) -> Self {
        let Some(goal) = statement.goal else {
            return self;
        };
        if !goal.is_finite() {
            self.passed = false;
            self.error = Some(format!("GOAL must be finite, got {goal}"));
            self.expected = Some(goal);
            self.tolerance = statement.tolerance;
            return self;
        }
        let tolerance = statement
            .tolerance
            .unwrap_or_else(|| (goal.abs() * 0.01).max(1e-12));
        self.expected = Some(goal);
        self.tolerance = Some(tolerance);
        if !tolerance.is_finite() || tolerance < 0.0 {
            self.passed = false;
            self.error = Some(format!(
                "TOL must be a finite non-negative value, got {tolerance}"
            ));
            return self;
        }
        if let Some(value) = self.value {
            if !value.is_finite() {
                self.passed = false;
                self.error = Some(format!("measurement value is non-finite ({value})"));
            } else if (value - goal).abs() > tolerance {
                self.passed = false;
                self.error = Some(format!(
                    "value {value:e} misses GOAL {goal:e} (tolerance {tolerance:e})"
                ));
            }
        }
        self
    }
}

/// One row produced by an Xyce continuous point-event measurement.
///
/// `value` is the value written to the measurement's output column.  Point
/// measurements retain their interpolated independent-axis location in
/// `event_axis`; delay measurements instead retain the independently matched
/// trigger and target locations.  Keeping this provenance avoids forcing
/// output adapters to reverse-engineer event locations from a scalar value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContinuousMeasureRecord {
    pub value: Value,
    pub event_axis: Option<Value>,
    pub trigger_axis: Option<Value>,
    pub target_axis: Option<Value>,
}

impl ContinuousMeasureRecord {
    fn point(value: Value, event_axis: Value) -> Self {
        Self {
            value,
            event_axis: Some(event_axis),
            trigger_axis: None,
            target_axis: None,
        }
    }

    fn delay(trigger_axis: Value, target_axis: Value) -> Self {
        Self {
            value: target_axis - trigger_axis,
            event_axis: None,
            trigger_axis: Some(trigger_axis),
            target_axis: Some(target_axis),
        }
    }
}

/// Vector-valued result of a `*_CONT` point-event measurement.
#[derive(Debug, Clone, PartialEq)]
pub struct ContinuousMeasureResult {
    pub name: String,
    pub records: Vec<ContinuousMeasureRecord>,
    pub failure: Option<String>,
    /// Xyce retains a successfully located TRIG or TARG endpoint even when
    /// its counterpart is absent and the delay measure is FAILED.
    pub failure_metadata: Option<ContinuousMeasureFailureMetadata>,
}

/// Partial event provenance retained for a failed continuous delay measure.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContinuousMeasureFailureMetadata {
    pub trigger_axis: Option<Value>,
    pub target_axis: Option<Value>,
}

impl ContinuousMeasureResult {
    fn success(name: &str, records: Vec<ContinuousMeasureRecord>) -> Self {
        Self {
            name: name.to_string(),
            records,
            failure: None,
            failure_metadata: None,
        }
    }

    fn failed(name: &str, failure: impl Into<String>) -> Self {
        Self {
            name: name.to_string(),
            records: Vec::new(),
            failure: Some(failure.into()),
            failure_metadata: None,
        }
    }

    fn failed_delay(
        name: &str,
        trigger_axis: Option<Value>,
        target_axis: Option<Value>,
        failure: impl Into<String>,
    ) -> Self {
        Self {
            name: name.to_string(),
            records: Vec::new(),
            failure: Some(failure.into()),
            failure_metadata: Some(ContinuousMeasureFailureMetadata {
                trigger_axis,
                target_axis,
            }),
        }
    }

    /// Validate the mutually exclusive success/failure representation and
    /// the finiteness of all published numeric metadata.
    pub fn validate_invariants(&self) -> Result<(), &'static str> {
        if self.name.trim().is_empty() {
            return Err("continuous measurement name is empty");
        }
        match self.failure.as_ref() {
            Some(failure) => {
                if failure.trim().is_empty() {
                    return Err("continuous measurement failure reason is empty");
                }
                if !self.records.is_empty() {
                    return Err("failed continuous measurement contains successful records");
                }
            }
            None => {
                if self.records.is_empty() {
                    return Err("successful continuous measurement contains no records");
                }
                if self.failure_metadata.is_some() {
                    return Err("successful continuous measurement contains failure metadata");
                }
            }
        }
        if self.records.iter().any(|record| {
            !record.value.is_finite()
                || record.event_axis.is_some_and(|value| !value.is_finite())
                || record.trigger_axis.is_some_and(|value| !value.is_finite())
                || record.target_axis.is_some_and(|value| !value.is_finite())
        }) {
            return Err("continuous measurement record contains a non-finite value");
        }
        if self.failure_metadata.is_some_and(|metadata| {
            metadata
                .trigger_axis
                .is_some_and(|value| !value.is_finite())
                || metadata.target_axis.is_some_and(|value| !value.is_finite())
        }) {
            return Err("continuous measurement failure metadata contains a non-finite value");
        }
        Ok(())
    }
}

//=============================================================================
// Measurement Engine
//=============================================================================

/// Resolve a signal by name, falling back to a case-insensitive scan.
/// SPICE netlists are case-insensitive, so `.MEAS ... V(OUT)` must find a
/// waveform stored under `V(out)`.
fn lookup_signal<'a>(signals: &HashMap<String, &'a [Value]>, name: &str) -> Option<&'a [Value]> {
    if let Some(signal) = signals.get(name) {
        return Some(signal);
    }
    signals
        .iter()
        .find_map(|(key, signal)| key.eq_ignore_ascii_case(name).then_some(*signal))
}

fn strictly_monotonic_direction(axis: &[Value]) -> Result<bool, String> {
    if axis.is_empty() {
        return Err("non-DC ERROR simulation axis is empty".to_string());
    }
    if axis.len() == 1 {
        return Ok(true);
    }
    let ascending = axis[1] > axis[0];
    if axis[1] == axis[0]
        || axis.windows(2).any(|window| {
            if ascending {
                window[1] <= window[0]
            } else {
                window[1] >= window[0]
            }
        })
    {
        return Err("non-DC ERROR simulation axis must be strictly monotonic".to_string());
    }
    Ok(ascending)
}

struct AkimaInterpolator {
    axis: Vec<Value>,
    signal: Vec<Value>,
    p1: Vec<Value>,
    p2: Vec<Value>,
    p3: Vec<Value>,
}

impl AkimaInterpolator {
    fn new(axis: &[Value], signal: &[Value]) -> Result<Self, String> {
        let ascending = strictly_monotonic_direction(axis)?;
        let mut axis = axis.to_vec();
        let mut signal = signal.to_vec();
        if !ascending {
            axis.reverse();
            signal.reverse();
        }
        let size = axis.len();
        let mut slopes = vec![0.0; size + 3];
        for index in 0..size.saturating_sub(1) {
            slopes[index + 2] =
                (signal[index + 1] - signal[index]) / (axis[index + 1] - axis[index]);
        }
        slopes[0] = 3.0 * slopes[2] - 2.0 * slopes[3];
        slopes[1] = 2.0 * slopes[2] - slopes[3];
        slopes[size + 1] = 2.0 * slopes[size] - slopes[size - 1];
        slopes[size + 2] = 3.0 * slopes[size] - 2.0 * slopes[size - 1];

        let derivatives = (0..size)
            .map(|index| {
                let upper_weight = (slopes[index + 3] - slopes[index + 2]).abs();
                let lower_weight = (slopes[index + 1] - slopes[index]).abs();
                if upper_weight + lower_weight == 0.0 {
                    0.5 * (slopes[index + 1] + slopes[index + 2])
                } else {
                    (upper_weight * slopes[index + 1] + lower_weight * slopes[index + 2])
                        / (upper_weight + lower_weight)
                }
            })
            .collect::<Vec<_>>();
        let mut p1 = vec![0.0; size];
        let mut p2 = vec![0.0; size];
        let mut p3 = vec![0.0; size];
        for index in 0..size.saturating_sub(1) {
            let dx = axis[index + 1] - axis[index];
            p1[index] = derivatives[index];
            p2[index] =
                (3.0 * slopes[index + 2] - 2.0 * derivatives[index] - derivatives[index + 1]) / dx;
            p3[index] =
                (derivatives[index] + derivatives[index + 1] - 2.0 * slopes[index + 2]) / (dx * dx);
        }
        Ok(Self {
            axis,
            signal,
            p1,
            p2,
            p3,
        })
    }

    fn evaluate(&self, target: Value) -> Value {
        if self.axis.len() == 1 {
            return self.signal[0];
        }
        let mut lower = 0usize;
        let mut upper = self.axis.len() - 1;
        while upper > lower + 1 {
            let middle = (upper + lower) >> 1;
            if self.axis[middle] > target {
                upper = middle;
            } else {
                lower = middle;
            }
        }
        let delta = target - self.axis[lower];
        self.signal[lower]
            + delta * (self.p1[lower] + delta * (self.p2[lower] + self.p3[lower] * delta))
    }
}

/// Engine for processing .MEAS statements on simulation results
pub struct MeasureEngine {
    /// Registered measurements
    measurements: Vec<MeasureStatement>,
}

impl MeasureEngine {
    pub fn new() -> Self {
        Self {
            measurements: Vec::new(),
        }
    }

    /// Add a measurement to be evaluated
    pub fn add(&mut self, measurement: MeasureStatement) {
        self.measurements.push(measurement);
    }

    /// Evaluate all measurements against simulation data
    ///
    /// # Arguments
    /// * `time` - Time points array
    /// * `signals` - Map of signal name to waveform data
    ///
    /// # Returns
    /// Vector of measurement results
    pub fn evaluate(
        &self,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
    ) -> Vec<MeasureResult> {
        self.evaluate_with_segment_starts(time, signals, &[])
    }

    /// Evaluate measurements while treating selected sample indices as the
    /// start of a new accepted-point segment. This is required for nested DC
    /// sweeps: the primary sweep restarts for every secondary value, and the
    /// synthetic jump between cycles is not a physical interpolation interval.
    pub fn evaluate_with_segment_starts(
        &self,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
        segment_starts: &[usize],
    ) -> Vec<MeasureResult> {
        self.evaluate_with_signal_maps(time, &[signals], segment_starts)
    }

    /// Evaluate vector-valued Xyce continuous point-event measurements.
    ///
    /// Unlike an ordinary measurement, a positive RISE/FALL/CROSS occurrence
    /// is a starting index: the selected event and every later qualifying
    /// event are emitted.  A negative occurrence selects exactly one event
    /// counting backward from the end.  This method deliberately accepts only
    /// the four point-event forms supported by Xyce's `*_CONT` modes.
    pub fn evaluate_continuous(
        &self,
        axis: &[Value],
        signals: &HashMap<String, &[Value]>,
        segment_starts: &[usize],
    ) -> Vec<ContinuousMeasureResult> {
        if axis.is_empty() {
            return self
                .measurements
                .iter()
                .map(|statement| {
                    ContinuousMeasureResult::failed(&statement.name, "measurement axis is empty")
                })
                .collect();
        }
        if let Some(index) = axis.iter().position(|value| !value.is_finite()) {
            return self
                .measurements
                .iter()
                .map(|statement| {
                    ContinuousMeasureResult::failed(
                        &statement.name,
                        format!("measurement axis contains non-finite sample at index {index}"),
                    )
                })
                .collect();
        }
        if let Some((name, signal)) = signals
            .iter()
            .find(|(_, signal)| signal.len() != axis.len())
        {
            return self
                .measurements
                .iter()
                .map(|statement| {
                    ContinuousMeasureResult::failed(
                        &statement.name,
                        format!(
                            "signal '{name}' has {} samples but measurement axis has {}",
                            signal.len(),
                            axis.len()
                        ),
                    )
                })
                .collect();
        }
        if let Some((name, index, value)) = signals.iter().find_map(|(name, signal)| {
            signal
                .iter()
                .enumerate()
                .find(|(_, value)| !value.is_finite())
                .map(|(index, value)| (name, index, *value))
        }) {
            return self
                .measurements
                .iter()
                .map(|statement| {
                    ContinuousMeasureResult::failed(
                        &statement.name,
                        format!(
                            "signal '{name}' contains non-finite sample at index {index}: {value}"
                        ),
                    )
                })
                .collect();
        }
        if segment_starts.iter().enumerate().any(|(index, start)| {
            *start == 0 || *start >= axis.len() || index > 0 && *start <= segment_starts[index - 1]
        }) {
            return self
                .measurements
                .iter()
                .map(|statement| {
                    ContinuousMeasureResult::failed(
                        &statement.name,
                        "measurement segment starts are invalid or unordered",
                    )
                })
                .collect();
        }

        self.measurements
            .iter()
            .map(|statement| {
                if !matches!(
                    statement.analysis.to_ascii_uppercase().as_str(),
                    "TRAN_CONT" | "DC_CONT" | "AC_CONT" | "NOISE_CONT"
                ) {
                    return ContinuousMeasureResult::failed(
                        &statement.name,
                        format!(
                            "continuous evaluation requires TRAN_CONT, DC_CONT, AC_CONT, or NOISE_CONT, got {}",
                            statement.analysis
                        ),
                    );
                }
                self.evaluate_continuous_one(statement, axis, signals, segment_starts)
            })
            .collect()
    }

    fn evaluate_continuous_one(
        &self,
        statement: &MeasureStatement,
        axis: &[Value],
        signals: &HashMap<String, &[Value]>,
        segment_starts: &[usize],
    ) -> ContinuousMeasureResult {
        match &statement.measure_type {
            MeasureType::When {
                condition,
                from,
                to,
            } => continuous_when(
                &statement.name,
                condition,
                *from,
                *to,
                axis,
                signals,
                segment_starts,
            ),
            MeasureType::Find {
                signal,
                at,
                when,
                from,
                to,
            } => continuous_find(
                &statement.name,
                signal,
                *at,
                when.as_ref(),
                *from,
                *to,
                axis,
                signals,
                segment_starts,
            ),
            MeasureType::Derivative {
                signal,
                at,
                when,
                from,
                to,
            } => continuous_derivative(
                &statement.name,
                signal,
                *at,
                when.as_ref(),
                *from,
                *to,
                axis,
                signals,
                segment_starts,
            ),
            MeasureType::Delay { trig, targ } => {
                continuous_delay(&statement.name, trig, targ, axis, signals, segment_starts)
            }
            _ => ContinuousMeasureResult::failed(
                &statement.name,
                "continuous measures support only WHEN, FIND, DERIV, and TRIG/TARG",
            ),
        }
    }

    /// Evaluate each statement against its own signal view. Continuous Xyce
    /// equation measures update in netlist order, so a point-event statement
    /// can legitimately observe a different equation trace depending on its
    /// position relative to the equation statement.
    pub(crate) fn evaluate_with_segment_starts_and_signal_maps(
        &self,
        time: &[Value],
        signal_maps: &[HashMap<String, &[Value]>],
        segment_starts: &[usize],
    ) -> Vec<MeasureResult> {
        let signal_map_refs = signal_maps.iter().collect::<Vec<_>>();
        self.evaluate_with_signal_maps(time, &signal_map_refs, segment_starts)
    }

    fn evaluate_with_signal_maps(
        &self,
        time: &[Value],
        signal_maps: &[&HashMap<String, &[Value]>],
        segment_starts: &[usize],
    ) -> Vec<MeasureResult> {
        if self.measurements.is_empty() {
            return Vec::new();
        }
        if signal_maps.len() != 1 && signal_maps.len() != self.measurements.len() {
            return self.fail_all("measurement signal-map count does not match statement count");
        }
        if time.is_empty() {
            return self.fail_all("measurement axis is empty");
        }
        if let Some(index) = time.iter().position(|value| !value.is_finite()) {
            return self.fail_all(&format!(
                "measurement axis contains non-finite sample at index {index}"
            ));
        }
        if let Some((name, signal)) = signal_maps
            .iter()
            .flat_map(|signals| signals.iter())
            .find(|(_, signal)| signal.len() != time.len())
        {
            return self.fail_all(&format!(
                "signal '{name}' has {} samples but measurement axis has {}",
                signal.len(),
                time.len()
            ));
        }
        if let Some((name, index, value)) = signal_maps
            .iter()
            .flat_map(|signals| signals.iter())
            .find_map(|(name, signal)| {
                signal
                    .iter()
                    .enumerate()
                    .find(|(_, value)| !value.is_finite())
                    .map(|(index, value)| (name, index, *value))
            })
        {
            return self.fail_all(&format!(
                "signal '{name}' contains non-finite sample at index {index}: {value}"
            ));
        }
        if segment_starts.iter().enumerate().any(|(index, start)| {
            *start == 0 || *start >= time.len() || index > 0 && *start <= segment_starts[index - 1]
        }) {
            return self.fail_all("measurement segment starts are invalid or unordered");
        }

        // Expression measures (PARAM='...') read other results by name, so
        // they evaluate in a second pass over the directly computed set —
        // and in statement order, so a PARAM may reference an earlier PARAM.
        let mut results: Vec<MeasureResult> = self
            .measurements
            .iter()
            .enumerate()
            .map(|(index, m)| match &m.measure_type {
                MeasureType::Param { .. } | MeasureType::Equation { .. } => {
                    MeasureResult::failed(&m.name, "PARAM expression not yet evaluated")
                }
                _ => {
                    let signals = if signal_maps.len() == 1 {
                        signal_maps[0]
                    } else {
                        signal_maps[index]
                    };
                    self.evaluate_one(m, time, signals, segment_starts)
                }
            })
            .collect();
        for (idx, m) in self.measurements.iter().enumerate() {
            if let MeasureType::Param { expression } = &m.measure_type {
                results[idx] = self.eval_param(&m.name, expression, &results).check_goal(m);
            }
        }
        results
    }

    fn fail_all(&self, reason: &str) -> Vec<MeasureResult> {
        self.measurements
            .iter()
            .map(|measurement| MeasureResult::failed(&measurement.name, reason))
            .collect()
    }

    fn evaluate_one(
        &self,
        measurement: &MeasureStatement,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
        segment_starts: &[usize],
    ) -> MeasureResult {
        self.evaluate_kind(measurement, time, signals, segment_starts)
            .check_goal(measurement)
    }

    fn evaluate_kind(
        &self,
        measurement: &MeasureStatement,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
        segment_starts: &[usize],
    ) -> MeasureResult {
        match &measurement.measure_type {
            MeasureType::Delay { trig, targ } => {
                self.eval_delay(&measurement.name, trig, targ, time, signals, segment_starts)
            }
            MeasureType::Derivative {
                signal,
                at,
                when,
                from,
                to,
            } => self.eval_derivative(
                &measurement.name,
                signal,
                *at,
                when.as_ref(),
                *from,
                *to,
                time,
                signals,
                segment_starts,
            ),
            MeasureType::Param { .. } => MeasureResult::failed(
                &measurement.name,
                "PARAM measures evaluate after the directly computed set",
            ),
            MeasureType::Equation { .. } => MeasureResult::failed(
                &measurement.name,
                "continuous equation measures evaluate on the analysis-point stream",
            ),
            MeasureType::ErrorFunction {
                measured,
                comparison,
                norm,
                from,
                to,
                minval,
                ymin,
                ymax,
                ..
            } => self.eval_error_function(
                &measurement.name,
                &measurement.analysis,
                measured,
                comparison,
                *norm,
                *from,
                *to,
                *minval,
                *ymin,
                *ymax,
                time,
                signals,
            ),
            MeasureType::FileError {
                signal,
                file,
                norm,
                independent_column,
                dependent_column,
            } => self.eval_file_error(
                &measurement.name,
                &measurement.analysis,
                signal,
                file,
                *norm,
                *independent_column,
                *dependent_column,
                time,
                signals,
            ),
            MeasureType::Min {
                signal,
                from,
                to,
                output,
            } => self.eval_min_max(
                &measurement.name,
                signal,
                *from,
                *to,
                *output,
                time,
                signals,
                false,
            ),
            MeasureType::Max {
                signal,
                from,
                to,
                output,
            } => self.eval_min_max(
                &measurement.name,
                signal,
                *from,
                *to,
                *output,
                time,
                signals,
                true,
            ),
            MeasureType::PeakToPeak { signal, from, to } => {
                self.eval_pp(&measurement.name, signal, *from, *to, time, signals)
            }
            MeasureType::Avg { signal, from, to } => {
                self.eval_avg(&measurement.name, signal, *from, *to, time, signals)
            }
            MeasureType::Rms { signal, from, to } => {
                self.eval_rms(&measurement.name, signal, *from, *to, time, signals)
            }
            MeasureType::RiseTime {
                signal,
                from_pct,
                to_pct,
                number,
            } => self.eval_rise_fall(
                &measurement.name,
                signal,
                *from_pct,
                *to_pct,
                *number,
                true,
                time,
                signals,
            ),
            MeasureType::FallTime {
                signal,
                from_pct,
                to_pct,
                number,
            } => self.eval_rise_fall(
                &measurement.name,
                signal,
                *from_pct,
                *to_pct,
                *number,
                false,
                time,
                signals,
            ),
            MeasureType::Find {
                signal,
                at,
                when,
                from,
                to,
            } => self.eval_find(
                &measurement.name,
                signal,
                *at,
                when.as_ref(),
                *from,
                *to,
                time,
                signals,
                segment_starts,
            ),
            MeasureType::When {
                condition,
                from,
                to,
            } => self.eval_when(
                &measurement.name,
                condition,
                *from,
                *to,
                time,
                signals,
                segment_starts,
            ),
            MeasureType::Integ { signal, from, to } => self.eval_integ(
                &measurement.name,
                &measurement.analysis,
                signal,
                *from,
                *to,
                time,
                signals,
            ),
        }
    }

    fn eval_file_error(
        &self,
        name: &str,
        analysis: &str,
        signal_name: &str,
        file: &str,
        norm: FileErrorNorm,
        independent_column: Option<isize>,
        dependent_column: usize,
        axis: &[Value],
        signals: &HashMap<String, &[Value]>,
    ) -> MeasureResult {
        if !matches!(
            analysis.to_ascii_uppercase().as_str(),
            "DC" | "TRAN" | "AC" | "NOISE"
        ) {
            return MeasureResult::failed(
                name,
                "file-backed ERROR is supported only for DC, TRAN, AC, and NOISE analyses",
            );
        }
        let Some(signal) = lookup_signal(signals, signal_name) else {
            return MeasureResult::failed(name, &format!("Signal '{signal_name}' not found"));
        };
        let pairs = if analysis.eq_ignore_ascii_case("DC") {
            let comparison =
                match super::measure_file::read_error_comparison_column(file, dependent_column) {
                    Ok(values) => values,
                    Err(error) => {
                        return MeasureResult::failed(
                            name,
                            &format!("could not load ERROR comparison file '{file}': {error}"),
                        );
                    }
                };
            if signal.len() < comparison.len() {
                return MeasureResult::failed(
                    name,
                    &format!(
                        "ERROR comparison has {} rows but the simulation produced only {} accepted points",
                        comparison.len(),
                        signal.len()
                    ),
                );
            }
            signal.iter().copied().zip(comparison).collect::<Vec<_>>()
        } else {
            let Some(independent_column) = independent_column else {
                return MeasureResult::failed(
                    name,
                    "non-DC ERROR requires a non-negative INDEPVARCOL",
                );
            };
            let Ok(independent_column) = usize::try_from(independent_column) else {
                return MeasureResult::failed(
                    name,
                    "non-DC ERROR requires a non-negative INDEPVARCOL",
                );
            };
            if independent_column == dependent_column {
                return MeasureResult::failed(
                    name,
                    "non-DC ERROR requires different INDEPVARCOL and DEPVARCOL values",
                );
            }
            let comparison = match super::measure_file::read_error_comparison_columns(
                file,
                Some(independent_column),
                dependent_column,
            ) {
                Ok(columns) => columns,
                Err(error) => {
                    return MeasureResult::failed(
                        name,
                        &format!("could not load ERROR comparison file '{file}': {error}"),
                    );
                }
            };
            let reference_axis = comparison
                .independent
                .expect("requested ERROR independent column must be returned");
            if reference_axis.first().is_some_and(|value| *value < 0.0)
                || reference_axis
                    .windows(2)
                    .any(|window| window[1] < window[0])
            {
                return MeasureResult::failed(
                    name,
                    "non-DC ERROR comparison axis must be monotonically increasing and non-negative",
                );
            }
            let interpolator = match AkimaInterpolator::new(axis, signal) {
                Ok(interpolator) => interpolator,
                Err(error) => return MeasureResult::failed(name, &error),
            };
            reference_axis
                .into_iter()
                .zip(comparison.dependent)
                .map(|(reference_axis_value, reference)| {
                    (interpolator.evaluate(reference_axis_value), reference)
                })
                .collect()
        };

        let mut l1 = 0.0;
        let mut l1_compensation = 0.0;
        let mut l2: Value = 0.0;
        let mut infinity: Value = 0.0;
        for (simulated, reference) in pairs {
            let difference = simulated - reference;
            if !difference.is_finite() {
                return MeasureResult::failed(name, "ERROR difference vector is non-finite");
            }
            let magnitude = difference.abs();
            infinity = infinity.max(magnitude);
            // Neumaier-compensated L1 and hypot-folded L2 preserve useful
            // precision and avoid intermediate square overflow/underflow.
            let next_l1 = l1 + magnitude;
            if l1.abs() >= magnitude {
                l1_compensation += (l1 - next_l1) + magnitude;
            } else {
                l1_compensation += (magnitude - next_l1) + l1;
            }
            l1 = next_l1;
            l2 = l2.hypot(difference);
        }
        let value = match norm {
            FileErrorNorm::Infinity => infinity,
            FileErrorNorm::L1 => l1 + l1_compensation,
            FileErrorNorm::L2 => l2,
        };
        MeasureResult::success(name, value)
    }

    /// Find time when signal crosses threshold
    fn find_crossing(
        &self,
        time: &[Value],
        signal: &[Value],
        threshold: Value,
        edge: EdgeType,
        occurrence: usize,
        start_at: Option<Value>,
    ) -> Option<Value> {
        let mut count = 0;

        for i in 1..signal.len() {
            if let Some(start_time) = start_at
                && time[i] < start_time
            {
                continue;
            }

            let prev = signal[i - 1];
            let curr = signal[i];
            let crossed = match edge {
                EdgeType::Rise => prev < threshold && curr >= threshold,
                EdgeType::Fall => prev > threshold && curr <= threshold,
                EdgeType::Cross => {
                    (prev < threshold && curr >= threshold)
                        || (prev > threshold && curr <= threshold)
                }
            };

            if crossed {
                // Linear interpolation for exact crossing time.
                let frac = (threshold - prev) / (curr - prev);
                let crossing_time = time[i - 1] + frac * (time[i] - time[i - 1]);

                if let Some(start_time) = start_at
                    && crossing_time < start_time
                {
                    continue;
                }

                count += 1;
                if count == occurrence {
                    return Some(crossing_time);
                }
            }
        }

        None
    }

    fn eval_delay(
        &self,
        name: &str,
        trig: &TrigSpec,
        targ: &TrigSpec,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
        segment_starts: &[usize],
    ) -> MeasureResult {
        let target_td = targ.td.or(trig.td);
        let t_trig = match delay_clause_event(trig, trig.td, time, signals, segment_starts) {
            Ok(Some(value)) => value,
            Ok(None) => return MeasureResult::failed(name, "Trigger condition not found"),
            Err(error) => return MeasureResult::failed(name, &error),
        };
        let t_targ = match delay_clause_event(targ, target_td, time, signals, segment_starts) {
            Ok(Some(value)) => value,
            Ok(None) => return MeasureResult::failed(name, "Target condition not found"),
            Err(error) => return MeasureResult::failed(name, &error),
        };

        MeasureResult::success(name, t_targ - t_trig)
    }

    fn measurement_window_bounds(
        time: &[Value],
        from: Option<Value>,
        to: Option<Value>,
    ) -> (Value, Value) {
        let ascending = time
            .windows(2)
            .find_map(|pair| (pair[0] != pair[1]).then_some(pair[1] > pair[0]))
            .unwrap_or(true);
        match (from, to) {
            (Some(from), Some(to)) => (from, to),
            (Some(from), None) if ascending => (from, Value::INFINITY),
            (Some(from), None) => (Value::NEG_INFINITY, from),
            (None, Some(to)) if ascending => (Value::NEG_INFINITY, to),
            (None, Some(to)) => (to, Value::INFINITY),
            (None, None) => (Value::NEG_INFINITY, Value::INFINITY),
        }
    }

    fn axis_in_measurement_window(axis: Value, lower: Value, upper: Value) -> bool {
        let lower_tolerance = if lower.is_finite() {
            lower.abs() * 1.0e-12
        } else {
            0.0
        };
        let upper_tolerance = if upper.is_finite() {
            upper.abs() * 1.0e-12
        } else {
            0.0
        };
        axis >= lower - lower_tolerance && axis <= upper + upper_tolerance
    }

    fn eval_min_max(
        &self,
        name: &str,
        signal_name: &str,
        from: Option<Value>,
        to: Option<Value>,
        output: ExtremaOutput,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
        is_max: bool,
    ) -> MeasureResult {
        let signal = match lookup_signal(signals, signal_name) {
            Some(s) => s,
            None => {
                return MeasureResult::failed(name, &format!("Signal '{}' not found", signal_name));
            }
        };

        let (lower, upper) = Self::measurement_window_bounds(time, from, to);
        let mut selected: Option<(usize, Value)> = None;
        for (index, (&axis, &value)) in time.iter().zip(signal).enumerate() {
            if !Self::axis_in_measurement_window(axis, lower, upper) {
                continue;
            }
            let replaces = selected.is_none_or(|(_, selected_value)| {
                if is_max {
                    value > selected_value
                } else {
                    value < selected_value
                }
            });
            if replaces {
                selected = Some((index, value));
            }
        }
        let Some((selected_index, selected_value)) = selected else {
            return MeasureResult::failed(name, "Empty range");
        };
        let result = match output {
            ExtremaOutput::Value => selected_value,
            ExtremaOutput::IndependentAxis => time[selected_index],
        };

        MeasureResult::success(name, result).with_event_axis(time[selected_index])
    }

    fn eval_pp(
        &self,
        name: &str,
        signal_name: &str,
        from: Option<Value>,
        to: Option<Value>,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
    ) -> MeasureResult {
        let signal = match lookup_signal(signals, signal_name) {
            Some(s) => s,
            None => {
                return MeasureResult::failed(name, &format!("Signal '{}' not found", signal_name));
            }
        };

        let (lower, upper) = Self::measurement_window_bounds(time, from, to);
        let mut min_val = Value::INFINITY;
        let mut max_val = Value::NEG_INFINITY;
        let mut selected = false;
        for (&axis, &value) in time.iter().zip(signal) {
            if Self::axis_in_measurement_window(axis, lower, upper) {
                min_val = min_val.min(value);
                max_val = max_val.max(value);
                selected = true;
            }
        }
        if !selected {
            return MeasureResult::failed(name, "Empty range");
        }

        MeasureResult::success(name, max_val - min_val)
    }

    #[allow(clippy::too_many_arguments)]
    fn eval_error_function(
        &self,
        name: &str,
        analysis: &str,
        measured_name: &str,
        comparison_name: &str,
        norm: ErrorFunctionNorm,
        from: Option<Value>,
        to: Option<Value>,
        minval: Value,
        ymin: Value,
        ymax: Value,
        axis: &[Value],
        signals: &HashMap<String, &[Value]>,
    ) -> MeasureResult {
        if !minval.is_finite() || !ymin.is_finite() || !ymax.is_finite() {
            return MeasureResult::failed(name, "ERR limits must be finite");
        }
        let Some(measured) = lookup_signal(signals, measured_name) else {
            return MeasureResult::failed(name, &format!("Signal '{measured_name}' not found"));
        };
        let Some(comparison) = lookup_signal(signals, comparison_name) else {
            return MeasureResult::failed(name, &format!("Signal '{comparison_name}' not found"));
        };
        let (lower, upper) = if analysis.eq_ignore_ascii_case("DC") {
            match (from, to) {
                (Some(from), Some(to)) => (from.min(to), from.max(to)),
                _ => Self::measurement_window_bounds(axis, from, to),
            }
        } else {
            Self::measurement_window_bounds(axis, from, to)
        };
        let mut sum = 0.0;
        let mut count = 0usize;
        for ((&axis_value, &measured_value), &comparison_value) in
            axis.iter().zip(measured).zip(comparison)
        {
            let magnitude = measured_value.abs();
            let ymin_tolerance = ymin.abs() * 1.0e-12;
            let ymax_tolerance = ymax.abs() * 1.0e-12;
            if !axis_in_error_window(axis_value, lower, upper, minval)
                || magnitude < ymin - ymin_tolerance
                || magnitude > ymax + ymax_tolerance
            {
                continue;
            }
            let denominator = magnitude.max(minval);
            if denominator <= 0.0 {
                return MeasureResult::failed(name, "ERR relative-error denominator is zero");
            }
            let relative_error = (measured_value - comparison_value) / denominator;
            sum += match norm {
                ErrorFunctionNorm::RootMeanSquare => relative_error * relative_error,
                ErrorFunctionNorm::MeanAbsolute => relative_error.abs(),
            };
            count += 1;
        }
        if count == 0 {
            return MeasureResult::failed(name, "ERR window contains no qualifying points");
        }
        let mean = sum / count as Value;
        let result = match norm {
            ErrorFunctionNorm::RootMeanSquare => mean.sqrt(),
            ErrorFunctionNorm::MeanAbsolute => mean,
        };
        if result.is_finite() {
            MeasureResult::success(name, result)
        } else {
            MeasureResult::failed(name, "ERR result is non-finite")
        }
    }

    fn eval_avg(
        &self,
        name: &str,
        signal_name: &str,
        from: Option<Value>,
        to: Option<Value>,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
    ) -> MeasureResult {
        let signal = match lookup_signal(signals, signal_name) {
            Some(s) => s,
            None => {
                return MeasureResult::failed(name, &format!("Signal '{}' not found", signal_name));
            }
        };

        let ascending = time
            .first()
            .zip(time.last())
            .is_none_or(|(first, last)| last >= first);
        let (lower, upper) = match (from, to) {
            (Some(from), Some(to)) => (from, to),
            (Some(from), None) if ascending => (from, Value::INFINITY),
            (Some(from), None) => (Value::NEG_INFINITY, from),
            (None, Some(to)) if ascending => (Value::NEG_INFINITY, to),
            (None, Some(to)) => (to, Value::INFINITY),
            (None, None) => (Value::NEG_INFINITY, Value::INFINITY),
        };
        if lower > upper {
            return MeasureResult::failed(name, "Empty range");
        }
        let mut integral = 0.0;
        let mut width = 0.0;
        let mut previous = None;
        for (&axis, &value) in time.iter().zip(signal) {
            let lower_tolerance = if lower.is_finite() {
                lower.abs() * 1.0e-12
            } else {
                0.0
            };
            let upper_tolerance = if upper.is_finite() {
                upper.abs() * 1.0e-12
            } else {
                0.0
            };
            if axis >= lower - lower_tolerance && axis <= upper + upper_tolerance {
                if let Some((previous_axis, previous_value)) = previous {
                    let dx = Value::abs(axis - previous_axis);
                    integral += 0.5 * (value + previous_value) * dx;
                    width += dx;
                }
                previous = Some((axis, value));
            } else {
                previous = None;
            }
        }
        if width == 0.0 {
            MeasureResult::failed(name, "Empty range")
        } else {
            MeasureResult::success(name, integral / width)
        }
    }

    fn eval_rms(
        &self,
        name: &str,
        signal_name: &str,
        from: Option<Value>,
        to: Option<Value>,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
    ) -> MeasureResult {
        let signal = match lookup_signal(signals, signal_name) {
            Some(s) => s,
            None => {
                return MeasureResult::failed(name, &format!("Signal '{}' not found", signal_name));
            }
        };

        let ascending = time
            .first()
            .zip(time.last())
            .is_none_or(|(first, last)| last >= first);
        let (lower, upper) = match (from, to) {
            (Some(from), Some(to)) => (from, to),
            (Some(from), None) if ascending => (from, Value::INFINITY),
            (Some(from), None) => (Value::NEG_INFINITY, from),
            (None, Some(to)) if ascending => (Value::NEG_INFINITY, to),
            (None, Some(to)) => (to, Value::INFINITY),
            (None, None) => (Value::NEG_INFINITY, Value::INFINITY),
        };
        if lower > upper {
            return MeasureResult::failed(name, "Empty range");
        }
        let mut integral = 0.0;
        let mut width = 0.0;
        let mut previous = None;
        for (&axis, &value) in time.iter().zip(signal) {
            let lower_tolerance = if lower.is_finite() {
                lower.abs() * 1.0e-12
            } else {
                0.0
            };
            let upper_tolerance = if upper.is_finite() {
                upper.abs() * 1.0e-12
            } else {
                0.0
            };
            if axis >= lower - lower_tolerance && axis <= upper + upper_tolerance {
                if let Some((previous_axis, previous_value)) = previous {
                    let dx = Value::abs(axis - previous_axis);
                    integral += 0.5 * (value * value + previous_value * previous_value) * dx;
                    width += dx;
                }
                previous = Some((axis, value));
            } else {
                previous = None;
            }
        }
        if width == 0.0 {
            MeasureResult::failed(name, "Empty range")
        } else {
            MeasureResult::success(name, (integral / width).sqrt())
        }
    }

    fn eval_rise_fall(
        &self,
        name: &str,
        signal_name: &str,
        from_pct: Value,
        to_pct: Value,
        number: usize,
        is_rise: bool,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
    ) -> MeasureResult {
        let signal = match lookup_signal(signals, signal_name) {
            Some(s) => s,
            None => {
                return MeasureResult::failed(name, &format!("Signal '{}' not found", signal_name));
            }
        };

        // Find min and max to compute thresholds
        let min_val = signal.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = signal.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = max_val - min_val;

        let (th_low, th_high) = if is_rise {
            (min_val + from_pct * range, min_val + to_pct * range)
        } else {
            (min_val + to_pct * range, min_val + from_pct * range)
        };

        let edge = if is_rise {
            EdgeType::Rise
        } else {
            EdgeType::Fall
        };

        let t1 = self.find_crossing(time, signal, th_low, edge, number, None);
        let t2 = self.find_crossing(time, signal, th_high, edge, number, None);

        match (t1, t2) {
            (Some(t1), Some(t2)) => MeasureResult::success(name, (t2 - t1).abs()),
            _ => MeasureResult::failed(name, "Rise/fall transition not found"),
        }
    }

    /// Segment slope of the interpolating polyline at the requested time —
    /// the same piecewise-linear data model every other measure uses.
    #[allow(clippy::too_many_arguments)]
    fn eval_derivative(
        &self,
        name: &str,
        signal_name: &str,
        at: Option<Value>,
        when: Option<&WhenCondition>,
        from: Option<Value>,
        to: Option<Value>,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
        segment_starts: &[usize],
    ) -> MeasureResult {
        let signal = match lookup_signal(signals, signal_name) {
            Some(s) => s,
            None => {
                return MeasureResult::failed(name, &format!("Signal '{}' not found", signal_name));
            }
        };
        if time.len() < 2 {
            return MeasureResult::failed(name, "Not enough points for a derivative");
        }
        let (lower, upper) = Self::measurement_window_bounds(time, from, to);

        if let Some(target) = at {
            if !Self::axis_in_measurement_window(target, lower, upper) {
                return MeasureResult::failed(name, "AT point is outside the measurement window")
                    .with_event_axis(target);
            }
            let Some(segment) = measurement_segment_containing(time, target, segment_starts) else {
                return MeasureResult::failed(name, "Time point not in simulation range")
                    .with_event_axis(target);
            };
            return measurement_segment_slope(name, time, signal, segment).with_event_axis(target);
        }

        let Some(condition) = when else {
            return MeasureResult::failed(name, "DERIV requires AT=time or WHEN signal=value");
        };
        match first_measure_condition_event(condition, time, signals, lower, upper, segment_starts)
        {
            Ok(Some((segment, _, event_axis))) => {
                return measurement_segment_slope(name, time, signal, segment)
                    .with_event_axis(event_axis);
            }
            Err(error) => return MeasureResult::failed(name, &error),
            Ok(None) => {}
        }
        MeasureResult::failed(name, "WHEN condition never met in the measurement window")
    }

    /// Evaluate a PARAM expression against the named results computed so far.
    fn eval_param(&self, name: &str, expression: &str, prior: &[MeasureResult]) -> MeasureResult {
        let mut ctx = crate::netlist::ParamContext::new();
        for result in prior {
            if let Some(value) = result.value {
                ctx.set(&result.name, value);
            }
        }
        match crate::netlist::expr::eval_expression_complex(expression, &ctx) {
            Ok(value) => {
                if !value.re.is_finite() || !value.im.is_finite() {
                    return MeasureResult::failed(
                        name,
                        &format!(
                            "PARAM expression produced non-finite value ({} {:+}j)",
                            value.re, value.im
                        ),
                    );
                }
                let imag_tolerance = 1.0e-15 * value.re.abs().max(1.0);
                if value.im.abs() > imag_tolerance {
                    return MeasureResult::failed(
                        name,
                        &format!(
                            "PARAM expression produced complex value ({} {:+}j); scalar measurement results must be real",
                            value.re, value.im
                        ),
                    );
                }
                MeasureResult::success(name, value.re)
            }
            Err(err) => MeasureResult::failed(name, &format!("PARAM expression failed: {err}")),
        }
    }

    fn eval_find(
        &self,
        name: &str,
        signal_name: &str,
        at: Option<Value>,
        when: Option<&WhenCondition>,
        from: Option<Value>,
        to: Option<Value>,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
        segment_starts: &[usize],
    ) -> MeasureResult {
        let signal = match lookup_signal(signals, signal_name) {
            Some(s) => s,
            None => {
                return MeasureResult::failed(name, &format!("Signal '{}' not found", signal_name));
            }
        };

        let (lower, upper) = Self::measurement_window_bounds(time, from, to);
        if let Some(t_at) = at {
            // FIND ... AT=time
            if !Self::axis_in_measurement_window(t_at, lower, upper) {
                return MeasureResult::failed(name, "AT point is outside the measurement window");
            }
            if let Some(value) =
                interpolate_measure_signal_segmented(time, signal, t_at, segment_starts)
            {
                return MeasureResult::success(name, value);
            }
            return MeasureResult::failed(name, "Time point not in simulation range");
        }

        if let Some(condition) = when {
            match first_measure_condition_event(
                condition,
                time,
                signals,
                lower,
                upper,
                segment_starts,
            ) {
                Ok(Some((segment, fraction, _))) => {
                    let value =
                        signal[segment] + fraction * (signal[segment + 1] - signal[segment]);
                    return MeasureResult::success(name, value);
                }
                Err(error) => return MeasureResult::failed(name, &error),
                Ok(None) => {}
            }
            return MeasureResult::failed(
                name,
                "WHEN condition not found in the measurement window",
            );
        }

        MeasureResult::failed(name, "FIND requires AT= or WHEN condition")
    }

    fn eval_when(
        &self,
        name: &str,
        condition: &WhenCondition,
        from: Option<Value>,
        to: Option<Value>,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
        segment_starts: &[usize],
    ) -> MeasureResult {
        let (lower, upper) = Self::measurement_window_bounds(time, from, to);
        match first_measure_condition_event(condition, time, signals, lower, upper, segment_starts)
        {
            Ok(Some((_, _, axis))) => MeasureResult::success(name, axis),
            Ok(None) => {
                MeasureResult::failed(name, "WHEN condition not found in the measurement window")
            }
            Err(error) => MeasureResult::failed(name, &error),
        }
    }

    fn eval_integ(
        &self,
        name: &str,
        analysis: &str,
        signal_name: &str,
        from: Option<Value>,
        to: Option<Value>,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
    ) -> MeasureResult {
        let signal = match lookup_signal(signals, signal_name) {
            Some(s) => s,
            None => {
                return MeasureResult::failed(name, &format!("Signal '{}' not found", signal_name));
            }
        };
        if !analysis.eq_ignore_ascii_case("DC")
            && matches!((from, to), (Some(from), Some(to)) if from > to)
        {
            return MeasureResult::failed(name, "Empty range");
        }

        let sweep_direction = time
            .first()
            .zip(time.last())
            .map_or(1.0, |(first, last)| Value::signum(last - first));
        let (lower, upper, direction) = match (from, to) {
            (Some(from), Some(to)) => (from.min(to), from.max(to), Value::signum(to - from)),
            (Some(from), None) if sweep_direction >= 0.0 => {
                (from, Value::INFINITY, sweep_direction)
            }
            (Some(from), None) => (Value::NEG_INFINITY, from, sweep_direction),
            (None, Some(to)) if sweep_direction >= 0.0 => {
                (Value::NEG_INFINITY, to, sweep_direction)
            }
            (None, Some(to)) => (to, Value::INFINITY, sweep_direction),
            (None, None) => (Value::NEG_INFINITY, Value::INFINITY, sweep_direction),
        };
        let mut selected_points = 0usize;
        let mut integral = 0.0;
        let mut previous = None;
        for (&axis, &value) in time.iter().zip(signal) {
            let lower_tolerance = if lower.is_finite() {
                lower.abs() * 1.0e-12
            } else {
                0.0
            };
            let upper_tolerance = if upper.is_finite() {
                upper.abs() * 1.0e-12
            } else {
                0.0
            };
            if axis >= lower - lower_tolerance && axis <= upper + upper_tolerance {
                selected_points += 1;
                if let Some((previous_axis, previous_value)) = previous {
                    let dx = Value::abs(axis - previous_axis);
                    integral += 0.5 * (value + previous_value) * dx;
                }
                previous = Some((axis, value));
            } else {
                previous = None;
            }
        }
        if selected_points == 0 {
            MeasureResult::failed(name, "Empty range")
        } else {
            MeasureResult::success(name, integral * direction)
        }
    }
}

#[derive(Clone, Copy)]
enum ResolvedMeasureOperand<'a> {
    Constant(Value),
    Waveform(&'a [Value]),
}

impl ResolvedMeasureOperand<'_> {
    fn value_at(self, index: usize) -> Option<Value> {
        match self {
            Self::Constant(value) => Some(value),
            Self::Waveform(values) => values.get(index).copied(),
        }
    }
}

fn resolve_measure_operand<'a>(
    operand: &MeasureOperand,
    signals: &HashMap<String, &'a [Value]>,
) -> Result<ResolvedMeasureOperand<'a>, String> {
    match operand {
        MeasureOperand::Constant(value) => Ok(ResolvedMeasureOperand::Constant(*value)),
        MeasureOperand::Waveform(name) => lookup_signal(signals, name)
            .map(ResolvedMeasureOperand::Waveform)
            .ok_or_else(|| format!("When signal '{name}' not found")),
    }
}

fn delay_clause_event(
    clause: &TrigSpec,
    effective_td: Option<Value>,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    segment_starts: &[usize],
) -> Result<Option<Value>, String> {
    match &clause.event {
        TriggerEvent::At(target) => {
            if !target.is_finite() {
                return Ok(None);
            }
            Ok(delay_at_is_reached(axis, *target, segment_starts).then_some(*target))
        }
        TriggerEvent::When(condition) => {
            // Xyce's TRIG/TARG contract recognizes only -1/LAST as a
            // negative occurrence. Other negative values parse as a failed
            // measure so the remaining measurements in the deck still run.
            if condition.occurrence.number < -1 {
                return Ok(None);
            }
            let left = lookup_signal(signals, &condition.left)
                .ok_or_else(|| format!("When signal '{}' not found", condition.left))?;
            let right = resolve_measure_operand(&condition.right, signals)?;
            let events = measurement_condition_crossings(
                left,
                right,
                axis.len(),
                segment_starts,
                condition.occurrence.edge,
                condition.minval,
            )
            .into_iter()
            .filter_map(|(segment, fraction)| {
                let event_axis = axis[segment] + fraction * (axis[segment + 1] - axis[segment]);
                delay_td_accepts_event(event_axis, effective_td).then_some(event_axis)
            });
            Ok(select_measure_occurrence(
                events,
                condition.occurrence.number,
            ))
        }
    }
}

fn delay_at_is_reached(axis: &[Value], target: Value, segment_starts: &[usize]) -> bool {
    let Some((&minimum, &maximum)) = axis
        .iter()
        .min_by(|left, right| left.total_cmp(right))
        .zip(axis.iter().max_by(|left, right| left.total_cmp(right)))
    else {
        return false;
    };
    if target < minimum - XYCE_DEFAULT_MEASURE_MINVAL
        || target > maximum + XYCE_DEFAULT_MEASURE_MINVAL
    {
        return false;
    }
    let ascending = axis
        .windows(2)
        .enumerate()
        .find_map(|(segment, pair)| {
            (segment_starts.binary_search(&(segment + 1)).is_err() && pair[0] != pair[1])
                .then_some(pair[1] > pair[0])
        })
        .unwrap_or(true);
    axis.iter().any(|sample| {
        if ascending {
            sample - XYCE_DEFAULT_MEASURE_MINVAL >= target
        } else {
            sample - XYCE_DEFAULT_MEASURE_MINVAL <= target
        }
    })
}

fn delay_td_accepts_event(event_axis: Value, td: Option<Value>) -> bool {
    const XYCE_TD_TOLERANCE: Value = 1.0e-12;
    td.is_none_or(|td| event_axis > td * (1.0 - XYCE_TD_TOLERANCE))
}

fn axis_in_error_window(axis: Value, lower: Value, upper: Value, minval: Value) -> bool {
    let lower_tolerance = if lower.is_finite() {
        (lower * minval).abs()
    } else {
        0.0
    };
    let upper_tolerance = if upper.is_finite() {
        (upper * minval).abs()
    } else {
        0.0
    };
    axis >= lower - lower_tolerance && axis <= upper + upper_tolerance
}

fn first_measure_condition_event(
    condition: &WhenCondition,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    lower: Value,
    upper: Value,
    segment_starts: &[usize],
) -> Result<Option<(usize, Value, Value)>, String> {
    let left = lookup_signal(signals, &condition.left)
        .ok_or_else(|| format!("When signal '{}' not found", condition.left))?;
    let right = resolve_measure_operand(&condition.right, signals)?;
    let events = measurement_condition_crossings(
        left,
        right,
        axis.len(),
        segment_starts,
        condition.occurrence.edge,
        condition.minval,
    )
    .into_iter()
    .filter_map(|(segment, fraction)| {
        let event_axis = axis[segment] + fraction * (axis[segment + 1] - axis[segment]);
        MeasureEngine::axis_in_measurement_window(event_axis, lower, upper)
            .then_some((segment, fraction, event_axis))
    });
    Ok(select_measure_occurrence(
        events,
        condition.occurrence.number,
    ))
}

type MeasureEvent = (usize, Value, Value);

fn continuous_condition_events(
    condition: &WhenCondition,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    lower: Value,
    upper: Value,
    segment_starts: &[usize],
) -> Result<Vec<MeasureEvent>, String> {
    let left = lookup_signal(signals, &condition.left)
        .ok_or_else(|| format!("When signal '{}' not found", condition.left))?;
    let right = resolve_measure_operand(&condition.right, signals)?;
    let events = measurement_condition_crossings(
        left,
        right,
        axis.len(),
        segment_starts,
        condition.occurrence.edge,
        condition.minval,
    )
    .into_iter()
    .filter_map(|(segment, fraction)| {
        let event_axis = axis[segment] + fraction * (axis[segment + 1] - axis[segment]);
        MeasureEngine::axis_in_measurement_window(event_axis, lower, upper)
            .then_some((segment, fraction, event_axis))
    })
    .collect::<Vec<_>>();
    Ok(select_continuous_occurrences(
        events,
        condition.occurrence.number,
    ))
}

fn select_continuous_occurrences<T>(mut events: Vec<T>, number: isize) -> Vec<T> {
    if number >= 0 {
        // Xyce stores the qualifier as zero when it is omitted.  Our parsed
        // representation uses one for both the omitted and explicit first
        // occurrence; both begin emitting at the first qualifying event.
        let skip = number.saturating_sub(1) as usize;
        if skip >= events.len() {
            Vec::new()
        } else {
            events.drain(skip..).collect()
        }
    } else {
        let Some(distance) = number.checked_abs().map(|value| value as usize) else {
            return Vec::new();
        };
        if distance > events.len() {
            Vec::new()
        } else {
            vec![events.swap_remove(events.len() - distance)]
        }
    }
}

fn continuous_when(
    name: &str,
    condition: &WhenCondition,
    from: Option<Value>,
    to: Option<Value>,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    segment_starts: &[usize],
) -> ContinuousMeasureResult {
    let (lower, upper) = MeasureEngine::measurement_window_bounds(axis, from, to);
    match continuous_condition_events(condition, axis, signals, lower, upper, segment_starts) {
        Ok(events) if !events.is_empty() => ContinuousMeasureResult::success(
            name,
            events
                .into_iter()
                .map(|(_, _, event_axis)| ContinuousMeasureRecord::point(event_axis, event_axis))
                .collect(),
        ),
        Ok(_) => ContinuousMeasureResult::failed(
            name,
            "WHEN condition not found in the measurement window",
        ),
        Err(error) => ContinuousMeasureResult::failed(name, error),
    }
}

fn continuous_find(
    name: &str,
    signal_name: &str,
    at: Option<Value>,
    when: Option<&WhenCondition>,
    from: Option<Value>,
    to: Option<Value>,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    segment_starts: &[usize],
) -> ContinuousMeasureResult {
    let Some(signal) = lookup_signal(signals, signal_name) else {
        return ContinuousMeasureResult::failed(name, format!("Signal '{signal_name}' not found"));
    };
    let (lower, upper) = MeasureEngine::measurement_window_bounds(axis, from, to);
    if let Some(target) = at {
        if !MeasureEngine::axis_in_measurement_window(target, lower, upper) {
            return ContinuousMeasureResult::failed(
                name,
                "AT point is outside the measurement window",
            );
        }
        return interpolate_measure_signal_segmented(axis, signal, target, segment_starts)
            .map(|value| {
                ContinuousMeasureResult::success(
                    name,
                    vec![ContinuousMeasureRecord::point(value, target)],
                )
            })
            .unwrap_or_else(|| {
                ContinuousMeasureResult::failed(name, "Time point not in simulation range")
            });
    }
    let Some(condition) = when else {
        return ContinuousMeasureResult::failed(name, "FIND requires AT= or WHEN condition");
    };
    match continuous_condition_events(condition, axis, signals, lower, upper, segment_starts) {
        Ok(events) if !events.is_empty() => ContinuousMeasureResult::success(
            name,
            events
                .into_iter()
                .map(|(segment, fraction, event_axis)| {
                    let value =
                        signal[segment] + fraction * (signal[segment + 1] - signal[segment]);
                    ContinuousMeasureRecord::point(value, event_axis)
                })
                .collect(),
        ),
        Ok(_) => ContinuousMeasureResult::failed(
            name,
            "WHEN condition not found in the measurement window",
        ),
        Err(error) => ContinuousMeasureResult::failed(name, error),
    }
}

fn continuous_derivative(
    name: &str,
    signal_name: &str,
    at: Option<Value>,
    when: Option<&WhenCondition>,
    from: Option<Value>,
    to: Option<Value>,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    segment_starts: &[usize],
) -> ContinuousMeasureResult {
    let Some(signal) = lookup_signal(signals, signal_name) else {
        return ContinuousMeasureResult::failed(name, format!("Signal '{signal_name}' not found"));
    };
    let (lower, upper) = MeasureEngine::measurement_window_bounds(axis, from, to);
    let make_record = |segment: usize, event_axis: Value| {
        let width = axis[segment + 1] - axis[segment];
        if width == 0.0 || !width.is_finite() {
            None
        } else {
            let value = (signal[segment + 1] - signal[segment]) / width;
            value
                .is_finite()
                .then_some(ContinuousMeasureRecord::point(value, event_axis))
        }
    };
    if let Some(target) = at {
        if !MeasureEngine::axis_in_measurement_window(target, lower, upper) {
            return ContinuousMeasureResult::failed(
                name,
                "AT point is outside the measurement window",
            );
        }
        let Some(segment) = measurement_segment_containing(axis, target, segment_starts) else {
            return ContinuousMeasureResult::failed(name, "Time point not in simulation range");
        };
        return make_record(segment, target)
            .map(|record| ContinuousMeasureResult::success(name, vec![record]))
            .unwrap_or_else(|| {
                ContinuousMeasureResult::failed(
                    name,
                    "Derivative interval has zero or non-finite width",
                )
            });
    }
    let Some(condition) = when else {
        return ContinuousMeasureResult::failed(
            name,
            "DERIV requires AT=time or WHEN signal=value",
        );
    };
    match continuous_condition_events(condition, axis, signals, lower, upper, segment_starts) {
        Ok(events) if !events.is_empty() => {
            let records = events
                .into_iter()
                .filter_map(|(segment, _, event_axis)| make_record(segment, event_axis))
                .collect::<Vec<_>>();
            if records.is_empty() {
                ContinuousMeasureResult::failed(
                    name,
                    "Derivative interval has zero or non-finite width",
                )
            } else {
                ContinuousMeasureResult::success(name, records)
            }
        }
        Ok(_) => ContinuousMeasureResult::failed(
            name,
            "WHEN condition never met in the measurement window",
        ),
        Err(error) => ContinuousMeasureResult::failed(name, error),
    }
}

fn continuous_delay_clause_events(
    clause: &TrigSpec,
    effective_td: Option<Value>,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    segment_starts: &[usize],
) -> Result<Vec<Value>, String> {
    match &clause.event {
        // Xyce treats AT as an exact clause result. TD gates conditional
        // events but does not override a valid explicit AT location.
        TriggerEvent::At(target) => Ok((target.is_finite()
            && delay_at_is_reached(axis, *target, segment_starts))
        .then_some(*target)
        .into_iter()
        .collect()),
        TriggerEvent::When(condition) => {
            if condition.occurrence.number < 0 {
                return Err(
                    "negative RISE/FALL/CROSS qualifiers are invalid for continuous TRIG/TARG"
                        .to_string(),
                );
            }
            let left = lookup_signal(signals, &condition.left)
                .ok_or_else(|| format!("When signal '{}' not found", condition.left))?;
            let right = resolve_measure_operand(&condition.right, signals)?;
            let events = measurement_condition_crossings(
                left,
                right,
                axis.len(),
                segment_starts,
                condition.occurrence.edge,
                condition.minval,
            )
            .into_iter()
            .filter_map(|(segment, fraction)| {
                let event_axis = axis[segment] + fraction * (axis[segment + 1] - axis[segment]);
                delay_td_accepts_event(event_axis, effective_td).then_some(event_axis)
            })
            .collect::<Vec<_>>();
            Ok(select_continuous_occurrences(
                events,
                condition.occurrence.number,
            ))
        }
    }
}

fn continuous_delay(
    name: &str,
    trig: &TrigSpec,
    targ: &TrigSpec,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    segment_starts: &[usize],
) -> ContinuousMeasureResult {
    let target_td = targ.td.or(trig.td);
    let triggers =
        match continuous_delay_clause_events(trig, trig.td, axis, signals, segment_starts) {
            Ok(events) => events,
            Err(error) => return ContinuousMeasureResult::failed(name, error),
        };
    let targets =
        match continuous_delay_clause_events(targ, target_td, axis, signals, segment_starts) {
            Ok(events) => events,
            Err(error) => return ContinuousMeasureResult::failed(name, error),
        };
    let partial_trigger = triggers.first().copied();
    let partial_target = targets.first().copied();
    let records = triggers
        .into_iter()
        .zip(targets)
        .map(|(trigger, target)| ContinuousMeasureRecord::delay(trigger, target))
        .collect::<Vec<_>>();
    if records.is_empty() {
        ContinuousMeasureResult::failed_delay(
            name,
            partial_trigger,
            partial_target,
            "trigger/target event pair not found",
        )
    } else {
        ContinuousMeasureResult::success(name, records)
    }
}

fn select_measure_occurrence<T>(events: impl Iterator<Item = T>, number: isize) -> Option<T> {
    if number > 0 {
        events.into_iter().nth(number as usize - 1)
    } else if number < 0 {
        let events = events.collect::<Vec<_>>();
        number
            .checked_abs()
            .and_then(|distance| events.len().checked_sub(distance as usize))
            .and_then(|index| events.into_iter().nth(index))
    } else {
        None
    }
}

/// Return every qualifying `WHEN left=right` crossing interval in traversal
/// order. Xyce requires the left operand itself to change over the interval;
/// a moving right operand cannot trigger against a constant left operand.
fn measurement_condition_crossings(
    left: &[Value],
    right: ResolvedMeasureOperand<'_>,
    point_count: usize,
    segment_starts: &[usize],
    edge: EdgeType,
    minval: Value,
) -> Vec<(usize, Value)> {
    const FLAT_LEFT_NUMERIC_TOLERANCE: Value = 1.0e-12;
    if left.len() != point_count || point_count < 2 || !minval.is_finite() || minval < 0.0 {
        return Vec::new();
    }

    let mut crossings = Vec::new();
    for segment in 0..point_count - 1 {
        if segment_starts.binary_search(&(segment + 1)).is_ok() {
            continue;
        }
        let left_previous = left[segment];
        let left_current = left[segment + 1];
        let left_scale = left_previous.abs().max(left_current.abs()).max(1.0);
        // MINVAL applies to equality with the target, not to Xyce's
        // independent requirement that the left operand itself move.
        if (left_current - left_previous).abs() <= FLAT_LEFT_NUMERIC_TOLERANCE * left_scale {
            continue;
        }
        let Some(right_previous) = right.value_at(segment) else {
            return Vec::new();
        };
        let Some(right_current) = right.value_at(segment + 1) else {
            return Vec::new();
        };
        let previous_difference = left_previous - right_previous;
        let current_difference = left_current - right_current;
        let previous_equal = previous_difference.abs() < minval;
        let current_equal = current_difference.abs() < minval;
        // Entering Xyce's MINVAL equality band is the crossing.  Normalize
        // that state before the next strict-sign test so leaving the band
        // cannot emit the same physical root a second time.
        let strict_crossing = !previous_equal
            && ((previous_difference < 0.0 && current_difference > 0.0)
                || (previous_difference > 0.0 && current_difference < 0.0));
        if !current_equal && !strict_crossing {
            continue;
        }
        let edge_matches = match edge {
            EdgeType::Rise => left_current > left_previous,
            EdgeType::Fall => left_current < left_previous,
            EdgeType::Cross => true,
        };
        if !edge_matches {
            continue;
        }

        let denominator = current_difference - previous_difference;
        let fraction = if current_equal {
            // The accepted endpoint is equal within Xyce's MINVAL contract.
            // Pinning the event to that endpoint prevents a tiny same-side
            // solver residual from extrapolating the nominal root beyond the
            // segment and changing after finite-precision PRN serialization.
            1.0
        } else if denominator == 0.0 {
            // Parallel, identical moving operands are considered equal at the
            // current accepted point by Xyce.
            1.0
        } else {
            -previous_difference / denominator
        };
        if fraction.is_finite() && (0.0..=1.0).contains(&fraction) {
            crossings.push((segment, fraction));
        }
    }
    crossings
}

fn measurement_segment_containing(
    axis: &[Value],
    target: Value,
    segment_starts: &[usize],
) -> Option<usize> {
    const XYCE_AT_ABSOLUTE_TOLERANCE: Value = 1.0e-12;
    axis.windows(2).enumerate().find_map(|(segment, pair)| {
        if segment_starts.binary_search(&(segment + 1)).is_ok() {
            return None;
        }
        let previous = pair[0];
        let current = pair[1];
        if previous == current {
            return None;
        }
        let strictly_between = target > previous.min(current) && target < previous.max(current);
        let matches_endpoint = (target - previous).abs() < XYCE_AT_ABSOLUTE_TOLERANCE
            || (target - current).abs() < XYCE_AT_ABSOLUTE_TOLERANCE;
        (strictly_between || matches_endpoint).then_some(segment)
    })
}

fn measurement_segment_slope(
    name: &str,
    axis: &[Value],
    signal: &[Value],
    segment: usize,
) -> MeasureResult {
    let delta_axis = axis[segment + 1] - axis[segment];
    if delta_axis == 0.0 || !delta_axis.is_finite() {
        return MeasureResult::failed(name, "Derivative interval has zero or non-finite width");
    }
    let slope = (signal[segment + 1] - signal[segment]) / delta_axis;
    MeasureResult::success(name, slope)
}

#[cfg(test)]
fn interpolate_measure_signal(axis: &[Value], signal: &[Value], target: Value) -> Option<Value> {
    interpolate_measure_signal_segmented(axis, signal, target, &[])
}

fn interpolate_measure_signal_segmented(
    axis: &[Value],
    signal: &[Value],
    target: Value,
    segment_starts: &[usize],
) -> Option<Value> {
    if axis.len() != signal.len() || axis.is_empty() || !target.is_finite() {
        return None;
    }
    if let Some(index) = axis.iter().position(|value| *value == target) {
        return signal.get(index).copied();
    }
    axis.windows(2).enumerate().find_map(|(index, segment)| {
        if segment_starts.binary_search(&(index + 1)).is_ok() {
            return None;
        }
        let left = segment[0];
        let right = segment[1];
        if left == right || target < left.min(right) || target > left.max(right) {
            return None;
        }
        let fraction = (target - left) / (right - left);
        Some(signal[index] + fraction * (signal[index + 1] - signal[index]))
    })
}

impl Default for MeasureEngine {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn engine_with(statement: MeasureStatement) -> MeasureEngine {
        let mut engine = MeasureEngine::new();
        engine.add(statement);
        engine
    }

    #[test]
    fn measurement_interpolation_accepts_exact_singleton_and_final_samples() {
        assert_eq!(interpolate_measure_signal(&[1.0], &[2.5], 1.0), Some(2.5));
        assert_eq!(
            interpolate_measure_signal(&[1.0, 2.0], &[3.0, 5.0], 2.0),
            Some(5.0)
        );
        assert_eq!(
            interpolate_measure_signal(&[1.0, 2.0], &[3.0, 5.0], 1.5),
            Some(4.0)
        );
        assert_eq!(interpolate_measure_signal(&[1.0], &[2.5], 2.0), None);
    }

    #[test]
    fn integration_over_one_selected_sample_is_zero() {
        let statement = MeasureStatement {
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
            goal: None,
            tolerance: None,
            name: "area".to_string(),
            measure_type: MeasureType::Integ {
                signal: "V(out)".to_string(),
                from: Some(2.0),
                to: Some(2.0),
            },
            analysis: "AC".to_string(),
        };
        let axis = [1.0, 2.0, 3.0];
        let values = [4.0, 5.0, 6.0];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &values);

        let results = engine_with(statement).evaluate(&axis, &signals);

        assert_eq!(results[0].value, Some(0.0));
        assert!(results[0].passed);
    }

    #[test]
    fn average_rejects_reversed_windows_without_analysis_normalization() {
        let statement = MeasureStatement {
            goal: None,
            tolerance: None,
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "average".to_string(),
            measure_type: MeasureType::Avg {
                signal: "V(out)".to_string(),
                from: Some(2.0),
                to: Some(1.0),
            },
            analysis: "AC".to_string(),
        };
        let axis = [1.0, 2.0, 3.0];
        let values = [1.0, 2.0, 3.0];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &values);

        let results = engine_with(statement).evaluate(&axis, &signals);

        assert_eq!(results[0].value, None);
        assert!(!results[0].passed);
        assert_eq!(results[0].error.as_deref(), Some("Empty range"));
    }

    #[test]
    fn integration_accepts_reversed_windows_only_for_dc() {
        let statement = |name: &str, analysis: &str| MeasureStatement {
            goal: None,
            tolerance: None,
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: name.to_string(),
            measure_type: MeasureType::Integ {
                signal: "V(out)".to_string(),
                from: Some(2.0),
                to: Some(1.0),
            },
            analysis: analysis.to_string(),
        };
        let axis = [1.0, 2.0];
        let values = [1.0, 2.0];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &values);
        let mut engine = MeasureEngine::new();
        engine.add(statement("ac_integral", "AC"));
        engine.add(statement("dc_integral", "DC"));

        let results = engine.evaluate(&axis, &signals);

        assert_eq!(results[0].value, None);
        assert!(!results[0].passed);
        assert_eq!(results[1].value, Some(-1.5));
        assert!(results[1].passed);
    }

    #[test]
    fn rms_rejects_reversed_windows_without_dc_normalization() {
        let statement = MeasureStatement {
            goal: None,
            tolerance: None,
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "rms".to_string(),
            measure_type: MeasureType::Rms {
                signal: "V(out)".to_string(),
                from: Some(2.0),
                to: Some(1.0),
            },
            analysis: "AC".to_string(),
        };
        let axis = [1.0, 2.0, 3.0];
        let values = [1.0, 2.0, 3.0];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &values);

        let results = engine_with(statement).evaluate(&axis, &signals);

        assert_eq!(results[0].value, None);
        assert!(!results[0].passed);
        assert_eq!(results[0].error.as_deref(), Some("Empty range"));
    }

    #[test]
    fn extrema_reject_reversed_windows_without_dc_normalization() {
        let maximum = MeasureStatement {
            goal: None,
            tolerance: None,
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "maximum".to_string(),
            measure_type: MeasureType::Max {
                signal: "V(out)".to_string(),
                from: Some(2.0),
                to: Some(1.0),
                output: ExtremaOutput::Value,
            },
            analysis: "AC".to_string(),
        };
        let peak_to_peak = MeasureStatement {
            goal: None,
            tolerance: None,
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "peak_to_peak".to_string(),
            measure_type: MeasureType::PeakToPeak {
                signal: "V(out)".to_string(),
                from: Some(2.0),
                to: Some(1.0),
            },
            analysis: "AC".to_string(),
        };
        let axis = [1.0, 2.0, 3.0];
        let values = [1.0, 3.0, 2.0];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &values);
        let mut engine = MeasureEngine::new();
        engine.add(maximum);
        engine.add(peak_to_peak);

        let results = engine.evaluate(&axis, &signals);

        assert!(results.iter().all(|result| result.value.is_none()));
        assert!(results.iter().all(|result| !result.passed));
    }

    #[test]
    fn extrema_independent_axis_output_keeps_first_tie() {
        let statement = MeasureStatement {
            goal: None,
            tolerance: None,
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "peak_frequency".to_string(),
            measure_type: MeasureType::Max {
                signal: "V(out)".to_string(),
                from: None,
                to: None,
                output: ExtremaOutput::IndependentAxis,
            },
            analysis: "AC".to_string(),
        };
        let axis = [10.0, 20.0, 30.0];
        let values = [2.0, 5.0, 5.0];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &values);

        let results = engine_with(statement).evaluate(&axis, &signals);

        assert_eq!(results[0].value, Some(20.0));
        assert_eq!(results[0].event_axis, Some(20.0));
    }

    fn max_statement(signal: &str) -> MeasureStatement {
        MeasureStatement {
            goal: None,
            tolerance: None,
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "peak".to_string(),
            measure_type: MeasureType::Max {
                signal: signal.to_string(),
                from: None,
                to: None,
                output: ExtremaOutput::Value,
            },
            analysis: "TRAN".to_string(),
        }
    }

    #[test]
    fn lookup_is_case_insensitive() {
        let time = [0.0, 1.0, 2.0];
        let data = [0.0, 3.0, 1.0];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &data);

        let results = engine_with(max_statement("v(OUT)")).evaluate(&time, &signals);
        assert_eq!(results[0].value, Some(3.0));
        assert_eq!(results[0].event_axis, Some(1.0));
    }

    #[test]
    fn exact_key_wins_over_case_fold() {
        let time = [0.0, 1.0];
        let exact = [0.0, 7.0];
        let folded = [0.0, 9.0];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &exact);
        signals.insert("V(OUT)".to_string(), &folded);

        let results = engine_with(max_statement("V(out)")).evaluate(&time, &signals);
        assert_eq!(results[0].value, Some(7.0));
    }

    #[test]
    fn missing_signal_reports_failure() {
        let time = [0.0, 1.0];
        let data = [0.0, 1.0];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &data);

        let results = engine_with(max_statement("V(nope)")).evaluate(&time, &signals);
        assert_eq!(results[0].value, None);
        assert!(results[0].error.as_deref().unwrap_or("").contains("nope"));
    }

    #[test]
    fn empty_axis_reports_failed_measurement_without_panicking() {
        let data: [Value; 0] = [];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &data);

        let results = engine_with(max_statement("V(out)")).evaluate(&[], &signals);

        assert_eq!(results[0].value, None);
        assert!(
            results[0]
                .error
                .as_deref()
                .unwrap_or("")
                .contains("axis is empty")
        );
    }

    #[test]
    fn mismatched_signal_length_reports_failed_measurement_without_panicking() {
        let time = [0.0, 1.0, 2.0];
        let data = [0.0, 1.0];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &data);

        let results = engine_with(max_statement("V(out)")).evaluate(&time, &signals);

        assert_eq!(results[0].value, None);
        assert!(
            results[0]
                .error
                .as_deref()
                .unwrap_or("")
                .contains("measurement axis has 3")
        );
    }

    #[test]
    fn non_finite_axis_reports_failed_measurement_without_passing_specs() {
        let time = [0.0, f64::NAN, 2.0];
        let data = [0.0, 1.0, 2.0];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &data);

        let results = engine_with(max_statement("V(out)")).evaluate(&time, &signals);

        assert!(!results[0].passed);
        assert_eq!(results[0].value, None);
        assert!(
            results[0]
                .error
                .as_deref()
                .unwrap_or("")
                .contains("axis contains non-finite")
        );
    }

    #[test]
    fn non_finite_signal_reports_failed_measurement_without_passing_specs() {
        let time = [0.0, 1.0, 2.0];
        let data = [0.0, f64::INFINITY, 2.0];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &data);

        let results = engine_with(max_statement("V(out)")).evaluate(&time, &signals);

        assert!(!results[0].passed);
        assert_eq!(results[0].value, None);
        assert!(
            results[0]
                .error
                .as_deref()
                .unwrap_or("")
                .contains("signal 'V(out)' contains non-finite")
        );
    }

    #[test]
    fn param_measure_rejects_complex_value() {
        let statement = MeasureStatement {
            goal: None,
            tolerance: None,
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "bad_param".to_string(),
            measure_type: MeasureType::Param {
                expression: "sqrt(-1)".to_string(),
            },
            analysis: "TRAN".to_string(),
        };
        let time = [0.0, 1.0];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &[0.0, 1.0]);

        let results = engine_with(statement).evaluate(&time, &signals);

        assert!(!results[0].passed);
        assert_eq!(results[0].value, None);
        assert!(
            results[0]
                .error
                .as_deref()
                .unwrap_or("")
                .contains("complex value")
        );
    }

    fn derivative_statement(
        name: &str,
        signal: &str,
        at: Option<Value>,
        when: Option<WhenCondition>,
        from: Option<Value>,
        to: Option<Value>,
    ) -> MeasureStatement {
        MeasureStatement {
            name: name.to_string(),
            measure_type: MeasureType::Derivative {
                signal: signal.to_string(),
                at,
                when,
                from,
                to,
            },
            analysis: "DC".to_string(),
            goal: None,
            tolerance: None,
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
        }
    }

    #[test]
    fn derivative_at_uses_first_traversed_secant_and_honors_windows() {
        let ascending_axis = [1.0, 2.0, 3.0, 4.0, 5.0];
        let squares = [1.0, 4.0, 9.0, 16.0, 25.0];
        let mut ascending_signals = HashMap::new();
        ascending_signals.insert("Y".to_string(), squares.as_slice());
        let mut ascending = MeasureEngine::new();
        ascending.add(derivative_statement(
            "start",
            "Y",
            Some(1.0),
            None,
            None,
            None,
        ));
        ascending.add(derivative_statement(
            "interior",
            "Y",
            Some(2.5),
            None,
            None,
            None,
        ));
        ascending.add(derivative_statement(
            "windowed_start",
            "Y",
            Some(1.0),
            None,
            Some(1.0),
            Some(1.0),
        ));
        ascending.add(derivative_statement(
            "outside",
            "Y",
            Some(2.0),
            None,
            None,
            Some(1.0),
        ));

        let results = ascending.evaluate(&ascending_axis, &ascending_signals);
        assert_eq!(results[0].value, Some(3.0));
        assert_eq!(results[1].value, Some(5.0));
        assert_eq!(results[2].value, Some(3.0));
        assert_eq!(results[0].event_axis, Some(1.0));
        assert_eq!(results[1].event_axis, Some(2.5));
        assert_eq!(results[2].event_axis, Some(1.0));
        assert!(!results[3].passed);

        let descending_axis = [5.0, 4.0, 3.0, 2.0, 1.0];
        let half_squares = [12.5, 8.0, 4.5, 2.0, 0.5];
        let mut descending_signals = HashMap::new();
        descending_signals.insert("Y".to_string(), half_squares.as_slice());
        let mut descending = MeasureEngine::new();
        descending.add(derivative_statement(
            "interior",
            "Y",
            Some(2.5),
            None,
            None,
            None,
        ));
        descending.add(derivative_statement(
            "stop",
            "Y",
            Some(1.0),
            None,
            None,
            None,
        ));

        let results = descending.evaluate(&descending_axis, &descending_signals);
        assert_eq!(results[0].value, Some(2.5));
        assert_eq!(results[1].value, Some(1.5));
        assert_eq!(results[0].event_axis, Some(2.5));
        assert_eq!(results[1].event_axis, Some(1.0));
    }

    #[test]
    fn derivative_when_intersects_moving_operands_and_filters_each_crossing() {
        let axis = [1.0, 2.0, 3.0, 4.0, 5.0];
        let squares = [1.0, 4.0, 9.0, 16.0, 25.0];
        let four_x = [4.0, 8.0, 12.0, 16.0, 20.0];
        let double_crossing = [4.2, 1.1, 0.0, 0.9, 3.8];
        // Linear solves can leave roundoff-scale jitter on a physically
        // constant waveform; it must not fabricate a WHEN event.
        let constant = [1.0, 1.0 + 1.0e-14, 1.0 - 1.0e-14, 1.0, 1.0 + 5.0e-15];
        let mut signals = HashMap::new();
        signals.insert("Y".to_string(), squares.as_slice());
        signals.insert("TARGET".to_string(), four_x.as_slice());
        signals.insert("DOUBLE".to_string(), double_crossing.as_slice());
        signals.insert("CONSTANT".to_string(), constant.as_slice());
        let mut engine = MeasureEngine::new();
        engine.add(derivative_statement(
            "moving",
            "Y",
            None,
            Some(WhenCondition {
                left: "Y".to_string(),
                right: MeasureOperand::Waveform("TARGET".to_string()),
                occurrence: EventOccurrence::default(),
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            }),
            None,
            None,
        ));
        engine.add(derivative_statement(
            "second_crossing",
            "DOUBLE",
            None,
            Some(WhenCondition {
                left: "DOUBLE".to_string(),
                right: MeasureOperand::Constant(0.5),
                occurrence: EventOccurrence::default(),
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            }),
            Some(2.75),
            None,
        ));
        engine.add(derivative_statement(
            "constant_left_fails",
            "Y",
            None,
            Some(WhenCondition {
                left: "CONSTANT".to_string(),
                right: MeasureOperand::Constant(1.0),
                occurrence: EventOccurrence::default(),
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            }),
            None,
            None,
        ));

        let results = engine.evaluate(&axis, &signals);
        assert_eq!(results[0].value, Some(7.0));
        assert_eq!(results[1].value, Some(0.9));
        assert_eq!(results[0].event_axis, Some(4.0));
        assert_eq!(results[1].event_axis, Some(3.5555555555555554));
        assert!(!results[2].passed);
    }

    #[test]
    fn when_returns_interpolated_axis_and_honors_inclusive_windows() {
        let axis = [5.0, 4.0, 3.0, 2.0, 1.0];
        let condition = [5.0, 4.0, 3.0, 2.0, 1.0];
        let mut signals = HashMap::new();
        signals.insert("CONDITION".to_string(), condition.as_slice());
        let statement = |name: &str, from: Option<Value>, to: Option<Value>| MeasureStatement {
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: name.to_string(),
            measure_type: MeasureType::When {
                condition: WhenCondition {
                    left: "CONDITION".to_string(),
                    right: MeasureOperand::Constant(2.5),
                    occurrence: EventOccurrence::default(),
                    minval: XYCE_DEFAULT_MEASURE_MINVAL,
                },
                from,
                to,
            },
            analysis: "DC".to_string(),
            goal: None,
            tolerance: None,
        };
        let mut engine = MeasureEngine::new();
        engine.add(statement("unbounded", None, None));
        engine.add(statement("singleton_window", Some(2.5), Some(2.5)));
        engine.add(statement("excluded", Some(5.0), Some(3.0)));

        let results = engine.evaluate(&axis, &signals);

        assert_eq!(results[0].value, Some(2.5));
        assert_eq!(results[1].value, Some(2.5));
        assert!(!results[2].passed);
    }

    #[test]
    fn find_and_when_do_not_interpolate_across_dc_cycle_restarts() {
        let axis = [5.0, 4.0, 3.0, 2.0, 1.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        // The first cycle stays below four. Crossing the flattened 1 -> 5
        // restart would be synthetic; the physical crossing is 4.0625 ->
        // 3.4375 in the second cycle, at an axis value of 3.9.
        let condition = [
            3.75, 3.125, 2.5, 1.875, 1.25, 4.6875, 4.0625, 3.4375, 2.8125, 2.1875,
        ];
        let found = [1.0, 2.0, 3.0, 4.0, 5.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let mut signals = HashMap::new();
        signals.insert("CONDITION".to_string(), condition.as_slice());
        signals.insert("FOUND".to_string(), found.as_slice());
        let when = WhenCondition {
            left: "CONDITION".to_string(),
            right: MeasureOperand::Constant(4.0),
            occurrence: EventOccurrence::default(),
            minval: XYCE_DEFAULT_MEASURE_MINVAL,
        };
        let mut engine = MeasureEngine::new();
        engine.add(MeasureStatement {
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "event_axis".to_string(),
            measure_type: MeasureType::When {
                condition: when.clone(),
                from: None,
                to: None,
            },
            analysis: "DC".to_string(),
            goal: None,
            tolerance: None,
        });
        engine.add(MeasureStatement {
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "found_value".to_string(),
            measure_type: MeasureType::Find {
                signal: "FOUND".to_string(),
                at: None,
                when: Some(when),
                from: None,
                to: None,
            },
            analysis: "DC".to_string(),
            goal: None,
            tolerance: None,
        });

        let results = engine.evaluate_with_segment_starts(&axis, &signals, &[5]);

        assert_eq!(results[0].value, Some(3.9));
        assert_eq!(results[1].value, Some(4.1));
    }

    #[test]
    fn conditional_occurrences_select_rise_fall_and_windowed_cross_counts() {
        let axis = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let alternating = [-1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
        let mut signals = HashMap::new();
        signals.insert("ALT".to_string(), alternating.as_slice());
        let statement =
            |name: &str, edge: EdgeType, number: isize, from: Option<Value>| -> MeasureStatement {
                MeasureStatement {
                    default_value: None,
                    print_policy: MeasurePrintPolicy::All,
                    name: name.to_string(),
                    measure_type: MeasureType::When {
                        condition: WhenCondition {
                            left: "ALT".to_string(),
                            right: MeasureOperand::Constant(0.0),
                            occurrence: EventOccurrence { edge, number },
                            minval: XYCE_DEFAULT_MEASURE_MINVAL,
                        },
                        from,
                        to: None,
                    },
                    analysis: "DC".to_string(),
                    goal: None,
                    tolerance: None,
                }
            };
        let mut engine = MeasureEngine::new();
        engine.add(statement("fourth_cross", EdgeType::Cross, 4, None));
        engine.add(statement("second_rise", EdgeType::Rise, 2, None));
        engine.add(statement("third_fall", EdgeType::Fall, 3, None));
        engine.add(statement(
            "second_windowed_cross",
            EdgeType::Cross,
            2,
            Some(2.0),
        ));

        let results = engine.evaluate(&axis, &signals);

        assert_eq!(results[0].value, Some(3.5));
        assert_eq!(results[1].value, Some(2.5));
        assert_eq!(results[2].value, Some(5.5));
        assert_eq!(results[3].value, Some(3.5));
    }

    #[test]
    fn minval_endpoint_equality_pins_crossing_inside_segment() {
        let nearly_equal = [-1.0, -1.0e-14];
        assert_eq!(
            measurement_condition_crossings(
                &nearly_equal,
                ResolvedMeasureOperand::Constant(0.0),
                nearly_equal.len(),
                &[],
                EdgeType::Cross,
                XYCE_DEFAULT_MEASURE_MINVAL,
            ),
            vec![(0, 1.0)]
        );
    }

    #[test]
    fn minval_endpoint_equality_is_not_recounted_on_band_exit() {
        let enters_then_leaves = [1.0, 1.0e-14, -1.0];
        assert_eq!(
            measurement_condition_crossings(
                &enters_then_leaves,
                ResolvedMeasureOperand::Constant(0.0),
                enters_then_leaves.len(),
                &[],
                EdgeType::Cross,
                XYCE_DEFAULT_MEASURE_MINVAL,
            ),
            vec![(0, 1.0)]
        );
    }

    #[test]
    fn custom_minval_controls_endpoint_equality_band() {
        let nearly_equal = [-1.0, -1.0e-14];
        assert!(
            measurement_condition_crossings(
                &nearly_equal,
                ResolvedMeasureOperand::Constant(0.0),
                nearly_equal.len(),
                &[],
                EdgeType::Cross,
                1.0e-16,
            )
            .is_empty()
        );
        assert_eq!(
            measurement_condition_crossings(
                &nearly_equal,
                ResolvedMeasureOperand::Constant(0.0),
                nearly_equal.len(),
                &[],
                EdgeType::Cross,
                1.0e-13,
            ),
            vec![(0, 1.0)]
        );
    }

    #[test]
    fn delay_events_support_at_td_inheritance_and_last_occurrences() {
        let axis = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let alternating = [-1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
        let mut signals = HashMap::new();
        signals.insert("ALT".to_string(), alternating.as_slice());
        let condition = |number| WhenCondition {
            left: "ALT".to_string(),
            right: MeasureOperand::Constant(0.0),
            occurrence: EventOccurrence {
                edge: EdgeType::Cross,
                number,
            },
            minval: XYCE_DEFAULT_MEASURE_MINVAL,
        };
        let mut engine = MeasureEngine::new();
        engine.add(MeasureStatement {
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "inherited_td".to_string(),
            measure_type: MeasureType::Delay {
                trig: TrigSpec {
                    event: TriggerEvent::At(4.0),
                    td: Some(5.0),
                },
                targ: TrigSpec {
                    event: TriggerEvent::When(condition(1)),
                    td: None,
                },
            },
            analysis: "DC".to_string(),
            goal: None,
            tolerance: None,
        });
        engine.add(MeasureStatement {
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "last_is_signed".to_string(),
            measure_type: MeasureType::Delay {
                trig: TrigSpec {
                    event: TriggerEvent::When(condition(-1)),
                    td: None,
                },
                targ: TrigSpec {
                    event: TriggerEvent::At(1.0),
                    td: None,
                },
            },
            analysis: "DC".to_string(),
            goal: None,
            tolerance: None,
        });
        engine.add(MeasureStatement {
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "unsupported_negative_occurrence".to_string(),
            measure_type: MeasureType::Delay {
                trig: TrigSpec {
                    event: TriggerEvent::When(condition(-2)),
                    td: None,
                },
                targ: TrigSpec {
                    event: TriggerEvent::At(1.0),
                    td: None,
                },
            },
            analysis: "DC".to_string(),
            goal: None,
            tolerance: None,
        });

        let results = engine.evaluate(&axis, &signals);

        assert_eq!(results[0].value, Some(1.5));
        assert_eq!(results[1].value, Some(-4.5));
        assert!(!results[2].passed);
    }

    #[test]
    fn trigger_target_at_requires_xyce_directional_accepted_point_reach() {
        assert!(delay_at_is_reached(&[0.0, 1.0, 2.0], 1.0, &[]));
        assert!(!delay_at_is_reached(&[0.0, 1.0, 2.0], 2.0, &[]));
        assert!(delay_at_is_reached(&[2.0, 1.0, 0.0], 0.0, &[]));
        assert!(!delay_at_is_reached(&[0.0, 1.0, 2.0], 3.0, &[]));
    }

    #[test]
    fn invalid_segment_boundaries_fail_all_measurements_without_panicking() {
        let axis = [0.0, 1.0, 2.0];
        let values = [0.0, 1.0, 2.0];
        let mut signals = HashMap::new();
        signals.insert("V(out)".to_string(), values.as_slice());

        for invalid in [&[0][..], &[3][..], &[2, 1][..], &[1, 1][..]] {
            let results = engine_with(max_statement("V(out)"))
                .evaluate_with_segment_starts(&axis, &signals, invalid);
            assert_eq!(results[0].value, None);
            assert!(!results[0].passed);
            assert!(
                results[0]
                    .error
                    .as_deref()
                    .is_some_and(|error| error.contains("segment starts"))
            );
        }
    }

    #[test]
    fn non_finite_goal_contract_fails_measurement() {
        let mut statement = max_statement("V(out)");
        statement.goal = Some(f64::NAN);
        let time = [0.0, 1.0];
        let data = [0.0, 1.0];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &data);

        let results = engine_with(statement).evaluate(&time, &signals);

        assert!(!results[0].passed);
        assert_eq!(results[0].value, Some(1.0));
        assert!(results[0].error.as_deref().unwrap_or("").contains("GOAL"));
    }

    fn continuous_statement(name: &str, measure_type: MeasureType) -> MeasureStatement {
        MeasureStatement {
            name: name.to_string(),
            measure_type,
            analysis: "NOISE_CONT".to_string(),
            goal: None,
            tolerance: None,
            default_value: None,
            print_policy: MeasurePrintPolicy::All,
        }
    }

    fn alternating_condition(number: isize) -> WhenCondition {
        WhenCondition {
            left: "ALT".to_string(),
            right: MeasureOperand::Constant(0.0),
            occurrence: EventOccurrence {
                edge: EdgeType::Cross,
                number,
            },
            minval: XYCE_DEFAULT_MEASURE_MINVAL,
        }
    }

    #[test]
    fn continuous_when_filters_window_before_emitting_occurrence_suffix() {
        let axis = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let alternating = [-1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
        let mut signals = HashMap::new();
        signals.insert("ALT".to_string(), alternating.as_slice());
        let mut engine = MeasureEngine::new();
        engine.add(continuous_statement(
            "suffix",
            MeasureType::When {
                condition: alternating_condition(2),
                from: Some(2.0),
                to: Some(5.0),
            },
        ));
        engine.add(continuous_statement(
            "from_end",
            MeasureType::When {
                condition: alternating_condition(-2),
                from: None,
                to: None,
            },
        ));

        let results = engine.evaluate_continuous(&axis, &signals, &[]);

        assert_eq!(
            results[0]
                .records
                .iter()
                .map(|record| record.value)
                .collect::<Vec<_>>(),
            vec![3.5, 4.5]
        );
        assert_eq!(results[1].records.len(), 1);
        assert_eq!(results[1].records[0].value, 4.5);
    }

    #[test]
    fn continuous_find_and_derivative_emit_interpolated_event_records() {
        let axis = [0.0, 1.0, 2.0, 3.0];
        let alternating = [-1.0, 1.0, -1.0, 1.0];
        let dependent = [0.0, 10.0, 40.0, 90.0];
        let mut signals = HashMap::new();
        signals.insert("ALT".to_string(), alternating.as_slice());
        signals.insert("Y".to_string(), dependent.as_slice());
        let mut engine = MeasureEngine::new();
        engine.add(continuous_statement(
            "find",
            MeasureType::Find {
                signal: "Y".to_string(),
                at: None,
                when: Some(alternating_condition(1)),
                from: None,
                to: None,
            },
        ));
        engine.add(continuous_statement(
            "derivative",
            MeasureType::Derivative {
                signal: "Y".to_string(),
                at: None,
                when: Some(alternating_condition(1)),
                from: None,
                to: None,
            },
        ));

        let results = engine.evaluate_continuous(&axis, &signals, &[]);

        assert_eq!(
            results[0]
                .records
                .iter()
                .map(|record| (record.event_axis, record.value))
                .collect::<Vec<_>>(),
            vec![(Some(0.5), 5.0), (Some(1.5), 25.0), (Some(2.5), 65.0)]
        );
        assert_eq!(
            results[1]
                .records
                .iter()
                .map(|record| record.value)
                .collect::<Vec<_>>(),
            vec![10.0, 30.0, 50.0]
        );
    }

    #[test]
    fn continuous_trigger_target_pairs_independent_event_vectors() {
        let axis = [0.0, 1.0, 2.0, 3.0, 4.0];
        let alternating = [-1.0, 1.0, -1.0, 1.0, -1.0];
        let mut signals = HashMap::new();
        signals.insert("ALT".to_string(), alternating.as_slice());
        let mut engine = MeasureEngine::new();
        engine.add(continuous_statement(
            "delay",
            MeasureType::Delay {
                trig: TrigSpec {
                    event: TriggerEvent::When(alternating_condition(2)),
                    td: None,
                },
                targ: TrigSpec {
                    event: TriggerEvent::When(WhenCondition {
                        left: "ALT".to_string(),
                        right: MeasureOperand::Constant(0.5),
                        occurrence: EventOccurrence {
                            edge: EdgeType::Cross,
                            number: 1,
                        },
                        minval: XYCE_DEFAULT_MEASURE_MINVAL,
                    }),
                    td: None,
                },
            },
        ));

        let results = engine.evaluate_continuous(&axis, &signals, &[]);

        assert_eq!(results[0].records.len(), 3);
        assert_eq!(results[0].records[0].trigger_axis, Some(1.5));
        assert_eq!(results[0].records[0].target_axis, Some(0.75));
        assert_eq!(results[0].records[0].value, -0.75);
    }

    #[test]
    fn failed_continuous_delay_retains_the_endpoint_that_was_found() {
        let axis = [0.0, 1.0, 2.0];
        let alternating = [-1.0, 1.0, -1.0];
        let mut signals = HashMap::new();
        signals.insert("ALT".to_string(), alternating.as_slice());
        let found = TrigSpec {
            event: TriggerEvent::When(alternating_condition(1)),
            td: None,
        };
        let missing = TrigSpec {
            event: TriggerEvent::At(10.0),
            td: None,
        };
        let mut engine = MeasureEngine::new();
        engine.add(continuous_statement(
            "missing_target",
            MeasureType::Delay {
                trig: found.clone(),
                targ: missing.clone(),
            },
        ));
        engine.add(continuous_statement(
            "missing_trigger",
            MeasureType::Delay {
                trig: missing,
                targ: found,
            },
        ));

        let results = engine.evaluate_continuous(&axis, &signals, &[]);

        assert!(results.iter().all(|result| result.failure.is_some()));
        assert!(
            results
                .iter()
                .all(|result| result.validate_invariants().is_ok())
        );
        assert_eq!(
            results[0].failure_metadata,
            Some(ContinuousMeasureFailureMetadata {
                trigger_axis: Some(0.5),
                target_axis: None,
            })
        );
        assert_eq!(
            results[1].failure_metadata,
            Some(ContinuousMeasureFailureMetadata {
                trigger_axis: None,
                target_axis: Some(0.5),
            })
        );

        let mut inconsistent = results[0].clone();
        inconsistent
            .records
            .push(ContinuousMeasureRecord::point(1.0, 0.5));
        assert_eq!(
            inconsistent.validate_invariants(),
            Err("failed continuous measurement contains successful records")
        );
    }

    #[test]
    fn continuous_evaluator_rejects_scalar_modes_and_unsupported_measure_types() {
        let axis = [0.0, 1.0];
        let values = [0.0, 1.0];
        let mut signals = HashMap::new();
        signals.insert("Y".to_string(), values.as_slice());
        let mut scalar = continuous_statement(
            "scalar_mode",
            MeasureType::When {
                condition: WhenCondition {
                    left: "Y".to_string(),
                    right: MeasureOperand::Constant(0.5),
                    occurrence: EventOccurrence::default(),
                    minval: XYCE_DEFAULT_MEASURE_MINVAL,
                },
                from: None,
                to: None,
            },
        );
        scalar.analysis = "NOISE".to_string();
        let mut engine = MeasureEngine::new();
        engine.add(scalar);
        engine.add(continuous_statement(
            "unsupported",
            MeasureType::Avg {
                signal: "Y".to_string(),
                from: None,
                to: None,
            },
        ));

        let results = engine.evaluate_continuous(&axis, &signals, &[]);

        assert!(
            results[0]
                .failure
                .as_deref()
                .unwrap()
                .contains("requires TRAN_CONT")
        );
        assert!(results[1].failure.as_deref().unwrap().contains("only WHEN"));
    }
}
