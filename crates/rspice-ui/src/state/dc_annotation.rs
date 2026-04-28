//! DC Operating Point Annotation State
//!
//! Stores simulation results mapped to schematic elements for overlay display.
//! Follows the pattern of displaying node voltages and branch
//! currents directly on the schematic after DC analysis.
//!
//! # Usage
//!
//! ```ignore
//! let mut annotations = DcAnnotationState::default();
//! annotations.mode = AnnotationMode::All;
//! annotations.populate_from_dc_op(&node_voltages, &branch_currents, &net_positions, &comp_positions);
//!
//! // In rendering loop:
//! for annotation in annotations.visible_annotations() {
//!     render_annotation_badge(annotation);
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::state::Point;

// =============================================================================
// Annotation Mode
// =============================================================================

/// DC annotation display mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AnnotationMode {
    /// No annotations shown
    #[default]
    Hidden,
    /// Show node voltages only
    Voltages,
    /// Show branch currents only
    Currents,
    /// Show both voltages and currents
    All,
}

impl AnnotationMode {
    /// Cycle to next mode (for toolbar toggle)
    pub fn cycle(self) -> Self {
        match self {
            AnnotationMode::Hidden => AnnotationMode::Voltages,
            AnnotationMode::Voltages => AnnotationMode::Currents,
            AnnotationMode::Currents => AnnotationMode::All,
            AnnotationMode::All => AnnotationMode::Hidden,
        }
    }

    /// Get display label for current mode
    pub fn label(self) -> &'static str {
        match self {
            AnnotationMode::Hidden => "Off",
            AnnotationMode::Voltages => "V",
            AnnotationMode::Currents => "I",
            AnnotationMode::All => "V+I",
        }
    }
}

// =============================================================================
// Annotation Data
// =============================================================================

/// Type of annotation for styling purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnnotationKind {
    /// Node voltage annotation
    Voltage,
    /// Branch current annotation
    Current,
    /// Power dissipation annotation
    Power,
}

impl AnnotationKind {
    /// Get color for this annotation type
    pub fn color(self) -> &'static str {
        match self {
            AnnotationKind::Voltage => "#3b82f6", // Blue for voltage
            AnnotationKind::Current => "#ef4444", // Red for current
            AnnotationKind::Power => "#f59e0b",   // Amber for power
        }
    }

    /// Get background color with transparency
    pub fn background(self) -> &'static str {
        match self {
            AnnotationKind::Voltage => "rgba(59, 130, 246, 0.15)",
            AnnotationKind::Current => "rgba(239, 68, 68, 0.15)",
            AnnotationKind::Power => "rgba(245, 158, 11, 0.15)",
        }
    }
}

/// A single annotation value for display on the schematic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Annotation {
    /// Display position (in grid coordinates) - used as fallback if wire not found
    pub position: Point,
    /// Offset from position for label placement (in pixels)
    /// Allows fine-tuning to avoid overlap
    pub offset: (f64, f64),
    /// Formatted value string (e.g., "3.28V", "-1.2mA")
    pub label: String,
    /// Raw numeric value (for sorting, comparison)
    pub value: f64,
    /// Type of annotation (for styling)
    pub kind: AnnotationKind,
    /// Associated element identifier (node name or component name)
    pub source: String,
    /// Wire ID this annotation is attached to (for live tracking)
    /// When the wire moves, the annotation position updates automatically
    #[serde(default)]
    pub wire_id: Option<u64>,
    /// Index of the point within the wire's points array (for tracking specific junction)
    #[serde(default)]
    pub point_index: Option<usize>,
}

impl Annotation {
    /// Create a voltage annotation
    pub fn voltage(position: Point, value: f64, source: String) -> Self {
        Self {
            position,
            offset: (8.0, -8.0), // Default offset: right and up
            label: format_voltage(value),
            value,
            kind: AnnotationKind::Voltage,
            source,
            wire_id: None,
            point_index: None,
        }
    }

    /// Create a current annotation
    pub fn current(position: Point, value: f64, source: String) -> Self {
        Self {
            position,
            offset: (8.0, 12.0), // Default offset: right and down (below voltage)
            label: format_current(value),
            value,
            kind: AnnotationKind::Current,
            source,
            wire_id: None,
            point_index: None,
        }
    }

    /// Create a power annotation
    pub fn power(position: Point, value: f64, source: String) -> Self {
        Self {
            position,
            offset: (8.0, 24.0), // Below current
            label: format_power(value),
            value,
            kind: AnnotationKind::Power,
            source,
            wire_id: None,
            point_index: None,
        }
    }
}

// =============================================================================
// DC Annotation State
// =============================================================================

/// DC annotation state - populated after simulation
///
/// This structure holds all annotations derived from DC operating point
/// analysis. It is designed to be stored alongside simulation state and
/// rendered as an overlay on the schematic.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DcAnnotationState {
    /// Display mode (hidden, voltages, currents, or all)
    pub mode: AnnotationMode,
    /// Voltage annotations keyed by net name
    pub voltages: HashMap<String, Annotation>,
    /// Current annotations keyed by component name
    pub currents: HashMap<String, Annotation>,
    /// Power annotations keyed by component name (optional)
    pub powers: HashMap<String, Annotation>,
    /// Whether annotations are stale (schematic changed since simulation)
    /// Stale annotations are rendered with reduced opacity
    pub is_stale: bool,
    /// Timestamp of when annotations were generated
    pub generated_at: Option<f64>,
}

impl DcAnnotationState {
    /// Create a new empty annotation state
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear all annotations
    pub fn clear(&mut self) {
        self.voltages.clear();
        self.currents.clear();
        self.powers.clear();
        self.is_stale = false;
        self.generated_at = None;
    }

    /// Mark annotations as stale (schematic was modified)
    pub fn mark_stale(&mut self) {
        if !self.voltages.is_empty() || !self.currents.is_empty() {
            self.is_stale = true;
        }
    }

    /// Check if any annotations exist
    pub fn has_annotations(&self) -> bool {
        !self.voltages.is_empty() || !self.currents.is_empty() || !self.powers.is_empty()
    }

    /// Populate voltage annotations from DC OP results
    ///
    /// # Arguments
    /// * `node_voltages` - List of (net_name, voltage) pairs from simulation
    /// * `net_positions` - Map from net name to schematic grid position
    pub fn populate_voltages(
        &mut self,
        node_voltages: &[(String, f64)],
        net_positions: &HashMap<String, Point>,
    ) {
        self.voltages.clear();

        for (net_name, voltage) in node_voltages {
            // Skip ground node
            if net_name == "0" || net_name.eq_ignore_ascii_case("gnd") {
                continue;
            }

            if let Some(&pos) = net_positions.get(net_name) {
                self.voltages.insert(
                    net_name.clone(),
                    Annotation::voltage(pos, *voltage, net_name.clone()),
                );
            }
        }

        self.is_stale = false;
    }

    /// Populate current annotations from DC OP results
    ///
    /// # Arguments
    /// * `branch_currents` - List of (component_name, current) pairs from simulation
    /// * `component_positions` - Map from component name to schematic grid position
    pub fn populate_currents(
        &mut self,
        branch_currents: &[(String, f64)],
        component_positions: &HashMap<String, Point>,
    ) {
        self.currents.clear();

        for (comp_name, current) in branch_currents {
            if let Some(&pos) = component_positions.get(comp_name) {
                self.currents.insert(
                    comp_name.clone(),
                    Annotation::current(pos, *current, comp_name.clone()),
                );
            }
        }
    }

    /// Populate all annotations from DC OP results
    pub fn populate_from_dc_op(
        &mut self,
        node_voltages: &[(String, f64)],
        branch_currents: &[(String, f64)],
        net_positions: &HashMap<String, Point>,
        component_positions: &HashMap<String, Point>,
    ) {
        self.populate_voltages(node_voltages, net_positions);
        self.populate_currents(branch_currents, component_positions);
        self.is_stale = false;

        // Record timestamp
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::time::{SystemTime, UNIX_EPOCH};
            self.generated_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .ok()
                .map(|d| d.as_secs_f64());
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.generated_at = None; // Could use js_sys::Date::now() if needed
        }
    }

    /// Get all annotations that should be visible based on current mode
    pub fn visible_annotations(&self) -> Vec<&Annotation> {
        match self.mode {
            AnnotationMode::Hidden => vec![],
            AnnotationMode::Voltages => self.voltages.values().collect(),
            AnnotationMode::Currents => self.currents.values().collect(),
            AnnotationMode::All => self
                .voltages
                .values()
                .chain(self.currents.values())
                .chain(self.powers.values())
                .collect(),
        }
    }

    /// Get annotation count for display
    pub fn count(&self) -> usize {
        self.voltages.len() + self.currents.len() + self.powers.len()
    }

    /// Get voltage annotation for a specific net
    pub fn get_voltage(&self, net_name: &str) -> Option<&Annotation> {
        self.voltages.get(net_name)
    }

    /// Get current annotation for a specific component
    pub fn get_current(&self, comp_name: &str) -> Option<&Annotation> {
        self.currents.get(comp_name)
    }
}

// =============================================================================
// Value Formatting Functions
// =============================================================================

/// Format voltage with appropriate SI prefix
///
/// Examples:
/// - 3.3 -> "3.300V"
/// - 0.001 -> "1.000mV"
/// - 0.000001 -> "1.000µV"
pub fn format_voltage(v: f64) -> String {
    let abs_v = v.abs();
    if abs_v >= 1.0 {
        format!("{:.3}V", v)
    } else if abs_v >= 1e-3 {
        format!("{:.3}mV", v * 1e3)
    } else if abs_v >= 1e-6 {
        format!("{:.3}µV", v * 1e6)
    } else if abs_v >= 1e-9 {
        format!("{:.3}nV", v * 1e9)
    } else if abs_v == 0.0 {
        "0.000V".to_string()
    } else {
        format!("{:.2e}V", v)
    }
}

/// Format current with appropriate SI prefix
///
/// Examples:
/// - 0.001 -> "1.000mA"
/// - 0.000001 -> "1.000µA"
pub fn format_current(i: f64) -> String {
    let abs_i = i.abs();
    if abs_i >= 1.0 {
        format!("{:.3}A", i)
    } else if abs_i >= 1e-3 {
        format!("{:.3}mA", i * 1e3)
    } else if abs_i >= 1e-6 {
        format!("{:.3}µA", i * 1e6)
    } else if abs_i >= 1e-9 {
        format!("{:.3}nA", i * 1e9)
    } else if abs_i >= 1e-12 {
        format!("{:.3}pA", i * 1e12)
    } else if abs_i == 0.0 {
        "0.000A".to_string()
    } else {
        format!("{:.2e}A", i)
    }
}

/// Format power with appropriate SI prefix
pub fn format_power(p: f64) -> String {
    let abs_p = p.abs();
    if abs_p >= 1.0 {
        format!("{:.3}W", p)
    } else if abs_p >= 1e-3 {
        format!("{:.3}mW", p * 1e3)
    } else if abs_p >= 1e-6 {
        format!("{:.3}µW", p * 1e6)
    } else if abs_p >= 1e-9 {
        format!("{:.3}nW", p * 1e9)
    } else if abs_p == 0.0 {
        "0.000W".to_string()
    } else {
        format!("{:.2e}W", p)
    }
}

// =============================================================================
// Tests
// =============================================================================
