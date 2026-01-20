//! Phase Noise (PNoise) Analysis Module
//!
//! This module provides commercial-grade phase noise analysis for periodic circuits.
//! PNoise measures the spectral purity of oscillators by quantifying noise power
//! at offset frequencies from the carrier.
//!
//! # Overview
//!
//! Phase noise analysis is critical for RF/MW design, particularly for:
//! - Voltage-Controlled Oscillators (VCOs)
//! - Phase-Locked Loops (PLLs)
//! - Crystal oscillators
//! - RF synthesizers
//!
//! # Algorithm
//!
//! 1. **Periodic steady-state**: Start from PSS or HB solution
//! 2. **Floquet analysis**: Compute time-varying transfer functions
//! 3. **Noise projection**: Project device noise onto amplitude/phase
//! 4. **Spectral integration**: Compute phase noise spectral density
//!
//! # Output Format
//!
//! Results are typically expressed in dBc/Hz (decibels relative to carrier
//! per Hz bandwidth) at various offset frequencies from the carrier.

mod config;
mod floquet;
mod result;
mod solver;

#[cfg(test)]
mod tests;

pub use config::{NoiseOutputNode, PnoiseConfig, PnoiseSideband, PnoiseSweep};
pub use floquet::{FloquetAnalyzer, FloquetMode, TransferFunction};
pub use result::{NoiseContributor, PhaseNoisePoint, PnoiseResult};
pub use solver::{PnoiseSolver, PnoiseState};
