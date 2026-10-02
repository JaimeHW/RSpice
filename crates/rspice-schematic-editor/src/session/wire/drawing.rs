//! Uncommitted wire routing and pointer preview state.

use super::routing::WireRoutingMode;
use rspice_design_model::Point;
use serde::{Deserialize, Serialize};

/// Wire drawing state for interactive wire placement
///
/// Tracks the in-progress wire being drawn by the user.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WireDrawing {
    /// Points in the current wire being drawn (committed vertices)
    pub points: Vec<Point>,

    /// Whether currently drawing
    pub active: bool,

    /// Current mouse position for preview (grid-aligned)
    pub preview_pos: Option<Point>,

    /// Routing mode for orthogonal wires
    pub routing_mode: WireRoutingMode,
}

impl WireDrawing {
    /// Begin a route while preserving its configured routing mode.
    pub fn start(&mut self, pos: Point) {
        self.clear();
        self.points.push(pos);
        self.active = true;
    }

    /// Add a distinct point and any nondegenerate routing corner.
    /// Pointer motion owns preview updates separately from committed vertices.
    pub fn add_point(&mut self, pos: Point) {
        if !self.active {
            return;
        }

        if let Some(last) = self.points.last().copied() {
            if last == pos {
                return;
            }

            if let Some(corner) = self.get_route_corner(pos)
                && corner != last
                && corner != pos
            {
                self.points.push(corner);
            }

            self.points.push(pos);
        }
    }

    /// Update the preview position
    pub fn update_preview(&mut self, pos: Point) {
        if self.active {
            self.preview_pos = Some(pos);
        }
    }

    /// Get intermediate points for orthogonal routing from last point to target
    ///
    /// Returns the corner point for L-shaped routing.
    /// Returns None if points are already aligned (no corner needed).
    fn get_route_corner(&self, target: Point) -> Option<Point> {
        let last = self.points.last()?;
        if last.x == target.x || last.y == target.y {
            // Already aligned - no corner needed
            return None;
        }

        match self.routing_mode {
            WireRoutingMode::HorizontalFirst => {
                // Go horizontal first, then vertical
                Some(Point::new(target.x, last.y))
            }
            WireRoutingMode::VerticalFirst => {
                // Go vertical first, then horizontal
                Some(Point::new(last.x, target.y))
            }
            WireRoutingMode::Diagonal => {
                // No corner needed - direct line
                None
            }
            WireRoutingMode::FortyFiveDegree => {
                // Use the 45-degree routing: return first intermediate point
                let route = self.routing_mode.suggest_route(*last, target);
                if route.len() > 1 {
                    // Return the first intermediate point (before final target)
                    Some(route[0])
                } else {
                    None
                }
            }
        }
    }

    /// Get the full wire path including preview
    pub fn get_full_path(&self) -> Vec<Point> {
        let mut path = self.points.clone();

        if let Some(target) = self.preview_pos
            && let Some(&last) = self.points.last()
            && last != target
        {
            if let Some(corner) = self.get_route_corner(target) {
                path.push(corner);
            }
            path.push(target);
        }

        path
    }

    /// Set the routing mode
    pub fn set_routing_mode(&mut self, mode: WireRoutingMode) {
        self.routing_mode = mode;
    }

    /// Clear the wire drawing state
    pub fn clear(&mut self) {
        self.points.clear();
        self.active = false;
        self.preview_pos = None;
    }
}
