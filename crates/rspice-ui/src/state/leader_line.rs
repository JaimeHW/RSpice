//! Leader Line Geometry Calculations
//!
//! Geometry utilities for DC annotation leader lines.
//! Implements the behavior seen in standard simulators:
//!
//! 1. Leader line starts from the point on the label closest to the target
//! 2. Leader line terminates at the first wire intersection, not the node point
//!
//! # Algorithm Details
//!
//! ## Closest Anchor Point
//!
//! Given a rectangular label bounding box and a target point, we compute the
//! closest point on the rectangle's perimeter. This handles all cases:
//! - Target above label → anchor at top center
//! - Target below label → anchor at bottom center
//! - Target left of label → anchor at left center
//! - Target right of label → anchor at right center
//! - Diagonal targets → anchor at the nearest corner or edge
//!
//! ## Wire Intersection
//!
//! The leader line is clipped at the first intersection with any wire segment.
//! This prevents the visual clutter of lines crossing through the schematic.

use crate::state::{Point, Wire};

// =============================================================================
// Bounding Box for Leader Line Calculations
// =============================================================================

/// Rectangle representing a label's bounding box in pixel coordinates.
#[derive(Debug, Clone, Copy)]
pub struct LabelBounds {
    /// Left edge X coordinate
    pub left: f64,
    /// Right edge X coordinate
    pub right: f64,
    /// Top edge Y coordinate
    pub top: f64,
    /// Bottom edge Y coordinate
    pub bottom: f64,
}

impl LabelBounds {
    /// Create bounds from position and dimensions.
    ///
    /// The label is positioned with (x, y) at the bottom-left corner,
    /// extending rightward by width and upward by height.
    #[inline]
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            left: x,
            right: x + width,
            top: y - height, // SVG: y increases downward
            bottom: y,
        }
    }

    /// Compute the center point of the label.
    #[inline]
    pub fn center(&self) -> (f64, f64) {
        (
            (self.left + self.right) / 2.0,
            (self.top + self.bottom) / 2.0,
        )
    }
}

// =============================================================================
// Closest Anchor Point Calculation
// =============================================================================

/// Find the point on a rectangle's perimeter closest to a target point.
///
/// This implements the behavior where the leader line
/// originates from the optimal position on the label, not a fixed corner.
///
/// # Arguments
///
/// * `bounds` - The label's bounding box
/// * `target` - The target point (typically the node position)
///
/// # Returns
///
/// (x, y) coordinates of the closest point on the label's perimeter.
pub fn closest_anchor_point(bounds: &LabelBounds, target: (f64, f64)) -> (f64, f64) {
    let (tx, ty) = target;
    let (cx, cy) = bounds.center();

    // Determine which edge(s) the target is closest to based on relative position
    let dx = tx - cx;
    let dy = ty - cy;

    // Handle degenerate case: target at center
    if dx.abs() < 1e-9 && dy.abs() < 1e-9 {
        // Default to bottom center
        return ((bounds.left + bounds.right) / 2.0, bounds.bottom);
    }

    // Compute intersection of ray from center to target with rectangle edges
    // The ray equation: P = center + t * (target - center), t > 0
    // We find the smallest positive t that puts P on an edge

    let half_w = (bounds.right - bounds.left) / 2.0;
    let half_h = (bounds.bottom - bounds.top) / 2.0;

    // Calculate t for each edge
    let t_right = if dx > 0.0 { half_w / dx } else { f64::INFINITY };
    let t_left = if dx < 0.0 {
        -half_w / dx
    } else {
        f64::INFINITY
    };
    let t_bottom = if dy > 0.0 { half_h / dy } else { f64::INFINITY };
    let t_top = if dy < 0.0 {
        -half_h / dy
    } else {
        f64::INFINITY
    };

    // Find minimum positive t
    let t = t_right.min(t_left).min(t_bottom).min(t_top);

    if t.is_infinite() || t <= 0.0 {
        // Fallback: bottom center
        return ((bounds.left + bounds.right) / 2.0, bounds.bottom);
    }

    // Calculate intersection point
    let px = cx + t * dx;
    let py = cy + t * dy;

    // Clamp to bounds (handles numerical precision issues)
    (
        px.clamp(bounds.left, bounds.right),
        py.clamp(bounds.top, bounds.bottom),
    )
}

// =============================================================================
// Line-Segment Intersection
// =============================================================================

/// Compute intersection point between two line segments.
///
/// Uses parametric line representation for robust intersection detection.
/// Returns None if segments don't intersect or are parallel.
///
/// # Arguments
///
/// * `p1`, `p2` - First segment endpoints
/// * `p3`, `p4` - Second segment endpoints
///
/// # Returns
///
/// Some((x, y, t)) where (x, y) is the intersection point and t is the
/// parameter along the first segment (0.0 = at p1, 1.0 = at p2).
/// Returns None if no intersection.
fn segment_intersection(
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
    p4: (f64, f64),
) -> Option<(f64, f64, f64)> {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;
    let (x4, y4) = p4;

    // Direction vectors
    let d1x = x2 - x1;
    let d1y = y2 - y1;
    let d2x = x4 - x3;
    let d2y = y4 - y3;

    // Cross product of directions (determinant)
    let denom = d1x * d2y - d1y * d2x;

    // Parallel or collinear lines
    if denom.abs() < 1e-10 {
        return None;
    }

    // Vector from p1 to p3
    let dx = x3 - x1;
    let dy = y3 - y1;

    // Parameters for intersection
    let t = (dx * d2y - dy * d2x) / denom;
    let u = (dx * d1y - dy * d1x) / denom;

    // Check if intersection is within both segments
    // Use small epsilon for robustness at endpoints
    const EPS: f64 = 1e-9;
    if t >= -EPS && t <= 1.0 + EPS && u >= -EPS && u <= 1.0 + EPS {
        let ix = x1 + t * d1x;
        let iy = y1 + t * d1y;
        Some((ix, iy, t.clamp(0.0, 1.0)))
    } else {
        None
    }
}

// =============================================================================
// Wire Intersection Detection
// =============================================================================

/// Find the first intersection of a leader line with any wire.
///
/// This function searches through ALL wire segments to find where the leader
/// line first crosses any wire. The result is used to terminate the leader line
/// at the wire instead of extending past it. This is the behavior
/// seen in standard simulators where leader lines never cross wires.
///
/// # Arguments
///
/// * `line_start` - Start point of leader line (on label)
/// * `line_end` - Target end point (typically node position)
/// * `wires` - All wires in the schematic
/// * `grid_size` - Grid size for coordinate conversion
///
/// # Returns
///
/// The point where the leader line should terminate:
/// - If intersection found: the first intersection point
/// - If no intersection: the original line_end
pub fn find_wire_intersection(
    line_start: (f64, f64),
    line_end: (f64, f64),
    wires: &[Wire],
    grid_size: i32,
) -> (f64, f64) {
    if wires.is_empty() {
        return line_end;
    }

    let gs = grid_size as f64;
    let mut closest_intersection: Option<(f64, f64, f64)> = None;

    // Check ALL wire segments for intersection
    // The leader line should stop at the first wire it crosses
    for wire in wires {
        for segment in wire.points.windows(2) {
            let p1 = (segment[0].x as f64 * gs, segment[0].y as f64 * gs);
            let p2 = (segment[1].x as f64 * gs, segment[1].y as f64 * gs);

            if let Some((ix, iy, t)) = segment_intersection(line_start, line_end, p1, p2) {
                // Skip intersections very close to the start (within label area)
                // This prevents false positives when label overlaps wire slightly
                if t < 0.02 {
                    continue;
                }

                // Track the closest intersection (smallest t = earliest along the line)
                match closest_intersection {
                    None => closest_intersection = Some((ix, iy, t)),
                    Some((_, _, prev_t)) if t < prev_t => {
                        closest_intersection = Some((ix, iy, t));
                    }
                    _ => {}
                }
            }
        }
    }

    // Return intersection point or original end point
    closest_intersection
        .map(|(x, y, _)| (x, y))
        .unwrap_or(line_end)
}

/// Check if a point lies on a wire segment (grid coordinates).
fn point_on_wire_segment(p: Point, a: Point, b: Point) -> bool {
    // Horizontal segment
    if a.y == b.y && p.y == a.y {
        let (min_x, max_x) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
        return p.x >= min_x && p.x <= max_x;
    }
    // Vertical segment
    if a.x == b.x && p.x == a.x {
        let (min_y, max_y) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };
        return p.y >= min_y && p.y <= max_y;
    }
    false
}

// =============================================================================
// High-Level API
// =============================================================================

/// Compute optimal leader line endpoints for a DC annotation.
///
/// This is the main entry point for the leader line calculation, implementing
/// the full behavior:
///
/// 1. Start point: closest point on label perimeter to the node
/// 2. End point: first intersection with a wire, or node position if no intersection
///
/// # Arguments
///
/// * `label_bounds` - Bounding box of the annotation label
/// * `node_pos` - Target node position in pixels
/// * `wires` - All wires in the schematic
/// * `grid_size` - Grid size for coordinate conversion
///
/// # Returns
///
/// Tuple of (start_x, start_y, end_x, end_y) for the leader line.
pub fn compute_leader_line(
    label_bounds: &LabelBounds,
    node_pos: (f64, f64),
    wires: &[Wire],
    grid_size: i32,
) -> (f64, f64, f64, f64) {
    // Step 1: Find closest anchor point on label
    let start = closest_anchor_point(label_bounds, node_pos);

    // Step 2: Find wire intersection (or use node_pos as fallback)
    let end = find_wire_intersection(start, node_pos, wires, grid_size);

    (start.0, start.1, end.0, end.1)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_bounds_new() {
        let bounds = LabelBounds::new(100.0, 50.0, 80.0, 20.0);
        assert_eq!(bounds.left, 100.0);
        assert_eq!(bounds.right, 180.0);
        assert_eq!(bounds.top, 30.0);
        assert_eq!(bounds.bottom, 50.0);
    }

    #[test]
    fn test_label_bounds_center() {
        let bounds = LabelBounds::new(100.0, 50.0, 80.0, 20.0);
        let (cx, cy) = bounds.center();
        assert!((cx - 140.0).abs() < 0.001);
        assert!((cy - 40.0).abs() < 0.001);
    }

    #[test]
    fn test_closest_anchor_target_below() {
        // Label at (100, 50) with width 80, height 20
        // Target directly below at (140, 100)
        let bounds = LabelBounds::new(100.0, 50.0, 80.0, 20.0);
        let target = (140.0, 100.0);
        let (ax, ay) = closest_anchor_point(&bounds, target);

        // Should be at bottom center (140, 50)
        assert!((ax - 140.0).abs() < 0.001, "Expected x=140, got {}", ax);
        assert!((ay - 50.0).abs() < 0.001, "Expected y=50, got {}", ay);
    }

    #[test]
    fn test_closest_anchor_target_above() {
        let bounds = LabelBounds::new(100.0, 50.0, 80.0, 20.0);
        let target = (140.0, 0.0); // Target above

        let (ax, ay) = closest_anchor_point(&bounds, target);

        // Should be at top center (140, 30)
        assert!((ax - 140.0).abs() < 0.001, "Expected x=140, got {}", ax);
        assert!((ay - 30.0).abs() < 0.001, "Expected y=30, got {}", ay);
    }

    #[test]
    fn test_closest_anchor_target_right() {
        let bounds = LabelBounds::new(100.0, 50.0, 80.0, 20.0);
        let target = (250.0, 40.0); // Target to the right

        let (ax, ay) = closest_anchor_point(&bounds, target);

        // Should be at right center (180, 40)
        assert!((ax - 180.0).abs() < 0.001, "Expected x=180, got {}", ax);
        assert!((ay - 40.0).abs() < 0.001, "Expected y=40, got {}", ay);
    }

    #[test]
    fn test_closest_anchor_target_left() {
        let bounds = LabelBounds::new(100.0, 50.0, 80.0, 20.0);
        let target = (10.0, 40.0); // Target to the left

        let (ax, ay) = closest_anchor_point(&bounds, target);

        // Should be at left center (100, 40)
        assert!((ax - 100.0).abs() < 0.001, "Expected x=100, got {}", ax);
        assert!((ay - 40.0).abs() < 0.001, "Expected y=40, got {}", ay);
    }

    #[test]
    fn test_closest_anchor_diagonal() {
        let bounds = LabelBounds::new(100.0, 50.0, 80.0, 20.0);
        let target = (200.0, 100.0); // Diagonal: bottom-right

        let (ax, ay) = closest_anchor_point(&bounds, target);

        // Should hit either bottom or right edge at corner region
        assert!(ax >= 100.0 && ax <= 180.0);
        assert!(ay >= 30.0 && ay <= 50.0);
    }

    #[test]
    fn test_segment_intersection_cross() {
        // Vertical segment from (50, 0) to (50, 100)
        // Horizontal segment from (0, 50) to (100, 50)
        let result = segment_intersection((50.0, 0.0), (50.0, 100.0), (0.0, 50.0), (100.0, 50.0));

        assert!(result.is_some());
        let (ix, iy, _t) = result.unwrap();
        assert!((ix - 50.0).abs() < 0.001);
        assert!((iy - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_segment_intersection_parallel() {
        // Two parallel horizontal segments
        let result = segment_intersection((0.0, 0.0), (100.0, 0.0), (0.0, 10.0), (100.0, 10.0));
        assert!(result.is_none());
    }

    #[test]
    fn test_segment_intersection_no_overlap() {
        // Segments that would intersect if extended, but don't actually touch
        let result = segment_intersection((0.0, 0.0), (10.0, 10.0), (20.0, 0.0), (30.0, 10.0));
        assert!(result.is_none());
    }

    #[test]
    fn test_wire_intersection_horizontal() {
        // Leader line from (50, 50) to (150, 100)
        // Horizontal wire from (100, 75) to (200, 75)
        let wires = vec![Wire::new(1, vec![Point::new(10, 7), Point::new(20, 7)])];
        let grid_size = 10;

        // Node at (15, 7) in grid coords = (150, 70) in pixels
        let line_start = (50.0, 50.0);
        let node_pos = (150.0, 70.0);

        let result = find_wire_intersection(line_start, node_pos, &wires, grid_size);

        // Should find intersection with wire
        assert!(result.0 >= 100.0 && result.0 <= 150.0);
    }

    #[test]
    fn test_wire_intersection_no_wire() {
        let wires: Vec<Wire> = vec![];
        let line_start = (50.0, 50.0);
        let node_pos = (150.0, 100.0);

        let result = find_wire_intersection(line_start, node_pos, &wires, 10);

        // Should return node_pos unchanged
        assert!((result.0 - 150.0).abs() < 0.001);
        assert!((result.1 - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_compute_leader_line_full() {
        let bounds = LabelBounds::new(100.0, 50.0, 80.0, 20.0);
        let node_pos = (140.0, 100.0); // Below the label
        let wires: Vec<Wire> = vec![]; // No wires for simple test

        let (sx, sy, ex, ey) = compute_leader_line(&bounds, node_pos, &wires, 10);

        // Start should be at bottom center of label
        assert!((sx - 140.0).abs() < 0.001);
        assert!((sy - 50.0).abs() < 0.001);

        // End should be at node (no wires to intersect)
        assert!((ex - 140.0).abs() < 0.001);
        assert!((ey - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_point_on_wire_segment_horizontal() {
        assert!(point_on_wire_segment(
            Point::new(5, 10),
            Point::new(0, 10),
            Point::new(10, 10)
        ));
        assert!(!point_on_wire_segment(
            Point::new(15, 10),
            Point::new(0, 10),
            Point::new(10, 10)
        ));
    }

    #[test]
    fn test_point_on_wire_segment_vertical() {
        assert!(point_on_wire_segment(
            Point::new(5, 5),
            Point::new(5, 0),
            Point::new(5, 10)
        ));
        assert!(!point_on_wire_segment(
            Point::new(5, 15),
            Point::new(5, 0),
            Point::new(5, 10)
        ));
    }
}
