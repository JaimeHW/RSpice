//! Clipboard Support
//!
//! Copy/paste functionality for schematic elements.

use super::component::Component;
use super::point::Point;
use super::wire::Wire;
use serde::{Deserialize, Serialize};

// =============================================================================
// ClipboardData
// =============================================================================

/// Clipboard data for copy/paste operations
///
/// Stores copied components, wires, and explicit junction intent with their
/// relative positions.
/// When pasting, elements are offset from the paste location based on
/// the original selection's center.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClipboardData {
    /// Copied components (stored with original positions)
    pub components: Vec<Component>,

    /// Copied wires (stored with original positions)
    pub wires: Vec<Wire>,

    /// Junction-dot positions sitting on the copied wires (stored with
    /// original positions) — without them a pasted multi-way joint loses
    /// its explicit connection dots.
    #[serde(default)]
    pub junctions: Vec<Point>,

    /// Origin point (center of copied selection)
    ///
    /// Used to calculate offsets when pasting at a new location.
    pub origin: Point,
}

impl ClipboardData {
    /// Create an empty clipboard
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if clipboard has any content
    pub fn has_content(&self) -> bool {
        !self.components.is_empty() || !self.wires.is_empty() || !self.junctions.is_empty()
    }

    /// Check if clipboard is empty
    pub fn is_empty(&self) -> bool {
        self.components.is_empty() && self.wires.is_empty() && self.junctions.is_empty()
    }

    /// Get total number of items in clipboard
    pub fn count(&self) -> usize {
        self.components.len() + self.wires.len() + self.junctions.len()
    }

    /// Clear all clipboard content
    pub fn clear(&mut self) {
        self.components.clear();
        self.wires.clear();
        self.junctions.clear();
        self.origin = Point::origin();
    }

    /// Create clipboard data from components, wires, and the junction dots
    /// that sit on those wires.
    ///
    /// Calculates the center of the selection as the origin for paste offsets.
    pub fn from_selection(
        components: Vec<Component>,
        wires: Vec<Wire>,
        junctions: Vec<Point>,
    ) -> Self {
        let origin = Self::calculate_center(&components, &wires, &junctions);
        Self {
            components,
            wires,
            junctions,
            origin,
        }
    }

    /// Calculate the center point of every copied schematic object.
    fn calculate_center(components: &[Component], wires: &[Wire], junctions: &[Point]) -> Point {
        let mut cx = 0i32;
        let mut cy = 0i32;
        let mut count = 0;

        for comp in components {
            cx += comp.pos.x;
            cy += comp.pos.y;
            count += 1;
        }

        for wire in wires {
            if let Some(first) = wire.points.first() {
                cx += first.x;
                cy += first.y;
                count += 1;
            }
        }

        for junction in junctions {
            cx += junction.x;
            cy += junction.y;
            count += 1;
        }

        if count > 0 {
            Point::new(cx / count, cy / count)
        } else {
            Point::origin()
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn junction_only_clipboard_is_real_content_with_its_own_origin() {
        let point = Point::new(30, -10);
        let clipboard = ClipboardData::from_selection(Vec::new(), Vec::new(), vec![point]);

        assert!(clipboard.has_content());
        assert!(!clipboard.is_empty());
        assert_eq!(clipboard.count(), 1);
        assert_eq!(clipboard.origin, point);
    }
}
