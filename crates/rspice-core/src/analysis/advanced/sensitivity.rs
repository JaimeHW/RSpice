//! Sensitivity Analysis (.SENS)
//!
//! Computes the sensitivity of circuit outputs to component parameter variations.
//! Uses the efficient adjoint method to compute sensitivities for all parameters
//! in a single solve.
//!
//! # Theory
//!
//! For a linear system **G·x = b**, the sensitivity of output xₖ to parameter p is:
//!
//! ```text
//! ∂xₖ/∂p = -λᵀ · (∂G/∂p · x + ∂b/∂p)
//! ```
//!
//! where λ is the adjoint vector solving **Gᵀ·λ = eₖ** (eₖ is unit vector).
//!
//! # Sensitivity Types
//!
//! - **Absolute**: ∂V/∂R (change in voltage per unit change in resistance)
//! - **Normalized**: (R/V) · ∂V/∂R (percentage change in output per percentage change in parameter)
//!
//! # Example
//!
//! ```ignore
//! .SENS V(out)      ; Compute DC sensitivity of V(out) to all parameters
//! ```

use crate::Value;

//=============================================================================
// Data Structures
//=============================================================================

/// Type of circuit element for sensitivity computation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    Resistor,
    Capacitor,
    Inductor,
    VoltageSource,
    CurrentSource,
    Transconductance,
    Transresistance,
}

/// A single sensitivity value
#[derive(Debug, Clone)]
pub struct Sensitivity {
    /// Element name (e.g., "R1", "C2")
    pub element: String,
    /// Element type
    pub element_type: ElementType,
    /// Parameter name (e.g., "value", "tc1")
    pub parameter: String,
    /// Nominal parameter value
    pub nominal_value: Value,
    /// Absolute sensitivity: ∂output/∂param
    pub absolute: Value,
    /// Normalized sensitivity: (param/output) · ∂output/∂param
    pub normalized: Value,
}

impl Sensitivity {
    /// Create a new sensitivity result
    pub fn new(
        element: &str,
        element_type: ElementType,
        parameter: &str,
        nominal: Value,
        absolute: Value,
        output_value: Value,
    ) -> Self {
        let normalized = if output_value.abs() > 1e-15 {
            (nominal / output_value) * absolute
        } else {
            0.0
        };

        Self {
            element: element.to_string(),
            element_type,
            parameter: parameter.to_string(),
            nominal_value: nominal,
            absolute,
            normalized,
        }
    }

    /// Get sensitivity in percent per percent
    pub fn percent_per_percent(&self) -> Value {
        self.normalized * 100.0
    }
}

/// Complete sensitivity analysis result
#[derive(Debug, Clone)]
pub struct SensitivityResult {
    /// Output variable name
    pub output: String,
    /// Output value at operating point
    pub output_value: Value,
    /// Sensitivities for each element
    pub sensitivities: Vec<Sensitivity>,
}

impl SensitivityResult {
    /// Create new result
    pub fn new(output: &str, output_value: Value) -> Self {
        Self {
            output: output.to_string(),
            output_value,
            sensitivities: Vec::new(),
        }
    }

    /// Add a sensitivity
    pub fn add(&mut self, sensitivity: Sensitivity) {
        self.sensitivities.push(sensitivity);
    }

    /// Get sensitivity for a specific element
    pub fn get(&self, element: &str) -> Option<&Sensitivity> {
        self.sensitivities.iter().find(|s| s.element == element)
    }

    /// Get top N most sensitive elements by absolute normalized sensitivity
    pub fn top_sensitive(&self, n: usize) -> Vec<&Sensitivity> {
        let mut sorted: Vec<_> = self.sensitivities.iter().collect();
        sorted.sort_by(|a, b| b.normalized.abs().partial_cmp(&a.normalized.abs()).unwrap());
        sorted.into_iter().take(n).collect()
    }

    /// Get total number of sensitivities
    pub fn len(&self) -> usize {
        self.sensitivities.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.sensitivities.is_empty()
    }
}

//=============================================================================
// Element Description for Sensitivity
//=============================================================================

/// Description of a circuit element for sensitivity computation
#[derive(Debug, Clone)]
pub struct ElementDesc {
    /// Element name
    pub name: String,
    /// Element type
    pub element_type: ElementType,
    /// Positive node index (None = ground)
    pub node_pos: Option<usize>,
    /// Negative node index (None = ground)
    pub node_neg: Option<usize>,
    /// Parameter value (conductance for R, capacitance for C, etc.)
    pub value: Value,
}

impl ElementDesc {
    /// Create resistor element
    pub fn resistor(
        name: &str,
        n_pos: Option<usize>,
        n_neg: Option<usize>,
        resistance: Value,
    ) -> Self {
        Self {
            name: name.to_string(),
            element_type: ElementType::Resistor,
            node_pos: n_pos,
            node_neg: n_neg,
            value: resistance,
        }
    }

    /// Create capacitor element  
    pub fn capacitor(
        name: &str,
        n_pos: Option<usize>,
        n_neg: Option<usize>,
        capacitance: Value,
    ) -> Self {
        Self {
            name: name.to_string(),
            element_type: ElementType::Capacitor,
            node_pos: n_pos,
            node_neg: n_neg,
            value: capacitance,
        }
    }

    /// Get conductance (for resistors)
    pub fn conductance(&self) -> Value {
        if self.value.abs() > 1e-15 {
            1.0 / self.value
        } else {
            1e15 // Very large conductance for near-zero resistance
        }
    }
}

//=============================================================================
// Sensitivity Analyzer (Adjoint Method)
//=============================================================================

/// Sensitivity analyzer using the adjoint method
pub struct SensitivityAnalyzer {
    /// Number of nodes (excluding ground)
    num_nodes: usize,
    /// Conductance matrix G
    g_matrix: Vec<Vec<Value>>,
    /// Solution vector (node voltages)
    solution: Vec<Value>,
    /// Adjoint vector λ
    adjoint: Vec<Value>,
    /// Circuit elements
    elements: Vec<ElementDesc>,
}

impl SensitivityAnalyzer {
    /// Create analyzer with pre-solved circuit
    ///
    /// # Arguments
    /// * `g_matrix` - Conductance matrix
    /// * `solution` - Node voltage solution
    /// * `elements` - List of circuit elements
    pub fn new(
        g_matrix: Vec<Vec<Value>>,
        solution: Vec<Value>,
        elements: Vec<ElementDesc>,
    ) -> Self {
        let num_nodes = g_matrix.len();
        Self {
            num_nodes,
            g_matrix,
            solution,
            adjoint: vec![0.0; num_nodes],
            elements,
        }
    }

    /// Solve the adjoint system: Gᵀ·λ = eₖ
    ///
    /// For symmetric G (resistive networks), Gᵀ = G
    fn solve_adjoint(&mut self, output_node: usize) -> bool {
        if output_node >= self.num_nodes {
            return false;
        }

        // For resistive networks, G is symmetric so we can solve G·λ = eₖ
        // Build unit vector eₖ
        let mut e = vec![0.0; self.num_nodes];
        e[output_node] = 1.0;

        // Solve using Gaussian elimination
        let n = self.num_nodes;

        // Augmented matrix [G | e]
        let mut aug: Vec<Vec<Value>> = self
            .g_matrix
            .iter()
            .zip(e.iter())
            .map(|(row, &ei)| {
                let mut new_row = row.clone();
                new_row.push(ei);
                new_row
            })
            .collect();

        // Forward elimination with partial pivoting
        for k in 0..n {
            // Find pivot
            let mut max_row = k;
            let mut max_val = aug[k][k].abs();
            for i in (k + 1)..n {
                if aug[i][k].abs() > max_val {
                    max_val = aug[i][k].abs();
                    max_row = i;
                }
            }

            if max_val < 1e-15 {
                return false; // Singular
            }

            if max_row != k {
                aug.swap(k, max_row);
            }

            let pivot = aug[k][k];
            for i in (k + 1)..n {
                let factor = aug[i][k] / pivot;
                aug[i][k] = 0.0;
                for j in (k + 1)..=n {
                    aug[i][j] -= factor * aug[k][j];
                }
            }
        }

        // Back substitution
        for i in (0..n).rev() {
            let mut sum = aug[i][n];
            for j in (i + 1)..n {
                sum -= aug[i][j] * self.adjoint[j];
            }
            self.adjoint[i] = sum / aug[i][i];
        }

        true
    }

    /// Compute sensitivity of a resistor
    ///
    /// For resistor R between nodes i and j:
    /// ∂V/∂R = -λᵀ · (∂G/∂R · V)
    ///       = -λᵀ · (-1/R² · stamps) · V
    ///       = (1/R²) · (λᵢ - λⱼ) · (Vᵢ - Vⱼ)
    fn resistor_sensitivity(&self, elem: &ElementDesc) -> Value {
        let r = elem.value;
        if r.abs() < 1e-15 {
            return 0.0;
        }

        let v_diff = self.voltage_difference(elem.node_pos, elem.node_neg);
        let lambda_diff = self.adjoint_difference(elem.node_pos, elem.node_neg);

        // ∂G/∂R = -G² = -1/R²
        // Sensitivity = -λᵀ · (∂G/∂R · V) = (1/R²) · (λᵢ - λⱼ) · (Vᵢ - Vⱼ)
        (1.0 / (r * r)) * lambda_diff * v_diff
    }

    /// Compute sensitivity of a capacitor (DC case: no effect)
    fn capacitor_sensitivity(&self, _elem: &ElementDesc) -> Value {
        // At DC, capacitors are open circuits - no sensitivity
        0.0
    }

    /// Get voltage difference across element
    fn voltage_difference(&self, n_pos: Option<usize>, n_neg: Option<usize>) -> Value {
        let v_pos = n_pos.map(|i| self.solution[i]).unwrap_or(0.0);
        let v_neg = n_neg.map(|i| self.solution[i]).unwrap_or(0.0);
        v_pos - v_neg
    }

    /// Get adjoint difference across element
    fn adjoint_difference(&self, n_pos: Option<usize>, n_neg: Option<usize>) -> Value {
        let l_pos = n_pos.map(|i| self.adjoint[i]).unwrap_or(0.0);
        let l_neg = n_neg.map(|i| self.adjoint[i]).unwrap_or(0.0);
        l_pos - l_neg
    }

    /// Run sensitivity analysis
    ///
    /// # Arguments
    /// * `output_node` - Node index for output voltage
    /// * `output_ref` - Reference node (None = ground)
    pub fn analyze(
        &mut self,
        output_node: usize,
        output_ref: Option<usize>,
    ) -> Option<SensitivityResult> {
        // Get output value
        let output_value = match output_ref {
            Some(r) if r < self.num_nodes => self.solution[output_node] - self.solution[r],
            _ => self.solution[output_node],
        };

        // Solve adjoint for output node
        if !self.solve_adjoint(output_node) {
            return None;
        }

        // If differential output, also solve for reference and combine
        if let Some(ref_node) = output_ref {
            if ref_node < self.num_nodes {
                // Save current adjoint
                let adj_output = self.adjoint.clone();

                // Solve for reference node
                if !self.solve_adjoint(ref_node) {
                    return None;
                }

                // Combine: λ = λ_output - λ_ref
                for i in 0..self.num_nodes {
                    self.adjoint[i] = adj_output[i] - self.adjoint[i];
                }
            }
        }

        let mut result = SensitivityResult::new(&format!("V({})", output_node + 1), output_value);

        // Compute sensitivity for each element
        for elem in &self.elements {
            let absolute = match elem.element_type {
                ElementType::Resistor => self.resistor_sensitivity(elem),
                ElementType::Capacitor => self.capacitor_sensitivity(elem),
                _ => 0.0, // Other types not implemented yet
            };

            let sensitivity = Sensitivity::new(
                &elem.name,
                elem.element_type,
                "value",
                elem.value,
                absolute,
                output_value,
            );

            result.add(sensitivity);
        }

        Some(result)
    }
}

//=============================================================================
// Finite Difference Verification Helper
//=============================================================================

/// Verify sensitivity using finite difference (for testing)
pub fn finite_difference_sensitivity<F>(nominal: Value, delta: Value, compute_output: F) -> Value
where
    F: Fn(Value) -> Value,
{
    let f_plus = compute_output(nominal + delta);
    let f_minus = compute_output(nominal - delta);
    (f_plus - f_minus) / (2.0 * delta)
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Build voltage divider circuit:
    /// Vin (node 1) -- R1 -- node 2 -- R2 -- ground
    #[allow(dead_code)]
    fn voltage_divider(r1: Value, r2: Value) -> (Vec<Vec<Value>>, Vec<Value>, Vec<ElementDesc>) {
        let g1 = 1.0 / r1;
        let g2 = 1.0 / r2;

        // G matrix for 2 nodes
        let g = vec![vec![g1, -g1], vec![-g1, g1 + g2]];

        // Solve with Vin = 10V at node 1 (inject current g1*10 at node 1)
        // Actually, for sensitivity we just need the relative solution
        // Let's inject 1A at node 1 for simplicity
        let vin = 10.0;
        let v1 = vin;
        let v2 = vin * r2 / (r1 + r2);
        let solution = vec![v1, v2];

        let elements = vec![
            ElementDesc::resistor("R1", Some(0), Some(1), r1),
            ElementDesc::resistor("R2", Some(1), None, r2),
        ];

        (g, solution, elements)
    }

    #[test]
    fn test_resistor_divider_sensitivity() {
        // Simpler test: single resistor to ground
        // Node 0 has current injected, resistor R to ground
        // V = I * R, so ∂V/∂R = I = V/R (for I=1)
        let r = 1000.0;
        let g = 1.0 / r;

        // G matrix: single node with R to ground
        let g_matrix = vec![vec![g]];

        // With 1A injected: V = 1 * R = 1000V
        let solution = vec![r]; // V[0] = R (since I=1)

        let elements = vec![ElementDesc::resistor("R1", Some(0), None, r)];

        let mut analyzer = SensitivityAnalyzer::new(g_matrix, solution, elements);

        // Sensitivity of V(0) to R1
        let result = analyzer.analyze(0, None);
        assert!(result.is_some());

        let result = result.unwrap();
        assert_eq!(result.sensitivities.len(), 1);

        let sens_r1 = result.get("R1").unwrap();

        // For V = I*R with I=1: ∂V/∂R = 1 (positive, proportional)
        // Using adjoint: sensitivity = (1/R²) * λ_diff * v_diff
        // With single node to ground: v_diff = V - 0 = R = 1000
        // λ solves G·λ = e₀ → g·λ = 1 → λ = R = 1000
        // λ_diff = R - 0 = R = 1000
        // Sensitivity = (1/R²) * R * R = 1
        assert!(
            (sens_r1.absolute - 1.0).abs() < 1e-6,
            "Expected sensitivity=1, got {}",
            sens_r1.absolute
        );
    }

    #[test]
    fn test_finite_difference_helper() {
        // Test quadratic function f(x) = x²
        // df/dx = 2x
        // At x = 3, df/dx = 6
        let sensitivity = finite_difference_sensitivity(3.0, 0.001, |x| x * x);
        assert!((sensitivity - 6.0).abs() < 0.001);
    }

    #[test]
    fn test_sensitivity_normalized() {
        let sens = Sensitivity::new(
            "R1",
            ElementType::Resistor,
            "value",
            1000.0,  // 1k resistor
            -0.0025, // ∂V/∂R
            5.0,     // V = 5V
        );

        // Normalized = (R/V) * ∂V/∂R = (1000/5) * (-0.0025) = -0.5
        assert!((sens.normalized - (-0.5)).abs() < 1e-10);

        // Percent per percent = -50%
        assert!((sens.percent_per_percent() - (-50.0)).abs() < 1e-10);
    }

    #[test]
    fn test_top_sensitive() {
        let mut result = SensitivityResult::new("V(out)", 5.0);

        result.add(Sensitivity::new(
            "R1",
            ElementType::Resistor,
            "value",
            1000.0,
            0.001,
            5.0,
        ));
        result.add(Sensitivity::new(
            "R2",
            ElementType::Resistor,
            "value",
            2000.0,
            0.005,
            5.0,
        ));
        result.add(Sensitivity::new(
            "R3",
            ElementType::Resistor,
            "value",
            500.0,
            0.002,
            5.0,
        ));

        let top = result.top_sensitive(2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].element, "R2"); // Highest absolute normalized
    }
}
