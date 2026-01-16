//! Analysis engines for circuit simulation
//!
//! Provides:
//! - DC operating point and sweep
//! - AC small-signal frequency response
//! - Transient time-domain simulation
//! - Noise analysis (thermal, shot, flicker)
//! - Fourier/THD analysis
//! - Parametric sweeps
//! - Temperature analysis
//! - Measurement statements
//! - Monte Carlo statistical analysis
//! - Transfer function analysis
//! - Sensitivity analysis
//! - S-parameter analysis
//! - Pole-zero analysis

mod dc;
pub mod ac;
pub mod transient;
pub mod noise;
pub mod parametric;
pub mod temperature;
pub mod fourier;
pub mod waveform;
pub mod measure;
pub mod monte_carlo;
pub mod transfer;
pub mod sensitivity;
pub mod s_param;
pub mod pole_zero;

pub use dc::{DcAnalysis, DcSweep};
pub use ac::{AcAnalysis, AcResult};
pub use transient::{TransientAnalysis, TimestepController, BreakpointManager, LteEstimator, TrapGearController};
pub use noise::{NoiseAnalysis, NoiseResult, NoiseSource, NoiseSourceType, IntegratedNoise};
pub use parametric::{ParametricSweep, StepSpec, StepType, StepTarget, ParametricResults};
pub use temperature::{TemperatureContext, ResistorTempCoeffs, JunctionTempScaling, MosfetTempScaling};
pub use fourier::{FourierAnalysis, FourierConfig, FourierResult, HarmonicComponent};
pub use waveform::{WaveformRecorder, CompressionConfig, TransientResultCompressed};
pub use measure::{MeasureEngine, MeasureStatement, MeasureType, MeasureResult, EdgeType, TrigSpec};
pub use monte_carlo::{MonteCarloConfig, MonteCarloRunner, MonteCarloResult, Distribution, Tolerance, VariableStatistics, VariationSet};
pub use transfer::{TransferFunctionResult, TransferFunctionConfig, TransferAnalyzer};
pub use sensitivity::{SensitivityResult, Sensitivity, SensitivityAnalyzer, ElementType, ElementDesc};
pub use s_param::{SParameterResult, SParameterConfig, SParameterAnalyzer, SMatrix, Port, FrequencySweep};
pub use pole_zero::{PoleZeroResult, PoleZeroConfig, PoleZeroAnalyzer, Matrix as PzMatrix};

use crate::Value;

/// Common analysis configuration
#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    /// Convergence tolerance
    pub tolerance: Value,
    /// Maximum iterations for nonlinear solve
    pub max_iterations: usize,
    /// Enable verbose output
    pub verbose: bool,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            tolerance: 1e-9,
            max_iterations: 50,
            verbose: false,
        }
    }
}
