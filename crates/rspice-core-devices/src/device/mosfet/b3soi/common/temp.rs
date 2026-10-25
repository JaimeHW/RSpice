//! The size- and temperature-dependent record the three B3SOI variants share.

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
