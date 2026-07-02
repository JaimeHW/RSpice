//! Event Queue for XSPICE Event-Driven Simulation
//!
//! Provides the event-driven scheduling infrastructure for digital and real
//! code models.
//! Events are scheduled with delays and processed at breakpoints during transient analysis.

use super::digital::DigitalValue;
use crate::Value;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet};
use std::ops::Bound::{Excluded, Included, Unbounded};

//=============================================================================
// Event Types
//=============================================================================

/// Payload carried by a scheduled XSPICE event.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventValue {
    /// 12-state digital event value.
    Digital(DigitalValue),
    /// Real-valued event-node value.
    Real(Value),
}

/// A scheduled event-driven XSPICE update.
#[derive(Debug, Clone)]
pub struct Event {
    /// Time at which the event occurs
    pub time: Value,
    /// Target node identifier (internal representation)
    pub node_id: usize,
    /// Port name (for lookup)
    pub port_name: String,
    /// Instance name that scheduled the event
    pub instance: String,
    /// New event value
    pub value: EventValue,
    /// Event priority (for tie-breaking)
    pub priority: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EventDriverKey {
    node_id: usize,
    port_name: String,
    instance: String,
}

impl Event {
    /// Create a new event
    pub fn new(
        time: Value,
        node_id: usize,
        port_name: impl Into<String>,
        instance: impl Into<String>,
        value: EventValue,
    ) -> Self {
        static PRIORITY_COUNTER: std::sync::atomic::AtomicU64 =
            std::sync::atomic::AtomicU64::new(0);
        Self {
            time,
            node_id,
            port_name: port_name.into(),
            instance: instance.into(),
            value,
            priority: PRIORITY_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }
    }

    fn driver_key(&self) -> EventDriverKey {
        EventDriverKey {
            node_id: self.node_id,
            port_name: self.port_name.clone(),
            instance: self.instance.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct EventTimeKey(Value);

impl PartialEq for EventTimeKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0) == Ordering::Equal
    }
}

impl Eq for EventTimeKey {}

impl PartialOrd for EventTimeKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EventTimeKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

// Events are ordered by time (earliest first), then by priority
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.priority == other.priority
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        // BinaryHeap is a max-heap, so we reverse the ordering for earliest-first
        other
            .time
            .partial_cmp(&self.time)
            .unwrap_or(Ordering::Equal)
            .then_with(|| other.priority.cmp(&self.priority))
    }
}

//=============================================================================
// Event Queue
//=============================================================================

/// Priority queue for XSPICE event-node updates.
///
/// Events are scheduled with a future time and processed in chronological order.
/// The queue automatically handles cancellation and replacement of events.
#[derive(Debug, Clone, Default)]
pub struct EventQueue {
    /// Pending events (min-heap by time)
    events: BinaryHeap<Event>,
    /// Active pending event priorities for lazy cancellation.
    active_event_priorities: HashSet<u64>,
    /// Active pending event counts by time.
    active_times: BTreeMap<EventTimeKey, usize>,
    /// Active pending event priorities by output driver and time.
    driver_event_priorities: HashMap<EventDriverKey, BTreeMap<EventTimeKey, Vec<u64>>>,
    /// Number of events processed
    events_processed: u64,
    /// Last event time processed
    last_event_time: Value,
}

impl EventQueue {
    /// Create a new empty event queue
    pub fn new() -> Self {
        Self::default()
    }

    /// Schedule an event
    pub fn schedule(&mut self, event: Event) {
        if !event.time.is_finite() {
            return;
        }
        let driver_key = event.driver_key();
        let event_time = EventTimeKey(event.time);
        if let Some(cancelled) = self.cancel_driver_events_at_or_after(&driver_key, event_time) {
            for (time, priorities) in cancelled {
                for priority in priorities {
                    if self.active_event_priorities.remove(&priority) {
                        self.decrement_active_time(time);
                    }
                }
            }
        }
        self.insert_active_event(&driver_key, event_time, event.priority);
        self.events.push(event);
        self.compact_stale_events_if_needed();
    }

    /// Schedule an event with delay from current time.
    pub fn schedule_delayed(
        &mut self,
        current_time: Value,
        delay: Value,
        node_id: usize,
        port_name: impl Into<String>,
        instance: impl Into<String>,
        value: DigitalValue,
    ) {
        if delay < 0.0 {
            return;
        }
        let event = Event::new(
            current_time + delay,
            node_id,
            port_name,
            instance,
            EventValue::Digital(value),
        );
        self.schedule(event);
    }

    /// Schedule a real-valued event with delay from current time.
    pub fn schedule_real_delayed(
        &mut self,
        current_time: Value,
        delay: Value,
        node_id: usize,
        port_name: impl Into<String>,
        instance: impl Into<String>,
        value: Value,
    ) {
        if delay < 0.0 {
            return;
        }
        let event = Event::new(
            current_time + delay,
            node_id,
            port_name,
            instance,
            EventValue::Real(value),
        );
        self.schedule(event);
    }

    /// Check if there are any pending events
    pub fn is_empty(&self) -> bool {
        self.active_event_priorities.is_empty()
    }

    /// Get the number of pending events
    pub fn len(&self) -> usize {
        self.active_event_priorities.len()
    }

    /// Get the time of the next pending event
    pub fn next_event_time(&self) -> Option<Value> {
        self.active_times.first_key_value().map(|(time, _)| time.0)
    }

    /// Check if there's an event at or before the given time
    pub fn has_event_at_or_before(&self, time: Value) -> bool {
        self.active_times
            .first_key_value()
            .is_some_and(|(event_time, _)| event_time.0 <= time)
    }

    /// Pop all events at or before the given time
    pub fn pop_events_at(&mut self, time: Value) -> Vec<Event> {
        let mut events = Vec::new();
        self.drain_events_at(time, |event| events.push(event));
        events
    }

    /// Drain all events at or before the given time into a caller-provided sink.
    pub fn drain_events_at<F>(&mut self, time: Value, mut sink: F)
    where
        F: FnMut(Event),
    {
        while let Some(event) = self.events.peek() {
            if event.time <= time {
                if let Some(e) = self.events.pop() {
                    if self.remove_active_event(&e) {
                        self.events_processed += 1;
                        self.last_event_time = e.time;
                        sink(e);
                    }
                }
            } else {
                break;
            }
        }
    }

    /// Pop events at exactly the given time
    pub fn pop_events_exactly_at(&mut self, time: Value) -> Vec<Event> {
        const EPSILON: Value = 1e-18;
        let mut events = Vec::new();
        let mut retained = BinaryHeap::with_capacity(self.events.len());
        while let Some(event) = self.events.pop() {
            if !self.active_event_priorities.contains(&event.priority) {
                continue;
            }
            if (event.time - time).abs() < EPSILON {
                if self.remove_active_event(&event) {
                    self.events_processed += 1;
                    self.last_event_time = event.time;
                    events.push(event);
                }
            } else {
                retained.push(event);
            }
        }
        self.events = retained;
        events
    }

    /// Cancel all events for a specific node
    pub fn cancel_node_events(&mut self, node_id: usize) {
        let active_event_priorities = &self.active_event_priorities;
        let mut removed_active_event = false;
        self.events.retain(|event| {
            let is_active = active_event_priorities.contains(&event.priority);
            if is_active && event.node_id == node_id {
                removed_active_event = true;
                false
            } else {
                is_active
            }
        });
        if removed_active_event {
            self.rebuild_indexes();
        }
    }

    /// Cancel all events for a specific instance
    pub fn cancel_instance_events(&mut self, instance: &str) {
        let active_event_priorities = &self.active_event_priorities;
        let mut removed_active_event = false;
        self.events.retain(|event| {
            let is_active = active_event_priorities.contains(&event.priority);
            if is_active && event.instance == instance {
                removed_active_event = true;
                false
            } else {
                is_active
            }
        });
        if removed_active_event {
            self.rebuild_indexes();
        }
    }

    /// Clear all pending events
    pub fn clear(&mut self) {
        self.events.clear();
        self.active_event_priorities.clear();
        self.active_times.clear();
        self.driver_event_priorities.clear();
    }

    /// Get statistics
    pub fn stats(&self) -> EventQueueStats {
        EventQueueStats {
            pending: self.active_event_priorities.len(),
            processed: self.events_processed,
            last_event_time: self.last_event_time,
        }
    }

    fn cancel_driver_events_at_or_after(
        &mut self,
        driver_key: &EventDriverKey,
        time: EventTimeKey,
    ) -> Option<BTreeMap<EventTimeKey, Vec<u64>>> {
        let driver_events = self.driver_event_priorities.get_mut(driver_key)?;
        let has_later_event = driver_events
            .last_key_value()
            .is_some_and(|(latest_time, _)| *latest_time >= time);
        has_later_event.then(|| driver_events.split_off(&time))
    }

    fn insert_active_event(
        &mut self,
        driver_key: &EventDriverKey,
        time: EventTimeKey,
        priority: u64,
    ) {
        self.active_event_priorities.insert(priority);
        *self.active_times.entry(time).or_insert(0) += 1;
        self.driver_event_priorities
            .entry(driver_key.clone())
            .or_default()
            .entry(time)
            .or_default()
            .push(priority);
    }

    fn remove_active_event(&mut self, event: &Event) -> bool {
        if !self.active_event_priorities.remove(&event.priority) {
            return false;
        }
        let time = EventTimeKey(event.time);
        self.decrement_active_time(time);
        let driver_key = event.driver_key();
        let mut remove_driver = false;
        if let Some(driver_events) = self.driver_event_priorities.get_mut(&driver_key) {
            let mut remove_time = false;
            if let Some(priorities) = driver_events.get_mut(&time) {
                priorities.retain(|priority| *priority != event.priority);
                remove_time = priorities.is_empty();
            }
            if remove_time {
                driver_events.remove(&time);
            }
            remove_driver = driver_events.is_empty();
        }
        if remove_driver {
            self.driver_event_priorities.remove(&driver_key);
        }
        true
    }

    fn decrement_active_time(&mut self, time: EventTimeKey) {
        let mut remove_time = false;
        if let Some(count) = self.active_times.get_mut(&time) {
            *count = count.saturating_sub(1);
            remove_time = *count == 0;
        }
        if remove_time {
            self.active_times.remove(&time);
        }
    }

    fn compact_stale_events_if_needed(&mut self) {
        let active_events = self.active_event_priorities.len();
        if self.events.len() <= active_events.saturating_mul(2).saturating_add(1024) {
            return;
        }
        let active_event_priorities = &self.active_event_priorities;
        self.events
            .retain(|event| active_event_priorities.contains(&event.priority));
    }

    fn rebuild_indexes(&mut self) {
        let mut active_event_priorities = HashSet::with_capacity(self.events.len());
        let mut active_times = BTreeMap::new();
        let mut driver_event_priorities: HashMap<EventDriverKey, BTreeMap<EventTimeKey, Vec<u64>>> =
            HashMap::with_capacity(self.events.len());
        for event in self.events.iter() {
            let driver_key = event.driver_key();
            let time = EventTimeKey(event.time);
            active_event_priorities.insert(event.priority);
            *active_times.entry(time).or_insert(0) += 1;
            driver_event_priorities
                .entry(driver_key)
                .or_default()
                .entry(time)
                .or_default()
                .push(event.priority);
        }
        self.active_event_priorities = active_event_priorities;
        self.active_times = active_times;
        self.driver_event_priorities = driver_event_priorities;
    }
}

/// Event queue statistics
#[derive(Debug, Clone)]
pub struct EventQueueStats {
    /// Number of pending events
    pub pending: usize,
    /// Total events processed
    pub processed: u64,
    /// Last event time
    pub last_event_time: Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_queue_ignores_negative_output_delays_like_ngspice() {
        let mut queue = EventQueue::new();

        queue.schedule_delayed(
            1.0e-9,
            -1.0e-12,
            1,
            "out",
            "a_negative_digital_delay",
            DigitalValue::one(),
        );
        queue.schedule_real_delayed(1.0e-9, -1.0e-12, 2, "out", "a_negative_real_delay", 1.0);

        assert!(
            queue.is_empty(),
            "ngspice reports 'Output delay < 0 not allowed' and ignores the output update"
        );
    }

    #[test]
    fn event_queue_replaces_later_events_from_same_output_driver() {
        let mut queue = EventQueue::new();

        queue.schedule(Event::new(
            0.5e-9,
            1,
            "out",
            "a_driver",
            EventValue::Digital(DigitalValue::zero()),
        ));
        queue.schedule(Event::new(
            2.0e-9,
            1,
            "out",
            "a_driver",
            EventValue::Digital(DigitalValue::one()),
        ));
        queue.schedule(Event::new(
            2.0e-9,
            1,
            "other",
            "a_driver",
            EventValue::Digital(DigitalValue::unknown()),
        ));
        queue.schedule(Event::new(
            1.0e-9,
            1,
            "out",
            "a_driver",
            EventValue::Digital(DigitalValue::unknown()),
        ));

        assert_eq!(queue.len(), 3);
        let events = queue.pop_events_at(2.0e-9);
        let values: Vec<_> = events.iter().map(|event| event.value).collect();
        assert!(values.contains(&EventValue::Digital(DigitalValue::zero())));
        assert!(values.contains(&EventValue::Digital(DigitalValue::unknown())));
        assert!(!values.contains(&EventValue::Digital(DigitalValue::one())));
    }

    #[test]
    fn event_queue_exact_pop_skips_cancelled_earlier_heap_entries() {
        let mut queue = EventQueue::new();

        queue.schedule(Event::new(
            1.0e-9,
            1,
            "out",
            "a_driver",
            EventValue::Digital(DigitalValue::zero()),
        ));
        queue.schedule(Event::new(
            3.0e-9,
            1,
            "out",
            "a_driver",
            EventValue::Digital(DigitalValue::one()),
        ));
        queue.schedule(Event::new(
            0.5e-9,
            1,
            "out",
            "a_driver",
            EventValue::Digital(DigitalValue::unknown()),
        ));

        assert_eq!(queue.pop_events_exactly_at(0.5e-9).len(), 1);

        queue.schedule(Event::new(
            2.0e-9,
            2,
            "out",
            "another_driver",
            EventValue::Digital(DigitalValue::one()),
        ));

        let events = queue.pop_events_exactly_at(2.0e-9);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].node_id, 2);
        assert!(queue.is_empty());
    }

    #[test]
    fn event_queue_exact_pop_is_not_blocked_by_earlier_active_events() {
        let mut queue = EventQueue::new();

        queue.schedule(Event::new(
            1.0e-9,
            1,
            "out",
            "early_driver",
            EventValue::Digital(DigitalValue::zero()),
        ));
        queue.schedule(Event::new(
            2.0e-9,
            2,
            "out",
            "target_driver",
            EventValue::Digital(DigitalValue::one()),
        ));
        queue.schedule(Event::new(
            3.0e-9,
            3,
            "out",
            "late_driver",
            EventValue::Digital(DigitalValue::unknown()),
        ));

        let events = queue.pop_events_exactly_at(2.0e-9);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].node_id, 2);
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.next_event_time(), Some(1.0e-9));

        let remaining = queue.pop_events_at(3.0e-9);
        let remaining_nodes: Vec<_> = remaining.iter().map(|event| event.node_id).collect();
        assert_eq!(remaining_nodes, vec![1, 3]);
    }

    #[test]
    fn event_queue_stats_record_actual_processed_event_time() {
        let mut queue = EventQueue::new();

        queue.schedule(Event::new(
            1.0e-9,
            1,
            "out",
            "driver",
            EventValue::Digital(DigitalValue::one()),
        ));

        let events = queue.pop_events_at(2.0e-9);
        assert_eq!(events.len(), 1);
        let stats = queue.stats();
        assert_eq!(stats.processed, 1);
        assert_eq!(stats.last_event_time, 1.0e-9);
    }

    #[test]
    fn event_queue_active_indexes_track_schedule_drain_cancel_and_clear() {
        let mut queue = EventQueue::new();
        let driver = EventDriverKey {
            node_id: 1,
            port_name: "out".to_string(),
            instance: "a_driver".to_string(),
        };

        queue.schedule(Event::new(
            1.0e-9,
            1,
            "out",
            "a_driver",
            EventValue::Digital(DigitalValue::zero()),
        ));
        queue.schedule(Event::new(
            3.0e-9,
            1,
            "out",
            "a_driver",
            EventValue::Digital(DigitalValue::one()),
        ));
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.next_event_time(), Some(1.0e-9));
        assert!(
            queue
                .driver_event_priorities
                .get(&driver)
                .is_some_and(|events| events.contains_key(&EventTimeKey(3.0e-9)))
        );

        queue.schedule(Event::new(
            2.0e-9,
            1,
            "out",
            "a_driver",
            EventValue::Digital(DigitalValue::unknown()),
        ));
        assert_eq!(queue.len(), 2);
        assert!(
            queue
                .driver_event_priorities
                .get(&driver)
                .is_some_and(|events| !events.contains_key(&EventTimeKey(3.0e-9))
                    && events.contains_key(&EventTimeKey(2.0e-9)))
        );

        let drained = queue.pop_events_at(1.0e-9);
        assert_eq!(drained.len(), 1);
        assert_eq!(queue.len(), 1);
        assert!(
            queue
                .driver_event_priorities
                .get(&driver)
                .is_some_and(|events| events.contains_key(&EventTimeKey(2.0e-9)))
        );

        queue.cancel_node_events(1);
        assert!(queue.active_event_priorities.is_empty());
        assert!(queue.active_times.is_empty());
        assert!(queue.driver_event_priorities.is_empty());

        queue.schedule(Event::new(
            4.0e-9,
            1,
            "out",
            "a_driver",
            EventValue::Digital(DigitalValue::one()),
        ));
        queue.clear();
        assert!(queue.active_event_priorities.is_empty());
        assert!(queue.active_times.is_empty());
        assert!(queue.driver_event_priorities.is_empty());
    }

    #[test]
    fn event_queue_cancel_filters_heap_in_place() {
        let mut queue = EventQueue::new();

        queue.schedule(Event::new(
            1.0e-9,
            1,
            "out",
            "a_left",
            EventValue::Digital(DigitalValue::one()),
        ));
        queue.schedule(Event::new(
            2.0e-9,
            2,
            "out",
            "a_left",
            EventValue::Digital(DigitalValue::zero()),
        ));
        queue.schedule(Event::new(
            3.0e-9,
            3,
            "out",
            "a_right",
            EventValue::Digital(DigitalValue::unknown()),
        ));

        queue.cancel_node_events(2);
        assert_eq!(queue.len(), 2);
        queue.cancel_instance_events("a_right");
        assert_eq!(queue.len(), 1);

        let events = queue.pop_events_at(3.0e-9);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].node_id, 1);
        assert_eq!(events[0].instance, "a_left");
    }

    #[test]
    fn event_queue_drains_due_events_into_caller_sink() {
        let mut queue = EventQueue::new();

        queue.schedule(Event::new(
            0.5e-9,
            1,
            "out",
            "a_driver",
            EventValue::Digital(DigitalValue::zero()),
        ));
        queue.schedule(Event::new(
            2.0e-9,
            2,
            "out",
            "a_driver",
            EventValue::Real(1.25),
        ));

        let mut drained = Vec::new();
        queue.drain_events_at(1.0e-9, |event| drained.push(event.value));

        assert_eq!(drained, vec![EventValue::Digital(DigitalValue::zero())]);
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.stats().processed, 1);
        assert_eq!(queue.next_event_time(), Some(2.0e-9));
    }
}

//=============================================================================
// Breakpoint Manager
//=============================================================================

/// Manages simulation breakpoints for event-driven coordination
///
/// Breakpoints are times at which the transient simulation must stop
/// to process digital events or other discrete changes.
#[derive(Debug, Default)]
pub struct BreakpointManager {
    /// Ordered set of breakpoint times.
    breakpoints: BTreeSet<EventTimeKey>,
    /// Tolerance for breakpoint matching
    tolerance: Value,
}

impl BreakpointManager {
    /// Create a new breakpoint manager
    pub fn new() -> Self {
        Self {
            breakpoints: BTreeSet::new(),
            tolerance: 1e-18,
        }
    }

    /// Add a breakpoint at the given time
    pub fn add(&mut self, time: Value) {
        if !time.is_finite() {
            return;
        }

        if self.has_breakpoint_within_tolerance(time) {
            return;
        }

        self.breakpoints.insert(EventTimeKey(time));
    }

    /// Get the next breakpoint after the given time
    pub fn next_after(&self, time: Value) -> Option<Value> {
        if time.is_nan() {
            return None;
        }
        let cutoff = time + self.tolerance;
        self.breakpoints
            .range((Excluded(EventTimeKey(cutoff)), Unbounded))
            .next()
            .map(|time| time.0)
    }

    /// Remove breakpoints at or before the given time
    pub fn remove_before(&mut self, time: Value) {
        if time.is_nan() {
            return;
        }
        let cutoff = time + self.tolerance;
        let mut retained = self.breakpoints.split_off(&EventTimeKey(cutoff));
        while retained
            .first()
            .is_some_and(|breakpoint| breakpoint.0 <= cutoff)
        {
            retained.pop_first();
        }
        self.breakpoints = retained;
    }

    /// Check if there's a breakpoint at the given time
    pub fn has_breakpoint_at(&self, time: Value) -> bool {
        if !time.is_finite() {
            return false;
        }
        self.has_breakpoint_within_tolerance(time)
    }

    /// Clear all breakpoints
    pub fn clear(&mut self) {
        self.breakpoints.clear();
    }

    /// Get all breakpoints in an interval
    pub fn in_interval(&self, start: Value, end: Value) -> Vec<Value> {
        if start.is_nan() || end.is_nan() || end <= start {
            return Vec::new();
        }
        self.breakpoints
            .range((Excluded(EventTimeKey(start)), Included(EventTimeKey(end))))
            .map(|time| time.0)
            .collect()
    }

    fn has_breakpoint_within_tolerance(&self, time: Value) -> bool {
        let lower = time - self.tolerance;
        self.breakpoints
            .range((Excluded(EventTimeKey(lower)), Unbounded))
            .next()
            .is_some_and(|breakpoint| (breakpoint.0 - time).abs() < self.tolerance)
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod breakpoint_tests {
    use super::*;

    #[test]
    fn breakpoint_manager_keeps_ordered_unique_times_with_tolerance() {
        let mut breakpoints = BreakpointManager::new();

        breakpoints.add(3.0e-9);
        breakpoints.add(1.0e-9);
        breakpoints.add(2.0e-9);
        breakpoints.add(2.0e-9 + 0.5e-18);

        assert_eq!(breakpoints.next_after(0.0), Some(1.0e-9));
        assert_eq!(
            breakpoints.in_interval(0.0, 3.0e-9),
            vec![1.0e-9, 2.0e-9, 3.0e-9]
        );
        assert!(breakpoints.has_breakpoint_at(2.0e-9 + 0.5e-18));
    }

    #[test]
    fn breakpoint_manager_removes_due_times_without_vec_shifts() {
        let mut breakpoints = BreakpointManager::new();

        for time in [4.0e-9, 1.0e-9, 3.0e-9, 2.0e-9] {
            breakpoints.add(time);
        }

        breakpoints.remove_before(2.0e-9);

        assert_eq!(breakpoints.next_after(0.0), Some(3.0e-9));
        assert_eq!(breakpoints.in_interval(0.0, 5.0e-9), vec![3.0e-9, 4.0e-9]);
    }
}
