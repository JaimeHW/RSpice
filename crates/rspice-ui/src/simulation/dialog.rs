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

// Core
pub(crate) mod op;

// Transfer/Stability
pub(crate) mod pz;
pub(crate) mod sens;
pub(crate) mod stb;

// Steady-state
pub(crate) mod hb;
pub(crate) mod pss;

// Periodic small-signal
pub(crate) mod pac;
pub(crate) mod pnoise;
pub(crate) mod pstb;
pub(crate) mod pxf;

// RF/Microwave
pub(crate) mod sp;

// Transfer function
pub(crate) mod xf;

// Statistical/Parametric
pub(crate) mod corner;
pub(crate) mod mc;

// Sweep
pub(crate) mod temp;

// Envelope/Fourier
pub(crate) mod envelope;
pub(crate) mod fourier;
pub(crate) mod optimization;
pub(crate) mod reliability;
pub(crate) mod soa;

// Options
pub(crate) mod options;

// Re-exports - Core. Only the operating point keeps a config type here, and it
// is the one the engine actually takes: `engine_bridge`, `execution`, and
// `multi_run` all pass `OpConfig` itself. AC, DC, noise, and transient had a
// second config apiece in this module -- their own `to_spice`, `validate`, and
// `total_points`, reachable from nothing but a `From` impl that nobody called.
// Execution takes the `simulation::config` types, so those four are gone.
pub use op::{
    OpAccuracy, OpAnnotation, OpConfig, OpDeviceDetail, OpDialogState, OpHomotopy, OpInitialGuess,
    OpNodeInitialization, OpPreviousState, OpRunPointContext, OpSaveDevice, OpTemperatureMode,
};

// Re-exports. Each analysis re-exports the dialog state its panel owns. The
// matching `*Config` types are deliberately absent: a dialog's config is its
// own business, and execution takes `simulation::config` types instead, so
// re-exporting both here only invited the two to be confused.
pub use hb::HbDialogState;
#[cfg(test)]
pub use pss::PssConfig;
pub use pss::{PssDialogState, PssSolverMethod};

// Re-exports - Periodic Small-Signal
pub use pac::PacDialogState;
pub use pnoise::{NoiseReferenceType, PnoiseDialogState};
pub use pstb::PstbDialogState;
pub use pxf::PxfDialogState;

// Re-exports - RF/Microwave
#[cfg(test)]
pub use sp::SpConfig;
pub use sp::SpDialogState;

// Re-exports - Transfer Function
pub use pz::PzDialogState;
pub use xf::{XfDialogState, XfNormalization};

// Re-exports - Stability/Sensitivity
pub use sens::SensDialogState;
pub use stb::{StbDialogState, StbProbeReference};

// Re-exports - Statistical/Parametric
pub use corner::CornerDialogState;
pub use mc::{McDialogState, McVariationSource};

// Re-exports - Temperature
pub use temp::TempDialogState;

// Re-exports - Envelope/Fourier
pub use envelope::EnvelopeDialogState;
pub use fourier::FourierDialogState;
pub use optimization::OptimizationDialogState;
pub use reliability::ReliabilityDialogState;
pub use soa::SoaDialogState;

// Re-exports - Framework
pub use options::{
    DampingStrategy, IntegrationMethod, MatrixSolver, OptionsDialogState, SimulationOptions,
    format_si_value, parse_si_value,
};
