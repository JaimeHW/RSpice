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
//! - **CAPMOD=3 charge model** ([`eval::eval`] with `compute_charges`): the
//!   gate/drain/back-gate node charges and capacitance matrix are integrated by
//!   the engine's B3SOI transient pass. FD pins the junction stored charge
//!   (`qjs = qjd = 0`) and never stamps the (algebraically-pinned) body charge.
//! - **Builder dispatch** is live for LEVEL=55 NMOS/PMOS (`engine/builder.rs`
//!   `build_b3soi_fd`).

pub use super::common;
pub use params::B3SoiFdModel;

pub mod eval;
pub mod params;
pub mod temp;

use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::{Value, circuit::NodeId};
use eval::{B3SoiFdBias, B3SoiFdOp, ModelConsts};
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
    pub name: String,
    /// +1 NMOS / -1 PMOS (model `mtype`).
    pub mtype: Value,

    // External nodes (already resolved to NodeId by the builder).
    pub node_drain: NodeId,
    pub node_gate: NodeId,
    pub node_source: NodeId,
    /// Back-gate / substrate-under-BOX node `E`.
    pub node_e: NodeId,
    /// Body-contact node `P`/`B`: read for the initial guess only (0 when
    /// floating). FD never stamps a body row/column.
    pub node_body: NodeId,

    pub body_mode: BodyMode,

    /// Shared model card (one per `.model`).
    pub model: Arc<B3SoiFdModel>,
    /// Size/temperature-resolved parameters (one per instance geometry).
    pub sized: Arc<B3SoiFdSized>,
    /// Model scalars needed inside the load.
    consts: ModelConsts,

    // Operating point (current iteration).
    op: B3SoiFdOp,
    bias: B3SoiFdBias,
    converged_ref: B3SoiFdBias,
    has_history: bool,
    /// DC/operating-point flag (kept for parity with DD; FD has no body-node
    /// convergence aids, so it only gates nothing today).
    dc_mode: std::cell::Cell<bool>,
}

impl B3SoiFd {
    /// Build an instance from a model card and instance geometry.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        node_drain: NodeId,
        node_gate: NodeId,
        node_source: NodeId,
        node_e: NodeId,
        node_body: NodeId,
        body_mode: BodyMode,
        model: Arc<B3SoiFdModel>,
        geom: B3SoiFdGeometry,
        temp_k: Value,
    ) -> Result<Self, String> {
        // Self-heating is not yet implemented; per spec it must be a hard error
        // when enabled rather than silently ignored (SHMOD=0 in supported decks).
        if model.sh_mod == 1 && geom.rth0 != 0.0 {
            return Err(format!(
                "B3SOIFD '{name}': self-heating (SHMOD=1 with RTH0!=0) is not yet implemented"
            ));
        }
        let sized = Arc::new(B3SoiFdSized::new(&model, &geom, temp_k)?);
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
            body_mode,
            model,
            sized,
            consts,
            op: B3SoiFdOp::default(),
            bias: B3SoiFdBias::default(),
            converged_ref: B3SoiFdBias::default(),
            has_history: false,
            dc_mode: std::cell::Cell::new(true),
        })
    }

    /// Select the analysis mode (kept for parity with the DD device; FD has no
    /// body-node convergence aids to toggle).
    pub fn set_dc_mode(&self, dc: bool) {
        self.dc_mode.set(dc);
    }

    /// Body node (0 when floating). Used by the harness `@m1[vbs]` probe.
    pub fn body_node(&self) -> NodeId {
        self.node_body
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

    /// Evaluate the CAPMOD=3 charge state at the given solution vector.
    pub fn charge_at(&self, voltages: &[Value]) -> eval::B3SoiFdCharge {
        let bias = self.branch_voltages(voltages);
        eval::eval(&self.sized, &self.consts, bias, self.mtype, true)
            .charge
            .expect("compute_charges=true yields a charge state")
    }

    /// Stamp the transient charge companion into the matrix and RHS.
    ///
    /// Mirrors the charge portion of `B3SOIFDload` (b3soifdld.c:3131-3245) for the
    /// floating/contact FD body: the gate, drain-prime, source-prime and back-gate
    /// (E) charge entries — but NO body row/column (FD has no body charge node).
    #[allow(clippy::too_many_arguments)]
    pub fn stamp_charge_companion(
        &self,
        charge: &eval::B3SoiFdCharge,
        ag0: Value,
        cqg: Value,
        cqb: Value,
        cqd: Value,
        cqe: Value,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
    ) {
        let (dp, g, sp, e, _b) = self.charge_nodes();
        let node = |n: NodeId| if n == 0 { 0.0 } else { voltages[n - 1] };
        let vg = node(g);
        let vd = node(dp);
        let vs = node(sp);
        let ve = node(e);
        // FD pins the body to its equilibrium value; the charge model evaluates
        // the `gc**` derivatives in the source-referenced body frame exactly as
        // DD, but with the body voltage taken from the device's pinned `vbs`.
        let bias = self.branch_voltages(voltages);
        let vb = self.mtype * bias.vbs + vs; // node-frame body potential
        let vgb = vg - vb;
        let vbd = vb - vd;
        let vbs = vb - vs;
        let veb = ve - vb;

        let c = charge;
        let gcggb = c.gcggb * ag0;
        let gcgdb = c.gcgdb * ag0;
        let gcgsb = c.gcgsb * ag0;
        let gcgeb = c.gcgeb * ag0;
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

        // Equivalent charge currents (b3soifdld.c:3131-3140). The body charge
        // `cqb`/`ceqqb` is folded into the source node (current sum) as ngspice
        // does, but never stamped to a body node.
        let mut ceqqg = cqg - gcggb * vgb + gcgdb * vbd + gcgsb * vbs - gcgeb * veb;
        let mut ceqqd = cqd - gcdgb * vgb + gcddb * vbd + gcdsb * vbs - gcdeb * veb;
        let mut ceqqe = cqe - gcegb * vgb + gcedb * vbd + gcesb * vbs - gceeb * veb;
        let ceqqb = cqb; // FD body charge has no node; only its sum closes KCL.
        if self.mtype < 0.0 {
            ceqqg = -ceqqg;
            ceqqd = -ceqqd;
            ceqqe = -ceqqe;
        }

        // RHS (b3soifdld.c:3131-3140): g, dp, sp (sum), e. No body node.
        stamp_rhs(matrix, g, -ceqqg);
        stamp_rhs(matrix, dp, -ceqqd);
        stamp_rhs(matrix, sp, ceqqg + ceqqb + ceqqd + ceqqe);
        stamp_rhs(matrix, e, -ceqqe);

        // Matrix charge entries (b3soifdld.c:3201-3243): the gate/drain/source/E
        // charge rows. The body-reference column is simply dropped — FD has no
        // body node, so each row stamps only its g/d/s/e columns (`gc*gb`,
        // `gc*db`, `gc*sb`, `gc*eb`); the body's contribution closes through the
        // `ceqq*` RHS terms. This mirrors ngspice exactly (no `Bx`/`*b` pointer).
        // E row.
        stamp(matrix, e, g, gcegb);
        stamp(matrix, e, dp, gcedb);
        stamp(matrix, e, sp, gcesb);
        stamp(matrix, e, e, gceeb);
        // G row.
        stamp(matrix, g, e, gcgeb);
        stamp(matrix, g, g, gcggb);
        stamp(matrix, g, dp, gcgdb);
        stamp(matrix, g, sp, gcgsb);
        // DP row.
        stamp(matrix, dp, e, gcdeb);
        stamp(matrix, dp, g, gcdgb);
        stamp(matrix, dp, dp, gcddb);
        stamp(matrix, dp, sp, gcdsb);
        // SP row.
        stamp(matrix, sp, e, gcseb);
        stamp(matrix, sp, g, gcsgb);
        stamp(matrix, sp, dp, gcsdb);
        stamp(matrix, sp, sp, gcssb);
    }

    /// Extract device-polarity branch voltages from the solution vector.
    ///
    /// Source-referenced, `mtype` folded in (b3soifdld.c:466-472). The body
    /// voltage read here is only the limiter seed; the FD load pins it to
    /// `Vbs0eff` internally, so its node value is immaterial.
    fn branch_voltages(&self, v: &[Value]) -> B3SoiFdBias {
        let node = |n: NodeId| if n == 0 { 0.0 } else { v[n - 1] };
        let vd = node(self.node_drain);
        let vg = node(self.node_gate);
        let vs = node(self.node_source);
        let ve = node(self.node_e);
        let vb = node(self.node_body);
        B3SoiFdBias {
            vbs: self.mtype * (vb - vs),
            vgs: self.mtype * (vg - vs),
            vds: self.mtype * (vd - vs),
            ves: self.mtype * (ve - vs),
            vps: 0.0,
        }
    }
}

impl NonlinearDevice for B3SoiFd {
    fn update(&mut self, voltages: &[Value]) {
        self.converged_ref = self.bias;
        let bias = self.branch_voltages(voltages);
        self.bias = bias;
        self.op = eval::eval_dc(&self.sized, &self.consts, bias, self.mtype);
        self.has_history = true;
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let bias = self.branch_voltages(voltages);
        let op = if biases_match(bias, self.bias) {
            self.op.clone()
        } else {
            eval::eval_dc(&self.sized, &self.consts, bias, self.mtype)
        };
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
    }
}

impl B3SoiFd {
    /// Stamp the linearized DC operating point.
    ///
    /// Faithful transcription of the DC portion of the B3SOIFD matrix/RHS load
    /// (b3soifdld.c:3010-3245) for the FD body (no body node, no temp node, no
    /// series R) with `ChargeComputationNeeded == 0`. The body-current terms are
    /// zero (see [`eval`]), so the junction/body conductance groups collapse, but
    /// they are retained structurally for provenance.
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
        let (gddpg, gddpdp, gddpe, gddpsp);
        let (gsspg, gsspdp, gsspe, gsspsp);
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

            // Drain/source-prime junction conductances (body column drops out:
            // the body is not a node, so `gddpb`/`gsspb` are not stamped).
            gddpg = -op.gjdg;
            gddpdp = -op.gjdd;
            gddpe = -op.gjde;
            gddpsp = -(gddpg + gddpdp + gddpe);

            gsspg = -op.gjsg;
            gsspdp = -op.gjsd;
            gsspe = 0.0;
            gsspsp = -(gsspg + gsspdp + gsspe);
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

            gddpg = -op.gjsg;
            gddpsp = -op.gjsd;
            gddpe = 0.0;
            gddpdp = -(gddpg + gddpsp + gddpe);

            gsspg = -op.gjdg;
            gsspdp = -op.gjdd;
            gsspe = -op.gjde;
            gsspsp = -(gsspg + gsspdp + gsspe);
        }

        // type<0: flip junction equivalent currents (b3soifdld.c:3118).
        let (ceqbs, ceqbd) = if mt < 0.0 { (-ceqbs, -ceqbd) } else { (ceqbs, ceqbd) };

        // ----- RHS (b3soifdld.c:3131-3135) -----
        stamp_rhs(matrix, dp, ceqbd - cdreq);
        stamp_rhs(matrix, sp, cdreq + ceqbs);

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

        // Gate row: gc* all zero in DC -> nothing.
        // E row: only EePtr (gceeb==0) -> nothing in DC.
        // No body row/column (FD has no body node).
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
fn biases_match(a: B3SoiFdBias, b: B3SoiFdBias) -> bool {
    a.vbs == b.vbs && a.vgs == b.vgs && a.vds == b.vds && a.ves == b.ves && a.vps == b.vps
}

#[cfg(test)]
mod tests;
