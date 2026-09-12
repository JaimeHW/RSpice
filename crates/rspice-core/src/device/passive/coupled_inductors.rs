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
//! where k is the signed coupling coefficient (-1 ≤ k ≤ 1). Negative k
//! reverses the mutual voltage polarity relative to the authored terminals.
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
//! where `[L]` is the inductance matrix with Lij = k*sqrt(Li*Lj) for i≠j.

use crate::device::traits::{DynamicDevice, MatrixStamper};
use crate::numerics::integration::CompanionCoefficients;
use crate::{NodeId, Value};

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
    /// Signed coupling coefficient (-1 ≤ k ≤ 1)
    pub coefficient: Value,
}

/// One winding of a coupled-inductor pair: the branch it spans and its
/// self-inductance. The pair constructor took two of these interleaved as six
/// positional arguments, where swapping a node between windings changed the
/// mutual sign without any type error.
#[derive(Clone, Copy)]
pub(crate) struct CoupledWinding {
    pub node_pos: NodeId,
    pub node_neg: NodeId,
    pub inductance: Value,
}

fn mutual_inductance_value(k: Value, l1: Value, l2: Value) -> Value {
    let product = l1 * l2;
    if product.is_normal() || !l1.is_finite() || !l2.is_finite() || l1 <= 0.0 || l2 <= 0.0 {
        return k * product.sqrt();
    }
    if l1 == l2 {
        return k * l1;
    }
    // The self-inductance product may overflow, underflow, or lose precision
    // in the subnormal range while its square root remains representable.
    // Keep the geometric mean and k scaled until the final conversion.
    use rspice_veriloga_runtime::arithmetic::ScaledValue;
    ScaledValue::new(l1.sqrt())
        .multiply(ScaledValue::new(l2.sqrt()))
        .multiply(ScaledValue::new(k))
        .binary64()
}

impl InductorCoupling {
    /// Create a new inductor coupling
    pub fn new(name: String, inductor_names: Vec<String>, coefficient: Value) -> Self {
        Self {
            name,
            inductor_names,
            coefficient,
        }
    }

    /// Calculate mutual inductance between two inductors
    pub fn mutual_inductance(&self, l1: Value, l2: Value) -> Value {
        mutual_inductance_value(self.coefficient, l1, l2)
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
    pub(crate) fn new(
        name: String,
        first: CoupledWinding,
        second: CoupledWinding,
        k: Value,
    ) -> Self {
        let CoupledWinding {
            node_pos: node1_pos,
            node_neg: node1_neg,
            inductance: l1,
        } = first;
        let CoupledWinding {
            node_pos: node2_pos,
            node_neg: node2_neg,
            inductance: l2,
        } = second;
        let m = mutual_inductance_value(k, l1, l2);

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
            k,
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
        self.restore_current_history([i1, i1], [i2, i2]);
    }

    /// Restore the mutual overlay from the already retained physical winding
    /// histories: latest accepted current, then the previous accepted current.
    pub(crate) fn restore_current_history(&mut self, first: [Value; 2], second: [Value; 2]) {
        [self.current1_prev, self.current1_prev_prev] = first;
        [self.current2_prev, self.current2_prev_prev] = second;
    }

    /// Get turns ratio (approximate, for ideal transformer)
    pub fn turns_ratio(&self) -> Value {
        let ratio = self.l1 / self.l2;
        if ratio.is_normal() {
            ratio.sqrt()
        } else {
            self.l1.sqrt() / self.l2.sqrt()
        }
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

    /// Add the mutual-inductance part of a cancellation-resistant Newton
    /// correction residual to two already-stabilized standalone branch rows.
    pub fn add_transient_mutual_correction_rhs(
        &self,
        branch1: NodeId,
        branch2: NodeId,
        correction_rhs: &mut [Value],
        iterate: &[Value],
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
        self.add_transient_mutual_correction_rhs_with_current_map(
            [branch1, branch2],
            correction_rhs,
            iterate,
            dt,
            coeff,
            |_, currents| currents,
        );
    }

    /// Use the same current-coordinate map as the standalone flux residual.
    pub(crate) fn add_transient_mutual_correction_rhs_with_current_map(
        &self,
        branches: [NodeId; 2],
        correction_rhs: &mut [Value],
        iterate: &[Value],
        dt: Value,
        coeff: &CompanionCoefficients,
        current_map: impl Fn(usize, [Value; 3]) -> [Value; 3],
    ) {
        let [branch1, branch2] = branches;
        let (Some(&current1), Some(&current2)) =
            (iterate.get(branch1 - 1), iterate.get(branch2 - 1))
        else {
            return;
        };
        let [current1, previous1, older1] = current_map(
            branch1,
            [current1, self.current1_prev, self.current1_prev_prev],
        );
        let [current2, previous2, older2] = current_map(
            branch2,
            [current2, self.current2_prev, self.current2_prev_prev],
        );
        let derivative1 =
            coeff.inductor_charge_derivative_correction(self.m, dt, current1, previous1, older1);
        let derivative2 =
            coeff.inductor_charge_derivative_correction(self.m, dt, current2, previous2, older2);
        if let Some(row1) = correction_rhs.get_mut(branch1 - 1) {
            *row1 += derivative2;
        }
        if let Some(row2) = correction_rhs.get_mut(branch2 - 1) {
            *row2 += derivative1;
        }
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

    /// Start a new trajectory at a supplied bias, discarding the previous
    /// trajectory's multistep history. Branch indices use the same one-based
    /// MNA convention as accepted-step updates.
    pub(crate) fn reset_state_with_branches(
        &mut self,
        solution: &[Value],
        branch1: NodeId,
        branch2: NodeId,
    ) {
        self.update_state_with_branches(solution, branch1, branch2);
        self.current1_prev_prev = self.current1_prev;
        self.current2_prev_prev = self.current2_prev;
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
    /// Node connections: [(pos, neg), ...] for each winding
    nodes: Vec<(NodeId, NodeId)>,
    /// Self-inductances
    inductances: Vec<Value>,
    /// Branch indices
    branches: Vec<Option<NodeId>>,
    /// Coupling matrix (symmetric, diagonal is 1.0)
    /// `k[i][j]` = coupling between winding `i` and `j`
    coupling_matrix: Vec<Vec<Value>>,
    /// Inductance matrix (`L[i][j] = k[i][j] * sqrt(Li * Lj)`)
    inductance_matrix: Vec<Vec<Value>>,
    /// Previous currents
    currents_prev: Vec<Value>,
    /// Current history from two accepted steps ago
    currents_prev_prev: Vec<Value>,
    /// Previous voltages
    voltages_prev: Vec<Value>,
}

impl MultiWindingTransformer {
    /// Create a transformer with immutable electrical parameters.
    ///
    /// Returns an error for empty or mismatched winding arrays, nonpositive
    /// or nonfinite self-inductances, or a coupling matrix that is not square,
    /// finite, symmetric, unit-diagonal, and bounded by -1 and 1.
    pub fn new(
        name: String,
        nodes: Vec<(NodeId, NodeId)>,
        inductances: Vec<Value>,
        coupling_coefficients: Vec<Vec<Value>>,
    ) -> Result<Self, String> {
        let n = inductances.len();
        let invalid = |reason: &str| format!("transformer '{name}': {reason}");
        if n == 0
            || nodes.len() != n
            || coupling_coefficients.len() != n
            || coupling_coefficients.iter().any(|row| row.len() != n)
        {
            return Err(invalid(
                "expected matching nonempty winding arrays and a square coupling matrix",
            ));
        }
        if inductances.iter().any(|l| !l.is_finite() || *l <= 0.0) {
            return Err(invalid("self-inductances must be finite and positive"));
        }
        for (i, row) in coupling_coefficients.iter().enumerate() {
            for (j, &k) in row.iter().enumerate() {
                if !k.is_finite() || !(-1.0..=1.0).contains(&k) {
                    return Err(invalid(
                        "coupling coefficients must be finite and in [-1, 1]",
                    ));
                }
                if (i == j && k != 1.0) || k != coupling_coefficients[j][i] {
                    return Err(invalid(
                        "coupling matrix must be symmetric with a unit diagonal",
                    ));
                }
            }
        }

        // Build inductance matrix
        let mut l_matrix = vec![vec![0.0; n]; n];
        for i in 0..n {
            l_matrix[i][i] = inductances[i];
            for j in 0..i {
                let mutual = mutual_inductance_value(
                    coupling_coefficients[i][j],
                    inductances[i],
                    inductances[j],
                );
                if !mutual.is_finite() {
                    return Err(invalid("inductance matrix must remain finite"));
                }
                l_matrix[i][j] = mutual;
                l_matrix[j][i] = mutual;
            }
        }

        Ok(Self {
            name,
            nodes,
            inductances,
            branches: vec![None; n],
            coupling_matrix: coupling_coefficients,
            inductance_matrix: l_matrix,
            currents_prev: vec![0.0; n],
            currents_prev_prev: vec![0.0; n],
            voltages_prev: vec![0.0; n],
        })
    }

    /// Number of windings.
    pub(crate) fn num_windings(&self) -> usize {
        self.inductances.len()
    }

    /// Terminal pairs in winding order.
    pub(crate) fn nodes(&self) -> &[(NodeId, NodeId)] {
        &self.nodes
    }

    pub(crate) fn nodes_mut(&mut self) -> &mut [(NodeId, NodeId)] {
        &mut self.nodes
    }

    /// Self-inductances in winding order.
    pub(crate) fn inductances(&self) -> &[Value] {
        &self.inductances
    }

    /// Symmetric signed coupling coefficients, including the unit diagonal.
    pub fn coupling_matrix(&self) -> &[Vec<Value>] {
        &self.coupling_matrix
    }

    /// One-based MNA branch indices, absent until assigned.
    pub(crate) fn branches(&self) -> &[Option<NodeId>] {
        &self.branches
    }

    /// Assign distinct, nonzero MNA branch indices for every winding.
    /// Invalid assignments leave the existing indices unchanged.
    pub fn set_branches(&mut self, branches: Vec<NodeId>) -> Result<(), String> {
        if branches.len() != self.num_windings()
            || branches
                .iter()
                .enumerate()
                .any(|(i, &branch)| branch == 0 || branches[..i].contains(&branch))
        {
            return Err(format!(
                "transformer '{}': expected one distinct nonzero branch index per winding",
                self.name
            ));
        }
        self.branches = branches.into_iter().map(Some).collect();
        Ok(())
    }

    /// Get mutual inductance between two windings
    pub fn mutual_inductance(&self, i: usize, j: usize) -> Value {
        self.inductance_matrix[i][j]
    }

    /// Set both accepted current-history samples for one winding.
    /// An invalid winding or nonfinite current leaves history unchanged.
    pub fn set_initial_current(&mut self, winding: usize, current: Value) -> Result<(), String> {
        if winding >= self.num_windings() || !current.is_finite() {
            return Err(format!(
                "transformer '{}': expected a valid winding and finite initial current",
                self.name
            ));
        }
        self.currents_prev[winding] = current;
        self.currents_prev_prev[winding] = current;
        Ok(())
    }

    /// Stamp DC short-circuit topology for all windings.
    pub fn stamp_dc_short(&self, matrix: &mut impl MatrixStamper, _rhs: &mut [Value]) {
        for i in 0..self.num_windings() {
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
        let n = self.num_windings();
        let r_matrix: Vec<Vec<Value>> = self
            .inductance_matrix
            .iter()
            .map(|row| row.iter().map(|&l| coeff.inductor_req(l, dt)).collect())
            .collect();

        // History magnitudes use coeff_v_n (not the matrix coefficient
        // coeff_g — they differ for Gear2); the voltage history applies for
        // Trapezoidal only. Dual of CompanionCoefficients::inductor_veq.
        let mut v_eq = vec![0.0; n];
        for ((equivalent, inductances), &previous_voltage) in v_eq
            .iter_mut()
            .zip(&self.inductance_matrix)
            .zip(&self.voltages_prev)
            .take(n)
        {
            for ((&inductance, &current), &current_prev_prev) in inductances
                .iter()
                .zip(&self.currents_prev)
                .zip(&self.currents_prev_prev)
                .take(n)
            {
                *equivalent += coeff.coeff_v_n * inductance * current / dt;
                if coeff.needs_two_history {
                    *equivalent += coeff.coeff_v_n_minus_1 * inductance * current_prev_prev / dt;
                }
            }
            if coeff.coeff_i_n != 0.0 {
                *equivalent += coeff.coeff_i_n * previous_voltage;
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
        let n = self.num_windings();
        let (r_matrix, v_eq) = self.companion_matrix(dt, coeff);

        for i in 0..n {
            let branch_i = self.branches[i].expect("Branch index must be set");
            let (pos_i, neg_i) = self.nodes[i];

            matrix.stamp(branch_i, pos_i, 1.0);
            matrix.stamp(branch_i, neg_i, -1.0);

            for (&resistance, branch) in r_matrix[i].iter().zip(&self.branches).take(n) {
                let branch_j = branch.expect("Branch index must be set");
                matrix.stamp(branch_i, branch_j, -resistance);
            }

            matrix.stamp(pos_i, branch_i, 1.0);
            matrix.stamp(neg_i, branch_i, -1.0);
            // Branch rows demand -v_eq; see companion_matrix.
            matrix.stamp_rhs(branch_i, -v_eq[i]);
        }
    }

    /// Replace transformer branch entries in a Newton correction RHS with
    /// the stable inductance-matrix DAE residual.
    pub fn overwrite_transient_correction_rhs(
        &self,
        correction_rhs: &mut [Value],
        iterate: &[Value],
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
        for row in 0..self.num_windings() {
            let Some(branch_row) = self.branches[row] else {
                continue;
            };
            let (pos, neg) = self.nodes[row];
            let voltage = if pos == 0 {
                0.0
            } else {
                iterate.get(pos - 1).copied().unwrap_or(0.0)
            } - if neg == 0 {
                0.0
            } else {
                iterate.get(neg - 1).copied().unwrap_or(0.0)
            };
            let mut correction = -voltage;
            if coeff.coeff_i_n != 0.0 {
                correction -= coeff.coeff_i_n * self.voltages_prev[row];
            }
            for column in 0..self.num_windings() {
                let Some(branch_column) = self.branches[column] else {
                    continue;
                };
                let Some(&current) = iterate.get(branch_column - 1) else {
                    continue;
                };
                correction += coeff.inductor_charge_derivative_correction(
                    self.inductance_matrix[row][column],
                    dt,
                    current,
                    self.currents_prev[column],
                    self.currents_prev_prev[column],
                );
            }
            if let Some(rhs_slot) = correction_rhs.get_mut(branch_row - 1) {
                *rhs_slot = correction;
            }
        }
    }

    /// Update history from an accepted solution vector.
    pub fn update_state_from_solution(&mut self, solution: &[Value]) {
        for i in 0..self.num_windings() {
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
mod correction_tests {
    use super::*;

    #[test]
    fn multi_winding_construction_rejects_malformed_electrical_data() {
        let nodes = vec![(1, 0), (2, 0)];
        let inductances = vec![2.0, 8.0];
        let coupling = vec![vec![1.0, -0.25], vec![-0.25, 1.0]];
        for (nodes, inductances, coupling) in [
            (vec![], vec![], vec![]),
            (vec![(1, 0)], inductances.clone(), coupling.clone()),
            (nodes.clone(), inductances.clone(), vec![]),
            (
                nodes.clone(),
                inductances.clone(),
                vec![vec![1.0], vec![-0.25, 1.0]],
            ),
            (
                nodes.clone(),
                inductances.clone(),
                vec![vec![1.0, -0.25], vec![-0.25, 1.0, 0.0]],
            ),
            (
                nodes.clone(),
                inductances.clone(),
                vec![vec![1.0, -0.25], vec![0.25, 1.0]],
            ),
            (
                nodes.clone(),
                inductances.clone(),
                vec![vec![0.5, -0.25], vec![-0.25, 1.0]],
            ),
        ] {
            let error = MultiWindingTransformer::new("Tbad".into(), nodes, inductances, coupling)
                .expect_err("invalid electrical data must fail at construction");
            assert!(error.contains("Tbad"));
        }
        for invalid in [0.0, -1.0, Value::INFINITY, Value::NEG_INFINITY, Value::NAN] {
            assert!(
                MultiWindingTransformer::new(
                    "Tbad".into(),
                    nodes.clone(),
                    vec![2.0, invalid],
                    coupling.clone(),
                )
                .is_err()
            );
        }
        for invalid in [
            -1.01,
            1.01,
            Value::INFINITY,
            Value::NEG_INFINITY,
            Value::NAN,
        ] {
            assert!(
                MultiWindingTransformer::new(
                    "Tbad".into(),
                    nodes.clone(),
                    inductances.clone(),
                    vec![vec![1.0, invalid], vec![invalid, 1.0]],
                )
                .is_err()
            );
        }
    }

    #[test]
    fn multi_winding_invalid_assignments_preserve_physical_history() {
        let mut transformer = MultiWindingTransformer::new(
            "T1".into(),
            vec![(1, 0), (2, 0)],
            vec![2.0, 8.0],
            vec![vec![1.0, -0.25], vec![-0.25, 1.0]],
        )
        .unwrap();
        transformer.set_branches(vec![3, 4]).unwrap();
        transformer.set_initial_current(0, 2.0).unwrap();
        transformer.set_initial_current(1, -3.0).unwrap();
        for branches in [vec![], vec![3], vec![3, 4, 5], vec![0, 4], vec![3, 3]] {
            assert!(transformer.set_branches(branches).is_err());
            assert_eq!(transformer.branches(), &[Some(3), Some(4)]);
        }
        for (winding, current) in [
            (2, 0.0),
            (usize::MAX, 0.0),
            (0, Value::NAN),
            (1, Value::INFINITY),
        ] {
            assert!(transformer.set_initial_current(winding, current).is_err());
        }
        // L = [[2,-1],[-1,8]]; i(t) = [2+4t,-3+2t], v=L*i'=[6,12].
        // Invalid assignments must not alter the physical ramp at t=0.5.
        let mut correction = [0.0; 4];
        transformer.overwrite_transient_correction_rhs(
            &mut correction,
            &[6.0, 12.0, 4.0, -2.0],
            0.5,
            &CompanionCoefficients::backward_euler(),
        );
        assert_eq!(correction, [0.0; 4]);
        let (resistance, history) =
            transformer.companion_matrix(0.5, &CompanionCoefficients::backward_euler());
        assert_eq!(resistance, vec![vec![4.0, -2.0], vec![-2.0, 16.0]]);
        assert_eq!(history, vec![14.0, -52.0]);
    }

    #[test]
    fn winding_turns_ratio_extremes_preserve_representable_results() {
        for (first, second, expected) in [(1e200, 1e-200, 1e200), (1e-200, 1e200, 1e-200)] {
            let pair = CoupledInductorPair::new(
                "K1".into(),
                CoupledWinding {
                    node_pos: 1,
                    node_neg: 0,
                    inductance: first,
                },
                CoupledWinding {
                    node_pos: 2,
                    node_neg: 0,
                    inductance: second,
                },
                0.5,
            );
            assert!((pair.turns_ratio() / expected - 1.0).abs() < 1e-14);
        }
    }

    #[test]
    fn mutual_inductance_extremes_preserve_range_sign_and_subnormal_rounding() {
        let smallest = Value::from_bits(1);
        for (first, second, coefficient, expected) in [
            (2.0, 8.0, -0.5, -2.0),
            (1e200, 1e200, -0.5, -5e199),
            (1e200, 4e200, -0.5, -1e200),
            (1e-200, 4e-200, -0.5, -1e-200),
            (1e-160, 4e-160, 0.5, 1e-160),
            (1e300, 1e300, 1e-300, 1.0),
            (1e300, 4e300, 0.0, 0.0),
            (smallest, 4.0 * smallest, 0.5, smallest),
            (smallest, 4.0 * smallest, 0.25, 0.0),
            (smallest, 4.0 * smallest, 0.75, 2.0 * smallest),
        ] {
            for (l1, l2) in [(first, second), (second, first)] {
                let coupling =
                    InductorCoupling::new("K1".into(), vec!["L1".into(), "L2".into()], coefficient);
                let pair = CoupledInductorPair::new(
                    "K1".into(),
                    CoupledWinding {
                        node_pos: 1,
                        node_neg: 0,
                        inductance: l1,
                    },
                    CoupledWinding {
                        node_pos: 2,
                        node_neg: 0,
                        inductance: l2,
                    },
                    coefficient,
                );
                let transformer = MultiWindingTransformer::new(
                    "T1".into(),
                    vec![(1, 0), (2, 0)],
                    vec![l1, l2],
                    vec![vec![1.0, coefficient], vec![coefficient, 1.0]],
                )
                .expect("valid transformer");
                for actual in [
                    coupling.mutual_inductance(l1, l2),
                    pair.m,
                    transformer.mutual_inductance(0, 1),
                ] {
                    if expected == 0.0 {
                        assert_eq!(actual, 0.0);
                    } else {
                        assert!(
                            (actual / expected - 1.0).abs() < 1e-14,
                            "k={coefficient}, L1={l1:e}, L2={l2:e}: {actual:e} vs {expected:e}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn signed_coupling_preserves_constructor_polarity() {
        for coefficient in [-1.0, -0.5, 0.0, 0.5, 1.0] {
            let coupling =
                InductorCoupling::new("K1".into(), vec!["L1".into(), "L2".into()], coefficient);
            assert_eq!(coupling.coefficient, coefficient);
            assert_eq!(coupling.mutual_inductance(2.0, 8.0), 4.0 * coefficient);
            let pair = CoupledInductorPair::new(
                "K1".into(),
                CoupledWinding {
                    node_pos: 1,
                    node_neg: 0,
                    inductance: 2.0,
                },
                CoupledWinding {
                    node_pos: 2,
                    node_neg: 0,
                    inductance: 8.0,
                },
                coefficient,
            );
            assert_eq!(pair.k, coefficient);
            assert_eq!(pair.m, 4.0 * coefficient);
            let transformer = MultiWindingTransformer::new(
                "T1".into(),
                vec![(1, 0), (2, 0)],
                vec![2.0, 8.0],
                vec![vec![1.0, coefficient], vec![coefficient, 1.0]],
            )
            .expect("valid transformer");
            assert_eq!(transformer.mutual_inductance(0, 1), 4.0 * coefficient);
            assert_eq!(transformer.mutual_inductance(1, 0), 4.0 * coefficient);
        }
    }

    #[test]
    fn mutual_correction_matches_absolute_companion_polynomial() {
        let mut pair = CoupledInductorPair::new(
            "K1".to_string(),
            CoupledWinding {
                node_pos: 1,
                node_neg: 0,
                inductance: 2.0,
            },
            CoupledWinding {
                node_pos: 2,
                node_neg: 0,
                inductance: 8.0,
            },
            0.5,
        );
        pair.current1_prev = 1.25;
        pair.current1_prev_prev = 1.0;
        pair.current2_prev = -0.75;
        pair.current2_prev_prev = -0.5;
        let iterate = [0.0, 0.0, 1.5, -1.0];
        let dt = 0.125;

        for coeff in [
            CompanionCoefficients::backward_euler(),
            CompanionCoefficients::trapezoidal(),
            CompanionCoefficients::gear2_variable_step(dt, 0.25),
        ] {
            let mut correction_rhs = [0.0; 4];
            pair.add_transient_mutual_correction_rhs(
                3,
                4,
                &mut correction_rhs,
                &iterate,
                dt,
                &coeff,
            );
            let (mutual_resistance, history1, history2) = pair.mutual_companion_values(dt, &coeff);
            let expected1 = mutual_resistance * iterate[3] - history1;
            let expected2 = mutual_resistance * iterate[2] - history2;
            assert!((correction_rhs[2] - expected1).abs() <= 32.0 * Value::EPSILON);
            assert!((correction_rhs[3] - expected2).abs() <= 32.0 * Value::EPSILON);
        }
    }

    #[test]
    fn multi_winding_correction_matches_absolute_companion_polynomial() {
        let mut transformer = MultiWindingTransformer::new(
            "K1".to_string(),
            vec![(1, 0), (2, 0)],
            vec![2.0, 8.0],
            vec![vec![1.0, 0.25], vec![0.25, 1.0]],
        )
        .expect("valid transformer");
        transformer.set_branches(vec![3, 4]).unwrap();
        transformer.currents_prev = vec![1.25, -0.75];
        transformer.currents_prev_prev = vec![1.0, -0.5];
        transformer.voltages_prev = vec![0.125, -0.25];
        let iterate = [0.5, -1.0, 1.5, -1.0];
        let dt = 0.125;

        for coeff in [
            CompanionCoefficients::backward_euler(),
            CompanionCoefficients::trapezoidal(),
            CompanionCoefficients::gear2_variable_step(dt, 0.25),
        ] {
            let mut correction_rhs = [0.0; 4];
            transformer.overwrite_transient_correction_rhs(
                &mut correction_rhs,
                &iterate,
                dt,
                &coeff,
            );
            let (resistance, history) = transformer.companion_matrix(dt, &coeff);
            for row in 0..2 {
                let absolute = -history[row] - iterate[row]
                    + resistance[row][0] * iterate[2]
                    + resistance[row][1] * iterate[3];
                assert!(
                    (correction_rhs[row + 2] - absolute).abs() <= 64.0 * Value::EPSILON,
                    "row={row} coeff={coeff:?} stable={} absolute={absolute}",
                    correction_rhs[row + 2]
                );
            }
        }
    }
}
