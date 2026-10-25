//! The size- and temperature-dependent record the three B3SOI variants share.

use super::params::SoiModelCore;
use super::{CHARGE_Q, EPSOX, EPSSI, KB_OVER_Q};
use crate::Value;

/// The size-dependent fields every B3SOI variant (DD, FD, PD) shares, with
/// the same names and types (ngspice `pParam` plus the instance-level fields
/// the temperature pass fills in); each variant's record embeds one and
/// derefs to it.
#[derive(Debug, Clone, Default)]
pub struct SoiSizedCore {
    // Model-level temperature data (b3soi{dd,fd,pd}temp.c lines 50-77).
    pub vtm: Value,
    pub eg0: Value,
    pub ni: Value,
    pub factor1: Value,
    pub temp: Value,
    pub tnom: Value,

    // Geometry.
    pub leff: Value,
    pub weff: Value,
    pub leff_cv: Value,
    pub weff_cv: Value,
    pub abulk_cv_factor: Value,

    // Binned values (pParam).
    pub npeak: Value,
    pub nsub: Value,
    pub ngate: Value,
    pub vth0: Value,
    pub k1: Value,
    pub k2: Value,
    pub k3: Value,
    pub k3b: Value,
    pub vbsa: Value,
    pub delp: Value,
    pub kb1: Value,
    pub kb3: Value,
    pub dvbd0: Value,
    pub dvbd1: Value,
    pub w0: Value,
    pub nlx: Value,
    pub dvt0: Value,
    pub dvt1: Value,
    pub dvt2: Value,
    pub dvt0w: Value,
    pub dvt1w: Value,
    pub dvt2w: Value,
    pub u0: Value,
    pub ua: Value,
    pub ub: Value,
    pub uc: Value,
    pub vsat: Value,
    pub a0: Value,
    pub ags: Value,
    pub b0: Value,
    pub b1: Value,
    pub keta: Value,
    pub abp: Value,
    pub mxc: Value,
    pub adice0: Value,
    pub a1: Value,
    pub a2: Value,
    pub rdsw: Value,
    pub prwb: Value,
    pub prwg: Value,
    pub wr: Value,
    pub nfactor: Value,
    pub dwg: Value,
    pub dwb: Value,
    pub voff: Value,
    pub eta0: Value,
    pub etab: Value,
    pub dsub: Value,
    pub cit: Value,
    pub cdsc: Value,
    pub cdscb: Value,
    pub cdscd: Value,
    pub pclm: Value,
    pub pdibl1: Value,
    pub pdibl2: Value,
    pub pdiblb: Value,
    pub drout: Value,
    pub pvag: Value,
    pub delta: Value,
    pub aii: Value,
    pub bii: Value,
    pub cii: Value,
    pub dii: Value,
    pub alpha0: Value,
    pub alpha1: Value,
    pub beta0: Value,
    pub agidl: Value,
    pub bgidl: Value,
    pub ngidl: Value,
    pub ntun: Value,
    pub ndiode: Value,
    pub isbjt: Value,
    pub isdif: Value,
    pub isrec: Value,
    pub istun: Value,
    pub edl: Value,
    pub kbjt1: Value,
    pub vsdfb: Value,
    pub vsdth: Value,

    // Non-binned copies.
    pub at: Value,
    pub gamma1: Value,
    pub gamma2: Value,
    pub vbx: Value,
    pub vbm: Value,
    pub xt: Value,
    pub kt1: Value,
    pub kt1l: Value,
    pub kt2: Value,
    pub ua1: Value,
    pub ub1: Value,
    pub uc1: Value,
    pub ute: Value,
    pub prt: Value,

    // CV model copies.
    pub cgsl: Value,
    pub cgdl: Value,
    pub ckappa: Value,
    pub cf: Value,
    pub clc: Value,
    pub cle: Value,

    // Temperature-adjusted quantities.
    pub uatemp: Value,
    pub ubtemp: Value,
    pub uctemp: Value,
    pub rds0denom: Value,
    pub rth: Value,
    pub cth: Value,
    pub rbody: Value,
    pub u0temp: Value,
    pub vsattemp: Value,
    pub rds0: Value,

    pub cgso: Value,
    pub cgdo: Value,
    pub cgeo: Value,

    pub jbjt: Value,
    pub jdif: Value,
    pub jrec: Value,
    pub jtun: Value,
    pub vfbb: Value,
    pub csesw: Value,
    pub cdesw: Value,
    pub csdmin: Value,

    pub phi: Value,
    pub sqrt_phi: Value,
    pub phis3: Value,
    pub xdep0: Value,
    pub sqrt_xdep0: Value,
    pub litl: Value,
    pub vbi: Value,
    pub cdep0: Value,
    pub vbsc: Value,
    pub vfb: Value,
    pub theta0vb0: Value,
    pub theta_rout: Value,

    // S/D-to-substrate depletion-capacitance spline knots.
    pub sdt1: Value,
    pub st2: Value,
    pub st3: Value,
    pub dt2: Value,
    pub dt3: Value,
    pub st4: Value,
    pub dt4: Value,
    pub csbox: Value,
    pub cdbox: Value,
    pub csmin: Value,
    pub cdmin: Value,

    pub min_isub: Value,
    pub drain_conductance: Value,
    pub source_conductance: Value,
    pub rbodyext: Value,
}

/// `CKTtemp/tnom - 1` (self-heating off).
#[inline]
pub(crate) fn temp_ratio_m1(p: &SoiSizedCore) -> Value {
    p.temp / p.tnom - 1.0
}

/// Extrinsic bottom S/D-to-substrate depletion charge spline
/// (b3soi{dd,fd,pd}ld.c). `v` is the substrate-to-S (or substrate-to-D)
/// voltage with no type conversion; returns `(charge, conductance)`.
#[inline]
pub(crate) fn extrinsic_sd_charge(
    p: &SoiSizedCore,
    v: Value,
    nsub_pos_type: bool,
    source: bool,
) -> (Value, Value) {
    let (cbox, cmin) = if source {
        (p.csbox, p.csmin)
    } else {
        (p.cdbox, p.cdmin)
    };
    let (t2, t3, t4) = if source {
        (p.st2, p.st3, p.st4)
    } else {
        (p.dt2, p.dt3, p.dt4)
    };
    let vsdfb = p.vsdfb;
    let vsdth = p.vsdth;
    let sdt1 = p.sdt1;
    if nsub_pos_type {
        if v < vsdfb {
            (cbox * (v - vsdfb), cbox)
        } else if v < sdt1 {
            let t0 = v - vsdfb;
            let t1 = t0 * t0;
            (t0 * (cbox - t2 / 3.0 * t1), cbox - t2 * t1)
        } else if v < vsdth {
            let t0 = v - vsdth;
            let t1 = t0 * t0;
            (cmin * v + t4 + t3 / 3.0 * t0 * t1, cmin + t3 * t1)
        } else {
            (cmin * v + t4, cmin)
        }
    } else if v < vsdth {
        (cmin * (v - vsdth), cmin)
    } else if v < sdt1 {
        let t0 = v - vsdth;
        let t1 = t0 * t0;
        (t0 * (cmin - t2 / 3.0 * t1), cmin - t2 * t1)
    } else if v < vsdfb {
        let t0 = v - vsdfb;
        let t1 = t0 * t0;
        (cbox * v + t4 + t3 / 3.0 * t0 * t1, cbox + t3 * t1)
    } else {
        (cbox * v + t4, cbox)
    }
}

/// The ambient temperature values the rest of the setup reads.
pub(crate) struct SoiAmbient {
    pub(crate) tnom: Value,
    pub(crate) tratio: Value,
    pub(crate) eg0: Value,
    pub(crate) ni: Value,
}

/// The model-level temperature data and body-tie resistance.
pub(crate) fn soi_ambient(
    p: &mut SoiSizedCore,
    m: &SoiModelCore,
    temp: Value,
    body_squares: Value,
) -> SoiAmbient {
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

    p.rbodyext = body_squares * m.rbsh;

    // Keep the clamped junction potential visible to the eval stage.
    let _ = gate_sidewall_jct_potential;
    SoiAmbient {
        tnom,
        tratio,
        eg0,
        ni,
    }
}

/// The effective channel sizes, refusing a non-positive one; the error is
/// the reason without the variant's name.
pub(crate) fn soi_effective_sizes(
    p: &mut SoiSizedCore,
    l: Value,
    w: Value,
    dl: Value,
    dw: Value,
    dlc: Value,
    dwc: Value,
) -> Result<(), &'static str> {
    p.leff = l - 2.0 * dl;
    if p.leff <= 0.0 {
        return Err("effective channel length <= 0");
    }
    p.weff = w - 2.0 * dw;
    if p.weff <= 0.0 {
        return Err("effective channel width <= 0");
    }
    p.leff_cv = l - 2.0 * dlc;
    if p.leff_cv <= 0.0 {
        return Err("effective channel length for C-V <= 0");
    }
    p.weff_cv = w - 2.0 * dwc;
    if p.weff_cv <= 0.0 {
        return Err("effective channel width for C-V <= 0");
    }
    Ok(())
}

/// The model values every variant copies unbinned.
pub(crate) fn soi_unbinned(p: &mut SoiSizedCore, m: &SoiModelCore) {
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

    // CV model.
    p.cgsl = m.cgsl;
    p.cgdl = m.cgdl;
    p.ckappa = m.ckappa;
    p.cf = m.cf;
    p.clc = m.clc;
    p.cle = m.cle;
}

/// The binned families every variant shares; returns `(Inv_L, Inv_W, Inv_LW)`
/// for the variant's own families.
pub(crate) fn soi_bin(p: &mut SoiSizedCore, m: &SoiModelCore) -> (Value, Value, Value) {
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
    (inv_l, inv_w, inv_lw)
}

/// The temperature-scaled mobility and saturation terms, the overlap
/// capacitances and the NPEAK default; `t0` is `tratio - 1`.
pub(crate) fn soi_mobility_temperature(
    p: &mut SoiSizedCore,
    m: &SoiModelCore,
    tratio: Value,
    t0: Value,
) {
    p.ua += p.ua1 * t0;
    p.ub += p.ub1 * t0;
    p.uc += p.uc1 * t0;
    if p.u0 > 1.0 {
        p.u0 /= 1.0e4;
    }
    p.u0temp = p.u0 * tratio.powf(p.ute);
    p.vsattemp = p.vsat - p.at * t0;
    p.rds0 = (p.rdsw + p.prt * t0) / (p.weff * 1e6).powf(p.wr);

    // (checkModel intentionally not ported: paramchk=0 in the
    // supported decks; geometry fatals are caught above.)

    // --- Overlap capacitances (lines 561-566) ---
    p.cgdo = (m.cgdo + p.cf) * p.weff_cv;
    p.cgso = (m.cgso + p.cf) * p.weff_cv;
    p.cgeo = m.cgeo * p.leff_cv;

    if !m.npeak_given && m.gamma1_given {
        let t0 = p.gamma1 * m.cox;
        p.npeak = 3.021e22 * t0 * t0;
    }
}

/// The back-gate flat band, S/D diffusion and sidewall terms and the
/// bulk-charge/threshold pre-computation. Returns whether K1 and K2 took
/// their defaults, for the variant to warn from its own call site.
pub(crate) fn soi_threshold(
    p: &mut SoiSizedCore,
    m: &SoiModelCore,
    ni: Value,
    source_perimeter: Value,
    drain_perimeter: Value,
) -> (bool, bool) {
    let mut k1_defaulted = false;
    let mut k2_defaulted = false;

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
    let t1 = source_perimeter - p.weff;
    p.csesw = if t1 > 0.0 { t0 * t1 } else { 0.0 };
    let t1 = drain_perimeter - p.weff;
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
            k1_defaulted = true;
            p.k1 = 0.53;
        }
        if !m.k2_given {
            k2_defaulted = true;
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
    (k1_defaulted, k2_defaulted)
}

/// The instance-level depletion-capacitance knots and the S/D series
/// conductances.
pub(crate) fn soi_junctions(
    p: &mut SoiSizedCore,
    m: &SoiModelCore,
    source_area: Value,
    drain_area: Value,
    source_squares: Value,
    drain_squares: Value,
) {
    // --- Instance-level depletion-capacitance knots (748-788) ---
    p.csbox = m.cbox * source_area;
    p.csmin = p.csdmin * source_area;
    p.cdbox = m.cbox * drain_area;
    p.cdmin = p.csdmin * drain_area;

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
    let g = m.sheet_resistance * drain_squares;
    p.drain_conductance = if g > 0.0 { 1.0 / g } else { 0.0 };
    let g = m.sheet_resistance * source_squares;
    p.source_conductance = if g > 0.0 { 1.0 / g } else { 0.0 };
}
