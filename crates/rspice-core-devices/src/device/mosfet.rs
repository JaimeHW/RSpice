//! MOSFET and FET device models
//!
//! Includes various MOSFET models (Level 1-3, BSIM3, BSIM4, EKV, VDMOS) and JFETs.
//! Also contains the shared smooth region transitions and ngspice FET step limiters.

use crate::Value;

/// The four circuit nodes a four-terminal MOSFET attaches to. They are
/// positional in the netlist (`M<name> nd ng ns nb`) and trivial to transpose;
/// naming them makes a swapped pair a compile error rather than a device that
/// silently conducts the wrong way.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MosTerminals {
    pub drain: crate::NodeId,
    pub gate: crate::NodeId,
    pub source: crate::NodeId,
    pub bulk: crate::NodeId,
}

/// ngspice `DEVlimvds`: bounds one Newton step of a drain-source voltage.
#[inline]
pub(crate) fn dev_limvds(vnew: Value, vold: Value) -> Value {
    if vold >= 3.5 {
        if vnew > vold {
            vnew.min(3.0 * vold + 2.0)
        } else if vnew < 3.5 {
            vnew.max(2.0)
        } else {
            vnew
        }
    } else if vnew > vold {
        vnew.min(4.0)
    } else {
        vnew.max(-0.5)
    }
}

/// ngspice `DEVfetlim`: bounds one Newton step of a FET gate voltage around
/// the threshold `vto`.
#[inline]
pub(crate) fn dev_fetlim(vnew: Value, vold: Value, vto: Value) -> Value {
    let vtsthi = (2.0 * (vold - vto)).abs() + 2.0;
    let vtstlo = (vold - vto).abs() + 1.0;
    let vtox = vto + 3.5;
    let delv = vnew - vold;

    if vold >= vto {
        if vold >= vtox {
            if delv <= 0.0 {
                if vnew >= vtox {
                    if -delv > vtstlo { vold - vtstlo } else { vnew }
                } else {
                    vnew.max(vto + 2.0)
                }
            } else if delv >= vtsthi {
                vold + vtsthi
            } else {
                vnew
            }
        } else if delv <= 0.0 {
            vnew.max(vto - 0.5)
        } else {
            vnew.min(vto + 4.0)
        }
    } else if delv <= 0.0 {
        if -delv > vtsthi { vold - vtsthi } else { vnew }
    } else {
        let vtemp = vto + 0.5;
        if vnew <= vtemp {
            if delv > vtstlo { vold + vtstlo } else { vnew }
        } else {
            vtemp
        }
    }
}

mod ac_nqs;
pub mod b3soi;
pub mod bsim3v3;
pub mod bsim4v8;
mod classic;
mod dual;
mod ekv;
mod ekv3;
mod jfet;
mod legacy_bsim;
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub mod oracle_cards;
mod smooth;
mod vdmos;

pub use b3soi::{B3SoiDd, B3SoiDdModel, B3SoiFd, B3SoiFdModel, B3SoiPd, B3SoiPdModel, BodyMode};
#[doc(hidden)]
pub use b3soi::{B3SoiDdNodes, B3SoiPdNodes};
pub use bsim3v3::{Bsim3v3, Bsim3v3Device, Bsim3v3EquationSet, Bsim3v3Model};
pub use bsim4v8::{Bsim4v8, Bsim4v8Device, Bsim4v8Model};
#[cfg(feature = "parallel")]
#[doc(hidden)]
pub use classic::ClassicMosResidualRowPlan;
#[doc(hidden)]
pub use classic::{AcceptedMosfetNonlinearCheckpoint, MosBodyJunctionModel};
#[doc(hidden)]
pub use classic::{
    ClassicMosCachedStaticTerms, ClassicMosStaticStampPlan, ClassicMosTransientConstants,
    MosfetNonlinearState,
};
pub use classic::{MosType, Mosfet};
pub use ekv::EkvMosfet;
pub use ekv3::{Ekv3Device, Ekv3Op};
#[doc(hidden)]
pub use jfet::{AcceptedJfetNonlinearCheckpoint, JFET_CHECKPOINT_RUNTIME_TAGS};
pub use jfet::{Jfet, JfetChannelModel, JfetParams, JfetType};
pub use smooth::{SMOOTH_VOLTAGE, smooth_max, smooth_min, smooth_positive, smooth_step};
pub use vdmos::{Vdmos, VdmosRegion, VdmosType};
