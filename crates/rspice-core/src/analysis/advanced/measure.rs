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
        when_signal: Option<String>,
        when_value: Option<Value>,
    },

    /// Time-derivative of a signal at a point
    /// .MEAS TRAN name DERIV V(out) AT=time | WHEN sig=value
    Derivative {
        signal: String,
        at: Option<Value>,
        when_signal: Option<String>,
        when_value: Option<Value>,
    },

    /// Expression over previously evaluated measurement results
    /// .MEAS TRAN name PARAM='expr'
    Param { expression: String },

    /// Minimum value over range
    Min {
        signal: String,
        from: Option<Value>,
        to: Option<Value>,
    },

    /// Maximum value over range
    Max {
        signal: String,
        from: Option<Value>,
        to: Option<Value>,
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
    fn check_goal(mut self, statement: &MeasureStatement) -> Self {
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
                MeasureType::Param { .. } => {
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
                when_signal,
                when_value,
            } => self.eval_derivative(
                &measurement.name,
                signal,
                *at,
                when_signal.as_deref(),
                *when_value,
                time,
                signals,
            ),
            MeasureType::Param { .. } => MeasureResult::failed(
                &measurement.name,
                "PARAM measures evaluate after the directly computed set",
            ),
            MeasureType::Min { signal, from, to } => {
                self.eval_min_max(&measurement.name, signal, *from, *to, time, signals, false)
            }
            MeasureType::Max { signal, from, to } => {
                self.eval_min_max(&measurement.name, signal, *from, *to, time, signals, true)
            }
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
                when_signal,
                when_value,
            } => self.eval_find(
                &measurement.name,
                signal,
                *at,
                when_signal.as_deref(),
                *when_value,
                time,
                signals,
            ),
            MeasureType::Integ { signal, from, to } => {
                self.eval_integ(&measurement.name, signal, *from, *to, time, signals)
            }
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

    fn get_range_indices(
        &self,
        time: &[Value],
        from: Option<Value>,
        to: Option<Value>,
    ) -> (usize, usize) {
        let start = from
            .map(|t| time.iter().position(|&x| x >= t).unwrap_or(0))
            .unwrap_or(0);
        let end = to
            .map(|t| time.iter().rposition(|&x| x <= t).unwrap_or(time.len() - 1))
            .unwrap_or(time.len() - 1);
        (start, end.max(start))
    }

    fn eval_min_max(
        &self,
        name: &str,
        signal_name: &str,
        from: Option<Value>,
        to: Option<Value>,
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

        let (start, end) = self.get_range_indices(time, from, to);
        let slice = &signal[start..=end];

        if slice.is_empty() {
            return MeasureResult::failed(name, "Empty range");
        }

        let result = if is_max {
            slice.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
        } else {
            slice.iter().cloned().fold(f64::INFINITY, f64::min)
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

        let (start, end) = self.get_range_indices(time, from, to);
        let slice = &signal[start..=end];

        if slice.is_empty() {
            return MeasureResult::failed(name, "Empty range");
        }

        let min_val = slice.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = slice.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

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

        let (start, end) = self.get_range_indices(time, from, to);

        if start >= end {
            return MeasureResult::failed(name, "Empty range");
        }

        // Trapezoidal integration for accurate average
        let mut integral = 0.0;
        for i in start..end {
            let dt = time[i + 1] - time[i];
            integral += 0.5 * (signal[i] + signal[i + 1]) * dt;
        }

        let total_time = time[end] - time[start];
        MeasureResult::success(name, integral / total_time)
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

        let (start, end) = self.get_range_indices(time, from, to);

        if start >= end {
            return MeasureResult::failed(name, "Empty range");
        }

        // Trapezoidal integration of squared signal
        let mut integral = 0.0;
        for i in start..end {
            let dt = time[i + 1] - time[i];
            let sq1 = signal[i] * signal[i];
            let sq2 = signal[i + 1] * signal[i + 1];
            integral += 0.5 * (sq1 + sq2) * dt;
        }

        let total_time = time[end] - time[start];
        MeasureResult::success(name, (integral / total_time).sqrt())
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
        when_signal: Option<&str>,
        when_value: Option<Value>,
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
        let target_time = if at.is_some() {
            at
        } else if let (Some(when_name), Some(threshold)) = (when_signal, when_value) {
            let when_sig = match lookup_signal(signals, when_name) {
                Some(s) => s,
                None => {
                    return MeasureResult::failed(
                        name,
                        &format!("When signal '{}' not found", when_name),
                    );
                }
            };
            self.find_crossing(time, when_sig, threshold, EdgeType::Cross, 1, None)
        } else {
            return MeasureResult::failed(name, "DERIV requires AT=time or WHEN signal=value");
        };
        let Some(t_star) = target_time else {
            return MeasureResult::failed(name, "WHEN condition never met");
        };
        for i in 0..time.len() - 1 {
            if time[i] <= t_star && time[i + 1] >= t_star && time[i + 1] > time[i] {
                let slope = (signal[i + 1] - signal[i]) / (time[i + 1] - time[i]);
                return MeasureResult::success(name, slope);
            }
        }
        MeasureResult::failed(name, "Time point not in simulation range")
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
            Ok(value) if value.im == 0.0 => MeasureResult::success(name, value.re),
            Ok(value) => MeasureResult::failed(
                name,
                &format!(
                    "PARAM expression produced non-finite/non-real value {} + {}j",
                    value.re, value.im
                ),
            ),
            Err(err) => MeasureResult::failed(name, &format!("PARAM expression failed: {err}")),
        }
    }

    fn eval_find(
        &self,
        name: &str,
        signal_name: &str,
        at: Option<Value>,
        when_signal: Option<&str>,
        when_value: Option<Value>,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
    ) -> MeasureResult {
        let signal = match lookup_signal(signals, signal_name) {
            Some(s) => s,
            None => {
                return MeasureResult::failed(name, &format!("Signal '{}' not found", signal_name));
            }
        };

        if let Some(t_at) = at {
            // FIND ... AT=time
            for i in 0..time.len() - 1 {
                if time[i] <= t_at && time[i + 1] > t_at {
                    let frac = (t_at - time[i]) / (time[i + 1] - time[i]);
                    let value = signal[i] + frac * (signal[i + 1] - signal[i]);
                    return MeasureResult::success(name, value);
                }
            }
            return MeasureResult::failed(name, "Time point not in simulation range");
        }

        if let (Some(when_sig_name), Some(threshold)) = (when_signal, when_value) {
            // FIND ... WHEN condition=value
            let when_sig = match lookup_signal(signals, when_sig_name) {
                Some(s) => s,
                None => {
                    return MeasureResult::failed(
                        name,
                        &format!("When signal '{}' not found", when_sig_name),
                    );
                }
            };

            if let Some(t_when) =
                self.find_crossing(time, when_sig, threshold, EdgeType::Cross, 1, None)
            {
                // Interpolate signal at this time
                for i in 0..time.len() - 1 {
                    if time[i] <= t_when && time[i + 1] > t_when {
                        let frac = (t_when - time[i]) / (time[i + 1] - time[i]);
                        let value = signal[i] + frac * (signal[i + 1] - signal[i]);
                        return MeasureResult::success(name, value);
                    }
                }
            }
            return MeasureResult::failed(name, "WHEN condition not found");
        }

        MeasureResult::failed(name, "FIND requires AT= or WHEN condition")
    }

    fn eval_integ(
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

        let (start, end) = self.get_range_indices(time, from, to);

        if start >= end {
            return MeasureResult::failed(name, "Empty range");
        }

        let mut integral = 0.0;
        for i in start..end {
            let dt = time[i + 1] - time[i];
            integral += 0.5 * (signal[i] + signal[i + 1]) * dt;
        }

        MeasureResult::success(name, integral)
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

    fn max_statement(signal: &str) -> MeasureStatement {
        MeasureStatement {
            goal: None,
            tolerance: None,
            name: "peak".to_string(),
            measure_type: MeasureType::Max {
                signal: signal.to_string(),
                from: None,
                to: None,
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
    fn param_measure_rejects_non_finite_value() {
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
                .contains("non-finite")
        );
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
