//! B3SOIPD (BSIMSOI dynamic-depletion, MOS level 56) device.
//!
//! Ported from ngspice-46 `src/spicelib/devices/bsim3soi_dd/`. This module ties
//! the model card ([`params`]), the size/temperature setup ([`temp`]), and the
//! DC load equations ([`eval`]) into a [`NonlinearDevice`] that RSpice can stamp.
//!
//! # Node topology (b3soipdset.c:975-1037)
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
//!   `B3SOIPDload` DC block, including the back-gate (E) coupling columns.
//! - **CAPMOD=3 charge model** ([`eval::eval`] with `compute_charges`): the
//!   intrinsic + extrinsic + overlap charges and the coupled capacitance matrix
//!   (b3soipdld.c:2646-3784). Stamped as a transient companion by the engine's
//!   dedicated B3SOI pass; the four node charges (incl. the floating body) feed
//!   the local-truncation-error step control.
//! - **Convergence aids**: `B3SOIPDlimit` (per-iterate 0.2 V body-voltage cap)
//!   and `B3SOIPDSmartVbs` (DC floating-body `Vbs >= 0` clamp), applied each
//!   Newton iterate; SmartVbs is disabled during transient time-stepping.
//! - **Builder dispatch** is live for LEVEL=56 NMOS/PMOS (see `engine/builder.rs`
//!   `build_b3soi_dd`); levels 55/57 still fall through to the generic MOSFET.
//!
//! Verified: the DC sweeps (`t3`/`t4`/`t5`/`inv2`) match the checked-in ngspice
//! references; the `RampVg2` floating-body `@m1[vbs]` trace matches at the DC
//! anchor (t=0, ~0.0917 V) and tracks the transient (the fast-edge body
//! amplitude is still being calibrated against ngspice's body LTE). `ring51`
//! (51-stage SOI ring oscillator) does not yet reach a DC operating point.

pub use super::common;
pub use params::B3SoiPdModel;

pub mod eval;
pub mod params;
pub mod temp;

use crate::device::traits::{
    MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice,
};
use crate::{Value, circuit::NodeId};
use eval::{B3SoiPdBias, B3SoiPdOp, ModelConsts};
use std::sync::Arc;
use temp::{B3SoiPdGeometry, B3SoiPdSized};

/// Body-node configuration for one instance (b3soipdset.c node creation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyMode {
    /// Floating body: `B` is an internal node, `float = 1` (bodyMod 0).
    Floating,
    /// Ideal body tie: the external contact node is the body node (bodyMod 2).
    TiedIdeal,
    /// Nonideal body tie (bodyMod 1): an internal body node `B` sits behind the
    /// body resistor `Rbody`/`Rbsh`, with the external contact as the `P` node.
    /// The body-tie current `Ibp` couples `B` to `P` (b3soipdld.c body resistor).
    TiedResistive,
}

/// B3SOIPD device instance.
#[derive(Debug, Clone)]
pub struct B3SoiPd {
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
    pub model: Arc<B3SoiPdModel>,
    /// Size/temperature-resolved parameters (one per instance geometry).
    pub sized: Arc<B3SoiPdSized>,
    /// Model scalars needed inside the load.
    consts: ModelConsts,

    // Operating point (current iteration).
    op: B3SoiPdOp,
    // Branch voltages used at the last `update`, device polarity, pre-swap.
    bias: B3SoiPdBias,
    converged_ref: B3SoiPdBias,
    has_history: bool,
    /// Last accepted/limited `vbs` (device polarity) used as the limiter anchor
    /// for `B3SOIPDlimit`/`B3SOIPDSmartVbs` on the next Newton iterate.
    vbs_limit_anchor: Value,
    vbd_limit_anchor: Value,
    /// DC/operating-point mode: enables the `B3SOIPDSmartVbs` floating-body
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
}

impl B3SoiPd {
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
        model: Arc<B3SoiPdModel>,
        geom: B3SoiPdGeometry,
        temp_k: Value,
    ) -> Result<Self, String> {
        // Self-heating is not yet implemented; per spec it must be a hard error
        // when enabled rather than silently ignored (SHMOD=0 in supported decks).
        if model.sh_mod == 1 && geom.rth0 != 0.0 {
            return Err(format!(
                "B3SOIPD '{name}': self-heating (SHMOD=1 with RTH0!=0) is not yet implemented"
            ));
        }
        // The charge model implements CAPMOD=2 (the B3SOIPD default); the
        // CAPMOD=3 charge-thickness model is not ported. Reject rather than
        // silently simulating with the wrong charges.
        if model.cap_mod != 2 {
            return Err(format!(
                "B3SOIPD '{name}': CAPMOD={} is not implemented (only CAPMOD=2)",
                model.cap_mod
            ));
        }
        let sized = Arc::new(B3SoiPdSized::new(&model, &geom, temp_k)?);
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
            // ngspice clamps PhiBSWG model-wide to >= 0.1 (b3soipdtemp.c).
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
            op: B3SoiPdOp::default(),
            bias: B3SoiPdBias {
                vbs: 0.0,
                vgs: 0.0,
                vds: 0.0,
                ves: 0.0,
                vps: 0.0,
            },
            converged_ref: B3SoiPdBias {
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
        })
    }

    /// Select the analysis mode for the floating-body convergence aids.
    ///
    /// In DC/operating-point mode the `B3SOIPDSmartVbs` clamp (Vbs >= 0) is
    /// active; in transient it is disabled so a floating body may swing below
    /// the source potential. The per-iteration `B3SOIPDlimit` change cap applies
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
    /// Bypass is more than a speed optimization: the B3SOIPD mode select is
    /// discontinuous at `vds = 0`, so a device parked at that boundary injects
    /// a charge-current jump on every re-evaluation and Newton limit-cycles at
    /// any timestep. Freezing the evaluation once the branch voltages and
    /// predicted currents are stationary (b3soipdld.c:509-560) is how ngspice
    /// converges there.
    pub fn set_bypass_tolerances(&self, tolerances: Option<(Value, Value, Value)>) {
        self.bypass_tolerances.set(tolerances);
        self.bypass_active.set(false);
    }

    /// Mark the start of a new timestep attempt (ngspice `MODEINITPRED`): the
    /// next `update` must fully re-evaluate so bypass deltas are always
    /// measured against a state from the current timestep.
    pub fn begin_timestep_iteration(&self) {
        self.force_full_eval.set(true);
        self.bypass_active.set(false);
    }

    /// ngspice bypass predicate (b3soipdld.c:509-560): every branch-voltage
    /// delta against the previous iterate's state is inside the Newton
    /// tolerances and the linear current predictions `cdhat`/`cbhat` match the
    /// stored device currents. `bodyMod` 0/2 skips the `vps` voltage test
    /// exactly as ngspice does.
    fn bypass_check(
        &self,
        raw: B3SoiPdBias,
        reltol: Value,
        abstol: Value,
        vntol: Value,
    ) -> bool {
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

        // Linear predictions with the stored linearization (b3soipdld.c:486-507;
        // PD's cdhat has no gme*delves term and its cbhat uses gbps*delvps).
        let op = &self.op;
        let cdhat = if op.mode >= 0 {
            op.cd
                + (op.gm - op.gjdg) * delvgs
                + (op.gds - op.gjdd) * delvds
                + (op.gmbs - op.gjdb) * delvbs
        } else {
            let delvgd = (raw.vgs - raw.vds) - (old.vgs - old.vds);
            op.cd + (op.gm - op.gjdg) * delvgd - (op.gds - op.gjdd) * delvds
                + (op.gmbs - op.gjdb) * delvbd
        };
        let cbhat = op.cb
            + op.gbgs * delvgs
            + op.gbbs * delvbs
            + op.gbds * delvds
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
    /// b3soipdld.c:676-688) - mixing raw node voltages with limited-bias
    /// charges injects `ag0`-amplified phantom currents whenever the body
    /// limiter engages.
    fn charge_bias(&self, voltages: &[Value]) -> B3SoiPdBias {
        if self.bypass_active.get() {
            self.bias
        } else {
            self.branch_voltages(voltages)
        }
    }

    pub fn charge_at(&self, voltages: &[Value]) -> eval::B3SoiPdCharge {
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
    /// `B3SOIPDload` (b3soipdld.c:3679-3868, 4083-4128) for `bodyMod` 0/2, no
    /// temp node, no series R. The gate-overlap and extrinsic-substrate
    /// derivatives are already folded into `charge`'s `gc**` matrix.
    pub fn stamp_charge_companion(
        &self,
        charge: &eval::B3SoiPdCharge,
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
        // flip of the ceqq* terms (b3soipdld.c:551-552, 676-688, 738-740).
        // Raw node differences here would flip the sign of the G*v
        // linearization correction for every p-channel device and inject
        // ag0-amplified phantom currents whenever the body limiter engages.
        let bias = self.charge_bias(voltages);
        let vgb = bias.vgs - bias.vbs;
        let vbd = bias.vbs - bias.vds;
        let vbs = bias.vbs;
        let veb = bias.ves - bias.vbs;

        // gc** are multiplied by ag0 (b3soipdld.c:3680-3766).
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

        // Equivalent charge currents (b3soipdld.c:3860-3867). type<0 flips sign.
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

        // RHS (b3soipdld.c:4011-4017, charge parts).
        stamp_rhs(matrix, b, -ceqqb);
        stamp_rhs(matrix, g, -ceqqg);
        stamp_rhs(matrix, dp, -ceqqd);
        stamp_rhs(matrix, sp, ceqqg + ceqqb + ceqqd + ceqqe);
        stamp_rhs(matrix, e, -ceqqe);

        // Matrix charge entries (b3soipdld.c:4083-4128, gc** parts only).
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
    /// (b3soipdld.c:483-498). Source-referenced, `mtype` folded in.
    ///
    /// The solution vector is 0-indexed (node 1 -> `v[0]`); ground (NodeId 0)
    /// reads as 0.0.
    fn raw_branch_voltages(&self, v: &[Value]) -> B3SoiPdBias {
        let node = |n: NodeId| if n == 0 { 0.0 } else { v[n - 1] };
        let vd = node(self.node_drain);
        let vg = node(self.node_gate);
        let vs = node(self.node_source);
        let ve = node(self.node_e);
        let vb = node(self.node_body);
        let vp = node(self.node_p);
        B3SoiPdBias {
            vbs: self.mtype * (vb - vs),
            vgs: self.mtype * (vg - vs),
            vds: self.mtype * (vd - vs),
            ves: self.mtype * (ve - vs),
            vps: self.mtype * (vp - vs),
        }
    }

    fn branch_voltages(&self, v: &[Value]) -> B3SoiPdBias {
        let mut bias = self.raw_branch_voltages(v);
        let _ = self.apply_body_limiting(&mut bias);
        bias
    }

    /// Floating-body convergence aids `B3SOIPDlimit` + `B3SOIPDSmartVbs`
    /// (b3soipdld.c:50-99, 664-688), applied per Newton iterate.
    ///
    /// In the mode-selected (normal/inverse) frame, the body-source (or
    /// body-drain) voltage is clamped to move at most 0.2 V from the previous
    /// iterate's value and, in DC, floored at 0 for a floating body. This only
    /// reshapes the Newton path; the converged solution still satisfies KCL.
    ///
    /// Returns whether the per-iteration change cap actually engaged (the
    /// ngspice `Check` flag; the SmartVbs DC floor intentionally does not set
    /// it, matching `B3SOIPDSmartVbs`'s `NG_IGNORE(check)`).
    fn apply_body_limiting(&self, bias: &mut B3SoiPdBias) -> bool {
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

impl NonlinearDevice for B3SoiPd {
    fn update(&mut self, voltages: &[Value]) {
        self.converged_ref = self.bias;
        // ngspice transient bypass (b3soipdld.c:509-560): when the previous
        // iterate evaluated without limiting and the new branch voltages plus
        // predicted currents are stationary within tolerances, freeze the
        // evaluation (bias, op, mode). This is what lets Newton contract on a
        // device parked at the discontinuous vds = 0 mode boundary.
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
        let cmp = |new: Value, old: Value| {
            (new - old).abs() < reltol * new.abs().max(old.abs()) + vtol
        };
        cmp(self.bias.vbs, self.converged_ref.vbs)
            && cmp(self.bias.vgs, self.converged_ref.vgs)
            && cmp(self.bias.vds, self.converged_ref.vds)
            && cmp(self.bias.ves, self.converged_ref.ves)
            && cmp(self.bias.vps, self.converged_ref.vps)
    }
}

impl B3SoiPd {
    /// Stamp the linearized DC operating point.
    ///
    /// Faithful transcription of the DC portion of the B3SOIPD matrix/RHS load
    /// (b3soipdld.c:3886-4150) for `bodyMod` 0/2, no temp node, no series R, and
    /// `ChargeComputationNeeded == 0` (so all `gc*`/`ceqq*` charge terms vanish).
    /// `m` (multiplier) is 1. CKTgmin terms are omitted: the RSpice solver adds
    /// its own diagonal gmin, so replicating ngspice's per-device gmin would
    /// double-count it.
    ///
    /// In DC `dNodePrime == dNode`, `sNodePrime == sNode`, `bNode` is the body
    /// node (internal floating or external tie), and `pNode`/`tempNode` are
    /// absent.
    fn stamp_op(&self, op: &B3SoiPdOp, bias: B3SoiPdBias, matrix: &mut impl MatrixStamper) {
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

        // ----- conductance groups (b3soipdld.c:3888-3944) -----
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

        // type<0: flip junction/body equivalent currents (b3soipdld.c:3997)
        let ceqbody = -op.cbody;
        let (ceqbs, ceqbd, ceqbody) = if mt < 0.0 {
            (-ceqbs, -ceqbd, -ceqbody)
        } else {
            (ceqbs, ceqbd, ceqbody)
        };

        // ----- RHS (b3soipdld.c:4011-4017) -----
        // Routed through the stamper's `stamp_rhs` so the 1-indexed NodeId is
        // mapped to the 0-indexed RHS vector consistently with the matrix stamp.
        stamp_rhs(matrix, b, -ceqbody);
        stamp_rhs(matrix, dp, ceqbd - cdreq);
        stamp_rhs(matrix, sp, cdreq + ceqbs);

        // ----- matrix (b3soipdld.c:4090-4128, DC: gc*=0) -----
        // E row: only EePtr (gceeb==0) -> nothing in DC.
        // DP/SP body columns (b3soipdld.c:4092-4093):
        stamp(matrix, dp, b, -(-gddpb - gmbs));
        stamp(matrix, sp, b, -(-gsspb + gmbs));
        // Body row (b3soipdld.c:4094-4098):
        stamp(matrix, b, e, gbbe);
        stamp(matrix, b, g, gbbg);
        stamp(matrix, b, dp, gbbdp);
        stamp(matrix, b, sp, gbbsp);
        stamp(matrix, b, b, gbbb);

        // Gate row: gc* all zero in DC -> nothing.

        // Drain-prime row (b3soipdld.c:4106-4112):
        stamp(matrix, dp, g, gm + gddpg);
        stamp(matrix, dp, dp, op.gds + gddpdp + rev_sum);
        stamp(matrix, dp, sp, -(-gddpsp + op.gds + fwd_sum));

        // Source-prime row (b3soipdld.c:4114-4119):
        stamp(matrix, sp, g, -gm + gsspg);
        stamp(matrix, sp, dp, -(op.gds - gsspdp + rev_sum));
        stamp(matrix, sp, sp, op.gds + gsspsp + fwd_sum);

        // Back-gate (E) coupling columns of the drain/source-prime rows
        // (b3soipdld.c:4087-4088, DC: gc*=0). `gme` carries the mode sign.
        stamp(matrix, dp, e, gme + gddpe);
        stamp(matrix, sp, e, gsspe - gme);

        // Body-tie resistor (bodyMod 1): a linear conductance between the
        // internal body node `b` and the external contact `p` carrying
        // Ibp = Vbp / (Rbody + Rbodyext) (b3soipdld.c:2017-2042, 4131-4137). Pure
        // resistor, no equivalent-current source needed. Absent for floating
        // (bodyMod 0) and ideal-tie (bodyMod 2) devices.
        if self.body_mode == BodyMode::TiedResistive {
            let p = self.node_p;
            let rtot = self.sized.rbody + self.sized.rbodyext;
            let gbp = if rtot > 1e-30 { 1.0 / rtot } else { 1.0 / 1e-30 };
            stamp(matrix, b, b, gbp);
            stamp(matrix, b, p, -gbp);
            stamp(matrix, p, b, -gbp);
            stamp(matrix, p, p, gbp);
        }
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
fn biases_match(a: B3SoiPdBias, b: B3SoiPdBias) -> bool {
    a.vbs == b.vbs && a.vgs == b.vgs && a.vds == b.vds && a.ves == b.ves && a.vps == b.vps
}

#[cfg(test)]
mod tests;
