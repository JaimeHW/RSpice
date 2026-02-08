//! Simulation Results and Waveform Data
//!
//! Containers for simulation results that bridge rspice-core outputs
//! to the waveform viewer and other UI components.

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
#[derive(Debug, Clone)]
pub struct DcOpResult {
    /// Node voltages
    pub node_voltages: HashMap<String, f64>,

    /// Branch currents
    pub branch_currents: HashMap<String, f64>,

    /// Device operating points
    pub device_ops: HashMap<String, DeviceOpPoint>,
}

impl Default for DcOpResult {
    fn default() -> Self {
        Self {
            node_voltages: HashMap::new(),
            branch_currents: HashMap::new(),
            device_ops: HashMap::new(),
        }
    }
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

    /// Empty placeholder result (for unimplemented analyses)
    Empty {
        /// Associated measurements
        measurements: HashMap<String, f64>,
    },
}

impl Default for SimulationResult {
    fn default() -> Self {
        Self::Empty {
            measurements: HashMap::new(),
        }
    }
}

impl SimulationResult {
    /// Get all measurements associated with this result
    pub fn measurements(&self) -> HashMap<String, f64> {
        match self {
            SimulationResult::Empty { measurements } => measurements.clone(),
            _ => HashMap::new(),
        }
    }
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
            SimulationResult::Empty { .. } => false,
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
            SimulationResult::Empty { .. } => "Empty",
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waveform_data_time_domain() {
        let wf =
            WaveformData::new_time_domain("V(out)", vec![0.0, 1e-9, 2e-9], vec![0.0, 0.5, 1.0]);
        assert_eq!(wf.name, "V(out)");
        assert_eq!(wf.len(), 3);
        assert!(!wf.is_empty());
        assert_eq!(wf.x_unit, "s");
        assert_eq!(wf.y_unit, "V");
    }

    #[test]
    fn test_waveform_data_freq_domain() {
        let wf = WaveformData::new_freq_domain(
            "dB(V(out))",
            vec![1e3, 1e6, 1e9],
            vec![0.0, -3.0, -20.0],
        );
        assert_eq!(wf.x_unit, "Hz");
        assert_eq!(wf.y_unit, "dB");
    }

    #[test]
    fn test_waveform_data_complex() {
        let wf =
            WaveformData::new_complex("H(s)", vec![1e3, 1e6], vec![1.0, 0.707], vec![0.0, -0.707]);
        assert!(wf.is_complex);
        assert!(wf.y_imag.is_some());
    }

    #[test]
    fn test_waveform_magnitude() {
        let wf = WaveformData::new_complex("test", vec![1.0], vec![3.0], vec![4.0]);
        let mag = wf.magnitude();
        assert!((mag[0] - 5.0).abs() < 1e-10); // 3-4-5 triangle
    }

    #[test]
    fn test_waveform_magnitude_db() {
        let wf = WaveformData::new_complex("test", vec![1.0], vec![1.0], vec![0.0]);
        let db = wf.magnitude_db();
        assert!((db[0] - 0.0).abs() < 1e-10); // 20*log10(1) = 0
    }

    #[test]
    fn test_waveform_phase() {
        let wf = WaveformData::new_complex("test", vec![1.0], vec![0.0], vec![1.0]);
        let phase = wf.phase_deg().unwrap();
        assert!((phase[0] - 90.0).abs() < 1e-10); // atan2(1, 0) = 90°
    }

    #[test]
    fn test_waveform_x_range() {
        let wf = WaveformData::new_time_domain("test", vec![0.0, 1.0, 2.0], vec![0.0, 0.5, 1.0]);
        assert_eq!(wf.x_range(), Some((0.0, 2.0)));
    }

    #[test]
    fn test_waveform_y_range() {
        let wf = WaveformData::new_time_domain("test", vec![0.0, 1.0, 2.0], vec![-1.0, 0.5, 2.0]);
        assert_eq!(wf.y_range(), Some((-1.0, 2.0)));
    }

    #[test]
    fn test_dc_op_result() {
        let mut result = DcOpResult::default();
        result.node_voltages.insert("out".to_string(), 2.5);
        result.branch_currents.insert("I(R1)".to_string(), 1e-3);

        assert_eq!(result.voltage("out"), Some(2.5));
        assert_eq!(result.current("I(R1)"), Some(1e-3));
        assert_eq!(result.voltage("nonexistent"), None);
    }

    #[test]
    fn test_simulation_result_transient() {
        let mut waveforms = HashMap::new();
        waveforms.insert(
            "V(out)".to_string(),
            WaveformData::new_time_domain("V(out)", vec![0.0, 1.0], vec![0.0, 1.0]),
        );

        let result = SimulationResult::Transient {
            time: vec![0.0, 1.0],
            waveforms,
        };

        assert!(result.has_data());
        assert_eq!(result.type_name(), "Transient");
        assert_eq!(result.waveform_names(), vec!["V(out)"]);
        assert!(result.get_waveform("V(out)").is_some());
    }

    #[test]
    fn test_simulation_result_pole_zero() {
        let result = SimulationResult::PoleZero {
            poles: vec![(-1e6, 0.0), (-1e5, 1e5)],
            zeros: vec![],
            gain: 1.0,
        };

        assert!(result.has_data());
        assert_eq!(result.type_name(), "Pole-Zero");
    }

    #[test]
    fn test_simulation_result_no_data() {
        let result = SimulationResult::Transient {
            time: vec![],
            waveforms: HashMap::new(),
        };

        assert!(!result.has_data());
    }

    #[test]
    fn test_waveform_with_y_unit() {
        let wf = WaveformData::new_time_domain("I(R1)", vec![0.0], vec![1e-3]).with_y_unit("A");
        assert_eq!(wf.y_unit, "A");
    }

    #[test]
    fn test_simulation_result_monte_carlo_waveform_names() {
        let result = SimulationResult::MonteCarlo {
            runs_requested: 10,
            runs_completed: 10,
            num_failures: 0,
            all_converged: true,
            variables: vec![MonteCarloVariableResult {
                name: "V(out)".to_string(),
                mean: 1.0,
                std_dev: 0.01,
                min: 0.95,
                max: 1.05,
                histogram: vec![1, 4, 5],
                bin_edges: vec![0.9, 0.97, 1.03, 1.1],
            }],
        };

        assert!(result.has_data());
        assert_eq!(result.type_name(), "Monte Carlo");
        assert_eq!(result.waveform_names(), vec!["V(out)"]);
    }

    #[test]
    fn test_simulation_result_parametric_waveforms() {
        let mut waveforms = HashMap::new();
        waveforms.insert(
            "V(out)".to_string(),
            WaveformData::new_time_domain("V(out)", vec![-40.0, 25.0, 85.0], vec![1.1, 1.0, 0.9]),
        );
        let result = SimulationResult::Parametric {
            target: "TEMP".to_string(),
            sweep_values: vec![-40.0, 25.0, 85.0],
            waveforms,
            num_failures: 0,
        };

        assert!(result.has_data());
        assert_eq!(result.type_name(), "Parametric");
        assert!(result.get_waveform("V(out)").is_some());
    }

    #[test]
    fn test_simulation_result_corner_waveforms() {
        let mut waveforms = HashMap::new();
        waveforms.insert(
            "V(out)".to_string(),
            WaveformData::new_time_domain("V(out)", vec![-40.0, 25.0, 125.0], vec![1.2, 1.0, 0.8]),
        );
        let result = SimulationResult::Corner {
            x_values: vec![-40.0, 25.0, 125.0],
            x_label: "Temperature".to_string(),
            x_unit: "C".to_string(),
            temperatures_c: vec![-40.0, 25.0, 125.0],
            corner_labels: vec![
                "TT_1.000000V_-40.000000C".to_string(),
                "TT_1.000000V_25.000000C".to_string(),
                "TT_1.000000V_125.000000C".to_string(),
            ],
            waveforms,
            num_failures: 0,
        };

        assert!(result.has_data());
        assert_eq!(result.type_name(), "Corner");
        assert!(result.get_waveform("V(out)").is_some());
    }
}
