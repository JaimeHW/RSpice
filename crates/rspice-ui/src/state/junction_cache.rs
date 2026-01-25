//! Junction Cache
//!
//! High-performance caching system for schematic junction analysis.
//! This is a critical performance optimization for schematic rendering.
//!
//! Commercial-grade simulators pre-compute junction data because:
//! - Junction detection is O(n²) where n = number of wire endpoints
//! - Junction positions only change when wire topology changes
//! - Pan/zoom/hover operations should NOT trigger re-computation
//!
//! # Architecture
//!
//! The cache contains:
//! - Pre-computed junction points (where 3+ wire segments meet)
//! - Wire endpoint segment counts at each point
//! - Selection and probe highlight state per junction
//!
//! # Usage
//!
//! ```ignore
//! let mut cache = JunctionCache::new();
//! cache.rebuild(&wires, &components);
//!
//! // During render - O(1) lookup per junction
//! for (point, data) in cache.junctions() {
//!     render_junction_dot(point, data);
//! }
//! ```

use std::collections::{HashMap, HashSet};

use crate::state::schematic::Component;
use crate::state::schematic::Point;
use crate::state::schematic::Wire;

// =============================================================================
// JunctionData
// =============================================================================

/// Data for a single junction point
#[derive(Debug, Clone, Default)]
pub struct JunctionData {
    /// Number of wire endpoint segments meeting at this point
    pub segment_count: usize,

    /// Wire IDs that have endpoints at this junction
    pub wire_ids: Vec<u64>,

    /// Whether this is an explicit junction (user-placed)
    pub is_explicit: bool,
}

impl JunctionData {
    /// Create new junction data
    pub fn new() -> Self {
        Self {
            segment_count: 0,
            wire_ids: Vec::new(),
            is_explicit: false,
        }
    }

    /// Check if this is a proper junction (3+ wire segments meeting)
    pub fn is_junction(&self) -> bool {
        self.segment_count >= 3 || self.is_explicit
    }
}

// =============================================================================
// JunctionCache
// =============================================================================

/// Pre-computed junction analysis for efficient rendering
///
/// This cache stores all junction points and their properties,
/// allowing O(1) lookup during rendering instead of O(n²) computation.
#[derive(Debug, Clone, Default)]
pub struct JunctionCache {
    /// Junction data indexed by grid position
    junctions: HashMap<Point, JunctionData>,

    /// Terminal positions from components (excluded from junction rendering)
    terminal_positions: HashSet<Point>,

    /// Schematic topology version when cache was built
    cached_version: u64,

    /// Statistics for performance monitoring
    stats: JunctionCacheStats,
}

/// Cache performance statistics
#[derive(Debug, Clone, Default)]
pub struct JunctionCacheStats {
    /// Number of cache rebuilds
    pub rebuilds: u64,
    /// Total junction points computed in last rebuild
    pub junction_count: usize,
    /// Total wire endpoints processed in last rebuild
    pub endpoints_processed: usize,
}

impl JunctionCache {
    /// Create a new empty cache
    pub fn new() -> Self {
        Self {
            junctions: HashMap::new(),
            terminal_positions: HashSet::new(),
            cached_version: 0,
            stats: JunctionCacheStats::default(),
        }
    }

    /// Create cache with pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            junctions: HashMap::with_capacity(capacity),
            terminal_positions: HashSet::with_capacity(capacity / 2),
            cached_version: 0,
            stats: JunctionCacheStats::default(),
        }
    }

    /// Ensure cache is fresh for the given topology version
    ///
    /// Returns true if cache was invalidated and needs rebuild.
    pub fn ensure_fresh(&mut self, current_version: u64) -> bool {
        self.cached_version != current_version
    }

    /// Rebuild the junction cache from wires and components
    ///
    /// This is O(n) where n = total wire endpoints.
    pub fn rebuild(&mut self, wires: &[Wire], components: &[Component]) {
        self.junctions.clear();
        self.terminal_positions.clear();

        // Collect all component terminal positions
        for comp in components {
            for (_, pos) in comp.terminal_positions() {
                self.terminal_positions.insert(pos);
            }
        }

        // Count wire endpoint segments at each point
        let mut endpoints_processed = 0;
        for wire in wires {
            // Only endpoints contribute to junction count (first and last point)
            let endpoints = [wire.points.first(), wire.points.last()];

            for maybe_point in endpoints.iter() {
                if let Some(point) = maybe_point {
                    endpoints_processed += 1;

                    // Skip points at component terminals
                    if self.terminal_positions.contains(point) {
                        continue;
                    }

                    let entry = self
                        .junctions
                        .entry(**point)
                        .or_insert_with(JunctionData::new);
                    entry.segment_count += 1;
                    entry.wire_ids.push(wire.id);
                }
            }
        }

        // Update statistics
        self.stats.rebuilds += 1;
        self.stats.junction_count = self.junctions.values().filter(|j| j.is_junction()).count();
        self.stats.endpoints_processed = endpoints_processed;
    }

    /// Update cache version after rebuild
    pub fn set_version(&mut self, version: u64) {
        self.cached_version = version;
    }

    /// Mark explicit junctions from the schematic's junction list
    pub fn mark_explicit_junctions(&mut self, junction_positions: &[Point]) {
        for pos in junction_positions {
            let entry = self.junctions.entry(*pos).or_insert_with(JunctionData::new);
            entry.is_explicit = true;
        }
    }

    /// Get all junction points (3+ segments or explicit)
    pub fn junction_points(&self) -> impl Iterator<Item = (&Point, &JunctionData)> {
        self.junctions.iter().filter(|(_, data)| data.is_junction())
    }

    /// Get all junction points as a vector
    pub fn junction_points_vec(&self) -> Vec<(Point, JunctionData)> {
        self.junctions
            .iter()
            .filter(|(_, data)| data.is_junction())
            .map(|(p, d)| (*p, d.clone()))
            .collect()
    }

    /// Get junction data at a specific point
    pub fn get(&self, point: &Point) -> Option<&JunctionData> {
        self.junctions.get(point)
    }

    /// Check if a point is a junction
    pub fn is_junction(&self, point: &Point) -> bool {
        self.junctions.get(point).map_or(false, |d| d.is_junction())
    }

    /// Check if a point is at a component terminal
    pub fn is_terminal(&self, point: &Point) -> bool {
        self.terminal_positions.contains(point)
    }

    /// Get the cached topology version
    pub fn cached_version(&self) -> u64 {
        self.cached_version
    }

    /// Get cache statistics
    pub fn stats(&self) -> &JunctionCacheStats {
        &self.stats
    }

    /// Number of junction points
    pub fn len(&self) -> usize {
        self.junctions.values().filter(|j| j.is_junction()).count()
    }

    /// Check if cache has no junctions
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.junctions.clear();
        self.terminal_positions.clear();
    }

    /// Get junctions visible in the given viewport bounds
    ///
    /// Filters to only junctions within (min_x, min_y) to (max_x, max_y).
    pub fn visible_junctions(
        &self,
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
    ) -> Vec<(Point, &JunctionData)> {
        self.junctions
            .iter()
            .filter(|(p, data)| {
                data.is_junction() && p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y
            })
            .map(|(p, d)| (*p, d))
            .collect()
    }

    /// Compute selection state for junctions based on wire selection
    ///
    /// Returns a map of junction point -> is_all_selected
    pub fn compute_selection_state(&self, selected_wires: &HashSet<u64>) -> HashMap<Point, bool> {
        let mut result = HashMap::new();

        for (point, data) in &self.junctions {
            if data.is_junction() {
                let selected_count = data
                    .wire_ids
                    .iter()
                    .filter(|id| selected_wires.contains(id))
                    .count();
                let all_selected = selected_count >= data.segment_count;
                result.insert(*point, all_selected);
            }
        }

        result
    }

    /// Compute probe highlight state for junctions
    ///
    /// Returns set of junction points that should be highlighted
    pub fn compute_probe_state(&self, probe_wires: &HashSet<u64>) -> HashSet<Point> {
        if probe_wires.is_empty() {
            return HashSet::new();
        }

        self.junctions
            .iter()
            .filter(|(_, data)| {
                data.is_junction() && data.wire_ids.iter().any(|id| probe_wires.contains(id))
            })
            .map(|(p, _)| *p)
            .collect()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::schematic::Wire;

    /// Helper to create a wire with given points
    fn make_wire(id: u64, points: Vec<(i32, i32)>) -> Wire {
        Wire::new(
            id,
            points.into_iter().map(|(x, y)| Point::new(x, y)).collect(),
        )
    }

    // =========================================================================
    // Construction Tests
    // =========================================================================

    #[test]
    fn test_new_cache_is_empty() {
        let cache = JunctionCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.cached_version(), 0);
    }

    #[test]
    fn test_with_capacity_is_empty() {
        let cache = JunctionCache::with_capacity(100);
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_default_equals_new() {
        let default_cache = JunctionCache::default();
        let new_cache = JunctionCache::new();
        assert_eq!(default_cache.len(), new_cache.len());
        assert_eq!(default_cache.cached_version(), new_cache.cached_version());
    }

    // =========================================================================
    // Junction Detection Tests
    // =========================================================================

    #[test]
    fn test_no_junctions_with_single_wire() {
        let mut cache = JunctionCache::new();
        let wires = vec![make_wire(1, vec![(0, 0), (10, 0)])];

        cache.rebuild(&wires, &[]);

        // Single wire has no junctions (only 1 segment at each endpoint)
        assert!(cache.is_empty());
    }

    #[test]
    fn test_no_junction_with_two_separate_wires() {
        let mut cache = JunctionCache::new();
        let wires = vec![
            make_wire(1, vec![(0, 0), (10, 0)]),
            make_wire(2, vec![(20, 0), (30, 0)]),
        ];

        cache.rebuild(&wires, &[]);

        assert!(cache.is_empty());
    }

    #[test]
    fn test_no_junction_with_two_connected_wires() {
        let mut cache = JunctionCache::new();
        // Two wires sharing an endpoint = 2 segments, not a junction
        let wires = vec![
            make_wire(1, vec![(0, 0), (10, 0)]),
            make_wire(2, vec![(10, 0), (20, 0)]),
        ];

        cache.rebuild(&wires, &[]);

        // 2 segments at (10, 0) - not a junction (need 3+)
        assert!(cache.is_empty());
    }

    #[test]
    fn test_junction_with_three_wires() {
        let mut cache = JunctionCache::new();
        // Three wires meeting at (10, 0) = 3 segments = junction
        let wires = vec![
            make_wire(1, vec![(0, 0), (10, 0)]),
            make_wire(2, vec![(10, 0), (20, 0)]),
            make_wire(3, vec![(10, 0), (10, 10)]),
        ];

        cache.rebuild(&wires, &[]);

        assert_eq!(cache.len(), 1);
        assert!(cache.is_junction(&Point::new(10, 0)));

        let data = cache.get(&Point::new(10, 0)).unwrap();
        assert_eq!(data.segment_count, 3);
        assert!(data.wire_ids.contains(&1));
        assert!(data.wire_ids.contains(&2));
        assert!(data.wire_ids.contains(&3));
    }

    #[test]
    fn test_junction_with_four_wires() {
        let mut cache = JunctionCache::new();
        // Four-way junction (crossroads)
        let wires = vec![
            make_wire(1, vec![(-10, 0), (0, 0)]),
            make_wire(2, vec![(0, 0), (10, 0)]),
            make_wire(3, vec![(0, -10), (0, 0)]),
            make_wire(4, vec![(0, 0), (0, 10)]),
        ];

        cache.rebuild(&wires, &[]);

        assert_eq!(cache.len(), 1);
        let data = cache.get(&Point::new(0, 0)).unwrap();
        assert_eq!(data.segment_count, 4);
    }

    #[test]
    fn test_multiple_junctions() {
        let mut cache = JunctionCache::new();
        // Two T-junctions
        let wires = vec![
            // Junction at (0, 0)
            make_wire(1, vec![(-10, 0), (0, 0)]),
            make_wire(2, vec![(0, 0), (10, 0)]),
            make_wire(3, vec![(0, 0), (0, 10)]),
            // Junction at (20, 0)
            make_wire(4, vec![(10, 0), (20, 0)]),
            make_wire(5, vec![(20, 0), (30, 0)]),
            make_wire(6, vec![(20, 0), (20, 10)]),
        ];

        cache.rebuild(&wires, &[]);

        assert_eq!(cache.len(), 2);
        assert!(cache.is_junction(&Point::new(0, 0)));
        assert!(cache.is_junction(&Point::new(20, 0)));
    }

    // =========================================================================
    // Wire Endpoint Tests (not middle points)
    // =========================================================================

    #[test]
    fn test_middle_points_not_counted() {
        let mut cache = JunctionCache::new();
        // Wire with corner point at (10, 0) - the middle point should NOT count
        let wires = vec![
            make_wire(1, vec![(0, 0), (10, 0), (10, 10)]),
            make_wire(2, vec![(10, 0), (20, 0)]),
        ];

        cache.rebuild(&wires, &[]);

        // Only endpoints count, so (10, 0) has 2 segments (wire 1 endpoint + wire 2 endpoint)
        // Actually, wire 1's endpoints are (0,0) and (10,10), not (10,0)
        // So (10, 0) only has 1 segment from wire 2
        assert!(cache.is_empty());
    }

    // =========================================================================
    // Explicit Junction Tests
    // =========================================================================

    #[test]
    fn test_explicit_junction() {
        let mut cache = JunctionCache::new();
        let wires = vec![make_wire(1, vec![(0, 0), (10, 0)])];

        cache.rebuild(&wires, &[]);
        cache.mark_explicit_junctions(&[Point::new(5, 0)]);

        // Explicit junction should be marked
        assert_eq!(cache.len(), 1);
        assert!(cache.is_junction(&Point::new(5, 0)));

        let data = cache.get(&Point::new(5, 0)).unwrap();
        assert!(data.is_explicit);
    }

    #[test]
    fn test_explicit_junction_at_existing_point() {
        let mut cache = JunctionCache::new();
        let wires = vec![
            make_wire(1, vec![(0, 0), (10, 0)]),
            make_wire(2, vec![(10, 0), (20, 0)]),
        ];

        cache.rebuild(&wires, &[]);
        cache.mark_explicit_junctions(&[Point::new(10, 0)]);

        // (10, 0) has 2 segments but is now explicit
        assert_eq!(cache.len(), 1);
        assert!(cache.is_junction(&Point::new(10, 0)));
    }

    // =========================================================================
    // Version/Freshness Tests
    // =========================================================================

    #[test]
    fn test_ensure_fresh_same_version() {
        let mut cache = JunctionCache::new();
        cache.set_version(5);

        assert!(!cache.ensure_fresh(5)); // Same version - fresh
    }

    #[test]
    fn test_ensure_fresh_different_version() {
        let mut cache = JunctionCache::new();
        cache.set_version(5);

        assert!(cache.ensure_fresh(6)); // Different version - stale
    }

    // =========================================================================
    // Visibility Filtering Tests
    // =========================================================================

    #[test]
    fn test_visible_junctions() {
        let mut cache = JunctionCache::new();
        let wires = vec![
            // Junction at (0, 0)
            make_wire(1, vec![(-10, 0), (0, 0)]),
            make_wire(2, vec![(0, 0), (10, 0)]),
            make_wire(3, vec![(0, 0), (0, 10)]),
            // Junction at (100, 100) - outside viewport
            make_wire(4, vec![(90, 100), (100, 100)]),
            make_wire(5, vec![(100, 100), (110, 100)]),
            make_wire(6, vec![(100, 100), (100, 110)]),
        ];

        cache.rebuild(&wires, &[]);

        // Viewport from (-20, -20) to (50, 50)
        let visible = cache.visible_junctions(-20, -20, 50, 50);

        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].0, Point::new(0, 0));
    }

    // =========================================================================
    // Selection State Tests
    // =========================================================================

    #[test]
    fn test_selection_state_none_selected() {
        let mut cache = JunctionCache::new();
        let wires = vec![
            make_wire(1, vec![(-10, 0), (0, 0)]),
            make_wire(2, vec![(0, 0), (10, 0)]),
            make_wire(3, vec![(0, 0), (0, 10)]),
        ];

        cache.rebuild(&wires, &[]);

        let selection_state = cache.compute_selection_state(&HashSet::new());

        assert_eq!(selection_state.get(&Point::new(0, 0)), Some(&false));
    }

    #[test]
    fn test_selection_state_partial_selected() {
        let mut cache = JunctionCache::new();
        let wires = vec![
            make_wire(1, vec![(-10, 0), (0, 0)]),
            make_wire(2, vec![(0, 0), (10, 0)]),
            make_wire(3, vec![(0, 0), (0, 10)]),
        ];

        cache.rebuild(&wires, &[]);

        let mut selected = HashSet::new();
        selected.insert(1);
        selected.insert(2);
        // Wire 3 not selected

        let selection_state = cache.compute_selection_state(&selected);

        assert_eq!(selection_state.get(&Point::new(0, 0)), Some(&false));
    }

    #[test]
    fn test_selection_state_all_selected() {
        let mut cache = JunctionCache::new();
        let wires = vec![
            make_wire(1, vec![(-10, 0), (0, 0)]),
            make_wire(2, vec![(0, 0), (10, 0)]),
            make_wire(3, vec![(0, 0), (0, 10)]),
        ];

        cache.rebuild(&wires, &[]);

        let mut selected = HashSet::new();
        selected.insert(1);
        selected.insert(2);
        selected.insert(3);

        let selection_state = cache.compute_selection_state(&selected);

        assert_eq!(selection_state.get(&Point::new(0, 0)), Some(&true));
    }

    // =========================================================================
    // Probe State Tests
    // =========================================================================

    #[test]
    fn test_probe_state_empty() {
        let mut cache = JunctionCache::new();
        let wires = vec![
            make_wire(1, vec![(-10, 0), (0, 0)]),
            make_wire(2, vec![(0, 0), (10, 0)]),
            make_wire(3, vec![(0, 0), (0, 10)]),
        ];

        cache.rebuild(&wires, &[]);

        let probe_state = cache.compute_probe_state(&HashSet::new());

        assert!(probe_state.is_empty());
    }

    #[test]
    fn test_probe_state_with_probed_wire() {
        let mut cache = JunctionCache::new();
        let wires = vec![
            make_wire(1, vec![(-10, 0), (0, 0)]),
            make_wire(2, vec![(0, 0), (10, 0)]),
            make_wire(3, vec![(0, 0), (0, 10)]),
        ];

        cache.rebuild(&wires, &[]);

        let mut probed = HashSet::new();
        probed.insert(1);

        let probe_state = cache.compute_probe_state(&probed);

        assert!(probe_state.contains(&Point::new(0, 0)));
    }

    // =========================================================================
    // Statistics Tests
    // =========================================================================

    #[test]
    fn test_stats_initial() {
        let cache = JunctionCache::new();
        let stats = cache.stats();

        assert_eq!(stats.rebuilds, 0);
        assert_eq!(stats.junction_count, 0);
        assert_eq!(stats.endpoints_processed, 0);
    }

    #[test]
    fn test_stats_after_rebuild() {
        let mut cache = JunctionCache::new();
        let wires = vec![
            make_wire(1, vec![(-10, 0), (0, 0)]),
            make_wire(2, vec![(0, 0), (10, 0)]),
            make_wire(3, vec![(0, 0), (0, 10)]),
        ];

        cache.rebuild(&wires, &[]);
        let stats = cache.stats();

        assert_eq!(stats.rebuilds, 1);
        assert_eq!(stats.junction_count, 1);
        assert_eq!(stats.endpoints_processed, 6); // 3 wires × 2 endpoints
    }

    #[test]
    fn test_stats_multiple_rebuilds() {
        let mut cache = JunctionCache::new();
        let wires = vec![make_wire(1, vec![(0, 0), (10, 0)])];

        cache.rebuild(&wires, &[]);
        cache.rebuild(&wires, &[]);
        cache.rebuild(&wires, &[]);

        assert_eq!(cache.stats().rebuilds, 3);
    }

    // =========================================================================
    // Clear Tests
    // =========================================================================

    #[test]
    fn test_clear() {
        let mut cache = JunctionCache::new();
        let wires = vec![
            make_wire(1, vec![(-10, 0), (0, 0)]),
            make_wire(2, vec![(0, 0), (10, 0)]),
            make_wire(3, vec![(0, 0), (0, 10)]),
        ];

        cache.rebuild(&wires, &[]);
        assert!(!cache.is_empty());

        cache.clear();

        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_empty_wires() {
        let mut cache = JunctionCache::new();
        cache.rebuild(&[], &[]);

        assert!(cache.is_empty());
    }

    #[test]
    fn test_single_point_wire() {
        let mut cache = JunctionCache::new();
        // Edge case: wire with only one point
        let wires = vec![Wire::new(1, vec![Point::new(0, 0)])];

        cache.rebuild(&wires, &[]);

        // Single point has endpoints at same location - counts as 1 segment
        assert!(cache.is_empty());
    }

    #[test]
    fn test_duplicate_wire_endpoints() {
        let mut cache = JunctionCache::new();
        // Multiple wires all ending at same point
        let wires = vec![
            make_wire(1, vec![(0, 0), (10, 10)]),
            make_wire(2, vec![(0, 0), (10, 10)]),
            make_wire(3, vec![(0, 0), (10, 10)]),
        ];

        cache.rebuild(&wires, &[]);

        // Both (0, 0) and (10, 10) should have 3 segments
        assert_eq!(cache.len(), 2);
        assert!(cache.is_junction(&Point::new(0, 0)));
        assert!(cache.is_junction(&Point::new(10, 10)));
    }
}
