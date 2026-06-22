//! Size- and temperature-dependent parameter evaluation for BSIM4 v4.8.
//!
//! Faithful port of ngspice-46 `b4temp.c` (`BSIM4temp`) plus the always-on
//! checks of `b4check.c` (`BSIM4checkModel`). The split mirrors the C:
//!
//! - [`Bsim4v8ModelTemp`]: the per-model temperature block — `vtm`/`vtm0`,
//!   `vcrit`, `factor1`, S/D junction saturation-current and capacitance
//!   temperature scaling, `PhiB*` clamps, the trap-assisted `njts*temp`
//!   scaling, and the `ijth*`/`bvs`/`bvd` non-negativity resets.
//! - [`Bsim4v8SizeDep`]: the `bsim4SizeDependParam` knot, keyed by the drawn
//!   (W, L, NF) exactly like the C knot list; see [`SizeDepCache`].
//! - [`Bsim4v8InstTemp`]: the instance tail — `delvto`/`mulu0`,
//!   `vtfbphi1/2`, `vbsc`, `k2ox`, `vfbzb`, effective junction
//!   areas/perimeters (`BSIM4PAeffGeo`, geoMod 0 through 10), series
//!   conductances, junction-diode `dioMod` limiting anchors, and the reverse-bias TAT
//!   saturation currents.
//!
//! The C mutates the model card in a few checks (`njs`/`njd` < 0.1,
//! `mjs* >= 0.99`, `ckappas/d < 0.02`); those clamps are applied here on the
//! computed copies *before* first use, which differs from the C only for
//! cards that trip them (the C applies them after the first instance's temp
//! pass, leaving one stale evaluation).

use super::common::{
    CHARGE_Q, CONST_CHARGE, CONST_ROOT2, CONST_VT0, EPS0, EPSSI, EXP_THRESHOLD, KB_OVER_Q, MAX_EXP,
    MIN_EXP, PI, dexp_temp,
};
use super::params::Bsim4v8Model;
use crate::Value;
use std::collections::HashMap;
use std::sync::Arc;

const STRESS_DELTA: Value = 1.0e-9;

/// Per-model temperature data (b4temp.c lines 94-415 + the GEDL block).
#[derive(Debug, Clone, Default)]
pub struct Bsim4v8ModelTemp {
    /// Device temperature in Kelvin (ngspice `CKTtemp` + instance `dtemp`).
    pub temp: Value,
    /// TNOM in Kelvin.
    pub tnom: Value,
    /// `temp / tnom - 1` (the `TempRatio` of b4ld.c:1133).
    pub temp_ratio_m1: Value,
    /// `KboQ * temp` (`BSIM4vtm`).
    pub vtm: Value,
    /// `KboQ * tnom` (`BSIM4vtm0`).
    pub vtm0: Value,
    /// Energy gap at TNOM (`BSIM4Eg0`).
    pub eg0: Value,
    /// Intrinsic carrier density at TNOM (cm^-3).
    pub ni: Value,
    /// `CONSTvt0 * ln(CONSTvt0 / (sqrt(2) * 1e-14))` (`BSIM4vcrit`).
    pub vcrit: Value,
    /// `sqrt(epssub / (epsrox * EPS0) * toxe)` (`BSIM4factor1`).
    pub factor1: Value,
    /// `epsrox * EPS0 / toxe` (`BSIM4coxe`).
    pub coxe: Value,
    /// `epsrox * EPS0 / toxp` (`BSIM4coxp`, mtrlMod = 0).
    pub coxp: Value,
    /// Substrate permittivity (mtrlMod = 0: `EPSSI`).
    pub epssub: Value,

    // Junction saturation-current densities at the device temperature,
    // clamped >= 0 (S and D sides).
    pub s_jct_temp_sat_cur_density: Value,
    pub s_jct_sidewall_temp_sat_cur_density: Value,
    pub s_jct_gate_sidewall_temp_sat_cur_density: Value,
    pub d_jct_temp_sat_cur_density: Value,
    pub d_jct_sidewall_temp_sat_cur_density: Value,
    pub d_jct_gate_sidewall_temp_sat_cur_density: Value,

    // Junction depletion capacitances at the device temperature.
    pub s_unit_area_temp_jct_cap: Value,
    pub d_unit_area_temp_jct_cap: Value,
    pub s_unit_length_sidewall_temp_jct_cap: Value,
    pub d_unit_length_sidewall_temp_jct_cap: Value,
    pub s_unit_length_gate_sidewall_temp_jct_cap: Value,
    pub d_unit_length_gate_sidewall_temp_jct_cap: Value,

    // Built-in potentials at the device temperature, floored at 0.01.
    pub phi_bs: Value,
    pub phi_bd: Value,
    pub phi_bsws: Value,
    pub phi_bswd: Value,
    pub phi_bswgs: Value,
    pub phi_bswgd: Value,

    // ijth*/bv* after the <= 0 resets of b4temp.c:372-415.
    pub ijthsfwd: Value,
    pub ijthdfwd: Value,
    pub ijthsrev: Value,
    pub ijthdrev: Value,
    pub bvs: Value,
    pub bvd: Value,

    // Junction emission coefficients and grading coefficients after the
    // b4check.c clamps (njs/njd < 0.1, mj* >= 0.99).
    pub njs: Value,
    pub njd: Value,
    pub mjs: Value,
    pub mjsws: Value,
    pub mjswgs: Value,
    pub mjd: Value,
    pub mjswd: Value,
    pub mjswgd: Value,

    // GEDL trap-assisted tunneling temperature factors.
    pub njtsstemp: Value,
    pub njtsswstemp: Value,
    pub njtsswgstemp: Value,
    pub njtsdtemp: Value,
    pub njtsswdtemp: Value,
    pub njtsswgdtemp: Value,
    /// `exp(xtss * Eg0/vtm * (TRatio-1))` and friends (the T1..T6 of the
    /// GEDL block), consumed by the instance tail.
    pub xexp_tss: Value,
    pub xexp_tsd: Value,
    pub xexp_tssws: Value,
    pub xexp_tsswd: Value,
    pub xexp_tsswgs: Value,
    pub xexp_tsswgd: Value,
}

impl Bsim4v8ModelTemp {
    /// Port of the model-level head of `BSIM4temp`.
    ///
    /// `temp` is the device operating temperature in Kelvin. The
    /// `pbs`/`pbsws`/... < 0.1 clamps are applied to local copies; the model
    /// card itself stays immutable.
    pub fn new(model: &Bsim4v8Model, temp: Value) -> Self {
        let m = model;
        let clamp01 = |v: Value, name: &str| {
            if v < 0.1 {
                log::warn!("BSIM4: given {name} is less than 0.1; set to 0.1");
                0.1
            } else {
                v
            }
        };
        let pbs = clamp01(m.pbs, "pbs");
        let pbsws = clamp01(m.pbsws, "pbsws");
        let pbswgs = clamp01(m.pbswgs, "pbswgs");
        let pbd = clamp01(m.pbd, "pbd");
        let pbswd = clamp01(m.pbswd, "pbswd");
        let pbswgd = clamp01(m.pbswgd, "pbswgd");

        // b4check.c always-on diode-card clamps (see module docs).
        let njs_clamp = |v: Value, name: &str| {
            if v < 0.1 {
                log::warn!("BSIM4: {name} = {v} is less than 0.1; set to 0.1");
                0.1
            } else {
                if v < 0.7 {
                    log::warn!("BSIM4: {name} = {v} is less than 0.7");
                }
                v
            }
        };
        let njs = njs_clamp(m.njs, "njs");
        let njd = njs_clamp(m.njd, "njd");
        let mj_clamp = |v: Value, name: &str| {
            if v >= 0.99 {
                log::warn!("BSIM4: {name} = {v} is too big; set to 0.99");
                0.99
            } else {
                v
            }
        };
        let mjs = mj_clamp(m.mjs, "mjs");
        let mjsws = mj_clamp(m.mjsws, "mjsws");
        let mjswgs = mj_clamp(m.mjswgs, "mjswgs");
        let mjd = mj_clamp(m.mjd, "mjd");
        let mjswd = mj_clamp(m.mjswd, "mjswd");
        let mjswgd = mj_clamp(m.mjswgd, "mjswgd");

        // b4temp.c "dunga" material constants.
        let epsrox = m.effective_epsrox();
        let toxe = m.effective_toxe();
        let epssub = if m.mtrl_mod != 0 {
            EPS0 * m.epsrsub
        } else {
            EPSSI
        };
        let coxe = epsrox * EPS0 / toxe;
        let coxp = if m.mtrl_mod == 0 || m.mtrl_compat_mod != 0 {
            m.epsrox * EPS0 / m.toxp
        } else {
            coxe
        };

        let tnom = m.tnom;
        let tratio = temp / tnom;

        let vcrit = CONST_VT0 * (CONST_VT0 / (CONST_ROOT2 * 1.0e-14)).ln();
        let factor1 = (epssub / (epsrox * EPS0) * toxe).sqrt();

        let vtm0 = KB_OVER_Q * tnom;
        let (eg0, ni) = if m.mtrl_mod == 0 {
            let eg0 = 1.16 - 7.02e-4 * tnom * tnom / (tnom + 1108.0);
            let ni = 1.45e10
                * (tnom / 300.15)
                * (tnom / 300.15).sqrt()
                * (21.5565981 - eg0 / (2.0 * vtm0)).exp();
            (eg0, ni)
        } else {
            let eg0 = m.bg0sub - m.tbgasub * tnom * tnom / (tnom + m.tbgbsub);
            let t0 = m.bg0sub - m.tbgasub * 90090.0225 / (300.15 + m.tbgbsub);
            let ni = m.ni0sub
                * (tnom / 300.15)
                * (tnom / 300.15).sqrt()
                * ((t0 - eg0) / (2.0 * vtm0)).exp();
            (eg0, ni)
        };

        let vtm = KB_OVER_Q * temp;
        let eg = if m.mtrl_mod == 0 {
            1.16 - 7.02e-4 * temp * temp / (temp + 1108.0)
        } else {
            m.bg0sub - m.tbgasub * temp * temp / (temp + m.tbgbsub)
        };

        let (mut s_js, mut s_jsws, mut s_jswgs, mut d_js, mut d_jsws, mut d_jswgs);
        if temp != tnom {
            let t0 = eg0 / vtm0 - eg / vtm;
            let t1 = (temp / tnom).ln();
            let t2 = t0 + m.xtis * t1;
            let t3 = (t2 / njs).exp();
            s_js = m.jss * t3;
            s_jsws = m.jsws * t3;
            s_jswgs = m.jswgs * t3;
            let t2 = t0 + m.xtid * t1;
            let t3 = (t2 / njd).exp();
            d_js = m.jsd * t3;
            d_jsws = m.jswd * t3;
            d_jswgs = m.jswgd * t3;
        } else {
            s_js = m.jss;
            s_jsws = m.jsws;
            s_jswgs = m.jswgs;
            d_js = m.jsd;
            d_jsws = m.jswd;
            d_jswgs = m.jswgd;
        }
        for v in [
            &mut s_js,
            &mut s_jsws,
            &mut s_jswgs,
            &mut d_js,
            &mut d_jsws,
            &mut d_jswgs,
        ] {
            if *v < 0.0 {
                *v = 0.0;
            }
        }

        // Temperature dependence of the D/B and S/B diode capacitances.
        let del_temp = temp - tnom;
        let cap = |c: Value, tc: Value, name: &str| {
            let t0 = tc * del_temp;
            if t0 >= -1.0 {
                c * (1.0 + t0)
            } else if c > 0.0 {
                log::warn!(
                    "BSIM4: temperature effect has caused {name} to be negative; clamped to zero"
                );
                0.0
            } else {
                c * (1.0 + t0)
            }
        };
        // CJSWS/CJSWD < 0 are clamped to 0 before scaling (b4temp.c:296-301).
        let cjsws = if m.cjsws < 0.0 {
            log::warn!("BSIM4: CJSWS is negative; clamped to zero");
            0.0
        } else {
            m.cjsws
        };
        let cjswd = if m.cjswd < 0.0 {
            log::warn!("BSIM4: CJSWD is negative; clamped to zero");
            0.0
        } else {
            m.cjswd
        };
        let s_unit_area_temp_jct_cap = cap(m.cjs, m.tcj, "cjs");
        let d_unit_area_temp_jct_cap = cap(m.cjd, m.tcj, "cjd");
        let s_unit_length_sidewall_temp_jct_cap = cap(cjsws, m.tcjsw, "cjsws");
        let d_unit_length_sidewall_temp_jct_cap = cap(cjswd, m.tcjsw, "cjswd");
        let s_unit_length_gate_sidewall_temp_jct_cap = cap(m.cjswgs, m.tcjswg, "cjswgs");
        let d_unit_length_gate_sidewall_temp_jct_cap = cap(m.cjswgd, m.tcjswg, "cjswgd");

        let phi = |pb: Value, tpb: Value, name: &str| {
            let v = pb - tpb * del_temp;
            if v < 0.01 {
                log::warn!(
                    "BSIM4: temperature effect has caused {name} to be less than 0.01; \
                     clamped to 0.01"
                );
                0.01
            } else {
                v
            }
        };
        let phi_bs = phi(pbs, m.tpb, "pbs");
        let phi_bd = phi(pbd, m.tpb, "pbd");
        let phi_bsws = phi(pbsws, m.tpbsw, "pbsws");
        let phi_bswd = phi(pbswd, m.tpbsw, "pbswd");
        let phi_bswgs = phi(pbswgs, m.tpbswg, "pbswgs");
        let phi_bswgd = phi(pbswgd, m.tpbswg, "pbswgd");

        let reset0 = |v: Value, name: &str| {
            if v <= 0.0 {
                log::warn!("BSIM4: {name} reset to 0");
                0.0
            } else {
                v
            }
        };
        let ijthsfwd = reset0(m.ijthsfwd, "ijthsfwd");
        let ijthdfwd = reset0(m.ijthdfwd, "ijthdfwd");
        let ijthsrev = reset0(m.ijthsrev, "ijthsrev");
        let ijthdrev = reset0(m.ijthdrev, "ijthdrev");
        let bvs = reset0(m.bvs, "bvs");
        let bvd = reset0(m.bvd, "bvd");

        // GEDL trap-assisted tunneling temperature scaling (b4temp.c
        // lines 2227-2247; model-level even though the C computes it inside
        // the instance loop).
        let t0 = tratio - 1.0;
        let njtsstemp = m.njts * (1.0 + m.tnjts * t0);
        let njtsswstemp = m.njtssw * (1.0 + m.tnjtssw * t0);
        let njtsswgstemp = m.njtsswg * (1.0 + m.tnjtsswg * t0);
        let njtsdtemp = m.njtsd * (1.0 + m.tnjtsd * t0);
        let njtsswdtemp = m.njtsswd * (1.0 + m.tnjtsswd * t0);
        let njtsswgdtemp = m.njtsswgd * (1.0 + m.tnjtsswgd * t0);
        let t7 = eg0 / vtm * t0;
        let xexp_tss = dexp_temp(m.xtss * t7);
        let xexp_tsd = dexp_temp(m.xtsd * t7);
        let xexp_tssws = dexp_temp(m.xtssws * t7);
        let xexp_tsswd = dexp_temp(m.xtsswd * t7);
        let xexp_tsswgs = dexp_temp(m.xtsswgs * t7);
        let xexp_tsswgd = dexp_temp(m.xtsswgd * t7);

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
            coxe,
            coxp,
            epssub,
            s_jct_temp_sat_cur_density: s_js,
            s_jct_sidewall_temp_sat_cur_density: s_jsws,
            s_jct_gate_sidewall_temp_sat_cur_density: s_jswgs,
            d_jct_temp_sat_cur_density: d_js,
            d_jct_sidewall_temp_sat_cur_density: d_jsws,
            d_jct_gate_sidewall_temp_sat_cur_density: d_jswgs,
            s_unit_area_temp_jct_cap,
            d_unit_area_temp_jct_cap,
            s_unit_length_sidewall_temp_jct_cap,
            d_unit_length_sidewall_temp_jct_cap,
            s_unit_length_gate_sidewall_temp_jct_cap,
            d_unit_length_gate_sidewall_temp_jct_cap,
            phi_bs,
            phi_bd,
            phi_bsws,
            phi_bswd,
            phi_bswgs,
            phi_bswgd,
            ijthsfwd,
            ijthdfwd,
            ijthsrev,
            ijthdrev,
            bvs,
            bvd,
            njs,
            njd,
            mjs,
            mjsws,
            mjswgs,
            mjd,
            mjswd,
            mjswgd,
            njtsstemp,
            njtsswstemp,
            njtsswgstemp,
            njtsdtemp,
            njtsswdtemp,
            njtsswgdtemp,
            xexp_tss,
            xexp_tsd,
            xexp_tssws,
            xexp_tsswd,
            xexp_tsswgs,
            xexp_tsswgd,
        }
    }
}

/// `T1 / (T1 - 1)^2 + 2 T1 MIN_EXP` exponential form used by `theta0vb0`,
/// `thetaRout` and the `vfbzbfactor` terms (b4temp.c:1504-1557).
#[inline]
fn theta_form(t0: Value) -> Value {
    if t0 < EXP_THRESHOLD {
        let t1 = t0.exp();
        let t2 = t1 - 1.0;
        let t3 = t2 * t2;
        let t4 = t3 + 2.0 * t1 * MIN_EXP;
        t1 / t4
    } else {
        1.0 / (MAX_EXP - 2.0)
    }
}

/// `BSIM4polyDepletion` value path used by the compat0 EOT-to-TOXP
/// calculation in `b4temp.c`. The load-side evaluator carries the derivative;
/// the temp path only needs `Vgs_eff`.
#[inline]
fn poly_depletion_value(
    phi: Value,
    ngate: Value,
    epsgate: Value,
    coxe: Value,
    vgs: Value,
) -> Value {
    if ngate > 1.0e18 && ngate < 1.0e25 && vgs > phi && epsgate != 0.0 {
        let t1 = 1.0e6 * CONST_CHARGE * epsgate * ngate / (coxe * coxe);
        let t8 = vgs - phi;
        let t4 = (1.0 + 2.0 * t8 / t1).sqrt();
        let t2 = 2.0 * t8 / (t4 + 1.0);
        let t3 = 0.5 * t2 * t2 / t1;
        let t7 = 1.12 - t3 - 0.05;
        let t6 = (t7 * t7 + 0.224).sqrt();
        let t5 = 1.12 - 0.5 * (t7 + t6);
        vgs - t5
    } else {
        vgs
    }
}

/// Size-dependent parameter knot (`bsim4SizeDependParam`), one per drawn
/// (W, L, NF) triple per temperature pass.
#[derive(Debug, Clone, Default)]
pub struct Bsim4v8SizeDep {
    pub width: Value,
    pub length: Value,
    pub nfinger: Value,

    // Binned values, post temperature adjustment.
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
    pub ketac: Value,
    pub nsub: Value,
    pub ndep: Value,
    pub nsd: Value,
    pub phin: Value,
    pub ngate: Value,
    pub gamma1: Value,
    pub gamma2: Value,
    pub vbx: Value,
    pub vbm: Value,
    pub xt: Value,
    pub vfb: Value,
    pub k1: Value,
    pub kt1: Value,
    pub kt1l: Value,
    pub k2: Value,
    pub kt2: Value,
    pub k3: Value,
    pub k3b: Value,
    pub w0: Value,
    pub lpe0: Value,
    pub lpeb: Value,
    pub dvtp0: Value,
    pub dvtp1: Value,
    pub dvtp2: Value,
    pub dvtp3: Value,
    pub dvtp4: Value,
    pub dvtp5: Value,
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
    pub ud: Value,
    pub ud1: Value,
    pub up: Value,
    pub lp: Value,
    pub eu: Value,
    pub u0: Value,
    pub ute: Value,
    pub ucs: Value,
    pub ucste: Value,
    pub voff: Value,
    pub tvoff: Value,
    pub minv: Value,
    pub minvcv: Value,
    pub fprout: Value,
    pub pdits: Value,
    pub pditsd: Value,
    pub delta: Value,
    pub rdsw: Value,
    pub rdw: Value,
    pub rsw: Value,
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
    pub agidl: Value,
    pub bgidl: Value,
    pub cgidl: Value,
    pub egidl: Value,
    pub rgidl: Value,
    pub kgidl: Value,
    pub fgidl: Value,
    pub agisl: Value,
    pub bgisl: Value,
    pub cgisl: Value,
    pub egisl: Value,
    pub rgisl: Value,
    pub kgisl: Value,
    pub fgisl: Value,
    pub aigc: Value,
    pub bigc: Value,
    pub cigc: Value,
    pub aigs: Value,
    pub bigs: Value,
    pub cigs: Value,
    pub aigd: Value,
    pub bigd: Value,
    pub cigd: Value,
    pub aigbacc: Value,
    pub bigbacc: Value,
    pub cigbacc: Value,
    pub aigbinv: Value,
    pub bigbinv: Value,
    pub cigbinv: Value,
    pub nigc: Value,
    pub nigbacc: Value,
    pub nigbinv: Value,
    pub ntox: Value,
    pub eigbinv: Value,
    pub pigcd: Value,
    pub poxedge: Value,
    pub xrcrg1: Value,
    pub xrcrg2: Value,
    pub lambda: Value,
    pub vtl: Value,
    pub xn: Value,
    pub vfbsdoff: Value,
    pub tvfbsdoff: Value,

    // C-V model.
    pub cgsl: Value,
    pub cgdl: Value,
    pub ckappas: Value,
    pub ckappad: Value,
    pub cf: Value,
    pub clc: Value,
    pub cle: Value,
    pub vfbcv: Value,
    pub acde: Value,
    pub moin: Value,
    pub noff: Value,
    pub voffcv: Value,

    // Pre-calculated constants.
    pub dl: Value,
    pub dlc: Value,
    pub dw: Value,
    pub dwc_eff: Value,
    pub dwj: Value,
    pub leff: Value,
    pub weff: Value,
    pub leff_cv: Value,
    pub weff_cv: Value,
    pub weff_cj: Value,
    /// Drawn length plus `XL` (`Lnew`), used by gate-resistance geometry.
    pub lnew: Value,
    pub abulk_cv_factor: Value,
    pub cgso: Value,
    pub cgdo: Value,
    pub cgbo: Value,
    pub u0temp: Value,
    pub kvth0we: Value,
    pub k2we: Value,
    pub ku0we: Value,
    pub vsattemp: Value,
    pub rds0: Value,
    pub rdswmin: Value,
    pub rd0: Value,
    pub rdwmin: Value,
    pub rs0: Value,
    pub rswmin: Value,
    pub sqrt_phi: Value,
    pub phis3: Value,
    pub phi: Value,
    pub xdep0: Value,
    pub sqrt_xdep0: Value,
    pub theta0vb0: Value,
    pub theta_rout: Value,
    pub mstar: Value,
    pub mstarcv: Value,
    pub voffcbn: Value,
    pub voffcbncv: Value,
    pub ldeb: Value,
    pub vbi: Value,
    pub vfbsd: Value,
    pub cdep0: Value,
    pub litl: Value,
    pub k1ox: Value,
    pub vfbzbfactor: Value,
    pub stress_ku0: Value,
    pub stress_kvth0: Value,
    pub stress_ku0temp: Value,
    pub stress_inv_od_ref: Value,
    pub stress_rho_ref: Value,
    pub dvtp2factor: Value,
    /// `pParam->BSIM4VgsteffVth`, used by the high-k `mobMod=3` branch.
    pub vgsteff_vth: Value,
    pub tox_ratio: Value,
    pub tox_ratio_edge: Value,
    pub aechvb: Value,
    pub bechvb: Value,
    pub aechvb_edge_s: Value,
    pub aechvb_edge_d: Value,
    pub bechvb_edge: Value,
    pub lc_eff: Value,
    pub tfactor: Value,
}

impl Bsim4v8SizeDep {
    /// Port of the `Size_Not_Found` block of `BSIM4temp` for one drawn
    /// (W, L, NF), including the embedded `BSIM4checkModel` pass (fatal
    /// excursions return `Err`; value fixups follow the C's `paramChk`
    /// gating, which defaults to on).
    #[allow(clippy::too_many_lines)]
    pub fn new(
        model: &Bsim4v8Model,
        mt: &Bsim4v8ModelTemp,
        ldrn: Value,
        wdrn: Value,
        nf: Value,
    ) -> Result<Self, String> {
        let m = model;
        let mut p = Self {
            width: wdrn,
            length: ldrn,
            nfinger: nf,
            ..Self::default()
        };
        let temp = mt.temp;
        let tnom = mt.tnom;
        let tratio = temp / tnom;
        let del_temp = temp - tnom;
        let toxe = m.effective_toxe();
        let epsrox = m.effective_epsrox();
        let epssub = mt.epssub;
        let vtm0 = mt.vtm0;

        // --- Geometry scaling (b4temp.c:454-519) ---
        let lnew = ldrn + m.xl;
        let wnew = wdrn / nf + m.xw;
        p.lnew = lnew;

        let t0 = lnew.powf(m.lln);
        let t1 = wnew.powf(m.lwn);
        let tmp1 = m.ll / t0 + m.lw / t1 + m.lwl / (t0 * t1);
        p.dl = m.lint + tmp1;
        let tmp2 = m.llc / t0 + m.lwc / t1 + m.lwlc / (t0 * t1);
        p.dlc = m.dlc + tmp2;

        let t2 = lnew.powf(m.wln);
        let t3 = wnew.powf(m.wwn);
        let tmp1 = m.wl / t2 + m.ww / t3 + m.wwl / (t2 * t3);
        p.dw = m.wint + tmp1;
        let tmp2 = m.wlc / t2 + m.wwc / t3 + m.wwlc / (t2 * t3);
        p.dwc_eff = m.dwc + tmp2;
        p.dwj = m.dwj + tmp2;

        p.leff = lnew - 2.0 * p.dl;
        if p.leff <= 0.0 {
            return Err("BSIM4: effective channel length <= 0".to_string());
        }
        p.weff = wnew - 2.0 * p.dw;
        if p.weff <= 0.0 {
            return Err("BSIM4: effective channel width <= 0".to_string());
        }
        p.leff_cv = lnew - 2.0 * p.dlc;
        if p.leff_cv <= 0.0 {
            return Err("BSIM4: effective channel length for C-V <= 0".to_string());
        }
        p.weff_cv = wnew - 2.0 * p.dwc_eff;
        if p.weff_cv <= 0.0 {
            return Err("BSIM4: effective channel width for C-V <= 0".to_string());
        }
        p.weff_cj = wnew - 2.0 * p.dwj;
        if p.weff_cj <= 0.0 {
            return Err("BSIM4: effective channel width for S/D junctions <= 0".to_string());
        }

        // --- Binned parameters ---
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
            ($($field:ident),+ $(,)?) => {
                $(p.$field = m.$field.eval(inv_l, inv_w, inv_lw);)+
            };
        }
        bin!(
            cdsc, cdscb, cdscd, cit, nfactor, xj, vsat, at, a0, ags, a1, a2, keta, ketac, nsub,
            ndep, nsd, phin, ngate, gamma1, gamma2, vbx, vbm, xt, vfb, k1, kt1, kt1l, k2, kt2, k3,
            k3b, w0, lpe0, lpeb, dvtp0, dvtp1, dvtp2, dvtp3, dvtp4, dvtp5, dvt0, dvt1, dvt2, dvt0w,
            dvt1w, dvt2w, drout, dsub, vth0, ua, ua1, ub, ub1, uc, uc1, ud, ud1, up, lp, eu, u0,
            ute, ucs, ucste, voff, tvoff, minv, minvcv, fprout, pdits, pditsd, delta, rdsw, rdw,
            rsw, prwg, prwb, prt, eta0, etab, pclm, pdibl1, pdibl2, pdiblb, pscbe1, pscbe2, pvag,
            wr, dwg, dwb, b0, b1, alpha0, alpha1, beta0, agidl, bgidl, cgidl, egidl, rgidl, kgidl,
            fgidl, agisl, bgisl, cgisl, egisl, rgisl, kgisl, fgisl, aigc, bigc, cigc, aigs, bigs,
            cigs, aigd, bigd, cigd, aigbacc, bigbacc, cigbacc, aigbinv, bigbinv, cigbinv, nigc,
            nigbacc, nigbinv, ntox, eigbinv, pigcd, poxedge, xrcrg1, xrcrg2, lambda, vtl, xn,
            vfbsdoff, tvfbsdoff, cgsl, cgdl, ckappas, ckappad, cf, clc, cle, vfbcv, acde, moin,
            noff, voffcv, kvth0we, k2we, ku0we,
        );
        // v4.7 temperature dependence of leakage (applied to the binned
        // values below).
        let tnfactor = m.tnfactor.eval(inv_l, inv_w, inv_lw);
        let teta0 = m.teta0.eval(inv_l, inv_w, inv_lw);
        let tvoffcv = m.tvoffcv.eval(inv_l, inv_w, inv_lw);

        p.abulk_cv_factor = 1.0 + (p.clc / p.leff_cv).powf(p.cle);

        // --- Temperature adjustment (b4temp.c:1185-1298) ---
        let t0 = tratio - 1.0;
        let pow_weff_wr = (p.weff_cj * 1.0e6).powf(p.wr) * nf;
        let (mut rd0_temp, mut rdwmin_temp, mut rs0_temp, mut rswmin_temp) = (0.0, 0.0, 0.0, 0.0);

        p.ucs *= tratio.powf(p.ucste);
        if m.temp_mod == 0 {
            p.ua += p.ua1 * t0;
            p.ub += p.ub1 * t0;
            p.uc += p.uc1 * t0;
            p.ud += p.ud1 * t0;
            p.vsattemp = p.vsat - p.at * t0;
            let t10 = p.prt * t0;
            if m.rds_mod != 0 {
                rd0_temp = p.rdw + t10;
                rdwmin_temp = m.rdwmin + t10;
                rs0_temp = p.rsw + t10;
                rswmin_temp = m.rswmin + t10;
            }
            // Internal Rds(V) in IV (rdsMod = 0).
            p.rds0 = (p.rdsw + t10) * nf / pow_weff_wr;
            p.rdswmin = (m.rdswmin + t10) * nf / pow_weff_wr;
        } else {
            if m.temp_mod == 3 {
                p.ua *= tratio.powf(p.ua1);
                p.ub *= tratio.powf(p.ub1);
                p.uc *= tratio.powf(p.uc1);
                p.ud *= tratio.powf(p.ud1);
            } else {
                // tempMod = 1, 2.
                p.ua *= 1.0 + p.ua1 * del_temp;
                p.ub *= 1.0 + p.ub1 * del_temp;
                p.uc *= 1.0 + p.uc1 * del_temp;
                p.ud *= 1.0 + p.ud1 * del_temp;
            }
            p.vsattemp = p.vsat * (1.0 - p.at * del_temp);
            let t10 = 1.0 + p.prt * del_temp;
            if m.rds_mod != 0 {
                rd0_temp = p.rdw * t10;
                rdwmin_temp = m.rdwmin * t10;
                rs0_temp = p.rsw * t10;
                rswmin_temp = m.rswmin * t10;
            }
            p.rds0 = p.rdsw * t10 * nf / pow_weff_wr;
            p.rdswmin = m.rdswmin * t10 * nf / pow_weff_wr;
        }
        if rd0_temp < 0.0 {
            log::warn!("BSIM4: Rdw at current temperature is negative; set to zero");
            rd0_temp = 0.0;
        }
        if rdwmin_temp < 0.0 {
            log::warn!("BSIM4: Rdwmin at current temperature is negative; set to zero");
            rdwmin_temp = 0.0;
        }
        if rs0_temp < 0.0 {
            log::warn!("BSIM4: Rsw at current temperature is negative; set to zero");
            rs0_temp = 0.0;
        }
        if rswmin_temp < 0.0 {
            log::warn!("BSIM4: Rswmin at current temperature is negative; set to zero");
            rswmin_temp = 0.0;
        }
        p.rd0 = rd0_temp / pow_weff_wr;
        p.rdwmin = rdwmin_temp / pow_weff_wr;
        p.rs0 = rs0_temp / pow_weff_wr;
        p.rswmin = rswmin_temp / pow_weff_wr;

        if p.u0 > 1.0 {
            p.u0 /= 1.0e4;
        }
        // Mobility channel length dependence.
        let t5 = 1.0 - p.up * (-p.leff / p.lp).exp();
        p.u0temp = p.u0 * t5 * tratio.powf(p.ute);
        if p.eu < 0.0 {
            log::warn!("BSIM4: eu has been negative; reset to 0.0");
            p.eu = 0.0;
        }
        if p.ucs < 0.0 {
            log::warn!("BSIM4: ucs has been negative; reset to 0.0");
            p.ucs = 0.0;
        }

        p.vfbsdoff *= 1.0 + p.tvfbsdoff * del_temp;
        p.voff *= 1.0 + p.tvoff * del_temp;
        p.nfactor += tnfactor * del_temp / tnom;
        p.voffcv *= 1.0 + tvoffcv * del_temp;
        p.eta0 += teta0 * del_temp / tnom;

        // Source-end velocity limit (computed before the b4check.c xn/lc
        // fixups exactly like the C; the post-hoc fixups cannot reach
        // tfactor there either).
        if m.vtl_given && p.vtl > 0.0 {
            p.lc_eff = if m.lc < 0.0 { 0.0 } else { m.lc };
            let t0 = p.leff / (p.xn * p.leff + p.lc_eff);
            p.tfactor = (1.0 - t0) / (1.0 + t0);
        }

        p.cgdo = (m.cgdo + p.cf) * p.weff_cv;
        p.cgso = (m.cgso + p.cf) * p.weff_cv;
        p.cgbo = m.cgbo * p.leff_cv * nf;

        if !m.ndep_given && m.gamma1_given {
            let t0 = p.gamma1 * mt.coxe;
            p.ndep = 3.01248e22 * t0 * t0;
        }

        p.phi = vtm0 * (p.ndep / mt.ni).ln() + p.phin + 0.4;
        if p.phi <= 0.0 {
            return Err(format!(
                "BSIM4: Phi = {:e} is not positive; check Phin and Ndep (phin={:e} ndep={:e})",
                p.phi, p.phin, p.ndep
            ));
        }
        p.sqrt_phi = p.phi.sqrt();
        p.phis3 = p.sqrt_phi * p.phi;
        p.xdep0 = (2.0 * epssub / (CHARGE_Q * p.ndep * 1.0e6)).sqrt() * p.sqrt_phi;
        p.sqrt_xdep0 = p.xdep0.sqrt();

        p.litl = if m.mtrl_mod == 0 {
            (3.0 * 3.9 / epsrox * p.xj * toxe).sqrt()
        } else {
            (m.epsrsub / epsrox * p.xj * toxe).sqrt()
        };
        p.vbi = vtm0 * (p.nsd * p.ndep / (mt.ni * mt.ni)).ln();
        p.vfbsd = if m.mtrl_mod == 0 {
            if p.ngate > 0.0 {
                vtm0 * (p.ngate / p.nsd).ln()
            } else {
                0.0
            }
        } else {
            let mut t0 = vtm0 * (p.nsd / mt.ni).ln();
            let t1 = 0.5 * mt.eg0;
            if t0 > t1 {
                t0 = t1;
            }
            let t2 = m.easub + t1 - m.mtype * t0;
            m.phig - t2
        };

        p.cdep0 = (CHARGE_Q * epssub * p.ndep * 1.0e6 / 2.0 / p.phi).sqrt();

        p.tox_ratio = (p.ntox * (m.toxref / toxe).ln()).exp() / toxe / toxe;
        p.tox_ratio_edge = (p.ntox * (m.toxref / (toxe * p.poxedge)).ln()).exp()
            / toxe
            / toxe
            / p.poxedge
            / p.poxedge;
        p.aechvb = if m.mtype > 0.0 {
            4.97232e-7
        } else {
            3.42537e-7
        };
        p.bechvb = if m.mtype > 0.0 {
            7.45669e11
        } else {
            1.16645e12
        };
        // 4.8.1+ behavior: negative dlcig/dlcigd are clamped to zero.
        let dlcig = if m.dlcig < 0.0 {
            log::warn!("BSIM4: dlcig = {:e} is negative; set to zero", m.dlcig);
            0.0
        } else {
            m.dlcig
        };
        let dlcigd = if m.dlcigd < 0.0 {
            log::warn!("BSIM4: dlcigd = {:e} is negative; set to zero", m.dlcigd);
            0.0
        } else {
            m.dlcigd
        };
        p.aechvb_edge_s = p.aechvb * p.weff * dlcig * p.tox_ratio_edge;
        p.aechvb_edge_d = p.aechvb * p.weff * dlcigd * p.tox_ratio_edge;
        p.bechvb_edge = -p.bechvb * toxe * p.poxedge;
        p.aechvb *= p.weff * p.leff * p.tox_ratio;
        p.bechvb *= -toxe;

        p.mstar = 0.5 + p.minv.atan() / PI;
        p.mstarcv = 0.5 + p.minvcv.atan() / PI;
        p.voffcbn = p.voff + m.voffl / p.leff;
        p.voffcbncv = p.voffcv + m.voffcvl / p.leff;

        p.ldeb = (epssub * vtm0 / (CHARGE_Q * p.ndep * 1.0e6)).sqrt() / 3.0;
        p.acde *= (p.ndep / 2.0e16).powf(-0.25);

        // --- k1/k2 (b4temp.c:1420-1468) ---
        if m.k1_given || m.k2_given {
            if !m.k1_given {
                log::warn!("BSIM4: k1 should be specified with k2; using 0.53");
                p.k1 = 0.53;
            }
            if !m.k2_given {
                log::warn!("BSIM4: k2 should be specified with k1; using -0.0186");
                p.k2 = -0.0186;
            }
            if m.nsub_given {
                log::warn!("BSIM4: nsub is ignored because k1 or k2 is given");
            }
            if m.vbx_given {
                log::warn!("BSIM4: vbx is ignored because k1 or k2 is given");
            }
            if m.gamma1_given {
                log::warn!("BSIM4: gamma1 is ignored because k1 or k2 is given");
            }
            if m.gamma2_given {
                log::warn!("BSIM4: gamma2 is ignored because k1 or k2 is given");
            }
        } else {
            if !m.vbx_given {
                p.vbx = p.phi - 7.7348e-4 * p.ndep * p.xt * p.xt;
            }
            if p.vbx > 0.0 {
                p.vbx = -p.vbx;
            }
            if p.vbm > 0.0 {
                p.vbm = -p.vbm;
            }
            if !m.gamma1_given {
                p.gamma1 = 5.753e-12 * p.ndep.sqrt() / mt.coxe;
            }
            if !m.gamma2_given {
                p.gamma2 = 5.753e-12 * p.nsub.sqrt() / mt.coxe;
            }
            let t0 = p.gamma1 - p.gamma2;
            let t1 = (p.phi - p.vbx).sqrt() - p.sqrt_phi;
            let t2 = (p.phi * (p.phi - p.vbm)).sqrt() - p.phi;
            p.k2 = t0 * t1 / (2.0 * t2 + p.vbm);
            p.k1 = p.gamma2 - 2.0 * p.k2 * (p.phi - p.vbm).sqrt();
        }

        // --- vfb / vth0 (b4temp.c:1470-1499) ---
        if !m.vfb_given {
            if m.vth0_given {
                p.vfb = m.mtype * p.vth0 - p.phi - p.k1 * p.sqrt_phi;
            } else if m.mtrl_mod != 0 && m.phig_given && m.nsub_given {
                let mut t0 = vtm0 * (p.nsub / mt.ni).ln();
                let t1 = 0.5 * mt.eg0;
                if t0 > t1 {
                    t0 = t1;
                }
                let t2 = m.easub + t1 + m.mtype * t0;
                p.vfb = m.phig - t2;
            } else {
                p.vfb = -1.0;
            }
        }
        if !m.vth0_given {
            p.vth0 = m.mtype * (p.vfb + p.phi + p.k1 * p.sqrt_phi);
        }

        p.k1ox = p.k1 * toxe / m.toxm;

        // --- theta0vb0 / thetaRout (b4temp.c:1504-1527) ---
        let tmp = (epssub / (epsrox * EPS0) * toxe * p.xdep0).sqrt();
        p.theta0vb0 = theta_form(p.dsub * p.leff / tmp);
        p.theta_rout = p.pdibl1 * theta_form(p.drout * p.leff / tmp) + p.pdibl2;

        // --- vfbzbfactor (b4temp.c:1529-1572) ---
        let tmp = p.xdep0.sqrt();
        let tmp1 = p.vbi - p.phi;
        let tmp2 = mt.factor1 * tmp;

        let t8 = {
            let t0 = p.dvt1w * p.weff * p.leff / tmp2;
            let t8 = theta_form(t0);
            p.dvt0w * t8 * tmp1
        };
        let t9 = {
            let t0 = p.dvt1 * p.leff / tmp2;
            p.dvt0 * theta_form(t0) * tmp1
        };
        let t4 = toxe * p.phi / (p.weff + p.w0);

        let t0 = (1.0 + p.lpe0 / p.leff).sqrt();
        let t3 = if m.temp_mod == 1 || m.temp_mod == 0 {
            (p.kt1 + p.kt1l / p.leff) * (tratio - 1.0)
        } else {
            // tempMod = 2, 3.
            -p.kt1 * (tratio - 1.0)
        };
        let t5 = p.k1ox * (t0 - 1.0) * p.sqrt_phi + t3;
        p.vfbzbfactor = -t8 - t9 + p.k3 * t4 + t5 - p.phi - p.k1 * p.sqrt_phi;

        // Stress-effect size precompute (b4temp.c:1574-1601).
        let wlod = if m.wlod < 0.0 {
            log::warn!("BSIM4: WLOD = {} is less than zero; using zero", m.wlod);
            0.0
        } else {
            m.wlod
        };
        let w_tmp = wnew + wlod;
        let t0 = lnew.powf(m.llodku0);
        let t1 = w_tmp.powf(m.wlodku0);
        let tmp1 = m.lku0 / t0 + m.wku0 / t1 + m.pku0 / (t0 * t1);
        p.stress_ku0 = 1.0 + tmp1;

        let t0 = lnew.powf(m.llodvth);
        let t1 = w_tmp.powf(m.wlodvth);
        let tmp1 = m.lkvth0 / t0 + m.wkvth0 / t1 + m.pkvth0 / (t0 * t1);
        p.stress_kvth0 = 1.0 + tmp1;
        p.stress_kvth0 = (p.stress_kvth0 * p.stress_kvth0 + STRESS_DELTA).sqrt();

        p.stress_ku0temp = p.stress_ku0 * (1.0 + m.tku0 * (tratio - 1.0)) + STRESS_DELTA;
        let inv_saref = 1.0 / (m.saref + 0.5 * ldrn);
        let inv_sbref = 1.0 / (m.sbref + 0.5 * ldrn);
        p.stress_inv_od_ref = inv_saref + inv_sbref;
        p.stress_rho_ref = m.ku0 / p.stress_ku0temp * p.stress_inv_od_ref;

        // High-k mobility precompute for mobMod=3 (b4temp.c:1603-1646).
        if m.mob_mod == 3 {
            let lt1 = mt.factor1 * p.sqrt_xdep0;
            let theta0 = theta_form(p.dvt1 * p.leff / lt1);
            let tmp1 = epssub / p.xdep0;
            let tmp2 = p.nfactor * tmp1;
            let tmp3 = (tmp2 + p.cdsc * theta0 + p.cit) / mt.coxe;
            let n0 = if tmp3 >= -0.5 {
                1.0 + tmp3
            } else {
                let t0 = 1.0 / (3.0 + 8.0 * tmp3);
                (1.0 + 3.0 * tmp3) * t0
            };
            let t0 = n0 * mt.vtm;
            let t2 = p.voffcbn / t0;
            let t4 = if t2 < -EXP_THRESHOLD {
                p.mstar + mt.coxe * MIN_EXP / p.cdep0 * n0
            } else if t2 > EXP_THRESHOLD {
                p.mstar + mt.coxe * MAX_EXP / p.cdep0 * n0
            } else {
                p.mstar + t2.exp() * mt.coxe / p.cdep0 * n0
            };
            p.vgsteff_vth = t0 * 2.0_f64.ln() / t4;
        }

        // New DITS term added in 4.7.
        let t0 = -p.dvtp3 * p.leff.ln();
        p.dvtp2factor = p.dvtp5 + p.dvtp2 * dexp_temp(t0);

        // --- BSIM4checkModel (b4check.c) ---
        p.check_model(m, mt)?;

        Ok(p)
    }

    /// Port of `BSIM4checkModel` for the load-relevant parameters. Fatal
    /// excursions become `Err`; the value fixups run under `paramChk = 1`
    /// (the BSIM4 default) exactly as the C gates them, except the always-on
    /// `ckappas`/`ckappad` clamps.
    fn check_model(&mut self, m: &Bsim4v8Model, mt: &Bsim4v8ModelTemp) -> Result<(), String> {
        let p = self;
        let fatal = |msg: String| -> Result<(), String> { Err(format!("BSIM4: {msg}")) };

        if m.toxe <= 0.0 {
            return fatal(format!("Toxe = {:e} is not positive", m.toxe));
        }
        if m.toxp <= 0.0 {
            return fatal(format!("Toxp = {:e} is not positive", m.toxp));
        }
        if m.eot <= 0.0 {
            return fatal(format!("EOT = {:e} is not positive", m.eot));
        }
        if m.tempeot <= 0.0 {
            return fatal(format!("TEMPEOT = {:e} is not positive", m.tempeot));
        }
        if m.epsrgate < 0.0 {
            return fatal(format!("Epsrgate = {:e} is not positive", m.epsrgate));
        }
        if m.epsrsub < 0.0 {
            return fatal(format!("Epsrsub = {:e} is not positive", m.epsrsub));
        }
        if m.easub < 0.0 {
            return fatal(format!("Easub = {:e} is not positive", m.easub));
        }
        if m.ni0sub <= 0.0 {
            return fatal(format!("Ni0sub = {:e} is not positive", m.ni0sub));
        }
        if m.toxm <= 0.0 {
            return fatal(format!("Toxm = {:e} is not positive", m.toxm));
        }
        if m.toxref <= 0.0 {
            return fatal(format!("Toxref = {:e} is not positive", m.toxref));
        }
        if p.lpe0 < -p.leff {
            return fatal(format!("Lpe0 = {:e} is less than -Leff", p.lpe0));
        }
        if m.lintnoi > p.leff / 2.0 {
            return fatal(format!(
                "Lintnoi = {:e} is too large - Leff for noise is negative",
                m.lintnoi
            ));
        }
        if p.lpeb < -p.leff {
            return fatal(format!("Lpeb = {:e} is less than -Leff", p.lpeb));
        }
        if p.ndep <= 0.0 {
            return fatal(format!("Ndep = {:e} is not positive", p.ndep));
        }
        if p.phi <= 0.0 {
            return fatal(format!("Phi = {:e} is not positive", p.phi));
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
        // u0temp/vsattemp are knot-level here; stress/WPE/MULU0 adjust the
        // instance copies in `Bsim4v8InstTemp`.
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
        if p.fprout < 0.0 {
            return fatal(format!("fprout = {:e} is negative", p.fprout));
        }
        if p.pdits < 0.0 {
            return fatal(format!("pdits = {:e} is negative", p.pdits));
        }
        if m.pditsl < 0.0 {
            return fatal(format!("pditsl = {:e} is negative", m.pditsl));
        }
        if m.igb_mod != 0 {
            if p.nigbinv <= 0.0 {
                return fatal(format!("nigbinv = {:e} is non-positive", p.nigbinv));
            }
            if p.nigbacc <= 0.0 {
                return fatal(format!("nigbacc = {:e} is non-positive", p.nigbacc));
            }
        }
        if m.igc_mod != 0 {
            if p.nigc <= 0.0 {
                return fatal(format!("nigc = {:e} is non-positive", p.nigc));
            }
            if p.poxedge <= 0.0 {
                return fatal(format!("poxedge = {:e} is non-positive", p.poxedge));
            }
            if p.pigcd <= 0.0 {
                return fatal(format!("pigcd = {:e} is non-positive", p.pigcd));
            }
        }
        if p.clc < 0.0 {
            return fatal(format!("Clc = {:e} is negative", p.clc));
        }
        // Always-on overlap C-V clamps.
        if p.ckappas < 0.02 {
            log::warn!("BSIM4: ckappas = {} is too small; set to 0.02", p.ckappas);
            p.ckappas = 0.02;
        }
        if p.ckappad < 0.02 {
            log::warn!("BSIM4: ckappad = {} is too small; set to 0.02", p.ckappad);
            p.ckappad = 0.02;
        }
        for (v, name) in [
            (m.vtss, "Vtss"),
            (m.vtsd, "Vtsd"),
            (m.vtssws, "Vtssws"),
            (m.vtsswd, "Vtsswd"),
            (m.vtsswgs, "Vtsswgs"),
            (m.vtsswgd, "Vtsswgd"),
        ] {
            if v < 0.0 {
                return fatal(format!("{name} = {v:e} is negative"));
            }
        }
        let _ = mt;

        if m.param_chk == 1 {
            // Result-affecting fixups gated on paramChk (the default).
            if p.a2 < 0.01 {
                log::warn!("BSIM4: A2 = {} is too small; set to 0.01", p.a2);
                p.a2 = 0.01;
            } else if p.a2 > 1.0 {
                log::warn!(
                    "BSIM4: A2 = {} is larger than 1; A2 set to 1, A1 set to 0",
                    p.a2
                );
                p.a2 = 1.0;
                p.a1 = 0.0;
            }
            if p.prwg < 0.0 {
                log::warn!("BSIM4: Prwg = {} is negative; set to zero", p.prwg);
                p.prwg = 0.0;
            }
            if p.rdsw < 0.0 {
                log::warn!("BSIM4: Rdsw = {} is negative; set to zero", p.rdsw);
                p.rdsw = 0.0;
                p.rds0 = 0.0;
            }
            if p.rds0 < 0.0 {
                log::warn!(
                    "BSIM4: Rds at current temperature = {} is negative; set to zero",
                    p.rds0
                );
                p.rds0 = 0.0;
            }
            if p.rdswmin < 0.0 {
                log::warn!(
                    "BSIM4: Rdswmin at current temperature = {} is negative; set to zero",
                    p.rdswmin
                );
                p.rdswmin = 0.0;
            }
            if m.cgdo < 0.0 || m.cgso < 0.0 || m.cgbo < 0.0 {
                return Err(
                    "BSIM4: negative cgdo/cgso/cgbo with paramchk=1 (ngspice silently zeroes \
                     the model card; specify non-negative overlap capacitances instead)"
                        .to_string(),
                );
            }
            // xn < 3 / lc < 0 fixups (vtl case) deliberately do not run:
            // they would not reach the already-computed tfactor in the C
            // either, and xn/lc have no other load-side consumer.
        }
        Ok(())
    }
}

/// Instance geometry/options needed by the per-instance temperature tail and
/// the load (the `BSIM4instance` inputs of b4set.c/b4temp.c/b4ld.c).
#[derive(Debug, Clone, Copy)]
pub struct Bsim4v8Geometry {
    pub l: Value,
    pub w: Value,
    /// Number of fingers `NF`.
    pub nf: Value,
    /// Parallel multiplier `M` (applied by the stamp, not the eval).
    pub m: Value,
    /// Instance `GEOMOD` override; when not given, the model selector applies.
    pub geo_mod: i32,
    pub geo_mod_given: bool,
    /// Instance `RGEOMOD`: S/D resistance geometry selector.
    pub rgeo_mod: i32,
    pub rgeo_mod_given: bool,
    /// `MIN`: minimize either D (0) or S (1) diffusions for even NF.
    pub min_sd: i32,
    pub drain_area: Value,
    pub drain_area_given: bool,
    pub source_area: Value,
    pub source_area_given: bool,
    pub drain_perimeter: Value,
    pub drain_perimeter_given: bool,
    pub source_perimeter: Value,
    pub source_perimeter_given: bool,
    pub drain_squares: Value,
    pub drain_squares_given: bool,
    pub source_squares: Value,
    pub source_squares_given: bool,
    /// Zero-bias threshold shift `DELVTO`.
    pub delvto: Value,
    /// Low-field mobility multiplier `MULU0`.
    pub mulu0: Value,
    /// Instance temperature offset `DTEMP` (Celsius delta).
    pub dtemp: Value,
    /// Stress-effect layout distances (`SA`/`SB`/`SD`).
    pub sa: Value,
    pub sb: Value,
    pub sd: Value,
    /// Well-proximity spacing/integrals (`SC` derives SCA/SCB/SCC when none
    /// of the three integrals are given).
    pub sc: Value,
    pub sc_given: bool,
    pub sca: Value,
    pub sca_given: bool,
    pub scb: Value,
    pub scb_given: bool,
    pub scc: Value,
    pub scc_given: bool,
    /// `OFF`: start the device off in the initial DC iteration (engine
    /// seam; not used by the eval itself).
    pub off: bool,
    /// `IC=vds,vgs,vbs` initial conditions (engine seam for UIC).
    pub ic_vds: Value,
    pub ic_vgs: Value,
    pub ic_vbs: Value,
}

impl Default for Bsim4v8Geometry {
    /// Instance defaults per b4set.c (`l = w = 5u`, `nf = m = 1`, ...).
    fn default() -> Self {
        Self {
            l: 5.0e-6,
            w: 5.0e-6,
            nf: 1.0,
            m: 1.0,
            geo_mod: 0,
            geo_mod_given: false,
            rgeo_mod: 0,
            rgeo_mod_given: false,
            min_sd: 0,
            drain_area: 0.0,
            drain_area_given: false,
            source_area: 0.0,
            source_area_given: false,
            drain_perimeter: 0.0,
            drain_perimeter_given: false,
            source_perimeter: 0.0,
            source_perimeter_given: false,
            drain_squares: 1.0,
            drain_squares_given: false,
            source_squares: 1.0,
            source_squares_given: false,
            delvto: 0.0,
            mulu0: 1.0,
            dtemp: 0.0,
            sa: 0.0,
            sb: 0.0,
            sd: 0.0,
            sc: 0.0,
            sc_given: false,
            sca: 0.0,
            sca_given: false,
            scb: 0.0,
            scb_given: false,
            scc: 0.0,
            scc_given: false,
            off: false,
            ic_vds: 0.0,
            ic_vgs: 0.0,
            ic_vbs: 0.0,
        }
    }
}

/// `BSIM4NumFingerDiff` (b4geo.c): how many isolated/shared diffusions an
/// NF-finger device has on each side.
fn num_finger_diff(nf: Value, min_sd: i32) -> (Value, Value, Value, Value) {
    let nf_int = nf as i64;
    if nf_int % 2 != 0 {
        let nu_end = 1.0;
        let nu_int = 2.0 * ((nf - 1.0) / 2.0).max(0.0);
        (nu_int, nu_end, nu_int, nu_end)
    } else if min_sd == 1 {
        // Minimize the number of source diffusions.
        let nu_end_d = 2.0;
        let nu_int_d = 2.0 * (nf / 2.0 - 1.0).max(0.0);
        let nu_end_s = 0.0;
        let nu_int_s = nf;
        (nu_int_d, nu_end_d, nu_int_s, nu_end_s)
    } else {
        let nu_end_d = 0.0;
        let nu_int_d = nf;
        let nu_end_s = 2.0;
        let nu_int_s = 2.0 * (nf / 2.0 - 1.0).max(0.0);
        (nu_int_d, nu_end_d, nu_int_s, nu_end_s)
    }
}

/// `BSIM4PAeffGeo` diffusion geometry. Returns `(ps, pd, as_, ad)`.
fn pa_eff_geo(
    nf: Value,
    geo: i32,
    min_sd: i32,
    weff_cj: Value,
    dmcg: Value,
    dmci: Value,
    dmdg: Value,
) -> (Value, Value, Value, Value) {
    let (nu_int_d, nu_end_d, nu_int_s, nu_end_s) = if geo < 9 {
        num_finger_diff(nf, min_sd)
    } else {
        (0.0, 0.0, 0.0, 0.0)
    };
    let t0 = dmcg + dmci;
    let t1 = dmcg + dmcg;
    let t2 = dmdg + dmdg;
    let p_iso = t0 + t0 + weff_cj;
    let p_sha = t1;
    let p_mer = t2;
    let a_iso = t0 * weff_cj;
    let a_sha = dmcg * weff_cj;
    let a_mer = dmdg * weff_cj;

    match geo {
        0 => (
            nu_end_s * p_iso + nu_int_s * p_sha,
            nu_end_d * p_iso + nu_int_d * p_sha,
            nu_end_s * a_iso + nu_int_s * a_sha,
            nu_end_d * a_iso + nu_int_d * a_sha,
        ),
        1 => (
            nu_end_s * p_iso + nu_int_s * p_sha,
            (nu_end_d + nu_int_d) * p_sha,
            nu_end_s * a_iso + nu_int_s * a_sha,
            (nu_end_d + nu_int_d) * a_sha,
        ),
        2 => (
            (nu_end_s + nu_int_s) * p_sha,
            nu_end_d * p_iso + nu_int_d * p_sha,
            (nu_end_s + nu_int_s) * a_sha,
            nu_end_d * a_iso + nu_int_d * a_sha,
        ),
        3 => (
            (nu_end_s + nu_int_s) * p_sha,
            (nu_end_d + nu_int_d) * p_sha,
            (nu_end_s + nu_int_s) * a_sha,
            (nu_end_d + nu_int_d) * a_sha,
        ),
        4 => (
            nu_end_s * p_iso + nu_int_s * p_sha,
            nu_end_d * p_mer + nu_int_d * p_sha,
            nu_end_s * a_iso + nu_int_s * a_sha,
            nu_end_d * a_mer + nu_int_d * a_sha,
        ),
        5 => (
            (nu_end_s + nu_int_s) * p_sha,
            nu_end_d * p_mer + nu_int_d * p_sha,
            (nu_end_s + nu_int_s) * a_sha,
            nu_end_d * a_mer + nu_int_d * a_sha,
        ),
        6 => (
            nu_end_s * p_mer + nu_int_s * p_sha,
            nu_end_d * p_iso + nu_int_d * p_sha,
            nu_end_s * a_mer + nu_int_s * a_sha,
            nu_end_d * a_iso + nu_int_d * a_sha,
        ),
        7 => (
            nu_end_s * p_mer + nu_int_s * p_sha,
            (nu_end_d + nu_int_d) * p_sha,
            nu_end_s * a_mer + nu_int_s * a_sha,
            (nu_end_d + nu_int_d) * a_sha,
        ),
        8 => (
            nu_end_s * p_mer + nu_int_s * p_sha,
            nu_end_d * p_mer + nu_int_d * p_sha,
            nu_end_s * a_mer + nu_int_s * a_sha,
            nu_end_d * a_mer + nu_int_d * a_sha,
        ),
        9 => (
            p_iso + (nf - 1.0) * p_sha,
            nf * p_sha,
            a_iso + (nf - 1.0) * a_sha,
            nf * a_sha,
        ),
        10 => (
            nf * p_sha,
            p_iso + (nf - 1.0) * p_sha,
            nf * a_sha,
            a_iso + (nf - 1.0) * a_sha,
        ),
        _ => (0.0, 0.0, 0.0, 0.0),
    }
}

/// `BSIM4RdsEndIso` (`b4geo.c`): end resistance for isolated diffusions.
fn rds_end_iso(
    weff_cj: Value,
    rsh: Value,
    dmcg: Value,
    dmci: Value,
    nu_end: Value,
    rgeo: i32,
    source: bool,
) -> Value {
    if source {
        match rgeo {
            1 | 2 | 5 => {
                if nu_end == 0.0 {
                    0.0
                } else {
                    rsh * dmcg / (weff_cj * nu_end)
                }
            }
            3 | 4 | 6 => {
                let contact = dmcg + dmci;
                if nu_end == 0.0 || contact == 0.0 {
                    0.0
                } else {
                    rsh * weff_cj / (3.0 * nu_end * contact)
                }
            }
            _ => 0.0,
        }
    } else {
        match rgeo {
            1 | 3 | 7 => {
                if nu_end == 0.0 {
                    0.0
                } else {
                    rsh * dmcg / (weff_cj * nu_end)
                }
            }
            2 | 4 | 8 => {
                let contact = dmcg + dmci;
                if nu_end == 0.0 || contact == 0.0 {
                    0.0
                } else {
                    rsh * weff_cj / (3.0 * nu_end * contact)
                }
            }
            _ => 0.0,
        }
    }
}

/// `BSIM4RdsEndSha` (`b4geo.c`): end resistance for shared diffusions.
fn rds_end_sha(
    weff_cj: Value,
    rsh: Value,
    dmcg: Value,
    nu_end: Value,
    rgeo: i32,
    source: bool,
) -> Value {
    if source {
        match rgeo {
            1 | 2 | 5 => {
                if nu_end == 0.0 {
                    0.0
                } else {
                    rsh * dmcg / (weff_cj * nu_end)
                }
            }
            3 | 4 | 6 => {
                if nu_end == 0.0 || dmcg == 0.0 {
                    0.0
                } else {
                    rsh * weff_cj / (6.0 * nu_end * dmcg)
                }
            }
            _ => 0.0,
        }
    } else {
        match rgeo {
            1 | 3 | 7 => {
                if nu_end == 0.0 {
                    0.0
                } else {
                    rsh * dmcg / (weff_cj * nu_end)
                }
            }
            2 | 4 | 8 => {
                if nu_end == 0.0 || dmcg == 0.0 {
                    0.0
                } else {
                    rsh * weff_cj / (6.0 * nu_end * dmcg)
                }
            }
            _ => 0.0,
        }
    }
}

/// `BSIM4RdseffGeo` (`b4geo.c`): resistance from implicit S/D geometry.
#[allow(clippy::too_many_arguments)]
fn rdseff_geo(
    nf: Value,
    geo: i32,
    rgeo: i32,
    min_sd: i32,
    weff_cj: Value,
    rsh: Value,
    dmcg: Value,
    dmci: Value,
    dmdg: Value,
    source: bool,
) -> Value {
    if rgeo <= 0 || weff_cj <= 0.0 || rsh <= 0.0 {
        return 0.0;
    }

    let (nu_int_d, nu_end_d, nu_int_s, nu_end_s) = if geo < 9 {
        num_finger_diff(nf, min_sd)
    } else {
        (0.0, 0.0, 0.0, 0.0)
    };

    let mut rint = 0.0;
    if geo < 9 {
        let nu_int = if source { nu_int_s } else { nu_int_d };
        if nu_int != 0.0 {
            rint = rsh * dmcg / (weff_cj * nu_int);
        }
    }

    let rend = match geo {
        0 => {
            if source {
                rds_end_iso(weff_cj, rsh, dmcg, dmci, nu_end_s, rgeo, true)
            } else {
                rds_end_iso(weff_cj, rsh, dmcg, dmci, nu_end_d, rgeo, false)
            }
        }
        1 => {
            if source {
                rds_end_iso(weff_cj, rsh, dmcg, dmci, nu_end_s, rgeo, true)
            } else {
                rds_end_sha(weff_cj, rsh, dmcg, nu_end_d, rgeo, false)
            }
        }
        2 => {
            if source {
                rds_end_sha(weff_cj, rsh, dmcg, nu_end_s, rgeo, true)
            } else {
                rds_end_iso(weff_cj, rsh, dmcg, dmci, nu_end_d, rgeo, false)
            }
        }
        3 => {
            if source {
                rds_end_sha(weff_cj, rsh, dmcg, nu_end_s, rgeo, true)
            } else {
                rds_end_sha(weff_cj, rsh, dmcg, nu_end_d, rgeo, false)
            }
        }
        4 => {
            if source {
                rds_end_iso(weff_cj, rsh, dmcg, dmci, nu_end_s, rgeo, true)
            } else {
                rsh * dmdg / weff_cj
            }
        }
        5 => {
            if source {
                rds_end_sha(weff_cj, rsh, dmcg, nu_end_s, rgeo, true)
            } else {
                rsh * dmdg / (weff_cj * nu_end_d)
            }
        }
        6 => {
            if source {
                rsh * dmdg / weff_cj
            } else {
                rds_end_iso(weff_cj, rsh, dmcg, dmci, nu_end_d, rgeo, false)
            }
        }
        7 => {
            if source {
                rsh * dmdg / (weff_cj * nu_end_s)
            } else {
                rds_end_sha(weff_cj, rsh, dmcg, nu_end_d, rgeo, false)
            }
        }
        8 => rsh * dmdg / weff_cj,
        9 => {
            if source {
                if nf == 2.0 {
                    rint = 0.0;
                } else {
                    rint = rsh * dmcg / (weff_cj * (nf - 2.0));
                }
                0.5 * rsh * dmcg / weff_cj
            } else {
                rint = rsh * dmcg / (weff_cj * nf);
                0.0
            }
        }
        10 => {
            if source {
                rint = rsh * dmcg / (weff_cj * nf);
                0.0
            } else {
                if nf == 2.0 {
                    rint = 0.0;
                } else {
                    rint = rsh * dmcg / (weff_cj * (nf - 2.0));
                }
                0.5 * rsh * dmcg / weff_cj
            }
        }
        _ => 0.0,
    };

    if rint <= 0.0 {
        rend
    } else if rend <= 0.0 {
        rint
    } else {
        rint * rend / (rint + rend)
    }
}

/// Well-proximity effective scatter integral (`sceff`) from BSIM4temp.
fn wpe_sceff(model: &Bsim4v8Model, geom: &Bsim4v8Geometry) -> Value {
    let scref = if model.scref > 0.0 {
        model.scref
    } else {
        log::warn!("BSIM4: SCREF is not positive; using 1e-6 for WPE");
        1.0e-6
    };
    let mut sc = geom.sc;
    let mut sca = geom.sca;
    let mut scb = geom.scb;
    let mut scc = geom.scc;

    if !geom.sca_given && !geom.scb_given && !geom.scc_given {
        if geom.sc_given && geom.sc > 0.0 {
            let nf = if geom.nf.is_finite() && geom.nf > 0.0 {
                geom.nf
            } else {
                1.0
            };
            let wdrn = geom.w / nf;
            if wdrn > 0.0 {
                let t1 = geom.sc + wdrn;
                let t2 = 1.0 / scref;
                sca = scref * scref / (geom.sc * t1);
                scb = ((0.1 * geom.sc + 0.01 * scref) * (-10.0 * geom.sc * t2).exp()
                    - (0.1 * t1 + 0.01 * scref) * (-10.0 * t1 * t2).exp())
                    / wdrn;
                scc = ((0.05 * geom.sc + 0.0025 * scref) * (-20.0 * geom.sc * t2).exp()
                    - (0.05 * t1 + 0.0025 * scref) * (-20.0 * t1 * t2).exp())
                    / wdrn;
            } else {
                log::warn!("BSIM4: WPE SC given but drawn per-finger width is non-positive");
            }
        } else {
            log::warn!("BSIM4: WPE enabled but none of SCA, SCB, SCC, or positive SC is given");
        }
    }

    if sca < 0.0 {
        log::warn!("BSIM4: SCA is negative; set to zero");
        sca = 0.0;
    }
    if scb < 0.0 {
        log::warn!("BSIM4: SCB is negative; set to zero");
        scb = 0.0;
    }
    if scc < 0.0 {
        log::warn!("BSIM4: SCC is negative; set to zero");
        scc = 0.0;
    }
    if sc < 0.0 {
        log::warn!("BSIM4: SC is negative; set to zero");
        sc = 0.0;
    }
    let _ = sc;

    sca + model.web * scb + model.wec * scc
}

/// Per-instance temperature tail of `BSIM4temp` (post size knot).
#[derive(Debug, Clone, Default)]
pub struct Bsim4v8InstTemp {
    /// `here->BSIM4vth0` after stress/WPE plus `delvto`.
    pub vth0: Value,
    /// `pParam->vfb + type*delvto` (`here->BSIM4vfb`).
    pub vfb: Value,
    /// `pParam->vfbzbfactor + type*vth0` (`here->BSIM4vfbzb`).
    pub vfbzb: Value,
    /// `here->BSIM4u0temp` after stress/WPE plus `mulu0`.
    pub u0temp: Value,
    /// `here->BSIM4vsattemp` after stress.
    pub vsattemp: Value,
    /// `here->BSIM4eta0` after stress.
    pub eta0: Value,
    /// `here->BSIM4k2` after stress/WPE.
    pub k2: Value,
    /// `here->BSIM4k2ox = k2 * toxe / toxm`.
    pub k2ox: Value,
    /// `here->BSIM4vbsc`.
    pub vbsc: Value,
    /// `here->BSIM4vtfbphi1` (NMOS: `2(Vth0-Vfb-Phi)`, PMOS: 2.5x).
    pub vtfbphi1: Value,
    /// `here->BSIM4vtfbphi2 = 4(Vth0-Vfb-Phi)`, floored at 0.
    pub vtfbphi2: Value,
    /// `here->BSIM4toxp` / `coxp` (mtrlMod = 0: the model values).
    pub toxp: Value,
    pub coxp: Value,
    /// Effective junction perimeters/areas (`BSIM4Pseff` etc.).
    pub pseff: Value,
    pub pdeff: Value,
    pub aseff: Value,
    pub adeff: Value,
    /// Fixed drain-side S/D conductance from explicit squares or `RGEOMOD`.
    pub drain_conductance: Value,
    /// Fixed source-side S/D conductance from explicit squares or `RGEOMOD`.
    pub source_conductance: Value,
    /// `here->BSIM4grgeltd`, the constant electrode gate conductance for
    /// `RGATEMOD=1`.
    pub gate_conductance: Value,
    /// Body-resistance network conductances (`grbdb/grbpb/grbps/grbsb/grbpd`).
    pub body_drain_bulk_conductance: Value,
    pub body_prime_bulk_conductance: Value,
    pub body_prime_source_conductance: Value,
    pub body_source_bulk_conductance: Value,
    pub body_prime_drain_conductance: Value,
    /// ngspice `bodymode` for RBODYMOD noise/source selection: 0, 1, 3, or 5.
    pub body_resistance_mode: i32,
    /// Junction saturation currents (recomputed identically in b4ld.c).
    pub source_sat_current: Value,
    pub drain_sat_current: Value,
    /// `dioMod = 1/2` forward limiting anchors: `(vjsmFwd, IVjsmFwd)`.
    pub vjsm_fwd: Option<(Value, Value)>,
    pub vjdm_fwd: Option<(Value, Value)>,
    /// `dioMod = 0/2` breakdown exponential factors and `dioMod = 2`
    /// reverse limiting anchors/slopes.
    pub xexp_bvs: Value,
    pub xexp_bvd: Value,
    pub vjsm_rev: Value,
    pub vjdm_rev: Value,
    pub s_iv_rev: Value,
    pub d_iv_rev: Value,
    pub s_slp_fwd: Value,
    pub d_slp_fwd: Value,
    pub s_slp_rev: Value,
    pub d_slp_rev: Value,
    /// Reverse-bias trap-assisted saturation currents.
    pub s_jct_temp_rev_sat_cur: Value,
    pub d_jct_temp_rev_sat_cur: Value,
    pub s_sw_temp_rev_sat_cur: Value,
    pub d_sw_temp_rev_sat_cur: Value,
    pub s_swg_temp_rev_sat_cur: Value,
    pub d_swg_temp_rev_sat_cur: Value,
    /// Number of fingers (the load scales by it explicitly).
    pub nf: Value,
}

fn compat0_eot_toxp(
    m: &Bsim4v8Model,
    mt: &Bsim4v8ModelTemp,
    size: &Bsim4v8SizeDep,
    vth0: Value,
    vfb: Value,
) -> (Value, Value) {
    if m.mtrl_mod == 0 || m.mtrl_compat_mod != 0 {
        return (m.toxp, mt.coxp);
    }

    let toxe = m.effective_toxe();
    let epsrox = m.effective_epsrox();
    let vtm_eot = KB_OVER_Q * m.tempeot;
    let ni2 = mt.ni * mt.ni;
    let vbieot = vtm_eot * (size.nsd * size.ndep / ni2).ln();
    let phieot = vtm_eot * (size.ndep / mt.ni).ln() + size.phin + 0.4;
    if phieot <= 0.0 {
        log::warn!(
            "BSIM4: phieot = {phieot:e} is not positive during MTRLCOMPATMOD=0 TOXP iteration; using model TOXP"
        );
        return (m.toxp, mt.coxp);
    }

    let vfb_plus_phi = vfb + phieot;
    let vddeot = m.mtype * m.vddeot;
    let vgs_eff =
        poly_depletion_value(vfb_plus_phi, size.ngate, m.epsrgate * EPS0, mt.coxe, vddeot);

    let v0 = vbieot - phieot;
    let lt1 = mt.factor1 * size.sqrt_xdep0;
    let theta0 = theta_form(size.dvt1 * m.leffeot / lt1);
    let delt_vth = size.dvt0 * theta0 * v0;

    let theta0w = theta_form(size.dvt1w * m.weffeot * m.leffeot / lt1);
    let delt_vth_w = size.dvt0w * theta0w * v0;

    let temp_ratio_eot = m.tempeot / m.tnom - 1.0;
    let lpe = (1.0 + size.lpe0 / m.leffeot).sqrt();
    let temp_shift = (size.kt1 + size.kt1l / m.leffeot) * temp_ratio_eot;
    let k1_shift = size.k1ox * (lpe - 1.0) * phieot.sqrt() + temp_shift;
    let vth_narrow_w = toxe * phieot / (m.weffeot + size.w0);
    let lpe_vb = (1.0 + size.lpeb / m.leffeot).sqrt();
    let mut vth =
        m.mtype * vth0 + (size.k1ox - size.k1) * phieot.sqrt() * lpe_vb - delt_vth - delt_vth_w
            + size.k3 * vth_narrow_w
            + k1_shift;

    let tmp1 = mt.epssub / size.xdep0;
    let tmp2 = size.nfactor * tmp1;
    let tmp3 = (tmp2 + size.cdsc * theta0 + size.cit) / mt.coxe;
    let n = if tmp3 >= -0.5 {
        1.0 + tmp3
    } else {
        (1.0 + 3.0 * tmp3) / (3.0 + 8.0 * tmp3)
    };

    if size.dvtp0 > 0.0 {
        let pocket_length = m.leffeot + 2.0 * size.dvtp0;
        vth -= n * vtm_eot * (m.leffeot / pocket_length).ln();
    }

    let vgsteff = vgs_eff - vth;
    let vtfbphi2eot = (4.0 * (m.mtype * vth0 - vfb - phieot)).max(0.0);
    let mut toxpf = toxe;
    for _ in 0..=4 {
        let toxpi = toxpf;
        let tmp2 = 2.0e8 * toxpf;
        let t0 = (vgsteff + vtfbphi2eot) / tmp2;
        if t0 <= 0.0 {
            log::warn!("BSIM4: non-positive TOXP iteration argument {t0:e}; using model TOXP");
            return (m.toxp, mt.coxp);
        }
        let t1 = 1.0 + (m.bdos * 0.7 * t0.ln()).exp();
        let tcen = m.ados * 1.9e-9 / t1;
        toxpf = toxe - epsrox / m.epsrsub * tcen;
        if (toxpf - toxpi).abs() <= 1.0e-12 {
            break;
        }
    }

    (toxpf, epsrox * EPS0 / toxpf)
}

impl Bsim4v8InstTemp {
    pub fn new(
        model: &Bsim4v8Model,
        mt: &Bsim4v8ModelTemp,
        size: &Bsim4v8SizeDep,
        geom: &Bsim4v8Geometry,
    ) -> Self {
        let m = model;
        let nf = geom.nf;

        // Stress effect (b4temp.c:1656-1701). WPE remains an instance-tail
        // adjustment because its SC/SCA/SCB/SCC inputs are per-instance.
        let stress_active =
            geom.sa > 0.0 && geom.sb > 0.0 && (geom.nf == 1.0 || (geom.nf > 1.0 && geom.sd > 0.0));
        let (mut vth0_pre, eta0, mut k2, vsattemp, mut u0temp_pre) = if stress_active {
            let mut inv_sa = 0.0;
            let mut inv_sb = 0.0;
            let mut i = 0.0;
            while i < nf {
                let offset = i * (geom.sd + geom.l);
                inv_sa += 1.0 / nf / (geom.sa + 0.5 * geom.l + offset);
                inv_sb += 1.0 / nf / (geom.sb + 0.5 * geom.l + offset);
                i += 1.0;
            }

            let kvsat = if m.kvsat < -1.0 {
                log::warn!("BSIM4: KVSAT = {} is too small; using -1", m.kvsat);
                -1.0
            } else if m.kvsat > 1.0 {
                log::warn!("BSIM4: KVSAT = {} is too large; using 1", m.kvsat);
                1.0
            } else {
                m.kvsat
            };

            let inv_od_eff = inv_sa + inv_sb;
            let rho = m.ku0 / size.stress_ku0temp * inv_od_eff;
            let u0temp = size.u0temp * (1.0 + rho) / (1.0 + size.stress_rho_ref);
            let vsattemp =
                size.vsattemp * (1.0 + kvsat * rho) / (1.0 + kvsat * size.stress_rho_ref);

            let od_offset = inv_od_eff - size.stress_inv_od_ref;
            let dvth0_lod = m.kvth0 / size.stress_kvth0 * od_offset;
            let dk2_lod = m.stk2 / size.stress_kvth0.powf(m.lodk2) * od_offset;
            let deta0_lod = m.steta0 / size.stress_kvth0.powf(m.lodeta0) * od_offset;

            (
                size.vth0 + dvth0_lod,
                size.eta0 + deta0_lod,
                size.k2 + dk2_lod,
                vsattemp,
                u0temp,
            )
        } else {
            (size.vth0, size.eta0, size.k2, size.vsattemp, size.u0temp)
        };

        if m.wpemod != 0 {
            let sceff = wpe_sceff(m, geom);
            vth0_pre += size.kvth0we * sceff;
            k2 += size.k2we * sceff;
            let mut mobility_scale = 1.0 + size.ku0we * sceff;
            if mobility_scale <= 0.0 {
                log::warn!("BSIM4: KU0WE makes WPE mobility non-positive; clamped to zero");
                mobility_scale = 0.0;
            }
            u0temp_pre *= mobility_scale;
        }

        // delvto / mulu0.
        let vth0 = vth0_pre + geom.delvto;
        let vfb = size.vfb + m.mtype * geom.delvto;
        let u0temp = u0temp_pre * geom.mulu0;

        // Instance variables.
        let t3 = m.mtype * vth0 - vfb - size.phi;
        let t4 = t3 + t3;
        let t5 = 2.5 * t3;
        let vtfbphi1 = (if m.mtype > 0.0 { t4 } else { t5 }).max(0.0);
        let vtfbphi2 = (4.0 * t3).max(0.0);

        let mut vbsc = if k2 < 0.0 {
            let t0 = 0.5 * size.k1 / k2;
            let v = 0.9 * (size.phi - t0 * t0);
            v.clamp(-30.0, -3.0)
        } else {
            -30.0
        };
        if vbsc > size.vbm {
            vbsc = size.vbm;
        }
        let k2ox = k2 * m.effective_toxe() / m.toxm;
        let vfbzb = size.vfbzbfactor + m.mtype * vth0;
        let (toxp, coxp) = compat0_eot_toxp(m, mt, size, vth0, vfb);

        let rgeltd = m.rshg * (m.xgw + size.weff_cj / (3.0 * m.ngcon))
            / (m.ngcon * nf * (size.lnew - m.xgl));
        let gate_conductance = if rgeltd > 0.0 {
            1.0 / rgeltd
        } else {
            if m.rgate_mod != 0 {
                log::warn!("BSIM4: gate conductance reset to 1.0e3 mho");
            }
            1.0e3
        };

        // Effective junction perimeters and areas (New Diode Model v4.7).
        let dmcg_eff = m.dmcg - m.dmcgt;
        let dmci_eff = m.dmci;
        let dmdg_eff = m.dmdg - m.dmcgt;
        let geo_mod = if geom.geo_mod_given {
            geom.geo_mod
        } else {
            m.geo_mod
        };
        let rgeo_mod = if geom.rgeo_mod_given || geom.rgeo_mod != 0 {
            geom.rgeo_mod
        } else {
            m.rgeo_mod
        };
        let (geo_ps, geo_pd, geo_as, geo_ad) = pa_eff_geo(
            nf,
            geo_mod,
            geom.min_sd,
            size.weff_cj,
            dmcg_eff,
            dmci_eff,
            dmdg_eff,
        );

        let mut pseff = if geom.source_perimeter_given {
            if geom.source_perimeter == 0.0 {
                0.0
            } else if geom.source_perimeter < 0.0 {
                log::warn!("BSIM4: source perimeter is negative; set to zero");
                0.0
            } else if m.per_mod == 0 {
                geom.source_perimeter
            } else {
                geom.source_perimeter - size.weff_cj * nf
            }
        } else {
            geo_ps
        };
        if pseff < 0.0 {
            log::warn!("BSIM4: Pseff is negative; set to zero");
            pseff = 0.0;
        }
        let mut pdeff = if geom.drain_perimeter_given {
            if geom.drain_perimeter == 0.0 {
                0.0
            } else if geom.drain_perimeter < 0.0 {
                log::warn!("BSIM4: drain perimeter is negative; set to zero");
                0.0
            } else if m.per_mod == 0 {
                geom.drain_perimeter
            } else {
                geom.drain_perimeter - size.weff_cj * nf
            }
        } else {
            geo_pd
        };
        if pdeff < 0.0 {
            log::warn!("BSIM4: Pdeff is negative; set to zero");
            pdeff = 0.0;
        }
        let mut aseff = if geom.source_area_given {
            geom.source_area
        } else {
            geo_as
        };
        if aseff < 0.0 {
            log::warn!("BSIM4: Aseff is negative; set to zero");
            aseff = 0.0;
        }
        let mut adeff = if geom.drain_area_given {
            geom.drain_area
        } else {
            geo_ad
        };
        if adeff < 0.0 {
            log::warn!("BSIM4: Adeff is negative; set to zero");
            adeff = 0.0;
        }

        // Series conductances. RDSMOD=1 forces D'/S' and ngspice falls back
        // to a 1000 mho limiter when no explicit sheet path exists.
        let cond = |source: bool, squares_given: bool, squares: Value| {
            let resistance = if m.sheet_resistance > 0.0 {
                if squares_given {
                    Some(m.sheet_resistance * squares)
                } else if rgeo_mod > 0 {
                    Some(rdseff_geo(
                        nf,
                        geo_mod,
                        rgeo_mod,
                        geom.min_sd,
                        size.weff_cj,
                        m.sheet_resistance,
                        dmcg_eff,
                        dmci_eff,
                        dmdg_eff,
                        source,
                    ))
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(r) = resistance {
                if r > 0.0 {
                    1.0 / r
                } else if m.rds_mod != 0 {
                    1.0e3
                } else {
                    0.0
                }
            } else if m.rds_mod != 0 {
                1.0e3
            } else {
                0.0
            }
        };
        let source_conductance = cond(true, geom.source_squares_given, geom.source_squares);
        let drain_conductance = cond(false, geom.drain_squares_given, geom.drain_squares);
        let body_conductance = |resistance: Value| {
            if resistance < 1.0e-3 {
                1.0e3
            } else {
                m.gbmin + 1.0 / resistance
            }
        };
        let body_resistance_mode = if m.rbody_mod == 0 {
            0
        } else if m.rbody_mod == 1 {
            5
        } else if !m.rbps0_given || !m.rbpd0_given {
            1
        } else if (!m.rbsbx0_given && !m.rbsby0_given) || (!m.rbdbx0_given && !m.rbdby0_given) {
            3
        } else {
            5
        };
        let scaled_body_resistance =
            |base: Value, l_exp: Value, w_exp: Value, nf_exp: Value| -> Value {
                (base.ln()
                    + l_exp * (size.leff * 1.0e6).ln()
                    + w_exp * (size.weff * 1.0e6).ln()
                    + nf_exp * geom.nf.ln())
                .exp()
            };
        let parallel = |a: Value, b: Value| -> Value {
            let denom = a + b;
            if denom > 0.0 { a * b / denom } else { 0.0 }
        };
        let (rbdb, rbpb, rbsb, rbps, rbpd) = if m.rbody_mod == 2 {
            let rbpbx = scaled_body_resistance(m.rbpbx0, m.rbpbxl, m.rbpbxw, m.rbpbxnf);
            let rbpby = scaled_body_resistance(m.rbpby0, m.rbpbyl, m.rbpbyw, m.rbpbynf);
            let rbpb = parallel(rbpbx, rbpby);
            let (rbsb, rbdb) = if body_resistance_mode == 5 {
                let rbsbx = scaled_body_resistance(m.rbsbx0, m.rbsdbxl, m.rbsdbxw, m.rbsdbxnf);
                let rbsby = scaled_body_resistance(m.rbsby0, m.rbsdbyl, m.rbsdbyw, m.rbsdbynf);
                let rbdbx = scaled_body_resistance(m.rbdbx0, m.rbsdbxl, m.rbsdbxw, m.rbsdbxnf);
                let rbdby = scaled_body_resistance(m.rbdby0, m.rbsdbyl, m.rbsdbyw, m.rbsdbynf);
                (parallel(rbsbx, rbsby), parallel(rbdbx, rbdby))
            } else {
                (m.rbsb, m.rbdb)
            };
            let (rbps, rbpd) = if body_resistance_mode == 3 || body_resistance_mode == 5 {
                (
                    scaled_body_resistance(m.rbps0, m.rbpsl, m.rbpsw, m.rbpsnf),
                    scaled_body_resistance(m.rbpd0, m.rbpdl, m.rbpdw, m.rbpdnf),
                )
            } else {
                (m.rbps, m.rbpd)
            };
            (rbdb, rbpb, rbsb, rbps, rbpd)
        } else {
            (m.rbdb, m.rbpb, m.rbsb, m.rbps, m.rbpd)
        };
        let (
            body_drain_bulk_conductance,
            body_prime_bulk_conductance,
            body_prime_source_conductance,
            body_source_bulk_conductance,
            body_prime_drain_conductance,
        ) = if m.rbody_mod == 1 || (m.rbody_mod == 2 && body_resistance_mode == 5) {
            (
                body_conductance(rbdb),
                body_conductance(rbpb),
                body_conductance(rbps),
                body_conductance(rbsb),
                body_conductance(rbpd),
            )
        } else if m.rbody_mod == 2 && body_resistance_mode == 3 {
            (
                m.gbmin,
                body_conductance(rbpb),
                body_conductance(rbps),
                m.gbmin,
                body_conductance(rbpd),
            )
        } else if m.rbody_mod == 2 && body_resistance_mode == 1 {
            (m.gbmin, body_conductance(rbpb), 1.0e3, m.gbmin, 1.0e3)
        } else {
            (0.0, 0.0, 0.0, 0.0, 0.0)
        };

        // Junction saturation currents + diode model limiting anchors.
        let nvtms = mt.vtm * mt.njs;
        let source_sat_current = if aseff <= 0.0 && pseff <= 0.0 {
            0.0
        } else {
            aseff * mt.s_jct_temp_sat_cur_density
                + pseff * mt.s_jct_sidewall_temp_sat_cur_density
                + size.weff_cj * nf * mt.s_jct_gate_sidewall_temp_sat_cur_density
        };
        let mut vjsm_fwd = None;
        let mut xexp_bvs = 0.0;
        let mut vjsm_rev = 0.0;
        let mut s_iv_rev = 0.0;
        let mut s_slp_fwd = 0.0;
        let mut s_slp_rev = 0.0;
        if source_sat_current > 0.0 {
            match m.dio_mod {
                0 => {
                    xexp_bvs = if mt.bvs / nvtms > EXP_THRESHOLD {
                        m.xjbvs * MIN_EXP
                    } else {
                        m.xjbvs * (-mt.bvs / nvtms).exp()
                    };
                }
                1 => {
                    let v = dio_ijth_vjm_eval(nvtms, mt.ijthsfwd, source_sat_current, 0.0);
                    vjsm_fwd = Some((v, source_sat_current * (v / nvtms).exp()));
                }
                2 => {
                    xexp_bvs = if mt.bvs / nvtms > EXP_THRESHOLD {
                        MIN_EXP
                    } else {
                        (-mt.bvs / nvtms).exp()
                    };
                    xexp_bvs *= m.xjbvs;

                    let v = dio_ijth_vjm_eval(nvtms, mt.ijthsfwd, source_sat_current, xexp_bvs);
                    let t0 = (v / nvtms).exp();
                    let iv_fwd = source_sat_current * (t0 - xexp_bvs / t0 + xexp_bvs - 1.0);
                    s_slp_fwd = source_sat_current * (t0 + xexp_bvs / t0) / nvtms;
                    vjsm_fwd = Some((v, iv_fwd));

                    let mut t2 = mt.ijthsrev / source_sat_current;
                    if t2 < 1.0 {
                        t2 = 10.0;
                        log::warn!("BSIM4: ijthsrev too small and set to 10 times IsbSat");
                    }
                    vjsm_rev = -mt.bvs - nvtms * ((t2 - 1.0) / m.xjbvs).ln();
                    let t1 = m.xjbvs * (-(mt.bvs + vjsm_rev) / nvtms).exp();
                    s_iv_rev = source_sat_current * (1.0 + t1);
                    s_slp_rev = -source_sat_current * t1 / nvtms;
                }
                _ => {}
            }
        }

        let nvtmd = mt.vtm * mt.njd;
        let drain_sat_current = if adeff <= 0.0 && pdeff <= 0.0 {
            0.0
        } else {
            adeff * mt.d_jct_temp_sat_cur_density
                + pdeff * mt.d_jct_sidewall_temp_sat_cur_density
                + size.weff_cj * nf * mt.d_jct_gate_sidewall_temp_sat_cur_density
        };
        let mut vjdm_fwd = None;
        let mut xexp_bvd = 0.0;
        let mut vjdm_rev = 0.0;
        let mut d_iv_rev = 0.0;
        let mut d_slp_fwd = 0.0;
        let mut d_slp_rev = 0.0;
        if drain_sat_current > 0.0 {
            match m.dio_mod {
                0 => {
                    xexp_bvd = if mt.bvd / nvtmd > EXP_THRESHOLD {
                        m.xjbvd * MIN_EXP
                    } else {
                        m.xjbvd * (-mt.bvd / nvtmd).exp()
                    };
                }
                1 => {
                    let v = dio_ijth_vjm_eval(nvtmd, mt.ijthdfwd, drain_sat_current, 0.0);
                    vjdm_fwd = Some((v, drain_sat_current * (v / nvtmd).exp()));
                }
                2 => {
                    xexp_bvd = if mt.bvd / nvtmd > EXP_THRESHOLD {
                        MIN_EXP
                    } else {
                        (-mt.bvd / nvtmd).exp()
                    };
                    xexp_bvd *= m.xjbvd;

                    let v = dio_ijth_vjm_eval(nvtmd, mt.ijthdfwd, drain_sat_current, xexp_bvd);
                    let t0 = (v / nvtmd).exp();
                    let iv_fwd = drain_sat_current * (t0 - xexp_bvd / t0 + xexp_bvd - 1.0);
                    d_slp_fwd = drain_sat_current * (t0 + xexp_bvd / t0) / nvtmd;
                    vjdm_fwd = Some((v, iv_fwd));

                    let mut t2 = mt.ijthdrev / drain_sat_current;
                    if t2 < 1.0 {
                        t2 = 10.0;
                        log::warn!("BSIM4: ijthdrev too small and set to 10 times IdbSat");
                    }
                    vjdm_rev = -mt.bvd - nvtmd * ((t2 - 1.0) / m.xjbvd).ln();
                    let t1 = m.xjbvd * (-(mt.bvd + vjdm_rev) / nvtmd).exp();
                    d_iv_rev = drain_sat_current * (1.0 + t1);
                    d_slp_rev = -drain_sat_current * t1 / nvtmd;
                }
                _ => {}
            }
        }

        // Reverse-bias trap-assisted saturation currents.
        let jtweff = if m.jtweff < 0.0 {
            log::warn!("BSIM4: TAT width dependence effect is negative; Jtweff clamped to zero");
            0.0
        } else {
            m.jtweff
        };
        let t11 = (jtweff / size.weff_cj).sqrt() + 1.0;
        let t10 = size.weff_cj * nf;
        let s_jct_temp_rev_sat_cur = mt.xexp_tss * aseff * m.jtss;
        let d_jct_temp_rev_sat_cur = mt.xexp_tsd * adeff * m.jtsd;
        let s_sw_temp_rev_sat_cur = mt.xexp_tssws * pseff * m.jtssws;
        let d_sw_temp_rev_sat_cur = mt.xexp_tsswd * pdeff * m.jtsswd;
        let s_swg_temp_rev_sat_cur = mt.xexp_tsswgs * t10 * t11 * m.jtsswgs;
        let d_swg_temp_rev_sat_cur = mt.xexp_tsswgd * t10 * t11 * m.jtsswgd;

        Self {
            vth0,
            vfb,
            vfbzb,
            u0temp,
            vsattemp,
            eta0,
            k2,
            k2ox,
            vbsc,
            vtfbphi1,
            vtfbphi2,
            toxp,
            coxp,
            pseff,
            pdeff,
            aseff,
            adeff,
            drain_conductance,
            source_conductance,
            gate_conductance,
            body_drain_bulk_conductance,
            body_prime_bulk_conductance,
            body_prime_source_conductance,
            body_source_bulk_conductance,
            body_prime_drain_conductance,
            body_resistance_mode,
            source_sat_current,
            drain_sat_current,
            vjsm_fwd,
            vjdm_fwd,
            xexp_bvs,
            xexp_bvd,
            vjsm_rev,
            vjdm_rev,
            s_iv_rev,
            d_iv_rev,
            s_slp_fwd,
            d_slp_fwd,
            s_slp_rev,
            d_slp_rev,
            s_jct_temp_rev_sat_cur,
            d_jct_temp_rev_sat_cur,
            s_sw_temp_rev_sat_cur,
            d_sw_temp_rev_sat_cur,
            s_swg_temp_rev_sat_cur,
            d_swg_temp_rev_sat_cur,
            nf,
        }
    }
}

/// `BSIM4DioIjthVjmEval` (b4temp.c:52-65).
fn dio_ijth_vjm_eval(nvtm: Value, ijth: Value, isb: Value, xexp_bv: Value) -> Value {
    let tc = xexp_bv;
    let tb = 1.0 + ijth / isb - tc;
    let evjm_ov_nv = 0.5 * (tb + (tb * tb + 4.0 * tc).sqrt());
    nvtm * evjm_ov_nv.ln()
}

/// Size-knot cache mirroring `model->pSizeDependParamKnot`: ngspice walks
/// the knot list looking for an exact drawn (W, L, NF) match and computes a
/// new knot only on a miss. The C rebuilds the list inside every `BSIM4temp`
/// call, so the key also carries the temperature here.
#[derive(Debug, Default)]
pub struct SizeDepCache {
    knots: HashMap<(u64, u64, u64, u64), Arc<Bsim4v8SizeDep>>,
}

impl SizeDepCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Look up (or compute and memoize) the size knot for a drawn (W, L, NF)
    /// at the given device temperature.
    pub fn get(
        &mut self,
        model: &Bsim4v8Model,
        mt: &Bsim4v8ModelTemp,
        l: Value,
        w: Value,
        nf: Value,
    ) -> Result<Arc<Bsim4v8SizeDep>, String> {
        let key = (l.to_bits(), w.to_bits(), nf.to_bits(), mt.temp.to_bits());
        if let Some(knot) = self.knots.get(&key) {
            return Ok(Arc::clone(knot));
        }
        let knot = Arc::new(Bsim4v8SizeDep::new(model, mt, l, w, nf)?);
        self.knots.insert(key, Arc::clone(&knot));
        Ok(knot)
    }
}
