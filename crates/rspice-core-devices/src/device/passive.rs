//! Passive component device models
//!
//! Includes capacitors, inductors, and coupled inductors.

mod capacitor;
mod coupled_inductors;
mod inductor;
mod jiles_atherton;
mod saturable_inductor;

pub use capacitor::{
    Capacitor, SolutionDependentCapacitor, SolutionDependentCapacitorLinearization,
};
#[doc(hidden)]
pub use coupled_inductors::CoupledWinding;
pub use coupled_inductors::{CoupledInductorPair, InductorCoupling, MultiWindingTransformer};
pub use inductor::Inductor;
pub use jiles_atherton::{JilesAthertonInductor, JilesAthertonParams};
#[doc(hidden)]
pub use jiles_atherton::{XyceCoreStep, XyceCoreTrial};
pub use saturable_inductor::SaturableInductor;
