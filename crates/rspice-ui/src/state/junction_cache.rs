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

            for point in endpoints.iter().flatten() {
                endpoints_processed += 1;

                // Skip points at component terminals
                if self.terminal_positions.contains(point) {
                    continue;
                }

                let entry = self.junctions.entry(**point).or_default();
                entry.segment_count += 1;
                entry.wire_ids.push(wire.id);
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
            let entry = self.junctions.entry(*pos).or_default();
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
        self.junctions.get(point).is_some_and(|d| d.is_junction())
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

