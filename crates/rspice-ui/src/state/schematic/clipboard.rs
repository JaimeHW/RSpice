//! Clipboard Support
//!
//! Copy/paste functionality for schematic elements.

use super::bus::{Bus, BusTap};
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

    /// Copied bus polylines.
    #[serde(default)]
    pub buses: Vec<Bus>,

    /// Copied taps whose source buses are included above.
    #[serde(default)]
    pub bus_taps: Vec<BusTap>,

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
        !self.components.is_empty()
            || !self.wires.is_empty()
            || !self.junctions.is_empty()
            || !self.buses.is_empty()
            || !self.bus_taps.is_empty()
    }

    /// Check if clipboard is empty
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
            && self.wires.is_empty()
            && self.junctions.is_empty()
            && self.buses.is_empty()
            && self.bus_taps.is_empty()
    }

    /// Get total number of items in clipboard
    pub fn count(&self) -> usize {
        self.components.len()
            + self.wires.len()
            + self.junctions.len()
            + self.buses.len()
            + self.bus_taps.len()
    }

    /// Clear all clipboard content
    pub fn clear(&mut self) {
        self.components.clear();
        self.wires.clear();
        self.junctions.clear();
        self.buses.clear();
        self.bus_taps.clear();
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
        Self::from_selection_with_buses(components, wires, junctions, Vec::new(), Vec::new())
    }

    /// Create clipboard data including typed buses and their taps.
    pub fn from_selection_with_buses(
        components: Vec<Component>,
        wires: Vec<Wire>,
        junctions: Vec<Point>,
        buses: Vec<Bus>,
        bus_taps: Vec<BusTap>,
    ) -> Self {
        let origin = Self::calculate_center(&components, &wires, &junctions, &buses, &bus_taps);
        Self {
            components,
            wires,
            junctions,
            buses,
            bus_taps,
            origin,
        }
    }

    /// Calculate the center point of every copied schematic object.
    fn calculate_center(
        components: &[Component],
        wires: &[Wire],
        junctions: &[Point],
        buses: &[Bus],
        bus_taps: &[BusTap],
    ) -> Point {
        let mut cx = 0i64;
        let mut cy = 0i64;
        let mut count = 0i64;

        for comp in components {
            cx = cx.saturating_add(i64::from(comp.pos.x));
            cy = cy.saturating_add(i64::from(comp.pos.y));
            count = count.saturating_add(1);
        }

        for wire in wires {
            if let Some(first) = wire.points.first() {
                cx = cx.saturating_add(i64::from(first.x));
                cy = cy.saturating_add(i64::from(first.y));
                count = count.saturating_add(1);
            }
        }

        for junction in junctions {
            cx = cx.saturating_add(i64::from(junction.x));
            cy = cy.saturating_add(i64::from(junction.y));
            count = count.saturating_add(1);
        }

        for bus in buses {
            if let Some(first) = bus.points.first() {
                cx = cx.saturating_add(i64::from(first.x));
                cy = cy.saturating_add(i64::from(first.y));
                count = count.saturating_add(1);
            }
        }

        for tap in bus_taps {
            cx = cx.saturating_add(i64::from(tap.connection_point.x));
            cy = cy.saturating_add(i64::from(tap.connection_point.y));
            count = count.saturating_add(1);
        }

        if count > 0 {
            Point::new(
                i32::try_from(cx / count).unwrap_or(if cx.is_negative() {
                    i32::MIN
                } else {
                    i32::MAX
                }),
                i32::try_from(cy / count).unwrap_or(if cy.is_negative() {
                    i32::MIN
                } else {
                    i32::MAX
                }),
            )
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

    #[test]
    fn center_accumulation_does_not_overflow_i32_coordinates() {
        let clipboard = ClipboardData::from_selection(
            Vec::new(),
            Vec::new(),
            vec![
                Point::new(i32::MAX, i32::MIN),
                Point::new(i32::MAX, i32::MIN),
            ],
        );
        assert_eq!(clipboard.origin, Point::new(i32::MAX, i32::MIN));
    }
}
