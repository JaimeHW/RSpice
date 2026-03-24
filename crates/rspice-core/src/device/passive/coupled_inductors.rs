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

use crate::analysis::CompanionCoefficients;
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
    current1_prev_prev: Value,
    current2_prev: Value,
    current2_prev_prev: Value,
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
            current1_prev_prev: 0.0,
            current2_prev: 0.0,
            current2_prev_prev: 0.0,
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
        self.current1_prev_prev = i1;
        self.current2_prev = i2;
        self.current2_prev_prev = i2;
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

    /// Stamp DC short-circuit topology for the pair.
    pub fn stamp_dc_short(&self, matrix: &mut impl MatrixStamper, _rhs: &mut [Value]) {
        let branch1 = self.branch1.expect("Branch1 index must be set");
        let branch2 = self.branch2.expect("Branch2 index must be set");

        matrix.stamp(branch1, self.node1_pos, 1.0);
        matrix.stamp(branch1, self.node1_neg, -1.0);
        matrix.stamp(self.node1_pos, branch1, 1.0);
        matrix.stamp(self.node1_neg, branch1, -1.0);

        matrix.stamp(branch2, self.node2_pos, 1.0);
        matrix.stamp(branch2, self.node2_neg, -1.0);
        matrix.stamp(self.node2_pos, branch2, 1.0);
        matrix.stamp(self.node2_neg, branch2, -1.0);

        matrix.stamp_rhs(branch1, 0.0);
        matrix.stamp_rhs(branch2, 0.0);
    }

    /// Calculate equivalent circuit values for the configured integration method.
    fn companion_values(
        &self,
        dt: Value,
        coeff: &CompanionCoefficients,
    ) -> (Value, Value, Value, Value, Value) {
        let r11 = coeff.inductor_req(self.l1, dt);
        let r22 = coeff.inductor_req(self.l2, dt);
        let r12 = coeff.inductor_req(self.m, dt);

        let prev_mix_1 = coeff.coeff_v_n_minus_1
            * (self.l1 * self.current1_prev_prev + self.m * self.current2_prev_prev)
            / dt;
        let prev_mix_2 = coeff.coeff_v_n_minus_1
            * (self.m * self.current1_prev_prev + self.l2 * self.current2_prev_prev)
            / dt;

        let v1_eq =
            r11 * self.current1_prev + r12 * self.current2_prev + self.voltage1_prev + prev_mix_1;
        let v2_eq =
            r12 * self.current1_prev + r22 * self.current2_prev + self.voltage2_prev + prev_mix_2;

        (r11, r22, r12, v1_eq, v2_eq)
    }

    /// Stamp the transient companion for the configured integration method.
    pub fn stamp_transient_companion(
        &self,
        dt: Value,
        coeff: &CompanionCoefficients,
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let branch1 = self.branch1.expect("Branch1 index must be set");
        let branch2 = self.branch2.expect("Branch2 index must be set");

        let (r11, r22, r12, v1_eq, v2_eq) = self.companion_values(dt, coeff);

        matrix.stamp(branch1, self.node1_pos, 1.0);
        matrix.stamp(branch1, self.node1_neg, -1.0);
        matrix.stamp(branch1, branch1, -r11);
        matrix.stamp(branch1, branch2, -r12);

        matrix.stamp(branch2, self.node2_pos, 1.0);
        matrix.stamp(branch2, self.node2_neg, -1.0);
        matrix.stamp(branch2, branch1, -r12);
        matrix.stamp(branch2, branch2, -r22);

        matrix.stamp(self.node1_pos, branch1, 1.0);
        matrix.stamp(self.node1_neg, branch1, -1.0);
        matrix.stamp(self.node2_pos, branch2, 1.0);
        matrix.stamp(self.node2_neg, branch2, -1.0);

        matrix.stamp_rhs(branch1, v1_eq);
        matrix.stamp_rhs(branch2, v2_eq);
    }

    /// Update history from an accepted solution vector.
    pub fn update_state_from_solution(&mut self, solution: &[Value]) {
        let v1 = if self.node1_pos == 0 {
            0.0
        } else {
            solution.get(self.node1_pos - 1).copied().unwrap_or(0.0)
        } - if self.node1_neg == 0 {
            0.0
        } else {
            solution.get(self.node1_neg - 1).copied().unwrap_or(0.0)
        };
        let v2 = if self.node2_pos == 0 {
            0.0
        } else {
            solution.get(self.node2_pos - 1).copied().unwrap_or(0.0)
        } - if self.node2_neg == 0 {
            0.0
        } else {
            solution.get(self.node2_neg - 1).copied().unwrap_or(0.0)
        };

        let branch1 = self.branch1.expect("Branch1 index must be set");
        let branch2 = self.branch2.expect("Branch2 index must be set");
        let i1 = solution.get(branch1 - 1).copied().unwrap_or(0.0);
        let i2 = solution.get(branch2 - 1).copied().unwrap_or(0.0);

        self.current1_prev_prev = self.current1_prev;
        self.current1_prev = i1;
        self.current2_prev_prev = self.current2_prev;
        self.current2_prev = i2;
        self.voltage1_prev = v1;
        self.voltage2_prev = v2;
    }
}

impl DynamicDevice for CoupledInductorPair {
    fn stamp_transient(
        &self,
        _voltages: &[Value],
        dt: Value,
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
    ) {
        self.stamp_transient_companion(dt, &CompanionCoefficients::trapezoidal(), matrix, rhs);
    }

    fn step(&mut self, voltages: &[Value], _dt: Value) {
        self.update_state_from_solution(voltages);
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
    /// Current history from two accepted steps ago
    currents_prev_prev: Vec<Value>,
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
            currents_prev_prev: vec![0.0; n],
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
            self.currents_prev_prev[winding] = current;
        }
    }

    /// Stamp DC short-circuit topology for all windings.
    pub fn stamp_dc_short(&self, matrix: &mut impl MatrixStamper, _rhs: &mut [Value]) {
        for i in 0..self.num_windings {
            let branch = self.branches[i].expect("Branch index must be set");
            let (pos, neg) = self.nodes[i];
            matrix.stamp(branch, pos, 1.0);
            matrix.stamp(branch, neg, -1.0);
            matrix.stamp(pos, branch, 1.0);
            matrix.stamp(neg, branch, -1.0);
            matrix.stamp_rhs(branch, 0.0);
        }
    }

    fn companion_matrix(
        &self,
        dt: Value,
        coeff: &CompanionCoefficients,
    ) -> (Vec<Vec<Value>>, Vec<Value>) {
        let n = self.num_windings;
        let r_matrix: Vec<Vec<Value>> = self
            .inductance_matrix
            .iter()
            .map(|row| row.iter().map(|&l| coeff.inductor_req(l, dt)).collect())
            .collect();

        let mut v_eq = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                v_eq[i] += r_matrix[i][j] * self.currents_prev[j];
                if coeff.needs_two_history {
                    v_eq[i] += coeff.coeff_v_n_minus_1
                        * self.inductance_matrix[i][j]
                        * self.currents_prev_prev[j]
                        / dt;
                }
            }
            v_eq[i] += self.voltages_prev[i];
        }

        (r_matrix, v_eq)
    }

    /// Stamp the transient companion for the configured integration method.
    pub fn stamp_transient_companion(
        &self,
        dt: Value,
        coeff: &CompanionCoefficients,
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let n = self.num_windings;
        let (r_matrix, v_eq) = self.companion_matrix(dt, coeff);

        for i in 0..n {
            let branch_i = self.branches[i].expect("Branch index must be set");
            let (pos_i, neg_i) = self.nodes[i];

            matrix.stamp(branch_i, pos_i, 1.0);
            matrix.stamp(branch_i, neg_i, -1.0);

            for j in 0..n {
                let branch_j = self.branches[j].expect("Branch index must be set");
                matrix.stamp(branch_i, branch_j, -r_matrix[i][j]);
            }

            matrix.stamp(pos_i, branch_i, 1.0);
            matrix.stamp(neg_i, branch_i, -1.0);
            matrix.stamp_rhs(branch_i, v_eq[i]);
        }
    }

    /// Update history from an accepted solution vector.
    pub fn update_state_from_solution(&mut self, solution: &[Value]) {
        for i in 0..self.num_windings {
            let (pos, neg) = self.nodes[i];
            let v_pos = if pos == 0 {
                0.0
            } else {
                solution.get(pos - 1).copied().unwrap_or(0.0)
            };
            let v_neg = if neg == 0 {
                0.0
            } else {
                solution.get(neg - 1).copied().unwrap_or(0.0)
            };
            self.voltages_prev[i] = v_pos - v_neg;

            if let Some(branch) = self.branches[i] {
                self.currents_prev_prev[i] = self.currents_prev[i];
                self.currents_prev[i] = solution.get(branch - 1).copied().unwrap_or(0.0);
            }
        }
    }
}

impl DynamicDevice for MultiWindingTransformer {
    fn stamp_transient(
        &self,
        _voltages: &[Value],
        dt: Value,
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
    ) {
        self.stamp_transient_companion(dt, &CompanionCoefficients::trapezoidal(), matrix, rhs);
    }

    fn step(&mut self, voltages: &[Value], _dt: Value) {
        self.update_state_from_solution(voltages);
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestStamper {
        matrix: Vec<(NodeId, NodeId, Value)>,
        rhs: Vec<(NodeId, Value)>,
    }

    impl MatrixStamper for TestStamper {
        fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
            self.matrix.push((row, col, value));
        }

        fn stamp_rhs(&mut self, index: NodeId, value: Value) {
            self.rhs.push((index, value));
        }
    }

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

    #[test]
    fn test_set_initial_currents_primes_both_history_slots() {
        let mut pair = CoupledInductorPair::new("T1".to_string(), 1, 0, 1e-3, 2, 0, 1e-3, 1.0);
        pair.set_initial_currents(0.12, -0.08);

        assert!((pair.current1_prev - 0.12).abs() < 1e-15);
        assert!((pair.current1_prev_prev - 0.12).abs() < 1e-15);
        assert!((pair.current2_prev + 0.08).abs() < 1e-15);
        assert!((pair.current2_prev_prev + 0.08).abs() < 1e-15);
    }

    #[test]
    fn test_stamp_dc_short_exposes_both_branch_constraints() {
        let mut pair = CoupledInductorPair::new("T1".to_string(), 1, 0, 1e-3, 2, 3, 2e-3, 0.95);
        pair.set_branches(4, 5);

        let mut stamper = TestStamper::default();
        pair.stamp_dc_short(&mut stamper, &mut []);

        assert!(stamper.matrix.contains(&(4, 1, 1.0)));
        assert!(stamper.matrix.contains(&(1, 4, 1.0)));
        assert!(stamper.matrix.contains(&(5, 2, 1.0)));
        assert!(stamper.matrix.contains(&(5, 3, -1.0)));
        assert!(stamper.matrix.contains(&(3, 5, -1.0)));
    }

    #[test]
    fn test_stamp_transient_companion_emits_mutual_branch_terms() {
        let mut pair = CoupledInductorPair::new("T1".to_string(), 1, 0, 2e-3, 2, 0, 8e-3, 0.5);
        pair.set_branches(3, 4);
        pair.current1_prev = 0.01;
        pair.current1_prev_prev = 0.007;
        pair.current2_prev = -0.02;
        pair.current2_prev_prev = -0.015;
        pair.voltage1_prev = 0.3;
        pair.voltage2_prev = -0.1;

        let coeff = CompanionCoefficients::gear2();
        let mut stamper = TestStamper::default();
        pair.stamp_transient_companion(1e-6, &coeff, &mut stamper, &mut []);

        let mutual = pair.m * coeff.coeff_g / 1e-6;
        assert!(stamper.matrix.contains(&(3, 4, -mutual)));
        assert!(stamper.matrix.contains(&(4, 3, -mutual)));
        assert!(
            stamper
                .rhs
                .iter()
                .any(|(idx, value)| *idx == 3 && value.abs() > 0.0)
        );
        assert!(
            stamper
                .rhs
                .iter()
                .any(|(idx, value)| *idx == 4 && value.abs() > 0.0)
        );
    }

    #[test]
    fn test_update_state_from_solution_rotates_history() {
        let mut pair = CoupledInductorPair::new("T1".to_string(), 1, 0, 1e-3, 2, 0, 1e-3, 1.0);
        pair.set_branches(3, 4);
        pair.current1_prev = 0.01;
        pair.current2_prev = -0.02;

        let solution = vec![1.25, -0.75, 0.03, -0.04];
        pair.update_state_from_solution(&solution);

        assert!((pair.current1_prev_prev - 0.01).abs() < 1e-15);
        assert!((pair.current1_prev - 0.03).abs() < 1e-15);
        assert!((pair.current2_prev_prev + 0.02).abs() < 1e-15);
        assert!((pair.current2_prev + 0.04).abs() < 1e-15);
        assert!((pair.voltage1_prev - 1.25).abs() < 1e-15);
        assert!((pair.voltage2_prev + 0.75).abs() < 1e-15);
    }

    #[test]
    fn test_multi_winding_initial_current_primes_history() {
        let mut transformer = MultiWindingTransformer::new(
            "T1".to_string(),
            vec![(1, 0), (2, 0), (3, 0)],
            vec![1e-3, 1e-3, 1e-3],
            vec![
                vec![1.0, 0.2, 0.1],
                vec![0.2, 1.0, 0.15],
                vec![0.1, 0.15, 1.0],
            ],
        );
        transformer.set_initial_current(1, -0.25);

        assert!((transformer.currents_prev[1] + 0.25).abs() < 1e-15);
        assert!((transformer.currents_prev_prev[1] + 0.25).abs() < 1e-15);
    }

    #[test]
    fn test_multi_winding_transient_companion_stamps_dense_branch_matrix() {
        let mut transformer = MultiWindingTransformer::new(
            "T1".to_string(),
            vec![(1, 0), (2, 0), (3, 0)],
            vec![1e-3, 2e-3, 3e-3],
            vec![
                vec![1.0, 0.2, 0.1],
                vec![0.2, 1.0, 0.15],
                vec![0.1, 0.15, 1.0],
            ],
        );
        transformer.set_branches(vec![4, 5, 6]);
        transformer.set_initial_current(0, 0.01);
        transformer.set_initial_current(1, -0.02);
        transformer.set_initial_current(2, 0.03);

        let coeff = CompanionCoefficients::gear2();
        let mut stamper = TestStamper::default();
        transformer.stamp_transient_companion(1e-6, &coeff, &mut stamper, &mut []);

        let m01 = coeff.inductor_req(0.2 * (1e-3_f64 * 2e-3_f64).sqrt(), 1e-6);
        let m12 = coeff.inductor_req(0.15 * (2e-3_f64 * 3e-3_f64).sqrt(), 1e-6);
        let m20 = coeff.inductor_req(0.1 * (3e-3_f64 * 1e-3_f64).sqrt(), 1e-6);
        assert!(stamper.matrix.contains(&(4, 5, -m01)));
        assert!(stamper.matrix.contains(&(5, 6, -m12)));
        assert!(stamper.matrix.contains(&(6, 4, -m20)));
        assert!(
            stamper
                .rhs
                .iter()
                .any(|(idx, value)| *idx == 4 && value.abs() > 0.0)
        );
        assert!(
            stamper
                .rhs
                .iter()
                .any(|(idx, value)| *idx == 5 && value.abs() > 0.0)
        );
        assert!(
            stamper
                .rhs
                .iter()
                .any(|(idx, value)| *idx == 6 && value.abs() > 0.0)
        );
    }

    #[test]
    fn test_multi_winding_update_state_rotates_history() {
        let mut transformer = MultiWindingTransformer::new(
            "T1".to_string(),
            vec![(1, 0), (2, 0)],
            vec![1e-3, 1e-3],
            vec![vec![1.0, 0.5], vec![0.5, 1.0]],
        );
        transformer.set_branches(vec![3, 4]);
        transformer.currents_prev[0] = 0.02;
        transformer.currents_prev[1] = -0.03;

        let solution = vec![1.2, -0.8, 0.07, -0.09];
        transformer.update_state_from_solution(&solution);

        assert!((transformer.currents_prev_prev[0] - 0.02).abs() < 1e-15);
        assert!((transformer.currents_prev_prev[1] + 0.03).abs() < 1e-15);
        assert!((transformer.currents_prev[0] - 0.07).abs() < 1e-15);
        assert!((transformer.currents_prev[1] + 0.09).abs() < 1e-15);
        assert!((transformer.voltages_prev[0] - 1.2).abs() < 1e-15);
        assert!((transformer.voltages_prev[1] + 0.8).abs() < 1e-15);
    }
}
