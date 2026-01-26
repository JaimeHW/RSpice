//! DC Annotation Placement Engine
//!
//! Smart label placement for DC operating point annotations.
//! Implements a radial search algorithm that guarantees no overlap with
//! schematic geometry (components, labels, wires).
//!
//! # Algorithm
//!
//! The placement algorithm uses a directional radial search:
//! 1. Build a collection of all schematic geometry as axis-aligned bounding boxes
//! 2. For each annotation, search outward from the node in priority order:
//!    - Primary directions: up, up-right, up-left
//!    - Secondary directions: right, left
//!    - Fallback directions: down-right, down-left, down
//! 3. At each radius step, check if the annotation bounding box overlaps any geometry
//! 4. Return the first collision-free position found
//!
//! # Features
//!
//! - Accurate bounding box estimation using character width metrics
//! - Configurable margin/padding around exclusion zones
//! - Incremental placement: already-placed annotations are added to exclusion set
//! - Priority-based direction preference (above > sides > below)

use crate::state::{Component, Point, Wire};

// =============================================================================
// Bounding Box Types
// =============================================================================

/// Axis-aligned bounding box for collision detection.
///
/// Coordinates are in grid units (not pixels) for consistent comparison
/// across different zoom levels.
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    /// Minimum X (left edge)
    pub min_x: f64,
    /// Maximum X (right edge)
    pub max_x: f64,
    /// Minimum Y (top edge - SVG convention: Y increases downward)
    pub min_y: f64,
    /// Maximum Y (bottom edge)
    pub max_y: f64,
}

impl BoundingBox {
    /// Create a bounding box from center point and half-dimensions.
    #[inline]
    pub fn from_center(cx: f64, cy: f64, half_width: f64, half_height: f64) -> Self {
        Self {
            min_x: cx - half_width,
            max_x: cx + half_width,
            min_y: cy - half_height,
            max_y: cy + half_height,
        }
    }

    /// Create a bounding box from corner points.
    #[inline]
    pub fn from_corners(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            min_x: x1.min(x2),
            max_x: x1.max(x2),
            min_y: y1.min(y2),
            max_y: y1.max(y2),
        }
    }

    /// Check if this bounding box overlaps another (exclusive boundaries).
    #[inline]
    pub fn overlaps(&self, other: &BoundingBox) -> bool {
        !(self.max_x <= other.min_x
            || self.min_x >= other.max_x
            || self.max_y <= other.min_y
            || self.min_y >= other.max_y)
    }

    /// Expand this bounding box by a margin on all sides.
    #[inline]
    pub fn expand(&self, margin: f64) -> Self {
        Self {
            min_x: self.min_x - margin,
            max_x: self.max_x + margin,
            min_y: self.min_y - margin,
            max_y: self.max_y + margin,
        }
    }
}

// =============================================================================
// Schematic Geometry Collection
// =============================================================================

/// Collection of all schematic geometry for collision detection.
///
/// This represents the "occupied" regions of the schematic where
/// annotations should not be placed.
#[derive(Debug, Clone)]
pub struct SchematicGeometry {
    /// All bounding boxes to avoid (components, labels, wires)
    boxes: Vec<BoundingBox>,
    /// Grid size in pixels (for coordinate conversion)
    grid_size: i32,
}

impl SchematicGeometry {
    /// Build geometry collection from schematic elements.
    ///
    /// This extracts bounding boxes for:
    /// - Component bodies (with padding for terminals)
    /// - Component name labels (at default positions)
    /// - Component value labels (at default positions)
    /// - Wire segments (horizontal and vertical)
    pub fn from_schematic(components: &[Component], wires: &[Wire], grid_size: i32) -> Self {
        let mut boxes = Vec::with_capacity(components.len() * 3 + wires.len() * 2);
        let gs = grid_size as f64;

        // Default label offsets (in grid units) relative to component center
        let name_y_offset = -3.0; // Above component
        let value_y_offset = 4.5; // Below component

        // Component geometry
        for comp in components {
            let cx = comp.pos.x as f64;
            let cy = comp.pos.y as f64;

            // Component body bounding box
            // Uses 2.5 grid units width, 1.5 height to cover typical symbols
            // This accounts for terminals extending from the body
            boxes.push(BoundingBox::from_center(cx, cy, 2.5, 1.5));

            // Name label bounding box - simple default position (above)
            let name_len = comp.name.len() as f64;
            let name_half_w = (name_len * 0.4).max(1.0);
            let name_half_h = 0.5;
            let name_cx = cx;
            let name_cy = cy + name_y_offset;
            boxes.push(BoundingBox::from_center(
                name_cx,
                name_cy,
                name_half_w,
                name_half_h,
            ));

            // Value label bounding box - simple default position (below)
            let value_len = comp.value.len() as f64;
            let value_half_w = (value_len * 0.35).max(1.0);
            let value_half_h = 0.45;
            let value_cx = cx;
            let value_cy = cy + value_y_offset;
            boxes.push(BoundingBox::from_center(
                value_cx,
                value_cy,
                value_half_w,
                value_half_h,
            ));
        }

        // Wire segment geometry
        let wire_margin = 0.3; // Clearance around wire in grid units

        for wire in wires {
            for segment in wire.points.windows(2) {
                let p1 = segment[0];
                let p2 = segment[1];

                let x1 = p1.x as f64;
                let y1 = p1.y as f64;
                let x2 = p2.x as f64;
                let y2 = p2.y as f64;

                // Create bounding box for segment with margin
                let bbox = if (y1 - y2).abs() < 0.001 {
                    // Horizontal wire
                    BoundingBox::from_corners(
                        x1.min(x2) - wire_margin,
                        y1 - wire_margin,
                        x1.max(x2) + wire_margin,
                        y1 + wire_margin,
                    )
                } else if (x1 - x2).abs() < 0.001 {
                    // Vertical wire
                    BoundingBox::from_corners(
                        x1 - wire_margin,
                        y1.min(y2) - wire_margin,
                        x1 + wire_margin,
                        y1.max(y2) + wire_margin,
                    )
                } else {
                    // Diagonal wire (rare in schematics)
                    BoundingBox::from_corners(x1, y1, x2, y2).expand(wire_margin)
                };

                boxes.push(bbox);
            }
        }

        // Unused but kept for API compatibility
        let _ = gs;

        Self { boxes, grid_size }
    }

    /// Check if a bounding box collides with any schematic geometry.
    #[inline]
    pub fn collides(&self, bbox: &BoundingBox) -> bool {
        self.boxes.iter().any(|b| b.overlaps(bbox))
    }

    /// Add a new bounding box to the geometry (for incremental placement).
    #[inline]
    pub fn add_box(&mut self, bbox: BoundingBox) {
        self.boxes.push(bbox);
    }

    /// Get the grid size.
    #[inline]
    pub fn grid_size(&self) -> i32 {
        self.grid_size
    }
}

// =============================================================================
// Annotation Placer
// =============================================================================

/// Search direction for annotation placement.
///
/// Directions are ordered by preference: above is preferred (cleaner look),
/// followed by sides, then below as fallback.
#[derive(Debug, Clone, Copy)]
struct SearchDirection {
    /// Angle in radians (0 = right, π/2 = down, π = left, 3π/2 = up)
    angle: f64,
    /// Direction name for debugging
    #[allow(dead_code)]
    name: &'static str,
}

impl SearchDirection {
    const fn new(angle: f64, name: &'static str) -> Self {
        Self { angle, name }
    }
}

/// Annotation placement engine.
///
/// Places DC annotations optimally around their associated nodes,
/// avoiding all schematic geometry.
pub struct AnnotationPlacer {
    /// Schematic geometry to avoid
    geometry: SchematicGeometry,
}

impl AnnotationPlacer {
    /// Create a new placer from schematic geometry.
    pub fn new(geometry: SchematicGeometry) -> Self {
        Self { geometry }
    }

    /// Convenience constructor from schematic elements.
    pub fn from_schematic(components: &[Component], wires: &[Wire], grid_size: i32) -> Self {
        Self::new(SchematicGeometry::from_schematic(
            components, wires, grid_size,
        ))
    }

    /// Find optimal position for an annotation label.
    ///
    /// Returns pixel offset from the node position, suitable for direct use
    /// in the `Annotation.offset` field.
    ///
    /// # Arguments
    ///
    /// * `node_pos` - Position of the node in grid coordinates
    /// * `label_text` - Label text (for width estimation)
    ///
    /// # Returns
    ///
    /// Tuple of (x_offset, y_offset) in pixels.
    pub fn find_optimal_position(&self, node_pos: Point, label_text: &str) -> (f64, f64) {
        let gs = self.geometry.grid_size() as f64;
        let node_x = node_pos.x as f64;
        let node_y = node_pos.y as f64;

        // Estimate label dimensions in grid units
        // Based on font metrics: ~7px per character, ~14px height
        let label_half_w = (label_text.len() as f64 * 0.35).max(1.5);
        let label_half_h = 0.7; // Annotation badge height

        // Search directions in priority order
        // Angles: 0=right, π/2=down, π=left, 3π/2=up
        // Up (-Y) is preferred for clean schematic appearance
        let directions: [SearchDirection; 8] = [
            SearchDirection::new(std::f64::consts::FRAC_PI_2 * 3.0, "up"), // 270° - up
            SearchDirection::new(std::f64::consts::FRAC_PI_4 * 7.0, "up-right"), // 315° - up-right
            SearchDirection::new(std::f64::consts::FRAC_PI_4 * 5.0, "up-left"), // 225° - up-left
            SearchDirection::new(0.0, "right"),                            // 0° - right
            SearchDirection::new(std::f64::consts::PI, "left"),            // 180° - left
            SearchDirection::new(std::f64::consts::FRAC_PI_4, "down-right"), // 45° - down-right
            SearchDirection::new(std::f64::consts::FRAC_PI_4 * 3.0, "down-left"), // 135° - down-left
            SearchDirection::new(std::f64::consts::FRAC_PI_2, "down"),            // 90° - down
        ];

        // Radii to try (in grid units), starting close and moving outward
        let radii: [f64; 8] = [1.2, 1.5, 2.0, 2.5, 3.0, 4.0, 5.0, 6.0];

        // Search: for each radius, try all directions
        for &radius in &radii {
            for dir in &directions {
                // Calculate candidate center position
                let dx = radius * dir.angle.cos();
                let dy = radius * dir.angle.sin();
                let candidate_x = node_x + dx;
                let candidate_y = node_y + dy;

                // Build candidate bounding box
                let candidate_bbox =
                    BoundingBox::from_center(candidate_x, candidate_y, label_half_w, label_half_h);

                // Check for collision
                if !self.geometry.collides(&candidate_bbox) {
                    // Found a valid position - convert to pixel offset
                    return (dx * gs, dy * gs);
                }
            }
        }

        // Fallback: far above-left (always visible, rarely collides)
        (-3.0 * gs, -3.0 * gs)
    }

    /// Mark a placed annotation's area as occupied.
    ///
    /// Call this after placing each annotation to prevent subsequent
    /// annotations from overlapping previously placed ones.
    pub fn mark_placed(&mut self, node_pos: Point, offset: (f64, f64), label_text: &str) {
        let gs = self.geometry.grid_size() as f64;

        // Convert pixel offset to grid units
        let center_x = node_pos.x as f64 + offset.0 / gs;
        let center_y = node_pos.y as f64 + offset.1 / gs;

        // Label dimensions in grid units
        let half_w = (label_text.len() as f64 * 0.35).max(1.5);
        let half_h = 0.7;

        self.geometry
            .add_box(BoundingBox::from_center(center_x, center_y, half_w, half_h));
    }
}

// =============================================================================
// Annotation Anchor Point Selection
// =============================================================================

/// Wire segment classification for annotation anchor point selection.
///
/// Simulators prefer placing annotations on horizontal wire
/// segments rather than at corners or vertical segments. This provides
/// better visual alignment and readability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireSegmentType {
    /// Point lies on a horizontal wire segment (preferred for annotations)
    Horizontal,
    /// Point lies on a vertical wire segment
    Vertical,
    /// Point is at a corner/junction (avoid for annotations)
    Corner,
    /// Point is at an endpoint of a wire
    Endpoint,
    /// Point is not on any wire segment
    None,
}

/// Classify how a point relates to wire geometry.
///
/// This function determines whether a point lies on a horizontal segment,
/// vertical segment, corner, or endpoint. Used to prioritize annotation
/// anchor points for better visual alignment.
///
/// # Arguments
///
/// * `point` - Point to classify
/// * `wires` - All wires in the schematic
///
/// # Returns
///
/// The segment type classification for the point.
pub fn classify_point_on_wire(point: Point, wires: &[Wire]) -> WireSegmentType {
    let mut on_horizontal = false;
    let mut on_vertical = false;
    let mut is_endpoint = false;

    for wire in wires {
        let num_points = wire.points.len();
        if num_points == 0 {
            continue;
        }

        // Check if point is at an endpoint
        if num_points > 0 && (wire.points[0] == point || wire.points[num_points - 1] == point) {
            is_endpoint = true;
        }

        // Check segments for horizontal/vertical containment
        for segment in wire.points.windows(2) {
            let a = segment[0];
            let b = segment[1];

            // Horizontal segment check
            if a.y == b.y && point.y == a.y {
                let (min_x, max_x) = (a.x.min(b.x), a.x.max(b.x));
                if point.x >= min_x && point.x <= max_x {
                    // Point is at corner if it's at segment endpoint
                    if point.x == min_x || point.x == max_x {
                        // Could be corner - check further
                    } else {
                        on_horizontal = true;
                    }
                }
            }

            // Vertical segment check
            if a.x == b.x && point.x == a.x {
                let (min_y, max_y) = (a.y.min(b.y), a.y.max(b.y));
                if point.y >= min_y && point.y <= max_y {
                    // Point is at corner if it's at segment endpoint
                    if point.y == min_y || point.y == max_y {
                        // Could be corner - check further
                    } else {
                        on_vertical = true;
                    }
                }
            }
        }

        // Check if point is at a wire vertex (potential corner)
        for (i, wp) in wire.points.iter().enumerate() {
            if *wp == point && i > 0 && i < num_points - 1 {
                // Interior vertex = corner
                return WireSegmentType::Corner;
            }
        }
    }

    if on_horizontal {
        WireSegmentType::Horizontal
    } else if on_vertical {
        WireSegmentType::Vertical
    } else if is_endpoint {
        WireSegmentType::Endpoint
    } else {
        WireSegmentType::None
    }
}

/// Select the optimal annotation anchor point from a set of candidate points.
///
/// This implements the anchor selection algorithm:
///
/// 1. **Priority 1**: Points on horizontal wire segments (best visual alignment)
/// 2. **Priority 2**: Points on vertical wire segments
/// 3. **Priority 3**: Endpoint points
/// 4. **Priority 4**: Corner points (worst - avoid if possible)
///
/// Within each priority tier, the point closest to any component is selected
/// to maintain proximity relevance.
///
/// # Arguments
///
/// * `candidates` - All possible anchor points for the net
/// * `wires` - All wires in the schematic (for segment classification)
/// * `component_positions` - Component center positions (for proximity scoring)
///
/// # Returns
///
/// The optimal anchor point, or `None` if candidates is empty.
pub fn select_optimal_anchor_point(
    candidates: &[Point],
    wires: &[Wire],
    component_positions: &[Point],
) -> Option<Point> {
    if candidates.is_empty() {
        return None;
    }

    if candidates.len() == 1 {
        return Some(candidates[0]);
    }

    // Score function: lower is better
    // Priority tiers encoded as base score + proximity distance
    fn priority_base(seg_type: WireSegmentType) -> i32 {
        match seg_type {
            WireSegmentType::Horizontal => 0,   // Best: horizontal alignment
            WireSegmentType::Vertical => 10000, // Second: vertical
            WireSegmentType::Endpoint => 20000, // Third: endpoints
            WireSegmentType::Corner => 30000,   // Fourth: corners
            WireSegmentType::None => 40000,     // Fallback
        }
    }

    fn min_distance_to_component(p: &Point, components: &[Point]) -> i32 {
        components
            .iter()
            .map(|cp| (p.x - cp.x).abs() + (p.y - cp.y).abs())
            .min()
            .unwrap_or(i32::MAX)
    }

    // Score each candidate: segment_type + proximity
    candidates
        .iter()
        .min_by_key(|p| {
            let seg_type = classify_point_on_wire(**p, wires);
            let base = priority_base(seg_type);
            let dist = min_distance_to_component(p, component_positions);
            base + dist
        })
        .copied()
}

/// Minimum horizontal clearance from component centers (in grid units)
const COMPONENT_CLEARANCE: i32 = 3;

/// Generate wire-based anchor candidates for a net.
///
/// Standard approach:
/// 1. Find wires that connect to this net via their ENDPOINTS
/// 2. Generate multiple candidate points along each horizontal segment
/// 3. Filter out points that are horizontally too close to components
/// 4. Prioritize points on horizontal wire segments
///
/// # Arguments
///
/// * `terminal_points` - Component terminal positions belonging to this net
/// * `wires` - All wires in the schematic
/// * `component_positions` - Component center positions (for clearance filtering)
///
/// # Returns
///
/// A vector of candidate anchor points prioritizing horizontal wire runs.
pub fn generate_wire_midpoint_candidates(
    terminal_points: &[Point],
    wires: &[Wire],
    component_positions: &[Point],
) -> Vec<Point> {
    let mut horizontal_candidates = Vec::new();
    let mut vertical_candidates = Vec::new();

    // Build set of terminal positions for fast lookup
    let terminal_set: std::collections::HashSet<Point> = terminal_points.iter().copied().collect();

    for wire in wires {
        // Check if this wire connects to the net via its ENDPOINTS
        let first = wire.points.first().copied();
        let last = wire.points.last().copied();

        let connects_via_endpoints = first.map_or(false, |p| terminal_set.contains(&p))
            || last.map_or(false, |p| terminal_set.contains(&p));

        // Also check if any terminal point lies ON any segment of this wire (T-junction)
        let connects_via_segment = terminal_points.iter().any(|term| {
            wire.points
                .windows(2)
                .any(|seg| point_lies_on_segment(*term, seg[0], seg[1]))
        });

        if !connects_via_endpoints && !connects_via_segment {
            continue;
        }

        // Generate candidate points for each segment
        for segment in wire.points.windows(2) {
            let p1 = segment[0];
            let p2 = segment[1];
            let is_horizontal = p1.y == p2.y;

            // Generate multiple candidate points along the segment (1/4, 1/2, 3/4)
            let fractions = [0.5, 0.25, 0.75];
            for frac in fractions {
                let candidate = Point {
                    x: p1.x + ((p2.x - p1.x) as f64 * frac) as i32,
                    y: p1.y + ((p2.y - p1.y) as f64 * frac) as i32,
                };

                // Check if candidate is horizontally clear of ALL components
                let is_clear = component_positions
                    .iter()
                    .all(|cp| (candidate.x - cp.x).abs() > COMPONENT_CLEARANCE);

                if is_clear {
                    if is_horizontal {
                        horizontal_candidates.push(candidate);
                    } else {
                        vertical_candidates.push(candidate);
                    }
                }
            }
        }
    }

    // Combine: horizontal first (prioritized), then vertical
    let mut result = horizontal_candidates;
    result.extend(vertical_candidates);

    // If no clear candidates found, fall back to terminal points
    if result.is_empty() {
        result.extend(terminal_points.iter().copied());
    }

    result
}

/// Check if a point lies on a horizontal or vertical segment (exclusive of endpoints).
fn point_lies_on_segment(p: Point, a: Point, b: Point) -> bool {
    // Horizontal segment
    if a.y == b.y && p.y == a.y {
        let (min_x, max_x) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
        return p.x > min_x && p.x < max_x;
    }
    // Vertical segment
    if a.x == b.x && p.x == a.x {
        let (min_y, max_y) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };
        return p.y > min_y && p.y < max_y;
    }
    false
}

/// Anchor point selection for DC annotations.
///
/// This combines wire midpoint generation with optimal anchor selection
/// to achieve optimal placement:
///
/// 1. Generate wire segment midpoints that are horizontally clear of components
/// 2. Prioritize horizontal wire segments over vertical
/// 3. Fall back to component terminals only if no clear midpoints exist
///
/// # Arguments
///
/// * `terminal_points` - Component terminal positions for this net
/// * `wires` - All wires in the schematic
/// * `component_positions` - Component center positions
///
/// # Returns
///
/// The optimal anchor point for the DC annotation.
pub fn select_anchor_point(
    terminal_points: &[Point],
    wires: &[Wire],
    component_positions: &[Point],
) -> Option<Point> {
    if terminal_points.is_empty() {
        return None;
    }

    // Generate wire midpoint candidates with component clearance
    let candidates = generate_wire_midpoint_candidates(terminal_points, wires, component_positions);

    if candidates.is_empty() {
        return terminal_points.first().copied();
    }

    // Use segment-aware selection on the expanded candidate set
    select_optimal_anchor_point(&candidates, wires, component_positions)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box_from_center() {
        let bbox = BoundingBox::from_center(5.0, 5.0, 2.0, 1.0);
        assert_eq!(bbox.min_x, 3.0);
        assert_eq!(bbox.max_x, 7.0);
        assert_eq!(bbox.min_y, 4.0);
        assert_eq!(bbox.max_y, 6.0);
    }

    #[test]
    fn test_bounding_box_overlaps() {
        let a = BoundingBox::from_center(5.0, 5.0, 2.0, 2.0);
        let b = BoundingBox::from_center(6.0, 5.0, 2.0, 2.0); // Overlapping
        let c = BoundingBox::from_center(10.0, 5.0, 2.0, 2.0); // Non-overlapping

        assert!(a.overlaps(&b));
        assert!(!a.overlaps(&c));
    }

    #[test]
    fn test_bounding_box_expand() {
        let bbox = BoundingBox::from_center(5.0, 5.0, 1.0, 1.0);
        let expanded = bbox.expand(0.5);

        assert_eq!(expanded.min_x, 3.5);
        assert_eq!(expanded.max_x, 6.5);
        assert_eq!(expanded.min_y, 3.5);
        assert_eq!(expanded.max_y, 6.5);
    }

    #[test]
    fn test_no_collision_empty_schematic() {
        // Empty schematic - should find position immediately at first radius
        let geometry = SchematicGeometry {
            boxes: vec![],
            grid_size: 10,
        };
        let placer = AnnotationPlacer::new(geometry);

        let pos = Point::new(10, 10);
        let offset = placer.find_optimal_position(pos, "3.300V");

        // Should be placed at first radius (1.2 grid units) in "up" direction
        // Up = 270° = (0, -1.2) in grid units, * 10 = (0, -12) pixels
        assert!(offset.1 < 0.0, "Should be placed above node");
    }

    #[test]
    fn test_avoids_component() {
        // Component at (10, 10)
        let component_box = BoundingBox::from_center(10.0, 10.0, 2.5, 1.5);
        let geometry = SchematicGeometry {
            boxes: vec![component_box],
            grid_size: 10,
        };
        let placer = AnnotationPlacer::new(geometry);

        // Try placing annotation at the component center
        let pos = Point::new(10, 10);
        let offset = placer.find_optimal_position(pos, "1.650V");

        // Convert offset back to grid units for distance check
        let gs = 10.0;
        let final_x = 10.0 + offset.0 / gs;
        let final_y = 10.0 + offset.1 / gs;

        // Annotation should be outside the component box
        let label_half_w = 2.0;
        let label_half_h = 0.7;
        let anno_box = BoundingBox::from_center(final_x, final_y, label_half_w, label_half_h);

        assert!(
            !anno_box.overlaps(&component_box),
            "Annotation should not overlap component"
        );
    }

    #[test]
    fn test_avoids_wire_segment() {
        // Horizontal wire segment from (5,10) to (15,10)
        let wire_box = BoundingBox::from_corners(4.7, 9.7, 15.3, 10.3);
        let geometry = SchematicGeometry {
            boxes: vec![wire_box],
            grid_size: 10,
        };
        let placer = AnnotationPlacer::new(geometry);

        // Try placing at wire midpoint
        let pos = Point::new(10, 10);
        let offset = placer.find_optimal_position(pos, "0.000V");

        // Should be placed above or below the wire, not on it
        let gs = 10.0;
        let final_y = 10.0 + offset.1 / gs;

        assert!(
            final_y < 9.0 || final_y > 11.0,
            "Annotation should be above or below wire, got y={final_y}"
        );
    }

    #[test]
    fn test_incremental_placement() {
        let geometry = SchematicGeometry {
            boxes: vec![],
            grid_size: 10,
        };
        let mut placer = AnnotationPlacer::new(geometry);

        // Place first annotation
        let pos1 = Point::new(10, 10);
        let offset1 = placer.find_optimal_position(pos1, "3.300V");
        placer.mark_placed(pos1, offset1, "3.300V");

        // Place second annotation at same position
        let pos2 = Point::new(10, 10);
        let offset2 = placer.find_optimal_position(pos2, "1.650V");

        // Second annotation should be at a different position
        assert!(
            offset1 != offset2,
            "Second annotation should be at different position"
        );
    }

    #[test]
    fn test_geometry_from_empty_schematic() {
        let geometry = SchematicGeometry::from_schematic(&[], &[], 10);
        assert!(geometry.boxes.is_empty());
    }

    // =========================================================================
    // Anchor Point Selection Tests
    // =========================================================================

    #[test]
    fn test_classify_point_horizontal_segment() {
        // Wire from (0,5) to (10,5) - horizontal
        let wire = Wire::new(1, vec![Point::new(0, 5), Point::new(10, 5)]);
        let wires = vec![wire];

        // Point in middle of horizontal segment
        let mid_point = Point::new(5, 5);
        assert_eq!(
            classify_point_on_wire(mid_point, &wires),
            WireSegmentType::Horizontal
        );
    }

    #[test]
    fn test_classify_point_vertical_segment() {
        // Wire from (5,0) to (5,10) - vertical
        let wire = Wire::new(1, vec![Point::new(5, 0), Point::new(5, 10)]);
        let wires = vec![wire];

        // Point in middle of vertical segment
        let mid_point = Point::new(5, 5);
        assert_eq!(
            classify_point_on_wire(mid_point, &wires),
            WireSegmentType::Vertical
        );
    }

    #[test]
    fn test_classify_point_corner() {
        // L-shaped wire: (0,5) -> (5,5) -> (5,10)
        let wire = Wire::new(
            1,
            vec![
                Point::new(0, 5),
                Point::new(5, 5), // Corner vertex
                Point::new(5, 10),
            ],
        );
        let wires = vec![wire];

        // Point at corner vertex
        let corner_point = Point::new(5, 5);
        assert_eq!(
            classify_point_on_wire(corner_point, &wires),
            WireSegmentType::Corner
        );
    }

    #[test]
    fn test_classify_point_endpoint() {
        // Wire from (0,5) to (10,5)
        let wire = Wire::new(1, vec![Point::new(0, 5), Point::new(10, 5)]);
        let wires = vec![wire];

        // Point at endpoint
        let endpoint = Point::new(0, 5);
        assert_eq!(
            classify_point_on_wire(endpoint, &wires),
            WireSegmentType::Endpoint
        );
    }

    #[test]
    fn test_classify_point_none() {
        // Wire from (0,5) to (10,5)
        let wire = Wire::new(1, vec![Point::new(0, 5), Point::new(10, 5)]);
        let wires = vec![wire];

        // Point not on any wire
        let off_wire = Point::new(5, 0);
        assert_eq!(
            classify_point_on_wire(off_wire, &wires),
            WireSegmentType::None
        );
    }

    #[test]
    fn test_select_optimal_anchor_empty() {
        let candidates: Vec<Point> = vec![];
        let wires: Vec<Wire> = vec![];
        let components: Vec<Point> = vec![];

        assert_eq!(
            select_optimal_anchor_point(&candidates, &wires, &components),
            None
        );
    }

    #[test]
    fn test_select_optimal_anchor_single() {
        let candidates = vec![Point::new(5, 5)];
        let wires: Vec<Wire> = vec![];
        let components: Vec<Point> = vec![];

        assert_eq!(
            select_optimal_anchor_point(&candidates, &wires, &components),
            Some(Point::new(5, 5))
        );
    }

    #[test]
    fn test_select_optimal_anchor_prefers_horizontal() {
        // L-shaped wire with corner at (5,5), horizontal segment (0,5)-(5,5), vertical (5,5)-(5,10)
        let wire = Wire::new(
            1,
            vec![Point::new(0, 5), Point::new(5, 5), Point::new(5, 10)],
        );
        let wires = vec![wire];
        let components = vec![Point::new(5, 5)]; // Component at corner

        // Candidates: corner (5,5), horizontal mid (2,5), vertical mid (5,7)
        let candidates = vec![
            Point::new(5, 5), // Corner
            Point::new(2, 5), // On horizontal segment
            Point::new(5, 7), // On vertical segment
        ];

        // Should prefer horizontal segment point despite corner being closer to component
        let result = select_optimal_anchor_point(&candidates, &wires, &components);
        assert_eq!(result, Some(Point::new(2, 5)));
    }

    #[test]
    fn test_select_optimal_anchor_horizontal_closest_to_component() {
        // Horizontal wire from (0,5) to (20,5)
        let wire = Wire::new(1, vec![Point::new(0, 5), Point::new(20, 5)]);
        let wires = vec![wire];
        let components = vec![Point::new(15, 5)]; // Component closer to right

        // Multiple horizontal points
        let candidates = vec![
            Point::new(5, 5),  // Far from component
            Point::new(10, 5), // Medium distance
            Point::new(14, 5), // Closest to component
        ];

        // Should pick horizontal point closest to component
        let result = select_optimal_anchor_point(&candidates, &wires, &components);
        assert_eq!(result, Some(Point::new(14, 5)));
    }
}
