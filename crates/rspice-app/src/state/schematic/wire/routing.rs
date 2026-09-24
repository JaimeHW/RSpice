//! Wire Routing Mode and Utilities
//!
//! Wire routing algorithms and route optimization functions for
//! interactive wire drawing in schematic editors.

use super::super::point::Point;
use serde::{Deserialize, Serialize};

// =============================================================================
// Wire Routing Mode
// =============================================================================

/// Wire routing mode for drawing
///
/// Controls how the cursor position is connected to the last wire point
/// when drawing wires interactively. Professional EDA tools like Cadence
/// and Altium support multiple routing modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WireRoutingMode {
    /// Horizontal first, then vertical (L-shape: →↓)
    /// Standard orthogonal routing with X movement before Y
    #[default]
    HorizontalFirst,

    /// Vertical first, then horizontal (inverted L-shape: ↓→)
    /// Standard orthogonal routing with Y movement before X
    VerticalFirst,

    /// Direct diagonal connection (line from start to end)
    /// Allows any angle, not just orthogonal or 45°
    Diagonal,

    /// 45-degree routing mode (octagonal)
    /// Routes in H/V plus 45° diagonal segments
    /// Similar to what PCB routing tools use
    FortyFiveDegree,
}

impl WireRoutingMode {
    /// Toggle between routing modes
    ///
    /// Cycles: HorizontalFirst -> VerticalFirst -> Diagonal -> FortyFiveDegree -> HorizontalFirst
    pub fn toggle(self) -> Self {
        match self {
            WireRoutingMode::HorizontalFirst => WireRoutingMode::VerticalFirst,
            WireRoutingMode::VerticalFirst => WireRoutingMode::Diagonal,
            WireRoutingMode::Diagonal => WireRoutingMode::FortyFiveDegree,
            WireRoutingMode::FortyFiveDegree => WireRoutingMode::HorizontalFirst,
        }
    }

    /// Toggle only between orthogonal modes (for compatibility)
    pub fn toggle_orthogonal(self) -> Self {
        match self {
            WireRoutingMode::HorizontalFirst => WireRoutingMode::VerticalFirst,
            WireRoutingMode::VerticalFirst => WireRoutingMode::HorizontalFirst,
            // Non-orthogonal modes default to HorizontalFirst
            _ => WireRoutingMode::HorizontalFirst,
        }
    }

    /// Check if this mode is orthogonal-only (no diagonals)
    pub fn is_orthogonal(&self) -> bool {
        matches!(
            self,
            WireRoutingMode::HorizontalFirst | WireRoutingMode::VerticalFirst
        )
    }

    /// Check if this mode allows diagonal segments
    pub fn allows_diagonal(&self) -> bool {
        matches!(
            self,
            WireRoutingMode::Diagonal | WireRoutingMode::FortyFiveDegree
        )
    }

    /// Get a human-readable name for this mode
    pub fn display_name(&self) -> &'static str {
        match self {
            WireRoutingMode::HorizontalFirst => "Orthogonal (H→V)",
            WireRoutingMode::VerticalFirst => "Orthogonal (V→H)",
            WireRoutingMode::Diagonal => "Diagonal",
            WireRoutingMode::FortyFiveDegree => "45° Routing",
        }
    }

    /// Get a short keyboard hint for this mode
    pub fn keyboard_hint(&self) -> &'static str {
        match self {
            WireRoutingMode::HorizontalFirst => "H→V",
            WireRoutingMode::VerticalFirst => "V→H",
            WireRoutingMode::Diagonal => "Diag",
            WireRoutingMode::FortyFiveDegree => "45°",
        }
    }

    /// Suggest a route from one point to another
    ///
    /// Returns a list of intermediate points (excluding start, including end).
    /// The result depends on the routing mode.
    pub fn suggest_route(&self, from: Point, to: Point) -> Vec<Point> {
        if from == to {
            return vec![];
        }

        match self {
            WireRoutingMode::HorizontalFirst => {
                if from.x == to.x || from.y == to.y {
                    vec![to]
                } else {
                    // L-shape: horizontal first
                    vec![Point::new(to.x, from.y), to]
                }
            }
            WireRoutingMode::VerticalFirst => {
                if from.x == to.x || from.y == to.y {
                    vec![to]
                } else {
                    // L-shape: vertical first
                    vec![Point::new(from.x, to.y), to]
                }
            }
            WireRoutingMode::Diagonal => {
                // Direct line
                vec![to]
            }
            WireRoutingMode::FortyFiveDegree => Self::suggest_45_degree_route(from, to),
        }
    }

    /// Suggest a 45-degree route between two points
    ///
    /// Creates a route using only horizontal, vertical, and 45° diagonal segments.
    fn suggest_45_degree_route(from: Point, to: Point) -> Vec<Point> {
        let dx = (to.x - from.x).abs();
        let dy = (to.y - from.y).abs();

        if dx == 0 || dy == 0 || dx == dy {
            // Already aligned (H/V/45°)
            return vec![to];
        }

        // Route: orthogonal segment, then 45° diagonal
        if dx > dy {
            // More horizontal: go H first for 'remaining', then 45° diagonal
            let mid_x = if to.x > from.x {
                from.x + (dx - dy)
            } else {
                from.x - (dx - dy)
            };
            let mid = Point::new(mid_x, from.y);
            vec![mid, to]
        } else {
            // More vertical: go V first for 'remaining', then 45° diagonal
            let mid_y = if to.y > from.y {
                from.y + (dy - dx)
            } else {
                from.y - (dy - dx)
            };
            let mid = Point::new(from.x, mid_y);
            vec![mid, to]
        }
    }

    /// All routing modes in order
    pub const ALL: [WireRoutingMode; 4] = [
        WireRoutingMode::HorizontalFirst,
        WireRoutingMode::VerticalFirst,
        WireRoutingMode::Diagonal,
        WireRoutingMode::FortyFiveDegree,
    ];
}

// =============================================================================
// Tests
// =============================================================================
