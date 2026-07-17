//! Advanced analysis types
//!
//! Specialized analyses: noise, Fourier, sensitivity, pole-zero, S-parameters, Monte Carlo,
//! PSS (Periodic Steady State), Harmonic Balance, Phase Noise,
//! PSTB (Periodic Stability), PXF (Periodic Transfer Function), and Corner Analysis.

pub mod corner;
pub mod distortion;
pub mod fourier;
pub mod harmonic_balance;
pub mod measure;
mod measure_file;
pub mod measure_signals;
pub mod monte_carlo;
pub mod noise;
pub mod pac;
pub mod parametric;
pub mod pnoise;
pub mod pole_zero;
pub mod pss;
pub mod pstb;
pub mod pxf;
pub mod s_param;
pub mod sensitivity;
pub mod stb;
pub mod transfer;

pub use corner::{
    CornerConfig, CornerPoint, CornerResult, CornerRunner, CornerSimResult, CornerSummary,
    ProcessCorner,
};
pub use distortion::{
    DistortionAnalysisResult, DistortionPointResult, DistortionProduct, DistortionProductResult,
};
pub use fourier::{FourierAnalysis, FourierConfig, FourierResult, HarmonicComponent};
pub use harmonic_balance::{
    FrequencyMap, HarmonicData, HbConfig, HbFft, HbResult, HbSolver, HbTone, MultiToneConfig,
    SpectralVoltage,
};
pub use measure::{
    ContinuousMeasureFailureMetadata, ContinuousMeasureRecord, ContinuousMeasureResult, EdgeType,
    ErrorFunctionNorm, EventOccurrence, ExtremaOutput, FileErrorNorm, MeasureEngine,
    MeasureOperand, MeasurePrintPolicy, MeasureResult, MeasureStatement, MeasureType, TrigSpec,
    TriggerEvent, WhenCondition,
};
pub use measure_signals::{
    AcSweepSeries, DcSweepSeries, EquationMeasureTrace, NoiseSweepSeries,
    evaluate_ac_continuous_measurements, evaluate_ac_equation_measurements,
    evaluate_ac_measurements, evaluate_dc_continuous_measurements,
    evaluate_dc_continuous_measurements_with_parameter_contexts, evaluate_dc_equation_measurements,
    evaluate_dc_measurements, evaluate_dc_measurements_with_parameter_contexts,
    evaluate_noise_continuous_measurements, evaluate_noise_equation_measurements,
    evaluate_noise_measurements, evaluate_tran_continuous_measurements,
    evaluate_tran_equation_measurements, evaluate_tran_measurements, measurements_for_analysis,
    transient_signal_map, unevaluated_measurements,
};
pub use monte_carlo::{
    Distribution, MonteCarloConfig, MonteCarloResult, MonteCarloRunner, Tolerance,
    VariableStatistics, VariationSet,
};
pub use noise::{
    IntegratedContribution, IntegratedNoise, NoiseAnalysis, NoiseContribution,
    NoiseContributionKind, NoiseContributionProbe, NoiseContributionProbeError, NoiseResult,
    NoiseSource, NoiseSourceIdentity, NoiseSourceType, PortNoiseCorrelationResult,
};
pub use pac::{ConversionMatrix, PacConfig, PacError, PacResult, PacSweepType};
pub use parametric::{ParametricResults, ParametricSweep, StepSpec, StepTarget, StepType};
pub use pole_zero::{Matrix as PzMatrix, PoleZeroAnalyzer, PoleZeroConfig, PoleZeroResult};
pub use pstb::{FloquetMultiplier, PstbAnalyzer, PstbConfig, PstbResult, StabilityType};
pub use pxf::{PxfAnalyzer, PxfConfig, PxfError, PxfResult, PxfSweepType, TransferPoint};
pub use s_param::{
    FrequencySweep, Port, SMatrix, SParameterAnalyzer, SParameterConfig, SParameterResult,
};
pub use sensitivity::{
    AcSensitivity, AcSensitivityOutput, AcSensitivityResult, ElementDesc, ElementType, Sensitivity,
    SensitivityAnalyzer, SensitivityResult,
};
pub use stb::{
    BodePoint, NyquistPoint, StabilityMargins, StbAnalyzer, StbConfig, StbResult, StbSweepType,
};
pub use transfer::{TransferAnalyzer, TransferFunctionConfig, TransferFunctionResult};
