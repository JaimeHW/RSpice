//! Result Mapper
//!
//! Maps simulation output from rspice-core to UI data structures.
//! Handles waveform conversion, operating point extraction, and analysis results.
//!
//! # Features
//!
//! - Waveform data conversion
//! - Operating point extraction
//! - AC/noise data with complex numbers
//! - Measurement extraction
//! - Corner result aggregation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Mapped Results
// =============================================================================

/// Mapped simulation result ready for UI consumption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappedResult {
    /// Analysis type
    pub analysis_type: MappedAnalysisType,
    /// Status
    pub status: ResultStatus,
    /// Waveforms (for sweep/transient/AC)
    pub waveforms: Vec<MappedWaveform>,
    /// Operating point data
    pub op_data: Option<OperatingPointMap>,
    /// Measurements
    pub measurements: Vec<MappedMeasurement>,
    /// Warnings
    pub warnings: Vec<String>,
    /// Simulation time (seconds)
    pub elapsed_time: f64,
    /// Corner name (if applicable)
    pub corner: Option<String>,
}

/// Analysis type for mapped result
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MappedAnalysisType {
    /// DC operating point
    DcOp,
    /// DC sweep
    DcSweep,
    /// Transient
    Transient,
    /// AC analysis
    Ac,
    /// Noise analysis
    Noise,
    /// Transfer function
    Tf,
    /// Sensitivity analysis
    Sensitivity,
    /// Pole-Zero
    PoleZero,
    /// S-Parameter
    SParameter,
    /// Harmonic Balance
    HarmonicBalance,
    /// Periodic Steady-State
    Pss,
    /// Periodic AC
    Pac,
    /// Periodic transfer function
    Pxf,
    /// Stability analysis
    Stb,
    /// Periodic stability analysis
    Pstb,
    /// Periodic Noise
    Pnoise,
    /// Monte Carlo
    MonteCarlo,
    /// Parametric sweep
    Parametric,
    /// Corner analysis
    Corner,
    /// Unknown
    #[default]
    Unknown,
}

/// Result status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ResultStatus {
    /// Successful
    #[default]
    Success,
    /// Completed with warnings
    Warning,
    /// Failed to converge
    ConvergenceFailure,
    /// Error during simulation
    Error,
}

impl Default for MappedResult {
    fn default() -> Self {
        Self {
            analysis_type: MappedAnalysisType::Unknown,
            status: ResultStatus::Success,
            waveforms: Vec::new(),
            op_data: None,
            measurements: Vec::new(),
            warnings: Vec::new(),
            elapsed_time: 0.0,
            corner: None,
        }
    }
}

// =============================================================================
// Waveform Mapping
// =============================================================================

/// Mapped waveform for UI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappedWaveform {
    /// Signal name
    pub name: String,
    /// X values (time, frequency, voltage, etc.)
    pub x: Vec<f64>,
    /// Y values (real)
    pub y: Vec<f64>,
    /// Y imaginary values (for AC)
    pub y_imag: Option<Vec<f64>>,
    /// X axis label
    pub x_label: String,
    /// Y axis label
    pub y_label: String,
    /// X axis unit
    pub x_unit: String,
    /// Y axis unit
    pub y_unit: String,
    /// Is complex data
    pub is_complex: bool,
}

impl Default for MappedWaveform {
    fn default() -> Self {
        Self {
            name: String::new(),
            x: Vec::new(),
            y: Vec::new(),
            y_imag: None,
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            x_unit: "".to_string(),
            y_unit: "".to_string(),
            is_complex: false,
        }
    }
}

impl MappedWaveform {
    /// Create time-domain waveform
    pub fn time_domain(name: impl Into<String>, time: Vec<f64>, values: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            x: time,
            y: values,
            x_label: "Time".to_string(),
            x_unit: "s".to_string(),
            ..Default::default()
        }
    }

    /// Create frequency-domain waveform
    pub fn frequency_domain(
        name: impl Into<String>,
        freq: Vec<f64>,
        values: Vec<f64>,
        y_label: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            x: freq,
            y: values,
            x_label: "Frequency".to_string(),
            x_unit: "Hz".to_string(),
            y_label: y_label.into(),
            ..Default::default()
        }
    }

    /// Create complex frequency response
    pub fn complex_ac(
        name: impl Into<String>,
        freq: Vec<f64>,
        real: Vec<f64>,
        imag: Vec<f64>,
    ) -> Self {
        Self {
            name: name.into(),
            x: freq,
            y: real,
            y_imag: Some(imag),
            x_label: "Frequency".to_string(),
            x_unit: "Hz".to_string(),
            is_complex: true,
            ..Default::default()
        }
    }

    /// Get magnitude (for AC)
    pub fn magnitude(&self) -> Vec<f64> {
        if let Some(ref imag) = self.y_imag {
            self.y
                .iter()
                .zip(imag.iter())
                .map(|(r, i)| (r * r + i * i).sqrt())
                .collect()
        } else {
            self.y.clone()
        }
    }

    /// Get magnitude in dB
    pub fn magnitude_db(&self) -> Vec<f64> {
        self.magnitude().iter().map(|m| 20.0 * m.log10()).collect()
    }

    /// Get phase in degrees
    pub fn phase_deg(&self) -> Vec<f64> {
        if let Some(ref imag) = self.y_imag {
            self.y
                .iter()
                .zip(imag.iter())
                .map(|(r, i)| i.atan2(*r).to_degrees())
                .collect()
        } else {
            vec![0.0; self.y.len()]
        }
    }

    /// Get number of points
    pub fn len(&self) -> usize {
        self.x.len()
    }

    /// Is empty
    pub fn is_empty(&self) -> bool {
        self.x.is_empty()
    }

    /// Min/max Y values
    pub fn y_range(&self) -> (f64, f64) {
        let min = self.y.iter().copied().fold(f64::INFINITY, f64::min);
        let max = self.y.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        (min, max)
    }
}

// =============================================================================
// Operating Point
// =============================================================================

/// Operating point data map
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OperatingPointMap {
    /// Node voltages (name -> value)
    pub node_voltages: HashMap<String, f64>,
    /// Branch currents (name -> value)
    pub branch_currents: HashMap<String, f64>,
    /// Device operating points
    pub device_ops: HashMap<String, DeviceOpPoint>,
}

/// Device operating point
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceOpPoint {
    /// Device name
    pub name: String,
    /// Device type
    pub device_type: String,
    /// Operating region (for transistors)
    pub region: Option<String>,
    /// Parameters (gm, gds, vth, etc.)
    pub parameters: HashMap<String, f64>,
}

impl OperatingPointMap {
    /// Get node voltage
    pub fn voltage(&self, node: &str) -> Option<f64> {
        self.node_voltages.get(node).copied()
    }

    /// Get branch current
    pub fn current(&self, branch: &str) -> Option<f64> {
        self.branch_currents.get(branch).copied()
    }

    /// Get all node names
    pub fn node_names(&self) -> Vec<&str> {
        self.node_voltages.keys().map(|s| s.as_str()).collect()
    }

    /// Get device operating point
    pub fn device(&self, name: &str) -> Option<&DeviceOpPoint> {
        self.device_ops.get(name)
    }
}

// =============================================================================
// Measurement
// =============================================================================

/// A simulation measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappedMeasurement {
    /// Measurement name
    pub name: String,
    /// Measurement type
    pub meas_type: MeasurementType,
    /// Measured value
    pub value: f64,
    /// Unit
    pub unit: String,
    /// Source signal
    pub signal: String,
    /// Status
    pub status: MeasurementStatus,
}

/// Measurement type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeasurementType {
    /// Cross threshold measurement
    Cross,
    /// Rise time
    RiseTime,
    /// Fall time
    FallTime,
    /// Delay
    Delay,
    /// Maximum
    Max,
    /// Minimum
    Min,
    /// Average
    Average,
    /// RMS
    Rms,
    /// Peak-to-peak
    PeakToPeak,
    /// Period
    Period,
    /// Frequency
    Frequency,
    /// Integration
    Integral,
    /// Custom
    Custom,
}

/// Measurement status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MeasurementStatus {
    /// Successfully measured
    #[default]
    Success,
    /// Target not found
    NotFound,
    /// Invalid range
    InvalidRange,
    /// Error
    Error,
}

impl Default for MappedMeasurement {
    fn default() -> Self {
        Self {
            name: String::new(),
            meas_type: MeasurementType::Max,
            value: 0.0,
            unit: String::new(),
            signal: String::new(),
            status: MeasurementStatus::Success,
        }
    }
}

// =============================================================================
// Result Mapper
// =============================================================================

/// Maps engine results to UI format
#[derive(Debug, Clone, Default)]
pub struct ResultMapper;

impl ResultMapper {
    /// Create new mapper
    pub fn new() -> Self {
        Self
    }

    /// Map DC operating point result
    pub fn map_dc_op(&self, node_voltages: HashMap<String, f64>) -> MappedResult {
        let mut op_data = OperatingPointMap::default();
        op_data.node_voltages = node_voltages;

        MappedResult {
            analysis_type: MappedAnalysisType::DcOp,
            status: ResultStatus::Success,
            op_data: Some(op_data),
            ..Default::default()
        }
    }

    /// Map transient result
    pub fn map_transient(
        &self,
        time: Vec<f64>,
        signals: HashMap<String, Vec<f64>>,
    ) -> MappedResult {
        let waveforms = signals
            .into_iter()
            .map(|(name, values)| MappedWaveform::time_domain(name, time.clone(), values))
            .collect();

        MappedResult {
            analysis_type: MappedAnalysisType::Transient,
            status: ResultStatus::Success,
            waveforms,
            ..Default::default()
        }
    }

    /// Map AC result
    pub fn map_ac(
        &self,
        frequencies: Vec<f64>,
        signals: HashMap<String, (Vec<f64>, Vec<f64>)>,
    ) -> MappedResult {
        let waveforms = signals
            .into_iter()
            .map(|(name, (real, imag))| {
                MappedWaveform::complex_ac(name, frequencies.clone(), real, imag)
            })
            .collect();

        MappedResult {
            analysis_type: MappedAnalysisType::Ac,
            status: ResultStatus::Success,
            waveforms,
            ..Default::default()
        }
    }

    /// Add measurement to result
    pub fn add_measurement(&self, result: &mut MappedResult, meas: MappedMeasurement) {
        result.measurements.push(meas);
    }

    /// Set corner name
    pub fn set_corner(&self, result: &mut MappedResult, corner: impl Into<String>) {
        result.corner = Some(corner.into());
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // MappedWaveform Tests
    // =========================================================================

    #[test]
    fn test_waveform_time_domain() {
        let wf = MappedWaveform::time_domain("v(out)", vec![0.0, 1.0, 2.0], vec![0.0, 0.5, 1.0]);
        assert_eq!(wf.name, "v(out)");
        assert_eq!(wf.len(), 3);
        assert_eq!(wf.x_label, "Time");
    }

    #[test]
    fn test_waveform_complex_ac() {
        let wf = MappedWaveform::complex_ac(
            "v(out)",
            vec![1e3, 1e6],
            vec![1.0, 0.707],
            vec![0.0, -0.707],
        );
        assert!(wf.is_complex);
        assert!(wf.y_imag.is_some());
    }

    #[test]
    fn test_waveform_magnitude() {
        let wf = MappedWaveform::complex_ac("test", vec![1.0], vec![3.0], vec![4.0]);
        let mag = wf.magnitude();
        assert!((mag[0] - 5.0).abs() < 0.01); // 3-4-5 triangle
    }

    #[test]
    fn test_waveform_magnitude_db() {
        let wf = MappedWaveform::complex_ac("test", vec![1.0], vec![1.0], vec![0.0]);
        let db = wf.magnitude_db();
        assert!(db[0].abs() < 0.01); // 0 dB for magnitude 1
    }

    #[test]
    fn test_waveform_phase() {
        let wf = MappedWaveform::complex_ac("test", vec![1.0], vec![0.0], vec![1.0]);
        let phase = wf.phase_deg();
        assert!((phase[0] - 90.0).abs() < 0.01); // 90 degrees
    }

    #[test]
    fn test_waveform_y_range() {
        let wf = MappedWaveform::time_domain("test", vec![0.0, 1.0], vec![-1.0, 2.0]);
        let (min, max) = wf.y_range();
        assert_eq!(min, -1.0);
        assert_eq!(max, 2.0);
    }

    // =========================================================================
    // OperatingPointMap Tests
    // =========================================================================

    #[test]
    fn test_op_map_voltage() {
        let mut op = OperatingPointMap::default();
        op.node_voltages.insert("out".to_string(), 1.5);

        assert_eq!(op.voltage("out"), Some(1.5));
        assert_eq!(op.voltage("nonexistent"), None);
    }

    #[test]
    fn test_op_map_current() {
        let mut op = OperatingPointMap::default();
        op.branch_currents.insert("I(Vdd)".to_string(), 1e-3);

        assert_eq!(op.current("I(Vdd)"), Some(1e-3));
    }

    // =========================================================================
    // ResultMapper Tests
    // =========================================================================

    #[test]
    fn test_map_dc_op() {
        let mapper = ResultMapper::new();
        let mut voltages = HashMap::new();
        voltages.insert("out".to_string(), 0.9);

        let result = mapper.map_dc_op(voltages);
        assert_eq!(result.analysis_type, MappedAnalysisType::DcOp);
        assert!(result.op_data.is_some());
    }

    #[test]
    fn test_map_transient() {
        let mapper = ResultMapper::new();
        let time = vec![0.0, 1e-9, 2e-9];
        let mut signals = HashMap::new();
        signals.insert("v(out)".to_string(), vec![0.0, 0.5, 1.0]);

        let result = mapper.map_transient(time, signals);
        assert_eq!(result.analysis_type, MappedAnalysisType::Transient);
        assert_eq!(result.waveforms.len(), 1);
    }

    #[test]
    fn test_map_ac() {
        let mapper = ResultMapper::new();
        let freq = vec![1e3, 1e6, 1e9];
        let mut signals = HashMap::new();
        signals.insert(
            "v(out)".to_string(),
            (vec![1.0, 0.5, 0.1], vec![0.0, -0.5, -0.9]),
        );

        let result = mapper.map_ac(freq, signals);
        assert_eq!(result.analysis_type, MappedAnalysisType::Ac);
        assert!(result.waveforms[0].is_complex);
    }

    #[test]
    fn test_add_measurement() {
        let mapper = ResultMapper::new();
        let mut result = MappedResult::default();

        let meas = MappedMeasurement {
            name: "rise_time".to_string(),
            meas_type: MeasurementType::RiseTime,
            value: 1e-9,
            unit: "s".to_string(),
            ..Default::default()
        };

        mapper.add_measurement(&mut result, meas);
        assert_eq!(result.measurements.len(), 1);
    }
}
