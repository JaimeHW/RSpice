//! Indexed waveform subscriptions, independent of functional event fanout.
use super::*;
use crate::xspice::{DigitalValue, EventValue};
use crate::xspice::event_trace::{EventTraceJournal, EventTracePoint};

#[derive(Clone, Copy)]
pub(crate) enum TraceSource {
    ResolvedBit(usize),
    SignalBit(DigitalSignalId, u32),
    Real(DigitalSignalId),
}

#[derive(Clone, Default)]
struct TraceBindings {
    bits: BTreeMap<DigitalSignalId, Vec<(usize, u32)>>,
    reals: BTreeMap<DigitalSignalId, Vec<usize>>,
    resolved: BTreeMap<usize, Vec<usize>>,
}

#[derive(Clone, Default)]
pub(super) struct StoreTraces {
    bindings: Arc<TraceBindings>,
    journal: EventTraceJournal,
}

impl DigitalSignalStore {
    pub(crate) fn configure_traces(
        &mut self,
        sources: &[(usize, TraceSource)],
        retained: Arc<[bool]>,
    ) {
        let mut bindings = TraceBindings::default();
        for &(node, source) in sources {
            if !node
                .checked_sub(1)
                .and_then(|index| retained.get(index))
                .copied()
                .unwrap_or(false)
            {
                continue;
            }
            match source {
                TraceSource::ResolvedBit(net) => {
                    bindings.resolved.entry(net).or_default().push(node)
                }
                TraceSource::SignalBit(signal, bit) => {
                    bindings.bits.entry(signal).or_default().push((node, bit))
                }
                TraceSource::Real(signal) => bindings.reals.entry(signal).or_default().push(node),
            }
        }
        self.traces.bindings = Arc::new(bindings);
        self.traces.journal.configure(retained);
    }

    pub(crate) fn drain_traces(&mut self, points: &mut Vec<EventTracePoint>) {
        self.traces.journal.drain_into(points);
    }

    pub(crate) fn has_traces(&self) -> bool {
        !self.traces.journal.is_empty()
    }

    pub(super) fn trace_transition(&mut self, signal: DigitalSignalId, values: &TransitionValues) {
        let Some(clock) = self.activation_clock else {
            return;
        };
        match values {
            TransitionValues::FourState { previous, next } => {
                if let Some(nodes) = self.traces.bindings.bits.get(&signal) {
                    for &(node, bit) in nodes {
                        if previous.bit(bit) == next.bit(bit) {
                            continue;
                        }
                        let value = match next.bit(bit) {
                            FourStateBit::Zero => DigitalValue::zero(),
                            FourStateBit::One => DigitalValue::one(),
                            FourStateBit::Unknown => DigitalValue::unknown(),
                            FourStateBit::HighImpedance => DigitalValue::new(
                                crate::xspice::DigitalState::HighZ,
                                crate::xspice::DigitalStrength::Strong,
                            ),
                        };
                        self.traces.journal.record(
                            clock.absolute_seconds,
                            node,
                            EventValue::Digital(value),
                        );
                    }
                }
            }
            TransitionValues::Real { next, .. } => {
                if let Some(nodes) = self.traces.bindings.reals.get(&signal) {
                    for &node in nodes {
                        self.traces.journal.record(
                            clock.absolute_seconds,
                            node,
                            EventValue::Real(*next),
                        );
                    }
                }
            }
        }
    }

    pub(super) fn trace_resolved_bit(&mut self, net: usize, value: DigitalValue) {
        let Some(clock) = self.activation_clock else {
            return;
        };
        if let Some(nodes) = self.traces.bindings.resolved.get(&net) {
            for &node in nodes {
                self.traces.journal.record(
                    clock.absolute_seconds,
                    node,
                    EventValue::Digital(value),
                );
            }
        }
    }
}
