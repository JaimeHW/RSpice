//! B3SOIDD (BSIMSOI dynamic-depletion, MOS level 56) device.
//!
//! Ported from ngspice-46 `src/spicelib/devices/bsim3soi_dd/`. This module ties
//! the model card ([`params`]), the size/temperature setup ([`temp`]), and the
//! DC load equations ([`eval`]) into a [`NonlinearDevice`] that RSpice can stamp.
//!
//! # Node topology (b3soiddset.c:975-1037)
//!
//! External terminals are drain `d`, gate `g`, source `s`, back-gate/substrate
//! `e` (substrate under the buried oxide), and an optional body contact `p`.
//! The device additionally owns an internal body node `B` and (in the full
//! model) optional drain/source primes and a self-heating temperature node.
//!
//! For the supported decks (`shmod = 0`):
//! - **Floating body** (`d g s e`): ngspice creates an internal `Body` node,
//!   sets `bodyMod = 0`, `float = 1`. RSpice allocates one internal node.
//! - **Ideal body tie** (`d g s e p`): ngspice sets `bodyMod = 2` and the
//!   external `p` node *is* the body node; no internal node is created.
//! - **Nonideal body tie** (`d g s e p`, nonzero `rbody`/`rbsh`): ngspice sets
//!   `bodyMod = 1`; RSpice allocates an internal body node and stamps the
//!   body-contact resistor between `B` and `P`.
//!
//! Positive `RSH * NRD/NRS` is lowered by the builder into ordinary linear
//! resistors between the external terminals and the drain/source primes. When
//! no such resistance exists, the primes coincide with the external terminals.
//! When `SHMOD=1` with positive `RTH0`, RSpice allocates the ngspice
//! self-heating temperature-rise node and stamps the thermal power row.
//!
//! # Status
//!
//! - **DC current path** ([`eval::eval_dc`]): faithful transcription of the
//!   `B3SOIDDload` DC block, including the back-gate (E) coupling columns.
//! - **CAPMOD=2/3 charge model** ([`eval::eval`] with `compute_charges`): the
//!   intrinsic + extrinsic + overlap charges and the coupled capacitance matrix
//!   (b3soiddld.c:2646-3784). Stamped as a transient companion by the engine's
//!   dedicated B3SOI pass; the electrical node charges and DD self-heating
//!   `qth` state feed the local-truncation-error step control.
//! - **Convergence aids**: `B3SOIDDlimit` (per-iterate 0.2 V body-voltage cap)
//!   and `B3SOIDDSmartVbs` (DC floating-body `Vbs >= 0` clamp), applied each
//!   Newton iterate; SmartVbs is disabled during transient time-stepping.
//! - **Builder dispatch** is live for LEVEL=56 NMOS/PMOS (see `engine/builder.rs`
//!   `build_b3soi_dd`); LEVEL=55/57 route to the FD/PD sibling builders.
//!
//! Verified: the DC sweeps (`t3`/`t4`/`t5`/`inv2`) match the checked-in ngspice
//! references; `ring51` (the 51-stage SOI ring oscillator) runs its full 50 ns
//! transient. The `RampVg2` floating-body `@m1[vbs]` trace matches at the DC
//! anchor (t=0, ~0.0917 V) and tracks the transient (the fast-edge body
//! amplitude is still being calibrated against ngspice's body LTE).

#![allow(clippy::too_many_arguments)]

pub use super::common;
pub use params::B3SoiDdModel;

pub mod eval;
pub mod params;
pub mod temp;

use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::{Value, circuit::NodeId};
use eval::{B3SoiDdBias, B3SoiDdOp, ModelConsts};
use std::borrow::Cow;
use std::sync::Arc;
use temp::{B3SoiDdGeometry, B3SoiDdSized};

/// Body-node configuration for one instance (b3soiddset.c node creation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyMode {
    /// Floating body: `B` is an internal node, `float = 1` (bodyMod 0).
    Floating,
    /// Ideal body tie: the external contact node is the body node (bodyMod 2).
    TiedIdeal,
    /// Nonideal body tie: an internal body node `B` sits behind the external
    /// body contact `P` through the BSIMSOI body-contact resistance (bodyMod 1).
    TiedResistive,
}

/// B3SOIDD device instance.
#[derive(Debug, Clone)]
pub struct B3SoiDd {
    pub name: String,
    /// +1 NMOS / -1 PMOS (model `mtype`).
    pub mtype: Value,

    // External / internal nodes (already resolved to NodeId by the builder).
    pub node_drain: NodeId,
    pub node_gate: NodeId,
    pub node_source: NodeId,
    /// Back-gate / substrate-under-BOX node `E`.
    pub node_e: NodeId,
    /// Body node: internal (floating) or the external contact (tied).
    pub node_body: NodeId,
    /// Body-contact node `P` (== `node_body` for ideal ties; unused when
    /// floating).
    pub node_p: NodeId,
    /// Self-heating temperature-rise node (`Temp`), or 0 when disabled.
    pub node_temp: NodeId,

    pub body_mode: BodyMode,

    /// Shared model card (one per `.model`).
    pub model: Arc<B3SoiDdModel>,
    /// Size/temperature-resolved parameters (one per instance geometry).
    pub sized: Arc<B3SoiDdSized>,
    /// Instance geometry retained for self-heating re-evaluation at
    /// `CKTtemp + delTemp`.
    geometry: B3SoiDdGeometry,
    base_temp_k: Value,
    /// Model scalars needed inside the load.
    consts: ModelConsts,
    /// Xyce-style BSIMSOI3 terminal GMIN used for body-source and gate-drain
    /// conductance branches.
    eval_gmin: Value,

    // Operating point (current iteration).
    op: B3SoiDdOp,
    // Branch voltages used at the last `update`, device polarity, pre-swap.
    bias: B3SoiDdBias,
    converged_ref: B3SoiDdBias,
    has_history: bool,
    /// Last accepted/limited `vbs` (device polarity) used as the limiter anchor
    /// for `B3SOIDDlimit`/`B3SOIDDSmartVbs` on the next Newton iterate.
    vbs_limit_anchor: Value,
    vbd_limit_anchor: Value,
    /// Last accepted/limited self-heating temperature rise used by
    /// `B3SOIDDlimit(delTemp, oldDelTemp, 5.0)`.
    del_temp_limit_anchor: Value,
    /// DC/operating-point mode: enables the `B3SOIDDSmartVbs` floating-body
    /// clamp (Vbs >= 0). Cleared during transient where the body may go
    /// negative. Set by the engine before each analysis phase.
    dc_mode: std::cell::Cell<bool>,
    /// Whether the limiter anchor has been seeded (first iterate uses the raw
    /// node solution).
    limit_anchor_valid: std::cell::Cell<bool>,
    /// Transient device-bypass tolerances `(reltol, current abstol, vntol)`,
    /// the ngspice `CKTreltol`/`CKTabstol`/`CKTvoltTol` triple. `None`
    /// disables bypass (DC operating point never bypasses).
    bypass_tolerances: std::cell::Cell<Option<(Value, Value, Value)>>,
    /// Bypass engaged for the current Newton iterate (ngspice `ByPass`): the
    /// device state (`bias`, `op`, mode, charge partition) is frozen and the
    /// stamps reuse the previous evaluation.
    bypass_active: std::cell::Cell<bool>,
    /// Set at the start of every timestep attempt (ngspice `MODEINITPRED`):
    /// the next `update` must perform a full evaluation so the bypass anchor
    /// always belongs to the current timestep.
    force_full_eval: std::cell::Cell<bool>,
    /// The previous full evaluation engaged the body limiter (ngspice
    /// `Check != 0`), which disqualifies the next iterate from bypassing.
    last_limited: std::cell::Cell<bool>,
    /// `DEBUG=-1` instance flag (ngspice `debugMod`): the charge state is
    /// still evaluated for probes, but `ChargeComputationNeeded` is forced to
    /// zero before the companion assembly, so the device contributes no
    /// dynamic charges to the matrix, RHS, or LTE - the transient runs
    /// quasi-statically.
    charges_suppressed: bool,
    /// Xyce-style explicit instance `IC=` constraints, loaded as internal MNA
    /// branch equations during the operating-point solve.
    instance_ic: super::common::B3SoiInstanceIc,
}

impl B3SoiDd {
    /// Build an instance from a model card and instance geometry.
    pub fn new(
        name: String,
        node_drain: NodeId,
        node_gate: NodeId,
        node_source: NodeId,
        node_e: NodeId,
        node_body: NodeId,
        node_p: NodeId,
        node_temp: NodeId,
        body_mode: BodyMode,
        model: Arc<B3SoiDdModel>,
        geom: B3SoiDdGeometry,
        temp_k: Value,
    ) -> Result<Self, String> {
        if model.sh_mod == 1 && geom.rth0 != 0.0 && node_temp == 0 {
            return Err(format!(
                "B3SOIDD '{name}': self-heating (SHMOD=1 with RTH0!=0) requires a temperature node"
            ));
        }
        let sized = Arc::new(B3SoiDdSized::new(&model, &geom, temp_k)?);
        let consts = ModelConsts {
            cap_mod: model.cap_mod,
            cox: model.cox,
            cbox: model.cbox,
            csi: model.csi,
            csieff: model.csieff,
            qsi: model.qsi,
            qsieff: model.qsieff,
            adice: model.adice,
            tox: model.tox,
            tsi: model.tsi,
            xj: model.xj,
            charge_q: super::common::CHARGE_Q,
            mob_mod: model.mob_mod,
            cboxt: model.cboxt,
            xpart: model.xpart,
            tt: model.tt,
            mjswg: model.body_jct_gate_side_grading_coeff,
            // ngspice clamps PhiBSWG model-wide to >= 0.1 (b3soiddtemp.c).
            phibswg: model.gate_sidewall_jct_potential.max(0.1),
            cjswg: model.unit_length_gate_sidewall_jct_cap,
            mtype: model.mtype,
        };
        Ok(Self {
            name,
            mtype: model.mtype,
            node_drain,
            node_gate,
            node_source,
            node_e,
            node_body,
            node_p,
            node_temp,
            body_mode,
            model,
            sized,
            geometry: geom,
            base_temp_k: temp_k,
            consts,
            eval_gmin: 0.0,
            op: B3SoiDdOp::default(),
            bias: B3SoiDdBias {
                vbs: 0.0,
                vgs: 0.0,
                vds: 0.0,
                ves: 0.0,
                vps: 0.0,
                del_temp: 0.0,
            },
            converged_ref: B3SoiDdBias {
                vbs: 0.0,
                vgs: 0.0,
                vds: 0.0,
                ves: 0.0,
                vps: 0.0,
                del_temp: 0.0,
            },
            has_history: false,
            vbs_limit_anchor: 0.0,
            vbd_limit_anchor: 0.0,
            del_temp_limit_anchor: 0.0,
            dc_mode: std::cell::Cell::new(true),
            limit_anchor_valid: std::cell::Cell::new(false),
            bypass_tolerances: std::cell::Cell::new(None),
            bypass_active: std::cell::Cell::new(false),
            force_full_eval: std::cell::Cell::new(true),
            last_limited: std::cell::Cell::new(false),
            charges_suppressed: false,
            instance_ic: super::common::B3SoiInstanceIc::new(),
        })
    }

    /// Select the analysis mode for the floating-body convergence aids.
    ///
    /// In DC/operating-point mode the `B3SOIDDSmartVbs` clamp (Vbs >= 0) is
    /// active; in transient it is disabled so a floating body may swing below
    /// the source potential. The per-iteration `B3SOIDDlimit` change cap applies
    /// in both modes (it only protects the Newton path, not the solution).
    pub fn set_dc_mode(&self, dc: bool) {
        if self.dc_mode.replace(dc) == dc {
            return;
        }
        // A mode switch invalidates the limiter anchor (different state vector).
        self.limit_anchor_valid.set(false);
        self.bypass_active.set(false);
        self.force_full_eval.set(true);
    }

    /// Enable the ngspice-style transient device bypass with the engine's
    /// `(reltol, current abstol, vntol)` triple, or disable it with `None`.
    ///
    /// Bypass is more than a speed optimization: the B3SOIDD charge partition
    /// (`dxpart` 0.4/0.6) and mode select are discontinuous at `vds = 0`, so a
    /// device parked at that boundary injects an `ag0`-amplified charge-current
    /// jump on every re-evaluation and Newton limit-cycles at any timestep.
    /// Freezing the evaluation once the branch voltages and predicted currents
    /// are stationary (b3soiddld.c:589-643) is how ngspice converges there.
    pub fn set_bypass_tolerances(&self, tolerances: Option<(Value, Value, Value)>) {
        self.bypass_tolerances.set(tolerances);
        self.bypass_active.set(false);
    }

    /// `DEBUG=-1` (ngspice `debugMod == -1`): evaluate charges for probes but
    /// contribute no dynamic charges to the matrix, RHS, or LTE.
    pub fn set_debug_mod(&mut self, debug_mod: i32) {
        self.charges_suppressed = debug_mod == -1;
    }

    pub fn set_instance_ic(&mut self, instance_ic: super::common::B3SoiInstanceIc) {
        self.instance_ic = instance_ic;
    }

    pub fn instance_ic(&self) -> &super::common::B3SoiInstanceIc {
        &self.instance_ic
    }

    pub fn resolve_instance_ic_branches(&mut self, num_nodes: NodeId) {
        self.instance_ic.resolve_branch_matrix_indices(num_nodes);
    }

    pub fn set_eval_gmin(&mut self, gmin: Value) {
        self.eval_gmin = if gmin.is_finite() && gmin > 0.0 {
            gmin
        } else {
            0.0
        };
    }

    pub fn eval_gmin(&self) -> Value {
        self.eval_gmin
    }

    /// Whether `DEBUG=-1` suppresses this device's charge contributions.
    pub fn charges_suppressed(&self) -> bool {
        self.charges_suppressed
    }

    pub fn self_heating_active(&self) -> bool {
        self.node_temp != 0 && self.sized.rth.is_finite() && self.sized.rth > 0.0
    }

    pub fn thermal_capacitance(&self) -> Value {
        if self.self_heating_active() {
            self.sized.cth.max(0.0)
        } else {
            0.0
        }
    }

    /// Mark the start of a new timestep attempt (ngspice `MODEINITPRED`): the
    /// next `update` must fully re-evaluate so bypass deltas are always
    /// measured against a state from the current timestep.
    pub fn begin_timestep_iteration(&self) {
        self.force_full_eval.set(true);
        self.bypass_active.set(false);
    }

    /// Re-linearize directly at the supplied static candidate solution.
    ///
    /// Regular Newton updates intentionally use the BSIMSOI body limiter.
    /// Residual and fallback validation probes must evaluate the true
    /// operating-point equations at the candidate voltage itself.
    pub(crate) fn update_static_linearization(&mut self, voltages: &[Value]) {
        self.converged_ref = self.bias;
        let bias = self.raw_branch_voltages(voltages);
        self.bias = bias;
        self.op = self.eval_op_for_bias(bias);
        self.has_history = true;
        self.vbs_limit_anchor = bias.vbs;
        self.vbd_limit_anchor = bias.vbs - bias.vds;
        self.del_temp_limit_anchor = bias.del_temp;
        self.limit_anchor_valid.set(true);
        self.bypass_active.set(false);
        self.force_full_eval.set(false);
        self.last_limited.set(false);
    }

    /// ngspice bypass predicate (b3soiddld.c:589-643): every branch-voltage
    /// delta against the previous iterate's state is inside the Newton
    /// tolerances and the linear current predictions `cdhat`/`cbhat` match the
    /// stored device currents. `bodyMod` 0/2 skips the `vps` voltage test
    /// exactly as ngspice does.
    fn bypass_check(&self, raw: B3SoiDdBias, reltol: Value, abstol: Value, vntol: Value) -> bool {
        let vtol = |new: Value, old: Value| reltol * new.abs().max(old.abs()) + vntol;
        let old = &self.bias;
        let delvbs = raw.vbs - old.vbs;
        let delvds = raw.vds - old.vds;
        let delvgs = raw.vgs - old.vgs;
        let delves = raw.ves - old.ves;
        let delvps = raw.vps - old.vps;
        let deldel_temp = raw.del_temp - old.del_temp;
        let vbd_new = raw.vbs - raw.vds;
        let vbd_old = old.vbs - old.vds;
        let delvbd = vbd_new - vbd_old;
        if delvbs.abs() >= vtol(raw.vbs, old.vbs)
            || delvbd.abs() >= vtol(vbd_new, vbd_old)
            || delvgs.abs() >= vtol(raw.vgs, old.vgs)
            || delves.abs() >= vtol(raw.ves, old.ves)
            || delvds.abs() >= vtol(raw.vds, old.vds)
            || (self.self_heating_active() && deldel_temp.abs() >= vtol(raw.del_temp, old.del_temp))
        {
            return false;
        }

        // Linear predictions with the stored linearization (b3soiddld.c:542-563).
        let op = &self.op;
        let cdhat = if op.mode >= 0 {
            op.cd
                + (op.gm - op.gjdg) * delvgs
                + (op.gds - op.gjdd) * delvds
                + (op.gmbs - op.gjdb) * delvbs
                + (op.gme - op.gjde) * delves
        } else {
            let delvgd = (raw.vgs - raw.vds) - (old.vgs - old.vds);
            let delved = (raw.ves - raw.vds) - (old.ves - old.vds);
            op.cd + (op.gm - op.gjdg) * delvgd - (op.gds - op.gjdd) * delvds
                + (op.gmbs - op.gjdb) * delvbd
                + (op.gme - op.gjde) * delved
        };
        let cbhat = op.cb
            + op.gbgs * delvgs
            + op.gbbs * delvbs
            + op.gbds * delvds
            + op.gbes * delves
            + op.gbps * delvps;
        (cdhat - op.cd).abs() < reltol * cdhat.abs().max(op.cd.abs()) + abstol
            && (cbhat - op.cb).abs() < reltol * cbhat.abs().max(op.cb.abs()) + abstol
    }

    /// Internal body node (used by the harness `@m1[vbs]` probe resolution).
    pub fn body_node(&self) -> NodeId {
        self.node_body
    }

    /// Terminal NodeIds in (drain, gate, source, e, body) order.
    pub fn charge_nodes(&self) -> (NodeId, NodeId, NodeId, NodeId, NodeId) {
        (
            self.node_drain,
            self.node_gate,
            self.node_source,
            self.node_e,
            self.node_body,
        )
    }

    /// Evaluate the CAPMOD=2/3 charge state at the given solution vector.
    ///
    /// Returns the electrical node charges, optional thermal charge, and the
    /// intrinsic+overlap capacitance matrix (pre-`ag0`). Used by the engine's
    /// transient charge companion.
    /// The bias the transient charge evaluation and its companion stamp use:
    /// the frozen iterate under bypass, otherwise the limited branch voltages.
    /// The companion's `ceqq*` linearization corrections must be formed from
    /// this same bias (ngspice rebuilds `vb` from the limited `vbs`,
    /// b3soiddld.c:676-688) - mixing raw node voltages with limited-bias
    /// charges injects `ag0`-amplified phantom currents whenever the body
    /// limiter engages.
    fn charge_bias(&self, voltages: &[Value]) -> B3SoiDdBias {
        if self.bypass_active.get() {
            self.bias
        } else {
            self.branch_voltages(voltages)
        }
    }

    pub fn charge_at(&self, voltages: &[Value]) -> eval::B3SoiDdCharge {
        // A bypassed iterate freezes the whole evaluation, charges included
        // (ngspice reuses the CKTstate charges verbatim under ByPass).
        let bias = self.charge_bias(voltages);
        let sized = self.sized_for_bias(bias);
        let mut charge = eval::eval(&sized, &self.consts, bias, self.mtype, true)
            .charge
            .expect("compute_charges=true yields a charge state");
        if self.self_heating_active() {
            self.fill_charge_thermal_derivatives(&mut charge, bias);
        }
        charge.qth = self.thermal_capacitance() * bias.del_temp;
        charge
    }

    /// Operating-point snapshot for the OP report: `(id, vgs, vds, vbs, vth,
    /// vdsat, gm, gds, gmbs, region)`, in device polarity.
    pub fn op_values(
        &self,
    ) -> (
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
        &'static str,
    ) {
        let op = &self.op;
        let bias = self.bias;
        let (vgs_mode, vds_mode) = if op.mode >= 0 {
            (bias.vgs, bias.vds)
        } else {
            (bias.vgs - bias.vds, -bias.vds)
        };
        let region = if vgs_mode < op.von {
            "subthreshold"
        } else if vds_mode > op.vdsat {
            "saturation"
        } else {
            "linear"
        };
        (
            op.ids, bias.vgs, bias.vds, bias.vbs, op.von, op.vdsat, op.gm, op.gds, op.gmbs, region,
        )
    }

    /// Stamp the transient charge companion into the matrix and RHS.
    ///
    /// `ag0` is the integration coefficient (`d/dt` operator gain), and the
    /// `cq*` are the integrated charge-current histories from the engine
    /// (`cq = ag0*q + history`). This mirrors the charge portion of
    /// `B3SOIDDload` (b3soiddld.c:3679-3868, 4083-4128) for `bodyMod` 0/1/2 and
    /// the self-heating temperature node. Any drain/source series R has already
    /// been externalized into the prime nodes. The gate-overlap and
    /// extrinsic-substrate derivatives are already folded into `charge`'s
    /// `gc**` matrix.
    pub fn stamp_charge_companion(
        &self,
        charge: &eval::B3SoiDdCharge,
        ag0: Value,
        cqg: Value,
        cqb: Value,
        cqd: Value,
        cqe: Value,
        cqth: Value,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
    ) {
        let (dp, g, sp, e, b) = self.charge_nodes();
        // Branch voltages are device-polarity (type-folded) and LIMITED,
        // exactly as ngspice forms them from the folded vgs/vbs/vds/ves state
        // (with `vb` rebuilt from the limited `vbs`) before the type<0 sign
        // flip of the ceqq* terms (b3soiddld.c:495-500, 676-688, 834-836,
        // 4000-4009). Raw node differences here would flip the sign of the
        // G*v linearization correction for every p-channel device and inject
        // ag0-amplified phantom currents whenever the body limiter engages.
        let bias = self.charge_bias(voltages);
        let vgb = bias.vgs - bias.vbs;
        let vbd = bias.vbs - bias.vds;
        let vbs = bias.vbs;
        let veb = bias.ves - bias.vbs;

        // gc** are multiplied by ag0 (b3soiddld.c:3680-3766).
        let c = charge;
        let gcggb = c.gcggb * ag0;
        let gcgdb = c.gcgdb * ag0;
        let gcgsb = c.gcgsb * ag0;
        let gcgeb = c.gcgeb * ag0;
        let gcbgb = c.gcbgb * ag0;
        let gcbdb = c.gcbdb * ag0;
        let gcbsb = c.gcbsb * ag0;
        let gcbeb = c.gcbeb * ag0;
        let gcdgb = c.gcdgb * ag0;
        let gcddb = c.gcddb * ag0;
        let gcdsb = c.gcdsb * ag0;
        let gcdeb = c.gcdeb * ag0;
        let gcsgb = c.gcsgb * ag0;
        let gcsdb = c.gcsdb * ag0;
        let gcssb = c.gcssb * ag0;
        let gcseb = c.gcseb * ag0;
        let gcegb = c.gcegb * ag0;
        let gcedb = c.gcedb * ag0;
        let gcesb = c.gcesb * ag0;
        let gceeb = c.gceeb * ag0;
        let gcg_t = c.gcg_t * ag0;
        let gcb_t = c.gcb_t * ag0;
        let gcd_t = c.gcd_t * ag0;
        let gce_t = c.gce_t * ag0;
        let gcs_t = -(gcg_t + gcb_t + gcd_t + gce_t);

        // Equivalent charge currents (b3soiddld.c:3860-3867). type<0 flips sign.
        let mut ceqqg =
            cqg - gcggb * vgb + gcgdb * vbd + gcgsb * vbs - gcgeb * veb - gcg_t * bias.del_temp;
        let mut ceqqb =
            cqb - gcbgb * vgb + gcbdb * vbd + gcbsb * vbs - gcbeb * veb - gcb_t * bias.del_temp;
        let mut ceqqd =
            cqd - gcdgb * vgb + gcddb * vbd + gcdsb * vbs - gcdeb * veb - gcd_t * bias.del_temp;
        let mut ceqqe =
            cqe - gcegb * vgb + gcedb * vbd + gcesb * vbs - gceeb * veb - gce_t * bias.del_temp;
        if self.mtype < 0.0 {
            ceqqg = -ceqqg;
            ceqqb = -ceqqb;
            ceqqd = -ceqqd;
            ceqqe = -ceqqe;
        }
        let gc_tt = self.thermal_capacitance() * ag0;
        let ceqqth = cqth - gc_tt * bias.del_temp;

        // RHS (b3soiddld.c:4011-4017, charge parts).
        stamp_rhs(matrix, b, -ceqqb);
        stamp_rhs(matrix, g, -ceqqg);
        stamp_rhs(matrix, dp, -ceqqd);
        stamp_rhs(matrix, sp, ceqqg + ceqqb + ceqqd + ceqqe);
        stamp_rhs(matrix, e, -ceqqe);
        stamp_rhs(matrix, self.node_temp, -ceqqth);

        // Matrix charge entries (b3soiddld.c:4083-4128, gc** parts only).
        // Thermal row.
        stamp(matrix, self.node_temp, self.node_temp, gc_tt);
        // E row.
        stamp(matrix, e, g, gcegb);
        stamp(matrix, e, dp, gcedb);
        stamp(matrix, e, sp, gcesb);
        stamp(matrix, e, b, -(gcegb + gcedb + gcesb + gceeb));
        stamp(matrix, e, e, gceeb);
        stamp(matrix, e, self.node_temp, gce_t);
        // G row.
        stamp(matrix, g, e, gcgeb);
        stamp(matrix, g, b, -(gcggb + gcgdb + gcgsb + gcgeb));
        stamp(matrix, g, g, gcggb);
        stamp(matrix, g, dp, gcgdb);
        stamp(matrix, g, sp, gcgsb);
        stamp(matrix, g, self.node_temp, gcg_t);
        // DP row.
        stamp(matrix, dp, e, gcdeb);
        stamp(matrix, dp, b, -(gcdgb + gcddb + gcdeb + gcdsb));
        stamp(matrix, dp, g, gcdgb);
        stamp(matrix, dp, dp, gcddb);
        stamp(matrix, dp, sp, gcdsb);
        stamp(matrix, dp, self.node_temp, gcd_t);
        // SP row.
        stamp(matrix, sp, e, gcseb);
        stamp(matrix, sp, b, -(gcsgb + gcsdb + gcseb + gcssb));
        stamp(matrix, sp, g, gcsgb);
        stamp(matrix, sp, dp, gcsdb);
        stamp(matrix, sp, sp, gcssb);
        stamp(matrix, sp, self.node_temp, gcs_t);
        // B row.
        stamp(matrix, b, e, gcbeb);
        stamp(matrix, b, g, gcbgb);
        stamp(matrix, b, dp, gcbdb);
        stamp(matrix, b, sp, gcbsb);
        stamp(matrix, b, b, -(gcbgb + gcbdb + gcbsb + gcbeb));
        stamp(matrix, b, self.node_temp, gcb_t);
    }

    /// Extract device-polarity branch voltages from the solution vector
    /// (b3soiddld.c:483-498). Source-referenced, `mtype` folded in.
    ///
    /// The solution vector is 0-indexed (node 1 -> `v[0]`); ground (NodeId 0)
    /// reads as 0.0.
    fn raw_branch_voltages(&self, v: &[Value]) -> B3SoiDdBias {
        let node = |n: NodeId| if n == 0 { 0.0 } else { v[n - 1] };
        let vd = node(self.node_drain);
        let vg = node(self.node_gate);
        let vs = node(self.node_source);
        let ve = node(self.node_e);
        let vb = node(self.node_body);
        let vp = node(self.node_p);
        let vt = node(self.node_temp);
        B3SoiDdBias {
            vbs: self.mtype * (vb - vs),
            vgs: self.mtype * (vg - vs),
            vds: self.mtype * (vd - vs),
            ves: self.mtype * (ve - vs),
            vps: self.mtype * (vp - vs),
            del_temp: vt,
        }
    }

    fn branch_voltages(&self, v: &[Value]) -> B3SoiDdBias {
        let mut bias = self.raw_branch_voltages(v);
        let _ = self.apply_body_limiting(&mut bias);
        bias
    }

    fn sized_for_bias(&self, bias: B3SoiDdBias) -> Cow<'_, B3SoiDdSized> {
        if !self.self_heating_active() {
            return Cow::Borrowed(self.sized.as_ref());
        }

        let temp = self.base_temp_k + bias.del_temp;
        if (temp - self.sized.temp).abs() <= f64::EPSILON * self.sized.temp.abs().max(1.0) {
            return Cow::Borrowed(self.sized.as_ref());
        }

        Cow::Owned(
            B3SoiDdSized::new(&self.model, &self.geometry, temp)
                .expect("self-heated B3SOIDD temperature evaluation"),
        )
    }

    fn eval_op_for_bias(&self, bias: B3SoiDdBias) -> B3SoiDdOp {
        let sized = self.sized_for_bias(bias);
        let mut op = eval::eval_dc(&sized, &self.consts, bias, self.mtype);
        if self.self_heating_active() {
            self.fill_electrothermal_derivatives(&mut op, bias);
        }
        op
    }

    fn fill_electrothermal_derivatives(&self, op: &mut B3SoiDdOp, bias: B3SoiDdBias) {
        let temp = self.base_temp_k + bias.del_temp;
        let h = (temp.abs().max(1.0) * 1.0e-5).clamp(1.0e-3, 5.0e-2);
        let lower_h = if temp - h > 1.0 { h } else { 0.0 };

        let sample = |del_temp: Value| {
            let sample_bias = B3SoiDdBias { del_temp, ..bias };
            let sized = B3SoiDdSized::new(&self.model, &self.geometry, self.base_temp_k + del_temp)
                .expect("self-heated B3SOIDD derivative temperature evaluation");
            let sample_op = eval::eval_dc(&sized, &self.consts, sample_bias, self.mtype);
            let drain_junction = sample_op.cjd
                + sample_op.gjdb * sample_bias.vbs
                + sample_op.gjdd * sample_bias.vds
                + sample_op.gjdg * sample_bias.vgs
                + sample_op.gjde * sample_bias.ves
                + 0.5 * sized.min_isub;
            let source_junction = sample_op.cjs
                + sample_op.gjsb * sample_bias.vbs
                + sample_op.gjsd * sample_bias.vds
                + sample_op.gjsg * sample_bias.vgs
                + 0.5 * sized.min_isub;
            let body_current = sample_op.cbody
                + sample_op.gbbs * sample_bias.vbs
                + sample_op.gbgs * sample_bias.vgs
                + sample_op.gbds * sample_bias.vds
                + sample_op.gbes * sample_bias.ves
                + sample_op.gbps * sample_bias.vps
                - sized.min_isub;
            (
                sample_op.cdrain,
                drain_junction,
                source_junction,
                body_current,
                sample_op.ids,
            )
        };

        let center = sample(bias.del_temp);
        let plus = sample(bias.del_temp + h);
        let minus = if lower_h > 0.0 {
            sample(bias.del_temp - lower_h)
        } else {
            center
        };
        let denom = h + lower_h;
        op.gm_t = (plus.0 - minus.0) / denom;
        op.gjd_t = (plus.1 - minus.1) / denom;
        op.gjs_t = (plus.2 - minus.2) / denom;
        op.gb_t = (plus.3 - minus.3) / denom;
        let ids_t = (plus.4 - minus.4) / denom;
        op.gtemp_t = -ids_t * bias.vds;
        op.thermal_eq_current = -op.ids * bias.vds
            - self.mtype
                * (op.gtemp_g * bias.vgs
                    + op.gtemp_b * bias.vbs
                    + op.gtemp_e * bias.ves
                    + op.gtemp_d * bias.vds)
            - op.gtemp_t * bias.del_temp;
    }

    fn fill_charge_thermal_derivatives(&self, charge: &mut eval::B3SoiDdCharge, bias: B3SoiDdBias) {
        let temp = self.base_temp_k + bias.del_temp;
        let h = (temp.abs().max(1.0) * 1.0e-5).clamp(1.0e-3, 5.0e-2);
        let lower_h = if temp - h > 1.0 { h } else { 0.0 };

        let sample = |del_temp: Value| {
            let sample_bias = B3SoiDdBias { del_temp, ..bias };
            let sized = B3SoiDdSized::new(&self.model, &self.geometry, self.base_temp_k + del_temp)
                .expect("self-heated B3SOIDD charge derivative temperature evaluation");
            eval::eval(&sized, &self.consts, sample_bias, self.mtype, true)
                .charge
                .expect("compute_charges=true yields a charge state")
        };

        let center = sample(bias.del_temp);
        let plus = sample(bias.del_temp + h);
        let minus = if lower_h > 0.0 {
            sample(bias.del_temp - lower_h)
        } else {
            center
        };
        let denom = h + lower_h;
        charge.gcg_t = (plus.qg - minus.qg) / denom;
        charge.gcb_t = (plus.qb - minus.qb) / denom;
        charge.gcd_t = (plus.qd - minus.qd) / denom;
        charge.gce_t = (plus.qe - minus.qe) / denom;
    }

    /// Floating-body convergence aids `B3SOIDDlimit` + `B3SOIDDSmartVbs`
    /// (b3soiddld.c:50-99, 664-688), applied per Newton iterate.
    ///
    /// In the mode-selected (normal/inverse) frame, the body-source (or
    /// body-drain) voltage is clamped to move at most 0.2 V from the previous
    /// iterate's value and, in DC, floored at 0 for a floating body. This only
    /// reshapes the Newton path; the converged solution still satisfies KCL.
    /// Returns whether the per-iteration change cap actually engaged (the
    /// ngspice `Check` flag; the SmartVbs DC floor intentionally does not set
    /// it, matching `B3SOIDDSmartVbs`'s `NG_IGNORE(check)`).
    fn apply_body_limiting(&self, bias: &mut B3SoiDdBias) -> bool {
        if !self.limit_anchor_valid.get() {
            // First iterate of a phase: accept the raw bias, but still apply the
            // DC SmartVbs floor so a floating body never starts negative.
            if self.dc_mode.get() && self.body_mode == BodyMode::Floating {
                if bias.vds >= 0.0 {
                    if bias.vbs < 0.0 {
                        bias.vbs = 0.0;
                    }
                } else {
                    let mut vbd = bias.vbs - bias.vds;
                    if vbd < 0.0 {
                        vbd = 0.0;
                        bias.vbs = vbd + bias.vds;
                    }
                }
            }
            return false;
        }
        let mut check = false;
        let smart = self.dc_mode.get() && self.body_mode == BodyMode::Floating;
        if bias.vds >= 0.0 {
            let mut vbs = common::soi_limit(bias.vbs, self.vbs_limit_anchor, 0.2, &mut check);
            if smart && vbs < 0.0 {
                vbs = 0.0;
            }
            bias.vbs = vbs;
        } else {
            let vbd0 = bias.vbs - bias.vds;
            let mut vbd = common::soi_limit(vbd0, self.vbd_limit_anchor, 0.2, &mut check);
            if smart && vbd < 0.0 {
                vbd = 0.0;
            }
            bias.vbs = vbd + bias.vds;
        }
        if self.self_heating_active() {
            bias.del_temp =
                common::soi_limit(bias.del_temp, self.del_temp_limit_anchor, 5.0, &mut check);
        }
        check
    }
}

impl NonlinearDevice for B3SoiDd {
    fn update(&mut self, voltages: &[Value]) {
        self.converged_ref = self.bias;
        // ngspice transient bypass (b3soiddld.c:589-643): when the previous
        // iterate evaluated without limiting and the new branch voltages plus
        // predicted currents are stationary within tolerances, freeze the
        // evaluation (bias, op, mode, charge partition). This is what lets
        // Newton contract on a device parked at the discontinuous vds = 0
        // mode/charge-partition boundary.
        if let Some((reltol, abstol, vntol)) = self.bypass_tolerances.get()
            && !self.force_full_eval.get()
            && !self.last_limited.get()
            && self.has_history
        {
            let raw = self.raw_branch_voltages(voltages);
            if self.bypass_check(raw, reltol, abstol, vntol) {
                self.bypass_active.set(true);
                return;
            }
        }
        self.bypass_active.set(false);
        self.force_full_eval.set(false);
        let mut bias = self.raw_branch_voltages(voltages);
        let limited = self.apply_body_limiting(&mut bias);
        self.last_limited.set(limited);
        self.bias = bias;
        self.op = self.eval_op_for_bias(bias);
        self.has_history = true;
        // Anchor the per-iteration limiter at this (limited) iterate, in the
        // mode-selected frame, for the next Newton step.
        self.vbs_limit_anchor = bias.vbs;
        self.vbd_limit_anchor = bias.vbs - bias.vds;
        self.del_temp_limit_anchor = bias.del_temp;
        self.limit_anchor_valid.set(true);
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        if self.bypass_active.get() {
            // Bypassed iterate: restamp the frozen linearization unchanged.
            self.stamp_op(&self.op, self.bias, matrix);
            self.stamp_instance_ic(matrix);
            return;
        }
        let bias = self.branch_voltages(voltages);
        let op = if biases_match(bias, self.bias) {
            self.op.clone()
        } else {
            self.eval_op_for_bias(bias)
        };
        // `cdreq`/`ceq*` must be formed from the *same* bias that produced `op`,
        // not the (possibly stale) `self.bias` cached at the last `update`.
        self.stamp_op(&op, bias, matrix);
        self.stamp_instance_ic(matrix);
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        if !self.has_history {
            return false;
        }
        let reltol = criteria.relative_tolerance();
        let vtol = criteria.voltage_tolerance();
        let cmp =
            |new: Value, old: Value| (new - old).abs() < reltol * new.abs().max(old.abs()) + vtol;
        cmp(self.bias.vbs, self.converged_ref.vbs)
            && cmp(self.bias.vgs, self.converged_ref.vgs)
            && cmp(self.bias.vds, self.converged_ref.vds)
            && cmp(self.bias.ves, self.converged_ref.ves)
            && cmp(self.bias.vps, self.converged_ref.vps)
            && (!self.self_heating_active() || cmp(self.bias.del_temp, self.converged_ref.del_temp))
    }
}

impl B3SoiDd {
    fn stamp_instance_ic(&self, matrix: &mut impl MatrixStamper) {
        self.instance_ic.stamp(self.dc_mode.get(), matrix);
    }

    /// Stamp the linearized DC operating point.
    ///
    /// Faithful transcription of the DC portion of the B3SOIDD matrix/RHS load
    /// (b3soiddld.c:3886-4150) for `bodyMod` 0/2 and
    /// `ChargeComputationNeeded == 0` (so all `gc*`/`ceqq*` charge terms vanish;
    /// the self-heating row is stamped when its temperature node exists).
    /// `m` (multiplier) is 1. Xyce's BSIMSOI3 terminal-GMIN branches are
    /// stamped explicitly; they are not equivalent to the solver's nodal
    /// conditioning shunt.
    ///
    /// The device's drain/source node ids are already the prime nodes; they
    /// equal the external terminals only when no builder-lowered series
    /// resistance exists. `bNode` is the body node (internal floating or
    /// external tie). For `bodyMod=1`, `pNode` is the external body contact and
    /// the body-contact resistor is stamped after the intrinsic compact model.
    fn stamp_op(&self, op: &B3SoiDdOp, bias: B3SoiDdBias, matrix: &mut impl MatrixStamper) {
        let (dp, g, sp, e, b) = (
            self.node_drain,
            self.node_gate,
            self.node_source,
            self.node_e,
            self.node_body,
        );
        let mt = self.mtype;

        // The branch voltages used to form cdreq, in the *un-swapped* (raw)
        // frame, exactly as ngspice (vds/vgs/vbs/ves are pre-swap, device
        // polarity) and consistent with the `op` passed in.

        // ----- conductance groups (b3soiddld.c:3888-3944) -----
        let (gm, gmbs, gme, fwd_sum, rev_sum, cdreq, ceqbs, ceqbd);
        let (gm_t, gddp_t, gssp_t, gbb_t);
        let (gbbg, gbbdp, gbbb, gbbe, gbbsp);
        let (gddpg, gddpdp, gddpb, gddpe, gddpsp);
        let (gsspg, gsspdp, gsspb, gsspe, gsspsp);
        if op.mode >= 0 {
            gm = op.gm;
            gmbs = op.gmbs;
            gme = op.gme;
            gm_t = mt * op.gm_t;
            fwd_sum = gm + gmbs + gme;
            rev_sum = 0.0;
            cdreq = mt
                * (op.cdrain
                    - op.gds * bias.vds
                    - gm * bias.vgs
                    - gmbs * bias.vbs
                    - gme * bias.ves
                    - gm_t * bias.del_temp);
            ceqbs = op.cjs - op.gjs_t * bias.del_temp;
            ceqbd = op.cjd - op.gjd_t * bias.del_temp;

            gbbg = -op.gbgs;
            gbbdp = -op.gbds;
            gbbb = -op.gbbs;
            gbbe = -op.gbes;
            let gbbp = -op.gbps;
            gbb_t = -mt * op.gb_t;
            gbbsp = -(gbbg + gbbdp + gbbb + gbbe + gbbp);

            gddpg = -op.gjdg;
            gddpdp = -op.gjdd;
            gddpb = -op.gjdb;
            gddpe = -op.gjde;
            gddp_t = -mt * op.gjd_t;
            gddpsp = -(gddpg + gddpdp + gddpb + gddpe);

            gsspg = -op.gjsg;
            gsspdp = -op.gjsd;
            gsspb = -op.gjsb;
            gsspe = 0.0;
            gssp_t = -mt * op.gjs_t;
            gsspsp = -(gsspg + gsspdp + gsspb + gsspe);
        } else {
            gm = -op.gm;
            gmbs = -op.gmbs;
            gme = -op.gme;
            gm_t = -mt * op.gm_t;
            fwd_sum = 0.0;
            rev_sum = -(gm + gmbs + gme);
            let vgd = bias.vgs - bias.vds;
            let vbd = bias.vbs - bias.vds;
            cdreq = -mt
                * (op.cdrain
                    + op.gds * bias.vds
                    + gm * vgd
                    + gmbs * vbd
                    + gme * (bias.ves - bias.vds)
                    + gm_t * bias.del_temp);
            ceqbs = op.cjd - op.gjd_t * bias.del_temp;
            ceqbd = op.cjs - op.gjs_t * bias.del_temp;

            gbbg = -op.gbgs;
            gbbb = -op.gbbs;
            gbbe = -op.gbes;
            let gbbp = -op.gbps;
            gbb_t = -mt * op.gb_t;
            gbbsp = -op.gbds;
            gbbdp = -(gbbg + gbbsp + gbbb + gbbe + gbbp);

            gddpg = -op.gjsg;
            gddpsp = -op.gjsd;
            gddpb = -op.gjsb;
            gddpe = 0.0;
            gddp_t = -mt * op.gjs_t;
            gddpdp = -(gddpg + gddpsp + gddpb + gddpe);

            gsspg = -op.gjdg;
            gsspdp = -op.gjdd;
            gsspb = -op.gjdb;
            gsspe = -op.gjde;
            gssp_t = -mt * op.gjd_t;
            gsspsp = -(gsspg + gsspdp + gsspb + gsspe);
        }

        // type<0: flip junction/body equivalent currents (b3soiddld.c:3997)
        let ceqbody = -(op.cbody - op.gb_t * bias.del_temp);
        let (ceqbs, ceqbd, ceqbody) = if mt < 0.0 {
            (-ceqbs, -ceqbd, -ceqbody)
        } else {
            (ceqbs, ceqbd, ceqbody)
        };

        // ----- RHS (b3soiddld.c:4011-4017) -----
        // Routed through the stamper's `stamp_rhs` so the 1-indexed NodeId is
        // mapped to the 0-indexed RHS vector consistently with the matrix stamp.
        stamp_rhs(matrix, b, -ceqbody);
        stamp_rhs(matrix, dp, ceqbd - cdreq);
        stamp_rhs(matrix, sp, cdreq + ceqbs);
        self.stamp_thermal_rhs(op, matrix);

        stamp_conductance(matrix, b, sp, self.eval_gmin);
        stamp_conductance(matrix, g, dp, self.eval_gmin);

        // ----- matrix (b3soiddld.c:4090-4128, DC: gc*=0) -----
        // E row: only EePtr (gceeb==0) -> nothing in DC.
        // DP/SP body columns (b3soiddld.c:4092-4093):
        stamp(matrix, dp, b, -(-gddpb - gmbs));
        stamp(matrix, sp, b, -(-gsspb + gmbs));
        // Body row (b3soiddld.c:4094-4098):
        stamp(matrix, b, e, gbbe);
        stamp(matrix, b, g, gbbg);
        stamp(matrix, b, dp, gbbdp);
        stamp(matrix, b, sp, gbbsp);
        stamp(matrix, b, b, gbbb);

        // Gate row: gc* all zero in DC -> nothing.

        // Drain-prime row (b3soiddld.c:4106-4112):
        stamp(matrix, dp, g, gm + gddpg);
        stamp(matrix, dp, dp, op.gds + gddpdp + rev_sum);
        stamp(matrix, dp, sp, -(-gddpsp + op.gds + fwd_sum));

        // Source-prime row (b3soiddld.c:4114-4119):
        stamp(matrix, sp, g, -gm + gsspg);
        stamp(matrix, sp, dp, -(op.gds - gsspdp + rev_sum));
        stamp(matrix, sp, sp, op.gds + gsspsp + fwd_sum);

        // Back-gate (E) coupling columns of the drain/source-prime rows
        // (b3soiddld.c:4087-4088, DC: gc*=0). `gme` carries the mode sign.
        stamp(matrix, dp, e, gme + gddpe);
        stamp(matrix, sp, e, gsspe - gme);
        if self.self_heating_active() {
            let t = self.node_temp;
            stamp(matrix, dp, t, gm_t + gddp_t);
            stamp(matrix, sp, t, -gm_t + gssp_t);
            stamp(matrix, b, t, gbb_t);
        }
        self.stamp_thermal_matrix(op, matrix);

        // Body-tie resistor (bodyMod 1): a linear conductance between the
        // internal body node `B` and the external contact `P`, matching Xyce's
        // Ibp = Vbp / (Rbody + Rbodyext) with the BSIMSOI 1 mOhm guard.
        if self.body_mode == BodyMode::TiedResistive {
            stamp_conductance(matrix, b, self.node_p, self.body_tie_conductance());
        }
    }

    fn body_tie_conductance(&self) -> Value {
        let rbody = self.sized.rbody;
        let rbodyext = self.sized.rbodyext;
        let resistance = if rbody < 1.0e-3 {
            rbodyext.max(1.0e-3)
        } else {
            rbody + rbodyext
        };
        1.0 / resistance
    }

    fn stamp_thermal_rhs(&self, op: &B3SoiDdOp, matrix: &mut impl MatrixStamper) {
        if self.self_heating_active() {
            stamp_rhs(matrix, self.node_temp, -op.thermal_eq_current);
        }
    }

    fn stamp_thermal_matrix(&self, op: &B3SoiDdOp, matrix: &mut impl MatrixStamper) {
        if !self.self_heating_active() {
            return;
        }

        let t = self.node_temp;
        let (dp, sp) = (self.node_drain, self.node_source);
        let g = self.node_gate;
        let b = self.node_body;
        let e = self.node_e;

        let (gtemp_dp, gtemp_sp) = if op.mode >= 0 {
            (
                op.gtemp_d,
                -(op.gtemp_g + op.gtemp_b + op.gtemp_e + op.gtemp_d),
            )
        } else {
            (
                -(op.gtemp_g + op.gtemp_b + op.gtemp_e + op.gtemp_d),
                op.gtemp_d,
            )
        };

        stamp(matrix, t, t, op.gtemp_t + 1.0 / self.sized.rth);
        stamp(matrix, t, g, op.gtemp_g);
        stamp(matrix, t, b, op.gtemp_b);
        stamp(matrix, t, e, op.gtemp_e);
        stamp(matrix, t, dp, gtemp_dp);
        stamp(matrix, t, sp, gtemp_sp);
    }
}

#[inline]
fn stamp(matrix: &mut impl MatrixStamper, row: NodeId, col: NodeId, value: Value) {
    if row != 0 && col != 0 && value != 0.0 {
        matrix.stamp(row, col, value);
    }
}

#[inline]
fn stamp_rhs(matrix: &mut impl MatrixStamper, node: NodeId, value: Value) {
    if node != 0 && value != 0.0 {
        matrix.stamp_rhs(node, value);
    }
}

#[inline]
fn stamp_conductance(matrix: &mut impl MatrixStamper, a: NodeId, b: NodeId, g: Value) {
    if g <= 0.0 || a == b {
        return;
    }
    stamp(matrix, a, a, g);
    stamp(matrix, a, b, -g);
    stamp(matrix, b, a, -g);
    stamp(matrix, b, b, g);
}

#[inline]
fn biases_match(a: B3SoiDdBias, b: B3SoiDdBias) -> bool {
    a.vbs == b.vbs
        && a.vgs == b.vgs
        && a.vds == b.vds
        && a.ves == b.ves
        && a.vps == b.vps
        && a.del_temp == b.del_temp
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, sync::Arc};

    /// The `N1` NMOS card from `tests/bsim3soidd/nmosdd.mod` (BSIMDD2.0).
    fn n1_params() -> HashMap<String, Value> {
        let pairs: &[(&str, Value)] = &[
            ("LEVEL", 56.0),
            ("TNOM", 27.0),
            ("TOX", 4.5e-9),
            ("TSI", 5e-8),
            ("TBOX", 8e-8),
            ("MOBMOD", 0.0),
            ("CAPMOD", 3.0),
            ("SHMOD", 0.0),
            ("PARAMCHK", 0.0),
            ("WINT", 0.0),
            ("LINT", -2e-8),
            ("VTH0", 0.52),
            ("K1", 0.39),
            ("K2", 0.1),
            ("K3", 0.0),
            ("KB1", 0.95),
            ("K3B", 2.2),
            ("NLX", 7.2e-8),
            ("DVT0", 0.55),
            ("DVT1", 0.28),
            ("DVT2", -1.4),
            ("DVT0W", 0.0),
            ("DVT1W", 0.0),
            ("DVT2W", 0.0),
            ("NCH", 3.3e17),
            ("NSUB", 1e15),
            ("NGATE", 1e20),
            ("DVBD0", 60.0),
            ("DVBD1", 1.1),
            ("VBSA", 0.0),
            ("KB3", 2.2),
            ("DELP", 0.02),
            ("ABP", 0.9),
            ("MXC", 0.9),
            ("ADICE0", 0.93),
            ("KBJT1", 1e-8),
            ("EDL", 5e-7),
            ("NDIODE", 1.13),
            ("NTUN", 14.0),
            ("ISBJT", 2e-6),
            ("ISDIF", 1e-6),
            ("ISTUN", 0.0),
            ("ISREC", 1e-5),
            ("XBJT", 0.01),
            ("XDIF", 0.01),
            ("XREC", 0.01),
            ("XTUN", 0.001),
            ("U0", 352.0),
            ("UA", 1.3e-11),
            ("UB", 1.7e-18),
            ("UC", -4e-10),
            ("W0", 1.16e-6),
            ("AGS", 0.25),
            ("A1", 0.0),
            ("A2", 1.0),
            ("B0", 0.01),
            ("B1", 10.0),
            ("RDSW", 700.0),
            ("PRWG", 0.0),
            ("PRWB", -0.2),
            ("WR", 1.0),
            ("RBODY", 0.0),
            ("RBSH", 0.0),
            ("A0", 1.4),
            ("KETA", -0.67),
            ("VSAT", 135000.0),
            ("DWG", 0.0),
            ("DWB", 0.0),
            ("ALPHA0", 0.0),
            ("ALPHA1", 1.5),
            ("BETA0", 20.5),
            ("AII", 1.2),
            ("BII", 0.1e-7),
            ("CII", 0.8),
            ("DII", 0.6),
            ("VOFF", -0.14),
            ("NFACTOR", 0.7),
            ("CDSC", 2e-5),
            ("CDSCB", 0.0),
            ("CDSCD", 0.0),
            ("CIT", 0.0),
            ("PCLM", 2.9),
            ("PVAG", 12.0),
            ("PDIBLC1", 0.18),
            ("PDIBLC2", 0.004),
            ("PDIBLCB", -0.234),
            ("DROUT", 0.2),
            ("DELTA", 0.01),
            ("ETA0", 0.01),
            ("ETAB", 0.0),
            ("DSUB", 0.3),
            ("RTH0", 0.006),
            ("CLC", 1e-7),
            ("CLE", 0.6),
            ("CF", 1e-20),
            ("CKAPPA", 0.6),
            ("CGDL", 1e-20),
            ("CGSL", 1e-20),
            ("KT1", -0.3),
            ("KT1L", 0.0),
            ("KT2", 0.022),
            ("UTE", -1.5),
            ("UA1", 4.31e-9),
            ("UB1", -7.61e-18),
            ("UC1", -5.6e-11),
            ("PRT", 760.0),
            ("AT", 22400.0),
            ("CGSO", 1e-10),
            ("CGDO", 1e-10),
            ("CJSWG", 5e-10),
            ("TT", 3e-10),
            ("ASD", 0.3),
            ("CSDESW", 1e-12),
        ];
        pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    fn geom() -> B3SoiDdGeometry {
        // w=10u l=0.25u, no area/perimeter given on the card.
        B3SoiDdGeometry {
            l: 0.25e-6,
            w: 10e-6,
            drain_area: 0.0,
            source_area: 0.0,
            drain_squares: 0.0,
            source_squares: 0.0,
            drain_perimeter: 0.0,
            source_perimeter: 0.0,
            body_squares: 0.0,
            rth0: 0.006,
            cth0: 0.0,
        }
    }

    fn model_consts(m: &B3SoiDdModel) -> ModelConsts {
        ModelConsts {
            cap_mod: m.cap_mod,
            cox: m.cox,
            cbox: m.cbox,
            csi: m.csi,
            csieff: m.csieff,
            qsi: m.qsi,
            qsieff: m.qsieff,
            adice: m.adice,
            tox: m.tox,
            tsi: m.tsi,
            xj: m.xj,
            charge_q: super::super::common::CHARGE_Q,
            mob_mod: m.mob_mod,
            cboxt: m.cboxt,
            xpart: m.xpart,
            tt: m.tt,
            mjswg: m.body_jct_gate_side_grading_coeff,
            phibswg: m.gate_sidewall_jct_potential.max(0.1),
            cjswg: m.unit_length_gate_sidewall_jct_cap,
            mtype: m.mtype,
        }
    }

    #[derive(Default)]
    struct CaptureStamper {
        matrix: Vec<(NodeId, NodeId, Value)>,
        rhs: Vec<(NodeId, Value)>,
    }

    impl MatrixStamper for CaptureStamper {
        fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
            self.matrix.push((row, col, value));
        }

        fn stamp_rhs(&mut self, index: NodeId, value: Value) {
            self.rhs.push((index, value));
        }
    }

    impl CaptureStamper {
        fn matrix_sum(&self, row: NodeId, col: NodeId) -> Value {
            self.matrix
                .iter()
                .filter(|(r, c, _)| *r == row && *c == col)
                .map(|(_, _, v)| *v)
                .sum()
        }

        fn rhs_sum(&self, node: NodeId) -> Value {
            self.rhs
                .iter()
                .filter(|(n, _)| *n == node)
                .map(|(_, v)| *v)
                .sum()
        }
    }

    #[test]
    fn nonideal_body_tie_stamps_resolved_body_contact_resistance() {
        let mut params = n1_params();
        params.insert("RBODY".to_string(), 1.0);
        params.insert("RBSH".to_string(), 0.25);
        let model = Arc::new(B3SoiDdModel::from_params(&params, false, 300.15));
        let mut geometry = geom();
        geometry.body_squares = 4.0;
        let dev = B3SoiDd::new(
            "m1".to_string(),
            1,
            2,
            3,
            4,
            5,
            7,
            0,
            BodyMode::TiedResistive,
            model,
            geometry,
            300.15,
        )
        .expect("DD device with nonideal body tie builds");

        let expected_gbp = 1.0 / (dev.sized.rbody + dev.sized.rbodyext);
        assert!(expected_gbp.is_finite() && expected_gbp > 0.0);

        let mut stamper = CaptureStamper::default();
        dev.stamp_op(
            &B3SoiDdOp::default(),
            B3SoiDdBias {
                vbs: 0.0,
                vgs: 0.0,
                vds: 0.0,
                ves: 0.0,
                vps: 0.0,
                del_temp: 0.0,
            },
            &mut stamper,
        );

        assert!((stamper.matrix_sum(5, 5) - expected_gbp).abs() <= 1.0e-12 * expected_gbp);
        assert!((stamper.matrix_sum(5, 7) + expected_gbp).abs() <= 1.0e-12 * expected_gbp);
        assert!((stamper.matrix_sum(7, 5) + expected_gbp).abs() <= 1.0e-12 * expected_gbp);
        assert!((stamper.matrix_sum(7, 7) - expected_gbp).abs() <= 1.0e-12 * expected_gbp);
    }

    #[test]
    fn self_heating_charge_companion_stamps_qth_capacitance() {
        let mut params = n1_params();
        params.insert("SHMOD".to_string(), 1.0);
        let model = Arc::new(B3SoiDdModel::from_params(&params, false, 300.15));
        let mut geometry = geom();
        geometry.cth0 = 2.0;
        let dev = B3SoiDd::new(
            "m1".to_string(),
            1,
            2,
            3,
            4,
            5,
            0,
            6,
            BodyMode::TiedIdeal,
            model,
            geometry,
            300.15,
        )
        .expect("self-heating DD device builds with a temp node");
        let cth = dev.thermal_capacitance();
        assert!(cth > 0.0, "cth={cth:.6e}");

        let ag0 = 7.0;
        let del_temp = 0.25;
        let voltages = [0.5, 1.2, 0.0, 0.0, 0.1, del_temp];
        let mut stamper = CaptureStamper::default();

        dev.stamp_charge_companion(
            &eval::B3SoiDdCharge::default(),
            ag0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            &voltages,
            &mut stamper,
        );

        let expected_gc_tt = ag0 * cth;
        let expected_rhs = expected_gc_tt * del_temp;
        let actual_gc_tt = stamper.matrix_sum(6, 6);
        let actual_rhs = stamper.rhs_sum(6);
        assert!(
            (actual_gc_tt - expected_gc_tt).abs() <= 1e-15,
            "temp-node companion conductance {actual_gc_tt:.6e} vs {expected_gc_tt:.6e}"
        );
        assert!(
            (actual_rhs - expected_rhs).abs() <= 1e-15,
            "temp-node companion RHS {actual_rhs:.6e} vs {expected_rhs:.6e}"
        );
    }

    #[test]
    fn self_heating_update_limits_temp_rise_per_newton_iteration() {
        let mut params = n1_params();
        params.insert("SHMOD".to_string(), 1.0);
        let model = Arc::new(B3SoiDdModel::from_params(&params, false, 300.15));
        let mut dev = B3SoiDd::new(
            "m1".to_string(),
            1,
            2,
            3,
            4,
            5,
            0,
            6,
            BodyMode::TiedIdeal,
            model,
            geom(),
            300.15,
        )
        .expect("self-heating DD device builds with a temp node");

        let zero_temp = [0.5, 1.2, 0.0, 0.0, 0.1, 0.0];
        dev.update(&zero_temp);
        assert_eq!(dev.bias.del_temp, 0.0);

        let hot_prediction = [0.5, 1.2, 0.0, 0.0, 0.1, 100.0];
        dev.update(&hot_prediction);
        assert!(
            (dev.bias.del_temp - 5.0).abs() <= 1.0e-12,
            "delTemp should be limited to 5 K per iteration, got {:.6e}",
            dev.bias.del_temp
        );
        assert!(
            dev.last_limited.get(),
            "temperature limiting should mark the iterate non-bypassable"
        );
    }

    #[test]
    fn self_heating_update_uses_temp_node_for_electrical_eval() {
        let mut params = n1_params();
        params.insert("SHMOD".to_string(), 1.0);
        let model = Arc::new(B3SoiDdModel::from_params(&params, false, 300.15));
        let geometry = geom();
        let mut dev = B3SoiDd::new(
            "m1".to_string(),
            1,
            2,
            3,
            4,
            5,
            0,
            6,
            BodyMode::TiedIdeal,
            model.clone(),
            geometry,
            300.15,
        )
        .expect("self-heating DD device builds with a temp node");

        let hot_voltages = [0.8, 1.2, 0.0, 0.0, 0.0, 50.0];
        dev.update(&hot_voltages);

        let mc = model_consts(&model);
        let bias = B3SoiDdBias {
            vbs: 0.0,
            vgs: 1.2,
            vds: 0.8,
            ves: 0.0,
            vps: 0.0,
            del_temp: 50.0,
        };
        let cold_sized = B3SoiDdSized::new(&model, &geometry, 300.15).expect("cold sized");
        let hot_sized = B3SoiDdSized::new(&model, &geometry, 350.15).expect("hot sized");
        let cold = eval::eval_dc(&cold_sized, &mc, bias, 1.0);
        let hot = eval::eval_dc(&hot_sized, &mc, bias, 1.0);

        assert!(
            (hot.ids - cold.ids).abs() > 1.0e-8 * hot.ids.abs().max(cold.ids.abs()) + 1.0e-15,
            "test bias must expose electrical temperature dependence: cold ids={:.9e}, hot ids={:.9e}",
            cold.ids,
            hot.ids
        );
        assert!(
            (dev.op.ids - hot.ids).abs() <= 1.0e-10 * hot.ids.abs().max(1.0) + 1.0e-15,
            "self-heated update should evaluate at CKTtemp + delTemp; got ids={:.9e}, expected hot ids={:.9e}, cold ids={:.9e}",
            dev.op.ids,
            hot.ids,
            cold.ids
        );
    }

    #[test]
    fn self_heating_dc_stamp_adds_electrical_temp_column() {
        let mut params = n1_params();
        params.insert("SHMOD".to_string(), 1.0);
        let model = Arc::new(B3SoiDdModel::from_params(&params, false, 300.15));
        let geometry = geom();
        let mut dev = B3SoiDd::new(
            "m1".to_string(),
            1,
            2,
            3,
            4,
            5,
            0,
            6,
            BodyMode::TiedIdeal,
            model.clone(),
            geometry,
            300.15,
        )
        .expect("self-heating DD device builds with a temp node");

        let hot_voltages = [0.8, 1.2, 0.0, 0.0, 0.0, 50.0];
        dev.update(&hot_voltages);
        let mut stamper = CaptureStamper::default();
        let mut rhs: [Value; 0] = [];
        dev.stamp_nonlinear(&hot_voltages, &mut stamper, &mut rhs);

        let mc = model_consts(&model);
        let eval_cd_at = |del_temp: Value| {
            let sized = B3SoiDdSized::new(&model, &geometry, 300.15 + del_temp).expect("sized");
            eval::eval_dc(
                &sized,
                &mc,
                B3SoiDdBias {
                    vbs: 0.0,
                    vgs: 1.2,
                    vds: 0.8,
                    ves: 0.0,
                    vps: 0.0,
                    del_temp,
                },
                1.0,
            )
            .cd
        };
        let h = 1.0e-3;
        let expected = (eval_cd_at(50.0 + h) - eval_cd_at(50.0 - h)) / (2.0 * h);
        let actual = stamper.matrix_sum(1, 6);
        assert!(
            (actual - expected).abs() <= 5.0e-3 * expected.abs().max(actual.abs()) + 1.0e-15,
            "DP,temp stamp should track d(cd)/dTemp; actual={actual:.9e}, expected={expected:.9e}"
        );
    }

    #[test]
    fn self_heating_charge_companion_adds_electrical_temp_columns() {
        let mut params = n1_params();
        params.insert("SHMOD".to_string(), 1.0);
        let model = Arc::new(B3SoiDdModel::from_params(&params, false, 300.15));
        let geometry = geom();
        let dev = B3SoiDd::new(
            "m1".to_string(),
            1,
            2,
            3,
            4,
            5,
            0,
            6,
            BodyMode::TiedIdeal,
            model.clone(),
            geometry,
            300.15,
        )
        .expect("self-heating DD device builds with a temp node");

        let hot_voltages = [0.8, 1.2, 0.0, 0.0, 0.0, 50.0];
        let charge = dev.charge_at(&hot_voltages);
        let ag0 = 2.0e6;
        let mut stamper = CaptureStamper::default();
        dev.stamp_charge_companion(
            &charge,
            ag0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            &hot_voltages,
            &mut stamper,
        );

        let mc = model_consts(&model);
        let eval_qg_at = |del_temp: Value| {
            let sized = B3SoiDdSized::new(&model, &geometry, 300.15 + del_temp).expect("sized");
            eval::eval(
                &sized,
                &mc,
                B3SoiDdBias {
                    vbs: 0.0,
                    vgs: 1.2,
                    vds: 0.8,
                    ves: 0.0,
                    vps: 0.0,
                    del_temp,
                },
                1.0,
                true,
            )
            .charge
            .unwrap()
            .qg
        };
        let h = 1.0e-3;
        let expected = ag0 * (eval_qg_at(50.0 + h) - eval_qg_at(50.0 - h)) / (2.0 * h);
        let actual = stamper.matrix_sum(2, 6);
        assert!(
            (actual - expected).abs() <= 5.0e-3 * expected.abs().max(actual.abs()) + 1.0e-18,
            "G,temp charge stamp should track ag0*d(qg)/dTemp; actual={actual:.9e}, expected={expected:.9e}"
        );
    }

    #[test]
    fn temp_setup_is_finite_and_physical() {
        let model = B3SoiDdModel::from_params(&n1_params(), false, 300.15);
        let sized = B3SoiDdSized::new(&model, &geom(), 300.15).expect("sized");
        assert!(sized.phi > 0.5 && sized.phi < 1.2, "phi={}", sized.phi);
        assert!(sized.vtm > 0.02 && sized.vtm < 0.03, "vtm={}", sized.vtm);
        assert!(sized.u0temp > 0.0 && sized.u0temp < 1.0);
        assert!(sized.vth0.is_finite());
        assert!(sized.rds0 >= 0.0);
        // jbjt etc. must be positive saturation densities.
        assert!(sized.jbjt > 0.0 && sized.jrec > 0.0 && sized.jdif > 0.0);
    }

    /// The floating-body DC equilibrium must rise into forward bias as the gate
    /// turns the device on with the drain at 1.5 V (the RampVg2 bias), driven by
    /// impact ionization charging the body until the source diode clamps it.
    /// This is the DC anchor the RampVg2 transient relaxes toward.
    #[test]
    fn floating_body_dc_equilibrium_is_forward_biased() {
        use crate::{Engine, Netlist};
        let model = n1_card_text();
        let solve_vb = |vg: f64| -> f64 {
            let deck = format!(
                "*ramp op\nvd d 0 dc 1.5\nvs s 0 dc 0\nve e 0 dc 0\nvg g 0 dc {vg}\nm1 d g s e n1 w=10u l=0.25u\n.option gmin=1e-20 itl1=500\n.op\n{model}\n.end\n"
            );
            let netlist = Netlist::parse(&deck).expect("parse");
            let engine = Engine::new(crate::SimulationConfig::default());
            let res = engine.run_dc_op(&netlist).expect("dc op");
            res.try_voltage_named("m1.__body.internal")
                .expect("body node")
        };
        // Off (Vg=0): body near the t=0 RampVg2 reference (~0.092 V).
        let vb0 = solve_vb(0.0);
        assert!((vb0 - 0.0917).abs() < 5e-3, "Vb(Vg=0)={vb0:.4e}");
        // Saturation (Vg=1): strong impact ionization forward-biases the body.
        let vb1 = solve_vb(1.0);
        assert!(vb1 > 0.4 && vb1 < 0.7, "Vb(Vg=1)={vb1:.4e}");
    }

    /// End-to-end check: a tied-body NMOS at the t4 first operating point
    /// (Vg=0, Vd=0.05, Vb=-0.3) must reproduce the checked-in ngspice reference
    /// drain current to within 0.1%. Exercises the builder dispatch, the SOI
    /// stamping consistency (cdreq vs. linearized conductances), and the
    /// `VOFF=-.14` model-card sign parsing all at once.
    #[test]
    fn t4_first_point_matches_ngspice_reference() {
        use crate::{Engine, Netlist};
        let model = n1_card_text();
        let deck = format!(
            "*t4 single point\nvd d 0 dc 0.05\nvs s 0 dc 0\nve e 0 dc 0\nvg g 0 dc 0\nvb b 0 dc -0.3\nm1 d g s e b n1 w=10u l=0.25u\n.option gmin=1e-25 itl1=500\n.op\n{model}\n.end\n"
        );
        let netlist = Netlist::parse(&deck).expect("parse t4 single point deck");
        let engine = Engine::new(crate::SimulationConfig::default());
        let res = engine.run_dc_op(&netlist).expect("dc op");
        // vs is the 2nd voltage source (branch index 1); i(vs) == drain current.
        let i_vs = res.branch_currents[1];
        // ngspice t4.out row 0: vs#branch = 4.146227e-12 at Vg=0, Vb=-0.3.
        let reference = 4.146227e-12;
        assert!(
            (i_vs - reference).abs() <= 1e-3 * reference.abs(),
            "i(vs)={i_vs:.6e} vs reference {reference:.6e}"
        );
    }

    /// The `N1` NMOS model card text from `tests/bsim3soidd/nmosdd.mod`, kept in
    /// sync with [`n1_params`]; used by the end-to-end builder test so it does
    /// not depend on the test working directory.
    fn n1_card_text() -> &'static str {
        ".Model N1 NMOS Level=56\n\
         +TNOM=27 TOX=4.5E-09 TSI=5e-8 TBOX=8E-08\n\
         +MOBMOD=0 CAPMOD=3 SHMOD=0\n\
         +PARAMCHK=0 WINT=0 LINT=-2E-08\n\
         +VTH0=.52 K1=.39 K2=.1 K3=0\n\
         +KB1=.95 K3B=2.2 NLX=7.2E-08\n\
         +DVT0=.55 DVT1=.28 DVT2=-1.4\n\
         +DVT0W=0 DVT1W=0 DVT2W=0\n\
         +NCH=3.3E+17 NSUB=1E+15 NGATE=1E+20\n\
         +DVBD0=60.0 DVBD1=1.1 VBSA=0.0\n\
         +KB3=2.2 DELP=0.02\n\
         +ABP=0.9 MXC=0.9 ADICE0=0.93\n\
         +KBJT1=1.0E-08 EDL=.0000005\n\
         +NDIODE=1.13 NTUN=14.0\n\
         +ISBJT=2e-6 ISDIF=1e-6 ISTUN=0.0 ISREC=1e-5\n\
         +XBJT=0.01 XDIF=0.01 XREC=0.01 XTUN=0.001\n\
         +U0=352 UA=1.3E-11 UB=1.7E-18 UC=-4E-10\n\
         +W0=1.16E-06 AGS=.25 A1=0 A2=1\n\
         +B0=.01 B1=10\n\
         +RDSW=700 PRWG=0 PRWB=-.2 WR=1\n\
         +RBODY=0.0 RBSH=0.0\n\
         +A0=1.4 KETA=-.67 VSAT=135000\n\
         +DWG=0 DWB=0\n\
         +ALPHA0=0.0 ALPHA1=1.5 BETA0=20.5\n\
         +AII=1.2 BII=0.1e-7 CII=0.8 DII=0.6\n\
         +VOFF=-.14 NFACTOR=.7 CDSC=.00002 CDSCB=0\n\
         +CDSCD=0 CIT=0\n\
         +PCLM=2.9 PVAG=12 PDIBLC1=.18 PDIBLC2=.004\n\
         +PDIBLCB=-.234 DROUT=.2\n\
         +DELTA=.01 ETA0=.01 ETAB=0\n\
         +DSUB=.3 RTH0=.006\n\
         +CLC=.0000001 CLE=.6 CF=1E-20 CKAPPA=.6\n\
         +CGDL=1E-20 CGSL=1E-20 KT1=-.3 KT1L=0\n\
         +KT2=.022 UTE=-1.5 UA1=4.31E-09 UB1=-7.61E-18\n\
         +UC1=-5.6E-11 PRT=760 AT=22400\n\
         +CGSO=1e-10 CGDO=1e-10 CJSWG=5e-10 TT=3e-10\n\
         +ASD=0.3 CSDESW=1e-12\n"
    }

    /// The CAPMOD=2/3 capacitance matrix must be the Jacobian of the node charges:
    /// `cXgb ≈ d(qX)/d(vg)` etc. Validates the charge derivatives by finite
    /// difference against the charges themselves, and that the four node charges
    /// conserve (`qg+qb+qd+qe+qs == 0`, with qs implied).
    fn charge_matrix_is_consistent_with_charges_for_capmod(cap_mod: Value) {
        let mut params = n1_params();
        params.insert("CAPMOD".to_string(), cap_mod);
        let model = B3SoiDdModel::from_params(&params, false, 300.15);
        let sized = B3SoiDdSized::new(&model, &geom(), 300.15).expect("sized");
        let mc = model_consts(&model);
        let charge = |vg: Value, vd: Value, vb: Value, ve: Value| {
            eval::eval(
                &sized,
                &mc,
                B3SoiDdBias {
                    vbs: vb,
                    vgs: vg,
                    vds: vd,
                    ves: ve,
                    vps: 0.0,
                    del_temp: 0.0,
                },
                1.0,
                true,
            )
            .charge
            .unwrap()
        };
        let (vg, vd, vb, ve) = (1.2_f64, 0.8, 0.05, 0.0);
        let h = 1e-6;
        let c0 = charge(vg, vd, vb, ve);
        // d/dVg by central difference.
        let cp = charge(vg + h, vd, vb, ve);
        let cm = charge(vg - h, vd, vb, ve);
        let dqg_dvg = (cp.qg - cm.qg) / (2.0 * h);
        let dqb_dvg = (cp.qb - cm.qb) / (2.0 * h);
        // gXgb = d(qX)/d(vg) at fixed other terminals (vb reference folds out for
        // the gate column). Compare within 1% + small abs floor.
        let ok = |analytic: Value, fd: Value| {
            (analytic - fd).abs() <= 1e-2 * analytic.abs().max(fd.abs()) + 1e-14
        };
        assert!(ok(c0.gcggb, dqg_dvg), "cggb {} vs FD {}", c0.gcggb, dqg_dvg);
        assert!(ok(c0.gcbgb, dqb_dvg), "cbgb {} vs FD {}", c0.gcbgb, dqb_dvg);
        // Charge conservation: qg+qb+qd+qe+qs == 0 -> qs = -(qg+qb+qd+qe).
        let total = c0.qg + c0.qb + c0.qd + c0.qe;
        assert!(total.is_finite(), "charge sum not finite: {total}");
    }

    #[test]
    fn capmod3_charge_matrix_is_consistent_with_charges() {
        charge_matrix_is_consistent_with_charges_for_capmod(3.0);
    }

    #[test]
    fn capmod2_charge_matrix_is_consistent_with_charges() {
        charge_matrix_is_consistent_with_charges_for_capmod(2.0);
    }

    #[test]
    fn eval_dc_strong_inversion_is_sane() {
        let model = B3SoiDdModel::from_params(&n1_params(), false, 300.15);
        let sized = B3SoiDdSized::new(&model, &geom(), 300.15).expect("sized");
        let mc = model_consts(&model);

        // Tied body (vbs=0), Vg=3, Vd=0.05 (linear), like t4 first point.
        let bias = B3SoiDdBias {
            vbs: 0.0,
            vgs: 3.0,
            vds: 0.05,
            ves: 0.0,
            vps: 0.0,
            del_temp: 0.0,
        };
        let op = eval::eval_dc(&sized, &mc, bias, 1.0);
        assert!(op.ids.is_finite() && op.ids > 0.0, "ids={}", op.ids);
        assert!(op.gds.is_finite() && op.gds > 0.0, "gds={}", op.gds);
        assert!(op.gm.is_finite() && op.gm >= 0.0, "gm={}", op.gm);
        assert!(op.von.is_finite());
        // Linear-region drain current for W/L=40, this device is ~ mA range.
        assert!(op.ids < 1.0, "ids unreasonably large: {}", op.ids);
    }

    #[test]
    fn eval_dc_monotonic_in_vg() {
        let model = B3SoiDdModel::from_params(&n1_params(), false, 300.15);
        let sized = B3SoiDdSized::new(&model, &geom(), 300.15).expect("sized");
        let mc = model_consts(&model);
        let mk = |vg: Value| {
            eval::eval_dc(
                &sized,
                &mc,
                B3SoiDdBias {
                    vbs: 0.0,
                    vgs: vg,
                    vds: 0.05,
                    ves: 0.0,
                    vps: 0.0,
                    del_temp: 0.0,
                },
                1.0,
            )
            .ids
        };
        let i_low = mk(0.3); // below threshold
        let i_mid = mk(0.8);
        let i_high = mk(1.5);
        assert!(i_low < i_mid, "{} !< {}", i_low, i_mid);
        assert!(i_mid < i_high, "{} !< {}", i_mid, i_high);
        assert!(i_low >= 0.0);
    }

    #[test]
    fn eval_dc_no_nan_across_sweep() {
        let model = B3SoiDdModel::from_params(&n1_params(), false, 300.15);
        let sized = B3SoiDdSized::new(&model, &geom(), 300.15).expect("sized");
        let mc = model_consts(&model);
        for vg_i in 0..=15 {
            for vd_i in 0..=30 {
                for vbs_i in 0..=4 {
                    let bias = B3SoiDdBias {
                        vbs: 0.1 * vbs_i as Value,
                        vgs: 0.1 * vg_i as Value,
                        vds: 0.1 * vd_i as Value,
                        ves: 0.0,
                        vps: 0.0,
                        del_temp: 0.0,
                    };
                    let op = eval::eval_dc(&sized, &mc, bias, 1.0);
                    assert!(op.ids.is_finite(), "ids NaN at {bias:?}");
                    assert!(op.cbody.is_finite(), "cbody NaN at {bias:?}");
                    assert!(op.gds.is_finite() && op.gm.is_finite());
                    assert!(op.cjd.is_finite() && op.cjs.is_finite());
                }
            }
        }
    }

    /// Oracle-pinned charge check: the body charge and the full body row of
    /// the capacitance matrix must reproduce a debug ngspice-46 evaluation of
    /// the RampVg2 deck captured mid-gate-ramp (gdb at b3soiddld.c:3496,
    /// t = 22.5 ps, Vbs = 0.1615795006, Vgs = 0.05, Vds = 1.5, Ves = 0).
    /// qbody and cb** are pre-overlap quantities in ngspice and map directly
    /// onto RSpice's charge.qb / gcb** (the overlap lump never touches the
    /// body row).
    #[test]
    fn body_charge_matches_ngspice_oracle_at_ramp_bias() {
        let model = B3SoiDdModel::from_params(&n1_params(), false, 300.15);
        let sized = B3SoiDdSized::new(&model, &geom(), 300.15).expect("sized");
        let mc = model_consts(&model);
        let charge = eval::eval(
            &sized,
            &mc,
            B3SoiDdBias {
                vbs: 0.1615795006,
                vgs: 0.05,
                vds: 1.5,
                ves: 0.0,
                vps: 0.0,
                del_temp: 0.0,
            },
            1.0,
            true,
        )
        .charge
        .unwrap();
        let ok = |actual: Value, oracle: Value| (actual - oracle).abs() <= 1e-6 * oracle.abs();
        assert!(ok(charge.qb, -5.755219597e-15), "qb={:.9e}", charge.qb);
        assert!(
            ok(charge.gcbgb, -6.264670704e-15),
            "gcbgb={:.9e}",
            charge.gcbgb
        );
        assert!(
            ok(charge.gcbdb, -1.482516887e-15),
            "gcbdb={:.9e}",
            charge.gcbdb
        );
        assert!(
            ok(charge.gcbsb, -8.674246038e-15),
            "gcbsb={:.9e}",
            charge.gcbsb
        );
        assert!(
            ok(charge.gcbeb, -2.308017492e-15),
            "gcbeb={:.9e}",
            charge.gcbeb
        );
    }

    #[test]
    fn probe_body_ratio_at_ramp_start() {
        let model = B3SoiDdModel::from_params(&n1_params(), false, 300.15);
        let sized = B3SoiDdSized::new(&model, &geom(), 300.15).expect("sized");
        let mc = model_consts(&model);
        let qb = |vg: Value, vb: Value| {
            eval::eval(
                &sized,
                &mc,
                B3SoiDdBias {
                    vbs: vb,
                    vgs: vg,
                    vds: 1.5,
                    ves: 0.0,
                    vps: 0.0,
                    del_temp: 0.0,
                },
                1.0,
                true,
            )
            .charge
            .unwrap()
            .qb
        };
        let h = 1e-6;
        for (vg, vb) in [(0.0_f64, 0.09166_f64), (0.025, 0.13), (0.05, 0.1635)] {
            let dqb_dvg = (qb(vg + h, vb) - qb(vg - h, vb)) / (2.0 * h);
            let dqb_dvb = (qb(vg, vb + h) - qb(vg, vb - h)) / (2.0 * h);
            println!(
                "vg={vg} vb={vb} dqb_dvg={dqb_dvg:.6e} dqb_dvb={dqb_dvb:.6e} ratio={:.4}",
                -dqb_dvg / dqb_dvb
            );
        }
    }
}
