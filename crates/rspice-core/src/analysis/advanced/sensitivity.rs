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

#![allow(clippy::needless_range_loop)]
use crate::{Complex64, Value};

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
    Diode,
    Bjt,
    Mosfet,
    Jfet,
    Mesfet,
    BehavioralSource,
    Switch,
    TransmissionLine,
    Coupling,
    Xspice,
    Model,
    Other,
}

/// Output selected for a complete AC sensitivity analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AcSensitivityOutput {
    /// Node voltage, optionally relative to a second node. Node indices use
    /// the public SPICE convention: ground is zero and non-ground nodes are
    /// one-based.
    Voltage {
        positive: usize,
        negative: Option<usize>,
    },
    /// Current through a branch-producing element (normally a voltage
    /// source), matched case-insensitively against the AC branch names.
    BranchCurrent(String),
}

/// Frequency-dependent sensitivity of one output to one real-valued circuit
/// parameter.
#[derive(Debug, Clone)]
pub struct AcSensitivity {
    /// Stable SPICE-compatible vector name (`R1`, `M1_W`, `MOD:VTO`, ...).
    pub vector_name: String,
    /// Device or model that owns the parameter.
    pub element: String,
    /// Broad owner category.
    pub element_type: ElementType,
    /// Parameter name within the owner.
    pub parameter: String,
    /// Nominal real parameter value.
    pub nominal_value: Value,
    /// Complex derivative of the selected output at every frequency.
    pub absolute: Vec<Complex64>,
    /// Complex normalized derivative `(p / output) * d(output)/dp`.
    pub normalized: Vec<Complex64>,
    /// Derivative of output magnitude with respect to the parameter.
    pub magnitude: Vec<Value>,
    /// Derivative of output phase in radians with respect to the parameter.
    pub phase: Vec<Value>,
}

/// Complete netlist-wide AC sensitivity result.
#[derive(Debug, Clone)]
pub struct AcSensitivityResult {
    /// Human-readable selected output probe.
    pub output: String,
    /// Frequency grid in hertz.
    pub frequencies: Vec<Value>,
    /// Nominal complex output at every frequency.
    pub output_values: Vec<Complex64>,
    /// One trace for every eligible, selected real-valued parameter.
    pub sensitivities: Vec<AcSensitivity>,
}

impl AcSensitivityResult {
    /// Return the trace with this vector name, case-insensitively.
    pub fn get(&self, vector_name: &str) -> Option<&AcSensitivity> {
        self.sensitivities
            .iter()
            .find(|trace| trace.vector_name.eq_ignore_ascii_case(vector_name))
    }

    /// Number of parameter traces in this result.
    pub fn len(&self) -> usize {
        self.sensitivities.len()
    }

    /// Whether no parameter matched the requested selection.
    pub fn is_empty(&self) -> bool {
        self.sensitivities.is_empty()
    }
}

/// A single sensitivity value
#[derive(Debug, Clone)]
pub struct Sensitivity {
    /// Stable SPICE-compatible vector name (`R1`, `M1_W`, `MOD:VTO`, ...).
    pub vector_name: String,
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
        let normalized = if normalized.is_finite() {
            normalized
        } else {
            0.0
        };

        Self {
            vector_name: element.to_string(),
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
        // If dy/y = S * dp/p, then a one-percent parameter change produces
        // S percent output change. The numeric percent-per-percent value is
        // therefore the normalized sensitivity itself, not 100*S.
        self.normalized
    }

    /// Create an entry with a distinct SPICE vector name.
    #[allow(clippy::too_many_arguments)]
    pub fn new_named(
        vector_name: &str,
        element: &str,
        element_type: ElementType,
        parameter: &str,
        nominal: Value,
        absolute: Value,
        output_value: Value,
    ) -> Self {
        let mut sensitivity = Self::new(
            element,
            element_type,
            parameter,
            nominal,
            absolute,
            output_value,
        );
        sensitivity.vector_name = vector_name.to_string();
        sensitivity
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
        self.sensitivities.iter().find(|s| {
            s.vector_name.eq_ignore_ascii_case(element) || s.element.eq_ignore_ascii_case(element)
        })
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
    /// Optional MNA branch-equation index for branch-based elements.
    pub branch_index: Option<usize>,
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
            branch_index: None,
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
            branch_index: None,
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
            branch_index: None,
            value: current,
        }
    }

    /// Create independent voltage source element.
    ///
    /// `branch_index` is the 0-based MNA branch-equation index.
    pub fn voltage_source(
        name: &str,
        n_pos: Option<usize>,
        n_neg: Option<usize>,
        branch_index: usize,
        voltage: Value,
    ) -> Self {
        Self {
            name: name.to_string(),
            element_type: ElementType::VoltageSource,
            node_pos: n_pos,
            node_neg: n_neg,
            branch_index: Some(branch_index),
            value: voltage,
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
    /// Dimension of the linearized MNA system (nodes + branch equations).
    system_size: usize,
    /// Conductance matrix G
    g_matrix: Vec<Vec<Value>>,
    /// Linearized MNA operating-point solution (node voltages + branch currents)
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
        let system_size = g_matrix.len();
        Self {
            system_size,
            g_matrix,
            solution,
            adjoint: vec![0.0; system_size],
            elements,
        }
    }

    fn solve_adjoint_transposed(&mut self, output_node: usize) -> bool {
        if output_node >= self.system_size {
            return false;
        }

        let mut e = vec![0.0; self.system_size];
        e[output_node] = 1.0;
        let n = self.system_size;
        let mut aug = vec![vec![0.0; n + 1]; n];

        for row in 0..n {
            for col in 0..n {
                aug[row][col] = self.g_matrix[col][row];
            }
            aug[row][n] = e[row];
        }

        for k in 0..n {
            let mut max_row = k;
            let mut max_val = aug[k][k].abs();
            for i in (k + 1)..n {
                if aug[i][k].abs() > max_val {
                    max_val = aug[i][k].abs();
                    max_row = i;
                }
            }

            if max_val < 1e-15 {
                return false;
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

    /// Compute sensitivity of an independent voltage source DC value.
    ///
    /// In MNA, the source value appears directly in the branch equation RHS,
    /// so d(output)/dVs equals the adjoint value at that branch row.
    fn voltage_source_sensitivity(&self, elem: &ElementDesc) -> Value {
        elem.branch_index
            .and_then(|idx| self.adjoint.get(idx).copied())
            .unwrap_or(0.0)
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
            Some(r) if r < self.system_size => self.solution[output_node] - self.solution[r],
            _ => self.solution[output_node],
        };

        // Solve adjoint for output node
        if !self.solve_adjoint_transposed(output_node) {
            return None;
        }

        // If differential output, also solve for reference and combine
        if let Some(ref_node) = output_ref
            && ref_node < self.system_size
        {
            // Save current adjoint
            let adj_output = self.adjoint.clone();

            // Solve for reference node
            if !self.solve_adjoint_transposed(ref_node) {
                return None;
            }

            // Combine: λ = λ_output - λ_ref
            for i in 0..self.system_size {
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
                ElementType::VoltageSource => self.voltage_source_sensitivity(elem),
                ElementType::Inductor
                | ElementType::Transconductance
                | ElementType::Transresistance
                | ElementType::Diode
                | ElementType::Bjt
                | ElementType::Mosfet
                | ElementType::Jfet
                | ElementType::Mesfet
                | ElementType::BehavioralSource
                | ElementType::Switch
                | ElementType::TransmissionLine
                | ElementType::Coupling
                | ElementType::Xspice
                | ElementType::Model
                | ElementType::Other => self.unsupported_linearized_sensitivity(elem),
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
