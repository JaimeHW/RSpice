//! Batch BJT Storage and Evaluation
//!
//! SoA storage for BJTs (Ebers-Moll model) to enable SIMD batch evaluation.
//! This is critical for circuits with many transistors like differential
//! amplifiers and current mirrors.

#![allow(clippy::too_many_arguments)]

use crate::Value;
use crate::circuit::NodeId;
use crate::device::semiconductor::BjtType;
use crate::solver::{CscIndex, StaticMatrix};

#[cfg(feature = "simd")]
use wide::f64x4;

//=============================================================================
// Batch BJT Storage
//=============================================================================

/// Batch BJT storage with Struct-of-Arrays layout for SIMD evaluation.
///
/// Implements the Ebers-Moll model with Gummel-Poon enhancements for
/// high-injection effects.
#[derive(Debug, Default, Clone)]
pub struct BatchBjts {
    //=========================================================================
    // Connectivity
    //=========================================================================
    pub node_collector: Vec<NodeId>,
    pub node_base: Vec<NodeId>,
    pub node_emitter: Vec<NodeId>,

    //=========================================================================
    // Device Parameters (constant during simulation)
    //=========================================================================
    /// Polarity: +1.0 for NPN, -1.0 for PNP
    pub polarity: Vec<Value>,
    /// Saturation current (IS)
    pub is: Vec<Value>,
    /// Pre-computed nf * vt
    pub nf_vt: Vec<Value>,
    /// Pre-computed nr * vt
    pub nr_vt: Vec<Value>,
    /// Forward beta (BF)
    pub bf: Vec<Value>,
    /// Reverse beta (BR)
    pub br: Vec<Value>,
    /// Forward Early voltage (VAF)
    pub vaf: Vec<Value>,
    /// Knee current (IKF)
    pub ikf: Vec<Value>,

    //=========================================================================
    // Matrix Indices
    //=========================================================================
    // Collector row
    pub idx_cc: Vec<Option<CscIndex>>,
    pub idx_cb: Vec<Option<CscIndex>>,
    pub idx_ce: Vec<Option<CscIndex>>,
    // Base row
    pub idx_bc: Vec<Option<CscIndex>>,
    pub idx_bb: Vec<Option<CscIndex>>,
    pub idx_be: Vec<Option<CscIndex>>,
    // Emitter row
    pub idx_ec: Vec<Option<CscIndex>>,
    pub idx_eb: Vec<Option<CscIndex>>,
    pub idx_ee: Vec<Option<CscIndex>>,

    //=========================================================================
    // State Variables
    //=========================================================================
    pub vbe: Vec<Value>,
    pub vbc: Vec<Value>,
    pub vbe_prev: Vec<Value>,
    pub vbc_prev: Vec<Value>,

    /// Collector current
    pub ic: Vec<Value>,
    /// Base current
    pub ib: Vec<Value>,
    /// Transconductance gm = dIc/dVbe
    pub gm: Vec<Value>,
    /// Output conductance go = dIc/dVce
    pub go: Vec<Value>,
    /// Base-emitter conductance
    pub gbe: Vec<Value>,
    /// Base-collector conductance
    pub gbc: Vec<Value>,
    /// Equivalent collector current source
    pub ic_eq: Vec<Value>,
    /// Equivalent base current source
    pub ib_eq: Vec<Value>,

    //=========================================================================
    // RHS indices
    //=========================================================================
    rhs_collector: Vec<Option<usize>>,
    rhs_base: Vec<Option<usize>>,
    rhs_emitter: Vec<Option<usize>>,
}

impl BatchBjts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            node_collector: Vec::with_capacity(capacity),
            node_base: Vec::with_capacity(capacity),
            node_emitter: Vec::with_capacity(capacity),
            polarity: Vec::with_capacity(capacity),
            is: Vec::with_capacity(capacity),
            nf_vt: Vec::with_capacity(capacity),
            nr_vt: Vec::with_capacity(capacity),
            bf: Vec::with_capacity(capacity),
            br: Vec::with_capacity(capacity),
            vaf: Vec::with_capacity(capacity),
            ikf: Vec::with_capacity(capacity),
            idx_cc: Vec::with_capacity(capacity),
            idx_cb: Vec::with_capacity(capacity),
            idx_ce: Vec::with_capacity(capacity),
            idx_bc: Vec::with_capacity(capacity),
            idx_bb: Vec::with_capacity(capacity),
            idx_be: Vec::with_capacity(capacity),
            idx_ec: Vec::with_capacity(capacity),
            idx_eb: Vec::with_capacity(capacity),
            idx_ee: Vec::with_capacity(capacity),
            vbe: Vec::with_capacity(capacity),
            vbc: Vec::with_capacity(capacity),
            vbe_prev: Vec::with_capacity(capacity),
            vbc_prev: Vec::with_capacity(capacity),
            ic: Vec::with_capacity(capacity),
            ib: Vec::with_capacity(capacity),
            gm: Vec::with_capacity(capacity),
            go: Vec::with_capacity(capacity),
            gbe: Vec::with_capacity(capacity),
            gbc: Vec::with_capacity(capacity),
            ic_eq: Vec::with_capacity(capacity),
            ib_eq: Vec::with_capacity(capacity),
            rhs_collector: Vec::with_capacity(capacity),
            rhs_base: Vec::with_capacity(capacity),
            rhs_emitter: Vec::with_capacity(capacity),
        }
    }

    /// Add a BJT to the batch storage.
    pub fn add(
        &mut self,
        node_collector: NodeId,
        node_base: NodeId,
        node_emitter: NodeId,
        bjt_type: BjtType,
        is: Value,
        nf: Value,
        nr: Value,
        vt: Value,
        bf: Value,
        br: Value,
        vaf: Value,
        ikf: Value,
    ) {
        self.node_collector.push(node_collector);
        self.node_base.push(node_base);
        self.node_emitter.push(node_emitter);

        self.polarity.push(match bjt_type {
            BjtType::Npn => 1.0,
            BjtType::Pnp => -1.0,
        });
        self.is.push(is);
        self.nf_vt.push(nf * vt);
        self.nr_vt.push(nr * vt);
        self.bf.push(bf);
        self.br.push(br);
        self.vaf.push(vaf);
        self.ikf.push(ikf);

        // Initialize indices
        self.idx_cc.push(None);
        self.idx_cb.push(None);
        self.idx_ce.push(None);
        self.idx_bc.push(None);
        self.idx_bb.push(None);
        self.idx_be.push(None);
        self.idx_ec.push(None);
        self.idx_eb.push(None);
        self.idx_ee.push(None);

        // Initialize state
        self.vbe.push(0.0);
        self.vbc.push(0.0);
        self.vbe_prev.push(0.0);
        self.vbc_prev.push(0.0);
        self.ic.push(0.0);
        self.ib.push(0.0);
        self.gm.push(1e-12);
        self.go.push(1e-12);
        self.gbe.push(1e-12);
        self.gbc.push(1e-12);
        self.ic_eq.push(0.0);
        self.ib_eq.push(0.0);

        self.rhs_collector.push(if node_collector > 0 {
            Some(node_collector - 1)
        } else {
            None
        });
        self.rhs_base.push(if node_base > 0 {
            Some(node_base - 1)
        } else {
            None
        });
        self.rhs_emitter.push(if node_emitter > 0 {
            Some(node_emitter - 1)
        } else {
            None
        });
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.node_collector.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.node_collector.is_empty()
    }

    /// Link all BJTs to the sparse matrix.
    pub fn link(&mut self, matrix: &StaticMatrix) {
        for i in 0..self.len() {
            let c = self.node_collector[i];
            let b = self.node_base[i];
            let e = self.node_emitter[i];

            // Collector row
            if c > 0 {
                self.idx_cc[i] = matrix.get_index(c - 1, c - 1);
            }
            if c > 0 && b > 0 {
                self.idx_cb[i] = matrix.get_index(c - 1, b - 1);
            }
            if c > 0 && e > 0 {
                self.idx_ce[i] = matrix.get_index(c - 1, e - 1);
            }
            // Base row
            if b > 0 && c > 0 {
                self.idx_bc[i] = matrix.get_index(b - 1, c - 1);
            }
            if b > 0 {
                self.idx_bb[i] = matrix.get_index(b - 1, b - 1);
            }
            if b > 0 && e > 0 {
                self.idx_be[i] = matrix.get_index(b - 1, e - 1);
            }
            // Emitter row
            if e > 0 && c > 0 {
                self.idx_ec[i] = matrix.get_index(e - 1, c - 1);
            }
            if e > 0 && b > 0 {
                self.idx_eb[i] = matrix.get_index(e - 1, b - 1);
            }
            if e > 0 {
                self.idx_ee[i] = matrix.get_index(e - 1, e - 1);
            }
        }
    }

    /// Gather voltages from node solution.
    #[inline]
    pub fn gather_voltages(&mut self, voltages: &[Value]) {
        for i in 0..self.len() {
            let vc = if self.node_collector[i] == 0 {
                0.0
            } else {
                voltages
                    .get(self.node_collector[i] - 1)
                    .copied()
                    .unwrap_or(0.0)
            };
            let vb = if self.node_base[i] == 0 {
                0.0
            } else {
                voltages.get(self.node_base[i] - 1).copied().unwrap_or(0.0)
            };
            let ve = if self.node_emitter[i] == 0 {
                0.0
            } else {
                voltages
                    .get(self.node_emitter[i] - 1)
                    .copied()
                    .unwrap_or(0.0)
            };

            self.vbe_prev[i] = self.vbe[i];
            self.vbc_prev[i] = self.vbc[i];
            self.vbe[i] = vb - ve;
            self.vbc[i] = vb - vc;
        }
    }

    /// Evaluate all BJTs using SIMD where possible.
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

    /// Evaluate 4 BJTs using SIMD.
    #[cfg(feature = "simd")]
    fn evaluate_simd_chunk(&mut self, i: usize) {
        let vbe = f64x4::from(&self.vbe[i..i + 4]);
        let vbc = f64x4::from(&self.vbc[i..i + 4]);
        let polarity = f64x4::from(&self.polarity[i..i + 4]);
        let is = f64x4::from(&self.is[i..i + 4]);
        let nf_vt = f64x4::from(&self.nf_vt[i..i + 4]);
        let nr_vt = f64x4::from(&self.nr_vt[i..i + 4]);
        let bf = f64x4::from(&self.bf[i..i + 4]);
        let br = f64x4::from(&self.br[i..i + 4]);
        let vaf = f64x4::from(&self.vaf[i..i + 4]);
        let ikf = f64x4::from(&self.ikf[i..i + 4]);

        let vbe_eff = polarity * vbe;
        let vbc_eff = polarity * vbc;

        // Forward and reverse diode currents with limiting
        let vbe_limited = vbe_eff.min(f64x4::splat(80.0) * nf_vt);
        let vbc_limited = vbc_eff.min(f64x4::splat(80.0) * nr_vt);

        let exp_be = simd_exp(vbe_limited / nf_vt);
        let exp_bc = simd_exp(vbc_limited / nr_vt);

        let if_diode = is * (exp_be - f64x4::splat(1.0));
        let ir_diode = is * (exp_bc - f64x4::splat(1.0));

        // High-injection factor (Gummel-Poon)
        let ikf_ratio = if_diode / ikf.max(f64x4::splat(1e-6));
        let hf =
            f64x4::splat(1.0) / (f64x4::splat(1.0) + simd_sqrt(ikf_ratio)).max(f64x4::splat(0.1));

        // Currents
        let ic = polarity * (if_diode * hf - ir_diode - ir_diode / br);
        let ib = polarity * (if_diode / bf + ir_diode / br);

        // Conductances
        let g_diode_be = is / nf_vt * exp_be;
        let g_diode_bc = is / nr_vt * exp_bc;
        let gm = g_diode_be * hf;
        let go = ic.abs() / vaf.max(f64x4::splat(1e-6));
        let gbe = g_diode_be / bf;
        let gbc = g_diode_bc / br;

        // Equivalent current sources
        let vce = vbe - vbc; // Vce = Vbe - Vbc
        let ic_eq = ic - gm * vbe - go * vce;
        let ib_eq = ib - gbe * vbe - gbc * vbc;

        // Store results
        store_f64x4(ic, &mut self.ic[i..]);
        store_f64x4(ib, &mut self.ib[i..]);
        store_f64x4(gm, &mut self.gm[i..]);
        store_f64x4(go, &mut self.go[i..]);
        store_f64x4(gbe, &mut self.gbe[i..]);
        store_f64x4(gbc, &mut self.gbc[i..]);
        store_f64x4(ic_eq, &mut self.ic_eq[i..]);
        store_f64x4(ib_eq, &mut self.ib_eq[i..]);
    }

    /// Scalar evaluation.
    fn evaluate_scalar(&mut self, i: usize) {
        let p = self.polarity[i];
        let vbe_eff = p * self.vbe[i];
        let vbc_eff = p * self.vbc[i];

        let vbe_limited = vbe_eff.min(80.0 * self.nf_vt[i]);
        let vbc_limited = vbc_eff.min(80.0 * self.nr_vt[i]);

        let exp_be = (vbe_limited / self.nf_vt[i]).exp();
        let exp_bc = (vbc_limited / self.nr_vt[i]).exp();

        let if_diode = self.is[i] * (exp_be - 1.0);
        let ir_diode = self.is[i] * (exp_bc - 1.0);

        let ikf_ratio = if_diode / self.ikf[i].max(1e-6);
        let hf = 1.0 / (1.0 + ikf_ratio.sqrt()).max(0.1);

        self.ic[i] = p * (if_diode * hf - ir_diode - ir_diode / self.br[i]);
        self.ib[i] = p * (if_diode / self.bf[i] + ir_diode / self.br[i]);

        let g_diode_be = self.is[i] / self.nf_vt[i] * exp_be;
        let g_diode_bc = self.is[i] / self.nr_vt[i] * exp_bc;
        self.gm[i] = g_diode_be * hf;
        self.go[i] = self.ic[i].abs() / self.vaf[i].max(1e-6);
        self.gbe[i] = g_diode_be / self.bf[i];
        self.gbc[i] = g_diode_bc / self.br[i];

        let vce = self.vbe[i] - self.vbc[i];
        self.ic_eq[i] = self.ic[i] - self.gm[i] * self.vbe[i] - self.go[i] * vce;
        self.ib_eq[i] = self.ib[i] - self.gbe[i] * self.vbe[i] - self.gbc[i] * self.vbc[i];
    }

    #[cfg(not(feature = "simd"))]
    pub fn evaluate(&mut self) {
        for i in 0..self.len() {
            self.evaluate_scalar(i);
        }
    }

    /// Stamp all BJTs to matrix and RHS.
    #[inline]
    pub fn stamp(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        for i in 0..self.len() {
            let gm = self.gm[i];
            let go = self.go[i];
            let gbe = self.gbe[i];
            let gbc = self.gbc[i];
            let ic_eq = self.ic_eq[i];
            let ib_eq = self.ib_eq[i];

            // Collector row
            if let Some(idx) = self.idx_cc[i] {
                matrix.stamp_direct(idx, go + gbc);
            }
            if let Some(idx) = self.idx_cb[i] {
                matrix.stamp_direct(idx, gm - gbc);
            }
            if let Some(idx) = self.idx_ce[i] {
                matrix.stamp_direct(idx, -gm - go);
            }
            // Base row
            if let Some(idx) = self.idx_bc[i] {
                matrix.stamp_direct(idx, -gbc);
            }
            if let Some(idx) = self.idx_bb[i] {
                matrix.stamp_direct(idx, gbe + gbc);
            }
            if let Some(idx) = self.idx_be[i] {
                matrix.stamp_direct(idx, -gbe);
            }
            // Emitter row
            if let Some(idx) = self.idx_ec[i] {
                matrix.stamp_direct(idx, -go);
            }
            if let Some(idx) = self.idx_eb[i] {
                matrix.stamp_direct(idx, -gm - gbe);
            }
            if let Some(idx) = self.idx_ee[i] {
                matrix.stamp_direct(idx, gm + go + gbe);
            }

            // RHS
            if let Some(idx) = self.rhs_collector[i] {
                rhs[idx] -= ic_eq;
            }
            if let Some(idx) = self.rhs_base[i] {
                rhs[idx] -= ib_eq;
            }
            if let Some(idx) = self.rhs_emitter[i] {
                rhs[idx] += ic_eq + ib_eq;
            }
        }
    }

    /// Check convergence.
    ///
    /// Uses both absolute and relative tolerance to avoid false non-convergence
    /// when junction voltages are large or oscillating slightly.
    #[inline]
    pub fn all_converged(&self, tolerance: Value) -> bool {
        // Use a relative tolerance of 0.1% plus absolute tolerance
        // This prevents tiny oscillations at larger voltages from blocking convergence
        const REL_TOL: Value = 1e-3; // 0.1% relative tolerance

        for i in 0..self.len() {
            let delta_vbe = (self.vbe[i] - self.vbe_prev[i]).abs();
            let delta_vbc = (self.vbc[i] - self.vbc_prev[i]).abs();

            // Effective tolerance = max(abs_tol, rel_tol * |V|)
            let tol_vbe = tolerance.max(REL_TOL * self.vbe[i].abs());
            let tol_vbc = tolerance.max(REL_TOL * self.vbc[i].abs());

            if delta_vbe >= tol_vbe || delta_vbc >= tol_vbc {
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
use crate::simd::{exp_f64x4 as simd_exp, sqrt_f64x4 as simd_sqrt, store_f64x4};

//=============================================================================
// Tests
//=============================================================================
