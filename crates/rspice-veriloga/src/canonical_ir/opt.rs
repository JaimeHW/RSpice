use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::HashSet;

use super::{
    BranchId, BranchUnknownId, CompilerPhase, EquationId, IrDiagnostic, IrValidationResult,
    MirModel, NodeId, ParamId, ScheduleId, ValueId,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptUnaryOp {
    Pos,
    Neg,
    Not,
    Exp,
    Ln,
    Sqrt,
    Abs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

        let mut schedules = Vec::new();
        if !mir.parameters.is_empty() {
            schedules.push(OptSchedule {
                id: ScheduleId::from(schedules.len()),
                invalidation: InvalidationClass::InstanceStatic,
                ops: Vec::new(),
            });
        }

        let newton_ops = mir
            .equations
            .iter()
            .map(|equation| OptOp::EvaluateEquation {
                equation: equation.id,
            })
            .collect();
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
            values: Vec::new(),
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
