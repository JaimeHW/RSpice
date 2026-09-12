//! Behavioral sources (B-elements)
//!
//! Implements voltage and current sources defined by arbitrary expressions.
//! Expressions are compiled to bytecode during circuit build for efficient
//! evaluation in the Newton-Raphson loop.

use crate::Value;
use crate::config::ExpressionDialect;
use crate::expr::{
    BinaryOp, CompiledExpr, Context, EXPR_ZERO_TOLERANCE, Expr, Function, UnaryOp, Vm, compile,
    lookup_table_interpolate_with_derivative, normalize_expression_boundary, ordered_limit,
    ordered_sign, parse_expression_strict, real_function_pow_with_derivative,
    real_function_pwr_with_derivative, real_function_pwrs_with_derivative,
    real_pow_with_derivative, resolve_file_lookup_functions_with_limits,
};
use crate::expr::{Derivative, derivative_pair};
use crate::solver::StaticMatrix;
use std::path::Path;
use thiserror::Error;

mod breakpoints;
mod periodicity;
mod resolution;
pub(crate) use breakpoints::BehavioralBreakpointError;
#[cfg(test)]
use breakpoints::expression_transient_breakpoints;

const DERIVATIVE_REL_STEP: Value = 1e-6;
const DERIVATIVE_ABS_STEP: Value = 1e-9;
const XYCE_ATANH_EPSILON: Value = 1.0e-12;
const XYCE_TANH_SATURATION_THRESHOLD: Value = 20.0;

fn finite_difference_with_one_sided_fallback(
    f0: Value,
    fp: Value,
    fm: Value,
    step: Value,
) -> Option<Value> {
    let central = (fp.is_finite() && fm.is_finite())
        .then(|| (fp - fm) / (2.0 * step))
        .filter(|derivative| derivative.is_finite());
    let forward = (fp.is_finite() && f0.is_finite())
        .then(|| (fp - f0) / step)
        .filter(|derivative| derivative.is_finite());
    let backward = (fm.is_finite() && f0.is_finite())
        .then(|| (f0 - fm) / step)
        .filter(|derivative| derivative.is_finite());
    central.or(forward).or(backward)
}

/// Result of resolving an `I(device)` operand in a behavioral expression.
///
/// A device can exist without owning an MNA branch-current solution variable.
/// Keeping that state distinct from an absent device prevents invalid lead
/// currents from being misreported as misspelled instance names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BehavioralBranchResolution {
    Branch(usize),
    DeviceWithoutBranch,
    MissingDevice,
}

/// Stable reason why a behavioral expression reference could not be bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BehavioralReferenceReason {
    LeadCurrentNotSolutionVariable,
    UnknownDevice,
    UnknownNode,
}

impl BehavioralReferenceReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LeadCurrentNotSolutionVariable => "lead_current_not_solution_variable",
            Self::UnknownDevice => "unknown_device",
            Self::UnknownNode => "unknown_node",
        }
    }
}

impl std::fmt::Display for BehavioralReferenceReason {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A node or `I(device)` operand in a behavioral expression could not be bound.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error(
    "Device instance {canonical_owner_name}: Problem with value for {canonical_dependency_name} in {canonical_owner_name} ({reason})"
)]
pub struct BehavioralReferenceError {
    pub owner_name: String,
    pub canonical_owner_name: String,
    pub dependency_name: String,
    pub canonical_dependency_name: String,
    pub reason: BehavioralReferenceReason,
}

/// A behavioral source could not be evaluated or stamped without losing a
/// finite physical value.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum BehavioralEvaluationError {
    #[error(
        "behavioral {source_kind} source '{source_name}' produced non-finite {quantity} at time {time:.17e} s and frequency {frequency:.17e} Hz: {value}"
    )]
    NonFinite {
        source_kind: &'static str,
        source_name: String,
        quantity: String,
        time: Value,
        frequency: Value,
        value: Value,
    },
    #[error(
        "behavioral {source_kind} source '{source_name}' stamp failed at time {time:.17e} s and frequency {frequency:.17e} Hz: {detail}"
    )]
    Stamp {
        source_kind: &'static str,
        source_name: String,
        time: Value,
        frequency: Value,
        detail: String,
    },
}

/// One matrix entry a behavioral source contributes: where it lands and how
/// much it adds.
#[derive(Clone, Copy)]
struct BehavioralMatrixEntry {
    row: usize,
    column: usize,
    coefficient: Value,
}

/// The point a behavioral expression is evaluated at, beyond its own operands:
/// the analysis coordinate, the ambient temperature, the conductance floor the
/// solver adds, and the dialect whose function set and rounding apply.
#[derive(Clone, Copy)]
pub(crate) struct BehavioralEnvironment {
    pub(crate) time: Value,
    pub(crate) frequency: Value,
    pub(crate) temperature: Value,
    pub(crate) gmin: Value,
    pub(crate) expression_dialect: ExpressionDialect,
}

fn try_stamp_behavioral_matrix_coefficient(
    matrix: &mut StaticMatrix,
    entry: BehavioralMatrixEntry,
    source_kind: &'static str,
    source_name: &str,
    time: Value,
    frequency: Value,
) -> Result<(), BehavioralEvaluationError> {
    let BehavioralMatrixEntry {
        row,
        column,
        coefficient,
    } = entry;
    matrix
        .try_add(row, column, coefficient)
        .map_err(|error| BehavioralEvaluationError::Stamp {
            source_kind,
            source_name: source_name.to_string(),
            time,
            frequency,
            detail: format!(
                "matrix coefficient {coefficient} at row {row}, column {column}: {error}"
            ),
        })
}

impl BehavioralReferenceError {
    fn new(owner_name: &str, dependency_name: &str, reason: BehavioralReferenceReason) -> Self {
        Self {
            owner_name: owner_name.to_string(),
            canonical_owner_name: owner_name.to_ascii_uppercase(),
            dependency_name: dependency_name.to_string(),
            canonical_dependency_name: dependency_name.to_ascii_uppercase(),
            reason,
        }
    }
}

#[derive(Clone, Copy)]
enum DerivativeTarget<'a> {
    Node(usize),
    Branch(usize),
    NodeDirection(&'a [Derivative]),
    Time,
}

struct BehavioralDerivativeContext<'a> {
    program: &'a CompiledExpr,
    node_values: &'a [Value],
    branch_values: &'a [Value],
    time: Value,
    frequency: Value,
    temperature: Value,
    gmin: Value,
    expression_dialect: ExpressionDialect,
    target: DerivativeTarget<'a>,
}

/// Compiled behavioral voltage source
#[derive(Debug, Clone)]
pub struct BehavioralVoltageSource {
    /// Device name
    pub name: String,
    /// Positive node
    pub node_pos: usize,
    /// Negative node  
    pub node_neg: usize,
    /// Branch ordinal for MNA (1-based, converted to matrix index at stamp time)
    pub branch_ordinal: usize,
    /// Parsed expression used for structural analysis such as breakpoint extraction.
    ast: Expr,
    /// Compiled expression
    pub program: CompiledExpr,
    /// VM for evaluation
    vm: Vm,
    /// Compiled-expression node references mapped to circuit solution indices
    node_bindings: Vec<Option<usize>>,
    /// Compiled-expression branch references mapped to circuit solution indices
    branch_bindings: Vec<Option<usize>>,
    /// Reused scratch storage for expression node values
    node_values: Vec<Value>,
    /// Reused scratch storage for expression branch-current values
    branch_values: Vec<Value>,
    /// Linearization partials d(expr)/d(node_values`[idx]`)
    node_partials: Vec<Value>,
    /// Linearization partials d(expr)/d(branch_values`[idx]`)
    branch_partials: Vec<Value>,
    /// Affine term for the most recent expression linearization.
    linearized_affine: Value,
    /// Exact expression value and analysis time from the latest finite linearization.
    ///
    /// Transient correction-form solves may reconstruct an ideal voltage as
    /// `predictor + (target - predictor)`, losing low bits to cancellation.
    /// Retaining the expression result lets the circuit re-project a
    /// solution-independent voltage source without evaluating its VM twice.
    cached_exact_constraint: Option<(Value, Value)>,
    /// Circuit temperature in degrees Celsius, surfaced as `temper`.
    temperature: Value,
    /// Active analysis frequency in hertz.
    frequency: Value,
    /// Whether the resolved expression contains the live AC frequency.
    frequency_dependent: bool,
    /// Active nonlinear minimum conductance, surfaced as `GMIN`.
    gmin: Value,
    /// Dialect-specific expression-function semantics.
    expression_dialect: ExpressionDialect,
    /// True when the expression can jump discontinuously as its inputs cross a predicate.
    transient_voltage_lte_excluded: bool,
}

impl BehavioralVoltageSource {
    pub(crate) fn explicit_time_derivative(&self, time: Value) -> Option<Value> {
        if self.is_solution_dependent() {
            return None;
        }
        let context = BehavioralDerivativeContext {
            program: &self.program,
            node_values: &[],
            branch_values: &[],
            time,
            frequency: self.frequency,
            temperature: self.temperature,
            gmin: self.gmin,
            expression_dialect: self.expression_dialect,
            target: DerivativeTarget::Time,
        };
        let (outgoing, derivative) = eval_behavioral_expr_with_derivative(&self.ast, &context)?;
        let derivative = derivative.binary64();
        let point = Vm::new().execute(
            &self.program,
            &Context::transient(&[], &[], time)
                .with_temperature(self.temperature)
                .with_frequency(self.frequency)
                .with_gmin(self.gmin)
                .with_expression_dialect(self.expression_dialect)
                .with_ieee_logarithm(),
        );
        // A one-sided expression branch may have a finite slope after a jump.
        // It is not a finite displacement current at the published value.
        let scale = point.abs().max(outgoing.abs());
        (derivative.is_finite()
            && point.is_finite()
            && outgoing.is_finite()
            && (point == outgoing || (point - outgoing).abs() <= 8.0 * Value::EPSILON * scale))
            .then_some(derivative)
    }

    fn nonfinite_error(
        &self,
        quantity: impl Into<String>,
        time: Value,
        value: Value,
    ) -> BehavioralEvaluationError {
        BehavioralEvaluationError::NonFinite {
            source_kind: "voltage",
            source_name: self.name.clone(),
            quantity: quantity.into(),
            time,
            frequency: self.frequency,
            value,
        }
    }

    fn stamp_error(&self, time: Value, detail: impl Into<String>) -> BehavioralEvaluationError {
        BehavioralEvaluationError::Stamp {
            source_kind: "voltage",
            source_name: self.name.clone(),
            time,
            frequency: self.frequency,
            detail: detail.into(),
        }
    }

    /// Create a new behavioral voltage source
    pub fn new(
        name: String,
        node_pos: usize,
        node_neg: usize,
        branch_ordinal: usize,
        expression: &str,
    ) -> Result<Self, String> {
        Self::new_with_source_path(name, node_pos, node_neg, branch_ordinal, expression, None)
    }

    /// Create a new behavioral voltage source with deck-relative file-function support.
    pub fn new_with_source_path(
        name: String,
        node_pos: usize,
        node_neg: usize,
        branch_ordinal: usize,
        expression: &str,
        source_path: Option<&Path>,
    ) -> Result<Self, String> {
        Self::new_with_source_path_and_limits(
            name,
            node_pos,
            node_neg,
            branch_ordinal,
            expression,
            source_path,
            crate::resource::ResourceLimits::default(),
        )
    }

    /// Create a behavioral voltage source with file lookups governed by an explicit policy.
    pub fn new_with_source_path_and_limits(
        name: String,
        node_pos: usize,
        node_neg: usize,
        branch_ordinal: usize,
        expression: &str,
        source_path: Option<&Path>,
        resource_limits: crate::resource::ResourceLimits,
    ) -> Result<Self, String> {
        let ast = parse_expression_strict(expression)
            .map_err(|e| format!("Invalid behavioral expression '{}': {}", expression, e))?;
        let ast = resolve_file_lookup_functions_with_limits(ast, source_path, resource_limits)
            .map_err(|e| format!("Invalid behavioral expression '{}': {}", expression, e))?;
        let transient_voltage_lte_excluded =
            expression_excludes_voltage_output_from_transient_lte(&ast);
        let frequency_dependent = expression_depends_on_frequency(&ast);
        let program = compile(&ast);

        Ok(Self {
            name,
            node_pos,
            node_neg,
            branch_ordinal,
            ast,
            program,
            vm: Vm::new(),
            node_bindings: Vec::new(),
            branch_bindings: Vec::new(),
            node_values: Vec::new(),
            branch_values: Vec::new(),
            node_partials: Vec::new(),
            branch_partials: Vec::new(),
            linearized_affine: 0.0,
            cached_exact_constraint: None,
            temperature: crate::constants::kelvin_to_celsius(crate::constants::TEMP_REFERENCE),
            frequency: 0.0,
            frequency_dependent,
            gmin: crate::constants::GMIN,
            expression_dialect: ExpressionDialect::Ngspice,
            transient_voltage_lte_excluded,
        })
    }

    /// Resolve V(...) and I(...) references against circuit node/branch indices.
    pub fn bind_references<FN, FB>(
        &mut self,
        resolve_node: FN,
        resolve_branch: FB,
    ) -> Result<(), BehavioralReferenceError>
    where
        FN: Fn(&str) -> Option<usize>,
        FB: Fn(&str) -> BehavioralBranchResolution,
    {
        self.invalidate_cached_exact_constraint();
        self.node_bindings = vec![None; self.program.node_map.len()];
        for (name, &local_idx) in &self.program.node_map {
            let resolved = if crate::naming::is_spice_ground_name(name) {
                Some(0usize)
            } else {
                resolve_node(name)
            }
            .ok_or_else(|| {
                BehavioralReferenceError::new(
                    &self.name,
                    name,
                    BehavioralReferenceReason::UnknownNode,
                )
            })?;
            self.node_bindings[local_idx] = resolved.checked_sub(1);
        }

        self.branch_bindings = vec![None; self.program.branch_map.len()];
        for (name, &local_idx) in &self.program.branch_map {
            let resolved = match resolve_branch(name) {
                BehavioralBranchResolution::Branch(index) => index,
                BehavioralBranchResolution::DeviceWithoutBranch => {
                    return Err(BehavioralReferenceError::new(
                        &self.name,
                        name,
                        BehavioralReferenceReason::LeadCurrentNotSolutionVariable,
                    ));
                }
                BehavioralBranchResolution::MissingDevice => {
                    return Err(BehavioralReferenceError::new(
                        &self.name,
                        name,
                        BehavioralReferenceReason::UnknownDevice,
                    ));
                }
            };
            self.branch_bindings[local_idx] = Some(resolved);
        }

        self.node_values.resize(self.node_bindings.len(), 0.0);
        self.branch_values.resize(self.branch_bindings.len(), 0.0);
        self.node_partials.resize(self.node_bindings.len(), 0.0);
        self.branch_partials.resize(self.branch_bindings.len(), 0.0);
        Ok(())
    }

    #[inline]
    fn refresh_expression_inputs(&mut self, solution: &[Value]) {
        for (idx, binding) in self.node_bindings.iter().enumerate() {
            self.node_values[idx] = binding
                .and_then(|global_idx| solution.get(global_idx).copied())
                .unwrap_or(0.0);
        }
        for (idx, binding) in self.branch_bindings.iter().enumerate() {
            self.branch_values[idx] = binding
                .and_then(|global_idx| solution.get(global_idx).copied())
                .unwrap_or(0.0);
        }
    }

    /// Evaluate the expression with current circuit solution.
    pub fn evaluate(
        &mut self,
        solution: &[Value],
        time: Value,
    ) -> Result<Value, BehavioralEvaluationError> {
        self.refresh_expression_inputs(solution);
        let value = self.evaluate_with_cached_inputs(time);
        if !value.is_finite() {
            self.invalidate_cached_exact_constraint();
            return Err(self.nonfinite_error("expression value", time, value));
        }
        Ok(value)
    }

    /// Evaluate an owned VM candidate without changing accepted operator state.
    /// The compiled expression and binding tables remain shared with this source.
    fn prepare_transient_step(
        &mut self,
        solution: &[Value],
        time: Value,
    ) -> Result<Vm, BehavioralEvaluationError> {
        self.refresh_expression_inputs(solution);
        let context = Context::transient(&self.node_values, &self.branch_values, time)
            .with_frequency(self.frequency)
            .with_temperature(self.temperature)
            .with_gmin(self.gmin)
            .with_expression_dialect(self.expression_dialect)
            .with_ieee_logarithm();
        let mut candidate = self.vm.clone();
        let value = candidate.execute(&self.program, &context);
        if !value.is_finite() {
            return Err(self.nonfinite_error("expression value", time, value));
        }
        candidate.accept_transient_step(time);
        Ok(candidate)
    }

    #[inline]
    pub(crate) fn bound_solution_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.node_bindings
            .iter()
            .chain(self.branch_bindings.iter())
            .filter_map(|binding| *binding)
    }

    #[inline]
    pub(crate) fn excludes_output_from_transient_voltage_lte(&self) -> bool {
        self.transient_voltage_lte_excluded
    }

    pub(crate) fn is_solution_dependent(&self) -> bool {
        !self.program.node_map.is_empty() || !self.program.branch_map.is_empty()
    }

    #[inline]
    fn invalidate_cached_exact_constraint(&mut self) {
        self.cached_exact_constraint = None;
    }

    /// Return the exact voltage constraint cached by a finite linearization at
    /// this exact transient time. Solution-dependent expressions remain Newton
    /// equations and must never be projected as prescribed voltages.
    pub(crate) fn cached_exact_constraint_at(&self, time: Value) -> Option<Value> {
        if self.is_solution_dependent() {
            return None;
        }
        self.cached_exact_constraint
            .filter(|(cached_time, _)| cached_time.to_bits() == time.to_bits())
            .map(|(_, value)| value)
    }

    #[inline]
    fn evaluate_with_cached_inputs(&mut self, time: Value) -> Value {
        let ctx = Context::transient(&self.node_values, &self.branch_values, time)
            .with_frequency(self.frequency)
            .with_temperature(self.temperature)
            .with_gmin(self.gmin)
            .with_expression_dialect(self.expression_dialect)
            .with_ieee_logarithm();
        self.vm.execute(&self.program, &ctx)
    }

    /// Set the circuit temperature (degrees Celsius) surfaced as `temper`.
    pub fn set_temperature(&mut self, temperature: Value) {
        self.invalidate_cached_exact_constraint();
        self.temperature = temperature;
    }

    pub fn set_frequency(&mut self, frequency: Value) {
        self.invalidate_cached_exact_constraint();
        self.frequency = frequency;
    }

    /// Whether this source's resolved expression depends on `FREQ`/`HERTZ`.
    #[inline]
    pub(crate) fn is_frequency_dependent(&self) -> bool {
        self.frequency_dependent
    }

    pub fn set_gmin(&mut self, gmin: Value) {
        self.invalidate_cached_exact_constraint();
        self.gmin = gmin;
    }

    /// Set dialect-specific expression-function semantics.
    pub fn set_expression_dialect(&mut self, dialect: ExpressionDialect) {
        self.invalidate_cached_exact_constraint();
        self.expression_dialect = dialect;
    }

    #[inline]
    fn derivative_step(base: Value) -> Value {
        DERIVATIVE_ABS_STEP + DERIVATIVE_REL_STEP * base.abs().max(1.0)
    }

    #[inline]
    fn estimate_node_partial(
        &mut self,
        idx: usize,
        f0: Value,
        time: Value,
    ) -> Result<Value, BehavioralEvaluationError> {
        let base = self.node_values[idx];
        let h = Self::derivative_step(base);
        self.node_values[idx] = base + h;
        let fp = self.evaluate_with_cached_inputs(time);
        self.node_values[idx] = base - h;
        let fm = self.evaluate_with_cached_inputs(time);
        self.node_values[idx] = base;

        let df = finite_difference_with_one_sided_fallback(f0, fp, fm, h).ok_or_else(|| {
            self.nonfinite_error(
                format!("node derivative {idx} (no finite difference exists)"),
                time,
                Value::NAN,
            )
        })?;
        Ok(df)
    }

    #[inline]
    fn estimate_branch_partial(
        &mut self,
        idx: usize,
        f0: Value,
        time: Value,
    ) -> Result<Value, BehavioralEvaluationError> {
        let base = self.branch_values[idx];
        let h = Self::derivative_step(base);
        self.branch_values[idx] = base + h;
        let fp = self.evaluate_with_cached_inputs(time);
        self.branch_values[idx] = base - h;
        let fm = self.evaluate_with_cached_inputs(time);
        self.branch_values[idx] = base;

        let df = finite_difference_with_one_sided_fallback(f0, fp, fm, h).ok_or_else(|| {
            self.nonfinite_error(
                format!("branch-current derivative {idx} (no finite difference exists)"),
                time,
                Value::NAN,
            )
        })?;
        Ok(df)
    }

    fn linearize_expression(
        &mut self,
        solution: &[Value],
        time: Value,
    ) -> Result<Value, BehavioralEvaluationError> {
        self.invalidate_cached_exact_constraint();
        self.refresh_expression_inputs(solution);
        let f0 = self.evaluate_with_cached_inputs(time);

        if !f0.is_finite() {
            self.node_partials.fill(0.0);
            self.branch_partials.fill(0.0);
            self.linearized_affine = 0.0;
            return Err(self.nonfinite_error("expression value", time, f0));
        }

        for idx in 0..self.node_bindings.len() {
            self.node_partials[idx] = if self.node_bindings[idx].is_some() {
                analytic_expression_partial(
                    &self.ast,
                    &self.program,
                    &self.node_values,
                    &self.branch_values,
                    BehavioralEnvironment {
                        time,
                        frequency: self.frequency,
                        temperature: self.temperature,
                        gmin: self.gmin,
                        expression_dialect: self.expression_dialect,
                    },
                    DerivativeTarget::Node(idx),
                )
                .map(Ok)
                .unwrap_or_else(|| self.estimate_node_partial(idx, f0, time))?
            } else {
                0.0
            };
        }
        for idx in 0..self.branch_bindings.len() {
            self.branch_partials[idx] = if self.branch_bindings[idx].is_some() {
                analytic_expression_partial(
                    &self.ast,
                    &self.program,
                    &self.node_values,
                    &self.branch_values,
                    BehavioralEnvironment {
                        time,
                        frequency: self.frequency,
                        temperature: self.temperature,
                        gmin: self.gmin,
                        expression_dialect: self.expression_dialect,
                    },
                    DerivativeTarget::Branch(idx),
                )
                .map(Ok)
                .unwrap_or_else(|| self.estimate_branch_partial(idx, f0, time))?
            } else {
                0.0
            };
        }

        let mut affine = f0;
        for (idx, binding) in self.node_bindings.iter().enumerate() {
            if let Some(global_idx) = binding {
                let term = self.node_partials[idx] * solution[*global_idx];
                if !term.is_finite() {
                    return Err(self.nonfinite_error(
                        format!("node affine product {idx}"),
                        time,
                        term,
                    ));
                }
                affine -= term;
                if !affine.is_finite() {
                    return Err(self.nonfinite_error("affine linearization", time, affine));
                }
            }
        }
        for (idx, binding) in self.branch_bindings.iter().enumerate() {
            if let Some(global_idx) = binding {
                let term = self.branch_partials[idx] * solution[*global_idx];
                if !term.is_finite() {
                    return Err(self.nonfinite_error(
                        format!("branch-current affine product {idx}"),
                        time,
                        term,
                    ));
                }
                affine -= term;
                if !affine.is_finite() {
                    return Err(self.nonfinite_error("affine linearization", time, affine));
                }
            }
        }
        self.cached_exact_constraint = Some((time, f0));
        self.linearized_affine = affine;
        Ok(affine)
    }

    /// Refresh the linearization (value and partials) at the given
    /// operating point for small-signal assembly. AC has no time axis;
    /// expressions see t = 0.
    pub(crate) fn linearize_at(
        &mut self,
        solution: &[Value],
    ) -> Result<(), BehavioralEvaluationError> {
        self.set_frequency(0.0);
        self.linearize_expression(solution, 0.0)?;
        Ok(())
    }

    pub(crate) fn linearize_at_frequency(
        &mut self,
        solution: &[Value],
        frequency: Value,
    ) -> Result<(), BehavioralEvaluationError> {
        if !self.frequency_dependent {
            return Ok(());
        }
        self.set_frequency(frequency);
        self.linearize_expression(solution, 0.0)?;
        Ok(())
    }

    /// Linearize at an arbitrary state and frequency. Unlike
    /// [`Self::linearize_at_frequency`], this always refreshes the Jacobian
    /// because the supplied state may have changed.
    pub(crate) fn linearize_at_state_and_frequency(
        &mut self,
        solution: &[Value],
        frequency: Value,
    ) -> Result<(), BehavioralEvaluationError> {
        self.set_frequency(frequency);
        self.linearize_expression(solution, 0.0)?;
        Ok(())
    }

    /// Visit the cached linearized partials as `(solution_index, df/dx)`
    /// pairs. Valid after `linearize_at` (or any stamp call).
    pub(crate) fn linearized_partials(&self) -> impl Iterator<Item = (usize, Value)> + '_ {
        self.node_bindings
            .iter()
            .zip(&self.node_partials)
            .chain(self.branch_bindings.iter().zip(&self.branch_partials))
            .filter_map(|(binding, df)| binding.map(|idx| (idx, *df)))
    }

    fn linearized_expression_value(&self, solution: &[Value]) -> Value {
        let node_value = self
            .node_bindings
            .iter()
            .zip(&self.node_partials)
            .filter_map(|(binding, df)| {
                binding.and_then(|idx| solution.get(idx).map(|value| *df * *value))
            })
            .sum::<Value>();
        let branch_value = self
            .branch_bindings
            .iter()
            .zip(&self.branch_partials)
            .filter_map(|(binding, df)| {
                binding.and_then(|idx| solution.get(idx).map(|value| *df * *value))
            })
            .sum::<Value>();
        self.linearized_affine + node_value + branch_value
    }

    fn linearization_converged(
        &mut self,
        solution: &[Value],
        time: Value,
        reltol: Value,
        abstol: Value,
    ) -> Result<bool, BehavioralEvaluationError> {
        let actual = self.evaluate(solution, time)?;
        let linearized = self.linearized_expression_value(solution);
        if !linearized.is_finite() {
            return Err(self.nonfinite_error("cached linearized value", time, linearized));
        }
        Ok(linearization_values_converged(
            actual, linearized, reltol, abstol,
        ))
    }

    /// Stamp into the matrix (MNA voltage source with computed value)
    pub fn stamp(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        num_nodes: usize,
        time: Value,
    ) -> Result<(), BehavioralEvaluationError> {
        let v_affine = self.linearize_expression(solution, time)?;
        let br = num_nodes + self.branch_ordinal;
        let np = self.node_pos;
        let nn = self.node_neg;

        // Standard voltage source MNA stamping
        // Branch equation: V(n+) - V(n-) = v_value
        if np > 0 {
            try_stamp_behavioral_matrix_coefficient(
                matrix,
                BehavioralMatrixEntry {
                    row: br - 1,
                    column: np - 1,
                    coefficient: 1.0,
                },
                "voltage",
                &self.name,
                time,
                self.frequency,
            )?;
            try_stamp_behavioral_matrix_coefficient(
                matrix,
                BehavioralMatrixEntry {
                    row: np - 1,
                    column: br - 1,
                    coefficient: 1.0,
                },
                "voltage",
                &self.name,
                time,
                self.frequency,
            )?;
        }
        if nn > 0 {
            try_stamp_behavioral_matrix_coefficient(
                matrix,
                BehavioralMatrixEntry {
                    row: br - 1,
                    column: nn - 1,
                    coefficient: -1.0,
                },
                "voltage",
                &self.name,
                time,
                self.frequency,
            )?;
            try_stamp_behavioral_matrix_coefficient(
                matrix,
                BehavioralMatrixEntry {
                    row: nn - 1,
                    column: br - 1,
                    coefficient: -1.0,
                },
                "voltage",
                &self.name,
                time,
                self.frequency,
            )?;
        }

        // Linearized behavioral dependency terms on branch equation row:
        // V(np)-V(nn)-f(x) = 0 => row(br): ... + (-df/dx)*x = affine
        for (idx, binding) in self.node_bindings.iter().enumerate() {
            if let Some(global_idx) = binding {
                let df = self.node_partials[idx];
                if df != 0.0 {
                    try_stamp_behavioral_matrix_coefficient(
                        matrix,
                        BehavioralMatrixEntry {
                            row: br - 1,
                            column: *global_idx,
                            coefficient: -df,
                        },
                        "voltage",
                        &self.name,
                        time,
                        self.frequency,
                    )?;
                }
            }
        }
        for (idx, binding) in self.branch_bindings.iter().enumerate() {
            if let Some(global_idx) = binding {
                let df = self.branch_partials[idx];
                if df != 0.0 {
                    try_stamp_behavioral_matrix_coefficient(
                        matrix,
                        BehavioralMatrixEntry {
                            row: br - 1,
                            column: *global_idx,
                            coefficient: -df,
                        },
                        "voltage",
                        &self.name,
                        time,
                        self.frequency,
                    )?;
                }
            }
        }

        // RHS: branch equation
        let rhs_slot = rhs.get_mut(br - 1).ok_or_else(|| {
            self.stamp_error(
                time,
                format!("RHS row {} is outside the MNA system", br - 1),
            )
        })?;
        *rhs_slot = v_affine;
        Ok(())
    }
}

fn expression_excludes_voltage_output_from_transient_lte(expr: &Expr) -> bool {
    // Ideal voltage-source outputs are algebraic constraints. For source-imposed
    // time waveforms, breakpoint scheduling and connected dynamic states should
    // control accuracy; generic node-voltage LTE can otherwise collapse dt at
    // startup while chasing the source value itself.
    match expr {
        Expr::Const(_)
        | Expr::NodeVoltage(_)
        | Expr::BranchCurrent(_)
        | Expr::StringLiteral(_)
        | Expr::Frequency
        | Expr::Temperature
        | Expr::ThermalVoltage
        | Expr::Gmin => false,
        Expr::Time => true,
        Expr::LookupTable { input, table } => {
            table.transient_breakpoints
                && expression_excludes_voltage_output_from_transient_lte(input)
        }
        Expr::Unary { op, operand } => {
            matches!(op, UnaryOp::Not)
                || expression_excludes_voltage_output_from_transient_lte(operand)
        }
        Expr::Binary { op, left, right } => {
            matches!(
                op,
                BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge
                    | BinaryOp::Eq
                    | BinaryOp::Ne
                    | BinaryOp::And
                    | BinaryOp::Or
            ) || expression_excludes_voltage_output_from_transient_lte(left)
                || expression_excludes_voltage_output_from_transient_lte(right)
        }
        Expr::Function { func, args } => {
            matches!(
                func,
                Function::Trunc
                    | Function::Floor
                    | Function::Ceil
                    | Function::Round
                    | Function::Sign
                    | Function::HspiceSign
                    | Function::Stp
                    | Function::Ustep
                    | Function::Eq0
                    | Function::Ne0
                    | Function::Gt0
                    | Function::Lt0
                    | Function::Ge0
                    | Function::Le0
                    | Function::If
            ) || args
                .iter()
                .any(expression_excludes_voltage_output_from_transient_lte)
        }
    }
}

/// Detect the live frequency after parameter/function expansion has produced
/// the canonical behavioral AST. Probe names remain ordinary strings in
/// `NodeVoltage`/`BranchCurrent`, so a node named `FREQ` is not misclassified.
fn expression_depends_on_frequency(expr: &Expr) -> bool {
    match expr {
        Expr::Frequency => true,
        Expr::Unary { operand, .. } => expression_depends_on_frequency(operand),
        Expr::Binary { left, right, .. } => {
            expression_depends_on_frequency(left) || expression_depends_on_frequency(right)
        }
        Expr::Function { args, .. } => args.iter().any(expression_depends_on_frequency),
        Expr::Const(_)
        | Expr::NodeVoltage(_)
        | Expr::BranchCurrent(_)
        | Expr::StringLiteral(_)
        | Expr::Time
        | Expr::Temperature
        | Expr::ThermalVoltage
        | Expr::Gmin => false,
        Expr::LookupTable { input, .. } => expression_depends_on_frequency(input),
    }
}

fn analytic_expression_partial(
    expr: &Expr,
    program: &CompiledExpr,
    node_values: &[Value],
    branch_values: &[Value],
    environment: BehavioralEnvironment,
    target: DerivativeTarget<'_>,
) -> Option<Value> {
    let BehavioralEnvironment {
        time,
        frequency,
        temperature,
        gmin,
        expression_dialect,
    } = environment;
    let context = BehavioralDerivativeContext {
        program,
        node_values,
        branch_values,
        time,
        frequency,
        temperature,
        gmin,
        expression_dialect,
        target,
    };
    let (_, derivative) = eval_behavioral_expr_with_derivative_at_boundary(expr, &context)?;
    derivative.is_finite().then_some(derivative)
}

/// Evaluate the analytic derivative of a compiled behavioral expression with
/// respect to one expression-local node voltage. Passive behavioral-value
/// evaluators use this shared implementation so their Jacobian semantics stay
/// aligned with B-sources; callers provide a finite-difference fallback when
/// an expression operator has no analytic derivative.
pub(crate) fn compiled_expression_node_partial(
    expr: &Expr,
    program: &CompiledExpr,
    node_values: &[Value],
    branch_values: &[Value],
    environment: BehavioralEnvironment,
    node_index: usize,
) -> Option<Value> {
    let BehavioralEnvironment {
        time,
        frequency,
        temperature,
        gmin,
        expression_dialect,
    } = environment;
    analytic_expression_partial(
        expr,
        program,
        node_values,
        branch_values,
        BehavioralEnvironment {
            time,
            frequency,
            temperature,
            gmin,
            expression_dialect,
        },
        DerivativeTarget::Node(node_index),
    )
}

/// Evaluate the analytic derivative of a compiled behavioral expression with
/// respect to one expression-local branch current. See
/// [`compiled_expression_node_partial`] for the shared derivative contract.
pub(crate) fn compiled_expression_branch_partial(
    expr: &Expr,
    program: &CompiledExpr,
    node_values: &[Value],
    branch_values: &[Value],
    environment: BehavioralEnvironment,
    branch_index: usize,
) -> Option<Value> {
    let BehavioralEnvironment {
        time,
        frequency,
        temperature,
        gmin,
        expression_dialect,
    } = environment;
    analytic_expression_partial(
        expr,
        program,
        node_values,
        branch_values,
        BehavioralEnvironment {
            time,
            frequency,
            temperature,
            gmin,
            expression_dialect,
        },
        DerivativeTarget::Branch(branch_index),
    )
}

/// Propagate a caller-supplied direction through expression-local scalar
/// leaves. Keep the derivative exponent until the owning consumer converts it.
pub(crate) fn compiled_expression_node_direction(
    expr: &Expr,
    program: &CompiledExpr,
    node_values: &[Value],
    node_directions: &[Derivative],
    environment: BehavioralEnvironment,
) -> Option<(Value, Derivative)> {
    if node_values.len() != program.node_map.len()
        || node_directions.len() != node_values.len()
        || !program.branch_map.is_empty()
    {
        return None;
    }
    eval_behavioral_expr_with_derivative(
        expr,
        &BehavioralDerivativeContext {
            program,
            node_values,
            branch_values: &[],
            time: environment.time,
            frequency: environment.frequency,
            temperature: environment.temperature,
            gmin: environment.gmin,
            expression_dialect: environment.expression_dialect,
            target: DerivativeTarget::NodeDirection(node_directions),
        },
    )
}

/// Evaluate the exact directional derivative of a real expression from
/// caller-supplied scalar leaves, including scoped user-defined functions.
///
/// Every entry in `derivative_targets` must be an alias for the same physical
/// scalar and therefore have the same numeric value in `parameters`. This is
/// the semantic primitive used by output-domain `DDX`: Xyce differentiates
/// the expression AST directly, so numerical differencing is neither precise
/// enough nor correct at branch points.
pub fn evaluate_parameter_directional_derivative(
    expression: &str,
    parameters: &crate::netlist::expr::ParamContext,
    derivative_targets: &[String],
) -> Result<Value, String> {
    let derivative_targets = derivative_targets
        .iter()
        .map(|target| target.to_ascii_uppercase())
        .collect::<std::collections::BTreeSet<_>>();
    let parsed =
        crate::netlist::expr::parse_expression(expression).map_err(|error| error.to_string())?;
    if derivative_targets.is_empty() {
        return Err("DDX has no derivative target".to_string());
    }
    let mut program = crate::netlist::expr::PreparedExpression::compile(&parsed, parameters)
        .map_err(|error| error.to_string())?;
    let representative = derivative_targets
        .iter()
        .next()
        .expect("non-empty target set");
    let real_target = |name: &str| {
        let value = parameters
            .get_complex(name)
            .ok_or_else(|| format!("DDX derivative target '{name}' has no numeric value"))?;
        if !crate::netlist::expr::is_real(value) {
            return Err(format!("DDX derivative target '{name}' is not real-valued"));
        }
        Ok(value.re)
    };
    let target_value = real_target(representative)?;
    for alias in derivative_targets.iter().skip(1) {
        if real_target(alias)?.to_bits() != target_value.to_bits() {
            return Err(format!(
                "DDX derivative aliases '{representative}' and '{alias}' have different values"
            ));
        }
    }
    let mut target_present = false;
    program.visit_runtime_parameters(|name| {
        target_present |= derivative_targets.contains(name);
    });
    if !target_present {
        return Err(format!(
            "directional output derivative target '{representative}' is absent from the expression"
        ));
    }
    let (_, derivative) = program
        .evaluate_scalar_direction_with(parameters, &mut |name| {
            Ok(derivative_targets
                .contains(name)
                .then_some((target_value.into(), 1.0.into())))
        })
        .map_err(|error| error.to_string())?;
    Ok(normalize_expression_boundary(
        derivative.binary64(),
        parameters.expression_dialect(),
    ))
}

/// The argument a `B` source's logarithm is evaluated at, and its slope.
///
/// A behavioral expression is a circuit equation, so this evaluator only ever
/// runs at a point a Newton loop offered. The guard against `ln(0)` stays —
/// zero is the boundary of the domain and a finite stand-in there costs
/// nothing — but a strictly negative argument is outside the domain, and
/// clamping it is what let the R1.14 deck `B1 a 0 I={ln(v(a)+0.1)}` report
/// V(a) = -4.999995 as an operating point: the clamp fabricates
/// `ln(1e-38) = -87.5` for the value and `1/1e-38 = 1e38` for the slope, and a
/// 1e38 conductance pins the node so hard that the next Newton update is zero
/// and the `vntol` check calls it converged. NaN here is what Spectre
/// evaluates, and R1.14's `StampError::NonFiniteTrial` is what turns it into a
/// rejected iterate rather than a refused run.
///
/// `expr::LogarithmDomain::Ieee` is the same rule for the bytecode VM; the two
/// must agree, because a `B` source's value and its Jacobian entry come from
/// these two evaluators respectively.
#[inline]
fn behavioral_logarithm_argument(value: Value) -> Value {
    if value < 0.0 {
        Value::NAN
    } else {
        value.max(crate::expr::LOGARITHM_MIN_ARGUMENT)
    }
}

fn eval_behavioral_expr_with_derivative_at_boundary(
    expr: &Expr,
    context: &BehavioralDerivativeContext<'_>,
) -> Option<(Value, Value)> {
    let (value, derivative) = eval_behavioral_expr_with_derivative(expr, context)?;
    Some((
        normalize_expression_boundary(value, context.expression_dialect),
        normalize_expression_boundary(derivative.binary64(), context.expression_dialect),
    ))
}

fn eval_behavioral_expr_with_derivative(
    expr: &Expr,
    context: &BehavioralDerivativeContext<'_>,
) -> Option<(Value, Derivative)> {
    if matches!(context.target, DerivativeTarget::Time) && !crate::expr::constant_over_time(expr) {
        // Newton's regularized slopes and pointwise Boolean/table derivatives
        // are not necessarily outgoing time derivatives. Only use the rules
        // qualified below for physical displacement current; unsupported
        // corners must not silently publish a zero or regularized current.
        let qualified = match expr {
            Expr::Time
            | Expr::Unary {
                op: UnaryOp::Neg, ..
            } => true,
            Expr::Binary {
                op:
                    BinaryOp::Add
                    | BinaryOp::Sub
                    | BinaryOp::Mul
                    | BinaryOp::Div
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge,
                ..
            } => true,
            Expr::Binary {
                op: BinaryOp::Pow,
                right,
                ..
            } => matches!(right.as_ref(),
                Expr::Const(value) if value.is_finite() && *value >= 0.0 && value.fract() == 0.0),
            Expr::Function {
                func:
                    Function::Abs
                    | Function::Exp
                    | Function::Sin
                    | Function::Cos
                    | Function::Tan
                    | Function::Atan
                    | Function::Sinh
                    | Function::Cosh
                    | Function::Tanh
                    | Function::Asinh
                    | Function::Sqr
                    | Function::Min
                    | Function::Max
                    | Function::Uramp
                    | Function::If,
                ..
            } => true,
            Expr::Function {
                func:
                    Function::SpicePulse | Function::SpiceSin | Function::SpiceExp | Function::SpiceSffm,
                args,
            } => args.iter().all(crate::expr::constant_over_time),
            _ => false,
        };
        if !qualified {
            return None;
        }
    }
    match expr {
        Expr::Const(value) => derivative_pair(*value, 0.0),
        Expr::Time => derivative_pair(
            context.time,
            Value::from(matches!(context.target, DerivativeTarget::Time)),
        ),
        Expr::Frequency => derivative_pair(context.frequency, 0.0),
        Expr::Temperature => derivative_pair(context.temperature, 0.0),
        Expr::ThermalVoltage => derivative_pair(
            crate::constants::thermal_voltage(crate::constants::celsius_to_kelvin(
                context.temperature,
            )),
            0.0,
        ),
        Expr::Gmin => derivative_pair(context.gmin, 0.0),
        Expr::StringLiteral(_) => derivative_pair(0.0, 0.0),
        Expr::LookupTable { input, table } => {
            let (input_value, input_derivative) =
                eval_behavioral_expr_with_derivative(input, context)?;
            let (value, derivative) = lookup_table_interpolate_with_derivative(
                input_value,
                table,
                context.expression_dialect,
            );
            derivative_pair(
                value,
                if input_derivative == 0.0 {
                    0.0.into()
                } else {
                    derivative * input_derivative
                },
            )
        }
        Expr::NodeVoltage(name) => {
            let idx = *context.program.node_map.get(name)?;
            let value = *context.node_values.get(idx)?;
            let derivative = match context.target {
                DerivativeTarget::Node(target_idx) if target_idx == idx => 1.0.into(),
                DerivativeTarget::NodeDirection(direction) => *direction.get(idx)?,
                _ => 0.0.into(),
            };
            derivative_pair(value, derivative)
        }
        Expr::BranchCurrent(name) => {
            let idx = *context.program.branch_map.get(name)?;
            let value = *context.branch_values.get(idx)?;
            let derivative = match context.target {
                DerivativeTarget::Branch(target_idx) if target_idx == idx => 1.0,
                _ => 0.0,
            };
            derivative_pair(value, derivative)
        }
        Expr::Unary { op, operand } => {
            let (value, derivative) = eval_behavioral_expr_with_derivative(operand, context)?;
            match op {
                UnaryOp::Neg => derivative_pair(-value, -derivative),
                UnaryOp::Not => derivative_pair(if value == 0.0 { 1.0 } else { 0.0 }, 0.0),
            }
        }
        Expr::Binary { op, left, right } => {
            let (left_value, left_derivative) =
                eval_behavioral_expr_with_derivative(left, context)?;
            let (right_value, right_derivative) =
                eval_behavioral_expr_with_derivative(right, context)?;
            if matches!(context.target, DerivativeTarget::Time)
                && matches!(
                    op,
                    BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge
                )
                && left_value == right_value
                && !(crate::expr::constant_over_time(left)
                    && crate::expr::constant_over_time(right))
            {
                if left_derivative == right_derivative {
                    return None;
                }
                let positive = left_derivative > right_derivative;
                let truth = match op {
                    BinaryOp::Gt | BinaryOp::Ge => positive,
                    _ => !positive,
                };
                return derivative_pair(bool_value(truth), 0.0);
            }
            if matches!(context.target, DerivativeTarget::Time) && *op == BinaryOp::Pow {
                // The time rule uses the polynomial derivative at zero too;
                // Xyce's Newton rule deliberately zeroes that derivative.
                return derivative_pair(
                    left_value.powf(right_value),
                    if right_value == 0.0 {
                        0.0.into()
                    } else {
                        left_derivative * right_value * left_value.powf(right_value - 1.0)
                    },
                );
            }
            eval_binary_with_derivative(
                *op,
                left_value,
                left_derivative,
                right_value,
                right_derivative,
                context.expression_dialect,
            )
        }
        Expr::Function { func, args } => eval_function_with_derivative(*func, args, context),
    }
}

pub(crate) fn eval_binary_with_derivative(
    op: BinaryOp,
    left: Value,
    d_left: Derivative,
    right: Value,
    d_right: Derivative,
    expression_dialect: ExpressionDialect,
) -> Option<(Value, Derivative)> {
    match op {
        BinaryOp::Add => derivative_pair(left + right, d_left + d_right),
        BinaryOp::Sub => derivative_pair(left - right, d_left - d_right),
        BinaryOp::Mul => derivative_pair(left * right, d_left * right + left * d_right),
        BinaryOp::Div => {
            if right == 0.0 {
                None
            } else {
                derivative_pair(
                    left / right,
                    Derivative::product_ratio(
                        [(d_left, right), (-d_right, left)],
                        [(right, right), (0.0, 0.0)],
                    ),
                )
            }
        }
        BinaryOp::Mod => {
            if right == 0.0 {
                None
            } else {
                let quotient = (left / right).trunc();
                derivative_pair(left % right, d_left - quotient * d_right)
            }
        }
        BinaryOp::Pow => real_pow_with_derivative(left, d_left, right, d_right, expression_dialect),
        BinaryOp::Lt => derivative_pair(bool_value(left < right), 0.0),
        BinaryOp::Le => derivative_pair(bool_value(left <= right), 0.0),
        BinaryOp::Gt => derivative_pair(bool_value(left > right), 0.0),
        BinaryOp::Ge => derivative_pair(bool_value(left >= right), 0.0),
        BinaryOp::Eq => {
            derivative_pair(bool_value((left - right).abs() < EXPR_ZERO_TOLERANCE), 0.0)
        }
        BinaryOp::Ne => {
            derivative_pair(bool_value((left - right).abs() >= EXPR_ZERO_TOLERANCE), 0.0)
        }
        BinaryOp::And => derivative_pair(bool_value(left != 0.0 && right != 0.0), 0.0),
        BinaryOp::Or => derivative_pair(bool_value(left != 0.0 || right != 0.0), 0.0),
    }
}

fn eval_function_with_derivative(
    func: Function,
    args: &[Expr],
    context: &BehavioralDerivativeContext<'_>,
) -> Option<(Value, Derivative)> {
    let eval_arg = |index: usize| {
        args.get(index)
            .and_then(|arg| eval_behavioral_expr_with_derivative(arg, context))
    };

    match func {
        Function::Abs => {
            let (x, dx) = eval_arg(0)?;
            if x == 0.0 && matches!(context.target, DerivativeTarget::Time) {
                return derivative_pair(0.0, dx.abs());
            }
            derivative_pair(x.abs(), x.signum() * dx)
        }
        Function::Sqrt => {
            let (x, dx) = eval_arg(0)?;
            let value = x.max(0.0).sqrt();
            if value == 0.0 {
                derivative_pair(value, 0.0)
            } else {
                derivative_pair(value, 0.5 * dx / value)
            }
        }
        Function::Exp => unary_derivative(eval_arg(0)?, |x| x.exp(), |x| x.exp()),
        Function::Log => {
            let (x, dx) = eval_arg(0)?;
            let clamped = behavioral_logarithm_argument(x);
            if context.expression_dialect == ExpressionDialect::Xyce {
                derivative_pair(clamped.log10(), dx / clamped / std::f64::consts::LN_10)
            } else {
                derivative_pair(clamped.ln(), dx / clamped)
            }
        }
        Function::Ln => {
            let (x, dx) = eval_arg(0)?;
            let clamped = behavioral_logarithm_argument(x);
            derivative_pair(clamped.ln(), dx / clamped)
        }
        Function::Log10 => {
            let (x, dx) = eval_arg(0)?;
            let clamped = behavioral_logarithm_argument(x);
            derivative_pair(clamped.log10(), dx / clamped / std::f64::consts::LN_10)
        }
        Function::Sin => unary_derivative(eval_arg(0)?, |x| x.sin(), |x| x.cos()),
        Function::Cos => unary_derivative(eval_arg(0)?, |x| x.cos(), |x| -x.sin()),
        Function::Tan => unary_derivative(
            eval_arg(0)?,
            |x| x.tan(),
            |x| {
                let cos_x = x.cos();
                1.0 / (cos_x * cos_x)
            },
        ),
        Function::Asin => unary_derivative(
            eval_arg(0)?,
            |x| x.clamp(-1.0, 1.0).asin(),
            |x| 1.0 / (1.0 - x * x).sqrt(),
        ),
        Function::Acos => unary_derivative(
            eval_arg(0)?,
            |x| x.clamp(-1.0, 1.0).acos(),
            |x| -1.0 / (1.0 - x * x).sqrt(),
        ),
        Function::Atan => {
            let (x, dx) = eval_arg(0)?;
            derivative_pair(
                x.atan(),
                Derivative::product_ratio([(dx, 1.0), (0.0.into(), 0.0)], [(1.0, 1.0), (x, x)]),
            )
        }
        Function::Atan2 => {
            let (y, dy) = eval_arg(0)?;
            let (x, dx) = eval_arg(1)?;
            if x == 0.0 && y == 0.0 {
                None
            } else {
                derivative_pair(
                    y.atan2(x),
                    Derivative::product_ratio([(dy, x), (-dx, y)], [(x, x), (y, y)]),
                )
            }
        }
        Function::Sinh => unary_derivative(eval_arg(0)?, |x| x.sinh(), |x| x.cosh()),
        Function::Cosh => unary_derivative(eval_arg(0)?, |x| x.cosh(), |x| x.sinh()),
        Function::Tanh => {
            let (x, dx) = eval_arg(0)?;
            if context.expression_dialect == ExpressionDialect::Xyce {
                let value = xyce_tanh_behavioral(x);
                let derivative = if (-XYCE_TANH_SATURATION_THRESHOLD
                    ..=XYCE_TANH_SATURATION_THRESHOLD)
                    .contains(&x)
                {
                    let cosh_x = x.cosh();
                    dx / (cosh_x * cosh_x)
                } else {
                    0.0.into()
                };
                derivative_pair(value, derivative)
            } else {
                let value = x.tanh();
                // 1-tanh(x)^2 cancels near saturation. Keep the decaying
                // exponential separate from its incoming tangent as well.
                let argument = -2.0 * x.abs();
                let denominator = 1.0 + argument.exp();
                derivative_pair(
                    value,
                    (dx * 4.0 / denominator / denominator).multiply_exp(argument),
                )
            }
        }
        Function::Asinh => {
            let (x, dx) = eval_arg(0)?;
            derivative_pair(x.asinh(), dx / x.hypot(1.0))
        }
        Function::Acosh => {
            let (x, dx) = eval_arg(0)?;
            derivative_pair(x.acosh(), dx / (x - 1.0).sqrt() / (x + 1.0).sqrt())
        }
        Function::Atanh => {
            let (x, dx) = eval_arg(0)?;
            if context.expression_dialect == ExpressionDialect::Xyce {
                let lower = XYCE_ATANH_EPSILON - 1.0;
                let upper = 1.0 - XYCE_ATANH_EPSILON;
                let clamped = x.clamp(lower, upper);
                let derivative = if x >= lower && x <= upper {
                    dx / (1.0 - x * x)
                } else {
                    0.0.into()
                };
                derivative_pair(clamped.atanh(), derivative)
            } else {
                derivative_pair(x.atanh(), dx / (1.0 - x * x))
            }
        }
        Function::Trunc => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(x.trunc(), 0.0)
        }
        Function::Floor => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(x.floor(), 0.0)
        }
        Function::Ceil => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(x.ceil(), 0.0)
        }
        Function::Round => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(x.round_ties_even(), 0.0)
        }
        Function::Sqr => {
            let (x, dx) = eval_arg(0)?;
            derivative_pair(x * x, dx * 2.0 * x)
        }
        Function::Pwr => {
            let (base, d_base) = eval_arg(0)?;
            let (exponent, d_exponent) = eval_arg(1)?;
            real_function_pwr_with_derivative(
                base,
                d_base,
                exponent,
                d_exponent,
                context.expression_dialect,
            )
        }
        Function::Pwrs => {
            let (base, d_base) = eval_arg(0)?;
            let (exponent, d_exponent) = eval_arg(1)?;
            real_function_pwrs_with_derivative(
                base,
                d_base,
                exponent,
                d_exponent,
                context.expression_dialect,
            )
        }
        Function::Limit => match args.len() {
            2 => {
                let (nom, d_nom) = eval_arg(0)?;
                derivative_pair(nom, d_nom)
            }
            3 => {
                let (x, dx) = eval_arg(0)?;
                let (min, _) = eval_arg(1)?;
                let (max, _) = eval_arg(2)?;
                let (value, retains_input_derivative) =
                    ordered_limit(x, min, max, context.expression_dialect);
                derivative_pair(
                    value,
                    if retains_input_derivative {
                        dx
                    } else {
                        0.0.into()
                    },
                )
            }
            _ => None,
        },
        Function::Min => {
            let mut best = eval_arg(0)?;
            for index in 1..args.len() {
                let candidate = eval_arg(index)?;
                if candidate.0 < best.0
                    || (candidate.0 == best.0
                        && matches!(context.target, DerivativeTarget::Time)
                        && candidate.1 < best.1)
                {
                    best = candidate;
                }
            }
            Some(best)
        }
        Function::Max => {
            let mut best = eval_arg(0)?;
            for index in 1..args.len() {
                let candidate = eval_arg(index)?;
                if candidate.0 > best.0
                    || (candidate.0 == best.0
                        && matches!(context.target, DerivativeTarget::Time)
                        && candidate.1 > best.1)
                {
                    best = candidate;
                }
            }
            Some(best)
        }
        Function::Sign => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(ordered_sign(x), 0.0)
        }
        Function::HspiceSign => {
            let (magnitude, d_magnitude) = eval_arg(0)?;
            let (polarity, _) = eval_arg(1)?;
            let sign = ordered_sign(polarity);
            let magnitude_derivative = if magnitude >= 0.0 {
                d_magnitude
            } else {
                -d_magnitude
            };
            derivative_pair(magnitude.abs() * sign, magnitude_derivative * sign)
        }
        Function::Uramp => {
            let (x, dx) = eval_arg(0)?;
            if x == 0.0 && matches!(context.target, DerivativeTarget::Time) {
                return derivative_pair(0.0, dx.max(0.0));
            }
            derivative_pair(x.max(0.0), if x > 0.0 { dx } else { 0.0.into() })
        }
        Function::Stp => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(bool_value(x > EXPR_ZERO_TOLERANCE), 0.0)
        }
        Function::Ustep => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(
                if x > 0.0 {
                    1.0
                } else if x < 0.0 {
                    0.0
                } else {
                    0.5
                },
                0.0,
            )
        }
        Function::U2 => {
            let (x, dx) = eval_arg(0)?;
            derivative_pair(
                x.clamp(0.0, 1.0),
                if x > 0.0 && x < 1.0 { dx } else { 0.0.into() },
            )
        }
        Function::Eq0 => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(bool_value(x.abs() < EXPR_ZERO_TOLERANCE), 0.0)
        }
        Function::Ne0 => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(bool_value(x.abs() >= EXPR_ZERO_TOLERANCE), 0.0)
        }
        Function::Gt0 => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(bool_value(x > 0.0), 0.0)
        }
        Function::Lt0 => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(bool_value(x < 0.0), 0.0)
        }
        Function::Ge0 => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(bool_value(x >= 0.0), 0.0)
        }
        Function::Le0 => {
            let (x, _) = eval_arg(0)?;
            derivative_pair(bool_value(x <= 0.0), 0.0)
        }
        Function::Pow => {
            let (left, d_left) = eval_arg(0)?;
            let (right, d_right) = eval_arg(1)?;
            real_function_pow_with_derivative(
                left,
                d_left,
                right,
                d_right,
                context.expression_dialect,
            )
        }
        Function::Mod => {
            let (left, d_left) = eval_arg(0)?;
            let (right, d_right) = eval_arg(1)?;
            if right == 0.0 {
                None
            } else {
                let quotient = (left / right).trunc();
                derivative_pair(left % right, d_left - quotient * d_right)
            }
        }
        Function::Table => eval_table_function_with_derivative(args, context),
        Function::Pwl => eval_pwl_function_with_derivative(args, context),
        Function::TableFile
        | Function::FastTable
        | Function::FastTableFile
        | Function::Cubic
        | Function::CubicFile
        | Function::Akima
        | Function::AkimaFile
        | Function::Wodicka
        | Function::WodickaFile
        | Function::Barycentric
        | Function::BarycentricFile => derivative_pair(0.0, 0.0),
        Function::Sdt => None,
        Function::SpicePulse | Function::SpiceSin | Function::SpiceExp | Function::SpiceSffm => {
            if !matches!(context.target, DerivativeTarget::Time) {
                return None;
            }
            let mut values = Vec::with_capacity(args.len());
            for index in 0..args.len() {
                let (value, derivative) = eval_arg(index)?;
                if derivative != 0.0 {
                    return None;
                }
                values.push(value);
            }
            crate::expr::spice_waveform_value_and_time_derivative(func, &values, context.time)
                .and_then(|(value, derivative)| derivative_pair(value, derivative))
        }
        Function::If => {
            let (condition, derivative) = eval_arg(0)?;
            if condition == 0.0
                && matches!(context.target, DerivativeTarget::Time)
                && !crate::expr::constant_over_time(&args[0])
                && !matches!(
                    &args[0],
                    Expr::Binary {
                        op: BinaryOp::Lt
                            | BinaryOp::Le
                            | BinaryOp::Gt
                            | BinaryOp::Ge
                            | BinaryOp::Eq
                            | BinaryOp::Ne
                            | BinaryOp::And
                            | BinaryOp::Or,
                        ..
                    }
                )
            {
                if derivative == 0.0 {
                    return None;
                }
                return eval_arg(1);
            }
            if condition != 0.0 {
                eval_arg(1)
            } else {
                eval_arg(2)
            }
        }
    }
}

fn unary_derivative(
    input: (Value, Derivative),
    value_fn: impl FnOnce(Value) -> Value,
    derivative_fn: impl FnOnce(Value) -> Value,
) -> Option<(Value, Derivative)> {
    let (x, dx) = input;
    derivative_pair(value_fn(x), derivative_fn(x) * dx)
}

fn eval_table_function_with_derivative(
    args: &[Expr],
    context: &BehavioralDerivativeContext<'_>,
) -> Option<(Value, Derivative)> {
    eval_piecewise_function_with_derivative(args, context, eval_table_points_with_derivative)
}

fn eval_pwl_function_with_derivative(
    args: &[Expr],
    context: &BehavioralDerivativeContext<'_>,
) -> Option<(Value, Derivative)> {
    eval_piecewise_function_with_derivative(args, context, eval_pwl_points_with_derivative)
}

/// Interpolate a breakpoint table and report the value and its slope.
///
/// Takes the point to interpolate at, that point's derivative with respect to
/// the differentiation variable, and the `(x, y)` breakpoints.
type PiecewiseDerivativeEvaluator =
    fn(Value, Derivative, &[(Value, Value)]) -> Option<(Value, Derivative)>;

fn eval_piecewise_function_with_derivative(
    args: &[Expr],
    context: &BehavioralDerivativeContext<'_>,
    evaluator: PiecewiseDerivativeEvaluator,
) -> Option<(Value, Derivative)> {
    if args.len() < 3 {
        return None;
    }
    let (x, dx) = eval_behavioral_expr_with_derivative(&args[0], context)?;
    let mut points = Vec::new();
    for pair in args[1..].chunks(2) {
        let x_expr = pair.first()?;
        let y_expr = pair.get(1)?;
        let (px, _) = eval_behavioral_expr_with_derivative(x_expr, context)?;
        let (py, _) = eval_behavioral_expr_with_derivative(y_expr, context)?;
        points.push((px, py));
    }
    evaluator(x, dx, &points)
}

fn eval_table_points_with_derivative(
    x: Value,
    dx: Derivative,
    points: &[(Value, Value)],
) -> Option<(Value, Derivative)> {
    if points.is_empty() {
        return derivative_pair(0.0, 0.0);
    }
    if points.len() == 1 {
        return derivative_pair(points[0].1, 0.0);
    }
    if x <= points[0].0 {
        return derivative_pair(points[0].1, 0.0);
    }
    let last = points.len() - 1;
    if x >= points[last].0 {
        return derivative_pair(points[last].1, 0.0);
    }
    let mut segment = (points[0], points[1]);
    if x > points[0].0 {
        for pair in points.windows(2) {
            let left = pair[0];
            let right = pair[1];
            segment = (left, right);
            if x <= right.0 {
                break;
            }
        }
    }
    eval_linear_piecewise_segment_with_derivative(x, dx, segment)
}

fn eval_pwl_points_with_derivative(
    x: Value,
    dx: Derivative,
    points: &[(Value, Value)],
) -> Option<(Value, Derivative)> {
    if points.is_empty() {
        return derivative_pair(0.0, 0.0);
    }
    if points.len() == 1 {
        return derivative_pair(points[0].1, 0.0);
    }

    let last = points.len() - 1;
    let ascending = points[last].0 >= points[0].0;
    let segment = if ascending {
        if x <= points[0].0 {
            (points[0], points[1])
        } else if x >= points[last].0 {
            (points[last - 1], points[last])
        } else {
            points
                .windows(2)
                .find(|pair| x >= pair[0].0 && x <= pair[1].0)
                .map(|pair| (pair[0], pair[1]))
                .unwrap_or((points[last - 1], points[last]))
        }
    } else if x >= points[0].0 {
        (points[0], points[1])
    } else if x <= points[last].0 {
        (points[last - 1], points[last])
    } else {
        points
            .windows(2)
            .find(|pair| x <= pair[0].0 && x >= pair[1].0)
            .map(|pair| (pair[0], pair[1]))
            .unwrap_or((points[last - 1], points[last]))
    };

    eval_linear_piecewise_segment_with_derivative(x, dx, segment)
}

fn eval_linear_piecewise_segment_with_derivative(
    x: Value,
    dx: Derivative,
    ((x0, y0), (x1, y1)): ((Value, Value), (Value, Value)),
) -> Option<(Value, Derivative)> {
    let span = x1 - x0;
    if !span.is_finite() || span == 0.0 {
        return derivative_pair(y0, 0.0);
    }
    let slope = (y1 - y0) / span;
    derivative_pair(y0 + (x - x0) * slope, slope * dx)
}

fn xyce_tanh_behavioral(value: Value) -> Value {
    if value > XYCE_TANH_SATURATION_THRESHOLD {
        1.0
    } else if value < -XYCE_TANH_SATURATION_THRESHOLD {
        -1.0
    } else {
        value.tanh()
    }
}

fn bool_value(value: bool) -> Value {
    if value { 1.0 } else { 0.0 }
}

fn linearization_values_converged(
    actual: Value,
    linearized: Value,
    reltol: Value,
    abstol: Value,
) -> bool {
    if !actual.is_finite() || !linearized.is_finite() {
        return false;
    }
    let scale = actual.abs().max(linearized.abs()).max(1.0);
    (actual - linearized).abs() <= abstol + reltol * scale
}

/// Compiled behavioral current source
#[derive(Debug, Clone)]
pub struct BehavioralCurrentSource {
    /// Device name
    pub name: String,
    /// Positive node (current flows into)
    pub node_pos: usize,
    /// Negative node (current flows out of)
    pub node_neg: usize,
    /// Parsed expression used for structural analysis such as breakpoint extraction.
    ast: Expr,
    /// Compiled expression
    pub program: CompiledExpr,
    /// VM for evaluation
    vm: Vm,
    /// Compiled-expression node references mapped to circuit solution indices
    node_bindings: Vec<Option<usize>>,
    /// Compiled-expression branch references mapped to circuit solution indices
    branch_bindings: Vec<Option<usize>>,
    /// Reused scratch storage for expression node values
    node_values: Vec<Value>,
    /// Reused scratch storage for expression branch-current values
    branch_values: Vec<Value>,
    /// Linearization partials d(expr)/d(node_values`[idx]`)
    node_partials: Vec<Value>,
    /// Linearization partials d(expr)/d(branch_values`[idx]`)
    branch_partials: Vec<Value>,
    /// Affine term for the most recent expression linearization.
    linearized_affine: Value,
    /// Circuit temperature in degrees Celsius, surfaced as `temper`.
    temperature: Value,
    /// Active analysis frequency in hertz.
    frequency: Value,
    /// Whether the resolved expression contains the live AC frequency.
    frequency_dependent: bool,
    /// Active nonlinear minimum conductance, surfaced as `GMIN`.
    gmin: Value,
    /// Dialect-specific expression-function semantics.
    expression_dialect: ExpressionDialect,
    /// Whether this expression represents a two-terminal device whose lead
    /// current and power are part of the public device-observable surface.
    two_terminal_observables: bool,
}

impl BehavioralCurrentSource {
    fn nonfinite_error(
        &self,
        quantity: impl Into<String>,
        time: Value,
        value: Value,
    ) -> BehavioralEvaluationError {
        BehavioralEvaluationError::NonFinite {
            source_kind: "current",
            source_name: self.name.clone(),
            quantity: quantity.into(),
            time,
            frequency: self.frequency,
            value,
        }
    }

    fn stamp_error(&self, time: Value, detail: impl Into<String>) -> BehavioralEvaluationError {
        BehavioralEvaluationError::Stamp {
            source_kind: "current",
            source_name: self.name.clone(),
            time,
            frequency: self.frequency,
            detail: detail.into(),
        }
    }

    /// Create a new behavioral current source
    pub fn new(
        name: String,
        node_pos: usize,
        node_neg: usize,
        expression: &str,
    ) -> Result<Self, String> {
        Self::new_with_source_path(name, node_pos, node_neg, expression, None)
    }

    /// Create a new behavioral current source with deck-relative file-function support.
    pub fn new_with_source_path(
        name: String,
        node_pos: usize,
        node_neg: usize,
        expression: &str,
        source_path: Option<&Path>,
    ) -> Result<Self, String> {
        Self::new_with_source_path_and_limits(
            name,
            node_pos,
            node_neg,
            expression,
            source_path,
            crate::resource::ResourceLimits::default(),
        )
    }

    /// Create a behavioral current source with file lookups governed by an explicit policy.
    pub fn new_with_source_path_and_limits(
        name: String,
        node_pos: usize,
        node_neg: usize,
        expression: &str,
        source_path: Option<&Path>,
        resource_limits: crate::resource::ResourceLimits,
    ) -> Result<Self, String> {
        let ast = parse_expression_strict(expression)
            .map_err(|e| format!("Invalid behavioral expression '{}': {}", expression, e))?;
        let ast = resolve_file_lookup_functions_with_limits(ast, source_path, resource_limits)
            .map_err(|e| format!("Invalid behavioral expression '{}': {}", expression, e))?;
        let frequency_dependent = expression_depends_on_frequency(&ast);
        let program = compile(&ast);

        Ok(Self {
            name,
            node_pos,
            node_neg,
            ast,
            program,
            vm: Vm::new(),
            node_bindings: Vec::new(),
            branch_bindings: Vec::new(),
            node_values: Vec::new(),
            branch_values: Vec::new(),
            node_partials: Vec::new(),
            branch_partials: Vec::new(),
            linearized_affine: 0.0,
            temperature: crate::constants::kelvin_to_celsius(crate::constants::TEMP_REFERENCE),
            frequency: 0.0,
            frequency_dependent,
            gmin: crate::constants::GMIN,
            expression_dialect: ExpressionDialect::Ngspice,
            two_terminal_observables: false,
        })
    }

    /// Resolve V(...) and I(...) references against circuit node/branch indices.
    pub fn bind_references<FN, FB>(
        &mut self,
        resolve_node: FN,
        resolve_branch: FB,
    ) -> Result<(), BehavioralReferenceError>
    where
        FN: Fn(&str) -> Option<usize>,
        FB: Fn(&str) -> BehavioralBranchResolution,
    {
        self.node_bindings = vec![None; self.program.node_map.len()];
        for (name, &local_idx) in &self.program.node_map {
            let resolved = if crate::naming::is_spice_ground_name(name) {
                Some(0usize)
            } else {
                resolve_node(name)
            }
            .ok_or_else(|| {
                BehavioralReferenceError::new(
                    &self.name,
                    name,
                    BehavioralReferenceReason::UnknownNode,
                )
            })?;
            self.node_bindings[local_idx] = resolved.checked_sub(1);
        }

        self.branch_bindings = vec![None; self.program.branch_map.len()];
        for (name, &local_idx) in &self.program.branch_map {
            let resolved = match resolve_branch(name) {
                BehavioralBranchResolution::Branch(index) => index,
                BehavioralBranchResolution::DeviceWithoutBranch => {
                    return Err(BehavioralReferenceError::new(
                        &self.name,
                        name,
                        BehavioralReferenceReason::LeadCurrentNotSolutionVariable,
                    ));
                }
                BehavioralBranchResolution::MissingDevice => {
                    return Err(BehavioralReferenceError::new(
                        &self.name,
                        name,
                        BehavioralReferenceReason::UnknownDevice,
                    ));
                }
            };
            self.branch_bindings[local_idx] = Some(resolved);
        }

        self.node_values.resize(self.node_bindings.len(), 0.0);
        self.branch_values.resize(self.branch_bindings.len(), 0.0);
        self.node_partials.resize(self.node_bindings.len(), 0.0);
        self.branch_partials.resize(self.branch_bindings.len(), 0.0);
        Ok(())
    }

    #[inline]
    fn refresh_expression_inputs(&mut self, solution: &[Value]) {
        for (idx, binding) in self.node_bindings.iter().enumerate() {
            self.node_values[idx] = binding
                .and_then(|global_idx| solution.get(global_idx).copied())
                .unwrap_or(0.0);
        }
        for (idx, binding) in self.branch_bindings.iter().enumerate() {
            self.branch_values[idx] = binding
                .and_then(|global_idx| solution.get(global_idx).copied())
                .unwrap_or(0.0);
        }
    }

    /// Evaluate the expression with current circuit solution.
    pub fn evaluate(
        &mut self,
        solution: &[Value],
        time: Value,
    ) -> Result<Value, BehavioralEvaluationError> {
        self.refresh_expression_inputs(solution);
        let value = self.evaluate_with_cached_inputs(time);
        if !value.is_finite() {
            return Err(self.nonfinite_error("expression value", time, value));
        }
        Ok(value)
    }

    /// Evaluate an owned VM candidate without changing accepted operator state.
    /// The compiled expression and binding tables remain shared with this source.
    fn prepare_transient_step(
        &mut self,
        solution: &[Value],
        time: Value,
    ) -> Result<Vm, BehavioralEvaluationError> {
        self.refresh_expression_inputs(solution);
        let context = Context::transient(&self.node_values, &self.branch_values, time)
            .with_frequency(self.frequency)
            .with_temperature(self.temperature)
            .with_gmin(self.gmin)
            .with_expression_dialect(self.expression_dialect)
            .with_ieee_logarithm();
        let mut candidate = self.vm.clone();
        let value = candidate.execute(&self.program, &context);
        if !value.is_finite() {
            return Err(self.nonfinite_error("expression value", time, value));
        }
        candidate.accept_transient_step(time);
        Ok(candidate)
    }

    #[inline]
    pub(crate) fn bound_solution_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.node_bindings
            .iter()
            .chain(self.branch_bindings.iter())
            .filter_map(|binding| *binding)
    }

    pub(crate) fn is_solution_dependent(&self) -> bool {
        !self.program.node_map.is_empty() || !self.program.branch_map.is_empty()
    }

    #[inline]
    fn evaluate_with_cached_inputs(&mut self, time: Value) -> Value {
        let ctx = Context::transient(&self.node_values, &self.branch_values, time)
            .with_frequency(self.frequency)
            .with_temperature(self.temperature)
            .with_gmin(self.gmin)
            .with_expression_dialect(self.expression_dialect)
            .with_ieee_logarithm();
        self.vm.execute(&self.program, &ctx)
    }

    /// Set the circuit temperature (degrees Celsius) surfaced as `temper`.
    pub fn set_temperature(&mut self, temperature: Value) {
        self.temperature = temperature;
    }

    pub fn set_frequency(&mut self, frequency: Value) {
        self.frequency = frequency;
    }

    /// Whether this source's resolved expression depends on `FREQ`/`HERTZ`.
    #[inline]
    pub(crate) fn is_frequency_dependent(&self) -> bool {
        self.frequency_dependent
    }

    pub fn set_gmin(&mut self, gmin: Value) {
        self.gmin = gmin;
    }

    /// Set dialect-specific expression-function semantics.
    pub fn set_expression_dialect(&mut self, dialect: ExpressionDialect) {
        self.expression_dialect = dialect;
    }

    /// Mark this current expression as the constitutive law of a two-terminal
    /// device (currently a solution-dependent resistor), rather than an
    /// independent behavioral current source.
    pub(crate) fn enable_two_terminal_observables(&mut self) {
        self.two_terminal_observables = true;
    }

    pub(crate) fn has_two_terminal_observables(&self) -> bool {
        self.two_terminal_observables
    }

    #[inline]
    fn derivative_step(base: Value) -> Value {
        DERIVATIVE_ABS_STEP + DERIVATIVE_REL_STEP * base.abs().max(1.0)
    }

    #[inline]
    fn estimate_node_partial(
        &mut self,
        idx: usize,
        f0: Value,
        time: Value,
    ) -> Result<Value, BehavioralEvaluationError> {
        let base = self.node_values[idx];
        let h = Self::derivative_step(base);
        self.node_values[idx] = base + h;
        let fp = self.evaluate_with_cached_inputs(time);
        self.node_values[idx] = base - h;
        let fm = self.evaluate_with_cached_inputs(time);
        self.node_values[idx] = base;

        let df = finite_difference_with_one_sided_fallback(f0, fp, fm, h).ok_or_else(|| {
            self.nonfinite_error(
                format!("node derivative {idx} (no finite difference exists)"),
                time,
                Value::NAN,
            )
        })?;
        Ok(df)
    }

    #[inline]
    fn estimate_branch_partial(
        &mut self,
        idx: usize,
        f0: Value,
        time: Value,
    ) -> Result<Value, BehavioralEvaluationError> {
        let base = self.branch_values[idx];
        let h = Self::derivative_step(base);
        self.branch_values[idx] = base + h;
        let fp = self.evaluate_with_cached_inputs(time);
        self.branch_values[idx] = base - h;
        let fm = self.evaluate_with_cached_inputs(time);
        self.branch_values[idx] = base;

        let df = finite_difference_with_one_sided_fallback(f0, fp, fm, h).ok_or_else(|| {
            self.nonfinite_error(
                format!("branch-current derivative {idx} (no finite difference exists)"),
                time,
                Value::NAN,
            )
        })?;
        Ok(df)
    }

    fn linearize_expression(
        &mut self,
        solution: &[Value],
        time: Value,
    ) -> Result<Value, BehavioralEvaluationError> {
        self.refresh_expression_inputs(solution);
        let f0 = self.evaluate_with_cached_inputs(time);

        if !f0.is_finite() {
            self.node_partials.fill(0.0);
            self.branch_partials.fill(0.0);
            self.linearized_affine = 0.0;
            return Err(self.nonfinite_error("expression value", time, f0));
        }

        for idx in 0..self.node_bindings.len() {
            self.node_partials[idx] = if self.node_bindings[idx].is_some() {
                analytic_expression_partial(
                    &self.ast,
                    &self.program,
                    &self.node_values,
                    &self.branch_values,
                    BehavioralEnvironment {
                        time,
                        frequency: self.frequency,
                        temperature: self.temperature,
                        gmin: self.gmin,
                        expression_dialect: self.expression_dialect,
                    },
                    DerivativeTarget::Node(idx),
                )
                .map(Ok)
                .unwrap_or_else(|| self.estimate_node_partial(idx, f0, time))?
            } else {
                0.0
            };
        }
        for idx in 0..self.branch_bindings.len() {
            self.branch_partials[idx] = if self.branch_bindings[idx].is_some() {
                analytic_expression_partial(
                    &self.ast,
                    &self.program,
                    &self.node_values,
                    &self.branch_values,
                    BehavioralEnvironment {
                        time,
                        frequency: self.frequency,
                        temperature: self.temperature,
                        gmin: self.gmin,
                        expression_dialect: self.expression_dialect,
                    },
                    DerivativeTarget::Branch(idx),
                )
                .map(Ok)
                .unwrap_or_else(|| self.estimate_branch_partial(idx, f0, time))?
            } else {
                0.0
            };
        }

        let mut affine = f0;
        for (idx, binding) in self.node_bindings.iter().enumerate() {
            if let Some(global_idx) = binding {
                let term = self.node_partials[idx] * solution[*global_idx];
                if !term.is_finite() {
                    return Err(self.nonfinite_error(
                        format!("node affine product {idx}"),
                        time,
                        term,
                    ));
                }
                affine -= term;
                if !affine.is_finite() {
                    return Err(self.nonfinite_error("affine linearization", time, affine));
                }
            }
        }
        for (idx, binding) in self.branch_bindings.iter().enumerate() {
            if let Some(global_idx) = binding {
                let term = self.branch_partials[idx] * solution[*global_idx];
                if !term.is_finite() {
                    return Err(self.nonfinite_error(
                        format!("branch-current affine product {idx}"),
                        time,
                        term,
                    ));
                }
                affine -= term;
                if !affine.is_finite() {
                    return Err(self.nonfinite_error("affine linearization", time, affine));
                }
            }
        }
        self.linearized_affine = affine;
        Ok(affine)
    }

    /// Refresh the linearization (value and partials) at the given
    /// operating point for small-signal assembly. AC has no time axis;
    /// expressions see t = 0.
    pub(crate) fn linearize_at(
        &mut self,
        solution: &[Value],
    ) -> Result<(), BehavioralEvaluationError> {
        self.frequency = 0.0;
        self.linearize_expression(solution, 0.0)?;
        Ok(())
    }

    pub(crate) fn linearize_at_frequency(
        &mut self,
        solution: &[Value],
        frequency: Value,
    ) -> Result<(), BehavioralEvaluationError> {
        if !self.frequency_dependent {
            return Ok(());
        }
        self.frequency = frequency;
        self.linearize_expression(solution, 0.0)?;
        Ok(())
    }

    /// Linearize at an arbitrary state and frequency. Unlike
    /// [`Self::linearize_at_frequency`], this always refreshes the Jacobian
    /// because the supplied state may have changed.
    pub(crate) fn linearize_at_state_and_frequency(
        &mut self,
        solution: &[Value],
        frequency: Value,
    ) -> Result<(), BehavioralEvaluationError> {
        self.frequency = frequency;
        self.linearize_expression(solution, 0.0)?;
        Ok(())
    }

    /// Visit the cached linearized partials as `(solution_index, df/dx)`
    /// pairs. Valid after `linearize_at` (or any stamp call).
    pub(crate) fn linearized_partials(&self) -> impl Iterator<Item = (usize, Value)> + '_ {
        self.node_bindings
            .iter()
            .zip(&self.node_partials)
            .chain(self.branch_bindings.iter().zip(&self.branch_partials))
            .filter_map(|(binding, df)| binding.map(|idx| (idx, *df)))
    }

    fn linearized_expression_value(&self, solution: &[Value]) -> Value {
        let node_value = self
            .node_bindings
            .iter()
            .zip(&self.node_partials)
            .filter_map(|(binding, df)| {
                binding.and_then(|idx| solution.get(idx).map(|value| *df * *value))
            })
            .sum::<Value>();
        let branch_value = self
            .branch_bindings
            .iter()
            .zip(&self.branch_partials)
            .filter_map(|(binding, df)| {
                binding.and_then(|idx| solution.get(idx).map(|value| *df * *value))
            })
            .sum::<Value>();
        self.linearized_affine + node_value + branch_value
    }

    fn linearization_converged(
        &mut self,
        solution: &[Value],
        time: Value,
        reltol: Value,
        abstol: Value,
    ) -> Result<bool, BehavioralEvaluationError> {
        let actual = self.evaluate(solution, time)?;
        let linearized = self.linearized_expression_value(solution);
        if !linearized.is_finite() {
            return Err(self.nonfinite_error("cached linearized value", time, linearized));
        }
        Ok(linearization_values_converged(
            actual, linearized, reltol, abstol,
        ))
    }

    /// Stamp linearized behavioral current source into matrix and RHS.
    pub fn stamp(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
    ) -> Result<(), BehavioralEvaluationError> {
        let i_affine = self.linearize_expression(solution, time)?;
        let np = self.node_pos;
        let nn = self.node_neg;

        // Current source orientation: I flows from n+ to n-.
        // Linearized form:
        // I(x) ~= affine + sum(df/dx * x)
        // KCL rows:
        // row(n+) += -I(x), row(n-) += +I(x)
        for (idx, binding) in self.node_bindings.iter().enumerate() {
            if let Some(global_idx) = binding {
                let df = self.node_partials[idx];
                if df != 0.0 {
                    if np > 0 {
                        try_stamp_behavioral_matrix_coefficient(
                            matrix,
                            BehavioralMatrixEntry {
                                row: np - 1,
                                column: *global_idx,
                                coefficient: df,
                            },
                            "current",
                            &self.name,
                            time,
                            self.frequency,
                        )?;
                    }
                    if nn > 0 {
                        try_stamp_behavioral_matrix_coefficient(
                            matrix,
                            BehavioralMatrixEntry {
                                row: nn - 1,
                                column: *global_idx,
                                coefficient: -df,
                            },
                            "current",
                            &self.name,
                            time,
                            self.frequency,
                        )?;
                    }
                }
            }
        }
        for (idx, binding) in self.branch_bindings.iter().enumerate() {
            if let Some(global_idx) = binding {
                let df = self.branch_partials[idx];
                if df != 0.0 {
                    if np > 0 {
                        try_stamp_behavioral_matrix_coefficient(
                            matrix,
                            BehavioralMatrixEntry {
                                row: np - 1,
                                column: *global_idx,
                                coefficient: df,
                            },
                            "current",
                            &self.name,
                            time,
                            self.frequency,
                        )?;
                    }
                    if nn > 0 {
                        try_stamp_behavioral_matrix_coefficient(
                            matrix,
                            BehavioralMatrixEntry {
                                row: nn - 1,
                                column: *global_idx,
                                coefficient: -df,
                            },
                            "current",
                            &self.name,
                            time,
                            self.frequency,
                        )?;
                    }
                }
            }
        }

        if np > 0 {
            let rhs_slot = rhs.get_mut(np - 1).ok_or_else(|| {
                self.stamp_error(
                    time,
                    format!("RHS row {} is outside the MNA system", np - 1),
                )
            })?;
            let updated = *rhs_slot - i_affine;
            if !updated.is_finite() {
                return Err(self.nonfinite_error("positive-node RHS stamp", time, updated));
            }
            *rhs_slot = updated;
        }
        if nn > 0 {
            let rhs_slot = rhs.get_mut(nn - 1).ok_or_else(|| {
                self.stamp_error(
                    time,
                    format!("RHS row {} is outside the MNA system", nn - 1),
                )
            })?;
            let updated = *rhs_slot + i_affine;
            if !updated.is_finite() {
                return Err(self.nonfinite_error("negative-node RHS stamp", time, updated));
            }
            *rhs_slot = updated;
        }
        Ok(())
    }
}

/// Storage for behavioral sources (not SoA due to compiled programs)
#[derive(Debug, Clone, Default)]
pub struct BehavioralSources {
    pub voltage_sources: Vec<BehavioralVoltageSource>,
    pub current_sources: Vec<BehavioralCurrentSource>,
}

/// Evaluated operator frames for an unchanged behavioral-source topology.
/// Dropping the candidate leaves every source's accepted VM state intact.
#[must_use]
pub(crate) struct PreparedBehavioralStep {
    voltage: Vec<Vm>,
    current: Vec<Vm>,
}

impl BehavioralSources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_voltage(&mut self, source: BehavioralVoltageSource) {
        self.voltage_sources.push(source);
    }

    pub fn add_current(&mut self, source: BehavioralCurrentSource) {
        self.current_sources.push(source);
    }

    /// Update the live solver GMIN seen by every retained expression.
    pub fn set_gmin(&mut self, gmin: Value) {
        for source in &mut self.voltage_sources {
            source.set_gmin(gmin);
        }
        for source in &mut self.current_sources {
            source.set_gmin(gmin);
        }
    }

    /// Resolve expression V(...) and I(...) references for all behavioral sources.
    pub fn bind_references<FN, FB>(
        &mut self,
        resolve_node: FN,
        resolve_branch: FB,
    ) -> Result<(), BehavioralReferenceError>
    where
        FN: Fn(&str) -> Option<usize> + Copy,
        FB: Fn(&str) -> BehavioralBranchResolution + Copy,
    {
        for source in &mut self.voltage_sources {
            source.bind_references(resolve_node, resolve_branch)?;
        }
        for source in &mut self.current_sources {
            source.bind_references(resolve_node, resolve_branch)?;
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.voltage_sources.is_empty() && self.current_sources.is_empty()
    }

    /// Total number of B-element instances, of either orientation.
    pub fn len(&self) -> usize {
        self.voltage_sources.len() + self.current_sources.len()
    }

    pub(crate) fn has_solution_dependent_sources(&self) -> bool {
        self.voltage_sources
            .iter()
            .any(BehavioralVoltageSource::is_solution_dependent)
            || self
                .current_sources
                .iter()
                .any(BehavioralCurrentSource::is_solution_dependent)
    }

    pub(crate) fn linearizations_converged(
        &mut self,
        solution: &[Value],
        time: Value,
        reltol: Value,
        voltage_abstol: Value,
        current_abstol: Value,
    ) -> Result<bool, BehavioralEvaluationError> {
        for source in &mut self.voltage_sources {
            if !source.linearization_converged(solution, time, reltol, voltage_abstol)? {
                return Ok(false);
            }
        }
        for source in &mut self.current_sources {
            if !source.linearization_converged(solution, time, reltol, current_abstol)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Commit stateful expression operators once at a successful timestep.
    pub(crate) fn accept_transient_step(
        &mut self,
        solution: &[Value],
        time: Value,
    ) -> Result<(), BehavioralEvaluationError> {
        let prepared = self.prepare_transient_step(solution, time)?;
        self.commit_transient_step(prepared);
        Ok(())
    }

    pub(crate) fn prepare_transient_step(
        &mut self,
        solution: &[Value],
        time: Value,
    ) -> Result<PreparedBehavioralStep, BehavioralEvaluationError> {
        let voltage = self
            .voltage_sources
            .iter_mut()
            .map(|source| source.prepare_transient_step(solution, time))
            .collect::<Result<_, _>>()?;
        let current = self
            .current_sources
            .iter_mut()
            .map(|source| source.prepare_transient_step(solution, time))
            .collect::<Result<_, _>>()?;
        Ok(PreparedBehavioralStep { voltage, current })
    }

    /// Promote already evaluated frames; no expression is executed here.
    pub(crate) fn commit_transient_step(&mut self, prepared: PreparedBehavioralStep) {
        debug_assert_eq!(self.voltage_sources.len(), prepared.voltage.len());
        debug_assert_eq!(self.current_sources.len(), prepared.current.len());
        for (source, vm) in self.voltage_sources.iter_mut().zip(prepared.voltage) {
            source.vm = vm;
        }
        for (source, vm) in self.current_sources.iter_mut().zip(prepared.current) {
            source.vm = vm;
        }
    }

    /// Stamp all behavioral sources
    pub fn stamp_all(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        num_nodes: usize,
        time: Value,
    ) -> Result<(), BehavioralEvaluationError> {
        for vs in &mut self.voltage_sources {
            vs.stamp(matrix, rhs, solution, num_nodes, time)?;
        }
        for cs in &mut self.current_sources {
            cs.stamp(matrix, rhs, solution, time)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn exact_voltage_constraint_requires_same_time_and_no_solution_dependencies() {
        let mut prescribed = BehavioralVoltageSource::new("Bfixed".to_string(), 1, 0, 1, "3.25")
            .expect("constant behavioral voltage source parses");
        let time = 1.25;
        prescribed
            .linearize_expression(&[], time)
            .expect("finite prescribed voltage");

        assert_eq!(prescribed.cached_exact_constraint_at(time), Some(3.25));
        assert_eq!(
            prescribed.cached_exact_constraint_at(Value::from_bits(time.to_bits() + 1)),
            None,
            "a constraint from a neighboring floating-point time is stale"
        );

        let mut dependent =
            BehavioralVoltageSource::new("Bdependent".to_string(), 1, 0, 1, "v(ctrl)+1")
                .expect("solution-dependent behavioral voltage source parses");
        dependent
            .bind_references(
                |name| (name == "ctrl").then_some(1),
                |_| BehavioralBranchResolution::MissingDevice,
            )
            .expect("solution-dependent source binds");
        dependent
            .linearize_expression(&[2.0], time)
            .expect("finite dependent voltage");

        assert_eq!(dependent.cached_exact_constraint_at(time), None);
    }

    #[test]
    fn exact_voltage_constraint_cache_is_invalidated_by_context_changes_and_nonfinite_values() {
        let mut source =
            BehavioralVoltageSource::new("Bcontext".to_string(), 1, 0, 1, "temper+freq+gmin")
                .expect("context-dependent behavioral voltage source parses");
        let time = 2.0;

        source
            .linearize_expression(&[], time)
            .expect("finite initial context");
        assert!(source.cached_exact_constraint_at(time).is_some());
        source.set_temperature(50.0);
        assert_eq!(source.cached_exact_constraint_at(time), None);

        source
            .linearize_expression(&[], time)
            .expect("finite temperature context");
        source.set_frequency(1.0e6);
        assert_eq!(source.cached_exact_constraint_at(time), None);

        source
            .linearize_expression(&[], time)
            .expect("finite frequency context");
        source.set_gmin(1.0e-9);
        assert_eq!(source.cached_exact_constraint_at(time), None);

        source
            .linearize_expression(&[], time)
            .expect("finite GMIN context");
        source.set_expression_dialect(ExpressionDialect::Xyce);
        assert_eq!(source.cached_exact_constraint_at(time), None);

        let mut nonfinite =
            BehavioralVoltageSource::new("Binf".to_string(), 1, 0, 1, "1e308*1e308")
                .expect("nonfinite behavioral expression parses");
        assert!(nonfinite.linearize_expression(&[], time).is_err());
        assert_eq!(nonfinite.cached_exact_constraint_at(time), None);

        let mut later_nonfinite =
            BehavioralVoltageSource::new("Blater".to_string(), 1, 0, 1, "exp(time*1000)")
                .expect("time-dependent behavioral expression parses");
        later_nonfinite
            .linearize_expression(&[], 0.0)
            .expect("initial expression is finite");
        assert!(later_nonfinite.cached_exact_constraint_at(0.0).is_some());
        assert!(later_nonfinite.evaluate(&[], 1.0).is_err());
        assert_eq!(later_nonfinite.cached_exact_constraint_at(0.0), None);
    }

    #[test]
    fn voltage_and_current_linearization_reject_nonfinite_derivatives() {
        let expression = "1e308*exp(2*v(ctrl))";
        let mut voltage = BehavioralVoltageSource::new("BV_DERIV".to_string(), 2, 0, 1, expression)
            .expect("behavioral voltage expression parses");
        voltage
            .bind_references(
                |name| (name == "ctrl").then_some(1),
                |_| BehavioralBranchResolution::MissingDevice,
            )
            .expect("voltage-source control binds");
        let voltage_error = voltage
            .linearize_expression(&[0.0, 0.0, 0.0], 0.0)
            .expect_err("overflowing voltage derivative must fail closed");
        assert!(voltage_error.to_string().contains("node derivative"));
        assert!(voltage_error.to_string().contains("BV_DERIV"));

        let mut current = BehavioralCurrentSource::new("BI_DERIV".to_string(), 2, 0, expression)
            .expect("behavioral current expression parses");
        current
            .bind_references(
                |name| (name == "ctrl").then_some(1),
                |_| BehavioralBranchResolution::MissingDevice,
            )
            .expect("current-source control binds");
        let current_error = current
            .linearize_expression(&[0.0, 0.0], 0.0)
            .expect_err("overflowing current derivative must fail closed");
        assert!(current_error.to_string().contains("node derivative"));
        assert!(current_error.to_string().contains("BI_DERIV"));
    }

    #[test]
    fn voltage_and_current_linearization_reject_nonfinite_affine_products() {
        let expression = "1.1e308*(v(ctrl)-1.7)";
        let mut voltage =
            BehavioralVoltageSource::new("BV_AFFINE".to_string(), 2, 0, 1, expression)
                .expect("behavioral voltage expression parses");
        voltage
            .bind_references(
                |name| (name == "ctrl").then_some(1),
                |_| BehavioralBranchResolution::MissingDevice,
            )
            .expect("voltage-source control binds");
        let voltage_error = voltage
            .linearize_expression(&[1.7, 0.0, 0.0], 0.0)
            .expect_err("overflowing voltage affine product must fail closed");
        assert!(voltage_error.to_string().contains("node affine product"));
        assert!(voltage_error.to_string().contains("BV_AFFINE"));

        let mut current = BehavioralCurrentSource::new("BI_AFFINE".to_string(), 2, 0, expression)
            .expect("behavioral current expression parses");
        current
            .bind_references(
                |name| (name == "ctrl").then_some(1),
                |_| BehavioralBranchResolution::MissingDevice,
            )
            .expect("current-source control binds");
        let current_error = current
            .linearize_expression(&[1.7, 0.0], 0.0)
            .expect_err("overflowing current affine product must fail closed");
        assert!(current_error.to_string().contains("node affine product"));
        assert!(current_error.to_string().contains("BI_AFFINE"));
    }

    #[test]
    fn voltage_and_current_stamps_reject_nonfinite_matrix_accumulation() {
        let expression = "1e308*v(ctrl)";
        let mut voltage = BehavioralVoltageSource::new("BV_STAMP".to_string(), 2, 0, 1, expression)
            .expect("behavioral voltage expression parses");
        voltage
            .bind_references(
                |name| (name == "ctrl").then_some(1),
                |_| BehavioralBranchResolution::MissingDevice,
            )
            .expect("voltage-source control binds");
        let mut voltage_matrix =
            StaticMatrix::from_triplets(3, 3, &[(2, 1, 0.0), (1, 2, 0.0), (2, 0, -Value::MAX)])
                .expect("voltage stamp matrix builds");
        let voltage_error = voltage
            .stamp(&mut voltage_matrix, &mut [0.0; 3], &[1.0, 0.0, 0.0], 2, 0.0)
            .expect_err("overflowing voltage matrix accumulation must fail closed");
        let voltage_message = voltage_error.to_string();
        assert!(voltage_message.contains("BV_STAMP"));
        assert!(voltage_message.contains("matrix coefficient -100000"));

        let mut current = BehavioralCurrentSource::new("BI_STAMP".to_string(), 2, 0, expression)
            .expect("behavioral current expression parses");
        current
            .bind_references(
                |name| (name == "ctrl").then_some(1),
                |_| BehavioralBranchResolution::MissingDevice,
            )
            .expect("current-source control binds");
        let mut current_matrix = StaticMatrix::from_triplets(2, 2, &[(1, 0, Value::MAX)])
            .expect("current stamp matrix builds");
        let current_error = current
            .stamp(&mut current_matrix, &mut [0.0; 2], &[1.0, 0.0], 0.0)
            .expect_err("overflowing current matrix accumulation must fail closed");
        let current_message = current_error.to_string();
        assert!(current_message.contains("BI_STAMP"));
        assert!(current_message.contains("matrix coefficient 100000"));
    }

    #[test]
    fn frequency_dependency_is_tracked_from_the_resolved_ast() {
        let voltage = BehavioralVoltageSource::new(
            "Bfreq".to_string(),
            1,
            0,
            1,
            "if(v(ctrl)>0, hertz, 2*freq)",
        )
        .expect("frequency-dependent voltage source parses");
        assert!(voltage.is_frequency_dependent());

        let probe_named_freq = BehavioralCurrentSource::new("Bprobe".to_string(), 1, 0, "v(freq)")
            .expect("probe named FREQ parses");
        assert!(!probe_named_freq.is_frequency_dependent());
    }

    #[test]
    fn per_frequency_refresh_skips_invariant_source_jacobians() {
        let mut invariant = BehavioralCurrentSource::new("Binvariant".to_string(), 1, 0, "2*v(n)")
            .expect("invariant source parses");
        invariant
            .bind_references(
                |name| (name == "n").then_some(1),
                |_| BehavioralBranchResolution::MissingDevice,
            )
            .expect("invariant source binds");
        invariant
            .linearize_at(&[3.0])
            .expect("finite invariant linearization");
        let initial_partials = invariant.linearized_partials().collect::<Vec<_>>();
        invariant
            .linearize_at_frequency(&[9.0], 100.0)
            .expect("finite invariant frequency preparation");
        assert_eq!(invariant.frequency, 0.0);
        assert_eq!(
            invariant.linearized_partials().collect::<Vec<_>>(),
            initial_partials
        );

        let mut dependent =
            BehavioralCurrentSource::new("Bdependent".to_string(), 1, 0, "freq*v(n)")
                .expect("frequency-dependent source parses");
        dependent
            .bind_references(
                |name| (name == "n").then_some(1),
                |_| BehavioralBranchResolution::MissingDevice,
            )
            .expect("frequency-dependent source binds");
        dependent
            .linearize_at(&[3.0])
            .expect("finite dependent linearization");
        dependent
            .linearize_at_frequency(&[3.0], 100.0)
            .expect("finite dependent frequency preparation");
        assert_eq!(dependent.frequency, 100.0);
        assert_eq!(
            dependent.linearized_partials().collect::<Vec<_>>(),
            vec![(0, 100.0)]
        );
    }

    #[test]
    fn xyce_power_analytic_derivative_matches_bytecode_and_finite_difference() {
        for (expression, point) in [
            ("v(n)**2.1", -2.5),
            ("pow(v(n),2.1)", -2.5),
            ("(-v(n))**3.1", 2.5),
            ("v(n)**-3", -2.0),
        ] {
            let (analytic_value, analytic_derivative) = eval_node_derivative(expression, point);
            let bytecode_value = eval_node_vm(expression, point, ExpressionDialect::Xyce);
            assert_eq!(
                analytic_value, bytecode_value,
                "value mismatch for {expression}"
            );

            let step = 1.0e-6 * point.abs().max(1.0);
            let numerical_derivative =
                (eval_node_vm(expression, point + step, ExpressionDialect::Xyce)
                    - eval_node_vm(expression, point - step, ExpressionDialect::Xyce))
                    / (2.0 * step);
            let scale = analytic_derivative
                .abs()
                .max(numerical_derivative.abs())
                .max(1.0);
            assert!(
                (analytic_derivative - numerical_derivative).abs() <= 2.0e-9 * scale,
                "derivative mismatch for {expression} at {point}: analytic={analytic_derivative:e}, numerical={numerical_derivative:e}"
            );
        }
    }

    #[test]
    fn xyce_power_exponent_derivative_matches_finite_difference() {
        let expression = "(-2.5)**v(n)";
        let point = 3.1;
        let (analytic_value, analytic_derivative) = eval_node_derivative(expression, point);
        assert_eq!(
            analytic_value,
            eval_node_vm(expression, point, ExpressionDialect::Xyce)
        );

        let step = 1.0e-6;
        let numerical_derivative =
            (eval_node_vm(expression, point + step, ExpressionDialect::Xyce)
                - eval_node_vm(expression, point - step, ExpressionDialect::Xyce))
                / (2.0 * step);
        assert!(
            (analytic_derivative - numerical_derivative).abs()
                <= 2.0e-9 * analytic_derivative.abs().max(1.0),
            "exponent derivative mismatch: analytic={analytic_derivative:e}, numerical={numerical_derivative:e}"
        );
    }

    #[test]
    fn xyce_analytic_boundary_normalizes_signed_nonfinite_value_and_derivative() {
        let positive = eval_node_derivative("0*exp(v(n))", 1000.0);
        assert_eq!(positive.0.abs(), 1.0e50);
        assert_eq!(positive.1.abs(), 1.0e50);
        let negative = eval_node_derivative("-(0*exp(v(n)))", 1000.0);
        assert_eq!(negative, (-positive.0, -positive.1));

        let (value, derivative) =
            eval_node_derivative_with_dialect("0*exp(v(n))", 1000.0, ExpressionDialect::Ngspice);
        assert!(value.is_nan());
        assert!(derivative.is_nan());
    }

    #[test]
    fn xyce_zero_base_power_has_zero_analytic_derivative_for_every_exponent_domain() {
        for expression in [
            "v(n)**-1",
            "v(n)**0.5",
            "v(n)**0",
            "pow(v(n),-1)",
            "0**v(n)",
        ] {
            let node_value = if expression == "0**v(n)" { -1.0 } else { 0.0 };
            let (value, derivative) = eval_node_derivative(expression, node_value);
            assert_eq!(derivative, 0.0, "zero-base slope changed for {expression}");
            assert_eq!(
                value,
                eval_node_vm(expression, node_value, ExpressionDialect::Xyce),
                "VM and analytic value differ for {expression}"
            );
        }

        assert_eq!(eval_node_derivative("v(n)**-1", 0.0).0, 1.0e50);
        assert_eq!(eval_node_derivative("v(n)**0.5", 0.0).0, 0.0);
        assert_eq!(eval_node_derivative("v(n)**0", 0.0).0, 1.0);
    }

    #[test]
    fn analytic_derivative_keeps_ustep_boundary_value_and_zero_slope() {
        assert_eq!(eval_const_derivative("stp(0)"), (0.0, 0.0));
        assert_eq!(eval_const_derivative("u(-1)"), (0.0, 0.0));
        assert_eq!(eval_const_derivative("u(0)"), (0.5, 0.0));
        assert_eq!(eval_const_derivative("u(1e-15)"), (1.0, 0.0));
        assert_eq!(eval_const_derivative("ustep(0)"), (0.5, 0.0));
    }

    #[test]
    fn analytic_derivative_supports_modulo_operator() {
        let (value, derivative) = eval_node_derivative("v(n)%2", 5.25);
        assert_eq!(value, 1.25);
        assert_eq!(derivative, 1.0);
    }

    #[test]
    fn analytic_derivative_distinguishes_int_from_floor_for_negative_values() {
        let (integer, integer_derivative) = eval_node_derivative("int(v(n))", -1.75);
        let (floor, floor_derivative) = eval_node_derivative("floor(v(n))", -1.75);

        assert_eq!(integer, -1.0);
        assert_eq!(floor, -2.0);
        assert_eq!(integer_derivative, 0.0);
        assert_eq!(floor_derivative, 0.0);
    }

    #[test]
    fn periodic_time_table_breakpoints_repeat_knots() {
        let ast =
            parse_expression_strict("table(time%120n,0,0,60n,3.3,100n,0)").expect("parse table");
        let ast = resolve_file_lookup_functions_with_limits(
            ast,
            None,
            crate::resource::ResourceLimits::default(),
        )
        .expect("constant inline table resolves");
        let breakpoints = expression_transient_breakpoints(&ast, 200.0e-9);

        for expected in [0.0, 60.0e-9, 100.0e-9, 120.0e-9, 180.0e-9] {
            assert!(
                breakpoints
                    .iter()
                    .any(|actual| (*actual - expected).abs() < 1.0e-18),
                "missing breakpoint {expected:e}; got {breakpoints:?}"
            );
        }
    }

    #[test]
    fn inline_lookup_jacobians_follow_xyce_and_voltage_inputs_add_no_time_knots() {
        let mut akima = BehavioralVoltageSource::new(
            "Bakima".to_string(),
            1,
            0,
            1,
            "akima(v(a),1,4,0.5,2.25,0,1)",
        )
        .expect("inline Akima source resolves");
        akima.set_expression_dialect(ExpressionDialect::Xyce);
        akima
            .bind_references(
                |name| name.eq_ignore_ascii_case("a").then_some(1),
                |_| BehavioralBranchResolution::MissingDevice,
            )
            .expect("Akima input binds");
        akima
            .linearize_at(&[0.3])
            .expect("finite Akima linearization");
        let (_, derivative) = akima
            .linearized_partials()
            .next()
            .expect("Akima source has one node partial");
        assert!((derivative - 2.6).abs() < 2.0e-14, "{derivative}");
        assert!(expression_transient_breakpoints(&akima.ast, 1.0).is_empty());

        let step = 1.0e-6;
        let mut vm = Vm::new();
        let upper = vm.execute(
            &akima.program,
            &Context::dc(&[0.3 + step], &[]).with_expression_dialect(ExpressionDialect::Xyce),
        );
        let lower = vm.execute(
            &akima.program,
            &Context::dc(&[0.3 - step], &[]).with_expression_dialect(ExpressionDialect::Xyce),
        );
        let numerical = (upper - lower) / (2.0 * step);
        assert!((derivative - numerical).abs() < 2.0e-9, "{numerical}");

        let mut table = BehavioralVoltageSource::new(
            "Btable".to_string(),
            1,
            0,
            1,
            "table(v(a),1,3,0.5,2,0,1)",
        )
        .expect("inline table source resolves");
        table.set_expression_dialect(ExpressionDialect::Xyce);
        table
            .bind_references(
                |name| name.eq_ignore_ascii_case("a").then_some(1),
                |_| BehavioralBranchResolution::MissingDevice,
            )
            .expect("table input binds");
        table
            .linearize_at(&[0.1])
            .expect("finite table linearization");
        let (_, derivative) = table
            .linearized_partials()
            .next()
            .expect("table source has one node partial");
        assert!((derivative - 0.8).abs() < 2.0e-14, "{derivative}");
        assert!(expression_transient_breakpoints(&table.ast, 1.0).is_empty());
    }

    #[test]
    fn xyce_barycentric_knot_derivative_does_not_poison_an_inactive_direction() {
        let dir = unique_temp_dir("behavioral-barycentric-inactive-direction");
        std::fs::create_dir_all(&dir).expect("create temp table directory");
        std::fs::write(dir.join("curve.dat"), "0 1\n1 2\n2 5\n")
            .expect("write barycentric table data");
        let deck_path = dir.join("deck.cir");

        let mut source = BehavioralVoltageSource::new_with_source_path(
            "Bbli".to_string(),
            1,
            0,
            1,
            "bli(\"curve.dat\") + 0*v(a)",
            Some(&deck_path),
        )
        .expect("barycentric source resolves");
        source.set_expression_dialect(ExpressionDialect::Xyce);
        source
            .bind_references(
                |name| name.eq_ignore_ascii_case("a").then_some(1),
                |_| BehavioralBranchResolution::MissingDevice,
            )
            .expect("inactive voltage reference binds");

        source
            .linearize_at(&[7.0])
            .expect("finite file lookup linearization");
        let (_, derivative) = source
            .linearized_partials()
            .next()
            .expect("source retains its inactive node partial");
        assert_eq!(derivative.to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn file_table_voltage_source_excludes_output_from_generic_voltage_lte() {
        let dir = unique_temp_dir("behavioral-file-table-lte");
        std::fs::create_dir_all(&dir).expect("create temp table directory");
        std::fs::write(dir.join("wave.dat"), "0 0\n1e-6 1\n").expect("write table data");
        let deck_path = dir.join("deck.cir");

        let source = BehavioralVoltageSource::new_with_source_path(
            "B1".to_string(),
            1,
            0,
            1,
            "tablefile(\"wave.dat\")",
            Some(&deck_path),
        )
        .expect("file table behavioral voltage source parses");

        assert!(source.excludes_output_from_transient_voltage_lte());
        assert!(!source.is_solution_dependent());
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("rspice-{label}-{unique}"))
    }

    fn eval_const_derivative(expression: &str) -> (Value, Value) {
        let ast = parse_expression_strict(expression)
            .unwrap_or_else(|err| panic!("parse `{expression}` failed: {err}"));
        let program = compile(&ast);
        let context = BehavioralDerivativeContext {
            program: &program,
            node_values: &[],
            branch_values: &[],
            time: 0.0,
            frequency: 0.0,
            temperature: 27.0,
            gmin: crate::constants::GMIN,
            expression_dialect: ExpressionDialect::Ngspice,
            target: DerivativeTarget::Node(0),
        };
        eval_behavioral_expr_with_derivative(&ast, &context)
            .map(|(value, derivative)| (value, derivative.binary64()))
            .unwrap_or_else(|| panic!("analytic derivative for `{expression}` failed"))
    }

    #[test]
    fn vcvs_initial_source_derivatives_follow_implicit_waveform_defaults_and_corners() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            for (expression, time, expected) in [
                ("2*spice_sin(0,1,1)", 0.0, 2.0 * std::f64::consts::TAU),
                ("sin(2*pi*time)^1", 0.0, std::f64::consts::TAU),
                (
                    "if(sin(2*pi*time)<=0,2*sin(2*pi*time),sin(2*pi*time))",
                    0.0,
                    std::f64::consts::TAU,
                ),
                ("abs(-sin(2*pi*time))", 0.0, std::f64::consts::TAU),
                (
                    "min(sin(2*pi*time),-sin(2*pi*time))",
                    0.0,
                    -std::f64::consts::TAU,
                ),
                (
                    "max(sin(2*pi*time),2*sin(2*pi*time))",
                    0.0,
                    2.0 * std::f64::consts::TAU,
                ),
                ("spice_pulse(0,3,0,1,2,1,5)", 0.0, 3.0),
                ("spice_pulse(0,3,0,1,2,1,5)", 1.0, 0.0),
                ("spice_pulse(0,3,0,1,2,1,5)", 2.0, -1.5),
                ("spice_pulse(0,3,0,1,2,1,5)", 5.0, 3.0),
                ("spice_exp(0,2,0.5,0.25,0.75,0.5)", 0.5, 8.0),
                (
                    "spice_exp(0,2,0.5,0.25,0.75,0.5)",
                    0.75,
                    8.0 * (-1.0_f64).exp() - 4.0,
                ),
                (
                    "spice_sffm(0,0.2,3,0.5,2)",
                    0.0,
                    0.8 * std::f64::consts::TAU,
                ),
            ] {
                let mut source =
                    BehavioralVoltageSource::new("B1".to_owned(), 1, 0, 1, expression).unwrap();
                source.expression_dialect = dialect;
                let actual = source.explicit_time_derivative(time).unwrap();
                assert!(
                    (actual - expected).abs() < 4e-13 * expected.abs().max(1.0),
                    "{expression} at {time}: {actual} vs {expected}"
                );
            }
            let mut source = BehavioralVoltageSource::new(
                "B1".to_owned(),
                1,
                0,
                1,
                "spice_pulse(0,1,0,0,1,1,3)",
            )
            .unwrap();
            source.expression_dialect = dialect;
            assert_eq!(source.explicit_time_derivative(0.0), None);
            for expression in [
                "if(sin(2*pi*time)>0,1,0)",
                "if(sin(2*pi*time)^2>0,time,0)",
                // Newton slopes cannot certify these outgoing derivatives.
                "floor(-sin(2*pi*time))",
                "ln(sin(2*pi*time)-1)",
                "sqrt(sin(2*pi*time)^2)",
                "table(sin(2*pi*time),0,0,1,1)",
                "if(!sin(2*pi*time),2*sin(2*pi*time),sin(2*pi*time))",
                "spice_sin(0,sin(2*pi*time)^2,1)",
                "exp(1000+sin(2*pi*time))",
            ] {
                let mut source =
                    BehavioralVoltageSource::new("B1".to_owned(), 1, 0, 1, expression).unwrap();
                source.expression_dialect = dialect;
                assert_eq!(source.explicit_time_derivative(0.0), None);
            }
        }
    }

    #[test]
    fn analytic_primitive_coefficients_preserve_finite_scaled_derivatives() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let log_slope = if dialect == ExpressionDialect::Xyce {
                1.0 / std::f64::consts::LN_10
            } else {
                1.0
            };
            for (expression, point, expected) in [
                ("asinh(1e200*v(in))", 1.0, 1.0),
                ("asinh(1e200*v(in))", -1.0, 1.0),
                ("asinh(v(in))", 0.0, 1.0),
                ("acosh(1e308*v(in))", 1.0, 1.0),
                ("acosh(v(in))", 2.0, 1.0 / 3.0_f64.sqrt()),
                ("log10(1e308*v(in))", 1.0, 1.0 / std::f64::consts::LN_10),
                ("log(1e308*v(in))", 1.0, log_slope),
            ] {
                let (_, actual) = eval_node_derivative_with_dialect(expression, point, dialect);
                assert!(
                    (actual / expected - 1.0).abs() < 3e-15,
                    "{dialect:?} {expression}: {actual:e}, expected {expected:e}"
                );
            }
        }
    }

    #[test]
    fn expression_directions_preserve_weighted_and_retained_parameter_derivatives() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            for (expression, values, directions, expected_value, expected_derivative) in [
                (
                    "1+1e-8*v(p)+1e200*v(q)",
                    [0.0, 0.0],
                    [2.0.into(), (-1e-208).into()],
                    1.0,
                    1e-8,
                ),
                (
                    "1+1e-200*v(q)",
                    [0.0, 0.0],
                    [0.0.into(), Derivative::from(1e200) * 1e200],
                    1.0,
                    1e200,
                ),
                (
                    "if(v(p)>0,v(q)*v(q),v(q))",
                    [-1.0, 3.0],
                    [0.0.into(), 2.0.into()],
                    3.0,
                    2.0,
                ),
                (
                    "if(v(p)>0,v(q)*v(q),v(q))",
                    [1.0, 3.0],
                    [0.0.into(), 2.0.into()],
                    9.0,
                    12.0,
                ),
            ] {
                let ast = crate::expr::parse_expression_strict(expression).unwrap();
                let program = compile(&ast);
                let mut inputs = vec![0.0; program.node_map.len()];
                let mut incoming = vec![0.0.into(); inputs.len()];
                for (name, index) in &program.node_map {
                    let parameter = if name.eq_ignore_ascii_case("p") { 0 } else { 1 };
                    inputs[*index] = values[parameter];
                    incoming[*index] = directions[parameter];
                }
                let (value, derivative) = compiled_expression_node_direction(
                    &ast,
                    &program,
                    &inputs,
                    &incoming,
                    BehavioralEnvironment {
                        time: 0.0,
                        frequency: 0.0,
                        temperature: 27.0,
                        gmin: crate::constants::GMIN,
                        expression_dialect: dialect,
                    },
                )
                .unwrap();
                assert_eq!(value, expected_value, "{dialect:?} {expression}");
                assert!(
                    (derivative.binary64() / expected_derivative - 1.0).abs() < 2e-15,
                    "{dialect:?} {expression}: {:?}, expected {expected_derivative}",
                    derivative
                );
            }
        }
    }

    #[test]
    fn ddx_user_functions_preserve_formal_scope_and_lazy_branches() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let mut parameters = crate::netlist::expr::ParamContext::new();
            parameters.set_expression_dialect(dialect);
            parameters.set("P", 4.0);
            parameters.set("Q", 3.0);
            parameters.define_function("INNER", vec!["P".to_owned()], "P+Q");
            parameters.define_function("OUTER", vec!["Q".to_owned()], "INNER(Q)+P");
            parameters.define_function("LAZY", vec!["X".to_owned()], "IF(X>0,X*X,MISSING)");
            let targets = vec!["p".to_owned()];
            for (source, expected) in [
                ("OUTER(2*P)", 3.0),
                ("OUTER(2)", 1.0),
                ("LAZY(P)", 8.0),
                ("IF(P>0,0,MISSING)", 0.0),
            ] {
                let actual =
                    evaluate_parameter_directional_derivative(source, &parameters, &targets)
                        .unwrap_or_else(|error| panic!("{dialect:?} {source}: {error}"));
                assert_eq!(actual, expected, "{dialect:?} {source}");
            }
            let error =
                evaluate_parameter_directional_derivative("INNER(2)", &parameters, &targets)
                    .unwrap_err();
            assert!(
                error.contains("absent"),
                "formal P is not a root target: {error}"
            );
            parameters.set_complex("Q", crate::ComplexValue::new(3.0, 1e-30));
            assert!(
                evaluate_parameter_directional_derivative("P*Q", &parameters, &targets)
                    .unwrap_err()
                    .contains("real-valued")
            );
        }
    }

    #[test]
    fn ddx_prepared_traversal_preserves_scalar_kernel_semantics() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let mut parameters = crate::netlist::expr::ParamContext::new();
            parameters.set_expression_dialect(dialect);
            let targets = vec!["p".to_owned()];
            for (source, point) in [
                ("1+1e-8*v(p)", 0.0),
                ("log(v(p))+ln(v(p))", 1e-50),
                ("tanh(21+v(p))", 0.0),
                ("sqrt(v(p))", -1.0),
                ("min(v(p),2)+max(-v(p),-3)", 1.0),
                ("table(v(p),0,0,2,3,4,9)", 3.0),
                ("if(v(p)>0,exp(v(p)),pow(v(p),2))", -2.0),
                ("round(v(p))+sign(v(p),-1)", 2.5),
            ] {
                parameters.set("P", point);
                let ast = crate::expr::parse_expression_strict(source).unwrap();
                let program = compile(&ast);
                let (_, expected) = compiled_expression_node_direction(
                    &ast,
                    &program,
                    &[point],
                    &[1.0.into()],
                    BehavioralEnvironment {
                        time: 0.0,
                        frequency: 0.0,
                        temperature: 27.0,
                        gmin: crate::constants::GMIN,
                        expression_dialect: dialect,
                    },
                )
                .unwrap();
                let actual = evaluate_parameter_directional_derivative(
                    &source.replace("v(p)", "p"),
                    &parameters,
                    &targets,
                )
                .unwrap();
                assert_eq!(
                    actual,
                    normalize_expression_boundary(expected.binary64(), dialect),
                    "{dialect:?} {source} at {point}",
                );
            }
        }
    }

    #[test]
    fn ddx_aliases_share_one_parameter_direction() {
        let mut parameters = crate::netlist::ParamContext::new();
        parameters.set_expression_dialect(ExpressionDialect::Xyce);
        parameters.set("a", 0.0);
        parameters.set("b", 0.0);
        let targets = ["a".to_owned(), "B".to_owned()];
        let expression = "1+1e-8*(a+b)";
        let derivative =
            evaluate_parameter_directional_derivative(expression, &parameters, &targets).unwrap();
        assert!((derivative / 2e-8 - 1.0).abs() < 2e-15);
        parameters.set("b", 1.0);
        assert!(
            evaluate_parameter_directional_derivative(expression, &parameters, &targets)
                .unwrap_err()
                .contains("different values")
        );
    }

    #[test]
    fn tanh_analytic_derivative_preserves_saturation_tails() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            for point in [
                0.0_f64, 1.0, -1.0, 19.0, -19.0, 20.0, -20.0, 21.0, -21.0, 400.0, -400.0,
            ] {
                let expression = format!("tanh({point}+1e300*v(in))");
                let (_, actual) = eval_node_derivative_with_dialect(&expression, 0.0, dialect);
                let expected = if dialect == ExpressionDialect::Xyce && point.abs() > 20.0 {
                    0.0
                } else {
                    (1e300 / point.cosh()) / point.cosh()
                };
                if expected == 0.0 {
                    assert_eq!(actual, 0.0, "{dialect:?} {expression}");
                } else {
                    assert!(
                        (actual / expected - 1.0).abs() < 5e-15,
                        "{dialect:?} {expression}: {actual:e}, expected {expected:e}"
                    );
                }
            }
            for point in [-1000.0, 1000.0] {
                let expression =
                    format!("tanh({point}+1e200*(1e200*(1e200*(1e200*(1e200*v(in))))))");
                let (_, actual) = eval_node_derivative_with_dialect(&expression, 0.0, dialect);
                if dialect == ExpressionDialect::Xyce {
                    assert_eq!(actual, 0.0);
                } else {
                    // cosh(1000) = 2*cosh(500)^2-1; the -1 is negligible
                    // here. This reference keeps every intermediate finite.
                    let ratio = 1e250 / 500.0_f64.cosh();
                    let expected = (ratio * ratio).powi(2) / 4.0;
                    assert!(
                        (actual / expected - 1.0).abs() < 5e-15,
                        "{expression}: {actual:e}, expected {expected:e}"
                    );
                }
            }
        }
    }

    #[test]
    fn analytic_power_coefficients_preserve_finite_scaled_derivatives() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            for (expression, expected) in [
                ("pow(1e-300*v(in),-0.03)*1e-9", -0.03),
                ("pow(-1e-200*v(in),2)*1e200", 2e-200),
                ("(-1e-200*v(in))^2*1e200", 2e-200),
                ("pwrs(-1e-200*v(in),2)*1e200", -2e-200),
                (
                    "pwr(-1e-200*v(in),2)*1e200",
                    if dialect == ExpressionDialect::Xyce {
                        2e-200
                    } else {
                        -2e-200
                    },
                ),
            ] {
                let (_, actual) = eval_node_derivative_with_dialect(expression, 1.0, dialect);
                assert!(
                    (actual / expected - 1.0).abs() < 3e-15,
                    "{dialect:?} {expression}: {actual:e}, expected {expected:e}"
                );
            }
            for expression in ["v(in)^0", "pow(v(in),0)"] {
                assert_eq!(
                    eval_node_derivative_with_dialect(expression, 0.0, dialect),
                    (1.0, 0.0)
                );
            }
        }
    }

    #[test]
    fn analytic_derivatives_retain_intermediate_exponents() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            for (expression, point, expected) in [
                ("1e-200*(1e200*(1e200*v(in)))", 0.0, 1e200),
                ("1e200*(1e-200*(1e-200*v(in)))", 0.0, 1e-200),
                ("(1e308*v(in)+1e308*v(in))*1e-308", 0.0, 2.0),
                ("((1e200*v(in))/1e-200)*1e-200", 0.0, 1e200),
                ("atan(1e200*(1e200*v(in)))*1e-200", 0.0, 1e200),
                ("pow(1e200*v(in),2)*1e-200", 1e-50, 2e150),
                (
                    "exp(709*v(in))*1e-307",
                    1.0,
                    (709.0_f64.exp() * 1e-307) * 709.0,
                ),
                (
                    "sqrt(5e-324*v(in))*1e162",
                    1.0,
                    Value::from_bits(1).sqrt() * 1e162 * 0.5,
                ),
            ] {
                let (_, actual) = eval_node_derivative_with_dialect(expression, point, dialect);
                assert!(
                    (actual / expected - 1.0).abs() < 3e-15,
                    "{dialect:?} {expression}: {actual:e}, expected {expected:e}"
                );
            }
        }
    }

    #[test]
    fn analytic_derivatives_preserve_finite_ratios_across_extreme_scales() {
        let mut failures = Vec::new();
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            for (expression, expected) in [
                ("v(in)/1e200", 1e-200_f64),
                ("v(in)/1e-200", 1e200),
                ("(1e-200*v(in))/1e-200", 1.0),
                ("(1e200*v(in))/(1e200*v(in))", 0.0),
                ("atan2(1e200*v(in),1e200)", 0.5),
                ("atan2(1e-200*v(in),1e-200)", 0.5),
                ("atan(1e200*v(in))", 1e-200),
            ] {
                let (_, actual) = eval_node_derivative_with_dialect(expression, 1.0, dialect);
                let accurate = if expected == 0.0 {
                    actual == 0.0
                } else {
                    (actual / expected - 1.0).abs() < 2e-15
                };
                if !accurate {
                    failures.push(format!(
                        "{dialect:?} {expression}: {actual}, expected {expected}"
                    ));
                }
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    fn eval_node_derivative(expression: &str, node_value: Value) -> (Value, Value) {
        eval_node_derivative_with_dialect(expression, node_value, ExpressionDialect::Xyce)
    }

    fn eval_node_derivative_with_dialect(
        expression: &str,
        node_value: Value,
        expression_dialect: ExpressionDialect,
    ) -> (Value, Value) {
        let ast = parse_expression_strict(expression)
            .unwrap_or_else(|err| panic!("parse `{expression}` failed: {err}"));
        let program = compile(&ast);
        let context = BehavioralDerivativeContext {
            program: &program,
            node_values: &[node_value],
            branch_values: &[],
            time: 0.0,
            frequency: 0.0,
            temperature: 27.0,
            gmin: crate::constants::GMIN,
            expression_dialect,
            target: DerivativeTarget::Node(0),
        };
        eval_behavioral_expr_with_derivative_at_boundary(&ast, &context)
            .unwrap_or_else(|| panic!("analytic derivative for `{expression}` failed"))
    }

    fn eval_node_vm(expression: &str, node_value: Value, dialect: ExpressionDialect) -> Value {
        let ast = parse_expression_strict(expression)
            .unwrap_or_else(|err| panic!("parse `{expression}` failed: {err}"));
        let program = compile(&ast);
        Vm::new().execute(
            &program,
            &Context::dc(&[node_value], &[]).with_expression_dialect(dialect),
        )
    }

    #[test]
    fn xyce_sign_and_limit_match_between_vm_and_analytic_derivatives() {
        for (expression, expected_value, expected_derivative) in [
            ("sign(V(n),2)", 3.0, -1.0),
            ("sign(V(n),-2)", -3.0, 1.0),
            ("sign(V(n),0)", 0.0, 0.0),
            ("limit(V(n),0.5)", -3.0, 1.0),
            ("limit(V(n),0,2)", 0.0, 0.0),
        ] {
            assert_eq!(
                eval_node_vm(expression, -3.0, ExpressionDialect::Xyce),
                expected_value,
                "VM value for {expression}"
            );
            assert_eq!(
                eval_node_derivative(expression, -3.0),
                (expected_value, expected_derivative),
                "analytic value/derivative for {expression}"
            );
        }

        assert_eq!(
            eval_node_derivative("sign(V(n),2)", 0.0),
            (0.0, 1.0),
            "Xyce SIGN uses the nonnegative derivative branch at zero"
        );
        assert_eq!(
            eval_node_derivative("limit(V(n),2,0)", 1.0),
            (2.0, 0.0),
            "reversed LIMIT bounds preserve Xyce's ordered first branch"
        );
    }

    #[test]
    fn signed_magnitude_zero_branch_matches_between_vm_and_analytic_evaluation() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            assert_eq!(eval_node_vm("pwrs(0*V(n),0)", 3.0, dialect), 1.0);
            assert_eq!(
                eval_node_derivative_with_dialect("pwrs(0*V(n),0)", 3.0, dialect),
                (1.0, 0.0)
            );
        }
        assert_eq!(
            eval_node_derivative_with_dialect("pwr(0*V(n),0)", 3.0, ExpressionDialect::Ngspice,),
            (1.0, 0.0)
        );
    }
}
