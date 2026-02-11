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
pub type HandlerFn = Box<dyn Fn(&Event) + Send + Sync>;

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
    pub fn invoke(&self, event: &Event) {
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
                handler.invoke(&event);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    // =========================================================================
    // Event Tests
    // =========================================================================

    #[test]
    fn test_event_creation() {
        let source = EventSource::new("schematic", ViewType::Schematic);
        let event = Event::new(EventType::SelectionChanged, source);

        assert_eq!(event.event_type, EventType::SelectionChanged);
        assert!(!event.is_consumed());
    }

    #[test]
    fn test_event_with_data() {
        let event = Event::new(EventType::SelectionChanged, EventSource::system())
            .with_components(vec!["R1".to_string(), "C1".to_string()])
            .with_message("Test");

        assert_eq!(event.data.components.len(), 2);
        assert_eq!(event.data.message, Some("Test".to_string()));
    }

    #[test]
    fn test_event_consume() {
        let mut event = Event::new(EventType::SelectionChanged, EventSource::system());
        assert!(!event.is_consumed());

        event.consume();
        assert!(event.is_consumed());
    }

    // =========================================================================
    // EventHandler Tests
    // =========================================================================

    #[test]
    fn test_handler_handles() {
        let handler = EventHandler::new(1, Box::new(|_| {}))
            .for_event(EventType::SelectionChanged)
            .for_event(EventType::Hover);

        assert!(handler.handles(EventType::SelectionChanged));
        assert!(handler.handles(EventType::Hover));
        assert!(!handler.handles(EventType::SimulationStarted));
    }

    #[test]
    fn test_handler_handles_all() {
        // Empty event_types = handles all
        let handler = EventHandler::new(1, Box::new(|_| {}));
        assert!(handler.handles(EventType::SelectionChanged));
        assert!(handler.handles(EventType::Error));
    }

    // =========================================================================
    // EventBus Tests
    // =========================================================================

    #[test]
    fn test_bus_subscribe() {
        let bus = EventBus::new();
        let id = bus.subscribe(Box::new(|_| {}));
        assert!(id > 0);
        assert_eq!(bus.handler_count(), 1);
    }

    #[test]
    fn test_bus_unsubscribe() {
        let bus = EventBus::new();
        let id = bus.subscribe(Box::new(|_| {}));
        assert_eq!(bus.handler_count(), 1);

        bus.unsubscribe(id);
        assert_eq!(bus.handler_count(), 0);
    }

    #[test]
    fn test_bus_publish() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        bus.subscribe(Box::new(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        }));

        let event = Event::new(EventType::SelectionChanged, EventSource::system());
        bus.publish(event);

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_bus_subscribe_to() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        bus.subscribe_to(
            &[EventType::SelectionChanged],
            Box::new(move |_| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }),
        );

        // This should trigger
        bus.publish(Event::new(
            EventType::SelectionChanged,
            EventSource::system(),
        ));
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        // This should NOT trigger
        bus.publish(Event::new(
            EventType::SimulationStarted,
            EventSource::system(),
        ));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_bus_history() {
        let bus = EventBus::new().with_history(true);

        bus.publish(Event::new(
            EventType::SelectionChanged,
            EventSource::system(),
        ));
        bus.publish(Event::new(EventType::Hover, EventSource::system()));

        let history = bus.history();
        assert_eq!(history.len(), 2);

        bus.clear_history();
        assert!(bus.history().is_empty());
    }

    #[test]
    fn test_bus_publish_helpers() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        bus.subscribe(Box::new(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        }));

        bus.publish_selection(EventSource::system(), vec!["R1".to_string()]);
        bus.publish_signal_highlight(EventSource::system(), vec!["v(out)".to_string()]);
        bus.publish_progress(50);

        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_bus_subscribe_with_priority_dispatches_high_to_low() {
        let bus = EventBus::new();
        let call_order = Arc::new(RwLock::new(Vec::<&'static str>::new()));
        let low_order = call_order.clone();
        let mid_order = call_order.clone();
        let high_order = call_order.clone();

        bus.subscribe_with_priority(
            -10,
            Box::new(move |_| {
                low_order
                    .write()
                    .expect("call-order lock should be writable")
                    .push("low");
            }),
        );
        bus.subscribe_with_priority(
            0,
            Box::new(move |_| {
                mid_order
                    .write()
                    .expect("call-order lock should be writable")
                    .push("mid");
            }),
        );
        bus.subscribe_with_priority(
            10,
            Box::new(move |_| {
                high_order
                    .write()
                    .expect("call-order lock should be writable")
                    .push("high");
            }),
        );

        bus.publish(Event::new(EventType::Custom, EventSource::system()));

        let call_order = call_order
            .read()
            .expect("call-order lock should be readable")
            .clone();
        assert_eq!(call_order, vec!["high", "mid", "low"]);
    }

    #[test]
    fn test_bus_history_trim_keeps_most_recent_events() {
        let mut bus = EventBus::new().with_history(true);
        bus.max_history = 2;

        bus.publish(Event::new(EventType::Custom, EventSource::system()).with_message("one"));
        bus.publish(Event::new(EventType::Custom, EventSource::system()).with_message("two"));
        bus.publish(Event::new(EventType::Custom, EventSource::system()).with_message("three"));

        let history = bus.history();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].data.message.as_deref(), Some("two"));
        assert_eq!(history[1].data.message.as_deref(), Some("three"));
    }

    #[test]
    fn test_bus_recovers_from_poisoned_next_id_lock() {
        let bus = Arc::new(EventBus::new());
        let poison_bus = bus.clone();
        let _ = std::thread::spawn(move || {
            let _guard = poison_bus
                .next_id
                .write()
                .expect("next_id lock should be writable before poison");
            panic!("intentional lock poison for event bus next_id");
        })
        .join();

        let id = bus.subscribe(Box::new(|_| {}));
        assert!(id > 0);
        assert_eq!(bus.handler_count(), 1);
    }
}
