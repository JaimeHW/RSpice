//! Size- and temperature-dependent parameter evaluation for BSIM3v3.3.
//!
//! Faithful port of ngspice-46 `b3temp.c` (`BSIM3temp`) plus the always-on
//! fatal checks of `b3check.c` (`BSIM3checkModel`). The split mirrors the C:
//!
//! - [`Bsim3v3ModelTemp`]: the per-model temperature block (b3temp.c lines
//!   48-149) — `vtm`, `vcrit`, `factor1`, junction saturation-current and
//!   capacitance temperature scaling, `PhiB`/`PhiBSW`/`PhiBSWG` clamps.
//! - [`Bsim3v3SizeDep`]: the `bsim3SizeDependParam` knot (lines 171-793),
//!   keyed by the drawn (W, L); see [`SizeDepCache`].
//! - [`Bsim3v3InstTemp`]: the instance tail (lines 796-889) — `delvto`,
//!   `mulu0`, series conductances, and the `ijth` diode-limiting anchors
//!   (`vjsm`/`IsEvjsm`).
//!
//! The instance-adjusted NQS Elmore constant `tconst` is ported for AC-only
//! charge-deficit (`ACNQSMOD=1`) and transient charge-deficit (`NQSMOD=1`)
//! support. It mirrors the actual ngspice-46 instance-tail denominator at
//! b3temp.c:803-804 for compatibility.

use super::common::{
    CHARGE_Q, CONST_ROOT2, CONST_VT0, EPSOX, EPSSI, EXP_THRESHOLD, KB_OVER_Q, MIN_EXP,
};
use super::params::Bsim3v3Model;
use crate::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Per-model temperature data (b3temp.c lines 48-149).
#[derive(Debug, Clone, Default)]
pub struct Bsim3v3ModelTemp {
    /// Device temperature in Kelvin (ngspice `CKTtemp`).
    pub temp: Value,
    /// TNOM in Kelvin.
    pub tnom: Value,
    /// `CKTtemp / tnom - 1` (the `TempRatio` of b3ld.c:590).
    pub temp_ratio_m1: Value,
    /// `KboQ * CKTtemp` (`BSIM3vtm`).
    pub vtm: Value,
    /// `KboQ * tnom`.
    pub vtm0: Value,
    /// Energy gap at TNOM.
    pub eg0: Value,
    /// Intrinsic carrier density at TNOM (cm^-3).
    pub ni: Value,
    /// `CONSTvt0 * ln(CONSTvt0 / (sqrt(2) * 1e-14))` (`BSIM3vcrit`).
    pub vcrit: Value,
    /// `sqrt(EPSSI / EPSOX * tox)` (`BSIM3factor1`).
    pub factor1: Value,
    /// `js` scaled to the device temperature, clamped >= 0.
    pub jct_temp_sat_cur_density: Value,
    /// `jsw` scaled to the device temperature, clamped >= 0.
    pub jct_sidewall_temp_sat_cur_density: Value,
    /// `cj * (1 + tcj*delTemp)` clamped >= 0.
    pub unit_area_temp_jct_cap: Value,
    /// `cjsw * (1 + tcjsw*delTemp)` clamped >= 0.
    pub unit_length_sidewall_temp_jct_cap: Value,
    /// `cjswg * (1 + tcjswg*delTemp)` clamped >= 0.
    pub unit_length_gate_sidewall_temp_jct_cap: Value,
    /// `pb - tpb*delTemp`, floored at 0.01 (`BSIM3PhiB`).
    pub phi_b: Value,
    /// `pbsw - tpbsw*delTemp`, floored at 0.01 (`BSIM3PhiBSW`).
    pub phi_bsw: Value,
    /// `pbswg - tpbswg*delTemp`, floored at 0.01 (`BSIM3PhiBSWG`).
    pub phi_bswg: Value,
}

impl Bsim3v3ModelTemp {
    /// Port of the model-level head of `BSIM3temp` (b3temp.c lines 48-149).
    ///
    /// `temp` is the device operating temperature in Kelvin (ngspice
    /// `CKTtemp`). The `pb`/`pbsw`/`pbswg` < 0.1 clamps (lines 49-60) are
    /// applied to local copies; the model card itself stays immutable.
    pub fn new(model: &Bsim3v3Model, temp: Value) -> Self {
        let m = model;
        let bulk_jct_potential = m.bulk_jct_potential.max(0.1);
        let sidewall_jct_potential = m.sidewall_jct_potential.max(0.1);
        let gate_sidewall_jct_potential = m.gate_sidewall_jct_potential.max(0.1);

        let tnom = m.tnom;
        let vtm0 = KB_OVER_Q * tnom;
        let eg0 = 1.16 - 7.02e-4 * tnom * tnom / (tnom + 1108.0);
        let ni = 1.45e10
            * (tnom / 300.15)
            * (tnom / 300.15).sqrt()
            * (21.5565981 - eg0 / (2.0 * vtm0)).exp();

        let vcrit = CONST_VT0 * (CONST_VT0 / (CONST_ROOT2 * 1.0e-14)).ln();
        let factor1 = (EPSSI / EPSOX * m.tox).sqrt();

        let vtm = KB_OVER_Q * temp;
        let eg = 1.16 - 7.02e-4 * temp * temp / (temp + 1108.0);
        let (mut jct_temp, mut jct_sw_temp);
        if temp != tnom {
            let t0 = eg0 / vtm0 - eg / vtm + m.jct_temp_exponent * (temp / tnom).ln();
            let t1 = (t0 / m.jct_emission_coeff).exp();
            jct_temp = m.jct_sat_cur_density * t1;
            jct_sw_temp = m.jct_sidewall_sat_cur_density * t1;
        } else {
            jct_temp = m.jct_sat_cur_density;
            jct_sw_temp = m.jct_sidewall_sat_cur_density;
        }
        if jct_temp < 0.0 {
            jct_temp = 0.0;
        }
        if jct_sw_temp < 0.0 {
            jct_sw_temp = 0.0;
        }

        // Temperature dependence of the D/B and S/B diode capacitances
        // (b3temp.c:104-148).
        let del_temp = temp - tnom;
        let cap = |c: Value, tc: Value| {
            let t0 = tc * del_temp;
            if t0 >= -1.0 {
                c * (1.0 + t0)
            } else if c > 0.0 {
                log::warn!(
                    "BSIM3: temperature effect has caused a junction cap to be negative; clamped to zero"
                );
                0.0
            } else {
                c * (1.0 + t0)
            }
        };
        let unit_area_temp_jct_cap = cap(m.unit_area_jct_cap, m.tcj);
        let unit_length_sidewall_temp_jct_cap = cap(m.unit_length_sidewall_jct_cap, m.tcjsw);
        let unit_length_gate_sidewall_temp_jct_cap =
            cap(m.unit_length_gate_sidewall_jct_cap, m.tcjswg);

        let phi_b = (bulk_jct_potential - m.tpb * del_temp).max(0.01);
        let phi_bsw = (sidewall_jct_potential - m.tpbsw * del_temp).max(0.01);
        let phi_bswg = (gate_sidewall_jct_potential - m.tpbswg * del_temp).max(0.01);

        Self {
            temp,
            tnom,
            temp_ratio_m1: temp / tnom - 1.0,
            vtm,
            vtm0,
            eg0,
            ni,
            vcrit,
            factor1,
            jct_temp_sat_cur_density: jct_temp,
            jct_sidewall_temp_sat_cur_density: jct_sw_temp,
            unit_area_temp_jct_cap,
            unit_length_sidewall_temp_jct_cap,
            unit_length_gate_sidewall_temp_jct_cap,
            phi_b,
            phi_bsw,
            phi_bswg,
        }
    }
}

/// Size-dependent parameter knot (`bsim3SizeDependParam`), one per drawn
/// (W, L) pair per temperature pass (ngspice rebuilds the knot list inside
/// every `BSIM3temp` call).
#[derive(Debug, Clone, Default)]
pub struct Bsim3v3SizeDep {
    pub width: Value,
    pub length: Value,

    // Binned values (b3temp.c:252-588), post temperature adjustment.
    pub cdsc: Value,
    pub cdscb: Value,
    pub cdscd: Value,
    pub cit: Value,
    pub nfactor: Value,
    pub xj: Value,
    pub vsat: Value,
    pub at: Value,
    pub a0: Value,
    pub ags: Value,
    pub a1: Value,
    pub a2: Value,
    pub keta: Value,
    pub nsub: Value,
    pub npeak: Value,
    pub ngate: Value,
    pub gamma1: Value,
    pub gamma2: Value,
    pub vbx: Value,
    pub vbi: Value,
    pub vbm: Value,
    pub vbsc: Value,
    pub xt: Value,
    pub phi: Value,
    pub litl: Value,
    pub k1: Value,
    pub kt1: Value,
    pub kt1l: Value,
    pub kt2: Value,
    pub k2: Value,
    pub k3: Value,
    pub k3b: Value,
    pub w0: Value,
    pub nlx: Value,
    pub dvt0: Value,
    pub dvt1: Value,
    pub dvt2: Value,
    pub dvt0w: Value,
    pub dvt1w: Value,
    pub dvt2w: Value,
    pub drout: Value,
    pub dsub: Value,
    pub vth0: Value,
    pub ua: Value,
    pub ua1: Value,
    pub ub: Value,
    pub ub1: Value,
    pub uc: Value,
    pub uc1: Value,
    pub u0: Value,
    pub ute: Value,
    pub voff: Value,
    pub vfb: Value,
    pub delta: Value,
    pub rdsw: Value,
    pub rds0: Value,
    pub prwg: Value,
    pub prwb: Value,
    pub prt: Value,
    pub eta0: Value,
    pub etab: Value,
    pub pclm: Value,
    pub pdibl1: Value,
    pub pdibl2: Value,
    pub pdiblb: Value,
    pub pscbe1: Value,
    pub pscbe2: Value,
    pub pvag: Value,
    pub wr: Value,
    pub dwg: Value,
    pub dwb: Value,
    pub b0: Value,
    pub b1: Value,
    pub alpha0: Value,
    pub alpha1: Value,
    pub beta0: Value,

    // CV model.
    pub elm: Value,
    pub cgsl: Value,
    pub cgdl: Value,
    pub ckappa: Value,
    pub cf: Value,
    pub clc: Value,
    pub cle: Value,
    pub vfbcv: Value,
    pub noff: Value,
    pub voffcv: Value,
    pub acde: Value,
    pub moin: Value,

    // Pre-calculated constants.
    pub dw: Value,
    pub dl: Value,
    pub leff: Value,
    pub weff: Value,
    pub dwc: Value,
    pub dlc: Value,
    pub leff_cv: Value,
    pub weff_cv: Value,
    pub abulk_cv_factor: Value,
    pub cgso: Value,
    pub cgdo: Value,
    pub cgbo: Value,
    pub u0temp: Value,
    pub vsattemp: Value,
    pub sqrt_phi: Value,
    pub phis3: Value,
    pub xdep0: Value,
    pub sqrt_xdep0: Value,
    pub theta0vb0: Value,
    pub theta_rout: Value,
    pub cdep0: Value,
    pub vfbzb: Value,
    pub ldeb: Value,
    pub k1ox: Value,
    pub k2ox: Value,
}

impl Bsim3v3SizeDep {
    /// Port of the `Size_Not_Found` block of `BSIM3temp` (b3temp.c lines
    /// 180-793) for one drawn (W, L), including the embedded
    /// `BSIM3checkModel` call (fatal checks always run; the silent value
    /// fixups run only under `paramchk=1`, exactly as b3check.c does).
    pub fn new(
        model: &Bsim3v3Model,
        mt: &Bsim3v3ModelTemp,
        ldrn: Value,
        wdrn: Value,
    ) -> Result<Self, String> {
        let m = model;
        let mut p = Self {
            width: wdrn,
            length: ldrn,
            ..Self::default()
        };
        let tratio = mt.temp / mt.tnom;

        // --- Geometry scaling (lines 185-237) ---
        let t0 = ldrn.powf(m.lln);
        let t1 = wdrn.powf(m.lwn);
        let tmp1 = m.ll / t0 + m.lw / t1 + m.lwl / (t0 * t1);
        p.dl = m.lint + tmp1;
        let tmp2 = m.llc / t0 + m.lwc / t1 + m.lwlc / (t0 * t1);
        p.dlc = m.dlc + tmp2;

        let t2 = ldrn.powf(m.wln);
        let t3 = wdrn.powf(m.wwn);
        let tmp1 = m.wl / t2 + m.ww / t3 + m.wwl / (t2 * t3);
        p.dw = m.wint + tmp1;
        let tmp2 = m.wlc / t2 + m.wwc / t3 + m.wwlc / (t2 * t3);
        p.dwc = m.dwc + tmp2;

        p.leff = ldrn + m.xl - 2.0 * p.dl;
        if p.leff <= 0.0 {
            return Err("BSIM3: effective channel length <= 0".to_string());
        }
        p.weff = wdrn + m.xw - 2.0 * p.dw;
        if p.weff <= 0.0 {
            return Err("BSIM3: effective channel width <= 0".to_string());
        }
        p.leff_cv = ldrn + m.xl - 2.0 * p.dlc;
        if p.leff_cv <= 0.0 {
            return Err("BSIM3: effective channel length for C-V <= 0".to_string());
        }
        p.weff_cv = wdrn + m.xw - 2.0 * p.dwc;
        if p.weff_cv <= 0.0 {
            return Err("BSIM3: effective channel width for C-V <= 0".to_string());
        }

        // --- Binned parameters (lines 240-588) ---
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
        bin!(cdsc);
        bin!(cdscb);
        bin!(cdscd);
        bin!(cit);
        bin!(nfactor);
        bin!(xj);
        bin!(vsat);
        bin!(at);
        bin!(a0);
        bin!(ags);
        bin!(a1);
        bin!(a2);
        bin!(keta);
        bin!(nsub);
        bin!(npeak);
        bin!(ngate);
        bin!(gamma1);
        bin!(gamma2);
        bin!(vbx);
        bin!(vbm);
        bin!(xt);
        bin!(vfb);
        bin!(k1);
        bin!(kt1);
        bin!(kt1l);
        bin!(k2);
        bin!(kt2);
        bin!(k3);
        bin!(k3b);
        bin!(w0);
        bin!(nlx);
        bin!(dvt0);
        bin!(dvt1);
        bin!(dvt2);
        bin!(dvt0w);
        bin!(dvt1w);
        bin!(dvt2w);
        bin!(drout);
        bin!(dsub);
        bin!(vth0);
        bin!(ua);
        bin!(ua1);
        bin!(ub);
        bin!(ub1);
        bin!(uc);
        bin!(uc1);
        bin!(u0);
        bin!(ute);
        bin!(voff);
        bin!(delta);
        bin!(rdsw);
        bin!(prwg);
        bin!(prwb);
        bin!(prt);
        bin!(eta0);
        bin!(etab);
        bin!(pclm);
        bin!(pdibl1);
        bin!(pdibl2);
        bin!(pdiblb);
        bin!(pscbe1);
        bin!(pscbe2);
        bin!(pvag);
        bin!(wr);
        bin!(dwg);
        bin!(dwb);
        bin!(b0);
        bin!(b1);
        bin!(alpha0);
        bin!(alpha1);
        bin!(beta0);
        // CV model.
        bin!(elm);
        bin!(cgsl);
        bin!(cgdl);
        bin!(ckappa);
        bin!(cf);
        bin!(clc);
        bin!(cle);
        bin!(vfbcv);
        bin!(acde);
        bin!(moin);
        bin!(noff);
        bin!(voffcv);

        p.abulk_cv_factor = 1.0 + (p.clc / p.leff_cv).powf(p.cle);

        // --- Temperature adjustment (lines 594-606) ---
        let t0 = tratio - 1.0;
        p.ua += p.ua1 * t0;
        p.ub += p.ub1 * t0;
        p.uc += p.uc1 * t0;
        if p.u0 > 1.0 {
            p.u0 /= 1.0e4;
        }
        p.u0temp = p.u0 * tratio.powf(p.ute);
        p.vsattemp = p.vsat - p.at * t0;
        p.rds0 = (p.rdsw + p.prt * t0) / (p.weff * 1e6).powf(p.wr);

        // --- BSIM3checkModel (b3check.c), called at b3temp.c:608 ---
        p.check_model(m)?;

        // --- Overlap capacitances (lines 614-618) ---
        p.cgdo = (m.cgdo + p.cf) * p.weff_cv;
        p.cgso = (m.cgso + p.cf) * p.weff_cv;
        p.cgbo = m.cgbo * p.leff_cv;

        if !m.npeak_given && m.gamma1_given {
            let t0 = p.gamma1 * m.cox;
            p.npeak = 3.021e22 * t0 * t0;
        }

        // --- Bulk-charge / threshold pre-computation (lines 629-649) ---
        p.phi = 2.0 * mt.vtm0 * (p.npeak / mt.ni).ln();
        p.sqrt_phi = p.phi.sqrt();
        p.phis3 = p.sqrt_phi * p.phi;
        p.xdep0 = (2.0 * EPSSI / (CHARGE_Q * p.npeak * 1.0e6)).sqrt() * p.sqrt_phi;
        p.sqrt_xdep0 = p.xdep0.sqrt();
        p.litl = (3.0 * p.xj * m.tox).sqrt();
        p.vbi = mt.vtm0 * (1.0e20 * p.npeak / (mt.ni * mt.ni)).ln();
        p.cdep0 = (CHARGE_Q * EPSSI * p.npeak * 1.0e6 / 2.0 / p.phi).sqrt();
        p.ldeb = (EPSSI * mt.vtm0 / (CHARGE_Q * p.npeak * 1.0e6)).sqrt() / 3.0;
        p.acde *= (p.npeak / 2.0e16).powf(-0.25);

        // --- k1/k2 (lines 652-706) ---
        if m.k1_given || m.k2_given {
            if !m.k1_given {
                log::warn!("BSIM3: k1 should be specified with k2; using 0.53");
                p.k1 = 0.53;
            }
            if !m.k2_given {
                log::warn!("BSIM3: k2 should be specified with k1; using -0.0186");
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

        // --- vbsc (lines 708-720) ---
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

        // --- vfb / vth0 (lines 722-736) ---
        if !m.vfb_given {
            if m.vth0_given {
                p.vfb = m.mtype * p.vth0 - p.phi - p.k1 * p.sqrt_phi;
            } else {
                p.vfb = -1.0;
            }
        }
        if !m.vth0_given {
            p.vth0 = m.mtype * (p.vfb + p.phi + p.k1 * p.sqrt_phi);
        }

        p.k1ox = p.k1 * m.tox / m.toxm;
        p.k2ox = p.k2 * m.tox / m.toxm;

        // --- theta0vb0 / thetaRout (lines 743-751) ---
        let t1 = (EPSSI / EPSOX * m.tox * p.xdep0).sqrt();
        let t0 = (-0.5 * p.dsub * p.leff / t1).exp();
        p.theta0vb0 = t0 + 2.0 * t0 * t0;
        let t0 = (-0.5 * p.drout * p.leff / t1).exp();
        let t2 = t0 + 2.0 * t0 * t0;
        p.theta_rout = p.pdibl1 * t2 + p.pdibl2;

        // --- vfbzb: Vth at Vbs = 0, Vds = 0 (lines 753-793) ---
        let tmp = p.xdep0.sqrt();
        let tmp1 = p.vbi - p.phi;
        let tmp2 = mt.factor1 * tmp;

        let t0 = -0.5 * p.dvt1w * p.weff * p.leff / tmp2;
        let t2 = if t0 > -EXP_THRESHOLD {
            let t1 = t0.exp();
            t1 * (1.0 + 2.0 * t1)
        } else {
            let t1 = MIN_EXP;
            t1 * (1.0 + 2.0 * t1)
        };
        let t0 = p.dvt0w * t2;
        let t2 = t0 * tmp1;

        let t0 = -0.5 * p.dvt1 * p.leff / tmp2;
        let t3 = if t0 > -EXP_THRESHOLD {
            let t1 = t0.exp();
            t1 * (1.0 + 2.0 * t1)
        } else {
            let t1 = MIN_EXP;
            t1 * (1.0 + 2.0 * t1)
        };
        let t3 = p.dvt0 * t3 * tmp1;

        let t4 = m.tox * p.phi / (p.weff + p.w0);

        let t0 = (1.0 + p.nlx / p.leff).sqrt();
        let t5 = p.k1ox * (t0 - 1.0) * p.sqrt_phi + (p.kt1 + p.kt1l / p.leff) * (tratio - 1.0);

        let tmp3 = m.mtype * p.vth0 - t2 - t3 + p.k3 * t4 + t5;
        p.vfbzb = tmp3 - p.phi - p.k1 * p.sqrt_phi;

        Ok(p)
    }

    /// Port of `BSIM3checkModel` (b3check.c). Fatal excursions become `Err`;
    /// the silent value fixups (a2/a1, rdsw/rds0, cgdo/cgso/cgbo clamps) run
    /// only under `paramchk=1` exactly as the C does. The model-level
    /// `cgdo`/`cgso`/`cgbo` < 0 fixups are not replicated because the model
    /// card is immutable here; a card that trips them under `paramchk=1` is
    /// rejected instead of silently patched.
    fn check_model(&mut self, m: &Bsim3v3Model) -> Result<(), String> {
        let p = self;
        let fatal = |msg: String| -> Result<(), String> { Err(format!("BSIM3: {msg}")) };

        if p.nlx < -p.leff {
            return fatal(format!("Nlx = {:e} is less than -Leff", p.nlx));
        }
        if m.tox <= 0.0 {
            return fatal(format!("Tox = {:e} is not positive", m.tox));
        }
        if m.toxm <= 0.0 {
            return fatal(format!("Toxm = {:e} is not positive", m.toxm));
        }
        if m.lintnoi > p.leff / 2.0 {
            return fatal(format!(
                "Lintnoi = {:e} is too large - Leff for noise is negative",
                m.lintnoi
            ));
        }
        if p.npeak <= 0.0 {
            return fatal(format!("Nch = {:e} is not positive", p.npeak));
        }
        if p.nsub <= 0.0 {
            return fatal(format!("Nsub = {:e} is not positive", p.nsub));
        }
        if p.ngate < 0.0 {
            return fatal(format!("Ngate = {:e} is not positive", p.ngate));
        }
        if p.ngate > 1.0e25 {
            return fatal(format!("Ngate = {:e} is too high", p.ngate));
        }
        if p.xj <= 0.0 {
            return fatal(format!("Xj = {:e} is not positive", p.xj));
        }
        if p.dvt1 < 0.0 {
            return fatal(format!("Dvt1 = {:e} is negative", p.dvt1));
        }
        if p.dvt1w < 0.0 {
            return fatal(format!("Dvt1w = {:e} is negative", p.dvt1w));
        }
        if p.w0 == -p.weff {
            return fatal("(W0 + Weff) = 0 causing divided-by-zero".to_string());
        }
        if p.dsub < 0.0 {
            return fatal(format!("Dsub = {:e} is negative", p.dsub));
        }
        if p.b1 == -p.weff {
            return fatal("(B1 + Weff) = 0 causing divided-by-zero".to_string());
        }
        if p.u0temp <= 0.0 {
            return fatal(format!(
                "u0 at current temperature = {:e} is not positive",
                p.u0temp
            ));
        }
        if p.delta < 0.0 {
            return fatal(format!("Delta = {:e} is less than zero", p.delta));
        }
        if p.vsattemp <= 0.0 {
            return fatal(format!(
                "Vsat at current temperature = {:e} is not positive",
                p.vsattemp
            ));
        }
        if p.pclm <= 0.0 {
            return fatal(format!("Pclm = {:e} is not positive", p.pclm));
        }
        if p.drout < 0.0 {
            return fatal(format!("Drout = {:e} is negative", p.drout));
        }
        if m.ijth < 0.0 {
            return fatal(format!("Ijth = {:e} cannot be negative", m.ijth));
        }
        if p.clc < 0.0 {
            return fatal(format!("Clc = {:e} is negative", p.clc));
        }

        if m.param_chk == 1 {
            // Result-affecting fixups gated on paramchk=1 (b3check.c:324-371).
            if p.a2 < 0.01 {
                log::warn!("BSIM3: A2 = {} is too small; set to 0.01", p.a2);
                p.a2 = 0.01;
            } else if p.a2 > 1.0 {
                log::warn!(
                    "BSIM3: A2 = {} is larger than 1; A2 set to 1, A1 set to 0",
                    p.a2
                );
                p.a2 = 1.0;
                p.a1 = 0.0;
            }
            if p.rdsw < 0.0 {
                log::warn!("BSIM3: Rdsw = {} is negative; set to zero", p.rdsw);
                p.rdsw = 0.0;
                p.rds0 = 0.0;
            } else if p.rds0 > 0.0 && p.rds0 < 0.001 {
                log::warn!(
                    "BSIM3: Rds at current temperature = {} is less than 0.001 ohm; set to zero",
                    p.rds0
                );
                p.rds0 = 0.0;
            }
            if m.cgdo < 0.0 || m.cgso < 0.0 || m.cgbo < 0.0 {
                return Err(
                    "BSIM3: negative cgdo/cgso/cgbo with paramchk=1 (ngspice silently zeroes \
                     the model card; specify non-negative overlap capacitances instead)"
                        .to_string(),
                );
            }
        }
        Ok(())
    }
}

/// Instance geometry/options needed by the per-instance temperature tail and
/// the load (the `BSIM3instance` inputs of b3set.c/b3temp.c/b3ld.c).
#[derive(Debug, Clone, Copy)]
pub struct Bsim3v3Geometry {
    pub l: Value,
    pub w: Value,
    /// Parallel multiplier `M` (b3set.c:986; BSIM3v3.3 has no NF parameter —
    /// multi-finger devices are expressed through `m`).
    pub m: Value,
    pub drain_area: Value,
    pub source_area: Value,
    pub drain_squares: Value,
    pub source_squares: Value,
    pub drain_perimeter: Value,
    pub source_perimeter: Value,
    /// Zero-bias threshold shift `DELVTO`.
    pub delvto: Value,
    /// Low-field mobility multiplier `MULU0`.
    pub mulu0: Value,
    /// `OFF`: start the device off in the initial DC iteration (engine seam;
    /// not used by the eval itself).
    pub off: bool,
    /// `IC=vds,vgs,vbs` initial conditions (engine seam for UIC).
    pub ic_vds: Value,
    pub ic_vgs: Value,
    pub ic_vbs: Value,
}

impl Default for Bsim3v3Geometry {
    /// Instance defaults per b3set.c lines 931-986 (acmMod = 0).
    fn default() -> Self {
        Self {
            l: 5.0e-6,
            w: 5.0e-6,
            m: 1.0,
            drain_area: 0.0,
            source_area: 0.0,
            drain_squares: 1.0,
            source_squares: 1.0,
            drain_perimeter: 0.0,
            source_perimeter: 0.0,
            delvto: 0.0,
            mulu0: 1.0,
            off: false,
            ic_vds: 0.0,
            ic_vgs: 0.0,
            ic_vbs: 0.0,
        }
    }
}

/// Per-instance temperature tail of `BSIM3temp` (b3temp.c lines 796-889).
#[derive(Debug, Clone, Default)]
pub struct Bsim3v3InstTemp {
    /// `pParam->vth0 + delvto` (`here->BSIM3vth0`).
    pub vth0: Value,
    /// `pParam->vfb + type*delvto` (`here->BSIM3vfb`).
    pub vfb: Value,
    /// `pParam->vfbzb + type*delvto` (`here->BSIM3vfbzb`).
    pub vfbzb: Value,
    /// `pParam->u0temp * mulu0` (`here->BSIM3u0temp`).
    pub u0temp: Value,
    /// Instance-adjusted Elmore constant (`here->BSIM3tconst`).
    pub tconst: Value,
    /// `1 / (rsh * nrd)` or 0 (`here->BSIM3drainConductance`).
    pub drain_conductance: Value,
    /// `1 / (rsh * nrs)` or 0 (`here->BSIM3sourceConductance`).
    pub source_conductance: Value,
    /// Source junction saturation current (recomputed identically in b3ld.c).
    pub source_sat_current: Value,
    /// Drain junction saturation current.
    pub drain_sat_current: Value,
    /// `ijth` linearization anchor: `(vjsm, IsEvjsm)` when the source diode
    /// has both a positive saturation current and `ijth > 0`.
    pub vjsm: Option<(Value, Value)>,
    /// `(vjdm, IsEvjdm)` for the drain diode.
    pub vjdm: Option<(Value, Value)>,
    // Instance geometry copies consumed by the load (`here->BSIM3drainArea`
    // and friends; the junction depletion split needs them each evaluation).
    pub drain_area: Value,
    pub source_area: Value,
    pub drain_perimeter: Value,
    pub source_perimeter: Value,
}

impl Bsim3v3InstTemp {
    pub fn new(
        model: &Bsim3v3Model,
        mt: &Bsim3v3ModelTemp,
        size: &Bsim3v3SizeDep,
        geom: &Bsim3v3Geometry,
    ) -> Self {
        let m = model;
        let vth0 = size.vth0 + geom.delvto;
        let vfb = size.vfb + m.mtype * geom.delvto;
        let vfbzb = size.vfbzb + m.mtype * geom.delvto;
        let u0temp = size.u0temp * geom.mulu0;
        // b3temp.c:803-804 reuses the final size-knot `T0`, which is
        // sqrt(1 + nlx / leff) from the vfbzb calculation, not leffCV^2.
        let nqs_tail_t0 = (1.0 + size.nlx / size.leff).sqrt();
        let tconst = u0temp * size.elm / (m.cox * size.weff_cv * size.leff_cv * nqs_tail_t0);

        // Series resistance (b3temp.c:811-851 plus ACM=1 devsup.c helper).
        let (drain_resistance, source_resistance) = source_drain_resistances_acm0_or_1(m, geom);
        let drain_conductance = if drain_resistance > 0.0 {
            1.0 / drain_resistance
        } else {
            0.0
        };
        let source_conductance = if source_resistance > 0.0 {
            1.0 / source_resistance
        } else {
            0.0
        };

        // Junction saturation currents + ijth anchors (b3temp.c:856-889).
        let nvtm = mt.vtm * m.jct_emission_coeff;
        let (drain_sat_current, source_sat_current) = saturation_currents_acm0_or_1(m, mt, geom);
        let vjsm = if source_sat_current > 0.0 && m.ijth > 0.0 {
            let v = nvtm * (m.ijth / source_sat_current + 1.0).ln();
            Some((v, source_sat_current * (v / nvtm).exp()))
        } else {
            None
        };

        let vjdm = if drain_sat_current > 0.0 && m.ijth > 0.0 {
            let v = nvtm * (m.ijth / drain_sat_current + 1.0).ln();
            Some((v, drain_sat_current * (v / nvtm).exp()))
        } else {
            None
        };

        Self {
            vth0,
            vfb,
            vfbzb,
            u0temp,
            tconst,
            drain_conductance,
            source_conductance,
            source_sat_current,
            drain_sat_current,
            vjsm,
            vjdm,
            drain_area: geom.drain_area,
            source_area: geom.source_area,
            drain_perimeter: geom.drain_perimeter,
            source_perimeter: geom.source_perimeter,
        }
    }
}

fn acm_effective_width(model: &Bsim3v3Model, geom: &Bsim3v3Geometry) -> Value {
    geom.w * model.wmlt + model.xw
}

fn source_drain_resistances_acm0_or_1(
    model: &Bsim3v3Model,
    geom: &Bsim3v3Geometry,
) -> (Value, Value) {
    if model.acm_mod == 1 {
        let w_eff = acm_effective_width(model, geom);
        let l_diff = model.ld + model.ldif;
        (
            l_diff / w_eff * model.rd + model.sheet_resistance * geom.drain_squares + model.rdc,
            l_diff / w_eff * model.rs + model.sheet_resistance * geom.source_squares + model.rsc,
        )
    } else {
        (
            model.sheet_resistance * geom.drain_squares,
            model.sheet_resistance * geom.source_squares,
        )
    }
}

fn saturation_currents_acm0_or_1(
    model: &Bsim3v3Model,
    mt: &Bsim3v3ModelTemp,
    geom: &Bsim3v3Geometry,
) -> (Value, Value) {
    if model.acm_mod == 1 {
        let w_eff = acm_effective_width(model, geom);
        let area = w_eff * model.wmlt;
        let perimeter = w_eff;
        let mut sat =
            area * mt.jct_temp_sat_cur_density + perimeter * mt.jct_sidewall_temp_sat_cur_density;
        if sat <= 0.0 {
            sat = 1.0e-14;
        }
        (sat, sat)
    } else {
        let source = if geom.source_area <= 0.0 && geom.source_perimeter <= 0.0 {
            1.0e-14
        } else {
            geom.source_area * mt.jct_temp_sat_cur_density
                + geom.source_perimeter * mt.jct_sidewall_temp_sat_cur_density
        };
        let drain = if geom.drain_area <= 0.0 && geom.drain_perimeter <= 0.0 {
            1.0e-14
        } else {
            geom.drain_area * mt.jct_temp_sat_cur_density
                + geom.drain_perimeter * mt.jct_sidewall_temp_sat_cur_density
        };
        (drain, source)
    }
}

/// Size-knot cache mirroring `model->pSizeDependParamKnot`: ngspice walks the
/// knot list looking for an exact drawn (W, L) match and computes a new knot
/// only on a miss. The C rebuilds the list inside every `BSIM3temp` call, so
/// the key also carries the temperature here (one cache instance may outlive
/// a temperature change).
#[derive(Debug, Default)]
pub struct SizeDepCache {
    knots: HashMap<(u64, u64, u64), Arc<Bsim3v3SizeDep>>,
}

impl SizeDepCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Look up (or compute and memoize) the size knot for a drawn (W, L) at
    /// the given device temperature.
    pub fn get(
        &mut self,
        model: &Bsim3v3Model,
        mt: &Bsim3v3ModelTemp,
        l: Value,
        w: Value,
    ) -> Result<Arc<Bsim3v3SizeDep>, String> {
        let key = (l.to_bits(), w.to_bits(), mt.temp.to_bits());
        if let Some(knot) = self.knots.get(&key) {
            return Ok(Arc::clone(knot));
        }
        let knot = Arc::new(Bsim3v3SizeDep::new(model, mt, l, w)?);
        self.knots.insert(key, Arc::clone(&knot));
        Ok(knot)
    }
}
