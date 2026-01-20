//! Batch MOSFET Storage and Evaluation
//!
//! SoA storage for MOSFETs to enable SIMD batch evaluation.
//! Handles Level 1 (Shichman-Hodges) models for maximum vectorization.

use crate::Value;
use crate::circuit::NodeId;
use crate::device::mosfet::{MosRegion, MosType};
use crate::solver::{CscIndex, StaticMatrix};

#[cfg(feature = "simd")]
use wide::f64x4;

//=============================================================================
// Batch MOSFET Storage
//=============================================================================

/// Batch MOSFET storage with Struct-of-Arrays layout for SIMD evaluation.
///
/// Designed for Level 1 MOSFETs which have simpler equations that
/// vectorize well. More complex models (BSIM3/4) have too many branches
/// to benefit significantly from SIMD.
#[derive(Debug, Default, Clone)]
pub struct BatchMosfets {
    //=========================================================================
    // Connectivity
    //=========================================================================
    pub node_drain: Vec<NodeId>,
    pub node_gate: Vec<NodeId>,
    pub node_source: Vec<NodeId>,
    pub node_bulk: Vec<NodeId>,

    //=========================================================================
    // Device Parameters (constant during simulation)
    //=========================================================================
    /// Polarity: +1.0 for NMOS, -1.0 for PMOS
    pub polarity: Vec<Value>,
    /// Transconductance: beta = KP * W/L
    pub beta: Vec<Value>,
    /// Threshold voltage (VTO)
    pub vto: Vec<Value>,
    /// Body effect coefficient (GAMMA)
    pub gamma: Vec<Value>,
    /// Surface potential (PHI)
    pub phi: Vec<Value>,
    /// Channel length modulation (LAMBDA)
    pub lambda: Vec<Value>,

    //=========================================================================
    // Matrix Indices (set once after linking)
    //=========================================================================
    // Drain row
    pub idx_dd: Vec<Option<CscIndex>>,
    pub idx_dg: Vec<Option<CscIndex>>,
    pub idx_ds: Vec<Option<CscIndex>>,
    pub idx_db: Vec<Option<CscIndex>>,
    // Source row
    pub idx_sd: Vec<Option<CscIndex>>,
    pub idx_sg: Vec<Option<CscIndex>>,
    pub idx_ss: Vec<Option<CscIndex>>,
    pub idx_sb: Vec<Option<CscIndex>>,

    //=========================================================================
    // State Variables (updated each iteration)
    //=========================================================================
    pub vgs: Vec<Value>,
    pub vds: Vec<Value>,
    pub vbs: Vec<Value>,
    pub vgs_prev: Vec<Value>,
    pub vds_prev: Vec<Value>,

    /// Drain current
    pub id: Vec<Value>,
    /// Transconductance gm = dId/dVgs
    pub gm: Vec<Value>,
    /// Output conductance gds = dId/dVds
    pub gds: Vec<Value>,
    /// Body transconductance gmb = dId/dVbs
    pub gmb: Vec<Value>,
    /// Equivalent current source
    pub id_eq: Vec<Value>,

    /// Operating region (for diagnostics)
    pub region: Vec<MosRegion>,

    //=========================================================================
    // RHS indices
    //=========================================================================
    rhs_drain: Vec<Option<usize>>,
    rhs_source: Vec<Option<usize>>,
}

impl BatchMosfets {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            node_drain: Vec::with_capacity(capacity),
            node_gate: Vec::with_capacity(capacity),
            node_source: Vec::with_capacity(capacity),
            node_bulk: Vec::with_capacity(capacity),
            polarity: Vec::with_capacity(capacity),
            beta: Vec::with_capacity(capacity),
            vto: Vec::with_capacity(capacity),
            gamma: Vec::with_capacity(capacity),
            phi: Vec::with_capacity(capacity),
            lambda: Vec::with_capacity(capacity),
            idx_dd: Vec::with_capacity(capacity),
            idx_dg: Vec::with_capacity(capacity),
            idx_ds: Vec::with_capacity(capacity),
            idx_db: Vec::with_capacity(capacity),
            idx_sd: Vec::with_capacity(capacity),
            idx_sg: Vec::with_capacity(capacity),
            idx_ss: Vec::with_capacity(capacity),
            idx_sb: Vec::with_capacity(capacity),
            vgs: Vec::with_capacity(capacity),
            vds: Vec::with_capacity(capacity),
            vbs: Vec::with_capacity(capacity),
            vgs_prev: Vec::with_capacity(capacity),
            vds_prev: Vec::with_capacity(capacity),
            id: Vec::with_capacity(capacity),
            gm: Vec::with_capacity(capacity),
            gds: Vec::with_capacity(capacity),
            gmb: Vec::with_capacity(capacity),
            id_eq: Vec::with_capacity(capacity),
            region: Vec::with_capacity(capacity),
            rhs_drain: Vec::with_capacity(capacity),
            rhs_source: Vec::with_capacity(capacity),
        }
    }

    /// Add a MOSFET to the batch storage.
    pub fn add(
        &mut self,
        node_drain: NodeId,
        node_gate: NodeId,
        node_source: NodeId,
        node_bulk: NodeId,
        mos_type: MosType,
        beta: Value,
        vto: Value,
        gamma: Value,
        phi: Value,
        lambda: Value,
    ) {
        self.node_drain.push(node_drain);
        self.node_gate.push(node_gate);
        self.node_source.push(node_source);
        self.node_bulk.push(node_bulk);

        self.polarity.push(match mos_type {
            MosType::Nmos => 1.0,
            MosType::Pmos => -1.0,
        });
        self.beta.push(beta);
        self.vto.push(vto);
        self.gamma.push(gamma);
        self.phi.push(phi);
        self.lambda.push(lambda);

        // Initialize indices as None
        self.idx_dd.push(None);
        self.idx_dg.push(None);
        self.idx_ds.push(None);
        self.idx_db.push(None);
        self.idx_sd.push(None);
        self.idx_sg.push(None);
        self.idx_ss.push(None);
        self.idx_sb.push(None);

        // Initialize state
        self.vgs.push(0.0);
        self.vds.push(0.0);
        self.vbs.push(0.0);
        self.vgs_prev.push(0.0);
        self.vds_prev.push(0.0);
        self.id.push(0.0);
        self.gm.push(1e-12);
        self.gds.push(1e-12);
        self.gmb.push(0.0);
        self.id_eq.push(0.0);
        self.region.push(MosRegion::Cutoff);

        self.rhs_drain.push(if node_drain > 0 {
            Some(node_drain - 1)
        } else {
            None
        });
        self.rhs_source.push(if node_source > 0 {
            Some(node_source - 1)
        } else {
            None
        });
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.node_drain.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.node_drain.is_empty()
    }

    /// Link all MOSFETs to the sparse matrix.
    pub fn link(&mut self, matrix: &StaticMatrix) {
        for i in 0..self.len() {
            let d = self.node_drain[i];
            let g = self.node_gate[i];
            let s = self.node_source[i];
            let b = self.node_bulk[i];

            // Drain row
            if d > 0 {
                self.idx_dd[i] = matrix.get_index(d - 1, d - 1);
            }
            if d > 0 && g > 0 {
                self.idx_dg[i] = matrix.get_index(d - 1, g - 1);
            }
            if d > 0 && s > 0 {
                self.idx_ds[i] = matrix.get_index(d - 1, s - 1);
            }
            if d > 0 && b > 0 {
                self.idx_db[i] = matrix.get_index(d - 1, b - 1);
            }
            // Source row
            if s > 0 && d > 0 {
                self.idx_sd[i] = matrix.get_index(s - 1, d - 1);
            }
            if s > 0 && g > 0 {
                self.idx_sg[i] = matrix.get_index(s - 1, g - 1);
            }
            if s > 0 {
                self.idx_ss[i] = matrix.get_index(s - 1, s - 1);
            }
            if s > 0 && b > 0 {
                self.idx_sb[i] = matrix.get_index(s - 1, b - 1);
            }
        }
    }

    /// Gather voltages from node solution.
    #[inline]
    pub fn gather_voltages(&mut self, voltages: &[Value]) {
        for i in 0..self.len() {
            let vd = if self.node_drain[i] == 0 {
                0.0
            } else {
                voltages.get(self.node_drain[i] - 1).copied().unwrap_or(0.0)
            };
            let vg = if self.node_gate[i] == 0 {
                0.0
            } else {
                voltages.get(self.node_gate[i] - 1).copied().unwrap_or(0.0)
            };
            let vs = if self.node_source[i] == 0 {
                0.0
            } else {
                voltages
                    .get(self.node_source[i] - 1)
                    .copied()
                    .unwrap_or(0.0)
            };
            let vb = if self.node_bulk[i] == 0 {
                0.0
            } else {
                voltages.get(self.node_bulk[i] - 1).copied().unwrap_or(0.0)
            };

            self.vgs_prev[i] = self.vgs[i];
            self.vds_prev[i] = self.vds[i];
            self.vgs[i] = vg - vs;
            self.vds[i] = vd - vs;
            self.vbs[i] = vb - vs;
        }
    }

    /// Evaluate all MOSFETs using SIMD where possible.
    #[cfg(feature = "simd")]
    pub fn evaluate(&mut self) {
        let len = self.len();
        let simd_len = len - (len % 4);

        // Process 4 MOSFETs at a time
        let mut i = 0;
        while i < simd_len {
            self.evaluate_simd_chunk(i);
            i += 4;
        }

        // Process remainder
        for j in simd_len..len {
            self.evaluate_scalar(j);
        }
    }

    /// Evaluate 4 MOSFETs using SIMD.
    #[cfg(feature = "simd")]
    fn evaluate_simd_chunk(&mut self, i: usize) {
        // Load voltages
        let vgs = f64x4::from(&self.vgs[i..i + 4]);
        let vds = f64x4::from(&self.vds[i..i + 4]);
        let vbs = f64x4::from(&self.vbs[i..i + 4]);
        let polarity = f64x4::from(&self.polarity[i..i + 4]);
        let beta = f64x4::from(&self.beta[i..i + 4]);
        let vto = f64x4::from(&self.vto[i..i + 4]);
        let gamma = f64x4::from(&self.gamma[i..i + 4]);
        let phi = f64x4::from(&self.phi[i..i + 4]);
        let lambda = f64x4::from(&self.lambda[i..i + 4]);

        // Effective voltages (polarity adjusted)
        let vgs_eff = polarity * vgs;
        let vds_eff = (polarity * vds).max(f64x4::splat(1e-12));
        let vbs_eff = polarity * vbs;

        // Threshold voltage with body effect
        // Vth = Vto + gamma * (sqrt(phi - vbs) - sqrt(phi))
        let phi_vbs = (phi - vbs_eff).max(f64x4::splat(0.0));
        let phi_vbs_sqrt = simd_sqrt(phi_vbs);
        let phi_sqrt = simd_sqrt(phi);
        let vth = vto + gamma * (phi_vbs_sqrt - phi_sqrt);

        // Gate overdrive
        let vgt = (vgs_eff - vth).max(f64x4::splat(0.0));

        // Saturation voltage (minimum of Vgt and Vds)
        let vdsat = vgt.min(vds_eff);

        // Level 1 current: Id = beta * (Vgt * Vdsat - Vdsat²/2) * (1 + lambda * Vds)
        let id_core = beta * (vgt * vdsat - f64x4::splat(0.5) * vdsat * vdsat);
        let id = polarity * id_core * (f64x4::splat(1.0) + lambda * vds_eff);

        // Conductances (simplified Level 1 derivatives)
        // gm = dId/dVgs ≈ beta * Vdsat * (1 + lambda * Vds) in saturation
        // gds = dId/dVds ≈ beta * (Vgt - Vds) + Id * lambda in linear
        let in_saturation = simd_step(vds_eff - vgt);
        let gm = beta * vdsat * (f64x4::splat(1.0) + lambda * vds_eff);
        let gds_lin = beta * (vgt - vdsat) * (f64x4::splat(1.0) + lambda * vds_eff);
        let gds_sat = id_core.abs() * lambda;
        let gds = simd_blend(in_saturation, gds_sat, gds_lin).max(f64x4::splat(1e-12));

        // Body transconductance (simplified)
        let gmb = f64x4::splat(0.0); // Simplified for Level 1

        // Equivalent current source
        let id_eq = id - gm * vgs - gds * vds - gmb * vbs;

        // Store results
        store_f64x4(id, &mut self.id[i..]);
        store_f64x4(gm, &mut self.gm[i..]);
        store_f64x4(gds, &mut self.gds[i..]);
        store_f64x4(gmb, &mut self.gmb[i..]);
        store_f64x4(id_eq, &mut self.id_eq[i..]);

        // Update regions (scalar, for diagnostics only)
        for j in 0..4 {
            let idx = i + j;
            let vgt_val = self.vgs[idx] * self.polarity[idx] - self.vto[idx];
            let vds_val = self.vds[idx] * self.polarity[idx];
            self.region[idx] = if vgt_val <= 0.0 {
                MosRegion::Cutoff
            } else if vds_val < vgt_val {
                MosRegion::Linear
            } else {
                MosRegion::Saturation
            };
        }
    }

    /// Scalar evaluation for remainder elements.
    fn evaluate_scalar(&mut self, i: usize) {
        let p = self.polarity[i];
        let vgs_eff = p * self.vgs[i];
        let vds_eff = (p * self.vds[i]).max(1e-12);
        let vbs_eff = p * self.vbs[i];

        // Threshold voltage with body effect
        let phi_vbs = (self.phi[i] - vbs_eff).max(0.0);
        let vth = self.vto[i] + self.gamma[i] * (phi_vbs.sqrt() - self.phi[i].sqrt());

        let vgt = (vgs_eff - vth).max(0.0);
        let vdsat = vgt.min(vds_eff);

        // Current
        let id_core = self.beta[i] * (vgt * vdsat - 0.5 * vdsat * vdsat);
        self.id[i] = p * id_core * (1.0 + self.lambda[i] * vds_eff);

        // Conductances
        self.gm[i] = self.beta[i] * vdsat * (1.0 + self.lambda[i] * vds_eff);
        let gds_lin = self.beta[i] * (vgt - vdsat) * (1.0 + self.lambda[i] * vds_eff);
        let gds_sat = id_core.abs() * self.lambda[i];
        self.gds[i] = if vds_eff >= vgt { gds_sat } else { gds_lin }.max(1e-12);
        self.gmb[i] = 0.0;

        self.id_eq[i] = self.id[i] - self.gm[i] * self.vgs[i] - self.gds[i] * self.vds[i];

        // Region
        self.region[i] = if vgt <= 0.0 {
            MosRegion::Cutoff
        } else if vds_eff < vgt {
            MosRegion::Linear
        } else {
            MosRegion::Saturation
        };
    }

    /// Non-SIMD fallback.
    #[cfg(not(feature = "simd"))]
    pub fn evaluate(&mut self) {
        for i in 0..self.len() {
            self.evaluate_scalar(i);
        }
    }

    /// Stamp all MOSFETs to matrix and RHS.
    #[inline]
    pub fn stamp(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        for i in 0..self.len() {
            let gm = self.gm[i];
            let gds = self.gds[i];
            let gmb = self.gmb[i];
            let id_eq = self.id_eq[i];

            // Drain row
            if let Some(idx) = self.idx_dd[i] {
                matrix.stamp_direct(idx, gds);
            }
            if let Some(idx) = self.idx_dg[i] {
                matrix.stamp_direct(idx, gm);
            }
            if let Some(idx) = self.idx_ds[i] {
                matrix.stamp_direct(idx, -gm - gds - gmb);
            }
            if let Some(idx) = self.idx_db[i] {
                matrix.stamp_direct(idx, gmb);
            }
            // Source row
            if let Some(idx) = self.idx_sd[i] {
                matrix.stamp_direct(idx, -gds);
            }
            if let Some(idx) = self.idx_sg[i] {
                matrix.stamp_direct(idx, -gm);
            }
            if let Some(idx) = self.idx_ss[i] {
                matrix.stamp_direct(idx, gm + gds + gmb);
            }
            if let Some(idx) = self.idx_sb[i] {
                matrix.stamp_direct(idx, -gmb);
            }

            // RHS
            if let Some(idx) = self.rhs_drain[i] {
                rhs[idx] -= id_eq;
            }
            if let Some(idx) = self.rhs_source[i] {
                rhs[idx] += id_eq;
            }
        }
    }

    /// Check convergence.
    #[inline]
    pub fn all_converged(&self, tolerance: Value) -> bool {
        for i in 0..self.len() {
            if (self.vgs[i] - self.vgs_prev[i]).abs() >= tolerance {
                return false;
            }
            if (self.vds[i] - self.vds_prev[i]).abs() >= tolerance {
                return false;
            }
        }
        true
    }
}

//=============================================================================
// SIMD Helper Functions - Import from centralized simd module
//=============================================================================

#[cfg(feature = "simd")]
use crate::simd::{
    blend_f64x4 as simd_blend, sqrt_f64x4 as simd_sqrt, step_f64x4 as simd_step, store_f64x4,
};

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_mosfets_creation() {
        let mut batch = BatchMosfets::new();

        batch.add(1, 2, 0, 0, MosType::Nmos, 1.1e-3, 0.7, 0.4, 0.65, 0.01);
        batch.add(3, 4, 0, 0, MosType::Pmos, 0.55e-3, -0.7, 0.4, 0.65, 0.01);

        assert_eq!(batch.len(), 2);
        assert!(!batch.is_empty());
        assert_eq!(batch.polarity[0], 1.0);
        assert_eq!(batch.polarity[1], -1.0);
    }

    #[test]
    fn test_batch_voltage_gather() {
        let mut batch = BatchMosfets::new();
        batch.add(1, 2, 0, 0, MosType::Nmos, 1e-3, 0.7, 0.4, 0.65, 0.01);

        let voltages = [1.0, 2.0]; // V(1) = 1.0, V(2) = 2.0

        batch.gather_voltages(&voltages);

        assert!((batch.vds[0] - 1.0).abs() < 1e-10); // V(drain) - V(source) = 1.0
        assert!((batch.vgs[0] - 2.0).abs() < 1e-10); // V(gate) - V(source) = 2.0
    }

    #[test]
    fn test_batch_evaluate() {
        let mut batch = BatchMosfets::new();

        // Add 8 NMOS for SIMD
        for _ in 0..8 {
            batch.add(1, 2, 0, 0, MosType::Nmos, 1e-3, 0.7, 0.4, 0.65, 0.01);
        }

        // Set to saturation: Vgs = 1.5V, Vds = 2V (Vgs - Vth = 0.8V < 2V)
        for i in 0..8 {
            batch.vgs[i] = 1.5;
            batch.vds[i] = 2.0;
            batch.vbs[i] = 0.0;
        }

        batch.evaluate();

        // All should be in saturation with positive current
        for i in 0..8 {
            assert!(batch.id[i] > 0.0, "Expected positive drain current");
            assert_eq!(batch.region[i], MosRegion::Saturation);
            assert!(batch.gm[i] > 0.0);
            assert!(batch.gds[i] > 0.0);
        }
    }
}
