//! Cross-Probe Events
//!
//! Event-driven communication for probe state changes.

use std::sync::{Arc, RwLock};

use super::signal::{SignalId, SignalPath};

// =============================================================================
// Probe Event
// =============================================================================

/// Types of probe events
#[derive(Debug, Clone, PartialEq)]
pub enum ProbeEvent {
    /// Signal selected for viewing
    SignalSelected {
        signal_id: SignalId,
        source: ProbeSource,
    },
    /// Signal deselected
    SignalDeselected {
        signal_id: SignalId,
        source: ProbeSource,
    },
    /// Signal highlighted (hover)
    SignalHighlighted {
        signal_id: SignalId,
        source: ProbeSource,
    },
    /// Signal unhighlighted
    SignalUnhighlighted {
        signal_id: SignalId,
        source: ProbeSource,
    },
    /// Node clicked in schematic
    NodeClicked { path: SignalPath, node_name: String },
    /// Trace clicked in waveform
    TraceClicked {
        signal_id: SignalId,
        time: Option<f64>,
    },
    /// Cursor position changed
    CursorMoved { time: f64, source: ProbeSource },
    /// All signals cleared
    AllCleared,
    /// Hierarchy changed
    HierarchyNavigated { path: SignalPath },
}

/// Source of probe event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProbeSource {
    /// Event from schematic view
    Schematic,
    /// Event from waveform view
    Waveform,
    /// Event from signal browser
    Browser,
    /// Event from user script/command
    Command,
    /// Internal system event
    System,
}

impl ProbeSource {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Schematic => "Schematic",
            Self::Waveform => "Waveform",
            Self::Browser => "Browser",
            Self::Command => "Command",
            Self::System => "System",
        }
    }
}

// =============================================================================
// Event Handler
// =============================================================================

/// Trait for handling probe events
pub trait ProbeEventHandler: Send + Sync {
    /// Handle event
    fn handle_event(&mut self, event: &ProbeEvent);

    /// Get handler ID
    fn handler_id(&self) -> u64;

    /// Get handler name for debugging
    fn name(&self) -> &str;
}

/// Default handler implementation that logs events
#[derive(Debug)]
pub struct LoggingProbeHandler {
    id: u64,
    name: String,
    events: Vec<ProbeEvent>,
    max_events: usize,
}

impl LoggingProbeHandler {
    /// Create new logging handler
    pub fn new(name: &str) -> Self {
        static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        Self {
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            name: name.to_string(),
            events: Vec::new(),
            max_events: 1000,
        }
    }

    /// Get recorded events
    pub fn events(&self) -> &[ProbeEvent] {
        &self.events
    }

    /// Clear events
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Number of events
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
}

impl ProbeEventHandler for LoggingProbeHandler {
    fn handle_event(&mut self, event: &ProbeEvent) {
        if self.events.len() < self.max_events {
            self.events.push(event.clone());
        }
    }

    fn handler_id(&self) -> u64 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// =============================================================================
// Event Bus
// =============================================================================

/// Event bus for distributing probe events
#[derive(Default)]
pub struct ProbeEventBus {
    handlers: Vec<Arc<RwLock<dyn ProbeEventHandler>>>,
    paused: bool,
}

impl ProbeEventBus {
    /// Create new event bus
    pub fn new() -> Self {
        Self::default()
    }

    /// Register handler
    pub fn register(&mut self, handler: Arc<RwLock<dyn ProbeEventHandler>>) {
        self.handlers.push(handler);
    }

    /// Unregister handler by ID
    pub fn unregister(&mut self, handler_id: u64) {
        self.handlers.retain(|h| {
            h.read()
                .map(|h| h.handler_id() != handler_id)
                .unwrap_or(true)
        });
    }

    /// Broadcast event to all handlers
    pub fn broadcast(&self, event: &ProbeEvent, _exclude_source: Option<ProbeSource>) {
        if self.paused {
            return;
        }

        // Skip broadcasting to handlers that match the source to avoid loops
        for handler in &self.handlers {
            if let Ok(mut h) = handler.write() {
                h.handle_event(event);
            }
        }
    }

    /// Pause event distribution
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resume event distribution
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Is paused?
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Number of registered handlers
    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }

    /// Clear all handlers
    pub fn clear(&mut self) {
        self.handlers.clear();
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ProbeEvent Tests
    // =========================================================================

    #[test]
    fn test_probe_event_signal_selected() {
        let event = ProbeEvent::SignalSelected {
            signal_id: SignalId::new("v(out)"),
            source: ProbeSource::Schematic,
        };

        match event {
            ProbeEvent::SignalSelected { signal_id, source } => {
                assert_eq!(signal_id.name(), "v(out)");
                assert_eq!(source, ProbeSource::Schematic);
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_probe_event_node_clicked() {
        let event = ProbeEvent::NodeClicked {
            path: SignalPath::from_str("top.amp"),
            node_name: "out".to_string(),
        };

        match event {
            ProbeEvent::NodeClicked { path, node_name } => {
                assert_eq!(path.full_path(), "top.amp");
                assert_eq!(node_name, "out");
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_probe_event_cursor_moved() {
        let event = ProbeEvent::CursorMoved {
            time: 1.5e-9,
            source: ProbeSource::Waveform,
        };

        match event {
            ProbeEvent::CursorMoved { time, source } => {
                assert!((time - 1.5e-9).abs() < 1e-15);
                assert_eq!(source, ProbeSource::Waveform);
            }
            _ => panic!("Wrong event type"),
        }
    }

    // =========================================================================
    // ProbeSource Tests
    // =========================================================================

    #[test]
    fn test_probe_source_names() {
        assert_eq!(ProbeSource::Schematic.display_name(), "Schematic");
        assert_eq!(ProbeSource::Waveform.display_name(), "Waveform");
        assert_eq!(ProbeSource::Browser.display_name(), "Browser");
    }

    // =========================================================================
    // LoggingProbeHandler Tests
    // =========================================================================

    #[test]
    fn test_logging_handler_new() {
        let handler = LoggingProbeHandler::new("Test");
        assert_eq!(handler.name(), "Test");
        assert_eq!(handler.event_count(), 0);
    }

    #[test]
    fn test_logging_handler_record() {
        let mut handler = LoggingProbeHandler::new("Test");
        let event = ProbeEvent::AllCleared;

        handler.handle_event(&event);

        assert_eq!(handler.event_count(), 1);
        assert_eq!(handler.events()[0], ProbeEvent::AllCleared);
    }

    #[test]
    fn test_logging_handler_clear() {
        let mut handler = LoggingProbeHandler::new("Test");
        handler.handle_event(&ProbeEvent::AllCleared);
        handler.handle_event(&ProbeEvent::AllCleared);

        handler.clear();

        assert_eq!(handler.event_count(), 0);
    }

    #[test]
    fn test_logging_handler_unique_ids() {
        let h1 = LoggingProbeHandler::new("H1");
        let h2 = LoggingProbeHandler::new("H2");
        assert_ne!(h1.handler_id(), h2.handler_id());
    }

    // =========================================================================
    // ProbeEventBus Tests
    // =========================================================================

    #[test]
    fn test_event_bus_new() {
        let bus = ProbeEventBus::new();
        assert_eq!(bus.handler_count(), 0);
        assert!(!bus.is_paused());
    }

    #[test]
    fn test_event_bus_register() {
        let mut bus = ProbeEventBus::new();
        let handler = Arc::new(RwLock::new(LoggingProbeHandler::new("Test")));

        bus.register(handler);

        assert_eq!(bus.handler_count(), 1);
    }

    #[test]
    fn test_event_bus_unregister() {
        let mut bus = ProbeEventBus::new();
        let handler = Arc::new(RwLock::new(LoggingProbeHandler::new("Test")));
        let handler_id = handler.read().unwrap().handler_id();

        bus.register(handler);
        assert_eq!(bus.handler_count(), 1);

        bus.unregister(handler_id);
        assert_eq!(bus.handler_count(), 0);
    }

    #[test]
    fn test_event_bus_broadcast() {
        let mut bus = ProbeEventBus::new();
        let handler = Arc::new(RwLock::new(LoggingProbeHandler::new("Test")));

        bus.register(handler.clone());
        bus.broadcast(&ProbeEvent::AllCleared, None);

        assert_eq!(handler.read().unwrap().event_count(), 1);
    }

    #[test]
    fn test_event_bus_pause() {
        let mut bus = ProbeEventBus::new();
        let handler = Arc::new(RwLock::new(LoggingProbeHandler::new("Test")));

        bus.register(handler.clone());
        bus.pause();
        bus.broadcast(&ProbeEvent::AllCleared, None);

        // Should not receive event when paused
        assert_eq!(handler.read().unwrap().event_count(), 0);
    }

    #[test]
    fn test_event_bus_resume() {
        let mut bus = ProbeEventBus::new();
        let handler = Arc::new(RwLock::new(LoggingProbeHandler::new("Test")));

        bus.register(handler.clone());
        bus.pause();
        bus.resume();
        bus.broadcast(&ProbeEvent::AllCleared, None);

        assert_eq!(handler.read().unwrap().event_count(), 1);
    }

    #[test]
    fn test_event_bus_clear() {
        let mut bus = ProbeEventBus::new();
        bus.register(Arc::new(RwLock::new(LoggingProbeHandler::new("H1"))));
        bus.register(Arc::new(RwLock::new(LoggingProbeHandler::new("H2"))));

        assert_eq!(bus.handler_count(), 2);

        bus.clear();

        assert_eq!(bus.handler_count(), 0);
    }

    #[test]
    fn test_event_bus_multiple_handlers() {
        let mut bus = ProbeEventBus::new();
        let h1 = Arc::new(RwLock::new(LoggingProbeHandler::new("H1")));
        let h2 = Arc::new(RwLock::new(LoggingProbeHandler::new("H2")));

        bus.register(h1.clone());
        bus.register(h2.clone());
        bus.broadcast(&ProbeEvent::AllCleared, None);

        assert_eq!(h1.read().unwrap().event_count(), 1);
        assert_eq!(h2.read().unwrap().event_count(), 1);
    }
}
