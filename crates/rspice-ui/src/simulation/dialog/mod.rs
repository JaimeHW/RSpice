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
pub mod ac;
pub mod dc;
pub mod noise;
pub mod op;
pub mod transient;

// Transfer/Stability
pub mod pz;
pub mod sens;
pub mod stb;

// Steady-state
pub mod hb;
pub mod pss;

// Periodic small-signal
pub mod pac;
pub mod pnoise;
pub mod pstb;
pub mod pxf;

// RF/Microwave
pub mod sp;

// Transfer function
pub mod xf;

// Statistical/Parametric
pub mod corner;
pub mod mc;

// Sweep
pub mod temp;

// Envelope/Fourier
pub mod envelope;
pub mod fourier;
pub mod optimization;
pub mod reliability;
pub mod soa;

// Options
pub mod options;

// Re-exports - Core
pub use ac::AcConfig;
pub use dc::DcConfig;
pub use noise::NoiseConfig;
pub use op::{OpConfig, OpDialogState};
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
pub use xf::{XfConfig, XfDialogState, XfSweepType};

// Re-exports - Stability/Sensitivity
pub use sens::{SensConfig, SensDialogState, SensType};
pub use stb::{StbConfig, StbDialogState};

// Re-exports - Statistical/Parametric
pub use corner::{CornerBaseAnalysis, CornerConfig, CornerDialogState, ProcessCorner};
pub use mc::{McBaseAnalysis, McConfig, McDialogState, McDistribution};

// Re-exports - Temperature
pub use temp::{TempBaseAnalysis, TempConfig, TempDialogState};

// Re-exports - Envelope/Fourier
pub use envelope::{EnvelopeConfig, EnvelopeDialogState, ModulationType};
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
