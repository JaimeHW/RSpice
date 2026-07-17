//! Behavioral sources (B-elements)
//!
//! Implements voltage and current sources defined by arbitrary expressions.
//! Expressions are compiled to bytecode during circuit build for efficient
//! evaluation in the Newton-Raphson loop.

use crate::Value;
use crate::expr::{
    BinaryOp, CompiledExpr, Context, Expr, Function, UnaryOp, Vm, compile,
    normalize_expression_boundary, parse_expression_strict, real_pow_with_derivative,
    resolve_file_lookup_functions_with_limits,
};
use crate::netlist::ExpressionDialect;
use crate::solver::StaticMatrix;
use std::path::Path;

const DERIVATIVE_REL_STEP: Value = 1e-6;
const DERIVATIVE_ABS_STEP: Value = 1e-9;
const EXPR_ZERO_TOLERANCE: Value = 1.0e-12;
const XYCE_ATANH_EPSILON: Value = 1.0e-12;
const XYCE_TANH_SATURATION_THRESHOLD: Value = 20.0;

#[derive(Clone, Copy)]
enum DerivativeTarget {
    Node(usize),
    Branch(usize),
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
    target: DerivativeTarget,
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
    /// Linearization partials d(expr)/d(node_values[idx])
    node_partials: Vec<Value>,
    /// Linearization partials d(expr)/d(branch_values[idx])
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
    /// True when the expression can jump discontinuously as its inputs cross a predicate.
    transient_voltage_lte_excluded: bool,
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
            temperature: crate::analysis::temperature::kelvin_to_celsius(
                crate::constants::TEMP_REFERENCE,
            ),
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

    /// Evaluate and commit stateful expression operators at an accepted point.
    pub(crate) fn accept_transient_step(&mut self, solution: &[Value], time: Value) {
        let _ = self.evaluate(solution, time);
        self.vm.accept_transient_step(time);
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

    pub(crate) fn transient_breakpoints(&self, tstop: Value, _tstep_hint: Value) -> Vec<Value> {
        expression_transient_breakpoints(&self.ast, tstop)
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
            .with_expression_dialect(self.expression_dialect);
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
            self.linearized_affine = 0.0;
            return 0.0;
        }

        for idx in 0..self.node_bindings.len() {
            self.node_partials[idx] = if self.node_bindings[idx].is_some() {
                analytic_expression_partial(
                    &self.ast,
                    &self.program,
                    &self.node_values,
                    &self.branch_values,
                    time,
                    self.frequency,
                    self.temperature,
                    self.gmin,
                    self.expression_dialect,
                    DerivativeTarget::Node(idx),
                )
                .unwrap_or_else(|| self.estimate_node_partial(idx, f0, time))
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
                    time,
                    self.frequency,
                    self.temperature,
                    self.gmin,
                    self.expression_dialect,
                    DerivativeTarget::Branch(idx),
                )
                .unwrap_or_else(|| self.estimate_branch_partial(idx, f0, time))
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
        let affine = if !affine.is_finite() { 0.0 } else { affine };
        self.linearized_affine = affine;
        affine
    }

    /// Refresh the linearization (value and partials) at the given
    /// operating point for small-signal assembly. AC has no time axis;
    /// expressions see t = 0.
    pub(crate) fn linearize_at(&mut self, solution: &[Value]) {
        self.frequency = 0.0;
        let _ = self.linearize_expression(solution, 0.0);
    }

    pub(crate) fn linearize_at_frequency(&mut self, solution: &[Value], frequency: Value) {
        if !self.frequency_dependent {
            return;
        }
        self.frequency = frequency;
        let _ = self.linearize_expression(solution, 0.0);
    }

    /// Linearize at an arbitrary state and frequency. Unlike
    /// [`Self::linearize_at_frequency`], this always refreshes the Jacobian
    /// because the supplied state may have changed.
    pub(crate) fn linearize_at_state_and_frequency(
        &mut self,
        solution: &[Value],
        frequency: Value,
    ) {
        self.frequency = frequency;
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
    ) -> bool {
        let actual = self.evaluate(solution, time);
        let linearized = self.linearized_expression_value(solution);
        linearization_values_converged(actual, linearized, reltol, abstol)
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
        Expr::LookupTable(table) => table.transient_breakpoints,
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

fn expression_transient_breakpoints(expr: &Expr, tstop: Value) -> Vec<Value> {
    let mut breakpoints = Vec::new();
    collect_expression_transient_breakpoints(expr, tstop, &mut breakpoints);
    breakpoints.retain(|time| time.is_finite() && *time >= 0.0 && *time <= tstop);
    breakpoints.sort_by(Value::total_cmp);
    breakpoints.dedup_by(|a, b| {
        let scale = a.abs().max(b.abs()).max(1.0);
        (*a - *b).abs() <= 64.0 * Value::EPSILON * scale
    });
    breakpoints
}

fn collect_expression_transient_breakpoints(
    expr: &Expr,
    tstop: Value,
    breakpoints: &mut Vec<Value>,
) {
    match expr {
        Expr::Function { func, args } => {
            match func {
                Function::Table | Function::Pwl => {
                    collect_time_table_breakpoints(args, tstop, breakpoints);
                }
                Function::SpicePulse => {
                    collect_spice_pulse_breakpoints(args, tstop, breakpoints);
                }
                Function::SpiceSin => {
                    collect_spice_delay_breakpoint(args, 3, tstop, breakpoints);
                }
                Function::SpiceExp => {
                    collect_spice_delay_breakpoint(args, 2, tstop, breakpoints);
                    collect_spice_delay_breakpoint(args, 4, tstop, breakpoints);
                }
                _ => {}
            }
            for arg in args {
                collect_expression_transient_breakpoints(arg, tstop, breakpoints);
            }
        }
        Expr::Unary { operand, .. } => {
            collect_expression_transient_breakpoints(operand, tstop, breakpoints);
        }
        Expr::Binary { left, right, .. } => {
            collect_expression_transient_breakpoints(left, tstop, breakpoints);
            collect_expression_transient_breakpoints(right, tstop, breakpoints);
        }
        Expr::Const(_)
        | Expr::NodeVoltage(_)
        | Expr::BranchCurrent(_)
        | Expr::StringLiteral(_)
        | Expr::Time
        | Expr::Frequency
        | Expr::Temperature
        | Expr::ThermalVoltage
        | Expr::Gmin => {}
        Expr::LookupTable(table) => {
            if table.transient_breakpoints {
                breakpoints.extend(table.points.iter().map(|(time, _)| *time));
            }
        }
    }
}

fn collect_time_table_breakpoints(args: &[Expr], tstop: Value, breakpoints: &mut Vec<Value>) {
    let Some(input) = args.first() else {
        return;
    };

    let mut knots = Vec::new();
    for pair in args[1..].chunks(2) {
        let Some(x_expr) = pair.first() else {
            continue;
        };
        if let Some(time) = constant_expression_value(x_expr) {
            knots.push(time);
        }
    }

    match input {
        Expr::Time => breakpoints.extend(knots),
        Expr::Binary {
            op: BinaryOp::Mod,
            left,
            right,
        } if matches!(left.as_ref(), Expr::Time) => {
            let Some(period) = constant_expression_value(right) else {
                return;
            };
            if !period.is_finite() || period <= 0.0 {
                return;
            }

            let cycle_count = ((tstop.max(0.0) / period).ceil() as usize)
                .saturating_add(1)
                .min(1_000_000);
            for cycle in 0..cycle_count {
                let cycle_start = period * cycle as Value;
                if cycle_start > tstop {
                    break;
                }
                breakpoints.push(cycle_start);
                breakpoints.push(cycle_start + period);
                for knot in knots.iter().copied() {
                    if knot.is_finite() && knot >= 0.0 && knot <= period {
                        breakpoints.push(cycle_start + knot);
                    }
                }
            }
        }
        _ => {}
    }
}

fn collect_spice_pulse_breakpoints(args: &[Expr], tstop: Value, breakpoints: &mut Vec<Value>) {
    if args.len() < 6 {
        return;
    }

    let Some(delay) = constant_expression_value(&args[2]) else {
        return;
    };
    let Some(rise) = constant_expression_value(&args[3]) else {
        return;
    };
    let Some(fall) = constant_expression_value(&args[4]) else {
        return;
    };
    let Some(width) = constant_expression_value(&args[5]) else {
        return;
    };

    let delay = finite_nonnegative(delay, 0.0);
    let rise = finite_nonnegative(rise, 0.0);
    let fall = finite_nonnegative(fall, 0.0);
    let width = finite_nonnegative(width, 0.0);
    let period = args
        .get(6)
        .and_then(constant_expression_value)
        .filter(|period| period.is_finite() && *period > 0.0);

    let cycle_count = period
        .map(|period| {
            (((tstop - delay).max(0.0) / period).ceil() as usize)
                .saturating_add(1)
                .min(1_000_000)
        })
        .unwrap_or(1);

    for cycle in 0..cycle_count {
        let cycle_start = delay + period.unwrap_or(0.0) * cycle as Value;
        if cycle_start > tstop {
            break;
        }
        breakpoints.push(cycle_start);
        breakpoints.push(cycle_start + rise);
        breakpoints.push(cycle_start + rise + width);
        breakpoints.push(cycle_start + rise + width + fall);
    }
}

fn collect_spice_delay_breakpoint(
    args: &[Expr],
    delay_arg_index: usize,
    tstop: Value,
    breakpoints: &mut Vec<Value>,
) {
    let Some(delay_expr) = args.get(delay_arg_index) else {
        return;
    };
    let Some(delay) = constant_expression_value(delay_expr) else {
        return;
    };
    let delay = finite_nonnegative(delay, 0.0);
    if delay <= tstop {
        breakpoints.push(delay);
    }
}

fn constant_expression_value(expr: &Expr) -> Option<Value> {
    if expression_depends_on_runtime_quantity(expr) {
        return None;
    }
    let program = compile(expr);
    let mut vm = Vm::new();
    let value = vm.execute(&program, &Context::dc(&[], &[]));
    value.is_finite().then_some(value)
}

fn expression_depends_on_runtime_quantity(expr: &Expr) -> bool {
    match expr {
        Expr::Const(_) => false,
        Expr::NodeVoltage(_)
        | Expr::BranchCurrent(_)
        | Expr::StringLiteral(_)
        | Expr::Time
        | Expr::Frequency
        | Expr::Temperature
        | Expr::ThermalVoltage
        | Expr::Gmin => true,
        Expr::LookupTable(_) => true,
        Expr::Unary { operand, .. } => expression_depends_on_runtime_quantity(operand),
        Expr::Binary { left, right, .. } => {
            expression_depends_on_runtime_quantity(left)
                || expression_depends_on_runtime_quantity(right)
        }
        Expr::Function { args, .. } => args.iter().any(expression_depends_on_runtime_quantity),
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
        | Expr::LookupTable(_)
        | Expr::Time
        | Expr::Temperature
        | Expr::ThermalVoltage
        | Expr::Gmin => false,
    }
}

#[inline]
fn finite_nonnegative(value: Value, default: Value) -> Value {
    if value.is_finite() && value >= 0.0 {
        value
    } else {
        default
    }
}

fn analytic_expression_partial(
    expr: &Expr,
    program: &CompiledExpr,
    node_values: &[Value],
    branch_values: &[Value],
    time: Value,
    frequency: Value,
    temperature: Value,
    gmin: Value,
    expression_dialect: ExpressionDialect,
    target: DerivativeTarget,
) -> Option<Value> {
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

fn eval_behavioral_expr_with_derivative_at_boundary(
    expr: &Expr,
    context: &BehavioralDerivativeContext<'_>,
) -> Option<(Value, Value)> {
    let (value, derivative) = eval_behavioral_expr_with_derivative(expr, context)?;
    Some((
        normalize_expression_boundary(value, context.expression_dialect),
        normalize_expression_boundary(derivative, context.expression_dialect),
    ))
}

fn eval_behavioral_expr_with_derivative(
    expr: &Expr,
    context: &BehavioralDerivativeContext<'_>,
) -> Option<(Value, Value)> {
    match expr {
        Expr::Const(value) => Some((*value, 0.0)),
        Expr::Time => Some((context.time, 0.0)),
        Expr::Frequency => Some((context.frequency, 0.0)),
        Expr::Temperature => Some((context.temperature, 0.0)),
        Expr::ThermalVoltage => Some((
            crate::constants::thermal_voltage(crate::analysis::temperature::celsius_to_kelvin(
                context.temperature,
            )),
            0.0,
        )),
        Expr::Gmin => Some((context.gmin, 0.0)),
        Expr::StringLiteral(_) => Some((0.0, 0.0)),
        Expr::LookupTable(table) => {
            let value = eval_lookup_table(table.points.as_ref(), context.time)?;
            Some((value, 0.0))
        }
        Expr::NodeVoltage(name) => {
            let idx = *context.program.node_map.get(name)?;
            let value = *context.node_values.get(idx)?;
            let derivative = match context.target {
                DerivativeTarget::Node(target_idx) if target_idx == idx => 1.0,
                _ => 0.0,
            };
            Some((value, derivative))
        }
        Expr::BranchCurrent(name) => {
            let idx = *context.program.branch_map.get(name)?;
            let value = *context.branch_values.get(idx)?;
            let derivative = match context.target {
                DerivativeTarget::Branch(target_idx) if target_idx == idx => 1.0,
                _ => 0.0,
            };
            Some((value, derivative))
        }
        Expr::Unary { op, operand } => {
            let (value, derivative) = eval_behavioral_expr_with_derivative(operand, context)?;
            match op {
                UnaryOp::Neg => Some((-value, -derivative)),
                UnaryOp::Not => Some((if value == 0.0 { 1.0 } else { 0.0 }, 0.0)),
            }
        }
        Expr::Binary { op, left, right } => {
            let (left_value, left_derivative) =
                eval_behavioral_expr_with_derivative(left, context)?;
            let (right_value, right_derivative) =
                eval_behavioral_expr_with_derivative(right, context)?;
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

fn eval_binary_with_derivative(
    op: BinaryOp,
    left: Value,
    d_left: Value,
    right: Value,
    d_right: Value,
    expression_dialect: ExpressionDialect,
) -> Option<(Value, Value)> {
    match op {
        BinaryOp::Add => Some((left + right, d_left + d_right)),
        BinaryOp::Sub => Some((left - right, d_left - d_right)),
        BinaryOp::Mul => Some((left * right, d_left * right + left * d_right)),
        BinaryOp::Div => {
            if right == 0.0 {
                None
            } else {
                Some((
                    left / right,
                    (d_left * right - left * d_right) / (right * right),
                ))
            }
        }
        BinaryOp::Mod => {
            if right == 0.0 {
                None
            } else {
                let quotient = (left / right).trunc();
                Some((left % right, d_left - quotient * d_right))
            }
        }
        BinaryOp::Pow => real_pow_with_derivative(left, d_left, right, d_right, expression_dialect),
        BinaryOp::Lt => Some((bool_value(left < right), 0.0)),
        BinaryOp::Le => Some((bool_value(left <= right), 0.0)),
        BinaryOp::Gt => Some((bool_value(left > right), 0.0)),
        BinaryOp::Ge => Some((bool_value(left >= right), 0.0)),
        BinaryOp::Eq => Some((bool_value((left - right).abs() < EXPR_ZERO_TOLERANCE), 0.0)),
        BinaryOp::Ne => Some((bool_value((left - right).abs() >= EXPR_ZERO_TOLERANCE), 0.0)),
        BinaryOp::And => Some((bool_value(left != 0.0 && right != 0.0), 0.0)),
        BinaryOp::Or => Some((bool_value(left != 0.0 || right != 0.0), 0.0)),
    }
}

fn eval_function_with_derivative(
    func: Function,
    args: &[Expr],
    context: &BehavioralDerivativeContext<'_>,
) -> Option<(Value, Value)> {
    let eval_arg = |index: usize| {
        args.get(index)
            .and_then(|arg| eval_behavioral_expr_with_derivative(arg, context))
    };

    match func {
        Function::Abs => {
            let (x, dx) = eval_arg(0)?;
            Some((x.abs(), x.signum() * dx))
        }
        Function::Sqrt => {
            let (x, dx) = eval_arg(0)?;
            let value = x.max(0.0).sqrt();
            if value == 0.0 {
                Some((value, 0.0))
            } else {
                Some((value, 0.5 * dx / value))
            }
        }
        Function::Exp => unary_derivative(eval_arg(0)?, |x| x.exp(), |x| x.exp()),
        Function::Log => {
            let (x, dx) = eval_arg(0)?;
            let clamped = x.max(1.0e-38);
            if context.expression_dialect == ExpressionDialect::Xyce {
                Some((clamped.log10(), dx / (std::f64::consts::LN_10 * clamped)))
            } else {
                Some((clamped.ln(), dx / clamped))
            }
        }
        Function::Ln => {
            let (x, dx) = eval_arg(0)?;
            let clamped = x.max(1.0e-38);
            Some((clamped.ln(), dx / clamped))
        }
        Function::Log10 => {
            let (x, dx) = eval_arg(0)?;
            let clamped = x.max(1.0e-38);
            Some((clamped.log10(), dx / (std::f64::consts::LN_10 * clamped)))
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
        Function::Atan => unary_derivative(eval_arg(0)?, |x| x.atan(), |x| 1.0 / (1.0 + x * x)),
        Function::Atan2 => {
            let (y, dy) = eval_arg(0)?;
            let (x, dx) = eval_arg(1)?;
            let denom = x * x + y * y;
            if denom == 0.0 {
                None
            } else {
                Some((y.atan2(x), (x * dy - y * dx) / denom))
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
                    0.0
                };
                Some((value, derivative))
            } else {
                let value = x.tanh();
                Some((value, dx * (1.0 - value * value)))
            }
        }
        Function::Asinh => {
            unary_derivative(eval_arg(0)?, |x| x.asinh(), |x| 1.0 / (x * x + 1.0).sqrt())
        }
        Function::Acosh => unary_derivative(
            eval_arg(0)?,
            |x| x.acosh(),
            |x| 1.0 / ((x - 1.0).sqrt() * (x + 1.0).sqrt()),
        ),
        Function::Atanh => {
            let (x, dx) = eval_arg(0)?;
            if context.expression_dialect == ExpressionDialect::Xyce {
                let lower = XYCE_ATANH_EPSILON - 1.0;
                let upper = 1.0 - XYCE_ATANH_EPSILON;
                let clamped = x.clamp(lower, upper);
                let derivative = if x >= lower && x <= upper {
                    dx / (1.0 - x * x)
                } else {
                    0.0
                };
                Some((clamped.atanh(), derivative))
            } else {
                Some((x.atanh(), dx / (1.0 - x * x)))
            }
        }
        Function::Trunc => {
            let (x, _) = eval_arg(0)?;
            Some((x.trunc(), 0.0))
        }
        Function::Floor => {
            let (x, _) = eval_arg(0)?;
            Some((x.floor(), 0.0))
        }
        Function::Ceil => {
            let (x, _) = eval_arg(0)?;
            Some((x.ceil(), 0.0))
        }
        Function::Round => {
            let (x, _) = eval_arg(0)?;
            Some((x.round_ties_even(), 0.0))
        }
        Function::Sqr => {
            let (x, dx) = eval_arg(0)?;
            Some((x * x, 2.0 * x * dx))
        }
        Function::Pwr => {
            let (base, d_base) = eval_arg(0)?;
            let (exponent, d_exponent) = eval_arg(1)?;
            let abs_base = base.abs();
            if d_exponent == 0.0 {
                let value = abs_base.powf(exponent);
                Some((
                    value,
                    exponent * abs_base.powf(exponent - 1.0) * base.signum() * d_base,
                ))
            } else if abs_base > 0.0 {
                let value = abs_base.powf(exponent);
                Some((
                    value,
                    value
                        * (d_exponent * abs_base.ln()
                            + exponent * base.signum() * d_base / abs_base),
                ))
            } else {
                None
            }
        }
        Function::Pwrs => {
            let (base, d_base) = eval_arg(0)?;
            let (exponent, d_exponent) = eval_arg(1)?;
            let abs_base = base.abs();
            let sign = base.signum();
            if d_exponent == 0.0 {
                let value = sign * abs_base.powf(exponent);
                Some((value, exponent * abs_base.powf(exponent - 1.0) * d_base))
            } else if abs_base > 0.0 {
                let magnitude = abs_base.powf(exponent);
                Some((
                    sign * magnitude,
                    sign * magnitude
                        * (d_exponent * abs_base.ln()
                            + exponent * base.signum() * d_base / abs_base),
                ))
            } else {
                None
            }
        }
        Function::Limit => match args.len() {
            2 => {
                let (nom, d_nom) = eval_arg(0)?;
                Some((nom, d_nom))
            }
            3 => {
                let (x, dx) = eval_arg(0)?;
                let (min, _) = eval_arg(1)?;
                let (max, _) = eval_arg(2)?;
                Some((
                    x.clamp(min, max),
                    if x >= min && x <= max { dx } else { 0.0 },
                ))
            }
            _ => None,
        },
        Function::Min => {
            let (left, d_left) = eval_arg(0)?;
            let (right, d_right) = eval_arg(1)?;
            Some(if left <= right {
                (left, d_left)
            } else {
                (right, d_right)
            })
        }
        Function::Max => {
            let (left, d_left) = eval_arg(0)?;
            let (right, d_right) = eval_arg(1)?;
            Some(if left >= right {
                (left, d_left)
            } else {
                (right, d_right)
            })
        }
        Function::Sign => {
            let (x, _) = eval_arg(0)?;
            Some((x.signum(), 0.0))
        }
        Function::Uramp => {
            let (x, dx) = eval_arg(0)?;
            Some((x.max(0.0), if x > 0.0 { dx } else { 0.0 }))
        }
        Function::Stp => {
            let (x, _) = eval_arg(0)?;
            Some((bool_value(x > EXPR_ZERO_TOLERANCE), 0.0))
        }
        Function::Ustep => {
            let (x, _) = eval_arg(0)?;
            Some((
                if x > 0.0 {
                    1.0
                } else if x < 0.0 {
                    0.0
                } else {
                    0.5
                },
                0.0,
            ))
        }
        Function::U2 => {
            let (x, dx) = eval_arg(0)?;
            Some((x.clamp(0.0, 1.0), if x > 0.0 && x < 1.0 { dx } else { 0.0 }))
        }
        Function::Eq0 => {
            let (x, _) = eval_arg(0)?;
            Some((bool_value(x.abs() < EXPR_ZERO_TOLERANCE), 0.0))
        }
        Function::Ne0 => {
            let (x, _) = eval_arg(0)?;
            Some((bool_value(x.abs() >= EXPR_ZERO_TOLERANCE), 0.0))
        }
        Function::Gt0 => {
            let (x, _) = eval_arg(0)?;
            Some((bool_value(x > 0.0), 0.0))
        }
        Function::Lt0 => {
            let (x, _) = eval_arg(0)?;
            Some((bool_value(x < 0.0), 0.0))
        }
        Function::Ge0 => {
            let (x, _) = eval_arg(0)?;
            Some((bool_value(x >= 0.0), 0.0))
        }
        Function::Le0 => {
            let (x, _) = eval_arg(0)?;
            Some((bool_value(x <= 0.0), 0.0))
        }
        Function::Pow => {
            let (left, d_left) = eval_arg(0)?;
            let (right, d_right) = eval_arg(1)?;
            real_pow_with_derivative(left, d_left, right, d_right, context.expression_dialect)
        }
        Function::Mod => {
            let (left, d_left) = eval_arg(0)?;
            let (right, d_right) = eval_arg(1)?;
            if right == 0.0 {
                None
            } else {
                let quotient = (left / right).trunc();
                Some((left % right, d_left - quotient * d_right))
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
        | Function::BarycentricFile => Some((0.0, 0.0)),
        Function::Sdt => None,
        Function::SpicePulse | Function::SpiceSin | Function::SpiceExp | Function::SpiceSffm => {
            None
        }
        Function::If => {
            let (condition, _) = eval_arg(0)?;
            if condition != 0.0 {
                eval_arg(1)
            } else {
                eval_arg(2)
            }
        }
    }
}

fn unary_derivative(
    input: (Value, Value),
    value_fn: impl FnOnce(Value) -> Value,
    derivative_fn: impl FnOnce(Value) -> Value,
) -> Option<(Value, Value)> {
    let (x, dx) = input;
    Some((value_fn(x), derivative_fn(x) * dx))
}

fn eval_table_function_with_derivative(
    args: &[Expr],
    context: &BehavioralDerivativeContext<'_>,
) -> Option<(Value, Value)> {
    eval_piecewise_function_with_derivative(args, context, eval_table_points_with_derivative)
}

fn eval_pwl_function_with_derivative(
    args: &[Expr],
    context: &BehavioralDerivativeContext<'_>,
) -> Option<(Value, Value)> {
    eval_piecewise_function_with_derivative(args, context, eval_pwl_points_with_derivative)
}

fn eval_piecewise_function_with_derivative(
    args: &[Expr],
    context: &BehavioralDerivativeContext<'_>,
    evaluator: fn(Value, Value, &[(Value, Value)]) -> Option<(Value, Value)>,
) -> Option<(Value, Value)> {
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

fn eval_lookup_table(points: &[(Value, Value)], x: Value) -> Option<Value> {
    eval_table_points_with_derivative(x, 0.0, points).map(|(value, _)| value)
}

fn eval_table_points_with_derivative(
    x: Value,
    dx: Value,
    points: &[(Value, Value)],
) -> Option<(Value, Value)> {
    if points.is_empty() {
        return Some((0.0, 0.0));
    }
    if points.len() == 1 {
        return Some((points[0].1, 0.0));
    }
    if x <= points[0].0 {
        return Some((points[0].1, 0.0));
    }
    let last = points.len() - 1;
    if x >= points[last].0 {
        return Some((points[last].1, 0.0));
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
    dx: Value,
    points: &[(Value, Value)],
) -> Option<(Value, Value)> {
    if points.is_empty() {
        return Some((0.0, 0.0));
    }
    if points.len() == 1 {
        return Some((points[0].1, 0.0));
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
    dx: Value,
    ((x0, y0), (x1, y1)): ((Value, Value), (Value, Value)),
) -> Option<(Value, Value)> {
    let span = x1 - x0;
    if !span.is_finite() || span == 0.0 {
        return Some((y0, 0.0));
    }
    let slope = (y1 - y0) / span;
    Some((y0 + (x - x0) * slope, slope * dx))
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
    /// Linearization partials d(expr)/d(node_values[idx])
    node_partials: Vec<Value>,
    /// Linearization partials d(expr)/d(branch_values[idx])
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
            temperature: crate::analysis::temperature::kelvin_to_celsius(
                crate::constants::TEMP_REFERENCE,
            ),
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

    /// Evaluate and commit stateful expression operators at an accepted point.
    pub(crate) fn accept_transient_step(&mut self, solution: &[Value], time: Value) {
        let _ = self.evaluate(solution, time);
        self.vm.accept_transient_step(time);
    }

    #[inline]
    pub(crate) fn bound_solution_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.node_bindings
            .iter()
            .chain(self.branch_bindings.iter())
            .filter_map(|binding| *binding)
    }

    pub(crate) fn transient_breakpoints(&self, tstop: Value, _tstep_hint: Value) -> Vec<Value> {
        expression_transient_breakpoints(&self.ast, tstop)
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
            .with_expression_dialect(self.expression_dialect);
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
            self.linearized_affine = 0.0;
            return 0.0;
        }

        for idx in 0..self.node_bindings.len() {
            self.node_partials[idx] = if self.node_bindings[idx].is_some() {
                analytic_expression_partial(
                    &self.ast,
                    &self.program,
                    &self.node_values,
                    &self.branch_values,
                    time,
                    self.frequency,
                    self.temperature,
                    self.gmin,
                    self.expression_dialect,
                    DerivativeTarget::Node(idx),
                )
                .unwrap_or_else(|| self.estimate_node_partial(idx, f0, time))
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
                    time,
                    self.frequency,
                    self.temperature,
                    self.gmin,
                    self.expression_dialect,
                    DerivativeTarget::Branch(idx),
                )
                .unwrap_or_else(|| self.estimate_branch_partial(idx, f0, time))
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
        let affine = if !affine.is_finite() { 0.0 } else { affine };
        self.linearized_affine = affine;
        affine
    }

    /// Refresh the linearization (value and partials) at the given
    /// operating point for small-signal assembly. AC has no time axis;
    /// expressions see t = 0.
    pub(crate) fn linearize_at(&mut self, solution: &[Value]) {
        self.frequency = 0.0;
        let _ = self.linearize_expression(solution, 0.0);
    }

    pub(crate) fn linearize_at_frequency(&mut self, solution: &[Value], frequency: Value) {
        if !self.frequency_dependent {
            return;
        }
        self.frequency = frequency;
        let _ = self.linearize_expression(solution, 0.0);
    }

    /// Linearize at an arbitrary state and frequency. Unlike
    /// [`Self::linearize_at_frequency`], this always refreshes the Jacobian
    /// because the supplied state may have changed.
    pub(crate) fn linearize_at_state_and_frequency(
        &mut self,
        solution: &[Value],
        frequency: Value,
    ) {
        self.frequency = frequency;
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
    ) -> bool {
        let actual = self.evaluate(solution, time);
        let linearized = self.linearized_expression_value(solution);
        linearization_values_converged(actual, linearized, reltol, abstol)
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
    ) -> bool {
        self.voltage_sources
            .iter_mut()
            .all(|source| source.linearization_converged(solution, time, reltol, voltage_abstol))
            && self.current_sources.iter_mut().all(|source| {
                source.linearization_converged(solution, time, reltol, current_abstol)
            })
    }

    pub(crate) fn transient_breakpoints(&self, tstop: Value, tstep_hint: Value) -> Vec<Value> {
        let mut breakpoints = Vec::new();
        for source in &self.voltage_sources {
            breakpoints.extend(source.transient_breakpoints(tstop, tstep_hint));
        }
        for source in &self.current_sources {
            breakpoints.extend(source.transient_breakpoints(tstop, tstep_hint));
        }
        breakpoints.sort_by(Value::total_cmp);
        breakpoints.dedup_by(|a, b| {
            let scale = a.abs().max(b.abs()).max(1.0);
            (*a - *b).abs() <= 64.0 * Value::EPSILON * scale
        });
        breakpoints
    }

    /// Commit stateful expression operators once at a successful timestep.
    pub(crate) fn accept_transient_step(&mut self, solution: &[Value], time: Value) {
        for source in &mut self.voltage_sources {
            source.accept_transient_step(solution, time);
        }
        for source in &mut self.current_sources {
            source.accept_transient_step(solution, time);
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
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

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
            .bind_references(|name| (name == "n").then_some(1), |_| None)
            .expect("invariant source binds");
        invariant.linearize_at(&[3.0]);
        let initial_partials = invariant.linearized_partials().collect::<Vec<_>>();
        invariant.linearize_at_frequency(&[9.0], 100.0);
        assert_eq!(invariant.frequency, 0.0);
        assert_eq!(
            invariant.linearized_partials().collect::<Vec<_>>(),
            initial_partials
        );

        let mut dependent =
            BehavioralCurrentSource::new("Bdependent".to_string(), 1, 0, "freq*v(n)")
                .expect("frequency-dependent source parses");
        dependent
            .bind_references(|name| (name == "n").then_some(1), |_| None)
            .expect("frequency-dependent source binds");
        dependent.linearize_at(&[3.0]);
        dependent.linearize_at_frequency(&[3.0], 100.0);
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
            .unwrap_or_else(|| panic!("analytic derivative for `{expression}` failed"))
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
}
