use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use super::{
    BranchId, BranchUnknownId, CanonicalValueType, CompilerPhase, EquationId, ExprId,
    HirAnalogOperator, HirExprKind, HirModel, HirStatement, IrDiagnostic, IrValidationResult,
    MirModel, NodeId, ParamId, ScheduleId, ValueId, VariableId,
};

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
        let mut equation_values: Vec<_> = mir
            .equations
            .iter()
            .map(|equation| builder.lower_expression(equation.expression.id))
            .collect();
        builder.add_sparse_derivatives();
        let values = builder.finish(&mut equation_values);

        let mut schedules = Vec::new();
        if !mir.parameters.is_empty() {
            schedules.push(OptSchedule {
                id: ScheduleId::from(schedules.len()),
                invalidation: InvalidationClass::InstanceStatic,
                ops: Vec::new(),
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum OptValueKey {
    RealConstant(u64),
    BooleanConstant(bool),
    Parameter(ParamId),
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
            match statement {
                HirStatement::Assignment(assignment) => {
                    let value = if assignment.index.is_none()
                        && supported_assignment_value_type(assignment.expr_type)
                    {
                        self.lower_expression(assignment.expr.id)
                    } else {
                        None
                    };
                    self.variable_values.insert(assignment.target, value);
                }
                HirStatement::Loop(_) => {}
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
            HirExprKind::BranchAccess { access, pos, neg } => {
                self.lower_branch_access(access, pos, neg.as_deref())
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
            } => self.lower_intrinsic_unary(OptUnaryOp::Exp, *expr),
            _ => None,
        };

        self.expression_values.insert(expr, lowered);
        lowered
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
                if let (OptUnaryOp::Not, Some(value)) = (op, self.boolean_constant(input)) {
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
            other => (value_type, other),
        }
    }

    fn fold_constant_binary(
        &self,
        op: OptBinaryOp,
        left: ValueId,
        right: ValueId,
    ) -> Option<(OptValueType, OptValueKind)> {
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
            | OptValueKind::EquationValue { .. } => BTreeMap::new(),
            OptValueKind::NodePotential { node } => {
                let derivative =
                    self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
                BTreeMap::from([(DerivativeLane::node(node), derivative)])
            }
            OptValueKind::BranchFlow { .. } => BTreeMap::new(),
            OptValueKind::Unary { op, input } => self.lower_unary_derivatives(value, op, input),
            OptValueKind::Binary { op, left, right } => {
                self.lower_binary_derivatives(op, left, right)
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
                OptUnaryOp::Abs | OptUnaryOp::Not => continue,
            };
            derivatives.insert(lane, derivative);
        }

        derivatives
    }

    fn lower_binary_derivatives(
        &mut self,
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
            OptBinaryOp::Pow
            | OptBinaryOp::Eq
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

    fn lower_variable_identifier(&self, name: &SmolStr) -> Option<Option<ValueId>> {
        let variable = self
            .hir?
            .variables
            .iter()
            .find(|variable| variable.name == *name)?;
        self.variable_values.get(&variable.id).copied()
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
        if args.len() != 1 {
            return None;
        }

        let op = match name.as_str() {
            "exp" => OptUnaryOp::Exp,
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
        self.lower_intrinsic_unary(op, args[0])
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

fn supported_assignment_value_type(value_type: CanonicalValueType) -> bool {
    matches!(
        value_type,
        CanonicalValueType::Real | CanonicalValueType::Integer | CanonicalValueType::Boolean
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
        OptValueKind::Parameter { parameter } => {
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
