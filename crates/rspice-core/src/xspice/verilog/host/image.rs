//! The host as a trial image.
//!
//! A mixed circuit's shared wheel images this host before every candidate and
//! copies it on the candidate's first write (`MixedCell` in `mixed.rs`), so a
//! settled design is copied once per Newton probe. That copy is refilled into
//! the allocations of the image the previous candidate left behind rather than
//! built afresh, which is what [`Clone::clone_from`] is for: derived, it is
//! `*self = source.clone()`, a fresh allocation for every queue, table and
//! index below and a free for every one it replaces.
//!
//! Both methods destructure the host, so a field added to it does not compile
//! until it has been placed here.

use super::*;

impl Clone for DigitalHost {
    fn clone(&self) -> Self {
        let Self {
            checkpoint_failed,
            plan,
            store,
            scheduler,
            resolution,
            nba_target,
            external_targets,
            external_real_targets,
            elaboration_closed,
            delayed_updates,
            event_updates,
            expression_updates,
            expression_processes,
            event_waiters,
            event_candidates,
            slots,
            free_threads,
            disabling,
            withdrawn,
            waiters,
            inactive,
            analog_waiters,
            analog_ready,
            analog_activation_seconds,
            analog_exchange_seconds,
            targets,
            process_of_target,
            fired,
            drained,
            scratch,
            effects,
            reverse_active,
        } = self;
        Self {
            checkpoint_failed: *checkpoint_failed,
            plan: Arc::clone(plan),
            store: store.clone(),
            scheduler: scheduler.clone(),
            resolution: *resolution,
            nba_target: *nba_target,
            external_targets: external_targets.clone(),
            external_real_targets: external_real_targets.clone(),
            elaboration_closed: *elaboration_closed,
            delayed_updates: delayed_updates.clone(),
            event_updates: event_updates.clone(),
            expression_updates: expression_updates.clone(),
            expression_processes: expression_processes.clone(),
            event_waiters: event_waiters.clone(),
            event_candidates: event_candidates.clone(),
            slots: slots.clone(),
            free_threads: free_threads.clone(),
            disabling: *disabling,
            withdrawn: withdrawn.clone(),
            waiters: waiters.clone(),
            inactive: inactive.clone(),
            analog_waiters: analog_waiters.clone(),
            analog_ready: analog_ready.clone(),
            analog_activation_seconds: *analog_activation_seconds,
            analog_exchange_seconds: *analog_exchange_seconds,
            targets: targets.clone(),
            process_of_target: process_of_target.clone(),
            fired: fired.clone(),
            drained: drained.clone(),
            scratch: scratch.clone(),
            effects: effects.clone(),
            reverse_active: *reverse_active,
        }
    }

    /// Every field is overwritten with `source`'s, reusing what this image
    /// already holds — except the interpreter's scratch.
    ///
    /// `DigitalEvalScratch` is storage, never state, by its own contract: every
    /// entry empties the value table it reads, every list it lends is cleared
    /// before it is filled, and its caches are derived from the plan and keyed
    /// by the plan's identity. Two hosts that differ only in it run every
    /// activation identically, so the refilled image keeps its own rather
    /// than copying three hash maps of derived programs.
    fn clone_from(&mut self, source: &Self) {
        let Self {
            checkpoint_failed,
            plan,
            store,
            scheduler,
            resolution,
            nba_target,
            external_targets,
            external_real_targets,
            elaboration_closed,
            delayed_updates,
            event_updates,
            expression_updates,
            expression_processes,
            event_waiters,
            event_candidates,
            slots,
            free_threads,
            disabling,
            withdrawn,
            waiters,
            inactive,
            analog_waiters,
            analog_ready,
            analog_activation_seconds,
            analog_exchange_seconds,
            targets,
            process_of_target,
            fired,
            drained,
            scratch: _,
            effects,
            reverse_active,
        } = self;
        let Self {
            checkpoint_failed: source_checkpoint_failed,
            plan: source_plan,
            store: source_store,
            scheduler: source_scheduler,
            resolution: source_resolution,
            nba_target: source_nba_target,
            external_targets: source_external_targets,
            external_real_targets: source_external_real_targets,
            elaboration_closed: source_elaboration_closed,
            delayed_updates: source_delayed_updates,
            event_updates: source_event_updates,
            expression_updates: source_expression_updates,
            expression_processes: source_expression_processes,
            event_waiters: source_event_waiters,
            event_candidates: source_event_candidates,
            slots: source_slots,
            free_threads: source_free_threads,
            disabling: source_disabling,
            withdrawn: source_withdrawn,
            waiters: source_waiters,
            inactive: source_inactive,
            analog_waiters: source_analog_waiters,
            analog_ready: source_analog_ready,
            analog_activation_seconds: source_analog_activation_seconds,
            analog_exchange_seconds: source_analog_exchange_seconds,
            targets: source_targets,
            process_of_target: source_process_of_target,
            fired: source_fired,
            drained: source_drained,
            scratch: _,
            effects: source_effects,
            reverse_active: source_reverse_active,
        } = source;
        *checkpoint_failed = *source_checkpoint_failed;
        plan.clone_from(source_plan);
        store.clone_from(source_store);
        scheduler.clone_from(source_scheduler);
        *resolution = *source_resolution;
        *nba_target = *source_nba_target;
        external_targets.clone_from(source_external_targets);
        external_real_targets.clone_from(source_external_real_targets);
        *elaboration_closed = *source_elaboration_closed;
        delayed_updates.clone_from(source_delayed_updates);
        event_updates.clone_from(source_event_updates);
        expression_updates.clone_from(source_expression_updates);
        expression_processes.clone_from(source_expression_processes);
        event_waiters.clone_from(source_event_waiters);
        event_candidates.clone_from(source_event_candidates);
        slots.clone_from(source_slots);
        free_threads.clone_from(source_free_threads);
        *disabling = *source_disabling;
        withdrawn.clone_from(source_withdrawn);
        waiters.clone_from(source_waiters);
        inactive.clone_from(source_inactive);
        analog_waiters.clone_from(source_analog_waiters);
        analog_ready.clone_from(source_analog_ready);
        *analog_activation_seconds = *source_analog_activation_seconds;
        *analog_exchange_seconds = *source_analog_exchange_seconds;
        targets.clone_from(source_targets);
        process_of_target.clone_from(source_process_of_target);
        fired.clone_from(source_fired);
        drained.clone_from(source_drained);
        effects.clone_from(source_effects);
        *reverse_active = *source_reverse_active;
    }
}
