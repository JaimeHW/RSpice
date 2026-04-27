//! Legacy Berkeley short-channel MOS models.
//!
//! Levels 4 and 5 in SPICE3/ngspice are the original BSIM1 and BSIM2
//! implementations. Their parameter names and equations predate BSIM3 and
//! cannot be interpreted as BSIM3/BSIM4 cards.

use crate::Value;
use std::collections::HashMap;

const EPS_OX_CGS: Value = 3.453e-13;
const MICRON: Value = 1.0e-6;
const LEGACY_VT_300K: Value = 0.025_864_19;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyBsimRegion {
    Cutoff,
    Linear,
    Saturation,
}

#[derive(Debug, Clone)]
pub enum LegacyBsimModel {
    Bsim1(LegacyBsim1Model),
    Bsim2(LegacyBsim2Model),
}

#[derive(Debug, Clone)]
pub enum LegacyBsimSizedModel {
    Bsim1(LegacyBsim1Sized),
    Bsim2(LegacyBsim2Sized),
}

impl LegacyBsimModel {
    pub fn from_level_and_params(level: i32, params: &HashMap<String, Value>) -> Option<Self> {
        match level {
            4 => Some(Self::Bsim1(LegacyBsim1Model::from_params(params))),
            5 => Some(Self::Bsim2(LegacyBsim2Model::from_params(params))),
            _ => None,
        }
    }

    pub fn sized(&self, width: Value, length: Value) -> Option<LegacyBsimSizedModel> {
        match self {
            Self::Bsim1(model) => model.sized(width, length).map(LegacyBsimSizedModel::Bsim1),
            Self::Bsim2(model) => model.sized(width, length).map(LegacyBsimSizedModel::Bsim2),
        }
    }
}

impl LegacyBsimSizedModel {
    pub fn evaluate(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, LegacyBsimRegion) {
        match self {
            Self::Bsim1(model) => model.evaluate(vgs, vds, vbs),
            Self::Bsim2(model) => model.evaluate(vgs, vds, vbs),
        }
    }

    pub fn threshold(&self, vds: Value, vbs: Value) -> Value {
        match self {
            Self::Bsim1(model) => model.threshold(vds, vbs),
            Self::Bsim2(model) => model.threshold(vds, vbs),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LegacyBsim1Model {
    vfb: SizeDependence,
    phi: SizeDependence,
    k1: SizeDependence,
    k2: SizeDependence,
    eta: SizeDependence,
    eta_b: SizeDependence,
    eta_d: SizeDependence,
    delta_l: Value,
    delta_w: Value,
    beta_zero: SizeDependence,
    beta_zero_b: SizeDependence,
    beta_vdd: SizeDependence,
    beta_vdd_b: SizeDependence,
    beta_vdd_d: SizeDependence,
    ugs: SizeDependence,
    ugs_b: SizeDependence,
    uds: SizeDependence,
    uds_b: SizeDependence,
    uds_d: SizeDependence,
    subth_slope: SizeDependence,
    subth_slope_b: SizeDependence,
    subth_slope_d: SizeDependence,
    tox: Value,
    vdd: Value,
}

#[derive(Debug, Clone)]
pub struct LegacyBsim1Sized {
    vfb: Value,
    phi: Value,
    k1: Value,
    k2: Value,
    eta: Value,
    eta_b: Value,
    eta_d: Value,
    beta_zero: Value,
    beta_zero_b: Value,
    beta_vdd: Value,
    beta_vdd_b: Value,
    beta_vdd_d: Value,
    ugs: Value,
    ugs_b: Value,
    uds: Value,
    uds_b: Value,
    uds_d: Value,
    subth_slope: Value,
    subth_slope_b: Value,
    subth_slope_d: Value,
    vdd: Value,
    leff_um: Value,
}

#[derive(Debug, Clone)]
pub struct LegacyBsim2Model {
    vfb: SizeDependence,
    phi: SizeDependence,
    k1: SizeDependence,
    k2: SizeDependence,
    eta0: SizeDependence,
    eta_b: SizeDependence,
    delta_l: Value,
    delta_w: Value,
    beta0: SizeDependence,
    beta0_b: SizeDependence,
    betas0: SizeDependence,
    betas_b: SizeDependence,
    beta20: SizeDependence,
    beta2_b: SizeDependence,
    beta2_g: SizeDependence,
    beta30: SizeDependence,
    beta3_b: SizeDependence,
    beta3_g: SizeDependence,
    beta40: SizeDependence,
    beta4_b: SizeDependence,
    beta4_g: SizeDependence,
    ua0: SizeDependence,
    ua_b: SizeDependence,
    ub0: SizeDependence,
    ub_b: SizeDependence,
    u10: SizeDependence,
    u1_b: SizeDependence,
    u1_d: SizeDependence,
    n0: SizeDependence,
    n_b: SizeDependence,
    n_d: SizeDependence,
    vof0: SizeDependence,
    vof_b: SizeDependence,
    vof_d: SizeDependence,
    ai0: SizeDependence,
    ai_b: SizeDependence,
    bi0: SizeDependence,
    bi_b: SizeDependence,
    vghigh: SizeDependence,
    vglow: SizeDependence,
    tox: Value,
    temp_c: Value,
    vdd: Value,
    vgg: Value,
    vbb: Value,
}

#[derive(Debug, Clone)]
pub struct LegacyBsim2Sized {
    vfb: Value,
    phi: Value,
    k1: Value,
    k2: Value,
    eta0: Value,
    eta_b: Value,
    beta0: Value,
    beta0_b: Value,
    betas0: Value,
    betas_b: Value,
    beta20: Value,
    beta2_b: Value,
    beta2_g: Value,
    beta30: Value,
    beta3_b: Value,
    beta3_g: Value,
    beta40: Value,
    beta4_b: Value,
    beta4_g: Value,
    ua0: Value,
    ua_b: Value,
    ub0: Value,
    ub_b: Value,
    u10: Value,
    u1_b: Value,
    u1_d: Value,
    n0: Value,
    n_b: Value,
    n_d: Value,
    vof0: Value,
    vof_b: Value,
    vof_d: Value,
    ai0: Value,
    ai_b: Value,
    bi0: Value,
    bi_b: Value,
    vghigh: Value,
    vglow: Value,
    vdd: Value,
    vgg: Value,
    vbb: Value,
    vtm: Value,
    phi_sqrt_phi: Value,
}

#[derive(Debug, Clone, Copy)]
struct SizeDependence {
    nominal: Value,
    length: Value,
    width: Value,
}

impl SizeDependence {
    fn from_params(params: &HashMap<String, Value>, name: &str, default: Value) -> Self {
        Self {
            nominal: param(params, name, default),
            length: param(params, &format!("L{name}"), 0.0),
            width: param(params, &format!("W{name}"), 0.0),
        }
    }

    fn eval(self, inv_l_um: Value, inv_w_um: Value) -> Value {
        self.nominal + self.length * inv_l_um + self.width * inv_w_um
    }
}

impl LegacyBsim1Model {
    fn from_params(params: &HashMap<String, Value>) -> Self {
        Self {
            vfb: SizeDependence::from_params(params, "VFB", 0.0),
            phi: SizeDependence::from_params(params, "PHI", 0.0),
            k1: SizeDependence::from_params(params, "K1", 0.0),
            k2: SizeDependence::from_params(params, "K2", 0.0),
            eta: SizeDependence::from_params(params, "ETA", 0.0),
            eta_b: SizeDependence::from_params(params, "X2E", 0.0),
            eta_d: SizeDependence::from_params(params, "X3E", 0.0),
            delta_l: param(params, "DL", 0.0),
            delta_w: param(params, "DW", 0.0),
            beta_zero: SizeDependence::from_params(params, "MUZ", 0.0),
            beta_zero_b: SizeDependence::from_params(params, "X2MZ", 0.0),
            beta_vdd: SizeDependence::from_params(params, "MUS", 0.0),
            beta_vdd_b: SizeDependence::from_params(params, "X2MS", 0.0),
            beta_vdd_d: SizeDependence::from_params(params, "X3MS", 0.0),
            ugs: SizeDependence::from_params(params, "U0", 0.0),
            ugs_b: SizeDependence::from_params(params, "X2U0", 0.0),
            uds: SizeDependence::from_params(params, "U1", 0.0),
            uds_b: SizeDependence::from_params(params, "X2U1", 0.0),
            uds_d: SizeDependence::from_params(params, "X3U1", 0.0),
            subth_slope: SizeDependence::from_params(params, "N0", 0.0),
            subth_slope_b: SizeDependence::from_params(params, "NB", 0.0),
            subth_slope_d: SizeDependence::from_params(params, "ND", 0.0),
            tox: param(params, "TOX", 0.0),
            vdd: param(params, "VDD", 0.0),
        }
    }

    fn sized(&self, width: Value, length: Value) -> Option<LegacyBsim1Sized> {
        let effective_length = length - self.delta_l * MICRON;
        let effective_width = width - self.delta_w * MICRON;
        if !effective_length.is_finite()
            || !effective_width.is_finite()
            || effective_length <= 0.0
            || effective_width <= 0.0
        {
            return None;
        }

        let leff_um = effective_length / MICRON;
        let weff_um = effective_width / MICRON;
        let inv_l_um = 1.0 / leff_um;
        let inv_w_um = 1.0 / weff_um;
        let cox = legacy_cox(self.tox);
        let cox_w_over_l = cox * weff_um / leff_um;

        let phi = self.phi.eval(inv_l_um, inv_w_um).max(0.1);
        let k1 = self.k1.eval(inv_l_um, inv_w_um).max(0.0);
        let k2 = self.k2.eval(inv_l_um, inv_w_um).max(0.0);

        Some(LegacyBsim1Sized {
            vfb: self.vfb.eval(inv_l_um, inv_w_um),
            phi,
            k1,
            k2,
            eta: self.eta.eval(inv_l_um, inv_w_um),
            eta_b: self.eta_b.eval(inv_l_um, inv_w_um),
            eta_d: self.eta_d.eval(inv_l_um, inv_w_um),
            beta_zero: self.beta_zero.eval(inv_l_um, inv_w_um) * cox_w_over_l,
            beta_zero_b: self.beta_zero_b.eval(inv_l_um, inv_w_um) * cox_w_over_l,
            beta_vdd: self.beta_vdd.eval(inv_l_um, inv_w_um) * cox_w_over_l,
            beta_vdd_b: self.beta_vdd_b.eval(inv_l_um, inv_w_um) * cox_w_over_l,
            beta_vdd_d: (self.beta_vdd_d.eval(inv_l_um, inv_w_um) * cox_w_over_l).max(0.0),
            ugs: self.ugs.eval(inv_l_um, inv_w_um),
            ugs_b: self.ugs_b.eval(inv_l_um, inv_w_um),
            uds: self.uds.eval(inv_l_um, inv_w_um),
            uds_b: self.uds_b.eval(inv_l_um, inv_w_um),
            uds_d: self.uds_d.eval(inv_l_um, inv_w_um),
            subth_slope: self.subth_slope.eval(inv_l_um, inv_w_um),
            subth_slope_b: self.subth_slope_b.eval(inv_l_um, inv_w_um),
            subth_slope_d: self.subth_slope_d.eval(inv_l_um, inv_w_um),
            vdd: self.vdd,
            leff_um,
        })
    }
}

impl LegacyBsim1Sized {
    fn threshold_components(&self, vds: Value, vbs: Value) -> (Value, Value, Value) {
        if !vds.is_finite() || !vbs.is_finite() {
            return (self.vfb + self.phi, self.phi, self.phi.sqrt());
        }
        let mut eta = self.eta + self.eta_b * vbs + self.eta_d * (vds - self.vdd);
        eta = eta.clamp(0.0, 1.0);
        let vpb = if vbs < 0.0 { self.phi - vbs } else { self.phi };
        let sqrt_vpb = vpb.sqrt();
        let threshold = self.vfb + self.phi + self.k1 * sqrt_vpb - self.k2 * vpb - eta * vds;
        (threshold, vpb, sqrt_vpb)
    }

    fn threshold(&self, vds: Value, vbs: Value) -> Value {
        self.threshold_components(vds, vbs).0
    }

    fn evaluate(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, LegacyBsimRegion) {
        if !vgs.is_finite() || !vds.is_finite() || !vbs.is_finite() {
            return (0.0, LegacyBsimRegion::Cutoff);
        }

        let mut ugs = self.ugs + self.ugs_b * vbs;
        if ugs <= 0.0 {
            ugs = 0.0;
        }

        let mut uds = self.uds + self.uds_b * vbs + self.uds_d * (vds - self.vdd);
        if uds <= 0.0 {
            uds = 0.0;
        } else {
            uds /= self.leff_um;
        }

        let (vth, vpb, sqrt_vpb) = self.threshold_components(vds, vbs);
        let vgt = vgs - vth;
        let g = 1.0 - 1.0 / (1.744 + 0.8364 * vpb);
        let a = (1.0 + 0.5 * g * self.k1 / sqrt_vpb).max(1.0);
        let arg = (1.0 + ugs * vgt).max(1.0);

        let mut drain_current = 0.0;
        let mut region = LegacyBsimRegion::Cutoff;
        if vgt >= 0.0 {
            let beta_vds_0 = self.beta_zero + self.beta_zero_b * vbs;
            let beta_vdd = self.beta_vdd + self.beta_vdd_b * vbs;
            let beta0 = if vds > self.vdd {
                beta_vdd + self.beta_vdd_d * (vds - self.vdd)
            } else if self.vdd.abs() > 1e-30 {
                let vdd_square = self.vdd * self.vdd;
                let c1 = (-beta_vdd + beta_vds_0 + self.beta_vdd_d * self.vdd) / vdd_square;
                let c2 = 2.0 * (beta_vdd - beta_vds_0) / self.vdd - self.beta_vdd_d;
                (c1 * vds + c2) * vds + beta_vds_0
            } else {
                beta_vds_0
            };
            let beta = beta0 / arg;
            let vc = (uds * vgt / a).max(0.0);
            let term1 = (1.0 + 2.0 * vc).sqrt();
            let k = 0.5 * (1.0 + vc + term1);
            let vdsat = (vgt / (a * k.sqrt())).max(0.0);

            if vds < vdsat {
                region = LegacyBsimRegion::Linear;
                let arg_l1 = (1.0 + uds * vds).max(1.0);
                let arg_l2 = vgt - 0.5 * a * vds;
                drain_current = beta * arg_l2 * vds / arg_l1;
            } else {
                region = LegacyBsimRegion::Saturation;
                let arg_s2 = vgt / a / k;
                let arg_s3 = arg_s2 * vgt;
                drain_current = 0.5 * beta * arg_s3;
            }
        }

        if self.subth_slope < 200.0 {
            let n =
                (self.subth_slope + self.subth_slope_b * vbs + self.subth_slope_d * vds).max(0.5);
            let wds = 1.0 - (-vds / LEGACY_VT_300K).exp();
            let wgs = (vgt / (n * LEGACY_VT_300K)).exp();
            let vt_square = LEGACY_VT_300K * LEGACY_VT_300K;
            let warg2 = 6.04965 * vt_square * self.beta_zero;
            let ilimit = 4.5 * vt_square * self.beta_zero;
            let iexp = warg2 * wgs * wds;
            let denom = ilimit + iexp;
            if denom.abs() > 1e-300 {
                drain_current += ilimit * iexp / denom;
            }
        }

        (drain_current.max(0.0), region)
    }
}

impl LegacyBsim2Model {
    fn from_params(params: &HashMap<String, Value>) -> Self {
        Self {
            vfb: SizeDependence::from_params(params, "VFB", -1.0),
            phi: SizeDependence::from_params(params, "PHI", 0.75),
            k1: SizeDependence::from_params(params, "K1", 0.8),
            k2: SizeDependence::from_params(params, "K2", 0.0),
            eta0: SizeDependence::from_params(params, "ETA0", 0.0),
            eta_b: SizeDependence::from_params(params, "ETAB", 0.0),
            delta_l: param(params, "DL", 0.0),
            delta_w: param(params, "DW", 0.0),
            beta0: SizeDependence::from_params(params, "MU0", 400.0),
            beta0_b: SizeDependence::from_params(params, "MU0B", 0.0),
            betas0: SizeDependence::from_params(params, "MUS0", 500.0),
            betas_b: SizeDependence::from_params(params, "MUSB", 0.0),
            beta20: SizeDependence::from_params(params, "MU20", 1.5),
            beta2_b: SizeDependence::from_params(params, "MU2B", 0.0),
            beta2_g: SizeDependence::from_params(params, "MU2G", 0.0),
            beta30: SizeDependence::from_params(params, "MU30", 10.0),
            beta3_b: SizeDependence::from_params(params, "MU3B", 0.0),
            beta3_g: SizeDependence::from_params(params, "MU3G", 0.0),
            beta40: SizeDependence::from_params(params, "MU40", 0.0),
            beta4_b: SizeDependence::from_params(params, "MU4B", 0.0),
            beta4_g: SizeDependence::from_params(params, "MU4G", 0.0),
            ua0: SizeDependence::from_params(params, "UA0", 0.2),
            ua_b: SizeDependence::from_params(params, "UAB", 0.0),
            ub0: SizeDependence::from_params(params, "UB0", 0.0),
            ub_b: SizeDependence::from_params(params, "UBB", 0.0),
            u10: SizeDependence::from_params(params, "U10", 0.1),
            u1_b: SizeDependence::from_params(params, "U1B", 0.0),
            u1_d: SizeDependence::from_params(params, "U1D", 0.0),
            n0: SizeDependence::from_params(params, "N0", 1.4),
            n_b: SizeDependence::from_params(params, "NB", 0.5),
            n_d: SizeDependence::from_params(params, "ND", 0.0),
            vof0: SizeDependence::from_params(params, "VOF0", 1.8),
            vof_b: SizeDependence::from_params(params, "VOFB", 0.0),
            vof_d: SizeDependence::from_params(params, "VOFD", 0.0),
            ai0: SizeDependence::from_params(params, "AI0", 0.0),
            ai_b: SizeDependence::from_params(params, "AIB", 0.0),
            bi0: SizeDependence::from_params(params, "BI0", 0.0),
            bi_b: SizeDependence::from_params(params, "BIB", 0.0),
            vghigh: SizeDependence::from_params(params, "VGHIGH", 0.2),
            vglow: SizeDependence::from_params(params, "VGLOW", -0.15),
            tox: param(params, "TOX", 0.03),
            temp_c: param(params, "TEMP", 27.0),
            vdd: param(params, "VDD", 5.0),
            vgg: param(params, "VGG", 5.0),
            vbb: param(params, "VBB", 5.0),
        }
    }

    fn sized(&self, width: Value, length: Value) -> Option<LegacyBsim2Sized> {
        let effective_length = length - self.delta_l * MICRON;
        let effective_width = width - self.delta_w * MICRON;
        if !effective_length.is_finite()
            || !effective_width.is_finite()
            || effective_length <= 0.0
            || effective_width <= 0.0
        {
            return None;
        }

        let inv_l_um = MICRON / effective_length;
        let inv_w_um = MICRON / effective_width;
        let cox_w_over_l = legacy_cox(self.tox) * effective_width / effective_length;

        let mut beta0 = self.beta0.eval(inv_l_um, inv_w_um);
        let mut beta0_b = self.beta0_b.eval(inv_l_um, inv_w_um);
        let mut betas0 = self.betas0.eval(inv_l_um, inv_w_um);
        let mut betas_b = self.betas_b.eval(inv_l_um, inv_w_um);
        let beta20 = self.beta20.eval(inv_l_um, inv_w_um);
        let beta2_b = self.beta2_b.eval(inv_l_um, inv_w_um);
        let beta2_g = self.beta2_g.eval(inv_l_um, inv_w_um);
        let mut beta30 = self.beta30.eval(inv_l_um, inv_w_um);
        let mut beta3_b = self.beta3_b.eval(inv_l_um, inv_w_um);
        let mut beta3_g = self.beta3_g.eval(inv_l_um, inv_w_um);
        let mut beta40 = self.beta40.eval(inv_l_um, inv_w_um);
        let mut beta4_b = self.beta4_b.eval(inv_l_um, inv_w_um);
        let mut beta4_g = self.beta4_g.eval(inv_l_um, inv_w_um);

        if betas0 < 1.01 * beta0 {
            betas0 = 1.01 * beta0;
        }
        let tmp = betas0 - beta0 - beta0_b * self.vbb;
        if (-betas_b * self.vbb) > tmp && self.vbb.abs() > 1e-30 {
            betas_b = -tmp / self.vbb;
        }

        beta0 *= cox_w_over_l;
        beta0_b *= cox_w_over_l;
        betas0 *= cox_w_over_l;
        betas_b *= cox_w_over_l;
        beta30 *= cox_w_over_l;
        beta3_b *= cox_w_over_l;
        beta3_g *= cox_w_over_l;
        beta40 *= cox_w_over_l;
        beta4_b *= cox_w_over_l;
        beta4_g *= cox_w_over_l;

        let phi = self.phi.eval(inv_l_um, inv_w_um);
        let sqrt_phi = phi.sqrt();

        Some(LegacyBsim2Sized {
            vfb: self.vfb.eval(inv_l_um, inv_w_um),
            phi,
            k1: self.k1.eval(inv_l_um, inv_w_um),
            k2: self.k2.eval(inv_l_um, inv_w_um),
            eta0: self.eta0.eval(inv_l_um, inv_w_um),
            eta_b: self.eta_b.eval(inv_l_um, inv_w_um),
            beta0,
            beta0_b,
            betas0,
            betas_b,
            beta20,
            beta2_b,
            beta2_g,
            beta30,
            beta3_b,
            beta3_g,
            beta40,
            beta4_b,
            beta4_g,
            ua0: self.ua0.eval(inv_l_um, inv_w_um),
            ua_b: self.ua_b.eval(inv_l_um, inv_w_um),
            ub0: self.ub0.eval(inv_l_um, inv_w_um),
            ub_b: self.ub_b.eval(inv_l_um, inv_w_um),
            u10: self.u10.eval(inv_l_um, inv_w_um),
            u1_b: self.u1_b.eval(inv_l_um, inv_w_um),
            u1_d: self.u1_d.eval(inv_l_um, inv_w_um),
            n0: self.n0.eval(inv_l_um, inv_w_um).max(0.0),
            n_b: self.n_b.eval(inv_l_um, inv_w_um),
            n_d: self.n_d.eval(inv_l_um, inv_w_um),
            vof0: self.vof0.eval(inv_l_um, inv_w_um),
            vof_b: self.vof_b.eval(inv_l_um, inv_w_um),
            vof_d: self.vof_d.eval(inv_l_um, inv_w_um),
            ai0: self.ai0.eval(inv_l_um, inv_w_um),
            ai_b: self.ai_b.eval(inv_l_um, inv_w_um),
            bi0: self.bi0.eval(inv_l_um, inv_w_um),
            bi_b: self.bi_b.eval(inv_l_um, inv_w_um),
            vghigh: self.vghigh.eval(inv_l_um, inv_w_um),
            vglow: self.vglow.eval(inv_l_um, inv_w_um),
            vdd: self.vdd,
            vgg: self.vgg,
            vbb: self.vbb,
            vtm: 8.625e-5 * (self.temp_c + 273.0),
            phi_sqrt_phi: sqrt_phi * phi,
        })
    }
}

impl LegacyBsim2Sized {
    fn threshold_terms(&self, vds: Value, vbs: Value) -> (Value, Value, Value) {
        if !vds.is_finite() || !vbs.is_finite() {
            return (self.vfb + self.phi, self.phi, self.sqrt_phi_or_default());
        }
        let (phis_b, t1s) = if vbs <= 0.0 {
            let phis_b = self.phi - vbs;
            (phis_b, phis_b.sqrt())
        } else {
            let tmp = self.phi / (self.phi + vbs);
            (self.phi * tmp, self.phi_sqrt_phi / (self.phi + 0.5 * vbs))
        };
        let eta = self.eta0 + self.eta_b * vbs;
        let threshold = self.vfb + self.phi + self.k1 * t1s - self.k2 * phis_b - eta * vds;
        (threshold, phis_b, t1s)
    }

    fn threshold(&self, vds: Value, vbs: Value) -> Value {
        self.threshold_terms(vds, vbs).0
    }

    fn evaluate(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, LegacyBsimRegion) {
        if !vgs.is_finite() || !vds.is_finite() || !vbs.is_finite() {
            return (0.0, LegacyBsimRegion::Cutoff);
        }

        let vbs = vbs.max(2.0 * self.vbb);
        let vgs = vgs.min(2.0 * self.vgg);
        let vds = vds.min(2.0 * self.vdd);

        let (vth, phis_b, t1s) = self.threshold_terms(vds, vbs);
        let ua = self.ua0 + self.ua_b * vbs;
        let ub = self.ub0 + self.ub_b * vbs;
        let u1s = self.u10 + self.u1_b * vbs;
        let vgst = vgs - vth;

        let gg = 1.0 - 1.0 / (1.744 + 0.8364 * phis_b);
        let aa = 1.0 + 0.5 * gg * self.k1 / t1s;
        let inv_aa = 1.0 / aa;
        let mut exp0 = 0.0;
        let mut exp1 = 0.0;
        let mut subthreshold_slope = None;

        let vgeff = if vgst >= self.vghigh || self.n0 == 0.0 {
            vgst
        } else {
            let vof = self.vof0 + self.vof_b * vbs + self.vof_d * vds;
            let n = self.n0 + self.n_b / t1s + self.n_d * vds;
            if !n.is_finite() || n.abs() <= 1e-30 || self.vtm <= 0.0 {
                return (0.0, LegacyBsimRegion::Cutoff);
            }
            subthreshold_slope = Some(n);
            let tmp = 0.5 / (n * self.vtm);
            exp1 = (-vds / self.vtm).max(-30.0).exp();
            let one_minus_exp1 = (1.0 - exp1).max(1.0e-18);
            let sqrt_arg = 2.0 * aa * one_minus_exp1;
            if vgst <= self.vglow {
                exp0 = (0.5 * vof + (vgst * tmp).max(-30.0)).exp();
                sqrt_arg.sqrt() * self.vtm * exp0
            } else {
                exp0 = (0.5 * vof + (self.vglow * tmp).max(-30.0)).exp();
                let con1 = self.vghigh;
                let con3 = sqrt_arg.sqrt() * self.vtm * exp0;
                let con4 = con3 * tmp;
                let sqr_vghigh = self.vghigh * self.vghigh;
                let sqr_vglow = self.vglow * self.vglow;
                let cub_vghigh = sqr_vghigh * self.vghigh;
                let cub_vglow = sqr_vglow * self.vglow;
                let t0 = 2.0 * self.vghigh;
                let t1 = 2.0 * self.vglow;
                let t2 = 3.0 * sqr_vghigh;
                let t3 = 3.0 * sqr_vglow;
                let t4 = self.vghigh - self.vglow;
                let t5 = sqr_vghigh - sqr_vglow;
                let t6 = cub_vghigh - cub_vglow;
                let t7 = con1 - con3;
                let delta = 1.0 / ((t1 - t0) * t6 + (t2 - t3) * t5 + (t0 * t3 - t1 * t2) * t4);
                let coeff_b =
                    (t1 - con4 * t0) * t6 + (con4 * t2 - t3) * t5 + (t0 * t3 - t1 * t2) * t7;
                let coeff_c = (con4 - 1.0) * t6 + (t2 - t3) * t7 + (t3 - con4 * t2) * t4;
                let coeff_d = (t1 - t0) * t7 + (1.0 - con4) * t5 + (con4 * t0 - t1) * t4;
                let coeff_a = sqr_vghigh * (coeff_c + coeff_d * t0);
                (coeff_a + vgst * (coeff_b + vgst * (coeff_c + vgst * coeff_d))) * delta
            }
        };

        if !vgeff.is_finite() || vgeff <= 0.0 {
            return (1.0e-50, LegacyBsimRegion::Cutoff);
        }

        let uvert = (1.0 + vgeff * (ua + vgeff * ub)).max(0.2);
        let inv_uvert = 1.0 / uvert;
        let vc = u1s * inv_aa * inv_uvert * vgeff;
        let sqrt_kk_arg = (1.0 + 2.0 * vc).sqrt();
        let kk = 0.5 * (1.0 + vc + sqrt_kk_arg);
        let inv_kk = 1.0 / kk;
        let vdsat = (vgeff * inv_aa / kk.sqrt()).max(1.0e-18);
        let inv_vdsat = 1.0 / vdsat;

        let beta0 = self.beta0 + self.beta0_b * vbs;
        let betas = self.betas0 + self.betas_b * vbs;
        let beta2 = self.beta20 + self.beta2_b * vbs + self.beta2_g * vgs;
        let beta3 = self.beta30 + self.beta3_b * vbs + self.beta3_g * vgs;
        let beta4 = self.beta40 + self.beta4_b * vbs + self.beta4_g * vgs;
        let beta1 = betas - (beta0 + self.vdd * (beta3 - self.vdd * beta4));

        let t0 = (vds * beta2 * inv_vdsat).min(30.0);
        let t1 = t0.exp();
        let t2 = t1 * t1;
        let tanh = (t2 - 1.0) / (t2 + 1.0);
        let beta = beta0 + beta1 * tanh + vds * (beta3 - beta4 * vds);

        let mut region = LegacyBsimRegion::Saturation;
        let ids = if vgst > self.vglow {
            if vds <= vdsat {
                region = LegacyBsimRegion::Linear;
                let t3 = vds * inv_vdsat;
                let t4 = t3 - 1.0;
                let t2 = 1.0 - self.u1_d * t4 * t4;
                let u1 = u1s * t2;
                let utot = (uvert + u1 * vds).max(0.5);
                let channel = (vgeff - 0.5 * aa * vds) * vds;
                beta / utot * channel
            } else {
                let channel = 0.5 * vgeff * (vgeff * inv_aa * inv_kk);
                let mut ids = beta * inv_uvert * channel;
                ids *= self.hot_electron_factor(vds, vdsat, vbs);
                ids
            }
        } else {
            region = LegacyBsimRegion::Cutoff;
            let n = subthreshold_slope.unwrap_or(1.0);
            let mut ids = beta * self.vtm * self.vtm * exp0 * exp0 * (1.0 - exp1);
            if !n.is_finite() {
                return (0.0, LegacyBsimRegion::Cutoff);
            }
            if vds > vdsat {
                ids *= self.hot_electron_factor(vds, vdsat, vbs);
            }
            ids
        };

        (ids.max(1.0e-50), region)
    }

    fn hot_electron_factor(&self, vds: Value, vdsat: Value, vbs: Value) -> Value {
        if self.ai0 == 0.0 || vds <= vdsat {
            return 1.0;
        }
        let denom = vds - vdsat;
        if denom <= 1e-30 {
            return 1.0;
        }
        let ai = self.ai0 + self.ai_b * vbs;
        let bi = self.bi0 + self.bi_b * vbs;
        let exponent = (bi / denom).min(30.0);
        1.0 + ai * (-exponent).exp()
    }

    fn sqrt_phi_or_default(&self) -> Value {
        if self.phi.is_finite() && self.phi > 0.0 {
            self.phi.sqrt()
        } else {
            1.0
        }
    }
}

fn legacy_cox(tox_um: Value) -> Value {
    if tox_um.is_finite() && tox_um > 0.0 {
        EPS_OX_CGS / (tox_um * 1.0e-4)
    } else {
        0.0
    }
}

fn param(params: &HashMap<String, Value>, name: &str, default: Value) -> Value {
    params
        .get(name)
        .copied()
        .filter(|value| value.is_finite())
        .unwrap_or(default)
}
