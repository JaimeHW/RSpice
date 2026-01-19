//! Batch Device Storage and Evaluation
//!
//! Provides Struct-of-Arrays (SoA) storage for semiconductor devices to enable
//! efficient SIMD batch evaluation. This is the key optimization for circuits
//! with many devices of the same type.
//!
//! # Architecture
//!
//! Instead of storing devices as an array of structs:
//! ```text
//! [{is, n, vt, node_a, node_c}, {is, n, vt, node_a, node_c}, ...]
//! ```
//!
//! We use struct of arrays:
//! ```text
//! {is: [val, val, ...], n: [val, val, ...], vt: [val, val, ...], ...}
//! ```
//!
//! This allows SIMD operations to load 4 `is` values at once, compute 4
//! exponentials in parallel, etc.
//!
//! # Usage
//!
//! The batch storage is built from individual devices and used during
//! Newton-Raphson iteration for faster evaluation.

use crate::Value;
use crate::circuit::NodeId;
use crate::solver::{CscIndex, StaticMatrix};

#[cfg(feature = "simd")]
use crate::simd;

//=============================================================================
// Batch Diode Storage
//=============================================================================

/// Batch diode storage with Struct-of-Arrays layout for SIMD evaluation.
///
/// This structure mirrors the individual `Diode` struct but stores each field
/// as a contiguous array, enabling efficient SIMD processing.
#[derive(Debug, Default, Clone)]
pub struct BatchDiodes {
    //=========================================================================
    // Connectivity (constant during simulation)
    //=========================================================================
    /// Anode node indices
    pub node_anode: Vec<NodeId>,
    /// Cathode node indices
    pub node_cathode: Vec<NodeId>,

    //=========================================================================
    // Device Parameters (constant during simulation)
    //=========================================================================
    /// Saturation currents (Is)
    pub is: Vec<Value>,
    /// Pre-computed n * vt (emission coefficient * thermal voltage)
    pub n_vt: Vec<Value>,

    //=========================================================================
    // Pre-computed Matrix Indices (set once after matrix structure is known)
    //=========================================================================
    /// Matrix index for (anode, anode) stamp
    pub idx_aa: Vec<Option<CscIndex>>,
    /// Matrix index for (anode, cathode) stamp
    pub idx_ac: Vec<Option<CscIndex>>,
    /// Matrix index for (cathode, anode) stamp
    pub idx_ca: Vec<Option<CscIndex>>,
    /// Matrix index for (cathode, cathode) stamp
    pub idx_cc: Vec<Option<CscIndex>>,

    //=========================================================================
    // State Variables (updated each Newton iteration)
    //=========================================================================
    /// Current diode voltages (Vd = Va - Vc)
    pub vd: Vec<Value>,
    /// Previous iteration voltages (for convergence check)
    pub vd_prev: Vec<Value>,
    /// Computed diode currents
    pub id: Vec<Value>,
    /// Computed small-signal conductances
    pub gd: Vec<Value>,
    /// Equivalent current sources (ieq = id - gd * vd)
    pub ieq: Vec<Value>,

    //=========================================================================
    // RHS node indices (for efficient stamping)
    //=========================================================================
    /// Anode RHS indices (node_anode - 1, or None if ground)
    rhs_anode: Vec<Option<usize>>,
    /// Cathode RHS indices
    rhs_cathode: Vec<Option<usize>>,
}

impl BatchDiodes {
    /// Create a new empty batch storage.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create batch storage with pre-allocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            node_anode: Vec::with_capacity(capacity),
            node_cathode: Vec::with_capacity(capacity),
            is: Vec::with_capacity(capacity),
            n_vt: Vec::with_capacity(capacity),
            idx_aa: Vec::with_capacity(capacity),
            idx_ac: Vec::with_capacity(capacity),
            idx_ca: Vec::with_capacity(capacity),
            idx_cc: Vec::with_capacity(capacity),
            vd: Vec::with_capacity(capacity),
            vd_prev: Vec::with_capacity(capacity),
            id: Vec::with_capacity(capacity),
            gd: Vec::with_capacity(capacity),
            ieq: Vec::with_capacity(capacity),
            rhs_anode: Vec::with_capacity(capacity),
            rhs_cathode: Vec::with_capacity(capacity),
        }
    }

    /// Add a diode to the batch storage.
    ///
    /// # Arguments
    ///
    /// * `node_anode` - Anode node index
    /// * `node_cathode` - Cathode node index
    /// * `is` - Saturation current
    /// * `n` - Emission coefficient
    /// * `vt` - Thermal voltage
    pub fn add(
        &mut self,
        node_anode: NodeId,
        node_cathode: NodeId,
        is: Value,
        n: Value,
        vt: Value,
    ) {
        self.node_anode.push(node_anode);
        self.node_cathode.push(node_cathode);
        self.is.push(is);
        self.n_vt.push(n * vt);

        // Initialize matrix indices as None (set later via link)
        self.idx_aa.push(None);
        self.idx_ac.push(None);
        self.idx_ca.push(None);
        self.idx_cc.push(None);

        // Initialize state
        self.vd.push(0.0);
        self.vd_prev.push(0.0);
        self.id.push(0.0);
        self.gd.push(1e-12); // Small conductance to avoid singularity
        self.ieq.push(0.0);

        // RHS indices
        self.rhs_anode.push(if node_anode > 0 {
            Some(node_anode - 1)
        } else {
            None
        });
        self.rhs_cathode.push(if node_cathode > 0 {
            Some(node_cathode - 1)
        } else {
            None
        });
    }

    /// Get the number of diodes in the batch.
    #[inline]
    pub fn len(&self) -> usize {
        self.node_anode.len()
    }

    /// Check if the batch is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.node_anode.is_empty()
    }

    /// Link all diodes to the sparse matrix for O(1) stamping.
    pub fn link(&mut self, matrix: &StaticMatrix) {
        for i in 0..self.len() {
            let a = self.node_anode[i];
            let c = self.node_cathode[i];

            if a > 0 {
                self.idx_aa[i] = matrix.get_index(a - 1, a - 1);
            }
            if a > 0 && c > 0 {
                self.idx_ac[i] = matrix.get_index(a - 1, c - 1);
                self.idx_ca[i] = matrix.get_index(c - 1, a - 1);
            }
            if c > 0 {
                self.idx_cc[i] = matrix.get_index(c - 1, c - 1);
            }
        }
    }

    /// Gather diode voltages from the node voltage solution.
    ///
    /// Computes Vd = Va - Vc for each diode.
    #[inline]
    pub fn gather_voltages(&mut self, voltages: &[Value]) {
        for i in 0..self.len() {
            let va = if self.node_anode[i] == 0 {
                0.0
            } else {
                voltages.get(self.node_anode[i] - 1).copied().unwrap_or(0.0)
            };
            let vc = if self.node_cathode[i] == 0 {
                0.0
            } else {
                voltages
                    .get(self.node_cathode[i] - 1)
                    .copied()
                    .unwrap_or(0.0)
            };

            self.vd_prev[i] = self.vd[i];
            self.vd[i] = va - vc;
        }
    }

    /// Evaluate all diodes using SIMD-accelerated math.
    ///
    /// Computes current, conductance, and equivalent current source for each diode.
    #[cfg(feature = "simd")]
    pub fn evaluate(&mut self) {
        use wide::f64x4;

        let len = self.len();
        let simd_len = len - (len % 4);

        // Process 4 diodes at a time using SIMD
        let mut i = 0;
        while i < simd_len {
            let vd = f64x4::from(&self.vd[i..i + 4]);
            let is = f64x4::from(&self.is[i..i + 4]);
            let n_vt = f64x4::from(&self.n_vt[i..i + 4]);

            // SIMD diode I-V calculation
            let (id, gd) = simd::diode_iv(vd, is, n_vt);
            let ieq = simd::diode_ieq(id, gd, vd);

            // Store results
            simd::store_f64x4(id, &mut self.id[i..i + 4]);
            simd::store_f64x4(gd, &mut self.gd[i..i + 4]);
            simd::store_f64x4(ieq, &mut self.ieq[i..i + 4]);

            i += 4;
        }

        // Process remainder elements with scalar math
        for j in simd_len..len {
            let vd = self.vd[j];
            let is = self.is[j];
            let n_vt = self.n_vt[j];

            // Scalar diode calculation
            let vd_limited = vd.min(80.0 * n_vt);
            let exp_term = (vd_limited / n_vt).exp();
            self.id[j] = is * (exp_term - 1.0);
            self.gd[j] = (is * exp_term) / n_vt;
            self.ieq[j] = self.id[j] - self.gd[j] * vd;
        }
    }

    /// Evaluate all diodes using scalar math (non-SIMD fallback).
    #[cfg(not(feature = "simd"))]
    pub fn evaluate(&mut self) {
        for i in 0..self.len() {
            let vd = self.vd[i];
            let is = self.is[i];
            let n_vt = self.n_vt[i];

            let vd_limited = vd.min(80.0 * n_vt);
            let exp_term = (vd_limited / n_vt).exp();
            self.id[i] = is * (exp_term - 1.0);
            self.gd[i] = (is * exp_term) / n_vt;
            self.ieq[i] = self.id[i] - self.gd[i] * vd;
        }
    }

    /// Stamp all diodes to the matrix and RHS.
    ///
    /// This must be sequential due to the sparse matrix structure (each diode
    /// stamps to different matrix locations).
    #[inline]
    pub fn stamp(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        for i in 0..self.len() {
            let gd = self.gd[i];
            let ieq = self.ieq[i];

            // Stamp conductance matrix (using pre-linked indices)
            if let Some(idx) = self.idx_aa[i] {
                matrix.stamp_direct(idx, gd);
            }
            if let Some(idx) = self.idx_ac[i] {
                matrix.stamp_direct(idx, -gd);
            }
            if let Some(idx) = self.idx_ca[i] {
                matrix.stamp_direct(idx, -gd);
            }
            if let Some(idx) = self.idx_cc[i] {
                matrix.stamp_direct(idx, gd);
            }

            // Stamp RHS
            if let Some(idx) = self.rhs_anode[i] {
                rhs[idx] -= ieq;
            }
            if let Some(idx) = self.rhs_cathode[i] {
                rhs[idx] += ieq;
            }
        }
    }

    /// Check if all diodes have converged.
    #[inline]
    pub fn all_converged(&self, tolerance: Value) -> bool {
        for i in 0..self.len() {
            if (self.vd[i] - self.vd_prev[i]).abs() >= tolerance {
                return false;
            }
        }
        true
    }

    /// Update state from individual diode devices.
    ///
    /// This is used to synchronize state back to individual device structs
    /// if needed for other operations.
    pub fn sync_to_devices(&self, devices: &mut [super::semiconductor::Diode]) {
        // This would sync state if needed
        // Currently we keep batch as the source of truth during iteration
        let _ = devices; // Suppress unused warning
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_diodes_creation() {
        let mut batch = BatchDiodes::new();

        // Add 5 diodes
        for i in 0..5 {
            batch.add(i + 1, 0, 1e-14, 1.0, 0.026);
        }

        assert_eq!(batch.len(), 5);
        assert!(!batch.is_empty());
    }

    #[test]
    fn test_batch_voltage_gather() {
        let mut batch = BatchDiodes::new();
        batch.add(1, 0, 1e-14, 1.0, 0.026); // D1: node 1 to ground
        batch.add(2, 0, 1e-14, 1.0, 0.026); // D2: node 2 to ground
        batch.add(1, 2, 1e-14, 1.0, 0.026); // D3: node 1 to node 2

        let voltages = [0.7, 0.3]; // V(1) = 0.7, V(2) = 0.3

        batch.gather_voltages(&voltages);

        assert!((batch.vd[0] - 0.7).abs() < 1e-10); // V(1) - V(0) = 0.7
        assert!((batch.vd[1] - 0.3).abs() < 1e-10); // V(2) - V(0) = 0.3
        assert!((batch.vd[2] - 0.4).abs() < 1e-10); // V(1) - V(2) = 0.4
    }

    #[test]
    fn test_batch_evaluate() {
        let mut batch = BatchDiodes::new();

        // Add 8 diodes for SIMD testing (2 groups of 4)
        for _ in 0..8 {
            batch.add(1, 0, 1e-14, 1.0, 0.026);
        }

        // Set voltages
        batch.vd = vec![0.6, 0.7, 0.0, -1.0, 0.5, 0.65, 0.55, 0.75];

        batch.evaluate();

        // Forward bias should have positive current
        assert!(batch.id[0] > 0.0);
        assert!(batch.id[1] > 0.0);
        assert!(batch.id[1] > batch.id[0]); // Higher voltage = more current

        // Zero bias: current ~ 0
        assert!(batch.id[2].abs() < 1e-10);

        // Reverse bias: small negative current
        assert!(batch.id[3] < 0.0);

        // All conductances should be positive
        for gd in &batch.gd {
            assert!(*gd > 0.0);
        }
    }

    #[test]
    fn test_convergence_check() {
        let mut batch = BatchDiodes::new();
        batch.add(1, 0, 1e-14, 1.0, 0.026);
        batch.add(2, 0, 1e-14, 1.0, 0.026);

        batch.vd = vec![0.7, 0.6];
        batch.vd_prev = vec![0.7, 0.6];

        assert!(batch.all_converged(1e-12));

        batch.vd[0] = 0.71;
        assert!(!batch.all_converged(0.001));
    }
}
