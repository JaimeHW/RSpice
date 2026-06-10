//! B3SOIPD DC load equations (faithful port of ngspice-46 `b3soipdld.c`).
//!
//! This module transcribes the **DC current path** of `B3SOIPDload`
//! (b3soipdld.c lines ~860-2640): the SOI body-coupled threshold chain, the
//! BSIM3 `Vgsteff` smoothing, `Abulk`/`Abeff`, MOBMOD mobility, `Vdsat`, CLM /
//! DIBL / `Va`, the channel current `Ids`, and the SOI body currents (impact
//! ionization, GIDL, and the source/drain body diodes + parasitic BJT). The
//! result is the linearized operating point that ngspice stores in the
//! `here->B3SOIPD*` conductance/current fields and then stamps.
//!
//! Scope / provenance:
//! - Tested decks all use MOBMOD=0, CAPMOD=3, SHMOD=0 and either a floating
//!   body (`bodyMod=0`) or an ideal body tie (`bodyMod=2`). Accordingly:
//!   * Self-heating (`selfheat`) is **0** throughout; every `if (selfheat)`
//!     branch in the C reduces to the `else` (derivative = 0) and the temp
//!     node does not exist. The temperature-dependent quantities (jbjt, jdif,
//!     jrec, jtun, u0temp, vsattemp, rds0, ua/ub/uc, vbi, vfbb, phi, Xdep0)
//!     are taken from the precomputed [`B3SoiPdSized`] (the `else` branch of
//!     the big temp block at b3soipdld.c:803-822).
//!   * Body-resistor current `Ibp` (b3soipdld.c:2480-2540) is zero for
//!     `bodyMod` 0/2 and is therefore omitted here; the body tie is handled by
//!     the device stamping (the external body node is the body node directly).
//! - The **charge model** (CAPMOD=3, b3soipdld.c:2640-3400) and the matrix
//!   **stamping** (b3soipdld.c:3400-4460) are handled in [`super`]; this file
//!   stops at the `here->B3SOIPD*` operating-point assignment block
//!   (b3soipdld.c:2556-2640).
//!
//! Sign / mode convention matches ngspice exactly: all internal math is done in
//! the device's own polarity (`mtype` folded into the branch voltages by the
//! caller), and the normal/inverse `mode` swap (b3soipdld.c:836-860) is applied
//! to the *evaluation* voltages while the externally-meaningful currents are
//! re-expressed on the drain/source primes.

// `abulk0`/`dabulk0_dvb`/`exp_vgst` are computed in the DC path but only read
// by the (not-yet-ported) CAPMOD=3 charge model; ngspice keeps them here, so we
// retain the assignments for a faithful seam rather than dropping them.
#![allow(unused_assignments)]

use super::super::common::{EPSSI, EXPL_THRESHOLD, MAX_EXPL, MIN_EXPL};
use super::super::common::{DELT_VBS0DIO, DELT_VBS0EFF, DELT_VBSDIO, OFF_VBSDIO};
use super::temp::B3SoiPdSized;
use crate::Value;

/// Linearized DC operating point of one B3SOIPD instance.
///
/// Field names mirror the `here->B3SOIPD*` slots that ngspice fills at the end
/// of the DC block (b3soipdld.c:2556-2640) and consumes during stamping. All
/// conductances are in the device's internal polarity; `mode` records the
/// normal(+1)/inverse(-1) channel direction.
#[derive(Debug, Clone, Default)]
pub struct B3SoiPdOp {
    pub mode: i32,

    /// Channel + collector current into the drain prime (`B3SOIPDcdrain`).
    pub cdrain: Value,
    /// Net drain-prime current `Ids + Ic - Ibd + Iii + Idgidl` (`B3SOIPDcd`).
    pub cd: Value,
    /// Net body current source term (`B3SOIPDcb`).
    pub cb: Value,

    /// Channel current `Ids` (`B3SOIPDids`).
    pub ids: Value,
    /// Threshold voltage at the operating point (`B3SOIPDvon`).
    pub von: Value,
    /// Saturation voltage (`B3SOIPDvdsat`).
    pub vdsat: Value,

    // Transconductances of the channel current (B3SOIPDg*).
    pub gm: Value,
    pub gds: Value,
    pub gmbs: Value,
    pub gme: Value,

    // Drain-side body junction current linearization (B3SOIPDgjd*, B3SOIPDcjd).
    pub gjdb: Value,
    pub gjdd: Value,
    pub gjdg: Value,
    pub gjde: Value,
    pub cjd: Value,

    // Source-side body junction current linearization (B3SOIPDgjs*, B3SOIPDcjs).
    pub gjsb: Value,
    pub gjsd: Value,
    pub gjsg: Value,
    pub cjs: Value,

    // Body-node KCL linearization (B3SOIPDgb*, B3SOIPDcbody).
    pub gbbs: Value,
    pub gbgs: Value,
    pub gbds: Value,
    pub gbes: Value,
    pub gbps: Value,
    pub cbody: Value,

    /// Inversion charge proxy used by noise (`B3SOIPDqinv`).
    pub qinv: Value,

    /// CAPMOD=3 charge state (set only when [`eval`] is asked to compute it).
    pub charge: Option<B3SoiPdCharge>,
}

/// CAPMOD=3 charge-model output for one B3SOIPD instance.
///
/// Mirrors the `here->B3SOIPDq*` node charges and the `here->B3SOIPDc*` intrinsic
/// capacitance matrix that ngspice fills at the end of `B3SOIPDload`
/// (b3soipdld.c:3387-3429) plus the extrinsic S/D-to-substrate spline charges
/// (b3soipdld.c:3438-3609) and the gate overlap charges (b3soipdld.c:3655-3784).
///
/// The four node charges (`qg/qb/qd/qe`) include the overlap and extrinsic lumps
/// exactly as ngspice does in the `mode>0` branch at b3soipdld.c:3722-3729, so
/// `qg+qb+qd+qe+qs == 0`. Capacitances are the `gc**b`-style derivatives *before*
/// multiplication by the integration coefficient `ag0` (the device applies `ag0`
/// when it forms the transient companion). All quantities are in device polarity
/// with `mtype` already folded in where ngspice folds it.
#[derive(Debug, Clone, Default)]
pub struct B3SoiPdCharge {
    /// Channel direction at evaluation (`here->B3SOIPDmode`).
    pub mode: i32,

    // Node charges (after overlap + extrinsic lumping, b3soipdld.c:3722-3729).
    pub qg: Value,
    pub qb: Value,
    pub qd: Value,
    pub qe: Value,

    // Intrinsic + overlap capacitance matrix (the `gc**`/ag0 coefficients).
    // Row = charge node, col = controlling node. Drain/source are the *primes*
    // (== external in the supported decks). Already includes overlap and
    // extrinsic S/D-substrate derivatives, matching b3soipdld.c:3680-3766.
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
/// `ves`/`vps` after limiting (b3soipdld.c:836).
#[derive(Debug, Clone, Copy)]
pub struct B3SoiPdBias {
    pub vbs: Value,
    pub vgs: Value,
    pub vds: Value,
    pub ves: Value,
    pub vps: Value,
}

/// Evaluate the B3SOIPD DC operating point.
///
/// `p` is the size/temperature-resolved parameter set, `m_*` the few model-card
/// scalars needed in the load, and `bias` the device-polarity branch voltages.
/// `temp_k` is the (constant, no self-heating) device temperature in Kelvin and
/// `mtype` the polarity (+1 NMOS / -1 PMOS).
#[allow(clippy::too_many_lines)]
pub fn eval_dc(
    p: &B3SoiPdSized,
    m: &ModelConsts,
    bias: B3SoiPdBias,
    mtype: Value,
) -> B3SoiPdOp {
    eval(p, m, bias, mtype, false)
}

/// Evaluate the B3SOIPD operating point, optionally including the CAPMOD=3
/// charge model (`compute_charges == true`, the `ChargeComputationNeeded` path).
///
/// The DC current path is identical to [`eval_dc`]; when `compute_charges` is set
/// the resulting [`B3SoiPdOp::charge`] carries the intrinsic + extrinsic charge
/// state (b3soipdld.c:2637-3784, capMod==3, selfheat==0).
#[allow(clippy::too_many_lines)]
pub fn eval(
    p: &B3SoiPdSized,
    m: &ModelConsts,
    bias: B3SoiPdBias,
    mtype: Value,
    compute_charges: bool,
) -> B3SoiPdOp {
    let mut op = B3SoiPdOp::default();

    // --- Temperature-dependent quantities (selfheat == 0 branch,
    //     b3soipdld.c:803-822) ---
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
    // (b3soipdtemp.c folds in the *1*(tratio-1) term). selfheat==0 means no
    // further adjustment here.
    let ua = p.ua;
    let ub = p.ub;
    let uc = p.uc;

    let temp_ratio = temp_ratio_m1(p); // CKTtemp/tnom - 1 (selfheat==0)

    let vtm = p.vtm;

    // --- Mode setup (b3soipdld.c:836-860) ---
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

    // --- Poly-gate depletion (b3soipdld.c:886-905) ---
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

    // --- PD body voltage chain (b3soipdld.c:828-872) ---
    //
    // PD vs DD/FD: the partially-depleted body voltage IS the body-node voltage.
    // There is no Vbs0t/Vbs0/Vbsdio equilibrium chain. `Vbseff` is a direct
    // smooth clamp of the actual `Vbs` to (-5, 1.5) then to 0.95*phi, plus the
    // `sqrtPhisExt` extension used by Vth (b3soipdld.c:828-1000). The unused DD
    // film constants are kept alive for provenance.
    let factor1 = p.factor1;
    let _ = (vesfb, p.dvbd0, p.dvbd1, p.litl, p.vbsa, p.delp, p.kb1, p.kb3);
    let _ = (DELT_VBS0DIO, DELT_VBS0EFF, OFF_VBSDIO, DELT_VBSDIO, vps, k1);
    // The body-tie resistor current Ibp (b3soipdld.c:2017-2042) is stamped as a
    // body<->P resistor by the device (bodyMod 1); the eval emits no Ibp.

    // T2 = Vbs limited above Vbsc = -5 (b3soipdld.c:830-833).
    let (vbsh, dvbsh_dvb);
    {
        let t0 = vbs + 5.0 - 0.001;
        let t1 = (t0 * t0 - 0.004 * (-5.0)).sqrt();
        let t2 = -5.0 + 0.5 * (t0 + t1);
        let dt2_dvb = 0.5 * (1.0 + t0 / t1);

        // Vbsh = T2 limited below 1.5 (b3soipdld.c:836-840).
        let t0 = 1.5;
        let t1 = t0 - t2 - 0.002;
        let t3 = (t1 * t1 + 0.008 * t0).sqrt();
        vbsh = t0 - 0.5 * (t1 + t3);
        dvbsh_dvb = 0.5 * (1.0 + t1 / t3) * dt2_dvb;
    }

    // Vbseff = Vbsh limited to 0.95*phi (b3soipdld.c:843-857).
    let vbseff;
    let mut dvbseff_dvb;
    let mut dvbsh_dvb_eff = dvbsh_dvb;
    {
        let t0 = 0.95 * phi;
        let t1 = t0 - vbsh - 0.002;
        let t2 = (t1 * t1 + 0.008 * t0).sqrt();
        vbseff = t0 - 0.5 * (t1 + t2);
        dvbseff_dvb = 0.5 * (1.0 + t1 / t2) * dvbsh_dvb;
        // Normalize dVbsh/dVb relative to dVbseff/dVb (b3soipdld.c:851-857).
        if dvbseff_dvb < 1e-20 {
            dvbseff_dvb = 1e-20;
            dvbsh_dvb_eff *= 1e20;
        } else {
            dvbsh_dvb_eff /= dvbseff_dvb;
        }
    }
    // PD's Vbseff depends only on Vbs (g/d/e derivatives are zero).
    let dvbseff_dvg = 0.0;
    let dvbseff_dvd = 0.0;
    let dvbseff_dve = 0.0;

    // PD has no separate Vcs (the DD charge-side body coupling); zero it.
    let vcs = 0.0;
    let (dvcs_dvg, dvcs_dvd, dvcs_dvb, dvcs_dve) = (0.0, 0.0, 0.0, 0.0);

    // --- Vth (with Vbseff + sqrtPhisExt), b3soipdld.c:860-1000 ---
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
        if t0 > -EXPL_THRESHOLD {
            let t1 = t0.exp();
            theta0 = t1 * (1.0 + 2.0 * t1);
            let dt1_dvb = -t0 / lt1 * t1 * dlt1_dvb;
            dtheta0_dvb = (1.0 + 4.0 * t1) * dt1_dvb;
        } else {
            let t1 = MIN_EXPL;
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
        if t0 > -EXPL_THRESHOLD {
            let t1 = t0.exp();
            t2v = t1 * (1.0 + 2.0 * t1);
            let dt1_dvb = -t0 / ltw * t1 * dltw_dvb;
            dt2_dvb = (1.0 + 4.0 * t1) * dt1_dvb;
        } else {
            let t1 = MIN_EXPL;
            t2v = t1 * (1.0 + 2.0 * t1);
            dt2_dvb = 0.0;
        }
        deltvthw = p.dvt0w * t2v * v0;
        ddeltvthw_dvb = p.dvt0w * dt2_dvb * v0;
    }

    let t0t = (1.0 + p.nlx / leff).sqrt();
    let t1t = p.kt1 + p.kt1l / leff + p.kt2 * vbseff;
    // PD uses k1eff (k1 with the body-tie width correction) here and in Vth.
    let delt_vthtemp = p.k1eff * (t0t - 1.0) * sqrt_phi + t1t * temp_ratio;

    let tmp2 = m.tox * phi / (p.weff + p.w0);

    let (t3e, dt3_dvb_eta) = smooth_etab(p.eta0, p.etab, vbseff);
    let dibl_sft = t3e * p.theta0vb0 * vds;
    let ddibl_sft_dvd = p.theta0vb0 * t3e;
    let ddibl_sft_dvb = p.theta0vb0 * vds * dt3_dvb_eta;

    // sqrtPhisExt: PD's body-charge sqrt extension (b3soipdld.c:976-978).
    let t9_ext = 2.2361 / sqrt_phi;
    let sqrt_phis_ext = sqrt_phis - t9_ext * (vbsh - vbseff);
    let dsqrt_phis_ext_dvb = dsqrt_phis_dvb - t9_ext * (dvbsh_dvb_eff - 1.0);

    let vth = mtype * p.vth0 + p.k1eff * (sqrt_phis_ext - sqrt_phi) - p.k2 * vbseff - delt_vth
        - deltvthw
        + (p.k3 + p.k3b * vbseff) * tmp2
        + delt_vthtemp
        - dibl_sft;
    op.von = vth;

    let t6v = p.k3b * tmp2 - p.k2 + p.kt2 * temp_ratio;
    let dvth_dvb =
        p.k1eff * dsqrt_phis_ext_dvb - ddelt_vth_dvb - ddeltvthw_dvb + t6v - ddibl_sft_dvb;
    let dvth_dvd = -ddibl_sft_dvd;

    // --- n (subthreshold swing), b3soipdld.c:1363-1390 ---
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

    // --- Vgsteff (b3soipdld.c:1393-1490) ---
    let vgst = vgs_eff - vth;
    let t10 = 2.0 * n * vtm;
    let vgst_n_vt = vgst / t10;
    let exp_arg = (2.0 * p.voff - vgst) / t10;

    let (vgsteff, dvgsteff_dvg, dvgsteff_dvd, dvgsteff_dvb, dvgsteff_dve);
    let mut exp_vgst = 0.0_f64;
    if vgst_n_vt > EXPL_THRESHOLD {
        vgsteff = vgst;
        let t0 = -dvth_dvb;
        dvgsteff_dvg = dvgs_eff_dvg + t0 * dvbseff_dvg;
        dvgsteff_dvd = -dvth_dvd + t0 * dvbseff_dvd;
        dvgsteff_dvb = t0 * dvbseff_dvb;
        dvgsteff_dve = t0 * dvbseff_dve;
    } else if exp_arg > EXPL_THRESHOLD {
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

    // --- Effective W, Rds (b3soipdld.c:1492-1556) ---
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

    // --- PD Abulk / Abulk0 (b3soipdld.c:1178-1253) ---
    //
    // PD's bulk-charge factor differs from DD: it uses the *un*-clamped body
    // voltage `Vbsh` (not Vbseff) through a `keta`/`ketas` body-charge roll-off
    // with a 1/sqrt(1-T13) limiter, and `k1eff` over `sqrt(phi+ketas)`. The
    // result already includes the leading `1 +` (no separate increment).
    let (mut abulk0, mut abulk, mut dabulk0_dvb, dabulk_dvg, mut dabulk_dvb);
    if p.a0 == 0.0 {
        abulk0 = 1.0;
        abulk = 1.0;
        dabulk0_dvb = 0.0;
        dabulk_dvg = 0.0;
        dabulk_dvb = 0.0;
    } else {
        // keta body-charge roll-off on Vbsh.
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
        let t10d = p.phi + p.ketas;
        let t13 = vbsh * t11 / t10d;
        let dt13_dvb = (vbsh * dt11_dvb + t11 * dvbsh_dvb_eff) / t10d;

        // limit 1/sqrt(1-T13) to ~6 starting at T13=0.96.
        let (t14, dt14_dvb);
        if t13 < 0.96 {
            let t = 1.0 / (1.0 - t13).sqrt();
            let t10b = 0.5 * t / (1.0 - t13);
            t14 = t;
            dt14_dvb = t10b * dt13_dvb;
        } else {
            let t11b = 1.0 / (1.0 - 1.043406 * t13);
            t14 = (6.00167 - 6.26044 * t13) * t11b;
            let t10b = 0.001742 * t11b * t11b;
            dt14_dvb = t10b * dt13_dvb;
        }

        let t10c = 0.5 * p.k1eff / (p.phi + p.ketas).sqrt();
        let t1 = t10c * t14;
        let dt1_dvb = t10c * dt14_dvb;

        let t9 = (m.xj * xdep).sqrt();
        let tmp1 = leff + 2.0 * t9;
        let t5 = leff / tmp1;
        let tmp2 = p.a0 * t5;
        let tmp3 = p.weff + p.b1;
        let tmp4 = p.b0 / tmp3;
        let t2 = tmp2 + tmp4;
        let dt2_dvb = -t9 * tmp2 / tmp1 / xdep * dxdep_dvb;
        let t6 = t5 * t5;
        let t7 = t5 * t6;

        abulk0 = 1.0 + t1 * t2;
        dabulk0_dvb = t1 * dt2_dvb + t2 * dt1_dvb;

        let t8 = p.ags * p.a0 * t7;
        dabulk_dvg = -t1 * t8;
        abulk = abulk0 + dabulk_dvg * vgsteff;
        dabulk_dvb = dabulk0_dvb - t8 * vgsteff * (dt1_dvb + 3.0 * t1 * dt2_dvb / tmp2);
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
    // --- Mobility (MOBMOD 0 uses the mobMod==1 branch in ngspice, see note) ---
    // ngspice b3soipdld.c only implements mobMod 1/2/3; MOBMOD=0 cards fall to
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

    // --- Vdsat (b3soipdld.c:1719-1820) ---
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
        let t0 = 1.0 / (abulk * esat_l + vgst2vtm);
        let t1 = t0 * t0;
        let t2 = vgst2vtm * t0;
        let t3 = esat_l * vgst2vtm;
        vdsat = t3 * t0;
        let dt0_dvg = -(abulk * desat_l_dvg + esat_l * dabulk_dvg + 1.0) * t1;
        let dt0_dvd = -(abulk * desat_l_dvd) * t1;
        let dt0_dvb = -(abulk * desat_l_dvb + esat_l * dabulk_dvb) * t1;
        let dt0_dvc = -(esat_l * 0.0) * t1;
        dvdsat_dvg = t3 * dt0_dvg + t2 * desat_l_dvg + esat_l * t0;
        dvdsat_dvd = t3 * dt0_dvd + t2 * desat_l_dvd;
        dvdsat_dvb = t3 * dt0_dvb + t2 * desat_l_dvb;
        dvdsat_dvc = t3 * dt0_dvc;
    } else {
        tmp1l = dlambda_dvg / (lambda * lambda);
        let t9 = abulk * wvcox_rds;
        let t8 = abulk * t9;
        let t7 = vgst2vtm * t9;
        let t6 = vgst2vtm * wvcox_rds;
        let t0 = 2.0 * abulk * (t9 - 1.0 + 1.0 / lambda);
        let dt0_dvg = 2.0
            * (t8 * tmp2v - abulk * tmp1l + (2.0 * t9 + 1.0 / lambda - 1.0) * dabulk_dvg);
        let dt0_dvb = 2.0
            * (t8 * (2.0 / abulk * dabulk_dvb + tmp3v) + (1.0 / lambda - 1.0) * dabulk_dvb);
        let _dt0_dvd = 0.0; // ngspice dT0_dVd = 0 in this branch (unused below)
        let dt0_dvc = 4.0 * t9 * 0.0;

        let t1 = vgst2vtm * (2.0 / lambda - 1.0) + abulk * esat_l + 3.0 * t7;
        let dt1_dvg = (2.0 / lambda - 1.0) - 2.0 * vgst2vtm * tmp1l + abulk * desat_l_dvg
            + esat_l * dabulk_dvg
            + 3.0 * (t9 + t7 * tmp2v + t6 * dabulk_dvg);
        let dt1_dvb =
            abulk * desat_l_dvb + esat_l * dabulk_dvb + 3.0 * (t6 * dabulk_dvb + t7 * tmp3v);
        let dt1_dvd = abulk * desat_l_dvd;
        let dt1_dvc = esat_l * 0.0 + 3.0 * t6 * 0.0;

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

    // --- Vdsatii (impact-ionization Vdsat), b3soipdld.c:1823-1880 ---
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

    // --- Vdseff (b3soipdld.c:1883-1918) ---
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

    // --- Vdseffii (b3soipdld.c:1920-1945) ---
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

    // --- VAsat (b3soipdld.c:1948-1985) ---
    let (vasat, dvasat_dvg, dvasat_dvd, dvasat_dvb, dvasat_dvc);
    {
        let tmp4 = 1.0 - 0.5 * abulk * vdsat / vgst2vtm;
        let t9 = wvcox_rds * vgsteff;
        let t8 = t9 / vgst2vtm;
        let t0 = esat_l + vdsat + 2.0 * t9 * tmp4;
        let t7 = 2.0 * wvcox_rds * tmp4;
        let dt0_dvg = desat_l_dvg + dvdsat_dvg + t7 * (1.0 + tmp2v * vgsteff)
            - t8 * (abulk * dvdsat_dvg - abulk * vdsat / vgst2vtm + vdsat * dabulk_dvg);
        let dt0_dvb = desat_l_dvb + dvdsat_dvb + t7 * tmp3v * vgsteff
            - t8 * (dabulk_dvb * vdsat + abulk * dvdsat_dvb);
        let dt0_dvd = desat_l_dvd + dvdsat_dvd - t8 * abulk * dvdsat_dvd;
        let dt0_dvc = dvdsat_dvc - t8 * (abulk * dvdsat_dvc + vdsat * 0.0);
        let t9b = wvcox_rds * abulk;
        let t1 = 2.0 / lambda - 1.0 + t9b;
        let dt1_dvg = -2.0 * tmp1l + wvcox_rds * (abulk * tmp2v + dabulk_dvg);
        let dt1_dvb = dabulk_dvb * wvcox_rds + t9b * tmp3v;
        let dt1_dvc = 0.0 * wvcox_rds;
        vasat = t0 / t1;
        dvasat_dvg = (dt0_dvg - vasat * dt1_dvg) / t1;
        dvasat_dvb = (dt0_dvb - vasat * dt1_dvb) / t1;
        dvasat_dvd = dt0_dvd / t1;
        dvasat_dvc = (dt0_dvc - vasat * dt1_dvc) / t1;
    }

    // --- VACLM (b3soipdld.c:1988-2018) ---
    let (vaclm, dvaclm_dvg, dvaclm_dvd, dvaclm_dvb, dvaclm_dvc);
    if p.pclm > 0.0 && diff_vds > 1.0e-10 {
        let t0 = 1.0 / (p.pclm * abulk * p.litl);
        let dt0_dvb = -t0 / abulk * dabulk_dvb;
        let dt0_dvg = -t0 / abulk * dabulk_dvg;
        let dt0_dvc = -t0 / abulk * 0.0;
        let t2 = vgsteff / esat_l;
        let t1 = leff * (abulk + t2);
        let dt1_dvg = leff * ((1.0 - t2 * desat_l_dvg) / esat_l + dabulk_dvg);
        let dt1_dvb = leff * (dabulk_dvb - t2 * desat_l_dvb / esat_l);
        let dt1_dvd = -t2 * desat_l_dvd / esat;
        let dt1_dvc = leff * 0.0;
        let t9 = t0 * t1;
        vaclm = t9 * diff_vds;
        dvaclm_dvg = t0 * dt1_dvg * diff_vds - t9 * dvdseff_dvg + t1 * diff_vds * dt0_dvg;
        dvaclm_dvb = (dt0_dvb * t1 + t0 * dt1_dvb) * diff_vds - t9 * dvdseff_dvb;
        dvaclm_dvd = t0 * dt1_dvd * diff_vds + t9 * (1.0 - dvdseff_dvd);
        dvaclm_dvc = (t1 * dt0_dvc + t0 * dt1_dvc) * diff_vds - t9 * dvdseff_dvc;
    } else {
        vaclm = MAX_EXPL;
        dvaclm_dvd = 0.0;
        dvaclm_dvg = 0.0;
        dvaclm_dvb = 0.0;
        dvaclm_dvc = 0.0;
    }

    // --- VADIBL (b3soipdld.c:2021-2090) ---
    let (mut vadibl, mut dvadibl_dvg, mut dvadibl_dvd, mut dvadibl_dvb, mut dvadibl_dvc);
    if p.theta_rout > 0.0 {
        let t8 = abulk * vdsat;
        let t0 = vgst2vtm * t8;
        let t1 = vgst2vtm + t8;
        let dt0_dvg = vgst2vtm * abulk * dvdsat_dvg + t8 + vgst2vtm * vdsat * dabulk_dvg;
        let dt1_dvg = 1.0 + abulk * dvdsat_dvg + vdsat * dabulk_dvg;
        let dt1_dvb = dabulk_dvb * vdsat + abulk * dvdsat_dvb;
        let dt0_dvb = vgst2vtm * dt1_dvb;
        let dt1_dvd = abulk * dvdsat_dvd;
        let dt0_dvd = vgst2vtm * dt1_dvd;
        let dt1_dvc = abulk * dvdsat_dvc + vdsat * 0.0;
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
        vadibl = MAX_EXPL;
        dvadibl_dvd = 0.0;
        dvadibl_dvg = 0.0;
        dvadibl_dvb = 0.0;
        dvadibl_dvc = 0.0;
    }

    // --- Va (b3soipdld.c:2093-2150) ---
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

    // --- Ids (b3soipdld.c:2153-2230) ---
    let cox_wov_l = m.cox * weff / leff;
    let beta = ueff * cox_wov_l;
    let dbeta_dvg = cox_wov_l * dueff_dvg + beta * dweff_dvg / weff;
    let dbeta_dvd = cox_wov_l * dueff_dvd;
    let dbeta_dvb = cox_wov_l * dueff_dvb + beta * dweff_dvb / weff;

    let (fgche1, dfgche1_dvg, dfgche1_dvd, dfgche1_dvb, dfgche1_dvc);
    {
        let t0 = 1.0 - 0.5 * abulk * vdseff / vgst2vtm;
        let dt0_dvg = -0.5
            * (abulk * dvdseff_dvg - abulk * vdseff / vgst2vtm + vdseff * dabulk_dvg)
            / vgst2vtm;
        let dt0_dvd = -0.5 * abulk * dvdseff_dvd / vgst2vtm;
        let dt0_dvb =
            -0.5 * (abulk * dvdseff_dvb + dabulk_dvb * vdseff) / vgst2vtm;
        let dt0_dvc =
            -0.5 * (abulk * dvdseff_dvc + 0.0 * vdseff) / vgst2vtm;
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

    // --- PD impact-ionization Iii (b3soipdld.c:2534-2620) ---
    //
    // PD uses a Vdsatii0/VgsStep/Vdiff exponential model with parameters
    // sii0/sii1/sii2/siid, vdsatii0, lii, esatii, beta0/beta1/beta2 and alpha0,
    // entirely different from the DD AII/BII/CII/DII formulation. The body BJT
    // collector contributes via `fbjtii*Ic` (zero in the supported decks). The
    // channel total conductances `gm`,`gmb`,`gds` (computed above) carry the
    // Vgsteff/Vbseff chain so Iii's derivatives reuse them directly.
    let (mut iii, mut giig, mut giib, mut giid, mut giie) = (0.0, 0.0, 0.0, 0.0, 0.0);
    if p.alpha0 > 0.0 {
        let dvgst_dvg = dvgs_eff_dvg;
        let dvgst_dvd = -dvth_dvd;
        let dvgst_dvb = -dvth_dvb;

        let vdsatii0 = p.vdsatii0 * (1.0 + p.tii * temp_ratio) - p.lii / leff;

        // VgsStep
        let t0 = p.esatii * leff;
        let t1c = p.sii0 * t0 / (1.0 + t0);
        let t0 = 1.0 / (1.0 + p.sii1 * vgsteff);
        let t3 = t0 + p.sii2;
        let t4 = vgst * p.sii1 * t0 * t0;
        let t2 = vgst * t3;
        let dt2_dvg = t3 * dvgst_dvg - t4 * dvgsteff_dvg;
        let dt2_dvb = t3 * dvgst_dvb * dvbseff_dvb - t4 * dvgsteff_dvb;
        let dt2_dvd = t3 * dvgst_dvd - t4 * dvgsteff_dvd;

        let t3d = 1.0 / (1.0 + p.siid * vds);
        let dt3_dvd = -p.siid * t3d * t3d;

        let vgs_step = t1c * t2 * t3d;
        let vdsatii = vdsatii0 + vgs_step;
        let vdiff = vds - vdsatii;
        let dvdiff_dvg = -t1c * t3d * dt2_dvg;
        let dvdiff_dvb = -t1c * t3d * dt2_dvb;
        let dvdiff_dvd = 1.0 - t1c * (t3d * dt2_dvd + t2 * dt3_dvd);

        let t0b = p.beta2 + p.beta1 * vdiff + p.beta0 * vdiff * vdiff;
        let (t0v, dt0_dvg, dt0_dvb, dt0_dvd);
        if t0b < 1e-5 {
            t0v = 1e-5;
            dt0_dvg = 0.0;
            dt0_dvb = 0.0;
            dt0_dvd = 0.0;
        } else {
            t0v = t0b;
            let t1 = p.beta1 + 2.0 * p.beta0 * vdiff;
            dt0_dvg = t1 * dvdiff_dvg;
            dt0_dvb = t1 * dvdiff_dvb;
            dt0_dvd = t1 * dvdiff_dvd;
        }

        let (mut ratio, mut dratio_dvg, mut dratio_dvb, mut dratio_dvd);
        if t0v < vdiff / EXPL_THRESHOLD && vdiff > 0.0 {
            ratio = p.alpha0 * MAX_EXPL;
            dratio_dvg = 0.0;
            dratio_dvb = 0.0;
            dratio_dvd = 0.0;
        } else if t0v < -vdiff / EXPL_THRESHOLD && vdiff < 0.0 {
            ratio = p.alpha0 * MIN_EXPL;
            dratio_dvg = 0.0;
            dratio_dvb = 0.0;
            dratio_dvd = 0.0;
        } else {
            ratio = p.alpha0 * (vdiff / t0v).exp();
            let t1 = ratio / t0v / t0v;
            dratio_dvg = t1 * (t0v * dvdiff_dvg - vdiff * dt0_dvg);
            dratio_dvb = t1 * (t0v * dvdiff_dvb - vdiff * dt0_dvb);
            dratio_dvd = t1 * (t0v * dvdiff_dvd - vdiff * dt0_dvd);
        }
        if ratio > 10.0 {
            ratio = 10.0;
            dratio_dvg = 0.0;
            dratio_dvb = 0.0;
            dratio_dvd = 0.0;
        }

        // T0 = Ids + fbjtii*Ic. The BJT Ic is computed below (after Iii in this
        // port); with fbjtii==0 in the supported decks the term vanishes, so use
        // Ids alone. (A nonzero fbjtii would require evaluating Ic first.)
        let t0i = ids + p.fbjtii * 0.0;
        iii = ratio * t0i;
        giig = ratio * gm + t0i * dratio_dvg;
        giib = ratio * gmb + t0i * dratio_dvb;
        giid = ratio * gds + t0i * dratio_dvd;
        giie = 0.0;
    }
    let _ = (diff_vdsii, dvdseffii_dvg, dvdseffii_dvd, dvdseffii_dvb, gm0, gds0, gmb0, gmc);

    // --- GIDL (b3soipdld.c:2293-2350) ---
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
            if t2 < EXPL_THRESHOLD {
                idgidl = p.weff * p.agidl * t1 * (-t2).exp();
                let t3 = idgidl / t1 * (t2 + 1.0);
                gdgidld = t3 * dt1_dvd;
                gdgidlg = t3 * dt1_dvg;
            } else {
                let t3 = p.weff * p.agidl * MIN_EXPL;
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
            if t2 < EXPL_THRESHOLD {
                isgidl = p.weff * p.agidl * t1 * (-t2).exp();
                let t3 = isgidl / t1 * (t2 + 1.0);
                gsgidlg = t3 * dt1_dvg;
            } else {
                let t3 = p.weff * p.agidl * MIN_EXPL;
                isgidl = t3 * t1;
                gsgidlg = t3 * dt1_dvg;
            }
        }
    }

    // --- PD body diodes + parasitic BJT (b3soipdld.c:1825-2270) ---
    //
    // PD's body junction is a full diode model: diffusion (Ibs1/Ibd1),
    // recombination + reverse trap-assisted tunneling (Ibs2/Ibd2), neutral-body
    // recombination with high-level-injection roll-off (Ibs3/Ibd3), and band-to-
    // band tunneling (Ibs4/Ibd4), plus a parasitic-BJT collector current (Ic)
    // with an Early-voltage second-order factor. This is entirely different from
    // DD's single-exp diode + simple lateral BJT.
    let w_tsi = p.weff * m.tsi; // nseg == 1, pdbcp/psbcp == 0 (supported decks)
    let vbd = vbs - vds; // device-frame Vbd
    let n_vtm1 = vtm * p.ndiode;

    // DEXP: ngspice's exp clamped above 40 to a linear extension (returns value
    // and its derivative factor).
    let dexp = |arg: Value| -> (Value, Value) {
        if arg > 40.0 {
            let e = 2.353_852_668_370_2e17; // exp(40)
            (e * (arg - 39.0), e)
        } else if arg < -40.0 {
            let e = 4.248_354_255_291_589e-18; // exp(-40)
            (e * (41.0 + arg), e)
        } else {
            let e = arg.exp();
            (e, e)
        }
    };

    let (exp_vbs_n, dexp_vbs_n) = dexp(vbs / n_vtm1);
    let dexp_vbs_n_dvb = dexp_vbs_n / n_vtm1;
    let (exp_vbd_n, dexp_vbd_n) = dexp(vbd / n_vtm1);
    let dexp_vbd_n_dvb = dexp_vbd_n / n_vtm1;
    let dexp_vbd_n_dvd = -dexp_vbd_n_dvb;

    // Ibs1 / Ibd1: diffusion current (b3soipdld.c:1862-1891).
    let (ibs1, dibs1_dvb, ibd1, dibd1_dvb, dibd1_dvd);
    if jdif == 0.0 {
        ibs1 = 0.0;
        dibs1_dvb = 0.0;
        ibd1 = 0.0;
        dibd1_dvb = 0.0;
        dibd1_dvd = 0.0;
    } else {
        let t0 = w_tsi * jdif;
        ibs1 = t0 * (exp_vbs_n - 1.0);
        dibs1_dvb = t0 * dexp_vbs_n_dvb;
        ibd1 = t0 * (exp_vbd_n - 1.0);
        dibd1_dvb = t0 * dexp_vbd_n_dvb;
        dibd1_dvd = -dibd1_dvb;
    }

    // Ibs2 / Ibd2: recombination + reverse trap-assisted tunneling
    // (b3soipdld.c:1893-1995). NVtmf/NVtmr use 0.026 (not Vtm).
    let n_vtmf = 0.026 * p.nrecf0 * (1.0 + p.ntrecf * (temp_ratio));
    let n_vtmr = 0.026 * p.nrecr0 * (1.0 + p.ntrecr * (temp_ratio));
    let (ibs2, dibs2_dvb, ibd2, dibd2_dvb, dibd2_dvd);
    if jrec == 0.0 {
        ibs2 = 0.0;
        dibs2_dvb = 0.0;
        ibd2 = 0.0;
        dibd2_dvb = 0.0;
        dibd2_dvd = 0.0;
    } else {
        // Source: forward + reverse.
        let (t10s, e10s) = dexp(vbs / n_vtmf);
        let dt10s_dvb = e10s / n_vtmf;
        let (t11s, dt11s_dvb) = if (p.vrec0 - vbs) < 1e-3 {
            let t0 = -vbs / n_vtmr * p.vrec0 * 1e3;
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
        let (t11d, dt11d_dvb) = if (p.vrec0 - vbd) < 1e-3 {
            let t0 = -vbd / n_vtmr * p.vrec0 * 1e3;
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

    // Ibs3 / Ibd3: neutral-body recombination with high-level injection, plus
    // BJT collector Ic (b3soipdld.c:1997-2192).
    let (mut ic, mut gcd, mut gcb) = (0.0, 0.0, 0.0);
    let (ibs3, dibs3_dvb, dibs3_dvd, ibd3, dibd3_dvb, dibd3_dvd);
    if jbjt == 0.0 {
        ibs3 = 0.0;
        dibs3_dvb = 0.0;
        dibs3_dvd = 0.0;
        ibd3 = 0.0;
        dibd3_dvb = 0.0;
        dibd3_dvd = 0.0;
    } else {
        let ien = w_tsi * jbjt * p.lratio;

        // High-level injection roll-off factors (source / drain).
        let ahli = p.ahli0;
        let (ehlis, dehlis_dvb, ehlis_factor, dehlis_factor_dvb) = {
            let e = ahli * (exp_vbs_n - 1.0);
            if e < 1e-5 {
                (0.0, 0.0, 1.0, 0.0)
            } else {
                let de = ahli * dexp_vbs_n_dvb;
                let f = 1.0 / (1.0 + e).sqrt();
                let t0 = -0.5 * f / (1.0 + e);
                (e, de, f, t0 * de)
            }
        };
        let (ehlid, dehlid_dvb, dehlid_dvd, ehlid_factor, dehlid_factor_dvb, _dehlid_factor_dvd) = {
            let e = ahli * (exp_vbd_n - 1.0);
            if e < 1e-5 {
                (0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
            } else {
                let de = ahli * dexp_vbd_n_dvb;
                let de_dvd = -de;
                let f = 1.0 / (1.0 + e).sqrt();
                let t0 = -0.5 * f / (1.0 + e);
                (e, de, de_dvd, f, t0 * de, -(t0 * de))
            }
        };

        // Neutral-body recombination Ibs3/Ibd3 (suppressed when arfabjt~1).
        let t0a = 1.0 - p.arfabjt;
        if t0a < 1e-2 {
            ibs3 = 0.0;
            dibs3_dvb = 0.0;
            dibs3_dvd = 0.0;
            ibd3 = 0.0;
            dibd3_dvb = 0.0;
            dibd3_dvd = 0.0;
        } else {
            let t1 = t0a * ien;
            ibs3 = t1 * (exp_vbs_n - 1.0) * ehlis_factor;
            dibs3_dvb = t1
                * (dexp_vbs_n_dvb * ehlis_factor + (exp_vbs_n - 1.0) * dehlis_factor_dvb);
            dibs3_dvd = 0.0;
            ibd3 = t1 * (exp_vbd_n - 1.0) * ehlid_factor;
            dibd3_dvb = t1
                * (dexp_vbd_n_dvb * ehlid_factor + (exp_vbd_n - 1.0) * dehlid_factor_dvb);
            dibd3_dvd = -dibd3_dvb;
        }

        // BJT collector current Ic with Early-voltage second-order factor
        // (b3soipdld.c:2120-2192). bjtoff == 0 in the supported decks.
        if vds == 0.0 {
            ic = 0.0;
            gcd = 0.0;
            gcb = 0.0;
        } else {
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
                * ((dexp_vbs_n_dvb - dexp_vbd_n_dvb) * e2nd
                    + (exp_vbs_n - exp_vbd_n) * de2nd_dvb);
            gcd = t0c
                * (-dexp_vbd_n_dvd * e2nd + (exp_vbs_n - exp_vbd_n) * de2nd_dvd);
        }
    }

    // Ibs4 / Ibd4: band-to-band tunneling (b3soipdld.c:2197-2257).
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
        if (p.vtun0 - vbs) < 1e-3 {
            let t0 = -vbs / n_vtm2 * p.vtun0 * 1e3;
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
        if (p.vtun0 - vbd) < 1e-3 {
            let t0 = -vbd / n_vtm2 * p.vtun0 * 1e3;
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

    let ibs = ibs1 + ibs2 + ibs3 + ibs4;
    let ibd = ibd1 + ibd2 + ibd3 + ibd4;

    let gjsb = dibs1_dvb + dibs2_dvb + dibs3_dvb + dibs4_dvb;
    let gjsd = dibs3_dvd;
    let gjdb = dibd1_dvb + dibd2_dvb + dibd3_dvb + dibd4_dvb;
    let gjdd = dibd1_dvd + dibd2_dvd + dibd3_dvd + dibd4_dvd;

    // bodyMod 0/2: Ibp == 0.
    let min_isub = p.min_isub;

    // --- Operating-point assembly (b3soipdld.c:2556-2640) ---
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
    let t1q = vgsteff * (1.0 - 0.5 * abulk * vdseff / vgst2vtm);
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
                // PD has no DD-style Vbs0/Vbsdio equilibrium chain. The CAPMOD=3
                // charge model these feed is the DD dynamic-depletion model and is
                // not the PD CAPMOD=2 charge model (PD transient is a separate,
                // best-effort path). For the DC solve (`compute_charges=false`)
                // these are never read; map them to the PD body-voltage proxies so
                // the struct still type-checks.
                vbs0t: vbsh,
                vbs0: vbseff,
                dvbs0_dve: 0.0,
                vbs0mos: vbseff,
                dvbs0mos_dve: 0.0,
                vbs0eff: vbseff,
                dvbs0eff_dvg: 0.0,
                dvbs0eff_dvd: 0.0,
                dvbs0eff_dve: 0.0,
                vbsdio: vbs,
                dvbsdio_dvg: 0.0,
                dvbsdio_dvd: 0.0,
                dvbsdio_dve: 0.0,
                dvbsdio_dvb: dvbseff_dvb,
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

    // CAPMOD=3 charge-model model-card scalars (b3soipdld.c CV block).
    /// Buried-oxide series capacitance per area `cboxt = cbox*csi/(cbox+csi)`.
    pub cboxt: Value,
    /// Charge partition selector (`B3SOIPDxpart`): >0.5 0/100, <0.5 40/60, else 50/50.
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
fn temp_ratio_m1(p: &B3SoiPdSized) -> Value {
    p.temp / p.tnom - 1.0
}

/// Smooth `dvt2`-type discontinuity guard (b3soipdld.c:1020-1031 pattern).
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

/// Smooth `etab` guard (b3soipdld.c:1063-1074). Returns `(T3, dT3/dVb)`.
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

/// CAPMOD=3 charge model + extrinsic/overlap charges (b3soipdld.c:2646-3784).
///
/// Faithful transcription of the `capMod == 3` branch, the common capMod 2/3
/// backgate/inversion-charge code, the intrinsic S/D junction charge, the
/// extrinsic bottom-S/D-to-substrate spline, and the gate overlap charges, all
/// for `selfheat == 0` and the card's `xpart`. Returns the four node charges and
/// the intrinsic+overlap capacitance matrix (pre-`ag0`). `_mtype` is accepted for
/// symmetry with the DC eval; polarity is read from `m.mtype`.
#[allow(clippy::too_many_lines)]
fn eval_charges_capmod3(
    p: &B3SoiPdSized,
    m: &ModelConsts,
    _mtype: Value,
    mode: i32,
    i: &ChargeInputs,
) -> B3SoiPdCharge {
    use super::super::common::{CONST_2OV3, DELTA_1, DELTA_3, DELTA_VCSCV, QEX_FACT};

    let k1 = i.k1;
    let phi = i.phi;
    let cbox = i.cbox;

    // CoxWL (b3soipdld.c:2648).
    let cox_wl = m.cox * p.weff_cv * p.leff_cv;

    // Recompute Vgsteff,cv (b3soipdld.c:2659-2672) when in the moderate-inversion
    // window. Mirrors the DC subthreshold smoothing but with ExpVgst squared.
    let (mut vgsteff, mut dvgsteff_dvg, mut dvgsteff_dvd, mut dvgsteff_dvb, mut dvgsteff_dve) = (
        i.vgsteff,
        i.dvgsteff_dvg,
        i.dvgsteff_dvd,
        i.dvgsteff_dvb,
        i.dvgsteff_dve,
    );
    if i.vgst_n_vt > -EXPL_THRESHOLD && i.vgst_n_vt < EXPL_THRESHOLD {
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

    // Vfb (b3soipdld.c:2675-2678). dPhis_dVb == -1, dsqrtPhis_dVb == -0.5/sqrtPhis.
    let sqrt_phis = i.sqrt_phis;
    let dsqrt_phis_dvb = i.dsqrt_phis_dvb;
    let phis = i.phis;
    let d_phis_dvb = -1.0;
    let vfb = i.vth - phi - k1 * sqrt_phis;
    let dvfb_dvb = i.dvth_dvb - k1 * dsqrt_phis_dvb;
    let dvfb_dvd = i.dvth_dvd;

    // Vgsteff += 1e-4 (b3soipdld.c:2685).
    vgsteff += 1e-4;

    // Vfbeff (b3soipdld.c:2688-2704).
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

    // Qac0 (b3soipdld.c:2706-2711).
    let qac0 = -cox_wl * (vfbeff - vfb);
    let dqac0_dvrg = -cox_wl * dvfbeff_dvrg;
    let dqac0_dvd = -cox_wl * (dvfbeff_dvd - dvfb_dvd);
    let dqac0_dvb = -cox_wl * (dvfbeff_dvb - dvfb_dvb);

    // Qsub0 (b3soipdld.c:2713-2735).
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

    // VdsatCV redefined for capMod==3 (b3soipdld.c:2893-2904).
    let t1 = vgsteff + k1 * sqrt_phis + 0.5 * k1 * k1;
    let t2 = vgsteff + k1 * sqrt_phis + phis + 0.25 * k1 * k1;
    let dt1_dvb = k1 * dsqrt_phis_dvb;
    let dt2_dvb = dt1_dvb + d_phis_dvb;
    let dt1_dvg = 1.0;
    let dt2_dvg = 1.0;
    let vdsat_cv = t1 - k1 * t2.sqrt();
    let dvdsat_cv_dvb = dt1_dvb - k1 / 2.0 / t2.sqrt() * dt2_dvb;
    let dvdsat_cv_dvg = dt1_dvg - k1 / 2.0 / t2.sqrt() * dt2_dvg;

    // VdsCV from Vdsat / Vdseff (b3soipdld.c:2906-2978).
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

    // Qdep0 (b3soipdld.c:2992-2995).
    let t10 = cox_wl * k1;
    let qdep0 = t10 * sqrt_phis;
    let dqdep0_dvb = t10 * dsqrt_phis_dvb;

    // VcsCV (b3soipdld.c:2997-3028).
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

    // Xc (b3soipdld.c:3038-3099).
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

    // Nomi / Denomi -> Qsubs1 (b3soipdld.c:3101-3194).
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

    // Qsubs2 (b3soipdld.c:3196-3210).
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

    // Qbf (b3soipdld.c:3212-3223).
    let qbf = qac0 + qsub0 + qsubs1 + qsubs2 + qdep0;
    let dqbf_dvrg = dqac0_dvrg + dqsub0_dvrg + dqsubs2_dvrg;
    let dqbf_dvg = dqsub0_dvg + dqsubs1_dvg + dqsubs2_dvg;
    let dqbf_dvd = dqac0_dvd + dqsub0_dvd + dqsubs1_dvd + dqsubs2_dvd;
    let dqbf_dvb = dqac0_dvb + dqsub0_dvb + dqsubs1_dvb + dqsubs2_dvb + dqdep0_dvb;
    let dqbf_dvc = dqsubs1_dvc + dqsubs2_dvc;
    let dqbf_dve = dqsubs2_dve;

    // Common capMod 2/3: backgate charge (b3soipdld.c:3228-3284).
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
    // (b3soipdld.c:3288-3311).
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

    // Total inversion charge (b3soipdld.c:3313-3326). VdseffCV == IV Vdseff here.
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

    // Charge partition into S (b3soipdld.c:3329-3368).
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

    // Qex (b3soipdld.c:3378-3385).
    let t0 = QEX_FACT * k1 * cox_wl;
    let qex = t0 * (i.vbs - i.vbsdio);
    let dqex_dvg = -t0 * i.dvbsdio_dvg;
    let dqex_dvb = t0 * (1.0 - i.dvbsdio_dvb);
    let dqex_dvd = -t0 * i.dvbsdio_dvd;
    let dqex_dve = -t0 * i.dvbsdio_dve;

    // Node charges (intrinsic, b3soipdld.c:3387-3390).
    let mut qgate = qinv - (qbf + qe2);
    let mut qbody = qbf - qe1 + qex;
    let mut qsub = qe1 + qe2 - qex;
    let mut qdrn = -(qinv + qsrc);

    // Cgg/Cgd/Cgb/Cge (b3soipdld.c:3392-3398).
    let cgg = (cgg1 * dvgsteff_dvg + cgb1 * i.dvbseff_dvg) - cbg;
    let cgd = (cgd1 + cgg1 * dvgsteff_dvd + cgb1 * i.dvbseff_dvd) - cbd;
    let cgb = (cgb1 * i.dvbseff_dvb + cgg1 * dvgsteff_dvb) - cbb;
    let cge = (cgg1 * dvgsteff_dve + cgb1 * i.dvbseff_dve) - cbe;

    // Intrinsic capacitance matrix (b3soipdld.c:3400-3429).
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

    // --- Intrinsic S/D junction charge (b3soipdld.c:3438-3494) ---
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

    // --- Extrinsic bottom S/D-to-substrate charge (b3soipdld.c:3496-3609) ---
    let nsub_pos_type = (p.nsub > 0.0 && m.mtype > 0.0) || (p.nsub < 0.0 && m.mtype < 0.0);
    let t10 = -m.mtype * i.ves_raw; // vse without type
    let (mut qse, gcse) = extrinsic_sd_charge(p, t10, nsub_pos_type, true);
    let t11 = m.mtype * (i.vds_raw - i.ves_raw); // vde without type
    let (mut qde, gcde) = extrinsic_sd_charge(p, t11, nsub_pos_type, false);

    // Sidewall fringing (b3soipdld.c:3600-3604).
    qse += p.csesw * t10;
    let gcse = gcse + p.csesw;
    qde += p.cdesw * t11;
    let gcde = gcde + p.cdesw;

    // qse/qde carry true polarity -> premultiply by type (b3soipdld.c:3606-3609).
    let qse = m.mtype * qse;
    let qde = m.mtype * qde;

    // --- Overlap (Meyer fringing) charges (b3soipdld.c:3655-3784) ---
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
    // (b3soipdld.c:3679-3781). `ag0` is applied later by the device.
    if mode > 0 {
        // Charge lumping (b3soipdld.c:3722-3729).
        qgate += qgd + qgs + qge;
        qdrn += qde - qgd;
        qsub -= qge + qse + qde;

        B3SoiPdCharge {
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
        // Inverse mode (b3soipdld.c:3732-3781): D/S roles swap in the matrix.
        qgate += qgd + qgs + qge;
        let qsrc = qdrn - qgs + qse;
        qsub -= qge + qse + qde;
        qdrn = -(qgate + qbody + qsrc + qsub);

        B3SoiPdCharge {
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
/// (b3soipdld.c:3499-3598). `v` is the substrate-to-S (or substrate-to-D)
/// voltage with no type conversion; returns `(charge, conductance)`.
#[inline]
fn extrinsic_sd_charge(
    p: &B3SoiPdSized,
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
