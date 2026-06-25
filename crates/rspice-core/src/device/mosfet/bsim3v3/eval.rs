//! BSIM3v3.3 load equations (faithful port of ngspice-46 `b3ld.c`).
//!
//! This module transcribes the evaluation core of `BSIM3load`:
//!
//! - the source/drain junction diode DC model with the `ijth` linearization
//!   (b3ld.c:376-482),
//! - the normal/inverse mode select (484-497) and the whole DC current path:
//!   `Vbseff`, the Vth chain (short/narrow channel, body effect through the
//!   `sqrtPhis` machinery, DIBL, temperature), subthreshold `n`, poly-gate
//!   depletion, the `Vgsteff` weak-to-strong smoothing, effective W/Rds,
//!   `Abulk`, MOBMOD 1/2/3 mobility, `Vdsat`, `Vdseff`, the Early-voltage
//!   stack (`Vasat`/`VACLM`/`VADIBL`/`Va`/`VASCBE`), `Ids` with `Gm`/`Gds`/
//!   `Gmb`, and the substrate current `Isub` (499-1250),
//! - the CAPMOD=0/1/2 intrinsic charge models (1265-1951), the CAPMOD=3
//!   charge-thickness intrinsic charge model (1955-2239), the
//!   junction depletion charges (2253-2431), the capMod-specific overlap
//!   charges (2497-2557), and the mode-dependent node-charge assembly for
//!   `nqsMod = 0` (2560-2594, 2676-2710).
//!
//! All derivatives are the analytic expressions of the C source, transcribed
//! with the same temporary structure (`t0`..`t12`, `tmp*`) so the Rust can be
//! diffed against `b3ld.c` line by line.
//!
//! Scope notes:
//! - Transient NQS (`nqsMod` = 1) computes the charge-deficit state terms;
//!   AC-only charge-deficit (`acnqsMod` = 1) computes `taunet` for the AC
//!   matrix correction.
//! - `gmin` is an explicit argument (ngspice `CKTgmin`): the diode currents
//!   and conductances include it exactly as the C does, so a caller wiring
//!   this into an engine must not add a second per-device gmin on top.

use super::common::{CHARGE_Q, DELTA_1, DELTA_3, DELTA_4, EPSSI, EXP_THRESHOLD, MAX_EXP, MIN_EXP};
use super::params::Bsim3v3Model;
use super::temp::{Bsim3v3InstTemp, Bsim3v3ModelTemp, Bsim3v3SizeDep};
use crate::Value;

/// Input branch voltages, in device polarity (`mtype` folded in by the
/// caller) and already limited — the `vbs`/`vgs`/`vds` of b3ld.c:371 after
/// `DEVfetlim`/`DEVlimvds`/`DEVpnjlim`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bsim3v3Bias {
    pub vds: Value,
    pub vgs: Value,
    pub vbs: Value,
}

/// Linearized operating point of one BSIM3v3.3 instance.
///
/// Field names mirror the `here->BSIM3*` slots that ngspice fills during the
/// load and consumes when stamping. All quantities are in the device's own
/// polarity; `mode` records the normal(+1)/inverse(-1) channel direction.
/// None of the fields carries the `m` multiplier — ngspice applies `m` at
/// stamp time and so must the caller.
#[derive(Debug, Clone, Default)]
pub struct Bsim3v3Op {
    pub mode: i32,

    /// Channel current `Ids` (`BSIM3cd`, the `cdrain` of the load).
    pub cd: Value,
    /// Substrate (impact-ionization) current (`BSIM3csub`).
    pub csub: Value,
    /// Source junction diode current (`BSIM3cbs`).
    pub cbs: Value,
    /// Drain junction diode current (`BSIM3cbd`).
    pub cbd: Value,

    // Channel-current linearization (B3ld stores these mode-unswapped; the
    // stamp applies the swap).
    pub gm: Value,
    pub gds: Value,
    pub gmbs: Value,
    // Junction diode conductances.
    pub gbd: Value,
    pub gbs: Value,
    // Substrate-current linearization (`BSIM3gbbs`/`gbgs`/`gbds`).
    pub gbbs: Value,
    pub gbgs: Value,
    pub gbds: Value,

    /// Threshold voltage at the operating point (`BSIM3von`).
    pub von: Value,
    /// Saturation voltage (`BSIM3vdsat`).
    pub vdsat: Value,
    /// Effective mobility (`BSIM3ueff`).
    pub ueff: Value,
    /// `Vgsteff` of the DC path (`BSIM3Vgsteff`).
    pub vgsteff: Value,
    /// `Vdseff` (`BSIM3Vdseff`).
    pub vdseff: Value,
    /// `Abulk` (`BSIM3Abulk`).
    pub abulk: Value,
    /// Bias-dependent S/D series resistance (`BSIM3rds`).
    pub rds: Value,
    /// `dvt0 * Theta0` (`BSIM3thetavth`).
    pub thetavth: Value,
    /// `Abulk / (Vgsteff + 2 vtm)` (`BSIM3AbovVgst2Vtm`).
    pub ab_ov_vgst2vtm: Value,
    /// Inversion-charge proxy for noise (`BSIM3qinv`).
    pub qinv: Value,

    /// Charge state (set only when [`eval`] is asked to compute it).
    pub charge: Option<Bsim3v3Charge>,
}

/// Charge-model output for one BSIM3v3.3 instance.
///
/// `qgate`/`qbulk`/`qdrn` are the node charges *after* the overlap lumping of
/// b3ld.c:2582-2588 (mode > 0) / 2698-2704 (mode < 0): they correspond to the
/// quantities ngspice integrates, with `qsrc = -(qgate + qbulk + qdrn)`. The
/// CKTstate composition adds the junction charges:
/// `qg = qgate`, `qd = qdrn - qbd`, `qb = qbulk + qbd + qbs` (b3ld.c:2796-2801).
///
/// The `c***` entries are the intrinsic capacitance matrix exactly as stored
/// in `here->BSIM3c***` (before the `ag0`/mode assembly of the transient
/// companion); `capbd`/`capbs` the junction depletion capacitances; `cgdo`/
/// `cgso` the bias-dependent effective overlap capacitances of b3ld.c:2556.
#[derive(Debug, Clone, Default)]
pub struct Bsim3v3Charge {
    pub qgate: Value,
    pub qbulk: Value,
    pub qdrn: Value,
    /// Intrinsic channel drain charge before overlap/NQS node-charge assembly.
    pub qdrn_channel: Value,
    /// Source junction depletion charge (`CKTstate qbs`).
    pub qbs: Value,
    /// Drain junction depletion charge (`CKTstate qbd`).
    pub qbd: Value,

    pub capbs: Value,
    pub capbd: Value,

    pub cggb: Value,
    pub cgdb: Value,
    pub cgsb: Value,
    pub cdgb: Value,
    pub cddb: Value,
    pub cdsb: Value,
    pub cbgb: Value,
    pub cbdb: Value,
    pub cbsb: Value,

    /// Effective gate-drain overlap capacitance (`here->BSIM3cgdo`).
    pub cgdo: Value,
    /// Effective gate-source overlap capacitance (`here->BSIM3cgso`).
    pub cgso: Value,
    /// Gate-bulk overlap capacitance (`pParam->BSIM3cgbo`, bias-independent).
    pub cgbo: Value,
    /// Channel charge-equivalent state (`here->BSIM3qcheq`).
    pub qcheq: Value,
    /// Oxide capacitance over effective channel area.
    pub cox_wl: Value,
    /// Transient NQS relaxation conductance (`here->BSIM3gtau`).
    pub gtau: Value,
    /// Charge-deficit row capacitance derivatives (`here->BSIM3cq**`).
    pub cqgb: Value,
    pub cqdb: Value,
    pub cqsb: Value,
    pub cqbb: Value,
    /// AC-NQS time constant (`here->BSIM3taunet`), nonzero only for `ACNQSMOD=1`.
    pub taunet: Value,
}

impl Bsim3v3Charge {
    /// State-vector gate charge (`CKTstate0 qg`).
    pub fn qg_state(&self) -> Value {
        self.qgate
    }
    /// State-vector drain charge (`CKTstate0 qd = qdrn - qbd`).
    pub fn qd_state(&self) -> Value {
        self.qdrn - self.qbd
    }
    /// State-vector bulk charge (`CKTstate0 qb = qbulk + qbd + qbs`).
    pub fn qb_state(&self) -> Value {
        self.qbulk + self.qbd + self.qbs
    }
}

/// Evaluate the DC operating point only (no charges) — infallible.
pub fn eval_dc(
    model: &Bsim3v3Model,
    mt: &Bsim3v3ModelTemp,
    p: &Bsim3v3SizeDep,
    inst: &Bsim3v3InstTemp,
    bias: Bsim3v3Bias,
    gmin: Value,
) -> Bsim3v3Op {
    match eval(model, mt, p, inst, bias, gmin, false) {
        Ok(op) => op,
        // compute_charges = false cannot hit the capMod guard.
        Err(e) => unreachable!("BSIM3 DC eval cannot fail: {e}"),
    }
}

/// Evaluate the BSIM3v3.3 operating point, optionally including the charge
/// model (`compute_charges == true`, the `ChargeComputationNeeded` path).
///
/// Errors only when charges are requested with an invalid `capMod`.
#[allow(clippy::too_many_lines)]
pub fn eval(
    model: &Bsim3v3Model,
    mt: &Bsim3v3ModelTemp,
    p: &Bsim3v3SizeDep,
    inst: &Bsim3v3InstTemp,
    bias: Bsim3v3Bias,
    gmin: Value,
    compute_charges: bool,
) -> Result<Bsim3v3Op, String> {
    if compute_charges && !(0..=3).contains(&model.cap_mod) {
        return Err(format!(
            "BSIM3: CAPMOD={} charge model is invalid (expected 0-3)",
            model.cap_mod
        ));
    }

    let mut op = Bsim3v3Op::default();

    let vds = bias.vds;
    let vgs = bias.vgs;
    let vbs = bias.vbs;
    let vbd = vbs - vds;
    let vgd = vgs - vds;
    let vgb = vgs - vbs;

    // --- Source/drain junction diode DC model (b3ld.c:376-482) ---
    let nvtm = mt.vtm * model.jct_emission_coeff;
    let source_sat_current = inst.source_sat_current;
    let drain_sat_current = inst.drain_sat_current;

    if source_sat_current <= 0.0 {
        op.gbs = gmin;
        op.cbs = op.gbs * vbs;
    } else if model.ijth == 0.0 {
        let evbs = (vbs / nvtm).exp();
        op.gbs = source_sat_current * evbs / nvtm + gmin;
        op.cbs = source_sat_current * (evbs - 1.0) + gmin * vbs;
    } else {
        // inst.vjsm is Some whenever SatCurrent > 0 and ijth > 0 (b3temp.c).
        let (vjsm, is_evjsm) = inst.vjsm.expect("vjsm anchors exist when ijth > 0");
        if vbs < vjsm {
            let evbs = (vbs / nvtm).exp();
            op.gbs = source_sat_current * evbs / nvtm + gmin;
            op.cbs = source_sat_current * (evbs - 1.0) + gmin * vbs;
        } else {
            let t0 = is_evjsm / nvtm;
            op.gbs = t0 + gmin;
            op.cbs = is_evjsm - source_sat_current + t0 * (vbs - vjsm) + gmin * vbs;
        }
    }

    if drain_sat_current <= 0.0 {
        op.gbd = gmin;
        op.cbd = op.gbd * vbd;
    } else if model.ijth == 0.0 {
        let evbd = (vbd / nvtm).exp();
        op.gbd = drain_sat_current * evbd / nvtm + gmin;
        op.cbd = drain_sat_current * (evbd - 1.0) + gmin * vbd;
    } else {
        let (vjdm, is_evjdm) = inst.vjdm.expect("vjdm anchors exist when ijth > 0");
        if vbd < vjdm {
            let evbd = (vbd / nvtm).exp();
            op.gbd = drain_sat_current * evbd / nvtm + gmin;
            op.cbd = drain_sat_current * (evbd - 1.0) + gmin * vbd;
        } else {
            let t0 = is_evjdm / nvtm;
            op.gbd = t0 + gmin;
            op.cbd = is_evjdm - drain_sat_current + t0 * (vbd - vjdm) + gmin * vbd;
        }
    }

    // --- Mode select (b3ld.c:484-497) ---
    let (mode, vds_m, vgs_m, vbs_m);
    if vds >= 0.0 {
        mode = 1;
        vds_m = vds;
        vgs_m = vgs;
        vbs_m = vbs;
    } else {
        mode = -1;
        vds_m = -vds;
        vgs_m = vgd;
        vbs_m = vbd;
    }
    op.mode = mode;
    // From here on, the C's `Vds`/`Vgs`/`Vbs` are the mode-swapped values.
    let (vds_c, vgs_c, vbs_c) = (vds_m, vgs_m, vbs_m);

    // --- Vbseff (b3ld.c:499-505) ---
    let t0 = vbs_c - p.vbsc - 0.001;
    let t1 = (t0 * t0 - 0.004 * p.vbsc).sqrt();
    let mut vbseff = p.vbsc + 0.5 * (t0 + t1);
    let dvbseff_dvb = 0.5 * (1.0 + t0 / t1);
    if vbseff < vbs_c {
        vbseff = vbs_c;
    }

    // --- Phis / sqrtPhis (507-522) ---
    let (phis, dphis_dvb, sqrt_phis, dsqrt_phis_dvb);
    if vbseff > 0.0 {
        let t0 = p.phi / (p.phi + vbseff);
        phis = p.phi * t0;
        dphis_dvb = -t0 * t0;
        sqrt_phis = p.phis3 / (p.phi + 0.5 * vbseff);
        dsqrt_phis_dvb = -0.5 * sqrt_phis * sqrt_phis / p.phis3;
    } else {
        phis = p.phi - vbseff;
        dphis_dvb = -1.0;
        sqrt_phis = phis.sqrt();
        dsqrt_phis_dvb = -0.5 / sqrt_phis;
    }
    let xdep = p.xdep0 * sqrt_phis / p.sqrt_phi;
    let dxdep_dvb = (p.xdep0 / p.sqrt_phi) * dsqrt_phis_dvb;

    let leff = p.leff;
    let vtm = mt.vtm;

    // --- Vth (526-621) ---
    let t3 = xdep.sqrt();
    let v0 = p.vbi - p.phi;

    let t0 = p.dvt2 * vbseff;
    let (t1, t2);
    if t0 >= -0.5 {
        t1 = 1.0 + t0;
        t2 = p.dvt2;
    } else {
        // Avoid discontinuity problems caused by dvt2.
        let t4 = 1.0 / (3.0 + 8.0 * t0);
        t1 = (1.0 + 3.0 * t0) * t4;
        t2 = p.dvt2 * t4 * t4;
    }
    let lt1 = mt.factor1 * t3 * t1;
    let dlt1_dvb = mt.factor1 * (0.5 / t3 * t1 * dxdep_dvb + t3 * t2);

    let t0 = p.dvt2w * vbseff;
    let (t1, t2);
    if t0 >= -0.5 {
        t1 = 1.0 + t0;
        t2 = p.dvt2w;
    } else {
        let t4 = 1.0 / (3.0 + 8.0 * t0);
        t1 = (1.0 + 3.0 * t0) * t4;
        t2 = p.dvt2w * t4 * t4;
    }
    let ltw = mt.factor1 * t3 * t1;
    let dltw_dvb = mt.factor1 * (0.5 / t3 * t1 * dxdep_dvb + t3 * t2);

    let t0 = -0.5 * p.dvt1 * leff / lt1;
    let (theta0, dtheta0_dvb);
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

    op.thetavth = p.dvt0 * theta0;
    let delt_vth = op.thetavth * v0;
    let ddelt_vth_dvb = p.dvt0 * dtheta0_dvb * v0;

    let t0 = -0.5 * p.dvt1w * p.weff * leff / ltw;
    let (t2w, dt2w_dvb);
    if t0 > -EXP_THRESHOLD {
        let t1 = t0.exp();
        let t2i = t1 * (1.0 + 2.0 * t1);
        let dt1_dvb = -t0 / ltw * t1 * dltw_dvb;
        t2w = t2i;
        dt2w_dvb = (1.0 + 4.0 * t1) * dt1_dvb;
    } else {
        let t1 = MIN_EXP;
        t2w = t1 * (1.0 + 2.0 * t1);
        dt2w_dvb = 0.0;
    }
    let t0 = p.dvt0w * t2w;
    let t2 = t0 * v0;
    let dt2_dvb = p.dvt0w * dt2w_dvb * v0;

    let temp_ratio = mt.temp_ratio_m1;
    let t0 = (1.0 + p.nlx / leff).sqrt();
    let t1 =
        p.k1ox * (t0 - 1.0) * p.sqrt_phi + (p.kt1 + p.kt1l / leff + p.kt2 * vbseff) * temp_ratio;
    let tmp2 = model.tox * p.phi / (p.weff + p.w0);

    let mut t3v = p.eta0 + p.etab * vbseff;
    let t4;
    if t3v < 1.0e-4 {
        // Avoid discontinuity problems caused by etab.
        let t9 = 1.0 / (3.0 - 2.0e4 * t3v);
        t3v = (2.0e-4 - t3v) * t9;
        t4 = t9 * t9;
    } else {
        t4 = 1.0;
    }
    let ddibl_sft_dvd = t3v * p.theta0vb0;
    let dibl_sft = ddibl_sft_dvd * vds_c;

    let vth = model.mtype * inst.vth0 - p.k1 * p.sqrt_phi + p.k1ox * sqrt_phis
        - p.k2ox * vbseff
        - delt_vth
        - t2
        + (p.k3 + p.k3b * vbseff) * tmp2
        + t1
        - dibl_sft;
    op.von = vth;

    let dvth_dvb = p.k1ox * dsqrt_phis_dvb - p.k2ox - ddelt_vth_dvb - dt2_dvb + p.k3b * tmp2
        - p.etab * vds_c * p.theta0vb0 * t4
        + p.kt2 * temp_ratio;
    let dvth_dvd = -ddibl_sft_dvd;

    // --- Calculate n (623-642) ---
    let tmp2 = p.nfactor * EPSSI / xdep;
    let tmp3 = p.cdsc + p.cdscb * vbseff + p.cdscd * vds_c;
    let tmp4 = (tmp2 + tmp3 * theta0 + p.cit) / model.cox;
    let (n, dn_dvb, dn_dvd);
    if tmp4 >= -0.5 {
        n = 1.0 + tmp4;
        dn_dvb = (-tmp2 / xdep * dxdep_dvb + tmp3 * dtheta0_dvb + p.cdscb * theta0) / model.cox;
        dn_dvd = p.cdscd * theta0 / model.cox;
    } else {
        // Avoid discontinuity problems caused by tmp4.
        let t0 = 1.0 / (3.0 + 8.0 * tmp4);
        n = (1.0 + 3.0 * tmp4) * t0;
        let t0sq = t0 * t0;
        dn_dvb =
            (-tmp2 / xdep * dxdep_dvb + tmp3 * dtheta0_dvb + p.cdscb * theta0) / model.cox * t0sq;
        dn_dvd = p.cdscd * theta0 / model.cox * t0sq;
    }

    // --- Poly Gate Si Depletion Effect (644-663) ---
    let t0 = inst.vfb + p.phi;
    let (vgs_eff, dvgs_eff_dvg);
    if p.ngate > 1.0e18 && p.ngate < 1.0e25 && vgs_c > t0 {
        let t1 = 1.0e6 * CHARGE_Q * EPSSI * p.ngate / (model.cox * model.cox);
        let t4 = (1.0 + 2.0 * (vgs_c - t0) / t1).sqrt();
        let t2 = t1 * (t4 - 1.0);
        let t3 = 0.5 * t2 * t2 / t1; // Vpoly
        let t7 = 1.12 - t3 - 0.05;
        let t6 = (t7 * t7 + 0.224).sqrt();
        let t5 = 1.12 - 0.5 * (t7 + t6);
        vgs_eff = vgs_c - t5;
        dvgs_eff_dvg = 1.0 - (0.5 - 0.5 / t4) * (1.0 + t7 / t6);
    } else {
        vgs_eff = vgs_c;
        dvgs_eff_dvg = 1.0;
    }
    let vgst = vgs_eff - vth;

    // --- Effective Vgst (Vgsteff) Calculation (666-711) ---
    let t10 = 2.0 * n * vtm;
    let vgst_nvt = vgst / t10;
    let exp_arg = (2.0 * p.voff - vgst) / t10;

    let (vgsteff, mut dvgsteff_dvg, dvgsteff_dvd, dvgsteff_dvb);
    if vgst_nvt > EXP_THRESHOLD {
        vgsteff = vgst;
        dvgsteff_dvg = dvgs_eff_dvg;
        dvgsteff_dvd = -dvth_dvd;
        dvgsteff_dvb = -dvth_dvb;
    } else if exp_arg > EXP_THRESHOLD {
        let t0 = (vgst - p.voff) / (n * vtm);
        let exp_vgst = t0.exp();
        vgsteff = vtm * p.cdep0 / model.cox * exp_vgst;
        dvgsteff_dvg = vgsteff / (n * vtm);
        dvgsteff_dvd = -dvgsteff_dvg * (dvth_dvd + t0 * vtm * dn_dvd);
        dvgsteff_dvb = -dvgsteff_dvg * (dvth_dvb + t0 * vtm * dn_dvb);
        dvgsteff_dvg *= dvgs_eff_dvg;
    } else {
        let exp_vgst = vgst_nvt.exp();
        let t1 = t10 * (1.0 + exp_vgst).ln();
        let dt1_dvg = exp_vgst / (1.0 + exp_vgst);
        let dt1_dvb = -dt1_dvg * (dvth_dvb + vgst / n * dn_dvb) + t1 / n * dn_dvb;
        let dt1_dvd = -dt1_dvg * (dvth_dvd + vgst / n * dn_dvd) + t1 / n * dn_dvd;

        let dt2_dvg = -model.cox / (vtm * p.cdep0) * exp_arg.exp();
        let t2 = 1.0 - t10 * dt2_dvg;
        let dt2_dvd =
            -dt2_dvg * (dvth_dvd - 2.0 * vtm * exp_arg * dn_dvd) + (t2 - 1.0) / n * dn_dvd;
        let dt2_dvb =
            -dt2_dvg * (dvth_dvb - 2.0 * vtm * exp_arg * dn_dvb) + (t2 - 1.0) / n * dn_dvb;

        vgsteff = t1 / t2;
        let t3 = t2 * t2;
        dvgsteff_dvg = (t2 * dt1_dvg - t1 * dt2_dvg) / t3 * dvgs_eff_dvg;
        dvgsteff_dvd = (t2 * dt1_dvd - t1 * dt2_dvd) / t3;
        dvgsteff_dvb = (t2 * dt1_dvb - t1 * dt2_dvb) / t3;
    }
    op.vgsteff = vgsteff;

    // --- Effective Channel Geometry (713-743) ---
    let t9 = sqrt_phis - p.sqrt_phi;
    let mut weff = p.weff - 2.0 * (p.dwg * vgsteff + p.dwb * t9);
    let mut dweff_dvg = -2.0 * p.dwg;
    let mut dweff_dvb = -2.0 * p.dwb * dsqrt_phis_dvb;

    if weff < 2.0e-8 {
        // Avoid the discontinuity problem due to Weff.
        let t0 = 1.0 / (6.0e-8 - 2.0 * weff);
        weff = 2.0e-8 * (4.0e-8 - weff) * t0;
        let t0 = t0 * t0 * 4.0e-16;
        dweff_dvg *= t0;
        dweff_dvb *= t0;
    }

    let t0 = p.prwg * vgsteff + p.prwb * t9;
    let (rds, drds_dvg, drds_dvb);
    if t0 >= -0.9 {
        rds = p.rds0 * (1.0 + t0);
        drds_dvg = p.rds0 * p.prwg;
        drds_dvb = p.rds0 * p.prwb * dsqrt_phis_dvb;
    } else {
        // Avoid the discontinuity problem due to prwg and prwb.
        let t1 = 1.0 / (17.0 + 20.0 * t0);
        rds = p.rds0 * (0.8 + t0) * t1;
        let t1 = t1 * t1;
        drds_dvg = p.rds0 * p.prwg * t1;
        drds_dvb = p.rds0 * p.prwb * dsqrt_phis_dvb * t1;
    }
    op.rds = rds; // Noise bugfix.

    // --- Abulk (745-800) ---
    let t1 = 0.5 * p.k1ox / sqrt_phis;
    let dt1_dvb = -t1 / sqrt_phis * dsqrt_phis_dvb;

    let t9 = (p.xj * xdep).sqrt();
    let tmp1 = leff + 2.0 * t9;
    let t5 = leff / tmp1;
    let tmp2 = p.a0 * t5;
    let tmp3 = p.weff + p.b1;
    let tmp4 = p.b0 / tmp3;
    let t2 = tmp2 + tmp4;
    let dt2_dvb = -t9 / tmp1 / xdep * dxdep_dvb;
    let t6 = t5 * t5;
    let t7 = t5 * t6;

    let mut abulk0 = 1.0 + t1 * t2;
    let mut dabulk0_dvb = t1 * tmp2 * dt2_dvb + t2 * dt1_dvb;

    let t8 = p.ags * p.a0 * t7;
    let mut dabulk_dvg = -t1 * t8;
    let mut abulk = abulk0 + dabulk_dvg * vgsteff;
    let mut dabulk_dvb = dabulk0_dvb - t8 * vgsteff * (dt1_dvb + 3.0 * t1 * dt2_dvb);

    if abulk0 < 0.1 {
        // Avoid the problems caused by Abulk0.
        let t9 = 1.0 / (3.0 - 20.0 * abulk0);
        abulk0 = (0.2 - abulk0) * t9;
        dabulk0_dvb *= t9 * t9;
    }
    if abulk < 0.1 {
        // Avoid the problems caused by Abulk.
        let t9 = 1.0 / (3.0 - 20.0 * abulk);
        abulk = (0.2 - abulk) * t9;
        let t10 = t9 * t9;
        dabulk_dvb *= t10;
        dabulk_dvg *= t10;
    }
    op.abulk = abulk;

    let t2 = p.keta * vbseff;
    let (t0, dt0_dvb);
    if t2 >= -0.9 {
        t0 = 1.0 / (1.0 + t2);
        dt0_dvb = -p.keta * t0 * t0;
    } else {
        // Avoid the problems caused by Keta.
        let t1 = 1.0 / (0.8 + t2);
        t0 = (17.0 + 20.0 * t2) * t1;
        dt0_dvb = -p.keta * t1 * t1;
    }
    dabulk_dvg *= t0;
    dabulk_dvb = dabulk_dvb * t0 + abulk * dt0_dvb;
    dabulk0_dvb = dabulk0_dvb * t0 + abulk0 * dt0_dvb;
    abulk *= t0;
    abulk0 *= t0;

    // --- Mobility calculation (803-851) ---
    let (t5m, mut ddenomi_dvg, mut ddenomi_dvd, mut ddenomi_dvb);
    if model.mob_mod == 1 {
        let t0 = vgsteff + vth + vth;
        let t2 = p.ua + p.uc * vbseff;
        let t3 = t0 / model.tox;
        t5m = t3 * (t2 + p.ub * t3);
        ddenomi_dvg = (t2 + 2.0 * p.ub * t3) / model.tox;
        ddenomi_dvd = ddenomi_dvg * 2.0 * dvth_dvd;
        ddenomi_dvb = ddenomi_dvg * 2.0 * dvth_dvb + p.uc * t3;
    } else if model.mob_mod == 2 {
        t5m = vgsteff / model.tox * (p.ua + p.uc * vbseff + p.ub * vgsteff / model.tox);
        ddenomi_dvg = (p.ua + p.uc * vbseff + 2.0 * p.ub * vgsteff / model.tox) / model.tox;
        ddenomi_dvd = 0.0;
        ddenomi_dvb = vgsteff * p.uc / model.tox;
    } else {
        let t0 = vgsteff + vth + vth;
        let t2 = 1.0 + p.uc * vbseff;
        let t3 = t0 / model.tox;
        let t4 = t3 * (p.ua + p.ub * t3);
        t5m = t4 * t2;
        ddenomi_dvg = (p.ua + 2.0 * p.ub * t3) * t2 / model.tox;
        ddenomi_dvd = ddenomi_dvg * 2.0 * dvth_dvd;
        ddenomi_dvb = ddenomi_dvg * 2.0 * dvth_dvb + p.uc * t4;
    }

    let denomi;
    if t5m >= -0.8 {
        denomi = 1.0 + t5m;
    } else {
        // Avoid the discontinuity problem caused by ua and ub.
        let t9 = 1.0 / (7.0 + 10.0 * t5m);
        denomi = (0.6 + t5m) * t9;
        let t9 = t9 * t9;
        ddenomi_dvg *= t9;
        ddenomi_dvd *= t9;
        ddenomi_dvb *= t9;
    }

    let ueff = inst.u0temp / denomi;
    op.ueff = ueff;
    let t9 = -ueff / denomi;
    let dueff_dvg = t9 * ddenomi_dvg;
    let dueff_dvd = t9 * ddenomi_dvd;
    let dueff_dvb = t9 * ddenomi_dvb;

    // --- Saturation Drain Voltage Vdsat (853-957) ---
    let wv_cox = weff * p.vsattemp * model.cox;
    let wv_cox_rds = wv_cox * rds;

    let esat = 2.0 * p.vsattemp / ueff;
    let esat_l = esat * leff;
    let t0 = -esat_l / ueff;
    let desat_l_dvg = t0 * dueff_dvg;
    let desat_l_dvd = t0 * dueff_dvd;
    let desat_l_dvb = t0 * dueff_dvb;

    let a1 = p.a1;
    let (lambda, dlambda_dvg);
    if a1 == 0.0 {
        lambda = p.a2;
        dlambda_dvg = 0.0;
    } else if a1 > 0.0 {
        // Avoid the discontinuity problem caused by a1 and a2 (Lambda).
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

    let vgst2vtm = vgsteff + 2.0 * vtm;
    op.ab_ov_vgst2vtm = abulk / vgst2vtm;

    let (tmp2, tmp3);
    if rds > 0.0 {
        tmp2 = drds_dvg / rds + dweff_dvg / weff;
        tmp3 = drds_dvb / rds + dweff_dvb / weff;
    } else {
        tmp2 = dweff_dvg / weff;
        tmp3 = dweff_dvb / weff;
    }
    let (vdsat, dvdsat_dvg, dvdsat_dvd, dvdsat_dvb, tmp1l);
    if rds == 0.0 && lambda == 1.0 {
        let t0 = 1.0 / (abulk * esat_l + vgst2vtm);
        tmp1l = 0.0;
        let t1 = t0 * t0;
        let t2 = vgst2vtm * t0;
        let t3 = esat_l * vgst2vtm;
        vdsat = t3 * t0;

        let dt0_dvg = -(abulk * desat_l_dvg + esat_l * dabulk_dvg + 1.0) * t1;
        let dt0_dvd = -(abulk * desat_l_dvd) * t1;
        let dt0_dvb = -(abulk * desat_l_dvb + dabulk_dvb * esat_l) * t1;

        dvdsat_dvg = t3 * dt0_dvg + t2 * desat_l_dvg + esat_l * t0;
        dvdsat_dvd = t3 * dt0_dvd + t2 * desat_l_dvd;
        dvdsat_dvb = t3 * dt0_dvb + t2 * desat_l_dvb;
    } else {
        tmp1l = dlambda_dvg / (lambda * lambda);
        let t9 = abulk * wv_cox_rds;
        let t8 = abulk * t9;
        let t7 = vgst2vtm * t9;
        let t6 = vgst2vtm * wv_cox_rds;
        let t0 = 2.0 * abulk * (t9 - 1.0 + 1.0 / lambda);
        let dt0_dvg =
            2.0 * (t8 * tmp2 - abulk * tmp1l + (2.0 * t9 + 1.0 / lambda - 1.0) * dabulk_dvg);
        let dt0_dvb =
            2.0 * (t8 * (2.0 / abulk * dabulk_dvb + tmp3) + (1.0 / lambda - 1.0) * dabulk_dvb);
        let dt0_dvd = 0.0;

        let t1 = vgst2vtm * (2.0 / lambda - 1.0) + abulk * esat_l + 3.0 * t7;
        let dt1_dvg = (2.0 / lambda - 1.0) - 2.0 * vgst2vtm * tmp1l
            + abulk * desat_l_dvg
            + esat_l * dabulk_dvg
            + 3.0 * (t9 + t7 * tmp2 + t6 * dabulk_dvg);
        let dt1_dvb =
            abulk * desat_l_dvb + esat_l * dabulk_dvb + 3.0 * (t6 * dabulk_dvb + t7 * tmp3);
        let dt1_dvd = abulk * desat_l_dvd;

        let t2 = vgst2vtm * (esat_l + 2.0 * t6);
        let dt2_dvg = esat_l + vgst2vtm * desat_l_dvg + t6 * (4.0 + 2.0 * vgst2vtm * tmp2);
        let dt2_dvb = vgst2vtm * (desat_l_dvb + 2.0 * t6 * tmp3);
        let dt2_dvd = vgst2vtm * desat_l_dvd;

        let t3 = (t1 * t1 - 2.0 * t0 * t2).sqrt();
        vdsat = (t1 - t3) / t0;

        let dt3_dvg = (t1 * dt1_dvg - 2.0 * (t0 * dt2_dvg + t2 * dt0_dvg)) / t3;
        let dt3_dvd = (t1 * dt1_dvd - 2.0 * (t0 * dt2_dvd + t2 * dt0_dvd)) / t3;
        let dt3_dvb = (t1 * dt1_dvb - 2.0 * (t0 * dt2_dvb + t2 * dt0_dvb)) / t3;
        // dT3 terms are kept for parity with the C even though the closed
        // forms below re-expand them.
        let _ = (dt3_dvg, dt3_dvd, dt3_dvb);

        dvdsat_dvg =
            (dt1_dvg - (t1 * dt1_dvg - dt0_dvg * t2 - t0 * dt2_dvg) / t3 - vdsat * dt0_dvg) / t0;
        dvdsat_dvb =
            (dt1_dvb - (t1 * dt1_dvb - dt0_dvb * t2 - t0 * dt2_dvb) / t3 - vdsat * dt0_dvb) / t0;
        dvdsat_dvd = (dt1_dvd - (t1 * dt1_dvd - t0 * dt2_dvd) / t3) / t0;
    }
    op.vdsat = vdsat;

    // --- Effective Vds (Vdseff) Calculation (959-981) ---
    let t1 = vdsat - vds_c - p.delta;
    let dt1_dvg = dvdsat_dvg;
    let dt1_dvd = dvdsat_dvd - 1.0;
    let dt1_dvb = dvdsat_dvb;

    let t2 = (t1 * t1 + 4.0 * p.delta * vdsat).sqrt();
    let t0 = t1 / t2;
    let t3 = 2.0 * p.delta / t2;
    let dt2_dvg = t0 * dt1_dvg + t3 * dvdsat_dvg;
    let dt2_dvd = t0 * dt1_dvd + t3 * dvdsat_dvd;
    let dt2_dvb = t0 * dt1_dvb + t3 * dvdsat_dvb;

    let mut vdseff = vdsat - 0.5 * (t1 + t2);
    let mut dvdseff_dvg = dvdsat_dvg - 0.5 * (dt1_dvg + dt2_dvg);
    let dvdseff_dvd = dvdsat_dvd - 0.5 * (dt1_dvd + dt2_dvd);
    let mut dvdseff_dvb = dvdsat_dvb - 0.5 * (dt1_dvb + dt2_dvb);
    // Added to eliminate non-zero Vdseff at Vds=0.0.
    if vds_c == 0.0 {
        vdseff = 0.0;
        dvdseff_dvg = 0.0;
        dvdseff_dvb = 0.0;
    }

    // --- Calculate VAsat (983-1011) ---
    let tmp4 = 1.0 - 0.5 * abulk * vdsat / vgst2vtm;
    let t9 = wv_cox_rds * vgsteff;
    let t8 = t9 / vgst2vtm;
    let t0 = esat_l + vdsat + 2.0 * t9 * tmp4;

    let t7 = 2.0 * wv_cox_rds * tmp4;
    let dt0_dvg = desat_l_dvg + dvdsat_dvg + t7 * (1.0 + tmp2 * vgsteff)
        - t8 * (abulk * dvdsat_dvg - abulk * vdsat / vgst2vtm + vdsat * dabulk_dvg);
    let dt0_dvb = desat_l_dvb + dvdsat_dvb + t7 * tmp3 * vgsteff
        - t8 * (dabulk_dvb * vdsat + abulk * dvdsat_dvb);
    let dt0_dvd = desat_l_dvd + dvdsat_dvd - t8 * abulk * dvdsat_dvd;

    let t9 = wv_cox_rds * abulk;
    let t1 = 2.0 / lambda - 1.0 + t9;
    let dt1_dvg = -2.0 * tmp1l + wv_cox_rds * (abulk * tmp2 + dabulk_dvg);
    let dt1_dvb = dabulk_dvb * wv_cox_rds + t9 * tmp3;

    let vasat = t0 / t1;
    let dvasat_dvg = (dt0_dvg - vasat * dt1_dvg) / t1;
    let dvasat_dvb = (dt0_dvb - vasat * dt1_dvb) / t1;
    let dvasat_dvd = dt0_dvd / t1;

    if vdseff > vds_c {
        vdseff = vds_c;
    }
    let diff_vds = vds_c - vdseff;
    op.vdseff = vdseff;

    // --- Calculate VACLM (1013-1036) ---
    let (vaclm, dvaclm_dvg, dvaclm_dvd, dvaclm_dvb);
    if p.pclm > 0.0 && diff_vds > 1.0e-10 {
        let t0 = 1.0 / (p.pclm * abulk * p.litl);
        let dt0_dvb = -t0 / abulk * dabulk_dvb;
        let dt0_dvg = -t0 / abulk * dabulk_dvg;

        let t2 = vgsteff / esat_l;
        let t1 = leff * (abulk + t2);
        let dt1_dvg = leff * ((1.0 - t2 * desat_l_dvg) / esat_l + dabulk_dvg);
        let dt1_dvb = leff * (dabulk_dvb - t2 * desat_l_dvb / esat_l);
        // NOTE: the C divides by Esat (not EsatL) here; transcribed verbatim.
        let dt1_dvd = -t2 * desat_l_dvd / esat;

        let t9 = t0 * t1;
        vaclm = t9 * diff_vds;
        dvaclm_dvg = t0 * dt1_dvg * diff_vds - t9 * dvdseff_dvg + t1 * diff_vds * dt0_dvg;
        dvaclm_dvb = (dt0_dvb * t1 + t0 * dt1_dvb) * diff_vds - t9 * dvdseff_dvb;
        dvaclm_dvd = t0 * dt1_dvd * diff_vds + t9 * (1.0 - dvdseff_dvd);
    } else {
        vaclm = MAX_EXP;
        dvaclm_dvd = 0.0;
        dvaclm_dvg = 0.0;
        dvaclm_dvb = 0.0;
    }

    // --- Calculate VADIBL (1038-1082) ---
    let (mut vadibl, mut dvadibl_dvg, mut dvadibl_dvd, mut dvadibl_dvb);
    if p.theta_rout > 0.0 {
        let t8 = abulk * vdsat;
        let t0 = vgst2vtm * t8;
        let dt0_dvg = vgst2vtm * abulk * dvdsat_dvg + t8 + vgst2vtm * vdsat * dabulk_dvg;
        let dt0_dvb = vgst2vtm * (dabulk_dvb * vdsat + abulk * dvdsat_dvb);
        let dt0_dvd = vgst2vtm * abulk * dvdsat_dvd;

        let t1 = vgst2vtm + t8;
        let dt1_dvg = 1.0 + abulk * dvdsat_dvg + vdsat * dabulk_dvg;
        let dt1_dvb = abulk * dvdsat_dvb + dabulk_dvb * vdsat;
        let dt1_dvd = abulk * dvdsat_dvd;

        let t9 = t1 * t1;
        let t2 = p.theta_rout;
        vadibl = (vgst2vtm - t0 / t1) / t2;
        dvadibl_dvg = (1.0 - dt0_dvg / t1 + t0 * dt1_dvg / t9) / t2;
        dvadibl_dvb = (-dt0_dvb / t1 + t0 * dt1_dvb / t9) / t2;
        dvadibl_dvd = (-dt0_dvd / t1 + t0 * dt1_dvd / t9) / t2;

        let t7 = p.pdiblb * vbseff;
        if t7 >= -0.9 {
            let t3 = 1.0 / (1.0 + t7);
            vadibl *= t3;
            dvadibl_dvg *= t3;
            dvadibl_dvb = (dvadibl_dvb - vadibl * p.pdiblb) * t3;
            dvadibl_dvd *= t3;
        } else {
            // Avoid the discontinuity problem caused by pdiblcb.
            let t4 = 1.0 / (0.8 + t7);
            let t3 = (17.0 + 20.0 * t7) * t4;
            dvadibl_dvg *= t3;
            dvadibl_dvb = dvadibl_dvb * t3 - vadibl * p.pdiblb * t4 * t4;
            dvadibl_dvd *= t3;
            vadibl *= t3;
        }
    } else {
        vadibl = MAX_EXP;
        dvadibl_dvd = 0.0;
        dvadibl_dvg = 0.0;
        dvadibl_dvb = 0.0;
    }

    // --- Calculate VA (1084-1118) ---
    let t8 = p.pvag / esat_l;
    let t9 = t8 * vgsteff;
    let (t0, dt0_dvg, dt0_dvb, dt0_dvd);
    if t9 > -0.9 {
        t0 = 1.0 + t9;
        dt0_dvg = t8 * (1.0 - vgsteff * desat_l_dvg / esat_l);
        dt0_dvb = -t9 * desat_l_dvb / esat_l;
        dt0_dvd = -t9 * desat_l_dvd / esat_l;
    } else {
        // Avoid the discontinuity problems caused by pvag.
        let t1 = 1.0 / (17.0 + 20.0 * t9);
        t0 = (0.8 + t9) * t1;
        let t1 = t1 * t1;
        dt0_dvg = t8 * (1.0 - vgsteff * desat_l_dvg / esat_l) * t1;
        let t9 = t9 * t1 / esat_l;
        dt0_dvb = -t9 * desat_l_dvb;
        dt0_dvd = -t9 * desat_l_dvd;
    }

    let tmp1 = vaclm * vaclm;
    let tmp2v = vadibl * vadibl;
    let mut tmp3v = vaclm + vadibl;

    let t1 = vaclm * vadibl / tmp3v;
    tmp3v *= tmp3v;
    let dt1_dvg = (tmp1 * dvadibl_dvg + tmp2v * dvaclm_dvg) / tmp3v;
    let dt1_dvd = (tmp1 * dvadibl_dvd + tmp2v * dvaclm_dvd) / tmp3v;
    let dt1_dvb = (tmp1 * dvadibl_dvb + tmp2v * dvaclm_dvb) / tmp3v;

    let va = vasat + t0 * t1;
    let dva_dvg = dvasat_dvg + t1 * dt0_dvg + t0 * dt1_dvg;
    let dva_dvd = dvasat_dvd + t1 * dt0_dvd + t0 * dt1_dvd;
    let dva_dvb = dvasat_dvb + t1 * dt0_dvb + t0 * dt1_dvb;

    // --- Calculate VASCBE (1120-1139) ---
    let (vascbe, dvascbe_dvg, dvascbe_dvd, dvascbe_dvb);
    if p.pscbe2 > 0.0 {
        if diff_vds > p.pscbe1 * p.litl / EXP_THRESHOLD {
            let t0 = p.pscbe1 * p.litl / diff_vds;
            vascbe = leff * t0.exp() / p.pscbe2;
            let t1 = t0 * vascbe / diff_vds;
            dvascbe_dvg = t1 * dvdseff_dvg;
            dvascbe_dvd = -t1 * (1.0 - dvdseff_dvd);
            dvascbe_dvb = t1 * dvdseff_dvb;
        } else {
            vascbe = MAX_EXP * leff / p.pscbe2;
            dvascbe_dvg = 0.0;
            dvascbe_dvd = 0.0;
            dvascbe_dvb = 0.0;
        }
    } else {
        vascbe = MAX_EXP;
        dvascbe_dvg = 0.0;
        dvascbe_dvd = 0.0;
        dvascbe_dvb = 0.0;
    }

    // --- Calculate Ids (1141-1206) ---
    let cox_w_ov_l = model.cox * weff / leff;
    let beta = ueff * cox_w_ov_l;
    let dbeta_dvg = cox_w_ov_l * dueff_dvg + beta * dweff_dvg / weff;
    let dbeta_dvd = cox_w_ov_l * dueff_dvd;
    let dbeta_dvb = cox_w_ov_l * dueff_dvb + beta * dweff_dvb / weff;

    let t0 = 1.0 - 0.5 * abulk * vdseff / vgst2vtm;
    let dt0_dvg =
        -0.5 * (abulk * dvdseff_dvg - abulk * vdseff / vgst2vtm + vdseff * dabulk_dvg) / vgst2vtm;
    let dt0_dvd = -0.5 * abulk * dvdseff_dvd / vgst2vtm;
    let dt0_dvb = -0.5 * (abulk * dvdseff_dvb + dabulk_dvb * vdseff) / vgst2vtm;

    let fgche1 = vgsteff * t0;
    let dfgche1_dvg = vgsteff * dt0_dvg + t0;
    let dfgche1_dvd = vgsteff * dt0_dvd;
    let dfgche1_dvb = vgsteff * dt0_dvb;

    let t9 = vdseff / esat_l;
    let fgche2 = 1.0 + t9;
    let dfgche2_dvg = (dvdseff_dvg - t9 * desat_l_dvg) / esat_l;
    let dfgche2_dvd = (dvdseff_dvd - t9 * desat_l_dvd) / esat_l;
    let dfgche2_dvb = (dvdseff_dvb - t9 * desat_l_dvb) / esat_l;

    let gche = beta * fgche1 / fgche2;
    let dgche_dvg = (beta * dfgche1_dvg + fgche1 * dbeta_dvg - gche * dfgche2_dvg) / fgche2;
    let dgche_dvd = (beta * dfgche1_dvd + fgche1 * dbeta_dvd - gche * dfgche2_dvd) / fgche2;
    let dgche_dvb = (beta * dfgche1_dvb + fgche1 * dbeta_dvb - gche * dfgche2_dvb) / fgche2;

    let t0 = 1.0 + gche * rds;
    let t9 = vdseff / t0;
    let idl = gche * t9;

    let didl_dvg = (gche * dvdseff_dvg + t9 * dgche_dvg) / t0 - idl * gche / t0 * drds_dvg;
    let didl_dvd = (gche * dvdseff_dvd + t9 * dgche_dvd) / t0;
    let didl_dvb = (gche * dvdseff_dvb + t9 * dgche_dvb - idl * drds_dvb * gche) / t0;

    let t9 = diff_vds / va;
    let t0 = 1.0 + t9;
    let idsa = idl * t0;
    let didsa_dvg = t0 * didl_dvg - idl * (dvdseff_dvg + t9 * dva_dvg) / va;
    let didsa_dvd = t0 * didl_dvd + idl * (1.0 - dvdseff_dvd - t9 * dva_dvd) / va;
    let didsa_dvb = t0 * didl_dvb - idl * (dvdseff_dvb + t9 * dva_dvb) / va;

    let t9 = diff_vds / vascbe;
    let t0 = 1.0 + t9;
    let ids = idsa * t0;

    let mut gm = t0 * didsa_dvg - idsa * (dvdseff_dvg + t9 * dvascbe_dvg) / vascbe;
    let mut gds = t0 * didsa_dvd + idsa * (1.0 - dvdseff_dvd - t9 * dvascbe_dvd) / vascbe;
    let mut gmb = t0 * didsa_dvb - idsa * (dvdseff_dvb + t9 * dvascbe_dvb) / vascbe;

    gds += gm * dvgsteff_dvd;
    gmb += gm * dvgsteff_dvb;
    gm *= dvgsteff_dvg;
    gmb *= dvbseff_dvb;

    // --- Substrate current (1208-1239) ---
    let tmp = p.alpha0 + p.alpha1 * leff;
    let (isub, mut gbd_sub, mut gbb_sub, mut gbg_sub);
    if tmp <= 0.0 || p.beta0 <= 0.0 {
        isub = 0.0;
        gbd_sub = 0.0;
        gbb_sub = 0.0;
        gbg_sub = 0.0;
    } else {
        let t2 = tmp / leff;
        let (t1, dt1_dvg, dt1_dvd, dt1_dvb);
        if diff_vds > p.beta0 / EXP_THRESHOLD {
            let t0 = -p.beta0 / diff_vds;
            t1 = t2 * diff_vds * t0.exp();
            let t3 = t1 / diff_vds * (t0 - 1.0);
            dt1_dvg = t3 * dvdseff_dvg;
            dt1_dvd = t3 * (dvdseff_dvd - 1.0);
            dt1_dvb = t3 * dvdseff_dvb;
        } else {
            let t3 = t2 * MIN_EXP;
            t1 = t3 * diff_vds;
            dt1_dvg = -t3 * dvdseff_dvg;
            dt1_dvd = t3 * (1.0 - dvdseff_dvd);
            dt1_dvb = -t3 * dvdseff_dvb;
        }
        isub = t1 * idsa;
        gbg_sub = t1 * didsa_dvg + idsa * dt1_dvg;
        gbd_sub = t1 * didsa_dvd + idsa * dt1_dvd;
        gbb_sub = t1 * didsa_dvb + idsa * dt1_dvb;

        gbd_sub += gbg_sub * dvgsteff_dvd;
        gbb_sub += gbg_sub * dvgsteff_dvb;
        gbg_sub *= dvgsteff_dvg;
        gbb_sub *= dvbseff_dvb; // bug fixing
    }

    let cdrain = ids;
    op.gds = gds;
    op.gm = gm;
    op.gmbs = gmb;
    op.gbbs = gbb_sub;
    op.gbgs = gbg_sub;
    op.gbds = gbd_sub;
    op.csub = isub;
    op.cd = cdrain;

    if !compute_charges {
        // DC sweep / operating point without charge bookkeeping: ngspice
        // zeroes the c*** matrix and leaves qinv at 0 (b3ld.c:1255-1264).
        return Ok(op);
    }

    // ==================== Charge model ====================
    let mut charge = Bsim3v3Charge::default();
    let mut qgate;
    let mut qdrn;
    let mut qbulk;

    if model.xpart < 0.0 {
        // `xpart < 0`: intrinsic charges suppressed; overlap and junction
        // parts below still apply (b3ld.c:1255-1264).
        qgate = 0.0;
        qdrn = 0.0;
        qbulk = 0.0;
        op.qinv = 0.0;
    } else if model.cap_mod == 0 {
        // CAPMOD=0 intrinsic charge model (b3ld.c:1265-1583).
        let (vbseff0, dvbseff0_dvb);
        if vbseff < 0.0 {
            vbseff0 = vbs_c;
            dvbseff0_dvb = 1.0;
        } else {
            vbseff0 = p.phi - phis;
            dvbseff0_dvb = -dphis_dvb;
        }

        let vfb = p.vfbcv;
        let vth0_cv = vfb + p.phi + p.k1ox * sqrt_phis;
        let vgst0 = vgs_eff - vth0_cv;
        let dvth0_cv_dvb = p.k1ox * dsqrt_phis_dvb;
        let cox_wl = model.cox * p.weff_cv * p.leff_cv;
        let arg1 = vgs_eff - vbseff0 - vfb;

        if arg1 <= 0.0 {
            qgate = cox_wl * arg1;
            qbulk = -qgate;
            qdrn = 0.0;

            charge.cggb = cox_wl * dvgs_eff_dvg;
            charge.cgdb = 0.0;
            charge.cgsb = cox_wl * (dvbseff0_dvb - dvgs_eff_dvg);
            charge.cdgb = 0.0;
            charge.cddb = 0.0;
            charge.cdsb = 0.0;
            charge.cbgb = -cox_wl * dvgs_eff_dvg;
            charge.cbdb = 0.0;
            charge.cbsb = -charge.cgsb;
            op.qinv = 0.0;
        } else if vgst0 <= 0.0 {
            let t1 = 0.5 * p.k1ox;
            let t2 = (t1 * t1 + arg1).sqrt();
            qgate = cox_wl * p.k1ox * (t2 - t1);
            qbulk = -qgate;
            qdrn = 0.0;

            let t0 = cox_wl * t1 / t2;
            charge.cggb = t0 * dvgs_eff_dvg;
            charge.cgdb = 0.0;
            charge.cgsb = t0 * (dvbseff0_dvb - dvgs_eff_dvg);
            charge.cdgb = 0.0;
            charge.cddb = 0.0;
            charge.cdsb = 0.0;
            charge.cbgb = -charge.cggb;
            charge.cbdb = 0.0;
            charge.cbsb = -charge.cgsb;
            op.qinv = 0.0;
        } else {
            let one_third_cox_wl = cox_wl / 3.0;
            let two_third_cox_wl = 2.0 * one_third_cox_wl;

            let abulk_cv = abulk0 * p.abulk_cv_factor;
            let dabulk_cv_dvb = p.abulk_cv_factor * dabulk0_dvb;
            let vdsat0 = vgst0 / abulk_cv;
            let dvdsat_dvg = dvgs_eff_dvg / abulk_cv;
            let dvdsat_dvb = -(vdsat0 * dabulk_cv_dvb + dvth0_cv_dvb) / abulk_cv;

            if model.xpart > 0.5 {
                // 0/100 charge partition model.
                if vdsat0 <= vds_c {
                    let t1 = vdsat0 / 3.0;
                    qgate = cox_wl * (vgs_eff - vfb - p.phi - t1);
                    let t2 = -two_third_cox_wl * vgst0;
                    qbulk = -(qgate + t2);
                    qdrn = 0.0;

                    charge.cggb = one_third_cox_wl * (3.0 - dvdsat_dvg) * dvgs_eff_dvg;
                    let t2 = -one_third_cox_wl * dvdsat_dvb;
                    charge.cgsb = -(charge.cggb + t2);
                    charge.cgdb = 0.0;
                    charge.cdgb = 0.0;
                    charge.cddb = 0.0;
                    charge.cdsb = 0.0;
                    charge.cbgb = -(charge.cggb - two_third_cox_wl * dvgs_eff_dvg);
                    let t3 = -(t2 + two_third_cox_wl * dvth0_cv_dvb);
                    charge.cbsb = -(charge.cbgb + t3);
                    charge.cbdb = 0.0;
                    op.qinv = -(qgate + qbulk);
                } else {
                    let alphaz = vgst0 / vdsat0;
                    let t1 = 2.0 * vdsat0 - vds_c;
                    let t2 = vds_c / (3.0 * t1);
                    let t3 = t2 * vds_c;
                    let t9 = 0.25 * cox_wl;
                    let t4 = t9 * alphaz;
                    let mut t7 = 2.0 * vds_c - t1 - 3.0 * t3;
                    let mut t8 = t3 - t1 - 2.0 * vds_c;
                    qgate = cox_wl * (vgs_eff - vfb - p.phi - 0.5 * (vds_c - t3));
                    let t10 = t4 * t8;
                    qdrn = t4 * t7;
                    qbulk = -(qgate + qdrn + t10);

                    let t5 = t3 / t1;
                    charge.cggb = cox_wl * (1.0 - t5 * dvdsat_dvg) * dvgs_eff_dvg;
                    let t11 = -cox_wl * t5 * dvdsat_dvb;
                    charge.cgdb = cox_wl * (t2 - 0.5 + 0.5 * t5);
                    charge.cgsb = -(charge.cggb + t11 + charge.cgdb);
                    let t6 = 1.0 / vdsat0;
                    let d_alphaz_dvg = t6 * (1.0 - alphaz * dvdsat_dvg);
                    let d_alphaz_dvb = -t6 * (dvth0_cv_dvb + alphaz * dvdsat_dvb);
                    t7 *= t9;
                    t8 *= t9;
                    let t9 = 2.0 * t4 * (1.0 - 3.0 * t5);
                    charge.cdgb = (t7 * d_alphaz_dvg - t9 * dvdsat_dvg) * dvgs_eff_dvg;
                    let t12 = t7 * d_alphaz_dvb - t9 * dvdsat_dvb;
                    charge.cddb = t4 * (3.0 - 6.0 * t2 - 3.0 * t5);
                    charge.cdsb = -(charge.cdgb + t12 + charge.cddb);

                    let t9 = 2.0 * t4 * (1.0 + t5);
                    let t10 = (t8 * d_alphaz_dvg - t9 * dvdsat_dvg) * dvgs_eff_dvg;
                    let t11 = t8 * d_alphaz_dvb - t9 * dvdsat_dvb;
                    let t12 = t4 * (2.0 * t2 + t5 - 1.0);
                    let t0 = -(t10 + t11 + t12);
                    charge.cbgb = -(charge.cggb + charge.cdgb + t10);
                    charge.cbdb = -(charge.cgdb + charge.cddb + t12);
                    charge.cbsb = -(charge.cgsb + charge.cdsb + t0);
                    op.qinv = -(qgate + qbulk);
                }
            } else if model.xpart < 0.5 {
                // 40/60 charge partition model.
                if vds_c >= vdsat0 {
                    let t1 = vdsat0 / 3.0;
                    qgate = cox_wl * (vgs_eff - vfb - p.phi - t1);
                    let t2 = -two_third_cox_wl * vgst0;
                    qbulk = -(qgate + t2);
                    qdrn = 0.4 * t2;

                    charge.cggb = one_third_cox_wl * (3.0 - dvdsat_dvg) * dvgs_eff_dvg;
                    let t2 = -one_third_cox_wl * dvdsat_dvb;
                    charge.cgsb = -(charge.cggb + t2);
                    charge.cgdb = 0.0;
                    let t3 = 0.4 * two_third_cox_wl;
                    charge.cdgb = -t3 * dvgs_eff_dvg;
                    charge.cddb = 0.0;
                    let t4 = t3 * dvth0_cv_dvb;
                    charge.cdsb = -(t4 + charge.cdgb);
                    charge.cbgb = -(charge.cggb - two_third_cox_wl * dvgs_eff_dvg);
                    let t3 = -(t2 + two_third_cox_wl * dvth0_cv_dvb);
                    charge.cbsb = -(charge.cbgb + t3);
                    charge.cbdb = 0.0;
                    op.qinv = -(qgate + qbulk);
                } else {
                    let alphaz = vgst0 / vdsat0;
                    let t1 = 2.0 * vdsat0 - vds_c;
                    let t2 = vds_c / (3.0 * t1);
                    let t3 = t2 * vds_c;
                    let t9 = 0.25 * cox_wl;
                    let t4 = t9 * alphaz;
                    qgate = cox_wl * (vgs_eff - vfb - p.phi - 0.5 * (vds_c - t3));

                    let t5 = t3 / t1;
                    charge.cggb = cox_wl * (1.0 - t5 * dvdsat_dvg) * dvgs_eff_dvg;
                    let tmp = -cox_wl * t5 * dvdsat_dvb;
                    charge.cgdb = cox_wl * (t2 - 0.5 + 0.5 * t5);
                    charge.cgsb = -(charge.cggb + charge.cgdb + tmp);

                    let t6 = 1.0 / vdsat0;
                    let d_alphaz_dvg = t6 * (1.0 - alphaz * dvdsat_dvg);
                    let d_alphaz_dvb = -t6 * (dvth0_cv_dvb + alphaz * dvdsat_dvb);

                    let t6 = 8.0 * vdsat0 * vdsat0 - 6.0 * vdsat0 * vds_c + 1.2 * vds_c * vds_c;
                    let t8 = t2 / t1;
                    let mut t7 = vds_c - t1 - t8 * t6;
                    qdrn = t4 * t7;
                    t7 *= t9;
                    let tmp = t8 / t1;
                    let tmp1 = t4 * (2.0 - 4.0 * tmp * t6 + t8 * (16.0 * vdsat0 - 6.0 * vds_c));
                    charge.cdgb = (t7 * d_alphaz_dvg - tmp1 * dvdsat_dvg) * dvgs_eff_dvg;
                    let t10 = t7 * d_alphaz_dvb - tmp1 * dvdsat_dvb;
                    charge.cddb = t4
                        * (2.0 - (1.0 / (3.0 * t1 * t1) + 2.0 * tmp) * t6
                            + t8 * (6.0 * vdsat0 - 2.4 * vds_c));
                    charge.cdsb = -(charge.cdgb + t10 + charge.cddb);

                    let mut t7 = 2.0 * (t1 + t3);
                    qbulk = -(qgate - t4 * t7);
                    t7 *= t9;
                    let t0 = 4.0 * t4 * (1.0 - t5);
                    let t12 = (-t7 * d_alphaz_dvg - charge.cdgb - t0 * dvdsat_dvg) * dvgs_eff_dvg;
                    let t11 = -t7 * d_alphaz_dvb - t10 - t0 * dvdsat_dvb;
                    let t10 = -4.0 * t4 * (t2 - 0.5 + 0.5 * t5) - charge.cddb;
                    let tmp = -(t10 + t11 + t12);
                    charge.cbgb = -(charge.cggb + charge.cdgb + t12);
                    charge.cbdb = -(charge.cgdb + charge.cddb + t10);
                    charge.cbsb = -(charge.cgsb + charge.cdsb + tmp);
                    op.qinv = -(qgate + qbulk);
                }
            } else {
                // 50/50 charge partition model.
                if vds_c >= vdsat0 {
                    let t1 = vdsat0 / 3.0;
                    qgate = cox_wl * (vgs_eff - vfb - p.phi - t1);
                    let t2 = -two_third_cox_wl * vgst0;
                    qbulk = -(qgate + t2);
                    qdrn = 0.5 * t2;

                    charge.cggb = one_third_cox_wl * (3.0 - dvdsat_dvg) * dvgs_eff_dvg;
                    let t2 = -one_third_cox_wl * dvdsat_dvb;
                    charge.cgsb = -(charge.cggb + t2);
                    charge.cgdb = 0.0;
                    charge.cdgb = -one_third_cox_wl * dvgs_eff_dvg;
                    charge.cddb = 0.0;
                    let t4 = one_third_cox_wl * dvth0_cv_dvb;
                    charge.cdsb = -(t4 + charge.cdgb);
                    charge.cbgb = -(charge.cggb - two_third_cox_wl * dvgs_eff_dvg);
                    let t3 = -(t2 + two_third_cox_wl * dvth0_cv_dvb);
                    charge.cbsb = -(charge.cbgb + t3);
                    charge.cbdb = 0.0;
                    op.qinv = -(qgate + qbulk);
                } else {
                    let alphaz = vgst0 / vdsat0;
                    let t1 = 2.0 * vdsat0 - vds_c;
                    let t2 = vds_c / (3.0 * t1);
                    let t3 = t2 * vds_c;
                    let t9 = 0.25 * cox_wl;
                    let t4 = t9 * alphaz;
                    qgate = cox_wl * (vgs_eff - vfb - p.phi - 0.5 * (vds_c - t3));

                    let t5 = t3 / t1;
                    charge.cggb = cox_wl * (1.0 - t5 * dvdsat_dvg) * dvgs_eff_dvg;
                    let tmp = -cox_wl * t5 * dvdsat_dvb;
                    charge.cgdb = cox_wl * (t2 - 0.5 + 0.5 * t5);
                    charge.cgsb = -(charge.cggb + charge.cgdb + tmp);

                    let t6 = 1.0 / vdsat0;
                    let d_alphaz_dvg = t6 * (1.0 - alphaz * dvdsat_dvg);
                    let d_alphaz_dvb = -t6 * (dvth0_cv_dvb + alphaz * dvdsat_dvb);

                    let mut t7 = t1 + t3;
                    qdrn = -t4 * t7;
                    qbulk = -(qgate + qdrn + qdrn);
                    t7 *= t9;
                    let t0 = t4 * (2.0 * t5 - 2.0);
                    charge.cdgb = (t0 * dvdsat_dvg - t7 * d_alphaz_dvg) * dvgs_eff_dvg;
                    let t12 = t0 * dvdsat_dvb - t7 * d_alphaz_dvb;
                    charge.cddb = t4 * (1.0 - 2.0 * t2 - t5);
                    charge.cdsb = -(charge.cdgb + t12 + charge.cddb);
                    charge.cbgb = -(charge.cggb + 2.0 * charge.cdgb);
                    charge.cbdb = -(charge.cgdb + 2.0 * charge.cddb);
                    charge.cbsb = -(charge.cgsb + 2.0 * charge.cdsb);
                    op.qinv = -(qgate + qbulk);
                }
            }
        }
    } else {
        // Shared CAPMOD=1/2/3 CV prelude (b3ld.c:1584-1628).
        let (vbseff_cv, dvbseff_cv_dvb);
        if vbseff < 0.0 {
            vbseff_cv = vbseff;
            dvbseff_cv_dvb = 1.0;
        } else {
            vbseff_cv = p.phi - phis;
            dvbseff_cv_dvb = -dphis_dvb;
        }

        let cox_wl = model.cox * p.weff_cv * p.leff_cv;

        // Separate VgsteffCV with noff and voffcv (b3ld.c:1597-1627).
        let noff = n * p.noff;
        let dnoff_dvd = p.noff * dn_dvd;
        let dnoff_dvb = p.noff * dn_dvb;
        let t0 = vtm * noff;
        let voffcv = p.voffcv;
        let vgst_nvt_cv = (vgst - voffcv) / t0;

        let (vgsteff_cv, mut dvgsteff_cv_dvg, dvgsteff_cv_dvd, dvgsteff_cv_dvb);
        if vgst_nvt_cv > EXP_THRESHOLD {
            vgsteff_cv = vgst - voffcv;
            dvgsteff_cv_dvg = dvgs_eff_dvg;
            dvgsteff_cv_dvd = -dvth_dvd;
            dvgsteff_cv_dvb = -dvth_dvb;
        } else if vgst_nvt_cv < -EXP_THRESHOLD {
            vgsteff_cv = t0 * (1.0 + MIN_EXP).ln();
            dvgsteff_cv_dvg = 0.0;
            let d_dvd = vgsteff_cv / noff;
            dvgsteff_cv_dvb = d_dvd * dnoff_dvb;
            dvgsteff_cv_dvd = d_dvd * dnoff_dvd;
        } else {
            let exp_vgst = vgst_nvt_cv.exp();
            vgsteff_cv = t0 * (1.0 + exp_vgst).ln();
            dvgsteff_cv_dvg = exp_vgst / (1.0 + exp_vgst);
            dvgsteff_cv_dvd = -dvgsteff_cv_dvg * (dvth_dvd + (vgst - voffcv) / noff * dnoff_dvd)
                + vgsteff_cv / noff * dnoff_dvd;
            dvgsteff_cv_dvb = -dvgsteff_cv_dvg * (dvth_dvb + (vgst - voffcv) / noff * dnoff_dvb)
                + vgsteff_cv / noff * dnoff_dvb;
            dvgsteff_cv_dvg *= dvgs_eff_dvg;
        }

        if model.cap_mod == 1 {
            // CAPMOD=1 intrinsic charge model (b3ld.c:1629-1789).
            let vfb = inst.vfbzb;
            let arg1 = vgs_eff - vbseff_cv - vfb - vgsteff_cv;

            let (mut cgg, mut cgd, mut cgb);
            if arg1 <= 0.0 {
                qgate = cox_wl * arg1;
                cgg = cox_wl * (dvgs_eff_dvg - dvgsteff_cv_dvg);
                cgd = -cox_wl * dvgsteff_cv_dvd;
                cgb = -cox_wl * (dvbseff_cv_dvb + dvgsteff_cv_dvb);
            } else {
                let t0 = 0.5 * p.k1ox;
                let t1 = (t0 * t0 + arg1).sqrt();
                let t2 = cox_wl * t0 / t1;
                qgate = cox_wl * p.k1ox * (t1 - t0);
                cgg = t2 * (dvgs_eff_dvg - dvgsteff_cv_dvg);
                cgd = -t2 * dvgsteff_cv_dvd;
                cgb = -t2 * (dvbseff_cv_dvb + dvgsteff_cv_dvb);
            }

            qbulk = -qgate;
            let mut cbg = -cgg;
            let mut cbd = -cgd;
            let mut cbb = -cgb;

            let one_third_cox_wl = cox_wl / 3.0;
            let two_third_cox_wl = 2.0 * one_third_cox_wl;
            let abulk_cv = abulk0 * p.abulk_cv_factor;
            let dabulk_cv_dvb = p.abulk_cv_factor * dabulk0_dvb;
            let vdsat_cv = vgsteff_cv / abulk_cv;

            let qsrc;
            let (csg, csd, mut csb);
            if vdsat_cv < vds_c {
                let dvdsat_cv_dvg = 1.0 / abulk_cv;
                let dvdsat_cv_dvb = -vdsat_cv * dabulk_cv_dvb / abulk_cv;

                let t0 = vgsteff_cv - vdsat_cv / 3.0;
                let dt0_dvg = 1.0 - dvdsat_cv_dvg / 3.0;
                let dt0_dvb = -dvdsat_cv_dvb / 3.0;
                qgate += cox_wl * t0;
                let mut cgg1 = cox_wl * dt0_dvg;
                let cgb1 = cox_wl * dt0_dvb + cgg1 * dvgsteff_cv_dvb;
                let cgd1 = cgg1 * dvgsteff_cv_dvd;
                cgg1 *= dvgsteff_cv_dvg;
                cgg += cgg1;
                cgb += cgb1;
                cgd += cgd1;

                let t0 = vdsat_cv - vgsteff_cv;
                let dt0_dvg = dvdsat_cv_dvg - 1.0;
                let dt0_dvb = dvdsat_cv_dvb;
                qbulk += one_third_cox_wl * t0;
                let mut cbg1 = one_third_cox_wl * dt0_dvg;
                let cbb1 = one_third_cox_wl * dt0_dvb + cbg1 * dvgsteff_cv_dvb;
                let cbd1 = cbg1 * dvgsteff_cv_dvd;
                cbg1 *= dvgsteff_cv_dvg;
                cbg += cbg1;
                cbb += cbb1;
                cbd += cbd1;

                let t0 = if model.xpart > 0.5 {
                    -two_third_cox_wl
                } else if model.xpart < 0.5 {
                    -0.4 * cox_wl
                } else {
                    -one_third_cox_wl
                };
                qsrc = t0 * vgsteff_cv;
                csg = t0 * dvgsteff_cv_dvg;
                csb = t0 * dvgsteff_cv_dvb;
                csd = t0 * dvgsteff_cv_dvd;
            } else {
                let t0 = abulk_cv * vds_c;
                let mut t1 = 12.0 * (vgsteff_cv - 0.5 * t0 + 1.0e-20);
                let t2 = vds_c / t1;
                let t3 = t0 * t2;
                let dt3_dvg = -12.0 * t2 * t2 * abulk_cv;
                let dt3_dvd = 6.0 * t0 * (4.0 * vgsteff_cv - t0) / (t1 * t1) - 0.5;
                let dt3_dvb = 12.0 * t2 * t2 * dabulk_cv_dvb * vgsteff_cv;

                qgate += cox_wl * (vgsteff_cv - 0.5 * vds_c + t3);
                let mut cgg1 = cox_wl * (1.0 + dt3_dvg);
                let cgb1 = cox_wl * dt3_dvb + cgg1 * dvgsteff_cv_dvb;
                let cgd1 = cox_wl * dt3_dvd + cgg1 * dvgsteff_cv_dvd;
                cgg1 *= dvgsteff_cv_dvg;
                cgg += cgg1;
                cgb += cgb1;
                cgd += cgd1;

                qbulk += cox_wl * (1.0 - abulk_cv) * (0.5 * vds_c - t3);
                let mut cbg1 = -cox_wl * ((1.0 - abulk_cv) * dt3_dvg);
                let cbb1 = -cox_wl
                    * ((1.0 - abulk_cv) * dt3_dvb + (0.5 * vds_c - t3) * dabulk_cv_dvb)
                    + cbg1 * dvgsteff_cv_dvb;
                let cbd1 = -cox_wl * (1.0 - abulk_cv) * dt3_dvd + cbg1 * dvgsteff_cv_dvd;
                cbg1 *= dvgsteff_cv_dvg;
                cbg += cbg1;
                cbb += cbb1;
                cbd += cbd1;

                if model.xpart > 0.5 {
                    // 0/100 charge partition model.
                    t1 += t1;
                    qsrc = -cox_wl * (0.5 * vgsteff_cv + 0.25 * t0 - t0 * t0 / t1);
                    let mut csg1 = -cox_wl * (0.5 + 24.0 * t0 * vds_c / (t1 * t1) * abulk_cv);
                    csb = -cox_wl
                        * (0.25 * vds_c * dabulk_cv_dvb
                            - 12.0 * t0 * vds_c / (t1 * t1)
                                * (4.0 * vgsteff_cv - t0)
                                * dabulk_cv_dvb)
                        + csg1 * dvgsteff_cv_dvb;
                    csd = -cox_wl
                        * (0.25 * abulk_cv
                            - 12.0 * abulk_cv * t0 / (t1 * t1) * (4.0 * vgsteff_cv - t0))
                        + csg1 * dvgsteff_cv_dvd;
                    csg1 *= dvgsteff_cv_dvg;
                    csg = csg1;
                } else if model.xpart < 0.5 {
                    // 40/60 charge partition model.
                    let t1 = t1 / 12.0;
                    let t2 = 0.5 * cox_wl / (t1 * t1);
                    let t3 = vgsteff_cv
                        * (2.0 * t0 * t0 / 3.0 + vgsteff_cv * (vgsteff_cv - 4.0 * t0 / 3.0))
                        - 2.0 * t0 * t0 * t0 / 15.0;
                    qsrc = -t2 * t3;
                    let t4 = 4.0 / 3.0 * vgsteff_cv * (vgsteff_cv - t0) + 0.4 * t0 * t0;
                    let mut csg1 = -2.0 * qsrc / t1
                        - t2 * (vgsteff_cv * (3.0 * vgsteff_cv - 8.0 * t0 / 3.0)
                            + 2.0 * t0 * t0 / 3.0);
                    csb = (qsrc / t1 * vds_c + t2 * t4 * vds_c) * dabulk_cv_dvb
                        + csg1 * dvgsteff_cv_dvb;
                    csd = (qsrc / t1 + t2 * t4) * abulk_cv + csg1 * dvgsteff_cv_dvd;
                    csg1 *= dvgsteff_cv_dvg;
                    csg = csg1;
                } else {
                    // 50/50 charge partition model.
                    qsrc = -0.5 * (qgate + qbulk);
                    csg = -0.5 * (cgg1 + cbg1);
                    csb = -0.5 * (cgb1 + cbb1);
                    csd = -0.5 * (cgd1 + cbd1);
                }
            }

            cgb *= dvbseff_dvb;
            cbb *= dvbseff_dvb;
            csb *= dvbseff_dvb;

            qdrn = -(qgate + qbulk + qsrc);
            charge.cggb = cgg;
            charge.cgsb = -(cgg + cgd + cgb);
            charge.cgdb = cgd;
            charge.cdgb = -(cgg + cbg + csg);
            charge.cdsb = cgg + cgd + cgb + cbg + cbd + cbb + csg + csd + csb;
            charge.cddb = -(cgd + cbd + csd);
            charge.cbgb = cbg;
            charge.cbsb = -(cbg + cbd + cbb);
            charge.cbdb = cbd;
            op.qinv = -(qgate + qbulk);
        } else if model.cap_mod == 2 {
            // CAPMOD=2 intrinsic charge model (b3ld.c:1791-1951).
            let vfb = inst.vfbzb;
            let v3 = vfb - vgs_eff + vbseff_cv - DELTA_3;
            let t0 = if vfb <= 0.0 {
                (v3 * v3 - 4.0 * DELTA_3 * vfb).sqrt()
            } else {
                (v3 * v3 + 4.0 * DELTA_3 * vfb).sqrt()
            };

            let t1 = 0.5 * (1.0 + v3 / t0);
            let vfbeff = vfb - 0.5 * (v3 + t0);
            let dvfbeff_dvg = t1 * dvgs_eff_dvg;
            let dvfbeff_dvb = -t1 * dvbseff_cv_dvb;
            let qac0 = cox_wl * (vfbeff - vfb);
            let dqac0_dvg = cox_wl * dvfbeff_dvg;
            let dqac0_dvb = cox_wl * dvfbeff_dvb;

            let t0 = 0.5 * p.k1ox;
            let t3 = vgs_eff - vfbeff - vbseff_cv - vgsteff_cv;
            let (t1, t2);
            if p.k1ox == 0.0 {
                t1 = 0.0;
                t2 = 0.0;
            } else if t3 < 0.0 {
                t1 = t0 + t3 / p.k1ox;
                t2 = cox_wl;
            } else {
                t1 = (t0 * t0 + t3).sqrt();
                t2 = cox_wl * t0 / t1;
            }

            let qsub0 = cox_wl * p.k1ox * (t1 - t0);
            let dqsub0_dvg = t2 * (dvgs_eff_dvg - dvfbeff_dvg - dvgsteff_cv_dvg);
            let dqsub0_dvd = -t2 * dvgsteff_cv_dvd;
            let dqsub0_dvb = -t2 * (dvfbeff_dvb + dvbseff_cv_dvb + dvgsteff_cv_dvb);

            let abulk_cv = abulk0 * p.abulk_cv_factor;
            let dabulk_cv_dvb = p.abulk_cv_factor * dabulk0_dvb;
            let vdsat_cv = vgsteff_cv / abulk_cv;

            let v4 = vdsat_cv - vds_c - DELTA_4;
            let t0 = (v4 * v4 + 4.0 * DELTA_4 * vdsat_cv).sqrt();
            let mut vdseff_cv = vdsat_cv - 0.5 * (v4 + t0);
            let t1 = 0.5 * (1.0 + v4 / t0);
            let t2 = DELTA_4 / t0;
            let t3 = (1.0 - t1 - t2) / abulk_cv;
            let mut dvdseff_cv_dvg = t3;
            let dvdseff_cv_dvd = t1;
            let mut dvdseff_cv_dvb = -t3 * vdsat_cv * dabulk_cv_dvb;
            if vds_c == 0.0 {
                vdseff_cv = 0.0;
                dvdseff_cv_dvg = 0.0;
                dvdseff_cv_dvb = 0.0;
            }

            let t0 = abulk_cv * vdseff_cv;
            let mut t1 = 12.0 * (vgsteff_cv - 0.5 * t0 + 1.0e-20);
            let t2 = vdseff_cv / t1;
            let t3 = t0 * t2;

            let t4 = 1.0 - 12.0 * t2 * t2 * abulk_cv;
            let t5 = 6.0 * t0 * (4.0 * vgsteff_cv - t0) / (t1 * t1) - 0.5;
            let t6 = 12.0 * t2 * t2 * vgsteff_cv;

            let qinoi = -cox_wl * (vgsteff_cv - 0.5 * t0 + abulk_cv * t3);
            qgate = cox_wl * (vgsteff_cv - 0.5 * vdseff_cv + t3);
            let mut cgg1 = cox_wl * (t4 + t5 * dvdseff_cv_dvg);
            let cgd1 = cox_wl * t5 * dvdseff_cv_dvd + cgg1 * dvgsteff_cv_dvd;
            let cgb1 = cox_wl * (t5 * dvdseff_cv_dvb + t6 * dabulk_cv_dvb) + cgg1 * dvgsteff_cv_dvb;
            cgg1 *= dvgsteff_cv_dvg;

            let t7 = 1.0 - abulk_cv;
            qbulk = cox_wl * t7 * (0.5 * vdseff_cv - t3);
            let t4 = -t7 * (t4 - 1.0);
            let t5 = -t7 * t5;
            let t6 = -(t7 * t6 + (0.5 * vdseff_cv - t3));
            let mut cbg1 = cox_wl * (t4 + t5 * dvdseff_cv_dvg);
            let cbd1 = cox_wl * t5 * dvdseff_cv_dvd + cbg1 * dvgsteff_cv_dvd;
            let cbb1 = cox_wl * (t5 * dvdseff_cv_dvb + t6 * dabulk_cv_dvb) + cbg1 * dvgsteff_cv_dvb;
            cbg1 *= dvgsteff_cv_dvg;

            let qsrc;
            let (mut csg, csd, mut csb);
            if model.xpart > 0.5 {
                // 0/100 charge partition model.
                t1 += t1;
                qsrc = -cox_wl * (0.5 * vgsteff_cv + 0.25 * t0 - t0 * t0 / t1);
                let t7 = (4.0 * vgsteff_cv - t0) / (t1 * t1);
                let t4 = -(0.5 + 24.0 * t0 * t0 / (t1 * t1));
                let t5 = -(0.25 * abulk_cv - 12.0 * abulk_cv * t0 * t7);
                let t6 = -(0.25 * vdseff_cv - 12.0 * t0 * vdseff_cv * t7);
                csg = cox_wl * (t4 + t5 * dvdseff_cv_dvg);
                csd = cox_wl * t5 * dvdseff_cv_dvd + csg * dvgsteff_cv_dvd;
                csb = cox_wl * (t5 * dvdseff_cv_dvb + t6 * dabulk_cv_dvb) + csg * dvgsteff_cv_dvb;
                csg *= dvgsteff_cv_dvg;
            } else if model.xpart < 0.5 {
                // 40/60 charge partition model.
                let t1 = t1 / 12.0;
                let t2 = 0.5 * cox_wl / (t1 * t1);
                let t3 = vgsteff_cv
                    * (2.0 * t0 * t0 / 3.0 + vgsteff_cv * (vgsteff_cv - 4.0 * t0 / 3.0))
                    - 2.0 * t0 * t0 * t0 / 15.0;
                qsrc = -t2 * t3;
                let t7 = 4.0 / 3.0 * vgsteff_cv * (vgsteff_cv - t0) + 0.4 * t0 * t0;
                let t4 = -2.0 * qsrc / t1
                    - t2 * (vgsteff_cv * (3.0 * vgsteff_cv - 8.0 * t0 / 3.0) + 2.0 * t0 * t0 / 3.0);
                let t5 = (qsrc / t1 + t2 * t7) * abulk_cv;
                let t6 = qsrc / t1 * vdseff_cv + t2 * t7 * vdseff_cv;
                csg = t4 + t5 * dvdseff_cv_dvg;
                csd = t5 * dvdseff_cv_dvd + csg * dvgsteff_cv_dvd;
                csb = t5 * dvdseff_cv_dvb + t6 * dabulk_cv_dvb + csg * dvgsteff_cv_dvb;
                csg *= dvgsteff_cv_dvg;
            } else {
                // 50/50 charge partition model.
                qsrc = -0.5 * (qgate + qbulk);
                csg = -0.5 * (cgg1 + cbg1);
                csb = -0.5 * (cgb1 + cbb1);
                csd = -0.5 * (cgd1 + cbd1);
            }

            qgate += qac0 + qsub0;
            qbulk -= qac0 + qsub0;
            qdrn = -(qgate + qbulk + qsrc);

            let cgg = dqac0_dvg + dqsub0_dvg + cgg1;
            let cgd = dqsub0_dvd + cgd1;
            let mut cgb = dqac0_dvb + dqsub0_dvb + cgb1;

            let cbg = cbg1 - dqac0_dvg - dqsub0_dvg;
            let cbd = cbd1 - dqsub0_dvd;
            let mut cbb = cbb1 - dqac0_dvb - dqsub0_dvb;

            cgb *= dvbseff_dvb;
            cbb *= dvbseff_dvb;
            csb *= dvbseff_dvb;

            charge.cggb = cgg;
            charge.cgsb = -(cgg + cgd + cgb);
            charge.cgdb = cgd;
            charge.cdgb = -(cgg + cbg + csg);
            charge.cdsb = cgg + cgd + cgb + cbg + cbd + cbb + csg + csd + csb;
            charge.cddb = -(cgd + cbd + csd);
            charge.cbgb = cbg;
            charge.cbsb = -(cbg + cbd + cbb);
            charge.cbdb = cbd;
            op.qinv = qinoi;
        } else {
            // Charge-Thickness capMod (CTM) begins (b3ld.c:1955-2239).
            let v3 = inst.vfbzb - vgs_eff + vbseff_cv - DELTA_3;
            let (t0, t2);
            if inst.vfbzb <= 0.0 {
                t0 = (v3 * v3 - 4.0 * DELTA_3 * inst.vfbzb).sqrt();
                t2 = -DELTA_3 / t0;
            } else {
                t0 = (v3 * v3 + 4.0 * DELTA_3 * inst.vfbzb).sqrt();
                t2 = DELTA_3 / t0;
            }
            let _ = t2; // T2 of the C is recomputed below before use.

            let t1 = 0.5 * (1.0 + v3 / t0);
            let vfbeff = inst.vfbzb - 0.5 * (v3 + t0);
            let dvfbeff_dvg = t1 * dvgs_eff_dvg;
            let dvfbeff_dvb = -t1 * dvbseff_cv_dvb;

            let cox = model.cox;
            let mut tox = 1.0e8 * model.tox;
            let t0 = (vgs_eff - vbseff_cv - inst.vfbzb) / tox;
            let dt0_dvg = dvgs_eff_dvg / tox;
            let dt0_dvb = -dvbseff_cv_dvb / tox;

            let tmp = t0 * p.acde;
            let (mut tcen, mut dtcen_dvg, mut dtcen_dvb);
            if -EXP_THRESHOLD < tmp && tmp < EXP_THRESHOLD {
                tcen = p.ldeb * tmp.exp();
                dtcen_dvg = p.acde * tcen;
                dtcen_dvb = dtcen_dvg * dt0_dvb;
                dtcen_dvg *= dt0_dvg;
            } else if tmp <= -EXP_THRESHOLD {
                tcen = p.ldeb * MIN_EXP;
                dtcen_dvg = 0.0;
                dtcen_dvb = 0.0;
            } else {
                tcen = p.ldeb * MAX_EXP;
                dtcen_dvg = 0.0;
                dtcen_dvb = 0.0;
            }

            let link = 1.0e-3 * model.tox;
            let v3 = p.ldeb - tcen - link;
            let v4 = (v3 * v3 + 4.0 * link * p.ldeb).sqrt();
            tcen = p.ldeb - 0.5 * (v3 + v4);
            let t1 = 0.5 * (1.0 + v3 / v4);
            dtcen_dvg *= t1;
            dtcen_dvb *= t1;

            let mut ccen = EPSSI / tcen;
            let t2 = cox / (cox + ccen);
            let mut coxeff = t2 * ccen;
            let t3 = -ccen / tcen;
            let mut dcoxeff_dvg = t2 * t2 * t3;
            let mut dcoxeff_dvb = dcoxeff_dvg * dtcen_dvb;
            dcoxeff_dvg *= dtcen_dvg;
            let mut cox_wl_cen = cox_wl * coxeff / cox;

            let qac0 = cox_wl_cen * (vfbeff - inst.vfbzb);
            let mut qov_cox = qac0 / coxeff;
            let dqac0_dvg = cox_wl_cen * dvfbeff_dvg + qov_cox * dcoxeff_dvg;
            let dqac0_dvb = cox_wl_cen * dvfbeff_dvb + qov_cox * dcoxeff_dvb;

            let t0 = 0.5 * p.k1ox;
            let t3 = vgs_eff - vfbeff - vbseff_cv - vgsteff_cv;
            let (t1, t2);
            if p.k1ox == 0.0 {
                t1 = 0.0;
                t2 = 0.0;
            } else if t3 < 0.0 {
                t1 = t0 + t3 / p.k1ox;
                t2 = cox_wl_cen;
            } else {
                t1 = (t0 * t0 + t3).sqrt();
                t2 = cox_wl_cen * t0 / t1;
            }

            let qsub0 = cox_wl_cen * p.k1ox * (t1 - t0);
            qov_cox = qsub0 / coxeff;
            let dqsub0_dvg =
                t2 * (dvgs_eff_dvg - dvfbeff_dvg - dvgsteff_cv_dvg) + qov_cox * dcoxeff_dvg;
            let dqsub0_dvd = -t2 * dvgsteff_cv_dvd;
            let dqsub0_dvb =
                -t2 * (dvfbeff_dvb + dvbseff_cv_dvb + dvgsteff_cv_dvb) + qov_cox * dcoxeff_dvb;

            // Gate-bias dependent delta Phis (b3ld.c:2040-2054).
            let (denomi, t0);
            if p.k1ox <= 0.0 {
                denomi = 0.25 * p.moin * vtm;
                t0 = 0.5 * p.sqrt_phi;
            } else {
                denomi = p.moin * vtm * p.k1ox * p.k1ox;
                t0 = p.k1ox * p.sqrt_phi;
            }
            let t1 = 2.0 * t0 + vgsteff_cv;

            let delta_phi = vtm * (1.0 + t1 * vgsteff_cv / denomi).ln();
            let ddelta_phi_dvg = 2.0 * vtm * (t1 - t0) / (denomi + t1 * vgsteff_cv);

            // VgDP = Vgsteff - DeltaPhi (b3ld.c:2056-2061).
            let t0 = vgsteff_cv - delta_phi - 0.001;
            let dt0_dvg = 1.0 - ddelta_phi_dvg;
            let t1 = (t0 * t0 + vgsteff_cv * 0.004).sqrt();
            let vg_dp = 0.5 * (t0 + t1);
            let dvg_dp_dvg = 0.5 * (dt0_dvg + (t0 * dt0_dvg + 0.002) / t1);

            let t3 = 4.0 * (vth - inst.vfbzb - p.phi);
            tox += tox;
            let (t0, dt0_dvd, dt0_dvb);
            if t3 >= 0.0 {
                t0 = (vgsteff_cv + t3) / tox;
                dt0_dvd = (dvgsteff_cv_dvd + 4.0 * dvth_dvd) / tox;
                dt0_dvb = (dvgsteff_cv_dvb + 4.0 * dvth_dvb) / tox;
            } else {
                t0 = (vgsteff_cv + 1.0e-20) / tox;
                dt0_dvd = dvgsteff_cv_dvd / tox;
                dt0_dvb = dvgsteff_cv_dvb / tox;
            }
            let tmp = (0.7 * t0.ln()).exp();
            let t1 = 1.0 + tmp;
            let t2 = 0.7 * tmp / (t0 * tox);
            tcen = 1.9e-9 / t1;
            dtcen_dvg = -1.9e-9 * t2 / t1 / t1;
            let mut dtcen_dvd = tox * dtcen_dvg;
            dtcen_dvb = dtcen_dvd * dt0_dvb;
            dtcen_dvd *= dt0_dvd;
            dtcen_dvg *= dvgsteff_cv_dvg;

            ccen = EPSSI / tcen;
            let t0 = cox / (cox + ccen);
            coxeff = t0 * ccen;
            let t1 = -ccen / tcen;
            dcoxeff_dvg = t0 * t0 * t1;
            let dcoxeff_dvd = dcoxeff_dvg * dtcen_dvd;
            dcoxeff_dvb = dcoxeff_dvg * dtcen_dvb;
            dcoxeff_dvg *= dtcen_dvg;
            cox_wl_cen = cox_wl * coxeff / cox;

            let abulk_cv = abulk0 * p.abulk_cv_factor;
            let dabulk_cv_dvb = p.abulk_cv_factor * dabulk0_dvb;
            let vdsat_cv = vg_dp / abulk_cv;
            let t0 = vdsat_cv - vds_c - DELTA_4;
            let dt0_dvg = dvg_dp_dvg / abulk_cv;
            let dt0_dvb = -vdsat_cv * dabulk_cv_dvb / abulk_cv;
            let t1 = (t0 * t0 + 4.0 * DELTA_4 * vdsat_cv).sqrt();
            let mut dt1_dvg = (t0 + DELTA_4 + DELTA_4) / t1;
            let dt1_dvd = -t0 / t1;
            let dt1_dvb = dt1_dvg * dt0_dvb;
            dt1_dvg *= dt0_dvg;
            let (mut vdseff_cv, mut dvdseff_cv_dvg, dvdseff_cv_dvd, mut dvdseff_cv_dvb);
            if t0 >= 0.0 {
                vdseff_cv = vdsat_cv - 0.5 * (t0 + t1);
                dvdseff_cv_dvg = 0.5 * (dt0_dvg - dt1_dvg);
                dvdseff_cv_dvd = 0.5 * (1.0 - dt1_dvd);
                dvdseff_cv_dvb = 0.5 * (dt0_dvb - dt1_dvb);
            } else {
                let t3 = (DELTA_4 + DELTA_4) / (t1 - t0);
                let t4 = 1.0 - t3;
                let t5 = vdsat_cv * t3 / (t1 - t0);
                vdseff_cv = vdsat_cv * t4;
                dvdseff_cv_dvg = dt0_dvg * t4 + t5 * (dt1_dvg - dt0_dvg);
                dvdseff_cv_dvd = t5 * (dt1_dvd + 1.0);
                dvdseff_cv_dvb = dt0_dvb * (1.0 - t5) + t5 * dt1_dvb;
            }
            // Added to eliminate non-zero VdseffCV at Vds=0.0.
            if vds_c == 0.0 {
                vdseff_cv = 0.0;
                dvdseff_cv_dvg = 0.0;
                dvdseff_cv_dvb = 0.0;
            }

            let t0 = abulk_cv * vdseff_cv;
            let t1 = vg_dp;
            let mut t2 = 12.0 * (t1 - 0.5 * t0 + 1.0e-20);
            let t3 = t0 / t2;
            let t4 = 1.0 - 12.0 * t3 * t3;
            let mut t5 = abulk_cv * (6.0 * t0 * (4.0 * t1 - t0) / (t2 * t2) - 0.5);
            let t6 = t5 * vdseff_cv / abulk_cv;

            qgate = cox_wl_cen * (t1 - t0 * (0.5 - t3));
            let qinoi = qgate;
            qov_cox = qgate / coxeff;
            let mut cgg1 = cox_wl_cen * (t4 * dvg_dp_dvg + t5 * dvdseff_cv_dvg);
            let cgd1 =
                cox_wl_cen * t5 * dvdseff_cv_dvd + cgg1 * dvgsteff_cv_dvd + qov_cox * dcoxeff_dvd;
            let cgb1 = cox_wl_cen * (t5 * dvdseff_cv_dvb + t6 * dabulk_cv_dvb)
                + cgg1 * dvgsteff_cv_dvb
                + qov_cox * dcoxeff_dvb;
            cgg1 = cgg1 * dvgsteff_cv_dvg + qov_cox * dcoxeff_dvg;

            let t7 = 1.0 - abulk_cv;
            let t8 = t2 * t2;
            let t9 = 12.0 * t7 * t0 * t0 / (t8 * abulk_cv);
            let t10 = t9 * dvg_dp_dvg;
            let t11 = -t7 * t5 / abulk_cv;
            let t12 = -(t9 * t1 / abulk_cv + vdseff_cv * (0.5 - t0 / t2));

            qbulk = cox_wl_cen * t7 * (0.5 * vdseff_cv - t0 * vdseff_cv / t2);
            qov_cox = qbulk / coxeff;
            let mut cbg1 = cox_wl_cen * (t10 + t11 * dvdseff_cv_dvg);
            let cbd1 =
                cox_wl_cen * t11 * dvdseff_cv_dvd + cbg1 * dvgsteff_cv_dvd + qov_cox * dcoxeff_dvd;
            let cbb1 = cox_wl_cen * (t11 * dvdseff_cv_dvb + t12 * dabulk_cv_dvb)
                + cbg1 * dvgsteff_cv_dvb
                + qov_cox * dcoxeff_dvb;
            cbg1 = cbg1 * dvgsteff_cv_dvg + qov_cox * dcoxeff_dvg;

            let qsrc;
            let (mut csg, csd, mut csb);
            if model.xpart > 0.5 {
                // 0/100 partition.
                qsrc = -cox_wl_cen * (t1 / 2.0 + t0 / 4.0 - 0.5 * t0 * t0 / t2);
                qov_cox = qsrc / coxeff;
                t2 += t2;
                let t3 = t2 * t2;
                let t7 = -(0.25 - 12.0 * t0 * (4.0 * t1 - t0) / t3);
                let t4 = -(0.5 + 24.0 * t0 * t0 / t3) * dvg_dp_dvg;
                t5 = t7 * abulk_cv;
                let t6 = t7 * vdseff_cv;

                csg = cox_wl_cen * (t4 + t5 * dvdseff_cv_dvg);
                csd = cox_wl_cen * t5 * dvdseff_cv_dvd
                    + csg * dvgsteff_cv_dvd
                    + qov_cox * dcoxeff_dvd;
                csb = cox_wl_cen * (t5 * dvdseff_cv_dvb + t6 * dabulk_cv_dvb)
                    + csg * dvgsteff_cv_dvb
                    + qov_cox * dcoxeff_dvb;
                csg = csg * dvgsteff_cv_dvg + qov_cox * dcoxeff_dvg;
            } else if model.xpart < 0.5 {
                // 40/60 partition.
                let t2 = t2 / 12.0;
                let t3 = 0.5 * cox_wl_cen / (t2 * t2);
                let t4 = t1 * (2.0 * t0 * t0 / 3.0 + t1 * (t1 - 4.0 * t0 / 3.0))
                    - 2.0 * t0 * t0 * t0 / 15.0;
                qsrc = -t3 * t4;
                qov_cox = qsrc / coxeff;
                let t8 = 4.0 / 3.0 * t1 * (t1 - t0) + 0.4 * t0 * t0;
                let t5 = -2.0 * qsrc / t2
                    - t3 * (t1 * (3.0 * t1 - 8.0 * t0 / 3.0) + 2.0 * t0 * t0 / 3.0);
                let t6 = abulk_cv * (qsrc / t2 + t3 * t8);
                let t7 = t6 * vdseff_cv / abulk_cv;

                csg = t5 * dvg_dp_dvg + t6 * dvdseff_cv_dvg;
                csd = csg * dvgsteff_cv_dvd + t6 * dvdseff_cv_dvd + qov_cox * dcoxeff_dvd;
                csb = csg * dvgsteff_cv_dvb
                    + t6 * dvdseff_cv_dvb
                    + t7 * dabulk_cv_dvb
                    + qov_cox * dcoxeff_dvb;
                csg = csg * dvgsteff_cv_dvg + qov_cox * dcoxeff_dvg;
            } else {
                // 50/50 partition.
                qsrc = -0.5 * qgate;
                csg = -0.5 * cgg1;
                csd = -0.5 * cgd1;
                csb = -0.5 * cgb1;
            }

            qgate += qac0 + qsub0 - qbulk;
            qbulk -= qac0 + qsub0;
            qdrn = -(qgate + qbulk + qsrc);

            let cbg = cbg1 - dqac0_dvg - dqsub0_dvg;
            let cbd = cbd1 - dqsub0_dvd;
            let mut cbb = cbb1 - dqac0_dvb - dqsub0_dvb;

            let cgg = cgg1 - cbg;
            let cgd = cgd1 - cbd;
            let mut cgb = cgb1 - cbb;

            cgb *= dvbseff_dvb;
            cbb *= dvbseff_dvb;
            csb *= dvbseff_dvb;

            charge.cggb = cgg;
            charge.cgsb = -(cgg + cgd + cgb);
            charge.cgdb = cgd;
            charge.cdgb = -(cgg + cbg + csg);
            charge.cdsb = cgg + cgd + cgb + cbg + cbd + cbb + csg + csd + csb;
            charge.cddb = -(cgd + cbd + csd);
            charge.cbgb = cbg;
            charge.cbsb = -(cbg + cbd + cbb);
            charge.cbdb = cbd;
            op.qinv = -qinoi;
        }
    }

    // --- Junction depletion charges and capacitances (b3ld.c:2253-2431),
    //     ACM=0/1 geometry paths ---
    junction_depletion(model, mt, p, inst, vbs, vbd, &mut charge);

    // --- NQS charge-deficit terms (b3ld.c:2477-2493) ---
    charge.qdrn_channel = qdrn;
    charge.qcheq = -(qbulk + qgate);
    charge.cox_wl = model.cox * p.weff_cv * p.leff_cv;
    if model.nqs_mod != 0 || model.acnqs_mod != 0 {
        let scaling_factor = 1.0e-9;
        charge.cqgb = -(charge.cggb + charge.cbgb);
        charge.cqdb = -(charge.cgdb + charge.cbdb);
        charge.cqsb = -(charge.cgsb + charge.cbsb);
        charge.cqbb = -(charge.cqgb + charge.cqdb + charge.cqsb);
        let gtau_drift = (inst.tconst * charge.qcheq).abs() * scaling_factor;
        let leff_cv_sq = p.leff_cv * p.leff_cv;
        let gtau_diff = 16.0 * inst.u0temp * mt.vtm / leff_cv_sq * scaling_factor;
        charge.gtau = gtau_drift + gtau_diff;
        charge.taunet = if model.acnqs_mod != 0 && charge.gtau > 0.0 && charge.gtau.is_finite() {
            scaling_factor / charge.gtau
        } else {
            0.0
        };
    }

    // --- Overlap charges (b3ld.c:2497-2557) ---
    let (cgdo, qgdo, cgso, qgso) = if model.cap_mod == 0 {
        (p.cgdo, p.cgdo * vgd, p.cgso, p.cgso * vgs)
    } else if model.cap_mod == 1 {
        let (cgdo, qgdo) = if vgd < 0.0 {
            let t1 = (1.0 - 4.0 * vgd / p.ckappa).sqrt();
            (
                p.cgdo + p.weff_cv * p.cgdl / t1,
                p.cgdo * vgd - p.weff_cv * 0.5 * p.cgdl * p.ckappa * (t1 - 1.0),
            )
        } else {
            (
                p.cgdo + p.weff_cv * p.cgdl,
                (p.weff_cv * p.cgdl + p.cgdo) * vgd,
            )
        };

        let (cgso, qgso) = if vgs < 0.0 {
            let t1 = (1.0 - 4.0 * vgs / p.ckappa).sqrt();
            (
                p.cgso + p.weff_cv * p.cgsl / t1,
                p.cgso * vgs - p.weff_cv * 0.5 * p.cgsl * p.ckappa * (t1 - 1.0),
            )
        } else {
            (
                p.cgso + p.weff_cv * p.cgsl,
                (p.weff_cv * p.cgsl + p.cgso) * vgs,
            )
        };
        (cgdo, qgdo, cgso, qgso)
    } else {
        let t0 = vgd + DELTA_1;
        let t1 = (t0 * t0 + 4.0 * DELTA_1).sqrt();
        let t2 = 0.5 * (t0 - t1);
        let t3 = p.weff_cv * p.cgdl;
        let t4 = (1.0 - 4.0 * t2 / p.ckappa).sqrt();
        let cgdo = p.cgdo + t3 - t3 * (1.0 - 1.0 / t4) * (0.5 - 0.5 * t0 / t1);
        let qgdo = (p.cgdo + t3) * vgd - t3 * (t2 + 0.5 * p.ckappa * (t4 - 1.0));

        let t0 = vgs + DELTA_1;
        let t1 = (t0 * t0 + 4.0 * DELTA_1).sqrt();
        let t2 = 0.5 * (t0 - t1);
        let t3 = p.weff_cv * p.cgsl;
        let t4 = (1.0 - 4.0 * t2 / p.ckappa).sqrt();
        let cgso = p.cgso + t3 - t3 * (1.0 - 1.0 / t4) * (0.5 - 0.5 * t0 / t1);
        let qgso = (p.cgso + t3) * vgs - t3 * (t2 + 0.5 * p.ckappa * (t4 - 1.0));
        (cgdo, qgdo, cgso, qgso)
    };

    charge.cgdo = cgdo;
    charge.cgso = cgso;
    charge.cgbo = p.cgbo;

    // --- Node-charge assembly (b3ld.c:2560-2790) ---
    let qgd = qgdo;
    let qgs = qgso;
    let qgb = p.cgbo * vgb;
    if model.nqs_mod == 0 && mode > 0 {
        qgate += qgd + qgs + qgb;
        qbulk -= qgb;
        qdrn -= qgd;
        // qsrc = -(qgate + qbulk + qdrn) implied.
    } else if model.nqs_mod == 0 {
        qgate += qgd + qgs + qgb;
        qbulk -= qgb;
        let qsrc = qdrn - qgs;
        qdrn = -(qgate + qbulk + qsrc);
    } else if mode > 0 {
        qgate = qgd + qgs + qgb;
        qbulk = -qgb;
        qdrn = -qgd;
        // qsrc = -(qgate + qbulk + qdrn) implied.
    } else {
        qgate = qgd + qgs + qgb;
        qbulk = -qgb;
        let qsrc = -qgs;
        qdrn = -(qgate + qbulk + qsrc);
    }

    charge.qgate = qgate;
    charge.qbulk = qbulk;
    charge.qdrn = qdrn;

    op.charge = Some(charge);
    Ok(op)
}

/// Source/drain junction depletion charge + capacitance (b3ld.c:2268-2430,
/// with ACM=1's devsup.c geometry helper). `vbs`/`vbd` are the raw
/// (mode-unswapped) junction voltages.
fn junction_depletion(
    model: &Bsim3v3Model,
    mt: &Bsim3v3ModelTemp,
    p: &Bsim3v3SizeDep,
    inst: &Bsim3v3InstTemp,
    vbs: Value,
    vbd: Value,
    charge: &mut Bsim3v3Charge,
) {
    // Zero-bias capacitances split between bottom, field-oxide sidewall and
    // gate-side sidewall (b3ld.c:2268-2297 plus ACM=1 devsup.c helper).
    let (czbd, czbdsw, czbdswg, czbs, czbssw, czbsswg) =
        junction_zero_bias_caps_acm0_or_1(model, mt, p, inst);

    let mj = model.bulk_jct_bot_grading_coeff;
    let mjsw = model.bulk_jct_side_grading_coeff;
    let mjswg = model.bulk_jct_gate_side_grading_coeff;

    // Source Bulk Junction (2333-2381).
    if vbs == 0.0 {
        charge.qbs = 0.0;
        charge.capbs = czbs + czbssw + czbsswg;
    } else if vbs < 0.0 {
        let mut qbs = 0.0;
        let mut capbs = 0.0;
        if czbs > 0.0 {
            let arg = 1.0 - vbs / mt.phi_b;
            let sarg = if mj == 0.5 {
                1.0 / arg.sqrt()
            } else {
                (-mj * arg.ln()).exp()
            };
            qbs = mt.phi_b * czbs * (1.0 - arg * sarg) / (1.0 - mj);
            capbs = czbs * sarg;
        }
        if czbssw > 0.0 {
            let arg = 1.0 - vbs / mt.phi_bsw;
            let sarg = if mjsw == 0.5 {
                1.0 / arg.sqrt()
            } else {
                (-mjsw * arg.ln()).exp()
            };
            qbs += mt.phi_bsw * czbssw * (1.0 - arg * sarg) / (1.0 - mjsw);
            capbs += czbssw * sarg;
        }
        if czbsswg > 0.0 {
            let arg = 1.0 - vbs / mt.phi_bswg;
            let sarg = if mjswg == 0.5 {
                1.0 / arg.sqrt()
            } else {
                (-mjswg * arg.ln()).exp()
            };
            qbs += mt.phi_bswg * czbsswg * (1.0 - arg * sarg) / (1.0 - mjswg);
            capbs += czbsswg * sarg;
        }
        charge.qbs = qbs;
        charge.capbs = capbs;
    } else {
        let t0 = czbs + czbssw + czbsswg;
        let t1 = vbs
            * (czbs * mj / mt.phi_b + czbssw * mjsw / mt.phi_bsw + czbsswg * mjswg / mt.phi_bswg);
        charge.qbs = vbs * (t0 + 0.5 * t1);
        charge.capbs = t0 + t1;
    }

    // Drain Bulk Junction (2383-2430).
    if vbd == 0.0 {
        charge.qbd = 0.0;
        charge.capbd = czbd + czbdsw + czbdswg;
    } else if vbd < 0.0 {
        let mut qbd = 0.0;
        let mut capbd = 0.0;
        if czbd > 0.0 {
            let arg = 1.0 - vbd / mt.phi_b;
            let sarg = if mj == 0.5 {
                1.0 / arg.sqrt()
            } else {
                (-mj * arg.ln()).exp()
            };
            qbd = mt.phi_b * czbd * (1.0 - arg * sarg) / (1.0 - mj);
            capbd = czbd * sarg;
        }
        if czbdsw > 0.0 {
            let arg = 1.0 - vbd / mt.phi_bsw;
            let sarg = if mjsw == 0.5 {
                1.0 / arg.sqrt()
            } else {
                (-mjsw * arg.ln()).exp()
            };
            qbd += mt.phi_bsw * czbdsw * (1.0 - arg * sarg) / (1.0 - mjsw);
            capbd += czbdsw * sarg;
        }
        if czbdswg > 0.0 {
            let arg = 1.0 - vbd / mt.phi_bswg;
            let sarg = if mjswg == 0.5 {
                1.0 / arg.sqrt()
            } else {
                (-mjswg * arg.ln()).exp()
            };
            qbd += mt.phi_bswg * czbdswg * (1.0 - arg * sarg) / (1.0 - mjswg);
            capbd += czbdswg * sarg;
        }
        charge.qbd = qbd;
        charge.capbd = capbd;
    } else {
        let t0 = czbd + czbdsw + czbdswg;
        let t1 = vbd
            * (czbd * mj / mt.phi_b + czbdsw * mjsw / mt.phi_bsw + czbdswg * mjswg / mt.phi_bswg);
        charge.qbd = vbd * (t0 + 0.5 * t1);
        charge.capbd = t0 + t1;
    }
}

fn junction_zero_bias_caps_acm0_or_1(
    model: &Bsim3v3Model,
    mt: &Bsim3v3ModelTemp,
    p: &Bsim3v3SizeDep,
    inst: &Bsim3v3InstTemp,
) -> (Value, Value, Value, Value, Value, Value) {
    if model.acm_mod == 1 {
        let width = p.width * model.wmlt + model.xw;
        let area = width * model.wmlt;
        let perimeter = width;
        let area_cap = area * mt.unit_area_temp_jct_cap;
        let side_cap = perimeter * mt.unit_length_sidewall_temp_jct_cap;
        (area_cap, side_cap, 0.0, area_cap, side_cap, 0.0)
    } else {
        let czbd = mt.unit_area_temp_jct_cap * inst.drain_area;
        let czbs = mt.unit_area_temp_jct_cap * inst.source_area;
        let (czbdsw, czbdswg);
        if inst.drain_perimeter < p.weff {
            czbdswg = mt.unit_length_gate_sidewall_temp_jct_cap * inst.drain_perimeter;
            czbdsw = 0.0;
        } else {
            czbdsw = mt.unit_length_sidewall_temp_jct_cap * (inst.drain_perimeter - p.weff);
            czbdswg = mt.unit_length_gate_sidewall_temp_jct_cap * p.weff;
        }
        let (czbssw, czbsswg);
        if inst.source_perimeter < p.weff {
            czbssw = 0.0;
            czbsswg = mt.unit_length_gate_sidewall_temp_jct_cap * inst.source_perimeter;
        } else {
            czbssw = mt.unit_length_sidewall_temp_jct_cap * (inst.source_perimeter - p.weff);
            czbsswg = mt.unit_length_gate_sidewall_temp_jct_cap * p.weff;
        }
        (czbd, czbdsw, czbdswg, czbs, czbssw, czbsswg)
    }
}
