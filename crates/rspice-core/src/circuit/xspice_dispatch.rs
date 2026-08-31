//! Net-to-instance sensitivity for the XSPICE settle loop.
//!
//! The settle loop used to open every delta cycle with a full pass: for each
//! registered instance, refresh its inputs, evaluate it, and drain whatever it
//! scheduled. On a gate-level design that is the wrong shape — a 288-gate
//! adder evaluated 288 instances per delta even when one input toggled — and
//! it is unnecessary, because the event kernel already knows which drivers
//! fired. This module is the inverse of that knowledge: given the nets an
//! executed event touched, which instances can possibly care.
//!
//! # What the map holds
//!
//! One entry per net that some instance reads events from, listing the
//! *registration indices* of those instances in ascending order. Registration
//! order is the order the settle loop walks, so a dispatch built from this map
//! visits instances in exactly the order a full pass would; the subset changes,
//! the sequence does not. That is what keeps a settled fixpoint bit-identical
//! rather than merely equivalent.
//!
//! The map is derived from port directions and connections alone, which are
//! fixed once the circuit is built, so it is cached and rebuilt only when the
//! instance list or the node numbering changes.
//!
//! # Why skipping an instance is sound
//!
//! Skipping is not justified by reasoning about what a model computes. It is
//! justified by showing that the skip is a *subset* of a skip the engine
//! already performs, and has been performing since before this module existed.
//!
//! [`XspiceInstance::evaluate`] already declines to run the model body when the
//! *event-input signature* — the value and accepted event time of every
//! event-driven input port — equals the signature recorded at the previous
//! evaluation, for models that declare
//! [`CodeModel::can_skip_unchanged_event_inputs`]. So the question is not "is
//! this model a pure function of its inputs and state" — the engine has already
//! taken that position — but "can the caller predict that the signature check
//! will match, without paying for the check".
//!
//! It can, because of where the signature's ingredients come from.
//!
//! **Lemma.** Let `I` be an instance whose model opts in, and suppose no event
//! has touched any net in `I`'s input fan-in since `I` was last evaluated. Then
//! `refresh_event_input_signature` would produce exactly the recorded
//! signature.
//!
//! *Proof.* Every signature entry is read out of the instance context slots
//! written by `update_inputs_with_analog_transitions`, and for an event
//! connection those writes are copies of `digital_values[node]`,
//! `digital_event_times[node]`, `real_values[node]`, or `real_event_times[node]`
//! for a node the port is wired to. Those four maps are mutated in exactly one
//! place, `apply_xspice_events_at_or_before`, which writes only entries for the
//! nodes it reports as touched. Untouched nets therefore carry unchanged map
//! entries, hence unchanged context slots, hence an equal signature. ∎
//!
//! **Theorem.** Under the Lemma's hypothesis, omitting `I` from a pass is
//! observationally identical to including it. Including it would have:
//!
//! 1. set the transient companion coefficients and the one-step order flag on
//!    the instance context — slots read only by analog-output models, which
//!    [`XspiceInstance::supports_event_dirty_dispatch`] excludes;
//! 2. run `update_inputs_with_analog_transitions`, which rewrites the same
//!    values it wrote last time (Lemma) together with the per-port event load,
//!    itself a topology constant;
//! 3. run `evaluate`, which by the Lemma takes the early return and executes no
//!    model code;
//! 4. collected `analog_output_transitions`, empty because the eligibility rule
//!    admits no analog, differential, current-output, or hybrid connection;
//! 5. run `schedule_events`, which drains a pending queue that step 3 left
//!    empty;
//! 6. re-entered the event drain, which finds nothing new to apply.
//!
//! None of the six is observable, so the fixpoint the loop settles to is
//! unchanged. ∎
//!
//! The hypothesis is tracked by one bit per instance,
//! [`XspiceInstance::event_inputs_dirty`], which is set whenever an event
//! touches a net in the instance's fan-in and cleared only inside `evaluate`,
//! in the same two places that establish the recorded signature is current. It
//! starts set, because before the first evaluation there is no recorded
//! signature for the Lemma to be about.
//!
//! Because the bit lives beside the signature it mirrors, on the instance
//! itself, it is captured and restored by exactly the snapshots that capture
//! and restore the signature and the four value maps —
//! `CircuitData::nonlinear_state_snapshot` clones them together — so a rejected
//! Newton probe or a retried timestep cannot leave the bit disagreeing with the
//! state it describes.
//!
//! The eligibility rule is deliberately narrow: a model that does not opt in,
//! or an instance with any non-event connection, is evaluated every pass as
//! before. Sources, oscillators, bridges, file-driven stimulus, and every
//! analog and hybrid model stay in that set.

use super::{CircuitData, NodeId};
use crate::xspice::XspiceInstance;
use std::collections::HashMap;

/// Which instances read events from which nets.
#[derive(Debug, Clone, Default)]
pub(crate) struct XspiceEventDispatch {
    /// Net to the ascending registration indices of the instances whose input
    /// ports read it.
    fanout: HashMap<NodeId, Vec<u32>>,
    /// Per instance, in registration order: whether it may be skipped while
    /// its input nets are quiet. Every other instance is evaluated on the
    /// opening pass of every settle call, exactly as it was before this map
    /// existed.
    dirty_dispatched: Vec<bool>,
}

impl XspiceEventDispatch {
    /// Build the map for a fixed instance list.
    pub(crate) fn build(instances: &[XspiceInstance]) -> Self {
        let mut fanout: HashMap<NodeId, Vec<u32>> = HashMap::new();
        let mut dirty_dispatched = Vec::with_capacity(instances.len());
        for (index, instance) in instances.iter().enumerate() {
            let index = index as u32;
            instance.for_each_event_input_net(|node| {
                let entry = fanout.entry(node).or_default();
                // Instances are visited in ascending registration order, so
                // the list stays sorted and a net repeated across an
                // instance's ports is appended at most once.
                if entry.last() != Some(&index) {
                    entry.push(index);
                }
            });
            dirty_dispatched.push(instance.supports_event_dirty_dispatch());
        }
        Self {
            fanout,
            dirty_dispatched,
        }
    }

    /// Registration indices of the instances reading events from `node`.
    #[inline]
    pub(crate) fn fanout(&self, node: NodeId) -> &[u32] {
        self.fanout.get(&node).map_or(&[], Vec::as_slice)
    }

    /// Whether the instance at `index` may be skipped while its inputs are
    /// quiet.
    #[inline]
    pub(crate) fn is_dirty_dispatched(&self, index: usize) -> bool {
        self.dirty_dispatched.get(index).copied().unwrap_or(false)
    }

    /// Mark every instance reading events from `nodes` as having dirty inputs.
    ///
    /// This is the flag that outlives the current settle call: a driver moved
    /// onto the net, so the recorded event-input signature of everything
    /// downstream can no longer be assumed current. It is set even when the
    /// net's resolved value did not change, because the accepted event time is
    /// part of that signature.
    pub(crate) fn mark_fanout_dirty(&self, instances: &mut [XspiceInstance], nodes: &[NodeId]) {
        for &node in nodes {
            for &index in self.fanout(node) {
                if let Some(instance) = instances.get_mut(index as usize) {
                    instance.mark_event_inputs_dirty();
                }
            }
        }
    }

    /// Record every instance reading events from `nodes` as owed an
    /// evaluation in the pass `pending` describes.
    pub(crate) fn record_fanout_pending(&self, pending: &mut [bool], nodes: &[NodeId]) {
        for &node in nodes {
            for &index in self.fanout(node) {
                if let Some(slot) = pending.get_mut(index as usize) {
                    *slot = true;
                }
            }
        }
    }
}

impl CircuitData {
    /// Discard the cached sensitivity map.
    ///
    /// Called wherever the instance list grows or the node numbering moves,
    /// which are the only two things the map depends on.
    pub(crate) fn invalidate_xspice_event_dispatch(&mut self) {
        self.xspice_event_dispatch = None;
    }

    /// Build the sensitivity map if it is not cached, so that later code can
    /// hold it by shared reference alongside mutable borrows of sibling
    /// fields.
    pub(crate) fn ensure_xspice_event_dispatch(&mut self) {
        if self.xspice_event_dispatch.is_none() {
            self.xspice_event_dispatch = Some(XspiceEventDispatch::build(&self.xspice_instances));
        }
    }
}
