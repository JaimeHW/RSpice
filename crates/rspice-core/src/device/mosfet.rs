//! MOSFET and FET device models
//!
//! Includes various MOSFET models (Level 1-3, BSIM3, BSIM4, EKV, VDMOS) and JFETs.
//! Also contains shared utilities for smooth region transitions.

pub mod b3soi;
pub mod bsim3v3;
pub mod bsim4v8;
mod classic;
mod ekv;
mod ekv3;
mod jfet;
mod legacy_bsim;
mod mos_models;
mod smooth;
mod vdmos;

pub use b3soi::{B3SoiDd, B3SoiDdModel, B3SoiFd, B3SoiFdModel, B3SoiPd, B3SoiPdModel, BodyMode};
pub use bsim3v3::{Bsim3v3, Bsim3v3Device, Bsim3v3EquationSet, Bsim3v3Model};
pub use bsim4v8::{Bsim4v8, Bsim4v8Device, Bsim4v8Model};
#[cfg(feature = "parallel")]
pub(crate) use classic::ClassicMosResidualRowPlan;
pub(crate) use classic::{
    ClassicMosCachedStaticTerms, ClassicMosStaticStampPlan, ClassicMosTransientConstants,
    MosfetNonlinearState,
};
pub use classic::{MosBodyJunctionModel, MosRegion, MosType, Mosfet, MosfetIndices};
pub use ekv::EkvMosfet;
pub use ekv3::{Ekv3Device, Ekv3Op};
pub use jfet::{Jfet, JfetChannelModel, JfetParams, JfetType};
pub use mos_models::MosParams;
pub use smooth::{SMOOTH_VOLTAGE, smooth_max, smooth_min, smooth_positive, smooth_step};
pub use vdmos::{Vdmos, VdmosRegion, VdmosType};
