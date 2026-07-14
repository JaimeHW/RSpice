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

// Subdirectories
pub mod advanced;
mod core;
mod output;

// Waveform measurement module - Spectre/ADE-style calculator functions
pub mod measurements;

// Signal integrity analysis - eye diagrams, jitter analysis
pub mod signal_integrity;

// Post-processing utilities - THD, SFDR, IMD, group delay
pub mod post_processing;

// Re-export submodules for backwards-compatible paths (crate::analysis::ac etc)
pub use advanced::distortion;
pub use advanced::fourier;
pub use advanced::measure;
pub use advanced::monte_carlo;
pub use advanced::noise;
pub use advanced::parametric;
pub use advanced::pole_zero;
pub use advanced::s_param;
pub use advanced::sensitivity;
pub use advanced::transfer;
pub use core::ac;
pub use core::dc;
pub use core::laplace;
pub use core::temperature;
pub use core::transient;
pub use output::raw_export;
pub use output::waveform;
pub use output::waveform_stream;

// Re-export key types from core
pub use core::{
    AcAnalysis, AcResult, BreakpointManager, CompanionCoefficients, DcAnalysis, DcSweep,
    DiscreteFilter, IntegrationMethod, JunctionTempScaling, LteEstimator, MosfetTempScaling,
    ResistorTempCoeffs, TemperatureContext, TimestepController, TransferFunction,
    TrapGearController,
};

// Re-export key types from advanced
pub use advanced::{
    AcSensitivity, AcSensitivityOutput, AcSensitivityResult, ContinuousMeasureRecord,
    ContinuousMeasureResult, DistortionAnalysisResult, DistortionPointResult, DistortionProduct,
    DistortionProductResult, Distribution, EdgeType, ElementDesc, ElementType, ErrorFunctionNorm,
    EventOccurrence, ExtremaOutput, FileErrorNorm, FourierAnalysis, FourierConfig, FourierResult,
    FrequencySweep, HarmonicComponent, IntegratedContribution, IntegratedNoise, MeasureEngine,
    MeasureOperand, MeasurePrintPolicy, MeasureResult, MeasureStatement, MeasureType,
    MonteCarloConfig, MonteCarloResult, MonteCarloRunner, NoiseAnalysis, NoiseResult, NoiseSource,
    NoiseSourceType, ParametricResults, ParametricSweep, PoleZeroAnalyzer, PoleZeroConfig,
    PoleZeroResult, Port, PortNoiseCorrelationResult, PzMatrix, SMatrix, SParameterAnalyzer,
    SParameterConfig, SParameterResult, Sensitivity, SensitivityAnalyzer, SensitivityResult,
    StepSpec, StepTarget, StepType, Tolerance, TransferAnalyzer, TransferFunctionConfig,
    TransferFunctionResult, TrigSpec, TriggerEvent, VariableStatistics, VariationSet,
    WhenCondition,
};

// Re-export PSS types
pub use advanced::pss::{
    PeriodDetector, PeriodEstimate, PeriodicWaveform, PssConfig, PssResult, ShootingNewtonSolver,
    ShootingState,
};

// Re-export Harmonic Balance types
pub use advanced::harmonic_balance::{
    FrequencyIndex, FrequencyMap, HarmonicData, HbConfig, HbError, HbFft, HbResult, HbSolver,
    HbSolverState, HbTone, MultiToneConfig, SpectralVoltage,
};

// Re-export key types from output
pub use output::{
    CompressionConfig, RawExporter, RawFormat, RawVariable, StreamingWaveformReader,
    StreamingWaveformWriter, TransientResultCompressed, VariableType, WaveformRecorder,
    export_dc_sweep, export_transient,
};

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
