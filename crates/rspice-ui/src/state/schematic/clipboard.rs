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
/// Stores copied components and wires with their relative positions.
/// When pasting, elements are offset from the paste location based on
/// the original selection's center.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClipboardData {
    /// Copied components (stored with original positions)
    pub components: Vec<Component>,

    /// Copied wires (stored with original positions)
    pub wires: Vec<Wire>,

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
        !self.components.is_empty() || !self.wires.is_empty()
    }

    /// Check if clipboard is empty
    pub fn is_empty(&self) -> bool {
        self.components.is_empty() && self.wires.is_empty()
    }

    /// Get total number of items in clipboard
    pub fn count(&self) -> usize {
        self.components.len() + self.wires.len()
    }

    /// Clear all clipboard content
    pub fn clear(&mut self) {
        self.components.clear();
        self.wires.clear();
        self.origin = Point::origin();
    }

    /// Create clipboard data from components and wires
    ///
    /// Calculates the center of the selection as the origin for paste offsets.
    pub fn from_selection(components: Vec<Component>, wires: Vec<Wire>) -> Self {
        let origin = Self::calculate_center(&components, &wires);
        Self {
            components,
            wires,
            origin,
        }
    }

    /// Calculate center point of a selection
    fn calculate_center(components: &[Component], wires: &[Wire]) -> Point {
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
    use crate::state::schematic::component_type::ComponentType;

    #[test]
    fn test_clipboard_new() {
        let clip = ClipboardData::new();
        assert!(clip.is_empty());
        assert!(!clip.has_content());
        assert_eq!(clip.count(), 0);
    }

    #[test]
    fn test_clipboard_has_content() {
        let mut clip = ClipboardData::new();
        assert!(!clip.has_content());

        clip.components
            .push(Component::new(1, ComponentType::Resistor, Point::new(0, 0)));
        assert!(clip.has_content());
    }

    #[test]
    fn test_clipboard_count() {
        let mut clip = ClipboardData::new();
        clip.components
            .push(Component::new(1, ComponentType::Resistor, Point::new(0, 0)));
        clip.wires
            .push(Wire::segment(1, Point::new(0, 0), Point::new(10, 0)));
        assert_eq!(clip.count(), 2);
    }

    #[test]
    fn test_clipboard_clear() {
        let mut clip = ClipboardData::new();
        clip.components
            .push(Component::new(1, ComponentType::Resistor, Point::new(0, 0)));
        clip.clear();
        assert!(clip.is_empty());
        assert_eq!(clip.origin, Point::origin());
    }

    #[test]
    fn test_clipboard_from_selection() {
        let comps = vec![
            Component::new(1, ComponentType::Resistor, Point::new(0, 0)),
            Component::new(2, ComponentType::Capacitor, Point::new(10, 0)),
        ];
        let wires = vec![];

        let clip = ClipboardData::from_selection(comps, wires);

        assert_eq!(clip.components.len(), 2);
        assert_eq!(clip.origin, Point::new(5, 0)); // Center of (0,0) and (10,0)
    }

    #[test]
    fn test_clipboard_center_calculation() {
        let comps = vec![Component::new(
            1,
            ComponentType::Resistor,
            Point::new(10, 20),
        )];
        let wires = vec![Wire::segment(1, Point::new(30, 40), Point::new(50, 60))];

        let clip = ClipboardData::from_selection(comps, wires);

        // Center of (10,20) and (30,40) = (20, 30)
        assert_eq!(clip.origin, Point::new(20, 30));
    }
}
