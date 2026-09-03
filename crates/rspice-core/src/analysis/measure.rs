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
use crate::netlist::canonical_symbol;
use crate::netlist::measure::XYCE_DEFAULT_MEASURE_MINVAL;
use std::collections::HashMap;

// The statement types describe a parsed `.MEAS` card, so they live with the
// rest of the deck syntax in `netlist::measure`. They are re-exported here
// because this is where callers expect to find them, and because the engine
// below is the only thing that gives them meaning.
pub use crate::netlist::measure::{
    EdgeType, ErrorFunctionNorm, EventOccurrence, ExtremaOutput, FileErrorNorm, MeasureExpression,
    MeasureOperand, MeasurePrintPolicy, MeasureStatement, MeasureType, TrigSpec, TriggerEvent,
    WhenCondition,
};

//=============================================================================
// Measurement Engine
//=============================================================================

/// Result of a measurement
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureResult {
    /// Measurement name
    pub name: String,
    /// Numeric payload, when the measurement produced one. A failed GOAL/TOL
    /// or FAILVALUE check retains its numeric payload; inspect [`Self::passed`]
    /// and [`Self::error`] for the measurement outcome.
    pub value: Option<Value>,
    /// Exact dependent scalar produced by the measurement before an output
    /// projection such as Xyce `MIN/MAX OUTPUT=TIME|FREQ|SV` replaces the
    /// published value with an independent-axis location. For ordinary
    /// measurements this is identical to [`Self::value`].
    pub raw_value: Option<Value>,
    /// Error message if failed
    pub error: Option<String>,
    /// Whether the measurement produced a value and every authored GOAL/TOL
    /// and FAILVALUE verification contract passed.
    pub passed: bool,
    /// The declared goal, when the statement carried one.
    pub expected: Option<Value>,
    /// The effective tolerance applied to the goal check.
    pub tolerance: Option<Value>,
    /// Authored Xyce `FAILVALUE` threshold, when present.
    pub failure_limit: Option<Value>,
    /// Whether `abs(raw_value) >= failure_limit`. This remains independently
    /// observable when another contract (for example `GOAL/TOL`) also fails.
    pub failure_limit_exceeded: bool,
    /// Independent-axis location associated with a point or extrema result.
    ///
    /// Xyce reports this metadata alongside scalar `AT`, `WHEN`, `MIN`, and
    /// `MAX` measurements. Keeping it typed prevents output adapters and
    /// regression oracles from reverse-engineering the event from a value.
    pub event_axis: Option<Value>,
}

impl MeasureResult {
    pub fn success(name: &str, value: Value) -> Self {
        Self {
            name: name.to_string(),
            value: Some(value),
            raw_value: Some(value),
            error: None,
            passed: true,
            expected: None,
            tolerance: None,
            failure_limit: None,
            failure_limit_exceeded: false,
            event_axis: None,
        }
    }

    pub fn failed(name: &str, error: &str) -> Self {
        Self {
            name: name.to_string(),
            value: None,
            raw_value: None,
            error: Some(error.to_string()),
            passed: false,
            expected: None,
            tolerance: None,
            failure_limit: None,
            failure_limit_exceeded: false,
            event_axis: None,
        }
    }

    /// Build a failed scalar result while retaining every verification
    /// contract authored on the originating statement.
    ///
    /// Evaluation failures have no raw value, so they cannot exceed a
    /// `FAILVALUE` threshold. The authored threshold and GOAL/TOL metadata
    /// remain observable, and the original evaluation error keeps priority.
    pub(super) fn failed_for_statement(statement: &MeasureStatement, error: &str) -> Self {
        Self::failed(&statement.name, error).check_contract(statement)
    }

    fn with_event_axis(mut self, event_axis: Value) -> Self {
        self.event_axis = Some(event_axis);
        self
    }

    fn with_output_projection(mut self, value: Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Apply a statement's verification contracts to a computed result.
    pub(super) fn check_contract(mut self, statement: &MeasureStatement) -> Self {
        if let Some(raw_value) = self.raw_value
            && statement.fail_value.is_some()
            && !raw_value.is_finite()
        {
            self.passed = false;
            self.error
                .get_or_insert_with(|| format!("measurement raw value is non-finite: {raw_value}"));
        }

        if let Some(goal) = statement.goal {
            if !goal.is_finite() {
                self.passed = false;
                self.error
                    .get_or_insert_with(|| format!("GOAL must be finite, got {goal}"));
                self.expected = Some(goal);
                self.tolerance = statement.tolerance;
            } else {
                let tolerance = statement
                    .tolerance
                    .unwrap_or_else(|| (goal.abs() * 0.01).max(1e-12));
                self.expected = Some(goal);
                self.tolerance = Some(tolerance);
                if !tolerance.is_finite() || tolerance < 0.0 {
                    self.passed = false;
                    self.error.get_or_insert_with(|| {
                        format!("TOL must be a finite non-negative value, got {tolerance}")
                    });
                } else if let Some(value) = self.value {
                    if value.is_nan() {
                        self.passed = false;
                        self.error
                            .get_or_insert_with(|| "measurement value is NaN".to_string());
                    } else if (value - goal).abs() > tolerance {
                        self.passed = false;
                        self.error.get_or_insert_with(|| {
                            format!(
                                "value {value:e} misses GOAL {goal:e} (tolerance {tolerance:e})"
                            )
                        });
                    }
                }
            }
        }

        self.failure_limit = statement.fail_value;
        if let (Some(value), Some(limit)) = (self.raw_value, statement.fail_value) {
            self.failure_limit_exceeded = value.abs() >= limit;
            if self.failure_limit_exceeded {
                self.passed = false;
                self.error.get_or_insert_with(|| {
                    format!("measurement magnitude {value:e} meets or exceeds FAILVALUE {limit:e}")
                });
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContinuousMeasureVerificationFailure {
    /// A FAILVALUE comparison cannot classify a NaN or infinity as passing.
    NonFiniteRawValue,
    /// A malformed programmatic threshold must not silently disable checking.
    NonFiniteFailureLimit,
    /// The inclusive `abs(raw_value) >= FAILVALUE` contract was met.
    FailureLimitExceeded,
}

/// Typed independent-axis provenance for one continuous measurement row.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContinuousMeasureCoordinate {
    Point {
        axis: Value,
    },
    Delay {
        trigger_axis: Value,
        target_axis: Value,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContinuousMeasureRecord {
    /// Published record value.
    pub value: Value,
    /// Exact dependent value to which verification contracts apply. This is
    /// additive to `value` so future output projections cannot accidentally
    /// verify an independent-axis coordinate instead.
    pub raw_value: Value,
    pub event_axis: Option<Value>,
    pub trigger_axis: Option<Value>,
    pub target_axis: Option<Value>,
    /// Authored Xyce FAILVALUE threshold for this row.
    pub failure_limit: Option<Value>,
    /// Exact inclusive comparator verdict.
    pub failure_limit_exceeded: bool,
    /// Per-record verification outcome.
    pub passed: bool,
    /// Typed reason for a failed per-record verification contract.
    pub verification_failure: Option<ContinuousMeasureVerificationFailure>,
}

impl ContinuousMeasureRecord {
    fn point(value: Value, event_axis: Value) -> Self {
        Self {
            value,
            raw_value: value,
            event_axis: Some(event_axis),
            trigger_axis: None,
            target_axis: None,
            failure_limit: None,
            failure_limit_exceeded: false,
            passed: true,
            verification_failure: None,
        }
    }

    fn delay(trigger_axis: Value, target_axis: Value) -> Self {
        Self {
            value: target_axis - trigger_axis,
            raw_value: target_axis - trigger_axis,
            event_axis: None,
            trigger_axis: Some(trigger_axis),
            target_axis: Some(target_axis),
            failure_limit: None,
            failure_limit_exceeded: false,
            passed: true,
            verification_failure: None,
        }
    }

    fn check_fail_value(mut self, failure_limit: Option<Value>) -> Self {
        self.failure_limit = failure_limit;
        let Some(limit) = failure_limit else {
            return self;
        };
        self.verification_failure = if !self.raw_value.is_finite() {
            Some(ContinuousMeasureVerificationFailure::NonFiniteRawValue)
        } else if !limit.is_finite() {
            Some(ContinuousMeasureVerificationFailure::NonFiniteFailureLimit)
        } else {
            self.failure_limit_exceeded = self.raw_value.abs() >= limit;
            self.failure_limit_exceeded
                .then_some(ContinuousMeasureVerificationFailure::FailureLimitExceeded)
        };
        self.passed = self.verification_failure.is_none();
        self
    }

    /// Independent-axis coordinate retained for this row.
    pub fn coordinate(&self) -> Option<ContinuousMeasureCoordinate> {
        match (self.event_axis, self.trigger_axis, self.target_axis) {
            (Some(axis), None, None) => Some(ContinuousMeasureCoordinate::Point { axis }),
            (None, Some(trigger_axis), Some(target_axis)) => {
                Some(ContinuousMeasureCoordinate::Delay {
                    trigger_axis,
                    target_axis,
                })
            }
            _ => None,
        }
    }

    /// Stable human-readable verification diagnostic for reporting adapters.
    pub fn verification_failure_message(&self) -> Option<String> {
        match self.verification_failure? {
            ContinuousMeasureVerificationFailure::NonFiniteRawValue => Some(format!(
                "continuous measurement raw value is non-finite: {}",
                self.raw_value
            )),
            ContinuousMeasureVerificationFailure::NonFiniteFailureLimit => Some(format!(
                "continuous measurement FAILVALUE must be finite, got {}",
                self.failure_limit.unwrap_or(Value::NAN)
            )),
            ContinuousMeasureVerificationFailure::FailureLimitExceeded => Some(format!(
                "continuous measurement magnitude {:e} meets or exceeds FAILVALUE {:e}",
                self.raw_value,
                self.failure_limit.unwrap_or(Value::NAN)
            )),
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

/// How a continuous measurement's per-record verdicts become one verdict for
/// the stream.
///
/// The rule is part of the result, not of whatever renders it: CSV, JSON,
/// JUnit and TAP all report the same policy beside the same records, and none
/// of them decides what an aggregate means. Its `Display` form is the stable
/// identifier those formats write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContinuousMeasureAggregatePolicy {
    /// Evaluation must succeed and every retained record must pass its own
    /// verification contract.
    #[default]
    AllRecordsMustPass,
}

impl std::fmt::Display for ContinuousMeasureAggregatePolicy {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AllRecordsMustPass => formatter.write_str("all_records_must_pass"),
        }
    }
}

/// Partial event provenance retained for a failed continuous delay measure.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContinuousMeasureFailureMetadata {
    pub trigger_axis: Option<Value>,
    pub target_axis: Option<Value>,
}

impl ContinuousMeasureResult {
    fn success(name: &str, records: Vec<ContinuousMeasureRecord>) -> Self {
        let result = Self {
            name: name.to_string(),
            records,
            failure: None,
            failure_metadata: None,
        };
        match result.validate_invariants() {
            Ok(()) => result,
            Err(error) => Self::failed(name, error),
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

    fn check_contract(mut self, statement: &MeasureStatement) -> Self {
        if self.failure.is_none() {
            for record in &mut self.records {
                *record = record.check_fail_value(statement.fail_value);
            }
        }
        match self.validate_invariants() {
            Ok(()) => self,
            Err(error) => Self::failed(&statement.name, error),
        }
    }

    /// The rule [`Self::passed`] applies, which every report format names
    /// beside the stream's records.
    pub const fn aggregate_policy(&self) -> ContinuousMeasureAggregatePolicy {
        ContinuousMeasureAggregatePolicy::AllRecordsMustPass
    }

    /// Apply [`Self::aggregate_policy`]: evaluation must succeed and every
    /// emitted record must pass its own verification contract. Failed records
    /// remain present.
    pub fn passed(&self) -> bool {
        self.failure.is_none()
            && !self.records.is_empty()
            && self.records.iter().all(|record| record.passed)
    }

    /// Number of retained rows that failed their per-record contract.
    pub fn failed_record_count(&self) -> usize {
        self.records.iter().filter(|record| !record.passed).count()
    }

    /// Validate the mutually exclusive success/failure representation and
    /// the structural validity of published records. Numeric fields retain
    /// IEEE extended-real values, including NaN, to distinguish a computed
    /// undefined result from an event that was never found.
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
                for record in &self.records {
                    if record.coordinate().is_none() {
                        return Err(
                            "continuous measurement record has invalid coordinate metadata",
                        );
                    }
                    let expected_failure = match record.failure_limit {
                        None => None,
                        Some(_) if !record.raw_value.is_finite() => {
                            Some(ContinuousMeasureVerificationFailure::NonFiniteRawValue)
                        }
                        Some(limit) if !limit.is_finite() => {
                            Some(ContinuousMeasureVerificationFailure::NonFiniteFailureLimit)
                        }
                        Some(limit) if record.raw_value.abs() >= limit => {
                            Some(ContinuousMeasureVerificationFailure::FailureLimitExceeded)
                        }
                        Some(_) => None,
                    };
                    if record.verification_failure != expected_failure {
                        return Err(
                            "continuous measurement typed verification failure is inconsistent",
                        );
                    }
                    if record.passed != expected_failure.is_none() {
                        return Err("continuous measurement record verdict is inconsistent");
                    }
                    if record.failure_limit_exceeded
                        != matches!(
                            expected_failure,
                            Some(ContinuousMeasureVerificationFailure::FailureLimitExceeded)
                        )
                    {
                        return Err("continuous measurement FAILVALUE verdict is inconsistent");
                    }
                }
            }
        }
        Ok(())
    }
}

//=============================================================================
// Measurement Engine
//=============================================================================

/// Normalize a serialized measurement signal name under SPICE's
/// case-insensitive hierarchy namespace.
pub fn canonical_measure_signal_name(name: &str) -> String {
    canonical_symbol(
        &name
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect::<String>(),
    )
}

/// Resolve a signal under SPICE's case-insensitive hierarchy namespace.
/// Distinct columns that collapse under case/separator/whitespace
/// normalization fail closed instead of depending on `HashMap` iteration.
fn lookup_signal<'a>(signals: &HashMap<String, &'a [Value]>, name: &str) -> Option<&'a [Value]> {
    // An exact key represents the execution adapter's authoritative physical
    // spelling and preserves the long-standing solution-node-first contract.
    // Alias projection performs its own ambiguity check before this layer.
    if let Some(signal) = signals.get(name) {
        return Some(*signal);
    }
    signals
        .get(&canonical_measure_signal_index_key(name))
        .copied()
}

const CANONICAL_MEASURE_SIGNAL_INDEX_PREFIX: &str = "\0RSPICE_MEASURE_CANONICAL\0";

fn canonical_measure_signal_index_key(name: &str) -> String {
    format!(
        "{CANONICAL_MEASURE_SIGNAL_INDEX_PREFIX}{}",
        canonical_measure_signal_name(name)
    )
}

/// Clone one adapter-owned signal view and add an internal O(1) canonical
/// lookup namespace. Exact authored keys remain authoritative; ambiguous
/// normalized spellings deliberately receive no index entry.
fn index_measure_signals<'a>(
    signals: &HashMap<String, &'a [Value]>,
) -> HashMap<String, &'a [Value]> {
    let mut indexed = signals.clone();
    let mut canonical = HashMap::<String, (&'a [Value], bool)>::with_capacity(signals.len());
    for (name, waveform) in signals {
        if name.starts_with(CANONICAL_MEASURE_SIGNAL_INDEX_PREFIX) {
            continue;
        }
        canonical
            .entry(canonical_measure_signal_name(name))
            .and_modify(|(existing, ambiguous)| {
                if existing.len() != waveform.len()
                    || !std::ptr::eq(existing.as_ptr(), waveform.as_ptr())
                {
                    *ambiguous = true;
                }
            })
            .or_insert((*waveform, false));
    }
    for (name, (waveform, ambiguous)) in canonical {
        if !ambiguous {
            indexed.insert(
                format!("{CANONICAL_MEASURE_SIGNAL_INDEX_PREFIX}{name}"),
                waveform,
            );
        }
    }
    indexed
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
    /// Xyce compatibility switch for the legacy non-continuous TRAN
    /// RiseFallDelay implementation. Modern TrigTarg remains mandatory for
    /// every other analysis and for all continuous measurement modes.
    use_legacy_tran_trig_targ: bool,
}

impl MeasureEngine {
    pub fn new() -> Self {
        Self {
            measurements: Vec::new(),
            use_legacy_tran_trig_targ: false,
        }
    }

    pub(crate) fn set_use_legacy_tran_trig_targ(&mut self, enabled: bool) {
        self.use_legacy_tran_trig_targ = enabled;
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

    /// Evaluate the raw scalar cached by a file-backed ERROR getter.
    ///
    /// Xyce freezes that getter on its first read, including an IEEE NaN
    /// produced by the norm. Keep the raw scalar inside the declaration-
    /// ordered runtime so dependent measures and the terminal result retain
    /// the same computed numeric payload.
    pub(crate) fn evaluate_file_error_prefix_raw(
        statement: &MeasureStatement,
        axis: &[Value],
        signals: &HashMap<String, &[Value]>,
    ) -> Result<Value, String> {
        let MeasureType::FileError {
            signal,
            file,
            norm,
            independent_column,
            dependent_column,
        } = &statement.measure_type
        else {
            return Err("raw file ERROR evaluation requires a FileError statement".to_string());
        };
        let engine = Self {
            measurements: Vec::new(),
            use_legacy_tran_trig_targ: false,
        };
        let signals = index_measure_signals(signals);
        engine.file_error_value(
            &statement.analysis,
            signal,
            file,
            *norm,
            *independent_column,
            *dependent_column,
            axis,
            &signals,
        )
    }

    /// Evaluate measurements while treating selected sample indices as the
    /// start of a new accepted-point segment. This is required for nested DC
    /// sweeps: the primary sweep restarts for every secondary value, and the
    /// synthetic jump between cycles is not a physical interpolation interval.
    pub(crate) fn evaluate_with_segment_starts(
        &self,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
        segment_starts: &[usize],
    ) -> Vec<MeasureResult> {
        self.evaluate_with_segment_starts_and_context(
            time,
            signals,
            segment_starts,
            &crate::netlist::ParamContext::new(),
        )
    }

    pub(crate) fn evaluate_with_segment_starts_and_context(
        &self,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
        segment_starts: &[usize],
        params: &crate::netlist::ParamContext,
    ) -> Vec<MeasureResult> {
        self.evaluate_with_signal_maps(time, &[signals], segment_starts, params)
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
        let indexed_signals = index_measure_signals(signals);

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
                self.evaluate_continuous_one(statement, axis, &indexed_signals, segment_starts)
                    .check_contract(statement)
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
                td,
                minval,
            } => continuous_when(
                &statement.name,
                &statement.analysis,
                condition,
                *from,
                *to,
                *td,
                *minval,
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
                td,
                minval,
            } => continuous_find(
                &statement.name,
                &statement.analysis,
                signal,
                *at,
                when.as_ref(),
                *from,
                *to,
                *td,
                *minval,
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
                td,
                minval,
            } => continuous_derivative(
                &statement.name,
                &statement.analysis,
                signal,
                *at,
                when.as_ref(),
                *from,
                *to,
                *td,
                *minval,
                axis,
                signals,
                segment_starts,
            ),
            MeasureType::Delay {
                trig, targ, minval, ..
            } => {
                if trig.frac_max.is_some() || targ.frac_max.is_some() {
                    ContinuousMeasureResult::failed(
                        &statement.name,
                        "FRAC_MAX is supported only by scalar TRAN TRIG/TARG",
                    )
                } else {
                    continuous_delay(
                        &statement.name,
                        trig,
                        targ,
                        *minval,
                        axis,
                        signals,
                        segment_starts,
                    )
                }
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
    pub(crate) fn evaluate_with_segment_starts_and_signal_maps_and_context(
        &self,
        time: &[Value],
        signal_maps: &[HashMap<String, &[Value]>],
        segment_starts: &[usize],
        params: &crate::netlist::ParamContext,
    ) -> Vec<MeasureResult> {
        let signal_map_refs = signal_maps.iter().collect::<Vec<_>>();
        self.evaluate_with_signal_maps(time, &signal_map_refs, segment_starts, params)
    }

    fn evaluate_with_signal_maps(
        &self,
        time: &[Value],
        signal_maps: &[&HashMap<String, &[Value]>],
        segment_starts: &[usize],
        params: &crate::netlist::ParamContext,
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
        if segment_starts.iter().enumerate().any(|(index, start)| {
            *start == 0 || *start >= time.len() || index > 0 && *start <= segment_starts[index - 1]
        }) {
            return self.fail_all("measurement segment starts are invalid or unordered");
        }
        let indexed_signal_maps = signal_maps
            .iter()
            .map(|signals| index_measure_signals(signals))
            .collect::<Vec<_>>();
        let signal_maps = indexed_signal_maps.iter().collect::<Vec<_>>();

        // Expression measures (PARAM='...') read other results by name, so
        // they evaluate in a second pass over the directly computed set —
        // and in statement order, so a PARAM may reference an earlier PARAM.
        let mut results: Vec<MeasureResult> = self
            .measurements
            .iter()
            .enumerate()
            .map(|(index, m)| match &m.measure_type {
                MeasureType::Param { .. } | MeasureType::Equation { .. } => {
                    MeasureResult::failed_for_statement(m, "PARAM expression not yet evaluated")
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
                results[idx] = self
                    .eval_param(&m.name, expression, &results, params)
                    .check_contract(m);
            }
        }
        results
    }

    fn fail_all(&self, reason: &str) -> Vec<MeasureResult> {
        self.measurements
            .iter()
            .map(|measurement| MeasureResult::failed_for_statement(measurement, reason))
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
            .check_contract(measurement)
    }

    fn evaluate_kind(
        &self,
        measurement: &MeasureStatement,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
        segment_starts: &[usize],
    ) -> MeasureResult {
        match &measurement.measure_type {
            MeasureType::Delay {
                trig,
                targ,
                from,
                to,
                minval,
            } => self.eval_delay(
                &measurement.name,
                &measurement.analysis,
                trig,
                targ,
                *from,
                *to,
                *minval,
                time,
                signals,
                segment_starts,
            ),
            MeasureType::Derivative {
                signal,
                at,
                when,
                from,
                to,
                td,
                minval,
            } => self.eval_derivative(
                &measurement.name,
                &measurement.analysis,
                signal,
                *at,
                when.as_ref(),
                *from,
                *to,
                *td,
                *minval,
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
                td,
                minval,
            } => self.eval_find(
                &measurement.name,
                &measurement.analysis,
                signal,
                *at,
                when.as_ref(),
                *from,
                *to,
                *td,
                *minval,
                time,
                signals,
                segment_starts,
            ),
            MeasureType::When {
                condition,
                from,
                to,
                td,
                minval,
            } => self.eval_when(
                &measurement.name,
                &measurement.analysis,
                condition,
                *from,
                *to,
                *td,
                *minval,
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
        match self.file_error_value(
            analysis,
            signal_name,
            file,
            norm,
            independent_column,
            dependent_column,
            axis,
            signals,
        ) {
            Ok(value) => MeasureResult::success(name, value),
            Err(error) => MeasureResult::failed(name, &error),
        }
    }

    fn file_error_value(
        &self,
        analysis: &str,
        signal_name: &str,
        file: &str,
        norm: FileErrorNorm,
        independent_column: Option<isize>,
        dependent_column: usize,
        axis: &[Value],
        signals: &HashMap<String, &[Value]>,
    ) -> Result<Value, String> {
        if !matches!(
            analysis.to_ascii_uppercase().as_str(),
            "DC" | "TRAN" | "AC" | "NOISE"
        ) {
            return Err(
                "file-backed ERROR is supported only for DC, TRAN, AC, and NOISE analyses"
                    .to_string(),
            );
        }
        let Some(signal) = lookup_signal(signals, signal_name) else {
            return Err(format!("Signal '{signal_name}' not found"));
        };
        let pairs = if analysis.eq_ignore_ascii_case("DC") {
            let comparison =
                match super::measure_file::read_error_comparison_column(file, dependent_column) {
                    Ok(values) => values,
                    Err(error) => {
                        return Err(format!(
                            "could not load ERROR comparison file '{file}': {error}"
                        ));
                    }
                };
            if signal.len() < comparison.len() {
                return Err(format!(
                    "ERROR comparison has {} rows but the simulation produced only {} accepted points",
                    comparison.len(),
                    signal.len()
                ));
            }
            signal.iter().copied().zip(comparison).collect::<Vec<_>>()
        } else {
            let Some(independent_column) = independent_column else {
                return Err("non-DC ERROR requires a non-negative INDEPVARCOL".to_string());
            };
            let Ok(independent_column) = usize::try_from(independent_column) else {
                return Err("non-DC ERROR requires a non-negative INDEPVARCOL".to_string());
            };
            if independent_column == dependent_column {
                return Err(
                    "non-DC ERROR requires different INDEPVARCOL and DEPVARCOL values".to_string(),
                );
            }
            let comparison = match super::measure_file::read_error_comparison_columns(
                file,
                Some(independent_column),
                dependent_column,
            ) {
                Ok(columns) => columns,
                Err(error) => {
                    return Err(format!(
                        "could not load ERROR comparison file '{file}': {error}"
                    ));
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
                return Err(
                    "non-DC ERROR comparison axis must be monotonically increasing and non-negative"
                        .to_string(),
                );
            }
            let interpolator = AkimaInterpolator::new(axis, signal)?;
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
        Ok(match norm {
            FileErrorNorm::Infinity => infinity,
            FileErrorNorm::L1 => l1 + l1_compensation,
            FileErrorNorm::L2 => l2,
        })
    }

    fn eval_delay(
        &self,
        name: &str,
        analysis: &str,
        trig: &TrigSpec,
        targ: &TrigSpec,
        from: Option<Value>,
        to: Option<Value>,
        minval: Value,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
        segment_starts: &[usize],
    ) -> MeasureResult {
        let legacy = analysis.eq_ignore_ascii_case("TRAN")
            && (self.use_legacy_tran_trig_targ
                || trig.frac_max.is_some()
                || targ.frac_max.is_some());
        if !legacy && (trig.frac_max.is_some() || targ.frac_max.is_some()) {
            return MeasureResult::failed(
                name,
                "FRAC_MAX is supported only by scalar TRAN TRIG/TARG",
            );
        }
        if legacy && matches!(&targ.event, TriggerEvent::At(_)) {
            return MeasureResult::failed(name, "AT keyword not allowed in legacy TARG block");
        }
        // RiseFallDelay has one Base::td_ measurement window. Since the TARG
        // block is parsed last, its TD wins when present and the resulting
        // window gates both clause histories. TrigTarg retains separate TRIG
        // and inherited TARG windows.
        let legacy_td = targ.td.or(trig.td);
        let trigger_td = if legacy { legacy_td } else { trig.td };
        let target_td = if legacy {
            legacy_td
        } else {
            targ.td.or(trig.td)
        };
        if legacy && (trig.frac_max.is_some() || targ.frac_max.is_some()) {
            return match eval_legacy_frac_delay(
                trig, targ, legacy_td, from, to, minval, time, signals,
            ) {
                Ok(Some((trigger, target))) => MeasureResult::success(name, target - trigger),
                Ok(None) => MeasureResult::failed(name, "Trigger or target condition not found"),
                Err(error) => MeasureResult::failed(name, &error),
            };
        }
        let t_trig = match delay_clause_event(
            trig,
            trigger_td,
            from,
            to,
            minval,
            time,
            signals,
            segment_starts,
            legacy,
            None,
        ) {
            Ok(Some(value)) => value,
            Ok(None) => return MeasureResult::failed(name, "Trigger condition not found"),
            Err(error) => return MeasureResult::failed(name, &error),
        };
        let t_targ = match delay_clause_event(
            targ,
            target_td,
            from,
            to,
            minval,
            time,
            signals,
            segment_starts,
            legacy,
            legacy.then_some(t_trig),
        ) {
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
        Self::axis_in_measurement_window_with_minval(
            axis,
            lower,
            upper,
            XYCE_DEFAULT_MEASURE_MINVAL,
        )
    }

    fn axis_in_measurement_window_with_minval(
        axis: Value,
        lower: Value,
        upper: Value,
        minval: Value,
    ) -> bool {
        let lower_tolerance = if lower.is_finite() {
            lower.abs() * minval
        } else {
            0.0
        };
        let upper_tolerance = if upper.is_finite() {
            upper.abs() * minval
        } else {
            0.0
        };
        axis >= lower - lower_tolerance && axis <= upper + upper_tolerance
    }

    fn point_measurement_window_bounds(
        axis: &[Value],
        analysis: &str,
        from: Option<Value>,
        to: Option<Value>,
        td: Option<Value>,
    ) -> (Value, Value) {
        let (mut lower, upper) = Self::measurement_window_bounds(axis, from, to);
        if matches!(analysis.to_ascii_uppercase().as_str(), "TRAN" | "TRAN_CONT")
            && let Some(td) = td
        {
            lower = lower.max(td);
        }
        (lower, upper)
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
        let mut result = MeasureResult::success(name, selected_value);
        if output == ExtremaOutput::IndependentAxis {
            result = result.with_output_projection(time[selected_index]);
        }
        result.with_event_axis(time[selected_index])
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
        let mut selected: Option<(Value, Value)> = None;
        for (&axis, &value) in time.iter().zip(signal) {
            if Self::axis_in_measurement_window(axis, lower, upper) {
                match &mut selected {
                    None => selected = Some((value, value)),
                    Some((minimum, maximum)) => {
                        if value < *minimum {
                            *minimum = value;
                        }
                        if value > *maximum {
                            *maximum = value;
                        }
                    }
                }
            }
        }
        let Some((min_val, max_val)) = selected else {
            return MeasureResult::failed(name, "Empty range");
        };

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
        let mut window_active = false;
        for ((&axis_value, &measured_value), &comparison_value) in
            axis.iter().zip(measured).zip(comparison)
        {
            if !axis_in_error_window(axis_value, lower, upper, minval) {
                continue;
            }
            window_active = true;
            let magnitude = measured_value.abs();
            let ymin_tolerance = ymin.abs() * 1.0e-12;
            let ymax_tolerance = ymax.abs() * 1.0e-12;
            if !(magnitude >= ymin - ymin_tolerance && magnitude <= ymax + ymax_tolerance) {
                continue;
            }
            let denominator = magnitude.max(minval);
            let relative_error = (measured_value - comparison_value) / denominator;
            sum += match norm {
                ErrorFunctionNorm::RootMeanSquare => relative_error * relative_error,
                ErrorFunctionNorm::MeanAbsolute => relative_error.abs(),
            };
            count += 1;
        }
        if count == 0 {
            return if window_active {
                // Xyce initializes ERR on entry to the axis window, then its
                // zero-sample getter computes 0/0. Preserve that computed NaN
                // distinctly from a window that was never entered.
                MeasureResult::success(name, Value::NAN)
            } else {
                MeasureResult::failed(name, "ERR window contains no points")
            };
        }
        let mean = sum / count as Value;
        let result = match norm {
            ErrorFunctionNorm::RootMeanSquare => mean.sqrt(),
            ErrorFunctionNorm::MeanAbsolute => mean,
        };
        MeasureResult::success(name, result)
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

        match rise_fall_duration(time, signal, from_pct, to_pct, number, is_rise) {
            Some(duration) => MeasureResult::success(name, duration),
            _ => MeasureResult::failed(name, "Rise/fall transition not found"),
        }
    }

    /// Segment slope of the interpolating polyline at the requested time —
    /// the same piecewise-linear data model every other measure uses.
    #[allow(clippy::too_many_arguments)]
    fn eval_derivative(
        &self,
        name: &str,
        analysis: &str,
        signal_name: &str,
        at: Option<Value>,
        when: Option<&WhenCondition>,
        from: Option<Value>,
        to: Option<Value>,
        td: Option<Value>,
        minval: Value,
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
        let (lower, upper) = Self::point_measurement_window_bounds(time, analysis, from, to, td);

        if let Some(target) = at {
            if !Self::axis_in_measurement_window_with_minval(target, lower, upper, minval) {
                return MeasureResult::failed(name, "AT point is outside the measurement window")
                    .with_event_axis(target);
            }
            return match derivative_at_accepted_points(time, signal, target, minval, segment_starts)
            {
                Ok(Some(slope)) => MeasureResult::success(name, slope).with_event_axis(target),
                Ok(None) => MeasureResult::failed(name, "Time point not in simulation range")
                    .with_event_axis(target),
                Err(error) => MeasureResult::failed(name, error).with_event_axis(target),
            };
        }

        let Some(condition) = when else {
            return MeasureResult::failed(name, "DERIV requires AT=time or WHEN signal=value");
        };
        match first_measure_condition_event(
            condition,
            minval,
            time,
            signals,
            lower,
            upper,
            segment_starts,
        ) {
            Ok(Some((segment, _, event_axis, _))) => {
                return measurement_segment_slope(name, time, signal, segment)
                    .with_event_axis(event_axis);
            }
            Err(error) => return MeasureResult::failed(name, &error),
            Ok(None) => {}
        }
        MeasureResult::failed(name, "WHEN condition never met in the measurement window")
    }

    /// Evaluate a PARAM expression against the named results computed so far.
    fn eval_param(
        &self,
        name: &str,
        expression: &MeasureExpression,
        prior: &[MeasureResult],
        params: &crate::netlist::ParamContext,
    ) -> MeasureResult {
        let mut ctx = params.clone();
        for result in prior {
            if let Some(value) = result.raw_value {
                ctx.set(&result.name, value);
            }
        }
        let parsed = match crate::netlist::expr::parse_expression(&expression.text) {
            Ok(parsed) => parsed,
            Err(err) => {
                return MeasureResult::failed(name, &format!("PARAM expression failed: {err}"));
            }
        };
        match crate::netlist::expr::evaluate_complex_raw(&parsed, &ctx) {
            Ok(value) => {
                let xyce = params.expression_dialect() == crate::config::ExpressionDialect::Xyce;
                let value = if xyce && expression.is_expression() {
                    crate::netlist::expr::normalize_xyce_expression_result(value)
                } else {
                    value
                };
                if !xyce {
                    if value.re.is_nan() || value.im.is_nan() {
                        return MeasureResult::failed(
                            name,
                            &format!(
                                "PARAM expression produced NaN ({} {:+}j)",
                                value.re, value.im
                            ),
                        );
                    }
                    let imag_tolerance = if value.re.is_finite() {
                        1.0e-15 * value.re.abs().max(1.0)
                    } else {
                        0.0
                    };
                    if value.im.abs() > imag_tolerance {
                        return MeasureResult::failed(
                            name,
                            &format!(
                                "PARAM expression produced complex value ({} {:+}j); scalar measurement results must be real",
                                value.re, value.im
                            ),
                        );
                    }
                }
                // MeasureBase applies fixNan/fixInf to both components at an
                // authored ExpressionOp root, then exposes the real output.
                // Typed raw MeasureOp references deliberately bypass that
                // normalization and retain their IEEE real projection.
                MeasureResult::success(name, value.re)
            }
            Err(err) => MeasureResult::failed(name, &format!("PARAM expression failed: {err}")),
        }
    }

    fn eval_find(
        &self,
        name: &str,
        analysis: &str,
        signal_name: &str,
        at: Option<Value>,
        when: Option<&WhenCondition>,
        from: Option<Value>,
        to: Option<Value>,
        td: Option<Value>,
        minval: Value,
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

        let (lower, upper) = Self::point_measurement_window_bounds(time, analysis, from, to, td);
        if let Some(t_at) = at {
            // FIND ... AT=time
            if !Self::axis_in_measurement_window_with_minval(t_at, lower, upper, minval) {
                return MeasureResult::failed(name, "AT point is outside the measurement window")
                    .with_event_axis(t_at);
            }
            match find_at_accepted_points(time, signal, t_at, minval, segment_starts) {
                Ok(Some(value)) => {
                    return MeasureResult::success(name, value).with_event_axis(t_at);
                }
                Err(error) => return MeasureResult::failed(name, error).with_event_axis(t_at),
                Ok(None) => {}
            }
            return MeasureResult::failed(name, "Time point not in simulation range")
                .with_event_axis(t_at);
        }

        if let Some(condition) = when {
            match first_measure_condition_event(
                condition,
                minval,
                time,
                signals,
                lower,
                upper,
                segment_starts,
            ) {
                Ok(Some((segment, fraction, event_axis, current_within_minval))) => {
                    // Xyce's FIND-WHEN uses the current accepted-row value
                    // when that row is already inside the MINVAL equality
                    // band. Only a strict interval crossing interpolates.
                    let value = if current_within_minval {
                        signal[segment + 1]
                    } else {
                        interpolate_extended_real(signal[segment], signal[segment + 1], fraction)
                    };
                    return MeasureResult::success(name, value).with_event_axis(event_axis);
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
        analysis: &str,
        condition: &WhenCondition,
        from: Option<Value>,
        to: Option<Value>,
        td: Option<Value>,
        minval: Value,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
        segment_starts: &[usize],
    ) -> MeasureResult {
        let (lower, upper) = Self::point_measurement_window_bounds(time, analysis, from, to, td);
        match first_measure_condition_event(
            condition,
            minval,
            time,
            signals,
            lower,
            upper,
            segment_starts,
        ) {
            Ok(Some((_, _, axis, _))) => MeasureResult::success(name, axis).with_event_axis(axis),
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
    from: Option<Value>,
    to: Option<Value>,
    minval: Value,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    segment_starts: &[usize],
    legacy: bool,
    after: Option<Value>,
) -> Result<Option<Value>, String> {
    match &clause.event {
        TriggerEvent::At(target) => {
            if !target.is_finite() {
                return Ok(None);
            }
            if legacy {
                Ok(axis.iter().copied().find(|sample| {
                    legacy_delay_accepts_sample(*sample, effective_td, from, to, minval)
                        && *sample >= *target
                }))
            } else {
                Ok(delay_at_is_reached(axis, *target, segment_starts, minval).then_some(*target))
            }
        }
        TriggerEvent::When(condition) => {
            let left = lookup_signal(signals, &condition.left)
                .ok_or_else(|| format!("When signal '{}' not found", condition.left))?;
            let right = resolve_measure_operand(&condition.right, signals)?;
            let mut tracker = if legacy {
                DelayConditionTracker::new_legacy(
                    condition.occurrence.edge,
                    condition.occurrence.number,
                    clause.occurrence_explicit,
                    minval,
                )
            } else {
                DelayConditionTracker::new(
                    condition.occurrence.edge,
                    condition.occurrence.number,
                    clause.occurrence_explicit,
                    minval,
                )
            };
            let mut events = Vec::new();
            for row in 0..axis.len() {
                if legacy && !legacy_delay_accepts_sample(axis[row], effective_td, from, to, minval)
                {
                    continue;
                }
                if segment_starts.binary_search(&row).is_ok() {
                    tracker.reset_segment();
                }
                let Some(right_value) = right.value_at(row) else {
                    return Ok(None);
                };
                if let Some(event_axis) = tracker.update_with_td(
                    axis[row],
                    left[row],
                    right_value,
                    (!legacy).then_some(effective_td).flatten(),
                ) && after.is_none_or(|trigger| axis[row] > trigger)
                {
                    events.push(event_axis);
                }
            }
            Ok(if condition.occurrence.number < 0 && !legacy {
                let distance = condition.occurrence.number.unsigned_abs();
                events.iter().rev().nth(distance - 1).copied()
            } else if condition.occurrence.number < 0 {
                // RiseFallDelay interprets every negative RFC value as LAST.
                events.last().copied()
            } else {
                events.first().copied()
            })
        }
    }
}

fn resolve_legacy_frac_clause<'a>(
    clause: &TrigSpec,
    signals: &HashMap<String, &'a [Value]>,
) -> Result<(Option<&'a [Value]>, ResolvedMeasureOperand<'a>), String> {
    match &clause.event {
        TriggerEvent::At(_) => Ok((None, ResolvedMeasureOperand::Constant(0.0))),
        TriggerEvent::When(condition) => {
            let left = lookup_signal(signals, &condition.left)
                .ok_or_else(|| format!("When signal '{}' not found", condition.left))?;
            Ok((
                Some(left),
                resolve_measure_operand(&condition.right, signals)?,
            ))
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn eval_legacy_frac_delay(
    trig: &TrigSpec,
    targ: &TrigSpec,
    td: Option<Value>,
    from: Option<Value>,
    to: Option<Value>,
    minval: Value,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
) -> Result<Option<(Value, Value)>, String> {
    let (trigger_signal, trigger_target) = resolve_legacy_frac_clause(trig, signals)?;
    let (Some(target_signal), target_target) = resolve_legacy_frac_clause(targ, signals)? else {
        return Ok(None);
    };
    let mut tracker = LegacyFracDelayTracker::new(trig, targ, minval);
    let mut result = None;
    for row in 0..axis.len() {
        if !legacy_delay_accepts_sample(axis[row], td, from, to, minval) {
            continue;
        }
        let trigger_value = trigger_signal.map_or(0.0, |signal| signal[row]);
        let Some(trigger_target) = trigger_target.value_at(row) else {
            return Ok(None);
        };
        let Some(target_target) = target_target.value_at(row) else {
            return Ok(None);
        };
        result = tracker.update(
            axis[row],
            trigger_value,
            trigger_target,
            target_signal[row],
            target_target,
        );
    }
    Ok(result)
}

/// Stateful detector for Xyce TRIG/TARG delay clauses.
///
/// Xyce 7.10 has two deliberately different implementations. Modern
/// `TrigTarg` is the default and uses moving-target interpolation. Legacy
/// `RiseFallDelay` is selected only for non-continuous TRAN by USE_LTTM (or
/// FRAC_MAX) and retains the older sign-history/equality-on-departure rules.
#[derive(Debug, Clone)]
pub(crate) struct DelayConditionTracker {
    mode: DelayTrackerMode,
    edge: EdgeType,
    number: isize,
    occurrence_explicit: bool,
    minval: Value,
    actual_count: usize,
    previous: Option<(Value, Value, Value)>,
    continuous: bool,
    legacy_negative_search_open: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DelayTrackerMode {
    Modern,
    Legacy,
}

/// Streaming state for Xyce's legacy `RiseFallDelay` FRAC_MAX path.
///
/// Unlike an ordinary WHEN target, FRAC_MAX is revised whenever a larger
/// in-window maximum is observed.  Xyce consequently retains the accepted
/// waveform history and replays it from the last useful bracket.  Keeping the
/// same state machine here makes batch and live measurement consumers share
/// one implementation and avoids an end-of-run approximation.
#[derive(Debug, Clone)]
pub(crate) struct LegacyFracDelayTracker {
    trigger: LegacyFracClauseTracker,
    target: LegacyFracClauseTracker,
    minval: Value,
    initialized: bool,
    previous_trigger: Value,
    previous_target: Value,
}

#[derive(Debug, Clone)]
struct LegacyFracClauseTracker {
    at: Option<Value>,
    edge: EdgeType,
    number: isize,
    occurrence_explicit: bool,
    frac_max: Option<Value>,
    history: Vec<(Value, Value)>,
    maximum: Value,
    result_index: usize,
    found: Option<Value>,
    output_target: Value,
    target_changed: bool,
    actual_rise: usize,
    actual_fall: usize,
    actual_cross: usize,
    is_rising: bool,
    is_falling: bool,
    last_output: Value,
}

impl LegacyFracClauseTracker {
    fn new(clause: &TrigSpec) -> Self {
        let (at, edge, number) = match &clause.event {
            TriggerEvent::At(at) => (Some(*at), EdgeType::Cross, 1),
            TriggerEvent::When(condition) => {
                (None, condition.occurrence.edge, condition.occurrence.number)
            }
        };
        Self {
            at,
            edge,
            number,
            occurrence_explicit: clause.occurrence_explicit,
            frac_max: clause.frac_max,
            history: Vec::new(),
            maximum: 0.0,
            result_index: 0,
            found: None,
            output_target: 0.0,
            target_changed: false,
            actual_rise: 0,
            actual_fall: 0,
            actual_cross: 0,
            is_rising: false,
            is_falling: false,
            last_output: 0.0,
        }
    }

    fn variable_history(&self) -> bool {
        self.frac_max.is_some() && self.at.is_none()
    }

    fn within_rfc_window(&self) -> bool {
        if !self.occurrence_explicit {
            return true;
        }
        let actual = match self.edge {
            EdgeType::Rise => self.actual_rise,
            EdgeType::Fall => self.actual_fall,
            EdgeType::Cross => self.actual_cross,
        };
        self.number < 0 || actual == self.number as usize
    }

    fn variable_history_changed(&self, value: Value, minval: Value) -> bool {
        self.variable_history()
            && self
                .history
                .last()
                .is_none_or(|(_, previous)| (previous - value).abs() > minval)
    }

    fn record_before_count(
        &mut self,
        axis: Value,
        value: Value,
        previous_value: Value,
        record_data: bool,
    ) {
        if self.variable_history() {
            if record_data && self.within_rfc_window() {
                self.history.push((axis, value));
                if value > self.maximum || previous_value > self.maximum {
                    // Preserve C++ comparison/selection behavior exactly:
                    // unlike f64::max, a NaN previous sample is selected when
                    // the current sample alone makes the outer condition true.
                    self.maximum = if value >= previous_value {
                        value
                    } else {
                        previous_value
                    };
                    self.target_changed = true;
                }
            }
        } else if self.at.is_none() {
            self.history.push((axis, value));
            if self.history.len() > 2 {
                self.history.remove(0);
            }
            self.result_index = 0;
        }
    }

    /// Update legacy RFC counters and report a newly entered requested window
    /// for the non-FRAC LAST reopen/close rule.
    fn update_rfc_counts(&mut self, value: Value, target: Value) -> bool {
        if !self.occurrence_explicit {
            return false;
        }
        let mut new_rise = false;
        let mut new_fall = false;
        if self.frac_max.is_some() {
            if value > self.last_output && !self.is_rising {
                self.is_rising = true;
                self.is_falling = false;
                self.actual_rise += 1;
            }
            if value < self.last_output && !self.is_falling {
                self.is_rising = false;
                self.is_falling = true;
                self.actual_fall += 1;
            }
        } else {
            if value - target >= 0.0 && self.last_output - target < 0.0 {
                self.actual_rise += 1;
                new_rise = true;
            } else if value - target <= 0.0 && self.last_output - target > 0.0 {
                self.actual_fall += 1;
                new_fall = true;
            }
        }
        let cross_target = if self.frac_max.is_some() { 0.0 } else { target };
        let current = value - cross_target;
        let previous = self.last_output - cross_target;
        let crossed = current <= 0.0 && previous > 0.0 || current >= 0.0 && previous < 0.0;
        if crossed {
            self.actual_cross += 1;
        }
        self.last_output = value;
        self.number < 0
            && self.frac_max.is_none()
            && match self.edge {
                EdgeType::Rise => new_rise,
                EdgeType::Fall => new_fall,
                EdgeType::Cross => crossed,
            }
    }

    fn refresh_target(&mut self, authored_target: Value) {
        if let Some(frac_max) = self.frac_max {
            if self.target_changed {
                self.output_target = frac_max * self.maximum;
            }
        } else {
            // Legacy moving-RHS syntax uses the current RHS as a fixed level
            // while searching the retained dependent-variable bracket.
            self.output_target = authored_target;
        }
    }

    fn search_history(&mut self, minval: Value, after: Option<Value>) -> Option<Value> {
        if self.history.len() < 2 {
            return self.found;
        }
        for index in self.result_index..self.history.len() - 1 {
            let (axis, value) = self.history[index];
            let (next_axis, next_value) = self.history[index + 1];
            if after.is_some_and(|trigger| next_axis <= trigger) {
                continue;
            }
            let difference = value - self.output_target;
            let next_difference = next_value - self.output_target;
            if (difference < 0.0) != (next_difference < 0.0) {
                if (next_value - value).abs() < minval {
                    // RiseFallDelay updates an already-found dynamic result to
                    // the left bracket, but deliberately does not turn this
                    // near-flat first crossing into a found result.
                    if self.found.is_some() {
                        self.found = Some(axis);
                    }
                } else {
                    self.found = Some(
                        (next_axis - axis) * ((self.output_target - value) / (next_value - value))
                            + axis,
                    );
                }
                self.target_changed = false;
                self.result_index = index;
                break;
            }
            if difference.abs() < minval && next_difference.abs() >= minval {
                self.found = Some(axis);
            }
        }
        self.found
    }

    fn prune_consumed_history(&mut self) {
        // Match Xyce's bounded-history maintenance. Retain the bracket at the
        // consumed index so a revised FRAC_MAX target can resume from it.
        const PRUNING_THRESHOLD: usize = 1000;
        if self.variable_history() && self.result_index > PRUNING_THRESHOLD {
            self.history.drain(..self.result_index);
            self.result_index = 0;
        }
    }
}

impl LegacyFracDelayTracker {
    pub(crate) fn new(trigger: &TrigSpec, target: &TrigSpec, minval: Value) -> Self {
        Self {
            trigger: LegacyFracClauseTracker::new(trigger),
            target: LegacyFracClauseTracker::new(target),
            minval,
            initialized: false,
            previous_trigger: 0.0,
            previous_target: 0.0,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn update(
        &mut self,
        axis: Value,
        trigger_value: Value,
        trigger_target: Value,
        target_value: Value,
        target_target: Value,
    ) -> Option<(Value, Value)> {
        if !self.initialized {
            self.trigger.last_output = trigger_value;
            self.target.last_output = target_value;
            self.initialized = true;
        }

        let record_data = self
            .trigger
            .variable_history_changed(trigger_value, self.minval)
            || self
                .target
                .variable_history_changed(target_value, self.minval);
        self.trigger
            .record_before_count(axis, trigger_value, self.previous_trigger, record_data);
        self.target
            .record_before_count(axis, target_value, self.previous_target, record_data);

        self.trigger.refresh_target(trigger_target);
        let new_trigger_window = self
            .trigger
            .update_rfc_counts(trigger_value, self.trigger.output_target);
        if new_trigger_window {
            self.trigger.found = None;
            self.target.found = None;
        }
        let new_target_window = self
            .target
            .update_rfc_counts(target_value, self.target.output_target);
        if new_target_window {
            self.target.found = None;
        }

        if let Some(at) = self.trigger.at {
            if self.trigger.found.is_none() && axis >= at {
                self.trigger.found = Some(axis);
            }
        } else if (self.trigger.found.is_none() || self.trigger.target_changed)
            && self.trigger.within_rfc_window()
        {
            let old_trigger = self.trigger.found;
            self.trigger.search_history(self.minval, None);
            if self.trigger.found != old_trigger
                && self
                    .trigger
                    .found
                    .zip(self.target.found)
                    .is_some_and(|(trigger, target)| target < trigger)
            {
                self.target.found = None;
            }
        }

        self.target.refresh_target(target_target);
        if let Some(trigger) = self.trigger.found
            && (self.target.found.is_none()
                || self.target.target_changed
                || self.target.found.is_some_and(|target| target < trigger))
            && self.target.within_rfc_window()
        {
            self.target.search_history(self.minval, Some(trigger));
        }

        self.trigger.prune_consumed_history();
        self.target.prune_consumed_history();

        self.previous_trigger = trigger_value;
        self.previous_target = target_value;
        self.trigger.found.zip(self.target.found)
    }
}

impl DelayConditionTracker {
    pub(crate) fn new(
        edge: EdgeType,
        number: isize,
        occurrence_explicit: bool,
        minval: Value,
    ) -> Self {
        Self {
            mode: DelayTrackerMode::Modern,
            edge,
            number,
            occurrence_explicit,
            minval,
            actual_count: 0,
            previous: None,
            continuous: false,
            legacy_negative_search_open: true,
        }
    }

    pub(crate) fn new_legacy(
        edge: EdgeType,
        number: isize,
        occurrence_explicit: bool,
        minval: Value,
    ) -> Self {
        let mut tracker = Self::new(edge, number, occurrence_explicit, minval);
        tracker.mode = DelayTrackerMode::Legacy;
        tracker
    }

    pub(crate) fn new_continuous(
        edge: EdgeType,
        number: isize,
        occurrence_explicit: bool,
        minval: Value,
    ) -> Self {
        let mut tracker = Self::new(edge, number, occurrence_explicit, minval);
        tracker.continuous = true;
        tracker
    }

    pub(crate) fn reset_segment(&mut self) {
        self.previous = None;
    }

    pub(crate) fn minval(&self) -> Value {
        self.minval
    }

    pub(crate) fn update_with_td(
        &mut self,
        axis: Value,
        value: Value,
        target: Value,
        td: Option<Value>,
    ) -> Option<Value> {
        let previous = self.previous.replace((axis, value, target));
        let (previous_axis, previous_value, previous_target) = previous?;
        match self.mode {
            DelayTrackerMode::Modern => self.update_modern(
                previous_axis,
                previous_value,
                previous_target,
                axis,
                value,
                target,
                td,
            ),
            DelayTrackerMode::Legacy => {
                self.update_legacy(previous_axis, previous_value, axis, value, target)
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn update_modern(
        &mut self,
        previous_axis: Value,
        previous_value: Value,
        previous_target: Value,
        axis: Value,
        value: Value,
        target: Value,
        td: Option<Value>,
    ) -> Option<Value> {
        // TrigTarg::isWHENcondition rejects a constant dependent signal even
        // when a moving RHS passes through it.
        if value == previous_value {
            return None;
        }
        let previous_difference = previous_value - previous_target;
        let current_difference = value - target;
        let found = current_difference.abs() < self.minval
            || previous_difference < 0.0 && current_difference > 0.0
            || previous_difference > 0.0 && current_difference < 0.0;
        if !found {
            return None;
        }

        let delta_axis = axis - previous_axis;
        let dependent_slope = (value - previous_value) / delta_axis;
        let target_slope = (target - previous_target) / delta_axis;
        let dependent_intercept = value - dependent_slope * axis;
        let target_intercept = target - target_slope * axis;
        let event_axis =
            if dependent_slope == target_slope && target_intercept == dependent_intercept {
                axis
            } else {
                (target_intercept - dependent_intercept) / (dependent_slope - target_slope)
            };
        if !delay_td_accepts_modern_event(event_axis, td, self.minval) {
            return None;
        }

        let rise = value > previous_value;
        let fall = value < previous_value;
        let requested = !self.occurrence_explicit
            || match self.edge {
                EdgeType::Rise => rise,
                EdgeType::Fall => fall,
                EdgeType::Cross => true,
            };
        if !requested {
            return None;
        }
        self.actual_count += 1;
        if self.number > 0
            && if self.continuous {
                self.actual_count < self.number as usize
            } else {
                self.actual_count != self.number as usize
            }
        {
            return None;
        }
        Some(event_axis)
    }

    fn update_legacy(
        &mut self,
        previous_axis: Value,
        previous_value: Value,
        axis: Value,
        value: Value,
        target: Value,
    ) -> Option<Value> {
        let previous_difference = previous_value - target;
        let current_difference = value - target;

        if self.occurrence_explicit {
            let rise = current_difference >= 0.0 && previous_difference < 0.0;
            let fall = current_difference <= 0.0 && previous_difference > 0.0;
            let crossed = rise || fall;
            let requested_window = match self.edge {
                EdgeType::Rise => rise,
                EdgeType::Fall => fall,
                EdgeType::Cross => crossed,
            };
            self.actual_count += usize::from(requested_window);
            if self.number < 0 {
                if requested_window {
                    self.legacy_negative_search_open = true;
                }
                if !self.legacy_negative_search_open {
                    return None;
                }
            }
            if self.number >= 0
                && if self.continuous {
                    self.actual_count < self.number as usize
                } else {
                    self.actual_count != self.number as usize
                }
            {
                return None;
            }
        }

        let previous_negative = previous_difference < 0.0;
        let current_negative = current_difference < 0.0;
        if previous_negative != current_negative {
            if (value - previous_value).abs() < self.minval {
                // Xyce avoids dividing here but does not mark the clause as
                // found in this branch.
                return None;
            }
            let event = Some(
                (axis - previous_axis) * ((target - previous_value) / (value - previous_value))
                    + previous_axis,
            );
            if self.occurrence_explicit && self.number < 0 {
                self.legacy_negative_search_open = false;
            }
            return event;
        }
        if previous_difference.abs() < self.minval && current_difference.abs() >= self.minval {
            if self.occurrence_explicit && self.number < 0 {
                self.legacy_negative_search_open = false;
            }
            return Some(previous_axis);
        }
        None
    }
}

fn delay_at_is_reached(
    axis: &[Value],
    target: Value,
    segment_starts: &[usize],
    minval: Value,
) -> bool {
    let Some((&minimum, &maximum)) = axis
        .iter()
        .min_by(|left, right| left.total_cmp(right))
        .zip(axis.iter().max_by(|left, right| left.total_cmp(right)))
    else {
        return false;
    };
    if target < minimum || target > maximum {
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
            sample - minval >= target
        } else {
            sample - minval <= target
        }
    })
}

pub(crate) fn delay_td_accepts_sample(axis: Value, td: Option<Value>, minval: Value) -> bool {
    td.is_none_or(|td| axis.partial_cmp(&(td * (1.0 - minval))) != Some(std::cmp::Ordering::Less))
}

pub(crate) fn legacy_delay_accepts_sample(
    axis: Value,
    td: Option<Value>,
    from: Option<Value>,
    to: Option<Value>,
    minval: Value,
) -> bool {
    delay_td_accepts_sample(axis, td, minval)
        && from.is_none_or(|from| {
            axis.partial_cmp(&(from * (1.0 - minval))) != Some(std::cmp::Ordering::Less)
        })
        && to.is_none_or(|to| {
            axis.partial_cmp(&(to * (1.0 + minval))) != Some(std::cmp::Ordering::Greater)
        })
}

fn delay_td_accepts_modern_event(axis: Value, td: Option<Value>, minval: Value) -> bool {
    td.is_none_or(|td| axis > td * (1.0 - minval))
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
    minval: Value,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    lower: Value,
    upper: Value,
    segment_starts: &[usize],
) -> Result<Option<MeasureEvent>, String> {
    let left = lookup_signal(signals, &condition.left)
        .ok_or_else(|| format!("When signal '{}' not found", condition.left))?;
    let right = resolve_measure_operand(&condition.right, signals)?;
    let candidates =
        measurement_condition_candidates(left, right, axis.len(), segment_starts, minval);
    Ok(select_measure_condition_occurrence(
        candidates.into_iter().filter_map(|(segment, crossing)| {
            let event_axis =
                axis[segment] + crossing.fraction * (axis[segment + 1] - axis[segment]);
            point_event_axis_in_window(event_axis, lower, upper, minval).then_some((
                segment,
                crossing.fraction,
                event_axis,
                crossing.current_within_minval,
                crossing.direction,
            ))
        }),
        condition.occurrence.edge,
        condition.occurrence.number,
    ))
}

type MeasureEvent = (usize, Value, Value, bool);

fn continuous_condition_events(
    condition: &WhenCondition,
    minval: Value,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    lower: Value,
    upper: Value,
    segment_starts: &[usize],
) -> Result<Vec<MeasureEvent>, String> {
    let left = lookup_signal(signals, &condition.left)
        .ok_or_else(|| format!("When signal '{}' not found", condition.left))?;
    let right = resolve_measure_operand(&condition.right, signals)?;
    let candidates =
        measurement_condition_candidates(left, right, axis.len(), segment_starts, minval);
    Ok(select_continuous_measure_condition_occurrences(
        candidates.into_iter().filter_map(|(segment, crossing)| {
            let event_axis =
                axis[segment] + crossing.fraction * (axis[segment + 1] - axis[segment]);
            point_event_axis_in_window(event_axis, lower, upper, minval).then_some((
                segment,
                crossing.fraction,
                event_axis,
                crossing.current_within_minval,
                crossing.direction,
            ))
        }),
        condition.occurrence.edge,
        condition.occurrence.number,
    ))
}

pub(crate) fn point_event_axis_in_window(
    axis: Value,
    lower: Value,
    upper: Value,
    minval: Value,
) -> bool {
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
    // Xyce's window routines reject only an affirmative out-of-window
    // comparison. IEEE NaN therefore remains in-window and is retained as a
    // computed event location by both raw getters and terminal results.
    !(axis < lower - lower_tolerance || axis > upper + upper_tolerance)
}

fn continuous_when(
    name: &str,
    analysis: &str,
    condition: &WhenCondition,
    from: Option<Value>,
    to: Option<Value>,
    td: Option<Value>,
    minval: Value,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    segment_starts: &[usize],
) -> ContinuousMeasureResult {
    let (lower, upper) =
        MeasureEngine::point_measurement_window_bounds(axis, analysis, from, to, td);
    match continuous_condition_events(
        condition,
        minval,
        axis,
        signals,
        lower,
        upper,
        segment_starts,
    ) {
        Ok(events) if !events.is_empty() => ContinuousMeasureResult::success(
            name,
            events
                .into_iter()
                .map(|(_, _, event_axis, _)| ContinuousMeasureRecord::point(event_axis, event_axis))
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
    analysis: &str,
    signal_name: &str,
    at: Option<Value>,
    when: Option<&WhenCondition>,
    from: Option<Value>,
    to: Option<Value>,
    td: Option<Value>,
    minval: Value,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    segment_starts: &[usize],
) -> ContinuousMeasureResult {
    let Some(signal) = lookup_signal(signals, signal_name) else {
        return ContinuousMeasureResult::failed(name, format!("Signal '{signal_name}' not found"));
    };
    let (lower, upper) =
        MeasureEngine::point_measurement_window_bounds(axis, analysis, from, to, td);
    if let Some(target) = at {
        if !MeasureEngine::axis_in_measurement_window_with_minval(target, lower, upper, minval) {
            return ContinuousMeasureResult::failed(
                name,
                "AT point is outside the measurement window",
            );
        }
        return match find_at_accepted_points(axis, signal, target, minval, segment_starts) {
            Ok(Some(value)) => ContinuousMeasureResult::success(
                name,
                vec![ContinuousMeasureRecord::point(value, target)],
            ),
            Ok(None) => ContinuousMeasureResult::failed(name, "Time point not in simulation range"),
            Err(error) => ContinuousMeasureResult::failed(name, error),
        };
    }
    let Some(condition) = when else {
        return ContinuousMeasureResult::failed(name, "FIND requires AT= or WHEN condition");
    };
    match continuous_condition_events(
        condition,
        minval,
        axis,
        signals,
        lower,
        upper,
        segment_starts,
    ) {
        Ok(events) if !events.is_empty() => ContinuousMeasureResult::success(
            name,
            events
                .into_iter()
                .map(|(segment, fraction, event_axis, current_within_minval)| {
                    let value = if current_within_minval {
                        signal[segment + 1]
                    } else {
                        interpolate_extended_real(signal[segment], signal[segment + 1], fraction)
                    };
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
    analysis: &str,
    signal_name: &str,
    at: Option<Value>,
    when: Option<&WhenCondition>,
    from: Option<Value>,
    to: Option<Value>,
    td: Option<Value>,
    minval: Value,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    segment_starts: &[usize],
) -> ContinuousMeasureResult {
    let Some(signal) = lookup_signal(signals, signal_name) else {
        return ContinuousMeasureResult::failed(name, format!("Signal '{signal_name}' not found"));
    };
    let (lower, upper) =
        MeasureEngine::point_measurement_window_bounds(axis, analysis, from, to, td);
    let make_record = |segment: usize, event_axis: Value| {
        let value = accepted_row_secant_slope(
            axis[segment],
            signal[segment],
            axis[segment + 1],
            signal[segment + 1],
        );
        ContinuousMeasureRecord::point(value, event_axis)
    };
    if let Some(target) = at {
        if !MeasureEngine::axis_in_measurement_window_with_minval(target, lower, upper, minval) {
            return ContinuousMeasureResult::failed(
                name,
                "AT point is outside the measurement window",
            );
        }
        return match derivative_at_accepted_points(axis, signal, target, minval, segment_starts) {
            Ok(Some(slope)) => ContinuousMeasureResult::success(
                name,
                vec![ContinuousMeasureRecord::point(slope, target)],
            ),
            Ok(None) => ContinuousMeasureResult::failed(name, "Time point not in simulation range"),
            Err(error) => ContinuousMeasureResult::failed(name, error),
        };
    }
    let Some(condition) = when else {
        return ContinuousMeasureResult::failed(
            name,
            "DERIV requires AT=time or WHEN signal=value",
        );
    };
    match continuous_condition_events(
        condition,
        minval,
        axis,
        signals,
        lower,
        upper,
        segment_starts,
    ) {
        Ok(events) if !events.is_empty() => {
            let records = events
                .into_iter()
                .map(|(segment, _, event_axis, _)| make_record(segment, event_axis))
                .collect::<Vec<_>>();
            ContinuousMeasureResult::success(name, records)
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
    minval: Value,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    segment_starts: &[usize],
) -> Result<Vec<Value>, String> {
    match &clause.event {
        // Xyce treats AT as an exact clause result. TD gates conditional
        // events but does not override a valid explicit AT location.
        TriggerEvent::At(target) => Ok((target.is_finite()
            && delay_at_is_reached(axis, *target, segment_starts, minval))
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
            let mut tracker = DelayConditionTracker::new_continuous(
                condition.occurrence.edge,
                condition.occurrence.number,
                clause.occurrence_explicit,
                minval,
            );
            let mut events = Vec::new();
            for row in 0..axis.len() {
                if segment_starts.binary_search(&row).is_ok() {
                    tracker.reset_segment();
                }
                let Some(right_value) = right.value_at(row) else {
                    return Ok(Vec::new());
                };
                if let Some(event_axis) =
                    tracker.update_with_td(axis[row], left[row], right_value, effective_td)
                {
                    events.push(event_axis);
                }
            }
            Ok(events)
        }
    }
}

fn continuous_delay(
    name: &str,
    trig: &TrigSpec,
    targ: &TrigSpec,
    minval: Value,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    segment_starts: &[usize],
) -> ContinuousMeasureResult {
    let target_td = targ.td.or(trig.td);
    let triggers = match continuous_delay_clause_events(
        trig,
        trig.td,
        minval,
        axis,
        signals,
        segment_starts,
    ) {
        Ok(events) => events,
        Err(error) => return ContinuousMeasureResult::failed(name, error),
    };
    let targets = match continuous_delay_clause_events(
        targ,
        target_td,
        minval,
        axis,
        signals,
        segment_starts,
    ) {
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

type MeasureConditionCandidate = (usize, Value, Value, bool, MeasureConditionDirection);

fn select_measure_condition_occurrence(
    events: impl Iterator<Item = MeasureConditionCandidate>,
    edge: EdgeType,
    number: isize,
) -> Option<MeasureEvent> {
    if number < 0 {
        return select_measure_occurrence(
            events
                .filter(|event| edge_matches_measure_condition(edge, event.4))
                .map(|event| (event.0, event.1, event.2, event.3)),
            number,
        );
    }
    let requested = number.max(1) as usize;
    let mut count = 0usize;
    for event in events {
        count += match edge {
            EdgeType::Cross => 1,
            EdgeType::Rise => usize::from(event.4 == MeasureConditionDirection::Rise),
            // updateRFCcountForWhen increments FALL through an `else` when
            // the current sample is not greater than the prior sample. This
            // includes an indeterminate NaN comparison, even though the same
            // candidate cannot satisfy withinRFCWindowForWhen's strict `<`.
            EdgeType::Fall => usize::from(event.4 != MeasureConditionDirection::Rise),
        };
        if count >= requested && edge_matches_measure_condition(edge, event.4) {
            return Some((event.0, event.1, event.2, event.3));
        }
    }
    None
}

fn select_continuous_measure_condition_occurrences(
    events: impl Iterator<Item = MeasureConditionCandidate>,
    edge: EdgeType,
    number: isize,
) -> Vec<MeasureEvent> {
    if number < 0 {
        return select_measure_condition_occurrence(events, edge, number)
            .into_iter()
            .collect();
    }
    let requested = number.max(1) as usize;
    let mut count = 0usize;
    let mut selected = Vec::new();
    for event in events {
        count += match edge {
            EdgeType::Cross => 1,
            EdgeType::Rise => usize::from(event.4 == MeasureConditionDirection::Rise),
            EdgeType::Fall => usize::from(event.4 != MeasureConditionDirection::Rise),
        };
        if count >= requested && edge_matches_measure_condition(edge, event.4) {
            selected.push((event.0, event.1, event.2, event.3));
        }
    }
    selected
}

/// Return every qualifying `WHEN left=right` crossing interval in traversal
/// order. Xyce requires the left operand itself to change over the interval;
/// a moving right operand cannot trigger against a constant left operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MeasureConditionDirection {
    Rise,
    Fall,
    Indeterminate,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct MeasureConditionCrossing {
    pub(crate) fraction: Value,
    pub(crate) current_within_minval: bool,
    pub(crate) direction: MeasureConditionDirection,
}

fn measurement_condition_candidates(
    left: &[Value],
    right: ResolvedMeasureOperand<'_>,
    point_count: usize,
    segment_starts: &[usize],
    minval: Value,
) -> Vec<(usize, MeasureConditionCrossing)> {
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
        let Some(right_previous) = right.value_at(segment) else {
            return Vec::new();
        };
        let Some(right_current) = right.value_at(segment + 1) else {
            return Vec::new();
        };
        if let Some(crossing) = measure_condition_crossing(
            left_previous,
            left_current,
            right_previous,
            right_current,
            minval,
        ) {
            crossings.push((segment, crossing));
        }
    }
    crossings
}

#[cfg(test)]
fn measurement_condition_crossings(
    left: &[Value],
    right: ResolvedMeasureOperand<'_>,
    point_count: usize,
    segment_starts: &[usize],
    edge: EdgeType,
    minval: Value,
) -> Vec<(usize, Value)> {
    measurement_condition_candidates(left, right, point_count, segment_starts, minval)
        .into_iter()
        .filter(|(_, crossing)| edge_matches_measure_condition(edge, crossing.direction))
        .map(|(segment, crossing)| (segment, crossing.fraction))
        .collect()
}

/// Compute the legacy percentage-threshold rise/fall duration over a complete
/// accepted-point waveform. The live runtime deliberately calls this only at
/// the final point because both thresholds depend on the waveform's global
/// extrema.
pub(crate) fn rise_fall_duration(
    axis: &[Value],
    signal: &[Value],
    from_pct: Value,
    to_pct: Value,
    number: usize,
    is_rise: bool,
) -> Option<Value> {
    let minimum = signal.iter().copied().fold(Value::INFINITY, Value::min);
    let maximum = signal.iter().copied().fold(Value::NEG_INFINITY, Value::max);
    let range = maximum - minimum;
    let (lower_threshold, upper_threshold) = if is_rise {
        (minimum + from_pct * range, minimum + to_pct * range)
    } else {
        (minimum + to_pct * range, minimum + from_pct * range)
    };
    let edge = if is_rise {
        EdgeType::Rise
    } else {
        EdgeType::Fall
    };
    let crossing = |threshold: Value| {
        let mut count = 0usize;
        for (segment, samples) in signal.windows(2).enumerate() {
            let previous = samples[0];
            let current = samples[1];
            let crossed = match edge {
                EdgeType::Rise => previous < threshold && current >= threshold,
                EdgeType::Fall => previous > threshold && current <= threshold,
                EdgeType::Cross => unreachable!("rise/fall duration selects a directional edge"),
            };
            if !crossed {
                continue;
            }
            count += 1;
            if count == number {
                let fraction = (threshold - previous) / (current - previous);
                return Some(axis[segment] + fraction * (axis[segment + 1] - axis[segment]));
            }
        }
        None
    };
    crossing(lower_threshold)
        .zip(crossing(upper_threshold))
        .map(|(lower, upper)| (upper - lower).abs())
}

pub(crate) fn measure_condition_crossing(
    left_previous: Value,
    left_current: Value,
    right_previous: Value,
    right_current: Value,
    minval: Value,
) -> Option<MeasureConditionCrossing> {
    if !minval.is_finite() || minval < 0.0 || left_current == left_previous {
        return None;
    }
    let previous_difference = left_previous - right_previous;
    let current_difference = left_current - right_current;
    let previous_equal = previous_difference.abs() < minval;
    let current_equal = current_difference.abs() < minval;
    // Entering Xyce's MINVAL equality band is the crossing. Normalize that
    // state before the next strict-sign test so leaving the band cannot emit
    // the same physical root a second time.
    let strict_crossing = !previous_equal
        && ((previous_difference < 0.0 && current_difference > 0.0)
            || (previous_difference > 0.0 && current_difference < 0.0));
    if !current_equal && !strict_crossing {
        return None;
    }
    let direction = if left_current > left_previous {
        MeasureConditionDirection::Rise
    } else if left_current < left_previous {
        MeasureConditionDirection::Fall
    } else {
        // IEEE comparisons with a NaN endpoint are both false. Xyce still
        // accepts current-point MINVAL equality for default CROSS, increments
        // its fall counter through the updateRFC `else`, but does not select
        // the event for an authored RISE or FALL qualifier.
        MeasureConditionDirection::Indeterminate
    };

    let denominator = current_difference - previous_difference;
    let fraction = if denominator == 0.0 {
        if previous_difference == 0.0 && current_difference == 0.0 {
            // Parallel, identical moving operands are equal at the current
            // accepted point in Xyce.
            1.0
        } else {
            -previous_difference / denominator
        }
    } else {
        -previous_difference / denominator
    };
    // Xyce preserves the raw interpolated instant. In particular, a prior
    // NaN followed by current-point MINVAL equality produces a NaN instant
    // that passes its negative-form window checks and participates in RFC
    // occurrence counting.
    Some(MeasureConditionCrossing {
        fraction,
        current_within_minval: current_equal,
        direction,
    })
}

fn edge_matches_measure_condition(edge: EdgeType, direction: MeasureConditionDirection) -> bool {
    match edge {
        EdgeType::Rise => direction == MeasureConditionDirection::Rise,
        EdgeType::Fall => direction == MeasureConditionDirection::Fall,
        EdgeType::Cross => true,
    }
}

fn measurement_segment_slope(
    name: &str,
    axis: &[Value],
    signal: &[Value],
    segment: usize,
) -> MeasureResult {
    MeasureResult::success(
        name,
        accepted_row_secant_slope(
            axis[segment],
            signal[segment],
            axis[segment + 1],
            signal[segment + 1],
        ),
    )
}

pub(crate) fn accepted_row_secant_slope(
    previous_axis: Value,
    previous_signal: Value,
    current_axis: Value,
    current_signal: Value,
) -> Value {
    (current_signal - previous_signal) / (current_axis - previous_axis)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum AcceptedRowAtMatch {
    Current,
    PreviousSegment { fraction: Value },
}

/// Classify one accepted row using Xyce's shared `AT=` rule.
///
/// The current endpoint has priority because FIND snaps to its exact dependent
/// value while DERIV uses the accepted row's backward secant. Otherwise, a
/// strict bracket or a previous endpoint inside the authored `MINVAL` band
/// qualifies the row. Passing `None` at a DC rollover prevents a cross-segment
/// bracket while still permitting a current-endpoint match.
pub(crate) fn accepted_row_at_match(
    previous_axis: Option<Value>,
    current_axis: Value,
    target: Value,
    minval: Value,
) -> Option<AcceptedRowAtMatch> {
    if !target.is_finite() || !minval.is_finite() || minval < 0.0 {
        return None;
    }
    if !current_axis.is_finite() {
        return None;
    }
    let current_difference = current_axis - target;
    if current_difference.abs() < minval {
        return Some(AcceptedRowAtMatch::Current);
    }

    let previous_axis = previous_axis?;
    if !previous_axis.is_finite() {
        return None;
    }
    let previous_difference = previous_axis - target;
    let strictly_bracketed = (previous_difference < 0.0 && current_difference > 0.0)
        || (previous_difference > 0.0 && current_difference < 0.0);
    if !strictly_bracketed && previous_difference.abs() >= minval {
        return None;
    }

    let width = current_axis - previous_axis;
    if width == 0.0 || !width.is_finite() {
        return None;
    }
    let fraction = (target - previous_axis) / width;
    if !fraction.is_finite() {
        return None;
    }
    Some(AcceptedRowAtMatch::PreviousSegment { fraction })
}

fn find_at_accepted_row(
    previous: Option<(Value, Value)>,
    current: (Value, Value),
    target: Value,
    minval: Value,
) -> Result<Option<Value>, &'static str> {
    let Some(relation) =
        accepted_row_at_match(previous.map(|(axis, _)| axis), current.0, target, minval)
    else {
        return Ok(None);
    };
    let value = match relation {
        AcceptedRowAtMatch::Current => current.1,
        AcceptedRowAtMatch::PreviousSegment { fraction } => {
            let Some((_, previous_signal)) = previous else {
                return Ok(None);
            };
            interpolate_extended_real(previous_signal, current.1, fraction)
        }
    };
    Ok(Some(value))
}

fn find_at_accepted_points(
    axis: &[Value],
    signal: &[Value],
    target: Value,
    minval: Value,
    segment_starts: &[usize],
) -> Result<Option<Value>, &'static str> {
    if axis.len() != signal.len() || axis.is_empty() {
        return Ok(None);
    }
    for row in 0..axis.len() {
        let starts_segment = row == 0 || segment_starts.binary_search(&row).is_ok();
        let previous = (!starts_segment).then(|| (axis[row - 1], signal[row - 1]));
        if let Some(value) =
            find_at_accepted_row(previous, (axis[row], signal[row]), target, minval)?
        {
            return Ok(Some(value));
        }
    }
    Ok(None)
}

fn derivative_at_accepted_points(
    axis: &[Value],
    signal: &[Value],
    target: Value,
    minval: Value,
    segment_starts: &[usize],
) -> Result<Option<Value>, &'static str> {
    if axis.len() != signal.len() || axis.len() < 2 {
        return Ok(None);
    }
    // Xyce initializes state at the first global row but does not evaluate a
    // DERIV AT there. Later DC rollovers remain eligible after resetting the
    // previous state to the current accepted row, which correctly classifies
    // their self-secants as undefined.
    for row in 1..axis.len() {
        let starts_segment = segment_starts.binary_search(&row).is_ok();
        let previous_axis = (!starts_segment).then_some(axis[row - 1]);
        if accepted_row_at_match(previous_axis, axis[row], target, minval).is_none() {
            continue;
        }
        let previous_row = if starts_segment { row } else { row - 1 };
        return Ok(Some(accepted_row_secant_slope(
            axis[previous_row],
            signal[previous_row],
            axis[row],
            signal[row],
        )));
    }
    Ok(None)
}

/// Interpolate values in the extended-real domain used by direct measurement
/// operands. Finite endpoints retain the conventional linear formula. At an
/// interior point, a single infinite endpoint dominates and equal signed
/// infinities remain constant; opposite infinities are undefined and yield
/// NaN, which is retained as a computed undefined numeric result.
pub(crate) fn interpolate_extended_real(left: Value, right: Value, fraction: Value) -> Value {
    if fraction == 0.0 {
        return left;
    }
    if fraction == 1.0 {
        return right;
    }
    if left == right {
        return left;
    }
    if !(0.0..1.0).contains(&fraction) {
        // MINVAL endpoint handling can intentionally extrapolate a crossing.
        // Preserve the canonical affine arithmetic there; the extended-real
        // dominance rules below apply only to true interpolation.
        return left + fraction * (right - left);
    }
    if left.is_finite() && right.is_finite() {
        return left + fraction * (right - left);
    }
    match (left.is_infinite(), right.is_infinite()) {
        (true, false) => left,
        (false, true) => right,
        _ => Value::NAN,
    }
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

    fn find_at_statement(
        name: &str,
        analysis: &str,
        target: Value,
        minval: Value,
    ) -> MeasureStatement {
        MeasureStatement {
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: name.to_string(),
            measure_type: MeasureType::Find {
                signal: "Y".to_string(),
                at: Some(target),
                when: None,
                from: None,
                to: None,
                td: None,
                minval,
            },
            analysis: analysis.to_string(),
            goal: None,
            tolerance: None,
        }
    }

    fn derivative_at_statement(
        name: &str,
        analysis: &str,
        target: Value,
        minval: Value,
    ) -> MeasureStatement {
        MeasureStatement {
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: name.to_string(),
            measure_type: MeasureType::Derivative {
                signal: "Y".to_string(),
                at: Some(target),
                when: None,
                from: None,
                to: None,
                td: None,
                minval,
            },
            analysis: analysis.to_string(),
            goal: None,
            tolerance: None,
        }
    }

    #[test]
    fn extended_real_interpolation_preserves_endpoints_and_infinities() {
        assert_eq!(interpolate_extended_real(3.0, 5.0, 0.5), 4.0);
        assert_eq!(
            interpolate_extended_real(Value::INFINITY, Value::NEG_INFINITY, 0.0),
            Value::INFINITY
        );
        assert_eq!(
            interpolate_extended_real(Value::INFINITY, Value::NEG_INFINITY, 1.0),
            Value::NEG_INFINITY
        );
        assert_eq!(
            interpolate_extended_real(Value::NEG_INFINITY, Value::NEG_INFINITY, 0.5),
            Value::NEG_INFINITY
        );
        assert_eq!(
            interpolate_extended_real(Value::INFINITY, 0.0, 0.5),
            Value::INFINITY
        );
        assert!(interpolate_extended_real(Value::INFINITY, Value::NEG_INFINITY, 0.5).is_nan());
        assert!(interpolate_extended_real(Value::INFINITY, 0.0, 1.5).is_nan());
    }

    #[test]
    fn find_at_accepted_rows_follow_xyce_minval_and_segment_rules() {
        let default_minval = XYCE_DEFAULT_MEASURE_MINVAL;

        assert_eq!(
            find_at_accepted_row(None, (0.0, 2.0), 5.0e-13, default_minval).unwrap(),
            Some(2.0),
            "the default equality band returns the first accepted value"
        );
        assert_eq!(
            find_at_accepted_row(None, (0.0, 2.0), 0.25, 0.5).unwrap(),
            Some(2.0),
            "an authored large MINVAL returns the current value early"
        );
        assert_eq!(
            find_at_accepted_row(None, (0.0, 0.0), 5.0e-13, 1.0e-14).unwrap(),
            None
        );
        assert_eq!(
            find_at_accepted_row(Some((0.0, 0.0)), (1.0, 1.0e12), 5.0e-13, 1.0e-14,).unwrap(),
            Some(0.5),
            "a smaller MINVAL waits for the steep segment interpolation"
        );
        assert_eq!(
            find_at_accepted_row(Some((0.0, 0.0)), (1.0, 10.0), 1.0 - 5.0e-13, default_minval,)
                .unwrap(),
            Some(10.0),
            "a near-current endpoint returns the exact current signal"
        );
        assert_eq!(
            find_at_accepted_row(Some((0.0, 2.0)), (1.0, 3.0), 1.0, 0.0).unwrap(),
            None,
            "strict MINVAL comparison makes zero reject even exact endpoints"
        );

        let extrapolated = find_at_accepted_row(Some((1.0, 4.0)), (2.0, 10.0), 0.995, 0.01)
            .expect("the extrapolation is defined")
            .expect("a prior point inside MINVAL qualifies the accepted row");
        assert!((extrapolated - 3.97).abs() < 1.0e-12);
        assert_eq!(
            find_at_accepted_row(Some((1.0, 4.0)), (2.0, 10.0), 0.98, 0.01).unwrap(),
            None,
            "a target beyond the prior equality band and segment range fails"
        );

        let nested_axis = [0.0, 1.0, 2.0, 3.0];
        let nested_signal = [10.0, 11.0, 20.0, 21.0];
        assert_eq!(
            find_at_accepted_points(&nested_axis, &nested_signal, 2.0, default_minval, &[2],)
                .unwrap(),
            Some(20.0)
        );
        assert_eq!(
            find_at_accepted_points(
                &nested_axis,
                &nested_signal,
                2.0 + 5.0e-13,
                default_minval,
                &[2],
            )
            .unwrap(),
            Some(20.0),
            "a near target at a later segment start has no prior row"
        );
        assert_eq!(
            find_at_accepted_points(&nested_axis, &nested_signal, 1.5, default_minval, &[2],)
                .unwrap(),
            None,
            "the apparent bracket across a DC segment barrier is invalid"
        );

        let undefined = find_at_accepted_points(
            &[0.0, 1.0, 0.5],
            &[Value::INFINITY, Value::NEG_INFINITY, 7.0],
            0.5,
            default_minval,
            &[],
        )
        .expect("the first accepted interpolation is a computed numeric result")
        .expect("the accepted point is found");
        assert!(undefined.is_nan());
    }

    #[test]
    fn scalar_find_and_when_results_retain_their_event_axes() {
        let axis = [0.0, 1.0];
        let dependent = [0.0, 10.0];
        let condition = [-1.0, 1.0];
        let signals = HashMap::from([
            ("Y".to_string(), dependent.as_slice()),
            ("CONDITION".to_string(), condition.as_slice()),
        ]);
        let when = WhenCondition {
            left: "CONDITION".to_string(),
            right: MeasureOperand::Constant(0.0),
            occurrence: EventOccurrence::default(),
        };
        let mut engine = MeasureEngine::new();
        engine.add(find_at_statement(
            "find_at",
            "TRAN",
            0.25,
            XYCE_DEFAULT_MEASURE_MINVAL,
        ));
        engine.add(find_at_statement(
            "find_at_outside",
            "TRAN",
            2.0,
            XYCE_DEFAULT_MEASURE_MINVAL,
        ));
        engine.add(MeasureStatement {
            name: "find_when".to_string(),
            measure_type: MeasureType::Find {
                signal: "Y".to_string(),
                at: None,
                when: Some(when.clone()),
                from: None,
                to: None,
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
            analysis: "TRAN".to_string(),
            goal: None,
            tolerance: None,
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
        });
        engine.add(MeasureStatement {
            name: "when".to_string(),
            measure_type: MeasureType::When {
                condition: when,
                from: None,
                to: None,
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
            analysis: "TRAN".to_string(),
            goal: None,
            tolerance: None,
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
        });

        let results = engine.evaluate(&axis, &signals);
        assert_eq!(results[0].value, Some(2.5));
        assert_eq!(results[0].event_axis, Some(0.25));
        assert!(!results[1].passed);
        assert_eq!(results[1].event_axis, Some(2.0));
        assert_eq!(results[2].value, Some(5.0));
        assert_eq!(results[2].event_axis, Some(0.5));
        assert_eq!(results[3].value, Some(0.5));
        assert_eq!(results[3].event_axis, Some(0.5));
        assert!(results[0].passed && results[2].passed && results[3].passed);
    }

    #[test]
    fn find_at_accepted_rows_reject_invalid_programmatic_limits() {
        let previous = Some((0.0, 1.0));
        let current = (1.0, 2.0);
        assert_eq!(
            find_at_accepted_row(previous, current, 0.5, -1.0).unwrap(),
            None
        );
        assert_eq!(
            find_at_accepted_row(previous, current, 0.5, Value::NAN).unwrap(),
            None
        );
        assert_eq!(
            find_at_accepted_row(previous, current, 0.5, Value::INFINITY).unwrap(),
            None
        );
        assert_eq!(
            find_at_accepted_row(previous, current, Value::NAN, 1.0).unwrap(),
            None
        );
        assert_eq!(
            find_at_accepted_row(previous, current, Value::INFINITY, 1.0).unwrap(),
            None
        );

        let axis = [0.0, 1.0];
        let values = [0.0, 1.0];
        let mut signals = HashMap::new();
        signals.insert("Y".to_string(), values.as_slice());
        for minval in [-1.0, Value::NAN, Value::INFINITY] {
            let scalar = engine_with(find_at_statement("invalid", "TRAN", 0.5, minval))
                .evaluate(&axis, &signals);
            assert!(!scalar[0].passed);

            let continuous = engine_with(find_at_statement(
                "invalid_continuous",
                "TRAN_CONT",
                0.5,
                minval,
            ))
            .evaluate_continuous(&axis, &signals, &[]);
            assert!(continuous[0].failure.is_some());
            assert!(continuous[0].records.is_empty());

            let derivative = engine_with(derivative_at_statement(
                "invalid_derivative",
                "TRAN",
                0.5,
                minval,
            ))
            .evaluate(&axis, &signals);
            assert!(!derivative[0].passed);

            let continuous_derivative = engine_with(derivative_at_statement(
                "invalid_continuous_derivative",
                "TRAN_CONT",
                0.5,
                minval,
            ))
            .evaluate_continuous(&axis, &signals, &[]);
            assert!(continuous_derivative[0].failure.is_some());
            assert!(continuous_derivative[0].records.is_empty());
        }
    }

    #[test]
    fn scalar_and_continuous_find_at_keep_accepted_row_order() {
        let axis = [0.0, 2.0, 1.0];
        let values = [0.0, 20.0, 100.0];
        let mut signals = HashMap::new();
        signals.insert("Y".to_string(), values.as_slice());

        let scalar = engine_with(find_at_statement(
            "scalar",
            "TRAN",
            1.0,
            XYCE_DEFAULT_MEASURE_MINVAL,
        ))
        .evaluate(&axis, &signals);
        assert_eq!(scalar[0].value, Some(10.0));

        let continuous = engine_with(find_at_statement(
            "continuous",
            "TRAN_CONT",
            1.0,
            XYCE_DEFAULT_MEASURE_MINVAL,
        ))
        .evaluate_continuous(&axis, &signals, &[]);
        assert_eq!(continuous[0].records.len(), 1);
        assert_eq!(continuous[0].records[0].value, 10.0);
        assert_eq!(continuous[0].records[0].event_axis, Some(1.0));
    }

    #[test]
    fn derivative_at_uses_the_first_accepted_rows_backward_secant() {
        let axis = [0.0, 1.0, 2.0];
        let values = [0.0, 10.0, 40.0];
        let mut signals = HashMap::new();
        signals.insert("Y".to_string(), values.as_slice());

        let scalar = engine_with(derivative_at_statement("slope", "TRAN", 1.05, 0.1))
            .evaluate(&axis, &signals);
        assert_eq!(scalar[0].value, Some(10.0));

        let continuous = engine_with(derivative_at_statement(
            "slope_continuous",
            "TRAN_CONT",
            1.05,
            0.1,
        ))
        .evaluate_continuous(&axis, &signals, &[]);
        assert_eq!(continuous[0].records[0].value, 10.0);

        let nonmonotonic_axis = [0.0, 2.0, 1.0];
        let nonmonotonic_values = [0.0, 20.0, 100.0];
        signals.insert("Y".to_string(), nonmonotonic_values.as_slice());
        let first_bracket = engine_with(derivative_at_statement(
            "first_bracket",
            "TRAN",
            1.0,
            XYCE_DEFAULT_MEASURE_MINVAL,
        ))
        .evaluate(&nonmonotonic_axis, &signals);
        assert_eq!(first_bracket[0].value, Some(10.0));

        let exact_zero_band = engine_with(derivative_at_statement("zero_band", "TRAN", 1.0, 0.0))
            .evaluate(&axis, &signals);
        assert!(!exact_zero_band[0].passed);
    }

    #[test]
    fn find_and_derivative_at_classify_singletons_barriers_and_zero_widths() {
        let singleton_axis = [0.0];
        let singleton_values = [3.0];
        let mut singleton_signals = HashMap::new();
        singleton_signals.insert("Y".to_string(), singleton_values.as_slice());
        let found = engine_with(find_at_statement(
            "singleton_find",
            "TRAN",
            0.0,
            XYCE_DEFAULT_MEASURE_MINVAL,
        ))
        .evaluate(&singleton_axis, &singleton_signals);
        assert_eq!(found[0].value, Some(3.0));
        let derivative = engine_with(derivative_at_statement(
            "singleton_derivative",
            "TRAN",
            0.0,
            XYCE_DEFAULT_MEASURE_MINVAL,
        ))
        .evaluate(&singleton_axis, &singleton_signals);
        assert!(!derivative[0].passed);

        let nested_axis = [0.0, 1.0, 2.0, 3.0];
        let nested_values = [10.0, 11.0, 20.0, 21.0];
        let mut nested_signals = HashMap::new();
        nested_signals.insert("Y".to_string(), nested_values.as_slice());
        let nested_engine = engine_with(derivative_at_statement(
            "segment_start_derivative",
            "DC",
            2.0,
            XYCE_DEFAULT_MEASURE_MINVAL,
        ));
        let nested =
            nested_engine.evaluate_with_segment_starts(&nested_axis, &nested_signals, &[2]);
        assert!(nested[0].passed, "the rollover self-secant is computed 0/0");
        assert!(nested[0].value.is_some_and(Value::is_nan));

        let duplicate_axis = [0.0, 0.0];
        let changing_values = [0.0, 1.0];
        let constant_values = [1.0, 1.0];
        let mut duplicate_signals = HashMap::new();
        duplicate_signals.insert("Y".to_string(), changing_values.as_slice());
        let infinite = engine_with(derivative_at_statement(
            "infinite",
            "TRAN",
            0.0,
            XYCE_DEFAULT_MEASURE_MINVAL,
        ))
        .evaluate(&duplicate_axis, &duplicate_signals);
        assert_eq!(infinite[0].value, Some(Value::INFINITY));
        duplicate_signals.insert("Y".to_string(), constant_values.as_slice());
        let undefined = engine_with(derivative_at_statement(
            "undefined",
            "TRAN",
            0.0,
            XYCE_DEFAULT_MEASURE_MINVAL,
        ))
        .evaluate(&duplicate_axis, &duplicate_signals);
        assert!(undefined[0].passed);
        assert!(undefined[0].value.is_some_and(Value::is_nan));
    }

    #[test]
    fn integration_over_one_selected_sample_is_zero() {
        let statement = MeasureStatement {
            default_value: None,
            fail_value: None,
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
            fail_value: None,
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
            fail_value: None,
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
            fail_value: None,
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
            fail_value: None,
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
            fail_value: None,
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
            fail_value: None,
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
            fail_value: None,
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
    fn nan_in_an_unreferenced_sample_does_not_fail_other_measurement_state() {
        let time = [0.0, 1.0, 2.0];
        let data = [0.0, f64::NAN, 2.0];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &data);

        let results = engine_with(max_statement("V(out)")).evaluate(&time, &signals);

        assert!(results[0].passed);
        assert_eq!(results[0].value, Some(2.0));
        assert_eq!(results[0].error, None);
    }

    #[test]
    fn infinite_extended_real_measurement_result_is_valid() {
        let time = [0.0, 1.0, 2.0];
        let data = [Value::NEG_INFINITY; 3];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("VDB(0)".to_string(), &data);

        let result = engine_with(max_statement("VDB(0)"))
            .evaluate(&time, &signals)
            .remove(0);

        assert!(result.passed, "{result:?}");
        assert_eq!(result.value, Some(Value::NEG_INFINITY));
        assert_eq!(result.event_axis, Some(0.0));
    }

    #[test]
    fn param_measure_preserves_ngspice_complex_rejection_and_xyce_real_projection() {
        let statement = MeasureStatement {
            goal: None,
            tolerance: None,
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "bad_param".to_string(),
            measure_type: MeasureType::Param {
                expression: MeasureExpression::expression("sqrt(-1)"),
            },
            analysis: "TRAN".to_string(),
        };
        let time = [0.0, 1.0];
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        signals.insert("V(out)".to_string(), &[0.0, 1.0]);

        let engine = engine_with(statement);
        let ngspice = engine.evaluate(&time, &signals);
        assert!(!ngspice[0].passed, "{:?}", ngspice[0]);
        assert_eq!(ngspice[0].value, None);
        assert!(
            ngspice[0]
                .error
                .as_deref()
                .is_some_and(|error| error.contains("complex value"))
        );

        let mut params = crate::netlist::ParamContext::new();
        params.set_expression_dialect(crate::config::ExpressionDialect::Xyce);
        let xyce = engine.evaluate_with_segment_starts_and_context(&time, &signals, &[], &params);
        assert!(xyce[0].passed, "{:?}", xyce[0]);
        assert_eq!(xyce[0].value, Some(0.0));
        assert_eq!(xyce[0].error, None);
    }

    #[test]
    fn generic_param_measure_distinguishes_authored_and_raw_extended_real_results() {
        let mut engine = MeasureEngine::new();
        engine.add(max_statement("VDB(0)"));
        engine.add(MeasureStatement {
            goal: None,
            tolerance: None,
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "derived".to_string(),
            measure_type: MeasureType::Param {
                expression: MeasureExpression::expression("peak"),
            },
            analysis: "TRAN".to_string(),
        });
        engine.add(MeasureStatement {
            goal: None,
            tolerance: None,
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "raw_derived".to_string(),
            measure_type: MeasureType::Param {
                expression: MeasureExpression::raw_reference("peak"),
            },
            analysis: "TRAN".to_string(),
        });
        let axis = [0.0, 1.0];
        let ground_db = [Value::NEG_INFINITY; 2];
        let signals = HashMap::from([("VDB(0)".to_string(), ground_db.as_slice())]);
        let mut params = crate::netlist::ParamContext::new();
        params.set_expression_dialect(crate::config::ExpressionDialect::Xyce);

        let results =
            engine.evaluate_with_segment_starts_and_context(&axis, &signals, &[], &params);

        assert_eq!(results[0].value, Some(Value::NEG_INFINITY));
        assert_eq!(results[1].value, Some(-1.0e50));
        assert_eq!(results[2].value, Some(Value::NEG_INFINITY));
        assert!(results.iter().all(|result| result.passed), "{results:?}");
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
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
            analysis: "DC".to_string(),
            goal: None,
            tolerance: None,
            default_value: None,
            fail_value: None,
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
    fn derivative_when_retains_found_event_with_nan_secant() {
        let axis = [0.0, 1.0];
        let source = [Value::INFINITY, Value::INFINITY];
        let condition = [-1.0, 1.0];
        let signals = HashMap::from([
            ("Y".to_string(), source.as_slice()),
            ("COND".to_string(), condition.as_slice()),
        ]);
        let statement = derivative_statement(
            "undefined_slope",
            "Y",
            None,
            Some(WhenCondition {
                left: "COND".to_string(),
                right: MeasureOperand::Constant(0.0),
                occurrence: EventOccurrence {
                    edge: EdgeType::Cross,
                    number: 1,
                },
            }),
            None,
            None,
        );

        let result = engine_with(statement).evaluate(&axis, &signals).remove(0);
        assert!(result.passed, "{result:?}");
        assert!(result.value.is_some_and(Value::is_nan));
        assert_eq!(result.event_axis, Some(0.5));
    }

    #[test]
    fn derivative_when_intersects_moving_operands_and_filters_each_crossing() {
        let axis = [1.0, 2.0, 3.0, 4.0, 5.0];
        let squares = [1.0, 4.0, 9.0, 16.0, 25.0];
        let four_x = [4.0, 8.0, 12.0, 16.0, 20.0];
        let double_crossing = [4.2, 1.1, 0.0, 0.9, 3.8];
        // Xyce tests exact dependent-variable equality before testing whether
        // the current row is inside MINVAL. A changed row this close to the
        // target is therefore a real endpoint event, even at roundoff scale.
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
            }),
            None,
            None,
        ));

        let results = engine.evaluate(&axis, &signals);
        assert_eq!(results[0].value, Some(7.0));
        assert_eq!(results[1].value, Some(0.9));
        assert_eq!(results[0].event_axis, Some(4.0));
        assert_eq!(results[1].event_axis, Some(3.5555555555555554));
        assert_eq!(results[2].value, Some(3.0));
        assert_eq!(results[2].event_axis, Some(1.0));
    }

    #[test]
    fn when_returns_interpolated_axis_and_honors_inclusive_windows() {
        let axis = [5.0, 4.0, 3.0, 2.0, 1.0];
        let condition = [5.0, 4.0, 3.0, 2.0, 1.0];
        let mut signals = HashMap::new();
        signals.insert("CONDITION".to_string(), condition.as_slice());
        let statement = |name: &str, from: Option<Value>, to: Option<Value>| MeasureStatement {
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: name.to_string(),
            measure_type: MeasureType::When {
                condition: WhenCondition {
                    left: "CONDITION".to_string(),
                    right: MeasureOperand::Constant(2.5),
                    occurrence: EventOccurrence::default(),
                },
                from,
                to,
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
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
        };
        let mut engine = MeasureEngine::new();
        engine.add(MeasureStatement {
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "event_axis".to_string(),
            measure_type: MeasureType::When {
                condition: when.clone(),
                from: None,
                to: None,
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
            analysis: "DC".to_string(),
            goal: None,
            tolerance: None,
        });
        engine.add(MeasureStatement {
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "found_value".to_string(),
            measure_type: MeasureType::Find {
                signal: "FOUND".to_string(),
                at: None,
                when: Some(when),
                from: None,
                to: None,
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
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
                    fail_value: None,
                    print_policy: MeasurePrintPolicy::All,
                    name: name.to_string(),
                    measure_type: MeasureType::When {
                        condition: WhenCondition {
                            left: "ALT".to_string(),
                            right: MeasureOperand::Constant(0.0),
                            occurrence: EventOccurrence { edge, number },
                        },
                        from,
                        to: None,
                        td: None,
                        minval: XYCE_DEFAULT_MEASURE_MINVAL,
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
    fn transient_td_gates_interpolated_events_before_occurrence_counting() {
        let axis = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let alternating = [-1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
        let mut signals = HashMap::new();
        signals.insert("ALT".to_string(), alternating.as_slice());
        let statement =
            |name: &str, analysis: &str, from: Option<Value>, td: Option<Value>, minval: Value| {
                MeasureStatement {
                    default_value: None,
                    fail_value: None,
                    print_policy: MeasurePrintPolicy::All,
                    name: name.to_string(),
                    measure_type: MeasureType::When {
                        condition: WhenCondition {
                            left: "ALT".to_string(),
                            right: MeasureOperand::Constant(0.0),
                            occurrence: EventOccurrence::default(),
                        },
                        from,
                        to: None,
                        td,
                        minval,
                    },
                    analysis: analysis.to_string(),
                    goal: None,
                    tolerance: None,
                }
            };
        let mut engine = MeasureEngine::new();
        engine.add(statement(
            "td",
            "TRAN",
            None,
            Some(2.0),
            XYCE_DEFAULT_MEASURE_MINVAL,
        ));
        engine.add(statement(
            "from_and_td",
            "TRAN",
            Some(4.0),
            Some(2.0),
            XYCE_DEFAULT_MEASURE_MINVAL,
        ));
        engine.add(statement(
            "default_boundary",
            "TRAN",
            None,
            Some(0.55),
            XYCE_DEFAULT_MEASURE_MINVAL,
        ));
        engine.add(statement("custom_boundary", "TRAN", None, Some(0.55), 0.1));
        engine.add(statement(
            "negative_td",
            "TRAN",
            None,
            Some(-1.0),
            XYCE_DEFAULT_MEASURE_MINVAL,
        ));
        for analysis in ["AC", "DC", "NOISE"] {
            engine.add(statement(
                &format!("ignored_{analysis}"),
                analysis,
                None,
                Some(10.0),
                XYCE_DEFAULT_MEASURE_MINVAL,
            ));
        }

        let results = engine.evaluate(&axis, &signals);
        assert_eq!(results[0].value, Some(2.5));
        assert_eq!(results[1].value, Some(4.5));
        assert_eq!(results[2].value, Some(1.5));
        assert_eq!(results[3].value, Some(0.5));
        assert_eq!(results[4].value, Some(0.5));
        assert!(results[5..].iter().all(|result| result.value == Some(0.5)));
    }

    #[test]
    fn minval_endpoint_equality_uses_xyce_linear_intersection() {
        let nearly_equal = [-1.0, -1.0e-14];
        let events = measurement_condition_crossings(
            &nearly_equal,
            ResolvedMeasureOperand::Constant(0.0),
            nearly_equal.len(),
            &[],
            EdgeType::Cross,
            XYCE_DEFAULT_MEASURE_MINVAL,
        );
        assert_eq!(events.len(), 1);
        assert!(events[0].1 > 1.0);
        assert!((events[0].1 - 1.00000000000001).abs() < 1.0e-15);
    }

    #[test]
    fn minval_endpoint_equality_is_not_recounted_on_band_exit() {
        let enters_then_leaves = [1.0, 1.0e-14, -1.0];
        let events = measurement_condition_crossings(
            &enters_then_leaves,
            ResolvedMeasureOperand::Constant(0.0),
            enters_then_leaves.len(),
            &[],
            EdgeType::Cross,
            XYCE_DEFAULT_MEASURE_MINVAL,
        );
        assert_eq!(events.len(), 1);
        assert!(events[0].1 > 1.0);
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
        let events = measurement_condition_crossings(
            &nearly_equal,
            ResolvedMeasureOperand::Constant(0.0),
            nearly_equal.len(),
            &[],
            EdgeType::Cross,
            1.0e-13,
        );
        assert_eq!(events.len(), 1);
        assert!(events[0].1 > 1.0);
    }

    #[test]
    fn sub_picounit_noise_motion_remains_eligible_for_crossing_detection() {
        let falling_psd = [2.0e-14, 1.2e-14, 8.0e-15];
        let events = measurement_condition_crossings(
            &falling_psd,
            ResolvedMeasureOperand::Constant(1.0e-14),
            falling_psd.len(),
            &[],
            EdgeType::Fall,
            1.0e-16,
        );
        assert_eq!(events, vec![(1, 0.5)]);

        let flat_psd = [1.2e-14, 1.2e-14];
        assert!(
            measurement_condition_crossings(
                &flat_psd,
                ResolvedMeasureOperand::Constant(1.0e-14),
                flat_psd.len(),
                &[],
                EdgeType::Cross,
                1.0e-16,
            )
            .is_empty()
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
        };
        let mut engine = MeasureEngine::new();
        engine.add(MeasureStatement {
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "inherited_td".to_string(),
            measure_type: MeasureType::Delay {
                from: None,
                to: None,
                trig: TrigSpec {
                    event: TriggerEvent::At(4.0),
                    td: Some(5.0),
                    frac_max: None,
                    occurrence_explicit: false,
                },
                targ: TrigSpec {
                    event: TriggerEvent::When(condition(1)),
                    td: None,
                    frac_max: None,
                    occurrence_explicit: true,
                },
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
            analysis: "DC".to_string(),
            goal: None,
            tolerance: None,
        });
        engine.add(MeasureStatement {
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "last_is_signed".to_string(),
            measure_type: MeasureType::Delay {
                from: None,
                to: None,
                trig: TrigSpec {
                    event: TriggerEvent::When(condition(-1)),
                    td: None,
                    frac_max: None,
                    occurrence_explicit: true,
                },
                targ: TrigSpec {
                    event: TriggerEvent::At(1.0),
                    td: None,
                    frac_max: None,
                    occurrence_explicit: false,
                },
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
            analysis: "DC".to_string(),
            goal: None,
            tolerance: None,
        });
        engine.add(MeasureStatement {
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: "second_from_last".to_string(),
            measure_type: MeasureType::Delay {
                from: None,
                to: None,
                trig: TrigSpec {
                    event: TriggerEvent::When(condition(-2)),
                    td: None,
                    frac_max: None,
                    occurrence_explicit: true,
                },
                targ: TrigSpec {
                    event: TriggerEvent::At(1.0),
                    td: None,
                    frac_max: None,
                    occurrence_explicit: false,
                },
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
            analysis: "DC".to_string(),
            goal: None,
            tolerance: None,
        });

        let results = engine.evaluate(&axis, &signals);

        assert_eq!(results[0].value, Some(1.5));
        assert_eq!(results[1].value, Some(-4.5));
        assert_eq!(results[2].value, Some(-3.5));
    }

    #[test]
    fn legacy_frac_max_recomputes_dynamic_levels_and_absolute_rfc_windows() {
        let make_clause = |signal: &str, frac_max, edge, number, explicit| TrigSpec {
            event: TriggerEvent::When(WhenCondition {
                left: signal.to_string(),
                // FRAC_MAX owns the dynamic target; the parsed placeholder is
                // intentionally not used by the legacy evaluator.
                right: MeasureOperand::Constant(0.0),
                occurrence: EventOccurrence { edge, number },
            }),
            td: None,
            frac_max: Some(frac_max),
            occurrence_explicit: explicit,
        };
        let statement = |name: &str, trig: TrigSpec, targ: TrigSpec| MeasureStatement {
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
            name: name.to_string(),
            measure_type: MeasureType::Delay {
                trig,
                targ,
                from: None,
                to: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
            analysis: "TRAN".to_string(),
            goal: None,
            tolerance: None,
        };

        let axis = [0.0, 1.0, 2.0, 3.0, 4.0];
        let pulse = [0.0, 1.0, 2.0, 1.0, 0.0];
        let mut signals = HashMap::new();
        signals.insert("PULSE".to_string(), pulse.as_slice());
        let mut engine = MeasureEngine::new();
        engine.add(statement(
            "dynamic",
            make_clause("PULSE", 0.5, EdgeType::Cross, 1, false),
            make_clause("PULSE", 0.75, EdgeType::Cross, 1, false),
        ));
        // The early 1 V maximum initially produces 0.5/1.0 crossings. Once
        // the 2 V maximum arrives, Xyce revises both retained-history results
        // to 1.0 and 1.5 respectively.
        assert_eq!(engine.evaluate(&axis, &signals)[0].value, Some(0.5));

        let axis = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let alternating_lobes = [0.0, 1.0, 2.0, 1.0, 0.0, 1.0, 2.0, 1.0, 0.0, 1.0];
        let mut signals = HashMap::new();
        signals.insert("LOBES".to_string(), alternating_lobes.as_slice());
        let mut engine = MeasureEngine::new();
        engine.add(statement(
            "absolute_windows",
            make_clause("LOBES", 0.5, EdgeType::Rise, 2, true),
            make_clause("LOBES", 0.5, EdgeType::Fall, 2, true),
        ));
        assert_eq!(engine.evaluate(&axis, &signals)[0].value, Some(1.5));
    }

    #[test]
    fn trigger_target_at_requires_xyce_directional_accepted_point_reach() {
        assert!(delay_at_is_reached(
            &[0.0, 1.0, 2.0],
            1.0,
            &[],
            XYCE_DEFAULT_MEASURE_MINVAL
        ));
        assert!(!delay_at_is_reached(
            &[0.0, 1.0, 2.0],
            2.0,
            &[],
            XYCE_DEFAULT_MEASURE_MINVAL
        ));
        assert!(delay_at_is_reached(
            &[2.0, 1.0, 0.0],
            0.0,
            &[],
            XYCE_DEFAULT_MEASURE_MINVAL
        ));
        assert!(!delay_at_is_reached(
            &[0.0, 1.0, 2.0],
            3.0,
            &[],
            XYCE_DEFAULT_MEASURE_MINVAL
        ));
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
    fn err_distinguishes_inactive_windows_from_computed_ieee_results() {
        let statement = |name: &str,
                         comparison: &str,
                         from: Option<Value>,
                         to: Option<Value>,
                         minval: Value,
                         ymin: Value,
                         ymax: Value| MeasureStatement {
            name: name.to_string(),
            measure_type: MeasureType::ErrorFunction {
                measured: "M".to_string(),
                comparison: comparison.to_string(),
                norm: ErrorFunctionNorm::RootMeanSquare,
                from,
                to,
                minval,
                ymin,
                ymax,
                weight: None,
            },
            analysis: "TRAN".to_string(),
            goal: None,
            tolerance: None,
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
        };
        let mut engine = MeasureEngine::new();
        engine.add(statement("filtered", "ZERO", None, None, 0.0, 1.0, 2.0));
        engine.add(statement(
            "inactive",
            "ZERO",
            Some(3.0),
            Some(4.0),
            0.0,
            0.0,
            1.0,
        ));
        engine.add(statement(
            "zero_over_zero",
            "ZERO",
            None,
            None,
            0.0,
            0.0,
            1.0,
        ));
        engine.add(statement(
            "nonzero_over_zero",
            "ONE",
            None,
            None,
            0.0,
            0.0,
            1.0,
        ));
        let axis = [0.0, 1.0];
        let measured = [0.0, 0.0];
        let zero = [0.0, 0.0];
        let one = [1.0, 1.0];
        let signals = HashMap::from([
            ("M".to_string(), measured.as_slice()),
            ("ZERO".to_string(), zero.as_slice()),
            ("ONE".to_string(), one.as_slice()),
        ]);

        let results = engine.evaluate(&axis, &signals);
        assert!(results[0].passed && results[0].value.is_some_and(Value::is_nan));
        assert!(!results[1].passed && results[1].value.is_none());
        assert!(results[2].passed && results[2].value.is_some_and(Value::is_nan));
        assert_eq!(results[3].value, Some(Value::INFINITY));
        assert!(results[3].passed, "{:?}", results[3]);
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

    #[test]
    fn computed_nan_is_success_without_goal_and_retained_on_goal_failure() {
        let mut statement = max_statement("V(out)");
        let no_goal = MeasureResult::success("nan", Value::NAN).with_event_axis(Value::NAN);
        assert!(no_goal.passed);
        assert!(no_goal.value.is_some_and(Value::is_nan));
        assert!(no_goal.event_axis.is_some_and(Value::is_nan));
        assert_eq!(no_goal.error, None);

        statement.goal = Some(0.0);
        let with_goal = MeasureResult::success("nan", Value::NAN).check_contract(&statement);
        assert!(!with_goal.passed);
        assert!(with_goal.value.is_some_and(Value::is_nan));
        assert_eq!(with_goal.expected, Some(0.0));
        assert!(
            with_goal
                .error
                .as_deref()
                .is_some_and(|error| error.contains("NaN"))
        );
    }

    #[test]
    fn failvalue_uses_exact_inclusive_absolute_value_semantics() {
        let cases = [
            (1.999, 2.0, false),
            (2.0, 2.0, true),
            (-2.0, 2.0, true),
            (0.0, 0.0, true),
            (0.0, -1.0, true),
            (Value::INFINITY, 1.0, true),
            (Value::NAN, 1.0, false),
        ];

        for (value, limit, exceeded) in cases {
            let mut statement = max_statement("V(out)");
            statement.fail_value = Some(limit);
            let result = MeasureResult::success("value", value).check_contract(&statement);

            assert_eq!(result.raw_value.map(Value::to_bits), Some(value.to_bits()));
            assert_eq!(result.failure_limit, Some(limit));
            assert_eq!(result.failure_limit_exceeded, exceeded);
            assert_eq!(
                result.passed,
                value.is_finite() && !exceeded,
                "value={value:?}, limit={limit:?}"
            );
            if !value.is_finite() {
                assert!(
                    result
                        .error
                        .as_deref()
                        .is_some_and(|error| error.contains("non-finite"))
                );
            } else if exceeded {
                assert_eq!(result.value, Some(value));
                assert!(
                    result
                        .error
                        .as_deref()
                        .is_some_and(|error| error.contains("FAILVALUE"))
                );
            }
        }
    }

    #[test]
    fn early_scalar_failure_retains_authored_contract_metadata() {
        let mut statement = max_statement("V(out)");
        statement.goal = Some(3.0);
        statement.tolerance = Some(0.25);
        statement.fail_value = Some(5.0);
        let signals = HashMap::new();

        let results = engine_with(statement).evaluate(&[], &signals);

        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert!(!result.passed);
        assert_eq!(result.value, None);
        assert_eq!(result.raw_value, None);
        assert_eq!(result.expected, Some(3.0));
        assert_eq!(result.tolerance, Some(0.25));
        assert_eq!(result.failure_limit, Some(5.0));
        assert!(!result.failure_limit_exceeded);
        assert_eq!(result.error.as_deref(), Some("measurement axis is empty"));
    }

    #[test]
    fn projected_extrema_publish_axis_but_verify_and_feed_param_from_raw_value() {
        let mut peak = max_statement("V(out)");
        peak.name = "peak".to_string();
        peak.measure_type = MeasureType::Max {
            signal: "V(out)".to_string(),
            from: None,
            to: None,
            output: ExtremaOutput::IndependentAxis,
        };
        peak.fail_value = Some(4.0);
        let dependent = MeasureStatement {
            name: "dependent".to_string(),
            measure_type: MeasureType::Param {
                expression: MeasureExpression::expression("peak"),
            },
            analysis: "TRAN".to_string(),
            goal: None,
            tolerance: None,
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
        };
        let axis = [10.0, 20.0, 30.0];
        let values = [2.0, 5.0, 3.0];
        let signals = HashMap::from([("V(out)".to_string(), values.as_slice())]);
        let mut engine = MeasureEngine::new();
        engine.add(peak);
        engine.add(dependent);

        let results = engine.evaluate(&axis, &signals);

        assert_eq!(results[0].value, Some(20.0));
        assert_eq!(results[0].raw_value, Some(5.0));
        assert_eq!(results[0].event_axis, Some(20.0));
        assert!(results[0].failure_limit_exceeded);
        assert!(!results[0].passed);
        assert_eq!(results[1].value, Some(5.0));
        assert_eq!(results[1].raw_value, Some(5.0));
        assert!(results[1].passed);
    }

    #[test]
    fn failvalue_verdict_does_not_overwrite_an_earlier_goal_failure() {
        let mut statement = max_statement("V(out)");
        statement.goal = Some(0.0);
        statement.tolerance = Some(0.1);
        statement.fail_value = Some(1.0);

        let result = MeasureResult::success("value", 2.0).check_contract(&statement);

        assert!(!result.passed);
        assert!(result.failure_limit_exceeded);
        assert!(
            result
                .error
                .as_deref()
                .is_some_and(|error| error.contains("GOAL"))
        );
    }

    fn continuous_statement(name: &str, measure_type: MeasureType) -> MeasureStatement {
        MeasureStatement {
            name: name.to_string(),
            measure_type,
            analysis: "NOISE_CONT".to_string(),
            goal: None,
            tolerance: None,
            default_value: None,
            fail_value: None,
            print_policy: MeasurePrintPolicy::All,
        }
    }

    #[test]
    fn continuous_failvalue_verifies_each_record_without_collapsing_the_stream() {
        let axis = [0.0, 1.0, 2.0];
        let signal = [-1.0, 1.0, -1.0];
        let mut signals = HashMap::new();
        signals.insert("ALT".to_string(), signal.as_slice());

        let mut statement = continuous_statement(
            "contract",
            MeasureType::When {
                condition: alternating_condition(1),
                from: None,
                to: None,
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
        );
        statement.fail_value = Some(1.0);
        let mut engine = MeasureEngine::new();
        engine.add(statement);

        let result = &engine.evaluate_continuous(&axis, &signals, &[])[0];
        assert_eq!(result.failure, None);
        assert_eq!(result.records.len(), 2);
        assert_eq!(result.records[0].raw_value, 0.5);
        assert_eq!(result.records[0].failure_limit, Some(1.0));
        assert!(!result.records[0].failure_limit_exceeded);
        assert!(result.records[0].passed);
        assert_eq!(
            result.records[0].coordinate(),
            Some(ContinuousMeasureCoordinate::Point { axis: 0.5 })
        );
        assert_eq!(result.records[1].raw_value, 1.5);
        assert!(result.records[1].failure_limit_exceeded);
        assert!(!result.records[1].passed);
        assert_eq!(
            result.records[1].verification_failure,
            Some(ContinuousMeasureVerificationFailure::FailureLimitExceeded)
        );
        assert!(!result.passed());
        assert_eq!(result.failed_record_count(), 1);
    }

    #[test]
    fn continuous_failvalue_fails_closed_for_non_finite_values_and_thresholds() {
        let axis = [0.0, 1.0];
        let crossing = [-1.0, 1.0];
        let infinite = [Value::INFINITY, Value::INFINITY];
        let signals = HashMap::from([
            ("CROSSING".to_string(), crossing.as_slice()),
            ("INFINITE".to_string(), infinite.as_slice()),
        ]);
        let when = || WhenCondition {
            left: "CROSSING".to_string(),
            right: MeasureOperand::Constant(0.0),
            occurrence: EventOccurrence::default(),
        };

        let mut non_finite_raw = continuous_statement(
            "raw",
            MeasureType::Find {
                signal: "INFINITE".to_string(),
                at: None,
                when: Some(when()),
                from: None,
                to: None,
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
        );
        non_finite_raw.fail_value = Some(1.0);
        let mut non_finite_limit = continuous_statement(
            "limit",
            MeasureType::When {
                condition: when(),
                from: None,
                to: None,
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
        );
        non_finite_limit.fail_value = Some(Value::NAN);

        let mut engine = MeasureEngine::new();
        engine.add(non_finite_raw);
        engine.add(non_finite_limit);
        let results = engine.evaluate_continuous(&axis, &signals, &[]);

        assert_eq!(results[0].records[0].raw_value, Value::INFINITY);
        assert_eq!(
            results[0].records[0].verification_failure,
            Some(ContinuousMeasureVerificationFailure::NonFiniteRawValue)
        );
        assert!(!results[0].records[0].passed);
        assert_eq!(
            results[1].records[0].verification_failure,
            Some(ContinuousMeasureVerificationFailure::NonFiniteFailureLimit)
        );
        assert!(!results[1].records[0].passed);
        assert!(results.iter().all(|result| !result.passed()));
    }

    #[test]
    fn continuous_public_records_fail_invariants_when_contract_fields_disagree() {
        let malformed = ContinuousMeasureRecord {
            value: 2.0,
            raw_value: 2.0,
            event_axis: Some(0.5),
            trigger_axis: None,
            target_axis: None,
            failure_limit: Some(1.0),
            failure_limit_exceeded: false,
            passed: false,
            verification_failure: Some(ContinuousMeasureVerificationFailure::FailureLimitExceeded),
        };
        let result = ContinuousMeasureResult {
            name: "malformed".to_string(),
            records: vec![malformed],
            failure: None,
            failure_metadata: None,
        };

        assert_eq!(
            result.validate_invariants(),
            Err("continuous measurement FAILVALUE verdict is inconsistent")
        );

        let mut wrong_reason = result;
        wrong_reason.records[0].failure_limit_exceeded = true;
        wrong_reason.records[0].verification_failure =
            Some(ContinuousMeasureVerificationFailure::NonFiniteRawValue);
        assert_eq!(
            wrong_reason.validate_invariants(),
            Err("continuous measurement typed verification failure is inconsistent")
        );
    }

    fn alternating_condition(number: isize) -> WhenCondition {
        WhenCondition {
            left: "ALT".to_string(),
            right: MeasureOperand::Constant(0.0),
            occurrence: EventOccurrence {
                edge: EdgeType::Cross,
                number,
            },
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
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
        ));
        engine.add(continuous_statement(
            "from_end",
            MeasureType::When {
                condition: alternating_condition(-2),
                from: None,
                to: None,
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
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
    fn continuous_td_applies_only_to_transient_event_streams() {
        let axis = [0.0, 1.0, 2.0, 3.0, 4.0];
        let alternating = [-1.0, 1.0, -1.0, 1.0, -1.0];
        let mut signals = HashMap::new();
        signals.insert("ALT".to_string(), alternating.as_slice());
        let point_measure = |name: &str, td: Value| {
            continuous_statement(
                name,
                MeasureType::When {
                    condition: alternating_condition(1),
                    from: None,
                    to: None,
                    td: Some(td),
                    minval: XYCE_DEFAULT_MEASURE_MINVAL,
                },
            )
        };
        let mut transient = point_measure("transient", 2.0);
        transient.analysis = "TRAN_CONT".to_string();
        let mut ac = point_measure("ac", 10.0);
        ac.analysis = "AC_CONT".to_string();
        let mut engine = MeasureEngine::new();
        engine.add(transient);
        engine.add(ac);

        let results = engine.evaluate_continuous(&axis, &signals, &[]);
        assert_eq!(
            results[0]
                .records
                .iter()
                .filter_map(|record| record.event_axis)
                .collect::<Vec<_>>(),
            vec![2.5, 3.5]
        );
        assert_eq!(
            results[1]
                .records
                .iter()
                .filter_map(|record| record.event_axis)
                .collect::<Vec<_>>(),
            vec![0.5, 1.5, 2.5, 3.5]
        );
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
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
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
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
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
    fn continuous_find_and_derivative_preserve_extended_real_values() {
        let axis = [0.0, 1.0];
        let crossing = [-1.0, 1.0];
        let constant_infinite = [Value::NEG_INFINITY, Value::NEG_INFINITY];
        let infinite_slope = [0.0, Value::INFINITY];
        let mut signals = HashMap::new();
        signals.insert("CROSSING".to_string(), crossing.as_slice());
        signals.insert(
            "CONSTANT_INFINITY".to_string(),
            constant_infinite.as_slice(),
        );
        signals.insert("INFINITE_SLOPE".to_string(), infinite_slope.as_slice());
        let when = || WhenCondition {
            left: "CROSSING".to_string(),
            right: MeasureOperand::Constant(0.0),
            occurrence: EventOccurrence::default(),
        };
        let mut engine = MeasureEngine::new();
        engine.add(continuous_statement(
            "find",
            MeasureType::Find {
                signal: "CONSTANT_INFINITY".to_string(),
                at: None,
                when: Some(when()),
                from: None,
                to: None,
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
        ));
        engine.add(continuous_statement(
            "derivative",
            MeasureType::Derivative {
                signal: "INFINITE_SLOPE".to_string(),
                at: None,
                when: Some(when()),
                from: None,
                to: None,
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
        ));

        let results = engine.evaluate_continuous(&axis, &signals, &[]);
        assert_eq!(results[0].records[0].value, Value::NEG_INFINITY);
        assert_eq!(results[1].records[0].value, Value::INFINITY);
        assert!(results.iter().all(|result| result.failure.is_none()));
    }

    #[test]
    fn continuous_extended_real_undefined_results_preserve_ieee_records() {
        let axis = [0.0, 1.0, 2.0];
        let crossing = [-1.0, 1.0, -1.0];
        let opposite_infinities = [Value::INFINITY, Value::NEG_INFINITY, Value::NEG_INFINITY];
        let mixed_derivative = [Value::INFINITY, Value::INFINITY, 0.0];
        let mut signals = HashMap::new();
        signals.insert("CROSSING".to_string(), crossing.as_slice());
        signals.insert("OPPOSITE".to_string(), opposite_infinities.as_slice());
        signals.insert("MIXED_DERIVATIVE".to_string(), mixed_derivative.as_slice());
        let when = || WhenCondition {
            left: "CROSSING".to_string(),
            right: MeasureOperand::Constant(0.0),
            occurrence: EventOccurrence::default(),
        };
        let mut engine = MeasureEngine::new();
        engine.add(continuous_statement(
            "find_at",
            MeasureType::Find {
                signal: "OPPOSITE".to_string(),
                at: Some(0.5),
                when: None,
                from: None,
                to: None,
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
        ));
        engine.add(continuous_statement(
            "find_when",
            MeasureType::Find {
                signal: "OPPOSITE".to_string(),
                at: None,
                when: Some(when()),
                from: None,
                to: None,
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
        ));
        engine.add(continuous_statement(
            "derivative",
            MeasureType::Derivative {
                signal: "MIXED_DERIVATIVE".to_string(),
                at: None,
                when: Some(when()),
                from: None,
                to: None,
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
        ));

        let results = engine.evaluate_continuous(&axis, &signals, &[]);
        assert!(results.iter().all(|result| result.failure.is_none()));
        assert!(results.iter().all(|result| !result.records.is_empty()));
        assert!(
            results
                .iter()
                .all(|result| result.records.iter().any(|record| record.value.is_nan()))
        );
        assert!(
            results
                .iter()
                .all(|result| result.validate_invariants().is_ok())
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
                from: None,
                to: None,
                trig: TrigSpec {
                    event: TriggerEvent::When(alternating_condition(2)),
                    td: None,
                    frac_max: None,
                    occurrence_explicit: true,
                },
                targ: TrigSpec {
                    event: TriggerEvent::When(WhenCondition {
                        left: "ALT".to_string(),
                        right: MeasureOperand::Constant(0.5),
                        occurrence: EventOccurrence {
                            edge: EdgeType::Cross,
                            number: 1,
                        },
                    }),
                    td: None,
                    frac_max: None,
                    occurrence_explicit: true,
                },
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
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
            frac_max: None,
            occurrence_explicit: true,
        };
        let missing = TrigSpec {
            event: TriggerEvent::At(10.0),
            td: None,
            frac_max: None,
            occurrence_explicit: false,
        };
        let mut engine = MeasureEngine::new();
        engine.add(continuous_statement(
            "missing_target",
            MeasureType::Delay {
                from: None,
                to: None,
                trig: found.clone(),
                targ: missing.clone(),
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
            },
        ));
        engine.add(continuous_statement(
            "missing_trigger",
            MeasureType::Delay {
                from: None,
                to: None,
                trig: missing,
                targ: found,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
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
                },
                from: None,
                to: None,
                td: None,
                minval: XYCE_DEFAULT_MEASURE_MINVAL,
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
