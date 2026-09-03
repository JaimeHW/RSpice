//! DC operating-point and sweep results.
//!
//! `SimulationResult` is the single-point operating solution and `DcSweepResult`
//! a swept sequence of them. `.STEP` and `.TEMP` runs produce the same shape and
//! reuse `DcSweepResult` rather than duplicating it. `DeviceOperatingPoint` is
//! the per-device small-signal projection taken at an operating point.
//!
//! The two live in sibling modules because they are two result families that
//! happen to share a point type, not one family with two views: a sweep owns a
//! coordinate axis, a nesting shape, and its own export columns.

use super::*;

mod operating_point;
mod sweep;

pub(crate) use operating_point::{PyDeviceOperatingPoint, PySimulationResult};
pub(crate) use sweep::PyDcSweepResult;
