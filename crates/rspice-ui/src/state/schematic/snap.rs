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

                if closest != segment.start && closest != segment.end && in_x_bounds && in_y_bounds
                {
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

