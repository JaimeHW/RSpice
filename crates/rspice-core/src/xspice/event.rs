//! Event Queue for Digital Simulation
//!
//! Provides the event-driven scheduling infrastructure for digital code models.
//! Events are scheduled with delays and processed at breakpoints during transient analysis.

use super::digital::DigitalValue;
use crate::Value;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

//=============================================================================
// Event Types
//=============================================================================

/// A scheduled digital event
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
    /// New digital value
    pub value: DigitalValue,
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
        value: DigitalValue,
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

/// Priority queue for digital events
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
        self.events.push(event);
    }

    /// Schedule an event with delay from current time
    pub fn schedule_delayed(
        &mut self,
        current_time: Value,
        delay: Value,
        node_id: usize,
        port_name: impl Into<String>,
        instance: impl Into<String>,
        value: DigitalValue,
    ) {
        let event = Event::new(current_time + delay, node_id, port_name, instance, value);
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
        let mut remaining: Vec<Event> = self.events.drain().collect();
        remaining.retain(|e| e.node_id != node_id);
        self.events = remaining.into_iter().collect();
    }

    /// Cancel all events for a specific instance
    pub fn cancel_instance_events(&mut self, instance: &str) {
        let mut remaining: Vec<Event> = self.events.drain().collect();
        remaining.retain(|e| e.instance != instance);
        self.events = remaining.into_iter().collect();
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
