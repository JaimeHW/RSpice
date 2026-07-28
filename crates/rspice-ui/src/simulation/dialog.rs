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

// Re-exports - Steady-State
pub use hb::{HbConfig, HbDialogState, HbSolverType, HbToneConfig};
pub use pss::{PssConfig, PssDialogState, PssSolverMethod};

// Re-exports - Periodic Small-Signal
pub use pac::{PacConfig, PacDialogState, PacSweepType};
pub use pnoise::{NoiseReferenceType, PnoiseConfig, PnoiseDialogState, PnoiseSweepType};
pub use pstb::{PstbConfig, PstbDialogState};
pub use pxf::{PxfConfig, PxfDialogState, PxfSweepType};

// Re-exports - RF/Microwave
pub use sp::{SpConfig, SpDialogState, SpPortConfig, SpSweepType};

// Re-exports - Transfer Function
pub use pz::{PzAnalysisType, PzConfig, PzDialogState, PzTransferType};
pub use xf::{XfAccuracy, XfConfig, XfDialogState, XfNormalization};

// Re-exports - Stability/Sensitivity
pub use sens::{SensConfig, SensDialogState, SensType};
pub use stb::{StbConfig, StbDialogState};

// Re-exports - Statistical/Parametric
pub use corner::{CornerBaseAnalysis, CornerConfig, CornerDialogState, ProcessCorner};
pub use mc::{McBaseAnalysis, McConfig, McDialogState, McDistribution};

// Re-exports - Temperature
pub use temp::{TempBaseAnalysis, TempConfig, TempDialogState};

// Re-exports - Envelope/Fourier
pub use envelope::{EnvelopeConfig, EnvelopeDialogState};
pub use fourier::{FourierConfig, FourierDialogState};
pub use optimization::{
    OptimizationAlgorithmMode, OptimizationConfig, OptimizationDialogState, OptimizationGoalMode,
    OptimizationVariableConfig,
};
pub use reliability::{ReliabilityConfig, ReliabilityDialogState};
pub use soa::{SoaConfig, SoaDialogState};

// Re-exports - Framework
pub use options::{
    DampingStrategy, IntegrationMethod, MatrixSolver, OptionsDialogState, SimulationOptions,
};
