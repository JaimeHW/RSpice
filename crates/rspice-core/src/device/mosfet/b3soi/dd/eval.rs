//! B3SOIDD DC load equations (faithful port of ngspice-46 `b3soiddld.c`).
//!
//! This module transcribes the **DC current path** of `B3SOIDDload`
//! (b3soiddld.c lines ~860-2640): the SOI body-coupled threshold chain, the
//! BSIM3 `Vgsteff` smoothing, `Abulk`, MOBMOD mobility, `Vdsat`, CLM /
//! DIBL / `Va`, the channel current `Ids`, and the SOI body currents (impact
//! ionization, GIDL, and the source/drain body diodes + parasitic BJT). The
//! result is the linearized operating point that ngspice stores in the
//! `here->B3SOIDD*` conductance/current fields and then stamps.
//!
//! Scope / provenance:
//! - Tested decks use MOBMOD=0, CAPMOD=2/3 and either a floating body
//!   (`bodyMod=0`) or an ideal body tie (`bodyMod=2`). The temperature-
//!   dependent electrical quantities (jbjt, jdif, jrec, jtun, u0temp,
//!   vsattemp, rds0, ua/ub/uc, vbi, vfbb) are taken from the precomputed
//!   [`B3SoiDdSized`] (the `else` branch of the big temp block at
//!   b3soiddld.c:803-822). When the DD self-heating node is present, the device
//!   wrapper updates only Xyce's local self-heating subset at
//!   `CKTtemp + delTemp`; the size-dependent cache values such as `phi` and
//!   `Xdep0` stay at their ambient `paramPtr` values.
//!   * Body-resistor current `Ibp` (b3soiddld.c:2480-2540) is zero for
//!     `bodyMod` 0/2 and is therefore omitted here; the body tie is handled by
//!     the device stamping (the external body node is the body node directly).
//! - The **charge model** (CAPMOD=2/3, b3soiddld.c:2640-3400) and the matrix
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
// by the CAPMOD=2/3 charge model; ngspice keeps them here, so we
// retain the assignments for a faithful seam rather than dropping them.
#![allow(unused_assignments)]

use super::super::common::{
    B3SoiDialect, DELT_VBS0DIO, DELT_VBS0EFF, DELT_VBSDIO, DELT_VBSEFF, DELT_VBSMOS, OFF_VBSDIO,
};
use super::super::common::{EG300, EPSSI, EXP_THRESHOLD, KB_OVER_Q, MAX_EXP, MIN_EXP};
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

    // Self-heating electrical temperature derivatives.
    pub gm_t: Value,
    pub gjd_t: Value,
    pub gjs_t: Value,
    pub gb_t: Value,

    // Self-heating thermal row (`B3SOIDDgtemp*`, `B3SOIDDcth`).
    pub gtemp_g: Value,
    pub gtemp_b: Value,
    pub gtemp_e: Value,
    pub gtemp_d: Value,
    pub gtemp_t: Value,
    pub thermal_eq_current: Value,

    /// Bias-dependent gate-resistance conductance and derivatives
    /// (`B3SOIDDgcrg*`, used by `RGATEMOD=2`).
    pub gcrg: Value,
    pub gcrgg: Value,
    pub gcrgd: Value,
    pub gcrgs: Value,
    pub gcrgb: Value,

    /// Inversion charge proxy used by noise (`B3SOIDDqinv`).
    pub qinv: Value,

    /// CAPMOD=2/3 charge state (set only when [`eval`] is asked to compute it).
    pub charge: Option<B3SoiDdCharge>,
}

/// CAPMOD=2/3 charge-model output for one B3SOIDD instance.
///
/// Mirrors the `here->B3SOIDDq*` node charges and the `here->B3SOIDDc*` intrinsic
/// capacitance matrix that ngspice fills at the end of `B3SOIDDload`
/// (b3soiddld.c:3387-3429) plus the extrinsic S/D-to-substrate spline charges
/// (b3soiddld.c:3438-3609) and the gate overlap charges (b3soiddld.c:3655-3784).
///
/// The electrical node charges (`qg/qb/qd/qe`) include the overlap and extrinsic
/// lumps exactly as ngspice does in the `mode>0` branch at b3soiddld.c:3722-3729,
/// so `qg+qb+qd+qe+qs == 0`. `qth` is the optional self-heating temperature
/// charge (`Cth * delTemp`) filled by the device wrapper. Capacitances are the
/// `gc**b`-style derivatives *before* multiplication by the integration
/// coefficient `ag0` (the device applies `ag0` when it forms the transient
/// companion). All quantities are in device polarity with `mtype` already folded
/// in where ngspice folds it.
#[derive(Debug, Clone, Default)]
pub struct B3SoiDdCharge {
    /// Channel direction at evaluation (`here->B3SOIDDmode`).
    pub mode: i32,

    // Node charges (after overlap + extrinsic lumping, b3soiddld.c:3722-3729).
    pub qg: Value,
    pub qb: Value,
    pub qd: Value,
    pub qe: Value,
    pub qth: Value,

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

    // Self-heating charge derivatives with respect to the temperature-rise node.
    pub gcg_t: Value,
    pub gcb_t: Value,
    pub gcd_t: Value,
    pub gce_t: Value,
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
    pub del_temp: Value,
}

/// Evaluate the B3SOIDD DC operating point.
///
/// `p` is the size/temperature-resolved parameter set, `m_*` the few model-card
/// scalars needed in the load, and `bias` the device-polarity branch voltages.
/// `temp_k` is the (constant, no self-heating) device temperature in Kelvin and
/// `mtype` the polarity (+1 NMOS / -1 PMOS).
#[allow(clippy::too_many_lines)]
pub fn eval_dc(p: &B3SoiDdSized, m: &ModelConsts, bias: B3SoiDdBias, mtype: Value) -> B3SoiDdOp {
    eval_with_self_heat(p, m, bias, mtype, false, false)
}

/// Evaluate the B3SOIDD operating point, optionally including the CAPMOD=2/3
/// charge model (`compute_charges == true`, the `ChargeComputationNeeded` path).
///
/// The DC current path is identical to [`eval_dc`]; when `compute_charges` is set
/// the resulting [`B3SoiDdOp::charge`] carries the intrinsic + extrinsic charge
/// state (b3soiddld.c:2637-3784, capMod==2/3, selfheat==0).
#[allow(clippy::too_many_lines)]
pub fn eval(
    p: &B3SoiDdSized,
    m: &ModelConsts,
    bias: B3SoiDdBias,
    mtype: Value,
    compute_charges: bool,
) -> B3SoiDdOp {
    eval_with_self_heat(p, m, bias, mtype, compute_charges, false)
}

/// Evaluate a B3SOIDD operating point with Xyce self-heating charge derivatives.
pub fn eval_with_self_heat(
    p: &B3SoiDdSized,
    m: &ModelConsts,
    bias: B3SoiDdBias,
    mtype: Value,
    compute_charges: bool,
    selfheat: bool,
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
    let sh = self_heat_derivs(p, m, selfheat);

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
    let qsi = match m.dialect {
        B3SoiDialect::Ngspice => m.qsi,
        B3SoiDialect::Xyce => m.qsi * (1.0 + p.nlx / leff),
    };
    let v0 = vbi - phi;

    // --- Vbs0t (b3soiddld.c:923-933) ---
    let (vbs0t, dvbs0t_dt);
    {
        let t0 = -p.dvbd1 * p.leff / p.litl;
        let t1 = p.dvbd0 * ((0.5 * t0).exp() + 2.0 * t0.exp());
        let t2 = t1 * (vbi - phi);
        let t3 = 0.5 * qsi / m.csi;
        vbs0t = phi - t3 + p.vbsa + t2;
        dvbs0t_dt = t1 * sh.dvbi_dt;
    }

    // --- Vbs0 / Vbs0mos (ngspice b3soiddld.c:935-985; Xyce B3SOI.C:9448-9485) ---
    let (vbs0, dvbs0_dve, dvbs0_dt);
    let (vbs0mos, dvbs0mos_dve, dvbs0mos_dt);
    match m.dialect {
        B3SoiDialect::Ngspice => {
            let t0 = 1.0 + m.csieff / cbox;
            let t1 = p.kb1 / t0;
            let t2 = t1 * (vbs0t - vesfb);
            let t6 = vbs0t - t2;
            let dt6_dve = t1;
            let dt6_dt = (1.0 - t1) * dvbs0t_dt - t1 * sh.dvfbb_dt;

            // limit Vbs0 below phi
            let l1 = phi - p.delp;
            let t2b = l1 - t6 - DELT_VBSEFF;
            let t3 = (t2b * t2b + 4.0 * DELT_VBSEFF).sqrt();
            vbs0 = l1 - 0.5 * (t2b + t3);
            let t4 = 0.5 * (1.0 + t2b / t3);
            dvbs0_dve = t4 * dt6_dve;
            dvbs0_dt = t4 * dt6_dt;

            let t1b = vbs0t - vbs0 - DELT_VBSMOS;
            let t2c = (t1b * t1b + DELT_VBSMOS * DELT_VBSMOS).sqrt();
            let t3b = 0.5 * (t1b + t2c);
            let t4b = t3b * m.csieff / m.qsieff;
            vbs0mos = vbs0 - 0.5 * t3b * t4b;
            let t5 = 0.5 * t4b * (1.0 + t1b / t2c);
            dvbs0mos_dve = dvbs0_dve * (1.0 + t5);
            let dt3b_dt = 0.5 * (1.0 + t1b / t2c) * (dvbs0t_dt - dvbs0_dt);
            dvbs0mos_dt = dvbs0_dt - t4b * dt3b_dt;
        }
        B3SoiDialect::Xyce => {
            let t0 = 1.0 + m.csi / cbox;
            let t3 = -m.dk2b * p.leff / p.litl;
            let t5 = m.k2b * ((0.5 * t3).exp() + 2.0 * t3.exp());
            let t1 = (m.k1b - t5) / t0;
            let t2 = t1 * vesfb;
            let t4 = 1.0 / (1.0 + cbox / m.csi);
            vbs0 = t4 * vbs0t + t2;
            dvbs0_dve = t1;
            dvbs0_dt = if selfheat {
                t4 * dvbs0t_dt - t1 * sh.dvfbb_dt
            } else {
                0.0
            };

            let t1b = vbs0t - vbs0 - DELT_VBSMOS;
            let t2b = (t1b * t1b + DELT_VBSMOS * DELT_VBSMOS).sqrt();
            let t3b = 0.5 * (t1b + t2b);
            let t4b = t3b * m.csi / qsi;
            let mut xyce_vbs0mos = vbs0 - 0.5 * t3b * t4b;
            let t5b = 0.5 * t4b * (1.0 + t1b / t2b);
            let mut xyce_dvbs0mos_dve = dvbs0_dve * (1.0 + t5b);
            let mut xyce_dvbs0mos_dt = if selfheat {
                dvbs0_dt * (1.0 + t5b) - t5b * dvbs0t_dt
            } else {
                0.0
            };

            let t1c = phi - 0.02;
            let t2c = t1c - xyce_vbs0mos - DELT_VBSMOS;
            let t3c = (t2c * t2c + 4.0 * DELT_VBSMOS).sqrt();
            xyce_vbs0mos = t1c - 0.5 * (t2c + t3c);
            let t4c = 0.5 * (1.0 + t2c / t3c);
            xyce_dvbs0mos_dve *= t4c;
            if selfheat {
                xyce_dvbs0mos_dt *= t4c;
            } else {
                xyce_dvbs0mos_dt = 0.0;
            }

            vbs0mos = xyce_vbs0mos;
            dvbs0mos_dve = xyce_dvbs0mos_dve;
            dvbs0mos_dt = xyce_dvbs0mos_dt;
        }
    }

    // --- Vthfd (treat Vbs0mos as Vb), b3soiddld.c:990-1083 ---
    let factor1 = p.factor1;
    let (vthfd, dvthfd_dt);
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
        let ddelt_vth_dt = thetavth * sh.dvbi_dt;

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
        let delt_vthw_coeff = p.dvt0w * t2dw;
        let delt_vthw = delt_vthw_coeff * v0;
        let ddelt_vthw_dvb = p.dvt0w * dt2dw_dvb * v0;
        let ddelt_vthw_dt = delt_vthw_coeff * sh.dvbi_dt;

        // DeltVthtemp
        let t0t = (1.0 + p.nlx / leff).sqrt();
        let t1t = p.kt1 + p.kt1l / leff + p.kt2 * vbs0mos;
        let delt_vthtemp = p.k1 * (t0t - 1.0) * sqrt_phi + t1t * temp_ratio;
        let ddelt_vthtemp_dt = if selfheat { t1t / p.tnom } else { 0.0 };

        let tmp2 = m.tox * phi / (p.weff + p.w0);

        let (t3d, dt3_dvb) = smooth_etab(p.eta0, p.etab, vbs0mos);
        let dibl_sft = t3d * p.theta0vb0 * vds;
        let ddibl_sft_dvd = t3d * p.theta0vb0;
        let ddibl_sft_dvb = p.theta0vb0 * vds * dt3_dvb;

        vthfd =
            mtype * p.vth0 + p.k1 * (sqrt_phis - sqrt_phi) - p.k2 * vbs0mos - delt_vth - delt_vthw
                + (p.k3 + p.k3b * vbs0mos) * tmp2
                + delt_vthtemp
                - dibl_sft;

        let t6 = p.k3b * tmp2 - p.k2 + p.kt2 * temp_ratio;
        dvthfd_dvd = -ddibl_sft_dvd;
        let t7 = p.k1 * dsqrt_phis_dvb - ddelt_vth_dvb - ddelt_vthw_dvb + t6 - ddibl_sft_dvb;
        dvthfd_dve = t7 * dvbs0mos_dve;
        dvthfd_dt = if selfheat {
            ddelt_vthtemp_dt - ddelt_vth_dt - ddelt_vthw_dt + t7 * dvbs0mos_dt
        } else {
            0.0
        };
    }

    let (
        charge_vbs0t,
        charge_vbs0,
        charge_dvbs0_dve,
        charge_vbs0mos,
        charge_dvbs0mos_dve,
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
        vbseff,
        dvbseff_dvg,
        dvbseff_dvd,
        dvbseff_dvb,
        dvbseff_dve,
        dvbseff_dt,
        vbsh,
        dvbsh_dvb_eff,
    ) = match m.dialect {
        B3SoiDialect::Ngspice => {
            // --- Vbs0teff / nfb / Vbs0eff (b3soiddld.c:1085-1145) ---
            let (vbs0teff, dvbs0teff_dvg, dvbs0teff_dvd, dvbs0teff_dve, dvbs0teff_dt);
            let (vbs0eff, dvbs0eff_dvg, dvbs0eff_dvd, dvbs0eff_dve, dvbs0eff_dt);
            {
                let t1 = vthfd - vgs_eff - DELT_VBS0EFF;
                let t2 = (t1 * t1 + DELT_VBS0EFF * DELT_VBS0EFF).sqrt();
                vbs0teff = vbs0t - 0.5 * (t1 + t2);
                let half = 0.5 * (1.0 + t1 / t2);
                dvbs0teff_dvg = half * dvgs_eff_dvg;
                dvbs0teff_dvd = -half * dvthfd_dvd;
                dvbs0teff_dve = -half * dvthfd_dve;
                dvbs0teff_dt = dvbs0t_dt - half * dvthfd_dt;

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
                dvbs0eff_dt =
                    dvbs0_dt - nfb * half * dvthfd_dt - t7 * 0.5 * (t1 + t2) * dvbs0mos_dt;
            }

            // --- Vbsdio (b3soiddld.c:1147-1162) ---
            let (vbsdio, dvbsdio_dvg, dvbsdio_dvd, dvbsdio_dve, dvbsdio_dvb, dvbsdio_dt);
            {
                let t1 = vbs - (vbs0eff + OFF_VBSDIO) - DELT_VBSDIO;
                let t2 = (t1 * t1 + DELT_VBSDIO * DELT_VBSDIO).sqrt();
                let t3 = 0.5 * (1.0 + t1 / t2);
                vbsdio = vbs0eff + OFF_VBSDIO + 0.5 * (t1 + t2);
                dvbsdio_dvg = (1.0 - t3) * dvbs0eff_dvg;
                dvbsdio_dvd = (1.0 - t3) * dvbs0eff_dvd;
                dvbsdio_dve = (1.0 - t3) * dvbs0eff_dve;
                dvbsdio_dvb = t3;
                dvbsdio_dt = (1.0 - t3) * dvbs0eff_dt;
            }

            // --- Vbsmos (b3soiddld.c:1164-1183) ---
            let (vbsmos, dvbsmos_dvg, dvbsmos_dvd, dvbsmos_dvb, dvbsmos_dve, dvbsmos_dt);
            {
                let t1 = vbs0teff - vbsdio - DELT_VBSMOS;
                let t2 = (t1 * t1 + DELT_VBSMOS * DELT_VBSMOS).sqrt();
                let t3 = 0.5 * (t1 + t2);
                let t5 = 0.5 * (1.0 + t1 / t2);
                let dt3_dvg = t5 * (dvbs0teff_dvg - dvbsdio_dvg);
                let dt3_dvd = t5 * (dvbs0teff_dvd - dvbsdio_dvd);
                let dt3_dvb = -t5 * dvbsdio_dvb;
                let dt3_dve = t5 * (dvbs0teff_dve - dvbsdio_dve);
                let dt3_dt = t5 * (dvbs0teff_dt - dvbsdio_dt);
                let t4 = t3 * m.csieff / m.qsieff;
                vbsmos = vbsdio - 0.5 * t3 * t4;
                dvbsmos_dvg = dvbsdio_dvg - t4 * dt3_dvg;
                dvbsmos_dvd = dvbsdio_dvd - t4 * dt3_dvd;
                dvbsmos_dvb = dvbsdio_dvb - t4 * dt3_dvb;
                dvbsmos_dve = dvbsdio_dve - t4 * dt3_dve;
                dvbsmos_dt = dvbsdio_dt - t4 * dt3_dt;
            }

            let vcs = vbsdio - vbs0eff;
            let dvcs_dvb = dvbsdio_dvb;
            let dvcs_dvg = dvbsdio_dvg - dvbs0eff_dvg;
            let dvcs_dvd = dvbsdio_dvd - dvbs0eff_dvd;
            let dvcs_dve = dvbsdio_dve - dvbs0eff_dve;

            let (vbseff, dvbseff_dvg, dvbseff_dvd, dvbseff_dvb, dvbseff_dve, dvbseff_dt);
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
                dvbseff_dt = t4 * dvbsmos_dt;
            }

            (
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
                vbseff,
                dvbseff_dvg,
                dvbseff_dvd,
                dvbseff_dvb,
                dvbseff_dve,
                dvbseff_dt,
                vbseff,
                dvbseff_dvb,
            )
        }
        B3SoiDialect::Xyce => {
            let t10 = m.nofffd * vtm;
            let vtgs_fd = vthfd - vgs_eff;
            let (exp_vtgs_fd, mut t0) = cexp100((vtgs_fd - m.vofffd) / t10);
            let vtgseff_fd = t10 * (1.0 + exp_vtgs_fd).ln();
            t0 /= 1.0 + exp_vtgs_fd;
            let dvtgseff_fd_dvd = t0 * dvthfd_dvd;
            let dvtgseff_fd_dvg = -t0 * dvgs_eff_dvg;
            let dvtgseff_fd_dve = t0 * dvthfd_dve;
            let dvtgseff_fd_dt = if selfheat {
                t0 * (dvthfd_dt - (vtgs_fd - m.vofffd) / p.temp) + vtgseff_fd / p.temp
            } else {
                0.0
            };

            let vgst_fd = vgs_eff - vthfd;
            let (exp_vgst_fd, mut t0) = cexp100((vgst_fd - m.vofffd) / t10);
            let vgsteff_fd = t10 * (1.0 + exp_vgst_fd).ln();
            t0 /= 1.0 + exp_vgst_fd;
            let dvgsteff_fd_dvd = -t0 * dvthfd_dvd;
            let dvgsteff_fd_dvg = t0 * dvgs_eff_dvg;
            let dvgsteff_fd_dve = -t0 * dvthfd_dve;
            let dvgsteff_fd_dt = if selfheat {
                t0 * (-dvthfd_dt - (vgst_fd - m.vofffd) / p.temp) + vgsteff_fd / p.temp
            } else {
                0.0
            };

            let t1 = m.moin_fd * k1 * vtm * vtm;
            let dt1_dt = if selfheat { 2.0 * t1 / p.temp } else { 0.0 };
            let t2 = vgsteff_fd + 2.0 * k1 * sqrt_phi;
            let dt2_dvg = dvgsteff_fd_dvg;
            let dt2_dvd = dvgsteff_fd_dvd;
            let dt2_dve = dvgsteff_fd_dve;
            let dt2_dt = if selfheat { dvgsteff_fd_dt } else { 0.0 };
            let t0 = 1.0 + vgsteff_fd * t2 / t1;
            let dt0_dvg = (vgsteff_fd * dt2_dvg + t2 * dvgsteff_fd_dvg) / t1;
            let dt0_dvd = (vgsteff_fd * dt2_dvd + t2 * dvgsteff_fd_dvd) / t1;
            let dt0_dve = (vgsteff_fd * dt2_dve + t2 * dvgsteff_fd_dve) / t1;
            let dt0_dt = if selfheat {
                (vgsteff_fd * (dt2_dt - t2 / t1 * dt1_dt) + t2 * dvgsteff_fd_dt) / t1
            } else {
                0.0
            };

            let phi_on = phi + vtm * t0.ln();
            let dphi_on_dvg = vtm * dt0_dvg / t0;
            let dphi_on_dvd = vtm * dt0_dvd / t0;
            let dphi_on_dve = vtm * dt0_dve / t0;
            let dphi_on_dt = if selfheat {
                vtm * dt0_dt / t0 + (phi_on - phi) / p.temp
            } else {
                0.0
            };

            let t0 = m.cox / (m.cox + 1.0 / (1.0 / m.csi + 1.0 / cbox));
            let phi_fd = phi_on - t0 * vtgseff_fd;
            let dphi_fd_dvg = dphi_on_dvg - t0 * dvtgseff_fd_dvg;
            let dphi_fd_dvd = dphi_on_dvd - t0 * dvtgseff_fd_dvd;
            let dphi_fd_dve = dphi_on_dve - t0 * dvtgseff_fd_dve;
            let dphi_fd_dt = if selfheat {
                dphi_on_dt - t0 * dvtgseff_fd_dt
            } else {
                0.0
            };

            let t0 = -p.dvbd1 * p.leff / p.litl;
            let t1 = p.dvbd0 * ((0.5 * t0).exp() + 2.0 * t0.exp());
            let t2 = t1 * (vbi - phi);
            let t3 = 0.5 * qsi / m.csi;
            let xyce_vbs0t = phi_fd - t3 + p.vbsa + t2;
            let dxyce_vbs0t_dvg = dphi_fd_dvg;
            let dxyce_vbs0t_dvd = dphi_fd_dvd;
            let dxyce_vbs0t_dve = dphi_fd_dve;
            let dxyce_vbs0t_dt = if selfheat {
                dphi_fd_dt + t1 * sh.dvbi_dt
            } else {
                0.0
            };

            let t0 = 1.0 + m.csi / cbox;
            let t3 = -m.dk2b * p.leff / p.litl;
            let t5 = m.k2b * ((0.5 * t3).exp() + 2.0 * t3.exp());
            let t1 = (m.k1b - t5) / t0;
            let t2 = t1 * vesfb;
            let t0 = 1.0 / (1.0 + cbox / m.csi);
            let xyce_vbs0 = t0 * xyce_vbs0t + t2;
            let dxyce_vbs0_dvg = t0 * dxyce_vbs0t_dvg;
            let dxyce_vbs0_dvd = t0 * dxyce_vbs0t_dvd;
            let dxyce_vbs0_dve = t0 * dxyce_vbs0t_dve + t1;
            let dxyce_vbs0_dt = if selfheat {
                t0 * dxyce_vbs0t_dt - t1 * sh.dvfbb_dt
            } else {
                0.0
            };

            let t1 = vbs - (xyce_vbs0 + 0.02) - 0.01;
            let t2 = (t1 * t1 + 0.0001).sqrt();
            let t3 = 0.5 * (1.0 + t1 / t2);
            let vbsitf = xyce_vbs0 + 0.02 + 0.5 * (t1 + t2);
            let dvbsitf_dvg = (1.0 - t3) * dxyce_vbs0_dvg;
            let dvbsitf_dvd = (1.0 - t3) * dxyce_vbs0_dvd;
            let dvbsitf_dve = (1.0 - t3) * dxyce_vbs0_dve;
            let dvbsitf_dvb = t3;
            let dvbsitf_dt = if selfheat {
                (1.0 - t3) * dxyce_vbs0_dt
            } else {
                0.0
            };

            let t1 = xyce_vbs0t - vbsitf - DELT_VBSMOS;
            let t2 = (t1 * t1 + DELT_VBSMOS * DELT_VBSMOS).sqrt();
            let t3 = 0.5 * (t1 + t2);
            let t4 = t3 * m.csi / qsi;
            let vbsmos = vbsitf - 0.5 * t3 * t4;
            let t5 = 0.5 * t4 * (1.0 + t1 / t2);
            let dvbsmos_dvg = dvbsitf_dvg * (1.0 + t5) - t5 * dxyce_vbs0t_dvg;
            let dvbsmos_dvd = dvbsitf_dvd * (1.0 + t5) - t5 * dxyce_vbs0t_dvd;
            let dvbsmos_dvb = dvbsitf_dvb * (1.0 + t5);
            let dvbsmos_dve = dvbsitf_dve * (1.0 + t5) - t5 * dxyce_vbs0t_dve;
            let dvbsmos_dt = if selfheat {
                dvbsitf_dt * (1.0 + t5) - t5 * dxyce_vbs0t_dt
            } else {
                0.0
            };

            let t0 = vbsmos + 5.0 - 0.001;
            let t1 = (t0 * t0 + 0.02).sqrt();
            let scale = 0.5 * (1.0 + t0 / t1);
            let t2 = -5.0 + 0.5 * (t0 + t1);
            let dt2_dvg = scale * dvbsmos_dvg;
            let dt2_dvd = scale * dvbsmos_dvd;
            let dt2_dvb = scale * dvbsmos_dvb;
            let dt2_dve = scale * dvbsmos_dve;
            let dt2_dt = if selfheat { scale * dvbsmos_dt } else { 0.0 };

            let t0 = 1.5;
            let t1 = t0 - t2 - 0.002;
            let t3 = (t1 * t1 + 0.008 * t0).sqrt();
            let vbsh = t0 - 0.5 * (t1 + t3);
            let scale = 0.5 * (1.0 + t1 / t3);
            let dvbsh_dvg = scale * dt2_dvg;
            let dvbsh_dvd = scale * dt2_dvd;
            let dvbsh_dvb = scale * dt2_dvb;
            let dvbsh_dve = scale * dt2_dve;
            let dvbsh_dt = if selfheat { scale * dt2_dt } else { 0.0 };

            let t0 = 0.95 * phi;
            let t1 = t0 - vbsh - 0.002;
            let t2 = (t1 * t1 + 0.008 * t0).sqrt();
            let xyce_vbseff = t0 - 0.5 * (t1 + t2);
            let scale = 0.5 * (1.0 + t1 / t2);
            let dvbseff_dvg = scale * dvbsh_dvg;
            let dvbseff_dvd = scale * dvbsh_dvd;
            let mut dvbseff_dvb = scale * dvbsh_dvb;
            let dvbseff_dve = scale * dvbsh_dve;
            let dvbseff_dt = if selfheat { scale * dvbsh_dt } else { 0.0 };
            let dvbsh_dvb_eff = if dvbseff_dvb < 1.0e-20 {
                dvbsh_dvb * 1.0e20
            } else {
                dvbsh_dvb / dvbseff_dvb
            };
            if dvbseff_dvb < 1.0e-20 {
                dvbseff_dvb = 1.0e-20;
            }

            (
                xyce_vbs0t,
                xyce_vbs0,
                dxyce_vbs0_dve,
                vbsmos,
                dvbsmos_dve,
                xyce_vbs0,
                dxyce_vbs0_dvg,
                dxyce_vbs0_dvd,
                dxyce_vbs0_dve,
                vbsitf,
                dvbsitf_dvg,
                dvbsitf_dvd,
                dvbsitf_dve,
                dvbsitf_dvb,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                xyce_vbseff,
                dvbseff_dvg,
                dvbseff_dvd,
                dvbseff_dvb,
                dvbseff_dve,
                dvbseff_dt,
                vbsh,
                dvbsh_dvb_eff,
            )
        }
    };

    // --- Vps check / Vpsdio / Vbp (b3soiddld.c:1193-1230) ---
    // bodyMod 0/2: Ibp == 0 so Vbp/Vpsdio derivatives are not needed downstream.
    let _ = (vps, DELT_VBS0DIO);

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
    let ddelt_vth_dt = thetavth * sh.dvbi_dt;

    let (deltvthw, ddeltvthw_dvb, deltvthw_coeff);
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
        deltvthw_coeff = p.dvt0w * t2v;
        deltvthw = deltvthw_coeff * v0;
        ddeltvthw_dvb = p.dvt0w * dt2_dvb * v0;
    }
    let ddeltvthw_dt = deltvthw_coeff * sh.dvbi_dt;

    let t0t = (1.0 + p.nlx / leff).sqrt();
    let t1t = p.kt1 + p.kt1l / leff + p.kt2 * vbseff;
    let delt_vthtemp = p.k1 * (t0t - 1.0) * sqrt_phi + t1t * temp_ratio;
    let ddelt_vthtemp_dt = if selfheat { t1t / p.tnom } else { 0.0 };

    let tmp2 = m.tox * phi / (p.weff + p.w0);

    let (t3e, dt3_dvb_eta) = smooth_etab(p.eta0, p.etab, vbseff);
    let dibl_sft = t3e * p.theta0vb0 * vds;
    let ddibl_sft_dvd = p.theta0vb0 * t3e;
    let ddibl_sft_dvb = p.theta0vb0 * vds * dt3_dvb_eta;

    let (sqrt_phis_vth, dsqrt_phis_vth_dvb) = match m.dialect {
        B3SoiDialect::Ngspice => (sqrt_phis, dsqrt_phis_dvb),
        B3SoiDialect::Xyce => {
            let t9 = 2.2361 / sqrt_phi;
            (
                sqrt_phis - t9 * (vbsh - vbseff),
                dsqrt_phis_dvb - t9 * (dvbsh_dvb_eff - 1.0),
            )
        }
    };

    let vth =
        mtype * p.vth0 + p.k1 * (sqrt_phis_vth - sqrt_phi) - p.k2 * vbseff - delt_vth - deltvthw
            + (p.k3 + p.k3b * vbseff) * tmp2
            + delt_vthtemp
            - dibl_sft;
    op.von = vth;

    let t6v = p.k3b * tmp2 - p.k2 + p.kt2 * temp_ratio;
    let dvth_dvb = p.k1 * dsqrt_phis_vth_dvb - ddelt_vth_dvb - ddeltvthw_dvb + t6v - ddibl_sft_dvb;
    let dvth_dvd = -ddibl_sft_dvd;
    let dvth_dt = ddelt_vthtemp_dt - ddelt_vth_dt - ddeltvthw_dt;

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

    let (vgsteff, dvgsteff_dvg, dvgsteff_dvd, dvgsteff_dvb, dvgsteff_dve, dvgsteff_dt);
    let mut exp_vgst = 0.0_f64;
    if vgst_n_vt > EXP_THRESHOLD {
        vgsteff = vgst;
        let t0 = -dvth_dvb;
        dvgsteff_dvg = dvgs_eff_dvg + t0 * dvbseff_dvg;
        dvgsteff_dvd = -dvth_dvd + t0 * dvbseff_dvd;
        dvgsteff_dvb = t0 * dvbseff_dvb;
        dvgsteff_dve = t0 * dvbseff_dve;
        dvgsteff_dt = if selfheat {
            -dvth_dt + t0 * dvbseff_dt
        } else {
            0.0
        };
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
        dvgsteff_dt = if selfheat {
            -t3 * (dvth_dt + t0 * KB_OVER_Q * n) + vgsteff / p.temp + t1 * dvbseff_dt
        } else {
            0.0
        };
    } else {
        exp_vgst = vgst_n_vt.exp();
        let t1 = t10 * (1.0 + exp_vgst).ln();
        let dt1_dvg = exp_vgst / (1.0 + exp_vgst);
        let dt1_dvb = -dt1_dvg * (dvth_dvb + vgst / n * dn_dvb) + t1 / n * dn_dvb;
        let dt1_dvd = -dt1_dvg * (dvth_dvd + vgst / n * dn_dvd) + t1 / n * dn_dvd;
        let dt1_dt = if selfheat {
            -dt1_dvg * (dvth_dt + vgst / p.temp) + t1 / p.temp
        } else {
            0.0
        };
        let dt2_dvg = -m.cox / (vtm * p.cdep0) * exp_arg.exp();
        let t2 = 1.0 - t10 * dt2_dvg;
        let dt2_dvd =
            -dt2_dvg * (dvth_dvd - 2.0 * vtm * exp_arg * dn_dvd) + (t2 - 1.0) / n * dn_dvd;
        let dt2_dvb =
            -dt2_dvg * (dvth_dvb - 2.0 * vtm * exp_arg * dn_dvb) + (t2 - 1.0) / n * dn_dvb;
        let dt2_dt = if selfheat {
            -dt2_dvg * (dvth_dt - exp_arg * t10 / p.temp)
        } else {
            0.0
        };
        vgsteff = t1 / t2;
        let t3 = t2 * t2;
        let t4 = (t2 * dt1_dvb - t1 * dt2_dvb) / t3;
        dvgsteff_dvb = t4 * dvbseff_dvb;
        dvgsteff_dve = t4 * dvbseff_dve;
        dvgsteff_dvg = (t2 * dt1_dvg - t1 * dt2_dvg) / t3 * dvgs_eff_dvg + t4 * dvbseff_dvg;
        dvgsteff_dvd = (t2 * dt1_dvd - t1 * dt2_dvd) / t3 + t4 * dvbseff_dvd;
        dvgsteff_dt = if selfheat {
            (t2 * dt1_dt - t1 * dt2_dt) / t3 + t4 * dvbseff_dt
        } else {
            0.0
        };
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

    // --- Abulk / Abulk0 ---
    let (abulk0, abulk, dabulk0_dvb, dabulk_dvg, dabulk_dvb) = match m.dialect {
        B3SoiDialect::Ngspice => {
            // ngspice b3soiddld.c:1558-1620.
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
                dabulk_dvg *= t9 * t9;
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
            (
                abulk0 + 1.0,
                abulk + 1.0,
                dabulk0_dvb,
                dabulk_dvg,
                dabulk_dvb,
            )
        }
        B3SoiDialect::Xyce => {
            // Xyce N_DEV_MOSFET_B3SOI.C:10134-10196.
            let (mut abulk0, mut abulk, mut dabulk0_dvb, mut dabulk_dvg, mut dabulk_dvb);
            if p.a0 == 0.0 {
                abulk0 = 1.0;
                abulk = 1.0;
                dabulk0_dvb = 0.0;
                dabulk_dvg = 0.0;
                dabulk_dvb = 0.0;
            } else {
                let t10 = p.keta * vbsh;
                let (t11, dt11_dvb);
                if t10 >= -0.9 {
                    let t = 1.0 / (1.0 + t10);
                    t11 = t;
                    dt11_dvb = -p.keta * t * t * dvbsh_dvb_eff;
                } else {
                    let t12 = 1.0 / (0.8 + t10);
                    t11 = (17.0 + 20.0 * t10) * t12;
                    dt11_dvb = -p.keta * t12 * t12 * dvbsh_dvb_eff;
                }

                let t10 = phi + p.ketas;
                let t13 = vbsh * t11 / t10;
                let dt13_dvb = (vbsh * dt11_dvb + t11 * dvbsh_dvb_eff) / t10;
                let (t14, dt14_dvb);
                if t13 < 0.96 {
                    let t = 1.0 / (1.0 - t13).sqrt();
                    t14 = t;
                    dt14_dvb = 0.5 * t / (1.0 - t13) * dt13_dvb;
                } else {
                    let t11b = 1.0 / (1.0 - 1.043406 * t13);
                    t14 = (6.00167 - 6.26044 * t13) * t11b;
                    dt14_dvb = 0.001742 * t11b * t11b * dt13_dvb;
                }

                let t1 = 0.5 * p.k1 / (phi + p.ketas).sqrt() * t14;
                let dt1_dvb = 0.5 * p.k1 / (phi + p.ketas).sqrt() * dt14_dvb;
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

                abulk0 = 1.0 + t1 * t2;
                dabulk0_dvb = t1 * dt2_dvb + t2 * dt1_dvb;
                let t8 = p.ags * p.a0 * t7;
                dabulk_dvg = -t1 * t8;
                abulk = abulk0 + dabulk_dvg * vgsteff;
                dabulk_dvb = dabulk0_dvb - t8 * vgsteff * (dt1_dvb + 3.0 * t1 * dt2_dvb / tmp2b);
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
                dabulk_dvg *= t9 * t9;
            }
            (abulk0, abulk, dabulk0_dvb, dabulk_dvg, dabulk_dvb)
        }
    };

    let (abeff, dabeff_dvg, dabeff_dvb, dabeff_dvc) = match m.dialect {
        B3SoiDialect::Ngspice => {
            // Ngspice DD `B3SOIDDload` computes an effective Abulk that blends
            // the channel Abulk with `ADICE` through the Vcs-dependent Xcsat
            // transition (`b3soiddld.c`, "Prepare Abeff").
            const DELT_XCSAT: Value = 0.2;
            let t0 = p.abp * vgst2vtm;
            let t1 = 1.0 - vcs / t0 - DELT_XCSAT;
            let t2 = (t1 * t1 + DELT_XCSAT * DELT_XCSAT).sqrt();
            let t3 = 1.0 - 0.5 * (t1 + t2);
            let dt1_dvg = vcs / (vgst2vtm * t0);
            let dt1_dvc = -1.0 / t0;
            let t4 = -0.5 * (1.0 + t1 / t2);
            let dt3_dvg = t4 * dt1_dvg;
            let dt3_dvc = t4 * dt1_dvc;
            let xcsat = p.mxc * t3 * t3 + (1.0 - p.mxc) * t3;
            let dxcsat_factor = 2.0 * p.mxc * t3 + (1.0 - p.mxc);
            let dxcsat_dvg = dxcsat_factor * dt3_dvg;
            let dxcsat_dvc = dxcsat_factor * dt3_dvc;
            (
                xcsat * abulk + (1.0 - xcsat) * m.adice,
                xcsat * dabulk_dvg + (abulk - m.adice) * dxcsat_dvg,
                xcsat * dabulk_dvb,
                (abulk - m.adice) * dxcsat_dvc,
            )
        }
        B3SoiDialect::Xyce => {
            // Xyce B3SOI v3.2 uses Abulk directly in the current path; there
            // is no Abeff/Xcsat correction in `N_DEV_MOSFET_B3SOI.C`.
            (abulk, dabulk_dvg, dabulk_dvb, 0.0)
        }
    };

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
        let dt0_dvg =
            2.0 * (t8 * tmp2v - abeff * tmp1l + (2.0 * t9 + 1.0 / lambda - 1.0) * dabeff_dvg);
        let dt0_dvb =
            2.0 * (t8 * (2.0 / abeff * dabeff_dvb + tmp3v) + (1.0 / lambda - 1.0) * dabeff_dvb);
        let _dt0_dvd = 0.0; // ngspice dT0_dVd = 0 in this branch (unused below)
        let dt0_dvc = 4.0 * t9 * dabeff_dvc;

        let t1 = vgst2vtm * (2.0 / lambda - 1.0) + abeff * esat_l + 3.0 * t7;
        let dt1_dvg = (2.0 / lambda - 1.0) - 2.0 * vgst2vtm * tmp1l
            + abeff * desat_l_dvg
            + esat_l * dabeff_dvg
            + 3.0 * (t9 + t7 * tmp2v + t6 * dabeff_dvg);
        let dt1_dvb =
            abeff * desat_l_dvb + esat_l * dabeff_dvb + 3.0 * (t6 * dabeff_dvb + t7 * tmp3v);
        let dt1_dvd = abeff * desat_l_dvd;
        let dt1_dvc = esat_l * dabeff_dvc + 3.0 * t6 * dabeff_dvc;

        let t2 = vgst2vtm * (esat_l + 2.0 * t6);
        let dt2_dvg = esat_l + vgst2vtm * desat_l_dvg + t6 * (4.0 + 2.0 * vgst2vtm * tmp2v);
        let dt2_dvb = vgst2vtm * (desat_l_dvb + 2.0 * t6 * tmp3v);
        let dt2_dvd = vgst2vtm * desat_l_dvd;

        let t3 = (t1 * t1 - 2.0 * t0 * t2).sqrt();
        vdsat = (t1 - t3) / t0;
        dvdsat_dvg =
            (dt1_dvg - (t1 * dt1_dvg - dt0_dvg * t2 - t0 * dt2_dvg) / t3 - vdsat * dt0_dvg) / t0;
        dvdsat_dvb =
            (dt1_dvb - (t1 * dt1_dvb - dt0_dvb * t2 - t0 * dt2_dvb) / t3 - vdsat * dt0_dvb) / t0;
        dvdsat_dvd = (dt1_dvd - (t1 * dt1_dvd - t0 * dt2_dvd) / t3) / t0;
        dvdsat_dvc = (dt1_dvc - (t1 * dt1_dvc - dt0_dvc * t2) / t3 - vdsat * dt0_dvc) / t0;
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
        let dt0_dvb = -0.5 * (abeff * dvdseff_dvb + dabeff_dvb * vdseff) / vgst2vtm;
        let dt0_dvc = -0.5 * (abeff * dvdseff_dvc + dabeff_dvc * vdseff) / vgst2vtm;
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

    let (iii, giig, giib, giid, giie) = match m.dialect {
        B3SoiDialect::Ngspice => {
            let t2 = p.alpha1 + p.alpha0 / leff;
            if t2 <= 0.0 || p.beta0 <= 0.0 {
                (0.0, 0.0, 0.0, 0.0, 0.0)
            } else {
                let t5 = p.beta0;
                let (t1, dt1_dvg, dt1_dvd, dt1_dvb) = if diff_vdsii > t5 / EXP_THRESHOLD {
                    let t0 = -t5 / diff_vdsii;
                    let t10 = t0 / diff_vdsii;
                    let dt0_dvg = t10 * dvdseffii_dvg;
                    let t1 = t2 * diff_vdsii * t0.exp();
                    let t3 = t1 / diff_vdsii * (t0 - 1.0);
                    (
                        t1,
                        t1 * (dt0_dvg - dvdseffii_dvg / diff_vdsii),
                        -t3 * (1.0 - dvdseffii_dvd),
                        t3 * dvdseffii_dvb,
                    )
                } else {
                    let t3 = t2 * MIN_EXP;
                    (
                        t3 * diff_vdsii,
                        -t3 * dvdseffii_dvg,
                        t3 * (1.0 - dvdseffii_dvd),
                        -t3 * dvdseffii_dvb,
                    )
                };

                let iii = t1 * ids;
                let t2g = t1 * gm0 + ids * dt1_dvg;
                let t3d = t1 * gds0 + ids * dt1_dvd;
                let t4b = t1 * gmb0 + ids * dt1_dvb;
                let t5c = t1 * gmc;
                (
                    iii,
                    t2g * dvgsteff_dvg + t4b * dvbseff_dvg + t5c * dvcs_dvg,
                    t2g * dvgsteff_dvb + t4b * dvbseff_dvb + t5c * dvcs_dvb,
                    t2g * dvgsteff_dvd + t4b * dvbseff_dvd + t5c * dvcs_dvd + t3d,
                    t2g * dvgsteff_dve + t4b * dvbseff_dve + t5c * dvcs_dve,
                )
            }
        }
        B3SoiDialect::Xyce => {
            let (mut iii, mut giig, mut giib, mut giid, mut giie) = (0.0, 0.0, 0.0, 0.0, 0.0);
            if p.alpha0 > 0.0 {
                let dvgst_dvg = dvgs_eff_dvg;
                let dvgst_dvd = -dvth_dvd;
                let dvgst_dvb = -dvth_dvb;

                let vdsatii0 = p.vdsatii0 * (1.0 + p.tii * temp_ratio) - p.lii / leff;

                let t0 = p.esatii * leff;
                let t1 = p.sii0 * t0 / (1.0 + t0);
                let t0 = 1.0 / (1.0 + p.sii1 * vgsteff);
                let t3 = t0 + p.sii2;
                let t4 = vgst * p.sii1 * t0 * t0;
                let t2 = vgst * t3;
                let dt2_dvg = t3 * (dvgst_dvg - dvth_dvb * dvbseff_dvg) - t4 * dvgsteff_dvg;
                let dt2_dvb = t3 * dvgst_dvb * dvbseff_dvb - t4 * dvgsteff_dvb;
                let dt2_dve = t3 * dvgst_dvb * dvbseff_dve - t4 * dvgsteff_dve;
                let dt2_dvd = t3 * (dvgst_dvd - dvth_dvb * dvbseff_dvd) - t4 * dvgsteff_dvd;

                let t3d = 1.0 / (1.0 + p.siid * vds);
                let dt3_dvd = -p.siid * t3d * t3d;

                let vgs_step = t1 * t2 * t3d;
                let vdsatii = vdsatii0 + vgs_step;
                let vdiff = vds - vdsatii;
                let dvdiff_dvg = -t1 * t3d * dt2_dvg;
                let dvdiff_dvb = -t1 * t3d * dt2_dvb;
                let dvdiff_dve = -t1 * t3d * dt2_dve;
                let dvdiff_dvd = 1.0 - t1 * (t3d * dt2_dvd + t2 * dt3_dvd);

                let t0 = p.beta2 + p.beta1 * vdiff + p.beta0 * vdiff * vdiff;
                let (t0, dt0_dvg, dt0_dvb, dt0_dve, dt0_dvd) = if t0 < 1.0e-5 {
                    (1.0e-5, 0.0, 0.0, 0.0, 0.0)
                } else {
                    let t1 = p.beta1 + 2.0 * p.beta0 * vdiff;
                    (
                        t0,
                        t1 * dvdiff_dvg,
                        t1 * dvdiff_dvb,
                        t1 * dvdiff_dve,
                        t1 * dvdiff_dvd,
                    )
                };

                let (mut ratio, mut dratio_dvg, mut dratio_dvb, mut dratio_dve, mut dratio_dvd);
                if t0 < vdiff / EXP_THRESHOLD && vdiff > 0.0 {
                    ratio = p.alpha0 * MAX_EXP;
                    dratio_dvg = 0.0;
                    dratio_dvb = 0.0;
                    dratio_dve = 0.0;
                    dratio_dvd = 0.0;
                } else if t0 < -vdiff / EXP_THRESHOLD && vdiff < 0.0 {
                    ratio = p.alpha0 * MIN_EXP;
                    dratio_dvg = 0.0;
                    dratio_dvb = 0.0;
                    dratio_dve = 0.0;
                    dratio_dvd = 0.0;
                } else {
                    ratio = p.alpha0 * (vdiff / t0).exp();
                    let t1 = ratio / t0 / t0;
                    dratio_dvg = t1 * (t0 * dvdiff_dvg - vdiff * dt0_dvg);
                    dratio_dvb = t1 * (t0 * dvdiff_dvb - vdiff * dt0_dvb);
                    dratio_dve = t1 * (t0 * dvdiff_dve - vdiff * dt0_dve);
                    dratio_dvd = t1 * (t0 * dvdiff_dvd - vdiff * dt0_dvd);
                }
                if ratio > 10.0 {
                    ratio = 10.0;
                    dratio_dvg = 0.0;
                    dratio_dvb = 0.0;
                    dratio_dve = 0.0;
                    dratio_dvd = 0.0;
                }

                // Xyce adds `fbjtii * Ic`; Ic is evaluated later in this port and the
                // covered Xyce decks use FBJTII=0, so this remains the Ids term.
                let t0 = ids + p.fbjtii * 0.0;
                iii = ratio * t0;
                giig = ratio * gm + t0 * dratio_dvg;
                giib = ratio * gmb + t0 * dratio_dvb;
                giid = ratio * gds + t0 * dratio_dvd;
                giie = ratio * gme + t0 * dratio_dve;
            }
            (iii, giig, giib, giid, giie)
        }
    };

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

    // --- Xyce v3.2 body diodes + parasitic BJT (N_DEV_MOSFET_B3SOI.C). ---
    let w_tsi = p.weff / p.nseg * m.tsi;
    let vbd = vbs - vds; // device-internal Vbd in evaluation frame
    let n_vtm1 = vtm * p.ndiode;
    let dexp = |arg: Value| -> (Value, Value) {
        const MAX_EXPL: Value = 2.688_117_142e43;
        const MIN_EXPL: Value = 3.720_075_976e-44;
        const EXPL_THRESHOLD: Value = 100.0;
        if arg > EXPL_THRESHOLD {
            (MAX_EXPL * (1.0 + arg - EXPL_THRESHOLD), MAX_EXPL)
        } else if arg < -EXPL_THRESHOLD {
            (MIN_EXPL, 0.0)
        } else {
            let e = arg.exp();
            (e, e)
        }
    };
    let (exp_vbs_n, dexp_vbs_n) = dexp(vbs / n_vtm1);
    let dexp_vbs_n_dvb = dexp_vbs_n / n_vtm1;
    let dexp_vbs_n_dt = if selfheat {
        dexp_vbs_n * (-vbs / (n_vtm1 * n_vtm1) * p.ndiode * KB_OVER_Q)
    } else {
        0.0
    };
    let (exp_vbd_n, dexp_vbd_n) = dexp(vbd / n_vtm1);
    let dexp_vbd_n_dvb = dexp_vbd_n / n_vtm1;
    let dexp_vbd_n_dvd = -dexp_vbd_n_dvb;
    let dexp_vbd_n_dt = if selfheat {
        dexp_vbd_n * (-vbd / (n_vtm1 * n_vtm1) * p.ndiode * KB_OVER_Q)
    } else {
        0.0
    };

    // Ibs1 / Ibd1: diffusion.
    let (ibs1, dibs1_dvb, _dibs1_dt, ibd1, dibd1_dvb, dibd1_dvd, _dibd1_dt);
    if jdif == 0.0 {
        ibs1 = 0.0;
        dibs1_dvb = 0.0;
        _dibs1_dt = 0.0;
        ibd1 = 0.0;
        dibd1_dvb = 0.0;
        dibd1_dvd = 0.0;
        _dibd1_dt = 0.0;
    } else {
        let t0 = w_tsi * jdif;
        let dt0_dt = w_tsi * sh.djdif_dt;
        ibs1 = t0 * (exp_vbs_n - 1.0);
        dibs1_dvb = t0 * dexp_vbs_n_dvb;
        _dibs1_dt = if selfheat {
            dt0_dt * (exp_vbs_n - 1.0) + t0 * dexp_vbs_n_dt
        } else {
            0.0
        };
        ibd1 = t0 * (exp_vbd_n - 1.0);
        dibd1_dvb = t0 * dexp_vbd_n_dvb;
        dibd1_dvd = -dibd1_dvb;
        _dibd1_dt = if selfheat {
            dt0_dt * (exp_vbd_n - 1.0) + t0 * dexp_vbd_n_dt
        } else {
            0.0
        };
    }

    // Ibs2 / Ibd2: recombination + reverse trap-assisted tunneling.
    let n_vtmf = 0.026 * p.nrecf0 * (1.0 + p.ntrecf * temp_ratio);
    let n_vtmr = 0.026 * p.nrecr0 * (1.0 + p.ntrecr * temp_ratio);
    let (ibs2, dibs2_dvb, ibd2, dibd2_dvb, dibd2_dvd);
    if jrec == 0.0 {
        ibs2 = 0.0;
        dibs2_dvb = 0.0;
        ibd2 = 0.0;
        dibd2_dvb = 0.0;
        dibd2_dvd = 0.0;
    } else {
        let (t10s, e10s) = dexp(vbs / n_vtmf);
        let dt10s_dvb = e10s / n_vtmf;
        let (t11s, dt11s_dvb) = if (p.vrec0 - vbs) < 1.0e-3 {
            let t0 = -vbs / n_vtmr * p.vrec0 * 1.0e3;
            (-t0.exp(), 0.0)
        } else {
            let t1 = 1.0 / (p.vrec0 - vbs);
            let t0 = -vbs / n_vtmr * p.vrec0 * t1;
            let dt0_dvb = -p.vrec0 / n_vtmr * (t1 + vbs * t1 * t1);
            let (e, de) = dexp(t0);
            (-e, -de * dt0_dvb)
        };
        let t3s = w_tsi * jrec;
        ibs2 = t3s * (t10s + t11s);
        dibs2_dvb = t3s * (dt10s_dvb + dt11s_dvb);

        let (t10d, e10d) = dexp(vbd / n_vtmf);
        let dt10d_dvb = e10d / n_vtmf;
        let (t11d, dt11d_dvb) = if (p.vrec0 - vbd) < 1.0e-3 {
            let t0 = -vbd / n_vtmr * p.vrec0 * 1.0e3;
            (-t0.exp(), 0.0)
        } else {
            let t1 = 1.0 / (p.vrec0 - vbd);
            let t0 = -vbd / n_vtmr * p.vrec0 * t1;
            let dt0_dvb = -p.vrec0 / n_vtmr * (t1 + vbd * t1 * t1);
            let (e, de) = dexp(t0);
            (-e, -de * dt0_dvb)
        };
        let t3d = w_tsi * jrec;
        ibd2 = t3d * (t10d + t11d);
        dibd2_dvb = t3d * (dt10d_dvb + dt11d_dvb);
        dibd2_dvd = -dibd2_dvb;
    }

    // Ibs3 / Ibd3: neutral-body recombination with high-level injection, plus Ic.
    let (mut ic, mut gcd, mut gcb) = (0.0, 0.0, 0.0);
    let (
        ibs3,
        dibs3_dvb,
        dibs3_dvd,
        ibd3,
        dibd3_dvb,
        dibd3_dvd,
        ibsdif,
        dibsdif_dvb,
        dibsdif_dt,
        ibddif,
        dibddif_dvb,
        dibddif_dvd,
        dibddif_dt,
    );
    if jbjt == 0.0 {
        ibs3 = 0.0;
        dibs3_dvb = 0.0;
        dibs3_dvd = 0.0;
        ibd3 = 0.0;
        dibd3_dvb = 0.0;
        dibd3_dvd = 0.0;
        ibsdif = 0.0;
        dibsdif_dvb = 0.0;
        dibsdif_dt = 0.0;
        ibddif = 0.0;
        dibddif_dvb = 0.0;
        dibddif_dvd = 0.0;
        dibddif_dt = 0.0;
    } else {
        let ien = w_tsi * jbjt * p.lratio;
        let ahli = p.ahli0;

        let (ehlis, dehlis_dvb, ehlis_factor, dehlis_factor_dvb, dehlis_factor_dt) = {
            let e = ahli * (exp_vbs_n - 1.0);
            if e < 1.0e-5 {
                (0.0, 0.0, 1.0, 0.0, 0.0)
            } else {
                let de = ahli * dexp_vbs_n_dvb;
                let de_dt = ahli * dexp_vbs_n_dt + (exp_vbs_n - 1.0) * sh.dahli_dt;
                let f = 1.0 / (1.0 + e).sqrt();
                let t0 = -0.5 * f / (1.0 + e);
                (e, de, f, t0 * de, t0 * de_dt)
            }
        };
        let (ehlid, dehlid_dvb, dehlid_dvd, ehlid_factor, ehlid_factor_dvb, dehlid_factor_dt) = {
            let e = ahli * (exp_vbd_n - 1.0);
            if e < 1.0e-5 {
                (0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
            } else {
                let de = ahli * dexp_vbd_n_dvb;
                let de_dvd = -de;
                let de_dt = ahli * dexp_vbd_n_dt + (exp_vbd_n - 1.0) * sh.dahli_dt;
                let f = 1.0 / (1.0 + e).sqrt();
                let t0 = -0.5 * f / (1.0 + e);
                (e, de, de_dvd, f, t0 * de, t0 * de_dt)
            }
        };

        let t1 = (1.0 - p.arfabjt) * ien;
        ibs3 = t1 * (exp_vbs_n - 1.0) * ehlis_factor;
        dibs3_dvb = t1 * (dexp_vbs_n_dvb * ehlis_factor + (exp_vbs_n - 1.0) * dehlis_factor_dvb);
        dibs3_dvd = 0.0;
        ibd3 = t1 * (exp_vbd_n - 1.0) * ehlid_factor;
        dibd3_dvb = t1 * (dexp_vbd_n_dvb * ehlid_factor + (exp_vbd_n - 1.0) * ehlid_factor_dvb);
        dibd3_dvd = -dibd3_dvb;

        let iendif = w_tsi * jbjt * p.lratiodif;
        let diendif_dt = w_tsi * sh.djbjt_dt * p.lratiodif;
        ibsdif = iendif * (exp_vbs_n - 1.0) * ehlis_factor;
        dibsdif_dvb =
            iendif * (dexp_vbs_n_dvb * ehlis_factor + (exp_vbs_n - 1.0) * dehlis_factor_dvb);
        dibsdif_dt = if selfheat {
            diendif_dt * (exp_vbs_n - 1.0) * ehlis_factor
                + iendif * (dexp_vbs_n_dt * ehlis_factor + (exp_vbs_n - 1.0) * dehlis_factor_dt)
        } else {
            0.0
        };
        ibddif = iendif * (exp_vbd_n - 1.0) * ehlid_factor;
        dibddif_dvb =
            iendif * (dexp_vbd_n_dvb * ehlid_factor + (exp_vbd_n - 1.0) * ehlid_factor_dvb);
        dibddif_dvd = -dibddif_dvb;
        dibddif_dt = if selfheat {
            diendif_dt * (exp_vbd_n - 1.0) * ehlid_factor
                + iendif * (dexp_vbd_n_dt * ehlid_factor + (exp_vbd_n - 1.0) * dehlid_factor_dt)
        } else {
            0.0
        };

        if vds != 0.0 {
            let t0 = 1.0 + (vbs + vbd) / p.vearly;
            let dt0_dvb = 2.0 / p.vearly;
            let dt0_dvd = -1.0 / p.vearly;
            let t1 = ehlis + ehlid;
            let dt1_dvb = dehlis_dvb + dehlid_dvb;
            let dt1_dvd = dehlid_dvd;
            let t3 = (t0 * t0 + 4.0 * t1).sqrt();
            let dt3_dvb = 0.5 / t3 * (2.0 * t0 * dt0_dvb + 4.0 * dt1_dvb);
            let dt3_dvd = 0.5 / t3 * (2.0 * t0 * dt0_dvd + 4.0 * dt1_dvd);
            let t2 = (t0 + t3) / 2.0;
            let dt2_dvb = (dt0_dvb + dt3_dvb) / 2.0;
            let dt2_dvd = (dt0_dvd + dt3_dvd) / 2.0;
            let (e2nd, de2nd_dvb, de2nd_dvd) = if t2 < 0.1 {
                (10.0, 0.0, 0.0)
            } else {
                let f = 1.0 / t2;
                (f, -f / t2 * dt2_dvb, -f / t2 * dt2_dvd)
            };
            let t0c = p.arfabjt * ien;
            ic = t0c * (exp_vbs_n - exp_vbd_n) * e2nd;
            gcb = t0c
                * ((dexp_vbs_n_dvb - dexp_vbd_n_dvb) * e2nd + (exp_vbs_n - exp_vbd_n) * de2nd_dvb);
            gcd = t0c * (-dexp_vbd_n_dvd * e2nd + (exp_vbs_n - exp_vbd_n) * de2nd_dvd);
        }
    }

    // Ibs4 / Ibd4: band-to-band tunneling.
    let n_vtm2 = 0.026 * p.ntun;
    let (ibs4, dibs4_dvb, ibd4, dibd4_dvb, dibd4_dvd);
    if jtun == 0.0 {
        ibs4 = 0.0;
        dibs4_dvb = 0.0;
        ibd4 = 0.0;
        dibd4_dvb = 0.0;
        dibd4_dvd = 0.0;
    } else {
        let t3s = w_tsi * jtun;
        if (p.vtun0 - vbs) < 1.0e-3 {
            let t0 = -vbs / n_vtm2 * p.vtun0 * 1.0e3;
            ibs4 = t3s * (1.0 - t0.exp());
            dibs4_dvb = 0.0;
        } else {
            let t1 = 1.0 / (p.vtun0 - vbs);
            let t0 = -vbs / n_vtm2 * p.vtun0 * t1;
            let dt0_dvb = -p.vtun0 / n_vtm2 * (t1 + vbs * t1 * t1);
            let (e, de) = dexp(t0);
            ibs4 = t3s * (1.0 - e);
            dibs4_dvb = -t3s * de * dt0_dvb;
        }

        let t3d = w_tsi * jtun;
        if (p.vtun0 - vbd) < 1.0e-3 {
            let t0 = -vbd / n_vtm2 * p.vtun0 * 1.0e3;
            ibd4 = t3d * (1.0 - t0.exp());
            dibd4_dvb = 0.0;
            dibd4_dvd = 0.0;
        } else {
            let t1 = 1.0 / (p.vtun0 - vbd);
            let t0 = -vbd / n_vtm2 * p.vtun0 * t1;
            let dt0_dvb = -p.vtun0 / n_vtm2 * (t1 + vbd * t1 * t1);
            let (e, de) = dexp(t0);
            ibd4 = t3d * (1.0 - e);
            dibd4_dvb = -t3d * de * dt0_dvb;
            dibd4_dvd = -dibd4_dvb;
        }
    }

    let xyce_body = BodyCurrents {
        ic,
        gcd,
        gcb,
        ibs: ibs1 + ibs2 + ibs3 + ibs4,
        ibd: ibd1 + ibd2 + ibd3 + ibd4,
        gjsb: dibs1_dvb + dibs2_dvb + dibs3_dvb + dibs4_dvb,
        gjsd: dibs3_dvd,
        gjdb: dibd1_dvb + dibd2_dvb + dibd3_dvb + dibd4_dvb,
        gjdd: dibd1_dvd + dibd2_dvd + dibd3_dvd + dibd4_dvd,
        storage_ibs: ibsdif,
        storage_dibs_dvb: dibsdif_dvb,
        storage_dibs_dt: dibsdif_dt,
        storage_ibd: ibddif,
        storage_dibd_dvb: dibddif_dvb,
        storage_dibd_dvd: dibddif_dvd,
        storage_dibd_dt: dibddif_dt,
    };

    let ngspice_body = || {
        let w_tsi = p.weff * m.tsi;
        let n_vtm1 = vtm * p.ndiode;
        let n_vtm2 = vtm * p.ntun;
        let exp30 = 1.0686e13;
        let exp_forward = |voltage: Value| -> (Value, Value, Value) {
            let arg = voltage / n_vtm1;
            if arg < 30.0 {
                let exp_arg = arg.exp();
                let dexp_dv = exp_arg / n_vtm1;
                let dexp_dt = if selfheat {
                    -arg * exp_arg / p.temp
                } else {
                    0.0
                };
                (exp_arg, dexp_dv, dexp_dt)
            } else {
                let dexp_dv = exp30 / n_vtm1;
                let exp_arg = dexp_dv * voltage - 29.0 * exp30;
                let dexp_dt = if selfheat {
                    -dexp_dv * voltage / p.temp
                } else {
                    0.0
                };
                (exp_arg, dexp_dv, dexp_dt)
            }
        };
        let exp_reverse = |voltage: Value| -> (Value, Value, Value) {
            let arg = -voltage / n_vtm2;
            if arg < 30.0 {
                let exp_arg = arg.exp();
                let dexp_dv = -exp_arg / n_vtm2;
                let dexp_dt = if selfheat {
                    -arg * exp_arg / p.temp
                } else {
                    0.0
                };
                (exp_arg, dexp_dv, dexp_dt)
            } else {
                let dexp_dv = -exp30 / n_vtm2;
                let exp_arg = dexp_dv * voltage - 29.0 * exp30;
                let dexp_dt = if selfheat {
                    -dexp_dv * voltage / p.temp
                } else {
                    0.0
                };
                (exp_arg, dexp_dv, dexp_dt)
            }
        };

        let (exp_vbs1, dexp_vbs1_dvb, dexp_vbs1_dt) = exp_forward(vbs);
        let (exp_vbd1, dexp_vbd1_dvb, dexp_vbd1_dt) = exp_forward(vbd);

        let (n_ibs1, n_dibs1_dvb, n_dibs1_dt, n_ibd1, n_dibd1_dvb, n_dibd1_dvd, n_dibd1_dt) =
            if jdif == 0.0 {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
            } else {
                let t5 = w_tsi * jdif;
                let ibs = t5 * (exp_vbs1 - 1.0);
                let ibd = t5 * (exp_vbd1 - 1.0);
                (
                    ibs,
                    t5 * dexp_vbs1_dvb,
                    if selfheat {
                        ibs / jdif * sh.djdif_dt + t5 * dexp_vbs1_dt
                    } else {
                        0.0
                    },
                    ibd,
                    t5 * dexp_vbd1_dvb,
                    -t5 * dexp_vbd1_dvb,
                    if selfheat {
                        ibd / jdif * sh.djdif_dt + t5 * dexp_vbd1_dt
                    } else {
                        0.0
                    },
                )
            };

        let (n_ibs2, n_dibs2_dvb, n_ibd2, n_dibd2_dvb, n_dibd2_dvd) = if jrec == 0.0 {
            (0.0, 0.0, 0.0, 0.0, 0.0)
        } else {
            let exp_vbs2 = exp_vbs1.sqrt();
            let dexp_vbs2_dvb = if exp_vbs2 > 1.0e-20 {
                0.5 / exp_vbs2 * dexp_vbs1_dvb
            } else {
                0.0
            };
            let exp_vbd2 = exp_vbd1.sqrt();
            let dexp_vbd2_dvb = if exp_vbd2 > 1.0e-20 {
                0.5 / exp_vbd2 * dexp_vbd1_dvb
            } else {
                0.0
            };
            let t8 = w_tsi * jrec;
            (
                t8 * (exp_vbs2 - 1.0),
                t8 * dexp_vbs2_dvb,
                t8 * (exp_vbd2 - 1.0),
                t8 * dexp_vbd2_dvb,
                -t8 * dexp_vbd2_dvb,
            )
        };

        let (mut n_ic, mut n_gcd, mut n_gcb) = (0.0, 0.0, 0.0);
        let (n_ibs3, n_dibs3_dvb, n_dibs3_dvd, n_ibd3, n_dibd3_dvb, n_dibd3_dvd) =
            if vds == 0.0 || jbjt == 0.0 {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
            } else {
                let mut t1 = (leff - p.kbjt1 * vds) / p.edl;
                let mut dt1_dvd = -p.kbjt1 / p.edl;
                if t1 < 1.0e-3 {
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
                let ibs = t3 * exp_vbs1;
                let dibs_dvb = t3 * dexp_vbs1_dvb;
                let dibs_dvd = t4 * exp_vbs1;
                let ibd = t3 * exp_vbd1;
                let dibd_dvb = t3 * dexp_vbd1_dvb;
                let dibd_dvd = t4 * exp_vbd1 - dibd_dvb;

                n_ic = ibjt - ibs + ibd;
                n_gcd = dibjt_dvd - dibs_dvd + dibd_dvd;
                n_gcb = dibjt_dvb - dibs_dvb + dibd_dvb;

                (ibs, dibs_dvb, dibs_dvd, ibd, dibd_dvb, dibd_dvd)
            };

        let (n_ibs4, n_dibs4_dvb, n_ibd4, n_dibd4_dvb, n_dibd4_dvd) = if jtun == 0.0 {
            (0.0, 0.0, 0.0, 0.0, 0.0)
        } else {
            let (exp_vbs4, dexp_vbs4_dvb, _) = exp_reverse(vbs);
            let (exp_vbd4, dexp_vbd4_dvb, _) = exp_reverse(vbd);
            let t5 = w_tsi * jtun;
            let ibs = t5 * (1.0 - exp_vbs4);
            let dibs_dvb = -t5 * dexp_vbs4_dvb;
            let ibd = t5 * (1.0 - exp_vbd4);
            let dibd_dvb = -t5 * dexp_vbd4_dvb;
            (ibs, dibs_dvb, ibd, dibd_dvb, -dibd_dvb)
        };

        BodyCurrents {
            ic: n_ic,
            gcd: n_gcd,
            gcb: n_gcb,
            ibs: n_ibs1 + n_ibs2 + n_ibs3 + n_ibs4,
            ibd: n_ibd1 + n_ibd2 + n_ibd3 + n_ibd4,
            gjsb: n_dibs1_dvb + n_dibs2_dvb + n_dibs3_dvb + n_dibs4_dvb,
            gjsd: n_dibs3_dvd,
            gjdb: n_dibd1_dvb + n_dibd2_dvb + n_dibd3_dvb + n_dibd4_dvb,
            gjdd: n_dibd1_dvd + n_dibd2_dvd + n_dibd3_dvd + n_dibd4_dvd,
            storage_ibs: n_ibs1,
            storage_dibs_dvb: n_dibs1_dvb,
            storage_dibs_dt: n_dibs1_dt,
            storage_ibd: n_ibd1,
            storage_dibd_dvb: n_dibd1_dvb,
            storage_dibd_dvd: n_dibd1_dvd,
            storage_dibd_dt: n_dibd1_dt,
        }
    };

    let body = match m.dialect {
        B3SoiDialect::Ngspice => ngspice_body(),
        B3SoiDialect::Xyce => xyce_body,
    };
    let ic = body.ic;
    let gcd = body.gcd;
    let gcb = body.gcb;
    let ibs = body.ibs;
    let ibd = body.ibd;
    let gjsb = body.gjsb;
    let gjsd = body.gjsd;
    let gjdb = body.gjdb;
    let gjdd = body.gjdd;

    // bodyMod 0/2: Ibp == 0. Ngspice DD includes a tiny `minIsub`
    // convergence current in the equivalent junction/body sources; Xyce's
    // B3SOI equations do not.
    let min_isub = match m.dialect {
        B3SoiDialect::Ngspice => p.min_isub,
        B3SoiDialect::Xyce => 0.0,
    };

    // --- Operating-point assembly (b3soiddld.c:2556-2640) ---
    op.cdrain = ids + ic;
    op.cd = ids + ic - ibd + iii + idgidl;
    op.cb = ibs + ibd - iii - idgidl - isgidl;

    op.gds = gds + gcd;
    op.gm = gm;
    op.gmbs = gmb + gcb;
    op.gme = gme;

    // Bias-dependent gate resistance (Xyce/N_DEV_MOSFET_B3SOI.C, RF `Rg`
    // block). For `RGATEMOD=2`, `gcrg` is the combined gate-electrode and
    // intrinsic input conductance; derivatives remain in device polarity and
    // are assembled by the device stamp with the mode swap.
    if m.rgate_mod == 2 {
        let t9 = p.xrcrg2 * vtm;
        let t0 = t9 * beta;
        let dt0_dvd = (dbeta_dvd + dbeta_dvg * dvgsteff_dvd) * t9;
        let dt0_dvb = (dbeta_dvb + dbeta_dvg * dvgsteff_dvb) * t9;
        let dt0_dvg = dbeta_dvg * t9;

        op.gcrg = p.xrcrg1 * (t0 + ids);
        op.gcrgd = p.xrcrg1 * (dt0_dvd + gds0);
        op.gcrgb = p.xrcrg1 * (dt0_dvb + gmb0) * dvbseff_dvb;
        op.gcrgg = p.xrcrg1 * (dt0_dvg + gm0) * dvgsteff_dvg;

        let denom = p.grgeltd + op.gcrg;
        if denom != 0.0 {
            let scale = p.grgeltd * p.grgeltd / (denom * denom);
            op.gcrg = p.grgeltd * op.gcrg / denom;
            op.gcrgg *= scale;
            op.gcrgd *= scale;
            op.gcrgb *= scale;
        } else {
            op.gcrg = 0.0;
            op.gcrgg = 0.0;
            op.gcrgd = 0.0;
            op.gcrgb = 0.0;
        }
        op.gcrgs = -(op.gcrgg + op.gcrgd + op.gcrgb);
    }

    // Drain-side junction current into drain prime.
    op.gjdb = gjdb - giib;
    op.gjdd = gjdd - (giid + gdgidld);
    op.gjdg = -(giig + gdgidlg);
    op.gjde = -giie;
    op.cjd = ibd
        - iii
        - idgidl
        - min_isub / 2.0
        - (op.gjdb * vbs + op.gjdd * vds + op.gjdg * vgs + op.gjde * ves);

    // Source-side junction current into source prime.
    op.gjsb = gjsb;
    op.gjsd = gjsd;
    op.gjsg = -gsgidlg;
    op.cjs = ibs - isgidl - min_isub / 2.0 - (op.gjsb * vbs + op.gjsd * vds + op.gjsg * vgs);

    // Body-node KCL.
    op.gbbs = giib - gjsb - gjdb;
    op.gbgs = giig + gdgidlg + gsgidlg;
    op.gbds = giid + gdgidld - gjsd - gjdd;
    op.gbes = giie;
    op.gbps = 0.0;
    op.cbody = iii + idgidl + isgidl - ibs - ibd + min_isub
        - (op.gbbs * vbs + op.gbgs * vgs + op.gbds * vds + op.gbes * ves);

    // Thermal power row. ngspice intentionally omits the bipolar-current power
    // term here for convergence (b3soiddld.c operating-point assignment).
    op.gtemp_g = -gm * vds;
    op.gtemp_b = -gmb * vds;
    op.gtemp_e = -gme * vds;
    op.gtemp_d = -gds * vds - ids;
    op.gtemp_t = 0.0;
    op.thermal_eq_current = -ids * vds
        - mtype * (op.gtemp_g * vgs + op.gtemp_b * vbs + op.gtemp_e * ves + op.gtemp_d * vds)
        - op.gtemp_t * bias.del_temp;

    // qinv for noise.
    let t1q = vgsteff * (1.0 - 0.5 * abeff * vdseff / vgst2vtm);
    op.qinv = -m.cox * p.weff * leff * t1q;

    if compute_charges {
        op.charge = Some(eval_charges_capmod2_or_3(
            p,
            m,
            mtype,
            mode,
            &ChargeInputs {
                selfheat,
                phi,
                k1,
                cbox,
                vgs_eff,
                dvgs_eff_dvg,
                vth,
                dvth_dvb,
                dvth_dvd,
                dvth_dt,
                dvfbb_dt: sh.dvfbb_dt,
                vgst,
                vgsteff,
                dvgsteff_dvg,
                dvgsteff_dvd,
                dvgsteff_dvb,
                dvgsteff_dve,
                dvgsteff_dt,
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
                dvbseff_dt,
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
                vbs0t: charge_vbs0t,
                vbs0: charge_vbs0,
                dvbs0_dve: charge_dvbs0_dve,
                vbs0mos: charge_vbs0mos,
                dvbs0mos_dve: charge_dvbs0mos_dve,
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
                ibsdif: body.storage_ibs,
                dibsdif_dvb: body.storage_dibs_dvb,
                dibsdif_dt: body.storage_dibs_dt,
                ibddif: body.storage_ibd,
                dibddif_dvb: body.storage_dibd_dvb,
                dibddif_dvd: body.storage_dibd_dvd,
                dibddif_dt: body.storage_dibd_dt,
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
    pub cap_mod: i32,
    pub rgate_mod: i32,
    pub dialect: B3SoiDialect,
    pub k1b: Value,
    pub k2b: Value,
    pub dk2b: Value,
    pub nofffd: Value,
    pub vofffd: Value,
    pub moin_fd: Value,
    pub cox: Value,
    pub cbox: Value,
    pub csi: Value,
    pub csieff: Value,
    pub qsi: Value,
    pub qsieff: Value,
    pub adice: Value,
    pub tox: Value,
    pub tsi: Value,
    /// BJT temperature exponent `XBJT`, used by Xyce self-heating charge derivatives.
    pub xbjt: Value,
    /// Diffusion-current temperature exponent `XDIF`.
    pub xdif: Value,
    pub xj: Value,
    pub charge_q: Value,
    pub mob_mod: i32,

    // CAPMOD=2/3 charge-model model-card scalars (b3soiddld.c CV block).
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
    /// Temperature coefficient of `PBSWG`.
    pub tpbswg: Value,
    /// Temperature coefficient of `CJSWG`.
    pub tcjswg: Value,
    /// Device polarity (`+1` NMOS / `-1` PMOS) for the extrinsic charge sign.
    pub mtype: Value,
}

#[inline]
fn temp_ratio_m1(p: &B3SoiDdSized) -> Value {
    p.temp / p.tnom - 1.0
}

#[derive(Debug, Clone, Copy, Default)]
struct SelfHeatDerivs {
    dvbi_dt: Value,
    dvfbb_dt: Value,
    dahli_dt: Value,
    djbjt_dt: Value,
    djdif_dt: Value,
}

#[derive(Debug, Clone, Copy)]
struct BodyCurrents {
    ic: Value,
    gcd: Value,
    gcb: Value,
    ibs: Value,
    ibd: Value,
    gjsb: Value,
    gjsd: Value,
    gjdb: Value,
    gjdd: Value,
    storage_ibs: Value,
    storage_dibs_dvb: Value,
    storage_dibs_dt: Value,
    storage_ibd: Value,
    storage_dibd_dvb: Value,
    storage_dibd_dvd: Value,
    storage_dibd_dt: Value,
}

#[inline]
fn cexp100(arg: Value) -> (Value, Value) {
    const MAX_EXPL: Value = 2.688_117_142e43;
    const MIN_EXPL: Value = 3.720_075_976e-44;
    const EXPL_THRESHOLD: Value = 100.0;

    if arg > EXPL_THRESHOLD {
        (MAX_EXPL * (1.0 + arg - EXPL_THRESHOLD), MAX_EXPL)
    } else if arg < -EXPL_THRESHOLD {
        (MIN_EXPL, 0.0)
    } else {
        let exp_arg = arg.exp();
        (exp_arg, exp_arg)
    }
}

fn self_heat_derivs(p: &B3SoiDdSized, m: &ModelConsts, selfheat: bool) -> SelfHeatDerivs {
    if !selfheat {
        return SelfHeatDerivs::default();
    }

    let temp = p.temp;
    let vtm = p.vtm;
    let eg = 1.16 - 7.02e-4 * temp * temp / (1108.0 + temp);
    let deg_dt = ((7.02e-4 * temp * temp) - (1108.0 + temp) * (14.04e-4 * temp))
        / ((1108.0 + temp) * (1108.0 + temp));
    let ni_prefactor = 1.45e10 * temp * temp.sqrt() * 1.923_058_4e-4;
    let ni_exp = (21.556_598_1 - eg / (2.0 * vtm)).exp();
    let ni = ni_prefactor * ni_exp;
    let dni_dt = 2.175e10 * 1.923_058_4e-4 * temp.sqrt() * ni_exp
        + ni_prefactor * ni_exp * (-vtm * deg_dt + eg * KB_OVER_Q) / (2.0 * vtm * vtm);

    let vbi_log = (1.0e20 * p.npeak / (ni * ni)).ln();
    let dvbi_dt = KB_OVER_Q * vbi_log + vtm * (-2.0 * dni_dt / ni);
    let dvfbb_dt = if p.nsub > 0.0 {
        -m.mtype * KB_OVER_Q * (p.npeak / p.nsub).ln()
    } else {
        let vfbb_log = (-p.npeak * p.nsub / (ni * ni)).ln();
        -m.mtype * (KB_OVER_Q * vfbb_log - vtm * 2.0 * dni_dt / ni)
    };

    let temp_ratio = temp / p.tnom - 1.0;
    let inv_tnom = 1.0 / p.tnom;
    let t4 = EG300 / vtm * temp_ratio;
    let dt4_dt = EG300 / (vtm * vtm) * (vtm * inv_tnom - temp_ratio * KB_OVER_Q);
    let xbjt_arg = m.xbjt * t4 / p.ndiode;
    let (_, dbjt_scale_darg) = cexp100(xbjt_arg);
    let dbjt_scale_dt = dbjt_scale_darg * m.xbjt * dt4_dt / p.ndiode;
    let xdif_arg = m.xdif * t4 / p.ndiode;
    let (_, ddif_scale_darg) = cexp100(xdif_arg);
    let ddif_scale_dt = ddif_scale_darg * m.xdif * dt4_dt / p.ndiode;

    SelfHeatDerivs {
        dvbi_dt,
        dvfbb_dt,
        dahli_dt: p.ahli * dbjt_scale_dt,
        djbjt_dt: p.isbjt * dbjt_scale_dt,
        djdif_dt: p.isdif * ddif_scale_dt,
    }
}

#[inline]
fn gate_sidewall_junction_charge_params(
    p: &B3SoiDdSized,
    m: &ModelConsts,
    width_cv: Value,
) -> (Value, Value) {
    let temp_delta = p.temp - p.tnom;
    let phi_bswg = m.phibswg - m.tpbswg * temp_delta;
    let cjs = m.cjswg * width_cv * m.tsi / 1e-7;
    (phi_bswg, cjs * (1.0 + m.tcjswg * temp_delta))
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

/// Intermediates from the DC path that the CAPMOD=2/3 charge model consumes.
///
/// Every field maps 1:1 to the like-named ngspice local at the point the charge
/// block runs (`raw` suffixes are the pre-mode-swap node-frame voltages ngspice
/// uses for the overlap/extrinsic lumps; everything else is the mode-swapped
/// evaluation frame). Grouped into a struct to keep [`eval`]'s call site legible.
#[derive(Debug, Clone, Copy)]
struct ChargeInputs {
    selfheat: bool,
    phi: Value,
    k1: Value,
    cbox: Value,
    vgs_eff: Value,
    dvgs_eff_dvg: Value,
    vth: Value,
    dvth_dvb: Value,
    dvth_dvd: Value,
    dvth_dt: Value,
    dvfbb_dt: Value,
    vgst: Value,
    vgsteff: Value,
    dvgsteff_dvg: Value,
    dvgsteff_dvd: Value,
    dvgsteff_dvb: Value,
    dvgsteff_dve: Value,
    dvgsteff_dt: Value,
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
    dvbseff_dt: Value,
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
    ibsdif: Value,
    dibsdif_dvb: Value,
    dibsdif_dt: Value,
    ibddif: Value,
    dibddif_dvb: Value,
    dibddif_dvd: Value,
    dibddif_dt: Value,
    vgs_raw: Value,
    vgd_raw: Value,
    vge_raw: Value,
    vds_raw: Value,
    ves_raw: Value,
}

/// CAPMOD=2/3 charge model + extrinsic/overlap charges (b3soiddld.c:2646-3784).
///
/// Faithful transcription of the `capMod == 3` branch, the common capMod 2/3
/// backgate/inversion-charge code, the intrinsic S/D junction charge, the
/// extrinsic bottom-S/D-to-substrate spline, and the gate overlap charges, all
/// for `selfheat == 0` and the card's `xpart`. Returns the electrical node
/// charges and the intrinsic+overlap capacitance matrix (pre-`ag0`). `_mtype` is
/// accepted for symmetry with the DC eval; polarity is read from `m.mtype`.
#[allow(clippy::too_many_lines)]
fn eval_charges_capmod2_or_3(
    p: &B3SoiDdSized,
    m: &ModelConsts,
    _mtype: Value,
    mode: i32,
    i: &ChargeInputs,
) -> B3SoiDdCharge {
    if m.cap_mod == 2 && m.dialect == B3SoiDialect::Xyce {
        return eval_charges_capmod2_xyce(p, m, mode, i);
    }

    use super::super::common::{CONST_2OV3, DELTA_1, DELTA_3, DELTA_4, DELTA_VCSCV, QEX_FACT};

    let k1 = i.k1;
    let phi = i.phi;
    let cbox = i.cbox;

    // CoxWL (b3soiddld.c:2648).
    let cox_wl = m.cox * (p.weff_cv / p.nseg) * p.leff_cv;

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

    // CV-consistent VdseffCV (b3soiddld.c:2743-2756, computed for capMod 2 and
    // 3 alike). The inversion-charge partition below requires
    // `AbulkCV*VdseffCV <= Vgsteff` so that `T1 = 12*(Vgsteff - 0.5*T0)` stays
    // strictly positive; the IV-section `Vdseff` (different Vdsat, velocity
    // saturation folded in) does not honor that bound and drives the 40/60
    // partition derivatives through a pole, producing nF-scale sign-flipped
    // capacitances on switching devices.
    let vdsat_cv_base = vgsteff / abulk_cv;
    let dvdsat_cv_common_dvg = 1.0 / abulk_cv;
    let dvdsat_cv_common_dvb = -vdsat_cv_base * dabulk_cv_dvb / abulk_cv;
    let vdsat_cv_common = vdsat_cv_base + 1.0e-5;
    // `Vds` in the CV section is the mode-folded drain-source voltage.
    let vds_mode = if mode > 0 { i.vds_raw } else { -i.vds_raw };
    let v4 = vdsat_cv_common - vds_mode - DELTA_4;
    let t0v = (v4 * v4 + 4.0 * DELTA_4 * vdsat_cv_common).sqrt();
    let vdseff_cv = vdsat_cv_common - 0.5 * (v4 + t0v);
    // dVdseffCV_dVg folds dVdsatCV_dVg = 1/AbulkCV into T3; dVdseffCV_dVb
    // re-expresses dVdsatCV_dVb = -VdsatCV*dAbulkCV_dVb/AbulkCV through T3.
    let t1v = 0.5 * (1.0 + v4 / t0v);
    let t2v = DELTA_4 / t0v;
    let t3v = (1.0 - t1v - t2v) / abulk_cv;
    let dvdseff_cv_dvg = t3v;
    let dvdseff_cv_dvd = t1v;
    let dvdseff_cv_dvb = -t3v * vdsat_cv_common * dabulk_cv_dvb;

    let (
        vds_cv,
        dvds_cv_dvg,
        dvds_cv_dvd,
        dvds_cv_dvb,
        dvds_cv_dvc,
        vcs_cv,
        dvcs_cv_dvg,
        dvcs_cv_dvd,
        dvcs_cv_dvb,
        dvcs_cv_dvc,
        xc,
        dxc_dvg,
        dxc_dvd,
        dxc_dvb,
        dxc_dvc,
        qbf,
        dqbf_dvrg,
        dqbf_dvg,
        dqbf_dvd,
        dqbf_dvb,
        dqbf_dvc,
        dqbf_dve,
    ) = if m.cap_mod == 2 {
        // B3SOIDD CAPMOD=2 front-gate depletion/body-charge branch
        // (b3soiddld.c:2758-2889). DD keeps its own Xc/Qsubs/Qbf equations;
        // only the common tail below is shared with CAPMOD=3.
        let mut vds_cv = vdseff_cv;
        let dvds_cv_dvg = dvdseff_cv_dvg;
        let dvds_cv_dvd = dvdseff_cv_dvd;
        let dvds_cv_dvb = dvdseff_cv_dvb;
        let dvds_cv_dvc = 0.0;

        vds_cv += 1.0e-5;
        if vds_cv > vdsat_cv_common - 1.0e-7 {
            vds_cv = vdsat_cv_common - 1.0e-7;
        }

        let t1 = vds_cv - i.vcs - vds_cv * vds_cv * DELTA_VCSCV;
        let t5 = 2.0 * DELTA_VCSCV;
        let t2 = (t1 * t1 + t5 * vds_cv * vds_cv).sqrt();
        let dt1_dvb = dvds_cv_dvb * (1.0 - 2.0 * vds_cv * DELTA_VCSCV);
        let dt2_dvb = (t1 * dt1_dvb + t5 * vds_cv * dvds_cv_dvb) / t2;
        let dt1_dvd = dvds_cv_dvd * (1.0 - 2.0 * vds_cv * DELTA_VCSCV);
        let dt2_dvd = (t1 * dt1_dvd + t5 * vds_cv * dvds_cv_dvd) / t2;
        let dt1_dvg = dvds_cv_dvg * (1.0 - 2.0 * vds_cv * DELTA_VCSCV);
        let dt2_dvg = (t1 * dt1_dvg + t5 * vds_cv * dvds_cv_dvg) / t2;
        let dt1_dvc = -1.0;
        let dt2_dvc = t1 * dt1_dvc / t2;
        let mut vcs_cv = i.vcs + 0.5 * (t1 - t2);
        let dvcs_cv_dvb = 0.5 * (dt1_dvb - dt2_dvb);
        let dvcs_cv_dvg = 0.5 * (dt1_dvg - dt2_dvg);
        let dvcs_cv_dvd = 0.5 * (dt1_dvd - dt2_dvd);
        let dvcs_cv_dvc = 1.0 + 0.5 * (dt1_dvc - dt2_dvc);
        if vcs_cv < 0.0 {
            vcs_cv = 0.0;
        } else if vcs_cv > vds_cv {
            vcs_cv = vds_cv;
        }

        let t3 = 2.0 * vdsat_cv_common - vcs_cv;
        let t4 = 2.0 * vdsat_cv_common - vds_cv;
        let dt4_dvg = 2.0 * dvdsat_cv_common_dvg - dvds_cv_dvg;
        let dt4_dvd = -dvds_cv_dvd;
        let dt4_dvb = 2.0 * dvdsat_cv_common_dvb - dvds_cv_dvb;
        let t0 = t3 * vcs_cv;
        let t1 = t4 * vds_cv;
        let xc = t0 / t1;

        let dt0_dvb = vcs_cv * (2.0 * dvdsat_cv_common_dvb - dvcs_cv_dvb) + t3 * dvcs_cv_dvb;
        let dt0_dvg = vcs_cv * (2.0 * dvdsat_cv_common_dvg - dvcs_cv_dvg) + t3 * dvcs_cv_dvg;
        let dt0_dvd = 2.0 * dvcs_cv_dvd * (vdsat_cv_common - vcs_cv);
        let dt0_dvc = 2.0 * dvcs_cv_dvc * (vdsat_cv_common - vcs_cv);

        let dt1_dvb = vds_cv * dt4_dvb + t4 * dvds_cv_dvb;
        let dt1_dvg = vds_cv * dt4_dvg + t4 * dvds_cv_dvg;
        let dt1_dvd = dvds_cv_dvd * t4 + vds_cv * dt4_dvd;

        let dxc_dvb = (dt0_dvb - dt1_dvb * xc) / t1;
        let dxc_dvg = (dt0_dvg - dt1_dvg * xc) / t1;
        let dxc_dvd = (dt0_dvd - dt1_dvd * xc) / t1;
        let dxc_dvc = dt0_dvc / t1;

        let t0 = abulk_cv * vcs_cv;
        let dt0_dvb = dabulk_cv_dvb * vcs_cv + dvcs_cv_dvb * abulk_cv;
        let dt0_dvg = dvcs_cv_dvg * abulk_cv;
        let dt0_dvd = abulk_cv * dvcs_cv_dvd;
        let dt0_dvc = abulk_cv * dvcs_cv_dvc;

        let t1 = 12.0 * (vgsteff - 0.5 * t0 + 1.0e-20);
        let dt1_dvb = -6.0 * dt0_dvb;
        let dt1_dvg = 12.0 * (1.0 - 0.5 * dt0_dvg);
        let dt1_dvd = -6.0 * dt0_dvd;
        let dt1_dvc = -6.0 * dt0_dvc;

        let t2 = vcs_cv / t1;
        let t4 = t1 * t1;
        let dt2_dvb = (dvcs_cv_dvb * t1 - dt1_dvb * vcs_cv) / t4;
        let dt2_dvg = (dvcs_cv_dvg * t1 - dt1_dvg * vcs_cv) / t4;
        let dt2_dvd = (dvcs_cv_dvd * t1 - dt1_dvd * vcs_cv) / t4;
        let dt2_dvc = (dvcs_cv_dvc * t1 - dt1_dvc * vcs_cv) / t4;

        let t3 = t0 * t2;
        let dt3_dvb = dt0_dvb * t2 + dt2_dvb * t0;
        let dt3_dvg = dt0_dvg * t2 + dt2_dvg * t0;
        let dt3_dvd = dt0_dvd * t2 + dt2_dvd * t0;
        let dt3_dvc = dt0_dvc * t2 + dt2_dvc * t0;

        let t4 = 1.0 - abulk_cv;
        let dt4_dvb = -dabulk_cv_dvb;

        let t5 = 0.5 * vcs_cv - t3;
        let dt5_dvb = 0.5 * dvcs_cv_dvb - dt3_dvb;
        let dt5_dvg = 0.5 * dvcs_cv_dvg - dt3_dvg;
        let dt5_dvd = 0.5 * dvcs_cv_dvd - dt3_dvd;
        let dt5_dvc = 0.5 * dvcs_cv_dvc - dt3_dvc;

        let t6 = t4 * t5 * cox_wl;
        let t7 = cox_wl * xc;
        let qsubs1 = cox_wl * xc * t4 * t5;
        let dqsubs1_dvb = t6 * dxc_dvb + t7 * (t4 * dt5_dvb + dt4_dvb * t5);
        let dqsubs1_dvg = t6 * dxc_dvg + t7 * t4 * dt5_dvg;
        let dqsubs1_dvd = t6 * dxc_dvd + t7 * t4 * dt5_dvd;
        let dqsubs1_dvc = t6 * dxc_dvc + t7 * t4 * dt5_dvc;

        let qsubs2 = -cox_wl * (1.0 - xc) * (abulk_cv - 1.0) * i.vcs;
        let t2 = cox_wl * (abulk_cv - 1.0) * i.vcs;
        let dqsubs2_dvb = t2 * dxc_dvb - cox_wl * (1.0 - xc) * i.vcs * dabulk_cv_dvb;
        let dqsubs2_dvg = t2 * dxc_dvg;
        let dqsubs2_dvd = t2 * dxc_dvd;
        let dqsubs2_dvc = t2 * dxc_dvc - cox_wl * (1.0 - xc) * (abulk_cv - 1.0);

        let qbf = qac0 + qsub0 + qsubs1 + qsubs2;
        let dqbf_dvrg = dqac0_dvrg + dqsub0_dvrg;
        let dqbf_dvg = dqsub0_dvg + dqsubs1_dvg + dqsubs2_dvg;
        let dqbf_dvd = dqac0_dvd + dqsub0_dvd + dqsubs1_dvd + dqsubs2_dvd;
        let dqbf_dvb = dqac0_dvb + dqsub0_dvb + dqsubs1_dvb + dqsubs2_dvb;
        let dqbf_dvc = dqsubs1_dvc + dqsubs2_dvc;
        let dqbf_dve = 0.0;

        (
            vds_cv,
            dvds_cv_dvg,
            dvds_cv_dvd,
            dvds_cv_dvb,
            dvds_cv_dvc,
            vcs_cv,
            dvcs_cv_dvg,
            dvcs_cv_dvd,
            dvcs_cv_dvb,
            dvcs_cv_dvc,
            xc,
            dxc_dvg,
            dxc_dvd,
            dxc_dvb,
            dxc_dvc,
            qbf,
            dqbf_dvrg,
            dqbf_dvg,
            dqbf_dvd,
            dqbf_dvb,
            dqbf_dvc,
            dqbf_dve,
        )
    } else {
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
                dvds_cv_dvd = t3 * i.dvdsat_dvd
                    + i.vdsat * dt3_dvd
                    + t3 * (2.0 * t1 * dt3_dvd + t3 * dt1_dvd);
                dvds_cv_dvg = t3 * i.dvdsat_dvg
                    + i.vdsat * dt3_dvg
                    + t3 * (2.0 * t1 * dt3_dvg + t3 * dt1_dvg);
                dvds_cv_dvb = t3 * i.dvdsat_dvb
                    + i.vdsat * dt3_dvb
                    + t3 * (2.0 * t1 * dt3_dvb + t3 * dt1_dvb);
                dvds_cv_dvc = t3 * i.dvdsat_dvc
                    + i.vdsat * dt3_dvc
                    + t3 * (2.0 * t1 * dt3_dvc + t3 * dt1_dvc);
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
        let dnomi_dvb =
            k1 * (CONST_2OV3 * (t1n * dt0_dvb + t0n * dt1_dvb) - 0.4 * dt2_dvb - dt3_dvb);
        let dnomi_dvd =
            k1 * (CONST_2OV3 * (t1n * dt0_dvd + t0n * dt1_dvd) - 0.4 * dt2_dvd - dt3_dvd);
        let dnomi_dvg =
            k1 * (CONST_2OV3 * (t1n * dt0_dvg + t0n * dt1_dvg) - 0.4 * dt2_dvg - dt3_dvg);
        let dnomi_dvc =
            k1 * (CONST_2OV3 * (t1n * dt0_dvc + t0n * dt1_dvc) - 0.4 * dt2_dvc - dt3_dvc);

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
        (
            vds_cv,
            dvds_cv_dvg,
            dvds_cv_dvd,
            dvds_cv_dvb,
            dvds_cv_dvc,
            vcs_cv,
            dvcs_cv_dvg,
            dvcs_cv_dvd,
            dvcs_cv_dvb,
            dvcs_cv_dvc,
            xc,
            dxc_dvg,
            dxc_dvd,
            dxc_dvb,
            dxc_dvc,
            qbf,
            dqbf_dvrg,
            dqbf_dvg,
            dqbf_dvd,
            dqbf_dvb,
            dqbf_dvc,
            dqbf_dve,
        )
    };

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
    let dqe1_dvd = t5
        * (dxc_dvg * dvgsteff_dvd + dxc_dvb * i.dvbseff_dvd + dxc_dvc * i.dvcs_dvd + dxc_dvd)
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
    let dqe2_dvd = t10g * dvgsteff_dvd
        + t11g * i.dvbseff_dvd
        + t12g * i.dvcs_dvd
        + t3 * (dvds_cv_dvd - dvcs_cv_dvd)
        - t4 * dxc_dvd;
    let dqe2_dve = t10g * dvgsteff_dve + t11g * i.dvbseff_dve + t12g * i.dvcs_dve;

    // Transform Qbf dependency on (Vgsteff,Vbseff,Vcs) into node ones
    // (b3soiddld.c:3288-3311).
    let cbg =
        dqbf_dvrg + dqbf_dvg * dvgsteff_dvg + dqbf_dvb * i.dvbseff_dvg + dqbf_dvc * i.dvcs_dvg;
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

    // Total inversion charge (b3soiddld.c:3313-3326).
    let t0 = abulk_cv * vdseff_cv;
    let t1 = 12.0 * (vgsteff - 0.5 * t0 + 1e-20);
    let t2 = vdseff_cv / t1;
    let t3 = t0 * t2;
    let t4 = 1.0 - 12.0 * t2 * t2 * abulk_cv;
    let t5 = 6.0 * t0 * (4.0 * vgsteff - t0) / (t1 * t1) - 0.5;
    let t6 = 12.0 * t2 * t2 * vgsteff;
    let qinv = cox_wl * (vgsteff - 0.5 * vdseff_cv + t3);
    let cgg1 = cox_wl * (t4 + t5 * dvdseff_cv_dvg);
    let cgd1 = cox_wl * t5 * dvdseff_cv_dvd;
    let cgb1 = cox_wl * (t5 * dvdseff_cv_dvb + t6 * dabulk_cv_dvb);

    // Charge partition into S (b3soiddld.c:3329-3368).
    let (qsrc, csg1, csd1, csb1);
    if m.xpart > 0.5 {
        let t1p = t1 + t1;
        qsrc = -cox_wl * (0.5 * vgsteff + 0.25 * t0 - t0 * t0 / t1p);
        let t7 = (4.0 * vgsteff - t0) / (t1p * t1p);
        let t4p = -(0.5 + 24.0 * t0 * t0 / (t1p * t1p));
        let t5p = -(0.25 * abulk_cv - 12.0 * abulk_cv * t0 * t7);
        let t6p = -(0.25 * vdseff_cv - 12.0 * t0 * vdseff_cv * t7);
        csg1 = cox_wl * (t4p + t5p * dvdseff_cv_dvg);
        csd1 = cox_wl * t5p * dvdseff_cv_dvd;
        csb1 = cox_wl * (t5p * dvdseff_cv_dvb + t6p * dabulk_cv_dvb);
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
        csg1 = t4p + t5p * dvdseff_cv_dvg;
        csd1 = t5p * dvdseff_cv_dvd;
        csb1 = t5p * dvdseff_cv_dvb + t6p * dabulk_cv_dvb;
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
    let (phi_bswg, cjsbs) = gate_sidewall_junction_charge_params(p, m, p.weff_cv / p.nseg);
    let mjswg = m.mjswg;

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
    let qjs = cjsbs * t3 + m.tt * i.ibsdif;
    let gcjsbs = cjsbs * dt3_dvb_s + m.tt * i.dibsdif_dvb;

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
    let qjd = cjsbs * t3 + m.tt * i.ibddif;
    let gcjdbs = cjsbs * dt3_dvb_d + m.tt * i.dibddif_dvb;
    let gcjdds = cjsbs * dt3_dvd_d + m.tt * i.dibddif_dvd;

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
            qth: 0.0,
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
            gcg_t: 0.0,
            gcb_t: 0.0,
            gcd_t: 0.0,
            gce_t: 0.0,
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
            qth: 0.0,
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
            gcg_t: 0.0,
            gcb_t: 0.0,
            gcd_t: 0.0,
            gce_t: 0.0,
        }
    }
}

#[allow(clippy::too_many_lines)]
fn eval_charges_capmod2_xyce(
    p: &B3SoiDdSized,
    m: &ModelConsts,
    mode: i32,
    i: &ChargeInputs,
) -> B3SoiDdCharge {
    use super::super::common::{DELTA_1, DELTA_4};

    const DELTA_3_SOI: Value = 0.08;

    let k1 = i.k1;
    let phi = i.phi;
    let cox_wl = m.cox * (p.weff_cv / p.nseg) * p.leff_cv;
    let cox_wlb = p.fbody * m.cox * (p.weff_cv / p.nseg) * p.leff_cv_b;

    let (mut vgsteff, mut dvgsteff_dvg, mut dvgsteff_dvd, mut dvgsteff_dvb, mut dvgsteff_dt) = (
        i.vgsteff,
        i.dvgsteff_dvg,
        i.dvgsteff_dvd,
        i.dvgsteff_dvb,
        i.dvgsteff_dt,
    );
    if i.vgst_n_vt > -EXP_THRESHOLD && i.vgst_n_vt < EXP_THRESHOLD {
        let noff = i.n * p.noff;
        let dnoff_dvd = p.noff * i.dn_dvd;
        let dnoff_dvb = p.noff * i.dn_dvb;
        let exp_vgst = i.exp_vgst * i.exp_vgst * (-(p.delvt / (noff * p.vtm))).exp();
        vgsteff = noff * p.vtm * (1.0 + exp_vgst).ln();
        let t0 = exp_vgst / (1.0 + exp_vgst);
        let t1 =
            -t0 * (i.dvth_dvb + (i.vgst - p.delvt) / noff * dnoff_dvb) + vgsteff / noff * dnoff_dvb;
        dvgsteff_dvd = -t0 * (i.dvth_dvd + i.dvth_dvb * i.dvbseff_dvd + i.vgst / noff * dnoff_dvd)
            + vgsteff / noff * dnoff_dvd;
        dvgsteff_dvg = t0 * (i.dvgs_eff_dvg - i.dvth_dvb * i.dvbseff_dvg);
        dvgsteff_dvb = t1 * i.dvbseff_dvb;
        dvgsteff_dt = if i.selfheat {
            -t0 * (i.dvth_dt + i.dvth_dvb * i.dvbseff_dt + (i.vgst - p.delvt) / p.temp)
                + vgsteff / p.temp
        } else {
            0.0
        };
    }

    let sqrt_phis = i.sqrt_phis;
    let dsqrt_phis_dvb = i.dsqrt_phis_dvb;
    let vfb = i.vth - phi - k1 * sqrt_phis + p.delvt;
    let dvfb_dvb = i.dvth_dvb - k1 * dsqrt_phis_dvb;
    let dvfb_dvd = i.dvth_dvd;
    let dvfb_dt = if i.selfheat { i.dvth_dt } else { 0.0 };

    let v3 = vfb - i.vgs_eff + i.vbseff - DELTA_3_SOI;
    let (t0fb, t2fb);
    if vfb <= 0.0 {
        t0fb = (v3 * v3 - 4.0 * DELTA_3_SOI * vfb).sqrt();
        t2fb = -DELTA_3_SOI / t0fb;
    } else {
        t0fb = (v3 * v3 + 4.0 * DELTA_3_SOI * vfb).sqrt();
        t2fb = DELTA_3_SOI / t0fb;
    }
    let t1fb = 0.5 * (1.0 + v3 / t0fb);
    let vfbeff = vfb - 0.5 * (v3 + t0fb);
    let dvfbeff_dvd = (1.0 - t1fb - t2fb) * dvfb_dvd;
    let dvfbeff_dvb = (1.0 - t1fb - t2fb) * dvfb_dvb - t1fb;
    let dvfbeff_dvrg = t1fb * i.dvgs_eff_dvg;
    let dvfbeff_dt = if i.selfheat {
        (1.0 - t1fb - t2fb) * dvfb_dt
    } else {
        0.0
    };

    let qac0 = cox_wlb * (vfbeff - vfb);
    let dqac0_dvrg = cox_wlb * dvfbeff_dvrg;
    let dqac0_dvd = cox_wlb * (dvfbeff_dvd - dvfb_dvd);
    let dqac0_dvb = cox_wlb * (dvfbeff_dvb - dvfb_dvb);
    let dqac0_dt = if i.selfheat {
        cox_wlb * (dvfbeff_dt - dvfb_dt)
    } else {
        0.0
    };

    let t0 = 0.5 * k1;
    let t3 = i.vgs_eff - vfbeff - i.vbseff - vgsteff;
    let (t1s, t2s);
    if k1 == 0.0 {
        t1s = 0.0;
        t2s = 0.0;
    } else if t3 < 0.0 {
        t1s = t0 + t3 / k1;
        t2s = cox_wlb;
    } else {
        t1s = (t0 * t0 + t3).sqrt();
        t2s = cox_wlb * t0 / t1s;
    }
    let qsub0 = cox_wlb * k1 * (t1s - t0);
    let dqsub0_dvrg = t2s * (i.dvgs_eff_dvg - dvfbeff_dvrg);
    let dqsub0_dvg = -t2s;
    let dqsub0_dvd = -t2s * dvfbeff_dvd;
    let dqsub0_dvb = -t2s * (dvfbeff_dvb + 1.0);
    let dqsub0_dt = if i.selfheat { -t2s * dvfbeff_dt } else { 0.0 };

    let abulk_cv = i.abulk0 * p.abulk_cv_factor;
    let dabulk_cv_dvb = p.abulk_cv_factor * i.dabulk0_dvb;
    let vdsat_cv = vgsteff / abulk_cv;
    let vds_mode = if mode > 0 { i.vds_raw } else { -i.vds_raw };
    let v4 = vdsat_cv - vds_mode - DELTA_4;
    let t0v = (v4 * v4 + 4.0 * DELTA_4 * vdsat_cv).sqrt();
    let vdseff_cv = vdsat_cv - 0.5 * (v4 + t0v);
    let t1v = 0.5 * (1.0 + v4 / t0v);
    let t2v = DELTA_4 / t0v;
    let t3v = (1.0 - t1v - t2v) / abulk_cv;
    let dvdseff_cv_dvg = t3v;
    let dvdseff_cv_dvd = t1v;
    let dvdseff_cv_dvb = -t3v * vdsat_cv * dabulk_cv_dvb;

    let t0 = abulk_cv * vdseff_cv;
    let t1 = 12.0 * (vgsteff - 0.5 * t0 + 1e-20);
    let t2 = vdseff_cv / t1;
    let t3 = t0 * t2;
    let t4 = 1.0 - 12.0 * t2 * t2 * abulk_cv;
    let t5 = 6.0 * t0 * (4.0 * vgsteff - t0) / (t1 * t1) - 0.5;
    let t6 = 12.0 * t2 * t2 * vgsteff;
    let t7 = 1.0 - abulk_cv;
    let qbulk = cox_wlb * t7 * (0.5 * vdseff_cv - t3);
    let t4b = -t7 * (t4 - 1.0);
    let t5b = -t7 * t5;
    let t6b = -(t7 * t6 + (0.5 * vdseff_cv - t3));
    let cbg1 = cox_wlb * (t4b + t5b * dvdseff_cv_dvg);
    let cbd1 = cox_wlb * t5b * dvdseff_cv_dvd;
    let cbb1 = cox_wlb * (t5b * dvdseff_cv_dvb + t6b * dabulk_cv_dvb);

    let t2_inv = t0 / t1;
    let t3_inv = t0 * t2_inv;
    let t4_inv = 1.0 - 12.0 * t2_inv * t2_inv;
    let t7_inv = t2_inv * (2.0 + 6.0 * t2_inv) - 0.5;
    let t5_inv = t7_inv * abulk_cv;
    let t6_inv = t7_inv * vdseff_cv;
    let qinv = cox_wl * (vgsteff - 0.5 * t0 + t3_inv);
    let cgg1 = cox_wl * (t4_inv + t5_inv * dvdseff_cv_dvg);
    let cgd1 = cox_wl * t5_inv * dvdseff_cv_dvd;
    let cgb1 = cox_wl * (t5_inv * dvdseff_cv_dvb + t6_inv * dabulk_cv_dvb);

    let (qsrc, csg1, csd1, csb1);
    if m.xpart > 0.5 {
        let t1p = t1 + t1;
        qsrc = -cox_wl * (0.5 * vgsteff + 0.25 * t0 - t0 * t0 / t1p);
        let t7p = (4.0 * vgsteff - t0) / (t1p * t1p);
        let t4p = -(0.5 + 24.0 * t0 * t0 / (t1p * t1p));
        let t5p = -(0.25 * abulk_cv - 12.0 * abulk_cv * t0 * t7p);
        let t6p = -(0.25 * vdseff_cv - 12.0 * t0 * vdseff_cv * t7p);
        csg1 = cox_wl * (t4p + t5p * dvdseff_cv_dvg);
        csd1 = cox_wl * t5p * dvdseff_cv_dvd;
        csb1 = cox_wl * (t5p * dvdseff_cv_dvb + t6p * dabulk_cv_dvb);
    } else if m.xpart < 0.5 {
        let t1p = t1 / 12.0;
        let t2p = 0.5 * cox_wl / (t1p * t1p);
        let t3p = vgsteff * (2.0 * t0 * t0 / 3.0 + vgsteff * (vgsteff - 4.0 * t0 / 3.0))
            - 2.0 * t0 * t0 * t0 / 15.0;
        qsrc = -t2p * t3p;
        let t7p = 4.0 / 3.0 * vgsteff * (vgsteff - t0) + 0.4 * t0 * t0;
        let t4p = -2.0 * qsrc / t1p
            - t2p * (vgsteff * (3.0 * vgsteff - 8.0 * t0 / 3.0) + 2.0 * t0 * t0 / 3.0);
        let t5p = (qsrc / t1p + t2p * t7p) * abulk_cv;
        let t6p = qsrc / t1p * vdseff_cv + t2p * t7p * vdseff_cv;
        csg1 = t4p + t5p * dvdseff_cv_dvg;
        csd1 = t5p * dvdseff_cv_dvd;
        csb1 = t5p * dvdseff_cv_dvb + t6p * dabulk_cv_dvb;
    } else {
        qsrc = -0.5 * (qinv + qbulk);
        csg1 = -0.5 * (cgg1 + cbg1);
        csb1 = -0.5 * (cgb1 + cbb1);
        csd1 = -0.5 * (cgd1 + cbd1);
    }

    let cbox_wl = p.kb1 * p.fbody * i.cbox * (p.weff_cv / p.nseg) * p.leff_cv_bg;
    let qe1 = cbox_wl * (i.ves_raw - p.vfbb - i.vbs);
    let ce1b = -cbox_wl;
    let ce1e = cbox_wl;
    let ce1_t = if i.selfheat {
        -cbox_wl * i.dvfbb_dt
    } else {
        0.0
    };

    let mut qgate = qinv + qac0 + qsub0;
    let mut qbody = qbulk - qac0 - qsub0 - qe1;
    let mut qsub = qe1;
    let mut qdrn = -(qgate + qsrc + qbody + qsub);

    let csg = csg1 * dvgsteff_dvg;
    let csd = csd1 + csg1 * dvgsteff_dvd;
    let csb = csg1 * dvgsteff_dvb + csb1 * i.dvbseff_dvb;
    let cs_t = if i.selfheat { csg1 * dvgsteff_dt } else { 0.0 };

    let cgg = (cgg1 + dqsub0_dvg) * dvgsteff_dvg + dqac0_dvrg + dqsub0_dvrg;
    let cgd = (cgg1 + dqsub0_dvg) * dvgsteff_dvd + cgd1 + dqac0_dvd + dqsub0_dvd;
    let cgb = (cgg1 + dqsub0_dvg) * dvgsteff_dvb + (cgb1 + dqsub0_dvb + dqac0_dvb) * i.dvbseff_dvb;
    let cg_t = if i.selfheat {
        (cgg1 + dqsub0_dvg) * dvgsteff_dt + dqac0_dt + dqsub0_dt
    } else {
        0.0
    };

    let cbg = (cbg1 - dqsub0_dvg) * dvgsteff_dvg - dqac0_dvrg - dqsub0_dvrg;
    let cbd = (cbg1 - dqsub0_dvg) * dvgsteff_dvd + cbd1 - dqac0_dvd - dqsub0_dvd;
    let cbb =
        (cbg1 - dqsub0_dvg) * dvgsteff_dvb - ce1b + (cbb1 - dqsub0_dvb - dqac0_dvb) * i.dvbseff_dvb;
    let mut cb_t = if i.selfheat {
        (cbg1 - dqsub0_dvg) * dvgsteff_dt - dqac0_dt - dqsub0_dt - ce1_t
    } else {
        0.0
    };

    let cggb = cgg;
    let cgsb = -(cgg + cgd + cgb);
    let cgdb = cgd;
    let cgeb = 0.0;

    let cbgb = cbg;
    let mut cbsb = -(cbg + cbd + cbb) + ce1e;
    let mut cbdb = cbd;
    let cbeb = -ce1e;

    let cegb = 0.0;
    let cedb = 0.0;
    let cesb = 0.0;
    let ceeb = ce1e;

    let cdgb = -(cgg + cbg + csg);
    let mut cddb = -(cgd + cbd + csd);
    let cdeb = 0.0;
    let mut cdsb = cgg + cgd + cgb + cbg + cbd + cbb + csg + csd + csb + ce1b;
    let mut cd_t = -(cg_t + cb_t + cs_t) - ce1_t;

    let width_cv = p.weff_cv / p.nseg;
    let temp_delta = p.temp - p.tnom;
    let phi_bswg = m.phibswg - m.tpbswg * temp_delta;
    let dphi_bswg_dt = if i.selfheat { -m.tpbswg } else { 0.0 };
    let cjs_base = m.cjswg * width_cv * m.tsi / 1e-7;
    let dcjs_dt = if i.selfheat { cjs_base * m.tcjswg } else { 0.0 };
    let cjsbs = cjs_base + cjs_base * m.tcjswg * temp_delta;
    let mjswg = m.mjswg;
    let cjdbs = cjsbs;
    let dio_max = 0.9 * phi_bswg;
    let junction_t3 = |v: Value| -> (Value, Value, Value) {
        let arg = 1.0 - v.min(dio_max) / phi_bswg;
        let darg_dt = if i.selfheat {
            (1.0 - arg) / phi_bswg * dphi_bswg_dt
        } else {
            0.0
        };
        let dt3_dvb = if mjswg == 0.5 {
            1.0 / arg.sqrt()
        } else {
            (-mjswg * arg.ln()).exp()
        };
        let ddt3_dvb_dt = if i.selfheat {
            if mjswg == 0.5 {
                -0.5 * dt3_dvb / arg * darg_dt
            } else {
                -mjswg * dt3_dvb / arg * darg_dt
            }
        } else {
            0.0
        };
        let mut t3 = (1.0 - arg * dt3_dvb) * phi_bswg / (1.0 - mjswg);
        let dt3_dt = if i.selfheat {
            (1.0 - arg * dt3_dvb) * dphi_bswg_dt / (1.0 - mjswg)
                - (arg * ddt3_dvb_dt + darg_dt * dt3_dvb) * phi_bswg / (1.0 - mjswg)
        } else {
            0.0
        };
        if v > dio_max {
            t3 += dt3_dvb * (v - dio_max);
        }
        (t3, dt3_dvb, dt3_dt)
    };

    let (t3s, dt3_dvb_s, dt3_dt_s) = junction_t3(i.vbs);
    let qjs = cjsbs * t3s + m.tt * i.ibsdif;
    let gcjsbs = cjsbs * dt3_dvb_s + m.tt * i.dibsdif_dvb;
    let gcjs_t = if i.selfheat {
        m.tt * i.dibsdif_dt + dcjs_dt * t3s + dt3_dt_s * cjsbs
    } else {
        0.0
    };

    let (t3d, dt3_dvb_d, dt3_dt_d) = junction_t3(i.vbd);
    let dt3_dvd_d = -dt3_dvb_d;
    let qjd = cjdbs * t3d + m.tt * i.ibddif;
    let gcjdbs = cjdbs * dt3_dvb_d + m.tt * i.dibddif_dvb;
    let gcjdds = cjdbs * dt3_dvd_d + m.tt * i.dibddif_dvd;
    let gcjd_t = if i.selfheat {
        m.tt * i.dibddif_dt + dcjs_dt * t3d + dt3_dt_d * cjdbs
    } else {
        0.0
    };

    qdrn -= qjd;
    qbody += qjs + qjd;

    cddb -= gcjdds;
    cd_t -= gcjd_t;
    cdsb += gcjdds + gcjdbs;
    cbdb += gcjdds;
    cb_t += gcjd_t + gcjs_t;
    cbsb -= gcjdds + gcjdbs + gcjsbs;

    let nsub_pos_type = (p.nsub > 0.0 && m.mtype > 0.0) || (p.nsub < 0.0 && m.mtype < 0.0);
    let t10 = -m.mtype * i.ves_raw;
    let (mut qse, gcse) = extrinsic_sd_charge(p, t10, nsub_pos_type, true);
    let t11 = m.mtype * (i.vds_raw - i.ves_raw);
    let (mut qde, gcde) = extrinsic_sd_charge(p, t11, nsub_pos_type, false);

    qse += p.csesw * t10;
    let gcse = gcse + p.csesw;
    qde += p.cdesw * t11;
    let gcde = gcde + p.cdesw;

    let qse = m.mtype * qse;
    let qde = m.mtype * qde;

    let t0 = i.vgd_raw + DELTA_1;
    let t1 = (t0 * t0 + 4.0 * DELTA_1).sqrt();
    let t2 = 0.5 * (t0 - t1);
    let t3v = (p.weff_cv / p.nseg) * p.cgdl;
    let t4v = (1.0 - 4.0 * t2 / p.ckappa).sqrt();
    let cgdo = p.cgdo + t3v - t3v * (1.0 - 1.0 / t4v) * (0.5 - 0.5 * t0 / t1);
    let qgdo = (p.cgdo + t3v) * i.vgd_raw - t3v * (t2 + 0.5 * p.ckappa * (t4v - 1.0));

    let t0 = i.vgs_raw + DELTA_1;
    let t1 = (t0 * t0 + 4.0 * DELTA_1).sqrt();
    let t2 = 0.5 * (t0 - t1);
    let t3v = (p.weff_cv / p.nseg) * p.cgsl;
    let t4v = (1.0 - 4.0 * t2 / p.ckappa).sqrt();
    let cgso = p.cgso + t3v - t3v * (1.0 - 1.0 / t4v) * (0.5 - 0.5 * t0 / t1);
    let qgso = (p.cgso + t3v) * i.vgs_raw - t3v * (t2 + 0.5 * p.ckappa * (t4v - 1.0));

    let cgeo = p.cgeo;
    let qge = cgeo * i.vge_raw;
    let qgd = qgdo;
    let qgs = qgso;

    let (gcg_t, gcb_t, gcd_t, gce_t) = if mode > 0 {
        (
            m.mtype * cg_t,
            m.mtype * cb_t,
            m.mtype * cd_t,
            m.mtype * ce1_t,
        )
    } else {
        (
            m.mtype * cg_t,
            m.mtype * cb_t,
            -m.mtype * (cg_t + cb_t + cd_t + ce1_t),
            m.mtype * ce1_t,
        )
    };

    if mode > 0 {
        qgate += qgd + qgs + qge;
        qdrn += qde - qgd;
        qsub -= qge + qse + qde;

        B3SoiDdCharge {
            mode,
            qg: qgate,
            qb: qbody,
            qd: qdrn,
            qe: qsub,
            qth: 0.0,
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
            gcg_t,
            gcb_t,
            gcd_t,
            gce_t,
        }
    } else {
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
            qth: 0.0,
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
            gcg_t,
            gcb_t,
            gcd_t,
            gce_t,
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
