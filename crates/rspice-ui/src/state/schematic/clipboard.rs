//! Clipboard Support
//!
//! Copy/paste functionality for schematic elements.

use super::bus::{Bus, BusTap};
use super::component::Component;
use super::design_note::DesignNote;
use super::documentation_shape::DocumentationShape;
use super::net_label::NetLabel;
use super::point::Point;
use super::wire::Wire;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// =============================================================================
// ClipboardData
// =============================================================================

/// Clipboard data for copy/paste operations
///
/// Stores copied components, wires, explicit junction intent, net labels,
/// buses, and bus taps with their relative positions.
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

    /// Copied net labels, retaining their source stable IDs until paste remaps
    /// them into the destination document namespace.
    #[serde(default)]
    pub net_labels: Vec<NetLabel>,

    /// Copied non-electrical design notes.
    #[serde(default)]
    pub design_notes: Vec<DesignNote>,

    /// Copied non-electrical presentation geometry.
    #[serde(default)]
    pub documentation_shapes: Vec<DocumentationShape>,

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
            || !self.net_labels.is_empty()
            || !self.design_notes.is_empty()
            || !self.documentation_shapes.is_empty()
    }

    /// Check if clipboard is empty
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
            && self.wires.is_empty()
            && self.junctions.is_empty()
            && self.buses.is_empty()
            && self.bus_taps.is_empty()
            && self.net_labels.is_empty()
            && self.design_notes.is_empty()
            && self.documentation_shapes.is_empty()
    }

    /// Get total number of items in clipboard
    pub fn count(&self) -> usize {
        self.components.len()
            + self.wires.len()
            + self.junctions.len()
            + self.buses.len()
            + self.bus_taps.len()
            + self.net_labels.len()
            + self.design_notes.len()
            + self.documentation_shapes.len()
    }

    /// Clear all clipboard content
    pub fn clear(&mut self) {
        self.components.clear();
        self.wires.clear();
        self.junctions.clear();
        self.buses.clear();
        self.bus_taps.clear();
        self.net_labels.clear();
        self.design_notes.clear();
        self.documentation_shapes.clear();
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
        Self::from_selection_with_labels_and_buses(
            components,
            wires,
            junctions,
            Vec::new(),
            buses,
            bus_taps,
        )
    }

    /// Create clipboard data for every complete selectable schematic object.
    pub fn from_selection_with_labels_and_buses(
        components: Vec<Component>,
        wires: Vec<Wire>,
        junctions: Vec<Point>,
        net_labels: Vec<NetLabel>,
        buses: Vec<Bus>,
        bus_taps: Vec<BusTap>,
    ) -> Self {
        Self::from_complete_selection(Self {
            components,
            wires,
            junctions,
            buses,
            bus_taps,
            net_labels,
            design_notes: Vec::new(),
            documentation_shapes: Vec::new(),
            origin: Point::origin(),
        })
    }

    /// Create clipboard data for all selectable electrical and documentation
    /// object kinds.
    pub(crate) fn from_complete_selection(mut selection: Self) -> Self {
        selection.origin = Self::calculate_center(&selection);
        selection
    }

    /// Retain explicit named-net attachments for a duplicated selection.
    ///
    /// These labels are typed clipboard objects, not UI annotations. Paste
    /// remaps their temporary IDs into the destination document namespace and
    /// translates their anchors with the rest of the duplicated geometry.
    /// The source selection origin is intentionally preserved so choosing the
    /// attachment policy cannot move the duplicate.
    pub(crate) fn preserve_named_net_attachments(
        &mut self,
        attachments: impl IntoIterator<Item = (Point, String)>,
    ) -> usize {
        let mut occupied = self
            .net_labels
            .iter()
            .map(|label| (label.pos, label.name.clone()))
            .collect::<HashSet<_>>();
        let mut occupied_ids = self
            .net_labels
            .iter()
            .map(|label| label.id)
            .collect::<HashSet<_>>();
        let mut next_id = self
            .net_labels
            .iter()
            .map(|label| label.id)
            .max()
            .unwrap_or(0)
            .wrapping_add(1);
        let before = self.net_labels.len();

        for (pos, name) in attachments {
            if name.is_empty() || !occupied.insert((pos, name.clone())) {
                continue;
            }
            while occupied_ids.contains(&next_id) {
                next_id = next_id.wrapping_add(1);
            }
            self.net_labels.push(NetLabel::new(next_id, pos, name));
            occupied_ids.insert(next_id);
            next_id = next_id.wrapping_add(1);
        }

        self.net_labels.len().saturating_sub(before)
    }

    /// Calculate the center point of every copied schematic object.
    fn calculate_center(selection: &Self) -> Point {
        let mut cx = 0i64;
        let mut cy = 0i64;
        let mut count = 0i64;

        for comp in &selection.components {
            cx = cx.saturating_add(i64::from(comp.pos.x));
            cy = cy.saturating_add(i64::from(comp.pos.y));
            count = count.saturating_add(1);
        }

        for wire in &selection.wires {
            if let Some(first) = wire.points.first() {
                cx = cx.saturating_add(i64::from(first.x));
                cy = cy.saturating_add(i64::from(first.y));
                count = count.saturating_add(1);
            }
        }

        for junction in &selection.junctions {
            cx = cx.saturating_add(i64::from(junction.x));
            cy = cy.saturating_add(i64::from(junction.y));
            count = count.saturating_add(1);
        }

        for label in &selection.net_labels {
            cx = cx.saturating_add(i64::from(label.pos.x));
            cy = cy.saturating_add(i64::from(label.pos.y));
            count = count.saturating_add(1);
        }

        for bus in &selection.buses {
            if let Some(first) = bus.points.first() {
                cx = cx.saturating_add(i64::from(first.x));
                cy = cy.saturating_add(i64::from(first.y));
                count = count.saturating_add(1);
            }
        }

        for tap in &selection.bus_taps {
            cx = cx.saturating_add(i64::from(tap.connection_point.x));
            cy = cy.saturating_add(i64::from(tap.connection_point.y));
            count = count.saturating_add(1);
        }

        for note in &selection.design_notes {
            cx = cx.saturating_add(i64::from(note.pos.x));
            cy = cy.saturating_add(i64::from(note.pos.y));
            count = count.saturating_add(1);
        }

        for shape in &selection.documentation_shapes {
            let points = shape.geometry.points();
            if points.is_empty() {
                continue;
            }
            let sx: i64 = points.iter().map(|point| i64::from(point.x)).sum();
            let sy: i64 = points.iter().map(|point| i64::from(point.y)).sum();
            let point_count = i64::try_from(points.len()).unwrap_or(i64::MAX).max(1);
            cx = cx.saturating_add(sx / point_count);
            cy = cy.saturating_add(sy / point_count);
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

    #[test]
    fn label_only_clipboard_is_content_centered_on_the_label_anchor() {
        let label = NetLabel::new(17, Point::new(-30, 40), "sense_out");
        let clipboard = ClipboardData::from_selection_with_labels_and_buses(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec![label.clone()],
            Vec::new(),
            Vec::new(),
        );

        assert!(clipboard.has_content());
        assert_eq!(clipboard.count(), 1);
        assert_eq!(clipboard.origin, label.pos);
        assert_eq!(clipboard.net_labels, vec![label]);
    }

    #[test]
    fn legacy_clipboard_without_net_labels_deserializes_empty() {
        let mut value = serde_json::to_value(ClipboardData::default()).unwrap();
        value.as_object_mut().unwrap().remove("net_labels");

        let clipboard: ClipboardData = serde_json::from_value(value).unwrap();

        assert!(clipboard.net_labels.is_empty());
    }

    #[test]
    fn named_net_attachments_are_deduplicated_without_changing_origin() {
        let origin = Point::new(40, 20);
        let mut clipboard = ClipboardData::from_selection(Vec::new(), Vec::new(), vec![origin]);

        assert_eq!(
            clipboard.preserve_named_net_attachments([
                (origin, "sense".to_owned()),
                (origin, "sense".to_owned()),
                (Point::new(60, 20), "out".to_owned()),
            ]),
            2
        );
        assert_eq!(clipboard.origin, origin);
        assert_eq!(clipboard.net_labels.len(), 2);
        assert_ne!(clipboard.net_labels[0].id, clipboard.net_labels[1].id);
    }
}
