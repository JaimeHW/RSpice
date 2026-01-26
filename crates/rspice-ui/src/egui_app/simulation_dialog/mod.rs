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

// Framework
pub mod framework;
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

// Re-exports - Framework
pub use framework::{DialogResult, DialogTab, SimulationDialog};
pub use options::{
    DampingStrategy, IntegrationMethod, MatrixSolver, OptionsDialogState, SimulationOptions,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_exports() {
        let _ = (
            TransientConfig::default(),
            AcConfig::default(),
            DcConfig::default(),
            NoiseConfig::default(),
            OpConfig::default(),
        );
    }
    #[test]
    fn test_steady_state_exports() {
        let _ = (HbConfig::default(), PssConfig::default());
    }
    #[test]
    fn test_periodic_exports() {
        let _ = (
            PacConfig::default(),
            PnoiseConfig::default(),
            PxfConfig::default(),
            PstbConfig::default(),
        );
    }
    #[test]
    fn test_rf_exports() {
        let _ = SpConfig::default();
    }
    #[test]
    fn test_xf_exports() {
        let _ = XfConfig::default();
    }
    #[test]
    fn test_pz_exports() {
        let _ = PzConfig::default();
    }
    #[test]
    fn test_sens_exports() {
        let _ = SensConfig::default();
    }
    #[test]
    fn test_stb_exports() {
        let _ = StbConfig::default();
    }
    #[test]
    fn test_mc_exports() {
        let _ = McConfig::default();
    }
    #[test]
    fn test_temp_exports() {
        let _ = TempConfig::default();
    }
    #[test]
    fn test_statistical_exports() {
        let _ = CornerConfig::default();
    }
    #[test]
    fn test_envelope_fourier_exports() {
        let _ = (EnvelopeConfig::default(), FourierConfig::default());
    }

    #[test]
    fn test_all_analysis_validate() {
        assert!(HbConfig::default().validate().is_ok());
        assert!(SpConfig::default().validate().is_ok());
        assert!(PacConfig::default().validate().is_ok());
        assert!(PnoiseConfig::default().validate().is_ok());
        assert!(PxfConfig::default().validate().is_ok());
        assert!(PstbConfig::default().validate().is_ok());
        assert!(CornerConfig::default().validate().is_ok());
        assert!(EnvelopeConfig::default().validate().is_ok());
        assert!(FourierConfig::default().validate().is_ok());
        assert!(XfConfig::default().validate().is_ok());
        assert!(OpConfig::default().validate().is_ok());
        assert!(PzConfig::default().validate().is_ok());
        assert!(SensConfig::default().validate().is_ok());
        assert!(StbConfig::default().validate().is_ok());
        assert!(McConfig::default().validate().is_ok());
        assert!(TempConfig::default().validate().is_ok());
        assert!(PssConfig::default().validate().is_ok());
    }
}
