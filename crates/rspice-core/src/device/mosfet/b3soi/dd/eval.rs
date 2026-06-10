//! B3SOIDD DC load equations (faithful port of ngspice-46 `b3soiddld.c`).
//!
//! This module transcribes the **DC current path** of `B3SOIDDload`
//! (b3soiddld.c lines ~860-2640): the SOI body-coupled threshold chain, the
//! BSIM3 `Vgsteff` smoothing, `Abulk`/`Abeff`, MOBMOD mobility, `Vdsat`, CLM /
//! DIBL / `Va`, the channel current `Ids`, and the SOI body currents (impact
//! ionization, GIDL, and the source/drain body diodes + parasitic BJT). The
//! result is the linearized operating point that ngspice stores in the
//! `here->B3SOIDD*` conductance/current fields and then stamps.
//!
//! Scope / provenance:
//! - Tested decks all use MOBMOD=0, CAPMOD=3, SHMOD=0 and either a floating
//!   body (`bodyMod=0`) or an ideal body tie (`bodyMod=2`). Accordingly:
//!   * Self-heating (`selfheat`) is **0** throughout; every `if (selfheat)`
//!     branch in the C reduces to the `else` (derivative = 0) and the temp
//!     node does not exist. The temperature-dependent quantities (jbjt, jdif,
//!     jrec, jtun, u0temp, vsattemp, rds0, ua/ub/uc, vbi, vfbb, phi, Xdep0)
//!     are taken from the precomputed [`B3SoiDdSized`] (the `else` branch of
//!     the big temp block at b3soiddld.c:803-822).
//!   * Body-resistor current `Ibp` (b3soiddld.c:2480-2540) is zero for
//!     `bodyMod` 0/2 and is therefore omitted here; the body tie is handled by
//!     the device stamping (the external body node is the body node directly).
//! - The **charge model** (CAPMOD=3, b3soiddld.c:2640-3400) and the matrix
//!   **stamping** (b3soiddld.c:3400-4460) are handled in [`super`]; this file
//!   stops at the `here->B3SOIDD*` operating-point assignment block
//!   (b3soiddld.c:2556-2640).
//!
//! Sign / mode convention matches ngspice exactly: all internal math is done in
//! the device's own polarity (`mtype` folded into the branch voltages by the
//! caller), and the normal/inverse `mode` swap (b3soiddld.c:836-860) is applied
//! to the *evaluation* voltages while the externally-meaningful currents are
//! re-expressed on the drain/source primes.

// `abulk0`/`dabulk0_dvb`/`exp_vgst` are computed in the DC path but only read
// by the (not-yet-ported) CAPMOD=3 charge model; ngspice keeps them here, so we
// retain the assignments for a faithful seam rather than dropping them.
#![allow(unused_assignments)]

use super::super::common::{EPSSI, EXP_THRESHOLD, MAX_EXP, MIN_EXP};
use super::super::common::{
    DELT_VBS0DIO, DELT_VBS0EFF, DELT_VBSDIO, DELT_VBSEFF, DELT_VBSMOS, DELT_XCSAT, OFF_VBSDIO,
};
use super::temp::B3SoiDdSized;
use crate::Value;

/// Linearized DC operating point of one B3SOIDD instance.
///
/// Field names mirror the `here->B3SOIDD*` slots that ngspice fills at the end
/// of the DC block (b3soiddld.c:2556-2640) and consumes during stamping. All
/// conductances are in the device's internal polarity; `mode` records the
/// normal(+1)/inverse(-1) channel direction.
#[derive(Debug, Clone, Default)]
pub struct B3SoiDdOp {
    pub mode: i32,

    /// Channel + collector current into the drain prime (`B3SOIDDcdrain`).
    pub cdrain: Value,
    /// Net drain-prime current `Ids + Ic - Ibd + Iii + Idgidl` (`B3SOIDDcd`).
    pub cd: Value,
    /// Net body current source term (`B3SOIDDcb`).
    pub cb: Value,

    /// Channel current `Ids` (`B3SOIDDids`).
    pub ids: Value,
    /// Threshold voltage at the operating point (`B3SOIDDvon`).
    pub von: Value,
    /// Saturation voltage (`B3SOIDDvdsat`).
    pub vdsat: Value,

    // Transconductances of the channel current (B3SOIDDg*).
    pub gm: Value,
    pub gds: Value,
    pub gmbs: Value,
    pub gme: Value,

    // Drain-side body junction current linearization (B3SOIDDgjd*, B3SOIDDcjd).
    pub gjdb: Value,
    pub gjdd: Value,
    pub gjdg: Value,
    pub gjde: Value,
    pub cjd: Value,

    // Source-side body junction current linearization (B3SOIDDgjs*, B3SOIDDcjs).
    pub gjsb: Value,
    pub gjsd: Value,
    pub gjsg: Value,
    pub cjs: Value,

    // Body-node KCL linearization (B3SOIDDgb*, B3SOIDDcbody).
    pub gbbs: Value,
    pub gbgs: Value,
    pub gbds: Value,
    pub gbes: Value,
    pub gbps: Value,
    pub cbody: Value,

    /// Inversion charge proxy used by noise (`B3SOIDDqinv`).
    pub qinv: Value,

    /// CAPMOD=3 charge state (set only when [`eval`] is asked to compute it).
    pub charge: Option<B3SoiDdCharge>,
}

/// CAPMOD=3 charge-model output for one B3SOIDD instance.
///
/// Mirrors the `here->B3SOIDDq*` node charges and the `here->B3SOIDDc*` intrinsic
/// capacitance matrix that ngspice fills at the end of `B3SOIDDload`
/// (b3soiddld.c:3387-3429) plus the extrinsic S/D-to-substrate spline charges
/// (b3soiddld.c:3438-3609) and the gate overlap charges (b3soiddld.c:3655-3784).
///
/// The four node charges (`qg/qb/qd/qe`) include the overlap and extrinsic lumps
/// exactly as ngspice does in the `mode>0` branch at b3soiddld.c:3722-3729, so
/// `qg+qb+qd+qe+qs == 0`. Capacitances are the `gc**b`-style derivatives *before*
/// multiplication by the integration coefficient `ag0` (the device applies `ag0`
/// when it forms the transient companion). All quantities are in device polarity
/// with `mtype` already folded in where ngspice folds it.
#[derive(Debug, Clone, Default)]
pub struct B3SoiDdCharge {
    /// Channel direction at evaluation (`here->B3SOIDDmode`).
    pub mode: i32,

    // Node charges (after overlap + extrinsic lumping, b3soiddld.c:3722-3729).
    pub qg: Value,
    pub qb: Value,
    pub qd: Value,
    pub qe: Value,

    // Intrinsic + overlap capacitance matrix (the `gc**`/ag0 coefficients).
    // Row = charge node, col = controlling node. Drain/source are the *primes*
    // (== external in the supported decks). Already includes overlap and
    // extrinsic S/D-substrate derivatives, matching b3soiddld.c:3680-3766.
    pub gcggb: Value,
    pub gcgdb: Value,
    pub gcgsb: Value,
    pub gcgeb: Value,
    pub gcbgb: Value,
    pub gcbdb: Value,
    pub gcbsb: Value,
    pub gcbeb: Value,
    pub gcdgb: Value,
    pub gcddb: Value,
    pub gcdsb: Value,
    pub gcdeb: Value,
    pub gcsgb: Value,
    pub gcsdb: Value,
    pub gcssb: Value,
    pub gcseb: Value,
    pub gcegb: Value,
    pub gcedb: Value,
    pub gcesb: Value,
    pub gceeb: Value,
}

/// Input branch voltages for the DC eval, already in device polarity
/// (`mtype` folded in by the caller) and *before* the normal/inverse swap.
///
/// `vbs`,`vgs`,`vds`,`ves`,`vps` correspond to the ngspice `vbs`/`vgs`/`vds`/
/// `ves`/`vps` after limiting (b3soiddld.c:836).
#[derive(Debug, Clone, Copy)]
pub struct B3SoiDdBias {
    pub vbs: Value,
    pub vgs: Value,
    pub vds: Value,
    pub ves: Value,
    pub vps: Value,
}

/// Evaluate the B3SOIDD DC operating point.
///
/// `p` is the size/temperature-resolved parameter set, `m_*` the few model-card
/// scalars needed in the load, and `bias` the device-polarity branch voltages.
/// `temp_k` is the (constant, no self-heating) device temperature in Kelvin and
/// `mtype` the polarity (+1 NMOS / -1 PMOS).
#[allow(clippy::too_many_lines)]
pub fn eval_dc(
    p: &B3SoiDdSized,
    m: &ModelConsts,
    bias: B3SoiDdBias,
    mtype: Value,
) -> B3SoiDdOp {
    eval(p, m, bias, mtype, false)
}

/// Evaluate the B3SOIDD operating point, optionally including the CAPMOD=3
/// charge model (`compute_charges == true`, the `ChargeComputationNeeded` path).
///
/// The DC current path is identical to [`eval_dc`]; when `compute_charges` is set
/// the resulting [`B3SoiDdOp::charge`] carries the intrinsic + extrinsic charge
/// state (b3soiddld.c:2637-3784, capMod==3, selfheat==0).
#[allow(clippy::too_many_lines)]
pub fn eval(
    p: &B3SoiDdSized,
    m: &ModelConsts,
    bias: B3SoiDdBias,
    mtype: Value,
    compute_charges: bool,
) -> B3SoiDdOp {
    let mut op = B3SoiDdOp::default();

    // --- Temperature-dependent quantities (selfheat == 0 branch,
    //     b3soiddld.c:803-822) ---
    let vbi = p.vbi;
    let vfbb = p.vfbb;
    let phi = p.phi;
    let sqrt_phi = p.sqrt_phi;
    let xdep0 = p.xdep0;
    let jbjt = p.jbjt;
    let jdif = p.jdif;
    let jrec = p.jrec;
    let jtun = p.jtun;
    let u0temp = p.u0temp;
    let vsattemp = p.vsattemp;
    let rds0 = p.rds0;
    // ua/ub/uc already temperature-shifted into p.ua/p.ub/p.uc by temp.rs
    // (b3soiddtemp.c folds in the *1*(tratio-1) term). selfheat==0 means no
    // further adjustment here.
    let ua = p.ua;
    let ub = p.ub;
    let uc = p.uc;

    let temp_ratio = temp_ratio_m1(p); // CKTtemp/tnom - 1 (selfheat==0)

    let vtm = p.vtm;

    // --- Mode setup (b3soiddld.c:836-860) ---
    let vbs0 = bias.vbs;
    let vgs0 = bias.vgs;
    let vds0 = bias.vds;
    let ves0 = bias.ves;
    let vps0 = bias.vps;

    let vbd0 = vbs0 - vds0;
    let vgd0 = vgs0 - vds0;
    let ved0 = ves0 - vds0;
    let vpd0 = vps0 - vds0;

    let (mode, vds, vgs, vbs, _vbd, ves, vps);
    if vds0 >= 0.0 {
        mode = 1;
        vds = vds0;
        vgs = vgs0;
        vbs = vbs0;
        _vbd = vbd0;
        ves = ves0;
        vps = vps0;
    } else {
        mode = -1;
        vds = -vds0;
        vgs = vgd0;
        vbs = vbd0;
        _vbd = vbs0;
        ves = ved0;
        vps = vpd0;
    }
    op.mode = mode;

    let vesfb = ves - vfbb;
    let cbox = m.cbox;
    let k1 = p.k1;

    // --- Poly-gate depletion (b3soiddld.c:886-905) ---
    let (vgs_eff, dvgs_eff_dvg);
    {
        let t0 = p.vfb + phi;
        if p.ngate > 1.0e18 && p.ngate < 1.0e25 && vgs > t0 {
            let t1 = 1.0e6 * m.charge_q * EPSSI * p.ngate / (m.cox * m.cox);
            let t4 = (1.0 + 2.0 * (vgs - t0) / t1).sqrt();
            let t2 = t1 * (t4 - 1.0);
            let t3 = 0.5 * t2 * t2 / t1;
            let t7 = 1.12 - t3 - 0.05;
            let t6 = (t7 * t7 + 0.224).sqrt();
            let t5 = 1.12 - 0.5 * (t7 + t6);
            vgs_eff = vgs - t5;
            dvgs_eff_dvg = 1.0 - (0.5 - 0.5 / t4) * (1.0 + t7 / t6);
        } else {
            vgs_eff = vgs;
            dvgs_eff_dvg = 1.0;
        }
    }

    let leff = p.leff;
    let v0 = vbi - phi;

    // --- Vbs0t (b3soiddld.c:923-933) ---
    let vbs0t;
    {
        let t0 = -p.dvbd1 * p.leff / p.litl;
        let t1 = p.dvbd0 * ((0.5 * t0).exp() + 2.0 * t0.exp());
        let t2 = t1 * (vbi - phi);
        let t3 = 0.5 * m.qsi / m.csi;
        vbs0t = phi - t3 + p.vbsa + t2;
    }

    // --- Vbs0 / Vbs0mos (b3soiddld.c:935-985) ---
    let (vbs0, dvbs0_dve);
    let (vbs0mos, dvbs0mos_dve);
    {
        let t0 = 1.0 + m.csieff / cbox;
        let t1 = p.kb1 / t0;
        let t2 = t1 * (vbs0t - vesfb);
        let t6 = vbs0t - t2;
        let dt6_dve = t1;

        // limit Vbs0 below phi
        let l1 = phi - p.delp;
        let t2b = l1 - t6 - DELT_VBSEFF;
        let t3 = (t2b * t2b + 4.0 * DELT_VBSEFF).sqrt();
        vbs0 = l1 - 0.5 * (t2b + t3);
        let t4 = 0.5 * (1.0 + t2b / t3);
        dvbs0_dve = t4 * dt6_dve;

        let t1b = vbs0t - vbs0 - DELT_VBSMOS;
        let t2c = (t1b * t1b + DELT_VBSMOS * DELT_VBSMOS).sqrt();
        let t3b = 0.5 * (t1b + t2c);
        let t4b = t3b * m.csieff / m.qsieff;
        vbs0mos = vbs0 - 0.5 * t3b * t4b;
        let t5 = 0.5 * t4b * (1.0 + t1b / t2c);
        dvbs0mos_dve = dvbs0_dve * (1.0 + t5);
    }

    // --- Vthfd (treat Vbs0mos as Vb), b3soiddld.c:990-1083 ---
    let factor1 = p.factor1;
    let vthfd;
    let dvthfd_dvd;
    let dvthfd_dve;
    {
        let phis = phi - vbs0mos;
        let sqrt_phis = phis.sqrt();
        let dsqrt_phis_dvb = -0.5 / sqrt_phis;
        let xdep = xdep0 * sqrt_phis / sqrt_phi;
        let dxdep_dvb = (xdep0 / sqrt_phi) * dsqrt_phis_dvb;
        let sqrt_xdep = xdep.sqrt();

        let (t1, t2) = smooth_dvt2(p.dvt2, vbs0mos);
        let lt1 = factor1 * sqrt_xdep * t1;
        let dlt1_dvb = factor1 * (0.5 / sqrt_xdep * t1 * dxdep_dvb + sqrt_xdep * t2);

        let (t1w, t2w) = smooth_dvt2(p.dvt2w, vbs0mos);
        let ltw = factor1 * sqrt_xdep * t1w;
        let dltw_dvb = factor1 * (0.5 / sqrt_xdep * t1w * dxdep_dvb + sqrt_xdep * t2w);

        // Theta0 / Delt_vth
        let t0 = -0.5 * p.dvt1 * leff / lt1;
        let (theta0, dtheta0_dvb);
        if t0 > -EXP_THRESHOLD {
            let t1e = t0.exp();
            let dt1_dvb = -t0 / lt1 * t1e * dlt1_dvb;
            theta0 = t1e * (1.0 + 2.0 * t1e);
            dtheta0_dvb = (1.0 + 4.0 * t1e) * dt1_dvb;
        } else {
            let t1e = MIN_EXP;
            theta0 = t1e * (1.0 + 2.0 * t1e);
            dtheta0_dvb = 0.0;
        }
        let thetavth = p.dvt0 * theta0;
        let delt_vth = thetavth * v0;
        let ddelt_vth_dvb = p.dvt0 * dtheta0_dvb * v0;

        // DeltVthw
        let t0w = -0.5 * p.dvt1w * p.weff * leff / ltw;
        let (t2dw, dt2dw_dvb);
        if t0w > -EXP_THRESHOLD {
            let t1e = t0w.exp();
            t2dw = t1e * (1.0 + 2.0 * t1e);
            let dt1_dvb = -t0w / ltw * t1e * dltw_dvb;
            dt2dw_dvb = (1.0 + 4.0 * t1e) * dt1_dvb;
        } else {
            let t1e = MIN_EXP;
            t2dw = t1e * (1.0 + 2.0 * t1e);
            dt2dw_dvb = 0.0;
        }
        let delt_vthw = p.dvt0w * t2dw * v0;
        let ddelt_vthw_dvb = p.dvt0w * dt2dw_dvb * v0;

        // DeltVthtemp
        let t0t = (1.0 + p.nlx / leff).sqrt();
        let t1t = p.kt1 + p.kt1l / leff + p.kt2 * vbs0mos;
        let delt_vthtemp = p.k1 * (t0t - 1.0) * sqrt_phi + t1t * temp_ratio;

        let tmp2 = m.tox * phi / (p.weff + p.w0);

        let (t3d, dt3_dvb) = smooth_etab(p.eta0, p.etab, vbs0mos);
        let dibl_sft = t3d * p.theta0vb0 * vds;
        let ddibl_sft_dvd = t3d * p.theta0vb0;
        let ddibl_sft_dvb = p.theta0vb0 * vds * dt3_dvb;

        vthfd = mtype * p.vth0 + p.k1 * (sqrt_phis - sqrt_phi) - p.k2 * vbs0mos - delt_vth
            - delt_vthw
            + (p.k3 + p.k3b * vbs0mos) * tmp2
            + delt_vthtemp
            - dibl_sft;

        let t6 = p.k3b * tmp2 - p.k2 + p.kt2 * temp_ratio;
        dvthfd_dvd = -ddibl_sft_dvd;
        let t7 = p.k1 * dsqrt_phis_dvb - ddelt_vth_dvb - ddelt_vthw_dvb + t6 - ddibl_sft_dvb;
        dvthfd_dve = t7 * dvbs0mos_dve;
    }

    // --- Vbs0teff / nfb / Vbs0eff (b3soiddld.c:1085-1145) ---
    let (vbs0teff, dvbs0teff_dvg, dvbs0teff_dvd, dvbs0teff_dve);
    let (vbs0eff, dvbs0eff_dvg, dvbs0eff_dvd, dvbs0eff_dve);
    {
        let t1 = vthfd - vgs_eff - DELT_VBS0EFF;
        let t2 = (t1 * t1 + DELT_VBS0EFF * DELT_VBS0EFF).sqrt();
        vbs0teff = vbs0t - 0.5 * (t1 + t2);
        let half = 0.5 * (1.0 + t1 / t2);
        dvbs0teff_dvg = half * dvgs_eff_dvg;
        dvbs0teff_dvd = -half * dvthfd_dvd;
        dvbs0teff_dve = -half * dvthfd_dve;

        // nfb
        let t3 = 1.0 / (k1 * k1);
        let t4 = p.kb3 * cbox / m.cox;
        let t8 = (phi - vbs0mos).sqrt();
        let t5 = (1.0 + 4.0 * t3 * (phi + k1 * t8 - vbs0mos)).sqrt();
        let t6 = 1.0 + t4 * t5;
        let nfb = 1.0 / t6;
        let t7 = 2.0 * t3 * t4 * nfb * nfb / t5 * (0.5 * k1 / t8 + 1.0);
        vbs0eff = vbs0 - nfb * 0.5 * (t1 + t2);
        dvbs0eff_dvg = nfb * half * dvgs_eff_dvg;
        dvbs0eff_dvd = -nfb * half * dvthfd_dvd;
        dvbs0eff_dve =
            dvbs0_dve - nfb * half * dvthfd_dve - t7 * 0.5 * (t1 + t2) * dvbs0mos_dve;
    }

    // --- Vbsdio (b3soiddld.c:1147-1162) ---
    let (vbsdio, dvbsdio_dvg, dvbsdio_dvd, dvbsdio_dve, dvbsdio_dvb);
    {
        let t1 = vbs - (vbs0eff + OFF_VBSDIO) - DELT_VBSDIO;
        let t2 = (t1 * t1 + DELT_VBSDIO * DELT_VBSDIO).sqrt();
        let t3 = 0.5 * (1.0 + t1 / t2);
        vbsdio = vbs0eff + OFF_VBSDIO + 0.5 * (t1 + t2);
        dvbsdio_dvg = (1.0 - t3) * dvbs0eff_dvg;
        dvbsdio_dvd = (1.0 - t3) * dvbs0eff_dvd;
        dvbsdio_dve = (1.0 - t3) * dvbs0eff_dve;
        dvbsdio_dvb = t3;
    }

    // --- Vbsmos (b3soiddld.c:1164-1183) ---
    let (vbsmos, dvbsmos_dvg, dvbsmos_dvd, dvbsmos_dvb, dvbsmos_dve);
    {
        let t1 = vbs0teff - vbsdio - DELT_VBSMOS;
        let t2 = (t1 * t1 + DELT_VBSMOS * DELT_VBSMOS).sqrt();
        let t3 = 0.5 * (t1 + t2);
        let t5 = 0.5 * (1.0 + t1 / t2);
        let dt3_dvg = t5 * (dvbs0teff_dvg - dvbsdio_dvg);
        let dt3_dvd = t5 * (dvbs0teff_dvd - dvbsdio_dvd);
        let dt3_dvb = -t5 * dvbsdio_dvb;
        let dt3_dve = t5 * (dvbs0teff_dve - dvbsdio_dve);
        let t4 = t3 * m.csieff / m.qsieff;
        vbsmos = vbsdio - 0.5 * t3 * t4;
        dvbsmos_dvg = dvbsdio_dvg - t4 * dt3_dvg;
        dvbsmos_dvd = dvbsdio_dvd - t4 * dt3_dvd;
        dvbsmos_dvb = dvbsdio_dvb - t4 * dt3_dvb;
        dvbsmos_dve = dvbsdio_dve - t4 * dt3_dve;
    }

    // --- Vcs (b3soiddld.c:1185-1191) ---
    let vcs = vbsdio - vbs0eff;
    let dvcs_dvb = dvbsdio_dvb;
    let dvcs_dvg = dvbsdio_dvg - dvbs0eff_dvg;
    let dvcs_dvd = dvbsdio_dvd - dvbs0eff_dvd;
    let dvcs_dve = dvbsdio_dve - dvbs0eff_dve;

    // --- Vps check / Vpsdio / Vbp (b3soiddld.c:1193-1230) ---
    // bodyMod 0/2: Ibp == 0 so Vbp/Vpsdio derivatives are not needed downstream.
    let _ = (vps, DELT_VBS0DIO);

    // --- Vbseff (b3soiddld.c:1235-1252) ---
    let (vbseff, dvbseff_dvg, dvbseff_dvd, dvbseff_dvb, dvbseff_dve);
    {
        let t1 = phi - p.delp;
        let t2 = t1 - vbsmos - DELT_VBSEFF;
        let t3 = (t2 * t2 + 4.0 * DELT_VBSEFF * t1).sqrt();
        vbseff = t1 - 0.5 * (t2 + t3);
        let t4 = 0.5 * (1.0 + t2 / t3);
        dvbseff_dvg = t4 * dvbsmos_dvg;
        dvbseff_dvd = t4 * dvbsmos_dvd;
        dvbseff_dvb = t4 * dvbsmos_dvb;
        dvbseff_dve = t4 * dvbsmos_dve;
    }

    // --- Vth (with Vbseff), b3soiddld.c:1254-1360 ---
    let phis = phi - vbseff;
    let sqrt_phis = phis.sqrt();
    let dsqrt_phis_dvb = -0.5 / sqrt_phis;
    let xdep = xdep0 * sqrt_phis / sqrt_phi;
    let dxdep_dvb = (xdep0 / sqrt_phi) * dsqrt_phis_dvb;

    let t3x = xdep.sqrt();
    let (t1a, t2a) = smooth_dvt2(p.dvt2, vbseff);
    let lt1 = factor1 * t3x * t1a;
    let dlt1_dvb = factor1 * (0.5 / t3x * t1a * dxdep_dvb + t3x * t2a);

    let (t1aw, t2aw) = smooth_dvt2(p.dvt2w, vbseff);
    let ltw = factor1 * t3x * t1aw;
    let dltw_dvb = factor1 * (0.5 / t3x * t1aw * dxdep_dvb + t3x * t2aw);

    let (theta0, dtheta0_dvb);
    {
        let t0 = -0.5 * p.dvt1 * leff / lt1;
        if t0 > -EXP_THRESHOLD {
            let t1 = t0.exp();
            theta0 = t1 * (1.0 + 2.0 * t1);
            let dt1_dvb = -t0 / lt1 * t1 * dlt1_dvb;
            dtheta0_dvb = (1.0 + 4.0 * t1) * dt1_dvb;
        } else {
            let t1 = MIN_EXP;
            theta0 = t1 * (1.0 + 2.0 * t1);
            dtheta0_dvb = 0.0;
        }
    }
    let thetavth = p.dvt0 * theta0;
    let delt_vth = thetavth * v0;
    let ddelt_vth_dvb = p.dvt0 * dtheta0_dvb * v0;

    let (deltvthw, ddeltvthw_dvb);
    {
        let t0 = -0.5 * p.dvt1w * p.weff * leff / ltw;
        let (t2v, dt2_dvb);
        if t0 > -EXP_THRESHOLD {
            let t1 = t0.exp();
            t2v = t1 * (1.0 + 2.0 * t1);
            let dt1_dvb = -t0 / ltw * t1 * dltw_dvb;
            dt2_dvb = (1.0 + 4.0 * t1) * dt1_dvb;
        } else {
            let t1 = MIN_EXP;
            t2v = t1 * (1.0 + 2.0 * t1);
            dt2_dvb = 0.0;
        }
        deltvthw = p.dvt0w * t2v * v0;
        ddeltvthw_dvb = p.dvt0w * dt2_dvb * v0;
    }

    let t0t = (1.0 + p.nlx / leff).sqrt();
    let t1t = p.kt1 + p.kt1l / leff + p.kt2 * vbseff;
    let delt_vthtemp = p.k1 * (t0t - 1.0) * sqrt_phi + t1t * temp_ratio;

    let tmp2 = m.tox * phi / (p.weff + p.w0);

    let (t3e, dt3_dvb_eta) = smooth_etab(p.eta0, p.etab, vbseff);
    let dibl_sft = t3e * p.theta0vb0 * vds;
    let ddibl_sft_dvd = p.theta0vb0 * t3e;
    let ddibl_sft_dvb = p.theta0vb0 * vds * dt3_dvb_eta;

    let vth = mtype * p.vth0 + p.k1 * (sqrt_phis - sqrt_phi) - p.k2 * vbseff - delt_vth
        - deltvthw
        + (p.k3 + p.k3b * vbseff) * tmp2
        + delt_vthtemp
        - dibl_sft;
    op.von = vth;

    let t6v = p.k3b * tmp2 - p.k2 + p.kt2 * temp_ratio;
    let dvth_dvb = p.k1 * dsqrt_phis_dvb - ddelt_vth_dvb - ddeltvthw_dvb + t6v - ddibl_sft_dvb;
    let dvth_dvd = -ddibl_sft_dvd;

    // --- n (subthreshold swing), b3soiddld.c:1363-1390 ---
    let (n, dn_dvb, dn_dvd);
    {
        let t2 = p.nfactor * EPSSI / xdep;
        let dt2_dvb = -t2 / xdep * dxdep_dvb;
        let t3 = p.cdsc + p.cdscb * vbseff + p.cdscd * vds;
        let dt3_dvb = p.cdscb;
        let dt3_dvd = p.cdscd;
        let t4 = (t2 + t3 * theta0 + p.cit) / m.cox;
        let dt4_dvb = (dt2_dvb + theta0 * dt3_dvb + dtheta0_dvb * t3) / m.cox;
        let dt4_dvd = theta0 * dt3_dvd / m.cox;
        if t4 >= -0.5 {
            n = 1.0 + t4;
            dn_dvb = dt4_dvb;
            dn_dvd = dt4_dvd;
        } else {
            let t0 = 1.0 / (3.0 + 8.0 * t4);
            n = (1.0 + 3.0 * t4) * t0;
            let t0sq = t0 * t0;
            dn_dvb = t0sq * dt4_dvb;
            dn_dvd = t0sq * dt4_dvd;
        }
    }

    // --- Vgsteff (b3soiddld.c:1393-1490) ---
    let vgst = vgs_eff - vth;
    let t10 = 2.0 * n * vtm;
    let vgst_n_vt = vgst / t10;
    let exp_arg = (2.0 * p.voff - vgst) / t10;

    let (vgsteff, dvgsteff_dvg, dvgsteff_dvd, dvgsteff_dvb, dvgsteff_dve);
    let mut exp_vgst = 0.0_f64;
    if vgst_n_vt > EXP_THRESHOLD {
        vgsteff = vgst;
        let t0 = -dvth_dvb;
        dvgsteff_dvg = dvgs_eff_dvg + t0 * dvbseff_dvg;
        dvgsteff_dvd = -dvth_dvd + t0 * dvbseff_dvd;
        dvgsteff_dvb = t0 * dvbseff_dvb;
        dvgsteff_dve = t0 * dvbseff_dve;
    } else if exp_arg > EXP_THRESHOLD {
        let t0 = (vgst - p.voff) / (n * vtm);
        exp_vgst = t0.exp();
        vgsteff = vtm * p.cdep0 / m.cox * exp_vgst;
        let t3 = vgsteff / (n * vtm);
        let t1 = -t3 * (dvth_dvb + t0 * vtm * dn_dvb);
        dvgsteff_dvg = t3 * dvgs_eff_dvg + t1 * dvbseff_dvg;
        dvgsteff_dvd = -t3 * (dvth_dvd + t0 * vtm * dn_dvd) + t1 * dvbseff_dvd;
        dvgsteff_dve = t1 * dvbseff_dve;
        dvgsteff_dvb = t1 * dvbseff_dvb;
    } else {
        exp_vgst = vgst_n_vt.exp();
        let t1 = t10 * (1.0 + exp_vgst).ln();
        let dt1_dvg = exp_vgst / (1.0 + exp_vgst);
        let dt1_dvb = -dt1_dvg * (dvth_dvb + vgst / n * dn_dvb) + t1 / n * dn_dvb;
        let dt1_dvd = -dt1_dvg * (dvth_dvd + vgst / n * dn_dvd) + t1 / n * dn_dvd;
        let dt2_dvg = -m.cox / (vtm * p.cdep0) * exp_arg.exp();
        let t2 = 1.0 - t10 * dt2_dvg;
        let dt2_dvd = -dt2_dvg * (dvth_dvd - 2.0 * vtm * exp_arg * dn_dvd) + (t2 - 1.0) / n * dn_dvd;
        let dt2_dvb = -dt2_dvg * (dvth_dvb - 2.0 * vtm * exp_arg * dn_dvb) + (t2 - 1.0) / n * dn_dvb;
        vgsteff = t1 / t2;
        let t3 = t2 * t2;
        let t4 = (t2 * dt1_dvb - t1 * dt2_dvb) / t3;
        dvgsteff_dvb = t4 * dvbseff_dvb;
        dvgsteff_dve = t4 * dvbseff_dve;
        dvgsteff_dvg = (t2 * dt1_dvg - t1 * dt2_dvg) / t3 * dvgs_eff_dvg + t4 * dvbseff_dvg;
        dvgsteff_dvd = (t2 * dt1_dvd - t1 * dt2_dvd) / t3 + t4 * dvbseff_dvd;
    }
    let vgst2vtm = vgsteff + 2.0 * vtm;

    // --- Effective W, Rds (b3soiddld.c:1492-1556) ---
    let t9w = sqrt_phis - sqrt_phi;
    let (mut weff, mut dweff_dvg, mut dweff_dvb);
    weff = p.weff - 2.0 * (p.dwg * vgsteff + p.dwb * t9w);
    dweff_dvg = -2.0 * p.dwg;
    dweff_dvb = -2.0 * p.dwb * dsqrt_phis_dvb;
    if weff < 2.0e-8 {
        let t0 = 1.0 / (6.0e-8 - 2.0 * weff);
        weff = 2.0e-8 * (4.0e-8 - weff) * t0;
        let t0sq = t0 * t0 * 4.0e-16;
        dweff_dvg *= t0sq;
        dweff_dvb *= t0sq;
    }

    let (rds, drds_dvg, drds_dvb);
    {
        let t0 = p.prwg * vgsteff + p.prwb * t9w;
        if t0 >= -0.9 {
            rds = rds0 * (1.0 + t0);
            drds_dvg = rds0 * p.prwg;
            drds_dvb = rds0 * p.prwb * dsqrt_phis_dvb;
        } else {
            let t1 = 1.0 / (17.0 + 20.0 * t0);
            rds = rds0 * (0.8 + t0) * t1;
            let t1sq = t1 * t1;
            drds_dvg = rds0 * p.prwg * t1sq;
            drds_dvb = rds0 * p.prwb * dsqrt_phis_dvb * t1sq;
        }
    }

    // --- Abulk / Abulk0 (b3soiddld.c:1558-1620) ---
    let (mut abulk0, mut abulk, mut dabulk0_dvb, mut dabulk_dvg, mut dabulk_dvb);
    if p.a0 == 0.0 {
        abulk0 = 0.0;
        abulk = 0.0;
        dabulk0_dvb = 0.0;
        dabulk_dvg = 0.0;
        dabulk_dvb = 0.0;
    } else {
        let t1 = 0.5 * p.k1 / sqrt_phi;
        let t9 = (m.xj * xdep).sqrt();
        let tmp1 = leff + 2.0 * t9;
        let t5 = leff / tmp1;
        let tmp2b = p.a0 * t5;
        let tmp3 = p.weff + p.b1;
        let tmp4 = p.b0 / tmp3;
        let t2 = tmp2b + tmp4;
        let dt2_dvb = -t9 * tmp2b / tmp1 / xdep * dxdep_dvb;
        let t6 = t5 * t5;
        let t7 = t5 * t6;
        abulk0 = t1 * t2;
        dabulk0_dvb = t1 * dt2_dvb;
        let t8 = p.ags * p.a0 * t7;
        dabulk_dvg = -t1 * t8;
        abulk = abulk0 + dabulk_dvg * vgsteff;
        dabulk_dvb = dabulk0_dvb - t8 * vgsteff * 3.0 * t1 * dt2_dvb / tmp2b;
    }
    if abulk0 < 0.01 {
        let t9 = 1.0 / (3.0 - 200.0 * abulk0);
        abulk0 = (0.02 - abulk0) * t9;
        dabulk0_dvb *= t9 * t9;
    }
    if abulk < 0.01 {
        let t9 = 1.0 / (3.0 - 200.0 * abulk);
        abulk = (0.02 - abulk) * t9;
        dabulk_dvb *= t9 * t9;
    }
    {
        let t2 = p.keta * vbseff;
        let (t0, dt0_dvb);
        if t2 >= -0.9 {
            t0 = 1.0 / (1.0 + t2);
            dt0_dvb = -p.keta * t0 * t0;
        } else {
            let t1 = 1.0 / (0.8 + t2);
            t0 = (17.0 + 20.0 * t2) * t1;
            dt0_dvb = -p.keta * t1 * t1;
        }
        dabulk_dvg *= t0;
        dabulk_dvb = dabulk_dvb * t0 + abulk * dt0_dvb;
        dabulk0_dvb = dabulk0_dvb * t0 + abulk0 * dt0_dvb;
        abulk *= t0;
        abulk0 *= t0;
    }
    abulk += 1.0;
    abulk0 += 1.0;

    // --- Abeff (b3soiddld.c:1623-1645) ---
    let (abeff, dabeff_dvg, dabeff_dvb, dabeff_dvc);
    {
        let t0 = p.abp * vgst2vtm;
        let t1 = 1.0 - vcs / t0 - DELT_XCSAT;
        let t2 = (t1 * t1 + DELT_XCSAT * DELT_XCSAT).sqrt();
        let t3 = 1.0 - 0.5 * (t1 + t2);
        let t5 = -0.5 * (1.0 + t1 / t2);
        let dt1_dvg = vcs / vgst2vtm / t0;
        let dt3_dvg = t5 * dt1_dvg;
        let dt1_dvc = -1.0 / t0;
        let dt3_dvc = t5 * dt1_dvc;
        let xcsat = p.mxc * t3 * t3 + (1.0 - p.mxc) * t3;
        let t4 = 2.0 * p.mxc * t3 + (1.0 - p.mxc);
        let dxcsat_dvg = t4 * dt3_dvg;
        let dxcsat_dvc = t4 * dt3_dvc;
        abeff = xcsat * abulk + (1.0 - xcsat) * m.adice;
        let t0a = xcsat * dabulk_dvg + abulk * dxcsat_dvg;
        dabeff_dvg = t0a - m.adice * dxcsat_dvg;
        dabeff_dvb = xcsat * dabulk_dvb;
        dabeff_dvc = (abulk - m.adice) * dxcsat_dvc;
    }

    // --- Mobility (MOBMOD 0 uses the mobMod==1 branch in ngspice, see note) ---
    // ngspice b3soiddld.c only implements mobMod 1/2/3; MOBMOD=0 cards fall to
    // the `else` (mobMod==3) path? No: the model defaults mobMod via mpar; for
    // these decks MOBMOD=0 -> handled as mobMod 1 in ngspice's load `if/else`
    // chain (`== 1` then `== 2` then else==3). Since 0 != 1 and 0 != 2, the
    // else (mobMod 3) executes. We replicate that exactly.
    let (ueff, dueff_dvg, dueff_dvd, dueff_dvb);
    {
        let (t5, mut ddenomi_dvg, mut ddenomi_dvd, mut ddenomi_dvb);
        if m.mob_mod == 1 {
            let t0 = vgsteff + vth + vth;
            let t2 = ua + uc * vbseff;
            let t3 = t0 / m.tox;
            t5 = t3 * (t2 + ub * t3);
            ddenomi_dvg = (t2 + 2.0 * ub * t3) / m.tox;
            ddenomi_dvd = ddenomi_dvg * 2.0 * dvth_dvd;
            ddenomi_dvb = ddenomi_dvg * 2.0 * dvth_dvb + uc * t3;
        } else if m.mob_mod == 2 {
            t5 = vgsteff / m.tox * (ua + uc * vbseff + ub * vgsteff / m.tox);
            ddenomi_dvg = (ua + uc * vbseff + 2.0 * ub * vgsteff / m.tox) / m.tox;
            ddenomi_dvd = 0.0;
            ddenomi_dvb = vgsteff * uc / m.tox;
        } else {
            let t0 = vgsteff + vth + vth;
            let t2 = 1.0 + uc * vbseff;
            let t3 = t0 / m.tox;
            let t4 = t3 * (ua + ub * t3);
            t5 = t4 * t2;
            ddenomi_dvg = (ua + 2.0 * ub * t3) * t2 / m.tox;
            ddenomi_dvd = ddenomi_dvg * 2.0 * dvth_dvd;
            ddenomi_dvb = ddenomi_dvg * 2.0 * dvth_dvb + uc * t4;
        }
        let denomi;
        if t5 >= -0.8 {
            denomi = 1.0 + t5;
        } else {
            let t9 = 1.0 / (7.0 + 10.0 * t5);
            denomi = (0.6 + t5) * t9;
            let t9sq = t9 * t9;
            ddenomi_dvg *= t9sq;
            ddenomi_dvd *= t9sq;
            ddenomi_dvb *= t9sq;
        }
        ueff = u0temp / denomi;
        let t9 = -ueff / denomi;
        dueff_dvg = t9 * ddenomi_dvg;
        dueff_dvd = t9 * ddenomi_dvd;
        dueff_dvb = t9 * ddenomi_dvb;
    }

    // --- Vdsat (b3soiddld.c:1719-1820) ---
    let wvcox = weff * vsattemp * m.cox;
    let wvcox_rds = wvcox * rds;
    let esat = 2.0 * vsattemp / ueff;
    let esat_l = esat * leff;
    let (desat_l_dvg, desat_l_dvd, desat_l_dvb);
    {
        let t0 = -esat_l / ueff;
        desat_l_dvg = t0 * dueff_dvg;
        desat_l_dvd = t0 * dueff_dvd;
        desat_l_dvb = t0 * dueff_dvb;
    }

    let (lambda, dlambda_dvg);
    {
        let a1 = p.a1;
        if a1 == 0.0 {
            lambda = p.a2;
            dlambda_dvg = 0.0;
        } else if a1 > 0.0 {
            let t0 = 1.0 - p.a2;
            let t1 = t0 - p.a1 * vgsteff - 0.0001;
            let t2 = (t1 * t1 + 0.0004 * t0).sqrt();
            lambda = p.a2 + t0 - 0.5 * (t1 + t2);
            dlambda_dvg = 0.5 * p.a1 * (1.0 + t1 / t2);
        } else {
            let t1 = p.a2 + p.a1 * vgsteff - 0.0001;
            let t2 = (t1 * t1 + 0.0004 * p.a2).sqrt();
            lambda = 0.5 * (t1 + t2);
            dlambda_dvg = 0.5 * p.a1 * (1.0 + t1 / t2);
        }
    }

    let (tmp2v, tmp3v);
    if rds > 0.0 {
        tmp2v = drds_dvg / rds + dweff_dvg / weff;
        tmp3v = drds_dvb / rds + dweff_dvb / weff;
    } else {
        tmp2v = dweff_dvg / weff;
        tmp3v = dweff_dvb / weff;
    }

    let vdsat;
    let (dvdsat_dvg, dvdsat_dvd, dvdsat_dvb, dvdsat_dvc);
    let mut tmp1l = 0.0;
    if rds == 0.0 && lambda == 1.0 {
        let t0 = 1.0 / (abeff * esat_l + vgst2vtm);
        let t1 = t0 * t0;
        let t2 = vgst2vtm * t0;
        let t3 = esat_l * vgst2vtm;
        vdsat = t3 * t0;
        let dt0_dvg = -(abeff * desat_l_dvg + esat_l * dabeff_dvg + 1.0) * t1;
        let dt0_dvd = -(abeff * desat_l_dvd) * t1;
        let dt0_dvb = -(abeff * desat_l_dvb + esat_l * dabeff_dvb) * t1;
        let dt0_dvc = -(esat_l * dabeff_dvc) * t1;
        dvdsat_dvg = t3 * dt0_dvg + t2 * desat_l_dvg + esat_l * t0;
        dvdsat_dvd = t3 * dt0_dvd + t2 * desat_l_dvd;
        dvdsat_dvb = t3 * dt0_dvb + t2 * desat_l_dvb;
        dvdsat_dvc = t3 * dt0_dvc;
    } else {
        tmp1l = dlambda_dvg / (lambda * lambda);
        let t9 = abeff * wvcox_rds;
        let t8 = abeff * t9;
        let t7 = vgst2vtm * t9;
        let t6 = vgst2vtm * wvcox_rds;
        let t0 = 2.0 * abeff * (t9 - 1.0 + 1.0 / lambda);
        let dt0_dvg = 2.0
            * (t8 * tmp2v - abeff * tmp1l + (2.0 * t9 + 1.0 / lambda - 1.0) * dabeff_dvg);
        let dt0_dvb = 2.0
            * (t8 * (2.0 / abeff * dabeff_dvb + tmp3v) + (1.0 / lambda - 1.0) * dabeff_dvb);
        let _dt0_dvd = 0.0; // ngspice dT0_dVd = 0 in this branch (unused below)
        let dt0_dvc = 4.0 * t9 * dabeff_dvc;

        let t1 = vgst2vtm * (2.0 / lambda - 1.0) + abeff * esat_l + 3.0 * t7;
        let dt1_dvg = (2.0 / lambda - 1.0) - 2.0 * vgst2vtm * tmp1l + abeff * desat_l_dvg
            + esat_l * dabeff_dvg
            + 3.0 * (t9 + t7 * tmp2v + t6 * dabeff_dvg);
        let dt1_dvb =
            abeff * desat_l_dvb + esat_l * dabeff_dvb + 3.0 * (t6 * dabeff_dvb + t7 * tmp3v);
        let dt1_dvd = abeff * desat_l_dvd;
        let dt1_dvc = esat_l * dabeff_dvc + 3.0 * t6 * dabeff_dvc;

        let t2 = vgst2vtm * (esat_l + 2.0 * t6);
        let dt2_dvg =
            esat_l + vgst2vtm * desat_l_dvg + t6 * (4.0 + 2.0 * vgst2vtm * tmp2v);
        let dt2_dvb = vgst2vtm * (desat_l_dvb + 2.0 * t6 * tmp3v);
        let dt2_dvd = vgst2vtm * desat_l_dvd;

        let t3 = (t1 * t1 - 2.0 * t0 * t2).sqrt();
        vdsat = (t1 - t3) / t0;
        dvdsat_dvg = (dt1_dvg - (t1 * dt1_dvg - dt0_dvg * t2 - t0 * dt2_dvg) / t3
            - vdsat * dt0_dvg)
            / t0;
        dvdsat_dvb = (dt1_dvb - (t1 * dt1_dvb - dt0_dvb * t2 - t0 * dt2_dvb) / t3
            - vdsat * dt0_dvb)
            / t0;
        dvdsat_dvd = (dt1_dvd - (t1 * dt1_dvd - t0 * dt2_dvd) / t3) / t0;
        dvdsat_dvc =
            (dt1_dvc - (t1 * dt1_dvc - dt0_dvc * t2) / t3 - vdsat * dt0_dvc) / t0;
    }
    op.vdsat = vdsat;

    // --- Vdsatii (impact-ionization Vdsat), b3soiddld.c:1823-1880 ---
    let (vdsatii, dvdsatii_dvg, dvdsatii_dvd, dvdsatii_dvb);
    if p.aii > 0.0 {
        let (t0c, dt0_dvd_c);
        if p.cii != 0.0 {
            let t0 = p.cii / 3.0_f64.sqrt() + p.dii;
            let t1 = vds - t0 - 0.1;
            let t2 = (t1 * t1 + 0.4).sqrt();
            let t3 = t0 + 0.5 * (t1 + t2);
            let dt3_dvd = 0.5 * (1.0 + t1 / t2);
            let t4 = t3 - p.dii;
            let t5 = p.cii / t4;
            let t0b = t5 * t5;
            t0c = t0b;
            dt0_dvd_c = -2.0 * t0b / t4 * dt3_dvd;
        } else {
            t0c = 0.0;
            dt0_dvd_c = 0.0;
        }
        let t0 = t0c + 1.0;
        let t3 = p.aii + p.bii / leff;
        let t4 = 1.0 / (t0 * vgsteff + t3 * esat_l);
        let t5 = -t4 * t4;
        let t7 = esat_l * vgsteff;
        vdsatii = t7 * t4;
        let dt4_dvg = t5 * (t0 + t3 * desat_l_dvg);
        let dt4_dvb = t5 * t3 * desat_l_dvb;
        let dt4_dvd = t5 * (vgsteff * dt0_dvd_c + t3 * desat_l_dvd);
        let t8 = t4 * vgsteff;
        dvdsatii_dvg = t7 * dt4_dvg + t4 * (esat_l + vgsteff * desat_l_dvg);
        dvdsatii_dvb = t7 * dt4_dvb + t8 * desat_l_dvb;
        dvdsatii_dvd = t7 * dt4_dvd + t8 * desat_l_dvd;
    } else {
        vdsatii = vdsat;
        dvdsatii_dvg = dvdsat_dvg;
        dvdsatii_dvb = dvdsat_dvb;
        dvdsatii_dvd = dvdsat_dvd;
    }

    // --- Vdseff (b3soiddld.c:1883-1918) ---
    let (vdseff, dvdseff_dvg, dvdseff_dvd, dvdseff_dvb, dvdseff_dvc);
    {
        let t1 = vdsat - vds - p.delta;
        let dt1_dvg = dvdsat_dvg;
        let dt1_dvd = dvdsat_dvd - 1.0;
        let dt1_dvb = dvdsat_dvb;
        let dt1_dvc = dvdsat_dvc;
        let t2 = (t1 * t1 + 4.0 * p.delta * vdsat).sqrt();
        let t0 = t1 / t2;
        let t3 = 2.0 * p.delta / t2;
        let dt2_dvg = t0 * dt1_dvg + t3 * dvdsat_dvg;
        let dt2_dvd = t0 * dt1_dvd + t3 * dvdsat_dvd;
        let dt2_dvb = t0 * dt1_dvb + t3 * dvdsat_dvb;
        let dt2_dvc = t0 * dt1_dvc + t3 * dvdsat_dvc;
        let mut vd = vdsat - 0.5 * (t1 + t2);
        let mut dg = dvdsat_dvg - 0.5 * (dt1_dvg + dt2_dvg);
        let mut dd = dvdsat_dvd - 0.5 * (dt1_dvd + dt2_dvd);
        let mut db = dvdsat_dvb - 0.5 * (dt1_dvb + dt2_dvb);
        let mut dc = dvdsat_dvc - 0.5 * (dt1_dvc + dt2_dvc);
        if vd > vds {
            vd = vds;
            dg = 0.0;
            db = 0.0;
            dc = 0.0;
            dd = 1.0;
        }
        vdseff = vd;
        dvdseff_dvg = dg;
        dvdseff_dvd = dd;
        dvdseff_dvb = db;
        dvdseff_dvc = dc;
    }
    let diff_vds = vds - vdseff;

    // --- Vdseffii (b3soiddld.c:1920-1945) ---
    let (vdseffii, dvdseffii_dvg, dvdseffii_dvd, dvdseffii_dvb);
    {
        let t1 = vdsatii - vds - p.delta;
        let t2 = (t1 * t1 + 4.0 * p.delta * vdsatii).sqrt();
        let t0 = t1 / t2;
        let t3 = 2.0 * p.delta / t2;
        let t4 = t0 + t3;
        let dt2_dvg = t4 * dvdsatii_dvg;
        let dt2_dvd = t4 * dvdsatii_dvd - t0;
        let dt2_dvb = t4 * dvdsatii_dvb;
        vdseffii = vdsatii - 0.5 * (t1 + t2);
        dvdseffii_dvg = 0.5 * (dvdsatii_dvg - dt2_dvg);
        dvdseffii_dvd = 0.5 * (dvdsatii_dvd - dt2_dvd + 1.0);
        dvdseffii_dvb = 0.5 * (dvdsatii_dvb - dt2_dvb);
    }
    let diff_vdsii = vds - vdseffii;

    // --- VAsat (b3soiddld.c:1948-1985) ---
    let (vasat, dvasat_dvg, dvasat_dvd, dvasat_dvb, dvasat_dvc);
    {
        let tmp4 = 1.0 - 0.5 * abeff * vdsat / vgst2vtm;
        let t9 = wvcox_rds * vgsteff;
        let t8 = t9 / vgst2vtm;
        let t0 = esat_l + vdsat + 2.0 * t9 * tmp4;
        let t7 = 2.0 * wvcox_rds * tmp4;
        let dt0_dvg = desat_l_dvg + dvdsat_dvg + t7 * (1.0 + tmp2v * vgsteff)
            - t8 * (abeff * dvdsat_dvg - abeff * vdsat / vgst2vtm + vdsat * dabeff_dvg);
        let dt0_dvb = desat_l_dvb + dvdsat_dvb + t7 * tmp3v * vgsteff
            - t8 * (dabeff_dvb * vdsat + abeff * dvdsat_dvb);
        let dt0_dvd = desat_l_dvd + dvdsat_dvd - t8 * abeff * dvdsat_dvd;
        let dt0_dvc = dvdsat_dvc - t8 * (abeff * dvdsat_dvc + vdsat * dabeff_dvc);
        let t9b = wvcox_rds * abeff;
        let t1 = 2.0 / lambda - 1.0 + t9b;
        let dt1_dvg = -2.0 * tmp1l + wvcox_rds * (abeff * tmp2v + dabeff_dvg);
        let dt1_dvb = dabeff_dvb * wvcox_rds + t9b * tmp3v;
        let dt1_dvc = dabeff_dvc * wvcox_rds;
        vasat = t0 / t1;
        dvasat_dvg = (dt0_dvg - vasat * dt1_dvg) / t1;
        dvasat_dvb = (dt0_dvb - vasat * dt1_dvb) / t1;
        dvasat_dvd = dt0_dvd / t1;
        dvasat_dvc = (dt0_dvc - vasat * dt1_dvc) / t1;
    }

    // --- VACLM (b3soiddld.c:1988-2018) ---
    let (vaclm, dvaclm_dvg, dvaclm_dvd, dvaclm_dvb, dvaclm_dvc);
    if p.pclm > 0.0 && diff_vds > 1.0e-10 {
        let t0 = 1.0 / (p.pclm * abeff * p.litl);
        let dt0_dvb = -t0 / abeff * dabeff_dvb;
        let dt0_dvg = -t0 / abeff * dabeff_dvg;
        let dt0_dvc = -t0 / abeff * dabeff_dvc;
        let t2 = vgsteff / esat_l;
        let t1 = leff * (abeff + t2);
        let dt1_dvg = leff * ((1.0 - t2 * desat_l_dvg) / esat_l + dabeff_dvg);
        let dt1_dvb = leff * (dabeff_dvb - t2 * desat_l_dvb / esat_l);
        let dt1_dvd = -t2 * desat_l_dvd / esat;
        let dt1_dvc = leff * dabeff_dvc;
        let t9 = t0 * t1;
        vaclm = t9 * diff_vds;
        dvaclm_dvg = t0 * dt1_dvg * diff_vds - t9 * dvdseff_dvg + t1 * diff_vds * dt0_dvg;
        dvaclm_dvb = (dt0_dvb * t1 + t0 * dt1_dvb) * diff_vds - t9 * dvdseff_dvb;
        dvaclm_dvd = t0 * dt1_dvd * diff_vds + t9 * (1.0 - dvdseff_dvd);
        dvaclm_dvc = (t1 * dt0_dvc + t0 * dt1_dvc) * diff_vds - t9 * dvdseff_dvc;
    } else {
        vaclm = MAX_EXP;
        dvaclm_dvd = 0.0;
        dvaclm_dvg = 0.0;
        dvaclm_dvb = 0.0;
        dvaclm_dvc = 0.0;
    }

    // --- VADIBL (b3soiddld.c:2021-2090) ---
    let (mut vadibl, mut dvadibl_dvg, mut dvadibl_dvd, mut dvadibl_dvb, mut dvadibl_dvc);
    if p.theta_rout > 0.0 {
        let t8 = abeff * vdsat;
        let t0 = vgst2vtm * t8;
        let t1 = vgst2vtm + t8;
        let dt0_dvg = vgst2vtm * abeff * dvdsat_dvg + t8 + vgst2vtm * vdsat * dabeff_dvg;
        let dt1_dvg = 1.0 + abeff * dvdsat_dvg + vdsat * dabeff_dvg;
        let dt1_dvb = dabeff_dvb * vdsat + abeff * dvdsat_dvb;
        let dt0_dvb = vgst2vtm * dt1_dvb;
        let dt1_dvd = abeff * dvdsat_dvd;
        let dt0_dvd = vgst2vtm * dt1_dvd;
        let dt1_dvc = abeff * dvdsat_dvc + vdsat * dabeff_dvc;
        let dt0_dvc = vgst2vtm * dt1_dvc;
        let t9 = t1 * t1;
        let t2 = p.theta_rout;
        vadibl = (vgst2vtm - t0 / t1) / t2;
        dvadibl_dvg = (1.0 - dt0_dvg / t1 + t0 * dt1_dvg / t9) / t2;
        dvadibl_dvb = (-dt0_dvb / t1 + t0 * dt1_dvb / t9) / t2;
        dvadibl_dvd = (-dt0_dvd / t1 + t0 * dt1_dvd / t9) / t2;
        dvadibl_dvc = (-dt0_dvc / t1 + t0 * dt1_dvc / t9) / t2;

        let t7 = p.pdiblb * vbseff;
        if t7 >= -0.9 {
            let t3 = 1.0 / (1.0 + t7);
            vadibl *= t3;
            dvadibl_dvg *= t3;
            dvadibl_dvb = (dvadibl_dvb - vadibl * p.pdiblb) * t3;
            dvadibl_dvd *= t3;
            dvadibl_dvc *= t3;
        } else {
            let t4 = 1.0 / (0.8 + t7);
            let t3 = (17.0 + 20.0 * t7) * t4;
            dvadibl_dvg *= t3;
            dvadibl_dvb = dvadibl_dvb * t3 - vadibl * p.pdiblb * t4 * t4;
            dvadibl_dvd *= t3;
            dvadibl_dvc *= t3;
            vadibl *= t3;
        }
    } else {
        vadibl = MAX_EXP;
        dvadibl_dvd = 0.0;
        dvadibl_dvg = 0.0;
        dvadibl_dvb = 0.0;
        dvadibl_dvc = 0.0;
    }

    // --- Va (b3soiddld.c:2093-2150) ---
    let (va, dva_dvg, dva_dvd, dva_dvb, dva_dvc);
    {
        let t8 = p.pvag / esat_l;
        let t9 = t8 * vgsteff;
        let (t0, dt0_dvg, dt0_dvb, dt0_dvd);
        if t9 > -0.9 {
            t0 = 1.0 + t9;
            dt0_dvg = t8 * (1.0 - vgsteff * desat_l_dvg / esat_l);
            dt0_dvb = -t9 * desat_l_dvb / esat_l;
            dt0_dvd = -t9 * desat_l_dvd / esat_l;
        } else {
            let t1 = 1.0 / (17.0 + 20.0 * t9);
            t0 = (0.8 + t9) * t1;
            let t1sq = t1 * t1;
            dt0_dvg = t8 * (1.0 - vgsteff * desat_l_dvg / esat_l) * t1sq;
            let t9b = t9 * t1sq / esat_l;
            dt0_dvb = -t9b * desat_l_dvb;
            dt0_dvd = -t9b * desat_l_dvd;
        }
        let tmp1 = vaclm * vaclm;
        let tmp2 = vadibl * vadibl;
        let mut tmp3 = vaclm + vadibl;
        let t1 = vaclm * vadibl / tmp3;
        tmp3 *= tmp3;
        let dt1_dvg = (tmp1 * dvadibl_dvg + tmp2 * dvaclm_dvg) / tmp3;
        let dt1_dvd = (tmp1 * dvadibl_dvd + tmp2 * dvaclm_dvd) / tmp3;
        let dt1_dvb = (tmp1 * dvadibl_dvb + tmp2 * dvaclm_dvb) / tmp3;
        let dt1_dvc = (tmp1 * dvadibl_dvc + tmp2 * dvaclm_dvc) / tmp3;
        va = vasat + t0 * t1;
        dva_dvg = dvasat_dvg + t1 * dt0_dvg + t0 * dt1_dvg;
        dva_dvd = dvasat_dvd + t1 * dt0_dvd + t0 * dt1_dvd;
        dva_dvb = dvasat_dvb + t1 * dt0_dvb + t0 * dt1_dvb;
        dva_dvc = dvasat_dvc + t0 * dt1_dvc;
    }

    // --- Ids (b3soiddld.c:2153-2230) ---
    let cox_wov_l = m.cox * weff / leff;
    let beta = ueff * cox_wov_l;
    let dbeta_dvg = cox_wov_l * dueff_dvg + beta * dweff_dvg / weff;
    let dbeta_dvd = cox_wov_l * dueff_dvd;
    let dbeta_dvb = cox_wov_l * dueff_dvb + beta * dweff_dvb / weff;

    let (fgche1, dfgche1_dvg, dfgche1_dvd, dfgche1_dvb, dfgche1_dvc);
    {
        let t0 = 1.0 - 0.5 * abeff * vdseff / vgst2vtm;
        let dt0_dvg = -0.5
            * (abeff * dvdseff_dvg - abeff * vdseff / vgst2vtm + vdseff * dabeff_dvg)
            / vgst2vtm;
        let dt0_dvd = -0.5 * abeff * dvdseff_dvd / vgst2vtm;
        let dt0_dvb =
            -0.5 * (abeff * dvdseff_dvb + dabeff_dvb * vdseff) / vgst2vtm;
        let dt0_dvc =
            -0.5 * (abeff * dvdseff_dvc + dabeff_dvc * vdseff) / vgst2vtm;
        fgche1 = vgsteff * t0;
        dfgche1_dvg = vgsteff * dt0_dvg + t0;
        dfgche1_dvd = vgsteff * dt0_dvd;
        dfgche1_dvb = vgsteff * dt0_dvb;
        dfgche1_dvc = vgsteff * dt0_dvc;
    }

    let (fgche2, dfgche2_dvg, dfgche2_dvd, dfgche2_dvb, dfgche2_dvc);
    {
        let t9 = vdseff / esat_l;
        fgche2 = 1.0 + t9;
        dfgche2_dvg = (dvdseff_dvg - t9 * desat_l_dvg) / esat_l;
        dfgche2_dvd = (dvdseff_dvd - t9 * desat_l_dvd) / esat_l;
        dfgche2_dvb = (dvdseff_dvb - t9 * desat_l_dvb) / esat_l;
        dfgche2_dvc = dvdseff_dvc / esat_l;
    }

    let gche = beta * fgche1 / fgche2;
    let dgche_dvg = (beta * dfgche1_dvg + fgche1 * dbeta_dvg - gche * dfgche2_dvg) / fgche2;
    let dgche_dvd = (beta * dfgche1_dvd + fgche1 * dbeta_dvd - gche * dfgche2_dvd) / fgche2;
    let dgche_dvb = (beta * dfgche1_dvb + fgche1 * dbeta_dvb - gche * dfgche2_dvb) / fgche2;
    let dgche_dvc = (beta * dfgche1_dvc - gche * dfgche2_dvc) / fgche2;

    let t0 = 1.0 + gche * rds;
    let t9 = vdseff / t0;
    let idl = gche * t9;
    let didl_dvg = (gche * dvdseff_dvg + t9 * dgche_dvg) / t0 - idl * gche / t0 * drds_dvg;
    let didl_dvd = (gche * dvdseff_dvd + t9 * dgche_dvd) / t0;
    let didl_dvb = (gche * dvdseff_dvb + t9 * dgche_dvb - idl * drds_dvb * gche) / t0;
    let didl_dvc = (gche * dvdseff_dvc + t9 * dgche_dvc) / t0;

    let t9d = diff_vds / va;
    let t0d = 1.0 + t9d;
    let ids = idl * t0d;
    op.ids = ids;

    let gm0 = t0d * didl_dvg - idl * (dvdseff_dvg + t9d * dva_dvg) / va;
    let gds0 = t0d * didl_dvd + idl * (1.0 - dvdseff_dvd - t9d * dva_dvd) / va;
    let gmb0 = t0d * didl_dvb - idl * (dvdseff_dvb + t9d * dva_dvb) / va;
    let gmc = t0d * didl_dvc - idl * (dvdseff_dvc + t9d * dva_dvc) / va;

    let gm = gm0 * dvgsteff_dvg + gmb0 * dvbseff_dvg + gmc * dvcs_dvg;
    let gmb = gm0 * dvgsteff_dvb + gmb0 * dvbseff_dvb + gmc * dvcs_dvb;
    let gds = gm0 * dvgsteff_dvd + gmb0 * dvbseff_dvd + gmc * dvcs_dvd + gds0;
    let gme = gm0 * dvgsteff_dve + gmb0 * dvbseff_dve + gmc * dvcs_dve;

    // --- Impact-ionization Iii (b3soiddld.c:2233-2290) ---
    let (mut iii, mut giig, mut giib, mut giid, mut giie) = (0.0, 0.0, 0.0, 0.0, 0.0);
    {
        let t2 = p.alpha1 + p.alpha0 / leff;
        if t2 <= 0.0 || p.beta0 <= 0.0 {
            // zeros
        } else {
            let t5 = p.beta0;
            let (t1, dt1_dvg, dt1_dvd, dt1_dvb);
            if diff_vdsii > t5 / EXP_THRESHOLD {
                let t0 = -t5 / diff_vdsii;
                let t10 = t0 / diff_vdsii;
                let dt0_dvg = t10 * dvdseffii_dvg;
                t1 = t2 * diff_vdsii * t0.exp();
                let t3 = t1 / diff_vdsii * (t0 - 1.0);
                dt1_dvg = t1 * (dt0_dvg - dvdseffii_dvg / diff_vdsii);
                dt1_dvd = -t3 * (1.0 - dvdseffii_dvd);
                dt1_dvb = t3 * dvdseffii_dvb;
            } else {
                let t3 = t2 * MIN_EXP;
                t1 = t3 * diff_vdsii;
                dt1_dvg = -t3 * dvdseffii_dvg;
                dt1_dvd = t3 * (1.0 - dvdseffii_dvd);
                dt1_dvb = -t3 * dvdseffii_dvb;
            }
            iii = t1 * ids;
            let t2i = t1 * gm0 + ids * dt1_dvg;
            let t3i = t1 * gds0 + ids * dt1_dvd;
            let t4i = t1 * gmb0 + ids * dt1_dvb;
            let t5i = t1 * gmc;
            giig = t2i * dvgsteff_dvg + t4i * dvbseff_dvg + t5i * dvcs_dvg;
            giib = t2i * dvgsteff_dvb + t4i * dvbseff_dvb + t5i * dvcs_dvb;
            giid = t2i * dvgsteff_dvd + t4i * dvbseff_dvd + t5i * dvcs_dvd + t3i;
            giie = t2i * dvgsteff_dve + t4i * dvbseff_dve + t5i * dvcs_dve;
        }
    }

    // --- GIDL (b3soiddld.c:2293-2350) ---
    let (mut idgidl, mut gdgidld, mut gdgidlg) = (0.0, 0.0, 0.0);
    let (mut isgidl, mut gsgidlg) = (0.0, 0.0);
    {
        let t0 = 3.0 * m.tox;
        // drain side
        let t1 = (vds - vgs_eff - p.ngidl) / t0;
        if p.agidl > 0.0 && p.bgidl > 0.0 && t1 > 0.0 {
            let dt1_dvd = 1.0 / t0;
            let dt1_dvg = -dt1_dvd * dvgs_eff_dvg;
            let t2 = p.bgidl / t1;
            if t2 < EXP_THRESHOLD {
                idgidl = p.weff * p.agidl * t1 * (-t2).exp();
                let t3 = idgidl / t1 * (t2 + 1.0);
                gdgidld = t3 * dt1_dvd;
                gdgidlg = t3 * dt1_dvg;
            } else {
                let t3 = p.weff * p.agidl * MIN_EXP;
                idgidl = t3 * t1;
                gdgidld = t3 * dt1_dvd;
                gdgidlg = t3 * dt1_dvg;
            }
        }
        // source side
        let t1 = (-vgs_eff - p.ngidl) / t0;
        if p.agidl > 0.0 && p.bgidl > 0.0 && t1 > 0.0 {
            let dt1_dvg = -dvgs_eff_dvg / t0;
            let t2 = p.bgidl / t1;
            if t2 < EXP_THRESHOLD {
                isgidl = p.weff * p.agidl * t1 * (-t2).exp();
                let t3 = isgidl / t1 * (t2 + 1.0);
                gsgidlg = t3 * dt1_dvg;
            } else {
                let t3 = p.weff * p.agidl * MIN_EXP;
                isgidl = t3 * t1;
                gsgidlg = t3 * dt1_dvg;
            }
        }
    }

    // --- Body diodes + parasitic BJT (b3soiddld.c:2353-2470) ---
    let w_tsi = p.weff * m.tsi;
    let n_vtm1 = vtm * p.ndiode;
    let n_vtm2 = vtm * p.ntun;

    let vbd = vbs - vds; // device-internal Vbd in evaluation frame
    let (exp_vbs1, dexp_vbs1_dvb) = exp_lin(vbs, n_vtm1);
    let (exp_vbd1, dexp_vbd1_dvb) = exp_lin(vbd, n_vtm1);
    let (exp_vbs4, dexp_vbs4_dvb, exp_vbd4, dexp_vbd4_dvb) = if jtun > 0.0 {
        let (a, da) = exp_lin(-vbs, n_vtm2);
        let (b, db) = exp_lin(-vbd, n_vtm2);
        (a, -da, b, -db)
    } else {
        (0.0, 0.0, 0.0, 0.0)
    };

    // Ibs1 / Ibd1 (diffusion)
    let (ibs1, dibs1_dvb, ibd1, dibd1_dvb, dibd1_dvd);
    if jdif == 0.0 {
        ibs1 = 0.0;
        dibs1_dvb = 0.0;
        ibd1 = 0.0;
        dibd1_dvb = 0.0;
        dibd1_dvd = 0.0;
    } else {
        let t5 = w_tsi * jdif;
        ibs1 = t5 * (exp_vbs1 - 1.0);
        dibs1_dvb = t5 * dexp_vbs1_dvb;
        ibd1 = t5 * (exp_vbd1 - 1.0);
        dibd1_dvb = t5 * dexp_vbd1_dvb;
        dibd1_dvd = -dibd1_dvb;
    }

    // Ibs2 / Ibd2 (recombination)
    let (ibs2, dibs2_dvb, ibd2, dibd2_dvb, dibd2_dvd);
    if jrec == 0.0 {
        ibs2 = 0.0;
        dibs2_dvb = 0.0;
        ibd2 = 0.0;
        dibd2_dvb = 0.0;
        dibd2_dvd = 0.0;
    } else {
        let exp_vbs2 = exp_vbs1.sqrt();
        let dexp_vbs2_dvb = if exp_vbs2 > 1e-20 {
            0.5 / exp_vbs2 * dexp_vbs1_dvb
        } else {
            0.0
        };
        let exp_vbd2 = exp_vbd1.sqrt();
        let dexp_vbd2_dvb = if exp_vbd2 > 1e-20 {
            0.5 / exp_vbd2 * dexp_vbd1_dvb
        } else {
            0.0
        };
        let t8 = w_tsi * jrec;
        ibs2 = t8 * (exp_vbs2 - 1.0);
        dibs2_dvb = t8 * dexp_vbs2_dvb;
        ibd2 = t8 * (exp_vbd2 - 1.0);
        dibd2_dvb = t8 * dexp_vbd2_dvb;
        dibd2_dvd = -dibd2_dvb;
    }

    // Ibjt / Ibs3 / Ibd3 (parasitic BJT), b3soiddld.c:2398-2440
    let (mut ic, mut gcd, mut gcb) = (0.0, 0.0, 0.0);
    let (ibs3, dibs3_dvb, dibs3_dvd, ibd3, dibd3_dvb, dibd3_dvd);
    if vds == 0.0 || jbjt == 0.0 {
        ibs3 = 0.0;
        dibs3_dvb = 0.0;
        dibs3_dvd = 0.0;
        ibd3 = 0.0;
        dibd3_dvb = 0.0;
        dibd3_dvd = 0.0;
    } else {
        let t0 = leff - p.kbjt1 * vds;
        let mut t1 = t0 / p.edl;
        let mut dt1_dvd = -p.kbjt1 / p.edl;
        if t1 < 1e-3 {
            let t2 = 1.0 / (3.0 - 2.0e3 * t1);
            t1 = (2.0e-3 - t1) * t2;
            dt1_dvd *= t2 * t2;
        } else if t1 > 1.0 {
            t1 = 1.0;
            dt1_dvd = 0.0;
        }
        let bjt_a = 1.0 - 0.5 * t1 * t1;
        let dbjt_a_dvd = -t1 * dt1_dvd;
        let t5 = w_tsi * jbjt;
        let ibjt = t5 * (exp_vbs1 - exp_vbd1);
        let dibjt_dvb = t5 * (dexp_vbs1_dvb - dexp_vbd1_dvb);
        let dibjt_dvd = t5 * dexp_vbd1_dvb;
        let t3 = (1.0 - bjt_a) * t5;
        let t4 = -t5 * dbjt_a_dvd;
        ibs3 = t3 * exp_vbs1;
        dibs3_dvb = t3 * dexp_vbs1_dvb;
        dibs3_dvd = t4 * exp_vbs1;
        ibd3 = t3 * exp_vbd1;
        dibd3_dvb = t3 * dexp_vbd1_dvb;
        dibd3_dvd = t4 * exp_vbd1 - dibd3_dvb;
        ic = ibjt - ibs3 + ibd3;
        gcd = dibjt_dvd - dibs3_dvd + dibd3_dvd;
        gcb = dibjt_dvb - dibs3_dvb + dibd3_dvb;
    }

    // Ibs4 / Ibd4 (tunneling)
    let (ibs4, dibs4_dvb, ibd4, dibd4_dvb, dibd4_dvd);
    if jtun == 0.0 {
        ibs4 = 0.0;
        dibs4_dvb = 0.0;
        ibd4 = 0.0;
        dibd4_dvb = 0.0;
        dibd4_dvd = 0.0;
    } else {
        let t5 = w_tsi * jtun;
        ibs4 = t5 * (1.0 - exp_vbs4);
        dibs4_dvb = -t5 * dexp_vbs4_dvb;
        ibd4 = t5 * (1.0 - exp_vbd4);
        dibd4_dvb = -t5 * dexp_vbd4_dvb;
        dibd4_dvd = -dibd4_dvb;
    }

    let ibs = ibs1 + ibs2 + ibs3 + ibs4;
    let ibd = ibd1 + ibd2 + ibd3 + ibd4;

    let gjsb = dibs1_dvb + dibs2_dvb + dibs3_dvb + dibs4_dvb;
    let gjsd = dibs3_dvd;
    let gjdb = dibd1_dvb + dibd2_dvb + dibd3_dvb + dibd4_dvb;
    let gjdd = dibd1_dvd + dibd2_dvd + dibd3_dvd + dibd4_dvd;

    // bodyMod 0/2: Ibp == 0.
    let min_isub = p.min_isub;

    // --- Operating-point assembly (b3soiddld.c:2556-2640) ---
    op.cdrain = ids + ic;
    op.cd = ids + ic - ibd + iii + idgidl;
    op.cb = ibs + ibd - iii - idgidl - isgidl;

    op.gds = gds + gcd;
    op.gm = gm;
    op.gmbs = gmb + gcb;
    op.gme = gme;

    // Drain-side junction current into drain prime.
    op.gjdb = gjdb - giib;
    op.gjdd = gjdd - (giid + gdgidld);
    op.gjdg = -(giig + gdgidlg);
    op.gjde = -giie;
    op.cjd = ibd - iii - idgidl - min_isub / 2.0
        - (op.gjdb * vbs + op.gjdd * vds + op.gjdg * vgs + op.gjde * ves);

    // Source-side junction current into source prime.
    op.gjsb = gjsb;
    op.gjsd = gjsd;
    op.gjsg = -gsgidlg;
    op.cjs = ibs - isgidl - min_isub / 2.0
        - (op.gjsb * vbs + op.gjsd * vds + op.gjsg * vgs);

    // Body-node KCL.
    op.gbbs = giib - gjsb - gjdb;
    op.gbgs = giig + gdgidlg + gsgidlg;
    op.gbds = giid + gdgidld - gjsd - gjdd;
    op.gbes = giie;
    op.gbps = 0.0;
    op.cbody = iii + idgidl + isgidl - ibs - ibd + min_isub
        - (op.gbbs * vbs + op.gbgs * vgs + op.gbds * vds + op.gbes * ves);

    // qinv for noise.
    let t1q = vgsteff * (1.0 - 0.5 * abeff * vdseff / vgst2vtm);
    op.qinv = -m.cox * p.weff * leff * t1q;

    if compute_charges {
        op.charge = Some(eval_charges_capmod3(
            p,
            m,
            mtype,
            mode,
            &ChargeInputs {
                phi,
                k1,
                cbox,
                vgs_eff,
                dvgs_eff_dvg,
                vth,
                dvth_dvb,
                dvth_dvd,
                vgst,
                vgsteff,
                dvgsteff_dvg,
                dvgsteff_dvd,
                dvgsteff_dvb,
                dvgsteff_dve,
                vgst_n_vt,
                exp_vgst,
                n,
                dn_dvb,
                dn_dvd,
                vbseff,
                dvbseff_dvg,
                dvbseff_dvd,
                dvbseff_dvb,
                dvbseff_dve,
                phis,
                sqrt_phis,
                dsqrt_phis_dvb,
                vdsat,
                dvdsat_dvg,
                dvdsat_dvd,
                dvdsat_dvb,
                dvdsat_dvc,
                vdseff,
                dvdseff_dvg,
                dvdseff_dvd,
                dvdseff_dvb,
                dvdseff_dvc,
                abulk0,
                dabulk0_dvb,
                vbs0t,
                vbs0,
                dvbs0_dve,
                vbs0mos,
                dvbs0mos_dve,
                vbs0eff,
                dvbs0eff_dvg,
                dvbs0eff_dvd,
                dvbs0eff_dve,
                vbsdio,
                dvbsdio_dvg,
                dvbsdio_dvd,
                dvbsdio_dve,
                dvbsdio_dvb,
                vcs,
                dvcs_dvg,
                dvcs_dvd,
                dvcs_dvb,
                dvcs_dve,
                vbs,
                vbd,
                ibs1,
                dibs1_dvb,
                ibd1,
                dibd1_dvb,
                dibd1_dvd,
                vgs_raw: bias.vgs,
                vgd_raw: bias.vgs - bias.vds,
                vge_raw: bias.vgs - bias.ves,
                vds_raw: bias.vds,
                ves_raw: bias.ves,
            },
        ));
    }

    op
}

/// Model-card scalars referenced directly in the load (not size-dependent).
#[derive(Debug, Clone, Copy)]
pub struct ModelConsts {
    pub cox: Value,
    pub cbox: Value,
    pub csi: Value,
    pub csieff: Value,
    pub qsi: Value,
    pub qsieff: Value,
    pub adice: Value,
    pub tox: Value,
    pub tsi: Value,
    pub xj: Value,
    pub charge_q: Value,
    pub mob_mod: i32,

    // CAPMOD=3 charge-model model-card scalars (b3soiddld.c CV block).
    /// Buried-oxide series capacitance per area `cboxt = cbox*csi/(cbox+csi)`.
    pub cboxt: Value,
    /// Charge partition selector (`B3SOIDDxpart`): >0.5 0/100, <0.5 40/60, else 50/50.
    pub xpart: Value,
    /// Transit time `TT` for the body junction stored charge.
    pub tt: Value,
    /// Body junction gate-side grading coefficient `MJSWG`.
    pub mjswg: Value,
    /// Body junction gate-side potential `PhiBSWG` (clamped >= 0.1 by the caller).
    pub phibswg: Value,
    /// Unit-length gate-sidewall junction capacitance `CJSWG`.
    pub cjswg: Value,
    /// Device polarity (`+1` NMOS / `-1` PMOS) for the extrinsic charge sign.
    pub mtype: Value,
}

#[inline]
fn temp_ratio_m1(p: &B3SoiDdSized) -> Value {
    p.temp / p.tnom - 1.0
}

/// Smooth `dvt2`-type discontinuity guard (b3soiddld.c:1020-1031 pattern).
/// Returns `(T1, T2)` where `T1 = 1 + dvt2*Vb` (smoothed) and `T2 = dT1/dVb`.
#[inline]
fn smooth_dvt2(dvt2: Value, vb: Value) -> (Value, Value) {
    let t0 = dvt2 * vb;
    if t0 >= -0.5 {
        (1.0 + t0, dvt2)
    } else {
        let t4 = 1.0 / (3.0 + 8.0 * t0);
        ((1.0 + 3.0 * t0) * t4, dvt2 * t4 * t4)
    }
}

/// Smooth `etab` guard (b3soiddld.c:1063-1074). Returns `(T3, dT3/dVb)`.
#[inline]
fn smooth_etab(eta0: Value, etab: Value, vb: Value) -> (Value, Value) {
    let t3 = eta0 + etab * vb;
    if t3 < 1.0e-4 {
        let t9 = 1.0 / (3.0 - 2.0e4 * t3);
        ((2.0e-4 - t3) * t9, t9 * t9 * etab)
    } else {
        (t3, etab)
    }
}

/// Linearized exp with ngspice's 30-clamp (b3soiddld.c:2356-2396).
/// Returns `(exp(v/nvt), d/dv)`.
#[inline]
fn exp_lin(v: Value, nvt: Value) -> (Value, Value) {
    let t0 = v / nvt;
    if t0 < 30.0 {
        let e = t0.exp();
        (e, e / nvt)
    } else {
        let t1 = 1.0686e13; // exp(30)
        let d = t1 / nvt;
        (d * v - 29.0 * t1, d)
    }
}

/// Intermediates from the DC path that the CAPMOD=3 charge model consumes.
///
/// Every field maps 1:1 to the like-named ngspice local at the point the charge
/// block runs (`raw` suffixes are the pre-mode-swap node-frame voltages ngspice
/// uses for the overlap/extrinsic lumps; everything else is the mode-swapped
/// evaluation frame). Grouped into a struct to keep [`eval`]'s call site legible.
#[derive(Debug, Clone, Copy)]
struct ChargeInputs {
    phi: Value,
    k1: Value,
    cbox: Value,
    vgs_eff: Value,
    dvgs_eff_dvg: Value,
    vth: Value,
    dvth_dvb: Value,
    dvth_dvd: Value,
    vgst: Value,
    vgsteff: Value,
    dvgsteff_dvg: Value,
    dvgsteff_dvd: Value,
    dvgsteff_dvb: Value,
    dvgsteff_dve: Value,
    vgst_n_vt: Value,
    exp_vgst: Value,
    n: Value,
    dn_dvb: Value,
    dn_dvd: Value,
    vbseff: Value,
    dvbseff_dvg: Value,
    dvbseff_dvd: Value,
    dvbseff_dvb: Value,
    dvbseff_dve: Value,
    phis: Value,
    sqrt_phis: Value,
    dsqrt_phis_dvb: Value,
    vdsat: Value,
    dvdsat_dvg: Value,
    dvdsat_dvd: Value,
    dvdsat_dvb: Value,
    dvdsat_dvc: Value,
    vdseff: Value,
    dvdseff_dvg: Value,
    dvdseff_dvd: Value,
    dvdseff_dvb: Value,
    dvdseff_dvc: Value,
    abulk0: Value,
    dabulk0_dvb: Value,
    vbs0t: Value,
    vbs0: Value,
    dvbs0_dve: Value,
    vbs0mos: Value,
    dvbs0mos_dve: Value,
    vbs0eff: Value,
    dvbs0eff_dvg: Value,
    dvbs0eff_dvd: Value,
    dvbs0eff_dve: Value,
    vbsdio: Value,
    dvbsdio_dvg: Value,
    dvbsdio_dvd: Value,
    dvbsdio_dve: Value,
    dvbsdio_dvb: Value,
    vcs: Value,
    dvcs_dvg: Value,
    dvcs_dvd: Value,
    dvcs_dvb: Value,
    dvcs_dve: Value,
    vbs: Value,
    vbd: Value,
    ibs1: Value,
    dibs1_dvb: Value,
    ibd1: Value,
    dibd1_dvb: Value,
    dibd1_dvd: Value,
    vgs_raw: Value,
    vgd_raw: Value,
    vge_raw: Value,
    vds_raw: Value,
    ves_raw: Value,
}

/// CAPMOD=3 charge model + extrinsic/overlap charges (b3soiddld.c:2646-3784).
///
/// Faithful transcription of the `capMod == 3` branch, the common capMod 2/3
/// backgate/inversion-charge code, the intrinsic S/D junction charge, the
/// extrinsic bottom-S/D-to-substrate spline, and the gate overlap charges, all
/// for `selfheat == 0` and the card's `xpart`. Returns the four node charges and
/// the intrinsic+overlap capacitance matrix (pre-`ag0`). `_mtype` is accepted for
/// symmetry with the DC eval; polarity is read from `m.mtype`.
#[allow(clippy::too_many_lines)]
fn eval_charges_capmod3(
    p: &B3SoiDdSized,
    m: &ModelConsts,
    _mtype: Value,
    mode: i32,
    i: &ChargeInputs,
) -> B3SoiDdCharge {
    use super::super::common::{CONST_2OV3, DELTA_1, DELTA_3, DELTA_VCSCV, QEX_FACT};

    let k1 = i.k1;
    let phi = i.phi;
    let cbox = i.cbox;

    // CoxWL (b3soiddld.c:2648).
    let cox_wl = m.cox * p.weff_cv * p.leff_cv;

    // Recompute Vgsteff,cv (b3soiddld.c:2659-2672) when in the moderate-inversion
    // window. Mirrors the DC subthreshold smoothing but with ExpVgst squared.
    let (mut vgsteff, mut dvgsteff_dvg, mut dvgsteff_dvd, mut dvgsteff_dvb, mut dvgsteff_dve) = (
        i.vgsteff,
        i.dvgsteff_dvg,
        i.dvgsteff_dvd,
        i.dvgsteff_dvb,
        i.dvgsteff_dve,
    );
    if i.vgst_n_vt > -EXP_THRESHOLD && i.vgst_n_vt < EXP_THRESHOLD {
        let exp_vgst = i.exp_vgst * i.exp_vgst;
        vgsteff = i.n * p.vtm * (1.0 + exp_vgst).ln();
        let t0 = exp_vgst / (1.0 + exp_vgst);
        let t1 = -t0 * (i.dvth_dvb + i.vgst / i.n * i.dn_dvb) + vgsteff / i.n * i.dn_dvb;
        dvgsteff_dvd = -t0 * (i.dvth_dvd + i.vgst / i.n * i.dn_dvd)
            + vgsteff / i.n * i.dn_dvd
            + t1 * i.dvbseff_dvd;
        dvgsteff_dvg = t0 * i.dvgs_eff_dvg + t1 * i.dvbseff_dvg;
        dvgsteff_dvb = t1 * i.dvbseff_dvb;
        dvgsteff_dve = t1 * i.dvbseff_dve;
    }

    // Vfb (b3soiddld.c:2675-2678). dPhis_dVb == -1, dsqrtPhis_dVb == -0.5/sqrtPhis.
    let sqrt_phis = i.sqrt_phis;
    let dsqrt_phis_dvb = i.dsqrt_phis_dvb;
    let phis = i.phis;
    let d_phis_dvb = -1.0;
    let vfb = i.vth - phi - k1 * sqrt_phis;
    let dvfb_dvb = i.dvth_dvb - k1 * dsqrt_phis_dvb;
    let dvfb_dvd = i.dvth_dvd;

    // Vgsteff += 1e-4 (b3soiddld.c:2685).
    vgsteff += 1e-4;

    // Vfbeff (b3soiddld.c:2688-2704).
    let v3 = vfb - i.vgs_eff + i.vbseff - DELTA_3;
    let (t0fb, t2fb);
    if vfb <= 0.0 {
        t0fb = (v3 * v3 - 4.0 * DELTA_3 * vfb).sqrt();
        t2fb = -DELTA_3 / t0fb;
    } else {
        t0fb = (v3 * v3 + 4.0 * DELTA_3 * vfb).sqrt();
        t2fb = DELTA_3 / t0fb;
    }
    let t1fb = 0.5 * (1.0 + v3 / t0fb);
    let vfbeff = vfb - 0.5 * (v3 + t0fb);
    let dvfbeff_dvd = (1.0 - t1fb - t2fb) * dvfb_dvd;
    let dvfbeff_dvb = (1.0 - t1fb - t2fb) * dvfb_dvb - t1fb;
    let dvfbeff_dvrg = t1fb * i.dvgs_eff_dvg;

    // Qac0 (b3soiddld.c:2706-2711).
    let qac0 = -cox_wl * (vfbeff - vfb);
    let dqac0_dvrg = -cox_wl * dvfbeff_dvrg;
    let dqac0_dvd = -cox_wl * (dvfbeff_dvd - dvfb_dvd);
    let dqac0_dvb = -cox_wl * (dvfbeff_dvb - dvfb_dvb);

    // Qsub0 (b3soiddld.c:2713-2735).
    let t0 = 0.5 * k1;
    let t3 = i.vgs_eff - vfbeff - i.vbseff - vgsteff;
    let (t1s, t2s);
    if k1 == 0.0 {
        t1s = 0.0;
        t2s = 0.0;
    } else if t3 < 0.0 {
        t1s = t0 + t3 / k1;
        t2s = cox_wl;
    } else {
        t1s = (t0 * t0 + t3).sqrt();
        t2s = cox_wl * t0 / t1s;
    }
    let qsub0 = cox_wl * k1 * (t0 - t1s);
    let dqsub0_dvrg = t2s * (dvfbeff_dvrg - i.dvgs_eff_dvg);
    let dqsub0_dvg = t2s;
    let dqsub0_dvd = t2s * dvfbeff_dvd;
    let dqsub0_dvb = t2s * (dvfbeff_dvb + 1.0);

    let abulk_cv = i.abulk0 * p.abulk_cv_factor;
    let dabulk_cv_dvb = p.abulk_cv_factor * i.dabulk0_dvb;

    // VdsatCV redefined for capMod==3 (b3soiddld.c:2893-2904).
    let t1 = vgsteff + k1 * sqrt_phis + 0.5 * k1 * k1;
    let t2 = vgsteff + k1 * sqrt_phis + phis + 0.25 * k1 * k1;
    let dt1_dvb = k1 * dsqrt_phis_dvb;
    let dt2_dvb = dt1_dvb + d_phis_dvb;
    let dt1_dvg = 1.0;
    let dt2_dvg = 1.0;
    let vdsat_cv = t1 - k1 * t2.sqrt();
    let dvdsat_cv_dvb = dt1_dvb - k1 / 2.0 / t2.sqrt() * dt2_dvb;
    let dvdsat_cv_dvg = dt1_dvg - k1 / 2.0 / t2.sqrt() * dt2_dvg;

    // VdsCV from Vdsat / Vdseff (b3soiddld.c:2906-2978).
    let t1 = vdsat_cv - i.vdsat;
    let dt1_dvg = dvdsat_cv_dvg - i.dvdsat_dvg;
    let dt1_dvb = dvdsat_cv_dvb - i.dvdsat_dvb;
    let dt1_dvd = -i.dvdsat_dvd;
    let dt1_dvc = -i.dvdsat_dvc;

    let (mut vds_cv, dvds_cv_dvg, dvds_cv_dvd, dvds_cv_dvb, dvds_cv_dvc);
    if t1 != 0.0 {
        let t3 = -0.5 * i.vdsat / t1; // Vdsmax
        let t2 = t3 * i.vdsat;
        let t4 = t2 + t1 * t3 * t3; // fmax
        if i.vdseff > t2 && t1 < 0.0 {
            vds_cv = t4;
            let t5 = -0.5 / (t1 * t1);
            let dt3_dvg = t5 * (t1 * i.dvdsat_dvg - i.vdsat * dt1_dvg);
            let dt3_dvb = t5 * (t1 * i.dvdsat_dvb - i.vdsat * dt1_dvb);
            let dt3_dvd = t5 * (t1 * i.dvdsat_dvd - i.vdsat * dt1_dvd);
            let dt3_dvc = t5 * (t1 * i.dvdsat_dvc - i.vdsat * dt1_dvc);
            dvds_cv_dvd =
                t3 * i.dvdsat_dvd + i.vdsat * dt3_dvd + t3 * (2.0 * t1 * dt3_dvd + t3 * dt1_dvd);
            dvds_cv_dvg =
                t3 * i.dvdsat_dvg + i.vdsat * dt3_dvg + t3 * (2.0 * t1 * dt3_dvg + t3 * dt1_dvg);
            dvds_cv_dvb =
                t3 * i.dvdsat_dvb + i.vdsat * dt3_dvb + t3 * (2.0 * t1 * dt3_dvb + t3 * dt1_dvb);
            dvds_cv_dvc =
                t3 * i.dvdsat_dvc + i.vdsat * dt3_dvc + t3 * (2.0 * t1 * dt3_dvc + t3 * dt1_dvc);
        } else {
            let t5 = i.vdseff / i.vdsat;
            let t6 = t5 * t5;
            let t8 = 2.0 * t1 * t5 / i.vdsat / i.vdsat;
            vds_cv = i.vdseff + t1 * t6;
            dvds_cv_dvd = i.dvdseff_dvd
                + t8 * (i.vdsat * i.dvdseff_dvd - i.vdseff * i.dvdsat_dvd)
                + t6 * dt1_dvd;
            dvds_cv_dvb = i.dvdseff_dvb
                + t8 * (i.vdsat * i.dvdseff_dvb - i.vdseff * i.dvdsat_dvb)
                + t6 * dt1_dvb;
            dvds_cv_dvg = i.dvdseff_dvg
                + t8 * (i.vdsat * i.dvdseff_dvg - i.vdseff * i.dvdsat_dvg)
                + t6 * dt1_dvg;
            dvds_cv_dvc = i.dvdseff_dvc
                + t8 * (i.vdsat * i.dvdseff_dvc - i.vdseff * i.dvdsat_dvc)
                + t6 * dt1_dvc;
        }
    } else {
        vds_cv = i.vdseff;
        dvds_cv_dvb = i.dvdseff_dvb;
        dvds_cv_dvd = i.dvdseff_dvd;
        dvds_cv_dvg = i.dvdseff_dvg;
        dvds_cv_dvc = i.dvdseff_dvc;
    }
    if vds_cv < 0.0 {
        vds_cv = 0.0;
    }
    vds_cv += 1e-4;
    if vds_cv > vdsat_cv - 1e-7 {
        vds_cv = vdsat_cv - 1e-7;
    }
    let phisd = phis + vds_cv;
    let dphisd_dvb = d_phis_dvb + dvds_cv_dvb;
    let dphisd_dvd = dvds_cv_dvd;
    let dphisd_dvg = dvds_cv_dvg;
    let dphisd_dvc = dvds_cv_dvc;
    let sqrt_phisd = phisd.sqrt();

    // Qdep0 (b3soiddld.c:2992-2995).
    let t10 = cox_wl * k1;
    let qdep0 = t10 * sqrt_phis;
    let dqdep0_dvb = t10 * dsqrt_phis_dvb;

    // VcsCV (b3soiddld.c:2997-3028).
    let t1 = vds_cv - i.vcs - vds_cv * vds_cv * DELTA_VCSCV;
    let t5 = 2.0 * DELTA_VCSCV;
    let t2 = (t1 * t1 + t5 * vds_cv * vds_cv).sqrt();
    let dt1_dvb = dvds_cv_dvb * (1.0 - 2.0 * vds_cv * DELTA_VCSCV);
    let dt2_dvb = (t1 * dt1_dvb + t5 * vds_cv * dvds_cv_dvb) / t2;
    let dt1_dvd = dvds_cv_dvd * (1.0 - 2.0 * vds_cv * DELTA_VCSCV);
    let dt2_dvd = (t1 * dt1_dvd + t5 * vds_cv * dvds_cv_dvd) / t2;
    let dt1_dvg = dvds_cv_dvg * (1.0 - 2.0 * vds_cv * DELTA_VCSCV);
    let dt2_dvg = (t1 * dt1_dvg + t5 * vds_cv * dvds_cv_dvg) / t2;
    let dt1_dvc = dvds_cv_dvc * (1.0 - 2.0 * vds_cv * DELTA_VCSCV) - 1.0;
    let dt2_dvc = (t1 * dt1_dvc + t5 * vds_cv * dvds_cv_dvc) / t2;
    let vcs_cv = i.vcs + 0.5 * (t1 - t2);
    let dvcs_cv_dvb = 0.5 * (dt1_dvb - dt2_dvb);
    let dvcs_cv_dvg = 0.5 * (dt1_dvg - dt2_dvg);
    let dvcs_cv_dvd = 0.5 * (dt1_dvd - dt2_dvd);
    let dvcs_cv_dvc = 1.0 + 0.5 * (dt1_dvc - dt2_dvc);

    let phisc = phis + vcs_cv;
    let dphisc_dvb = d_phis_dvb + dvcs_cv_dvb;
    let dphisc_dvd = dvcs_cv_dvd;
    let dphisc_dvg = dvcs_cv_dvg;
    let dphisc_dvc = dvcs_cv_dvc;
    let sqrt_phisc = phisc.sqrt();

    // Xc (b3soiddld.c:3038-3099).
    let t1 = vgsteff + k1 * sqrt_phis - 0.5 * vds_cv;
    let t2 = CONST_2OV3 * k1 * (phisd * sqrt_phisd - phis * sqrt_phis);
    let t3 = vgsteff + k1 * sqrt_phis - 0.5 * vcs_cv;
    let t4 = CONST_2OV3 * k1 * (phisc * sqrt_phisc - phis * sqrt_phis);
    let t5 = t1 * vds_cv - t2;
    let t6 = t3 * vcs_cv - t4;
    let xc = t6 / t5;

    let dt1_dvb = k1 * dsqrt_phis_dvb - 0.5 * dvds_cv_dvb;
    let dt2_dvb = k1 * (sqrt_phisd * dphisd_dvb - sqrt_phis * d_phis_dvb);
    let dt3_dvb = k1 * dsqrt_phis_dvb - 0.5 * dvcs_cv_dvb;
    let dt4_dvb = k1 * (sqrt_phisc * dphisc_dvb - sqrt_phis * d_phis_dvb);

    let dt1_dvd = -0.5 * dvds_cv_dvd;
    let dt2_dvd = k1 * sqrt_phisd * dphisd_dvd;
    let dt3_dvd = -0.5 * dvcs_cv_dvd;
    let dt4_dvd = k1 * sqrt_phisc * dphisc_dvd;

    let dt1_dvg = 1.0 - 0.5 * dvds_cv_dvg;
    let dt2_dvg = k1 * sqrt_phisd * dphisd_dvg;
    let dt3_dvg = 1.0 - 0.5 * dvcs_cv_dvg;
    let dt4_dvg = k1 * sqrt_phisc * dphisc_dvg;

    let dt1_dvc = -0.5 * dvds_cv_dvc;
    let dt2_dvc = k1 * sqrt_phisd * dphisd_dvc;
    let dt3_dvc = -0.5 * dvcs_cv_dvc;
    let dt4_dvc = k1 * sqrt_phisc * dphisc_dvc;

    let dt5_dvb = t1 * dvds_cv_dvb + vds_cv * dt1_dvb - dt2_dvb;
    let dt6_dvb = t3 * dvcs_cv_dvb + vcs_cv * dt3_dvb - dt4_dvb;
    let dt5_dvd = t1 * dvds_cv_dvd + vds_cv * dt1_dvd - dt2_dvd;
    let dt6_dvd = t3 * dvcs_cv_dvd + vcs_cv * dt3_dvd - dt4_dvd;
    let dt5_dvg = t1 * dvds_cv_dvg + vds_cv * dt1_dvg - dt2_dvg;
    let dt6_dvg = t3 * dvcs_cv_dvg + vcs_cv * dt3_dvg - dt4_dvg;
    let dt5_dvc = t1 * dvds_cv_dvc + vds_cv * dt1_dvc - dt2_dvc;
    let dt6_dvc = t3 * dvcs_cv_dvc + vcs_cv * dt3_dvc - dt4_dvc;

    let dxc_dvb = (dt6_dvb - t6 / t5 * dt5_dvb) / t5;
    let dxc_dvd = (dt6_dvd - t6 / t5 * dt5_dvd) / t5;
    let dxc_dvg = (dt6_dvg - t6 / t5 * dt5_dvg) / t5;
    let dxc_dvc = (dt6_dvc - t6 / t5 * dt5_dvc) / t5;

    // Nomi / Denomi -> Qsubs1 (b3soiddld.c:3101-3194).
    let t10x = phis * sqrt_phis;
    let t5 = phisc * sqrt_phisc;
    let t0n = t5 - t10x;
    let t1n = vgsteff + k1 * sqrt_phis + phis;
    let t2n = phisc * t5 - phis * t10x;
    let t3n = k1 * vcs_cv * (phis + 0.5 * vcs_cv);

    let dt0_dvb = 1.5 * (sqrt_phisc * dphisc_dvb - sqrt_phis * d_phis_dvb);
    let dt1_dvb = (0.5 * k1 / sqrt_phis + 1.0) * d_phis_dvb;
    let dt2_dvb = 2.5 * (t5 * dphisc_dvb - t10x * d_phis_dvb);
    let dt3_dvb =
        k1 * (vcs_cv * (d_phis_dvb + 0.5 * dvcs_cv_dvb) + dvcs_cv_dvb * (phis + 0.5 * vcs_cv));

    let dt0_dvd = 1.5 * sqrt_phisc * dphisc_dvd;
    let dt1_dvd = 0.0;
    let dt2_dvd = 2.5 * t5 * dphisc_dvd;
    let dt3_dvd = k1 * (phis + vcs_cv) * dvcs_cv_dvd;

    let dt0_dvg = 1.5 * sqrt_phisc * dphisc_dvg;
    let dt1_dvg = 1.0;
    let dt2_dvg = 2.5 * t5 * dphisc_dvg;
    let dt3_dvg = k1 * (vcs_cv * 0.5 * dvcs_cv_dvg + dvcs_cv_dvg * (phis + 0.5 * vcs_cv));

    let dt0_dvc = 1.5 * sqrt_phisc * dphisc_dvc;
    let dt1_dvc = 0.0;
    let dt2_dvc = 2.5 * t5 * dphisc_dvc;
    let dt3_dvc = k1 * (vcs_cv * 0.5 * dvcs_cv_dvc + dvcs_cv_dvc * (phis + 0.5 * vcs_cv));

    let nomi = k1 * (CONST_2OV3 * t1n * t0n - 0.4 * t2n - t3n);
    let dnomi_dvb = k1 * (CONST_2OV3 * (t1n * dt0_dvb + t0n * dt1_dvb) - 0.4 * dt2_dvb - dt3_dvb);
    let dnomi_dvd = k1 * (CONST_2OV3 * (t1n * dt0_dvd + t0n * dt1_dvd) - 0.4 * dt2_dvd - dt3_dvd);
    let dnomi_dvg = k1 * (CONST_2OV3 * (t1n * dt0_dvg + t0n * dt1_dvg) - 0.4 * dt2_dvg - dt3_dvg);
    let dnomi_dvc = k1 * (CONST_2OV3 * (t1n * dt0_dvc + t0n * dt1_dvc) - 0.4 * dt2_dvc - dt3_dvc);

    let t4 = vgsteff + k1 * sqrt_phis - 0.5 * vds_cv;
    let t5 = CONST_2OV3 * k1 * (phisd * sqrt_phisd - t10x);
    let dt4_dvb = k1 * dsqrt_phis_dvb - 0.5 * dvds_cv_dvb;
    let dt5_dvb = k1 * (sqrt_phisd * dphisd_dvb - sqrt_phis * d_phis_dvb);
    let dt4_dvd = -0.5 * dvds_cv_dvd;
    let dt5_dvd = k1 * sqrt_phisd * dphisd_dvd;
    let dt4_dvg = 1.0 - 0.5 * dvds_cv_dvg;
    let dt5_dvg = k1 * sqrt_phisd * dphisd_dvg;
    let dt4_dvc = -0.5 * dvds_cv_dvc;
    let dt5_dvc = k1 * sqrt_phisd * dphisd_dvc;

    let denomi = t4 * vds_cv - t5;
    let ddenomi_dvb = vds_cv * dt4_dvb + t4 * dvds_cv_dvb - dt5_dvb;
    let ddenomi_dvd = vds_cv * dt4_dvd + t4 * dvds_cv_dvd - dt5_dvd;
    let ddenomi_dvg = vds_cv * dt4_dvg + t4 * dvds_cv_dvg - dt5_dvg;
    let ddenomi_dvc = vds_cv * dt4_dvc + t4 * dvds_cv_dvc - dt5_dvc;

    let t6 = -cox_wl / denomi;
    let qsubs1 = t6 * nomi;
    let dqsubs1_dvb = t6 * (dnomi_dvb - nomi / denomi * ddenomi_dvb);
    let dqsubs1_dvg = t6 * (dnomi_dvg - nomi / denomi * ddenomi_dvg);
    let dqsubs1_dvd = t6 * (dnomi_dvd - nomi / denomi * ddenomi_dvd);
    let dqsubs1_dvc = t6 * (dnomi_dvc - nomi / denomi * ddenomi_dvc);

    // Qsubs2 (b3soiddld.c:3196-3210).
    let t6 = (1e-4 + phi - i.vbs0eff).sqrt();
    let t7 = k1 * cox_wl;
    let t8 = 1.0 - xc;
    let t10 = t7 * t6;
    let t11 = t7 * t8 * 0.5 / t6;
    let qsubs2 = -t10 * t8;
    let dqsubs2_dvg = t10 * dxc_dvg;
    let dqsubs2_dvb = t10 * dxc_dvb;
    let dqsubs2_dvd = t10 * dxc_dvd + t11 * i.dvbs0eff_dvd;
    let dqsubs2_dvc = t10 * dxc_dvc;
    let dqsubs2_dve = t11 * i.dvbs0eff_dve;
    let dqsubs2_dvrg = t11 * i.dvbs0eff_dvg;

    // Qbf (b3soiddld.c:3212-3223).
    let qbf = qac0 + qsub0 + qsubs1 + qsubs2 + qdep0;
    let dqbf_dvrg = dqac0_dvrg + dqsub0_dvrg + dqsubs2_dvrg;
    let dqbf_dvg = dqsub0_dvg + dqsubs1_dvg + dqsubs2_dvg;
    let dqbf_dvd = dqac0_dvd + dqsub0_dvd + dqsubs1_dvd + dqsubs2_dvd;
    let dqbf_dvb = dqac0_dvb + dqsub0_dvb + dqsubs1_dvb + dqsubs2_dvb + dqdep0_dvb;
    let dqbf_dvc = dqsubs1_dvc + dqsubs2_dvc;
    let dqbf_dve = dqsubs2_dve;

    // Common capMod 2/3: backgate charge (b3soiddld.c:3228-3284).
    let cbox_wl = p.kb3 * cbox * p.weff_cv * p.leff_cv;
    let t0 = 0.5 * k1;
    let t2 = (phi - i.vbs0t).sqrt();
    let t3 = phi + k1 * t2 - i.vbs0t;
    let t4 = (t0 * t0 + t3).sqrt();
    let qsicv = k1 * cox_wl * (t0 - t4);

    let t2 = (phi - i.vbs0mos).sqrt();
    let t3 = phi + k1 * t2 - i.vbs0mos;
    let t4 = (t0 * t0 + t3).sqrt();
    let qbf0 = k1 * cox_wl * (t0 - t4);
    let t6 = cox_wl * t0 / t4 * (1.0 + t0 / t2);
    let dqbf0_dve = t6 * i.dvbs0mos_dve;

    let t5 = -cbox_wl * (i.vbsdio - i.vbs0);
    let t6 = cbox_wl * xc;
    let qe1 = -qsicv + qbf0 + t5 * xc;
    let dqe1_dvg = t5 * (dxc_dvg * dvgsteff_dvg + dxc_dvb * i.dvbseff_dvg + dxc_dvc * i.dvcs_dvg)
        - t6 * i.dvbsdio_dvg;
    let dqe1_dvb = t5 * (dxc_dvg * dvgsteff_dvb + dxc_dvb * i.dvbseff_dvb + dxc_dvc * i.dvcs_dvb)
        - t6 * i.dvbsdio_dvb;
    let dqe1_dvd =
        t5 * (dxc_dvg * dvgsteff_dvd + dxc_dvb * i.dvbseff_dvd + dxc_dvc * i.dvcs_dvd + dxc_dvd)
            - t6 * i.dvbsdio_dvd;
    let dqe1_dve = dqbf0_dve + t6 * (i.dvbs0_dve - i.dvbsdio_dve);

    let t2 = -m.cboxt * p.weff_cv * p.leff_cv;
    let t3 = t2 * 0.5 * (1.0 - xc);
    let t4 = t2 * 0.5 * (vds_cv - vcs_cv);
    let qe2 = t2 * 0.5 * (1.0 - xc) * (vds_cv - vcs_cv);
    let t10g = t3 * (dvds_cv_dvg - dvcs_cv_dvg) - t4 * dxc_dvg;
    let t11g = t3 * (dvds_cv_dvb - dvcs_cv_dvb) - t4 * dxc_dvb;
    let t12g = t3 * (dvds_cv_dvc - dvcs_cv_dvc) - t4 * dxc_dvc;
    let dqe2_dvg = t10g * dvgsteff_dvg + t11g * i.dvbseff_dvg + t12g * i.dvcs_dvg;
    let dqe2_dvb = t10g * dvgsteff_dvb + t11g * i.dvbseff_dvb + t12g * i.dvcs_dvb;
    let dqe2_dvd = t10g * dvgsteff_dvd + t11g * i.dvbseff_dvd + t12g * i.dvcs_dvd
        + t3 * (dvds_cv_dvd - dvcs_cv_dvd)
        - t4 * dxc_dvd;
    let dqe2_dve = t10g * dvgsteff_dve + t11g * i.dvbseff_dve + t12g * i.dvcs_dve;

    // Transform Qbf dependency on (Vgsteff,Vbseff,Vcs) into node ones
    // (b3soiddld.c:3288-3311).
    let cbg = dqbf_dvrg + dqbf_dvg * dvgsteff_dvg + dqbf_dvb * i.dvbseff_dvg + dqbf_dvc * i.dvcs_dvg;
    let cbb = dqbf_dvg * dvgsteff_dvb + dqbf_dvb * i.dvbseff_dvb + dqbf_dvc * i.dvcs_dvb;
    let cbd = dqbf_dvg * dvgsteff_dvd + dqbf_dvb * i.dvbseff_dvd + dqbf_dvc * i.dvcs_dvd + dqbf_dvd;
    let cbe = dqbf_dvg * dvgsteff_dve + dqbf_dvb * i.dvbseff_dve + dqbf_dvc * i.dvcs_dve + dqbf_dve;

    let ce1g = dqe1_dvg;
    let ce1b = dqe1_dvb;
    let ce1d = dqe1_dvd;
    let ce1e = dqe1_dve;

    let ce2g = dqe2_dvg;
    let ce2b = dqe2_dvb;
    let ce2d = dqe2_dvd;
    let ce2e = dqe2_dve;

    // Total inversion charge (b3soiddld.c:3313-3326). VdseffCV == IV Vdseff here.
    let vdseff_cv = i.vdseff;
    let t0 = abulk_cv * vdseff_cv;
    let t1 = 12.0 * (vgsteff - 0.5 * t0 + 1e-20);
    let t2 = vdseff_cv / t1;
    let t3 = t0 * t2;
    let t4 = 1.0 - 12.0 * t2 * t2 * abulk_cv;
    let t5 = 6.0 * t0 * (4.0 * vgsteff - t0) / (t1 * t1) - 0.5;
    let t6 = 12.0 * t2 * t2 * vgsteff;
    let qinv = cox_wl * (vgsteff - 0.5 * vdseff_cv + t3);
    let cgg1 = cox_wl * (t4 + t5 * i.dvdseff_dvg);
    let cgd1 = cox_wl * t5 * i.dvdseff_dvd;
    let cgb1 = cox_wl * (t5 * i.dvdseff_dvb + t6 * dabulk_cv_dvb);

    // Charge partition into S (b3soiddld.c:3329-3368).
    let (qsrc, csg1, csd1, csb1);
    if m.xpart > 0.5 {
        let t1p = t1 + t1;
        qsrc = -cox_wl * (0.5 * vgsteff + 0.25 * t0 - t0 * t0 / t1p);
        let t7 = (4.0 * vgsteff - t0) / (t1p * t1p);
        let t4p = -(0.5 + 24.0 * t0 * t0 / (t1p * t1p));
        let t5p = -(0.25 * abulk_cv - 12.0 * abulk_cv * t0 * t7);
        let t6p = -(0.25 * vdseff_cv - 12.0 * t0 * vdseff_cv * t7);
        csg1 = cox_wl * (t4p + t5p * i.dvdseff_dvg);
        csd1 = cox_wl * t5p * i.dvdseff_dvd;
        csb1 = cox_wl * (t5p * i.dvdseff_dvb + t6p * dabulk_cv_dvb);
    } else if m.xpart < 0.5 {
        let t1p = t1 / 12.0;
        let t2p = 0.5 * cox_wl / (t1p * t1p);
        let t3p = vgsteff * (2.0 * t0 * t0 / 3.0 + vgsteff * (vgsteff - 4.0 * t0 / 3.0))
            - 2.0 * t0 * t0 * t0 / 15.0;
        qsrc = -t2p * t3p;
        let t7 = 4.0 / 3.0 * vgsteff * (vgsteff - t0) + 0.4 * t0 * t0;
        let t4p = -2.0 * qsrc / t1p
            - t2p * (vgsteff * (3.0 * vgsteff - 8.0 * t0 / 3.0) + 2.0 * t0 * t0 / 3.0);
        let t5p = (qsrc / t1p + t2p * t7) * abulk_cv;
        let t6p = qsrc / t1p * vdseff_cv + t2p * t7 * vdseff_cv;
        csg1 = t4p + t5p * i.dvdseff_dvg;
        csd1 = t5p * i.dvdseff_dvd;
        csb1 = t5p * i.dvdseff_dvb + t6p * dabulk_cv_dvb;
    } else {
        qsrc = -0.5 * qinv;
        csg1 = -0.5 * cgg1;
        csb1 = -0.5 * cgb1;
        csd1 = -0.5 * cgd1;
    }

    let csg = csg1 * dvgsteff_dvg + csb1 * i.dvbseff_dvg;
    let csd = csd1 + csg1 * dvgsteff_dvd + csb1 * i.dvbseff_dvd;
    let csb = csg1 * dvgsteff_dvb + csb1 * i.dvbseff_dvb;
    let cse = csg1 * dvgsteff_dve + csb1 * i.dvbseff_dve;

    // Qex (b3soiddld.c:3378-3385).
    let t0 = QEX_FACT * k1 * cox_wl;
    let qex = t0 * (i.vbs - i.vbsdio);
    let dqex_dvg = -t0 * i.dvbsdio_dvg;
    let dqex_dvb = t0 * (1.0 - i.dvbsdio_dvb);
    let dqex_dvd = -t0 * i.dvbsdio_dvd;
    let dqex_dve = -t0 * i.dvbsdio_dve;

    // Node charges (intrinsic, b3soiddld.c:3387-3390).
    let mut qgate = qinv - (qbf + qe2);
    let mut qbody = qbf - qe1 + qex;
    let mut qsub = qe1 + qe2 - qex;
    let mut qdrn = -(qinv + qsrc);

    // Cgg/Cgd/Cgb/Cge (b3soiddld.c:3392-3398).
    let cgg = (cgg1 * dvgsteff_dvg + cgb1 * i.dvbseff_dvg) - cbg;
    let cgd = (cgd1 + cgg1 * dvgsteff_dvd + cgb1 * i.dvbseff_dvd) - cbd;
    let cgb = (cgb1 * i.dvbseff_dvb + cgg1 * dvgsteff_dvb) - cbb;
    let cge = (cgg1 * dvgsteff_dve + cgb1 * i.dvbseff_dve) - cbe;

    // Intrinsic capacitance matrix (b3soiddld.c:3400-3429).
    let cggb = cgg - ce2g;
    let cgsb = -(cgg + cgd + cgb + cge) + (ce2g + ce2d + ce2b + ce2e);
    let cgdb = cgd - ce2d;
    let cgeb = cge - ce2e;

    let cbgb = cbg - ce1g + dqex_dvg;
    let mut cbsb = -(cbg + cbd + cbb + cbe) + (ce1g + ce1d + ce1b + ce1e)
        - (dqex_dvg + dqex_dvd + dqex_dvb + dqex_dve);
    let mut cbdb = cbd - ce1d + dqex_dvd;
    let cbeb = cbe - ce1e + dqex_dve;

    let cegb = ce1g + ce2g - dqex_dvg;
    let cesb = -(ce1g + ce1d + ce1b + ce1e) - (ce2g + ce2d + ce2b + ce2e)
        + (dqex_dvg + dqex_dvd + dqex_dvb + dqex_dve);
    let cedb = ce1d + ce2d - dqex_dvd;
    let ceeb = ce1e + ce2e - dqex_dve;

    let cdgb = -(cgg + cbg + csg);
    let mut cddb = -(cgd + cbd + csd);
    let cdeb = -(cge + cbe + cse);
    let mut cdsb = cgg + cgd + cgb + cge + cbg + cbd + cbb + cbe + csg + csd + csb + cse;

    // --- Intrinsic S/D junction charge (b3soiddld.c:3438-3494) ---
    let phi_bswg = m.phibswg;
    let mjswg = m.mjswg;
    let cjsbs = m.cjswg * p.weff * m.tsi / 1e-7;

    let (t3, dt3_dvb_s);
    if i.vbs < 0.0 {
        let arg = 1.0 - i.vbs / phi_bswg;
        let d = if mjswg == 0.5 {
            1.0 / arg.sqrt()
        } else {
            (-mjswg * arg.ln()).exp()
        };
        t3 = (1.0 - arg * d) * phi_bswg / (1.0 - mjswg);
        dt3_dvb_s = d;
    } else {
        t3 = i.vbs * (1.0 + 0.5 * mjswg * i.vbs / phi_bswg);
        dt3_dvb_s = 1.0 + mjswg * i.vbs / phi_bswg;
    }
    let qjs = cjsbs * t3 + m.tt * i.ibs1;
    let gcjsbs = cjsbs * dt3_dvb_s + m.tt * i.dibs1_dvb;

    let (t3, dt3_dvb_d);
    if i.vbd < 0.0 {
        let arg = 1.0 - i.vbd / phi_bswg;
        let d = if mjswg == 0.5 {
            1.0 / arg.sqrt()
        } else {
            (-mjswg * arg.ln()).exp()
        };
        t3 = (1.0 - arg * d) * phi_bswg / (1.0 - mjswg);
        dt3_dvb_d = d;
    } else {
        t3 = i.vbd * (1.0 + 0.5 * mjswg * i.vbd / phi_bswg);
        dt3_dvb_d = 1.0 + mjswg * i.vbd / phi_bswg;
    }
    let dt3_dvd_d = -dt3_dvb_d;
    let qjd = cjsbs * t3 + m.tt * i.ibd1;
    let gcjdbs = cjsbs * dt3_dvb_d + m.tt * i.dibd1_dvb;
    let gcjdds = cjsbs * dt3_dvd_d + m.tt * i.dibd1_dvd;

    qdrn -= qjd;
    qbody += qjs + qjd;

    cddb -= gcjdds;
    cdsb += gcjdds + gcjdbs;
    cbdb += gcjdds;
    cbsb -= gcjdds + gcjdbs + gcjsbs;

    // --- Extrinsic bottom S/D-to-substrate charge (b3soiddld.c:3496-3609) ---
    let nsub_pos_type = (p.nsub > 0.0 && m.mtype > 0.0) || (p.nsub < 0.0 && m.mtype < 0.0);
    let t10 = -m.mtype * i.ves_raw; // vse without type
    let (mut qse, gcse) = extrinsic_sd_charge(p, t10, nsub_pos_type, true);
    let t11 = m.mtype * (i.vds_raw - i.ves_raw); // vde without type
    let (mut qde, gcde) = extrinsic_sd_charge(p, t11, nsub_pos_type, false);

    // Sidewall fringing (b3soiddld.c:3600-3604).
    qse += p.csesw * t10;
    let gcse = gcse + p.csesw;
    qde += p.cdesw * t11;
    let gcde = gcde + p.cdesw;

    // qse/qde carry true polarity -> premultiply by type (b3soiddld.c:3606-3609).
    let qse = m.mtype * qse;
    let qde = m.mtype * qde;

    // --- Overlap (Meyer fringing) charges (b3soiddld.c:3655-3784) ---
    // Drain overlap.
    let t0 = i.vgd_raw + DELTA_1;
    let t1 = (t0 * t0 + 4.0 * DELTA_1).sqrt();
    let t2 = 0.5 * (t0 - t1);
    let t3v = p.weff_cv * p.cgdl;
    let t4v = (1.0 - 4.0 * t2 / p.ckappa).sqrt();
    let cgdo = p.cgdo + t3v - t3v * (1.0 - 1.0 / t4v) * (0.5 - 0.5 * t0 / t1);
    let qgdo = (p.cgdo + t3v) * i.vgd_raw - t3v * (t2 + 0.5 * p.ckappa * (t4v - 1.0));

    // Source overlap.
    let t0 = i.vgs_raw + DELTA_1;
    let t1 = (t0 * t0 + 4.0 * DELTA_1).sqrt();
    let t2 = 0.5 * (t0 - t1);
    let t3v = p.weff_cv * p.cgsl;
    let t4v = (1.0 - 4.0 * t2 / p.ckappa).sqrt();
    let cgso = p.cgso + t3v - t3v * (1.0 - 1.0 / t4v) * (0.5 - 0.5 * t0 / t1);
    let qgso = (p.cgso + t3v) * i.vgs_raw - t3v * (t2 + 0.5 * p.ckappa * (t4v - 1.0));

    let cgeo = p.cgeo;
    let qge = cgeo * i.vge_raw;
    let qgd = qgdo;
    let qgs = qgso;

    // Assemble the capacitance matrix (overlap+extrinsic), mode-aware
    // (b3soiddld.c:3679-3781). `ag0` is applied later by the device.
    if mode > 0 {
        // Charge lumping (b3soiddld.c:3722-3729).
        qgate += qgd + qgs + qge;
        qdrn += qde - qgd;
        qsub -= qge + qse + qde;

        B3SoiDdCharge {
            mode,
            qg: qgate,
            qb: qbody,
            qd: qdrn,
            qe: qsub,
            gcdgb: cdgb - cgdo,
            gcddb: cddb + cgdo + gcde,
            gcdsb: cdsb,
            gcdeb: cdeb - gcde,
            gcsgb: -(cggb + cbgb + cdgb + cegb + cgso),
            gcsdb: -(cgdb + cbdb + cddb + cedb),
            gcssb: cgso + gcse - (cgsb + cbsb + cdsb + cesb),
            gcseb: -(gcse + cgeb + cbeb + cdeb + ceeb),
            gcggb: cggb + cgdo + cgso + cgeo,
            gcgdb: cgdb - cgdo,
            gcgsb: cgsb - cgso,
            gcgeb: cgeb - cgeo,
            gcbgb: cbgb,
            gcbdb: cbdb,
            gcbsb: cbsb,
            gcbeb: cbeb,
            gcegb: cegb - cgeo,
            gcedb: cedb - gcde,
            gcesb: cesb - gcse,
            gceeb: gcse + gcde + ceeb + cgeo,
        }
    } else {
        // Inverse mode (b3soiddld.c:3732-3781): D/S roles swap in the matrix.
        qgate += qgd + qgs + qge;
        let qsrc = qdrn - qgs + qse;
        qsub -= qge + qse + qde;
        qdrn = -(qgate + qbody + qsrc + qsub);

        B3SoiDdCharge {
            mode,
            qg: qgate,
            qb: qbody,
            qd: qdrn,
            qe: qsub,
            gcsgb: cdgb - cgso,
            gcssb: cddb + cgso + gcse,
            gcsdb: cdsb,
            gcseb: cdeb - gcse,
            gcdgb: -(cggb + cbgb + cdgb + cegb + cgdo),
            gcdsb: -(cgdb + cbdb + cddb + cedb),
            gcddb: cgdo + gcde - (cgsb + cbsb + cdsb + cesb),
            gcdeb: -(gcde + cgeb + cbeb + cdeb + ceeb),
            gcggb: cggb + cgdo + cgso + cgeo,
            gcgsb: cgdb - cgso,
            gcgdb: cgsb - cgdo,
            gcgeb: cgeb - cgeo,
            gcbgb: cbgb,
            gcbsb: cbdb,
            gcbdb: cbsb,
            gcbeb: cbeb,
            gcegb: cegb - cgeo,
            gcesb: cedb - gcse,
            gcedb: cesb - gcde,
            gceeb: ceeb + cgeo + gcse + gcde,
        }
    }
}

/// Extrinsic bottom S/D-to-substrate depletion charge spline
/// (b3soiddld.c:3499-3598). `v` is the substrate-to-S (or substrate-to-D)
/// voltage with no type conversion; returns `(charge, conductance)`.
#[inline]
fn extrinsic_sd_charge(
    p: &B3SoiDdSized,
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
