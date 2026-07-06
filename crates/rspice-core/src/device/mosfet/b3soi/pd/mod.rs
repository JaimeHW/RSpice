//! B3SOIPD (BSIMSOI partially-depleted, MOS level 57) device.
//!
//! Ported from ngspice-46 `src/spicelib/devices/bsim3soi_pd/`. This module ties
//! the model card ([`params`]), the size/temperature setup ([`temp`]), and the
//! DC load equations ([`eval`]) into a [`NonlinearDevice`] that RSpice can stamp.
//!
//! # Node topology (b3soipdset.c:975-1037)
//!
//! External terminals are drain `d`, gate `g`, source `s`, back-gate/substrate
//! `e` (substrate under the buried oxide), and an optional body contact `p`.
//! The device additionally owns an internal body node `B`, optional
//! drain/source primes for fixed `RSH * NRD/NRS` series resistance, and (in the
//! full model) a self-heating temperature node.
//!
//! For the supported decks (`rbody = rbsh = 0`, `shmod = 0`):
//! - **Floating body** (`d g s e`): ngspice creates an internal `Body` node,
//!   sets `bodyMod = 0`, `float = 1`. RSpice allocates one internal node.
//! - **Ideal body tie** (`d g s e p`): ngspice sets `bodyMod = 2` and the
//!   external `p` node *is* the body node; no internal node is created.
//!
//! When `RSH * NRD/NRS` is positive, the builder lowers the fixed
//! drain/source series resistance to ordinary linear resistors and passes the
//! prime nodes here as the device drain/source. The temperature node does not
//! exist (`shmod = 0`).
//!
//! # Status
//!
//! - **DC current path** ([`eval::eval_dc`]): faithful transcription of the
//!   `B3SOIPDload` DC block, including the back-gate (E) coupling columns.
//! - **CAPMOD=2/3 charge models** ([`eval::eval`] with `compute_charges`): the
//!   intrinsic + extrinsic + overlap charges and the coupled capacitance matrix
//!   (b3soipdld.c:2756-3784). Stamped as a transient companion by the engine's
//!   dedicated B3SOI pass; the four node charges (incl. the floating body) feed
//!   the local-truncation-error step control.
//! - **Convergence aids**: `B3SOIPDlimit` (per-iterate 0.2 V body-voltage cap)
//!   applied each Newton iterate.
//! - **Builder dispatch** is live for LEVEL=57 NMOS/PMOS and LEVEL=10
//!   `SOIMOD=0` through the native BSIMSOI routing in `engine/builder.rs`.
//!
//! Verified: the DC sweeps (`t3`/`t4`/`t5`/`inv2`) match the checked-in ngspice
//! references; the `RampVg2` floating-body `@m1[vbs]` trace matches at the DC
//! anchor (t=0, ~0.0917 V) and tracks the transient (the fast-edge body
//! amplitude is still being calibrated against ngspice's body LTE). `ring51`
//! (51-stage SOI ring oscillator) does not yet reach a DC operating point.

#![allow(clippy::too_many_arguments)]

pub use super::common;
pub use params::B3SoiPdModel;

pub mod eval;
pub mod params;
pub mod temp;

use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::{Value, circuit::NodeId};
use eval::{B3SoiPdBias, B3SoiPdOp, ModelConsts};
use std::borrow::Cow;
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

    // Device terminal nodes (already resolved to NodeId by the builder).
    // Drain/source are intrinsic primes when fixed RSH series resistors exist.
    pub node_drain: NodeId,
    pub node_gate: NodeId,
    pub node_source: NodeId,
    /// Back-gate / substrate-under-BOX node `E`.
    pub node_e: NodeId,
    /// Body node: internal (floating) or the external contact (tied).
    pub node_body: NodeId,
    /// Body-contact node `P` (== `node_body` when tied; unused when floating).
    pub node_p: NodeId,
    /// Self-heating temperature-rise node (`Temp`), or 0 when disabled.
    pub node_temp: NodeId,

    pub body_mode: BodyMode,

    /// Shared model card (one per `.model`).
    pub model: Arc<B3SoiPdModel>,
    /// Size/temperature-resolved parameters (one per instance geometry).
    pub sized: Arc<B3SoiPdSized>,
    /// Instance geometry retained for self-heating re-evaluation at
    /// `CKTtemp + delTemp`.
    geometry: B3SoiPdGeometry,
    base_temp_k: Value,
    /// Model scalars needed inside the load.
    consts: ModelConsts,
    /// Xyce-style BSIMSOI3 terminal GMIN used for body-source and gate-drain
    /// conductance branches.
    eval_gmin: Value,

    // Operating point (current iteration).
    op: B3SoiPdOp,
    // Branch voltages used at the last `update`, device polarity, pre-swap.
    bias: B3SoiPdBias,
    converged_ref: B3SoiPdBias,
    has_history: bool,
    /// Last accepted/limited `vbs` (device polarity) used as the limiter anchor
    /// for `B3SOIPDlimit` on the next Newton iterate.
    vbs_limit_anchor: Value,
    vbd_limit_anchor: Value,
    /// Last accepted/limited self-heating temperature rise used by
    /// `B3SOIPDlimit(delTemp, oldDelTemp, 5.0)`.
    del_temp_limit_anchor: Value,
    /// DC/operating-point mode. Cleared during transient and set by the engine
    /// before each analysis phase.
    dc_mode: std::cell::Cell<bool>,
    /// During electrothermal startup, solve the electrical floating-body
    /// operating point with the allocated thermal node anchored at zero.
    self_heating_startup_disabled: std::cell::Cell<bool>,
    /// Whether the limiter anchor has been seeded (first iterate uses the raw
    /// node solution).
    limit_anchor_valid: std::cell::Cell<bool>,
    /// The first DC load uses the MODEINITJCT bias directly. Without this guard
    /// `stamp_nonlinear` would immediately re-evaluate from the raw node seed
    /// and discard the startup branch that ngspice loads.
    startup_seed_pending: std::cell::Cell<bool>,
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
        node_temp: NodeId,
        body_mode: BodyMode,
        model: Arc<B3SoiPdModel>,
        geom: B3SoiPdGeometry,
        temp_k: Value,
    ) -> Result<Self, String> {
        if model.sh_mod == 1 && geom.rth0 != 0.0 && node_temp == 0 {
            return Err(format!(
                "B3SOIPD '{name}': self-heating (SHMOD=1 with RTH0!=0) requires a temperature node"
            ));
        }
        // DC evaluation is capmod-independent. Charge-based analyses are
        // guarded by the engine for unsupported CAPMOD values.
        let sized = Arc::new(B3SoiPdSized::new(&model, &geom, temp_k)?);
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
            dtoxcv: model.dtoxcv,
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
            node_temp,
            body_mode,
            model,
            sized,
            geometry: geom,
            base_temp_k: temp_k,
            consts,
            eval_gmin: 0.0,
            op: B3SoiPdOp::default(),
            bias: B3SoiPdBias::default(),
            converged_ref: B3SoiPdBias::default(),
            has_history: false,
            vbs_limit_anchor: 0.0,
            vbd_limit_anchor: 0.0,
            del_temp_limit_anchor: 0.0,
            dc_mode: std::cell::Cell::new(true),
            self_heating_startup_disabled: std::cell::Cell::new(false),
            limit_anchor_valid: std::cell::Cell::new(false),
            startup_seed_pending: std::cell::Cell::new(false),
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
    /// The per-iteration `B3SOIPDlimit` change cap applies in both modes (it
    /// only protects the Newton path, not the solution).
    pub fn set_dc_mode(&self, dc: bool) {
        if self.dc_mode.replace(dc) == dc {
            return;
        }
        // A mode switch invalidates the limiter anchor (different state vector).
        self.limit_anchor_valid.set(false);
        self.bypass_active.set(false);
        self.force_full_eval.set(true);
    }

    /// Clear the cached operating-point linearization before a fresh DC path.
    pub(crate) fn reset_operating_point_history(&mut self) {
        self.op = B3SoiPdOp::default();
        let zero_bias = B3SoiPdBias {
            vbs: 0.0,
            vgs: 0.0,
            vds: 0.0,
            ves: 0.0,
            vps: 0.0,
            del_temp: 0.0,
        };
        self.bias = zero_bias;
        self.converged_ref = zero_bias;
        self.has_history = false;
        self.vbs_limit_anchor = 0.0;
        self.vbd_limit_anchor = 0.0;
        self.del_temp_limit_anchor = 0.0;
        self.dc_mode.set(true);
        self.limit_anchor_valid.set(false);
        self.startup_seed_pending.set(false);
        self.bypass_active.set(false);
        self.force_full_eval.set(true);
        self.last_limited.set(false);
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

    /// `DEBUG=-1` (the only negative debug mode accepted by the builder):
    /// evaluate charges for probes but
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
        !self.self_heating_startup_disabled.get() && self.has_self_heating_node()
    }

    pub(crate) fn has_self_heating_node(&self) -> bool {
        self.node_temp != 0 && self.sized.rth.is_finite() && self.sized.rth > 0.0
    }

    pub(crate) fn set_self_heating_startup_disabled(&self, disabled: bool) {
        self.self_heating_startup_disabled.set(disabled);
        self.bypass_active.set(false);
        self.force_full_eval.set(true);
        self.last_limited.set(false);
    }

    pub(crate) fn seed_self_heating_temperature_from_power(&self, solution: &mut [Value]) {
        if !self.has_self_heating_node() {
            return;
        }
        let mut bias = self.raw_branch_voltages(solution);
        bias.del_temp = 0.0;
        let sized = self.sized_for_bias(bias);
        let op = eval::eval_dc(&sized, &self.consts, bias, self.mtype);
        let vds = if op.mode >= 0 { bias.vds } else { -bias.vds };
        let delta_temp = (op.ids * vds * self.sized.rth).clamp(0.0, 1.0e3);
        if delta_temp.is_finite() {
            if let Some(slot) = solution.get_mut(self.node_temp - 1) {
                *slot = delta_temp;
            }
        }
    }

    pub(crate) fn prime_operating_point_from_solution(&mut self, solution: &[Value]) {
        let bias = self.raw_branch_voltages(solution);
        self.converged_ref = bias;
        self.bias = bias;
        self.op = self.eval_op_for_bias(bias);
        self.has_history = true;
        self.vbs_limit_anchor = bias.vbs;
        self.vbd_limit_anchor = bias.vbs - bias.vds;
        self.del_temp_limit_anchor = bias.del_temp;
        self.limit_anchor_valid.set(true);
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
    /// Regular Newton updates intentionally use the BSIMSOI branch/body
    /// limiters. Residual and fallback validation probes need the compact-model
    /// equations at the candidate voltage itself so they can measure the true
    /// operating-point residual.
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
        self.startup_seed_pending.set(false);
        self.bypass_active.set(false);
        self.force_full_eval.set(false);
        self.last_limited.set(false);
    }

    /// ngspice bypass predicate (b3soipdld.c:509-560): every branch-voltage
    /// delta against the previous iterate's state is inside the Newton
    /// tolerances and the linear current predictions `cdhat`/`cbhat` match the
    /// stored device currents. `bodyMod` 0/2 skips the `vps` voltage test
    /// exactly as ngspice does.
    fn bypass_check(&self, raw: B3SoiPdBias, reltol: Value, abstol: Value, vntol: Value) -> bool {
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

        // Linear predictions with the stored linearization (b3soipdld.c:486-507;
        // PD's cdhat has no gme*delves term and its cbhat uses gbps*delvps).
        let op = &self.op;
        let cdhat = if op.mode >= 0 {
            op.cd
                + (op.gm - op.gjdg) * delvgs
                + (op.gds - op.gjdd) * delvds
                + (op.gmbs - op.gjdb) * delvbs
                + (op.gm_t - op.gjd_t) * deldel_temp
        } else {
            let delvgd = (raw.vgs - raw.vds) - (old.vgs - old.vds);
            op.cd + (op.gm - op.gjdg) * delvgd - (op.gds - op.gjdd) * delvds
                + (op.gmbs - op.gjdb) * delvbd
                + (op.gm_t - op.gjd_t) * deldel_temp
        };
        let cbhat = op.cb
            + op.gbgs * delvgs
            + op.gbbs * delvbs
            + op.gbds * delvds
            + op.gbps * delvps
            + op.gb_t * deldel_temp;
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

    /// Evaluate the selected CAPMOD charge state at the given solution vector.
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

    fn sized_for_bias(&self, bias: B3SoiPdBias) -> Cow<'_, B3SoiPdSized> {
        if !self.self_heating_active() {
            return Cow::Borrowed(self.sized.as_ref());
        }

        let temp = self.base_temp_k + bias.del_temp;
        if (temp - self.sized.temp).abs() <= f64::EPSILON * self.sized.temp.abs().max(1.0) {
            return Cow::Borrowed(self.sized.as_ref());
        }

        Cow::Owned(
            B3SoiPdSized::new(&self.model, &self.geometry, temp)
                .expect("self-heated B3SOIPD temperature evaluation"),
        )
    }

    fn eval_op_for_bias(&self, bias: B3SoiPdBias) -> B3SoiPdOp {
        let sized = self.sized_for_bias(bias);
        let mut op = eval::eval_dc(&sized, &self.consts, bias, self.mtype);
        if self.self_heating_active() {
            self.fill_electrothermal_derivatives(&mut op, bias);
        }
        op
    }

    fn fill_electrothermal_derivatives(&self, op: &mut B3SoiPdOp, bias: B3SoiPdBias) {
        let temp = self.base_temp_k + bias.del_temp;
        let h = (temp.abs().max(1.0) * 1.0e-5).clamp(1.0e-3, 5.0e-2);
        let lower_h = if temp - h > 1.0 { h } else { 0.0 };

        let sample = |del_temp: Value| {
            let sample_bias = B3SoiPdBias { del_temp, ..bias };
            let sized = B3SoiPdSized::new(&self.model, &self.geometry, self.base_temp_k + del_temp)
                .expect("self-heated B3SOIPD derivative temperature evaluation");
            let sample_op = eval::eval_dc(&sized, &self.consts, sample_bias, self.mtype);
            let drain_junction = sample_op.cjd
                + sample_op.gjdb * sample_bias.vbs
                + sample_op.gjdd * sample_bias.vds
                + sample_op.gjdg * sample_bias.vgs
                + sample_op.gjde * sample_bias.ves;
            let source_junction = sample_op.cjs
                + sample_op.gjsb * sample_bias.vbs
                + sample_op.gjsd * sample_bias.vds
                + sample_op.gjsg * sample_bias.vgs;
            let body_current = sample_op.cbody
                + sample_op.gbbs * sample_bias.vbs
                + sample_op.gbgs * sample_bias.vgs
                + sample_op.gbds * sample_bias.vds
                + sample_op.gbes * sample_bias.ves
                + sample_op.gbps * sample_bias.vps;
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
        let (vds, vgs, vbs) = if op.mode >= 0 {
            (bias.vds, bias.vgs, bias.vbs)
        } else {
            (-bias.vds, bias.vgs - bias.vds, bias.vbs - bias.vds)
        };
        op.gtemp_g = -op.gm * vds;
        op.gtemp_b = -op.gmbs * vds;
        op.gtemp_d = -op.gds * vds - op.ids;
        op.gtemp_t = -ids_t * vds;
        op.thermal_eq_current = -op.ids * vds
            - self.mtype * (op.gtemp_g * vgs + op.gtemp_b * vbs + op.gtemp_d * vds)
            - op.gtemp_t * bias.del_temp;
    }

    fn fill_charge_thermal_derivatives(&self, charge: &mut eval::B3SoiPdCharge, bias: B3SoiPdBias) {
        let temp = self.base_temp_k + bias.del_temp;
        let h = (temp.abs().max(1.0) * 1.0e-5).clamp(1.0e-3, 5.0e-2);
        let lower_h = if temp - h > 1.0 { h } else { 0.0 };

        let sample = |del_temp: Value| {
            let sample_bias = B3SoiPdBias { del_temp, ..bias };
            let sized = B3SoiPdSized::new(&self.model, &self.geometry, self.base_temp_k + del_temp)
                .expect("self-heated B3SOIPD charge derivative temperature evaluation");
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
    /// `B3SOIPDload` (b3soipdld.c:3679-3868, 4083-4128) for `bodyMod` 0/2 and no
    /// temp node. Fixed drain/source series resistance is handled outside this
    /// device by builder-owned prime-node resistors. The gate-overlap and
    /// extrinsic-substrate derivatives are already folded into `charge`'s
    /// `gc**` matrix.
    pub fn stamp_charge_companion(
        &self,
        charge: &eval::B3SoiPdCharge,
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
        let gcg_t = c.gcg_t * ag0;
        let gcb_t = c.gcb_t * ag0;
        let gcd_t = c.gcd_t * ag0;
        let gce_t = c.gce_t * ag0;
        let gcs_t = -(gcg_t + gcb_t + gcd_t + gce_t);

        // Equivalent charge currents (b3soipdld.c:3860-3867). type<0 flips sign.
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

        // RHS (b3soipdld.c:4011-4017, charge parts).
        stamp_rhs(matrix, b, -ceqqb);
        stamp_rhs(matrix, g, -ceqqg);
        stamp_rhs(matrix, dp, -ceqqd);
        stamp_rhs(matrix, sp, ceqqg + ceqqb + ceqqd + ceqqe);
        stamp_rhs(matrix, e, -ceqqe);
        stamp_rhs(matrix, self.node_temp, -ceqqth);

        // Matrix charge entries (b3soipdld.c:4083-4128, gc** parts only).
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
        let vt = node(self.node_temp);
        B3SoiPdBias {
            vbs: self.mtype * (vb - vs),
            vgs: self.mtype * (vg - vs),
            vds: self.mtype * (vd - vs),
            ves: self.mtype * (ve - vs),
            vps: self.mtype * (vp - vs),
            del_temp: vt,
        }
    }

    fn branch_voltages(&self, v: &[Value]) -> B3SoiPdBias {
        let mut bias = self.raw_branch_voltages(v);
        let _ = self.apply_branch_limiting(&mut bias);
        let _ = self.apply_body_limiting(&mut bias);
        bias
    }

    /// ngspice limits the absolute gate/drain/source/body-contact/e-node
    /// voltages to 3 V per Newton step before forming most branch voltages.
    /// RSpice stores source-referenced branch voltages at the intrinsic
    /// drain/source nodes, so this is the equivalent limiter whether those nodes
    /// are external terminals or builder-owned series-resistance primes.
    fn apply_branch_limiting(&self, bias: &mut B3SoiPdBias) -> bool {
        if !self.limit_anchor_valid.get() {
            return false;
        }
        let mut check = false;
        bias.vgs = common::soi_limit(bias.vgs, self.bias.vgs, 3.0, &mut check);
        bias.vds = common::soi_limit(bias.vds, self.bias.vds, 3.0, &mut check);
        bias.ves = common::soi_limit(bias.ves, self.bias.ves, 3.0, &mut check);
        bias.vps = common::soi_limit(bias.vps, self.bias.vps, 3.0, &mut check);
        check
    }

    /// MODEINITJCT startup bias used by Xyce/ngspice for an uninitialized
    /// B3SOIPD operating-point solve with no explicit input operating point.
    ///
    /// Xyce's normal `initJctFlag` path preserves the source-solved terminal
    /// drops and only imposes the gate startup value; it does not zero Vbs,
    /// Vds, Ves, or Vps. That is important for `.NODESET` and DC-sweep
    /// warm-starts of floating-body devices.
    fn junction_init_bias(&self, raw: B3SoiPdBias) -> B3SoiPdBias {
        B3SoiPdBias {
            vgs: self.mtype * 0.1 + self.sized.vth0,
            ..raw
        }
    }

    /// Floating-body convergence aid `B3SOIPDlimit` (b3soipdld.c:50-99,
    /// 664-688), applied per Newton iterate.
    ///
    /// In the mode-selected (normal/inverse) frame, the body-source (or
    /// body-drain) voltage is clamped to move at most 0.2 V from the previous
    /// iterate's value. This only reshapes the Newton path; the converged
    /// solution still satisfies KCL.
    ///
    /// Returns whether the per-iteration change cap actually engaged (the
    /// ngspice `Check` flag).
    fn apply_body_limiting(&self, bias: &mut B3SoiPdBias) -> bool {
        if !self.limit_anchor_valid.get() {
            // First iterate of a phase: accept the raw bias.
            return false;
        }
        let mut check = false;
        if bias.vds >= 0.0 {
            bias.vbs = common::soi_limit(bias.vbs, self.vbs_limit_anchor, 0.2, &mut check);
        } else {
            let vbd0 = bias.vbs - bias.vds;
            let vbd = common::soi_limit(vbd0, self.vbd_limit_anchor, 0.2, &mut check);
            bias.vbs = vbd + bias.vds;
        }
        if self.self_heating_active() {
            bias.del_temp =
                common::soi_limit(bias.del_temp, self.del_temp_limit_anchor, 5.0, &mut check);
        }
        check
    }
}

impl NonlinearDevice for B3SoiPd {
    fn update(&mut self, voltages: &[Value]) {
        self.converged_ref = self.bias;
        if self.startup_seed_pending.get() && self.dc_mode.get() {
            return;
        }
        if !self.has_history && self.dc_mode.get() {
            let bias = self.junction_init_bias(self.raw_branch_voltages(voltages));
            self.bias = bias;
            self.op = self.eval_op_for_bias(bias);
            self.has_history = true;
            self.vbs_limit_anchor = bias.vbs;
            self.vbd_limit_anchor = bias.vbs - bias.vds;
            self.del_temp_limit_anchor = bias.del_temp;
            self.limit_anchor_valid.set(true);
            self.startup_seed_pending.set(true);
            self.bypass_active.set(false);
            self.force_full_eval.set(false);
            self.last_limited.set(false);
            return;
        }
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
        let limited = self.apply_branch_limiting(&mut bias) || self.apply_body_limiting(&mut bias);
        self.last_limited.set(limited);
        self.startup_seed_pending.set(false);
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
        _voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
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
        let bias = self.bias;
        let op = self.op.clone();
        self.stamp_op(&op, bias, matrix);
        self.stamp_instance_ic(matrix);
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
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
            && cmp(self.bias.vps, self.converged_ref.vps)
            && (!self.self_heating_active() || cmp(self.bias.del_temp, self.converged_ref.del_temp))
    }
}

impl B3SoiPd {
    fn stamp_instance_ic(&self, matrix: &mut impl MatrixStamper) {
        self.instance_ic.stamp(self.dc_mode.get(), matrix);
    }

    /// Stamp the linearized DC operating point.
    ///
    /// Faithful transcription of the DC portion of the B3SOIPD matrix/RHS load
    /// (b3soipdld.c:3886-4150) for `bodyMod` 0/2, no temp node, and
    /// `ChargeComputationNeeded == 0` (so all `gc*`/`ceqq*` charge terms vanish).
    /// `m` (multiplier) is 1. Xyce's BSIMSOI3 terminal-GMIN branches are
    /// stamped explicitly; they are not equivalent to the solver's nodal
    /// conditioning shunt.
    ///
    /// In DC the builder supplies `dNodePrime`/`sNodePrime` as this device's
    /// drain/source when fixed series resistance is present; otherwise they
    /// coincide with the external terminals. `bNode` is the body node (internal
    /// floating or external tie), and `tempNode` is absent.
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

        // type<0: flip junction/body equivalent currents (b3soipdld.c:3997)
        let ceqbody = -(op.cbody - op.gb_t * bias.del_temp);
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
        self.stamp_thermal_rhs(op, matrix);

        stamp_conductance(matrix, b, sp, self.eval_gmin);
        stamp_conductance(matrix, g, dp, self.eval_gmin);

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
        if self.self_heating_active() {
            let t = self.node_temp;
            stamp(matrix, dp, t, gm_t + gddp_t);
            stamp(matrix, sp, t, -gm_t + gssp_t);
            stamp(matrix, b, t, gbb_t);
        }
        self.stamp_thermal_matrix(op, matrix);

        // Body-tie resistor (bodyMod 1): a linear conductance between the
        // internal body node `b` and the external contact `p` carrying
        // Ibp = Vbp / (Rbody + Rbodyext) (b3soipdld.c:2017-2042, 4131-4137). Pure
        // resistor, no equivalent-current source needed. Absent for floating
        // (bodyMod 0) and ideal-tie (bodyMod 2) devices.
        if self.body_mode == BodyMode::TiedResistive {
            let p = self.node_p;
            let rtot = self.sized.rbody + self.sized.rbodyext;
            let gbp = if rtot > 1e-30 {
                1.0 / rtot
            } else {
                1.0 / 1e-30
            };
            stamp(matrix, b, b, gbp);
            stamp(matrix, b, p, -gbp);
            stamp(matrix, p, b, -gbp);
            stamp(matrix, p, p, gbp);
        }
    }

    fn stamp_thermal_rhs(&self, op: &B3SoiPdOp, matrix: &mut impl MatrixStamper) {
        if self.self_heating_active() {
            stamp_rhs(matrix, self.node_temp, -op.thermal_eq_current);
        }
    }

    fn stamp_thermal_matrix(&self, op: &B3SoiPdOp, matrix: &mut impl MatrixStamper) {
        if !self.self_heating_active() {
            if self.self_heating_startup_disabled.get() && self.has_self_heating_node() {
                stamp(matrix, self.node_temp, self.node_temp, 1.0);
            }
            return;
        }

        let t = self.node_temp;
        let (dp, sp) = (self.node_drain, self.node_source);
        let g = self.node_gate;
        let b = self.node_body;

        let (gtemp_dp, gtemp_sp) = if op.mode >= 0 {
            (op.gtemp_d, -(op.gtemp_g + op.gtemp_b + op.gtemp_d))
        } else {
            (-(op.gtemp_g + op.gtemp_b + op.gtemp_d), op.gtemp_d)
        };

        stamp(matrix, t, t, op.gtemp_t + 1.0 / self.sized.rth);
        stamp(matrix, t, g, op.gtemp_g);
        stamp(matrix, t, b, op.gtemp_b);
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

#[cfg(test)]
mod tests;
