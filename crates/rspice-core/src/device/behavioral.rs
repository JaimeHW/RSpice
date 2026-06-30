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
    /// Circuit temperature in degrees Celsius, surfaced as `temper`.
    temperature: Value,
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
            temperature: crate::analysis::temperature::kelvin_to_celsius(
                crate::constants::TEMP_REFERENCE,
            ),
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
            let resolved = if crate::compat::ground::is_spice_ground_name(name) {
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
        let ctx = Context::transient(&self.node_values, &self.branch_values, time)
            .with_temperature(self.temperature);
        self.vm.execute(&self.program, &ctx)
    }

    /// Set the circuit temperature (degrees Celsius) surfaced as `temper`.
    pub fn set_temperature(&mut self, temperature: Value) {
        self.temperature = temperature;
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

    /// Refresh the linearization (value and partials) at the given
    /// operating point for small-signal assembly. AC has no time axis;
    /// expressions see t = 0.
    pub(crate) fn linearize_at(&mut self, solution: &[Value]) {
        let _ = self.linearize_expression(solution, 0.0);
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
#[derive(Debug, Clone)]
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
    /// Circuit temperature in degrees Celsius, surfaced as `temper`.
    temperature: Value,
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
            temperature: crate::analysis::temperature::kelvin_to_celsius(
                crate::constants::TEMP_REFERENCE,
            ),
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
            let resolved = if crate::compat::ground::is_spice_ground_name(name) {
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
        let ctx = Context::transient(&self.node_values, &self.branch_values, time)
            .with_temperature(self.temperature);
        self.vm.execute(&self.program, &ctx)
    }

    /// Set the circuit temperature (degrees Celsius) surfaced as `temper`.
    pub fn set_temperature(&mut self, temperature: Value) {
        self.temperature = temperature;
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

    /// Refresh the linearization (value and partials) at the given
    /// operating point for small-signal assembly. AC has no time axis;
    /// expressions see t = 0.
    pub(crate) fn linearize_at(&mut self, solution: &[Value]) {
        let _ = self.linearize_expression(solution, 0.0);
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
#[derive(Debug, Clone, Default)]
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
