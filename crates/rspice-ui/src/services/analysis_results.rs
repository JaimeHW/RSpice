//! Analysis Results
//!
//! Unified result enum for all core engine analysis outputs.
//! Individual data types are defined in simulation_runner.rs for execution,
//! and other unique types are defined here for post-processing.

use num_complex::Complex64;
use rspice_core::Value;

// Import types from simulation_runner (using crate path for public access)
use crate::services::simulation_runner::{
    AcData, DcSweepData, HbData, NoiseData, PssData, StbData, TransientData,
};

// =============================================================================
// DC Operating Point
// =============================================================================

/// DC operating point result data
#[derive(Debug, Clone)]
pub struct DcOpData {
    /// Node voltages: (node_name, voltage)
    pub node_voltages: Vec<(String, Value)>,
    /// Branch currents: (branch_name, current)
    pub branch_currents: Vec<(String, Value)>,
}

// =============================================================================
// PAC (Periodic AC)
// =============================================================================

/// PAC analysis result data
#[derive(Debug, Clone)]
pub struct PacData {
    /// Frequency offset points
    pub frequencies: Vec<Value>,
    /// Sideband index (0 = fundamental)
    pub sideband: i32,
    /// Complex responses: (node_name, values)
    pub responses: Vec<(String, Vec<Complex64>)>,
}

// =============================================================================
// Monte Carlo
// =============================================================================

/// Monte Carlo analysis result data
#[derive(Debug, Clone)]
pub struct MonteCarloData {
    /// Number of runs completed
    pub num_runs: usize,
    /// Parameter names that were varied
    pub varied_params: Vec<String>,
    /// Output statistics: (output_name, mean, std_dev, min, max)
    pub statistics: Vec<(String, Value, Value, Value, Value)>,
    /// Histogram bins: (output_name, bin_edges, counts)
    pub histograms: Vec<(String, Vec<Value>, Vec<usize>)>,
}

// =============================================================================
// Pole-Zero
// =============================================================================

/// Pole-Zero analysis result data
#[derive(Debug, Clone)]
pub struct PoleZeroData {
    /// Poles: (real, imag) pairs
    pub poles: Vec<(Value, Value)>,
    /// Zeros: (real, imag) pairs  
    pub zeros: Vec<(Value, Value)>,
    /// DC gain
    pub dc_gain: Value,
}

// =============================================================================
// Sensitivity
// =============================================================================

/// Sensitivity analysis result data
#[derive(Debug, Clone)]
pub struct SensitivityData {
    /// Output variable name
    pub output_name: String,
    /// Sensitivities: (parameter_name, sensitivity_value, normalized_sensitivity)
    pub sensitivities: Vec<(String, Value, Value)>,
}

// =============================================================================
// Fourier/THD
// =============================================================================

/// Fourier/THD analysis result data
#[derive(Debug, Clone)]
pub struct FourierData {
    /// Fundamental frequency
    pub fundamental_freq: Value,
    /// Output signal name
    pub output_name: String,
    /// DC component
    pub dc_component: Value,
    /// Harmonic components: (harmonic_number, frequency, magnitude, phase_deg, magnitude_normalized)
    pub harmonics: Vec<(usize, Value, Value, Value, Value)>,
    /// Total Harmonic Distortion (%)
    pub thd_percent: Value,
}

// =============================================================================
// Transfer Function
// =============================================================================

/// DC Transfer function result data
#[derive(Debug, Clone)]
pub struct TransferData {
    /// Output variable
    pub output: String,
    /// Input source
    pub input: String,
    /// Transfer function value (V/V or A/A)
    pub gain: Value,
    /// Input resistance
    pub input_resistance: Value,
    /// Output resistance
    pub output_resistance: Value,
}

// =============================================================================
// Parametric Sweep
// =============================================================================

/// Parametric sweep result data
#[derive(Debug, Clone)]
pub struct ParametricData {
    /// Parameter name being swept
    pub param_name: String,
    /// Parameter values
    pub param_values: Vec<Value>,
    /// Results at each parameter value: (output_name, values)
    pub outputs: Vec<(String, Vec<Value>)>,
}

// =============================================================================
// Corner Analysis
// =============================================================================

/// Corner analysis result data
#[derive(Debug, Clone)]
pub struct CornerData {
    /// Corner names (e.g., "TT", "FF", "SS", "SF", "FS")
    pub corner_names: Vec<String>,
    /// Results at each corner: (output_name, corner_values)
    pub outputs: Vec<(String, Vec<Value>)>,
}

// =============================================================================
// Unified Result Enum
// =============================================================================

/// Unified analysis result wrapper
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AnalysisResult {
    DcOp(DcOpData),
    DcSweep(DcSweepData),
    Transient(TransientData),
    Ac(AcData),
    Noise(NoiseData),
    Pss(PssData),
    Pac(PacData),
    HarmonicBalance(HbData),
    Stb(StbData),
    MonteCarlo(MonteCarloData),
    PoleZero(PoleZeroData),
    Sensitivity(SensitivityData),
    Fourier(FourierData),
    Transfer(TransferData),
    Parametric(ParametricData),
    Corner(CornerData),
}

impl AnalysisResult {
    /// Get the analysis type name
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::DcOp(_) => "DC Operating Point",
            Self::DcSweep(_) => "DC Sweep",
            Self::Transient(_) => "Transient",
            Self::Ac(_) => "AC Analysis",
            Self::Noise(_) => "Noise",
            Self::Pss(_) => "PSS",
            Self::Pac(_) => "PAC",
            Self::HarmonicBalance(_) => "Harmonic Balance",
            Self::Stb(_) => "STB",
            Self::MonteCarlo(_) => "Monte Carlo",
            Self::PoleZero(_) => "Pole-Zero",
            Self::Sensitivity(_) => "Sensitivity",
            Self::Fourier(_) => "Fourier/THD",
            Self::Transfer(_) => "Transfer Function",
            Self::Parametric(_) => "Parametric",
            Self::Corner(_) => "Corner",
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

