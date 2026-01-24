//! Mouse Event Handlers for Schematic Editor
//!
//! This module provides testable mouse handling logic for the schematic editor.
//! The design separates pure calculation logic from UI event handling.
//!
//! ## Architecture
//!
//! 1. **Coordinate conversion** - Transform screen to grid coordinates
//! 2. **Hit detection** - Find elements at cursor position  
//! 3. **Net tracing** - Flood-fill connected wire nets
//! 4. **Drag handling** - Calculate movement deltas

use crate::state::{Point, Wire};
use std::collections::HashSet;

/// Result of a mouse hit detection query
#[derive(Debug, Clone, Default)]
pub struct HitTestResult {
    /// ID of component at cursor, if any
    pub component_id: Option<u64>,
    /// ID of wire at cursor, if any
    pub wire_id: Option<u64>,
    /// Grid point if cursor is on a wire junction/endpoint
    pub junction_point: Option<Point>,
    /// Number of wire endpoints at this position
    pub endpoint_count: usize,
}

impl HitTestResult {
    /// Returns true if no element was hit
    pub fn is_empty(&self) -> bool {
        self.component_id.is_none() && self.wire_id.is_none() && self.junction_point.is_none()
    }

    /// Returns true if a junction (2+ wire endpoints) was hit
    pub fn is_junction(&self) -> bool {
        self.endpoint_count >= 2
    }
}

/// Convert screen pixel coordinates to grid coordinates
///
/// # Arguments
/// * `screen_x` - X position in screen pixels
/// * `screen_y` - Y position in screen pixels
/// * `pan` - Current pan offset (px, py)
/// * `zoom` - Current zoom level
/// * `grid_size` - Grid size in pixels
///
/// # Returns
/// Grid point corresponding to screen position
pub fn screen_to_grid(
    screen_x: f64,
    screen_y: f64,
    pan: (f64, f64),
    zoom: f64,
    grid_size: i32,
) -> Point {
    let (px, py) = pan;
    let world_x = (screen_x - px) / zoom;
    let world_y = (screen_y - py) / zoom;
    Point::from_pixels(world_x, world_y, grid_size)
}

/// Convert grid coordinates to screen pixel coordinates
///
/// # Arguments
/// * `grid_point` - Grid point
/// * `pan` - Current pan offset (px, py)
/// * `zoom` - Current zoom level
/// * `grid_size` - Grid size in pixels
///
/// # Returns
/// Screen (x, y) position in pixels
pub fn grid_to_screen(grid_point: Point, pan: (f64, f64), zoom: f64, grid_size: i32) -> (f64, f64) {
    let (px, py) = pan;
    let (world_x, world_y) = grid_point.to_pixels(grid_size);
    (world_x * zoom + px, world_y * zoom + py)
}

/// Calculate movement delta in grid units for a drag operation
///
/// # Arguments
/// * `start_grid` - Starting grid position of drag
/// * `current_grid` - Current grid position during drag
///
/// # Returns
/// (delta_x, delta_y) in grid units
pub fn calculate_drag_delta(start_grid: Point, current_grid: Point) -> (i32, i32) {
    (current_grid.x - start_grid.x, current_grid.y - start_grid.y)
}

/// Find all wires connected to a given wire via shared endpoints (flood-fill)
///
/// This is used for:
/// - Net highlighting in probe mode
/// - Finding entire nets for selection
/// - Net label propagation
///
/// # Arguments
/// * `start_wire_id` - ID of wire to start from
/// * `wires` - All wires in schematic
///
/// # Returns
/// Set of all wire IDs in the same net
pub fn trace_connected_net(start_wire_id: u64, wires: &[Wire]) -> HashSet<u64> {
    let mut connected = HashSet::new();
    let mut to_visit = vec![start_wire_id];

    while let Some(wid) = to_visit.pop() {
        if connected.contains(&wid) {
            continue;
        }
        connected.insert(wid);

        // Find this wire's endpoints
        if let Some(wire) = wires.iter().find(|w| w.id == wid) {
            for endpoint in [wire.points.first(), wire.points.last()]
                .into_iter()
                .flatten()
            {
                // Find other wires sharing this endpoint
                for other in wires {
                    if connected.contains(&other.id) {
                        continue;
                    }
                    if other.points.contains(endpoint) {
                        to_visit.push(other.id);
                    }
                }
            }
        }
    }

    connected
}

/// Find all grid points that are wire junctions (3+ connections) or endpoints
///
/// # Arguments
/// * `wires` - All wires in schematic
///
/// # Returns
/// Map of grid points to count of connections at that point
pub fn find_all_junctions(wires: &[Wire]) -> std::collections::HashMap<Point, usize> {
    let mut point_counts: std::collections::HashMap<Point, usize> =
        std::collections::HashMap::new();

    for wire in wires {
        // Count endpoints
        if let Some(first) = wire.points.first() {
            *point_counts.entry(*first).or_insert(0) += 1;
        }
        if let Some(last) = wire.points.last() {
            if wire.points.len() > 1 {
                *point_counts.entry(*last).or_insert(0) += 1;
            }
        }
    }

    point_counts
}

/// Determine the cursor style based on current tool and context
///
/// # Arguments
/// * `tool_name` - Current tool ("select", "wire", "probe", "place", "label")
/// * `is_over_draggable` - Whether cursor is over something draggable
///
/// # Returns
/// CSS cursor style string
pub fn cursor_for_context(tool_name: &str, is_over_draggable: bool) -> &'static str {
    match tool_name {
        "probe" => "crosshair",
        "wire" => "crosshair",
        "place" => "copy",
        "label" => "text",
        "select" if is_over_draggable => "move",
        _ => "default",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =============================================================================
    // Coordinate Conversion Tests
    // =============================================================================

    #[test]
    fn test_screen_to_grid_no_pan_no_zoom() {
        let grid = screen_to_grid(50.0, 50.0, (0.0, 0.0), 1.0, 10);
        assert_eq!(grid, Point::new(5, 5));
    }

    #[test]
    fn test_screen_to_grid_with_pan() {
        let grid = screen_to_grid(50.0, 50.0, (20.0, 20.0), 1.0, 10);
        assert_eq!(grid, Point::new(3, 3));
    }

    #[test]
    fn test_screen_to_grid_with_zoom() {
        let grid = screen_to_grid(100.0, 100.0, (0.0, 0.0), 2.0, 10);
        assert_eq!(grid, Point::new(5, 5));
    }

    #[test]
    fn test_screen_to_grid_with_pan_and_zoom() {
        let grid = screen_to_grid(120.0, 120.0, (20.0, 20.0), 2.0, 10);
        assert_eq!(grid, Point::new(5, 5));
    }

    #[test]
    fn test_grid_to_screen_no_pan_no_zoom() {
        let (sx, sy) = grid_to_screen(Point::new(5, 5), (0.0, 0.0), 1.0, 10);
        assert_eq!(sx, 50.0);
        assert_eq!(sy, 50.0);
    }

    #[test]
    fn test_grid_to_screen_with_pan() {
        let (sx, sy) = grid_to_screen(Point::new(5, 5), (10.0, 20.0), 1.0, 10);
        assert_eq!(sx, 60.0);
        assert_eq!(sy, 70.0);
    }

    #[test]
    fn test_grid_to_screen_with_zoom() {
        let (sx, sy) = grid_to_screen(Point::new(5, 5), (0.0, 0.0), 2.0, 10);
        assert_eq!(sx, 100.0);
        assert_eq!(sy, 100.0);
    }

    // =============================================================================
    // Drag Delta Tests
    // =============================================================================

    #[test]
    fn test_drag_delta_positive() {
        let (dx, dy) = calculate_drag_delta(Point::new(0, 0), Point::new(3, 5));
        assert_eq!(dx, 3);
        assert_eq!(dy, 5);
    }

    #[test]
    fn test_drag_delta_negative() {
        let (dx, dy) = calculate_drag_delta(Point::new(5, 5), Point::new(2, 1));
        assert_eq!(dx, -3);
        assert_eq!(dy, -4);
    }

    #[test]
    fn test_drag_delta_zero() {
        let (dx, dy) = calculate_drag_delta(Point::new(3, 4), Point::new(3, 4));
        assert_eq!(dx, 0);
        assert_eq!(dy, 0);
    }

    // =============================================================================
    // Net Tracing Tests
    // =============================================================================

    #[test]
    fn test_trace_single_wire() {
        let wires = vec![Wire::new(1, vec![Point::new(0, 0), Point::new(5, 0)])];
        let result = trace_connected_net(1, &wires);
        assert_eq!(result.len(), 1);
        assert!(result.contains(&1));
    }

    #[test]
    fn test_trace_two_connected_wires() {
        let wires = vec![
            Wire::new(1, vec![Point::new(0, 0), Point::new(5, 0)]),
            Wire::new(2, vec![Point::new(5, 0), Point::new(5, 5)]),
        ];
        let result = trace_connected_net(1, &wires);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
    }

    #[test]
    fn test_trace_three_wires_chain() {
        let wires = vec![
            Wire::new(1, vec![Point::new(0, 0), Point::new(5, 0)]),
            Wire::new(2, vec![Point::new(5, 0), Point::new(5, 5)]),
            Wire::new(3, vec![Point::new(5, 5), Point::new(10, 5)]),
        ];
        let result = trace_connected_net(1, &wires);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_trace_disconnected_wires() {
        let wires = vec![
            Wire::new(1, vec![Point::new(0, 0), Point::new(5, 0)]),
            Wire::new(2, vec![Point::new(10, 10), Point::new(15, 10)]),
        ];
        let result = trace_connected_net(1, &wires);
        assert_eq!(result.len(), 1);
        assert!(result.contains(&1));
        assert!(!result.contains(&2));
    }

    #[test]
    fn test_trace_star_topology() {
        // Central junction with 4 wires meeting
        let wires = vec![
            Wire::new(1, vec![Point::new(0, 5), Point::new(5, 5)]),
            Wire::new(2, vec![Point::new(5, 5), Point::new(10, 5)]),
            Wire::new(3, vec![Point::new(5, 0), Point::new(5, 5)]),
            Wire::new(4, vec![Point::new(5, 5), Point::new(5, 10)]),
        ];
        let result = trace_connected_net(1, &wires);
        assert_eq!(result.len(), 4);
    }

    // =============================================================================
    // Junction Detection Tests
    // =============================================================================

    #[test]
    fn test_find_junctions_single_wire() {
        let wires = vec![Wire::new(1, vec![Point::new(0, 0), Point::new(5, 0)])];
        let junctions = find_all_junctions(&wires);
        assert_eq!(junctions.get(&Point::new(0, 0)), Some(&1));
        assert_eq!(junctions.get(&Point::new(5, 0)), Some(&1));
    }

    #[test]
    fn test_find_junctions_t_junction() {
        // Two wires meeting at (5,5): one horizontal ending at (5,5), one vertical ending at (5,5)
        let wires = vec![
            Wire::new(1, vec![Point::new(0, 5), Point::new(5, 5)]),
            Wire::new(2, vec![Point::new(5, 0), Point::new(5, 5)]),
        ];
        let junctions = find_all_junctions(&wires);
        // Point (5,5) should have 2 connections (both wires end here)
        assert_eq!(junctions.get(&Point::new(5, 5)), Some(&2));
    }

    #[test]
    fn test_find_junctions_cross() {
        // Four wires meeting at (5,5)
        let wires = vec![
            Wire::new(1, vec![Point::new(0, 5), Point::new(5, 5)]),
            Wire::new(2, vec![Point::new(5, 5), Point::new(10, 5)]),
            Wire::new(3, vec![Point::new(5, 0), Point::new(5, 5)]),
            Wire::new(4, vec![Point::new(5, 5), Point::new(5, 10)]),
        ];
        let junctions = find_all_junctions(&wires);
        assert_eq!(junctions.get(&Point::new(5, 5)), Some(&4));
    }

    // =============================================================================
    // Cursor Style Tests
    // =============================================================================

    #[test]
    fn test_cursor_probe_mode() {
        assert_eq!(cursor_for_context("probe", false), "crosshair");
        assert_eq!(cursor_for_context("probe", true), "crosshair");
    }

    #[test]
    fn test_cursor_wire_mode() {
        assert_eq!(cursor_for_context("wire", false), "crosshair");
    }

    #[test]
    fn test_cursor_place_mode() {
        assert_eq!(cursor_for_context("place", false), "copy");
    }

    #[test]
    fn test_cursor_select_draggable() {
        assert_eq!(cursor_for_context("select", true), "move");
        assert_eq!(cursor_for_context("select", false), "default");
    }

    #[test]
    fn test_cursor_label_mode() {
        assert_eq!(cursor_for_context("label", false), "text");
    }

    // =============================================================================
    // HitTestResult Tests
    // =============================================================================

    #[test]
    fn test_hit_result_is_empty() {
        let result = HitTestResult::default();
        assert!(result.is_empty());
    }

    #[test]
    fn test_hit_result_with_component() {
        let result = HitTestResult {
            component_id: Some(42),
            ..Default::default()
        };
        assert!(!result.is_empty());
    }

    #[test]
    fn test_hit_result_is_junction() {
        let result = HitTestResult {
            endpoint_count: 3,
            junction_point: Some(Point::new(5, 5)),
            ..Default::default()
        };
        assert!(result.is_junction());
    }

    #[test]
    fn test_hit_result_is_not_junction() {
        let result = HitTestResult {
            endpoint_count: 1,
            ..Default::default()
        };
        assert!(!result.is_junction());
    }
}
