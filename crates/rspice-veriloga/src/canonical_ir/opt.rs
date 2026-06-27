use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use super::{
    BranchId, BranchUnknownId, CanonicalValueType, CompilerPhase, EquationId, ExprId,
    HirAnalogOperator, HirAssignment, HirExprKind, HirLoop, HirModel, HirStatement, IrDiagnostic,
    IrValidationResult, MirEquation, MirEquationKind, MirModel, NodeId, ParamId, ScheduleId,
    ValueId, VariableId,
};

const MAX_SCALAR_LOOP_UNROLL_ITERATIONS: usize = 1024;
pub(crate) const LIMEXP_MAX: f64 = 5.54062238439351e34;
pub(crate) const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvalidationClass {
    InstanceStatic,
    TemperatureStatic,
    TimestepStatic,
    OperatingPointStatic,
    NewtonIteration,
    AcFrequency,
    NoiseFrequency,
    OperatingPointReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptValueType {
    Real,
    Boolean,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DerivativeLaneKind {
    Node,
    BranchUnknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DerivativeLane {
    pub kind: DerivativeLaneKind,
    pub index: u32,
}

impl DerivativeLane {
    pub const fn node(node: NodeId) -> Self {
        Self {
            kind: DerivativeLaneKind::Node,
            index: node.index(),
        }
    }

    pub const fn branch_unknown(branch_unknown: BranchUnknownId) -> Self {
        Self {
            kind: DerivativeLaneKind::BranchUnknown,
            index: branch_unknown.index(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptUnaryOp {
    Pos,
    Neg,
    Not,
    Exp,
    LimExp,
    LimExpDerivative,
    Ln,
    Sqrt,
    Abs,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Atan,
    Asinh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OptValueKind {
    RealConstant(f64),
    BooleanConstant(bool),
    Parameter {
        parameter: ParamId,
    },
    ParamGiven {
        parameter: ParamId,
    },
    Temperature,
    ThermalVoltage,
    Multiplicity,
    Time,
    NodePotential {
        node: NodeId,
    },
    BranchFlow {
        branch: BranchId,
    },
    Unary {
        op: OptUnaryOp,
        input: ValueId,
    },
    Binary {
        op: OptBinaryOp,
        left: ValueId,
        right: ValueId,
    },
    Select {
        condition: ValueId,
        then_value: ValueId,
        else_value: ValueId,
    },
    EquationValue {
        equation: EquationId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptDerivative {
    pub lane: DerivativeLane,
    pub value: ValueId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptValue {
    pub id: ValueId,
    pub value_type: OptValueType,
    pub kind: OptValueKind,
    pub derivatives: Vec<OptDerivative>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptOp {
    ComputeValue { value: ValueId },
    EvaluateEquation { equation: EquationId },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptSchedule {
    pub id: ScheduleId,
    pub invalidation: InvalidationClass,
    pub ops: Vec<OptOp>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptModel {
    pub module_name: SmolStr,
    pub node_count: u32,
    pub parameter_count: u32,
    pub branch_count: u32,
    pub branch_unknown_count: u32,
    pub equation_count: u32,
    pub values: Vec<OptValue>,
    pub schedules: Vec<OptSchedule>,
}

impl OptModel {
    pub fn from_mir(mir: &MirModel) -> Result<Self, Vec<IrDiagnostic>> {
        mir.validate()?;

        Self::from_validated_inputs(None, mir)
    }

    pub fn from_hir_and_mir(hir: &HirModel, mir: &MirModel) -> Result<Self, Vec<IrDiagnostic>> {
        hir.validate()?;
        mir.validate()?;

        Self::from_validated_inputs(Some(hir), mir)
    }

    fn from_validated_inputs(
        hir: Option<&HirModel>,
        mir: &MirModel,
    ) -> Result<Self, Vec<IrDiagnostic>> {
        let mut builder = ScalarGraphBuilder::new(hir, mir);
        if let Some(hir) = hir {
            builder.lower_statements(&hir.statements);
        }
        let mut equation_values = Vec::with_capacity(mir.equations.len());
        for equation in &mir.equations {
            let value = builder.lower_equation_expression(equation.expression.id);
            builder.cache_declared_branch_current(equation, value);
            equation_values.push(value);
        }
        builder.add_sparse_derivatives();
        let values = builder.finish(&mut equation_values);

        let instance_static_ops = collect_instance_static_ops(&values);
        let mut schedules = Vec::new();
        if !instance_static_ops.is_empty() {
            schedules.push(OptSchedule {
                id: ScheduleId::from(schedules.len()),
                invalidation: InvalidationClass::InstanceStatic,
                ops: instance_static_ops,
            });
        }

        let mut newton_ops = Vec::new();
        for (equation, value) in mir.equations.iter().zip(equation_values) {
            if let Some(value) = value {
                newton_ops.push(OptOp::ComputeValue { value });
            }
            newton_ops.push(OptOp::EvaluateEquation {
                equation: equation.id,
            });
        }
        schedules.push(OptSchedule {
            id: ScheduleId::from(schedules.len()),
            invalidation: InvalidationClass::NewtonIteration,
            ops: newton_ops,
        });

        let opt = Self {
            module_name: mir.module_name.clone(),
            node_count: u32::try_from(mir.nodes.len()).expect("MIR node count exceeds u32::MAX"),
            parameter_count: u32::try_from(mir.parameters.len())
                .expect("MIR parameter count exceeds u32::MAX"),
            branch_count: u32::try_from(mir.branches.len())
                .expect("MIR branch count exceeds u32::MAX"),
            branch_unknown_count: u32::try_from(mir.branch_unknowns.len())
                .expect("MIR branch unknown count exceeds u32::MAX"),
            equation_count: u32::try_from(mir.equations.len())
                .expect("MIR equation count exceeds u32::MAX"),
            values,
            schedules,
        };

        opt.validate().map(|()| opt)
    }

    pub fn validate(&self) -> IrValidationResult {
        let mut diagnostics = Vec::new();

        if self.module_name.is_empty() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                "OptIR module name must not be empty",
            ));
        }

        validate_dense_value_ids(&mut diagnostics, &self.values);
        validate_values(&mut diagnostics, self);
        validate_dense_schedule_ids(&mut diagnostics, &self.schedules);
        validate_schedules(
            &mut diagnostics,
            &self.schedules,
            self.values.len(),
            self.equation_count,
        );

        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }
}

struct ScalarGraphBuilder<'a> {
    hir: Option<&'a HirModel>,
    mir: &'a MirModel,
    values: Vec<OptValue>,
    value_keys: HashMap<OptValueKey, ValueId>,
    expression_values: HashMap<ExprId, Option<ValueId>>,
    variable_values: HashMap<VariableId, Option<ValueId>>,
    branch_current_values: HashMap<String, ValueId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum OptValueKey {
    RealConstant(u64),
    BooleanConstant(bool),
    Parameter(ParamId),
    ParamGiven(ParamId),
    Temperature,
    ThermalVoltage,
    Multiplicity,
    Time,
    NodePotential(NodeId),
    BranchFlow(BranchId),
    Unary {
        op: OptUnaryOp,
        input: ValueId,
    },
    Binary {
        op: OptBinaryOp,
        left: ValueId,
        right: ValueId,
    },
    Select {
        condition: ValueId,
        then_value: ValueId,
        else_value: ValueId,
    },
    EquationValue(EquationId),
}

impl OptValueKey {
    fn from_kind(kind: &OptValueKind) -> Self {
        match kind {
            OptValueKind::RealConstant(value) => Self::RealConstant(value.to_bits()),
            OptValueKind::BooleanConstant(value) => Self::BooleanConstant(*value),
            OptValueKind::Parameter { parameter } => Self::Parameter(*parameter),
            OptValueKind::ParamGiven { parameter } => Self::ParamGiven(*parameter),
            OptValueKind::Temperature => Self::Temperature,
            OptValueKind::ThermalVoltage => Self::ThermalVoltage,
            OptValueKind::Multiplicity => Self::Multiplicity,
            OptValueKind::Time => Self::Time,
            OptValueKind::NodePotential { node } => Self::NodePotential(*node),
            OptValueKind::BranchFlow { branch } => Self::BranchFlow(*branch),
            OptValueKind::Unary { op, input } => Self::Unary {
                op: *op,
                input: *input,
            },
            OptValueKind::Binary { op, left, right } => Self::Binary {
                op: *op,
                left: *left,
                right: *right,
            },
            OptValueKind::Select {
                condition,
                then_value,
                else_value,
            } => Self::Select {
                condition: *condition,
                then_value: *then_value,
                else_value: *else_value,
            },
            OptValueKind::EquationValue { equation } => Self::EquationValue(*equation),
        }
    }
}

impl<'a> ScalarGraphBuilder<'a> {
    fn new(hir: Option<&'a HirModel>, mir: &'a MirModel) -> Self {
        Self {
            hir,
            mir,
            values: Vec::new(),
            value_keys: HashMap::new(),
            expression_values: HashMap::new(),
            variable_values: HashMap::new(),
            branch_current_values: HashMap::new(),
        }
    }

    fn finish(mut self, equation_values: &mut [Option<ValueId>]) -> Vec<OptValue> {
        self.eliminate_dead_values(equation_values);
        self.values
    }

    fn eliminate_dead_values(&mut self, equation_values: &mut [Option<ValueId>]) {
        if self.values.is_empty() {
            return;
        }

        let mut observable_roots = HashSet::new();
        let mut live = vec![false; self.values.len()];
        for root in equation_values.iter().flatten().copied() {
            observable_roots.insert(root);
            self.mark_live_value(root, &mut live);
            for derivative in self.values[usize::from(root)].derivatives.clone() {
                self.mark_live_value(derivative.value, &mut live);
            }
        }

        if live.iter().all(|is_live| *is_live) {
            return;
        }

        let mut remap = vec![None; self.values.len()];
        let mut compacted = Vec::with_capacity(live.iter().filter(|is_live| **is_live).count());
        for (old_index, value) in self.values.iter().enumerate() {
            if live[old_index] {
                let new_id = ValueId::from(compacted.len());
                remap[old_index] = Some(new_id);
                let mut value = value.clone();
                value.id = new_id;
                compacted.push(value);
            }
        }

        let observable_new_roots: HashSet<_> = observable_roots
            .iter()
            .map(|root| remap_value_id(*root, &remap))
            .collect();

        for value in &mut compacted {
            value.kind = remap_value_kind(&value.kind, &remap);
            if observable_new_roots.contains(&value.id) {
                value.derivatives = value
                    .derivatives
                    .iter()
                    .map(|derivative| OptDerivative {
                        lane: derivative.lane,
                        value: remap_value_id(derivative.value, &remap),
                    })
                    .collect();
            } else {
                value.derivatives.clear();
            }
        }

        for root in equation_values.iter_mut().flatten() {
            *root = remap_value_id(*root, &remap);
        }

        self.values = compacted;
    }

    fn mark_live_value(&self, value: ValueId, live: &mut [bool]) {
        let index = usize::from(value);
        if live.get(index).copied().unwrap_or(false) {
            return;
        }
        let Some(slot) = live.get_mut(index) else {
            return;
        };
        *slot = true;
        match self.values[index].kind {
            OptValueKind::RealConstant(_)
            | OptValueKind::BooleanConstant(_)
            | OptValueKind::Parameter { .. }
            | OptValueKind::ParamGiven { .. }
            | OptValueKind::Temperature
            | OptValueKind::ThermalVoltage
            | OptValueKind::Multiplicity
            | OptValueKind::Time
            | OptValueKind::NodePotential { .. }
            | OptValueKind::BranchFlow { .. }
            | OptValueKind::EquationValue { .. } => {}
            OptValueKind::Unary { input, .. } => self.mark_live_value(input, live),
            OptValueKind::Binary { left, right, .. } => {
                self.mark_live_value(left, live);
                self.mark_live_value(right, live);
            }
            OptValueKind::Select {
                condition,
                then_value,
                else_value,
            } => {
                self.mark_live_value(condition, live);
                self.mark_live_value(then_value, live);
                self.mark_live_value(else_value, live);
            }
        }
    }

    fn lower_statements(&mut self, statements: &[HirStatement]) {
        for statement in statements {
            self.lower_statement(statement);
        }
    }

    fn lower_statement(&mut self, statement: &HirStatement) {
        match statement {
            HirStatement::Assignment(assignment) => self.lower_assignment_statement(assignment),
            HirStatement::Loop(loop_statement) => self.lower_loop_statement(loop_statement),
        }
    }

    fn lower_assignment_statement(&mut self, assignment: &HirAssignment) {
        let value = if assignment.index.is_none()
            && supported_assignment_value_type(assignment.expr_type)
        {
            self.lower_expression(assignment.expr.id)
        } else {
            None
        };
        self.variable_values.insert(assignment.target, value);
        self.expression_values.clear();
    }

    fn lower_loop_statement(&mut self, loop_statement: &HirLoop) {
        if self.lower_counted_accumulator_loop(loop_statement) {
            return;
        }

        let mut iterations = 0;
        loop {
            self.expression_values.clear();
            let Some(condition) = self.lower_expression(loop_statement.condition.id) else {
                self.mark_loop_assignments_unknown(loop_statement);
                return;
            };
            match self.boolean_constant(condition) {
                Some(false) => {
                    self.expression_values.clear();
                    return;
                }
                Some(true) => {}
                None => {
                    self.mark_loop_assignments_unknown(loop_statement);
                    return;
                }
            }

            if iterations == MAX_SCALAR_LOOP_UNROLL_ITERATIONS {
                self.mark_loop_assignments_unknown(loop_statement);
                return;
            }
            iterations += 1;
            self.expression_values.clear();
            self.lower_statements(&loop_statement.body);
        }
    }

    fn lower_counted_accumulator_loop(&mut self, loop_statement: &HirLoop) -> bool {
        let original_values = self.values.clone();
        let original_value_keys = self.value_keys.clone();
        let original_expression_values = self.expression_values.clone();
        let original_variable_values = self.variable_values.clone();

        let matched = self.try_lower_counted_accumulator_loop(loop_statement);
        if matched {
            self.expression_values.clear();
            true
        } else {
            self.values = original_values;
            self.value_keys = original_value_keys;
            self.expression_values = original_expression_values;
            self.variable_values = original_variable_values;
            false
        }
    }

    fn try_lower_counted_accumulator_loop(&mut self, loop_statement: &HirLoop) -> bool {
        self.expression_values.clear();
        let Some((counter, bound)) = self.counted_loop_condition(loop_statement.condition.id)
        else {
            return false;
        };
        if self.variable_value_type(counter) != Some(CanonicalValueType::Integer) {
            return false;
        }
        let Some(counter_start) = self.current_variable_value(counter) else {
            return false;
        };
        if !self.is_real_constant(counter_start, 0.0) {
            return false;
        }

        let Some(assigned_variables) = self.loop_assignment_targets(&loop_statement.body) else {
            return false;
        };
        let mut saw_counter_increment = false;
        let mut saw_accumulator = false;

        for statement in &loop_statement.body {
            let HirStatement::Assignment(assignment) = statement else {
                return false;
            };
            if self.is_counter_increment_assignment(assignment, counter) {
                saw_counter_increment = true;
                continue;
            }

            let Some(term_expr) =
                self.accumulator_update_term(assignment, counter, &assigned_variables)
            else {
                return false;
            };
            if self.expr_references_any_variable(term_expr, &assigned_variables) {
                return false;
            }
            let Some(previous) = self.current_variable_value(assignment.target) else {
                return false;
            };
            self.expression_values.clear();
            let Some(term) = self.lower_expression(term_expr) else {
                return false;
            };
            let scaled_term = self.push_binary_value(OptBinaryOp::Mul, bound, term);
            let next = self.push_binary_value(OptBinaryOp::Add, previous, scaled_term);
            self.variable_values.insert(assignment.target, Some(next));
            saw_accumulator = true;
        }

        if !(saw_counter_increment && saw_accumulator) {
            return false;
        }

        self.variable_values.insert(counter, Some(bound));
        true
    }

    fn counted_loop_condition(&mut self, condition: ExprId) -> Option<(VariableId, ValueId)> {
        let expression = self.mir.expressions.get(usize::from(condition))?;
        let HirExprKind::Binary { op, left, right } = &expression.kind else {
            return None;
        };
        if op.as_str() != "Lt" {
            return None;
        }

        let counter = self.variable_identifier(*left)?;
        let bound = self.nonnegative_integer_loop_bound(*right)?;
        Some((counter, bound))
    }

    fn nonnegative_integer_loop_bound(&mut self, expr: ExprId) -> Option<ValueId> {
        if let Some(parameter) = self.parameter_identifier(expr) {
            let slot = self.mir.parameters.get(usize::from(parameter))?;
            if slot.value_type == CanonicalValueType::Integer
                && slot
                    .range
                    .as_ref()
                    .and_then(|range| range.min)
                    .is_some_and(|min| min >= 0.0)
            {
                return self.lower_expression(expr);
            }
            return None;
        }

        let value = self.number_constant_expr(expr)?;
        if value >= 0.0 && value.fract() == 0.0 {
            return self.lower_expression(expr);
        }
        None
    }

    fn loop_assignment_targets(&self, statements: &[HirStatement]) -> Option<HashSet<VariableId>> {
        let mut targets = HashSet::new();
        for statement in statements {
            let HirStatement::Assignment(assignment) = statement else {
                return None;
            };
            if assignment.index.is_some() {
                return None;
            }
            targets.insert(assignment.target);
        }
        Some(targets)
    }

    fn is_counter_increment_assignment(
        &self,
        assignment: &HirAssignment,
        counter: VariableId,
    ) -> bool {
        if assignment.target != counter || assignment.index.is_some() {
            return false;
        }
        let Some((left, right)) = self.add_operands(assignment.expr.id) else {
            return false;
        };
        (self.variable_identifier(left) == Some(counter)
            && self.number_constant_expr(right) == Some(1.0))
            || (self.variable_identifier(right) == Some(counter)
                && self.number_constant_expr(left) == Some(1.0))
    }

    fn accumulator_update_term(
        &self,
        assignment: &HirAssignment,
        counter: VariableId,
        assigned_variables: &HashSet<VariableId>,
    ) -> Option<ExprId> {
        if assignment.target == counter
            || assignment.index.is_some()
            || !supported_assignment_value_type(assignment.expr_type)
        {
            return None;
        }
        if !assigned_variables.contains(&assignment.target) {
            return None;
        }
        let (left, right) = self.add_operands(assignment.expr.id)?;
        if self.variable_identifier(left) == Some(assignment.target) {
            return Some(right);
        }
        if self.variable_identifier(right) == Some(assignment.target) {
            return Some(left);
        }
        None
    }

    fn add_operands(&self, expr: ExprId) -> Option<(ExprId, ExprId)> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::Binary { op, left, right } = &expression.kind else {
            return None;
        };
        (op.as_str() == "Add").then_some((*left, *right))
    }

    fn current_variable_value(&mut self, variable: VariableId) -> Option<ValueId> {
        match self.variable_values.get(&variable).copied() {
            Some(value) => value,
            None => self.default_variable_value(self.variable_value_type(variable)?),
        }
    }

    fn variable_value_type(&self, variable: VariableId) -> Option<CanonicalValueType> {
        self.hir?
            .variables
            .get(usize::from(variable))
            .map(|variable| variable.value_type)
    }

    fn variable_identifier(&self, expr: ExprId) -> Option<VariableId> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::Identifier { name } = &expression.kind else {
            return None;
        };
        self.hir?
            .variables
            .iter()
            .find(|variable| variable.name == *name)
            .map(|variable| variable.id)
    }

    fn parameter_identifier(&self, expr: ExprId) -> Option<ParamId> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::Identifier { name } = &expression.kind else {
            return None;
        };
        self.mir
            .parameters
            .iter()
            .find(|parameter| parameter.name == *name)
            .map(|parameter| parameter.id)
    }

    fn number_constant_expr(&self, expr: ExprId) -> Option<f64> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::Number { value, .. } = expression.kind else {
            return None;
        };
        Some(value)
    }

    fn expr_references_any_variable(&self, expr: ExprId, variables: &HashSet<VariableId>) -> bool {
        let mut visited = HashSet::new();
        self.expr_references_any_variable_inner(expr, variables, &mut visited)
    }

    fn expr_references_any_variable_inner(
        &self,
        expr: ExprId,
        variables: &HashSet<VariableId>,
        visited: &mut HashSet<ExprId>,
    ) -> bool {
        if !visited.insert(expr) {
            return false;
        }
        let Some(expression) = self.mir.expressions.get(usize::from(expr)) else {
            return true;
        };
        match &expression.kind {
            HirExprKind::Identifier { .. } => self
                .variable_identifier(expr)
                .is_some_and(|variable| variables.contains(&variable)),
            HirExprKind::Binary { left, right, .. } => {
                self.expr_references_any_variable_inner(*left, variables, visited)
                    || self.expr_references_any_variable_inner(*right, variables, visited)
            }
            HirExprKind::Unary { operand, .. } => {
                self.expr_references_any_variable_inner(*operand, variables, visited)
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                self.expr_references_any_variable_inner(*condition, variables, visited)
                    || self.expr_references_any_variable_inner(*then_expr, variables, visited)
                    || self.expr_references_any_variable_inner(*else_expr, variables, visited)
            }
            HirExprKind::Call { args, .. } | HirExprKind::SystemFunction { args, .. } => args
                .iter()
                .any(|arg| self.expr_references_any_variable_inner(*arg, variables, visited)),
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Limexp { expr },
            } => self.expr_references_any_variable_inner(*expr, variables, visited),
            HirExprKind::Number { .. }
            | HirExprKind::StringLiteral { .. }
            | HirExprKind::BranchAccess { .. }
            | HirExprKind::NamedBranchAccess { .. } => false,
            HirExprKind::ArrayAccess { .. }
            | HirExprKind::ArrayLiteral { .. }
            | HirExprKind::AnalogOperator { .. }
            | HirExprKind::Laplace { .. }
            | HirExprKind::Zi { .. }
            | HirExprKind::NoiseSource { .. } => true,
        }
    }

    fn mark_loop_assignments_unknown(&mut self, loop_statement: &HirLoop) {
        self.mark_statement_assignments_unknown(&loop_statement.body);
        self.expression_values.clear();
    }

    fn mark_statement_assignments_unknown(&mut self, statements: &[HirStatement]) {
        for statement in statements {
            match statement {
                HirStatement::Assignment(assignment) => {
                    self.variable_values.insert(assignment.target, None);
                }
                HirStatement::Loop(loop_statement) => {
                    self.mark_statement_assignments_unknown(&loop_statement.body);
                }
            }
        }
    }

    fn lower_expression(&mut self, expr: ExprId) -> Option<ValueId> {
        if let Some(value) = self.expression_values.get(&expr) {
            return *value;
        }

        let expression = self.mir.expressions.get(usize::from(expr))?;
        let lowered = match &expression.kind {
            HirExprKind::Number { value, .. } => {
                Some(self.push_value(OptValueType::Real, OptValueKind::RealConstant(*value)))
            }
            HirExprKind::Identifier { name } => self.lower_identifier(name),
            HirExprKind::SystemFunction { name, args } => self.lower_system_function(name, args),
            HirExprKind::BranchAccess { access, pos, neg } => {
                self.lower_branch_access(access, pos, neg.as_deref())
            }
            HirExprKind::NamedBranchAccess { access, name } => {
                self.lower_named_branch_access(access, name)
            }
            HirExprKind::Binary { op, left, right } => self.lower_binary(op, *left, *right),
            HirExprKind::Unary { op, operand } => self.lower_unary(op, *operand),
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => self.lower_conditional(*condition, *then_expr, *else_expr),
            HirExprKind::Call { name, args } => self.lower_call(name, args),
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Limexp { expr },
            } => self.lower_intrinsic_unary(OptUnaryOp::LimExp, *expr),
            _ => None,
        };

        self.expression_values.insert(expr, lowered);
        lowered
    }

    fn lower_equation_expression(&mut self, expr: ExprId) -> Option<ValueId> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        match &expression.kind {
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Ddt { expr, abstol: None },
            } => self.lower_expression(*expr),
            HirExprKind::Call { name, args }
                if name.eq_ignore_ascii_case("ddt") && args.len() == 1 =>
            {
                self.lower_expression(args[0])
            }
            _ => self.lower_expression(expr),
        }
    }

    fn push_value(&mut self, value_type: OptValueType, kind: OptValueKind) -> ValueId {
        let (value_type, kind) = self.fold_constant_value(value_type, kind);
        if let Some(value) = self.simplified_existing_value(&kind) {
            return value;
        }

        let key = OptValueKey::from_kind(&kind);
        if let Some(value) = self.value_keys.get(&key) {
            return *value;
        }

        let id = ValueId::from(self.values.len());
        self.values.push(OptValue {
            id,
            value_type,
            kind,
            derivatives: Vec::new(),
        });
        self.value_keys.insert(key, id);
        id
    }

    fn fold_constant_value(
        &self,
        value_type: OptValueType,
        kind: OptValueKind,
    ) -> (OptValueType, OptValueKind) {
        match kind {
            OptValueKind::Unary { op, input } => {
                if let Some(value) = self.real_constant(input) {
                    let folded = match op {
                        OptUnaryOp::Pos => Some(value),
                        OptUnaryOp::Neg => Some(-value),
                        OptUnaryOp::Exp => Some(value.exp()),
                        OptUnaryOp::LimExp => Some(limexp_value(value)),
                        OptUnaryOp::LimExpDerivative => Some(limexp_derivative(value)),
                        OptUnaryOp::Ln => Some(value.ln()),
                        OptUnaryOp::Sqrt => Some(value.sqrt()),
                        OptUnaryOp::Abs => Some(value.abs()),
                        OptUnaryOp::Sin => Some(value.sin()),
                        OptUnaryOp::Cos => Some(value.cos()),
                        OptUnaryOp::Tan => Some(value.tan()),
                        OptUnaryOp::Sinh => Some(value.sinh()),
                        OptUnaryOp::Cosh => Some(value.cosh()),
                        OptUnaryOp::Tanh => Some(value.tanh()),
                        OptUnaryOp::Atan => Some(value.atan()),
                        OptUnaryOp::Asinh => Some(value.asinh()),
                        OptUnaryOp::Not => None,
                    };
                    if let Some(folded) = folded {
                        return (OptValueType::Real, OptValueKind::RealConstant(folded));
                    }
                }
                if let (OptUnaryOp::Not, Some(value)) = (op, self.constant_truth(input)) {
                    return (OptValueType::Boolean, OptValueKind::BooleanConstant(!value));
                }
                (value_type, OptValueKind::Unary { op, input })
            }
            OptValueKind::Binary { op, left, right } => {
                if let Some(folded) = self.fold_constant_binary(op, left, right) {
                    return folded;
                }
                (value_type, OptValueKind::Binary { op, left, right })
            }
            OptValueKind::Select {
                condition,
                then_value,
                else_value,
            } => {
                if let Some(condition) = self.boolean_constant(condition) {
                    return if condition {
                        (
                            self.values[usize::from(then_value)].value_type,
                            self.values[usize::from(then_value)].kind.clone(),
                        )
                    } else {
                        (
                            self.values[usize::from(else_value)].value_type,
                            self.values[usize::from(else_value)].kind.clone(),
                        )
                    };
                }
                (
                    value_type,
                    OptValueKind::Select {
                        condition,
                        then_value,
                        else_value,
                    },
                )
            }
            other => (value_type, other),
        }
    }

    fn fold_constant_binary(
        &self,
        op: OptBinaryOp,
        left: ValueId,
        right: ValueId,
    ) -> Option<(OptValueType, OptValueKind)> {
        if matches!(op, OptBinaryOp::And | OptBinaryOp::Or)
            && let (Some(left), Some(right)) =
                (self.constant_truth(left), self.constant_truth(right))
        {
            let value = match op {
                OptBinaryOp::And => left && right,
                OptBinaryOp::Or => left || right,
                _ => unreachable!(),
            };
            return Some((OptValueType::Boolean, OptValueKind::BooleanConstant(value)));
        }

        if let (Some(left), Some(right)) = (self.real_constant(left), self.real_constant(right)) {
            let real = |value| Some((OptValueType::Real, OptValueKind::RealConstant(value)));
            let boolean =
                |value| Some((OptValueType::Boolean, OptValueKind::BooleanConstant(value)));
            return match op {
                OptBinaryOp::Add => real(left + right),
                OptBinaryOp::Sub => real(left - right),
                OptBinaryOp::Mul => real(left * right),
                OptBinaryOp::Div => real(left / right),
                OptBinaryOp::Pow => real(left.powf(right)),
                OptBinaryOp::Eq => boolean(left == right),
                OptBinaryOp::Ne => boolean(left != right),
                OptBinaryOp::Lt => boolean(left < right),
                OptBinaryOp::Le => boolean(left <= right),
                OptBinaryOp::Gt => boolean(left > right),
                OptBinaryOp::Ge => boolean(left >= right),
                OptBinaryOp::And | OptBinaryOp::Or => None,
            };
        }

        if let (Some(left), Some(right)) =
            (self.boolean_constant(left), self.boolean_constant(right))
        {
            let boolean =
                |value| Some((OptValueType::Boolean, OptValueKind::BooleanConstant(value)));
            return match op {
                OptBinaryOp::Eq => boolean(left == right),
                OptBinaryOp::Ne => boolean(left != right),
                OptBinaryOp::And => boolean(left && right),
                OptBinaryOp::Or => boolean(left || right),
                _ => None,
            };
        }

        None
    }

    fn simplified_existing_value(&self, kind: &OptValueKind) -> Option<ValueId> {
        match kind {
            OptValueKind::Unary {
                op: OptUnaryOp::Pos,
                input,
            } => Some(*input),
            OptValueKind::Binary {
                op: OptBinaryOp::Mul,
                left,
                right,
            } if self.is_real_constant(*left, 1.0) => Some(*right),
            OptValueKind::Binary {
                op: OptBinaryOp::Mul,
                left,
                right,
            } if self.is_real_constant(*right, 1.0) => Some(*left),
            OptValueKind::Binary {
                op: OptBinaryOp::Div,
                left,
                right,
            } if self.is_real_constant(*right, 1.0) => Some(*left),
            _ => None,
        }
    }

    fn is_real_constant(&self, value: ValueId, expected: f64) -> bool {
        matches!(
            self.values.get(usize::from(value)).map(|value| &value.kind),
            Some(OptValueKind::RealConstant(actual)) if actual.to_bits() == expected.to_bits()
        )
    }

    fn real_constant(&self, value: ValueId) -> Option<f64> {
        match self.values.get(usize::from(value)).map(|value| &value.kind) {
            Some(OptValueKind::RealConstant(value)) => Some(*value),
            _ => None,
        }
    }

    fn boolean_constant(&self, value: ValueId) -> Option<bool> {
        match self.values.get(usize::from(value)).map(|value| &value.kind) {
            Some(OptValueKind::BooleanConstant(value)) => Some(*value),
            _ => None,
        }
    }

    fn constant_truth(&self, value: ValueId) -> Option<bool> {
        self.boolean_constant(value)
            .or_else(|| self.real_constant(value).map(real_truth_value))
    }

    fn add_sparse_derivatives(&mut self) {
        let primal_count = self.values.len();
        for index in 0..primal_count {
            let value = ValueId::from(index);
            let derivatives = self.lower_value_derivatives(value);
            self.values[index].derivatives = derivatives
                .into_iter()
                .map(|(lane, value)| OptDerivative { lane, value })
                .collect();
        }
    }

    fn lower_value_derivatives(&mut self, value: ValueId) -> BTreeMap<DerivativeLane, ValueId> {
        match self.values[usize::from(value)].kind.clone() {
            OptValueKind::RealConstant(_)
            | OptValueKind::BooleanConstant(_)
            | OptValueKind::Parameter { .. }
            | OptValueKind::ParamGiven { .. }
            | OptValueKind::Temperature
            | OptValueKind::ThermalVoltage
            | OptValueKind::Multiplicity
            | OptValueKind::Time
            | OptValueKind::EquationValue { .. } => BTreeMap::new(),
            OptValueKind::NodePotential { node } => {
                let derivative =
                    self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
                BTreeMap::from([(DerivativeLane::node(node), derivative)])
            }
            OptValueKind::BranchFlow { branch } => {
                if let Some(branch_unknown) = self.branch_unknown_for_branch(branch) {
                    let derivative =
                        self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
                    BTreeMap::from([(DerivativeLane::branch_unknown(branch_unknown), derivative)])
                } else {
                    BTreeMap::new()
                }
            }
            OptValueKind::Unary { op, input } => self.lower_unary_derivatives(value, op, input),
            OptValueKind::Binary { op, left, right } => {
                self.lower_binary_derivatives(value, op, left, right)
            }
            OptValueKind::Select {
                condition,
                then_value,
                else_value,
            } => self.lower_select_derivatives(condition, then_value, else_value),
        }
    }

    fn lower_unary_derivatives(
        &mut self,
        value: ValueId,
        op: OptUnaryOp,
        input: ValueId,
    ) -> BTreeMap<DerivativeLane, ValueId> {
        let input_derivatives = self.derivative_map(input);
        let mut derivatives = BTreeMap::new();

        for (lane, input_derivative) in input_derivatives {
            let derivative = match op {
                OptUnaryOp::Pos => input_derivative,
                OptUnaryOp::Neg => self.push_value(
                    OptValueType::Real,
                    OptValueKind::Unary {
                        op: OptUnaryOp::Neg,
                        input: input_derivative,
                    },
                ),
                OptUnaryOp::Exp => {
                    self.push_binary_value(OptBinaryOp::Mul, value, input_derivative)
                }
                OptUnaryOp::LimExp => {
                    let scale = self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::LimExpDerivative,
                            input,
                        },
                    );
                    self.push_binary_value(OptBinaryOp::Mul, scale, input_derivative)
                }
                OptUnaryOp::Ln => self.push_binary_value(OptBinaryOp::Div, input_derivative, input),
                OptUnaryOp::Sqrt => {
                    let two = self.push_value(OptValueType::Real, OptValueKind::RealConstant(2.0));
                    let denominator = self.push_binary_value(OptBinaryOp::Mul, two, value);
                    self.push_binary_value(OptBinaryOp::Div, input_derivative, denominator)
                }
                OptUnaryOp::Sin => {
                    let cos = self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Cos,
                            input,
                        },
                    );
                    self.push_binary_value(OptBinaryOp::Mul, cos, input_derivative)
                }
                OptUnaryOp::Cos => {
                    let sin = self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Sin,
                            input,
                        },
                    );
                    let scaled = self.push_binary_value(OptBinaryOp::Mul, sin, input_derivative);
                    self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Neg,
                            input: scaled,
                        },
                    )
                }
                OptUnaryOp::Tan => {
                    let cos = self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Cos,
                            input,
                        },
                    );
                    let denominator = self.push_binary_value(OptBinaryOp::Mul, cos, cos);
                    self.push_binary_value(OptBinaryOp::Div, input_derivative, denominator)
                }
                OptUnaryOp::Sinh => {
                    let cosh = self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Cosh,
                            input,
                        },
                    );
                    self.push_binary_value(OptBinaryOp::Mul, cosh, input_derivative)
                }
                OptUnaryOp::Cosh => {
                    let sinh = self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Sinh,
                            input,
                        },
                    );
                    self.push_binary_value(OptBinaryOp::Mul, sinh, input_derivative)
                }
                OptUnaryOp::Tanh => {
                    let one = self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
                    let square = self.push_binary_value(OptBinaryOp::Mul, value, value);
                    let scale = self.push_binary_value(OptBinaryOp::Sub, one, square);
                    self.push_binary_value(OptBinaryOp::Mul, scale, input_derivative)
                }
                OptUnaryOp::Atan => {
                    let one = self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
                    let square = self.push_binary_value(OptBinaryOp::Mul, input, input);
                    let denominator = self.push_binary_value(OptBinaryOp::Add, one, square);
                    self.push_binary_value(OptBinaryOp::Div, input_derivative, denominator)
                }
                OptUnaryOp::Asinh => {
                    let one = self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
                    let square = self.push_binary_value(OptBinaryOp::Mul, input, input);
                    let sum = self.push_binary_value(OptBinaryOp::Add, one, square);
                    let denominator = self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Sqrt,
                            input: sum,
                        },
                    );
                    self.push_binary_value(OptBinaryOp::Div, input_derivative, denominator)
                }
                OptUnaryOp::Abs | OptUnaryOp::Not | OptUnaryOp::LimExpDerivative => continue,
            };
            derivatives.insert(lane, derivative);
        }

        derivatives
    }

    fn lower_binary_derivatives(
        &mut self,
        value: ValueId,
        op: OptBinaryOp,
        left: ValueId,
        right: ValueId,
    ) -> BTreeMap<DerivativeLane, ValueId> {
        match op {
            OptBinaryOp::Add => self.combine_binary_derivatives(
                left,
                right,
                |builder, left, right| builder.push_binary_value(OptBinaryOp::Add, left, right),
                |_, value| value,
                |_, value| value,
            ),
            OptBinaryOp::Sub => self.combine_binary_derivatives(
                left,
                right,
                |builder, left, right| builder.push_binary_value(OptBinaryOp::Sub, left, right),
                |_, value| value,
                |builder, value| {
                    builder.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Neg,
                            input: value,
                        },
                    )
                },
            ),
            OptBinaryOp::Mul => self.product_derivatives(left, right),
            OptBinaryOp::Div => self.quotient_derivatives(left, right),
            OptBinaryOp::Pow => self.pow_derivatives(value, left, right),
            OptBinaryOp::Eq
            | OptBinaryOp::Ne
            | OptBinaryOp::Lt
            | OptBinaryOp::Le
            | OptBinaryOp::Gt
            | OptBinaryOp::Ge
            | OptBinaryOp::And
            | OptBinaryOp::Or => BTreeMap::new(),
        }
    }

    fn combine_binary_derivatives(
        &mut self,
        left: ValueId,
        right: ValueId,
        both: impl Fn(&mut Self, ValueId, ValueId) -> ValueId,
        only_left: impl Fn(&mut Self, ValueId) -> ValueId,
        only_right: impl Fn(&mut Self, ValueId) -> ValueId,
    ) -> BTreeMap<DerivativeLane, ValueId> {
        let left_derivatives = self.derivative_map(left);
        let right_derivatives = self.derivative_map(right);
        let mut lanes: BTreeSet<_> = left_derivatives.keys().copied().collect();
        lanes.extend(right_derivatives.keys().copied());

        let mut derivatives = BTreeMap::new();
        for lane in lanes {
            let derivative = match (
                left_derivatives.get(&lane).copied(),
                right_derivatives.get(&lane).copied(),
            ) {
                (Some(left), Some(right)) => both(self, left, right),
                (Some(left), None) => only_left(self, left),
                (None, Some(right)) => only_right(self, right),
                (None, None) => continue,
            };
            derivatives.insert(lane, derivative);
        }

        derivatives
    }

    fn product_derivatives(
        &mut self,
        left: ValueId,
        right: ValueId,
    ) -> BTreeMap<DerivativeLane, ValueId> {
        let left_derivatives = self.derivative_map(left);
        let right_derivatives = self.derivative_map(right);
        let mut lanes: BTreeSet<_> = left_derivatives.keys().copied().collect();
        lanes.extend(right_derivatives.keys().copied());

        let mut derivatives = BTreeMap::new();
        for lane in lanes {
            let left_term = left_derivatives
                .get(&lane)
                .copied()
                .map(|derivative| self.push_binary_value(OptBinaryOp::Mul, derivative, right));
            let right_term = right_derivatives
                .get(&lane)
                .copied()
                .map(|derivative| self.push_binary_value(OptBinaryOp::Mul, left, derivative));

            let derivative = match (left_term, right_term) {
                (Some(left_term), Some(right_term)) => {
                    self.push_binary_value(OptBinaryOp::Add, left_term, right_term)
                }
                (Some(term), None) | (None, Some(term)) => term,
                (None, None) => continue,
            };
            derivatives.insert(lane, derivative);
        }

        derivatives
    }

    fn pow_derivatives(
        &mut self,
        value: ValueId,
        left: ValueId,
        right: ValueId,
    ) -> BTreeMap<DerivativeLane, ValueId> {
        let left_derivatives = self.derivative_map(left);
        let right_derivatives = self.derivative_map(right);
        let mut lanes: BTreeSet<_> = left_derivatives.keys().copied().collect();
        lanes.extend(right_derivatives.keys().copied());

        let mut derivatives = BTreeMap::new();
        for lane in lanes {
            let left_term = left_derivatives
                .get(&lane)
                .copied()
                .map(|derivative| self.pow_base_derivative(left, right, derivative));
            let right_term = right_derivatives
                .get(&lane)
                .copied()
                .map(|derivative| self.pow_exponent_derivative(value, left, derivative));

            let derivative = match (left_term, right_term) {
                (Some(left_term), Some(right_term)) => {
                    self.push_binary_value(OptBinaryOp::Add, left_term, right_term)
                }
                (Some(term), None) | (None, Some(term)) => term,
                (None, None) => continue,
            };
            derivatives.insert(lane, derivative);
        }

        derivatives
    }

    fn pow_base_derivative(
        &mut self,
        left: ValueId,
        right: ValueId,
        derivative: ValueId,
    ) -> ValueId {
        if self.is_real_constant(right, 0.0) {
            return self.push_value(OptValueType::Real, OptValueKind::RealConstant(0.0));
        }

        let one = self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
        let exponent_minus_one = self.push_binary_value(OptBinaryOp::Sub, right, one);
        let reduced_power = self.push_binary_value(OptBinaryOp::Pow, left, exponent_minus_one);
        let scaled_power = self.push_binary_value(OptBinaryOp::Mul, right, reduced_power);
        self.push_binary_value(OptBinaryOp::Mul, scaled_power, derivative)
    }

    fn pow_exponent_derivative(
        &mut self,
        value: ValueId,
        left: ValueId,
        derivative: ValueId,
    ) -> ValueId {
        let ln_base = self.push_value(
            OptValueType::Real,
            OptValueKind::Unary {
                op: OptUnaryOp::Ln,
                input: left,
            },
        );
        let scaled = self.push_binary_value(OptBinaryOp::Mul, value, ln_base);
        self.push_binary_value(OptBinaryOp::Mul, scaled, derivative)
    }

    fn quotient_derivatives(
        &mut self,
        left: ValueId,
        right: ValueId,
    ) -> BTreeMap<DerivativeLane, ValueId> {
        let left_derivatives = self.derivative_map(left);
        let right_derivatives = self.derivative_map(right);
        let mut lanes: BTreeSet<_> = left_derivatives.keys().copied().collect();
        lanes.extend(right_derivatives.keys().copied());

        let mut derivatives = BTreeMap::new();
        for lane in lanes {
            let numerator = match (
                left_derivatives.get(&lane).copied(),
                right_derivatives.get(&lane).copied(),
            ) {
                (Some(left_derivative), Some(right_derivative)) => {
                    let left_term =
                        self.push_binary_value(OptBinaryOp::Mul, left_derivative, right);
                    let right_term =
                        self.push_binary_value(OptBinaryOp::Mul, left, right_derivative);
                    self.push_binary_value(OptBinaryOp::Sub, left_term, right_term)
                }
                (Some(left_derivative), None) => {
                    derivatives.insert(
                        lane,
                        self.push_binary_value(OptBinaryOp::Div, left_derivative, right),
                    );
                    continue;
                }
                (None, Some(right_derivative)) => {
                    let right_term =
                        self.push_binary_value(OptBinaryOp::Mul, left, right_derivative);
                    self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Neg,
                            input: right_term,
                        },
                    )
                }
                (None, None) => continue,
            };
            let denominator = self.push_binary_value(OptBinaryOp::Mul, right, right);
            let derivative = self.push_binary_value(OptBinaryOp::Div, numerator, denominator);
            derivatives.insert(lane, derivative);
        }

        derivatives
    }

    fn lower_select_derivatives(
        &mut self,
        condition: ValueId,
        then_value: ValueId,
        else_value: ValueId,
    ) -> BTreeMap<DerivativeLane, ValueId> {
        let then_derivatives = self.derivative_map(then_value);
        let else_derivatives = self.derivative_map(else_value);
        let mut lanes: BTreeSet<_> = then_derivatives.keys().copied().collect();
        lanes.extend(else_derivatives.keys().copied());

        let mut derivatives = BTreeMap::new();
        for lane in lanes {
            let then_derivative = then_derivatives.get(&lane).copied().unwrap_or_else(|| {
                self.push_value(OptValueType::Real, OptValueKind::RealConstant(0.0))
            });
            let else_derivative = else_derivatives.get(&lane).copied().unwrap_or_else(|| {
                self.push_value(OptValueType::Real, OptValueKind::RealConstant(0.0))
            });
            let derivative = self.push_value(
                OptValueType::Real,
                OptValueKind::Select {
                    condition,
                    then_value: then_derivative,
                    else_value: else_derivative,
                },
            );
            derivatives.insert(lane, derivative);
        }

        derivatives
    }

    fn derivative_map(&self, value: ValueId) -> BTreeMap<DerivativeLane, ValueId> {
        self.values[usize::from(value)]
            .derivatives
            .iter()
            .map(|derivative| (derivative.lane, derivative.value))
            .collect()
    }

    fn push_binary_value(&mut self, op: OptBinaryOp, left: ValueId, right: ValueId) -> ValueId {
        self.push_value(OptValueType::Real, OptValueKind::Binary { op, left, right })
    }

    fn lower_identifier(&mut self, name: &SmolStr) -> Option<ValueId> {
        if let Some(value) = self.lower_variable_identifier(name) {
            return value;
        }

        let parameter = self
            .mir
            .parameters
            .iter()
            .find(|parameter| parameter.name == *name)
            .map(|parameter| parameter.id)?;

        Some(self.push_value(OptValueType::Real, OptValueKind::Parameter { parameter }))
    }

    fn lower_variable_identifier(&mut self, name: &SmolStr) -> Option<Option<ValueId>> {
        let variable = self
            .hir?
            .variables
            .iter()
            .find(|variable| variable.name == *name)?;
        if let Some(value) = self.variable_values.get(&variable.id).copied() {
            return Some(value);
        }

        Some(Some(self.default_variable_value(variable.value_type)?))
    }

    fn default_variable_value(&mut self, value_type: CanonicalValueType) -> Option<ValueId> {
        match value_type {
            CanonicalValueType::Boolean => {
                Some(self.push_value(OptValueType::Boolean, OptValueKind::BooleanConstant(false)))
            }
            CanonicalValueType::Real
            | CanonicalValueType::Integer
            | CanonicalValueType::NatureAccess => {
                Some(self.push_value(OptValueType::Real, OptValueKind::RealConstant(0.0)))
            }
            _ => None,
        }
    }

    fn lower_branch_access(
        &mut self,
        access: &SmolStr,
        pos: &SmolStr,
        neg: Option<&str>,
    ) -> Option<ValueId> {
        if access.as_str() != "V" {
            return None;
        }

        let pos = self.resolve_endpoint(pos)?;
        let neg = match neg {
            Some(neg) => self.resolve_endpoint(neg)?,
            None => None,
        };

        Some(self.lower_voltage(pos, neg))
    }

    fn lower_named_branch_access(&mut self, access: &SmolStr, name: &SmolStr) -> Option<ValueId> {
        match access.as_str() {
            "I" => {
                if let Some(current) = self.branch_current_values.get(name.as_str()).copied() {
                    return Some(current);
                }
                let branch = self.branch_id_by_name(name)?;
                self.branch_unknown_for_branch(branch)?;
                Some(self.push_value(OptValueType::Real, OptValueKind::BranchFlow { branch }))
            }
            "V" => {
                let (pos, neg) = self
                    .mir
                    .branches
                    .iter()
                    .find(|branch| branch.name.as_str() == name)
                    .map(|branch| (branch.pos_node, branch.neg_node))?;
                Some(self.lower_voltage(pos, neg))
            }
            _ => None,
        }
    }

    fn lower_voltage(&mut self, pos: Option<NodeId>, neg: Option<NodeId>) -> ValueId {
        match (pos, neg) {
            (None, None) => self.push_value(OptValueType::Real, OptValueKind::RealConstant(0.0)),
            (Some(pos), None) => self.push_node_potential(pos),
            (None, Some(neg)) => {
                let zero = self.push_value(OptValueType::Real, OptValueKind::RealConstant(0.0));
                let neg = self.push_node_potential(neg);
                self.push_value(
                    OptValueType::Real,
                    OptValueKind::Binary {
                        op: OptBinaryOp::Sub,
                        left: zero,
                        right: neg,
                    },
                )
            }
            (Some(pos), Some(neg)) => {
                let pos = self.push_node_potential(pos);
                let neg = self.push_node_potential(neg);
                self.push_value(
                    OptValueType::Real,
                    OptValueKind::Binary {
                        op: OptBinaryOp::Sub,
                        left: pos,
                        right: neg,
                    },
                )
            }
        }
    }

    fn push_node_potential(&mut self, node: NodeId) -> ValueId {
        self.push_value(OptValueType::Real, OptValueKind::NodePotential { node })
    }

    fn cache_declared_branch_current(&mut self, equation: &MirEquation, value: Option<ValueId>) {
        if equation.kind != MirEquationKind::Current {
            return;
        }
        let Some(value) = value else {
            return;
        };
        let Some(branch_name) = self.declared_contribution_branch_name(equation) else {
            return;
        };

        let value = if let Some(previous) = self.branch_current_values.get(branch_name.as_str()) {
            self.push_value(
                OptValueType::Real,
                OptValueKind::Binary {
                    op: OptBinaryOp::Add,
                    left: *previous,
                    right: value,
                },
            )
        } else {
            value
        };
        self.branch_current_values.insert(branch_name, value);
    }

    fn declared_contribution_branch_name(&self, equation: &MirEquation) -> Option<String> {
        if let Some(name) = equation.branch.declared_name.as_deref() {
            return Some(name.to_string());
        }
        if self
            .mir
            .branches
            .iter()
            .any(|branch| branch.name.as_str() == equation.branch.label.as_str())
        {
            return Some(equation.branch.label.to_string());
        }

        let mut matches = self.mir.branches.iter().filter(|branch| {
            branch.pos_node == equation.branch.pos_node
                && branch.neg_node == equation.branch.neg_node
        });
        let first = matches.next()?;
        if matches.next().is_none() {
            Some(first.name.to_string())
        } else {
            None
        }
    }

    fn branch_id_by_name(&self, name: &str) -> Option<BranchId> {
        self.mir
            .branches
            .iter()
            .find(|branch| branch.name.as_str() == name)
            .map(|branch| branch.id)
    }

    fn branch_unknown_for_branch(&self, branch_id: BranchId) -> Option<BranchUnknownId> {
        let branch = self.mir.branches.get(usize::from(branch_id))?;
        if let Some(unknown) = self.mir.branch_unknowns.iter().find(|unknown| {
            unknown
                .declared_name
                .as_deref()
                .is_some_and(|name| name == branch.name.as_str())
        }) {
            return Some(unknown.id);
        }

        let mut matches = self.mir.branch_unknowns.iter().filter(|unknown| {
            unknown.pos_node == branch.pos_node && unknown.neg_node == branch.neg_node
        });
        let first = matches.next()?;
        if matches.next().is_none() {
            Some(first.id)
        } else {
            None
        }
    }

    fn resolve_endpoint(&self, name: &str) -> Option<Option<NodeId>> {
        if name == "0"
            || self
                .mir
                .ground_nodes
                .iter()
                .any(|ground| ground.as_str() == name)
        {
            return Some(None);
        }

        self.mir
            .nodes
            .iter()
            .find(|node| node.name.as_str() == name)
            .map(|node| Some(node.id))
    }

    fn lower_binary(&mut self, op: &SmolStr, left: ExprId, right: ExprId) -> Option<ValueId> {
        let op = binary_op(op)?;
        let left = self.lower_expression(left)?;
        let right = self.lower_expression(right)?;
        let value_type = match op {
            OptBinaryOp::Eq
            | OptBinaryOp::Ne
            | OptBinaryOp::Lt
            | OptBinaryOp::Le
            | OptBinaryOp::Gt
            | OptBinaryOp::Ge
            | OptBinaryOp::And
            | OptBinaryOp::Or => OptValueType::Boolean,
            OptBinaryOp::Add
            | OptBinaryOp::Sub
            | OptBinaryOp::Mul
            | OptBinaryOp::Div
            | OptBinaryOp::Pow => OptValueType::Real,
        };

        Some(self.push_value(value_type, OptValueKind::Binary { op, left, right }))
    }

    fn lower_unary(&mut self, op: &SmolStr, operand: ExprId) -> Option<ValueId> {
        let op = unary_op(op)?;
        self.lower_intrinsic_unary(op, operand)
    }

    fn lower_intrinsic_unary(&mut self, op: OptUnaryOp, input: ExprId) -> Option<ValueId> {
        let input = self.lower_expression(input)?;
        let value_type = if op == OptUnaryOp::Not {
            OptValueType::Boolean
        } else {
            OptValueType::Real
        };

        Some(self.push_value(value_type, OptValueKind::Unary { op, input }))
    }

    fn lower_conditional(
        &mut self,
        condition: ExprId,
        then_expr: ExprId,
        else_expr: ExprId,
    ) -> Option<ValueId> {
        let condition = self.lower_expression(condition)?;
        let then_value = self.lower_expression(then_expr)?;
        let else_value = self.lower_expression(else_expr)?;
        let value_type = self.values[usize::from(then_value)].value_type;

        Some(self.push_value(
            value_type,
            OptValueKind::Select {
                condition,
                then_value,
                else_value,
            },
        ))
    }

    fn lower_call(&mut self, name: &SmolStr, args: &[ExprId]) -> Option<ValueId> {
        if args.len() == 2 {
            return self.lower_binary_intrinsic_call(name, args[0], args[1]);
        }
        if args.len() == 1 {
            let op = match name.as_str() {
                "exp" => OptUnaryOp::Exp,
                "limexp" => OptUnaryOp::LimExp,
                "ln" | "log" => OptUnaryOp::Ln,
                "sqrt" => OptUnaryOp::Sqrt,
                "abs" => OptUnaryOp::Abs,
                "sin" => OptUnaryOp::Sin,
                "cos" => OptUnaryOp::Cos,
                "tan" => OptUnaryOp::Tan,
                "sinh" => OptUnaryOp::Sinh,
                "cosh" => OptUnaryOp::Cosh,
                "tanh" => OptUnaryOp::Tanh,
                "atan" => OptUnaryOp::Atan,
                "asinh" => OptUnaryOp::Asinh,
                _ => return None,
            };
            return self.lower_intrinsic_unary(op, args[0]);
        }

        None
    }

    fn lower_binary_intrinsic_call(
        &mut self,
        name: &SmolStr,
        left: ExprId,
        right: ExprId,
    ) -> Option<ValueId> {
        let op = match name.as_str() {
            "min" => OptBinaryOp::Lt,
            "max" => OptBinaryOp::Gt,
            "pow" => {
                let left = self.lower_expression(left)?;
                let right = self.lower_expression(right)?;
                return Some(self.push_binary_value(OptBinaryOp::Pow, left, right));
            }
            _ => return None,
        };
        let left = self.lower_expression(left)?;
        let right = self.lower_expression(right)?;
        let condition = self.push_value(
            OptValueType::Boolean,
            OptValueKind::Binary { op, left, right },
        );
        Some(self.push_value(
            OptValueType::Real,
            OptValueKind::Select {
                condition,
                then_value: left,
                else_value: right,
            },
        ))
    }

    fn lower_system_function(&mut self, name: &SmolStr, args: &[ExprId]) -> Option<ValueId> {
        match name.to_ascii_lowercase().as_str() {
            "$temperature" if args.is_empty() => {
                Some(self.push_value(OptValueType::Real, OptValueKind::Temperature))
            }
            "$abstime" | "$realtime" if args.is_empty() => {
                Some(self.push_value(OptValueType::Real, OptValueKind::Time))
            }
            "$mfactor" if args.is_empty() => {
                Some(self.push_value(OptValueType::Real, OptValueKind::Multiplicity))
            }
            "$vt" | "$thermal_vt" if args.is_empty() => {
                Some(self.push_value(OptValueType::Real, OptValueKind::ThermalVoltage))
            }
            "$vt" | "$thermal_vt" if args.len() == 1 => {
                let temperature = self.lower_expression(args[0])?;
                let scale = self.push_value(
                    OptValueType::Real,
                    OptValueKind::RealConstant(THERMAL_VOLTAGE_PER_K),
                );
                Some(self.push_binary_value(OptBinaryOp::Mul, temperature, scale))
            }
            "$simparam" if args.len() == 1 => {
                let default = self.simparam_default(args[0])?;
                Some(self.push_value(OptValueType::Real, OptValueKind::RealConstant(default)))
            }
            "$simparam" if args.len() == 2 => self.lower_expression(args[1]),
            "$param_given" if args.len() == 1 => {
                let parameter = self.parameter_arg(args[0])?;
                Some(self.push_value(OptValueType::Real, OptValueKind::ParamGiven { parameter }))
            }
            "$port_connected" if args.len() == 1 => {
                Some(self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0)))
            }
            _ => None,
        }
    }

    fn parameter_arg(&self, expr: ExprId) -> Option<ParamId> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::Identifier { name } = &expression.kind else {
            return None;
        };
        self.mir
            .parameters
            .iter()
            .find(|parameter| parameter.name == *name)
            .map(|parameter| parameter.id)
    }

    fn simparam_default(&self, expr: ExprId) -> Option<f64> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::StringLiteral { value } = &expression.kind else {
            return None;
        };
        match value.to_ascii_lowercase().as_str() {
            "gmin" => Some(1.0e-12),
            "tnom" => Some(27.0),
            "scale" => Some(1.0),
            "temp" | "temperature" => Some(27.0),
            _ => None,
        }
    }
}

fn binary_op(op: &str) -> Option<OptBinaryOp> {
    match op {
        "Add" => Some(OptBinaryOp::Add),
        "Sub" => Some(OptBinaryOp::Sub),
        "Mul" => Some(OptBinaryOp::Mul),
        "Div" => Some(OptBinaryOp::Div),
        "Pow" => Some(OptBinaryOp::Pow),
        "Eq" => Some(OptBinaryOp::Eq),
        "Ne" => Some(OptBinaryOp::Ne),
        "Lt" => Some(OptBinaryOp::Lt),
        "Le" => Some(OptBinaryOp::Le),
        "Gt" => Some(OptBinaryOp::Gt),
        "Ge" => Some(OptBinaryOp::Ge),
        "And" => Some(OptBinaryOp::And),
        "Or" => Some(OptBinaryOp::Or),
        _ => None,
    }
}

fn unary_op(op: &str) -> Option<OptUnaryOp> {
    match op {
        "Pos" => Some(OptUnaryOp::Pos),
        "Neg" => Some(OptUnaryOp::Neg),
        "Not" => Some(OptUnaryOp::Not),
        _ => None,
    }
}

pub(crate) fn limexp_value(value: f64) -> f64 {
    if value < 80.0 {
        value.exp()
    } else {
        LIMEXP_MAX * (1.0 + (value - 80.0))
    }
}

pub(crate) fn limexp_derivative(value: f64) -> f64 {
    if value < 80.0 {
        value.exp()
    } else {
        LIMEXP_MAX
    }
}

pub(crate) fn real_truth_value(value: f64) -> bool {
    value != 0.0
}

fn supported_assignment_value_type(value_type: CanonicalValueType) -> bool {
    matches!(
        value_type,
        CanonicalValueType::Real
            | CanonicalValueType::Integer
            | CanonicalValueType::Boolean
            | CanonicalValueType::NatureAccess
    )
}

fn remap_value_id(value: ValueId, remap: &[Option<ValueId>]) -> ValueId {
    remap
        .get(usize::from(value))
        .and_then(|value| *value)
        .expect("live OptIR value must have a compacted id")
}

fn remap_value_kind(kind: &OptValueKind, remap: &[Option<ValueId>]) -> OptValueKind {
    match kind {
        OptValueKind::RealConstant(value) => OptValueKind::RealConstant(*value),
        OptValueKind::BooleanConstant(value) => OptValueKind::BooleanConstant(*value),
        OptValueKind::Parameter { parameter } => OptValueKind::Parameter {
            parameter: *parameter,
        },
        OptValueKind::ParamGiven { parameter } => OptValueKind::ParamGiven {
            parameter: *parameter,
        },
        OptValueKind::Temperature => OptValueKind::Temperature,
        OptValueKind::ThermalVoltage => OptValueKind::ThermalVoltage,
        OptValueKind::Multiplicity => OptValueKind::Multiplicity,
        OptValueKind::Time => OptValueKind::Time,
        OptValueKind::NodePotential { node } => OptValueKind::NodePotential { node: *node },
        OptValueKind::BranchFlow { branch } => OptValueKind::BranchFlow { branch: *branch },
        OptValueKind::Unary { op, input } => OptValueKind::Unary {
            op: *op,
            input: remap_value_id(*input, remap),
        },
        OptValueKind::Binary { op, left, right } => OptValueKind::Binary {
            op: *op,
            left: remap_value_id(*left, remap),
            right: remap_value_id(*right, remap),
        },
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => OptValueKind::Select {
            condition: remap_value_id(*condition, remap),
            then_value: remap_value_id(*then_value, remap),
            else_value: remap_value_id(*else_value, remap),
        },
        OptValueKind::EquationValue { equation } => OptValueKind::EquationValue {
            equation: *equation,
        },
    }
}

fn validate_values(diagnostics: &mut Vec<IrDiagnostic>, opt: &OptModel) {
    for value in &opt.values {
        validate_value_kind(diagnostics, opt, value);
        validate_derivatives(diagnostics, opt, value);
    }
}

fn validate_value_kind(diagnostics: &mut Vec<IrDiagnostic>, opt: &OptModel, value: &OptValue) {
    match &value.kind {
        OptValueKind::RealConstant(_) | OptValueKind::BooleanConstant(_) => {}
        OptValueKind::Parameter { parameter } | OptValueKind::ParamGiven { parameter } => {
            if parameter.index() >= opt.parameter_count {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::OptValidation,
                    format!(
                        "OptIR value {} parameter {} is out of range for {} parameters",
                        value.id, parameter, opt.parameter_count
                    ),
                ));
            }
        }
        OptValueKind::Temperature
        | OptValueKind::ThermalVoltage
        | OptValueKind::Multiplicity
        | OptValueKind::Time => {}
        OptValueKind::NodePotential { node } => {
            if node.index() >= opt.node_count {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::OptValidation,
                    format!(
                        "OptIR value {} node {} is out of range for {} nodes",
                        value.id, node, opt.node_count
                    ),
                ));
            }
        }
        OptValueKind::BranchFlow { branch } => {
            if branch.index() >= opt.branch_count {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::OptValidation,
                    format!(
                        "OptIR value {} branch {} is out of range for {} branches",
                        value.id, branch, opt.branch_count
                    ),
                ));
            }
        }
        OptValueKind::Unary { input, .. } => {
            validate_value_operand(diagnostics, opt.values.len(), value.id, *input, "operand");
        }
        OptValueKind::Binary { left, right, .. } => {
            validate_value_operand(
                diagnostics,
                opt.values.len(),
                value.id,
                *left,
                "left operand",
            );
            validate_value_operand(
                diagnostics,
                opt.values.len(),
                value.id,
                *right,
                "right operand",
            );
        }
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => {
            validate_value_operand(
                diagnostics,
                opt.values.len(),
                value.id,
                *condition,
                "condition operand",
            );
            validate_value_operand(
                diagnostics,
                opt.values.len(),
                value.id,
                *then_value,
                "then operand",
            );
            validate_value_operand(
                diagnostics,
                opt.values.len(),
                value.id,
                *else_value,
                "else operand",
            );
        }
        OptValueKind::EquationValue { equation } => {
            if equation.index() >= opt.equation_count {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::OptValidation,
                    format!(
                        "OptIR value {} equation {} is out of range for {} equations",
                        value.id, equation, opt.equation_count
                    ),
                ));
            }
        }
    }
}

fn validate_value_operand(
    diagnostics: &mut Vec<IrDiagnostic>,
    value_count: usize,
    owner: ValueId,
    operand: ValueId,
    label: &str,
) {
    if usize::from(operand) >= value_count {
        diagnostics.push(IrDiagnostic::global_error(
            CompilerPhase::OptValidation,
            format!(
                "OptIR value {} {} {} is out of range for {} values",
                owner, label, operand, value_count
            ),
        ));
        return;
    }

    if operand.index() >= owner.index() {
        diagnostics.push(IrDiagnostic::global_error(
            CompilerPhase::OptValidation,
            format!(
                "OptIR value {} {} {} violates scalar value topological order",
                owner, label, operand
            ),
        ));
    }
}

fn validate_derivatives(diagnostics: &mut Vec<IrDiagnostic>, opt: &OptModel, value: &OptValue) {
    let mut previous_lane = None;
    let mut lanes = HashSet::new();

    for derivative in &value.derivatives {
        validate_derivative_lane(diagnostics, opt, value.id, derivative.lane);
        validate_value_reference(
            diagnostics,
            opt.values.len(),
            value.id,
            derivative.value,
            "derivative value",
        );

        if let Some(previous_lane) = previous_lane
            && previous_lane > derivative.lane
        {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR value {} derivative lanes must be sorted by lane",
                    value.id
                ),
            ));
        }
        previous_lane = Some(derivative.lane);

        if !lanes.insert(derivative.lane) {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR value {} has duplicate derivative lane {:?}",
                    value.id, derivative.lane
                ),
            ));
        }
    }
}

fn validate_value_reference(
    diagnostics: &mut Vec<IrDiagnostic>,
    value_count: usize,
    owner: ValueId,
    reference: ValueId,
    label: &str,
) {
    if usize::from(reference) >= value_count {
        diagnostics.push(IrDiagnostic::global_error(
            CompilerPhase::OptValidation,
            format!(
                "OptIR value {} {} {} is out of range for {} values",
                owner, label, reference, value_count
            ),
        ));
    }
}

fn validate_derivative_lane(
    diagnostics: &mut Vec<IrDiagnostic>,
    opt: &OptModel,
    owner: ValueId,
    lane: DerivativeLane,
) {
    let limit = match lane.kind {
        DerivativeLaneKind::Node => opt.node_count,
        DerivativeLaneKind::BranchUnknown => opt.branch_unknown_count,
    };

    if lane.index >= limit {
        diagnostics.push(IrDiagnostic::global_error(
            CompilerPhase::OptValidation,
            format!(
                "OptIR value {} derivative lane {:?} is out of range for limit {}",
                owner, lane, limit
            ),
        ));
    }
}

fn validate_dense_value_ids(diagnostics: &mut Vec<IrDiagnostic>, values: &[OptValue]) {
    for (expected, value) in values.iter().enumerate() {
        let expected = u32::try_from(expected).expect("OptIR value count exceeds u32::MAX");
        if value.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR value IDs must be dense: expected ValueId({}) at index {}, found {}",
                    expected, expected, value.id
                ),
            ));
        }
    }
}

fn collect_instance_static_ops(values: &[OptValue]) -> Vec<OptOp> {
    let mut invalidation_memo = vec![None; values.len()];
    let mut parameter_memo = vec![None; values.len()];
    values
        .iter()
        .filter(|value| {
            value_invalidation(values, value.id, &mut invalidation_memo)
                == InvalidationClass::InstanceStatic
                && value_depends_on_parameter(values, value.id, &mut parameter_memo)
        })
        .map(|value| OptOp::ComputeValue { value: value.id })
        .collect()
}

fn value_invalidation(
    values: &[OptValue],
    value: ValueId,
    memo: &mut [Option<InvalidationClass>],
) -> InvalidationClass {
    let index = usize::from(value);
    if let Some(invalidation) = memo[index] {
        return invalidation;
    }

    let invalidation = match values[index].kind {
        OptValueKind::RealConstant(_)
        | OptValueKind::BooleanConstant(_)
        | OptValueKind::Parameter { .. }
        | OptValueKind::ParamGiven { .. } => InvalidationClass::InstanceStatic,
        OptValueKind::Temperature | OptValueKind::ThermalVoltage => {
            InvalidationClass::TemperatureStatic
        }
        OptValueKind::Multiplicity | OptValueKind::Time => InvalidationClass::NewtonIteration,
        OptValueKind::NodePotential { .. }
        | OptValueKind::BranchFlow { .. }
        | OptValueKind::EquationValue { .. } => InvalidationClass::NewtonIteration,
        OptValueKind::Unary { input, .. } => value_invalidation(values, input, memo),
        OptValueKind::Binary { left, right, .. } => max_invalidation(
            value_invalidation(values, left, memo),
            value_invalidation(values, right, memo),
        ),
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => max_invalidation(
            value_invalidation(values, condition, memo),
            max_invalidation(
                value_invalidation(values, then_value, memo),
                value_invalidation(values, else_value, memo),
            ),
        ),
    };
    memo[index] = Some(invalidation);
    invalidation
}

fn value_depends_on_parameter(
    values: &[OptValue],
    value: ValueId,
    memo: &mut [Option<bool>],
) -> bool {
    let index = usize::from(value);
    if let Some(depends) = memo[index] {
        return depends;
    }

    let depends = match values[index].kind {
        OptValueKind::Parameter { .. } | OptValueKind::ParamGiven { .. } => true,
        OptValueKind::RealConstant(_)
        | OptValueKind::BooleanConstant(_)
        | OptValueKind::Temperature
        | OptValueKind::ThermalVoltage
        | OptValueKind::Multiplicity
        | OptValueKind::Time
        | OptValueKind::NodePotential { .. }
        | OptValueKind::BranchFlow { .. }
        | OptValueKind::EquationValue { .. } => false,
        OptValueKind::Unary { input, .. } => value_depends_on_parameter(values, input, memo),
        OptValueKind::Binary { left, right, .. } => {
            value_depends_on_parameter(values, left, memo)
                || value_depends_on_parameter(values, right, memo)
        }
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => {
            value_depends_on_parameter(values, condition, memo)
                || value_depends_on_parameter(values, then_value, memo)
                || value_depends_on_parameter(values, else_value, memo)
        }
    };
    memo[index] = Some(depends);
    depends
}

fn max_invalidation(left: InvalidationClass, right: InvalidationClass) -> InvalidationClass {
    if invalidation_rank(left) >= invalidation_rank(right) {
        left
    } else {
        right
    }
}

fn validate_dense_schedule_ids(diagnostics: &mut Vec<IrDiagnostic>, schedules: &[OptSchedule]) {
    for (expected, schedule) in schedules.iter().enumerate() {
        let expected = u32::try_from(expected).expect("OptIR schedule count exceeds u32::MAX");
        if schedule.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR schedule IDs must be dense: expected ScheduleId({}) at index {}, found {}",
                    expected, expected, schedule.id
                ),
            ));
        }
    }
}

fn validate_schedules(
    diagnostics: &mut Vec<IrDiagnostic>,
    schedules: &[OptSchedule],
    value_count: usize,
    equation_count: u32,
) {
    let mut invalidations = HashSet::new();
    let mut newton_count = 0;
    let mut previous_invalidation = None;

    for schedule in schedules {
        if let Some(previous) = previous_invalidation {
            if invalidation_rank(previous) > invalidation_rank(schedule.invalidation) {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::OptValidation,
                    format!(
                        "OptIR schedule order must follow invalidation order: {:?} appears before {:?}",
                        previous, schedule.invalidation
                    ),
                ));
            }
        }
        previous_invalidation = Some(schedule.invalidation);

        if !invalidations.insert(schedule.invalidation) {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR duplicate schedule for invalidation {:?}",
                    schedule.invalidation
                ),
            ));
        }

        if schedule.invalidation == InvalidationClass::NewtonIteration {
            newton_count += 1;
            validate_newton_schedule(diagnostics, schedule, value_count, equation_count);
        } else {
            validate_schedule_ops(diagnostics, schedule, value_count, equation_count);
        }
    }

    if newton_count != 1 {
        diagnostics.push(IrDiagnostic::global_error(
            CompilerPhase::OptValidation,
            format!(
                "OptIR must contain exactly one NewtonIteration schedule, found {}",
                newton_count
            ),
        ));
    }
}

fn invalidation_rank(invalidation: InvalidationClass) -> u8 {
    match invalidation {
        InvalidationClass::InstanceStatic => 0,
        InvalidationClass::TemperatureStatic => 1,
        InvalidationClass::TimestepStatic => 2,
        InvalidationClass::OperatingPointStatic => 3,
        InvalidationClass::NewtonIteration => 4,
        InvalidationClass::AcFrequency => 5,
        InvalidationClass::NoiseFrequency => 6,
        InvalidationClass::OperatingPointReport => 7,
    }
}

fn validate_newton_schedule(
    diagnostics: &mut Vec<IrDiagnostic>,
    schedule: &OptSchedule,
    value_count: usize,
    equation_count: u32,
) {
    validate_schedule_ops(diagnostics, schedule, value_count, equation_count);

    let equation_ops: Vec<_> = schedule
        .ops
        .iter()
        .filter_map(|op| match op {
            OptOp::EvaluateEquation { equation } => Some(*equation),
            OptOp::ComputeValue { .. } => None,
        })
        .collect();

    if equation_ops.len() != equation_count as usize {
        diagnostics.push(IrDiagnostic::global_error(
            CompilerPhase::OptValidation,
            format!(
                "OptIR NewtonIteration schedule must contain one op per equation: expected {}, found {}",
                equation_count,
                equation_ops.len()
            ),
        ));
    }

    for (expected, equation) in equation_ops.iter().copied().enumerate() {
        let expected = u32::try_from(expected).expect("OptIR op count exceeds u32::MAX");
        if equation.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR NewtonIteration op at index {} must evaluate EquationId({}), found {}",
                    expected, expected, equation
                ),
            ));
        }
    }
}

fn validate_schedule_ops(
    diagnostics: &mut Vec<IrDiagnostic>,
    schedule: &OptSchedule,
    value_count: usize,
    equation_count: u32,
) {
    let mut equations = HashSet::new();

    for op in &schedule.ops {
        match op {
            OptOp::ComputeValue { value } => {
                if usize::from(*value) >= value_count {
                    diagnostics.push(IrDiagnostic::global_error(
                        CompilerPhase::OptValidation,
                        format!(
                            "OptIR schedule {} ComputeValue {} is out of range for {} values",
                            schedule.id, value, value_count
                        ),
                    ));
                }
            }
            OptOp::EvaluateEquation { equation } => {
                if equation.index() >= equation_count {
                    diagnostics.push(IrDiagnostic::global_error(
                        CompilerPhase::OptValidation,
                        format!(
                            "OptIR schedule {} EvaluateEquation {} is out of range for {} equations",
                            schedule.id, equation, equation_count
                        ),
                    ));
                }

                if !equations.insert(*equation) {
                    diagnostics.push(IrDiagnostic::global_error(
                        CompilerPhase::OptValidation,
                        format!(
                            "OptIR schedule {} has duplicate equation {}",
                            schedule.id, equation
                        ),
                    ));
                }
            }
        }
    }
}
