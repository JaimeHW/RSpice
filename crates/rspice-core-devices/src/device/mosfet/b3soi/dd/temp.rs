//! Size- and temperature-dependent parameter evaluation for B3SOIDD.
//!
//! Faithful port of ngspice-46 `b3soiddtemp.c` (`B3SOIDDtemp`). One
//! [`B3SoiDdSized`] is computed per (W, L, rth0, cth0) combination; RSpice
//! computes one per device instance and shares it behind an `Arc`.

use super::super::common::temp::{
    SoiAmbient, SoiSizedCore, soi_ambient, soi_bin, soi_effective_sizes, soi_junctions,
    soi_mobility_temperature, soi_threshold, soi_unbinned,
};
use super::super::common::{B3SoiDialect, CHARGE_Q, EG300, EPSOX, EPSSI, KB_OVER_Q};
use super::params::B3SoiDdModel;
use crate::Value;

/// pParam + per-instance derived quantities (ngspice `b3soiddSizeDependParam`
/// plus the instance-level fields filled in by `B3SOIDDtemp`).
#[derive(Debug, Clone, Default)]
#[allow(clippy::too_many_arguments)]
pub struct B3SoiDdSized {
    core: SoiSizedCore,

    pub leff_cv_b: Value,
    pub leff_cv_bg: Value,
    pub fbody: Value,
    pub nseg: Value,

    pub ketas: Value,
    pub noff: Value,
    pub delvt: Value,
    /// Xyce CAPMOD 3: the centroid exponent `ACDE*(NCH/2e16)^-0.25`, `MOIN`,
    /// and the Debye length `ldeb`.
    pub acde: Value,
    pub moin: Value,
    pub ldeb: Value,
    pub beta1: Value,
    pub beta2: Value,
    pub vdsatii0: Value,
    pub esatii: Value,
    pub sii0: Value,
    pub sii1: Value,
    pub sii2: Value,
    pub siid: Value,
    pub fbjtii: Value,
    pub lii: Value,
    pub tii: Value,
    pub nrecf0: Value,
    pub nrecr0: Value,
    pub ahli: Value,
    pub ahli0: Value,
    pub lbjt0: Value,
    pub vabjt: Value,
    pub aely: Value,
    pub vrec0: Value,
    pub vtun0: Value,
    pub ntrecf: Value,
    pub ntrecr: Value,
    pub arfabjt: Value,
    pub lratio: Value,
    pub lratiodif: Value,
    pub vearly: Value,
    pub xrcrg1: Value,
    pub xrcrg2: Value,

    pub grgeltd: Value,
}

super::super::common::deref_core!(B3SoiDdSized => SoiSizedCore);

/// Instance geometry inputs needed by the B3SOIDD sizing pass.
pub type B3SoiDdGeometry = super::super::common::SoiGeometry;

impl B3SoiDdSized {
    /// Port of `B3SOIDDtemp` (b3soiddtemp.c lines 38-813).
    ///
    /// `temp` is the device operating temperature in Kelvin (ngspice
    /// `CKTtemp`).
    pub fn new(model: &B3SoiDdModel, geom: &B3SoiDdGeometry, temp: Value) -> Result<Self, String> {
        let mut p = Self::default();
        let m = model;

        let SoiAmbient {
            tnom, tratio, ni, ..
        } = soi_ambient(&mut p, m, temp, geom.body_squares);

        // --- Geometry scaling (lines 119-167) ---
        let ldrn = geom.l;
        let wdrn = geom.w;
        let t0 = ldrn.powf(m.lln);
        let t1 = wdrn.powf(m.lwn);
        let tmp1 = m.ll / t0 + m.lw / t1 + m.lwl / (t0 * t1);
        let dl = m.lint + tmp1;
        let dlc = m.dlc + tmp1;
        let t2 = ldrn.powf(m.wln);
        let t3 = wdrn.powf(m.wwn);
        let tmp2 = m.wl / t2 + m.ww / t3 + m.wwl / (t2 * t3);
        let dw = m.wint + tmp2;
        let dwc = m.dwc + tmp2;

        soi_effective_sizes(&mut p, geom.l, geom.w, dl, dw, dlc, dwc)
            .map_err(|reason| format!("B3SOIDD: {reason}"))?;
        p.leff_cv_b = geom.l - 2.0 * dlc - m.dlcb;
        if p.leff_cv_b <= 0.0 {
            return Err("B3SOIDD: effective body channel length for C-V <= 0".to_string());
        }
        p.leff_cv_bg = p.leff_cv_b + 2.0 * m.dlbg;
        if p.leff_cv_bg <= 0.0 {
            return Err("B3SOIDD: effective back-gate channel length for C-V <= 0".to_string());
        }
        p.fbody = m.fbody;

        soi_unbinned(&mut p, m);
        p.lii = m.lii;
        p.tii = m.tii;
        p.vrec0 = m.vrec0;
        p.vtun0 = m.vtun0;
        p.ntrecf = m.ntrecf;
        p.ntrecr = m.ntrecr;
        p.abulk_cv_factor = match m.dialect {
            B3SoiDialect::Ngspice => (1.0 + p.clc / p.leff).powf(p.cle),
            B3SoiDialect::Xyce => 1.0 + (p.clc / p.leff).powf(p.cle),
        };

        let (inv_l, inv_w, inv_lw) = soi_bin(&mut p, m);
        macro_rules! bin {
            ($field:ident) => {
                p.$field = m.$field.eval(inv_l, inv_w, inv_lw);
            };
        }
        bin!(ketas);
        bin!(noff);
        bin!(delvt);
        bin!(acde);
        bin!(moin);
        bin!(beta1);
        bin!(beta2);
        bin!(vdsatii0);
        bin!(esatii);
        bin!(sii0);
        bin!(sii1);
        bin!(sii2);
        bin!(siid);
        bin!(fbjtii);
        bin!(nrecf0);
        bin!(nrecr0);
        bin!(ahli);
        bin!(lbjt0);
        bin!(vabjt);
        bin!(aely);
        bin!(xrcrg1);
        bin!(xrcrg2);

        // --- Temperature adjustment (lines 530-552) ---
        let t0 = tratio - 1.0;
        p.uatemp = p.ua; // saved pre-temperature ua/ub/uc for self-heating in eval
        p.ubtemp = p.ub;
        p.uctemp = p.uc;
        p.rds0denom = (p.weff * 1e6).powf(p.wr);
        let nseg = geom.nseg.max(1.0e-30);
        p.nseg = nseg;
        let thermal_width = (p.weff + m.wth0).max(1.0e-30);
        p.rth = geom.rth0 / thermal_width * nseg;
        p.cth = geom.cth0 * thermal_width / nseg;
        let rbody_denom = 2.0 * m.rbody + m.rhalo * p.leff;
        p.rbody = if rbody_denom != 0.0 {
            geom.frbody * m.rbody * m.rhalo / rbody_denom * p.weff / nseg
        } else {
            0.0
        };
        let gate_length = geom.l - m.xgl;
        let gate_resistance = m.rshg * (m.xgw + p.weff / (3.0 * m.ngcon)) / (m.ngcon * gate_length);
        p.grgeltd = if gate_resistance > 0.0 {
            1.0 / gate_resistance
        } else {
            if m.rgate_mod != 0 {
                log::warn!("B3SOIDD: gate conductance reset to 1.0e3 mho");
            }
            1.0e3
        };
        soi_mobility_temperature(&mut p, m, tratio, t0);

        // --- Diode/BJT saturation current temperature scaling (Xyce v3.2). ---
        let t4 = EG300 / p.vtm * (tratio - 1.0);
        let t0 = (m.xbjt * t4 / p.ndiode).exp();
        let t1 = (m.xdif * t4 / p.ndiode).exp();
        let t2 = (m.xrec * t4 / p.nrecf0).exp();
        p.ahli0 = p.ahli * t0;
        p.jbjt = p.isbjt * t0;
        p.jdif = p.isdif * t1;
        p.jrec = p.isrec * t2;
        let t0 = (m.xtun * (tratio - 1.0)).exp();
        p.jtun = p.istun * t0;

        let ln = if m.ln < 1.0e-15 { 1.0e-15 } else { m.ln };
        let t0 = -0.5 * p.leff * p.leff / (ln * ln);
        p.arfabjt = t0.exp();
        let t0 = p.lbjt0 * (1.0 / p.leff + 1.0 / ln);
        p.lratio = t0.powf(m.nbjt);
        p.lratiodif = 1.0 + m.ldif0 * t0.powf(m.ndif);
        p.vearly = (p.vabjt + p.aely * p.leff).max(1.0);

        let (k1_defaulted, k2_defaulted) =
            soi_threshold(&mut p, m, ni, geom.source_perimeter, geom.drain_perimeter);
        if k1_defaulted {
            log::warn!("B3SOIDD: k1 should be specified with k2; using 0.53");
        }
        if k2_defaulted {
            log::warn!("B3SOIDD: k2 should be specified with k1; using -0.0186");
        }

        if m.vth0_given {
            p.vfb = m.mtype * p.vth0 - p.phi - p.k1 * p.sqrt_phi;
        } else {
            p.vfb = -1.0;
            p.vth0 = m.mtype * (p.vfb + p.phi + p.k1 * p.sqrt_phi);
        }
        let t1 = (EPSSI / EPSOX * m.tox * p.xdep0).sqrt();
        let t0 = (-0.5 * p.dsub * p.leff / t1).exp();
        p.theta0vb0 = t0 + 2.0 * t0 * t0;
        let t0 = (-0.5 * p.drout * p.leff / t1).exp();
        let t2 = t0 + 2.0 * t0 * t0;
        p.theta_rout = p.pdibl1 * t2 + p.pdibl2;

        p.min_isub = 5.0e-2 * p.weff * m.tsi * p.isdif.max(p.isrec);

        // Xyce refuses a non-positive CV oxide thickness on every card
        // (N_DEV_MOSFET_B3SOI.C:15900).
        if m.dialect == B3SoiDialect::Xyce && m.tox - m.dtoxcv <= 0.0 {
            return Err("B3SOIDD: TOX - DTOXCV must be positive".to_string());
        }
        // Xyce CAPMOD 3 charge centroid (N_DEV_MOSFET_B3SOI.C:8032-8033, 8463).
        p.acde *= (p.npeak / 2.0e16).powf(-0.25);
        let vtm0 = KB_OVER_Q * tnom;
        p.ldeb = (EPSSI * vtm0 / (CHARGE_Q * p.npeak * 1.0e6)).sqrt() / 3.0;

        soi_junctions(
            &mut p,
            m,
            geom.source_area,
            geom.drain_area,
            geom.source_squares,
            geom.drain_squares,
        );

        Ok(p)
    }

    /// Xyce B3SOI self-heating temperature update.
    ///
    /// `N_DEV_MOSFET_B3SOI.C` recomputes only the local temperature-dependent
    /// quantities inside the load equation when `SHMOD` is active. The rest of
    /// the size-dependent cache, including `phi`, `Xdep0`, `vsdfb`/`vsdth`,
    /// depletion spline knots, geometry, and overlap capacitances, remains the
    /// ambient `paramPtr` state.
    pub fn self_heated_at(&self, model: &B3SoiDdModel, temp: Value) -> Self {
        let mut p = self.clone();
        let tratio = temp / p.tnom;
        let temp_ratio_m1 = tratio - 1.0;

        p.temp = temp;
        p.vtm = KB_OVER_Q * temp;

        let eg = 1.16 - 7.02e-4 * temp * temp / (temp + 1108.0);
        let ni = 1.45e10
            * (temp / 300.15)
            * (temp / 300.15).sqrt()
            * (21.5565981 - eg / (2.0 * p.vtm)).exp();
        p.ni = ni;

        p.vbi = p.vtm * (1.0e20 * p.npeak / (ni * ni)).ln();
        if p.nsub > 0.0 {
            p.vfbb = -model.mtype * p.vtm * (p.npeak / p.nsub).ln();
        } else {
            p.vfbb = -model.mtype * p.vtm * (-p.npeak * p.nsub / (ni * ni)).ln();
        }

        let t4 = EG300 / p.vtm * temp_ratio_m1;
        let bjt_scale = (model.xbjt * t4 / p.ndiode).exp();
        let dif_scale = (model.xdif * t4 / p.ndiode).exp();
        let rec_scale = (model.xrec * t4 / p.nrecf0).exp();
        p.ahli0 = p.ahli * bjt_scale;
        p.jbjt = p.isbjt * bjt_scale;
        p.jdif = p.isdif * dif_scale;
        p.jrec = p.isrec * rec_scale;
        p.jtun = p.istun * (model.xtun * temp_ratio_m1).exp();

        p.u0temp = p.u0 * tratio.powf(p.ute);
        p.vsattemp = p.vsat - p.at * temp_ratio_m1;
        p.rds0 = (p.rdsw + p.prt * temp_ratio_m1) / p.rds0denom;
        p.ua = p.uatemp + p.ua1 * temp_ratio_m1;
        p.ub = p.ubtemp + p.ub1 * temp_ratio_m1;
        p.uc = p.uctemp + p.uc1 * temp_ratio_m1;

        p
    }
}
