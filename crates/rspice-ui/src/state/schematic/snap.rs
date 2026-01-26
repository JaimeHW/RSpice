//! Snap System Module
//!
//! Commercial-grade snapping infrastructure for wire endpoints.
//!
//! This module provides the foundation for intelligent cursor snapping in
//! schematic editors, matching behavior of professional EDA tools like:
//! - Cadence Allegro
//! - Altium Designer
//! - KiCad
//!
//! # Architecture
//!
//! The snap system uses a priority-based target selection:
//! 1. Component terminals (highest priority)
//! 2. Wire endpoints
//! 3. Junction points
//! 4. Wire segment midpoints
//! 5. Grid intersections (lowest priority)
//!
//! # Usage
//!
//! ```ignore
//! let engine = SnapEngine::default();
//! let result = engine.find_snap_target(cursor_pos, &schematic);
//! if result.show_indicator {
//!     draw_snap_indicator(result.snapped_position);
//! }
//! ```

use super::{Component, Junction, Point, Wire};
use serde::{Deserialize, Serialize};

// =============================================================================
// SnapTargetType - What the snap target represents
// =============================================================================

/// Type of snap target
///
/// Identifies what kind of schematic element a snap target represents.
/// This is used for:
/// - Visual feedback (different indicators for different types)
/// - Connection semantics (terminals create electrical connections)
/// - Priority ordering (terminals > wire points > grid)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SnapTargetType {
    /// Component terminal (e.g., resistor pin, transistor gate)
    ///
    /// Highest priority - these are the primary connection points.
    /// Stores the component ID and terminal name for netlist generation.
    Terminal {
        /// ID of the component owning this terminal
        component_id: u64,
        /// Name of the terminal (e.g., "+", "-", "B", "C", "E")
        terminal_name: String,
    },

    /// Wire endpoint (start or end of a wire)
    ///
    /// Second priority - allows connecting wires together.
    WireEndpoint {
        /// ID of the wire
        wire_id: u64,
        /// Whether this is the start (true) or end (false) endpoint
        is_start: bool,
    },

    /// Wire segment (anywhere along a wire segment)
    ///
    /// Medium priority - allows tapping into existing wires.
    WireSegment {
        /// ID of the wire
        wire_id: u64,
        /// Index of the segment (between points[index] and points[index+1])
        segment_index: usize,
    },

    /// Junction point (explicit electrical connection)
    ///
    /// Medium-high priority - marks electrical connection points.
    Junction,

    /// Grid intersection
    ///
    /// Lowest priority - fallback when no other targets are nearby.
    Grid,
}

impl SnapTargetType {
    /// Get the priority of this target type (lower = higher priority)
    ///
    /// Priority order:
    /// 1. Terminal (0) - Always prefer component pins
    /// 2. Junction (1) - Explicit connection points
    /// 3. WireEndpoint (2) - Wire connection points
    /// 4. WireSegment (3) - Mid-wire tap points
    /// 5. Grid (4) - Fallback
    pub fn priority(&self) -> u8 {
        match self {
            SnapTargetType::Terminal { .. } => 0,
            SnapTargetType::Junction => 1,
            SnapTargetType::WireEndpoint { .. } => 2,
            SnapTargetType::WireSegment { .. } => 3,
            SnapTargetType::Grid => 4,
        }
    }

    /// Check if this is a terminal snap
    pub fn is_terminal(&self) -> bool {
        matches!(self, SnapTargetType::Terminal { .. })
    }

    /// Check if this is a wire-related snap
    pub fn is_wire_related(&self) -> bool {
        matches!(
            self,
            SnapTargetType::WireEndpoint { .. } | SnapTargetType::WireSegment { .. }
        )
    }

    /// Check if this creates an electrical connection
    pub fn creates_connection(&self) -> bool {
        !matches!(self, SnapTargetType::Grid)
    }
}

// =============================================================================
// SnapTarget - A snap-able point in the schematic
// =============================================================================

/// A snap target with position and type
///
/// Represents a point in the schematic that the cursor can snap to.
/// Contains all information needed for:
/// - Positioning (where to snap)
/// - Visual feedback (what kind of indicator to show)
/// - Connection creation (what to connect to)
#[derive(Debug, Clone, PartialEq)]
pub struct SnapTarget {
    /// Position of the snap target in grid coordinates
    pub position: Point,

    /// Type of the snap target
    pub target_type: SnapTargetType,

    /// Distance from the query point (used for ranking)
    pub distance: f64,
}

impl SnapTarget {
    /// Create a new snap target
    pub fn new(position: Point, target_type: SnapTargetType, distance: f64) -> Self {
        Self {
            position,
            target_type,
            distance,
        }
    }

    /// Create a terminal snap target
    pub fn terminal(
        position: Point,
        component_id: u64,
        terminal_name: &str,
        distance: f64,
    ) -> Self {
        Self {
            position,
            target_type: SnapTargetType::Terminal {
                component_id,
                terminal_name: terminal_name.to_string(),
            },
            distance,
        }
    }

    /// Create a wire endpoint snap target
    pub fn wire_endpoint(position: Point, wire_id: u64, is_start: bool, distance: f64) -> Self {
        Self {
            position,
            target_type: SnapTargetType::WireEndpoint { wire_id, is_start },
            distance,
        }
    }

    /// Create a wire segment snap target
    pub fn wire_segment(
        position: Point,
        wire_id: u64,
        segment_index: usize,
        distance: f64,
    ) -> Self {
        Self {
            position,
            target_type: SnapTargetType::WireSegment {
                wire_id,
                segment_index,
            },
            distance,
        }
    }

    /// Create a junction snap target
    pub fn junction(position: Point, distance: f64) -> Self {
        Self {
            position,
            target_type: SnapTargetType::Junction,
            distance,
        }
    }

    /// Create a grid snap target
    pub fn grid(position: Point, distance: f64) -> Self {
        Self {
            position,
            target_type: SnapTargetType::Grid,
            distance,
        }
    }

    /// Get the priority of this target (lower = higher priority)
    pub fn priority(&self) -> u8 {
        self.target_type.priority()
    }

    /// Compare two targets for selection
    ///
    /// Returns true if self should be preferred over other.
    /// Uses priority first, then distance as tiebreaker.
    pub fn is_better_than(&self, other: &SnapTarget) -> bool {
        if self.priority() != other.priority() {
            self.priority() < other.priority()
        } else {
            self.distance < other.distance
        }
    }
}

// =============================================================================
// SnapResult - Result of a snap operation
// =============================================================================

/// Result of a snap operation
///
/// Contains the snapped position and optional target information.
/// Used by the UI to:
/// - Position drawing operations
/// - Show visual feedback
/// - Create connections when wire is completed
#[derive(Debug, Clone, PartialEq)]
pub struct SnapResult {
    /// The target that was snapped to (None if only grid snap)
    pub target: Option<SnapTarget>,

    /// The final snapped position
    pub snapped_position: Point,

    /// Whether to show a snap indicator
    ///
    /// True when snapping to a meaningful target (not just grid).
    pub show_indicator: bool,

    /// Original query position (for debugging/UI)
    pub original_position: Point,
}

impl SnapResult {
    /// Create a result with a target
    pub fn with_target(target: SnapTarget, original: Point) -> Self {
        let show = !matches!(target.target_type, SnapTargetType::Grid);
        let pos = target.position;
        Self {
            target: Some(target),
            snapped_position: pos,
            show_indicator: show,
            original_position: original,
        }
    }

    /// Create a grid-only snap result
    pub fn grid_only(snapped: Point, original: Point) -> Self {
        Self {
            target: None,
            snapped_position: snapped,
            show_indicator: false,
            original_position: original,
        }
    }

    /// Create a no-snap result (snapping disabled)
    pub fn no_snap(original: Point) -> Self {
        Self {
            target: None,
            snapped_position: original,
            show_indicator: false,
            original_position: original,
        }
    }

    /// Check if this result snapped to a real target (not grid)
    pub fn has_target(&self) -> bool {
        self.target.is_some() && self.show_indicator
    }

    /// Check if snapped to a terminal
    pub fn is_terminal_snap(&self) -> bool {
        self.target
            .as_ref()
            .is_some_and(|t| t.target_type.is_terminal())
    }

    /// Get the target type if present
    pub fn target_type(&self) -> Option<&SnapTargetType> {
        self.target.as_ref().map(|t| &t.target_type)
    }

    /// Get the component ID if snapped to a terminal
    pub fn terminal_component_id(&self) -> Option<u64> {
        match self.target_type() {
            Some(SnapTargetType::Terminal { component_id, .. }) => Some(*component_id),
            _ => None,
        }
    }

    /// Get the terminal name if snapped to a terminal
    pub fn terminal_name(&self) -> Option<&str> {
        match self.target_type() {
            Some(SnapTargetType::Terminal { terminal_name, .. }) => Some(terminal_name),
            _ => None,
        }
    }
}

// =============================================================================
// SnapEngine - The main snapping logic
// =============================================================================

/// Configuration for the snap engine
///
/// Controls which types of targets are considered and the snap radius.
/// Can be modified at runtime to enable/disable specific snap behaviors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapEngine {
    /// Master enable/disable for all snapping
    pub enabled: bool,

    /// Snap radius in grid units
    ///
    /// Points within this distance from the cursor are considered for snapping.
    /// Typical values: 1-3 grid units.
    pub snap_radius: i32,

    /// Enable snapping to component terminals
    pub snap_to_terminals: bool,

    /// Enable snapping to wire endpoints
    pub snap_to_wire_endpoints: bool,

    /// Enable snapping to wire segments (mid-wire)
    pub snap_to_wire_segments: bool,

    /// Enable snapping to junction points
    pub snap_to_junctions: bool,

    /// Enable snapping to grid intersections
    pub snap_to_grid: bool,

    /// Grid size for grid snapping (defaults to 1)
    pub grid_size: i32,
}

impl Default for SnapEngine {
    fn default() -> Self {
        Self {
            enabled: true,
            snap_radius: 2, // 2 grid units - good balance
            snap_to_terminals: true,
            snap_to_wire_endpoints: true,
            snap_to_wire_segments: true,
            snap_to_junctions: true,
            snap_to_grid: true,
            grid_size: 1,
        }
    }
}

impl SnapEngine {
    /// Create a new snap engine with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a snap engine with only terminal snapping
    pub fn terminals_only() -> Self {
        Self {
            snap_to_terminals: true,
            snap_to_wire_endpoints: false,
            snap_to_wire_segments: false,
            snap_to_junctions: false,
            snap_to_grid: true,
            ..Default::default()
        }
    }

    /// Create a snap engine with all snapping disabled
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }

    /// Set the snap radius
    pub fn with_radius(mut self, radius: i32) -> Self {
        self.snap_radius = radius.max(1);
        self
    }

    /// Find the best snap target for a given position
    ///
    /// # Arguments
    /// * `pos` - The cursor position in grid coordinates
    /// * `components` - All components in the schematic
    /// * `wires` - All wires in the schematic
    /// * `junctions` - All junction points in the schematic
    ///
    /// # Returns
    /// A SnapResult containing the snapped position and target info
    pub fn find_snap_target(
        &self,
        pos: Point,
        components: &[Component],
        wires: &[Wire],
        junctions: &[Junction],
    ) -> SnapResult {
        // If snapping is disabled, return position as-is
        if !self.enabled {
            return SnapResult::no_snap(pos);
        }

        // Collect all potential targets
        let mut candidates: Vec<SnapTarget> = Vec::new();

        // 1. Collect terminal targets
        if self.snap_to_terminals {
            self.collect_terminal_targets(pos, components, &mut candidates);
        }

        // 2. Collect junction targets
        if self.snap_to_junctions {
            self.collect_junction_targets(pos, junctions, &mut candidates);
        }

        // 3. Collect wire endpoint targets
        if self.snap_to_wire_endpoints {
            self.collect_wire_endpoint_targets(pos, wires, &mut candidates);
        }

        // 4. Collect wire segment targets
        if self.snap_to_wire_segments {
            self.collect_wire_segment_targets(pos, wires, &mut candidates);
        }

        // Filter by snap radius and find best candidate
        let radius_sq = (self.snap_radius as f64).powi(2);
        candidates.retain(|t| t.distance * t.distance <= radius_sq);

        if let Some(best) = self.select_best_target(&candidates) {
            return SnapResult::with_target(best, pos);
        }

        // Fallback to grid snap
        if self.snap_to_grid {
            let snapped = self.snap_to_grid_point(pos);
            if snapped != pos {
                let dist = ((snapped.x - pos.x).pow(2) + (snapped.y - pos.y).pow(2)) as f64;
                let target = SnapTarget::grid(snapped, dist.sqrt());
                return SnapResult::with_target(target, pos);
            }
        }

        SnapResult::grid_only(pos, pos)
    }

    /// Collect terminal snap targets from components
    fn collect_terminal_targets(
        &self,
        pos: Point,
        components: &[Component],
        candidates: &mut Vec<SnapTarget>,
    ) {
        for comp in components {
            for (name, term_pos) in comp.terminal_positions() {
                let dx = (term_pos.x - pos.x) as f64;
                let dy = (term_pos.y - pos.y) as f64;
                let dist = (dx * dx + dy * dy).sqrt();

                candidates.push(SnapTarget::terminal(term_pos, comp.id, name, dist));
            }
        }
    }

    /// Collect junction snap targets
    fn collect_junction_targets(
        &self,
        pos: Point,
        junctions: &[Junction],
        candidates: &mut Vec<SnapTarget>,
    ) {
        for junction in junctions {
            let dx = (junction.pos.x - pos.x) as f64;
            let dy = (junction.pos.y - pos.y) as f64;
            let dist = (dx * dx + dy * dy).sqrt();

            candidates.push(SnapTarget::junction(junction.pos, dist));
        }
    }

    /// Collect wire endpoint snap targets
    fn collect_wire_endpoint_targets(
        &self,
        pos: Point,
        wires: &[Wire],
        candidates: &mut Vec<SnapTarget>,
    ) {
        for wire in wires {
            if wire.points.is_empty() {
                continue;
            }

            // Start point
            let start = wire.points[0];
            let dx = (start.x - pos.x) as f64;
            let dy = (start.y - pos.y) as f64;
            let dist = (dx * dx + dy * dy).sqrt();
            candidates.push(SnapTarget::wire_endpoint(start, wire.id, true, dist));

            // End point
            if wire.points.len() > 1 {
                let end = wire.points[wire.points.len() - 1];
                let dx = (end.x - pos.x) as f64;
                let dy = (end.y - pos.y) as f64;
                let dist = (dx * dx + dy * dy).sqrt();
                candidates.push(SnapTarget::wire_endpoint(end, wire.id, false, dist));
            }
        }
    }

    /// Collect wire segment snap targets
    ///
    /// Note: Wire segment midpoints are snapped to the grid to ensure all connection
    /// points are grid-aligned. This is the commercial-standard approach (Cadence style).
    fn collect_wire_segment_targets(
        &self,
        pos: Point,
        wires: &[Wire],
        candidates: &mut Vec<SnapTarget>,
    ) {
        for wire in wires {
            for (idx, segment) in wire.segments().enumerate() {
                // Find closest point on segment, then snap to grid for commercial-grade alignment
                let closest_raw = segment.closest_point(pos);
                let closest = self.snap_to_grid_point(closest_raw);

                // Recalculate distance after grid snapping
                let dx = (closest.x - pos.x) as f64;
                let dy = (closest.y - pos.y) as f64;
                let dist = (dx * dx + dy * dy).sqrt();

                // Only add if it's not at an endpoint (those are already covered)
                // Check that snapped point is still within segment bounds
                let in_x_bounds = (closest.x >= segment.start.x.min(segment.end.x))
                    && (closest.x <= segment.start.x.max(segment.end.x));
                let in_y_bounds = (closest.y >= segment.start.y.min(segment.end.y))
                    && (closest.y <= segment.start.y.max(segment.end.y));

                if closest != segment.start && closest != segment.end && in_x_bounds && in_y_bounds {
                    candidates.push(SnapTarget::wire_segment(closest, wire.id, idx, dist));
                }
            }
        }
    }

    /// Select the best target from candidates
    fn select_best_target(&self, candidates: &[SnapTarget]) -> Option<SnapTarget> {
        candidates
            .iter()
            .min_by(|a, b| {
                // Compare by priority first
                match a.priority().cmp(&b.priority()) {
                    std::cmp::Ordering::Equal => {
                        // Same priority, use distance
                        a.distance
                            .partial_cmp(&b.distance)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    }
                    other => other,
                }
            })
            .cloned()
    }

    /// Snap a position to the nearest grid point
    fn snap_to_grid_point(&self, pos: Point) -> Point {
        let gs = self.grid_size.max(1);
        Point::new(((pos.x + gs / 2) / gs) * gs, ((pos.y + gs / 2) / gs) * gs)
    }

    /// Find the nearest terminal to a position
    ///
    /// Convenience method that only searches terminals.
    pub fn find_nearest_terminal(
        &self,
        pos: Point,
        components: &[Component],
    ) -> Option<SnapTarget> {
        let mut candidates = Vec::new();
        self.collect_terminal_targets(pos, components, &mut candidates);

        candidates
            .into_iter()
            .filter(|t| t.distance <= self.snap_radius as f64)
            .min_by(|a, b| {
                a.distance
                    .partial_cmp(&b.distance)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Check if a position is at a terminal
    ///
    /// Returns the terminal info if the position exactly matches a terminal.
    pub fn is_at_terminal(&self, pos: Point, components: &[Component]) -> Option<(u64, String)> {
        for comp in components {
            for (name, term_pos) in comp.terminal_positions() {
                if term_pos == pos {
                    return Some((comp.id, name.to_string()));
                }
            }
        }
        None
    }

    /// Get all terminals in the schematic
    ///
    /// Returns a list of (component_id, terminal_name, position) tuples.
    pub fn get_all_terminals(&self, components: &[Component]) -> Vec<(u64, String, Point)> {
        let mut terminals = Vec::new();
        for comp in components {
            for (name, pos) in comp.terminal_positions() {
                terminals.push((comp.id, name.to_string(), pos));
            }
        }
        terminals
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::schematic::{ComponentType, Rotation};

    // Helper to create a test component
    fn make_component(id: u64, x: i32, y: i32, kind: ComponentType) -> Component {
        Component::new(id, kind, Point::new(x, y))
    }

    fn make_resistor(id: u64, x: i32, y: i32) -> Component {
        make_component(id, x, y, ComponentType::Resistor)
    }

    fn make_capacitor(id: u64, x: i32, y: i32) -> Component {
        make_component(id, x, y, ComponentType::Capacitor)
    }

    fn make_wire(id: u64, points: Vec<Point>) -> Wire {
        Wire::new(id, points)
    }

    // =========================================================================
    // SnapTargetType Tests
    // =========================================================================

    #[test]
    fn test_snap_target_type_priority_order() {
        // Terminal should have highest priority (lowest number)
        assert!(
            SnapTargetType::Terminal {
                component_id: 1,
                terminal_name: "+".to_string()
            }
            .priority()
                < SnapTargetType::Junction.priority()
        );

        assert!(
            SnapTargetType::Junction.priority()
                < SnapTargetType::WireEndpoint {
                    wire_id: 1,
                    is_start: true
                }
                .priority()
        );

        assert!(
            SnapTargetType::WireEndpoint {
                wire_id: 1,
                is_start: true
            }
            .priority()
                < SnapTargetType::WireSegment {
                    wire_id: 1,
                    segment_index: 0
                }
                .priority()
        );

        assert!(
            SnapTargetType::WireSegment {
                wire_id: 1,
                segment_index: 0
            }
            .priority()
                < SnapTargetType::Grid.priority()
        );
    }

    #[test]
    fn test_snap_target_type_is_terminal() {
        let terminal = SnapTargetType::Terminal {
            component_id: 1,
            terminal_name: "+".to_string(),
        };
        assert!(terminal.is_terminal());

        let wire = SnapTargetType::WireEndpoint {
            wire_id: 1,
            is_start: true,
        };
        assert!(!wire.is_terminal());

        let grid = SnapTargetType::Grid;
        assert!(!grid.is_terminal());
    }

    #[test]
    fn test_snap_target_type_is_wire_related() {
        let endpoint = SnapTargetType::WireEndpoint {
            wire_id: 1,
            is_start: true,
        };
        assert!(endpoint.is_wire_related());

        let segment = SnapTargetType::WireSegment {
            wire_id: 1,
            segment_index: 0,
        };
        assert!(segment.is_wire_related());

        let terminal = SnapTargetType::Terminal {
            component_id: 1,
            terminal_name: "+".to_string(),
        };
        assert!(!terminal.is_wire_related());
    }

    #[test]
    fn test_snap_target_type_creates_connection() {
        let terminal = SnapTargetType::Terminal {
            component_id: 1,
            terminal_name: "+".to_string(),
        };
        assert!(terminal.creates_connection());

        let junction = SnapTargetType::Junction;
        assert!(junction.creates_connection());

        let grid = SnapTargetType::Grid;
        assert!(!grid.creates_connection());
    }

    // =========================================================================
    // SnapTarget Tests
    // =========================================================================

    #[test]
    fn test_snap_target_new() {
        let target = SnapTarget::new(
            Point::new(10, 20),
            SnapTargetType::Terminal {
                component_id: 5,
                terminal_name: "+".to_string(),
            },
            1.5,
        );
        assert_eq!(target.position, Point::new(10, 20));
        assert_eq!(target.distance, 1.5);
        assert!(target.target_type.is_terminal());
    }

    #[test]
    fn test_snap_target_factory_methods() {
        let term = SnapTarget::terminal(Point::new(0, 0), 1, "+", 0.5);
        assert!(term.target_type.is_terminal());

        let endpoint = SnapTarget::wire_endpoint(Point::new(1, 1), 2, true, 1.0);
        assert!(endpoint.target_type.is_wire_related());

        let segment = SnapTarget::wire_segment(Point::new(2, 2), 3, 0, 1.5);
        assert!(segment.target_type.is_wire_related());

        let junc = SnapTarget::junction(Point::new(3, 3), 2.0);
        assert_eq!(junc.target_type, SnapTargetType::Junction);

        let grid = SnapTarget::grid(Point::new(4, 4), 2.5);
        assert_eq!(grid.target_type, SnapTargetType::Grid);
    }

    #[test]
    fn test_snap_target_is_better_than_priority() {
        // Terminal should be better than wire endpoint
        let terminal = SnapTarget::terminal(Point::new(0, 0), 1, "+", 2.0);
        let wire = SnapTarget::wire_endpoint(Point::new(1, 1), 1, true, 1.0);

        assert!(terminal.is_better_than(&wire));
        assert!(!wire.is_better_than(&terminal));
    }

    #[test]
    fn test_snap_target_is_better_than_distance() {
        // Same priority, closer distance wins
        let closer = SnapTarget::terminal(Point::new(0, 0), 1, "+", 1.0);
        let farther = SnapTarget::terminal(Point::new(0, 0), 2, "-", 2.0);

        assert!(closer.is_better_than(&farther));
        assert!(!farther.is_better_than(&closer));
    }

    // =========================================================================
    // SnapResult Tests
    // =========================================================================

    #[test]
    fn test_snap_result_with_target() {
        let target = SnapTarget::terminal(Point::new(10, 20), 1, "+", 0.5);
        let result = SnapResult::with_target(target, Point::new(11, 21));

        assert!(result.has_target());
        assert!(result.show_indicator);
        assert_eq!(result.snapped_position, Point::new(10, 20));
        assert_eq!(result.original_position, Point::new(11, 21));
    }

    #[test]
    fn test_snap_result_grid_only() {
        let result = SnapResult::grid_only(Point::new(10, 10), Point::new(11, 11));

        assert!(!result.has_target());
        assert!(!result.show_indicator);
        assert_eq!(result.snapped_position, Point::new(10, 10));
    }

    #[test]
    fn test_snap_result_no_snap() {
        let result = SnapResult::no_snap(Point::new(15, 25));

        assert!(!result.has_target());
        assert!(!result.show_indicator);
        assert_eq!(result.snapped_position, Point::new(15, 25));
        assert_eq!(result.original_position, Point::new(15, 25));
    }

    #[test]
    fn test_snap_result_is_terminal_snap() {
        let term = SnapTarget::terminal(Point::new(0, 0), 1, "+", 0.0);
        let result = SnapResult::with_target(term, Point::new(0, 0));
        assert!(result.is_terminal_snap());

        let wire = SnapTarget::wire_endpoint(Point::new(0, 0), 1, true, 0.0);
        let result2 = SnapResult::with_target(wire, Point::new(0, 0));
        assert!(!result2.is_terminal_snap());
    }

    #[test]
    fn test_snap_result_terminal_info() {
        let term = SnapTarget::terminal(Point::new(0, 0), 42, "gate", 0.0);
        let result = SnapResult::with_target(term, Point::new(0, 0));

        assert_eq!(result.terminal_component_id(), Some(42));
        assert_eq!(result.terminal_name(), Some("gate"));
    }

    // =========================================================================
    // SnapEngine Tests
    // =========================================================================

    #[test]
    fn test_snap_engine_default() {
        let engine = SnapEngine::default();
        assert!(engine.enabled);
        assert!(engine.snap_to_terminals);
        assert!(engine.snap_to_wire_endpoints);
        assert!(engine.snap_to_grid);
        assert_eq!(engine.snap_radius, 2);
    }

    #[test]
    fn test_snap_engine_terminals_only() {
        let engine = SnapEngine::terminals_only();
        assert!(engine.snap_to_terminals);
        assert!(!engine.snap_to_wire_endpoints);
        assert!(!engine.snap_to_wire_segments);
        assert!(!engine.snap_to_junctions);
        assert!(engine.snap_to_grid);
    }

    #[test]
    fn test_snap_engine_disabled() {
        let engine = SnapEngine::disabled();
        assert!(!engine.enabled);

        // Should return no-snap result
        let result = engine.find_snap_target(Point::new(0, 0), &[make_resistor(1, 0, 0)], &[], &[]);
        assert!(!result.has_target());
        assert!(!result.show_indicator);
    }

    #[test]
    fn test_snap_engine_with_radius() {
        let engine = SnapEngine::default().with_radius(5);
        assert_eq!(engine.snap_radius, 5);

        // Minimum radius is 1
        let engine2 = SnapEngine::default().with_radius(0);
        assert_eq!(engine2.snap_radius, 1);
    }

    #[test]
    fn test_snap_to_terminal_exact() {
        let engine = SnapEngine::default();
        let resistor = make_resistor(1, 10, 10);

        // Get actual terminal position
        let terminals = resistor.terminal_positions();
        let (_, term_pos) = terminals[0];

        let result = engine.find_snap_target(term_pos, &[resistor], &[], &[]);

        assert!(result.has_target());
        assert!(result.is_terminal_snap());
        assert_eq!(result.snapped_position, term_pos);
    }

    #[test]
    fn test_snap_to_terminal_within_radius() {
        let engine = SnapEngine::default().with_radius(3);
        let resistor = make_resistor(1, 10, 10);

        let terminals = resistor.terminal_positions();
        let (_, term_pos) = terminals[0];

        // Query 1 unit away from terminal
        let query = Point::new(term_pos.x + 1, term_pos.y);
        let result = engine.find_snap_target(query, &[resistor], &[], &[]);

        assert!(result.has_target());
        assert!(result.is_terminal_snap());
        assert_eq!(result.snapped_position, term_pos);
    }

    #[test]
    fn test_snap_to_terminal_outside_radius() {
        let engine = SnapEngine::default().with_radius(2);
        let resistor = make_resistor(1, 10, 10);

        // Query far from any terminal
        let query = Point::new(100, 100);
        let result = engine.find_snap_target(query, &[resistor], &[], &[]);

        // Should not snap to terminal (but might snap to grid)
        assert!(!result.is_terminal_snap());
    }

    #[test]
    fn test_snap_priority_terminal_over_wire() {
        let engine = SnapEngine::default();
        let resistor = make_resistor(1, 10, 10);

        let terminals = resistor.terminal_positions();
        let (_, term_pos) = terminals[0];

        // Wire endpoint at same position
        let wire = make_wire(1, vec![term_pos, Point::new(20, 10)]);

        let result = engine.find_snap_target(term_pos, &[resistor], &[wire], &[]);

        // Should prefer terminal
        assert!(result.is_terminal_snap());
    }

    #[test]
    fn test_snap_to_wire_endpoint() {
        let engine = SnapEngine::default();
        let wire = make_wire(1, vec![Point::new(5, 5), Point::new(15, 5)]);

        // Query near start endpoint
        let query = Point::new(5, 6);
        let result = engine.find_snap_target(query, &[], &[wire], &[]);

        assert!(result.has_target());
        assert!(matches!(
            result.target_type(),
            Some(SnapTargetType::WireEndpoint {
                wire_id: 1,
                is_start: true
            })
        ));
    }

    #[test]
    fn test_snap_to_wire_end() {
        let engine = SnapEngine::default();
        let wire = make_wire(1, vec![Point::new(5, 5), Point::new(15, 5)]);

        // Query near end endpoint
        let query = Point::new(15, 6);
        let result = engine.find_snap_target(query, &[], &[wire], &[]);

        assert!(result.has_target());
        assert!(matches!(
            result.target_type(),
            Some(SnapTargetType::WireEndpoint {
                wire_id: 1,
                is_start: false
            })
        ));
    }

    #[test]
    fn test_snap_to_junction() {
        let engine = SnapEngine::default();
        let junction = Junction::new(1, Point::new(20, 20));

        let query = Point::new(20, 21);
        let result = engine.find_snap_target(query, &[], &[], &[junction]);

        assert!(result.has_target());
        assert_eq!(result.target_type(), Some(&SnapTargetType::Junction));
        assert_eq!(result.snapped_position, Point::new(20, 20));
    }

    #[test]
    fn test_snap_grid_fallback() {
        let engine = SnapEngine::default();

        // No targets, should snap to grid
        let query = Point::new(10, 10);
        let result = engine.find_snap_target(query, &[], &[], &[]);

        // Grid snap doesn't show indicator
        assert!(!result.show_indicator);
        assert_eq!(result.snapped_position, Point::new(10, 10));
    }

    #[test]
    fn test_snap_to_grid_point() {
        let engine = SnapEngine::default();

        assert_eq!(
            engine.snap_to_grid_point(Point::new(0, 0)),
            Point::new(0, 0)
        );
        assert_eq!(
            engine.snap_to_grid_point(Point::new(1, 1)),
            Point::new(1, 1)
        );
    }

    #[test]
    fn test_find_nearest_terminal_found() {
        let engine = SnapEngine::default();
        let resistor = make_resistor(1, 10, 10);

        let terminals = resistor.terminal_positions();
        let (_, term_pos) = terminals[0];

        let result = engine.find_nearest_terminal(term_pos, &[resistor]);
        assert!(result.is_some());
        assert_eq!(result.unwrap().position, term_pos);
    }

    #[test]
    fn test_find_nearest_terminal_not_found() {
        let engine = SnapEngine::default().with_radius(1);
        let resistor = make_resistor(1, 10, 10);

        // Far from any terminal
        let result = engine.find_nearest_terminal(Point::new(100, 100), &[resistor]);
        assert!(result.is_none());
    }

    #[test]
    fn test_is_at_terminal_exact() {
        let engine = SnapEngine::default();
        let resistor = make_resistor(1, 10, 10);

        let terminals = resistor.terminal_positions();
        let (name, term_pos) = terminals[0];

        let result = engine.is_at_terminal(term_pos, &[resistor]);
        assert!(result.is_some());
        let (comp_id, term_name) = result.unwrap();
        assert_eq!(comp_id, 1);
        assert_eq!(term_name, name);
    }

    #[test]
    fn test_is_at_terminal_not_exact() {
        let engine = SnapEngine::default();
        let resistor = make_resistor(1, 10, 10);

        let result = engine.is_at_terminal(Point::new(0, 0), &[resistor]);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_all_terminals() {
        let engine = SnapEngine::default();
        let r1 = make_resistor(1, 10, 10);
        let r2 = make_resistor(2, 20, 20);

        let terminals = engine.get_all_terminals(&[r1, r2]);

        // Each resistor has 2 terminals
        assert_eq!(terminals.len(), 4);
    }

    #[test]
    fn test_snap_with_rotated_component() {
        let engine = SnapEngine::default();
        let mut comp = make_resistor(1, 10, 10);
        comp.rotation = Rotation::R90;

        let terminals = comp.terminal_positions();
        let (_, term_pos) = terminals[0];

        let result = engine.find_snap_target(term_pos, &[comp], &[], &[]);
        assert!(result.is_terminal_snap());
        assert_eq!(result.snapped_position, term_pos);
    }

    #[test]
    fn test_snap_multiple_components_nearest() {
        let engine = SnapEngine::default();
        let r1 = make_resistor(1, 0, 0);
        let r2 = make_resistor(2, 10, 0);

        let terms1 = r1.terminal_positions();
        let (_, pos1) = terms1[0];

        // Query exactly at r1's terminal
        let result = engine.find_snap_target(pos1, &[r1, r2], &[], &[]);

        assert!(result.is_terminal_snap());
        assert_eq!(result.terminal_component_id(), Some(1));
    }

    #[test]
    fn test_snap_all_component_types() {
        let engine = SnapEngine::default();

        let components = vec![
            make_component(1, 0, 0, ComponentType::Resistor),
            make_component(2, 10, 0, ComponentType::Capacitor),
            make_component(3, 20, 0, ComponentType::Inductor),
            make_component(4, 30, 0, ComponentType::VoltageSource),
            make_component(5, 40, 0, ComponentType::CurrentSource),
            make_component(6, 50, 0, ComponentType::Diode),
            make_component(7, 60, 0, ComponentType::Ground),
        ];

        let all_terminals = engine.get_all_terminals(&components);

        // Should have terminals for all components (ground has 1, others have 2+)
        assert!(all_terminals.len() >= 7);
    }

    #[test]
    fn test_snap_to_capacitor_terminals() {
        let engine = SnapEngine::default();
        let cap = make_capacitor(1, 10, 10);

        for (_, term_pos) in cap.terminal_positions() {
            let result = engine.find_snap_target(term_pos, &[cap.clone()], &[], &[]);
            assert!(result.is_terminal_snap());
        }
    }

    #[test]
    fn test_snap_disabled_terminals() {
        let mut engine = SnapEngine::default();
        engine.snap_to_terminals = false;

        let resistor = make_resistor(1, 10, 10);
        let terminals = resistor.terminal_positions();
        let (_, term_pos) = terminals[0];

        let result = engine.find_snap_target(term_pos, &[resistor], &[], &[]);

        // Should not snap to terminal
        assert!(!result.is_terminal_snap());
    }

    #[test]
    fn test_snap_disabled_wire_endpoints() {
        let mut engine = SnapEngine::default();
        engine.snap_to_wire_endpoints = false;

        let wire = make_wire(1, vec![Point::new(5, 5), Point::new(15, 5)]);
        let result = engine.find_snap_target(Point::new(5, 5), &[], &[wire], &[]);

        // Should not snap to wire endpoint
        assert!(!matches!(
            result.target_type(),
            Some(SnapTargetType::WireEndpoint { .. })
        ));
    }

    #[test]
    fn test_snap_disabled_junctions() {
        let mut engine = SnapEngine::default();
        engine.snap_to_junctions = false;

        let junction = Junction::new(1, Point::new(20, 20));
        let result = engine.find_snap_target(Point::new(20, 20), &[], &[], &[junction]);

        // Should not snap to junction
        assert!(!matches!(
            result.target_type(),
            Some(SnapTargetType::Junction)
        ));
    }

    #[test]
    fn test_empty_schematic() {
        let engine = SnapEngine::default();
        let result = engine.find_snap_target(Point::new(5, 5), &[], &[], &[]);

        // Should still work, just no target found
        assert!(!result.show_indicator);
    }
}
