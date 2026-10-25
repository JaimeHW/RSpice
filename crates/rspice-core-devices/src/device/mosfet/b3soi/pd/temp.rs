//! Size- and temperature-dependent parameter evaluation for B3SOIPD.
//!
//! Faithful port of ngspice-46 `b3soipdtemp.c` (`B3SOIPDtemp`). One
//! [`B3SoiPdSized`] is computed per (W, L, rth0, cth0) combination; RSpice
//! computes one per device instance and shares it behind an `Arc`.

use super::super::common::temp::SoiSizedCore;
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

        // ngspice clamps the sidewall junction potential model-wide.
        let gate_sidewall_jct_potential = m.gate_sidewall_jct_potential.max(0.1);

        let tnom = m.tnom;
        let tratio = temp / tnom;

        p.temp = temp;
        p.tnom = tnom;
        p.factor1 = (EPSSI / EPSOX * m.tox).sqrt();

        let eg0 = 1.16 - 7.02e-4 * tnom * tnom / (tnom + 1108.0);
        p.eg0 = eg0;
        p.vtm = KB_OVER_Q * temp;

        let eg = 1.16 - 7.02e-4 * temp * temp / (temp + 1108.0);
        // ni in cm^-3.
        let ni = 1.45e10
            * (temp / 300.15)
            * (temp / 300.15).sqrt()
            * (21.5565981 - eg / (2.0 * p.vtm)).exp();
        p.ni = ni;

        p.rbodyext = geom.body_squares * m.rbsh;

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

        p.leff = geom.l - 2.0 * dl;
        if p.leff <= 0.0 {
            return Err("B3SOIPD: effective channel length <= 0".to_string());
        }
        p.weff = geom.w - 2.0 * dw;
        if p.weff <= 0.0 {
            return Err("B3SOIPD: effective channel width <= 0".to_string());
        }
        p.leff_cv = geom.l - 2.0 * dlc;
        if p.leff_cv <= 0.0 {
            return Err("B3SOIPD: effective channel length for C-V <= 0".to_string());
        }
        p.weff_cv = geom.w - 2.0 * dwc;
        if p.weff_cv <= 0.0 {
            return Err("B3SOIPD: effective channel width for C-V <= 0".to_string());
        }
        p.leff_cv_b = geom.l - 2.0 * dlc - m.dlcb;
        if p.leff_cv_b <= 0.0 {
            return Err("B3SOIPD: effective body channel length for C-V <= 0".to_string());
        }
        p.leff_cv_bg = p.leff_cv_b + 2.0 * m.dlbg;
        if p.leff_cv_bg <= 0.0 {
            return Err("B3SOIPD: effective back-gate channel length for C-V <= 0".to_string());
        }
        p.fbody = m.fbody;

        // --- Not binned (lines 170-184) ---
        p.at = m.at;
        p.gamma1 = m.gamma1;
        p.gamma2 = m.gamma2;
        p.vbx = m.vbx;
        p.vbm = m.vbm;
        p.xt = m.xt;
        p.kt1 = m.kt1;
        p.kt1l = m.kt1l;
        p.kt2 = m.kt2;
        p.ua1 = m.ua1;
        p.ub1 = m.ub1;
        p.uc1 = m.uc1;
        p.ute = m.ute;
        p.prt = m.prt;
        p.lii = m.lii;
        p.tii = m.tii;
        p.vrec0 = m.vrec0;
        p.vtun0 = m.vtun0;
        p.ntrecf = m.ntrecf;
        p.ntrecr = m.ntrecr;

        // CV model.
        p.cgsl = m.cgsl;
        p.cgdl = m.cgdl;
        p.ckappa = m.ckappa;
        p.cf = m.cf;
        p.clc = m.clc;
        p.cle = m.cle;
        p.abulk_cv_factor = 1.0 + (p.clc / p.leff).powf(p.cle);

        // --- Binned parameters (lines 199-527) ---
        let (inv_l, inv_w, inv_lw) = if m.bin_unit == 1 {
            (
                1.0e-6 / p.leff,
                1.0e-6 / p.weff,
                1.0e-12 / (p.leff * p.weff),
            )
        } else {
            (1.0 / p.leff, 1.0 / p.weff, 1.0 / (p.leff * p.weff))
        };
        macro_rules! bin {
            ($field:ident) => {
                p.$field = m.$field.eval(inv_l, inv_w, inv_lw);
            };
        }
        bin!(npeak);
        bin!(nsub);
        bin!(ngate);
        bin!(vth0);
        bin!(k1);
        bin!(k2);
        bin!(k3);
        bin!(k3b);
        bin!(vbsa);
        bin!(delp);
        bin!(delvt);
        bin!(kb1);
        bin!(kb3);
        bin!(dvbd0);
        bin!(dvbd1);
        bin!(w0);
        bin!(nlx);
        bin!(dvt0);
        bin!(dvt1);
        bin!(dvt2);
        bin!(dvt0w);
        bin!(dvt1w);
        bin!(dvt2w);
        bin!(u0);
        bin!(ua);
        bin!(ub);
        bin!(uc);
        bin!(vsat);
        bin!(a0);
        bin!(ags);
        bin!(b0);
        bin!(b1);
        bin!(keta);
        bin!(ketas);
        bin!(abp);
        bin!(mxc);
        bin!(adice0);
        bin!(a1);
        bin!(a2);
        bin!(rdsw);
        bin!(prwb);
        bin!(prwg);
        bin!(wr);
        bin!(nfactor);
        bin!(dwg);
        bin!(dwb);
        bin!(voff);
        bin!(eta0);
        bin!(etab);
        bin!(dsub);
        bin!(cit);
        bin!(cdsc);
        bin!(cdscb);
        bin!(cdscd);
        bin!(pclm);
        bin!(pdibl1);
        bin!(pdibl2);
        bin!(pdiblb);
        bin!(drout);
        bin!(pvag);
        bin!(delta);
        bin!(aii);
        bin!(bii);
        bin!(cii);
        bin!(dii);
        bin!(alpha0);
        bin!(alpha1);
        bin!(beta0);
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
        bin!(agidl);
        bin!(bgidl);
        bin!(ngidl);
        bin!(ntun);
        bin!(ndiode);
        bin!(isbjt);
        bin!(isdif);
        bin!(isrec);
        bin!(istun);
        bin!(edl);
        bin!(kbjt1);
        bin!(vsdfb);
        bin!(vsdth);
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
        p.ua += p.ua1 * t0;
        p.ub += p.ub1 * t0;
        p.uc += p.uc1 * t0;
        if p.u0 > 1.0 {
            p.u0 /= 1.0e4;
        }
        p.u0temp = p.u0 * tratio.powf(p.ute);
        p.vsattemp = p.vsat - p.at * t0;
        p.rds0 = (p.rdsw + p.prt * t0) / (p.weff * 1e6).powf(p.wr);

        // (B3SOIPDcheckModel intentionally not ported: paramchk=0 in the
        // supported decks; geometry fatals are caught above.)

        // --- Overlap capacitances (lines 561-566) ---
        p.cgdo = (m.cgdo + p.cf) * p.weff_cv;
        p.cgso = (m.cgso + p.cf) * p.weff_cv;
        p.cgeo = m.cgeo * p.leff_cv;

        if !m.npeak_given && m.gamma1_given {
            let t0 = p.gamma1 * m.cox;
            p.npeak = 3.021e22 * t0 * t0;
        }

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

        // --- Back-gate flat band (586-601) ---
        if p.nsub > 0.0 {
            p.vfbb = -m.mtype * p.vtm * (p.npeak / p.nsub).ln();
        } else {
            p.vfbb = -m.mtype * p.vtm * (-p.npeak * p.nsub / ni / ni).ln();
        }
        if !m.vsdfb_given {
            if p.nsub > 0.0 {
                p.vsdfb = -m.mtype * (p.vtm * (1e20 * p.nsub / ni / ni).ln() - 0.3);
            } else if p.nsub < 0.0 {
                p.vsdfb = -m.mtype * (p.vtm * (-1e20 / p.nsub).ln() + 0.3);
            }
        }

        // --- S/D diffusion Phi & Gamma (603-625) ---
        let sd_phi = 2.0 * p.vtm * (p.nsub.abs() / ni).ln();
        let sd_gamma = 5.753e-12 * p.nsub.abs().sqrt() / m.cbox;
        if !m.vsdth_given {
            if (p.nsub > 0.0 && m.mtype > 0.0) || (p.nsub < 0.0 && m.mtype < 0.0) {
                p.vsdth = p.vsdfb + sd_phi + sd_gamma * sd_phi.sqrt();
            } else {
                p.vsdth = p.vsdfb - sd_phi - sd_gamma * sd_phi.sqrt();
            }
        }
        p.csdmin = if m.csdmin_given {
            m.csdmin
        } else {
            // Cdmin.
            let tmp = (2.0 * EPSSI * sd_phi / (CHARGE_Q * p.nsub.abs() * 1.0e6)).sqrt();
            let tmp1 = EPSSI / tmp;
            tmp1 * m.cbox / (tmp1 + m.cbox)
        };

        // --- Sidewall fringing (628-639) ---
        let t0 = m.csdesw * (1.0 + m.tsi / m.tbox).ln();
        let t1 = geom.source_perimeter - p.weff;
        p.csesw = if t1 > 0.0 { t0 * t1 } else { 0.0 };
        let t1 = geom.drain_perimeter - p.weff;
        p.cdesw = if t1 > 0.0 { t0 * t1 } else { 0.0 };

        // --- Bulk-charge / threshold pre-computation (641-745) ---
        p.phi = 2.0 * p.vtm * (p.npeak / ni).ln();
        p.sqrt_phi = p.phi.sqrt();
        p.phis3 = p.sqrt_phi * p.phi;
        p.xdep0 = (2.0 * EPSSI / (CHARGE_Q * p.npeak * 1.0e6)).sqrt() * p.sqrt_phi;
        p.sqrt_xdep0 = p.xdep0.sqrt();
        p.litl = (3.0 * m.xj * m.tox).sqrt();
        p.vbi = p.vtm * (1.0e20 * p.npeak / (ni * ni)).ln();
        p.cdep0 = (CHARGE_Q * EPSSI * p.npeak * 1.0e6 / 2.0 / p.phi).sqrt();

        if m.k1_given || m.k2_given {
            if !m.k1_given {
                log::warn!("B3SOIPD: k1 should be specified with k2; using 0.53");
                p.k1 = 0.53;
            }
            if !m.k2_given {
                log::warn!("B3SOIPD: k2 should be specified with k1; using -0.0186");
                p.k2 = -0.0186;
            }
        } else {
            if !m.vbx_given {
                p.vbx = p.phi - 7.7348e-4 * p.npeak * p.xt * p.xt;
            }
            if p.vbx > 0.0 {
                p.vbx = -p.vbx;
            }
            if p.vbm > 0.0 {
                p.vbm = -p.vbm;
            }
            if !m.gamma1_given {
                p.gamma1 = 5.753e-12 * p.npeak.sqrt() / m.cox;
            }
            if !m.gamma2_given {
                p.gamma2 = 5.753e-12 * p.nsub.sqrt() / m.cox;
            }
            let t0 = p.gamma1 - p.gamma2;
            let t1 = (p.phi - p.vbx).sqrt() - p.sqrt_phi;
            let t2 = (p.phi * (p.phi - p.vbm)).sqrt() - p.phi;
            p.k2 = t0 * t1 / (2.0 * t2 + p.vbm);
            p.k1 = p.gamma2 - 2.0 * p.k2 * (p.phi - p.vbm).sqrt();
        }

        if p.k2 < 0.0 {
            let t0 = 0.5 * p.k1 / p.k2;
            p.vbsc = 0.9 * (p.phi - t0 * t0);
            p.vbsc = p.vbsc.clamp(-30.0, -3.0);
        } else {
            p.vbsc = -30.0;
        }
        if p.vbsc > p.vbm {
            p.vbsc = p.vbm;
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

        // --- Instance-level depletion-capacitance knots (748-788) ---
        p.csbox = m.cbox * geom.source_area;
        p.csmin = p.csdmin * geom.source_area;
        p.cdbox = m.cbox * geom.drain_area;
        p.cdmin = p.csdmin * geom.drain_area;

        if (p.nsub > 0.0 && m.mtype > 0.0) || (p.nsub < 0.0 && m.mtype < 0.0) {
            let t0 = p.vsdth - p.vsdfb;
            p.sdt1 = p.vsdfb + m.asd * t0;
            let t1 = p.csbox - p.csmin;
            let t2 = t1 / t0 / t0;
            p.st2 = t2 / m.asd;
            p.st3 = t2 / (1.0 - m.asd);
            p.st4 = t0 * t1 * (1.0 + m.asd) / 3.0 - p.csmin * p.vsdfb;

            let t1 = p.cdbox - p.cdmin;
            let t2 = t1 / t0 / t0;
            p.dt2 = t2 / m.asd;
            p.dt3 = t2 / (1.0 - m.asd);
            p.dt4 = t0 * t1 * (1.0 + m.asd) / 3.0 - p.cdmin * p.vsdfb;
        } else {
            let t0 = p.vsdfb - p.vsdth;
            p.sdt1 = p.vsdth + m.asd * t0;
            let t1 = p.csmin - p.csbox;
            let t2 = t1 / t0 / t0;
            p.st2 = t2 / m.asd;
            p.st3 = t2 / (1.0 - m.asd);
            p.st4 = t0 * t1 * (1.0 + m.asd) / 3.0 - p.csbox * p.vsdth;

            let t1 = p.cdmin - p.cdbox;
            let t2 = t1 / t0 / t0;
            p.dt2 = t2 / m.asd;
            p.dt3 = t2 / (1.0 - m.asd);
            p.dt4 = t0 * t1 * (1.0 + m.asd) / 3.0 - p.cdbox * p.vsdth;
        }

        // --- Series resistance (791-806) ---
        let g = m.sheet_resistance * geom.drain_squares;
        p.drain_conductance = if g > 0.0 { 1.0 / g } else { 0.0 };
        let g = m.sheet_resistance * geom.source_squares;
        p.source_conductance = if g > 0.0 { 1.0 / g } else { 0.0 };

        // Keep the clamped junction potential visible to the eval stage.
        let _ = gate_sidewall_jct_potential;

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
