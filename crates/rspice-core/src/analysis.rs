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

// One module per analysis, each a sibling.
//
// These used to be split across `core` and `advanced` submodules, both of
// which re-exported everything to this level anyway, so the directories were
// a hop that changed no path. The line they drew was not one the code
// enforced: `distortion` and `stb` are no less advanced than `noise`, and
// deciding which of the two to open was guesswork.
pub mod ac;
pub mod corner;
pub mod dc;
pub mod distortion;
pub mod floquet;
pub mod fourier;
pub mod harmonic_balance;
pub mod measure;
mod measure_file;
pub mod measure_signals;
pub mod measurements;
pub mod monte_carlo;
pub mod noise;
pub mod pac;
pub mod parametric;
pub mod pnoise;
pub mod pole_zero;
pub mod post_processing;
pub mod pss;
pub mod pstb;
pub mod pxf;
pub mod s_param;
pub mod sensitivity;
pub mod signal_integrity;
pub mod stb;
pub mod temperature;
pub mod transfer;

pub use floquet::{
    FLOQUET_UNIT_CIRCLE_BAND, FloquetOrbitKind, FloquetSpectrumCertificate,
    FloquetSpectrumEvidence, FloquetStabilityVerdict, classify_floquet_stability,
    select_autonomous_phase_mode,
};
pub use monte_carlo::{
    Distribution, MonteCarloConfig, MonteCarloResult, MonteCarloRunner, Tolerance,
    VariableStatistics, VariationSet,
};
pub use noise::{
    IntegratedContribution, IntegratedNoise, NOISE_MECHANISM_MAX_BYTES, NoiseContribution,
    NoiseContributionKind, NoiseContributionProbe, NoiseContributionProbeError, NoiseResult,
    NoiseSource, NoiseSourceIdentity, NoiseSourceType, PortNoiseCorrelationResult,
    is_persistable_noise_mechanism,
};
pub use pac::{ConversionMatrix, PacConfig, PacError, PacResult, PacSweepType};
pub use parametric::{ParametricResults, ParametricSweep, StepSpec, StepTarget, StepType};
pub use pole_zero::{
    Matrix as PzMatrix, PoleZeroAnalysisError, PoleZeroAnalyzer, PoleZeroConfig, PoleZeroResult,
    RootSetEvidence, SpectrumCertificate, StabilityVerdict,
};
pub use pss::{
    FloquetSpectrumError, PeriodDetector, PeriodEstimate, PeriodicWaveform, PssConfig, PssResult,
    PssStabilityVerdict, ShootingNewtonSolver, ShootingState,
};
pub use pstb::{FloquetMultiplier, PstbAnalyzer, PstbConfig, PstbResult, StabilityType};
pub use pxf::{PxfConfig, PxfError, PxfResult, PxfSweepType, TransferPoint};
pub use s_param::{
    NetworkError, Port, PortError, SMatrix, SParameterPort, SParameterResult, TwoPortNoise,
    collect_ports, derive_two_port_noise, invert_complex_matrix, s_from_y, set_excitations,
};
pub use sensitivity::{
    AcSensitivity, AcSensitivityOutput, AcSensitivityResult, ElementDesc, ElementType, Sensitivity,
    SensitivityAnalyzer, SensitivityResult,
};
pub use stb::{
    BodePoint, NyquistPoint, StabilityMargins, StbAnalyzer, StbConfig, StbResult, StbSweepType,
};
pub use temperature::{
    JunctionTempScaling, MosfetTempScaling, ResistorTempCoeffs, TemperatureContext,
};
pub use transfer::{TransferFunctionConfig, TransferFunctionResult};
pub use {
    ac::AcResult,
    corner::{
        CornerConfig, CornerPoint, CornerResult, CornerRunner, CornerSimResult, CornerSummary,
        ProcessCorner,
    },
    dc::{DcAnalysis, DcSweep},
    distortion::{
        DistortionAnalysisResult, DistortionPointResult, DistortionProduct, DistortionProductResult,
    },
    fourier::{FourierAnalysis, FourierConfig, FourierError, FourierResult, HarmonicComponent},
    harmonic_balance::{
        FrequencyIndex, FrequencyMap, HarmonicData, HbConfig, HbContinuationLimitation, HbError,
        HbFft, HbPhaseProjectionError, HbPhaseState, HbReactiveKind, HbReactivePhaseState,
        HbReactiveSpectrum, HbResult, HbSolver, HbSolverState, HbTone, MultiToneConfig,
        SpectralBranchCurrent, SpectralVoltage,
    },
    measure::{
        ContinuousMeasureFailureMetadata, ContinuousMeasureRecord, ContinuousMeasureResult,
        EdgeType, ErrorFunctionNorm, EventOccurrence, ExtremaOutput, FileErrorNorm, MeasureEngine,
        MeasureOperand, MeasurePrintPolicy, MeasureResult, MeasureStatement, MeasureType, TrigSpec,
        TriggerEvent, WhenCondition, canonical_measure_signal_name,
    },
    measure_signals::{
        AcSweepSeries, DcSweepSeries, EquationMeasureTrace, NoiseSweepSeries,
        evaluate_ac_continuous_measurements, evaluate_ac_equation_measurements,
        evaluate_ac_measurements, evaluate_dc_continuous_measurements,
        evaluate_dc_continuous_measurements_with_parameter_contexts,
        evaluate_dc_equation_measurements, evaluate_dc_measurements,
        evaluate_dc_measurements_with_parameter_contexts, evaluate_dc_output_requests_with_abort,
        evaluate_noise_continuous_measurements, evaluate_noise_equation_measurements,
        evaluate_noise_measurements, evaluate_tran_continuous_measurements,
        evaluate_tran_equation_measurements, evaluate_tran_four_output_requests_with_abort,
        evaluate_tran_measurements, evaluate_tran_output_requests_with_abort,
        evaluate_tran_remeasurements, measurements_for_analysis, transient_signal_map,
        unevaluated_measurements,
    },
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
