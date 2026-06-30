//! Event Queue for XSPICE Event-Driven Simulation
//!
//! Provides the event-driven scheduling infrastructure for digital and real
//! code models.
//! Events are scheduled with delays and processed at breakpoints during transient analysis.

use super::digital::DigitalValue;
use crate::Value;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

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

    fn same_output_driver(&self, other: &Self) -> bool {
        self.node_id == other.node_id
            && self.instance == other.instance
            && self.port_name == other.port_name
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
        self.events
            .retain(|pending| !(pending.same_output_driver(&event) && pending.time >= event.time));
        self.events.push(event);
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
        self.events.is_empty()
    }

    /// Get the number of pending events
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Get the time of the next pending event
    pub fn next_event_time(&self) -> Option<Value> {
        self.events.peek().map(|e| e.time)
    }

    /// Check if there's an event at or before the given time
    pub fn has_event_at_or_before(&self, time: Value) -> bool {
        self.events.peek().is_some_and(|e| e.time <= time)
    }

    /// Pop all events at or before the given time
    pub fn pop_events_at(&mut self, time: Value) -> Vec<Event> {
        let mut events = Vec::new();
        while let Some(event) = self.events.peek() {
            if event.time <= time {
                if let Some(e) = self.events.pop() {
                    events.push(e);
                    self.events_processed += 1;
                    self.last_event_time = time;
                }
            } else {
                break;
            }
        }
        events
    }

    /// Pop events at exactly the given time
    pub fn pop_events_exactly_at(&mut self, time: Value) -> Vec<Event> {
        const EPSILON: Value = 1e-18;
        let mut events = Vec::new();
        while let Some(event) = self.events.peek() {
            if (event.time - time).abs() < EPSILON {
                if let Some(e) = self.events.pop() {
                    events.push(e);
                    self.events_processed += 1;
                    self.last_event_time = time;
                }
            } else {
                break;
            }
        }
        events
    }

    /// Cancel all events for a specific node
    pub fn cancel_node_events(&mut self, node_id: usize) {
        self.events.retain(|event| event.node_id != node_id);
    }

    /// Cancel all events for a specific instance
    pub fn cancel_instance_events(&mut self, instance: &str) {
        self.events.retain(|event| event.instance != instance);
    }

    /// Clear all pending events
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Get statistics
    pub fn stats(&self) -> EventQueueStats {
        EventQueueStats {
            pending: self.events.len(),
            processed: self.events_processed,
            last_event_time: self.last_event_time,
        }
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
    /// Sorted list of breakpoint times
    breakpoints: Vec<Value>,
    /// Tolerance for breakpoint matching
    tolerance: Value,
}

impl BreakpointManager {
    /// Create a new breakpoint manager
    pub fn new() -> Self {
        Self {
            breakpoints: Vec::new(),
            tolerance: 1e-18,
        }
    }

    /// Add a breakpoint at the given time
    pub fn add(&mut self, time: Value) {
        if !time.is_finite() {
            return;
        }

        // Check if already exists (within tolerance)
        for &bp in &self.breakpoints {
            if (bp - time).abs() < self.tolerance {
                return;
            }
        }

        // Insert in sorted order
        match self.breakpoints.binary_search_by(|t| t.total_cmp(&time)) {
            Ok(_) => {} // Already exists
            Err(pos) => self.breakpoints.insert(pos, time),
        }
    }

    /// Get the next breakpoint after the given time
    pub fn next_after(&self, time: Value) -> Option<Value> {
        self.breakpoints
            .iter()
            .find(|&&bp| bp > time + self.tolerance)
            .copied()
    }

    /// Remove breakpoints at or before the given time
    pub fn remove_before(&mut self, time: Value) {
        self.breakpoints.retain(|&t| t > time + self.tolerance);
    }

    /// Check if there's a breakpoint at the given time
    pub fn has_breakpoint_at(&self, time: Value) -> bool {
        self.breakpoints
            .iter()
            .any(|&t| (t - time).abs() < self.tolerance)
    }

    /// Clear all breakpoints
    pub fn clear(&mut self) {
        self.breakpoints.clear();
    }

    /// Get all breakpoints in an interval
    pub fn in_interval(&self, start: Value, end: Value) -> Vec<Value> {
        self.breakpoints
            .iter()
            .copied()
            .filter(|&t| t > start && t <= end)
            .collect()
    }
}

//=============================================================================
// Tests
//=============================================================================
