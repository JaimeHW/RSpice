//! Advanced analysis types
//!
//! Specialized analyses: noise, Fourier, sensitivity, pole-zero, S-parameters, Monte Carlo,
//! PSS (Periodic Steady State), Harmonic Balance, and Phase Noise.

pub mod fourier;
pub mod harmonic_balance;
pub mod measure;
pub mod monte_carlo;
pub mod noise;
pub mod parametric;
pub mod pnoise;
pub mod pole_zero;
pub mod pss;
pub mod s_param;
pub mod sensitivity;
pub mod transfer;

pub use fourier::{FourierAnalysis, FourierConfig, FourierResult, HarmonicComponent};
pub use harmonic_balance::{
    FrequencyMap, HarmonicData, HbConfig, HbFft, HbResult, HbSolver, HbTone, MultiToneConfig,
    SpectralVoltage,
};
pub use measure::{
    EdgeType, MeasureEngine, MeasureResult, MeasureStatement, MeasureType, TrigSpec,
};
pub use monte_carlo::{
    Distribution, MonteCarloConfig, MonteCarloResult, MonteCarloRunner, Tolerance,
    VariableStatistics, VariationSet,
};
pub use noise::{IntegratedNoise, NoiseAnalysis, NoiseResult, NoiseSource, NoiseSourceType};
pub use parametric::{ParametricResults, ParametricSweep, StepSpec, StepTarget, StepType};
pub use pole_zero::{Matrix as PzMatrix, PoleZeroAnalyzer, PoleZeroConfig, PoleZeroResult};
pub use s_param::{
    FrequencySweep, Port, SMatrix, SParameterAnalyzer, SParameterConfig, SParameterResult,
};
pub use sensitivity::{
    ElementDesc, ElementType, Sensitivity, SensitivityAnalyzer, SensitivityResult,
};
pub use transfer::{TransferAnalyzer, TransferFunctionConfig, TransferFunctionResult};
