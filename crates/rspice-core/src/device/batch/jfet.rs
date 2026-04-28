//! Batch JFET Storage and Evaluation
//!
//! SoA storage for JFETs (Shichman-Hodges model) to enable SIMD batch evaluation.

use crate::Value;
use crate::circuit::NodeId;
use crate::device::mosfet::JfetType;
use crate::solver::{CscIndex, StaticMatrix};

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
    /// HFET-style DIBL/overdrive modulation coefficient
    pub eta: Vec<Value>,
    /// HFET/MESFET low-field channel conductivity term
    pub sigma0: Vec<Value>,
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
    /// Gate-source diode equivalent current source
    pub igs_eq: Vec<Value>,
    /// Gate-drain diode equivalent current source
    pub igd_eq: Vec<Value>,
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
            eta: Vec::with_capacity(capacity),
            sigma0: Vec::with_capacity(capacity),
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
            igs_eq: Vec::with_capacity(capacity),
            igd_eq: Vec::with_capacity(capacity),
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
        eta: Value,
        sigma0: Value,
        is: Value,
        n_vt: Value,
        mult: Value,
    ) {
        self.node_drain.push(node_drain);
        self.node_gate.push(node_gate);
        self.node_source.push(node_source);

        self.polarity.push(jfet_type.polarity());
        self.vto.push(vto);
        self.beta.push(beta);
        self.lambda.push(lambda);
        self.eta.push(eta);
        self.sigma0.push(sigma0);
        self.is.push(is);
        self.n_vt.push(n_vt.max(1e-12));
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
        self.igs_eq.push(0.0);
        self.igd_eq.push(0.0);
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
        // Accuracy-first path: use the full scalar model (reverse conduction
        // and gate junction Jacobians) for each device. This keeps SIMD and
        // non-SIMD behavior identical.
        for i in 0..self.len() {
            self.evaluate_scalar(i);
        }
    }

    /// Scalar evaluation.
    fn evaluate_scalar(&mut self, i: usize) {
        let p = self.polarity[i];
        let vgs = self.vgs[i];
        let vds = self.vds[i];
        let vgd = vgs - vds;
        let vto = self.vto[i];
        let beta = self.beta[i] * self.mult[i];
        let lambda = self.lambda[i];

        // Channel current and Jacobian match scalar JFET model implementation.
        let vgs_int = p * vgs;
        let vds_int = p * vds;
        let vgst = vgs_int - vto;

        let (ids, gm, gds, region) = if vgst <= 0.0 {
            (0.0, 0.0, 0.0, JfetRegion::Cutoff)
        } else if vds_int < 0.0 {
            let vds_rev = -vds_int;
            let vgs_rev = vgs_int - vds_int;
            let vgst_rev = vgs_rev - vto;

            if vgst_rev <= 0.0 {
                (0.0, 0.0, 0.0, JfetRegion::Cutoff)
            } else if vds_rev <= vgst_rev {
                let ids_fwd = beta
                    * (2.0 * vgst_rev * vds_rev - vds_rev * vds_rev)
                    * (1.0 + lambda * vds_rev);
                let gm_fwd = 2.0 * beta * vds_rev * (1.0 + lambda * vds_rev);
                let gds_fwd = beta * 2.0 * (vgst_rev - vds_rev) * (1.0 + lambda * vds_rev)
                    + beta * (2.0 * vgst_rev * vds_rev - vds_rev * vds_rev) * lambda;
                (-ids_fwd, -gm_fwd, gm_fwd + gds_fwd, JfetRegion::Linear)
            } else {
                let ids_fwd = beta * vgst_rev * vgst_rev * (1.0 + lambda * vds_rev);
                let gm_fwd = 2.0 * beta * vgst_rev * (1.0 + lambda * vds_rev);
                let gds_fwd = beta * vgst_rev * vgst_rev * lambda;
                (-ids_fwd, -gm_fwd, gm_fwd + gds_fwd, JfetRegion::Saturation)
            }
        } else if vds_int <= vgst {
            let ids = beta * (2.0 * vgst * vds_int - vds_int * vds_int) * (1.0 + lambda * vds_int);
            let gm = 2.0 * beta * vds_int * (1.0 + lambda * vds_int);
            let gds = beta * 2.0 * (vgst - vds_int) * (1.0 + lambda * vds_int)
                + beta * (2.0 * vgst * vds_int - vds_int * vds_int) * lambda;
            (ids, gm, gds, JfetRegion::Linear)
        } else {
            let ids = beta * vgst * vgst * (1.0 + lambda * vds_int);
            let gm = 2.0 * beta * vgst * (1.0 + lambda * vds_int);
            let gds = beta * vgst * vgst * lambda;
            (ids, gm, gds, JfetRegion::Saturation)
        };

        self.ids[i] = p * ids;
        self.gm[i] = gm;
        self.gds[i] = gds.max(1e-12);
        self.ids_eq[i] = self.ids[i] - self.gm[i] * vgs - self.gds[i] * vds;
        self.region[i] = region;

        // Gate junction diodes with SPICE-style limiting.
        let isat = self.is[i] * self.mult[i];
        let nvt = self.n_vt[i].max(1e-12);
        let v_crit = 80.0 * nvt;
        let v_rev = -5.0 * nvt;

        let eval_diode = |v_ak: Value| -> (Value, Value) {
            if v_ak > v_crit {
                let exp_crit = (v_crit / nvt).exp();
                let i_crit = isat * (exp_crit - 1.0);
                let g_crit = (isat / nvt) * exp_crit;
                (i_crit + g_crit * (v_ak - v_crit), g_crit.max(1e-15))
            } else if v_ak < v_rev {
                (-isat, 1e-15)
            } else {
                let exp_term = (v_ak / nvt).exp();
                (
                    isat * (exp_term - 1.0),
                    ((isat / nvt) * exp_term).max(1e-15),
                )
            }
        };

        let (igs_int, ggs) = eval_diode(p * vgs);
        let (igd_int, ggd) = eval_diode(p * vgd);
        self.ggs[i] = ggs;
        self.ggd[i] = ggd;

        let igs = p * igs_int; // current from gate to source
        let igd = p * igd_int; // current from gate to drain
        self.igs_eq[i] = igs - ggs * vgs;
        self.igd_eq[i] = igd - ggd * vgd;
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
            let ggs = self.ggs[i];
            let ggd = self.ggd[i];
            let ids_eq = self.ids_eq[i];
            let igs_eq = self.igs_eq[i];
            let igd_eq = self.igd_eq[i];

            // Drain row
            if let Some(idx) = self.idx_dd[i] {
                matrix.stamp_direct(idx, gds + ggd);
            }
            if let Some(idx) = self.idx_dg[i] {
                matrix.stamp_direct(idx, gm - ggd);
            }
            if let Some(idx) = self.idx_ds[i] {
                matrix.stamp_direct(idx, -gm - gds);
            }

            // Gate row
            if let Some(idx) = self.idx_gd[i] {
                matrix.stamp_direct(idx, -ggd);
            }
            if let Some(idx) = self.idx_gg[i] {
                matrix.stamp_direct(idx, ggs + ggd);
            }
            if let Some(idx) = self.idx_gs[i] {
                matrix.stamp_direct(idx, -ggs);
            }

            // Source row
            if let Some(idx) = self.idx_sd[i] {
                matrix.stamp_direct(idx, -gds);
            }
            if let Some(idx) = self.idx_sg[i] {
                matrix.stamp_direct(idx, -gm - ggs);
            }
            if let Some(idx) = self.idx_ss[i] {
                matrix.stamp_direct(idx, gm + gds + ggs);
            }

            // RHS
            if let Some(idx) = self.rhs_drain[i] {
                rhs[idx] -= ids_eq - igd_eq;
            }
            if let Some(idx) = self.rhs_gate[i] {
                rhs[idx] -= igs_eq + igd_eq;
            }
            if let Some(idx) = self.rhs_source[i] {
                rhs[idx] += ids_eq + igs_eq;
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
// Tests
//=============================================================================
