//! Component Struct
//!
//! A placed component on the schematic with position, rotation, and properties.

use super::component_type::ComponentType;
use super::point::{LabelPosition, Point};
use super::rotation::Rotation;
use serde::{Deserialize, Serialize};

// =============================================================================
// Component
// =============================================================================

/// A placed component on the schematic
///
/// Components are circuit elements placed on the schematic canvas.
/// Each component has a position, rotation, reference designator (name),
/// value, and optional parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    /// Unique identifier within the schematic
    pub id: u64,

    /// Component type (resistor, capacitor, etc.)
    pub kind: ComponentType,

    /// Position on grid (in grid units, not pixels)
    pub pos: Point,

    /// Rotation (0, 90, 180, or 270 degrees)
    pub rotation: Rotation,

    /// Component reference designator (e.g., "R1", "C2", "U3")
    pub name: String,

    /// Component value (e.g., "1k", "10u", "LM741")
    pub value: String,

    /// Additional SPICE parameters (e.g., "TC=0.01,0.001")
    pub params: String,

    /// Name label position (Auto for smart placement, Custom for user-defined)
    #[serde(default)]
    pub name_label_pos: LabelPosition,

    /// Value label position (Auto for smart placement, Custom for user-defined)
    #[serde(default)]
    pub value_label_pos: LabelPosition,
}

impl Component {
    /// Create a new component with default values
    ///
    /// # Arguments
    /// * `id` - Unique identifier
    /// * `kind` - Component type
    /// * `pos` - Position on the grid
    pub fn new(id: u64, kind: ComponentType, pos: Point) -> Self {
        Self {
            id,
            kind,
            pos,
            rotation: Rotation::default(),
            name: String::new(),
            value: String::new(),
            params: String::new(),
            name_label_pos: LabelPosition::Auto,
            value_label_pos: LabelPosition::Auto,
        }
    }

    /// Create a component with name and value
    pub fn with_name_value(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.name = name.into();
        self.value = value.into();
        self
    }

    /// Create a component with rotation
    pub fn with_rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = rotation;
        self
    }

    /// Get terminal positions in world coordinates
    ///
    /// Returns terminal positions accounting for component position and rotation.
    /// Each terminal is identified by name (e.g., "+", "-", "B", "C", "E").
    ///
    /// This is used for:
    /// - Wire snapping to terminals
    /// - Netlist generation
    /// - Rubber-banding during component moves
    pub fn terminal_positions(&self) -> Vec<(&'static str, Point)> {
        // Get terminal offset adjustments from the SVG asset (if any)
        // This is a runtime dependency on the view layer - consider refactoring
        let (term_x_off, term_y_off) = crate::views::symbol_assets::get_terminal_offsets(self.kind);
        let term_offset = Point::new(term_x_off, term_y_off);
        let rotated_term_offset = self.rotate_point(term_offset);

        self.kind
            .terminal_offsets()
            .into_iter()
            .map(|(name, offset)| {
                let rotated = self.rotate_point(offset);
                (
                    name,
                    Point::new(
                        self.pos.x + rotated.x + rotated_term_offset.x,
                        self.pos.y + rotated.y + rotated_term_offset.y,
                    ),
                )
            })
            .collect()
    }

    /// Rotate a point by the component's rotation
    ///
    /// Transforms a point from component-local coordinates to world coordinates,
    /// applying the component's rotation.
    pub fn rotate_point(&self, p: Point) -> Point {
        match self.rotation {
            Rotation::R0 => p,
            Rotation::R90 => Point::new(-p.y, p.x),
            Rotation::R180 => Point::new(-p.x, -p.y),
            Rotation::R270 => Point::new(p.y, -p.x),
        }
    }

    /// Get the bounding box of this component in grid coordinates
    ///
    /// Returns (min_x, min_y, max_x, max_y) representing the approximate
    /// footprint of the component symbol.
    pub fn bounding_box(&self) -> (i32, i32, i32, i32) {
        // Approximate symbol size depends on component type
        let (half_w, half_h) = match self.kind {
            ComponentType::Ground => (1, 2),
            ComponentType::Resistor
            | ComponentType::Capacitor
            | ComponentType::Inductor
            | ComponentType::Diode => (3, 2),
            ComponentType::NpnBjt
            | ComponentType::PnpBjt
            | ComponentType::Nmos
            | ComponentType::Pmos => (3, 3),
            ComponentType::VoltageSource
            | ComponentType::CurrentSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin => (2, 3),
            _ => (2, 2),
        };

        // Swap dimensions if rotated 90 or 270 degrees
        let (hw, hh) = if self.rotation.is_vertical() {
            (half_h, half_w)
        } else {
            (half_w, half_h)
        };

        (
            self.pos.x - hw,
            self.pos.y - hh,
            self.pos.x + hw,
            self.pos.y + hh,
        )
    }

    /// Check if a grid point is within this component's bounding box
    pub fn contains_point(&self, p: Point) -> bool {
        let (min_x, min_y, max_x, max_y) = self.bounding_box();
        p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y
    }

    /// Get the SPICE netlist line for this component
    ///
    /// Note: This is a simplified version. Full netlist generation
    /// requires node connectivity information.
    pub fn spice_instance_name(&self) -> String {
        if self.name.is_empty() {
            format!("{}{}", self.kind.spice_prefix(), self.id)
        } else {
            self.name.clone()
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
    fn test_component_new() {
        let comp = Component::new(1, ComponentType::Resistor, Point::new(10, 20));
        assert_eq!(comp.id, 1);
        assert_eq!(comp.kind, ComponentType::Resistor);
        assert_eq!(comp.pos, Point::new(10, 20));
        assert_eq!(comp.rotation, Rotation::R0);
        assert!(comp.name.is_empty());
        assert!(comp.value.is_empty());
    }

    #[test]
    fn test_component_with_name_value() {
        let comp = Component::new(1, ComponentType::Resistor, Point::new(10, 20))
            .with_name_value("R1", "1k");
        assert_eq!(comp.name, "R1");
        assert_eq!(comp.value, "1k");
    }

    #[test]
    fn test_component_with_rotation() {
        let comp = Component::new(1, ComponentType::Resistor, Point::new(10, 20))
            .with_rotation(Rotation::R90);
        assert_eq!(comp.rotation, Rotation::R90);
    }

    #[test]
    fn test_rotate_point_r0() {
        let comp = Component::new(1, ComponentType::Resistor, Point::new(0, 0));
        let p = Point::new(2, 3);
        let rotated = comp.rotate_point(p);
        assert_eq!(rotated, Point::new(2, 3)); // No rotation
    }

    #[test]
    fn test_rotate_point_r90() {
        let mut comp = Component::new(1, ComponentType::Resistor, Point::new(0, 0));
        comp.rotation = Rotation::R90;
        let p = Point::new(2, 0);
        let rotated = comp.rotate_point(p);
        assert_eq!(rotated, Point::new(0, 2)); // 90° CW: (x,y) -> (-y,x)
    }

    #[test]
    fn test_rotate_point_r180() {
        let mut comp = Component::new(1, ComponentType::Resistor, Point::new(0, 0));
        comp.rotation = Rotation::R180;
        let p = Point::new(2, 3);
        let rotated = comp.rotate_point(p);
        assert_eq!(rotated, Point::new(-2, -3)); // 180°: (x,y) -> (-x,-y)
    }

    #[test]
    fn test_rotate_point_r270() {
        let mut comp = Component::new(1, ComponentType::Resistor, Point::new(0, 0));
        comp.rotation = Rotation::R270;
        let p = Point::new(0, 2);
        let rotated = comp.rotate_point(p);
        assert_eq!(rotated, Point::new(2, 0)); // 270° CW: (x,y) -> (y,-x)
    }

    #[test]
    fn test_bounding_box() {
        let comp = Component::new(1, ComponentType::Resistor, Point::new(10, 20));
        let (min_x, min_y, max_x, max_y) = comp.bounding_box();
        assert!(min_x < comp.pos.x);
        assert!(min_y < comp.pos.y);
        assert!(max_x > comp.pos.x);
        assert!(max_y > comp.pos.y);
    }

    #[test]
    fn test_contains_point() {
        let comp = Component::new(1, ComponentType::Resistor, Point::new(10, 20));
        assert!(comp.contains_point(Point::new(10, 20))); // Center
        assert!(comp.contains_point(Point::new(10, 19))); // Within bounds
        assert!(!comp.contains_point(Point::new(0, 0))); // Far away
    }

    #[test]
    fn test_spice_instance_name() {
        let comp1 =
            Component::new(5, ComponentType::Resistor, Point::new(0, 0)).with_name_value("R1", "");
        assert_eq!(comp1.spice_instance_name(), "R1");

        let comp2 = Component::new(5, ComponentType::Resistor, Point::new(0, 0));
        assert_eq!(comp2.spice_instance_name(), "R5");
    }

    #[test]
    fn test_label_position_defaults() {
        let comp = Component::new(1, ComponentType::Resistor, Point::new(0, 0));
        assert!(comp.name_label_pos.is_auto());
        assert!(comp.value_label_pos.is_auto());
    }
}
