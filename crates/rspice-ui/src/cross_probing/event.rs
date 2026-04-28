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

