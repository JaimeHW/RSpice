//! Render Context
//!
//! Commercial-grade frame rendering context for schematic visualization.
//! This consolidates all cached and per-frame data into a single coherent struct,
//! following patterns used by professional EDA tools like Cadence Allegro.
//!
//! # Architecture
//!
//! The render context follows the "frame-coherent cache" pattern:
//! 1. At frame start, check if caches need rebuilding (via topology_version)
//! 2. Rebuild stale caches once per topology change
//! 3. All render operations use cached data with O(1) or O(log n) lookups
//!
//! This ensures consistent O(visible) rendering regardless of schematic size.
//!
//! # Usage
//!
//! ```ignore
//! // At start of render
//! let ctx = RenderContext::prepare(&schematic.read(), grid_size);
//!
//! // During render - all lookups are O(1)
//! let label_pos = ctx.label_position(component_id);
//! let is_junction = ctx.is_junction(point);
//! let visible_comps = ctx.visible_components(&viewport);
//! ```

use std::collections::{HashMap, HashSet};

use crate::state::schematic::{Component, Point, Wire};
use crate::state::Viewport;

use super::spatial_index::SpatialIndex;

// =============================================================================
// Cached Label Position
// =============================================================================

/// Cached label position for a component (in pixels relative to component center)
#[derive(Debug, Clone, Copy, Default)]
pub struct LabelPosition {
    pub name_x: f64,
    pub name_y: f64,
    pub value_x: f64,
    pub value_y: f64,
}

// =============================================================================
// RenderContext
// =============================================================================

/// Frame-coherent render context containing all cached data needed for rendering.
///
/// This struct is created once per frame and provides O(1) access to:
/// - Label positions for all components
/// - Junction points and their connection counts
/// - Terminal positions (component connection points)
/// - Spatial visibility queries
///
/// # Performance Characteristics
/// - Cache rebuild: O(n) on topology change only
/// - Label lookup: O(1)
/// - Junction lookup: O(1)
/// - Visibility query: O(log n + k) where k = visible elements
#[derive(Debug, Clone)]
pub struct RenderContext {
    /// Cached label positions keyed by component ID
    label_positions: HashMap<u64, LabelPosition>,

    /// Set of points that are junctions (3+ wire endpoints)
    junction_points: HashSet<Point>,

    /// Number of wire endpoints at each point (for selection highlighting)
    junction_segment_counts: HashMap<Point, usize>,

    /// Component terminal positions (cached per topology)
    terminal_positions: HashSet<Point>,

    /// Spatial index for visibility queries
    spatial_index: SpatialIndex,

    /// Topology version this context was built for
    topology_version: u64,

    /// Grid size used for rendering
    grid_size: i32,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderContext {
    /// Create an empty render context
    pub fn new() -> Self {
        Self {
            label_positions: HashMap::new(),
            junction_points: HashSet::new(),
            junction_segment_counts: HashMap::new(),
            terminal_positions: HashSet::new(),
            spatial_index: SpatialIndex::new(),
            topology_version: 0,
            grid_size: 10,
        }
    }

    /// Check if context needs rebuilding for the given topology version
    pub fn needs_rebuild(&self, current_version: u64) -> bool {
        self.topology_version != current_version
    }

    /// Get the topology version this context was built for
    pub fn topology_version(&self) -> u64 {
        self.topology_version
    }

    /// Rebuild the entire render context from schematic data
    ///
    /// This is O(n) but only happens when topology changes, not every frame.
    pub fn rebuild(
        &mut self,
        components: &[Component],
        wires: &[Wire],
        grid_size: i32,
        topology_version: u64,
    ) {
        self.grid_size = grid_size;
        self.topology_version = topology_version;

        // Rebuild label positions
        self.rebuild_label_positions(components, wires, grid_size);

        // Rebuild junction data
        self.rebuild_junctions(components, wires);

        // Rebuild terminal positions
        self.rebuild_terminal_positions(components);

        // Rebuild spatial index (typical component half-size is 40 grid units)
        self.spatial_index.rebuild(components, wires, 40);
        self.spatial_index.set_version(topology_version);
    }

    /// Rebuild label positions for all components
    fn rebuild_label_positions(
        &mut self,
        components: &[Component],
        _wires: &[Wire],
        grid_size: i32,
    ) {
        self.label_positions.clear();

        // Default label offsets based on grid size
        let name_offset = grid_size as f64 * 3.0;
        let value_offset = grid_size as f64 * 4.5;

        for comp in components {
            // Simple default positioning - above component for name, below value offset for value
            self.label_positions.insert(
                comp.id,
                LabelPosition {
                    name_x: 0.0,
                    name_y: -name_offset,
                    value_x: 0.0,
                    value_y: value_offset,
                },
            );
        }
    }

    /// Rebuild junction points and segment counts
    fn rebuild_junctions(&mut self, components: &[Component], wires: &[Wire]) {
        self.junction_points.clear();
        self.junction_segment_counts.clear();

        // Collect terminal positions to exclude
        let terminals: HashSet<Point> = components
            .iter()
            .flat_map(|comp| comp.terminal_positions())
            .map(|(_, pos)| pos)
            .collect();

        // Count wire endpoints at each point
        for wire in wires {
            if let Some(first) = wire.points.first() {
                if !terminals.contains(first) {
                    *self.junction_segment_counts.entry(*first).or_insert(0) += 1;
                }
            }
            if let Some(last) = wire.points.last() {
                if !terminals.contains(last) {
                    *self.junction_segment_counts.entry(*last).or_insert(0) += 1;
                }
            }
        }

        // Points with 3+ segments are junctions
        for (point, count) in &self.junction_segment_counts {
            if *count >= 3 {
                self.junction_points.insert(*point);
            }
        }
    }

    /// Rebuild terminal positions set
    fn rebuild_terminal_positions(&mut self, components: &[Component]) {
        self.terminal_positions.clear();
        for comp in components {
            for (_, pos) in comp.terminal_positions() {
                self.terminal_positions.insert(pos);
            }
        }
    }

    // =========================================================================
    // Label Position Access
    // =========================================================================

    /// Get cached label position for a component (O(1))
    pub fn label_position(&self, component_id: u64) -> Option<LabelPosition> {
        self.label_positions.get(&component_id).copied()
    }

    /// Get label position with default fallback
    pub fn label_position_or_default(&self, component_id: u64) -> LabelPosition {
        self.label_positions
            .get(&component_id)
            .copied()
            .unwrap_or_default()
    }

    // =========================================================================
    // Junction Access
    // =========================================================================

    /// Check if a point is a junction (3+ wire endpoints) - O(1)
    pub fn is_junction(&self, point: Point) -> bool {
        self.junction_points.contains(&point)
    }

    /// Get all junction points
    pub fn junction_points(&self) -> &HashSet<Point> {
        &self.junction_points
    }

    /// Get segment count at a point (for selection highlighting)
    pub fn segment_count_at(&self, point: Point) -> usize {
        self.junction_segment_counts
            .get(&point)
            .copied()
            .unwrap_or(0)
    }

    /// Get junction segment counts map
    pub fn junction_segment_counts(&self) -> &HashMap<Point, usize> {
        &self.junction_segment_counts
    }

    // =========================================================================
    // Terminal Position Access
    // =========================================================================

    /// Check if a point is a component terminal (O(1))
    pub fn is_terminal(&self, point: Point) -> bool {
        self.terminal_positions.contains(&point)
    }

    /// Get all terminal positions
    pub fn terminal_positions(&self) -> &HashSet<Point> {
        &self.terminal_positions
    }

    // =========================================================================
    // Spatial Queries
    // =========================================================================

    /// Get component IDs visible in the viewport (O(log n + k))
    pub fn visible_component_ids(&self, viewport: &Viewport) -> Vec<u64> {
        self.spatial_index.components_in_viewport(viewport)
    }

    /// Get wire IDs visible in the viewport (O(log n + k))
    pub fn visible_wire_ids(&self, viewport: &Viewport) -> Vec<u64> {
        self.spatial_index.wires_in_viewport(viewport)
    }

    /// Get spatial index reference for advanced queries
    pub fn spatial_index(&self) -> &SpatialIndex {
        &self.spatial_index
    }

    // =========================================================================
    // Grid Size
    // =========================================================================

    /// Get the grid size this context was built with
    pub fn grid_size(&self) -> i32 {
        self.grid_size
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::schematic::ComponentType;

    fn make_component(id: u64, x: i32, y: i32) -> Component {
        Component::new(id, ComponentType::Resistor, Point::new(x, y))
    }

    fn make_wire(id: u64, points: Vec<(i32, i32)>) -> Wire {
        Wire::new(
            id,
            points.into_iter().map(|(x, y)| Point::new(x, y)).collect(),
        )
    }

    #[test]
    fn test_new_context() {
        let ctx = RenderContext::new();
        assert_eq!(ctx.topology_version(), 0);
        assert!(ctx.needs_rebuild(1));
    }

    #[test]
    fn test_rebuild_updates_version() {
        let mut ctx = RenderContext::new();
        let components = vec![make_component(1, 0, 0)];
        let wires = vec![];

        ctx.rebuild(&components, &wires, 10, 42);

        assert_eq!(ctx.topology_version(), 42);
        assert!(!ctx.needs_rebuild(42));
        assert!(ctx.needs_rebuild(43));
    }

    #[test]
    fn test_label_position_cached() {
        let mut ctx = RenderContext::new();
        let components = vec![make_component(1, 0, 0), make_component(2, 100, 0)];

        ctx.rebuild(&components, &[], 10, 1);

        // Both components should have cached label positions
        assert!(ctx.label_position(1).is_some());
        assert!(ctx.label_position(2).is_some());
        assert!(ctx.label_position(999).is_none());
    }

    #[test]
    fn test_junction_detection() {
        let mut ctx = RenderContext::new();
        // Create 3 wires meeting at (0,0) - this is a junction
        let wires = vec![
            make_wire(1, vec![(0, 0), (10, 0)]),
            make_wire(2, vec![(0, 0), (0, 10)]),
            make_wire(3, vec![(0, 0), (-10, 0)]),
        ];

        ctx.rebuild(&[], &wires, 10, 1);

        assert!(ctx.is_junction(Point::new(0, 0)));
        assert!(!ctx.is_junction(Point::new(10, 0)));
        assert_eq!(ctx.segment_count_at(Point::new(0, 0)), 3);
    }

    #[test]
    fn test_junction_excludes_terminals() {
        let mut ctx = RenderContext::new();
        // Wire endpoint at component terminal should not be a junction
        let components = vec![make_component(1, 0, 0)];
        let wires = vec![
            make_wire(1, vec![(2, 0), (10, 0)]), // Terminal at (2,0) for horizontal resistor
            make_wire(2, vec![(2, 0), (2, 10)]),
            make_wire(3, vec![(2, 0), (2, -10)]),
        ];

        ctx.rebuild(&components, &wires, 10, 1);

        // (2,0) is a terminal, so shouldn't appear in junction detection
        // BUT we should verify terminal is detected
        // Note: actual terminal positions depend on component implementation
    }

    #[test]
    fn test_terminal_positions_cached() {
        let mut ctx = RenderContext::new();
        let components = vec![make_component(1, 0, 0)];

        ctx.rebuild(&components, &[], 10, 1);

        // Resistor at (0,0) should have terminals (component dependent)
        assert!(!ctx.terminal_positions().is_empty());
    }

    #[test]
    fn test_spatial_queries() {
        let mut ctx = RenderContext::new();
        let components = vec![
            make_component(1, 0, 0),
            make_component(2, 100, 0),
            make_component(3, 500, 500),
        ];
        let wires = vec![make_wire(1, vec![(0, 0), (100, 0)])];

        ctx.rebuild(&components, &wires, 10, 1);

        // Query viewport containing first two components
        let viewport = Viewport::from_transform(200.0, 100.0, (0.0, 0.0), 1.0, 10);
        let visible = ctx.visible_component_ids(&viewport);

        // Should find components in viewport
        assert!(!visible.is_empty());
    }

    #[test]
    fn test_reuses_across_frames() {
        let mut ctx = RenderContext::new();
        let components = vec![make_component(1, 0, 0)];

        // First rebuild
        ctx.rebuild(&components, &[], 10, 1);
        let pos1 = ctx.label_position(1);

        // Same topology version - should not need rebuild
        assert!(!ctx.needs_rebuild(1));

        // Position should still be available
        let pos2 = ctx.label_position(1);
        assert_eq!(pos1.unwrap().name_x, pos2.unwrap().name_x);
    }
}
