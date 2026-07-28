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
pub(crate) mod ac;
pub(crate) mod dc;
pub(crate) mod noise;
pub(crate) mod op;
pub(crate) mod transient;

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

// Re-exports - Core
pub use ac::AcConfig;
pub use dc::DcConfig;
pub use noise::NoiseConfig;
pub use op::{
    OpAccuracy, OpAnnotation, OpConfig, OpDeviceDetail, OpDialogState, OpHomotopy, OpInitialGuess,
    OpNodeInitialization, OpPreviousState, OpRunPointContext, OpSaveDevice, OpTemperatureMode,
};
pub use transient::TransientConfig;

// Re-exports. Each analysis re-exports the dialog state its panel owns. The
// matching `*Config` types are deliberately absent: a dialog's config is its
// own business, and execution takes `simulation::config` types instead, so
// re-exporting both here only invited the two to be confused.
pub use hb::{HbDialogState, HbSolverType};
pub use pss::{PssConfig, PssDialogState, PssSolverMethod};

// Re-exports - Periodic Small-Signal
pub use pac::PacDialogState;
pub use pnoise::{NoiseReferenceType, PnoiseDialogState};
pub use pstb::PstbDialogState;
pub use pxf::PxfDialogState;

// Re-exports - RF/Microwave
pub use sp::{SpConfig, SpDialogState};

// Re-exports - Transfer Function
pub use pz::PzDialogState;
pub use xf::{XfAccuracy, XfDialogState, XfNormalization};

// Re-exports - Stability/Sensitivity
pub use sens::{SensDialogState, SensType};
pub use stb::StbDialogState;

// Re-exports - Statistical/Parametric
pub use corner::CornerDialogState;
pub use mc::McDialogState;

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
};
