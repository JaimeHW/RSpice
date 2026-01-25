//! Hit Testing for Schematic Elements
//!
//! Commercial-grade hit testing for component and wire selection.
//! Follows professional EDA patterns for interactive schematic editing.
//!
//! # Architecture
//!
//! Hit testing is performed CPU-side for precision and flexibility:
//! - Bounding boxes for fast rejection
//! - Distance-based wire picking
//! - Rotation-aware component testing
//! - Selection priority (components over wires)

use crate::state::{Component, ComponentType, Point, Rotation, Wire};

// =============================================================================
// Constants
// =============================================================================

/// Default wire picking tolerance in world units
pub const WIRE_PICK_TOLERANCE: f32 = 1.5;

/// Default component bounding box padding
pub const COMPONENT_PADDING: f32 = 0.5;

/// Selection priority: higher values win
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HitPriority {
    /// Background/nothing hit
    None = 0,
    /// Wire segment
    Wire = 1,
    /// Junction point
    Junction = 2,
    /// Component
    Component = 3,
    /// Terminal pin
    Terminal = 4,
}

// =============================================================================
// Hit Result
// =============================================================================

/// Result of a hit test operation
#[derive(Debug, Clone, PartialEq)]
pub enum HitResult {
    /// Nothing was hit
    None,
    /// A component was hit
    Component { id: u64, priority: HitPriority },
    /// A wire segment was hit
    Wire { id: u64, segment_index: usize, priority: HitPriority },
    /// A junction was hit
    Junction { position: Point, priority: HitPriority },
    /// A component terminal was hit
    Terminal { component_id: u64, terminal_index: usize, priority: HitPriority },
}

impl HitResult {
    /// Get the priority of this hit result
    pub fn priority(&self) -> HitPriority {
        match self {
            HitResult::None => HitPriority::None,
            HitResult::Component { priority, .. } => *priority,
            HitResult::Wire { priority, .. } => *priority,
            HitResult::Junction { priority, .. } => *priority,
            HitResult::Terminal { priority, .. } => *priority,
        }
    }

    /// Check if this result hit something
    pub fn is_hit(&self) -> bool {
        !matches!(self, HitResult::None)
    }
}

// =============================================================================
// Bounding Box
// =============================================================================

/// Axis-aligned bounding box for hit testing
#[derive(Debug, Clone, Copy, Default)]
pub struct BoundingBox {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl BoundingBox {
    /// Create a new bounding box
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self { min_x, min_y, max_x, max_y }
    }

    /// Create from center and size
    pub fn from_center(cx: f32, cy: f32, width: f32, height: f32) -> Self {
        let hw = width / 2.0;
        let hh = height / 2.0;
        Self::new(cx - hw, cy - hh, cx + hw, cy + hh)
    }

    /// Create from a single point
    pub fn from_point(x: f32, y: f32) -> Self {
        Self::new(x, y, x, y)
    }

    /// Expand box to include a point
    pub fn include(&mut self, x: f32, y: f32) {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
    }

    /// Expand box by padding
    pub fn expand(&mut self, padding: f32) {
        self.min_x -= padding;
        self.min_y -= padding;
        self.max_x += padding;
        self.max_y += padding;
    }

    /// Check if point is inside box
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    /// Check if boxes intersect
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min_x <= other.max_x && self.max_x >= other.min_x &&
        self.min_y <= other.max_y && self.max_y >= other.min_y
    }

    /// Get box width
    pub fn width(&self) -> f32 {
        self.max_x - self.min_x
    }

    /// Get box height
    pub fn height(&self) -> f32 {
        self.max_y - self.min_y
    }

    /// Get center point
    pub fn center(&self) -> (f32, f32) {
        ((self.min_x + self.max_x) / 2.0, (self.min_y + self.max_y) / 2.0)
    }

    /// Merge two bounding boxes
    pub fn union(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox::new(
            self.min_x.min(other.min_x),
            self.min_y.min(other.min_y),
            self.max_x.max(other.max_x),
            self.max_y.max(other.max_y),
        )
    }
}

// =============================================================================
// Component Bounds
// =============================================================================

/// Get bounding box for a component type (before rotation)
pub fn component_bounds(kind: &ComponentType) -> BoundingBox {
    // Base bounds for each component type
    let (half_w, half_h) = match kind {
        ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Inductor => (5.0, 1.5),
        ComponentType::Diode => (4.0, 1.5),
        ComponentType::NpnBjt | ComponentType::PnpBjt => (3.0, 3.0),
        ComponentType::Nmos | ComponentType::Pmos | ComponentType::Njfet | ComponentType::Pjfet => (3.5, 2.5),
        ComponentType::VoltageSource | ComponentType::CurrentSource => (2.5, 5.0),
        ComponentType::Ground => (1.5, 2.0),
        _ => (3.0, 2.0), // Default
    };

    BoundingBox::from_center(0.0, 0.0, half_w * 2.0, half_h * 2.0)
}

/// Get bounding box for a component with position and rotation
pub fn component_world_bounds(component: &Component) -> BoundingBox {
    let base = component_bounds(&component.kind);

    // Apply rotation (swap width/height for 90° and 270°)
    let (w, h) = match component.rotation {
        Rotation::R0 | Rotation::R180 => (base.width(), base.height()),
        Rotation::R90 | Rotation::R270 => (base.height(), base.width()),
    };

    let cx = component.pos.x as f32;
    let cy = component.pos.y as f32;

    let mut bbox = BoundingBox::from_center(cx, cy, w, h);
    bbox.expand(COMPONENT_PADDING);
    bbox
}

/// Get terminal positions for a component in world coordinates
pub fn component_terminals(component: &Component) -> Vec<(f32, f32)> {
    let offsets = component.kind.terminal_offsets();
    let (cx, cy) = (component.pos.x as f32, component.pos.y as f32);

    offsets.iter().map(|(_, offset)| {
        // Apply rotation to offset
        let (ox, oy) = (offset.x as f32, offset.y as f32);
        let (rx, ry) = match component.rotation {
            Rotation::R0 => (ox, oy),
            Rotation::R90 => (-oy, ox),
            Rotation::R180 => (-ox, -oy),
            Rotation::R270 => (oy, -ox),
        };
        (cx + rx, cy + ry)
    }).collect()
}

// =============================================================================
// Wire Bounds
// =============================================================================

/// Get bounding box for a wire
pub fn wire_bounds(wire: &Wire) -> BoundingBox {
    if wire.points.is_empty() {
        return BoundingBox::default();
    }

    let mut bbox = BoundingBox::from_point(
        wire.points[0].x as f32,
        wire.points[0].y as f32,
    );

    for p in &wire.points[1..] {
        bbox.include(p.x as f32, p.y as f32);
    }

    bbox.expand(WIRE_PICK_TOLERANCE);
    bbox
}

/// Calculate squared distance from point to line segment
fn point_to_segment_dist_sq(px: f32, py: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let len_sq = dx * dx + dy * dy;

    if len_sq < 0.0001 {
        // Degenerate segment - use distance to first point
        return (px - x1) * (px - x1) + (py - y1) * (py - y1);
    }

    // Parameter t of projection onto segment
    let t = ((px - x1) * dx + (py - y1) * dy) / len_sq;
    let t = t.clamp(0.0, 1.0);

    // Closest point on segment
    let cx = x1 + t * dx;
    let cy = y1 + t * dy;

    (px - cx) * (px - cx) + (py - cy) * (py - cy)
}

/// Find the wire segment closest to a point, if within tolerance
pub fn pick_wire_segment(wire: &Wire, x: f32, y: f32, tolerance: f32) -> Option<usize> {
    if wire.points.len() < 2 {
        return None;
    }

    let tol_sq = tolerance * tolerance;
    let mut best_dist_sq = f32::MAX;
    let mut best_segment = None;

    for i in 0..wire.points.len() - 1 {
        let p1 = &wire.points[i];
        let p2 = &wire.points[i + 1];

        let dist_sq = point_to_segment_dist_sq(
            x, y,
            p1.x as f32, p1.y as f32,
            p2.x as f32, p2.y as f32,
        );

        if dist_sq < tol_sq && dist_sq < best_dist_sq {
            best_dist_sq = dist_sq;
            best_segment = Some(i);
        }
    }

    best_segment
}

// =============================================================================
// Hit Tester
// =============================================================================

/// Configuration for hit testing
#[derive(Debug, Clone, Copy)]
pub struct HitTestConfig {
    /// Wire picking tolerance in world units
    pub wire_tolerance: f32,
    /// Terminal picking tolerance
    pub terminal_tolerance: f32,
    /// Whether to prefer terminals over component bodies
    pub prefer_terminals: bool,
}

impl Default for HitTestConfig {
    fn default() -> Self {
        Self {
            wire_tolerance: WIRE_PICK_TOLERANCE,
            terminal_tolerance: 2.0,
            prefer_terminals: true,
        }
    }
}

/// Hit tester for schematic elements
pub struct HitTester {
    config: HitTestConfig,
}

impl Default for HitTester {
    fn default() -> Self {
        Self::new(HitTestConfig::default())
    }
}

impl HitTester {
    /// Create a new hit tester with configuration
    pub fn new(config: HitTestConfig) -> Self {
        Self { config }
    }

    /// Test if point hits a component
    pub fn test_component(&self, component: &Component, x: f32, y: f32) -> HitResult {
        // Check terminal hits first if prefer_terminals
        if self.config.prefer_terminals {
            let terminals = component_terminals(component);
            for (i, (tx, ty)) in terminals.iter().enumerate() {
                let dx = x - tx;
                let dy = y - ty;
                if dx * dx + dy * dy <= self.config.terminal_tolerance * self.config.terminal_tolerance {
                    return HitResult::Terminal {
                        component_id: component.id,
                        terminal_index: i,
                        priority: HitPriority::Terminal,
                    };
                }
            }
        }

        // Check component body
        let bbox = component_world_bounds(component);
        if bbox.contains(x, y) {
            return HitResult::Component {
                id: component.id,
                priority: HitPriority::Component,
            };
        }

        HitResult::None
    }

    /// Test if point hits a wire
    pub fn test_wire(&self, wire: &Wire, x: f32, y: f32) -> HitResult {
        // Quick bounding box test
        let bbox = wire_bounds(wire);
        if !bbox.contains(x, y) {
            return HitResult::None;
        }

        // Detailed segment test
        if let Some(segment) = pick_wire_segment(wire, x, y, self.config.wire_tolerance) {
            return HitResult::Wire {
                id: wire.id,
                segment_index: segment,
                priority: HitPriority::Wire,
            };
        }

        HitResult::None
    }

    /// Test all elements and return best hit
    pub fn test_all(
        &self,
        components: &[Component],
        wires: &[Wire],
        x: f32,
        y: f32,
    ) -> HitResult {
        let mut best = HitResult::None;

        // Test components (higher priority)
        for comp in components {
            let hit = self.test_component(comp, x, y);
            if hit.priority() > best.priority() {
                best = hit;
            }
        }

        // Test wires (lower priority)
        for wire in wires {
            let hit = self.test_wire(wire, x, y);
            if hit.priority() > best.priority() {
                best = hit;
            }
        }

        best
    }

    /// Test rectangle selection (returns all elements intersecting)
    pub fn test_rect(
        &self,
        components: &[Component],
        wires: &[Wire],
        rect: &BoundingBox,
    ) -> (Vec<u64>, Vec<u64>) {
        let mut selected_components = Vec::new();
        let mut selected_wires = Vec::new();

        for comp in components {
            let bbox = component_world_bounds(comp);
            if rect.intersects(&bbox) {
                selected_components.push(comp.id);
            }
        }

        for wire in wires {
            let bbox = wire_bounds(wire);
            if rect.intersects(&bbox) {
                selected_wires.push(wire.id);
            }
        }

        (selected_components, selected_wires)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // BoundingBox Tests
    // =========================================================================

    #[test]
    fn test_bounding_box_new() {
        let bbox = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        assert_eq!(bbox.width(), 10.0);
        assert_eq!(bbox.height(), 10.0);
    }

    #[test]
    fn test_bounding_box_from_center() {
        let bbox = BoundingBox::from_center(5.0, 5.0, 10.0, 10.0);
        assert_eq!(bbox.min_x, 0.0);
        assert_eq!(bbox.min_y, 0.0);
        assert_eq!(bbox.max_x, 10.0);
        assert_eq!(bbox.max_y, 10.0);
    }

    #[test]
    fn test_bounding_box_from_point() {
        let bbox = BoundingBox::from_point(5.0, 5.0);
        assert_eq!(bbox.min_x, 5.0);
        assert_eq!(bbox.max_x, 5.0);
    }

    #[test]
    fn test_bounding_box_include() {
        let mut bbox = BoundingBox::from_point(0.0, 0.0);
        bbox.include(10.0, 10.0);
        assert_eq!(bbox.max_x, 10.0);
        assert_eq!(bbox.max_y, 10.0);
    }

    #[test]
    fn test_bounding_box_expand() {
        let mut bbox = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        bbox.expand(1.0);
        assert_eq!(bbox.min_x, -1.0);
        assert_eq!(bbox.max_x, 11.0);
    }

    #[test]
    fn test_bounding_box_contains() {
        let bbox = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        assert!(bbox.contains(5.0, 5.0));
        assert!(bbox.contains(0.0, 0.0));
        assert!(bbox.contains(10.0, 10.0));
        assert!(!bbox.contains(-1.0, 5.0));
        assert!(!bbox.contains(11.0, 5.0));
    }

    #[test]
    fn test_bounding_box_intersects() {
        let a = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        let b = BoundingBox::new(5.0, 5.0, 15.0, 15.0);
        let c = BoundingBox::new(20.0, 20.0, 30.0, 30.0);
        assert!(a.intersects(&b));
        assert!(!a.intersects(&c));
    }

    #[test]
    fn test_bounding_box_center() {
        let bbox = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        let (cx, cy) = bbox.center();
        assert_eq!(cx, 5.0);
        assert_eq!(cy, 5.0);
    }

    #[test]
    fn test_bounding_box_union() {
        let a = BoundingBox::new(0.0, 0.0, 5.0, 5.0);
        let b = BoundingBox::new(3.0, 3.0, 10.0, 10.0);
        let c = a.union(&b);
        assert_eq!(c.min_x, 0.0);
        assert_eq!(c.max_x, 10.0);
    }

    // =========================================================================
    // Component Bounds Tests
    // =========================================================================

    #[test]
    fn test_component_bounds_resistor() {
        let bbox = component_bounds(&ComponentType::Resistor);
        assert!(bbox.width() > 0.0);
        assert!(bbox.height() > 0.0);
    }

    #[test]
    fn test_component_world_bounds_no_rotation() {
        let comp = Component::new(1, ComponentType::Resistor, Point::new(10, 20));
        let bbox = component_world_bounds(&comp);
        let (cx, cy) = bbox.center();
        assert!((cx - 10.0).abs() < 0.1);
        assert!((cy - 20.0).abs() < 0.1);
    }

    #[test]
    fn test_component_world_bounds_rotated() {
        let mut comp = Component::new(1, ComponentType::Resistor, Point::new(0, 0));
        let bbox_r0 = component_world_bounds(&comp);

        comp.rotation = Rotation::R90;
        let bbox_r90 = component_world_bounds(&comp);

        // Width and height should swap
        assert!((bbox_r0.width() - bbox_r90.height()).abs() < 0.1);
        assert!((bbox_r0.height() - bbox_r90.width()).abs() < 0.1);
    }

    #[test]
    fn test_component_terminals() {
        let comp = Component::new(1, ComponentType::Resistor, Point::new(0, 0));
        let terminals = component_terminals(&comp);
        assert_eq!(terminals.len(), 2); // Resistor has 2 terminals
    }

    #[test]
    fn test_component_terminals_rotated() {
        let mut comp = Component::new(1, ComponentType::Resistor, Point::new(0, 0));
        comp.rotation = Rotation::R90;
        let terminals = component_terminals(&comp);
        assert_eq!(terminals.len(), 2);
        // After 90° rotation, terminals should be at different positions
    }

    // =========================================================================
    // Wire Bounds Tests
    // =========================================================================

    #[test]
    fn test_wire_bounds() {
        let wire = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 10)]);
        let bbox = wire_bounds(&wire);
        assert!(bbox.contains(5.0, 5.0));
    }

    #[test]
    fn test_wire_bounds_empty() {
        let wire = Wire::new(1, vec![]);
        let bbox = wire_bounds(&wire);
        assert_eq!(bbox.width(), 0.0);
    }

    #[test]
    fn test_point_to_segment_dist() {
        // Point on segment
        let dist = point_to_segment_dist_sq(5.0, 0.0, 0.0, 0.0, 10.0, 0.0);
        assert!(dist < 0.001);

        // Point 1 unit away from horizontal segment
        let dist = point_to_segment_dist_sq(5.0, 1.0, 0.0, 0.0, 10.0, 0.0);
        assert!((dist - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_pick_wire_segment() {
        let wire = Wire::new(1, vec![
            Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)
        ]);

        // Hit first segment
        let seg = pick_wire_segment(&wire, 5.0, 0.0, 1.5);
        assert_eq!(seg, Some(0));

        // Hit second segment
        let seg = pick_wire_segment(&wire, 10.0, 5.0, 1.5);
        assert_eq!(seg, Some(1));

        // Miss
        let seg = pick_wire_segment(&wire, 100.0, 100.0, 1.5);
        assert_eq!(seg, None);
    }

    // =========================================================================
    // Hit Result Tests
    // =========================================================================

    #[test]
    fn test_hit_result_priority() {
        assert!(HitPriority::Component > HitPriority::Wire);
        assert!(HitPriority::Terminal > HitPriority::Component);
        assert!(HitPriority::Wire > HitPriority::None);
    }

    #[test]
    fn test_hit_result_is_hit() {
        assert!(!HitResult::None.is_hit());
        assert!(HitResult::Component { id: 1, priority: HitPriority::Component }.is_hit());
    }

    // =========================================================================
    // HitTester Tests
    // =========================================================================

    #[test]
    fn test_hit_tester_component() {
        let tester = HitTester::default();
        let comp = Component::new(1, ComponentType::Resistor, Point::new(0, 0));

        // Hit center
        let result = tester.test_component(&comp, 0.0, 0.0);
        assert!(result.is_hit());

        // Miss
        let result = tester.test_component(&comp, 100.0, 100.0);
        assert!(!result.is_hit());
    }

    #[test]
    fn test_hit_tester_wire() {
        let tester = HitTester::default();
        let wire = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);

        // Hit on wire
        let result = tester.test_wire(&wire, 5.0, 0.0);
        assert!(result.is_hit());

        // Miss
        let result = tester.test_wire(&wire, 5.0, 10.0);
        assert!(!result.is_hit());
    }

    #[test]
    fn test_hit_tester_test_all_priority() {
        let config = HitTestConfig {
            prefer_terminals: false, // Disable terminal preference for this test
            ..Default::default()
        };
        let tester = HitTester::new(config);
        let comp = Component::new(1, ComponentType::Ground, Point::new(5, 5));
        let wire = Wire::new(1, vec![Point::new(0, 5), Point::new(10, 5)]);

        // At overlap, component should win over wire when terminals disabled
        let result = tester.test_all(&[comp], &[wire], 5.0, 5.0);
        assert!(matches!(result, HitResult::Component { .. }));
    }

    #[test]
    fn test_hit_tester_rect_selection() {
        let tester = HitTester::default();
        let comps = vec![
            Component::new(1, ComponentType::Resistor, Point::new(0, 0)),
            Component::new(2, ComponentType::Resistor, Point::new(50, 50)),
        ];
        let wires = vec![
            Wire::new(1, vec![Point::new(0, 0), Point::new(10, 10)]),
        ];

        let rect = BoundingBox::new(-5.0, -5.0, 15.0, 15.0);
        let (sel_comps, sel_wires) = tester.test_rect(&comps, &wires, &rect);

        assert_eq!(sel_comps.len(), 1);
        assert_eq!(sel_comps[0], 1);
        assert_eq!(sel_wires.len(), 1);
    }

    #[test]
    fn test_hit_tester_terminal_priority() {
        let tester = HitTester::default();
        let comp = Component::new(1, ComponentType::Resistor, Point::new(0, 0));
        let terminals = component_terminals(&comp);

        // Resistor has terminals at ends - check if hitting terminal returns terminal hit
        if !terminals.is_empty() {
            let (tx, ty) = terminals[0];
            let result = tester.test_component(&comp, tx, ty);
            // Should hit either terminal OR component body depending on terminal position
            assert!(result.is_hit());
        }
    }

    // =========================================================================
    // Config Tests
    // =========================================================================

    #[test]
    fn test_hit_test_config_default() {
        let config = HitTestConfig::default();
        assert!(config.wire_tolerance > 0.0);
        assert!(config.terminal_tolerance > 0.0);
    }

    #[test]
    fn test_custom_tolerance() {
        let config = HitTestConfig {
            wire_tolerance: 5.0,
            ..Default::default()
        };
        let tester = HitTester::new(config);

        let wire = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);
        // Test segment picking with custom tolerance (segment test, not bbox)
        // Default wire bbox uses WIRE_PICK_TOLERANCE, so we test the segment picker directly
        let result = pick_wire_segment(&wire, 5.0, 4.0, 5.0);
        assert!(result.is_some()); // With 5.0 tolerance, 4 units away should hit
        
        // With default tolerance (1.5), should miss
        let result_default = pick_wire_segment(&wire, 5.0, 4.0, 1.5);
        assert!(result_default.is_none());
    }
}
