//! Simulation State
//!
//! Manages simulation execution state and results.

use rspice_core::Value;
use std::path::PathBuf;

/// Simulation execution state
#[derive(Debug, Clone, Default)]
pub struct SimulationState {
    /// Whether a simulation is currently running
    pub is_running: bool,

    /// Current simulation progress (0.0 to 1.0)
    pub progress: f64,

    /// Status message
    pub status: String,

    /// Waveform data for display
    pub waveforms: Vec<WaveformData>,

    /// Console log messages
    pub console_messages: Vec<ConsoleMessage>,

    /// Current netlist content (from editor)
    pub netlist_content: String,

    /// Current file path (if opened/saved)
    pub current_file: Option<PathBuf>,

    /// Whether the editor content has unsaved changes
    pub is_dirty: bool,
}

/// Waveform trace data
#[derive(Debug, Clone, PartialEq)]
pub struct WaveformData {
    /// Trace name (e.g., "V(out)")
    pub name: String,

    /// X-axis values (time or frequency)
    pub x: Vec<Value>,

    /// Y-axis values
    pub y: Vec<Value>,

    /// Trace color (hex string)
    pub color: String,

    /// Whether this trace is visible
    pub visible: bool,
}

impl WaveformData {
    /// Create a new waveform trace
    pub fn new(
        name: impl Into<String>,
        x: Vec<Value>,
        y: Vec<Value>,
        color: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            x,
            y,
            color: color.into(),
            visible: true,
        }
    }

    /// Get the X range (min, max)
    pub fn x_range(&self) -> (Value, Value) {
        let min = self.x.iter().copied().fold(Value::INFINITY, Value::min);
        let max = self.x.iter().copied().fold(Value::NEG_INFINITY, Value::max);
        (min, max)
    }

    /// Get the Y range (min, max)
    pub fn y_range(&self) -> (Value, Value) {
        let min = self.y.iter().copied().fold(Value::INFINITY, Value::min);
        let max = self.y.iter().copied().fold(Value::NEG_INFINITY, Value::max);
        (min, max)
    }
}

/// Console message severity
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageSeverity {
    Info,
    Warning,
    Error,
    Success,
}

/// Console log message
#[derive(Debug, Clone, PartialEq)]
pub struct ConsoleMessage {
    /// Message severity
    pub severity: MessageSeverity,

    /// Message content
    pub message: String,

    /// Timestamp (seconds since simulation start)
    pub timestamp: Option<f64>,
}

impl ConsoleMessage {
    /// Create an info message
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            severity: MessageSeverity::Info,
            message: message.into(),
            timestamp: None,
        }
    }

    /// Create a warning message
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: MessageSeverity::Warning,
            message: message.into(),
            timestamp: None,
        }
    }

    /// Create an error message
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: MessageSeverity::Error,
            message: message.into(),
            timestamp: None,
        }
    }

    /// Create a success message
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            severity: MessageSeverity::Success,
            message: message.into(),
            timestamp: None,
        }
    }
}

impl SimulationState {
    /// Add a console message
    pub fn log(&mut self, message: ConsoleMessage) {
        self.console_messages.push(message);
    }

    /// Clear console messages
    pub fn clear_console(&mut self) {
        self.console_messages.clear();
    }

    /// Clear waveforms
    pub fn clear_waveforms(&mut self) {
        self.waveforms.clear();
    }

    /// Add a waveform trace
    pub fn add_waveform(&mut self, waveform: WaveformData) {
        self.waveforms.push(waveform);
    }
}
