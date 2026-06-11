//! Core analysis types
//!
//! DC, AC, and transient time-domain analyses with supporting infrastructure.

pub mod ac;
pub mod dc;
pub mod laplace;
pub mod temperature;
pub mod transient;

pub use ac::{AcAnalysis, AcResult};
pub use dc::{DcAnalysis, DcSweep};
pub use laplace::{DiscreteFilter, TransferFunction};
pub use temperature::{
    JunctionTempScaling, MosfetTempScaling, ResistorTempCoeffs, TemperatureContext,
};
pub use transient::{
    BreakpointManager, CompanionCoefficients, IntegrationMethod, LteEstimator, TimestepController,
    TrapGearController,
};
