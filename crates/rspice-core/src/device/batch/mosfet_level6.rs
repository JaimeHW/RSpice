//! Batch Level 6 MOSFET Storage and Evaluation
//!
//! SoA storage for Level 6 (Double-Exponent) MOSFETs to enable SIMD batch evaluation.
//! Level 6 uses empirical fits that are simpler than BSIM models but more accurate
//! than Level 1 for certain characterizations.

use crate::Value;
use crate::circuit::NodeId;
use crate::device::mosfet::{MosRegion, MosType};
use crate::solver::{CscIndex, StaticMatrix};

#[cfg(feature = "simd")]
use wide::f64x4;

//=============================================================================
// Batch Level 6 MOSFET Storage
//=============================================================================

/// Batch Level 6 MOSFET storage with Struct-of-Arrays layout for SIMD evaluation.
///
/// Level 6 uses the double-exponent model:
/// Id = KC * W/L * (Vgs - Vth)^NC * (1 - exp(-Vds * KV))^NV * (1 + LAMBDA * Vds)
#[derive(Debug, Default, Clone)]
pub struct BatchMosfetsLevel6 {
    //=========================================================================
    // Connectivity
    //=========================================================================
    pub node_drain: Vec<NodeId>,
    pub node_gate: Vec<NodeId>,
    pub node_source: Vec<NodeId>,
    pub node_bulk: Vec<NodeId>,

    //=========================================================================
    // Device Parameters
    //=========================================================================
    /// Polarity: +1.0 for NMOS, -1.0 for PMOS
    pub polarity: Vec<Value>,
    /// W/L ratio
    pub wl: Vec<Value>,
    /// Threshold voltage (VTO)
    pub vto: Vec<Value>,
    /// Body effect coefficient (GAMMA)
    pub gamma: Vec<Value>,
    /// Surface potential (PHI)
    pub phi: Vec<Value>,
    /// Current gain coefficient (KC)
    pub kc: Vec<Value>,
    /// Current gain exponent (NC)
    pub nc: Vec<Value>,
    /// Voltage clipping coefficient (KV)
    pub kv: Vec<Value>,
    /// Voltage clipping exponent (NV)
    pub nv: Vec<Value>,
    /// First-order CLM (LAMBDA0)
    pub lambda0: Vec<Value>,
    /// Second-order CLM (LAMBDA1)
    pub lambda1: Vec<Value>,

    //=========================================================================
    // Matrix Indices
    //=========================================================================
    pub idx_dd: Vec<Option<CscIndex>>,
    pub idx_dg: Vec<Option<CscIndex>>,
    pub idx_ds: Vec<Option<CscIndex>>,
    pub idx_db: Vec<Option<CscIndex>>,
    pub idx_sd: Vec<Option<CscIndex>>,
    pub idx_sg: Vec<Option<CscIndex>>,
    pub idx_ss: Vec<Option<CscIndex>>,
    pub idx_sb: Vec<Option<CscIndex>>,

    //=========================================================================
    // State Variables
    //=========================================================================
    pub vgs: Vec<Value>,
    pub vds: Vec<Value>,
    pub vbs: Vec<Value>,
    pub vgs_prev: Vec<Value>,
    pub vds_prev: Vec<Value>,

    pub id: Vec<Value>,
    pub gm: Vec<Value>,
    pub gds: Vec<Value>,
    pub gmb: Vec<Value>,
    pub id_eq: Vec<Value>,
    pub region: Vec<MosRegion>,

    rhs_drain: Vec<Option<usize>>,
    rhs_source: Vec<Option<usize>>,
}

impl BatchMosfetsLevel6 {
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
            wl: Vec::with_capacity(capacity),
            vto: Vec::with_capacity(capacity),
            gamma: Vec::with_capacity(capacity),
            phi: Vec::with_capacity(capacity),
            kc: Vec::with_capacity(capacity),
            nc: Vec::with_capacity(capacity),
            kv: Vec::with_capacity(capacity),
            nv: Vec::with_capacity(capacity),
            lambda0: Vec::with_capacity(capacity),
            lambda1: Vec::with_capacity(capacity),
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

    /// Add a Level 6 MOSFET.
    #[allow(clippy::too_many_arguments)]
    pub fn add(
        &mut self,
        node_drain: NodeId,
        node_gate: NodeId,
        node_source: NodeId,
        node_bulk: NodeId,
        mos_type: MosType,
        wl: Value,
        vto: Value,
        gamma: Value,
        phi: Value,
        kc: Value,
        nc: Value,
        kv: Value,
        nv: Value,
        lambda0: Value,
        lambda1: Value,
    ) {
        self.node_drain.push(node_drain);
        self.node_gate.push(node_gate);
        self.node_source.push(node_source);
        self.node_bulk.push(node_bulk);

        self.polarity.push(match mos_type {
            MosType::Nmos => 1.0,
            MosType::Pmos => -1.0,
        });
        self.wl.push(wl);
        self.vto.push(vto);
        self.gamma.push(gamma);
        self.phi.push(phi);
        self.kc.push(kc);
        self.nc.push(nc);
        self.kv.push(kv);
        self.nv.push(nv);
        self.lambda0.push(lambda0);
        self.lambda1.push(lambda1);

        // Initialize indices
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

    /// Link to matrix.
    pub fn link(&mut self, matrix: &StaticMatrix) {
        for i in 0..self.len() {
            let d = self.node_drain[i];
            let g = self.node_gate[i];
            let s = self.node_source[i];
            let b = self.node_bulk[i];

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

    /// Gather voltages.
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

    /// SIMD evaluation for Level 6.
    #[cfg(feature = "simd")]
    fn evaluate_simd_chunk(&mut self, i: usize) {
        let vgs = f64x4::from(&self.vgs[i..i + 4]);
        let vds = f64x4::from(&self.vds[i..i + 4]);
        let vbs = f64x4::from(&self.vbs[i..i + 4]);
        let polarity = f64x4::from(&self.polarity[i..i + 4]);
        let wl = f64x4::from(&self.wl[i..i + 4]);
        let vto = f64x4::from(&self.vto[i..i + 4]);
        let gamma = f64x4::from(&self.gamma[i..i + 4]);
        let phi = f64x4::from(&self.phi[i..i + 4]);
        let kc = f64x4::from(&self.kc[i..i + 4]);
        let nc = f64x4::from(&self.nc[i..i + 4]);
        let kv = f64x4::from(&self.kv[i..i + 4]);
        let nv = f64x4::from(&self.nv[i..i + 4]);
        let lambda0 = f64x4::from(&self.lambda0[i..i + 4]);
        let lambda1 = f64x4::from(&self.lambda1[i..i + 4]);

        let vgs_eff = polarity * vgs;
        let vds_eff = (polarity * vds).max(f64x4::splat(1e-12));
        let vbs_eff = polarity * vbs;

        // Threshold voltage with body effect
        let phi_vbs = (phi - vbs_eff).max(f64x4::splat(0.0));
        let vth = vto + gamma * (simd_sqrt(phi_vbs) - simd_sqrt(phi));

        // Gate overdrive (smooth positive)
        let vgt = (vgs_eff - vth).max(f64x4::splat(1e-12));

        // Level 6 current calculation:
        // Id = KC * W/L * Vgt^NC * (1 - exp(-KV*Vds))^NV * (1 + LAMBDA0*Vds + LAMBDA1*Vds^2)
        let current_term = kc * wl * simd_pow(vgt, nc);
        let exp_term = simd_exp(-kv * vds_eff);
        let sat_factor = simd_pow((f64x4::splat(1.0) - exp_term).max(f64x4::splat(0.0)), nv);
        let clm = f64x4::splat(1.0) + lambda0 * vds_eff + lambda1 * vds_eff * vds_eff;
        let id = polarity * current_term * sat_factor * clm;

        // gm = dId/dVgs = Id / Vgt * NC
        let gm = (id.abs() / vgt * nc).max(f64x4::splat(1e-12));

        // gds = dId/dVds (from saturation factor and CLM)
        // d(sat_factor)/dVds = NV * (1-exp(-KV*Vds))^(NV-1) * KV * exp(-KV*Vds)
        let d_sat =
            nv * simd_pow(
                (f64x4::splat(1.0) - exp_term).max(f64x4::splat(1e-12)),
                nv - f64x4::splat(1.0),
            ) * kv
                * exp_term;
        let d_clm = lambda0 + f64x4::splat(2.0) * lambda1 * vds_eff;
        let gds = (current_term * (d_sat * clm + sat_factor * d_clm))
            .abs()
            .max(f64x4::splat(1e-12));

        let gmb = f64x4::splat(0.0);
        let id_eq = id - gm * vgs - gds * vds - gmb * vbs;

        store_f64x4(id, &mut self.id[i..]);
        store_f64x4(gm, &mut self.gm[i..]);
        store_f64x4(gds, &mut self.gds[i..]);
        store_f64x4(gmb, &mut self.gmb[i..]);
        store_f64x4(id_eq, &mut self.id_eq[i..]);

        // Update regions
        for j in 0..4 {
            let idx = i + j;
            let vgt_val = self.vgs[idx] * self.polarity[idx] - self.vto[idx];
            self.region[idx] = if vgt_val <= 0.0 {
                MosRegion::Cutoff
            } else {
                MosRegion::Saturation // Level 6 uses smooth transition
            };
        }
    }

    /// Scalar evaluation.
    fn evaluate_scalar(&mut self, i: usize) {
        let p = self.polarity[i];
        let vgs_eff = p * self.vgs[i];
        let vds_eff = (p * self.vds[i]).max(1e-12);
        let vbs_eff = p * self.vbs[i];

        let phi_vbs = (self.phi[i] - vbs_eff).max(0.0);
        let vth = self.vto[i] + self.gamma[i] * (phi_vbs.sqrt() - self.phi[i].sqrt());
        let vgt = (vgs_eff - vth).max(1e-12);

        let current_term = self.kc[i] * self.wl[i] * vgt.powf(self.nc[i]);
        let exp_term = (-self.kv[i] * vds_eff).exp();
        let sat_factor = (1.0 - exp_term).max(0.0).powf(self.nv[i]);
        let clm = 1.0 + self.lambda0[i] * vds_eff + self.lambda1[i] * vds_eff * vds_eff;

        self.id[i] = p * current_term * sat_factor * clm;
        self.gm[i] = (self.id[i].abs() / vgt * self.nc[i]).max(1e-12);

        let d_sat =
            self.nv[i] * (1.0 - exp_term).max(1e-12).powf(self.nv[i] - 1.0) * self.kv[i] * exp_term;
        let d_clm = self.lambda0[i] + 2.0 * self.lambda1[i] * vds_eff;
        self.gds[i] = (current_term * (d_sat * clm + sat_factor * d_clm))
            .abs()
            .max(1e-12);
        self.gmb[i] = 0.0;

        self.id_eq[i] = self.id[i] - self.gm[i] * self.vgs[i] - self.gds[i] * self.vds[i];

        self.region[i] = if vgt <= 0.0 {
            MosRegion::Cutoff
        } else {
            MosRegion::Saturation
        };
    }

    #[cfg(not(feature = "simd"))]
    pub fn evaluate(&mut self) {
        for i in 0..self.len() {
            self.evaluate_scalar(i);
        }
    }

    /// Stamp to matrix.
    #[inline]
    pub fn stamp(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        for i in 0..self.len() {
            let gm = self.gm[i];
            let gds = self.gds[i];
            let gmb = self.gmb[i];
            let id_eq = self.id_eq[i];

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
    exp_f64x4 as simd_exp, pow_f64x4 as simd_pow, sqrt_f64x4 as simd_sqrt, store_f64x4,
};

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_level6_creation() {
        let mut batch = BatchMosfetsLevel6::new();

        batch.add(
            1,
            2,
            0,
            0,
            MosType::Nmos,
            10.0,
            0.7,
            0.4,
            0.65,
            110e-6,
            1.0,
            0.9,
            0.9,
            0.01,
            0.0,
        );

        assert_eq!(batch.len(), 1);
        assert!(!batch.is_empty());
    }

    #[test]
    fn test_batch_level6_evaluate() {
        let mut batch = BatchMosfetsLevel6::new();

        // Add 8 NMOS
        for _ in 0..8 {
            batch.add(
                1,
                2,
                0,
                0,
                MosType::Nmos,
                10.0,
                0.7,
                0.4,
                0.65,
                110e-6,
                1.0,
                0.9,
                0.9,
                0.01,
                0.0,
            );
        }

        for i in 0..8 {
            batch.vgs[i] = 1.5;
            batch.vds[i] = 2.0;
            batch.vbs[i] = 0.0;
        }

        batch.evaluate();

        for i in 0..8 {
            assert!(batch.id[i] > 0.0, "Expected positive drain current");
            assert!(batch.gm[i] > 0.0);
            assert!(batch.gds[i] > 0.0);
        }
    }
}
