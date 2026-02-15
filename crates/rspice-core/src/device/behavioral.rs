//! Behavioral sources (B-elements)
//!
//! Implements voltage and current sources defined by arbitrary expressions.
//! Expressions are compiled to bytecode during circuit build for efficient
//! evaluation in the Newton-Raphson loop.

use crate::Value;
use crate::expr::{CompiledExpr, Context, Vm, compile, parse_expression_strict};
use crate::solver::StaticMatrix;

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
        let ctx = Context::transient(&self.node_values, &self.branch_values, time);
        self.vm.execute(&self.program, &ctx)
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
        let v_value = self.evaluate(solution, time);
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

        // RHS: branch equation
        rhs[br - 1] = v_value;
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
        let ctx = Context::transient(&self.node_values, &self.branch_values, time);
        self.vm.execute(&self.program, &ctx)
    }

    /// Stamp into the RHS (current source stamps directly)
    pub fn stamp(&mut self, rhs: &mut [Value], solution: &[Value], time: Value) {
        let i_value = self.evaluate(solution, time);
        let np = self.node_pos;
        let nn = self.node_neg;

        // Current source: current flows from n+ to n-
        if np > 0 {
            rhs[np - 1] -= i_value;
        }
        if nn > 0 {
            rhs[nn - 1] += i_value;
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
            cs.stamp(rhs, solution, time);
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
