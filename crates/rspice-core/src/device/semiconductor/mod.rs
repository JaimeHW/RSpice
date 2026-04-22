//! Two-terminal semiconductor device models
//!
//! Includes diodes and bipolar junction transistors (BJTs).

mod bjt;
mod diode;

pub(crate) use bjt::{
    BJT_DYNAMIC_CHARGE_COUNT, BJT_EXTERNAL_STATE_DIM, BJT_INTERNAL_STATE_DIM, BjtChargeBranch,
    BjtChargeSnapshot, VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT,
    VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX,
};
pub use bjt::{Bjt, BjtType};
pub use diode::Diode;
