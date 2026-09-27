//! Persisted per-instance label placement.

use serde::{Deserialize, Serialize};

/// Label position mode for component labels
///
/// Implements smart auto-placement with user override capability.
/// Auto mode uses heuristics to avoid collisions with wires and components.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum LabelPosition {
    /// Automatic smart placement - avoids collisions with wires and components
    #[default]
    Auto,
    /// User-defined custom offset from default position (in pixels)
    Custom { x_offset: f64, y_offset: f64 },
}

impl LabelPosition {
    /// Create a custom label position with the given offsets
    pub fn custom(x_offset: f64, y_offset: f64) -> Self {
        LabelPosition::Custom { x_offset, y_offset }
    }

    /// Check if this is an auto-positioned label
    pub fn is_auto(&self) -> bool {
        matches!(self, LabelPosition::Auto)
    }

    /// Get the custom offsets, or (0, 0) if auto
    pub fn offsets(&self) -> (f64, f64) {
        match self {
            LabelPosition::Auto => (0.0, 0.0),
            LabelPosition::Custom { x_offset, y_offset } => (*x_offset, *y_offset),
        }
    }
}
