//! Harmonic Balance Analysis Module
//!
//! This module provides Harmonic Balance (HB) analysis for
//! finding periodic steady-state solutions in the frequency domain.
//!
//! # Overview
//!
//! Harmonic Balance solves for the Fourier coefficients of node voltages directly,
//! handling nonlinear circuits by iterating between time and frequency domains:
//!
//! 1. Convert spectral coefficients to time-domain waveform (IFFT)
//! 2. Evaluate nonlinear device characteristics in time domain
//! 3. Convert back to frequency domain (FFT)
//! 4. Compute residual and update via Newton iteration
//!
//! # Key Features
//!
//! - **Multi-tone analysis**: Handle circuits with multiple incommensurate frequencies
//! - **Oversampling**: Anti-aliasing via FFT zero-padding
//! - **Sparse Jacobian**: Block-diagonal structure for efficiency
//! - **Krylov solvers**: Support for large circuits

mod config;
mod fft;
mod result;
mod solver;

pub use config::{HbConfig, HbConfigError, HbTone};
pub use fft::HbFft;
pub use result::{
    HbContinuationLimitation, HbPhaseProjectionError, HbPhaseState, HbReactiveKind,
    HbReactivePhaseState, HbReactiveSpectrum, HbResult, SpectralBranchCurrent, SpectralVoltage,
};
#[cfg(test)]
pub(crate) use solver::DC_SHORT_CONDUCTANCE;
pub use solver::{
    DepletionCap, HbError, HbSolver, HbSolverState, HbSwitchNodes, HbVoltageSwitchModel,
    NonlinearDeviceInstance, NonlinearDeviceParams, NonlinearDeviceType, PeriodicAcExcitation,
    PeriodicFlickerNoise, PeriodicNoiseSource,
};
pub(crate) use solver::{
    ExactPeriodicNetwork, HbDcSeedPolicy, PeriodicNoiseOutput, PeriodicNoiseProjection,
    PeriodicSidebandWindow,
};
pub(crate) use solver::{ScaledNonnegative, normalize_scaled_noise_waveform};
