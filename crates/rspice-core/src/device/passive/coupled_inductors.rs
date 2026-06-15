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

#![allow(clippy::needless_range_loop, clippy::too_many_arguments)]
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

    /// DC stamp for the coupling: intentionally nothing.
    ///
    /// The pair is a *mutual overlay* — the two standalone inductors own
    /// their branch rows (DC shorts included). Mutual inductance contributes
    /// no DC term, and stamping incidence here again would double the rows.
    pub fn stamp_dc_short(&self, _matrix: &mut impl MatrixStamper, _rhs: &mut [Value]) {}

    /// Mutual-coupling history magnitudes — the M-only terms of the dual of
    /// `CompanionCoefficients::inductor_veq` (the standalone inductors carry
    /// the self terms, including the trapezoidal voltage history).
    fn mutual_companion_values(
        &self,
        dt: Value,
        coeff: &CompanionCoefficients,
    ) -> (Value, Value, Value) {
        let r12 = coeff.inductor_req(self.m, dt);
        let h = coeff.coeff_v_n * self.m / dt;
        let mut v1_mut = h * self.current2_prev;
        let mut v2_mut = h * self.current1_prev;
        if coeff.needs_two_history {
            let h2 = coeff.coeff_v_n_minus_1 * self.m / dt;
            v1_mut += h2 * self.current2_prev_prev;
            v2_mut += h2 * self.current1_prev_prev;
        }
        (r12, v1_mut, v2_mut)
    }

    /// Stamp ONLY the mutual terms onto the two existing inductor branch rows
    /// (matrix indices supplied by the caller): `-r12` cross-coupling plus the
    /// mutual history sources. Branch rows demand `-v_eq`; `stamp_rhs`
    /// accumulates on top of the self terms the standalone inductors stamped.
    pub fn stamp_transient_mutual(
        &self,
        branch1: NodeId,
        branch2: NodeId,
        dt: Value,
        coeff: &CompanionCoefficients,
        matrix: &mut impl MatrixStamper,
    ) {
        let (r12, v1_mut, v2_mut) = self.mutual_companion_values(dt, coeff);
        matrix.stamp(branch1, branch2, -r12);
        matrix.stamp(branch2, branch1, -r12);
        matrix.stamp_rhs(branch1, -v1_mut);
        matrix.stamp_rhs(branch2, -v2_mut);
    }

    /// Update history from an accepted solution vector, given the two branch
    /// matrix indices (1-based, num_nodes + ordinal).
    pub fn update_state_with_branches(
        &mut self,
        solution: &[Value],
        branch1: NodeId,
        branch2: NodeId,
    ) {
        self.branch1 = Some(branch1);
        self.branch2 = Some(branch2);
        self.update_state_from_solution(solution);
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
        _rhs: &mut [Value],
    ) {
        let branch1 = self.branch1.expect("Branch1 index must be set");
        let branch2 = self.branch2.expect("Branch2 index must be set");
        self.stamp_transient_mutual(
            branch1,
            branch2,
            dt,
            &CompanionCoefficients::trapezoidal(),
            matrix,
        );
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

        // History magnitudes use coeff_v_n (not the matrix coefficient
        // coeff_g — they differ for Gear2); the voltage history applies for
        // Trapezoidal only. Dual of CompanionCoefficients::inductor_veq.
        let mut v_eq = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                v_eq[i] +=
                    coeff.coeff_v_n * self.inductance_matrix[i][j] * self.currents_prev[j] / dt;
                if coeff.needs_two_history {
                    v_eq[i] += coeff.coeff_v_n_minus_1
                        * self.inductance_matrix[i][j]
                        * self.currents_prev_prev[j]
                        / dt;
                }
            }
            if coeff.needs_current_history {
                v_eq[i] += self.voltages_prev[i];
            }
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
            // Branch rows demand -v_eq; see companion_matrix.
            matrix.stamp_rhs(branch_i, -v_eq[i]);
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
