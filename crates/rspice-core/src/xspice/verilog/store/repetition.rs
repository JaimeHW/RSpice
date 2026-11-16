//! The store's half of the proof that a time step repeats itself.

use super::*;

impl DigitalSignalStore {
    /// Whether this store is, for the rest of the time step being settled,
    /// the store `saved` was; see `DigitalHost::repeats`.
    ///
    /// Every field is named, so a new one has to be placed here. Two kinds
    /// are left out. What only describes the design (its plan, widths,
    /// topology, scratch buffers, traces written out) cannot differ. The
    /// write sequence and the orders it stamps only grow, so two states never
    /// share them; what they decide is the order of pending work against work
    /// to come, and with no change left undispatched (the host's eligibility
    /// test) every later write is ordered after all of it. Pending work that
    /// is keyed by those numbers rather than merely ordered by them — event
    /// and expression captures — is required empty instead of compared.
    pub(crate) fn same_step_state(&self, saved: &Self) -> bool {
        let Self {
            plan: _,
            connected,
            external_reals,
            connected_reals: _,
            external_changes: _,
            external_batch: _,
            expression_waits: _,
            expression_inputs: _,
            expression_scratch: _,
            expression_error: _,
            expression_captures: _,
            values,
            reals,
            kinds: _,
            widths: _,
            variables: _,
            contributions,
            spans: _,
            deferred,
            deferred_ordered: _,
            event_captures: _,
            sequence: _,
            delayed,
            transitions: _,
            analog_samples,
            analog_variable_probes,
            analog_variable_inputs: _,
            activation_clock,
            traces: _,
            analog_edges,
            overrides,
            strengths: _,
            passed,
            bridged,
            charge,
            delays,
        } = self;
        // Work whose order rests on the sequence itself must be absent, in
        // both states, rather than compared.
        let quiet = |store: &Self| {
            store.external_changes.is_empty()
                && store.external_batch.is_none()
                && store.expression_waits.is_empty()
                && store.expression_inputs.iter().all(BTreeSet::is_empty)
                && store.expression_error.is_none()
                && store.expression_captures.is_empty()
                && store.event_captures.is_empty()
                && store.transitions.is_empty()
                && store.deferred_ordered
        };
        quiet(self)
            && quiet(saved)
            && *values == saved.values
            && reals.len() == saved.reals.len()
            && reals
                .iter()
                .zip(&saved.reals)
                .all(|(now, then)| now.to_bits() == then.to_bits())
            && contributions.len() == saved.contributions.len()
            && contributions
                .iter()
                .zip(&saved.contributions)
                .all(|(now, then)| now.same_as(then))
            && deferred.len() == saved.deferred.len()
            && deferred
                .iter()
                .zip(&saved.deferred)
                .all(|(now, then)| now.update == then.update)
            && *delayed == saved.delayed
            && *analog_samples == saved.analog_samples
            && *analog_variable_probes == saved.analog_variable_probes
            && *activation_clock == saved.activation_clock
            && match (connected, &saved.connected) {
                (None, None) => true,
                (Some(now), Some(then)) => now.same_step_state(then),
                _ => false,
            }
            && match (external_reals, &saved.external_reals) {
                (None, None) => true,
                (Some(now), Some(then)) => now.same_step_state(then),
                _ => false,
            }
            && analog_edges.same_step_state(&saved.analog_edges)
            && overrides.same_step_state(&saved.overrides)
            && *passed == saved.passed
            && *bridged == saved.bridged
            && charge.same_step_state(&saved.charge)
            && delays.same_step_state(&saved.delays)
    }
}

impl Contribution {
    fn same_as(&self, saved: &Self) -> bool {
        self.low == saved.low
            && match (&self.value, &saved.value) {
                (ContributionValue::FourState(now), ContributionValue::FourState(then)) => {
                    now == then
                }
                (ContributionValue::Real(now), ContributionValue::Real(then)) => {
                    now.map(f64::to_bits) == then.map(f64::to_bits)
                }
                _ => false,
            }
    }
}
