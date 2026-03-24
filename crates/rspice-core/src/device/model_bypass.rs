//! Model Evaluation Bypass for Performance Optimization
//!
//! Implements latent device detection and stamp caching to skip expensive
//! device model evaluations when terminal voltages haven't changed significantly.
//!
//! # Overview
//!
//! During Newton-Raphson iteration, devices are evaluated repeatedly. For devices
//! whose terminal voltages haven't changed beyond a tolerance threshold, the
//! previous evaluation results can be reused. This optimization can provide
//! 2-5x speedup for large circuits.
//!
//! # Algorithm
//!
//! 1. Before device evaluation, check if all terminal voltages are within `reltol`
//!    of their values from the last evaluation
//! 2. If yes, device is "latent" - reuse cached conductances, currents, capacitances
//! 3. If no, device is "active" - perform full evaluation and cache results
//!
//! # Usage
//!
//! ```ignore
//! let mut bypass = ModelBypassManager::new(1e-3); // 0.1% relative tolerance
//!
//! // Before each device evaluation:
//! if bypass.should_bypass(device_id, &terminal_voltages) {
//!     let cached = bypass.get_cached(device_id);
//!     // Use cached values
//! } else {
//!     // Perform full evaluation
//!     let stamp = device.evaluate(...);
//!     bypass.cache_result(device_id, &terminal_voltages, stamp);
//! }
//! ```
//!
//! # References
//!
//! - Reference Manual: "Model Evaluation Bypassing"
//! - Nagel, L. "SPICE2: A Computer Program to Simulate Semiconductor Circuits"

use std::collections::HashMap;

use crate::Value;

//=============================================================================
// Device Identifiers
//=============================================================================

/// Unique identifier for a device in the circuit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceId(pub usize);

impl DeviceId {
    /// Create a new device ID
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

//=============================================================================
// Cached Stamp (Device Evaluation Result)
//=============================================================================

/// Cached device evaluation result for bypass reuse
///
/// Contains the linearized contributions a device makes to the circuit matrix.
/// For nonlinear devices (MOSFETs, diodes), this includes conductances and
/// currents. For dynamic devices, it also includes capacitances and charges.
#[derive(Debug, Clone, Default)]
pub struct CachedStamp {
    /// Conductance contributions (G matrix entries)
    /// Format: [(row, col, value), ...]
    pub conductances: Vec<(usize, usize, Value)>,

    /// Current contributions (RHS vector entries)
    /// Format: [(node, value), ...]
    pub currents: Vec<(usize, Value)>,

    /// Capacitance contributions (for transient analysis)
    /// Format: [(row, col, value), ...]
    pub capacitances: Vec<(usize, usize, Value)>,

    /// Charge values (for transient analysis with charge-based models)
    /// Format: [(node, charge), ...]
    pub charges: Vec<(usize, Value)>,

    /// Operating point values (for output/debugging)
    pub operating_point: OperatingPoint,
}

/// Device operating point for output/debugging
#[derive(Debug, Clone, Default)]
pub struct OperatingPoint {
    /// Drain/collector current for active devices
    pub id: Option<Value>,
    /// Transconductance
    pub gm: Option<Value>,
    /// Output conductance
    pub gds: Option<Value>,
    /// Gate-source capacitance
    pub cgs: Option<Value>,
    /// Gate-drain capacitance
    pub cgd: Option<Value>,
    /// Operating region (saturation, linear, cutoff, etc.)
    pub region: Option<String>,
}

impl CachedStamp {
    /// Create a new empty cached stamp
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a conductance entry
    pub fn add_conductance(&mut self, row: usize, col: usize, value: Value) {
        self.conductances.push((row, col, value));
    }

    /// Add a current entry
    pub fn add_current(&mut self, node: usize, value: Value) {
        self.currents.push((node, value));
    }

    /// Add a capacitance entry
    pub fn add_capacitance(&mut self, row: usize, col: usize, value: Value) {
        self.capacitances.push((row, col, value));
    }

    /// Add a charge entry
    pub fn add_charge(&mut self, node: usize, value: Value) {
        self.charges.push((node, value));
    }

    /// Clear all cached values
    pub fn clear(&mut self) {
        self.conductances.clear();
        self.currents.clear();
        self.capacitances.clear();
        self.charges.clear();
        self.operating_point = OperatingPoint::default();
    }
}

//=============================================================================
// Model Bypass Manager
//=============================================================================

/// Manager for model evaluation bypass optimization
///
/// Tracks device terminal voltages and caches evaluation results to skip
/// redundant calculations when voltages haven't changed significantly.
///
/// Simulators call this "Model Evaluation
/// Bypassing" and it provides significant speedup for large circuits.
#[derive(Debug)]
pub struct ModelBypassManager {
    /// Previous terminal voltages for each device
    /// Key: device ID, Value: terminal voltages from last evaluation
    prev_voltages: HashMap<DeviceId, Vec<Value>>,

    /// Cached device stamps (evaluation results)
    cached_stamps: HashMap<DeviceId, CachedStamp>,

    /// Relative tolerance for voltage change detection
    /// Devices are considered latent if all terminal voltage changes are below this
    reltol: Value,

    /// Absolute tolerance for small voltage detection
    /// Prevents division issues when voltages are near zero
    abstol: Value,

    /// Statistics for bypass effectiveness
    stats: BypassStats,

    /// Enable/disable bypass (for debugging or verification)
    enabled: bool,
}

/// Statistics for monitoring bypass effectiveness
#[derive(Debug, Default, Clone)]
pub struct BypassStats {
    /// Total device evaluations attempted
    pub total_evaluations: usize,
    /// Evaluations bypassed (used cache)
    pub bypassed: usize,
    /// Evaluations performed (cache miss or stale)
    pub computed: usize,
}

impl BypassStats {
    /// Calculate bypass ratio (percentage of evaluations skipped)
    pub fn bypass_ratio(&self) -> f64 {
        if self.total_evaluations == 0 {
            0.0
        } else {
            self.bypassed as f64 / self.total_evaluations as f64
        }
    }

    /// Reset statistics
    pub fn reset(&mut self) {
        self.total_evaluations = 0;
        self.bypassed = 0;
        self.computed = 0;
    }
}

impl ModelBypassManager {
    /// Create a new bypass manager with default tolerances
    ///
    /// Default tolerances match recommended values:
    /// - reltol: 1e-3 (0.1%)
    /// - abstol: 1e-12 (1pV)
    pub fn new() -> Self {
        Self {
            prev_voltages: HashMap::new(),
            cached_stamps: HashMap::new(),
            reltol: 1e-3,
            abstol: 1e-12,
            stats: BypassStats::default(),
            enabled: true,
        }
    }

    /// Create a bypass manager with custom tolerances
    pub fn with_tolerances(reltol: Value, abstol: Value) -> Self {
        Self {
            prev_voltages: HashMap::new(),
            cached_stamps: HashMap::new(),
            reltol,
            abstol,
            stats: BypassStats::default(),
            enabled: true,
        }
    }

    /// Enable or disable bypassing
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if bypass is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if a device can bypass evaluation based on voltage change
    ///
    /// Returns true if all terminal voltages are within tolerance of their
    /// previous values and a cached stamp exists.
    ///
    /// # Arguments
    /// * `device_id` - Unique identifier for the device
    /// * `voltages` - Current terminal voltages
    ///
    /// # Returns
    /// * `true` - Device is latent, can use cached stamp
    /// * `false` - Device is active, must recalculate
    pub fn should_bypass(&mut self, device_id: DeviceId, voltages: &[Value]) -> bool {
        self.stats.total_evaluations += 1;

        // Bypass disabled - always recalculate
        if !self.enabled {
            self.stats.computed += 1;
            return false;
        }

        // Check if we have previous voltages for this device
        let prev = match self.prev_voltages.get(&device_id) {
            Some(prev) => prev,
            None => {
                self.stats.computed += 1;
                return false;
            }
        };

        // Check if cached stamp exists
        if !self.cached_stamps.contains_key(&device_id) {
            self.stats.computed += 1;
            return false;
        }

        // Voltage arrays must match in length
        if prev.len() != voltages.len() {
            self.stats.computed += 1;
            return false;
        }

        // Check each terminal voltage for change within tolerance
        for (p, v) in prev.iter().zip(voltages.iter()) {
            let delta = (v - p).abs();
            let threshold = self.reltol * p.abs().max(v.abs()).max(self.abstol);

            if delta > threshold {
                // Voltage changed beyond tolerance - must recalculate
                self.stats.computed += 1;
                return false;
            }
        }

        // All voltages within tolerance - can bypass
        self.stats.bypassed += 1;
        true
    }

    /// Cache the result of a device evaluation
    ///
    /// Should be called after a full device evaluation to store results
    /// for potential future bypass.
    pub fn cache_result(&mut self, device_id: DeviceId, voltages: &[Value], stamp: CachedStamp) {
        self.prev_voltages.insert(device_id, voltages.to_vec());
        self.cached_stamps.insert(device_id, stamp);
    }

    /// Get the cached stamp for a device
    ///
    /// Returns None if no cached stamp exists (should call should_bypass first).
    pub fn get_cached(&self, device_id: DeviceId) -> Option<&CachedStamp> {
        self.cached_stamps.get(&device_id)
    }

    /// Get mutable reference to cached stamp
    pub fn get_cached_mut(&mut self, device_id: DeviceId) -> Option<&mut CachedStamp> {
        self.cached_stamps.get_mut(&device_id)
    }

    /// Invalidate cache for a specific device
    ///
    /// Use when device parameters change or at start of new analysis.
    pub fn invalidate(&mut self, device_id: DeviceId) {
        self.prev_voltages.remove(&device_id);
        self.cached_stamps.remove(&device_id);
    }

    /// Invalidate all cached data
    ///
    /// Use at start of new analysis or when global state changes.
    pub fn invalidate_all(&mut self) {
        self.prev_voltages.clear();
        self.cached_stamps.clear();
    }

    /// Get bypass statistics
    pub fn stats(&self) -> &BypassStats {
        &self.stats
    }

    /// Reset bypass statistics
    pub fn reset_stats(&mut self) {
        self.stats.reset();
    }

    /// Get the number of devices with cached stamps
    pub fn cached_device_count(&self) -> usize {
        self.cached_stamps.len()
    }

    /// Update tolerances
    pub fn set_tolerances(&mut self, reltol: Value, abstol: Value) {
        self.reltol = reltol;
        self.abstol = abstol;
    }

    /// Get current relative tolerance
    pub fn reltol(&self) -> Value {
        self.reltol
    }

    /// Get current absolute tolerance
    pub fn abstol(&self) -> Value {
        self.abstol
    }
}

impl Default for ModelBypassManager {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_id() {
        let id1 = DeviceId::new(0);
        let id2 = DeviceId::new(1);
        let id3 = DeviceId::new(0);

        assert_ne!(id1, id2);
        assert_eq!(id1, id3);
    }

    #[test]
    fn test_cached_stamp_creation() {
        let mut stamp = CachedStamp::new();

        stamp.add_conductance(0, 0, 1.0);
        stamp.add_conductance(0, 1, -1.0);
        stamp.add_current(0, 0.5);
        stamp.add_capacitance(0, 0, 1e-12);
        stamp.add_charge(0, 1e-15);

        assert_eq!(stamp.conductances.len(), 2);
        assert_eq!(stamp.currents.len(), 1);
        assert_eq!(stamp.capacitances.len(), 1);
        assert_eq!(stamp.charges.len(), 1);
    }

    #[test]
    fn test_bypass_manager_no_cache() {
        let mut bypass = ModelBypassManager::new();
        let device_id = DeviceId::new(0);
        let voltages = vec![1.0, 0.0, 0.5];

        // First evaluation - no cache, should not bypass
        assert!(!bypass.should_bypass(device_id, &voltages));
        assert_eq!(bypass.stats().computed, 1);
        assert_eq!(bypass.stats().bypassed, 0);
    }

    #[test]
    fn test_bypass_manager_with_cache_same_voltage() {
        let mut bypass = ModelBypassManager::new();
        let device_id = DeviceId::new(0);
        let voltages = vec![1.0, 0.0, 0.5];

        // Cache result
        let stamp = CachedStamp::new();
        bypass.cache_result(device_id, &voltages, stamp);

        // Same voltages - should bypass
        assert!(bypass.should_bypass(device_id, &voltages));
        assert_eq!(bypass.stats().bypassed, 1);
    }

    #[test]
    fn test_bypass_manager_voltage_changed() {
        let mut bypass = ModelBypassManager::new();
        let device_id = DeviceId::new(0);
        let voltages1 = vec![1.0, 0.0, 0.5];
        let voltages2 = vec![1.1, 0.0, 0.5]; // 10% change on first terminal

        // Cache result
        let stamp = CachedStamp::new();
        bypass.cache_result(device_id, &voltages1, stamp);

        // Voltage changed beyond tolerance - should not bypass
        assert!(!bypass.should_bypass(device_id, &voltages2));
        assert_eq!(bypass.stats().computed, 1);
    }

    #[test]
    fn test_bypass_manager_small_voltage_change() {
        let mut bypass = ModelBypassManager::with_tolerances(1e-3, 1e-12);
        let device_id = DeviceId::new(0);
        let voltages1 = vec![1.0, 0.0, 0.5];
        let voltages2 = vec![1.0001, 0.0, 0.5001]; // 0.01% change - within tolerance

        // Cache result
        let stamp = CachedStamp::new();
        bypass.cache_result(device_id, &voltages1, stamp);

        // Small voltage change within tolerance - should bypass
        assert!(bypass.should_bypass(device_id, &voltages2));
        assert_eq!(bypass.stats().bypassed, 1);
    }

    #[test]
    fn test_bypass_manager_disabled() {
        let mut bypass = ModelBypassManager::new();
        bypass.set_enabled(false);

        let device_id = DeviceId::new(0);
        let voltages = vec![1.0, 0.0, 0.5];

        // Cache result
        let stamp = CachedStamp::new();
        bypass.cache_result(device_id, &voltages, stamp);

        // Bypass disabled - should not bypass even with cache
        assert!(!bypass.should_bypass(device_id, &voltages));
        assert_eq!(bypass.stats().computed, 1);
    }

    #[test]
    fn test_bypass_manager_invalidate() {
        let mut bypass = ModelBypassManager::new();
        let device_id = DeviceId::new(0);
        let voltages = vec![1.0, 0.0, 0.5];

        // Cache result
        let stamp = CachedStamp::new();
        bypass.cache_result(device_id, &voltages, stamp);

        // Invalidate
        bypass.invalidate(device_id);

        // After invalidation - should not bypass
        assert!(!bypass.should_bypass(device_id, &voltages));
    }

    #[test]
    fn test_bypass_manager_invalidate_all() {
        let mut bypass = ModelBypassManager::new();

        // Cache multiple devices
        for i in 0..5 {
            let id = DeviceId::new(i);
            let stamp = CachedStamp::new();
            bypass.cache_result(id, &[1.0], stamp);
        }

        assert_eq!(bypass.cached_device_count(), 5);

        // Invalidate all
        bypass.invalidate_all();

        assert_eq!(bypass.cached_device_count(), 0);
    }

    #[test]
    fn test_bypass_stats() {
        let mut bypass = ModelBypassManager::new();
        let device_id = DeviceId::new(0);
        let voltages = vec![1.0, 0.0, 0.5];

        // First call - computed
        bypass.should_bypass(device_id, &voltages);

        // Cache result
        let stamp = CachedStamp::new();
        bypass.cache_result(device_id, &voltages, stamp);

        // Second call - bypassed
        bypass.should_bypass(device_id, &voltages);

        // Third call - bypassed
        bypass.should_bypass(device_id, &voltages);

        let stats = bypass.stats();
        assert_eq!(stats.total_evaluations, 3);
        assert_eq!(stats.computed, 1);
        assert_eq!(stats.bypassed, 2);
        assert!((stats.bypass_ratio() - 0.666666).abs() < 0.01);
    }

    #[test]
    fn test_bypass_ratio_calculation() {
        let mut stats = BypassStats::default();

        // Empty stats
        assert_eq!(stats.bypass_ratio(), 0.0);

        // 50% bypass
        stats.total_evaluations = 100;
        stats.bypassed = 50;
        stats.computed = 50;
        assert!((stats.bypass_ratio() - 0.5).abs() < 1e-6);

        // Reset
        stats.reset();
        assert_eq!(stats.total_evaluations, 0);
    }

    #[test]
    fn test_get_cached_stamp() {
        let mut bypass = ModelBypassManager::new();
        let device_id = DeviceId::new(0);
        let voltages = vec![1.0, 0.0, 0.5];

        // No cache yet
        assert!(bypass.get_cached(device_id).is_none());

        // Cache result
        let mut stamp = CachedStamp::new();
        stamp.add_conductance(0, 0, 1e-3);
        stamp.operating_point.id = Some(1e-3);
        bypass.cache_result(device_id, &voltages, stamp);

        // Get cached
        let cached = bypass.get_cached(device_id).unwrap();
        assert_eq!(cached.conductances.len(), 1);
        assert_eq!(cached.operating_point.id, Some(1e-3));
    }

    #[test]
    fn test_tolerance_configuration() {
        let mut bypass = ModelBypassManager::with_tolerances(1e-4, 1e-15);

        assert!((bypass.reltol() - 1e-4).abs() < 1e-20);
        assert!((bypass.abstol() - 1e-15).abs() < 1e-20);

        bypass.set_tolerances(1e-2, 1e-9);
        assert!((bypass.reltol() - 1e-2).abs() < 1e-20);
        assert!((bypass.abstol() - 1e-9).abs() < 1e-20);
    }
}
