//! Waveform changes belong to the same rollback image as their event values.
use super::EventValue;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct EventTracePoint {
    pub(crate) time: f64,
    pub(crate) node: usize,
    pub(crate) value: EventValue,
}

/// Disabled until transient initialization has produced its baseline snapshot.
/// Only retained nodes allocate entries; cloning a trial clones its pending
/// publications, so rejected events cannot escape into accepted results.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct EventTraceJournal {
    retained: Arc<[bool]>,
    points: Vec<EventTracePoint>,
}

impl EventTraceJournal {
    /// Trace retention is immutable checkpoint identity. Publications belong
    /// to the result sink and must be delivered before capture or replacement.
    pub(crate) fn checkpoint_retention(&self) -> Result<&[bool], String> {
        if !self.points.is_empty() {
            return Err("event waveform publications must be drained before checkpointing".into());
        }
        Ok(&self.retained)
    }

    pub(crate) fn configure(&mut self, retained: Arc<[bool]>) {
        self.retained = retained;
        self.points.clear();
    }

    pub(crate) fn record(&mut self, time: f64, node: usize, value: EventValue) {
        if node
            .checked_sub(1)
            .and_then(|index| self.retained.get(index))
            .copied()
            .unwrap_or(false)
        {
            self.points.push(EventTracePoint { time, node, value });
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    pub(crate) fn drain_into(&mut self, points: &mut Vec<EventTracePoint>) {
        points.append(&mut self.points);
    }
}

/// One waveform value per node, domain and physical instant. Stable ordering
/// retains the final delta-cycle value without inventing zero-duration pulses.
/// Digital and real domains remain distinct even on a hybrid circuit node.
pub(crate) fn settle_trace_points(points: &mut Vec<EventTracePoint>) {
    fn domain(value: EventValue) -> bool {
        matches!(value, EventValue::Real(_))
    }
    points.sort_by(|left, right| {
        left.time
            .total_cmp(&right.time)
            .then_with(|| left.node.cmp(&right.node))
            .then_with(|| domain(left.value).cmp(&domain(right.value)))
    });
    points.dedup_by(|later, earlier| {
        if later.time == earlier.time
            && later.node == earlier.node
            && domain(later.value) == domain(earlier.value)
        {
            earlier.value = later.value;
            true
        } else {
            false
        }
    });
}
