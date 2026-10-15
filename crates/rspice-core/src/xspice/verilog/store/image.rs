//! The store as part of a trial image; see `host/image.rs`.
//!
//! Both methods destructure the store, so a field added to it does not
//! compile until it has been placed here.

use super::*;

impl Clone for DigitalSignalStore {
    fn clone(&self) -> Self {
        let Self {
            plan,
            connected,
            external_reals,
            connected_reals,
            external_changes,
            external_batch,
            expression_waits,
            expression_inputs,
            expression_scratch,
            expression_error,
            expression_captures,
            values,
            reals,
            kinds,
            widths,
            variables,
            contributions,
            spans,
            deferred,
            deferred_ordered,
            event_captures,
            sequence,
            delayed,
            transitions,
            analog_samples,
            analog_variable_probes,
            analog_variable_inputs,
            activation_clock,
            traces,
            analog_edges,
            overrides,
        } = self;
        Self {
            plan: Arc::clone(plan),
            connected: connected.clone(),
            external_reals: external_reals.clone(),
            connected_reals: connected_reals.clone(),
            external_changes: external_changes.clone(),
            external_batch: external_batch.clone(),
            expression_waits: expression_waits.clone(),
            expression_inputs: expression_inputs.clone(),
            expression_scratch: expression_scratch.clone(),
            expression_error: expression_error.clone(),
            expression_captures: expression_captures.clone(),
            values: values.clone(),
            reals: reals.clone(),
            kinds: kinds.clone(),
            widths: widths.clone(),
            variables: variables.clone(),
            contributions: contributions.clone(),
            spans: spans.clone(),
            deferred: deferred.clone(),
            deferred_ordered: *deferred_ordered,
            event_captures: event_captures.clone(),
            sequence: *sequence,
            delayed: delayed.clone(),
            transitions: transitions.clone(),
            analog_samples: analog_samples.clone(),
            analog_variable_probes: analog_variable_probes.clone(),
            analog_variable_inputs: analog_variable_inputs.clone(),
            activation_clock: *activation_clock,
            traces: traces.clone(),
            analog_edges: analog_edges.clone(),
            overrides: overrides.clone(),
        }
    }

    /// Every field is overwritten with `source`'s, reusing what this store
    /// already holds — except the expression evaluator's scratch, which is
    /// storage rather than state for the reason `DigitalHost::clone_from`
    /// keeps the interpreter's.
    fn clone_from(&mut self, source: &Self) {
        let Self {
            plan,
            connected,
            external_reals,
            connected_reals,
            external_changes,
            external_batch,
            expression_waits,
            expression_inputs,
            expression_scratch: _,
            expression_error,
            expression_captures,
            values,
            reals,
            kinds,
            widths,
            variables,
            contributions,
            spans,
            deferred,
            deferred_ordered,
            event_captures,
            sequence,
            delayed,
            transitions,
            analog_samples,
            analog_variable_probes,
            analog_variable_inputs,
            activation_clock,
            traces,
            analog_edges,
            overrides,
        } = self;
        let Self {
            plan: source_plan,
            connected: source_connected,
            external_reals: source_external_reals,
            connected_reals: source_connected_reals,
            external_changes: source_external_changes,
            external_batch: source_external_batch,
            expression_waits: source_expression_waits,
            expression_inputs: source_expression_inputs,
            expression_scratch: _,
            expression_error: source_expression_error,
            expression_captures: source_expression_captures,
            values: source_values,
            reals: source_reals,
            kinds: source_kinds,
            widths: source_widths,
            variables: source_variables,
            contributions: source_contributions,
            spans: source_spans,
            deferred: source_deferred,
            deferred_ordered: source_deferred_ordered,
            event_captures: source_event_captures,
            sequence: source_sequence,
            delayed: source_delayed,
            transitions: source_transitions,
            analog_samples: source_analog_samples,
            analog_variable_probes: source_analog_variable_probes,
            analog_variable_inputs: source_analog_variable_inputs,
            activation_clock: source_activation_clock,
            traces: source_traces,
            analog_edges: source_analog_edges,
            overrides: source_overrides,
        } = source;
        plan.clone_from(source_plan);
        connected.clone_from(source_connected);
        external_reals.clone_from(source_external_reals);
        connected_reals.clone_from(source_connected_reals);
        external_changes.clone_from(source_external_changes);
        external_batch.clone_from(source_external_batch);
        expression_waits.clone_from(source_expression_waits);
        expression_inputs.clone_from(source_expression_inputs);
        expression_error.clone_from(source_expression_error);
        expression_captures.clone_from(source_expression_captures);
        values.clone_from(source_values);
        reals.clone_from(source_reals);
        kinds.clone_from(source_kinds);
        widths.clone_from(source_widths);
        variables.clone_from(source_variables);
        contributions.clone_from(source_contributions);
        spans.clone_from(source_spans);
        deferred.clone_from(source_deferred);
        *deferred_ordered = *source_deferred_ordered;
        event_captures.clone_from(source_event_captures);
        *sequence = *source_sequence;
        delayed.clone_from(source_delayed);
        transitions.clone_from(source_transitions);
        analog_samples.clone_from(source_analog_samples);
        analog_variable_probes.clone_from(source_analog_variable_probes);
        analog_variable_inputs.clone_from(source_analog_variable_inputs);
        *activation_clock = *source_activation_clock;
        traces.clone_from(source_traces);
        analog_edges.clone_from(source_analog_edges);
        overrides.clone_from(source_overrides);
    }
}
