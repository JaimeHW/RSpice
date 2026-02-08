//! Coupled Inductors / Transformer Model
//!
//! Implements magnetic coupling between inductors for transformer modeling.
//! Essential for power electronics: flyback, forward, LLC resonant converters.
//!
//! # SPICE Syntax
//! ```text
//! K<name> L1 L2 [L3...] <coupling>
//! K1 L1 L2 0.99        ; Two-winding transformer
//! K2 L1 L2 L3 0.999    ; Three-winding transformer
//! ```
//!
//! # Physics
//! The mutual inductance between two inductors is:
//! ```text
//! M = k * sqrt(L1 * L2)
//! ```
//! where k is the coupling coefficient (0 < k ≤ 1).
//!
//! For perfectly coupled inductors (k=1), the turns ratio is:
//! ```text
//! n = sqrt(L1 / L2)
//! ```
//!
//! # Implementation
//! Uses the flux linkage formulation:
//! ```text
//! λ1 = L1*i1 + M*i2
//! λ2 = M*i1 + L2*i2
//! v1 = dλ1/dt, v2 = dλ2/dt
//! ```
//!
//! For N coupled inductors, this generalizes to matrix form:
//! ```text
//! [λ] = [L] * [i]
//! [v] = d[λ]/dt
//! ```
//! where [L] is the inductance matrix with Lij = k*sqrt(Li*Lj) for i≠j.

use crate::device::traits::{DynamicDevice, MatrixStamper};
use crate::{Value, circuit::NodeId};

//=============================================================================
// Coupling Coefficient
//=============================================================================

/// Inductor coupling specification
#[derive(Debug, Clone)]
pub struct InductorCoupling {
    /// Name of this coupling (K1, K2, etc.)
    pub name: String,
    /// Names of coupled inductors
    pub inductor_names: Vec<String>,
    /// Coupling coefficient (0 < k ≤ 1)
    pub coefficient: Value,
}

impl InductorCoupling {
    /// Create a new inductor coupling
    pub fn new(name: String, inductor_names: Vec<String>, coefficient: Value) -> Self {
        // Clamp coefficient to valid range
        let k = coefficient.abs().min(1.0);
        Self {
            name,
            inductor_names,
            coefficient: k,
        }
    }

    /// Calculate mutual inductance between two inductors
    pub fn mutual_inductance(&self, l1: Value, l2: Value) -> Value {
        self.coefficient * (l1 * l2).sqrt()
    }
}

//=============================================================================
// Coupled Inductor Pair (Basic Implementation)
//=============================================================================

/// Two coupled inductors (basic transformer)
///
/// This is the most common case: a two-winding transformer.
/// Uses MNA stamp for coupled inductors.
#[derive(Debug, Clone)]
pub struct CoupledInductorPair {
    /// Coupling name
    pub name: String,

    // Primary winding
    pub node1_pos: NodeId,
    pub node1_neg: NodeId,
    pub l1: Value,
    pub branch1: Option<NodeId>,

    // Secondary winding
    pub node2_pos: NodeId,
    pub node2_neg: NodeId,
    pub l2: Value,
    pub branch2: Option<NodeId>,

    /// Coupling coefficient
    pub k: Value,

    /// Mutual inductance (M = k * sqrt(L1 * L2))
    pub m: Value,

    // State for transient
    current1_prev: Value,
    current2_prev: Value,
    voltage1_prev: Value,
    voltage2_prev: Value,
}

impl CoupledInductorPair {
    /// Create a new coupled inductor pair
    pub fn new(
        name: String,
        node1_pos: NodeId,
        node1_neg: NodeId,
        l1: Value,
        node2_pos: NodeId,
        node2_neg: NodeId,
        l2: Value,
        k: Value,
    ) -> Self {
        let m = k.abs().min(1.0) * (l1 * l2).sqrt();

        Self {
            name,
            node1_pos,
            node1_neg,
            l1,
            branch1: None,
            node2_pos,
            node2_neg,
            l2,
            branch2: None,
            k: k.abs().min(1.0),
            m,
            current1_prev: 0.0,
            current2_prev: 0.0,
            voltage1_prev: 0.0,
            voltage2_prev: 0.0,
        }
    }

    /// Set branch indices for MNA
    pub fn set_branches(&mut self, branch1: NodeId, branch2: NodeId) {
        self.branch1 = Some(branch1);
        self.branch2 = Some(branch2);
    }

    /// Set initial currents
    pub fn set_initial_currents(&mut self, i1: Value, i2: Value) {
        self.current1_prev = i1;
        self.current2_prev = i2;
    }

    /// Get turns ratio (approximate, for ideal transformer)
    pub fn turns_ratio(&self) -> Value {
        (self.l1 / self.l2).sqrt()
    }

    /// Get leakage inductance of primary (for k < 1)
    pub fn leakage_primary(&self) -> Value {
        self.l1 * (1.0 - self.k * self.k)
    }

    /// Get leakage inductance of secondary (for k < 1)
    pub fn leakage_secondary(&self) -> Value {
        self.l2 * (1.0 - self.k * self.k)
    }

    /// Calculate equivalent circuit values for companion model
    /// Returns (req1, req2, reqm, veq1, veq2) for trapezoidal integration
    fn companion_values(&self, dt: Value) -> (Value, Value, Value, Value, Value) {
        // For coupled inductors with trapezoidal integration:
        // [v1]   [R11 R12] [i1]   [v1_eq]
        // [v2] = [R21 R22] [i2] + [v2_eq]
        //
        // where R11 = 2*L1/dt, R22 = 2*L2/dt, R12 = R21 = 2*M/dt

        let r11 = 2.0 * self.l1 / dt;
        let r22 = 2.0 * self.l2 / dt;
        let r12 = 2.0 * self.m / dt;

        // Equivalent voltage sources
        let v1_eq = r11 * self.current1_prev + r12 * self.current2_prev + self.voltage1_prev;
        let v2_eq = r12 * self.current1_prev + r22 * self.current2_prev + self.voltage2_prev;

        (r11, r22, r12, v1_eq, v2_eq)
    }
}

impl DynamicDevice for CoupledInductorPair {
    fn stamp_transient(
        &self,
        _voltages: &[Value],
        dt: Value,
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let branch1 = self.branch1.expect("Branch1 index must be set");
        let branch2 = self.branch2.expect("Branch2 index must be set");

        let (r11, r22, r12, v1_eq, v2_eq) = self.companion_values(dt);

        // MNA for coupled inductors:
        // Branch 1 equation: v1+ - v1- = R11*i1 + R12*i2 + V1_eq
        // Branch 2 equation: v2+ - v2- = R12*i1 + R22*i2 + V2_eq

        // Row for branch1 current
        matrix.stamp(branch1, self.node1_pos, 1.0);
        matrix.stamp(branch1, self.node1_neg, -1.0);
        matrix.stamp(branch1, branch1, -r11);
        matrix.stamp(branch1, branch2, -r12);

        // Row for branch2 current
        matrix.stamp(branch2, self.node2_pos, 1.0);
        matrix.stamp(branch2, self.node2_neg, -1.0);
        matrix.stamp(branch2, branch1, -r12); // Mutual coupling
        matrix.stamp(branch2, branch2, -r22);

        // KCL: current contributions to nodes
        matrix.stamp(self.node1_pos, branch1, 1.0);
        matrix.stamp(self.node1_neg, branch1, -1.0);
        matrix.stamp(self.node2_pos, branch2, 1.0);
        matrix.stamp(self.node2_neg, branch2, -1.0);

        // RHS for equivalent voltage sources
        matrix.stamp_rhs(branch1, v1_eq);
        matrix.stamp_rhs(branch2, v2_eq);
    }

    fn step(&mut self, voltages: &[Value], _dt: Value) {
        // Get node voltages
        let v1_pos = if self.node1_pos == 0 {
            0.0
        } else {
            voltages[self.node1_pos - 1]
        };
        let v1_neg = if self.node1_neg == 0 {
            0.0
        } else {
            voltages[self.node1_neg - 1]
        };
        let v2_pos = if self.node2_pos == 0 {
            0.0
        } else {
            voltages[self.node2_pos - 1]
        };
        let v2_neg = if self.node2_neg == 0 {
            0.0
        } else {
            voltages[self.node2_neg - 1]
        };

        let v1 = v1_pos - v1_neg;
        let v2 = v2_pos - v2_neg;

        // Get branch currents (stored in extended voltage vector)
        let branch1 = self.branch1.unwrap();
        let branch2 = self.branch2.unwrap();
        let i1 = if branch1 > 0 && branch1 <= voltages.len() {
            voltages[branch1 - 1]
        } else {
            0.0
        };
        let i2 = if branch2 > 0 && branch2 <= voltages.len() {
            voltages[branch2 - 1]
        } else {
            0.0
        };

        // Update for next step using trapezoidal rule
        // For coupled inductors: d/dt [L][i] = [v]
        // This requires solving the coupled system

        // Using the inverse of the inductance matrix:
        // [L]^-1 = 1/(L1*L2 - M^2) * [L2, -M; -M, L1]
        let det = self.l1 * self.l2 - self.m * self.m;

        if det.abs() > 1e-20 {
            let inv_det = 1.0 / det;

            // di/dt = [L]^-1 * [v]
            let _di1_dt = inv_det * (self.l2 * v1 - self.m * v2);
            let _di2_dt = inv_det * (-self.m * v1 + self.l1 * v2);

            // Trapezoidal update (simplified, using average of old and new di/dt)
            // For better accuracy, we'd need to iterate
            self.current1_prev = i1;
            self.current2_prev = i2;
        }

        self.voltage1_prev = v1;
        self.voltage2_prev = v2;
    }
}

//=============================================================================
// Multi-Winding Transformer
//=============================================================================

/// N-winding coupled inductor system
///
/// Generalizes to any number of coupled windings.
/// Uses inductance matrix formulation.
#[derive(Debug, Clone)]
pub struct MultiWindingTransformer {
    /// Name
    pub name: String,
    /// Number of windings
    pub num_windings: usize,
    /// Node connections: [(pos, neg), ...] for each winding
    pub nodes: Vec<(NodeId, NodeId)>,
    /// Self-inductances
    pub inductances: Vec<Value>,
    /// Branch indices
    pub branches: Vec<Option<NodeId>>,
    /// Coupling matrix (symmetric, diagonal is 1.0)
    /// k[i][j] = coupling between winding i and j
    pub coupling_matrix: Vec<Vec<Value>>,
    /// Inductance matrix (L[i][j] = k[i][j] * sqrt(Li * Lj))
    inductance_matrix: Vec<Vec<Value>>,
    /// Previous currents
    currents_prev: Vec<Value>,
    /// Previous voltages
    voltages_prev: Vec<Value>,
}

impl MultiWindingTransformer {
    /// Create a new multi-winding transformer
    pub fn new(
        name: String,
        nodes: Vec<(NodeId, NodeId)>,
        inductances: Vec<Value>,
        coupling_coefficients: Vec<Vec<Value>>,
    ) -> Self {
        let n = inductances.len();
        assert_eq!(nodes.len(), n);
        assert_eq!(coupling_coefficients.len(), n);

        // Build inductance matrix
        let mut l_matrix = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    l_matrix[i][j] = inductances[i];
                } else {
                    let k = coupling_coefficients[i][j].abs().min(1.0);
                    l_matrix[i][j] = k * (inductances[i] * inductances[j]).sqrt();
                }
            }
        }

        Self {
            name,
            num_windings: n,
            nodes,
            inductances,
            branches: vec![None; n],
            coupling_matrix: coupling_coefficients,
            inductance_matrix: l_matrix,
            currents_prev: vec![0.0; n],
            voltages_prev: vec![0.0; n],
        }
    }

    /// Set branch indices
    pub fn set_branches(&mut self, branches: Vec<NodeId>) {
        assert_eq!(branches.len(), self.num_windings);
        self.branches = branches.into_iter().map(Some).collect();
    }

    /// Get mutual inductance between two windings
    pub fn mutual_inductance(&self, i: usize, j: usize) -> Value {
        self.inductance_matrix[i][j]
    }

    /// Set initial current for a winding
    pub fn set_initial_current(&mut self, winding: usize, current: Value) {
        if winding < self.num_windings {
            self.currents_prev[winding] = current;
        }
    }
}

impl DynamicDevice for MultiWindingTransformer {
    fn stamp_transient(
        &self,
        _voltages: &[Value],
        dt: Value,
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let n = self.num_windings;

        // Calculate equivalent resistances: R[i][j] = 2 * L[i][j] / dt
        let r_matrix: Vec<Vec<Value>> = self
            .inductance_matrix
            .iter()
            .map(|row| row.iter().map(|&l| 2.0 * l / dt).collect())
            .collect();

        // Calculate equivalent voltages for each branch
        let mut v_eq = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                v_eq[i] += r_matrix[i][j] * self.currents_prev[j];
            }
            v_eq[i] += self.voltages_prev[i];
        }

        // Stamp each branch
        for i in 0..n {
            let branch_i = self.branches[i].expect("Branch index must be set");
            let (pos_i, neg_i) = self.nodes[i];

            // Row for branch i: v_pos - v_neg = sum_j(R[i][j] * i[j]) + V_eq[i]
            matrix.stamp(branch_i, pos_i, 1.0);
            matrix.stamp(branch_i, neg_i, -1.0);

            for j in 0..n {
                let branch_j = self.branches[j].expect("Branch index must be set");
                matrix.stamp(branch_i, branch_j, -r_matrix[i][j]);
            }

            // KCL stamps
            matrix.stamp(pos_i, branch_i, 1.0);
            matrix.stamp(neg_i, branch_i, -1.0);

            // RHS
            matrix.stamp_rhs(branch_i, v_eq[i]);
        }
    }

    fn step(&mut self, voltages: &[Value], _dt: Value) {
        // Update previous values
        for i in 0..self.num_windings {
            let (pos, neg) = self.nodes[i];
            let v_pos = if pos == 0 { 0.0 } else { voltages[pos - 1] };
            let v_neg = if neg == 0 { 0.0 } else { voltages[neg - 1] };
            self.voltages_prev[i] = v_pos - v_neg;

            if let Some(branch) = self.branches[i] {
                if branch > 0 && branch <= voltages.len() {
                    self.currents_prev[i] = voltages[branch - 1];
                }
            }
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inductor_coupling() {
        let coupling = InductorCoupling::new(
            "K1".to_string(),
            vec!["L1".to_string(), "L2".to_string()],
            0.95,
        );

        let m = coupling.mutual_inductance(100e-6, 100e-6);
        // M = 0.95 * sqrt(100e-6 * 100e-6) = 0.95 * 100e-6 = 95e-6
        assert!((m - 95e-6).abs() < 1e-10);
    }

    #[test]
    fn test_coupled_pair_creation() {
        let pair = CoupledInductorPair::new(
            "T1".to_string(),
            1,
            0,
            100e-6, // Primary: 100µH
            2,
            0,
            25e-6, // Secondary: 25µH (2:1 turns ratio)
            0.99,
        );

        // Check turns ratio
        let n = pair.turns_ratio();
        assert!((n - 2.0).abs() < 0.01);

        // Check mutual inductance
        // M = 0.99 * sqrt(100e-6 * 25e-6) = 0.99 * 50e-6 = 49.5e-6
        assert!((pair.m - 49.5e-6).abs() < 1e-10);
    }

    #[test]
    fn test_leakage_inductance() {
        let pair = CoupledInductorPair::new(
            "T1".to_string(),
            1,
            0,
            100e-6,
            2,
            0,
            100e-6,
            0.9, // 10% leakage
        );

        // Leakage = L * (1 - k^2) = 100e-6 * (1 - 0.81) = 19e-6
        let leakage = pair.leakage_primary();
        assert!((leakage - 19e-6).abs() < 1e-10);
    }

    #[test]
    fn test_perfect_coupling() {
        let pair = CoupledInductorPair::new(
            "T1".to_string(),
            1,
            0,
            100e-6,
            2,
            0,
            100e-6,
            1.0, // Perfect coupling
        );

        // No leakage with perfect coupling
        assert!(pair.leakage_primary() < 1e-15);
        assert!(pair.leakage_secondary() < 1e-15);
    }

    #[test]
    fn test_multi_winding() {
        let nodes = vec![(1, 0), (2, 0), (3, 0)];
        let inductances = vec![100e-6, 100e-6, 25e-6];
        let couplings = vec![
            vec![1.0, 0.99, 0.99],
            vec![0.99, 1.0, 0.99],
            vec![0.99, 0.99, 1.0],
        ];

        let transformer =
            MultiWindingTransformer::new("T1".to_string(), nodes, inductances, couplings);

        assert_eq!(transformer.num_windings, 3);

        // Check self-inductance
        assert_eq!(transformer.mutual_inductance(0, 0), 100e-6);

        // Check mutual inductance
        let m01 = transformer.mutual_inductance(0, 1);
        // M = 0.99 * sqrt(100e-6 * 100e-6) = 99e-6
        assert!((m01 - 99e-6).abs() < 1e-10);
    }

    #[test]
    fn test_coupling_coefficient_clamping() {
        // Coefficient > 1 should be clamped
        let coupling = InductorCoupling::new(
            "K1".to_string(),
            vec!["L1".to_string(), "L2".to_string()],
            1.5, // Invalid, should be clamped to 1.0
        );

        assert_eq!(coupling.coefficient, 1.0);
    }

    #[test]
    fn test_unequal_windings() {
        // 10:1 turns ratio (100:1 inductance ratio)
        let pair = CoupledInductorPair::new(
            "T1".to_string(),
            1,
            0,
            1e-3, // Primary: 1mH
            2,
            0,
            10e-6, // Secondary: 10µH
            0.99,
        );

        // Turns ratio = sqrt(1e-3 / 10e-6) = sqrt(100) = 10
        let n = pair.turns_ratio();
        assert!((n - 10.0).abs() < 0.01);
    }
}
