//! MOSFET and FET device models
//!
//! Includes various MOSFET models (Level 1-3, BSIM3, BSIM4, EKV, VDMOS) and JFETs.
//! Also contains shared utilities for smooth region transitions.

#![allow(clippy::module_inception)]
pub mod b3soi;
mod bsim3;
pub mod bsim3v3;
pub mod bsim4v8;
mod bsim4;
mod ekv;
mod jfet;
mod legacy_bsim;
mod mos_models;
mod mosfet;
mod smooth;
mod vdmos;

pub use b3soi::{B3SoiDd, B3SoiDdModel, B3SoiFd, B3SoiFdModel, B3SoiPd, B3SoiPdModel, BodyMode};
pub use bsim3::{Bsim3, Bsim3Params, Bsim3Region, Bsim3Type};
pub use bsim3v3::{Bsim3v3, Bsim3v3Device, Bsim3v3Model};
pub use bsim4::{Bsim4, Bsim4Params, Bsim4Type};
pub use ekv::EkvMosfet;
pub use jfet::{Jfet, JfetChannelModel, JfetParams, JfetType};
pub use mos_models::MosParams;
pub use mosfet::{MosRegion, MosType, Mosfet, MosfetIndices};
pub use smooth::{
    SMOOTH_VOLTAGE, smooth_max, smooth_min, smooth_positive, smooth_step,
};
pub use vdmos::{Vdmos, VdmosRegion, VdmosType};
