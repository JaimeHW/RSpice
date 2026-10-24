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

use rspice_design::resolved_symbol::ResolvedCellSymbol;
use rspice_design::schematic::{component::Component, net_label::Junction, wire::Wire};
use rspice_design_model::Point;
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

    /// Get the target type if present
    pub fn target_type(&self) -> Option<&SnapTargetType> {
        self.target.as_ref().map(|t| &t.target_type)
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

    /// Create a snap engine with all snapping disabled
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
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

    /// Find the best snap target, resolving cell-instance terminals through
    /// authored symbol geometry when the caller has frame-local symbol context.
    pub fn find_snap_target_resolved<'a>(
        &self,
        pos: Point,
        components: &[Component],
        wires: &[Wire],
        junctions: &[Junction],
        mut resolved_symbol_for: impl FnMut(&Component) -> Option<&'a ResolvedCellSymbol>,
    ) -> SnapResult {
        if !self.enabled {
            return SnapResult::no_snap(pos);
        }

        let mut candidates: Vec<SnapTarget> = Vec::new();

        if self.snap_to_terminals {
            self.collect_terminal_targets_resolved(
                pos,
                components,
                &mut candidates,
                &mut resolved_symbol_for,
            );
        }

        if self.snap_to_junctions {
            self.collect_junction_targets(pos, junctions, &mut candidates);
        }

        if self.snap_to_wire_endpoints {
            self.collect_wire_endpoint_targets(pos, wires, &mut candidates);
        }

        if self.snap_to_wire_segments {
            self.collect_wire_segment_targets(pos, wires, &mut candidates);
        }

        let radius_sq = (self.snap_radius as f64).powi(2);
        candidates.retain(|t| t.distance * t.distance <= radius_sq);

        if let Some(best) = self.select_best_target(&candidates) {
            return SnapResult::with_target(best, pos);
        }

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

    fn collect_terminal_targets_resolved<'a>(
        &self,
        pos: Point,
        components: &[Component],
        candidates: &mut Vec<SnapTarget>,
        resolved_symbol_for: &mut impl FnMut(&Component) -> Option<&'a ResolvedCellSymbol>,
    ) {
        for comp in components {
            let resolved_symbol = resolved_symbol_for(comp);
            for (name, term_pos) in comp.terminal_positions_resolved(resolved_symbol) {
                let dx = (term_pos.x - pos.x) as f64;
                let dy = (term_pos.y - pos.y) as f64;
                let dist = (dx * dx + dy * dy).sqrt();

                candidates.push(SnapTarget::terminal(term_pos, comp.id, name.as_str(), dist));
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
    /// In grid mode, wire-segment attachments remain grid aligned. In Free
    /// mode, the geometrically closest integer schematic coordinate is kept so
    /// disabling grid intersections cannot silently quantize the attachment.
    fn collect_wire_segment_targets(
        &self,
        pos: Point,
        wires: &[Wire],
        candidates: &mut Vec<SnapTarget>,
    ) {
        for wire in wires {
            for (idx, segment) in wire.segments().enumerate() {
                // Find the closest representable point on the segment. Grid
                // quantization is conditional: `Free` is an exact no-grid
                // contract, not merely a hidden-grid presentation.
                let closest_raw = segment.closest_point(pos);
                let closest = if self.snap_to_grid {
                    self.snap_to_grid_point(closest_raw)
                } else {
                    closest_raw
                };

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
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::{
        library::{Cell, Library, LibraryCatalog, View, ViewType},
        schematic::{
            component::LibraryCellInstance, component_type::ComponentType,
            document::SchematicDocument,
        },
        symbol::{SymbolDocument, SymbolPin},
        symbol_resolver::SymbolResolver,
    };
    use rspice_design_model::port::{PortDirection, PortSpec};
    use std::collections::HashMap;

    fn engine_with(f: impl FnOnce(&mut SnapEngine)) -> SnapEngine {
        let mut e = SnapEngine::default();
        f(&mut e);
        e
    }

    /// A resistor at `pos` exposes "+" at (-20, 0) and "-" at (+20, 0).
    fn resistor(id: u64, pos: Point) -> Component {
        Component::new(id, ComponentType::Resistor, pos)
    }

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    fn resolved_amp_symbol() -> ResolvedCellSymbol {
        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
            ],
            ..SymbolDocument::default()
        };

        let mut libraries = LibraryCatalog::default();
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        document
            .store_in_view(&mut symbol_view)
            .expect("symbol stores");
        cell.add_view(symbol_view);
        library.add_cell(cell);
        libraries.add_library(library);

        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);

        let buffers = HashMap::<String, SchematicDocument>::new();
        SymbolResolver::new(&libraries, &buffers)
            .resolve_binding(&binding)
            .expect("symbol resolves")
    }

    fn amp_instance(id: u64, pos: Point) -> Component {
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);
        Component::new(id, ComponentType::CellInstance, pos).with_library_cell(binding)
    }

    // -------------------------------------------------------------------------
    // Grid snapping
    // -------------------------------------------------------------------------

    /// Points already on the grid are returned unchanged.
    #[test]
    fn grid_snap_on_grid_passthrough() {
        let engine = engine_with(|e| e.grid_size = 10);
        assert_eq!(
            engine.snap_to_grid_point(Point::new(20, 30)),
            Point::new(20, 30)
        );
        assert_eq!(engine.snap_to_grid_point(Point::origin()), Point::origin());
    }

    /// Off-grid points round to the nearest grid line; the exact midpoint rounds up.
    #[test]
    fn grid_snap_rounds_to_nearest() {
        let engine = engine_with(|e| e.grid_size = 10);
        assert_eq!(
            engine.snap_to_grid_point(Point::new(4, 6)),
            Point::new(0, 10)
        );
        assert_eq!(
            engine.snap_to_grid_point(Point::new(5, 15)),
            Point::new(10, 20)
        );
        assert_eq!(
            engine.snap_to_grid_point(Point::new(14, 16)),
            Point::new(10, 20)
        );
    }

    /// Unit grid is the identity, including negative coordinates.
    #[test]
    fn grid_snap_unit_grid_is_identity() {
        let engine = SnapEngine::default(); // grid_size = 1
        assert_eq!(
            engine.snap_to_grid_point(Point::new(-7, -3)),
            Point::new(-7, -3)
        );
        assert_eq!(
            engine.snap_to_grid_point(Point::new(13, -41)),
            Point::new(13, -41)
        );
    }

    /// A non-positive grid_size is clamped to 1 rather than dividing by zero.
    #[test]
    fn grid_snap_clamps_grid_size_to_one() {
        let engine = engine_with(|e| e.grid_size = 0);
        assert_eq!(
            engine.snap_to_grid_point(Point::new(7, -9)),
            Point::new(7, -9)
        );
    }

    /// With no nearby targets and a coarse grid, find_snap_target falls back to
    /// a Grid target (no indicator) at the nearest intersection.
    #[test]
    fn grid_fallback_with_coarse_grid() {
        let engine = engine_with(|e| e.grid_size = 10);
        let result = engine.find_snap_target(Point::new(13, 17), &[], &[], &[]);
        assert_eq!(result.snapped_position, Point::new(10, 20));
        assert!(!result.show_indicator);
        assert_eq!(result.target_type(), Some(&SnapTargetType::Grid));
    }

    /// An on-grid cursor with no targets yields a grid-only result (no target).
    #[test]
    fn grid_fallback_on_grid_cursor_is_grid_only() {
        let engine = engine_with(|e| e.grid_size = 10);
        let result = engine.find_snap_target(Point::new(10, 20), &[], &[], &[]);
        assert_eq!(result.snapped_position, Point::new(10, 20));
        assert!(result.target.is_none());
        assert!(!result.show_indicator);
    }

    // -------------------------------------------------------------------------
    // Priority and candidate selection
    // -------------------------------------------------------------------------

    /// Priority order: Terminal < Junction < WireEndpoint < WireSegment < Grid.
    #[test]
    fn target_type_priority_order() {
        let terminal = SnapTargetType::Terminal {
            component_id: 1,
            terminal_name: "+".to_string(),
        };
        let endpoint = SnapTargetType::WireEndpoint {
            wire_id: 1,
            is_start: true,
        };
        let segment = SnapTargetType::WireSegment {
            wire_id: 1,
            segment_index: 0,
        };
        assert!(terminal.priority() < SnapTargetType::Junction.priority());
        assert!(SnapTargetType::Junction.priority() < endpoint.priority());
        assert!(endpoint.priority() < segment.priority());
        assert!(segment.priority() < SnapTargetType::Grid.priority());
    }

    /// A terminal one unit away beats a junction at zero distance.
    #[test]
    fn terminal_beats_closer_junction() {
        let engine = SnapEngine::default();
        let comps = [resistor(1, Point::origin())]; // "-" terminal at (20, 0)
        let junctions = [Junction::new(7, Point::new(21, 0))];
        let result = engine.find_snap_target(Point::new(21, 0), &comps, &[], &junctions);
        assert_eq!(
            result.target_type(),
            Some(&SnapTargetType::Terminal {
                component_id: 1,
                terminal_name: "-".to_string()
            })
        );
        assert_eq!(result.snapped_position, Point::new(20, 0));
        assert!(result.show_indicator);
    }

    /// A junction one unit away beats a wire endpoint at zero distance.
    #[test]
    fn junction_beats_closer_wire_endpoint() {
        let engine = SnapEngine::default();
        let wires = [Wire::segment(3, Point::origin(), Point::new(10, 0))];
        let junctions = [Junction::new(7, Point::new(1, 0))];
        let result = engine.find_snap_target(Point::origin(), &[], &wires, &junctions);
        assert_eq!(result.target_type(), Some(&SnapTargetType::Junction));
        assert_eq!(result.snapped_position, Point::new(1, 0));
    }

    /// A wire endpoint beats a closer mid-segment point.
    #[test]
    fn wire_endpoint_beats_closer_segment_point() {
        let engine = SnapEngine::default();
        let wires = [Wire::segment(3, Point::origin(), Point::new(10, 0))];
        // Endpoint (0,0) is sqrt(2) away; segment point (1,0) is 1 away.
        let result = engine.find_snap_target(Point::new(1, 1), &[], &wires, &[]);
        assert_eq!(
            result.target_type(),
            Some(&SnapTargetType::WireEndpoint {
                wire_id: 3,
                is_start: true
            })
        );
        assert_eq!(result.snapped_position, Point::origin());
    }

    /// Within the same priority class, the nearer target wins.
    #[test]
    fn nearer_target_wins_within_same_priority() {
        let engine = SnapEngine::default();
        let junctions = [
            Junction::new(1, Point::new(2, 0)),
            Junction::new(2, Point::new(0, 1)),
        ];
        let result = engine.find_snap_target(Point::origin(), &[], &[], &junctions);
        assert_eq!(result.snapped_position, Point::new(0, 1));
    }

    // -------------------------------------------------------------------------
    // Snap radius boundaries
    // -------------------------------------------------------------------------

    /// A target exactly at the snap radius is still captured (inclusive bound).
    #[test]
    fn target_exactly_at_radius_snaps() {
        let engine = SnapEngine::default(); // snap_radius = 2
        let junctions = [Junction::new(1, Point::new(2, 0))];
        let result = engine.find_snap_target(Point::origin(), &[], &[], &junctions);
        assert_eq!(result.target_type(), Some(&SnapTargetType::Junction));
        assert_eq!(result.snapped_position, Point::new(2, 0));
    }

    /// A target just outside the radius (sqrt(5) > 2) is ignored.
    #[test]
    fn target_just_outside_radius_does_not_snap() {
        let engine = SnapEngine::default();
        let junctions = [Junction::new(1, Point::new(2, 1))];
        let result = engine.find_snap_target(Point::origin(), &[], &[], &junctions);
        assert!(result.target.is_none());
        assert_eq!(result.snapped_position, Point::origin());
    }

    /// A larger radius captures targets the default radius rejects.
    #[test]
    fn larger_radius_extends_capture() {
        let junctions = [Junction::new(1, Point::new(3, 0))];
        let default_engine = SnapEngine::default();
        assert!(
            default_engine
                .find_snap_target(Point::origin(), &[], &[], &junctions)
                .target
                .is_none()
        );
        let mut wide = SnapEngine::new();
        wide.snap_radius = 3;
        let result = wide.find_snap_target(Point::origin(), &[], &[], &junctions);
        assert_eq!(result.snapped_position, Point::new(3, 0));
    }

    // -------------------------------------------------------------------------
    // Engine configuration flags
    // -------------------------------------------------------------------------

    /// A disabled engine returns the cursor position untouched, even off-grid.
    #[test]
    fn disabled_engine_returns_original_position() {
        let engine = engine_with(|e| {
            e.enabled = false;
            e.grid_size = 10;
        });
        let junctions = [Junction::new(1, Point::new(13, 17))];
        let result = engine.find_snap_target(Point::new(13, 17), &[], &[], &junctions);
        assert_eq!(result.snapped_position, Point::new(13, 17));
        assert!(result.target.is_none());
        assert!(!result.show_indicator);
        assert_eq!(result.original_position, Point::new(13, 17));
    }

    /// Per-class flags gate their target type out of consideration.
    #[test]
    fn class_flags_gate_targets() {
        let engine = engine_with(|e| e.snap_to_junctions = false);
        let junctions = [Junction::new(1, Point::new(1, 0))];
        let result = engine.find_snap_target(Point::origin(), &[], &[], &junctions);
        assert!(result.target.is_none());
    }

    /// Resolved-aware snapping uses authored symbol pin offsets instead of the
    /// generated fallback block geometry.
    #[test]
    fn resolved_instance_terminal_snap_uses_authored_offsets() {
        let engine = engine_with(|e| {
            e.snap_to_wire_endpoints = false;
            e.snap_to_wire_segments = false;
            e.snap_to_junctions = false;
        });
        let comps = [amp_instance(7, Point::new(100, 50))];
        let resolved = resolved_amp_symbol();

        let result =
            engine.find_snap_target_resolved(Point::new(61, 40), &comps, &[], &[], |component| {
                (component.id == 7).then_some(&resolved)
            });

        assert_eq!(
            result.target_type(),
            Some(&SnapTargetType::Terminal {
                component_id: 7,
                terminal_name: "IN".to_string()
            })
        );
        assert_eq!(result.snapped_position, Point::new(60, 40));
    }

    // -------------------------------------------------------------------------
    // Wire endpoint and segment targets
    // -------------------------------------------------------------------------

    /// Endpoint snaps report which end of the wire was hit.
    #[test]
    fn wire_endpoint_reports_start_vs_end() {
        let engine = SnapEngine::default();
        let wires = [Wire::segment(3, Point::origin(), Point::new(10, 0))];

        let result = engine.find_snap_target(Point::new(0, 1), &[], &wires, &[]);
        assert_eq!(
            result.target_type(),
            Some(&SnapTargetType::WireEndpoint {
                wire_id: 3,
                is_start: true
            })
        );

        let result = engine.find_snap_target(Point::new(10, 1), &[], &wires, &[]);
        assert_eq!(
            result.target_type(),
            Some(&SnapTargetType::WireEndpoint {
                wire_id: 3,
                is_start: false
            })
        );
    }

    /// Degenerate wires: a single-point wire snaps as a start endpoint,
    /// an empty wire contributes nothing.
    #[test]
    fn degenerate_wires_handled() {
        let engine = SnapEngine::default();
        let wires = [Wire::new(1, vec![Point::new(5, 5)]), Wire::new(2, vec![])];
        let result = engine.find_snap_target(Point::new(5, 6), &[], &wires, &[]);
        assert_eq!(
            result.target_type(),
            Some(&SnapTargetType::WireEndpoint {
                wire_id: 1,
                is_start: true
            })
        );

        let result = engine.find_snap_target(Point::new(50, 50), &[], &wires, &[]);
        assert!(result.target.is_none());
    }

    /// Mid-segment snapping projects onto the segment; points that land on an
    /// endpoint are excluded (endpoints own those positions).
    #[test]
    fn segment_snap_projects_and_excludes_endpoints() {
        let engine = engine_with(|e| {
            e.snap_to_terminals = false;
            e.snap_to_junctions = false;
            e.snap_to_wire_endpoints = false;
        });
        let wires = [Wire::segment(3, Point::origin(), Point::new(10, 0))];

        let result = engine.find_snap_target(Point::new(5, 1), &[], &wires, &[]);
        assert_eq!(
            result.target_type(),
            Some(&SnapTargetType::WireSegment {
                wire_id: 3,
                segment_index: 0
            })
        );
        assert_eq!(result.snapped_position, Point::new(5, 0));

        // Projection lands on the start vertex: no segment candidate.
        let result = engine.find_snap_target(Point::new(0, 1), &[], &wires, &[]);
        assert!(result.target.is_none());
    }

    #[test]
    fn free_mode_keeps_unquantized_wire_segment_attachment() {
        let wires = [Wire::segment(3, Point::origin(), Point::new(40, 0))];
        let mut engine = engine_with(|engine| {
            engine.grid_size = 10;
            engine.snap_radius = 5;
            engine.snap_to_terminals = false;
            engine.snap_to_junctions = false;
            engine.snap_to_wire_endpoints = false;
        });

        let grid = engine.find_snap_target(Point::new(17, 3), &[], &wires, &[]);
        assert_eq!(grid.snapped_position, Point::new(20, 0));

        engine.snap_to_grid = false;
        let free = engine.find_snap_target(Point::new(17, 3), &[], &wires, &[]);
        assert_eq!(free.snapped_position, Point::new(17, 0));
        assert!(free.show_indicator);
    }

    /// Multi-segment wires report the index of the segment that was hit.
    #[test]
    fn segment_snap_reports_segment_index() {
        let engine = SnapEngine::default();
        let wires = [Wire::new(
            3,
            vec![Point::origin(), Point::new(10, 0), Point::new(10, 10)],
        )];
        let result = engine.find_snap_target(Point::new(11, 5), &[], &wires, &[]);
        assert_eq!(
            result.target_type(),
            Some(&SnapTargetType::WireSegment {
                wire_id: 3,
                segment_index: 1
            })
        );
        assert_eq!(result.snapped_position, Point::new(10, 5));
    }

    /// Diagonal (45-degree) segments snap to the perpendicular projection.
    #[test]
    fn diagonal_segment_snap() {
        let engine = SnapEngine::default();
        let wires = [Wire::segment(3, Point::origin(), Point::new(10, 10))];
        let result = engine.find_snap_target(Point::new(6, 4), &[], &wires, &[]);
        assert_eq!(
            result.target_type(),
            Some(&SnapTargetType::WireSegment {
                wire_id: 3,
                segment_index: 0
            })
        );
        assert_eq!(result.snapped_position, Point::new(5, 5));
    }
}
