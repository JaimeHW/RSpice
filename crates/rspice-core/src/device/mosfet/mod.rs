//! MOSFET and FET device models
//!
//! Includes various MOSFET models (Level 1-3, BSIM4, EKV, VDMOS) and JFETs.
//! Also contains shared utilities for smooth region transitions.

mod bsim4;
mod ekv;
mod jfet;
mod mos_models;
mod mosfet;
mod smooth;
mod vdmos;

pub use bsim4::{Bsim4, Bsim4Params, Bsim4Type};
pub use ekv::EkvMosfet;
pub use jfet::{Jfet, JfetParams, JfetType};
pub use mos_models::MosParams;
pub use mosfet::{MosRegion, MosType, Mosfet, MosfetIndices};
pub use smooth::{
    SMOOTH_VOLTAGE, smooth_clamp, smooth_max, smooth_min, smooth_positive, smooth_step,
};
pub use vdmos::{Vdmos, VdmosRegion, VdmosType};
