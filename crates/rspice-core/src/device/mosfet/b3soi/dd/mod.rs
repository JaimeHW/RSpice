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
//! For the supported decks (`rbody = rbsh = 0`, `shmod = 0`, no series R):
//! - **Floating body** (`d g s e`): ngspice creates an internal `Body` node,
//!   sets `bodyMod = 0`, `float = 1`. RSpice allocates one internal node.
//! - **Ideal body tie** (`d g s e p`): ngspice sets `bodyMod = 2` and the
//!   external `p` node *is* the body node; no internal node is created.
//!
//! The drain/source primes coincide with the external drain/source (no series
//! resistance in the supported decks), and the temperature node does not exist
//! (`shmod = 0`).
//!
//! # Status
//!
//! - **DC current path** ([`eval::eval_dc`]): faithful transcription of the
//!   `B3SOIDDload` DC block, including the back-gate (E) coupling columns.
//! - **CAPMOD=3 charge model** ([`eval::eval`] with `compute_charges`): the
//!   intrinsic + extrinsic + overlap charges and the coupled capacitance matrix
//!   (b3soiddld.c:2646-3784). Stamped as a transient companion by the engine's
//!   dedicated B3SOI pass; the four node charges (incl. the floating body) feed
//!   the local-truncation-error step control.
//! - **Convergence aids**: `B3SOIDDlimit` (per-iterate 0.2 V body-voltage cap)
//!   and `B3SOIDDSmartVbs` (DC floating-body `Vbs >= 0` clamp), applied each
//!   Newton iterate; SmartVbs is disabled during transient time-stepping.
//! - **Builder dispatch** is live for LEVEL=56 NMOS/PMOS (see `engine/builder.rs`
//!   `build_b3soi_dd`); levels 55/57 still fall through to the generic MOSFET.
//!
//! Verified: the DC sweeps (`t3`/`t4`/`t5`/`inv2`) match the checked-in ngspice
//! references; `ring51` (the 51-stage SOI ring oscillator) runs its full 50 ns
//! transient. The `RampVg2` floating-body `@m1[vbs]` trace matches at the DC
//! anchor (t=0, ~0.0917 V) and tracks the transient (the fast-edge body
//! amplitude is still being calibrated against ngspice's body LTE).

pub use super::common;
pub use params::B3SoiDdModel;

pub mod eval;
pub mod params;
pub mod temp;

use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::{Value, circuit::NodeId};
use eval::{B3SoiDdBias, B3SoiDdOp, ModelConsts};
use std::sync::Arc;
use temp::{B3SoiDdGeometry, B3SoiDdSized};

/// Body-node configuration for one instance (b3soiddset.c node creation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyMode {
    /// Floating body: `B` is an internal node, `float = 1` (bodyMod 0).
    Floating,
    /// Ideal body tie: the external contact node is the body node (bodyMod 2).
    TiedIdeal,
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
    /// Body-contact node `P` (== `node_body` when tied; unused when floating).
    pub node_p: NodeId,

    pub body_mode: BodyMode,

    /// Shared model card (one per `.model`).
    pub model: Arc<B3SoiDdModel>,
    /// Size/temperature-resolved parameters (one per instance geometry).
    pub sized: Arc<B3SoiDdSized>,
    /// Model scalars needed inside the load.
    consts: ModelConsts,

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
        body_mode: BodyMode,
        model: Arc<B3SoiDdModel>,
        geom: B3SoiDdGeometry,
        temp_k: Value,
    ) -> Result<Self, String> {
        // Self-heating is not yet implemented; per spec it must be a hard error
        // when enabled rather than silently ignored (SHMOD=0 in supported decks).
        if model.sh_mod == 1 && geom.rth0 != 0.0 {
            return Err(format!(
                "B3SOIDD '{name}': self-heating (SHMOD=1 with RTH0!=0) is not yet implemented"
            ));
        }
        let sized = Arc::new(B3SoiDdSized::new(&model, &geom, temp_k)?);
        let consts = ModelConsts {
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
            body_mode,
            model,
            sized,
            consts,
            op: B3SoiDdOp::default(),
            bias: B3SoiDdBias {
                vbs: 0.0,
                vgs: 0.0,
                vds: 0.0,
                ves: 0.0,
                vps: 0.0,
            },
            converged_ref: B3SoiDdBias {
                vbs: 0.0,
                vgs: 0.0,
                vds: 0.0,
                ves: 0.0,
                vps: 0.0,
            },
            has_history: false,
            vbs_limit_anchor: 0.0,
            vbd_limit_anchor: 0.0,
            dc_mode: std::cell::Cell::new(true),
            limit_anchor_valid: std::cell::Cell::new(false),
            bypass_tolerances: std::cell::Cell::new(None),
            bypass_active: std::cell::Cell::new(false),
            force_full_eval: std::cell::Cell::new(true),
            last_limited: std::cell::Cell::new(false),
            charges_suppressed: false,
        })
    }

    /// Select the analysis mode for the floating-body convergence aids.
    ///
    /// In DC/operating-point mode the `B3SOIDDSmartVbs` clamp (Vbs >= 0) is
    /// active; in transient it is disabled so a floating body may swing below
    /// the source potential. The per-iteration `B3SOIDDlimit` change cap applies
    /// in both modes (it only protects the Newton path, not the solution).
    pub fn set_dc_mode(&self, dc: bool) {
        self.dc_mode.set(dc);
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

    /// Whether `DEBUG=-1` suppresses this device's charge contributions.
    pub fn charges_suppressed(&self) -> bool {
        self.charges_suppressed
    }

    /// Mark the start of a new timestep attempt (ngspice `MODEINITPRED`): the
    /// next `update` must fully re-evaluate so bypass deltas are always
    /// measured against a state from the current timestep.
    pub fn begin_timestep_iteration(&self) {
        self.force_full_eval.set(true);
        self.bypass_active.set(false);
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
        let vbd_new = raw.vbs - raw.vds;
        let vbd_old = old.vbs - old.vds;
        let delvbd = vbd_new - vbd_old;
        if delvbs.abs() >= vtol(raw.vbs, old.vbs)
            || delvbd.abs() >= vtol(vbd_new, vbd_old)
            || delvgs.abs() >= vtol(raw.vgs, old.vgs)
            || delves.abs() >= vtol(raw.ves, old.ves)
            || delvds.abs() >= vtol(raw.vds, old.vds)
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

    /// Evaluate the CAPMOD=3 charge state at the given solution vector.
    ///
    /// Returns the four node charges and the intrinsic+overlap capacitance
    /// matrix (pre-`ag0`). Used by the engine's transient charge companion.
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
        eval::eval(&self.sized, &self.consts, bias, self.mtype, true)
            .charge
            .expect("compute_charges=true yields a charge state")
    }

    /// Stamp the transient charge companion into the matrix and RHS.
    ///
    /// `ag0` is the integration coefficient (`d/dt` operator gain), and the
    /// `cq*` are the integrated charge-current histories from the engine
    /// (`cq = ag0*q + history`). This mirrors the charge portion of
    /// `B3SOIDDload` (b3soiddld.c:3679-3868, 4083-4128) for `bodyMod` 0/2, no
    /// temp node, no series R. The gate-overlap and extrinsic-substrate
    /// derivatives are already folded into `charge`'s `gc**` matrix.
    pub fn stamp_charge_companion(
        &self,
        charge: &eval::B3SoiDdCharge,
        ag0: Value,
        cqg: Value,
        cqb: Value,
        cqd: Value,
        cqe: Value,
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

        // Equivalent charge currents (b3soiddld.c:3860-3867). type<0 flips sign.
        let mut ceqqg = cqg - gcggb * vgb + gcgdb * vbd + gcgsb * vbs - gcgeb * veb;
        let mut ceqqb = cqb - gcbgb * vgb + gcbdb * vbd + gcbsb * vbs - gcbeb * veb;
        let mut ceqqd = cqd - gcdgb * vgb + gcddb * vbd + gcdsb * vbs - gcdeb * veb;
        let mut ceqqe = cqe - gcegb * vgb + gcedb * vbd + gcesb * vbs - gceeb * veb;
        if self.mtype < 0.0 {
            ceqqg = -ceqqg;
            ceqqb = -ceqqb;
            ceqqd = -ceqqd;
            ceqqe = -ceqqe;
        }

        // RHS (b3soiddld.c:4011-4017, charge parts).
        stamp_rhs(matrix, b, -ceqqb);
        stamp_rhs(matrix, g, -ceqqg);
        stamp_rhs(matrix, dp, -ceqqd);
        stamp_rhs(matrix, sp, ceqqg + ceqqb + ceqqd + ceqqe);
        stamp_rhs(matrix, e, -ceqqe);

        // Matrix charge entries (b3soiddld.c:4083-4128, gc** parts only).
        // E row.
        stamp(matrix, e, g, gcegb);
        stamp(matrix, e, dp, gcedb);
        stamp(matrix, e, sp, gcesb);
        stamp(matrix, e, b, -(gcegb + gcedb + gcesb + gceeb));
        stamp(matrix, e, e, gceeb);
        // G row.
        stamp(matrix, g, e, gcgeb);
        stamp(matrix, g, b, -(gcggb + gcgdb + gcgsb + gcgeb));
        stamp(matrix, g, g, gcggb);
        stamp(matrix, g, dp, gcgdb);
        stamp(matrix, g, sp, gcgsb);
        // DP row.
        stamp(matrix, dp, e, gcdeb);
        stamp(matrix, dp, b, -(gcdgb + gcddb + gcdeb + gcdsb));
        stamp(matrix, dp, g, gcdgb);
        stamp(matrix, dp, dp, gcddb);
        stamp(matrix, dp, sp, gcdsb);
        // SP row.
        stamp(matrix, sp, e, gcseb);
        stamp(matrix, sp, b, -(gcsgb + gcsdb + gcseb + gcssb));
        stamp(matrix, sp, g, gcsgb);
        stamp(matrix, sp, dp, gcsdb);
        stamp(matrix, sp, sp, gcssb);
        // B row.
        stamp(matrix, b, e, gcbeb);
        stamp(matrix, b, g, gcbgb);
        stamp(matrix, b, dp, gcbdb);
        stamp(matrix, b, sp, gcbsb);
        stamp(matrix, b, b, -(gcbgb + gcbdb + gcbsb + gcbeb));
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
        B3SoiDdBias {
            vbs: self.mtype * (vb - vs),
            vgs: self.mtype * (vg - vs),
            vds: self.mtype * (vd - vs),
            ves: self.mtype * (ve - vs),
            vps: self.mtype * (vp - vs),
        }
    }

    fn branch_voltages(&self, v: &[Value]) -> B3SoiDdBias {
        let mut bias = self.raw_branch_voltages(v);
        let _ = self.apply_body_limiting(&mut bias);
        bias
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
        self.op = eval::eval_dc(&self.sized, &self.consts, bias, self.mtype);
        self.has_history = true;
        // Anchor the per-iteration limiter at this (limited) iterate, in the
        // mode-selected frame, for the next Newton step.
        self.vbs_limit_anchor = bias.vbs;
        self.vbd_limit_anchor = bias.vbs - bias.vds;
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
            return;
        }
        let bias = self.branch_voltages(voltages);
        let op = if biases_match(bias, self.bias) {
            self.op.clone()
        } else {
            eval::eval_dc(&self.sized, &self.consts, bias, self.mtype)
        };
        // `cdreq`/`ceq*` must be formed from the *same* bias that produced `op`,
        // not the (possibly stale) `self.bias` cached at the last `update`.
        self.stamp_op(&op, bias, matrix);
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
    }
}

impl B3SoiDd {
    /// Stamp the linearized DC operating point.
    ///
    /// Faithful transcription of the DC portion of the B3SOIDD matrix/RHS load
    /// (b3soiddld.c:3886-4150) for `bodyMod` 0/2, no temp node, no series R, and
    /// `ChargeComputationNeeded == 0` (so all `gc*`/`ceqq*` charge terms vanish).
    /// `m` (multiplier) is 1. CKTgmin terms are omitted: the RSpice solver adds
    /// its own diagonal gmin, so replicating ngspice's per-device gmin would
    /// double-count it.
    ///
    /// In DC `dNodePrime == dNode`, `sNodePrime == sNode`, `bNode` is the body
    /// node (internal floating or external tie), and `pNode`/`tempNode` are
    /// absent.
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
        let (gbbg, gbbdp, gbbb, gbbe, gbbsp);
        let (gddpg, gddpdp, gddpb, gddpe, gddpsp);
        let (gsspg, gsspdp, gsspb, gsspe, gsspsp);
        if op.mode >= 0 {
            gm = op.gm;
            gmbs = op.gmbs;
            gme = op.gme;
            fwd_sum = gm + gmbs + gme;
            rev_sum = 0.0;
            cdreq = mt
                * (op.cdrain
                    - op.gds * bias.vds
                    - gm * bias.vgs
                    - gmbs * bias.vbs
                    - gme * bias.ves);
            ceqbs = op.cjs;
            ceqbd = op.cjd;

            gbbg = -op.gbgs;
            gbbdp = -op.gbds;
            gbbb = -op.gbbs;
            gbbe = -op.gbes;
            let gbbp = -op.gbps;
            gbbsp = -(gbbg + gbbdp + gbbb + gbbe + gbbp);

            gddpg = -op.gjdg;
            gddpdp = -op.gjdd;
            gddpb = -op.gjdb;
            gddpe = -op.gjde;
            gddpsp = -(gddpg + gddpdp + gddpb + gddpe);

            gsspg = -op.gjsg;
            gsspdp = -op.gjsd;
            gsspb = -op.gjsb;
            gsspe = 0.0;
            gsspsp = -(gsspg + gsspdp + gsspb + gsspe);
        } else {
            gm = -op.gm;
            gmbs = -op.gmbs;
            gme = -op.gme;
            fwd_sum = 0.0;
            rev_sum = -(gm + gmbs + gme);
            let vgd = bias.vgs - bias.vds;
            let vbd = bias.vbs - bias.vds;
            cdreq = -mt
                * (op.cdrain
                    + op.gds * bias.vds
                    + gm * vgd
                    + gmbs * vbd
                    + gme * (bias.ves - bias.vds));
            ceqbs = op.cjd;
            ceqbd = op.cjs;

            gbbg = -op.gbgs;
            gbbb = -op.gbbs;
            gbbe = -op.gbes;
            let gbbp = -op.gbps;
            gbbsp = -op.gbds;
            gbbdp = -(gbbg + gbbsp + gbbb + gbbe + gbbp);

            gddpg = -op.gjsg;
            gddpsp = -op.gjsd;
            gddpb = -op.gjsb;
            gddpe = 0.0;
            gddpdp = -(gddpg + gddpsp + gddpb + gddpe);

            gsspg = -op.gjdg;
            gsspdp = -op.gjdd;
            gsspb = -op.gjdb;
            gsspe = -op.gjde;
            gsspsp = -(gsspg + gsspdp + gsspb + gsspe);
        }

        // type<0: flip junction/body equivalent currents (b3soiddld.c:3997)
        let ceqbody = -op.cbody;
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
fn biases_match(a: B3SoiDdBias, b: B3SoiDdBias) -> bool {
    a.vbs == b.vbs && a.vgs == b.vgs && a.vds == b.vds && a.ves == b.ves && a.vps == b.vps
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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

    /// The CAPMOD=3 capacitance matrix must be the Jacobian of the node charges:
    /// `cXgb ≈ d(qX)/d(vg)` etc. Validates the charge derivatives by finite
    /// difference against the charges themselves, and that the four node charges
    /// conserve (`qg+qb+qd+qe+qs == 0`, with qs implied).
    #[test]
    fn charge_matrix_is_consistent_with_charges() {
        let model = B3SoiDdModel::from_params(&n1_params(), false, 300.15);
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
