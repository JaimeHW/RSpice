//! The parameter bundle a native Berkeley-style MOSFET hands to its model
//! evaluation, and the extraction that fills it from a device instance.

use super::classic::Mosfet;
use crate::Value;

//=============================================================================
// MOS Model Parameters - extracted from Mosfet for calculation use
//=============================================================================

/// Parameters needed for model calculations
#[derive(Debug, Clone, Copy)]
pub struct MosParams {
    /// Device polarity (+1 for NMOS, -1 for PMOS)
    pub polarity: Value,
    /// Threshold voltage (VTO)
    pub vto: Value,
    /// Transconductance parameter (KP) in A/V^2  
    pub kp: Value,
    /// Body effect coefficient (GAMMA)
    pub gamma: Value,
    /// Surface potential (PHI)
    pub phi: Value,
    /// Channel-length modulation (LAMBDA)
    pub lambda: Value,
    /// Oxide capacitance per unit area (COX)
    pub cox: Value,
    /// W/L ratio
    pub wl_ratio: Value,
    /// Beta = KP * W/L
    pub beta: Value,
    /// Channel length (L) in meters
    pub l: Value,
    /// Model level
    pub level: i32,
    /// Low-field mobility (U0) in cm^2/V*s
    pub u0: Value,
    /// First-order mobility degradation (UA)
    pub ua: Value,
    /// Second-order mobility degradation (UB)
    pub ub: Value,
    /// Saturation velocity (VSAT) in m/s
    pub vsat: Value,
    /// DIBL coefficient 1 (ETA0)
    pub eta0: Value,
    /// DIBL coefficient 2 (ETAB)
    pub etab: Value,
    /// Subthreshold swing coefficient (NFACTOR)
    pub nfactor: Value,
    /// Channel length modulation coefficient (PCLM)
    pub pclm: Value,
    /// First body effect coefficient (K1)
    pub k1: Value,
    /// Second body effect coefficient (K2)
    pub k2: Value,
    /// Short-channel Vth roll-off coefficient 0 (DVT0)
    pub dvt0: Value,
    /// Short-channel Vth roll-off coefficient 1 (DVT1)
    pub dvt1: Value,
    /// Short-channel Vth roll-off body-bias coefficient (DVT2)
    pub dvt2: Value,
}

impl Mosfet {
    /// Extract parameters for model calculations
    pub fn params(&self) -> MosParams {
        MosParams {
            polarity: self.polarity(),
            vto: self.vto,
            kp: self.kp,
            gamma: self.gamma,
            phi: self.phi,
            lambda: self.lambda,
            cox: self.cox,
            wl_ratio: self.wl_ratio(),
            beta: self.beta(),
            l: self.l,
            level: self.level,
            u0: self.u0,
            ua: self.ua,
            ub: self.ub,
            vsat: self.vsat,
            eta0: self.eta0,
            etab: self.etab,
            nfactor: self.nfactor,
            pclm: self.pclm,
            k1: self.k1,
            k2: self.k2,
            dvt0: self.dvt0,
            dvt1: self.dvt1,
            dvt2: self.dvt2,
        }
    }
}
