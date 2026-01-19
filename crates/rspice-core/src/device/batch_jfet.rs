//! Batch JFET Storage and Evaluation
//!
//! SoA storage for JFETs (Shichman-Hodges model) to enable SIMD batch evaluation.

use crate::Value;
use crate::circuit::NodeId;
use crate::device::mosfet::JfetType;
use crate::solver::{CscIndex, StaticMatrix};

#[cfg(feature = "simd")]
use wide::f64x4;

//=============================================================================
// Batch JFET Storage
//=============================================================================

/// Operating region for JFETs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JfetRegion {
    #[default]
    Cutoff,
    Linear,
    Saturation,
}

/// Batch JFET storage with Struct-of-Arrays layout for SIMD evaluation.
#[derive(Debug, Default, Clone)]
pub struct BatchJfets {
    //=========================================================================
    // Connectivity
    //=========================================================================
    pub node_drain: Vec<NodeId>,
    pub node_gate: Vec<NodeId>,
    pub node_source: Vec<NodeId>,

    //=========================================================================
    // Device Parameters
    //=========================================================================
    /// Polarity: +1.0 for NJF, -1.0 for PJF
    pub polarity: Vec<Value>,
    /// Threshold voltage (VTO), negative for NJF
    pub vto: Vec<Value>,
    /// Transconductance coefficient (BETA) = IDSS/VTO²
    pub beta: Vec<Value>,
    /// Channel length modulation (LAMBDA)
    pub lambda: Vec<Value>,
    /// Gate saturation current (IS)
    pub is: Vec<Value>,
    /// Pre-computed n*Vt for gate junctions
    pub n_vt: Vec<Value>,
    /// Device multiplier (M * AREA)
    pub mult: Vec<Value>,

    //=========================================================================
    // Matrix Indices
    //=========================================================================
    // Drain row
    pub idx_dd: Vec<Option<CscIndex>>,
    pub idx_dg: Vec<Option<CscIndex>>,
    pub idx_ds: Vec<Option<CscIndex>>,
    // Gate row
    pub idx_gd: Vec<Option<CscIndex>>,
    pub idx_gg: Vec<Option<CscIndex>>,
    pub idx_gs: Vec<Option<CscIndex>>,
    // Source row
    pub idx_sd: Vec<Option<CscIndex>>,
    pub idx_sg: Vec<Option<CscIndex>>,
    pub idx_ss: Vec<Option<CscIndex>>,

    //=========================================================================
    // State Variables
    //=========================================================================
    pub vgs: Vec<Value>,
    pub vds: Vec<Value>,
    pub vgs_prev: Vec<Value>,
    pub vds_prev: Vec<Value>,

    /// Drain current
    pub ids: Vec<Value>,
    /// Transconductance gm = dIds/dVgs
    pub gm: Vec<Value>,
    /// Output conductance gds = dIds/dVds
    pub gds: Vec<Value>,
    /// Gate-source conductance (from gate junction)
    pub ggs: Vec<Value>,
    /// Gate-drain conductance (from gate junction)
    pub ggd: Vec<Value>,
    /// Equivalent current source
    pub ids_eq: Vec<Value>,
    /// Operating region
    pub region: Vec<JfetRegion>,

    //=========================================================================
    // RHS indices
    //=========================================================================
    rhs_drain: Vec<Option<usize>>,
    rhs_gate: Vec<Option<usize>>,
    rhs_source: Vec<Option<usize>>,
}

impl BatchJfets {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            node_drain: Vec::with_capacity(capacity),
            node_gate: Vec::with_capacity(capacity),
            node_source: Vec::with_capacity(capacity),
            polarity: Vec::with_capacity(capacity),
            vto: Vec::with_capacity(capacity),
            beta: Vec::with_capacity(capacity),
            lambda: Vec::with_capacity(capacity),
            is: Vec::with_capacity(capacity),
            n_vt: Vec::with_capacity(capacity),
            mult: Vec::with_capacity(capacity),
            idx_dd: Vec::with_capacity(capacity),
            idx_dg: Vec::with_capacity(capacity),
            idx_ds: Vec::with_capacity(capacity),
            idx_gd: Vec::with_capacity(capacity),
            idx_gg: Vec::with_capacity(capacity),
            idx_gs: Vec::with_capacity(capacity),
            idx_sd: Vec::with_capacity(capacity),
            idx_sg: Vec::with_capacity(capacity),
            idx_ss: Vec::with_capacity(capacity),
            vgs: Vec::with_capacity(capacity),
            vds: Vec::with_capacity(capacity),
            vgs_prev: Vec::with_capacity(capacity),
            vds_prev: Vec::with_capacity(capacity),
            ids: Vec::with_capacity(capacity),
            gm: Vec::with_capacity(capacity),
            gds: Vec::with_capacity(capacity),
            ggs: Vec::with_capacity(capacity),
            ggd: Vec::with_capacity(capacity),
            ids_eq: Vec::with_capacity(capacity),
            region: Vec::with_capacity(capacity),
            rhs_drain: Vec::with_capacity(capacity),
            rhs_gate: Vec::with_capacity(capacity),
            rhs_source: Vec::with_capacity(capacity),
        }
    }

    /// Add a JFET to the batch storage.
    #[allow(clippy::too_many_arguments)]
    pub fn add(
        &mut self,
        node_drain: NodeId,
        node_gate: NodeId,
        node_source: NodeId,
        jfet_type: JfetType,
        vto: Value,
        beta: Value,
        lambda: Value,
        is: Value,
        mult: Value,
    ) {
        self.node_drain.push(node_drain);
        self.node_gate.push(node_gate);
        self.node_source.push(node_source);

        self.polarity.push(jfet_type.polarity());
        self.vto.push(vto);
        self.beta.push(beta);
        self.lambda.push(lambda);
        self.is.push(is);
        self.n_vt.push(0.026); // Thermal voltage at 300K
        self.mult.push(mult);

        // Initialize indices
        self.idx_dd.push(None);
        self.idx_dg.push(None);
        self.idx_ds.push(None);
        self.idx_gd.push(None);
        self.idx_gg.push(None);
        self.idx_gs.push(None);
        self.idx_sd.push(None);
        self.idx_sg.push(None);
        self.idx_ss.push(None);

        // Initialize state
        self.vgs.push(0.0);
        self.vds.push(0.0);
        self.vgs_prev.push(0.0);
        self.vds_prev.push(0.0);
        self.ids.push(0.0);
        self.gm.push(1e-12);
        self.gds.push(1e-12);
        self.ggs.push(1e-12);
        self.ggd.push(1e-12);
        self.ids_eq.push(0.0);
        self.region.push(JfetRegion::Cutoff);

        self.rhs_drain.push(if node_drain > 0 {
            Some(node_drain - 1)
        } else {
            None
        });
        self.rhs_gate.push(if node_gate > 0 {
            Some(node_gate - 1)
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

    /// Link all JFETs to the sparse matrix.
    pub fn link(&mut self, matrix: &StaticMatrix) {
        for i in 0..self.len() {
            let d = self.node_drain[i];
            let g = self.node_gate[i];
            let s = self.node_source[i];

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
            // Gate row
            if g > 0 && d > 0 {
                self.idx_gd[i] = matrix.get_index(g - 1, d - 1);
            }
            if g > 0 {
                self.idx_gg[i] = matrix.get_index(g - 1, g - 1);
            }
            if g > 0 && s > 0 {
                self.idx_gs[i] = matrix.get_index(g - 1, s - 1);
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

            self.vgs_prev[i] = self.vgs[i];
            self.vds_prev[i] = self.vds[i];
            self.vgs[i] = vg - vs;
            self.vds[i] = vd - vs;
        }
    }

    /// Evaluate using SIMD.
    #[cfg(feature = "simd")]
    pub fn evaluate(&mut self) {
        let len = self.len();
        let simd_len = len - (len % 4);

        let mut i = 0;
        while i < simd_len {
            self.evaluate_simd_chunk(i);
            i += 4;
        }

        for j in simd_len..len {
            self.evaluate_scalar(j);
        }
    }

    /// SIMD evaluation for 4 JFETs.
    #[cfg(feature = "simd")]
    fn evaluate_simd_chunk(&mut self, i: usize) {
        let vgs = f64x4::from(&self.vgs[i..i + 4]);
        let vds = f64x4::from(&self.vds[i..i + 4]);
        let polarity = f64x4::from(&self.polarity[i..i + 4]);
        let vto = f64x4::from(&self.vto[i..i + 4]);
        let beta = f64x4::from(&self.beta[i..i + 4]);
        let lambda = f64x4::from(&self.lambda[i..i + 4]);
        let mult = f64x4::from(&self.mult[i..i + 4]);

        // Effective voltages
        let vgs_eff = polarity * vgs;
        let vds_eff = polarity * vds;

        // Gate overdrive: Vgt = Vgs - Vto (Vto is negative for NJF)
        let vgt = vgs_eff - vto;

        // Shichman-Hodges model
        // Cutoff: Vgt <= 0 -> Ids = 0
        // Linear: 0 < Vds < Vgt -> Ids = beta * (2*Vgt*Vds - Vds²) * (1 + lambda*Vds)
        // Saturation: Vds >= Vgt -> Ids = beta * Vgt² * (1 + lambda*Vds)

        let vgt_pos = vgt.max(f64x4::splat(0.0));
        let vds_pos = vds_eff.max(f64x4::splat(1e-12));
        let vdsat = vgt_pos.min(vds_pos);

        // Unified current
        let ids_core = beta * (f64x4::splat(2.0) * vgt_pos * vdsat - vdsat * vdsat);
        let ids = mult * polarity * ids_core * (f64x4::splat(1.0) + lambda * vds_pos);

        // Conductances
        let gm = mult * beta * f64x4::splat(2.0) * vdsat * (f64x4::splat(1.0) + lambda * vds_pos);
        let in_sat = simd_step(vds_pos - vgt_pos);
        let gds_lin = mult
            * beta
            * f64x4::splat(2.0)
            * (vgt_pos - vdsat)
            * (f64x4::splat(1.0) + lambda * vds_pos);
        let gds_sat = ids_core.abs() * lambda * mult;
        let gds = simd_blend(in_sat, gds_sat, gds_lin).max(f64x4::splat(1e-12));

        // Equivalent current
        let ids_eq = ids - gm * vgs - gds * vds;

        // Store results
        store_f64x4(ids, &mut self.ids[i..]);
        store_f64x4(gm, &mut self.gm[i..]);
        store_f64x4(gds, &mut self.gds[i..]);
        store_f64x4(ids_eq, &mut self.ids_eq[i..]);

        // Update regions
        for j in 0..4 {
            let idx = i + j;
            let p = self.polarity[idx];
            let vgt_val = p * self.vgs[idx] - self.vto[idx];
            let vds_val = p * self.vds[idx];
            self.region[idx] = if vgt_val <= 0.0 {
                JfetRegion::Cutoff
            } else if vds_val < vgt_val {
                JfetRegion::Linear
            } else {
                JfetRegion::Saturation
            };
        }
    }

    /// Scalar evaluation.
    fn evaluate_scalar(&mut self, i: usize) {
        let p = self.polarity[i];
        let vgs_eff = p * self.vgs[i];
        let vds_eff = (p * self.vds[i]).max(1e-12);

        let vgt = vgs_eff - self.vto[i];
        let vgt_pos = vgt.max(0.0);
        let vdsat = vgt_pos.min(vds_eff);

        let ids_core = self.beta[i] * (2.0 * vgt_pos * vdsat - vdsat * vdsat);
        self.ids[i] = self.mult[i] * p * ids_core * (1.0 + self.lambda[i] * vds_eff);

        self.gm[i] = self.mult[i] * self.beta[i] * 2.0 * vdsat * (1.0 + self.lambda[i] * vds_eff);
        let gds_lin = self.mult[i]
            * self.beta[i]
            * 2.0
            * (vgt_pos - vdsat)
            * (1.0 + self.lambda[i] * vds_eff);
        let gds_sat = ids_core.abs() * self.lambda[i] * self.mult[i];
        self.gds[i] = if vds_eff >= vgt_pos { gds_sat } else { gds_lin }.max(1e-12);

        self.ids_eq[i] = self.ids[i] - self.gm[i] * self.vgs[i] - self.gds[i] * self.vds[i];

        self.region[i] = if vgt <= 0.0 {
            JfetRegion::Cutoff
        } else if vds_eff < vgt_pos {
            JfetRegion::Linear
        } else {
            JfetRegion::Saturation
        };
    }

    #[cfg(not(feature = "simd"))]
    pub fn evaluate(&mut self) {
        for i in 0..self.len() {
            self.evaluate_scalar(i);
        }
    }

    /// Stamp all JFETs to matrix and RHS.
    #[inline]
    pub fn stamp(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        for i in 0..self.len() {
            let gm = self.gm[i];
            let gds = self.gds[i];
            let ids_eq = self.ids_eq[i];

            // Simplified stamp (ignoring gate junction currents for speed)
            if let Some(idx) = self.idx_dd[i] {
                matrix.stamp_direct(idx, gds);
            }
            if let Some(idx) = self.idx_dg[i] {
                matrix.stamp_direct(idx, gm);
            }
            if let Some(idx) = self.idx_ds[i] {
                matrix.stamp_direct(idx, -gm - gds);
            }
            if let Some(idx) = self.idx_sd[i] {
                matrix.stamp_direct(idx, -gds);
            }
            if let Some(idx) = self.idx_sg[i] {
                matrix.stamp_direct(idx, -gm);
            }
            if let Some(idx) = self.idx_ss[i] {
                matrix.stamp_direct(idx, gm + gds);
            }

            // RHS
            if let Some(idx) = self.rhs_drain[i] {
                rhs[idx] -= ids_eq;
            }
            if let Some(idx) = self.rhs_source[i] {
                rhs[idx] += ids_eq;
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
// SIMD Helper Functions
//=============================================================================

#[cfg(feature = "simd")]
#[inline]
fn simd_step(x: f64x4) -> f64x4 {
    let arr: [f64; 4] = x.into();
    f64x4::from([
        if arr[0] >= 0.0 { 1.0 } else { 0.0 },
        if arr[1] >= 0.0 { 1.0 } else { 0.0 },
        if arr[2] >= 0.0 { 1.0 } else { 0.0 },
        if arr[3] >= 0.0 { 1.0 } else { 0.0 },
    ])
}

#[cfg(feature = "simd")]
#[inline]
fn simd_blend(mask: f64x4, a: f64x4, b: f64x4) -> f64x4 {
    let m: [f64; 4] = mask.into();
    let av: [f64; 4] = a.into();
    let bv: [f64; 4] = b.into();
    f64x4::from([
        if m[0] > 0.5 { av[0] } else { bv[0] },
        if m[1] > 0.5 { av[1] } else { bv[1] },
        if m[2] > 0.5 { av[2] } else { bv[2] },
        if m[3] > 0.5 { av[3] } else { bv[3] },
    ])
}

#[cfg(feature = "simd")]
#[inline]
fn store_f64x4(v: f64x4, dst: &mut [Value]) {
    let arr: [f64; 4] = v.into();
    dst[0] = arr[0];
    dst[1] = arr[1];
    dst[2] = arr[2];
    dst[3] = arr[3];
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_jfets_creation() {
        let mut batch = BatchJfets::new();
        batch.add(1, 2, 0, JfetType::NJF, -2.0, 1e-4, 0.01, 1e-14, 1.0);
        batch.add(3, 4, 0, JfetType::PJF, 2.0, 1e-4, 0.01, 1e-14, 1.0);

        assert_eq!(batch.len(), 2);
        assert!(!batch.is_empty());
    }

    #[test]
    fn test_batch_jfet_evaluate() {
        let mut batch = BatchJfets::new();

        // Add 8 NJFETs
        for _ in 0..8 {
            batch.add(1, 2, 0, JfetType::NJF, -2.0, 1e-4, 0.01, 1e-14, 1.0);
        }

        // Set to saturation region
        for i in 0..8 {
            batch.vgs[i] = 0.0; // At pinchoff
            batch.vds[i] = 5.0;
        }

        batch.evaluate();

        for i in 0..8 {
            assert!(batch.ids[i] > 0.0, "Expected positive drain current");
            assert!(batch.gm[i] > 0.0);
        }
    }
}
