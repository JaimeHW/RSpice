//! Proof that a time step can never settle: a delta cycle leaves the design in
//! a state it was already in at that instant.
//!
//! A design is deterministic, so a step that comes back to a state it has been
//! in goes round the same cycle for ever. IEEE 1364-2005 section 11.4 leaves
//! the settling of a step unbounded, and no count tells a long legal step (a
//! zero-time counter, a deep nonblocking chain) from a loop; a repeated state
//! does. The search is Brent's (BIT 20, 1980): one saved state, replaced each
//! time the distance from it reaches a power of two, compared with the live
//! state after every delta cycle. A cycle of period λ entered after μ delta
//! cycles is found within about 2(μ + λ) of the search starting.
//!
//! The search starts only once a step has run [`REPETITION_SUSPICION`] delta
//! cycles, so a step that settles sooner pays nothing for it. From then on a
//! delta cycle costs one comparison of the state, which stops at the first
//! difference, and the saved copy is refilled in place at each power of two.

use super::*;
use crate::xspice::event_scheduler::DueEvent;

/// Delta cycles of one time step before the host starts looking for a
/// repeated state.
///
/// The longest settling a legal design was measured to need without a `#0`
/// is 76 delta cycles (the mixed-signal corpus) and 12 in the ivtest
/// selection; a step past this is unusual enough that comparing its state is
/// worth the time, and a loop it proves is still refused in a fraction of a
/// second.
pub(super) const REPETITION_SUSPICION: u32 = 1 << 16;

/// Brent's search over one time step's states.
#[derive(Default)]
pub(super) struct RepetitionSearch {
    /// The state compared against, once one is taken; refilled in place.
    saved: Option<Box<DigitalHost>>,
    /// Whether `saved` holds a state of this search.
    armed: bool,
    /// The kernel's due events in `saved`.
    saved_due: Vec<DueEvent>,
    /// The kernel's due events now, reused across delta cycles.
    due: Vec<DueEvent>,
    /// Delta cycles since `saved` was taken.
    distance: u32,
    /// The distance at which `saved` is replaced.
    power: u32,
}

impl RepetitionSearch {
    /// Compare the state the delta cycle just counted left with the saved
    /// one. `Some(period)` when it is the same: the step repeats every
    /// `period` delta cycles and can never settle.
    /// Start the search over when `hidden` is set, and say whether it did.
    pub(super) fn reset_if(&mut self, hidden: bool) -> bool {
        if hidden {
            self.armed = false;
        }
        hidden
    }

    pub(super) fn observe(&mut self, host: &DigitalHost, bound: Instant) -> Option<u32> {
        host.scheduler.due_events_into(bound, &mut self.due);
        if !self.armed {
            self.save(host);
            self.power = 1;
            return None;
        }
        self.distance += 1;
        let saved = self.saved.as_deref()?;
        if self.due == self.saved_due && host.repeats(saved) {
            return Some(self.distance);
        }
        if self.distance == self.power {
            self.save(host);
            self.power = self.power.saturating_mul(2);
        }
        None
    }

    fn save(&mut self, host: &DigitalHost) {
        match &mut self.saved {
            Some(saved) => (**saved).clone_from(host),
            None => self.saved = Some(Box::new(host.clone())),
        }
        std::mem::swap(&mut self.saved_due, &mut self.due);
        self.distance = 0;
        self.armed = true;
    }
}

impl DigitalHost {
    /// Whether this host is, for the rest of the time step being settled, in
    /// the state `saved` was.
    ///
    /// Sound in one direction only, and that is the direction that matters: a
    /// `true` must mean the step goes on exactly as it went on from `saved`.
    /// Every field is named, so a new one has to be placed here. Left out is
    /// what only describes the design or is scratch, and the write sequences
    /// a waiting process was armed at: with no change left undispatched every
    /// later change comes after all of them (see
    /// `DigitalSignalStore::same_step_state`). Work keyed by those sequences,
    /// and the parts of a design this proof does not cover (decaying charge,
    /// delayed drives, timing checks), are required absent instead: such a
    /// step is never called repeating and runs until it settles or is
    /// stopped. The kernel's due events are compared by the caller.
    pub(super) fn repeats(&self, saved: &Self) -> bool {
        let Self {
            checkpoint_failed: _,
            plan: _,
            store,
            scheduler: _,
            resolution: _,
            nba_target: _,
            charge: _,
            pending: _,
            checks: _,
            check_timer: _,
            drops: _,
            external_targets: _,
            external_real_targets: _,
            elaboration_closed: _,
            delayed_updates,
            event_updates: _,
            expression_updates: _,
            expression_processes: _,
            event_waiters: _,
            event_candidates: _,
            slots,
            free_threads,
            disabling: _,
            withdrawn: _,
            waiters,
            inactive,
            analog_waiters,
            analog_ready,
            analog_activation_seconds,
            analog_exchange_seconds,
            targets: _,
            process_of_target: _,
            fired: _,
            drained: _,
            scratch: _,
            effects,
            reverse_active: _,
            staged_start: _,
            deferred_start: _,
        } = self;
        let quiet = |host: &Self| {
            host.charge.is_none()
                && host.pending.is_none()
                && host.checks.is_none()
                && host.event_updates.is_empty()
                && host.expression_updates.is_empty()
                && host.expression_processes.is_empty()
                && host.event_waiters.iter().all(BTreeSet::is_empty)
                && host.event_candidates.is_empty()
                && host.disabling.is_none()
                && host.withdrawn.is_empty()
                && host.deferred_start.is_empty()
        };
        let bits = |value: &Option<f64>| value.map(f64::to_bits);
        quiet(self)
            && quiet(saved)
            && slots.len() == saved.slots.len()
            && slots
                .iter()
                .zip(&saved.slots)
                .all(|(now, then)| now.same_step_state(then))
            && *free_threads == saved.free_threads
            && *waiters == saved.waiters
            && *inactive == saved.inactive
            && *analog_waiters == saved.analog_waiters
            && *analog_ready == saved.analog_ready
            && *delayed_updates == saved.delayed_updates
            && bits(analog_activation_seconds) == bits(&saved.analog_activation_seconds)
            && bits(analog_exchange_seconds) == bits(&saved.analog_exchange_seconds)
            && store.same_step_state(&saved.store)
            && effects.same_step_state(&saved.effects)
    }
}

impl ProcessSlot {
    fn same_step_state(&self, saved: &Self) -> bool {
        let Self {
            process,
            parent,
            children,
            status,
            resume,
            wait_after_sequence: _,
            remaining_events,
        } = self;
        *process == saved.process
            && *parent == saved.parent
            && *children == saved.children
            && *status == saved.status
            && *resume == saved.resume
            && *remaining_events == saved.remaining_events
    }
}
