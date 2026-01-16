//! Behavioral sources (B-elements)
//!
//! Implements voltage and current sources defined by arbitrary expressions.
//! Expressions are compiled to bytecode during circuit build for efficient
//! evaluation in the Newton-Raphson loop.

use crate::Value;
use crate::expr::{parse_expression, compile, CompiledExpr, Vm, Context};
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
    /// Branch index for MNA
    pub branch_index: usize,
    /// Compiled expression
    pub program: CompiledExpr,
    /// VM for evaluation
    vm: Vm,
}

impl BehavioralVoltageSource {
    /// Create a new behavioral voltage source
    pub fn new(name: String, node_pos: usize, node_neg: usize, branch_index: usize, expression: &str) -> Self {
        let ast = parse_expression(expression);
        let program = compile(&ast);
        
        Self {
            name,
            node_pos,
            node_neg,
            branch_index,
            program,
            vm: Vm::new(),
        }
    }

    /// Evaluate the expression with current voltages/currents
    pub fn evaluate(&mut self, voltages: &[Value], currents: &[Value], time: Value) -> Value {
        let ctx = Context::transient(voltages, currents, time);
        self.vm.execute(&self.program, &ctx)
    }

    /// Stamp into the matrix (MNA voltage source with computed value)
    pub fn stamp(&mut self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value], currents: &[Value], time: Value) {
        let v_value = self.evaluate(voltages, currents, time);
        let br = self.branch_index;
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
}

impl BehavioralCurrentSource {
    /// Create a new behavioral current source
    pub fn new(name: String, node_pos: usize, node_neg: usize, expression: &str) -> Self {
        let ast = parse_expression(expression);
        let program = compile(&ast);
        
        Self {
            name,
            node_pos,
            node_neg,
            program,
            vm: Vm::new(),
        }
    }

    /// Evaluate the expression with current voltages/currents
    pub fn evaluate(&mut self, voltages: &[Value], currents: &[Value], time: Value) -> Value {
        let ctx = Context::transient(voltages, currents, time);
        self.vm.execute(&self.program, &ctx)
    }

    /// Stamp into the RHS (current source stamps directly)
    pub fn stamp(&mut self, rhs: &mut [Value], voltages: &[Value], currents: &[Value], time: Value) {
        let i_value = self.evaluate(voltages, currents, time);
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

    pub fn is_empty(&self) -> bool {
        self.voltage_sources.is_empty() && self.current_sources.is_empty()
    }

    /// Stamp all behavioral sources
    pub fn stamp_all(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        currents: &[Value],
        time: Value,
    ) {
        for vs in &mut self.voltage_sources {
            vs.stamp(matrix, rhs, voltages, currents, time);
        }
        for cs in &mut self.current_sources {
            cs.stamp(rhs, voltages, currents, time);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_behavioral_voltage_simple() {
        let mut bvs = BehavioralVoltageSource::new(
            "B1".to_string(),
            1, 0, 1,
            "5.0"
        );
        
        let voltages = [0.0];
        let currents = [];
        let v = bvs.evaluate(&voltages, &currents, 0.0);
        assert!((v - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_behavioral_voltage_expression() {
        let mut bvs = BehavioralVoltageSource::new(
            "B1".to_string(),
            1, 0, 1,
            "2 * 3 + 1"
        );
        
        let v = bvs.evaluate(&[], &[], 0.0);
        assert!((v - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_behavioral_current_simple() {
        let mut bcs = BehavioralCurrentSource::new(
            "B1".to_string(),
            1, 0,
            "0.001"
        );
        
        let i = bcs.evaluate(&[], &[], 0.0);
        assert!((i - 0.001).abs() < 1e-12);
    }
}
