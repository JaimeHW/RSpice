//! Struct-of-Arrays device storage for circuit elements.
//!
//! These containers own the per-device state used by DC, AC, and transient
//! stamping. Keeping them here leaves `circuit::mod` focused on topology,
//! branch allocation, and whole-circuit orchestration.

use super::{NodeId, TwoTerminalStamp, project_two_terminal_voltage};
use crate::analysis::{CompanionCoefficients, IntegrationMethod};
use crate::device::{
    Bjt, Diode, Ekv3Device, EkvMosfet, MatrixStamper, Mosfet, NonlinearConvergenceCriteria, Vdmos,
};
use crate::solver::{CscIndex, StaticMatrix, TripletMatrix};
use crate::{Complex64, Value};
use std::sync::{Arc, OnceLock, RwLock};
mod inductors;
mod nonlinear;
mod passive;
mod sources;

pub use inductors::Inductors;
pub use nonlinear::{
    B3SoiDds, B3SoiFds, B3SoiPds, Bjts, Bsim3v3s, Bsim4v8s, Diodes, Ekv3Mosfets, EkvMosfets,
    Mosfets, Vdmoses,
};
pub use passive::{Capacitors, ResistorBranches, Resistors};
pub use sources::{CurrentSources, VoltageSources};
