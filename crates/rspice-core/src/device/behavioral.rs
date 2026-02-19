//! Behavioral sources (B-elements)
//!
//! Implements voltage and current sources defined by arbitrary expressions.
//! Expressions are compiled to bytecode during circuit build for efficient
//! evaluation in the Newton-Raphson loop.

use crate::Value;
use crate::expr::{CompiledExpr, Context, Vm, compile, parse_expression_strict};
use crate::solver::StaticMatrix;

const DERIVATIVE_REL_STEP: Value = 1e-6;
const DERIVATIVE_ABS_STEP: Value = 1e-9;

/// Compiled behavioral voltage source
#[derive(Debug)]
pub struct BehavioralVoltageSource {
    /// Device name
    pub name: String,
    /// Positive node
    pub node_pos: usize,
    /// Negative node  
    pub node_neg: usize,
    /// Branch ordinal for MNA (1-based, converted to matrix index at stamp time)
    pub branch_ordinal: usize,
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
    /// Linearization partials d(expr)/d(node_values[idx])
    node_partials: Vec<Value>,
    /// Linearization partials d(expr)/d(branch_values[idx])
    branch_partials: Vec<Value>,
}

impl BehavioralVoltageSource {
    /// Create a new behavioral voltage source
    pub fn new(
        name: String,
        node_pos: usize,
        node_neg: usize,
        branch_ordinal: usize,
        expression: &str,
    ) -> Result<Self, String> {
        let ast = parse_expression_strict(expression)
            .map_err(|e| format!("Invalid behavioral expression '{}': {}", expression, e))?;
        let program = compile(&ast);

        Ok(Self {
            name,
            node_pos,
            node_neg,
            branch_ordinal,
            program,
            vm: Vm::new(),
            node_bindings: Vec::new(),
            branch_bindings: Vec::new(),
            node_values: Vec::new(),
            branch_values: Vec::new(),
            node_partials: Vec::new(),
            branch_partials: Vec::new(),
        })
    }

    /// Resolve V(...) and I(...) references against circuit node/branch indices.
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
            let resolved = if name.eq_ignore_ascii_case("0") || name.eq_ignore_ascii_case("gnd") {
                Some(0usize)
            } else {
                resolve_node(name)
            }
            .ok_or_else(|| {
                format!(
                    "Behavioral source '{}' references unknown node '{}'",
                    self.name, name
                )
            })?;
            self.node_bindings[local_idx] = resolved.checked_sub(1);
        }

        self.branch_bindings = vec![None; self.program.branch_map.len()];
        for (name, &local_idx) in &self.program.branch_map {
            let resolved = resolve_branch(name).ok_or_else(|| {
                format!(
                    "Behavioral source '{}' references unknown branch source '{}'",
                    self.name, name
                )
            })?;
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
    pub fn evaluate(&mut self, solution: &[Value], time: Value) -> Value {
        self.refresh_expression_inputs(solution);
        self.evaluate_with_cached_inputs(time)
    }

    #[inline]
    pub(crate) fn bound_solution_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.node_bindings
            .iter()
            .chain(self.branch_bindings.iter())
            .filter_map(|binding| *binding)
    }

    #[inline]
    fn evaluate_with_cached_inputs(&mut self, time: Value) -> Value {
        let ctx = Context::transient(&self.node_values, &self.branch_values, time);
        self.vm.execute(&self.program, &ctx)
    }

    #[inline]
    fn derivative_step(base: Value) -> Value {
        DERIVATIVE_ABS_STEP + DERIVATIVE_REL_STEP * base.abs().max(1.0)
    }

    #[inline]
    fn estimate_node_partial(&mut self, idx: usize, f0: Value, time: Value) -> Value {
        let base = self.node_values[idx];
        let h = Self::derivative_step(base);
        self.node_values[idx] = base + h;
        let fp = self.evaluate_with_cached_inputs(time);
        self.node_values[idx] = base - h;
        let fm = self.evaluate_with_cached_inputs(time);
        self.node_values[idx] = base;

        let mut df = if fp.is_finite() && fm.is_finite() {
            (fp - fm) / (2.0 * h)
        } else if fp.is_finite() && f0.is_finite() {
            (fp - f0) / h
        } else if fm.is_finite() && f0.is_finite() {
            (f0 - fm) / h
        } else {
            0.0
        };
        if !df.is_finite() {
            df = 0.0;
        }
        df
    }

    #[inline]
    fn estimate_branch_partial(&mut self, idx: usize, f0: Value, time: Value) -> Value {
        let base = self.branch_values[idx];
        let h = Self::derivative_step(base);
        self.branch_values[idx] = base + h;
        let fp = self.evaluate_with_cached_inputs(time);
        self.branch_values[idx] = base - h;
        let fm = self.evaluate_with_cached_inputs(time);
        self.branch_values[idx] = base;

        let mut df = if fp.is_finite() && fm.is_finite() {
            (fp - fm) / (2.0 * h)
        } else if fp.is_finite() && f0.is_finite() {
            (fp - f0) / h
        } else if fm.is_finite() && f0.is_finite() {
            (f0 - fm) / h
        } else {
            0.0
        };
        if !df.is_finite() {
            df = 0.0;
        }
        df
    }

    fn linearize_expression(&mut self, solution: &[Value], time: Value) -> Value {
        self.refresh_expression_inputs(solution);
        let f0 = self.evaluate_with_cached_inputs(time);

        if !f0.is_finite() {
            self.node_partials.fill(0.0);
            self.branch_partials.fill(0.0);
            return 0.0;
        }

        for idx in 0..self.node_bindings.len() {
            self.node_partials[idx] = if self.node_bindings[idx].is_some() {
                self.estimate_node_partial(idx, f0, time)
            } else {
                0.0
            };
        }
        for idx in 0..self.branch_bindings.len() {
            self.branch_partials[idx] = if self.branch_bindings[idx].is_some() {
                self.estimate_branch_partial(idx, f0, time)
            } else {
                0.0
            };
        }

        let mut affine = f0;
        for (idx, binding) in self.node_bindings.iter().enumerate() {
            if let Some(global_idx) = binding {
                affine -= self.node_partials[idx] * solution[*global_idx];
            }
        }
        for (idx, binding) in self.branch_bindings.iter().enumerate() {
            if let Some(global_idx) = binding {
                affine -= self.branch_partials[idx] * solution[*global_idx];
            }
        }
        if !affine.is_finite() { 0.0 } else { affine }
    }

    /// Stamp into the matrix (MNA voltage source with computed value)
    pub fn stamp(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        num_nodes: usize,
        time: Value,
    ) {
        let v_affine = self.linearize_expression(solution, time);
        let br = num_nodes + self.branch_ordinal;
        let np = self.node_pos;
        let nn = self.node_neg;

        // Standard voltage source MNA stamping
        // Branch equation: V(n+) - V(n-) = v_value
        if np > 0 {
            matrix.add(br - 1, np - 1, 1.0);
            matrix.add(np - 1, br - 1, 1.0);
        }
        if nn > 0 {
            matrix.add(br - 1, nn - 1, -1.0);
            matrix.add(nn - 1, br - 1, -1.0);
        }

        // Linearized behavioral dependency terms on branch equation row:
        // V(np)-V(nn)-f(x) = 0 => row(br): ... + (-df/dx)*x = affine
        for (idx, binding) in self.node_bindings.iter().enumerate() {
            if let Some(global_idx) = binding {
                let df = self.node_partials[idx];
                if df != 0.0 {
                    matrix.add(br - 1, *global_idx, -df);
                }
            }
        }
        for (idx, binding) in self.branch_bindings.iter().enumerate() {
            if let Some(global_idx) = binding {
                let df = self.branch_partials[idx];
                if df != 0.0 {
                    matrix.add(br - 1, *global_idx, -df);
                }
            }
        }

        // RHS: branch equation
        rhs[br - 1] = v_affine;
    }
}

/// Compiled behavioral current source
#[derive(Debug)]
pub struct BehavioralCurrentSource {
    /// Device name
    pub name: String,
    /// Positive node (current flows into)
    pub node_pos: usize,
    /// Negative node (current flows out of)
    pub node_neg: usize,
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
    /// Linearization partials d(expr)/d(node_values[idx])
    node_partials: Vec<Value>,
    /// Linearization partials d(expr)/d(branch_values[idx])
    branch_partials: Vec<Value>,
}

impl BehavioralCurrentSource {
    /// Create a new behavioral current source
    pub fn new(
        name: String,
        node_pos: usize,
        node_neg: usize,
        expression: &str,
    ) -> Result<Self, String> {
        let ast = parse_expression_strict(expression)
            .map_err(|e| format!("Invalid behavioral expression '{}': {}", expression, e))?;
        let program = compile(&ast);

        Ok(Self {
            name,
            node_pos,
            node_neg,
            program,
            vm: Vm::new(),
            node_bindings: Vec::new(),
            branch_bindings: Vec::new(),
            node_values: Vec::new(),
            branch_values: Vec::new(),
            node_partials: Vec::new(),
            branch_partials: Vec::new(),
        })
    }

    /// Resolve V(...) and I(...) references against circuit node/branch indices.
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
            let resolved = if name.eq_ignore_ascii_case("0") || name.eq_ignore_ascii_case("gnd") {
                Some(0usize)
            } else {
                resolve_node(name)
            }
            .ok_or_else(|| {
                format!(
                    "Behavioral source '{}' references unknown node '{}'",
                    self.name, name
                )
            })?;
            self.node_bindings[local_idx] = resolved.checked_sub(1);
        }

        self.branch_bindings = vec![None; self.program.branch_map.len()];
        for (name, &local_idx) in &self.program.branch_map {
            let resolved = resolve_branch(name).ok_or_else(|| {
                format!(
                    "Behavioral source '{}' references unknown branch source '{}'",
                    self.name, name
                )
            })?;
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
    pub fn evaluate(&mut self, solution: &[Value], time: Value) -> Value {
        self.refresh_expression_inputs(solution);
        self.evaluate_with_cached_inputs(time)
    }

    #[inline]
    pub(crate) fn bound_solution_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.node_bindings
            .iter()
            .chain(self.branch_bindings.iter())
            .filter_map(|binding| *binding)
    }

    #[inline]
    fn evaluate_with_cached_inputs(&mut self, time: Value) -> Value {
        let ctx = Context::transient(&self.node_values, &self.branch_values, time);
        self.vm.execute(&self.program, &ctx)
    }

    #[inline]
    fn derivative_step(base: Value) -> Value {
        DERIVATIVE_ABS_STEP + DERIVATIVE_REL_STEP * base.abs().max(1.0)
    }

    #[inline]
    fn estimate_node_partial(&mut self, idx: usize, f0: Value, time: Value) -> Value {
        let base = self.node_values[idx];
        let h = Self::derivative_step(base);
        self.node_values[idx] = base + h;
        let fp = self.evaluate_with_cached_inputs(time);
        self.node_values[idx] = base - h;
        let fm = self.evaluate_with_cached_inputs(time);
        self.node_values[idx] = base;

        let mut df = if fp.is_finite() && fm.is_finite() {
            (fp - fm) / (2.0 * h)
        } else if fp.is_finite() && f0.is_finite() {
            (fp - f0) / h
        } else if fm.is_finite() && f0.is_finite() {
            (f0 - fm) / h
        } else {
            0.0
        };
        if !df.is_finite() {
            df = 0.0;
        }
        df
    }

    #[inline]
    fn estimate_branch_partial(&mut self, idx: usize, f0: Value, time: Value) -> Value {
        let base = self.branch_values[idx];
        let h = Self::derivative_step(base);
        self.branch_values[idx] = base + h;
        let fp = self.evaluate_with_cached_inputs(time);
        self.branch_values[idx] = base - h;
        let fm = self.evaluate_with_cached_inputs(time);
        self.branch_values[idx] = base;

        let mut df = if fp.is_finite() && fm.is_finite() {
            (fp - fm) / (2.0 * h)
        } else if fp.is_finite() && f0.is_finite() {
            (fp - f0) / h
        } else if fm.is_finite() && f0.is_finite() {
            (f0 - fm) / h
        } else {
            0.0
        };
        if !df.is_finite() {
            df = 0.0;
        }
        df
    }

    fn linearize_expression(&mut self, solution: &[Value], time: Value) -> Value {
        self.refresh_expression_inputs(solution);
        let f0 = self.evaluate_with_cached_inputs(time);

        if !f0.is_finite() {
            self.node_partials.fill(0.0);
            self.branch_partials.fill(0.0);
            return 0.0;
        }

        for idx in 0..self.node_bindings.len() {
            self.node_partials[idx] = if self.node_bindings[idx].is_some() {
                self.estimate_node_partial(idx, f0, time)
            } else {
                0.0
            };
        }
        for idx in 0..self.branch_bindings.len() {
            self.branch_partials[idx] = if self.branch_bindings[idx].is_some() {
                self.estimate_branch_partial(idx, f0, time)
            } else {
                0.0
            };
        }

        let mut affine = f0;
        for (idx, binding) in self.node_bindings.iter().enumerate() {
            if let Some(global_idx) = binding {
                affine -= self.node_partials[idx] * solution[*global_idx];
            }
        }
        for (idx, binding) in self.branch_bindings.iter().enumerate() {
            if let Some(global_idx) = binding {
                affine -= self.branch_partials[idx] * solution[*global_idx];
            }
        }
        if !affine.is_finite() { 0.0 } else { affine }
    }

    /// Stamp linearized behavioral current source into matrix and RHS.
    pub fn stamp(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
    ) {
        let i_affine = self.linearize_expression(solution, time);
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
                        matrix.add(np - 1, *global_idx, df);
                    }
                    if nn > 0 {
                        matrix.add(nn - 1, *global_idx, -df);
                    }
                }
            }
        }
        for (idx, binding) in self.branch_bindings.iter().enumerate() {
            if let Some(global_idx) = binding {
                let df = self.branch_partials[idx];
                if df != 0.0 {
                    if np > 0 {
                        matrix.add(np - 1, *global_idx, df);
                    }
                    if nn > 0 {
                        matrix.add(nn - 1, *global_idx, -df);
                    }
                }
            }
        }

        if np > 0 {
            rhs[np - 1] -= i_affine;
        }
        if nn > 0 {
            rhs[nn - 1] += i_affine;
        }
    }
}

/// Storage for behavioral sources (not SoA due to compiled programs)
#[derive(Debug, Default)]
pub struct BehavioralSources {
    pub voltage_sources: Vec<BehavioralVoltageSource>,
    pub current_sources: Vec<BehavioralCurrentSource>,
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

    /// Resolve expression V(...) and I(...) references for all behavioral sources.
    pub fn bind_references<FN, FB>(
        &mut self,
        resolve_node: FN,
        resolve_branch: FB,
    ) -> Result<(), String>
    where
        FN: Fn(&str) -> Option<usize> + Copy,
        FB: Fn(&str) -> Option<usize> + Copy,
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

    /// Stamp all behavioral sources
    pub fn stamp_all(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        num_nodes: usize,
        time: Value,
    ) {
        for vs in &mut self.voltage_sources {
            vs.stamp(matrix, rhs, solution, num_nodes, time);
        }
        for cs in &mut self.current_sources {
            cs.stamp(matrix, rhs, solution, time);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_behavioral_voltage_simple() {
        let mut bvs = BehavioralVoltageSource::new("B1".to_string(), 1, 0, 1, "5.0")
            .expect("valid expression should parse");
        let v = bvs.evaluate(&[], 0.0);
        assert!((v - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_behavioral_voltage_expression() {
        let mut bvs = BehavioralVoltageSource::new("B1".to_string(), 1, 0, 1, "2 * 3 + 1")
            .expect("valid expression should parse");
        let v = bvs.evaluate(&[], 0.0);
        assert!((v - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_behavioral_current_simple() {
        let mut bcs = BehavioralCurrentSource::new("B1".to_string(), 1, 0, "0.001")
            .expect("valid expression should parse");
        let i = bcs.evaluate(&[], 0.0);
        assert!((i - 0.001).abs() < 1e-12);
    }

    #[test]
    fn test_behavioral_binding_resolves_node_and_branch_references() {
        let mut bvs = BehavioralVoltageSource::new("B1".to_string(), 2, 0, 1, "V(out) + I(VS)")
            .expect("valid expression should parse");
        bvs.bind_references(
            |name| if name == "out" { Some(2) } else { None },
            |name| {
                if name.eq_ignore_ascii_case("vs") {
                    Some(2)
                } else {
                    None
                }
            },
        )
        .expect("binding should resolve all references");

        let solution = [0.5, 1.25, -0.2];
        let evaluated = bvs.evaluate(&solution, 0.0);
        assert!((evaluated - 1.05).abs() < 1e-12);
    }

    #[test]
    fn test_behavioral_binding_rejects_unknown_references() {
        let mut bcs = BehavioralCurrentSource::new("B2".to_string(), 1, 0, "V(missing) + I(NOPE)")
            .expect("valid expression should parse");
        let err = bcs
            .bind_references(|_| None, |_| None)
            .expect_err("binding should reject unknown references");
        assert!(
            err.contains("unknown node") || err.contains("unknown branch"),
            "unexpected bind error: {}",
            err
        );
    }

    #[test]
    fn test_behavioral_current_stamp_linearization_signs_match_newton_form() {
        let mut bcs = BehavioralCurrentSource::new("B1".to_string(), 1, 0, "V(n1)")
            .expect("valid expression should parse");
        bcs.bind_references(|name| (name == "n1").then_some(1), |_| None)
            .expect("binding should resolve node reference");

        let mut matrix =
            StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).expect("valid 1x1 topology");
        let idx = matrix.get_index(0, 0).expect("matrix entry missing");
        let mut rhs = vec![0.0];
        let solution = [0.25];

        bcs.stamp(&mut matrix, &mut rhs, &solution, 0.0);

        let jac = matrix.values_mut()[idx.0];
        assert!(
            (jac - 1.0).abs() < 1e-6,
            "expected +dI/dV at np row, got {}",
            jac
        );
        assert!(
            rhs[0].abs() < 1e-9,
            "expected zero affine RHS for I=V linear source, got {}",
            rhs[0]
        );
    }

    #[test]
    fn test_behavioral_current_stamp_linearization_affine_term() {
        let mut bcs = BehavioralCurrentSource::new("B1".to_string(), 1, 0, "sqr(V(n1))")
            .expect("valid expression should parse");
        bcs.bind_references(|name| (name == "n1").then_some(1), |_| None)
            .expect("binding should resolve node reference");

        let mut matrix =
            StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).expect("valid 1x1 topology");
        let idx = matrix.get_index(0, 0).expect("matrix entry missing");
        let mut rhs = vec![0.0];
        let solution = [0.4];

        bcs.stamp(&mut matrix, &mut rhs, &solution, 0.0);

        let jac = matrix.values_mut()[idx.0];
        let expected_didv = 0.8;
        let expected_ieq = 0.16 - expected_didv * 0.4; // ieq = I - dI/dV * V = -0.16
        let expected_rhs = -expected_ieq; // np row contribution
        assert!(
            (jac - expected_didv).abs() < 1e-4,
            "expected dI/dV ~ {}, got {}",
            expected_didv,
            jac
        );
        assert!(
            (rhs[0] - expected_rhs).abs() < 1e-4,
            "expected affine RHS ~ {}, got {}",
            expected_rhs,
            rhs[0]
        );
    }

    #[test]
    fn test_behavioral_voltage_stamp_linearization_terms() {
        let mut bvs = BehavioralVoltageSource::new("B1".to_string(), 2, 0, 1, "2*V(ctrl)")
            .expect("valid expression should parse");
        bvs.bind_references(|name| (name == "ctrl").then_some(1), |_| None)
            .expect("binding should resolve node reference");

        // 2 nodes + 1 branch = matrix size 3.
        let mut matrix = StaticMatrix::from_triplets(
            3,
            3,
            &[
                (2, 1, 0.0), // branch equation row to output node
                (1, 2, 0.0), // output node row to branch current
                (2, 0, 0.0), // derivative term dVexpr/dV(ctrl)
            ],
        )
        .expect("valid topology for behavioral voltage source");
        let idx_deriv = matrix
            .get_index(2, 0)
            .expect("missing derivative stamp entry");
        let mut rhs = vec![0.0; 3];
        let solution = [0.5, 0.0, 0.0];

        bvs.stamp(&mut matrix, &mut rhs, &solution, 2, 0.0);

        let deriv = matrix.values_mut()[idx_deriv.0];
        assert!(
            (deriv + 2.0).abs() < 1e-4,
            "expected branch row coupling -dVexpr/dV(ctrl)=-2, got {}",
            deriv
        );
        assert!(
            rhs[2].abs() < 1e-9,
            "expected zero affine RHS for Vexpr=2*V(ctrl) at expansion point, got {}",
            rhs[2]
        );
    }

    #[test]
    fn test_behavioral_new_rejects_invalid_expression() {
        let err = BehavioralVoltageSource::new("Bbad".to_string(), 1, 0, 1, "V(1) @ 2")
            .expect_err("invalid expression should fail constructor");
        assert!(
            err.contains("Invalid behavioral expression"),
            "unexpected error: {}",
            err
        );
    }
}
