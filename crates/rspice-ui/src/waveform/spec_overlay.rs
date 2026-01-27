//! Specification Overlay for Waveform Viewer
//!
//! Visualizes design limits (Min/Max/Range) directly on the plot.
//! Highlights pass/fail zones and specification violations.

use crate::services::yield_manager::YieldSpec;
use serde::{Deserialize, Serialize};

/// Visual representation of a design specification on a plot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecOverlay {
    /// Associated specification
    pub spec: YieldSpec,
    /// Color for the pass zone (RGBA)
    pub pass_color: [u8; 4],
    /// Color for the fail zone (RGBA)
    pub fail_color: [u8; 4],
    /// Opacity of the shaded area (0.0 - 1.0)
    pub opacity: f32,
    /// Whether to show the target line
    pub show_target: bool,
    /// Whether to highlight violations on the trace
    pub highlight_violations: bool,
}

impl Default for SpecOverlay {
    fn default() -> Self {
        Self {
            spec: YieldSpec::range("v_out", 0.0, 1.0, "V"),
            pass_color: [46, 204, 113, 40], // Translucent Green
            fail_color: [231, 76, 60, 40],  // Translucent Red
            opacity: 0.2,
            show_target: true,
            highlight_violations: true,
        }
    }
}

impl SpecOverlay {
    /// Create from a YieldSpec
    pub fn from_spec(spec: YieldSpec) -> Self {
        Self {
            spec,
            ..Default::default()
        }
    }

    /// Check if a coordinate (x, y) violates the spec
    pub fn is_violating(&self, _x: f64, y: f64) -> bool {
        !self.spec.evaluates(y)
    }
}
