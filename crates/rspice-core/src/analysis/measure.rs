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

use crate::Value;
use std::collections::HashMap;

//=============================================================================
// Measurement Types
//=============================================================================

/// Edge type for trigger/target detection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    /// Rising edge (signal crosses threshold going up)
    Rise,
    /// Falling edge (signal crosses threshold going down)
    Fall,
    /// Either edge
    Cross,
}

impl Default for EdgeType {
    fn default() -> Self {
        EdgeType::Rise
    }
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
    Delay {
        trig: TrigSpec,
        targ: TrigSpec,
    },
    
    /// Find value at specific time or when condition is met
    /// .MEAS TRAN name FIND V(out) AT=time
    /// .MEAS TRAN name FIND V(out) WHEN V(in)=0.5
    Find {
        signal: String,
        at: Option<Value>,
        when_signal: Option<String>,
        when_value: Option<Value>,
    },
    
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
        from_pct: Value,   // e.g., 0.1 for 10%
        to_pct: Value,     // e.g., 0.9 for 90%
        number: usize,     // Which transition
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
}

impl MeasureResult {
    pub fn success(name: &str, value: Value) -> Self {
        Self {
            name: name.to_string(),
            value: Some(value),
            error: None,
        }
    }

    pub fn failed(name: &str, error: &str) -> Self {
        Self {
            name: name.to_string(),
            value: None,
            error: Some(error.to_string()),
        }
    }
}

//=============================================================================
// Measurement Engine
//=============================================================================

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
        self.measurements
            .iter()
            .map(|m| self.evaluate_one(m, time, signals))
            .collect()
    }

    fn evaluate_one(
        &self,
        measurement: &MeasureStatement,
        time: &[Value],
        signals: &HashMap<String, &[Value]>,
    ) -> MeasureResult {
        match &measurement.measure_type {
            MeasureType::Delay { trig, targ } => {
                self.eval_delay(&measurement.name, trig, targ, time, signals)
            }
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
            MeasureType::RiseTime { signal, from_pct, to_pct, number } => {
                self.eval_rise_fall(&measurement.name, signal, *from_pct, *to_pct, *number, true, time, signals)
            }
            MeasureType::FallTime { signal, from_pct, to_pct, number } => {
                self.eval_rise_fall(&measurement.name, signal, *from_pct, *to_pct, *number, false, time, signals)
            }
            MeasureType::Find { signal, at, when_signal, when_value } => {
                self.eval_find(&measurement.name, signal, *at, when_signal.as_deref(), *when_value, time, signals)
            }
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
    ) -> Option<Value> {
        let mut count = 0;

        for i in 1..signal.len() {
            let prev = signal[i - 1];
            let curr = signal[i];
            let crossed = match edge {
                EdgeType::Rise => prev < threshold && curr >= threshold,
                EdgeType::Fall => prev > threshold && curr <= threshold,
                EdgeType::Cross => (prev < threshold && curr >= threshold)
                    || (prev > threshold && curr <= threshold),
            };

            if crossed {
                count += 1;
                if count == occurrence {
                    // Linear interpolation for exact crossing time
                    let frac = (threshold - prev) / (curr - prev);
                    return Some(time[i - 1] + frac * (time[i] - time[i - 1]));
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
        let trig_sig = match signals.get(&trig.signal) {
            Some(s) => *s,
            None => return MeasureResult::failed(name, &format!("Signal '{}' not found", trig.signal)),
        };
        let targ_sig = match signals.get(&targ.signal) {
            Some(s) => *s,
            None => return MeasureResult::failed(name, &format!("Signal '{}' not found", targ.signal)),
        };

        let t_trig = match self.find_crossing(time, trig_sig, trig.value, trig.edge, trig.number) {
            Some(t) => t + trig.td,
            None => return MeasureResult::failed(name, "Trigger condition not found"),
        };

        let t_targ = match self.find_crossing(time, targ_sig, targ.value, targ.edge, targ.number) {
            Some(t) => t + targ.td,
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
        let start = from.map(|t| time.iter().position(|&x| x >= t).unwrap_or(0)).unwrap_or(0);
        let end = to.map(|t| time.iter().rposition(|&x| x <= t).unwrap_or(time.len() - 1))
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
        let signal = match signals.get(signal_name) {
            Some(s) => *s,
            None => return MeasureResult::failed(name, &format!("Signal '{}' not found", signal_name)),
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
        let signal = match signals.get(signal_name) {
            Some(s) => *s,
            None => return MeasureResult::failed(name, &format!("Signal '{}' not found", signal_name)),
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
        let signal = match signals.get(signal_name) {
            Some(s) => *s,
            None => return MeasureResult::failed(name, &format!("Signal '{}' not found", signal_name)),
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
        let signal = match signals.get(signal_name) {
            Some(s) => *s,
            None => return MeasureResult::failed(name, &format!("Signal '{}' not found", signal_name)),
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
        let signal = match signals.get(signal_name) {
            Some(s) => *s,
            None => return MeasureResult::failed(name, &format!("Signal '{}' not found", signal_name)),
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

        let edge = if is_rise { EdgeType::Rise } else { EdgeType::Fall };
        
        let t1 = self.find_crossing(time, signal, th_low, edge, number);
        let t2 = self.find_crossing(time, signal, th_high, edge, number);

        match (t1, t2) {
            (Some(t1), Some(t2)) => MeasureResult::success(name, (t2 - t1).abs()),
            _ => MeasureResult::failed(name, "Rise/fall transition not found"),
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
        let signal = match signals.get(signal_name) {
            Some(s) => *s,
            None => return MeasureResult::failed(name, &format!("Signal '{}' not found", signal_name)),
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
            let when_sig = match signals.get(when_sig_name) {
                Some(s) => *s,
                None => return MeasureResult::failed(name, &format!("When signal '{}' not found", when_sig_name)),
            };

            if let Some(t_when) = self.find_crossing(time, when_sig, threshold, EdgeType::Cross, 1) {
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
        let signal = match signals.get(signal_name) {
            Some(s) => *s,
            None => return MeasureResult::failed(name, &format!("Signal '{}' not found", signal_name)),
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

    fn make_sine_data() -> (Vec<Value>, Vec<Value>) {
        use std::f64::consts::PI;
        let time: Vec<Value> = (0..1001).map(|i| i as f64 / 1000.0).collect();
        let signal: Vec<Value> = time.iter().map(|&t| (2.0 * PI * t).sin()).collect();
        (time, signal)
    }

    fn make_pulse_data() -> (Vec<Value>, Vec<Value>) {
        let time: Vec<Value> = (0..1001).map(|i| i as f64 / 1000.0).collect();
        let signal: Vec<Value> = time.iter().map(|&t| {
            if t < 0.1 { 0.0 }
            else if t < 0.2 { (t - 0.1) * 10.0 } // Rise from 0 to 1
            else if t < 0.8 { 1.0 }
            else if t < 0.9 { 1.0 - (t - 0.8) * 10.0 } // Fall from 1 to 0
            else { 0.0 }
        }).collect();
        (time, signal)
    }

    #[test]
    fn test_max_measurement() {
        let (time, signal) = make_sine_data();
        let mut signals = HashMap::new();
        signals.insert("V(out)".to_string(), signal.as_slice());

        let mut engine = MeasureEngine::new();
        engine.add(MeasureStatement {
            name: "vmax".to_string(),
            measure_type: MeasureType::Max {
                signal: "V(out)".to_string(),
                from: None,
                to: None,
            },
            analysis: "TRAN".to_string(),
        });

        let results = engine.evaluate(&time, &signals);
        assert!(results[0].value.is_some());
        assert!((results[0].value.unwrap() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_peak_to_peak() {
        let (time, signal) = make_sine_data();
        let mut signals = HashMap::new();
        signals.insert("V(out)".to_string(), signal.as_slice());

        let mut engine = MeasureEngine::new();
        engine.add(MeasureStatement {
            name: "vpp".to_string(),
            measure_type: MeasureType::PeakToPeak {
                signal: "V(out)".to_string(),
                from: None,
                to: None,
            },
            analysis: "TRAN".to_string(),
        });

        let results = engine.evaluate(&time, &signals);
        assert!(results[0].value.is_some());
        assert!((results[0].value.unwrap() - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_avg_sine() {
        let (time, signal) = make_sine_data();
        let mut signals = HashMap::new();
        signals.insert("V(out)".to_string(), signal.as_slice());

        let mut engine = MeasureEngine::new();
        engine.add(MeasureStatement {
            name: "vavg".to_string(),
            measure_type: MeasureType::Avg {
                signal: "V(out)".to_string(),
                from: None,
                to: None,
            },
            analysis: "TRAN".to_string(),
        });

        let results = engine.evaluate(&time, &signals);
        assert!(results[0].value.is_some());
        // Average of sine over full period should be ~0
        assert!(results[0].value.unwrap().abs() < 0.01);
    }

    #[test]
    fn test_rms_sine() {
        let (time, signal) = make_sine_data();
        let mut signals = HashMap::new();
        signals.insert("V(out)".to_string(), signal.as_slice());

        let mut engine = MeasureEngine::new();
        engine.add(MeasureStatement {
            name: "vrms".to_string(),
            measure_type: MeasureType::Rms {
                signal: "V(out)".to_string(),
                from: None,
                to: None,
            },
            analysis: "TRAN".to_string(),
        });

        let results = engine.evaluate(&time, &signals);
        assert!(results[0].value.is_some());
        // RMS of unit sine = 1/sqrt(2) ≈ 0.707
        assert!((results[0].value.unwrap() - 0.707).abs() < 0.01);
    }

    #[test]
    fn test_rise_time() {
        let (time, signal) = make_pulse_data();
        let mut signals = HashMap::new();
        signals.insert("V(out)".to_string(), signal.as_slice());

        let mut engine = MeasureEngine::new();
        engine.add(MeasureStatement {
            name: "trise".to_string(),
            measure_type: MeasureType::RiseTime {
                signal: "V(out)".to_string(),
                from_pct: 0.1,
                to_pct: 0.9,
                number: 1,
            },
            analysis: "TRAN".to_string(),
        });

        let results = engine.evaluate(&time, &signals);
        assert!(results[0].value.is_some());
        // Rise from 0.1 to 0.9 of 1V over 100ms of rise = 80ms
        assert!((results[0].value.unwrap() - 0.08).abs() < 0.01);
    }

    #[test]
    fn test_delay_measurement() {
        let (time, _) = make_pulse_data();
        // Input: step at t=0.05
        let input: Vec<Value> = time.iter().map(|&t| if t < 0.05 { 0.0 } else { 1.0 }).collect();
        // Output: delayed step at t=0.15
        let output: Vec<Value> = time.iter().map(|&t| if t < 0.15 { 0.0 } else { 1.0 }).collect();

        let mut signals = HashMap::new();
        signals.insert("V(in)".to_string(), input.as_slice());
        signals.insert("V(out)".to_string(), output.as_slice());

        let mut engine = MeasureEngine::new();
        engine.add(MeasureStatement {
            name: "delay".to_string(),
            measure_type: MeasureType::Delay {
                trig: TrigSpec::new("V(in)", 0.5),
                targ: TrigSpec::new("V(out)", 0.5),
            },
            analysis: "TRAN".to_string(),
        });

        let results = engine.evaluate(&time, &signals);
        assert!(results[0].value.is_some());
        assert!((results[0].value.unwrap() - 0.10).abs() < 0.01);
    }

    #[test]
    fn test_find_at_time() {
        let (time, signal) = make_sine_data();
        let mut signals = HashMap::new();
        signals.insert("V(out)".to_string(), signal.as_slice());

        let mut engine = MeasureEngine::new();
        engine.add(MeasureStatement {
            name: "v_at_quarter".to_string(),
            measure_type: MeasureType::Find {
                signal: "V(out)".to_string(),
                at: Some(0.25), // sin(2π*0.25) = sin(π/2) = 1
                when_signal: None,
                when_value: None,
            },
            analysis: "TRAN".to_string(),
        });

        let results = engine.evaluate(&time, &signals);
        assert!(results[0].value.is_some());
        assert!((results[0].value.unwrap() - 1.0).abs() < 0.01);
    }
}
