//! Simulation Dialogs Module
//!
//! Commercial-grade simulation configuration dialogs organized per Cadence Spectre.
//!
//! # Analysis Categories
//! - **Core**: Transient, AC, DC, Noise, DC OP
//! - **Steady-State**: PSS, HB
//! - **Periodic Small-Signal**: PAC, PNoise, PXF, PSTB
//! - **RF/Microwave**: S-Parameters
//! - **Transfer Functions**: XF, Pole-Zero
//! - **Statistical/Parametric**: Monte Carlo, Corner
//! - **Stability**: STB, Sensitivity
//! - **Sweep**: Temperature
//! - **Post-Processing**: Fourier, Envelope

// RF/Microwave
pub(crate) mod sp;

// Statistical/Parametric
pub(crate) mod mc;

// Envelope/Fourier
pub(crate) mod envelope;
pub(crate) mod optimization;
pub(crate) mod soa;

// Options
pub(crate) mod options;

#[cfg(test)]
pub use rspice_simulation_contract::config::OpHomotopy;
#[cfg(test)]
pub use rspice_simulation_contract::config::{OpConfig, OpInitialGuess, OpNodeInitialization};
pub use rspice_simulation_contract::op_draft::OpDialogState;

// Re-exports. Each analysis re-exports the dialog state its panel owns. The
// matching `*Config` types are deliberately absent: a dialog's config is its
// own business, and execution takes `simulation::config` types instead, so
// re-exporting both here only invited the two to be confused.
pub use rspice_simulation_contract::hb_draft::HbDialogState;
#[cfg(test)]
pub use rspice_simulation_contract::pss_draft::PssConfig;
pub use rspice_simulation_contract::pss_draft::PssDialogState;

// Re-exports - Periodic Small-Signal
pub use rspice_simulation_contract::pac_draft::PacDialogState;
pub use rspice_simulation_contract::pnoise_draft::{NoiseReferenceType, PnoiseDialogState};
pub use rspice_simulation_contract::pstb_draft::PstbDialogState;
pub use rspice_simulation_contract::pxf_draft::PxfDialogState;

// Re-exports - RF/Microwave
#[cfg(test)]
pub use sp::SpConfig;
#[cfg(test)]
pub use sp::TOUCHSTONE_VERSIONS;
pub use sp::{SpDialogState, SpPortSource, TOUCHSTONE_VERSION_LABELS};

// Re-exports - Transfer Function
pub use rspice_simulation_contract::pz_draft::PzDialogState;
pub use rspice_simulation_contract::xf_draft::XfDialogState;

// Re-exports - Stability/Sensitivity
pub use rspice_simulation_contract::sens_draft::SensDialogState;
pub use rspice_simulation_contract::stb_draft::{StbDialogState, StbProbeReference};

// Re-exports - Statistical/Parametric
pub use mc::McDialogState;
#[cfg(test)]
pub use mc::McVariationSource;

// Re-exports - Temperature

// Re-exports - Envelope/Fourier
pub use envelope::EnvelopeDialogState;
pub use optimization::OptimizationDialogState;
pub use rspice_simulation_contract::fourier_draft::FourierDialogState;
pub use soa::SoaDialogState;

// Re-exports - Framework
pub use options::{
    DampingStrategy, HbTimeDomainMode, IntegrationMethod, MatrixSolver, OptionsDialogState,
    SimulationCompatibility, SimulationOptions, format_si_value, parse_si_value,
};
