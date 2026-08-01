//! Capacitor device model

use crate::config::ExpressionDialect;
use crate::device::behavioral::{
    compiled_expression_branch_partial, compiled_expression_node_partial,
};
use crate::device::traits::{DynamicDevice, MatrixStamper};
use crate::expr::{
    CompiledExpr, Context, Expr, Vm, compile, parse_expression_strict,
    resolve_file_lookup_functions_with_limits,
};
use crate::{NodeId, Value};
use std::path::Path;

const DERIVATIVE_REL_STEP: Value = 1.0e-6;
const DERIVATIVE_ABS_STEP: Value = 1.0e-9;

/// Value and Jacobian entries produced by a solution-dependent capacitor
/// expression at one trial solution.
#[derive(Debug, Clone, PartialEq)]
pub struct SolutionDependentCapacitorLinearization {
    /// Evaluated capacitance expression value.
    pub value: Value,
    /// `(global_solution_index, dC/dx)` entries for bound node voltages and
    /// branch currents referenced by the expression.
    pub partials: Vec<(usize, Value)>,
}

/// Compiled evaluator for a solution-dependent capacitor value expression.
///
/// Xyce permits the explicit capacitance (`C=`/`VALUE=`) of a capacitor to be
/// a behavioral expression involving circuit state, for example
/// `C={1p+V(ctrl)}`.  The ordinary [`Capacitor`] model intentionally remains a
/// compact numeric companion-model device; this evaluator owns only the
/// expression program and its bindings so the circuit storage can retain the
/// expression until a later stamping phase evaluates it.
///
/// The evaluator follows the same expression VM and reference-binding rules
/// as behavioral sources.  It does not stamp a device or apply capacitor
/// validity/temperature/multiplicity policy; those decisions belong to the
/// circuit builder and transient integrator.
#[derive(Debug, Clone)]
pub struct SolutionDependentCapacitor {
    /// Capacitor instance name, used to provide contextual binding errors.
    pub name: String,
    /// Original behavioral value expression for diagnostics and introspection.
    pub expression: String,
    /// Compiled expression program.
    pub program: CompiledExpr,
    /// Parsed expression used by the shared analytic derivative evaluator.
    ast: Expr,
    vm: Vm,
    node_bindings: Vec<Option<usize>>,
    branch_bindings: Vec<Option<usize>>,
    node_values: Vec<Value>,
    branch_values: Vec<Value>,
    temperature: Value,
    frequency: Value,
    gmin: Value,
    expression_dialect: ExpressionDialect,
}

impl SolutionDependentCapacitor {
    /// Compile a solution-dependent capacitor value expression.
    pub fn new(name: String, expression: &str) -> Result<Self, String> {
        Self::new_with_source_path_and_limits(
            name,
            expression,
            None,
            crate::resource::ResourceLimits::default(),
        )
    }

    /// Compile an expression with deck-relative file-function support.
    pub fn new_with_source_path(
        name: String,
        expression: &str,
        source_path: Option<&Path>,
    ) -> Result<Self, String> {
        Self::new_with_source_path_and_limits(
            name,
            expression,
            source_path,
            crate::resource::ResourceLimits::default(),
        )
    }

    /// Compile an expression with an explicit resource policy for file-backed
    /// lookup functions.
    pub fn new_with_source_path_and_limits(
        name: String,
        expression: &str,
        source_path: Option<&Path>,
        resource_limits: crate::resource::ResourceLimits,
    ) -> Result<Self, String> {
        let ast = parse_expression_strict(expression).map_err(|error| {
            format!(
                "Invalid capacitor value expression '{}': {}",
                expression, error
            )
        })?;
        let ast = resolve_file_lookup_functions_with_limits(ast, source_path, resource_limits)
            .map_err(|error| {
                format!(
                    "Invalid capacitor value expression '{}': {}",
                    expression, error
                )
            })?;
        let program = compile(&ast);

        Ok(Self {
            name,
            expression: expression.to_string(),
            program,
            ast,
            vm: Vm::new(),
            node_bindings: Vec::new(),
            branch_bindings: Vec::new(),
            node_values: Vec::new(),
            branch_values: Vec::new(),
            temperature: crate::constants::kelvin_to_celsius(crate::constants::TEMP_REFERENCE),
            frequency: 0.0,
            gmin: crate::constants::GMIN,
            expression_dialect: ExpressionDialect::Ngspice,
        })
    }

    /// Resolve V(...) and I(...) references against circuit solution indices.
    pub fn bind_references<FN, FB>(
        &mut self,
        resolve_node: FN,
        resolve_branch: FB,
    ) -> Result<(), String>
    where
        FN: Fn(&str) -> Option<usize>,
        FB: Fn(&str) -> Option<usize>,
    {
        self.node_bindings = vec![None; self.program.node_map.len()];
        for (name, &local_idx) in &self.program.node_map {
            let resolved = if crate::naming::is_spice_ground_name(name) {
                Some(0usize)
            } else {
                resolve_node(name)
            }
            .ok_or_else(|| {
                format!(
                    "Solution-dependent capacitor '{}' references unknown node '{}'",
                    self.name, name
                )
            })?;
            self.node_bindings[local_idx] = resolved.checked_sub(1);
        }

        self.branch_bindings = vec![None; self.program.branch_map.len()];
        for (name, &local_idx) in &self.program.branch_map {
            let resolved = resolve_branch(name).ok_or_else(|| {
                format!(
                    "Solution-dependent capacitor '{}' references unknown branch source '{}'",
                    self.name, name
                )
            })?;
            self.branch_bindings[local_idx] = Some(resolved);
        }

        self.node_values.resize(self.node_bindings.len(), 0.0);
        self.branch_values.resize(self.branch_bindings.len(), 0.0);
        Ok(())
    }

    #[inline]
    fn refresh_expression_inputs(&mut self, solution: &[Value]) {
        for (index, binding) in self.node_bindings.iter().enumerate() {
            self.node_values[index] = binding
                .and_then(|global_index| solution.get(global_index).copied())
                .unwrap_or(0.0);
        }
        for (index, binding) in self.branch_bindings.iter().enumerate() {
            self.branch_values[index] = binding
                .and_then(|global_index| solution.get(global_index).copied())
                .unwrap_or(0.0);
        }
    }

    /// Evaluate the capacitance expression against the current circuit state.
    pub fn evaluate(&mut self, solution: &[Value], time: Value) -> Value {
        self.refresh_expression_inputs(solution);
        self.evaluate_with_cached_inputs(time)
    }

    #[inline]
    fn evaluate_with_cached_inputs(&mut self, time: Value) -> Value {
        let context = Context::transient(&self.node_values, &self.branch_values, time)
            .with_frequency(self.frequency)
            .with_temperature(self.temperature)
            .with_gmin(self.gmin)
            .with_expression_dialect(self.expression_dialect);
        self.vm.execute(&self.program, &context)
    }

    #[inline]
    fn derivative_step(base: Value) -> Value {
        DERIVATIVE_ABS_STEP + DERIVATIVE_REL_STEP * base.abs().max(1.0)
    }

    fn estimate_node_partial(&mut self, index: usize, f0: Value, time: Value) -> Value {
        let base = self.node_values[index];
        let step = Self::derivative_step(base);
        self.node_values[index] = base + step;
        let plus = self.evaluate_with_cached_inputs(time);
        self.node_values[index] = base - step;
        let minus = self.evaluate_with_cached_inputs(time);
        self.node_values[index] = base;

        let derivative = if plus.is_finite() && minus.is_finite() {
            (plus - minus) / (2.0 * step)
        } else if plus.is_finite() && f0.is_finite() {
            (plus - f0) / step
        } else if minus.is_finite() && f0.is_finite() {
            (f0 - minus) / step
        } else {
            0.0
        };
        if derivative.is_finite() { derivative } else { 0.0 }
    }

    fn estimate_branch_partial(&mut self, index: usize, f0: Value, time: Value) -> Value {
        let base = self.branch_values[index];
        let step = Self::derivative_step(base);
        self.branch_values[index] = base + step;
        let plus = self.evaluate_with_cached_inputs(time);
        self.branch_values[index] = base - step;
        let minus = self.evaluate_with_cached_inputs(time);
        self.branch_values[index] = base;

        let derivative = if plus.is_finite() && minus.is_finite() {
            (plus - minus) / (2.0 * step)
        } else if plus.is_finite() && f0.is_finite() {
            (plus - f0) / step
        } else if minus.is_finite() && f0.is_finite() {
            (f0 - minus) / step
        } else {
            0.0
        };
        if derivative.is_finite() { derivative } else { 0.0 }
    }

    /// Evaluate and linearize the capacitance expression at a trial solution.
    ///
    /// Analytic derivatives share the behavioral-source expression evaluator.
    /// Expressions whose operators do not expose an analytic derivative use
    /// the same central finite-difference step and one-sided non-finite
    /// fallback as [`crate::device::BehavioralCurrentSource`].
    pub fn linearize(
        &mut self,
        solution: &[Value],
        time: Value,
    ) -> SolutionDependentCapacitorLinearization {
        self.refresh_expression_inputs(solution);
        let value = self.evaluate_with_cached_inputs(time);
        let mut partials =
            Vec::with_capacity(self.node_bindings.len() + self.branch_bindings.len());

        if !value.is_finite() {
            return SolutionDependentCapacitorLinearization { value, partials };
        }

        for index in 0..self.node_bindings.len() {
            let Some(global_index) = self.node_bindings[index] else {
                continue;
            };
            let derivative = compiled_expression_node_partial(
                &self.ast,
                &self.program,
                &self.node_values,
                &self.branch_values,
                time,
                self.frequency,
                self.temperature,
                self.gmin,
                self.expression_dialect,
                index,
            )
            .unwrap_or_else(|| self.estimate_node_partial(index, value, time));
            partials.push((global_index, derivative));
        }

        for index in 0..self.branch_bindings.len() {
            let Some(global_index) = self.branch_bindings[index] else {
                continue;
            };
            let derivative = compiled_expression_branch_partial(
                &self.ast,
                &self.program,
                &self.node_values,
                &self.branch_values,
                time,
                self.frequency,
                self.temperature,
                self.gmin,
                self.expression_dialect,
                index,
            )
            .unwrap_or_else(|| self.estimate_branch_partial(index, value, time));
            partials.push((global_index, derivative));
        }

        SolutionDependentCapacitorLinearization { value, partials }
    }

    /// Commit stateful expression operators after an accepted transient step.
    pub fn accept_transient_step(&mut self, solution: &[Value], time: Value) {
        let _ = self.evaluate(solution, time);
        self.vm.accept_transient_step(time);
    }

    /// Return the global solution indices referenced by this expression.
    pub fn bound_solution_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.node_bindings
            .iter()
            .chain(self.branch_bindings.iter())
            .filter_map(|binding| *binding)
    }

    /// Whether the expression reads a circuit node voltage or branch current.
    pub fn is_solution_dependent(&self) -> bool {
        !self.program.node_map.is_empty() || !self.program.branch_map.is_empty()
    }

    /// Set the circuit temperature (degrees Celsius) surfaced as `temper`.
    pub fn set_temperature(&mut self, temperature: Value) {
        self.temperature = temperature;
    }

    /// Set the active analysis frequency in hertz.
    pub fn set_frequency(&mut self, frequency: Value) {
        self.frequency = frequency;
    }

    /// Set the active nonlinear minimum conductance surfaced as `GMIN`.
    pub fn set_gmin(&mut self, gmin: Value) {
        self.gmin = gmin;
    }

    /// Set dialect-specific expression-function semantics.
    pub fn set_expression_dialect(&mut self, dialect: ExpressionDialect) {
        self.expression_dialect = dialect;
    }
}

/// Capacitor with companion model for transient analysis
#[derive(Debug, Clone)]
pub struct Capacitor {
    pub name: String,
    pub node_pos: NodeId,
    pub node_neg: NodeId,
    pub capacitance: Value,
    /// Voltage across capacitor at previous time step
    voltage_prev: Value,
    /// Companion model equivalent current source
    ieq: Value,
}

impl Capacitor {
    pub fn new(name: String, node_pos: NodeId, node_neg: NodeId, capacitance: Value) -> Self {
        Self {
            name,
            node_pos,
            node_neg,
            capacitance,
            voltage_prev: 0.0,
            ieq: 0.0,
        }
    }

    /// Get equivalent conductance for trapezoidal integration
    pub fn geq(&self, dt: Value) -> Value {
        2.0 * self.capacitance / dt
    }
}

impl DynamicDevice for Capacitor {
    fn stamp_transient(
        &self,
        _voltages: &[Value],
        dt: Value,
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        // Trapezoidal companion model:
        // i = geq * v + ieq
        // where geq = 2C/dt and ieq = 2C/dt * v_prev + i_prev

        let geq = self.geq(dt);

        // Stamp conductance
        matrix.stamp(self.node_pos, self.node_pos, geq);
        matrix.stamp(self.node_pos, self.node_neg, -geq);
        matrix.stamp(self.node_neg, self.node_pos, -geq);
        matrix.stamp(self.node_neg, self.node_neg, geq);

        // Stamp current source (ieq)
        matrix.stamp_rhs(self.node_pos, -self.ieq);
        matrix.stamp_rhs(self.node_neg, self.ieq);
    }

    fn step(&mut self, voltages: &[Value], dt: Value) {
        let v_pos = if self.node_pos == 0 {
            0.0
        } else {
            voltages[self.node_pos - 1]
        };
        let v_neg = if self.node_neg == 0 {
            0.0
        } else {
            voltages[self.node_neg - 1]
        };
        let v = v_pos - v_neg;

        // Update companion model current source for next step
        // For trapezoidal: ieq = 2C/dt * v + i_prev = 2C/dt * v + 2C/dt * v_prev + ieq_prev
        let geq = self.geq(dt);
        self.ieq += geq * v + geq * self.voltage_prev;

        self.voltage_prev = v;
    }
}

#[cfg(test)]
mod solution_dependent_capacitor_tests {
    use super::*;

    #[test]
    fn binds_and_evaluates_voltage_and_branch_references() {
        let mut evaluator =
            SolutionDependentCapacitor::new("C1".to_string(), "1e-6 + V(ctrl) + I(V1)")
                .expect("solution-dependent capacitor expression parses");
        assert!(evaluator.is_solution_dependent());

        evaluator
            .bind_references(
                |name| (name.eq_ignore_ascii_case("ctrl")).then_some(1),
                |name| (name.eq_ignore_ascii_case("V1")).then_some(1),
            )
            .expect("solution-dependent capacitor references bind");

        let value = evaluator.evaluate(&[2.0, 3.0], 0.0);
        assert!((value - 5.000001).abs() < 1e-12, "evaluated value: {value}");
        let linearization = evaluator.linearize(&[2.0, 3.0], 0.0);
        assert!((linearization.value - value).abs() < 1e-12);
        assert_eq!(linearization.partials, vec![(0, 1.0), (1, 1.0)]);
        let mut bound = evaluator.bound_solution_indices().collect::<Vec<_>>();
        bound.sort_unstable();
        assert_eq!(bound, vec![0, 1]);
    }

    #[test]
    fn stateful_operator_uses_finite_difference_derivative_fallback() {
        let mut evaluator = SolutionDependentCapacitor::new("C1".to_string(), "sdt(V(ctrl))")
            .expect("stateful capacitor expression parses");
        evaluator
            .bind_references(
                |name| (name.eq_ignore_ascii_case("ctrl")).then_some(1),
                |_| None,
            )
            .expect("stateful capacitor reference binds");
        assert_eq!(evaluator.evaluate(&[2.0], 0.0), 0.0);
        evaluator.accept_transient_step(&[2.0], 0.0);

        let linearization = evaluator.linearize(&[4.0], 1.0);
        assert!((linearization.value - 3.0).abs() < 1e-12);
        assert_eq!(linearization.partials.len(), 1);
        assert!((linearization.partials[0].1 - 0.5).abs() < 1e-7);
    }

    #[test]
    fn reports_unknown_references_with_capacitor_context() {
        let mut evaluator = SolutionDependentCapacitor::new("Cbad".to_string(), "V(missing)")
            .expect("solution-dependent capacitor expression parses");
        let error = evaluator
            .bind_references(|_| None, |_| None)
            .expect_err("unknown capacitor probe must be rejected");
        assert!(error.contains("Cbad"), "error: {error}");
        assert!(error.contains("missing"), "error: {error}");
    }
}
