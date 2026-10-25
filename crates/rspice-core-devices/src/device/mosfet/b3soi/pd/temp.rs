//! Size- and temperature-dependent parameter evaluation for B3SOIPD.
//!
//! Faithful port of ngspice-46 `b3soipdtemp.c` (`B3SOIPDtemp`). One
//! [`B3SoiPdSized`] is computed per (W, L, rth0, cth0) combination; RSpice
//! computes one per device instance and shares it behind an `Arc`.

use super::super::common::temp::{
    SoiAmbient, SoiSizedCore, soi_ambient, soi_bin, soi_effective_sizes, soi_junctions,
    soi_mobility_temperature, soi_threshold, soi_unbinned,
};
use super::super::common::{CHARGE_Q, EPSOX, EPSSI, EXPL_THRESHOLD, KB_OVER_Q, MIN_EXPL};
use super::params::B3SoiPdModel;
use crate::Value;

/// pParam + per-instance derived quantities (ngspice `b3soipdSizeDependParam`
/// plus the instance-level fields filled in by `B3SOIPDtemp`).
#[derive(Debug, Clone, Default)]
#[allow(clippy::too_many_arguments)]
pub struct B3SoiPdSized {
    core: SoiSizedCore,

    /// Body CV length `leffCVb = L - 2*dlc - dlcb` (b3soipdtemp.c:206-213).
    pub leff_cv_b: Value,
    /// Back-gate CV length `leffCVbg = leffCVb + 2*dlbg`
    /// (b3soipdtemp.c:215-222).
    pub leff_cv_bg: Value,
    /// Body-charge scaling factor (FBODY model card parameter).
    pub fbody: Value,

    /// PD body-tie width-corrected k1 (`k1eff`, b3soipdtemp.c:829).
    pub k1eff: Value,
    pub delvt: Value,
    pub ketas: Value,
    // PD impact-ionization (binned).
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
    // PD body-diode / parasitic-BJT (binned + derived).
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
    /// Recombination/diffusion length ratios + BJT alpha and Early voltage.
    pub arfabjt: Value,
    pub lratio: Value,
    pub lratiodif: Value,
    pub vearly: Value,

    pub acde: Value,
    pub moin: Value,

    pub vfbzb: Value,
    pub ldeb: Value,
}

super::super::common::deref_core!(B3SoiPdSized => SoiSizedCore);

/// Instance geometry inputs needed by the B3SOIPD sizing pass.
pub type B3SoiPdGeometry = super::super::common::SoiGeometry;

impl B3SoiPdSized {
    /// Port of `B3SOIPDtemp` (b3soipdtemp.c lines 38-813).
    ///
    /// `temp` is the device operating temperature in Kelvin (ngspice
    /// `CKTtemp`).
    pub fn new(model: &B3SoiPdModel, geom: &B3SoiPdGeometry, temp: Value) -> Result<Self, String> {
        let mut p = Self::default();
        let m = model;

        let SoiAmbient {
            tnom,
            tratio,
            eg0,
            ni,
        } = soi_ambient(&mut p, m, temp, geom.body_squares);

        // --- Geometry scaling (lines 119-167) ---
        let ldrn = geom.l;
        let wdrn = geom.w;
        let t0 = ldrn.powf(m.lln);
        let t1 = wdrn.powf(m.lwn);
        let tmp1 = m.ll / t0 + m.lw / t1 + m.lwl / (t0 * t1);
        let dl = m.lint + tmp1;
        let tmp1 = m.llc / t0 + m.lwc / t1 + m.lwlc / (t0 * t1);
        let dlc = m.dlc + tmp1;
        let t2 = ldrn.powf(m.wln);
        let t3 = wdrn.powf(m.wwn);
        let tmp2 = m.wl / t2 + m.ww / t3 + m.wwl / (t2 * t3);
        let dw = m.wint + tmp2;
        let tmp2 = m.wlc / t2 + m.wwc / t3 + m.wwlc / (t2 * t3);
        let dwc = m.dwc + tmp2;

        if m.tox - m.dtoxcv <= 0.0 {
            return Err("B3SOIPD: TOX - DTOXCV must be positive".to_string());
        }

        soi_effective_sizes(&mut p, geom.l, geom.w, dl, dw, dlc, dwc)
            .map_err(|reason| format!("B3SOIPD: {reason}"))?;
        p.leff_cv_b = geom.l - 2.0 * dlc - m.dlcb;
        if p.leff_cv_b <= 0.0 {
            return Err("B3SOIPD: effective body channel length for C-V <= 0".to_string());
        }
        p.leff_cv_bg = p.leff_cv_b + 2.0 * m.dlbg;
        if p.leff_cv_bg <= 0.0 {
            return Err("B3SOIPD: effective back-gate channel length for C-V <= 0".to_string());
        }
        p.fbody = m.fbody;

        soi_unbinned(&mut p, m);
        p.lii = m.lii;
        p.tii = m.tii;
        p.vrec0 = m.vrec0;
        p.vtun0 = m.vtun0;
        p.ntrecf = m.ntrecf;
        p.ntrecr = m.ntrecr;
        p.abulk_cv_factor = 1.0 + (p.clc / p.leff).powf(p.cle);

        let (inv_l, inv_w, inv_lw) = soi_bin(&mut p, m);
        macro_rules! bin {
            ($field:ident) => {
                p.$field = m.$field.eval(inv_l, inv_w, inv_lw);
            };
        }
        bin!(delvt);
        bin!(ketas);
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
        bin!(acde);
        bin!(moin);

        // --- Temperature adjustment (lines 530-552) ---
        let t0 = tratio - 1.0;
        p.uatemp = p.ua; // saved pre-temperature ua/ub/uc for self-heating in eval
        p.ubtemp = p.ub;
        p.uctemp = p.uc;
        p.rds0denom = (p.weff * 1e6).powf(p.wr);
        let nseg = geom.nseg.max(1.0e-30);
        let thermal_width = (p.weff + m.wth0).max(1.0e-30);
        p.rth = geom.rth0 / thermal_width * nseg;
        p.cth = geom.cth0 * thermal_width / nseg;
        let rbody_denom = 2.0 * m.rbody + m.rhalo * p.leff;
        p.rbody = if rbody_denom != 0.0 {
            geom.frbody * m.rbody * m.rhalo / rbody_denom * p.weff / nseg
        } else {
            0.0
        };
        soi_mobility_temperature(&mut p, m, tratio, t0);

        // --- Diode/BJT saturation current temperature scaling (PD form,
        //     b3soipdtemp.c:681-700). PD uses an Eg300/Vtm exponential with the
        //     recombination current scaled by nrecf0 (not ndiode/2). ---
        use super::super::common::EG300;
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
        let _ = eg0;

        // --- PD recombination-length ratios + BJT alpha/Early voltage
        //     (b3soipdtemp.c:932-944). nseg defaults to 1 in the supported decks. ---
        let ln = if m.ln < 1e-15 { 1e-15 } else { m.ln };
        let t0 = -0.5 * p.leff * p.leff / (ln * ln);
        p.arfabjt = t0.exp();
        let t0 = p.lbjt0 * (1.0 / p.leff + 1.0 / ln);
        p.lratio = t0.powf(m.nbjt);
        p.lratiodif = 1.0 + m.ldif0 * t0.powf(m.ndif);
        p.vearly = (p.vabjt + p.aely * p.leff).max(1.0);

        let (k1_defaulted, k2_defaulted) =
            soi_threshold(&mut p, m, ni, geom.source_perimeter, geom.drain_perimeter);
        if k1_defaulted {
            log::warn!("B3SOIPD: k1 should be specified with k2; using 0.53");
        }
        if k2_defaulted {
            log::warn!("B3SOIPD: k2 should be specified with k1; using -0.0186");
        }

        // PD body-tie width correction: k1eff = k1*(1 + k1w1/(weff+k1w2))
        // (b3soipdtemp.c:827-829). With k1w1=k1w2=0 (the supported decks) this
        // reduces to k1eff = k1.
        let k1w_denom = {
            let t0 = p.weff + m.k1w2;
            if t0 < 1e-8 { 1e-8 } else { t0 }
        };
        p.k1eff = p.k1 * (1.0 + m.k1w1 / k1w_denom);

        if m.vth0_given {
            p.vfb = m.mtype * p.vth0 - p.phi - p.k1eff * p.sqrt_phi;
        } else {
            p.vfb = -1.0;
            p.vth0 = m.mtype * (p.vfb + p.phi + p.k1eff * p.sqrt_phi);
        }
        let t1 = (EPSSI / EPSOX * m.tox * p.xdep0).sqrt();
        let t0 = (-0.5 * p.dsub * p.leff / t1).exp();
        p.theta0vb0 = t0 + 2.0 * t0 * t0;
        let t0 = (-0.5 * p.drout * p.leff / t1).exp();
        let t2 = t0 + 2.0 * t0 * t0;
        p.theta_rout = p.pdibl1 * t2 + p.pdibl2;

        // CAPMOD=3 vfbzb/charge-thickness pre-computation (b3soipdtemp.c:944-989).
        let tmp = p.xdep0.sqrt();
        let tmp1 = p.vbi - p.phi;
        let tmp2 = p.factor1 * tmp;

        let t0 = -0.5 * p.dvt1w * p.weff * p.leff / tmp2;
        let t2 = if t0 > -EXPL_THRESHOLD {
            let t1 = t0.exp();
            t1 * (1.0 + 2.0 * t1)
        } else {
            let t1 = MIN_EXPL;
            t1 * (1.0 + 2.0 * t1)
        };
        let t2 = p.dvt0w * t2 * tmp1;

        let t0 = -0.5 * p.dvt1 * p.leff / tmp2;
        let t3 = if t0 > -EXPL_THRESHOLD {
            let t1 = t0.exp();
            t1 * (1.0 + 2.0 * t1)
        } else {
            let t1 = MIN_EXPL;
            t1 * (1.0 + 2.0 * t1)
        };
        let t3 = p.dvt0 * t3 * tmp1;

        let t4 = (m.tox - m.dtoxcv) * p.phi / (p.weff + p.w0);
        let t0 = (1.0 + p.nlx / p.leff).sqrt();
        let t5 = p.k1eff * (t0 - 1.0) * p.sqrt_phi + (p.kt1 + p.kt1l / p.leff) * (tratio - 1.0);
        let tmp3 = m.mtype * p.vth0 - t2 - t3 + p.k3 * t4 + t5;
        p.vfbzb = tmp3 - p.phi - p.k1eff * p.sqrt_phi;

        let vtm0 = KB_OVER_Q * tnom;
        p.ldeb = (EPSSI * vtm0 / (CHARGE_Q * p.npeak * 1.0e6)).sqrt() / 3.0;
        p.acde *= (p.npeak / 2.0e16).powf(-0.25);

        p.min_isub = 5.0e-2 * p.weff * m.tsi * p.isdif.max(p.isrec);

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

    /// The values B3SOIPD's load re-evaluates at a self-heated temperature,
    /// in the load's own forms (b3soipdld.c:592-733). Everything else keeps
    /// its ambient value: the load takes `phi` from the instance, which the
    /// operating point resets to the ambient one (b3soipdld.c:361, 629-634),
    /// so `Xdep0` and all the temperature pass derives from `phi`, `vtm` or
    /// `ni` stay as computed at the ambient temperature.
    pub(crate) fn self_heated_at(&self, model: &B3SoiPdModel, temp: Value) -> Self {
        use super::super::common::EG300;
        let m = model;
        let mut p = self.clone();
        let temp_ratio = temp * (1.0 / p.tnom);
        let t3 = temp_ratio - 1.0;

        p.temp = temp;
        p.vtm = KB_OVER_Q * temp;
        let eg = 1.16 - 7.02e-4 * (temp * temp) / (1108.0 + temp);
        let ni =
            1.45e10 * temp * temp.sqrt() * 1.9230584e-4 * (21.5565981 - eg / (2.0 * p.vtm)).exp();
        p.ni = ni;
        p.vbi = p.vtm * (1.0e20 * p.npeak / (ni * ni)).ln();
        p.vfbb = if p.nsub > 0.0 {
            -m.mtype * p.vtm * (p.npeak / p.nsub).ln()
        } else {
            -m.mtype * p.vtm * (-p.npeak * p.nsub / ni / ni).ln()
        };

        let t4 = EG300 / p.vtm * t3;
        let bjt_scale = (m.xbjt * t4 / p.ndiode).exp();
        p.ahli0 = p.ahli * bjt_scale;
        p.jbjt = p.isbjt * bjt_scale;
        p.jdif = p.isdif * (m.xdif * t4 / p.ndiode).exp();
        p.jrec = p.isrec * (m.xrec * t4 / p.nrecf0).exp();
        p.jtun = p.istun * (m.xtun * t3).exp();

        p.u0temp = p.u0 * temp_ratio.powf(p.ute);
        p.vsattemp = p.vsat - p.at * t3;
        p.rds0 = (p.rdsw + p.prt * t3) / p.rds0denom;
        p.ua = p.uatemp + p.ua1 * t3;
        p.ub = p.ubtemp + p.ub1 * t3;
        p.uc = p.uctemp + p.uc1 * t3;
        p
    }
}
