//! Event Bus
//!
//! Publish-subscribe event system for decoupled component communication.
//! Enables cross-probing, selection sync, and coordination between views.
//!
//! # Features
//!
//! - Type-safe event registration and dispatch
//! - Priority-based handler ordering
//! - Async event support
//! - Event filtering and grouping

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::RwLock;

use super::lock::{read_lock, write_lock};

// =============================================================================
// Event Types
// =============================================================================

/// Event category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    /// Selection changed in a view
    SelectionChanged,
    /// Component hover
    Hover,
    /// Signal highlight request
    SignalHighlight,
    /// Cross-probe request
    CrossProbe,
    /// Zoom/pan changed
    ViewChanged,
    /// Simulation started
    SimulationStarted,
    /// Simulation progress
    SimulationProgress,
    /// Simulation completed
    SimulationCompleted,
    /// Waveform data updated
    WaveformUpdated,
    /// Schematic modified
    SchematicModified,
    /// File opened
    FileOpened,
    /// File saved
    FileSaved,
    /// Settings changed
    SettingsChanged,
    /// Error occurred
    Error,
    /// Custom event
    Custom,
}

/// Event source identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventSource {
    /// Component ID
    pub component_id: String,
    /// View type
    pub view_type: ViewType,
}

/// View types that can emit/receive events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ViewType {
    /// Schematic editor
    Schematic,
    /// Waveform viewer
    Waveform,
    /// Analysis results
    Analysis,
    /// Property panel
    Properties,
    /// Simulation controller
    Simulation,
    /// Console/log
    Console,
    /// Project browser
    Browser,
    /// System
    System,
}

// =============================================================================
// Event Data
// =============================================================================

/// A typed event with payload
#[derive(Debug, Clone)]
pub struct Event {
    /// Event type
    pub event_type: EventType,
    /// Source of the event
    pub source: EventSource,
    /// Timestamp (ms since epoch)
    pub timestamp: u64,
    /// Event data
    pub data: EventData,
    /// Whether event is consumed
    consumed: bool,
}

/// Event payload data
#[derive(Debug, Clone, Default)]
pub struct EventData {
    /// Selected component names
    pub components: Vec<String>,
    /// Selected signal names
    pub signals: Vec<String>,
    /// Selected nodes
    pub nodes: Vec<String>,
    /// Selected time range (start, end)
    pub time_range: Option<(f64, f64)>,
    /// Selected frequency range
    pub freq_range: Option<(f64, f64)>,
    /// Message string
    pub message: Option<String>,
    /// Progress value (0-100)
    pub progress: Option<u8>,
    /// Custom data
    pub custom: HashMap<String, String>,
}

impl Event {
    /// Create a new event
    pub fn new(event_type: EventType, source: EventSource) -> Self {
        Self {
            event_type,
            source,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0),
            data: EventData::default(),
            consumed: false,
        }
    }

    /// Create with components
    pub fn with_components(mut self, components: Vec<String>) -> Self {
        self.data.components = components;
        self
    }

    /// Create with signals
    pub fn with_signals(mut self, signals: Vec<String>) -> Self {
        self.data.signals = signals;
        self
    }

    /// Create with message
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.data.message = Some(message.into());
        self
    }

    /// Create with progress
    pub fn with_progress(mut self, progress: u8) -> Self {
        self.data.progress = Some(progress.min(100));
        self
    }

    /// Mark event as consumed
    pub fn consume(&mut self) {
        self.consumed = true;
    }

    /// Check if consumed
    pub fn is_consumed(&self) -> bool {
        self.consumed
    }
}

impl EventSource {
    /// Create a new source
    pub fn new(component_id: impl Into<String>, view_type: ViewType) -> Self {
        Self {
            component_id: component_id.into(),
            view_type,
        }
    }

    /// System source
    pub fn system() -> Self {
        Self::new("system", ViewType::System)
    }
}

// =============================================================================
// Event Handler
// =============================================================================

/// Handler function type
pub type HandlerFn = Box<dyn Fn(&mut Event) + Send + Sync>;

/// Event handler registration
pub struct EventHandler {
    /// Handler ID
    pub id: u64,
    /// Priority (higher = first)
    pub priority: i32,
    /// Event types to handle
    pub event_types: Vec<EventType>,
    /// Handler function
    pub handler: HandlerFn,
    /// Whether handler is active
    pub active: bool,
}

impl EventHandler {
    /// Create a new handler
    pub fn new(id: u64, handler: HandlerFn) -> Self {
        Self {
            id,
            priority: 0,
            event_types: Vec::new(),
            handler,
            active: true,
        }
    }

    /// Set priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Add event type filter
    pub fn for_event(mut self, event_type: EventType) -> Self {
        self.event_types.push(event_type);
        self
    }

    /// Add multiple event types
    pub fn for_events(mut self, types: &[EventType]) -> Self {
        self.event_types.extend(types);
        self
    }

    /// Check if handles this event type
    pub fn handles(&self, event_type: EventType) -> bool {
        self.event_types.is_empty() || self.event_types.contains(&event_type)
    }

    /// Invoke handler
    pub fn invoke(&self, event: &mut Event) {
        if self.active && self.handles(event.event_type) {
            (self.handler)(event);
        }
    }
}

// =============================================================================
// Event Bus
// =============================================================================

/// Central event bus for pub/sub
pub struct EventBus {
    /// Registered handlers
    handlers: RwLock<Vec<EventHandler>>,
    /// Next handler ID
    next_id: RwLock<u64>,
    /// Event history (for debugging)
    history: RwLock<VecDeque<Event>>,
    /// Max history size
    max_history: usize,
    /// Whether to record history
    record_history: bool,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(Vec::new()),
            next_id: RwLock::new(1),
            history: RwLock::new(VecDeque::new()),
            max_history: 100,
            record_history: false,
        }
    }

    /// Enable history recording
    pub fn with_history(mut self, enabled: bool) -> Self {
        self.record_history = enabled;
        self
    }

    fn insert_handler_sorted(handlers: &mut Vec<EventHandler>, registration: EventHandler) {
        let idx = handlers.partition_point(|handler| handler.priority >= registration.priority);
        handlers.insert(idx, registration);
    }

    /// Register a handler with explicit priority (higher priorities run first).
    pub fn subscribe_with_priority(&self, priority: i32, handler: HandlerFn) -> u64 {
        let mut id_lock = write_lock(&self.next_id, "EventBus::subscribe_with_priority(next_id)");
        let id = *id_lock;
        *id_lock += 1;

        let registration = EventHandler::new(id, handler).with_priority(priority);
        let mut handlers = write_lock(
            &self.handlers,
            "EventBus::subscribe_with_priority(handlers)",
        );
        Self::insert_handler_sorted(&mut handlers, registration);
        id
    }

    /// Register handler for specific event types and explicit priority.
    pub fn subscribe_to_with_priority(
        &self,
        types: &[EventType],
        priority: i32,
        handler: HandlerFn,
    ) -> u64 {
        let mut id_lock = write_lock(
            &self.next_id,
            "EventBus::subscribe_to_with_priority(next_id)",
        );
        let id = *id_lock;
        *id_lock += 1;

        let registration = EventHandler::new(id, handler)
            .with_priority(priority)
            .for_events(types);
        let mut handlers = write_lock(
            &self.handlers,
            "EventBus::subscribe_to_with_priority(handlers)",
        );
        Self::insert_handler_sorted(&mut handlers, registration);
        id
    }

    /// Register a handler
    pub fn subscribe(&self, handler: HandlerFn) -> u64 {
        self.subscribe_with_priority(0, handler)
    }

    /// Register handler for specific event types
    pub fn subscribe_to(&self, types: &[EventType], handler: HandlerFn) -> u64 {
        self.subscribe_to_with_priority(types, 0, handler)
    }

    /// Unsubscribe handler
    pub fn unsubscribe(&self, handler_id: u64) {
        let mut handlers = write_lock(&self.handlers, "EventBus::unsubscribe");
        handlers.retain(|h| h.id != handler_id);
    }

    /// Publish an event
    pub fn publish(&self, event: Event) {
        let mut event = event;

        // Record history
        if self.record_history {
            let mut history = write_lock(&self.history, "EventBus::publish(history)");
            history.push_back(event.clone());
            if history.len() > self.max_history {
                history.pop_front();
            }
        }

        // Dispatch to handlers
        let handlers = read_lock(&self.handlers, "EventBus::publish(handlers)");
        for handler in handlers.iter() {
            if !event.is_consumed() {
                handler.invoke(&mut event);
            }
        }
    }

    /// Publish selection change
    pub fn publish_selection(&self, source: EventSource, components: Vec<String>) {
        let event = Event::new(EventType::SelectionChanged, source).with_components(components);
        self.publish(event);
    }

    /// Publish signal highlight
    pub fn publish_signal_highlight(&self, source: EventSource, signals: Vec<String>) {
        let event = Event::new(EventType::SignalHighlight, source).with_signals(signals);
        self.publish(event);
    }

    /// Publish simulation progress
    pub fn publish_progress(&self, progress: u8) {
        let event = Event::new(EventType::SimulationProgress, EventSource::system())
            .with_progress(progress);
        self.publish(event);
    }

    /// Clear history
    pub fn clear_history(&self) {
        let mut history = write_lock(&self.history, "EventBus::clear_history");
        history.clear();
    }

    /// Get history
    pub fn history(&self) -> Vec<Event> {
        let history = read_lock(&self.history, "EventBus::history");
        history.iter().cloned().collect()
    }

    /// Get handler count
    pub fn handler_count(&self) -> usize {
        let handlers = read_lock(&self.handlers, "EventBus::handler_count");
        handlers.len()
    }
}

// =============================================================================
// Tests
// =============================================================================
