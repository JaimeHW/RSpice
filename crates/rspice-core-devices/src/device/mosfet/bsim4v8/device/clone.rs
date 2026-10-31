//! `Clone` for [`Bsim4v8Device`] whose `clone_from` refreshes in place.
//!
//! The transient engine snapshots every nonlinear device before each step
//! attempt and Newton merit trial. A derived `clone_from` rebuilds each
//! instance, allocating its names and touching its shared model handles; this
//! one copies the iteration state into the existing instance. The exhaustive
//! destructuring makes a new field a compile error until it is copied here.

use super::*;

impl Clone for Bsim4v8Device {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            node_drain_external: self.node_drain_external,
            node_drain: self.node_drain,
            node_gate_external: self.node_gate_external,
            node_gate_mid: self.node_gate_mid,
            node_gate: self.node_gate,
            node_source_external: self.node_source_external,
            node_source: self.node_source,
            node_bulk_external: self.node_bulk_external,
            node_bulk: self.node_bulk,
            node_drain_body: self.node_drain_body,
            node_source_body: self.node_source_body,
            node_charge_deficit: self.node_charge_deficit,
            multiplier: self.multiplier,
            core: self.core.clone(),
            gmin: self.gmin,
            op: self.op.clone(),
            bias: self.bias,
            converged_ref: self.converged_ref,
            junction_bias: self.junction_bias,
            converged_junction_ref: self.converged_junction_ref,
            von_prev: self.von_prev,
            has_history: self.has_history,
            limit_anchor_valid: self.limit_anchor_valid.clone(),
            last_update: self.last_update,
            last_update_open: self.last_update_open.clone(),
            last_limited: self.last_limited.clone(),
            initial_off: self.initial_off,
            initial_off_seed_pending: self.initial_off_seed_pending,
            initial_off_seed_evaluations: self.initial_off_seed_evaluations,
            initial_off_seed_raw: self.initial_off_seed_raw,
            eval_memo: self.eval_memo.clone(),
            charges_consumed: self.charges_consumed.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        let Self {
            name,
            node_drain_external,
            node_drain,
            node_gate_external,
            node_gate_mid,
            node_gate,
            node_source_external,
            node_source,
            node_bulk_external,
            node_bulk,
            node_drain_body,
            node_source_body,
            node_charge_deficit,
            multiplier,
            core,
            gmin,
            op,
            bias,
            converged_ref,
            junction_bias,
            converged_junction_ref,
            von_prev,
            has_history,
            limit_anchor_valid,
            last_update,
            last_update_open,
            last_limited,
            initial_off,
            initial_off_seed_pending,
            initial_off_seed_evaluations,
            initial_off_seed_raw,
            eval_memo: _,
            charges_consumed,
        } = source;
        self.name.clone_from(name);
        self.node_drain_external = *node_drain_external;
        self.node_drain = *node_drain;
        self.node_gate_external = *node_gate_external;
        self.node_gate_mid = *node_gate_mid;
        self.node_gate = *node_gate;
        self.node_source_external = *node_source_external;
        self.node_source = *node_source;
        self.node_bulk_external = *node_bulk_external;
        self.node_bulk = *node_bulk;
        self.node_drain_body = *node_drain_body;
        self.node_source_body = *node_source_body;
        self.node_charge_deficit = *node_charge_deficit;
        self.multiplier = *multiplier;
        self.core.clone_from(core);
        self.gmin = *gmin;
        self.op.clone_from(op);
        self.bias = *bias;
        self.converged_ref = *converged_ref;
        self.junction_bias = *junction_bias;
        self.converged_junction_ref = *converged_junction_ref;
        self.von_prev = *von_prev;
        self.has_history = *has_history;
        self.limit_anchor_valid.set(limit_anchor_valid.get());
        self.last_update = *last_update;
        self.last_update_open.set(last_update_open.get());
        self.last_limited.set(last_limited.get());
        self.initial_off = *initial_off;
        self.initial_off_seed_pending = *initial_off_seed_pending;
        self.initial_off_seed_evaluations = *initial_off_seed_evaluations;
        self.initial_off_seed_raw = *initial_off_seed_raw;
        // The records belong to the model data this instance held before.
        self.eval_memo.clear();
        self.charges_consumed.set(charges_consumed.get());
    }
}
