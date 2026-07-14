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

/// A typed conditional event used by FIND and DERIV measurements.
#[derive(Debug, Clone, PartialEq)]
pub struct WhenCondition {
    pub left: String,
    pub right: MeasureOperand,
}

/// Trigger/Target specification for delay measurements
#[derive(Debug, Clone)]
pub struct TrigSpec {
    /// Signal name (e.g., "V(out)")
    pub signal: String,
    /// Threshold value
    pub value: Value,
    /// Edge type (rise/fall)
    pub edge: EdgeType,
    /// Which occurrence (1 = first, 2 = second, etc.)
    pub number: usize,
    /// Time delay offset (TD=)
    pub td: Value,
}

impl TrigSpec {
    pub fn new(signal: &str, value: Value) -> Self {
        Self {
            signal: signal.to_string(),
            value,
            edge: EdgeType::Rise,
            number: 1,
            td: 0.0,
        }
    }

    pub fn with_edge(mut self, edge: EdgeType) -> Self {
        self.edge = edge;
        self
    }

    pub fn with_number(mut self, n: usize) -> Self {
        self.number = n;
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
        default_value: Option<Value>,
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
        }
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
        if self.measurements.is_empty() {
            return Vec::new();
        }
        if time.is_empty() {
            return self.fail_all("measurement axis is empty");
        }
        if let Some(index) = time.iter().position(|value| !value.is_finite()) {
            return self.fail_all(&format!(
                "measurement axis contains non-finite sample at index {index}"
            ));
        }
        if let Some((name, signal)) = signals
            .iter()
            .find(|(_, signal)| signal.len() != time.len())
        {
            return self.fail_all(&format!(
                "signal '{name}' has {} samples but measurement axis has {}",
                signal.len(),
                time.len()
            ));
        }
        if let Some((name, index, value)) = signals.iter().find_map(|(name, signal)| {
            signal
                .iter()
                .enumerate()
                .find(|(_, value)| !value.is_finite())
                .map(|(index, value)| (name, index, *value))
        }) {
            return self.fail_all(&format!(
                "signal '{name}' contains non-finite sample at index {index}: {value}"
            ));
        }

        // Expression measures (PARAM='...') read other results by name, so
        // they evaluate in a second pass over the directly computed set —
        // and in statement order, so a PARAM may reference an earlier PARAM.
        let mut results: Vec<MeasureResult> = self
            .measurements
            .iter()
            .map(|m| match &m.measure_type {
                MeasureType::Param { .. } | MeasureType::Equation { .. } => {
                    MeasureResult::failed(&m.name, "PARAM expression not yet evaluated")
                }
                _ => self.evaluate_one(m, time, signals),
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
    ) -> MeasureResult {
        self.evaluate_kind(measurement, time, signals)
            .check_goal(measurement)
    }

    fn evaluate_kind(
        &self,
        measurement: &MeasureStatement,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
    ) -> MeasureResult {
        match &measurement.measure_type {
            MeasureType::Delay { trig, targ } => {
                self.eval_delay(&measurement.name, trig, targ, time, signals)
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
            ),
            MeasureType::Param { .. } => MeasureResult::failed(
                &measurement.name,
                "PARAM measures evaluate after the directly computed set",
            ),
            MeasureType::Equation { .. } => MeasureResult::failed(
                &measurement.name,
                "continuous equation measures evaluate on the analysis-point stream",
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
    ) -> MeasureResult {
        let trig_sig = match lookup_signal(signals, &trig.signal) {
            Some(s) => s,
            None => {
                return MeasureResult::failed(name, &format!("Signal '{}' not found", trig.signal));
            }
        };
        let targ_sig = match lookup_signal(signals, &targ.signal) {
            Some(s) => s,
            None => {
                return MeasureResult::failed(name, &format!("Signal '{}' not found", targ.signal));
            }
        };

        let t_trig = match self.find_crossing(
            time,
            trig_sig,
            trig.value,
            trig.edge,
            trig.number,
            Some(trig.td),
        ) {
            Some(t) => t,
            None => return MeasureResult::failed(name, "Trigger condition not found"),
        };

        let t_targ = match self.find_crossing(
            time,
            targ_sig,
            targ.value,
            targ.edge,
            targ.number,
            Some(targ.td),
        ) {
            Some(t) => t,
            None => return MeasureResult::failed(name, "Target condition not found"),
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

        MeasureResult::success(name, result)
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
                return MeasureResult::failed(name, "AT point is outside the measurement window");
            }
            let Some(segment) = measurement_segment_containing(time, target) else {
                return MeasureResult::failed(name, "Time point not in simulation range");
            };
            return measurement_segment_slope(name, time, signal, segment);
        }

        let Some(condition) = when else {
            return MeasureResult::failed(name, "DERIV requires AT=time or WHEN signal=value");
        };
        let left = match lookup_signal(signals, &condition.left) {
            Some(signal) => signal,
            None => {
                return MeasureResult::failed(
                    name,
                    &format!("When signal '{}' not found", condition.left),
                );
            }
        };
        let right = match resolve_measure_operand(&condition.right, signals) {
            Ok(operand) => operand,
            Err(error) => return MeasureResult::failed(name, &error),
        };
        for (segment, fraction) in measurement_condition_crossings(left, right, time.len()) {
            let axis = time[segment] + fraction * (time[segment + 1] - time[segment]);
            if Self::axis_in_measurement_window(axis, lower, upper) {
                return measurement_segment_slope(name, time, signal, segment);
            }
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
            if let Some(value) = interpolate_measure_signal(time, signal, t_at) {
                return MeasureResult::success(name, value);
            }
            return MeasureResult::failed(name, "Time point not in simulation range");
        }

        if let Some(condition) = when {
            let left = match lookup_signal(signals, &condition.left) {
                Some(s) => s,
                None => {
                    return MeasureResult::failed(
                        name,
                        &format!("When signal '{}' not found", condition.left),
                    );
                }
            };
            let right = match resolve_measure_operand(&condition.right, signals) {
                Ok(operand) => operand,
                Err(error) => return MeasureResult::failed(name, &error),
            };
            for (segment, fraction) in measurement_condition_crossings(left, right, time.len()) {
                let axis = time[segment] + fraction * (time[segment + 1] - time[segment]);
                if Self::axis_in_measurement_window(axis, lower, upper) {
                    let value =
                        signal[segment] + fraction * (signal[segment + 1] - signal[segment]);
                    return MeasureResult::success(name, value);
                }
            }
            return MeasureResult::failed(
                name,
                "WHEN condition not found in the measurement window",
            );
        }

        MeasureResult::failed(name, "FIND requires AT= or WHEN condition")
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

/// Return every qualifying `WHEN left=right` crossing interval in traversal
/// order. Xyce requires the left operand itself to change over the interval;
/// a moving right operand cannot trigger against a constant left operand.
fn measurement_condition_crossings(
    left: &[Value],
    right: ResolvedMeasureOperand<'_>,
    point_count: usize,
) -> Vec<(usize, Value)> {
    const XYCE_WHEN_ABSOLUTE_TOLERANCE: Value = 1.0e-12;
    if left.len() != point_count || point_count < 2 {
        return Vec::new();
    }

    let mut crossings = Vec::new();
    for segment in 0..point_count - 1 {
        let left_previous = left[segment];
        let left_current = left[segment + 1];
        let left_scale = left_previous.abs().max(left_current.abs()).max(1.0);
        if (left_current - left_previous).abs() <= XYCE_WHEN_ABSOLUTE_TOLERANCE * left_scale {
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
        let current_equal = current_difference.abs() < XYCE_WHEN_ABSOLUTE_TOLERANCE;
        let strict_crossing = (previous_difference < 0.0 && current_difference > 0.0)
            || (previous_difference > 0.0 && current_difference < 0.0);
        if !current_equal && !strict_crossing {
            continue;
        }

        let denominator = current_difference - previous_difference;
        let fraction = if denominator == 0.0 {
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

fn measurement_segment_containing(axis: &[Value], target: Value) -> Option<usize> {
    const XYCE_AT_ABSOLUTE_TOLERANCE: Value = 1.0e-12;
    axis.windows(2).enumerate().find_map(|(segment, pair)| {
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

fn interpolate_measure_signal(axis: &[Value], signal: &[Value], target: Value) -> Option<Value> {
    if axis.len() != signal.len() || axis.is_empty() || !target.is_finite() {
        return None;
    }
    if let Some(index) = axis.iter().position(|value| *value == target) {
        return signal.get(index).copied();
    }
    axis.windows(2).enumerate().find_map(|(index, segment)| {
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
    }

    fn max_statement(signal: &str) -> MeasureStatement {
        MeasureStatement {
            goal: None,
            tolerance: None,
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
            }),
            None,
            None,
        ));

        let results = engine.evaluate(&axis, &signals);
        assert_eq!(results[0].value, Some(7.0));
        assert_eq!(results[1].value, Some(0.9));
        assert!(!results[2].passed);
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
}
