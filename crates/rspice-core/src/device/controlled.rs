//! Controlled sources (VCVS, VCCS, CCCS, CCVS)
//!
//! Implements the standard SPICE controlled sources:
//! - E: Voltage-Controlled Voltage Source (VCVS)
//! - F: Current-Controlled Current Source (CCCS)
//! - G: Voltage-Controlled Current Source (VCCS)
//! - H: Current-Controlled Voltage Source (CCVS)

use crate::solver::{StaticMatrix, TripletMatrix};
use crate::{Value, circuit::NodeId};

//=============================================================================
// VCVS (E-element): V_out = gain * V(nc+, nc-)
//=============================================================================

/// Voltage-Controlled Voltage Source storage (SoA)
#[derive(Debug, Default)]
pub struct Vcvs {
    pub names: Vec<String>,
    pub node_pos: Vec<NodeId>,
    pub node_neg: Vec<NodeId>,
    pub ctrl_pos: Vec<NodeId>,
    pub ctrl_neg: Vec<NodeId>,
    pub branch_indices: Vec<NodeId>,
    pub gains: Vec<Value>,
}

impl Vcvs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        ctrl_pos: NodeId,
        ctrl_neg: NodeId,
        branch_idx: NodeId,
        gain: Value,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.ctrl_pos.push(ctrl_pos);
        self.ctrl_neg.push(ctrl_neg);
        self.branch_indices.push(branch_idx);
        self.gains.push(gain);
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Stamp all VCVS elements
    /// MNA: Vout = gain * Vctrl
    /// Branch equation: V(n+) - V(n-) = gain * (V(nc+) - V(nc-))
    #[inline]
    pub fn stamp_all(&self, matrix: &mut TripletMatrix, num_nodes: usize) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let cp = self.ctrl_pos[i];
            let cn = self.ctrl_neg[i];
            let br = self.branch_indices[i];
            let gain = self.gains[i];
            let br_idx = num_nodes + br - 1;

            // Branch current flows: n+ to n- (standard for voltage source)
            if np > 0 {
                matrix.push(np - 1, br_idx, 1.0);
                matrix.push(br_idx, np - 1, 1.0);
            }
            if nn > 0 {
                matrix.push(nn - 1, br_idx, -1.0);
                matrix.push(br_idx, nn - 1, -1.0);
            }

            // Control voltage coefficient: -gain * V(nc+) + gain * V(nc-)
            if cp > 0 {
                matrix.push(br_idx, cp - 1, -gain);
            }
            if cn > 0 {
                matrix.push(br_idx, cn - 1, gain);
            }
        }
    }

    /// Stamp all VCVS elements using direct StaticMatrix access
    #[inline]
    pub fn stamp_all_direct<F>(&self, matrix: &mut StaticMatrix, branch_idx_fn: F)
    where
        F: Fn(usize) -> usize,
    {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let cp = self.ctrl_pos[i];
            let cn = self.ctrl_neg[i];
            let br_ordinal = self.branch_indices[i];
            let gain = self.gains[i];
            let br_idx = branch_idx_fn(br_ordinal);

            // Branch current flows: n+ to n- (standard for voltage source)
            if np > 0 {
                matrix.add(np - 1, br_idx - 1, 1.0);
                matrix.add(br_idx - 1, np - 1, 1.0);
            }
            if nn > 0 {
                matrix.add(nn - 1, br_idx - 1, -1.0);
                matrix.add(br_idx - 1, nn - 1, -1.0);
            }

            // Control voltage coefficient: -gain * V(nc+) + gain * V(nc-)
            if cp > 0 {
                matrix.add(br_idx - 1, cp - 1, -gain);
            }
            if cn > 0 {
                matrix.add(br_idx - 1, cn - 1, gain);
            }
        }
    }
}

//=============================================================================
// VCCS (G-element): I_out = gm * V(nc+, nc-)
//=============================================================================

/// Voltage-Controlled Current Source storage (SoA)
#[derive(Debug, Default)]
pub struct Vccs {
    pub names: Vec<String>,
    pub node_pos: Vec<NodeId>,
    pub node_neg: Vec<NodeId>,
    pub ctrl_pos: Vec<NodeId>,
    pub ctrl_neg: Vec<NodeId>,
    pub transconductances: Vec<Value>,
}

impl Vccs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        ctrl_pos: NodeId,
        ctrl_neg: NodeId,
        gm: Value,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.ctrl_pos.push(ctrl_pos);
        self.ctrl_neg.push(ctrl_neg);
        self.transconductances.push(gm);
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Stamp all VCCS elements
    /// Current flows from n+ to n-, controlled by V(nc+) - V(nc-)
    /// I = gm * (V(nc+) - V(nc-))
    #[inline]
    pub fn stamp_all(&self, matrix: &mut TripletMatrix) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let cp = self.ctrl_pos[i];
            let cn = self.ctrl_neg[i];
            let gm = self.transconductances[i];

            // Stamp gm into the matrix
            if np > 0 && cp > 0 {
                matrix.push(np - 1, cp - 1, gm);
            }
            if np > 0 && cn > 0 {
                matrix.push(np - 1, cn - 1, -gm);
            }
            if nn > 0 && cp > 0 {
                matrix.push(nn - 1, cp - 1, -gm);
            }
            if nn > 0 && cn > 0 {
                matrix.push(nn - 1, cn - 1, gm);
            }
        }
    }

    /// Stamp all VCCS elements using direct StaticMatrix access
    #[inline]
    pub fn stamp_all_direct(&self, matrix: &mut StaticMatrix) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let cp = self.ctrl_pos[i];
            let cn = self.ctrl_neg[i];
            let gm = self.transconductances[i];

            // Stamp gm into the matrix
            if np > 0 && cp > 0 {
                matrix.add(np - 1, cp - 1, gm);
            }
            if np > 0 && cn > 0 {
                matrix.add(np - 1, cn - 1, -gm);
            }
            if nn > 0 && cp > 0 {
                matrix.add(nn - 1, cp - 1, -gm);
            }
            if nn > 0 && cn > 0 {
                matrix.add(nn - 1, cn - 1, gm);
            }
        }
    }
}

//=============================================================================
// CCCS (F-element): I_out = gain * I_ctrl
//=============================================================================

/// Current-Controlled Current Source storage (SoA)
#[derive(Debug, Default)]
pub struct Cccs {
    pub names: Vec<String>,
    pub node_pos: Vec<NodeId>,
    pub node_neg: Vec<NodeId>,
    /// Branch index of the controlling voltage source
    pub ctrl_branch: Vec<NodeId>,
    pub gains: Vec<Value>,
}

impl Cccs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        ctrl_branch: NodeId,
        gain: Value,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.ctrl_branch.push(ctrl_branch);
        self.gains.push(gain);
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Stamp all CCCS elements
    /// Current I_out = gain * I_ctrl (branch current)
    #[inline]
    pub fn stamp_all(&self, matrix: &mut TripletMatrix, num_nodes: usize) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let cb = self.ctrl_branch[i];
            let gain = self.gains[i];
            let cb_idx = num_nodes + cb - 1;

            // Output current contribution to node equations
            if np > 0 {
                matrix.push(np - 1, cb_idx, gain);
            }
            if nn > 0 {
                matrix.push(nn - 1, cb_idx, -gain);
            }
        }
    }

    /// Stamp all CCCS elements using direct StaticMatrix access.
    #[inline]
    pub fn stamp_all_direct<F>(&self, matrix: &mut StaticMatrix, branch_idx_fn: F)
    where
        F: Fn(usize) -> usize,
    {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let cb_ordinal = self.ctrl_branch[i];
            let gain = self.gains[i];

            if cb_ordinal == 0 {
                continue;
            }
            let cb_idx = branch_idx_fn(cb_ordinal);

            if np > 0 {
                matrix.add(np - 1, cb_idx - 1, gain);
            }
            if nn > 0 {
                matrix.add(nn - 1, cb_idx - 1, -gain);
            }
        }
    }
}

//=============================================================================
// CCVS (H-element): V_out = rm * I_ctrl
//=============================================================================

/// Current-Controlled Voltage Source storage (SoA)
#[derive(Debug, Default)]
pub struct Ccvs {
    pub names: Vec<String>,
    pub node_pos: Vec<NodeId>,
    pub node_neg: Vec<NodeId>,
    pub branch_indices: Vec<NodeId>,
    /// Branch index of the controlling voltage source
    pub ctrl_branch: Vec<NodeId>,
    pub transresistances: Vec<Value>,
}

impl Ccvs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        branch_idx: NodeId,
        ctrl_branch: NodeId,
        rm: Value,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.branch_indices.push(branch_idx);
        self.ctrl_branch.push(ctrl_branch);
        self.transresistances.push(rm);
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Stamp all CCVS elements
    /// V_out = rm * I_ctrl
    #[inline]
    pub fn stamp_all(&self, matrix: &mut TripletMatrix, num_nodes: usize) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br = self.branch_indices[i];
            let cb = self.ctrl_branch[i];
            let rm = self.transresistances[i];
            let br_idx = num_nodes + br - 1;
            let cb_idx = num_nodes + cb - 1;

            // Standard voltage source stamp
            if np > 0 {
                matrix.push(np - 1, br_idx, 1.0);
                matrix.push(br_idx, np - 1, 1.0);
            }
            if nn > 0 {
                matrix.push(nn - 1, br_idx, -1.0);
                matrix.push(br_idx, nn - 1, -1.0);
            }

            // Control current coefficient: -rm * I_ctrl
            matrix.push(br_idx, cb_idx, -rm);
        }
    }

    /// Stamp all CCVS elements using direct StaticMatrix access.
    #[inline]
    pub fn stamp_all_direct<F>(&self, matrix: &mut StaticMatrix, branch_idx_fn: F)
    where
        F: Fn(usize) -> usize,
    {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br_ordinal = self.branch_indices[i];
            let cb_ordinal = self.ctrl_branch[i];
            let rm = self.transresistances[i];

            if br_ordinal == 0 || cb_ordinal == 0 {
                continue;
            }

            let br_idx = branch_idx_fn(br_ordinal);
            let cb_idx = branch_idx_fn(cb_ordinal);

            if np > 0 {
                matrix.add(np - 1, br_idx - 1, 1.0);
                matrix.add(br_idx - 1, np - 1, 1.0);
            }
            if nn > 0 {
                matrix.add(nn - 1, br_idx - 1, -1.0);
                matrix.add(br_idx - 1, nn - 1, -1.0);
            }

            matrix.add(br_idx - 1, cb_idx - 1, -rm);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vcvs_creation() {
        let mut vcvs = Vcvs::new();
        vcvs.add("E1".to_string(), 2, 0, 1, 0, 1, 2.0);
        assert_eq!(vcvs.len(), 1);
        assert_eq!(vcvs.gains[0], 2.0);
    }

    #[test]
    fn test_vccs_creation() {
        let mut vccs = Vccs::new();
        vccs.add("G1".to_string(), 2, 0, 1, 0, 0.001);
        assert_eq!(vccs.len(), 1);
        assert_eq!(vccs.transconductances[0], 0.001);
    }

    #[test]
    fn test_cccs_creation() {
        let mut cccs = Cccs::new();
        cccs.add("F1".to_string(), 2, 0, 1, 10.0);
        assert_eq!(cccs.len(), 1);
        assert_eq!(cccs.gains[0], 10.0);
    }

    #[test]
    fn test_ccvs_creation() {
        let mut ccvs = Ccvs::new();
        ccvs.add("H1".to_string(), 2, 0, 1, 2, 1000.0);
        assert_eq!(ccvs.len(), 1);
        assert_eq!(ccvs.transresistances[0], 1000.0);
    }
}
