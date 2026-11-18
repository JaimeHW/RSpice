//! Proof that a time step can never settle: a delta cycle leaves the design in
//! a state it was already in at that instant.
//!
//! A design is deterministic, so a step that comes back to a state it has been
//! in goes round the same cycle for ever. IEEE 1364-2005 section 11.4 leaves
//! the settling of a step unbounded, and no count tells a long legal step (a
//! zero-time counter, a deep nonblocking chain) from a loop; a repeated state
//! does. The search is Brent's (BIT 20, 1980): one saved state, replaced each
//! time the distance from it reaches a power of two, compared with the live
//! state. Compared after every delta cycle, a cycle of period λ entered after
//! μ delta cycles is found within about 2(μ + λ) of the search starting;
//! compared every s delta cycles, within about 2(μ + lcm(λ, s)).
//!
//! The search starts only once a step has run [`REPETITION_SUSPICION`] delta
//! cycles, so a step that settles sooner pays nothing for it. A comparison
//! visits every process and every signal, while a delta cycle of a long legal
//! chain may touch one of each: compared after every delta cycle, a count of
//! 200,000 delta cycles beside 2,000 idle processes ran 25 times slower. So
//! the state is compared once every [`comparison_stride`] delta cycles, a
//! number that grows with the size of the state, which keeps the comparisons
//! to a small part of the step however large the design. The saved copy is
//! refilled in place at each power of two. A repeat found that way is a
//! multiple of the design's period, and the period itself divides it: the
//! search runs on from the repeat, comparing only at those divisors, and
//! names the first one at which the state comes back.

use super::*;
use crate::xspice::event_scheduler::DueEvent;

/// Delta cycles of one time step before the host starts looking for a
/// repeated state.
///
/// The longest settling a legal design was measured to need without a `#0`
/// is 76 delta cycles (the mixed-signal corpus) and 12 in the ivtest
/// selection. Past this, the search costs a small part of the step
/// ([`comparison_stride`]), and an astable startup or a zero-delay loop is
/// refused after about a thousand delta cycles instead of after spinning
/// through tens of thousands.
pub(super) const REPETITION_SUSPICION: u32 = 1 << 10;

/// Processes and signals a comparison may visit per delta cycle it stands
/// for. A comparison visits one in about the time an interpreted delta cycle
/// takes to run one process and dispatch what it wrote, so at this rate the
/// comparisons cost about a twentieth of the delta cycles they cover.
const ELEMENTS_PER_DELTA_CYCLE: usize = 16;

/// What a comparison costs before it reaches the processes and signals (the
/// kernel's due events, the work that must be absent), in the same units.
const COMPARISON_OVERHEAD: usize = 128;

/// Delta cycles between two comparisons of `host`'s state.
fn comparison_stride(host: &DigitalHost) -> u32 {
    let elements = host.slots.len() + host.waiters.len() + COMPARISON_OVERHEAD;
    u32::try_from(elements / ELEMENTS_PER_DELTA_CYCLE).map_or(u32::MAX, |stride| stride.max(1))
}

/// Brent's search over one time step's states.
#[derive(Default)]
pub(super) struct RepetitionSearch {
    /// The state compared against, once one is taken; refilled in place.
    saved: Option<Box<DigitalHost>>,
    /// Whether `saved` holds a state of this search.
    armed: bool,
    /// The kernel's due events in `saved`.
    saved_due: Vec<DueEvent>,
    /// The kernel's due events now, reused across comparisons.
    due: Vec<DueEvent>,
    /// Delta cycles between two comparisons.
    stride: u32,
    /// Delta cycles left before the next comparison.
    countdown: u32,
    /// Comparisons since `saved` was taken.
    distance: u32,
    /// The distance at which `saved` is replaced.
    power: u32,
    /// Once the state has come back: how many delta cycles it took, which
    /// the period divides, and how many have run since.
    refining: Option<(u32, u32)>,
}

impl RepetitionSearch {
    /// Account for the delta cycle just counted. `Some(period)` once the
    /// state is proven to repeat: the step goes round every `period` delta
    /// cycles and can never settle.
    ///
    /// `hidden` says whether a participant holds state of its own that could
    /// break a cycle the host's state alone repeats. It is asked only where a
    /// state is taken or compared: a state taken while it does is no proof,
    /// and the search starts over.
    pub(super) fn observe(
        &mut self,
        host: &DigitalHost,
        bound: Instant,
        hidden: impl FnOnce() -> bool,
    ) -> Option<u32> {
        if !self.armed {
            if !hidden() {
                self.stride = comparison_stride(host);
                self.countdown = self.stride;
                self.refining = None;
                self.power = 1;
                host.scheduler.due_events_into(bound, &mut self.due);
                self.save(host);
            }
            return None;
        }
        if let Some((multiple, elapsed)) = &mut self.refining {
            *elapsed += 1;
            let (multiple, elapsed) = (*multiple, *elapsed);
            // `saved` is the state the repeat was found in. The period
            // divides `multiple`, so only a divisor can be it, and at
            // `multiple` itself the state is back whatever else holds.
            return (multiple % elapsed == 0
                && (elapsed == multiple || (!hidden() && self.matches(host, bound))))
            .then_some(elapsed);
        }
        self.countdown -= 1;
        if self.countdown > 0 {
            return None;
        }
        self.countdown = self.stride;
        if hidden() {
            self.armed = false;
            return None;
        }
        self.distance += 1;
        if self.matches(host, bound) {
            let multiple = self.distance * self.stride;
            if self.stride == 1 {
                return Some(multiple);
            }
            self.refining = Some((multiple, 0));
        } else if self.distance == self.power {
            self.save(host);
            self.power = self.power.saturating_mul(2);
        }
        None
    }

    /// Whether `host`, whose due events are the kernel's at `bound`, is in
    /// the saved state.
    fn matches(&mut self, host: &DigitalHost, bound: Instant) -> bool {
        host.scheduler.due_events_into(bound, &mut self.due);
        self.saved
            .as_deref()
            .is_some_and(|saved| self.due == self.saved_due && host.repeats(saved))
    }

    /// Take `host`, whose due events were just read into `due`, as the state
    /// compared against.
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
            holds_monitor_region: _,
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
