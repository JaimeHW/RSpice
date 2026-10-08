//! Sparse current updates and bounded, exact-time preview retention.

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use rspice_core::{
    CurrentImpulseDerivative, CurrentImpulseOwner, CurrentImpulsePoint, CurrentImpulseTrace,
};
use serde::{Deserialize, Serialize};

use super::{MAX_PENDING_LIVE_TRANSIENT_SAMPLES, TransientSampleDelta};
use rspice_results::current_impulses::CurrentImpulseHistoryEvidence;

const MAX_CURRENT_POINTS: usize = 8_192;
const MAX_CURRENT_OWNERS: usize = 8_192;
// Sequence numbers remain exact through the browser's JSON-number boundary.
const MAX_SEQUENCE: u64 = (1_u64 << 53) - 1;

/// A sparse suffix, or several consecutive suffixes coalesced by the queue.
/// Extents refer to the entire run segment; point times remain independent
/// of the analog sample carrying this message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "O: Serialize, O::Point: Serialize",
        deserialize = "O: Deserialize<'de>, O::Point: Deserialize<'de>"
    )
)]
pub struct ImpulseDelta<O: ImpulseOwner> {
    pub start_time_s: f64,
    pub stop_time_s: f64,
    pub first_sequence: u64,
    pub last_sequence: u64,
    pub delivery_complete: bool,
    pub traces: Vec<ImpulseUpdate<O>>,
}

/// Unlike a retained trace, an update can revoke coverage without adding a
/// point. Incomplete empty updates therefore have a distinct wire type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    deny_unknown_fields,
    bound(
        serialize = "O: Serialize, O::Point: Serialize",
        deserialize = "O: Deserialize<'de>, O::Point: Deserialize<'de>"
    )
)]
pub struct ImpulseUpdate<O: ImpulseOwner> {
    pub owner: O,
    pub complete: bool,
    pub points: Vec<O::Point>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derivatives: Vec<CurrentImpulseDerivative>,
}

impl<O: ImpulseOwner> ImpulseDelta<O> {
    fn valid(&self) -> bool {
        if !self.start_time_s.is_finite()
            || self.start_time_s < 0.0
            || !self.stop_time_s.is_finite()
            || self.stop_time_s < self.start_time_s
            || self.first_sequence > self.last_sequence
            || self.last_sequence > MAX_SEQUENCE
            || self.traces.len() > MAX_CURRENT_OWNERS
        {
            return false;
        }
        let mut count = 0usize;
        let mut owners = HashSet::new();
        for trace in &self.traces {
            count = count
                .saturating_add(trace.points.len())
                .saturating_add(trace.derivatives.len());
            let Some(key) = trace.owner.canonical_key() else {
                return false;
            };
            if count > MAX_CURRENT_POINTS || !owners.insert(key) {
                return false;
            }
            let mut previous = None;
            for point in &trace.points {
                if !point.time().is_finite()
                    || point.time() < self.start_time_s
                    || point.time() > self.stop_time_s
                    || !point.coefficient().is_finite()
                    || point.coefficient() == 0.0
                    || previous.is_some_and(|time| point.time() <= time)
                {
                    return false;
                }
                previous = Some(point.time());
            }
            let mut previous = None;
            for point in &trace.derivatives {
                if !point.time.is_finite()
                    || point.time < self.start_time_s
                    || point.time > self.stop_time_s
                    || point.order == 0
                    || !point.coefficient.is_finite()
                    || point.coefficient == 0.0
                    || previous.is_some_and(|pair| (point.time, point.order) <= pair)
                {
                    return false;
                }
                previous = Some((point.time, point.order));
            }
        }
        true
    }
}

#[derive(Debug, Default)]
struct PublishedCursor {
    count: usize,
    derivatives: usize,
    complete: bool,
}

/// The engine reports cumulative accepted history. Only newly appended
/// points and changed coverage cross the live boundary.
#[derive(Debug)]
pub(super) struct PublishedImpulses<O: ImpulseOwner> {
    cursors: HashMap<O, PublishedCursor>,
    start: Option<f64>,
    sequence: u64,
    lost: bool,
}

impl<O: ImpulseOwner> PublishedImpulses<O> {
    pub(super) fn publish(
        &mut self,
        sample: &rspice_core::abort_signal::TransientSample<'_>,
    ) -> Option<ImpulseDelta<O>> {
        let stop = *sample.time.last()?;
        let Some(source) = O::source(sample) else {
            self.lost |= self.start.is_some();
            return None;
        };
        let start = *self.start.get_or_insert(*sample.time.first()?);
        let sequence = self.sequence;
        if self.sequence < MAX_SEQUENCE {
            self.sequence += 1;
        } else {
            self.lost = true;
        }
        let mut remaining = MAX_CURRENT_POINTS;
        let mut updates = Vec::new();
        self.lost |= source.len() > MAX_CURRENT_OWNERS || source.len() < self.cursors.len();
        for source_trace in source.iter().take(MAX_CURRENT_OWNERS) {
            let trace = O::view(source_trace);
            let new = !self.cursors.contains_key(trace.owner);
            if new {
                if self.cursors.len() >= MAX_CURRENT_OWNERS {
                    self.lost = true;
                    continue;
                }
                self.cursors
                    .insert(trace.owner.clone(), PublishedCursor::default());
            }
            let cursor = self.cursors.get_mut(trace.owner).expect("inserted cursor");
            let suffix = match trace.points.get(cursor.count..) {
                Some(suffix) => suffix,
                None => {
                    self.lost = true;
                    &[]
                }
            };
            let derivatives = match trace.derivatives.get(cursor.derivatives..) {
                Some(suffix) => suffix,
                None => {
                    self.lost = true;
                    &[]
                }
            };
            if new
                || !suffix.is_empty()
                || !derivatives.is_empty()
                || cursor.complete != trace.complete
            {
                let count = suffix.len().min(remaining);
                let derivative_count = derivatives.len().min(remaining - count);
                self.lost |= count != suffix.len() || derivative_count != derivatives.len();
                updates.push(ImpulseUpdate {
                    owner: trace.owner.clone(),
                    complete: trace.complete,
                    points: suffix[..count].to_vec(),
                    derivatives: derivatives[..derivative_count].to_vec(),
                });
                remaining -= count + derivative_count;
            }
            cursor.count = trace.points.len();
            cursor.derivatives = trace.derivatives.len();
            cursor.complete = trace.complete;
        }
        Some(ImpulseDelta {
            start_time_s: start,
            stop_time_s: stop,
            first_sequence: sequence,
            last_sequence: sequence,
            delivery_complete: !self.lost,
            traces: updates,
        })
    }
}

#[derive(Debug)]
struct ImpulseBatch<O: ImpulseOwner> {
    start: f64,
    stop: f64,
    first_sequence: u64,
    last_sequence: u64,
    traces: BTreeMap<O, ImpulseUpdate<O>>,
    point_count: usize,
}

/// Used once in the pending queue and once in the displayed preview. Each
/// buffer has one aggregate point/owner budget, independent of queue length.
#[derive(Debug)]
pub struct ImpulseBuffer<O: ImpulseOwner> {
    batch: Option<ImpulseBatch<O>>,
    lost: bool,
}

impl<O: ImpulseOwner> ImpulseBuffer<O> {
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn is_empty(&self) -> bool {
        self.batch.is_none()
    }

    pub fn mark_lost(&mut self) {
        self.lost = true;
    }

    pub fn ingest(&mut self, delta: ImpulseDelta<O>) {
        if !delta.valid() {
            self.lost = true;
            return;
        }
        self.lost |= !delta.delivery_complete;
        if let Some(batch) = &self.batch {
            // Entirely replayed batches cannot append a past impulse again.
            if delta.last_sequence <= batch.last_sequence {
                return;
            }
            if delta.start_time_s != batch.start || delta.stop_time_s < batch.stop {
                self.lost = true;
                return;
            }
            self.lost |= batch.last_sequence.checked_add(1) != Some(delta.first_sequence);
        }
        let batch = self.batch.get_or_insert_with(|| ImpulseBatch {
            start: delta.start_time_s,
            stop: delta.stop_time_s,
            first_sequence: delta.first_sequence,
            last_sequence: delta.last_sequence,
            traces: BTreeMap::new(),
            point_count: 0,
        });
        batch.stop = delta.stop_time_s;
        batch.last_sequence = delta.last_sequence;
        for update in delta.traces {
            let key = update.owner.canonical_key().expect("validated owner");
            if !batch.traces.contains_key(&key) && batch.traces.len() >= MAX_CURRENT_OWNERS {
                self.lost = true;
                continue;
            }
            let trace = batch.traces.entry(key).or_insert_with(|| ImpulseUpdate {
                owner: update.owner.clone(),
                complete: update.complete,
                points: Vec::new(),
                derivatives: Vec::new(),
            });
            trace.complete &= update.complete;
            for point in update.points {
                if let Some(last) = trace.points.last()
                    && point.time() <= last.time()
                {
                    self.lost |= point != *last;
                    continue;
                }
                if batch.point_count >= MAX_CURRENT_POINTS {
                    self.lost = true;
                    continue;
                }
                trace.points.push(point);
                batch.point_count += 1;
            }
            for point in update.derivatives {
                if let Some(last) = trace.derivatives.last()
                    && (point.time, point.order) <= (last.time, last.order)
                {
                    self.lost |= point != *last;
                    continue;
                }
                if batch.point_count >= MAX_CURRENT_POINTS {
                    self.lost = true;
                    continue;
                }
                trace.derivatives.push(point);
                batch.point_count += 1;
            }
        }
    }

    pub fn history(&self) -> Option<O::History> {
        let batch = self.batch.as_ref()?;
        Some(O::history(
            batch.start,
            batch.stop,
            !self.lost && batch.first_sequence == 0,
            batch
                .traces
                .values()
                .filter(|trace| {
                    trace.complete || !trace.points.is_empty() || !trace.derivatives.is_empty()
                })
                .map(O::trace)
                .collect(),
        ))
    }

    fn take_delta(&mut self) -> Option<ImpulseDelta<O>> {
        let batch = self.batch.take()?;
        Some(ImpulseDelta {
            start_time_s: batch.start,
            stop_time_s: batch.stop,
            first_sequence: batch.first_sequence,
            last_sequence: batch.last_sequence,
            delivery_complete: !std::mem::take(&mut self.lost),
            traces: batch.traces.into_values().collect(),
        })
    }
}

/// Physical point traits keep current charge and voltage action units distinct.
pub trait ImpulsePoint: Copy + std::fmt::Debug + PartialEq {
    fn time(self) -> f64;
    fn coefficient(self) -> f64;
}
impl ImpulsePoint for CurrentImpulsePoint {
    fn time(self) -> f64 {
        self.time
    }
    fn coefficient(self) -> f64 {
        self.charge_coulombs
    }
}
impl ImpulsePoint for rspice_core::VoltageImpulsePoint {
    fn time(self) -> f64 {
        self.time
    }
    fn coefficient(self) -> f64 {
        self.volt_seconds
    }
}
pub struct ImpulseView<'a, O: ImpulseOwner> {
    owner: &'a O,
    complete: bool,
    points: &'a [O::Point],
    derivatives: &'a [CurrentImpulseDerivative],
}
pub trait ImpulseOwner: Clone + std::fmt::Debug + Eq + Ord + std::hash::Hash {
    type Point: ImpulsePoint;
    type Trace;
    type History;
    fn canonical_key(&self) -> Option<Self>;
    fn source<'a>(
        sample: &rspice_core::abort_signal::TransientSample<'a>,
    ) -> Option<&'a [Self::Trace]>;
    fn view(trace: &Self::Trace) -> ImpulseView<'_, Self>;
    fn trace(update: &ImpulseUpdate<Self>) -> Self::Trace;
    fn history(start: f64, stop: f64, complete: bool, traces: Vec<Self::Trace>) -> Self::History;
}
impl ImpulseOwner for CurrentImpulseOwner {
    type Point = CurrentImpulsePoint;
    type Trace = CurrentImpulseTrace;
    type History = CurrentImpulseHistoryEvidence;
    fn canonical_key(&self) -> Option<Self> {
        owner_key(self)
    }
    fn source<'a>(
        sample: &rspice_core::abort_signal::TransientSample<'a>,
    ) -> Option<&'a [Self::Trace]> {
        sample.current_impulses
    }
    fn view(trace: &Self::Trace) -> ImpulseView<'_, Self> {
        ImpulseView {
            owner: &trace.owner,
            complete: trace.complete,
            points: &trace.points,
            derivatives: &trace.derivatives,
        }
    }
    fn trace(update: &ImpulseUpdate<Self>) -> Self::Trace {
        CurrentImpulseTrace {
            owner: update.owner.clone(),
            complete: update.complete,
            points: update.points.clone(),
            derivatives: update.derivatives.clone(),
        }
    }
    fn history(start: f64, stop: f64, complete: bool, traces: Vec<Self::Trace>) -> Self::History {
        CurrentImpulseHistoryEvidence {
            start_time_s: start,
            stop_time_s: stop,
            delivery_complete: complete,
            traces,
        }
    }
}
impl ImpulseOwner for String {
    type Point = rspice_core::VoltageImpulsePoint;
    type Trace = rspice_core::VoltageImpulseTrace;
    type History = rspice_results::voltage_impulses::VoltageImpulseHistoryEvidence;
    fn canonical_key(&self) -> Option<Self> {
        (!self.trim().is_empty()).then(|| self.to_ascii_lowercase())
    }
    fn source<'a>(
        sample: &rspice_core::abort_signal::TransientSample<'a>,
    ) -> Option<&'a [Self::Trace]> {
        sample.voltage_impulses
    }
    fn view(trace: &Self::Trace) -> ImpulseView<'_, Self> {
        ImpulseView {
            owner: &trace.node_name,
            complete: trace.complete,
            points: &trace.points,
            derivatives: &trace.derivatives,
        }
    }
    fn trace(update: &ImpulseUpdate<Self>) -> Self::Trace {
        rspice_core::VoltageImpulseTrace {
            node_name: update.owner.clone(),
            complete: update.complete,
            points: update.points.clone(),
            derivatives: update.derivatives.clone(),
        }
    }
    fn history(start: f64, stop: f64, complete: bool, traces: Vec<Self::Trace>) -> Self::History {
        rspice_results::voltage_impulses::VoltageImpulseHistoryEvidence {
            start_time_s: start,
            stop_time_s: stop,
            delivery_complete: complete,
            traces,
        }
    }
}
pub type CurrentImpulseDelta = ImpulseDelta<CurrentImpulseOwner>;
pub type CurrentImpulseUpdate = ImpulseUpdate<CurrentImpulseOwner>;
pub type CurrentImpulseBuffer = ImpulseBuffer<CurrentImpulseOwner>;
pub(super) type PublishedCurrentImpulses = PublishedImpulses<CurrentImpulseOwner>;
pub type VoltageImpulseDelta = ImpulseDelta<String>;
pub type VoltageImpulseUpdate = ImpulseUpdate<String>;
pub type VoltageImpulseBuffer = ImpulseBuffer<String>;
pub(super) type PublishedVoltageImpulses = PublishedImpulses<String>;
impl<O: ImpulseOwner> Default for ImpulseBuffer<O> {
    fn default() -> Self {
        Self {
            batch: None,
            lost: false,
        }
    }
}
impl<O: ImpulseOwner> Default for PublishedImpulses<O> {
    fn default() -> Self {
        Self {
            cursors: HashMap::new(),
            start: None,
            sequence: 0,
            lost: false,
        }
    }
}
fn owner_key(owner: &CurrentImpulseOwner) -> Option<CurrentImpulseOwner> {
    match owner {
        CurrentImpulseOwner::Branch { branch_name } if !branch_name.trim().is_empty() => {
            Some(CurrentImpulseOwner::Branch {
                branch_name: branch_name.to_ascii_lowercase(),
            })
        }
        CurrentImpulseOwner::DeviceLead {
            device_name,
            parameter,
        } if !device_name.trim().is_empty() && !parameter.trim().is_empty() => {
            Some(CurrentImpulseOwner::DeviceLead {
                device_name: device_name.to_ascii_lowercase(),
                parameter: parameter.to_ascii_lowercase(),
            })
        }
        _ => None,
    }
}
#[derive(Debug, Default)]
pub struct LiveTransientQueue {
    pub(super) samples: VecDeque<TransientSampleDelta>,
    impulses: CurrentImpulseBuffer,
    voltage_impulses: VoltageImpulseBuffer,
    event_points: usize,
    events_lost: bool,
}

impl LiveTransientQueue {
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// A transport discarded a message before a typed sample was available.
    pub fn mark_event_delivery_lost(&mut self) {
        self.events_lost = true;
    }

    pub fn push(&mut self, mut delta: TransientSampleDelta) {
        if !delta.time.is_finite() {
            self.impulses.mark_lost();
            self.voltage_impulses.mark_lost();
            self.events_lost = true;
            return;
        }
        if let Some(impulses) = delta.current_impulses.take() {
            if impulses.stop_time_s == delta.time {
                self.impulses.ingest(impulses);
            } else {
                self.impulses.mark_lost();
            }
        }
        if let Some(impulses) = delta.voltage_impulses.take() {
            if impulses.stop_time_s == delta.time {
                self.voltage_impulses.ingest(impulses);
            } else {
                self.voltage_impulses.mark_lost();
            }
        }
        self.events_lost |= !delta.event_delivery_complete;
        let count = delta.events.len().saturating_add(delta.real_events.len());
        if count > super::MAX_PENDING_LIVE_EVENT_POINTS {
            self.events_lost = true;
            delta.events.clear();
            delta.real_events.clear();
        }
        let count = delta.events.len() + delta.real_events.len();
        while self.samples.len() >= MAX_PENDING_LIVE_TRANSIENT_SAMPLES
            || self.event_points.saturating_add(count) > super::MAX_PENDING_LIVE_EVENT_POINTS
        {
            let Some(discarded) = self.samples.pop_front() else {
                break;
            };
            let removed = discarded.events.len() + discarded.real_events.len();
            self.event_points = self.event_points.saturating_sub(removed);
            self.events_lost |= removed != 0 || !discarded.buses.is_empty();
        }
        self.event_points += count;
        delta.event_delivery_complete &= !self.events_lost;
        self.samples.push_back(delta);
    }

    pub fn drain(&mut self) -> Vec<TransientSampleDelta> {
        let mut samples: Vec<_> = self.samples.drain(..).collect();
        self.event_points = 0;
        if self.events_lost {
            for sample in &mut samples {
                sample.event_delivery_complete = false;
            }
        }
        if let Some(impulses) = self.impulses.take_delta() {
            // A separate event-only message preserves the interval even if
            // a later analog sample has no recorded current section.
            samples.push(TransientSampleDelta {
                event_delivery_complete: true,
                time: impulses.stop_time_s,
                waveforms: vec![],
                events: vec![],
                real_events: vec![],
                buses: vec![],
                current_impulses: Some(impulses),
                voltage_impulses: None,
            });
        }
        if let Some(impulses) = self.voltage_impulses.take_delta() {
            samples.push(TransientSampleDelta {
                event_delivery_complete: true,
                time: impulses.stop_time_s,
                waveforms: vec![],
                events: vec![],
                real_events: vec![],
                buses: vec![],
                current_impulses: None,
                voltage_impulses: Some(impulses),
            });
        }
        samples
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn delta(sequence: u64, points: &[(f64, f64)]) -> CurrentImpulseDelta {
        CurrentImpulseDelta {
            start_time_s: 0.0,
            stop_time_s: sequence as f64,
            first_sequence: sequence,
            last_sequence: sequence,
            delivery_complete: true,
            traces: vec![CurrentImpulseUpdate {
                derivatives: Vec::new(),
                owner: CurrentImpulseOwner::Branch {
                    branch_name: "V1".into(),
                },
                complete: true,
                points: points
                    .iter()
                    .map(|&(time, charge_coulombs)| CurrentImpulsePoint {
                        time,
                        charge_coulombs,
                    })
                    .collect(),
            }],
        }
    }

    #[test]
    fn live_current_derivatives_keep_same_time_orders_and_coverage() {
        let first = CurrentImpulseDerivative {
            time: 0.25,
            order: 1,
            coefficient: -1e-21,
        };
        let mut update = delta(1, &[]);
        update.first_sequence = 0;
        update.traces[0].complete = false;
        update.traces[0].derivatives = vec![first, CurrentImpulseDerivative { order: 2, ..first }];
        let wire = serde_json::to_string(&update).unwrap();
        let restored: CurrentImpulseDelta = serde_json::from_str(&wire).unwrap();
        assert_eq!(restored, update);
        let expected = update.traces[0].derivatives.clone();
        let mut buffer = CurrentImpulseBuffer::default();
        buffer.ingest(restored);
        let history = buffer.history().unwrap();
        assert!(history.delivery_complete);
        assert_eq!(history.traces.len(), 1);
        assert_eq!(history.traces[0].derivatives, expected);
        history.validate().unwrap();
        buffer.ingest(update); // An entirely replayed batch adds nothing.
        assert_eq!(buffer.history().unwrap(), history);
    }

    #[test]
    fn live_current_derivative_suffixes_are_bounded_and_never_replayed() {
        let source = [CurrentImpulseTrace {
            owner: CurrentImpulseOwner::Branch {
                branch_name: "H1".into(),
            },
            complete: true,
            points: vec![],
            derivatives: (0..=MAX_CURRENT_POINTS)
                .map(|index| CurrentImpulseDerivative {
                    time: index as f64,
                    order: 1,
                    coefficient: -1e-21,
                })
                .collect(),
        }];
        let sample = rspice_core::abort_signal::TransientSample {
            event_changes: None,
            time: &[0.0, MAX_CURRENT_POINTS as f64],
            node_names: &[],
            node_voltages: &[],
            branch_names: &[],
            branch_currents: &[],
            voltage_impulses: None,
            current_impulses: Some(&source),
            digital_values: &[],
            digital_buses: &[],
            real_values: &[],
        };
        let mut publisher = PublishedCurrentImpulses::default();
        let first = publisher.publish(&sample).unwrap();
        assert!(!first.delivery_complete);
        assert_eq!(first.traces[0].derivatives.len(), MAX_CURRENT_POINTS);
        assert!(publisher.publish(&sample).unwrap().traces.is_empty());
        let mut buffer = CurrentImpulseBuffer::default();
        buffer.ingest(first);
        let history = buffer.history().unwrap();
        assert!(!history.delivery_complete);
        assert_eq!(history.traces[0].derivatives.len(), MAX_CURRENT_POINTS);
        history.validate().unwrap();
    }

    #[test]
    fn live_current_impulses_survive_analog_queue_eviction_and_wire_round_trip() {
        let mut queue = LiveTransientQueue::default();
        for index in 0..MAX_PENDING_LIVE_TRANSIENT_SAMPLES + 17 {
            let mut impulse = delta(index as u64, &[]);
            if index == 0 {
                impulse.traces[0].points.push(CurrentImpulsePoint {
                    time: 0.0,
                    charge_coulombs: -0.002,
                });
            }
            if index == MAX_PENDING_LIVE_TRANSIENT_SAMPLES {
                impulse.traces[0].points.push(CurrentImpulsePoint {
                    time: index as f64 - 0.25,
                    charge_coulombs: 0.004,
                });
            }
            // Real producers send no repeated unchanged owner declarations.
            if index != 0 && impulse.traces[0].points.is_empty() {
                impulse.traces.clear();
            }
            queue.push(TransientSampleDelta {
                event_delivery_complete: true,
                time: index as f64,
                waveforms: vec![],
                events: vec![],
                real_events: vec![],
                buses: vec![],
                voltage_impulses: None,
                current_impulses: Some(impulse),
            });
        }
        let drained = queue.drain();
        assert_eq!(drained.len(), MAX_PENDING_LIVE_TRANSIENT_SAMPLES + 1);
        assert_eq!(drained[0].time, 17.0);
        assert!(
            drained[..drained.len() - 1]
                .iter()
                .all(|sample| sample.current_impulses.is_none())
        );
        let serialized = serde_json::to_string(drained.last().unwrap()).unwrap();
        let restored: TransientSampleDelta = serde_json::from_str(&serialized).unwrap();
        let mut preview = CurrentImpulseBuffer::default();
        preview.ingest(restored.current_impulses.unwrap());
        let history = preview.history().unwrap();
        history.validate().unwrap();
        assert!(history.delivery_complete);
        assert_eq!(
            history.traces[0].points,
            [
                CurrentImpulsePoint {
                    time: 0.0,
                    charge_coulombs: -0.002
                },
                CurrentImpulsePoint {
                    time: MAX_PENDING_LIVE_TRANSIENT_SAMPLES as f64 - 0.25,
                    charge_coulombs: 0.004
                },
            ]
        );
        assert!(queue.drain().is_empty());
    }

    #[test]
    fn live_current_impulses_detect_missing_batches_without_replaying_or_restoring_coverage() {
        let mut buffer = CurrentImpulseBuffer::default();
        buffer.ingest(delta(0, &[]));
        let first = delta(1, &[(0.3, -0.002)]);
        buffer.ingest(first.clone());
        buffer.ingest(first);
        buffer.ingest(delta(3, &[(2.7, 0.004)])); // missing sequence 2
        let mut revoked = delta(4, &[]);
        revoked.traces[0].complete = false;
        buffer.ingest(revoked);
        buffer.ingest(delta(5, &[]));
        let history = buffer.history().unwrap();
        history.validate().unwrap();
        assert!(!history.delivery_complete);
        assert!(!history.traces[0].complete);
        assert_eq!(history.traces[0].points.len(), 2);
        buffer.clear();
        assert!(buffer.history().is_none());
        buffer.ingest(delta(5, &[]));
        assert!(
            !buffer.history().unwrap().delivery_complete,
            "missing initial declarations are unknown"
        );
    }

    #[test]
    fn live_current_impulses_bound_aggregate_storage_and_preserve_loss_across_drains() {
        let source = [CurrentImpulseTrace {
            derivatives: Vec::new(),
            owner: CurrentImpulseOwner::Branch {
                branch_name: "V1".into(),
            },
            complete: true,
            points: (0..MAX_CURRENT_POINTS + 1)
                .map(|index| CurrentImpulsePoint {
                    time: index as f64,
                    charge_coulombs: 1e-12,
                })
                .collect(),
        }];
        let mut publisher = PublishedCurrentImpulses::default();
        let sample = rspice_core::abort_signal::TransientSample {
            event_changes: None,
            time: &[0.0, MAX_CURRENT_POINTS as f64],
            node_names: &[],
            node_voltages: &[],
            branch_names: &[],
            branch_currents: &[],
            voltage_impulses: None,
            current_impulses: Some(&source),
            digital_values: &[],
            digital_buses: &[],
            real_values: &[],
        };
        let published = publisher.publish(&sample).unwrap();
        assert!(!published.delivery_complete);
        assert_eq!(published.traces[0].points.len(), MAX_CURRENT_POINTS);
        assert!(
            publisher.publish(&sample).unwrap().traces.is_empty(),
            "a clipped suffix must not replay"
        );
        let mut buffer = CurrentImpulseBuffer::default();
        let mut initial = delta(0, &[]);
        initial.stop_time_s = MAX_CURRENT_POINTS as f64;
        initial.traces[0].points = (0..MAX_CURRENT_POINTS)
            .map(|index| CurrentImpulsePoint {
                time: index as f64,
                charge_coulombs: -1e-12,
            })
            .collect();
        buffer.ingest(initial);
        let mut extra = delta(1, &[(MAX_CURRENT_POINTS as f64 + 0.5, 1e-12)]);
        extra.stop_time_s = MAX_CURRENT_POINTS as f64 + 1.0;
        buffer.ingest(extra);
        let batch = buffer.take_delta().unwrap();
        assert_eq!(batch.traces[0].points.len(), MAX_CURRENT_POINTS);
        assert!(!batch.delivery_complete);
        let mut preview = CurrentImpulseBuffer::default();
        preview.ingest(batch);
        let mut later = delta(2, &[]);
        later.stop_time_s = MAX_CURRENT_POINTS as f64 + 2.0;
        preview.ingest(later);
        assert!(!preview.history().unwrap().delivery_complete);
        assert_eq!(
            preview.history().unwrap().traces[0].points.len(),
            MAX_CURRENT_POINTS
        );
    }

    #[test]
    fn live_current_impulses_reject_malformed_updates_and_keep_empty_coverage_revocations() {
        let mut buffer = CurrentImpulseBuffer::default();
        buffer.ingest(delta(0, &[]));
        let mut invalid = delta(1, &[(0.3, f64::NAN)]);
        buffer.ingest(invalid.clone());
        invalid.traces[0].points[0].charge_coulombs = 0.0;
        buffer.ingest(invalid);
        let mut revoked = delta(2, &[]);
        revoked.traces[0].complete = false;
        buffer.ingest(revoked);
        let batch = buffer.take_delta().unwrap();
        assert!(!batch.delivery_complete);
        assert!(!batch.traces[0].complete && batch.traces[0].points.is_empty());
        let mut preview = CurrentImpulseBuffer::default();
        preview.ingest(batch);
        preview.ingest(delta(3, &[]));
        let history = preview.history().unwrap();
        history.validate().unwrap();
        assert!(
            history.traces.is_empty(),
            "a later declaration cannot undo revoked coverage"
        );
        assert!(!history.delivery_complete);
    }
}
