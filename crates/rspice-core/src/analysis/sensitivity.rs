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
//! ∂xₖ/∂p = λᵀ · (∂b/∂p - ∂G/∂p · x)
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

use crate::abort_signal::{AbortSignal, NoAbort};
use crate::{Complex64, Value};
use rspice_veriloga_runtime::arithmetic::ScaledValue;
use serde::{Deserialize, Serialize};

/// A sensitivity quantity, or the reason it has no representable value.
/// Available samples serialize as ordinary numbers (or complex samples).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum SensitivityValue<T> {
    Available(T),
    Unavailable {
        unavailable: SensitivityUnavailability,
    },
}

impl<T> From<T> for SensitivityValue<T> {
    fn from(value: T) -> Self {
        Self::Available(value)
    }
}

/// Why a derived sensitivity cannot be reported as a number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SensitivityUnavailability {
    ZeroOutput,
    NondifferentiableMagnitude,
    OutOfRange,
    /// Malformed input, not a mathematical determination about a circuit.
    InvalidInput,
}

impl SensitivityUnavailability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ZeroOutput => "zero-output",
            Self::NondifferentiableMagnitude => "nondifferentiable-magnitude",
            Self::OutOfRange => "out-of-range",
            Self::InvalidInput => "invalid-input",
        }
    }
}

impl<T> SensitivityValue<T> {
    pub const fn unavailable(reason: SensitivityUnavailability) -> Self {
        Self::Unavailable {
            unavailable: reason,
        }
    }

    pub const fn reason(&self) -> Option<SensitivityUnavailability> {
        match self {
            Self::Available(_) => None,
            Self::Unavailable { unavailable } => Some(*unavailable),
        }
    }

    pub fn value(self) -> Option<T> {
        match self {
            Self::Available(value) => Some(value),
            Self::Unavailable { .. } => None,
        }
    }

    pub fn map<U>(self, map: impl FnOnce(T) -> U) -> SensitivityValue<U> {
        match self {
            Self::Available(value) => SensitivityValue::Available(map(value)),
            Self::Unavailable { unavailable } => SensitivityValue::unavailable(unavailable),
        }
    }

    pub fn zip<U>(self, other: SensitivityValue<U>) -> SensitivityValue<(T, U)> {
        match (self, other) {
            (Self::Available(left), SensitivityValue::Available(right)) => {
                SensitivityValue::Available((left, right))
            }
            (Self::Unavailable { unavailable }, _)
            | (_, SensitivityValue::Unavailable { unavailable }) => {
                SensitivityValue::unavailable(unavailable)
            }
        }
    }
}

impl SensitivityValue<Value> {
    pub(crate) fn from_scaled(value: ScaledValue) -> Self {
        let result = value.binary64();
        if !result.is_finite() || (result == 0.0 && !value.is_zero()) {
            Self::unavailable(SensitivityUnavailability::OutOfRange)
        } else {
            Self::Available(result)
        }
    }

    /// Relative output change per relative parameter change. Zero output has
    /// no relative scale, even when the parameter or derivative is also zero.
    pub fn normalized(nominal: Value, absolute: Value, output: Value) -> Self {
        if [nominal, absolute, output]
            .iter()
            .any(|value| !value.is_finite())
        {
            return Self::unavailable(SensitivityUnavailability::InvalidInput);
        }
        if output == 0.0 {
            return Self::unavailable(SensitivityUnavailability::ZeroOutput);
        }
        Self::from_scaled(
            ScaledValue::new(nominal)
                .multiply(ScaledValue::new(absolute))
                .divide(ScaledValue::new(output)),
        )
    }

    /// Convert a derived real quantity's units without losing availability or
    /// rounding a nonzero value below the representable range to zero.
    pub fn scaled(self, factor: Value) -> Self {
        match self {
            Self::Available(value) if value.is_finite() && factor.is_finite() => {
                Self::from_scaled(ScaledValue::new(value).multiply(ScaledValue::new(factor)))
            }
            Self::Available(_) => Self::unavailable(SensitivityUnavailability::InvalidInput),
            unavailable => unavailable,
        }
    }

    /// Derivative of `|output|` from a complex output derivative.
    /// At zero output the magnitude is differentiable only if the complex
    /// derivative is also zero. Intermediate norms and products retain their
    /// exponents, so a finite derivative does not require a finite norm.
    pub fn magnitude(output: Complex64, derivative: Complex64) -> Self {
        if [output.re, output.im, derivative.re, derivative.im]
            .iter()
            .any(|value| !value.is_finite())
        {
            return Self::unavailable(SensitivityUnavailability::InvalidInput);
        }
        let scale = output.re.abs().max(output.im.abs());
        if scale == 0.0 {
            return if derivative == Complex64::new(0.0, 0.0) {
                Self::Available(0.0)
            } else {
                Self::unavailable(SensitivityUnavailability::NondifferentiableMagnitude)
            };
        }
        let norm = ScaledValue::new(scale).multiply(ScaledValue::new(
            (output.re / scale).hypot(output.im / scale),
        ));
        let one = ScaledValue::new(1.0);
        match ScaledValue::sum_triple_products_ratio(
            [
                [
                    ScaledValue::new(output.re),
                    ScaledValue::new(derivative.re),
                    one,
                ],
                [
                    ScaledValue::new(output.im),
                    ScaledValue::new(derivative.im),
                    one,
                ],
            ]
            .into_iter(),
            [[norm, one, one]].into_iter(),
        ) {
            Ok(value) => Self::from_scaled(value),
            Err(_) => Self::unavailable(SensitivityUnavailability::InvalidInput),
        }
    }

    /// Derivative of `20 log10 |output|` from a complex output derivative.
    /// The dot product and squared norm retain their exponents, including
    /// when the output's magnitude itself exceeds the binary64 range.
    pub fn decibels(output: Complex64, derivative: Complex64) -> Self {
        if [output.re, output.im, derivative.re, derivative.im]
            .iter()
            .any(|value| !value.is_finite())
        {
            return Self::unavailable(SensitivityUnavailability::InvalidInput);
        }
        if output == Complex64::new(0.0, 0.0) {
            return Self::unavailable(SensitivityUnavailability::ZeroOutput);
        }
        let re = ScaledValue::new(output.re);
        let im = ScaledValue::new(output.im);
        let factor = ScaledValue::new(20.0 / std::f64::consts::LN_10);
        let one = ScaledValue::new(1.0);
        match ScaledValue::sum_triple_products_ratio(
            [
                [re, ScaledValue::new(derivative.re), factor],
                [im, ScaledValue::new(derivative.im), factor],
            ]
            .into_iter(),
            [[re, re, one], [im, im, one]].into_iter(),
        ) {
            Ok(value) => Self::from_scaled(value),
            Err(_) => Self::unavailable(SensitivityUnavailability::InvalidInput),
        }
    }
}

impl<T: std::fmt::LowerExp> std::fmt::LowerExp for SensitivityValue<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Available(value) => std::fmt::LowerExp::fmt(value, formatter),
            Self::Unavailable { unavailable } => {
                write!(formatter, "unavailable ({})", unavailable.as_str())
            }
        }
    }
}

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
    pub normalized: Vec<SensitivityValue<Complex64>>,
    /// Derivative of output magnitude with respect to the parameter.
    pub magnitude: Vec<SensitivityValue<Value>>,
    /// Derivative of output phase in radians with respect to the parameter.
    pub phase: Vec<SensitivityValue<Value>>,
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
    pub normalized: SensitivityValue<Value>,
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
        let normalized = SensitivityValue::normalized(nominal, absolute, output_value);

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
    pub fn percent_per_percent(&self) -> SensitivityValue<Value> {
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
    /// Parameter value (resistance for R, capacitance for C, etc.)
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
        1.0 / self.value
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

/// Failure while solving or projecting an adjoint sensitivity result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SensitivityAnalysisError {
    /// The caller cancelled the operation.
    Aborted,
}

impl std::fmt::Display for SensitivityAnalysisError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Aborted => formatter.write_str("sensitivity analysis was aborted"),
        }
    }
}

impl std::error::Error for SensitivityAnalysisError {}

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

    /// Create an analyzer from an adjoint vector solved by an external sparse
    /// backend. This keeps the element-derivative implementation shared while
    /// allowing production callers to avoid materializing and eliminating a
    /// dense copy of a sparse MNA matrix.
    pub fn with_precomputed_adjoint(
        solution: Vec<Value>,
        adjoint: Vec<Value>,
        elements: Vec<ElementDesc>,
    ) -> Option<Self> {
        if solution.is_empty()
            || solution.len() != adjoint.len()
            || solution
                .iter()
                .chain(adjoint.iter())
                .any(|value| !value.is_finite())
        {
            return None;
        }
        Some(Self {
            system_size: solution.len(),
            g_matrix: Vec::new(),
            solution,
            adjoint,
            elements,
        })
    }

    fn solve_adjoint_transposed_with_abort(
        &mut self,
        output_node: usize,
        abort: &dyn AbortSignal,
    ) -> Result<bool, SensitivityAnalysisError> {
        ensure_sensitivity_not_aborted(abort)?;
        if output_node >= self.system_size {
            return Ok(false);
        }

        let mut e = vec![0.0; self.system_size];
        e[output_node] = 1.0;
        let n = self.system_size;
        let mut aug = vec![vec![0.0; n + 1]; n];
        let mut trial = vec![0.0; n];
        let mut work = 0usize;

        for row in 0..n {
            for (col, entry) in aug[row][..n].iter_mut().enumerate() {
                poll_sensitivity_work(abort, &mut work)?;
                *entry = self.g_matrix[col][row];
            }
            aug[row][n] = e[row];
            let scale = aug[row][..n]
                .iter()
                .fold(0.0_f64, |scale, value| scale.max(value.abs()));
            if scale == 0.0 {
                return Ok(false);
            }
            for value in &mut aug[row] {
                poll_sensitivity_work(abort, &mut work)?;
                let scaled = *value / scale;
                if !scaled.is_finite() || (scaled == 0.0 && *value != 0.0) {
                    return Ok(false);
                }
                *value = scaled;
            }
        }

        for k in 0..n {
            poll_sensitivity_work(abort, &mut work)?;
            let mut max_row = k;
            let mut max_val = aug[k][k].abs();
            for (i, entries) in aug.iter().enumerate().take(n).skip(k + 1) {
                poll_sensitivity_work(abort, &mut work)?;
                if entries[k].abs() > max_val {
                    max_val = entries[k].abs();
                    max_row = i;
                }
            }

            if max_val == 0.0 || !max_val.is_finite() {
                return Ok(false);
            }

            if max_row != k {
                aug.swap(k, max_row);
            }

            let pivot = aug[k][k];
            for i in (k + 1)..n {
                poll_sensitivity_work(abort, &mut work)?;
                // `i > k`, so the pivot row stays in `above`.
                let (above, below) = aug.split_at_mut(i);
                let pivot_row = &above[k];
                let target_row = &mut below[0];
                let factor = target_row[k] / pivot;
                target_row[k] = 0.0;
                for (target, &value) in target_row[(k + 1)..=n]
                    .iter_mut()
                    .zip(&pivot_row[(k + 1)..=n])
                {
                    poll_sensitivity_work(abort, &mut work)?;
                    *target -= factor * value;
                }
            }
        }

        for i in (0..n).rev() {
            poll_sensitivity_work(abort, &mut work)?;
            let mut sum = aug[i][n];
            for (j, &coefficient) in aug[i].iter().enumerate().take(n).skip(i + 1) {
                poll_sensitivity_work(abort, &mut work)?;
                sum -= coefficient * trial[j];
            }
            trial[i] = sum / aug[i][i];
            if !trial[i].is_finite() {
                return Ok(false);
            }
        }

        // Certify the original transpose, not the rounded triangular system.
        // A finite solve can still be inaccurate after cancellation or growth.
        for row in 0..n {
            poll_sensitivity_index(abort, row)?;
            let terms = (0..aug[row].len()).map(|column| {
                if column == n {
                    (-e[row], 1.0)
                } else {
                    (self.g_matrix[column][row], trial[column])
                }
            });
            let Ok(residual) = ScaledValue::sum_products_div(
                terms.clone().map(|(coefficient, value)| {
                    [ScaledValue::new(coefficient), ScaledValue::new(value)]
                }),
                ScaledValue::new(1.0),
            ) else {
                return Ok(false);
            };
            let mut scale = ScaledValue::new(0.0);
            for (coefficient, value) in terms {
                poll_sensitivity_work(abort, &mut work)?;
                scale = ScaledValue::product_sum(
                    ScaledValue::new(coefficient.abs()),
                    ScaledValue::new(value.abs()),
                    scale,
                    ScaledValue::new(1.0),
                );
            }
            if !residual.is_zero()
                && (scale.is_zero()
                    || residual.divide(scale).binary64().abs()
                        > 128.0 * Value::EPSILON * n as Value)
            {
                return Ok(false);
            }
        }
        ensure_sensitivity_not_aborted(abort)?;
        self.adjoint = trial;
        Ok(true)
    }

    /// Compute sensitivity of a resistor
    ///
    /// For resistor R between nodes i and j:
    /// ∂V/∂R = -λᵀ · (∂G/∂R · V)
    ///       = -λᵀ · (-1/R² · stamps) · V
    ///       = (1/R²) · (λᵢ - λⱼ) · (Vᵢ - Vⱼ)
    fn resistor_sensitivity(&self, elem: &ElementDesc) -> Value {
        if let Some(branch) = elem.branch_index {
            // Vp - Vn - R*I = 0: -lambda^T (dG/dR) x = lambda_branch * I.
            return ScaledValue::new(self.adjoint[branch])
                .multiply(ScaledValue::new(self.solution[branch]))
                .binary64();
        }
        let r = elem.value;
        let difference = |values: &[Value]| {
            ScaledValue::product_sum(
                ScaledValue::new(elem.node_pos.map_or(0.0, |index| values[index])),
                ScaledValue::new(1.0),
                ScaledValue::new(-elem.node_neg.map_or(0.0, |index| values[index])),
                ScaledValue::new(1.0),
            )
        };

        // ∂G/∂R = -G² = -1/R²
        // Sensitivity = -λᵀ · (∂G/∂R · V) = (1/R²) · (λᵢ - λⱼ) · (Vᵢ - Vⱼ)
        difference(&self.adjoint)
            .divide(ScaledValue::new(r))
            .multiply(difference(&self.solution).divide(ScaledValue::new(r)))
            .binary64()
    }

    /// Compute sensitivity of a capacitor (DC case: no effect)
    fn capacitor_sensitivity(&self, _elem: &ElementDesc) -> Value {
        // At DC, capacitors are open circuits - no sensitivity
        0.0
    }

    /// Compute sensitivity of an independent current source.
    ///
    /// MNA stamp for a source flowing from n_pos -> n_neg:
    /// b`[n_pos]` -= I, b`[n_neg]` += I
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
    ///
    /// Returns `None` for invalid dimensions/indices, unsupported element
    /// derivatives, or a singular, nonfinite or inaccurate adjoint solution.
    pub fn analyze(
        &mut self,
        output_node: usize,
        output_ref: Option<usize>,
    ) -> Option<SensitivityResult> {
        self.analyze_with_abort(output_node, output_ref, &NoAbort)
            .expect("NoAbort cannot cancel sensitivity analysis")
    }

    /// Run sensitivity analysis with cooperative cancellation during the
    /// dense adjoint solve and per-element result projection.
    pub fn analyze_with_abort(
        &mut self,
        output_node: usize,
        output_ref: Option<usize>,
        abort: &dyn AbortSignal,
    ) -> Result<Option<SensitivityResult>, SensitivityAnalysisError> {
        ensure_sensitivity_not_aborted(abort)?;
        if output_node >= self.system_size
            || output_ref.is_some_and(|reference| reference >= self.system_size)
            || !self.valid_vectors_with_abort(abort)?
            || self.g_matrix.len() != self.system_size
        {
            return Ok(None);
        }
        let mut work = 0;
        for row in &self.g_matrix {
            if row.len() != self.system_size {
                return Ok(None);
            }
            for &value in row {
                poll_sensitivity_work(abort, &mut work)?;
                if !value.is_finite() {
                    return Ok(None);
                }
            }
        }
        // Get output value
        let output_value = match output_ref {
            Some(r) if r < self.system_size => self.solution[output_node] - self.solution[r],
            _ => self.solution[output_node],
        };

        // Solve adjoint for output node
        if !self.solve_adjoint_transposed_with_abort(output_node, abort)? {
            return Ok(None);
        }

        // If differential output, also solve for reference and combine
        if let Some(ref_node) = output_ref
            && ref_node < self.system_size
        {
            // Save current adjoint
            let adj_output = self.adjoint.clone();

            // Solve for reference node
            if !self.solve_adjoint_transposed_with_abort(ref_node, abort)? {
                return Ok(None);
            }

            // Combine: λ = λ_output - λ_ref
            let system_size = self.system_size;
            for (i, (adjoint, &output)) in self
                .adjoint
                .iter_mut()
                .zip(&adj_output)
                .take(system_size)
                .enumerate()
            {
                poll_sensitivity_index(abort, i)?;
                *adjoint = output - *adjoint;
            }
        }

        self.build_result_with_abort(output_node, output_value, abort)
    }

    /// Assemble sensitivities from the adjoint supplied to
    /// [`Self::with_precomputed_adjoint`]. `output_ref` affects only the
    /// reported nominal output; its `-1` observation coefficient must already
    /// be present in the supplied adjoint RHS.
    pub fn analyze_precomputed(
        &self,
        output_node: usize,
        output_ref: Option<usize>,
    ) -> Option<SensitivityResult> {
        self.analyze_precomputed_with_abort(output_node, output_ref, &NoAbort)
            .expect("NoAbort cannot cancel sensitivity result projection")
    }

    /// Assemble a precomputed-adjoint result with cooperative cancellation.
    pub fn analyze_precomputed_with_abort(
        &self,
        output_node: usize,
        output_ref: Option<usize>,
        abort: &dyn AbortSignal,
    ) -> Result<Option<SensitivityResult>, SensitivityAnalysisError> {
        ensure_sensitivity_not_aborted(abort)?;
        if output_node >= self.system_size
            || output_ref.is_some_and(|reference| reference >= self.system_size)
            || !self.valid_vectors_with_abort(abort)?
        {
            return Ok(None);
        }
        let output_value = output_ref
            .map(|reference| self.solution[output_node] - self.solution[reference])
            .unwrap_or(self.solution[output_node]);
        self.build_result_with_abort(output_node, output_value, abort)
    }

    fn valid_vectors_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<bool, SensitivityAnalysisError> {
        if self.solution.len() != self.system_size || self.adjoint.len() != self.system_size {
            return Ok(false);
        }
        for (index, value) in self.solution.iter().chain(&self.adjoint).enumerate() {
            poll_sensitivity_index(abort, index)?;
            if !value.is_finite() {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn build_result_with_abort(
        &self,
        output_node: usize,
        output_value: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Option<SensitivityResult>, SensitivityAnalysisError> {
        if !output_value.is_finite() {
            return Ok(None);
        }
        let mut result = SensitivityResult::new(&format!("V({})", output_node + 1), output_value);

        // Compute sensitivity for each element
        for (index, elem) in self.elements.iter().enumerate() {
            poll_sensitivity_index(abort, index)?;
            if !elem.value.is_finite()
                || [elem.node_pos, elem.node_neg, elem.branch_index]
                    .into_iter()
                    .flatten()
                    .any(|index| index >= self.system_size)
                || (elem.element_type == ElementType::Resistor
                    && elem.value == 0.0
                    && elem.branch_index.is_none())
                || (elem.element_type == ElementType::VoltageSource && elem.branch_index.is_none())
            {
                return Ok(None);
            }
            let absolute = match elem.element_type {
                ElementType::Resistor => self.resistor_sensitivity(elem),
                ElementType::Capacitor | ElementType::Inductor => self.capacitor_sensitivity(elem),
                ElementType::CurrentSource => self.current_source_sensitivity(elem),
                ElementType::VoltageSource => self.voltage_source_sensitivity(elem),
                ElementType::Transconductance
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
                | ElementType::Other => return Ok(None),
            };
            if !absolute.is_finite() {
                return Ok(None);
            }

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

        ensure_sensitivity_not_aborted(abort)?;
        Ok(Some(result))
    }
}

const SENSITIVITY_ABORT_POLL_STRIDE: usize = 256;

#[inline]
fn ensure_sensitivity_not_aborted(abort: &dyn AbortSignal) -> Result<(), SensitivityAnalysisError> {
    if abort.is_aborted() {
        Err(SensitivityAnalysisError::Aborted)
    } else {
        Ok(())
    }
}

#[inline]
fn poll_sensitivity_index(
    abort: &dyn AbortSignal,
    index: usize,
) -> Result<(), SensitivityAnalysisError> {
    if index.is_multiple_of(SENSITIVITY_ABORT_POLL_STRIDE) {
        ensure_sensitivity_not_aborted(abort)?;
    }
    Ok(())
}

#[inline]
fn poll_sensitivity_work(
    abort: &dyn AbortSignal,
    work: &mut usize,
) -> Result<(), SensitivityAnalysisError> {
    poll_sensitivity_index(abort, *work)?;
    *work = work.wrapping_add(1);
    Ok(())
}

//=============================================================================
// Finite Difference Verification Helper
//=============================================================================

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::CountingAbort;

    #[test]
    fn magnitude_sensitivity_preserves_scale_and_reports_undefined_values() {
        for scale in [1e-300, 1.0, 1.5e308] {
            let output = Complex64::new(scale, scale);
            let derivative = SensitivityValue::magnitude(output, Complex64::new(1.0, -0.5));
            let expected = 0.5 / 2.0_f64.sqrt();
            assert!((derivative.value().unwrap() / expected - 1.0).abs() < 2e-14);
        }
        assert_eq!(
            SensitivityValue::magnitude(Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)),
            SensitivityValue::Available(0.0)
        );
        for (output, derivative, reason) in [
            (
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 1.0),
                SensitivityUnavailability::NondifferentiableMagnitude,
            ),
            (
                Complex64::new(1.0, 1.0),
                Complex64::new(f64::MAX, f64::MAX),
                SensitivityUnavailability::OutOfRange,
            ),
            (
                Complex64::new(1.0, 1.0),
                Complex64::new(f64::NAN, 0.0),
                SensitivityUnavailability::InvalidInput,
            ),
            (
                Complex64::new(f64::INFINITY, 1.0),
                Complex64::new(1.0, 0.0),
                SensitivityUnavailability::InvalidInput,
            ),
        ] {
            assert_eq!(
                SensitivityValue::magnitude(output, derivative).reason(),
                Some(reason)
            );
        }
    }

    #[test]
    fn adjoint_refuses_malformed_systems_and_element_descriptions_without_panicking() {
        for (matrix, solution) in [
            (vec![vec![]], vec![1.0]),
            (vec![vec![1.0]], vec![]),
            (vec![vec![1.0]], vec![1.0, 2.0]),
            (vec![vec![Value::NAN]], vec![1.0]),
            (vec![vec![1.0]], vec![Value::INFINITY]),
        ] {
            assert!(
                SensitivityAnalyzer::new(matrix, solution, vec![])
                    .analyze(0, None)
                    .is_none()
            );
        }
        let mut invalid = ElementDesc::resistor("R", Some(1), None, 1.0);
        for kind in [
            ElementType::Resistor,
            ElementType::Capacitor,
            ElementType::CurrentSource,
        ] {
            invalid.element_type = kind;
            let analyzer = SensitivityAnalyzer::with_precomputed_adjoint(
                vec![1.0],
                vec![1.0],
                vec![invalid.clone()],
            )
            .unwrap();
            assert!(analyzer.analyze_precomputed(0, None).is_none());
        }
        for element in [
            ElementDesc::resistor("R", Some(0), None, 0.0),
            ElementDesc::resistor("R", Some(0), None, Value::NAN),
            ElementDesc::voltage_source("V", Some(0), None, 1, 1.0),
        ] {
            assert!(
                SensitivityAnalyzer::new(vec![vec![1.0]], vec![1.0], vec![element])
                    .analyze(0, None)
                    .is_none()
            );
        }
        let mut analyzer =
            SensitivityAnalyzer::with_precomputed_adjoint(vec![1.0], vec![1.0], vec![]).unwrap();
        assert!(analyzer.analyze(0, None).is_none());
    }

    #[test]
    fn adjoint_refuses_unsupported_derivatives_instead_of_reporting_zero() {
        let mut element = ElementDesc::resistor("D", Some(0), None, 1.0);
        element.element_type = ElementType::Diode;
        let analyzer =
            SensitivityAnalyzer::with_precomputed_adjoint(vec![1.0], vec![1.0], vec![element])
                .unwrap();
        assert!(analyzer.analyze_precomputed(0, None).is_none());
    }

    #[test]
    fn adjoint_preserves_resistor_derivatives_at_extreme_scales() {
        for resistance in [1e-200, 1e-20, 1.0, 1e20, 1e200, -1e200] {
            let element = ElementDesc::resistor("R", Some(0), None, resistance);
            let mut dense = SensitivityAnalyzer::new(
                vec![vec![1.0 / resistance]],
                vec![1.0],
                vec![element.clone()],
            );
            let sparse = SensitivityAnalyzer::with_precomputed_adjoint(
                vec![1.0],
                vec![resistance],
                vec![element],
            )
            .unwrap();
            for result in [dense.analyze(0, None), sparse.analyze_precomputed(0, None)] {
                let result = result.expect("the finite one-resistor sensitivity is defined");
                let resistor = result.get("R").unwrap();
                assert!(
                    (resistor.absolute * resistance - 1.0).abs() < 2e-14,
                    "R={resistance:e}: {resistor:?}"
                );
                assert!((resistor.normalized.value().unwrap() - 1.0).abs() < 2e-14);
            }
        }
        let element = ElementDesc::resistor("R", Some(0), Some(1), 1e200);
        let result = SensitivityAnalyzer::with_precomputed_adjoint(
            vec![1e308, -1e308],
            vec![1e200, -1e200],
            vec![element],
        )
        .unwrap()
        .analyze_precomputed(0, None)
        .unwrap();
        assert!((result.get("R").unwrap().absolute / 4e108 - 1.0).abs() < 2e-14);
    }

    #[test]
    fn normalized_sensitivity_has_no_dimensionful_output_floor() {
        for (parameter, derivative, output, expected) in [
            (1e-200, 1.0, 1e-200, 1.0),
            (1e200, 1e-200, 1e-200, 1e200),
            (1e-200, 1e200, 1e200, 1e-200),
        ] {
            let result = Sensitivity::new(
                "R",
                ElementType::Resistor,
                "value",
                parameter,
                derivative,
                output,
            );
            assert!(
                (result.normalized.value().unwrap() / expected - 1.0).abs() < 2e-14,
                "{result:?}"
            );
        }
    }

    #[test]
    fn sensitivity_availability_distinguishes_zero_scale_range_and_invalid_input() {
        use super::{SensitivityUnavailability as Reason, SensitivityValue};
        for (parameter, derivative, output, reason) in [
            (1.0, 2.0, 0.0, Reason::ZeroOutput),
            (0.0, 0.0, 0.0, Reason::ZeroOutput),
            (1e200, 1e200, 1.0, Reason::OutOfRange),
            (1e-200, 1e-200, 1.0, Reason::OutOfRange),
            (1.0, 2.0, f64::NAN, Reason::InvalidInput),
        ] {
            let result = Sensitivity::new(
                "R",
                ElementType::Resistor,
                "value",
                parameter,
                derivative,
                output,
            );
            assert_eq!(result.absolute, derivative);
            assert_eq!(result.normalized.reason(), Some(reason));
            assert_eq!(result.percent_per_percent().value(), None);
        }
        assert_eq!(
            SensitivityValue::normalized(0.0, 2.0, 1.0).value(),
            Some(0.0)
        );
        // The final ratio is finite despite the overflowing intermediate product.
        assert_eq!(
            SensitivityValue::normalized(1e200, 1e200, 1e200).value(),
            Some(1e200)
        );
    }

    #[test]
    fn dense_adjoint_preserves_subnormal_row_constraints() {
        let tiny = Value::from_bits(1);
        // G^T * lambda = [1, 0] implies lambda = [1/2, -1/2].
        // Dividing the second pivot row by two must not erase its coupling.
        let mut analyzer = SensitivityAnalyzer::new(
            vec![vec![2.0, tiny], vec![0.0, tiny]],
            vec![1.0, 1.0],
            vec![ElementDesc::current_source("I", None, Some(1), 1.0)],
        );
        let result = analyzer.analyze(0, None).unwrap();
        assert_eq!(result.get("I").unwrap().absolute, -0.5);
    }

    #[test]
    fn dense_adjoint_preserves_scaled_transpose_and_differential_observation() {
        for scale in [1e-200, 1.0, 1e200] {
            let mut analyzer = SensitivityAnalyzer::new(
                vec![vec![2.0 * scale, -scale], vec![0.0, scale]],
                vec![1.0, 2.0],
                vec![ElementDesc::current_source("I", None, Some(0), scale)],
            );
            let result = analyzer.analyze(0, None).unwrap();
            assert!((result.get("I").unwrap().absolute * scale - 0.5).abs() < 2e-14);
            let result = analyzer.analyze(0, Some(1)).unwrap();
            assert_eq!(result.output_value, -1.0);
            assert!((result.get("I").unwrap().absolute * scale - 0.5).abs() < 2e-14);
        }
    }

    #[test]
    fn precomputed_projection_observes_abort_within_one_poll_stride() {
        let elements = (0..SENSITIVITY_ABORT_POLL_STRIDE * 4)
            .map(|index| ElementDesc::resistor(&format!("R{index}"), Some(0), None, 1_000.0))
            .collect();
        let analyzer =
            SensitivityAnalyzer::with_precomputed_adjoint(vec![1.0], vec![1.0], elements)
                .expect("precomputed sensitivity fixture is valid");
        let abort = CountingAbort::new(2);

        let error = analyzer
            .analyze_precomputed_with_abort(0, None, &abort)
            .expect_err("counted cancellation must stop sensitivity projection");

        assert_eq!(error, SensitivityAnalysisError::Aborted);
        assert_eq!(
            abort.count(),
            3,
            "projection must stop on the first true poll"
        );
    }

    #[test]
    fn dense_adjoint_entry_preserves_typed_abort() {
        let mut analyzer = SensitivityAnalyzer::new(
            vec![vec![1.0]],
            vec![1.0],
            vec![ElementDesc::resistor("R1", Some(0), None, 1_000.0)],
        );
        let abort = CountingAbort::new(1);

        let error = analyzer
            .analyze_with_abort(0, None, &abort)
            .expect_err("counted cancellation must stop the dense adjoint path");

        assert_eq!(error, SensitivityAnalysisError::Aborted);
        assert_eq!(
            abort.count(),
            2,
            "adjoint path must stop on the first true poll"
        );
    }
}
