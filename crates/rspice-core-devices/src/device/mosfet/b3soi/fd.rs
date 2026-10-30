//! B3SOIFD (BSIMSOI fully-depleted, MOS level 55) device.
//!
//! Ported from ngspice-46 `src/spicelib/devices/bsim3soi_fd/`. This module ties
//! the model card ([`params`]), the size/temperature setup ([`temp`]), and the
//! DC load equations ([`eval`]) into a [`NonlinearDevice`] that RSpice can stamp.
//!
//! # FD vs DD: the fully-depleted body
//!
//! The defining difference between the FD (level 55) and DD (level 56) variants
//! is the body. In DD the floating body is a real circuit node whose potential
//! is solved by Newton iteration and coupled through impact ionization, GIDL and
//! the source/drain body diodes. In FD the body is **fully depleted** and sits in
//! quasi-static equilibrium, so ngspice:
//!
//! 1. **Creates no body node** for the floating-body case (`b3soifdset.c`:
//!    `bNode = pNode = 0`), versus DD which allocates an internal `Body` node.
//! 2. **Pins the body voltage algebraically** to the equilibrium value
//!    `Vbs = Vbsdio = Vbs0eff` with `dVbsdio_dVb = 0` (`b3soifdld.c:1087-1095`),
//!    versus DD's smoothed body-node tracking (`b3soiddld.c:1119-1129`).
//! 3. **Disables every body current** — impact ionization, GIDL, body diodes,
//!    parasitic BJT and the body-tie resistor are all forced to zero in the DC
//!    path (`b3soifdld.c:2121-2145`).
//! 4. **Stamps no body row/column** in the matrix: the load fills only the
//!    drain/gate/source/back-gate entries (`b3soifdld.c:3197-3245`), and the
//!    body charge `ceqbody`/`qbody` is never routed to a node.
//!
//! The net effect is a BSIM3-style SOI MOSFET with four active terminals
//! (drain `d`, gate `g`, source `s`, back-gate/substrate `e`) whose threshold and
//! body-bias dependence flow entirely through the algebraically-determined
//! `Vbs0eff`. A body-contact node, when present (`m d g s e b`), is read for the
//! initial guess but immediately overridden, so it only ever connects to whatever
//! external network drives it.
//!
//! # Status
//!
//! - **DC current path** ([`eval::eval_dc`]): faithful transcription of the
//!   `B3SOIFDload` DC block (channel current `Ids` plus the FD body pinning and
//!   zeroed body currents), including the back-gate (E) coupling columns.
//! - **CAPMOD=2/3 charge models** ([`eval::eval`] with `compute_charges`): the
//!   gate/drain/back-gate node charges and capacitance matrix are integrated by
//!   the engine's B3SOI transient pass. FD pins the intrinsic junction stored
//!   charge (`qjs = qjd = 0`) and never stamps the (algebraically-pinned) body
//!   charge to a body row.
//! - **Builder dispatch** is live for LEVEL=55 NMOS/PMOS (`engine/builder.rs`
//!   `build_b3soi_fd`).
//! - **Drain/source sheet resistance** is externalized by the builder as
//!   ordinary terminal-to-prime resistors from `RSH * NRD/NRS`.

pub use super::common;
pub use params::B3SoiFdModel;

pub mod eval;
pub mod params;
pub mod temp;

use crate::device::mosfet::b3soi::common::{
    SoiCompanionCurrents, SoiInstanceCore, biases_match, stamp, stamp_conductance, stamp_rhs,
};
use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::{NodeId, Value};
use eval::{B3SoiFdBias, B3SoiFdOp, ModelConsts};
use std::borrow::Cow;
use std::sync::Arc;
use temp::{B3SoiFdGeometry, B3SoiFdSized};

/// Body-node configuration for one instance (b3soifdset.c node creation).
///
/// Unlike DD, FD never solves the body as a circuit node; this only records how
/// the (overridden) initial body voltage is read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyMode {
    /// Floating body: no body node exists (`bodyMod 0`, `bNode = 0`).
    Floating,
    /// Body contact present: the external contact node is read for the initial
    /// guess (`bodyMod 2`) but the load pins `Vbs = Vbs0eff` regardless.
    TiedIdeal,
}

/// B3SOIFD device instance.
#[derive(Debug, Clone)]
pub struct B3SoiFd {
    core: SoiInstanceCore,

    pub name: String,
    /// +1 NMOS / -1 PMOS (model `mtype`).
    pub mtype: Value,

    // External nodes (already resolved to NodeId by the builder).
    pub node_drain: NodeId,
    pub node_gate: NodeId,
    pub node_source: NodeId,
    /// Back-gate / substrate-under-BOX node `E`.
    pub node_e: NodeId,
    /// Optional self-heating temperature-rise node (`delTemp`).
    pub node_temp: NodeId,
    /// Body-contact node `P`/`B`: read for the initial guess only (0 when
    /// floating). FD never stamps a body row/column.
    pub node_body: NodeId,

    pub body_mode: BodyMode,

    /// Shared model card (one per `.model`).
    pub model: Arc<B3SoiFdModel>,
    /// Size/temperature-resolved parameters (one per instance geometry).
    pub sized: Arc<B3SoiFdSized>,
    base_temp_k: Value,
    /// Model scalars needed inside the load.
    consts: ModelConsts,

    // Operating point (current iteration).
    op: B3SoiFdOp,
    bias: B3SoiFdBias,
    converged_ref: B3SoiFdBias,
    has_history: bool,
    /// Last accepted/limited self-heating temperature rise used by
    /// `B3SOIFDlimit(delTemp, oldDelTemp, 5.0)`.
    del_temp_limit_anchor: Value,
    /// The previous iterate's limited `[vg, vd, vs, ve]` node voltages, which
    /// `B3SOIFDlimit(v, vold, 3.0)` holds each terminal within 3 V of
    /// (b3soifdld.c:454-462).
    terminal_limit_anchor: [Value; 4],
    /// Whether the limiter anchors have been seeded.
    limit_anchor_valid: std::cell::Cell<bool>,
    /// The `OFF` startup load has not been stamped yet. Without this guard
    /// `stamp_nonlinear` would re-evaluate from the raw node seed and discard
    /// the startup branch before it reached the matrix.
    startup_seed_pending: std::cell::Cell<bool>,
    /// Raw `[vg, vd, vs, ve, delTemp]` of the iterate the last operating-point
    /// `update` evaluated, until a matrix load or a second `update` sees it.
    open_dc_update: std::cell::Cell<Option<[Value; 5]>>,
}

super::common::deref_core!(B3SoiFd => SoiInstanceCore);

impl B3SoiFd {
    /// Build an instance from a model card and instance geometry.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        node_drain: NodeId,
        node_gate: NodeId,
        node_source: NodeId,
        node_e: NodeId,
        node_temp: NodeId,
        node_body: NodeId,
        body_mode: BodyMode,
        model: Arc<B3SoiFdModel>,
        geom: B3SoiFdGeometry,
        temp_k: Value,
    ) -> Result<Self, String> {
        if model.sh_mod == 1 && geom.rth0 != 0.0 && node_temp == 0 {
            return Err(format!(
                "B3SOIFD '{name}': self-heating (SHMOD=1 with RTH0!=0) requires a temperature node"
            ));
        }
        let sized = Arc::new(B3SoiFdSized::new(&model, &geom, temp_k)?);
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
            mtype: model.mtype,
        };
        Ok(Self {
            core: SoiInstanceCore::new(true),
            name,
            mtype: model.mtype,
            node_drain,
            node_gate,
            node_source,
            node_e,
            node_temp,
            node_body,
            body_mode,
            model,
            sized,
            base_temp_k: temp_k,
            consts,
            op: B3SoiFdOp::default(),
            bias: B3SoiFdBias::default(),
            converged_ref: B3SoiFdBias::default(),
            has_history: false,
            del_temp_limit_anchor: 0.0,
            terminal_limit_anchor: [0.0; 4],
            limit_anchor_valid: std::cell::Cell::new(false),
            startup_seed_pending: std::cell::Cell::new(false),
            open_dc_update: std::cell::Cell::new(None),
        })
    }

    /// Select the analysis mode (kept for parity with the DD device; FD has no
    /// body-node convergence aids to toggle).
    pub fn set_dc_mode(&self, dc: bool) {
        if self.dc_mode.replace(dc) == dc {
            return;
        }
        self.limit_anchor_valid.set(false);
        self.open_dc_update.set(None);
        self.bypass_active.set(false);
        self.force_full_eval.set(true);
    }

    pub fn self_heating_active(&self) -> bool {
        !self.self_heating_startup_disabled.get() && self.has_self_heating_node()
    }

    #[doc(hidden)]
    pub fn has_self_heating_node(&self) -> bool {
        self.node_temp != 0 && self.sized.rth.is_finite() && self.sized.rth > 0.0
    }

    /// Seed the temperature-rise node from the isothermal dissipation
    /// `Ids * Vds * Rth` at `solution`.
    #[doc(hidden)]
    pub fn seed_self_heating_temperature_from_power(&self, solution: &mut [Value]) {
        if !self.has_self_heating_node() {
            return;
        }
        let mut bias = self.raw_branch_voltages(solution);
        bias.del_temp = 0.0;
        let sized = self.sized_for_bias(bias);
        let op = eval::eval_dc(&sized, &self.consts, bias, self.mtype);
        let vds = if op.mode >= 0 { bias.vds } else { -bias.vds };
        let delta_temp = (op.ids * vds * self.sized.rth).clamp(0.0, 1.0e3);
        if delta_temp.is_finite()
            && let Some(slot) = solution.get_mut(self.node_temp - 1)
        {
            *slot = delta_temp;
        }
    }

    /// Clear the cached operating-point linearization before a fresh DC path.
    #[doc(hidden)]
    pub fn reset_operating_point_history(&mut self) {
        self.op = B3SoiFdOp::default();
        self.bias = B3SoiFdBias::default();
        self.converged_ref = B3SoiFdBias::default();
        self.has_history = false;
        self.del_temp_limit_anchor = 0.0;
        self.terminal_limit_anchor = [0.0; 4];
        self.dc_mode.set(true);
        self.limit_anchor_valid.set(false);
        self.open_dc_update.set(None);
        self.startup_seed_pending.set(false);
        self.bypass_active.set(false);
        self.force_full_eval.set(true);
        self.last_limited.set(false);
    }

    /// Linearize at `solution` and anchor the limiters there, so Newton
    /// continues from a solution found by a startup aid.
    #[doc(hidden)]
    pub fn prime_operating_point_from_solution(&mut self, solution: &[Value]) {
        let bias = self.raw_branch_voltages(solution);
        self.converged_ref = bias;
        self.bias = bias;
        self.op = self.eval_op_for_bias(bias);
        self.has_history = true;
        self.del_temp_limit_anchor = bias.del_temp;
        self.terminal_limit_anchor = self.raw_terminals(solution);
        self.limit_anchor_valid.set(true);
        self.open_dc_update.set(None);
        self.startup_seed_pending.set(false);
        self.bypass_active.set(false);
        self.force_full_eval.set(false);
        self.last_limited.set(false);
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
    /// Regular Newton updates intentionally use the BSIMSOI temperature limiter.
    /// Residual and fallback validation probes must evaluate the true
    /// operating-point equations at the candidate voltage itself.
    #[doc(hidden)]
    pub fn update_static_linearization(&mut self, voltages: &[Value]) {
        self.converged_ref = self.bias;
        self.startup_seed_pending.set(false);
        let bias = self.raw_branch_voltages(voltages);
        self.bias = bias;
        self.op = self.eval_op_for_bias(bias);
        self.has_history = true;
        self.del_temp_limit_anchor = bias.del_temp;
        self.terminal_limit_anchor = self.raw_terminals(voltages);
        self.limit_anchor_valid.set(true);
        self.open_dc_update.set(None);
        self.bypass_active.set(false);
        self.force_full_eval.set(false);
        self.last_limited.set(false);
    }

    /// ngspice bypass predicate (b3soifdld.c:512-560): every branch-voltage
    /// delta against the previous iterate's state is inside the Newton
    /// tolerances and the linear current predictions `cdhat`/`cbhat` match the
    /// stored device currents. FD has no `vps` branch (`vps == 0` always).
    fn bypass_check(&self, raw: B3SoiFdBias, reltol: Value, abstol: Value, vntol: Value) -> bool {
        let vtol = |new: Value, old: Value| reltol * new.abs().max(old.abs()) + vntol;
        let old = &self.bias;
        let delvbs = raw.vbs - old.vbs;
        let delvds = raw.vds - old.vds;
        let delvgs = raw.vgs - old.vgs;
        let delves = raw.ves - old.ves;
        let vbd_new = raw.vbs - raw.vds;
        let vbd_old = old.vbs - old.vds;
        let delvbd = vbd_new - vbd_old;
        if delvbs.abs() >= vtol(raw.vbs, old.vbs)
            || delvbd.abs() >= vtol(vbd_new, vbd_old)
            || delvgs.abs() >= vtol(raw.vgs, old.vgs)
            || delves.abs() >= vtol(raw.ves, old.ves)
            || delvds.abs() >= vtol(raw.vds, old.vds)
            || (self.self_heating_active()
                && (raw.del_temp - old.del_temp).abs() >= vtol(raw.del_temp, old.del_temp))
        {
            return false;
        }

        // Linear predictions with the stored linearization (b3soifdld.c:512-532).
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
        let cbhat =
            op.cb + op.gbgs * delvgs + op.gbbs * delvbs + op.gbds * delvds + op.gbes * delves;
        (cdhat - op.cd).abs() < reltol * cdhat.abs().max(op.cd.abs()) + abstol
            && (cbhat - op.cb).abs() < reltol * cbhat.abs().max(op.cb.abs()) + abstol
    }

    /// Terminal NodeIds in (drain, gate, source, e, body) order. The body slot is
    /// the (possibly 0) contact node; FD does not treat it as a charge node.
    pub fn charge_nodes(&self) -> (NodeId, NodeId, NodeId, NodeId, NodeId) {
        (
            self.node_drain,
            self.node_gate,
            self.node_source,
            self.node_e,
            self.node_body,
        )
    }

    /// The bias the transient charge evaluation and its companion stamp use:
    /// the frozen iterate under bypass or the limited iterate `update`
    /// evaluated, otherwise the branch voltages. The companion's `ceqq*`
    /// linearization corrections must be formed from this same bias.
    fn charge_bias(&self, voltages: &[Value]) -> B3SoiFdBias {
        if self.bypass_active.get() || self.last_limited.get() {
            self.bias
        } else {
            self.branch_voltages(voltages)
        }
    }

    /// Evaluate the configured CAPMOD charge state at the given solution vector.
    pub fn charge_at(&self, voltages: &[Value]) -> eval::B3SoiFdCharge {
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

    /// Absolute temperature used by the cached accepted electrical evaluation.
    #[doc(hidden)]
    pub fn operating_temperature_kelvin(&self) -> Value {
        if self.self_heating_active() {
            self.base_temp_k + self.bias.del_temp
        } else {
            self.sized.temp
        }
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
        crate::op_label::OpLabel,
    ) {
        let op = &self.op;
        let bias = self.bias;
        let region = common::op_report_region(bias, op.mode, op.von, op.vdsat);
        (
            op.ids, bias.vgs, bias.vds, bias.vbs, op.von, op.vdsat, op.gm, op.gds, op.gmbs, region,
        )
    }

    /// Stamp the transient charge companion into the matrix and RHS.
    ///
    /// Mirrors the charge portion of `B3SOIFDload` (b3soifdld.c:3131-3245) for the
    /// floating/contact FD body: the gate, drain-prime, source-prime and back-gate
    /// (E) charge entries — but NO body row/column (FD has no body charge node).
    #[allow(clippy::too_many_arguments)]
    #[doc(hidden)]
    pub fn stamp_charge_companion(
        &self,
        charge: &eval::B3SoiFdCharge,
        ag0: Value,
        currents: SoiCompanionCurrents,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
    ) {
        let SoiCompanionCurrents {
            cqg,
            cqb,
            cqd,
            cqe,
            cqth,
        } = currents;
        let (dp, g, sp, e, _b) = self.charge_nodes();
        // FD pins the body to its equilibrium value; the charge model evaluates
        // the `gc**` derivatives in the source-referenced body frame exactly as
        // DD, but with the body voltage taken from the device's pinned `vbs`.
        // Branch voltages are device-polarity (type-folded), exactly as ngspice
        // forms them from the folded vgs/vbs/vds/ves state before the type<0
        // sign flip of the ceqq* terms (b3soifdld.c). Raw node differences here
        // would flip the sign of the G*v linearization correction for every
        // p-channel device. Under bypass the frozen iterate is reused.
        let bias = self.charge_bias(voltages);
        let vgb = bias.vgs - bias.vbs;
        let vbd = bias.vbs - bias.vds;
        let vbs = bias.vbs;
        let veb = bias.ves - bias.vbs;

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
        // A node charge is `type * q`, so its temperature column carries the
        // type sign, and its `delTemp` term is subtracted after the type flip
        // so that the right-hand side matches that column. ngspice-46 FD signs
        // the column but subtracts it inside the flip (b3soifdld.c:2804-2886,
        // 2980-2987, 3125), which leaves every PMOS charge current off by
        // 2*gc_t*delTemp. B4SOI's g/d/s rows form this right-hand side, the
        // sign dropped inside the flip (b4soild.c:9529-9546), but stamp the
        // unsigned value on the column (b4soild.c:10662-10666), so B4SOI's
        // own column is inconsistent with it.
        let mt = self.mtype;
        let gcg_t = mt * c.gcg_t * ag0;
        let gcb_t = mt * c.gcb_t * ag0;
        let gcd_t = mt * c.gcd_t * ag0;
        let gce_t = mt * c.gce_t * ag0;
        let gcs_t = -(gcg_t + gcb_t + gcd_t + gce_t);

        // Equivalent charge currents (b3soifdld.c:3131-3140). The body charge
        // `cqb`/`ceqqb` is folded into the source node (current sum) as ngspice
        // does, but never stamped to a body node.
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
        ceqqg -= gcg_t * bias.del_temp;
        ceqqb -= gcb_t * bias.del_temp;
        ceqqd -= gcd_t * bias.del_temp;
        ceqqe -= gce_t * bias.del_temp;
        let gc_tt = self.thermal_capacitance() * ag0;
        let ceqqth = cqth - gc_tt * bias.del_temp;

        // RHS (b3soifdld.c:3131-3140): g, dp, sp (sum), e. No body node.
        stamp_rhs(matrix, g, -ceqqg);
        stamp_rhs(matrix, dp, -ceqqd);
        stamp_rhs(matrix, sp, ceqqg + ceqqb + ceqqd + ceqqe);
        stamp_rhs(matrix, e, -ceqqe);
        stamp_rhs(matrix, self.node_temp, -ceqqth);

        // Matrix charge entries (b3soifdld.c:3201-3243): the gate/drain/source/E
        // charge rows. The body-reference column is simply dropped — FD has no
        // body node, so each row stamps only its g/d/s/e columns (`gc*gb`,
        // `gc*db`, `gc*sb`, `gc*eb`); the body's contribution closes through the
        // `ceqq*` RHS terms. This mirrors ngspice exactly (no `Bx`/`*b` pointer).
        // Thermal row.
        stamp(matrix, self.node_temp, self.node_temp, gc_tt);
        // E row.
        stamp(matrix, e, g, gcegb);
        stamp(matrix, e, dp, gcedb);
        stamp(matrix, e, sp, gcesb);
        stamp(matrix, e, e, gceeb);
        stamp(matrix, e, self.node_temp, gce_t);
        // G row.
        stamp(matrix, g, e, gcgeb);
        stamp(matrix, g, g, gcggb);
        stamp(matrix, g, dp, gcgdb);
        stamp(matrix, g, sp, gcgsb);
        stamp(matrix, g, self.node_temp, gcg_t);
        // DP row.
        stamp(matrix, dp, e, gcdeb);
        stamp(matrix, dp, g, gcdgb);
        stamp(matrix, dp, dp, gcddb);
        stamp(matrix, dp, sp, gcdsb);
        stamp(matrix, dp, self.node_temp, gcd_t);
        // SP row.
        stamp(matrix, sp, e, gcseb);
        stamp(matrix, sp, g, gcsgb);
        stamp(matrix, sp, dp, gcsdb);
        stamp(matrix, sp, sp, gcssb);
        stamp(matrix, sp, self.node_temp, gcs_t);
    }

    /// Extract device-polarity branch voltages from the solution vector.
    ///
    /// Source-referenced, `mtype` folded in (b3soifdld.c:466-472). The body
    /// voltage read here is only the limiter seed; the FD load pins it to
    /// `Vbs0eff` internally, so its node value is immaterial.
    fn raw_branch_voltages(&self, v: &[Value]) -> B3SoiFdBias {
        let node = |n: NodeId| if n == 0 { 0.0 } else { v[n - 1] };
        let vd = node(self.node_drain);
        let vg = node(self.node_gate);
        let vs = node(self.node_source);
        let ve = node(self.node_e);
        let vb = node(self.node_body);
        let vt = node(self.node_temp);
        B3SoiFdBias {
            vbs: self.mtype * (vb - vs),
            vgs: self.mtype * (vg - vs),
            vds: self.mtype * (vd - vs),
            ves: self.mtype * (ve - vs),
            vps: 0.0,
            del_temp: vt,
        }
    }

    fn branch_voltages(&self, v: &[Value]) -> B3SoiFdBias {
        let mut bias = self.raw_branch_voltages(v);
        let _ = self.apply_temp_limiting(&mut bias);
        bias
    }

    fn sized_for_bias(&self, bias: B3SoiFdBias) -> Cow<'_, B3SoiFdSized> {
        if !self.self_heating_active() {
            return Cow::Borrowed(self.sized.as_ref());
        }

        // Even at zero rise the load takes its self-heated forms.
        Cow::Owned(
            self.sized
                .self_heated_at(&self.model, self.base_temp_k + bias.del_temp),
        )
    }

    fn eval_op_for_bias(&self, bias: B3SoiFdBias) -> B3SoiFdOp {
        let sized = self.sized_for_bias(bias);
        let mut op = eval::eval_dc(&sized, &self.consts, bias, self.mtype);
        if self.self_heating_active() {
            self.fill_electrothermal_derivatives(&mut op, bias);
        }
        op
    }

    fn fill_electrothermal_derivatives(&self, op: &mut B3SoiFdOp, bias: B3SoiFdBias) {
        let temp = self.base_temp_k + bias.del_temp;
        let h = (temp.abs().max(1.0) * 1.0e-5).clamp(1.0e-3, 5.0e-2);
        let lower_h = if temp - h > 1.0 { h } else { 0.0 };

        let sample = |del_temp: Value| {
            let sample_bias = B3SoiFdBias { del_temp, ..bias };
            let sized = self
                .sized
                .self_heated_at(&self.model, self.base_temp_k + del_temp);
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
            (
                sample_op.cdrain,
                drain_junction,
                source_junction,
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
        let ids_t = (plus.3 - minus.3) / denom;
        // The power row is formed against the mode-selected voltages
        // (b3soifdld.c:2163-2172 read `Vds`/`Vgs`/`Vbs`/`Ves` as swapped at
        // b3soifdld.c:810-829), as the load's own `gtemp*` are.
        let (vds, vgs, vbs, ves) = if op.mode >= 0 {
            (bias.vds, bias.vgs, bias.vbs, bias.ves)
        } else {
            (
                -bias.vds,
                bias.vgs - bias.vds,
                bias.vbs - bias.vds,
                bias.ves - bias.vds,
            )
        };
        op.gtemp_t = -ids_t * vds;
        op.thermal_eq_current = -op.ids * vds
            - self.mtype
                * (op.gtemp_g * vgs + op.gtemp_b * vbs + op.gtemp_e * ves + op.gtemp_d * vds)
            - op.gtemp_t * bias.del_temp;
    }

    fn fill_charge_thermal_derivatives(&self, charge: &mut eval::B3SoiFdCharge, bias: B3SoiFdBias) {
        let temp = self.base_temp_k + bias.del_temp;
        let h = (temp.abs().max(1.0) * 1.0e-5).clamp(1.0e-3, 5.0e-2);
        let lower_h = if temp - h > 1.0 { h } else { 0.0 };

        let sample = |del_temp: Value| {
            let sample_bias = B3SoiFdBias { del_temp, ..bias };
            let sized = self
                .sized
                .self_heated_at(&self.model, self.base_temp_k + del_temp);
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

    fn apply_temp_limiting(&self, bias: &mut B3SoiFdBias) -> bool {
        if !self.self_heating_active() || !self.limit_anchor_valid.get() {
            return false;
        }

        let mut check = false;
        bias.del_temp =
            common::soi_limit(bias.del_temp, self.del_temp_limit_anchor, 5.0, &mut check);
        check
    }
}

impl NonlinearDevice for B3SoiFd {
    fn update(&mut self, voltages: &[Value]) {
        // b3soifdld.c:377 loads a marked instance at
        // `delTemp = vps = vbs = vgs = vds = ves = 0` on MODEINITJCT, in every
        // compatibility mode. The startup state survives repeated updates at
        // the same solution — the operating-point seed is primed and then
        // re-evaluated before anything is stamped — and the stamp consumes it,
        // after which Newton owns the device again. FD keeps no seed for an
        // unmarked instance, so this arm belongs to the keyword alone.
        if self.startup_seed_pending.get() && self.dc_mode.get() {
            self.converged_ref = self.bias;
            return;
        }
        if self.initial_off && !self.has_history && self.dc_mode.get() {
            self.converged_ref = self.bias;
            let bias = B3SoiFdBias {
                vbs: 0.0,
                vgs: 0.0,
                vds: 0.0,
                ves: 0.0,
                vps: 0.0,
                del_temp: 0.0,
            };
            self.bias = bias;
            self.op = self.eval_op_for_bias(bias);
            self.has_history = true;
            self.del_temp_limit_anchor = bias.del_temp;
            // b3soifdld.c:380 stores `vg = vd = vs = vp = ve = 0` for this arm.
            self.terminal_limit_anchor = [0.0; 4];
            self.limit_anchor_valid.set(true);
            self.startup_seed_pending.set(true);
            self.bypass_active.set(false);
            self.force_full_eval.set(false);
            self.last_limited.set(false);
            return;
        }
        // One Newton iterate reaches `update` twice, after its solve and again
        // before its stamp. b3soifdld.c limits each load once against
        // CKTstate0, so the second pass of a limited operating-point iterate
        // keeps the first pass's evaluation instead of limiting the same raw
        // voltages again against its own output (the DD and PD variants hold
        // the same invariant through `dc_stamp_reuse_pending`). After a matrix
        // load the same raw voltages are the next iteration.
        let raw = self.raw_limiter_inputs(voltages);
        if self.dc_mode.get() && self.last_limited.get() && self.open_dc_update.take() == Some(raw)
        {
            self.converged_ref = self.bias;
            return;
        }
        self.converged_ref = self.bias;
        let (bias, terminals, limited) = self.limited_branch_voltages(voltages);
        // ngspice transient bypass (b3soifdld.c:560-618): when the new branch
        // voltages plus predicted currents are stationary within tolerances,
        // freeze the evaluation (bias, op, mode) and restamp it. ngspice keeps
        // the charge work only on the transient/AC branch (line755) and skips
        // it outright for the operating point (line850), so restamping a frozen
        // linearization belongs to the former and stays out of the DC solve.
        if let Some((reltol, abstol, vntol)) = self.bypass_tolerances.get()
            && !self.dc_mode.get()
            && !self.force_full_eval.get()
            && !self.last_limited.get()
            && self.has_history
            && self.bypass_check(bias, reltol, abstol, vntol)
        {
            self.bypass_active.set(true);
            self.bypass_hits.set(self.bypass_hits.get() + 1);
            return;
        }
        self.bypass_active.set(false);
        self.force_full_eval.set(false);
        self.last_limited.set(limited);
        self.bias = bias;
        self.op = self.eval_op_for_bias(bias);
        self.has_history = true;
        self.del_temp_limit_anchor = bias.del_temp;
        self.terminal_limit_anchor = terminals;
        self.limit_anchor_valid.set(true);
        self.open_dc_update.set(self.dc_mode.get().then_some(raw));
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        // The load closes the iterate: an update at the same raw voltages
        // after it is the next Newton iteration.
        self.open_dc_update.set(None);
        if self.bypass_active.get() {
            // Bypassed iterate: restamp the frozen linearization unchanged.
            self.stamp_op(&self.op, self.bias, matrix);
            self.stamp_instance_ic(matrix);
            return;
        }
        if self.startup_seed_pending.replace(false) {
            self.stamp_op(&self.op, self.bias, matrix);
            self.stamp_instance_ic(matrix);
            return;
        }
        // A limited iterate is stamped where `update` evaluated it, not at the
        // raw node voltages.
        let (op, bias) = if self.last_limited.get() {
            (self.op.clone(), self.bias)
        } else {
            let bias = self.branch_voltages(voltages);
            if biases_match(bias, self.bias) {
                (self.op.clone(), bias)
            } else {
                (self.eval_op_for_bias(bias), bias)
            }
        };
        self.stamp_op(&op, bias, matrix);
        self.stamp_instance_ic(matrix);
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        // An iterate the limiter moved is not converged (b3soifdld.c:2745-2747).
        if !self.has_history || self.last_limited.get() {
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
            && (!self.self_heating_active() || cmp(self.bias.del_temp, self.converged_ref.del_temp))
    }
}

impl B3SoiFd {
    /// The `[vg, vd, vs, ve]` node voltages of a solution vector.
    /// The raw voltages the limiters act on: the four terminals and the
    /// temperature rise.
    fn raw_limiter_inputs(&self, v: &[Value]) -> [Value; 5] {
        let [vg, vd, vs, ve] = self.raw_terminals(v);
        [vg, vd, vs, ve, self.raw_branch_voltages(v).del_temp]
    }

    fn raw_terminals(&self, v: &[Value]) -> [Value; 4] {
        let node = |n: NodeId| if n == 0 { 0.0 } else { v[n - 1] };
        [
            node(self.node_gate),
            node(self.node_drain),
            node(self.node_source),
            node(self.node_e),
        ]
    }

    /// A Newton iterate's branch voltages: each terminal held within 3 V of the
    /// previous iterate (b3soifdld.c:454-462), then the temperature limiter.
    /// Returns the bias, its terminals, and whether a limiter moved it. `vbs`
    /// stays the raw body-node difference ngspice reads (b3soifdld.c:466);
    /// FD pins its body internally.
    fn limited_branch_voltages(&self, v: &[Value]) -> (B3SoiFdBias, [Value; 4], bool) {
        let mut terminals = self.raw_terminals(v);
        let mut check = false;
        if self.limit_anchor_valid.get() {
            for (t, old) in terminals.iter_mut().zip(self.terminal_limit_anchor) {
                *t = common::soi_limit(*t, old, 3.0, &mut check);
            }
        }
        let [vg, vd, vs, ve] = terminals;
        let mut bias = B3SoiFdBias {
            vgs: self.mtype * (vg - vs),
            vds: self.mtype * (vd - vs),
            ves: self.mtype * (ve - vs),
            ..self.raw_branch_voltages(v)
        };
        let temp_limited = self.apply_temp_limiting(&mut bias);
        (bias, terminals, check || temp_limited)
    }

    /// Stamp the linearized DC operating point.
    ///
    /// Faithful transcription of the DC portion of the B3SOIFD matrix/RHS load
    /// (b3soifdld.c:3010-3245) for the FD body (no solved body node) with
    /// `ChargeComputationNeeded == 0`. When self-heating is active the
    /// temperature-rise row/column is included. Any drain/source series R has
    /// already been externalized into the prime nodes. The body-current terms
    /// are zero (see [`eval`]), so the junction/body conductance groups
    /// collapse, but they are retained structurally for provenance.
    ///
    /// CKTgmin is omitted: the RSpice solver adds its own diagonal gmin.
    fn stamp_op(&self, op: &B3SoiFdOp, bias: B3SoiFdBias, matrix: &mut impl MatrixStamper) {
        let (dp, g, sp, e) = (
            self.node_drain,
            self.node_gate,
            self.node_source,
            self.node_e,
        );
        let mt = self.mtype;

        // ----- conductance groups (b3soifdld.c:3010-3110) -----
        let (gm, gmbs, gme, fwd_sum, rev_sum, cdreq, ceqbs, ceqbd);
        let (gm_t, gddp_t, gssp_t);
        let (gddpg, gddpdp, gddpe, gddpsp);
        let (gsspg, gsspdp, gsspe, gsspsp);
        if op.mode >= 0 {
            gm = op.gm;
            gmbs = op.gmbs;
            gme = op.gme;
            gm_t = mt * op.gm_t;
            fwd_sum = gm + gmbs + gme;
            rev_sum = 0.0;
            // `gm_t` already carries the type sign, so its term stays outside
            // the type factor. ngspice-46 FD (BSIMSOI 2.1) keeps it inside
            // (b3soifdld.c:3017-3018), which leaves a PMOS drain current off
            // the device's by 2*gm_t*delTemp; BSIMSOI fixed it in PD
            // (b3soipdld.c:3872-3874, "v2.2.2 bug fix") and B4SOI
            // (b4soild.c:9953-9956).
            cdreq = mt
                * (op.cdrain
                    - op.gds * bias.vds
                    - gm * bias.vgs
                    - gmbs * bias.vbs
                    - gme * bias.ves)
                - gm_t * bias.del_temp;
            ceqbs = op.cjs - op.gjs_t * bias.del_temp;
            ceqbd = op.cjd - op.gjd_t * bias.del_temp;

            // Drain/source-prime junction conductances (body column drops out:
            // the body is not a node, so `gddpb`/`gsspb` are not stamped).
            gddpg = -op.gjdg;
            gddpdp = -op.gjdd;
            gddpe = -op.gjde;
            gddp_t = -mt * op.gjd_t;
            gddpsp = -(gddpg + gddpdp + gddpe);

            gsspg = -op.gjsg;
            gsspdp = -op.gjsd;
            gsspe = 0.0;
            gssp_t = -mt * op.gjs_t;
            gsspsp = -(gsspg + gsspdp + gsspe);
        } else {
            gm = -op.gm;
            gmbs = -op.gmbs;
            gme = -op.gme;
            gm_t = -mt * op.gm_t;
            fwd_sum = 0.0;
            rev_sum = -(gm + gmbs + gme);
            let vgd = bias.vgs - bias.vds;
            let vbd = bias.vbs - bias.vds;
            // As in normal mode, the `gm_t` term is subtracted outside the type
            // factor (ngspice-46 FD keeps it inside, b3soifdld.c:3072-3073;
            // B4SOI's form, b4soild.c:10104-10107).
            cdreq = -mt
                * (op.cdrain
                    + op.gds * bias.vds
                    + gm * vgd
                    + gmbs * vbd
                    + gme * (bias.ves - bias.vds))
                - gm_t * bias.del_temp;
            ceqbs = op.cjd - op.gjd_t * bias.del_temp;
            ceqbd = op.cjs - op.gjs_t * bias.del_temp;

            gddpg = -op.gjsg;
            gddpsp = -op.gjsd;
            gddpe = 0.0;
            gddp_t = -mt * op.gjs_t;
            gddpdp = -(gddpg + gddpsp + gddpe);

            // In inverse mode the source prime is the mode drain, so the
            // mode-drain junction's `vds` derivative `gjdd` lands on the
            // source-prime column (b3soifdld.c:3096-3101).
            gsspg = -op.gjdg;
            gsspsp = -op.gjdd;
            gsspe = -op.gjde;
            gssp_t = -mt * op.gjd_t;
            gsspdp = -(gsspg + gsspsp + gsspe);
        }

        // type<0: flip junction equivalent currents (b3soifdld.c:3118).
        let (ceqbs, ceqbd) = if mt < 0.0 {
            (-ceqbs, -ceqbd)
        } else {
            (ceqbs, ceqbd)
        };

        // ----- RHS (b3soifdld.c:3131-3135) -----
        stamp_rhs(matrix, dp, ceqbd - cdreq);
        stamp_rhs(matrix, sp, cdreq + ceqbs);

        stamp_conductance(matrix, g, dp, self.eval_gmin);
        stamp(matrix, sp, sp, self.eval_gmin);

        // ----- matrix (b3soifdld.c:3197-3245, DC: gc*=0) -----
        // Drain-prime row.
        stamp(matrix, dp, g, gm + gddpg);
        stamp(matrix, dp, dp, op.gds + gddpdp + rev_sum);
        stamp(matrix, dp, sp, -(-gddpsp + op.gds + fwd_sum));

        // Source-prime row.
        stamp(matrix, sp, g, -gm + gsspg);
        stamp(matrix, sp, dp, -(op.gds - gsspdp + rev_sum));
        stamp(matrix, sp, sp, op.gds + gsspsp + fwd_sum);

        // Back-gate (E) coupling columns of the drain/source-prime rows
        // (b3soifdld.c:3201-3202, DC: gc*=0). `gme` carries the mode sign.
        stamp(matrix, dp, e, gme + gddpe);
        stamp(matrix, sp, e, gsspe - gme);
        if self.self_heating_active() {
            let t = self.node_temp;
            stamp(matrix, dp, t, gm_t + gddp_t);
            stamp(matrix, sp, t, -gm_t + gssp_t);
        }
        self.stamp_thermal_matrix(op, matrix);

        // Gate row: gc* all zero in DC -> nothing.
        // E row: only EePtr (gceeb==0) -> nothing in DC.
        // No body row/column (FD has no body node).
    }

    fn stamp_thermal_matrix(&self, op: &B3SoiFdOp, matrix: &mut impl MatrixStamper) {
        if !self.self_heating_active() {
            if self.self_heating_startup_disabled.get() && self.has_self_heating_node() {
                stamp(matrix, self.node_temp, self.node_temp, 1.0);
            }
            return;
        }

        let t = self.node_temp;
        let (dp, sp) = (self.node_drain, self.node_source);
        let g = self.node_gate;
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

        stamp_rhs(matrix, t, -op.thermal_eq_current);
        stamp(matrix, t, t, op.gtemp_t + 1.0 / self.sized.rth);
        stamp(matrix, t, g, op.gtemp_g);
        stamp(matrix, t, e, op.gtemp_e);
        stamp(matrix, t, dp, gtemp_dp);
        stamp(matrix, t, sp, gtemp_sp);
    }
}

#[cfg(test)]
mod tests;
