//! Intrinsic BJT branch equations, limiting, and static linearization.

use super::*;

pub(in crate::device::semiconductor::bjt) use state_solve::BjtNodeVoltages;

mod api;
mod branches;
mod charge;
mod junction;
mod limiting;
mod state_solve;
