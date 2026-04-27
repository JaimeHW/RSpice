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
//!
//! # Example
//!
//! ```ignore
//! use rspice_core::analysis::harmonic_balance::{HbConfig, HbSolver};
//!
//! let config = HbConfig::new(1e9)  // 1 GHz fundamental
//!     .with_harmonics(9)
//!     .with_tolerance(1e-6);
//!
//! let result = engine.run_hb(&netlist, config)?;
//! println!("Converged in {} iterations", result.iterations);
//! ```

mod config;
mod fft;
mod multi_tone;
pub mod nonlinear;
mod result;
mod solver;

pub use config::{HbConfig, HbTone};
pub use fft::HbFft;
pub use multi_tone::{FrequencyIndex, FrequencyMap, MultiToneConfig};
pub use nonlinear::{HbBjt, HbBjtType, HbDevice, HbDeviceCollection, HbDiode, HbMosType, HbMosfet};
pub use result::{HarmonicData, HbResult, SpectralVoltage};
pub use solver::{HbError, HbSolver, HbSolverState};
