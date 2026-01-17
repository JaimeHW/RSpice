//! Parallel Matrix Stamping
//!
//! Provides thread-safe matrix operations for parallel device stamping.
//! Uses atomic floating-point operations for lock-free concurrent writes.
//!
//! # Usage
//! ```ignore
//! // Create atomic matrix from static structure
//! let atomic = AtomicMatrix::from_static(&matrix);
//!
//! // Parallel stamping (with rayon)
//! devices.par_iter().for_each(|device| {
//!     device.stamp(&atomic);
//! });
//!
//! // Copy back to static matrix for solve
//! atomic.copy_to_static(&mut matrix);
//! ```
//!
//! # Performance Notes
//! - Atomic adds have ~10x overhead vs regular adds
//! - Only beneficial for circuits with 10k+ devices
//! - Small circuits should use sequential stamping

use crate::Value;

#[cfg(feature = "parallel")]
use portable_atomic::AtomicF64;
#[cfg(feature = "parallel")]
use std::sync::atomic::Ordering;

//=============================================================================
// Atomic Matrix for Parallel Stamping
//=============================================================================

/// Thread-safe matrix that allows concurrent stamping from multiple threads
/// 
/// Uses atomic f64 operations to allow lock-free parallel device stamping.
/// Each device can atomically add its contribution to matrix entries.
#[cfg(feature = "parallel")]
pub struct AtomicMatrix {
    /// Atomic values array (same layout as StaticMatrix.values)
    values: Vec<AtomicF64>,
    /// Number of values
    len: usize,
}

#[cfg(feature = "parallel")]
impl AtomicMatrix {
    /// Create a new atomic matrix with given capacity
    pub fn new(capacity: usize) -> Self {
        let values = (0..capacity).map(|_| AtomicF64::new(0.0)).collect();
        Self { values, len: capacity }
    }

    /// Create from a static matrix (copies structure, zeros values)
    pub fn from_static(matrix: &super::StaticMatrix) -> Self {
        let len = matrix.values_len();
        let values = (0..len).map(|_| AtomicF64::new(0.0)).collect();
        Self { values, len }
    }

    /// Clear all values to zero
    pub fn clear(&self) {
        for v in &self.values {
            v.store(0.0, Ordering::Relaxed);
        }
    }

    /// Atomically add a value at the given index
    /// 
    /// Uses fetch_add for lock-free concurrent writes.
    /// Multiple threads can safely call this on the same index.
    #[inline]
    pub fn add(&self, index: usize, value: Value) {
        if index < self.len {
            self.values[index].fetch_add(value, Ordering::Relaxed);
        }
    }

    /// Atomically add using a CscIndex
    #[inline]
    pub fn add_csc(&self, idx: super::CscIndex, value: Value) {
        self.add(idx.0, value);
    }

    /// Copy values back to a mutable f64 slice (for solving)
    pub fn copy_to(&self, dest: &mut [Value]) {
        for (i, atomic) in self.values.iter().enumerate() {
            if i < dest.len() {
                dest[i] = atomic.load(Ordering::Relaxed);
            }
        }
    }

    /// Copy values back to a StaticMatrix
    pub fn copy_to_static(&self, matrix: &mut super::StaticMatrix) {
        matrix.copy_values_from_atomic(self);
    }

    /// Get the current value at index (for debugging/testing)
    pub fn get(&self, index: usize) -> Value {
        if index < self.len {
            self.values[index].load(Ordering::Relaxed)
        } else {
            0.0
        }
    }

    /// Number of values
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

// Implement Sync for AtomicMatrix (already safe due to atomics)
#[cfg(feature = "parallel")]
unsafe impl Sync for AtomicMatrix {}
#[cfg(feature = "parallel")]
unsafe impl Send for AtomicMatrix {}

//=============================================================================
// Parallel Stamper Trait
//=============================================================================

/// Trait for devices that can stamp into an atomic matrix
#[cfg(feature = "parallel")]
pub trait ParallelStamp: Sync {
    /// Stamp device contributions into atomic matrix
    fn stamp_atomic(&self, matrix: &AtomicMatrix, voltages: &[Value], rhs: &AtomicRhs);
}

/// Thread-safe RHS vector for parallel stamping
#[cfg(feature = "parallel")]
pub struct AtomicRhs {
    values: Vec<AtomicF64>,
}

#[cfg(feature = "parallel")]
impl AtomicRhs {
    /// Create a new atomic RHS with given size
    pub fn new(size: usize) -> Self {
        let values = (0..size).map(|_| AtomicF64::new(0.0)).collect();
        Self { values }
    }

    /// Clear all values to zero
    pub fn clear(&self) {
        for v in &self.values {
            v.store(0.0, Ordering::Relaxed);
        }
    }

    /// Atomically add to RHS at given index
    #[inline]
    pub fn add(&self, index: usize, value: Value) {
        if index < self.values.len() {
            self.values[index].fetch_add(value, Ordering::Relaxed);
        }
    }

    /// Copy to regular f64 slice
    pub fn copy_to(&self, dest: &mut [Value]) {
        for (i, atomic) in self.values.iter().enumerate() {
            if i < dest.len() {
                dest[i] = atomic.load(Ordering::Relaxed);
            }
        }
    }

    /// Get current value
    pub fn get(&self, index: usize) -> Value {
        if index < self.values.len() {
            self.values[index].load(Ordering::Relaxed)
        } else {
            0.0
        }
    }

    /// Length
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Is empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(feature = "parallel")]
unsafe impl Sync for AtomicRhs {}
#[cfg(feature = "parallel")]
unsafe impl Send for AtomicRhs {}

//=============================================================================
// Parallel Stamping Utilities
//=============================================================================

/// Stamp multiple devices in parallel using rayon
/// 
/// # Arguments
/// * `devices` - Slice of devices implementing ParallelStamp
/// * `matrix` - Atomic matrix to stamp into
/// * `voltages` - Current voltage solution
/// * `rhs` - Atomic RHS vector
/// 
/// # Notes
/// Only beneficial for large circuits (10k+ devices).
/// For smaller circuits, sequential stamping is faster due to atomic overhead.
#[cfg(feature = "parallel")]
pub fn stamp_devices_parallel<D: ParallelStamp>(
    devices: &[D],
    matrix: &AtomicMatrix,
    voltages: &[Value],
    rhs: &AtomicRhs,
) {
    use rayon::prelude::*;
    
    devices.par_iter().for_each(|device| {
        device.stamp_atomic(matrix, voltages, rhs);
    });
}

/// Determine if parallel stamping is beneficial
/// 
/// Based on empirical testing:
/// - Atomic adds have ~10x overhead
/// - Rayon parallel iteration has startup cost ~10μs
/// - Break-even is around 1000 devices (conservative estimate)
#[inline]
pub fn should_use_parallel(num_devices: usize) -> bool {
    num_devices >= 1000
}

//=============================================================================
// Parallel Stamping for Diodes
//=============================================================================

/// Parallel-safe diode stamping data
/// 
/// Holds pre-computed indices for O(1) atomic stamping without locks.
#[cfg(feature = "parallel")]
#[derive(Debug, Clone)]
pub struct DiodeParallelStamper {
    /// Device name for debugging
    pub name: String,
    /// Anode node ID
    pub node_anode: usize,
    /// Cathode node ID  
    pub node_cathode: usize,
    /// Saturation current
    pub is: Value,
    /// Emission coefficient
    pub n: Value,
    /// Thermal voltage
    pub vt: Value,
    /// Pre-computed matrix indices
    pub idx_aa: Option<usize>,
    pub idx_ac: Option<usize>,
    pub idx_ca: Option<usize>,
    pub idx_cc: Option<usize>,
}

#[cfg(feature = "parallel")]
impl DiodeParallelStamper {
    /// Create from a Diode device with linked indices
    pub fn from_diode(d: &crate::device::diode::Diode) -> Self {
        Self {
            name: d.name.clone(),
            node_anode: d.node_anode,
            node_cathode: d.node_cathode,
            is: d.is,
            n: d.n,
            vt: d.vt,
            idx_aa: d.indices.aa.map(|i| i.0),
            idx_ac: d.indices.ac.map(|i| i.0),
            idx_ca: d.indices.ca.map(|i| i.0),
            idx_cc: d.indices.cc.map(|i| i.0),
        }
    }

    /// Compute diode current using Shockley equation
    #[inline]
    fn current(&self, vd: Value) -> Value {
        let vd_limited = vd.min(80.0 * self.n * self.vt);
        self.is * ((vd_limited / (self.n * self.vt)).exp() - 1.0)
    }

    /// Compute diode conductance
    #[inline]
    fn conductance(&self, vd: Value) -> Value {
        let vd_limited = vd.min(80.0 * self.n * self.vt);
        (self.is / (self.n * self.vt)) * (vd_limited / (self.n * self.vt)).exp()
    }
}

#[cfg(feature = "parallel")]
impl ParallelStamp for DiodeParallelStamper {
    fn stamp_atomic(&self, matrix: &AtomicMatrix, voltages: &[Value], rhs: &AtomicRhs) {
        let va = if self.node_anode == 0 { 0.0 } else { voltages[self.node_anode - 1] };
        let vc = if self.node_cathode == 0 { 0.0 } else { voltages[self.node_cathode - 1] };
        let vd = va - vc;
        
        let id = self.current(vd);
        let gd = self.conductance(vd);
        let ieq = id - gd * vd;
        
        // Atomic matrix stamping
        if let Some(idx) = self.idx_aa { matrix.add(idx, gd); }
        if let Some(idx) = self.idx_ac { matrix.add(idx, -gd); }
        if let Some(idx) = self.idx_ca { matrix.add(idx, -gd); }
        if let Some(idx) = self.idx_cc { matrix.add(idx, gd); }
        
        // Atomic RHS stamping
        if self.node_anode > 0 { rhs.add(self.node_anode - 1, -ieq); }
        if self.node_cathode > 0 { rhs.add(self.node_cathode - 1, ieq); }
    }
}

//=============================================================================
// Circuit-Level Parallel Stamping
//=============================================================================

/// Options for controlling parallel stamping behavior
#[derive(Debug, Clone, Copy)]
pub struct ParallelStampConfig {
    /// Minimum number of total nonlinear devices to enable parallel stamping
    pub min_devices: usize,
    /// Whether to force parallel stamping regardless of device count
    pub force_parallel: bool,
}

impl Default for ParallelStampConfig {
    fn default() -> Self {
        Self {
            min_devices: 1000,
            force_parallel: false,
        }
    }
}

impl ParallelStampConfig {
    /// Check if parallel stamping should be used
    #[inline]
    pub fn should_use(&self, num_devices: usize) -> bool {
        self.force_parallel || num_devices >= self.min_devices
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(all(test, feature = "parallel"))]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_matrix_basic() {
        let matrix = AtomicMatrix::new(10);
        
        matrix.add(0, 1.0);
        matrix.add(0, 2.0);
        matrix.add(5, 3.0);
        
        assert!((matrix.get(0) - 3.0).abs() < 1e-10);
        assert!((matrix.get(5) - 3.0).abs() < 1e-10);
        assert!((matrix.get(1) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_atomic_matrix_clear() {
        let matrix = AtomicMatrix::new(5);
        
        matrix.add(0, 10.0);
        matrix.add(1, 20.0);
        matrix.clear();
        
        assert!((matrix.get(0) - 0.0).abs() < 1e-10);
        assert!((matrix.get(1) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_atomic_rhs_basic() {
        let rhs = AtomicRhs::new(5);
        
        rhs.add(0, 1.0);
        rhs.add(0, 2.0);
        
        assert!((rhs.get(0) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_parallel_stamping() {
        use rayon::prelude::*;
        
        let matrix = AtomicMatrix::new(10);
        
        // Simulate parallel stamping
        let indices: Vec<usize> = (0..100).collect();
        indices.par_iter().for_each(|_| {
            matrix.add(0, 0.1);
        });
        
        // Should sum to 10.0 (100 * 0.1)
        assert!((matrix.get(0) - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_copy_to_slice() {
        let matrix = AtomicMatrix::new(3);
        matrix.add(0, 1.0);
        matrix.add(1, 2.0);
        matrix.add(2, 3.0);
        
        let mut dest = vec![0.0; 3];
        matrix.copy_to(&mut dest);
        
        assert!((dest[0] - 1.0).abs() < 1e-10);
        assert!((dest[1] - 2.0).abs() < 1e-10);
        assert!((dest[2] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_should_use_parallel() {
        assert!(!should_use_parallel(100));
        assert!(!should_use_parallel(500));
        assert!(should_use_parallel(1000));
        assert!(should_use_parallel(10000));
    }

    #[test]
    fn test_parallel_stamp_config_default() {
        let config = ParallelStampConfig::default();
        assert_eq!(config.min_devices, 1000);
        assert!(!config.force_parallel);
        
        assert!(!config.should_use(500));
        assert!(config.should_use(1000));
        assert!(config.should_use(5000));
    }

    #[test]
    fn test_parallel_stamp_config_force() {
        let config = ParallelStampConfig {
            min_devices: 1000,
            force_parallel: true,
        };
        
        // Should always use parallel when forced
        assert!(config.should_use(1));
        assert!(config.should_use(100));
        assert!(config.should_use(10000));
    }

    #[test]
    fn test_diode_parallel_stamper_current() {
        // Create a simple diode stamper
        let stamper = DiodeParallelStamper {
            name: "D1".to_string(),
            node_anode: 1,
            node_cathode: 0,
            is: 1e-14,
            n: 1.0,
            vt: 0.026,
            idx_aa: Some(0),
            idx_ac: None,
            idx_ca: None,
            idx_cc: None,
        };
        
        // Forward bias should give positive current
        let id_forward = stamper.current(0.7);
        assert!(id_forward > 0.0);
        
        // Reverse bias should give ~-Is
        let id_reverse = stamper.current(-1.0);
        assert!(id_reverse.abs() < 1e-8);
    }

    #[test]
    fn test_diode_parallel_stamper_atomic() {
        let matrix = AtomicMatrix::new(5);
        let rhs = AtomicRhs::new(3);
        
        let stamper = DiodeParallelStamper {
            name: "D1".to_string(),
            node_anode: 1,
            node_cathode: 2,
            is: 1e-14,
            n: 1.0,
            vt: 0.026,
            idx_aa: Some(0),
            idx_ac: Some(1),
            idx_ca: Some(2),
            idx_cc: Some(3),
        };
        
        // Stamp with voltages [0V, 0.7V, 0V]
        let voltages = vec![0.7, 0.0, 0.0];
        stamper.stamp_atomic(&matrix, &voltages, &rhs);
        
        // Conductance should be stamped
        let gd = matrix.get(0);
        assert!(gd > 0.0, "Conductance should be positive, got {}", gd);
        
        // RHS should have current contributions
        assert!(rhs.get(0).abs() > 0.0 || rhs.get(1).abs() > 0.0);
    }
}
