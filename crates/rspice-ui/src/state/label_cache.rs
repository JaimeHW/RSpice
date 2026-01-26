//! Label Position Cache
//!
//! High-performance caching system for component label positions.
//! This is a critical performance optimization for schematic rendering.
//!
//! Commercial-grade simulators cache label positions because:
//! - Label collision detection is O(n × m) where n=components, m=wires
//! - Label positions only change when schematic topology changes
//! - Pan/zoom/hover operations should NOT trigger re-computation
//!
//! # Architecture
//!
//! The cache uses a version-based invalidation strategy:
//! - Each SchematicState has a `topology_version` counter
//! - The counter increments on any topology change (add/remove/move)
//! - Cache is invalidated when versions mismatch

use std::collections::HashMap;

// =============================================================================
// ComputedLabelPos - Inline definition (was in deleted views module)
// =============================================================================

/// Computed label position in pixels relative to component center
#[derive(Debug, Clone, Copy, Default)]
pub struct ComputedLabelPos {
    /// X offset from component center in pixels
    pub x: f64,
    /// Y offset from component center in pixels
    pub y: f64,
}

// =============================================================================
// LabelPositionCache
// =============================================================================

/// Cached label positions for high-performance rendering
///
/// Stores computed name and value label positions for each component,
/// indexed by component ID. The cache is invalidated when the schematic
/// topology changes (version mismatch).
#[derive(Debug, Clone, Default)]
pub struct LabelPositionCache {
    /// Cached positions: component_id -> (name_pos, value_pos)
    cache: HashMap<u64, (ComputedLabelPos, ComputedLabelPos)>,

    /// Schematic topology version when cache was last rebuilt
    cached_version: u64,

    /// Statistics for performance monitoring
    stats: CacheStats,
}

/// Cache performance statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses (required computation)
    pub misses: u64,
    /// Number of cache invalidations
    pub invalidations: u64,
}

impl CacheStats {
    /// Calculate hit rate as percentage
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            (self.hits as f64 / total as f64) * 100.0
        }
    }

    /// Reset statistics
    pub fn reset(&mut self) {
        self.hits = 0;
        self.misses = 0;
        self.invalidations = 0;
    }
}

impl LabelPositionCache {
    /// Create a new empty cache
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            cached_version: 0,
            stats: CacheStats::default(),
        }
    }

    /// Create cache with pre-allocated capacity
    ///
    /// Use this when the approximate number of components is known
    /// to avoid HashMap reallocations.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(capacity),
            cached_version: 0,
            stats: CacheStats::default(),
        }
    }

    /// Ensure cache is fresh for the given topology version
    ///
    /// If the version has changed, the cache is cleared.
    /// Returns true if cache was invalidated.
    pub fn ensure_fresh(&mut self, current_version: u64) -> bool {
        if self.cached_version != current_version {
            self.invalidate(current_version);
            true
        } else {
            false
        }
    }

    /// Invalidate the cache and update to new version
    pub fn invalidate(&mut self, new_version: u64) {
        self.cache.clear();
        self.cached_version = new_version;
        self.stats.invalidations += 1;
    }

    /// Get cached position or compute and store it
    ///
    /// This is the primary API for label position lookup.
    /// The compute function is only called on cache miss.
    pub fn get_or_compute<F>(
        &mut self,
        component_id: u64,
        compute: F,
    ) -> (ComputedLabelPos, ComputedLabelPos)
    where
        F: FnOnce() -> (ComputedLabelPos, ComputedLabelPos),
    {
        if let Some(cached) = self.cache.get(&component_id) {
            self.stats.hits += 1;
            *cached
        } else {
            self.stats.misses += 1;
            let positions = compute();
            self.cache.insert(component_id, positions);
            positions
        }
    }

    /// Get cached position if available (no computation)
    pub fn get(&self, component_id: u64) -> Option<(ComputedLabelPos, ComputedLabelPos)> {
        self.cache.get(&component_id).copied()
    }

    /// Insert a position into the cache
    pub fn insert(&mut self, component_id: u64, positions: (ComputedLabelPos, ComputedLabelPos)) {
        self.cache.insert(component_id, positions);
    }

    /// Remove a component from the cache
    ///
    /// Useful when a single component is modified without full invalidation.
    pub fn remove(&mut self, component_id: u64) {
        self.cache.remove(&component_id);
    }

    /// Number of cached entries
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Get the cached topology version
    pub fn cached_version(&self) -> u64 {
        self.cached_version
    }

    /// Get cache statistics
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Reset cache statistics
    pub fn reset_stats(&mut self) {
        self.stats.reset();
    }

    /// Clear all cached entries without changing version
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a test ComputedLabelPos
    fn make_pos(x: f64, y: f64) -> ComputedLabelPos {
        ComputedLabelPos { x, y }
    }

    // =========================================================================
    // Construction Tests
    // =========================================================================

    #[test]
    fn test_new_cache_is_empty() {
        let cache = LabelPositionCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.cached_version(), 0);
    }

    #[test]
    fn test_with_capacity_is_empty() {
        let cache = LabelPositionCache::with_capacity(100);
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_default_equals_new() {
        let default_cache = LabelPositionCache::default();
        let new_cache = LabelPositionCache::new();
        assert_eq!(default_cache.len(), new_cache.len());
        assert_eq!(default_cache.cached_version(), new_cache.cached_version());
    }

    // =========================================================================
    // Basic Operations Tests
    // =========================================================================

    #[test]
    fn test_insert_and_get() {
        let mut cache = LabelPositionCache::new();
        let positions = (make_pos(10.0, 20.0), make_pos(30.0, 40.0));

        cache.insert(42, positions);

        assert_eq!(cache.len(), 1);
        assert!(!cache.is_empty());

        let retrieved = cache.get(42);
        assert!(retrieved.is_some());
        let (name, value) = retrieved.unwrap();
        assert_eq!(name.x, 10.0);
        assert_eq!(name.y, 20.0);
        assert_eq!(value.x, 30.0);
        assert_eq!(value.y, 40.0);
    }

    #[test]
    fn test_get_nonexistent_returns_none() {
        let cache = LabelPositionCache::new();
        assert!(cache.get(999).is_none());
    }

    #[test]
    fn test_remove() {
        let mut cache = LabelPositionCache::new();
        cache.insert(1, (make_pos(0.0, 0.0), make_pos(1.0, 1.0)));
        cache.insert(2, (make_pos(2.0, 2.0), make_pos(3.0, 3.0)));

        assert_eq!(cache.len(), 2);

        cache.remove(1);

        assert_eq!(cache.len(), 1);
        assert!(cache.get(1).is_none());
        assert!(cache.get(2).is_some());
    }

    #[test]
    fn test_remove_nonexistent_is_safe() {
        let mut cache = LabelPositionCache::new();
        cache.remove(999); // Should not panic
        assert!(cache.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut cache = LabelPositionCache::new();
        cache.insert(1, (make_pos(0.0, 0.0), make_pos(1.0, 1.0)));
        cache.insert(2, (make_pos(2.0, 2.0), make_pos(3.0, 3.0)));
        cache.cached_version = 5;

        cache.clear();

        assert!(cache.is_empty());
        // Version should NOT change on clear()
        assert_eq!(cache.cached_version(), 5);
    }

    // =========================================================================
    // Version-Based Invalidation Tests
    // =========================================================================

    #[test]
    fn test_ensure_fresh_same_version_no_invalidation() {
        let mut cache = LabelPositionCache::new();
        cache.insert(1, (make_pos(0.0, 0.0), make_pos(1.0, 1.0)));
        cache.cached_version = 5;

        let invalidated = cache.ensure_fresh(5);

        assert!(!invalidated);
        assert_eq!(cache.len(), 1);
        assert!(cache.get(1).is_some());
    }

    #[test]
    fn test_ensure_fresh_different_version_invalidates() {
        let mut cache = LabelPositionCache::new();
        cache.insert(1, (make_pos(0.0, 0.0), make_pos(1.0, 1.0)));
        cache.insert(2, (make_pos(2.0, 2.0), make_pos(3.0, 3.0)));
        cache.cached_version = 5;

        let invalidated = cache.ensure_fresh(6);

        assert!(invalidated);
        assert!(cache.is_empty());
        assert_eq!(cache.cached_version(), 6);
    }

    #[test]
    fn test_invalidate_clears_and_updates_version() {
        let mut cache = LabelPositionCache::new();
        cache.insert(1, (make_pos(0.0, 0.0), make_pos(1.0, 1.0)));
        cache.cached_version = 1;

        cache.invalidate(10);

        assert!(cache.is_empty());
        assert_eq!(cache.cached_version(), 10);
    }

    #[test]
    fn test_version_can_decrease() {
        // Edge case: version might decrease on undo
        let mut cache = LabelPositionCache::new();
        cache.cached_version = 10;

        cache.ensure_fresh(5);

        assert_eq!(cache.cached_version(), 5);
    }

    // =========================================================================
    // get_or_compute Tests
    // =========================================================================

    #[test]
    fn test_get_or_compute_cache_miss() {
        let mut cache = LabelPositionCache::new();
        let mut computed = false;

        let result = cache.get_or_compute(42, || {
            computed = true;
            (make_pos(10.0, 20.0), make_pos(30.0, 40.0))
        });

        assert!(computed);
        assert_eq!(result.0.x, 10.0);
        assert_eq!(result.0.y, 20.0);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_get_or_compute_cache_hit() {
        let mut cache = LabelPositionCache::new();
        cache.insert(42, (make_pos(10.0, 20.0), make_pos(30.0, 40.0)));

        let mut computed = false;

        let result = cache.get_or_compute(42, || {
            computed = true; // Should NOT be called
            (make_pos(999.0, 999.0), make_pos(999.0, 999.0))
        });

        assert!(!computed);
        assert_eq!(result.0.x, 10.0);
        assert_eq!(result.0.y, 20.0);
    }

    #[test]
    fn test_get_or_compute_stores_result() {
        let mut cache = LabelPositionCache::new();

        // First call - computes and stores
        cache.get_or_compute(1, || (make_pos(5.0, 5.0), make_pos(10.0, 10.0)));

        // Verify stored
        let cached = cache.get(1);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().0.x, 5.0);
    }

    // =========================================================================
    // Statistics Tests
    // =========================================================================

    #[test]
    fn test_stats_initial_values() {
        let cache = LabelPositionCache::new();
        let stats = cache.stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.invalidations, 0);
    }

    #[test]
    fn test_stats_hits_and_misses() {
        let mut cache = LabelPositionCache::new();

        // Miss
        cache.get_or_compute(1, || (make_pos(0.0, 0.0), make_pos(1.0, 1.0)));
        assert_eq!(cache.stats().misses, 1);
        assert_eq!(cache.stats().hits, 0);

        // Hit
        cache.get_or_compute(1, || (make_pos(999.0, 999.0), make_pos(999.0, 999.0)));
        assert_eq!(cache.stats().misses, 1);
        assert_eq!(cache.stats().hits, 1);

        // Another miss
        cache.get_or_compute(2, || (make_pos(2.0, 2.0), make_pos(3.0, 3.0)));
        assert_eq!(cache.stats().misses, 2);
        assert_eq!(cache.stats().hits, 1);
    }

    #[test]
    fn test_stats_invalidations() {
        let mut cache = LabelPositionCache::new();

        cache.ensure_fresh(1);
        assert_eq!(cache.stats().invalidations, 1);

        cache.ensure_fresh(1); // Same version - no invalidation
        assert_eq!(cache.stats().invalidations, 1);

        cache.ensure_fresh(2);
        assert_eq!(cache.stats().invalidations, 2);

        cache.invalidate(3);
        assert_eq!(cache.stats().invalidations, 3);
    }

    #[test]
    fn test_hit_rate_calculation() {
        let mut stats = CacheStats::default();
        assert_eq!(stats.hit_rate(), 0.0);

        stats.hits = 75;
        stats.misses = 25;
        assert!((stats.hit_rate() - 75.0).abs() < 0.001);

        stats.hits = 0;
        stats.misses = 100;
        assert_eq!(stats.hit_rate(), 0.0);

        stats.hits = 100;
        stats.misses = 0;
        assert!((stats.hit_rate() - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_reset_stats() {
        let mut cache = LabelPositionCache::new();
        cache.get_or_compute(1, || (make_pos(0.0, 0.0), make_pos(1.0, 1.0)));
        cache.get_or_compute(1, || (make_pos(0.0, 0.0), make_pos(1.0, 1.0)));
        cache.invalidate(1);

        cache.reset_stats();

        assert_eq!(cache.stats().hits, 0);
        assert_eq!(cache.stats().misses, 0);
        assert_eq!(cache.stats().invalidations, 0);
    }

    // =========================================================================
    // Multiple Components Tests
    // =========================================================================

    #[test]
    fn test_multiple_components() {
        let mut cache = LabelPositionCache::new();

        for i in 0..100 {
            cache.insert(
                i,
                (
                    make_pos(i as f64, i as f64 * 2.0),
                    make_pos(i as f64 * 3.0, i as f64 * 4.0),
                ),
            );
        }

        assert_eq!(cache.len(), 100);

        // Verify some entries
        let pos50 = cache.get(50).unwrap();
        assert_eq!(pos50.0.x, 50.0);
        assert_eq!(pos50.0.y, 100.0);
        assert_eq!(pos50.1.x, 150.0);
        assert_eq!(pos50.1.y, 200.0);
    }

    #[test]
    fn test_overwrite_existing() {
        let mut cache = LabelPositionCache::new();

        cache.insert(1, (make_pos(1.0, 1.0), make_pos(2.0, 2.0)));
        cache.insert(1, (make_pos(10.0, 10.0), make_pos(20.0, 20.0)));

        assert_eq!(cache.len(), 1);
        let pos = cache.get(1).unwrap();
        assert_eq!(pos.0.x, 10.0);
    }

    // =========================================================================
    // Edge Cases Tests
    // =========================================================================

    #[test]
    fn test_zero_component_id() {
        let mut cache = LabelPositionCache::new();
        cache.insert(0, (make_pos(1.0, 2.0), make_pos(3.0, 4.0)));

        let pos = cache.get(0);
        assert!(pos.is_some());
        assert_eq!(pos.unwrap().0.x, 1.0);
    }

    #[test]
    fn test_max_component_id() {
        let mut cache = LabelPositionCache::new();
        cache.insert(u64::MAX, (make_pos(1.0, 2.0), make_pos(3.0, 4.0)));

        let pos = cache.get(u64::MAX);
        assert!(pos.is_some());
    }

    #[test]
    fn test_negative_positions() {
        let mut cache = LabelPositionCache::new();
        cache.insert(1, (make_pos(-100.5, -200.5), make_pos(-300.5, -400.5)));

        let pos = cache.get(1).unwrap();
        assert_eq!(pos.0.x, -100.5);
        assert_eq!(pos.0.y, -200.5);
    }

    #[test]
    fn test_special_float_values() {
        let mut cache = LabelPositionCache::new();

        // Infinity
        cache.insert(
            1,
            (
                make_pos(f64::INFINITY, f64::NEG_INFINITY),
                make_pos(0.0, 0.0),
            ),
        );
        let pos = cache.get(1).unwrap();
        assert!(pos.0.x.is_infinite());
        assert!(pos.0.y.is_infinite());

        // NaN - note: NaN != NaN, so we just check it exists
        cache.insert(2, (make_pos(f64::NAN, 0.0), make_pos(0.0, 0.0)));
        let pos2 = cache.get(2);
        assert!(pos2.is_some());
    }

    // =========================================================================
    // Performance Scenario Tests
    // =========================================================================

    #[test]
    fn test_realistic_render_loop_scenario() {
        let mut cache = LabelPositionCache::new();

        // Simulate initial render - all misses
        for i in 0..50 {
            cache.get_or_compute(i, || {
                (
                    make_pos(i as f64 * 10.0, -25.0),
                    make_pos(i as f64 * 10.0, 35.0),
                )
            });
        }
        assert_eq!(cache.stats().misses, 50);
        assert_eq!(cache.stats().hits, 0);

        // Simulate subsequent frame - all hits
        for i in 0..50 {
            cache.get_or_compute(i, || {
                (make_pos(999.0, 999.0), make_pos(999.0, 999.0)) // Should not be called
            });
        }
        assert_eq!(cache.stats().misses, 50);
        assert_eq!(cache.stats().hits, 50);
        assert!((cache.stats().hit_rate() - 50.0).abs() < 0.001);

        // Simulate topology change
        cache.ensure_fresh(1);
        assert!(cache.is_empty());
        assert_eq!(cache.stats().invalidations, 1);

        // Simulate re-render after change - all misses again
        for i in 0..50 {
            cache.get_or_compute(i, || {
                (
                    make_pos(i as f64 * 10.0, -25.0),
                    make_pos(i as f64 * 10.0, 35.0),
                )
            });
        }
        assert_eq!(cache.stats().misses, 100); // 50 + 50
    }

    #[test]
    fn test_partial_visibility_scenario() {
        let mut cache = LabelPositionCache::new();

        // Full schematic has 100 components
        // Viewport shows only 20 at a time

        // Frame 1: Show components 0-19
        for i in 0..20 {
            cache.get_or_compute(i, || (make_pos(i as f64, 0.0), make_pos(i as f64, 10.0)));
        }

        // Frame 2: Pan to show components 10-29 (10 cached, 10 new)
        for i in 10..30 {
            cache.get_or_compute(i, || (make_pos(i as f64, 0.0), make_pos(i as f64, 10.0)));
        }

        // Should have 10 hits (10-19 from frame 1) and 30 total misses
        assert_eq!(cache.stats().hits, 10);
        assert_eq!(cache.stats().misses, 30);
        assert_eq!(cache.len(), 30); // 0-29

        // Frame 3: Pan back to 0-19 (all cached now)
        for i in 0..20 {
            cache.get_or_compute(i, || (make_pos(999.0, 999.0), make_pos(999.0, 999.0)));
        }
        assert_eq!(cache.stats().hits, 30); // 10 + 20
    }

    // =========================================================================
    // Clone Tests
    // =========================================================================

    #[test]
    fn test_clone() {
        let mut cache = LabelPositionCache::new();
        cache.insert(1, (make_pos(1.0, 2.0), make_pos(3.0, 4.0)));
        cache.cached_version = 5;
        cache.get_or_compute(1, || (make_pos(0.0, 0.0), make_pos(0.0, 0.0))); // 1 hit

        let cloned = cache.clone();

        assert_eq!(cloned.len(), cache.len());
        assert_eq!(cloned.cached_version(), cache.cached_version());
        assert_eq!(cloned.stats().hits, cache.stats().hits);

        // Verify entries are cloned
        let pos = cloned.get(1).unwrap();
        assert_eq!(pos.0.x, 1.0);
    }
}
