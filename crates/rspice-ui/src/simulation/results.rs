//! Simulation Results and Waveform Data
//!
//! Containers for simulation results that bridge rspice-core outputs
//! to the waveform viewer and other UI components.

use crate::services::safety::SoAViolation;
use crate::simulation::reliability_engine::ReliabilityResult;
use std::collections::HashMap;

//=============================================================================
// Waveform Data
//=============================================================================

/// Time-domain waveform data for a single signal
#[derive(Debug, Clone)]
pub struct WaveformData {
    /// Signal name (e.g., "V(out)", "I(R1)")
    pub name: String,

    /// X-axis values (time for transient, frequency for AC)
    pub x_values: Vec<f64>,

    /// Y-axis values (voltage, current, etc.)
    pub y_values: Vec<f64>,

    /// Unit for Y-axis (V, A, dB, degrees, etc.)
    pub y_unit: String,

    /// Unit for X-axis (s, Hz, etc.)
    pub x_unit: String,

    /// Whether this is a complex result (for AC analysis)
    pub is_complex: bool,

    /// Imaginary part (for AC analysis)
    pub y_imag: Option<Vec<f64>>,
}

impl WaveformData {
    /// Create new time-domain waveform
    pub fn new_time_domain(name: impl Into<String>, time: Vec<f64>, values: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            x_values: time,
            y_values: values,
            y_unit: "V".to_string(),
            x_unit: "s".to_string(),
            is_complex: false,
            y_imag: None,
        }
    }

    /// Create new frequency-domain waveform (magnitude only)
    pub fn new_freq_domain(name: impl Into<String>, freq: Vec<f64>, magnitude: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            x_values: freq,
            y_values: magnitude,
            y_unit: "dB".to_string(),
            x_unit: "Hz".to_string(),
            is_complex: false,
            y_imag: None,
        }
    }

    /// Create new complex frequency-domain waveform
    pub fn new_complex(
        name: impl Into<String>,
        freq: Vec<f64>,
        real: Vec<f64>,
        imag: Vec<f64>,
    ) -> Self {
        Self {
            name: name.into(),
            x_values: freq,
            y_values: real,
            y_unit: "".to_string(),
            x_unit: "Hz".to_string(),
            is_complex: true,
            y_imag: Some(imag),
        }
    }

    /// Get number of data points
    pub fn len(&self) -> usize {
        self.x_values.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.x_values.is_empty()
    }

    /// Get magnitude (for complex data)
    pub fn magnitude(&self) -> Vec<f64> {
        if let Some(ref imag) = self.y_imag {
            self.y_values
                .iter()
                .zip(imag.iter())
                .map(|(r, i)| (r * r + i * i).sqrt())
                .collect()
        } else {
            self.y_values.clone()
        }
    }

    /// Get magnitude in dB (for complex data)
    pub fn magnitude_db(&self) -> Vec<f64> {
        self.magnitude().iter().map(|m| 20.0 * m.log10()).collect()
    }

    /// Get phase in degrees (for complex data)
    pub fn phase_deg(&self) -> Option<Vec<f64>> {
        self.y_imag.as_ref().map(|imag| {
            self.y_values
                .iter()
                .zip(imag.iter())
                .map(|(r, i)| i.atan2(*r).to_degrees())
                .collect()
        })
    }

    /// Get X-axis range
    pub fn x_range(&self) -> Option<(f64, f64)> {
        if self.x_values.is_empty() {
            return None;
        }
        let min = self.x_values.iter().copied().fold(f64::INFINITY, f64::min);
        let max = self
            .x_values
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        Some((min, max))
    }

    /// Get Y-axis range
    pub fn y_range(&self) -> Option<(f64, f64)> {
        if self.y_values.is_empty() {
            return None;
        }
        let min = self.y_values.iter().copied().fold(f64::INFINITY, f64::min);
        let max = self
            .y_values
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        Some((min, max))
    }

    /// Set Y unit
    pub fn with_y_unit(mut self, unit: impl Into<String>) -> Self {
        self.y_unit = unit.into();
        self
    }
}

//=============================================================================
// DC Operating Point Result
//=============================================================================

/// DC operating point result
#[derive(Debug, Clone, Default)]
pub struct DcOpResult {
    /// Node voltages
    pub node_voltages: HashMap<String, f64>,

    /// Branch currents
    pub branch_currents: HashMap<String, f64>,

    /// Device operating points
    pub device_ops: HashMap<String, DeviceOpPoint>,
}

impl DcOpResult {
    /// Get voltage at a node
    pub fn voltage(&self, node: &str) -> Option<f64> {
        self.node_voltages.get(node).copied()
    }

    /// Get current through a branch
    pub fn current(&self, branch: &str) -> Option<f64> {
        self.branch_currents.get(branch).copied()
    }
}

/// Operating point data for a device
#[derive(Debug, Clone)]
pub struct DeviceOpPoint {
    /// Device type (R, C, M, Q, etc.)
    pub device_type: String,

    /// Operating point parameters
    pub parameters: HashMap<String, f64>,
}

/// Monte Carlo variable summary in UI-friendly form.
#[derive(Debug, Clone)]
pub struct MonteCarloVariableResult {
    /// Variable name (e.g., V(out), I(V1))
    pub name: String,
    /// Arithmetic mean over converged runs
    pub mean: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Minimum observed value
    pub min: f64,
    /// Maximum observed value
    pub max: f64,
    /// Histogram counts for post-processing/visualization
    pub histogram: Vec<usize>,
    /// Histogram bin edges (length = histogram.len() + 1)
    pub bin_edges: Vec<f64>,
}

//=============================================================================
// Simulation Result Container
//=============================================================================

/// Container for all simulation results
#[derive(Debug, Clone)]
pub enum SimulationResult {
    /// DC operating point
    DcOp(DcOpResult),

    /// DC sweep results
    DcSweep {
        /// Sweep variable name
        sweep_var: String,
        /// Sweep values
        sweep_values: Vec<f64>,
        /// Waveforms indexed by signal name
        waveforms: HashMap<String, WaveformData>,
    },

    /// Transient analysis results
    Transient {
        /// Time vector
        time: Vec<f64>,
        /// Waveforms indexed by signal name
        waveforms: HashMap<String, WaveformData>,
    },

    /// AC analysis results
    Ac {
        /// Frequency vector
        frequencies: Vec<f64>,
        /// Complex waveforms indexed by signal name
        waveforms: HashMap<String, WaveformData>,
    },

    /// Noise analysis results
    Noise {
        /// Frequency vector
        frequencies: Vec<f64>,
        /// Output noise spectral density (V²/Hz or A²/Hz)
        output_noise: Vec<f64>,
        /// Input-referred noise (optional)
        input_noise: Option<Vec<f64>>,
        /// Noise contributors by source
        contributors: HashMap<String, Vec<f64>>,
    },

    /// Pole-zero analysis results
    PoleZero {
        /// Poles (complex values)
        poles: Vec<(f64, f64)>,
        /// Zeros (complex values)
        zeros: Vec<(f64, f64)>,
        /// Transfer function gain
        gain: f64,
    },

    /// Sensitivity analysis results
    Sensitivity {
        /// Parameter sensitivities
        sensitivities: HashMap<String, f64>,
        /// Normalized sensitivities (% change in output / % change in param)
        normalized: HashMap<String, f64>,
    },

    /// Monte Carlo statistical analysis results.
    MonteCarlo {
        /// Number of samples requested by .MC
        runs_requested: usize,
        /// Number of converged runs completed
        runs_completed: usize,
        /// Number of failed/non-converged runs
        num_failures: usize,
        /// Whether all runs converged according to engine summary
        all_converged: bool,
        /// Per-variable statistical summaries
        variables: Vec<MonteCarloVariableResult>,
    },

    /// Parametric sweep result (including .STEP TEMP).
    Parametric {
        /// Sweep target label (e.g., PARAM rload, TEMP)
        target: String,
        /// Sweep values in execution order
        sweep_values: Vec<f64>,
        /// Waveforms indexed by signal name
        waveforms: HashMap<String, WaveformData>,
        /// Number of failed points (if any)
        num_failures: usize,
    },

    /// Corner sweep result.
    Corner {
        /// X-axis values in execution order.
        x_values: Vec<f64>,
        /// X-axis label (e.g. Temperature, Corner Index).
        x_label: String,
        /// X-axis unit (e.g. C for temperature).
        x_unit: String,
        /// Corner temperatures (Celsius) in execution order.
        temperatures_c: Vec<f64>,
        /// Corner labels in execution order (e.g. `TT_1.000000V_25.000000C`).
        corner_labels: Vec<String>,
        /// Waveforms indexed by signal name
        waveforms: HashMap<String, WaveformData>,
        /// Number of failed corners
        num_failures: usize,
    },

    /// Reliability aging analysis result.
    Reliability {
        /// Lifetime checkpoints in years.
        years: Vec<f64>,
        /// Waveforms indexed by signal name.
        waveforms: HashMap<String, WaveformData>,
        /// Structured per-device reliability outputs.
        device_results: Vec<ReliabilityResult>,
    },

    /// Optimization analysis result.
    Optimization {
        /// Iteration axis values.
        iterations: Vec<f64>,
        /// Waveforms indexed by signal name.
        waveforms: HashMap<String, WaveformData>,
        /// Best cost reached.
        best_cost: f64,
        /// Best variable values.
        best_variables: HashMap<String, f64>,
        /// Whether convergence criterion was met.
        converged: bool,
    },

    /// Safety / SOA analysis result.
    Soa {
        /// Time axis for SOA checks.
        time: Vec<f64>,
        /// Waveforms indexed by signal name.
        waveforms: HashMap<String, WaveformData>,
        /// Collected SOA violations.
        violations: Vec<SoAViolation>,
    },

    /// Scalar-only result payload for analyses that report measurements
    /// without generating waveform families.
    MeasurementsOnly {
        /// Associated measurements
        measurements: HashMap<String, f64>,
    },
}

impl Default for SimulationResult {
    fn default() -> Self {
        Self::MeasurementsOnly {
            measurements: HashMap::new(),
        }
    }
}

impl SimulationResult {
    /// Get a single scalar measurement by name without allocating a map.
    pub fn measurement(&self, name: &str) -> Option<f64> {
        let key = name.trim();
        if key.is_empty() {
            return None;
        }

        match self {
            SimulationResult::DcOp(op) => measurement_from_dc_op(op, key),
            SimulationResult::DcSweep { waveforms, .. }
            | SimulationResult::Transient { waveforms, .. }
            | SimulationResult::Ac { waveforms, .. }
            | SimulationResult::Parametric { waveforms, .. }
            | SimulationResult::Corner { waveforms, .. }
            | SimulationResult::Reliability { waveforms, .. }
            | SimulationResult::Optimization { waveforms, .. }
            | SimulationResult::Soa { waveforms, .. } => {
                waveform_last_value_by_name(waveforms, key)
            }
            SimulationResult::Noise {
                output_noise,
                input_noise,
                contributors,
                ..
            } => {
                if key.eq_ignore_ascii_case("output_noise")
                    || key.eq_ignore_ascii_case("onoise_total")
                {
                    return output_noise.last().copied();
                }
                if key.eq_ignore_ascii_case("input_noise")
                    || key.eq_ignore_ascii_case("inoise_total")
                {
                    return input_noise.as_ref().and_then(|vals| vals.last().copied());
                }
                contributors.get(key).and_then(|vals| vals.last().copied())
            }
            SimulationResult::PoleZero { poles, zeros, gain } => {
                if key.eq_ignore_ascii_case("gain") {
                    return Some(*gain);
                }
                if key.eq_ignore_ascii_case("num_poles") {
                    return Some(poles.len() as f64);
                }
                if key.eq_ignore_ascii_case("num_zeros") {
                    return Some(zeros.len() as f64);
                }
                None
            }
            SimulationResult::Sensitivity {
                sensitivities,
                normalized,
            } => sensitivities
                .get(key)
                .copied()
                .or_else(|| normalized.get(key).copied()),
            SimulationResult::MonteCarlo { variables, .. } => {
                if let Some(var) = variables.iter().find(|var| var.name == key) {
                    return Some(var.mean);
                }

                parse_wrapped_identifier(key, "mean")
                    .and_then(|inner| variables.iter().find(|var| var.name == inner))
                    .map(|var| var.mean)
            }
            SimulationResult::MeasurementsOnly { measurements } => measurements.get(key).copied(),
        }
    }

    /// Get all measurements associated with this result
    pub fn measurements(&self) -> HashMap<String, f64> {
        match self {
            SimulationResult::DcOp(op) => {
                let mut out = HashMap::with_capacity(
                    op.node_voltages.len().saturating_mul(2) + op.branch_currents.len(),
                );
                for (node, value) in &op.node_voltages {
                    out.insert(node.clone(), *value);
                    out.insert(format!("V({})", node), *value);
                }
                for (branch, value) in &op.branch_currents {
                    out.insert(branch.clone(), *value);
                    if !branch.starts_with("I(") {
                        out.insert(format!("I({})", branch), *value);
                    }
                }
                out
            }
            SimulationResult::DcSweep { waveforms, .. }
            | SimulationResult::Transient { waveforms, .. }
            | SimulationResult::Ac { waveforms, .. }
            | SimulationResult::Parametric { waveforms, .. }
            | SimulationResult::Corner { waveforms, .. }
            | SimulationResult::Reliability { waveforms, .. }
            | SimulationResult::Optimization { waveforms, .. }
            | SimulationResult::Soa { waveforms, .. } => waveforms
                .iter()
                .filter_map(|(name, wf)| {
                    wf.y_values
                        .last()
                        .copied()
                        .map(|value| (name.clone(), value))
                })
                .collect(),
            SimulationResult::Noise {
                output_noise,
                input_noise,
                contributors,
                ..
            } => {
                let mut out = HashMap::new();
                if let Some(v) = output_noise.last().copied() {
                    out.insert("output_noise".to_string(), v);
                    out.insert("onoise_total".to_string(), v);
                }
                if let Some(v) = input_noise.as_ref().and_then(|vals| vals.last().copied()) {
                    out.insert("input_noise".to_string(), v);
                    out.insert("inoise_total".to_string(), v);
                }
                for (name, vals) in contributors {
                    if let Some(v) = vals.last().copied() {
                        out.insert(name.clone(), v);
                    }
                }
                out
            }
            SimulationResult::PoleZero { poles, zeros, gain } => HashMap::from([
                ("gain".to_string(), *gain),
                ("num_poles".to_string(), poles.len() as f64),
                ("num_zeros".to_string(), zeros.len() as f64),
            ]),
            SimulationResult::Sensitivity {
                sensitivities,
                normalized,
            } => {
                let mut out = sensitivities.clone();
                for (name, value) in normalized {
                    out.insert(format!("normalized:{}", name), *value);
                }
                out
            }
            SimulationResult::MonteCarlo { variables, .. } => variables
                .iter()
                .map(|var| (var.name.clone(), var.mean))
                .collect(),
            SimulationResult::MeasurementsOnly { measurements } => measurements.clone(),
        }
    }
}

fn parse_wrapped_identifier<'a>(key: &'a str, prefix: &str) -> Option<&'a str> {
    if key.len() <= prefix.len() + 2 {
        return None;
    }
    if !key[..prefix.len()].eq_ignore_ascii_case(prefix) {
        return None;
    }
    if !key[prefix.len()..].starts_with('(') || !key.ends_with(')') {
        return None;
    }
    Some(&key[prefix.len() + 1..key.len() - 1])
}

fn measurement_from_dc_op(op: &DcOpResult, key: &str) -> Option<f64> {
    if let Some(v) = op.node_voltages.get(key).copied() {
        return Some(v);
    }
    if let Some(v) = op.branch_currents.get(key).copied() {
        return Some(v);
    }
    if let Some(node) = parse_wrapped_identifier(key, "V") {
        return op.node_voltages.get(node).copied();
    }
    if let Some(branch) = parse_wrapped_identifier(key, "I") {
        return op
            .branch_currents
            .get(branch)
            .copied()
            .or_else(|| op.branch_currents.get(key).copied());
    }
    None
}

fn waveform_last_value_by_name(
    waveforms: &HashMap<String, WaveformData>,
    key: &str,
) -> Option<f64> {
    if let Some(v) = waveforms
        .get(key)
        .and_then(|wf| wf.y_values.last().copied())
    {
        return Some(v);
    }

    if let Some(inner) =
        parse_wrapped_identifier(key, "V").or_else(|| parse_wrapped_identifier(key, "I"))
        && let Some(v) = waveforms
            .get(inner)
            .and_then(|wf| wf.y_values.last().copied())
    {
        return Some(v);
    }

    let voltage_key = format!("V({})", key);
    if let Some(v) = waveforms
        .get(&voltage_key)
        .and_then(|wf| wf.y_values.last().copied())
    {
        return Some(v);
    }

    let current_key = format!("I({})", key);
    waveforms
        .get(&current_key)
        .and_then(|wf| wf.y_values.last().copied())
}

impl SimulationResult {
    /// Get all waveform names
    pub fn waveform_names(&self) -> Vec<&str> {
        match self {
            SimulationResult::DcSweep { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::Transient { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::Ac { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::Parametric { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::Corner { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::Reliability { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::Optimization { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::Soa { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::MonteCarlo { variables, .. } => {
                variables.iter().map(|v| v.name.as_str()).collect()
            }
            _ => vec![],
        }
    }

    /// Get a specific waveform by name
    pub fn get_waveform(&self, name: &str) -> Option<&WaveformData> {
        match self {
            SimulationResult::DcSweep { waveforms, .. } => waveforms.get(name),
            SimulationResult::Transient { waveforms, .. } => waveforms.get(name),
            SimulationResult::Ac { waveforms, .. } => waveforms.get(name),
            SimulationResult::Parametric { waveforms, .. } => waveforms.get(name),
            SimulationResult::Corner { waveforms, .. } => waveforms.get(name),
            SimulationResult::Reliability { waveforms, .. } => waveforms.get(name),
            SimulationResult::Optimization { waveforms, .. } => waveforms.get(name),
            SimulationResult::Soa { waveforms, .. } => waveforms.get(name),
            _ => None,
        }
    }

    /// Check if this is a valid result with data
    pub fn has_data(&self) -> bool {
        match self {
            SimulationResult::DcOp(op) => !op.node_voltages.is_empty(),
            SimulationResult::DcSweep { waveforms, .. } => !waveforms.is_empty(),
            SimulationResult::Transient { time, .. } => !time.is_empty(),
            SimulationResult::Ac { frequencies, .. } => !frequencies.is_empty(),
            SimulationResult::Noise { frequencies, .. } => !frequencies.is_empty(),
            SimulationResult::PoleZero { poles, zeros, .. } => {
                !poles.is_empty() || !zeros.is_empty()
            }
            SimulationResult::Sensitivity { sensitivities, .. } => !sensitivities.is_empty(),
            SimulationResult::MonteCarlo {
                runs_completed,
                variables,
                ..
            } => *runs_completed > 0 || !variables.is_empty(),
            SimulationResult::Parametric { sweep_values, .. } => !sweep_values.is_empty(),
            SimulationResult::Corner { x_values, .. } => !x_values.is_empty(),
            SimulationResult::Reliability {
                years, waveforms, ..
            } => !years.is_empty() && !waveforms.is_empty(),
            SimulationResult::Optimization {
                iterations,
                waveforms,
                ..
            } => !iterations.is_empty() && !waveforms.is_empty(),
            SimulationResult::Soa {
                time, waveforms, ..
            } => !time.is_empty() && !waveforms.is_empty(),
            SimulationResult::MeasurementsOnly { .. } => false,
        }
    }

    /// Get display name for the result type
    pub fn type_name(&self) -> &'static str {
        match self {
            SimulationResult::DcOp(_) => "DC Operating Point",
            SimulationResult::DcSweep { .. } => "DC Sweep",
            SimulationResult::Transient { .. } => "Transient",
            SimulationResult::Ac { .. } => "AC Analysis",
            SimulationResult::Noise { .. } => "Noise Analysis",
            SimulationResult::PoleZero { .. } => "Pole-Zero",
            SimulationResult::Sensitivity { .. } => "Sensitivity",
            SimulationResult::MonteCarlo { .. } => "Monte Carlo",
            SimulationResult::Parametric { .. } => "Parametric",
            SimulationResult::Corner { .. } => "Corner",
            SimulationResult::Reliability { .. } => "Reliability",
            SimulationResult::Optimization { .. } => "Optimization",
            SimulationResult::Soa { .. } => "Safety (SOA)",
            SimulationResult::MeasurementsOnly { .. } => "Measurements Only",
        }
    }
}

//=============================================================================
// Tests
//=============================================================================
