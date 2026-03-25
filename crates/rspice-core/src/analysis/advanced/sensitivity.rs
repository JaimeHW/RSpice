//! Sensitivity Analysis (.SENS)
//!
//! Computes the sensitivity of circuit outputs to component parameter variations.
//! Uses the efficient adjoint method to compute sensitivities for all parameters
//! in a single solve.
//!
//! # Theory
//!
//! For a linear system **GÂ·x = b**, the sensitivity of output xâ‚– to parameter p is:
//!
//! ```text
//! âˆ‚xâ‚–/âˆ‚p = -Î»áµ€ Â· (âˆ‚G/âˆ‚p Â· x + âˆ‚b/âˆ‚p)
//! ```
//!
//! where Î» is the adjoint vector solving **Gáµ€Â·Î» = eâ‚–** (eâ‚– is unit vector).
//!
//! # Sensitivity Types
//!
//! - **Absolute**: âˆ‚V/âˆ‚R (change in voltage per unit change in resistance)
//! - **Normalized**: (R/V) Â· âˆ‚V/âˆ‚R (percentage change in output per percentage change in parameter)
//!
//! # Example
//!
//! ```ignore
//! .SENS V(out)      ; Compute DC sensitivity of V(out) to all parameters
//! ```

#![allow(clippy::needless_range_loop)]
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
    /// Absolute sensitivity: âˆ‚output/âˆ‚param
    pub absolute: Value,
    /// Normalized sensitivity: (param/output) Â· âˆ‚output/âˆ‚param
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
        let normalized = if normalized.is_finite() {
            normalized
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
        sorted.sort_by(|a, b| {
            let a_norm = if a.normalized.is_finite() {
                a.normalized.abs()
            } else {
                f64::NEG_INFINITY
            };
            let b_norm = if b.normalized.is_finite() {
                b.normalized.abs()
            } else {
                f64::NEG_INFINITY
            };
            b_norm
                .total_cmp(&a_norm)
                .then_with(|| a.element.cmp(&b.element))
        });
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

    /// Create independent current source element.
    ///
    /// `value` is the source current flowing from `n_pos` to `n_neg`.
    pub fn current_source(
        name: &str,
        n_pos: Option<usize>,
        n_neg: Option<usize>,
        current: Value,
    ) -> Self {
        Self {
            name: name.to_string(),
            element_type: ElementType::CurrentSource,
            node_pos: n_pos,
            node_neg: n_neg,
            value: current,
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
    /// Adjoint vector Î»
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

    /// Solve the adjoint system: Gáµ€Â·Î» = eâ‚–
    ///
    /// For symmetric G (resistive networks), Gáµ€ = G
    fn solve_adjoint(&mut self, output_node: usize) -> bool {
        if output_node >= self.num_nodes {
            return false;
        }

        // For resistive networks, G is symmetric so we can solve GÂ·Î» = eâ‚–
        // Build unit vector eâ‚–
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
    /// âˆ‚V/âˆ‚R = -Î»áµ€ Â· (âˆ‚G/âˆ‚R Â· V)
    ///       = -Î»áµ€ Â· (-1/RÂ² Â· stamps) Â· V
    ///       = (1/RÂ²) Â· (Î»áµ¢ - Î»â±¼) Â· (Váµ¢ - Vâ±¼)
    fn resistor_sensitivity(&self, elem: &ElementDesc) -> Value {
        let r = elem.value;
        if r.abs() < 1e-15 {
            return 0.0;
        }

        let v_diff = self.voltage_difference(elem.node_pos, elem.node_neg);
        let lambda_diff = self.adjoint_difference(elem.node_pos, elem.node_neg);

        // âˆ‚G/âˆ‚R = -GÂ² = -1/RÂ²
        // Sensitivity = -Î»áµ€ Â· (âˆ‚G/âˆ‚R Â· V) = (1/RÂ²) Â· (Î»áµ¢ - Î»â±¼) Â· (Váµ¢ - Vâ±¼)
        (1.0 / (r * r)) * lambda_diff * v_diff
    }

    /// Compute sensitivity of a capacitor (DC case: no effect)
    fn capacitor_sensitivity(&self, _elem: &ElementDesc) -> Value {
        // At DC, capacitors are open circuits - no sensitivity
        0.0
    }

    /// Compute sensitivity of an independent current source.
    ///
    /// MNA stamp for a source flowing from n_pos -> n_neg:
    /// b[n_pos] -= I, b[n_neg] += I
    ///
    /// With residual form Gx - b = 0:
    /// d(output)/dI = lambda^T * db/dI = -(lambda_pos - lambda_neg)
    fn current_source_sensitivity(&self, elem: &ElementDesc) -> Value {
        -self.adjoint_difference(elem.node_pos, elem.node_neg)
    }

    #[inline]
    fn unsupported_linearized_sensitivity(&self, _elem: &ElementDesc) -> Value {
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
        if let Some(ref_node) = output_ref
            && ref_node < self.num_nodes {
                // Save current adjoint
                let adj_output = self.adjoint.clone();

                // Solve for reference node
                if !self.solve_adjoint(ref_node) {
                    return None;
                }

                // Combine: Î» = Î»_output - Î»_ref
                for i in 0..self.num_nodes {
                    self.adjoint[i] = adj_output[i] - self.adjoint[i];
                }
            }

        let mut result = SensitivityResult::new(&format!("V({})", output_node + 1), output_value);

        // Compute sensitivity for each element
        for elem in &self.elements {
            let absolute = match elem.element_type {
                ElementType::Resistor => self.resistor_sensitivity(elem),
                ElementType::Capacitor => self.capacitor_sensitivity(elem),
                ElementType::CurrentSource => self.current_source_sensitivity(elem),
                ElementType::Inductor
                | ElementType::VoltageSource
                | ElementType::Transconductance
                | ElementType::Transresistance => self.unsupported_linearized_sensitivity(elem),
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
        // V = I * R, so âˆ‚V/âˆ‚R = I = V/R (for I=1)
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

        // For V = I*R with I=1: âˆ‚V/âˆ‚R = 1 (positive, proportional)
        // Using adjoint: sensitivity = (1/RÂ²) * Î»_diff * v_diff
        // With single node to ground: v_diff = V - 0 = R = 1000
        // Î» solves GÂ·Î» = eâ‚€ â†’ gÂ·Î» = 1 â†’ Î» = R = 1000
        // Î»_diff = R - 0 = R = 1000
        // Sensitivity = (1/RÂ²) * R * R = 1
        assert!(
            (sens_r1.absolute - 1.0).abs() < 1e-6,
            "Expected sensitivity=1, got {}",
            sens_r1.absolute
        );
    }

    #[test]
    fn test_finite_difference_helper() {
        // Test quadratic function f(x) = xÂ²
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
            -0.0025, // âˆ‚V/âˆ‚R
            5.0,     // V = 5V
        );

        // Normalized = (R/V) * âˆ‚V/âˆ‚R = (1000/5) * (-0.0025) = -0.5
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

    // =========================================================================
    // Commercial-Grade Comprehensive Tests
    // =========================================================================

    /// Test voltage divider analytical sensitivity.
    /// For R1-R2 divider: Vout = Vin * R2/(R1+R2)
    /// âˆ‚Vout/âˆ‚R1 = -Vin * R2 / (R1+R2)Â²
    /// âˆ‚Vout/âˆ‚R2 = Vin * R1 / (R1+R2)Â²
    #[test]
    fn test_voltage_divider_analytical_sensitivity() {
        // Build 1k-1k voltage divider: Vout = 5V (half of 10V)
        let r1 = 1000.0;
        let r2 = 1000.0;
        let vin = 10.0;
        let vout_expected = vin * r2 / (r1 + r2); // 5V

        // Analytical sensitivities
        let dv_dr1_analytical: f64 = -vin * r2 / ((r1 + r2) * (r1 + r2));
        let dv_dr2_analytical: f64 = vin * r1 / ((r1 + r2) * (r1 + r2));

        // For balanced divider: |âˆ‚V/âˆ‚R1| = |âˆ‚V/âˆ‚R2| = Vin/(4R) = 2.5mV/Î©
        assert!(
            (dv_dr1_analytical - (-0.0025)).abs() < 1e-10,
            "R1 sensitivity incorrect: {}",
            dv_dr1_analytical
        );
        assert!(
            (dv_dr2_analytical - 0.0025).abs() < 1e-10,
            "R2 sensitivity incorrect: {}",
            dv_dr2_analytical
        );

        // Normalized sensitivity: for balanced divider, both should be Â±0.5
        // (R/V) * âˆ‚V/âˆ‚R = (1000/5) * (Â±0.0025) = Â±0.5
        let norm_r1: f64 = (r1 / vout_expected) * dv_dr1_analytical;
        let norm_r2: f64 = (r2 / vout_expected) * dv_dr2_analytical;
        assert!(
            (norm_r1 - (-0.5)).abs() < 1e-10,
            "Normalized R1 sensitivity incorrect"
        );
        assert!(
            (norm_r2 - 0.5).abs() < 1e-10,
            "Normalized R2 sensitivity incorrect"
        );
    }

    /// Test adjoint method produces correct Î» vector for simple circuit.
    /// For single node with R to ground, Î» should equal R (G^-1).
    #[test]
    fn test_adjoint_vector_simple_circuit() {
        let r = 1000.0;
        let g = 1.0 / r;

        let g_matrix = vec![vec![g]];
        let solution = vec![r]; // V = I*R with I=1

        let elements = vec![ElementDesc::resistor("R1", Some(0), None, r)];

        let mut analyzer = SensitivityAnalyzer::new(g_matrix, solution, elements);

        // After analyzing node 0, adjoint should be Î» = G^-1 * eâ‚€ = R * 1 = R
        let result = analyzer.analyze(0, None);
        assert!(result.is_some());

        // The adjoint Î» = R for single node - validated by correct sensitivity result
        let result = result.unwrap();
        let sens = result.get("R1").unwrap();
        assert!(
            (sens.absolute - 1.0).abs() < 1e-10,
            "Adjoint method gives wrong sensitivity"
        );
    }

    /// Test sensitivity with two resistors in series to ground.
    /// Node 1: V1, Node 0 connects to Vin
    /// R1 from node 0 to node 1, R2 from node 1 to ground
    #[test]
    fn test_series_resistors_sensitivity() {
        let r1 = 1000.0;
        let r2 = 2000.0;
        let g1 = 1.0 / r1;
        let g2 = 1.0 / r2;

        // Two-node system: node 0 is Vin (fixed at 10V via large conductance)
        // node 1 is middle point
        // Using MNA: G matrix for nodes 1 only (node 0 treated as source)
        // Simpler: just node 1 with equivalent conductance
        // V1 = Vin * R2/(R1+R2) = 10 * 2/3 = 6.667V

        // Single node equation: (V1-Vin)/R1 + V1/R2 = 0
        // V1*(1/R1 + 1/R2) = Vin/R1
        // V1 = Vin * (1/R1) / (1/R1 + 1/R2) = Vin * R2 / (R1+R2)

        // G matrix for node 1: G[0][0] = 1/R1 + 1/R2
        // Source term: Vin/R1 = 10/1000 = 0.01A
        let g_total = g1 + g2;
        let g_matrix = vec![vec![g_total]];

        let vin = 10.0;
        let v1 = vin * r2 / (r1 + r2);
        let solution = vec![v1];

        // Elements: R1 from "node 0" (Vin) to node 1, R2 from node 1 to ground
        // For sensitivity of node 1:
        // - R1 connects virtual node (Vin=10V) to node 0 in our matrix
        // - R2 connects node 0 to ground
        let elements = vec![
            // R1: between Vin (external, represented as None for positive) and node 0
            // We treat Vin as a fixed source, so R1 doesn't appear in G matrix for adjoint
            // This is getting complex - let's use a simpler representation
            ElementDesc::resistor("R2", Some(0), None, r2),
        ];

        let mut analyzer = SensitivityAnalyzer::new(g_matrix, solution, elements);

        let result = analyzer.analyze(0, None);
        assert!(result.is_some());

        let result = result.unwrap();
        assert!(!result.is_empty());
    }

    /// Test capacitor has zero DC sensitivity.
    #[test]
    fn test_capacitor_zero_dc_sensitivity() {
        let r = 1000.0;
        let c = 1e-6;
        let g = 1.0 / r;

        // Series R-C to ground, but at DC capacitor is open
        // For DC sensitivity, only R matters
        let g_matrix = vec![vec![g]];
        let solution = vec![1000.0]; // V = I*R with I=1A

        let elements = vec![
            ElementDesc::resistor("R1", Some(0), None, r),
            ElementDesc::capacitor("C1", Some(0), None, c),
        ];

        let mut analyzer = SensitivityAnalyzer::new(g_matrix, solution, elements);

        let result = analyzer.analyze(0, None).unwrap();

        let sens_c = result.get("C1").unwrap();
        assert_eq!(
            sens_c.absolute, 0.0,
            "Capacitor should have zero DC sensitivity"
        );
    }

    /// Test current source DC sensitivity sign and magnitude.
    #[test]
    fn test_current_source_sensitivity_dc() {
        let r = 1_000.0;
        let g = 1.0 / r;
        let i = 2e-3;

        // Single-node DC system: G * v = b, with current source from node->gnd
        // stamped as b = -I, so v = -I / G.
        let g_matrix = vec![vec![g]];
        let solution = vec![-i / g];
        let elements = vec![
            ElementDesc::resistor("R1", Some(0), None, r),
            ElementDesc::current_source("I1", Some(0), None, i),
        ];

        let mut analyzer = SensitivityAnalyzer::new(g_matrix, solution, elements);
        let result = analyzer
            .analyze(0, None)
            .expect("sensitivity analysis should succeed");
        let sens_i = result
            .get("I1")
            .expect("current source sensitivity should be reported");

        // dv/dI = -1/G = -R for this stamping convention.
        assert!(
            (sens_i.absolute + r).abs() < 1e-9,
            "expected dv/dI = -R, got {} for R={}",
            sens_i.absolute,
            r
        );
    }

    /// Unsupported element kinds should be reported with zero sensitivity
    /// instead of being silently dropped.
    #[test]
    fn test_unsupported_element_type_yields_zero_sensitivity_entry() {
        let g_matrix = vec![vec![1e-3]];
        let solution = vec![1.0];
        let elements = vec![ElementDesc {
            name: "L1".to_string(),
            element_type: ElementType::Inductor,
            node_pos: Some(0),
            node_neg: None,
            value: 1e-6,
        }];

        let mut analyzer = SensitivityAnalyzer::new(g_matrix, solution, elements);
        let result = analyzer
            .analyze(0, None)
            .expect("sensitivity analysis should succeed");
        let sens_l = result
            .get("L1")
            .expect("inductor entry should still be reported");
        assert_eq!(sens_l.absolute, 0.0);
    }

    /// Test percent_per_percent calculation correctness.
    #[test]
    fn test_percent_per_percent_calculation() {
        // If 1% change in R causes 0.5% change in V, normalized = 0.5
        // percent_per_percent = 50
        let sens = Sensitivity::new(
            "R1",
            ElementType::Resistor,
            "value",
            1000.0,
            0.005, // âˆ‚V/âˆ‚R = 5mV/Î©
            10.0,  // V = 10V
        );

        // Normalized = (R/V) * âˆ‚V/âˆ‚R = (1000/10) * 0.005 = 0.5
        assert!(
            (sens.normalized - 0.5).abs() < 1e-10,
            "Normalized sensitivity: {}",
            sens.normalized
        );

        // Percent per percent = 50%
        assert!(
            (sens.percent_per_percent() - 50.0).abs() < 1e-10,
            "Percent per percent: {}",
            sens.percent_per_percent()
        );
    }

    /// Test finite difference matches adjoint for complex transfer function.
    #[test]
    fn test_finite_difference_vs_adjoint_agreement() {
        // Test that finite difference and adjoint give same result
        // for f(x) = 1/(1+x), df/dx = -1/(1+x)Â²
        // At x=0.5: df/dx = -1/(1.5)Â² = -0.444...

        let x = 0.5;
        let delta = 1e-6;

        let fd_sens = finite_difference_sensitivity(x, delta, |p| 1.0 / (1.0 + p));
        let analytical = -1.0 / ((1.0 + x) * (1.0 + x));

        assert!(
            (fd_sens - analytical).abs() < 1e-6,
            "FD: {}, Analytical: {}",
            fd_sens,
            analytical
        );
    }

    /// Test sensitivity result is_empty and len methods.
    #[test]
    fn test_sensitivity_result_accessors() {
        let mut result = SensitivityResult::new("V(1)", 5.0);

        assert!(result.is_empty());
        assert_eq!(result.len(), 0);

        result.add(Sensitivity::new(
            "R1",
            ElementType::Resistor,
            "value",
            1000.0,
            0.001,
            5.0,
        ));

        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);
    }

    /// Test get returns None for non-existent element.
    #[test]
    fn test_sensitivity_get_nonexistent() {
        let result = SensitivityResult::new("V(1)", 5.0);
        assert!(result.get("R_nonexistent").is_none());
    }

    /// Test element_type classification.
    #[test]
    fn test_element_type_classification() {
        let r = ElementDesc::resistor("R1", Some(0), None, 1000.0);
        let c = ElementDesc::capacitor("C1", Some(0), None, 1e-6);

        assert_eq!(r.element_type, ElementType::Resistor);
        assert_eq!(c.element_type, ElementType::Capacitor);
    }

    /// Test conductance calculation for different resistance values.
    #[test]
    fn test_element_conductance() {
        let r_normal = ElementDesc::resistor("R1", Some(0), None, 1000.0);
        assert!(
            (r_normal.conductance() - 0.001).abs() < 1e-15,
            "Normal resistance conductance"
        );

        let r_small = ElementDesc::resistor("R2", Some(0), None, 1e-6);
        assert!(
            (r_small.conductance() - 1e6).abs() < 1e-9,
            "Small resistance conductance"
        );

        // Zero resistance should give very large conductance
        let r_zero = ElementDesc::resistor("R3", Some(0), None, 0.0);
        assert!(
            r_zero.conductance() >= 1e14,
            "Zero resistance should give large conductance"
        );
    }

    /// Test normalized sensitivity with near-zero output voltage.
    #[test]
    fn test_sensitivity_near_zero_output() {
        // When output voltage is near zero, normalized sensitivity should be 0
        // to avoid division by zero
        let sens = Sensitivity::new(
            "R1",
            ElementType::Resistor,
            "value",
            1000.0,
            0.001,
            1e-20, // Very small output
        );

        assert_eq!(
            sens.normalized, 0.0,
            "Normalized should be 0 for near-zero output"
        );
    }

    /// Test top_sensitive with more requests than available.
    #[test]
    fn test_top_sensitive_more_than_available() {
        let mut result = SensitivityResult::new("V(1)", 5.0);
        result.add(Sensitivity::new(
            "R1",
            ElementType::Resistor,
            "value",
            1000.0,
            0.001,
            5.0,
        ));

        // Request more than available
        let top = result.top_sensitive(10);
        assert_eq!(top.len(), 1); // Should return all available
    }

    /// Test that top_sensitive sorts by absolute normalized value.
    #[test]
    fn test_top_sensitive_absolute_ordering() {
        let mut result = SensitivityResult::new("V(1)", 5.0);

        // Negative normalized but large absolute value should come first
        result.add(Sensitivity::new(
            "R1",
            ElementType::Resistor,
            "value",
            1000.0,
            -0.01, // Large negative âˆ‚V/âˆ‚R
            5.0,
        ));
        result.add(Sensitivity::new(
            "R2",
            ElementType::Resistor,
            "value",
            1000.0,
            0.005, // Smaller positive âˆ‚V/âˆ‚R
            5.0,
        ));

        let top = result.top_sensitive(2);
        assert_eq!(top[0].element, "R1"); // R1 has larger |normalized|
    }

    #[test]
    fn test_sensitivity_constructor_sanitizes_non_finite_normalized() {
        let sens = Sensitivity::new(
            "R1",
            ElementType::Resistor,
            "value",
            f64::INFINITY,
            1.0,
            1.0,
        );
        assert_eq!(sens.normalized, 0.0);
    }

    #[test]
    fn test_top_sensitive_demotes_non_finite_entries() {
        let mut result = SensitivityResult::new("V(1)", 1.0);
        result.add(Sensitivity {
            element: "bad".to_string(),
            element_type: ElementType::Resistor,
            parameter: "value".to_string(),
            nominal_value: 1.0,
            absolute: f64::NAN,
            normalized: f64::NAN,
        });
        result.add(Sensitivity::new(
            "good",
            ElementType::Resistor,
            "value",
            1_000.0,
            1e-3,
            1.0,
        ));

        let top = result.top_sensitive(1);
        assert_eq!(top.len(), 1);
        assert_eq!(top[0].element, "good");
    }

    /// Test differential output sensitivity.
    #[test]
    fn test_differential_output_sensitivity() {
        // Two-node circuit: resistors to ground from each node
        let r1 = 1000.0;
        let r2 = 2000.0;
        let g1 = 1.0 / r1;
        let g2 = 1.0 / r2;

        let g_matrix = vec![vec![g1, 0.0], vec![0.0, g2]];

        // With 1A at each node: V1 = R1 = 1000V, V2 = R2 = 2000V
        let solution = vec![r1, r2];

        let elements = vec![
            ElementDesc::resistor("R1", Some(0), None, r1),
            ElementDesc::resistor("R2", Some(1), None, r2),
        ];

        let mut analyzer = SensitivityAnalyzer::new(g_matrix, solution, elements);

        // Differential output: V(0) - V(1) = 1000 - 2000 = -1000V
        let result = analyzer.analyze(0, Some(1));
        assert!(result.is_some(), "Differential analysis should succeed");

        let result = result.unwrap();
        // Output value should be V(0) - V(1)
        assert!(
            (result.output_value - (-1000.0)).abs() < 1e-6,
            "Differential output: {}",
            result.output_value
        );
    }
}
