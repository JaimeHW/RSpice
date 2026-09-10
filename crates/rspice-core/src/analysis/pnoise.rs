//! Phase-noise spectral result types shared with result documents and frontends.
//!
//! Circuit-derived noise is evaluated by the engine using the physical device
//! equations and an authenticated periodic operating point. Driven conversion
//! noise uses [`crate::Engine::run_pnoise`]; oscillator phase noise uses
//! [`crate::Engine::run_pnoise_oscillator`]. Their retained-state variants reuse
//! a previously solved PSS or HB orbit. Authored `.PNOISE` cards use
//! [`crate::netlist::PnoiseCard`].
//!
//! Floquet stability evidence is shared by PSS and PSTB through
//! [`crate::analysis::FloquetSpectrumEvidence`]. This module does not contain
//! a separate waveform-based approximation of the phase response or charge.

mod result;

pub use result::{NoiseContributor, PhaseNoisePoint, PnoiseResult};
