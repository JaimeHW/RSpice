//! Numerical integration for the transient DAE.
//!
//! The integration method and the companion-model coefficients it produces
//! are the contract between the time-stepping loop and everything that
//! stamps a reactive element. A capacitor, an XSPICE code model and a
//! Verilog-A device all convert `C·dv/dt` into a conductance and an
//! equivalent source the same way, and they all need these coefficients.
//!
//! Around that contract sit the three decisions a variable-step integrator
//! makes each step: how large the step should be (`timestep`), which
//! discontinuities it must not step over (`breakpoint`), and whether the
//! step it just took was accurate enough to accept (`lte`).
//!
//! All of it used to live in `analysis::transient`, which put numerics
//! primitives above the circuit store, the device models and the XSPICE
//! subsystem that consume them — three layers reaching upward for a struct
//! describing how to discretize a derivative. Analysis *results* belong above
//! those layers; the discretization rule does not.

// Private: the four files divide the work, they do not divide the API. Every
// caller names `numerics::integration::Thing`, so a file can be split or
// merged without moving anything's path.
mod breakpoint;
mod companion;
mod lte;
mod timestep;

pub(crate) use breakpoint::XYCE_BREAKPOINT_TOLERANCE;
pub use breakpoint::{BreakpointManager, BreakpointStepPolicy};
pub(crate) use companion::parse_integration_method;
pub use companion::{CompanionCoefficients, IntegrationMethod};
pub(crate) use lte::LtePrefixWindow;
pub(crate) use lte::{
    ACCEPTED_BOUNDARY_LTE_ESTIMATOR_CHECKPOINT_VERSION, AcceptedBoundaryLteEstimatorCheckpoint,
};
pub use lte::{LteEstimator, TransientLteReference};
pub use timestep::{TimestepController, TransientErrorControl, TrapGearController};
pub(crate) use timestep::{
    TrapGearControllerSnapshot, XYCE_DEFAULT_MIN_TIME_STEPS_BREAKPOINT, XYCE_DEFAULT_NLMAX,
    XYCE_DEFAULT_NLMIN, XyceBreakpointSpanCeiling, xyce_iteration_step_accepts,
    xyce_iteration_step_scale,
};
