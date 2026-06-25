//! BSIM4 v4.8 load equations (faithful port of ngspice-46 `b4ld.c`).
//!
//! This module transcribes the evaluation core of `BSIM4load` for the
//! canonical mode set plus caller-supplied source/drain junction body biases
//! used by the `rbodyMod=1/2` DC substrate network (`dioMod` 0/1/2, `mobMod`
//! 0 through 6, `capMod = 0/1/2`, integer `cvchargeMod = 0/1/2/3`):
//!
//! - the source/drain junction diode DC model with the `ijth` linearization
//!   (b4ld.c:700-890) and the reverse-bias trap-assisted tunneling current
//!   (903-1000),
//! - the normal/inverse mode select (1004-1017) and the whole DC current
//!   path: `Vbseff` with the forward-body correction, the Vth chain
//!   (DVT/DSUB charge sharing, `K1ox`/`K2ox`, narrow width, pocket-implant
//!   DITS incl. the v4.7 `DITS_SFT2`), subthreshold `n`, poly-gate
//!   depletion, the `Vgsteff` smoothing, effective W and `Rds(V)`, `Abulk`
//!   (with the separate `ketac` C-V copy), MOBMOD 0 through 6 mobility,
//!   `Vdsat`
//!   (incl. the velocity-overshoot `lambda` and source-end velocity-limit
//!   `vtl` options), `Vdseff`, the Early stack (`Vasat`/`VACLM`/`VADIBL`/
//!   `VADITS`/`VASCBE`), `Ids` with analytic `Gm`/`Gds`/`Gmb`, the
//!   substrate current, and both GIDL/GISL models (`gidlMod` 0 and the
//!   v4.7 `gidlMod = 1`) (1021-2520),
//! - the CAPMOD=0/1/2 intrinsic charge models with the full capacitance
//!   matrix (3026-3920), the junction depletion charges (3978-4065),
//!   CAPMOD=0 linear overlap and CAPMOD=1/2 smoothed overlap charges
//!   (4138-4175), and the mode-dependent node-charge assembly for
//!   `trnqsMod = 0` (4188-4665).
//!
//! All derivatives are the analytic expressions of the C source,
//! transcribed with the same temporary structure (`t0`..`t14`, `tmp*`) so
//! the Rust can be diffed against `b4ld.c` line by line.
//!
//! Scope notes:
//! - unknown `cvchargeMod` charge selectors beyond `0/1/2/3` are not ported;
//!   nonzero supported selectors share ngspice's nonzero branch;
//!   requesting charges under them is a typed error (the DC path is
//!   capMod-independent).
//! - `gmin` is an explicit argument (ngspice `CKTgmin`): the diode currents
//!   and conductances include it exactly as the C does, so a caller wiring
//!   this into an engine must not add a second per-device gmin on top.
//! - The temperature entering `TempRatio` (b4ld.c:1133) and the diode
//!   `Nvtm*` is the one the temperature pass ran at; ngspice-46's BSIM4
//!   accepts an instance `dtemp` but never uses it, and this port keeps
//!   that behavior.

use super::common::{
    CHARGE_Q, CONST_CHARGE, DELTA_1, DELTA_3, DELTA_4, EPS0, EPSSI, EXP_THRESHOLD, EXPL_THRESHOLD,
    MAX_EXP, MAX_EXPL, MIN_EXP, MIN_EXPL, MM, dexp,
};
use super::params::Bsim4v8Model;
use super::temp::{Bsim4v8InstTemp, Bsim4v8ModelTemp, Bsim4v8SizeDep};
use crate::Value;

/// Input branch voltages, in device polarity (`mtype` folded in by the
/// caller) and already limited — the `vbs`/`vgs`/`vds` of b4ld.c after
/// `DEVfetlim`/`DEVlimvds`/`DEVpnjlim`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bsim4v8Bias {
    pub vds: Value,
    pub vgs: Value,
    pub vbs: Value,
}

/// Source/drain junction body biases for `rbodyMod > 0`.
///
/// `vbs` is the source-body diode voltage (`sbNode - sNodePrime`) and `vbd`
/// is the drain-body diode voltage (`dbNode - dNodePrime`), both already in
/// device polarity. The channel equations still use [`Bsim4v8Bias::vbs`],
/// which is the body-prime to source-prime voltage.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bsim4v8JunctionBias {
    pub vbs: Value,
    pub vbd: Value,
}

/// Linearized operating point of one BSIM4 v4.8 instance.
///
/// Field names mirror the `here->BSIM4*` slots that ngspice fills during
/// the load and consumes when stamping. All quantities are in the device's
/// own polarity; `mode` records the normal(+1)/inverse(-1) channel
/// direction. Everything that b4ld.c multiplies by `nf` is `nf`-scaled
/// here; none of the fields carries the `m` multiplier — ngspice applies
/// `m` at stamp time and so must the caller.
#[derive(Debug, Clone, Default)]
pub struct Bsim4v8Op {
    pub mode: i32,

    /// Channel current `Ids` (`BSIM4cd`, the `cdrain` of the load).
    pub cd: Value,
    /// Substrate (impact-ionization) current (`BSIM4csub`).
    pub csub: Value,
    /// Gate-induced drain/source leakage (`BSIM4Igidl`/`BSIM4Igisl`).
    pub igidl: Value,
    pub igisl: Value,
    /// Source/drain junction diode currents (`BSIM4cbs`/`BSIM4cbd`).
    pub cbs: Value,
    pub cbd: Value,

    // Channel-current linearization (b4ld stores these mode-unswapped; the
    // stamp applies the swap).
    pub gm: Value,
    pub gds: Value,
    pub gmbs: Value,
    // Junction diode conductances.
    pub gbd: Value,
    pub gbs: Value,
    // Substrate-current linearization (`BSIM4gbbs`/`gbgs`/`gbds`).
    pub gbbs: Value,
    pub gbgs: Value,
    pub gbds: Value,
    // GIDL/GISL linearization.
    pub ggidld: Value,
    pub ggidlg: Value,
    pub ggidlb: Value,
    pub ggidls: Value,
    pub ggisls: Value,
    pub ggislg: Value,
    pub ggislb: Value,
    pub ggisld: Value,

    // Gate tunneling currents and linearization (`BSIM4Ig*`/`gIg*`).
    pub igcs: Value,
    pub gigcsg: Value,
    pub gigcsd: Value,
    pub gigcss: Value,
    pub gigcsb: Value,
    pub igcd: Value,
    pub gigcdg: Value,
    pub gigcdd: Value,
    pub gigcds: Value,
    pub gigcdb: Value,
    pub igs: Value,
    pub gigsg: Value,
    pub gigss: Value,
    pub igd: Value,
    pub gigdg: Value,
    pub gigdd: Value,
    pub igb: Value,
    pub gigbg: Value,
    pub gigbd: Value,
    pub gigbs: Value,
    pub gigbb: Value,

    /// Threshold voltage at the operating point (`BSIM4von`).
    pub von: Value,
    /// Saturation voltage (`BSIM4vdsat`).
    pub vdsat: Value,
    /// `Vdseff` (`BSIM4Vdseff`).
    pub vdseff: Value,
    /// `Vgsteff` of the DC path (`BSIM4Vgsteff`).
    pub vgsteff: Value,
    /// Effective mobility (`BSIM4ueff`).
    pub ueff: Value,
    /// `Abulk` (`BSIM4Abulk`).
    pub abulk: Value,
    /// Bias-dependent internal S/D series resistance (per finger).
    pub rds: Value,
    /// `1 / Rds * nf` or 0 (`BSIM4grdsw`).
    pub grdsw: Value,
    /// `dvt0 * Theta0` (`BSIM4thetavth`).
    pub thetavth: Value,
    /// `Esat * Leff` after velocity overshoot (`BSIM4EsatL`).
    pub esat_l: Value,
    /// `Abulk / (Vgsteff + 2 vtm)` (`BSIM4AbovVgst2Vtm`).
    pub ab_ov_vgst2vtm: Value,
    /// `Ids/Vds` proxy clamped at `idovvdsc` (`BSIM4IdovVds`).
    pub idovvds: Value,
    /// DC effective oxide capacitance (`BSIM4Coxeff`).
    pub coxeff: Value,
    /// `vtm/q (coxe + epssub/Xdep + cit)` (`BSIM4nstar`).
    pub nstar: Value,
    /// Inversion-charge proxy for noise (`BSIM4qinv`, tnoiMod = 0).
    pub qinv: Value,
    /// Zero-bias channel conductance for correlated thermal noise
    /// (`BSIM4noiGd0`, tnoiMod = 2).
    pub noi_gd0: Value,

    /// Bias-dependent gate-resistance branch for `RGATEMOD=2`
    /// (`BSIM4gcrg` and derivatives).
    pub gcrg: Value,
    pub gcrgg: Value,
    pub gcrgd: Value,
    pub gcrgs: Value,
    pub gcrgb: Value,

    /// Charge state (set only when [`eval`] is asked to compute it).
    pub charge: Option<Bsim4v8Charge>,
}

/// Charge-model output for one BSIM4 v4.8 instance (capMod = 0/1/2).
///
/// `qgate`/`qbulk`/`qdrn`/`qsrc` are the **intrinsic** charges exactly as
/// stored in `here->BSIM4qgate`/... (what `@m[qg]` and friends report);
/// the overlap lumping of the transient companion happens on local copies
/// in the C and is reproduced by the `q*_node`/`q*_state` fields below.
/// The `c***` entries are the intrinsic capacitance matrix exactly as
/// stored in `here->BSIM4c***` (before the `ag0`/mode assembly); `capbd`/
/// `capbs` the junction depletion capacitances; `cgdo`/`cgso`/`qgdo`/
/// `qgso` the bias-dependent effective overlap capacitances and charges of
/// b4ld.c:4148-4175 (`nf`-scaled).
#[derive(Debug, Clone, Default)]
pub struct Bsim4v8Charge {
    pub qgate: Value,
    pub qbulk: Value,
    pub qdrn: Value,
    pub qsrc: Value,
    pub qchqs: Value,
    pub cox_wl: Value,
    pub taunet: Value,
    pub gcrg: Value,
    pub gcrgg: Value,
    pub gcrgd: Value,
    pub gcrgs: Value,
    pub gcrgb: Value,

    /// Source/drain junction depletion charges (`CKTstate qbs`/`qbd`).
    pub qbs: Value,
    pub qbd: Value,
    pub capbs: Value,
    pub capbd: Value,

    // Intrinsic capacitance matrix.
    pub cggb: Value,
    pub cgdb: Value,
    pub cgsb: Value,
    pub cdgb: Value,
    pub cddb: Value,
    pub cdsb: Value,
    pub cbgb: Value,
    pub cbdb: Value,
    pub cbsb: Value,
    // Derived rows (b4ld.c:3923-3929).
    pub csgb: Value,
    pub csdb: Value,
    pub cssb: Value,
    pub cgbb: Value,
    pub cdbb: Value,
    pub cbbb: Value,
    pub csbb: Value,

    /// Effective overlap capacitances/charges (`BSIM4cgdo`/`qgdo`/...).
    pub cgdo: Value,
    pub qgdo: Value,
    pub cgso: Value,
    pub qgso: Value,
    /// Gate-bulk overlap capacitance (`pParam->BSIM4cgbo`, bias-independent).
    pub cgbo: Value,

    /// Node charges after the mode-dependent overlap lumping
    /// (b4ld.c:4190-4427), i.e. the local `qgate`/`qdrn`/`qsrc`/`qbulk`
    /// right before the CKTstate composition.
    pub qg_node: Value,
    pub qgmid_node: Value,
    pub qd_node: Value,
    pub qs_node: Value,
    pub qb_node: Value,
}

impl Bsim4v8Charge {
    /// State-vector gate charge (`CKTstate0 qg`).
    pub fn qg_state(&self) -> Value {
        self.qg_node
    }
    /// State-vector middle-gate overlap charge (`CKTstate0 qgmid`) for
    /// `rgateMod=3`.
    pub fn qgmid_state(&self) -> Value {
        self.qgmid_node
    }
    /// State-vector drain charge (`CKTstate0 qd = qdrn - qbd`).
    pub fn qd_state(&self) -> Value {
        self.qd_node - self.qbd
    }
    /// State-vector source charge (`CKTstate0 qs = qsrc - qbs`).
    pub fn qs_state(&self) -> Value {
        self.qs_node - self.qbs
    }
    /// State-vector bulk charge (`CKTstate0 qb = qbulk + qbd + qbs`).
    pub fn qb_state(&self) -> Value {
        self.qb_node + self.qbd + self.qbs
    }
    /// State-vector bulk charge for the active body topology. With
    /// `rbodyMod>0`, ngspice integrates `qb = qbulk` and keeps `qbs`/`qbd`
    /// as separate states.
    pub fn qb_state_for_rbody(&self, rbody_enabled: bool) -> Value {
        if rbody_enabled {
            self.qb_node
        } else {
            self.qb_state()
        }
    }
}

/// `BSIM4polyDepletion` (b4ld.c:5402-5433). Returns `(vgs_eff,
/// dvgs_eff_dvg)`. Note this is the one BSIM4 routine on the modern
/// `CHARGE` constant rather than the truncated `Charge_q`.
fn poly_depletion(
    phi: Value,
    ngate: Value,
    epsgate: Value,
    coxe: Value,
    vgs: Value,
) -> (Value, Value) {
    if ngate > 1.0e18 && ngate < 1.0e25 && vgs > phi && epsgate != 0.0 {
        let t1 = 1.0e6 * CONST_CHARGE * epsgate * ngate / (coxe * coxe);
        let t8 = vgs - phi;
        let t4 = (1.0 + 2.0 * t8 / t1).sqrt();
        let t2 = 2.0 * t8 / (t4 + 1.0);
        let t3 = 0.5 * t2 * t2 / t1; // T3 = Vpoly
        let t7 = 1.12 - t3 - 0.05;
        let t6 = (t7 * t7 + 0.224).sqrt();
        let t5 = 1.12 - 0.5 * (t7 + t6);
        (vgs - t5, 1.0 - (0.5 - 0.5 / t4) * (1.0 + t7 / t6))
    } else {
        (vgs, 1.0)
    }
}

/// Evaluate the BSIM4 v4.8 operating point, optionally including the
/// charge model (`compute_charges == true`, the `ChargeComputationNeeded`
/// path of b4ld.c).
///
/// Errors only when charges are requested with an unported charge model
/// (`capMod` 0 or an unknown `cvchargeMod` selector).
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn eval(
    model: &Bsim4v8Model,
    mt: &Bsim4v8ModelTemp,
    p: &Bsim4v8SizeDep,
    inst: &Bsim4v8InstTemp,
    bias: Bsim4v8Bias,
    junction_bias: Option<Bsim4v8JunctionBias>,
    gate_mid_vgs: Option<Value>,
    gmin: Value,
    compute_charges: bool,
) -> Result<Bsim4v8Op, String> {
    if compute_charges {
        if !(0..=2).contains(&model.cap_mod) {
            return Err(format!(
                "BSIM4: CAPMOD={} charge model is not implemented (only CAPMOD=0, 1, or 2)",
                model.cap_mod
            ));
        }
        if !model.cvcharge_mod_supported_for_charges() {
            return Err(format!(
                "BSIM4: CVCHARGEMOD={} charge model is not implemented (only integer 0, 1, 2, or 3)",
                model.cvcharge_mod_value
            ));
        }
    }

    let mut op = Bsim4v8Op::default();
    let nf = inst.nf;

    let vds = bias.vds;
    let vgs = bias.vgs;
    let vbs = bias.vbs;
    let vbd = vbs - vds;
    let vgd = vgs - vds;
    let vgb = vgs - vbs;
    let vgms = gate_mid_vgs.unwrap_or(vgs);
    let vgmd = vgms - vds;
    let vgmb = vgms - vbs;

    // rbodyMod = 0 uses the channel-side body bias; rbodyMod > 0 supplies
    // separate source/drain body-node biases from the device wrapper.
    let (vbs_jct, vbd_jct) = junction_bias.map(|j| (j.vbs, j.vbd)).unwrap_or((vbs, vbd));

    // --- Source/drain junction diode DC model (b4ld.c:700-890) ---
    let nvtms = mt.vtm * mt.njs;
    let source_sat_current = inst.source_sat_current;
    if source_sat_current <= 0.0 {
        op.gbs = gmin;
        op.cbs = op.gbs * vbs_jct;
    } else {
        match model.dio_mod {
            0 => {
                let evbs = (vbs_jct / nvtms).exp();
                let t1 = model.xjbvs * (-(mt.bvs + vbs_jct) / nvtms).exp();
                op.gbs = source_sat_current * (evbs + t1) / nvtms + gmin;
                op.cbs = source_sat_current * (evbs + inst.xexp_bvs - t1 - 1.0) + gmin * vbs_jct;
            }
            1 => {
                let t2 = vbs_jct / nvtms;
                if t2 < -EXP_THRESHOLD {
                    op.gbs = gmin;
                    op.cbs = source_sat_current * (MIN_EXP - 1.0) + gmin * vbs_jct;
                } else {
                    let (vjsm_fwd, i_vjsm_fwd) = inst
                        .vjsm_fwd
                        .expect("dioMod=1 anchors exist when SourceSatCurrent > 0");
                    if vbs_jct <= vjsm_fwd {
                        let evbs = t2.exp();
                        op.gbs = source_sat_current * evbs / nvtms + gmin;
                        op.cbs = source_sat_current * (evbs - 1.0) + gmin * vbs_jct;
                    } else {
                        let t0 = i_vjsm_fwd / nvtms;
                        op.gbs = t0 + gmin;
                        op.cbs = i_vjsm_fwd - source_sat_current
                            + t0 * (vbs_jct - vjsm_fwd)
                            + gmin * vbs_jct;
                    }
                }
            }
            2 => {
                let (vjsm_fwd, i_vjsm_fwd) = inst
                    .vjsm_fwd
                    .expect("dioMod=2 anchors exist when SourceSatCurrent > 0");
                if vbs_jct < inst.vjsm_rev {
                    let t0 = vbs_jct / nvtms;
                    let (evbs, devbs_dvb) = if t0 < -EXP_THRESHOLD {
                        (MIN_EXP, 0.0)
                    } else {
                        let evbs = t0.exp();
                        (evbs, evbs / nvtms)
                    };
                    let t1 = evbs - 1.0;
                    let t2 = inst.s_iv_rev + inst.s_slp_rev * (vbs_jct - inst.vjsm_rev);
                    op.gbs = devbs_dvb * t2 + t1 * inst.s_slp_rev + gmin;
                    op.cbs = t1 * t2 + gmin * vbs_jct;
                } else if vbs_jct <= vjsm_fwd {
                    let t0 = vbs_jct / nvtms;
                    let (evbs, devbs_dvb) = if t0 < -EXP_THRESHOLD {
                        (MIN_EXP, 0.0)
                    } else {
                        let evbs = t0.exp();
                        (evbs, evbs / nvtms)
                    };
                    let t1 = (mt.bvs + vbs_jct) / nvtms;
                    let (t2, t3) = if t1 > EXP_THRESHOLD {
                        (MIN_EXP, 0.0)
                    } else {
                        let t2 = (-t1).exp();
                        (t2, -t2 / nvtms)
                    };
                    op.gbs = source_sat_current * (devbs_dvb - model.xjbvs * t3) + gmin;
                    op.cbs = source_sat_current * (evbs + inst.xexp_bvs - 1.0 - model.xjbvs * t2)
                        + gmin * vbs_jct;
                } else {
                    op.gbs = inst.s_slp_fwd + gmin;
                    op.cbs = i_vjsm_fwd + inst.s_slp_fwd * (vbs_jct - vjsm_fwd) + gmin * vbs_jct;
                }
            }
            _ => unreachable!("validated BSIM4 dioMod selector"),
        }
    }

    let nvtmd = mt.vtm * mt.njd;
    let drain_sat_current = inst.drain_sat_current;
    if drain_sat_current <= 0.0 {
        op.gbd = gmin;
        op.cbd = op.gbd * vbd_jct;
    } else {
        match model.dio_mod {
            0 => {
                let evbd = (vbd_jct / nvtmd).exp();
                let t1 = model.xjbvd * (-(mt.bvd + vbd_jct) / nvtmd).exp();
                op.gbd = drain_sat_current * (evbd + t1) / nvtmd + gmin;
                op.cbd = drain_sat_current * (evbd + inst.xexp_bvd - t1 - 1.0) + gmin * vbd_jct;
            }
            1 => {
                let t2 = vbd_jct / nvtmd;
                if t2 < -EXP_THRESHOLD {
                    op.gbd = gmin;
                    op.cbd = drain_sat_current * (MIN_EXP - 1.0) + gmin * vbd_jct;
                } else {
                    let (vjdm_fwd, i_vjdm_fwd) = inst
                        .vjdm_fwd
                        .expect("dioMod=1 anchors exist when DrainSatCurrent > 0");
                    if vbd_jct <= vjdm_fwd {
                        let evbd = t2.exp();
                        op.gbd = drain_sat_current * evbd / nvtmd + gmin;
                        op.cbd = drain_sat_current * (evbd - 1.0) + gmin * vbd_jct;
                    } else {
                        let t0 = i_vjdm_fwd / nvtmd;
                        op.gbd = t0 + gmin;
                        op.cbd = i_vjdm_fwd - drain_sat_current
                            + t0 * (vbd_jct - vjdm_fwd)
                            + gmin * vbd_jct;
                    }
                }
            }
            2 => {
                let (vjdm_fwd, i_vjdm_fwd) = inst
                    .vjdm_fwd
                    .expect("dioMod=2 anchors exist when DrainSatCurrent > 0");
                if vbd_jct < inst.vjdm_rev {
                    let t0 = vbd_jct / nvtmd;
                    let (evbd, devbd_dvb) = if t0 < -EXP_THRESHOLD {
                        (MIN_EXP, 0.0)
                    } else {
                        let evbd = t0.exp();
                        (evbd, evbd / nvtmd)
                    };
                    let t1 = evbd - 1.0;
                    let t2 = inst.d_iv_rev + inst.d_slp_rev * (vbd_jct - inst.vjdm_rev);
                    op.gbd = devbd_dvb * t2 + t1 * inst.d_slp_rev + gmin;
                    op.cbd = t1 * t2 + gmin * vbd_jct;
                } else if vbd_jct <= vjdm_fwd {
                    let t0 = vbd_jct / nvtmd;
                    let (evbd, devbd_dvb) = if t0 < -EXP_THRESHOLD {
                        (MIN_EXP, 0.0)
                    } else {
                        let evbd = t0.exp();
                        (evbd, evbd / nvtmd)
                    };
                    let t1 = (mt.bvd + vbd_jct) / nvtmd;
                    let (t2, t3) = if t1 > EXP_THRESHOLD {
                        (MIN_EXP, 0.0)
                    } else {
                        let t2 = (-t1).exp();
                        (t2, -t2 / nvtmd)
                    };
                    op.gbd = drain_sat_current * (devbd_dvb - model.xjbvd * t3) + gmin;
                    op.cbd = drain_sat_current * (evbd + inst.xexp_bvd - 1.0 - model.xjbvd * t2)
                        + gmin * vbd_jct;
                } else {
                    op.gbd = inst.d_slp_fwd + gmin;
                    op.cbd = i_vjdm_fwd + inst.d_slp_fwd * (vbd_jct - vjdm_fwd) + gmin * vbd_jct;
                }
            }
            _ => unreachable!("validated BSIM4 dioMod selector"),
        }
    }

    // --- Trap-assisted tunneling and recombination current (903-1000) ---
    {
        let nvtmrssws = mt.vtm0 * mt.njtsswstemp;
        let nvtmrsswgs = mt.vtm0 * mt.njtsswgstemp;
        let nvtmrss = mt.vtm0 * mt.njtsstemp;
        let nvtmrsswd = mt.vtm0 * mt.njtsswdtemp;
        let nvtmrsswgd = mt.vtm0 * mt.njtsswgdtemp;
        let nvtmrsd = mt.vtm0 * mt.njtsdtemp;

        // One leg of the six identical guarded-exponential forms.
        let tat = |vts: Value, vjct: Value, nvtmr: Value| -> (Value, Value) {
            if vts - vjct < vts * 1.0e-3 {
                let t9 = 1.0e3;
                let t0 = -vjct / nvtmr * t9;
                let (t1, t10) = dexp(t0);
                (t1, t10 / nvtmr * t9)
            } else {
                let t9 = 1.0 / (vts - vjct);
                let t0 = -vjct / nvtmr * vts * t9;
                let dt0_dvb = vts / nvtmr * (t9 + vjct * t9 * t9);
                let (t1, t10) = dexp(t0);
                (t1, t10 * dt0_dvb)
            }
        };
        let (t1, dt1_dvb) = tat(model.vtss, vbs_jct, nvtmrss);
        let (t2, dt2_dvb) = tat(model.vtsd, vbd_jct, nvtmrsd);
        let (t3, dt3_dvb) = tat(model.vtssws, vbs_jct, nvtmrssws);
        let (t4, dt4_dvb) = tat(model.vtsswd, vbd_jct, nvtmrsswd);
        let (t5, dt5_dvb) = tat(model.vtsswgs, vbs_jct, nvtmrsswgs);
        let (t6, dt6_dvb) = tat(model.vtsswgd, vbd_jct, nvtmrsswgd);

        op.gbs += inst.s_jct_temp_rev_sat_cur * dt1_dvb
            + inst.s_sw_temp_rev_sat_cur * dt3_dvb
            + inst.s_swg_temp_rev_sat_cur * dt5_dvb;
        op.cbs -= inst.s_jct_temp_rev_sat_cur * (t1 - 1.0)
            + inst.s_sw_temp_rev_sat_cur * (t3 - 1.0)
            + inst.s_swg_temp_rev_sat_cur * (t5 - 1.0);
        op.gbd += inst.d_jct_temp_rev_sat_cur * dt2_dvb
            + inst.d_sw_temp_rev_sat_cur * dt4_dvb
            + inst.d_swg_temp_rev_sat_cur * dt6_dvb;
        op.cbd -= inst.d_jct_temp_rev_sat_cur * (t2 - 1.0)
            + inst.d_sw_temp_rev_sat_cur * (t4 - 1.0)
            + inst.d_swg_temp_rev_sat_cur * (t6 - 1.0);
    }

    // --- Mode select (1004-1017) ---
    // The C's mode-swapped `Vgs` only feeds BSIM4polyDepletion, which reads
    // the raw vgs/vgd directly, so no swapped copy is kept here.
    let mode;
    let (vds_c, vgs_c, vbs_c);
    if vds >= 0.0 {
        mode = 1;
        vds_c = vds;
        vgs_c = vgs;
        vbs_c = vbs;
    } else {
        mode = -1;
        vds_c = -vds;
        vgs_c = vgd;
        vbs_c = vbd;
    }
    op.mode = mode;

    // b4ld.c "dunga" material constants.
    let epsrox = model.effective_epsrox();
    let toxe = model.effective_toxe();
    let epssub = if model.mtrl_mod != 0 {
        EPS0 * model.epsrsub
    } else {
        EPSSI
    };

    // --- Vbseff (1035-1053) ---
    let t0 = vbs_c - inst.vbsc - 0.001;
    let t1 = (t0 * t0 - 0.004 * inst.vbsc).sqrt();
    let (mut vbseff, mut dvbseff_dvb);
    if t0 >= 0.0 {
        vbseff = inst.vbsc + 0.5 * (t0 + t1);
        dvbseff_dvb = 0.5 * (1.0 + t0 / t1);
    } else {
        let t2 = -0.002 / (t1 - t0);
        vbseff = inst.vbsc * (1.0 + t2);
        dvbseff_dvb = t2 * inst.vbsc / t1;
    }

    // JX: Correction to forward body bias.
    let t9 = 0.95 * p.phi;
    let t0 = t9 - vbseff - 0.001;
    let t1 = (t0 * t0 + 0.004 * t9).sqrt();
    vbseff = t9 - 0.5 * (t0 + t1);
    dvbseff_dvb *= 0.5 * (1.0 + t0 / t1);

    let phis = p.phi - vbseff;
    let sqrt_phis = phis.sqrt();
    let dsqrt_phis_dvb = -0.5 / sqrt_phis;

    let xdep = p.xdep0 * sqrt_phis / p.sqrt_phi;
    let dxdep_dvb = (p.xdep0 / p.sqrt_phi) * dsqrt_phis_dvb;

    let leff = p.leff;
    let vtm = mt.vtm;
    let vtm0 = mt.vtm0;

    // --- Vth calculation (1067-1165) ---
    let t3 = xdep.sqrt();
    let v0 = p.vbi - p.phi;

    let t0 = p.dvt2 * vbseff;
    let (t1, t2);
    if t0 >= -0.5 {
        t1 = 1.0 + t0;
        t2 = p.dvt2;
    } else {
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

    let t0 = p.dvt1 * leff / lt1;
    let (theta0, dtheta0_dvb);
    if t0 < EXP_THRESHOLD {
        let t1 = t0.exp();
        let t2 = t1 - 1.0;
        let t3 = t2 * t2;
        let t4 = t3 + 2.0 * t1 * MIN_EXP;
        theta0 = t1 / t4;
        let dt1_dvb = -t0 * t1 * dlt1_dvb / lt1;
        dtheta0_dvb = dt1_dvb * (t4 - 2.0 * t1 * (t2 + MIN_EXP)) / t4 / t4;
    } else {
        theta0 = 1.0 / (MAX_EXP - 2.0); // 3.0 * MIN_EXP omitted
        dtheta0_dvb = 0.0;
    }
    op.thetavth = p.dvt0 * theta0;
    let delt_vth = op.thetavth * v0;
    let ddelt_vth_dvb = p.dvt0 * dtheta0_dvb * v0;

    let t0 = p.dvt1w * p.weff * leff / ltw;
    let (t5, dt5_dvb);
    if t0 < EXP_THRESHOLD {
        let t1 = t0.exp();
        let t2 = t1 - 1.0;
        let t3 = t2 * t2;
        let t4 = t3 + 2.0 * t1 * MIN_EXP;
        t5 = t1 / t4;
        let dt1_dvb = -t0 * t1 * dltw_dvb / ltw;
        dt5_dvb = dt1_dvb * (t4 - 2.0 * t1 * (t2 + MIN_EXP)) / t4 / t4;
    } else {
        t5 = 1.0 / (MAX_EXP - 2.0); // 3.0 * MIN_EXP omitted
        dt5_dvb = 0.0;
    }
    let t0 = p.dvt0w * t5;
    let t2 = t0 * v0;
    let dt2_dvb = p.dvt0w * dt5_dvb * v0;

    let temp_ratio = mt.temp_ratio_m1;
    let t0 = (1.0 + p.lpe0 / leff).sqrt();
    let t1 =
        p.k1ox * (t0 - 1.0) * p.sqrt_phi + (p.kt1 + p.kt1l / leff + p.kt2 * vbseff) * temp_ratio;
    let vth_narrow_w = toxe * p.phi / (p.weff + p.w0);

    let t3v = inst.eta0 + p.etab * vbseff;
    let (t3v, t4) = if t3v < 1.0e-4 {
        let t9 = 1.0 / (3.0 - 2.0e4 * t3v);
        ((2.0e-4 - t3v) * t9, t9 * t9)
    } else {
        (t3v, 1.0)
    };
    let ddibl_sft_dvd = t3v * p.theta0vb0;
    let dibl_sft = ddibl_sft_dvd * vds_c;

    let lpe_vb = (1.0 + p.lpeb / leff).sqrt();

    let mut vth = model.mtype * inst.vth0 + (p.k1ox * sqrt_phis - p.k1 * p.sqrt_phi) * lpe_vb
        - inst.k2ox * vbseff
        - delt_vth
        - t2
        + (p.k3 + p.k3b * vbseff) * vth_narrow_w
        + t1
        - dibl_sft;

    let mut dvth_dvb = lpe_vb * p.k1ox * dsqrt_phis_dvb - inst.k2ox - ddelt_vth_dvb - dt2_dvb
        + p.k3b * vth_narrow_w
        - p.etab * vds_c * p.theta0vb0 * t4
        + p.kt2 * temp_ratio;
    let mut dvth_dvd = -ddibl_sft_dvd;

    // --- Calculate n (1167-1189) ---
    let tmp1 = epssub / xdep;
    op.nstar = vtm / CHARGE_Q * (mt.coxe + tmp1 + p.cit);
    let tmp2 = p.nfactor * tmp1;
    let tmp3 = p.cdsc + p.cdscb * vbseff + p.cdscd * vds_c;
    let tmp4 = (tmp2 + tmp3 * theta0 + p.cit) / mt.coxe;
    let (n, dn_dvb, dn_dvd);
    if tmp4 >= -0.5 {
        n = 1.0 + tmp4;
        dn_dvb = (-tmp2 / xdep * dxdep_dvb + tmp3 * dtheta0_dvb + p.cdscb * theta0) / mt.coxe;
        dn_dvd = p.cdscd * theta0 / mt.coxe;
    } else {
        let t0 = 1.0 / (3.0 + 8.0 * tmp4);
        n = (1.0 + 3.0 * tmp4) * t0;
        let t0 = t0 * t0;
        dn_dvb = (-tmp2 / xdep * dxdep_dvb + tmp3 * dtheta0_dvb + p.cdscb * theta0) / mt.coxe * t0;
        dn_dvd = p.cdscd * theta0 / mt.coxe * t0;
    }

    // --- Vth correction for pocket implant (1191-1221) ---
    if p.dvtp0 > 0.0 {
        let t0 = -p.dvtp1 * vds_c;
        let (t2, dt2_dvd);
        if t0 < -EXP_THRESHOLD {
            t2 = MIN_EXP;
            dt2_dvd = 0.0;
        } else {
            t2 = t0.exp();
            dt2_dvd = -p.dvtp1 * t2;
        }
        let t3 = leff + p.dvtp0 * (1.0 + t2);
        let dt3_dvd = p.dvtp0 * dt2_dvd;
        let (t4, dt4_dvd) = if model.temp_mod < 2 {
            (vtm * (leff / t3).ln(), -vtm * dt3_dvd / t3)
        } else {
            (vtm0 * (leff / t3).ln(), -vtm0 * dt3_dvd / t3)
        };
        let ddits_sft_dvd = dn_dvd * t4 + n * dt4_dvd;
        let ddits_sft_dvb = t4 * dn_dvb;

        vth -= n * t4;
        dvth_dvd -= ddits_sft_dvd;
        dvth_dvb -= ddits_sft_dvb;
    }

    // v4.7 DITS_SFT2.
    if p.dvtp4 != 0.0 && p.dvtp2factor != 0.0 {
        let t1 = 2.0 * p.dvtp4 * vds_c;
        let (t0, t10) = dexp(t1);
        let dits_sft2 = p.dvtp2factor * (t0 - 1.0) / (t0 + 1.0);
        let ddits_sft2_dvd = p.dvtp2factor * p.dvtp4 * 4.0 * t10 / ((t0 + 1.0) * (t0 + 1.0));
        vth -= dits_sft2;
        dvth_dvd -= ddits_sft2_dvd;
    }

    op.von = vth;

    // --- Poly gate Si depletion effect (1245-1268) ---
    let t0 = inst.vfb + p.phi;
    let t1 = if model.mtrl_mod == 0 {
        EPSSI
    } else {
        model.epsrgate * EPS0
    };
    let (vgs_eff_raw, dvgs_eff_dvg_raw) = poly_depletion(t0, p.ngate, t1, mt.coxe, vgs);
    let (vgd_eff_raw, dvgd_eff_dvg_raw) = poly_depletion(t0, p.ngate, t1, mt.coxe, vgd);
    let (vgs_eff, dvgs_eff_dvg) = if mode > 0 {
        (vgs_eff_raw, dvgs_eff_dvg_raw)
    } else {
        (vgd_eff_raw, dvgd_eff_dvg_raw)
    };

    let vgst = vgs_eff - vth;

    // --- Calculate Vgsteff (1272-1330) ---
    let t0 = n * vtm;
    let t1 = p.mstar * vgst;
    let t2 = t1 / t0;
    let (t10, dt10_dvg, dt10_dvd, dt10_dvb);
    if t2 > EXP_THRESHOLD {
        t10 = t1;
        dt10_dvg = p.mstar * dvgs_eff_dvg;
        dt10_dvd = -dvth_dvd * p.mstar;
        dt10_dvb = -dvth_dvb * p.mstar;
    } else if t2 < -EXP_THRESHOLD {
        let t10p = vtm * (1.0 + MIN_EXP).ln();
        dt10_dvg = 0.0;
        dt10_dvd = t10p * dn_dvd;
        dt10_dvb = t10p * dn_dvb;
        t10 = t10p * n;
    } else {
        let exp_vgst = t2.exp();
        let t3 = vtm * (1.0 + exp_vgst).ln();
        t10 = n * t3;
        let dt10g = p.mstar * exp_vgst / (1.0 + exp_vgst);
        dt10_dvb = t3 * dn_dvb - dt10g * (dvth_dvb + vgst * dn_dvb / n);
        dt10_dvd = t3 * dn_dvd - dt10g * (dvth_dvd + vgst * dn_dvd / n);
        dt10_dvg = dt10g * dvgs_eff_dvg;
    }

    let t1 = p.voffcbn - (1.0 - p.mstar) * vgst;
    let t2 = t1 / t0;
    let (t9, dt9_dvg, dt9_dvd, dt9_dvb);
    if t2 < -EXP_THRESHOLD {
        let t3 = mt.coxe * MIN_EXP / p.cdep0;
        t9 = p.mstar + t3 * n;
        dt9_dvg = 0.0;
        dt9_dvd = dn_dvd * t3;
        dt9_dvb = dn_dvb * t3;
    } else if t2 > EXP_THRESHOLD {
        let t3 = mt.coxe * MAX_EXP / p.cdep0;
        t9 = p.mstar + t3 * n;
        dt9_dvg = 0.0;
        dt9_dvd = dn_dvd * t3;
        dt9_dvb = dn_dvb * t3;
    } else {
        let exp_vgst = t2.exp();
        let t3 = mt.coxe / p.cdep0;
        let t4 = t3 * exp_vgst;
        let t5 = t1 * t4 / t0;
        t9 = p.mstar + n * t4;
        let dt9g = t3 * (p.mstar - 1.0) * exp_vgst / vtm;
        dt9_dvb = t4 * dn_dvb - dt9g * dvth_dvb - t5 * dn_dvb;
        dt9_dvd = t4 * dn_dvd - dt9g * dvth_dvd - t5 * dn_dvd;
        dt9_dvg = dt9g * dvgs_eff_dvg;
    }
    let vgsteff = t10 / t9;
    op.vgsteff = vgsteff;
    let t11 = t9 * t9;
    let dvgsteff_dvg = (t9 * dt10_dvg - t10 * dt9_dvg) / t11;
    let dvgsteff_dvd = (t9 * dt10_dvd - t10 * dt9_dvd) / t11;
    let dvgsteff_dvb = (t9 * dt10_dvb - t10 * dt9_dvb) / t11;

    // --- Calculate effective channel geometry (1332-1370) ---
    let t9 = sqrt_phis - p.sqrt_phi;
    let mut weff = p.weff - 2.0 * (p.dwg * vgsteff + p.dwb * t9);
    let mut dweff_dvg = -2.0 * p.dwg;
    let mut dweff_dvb = -2.0 * p.dwb * dsqrt_phis_dvb;
    if weff < 2.0e-8 {
        // to avoid the discontinuity problem due to Weff
        let t0 = 1.0 / (6.0e-8 - 2.0 * weff);
        weff = 2.0e-8 * (4.0e-8 - weff) * t0;
        let t0 = t0 * t0 * 4.0e-16;
        dweff_dvg *= t0;
        dweff_dvb *= t0;
    }

    // rdsMod = 0 uses the internal Rds(V) path. rdsMod = 1 moves the
    // bias-dependent resistance to external D/S branches stamped by the
    // engine-facing wrapper.
    let (rds, drds_dvg, drds_dvb);
    if model.rds_mod == 1 {
        rds = 0.0;
        drds_dvg = 0.0;
        drds_dvb = 0.0;
        op.grdsw = 0.0;
        op.rds = 0.0;
    } else {
        let t0 = 1.0 + p.prwg * vgsteff;
        let dt0_dvg = -p.prwg / t0 / t0;
        let t1 = p.prwb * t9;
        let dt1_dvb = p.prwb * dsqrt_phis_dvb;

        let t2 = 1.0 / t0 + t1;
        let t3 = t2 + (t2 * t2 + 0.01).sqrt(); // 0.01 = 4.0 * 0.05 * 0.05
        let dt3_dvg = 1.0 + t2 / (t3 - t2);
        let dt3_dvb = dt3_dvg * dt1_dvb;
        let dt3_dvg = dt3_dvg * dt0_dvg;

        let t4 = p.rds0 * 0.5;
        rds = p.rdswmin + t3 * t4;
        drds_dvg = t4 * dt3_dvg;
        drds_dvb = t4 * dt3_dvb;

        op.grdsw = if rds > 0.0 { 1.0 / rds * nf } else { 0.0 };
        op.rds = rds;
    }

    // --- Calculate Abulk (1372-1429) ---
    let t9 = 0.5 * p.k1ox * lpe_vb / sqrt_phis;
    let t1 = t9 + inst.k2ox - p.k3b * vth_narrow_w;
    let dt1_dvb = -t9 / sqrt_phis * dsqrt_phis_dvb;

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
        // added to avoid the problems caused by Abulk0
        let t9 = 1.0 / (3.0 - 20.0 * abulk0);
        abulk0 = (0.2 - abulk0) * t9;
        dabulk0_dvb *= t9 * t9;
    }
    if abulk < 0.1 {
        let t9 = 1.0 / (3.0 - 20.0 * abulk);
        abulk = (0.2 - abulk) * t9;
        let t10 = t9 * t9;
        dabulk_dvb *= t10;
        dabulk_dvg *= t10;
    }

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
    let mut dabulk0_q_dvb = dabulk0_dvb; // copy before scaling
    dabulk0_dvb = dabulk0_dvb * t0 + abulk0 * dt0_dvb;
    abulk *= t0;
    let mut abulk0_q = abulk0; // copy before scaling
    abulk0 *= t0;
    op.abulk = abulk;

    // Calculate Abulk0_Q (the separate ketac scaling for the C-V model).
    if p.ketac != p.keta {
        let t2 = p.ketac * vbseff;
        let (t0, dt0_dvb);
        if t2 >= -0.9 {
            t0 = 1.0 / (1.0 + t2);
            dt0_dvb = -p.ketac * t0 * t0;
        } else {
            let t1 = 1.0 / (0.8 + t2);
            t0 = (17.0 + 20.0 * t2) * t1;
            dt0_dvb = -p.ketac * t1 * t1;
        }
        dabulk0_q_dvb = dabulk0_q_dvb * t0 + abulk0_q * dt0_dvb;
        abulk0_q *= t0;
    } else {
        dabulk0_q_dvb = dabulk0_dvb;
        abulk0_q = abulk0;
    }

    // --- Mobility calculation (1450-1612) ---
    let t14: Value = if model.mtrl_mod != 0 && model.mtrl_compat_mod == 0 {
        2.0 * model.mtype * (model.phig - model.easub - 0.5 * mt.eg0 + 0.45)
    } else {
        0.0
    };
    let (t5m, mut ddenomi_dvg, mut ddenomi_dvd, mut ddenomi_dvb);
    match model.mob_mod {
        0 => {
            let t0 = vgsteff + vth + vth - t14;
            let t2 = p.ua + p.uc * vbseff;
            let t3 = t0 / toxe;
            let t12 = (vth * vth + 0.0001).sqrt();
            let t9 = 1.0 / (vgsteff + 2.0 * t12);
            let t10 = t9 * toxe;
            let t8 = p.ud * t10 * t10 * vth;
            let t6 = t8 * vth;
            t5m = t3 * (t2 + p.ub * t3) + t6;
            let t7 = -2.0 * t6 * t9;
            let t11 = t7 * vth / t12;
            ddenomi_dvg = (t2 + 2.0 * p.ub * t3) / toxe;
            let t13 = 2.0 * (ddenomi_dvg + t11 + t8);
            ddenomi_dvd = t13 * dvth_dvd;
            ddenomi_dvb = t13 * dvth_dvb + p.uc * t3;
            ddenomi_dvg += t7;
        }
        1 => {
            let t0 = vgsteff + vth + vth - t14;
            let t2 = 1.0 + p.uc * vbseff;
            let t3 = t0 / toxe;
            let t4 = t3 * (p.ua + p.ub * t3);
            let t12 = (vth * vth + 0.0001).sqrt();
            let t9 = 1.0 / (vgsteff + 2.0 * t12);
            let t10 = t9 * toxe;
            let t8 = p.ud * t10 * t10 * vth;
            let t6 = t8 * vth;
            t5m = t4 * t2 + t6;
            let t7 = -2.0 * t6 * t9;
            let t11 = t7 * vth / t12;
            ddenomi_dvg = (p.ua + 2.0 * p.ub * t3) * t2 / toxe;
            let t13 = 2.0 * (ddenomi_dvg + t11 + t8);
            ddenomi_dvd = t13 * dvth_dvd;
            ddenomi_dvb = t13 * dvth_dvb + p.uc * t4;
            ddenomi_dvg += t7;
        }
        2 => {
            let t0 = (vgsteff + inst.vtfbphi1) / toxe;
            let t1 = (p.eu * t0.ln()).exp();
            let dt1_dvg = t1 * p.eu / t0 / toxe;
            let t2 = p.ua + p.uc * vbseff;
            let t12 = (vth * vth + 0.0001).sqrt();
            let t9 = 1.0 / (vgsteff + 2.0 * t12);
            let t10 = t9 * toxe;
            let t8 = p.ud * t10 * t10 * vth;
            let t6 = t8 * vth;
            t5m = t1 * t2 + t6;
            let t7 = -2.0 * t6 * t9;
            let t11 = t7 * vth / t12;
            ddenomi_dvg = t2 * dt1_dvg + t7;
            let t13 = 2.0 * (t11 + t8);
            ddenomi_dvd = t13 * dvth_dvd;
            ddenomi_dvb = t13 * dvth_dvb + t1 * p.uc;
        }
        3 => {
            let t0 = (vgsteff + inst.vtfbphi1) * 1.0e-8 / toxe / 6.0;
            let t1 = (p.eu * t0.ln()).exp();
            let dt1_dvg = t1 * p.eu * 1.0e-8 / t0 / toxe / 6.0;
            let t2 = p.ua + p.uc * vbseff;
            let t10_arg = 0.5 + 0.5 * vgsteff / p.vgsteff_vth;
            let t10 = (p.ucs * t10_arg.ln()).exp();
            let t11 = p.ud / t10;
            let dt11_dvg = -0.5 * p.ucs * t11 / t10_arg / p.vgsteff_vth;
            t5m = t1 * t2 + t11;
            ddenomi_dvg = t2 * dt1_dvg + dt11_dvg;
            ddenomi_dvd = 0.0;
            ddenomi_dvb = t1 * p.uc;
        }
        4 => {
            let t0 = vgsteff + inst.vtfbphi1 - t14;
            let t2 = p.ua + p.uc * vbseff;
            let t3 = t0 / toxe;
            let t12 = (inst.vtfbphi1 * inst.vtfbphi1 + 0.0001).sqrt();
            let t9 = 1.0 / (vgsteff + 2.0 * t12);
            let t10 = t9 * toxe;
            let t8 = p.ud * t10 * t10 * inst.vtfbphi1;
            let t6 = t8 * inst.vtfbphi1;
            t5m = t3 * (t2 + p.ub * t3) + t6;
            let t7 = -2.0 * t6 * t9;
            ddenomi_dvg = (t2 + 2.0 * p.ub * t3) / toxe + t7;
            ddenomi_dvd = 0.0;
            ddenomi_dvb = p.uc * t3;
        }
        5 => {
            let t0 = vgsteff + inst.vtfbphi1 - t14;
            let t2 = 1.0 + p.uc * vbseff;
            let t3 = t0 / toxe;
            let t4 = t3 * (p.ua + p.ub * t3);
            let t12 = (inst.vtfbphi1 * inst.vtfbphi1 + 0.0001).sqrt();
            let t9 = 1.0 / (vgsteff + 2.0 * t12);
            let t10 = t9 * toxe;
            let t8 = p.ud * t10 * t10 * inst.vtfbphi1;
            let t6 = t8 * inst.vtfbphi1;
            t5m = t4 * t2 + t6;
            let t7 = -2.0 * t6 * t9;
            ddenomi_dvg = (p.ua + 2.0 * p.ub * t3) * t2 / toxe + t7;
            ddenomi_dvd = 0.0;
            ddenomi_dvb = p.uc * t4;
        }
        6 => {
            let t0 = (vgsteff + inst.vtfbphi1) / toxe;
            let t1 = (p.eu * t0.ln()).exp();
            let dt1_dvg = t1 * p.eu / t0 / toxe;
            let t2 = p.ua + p.uc * vbseff;
            let t12 = (inst.vtfbphi1 * inst.vtfbphi1 + 0.0001).sqrt();
            let t9 = 1.0 / (vgsteff + 2.0 * t12);
            let t10 = t9 * toxe;
            let t8 = p.ud * t10 * t10 * inst.vtfbphi1;
            let t6 = t8 * inst.vtfbphi1;
            t5m = t1 * t2 + t6;
            let t7 = -2.0 * t6 * t9;
            ddenomi_dvg = t2 * dt1_dvg + t7;
            ddenomi_dvd = 0.0;
            ddenomi_dvb = t1 * p.uc;
        }
        other => {
            unreachable!("BSIM4: MOBMOD={other} reaches eval");
        }
    }

    let denomi;
    if t5m >= -0.8 {
        denomi = 1.0 + t5m;
    } else {
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

    // --- Saturation drain voltage Vdsat (1614-1714) ---
    let wv_cox = weff * inst.vsattemp * mt.coxe;
    let wv_cox_rds = wv_cox * rds;

    let mut esat = 2.0 * inst.vsattemp / ueff;
    let mut esat_l = esat * leff;
    let t0 = -esat_l / ueff;
    let mut desat_l_dvg = t0 * dueff_dvg;
    let mut desat_l_dvd = t0 * dueff_dvd;
    let mut desat_l_dvb = t0 * dueff_dvb;

    // Sqrt().
    let a1 = p.a1;
    let (lambda, dlambda_dvg);
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

    let vgst2vtm = vgsteff + 2.0 * vtm;
    let (tmp2, tmp3);
    if rds > 0.0 {
        tmp2 = drds_dvg / rds + dweff_dvg / weff;
        tmp3 = drds_dvb / rds + dweff_dvb / weff;
    } else {
        tmp2 = dweff_dvg / weff;
        tmp3 = dweff_dvb / weff;
    }
    let tmp1;
    let (vdsat, dvdsat_dvg, dvdsat_dvd, dvdsat_dvb);
    if rds == 0.0 && lambda == 1.0 {
        let t0 = 1.0 / (abulk * esat_l + vgst2vtm);
        tmp1 = 0.0;
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
        tmp1 = dlambda_dvg / (lambda * lambda);
        let t9 = abulk * wv_cox_rds;
        let t8 = abulk * t9;
        let t7 = vgst2vtm * t9;
        let t6 = vgst2vtm * wv_cox_rds;
        let t0 = 2.0 * abulk * (t9 - 1.0 + 1.0 / lambda);
        let dt0_dvg =
            2.0 * (t8 * tmp2 - abulk * tmp1 + (2.0 * t9 + 1.0 / lambda - 1.0) * dabulk_dvg);
        let dt0_dvb =
            2.0 * (t8 * (2.0 / abulk * dabulk_dvb + tmp3) + (1.0 / lambda - 1.0) * dabulk_dvb);
        // dT0_dVd = 0 in the C; the dVdsat_dVd expression below already
        // omits its (zero) contribution.
        let t1 = vgst2vtm * (2.0 / lambda - 1.0) + abulk * esat_l + 3.0 * t7;

        let dt1_dvg = (2.0 / lambda - 1.0) - 2.0 * vgst2vtm * tmp1
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

        dvdsat_dvg =
            (dt1_dvg - (t1 * dt1_dvg - dt0_dvg * t2 - t0 * dt2_dvg) / t3 - vdsat * dt0_dvg) / t0;
        dvdsat_dvb =
            (dt1_dvb - (t1 * dt1_dvb - dt0_dvb * t2 - t0 * dt2_dvb) / t3 - vdsat * dt0_dvb) / t0;
        dvdsat_dvd = (dt1_dvd - (t1 * dt1_dvd - t0 * dt2_dvd) / t3) / t0;
    }
    op.vdsat = vdsat;

    // --- Calculate Vdseff (1716-1797) ---
    let t1 = vdsat - vds_c - p.delta;
    let dt1_dvg = dvdsat_dvg;
    let dt1_dvd = dvdsat_dvd - 1.0;
    let dt1_dvb = dvdsat_dvb;

    let t2 = (t1 * t1 + 4.0 * p.delta * vdsat).sqrt();
    let t0 = t1 / t2;
    let t9 = 2.0 * p.delta;
    let t3 = t9 / t2;
    let dt2_dvg = t0 * dt1_dvg + t3 * dvdsat_dvg;
    let dt2_dvd = t0 * dt1_dvd + t3 * dvdsat_dvd;
    let dt2_dvb = t0 * dt1_dvb + t3 * dvdsat_dvb;

    let (mut vdseff, mut dvdseff_dvg, dvdseff_dvd, mut dvdseff_dvb);
    if t1 >= 0.0 {
        vdseff = vdsat - 0.5 * (t1 + t2);
        dvdseff_dvg = dvdsat_dvg - 0.5 * (dt1_dvg + dt2_dvg);
        dvdseff_dvd = dvdsat_dvd - 0.5 * (dt1_dvd + dt2_dvd);
        dvdseff_dvb = dvdsat_dvb - 0.5 * (dt1_dvb + dt2_dvb);
    } else {
        let t4 = t9 / (t2 - t1);
        let t5 = 1.0 - t4;
        let t6 = vdsat * t4 / (t2 - t1);
        vdseff = vdsat * t5;
        dvdseff_dvg = dvdsat_dvg * t5 + t6 * (dt2_dvg - dt1_dvg);
        dvdseff_dvd = dvdsat_dvd * t5 + t6 * (dt2_dvd - dt1_dvd);
        dvdseff_dvb = dvdsat_dvb * t5 + t6 * (dt2_dvb - dt1_dvb);
    }

    if vds_c == 0.0 {
        vdseff = 0.0;
        dvdseff_dvg = 0.0;
        dvdseff_dvb = 0.0;
    }
    if vdseff > vds_c {
        vdseff = vds_c;
    }
    let diff_vds = vds_c - vdseff;
    op.vdseff = vdseff;

    // --- Velocity overshoot (1800-1840) ---
    if model.lambda_given && p.lambda > 0.0 {
        let t1 = leff * ueff;
        let t2 = p.lambda / t1;
        let t3 = -t2 / t1 * leff;
        let dt2_dvd = t3 * dueff_dvd;
        let dt2_dvg = t3 * dueff_dvg;
        let dt2_dvb = t3 * dueff_dvb;
        let t5 = 1.0 / (esat * p.litl);
        let t4 = -t5 / esat_l;
        let dt5_dvg = desat_l_dvg * t4;
        let dt5_dvd = desat_l_dvd * t4;
        let dt5_dvb = desat_l_dvb * t4;
        let t6 = 1.0 + diff_vds * t5;
        let dt6_dvg = dt5_dvg * diff_vds - dvdseff_dvg * t5;
        let dt6_dvd = dt5_dvd * diff_vds + (1.0 - dvdseff_dvd) * t5;
        let dt6_dvb = dt5_dvb * diff_vds - dvdseff_dvb * t5;
        let t7 = 2.0 / (t6 * t6 + 1.0);
        let t8 = 1.0 - t7;
        let t9 = t6 * t7 * t7;
        let dt8_dvg = t9 * dt6_dvg;
        let dt8_dvd = t9 * dt6_dvd;
        let dt8_dvb = t9 * dt6_dvb;
        let t10 = 1.0 + t2 * t8;
        let (mut dt10_dvg, mut dt10_dvd, mut dt10_dvb) = (
            dt2_dvg * t8 + t2 * dt8_dvg,
            dt2_dvd * t8 + t2 * dt8_dvd,
            dt2_dvb * t8 + t2 * dt8_dvb,
        );
        if t10 == 1.0 {
            dt10_dvg = 0.0;
            dt10_dvd = 0.0;
            dt10_dvb = 0.0;
        }
        desat_l_dvg *= t10;
        desat_l_dvg += esat_l * dt10_dvg;
        desat_l_dvd *= t10;
        desat_l_dvd += esat_l * dt10_dvd;
        desat_l_dvb *= t10;
        desat_l_dvb += esat_l * dt10_dvb;
        esat_l *= t10;
        esat = esat_l / leff; // bugfix by Wenwei Yang (4.6.4)
    }
    op.esat_l = esat_l;

    // --- Calculate Vasat (1842-1882) ---
    let tmp4v = 1.0 - 0.5 * abulk * vdsat / vgst2vtm;
    let t9 = wv_cox_rds * vgsteff;
    let t8 = t9 / vgst2vtm;
    let t0 = esat_l + vdsat + 2.0 * t9 * tmp4v;

    let t7 = 2.0 * wv_cox_rds * tmp4v;
    let dt0_dvg = desat_l_dvg + dvdsat_dvg + t7 * (1.0 + tmp2 * vgsteff)
        - t8 * (abulk * dvdsat_dvg - abulk * vdsat / vgst2vtm + vdsat * dabulk_dvg);
    let dt0_dvb = desat_l_dvb + dvdsat_dvb + t7 * tmp3 * vgsteff
        - t8 * (dabulk_dvb * vdsat + abulk * dvdsat_dvb);
    let dt0_dvd = desat_l_dvd + dvdsat_dvd - t8 * abulk * dvdsat_dvd;

    let t9 = wv_cox_rds * abulk;
    let t1 = 2.0 / lambda - 1.0 + t9;
    let dt1_dvg = -2.0 * tmp1 + wv_cox_rds * (abulk * tmp2 + dabulk_dvg);
    let dt1_dvb = dabulk_dvb * wv_cox_rds + t9 * tmp3;

    let vasat = t0 / t1;
    let dvasat_dvg = (dt0_dvg - vasat * dt1_dvg) / t1;
    let dvasat_dvb = (dt0_dvb - vasat * dt1_dvb) / t1;
    let dvasat_dvd = dt0_dvd / t1;

    // --- Calculate Idl first (1824-1883) ---
    let tmp1i = inst.vtfbphi2;
    let tmp2i = 2.0e8 * inst.toxp;
    let dt0_dvg_i = 1.0 / tmp2i;
    let t0 = (vgsteff + tmp1i) * dt0_dvg_i;

    let tmp3i = (model.bdos * 0.7 * t0.ln()).exp();
    let t1 = 1.0 + tmp3i;
    let t2 = model.bdos * 0.7 * tmp3i / t0;
    let tcen = model.ados * 1.9e-9 / t1;
    let dtcen_dvg = -tcen * t2 * dt0_dvg_i / t1;

    let coxeff = epssub * inst.coxp / (epssub + inst.coxp * tcen);
    op.coxeff = coxeff;
    let dcoxeff_dvg = -coxeff * coxeff * dtcen_dvg / epssub;

    let coxeff_wovl = coxeff * weff / leff;
    let beta = ueff * coxeff_wovl;
    let t3 = ueff / leff;
    let dbeta_dvg = coxeff_wovl * dueff_dvg + t3 * (weff * dcoxeff_dvg + coxeff * dweff_dvg);
    let dbeta_dvd = coxeff_wovl * dueff_dvd;
    let dbeta_dvb = coxeff_wovl * dueff_dvb + t3 * coxeff * dweff_dvb;

    op.ab_ov_vgst2vtm = abulk / vgst2vtm;
    let t0 = 1.0 - 0.5 * vdseff * op.ab_ov_vgst2vtm;
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
    let idl = gche / t0;
    let t1 = (1.0 - idl * rds) / t0;
    let t2 = idl * idl;
    let didl_dvg = t1 * dgche_dvg - t2 * drds_dvg;
    let didl_dvd = t1 * dgche_dvd;
    let didl_dvb = t1 * dgche_dvb - t2 * drds_dvb;

    // --- Degradation factor due to pocket implant (1885-1895) ---
    let (fp, dfp_dvg);
    if p.fprout <= 0.0 {
        fp = 1.0;
        dfp_dvg = 0.0;
    } else {
        let t9 = p.fprout * leff.sqrt() / vgst2vtm;
        fp = 1.0 / (1.0 + t9);
        dfp_dvg = fp * fp * t9 / vgst2vtm;
    }

    // --- Calculate VACLM (1897-1945) ---
    let t8 = p.pvag / esat_l;
    let t9 = t8 * vgsteff;
    let (pvag_term, dpvag_term_dvg, dpvag_term_dvb, dpvag_term_dvd);
    if t9 > -0.9 {
        pvag_term = 1.0 + t9;
        dpvag_term_dvg = t8 * (1.0 - vgsteff * desat_l_dvg / esat_l);
        dpvag_term_dvb = -t9 * desat_l_dvb / esat_l;
        dpvag_term_dvd = -t9 * desat_l_dvd / esat_l;
    } else {
        let t4 = 1.0 / (17.0 + 20.0 * t9);
        pvag_term = (0.8 + t9) * t4;
        let t4 = t4 * t4;
        dpvag_term_dvg = t8 * (1.0 - vgsteff * desat_l_dvg / esat_l) * t4;
        let t9 = t9 * t4 / esat_l;
        dpvag_term_dvb = -t9 * desat_l_dvb;
        dpvag_term_dvd = -t9 * desat_l_dvd;
    }

    let (vaclm, dvaclm_dvg, dvaclm_dvb, dvaclm_dvd);
    let (cclm, dcclm_dvg, dcclm_dvb, dcclm_dvd);
    if p.pclm > MIN_EXP && diff_vds > 1.0e-10 {
        let t0 = 1.0 + rds * idl;
        let dt0_dvg = drds_dvg * idl + rds * didl_dvg;
        let dt0_dvd = rds * didl_dvd;
        let dt0_dvb = drds_dvb * idl + rds * didl_dvb;

        let t2 = vdsat / esat;
        let t1 = leff + t2;
        let dt1_dvg = (dvdsat_dvg - t2 * desat_l_dvg / leff) / esat;
        let dt1_dvd = (dvdsat_dvd - t2 * desat_l_dvd / leff) / esat;
        let dt1_dvb = (dvdsat_dvb - t2 * desat_l_dvb / leff) / esat;

        cclm = fp * pvag_term * t0 * t1 / (p.pclm * p.litl);
        dcclm_dvg =
            cclm * (dfp_dvg / fp + dpvag_term_dvg / pvag_term + dt0_dvg / t0 + dt1_dvg / t1);
        dcclm_dvb = cclm * (dpvag_term_dvb / pvag_term + dt0_dvb / t0 + dt1_dvb / t1);
        dcclm_dvd = cclm * (dpvag_term_dvd / pvag_term + dt0_dvd / t0 + dt1_dvd / t1);
        vaclm = cclm * diff_vds;

        dvaclm_dvg = dcclm_dvg * diff_vds - dvdseff_dvg * cclm;
        dvaclm_dvb = dcclm_dvb * diff_vds - dvdseff_dvb * cclm;
        dvaclm_dvd = dcclm_dvd * diff_vds + (1.0 - dvdseff_dvd) * cclm;
    } else {
        vaclm = MAX_EXP;
        cclm = MAX_EXP;
        dvaclm_dvd = 0.0;
        dvaclm_dvg = 0.0;
        dvaclm_dvb = 0.0;
        dcclm_dvd = 0.0;
        dcclm_dvg = 0.0;
        dcclm_dvb = 0.0;
    }

    // --- Calculate VADIBL (1947-1995) ---
    let (mut vadibl, mut dvadibl_dvg, mut dvadibl_dvb, mut dvadibl_dvd);
    if p.theta_rout > MIN_EXP {
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
            let t4 = 1.0 / (0.8 + t7);
            let t3 = (17.0 + 20.0 * t7) * t4;
            dvadibl_dvg *= t3;
            dvadibl_dvb = dvadibl_dvb * t3 - vadibl * p.pdiblb * t4 * t4;
            dvadibl_dvd *= t3;
            vadibl *= t3;
        }

        dvadibl_dvg = dvadibl_dvg * pvag_term + vadibl * dpvag_term_dvg;
        dvadibl_dvb = dvadibl_dvb * pvag_term + vadibl * dpvag_term_dvb;
        dvadibl_dvd = dvadibl_dvd * pvag_term + vadibl * dpvag_term_dvd;
        vadibl *= pvag_term;
    } else {
        vadibl = MAX_EXP;
        dvadibl_dvd = 0.0;
        dvadibl_dvg = 0.0;
        dvadibl_dvb = 0.0;
    }

    // --- Calculate Va (1997-2001) ---
    let va = vasat + vaclm;
    let dva_dvg = dvasat_dvg + dvaclm_dvg;
    let dva_dvb = dvasat_dvb + dvaclm_dvb;
    let dva_dvd = dvasat_dvd + dvaclm_dvd;

    // --- Calculate VADITS (2003-2024) ---
    let t0 = p.pditsd * vds_c;
    let (t1, dt1_dvd);
    if t0 > EXP_THRESHOLD {
        t1 = MAX_EXP;
        dt1_dvd = 0.0;
    } else {
        t1 = t0.exp();
        dt1_dvd = t1 * p.pditsd;
    }
    let (vadits, dvadits_dvg, dvadits_dvd);
    if p.pdits > MIN_EXP {
        let t2 = 1.0 + model.pditsl * leff;
        let v = (1.0 + t2 * t1) / p.pdits;
        // The C forms dVADITS_dVg from the pre-FP VADITS, then scales
        // VADITS by FP afterwards.
        dvadits_dvg = v * dfp_dvg;
        dvadits_dvd = fp * t2 * dt1_dvd / p.pdits;
        vadits = v * fp;
    } else {
        vadits = MAX_EXP;
        dvadits_dvg = 0.0;
        dvadits_dvd = 0.0;
    }

    // --- Calculate VASCBE (2026-2046) ---
    let (vascbe, dvascbe_dvg, dvascbe_dvd, dvascbe_dvb);
    if p.pscbe2 > 0.0 && p.pscbe1 >= 0.0 {
        // 4.6.2
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

    // --- Add DIBL to Ids (2048-2056) ---
    let t9 = diff_vds / vadibl;
    let t0 = 1.0 + t9;
    let mut idsa = idl * t0;
    let mut didsa_dvg = t0 * didl_dvg - idl * (dvdseff_dvg + t9 * dvadibl_dvg) / vadibl;
    let mut didsa_dvd = t0 * didl_dvd + idl * (1.0 - dvdseff_dvd - t9 * dvadibl_dvd) / vadibl;
    let mut didsa_dvb = t0 * didl_dvb - idl * (dvdseff_dvb + t9 * dvadibl_dvb) / vadibl;

    // --- Add DITS to Ids (2058-2065) ---
    let t9 = diff_vds / vadits;
    let t0 = 1.0 + t9;
    didsa_dvg = t0 * didsa_dvg - idsa * (dvdseff_dvg + t9 * dvadits_dvg) / vadits;
    didsa_dvd = t0 * didsa_dvd + idsa * (1.0 - dvdseff_dvd - t9 * dvadits_dvd) / vadits;
    didsa_dvb = t0 * didsa_dvb - idsa * dvdseff_dvb / vadits;
    idsa *= t0;

    // --- Add CLM to Ids (2067-2082) ---
    let t0 = (va / vasat).ln();
    let dt0_dvg = dva_dvg / va - dvasat_dvg / vasat;
    let dt0_dvb = dva_dvb / va - dvasat_dvb / vasat;
    let dt0_dvd = dva_dvd / va - dvasat_dvd / vasat;
    let t1 = t0 / cclm;
    let t9 = 1.0 + t1;
    let dt9_dvg = (dt0_dvg - t1 * dcclm_dvg) / cclm;
    let dt9_dvb = (dt0_dvb - t1 * dcclm_dvb) / cclm;
    let dt9_dvd = (dt0_dvd - t1 * dcclm_dvd) / cclm;

    didsa_dvg = didsa_dvg * t9 + idsa * dt9_dvg;
    didsa_dvb = didsa_dvb * t9 + idsa * dt9_dvb;
    didsa_dvd = didsa_dvd * t9 + idsa * dt9_dvd;
    idsa *= t9;

    // --- Substrate current (2084-2120) ---
    {
        let tmp = p.alpha0 + p.alpha1 * leff;
        let (isub, gbg, gbd, gbb);
        if tmp <= 0.0 || p.beta0 <= 0.0 {
            isub = 0.0;
            gbd = 0.0;
            gbb = 0.0;
            gbg = 0.0;
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
            let t4 = idsa * vdseff;
            isub = t1 * t4;
            let mut gbg_l = t1 * (didsa_dvg * vdseff + idsa * dvdseff_dvg) + t4 * dt1_dvg;
            let mut gbd_l = t1 * (didsa_dvd * vdseff + idsa * dvdseff_dvd) + t4 * dt1_dvd;
            let mut gbb_l = t1 * (didsa_dvb * vdseff + idsa * dvdseff_dvb) + t4 * dt1_dvb;

            gbd_l += gbg_l * dvgsteff_dvd;
            gbb_l += gbg_l * dvgsteff_dvb;
            gbg_l *= dvgsteff_dvg;
            gbb_l *= dvbseff_dvb;
            gbg = gbg_l;
            gbd = gbd_l;
            gbb = gbb_l;
        }
        op.csub = isub;
        op.gbbs = gbb;
        op.gbgs = gbg;
        op.gbds = gbd;
    }

    // --- Add SCBE to Ids (2122-2155) ---
    let t9 = diff_vds / vascbe;
    let t0 = 1.0 + t9;
    let ids = idsa * t0;

    let mut gm = t0 * didsa_dvg - idsa * (dvdseff_dvg + t9 * dvascbe_dvg) / vascbe;
    let mut gds = t0 * didsa_dvd + idsa * (1.0 - dvdseff_dvd - t9 * dvascbe_dvd) / vascbe;
    let mut gmb = t0 * didsa_dvb - idsa * (dvdseff_dvb + t9 * dvascbe_dvb) / vascbe;

    let tmp1 = gds + gm * dvgsteff_dvd;
    let tmp2 = gmb + gm * dvgsteff_dvb;
    let tmp3 = gm;

    gm = (ids * dvdseff_dvg + vdseff * tmp3) * dvgsteff_dvg;
    gds = ids * (dvdseff_dvd + dvdseff_dvg * dvgsteff_dvd) + vdseff * tmp1;
    gmb = (ids * (dvdseff_dvb + dvdseff_dvg * dvgsteff_dvb) + vdseff * tmp2) * dvbseff_dvb;

    let mut cdrain = ids * vdseff;

    // --- Source end velocity limit (2157-2196) ---
    if model.vtl_given && p.vtl > 0.0 {
        let t12 = 1.0 / leff / coxeff_wovl;
        let t11 = t12 / vgsteff;
        let t10 = -t11 / vgsteff;
        let vs = cdrain * t11; // vs
        let dvs_dvg = gm * t11 + cdrain * t10 * dvgsteff_dvg;
        let dvs_dvd = gds * t11 + cdrain * t10 * dvgsteff_dvd;
        let dvs_dvb = gmb * t11 + cdrain * t10 * dvgsteff_dvb;
        let t0 = 2.0 * MM;
        let t1 = vs / (p.vtl * p.tfactor);
        let (fsevl, dfsevl_dvg, dfsevl_dvd, dfsevl_dvb);
        if t1 > 0.0 {
            let t2 = 1.0 + (t0 * t1.ln()).exp();
            let t3 = (t2 - 1.0) * t0 / vs;
            fsevl = 1.0 / (t2.ln() / t0).exp();
            let dt2_dvg = t3 * dvs_dvg;
            let dt2_dvd = t3 * dvs_dvd;
            let dt2_dvb = t3 * dvs_dvb;
            let t4 = -1.0 / t0 * fsevl / t2;
            dfsevl_dvg = t4 * dt2_dvg;
            dfsevl_dvd = t4 * dt2_dvd;
            dfsevl_dvb = t4 * dt2_dvb;
        } else {
            fsevl = 1.0;
            dfsevl_dvg = 0.0;
            dfsevl_dvd = 0.0;
            dfsevl_dvb = 0.0;
        }
        gm *= fsevl;
        gm += cdrain * dfsevl_dvg;
        gmb *= fsevl;
        gmb += cdrain * dfsevl_dvb;
        gds *= fsevl;
        gds += cdrain * dfsevl_dvd;
        cdrain *= fsevl;
    }

    op.gds = gds;
    op.gm = gm;
    op.gmbs = gmb;
    op.idovvds = if ids <= model.idovvdsc {
        model.idovvdsc
    } else {
        ids
    };

    // Bias-dependent gate resistance (b4ld.c:2192-2225). This feeds native
    // `RGATEMOD=2` and the AC-NQS time constant.
    if model.rgate_mod > 1 || model.trnqs_mod != 0 || model.acnqs_mod != 0 {
        let t9 = p.xrcrg2 * mt.vtm;
        let t0 = t9 * beta;
        let dt0_dvd = (dbeta_dvd + dbeta_dvg * dvgsteff_dvd) * t9;
        let dt0_dvb = (dbeta_dvb + dbeta_dvg * dvgsteff_dvb) * t9;
        let dt0_dvg = dbeta_dvg * t9;

        op.gcrg = p.xrcrg1 * (t0 + ids);
        op.gcrgd = p.xrcrg1 * (dt0_dvd + tmp1);
        op.gcrgb = p.xrcrg1 * (dt0_dvb + tmp2) * dvbseff_dvb;
        op.gcrgg = p.xrcrg1 * (dt0_dvg + tmp3) * dvgsteff_dvg;

        if nf != 1.0 {
            op.gcrg *= nf;
            op.gcrgg *= nf;
            op.gcrgd *= nf;
            op.gcrgb *= nf;
        }

        if model.rgate_mod == 2 {
            let denom = inst.gate_conductance + op.gcrg;
            if denom != 0.0 {
                let scale = inst.gate_conductance * inst.gate_conductance / (denom * denom);
                op.gcrg = inst.gate_conductance * op.gcrg / denom;
                op.gcrgg *= scale;
                op.gcrgd *= scale;
                op.gcrgb *= scale;
            } else {
                op.gcrg = 0.0;
                op.gcrgg = 0.0;
                op.gcrgd = 0.0;
                op.gcrgb = 0.0;
            }
        }

        op.gcrgs = -(op.gcrgg + op.gcrgd + op.gcrgb);
    }

    // rdsMod=1 external S/D resistance is stamped by Bsim4v8Device.

    // --- GIDL/GISL models (2308-2525) ---
    let t0gidl = if model.mtrl_mod == 0 {
        3.0 * toxe
    } else {
        model.epsrsub * toxe / epsrox
    };
    let vfbsd_gidl = if model.mtrl_mod == 0 { 0.0 } else { p.vfbsd };
    let (mut igidl, mut ggidld, mut ggidlg, ggidlb);
    let (mut igisl, mut ggisls, mut ggislg, ggislb);
    if model.gidl_mod == 0 {
        // GIDL.
        let t1 = (vds - vgs_eff_raw - p.egidl + vfbsd_gidl) / t0gidl;
        if p.agidl <= 0.0 || p.bgidl <= 0.0 || t1 <= 0.0 || p.cgidl <= 0.0 || vbd > 0.0 {
            igidl = 0.0;
            ggidld = 0.0;
            ggidlg = 0.0;
            ggidlb = 0.0;
        } else {
            let dt1_dvd = 1.0 / t0gidl;
            let dt1_dvg = -dvgs_eff_dvg_raw * dt1_dvd;
            let t2 = p.bgidl / t1;
            if t2 < 100.0 {
                igidl = p.agidl * p.weff_cj * t1 * (-t2).exp();
                let t3 = igidl * (1.0 + t2) / t1;
                ggidld = t3 * dt1_dvd;
                ggidlg = t3 * dt1_dvg;
            } else {
                igidl = p.agidl * p.weff_cj * 3.720075976e-44;
                ggidld = igidl * dt1_dvd;
                ggidlg = igidl * dt1_dvg;
                igidl *= t1;
            }
            let t4 = vbd * vbd;
            let t5 = -vbd * t4;
            let t6 = p.cgidl + t5;
            let t7 = t5 / t6;
            let t8 = 3.0 * p.cgidl * t4 / t6 / t6;
            ggidld = ggidld * t7 + igidl * t8;
            ggidlg *= t7;
            ggidlb = -igidl * t8;
            igidl *= t7;
        }
        // GISL.
        let t1 = (-vds - vgd_eff_raw - p.egisl + vfbsd_gidl) / t0gidl;
        if p.agisl <= 0.0 || p.bgisl <= 0.0 || t1 <= 0.0 || p.cgisl <= 0.0 || vbs > 0.0 {
            igisl = 0.0;
            ggisls = 0.0;
            ggislg = 0.0;
            ggislb = 0.0;
        } else {
            let dt1_dvd = 1.0 / t0gidl;
            let dt1_dvg = -dvgd_eff_dvg_raw * dt1_dvd;
            let t2 = p.bgisl / t1;
            if t2 < 100.0 {
                igisl = p.agisl * p.weff_cj * t1 * (-t2).exp();
                let t3 = igisl * (1.0 + t2) / t1;
                ggisls = t3 * dt1_dvd;
                ggislg = t3 * dt1_dvg;
            } else {
                igisl = p.agisl * p.weff_cj * 3.720075976e-44;
                ggisls = igisl * dt1_dvd;
                ggislg = igisl * dt1_dvg;
                igisl *= t1;
            }
            let t4 = vbs * vbs;
            let t5 = -vbs * t4;
            let t6 = p.cgisl + t5;
            let t7 = t5 / t6;
            let t8 = 3.0 * p.cgisl * t4 / t6 / t6;
            ggisls = ggisls * t7 + igisl * t8;
            ggislg *= t7;
            ggislb = -igisl * t8;
            igisl *= t7;
        }
    } else {
        // v4.7 new GIDL/GISL model (gidlMod = 1).
        // GISL.
        let t1 = (-vds - p.rgisl * vgd_eff_raw - p.egisl + vfbsd_gidl) / t0gidl;
        if p.agisl <= 0.0 || p.bgisl <= 0.0 || t1 <= 0.0 || p.cgisl < 0.0 {
            igisl = 0.0;
            ggisls = 0.0;
            ggislg = 0.0;
            ggislb = 0.0;
        } else {
            let dt1_dvd = 1.0 / t0gidl;
            let dt1_dvg = -p.rgisl * dt1_dvd * dvgd_eff_dvg_raw;
            let t2 = p.bgisl / t1;
            if t2 < EXPL_THRESHOLD {
                igisl = p.weff_cj * p.agisl * t1 * (-t2).exp();
                let t3 = igisl / t1 * (t2 + 1.0);
                ggisls = t3 * dt1_dvd;
                ggislg = t3 * dt1_dvg;
            } else {
                let t3 = p.weff_cj * p.agisl * MIN_EXPL;
                igisl = t3 * t1;
                ggisls = t3 * dt1_dvd;
                ggislg = t3 * dt1_dvg;
            }
            let mut t4 = vbs - p.fgisl;
            // (chetan dabhi) solution for clamping T4.
            if t4 > model.gidlclamp {
                t4 = model.gidlclamp;
            }
            let t5 = if t4 == 0.0 {
                EXPL_THRESHOLD
            } else {
                p.kgisl / t4
            };
            let t6;
            if t5 < EXPL_THRESHOLD {
                t6 = t5.exp();
                ggislb = -igisl * t6 * t5 / t4;
            } else {
                t6 = MAX_EXPL;
                ggislb = 0.0;
            }
            ggisls *= t6;
            ggislg *= t6;
            igisl *= t6;
        }
        // GIDL.
        let t1 = (vds - p.rgidl * vgs_eff_raw - p.egidl + vfbsd_gidl) / t0gidl;
        if p.agidl <= 0.0 || p.bgidl <= 0.0 || t1 <= 0.0 || p.cgidl < 0.0 {
            igidl = 0.0;
            ggidld = 0.0;
            ggidlg = 0.0;
            ggidlb = 0.0;
        } else {
            let dt1_dvd = 1.0 / t0gidl;
            let dt1_dvg = -p.rgidl * dt1_dvd * dvgs_eff_dvg_raw;
            let t2 = p.bgidl / t1;
            if t2 < EXPL_THRESHOLD {
                igidl = p.weff_cj * p.agidl * t1 * (-t2).exp();
                let t3 = igidl / t1 * (t2 + 1.0);
                ggidld = t3 * dt1_dvd;
                ggidlg = t3 * dt1_dvg;
            } else {
                let t3 = p.weff_cj * p.agidl * MIN_EXPL;
                igidl = t3 * t1;
                ggidld = t3 * dt1_dvd;
                ggidlg = t3 * dt1_dvg;
            }
            let mut t4 = vbd - p.fgidl;
            if t4 > model.gidlclamp {
                t4 = model.gidlclamp;
            }
            let t5 = if t4 == 0.0 {
                EXPL_THRESHOLD
            } else {
                p.kgidl / t4
            };
            let t6;
            if t5 < EXPL_THRESHOLD {
                t6 = t5.exp();
                ggidlb = -igidl * t6 * t5 / t4;
            } else {
                t6 = MAX_EXPL;
                ggidlb = 0.0;
            }
            ggidld *= t6;
            ggidlg *= t6;
            igidl *= t6;
        }
    }
    op.igidl = igidl;
    op.ggidld = ggidld;
    op.ggidlg = ggidlg;
    op.ggidlb = ggidlb;
    op.igisl = igisl;
    op.ggisls = ggisls;
    op.ggislg = ggislg;
    op.ggislb = ggislb;

    // --- Gate tunneling currents (2528-2919) ---
    if model.igc_mod != 0 || model.igb_mod != 0 {
        let vfb = inst.vfbzb;
        let v3 = vfb - vgs_eff + vbseff - DELTA_3;
        let t0 = if vfb <= 0.0 {
            (v3 * v3 - 4.0 * DELTA_3 * vfb).sqrt()
        } else {
            (v3 * v3 + 4.0 * DELTA_3 * vfb).sqrt()
        };
        let t1 = 0.5 * (1.0 + v3 / t0);
        let vfbeff = vfb - 0.5 * (v3 + t0);
        let dvfbeff_dvg = t1 * dvgs_eff_dvg;
        let dvfbeff_dvb = -t1;

        let mut voxacc = vfb - vfbeff;
        let mut dvoxacc_dvg = -dvfbeff_dvg;
        let mut dvoxacc_dvb = -dvfbeff_dvb;
        if voxacc < 0.0 {
            voxacc = 0.0;
            dvoxacc_dvg = 0.0;
            dvoxacc_dvb = 0.0;
        }

        let (mut voxdepinv, mut dvoxdepinv_dvg, mut dvoxdepinv_dvd, mut dvoxdepinv_dvb);
        let t0 = 0.5 * p.k1ox;
        let t3 = vgs_eff - vfbeff - vbseff - vgsteff;
        if p.k1ox == 0.0 {
            voxdepinv = 0.0;
            dvoxdepinv_dvg = 0.0;
            dvoxdepinv_dvd = 0.0;
            dvoxdepinv_dvb = 0.0;
        } else if t3 < 0.0 {
            voxdepinv = -t3;
            dvoxdepinv_dvg = -dvgs_eff_dvg + dvfbeff_dvg + dvgsteff_dvg;
            dvoxdepinv_dvd = dvgsteff_dvd;
            dvoxdepinv_dvb = dvfbeff_dvb + 1.0 + dvgsteff_dvb;
        } else {
            let t1 = (t0 * t0 + t3).sqrt();
            let t2 = t0 / t1;
            voxdepinv = p.k1ox * (t1 - t0);
            dvoxdepinv_dvg = t2 * (dvgs_eff_dvg - dvfbeff_dvg - dvgsteff_dvg);
            dvoxdepinv_dvd = -t2 * dvgsteff_dvd;
            dvoxdepinv_dvb = -t2 * (dvfbeff_dvb + 1.0 + dvgsteff_dvb);
        }
        voxdepinv += vgsteff;
        dvoxdepinv_dvg += dvgsteff_dvg;
        dvoxdepinv_dvd += dvgsteff_dvd;
        dvoxdepinv_dvb += dvgsteff_dvb;

        let tmp_vt = if model.temp_mod < 2 { vtm } else { vtm0 };

        if model.igc_mod != 0 {
            let t0 = tmp_vt * p.nigc;
            let (vx_nvt, vbase, dvbase_dvg, dvbase_dvd, dvbase_dvb) = if model.igc_mod == 1 {
                (
                    (vgs_eff - model.mtype * inst.vth0) / t0,
                    vgs_eff - model.mtype * inst.vth0,
                    dvgs_eff_dvg,
                    0.0,
                    0.0,
                )
            } else {
                (
                    (vgs_eff - op.von) / t0,
                    vgs_eff - op.von,
                    dvgs_eff_dvg,
                    -dvth_dvd,
                    -dvth_dvb,
                )
            };
            let (vaux, dvaux_dvg, dvaux_dvd, dvaux_dvb);
            if vx_nvt > EXP_THRESHOLD {
                vaux = vbase;
                dvaux_dvg = dvbase_dvg;
                dvaux_dvd = dvbase_dvd;
                dvaux_dvb = dvbase_dvb;
            } else if vx_nvt < -EXP_THRESHOLD {
                vaux = t0 * (1.0 + MIN_EXP).ln();
                dvaux_dvg = 0.0;
                dvaux_dvd = 0.0;
                dvaux_dvb = 0.0;
            } else {
                let exp_vx_nvt = vx_nvt.exp();
                let frac = exp_vx_nvt / (1.0 + exp_vx_nvt);
                vaux = t0 * (1.0 + exp_vx_nvt).ln();
                dvaux_dvg = frac * dvbase_dvg;
                dvaux_dvd = frac * dvbase_dvd;
                dvaux_dvb = frac * dvbase_dvb;
            }

            let t2 = vgs_c * vaux;
            let dt2_dvg = vaux + vgs_c * dvaux_dvg;
            let dt2_dvd = vgs_c * dvaux_dvd;
            let dt2_dvb = vgs_c * dvaux_dvb;

            let t11 = p.aechvb;
            let t12 = p.bechvb;
            let t3 = p.aigc * p.cigc - p.bigc;
            let t4 = p.bigc * p.cigc;
            let t5 = t12 * (p.aigc + t3 * voxdepinv - t4 * voxdepinv * voxdepinv);
            let (t6, dt6_dvg, dt6_dvd, dt6_dvb);
            if t5 > EXP_THRESHOLD {
                t6 = MAX_EXP;
                dt6_dvg = 0.0;
                dt6_dvd = 0.0;
                dt6_dvb = 0.0;
            } else if t5 < -EXP_THRESHOLD {
                t6 = MIN_EXP;
                dt6_dvg = 0.0;
                dt6_dvd = 0.0;
                dt6_dvb = 0.0;
            } else {
                t6 = t5.exp();
                let dt6_dvox = t6 * t12 * (t3 - 2.0 * t4 * voxdepinv);
                dt6_dvg = dt6_dvox * dvoxdepinv_dvg;
                dt6_dvd = dt6_dvox * dvoxdepinv_dvd;
                dt6_dvb = dt6_dvox * dvoxdepinv_dvb;
            }
            let igc = t11 * t2 * t6;
            let digc_dvg = t11 * (t2 * dt6_dvg + t6 * dt2_dvg);
            let digc_dvd = t11 * (t2 * dt6_dvd + t6 * dt2_dvd);
            let digc_dvb = t11 * (t2 * dt6_dvb + t6 * dt2_dvb);

            let (pigcd, dpigcd_dvg, dpigcd_dvd, dpigcd_dvb);
            if model.pigcd_given {
                pigcd = p.pigcd;
                dpigcd_dvg = 0.0;
                dpigcd_dvd = 0.0;
                dpigcd_dvb = 0.0;
            } else {
                let t11 = -p.bechvb;
                let t12 = vgsteff + 1.0e-20;
                let t13 = t11 / t12 / t12;
                let t14 = -t13 / t12;
                pigcd = t13 * (1.0 - 0.5 * vdseff / t12);
                dpigcd_dvg = t14 * (2.0 + 0.5 * (dvdseff_dvg - 3.0 * vdseff / t12));
                dpigcd_dvd = 0.5 * t14 * dvdseff_dvd;
                dpigcd_dvb = 0.5 * t14 * dvdseff_dvb;
            }

            let t7 = -pigcd * vdseff;
            let mut dt7_dvg = -vdseff * dpigcd_dvg - pigcd * dvdseff_dvg;
            let dt7_dvd = -vdseff * dpigcd_dvd - pigcd * dvdseff_dvd + dt7_dvg * dvgsteff_dvd;
            let dt7_dvb = -vdseff * dpigcd_dvb - pigcd * dvdseff_dvb + dt7_dvg * dvgsteff_dvb;
            dt7_dvg *= dvgsteff_dvg;

            let t8 = t7 * t7 + 2.0e-4;
            let dt8_dvg = 2.0 * t7 * dt7_dvg;
            let dt8_dvd = 2.0 * t7 * dt7_dvd;
            let dt8_dvb = 2.0 * t7 * dt7_dvb;

            let (t9, dt9_dvg, dt9_dvd, dt9_dvb);
            if t7 > EXP_THRESHOLD {
                t9 = MAX_EXP;
                dt9_dvg = 0.0;
                dt9_dvd = 0.0;
                dt9_dvb = 0.0;
            } else if t7 < -EXP_THRESHOLD {
                t9 = MIN_EXP;
                dt9_dvg = 0.0;
                dt9_dvd = 0.0;
                dt9_dvb = 0.0;
            } else {
                t9 = t7.exp();
                dt9_dvg = t9 * dt7_dvg;
                dt9_dvd = t9 * dt7_dvd;
                dt9_dvb = t9 * dt7_dvb;
            }

            let t1 = t9 - 1.0 + 1.0e-4;
            let t10 = (t1 - t7) / t8;
            let dt10_dvg = (dt9_dvg - dt7_dvg - t10 * dt8_dvg) / t8;
            let dt10_dvd = (dt9_dvd - dt7_dvd - t10 * dt8_dvd) / t8;
            let dt10_dvb = (dt9_dvb - dt7_dvb - t10 * dt8_dvb) / t8;
            op.igcs = igc * t10;
            op.gigcsg = digc_dvg * t10 + igc * dt10_dvg;
            op.gigcsd = digc_dvd * t10 + igc * dt10_dvd;
            op.gigcsb = (digc_dvb * t10 + igc * dt10_dvb) * dvbseff_dvb;

            let t1 = t9 - 1.0 - 1.0e-4;
            let t10 = (t7 * t9 - t1) / t8;
            let dt10_dvg = (dt7_dvg * t9 + (t7 - 1.0) * dt9_dvg - t10 * dt8_dvg) / t8;
            let dt10_dvd = (dt7_dvd * t9 + (t7 - 1.0) * dt9_dvd - t10 * dt8_dvd) / t8;
            let dt10_dvb = (dt7_dvb * t9 + (t7 - 1.0) * dt9_dvb - t10 * dt8_dvb) / t8;
            op.igcd = igc * t10;
            op.gigcdg = digc_dvg * t10 + igc * dt10_dvg;
            op.gigcdd = digc_dvd * t10 + igc * dt10_dvd;
            op.gigcdb = (digc_dvb * t10 + igc * dt10_dvb) * dvbseff_dvb;

            let t0 = vgs - (p.vfbsd + p.vfbsdoff);
            let vgs_eff_edge = (t0 * t0 + 1.0e-4).sqrt();
            let dvgs_eff_edge_dvg = t0 / vgs_eff_edge;
            let t2 = vgs * vgs_eff_edge;
            let dt2_dvg = vgs * dvgs_eff_edge_dvg + vgs_eff_edge;
            let t11 = p.aechvb_edge_s;
            let t12 = p.bechvb_edge;
            let t3 = p.aigs * p.cigs - p.bigs;
            let t4 = p.bigs * p.cigs;
            let t5 = t12 * (p.aigs + t3 * vgs_eff_edge - t4 * vgs_eff_edge * vgs_eff_edge);
            let (t6, dt6_dvg);
            if t5 > EXP_THRESHOLD {
                t6 = MAX_EXP;
                dt6_dvg = 0.0;
            } else if t5 < -EXP_THRESHOLD {
                t6 = MIN_EXP;
                dt6_dvg = 0.0;
            } else {
                t6 = t5.exp();
                dt6_dvg = t6 * t12 * (t3 - 2.0 * t4 * vgs_eff_edge) * dvgs_eff_edge_dvg;
            }
            op.igs = t11 * t2 * t6;
            op.gigsg = t11 * (t2 * dt6_dvg + t6 * dt2_dvg);
            op.gigss = -op.gigsg;

            let t0 = vgd - (p.vfbsd + p.vfbsdoff);
            let vgd_eff_edge = (t0 * t0 + 1.0e-4).sqrt();
            let dvgd_eff_edge_dvg = t0 / vgd_eff_edge;
            let t2 = vgd * vgd_eff_edge;
            let dt2_dvg = vgd * dvgd_eff_edge_dvg + vgd_eff_edge;
            let t11 = p.aechvb_edge_d;
            let t3 = p.aigd * p.cigd - p.bigd;
            let t4 = p.bigd * p.cigd;
            let t5 = t12 * (p.aigd + t3 * vgd_eff_edge - t4 * vgd_eff_edge * vgd_eff_edge);
            let (t6, dt6_dvg);
            if t5 > EXP_THRESHOLD {
                t6 = MAX_EXP;
                dt6_dvg = 0.0;
            } else if t5 < -EXP_THRESHOLD {
                t6 = MIN_EXP;
                dt6_dvg = 0.0;
            } else {
                t6 = t5.exp();
                dt6_dvg = t6 * t12 * (t3 - 2.0 * t4 * vgd_eff_edge) * dvgd_eff_edge_dvg;
            }
            op.igd = t11 * t2 * t6;
            op.gigdg = t11 * (t2 * dt6_dvg + t6 * dt2_dvg);
            op.gigdd = -op.gigdg;
        }

        if model.igb_mod != 0 {
            let t0 = tmp_vt * p.nigbacc;
            let t1 = -vgs_eff + vbseff + vfb;
            let vx_nvt = t1 / t0;
            let (vaux, dvaux_dvg, dvaux_dvb);
            if vx_nvt > EXP_THRESHOLD {
                vaux = t1;
                dvaux_dvg = -dvgs_eff_dvg;
                dvaux_dvb = 1.0;
            } else if vx_nvt < -EXP_THRESHOLD {
                vaux = t0 * (1.0 + MIN_EXP).ln();
                dvaux_dvg = 0.0;
                dvaux_dvb = 0.0;
            } else {
                let exp_vx_nvt = vx_nvt.exp();
                let frac = exp_vx_nvt / (1.0 + exp_vx_nvt);
                vaux = t0 * (1.0 + exp_vx_nvt).ln();
                dvaux_dvb = frac;
                dvaux_dvg = -frac * dvgs_eff_dvg;
            }
            let t2 = (vgs_c - vbs_c) * vaux;
            let dt2_dvg = vaux + (vgs_c - vbs_c) * dvaux_dvg;
            let dt2_dvb = -vaux + (vgs_c - vbs_c) * dvaux_dvb;

            let mut t11 = 4.97232e-7 * p.weff * p.leff * p.tox_ratio;
            let mut t12 = -7.45669e11 * toxe;
            let t3 = p.aigbacc * p.cigbacc - p.bigbacc;
            let t4 = p.bigbacc * p.cigbacc;
            let t5 = t12 * (p.aigbacc + t3 * voxacc - t4 * voxacc * voxacc);
            let (t6, dt6_dvg, dt6_dvb);
            if t5 > EXP_THRESHOLD {
                t6 = MAX_EXP;
                dt6_dvg = 0.0;
                dt6_dvb = 0.0;
            } else if t5 < -EXP_THRESHOLD {
                t6 = MIN_EXP;
                dt6_dvg = 0.0;
                dt6_dvb = 0.0;
            } else {
                t6 = t5.exp();
                let dt6_dvox = t6 * t12 * (t3 - 2.0 * t4 * voxacc);
                dt6_dvg = dt6_dvox * dvoxacc_dvg;
                dt6_dvb = dt6_dvox * dvoxacc_dvb;
            }
            let igbacc = t11 * t2 * t6;
            let digbacc_dvg = t11 * (t2 * dt6_dvg + t6 * dt2_dvg);
            let digbacc_dvb = t11 * (t2 * dt6_dvb + t6 * dt2_dvb);

            let t0 = tmp_vt * p.nigbinv;
            let t1 = voxdepinv - p.eigbinv;
            let vx_nvt = t1 / t0;
            let (vaux, dvaux_dvg, dvaux_dvd, dvaux_dvb);
            if vx_nvt > EXP_THRESHOLD {
                vaux = t1;
                dvaux_dvg = dvoxdepinv_dvg;
                dvaux_dvd = dvoxdepinv_dvd;
                dvaux_dvb = dvoxdepinv_dvb;
            } else if vx_nvt < -EXP_THRESHOLD {
                vaux = t0 * (1.0 + MIN_EXP).ln();
                dvaux_dvg = 0.0;
                dvaux_dvd = 0.0;
                dvaux_dvb = 0.0;
            } else {
                let exp_vx_nvt = vx_nvt.exp();
                let frac = exp_vx_nvt / (1.0 + exp_vx_nvt);
                vaux = t0 * (1.0 + exp_vx_nvt).ln();
                dvaux_dvg = frac * dvoxdepinv_dvg;
                dvaux_dvd = frac * dvoxdepinv_dvd;
                dvaux_dvb = frac * dvoxdepinv_dvb;
            }
            let t2 = (vgs_c - vbs_c) * vaux;
            let dt2_dvg = vaux + (vgs_c - vbs_c) * dvaux_dvg;
            let dt2_dvd = (vgs_c - vbs_c) * dvaux_dvd;
            let dt2_dvb = -vaux + (vgs_c - vbs_c) * dvaux_dvb;

            t11 *= 0.75610;
            t12 *= 1.31724;
            let t3 = p.aigbinv * p.cigbinv - p.bigbinv;
            let t4 = p.bigbinv * p.cigbinv;
            let t5 = t12 * (p.aigbinv + t3 * voxdepinv - t4 * voxdepinv * voxdepinv);
            let (t6, dt6_dvg, dt6_dvd, dt6_dvb);
            if t5 > EXP_THRESHOLD {
                t6 = MAX_EXP;
                dt6_dvg = 0.0;
                dt6_dvd = 0.0;
                dt6_dvb = 0.0;
            } else if t5 < -EXP_THRESHOLD {
                t6 = MIN_EXP;
                dt6_dvg = 0.0;
                dt6_dvd = 0.0;
                dt6_dvb = 0.0;
            } else {
                t6 = t5.exp();
                let dt6_dvox = t6 * t12 * (t3 - 2.0 * t4 * voxdepinv);
                dt6_dvg = dt6_dvox * dvoxdepinv_dvg;
                dt6_dvd = dt6_dvox * dvoxdepinv_dvd;
                dt6_dvb = dt6_dvox * dvoxdepinv_dvb;
            }
            let igbinv = t11 * t2 * t6;
            let digbinv_dvg = t11 * (t2 * dt6_dvg + t6 * dt2_dvg);
            let digbinv_dvd = t11 * (t2 * dt6_dvd + t6 * dt2_dvd);
            let digbinv_dvb = t11 * (t2 * dt6_dvb + t6 * dt2_dvb);

            op.igb = igbinv + igbacc;
            op.gigbg = digbinv_dvg + digbacc_dvg;
            op.gigbd = digbinv_dvd;
            op.gigbb = (digbinv_dvb + digbacc_dvb) * dvbseff_dvb;
        }
    }

    // --- NF scaling (2921-2965) ---
    if nf != 1.0 {
        cdrain *= nf;
        op.gds *= nf;
        op.gm *= nf;
        op.gmbs *= nf;
        op.idovvds *= nf;

        op.gbbs *= nf;
        op.gbgs *= nf;
        op.gbds *= nf;
        op.csub *= nf;

        op.igidl *= nf;
        op.ggidld *= nf;
        op.ggidlg *= nf;
        op.ggidlb *= nf;

        op.igisl *= nf;
        op.ggisls *= nf;
        op.ggislg *= nf;
        op.ggislb *= nf;

        op.igcs *= nf;
        op.gigcsg *= nf;
        op.gigcsd *= nf;
        op.gigcsb *= nf;
        op.igcd *= nf;
        op.gigcdg *= nf;
        op.gigcdd *= nf;
        op.gigcdb *= nf;

        op.igs *= nf;
        op.gigsg *= nf;
        op.gigss *= nf;
        op.igd *= nf;
        op.gigdg *= nf;
        op.gigdd *= nf;

        op.igb *= nf;
        op.gigbg *= nf;
        op.gigbd *= nf;
        op.gigbb *= nf;
    }
    op.ggidls = -(op.ggidld + op.ggidlg + op.ggidlb);
    op.ggisld = -(op.ggisls + op.ggislg + op.ggislb);
    op.gigbs = -(op.gigbg + op.gigbd + op.gigbb);
    op.gigcss = -(op.gigcsg + op.gigcsd + op.gigcsb);
    op.gigcds = -(op.gigcdg + op.gigcdd + op.gigcdb);
    op.cd = cdrain;

    // --- qinv/noiGd0 for noise (b4ld.c:2980-3006) ---
    if model.tnoi_mod == 0 {
        let abulk_n = abulk0_q * p.abulk_cv_factor;
        let vdsat_n = vgsteff / abulk_n;
        let t0 = vdsat_n - vds_c - DELTA_4;
        let t1 = (t0 * t0 + 4.0 * DELTA_4 * vdsat_n).sqrt();
        let mut vdseff_n;
        if t0 >= 0.0 {
            vdseff_n = vdsat_n - 0.5 * (t0 + t1);
        } else {
            let t3 = (DELTA_4 + DELTA_4) / (t1 - t0);
            let t4 = 1.0 - t3;
            vdseff_n = vdsat_n * t4;
        }
        if vds_c == 0.0 {
            vdseff_n = 0.0;
        }
        let t0 = abulk_n * vdseff_n;
        let t1 = 12.0 * (vgsteff - 0.5 * t0 + 1.0e-20);
        let t2 = vdseff_n / t1;
        let t3 = t0 * t2;
        op.qinv = coxeff * p.weff_cv * nf * p.leff_cv * (vgsteff - 0.5 * t0 + abulk_n * t3);
    } else if model.tnoi_mod == 2 {
        let denom = 1.0 + gche * rds;
        if denom != 0.0 {
            op.noi_gd0 = nf * beta * vgsteff / denom;
        }
    }

    // ===================== BSIM4 C-V begins =====================
    if !compute_charges {
        return Ok(op);
    }

    let mut ch = Bsim4v8Charge::default();

    let (qgate, qdrn, qbulk);
    if model.xpart < 0.0 {
        // Intrinsic charge suppression (xpart < 0). Overlap and junction
        // charges below still use the selected CAPMOD overlap form.
        qgate = 0.0;
        qdrn = 0.0;
        qbulk = 0.0;
        // capacitance matrix stays zero.
    } else if model.cap_mod == 0 {
        // --- capMod = 0 intrinsic charge model (b4ld.c:3026-3334) ---
        let (vbseff_cv, dvbseff_cv_dvb) = if vbseff < 0.0 {
            (vbs_c, 1.0)
        } else {
            (p.phi - phis, dvbseff_dvb)
        };

        let vfb = p.vfbcv;
        let vth_cv = vfb + p.phi + p.k1ox * sqrt_phis;
        let vgst_cv = vgs_eff - vth_cv;
        let dvth_cv_dvb = p.k1ox * dsqrt_phis_dvb * dvbseff_dvb;
        let cox_wl = mt.coxe * p.weff_cv * p.leff_cv * nf;
        let arg1 = vgs_eff - vbseff_cv - vfb;

        let (qgate_l, qbulk_l, qdrn_l);
        if arg1 <= 0.0 {
            // Accumulation.
            qgate_l = cox_wl * arg1;
            qbulk_l = -qgate_l;
            qdrn_l = 0.0;

            ch.cggb = cox_wl * dvgs_eff_dvg;
            ch.cgdb = 0.0;
            ch.cgsb = cox_wl * (dvbseff_cv_dvb - dvgs_eff_dvg);

            ch.cdgb = 0.0;
            ch.cddb = 0.0;
            ch.cdsb = 0.0;

            ch.cbgb = -cox_wl * dvgs_eff_dvg;
            ch.cbdb = 0.0;
            ch.cbsb = -ch.cgsb;
        } else if vgst_cv <= 0.0 {
            // Depletion.
            let t1 = 0.5 * p.k1ox;
            let t2 = (t1 * t1 + arg1).sqrt();
            qgate_l = cox_wl * p.k1ox * (t2 - t1);
            qbulk_l = -qgate_l;
            qdrn_l = 0.0;

            let t0 = cox_wl * t1 / t2;
            ch.cggb = t0 * dvgs_eff_dvg;
            ch.cgdb = 0.0;
            ch.cgsb = t0 * (dvbseff_cv_dvb - dvgs_eff_dvg);

            ch.cdgb = 0.0;
            ch.cddb = 0.0;
            ch.cdsb = 0.0;

            ch.cbgb = -ch.cggb;
            ch.cbdb = 0.0;
            ch.cbsb = -ch.cgsb;
        } else {
            // Inversion.
            let one_third_cox_wl = cox_wl / 3.0;
            let two_third_cox_wl = 2.0 * one_third_cox_wl;

            let abulk_cv = abulk0_q * p.abulk_cv_factor;
            let dabulk_cv_dvb = p.abulk_cv_factor * dabulk0_q_dvb * dvbseff_dvb;
            let dvdsat_dvg = 1.0 / abulk_cv;
            let vdsat = vgst_cv * dvdsat_dvg;
            let dvdsat_dvb = -(vdsat * dabulk_cv_dvb + dvth_cv_dvb) * dvdsat_dvg;

            if model.xpart > 0.5 {
                // 0/100 partition.
                if vdsat <= vds_c {
                    // Saturation.
                    let t1 = vdsat / 3.0;
                    qgate_l = cox_wl * (vgs_eff - vfb - p.phi - t1);
                    let t2 = -two_third_cox_wl * vgst_cv;
                    qbulk_l = -(qgate_l + t2);
                    qdrn_l = 0.0;

                    ch.cggb = one_third_cox_wl * (3.0 - dvdsat_dvg) * dvgs_eff_dvg;
                    let t2 = -one_third_cox_wl * dvdsat_dvb;
                    ch.cgsb = -(ch.cggb + t2);
                    ch.cgdb = 0.0;

                    ch.cdgb = 0.0;
                    ch.cddb = 0.0;
                    ch.cdsb = 0.0;

                    ch.cbgb = -(ch.cggb - two_third_cox_wl * dvgs_eff_dvg);
                    let t3 = -(t2 + two_third_cox_wl * dvth_cv_dvb);
                    ch.cbsb = -(ch.cbgb + t3);
                    ch.cbdb = 0.0;
                } else {
                    // Linear.
                    let alphaz = vgst_cv / vdsat;
                    let t1 = 2.0 * vdsat - vds_c;
                    let t2 = vds_c / (3.0 * t1);
                    let t3 = t2 * vds_c;
                    let t9 = 0.25 * cox_wl;
                    let t4 = t9 * alphaz;
                    let t7 = 2.0 * vds_c - t1 - 3.0 * t3;
                    let t8 = t3 - t1 - 2.0 * vds_c;
                    qgate_l = cox_wl * (vgs_eff - vfb - p.phi - 0.5 * (vds_c - t3));
                    let t10_q = t4 * t8;
                    qdrn_l = t4 * t7;
                    qbulk_l = -(qgate_l + qdrn_l + t10_q);

                    let t5 = t3 / t1;
                    ch.cggb = cox_wl * (1.0 - t5 * dvdsat_dvg) * dvgs_eff_dvg;
                    let t11 = -cox_wl * t5 * dvdsat_dvb;
                    ch.cgdb = cox_wl * (t2 - 0.5 + 0.5 * t5);
                    ch.cgsb = -(ch.cggb + t11 + ch.cgdb);
                    let t6 = 1.0 / vdsat;
                    let dalphaz_dvg = t6 * (1.0 - alphaz * dvdsat_dvg);
                    let dalphaz_dvb = -t6 * (dvth_cv_dvb + alphaz * dvdsat_dvb);
                    let t7 = t9 * t7;
                    let t8 = t9 * t8;
                    let t9 = 2.0 * t4 * (1.0 - 3.0 * t5);
                    ch.cdgb = (t7 * dalphaz_dvg - t9 * dvdsat_dvg) * dvgs_eff_dvg;
                    let t12 = t7 * dalphaz_dvb - t9 * dvdsat_dvb;
                    ch.cddb = t4 * (3.0 - 6.0 * t2 - 3.0 * t5);
                    ch.cdsb = -(ch.cdgb + t12 + ch.cddb);

                    let t9 = 2.0 * t4 * (1.0 + t5);
                    let t10 = (t8 * dalphaz_dvg - t9 * dvdsat_dvg) * dvgs_eff_dvg;
                    let t11 = t8 * dalphaz_dvb - t9 * dvdsat_dvb;
                    let t12 = t4 * (2.0 * t2 + t5 - 1.0);
                    let t0 = -(t10 + t11 + t12);

                    ch.cbgb = -(ch.cggb + ch.cdgb + t10);
                    ch.cbdb = -(ch.cgdb + ch.cddb + t12);
                    ch.cbsb = -(ch.cgsb + ch.cdsb + t0);
                }
            } else if model.xpart < 0.5 {
                // 40/60 partition.
                if vds_c >= vdsat {
                    // Saturation.
                    let t1 = vdsat / 3.0;
                    qgate_l = cox_wl * (vgs_eff - vfb - p.phi - t1);
                    let t2 = -two_third_cox_wl * vgst_cv;
                    qbulk_l = -(qgate_l + t2);
                    qdrn_l = 0.4 * t2;

                    ch.cggb = one_third_cox_wl * (3.0 - dvdsat_dvg) * dvgs_eff_dvg;
                    let t2 = -one_third_cox_wl * dvdsat_dvb;
                    ch.cgsb = -(ch.cggb + t2);
                    ch.cgdb = 0.0;

                    let t3 = 0.4 * two_third_cox_wl;
                    ch.cdgb = -t3 * dvgs_eff_dvg;
                    ch.cddb = 0.0;
                    let t4 = t3 * dvth_cv_dvb;
                    ch.cdsb = -(t4 + ch.cdgb);

                    ch.cbgb = -(ch.cggb - two_third_cox_wl * dvgs_eff_dvg);
                    let t3 = -(t2 + two_third_cox_wl * dvth_cv_dvb);
                    ch.cbsb = -(ch.cbgb + t3);
                    ch.cbdb = 0.0;
                } else {
                    // Linear.
                    let alphaz = vgst_cv / vdsat;
                    let t1 = 2.0 * vdsat - vds_c;
                    let t2 = vds_c / (3.0 * t1);
                    let t3 = t2 * vds_c;
                    let t9 = 0.25 * cox_wl;
                    let t4 = t9 * alphaz;
                    qgate_l = cox_wl * (vgs_eff - vfb - p.phi - 0.5 * (vds_c - t3));

                    let t5 = t3 / t1;
                    ch.cggb = cox_wl * (1.0 - t5 * dvdsat_dvg) * dvgs_eff_dvg;
                    let tmp = -cox_wl * t5 * dvdsat_dvb;
                    ch.cgdb = cox_wl * (t2 - 0.5 + 0.5 * t5);
                    ch.cgsb = -(ch.cggb + ch.cgdb + tmp);

                    let t6 = 1.0 / vdsat;
                    let dalphaz_dvg = t6 * (1.0 - alphaz * dvdsat_dvg);
                    let dalphaz_dvb = -t6 * (dvth_cv_dvb + alphaz * dvdsat_dvb);

                    let t6 = 8.0 * vdsat * vdsat - 6.0 * vdsat * vds_c + 1.2 * vds_c * vds_c;
                    let t8 = t2 / t1;
                    let t7 = vds_c - t1 - t8 * t6;
                    qdrn_l = t4 * t7;
                    let t7 = t7 * t9;
                    let tmp = t8 / t1;
                    let tmp1 = t4 * (2.0 - 4.0 * tmp * t6 + t8 * (16.0 * vdsat - 6.0 * vds_c));

                    ch.cdgb = (t7 * dalphaz_dvg - tmp1 * dvdsat_dvg) * dvgs_eff_dvg;
                    let t10 = t7 * dalphaz_dvb - tmp1 * dvdsat_dvb;
                    ch.cddb = t4
                        * (2.0 - (1.0 / (3.0 * t1 * t1) + 2.0 * tmp) * t6
                            + t8 * (6.0 * vdsat - 2.4 * vds_c));
                    ch.cdsb = -(ch.cdgb + t10 + ch.cddb);

                    let t7 = 2.0 * (t1 + t3);
                    qbulk_l = -(qgate_l - t4 * t7);
                    let t7 = t7 * t9;
                    let t0 = 4.0 * t4 * (1.0 - t5);
                    let t12 = (-t7 * dalphaz_dvg - t0 * dvdsat_dvg) * dvgs_eff_dvg - ch.cdgb;
                    let t11 = -t7 * dalphaz_dvb - t10 - t0 * dvdsat_dvb;
                    let t10 = -4.0 * t4 * (t2 - 0.5 + 0.5 * t5) - ch.cddb;
                    let tmp = -(t10 + t11 + t12);

                    ch.cbgb = -(ch.cggb + ch.cdgb + t12);
                    ch.cbdb = -(ch.cgdb + ch.cddb + t10);
                    ch.cbsb = -(ch.cgsb + ch.cdsb + tmp);
                }
            } else {
                // 50/50 partition.
                if vds_c >= vdsat {
                    // Saturation.
                    let t1 = vdsat / 3.0;
                    qgate_l = cox_wl * (vgs_eff - vfb - p.phi - t1);
                    let t2 = -two_third_cox_wl * vgst_cv;
                    qbulk_l = -(qgate_l + t2);
                    qdrn_l = 0.5 * t2;

                    ch.cggb = one_third_cox_wl * (3.0 - dvdsat_dvg) * dvgs_eff_dvg;
                    let t2 = -one_third_cox_wl * dvdsat_dvb;
                    ch.cgsb = -(ch.cggb + t2);
                    ch.cgdb = 0.0;

                    ch.cdgb = -one_third_cox_wl * dvgs_eff_dvg;
                    ch.cddb = 0.0;
                    let t4 = one_third_cox_wl * dvth_cv_dvb;
                    ch.cdsb = -(t4 + ch.cdgb);

                    ch.cbgb = -(ch.cggb - two_third_cox_wl * dvgs_eff_dvg);
                    let t3 = -(t2 + two_third_cox_wl * dvth_cv_dvb);
                    ch.cbsb = -(ch.cbgb + t3);
                    ch.cbdb = 0.0;
                } else {
                    // Linear.
                    let alphaz = vgst_cv / vdsat;
                    let t1 = 2.0 * vdsat - vds_c;
                    let t2 = vds_c / (3.0 * t1);
                    let t3 = t2 * vds_c;
                    let t9 = 0.25 * cox_wl;
                    let t4 = t9 * alphaz;
                    qgate_l = cox_wl * (vgs_eff - vfb - p.phi - 0.5 * (vds_c - t3));

                    let t5 = t3 / t1;
                    ch.cggb = cox_wl * (1.0 - t5 * dvdsat_dvg) * dvgs_eff_dvg;
                    let tmp = -cox_wl * t5 * dvdsat_dvb;
                    ch.cgdb = cox_wl * (t2 - 0.5 + 0.5 * t5);
                    ch.cgsb = -(ch.cggb + ch.cgdb + tmp);

                    let t6 = 1.0 / vdsat;
                    let dalphaz_dvg = t6 * (1.0 - alphaz * dvdsat_dvg);
                    let dalphaz_dvb = -t6 * (dvth_cv_dvb + alphaz * dvdsat_dvb);

                    let t7 = t1 + t3;
                    qdrn_l = -t4 * t7;
                    qbulk_l = -(qgate_l + qdrn_l + qdrn_l);
                    let t7 = t7 * t9;
                    let t0 = t4 * (2.0 * t5 - 2.0);

                    ch.cdgb = (t0 * dvdsat_dvg - t7 * dalphaz_dvg) * dvgs_eff_dvg;
                    let t12 = t0 * dvdsat_dvb - t7 * dalphaz_dvb;
                    ch.cddb = t4 * (1.0 - 2.0 * t2 - t5);
                    ch.cdsb = -(ch.cdgb + t12 + ch.cddb);

                    ch.cbgb = -(ch.cggb + 2.0 * ch.cdgb);
                    ch.cbdb = -(ch.cgdb + 2.0 * ch.cddb);
                    ch.cbsb = -(ch.cgsb + 2.0 * ch.cdsb);
                }
            }
        }

        qgate = qgate_l;
        qbulk = qbulk_l;
        qdrn = qdrn_l;
    } else {
        // capMod = 1/2 share the C-V prelude and smoothed overlap form.
        let (vbseff_cv, dvbseff_cv_dvb);
        if vbseff < 0.0 {
            vbseff_cv = vbseff;
            dvbseff_cv_dvb = 1.0;
        } else {
            vbseff_cv = p.phi - phis;
            dvbseff_cv_dvb = 1.0; // -dPhis_dVb with dPhis_dVb = -1
        }

        let cox_wl = mt.coxe * p.weff_cv * p.leff_cv * nf;

        let (vgsteff_cv, dvgsteff_cv_dvg, dvgsteff_cv_dvd, dvgsteff_cv_dvb) =
            if model.cvcharge_mod == 0 {
                // VgsteffCV with noff and voffcv (cvchargeMod = 0; 3353-3387).
                let noff = n * p.noff;
                let dnoff_dvd = p.noff * dn_dvd;
                let dnoff_dvb = p.noff * dn_dvb;
                let t0 = vtm * noff;
                let voffcv = p.voffcv;
                let vgst_nvt = (vgst - voffcv) / t0;

                if vgst_nvt > EXP_THRESHOLD {
                    (vgst - voffcv, dvgs_eff_dvg, -dvth_dvd, -dvth_dvb)
                } else if vgst_nvt < -EXP_THRESHOLD {
                    let vgsteff_cv = t0 * (1.0 + MIN_EXP).ln();
                    let d = vgsteff_cv / noff;
                    (vgsteff_cv, 0.0, d * dnoff_dvd, d * dnoff_dvb)
                } else {
                    let exp_vgst = vgst_nvt.exp();
                    let vgsteff_cv = t0 * (1.0 + exp_vgst).ln();
                    let dvg = exp_vgst / (1.0 + exp_vgst);
                    let dvgsteff_cv_dvd = -dvg * (dvth_dvd + (vgst - voffcv) / noff * dnoff_dvd)
                        + vgsteff_cv / noff * dnoff_dvd;
                    let dvgsteff_cv_dvb = -dvg * (dvth_dvb + (vgst - voffcv) / noff * dnoff_dvb)
                        + vgsteff_cv / noff * dnoff_dvb;
                    (
                        vgsteff_cv,
                        dvg * dvgs_eff_dvg,
                        dvgsteff_cv_dvd,
                        dvgsteff_cv_dvb,
                    )
                }
            } else {
                // VgsteffCV for nonzero cvchargeMod (ngspice labels the
                // branch as cvchargeMod = 1; b4ld.c:3389-3456).
                let t0 = n * vtm;
                let t1 = p.mstarcv * vgst;
                let t2 = t1 / t0;
                let (t10, dt10_dvg, dt10_dvd, dt10_dvb);
                if t2 > EXP_THRESHOLD {
                    t10 = t1;
                    dt10_dvg = p.mstarcv * dvgs_eff_dvg;
                    dt10_dvd = -dvth_dvd * p.mstarcv;
                    dt10_dvb = -dvth_dvb * p.mstarcv;
                } else if t2 < -EXP_THRESHOLD {
                    let t10p = vtm * (1.0 + MIN_EXP).ln();
                    dt10_dvg = 0.0;
                    dt10_dvd = t10p * dn_dvd;
                    dt10_dvb = t10p * dn_dvb;
                    t10 = t10p * n;
                } else {
                    let exp_vgst = t2.exp();
                    let t3 = vtm * (1.0 + exp_vgst).ln();
                    t10 = n * t3;
                    let dt10g = p.mstarcv * exp_vgst / (1.0 + exp_vgst);
                    dt10_dvb = t3 * dn_dvb - dt10g * (dvth_dvb + vgst * dn_dvb / n);
                    dt10_dvd = t3 * dn_dvd - dt10g * (dvth_dvd + vgst * dn_dvd / n);
                    dt10_dvg = dt10g * dvgs_eff_dvg;
                }

                let t1 = p.voffcbncv - (1.0 - p.mstarcv) * vgst;
                let t2 = t1 / t0;
                let (t9, dt9_dvg, dt9_dvd, dt9_dvb);
                if t2 < -EXP_THRESHOLD {
                    let t3 = mt.coxe * MIN_EXP / p.cdep0;
                    t9 = p.mstarcv + t3 * n;
                    dt9_dvg = 0.0;
                    dt9_dvd = dn_dvd * t3;
                    dt9_dvb = dn_dvb * t3;
                } else if t2 > EXP_THRESHOLD {
                    let t3 = mt.coxe * MAX_EXP / p.cdep0;
                    t9 = p.mstarcv + t3 * n;
                    dt9_dvg = 0.0;
                    dt9_dvd = dn_dvd * t3;
                    dt9_dvb = dn_dvb * t3;
                } else {
                    let exp_vgst = t2.exp();
                    let t3 = mt.coxe / p.cdep0;
                    let t4 = t3 * exp_vgst;
                    let t5 = t1 * t4 / t0;
                    t9 = p.mstarcv + n * t4;
                    let dt9g = t3 * (p.mstarcv - 1.0) * exp_vgst / vtm;
                    dt9_dvb = t4 * dn_dvb - dt9g * dvth_dvb - t5 * dn_dvb;
                    dt9_dvd = t4 * dn_dvd - dt9g * dvth_dvd - t5 * dn_dvd;
                    dt9_dvg = dt9g * dvgs_eff_dvg;
                }

                let vgsteff_cv = t10 / t9;
                let t11 = t9 * t9;
                (
                    vgsteff_cv,
                    (t9 * dt10_dvg - t10 * dt9_dvg) / t11,
                    (t9 * dt10_dvd - t10 * dt9_dvd) / t11,
                    (t9 * dt10_dvb - t10 * dt9_dvb) / t11,
                )
            };

        if model.cap_mod == 1 {
            // --- capMod = 1 intrinsic charge model (b4ld.c:3460-3629) ---
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

            let abulk_cv = abulk0_q * p.abulk_cv_factor;
            let dabulk_cv_dvb = p.abulk_cv_factor * dabulk0_q_dvb;
            let vdsat_cv = vgsteff_cv / abulk_cv;

            let t0 = vdsat_cv - vds_c - DELTA_4;
            let dt0_dvg = 1.0 / abulk_cv;
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
                dvdseff_cv_dvb = dt0_dvb * (t4 - t5) + t5 * dt1_dvb;
            }

            if vds_c == 0.0 {
                vdseff_cv = 0.0;
                dvdseff_cv_dvg = 0.0;
                dvdseff_cv_dvb = 0.0;
            }

            let t0 = abulk_cv * vdseff_cv;
            let t1 = 12.0 * (vgsteff_cv - 0.5 * t0 + 1.0e-20);
            let t2 = vdseff_cv / t1;
            let t3 = t0 * t2;

            let t4 = 1.0 - 12.0 * t2 * t2 * abulk_cv;
            let t5 = 6.0 * t0 * (4.0 * vgsteff_cv - t0) / (t1 * t1) - 0.5;
            let t6 = 12.0 * t2 * t2 * vgsteff_cv;

            let mut qgate_l = cox_wl * (vgsteff_cv - 0.5 * vdseff_cv + t3);
            let mut cgg1 = cox_wl * (t4 + t5 * dvdseff_cv_dvg);
            let cgd1 = cox_wl * t5 * dvdseff_cv_dvd + cgg1 * dvgsteff_cv_dvd;
            let cgb1 = cox_wl * (t5 * dvdseff_cv_dvb + t6 * dabulk_cv_dvb) + cgg1 * dvgsteff_cv_dvb;
            cgg1 *= dvgsteff_cv_dvg;

            let t7 = 1.0 - abulk_cv;
            let mut qbulk_l = cox_wl * t7 * (0.5 * vdseff_cv - t3);
            let t4 = -t7 * (t4 - 1.0);
            let t5 = -t7 * t5;
            let t6 = -(t7 * t6 + (0.5 * vdseff_cv - t3));
            let mut cbg1 = cox_wl * (t4 + t5 * dvdseff_cv_dvg);
            let cbd1 = cox_wl * t5 * dvdseff_cv_dvd + cbg1 * dvgsteff_cv_dvd;
            let cbb1 = cox_wl * (t5 * dvdseff_cv_dvb + t6 * dabulk_cv_dvb) + cbg1 * dvgsteff_cv_dvb;
            cbg1 *= dvgsteff_cv_dvg;

            let (qsrc, mut csg, csd, mut csb);
            if model.xpart > 0.5 {
                // 0/100 partition.
                let t1x = t1 + t1;
                qsrc = -cox_wl * (0.5 * vgsteff_cv + 0.25 * t0 - t0 * t0 / t1x);
                let t7x = (4.0 * vgsteff_cv - t0) / (t1x * t1x);
                let t4x = -(0.5 + 24.0 * t0 * t0 / (t1x * t1x));
                let t5x = -(0.25 * abulk_cv - 12.0 * abulk_cv * t0 * t7x);
                let t6x = -(0.25 * vdseff_cv - 12.0 * t0 * vdseff_cv * t7x);
                csg = cox_wl * (t4x + t5x * dvdseff_cv_dvg);
                csd = cox_wl * t5x * dvdseff_cv_dvd + csg * dvgsteff_cv_dvd;
                csb = cox_wl * (t5x * dvdseff_cv_dvb + t6x * dabulk_cv_dvb) + csg * dvgsteff_cv_dvb;
                csg *= dvgsteff_cv_dvg;
            } else if model.xpart < 0.5 {
                // 40/60 partition.
                let t1x = t1 / 12.0;
                let t2x = 0.5 * cox_wl / (t1x * t1x);
                let t3x = vgsteff_cv
                    * (2.0 * t0 * t0 / 3.0 + vgsteff_cv * (vgsteff_cv - 4.0 * t0 / 3.0))
                    - 2.0 * t0 * t0 * t0 / 15.0;
                qsrc = -t2x * t3x;
                let t7x = 4.0 / 3.0 * vgsteff_cv * (vgsteff_cv - t0) + 0.4 * t0 * t0;
                let t4x = -2.0 * qsrc / t1x
                    - t2x
                        * (vgsteff_cv * (3.0 * vgsteff_cv - 8.0 * t0 / 3.0) + 2.0 * t0 * t0 / 3.0);
                let t5x = (qsrc / t1x + t2x * t7x) * abulk_cv;
                let t6x = qsrc / t1x * vdseff_cv + t2x * t7x * vdseff_cv;
                csg = t4x + t5x * dvdseff_cv_dvg;
                csd = t5x * dvdseff_cv_dvd + csg * dvgsteff_cv_dvd;
                csb = t5x * dvdseff_cv_dvb + t6x * dabulk_cv_dvb + csg * dvgsteff_cv_dvb;
                csg *= dvgsteff_cv_dvg;
            } else {
                // 50/50 partition.
                qsrc = -0.5 * (qgate_l + qbulk_l);
                csg = -0.5 * (cgg1 + cbg1);
                csb = -0.5 * (cgb1 + cbb1);
                csd = -0.5 * (cgd1 + cbd1);
            }

            qgate_l += qac0 + qsub0;
            qbulk_l -= qac0 + qsub0;
            let qdrn_l = -(qgate_l + qbulk_l + qsrc);

            let cgg = dqac0_dvg + dqsub0_dvg + cgg1;
            let cgd = dqsub0_dvd + cgd1;
            let mut cgb = dqac0_dvb + dqsub0_dvb + cgb1;

            let cbg = cbg1 - dqac0_dvg - dqsub0_dvg;
            let cbd = cbd1 - dqsub0_dvd;
            let mut cbb = cbb1 - dqac0_dvb - dqsub0_dvb;

            cgb *= dvbseff_dvb;
            cbb *= dvbseff_dvb;
            csb *= dvbseff_dvb;

            ch.cggb = cgg;
            ch.cgsb = -(cgg + cgd + cgb);
            ch.cgdb = cgd;
            ch.cdgb = -(cgg + cbg + csg);
            ch.cdsb = cgg + cgd + cgb + cbg + cbd + cbb + csg + csd + csb;
            ch.cddb = -(cgd + cbd + csd);
            ch.cbgb = cbg;
            ch.cbsb = -(cbg + cbd + cbb);
            ch.cbdb = cbd;

            qgate = qgate_l;
            qbulk = qbulk_l;
            qdrn = qdrn_l;
        } else {
            // --- Charge-thickness capMod (CTM) begins (3632-3920) ---
            let v3 = inst.vfbzb - vgs_eff + vbseff_cv - DELTA_3;
            let t0 = if inst.vfbzb <= 0.0 {
                (v3 * v3 - 4.0 * DELTA_3 * inst.vfbzb).sqrt()
            } else {
                (v3 * v3 + 4.0 * DELTA_3 * inst.vfbzb).sqrt()
            };
            let t1 = 0.5 * (1.0 + v3 / t0);
            let vfbeff = inst.vfbzb - 0.5 * (v3 + t0);
            let dvfbeff_dvg = t1 * dvgs_eff_dvg;
            let dvfbeff_dvb = -t1 * dvbseff_cv_dvb;

            let cox = inst.coxp;
            let mut tox = 1.0e8 * inst.toxp;
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

            let link = 1.0e-3 * inst.toxp;
            let v3 = p.ldeb - tcen - link;
            let v4 = (v3 * v3 + 4.0 * link * p.ldeb).sqrt();
            tcen = p.ldeb - 0.5 * (v3 + v4);
            let t1 = 0.5 * (1.0 + v3 / v4);
            dtcen_dvg *= t1;
            dtcen_dvb *= t1;

            let ccen = epssub / tcen;
            let t2 = cox / (cox + ccen);
            let coxeff_acc = t2 * ccen;
            let t3 = -ccen / tcen;
            let mut dcoxeff_dvg = t2 * t2 * t3;
            let dcoxeff_dvb = dcoxeff_dvg * dtcen_dvb;
            dcoxeff_dvg *= dtcen_dvg;
            let mut cox_wlcen = cox_wl * coxeff_acc / mt.coxe;

            let qac0 = cox_wlcen * (vfbeff - inst.vfbzb);
            let qov_cox = qac0 / coxeff_acc;
            let dqac0_dvg = cox_wlcen * dvfbeff_dvg + qov_cox * dcoxeff_dvg;
            let dqac0_dvb = cox_wlcen * dvfbeff_dvb + qov_cox * dcoxeff_dvb;

            let t0 = 0.5 * p.k1ox;
            let t3 = vgs_eff - vfbeff - vbseff_cv - vgsteff_cv;
            let (t1, t2);
            if p.k1ox == 0.0 {
                t1 = 0.0;
                t2 = 0.0;
            } else if t3 < 0.0 {
                t1 = t0 + t3 / p.k1ox;
                t2 = cox_wlcen;
            } else {
                t1 = (t0 * t0 + t3).sqrt();
                t2 = cox_wlcen * t0 / t1;
            }

            let qsub0 = cox_wlcen * p.k1ox * (t1 - t0);
            let qov_cox = qsub0 / coxeff_acc;
            let dqsub0_dvg =
                t2 * (dvgs_eff_dvg - dvfbeff_dvg - dvgsteff_cv_dvg) + qov_cox * dcoxeff_dvg;
            let dqsub0_dvd = -t2 * dvgsteff_cv_dvd;
            let dqsub0_dvb =
                -t2 * (dvfbeff_dvb + dvbseff_cv_dvb + dvgsteff_cv_dvb) + qov_cox * dcoxeff_dvb;

            // Gate-bias dependent delta Phis.
            let (denomi_dp, t0);
            if p.k1ox <= 0.0 {
                denomi_dp = 0.25 * p.moin * vtm;
                t0 = 0.5 * p.sqrt_phi;
            } else {
                denomi_dp = p.moin * vtm * p.k1ox * p.k1ox;
                t0 = p.k1ox * p.sqrt_phi;
            }
            let t1 = 2.0 * t0 + vgsteff_cv;
            let delta_phi = vtm * (1.0 + t1 * vgsteff_cv / denomi_dp).ln();
            let ddelta_phi_dvg = 2.0 * vtm * (t1 - t0) / (denomi_dp + t1 * vgsteff_cv);
            // End of delta Phis.

            // VgDP = Vgsteff - DeltaPhi.
            let t0 = vgsteff_cv - delta_phi - 0.001;
            let dt0_dvg = 1.0 - ddelta_phi_dvg;
            let t1 = (t0 * t0 + vgsteff_cv * 0.004).sqrt();
            let vg_dp = 0.5 * (t0 + t1);
            let dvg_dp_dvg = 0.5 * (dt0_dvg + (t0 * dt0_dvg + 0.002) / t1);

            tox += tox; // Tcen reevaluated below due to different Vgsteff
            let t0 = (vgsteff_cv + inst.vtfbphi2) / tox;
            let tmp = (model.bdos * 0.7 * t0.ln()).exp();
            let t1 = 1.0 + tmp;
            let t2 = model.bdos * 0.7 * tmp / (t0 * tox);
            tcen = model.ados * 1.9e-9 / t1;
            dtcen_dvg = -tcen * t2 / t1;
            let dtcen_dvd = dtcen_dvg * dvgsteff_cv_dvd;
            dtcen_dvb = dtcen_dvg * dvgsteff_cv_dvb;
            dtcen_dvg *= dvgsteff_cv_dvg;

            let ccen = epssub / tcen;
            let t0 = cox / (cox + ccen);
            let coxeff_inv = t0 * ccen;
            let t1 = -ccen / tcen;
            dcoxeff_dvg = t0 * t0 * t1;
            let dcoxeff_dvd = dcoxeff_dvg * dtcen_dvd;
            let dcoxeff_dvb = dcoxeff_dvg * dtcen_dvb;
            dcoxeff_dvg *= dtcen_dvg;
            cox_wlcen = cox_wl * coxeff_inv / mt.coxe;

            let abulk_cv = abulk0_q * p.abulk_cv_factor;
            let dabulk_cv_dvb = p.abulk_cv_factor * dabulk0_q_dvb;
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
                dvdseff_cv_dvb = dt0_dvb * (t4 - t5) + t5 * dt1_dvb;
            }

            if vds_c == 0.0 {
                vdseff_cv = 0.0;
                dvdseff_cv_dvg = 0.0;
                dvdseff_cv_dvb = 0.0;
            }

            let t0 = abulk_cv * vdseff_cv;
            let t1 = vg_dp;
            let t2 = 12.0 * (t1 - 0.5 * t0 + 1.0e-20);
            let t3 = t0 / t2;
            let t4 = 1.0 - 12.0 * t3 * t3;
            let t5 = abulk_cv * (6.0 * t0 * (4.0 * t1 - t0) / (t2 * t2) - 0.5);
            let t6 = t5 * vdseff_cv / abulk_cv;

            let mut qgate_l = cox_wlcen * (t1 - t0 * (0.5 - t3));
            let qov_cox = qgate_l / coxeff_inv;
            let mut cgg1 = cox_wlcen * (t4 * dvg_dp_dvg + t5 * dvdseff_cv_dvg);
            let cgd1 =
                cox_wlcen * t5 * dvdseff_cv_dvd + cgg1 * dvgsteff_cv_dvd + qov_cox * dcoxeff_dvd;
            let cgb1 = cox_wlcen * (t5 * dvdseff_cv_dvb + t6 * dabulk_cv_dvb)
                + cgg1 * dvgsteff_cv_dvb
                + qov_cox * dcoxeff_dvb;
            cgg1 = cgg1 * dvgsteff_cv_dvg + qov_cox * dcoxeff_dvg;

            let t7 = 1.0 - abulk_cv;
            let t8 = t2 * t2;
            let t9 = 12.0 * t7 * t0 * t0 / (t8 * abulk_cv);
            let t10 = t9 * dvg_dp_dvg;
            let t11 = -t7 * t5 / abulk_cv;
            let t12 = -(t9 * t1 / abulk_cv + vdseff_cv * (0.5 - t0 / t2));

            let mut qbulk_l = cox_wlcen * t7 * (0.5 * vdseff_cv - t0 * vdseff_cv / t2);
            let qov_cox = qbulk_l / coxeff_inv;
            let mut cbg1 = cox_wlcen * (t10 + t11 * dvdseff_cv_dvg);
            let cbd1 =
                cox_wlcen * t11 * dvdseff_cv_dvd + cbg1 * dvgsteff_cv_dvd + qov_cox * dcoxeff_dvd;
            let cbb1 = cox_wlcen * (t11 * dvdseff_cv_dvb + t12 * dabulk_cv_dvb)
                + cbg1 * dvgsteff_cv_dvb
                + qov_cox * dcoxeff_dvb;
            cbg1 = cbg1 * dvgsteff_cv_dvg + qov_cox * dcoxeff_dvg;

            let (qsrc, mut csg, csd, csb);
            if model.xpart > 0.5 {
                // 0/100 partition.
                qsrc = -cox_wlcen * (t1 / 2.0 + t0 / 4.0 - 0.5 * t0 * t0 / t2);
                let qov_cox = qsrc / coxeff_inv;
                let t2x = t2 + t2;
                let t3x = t2x * t2x;
                let t7x = -(0.25 - 12.0 * t0 * (4.0 * t1 - t0) / t3x);
                let t4x = -(0.5 + 24.0 * t0 * t0 / t3x) * dvg_dp_dvg;
                let t5x = t7x * abulk_cv;
                let t6x = t7x * vdseff_cv;

                csg = cox_wlcen * (t4x + t5x * dvdseff_cv_dvg);
                csd = cox_wlcen * t5x * dvdseff_cv_dvd
                    + csg * dvgsteff_cv_dvd
                    + qov_cox * dcoxeff_dvd;
                csb = cox_wlcen * (t5x * dvdseff_cv_dvb + t6x * dabulk_cv_dvb)
                    + csg * dvgsteff_cv_dvb
                    + qov_cox * dcoxeff_dvb;
                csg = csg * dvgsteff_cv_dvg + qov_cox * dcoxeff_dvg;
            } else if model.xpart < 0.5 {
                // 40/60 partition.
                let t2x = t2 / 12.0;
                let t3x = 0.5 * cox_wlcen / (t2x * t2x);
                let t4x = t1 * (2.0 * t0 * t0 / 3.0 + t1 * (t1 - 4.0 * t0 / 3.0))
                    - 2.0 * t0 * t0 * t0 / 15.0;
                qsrc = -t3x * t4x;
                let qov_cox = qsrc / coxeff_inv;
                let t8x = 4.0 / 3.0 * t1 * (t1 - t0) + 0.4 * t0 * t0;
                let t5x = -2.0 * qsrc / t2x
                    - t3x * (t1 * (3.0 * t1 - 8.0 * t0 / 3.0) + 2.0 * t0 * t0 / 3.0);
                let t6x = abulk_cv * (qsrc / t2x + t3x * t8x);
                let t7x = t6x * vdseff_cv / abulk_cv;

                csg = t5x * dvg_dp_dvg + t6x * dvdseff_cv_dvg;
                csd = csg * dvgsteff_cv_dvd + t6x * dvdseff_cv_dvd + qov_cox * dcoxeff_dvd;
                csb = csg * dvgsteff_cv_dvb
                    + t6x * dvdseff_cv_dvb
                    + t7x * dabulk_cv_dvb
                    + qov_cox * dcoxeff_dvb;
                csg = csg * dvgsteff_cv_dvg + qov_cox * dcoxeff_dvg;
            } else {
                // 50/50 partition.
                qsrc = -0.5 * qgate_l;
                csg = -0.5 * cgg1;
                csd = -0.5 * cgd1;
                csb = -0.5 * cgb1;
            }

            qgate_l += qac0 + qsub0 - qbulk_l;
            qbulk_l -= qac0 + qsub0;
            let qdrn_l = -(qgate_l + qbulk_l + qsrc);

            let cbg = cbg1 - dqac0_dvg - dqsub0_dvg;
            let cbd = cbd1 - dqsub0_dvd;
            let cbb = cbb1 - dqac0_dvb - dqsub0_dvb;

            let cgg = cgg1 - cbg;
            let cgd = cgd1 - cbd;
            let cgb = cgb1 - cbb;

            let cgb = cgb * dvbseff_dvb;
            let cbb = cbb * dvbseff_dvb;
            let csb = csb * dvbseff_dvb;

            ch.cggb = cgg;
            ch.cgsb = -(cgg + cgd + cgb);
            ch.cgdb = cgd;
            ch.cdgb = -(cgg + cbg + csg);
            ch.cdsb = cgg + cgd + cgb + cbg + cbd + cbb + csg + csd + csb;
            ch.cddb = -(cgd + cbd + csd);
            ch.cbgb = cbg;
            ch.cbsb = -(cbg + cbd + cbb);
            ch.cbdb = cbd;

            qgate = qgate_l;
            qbulk = qbulk_l;
            qdrn = qdrn_l;
        } // End of CTM
    } // End of CAPMOD=0/1/2 intrinsic charge

    ch.csgb = -ch.cggb - ch.cdgb - ch.cbgb;
    ch.csdb = -ch.cgdb - ch.cddb - ch.cbdb;
    ch.cssb = -ch.cgsb - ch.cdsb - ch.cbsb;
    ch.cgbb = -ch.cgdb - ch.cggb - ch.cgsb;
    ch.cdbb = -ch.cddb - ch.cdgb - ch.cdsb;
    ch.cbbb = -ch.cbgb - ch.cbdb - ch.cbsb;
    ch.csbb = -ch.cgbb - ch.cdbb - ch.cbbb;
    ch.qgate = qgate;
    ch.qbulk = qbulk;
    ch.qdrn = qdrn;
    ch.qsrc = -(qgate + qbulk + qdrn);

    // --- Junction C-V (3978-4065) ---
    {
        let czbd = mt.d_unit_area_temp_jct_cap * inst.adeff; // bug fix
        let czbs = mt.s_unit_area_temp_jct_cap * inst.aseff;
        let czbdsw = mt.d_unit_length_sidewall_temp_jct_cap * inst.pdeff;
        let czbdswg = mt.d_unit_length_gate_sidewall_temp_jct_cap * p.weff_cj * nf;
        let czbssw = mt.s_unit_length_sidewall_temp_jct_cap * inst.pseff;
        let czbsswg = mt.s_unit_length_gate_sidewall_temp_jct_cap * p.weff_cj * nf;

        let mjs = mt.mjs;
        let mjsws = mt.mjsws;
        let mjswgs = mt.mjswgs;
        let mjd = mt.mjd;
        let mjswd = mt.mjswd;
        let mjswgd = mt.mjswgd;

        // Source/bulk junction.
        if vbs_jct == 0.0 {
            ch.qbs = 0.0;
            ch.capbs = czbs + czbssw + czbsswg;
        } else if vbs_jct < 0.0 {
            ch.qbs = 0.0;
            ch.capbs = 0.0;
            if czbs > 0.0 {
                let arg = 1.0 - vbs_jct / mt.phi_bs;
                let sarg = if mjs == 0.5 {
                    1.0 / arg.sqrt()
                } else {
                    (-mjs * arg.ln()).exp()
                };
                ch.qbs = mt.phi_bs * czbs * (1.0 - arg * sarg) / (1.0 - mjs);
                ch.capbs = czbs * sarg;
            }
            if czbssw > 0.0 {
                let arg = 1.0 - vbs_jct / mt.phi_bsws;
                let sarg = if mjsws == 0.5 {
                    1.0 / arg.sqrt()
                } else {
                    (-mjsws * arg.ln()).exp()
                };
                ch.qbs += mt.phi_bsws * czbssw * (1.0 - arg * sarg) / (1.0 - mjsws);
                ch.capbs += czbssw * sarg;
            }
            if czbsswg > 0.0 {
                let arg = 1.0 - vbs_jct / mt.phi_bswgs;
                let sarg = if mjswgs == 0.5 {
                    1.0 / arg.sqrt()
                } else {
                    (-mjswgs * arg.ln()).exp()
                };
                ch.qbs += mt.phi_bswgs * czbsswg * (1.0 - arg * sarg) / (1.0 - mjswgs);
                ch.capbs += czbsswg * sarg;
            }
        } else {
            let t0 = czbs + czbssw + czbsswg;
            let t1 = vbs_jct
                * (czbs * mjs / mt.phi_bs
                    + czbssw * mjsws / mt.phi_bsws
                    + czbsswg * mjswgs / mt.phi_bswgs);
            ch.qbs = vbs_jct * (t0 + 0.5 * t1);
            ch.capbs = t0 + t1;
        }

        // Drain/bulk junction.
        if vbd_jct == 0.0 {
            ch.qbd = 0.0;
            ch.capbd = czbd + czbdsw + czbdswg;
        } else if vbd_jct < 0.0 {
            ch.qbd = 0.0;
            ch.capbd = 0.0;
            if czbd > 0.0 {
                let arg = 1.0 - vbd_jct / mt.phi_bd;
                let sarg = if mjd == 0.5 {
                    1.0 / arg.sqrt()
                } else {
                    (-mjd * arg.ln()).exp()
                };
                ch.qbd = mt.phi_bd * czbd * (1.0 - arg * sarg) / (1.0 - mjd);
                ch.capbd = czbd * sarg;
            }
            if czbdsw > 0.0 {
                let arg = 1.0 - vbd_jct / mt.phi_bswd;
                let sarg = if mjswd == 0.5 {
                    1.0 / arg.sqrt()
                } else {
                    (-mjswd * arg.ln()).exp()
                };
                ch.qbd += mt.phi_bswd * czbdsw * (1.0 - arg * sarg) / (1.0 - mjswd);
                ch.capbd += czbdsw * sarg;
            }
            if czbdswg > 0.0 {
                let arg = 1.0 - vbd_jct / mt.phi_bswgd;
                let sarg = if mjswgd == 0.5 {
                    1.0 / arg.sqrt()
                } else {
                    (-mjswgd * arg.ln()).exp()
                };
                ch.qbd += mt.phi_bswgd * czbdswg * (1.0 - arg * sarg) / (1.0 - mjswgd);
                ch.capbd += czbdswg * sarg;
            }
        } else {
            let t0 = czbd + czbdsw + czbdswg;
            let t1 = vbd_jct
                * (czbd * mjd / mt.phi_bd
                    + czbdsw * mjswd / mt.phi_bswd
                    + czbdswg * mjswgd / mt.phi_bswgd);
            ch.qbd = vbd_jct * (t0 + 0.5 * t1);
            ch.capbd = t0 + t1;
        }
    }

    // --- Overlap charges (4138-4185; rgateMod=3 evaluates them from GM) ---
    let (vgdx, vgsx) = if model.rgate_mod == 3 {
        (vgmd, vgms)
    } else {
        (vgd, vgs)
    };
    let (mut cgdo, mut qgdo, mut cgso, mut qgso);
    if model.cap_mod == 0 {
        cgdo = p.cgdo;
        qgdo = p.cgdo * vgdx;
        cgso = p.cgso;
        qgso = p.cgso * vgsx;
    } else {
        // capMod == 1/2 smoothed overlap form.
        let t0 = vgdx + DELTA_1;
        let t1 = (t0 * t0 + 4.0 * DELTA_1).sqrt();
        let t2 = 0.5 * (t0 - t1);
        let t3 = p.weff_cv * p.cgdl;
        let t4 = (1.0 - 4.0 * t2 / p.ckappad).sqrt();
        cgdo = p.cgdo + t3 - t3 * (1.0 - 1.0 / t4) * (0.5 - 0.5 * t0 / t1);
        qgdo = (p.cgdo + t3) * vgdx - t3 * (t2 + 0.5 * p.ckappad * (t4 - 1.0));

        let t0 = vgsx + DELTA_1;
        let t1 = (t0 * t0 + 4.0 * DELTA_1).sqrt();
        let t2 = 0.5 * (t0 - t1);
        let t3 = p.weff_cv * p.cgsl;
        let t4 = (1.0 - 4.0 * t2 / p.ckappas).sqrt();
        cgso = p.cgso + t3 - t3 * (1.0 - 1.0 / t4) * (0.5 - 0.5 * t0 / t1);
        qgso = (p.cgso + t3) * vgsx - t3 * (t2 + 0.5 * p.ckappas * (t4 - 1.0));
    }
    if nf != 1.0 {
        cgdo *= nf;
        cgso *= nf;
        qgdo *= nf;
        qgso *= nf;
    }
    ch.cgdo = cgdo;
    ch.qgdo = qgdo;
    ch.cgso = cgso;
    ch.qgso = qgso;
    ch.cgbo = p.cgbo;
    ch.qchqs = -(qbulk + qgate);
    let cox_wl = mt.coxe * p.weff_cv * p.leff_cv * nf;
    ch.cox_wl = cox_wl;
    ch.gcrg = op.gcrg;
    ch.gcrgg = op.gcrgg;
    ch.gcrgd = op.gcrgd;
    ch.gcrgs = op.gcrgs;
    ch.gcrgb = op.gcrgb;
    ch.taunet = if (model.trnqs_mod != 0 || model.acnqs_mod != 0) && cox_wl > 0.0 && op.gcrg > 0.0 {
        cox_wl / op.gcrg
    } else {
        0.0
    };

    // --- Mode-dependent node-charge assembly (4188-4427; trnqsMod = 0,
    // rbodyMod = 0) ---
    if mode > 0 && model.rgate_mod == 3 {
        let qdrn_n = qdrn - qgdo;
        let qgmb = p.cgbo * vgmb;
        let qgmid = qgdo + qgso + qgmb;
        let qbulk_n = qbulk - qgmb;
        let qsrc_n = -(qgate + qgmid + qbulk_n + qdrn_n);
        ch.qg_node = qgate;
        ch.qgmid_node = qgmid;
        ch.qd_node = qdrn_n;
        ch.qb_node = qbulk_n;
        ch.qs_node = qsrc_n;
    } else if mode > 0 {
        let qdrn_n = qdrn - qgdo;
        let qgb = p.cgbo * vgb;
        let qgate_n = qgate + qgdo + qgso + qgb;
        let qbulk_n = qbulk - qgb;
        let qsrc_n = -(qgate_n + qbulk_n + qdrn_n);
        ch.qg_node = qgate_n;
        ch.qgmid_node = 0.0;
        ch.qd_node = qdrn_n;
        ch.qb_node = qbulk_n;
        ch.qs_node = qsrc_n;
    } else if model.rgate_mod == 3 {
        let qsrc_n = qdrn - qgso;
        let qgmb = p.cgbo * vgmb;
        let qgmid = qgdo + qgso + qgmb;
        let qbulk_n = qbulk - qgmb;
        let qdrn_n = -(qgate + qgmid + qbulk_n + qsrc_n);
        ch.qg_node = qgate;
        ch.qgmid_node = qgmid;
        ch.qd_node = qdrn_n;
        ch.qb_node = qbulk_n;
        ch.qs_node = qsrc_n;
    } else {
        let qsrc_n = qdrn - qgso;
        let qgb = p.cgbo * vgb;
        let qgate_n = qgate + qgdo + qgso + qgb;
        let qbulk_n = qbulk - qgb;
        let qdrn_n = -(qgate_n + qbulk_n + qsrc_n);
        ch.qg_node = qgate_n;
        ch.qgmid_node = 0.0;
        ch.qd_node = qdrn_n;
        ch.qb_node = qbulk_n;
        ch.qs_node = qsrc_n;
    }

    op.charge = Some(ch);
    Ok(op)
}
