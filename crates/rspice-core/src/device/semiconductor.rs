//! Two-terminal semiconductor device models
//!
//! Includes diodes and bipolar junction transistors (BJTs).

mod bjt;
mod diode;
mod limiting;

pub(crate) use bjt::{
    AcceptedBjtChargeSnapshotCheckpoint, AcceptedBjtNonlinearCheckpoint,
    BJT_ACCEPTED_CHARGE_SNAPSHOT_STATE_VALUE_COUNT, BJT_ACCEPTED_NONLINEAR_RUNTIME_TAG,
    BJT_ACCEPTED_NONLINEAR_STATE_VALUE_COUNT, BJT_DYNAMIC_CHARGE_COUNT, BJT_EXTERNAL_STATE_DIM,
    BJT_INTERNAL_STATE_DIM, BjtChargeBranch, BjtChargeSnapshot, BjtCurrentBranch,
    VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT,
};
pub use bjt::{Bjt, BjtType};
pub(crate) use diode::{
    AcceptedDiodeNonlinearCheckpoint, DIODE_ACCEPTED_NONLINEAR_RUNTIME_TAG, DiodeNonlinearState,
};
pub use diode::{Diode, DiodeLevel};
