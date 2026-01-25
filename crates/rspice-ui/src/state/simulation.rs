//! Simulation State
//!
//! Manages simulation execution state and results.

use super::dc_annotation::{AnnotationMode, DcAnnotationState};
use rspice_core::Value;
use std::collections::HashMap;
use std::path::PathBuf;

/// Simulation execution state
#[derive(Debug, Clone, Default)]
pub struct SimulationState {
    /// Whether a simulation is currently running
    pub is_running: bool,

    /// Flag to trigger simulation from menu (toolbar watches this)
    /// When set to true, toolbar will start simulation and reset to false
    pub trigger_simulation: bool,

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

    /// Mapping from netlist node names (N001, N002) to waveform indices
    /// Populated after simulation to enable accurate probing
    pub node_to_waveform: HashMap<String, usize>,

    /// The node selected as ground reference (0V)
    /// When probing this node, we show a message that it's the ground reference
    pub ground_node: Option<String>,

    /// DC operating point annotations for display on schematic
    /// Populated after DC OP simulation, shows node voltages and branch currents
    pub dc_annotations: DcAnnotationState,
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

    /// Toggle visibility of a waveform by name, returns true if found
    /// Handles multiple naming conventions:
    /// - Exact match (e.g., "V(N001)" == "V(N001)")
    /// - Net name matching (e.g., "V(N001)" matches "N001")
    /// - N00X to numeric mapping (e.g., "V(N001)" matches "V(1)")
    pub fn toggle_waveform_visibility(&mut self, probe_name: &str) -> bool {
        // Try exact match first
        for wf in &mut self.waveforms {
            if wf.name.eq_ignore_ascii_case(probe_name) {
                wf.visible = !wf.visible;
                log::info!(
                    "Toggled waveform '{}' visibility to {}",
                    wf.name,
                    wf.visible
                );
                return true;
            }
        }

        // Extract net name from V()/I()
        let net_name = probe_name
            .trim_start_matches("V(")
            .trim_start_matches("I(")
            .trim_end_matches(')');

        // Try matching net name inside V() or I()
        for wf in &mut self.waveforms {
            let wf_net = wf
                .name
                .trim_start_matches("V(")
                .trim_start_matches("I(")
                .trim_end_matches(')');

            if wf_net.eq_ignore_ascii_case(net_name) {
                wf.visible = !wf.visible;
                log::info!(
                    "Toggled waveform '{}' (matched '{}') visibility to {}",
                    wf.name,
                    probe_name,
                    wf.visible
                );
                return true;
            }
        }

        // Handle N00X -> numeric index mapping
        // The netlist generator creates N001, N002, etc. but the simulation
        // engine uses internal numeric indices like 1, 2, 3
        if let Some(numeric_index) = Self::extract_n00x_numeric(net_name) {
            for wf in &mut self.waveforms {
                let wf_net = wf
                    .name
                    .trim_start_matches("V(")
                    .trim_start_matches("I(")
                    .trim_end_matches(')');

                if wf_net == numeric_index {
                    wf.visible = !wf.visible;
                    log::info!(
                        "Toggled waveform '{}' (N00X matched '{}') visibility to {}",
                        wf.name,
                        probe_name,
                        wf.visible
                    );
                    return true;
                }
            }
        }

        // Check if this is the ground reference node
        let net_name_check = probe_name
            .trim_start_matches("V(")
            .trim_start_matches("I(")
            .trim_end_matches(')');

        if let Some(ref ground) = self.ground_node {
            if ground.eq_ignore_ascii_case(net_name_check) {
                log::info!(
                    "Probe '{}' is the ground reference (0V) - no waveform displayed",
                    probe_name
                );
                return false;
            }
        }

        log::warn!(
            "Probe '{}' not found in {} waveforms",
            probe_name,
            self.waveforms.len()
        );
        false
    }

    /// Extract numeric index from N00X format (e.g., "N001" -> "1", "N002" -> "2")
    fn extract_n00x_numeric(name: &str) -> Option<String> {
        let name_upper = name.to_uppercase();
        if name_upper.starts_with('N') {
            let rest = &name[1..];
            // Try to parse as a number and strip leading zeros
            if let Ok(num) = rest.parse::<u32>() {
                return Some(num.to_string());
            }
        }
        None
    }

    /// Find waveform names that match a node/net name
    pub fn find_waveforms_for_node(&self, node_name: &str) -> Vec<String> {
        self.waveforms
            .iter()
            .filter(|wf| {
                let wf_net = wf
                    .name
                    .trim_start_matches("V(")
                    .trim_start_matches("I(")
                    .trim_end_matches(')');
                wf_net.eq_ignore_ascii_case(node_name)
            })
            .map(|wf| wf.name.clone())
            .collect()
    }
}
