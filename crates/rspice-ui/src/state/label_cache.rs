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

