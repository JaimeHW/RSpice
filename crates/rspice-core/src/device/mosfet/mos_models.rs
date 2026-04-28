//! MOSFET model calculations for Level 1 and BSIM3-like models
//!
//! This module contains drain current calculations and transconductance
//! functions for different MOSFET model levels:
//! - Level 1: Shichman-Hodges model (simple, widely used)
//! - Level 3+: BSIM3-like model with short-channel effects
//!
//! All calculations use C1-continuous smooth transition functions from
//! the `smooth` module to ensure Newton-Raphson convergence.

// These utility functions are kept for future mosfet.rs integration
#![allow(dead_code)]

use super::mosfet::{MosRegion, Mosfet};
use super::smooth::{SMOOTH_VOLTAGE, smooth_max, smooth_min, smooth_positive, smooth_step};
use crate::Value;

/// Separate smoothing width for Vds-dependent region transitions.
const VDS_SMOOTHING: Value = SMOOTH_VOLTAGE * 1e-1;

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

//=============================================================================
// Level 1 (Shichman-Hodges) Model
//=============================================================================

/// Level 1 (Shichman-Hodges) drain current calculation with C1 continuous transitions
///
/// Uses smooth blending between regions to ensure continuous first derivatives,
/// which is critical for Newton-Raphson convergence.
///
/// # Arguments
/// * `params` - MOSFET parameters
/// * `vgs` - Gate-source voltage
/// * `vds` - Drain-source voltage  
/// * `vbs` - Bulk-source voltage
/// * `vth` - Threshold voltage (already computed with body effect)
///
/// # Returns
/// (drain_current, operating_region)
#[inline]
pub fn calculate_id_level1(
    params: &MosParams,
    vgs: Value,
    vds: Value,
    vbs: Value,
    vth: Value,
) -> (Value, MosRegion) {
    let p = params.polarity;
    let vgs_eff = p * vgs;
    let vds_eff = smooth_positive(p * vds, VDS_SMOOTHING); // Ensure positive Vds
    let _ = vbs; // Used for vth calculation externally

    // Gate overdrive with smooth cutoff transition
    // vgt_smooth ≈ 0 when vgs < vth, ≈ (vgs - vth) when vgs > vth
    let vgt_raw = vgs_eff - vth;
    let vgt = smooth_positive(vgt_raw, SMOOTH_VOLTAGE);

    // Determine effective region for reporting (but calculations are smooth)
    let region = if vgt_raw <= -SMOOTH_VOLTAGE {
        MosRegion::Cutoff
    } else if vds_eff < vgt - SMOOTH_VOLTAGE {
        MosRegion::Linear
    } else {
        MosRegion::Saturation
    };

    // Smooth saturation voltage: Vdsat = min(Vgt, Vds) but smooth
    // This naturally blends linear and saturation regions
    let vdsat = smooth_min(vgt, vds_eff, SMOOTH_VOLTAGE);

    // Unified current equation that smoothly transitions between regions:
    // In linear: Id = beta * (Vgt * Vds - Vds²/2)
    // In saturation: Id = beta/2 * Vgt² (when Vds = Vgt)
    //
    // Using Vdsat as the effective drain voltage gives us both:
    // Id = beta * (Vgt * Vdsat - Vdsat²/2) * (1 + lambda * Vds)
    let id_core = params.beta * (vgt * vdsat - 0.5 * vdsat * vdsat);
    let id = p * id_core * (1.0 + params.lambda * vds_eff);

    (id, region)
}

//=============================================================================
// BSIM3-like Model
//=============================================================================

/// BSIM3-like drain current with short-channel effects and C1 continuous transitions
///
/// Includes:
/// - Mobility degradation due to vertical electric field
/// - Velocity saturation
/// - Drain-Induced Barrier Lowering (DIBL)
/// - Channel length modulation
/// - Subthreshold conduction
///
/// # Arguments
/// * `params` - MOSFET parameters
/// * `vgs` - Gate-source voltage
/// * `vds` - Drain-source voltage
/// * `vbs` - Bulk-source voltage
/// * `vth_base` - Base threshold voltage (without DIBL)
///
/// # Returns
/// (drain_current, operating_region)
#[inline]
pub fn calculate_id_bsim3(
    params: &MosParams,
    vgs: Value,
    vds: Value,
    vbs: Value,
    vth_base: Value,
) -> (Value, MosRegion) {
    let p = params.polarity;
    let vgs_eff = p * vgs;
    let vds_eff = smooth_positive(p * vds, VDS_SMOOTHING);
    let vbs_eff = p * vbs;

    // DIBL: threshold voltage reduction with drain bias (smooth minimum for Vth)
    let vth_dibl = vth_base - params.eta0 * vds_eff - params.etab * vbs_eff * vds_eff;
    let vth = smooth_max(vth_dibl, 0.1, SMOOTH_VOLTAGE);

    // Gate overdrive with smooth transition
    let vgt_raw = vgs_eff - vth;
    let vgt = smooth_positive(vgt_raw, SMOOTH_VOLTAGE);

    // Subthreshold current blended smoothly with above-threshold current
    let vt = 0.0259; // Thermal voltage at 300K
    let n = params.nfactor;
    // Smooth blend factor: 0 when well above threshold, 1 when below
    let subthreshold_blend = 1.0 - smooth_step(vgt_raw, SMOOTH_VOLTAGE);
    let i_sub = 1e-12 * (vgt_raw.min(100.0 * vt) / (n * vt)).exp().min(1e6);

    // Mobility degradation (vertical field effect)
    let eeff = vgt / 6e-9; // Assume tox = 6nm
    let mobility = params.u0 / (1.0 + params.ua * eeff + params.ub * eeff * eeff);

    // Effective beta with mobility degradation
    let beta_eff = mobility * 1e-4 * params.cox * params.wl_ratio;

    // Saturation voltage with velocity saturation (smooth formulation)
    let vsat_over_l = params.vsat / params.l;
    let mu_m2 = mobility * 1e-4;
    let vdsat_vel = vgt / (1.0 + vgt / (params.l * vsat_over_l / mu_m2).max(1e-6));

    // Smooth min between Vds and Vdsat for unified linear/saturation
    let vdsat = smooth_min(vdsat_vel, vds_eff, SMOOTH_VOLTAGE);

    // Channel length modulation (smooth)
    let vds_over_vdsat = vds_eff / vdsat_vel.max(1e-6);
    let clm_arg = smooth_positive(vds_over_vdsat - 1.0, 0.01);
    let clm = 1.0 + params.pclm * clm_arg.ln_1p();

    // Unified current equation
    let id_above = beta_eff * (vgt * vdsat - 0.5 * vdsat * vdsat) * clm;

    // Blend subthreshold and above-threshold currents
    let id = p * (subthreshold_blend * i_sub + (1.0 - subthreshold_blend) * id_above);

    // Region determination (for reporting only)
    let region = if vgt_raw <= -SMOOTH_VOLTAGE {
        MosRegion::Cutoff
    } else if vds_eff < vdsat_vel - SMOOTH_VOLTAGE {
        MosRegion::Linear
    } else {
        MosRegion::Saturation
    };

    (id, region)
}

//=============================================================================
// Transconductance Calculations
//=============================================================================

/// Calculate transconductance gm = dId/dVgs with C1 continuity
///
/// Uses the same smooth transitions as calculate_id for consistent derivatives.
#[inline]
pub fn calculate_gm(params: &MosParams, vgs: Value, vds: Value, vth: Value) -> Value {
    let p = params.polarity;
    let vgs_eff = p * vgs;
    let vds_eff = smooth_positive(p * vds, VDS_SMOOTHING);

    // Smooth gate overdrive
    let vgt_raw = vgs_eff - vth;
    let vgt = smooth_positive(vgt_raw, SMOOTH_VOLTAGE);

    // Smooth Vdsat for region blending
    let vdsat = smooth_min(vgt, vds_eff, SMOOTH_VOLTAGE);

    // Derivative of smooth_positive for cutoff transition
    // d(smooth_positive(x))/dx = smooth_step(x)
    let dvgt_dvgs = smooth_step(vgt_raw, SMOOTH_VOLTAGE);

    // In unified formulation: Id = beta * (Vgt * Vdsat - Vdsat²/2)
    // Taking derivative w.r.t Vgs:
    // dId/dVgs = beta * (Vdsat * dVgt/dVgs + Vgt * dVdsat/dVgs - Vdsat * dVdsat/dVgs)
    // When Vds > Vgt (saturation): dVdsat/dVgs ≈ dVgt/dVgs
    // When Vds < Vgt (linear): dVdsat/dVgs ≈ 0 (Vdsat = Vds)

    // Blend factor: how much Vdsat follows Vgt
    let sat_blend = smooth_step(vgt - vds_eff, SMOOTH_VOLTAGE);
    let dvdsat_dvgs = sat_blend * dvgt_dvgs;

    let gm_core = params.beta * (vdsat * dvgt_dvgs + (vgt - vdsat) * dvdsat_dvgs);
    let gm_clm = gm_core * (1.0 + params.lambda * vds_eff);

    // Ensure minimum conductance for numerical stability
    gm_clm.max(1e-12)
}

/// Calculate output conductance gds = dId/dVds with C1 continuity
#[inline]
pub fn calculate_gds(params: &MosParams, vgs: Value, vds: Value, vth: Value) -> Value {
    let p = params.polarity;
    let vgs_eff = p * vgs;
    let vds_eff = smooth_positive(p * vds, VDS_SMOOTHING);

    // Smooth gate overdrive
    let vgt_raw = vgs_eff - vth;
    let vgt = smooth_positive(vgt_raw, SMOOTH_VOLTAGE);

    // Smooth Vdsat
    let vdsat = smooth_min(vgt, vds_eff, SMOOTH_VOLTAGE);

    // Blend factor for region transition
    let lin_blend = smooth_step(vgt - vds_eff, SMOOTH_VOLTAGE);

    // In linear region: gds = beta * (Vgt - Vds) + lambda term
    // In saturation: gds = lambda * Id (small)
    // Derivative of Vdsat w.r.t Vds: ~1 in linear, ~0 in saturation
    let dvdsat_dvds = 1.0 - lin_blend;

    // dId/dVds from main current term
    let gds_core = params.beta * (vgt - vdsat) * dvdsat_dvds;

    // Channel length modulation term
    let id_core = params.beta * (vgt * vdsat - 0.5 * vdsat * vdsat);
    let gds_clm = id_core * params.lambda;

    // Total conductance
    let gds_total = gds_core * (1.0 + params.lambda * vds_eff) + gds_clm;

    // Ensure minimum conductance
    gds_total.max(1e-12)
}

/// Calculate body transconductance gmb = dId/dVbs with C1 continuity
///
/// gmb represents the change in drain current due to body bias.
#[inline]
pub fn calculate_gmb(params: &MosParams, vgs: Value, vds: Value, vbs: Value, vth: Value) -> Value {
    let p = params.polarity;
    let vbs_eff = p * vbs;

    // gmb = -gm * (gamma / (2 * sqrt(phi - Vbs)))
    // The gm function is already smooth, so gmb inherits smoothness
    let gm = calculate_gm(params, vgs, vds, vth);

    // Smooth the phi - Vbs term to avoid singularity
    let phi_vbs = smooth_max(params.phi - vbs_eff, SMOOTH_VOLTAGE, SMOOTH_VOLTAGE);

    gm * params.gamma / (2.0 * phi_vbs.sqrt())
}

//=============================================================================
// Threshold Voltage Calculation
//=============================================================================

/// Calculate effective threshold voltage with body effect and short-channel effects
///
/// For Level 1: Standard body effect formula
/// For Level 3+: Includes BSIM4 short-channel Vth roll-off
#[inline]
pub fn calculate_vth(params: &MosParams, vbs: Value) -> Value {
    let p = params.polarity;
    let vbs_eff = p * vbs;
    let vto_eff = if p < 0.0 {
        params.vto.abs()
    } else {
        params.vto
    };

    // Base body effect: Vth = Vto + gamma * (sqrt(phi - Vbs) - sqrt(phi))
    let phi_vbs = (params.phi - vbs_eff).max(0.0);
    let vth_base = vto_eff + params.gamma * (phi_vbs.sqrt() - params.phi.sqrt());

    if params.level < 3 {
        return vth_base;
    }

    // BSIM4 short-channel Vth roll-off
    // Delta_Vth = -DVT0 * L_eff / Ldrawn * (1 + DVT2 * Vbs)
    // where L_eff adjustment factor uses DVT1
    let l_ratio = 1e-6 / params.l.max(1e-9); // Normalize to 1um
    let dvth_sce = -params.dvt0 * l_ratio * (1.0 + params.dvt1 * l_ratio);

    // Body-bias modulation of SCE
    let dvth_bias = params.dvt2 * vbs_eff * l_ratio;

    // Enhanced body effect using K1/K2 (BSIM4 style)
    // Vth = Vto + K1 * sqrt(phi - Vbs) + K2 * (phi - Vbs)
    let vth_k1k2 = vto_eff + params.k1 * phi_vbs.sqrt() + params.k2 * (params.phi - vbs_eff);

    // Blend between GAMMA-based and K1/K2-based body effect based on model level
    // Use K1/K2 formulation for short channels (level 3+)
    vth_k1k2 + dvth_sce + dvth_bias
}
