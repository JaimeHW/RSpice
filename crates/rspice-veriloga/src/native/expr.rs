#![allow(dead_code)]

use super::{JitError, JitResult};
use crate::codegen::{BytecodeProgram, CompiledModel, Instruction};
use smol_str::SmolStr;

const LOGICAL_EPSILON: f64 = 1.0e-15;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EntryKind {
    Assignment,
    ParameterDefault,
    StaticCondition,
    StampValue,
    Jacobian,
    ReactiveJacobian,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum VoltageNode {
    Terminal(usize),
    Internal(usize),
    Ground,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum NativeOp {
    Const(f64),
    LoadParam(usize),
    LoadParamGiven(usize),
    LoadPortConnected(usize),
    LoadVoltage { pos: VoltageNode, neg: VoltageNode },
    LoadCurrent(usize),
    LoadInternalVoltage(usize),
    LoadVariable(usize),
    LoadVariableDyn { base: usize, len: usize, lower: i64 },
    LoadBranchUnknown(usize),
    LoadTemperature,
    LoadThermalVoltage,
    LoadTime,
    Analysis(u8),
    LoadMfactor,
    Add,
    Sub,
    Mul,
    Div,
    AddConst(f64),
    SubConst(f64),
    MulConst(f64),
    DivConst(f64),
    SubFromConst(f64),
    DivFromConst(f64),
    Neg,
    Abs,
    Square,
    Sqrt,
    Compare(CompareOp),
    CompareConst(CompareOp, f64),
    Logical(LogicalOp),
    LogicalConst(LogicalOp, bool),
    IfElse,
    Extremum(ExtremumOp),
    ExtremumConst(ExtremumOp, f64),
    UnaryMath(UnaryMathOp),
    BinaryMath(BinaryMathOp),
    IntegerBinary(IntegerBinaryOp),
    TableLookup(usize),
    TableDerivative(usize),
    LimitState(usize),
    LaplaceState(usize),
    ZiState(usize),
    TimerState(usize),
    TransitionState(usize),
    SlewState(usize),
    AbsDelayState(usize),
    CrossState(usize),
    WhiteNoise,
    FlickerNoise,
    DdtState(usize),
    DdtJacobian,
    IdtState(usize),
    IdtJacobian,
    IdtModState(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompareOp {
    Gt,
    Lt,
    Ge,
    Le,
    Eq,
    Ne,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LogicalOp {
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExtremumOp {
    Min,
    Max,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnaryMathOp {
    Exp,
    Log,
    Log10,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Limexp,
    Asin,
    Acos,
    Atan,
    Floor,
    Ceil,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BinaryMathOp {
    Pow,
    Atan2,
    Mod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IntegerBinaryOp {
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NativeProgram {
    ops: Vec<NativeOp>,
    max_stack_depth: usize,
    current_pair_dependencies: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NativeLoweringLimits<'a> {
    terminal_count: usize,
    internal_node_count: usize,
    parameter_count: usize,
    variable_count: usize,
    branch_unknown_count: usize,
    lookup_table_count: usize,
    laplace_filter_count: usize,
    zi_filter_count: usize,
    available_current_pairs: &'a [usize],
}

impl<'a> NativeLoweringLimits<'a> {
    pub(crate) fn new(
        terminal_count: usize,
        internal_node_count: usize,
        parameter_count: usize,
        variable_count: usize,
        branch_unknown_count: usize,
    ) -> Self {
        Self {
            terminal_count,
            internal_node_count,
            parameter_count,
            variable_count,
            branch_unknown_count,
            lookup_table_count: 0,
            laplace_filter_count: 0,
            zi_filter_count: 0,
            available_current_pairs: &[],
        }
    }

    pub(crate) fn for_model(model: &CompiledModel) -> NativeLoweringLimits<'static> {
        NativeLoweringLimits::new(
            model.num_terminals,
            model.internal_nodes,
            model.parameters.len(),
            model.num_variables,
            model.branch_sources.len(),
        )
        .with_lookup_table_count(model.lookup_tables.len())
        .with_laplace_filter_count(model.laplace_filters.len())
        .with_zi_filter_count(model.zi_filters.len())
    }

    pub(crate) fn with_available_current_pairs<'b>(
        self,
        available_current_pairs: &'b [usize],
    ) -> NativeLoweringLimits<'b> {
        NativeLoweringLimits {
            terminal_count: self.terminal_count,
            internal_node_count: self.internal_node_count,
            parameter_count: self.parameter_count,
            variable_count: self.variable_count,
            branch_unknown_count: self.branch_unknown_count,
            lookup_table_count: self.lookup_table_count,
            laplace_filter_count: self.laplace_filter_count,
            zi_filter_count: self.zi_filter_count,
            available_current_pairs,
        }
    }

    pub(crate) fn with_lookup_table_count(self, lookup_table_count: usize) -> Self {
        Self {
            lookup_table_count,
            ..self
        }
    }

    pub(crate) fn with_laplace_filter_count(self, laplace_filter_count: usize) -> Self {
        Self {
            laplace_filter_count,
            ..self
        }
    }

    pub(crate) fn with_zi_filter_count(self, zi_filter_count: usize) -> Self {
        Self {
            zi_filter_count,
            ..self
        }
    }
}

impl NativeProgram {
    pub(crate) fn from_bytecode(
        model: impl Into<SmolStr>,
        entry_kind: EntryKind,
        program: &BytecodeProgram,
        limits: NativeLoweringLimits<'_>,
    ) -> JitResult<Self> {
        let model = model.into();
        let mut ops = Vec::with_capacity(program.instructions.len());
        let mut current_pair_dependencies = Vec::new();
        let mut depth = 0usize;
        let mut max_stack_depth = 0usize;

        for instruction in &program.instructions {
            validate_entry_instruction(model.clone(), entry_kind, instruction)?;
            match instruction {
                Instruction::PushConst(value) => {
                    ops.push(NativeOp::Const(*value));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushParam(index) => {
                    validate_index(
                        model.clone(),
                        "PushParam parameter",
                        *index,
                        limits.parameter_count,
                    )?;
                    ops.push(NativeOp::LoadParam(*index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushParamGiven(index) => {
                    validate_index(
                        model.clone(),
                        "PushParamGiven parameter",
                        *index,
                        limits.parameter_count,
                    )?;
                    ops.push(NativeOp::LoadParamGiven(*index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushPortConnected(index) => {
                    validate_index(
                        model.clone(),
                        "PushPortConnected terminal",
                        *index,
                        limits.terminal_count,
                    )?;
                    ops.push(NativeOp::LoadPortConnected(*index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushVoltage(pos, neg) => {
                    ops.push(NativeOp::LoadVoltage {
                        pos: lower_voltage_node(
                            model.clone(),
                            *pos,
                            limits.terminal_count,
                            limits.internal_node_count,
                        )?,
                        neg: lower_voltage_node(
                            model.clone(),
                            *neg,
                            limits.terminal_count,
                            limits.internal_node_count,
                        )?,
                    });
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushInternalVoltage(index) => {
                    validate_index(
                        model.clone(),
                        "PushInternalVoltage internal node",
                        *index,
                        limits.internal_node_count,
                    )?;
                    ops.push(NativeOp::LoadInternalVoltage(*index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushVariable(index) => {
                    validate_index(
                        model.clone(),
                        "PushVariable variable",
                        *index,
                        limits.variable_count,
                    )?;
                    ops.push(NativeOp::LoadVariable(*index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushVariableDyn { base, len, lower } => {
                    validate_range(
                        model.clone(),
                        "PushVariableDyn variable range",
                        *base,
                        *len,
                        limits.variable_count,
                    )?;
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::LoadVariableDyn {
                        base: *base,
                        len: *len,
                        lower: *lower,
                    });
                }
                Instruction::PushBranchCurrent(index) => {
                    validate_index(
                        model.clone(),
                        "PushBranchCurrent branch unknown",
                        *index,
                        limits.branch_unknown_count,
                    )?;
                    ops.push(NativeOp::LoadBranchUnknown(*index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushTemperature => {
                    ops.push(NativeOp::LoadTemperature);
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushVt => {
                    ops.push(NativeOp::LoadThermalVoltage);
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushTime => {
                    ops.push(NativeOp::LoadTime);
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::Analysis(analysis_id) => {
                    ops.push(NativeOp::Analysis(*analysis_id));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushMfactor => {
                    ops.push(NativeOp::LoadMfactor);
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::Add | Instruction::Sub | Instruction::Mul | Instruction::Div => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    if lower_constant_binary_arithmetic(&mut ops, instruction) {
                        continue;
                    }
                    if lower_constant_rhs_arithmetic(&mut ops, instruction) {
                        continue;
                    }
                    if lower_constant_lhs_noncommutative_arithmetic(&mut ops, instruction) {
                        continue;
                    }
                    ops.push(arithmetic_op(instruction));
                }
                Instruction::Pow | Instruction::FnPow => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    if lower_constant_binary_math(&mut ops, BinaryMathOp::Pow) {
                        continue;
                    }
                    if lower_constant_square_power(&mut ops) {
                        continue;
                    }
                    ops.push(NativeOp::BinaryMath(BinaryMathOp::Pow));
                }
                Instruction::Atan2 | Instruction::Mod => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    let op = binary_math_op(instruction);
                    if lower_constant_binary_math(&mut ops, op) {
                        continue;
                    }
                    ops.push(NativeOp::BinaryMath(op));
                }
                Instruction::Shl
                | Instruction::Shr
                | Instruction::BitAnd
                | Instruction::BitOr
                | Instruction::BitXor => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::IntegerBinary(integer_binary_op(instruction)));
                }
                Instruction::TableLookup(table_id) => {
                    validate_index(
                        model.clone(),
                        "TableLookup table",
                        *table_id,
                        limits.lookup_table_count,
                    )?;
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::TableLookup(*table_id));
                }
                Instruction::TableDerivative(table_id) => {
                    validate_index(
                        model.clone(),
                        "TableDerivative table",
                        *table_id,
                        limits.lookup_table_count,
                    )?;
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::TableDerivative(*table_id));
                }
                Instruction::LimitState(index) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        2,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::LimitState(*index));
                }
                Instruction::LaplaceState(filter_id) => {
                    validate_index(
                        model.clone(),
                        "LaplaceState filter",
                        *filter_id,
                        limits.laplace_filter_count,
                    )?;
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::LaplaceState(*filter_id));
                }
                Instruction::ZiState(filter_id) => {
                    validate_index(
                        model.clone(),
                        "ZiState filter",
                        *filter_id,
                        limits.zi_filter_count,
                    )?;
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::ZiState(*filter_id));
                }
                Instruction::TimerState(timer_id) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        2,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::TimerState(*timer_id));
                }
                Instruction::TransitionState(filter_id) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        4,
                    )?;
                    depth -= 3;
                    ops.push(NativeOp::TransitionState(*filter_id));
                }
                Instruction::SlewState(filter_id) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        3,
                    )?;
                    depth -= 2;
                    ops.push(NativeOp::SlewState(*filter_id));
                }
                Instruction::AbsDelayState(buffer_id) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        2,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::AbsDelayState(*buffer_id));
                }
                Instruction::CrossState(detector_id) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        2,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::CrossState(*detector_id));
                }
                Instruction::WhiteNoise => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::WhiteNoise);
                }
                Instruction::FlickerNoise => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        2,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::FlickerNoise);
                }
                Instruction::Neg => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    if lower_constant_neg(&mut ops) {
                        continue;
                    }
                    ops.push(NativeOp::Neg);
                }
                Instruction::Abs => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    if lower_constant_abs(&mut ops) {
                        continue;
                    }
                    ops.push(NativeOp::Abs);
                }
                Instruction::Sqrt => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::Sqrt);
                }
                Instruction::Exp
                | Instruction::Log
                | Instruction::Log10
                | Instruction::Sin
                | Instruction::Cos
                | Instruction::Tan
                | Instruction::Sinh
                | Instruction::Cosh
                | Instruction::Tanh
                | Instruction::Limexp
                | Instruction::Asin
                | Instruction::Acos
                | Instruction::Atan
                | Instruction::Floor
                | Instruction::Ceil => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::UnaryMath(unary_math_op(instruction)));
                }
                Instruction::DdtState(index) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::DdtState(*index));
                }
                Instruction::DdtJacobian => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::DdtJacobian);
                }
                Instruction::IdtState(index) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        2,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::IdtState(*index));
                }
                Instruction::IdtJacobian => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::IdtJacobian);
                }
                Instruction::IdtModState(index) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        4,
                    )?;
                    depth -= 3;
                    ops.push(NativeOp::IdtModState(*index));
                }
                Instruction::Gt
                | Instruction::Lt
                | Instruction::Ge
                | Instruction::Le
                | Instruction::Eq
                | Instruction::Ne => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    let op = compare_op(instruction);
                    if lower_constant_binary_compare(&mut ops, op) {
                        continue;
                    }
                    if lower_constant_rhs_compare(&mut ops, op) {
                        continue;
                    }
                    if lower_constant_lhs_compare(&mut ops, op) {
                        continue;
                    }
                    ops.push(NativeOp::Compare(op));
                }
                Instruction::And | Instruction::Or => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    let op = logical_op(instruction);
                    if lower_constant_binary_logical(&mut ops, op) {
                        continue;
                    }
                    if lower_constant_rhs_logical(&mut ops, op) {
                        continue;
                    }
                    if lower_constant_lhs_logical(&mut ops, op) {
                        continue;
                    }
                    ops.push(NativeOp::Logical(op));
                }
                Instruction::Not => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::Logical(LogicalOp::Not));
                }
                Instruction::IfElse => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        3,
                    )?;
                    depth -= 2;
                    ops.push(NativeOp::IfElse);
                }
                Instruction::Min | Instruction::Max => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    if lower_constant_rhs_extremum(&mut ops, extremum_op(instruction)) {
                        continue;
                    }
                    ops.push(NativeOp::Extremum(extremum_op(instruction)));
                }
                Instruction::AboveState(_) => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    if lower_constant_rhs_compare(&mut ops, CompareOp::Gt) {
                        continue;
                    }
                    ops.push(NativeOp::Compare(CompareOp::Gt));
                }
                Instruction::PushCurrent(pos, neg) => {
                    let pair_index =
                        current_pair_index(model.clone(), *pos, *neg, limits.terminal_count)?;
                    if !limits.available_current_pairs.contains(&pair_index) {
                        return Err(JitError::unsupported_program_op(
                            model,
                            format!("PushCurrent terminal pair {pos},{neg} unavailable"),
                        ));
                    }
                    if !current_pair_dependencies.contains(&pair_index) {
                        current_pair_dependencies.push(pair_index);
                    }
                    ops.push(NativeOp::LoadCurrent(pair_index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
            }
        }

        if depth != 1 {
            return Err(stack_error(
                model,
                entry_kind,
                format!("final stack depth {depth}, expected 1"),
            ));
        }
        let optimized_max_stack_depth =
            compute_native_max_stack_depth(model.clone(), entry_kind, &ops)?;
        debug_assert!(optimized_max_stack_depth <= max_stack_depth);

        Ok(Self {
            ops,
            max_stack_depth: optimized_max_stack_depth,
            current_pair_dependencies,
        })
    }

    pub(crate) fn ops(&self) -> &[NativeOp] {
        &self.ops
    }

    pub(crate) fn max_stack_depth(&self) -> usize {
        self.max_stack_depth
    }

    pub(crate) fn current_pair_dependencies(&self) -> &[usize] {
        &self.current_pair_dependencies
    }
}

fn push_stack(depth: &mut usize, max_stack_depth: &mut usize) {
    *depth += 1;
    *max_stack_depth = (*max_stack_depth).max(*depth);
}

fn validate_entry_instruction(
    model: SmolStr,
    entry_kind: EntryKind,
    instruction: &Instruction,
) -> JitResult<()> {
    if matches!(entry_kind, EntryKind::StaticCondition)
        && !is_static_condition_instruction(instruction)
    {
        return Err(JitError::unsupported_program_op(
            model,
            format!("StaticCondition {}", instruction_name(instruction)),
        ));
    }

    if matches!(entry_kind, EntryKind::ParameterDefault)
        && !is_parameter_default_instruction(instruction)
    {
        return Err(JitError::unsupported_program_op(
            model,
            format!("ParameterDefault {}", instruction_name(instruction)),
        ));
    }

    Ok(())
}

fn is_parameter_default_instruction(instruction: &Instruction) -> bool {
    matches!(
        instruction,
        Instruction::PushConst(_)
            | Instruction::PushParam(_)
            | Instruction::PushParamGiven(_)
            | Instruction::Add
            | Instruction::Sub
            | Instruction::Mul
            | Instruction::Div
            | Instruction::Pow
            | Instruction::FnPow
            | Instruction::Atan2
            | Instruction::Mod
            | Instruction::Shl
            | Instruction::Shr
            | Instruction::BitAnd
            | Instruction::BitOr
            | Instruction::BitXor
            | Instruction::Neg
            | Instruction::Abs
            | Instruction::Sqrt
            | Instruction::Gt
            | Instruction::Lt
            | Instruction::Ge
            | Instruction::Le
            | Instruction::Eq
            | Instruction::Ne
            | Instruction::And
            | Instruction::Or
            | Instruction::Not
            | Instruction::IfElse
            | Instruction::Min
            | Instruction::Max
            | Instruction::Exp
            | Instruction::Log
            | Instruction::Log10
            | Instruction::Sin
            | Instruction::Cos
            | Instruction::Tan
            | Instruction::Sinh
            | Instruction::Cosh
            | Instruction::Tanh
            | Instruction::Limexp
            | Instruction::Asin
            | Instruction::Acos
            | Instruction::Atan
            | Instruction::Floor
            | Instruction::Ceil
    )
}

fn is_static_condition_instruction(instruction: &Instruction) -> bool {
    matches!(
        instruction,
        Instruction::PushConst(_)
            | Instruction::PushParam(_)
            | Instruction::PushParamGiven(_)
            | Instruction::PushPortConnected(_)
            | Instruction::PushVariable(_)
            | Instruction::PushVariableDyn { .. }
            | Instruction::PushTemperature
            | Instruction::PushVt
            | Instruction::PushMfactor
            | Instruction::Add
            | Instruction::Sub
            | Instruction::Mul
            | Instruction::Div
            | Instruction::Pow
            | Instruction::Mod
            | Instruction::Shl
            | Instruction::Shr
            | Instruction::BitAnd
            | Instruction::BitOr
            | Instruction::BitXor
            | Instruction::Neg
            | Instruction::Abs
            | Instruction::Sqrt
            | Instruction::Exp
            | Instruction::Log
            | Instruction::Log10
            | Instruction::Sin
            | Instruction::Cos
            | Instruction::Tan
            | Instruction::Sinh
            | Instruction::Cosh
            | Instruction::Tanh
            | Instruction::Min
            | Instruction::Max
            | Instruction::Limexp
            | Instruction::Asin
            | Instruction::Acos
            | Instruction::Atan
            | Instruction::Atan2
            | Instruction::Floor
            | Instruction::Ceil
            | Instruction::FnPow
            | Instruction::Gt
            | Instruction::Lt
            | Instruction::Ge
            | Instruction::Le
            | Instruction::Eq
            | Instruction::Ne
            | Instruction::And
            | Instruction::Or
            | Instruction::Not
            | Instruction::IfElse
    )
}

fn pop_binary_stack(
    model: SmolStr,
    entry_kind: EntryKind,
    op: &'static str,
    depth: usize,
) -> JitResult<()> {
    require_stack(model, entry_kind, op, depth, 2)
}

fn require_stack(
    model: SmolStr,
    entry_kind: EntryKind,
    op: &'static str,
    depth: usize,
    required: usize,
) -> JitResult<()> {
    if depth < required {
        return Err(stack_error(
            model,
            entry_kind,
            format!("{op} requires stack depth {required}, found {depth}"),
        ));
    }

    Ok(())
}

fn validate_index(model: SmolStr, op: &'static str, index: usize, len: usize) -> JitResult<()> {
    if index >= len {
        return Err(JitError::unsupported_program_op(
            model,
            format!("{op} {index}"),
        ));
    }

    Ok(())
}

fn validate_range(
    model: SmolStr,
    op: &'static str,
    base: usize,
    len: usize,
    limit: usize,
) -> JitResult<()> {
    let Some(end) = base.checked_add(len) else {
        return Err(JitError::unsupported_program_op(
            model,
            format!("{op} base {base} length {len} overflows"),
        ));
    };
    if len == 0 || end > limit {
        return Err(JitError::unsupported_program_op(
            model,
            format!("{op} {base}..{end} outside storage length {limit}"),
        ));
    }

    Ok(())
}

fn lower_constant_square_power(ops: &mut Vec<NativeOp>) -> bool {
    let Some(last) = ops.last_mut() else {
        return false;
    };
    if matches!(*last, NativeOp::Const(value) if value.to_bits() == 2.0_f64.to_bits()) {
        *last = NativeOp::Square;
        true
    } else {
        false
    }
}

fn lower_constant_binary_math(ops: &mut Vec<NativeOp>, op: BinaryMathOp) -> bool {
    if ops.len() < 2 {
        return false;
    }

    let lhs_index = ops.len() - 2;
    let rhs_index = ops.len() - 1;
    let (NativeOp::Const(left), NativeOp::Const(right)) = (ops[lhs_index], ops[rhs_index]) else {
        return false;
    };
    ops.truncate(lhs_index);
    ops.push(NativeOp::Const(constant_binary_math(op, left, right)));
    true
}

fn lower_constant_neg(ops: &mut Vec<NativeOp>) -> bool {
    let Some(NativeOp::Const(value)) = ops.last_mut() else {
        return false;
    };
    *value = f64::from_bits(value.to_bits() ^ 0x8000_0000_0000_0000);
    true
}

fn lower_constant_abs(ops: &mut Vec<NativeOp>) -> bool {
    let Some(NativeOp::Const(value)) = ops.last_mut() else {
        return false;
    };
    *value = f64::from_bits(value.to_bits() & 0x7fff_ffff_ffff_ffff);
    true
}

fn lower_constant_rhs_arithmetic(ops: &mut Vec<NativeOp>, instruction: &Instruction) -> bool {
    let Some(NativeOp::Const(value)) = ops.last().copied() else {
        return false;
    };
    let op = match instruction {
        Instruction::Add => NativeOp::AddConst(value),
        Instruction::Sub => NativeOp::SubConst(value),
        Instruction::Mul => NativeOp::MulConst(value),
        Instruction::Div => NativeOp::DivConst(value),
        _ => return false,
    };
    ops.pop();
    ops.push(op);
    true
}

fn lower_constant_binary_arithmetic(ops: &mut Vec<NativeOp>, instruction: &Instruction) -> bool {
    if ops.len() < 2 {
        return false;
    }

    let lhs_index = ops.len() - 2;
    let rhs_index = ops.len() - 1;
    let (NativeOp::Const(left), NativeOp::Const(right)) = (ops[lhs_index], ops[rhs_index]) else {
        return false;
    };
    let value = match instruction {
        Instruction::Add => left + right,
        Instruction::Sub => left - right,
        Instruction::Mul => left * right,
        Instruction::Div => left / right,
        _ => return false,
    };
    ops.truncate(lhs_index);
    ops.push(NativeOp::Const(value));
    true
}

fn lower_constant_lhs_noncommutative_arithmetic(
    ops: &mut Vec<NativeOp>,
    instruction: &Instruction,
) -> bool {
    if ops.len() < 2 {
        return false;
    }
    let rhs_index = ops.len() - 1;
    if !is_independent_value_push(&ops[rhs_index]) {
        return false;
    }

    let lhs_index = ops.len() - 2;
    let NativeOp::Const(value) = ops[lhs_index] else {
        return false;
    };
    let op = match instruction {
        Instruction::Sub => NativeOp::SubFromConst(value),
        Instruction::Div => NativeOp::DivFromConst(value),
        _ => return false,
    };
    ops.remove(lhs_index);
    ops.push(op);
    true
}

fn lower_constant_binary_compare(ops: &mut Vec<NativeOp>, op: CompareOp) -> bool {
    if ops.len() < 2 {
        return false;
    }

    let lhs_index = ops.len() - 2;
    let rhs_index = ops.len() - 1;
    let (NativeOp::Const(left), NativeOp::Const(right)) = (ops[lhs_index], ops[rhs_index]) else {
        return false;
    };
    let value = if constant_compare(op, left, right) {
        1.0
    } else {
        0.0
    };
    ops.truncate(lhs_index);
    ops.push(NativeOp::Const(value));
    true
}

fn lower_constant_rhs_compare(ops: &mut Vec<NativeOp>, op: CompareOp) -> bool {
    let Some(NativeOp::Const(value)) = ops.last().copied() else {
        return false;
    };
    ops.pop();
    ops.push(NativeOp::CompareConst(op, value));
    true
}

fn lower_constant_lhs_compare(ops: &mut Vec<NativeOp>, op: CompareOp) -> bool {
    if ops.len() < 2 {
        return false;
    }
    let rhs_index = ops.len() - 1;
    if !is_independent_value_push(&ops[rhs_index]) {
        return false;
    }

    let lhs_index = ops.len() - 2;
    let NativeOp::Const(value) = ops[lhs_index] else {
        return false;
    };
    ops.remove(lhs_index);
    ops.push(NativeOp::CompareConst(flip_compare_op(op), value));
    true
}

fn lower_constant_binary_logical(ops: &mut Vec<NativeOp>, op: LogicalOp) -> bool {
    if ops.len() < 2 {
        return false;
    }

    let lhs_index = ops.len() - 2;
    let rhs_index = ops.len() - 1;
    let (NativeOp::Const(left), NativeOp::Const(right)) = (ops[lhs_index], ops[rhs_index]) else {
        return false;
    };
    let value = if constant_logical(op, left, right) {
        1.0
    } else {
        0.0
    };
    ops.truncate(lhs_index);
    ops.push(NativeOp::Const(value));
    true
}

fn lower_constant_rhs_extremum(ops: &mut Vec<NativeOp>, op: ExtremumOp) -> bool {
    let Some(NativeOp::Const(value)) = ops.last().copied() else {
        return false;
    };
    ops.pop();
    ops.push(NativeOp::ExtremumConst(op, value));
    true
}

fn lower_constant_rhs_logical(ops: &mut Vec<NativeOp>, op: LogicalOp) -> bool {
    let Some(NativeOp::Const(value)) = ops.last().copied() else {
        return false;
    };
    ops.pop();
    ops.push(NativeOp::LogicalConst(op, constant_truthy(value)));
    true
}

fn lower_constant_lhs_logical(ops: &mut Vec<NativeOp>, op: LogicalOp) -> bool {
    if ops.len() < 2 {
        return false;
    }
    let rhs_index = ops.len() - 1;
    if !is_independent_value_push(&ops[rhs_index]) {
        return false;
    }

    let lhs_index = ops.len() - 2;
    let NativeOp::Const(value) = ops[lhs_index] else {
        return false;
    };
    ops.remove(lhs_index);
    ops.push(NativeOp::LogicalConst(op, constant_truthy(value)));
    true
}

fn constant_truthy(value: f64) -> bool {
    value.abs() > LOGICAL_EPSILON
}

fn constant_compare(op: CompareOp, left: f64, right: f64) -> bool {
    match op {
        CompareOp::Gt => left > right,
        CompareOp::Lt => left < right,
        CompareOp::Ge => left >= right,
        CompareOp::Le => left <= right,
        CompareOp::Eq => (left - right).abs() < LOGICAL_EPSILON,
        CompareOp::Ne => (left - right).abs() >= LOGICAL_EPSILON,
    }
}

fn constant_logical(op: LogicalOp, left: f64, right: f64) -> bool {
    match op {
        LogicalOp::And => constant_truthy(left) && constant_truthy(right),
        LogicalOp::Or => constant_truthy(left) || constant_truthy(right),
        LogicalOp::Not => unreachable!("binary logical lowering only accepts and/or"),
    }
}

fn constant_binary_math(op: BinaryMathOp, left: f64, right: f64) -> f64 {
    match op {
        BinaryMathOp::Pow => left.powf(right),
        BinaryMathOp::Atan2 => left.atan2(right),
        BinaryMathOp::Mod => left % right,
    }
}

fn is_independent_value_push(op: &NativeOp) -> bool {
    !matches!(op, NativeOp::Const(_)) && native_op_stack_effect(op) == (0, 1)
}

fn compute_native_max_stack_depth(
    model: SmolStr,
    entry_kind: EntryKind,
    ops: &[NativeOp],
) -> JitResult<usize> {
    let mut depth = 0usize;
    let mut max_stack_depth = 0usize;

    for op in ops {
        let (pops, pushes) = native_op_stack_effect(op);
        if depth < pops {
            return Err(stack_error(
                model.clone(),
                entry_kind,
                format!("optimized native op {op:?} requires stack depth {pops}, found {depth}"),
            ));
        }
        depth = depth - pops + pushes;
        max_stack_depth = max_stack_depth.max(depth);
    }

    if depth != 1 {
        return Err(stack_error(
            model,
            entry_kind,
            format!("optimized native stack depth {depth}, expected 1"),
        ));
    }

    Ok(max_stack_depth)
}

fn native_op_stack_effect(op: &NativeOp) -> (usize, usize) {
    match op {
        NativeOp::Const(_)
        | NativeOp::LoadParam(_)
        | NativeOp::LoadParamGiven(_)
        | NativeOp::LoadPortConnected(_)
        | NativeOp::LoadVoltage { .. }
        | NativeOp::LoadCurrent(_)
        | NativeOp::LoadInternalVoltage(_)
        | NativeOp::LoadVariable(_)
        | NativeOp::LoadBranchUnknown(_)
        | NativeOp::LoadTemperature
        | NativeOp::LoadThermalVoltage
        | NativeOp::LoadTime
        | NativeOp::Analysis(_)
        | NativeOp::LoadMfactor => (0, 1),

        NativeOp::LoadVariableDyn { .. }
        | NativeOp::AddConst(_)
        | NativeOp::SubConst(_)
        | NativeOp::MulConst(_)
        | NativeOp::DivConst(_)
        | NativeOp::SubFromConst(_)
        | NativeOp::DivFromConst(_)
        | NativeOp::CompareConst(_, _)
        | NativeOp::ExtremumConst(_, _)
        | NativeOp::LogicalConst(_, _)
        | NativeOp::Neg
        | NativeOp::Abs
        | NativeOp::Square
        | NativeOp::Sqrt
        | NativeOp::Logical(LogicalOp::Not)
        | NativeOp::UnaryMath(_)
        | NativeOp::TableLookup(_)
        | NativeOp::TableDerivative(_)
        | NativeOp::LaplaceState(_)
        | NativeOp::ZiState(_)
        | NativeOp::WhiteNoise
        | NativeOp::DdtState(_)
        | NativeOp::DdtJacobian
        | NativeOp::IdtJacobian => (1, 1),

        NativeOp::Add
        | NativeOp::Sub
        | NativeOp::Mul
        | NativeOp::Div
        | NativeOp::Compare(_)
        | NativeOp::Logical(LogicalOp::And | LogicalOp::Or)
        | NativeOp::Extremum(_)
        | NativeOp::BinaryMath(_)
        | NativeOp::IntegerBinary(_)
        | NativeOp::LimitState(_)
        | NativeOp::TimerState(_)
        | NativeOp::AbsDelayState(_)
        | NativeOp::CrossState(_)
        | NativeOp::FlickerNoise
        | NativeOp::IdtState(_) => (2, 1),

        NativeOp::SlewState(_) | NativeOp::IfElse => (3, 1),
        NativeOp::TransitionState(_) | NativeOp::IdtModState(_) => (4, 1),
    }
}

fn stack_error(model: SmolStr, entry_kind: EntryKind, detail: String) -> JitError {
    JitError::InvalidCanonicalIr {
        model,
        detail: format!("{entry_kind:?} expression stack invalid: {detail}").into(),
    }
}

fn lower_voltage_node(
    model: SmolStr,
    node: usize,
    terminal_count: usize,
    internal_node_count: usize,
) -> JitResult<VoltageNode> {
    if node == usize::MAX {
        return Ok(VoltageNode::Ground);
    }

    if node < terminal_count {
        return Ok(VoltageNode::Terminal(node));
    }

    let internal_index = node - terminal_count;
    if internal_index < internal_node_count {
        return Ok(VoltageNode::Internal(internal_index));
    }

    Err(JitError::unsupported_program_op(
        model,
        format!("PushVoltage unified node {node}"),
    ))
}

fn current_pair_index(
    model: SmolStr,
    pos: usize,
    neg: usize,
    terminal_count: usize,
) -> JitResult<usize> {
    if pos >= terminal_count || neg >= terminal_count {
        return Err(JitError::unsupported_program_op(
            model,
            format!("PushCurrent terminal pair {pos},{neg}"),
        ));
    }

    pos.checked_mul(terminal_count)
        .and_then(|base| base.checked_add(neg))
        .ok_or_else(|| JitError::InvalidCanonicalIr {
            model,
            detail: format!("PushCurrent terminal pair {pos},{neg} index overflow").into(),
        })
}

fn instruction_name(instruction: &Instruction) -> &'static str {
    match instruction {
        Instruction::PushConst(_) => "PushConst",
        Instruction::PushParam(_) => "PushParam",
        Instruction::PushParamGiven(_) => "PushParamGiven",
        Instruction::PushBranchCurrent(_) => "PushBranchCurrent",
        Instruction::PushVoltage(_, _) => "PushVoltage",
        Instruction::PushCurrent(_, _) => "PushCurrent",
        Instruction::PushInternalVoltage(_) => "PushInternalVoltage",
        Instruction::PushVariable(_) => "PushVariable",
        Instruction::PushVariableDyn { .. } => "PushVariableDyn",
        Instruction::PushTemperature => "PushTemperature",
        Instruction::PushVt => "PushVt",
        Instruction::PushTime => "PushTime",
        Instruction::PushMfactor => "PushMfactor",
        Instruction::PushPortConnected(_) => "PushPortConnected",
        Instruction::ZiState(_) => "ZiState",
        Instruction::Add => "Add",
        Instruction::Sub => "Sub",
        Instruction::Mul => "Mul",
        Instruction::Div => "Div",
        Instruction::Pow => "Pow",
        Instruction::Mod => "Mod",
        Instruction::Shl => "Shl",
        Instruction::Shr => "Shr",
        Instruction::BitAnd => "BitAnd",
        Instruction::BitOr => "BitOr",
        Instruction::BitXor => "BitXor",
        Instruction::Neg => "Neg",
        Instruction::Abs => "Abs",
        Instruction::Sqrt => "Sqrt",
        Instruction::Exp => "Exp",
        Instruction::Log => "Log",
        Instruction::Log10 => "Log10",
        Instruction::Sin => "Sin",
        Instruction::Cos => "Cos",
        Instruction::Tan => "Tan",
        Instruction::Sinh => "Sinh",
        Instruction::Cosh => "Cosh",
        Instruction::Tanh => "Tanh",
        Instruction::Min => "Min",
        Instruction::Max => "Max",
        Instruction::Limexp => "Limexp",
        Instruction::Asin => "Asin",
        Instruction::Acos => "Acos",
        Instruction::Atan => "Atan",
        Instruction::Atan2 => "Atan2",
        Instruction::Floor => "Floor",
        Instruction::Ceil => "Ceil",
        Instruction::FnPow => "FnPow",
        Instruction::Gt => "Gt",
        Instruction::Lt => "Lt",
        Instruction::Ge => "Ge",
        Instruction::Le => "Le",
        Instruction::Eq => "Eq",
        Instruction::Ne => "Ne",
        Instruction::And => "And",
        Instruction::Or => "Or",
        Instruction::Not => "Not",
        Instruction::DdtState(_) => "DdtState",
        Instruction::IdtState(_) => "IdtState",
        Instruction::IdtModState(_) => "IdtModState",
        Instruction::DdtJacobian => "DdtJacobian",
        Instruction::IdtJacobian => "IdtJacobian",
        Instruction::TableDerivative(_) => "TableDerivative",
        Instruction::LimitState(_) => "LimitState",
        Instruction::TableLookup(_) => "TableLookup",
        Instruction::AbsDelayState(_) => "AbsDelayState",
        Instruction::TransitionState(_) => "TransitionState",
        Instruction::SlewState(_) => "SlewState",
        Instruction::CrossState(_) => "CrossState",
        Instruction::WhiteNoise => "WhiteNoise",
        Instruction::FlickerNoise => "FlickerNoise",
        Instruction::Analysis(_) => "Analysis",
        Instruction::AboveState(_) => "AboveState",
        Instruction::TimerState(_) => "TimerState",
        Instruction::LaplaceState(_) => "LaplaceState",
        Instruction::IfElse => "IfElse",
    }
}

fn compare_op(instruction: &Instruction) -> CompareOp {
    match instruction {
        Instruction::Gt => CompareOp::Gt,
        Instruction::Lt => CompareOp::Lt,
        Instruction::Ge => CompareOp::Ge,
        Instruction::Le => CompareOp::Le,
        Instruction::Eq => CompareOp::Eq,
        Instruction::Ne => CompareOp::Ne,
        _ => unreachable!("comparison lowering only accepts ordered comparison instructions"),
    }
}

fn flip_compare_op(op: CompareOp) -> CompareOp {
    match op {
        CompareOp::Gt => CompareOp::Lt,
        CompareOp::Lt => CompareOp::Gt,
        CompareOp::Ge => CompareOp::Le,
        CompareOp::Le => CompareOp::Ge,
        CompareOp::Eq => CompareOp::Eq,
        CompareOp::Ne => CompareOp::Ne,
    }
}

fn logical_op(instruction: &Instruction) -> LogicalOp {
    match instruction {
        Instruction::And => LogicalOp::And,
        Instruction::Or => LogicalOp::Or,
        _ => unreachable!("logical lowering only accepts binary logical instructions"),
    }
}

fn extremum_op(instruction: &Instruction) -> ExtremumOp {
    match instruction {
        Instruction::Min => ExtremumOp::Min,
        Instruction::Max => ExtremumOp::Max,
        _ => unreachable!("extremum lowering only accepts min/max instructions"),
    }
}

fn unary_math_op(instruction: &Instruction) -> UnaryMathOp {
    match instruction {
        Instruction::Exp => UnaryMathOp::Exp,
        Instruction::Log => UnaryMathOp::Log,
        Instruction::Log10 => UnaryMathOp::Log10,
        Instruction::Sin => UnaryMathOp::Sin,
        Instruction::Cos => UnaryMathOp::Cos,
        Instruction::Tan => UnaryMathOp::Tan,
        Instruction::Sinh => UnaryMathOp::Sinh,
        Instruction::Cosh => UnaryMathOp::Cosh,
        Instruction::Tanh => UnaryMathOp::Tanh,
        Instruction::Limexp => UnaryMathOp::Limexp,
        Instruction::Asin => UnaryMathOp::Asin,
        Instruction::Acos => UnaryMathOp::Acos,
        Instruction::Atan => UnaryMathOp::Atan,
        Instruction::Floor => UnaryMathOp::Floor,
        Instruction::Ceil => UnaryMathOp::Ceil,
        _ => unreachable!("unary math lowering only accepts supported unary math instructions"),
    }
}

fn binary_math_op(instruction: &Instruction) -> BinaryMathOp {
    match instruction {
        Instruction::Pow | Instruction::FnPow => BinaryMathOp::Pow,
        Instruction::Atan2 => BinaryMathOp::Atan2,
        Instruction::Mod => BinaryMathOp::Mod,
        _ => unreachable!("binary math lowering only accepts supported binary math instructions"),
    }
}

fn arithmetic_op(instruction: &Instruction) -> NativeOp {
    match instruction {
        Instruction::Add => NativeOp::Add,
        Instruction::Sub => NativeOp::Sub,
        Instruction::Mul => NativeOp::Mul,
        Instruction::Div => NativeOp::Div,
        _ => unreachable!("arithmetic lowering only accepts add/sub/mul/div instructions"),
    }
}

fn integer_binary_op(instruction: &Instruction) -> IntegerBinaryOp {
    match instruction {
        Instruction::Shl => IntegerBinaryOp::Shl,
        Instruction::Shr => IntegerBinaryOp::Shr,
        Instruction::BitAnd => IntegerBinaryOp::BitAnd,
        Instruction::BitOr => IntegerBinaryOp::BitOr,
        Instruction::BitXor => IntegerBinaryOp::BitXor,
        _ => unreachable!("integer binary lowering only accepts supported integer instructions"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limits(terminal_count: usize, internal_node_count: usize) -> NativeLoweringLimits<'static> {
        NativeLoweringLimits::new(terminal_count, internal_node_count, 8, 8, 8)
            .with_lookup_table_count(8)
    }

    #[test]
    fn lowers_supported_stack_program_to_native_expr_ops() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushVoltage(0, 1),
                Instruction::PushParam(0),
                Instruction::Div,
                Instruction::PushConst(2.0),
                Instruction::Mul,
            ],
        };

        let lowered =
            NativeProgram::from_bytecode("res", EntryKind::StampValue, &program, limits(2, 0))
                .expect("lower supported program");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadVoltage {
                    pos: VoltageNode::Terminal(0),
                    neg: VoltageNode::Terminal(1),
                },
                NativeOp::LoadParam(0),
                NativeOp::Div,
                NativeOp::MulConst(2.0),
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 2);
    }

    #[test]
    fn lowers_constant_rhs_arithmetic_without_extra_stack_slot() {
        let cases = [
            (Instruction::Add, NativeOp::AddConst(4.0)),
            (Instruction::Sub, NativeOp::SubConst(4.0)),
            (Instruction::Mul, NativeOp::MulConst(4.0)),
            (Instruction::Div, NativeOp::DivConst(4.0)),
        ];

        for (instruction, expected) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(4.0),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-rhs",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant RHS arithmetic has a direct native lowering");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{instruction_name} should not allocate a second XMM stack slot for RHS constants"
            );
            assert_eq!(lowered.ops(), &[NativeOp::LoadTemperature, expected]);
        }
    }

    #[test]
    fn folds_constant_binary_arithmetic_to_exact_literal() {
        let cases = [
            (
                "add finite",
                Instruction::Add,
                3.25_f64,
                4.5_f64,
                (3.25_f64 + 4.5_f64).to_bits(),
            ),
            (
                "sub finite",
                Instruction::Sub,
                3.25_f64,
                4.5_f64,
                (3.25_f64 - 4.5_f64).to_bits(),
            ),
            (
                "mul signed zero",
                Instruction::Mul,
                -0.0_f64,
                4.5_f64,
                (-0.0_f64 * 4.5_f64).to_bits(),
            ),
            (
                "div negative zero",
                Instruction::Div,
                10.0_f64,
                -0.0_f64,
                (10.0_f64 / -0.0_f64).to_bits(),
            ),
            (
                "mul unordered",
                Instruction::Mul,
                f64::from_bits(0x7ff8_0000_0000_0003),
                4.5_f64,
                (f64::from_bits(0x7ff8_0000_0000_0003) * 4.5_f64).to_bits(),
            ),
        ];

        for (case, instruction, left, right, expected_bits) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-binary-arithmetic",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant binary arithmetic folds to exact literal");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{case} should not allocate an arithmetic stack slot"
            );
            match lowered.ops() {
                [NativeOp::Const(value)] => {
                    assert_eq!(value.to_bits(), expected_bits, "{case}");
                }
                ops => panic!("{case} lowered to unexpected ops: {ops:?}"),
            }
        }
    }

    #[test]
    fn lowers_constant_lhs_noncommutative_arithmetic_without_extra_stack_slot() {
        let cases = [
            (Instruction::Sub, NativeOp::SubFromConst(10.0)),
            (Instruction::Div, NativeOp::DivFromConst(10.0)),
        ];

        for (instruction, expected) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(10.0),
                    Instruction::PushTemperature,
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-lhs-arithmetic",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant LHS arithmetic has a direct native lowering");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{instruction_name} should not allocate a second XMM stack slot for LHS constants"
            );
            assert!(
                !lowered.ops().iter().any(
                    |op| matches!(op, NativeOp::Const(value) if value.to_bits() == 10.0_f64.to_bits())
                ),
                "{instruction_name} should consume the LHS literal in the arithmetic op"
            );
            assert_eq!(lowered.ops(), &[NativeOp::LoadTemperature, expected]);
        }
    }

    #[test]
    fn lowers_thermal_voltage_context_read() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushVt],
        };

        let lowered =
            NativeProgram::from_bytecode("vt", EntryKind::StampValue, &program, limits(0, 0))
                .expect("thermal voltage is a native context read");

        assert_eq!(lowered.max_stack_depth(), 1);
        assert_eq!(lowered.ops(), &[NativeOp::LoadThermalVoltage]);
        assert!(lowered.current_pair_dependencies().is_empty());
    }

    #[test]
    fn lowers_sqrt_as_native_unary_op() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushVoltage(0, 1), Instruction::Sqrt],
        };

        let lowered =
            NativeProgram::from_bytecode("sqrt", EntryKind::StampValue, &program, limits(2, 0))
                .expect("sqrt has a direct native x64 lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadVoltage {
                    pos: VoltageNode::Terminal(0),
                    neg: VoltageNode::Terminal(1),
                },
                NativeOp::Sqrt,
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_abs_as_native_unary_op() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::Abs],
        };

        let lowered =
            NativeProgram::from_bytecode("abs", EntryKind::Assignment, &program, limits(0, 0))
                .expect("abs has a direct native x64 lowering");

        assert_eq!(lowered.ops(), &[NativeOp::LoadTemperature, NativeOp::Abs]);
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn folds_constant_unary_ops_to_exact_literals() {
        let cases = [
            (
                "neg negative literal",
                Instruction::Neg,
                (-3.5_f64).to_bits(),
                3.5_f64.to_bits(),
            ),
            (
                "neg negative zero",
                Instruction::Neg,
                (-0.0_f64).to_bits(),
                0.0_f64.to_bits(),
            ),
            (
                "neg positive nan",
                Instruction::Neg,
                0x7ff8_0000_0000_0001,
                0xfff8_0000_0000_0001,
            ),
            (
                "abs negative literal",
                Instruction::Abs,
                (-3.5_f64).to_bits(),
                3.5_f64.to_bits(),
            ),
            (
                "abs negative zero",
                Instruction::Abs,
                (-0.0_f64).to_bits(),
                0.0_f64.to_bits(),
            ),
            (
                "abs negative nan",
                Instruction::Abs,
                0xfff8_0000_0000_0001,
                0x7ff8_0000_0000_0001,
            ),
        ];

        for (case, instruction, input_bits, expected_bits) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(f64::from_bits(input_bits)),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-unary",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant unary op folds to exact literal");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{case} should not allocate a scratch stack slot"
            );
            match lowered.ops() {
                [NativeOp::Const(value)] => {
                    assert_eq!(value.to_bits(), expected_bits, "{case}");
                }
                ops => panic!("{case} lowered to unexpected ops: {ops:?}"),
            }
        }
    }

    #[test]
    fn lowers_ordered_comparisons_as_native_binary_ops() {
        let cases = [
            (Instruction::Gt, CompareOp::Gt),
            (Instruction::Lt, CompareOp::Lt),
            (Instruction::Ge, CompareOp::Ge),
            (Instruction::Le, CompareOp::Le),
        ];

        for (instruction, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushVariable(0),
                    instruction,
                ],
            };

            let lowered =
                NativeProgram::from_bytecode("cmp", EntryKind::Assignment, &program, limits(0, 0))
                    .expect("ordered comparison has a direct native x64 lowering");

            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::LoadVariable(0),
                    NativeOp::Compare(expected),
                ]
            );
            assert_eq!(lowered.max_stack_depth(), 2);
        }
    }

    #[test]
    fn lowers_constant_rhs_comparisons_without_extra_stack_slot() {
        let cases = [
            (Instruction::Gt, CompareOp::Gt),
            (Instruction::Lt, CompareOp::Lt),
            (Instruction::Ge, CompareOp::Ge),
            (Instruction::Le, CompareOp::Le),
            (Instruction::Eq, CompareOp::Eq),
            (Instruction::Ne, CompareOp::Ne),
        ];

        for (instruction, expected) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(300.0),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "cmp-literal",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant RHS comparison has a direct native lowering");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{instruction_name} should not allocate a second XMM stack slot for RHS constants"
            );
            assert!(
                !lowered
                    .ops()
                    .iter()
                    .any(|op| matches!(op, NativeOp::Const(value) if value.to_bits() == 300.0_f64.to_bits())),
                "{expected:?} should consume the RHS literal in the compare op"
            );
            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::CompareConst(expected, 300.0)
                ]
            );
        }
    }

    #[test]
    fn lowers_constant_lhs_comparisons_without_extra_stack_slot() {
        let cases = [
            (Instruction::Gt, CompareOp::Lt),
            (Instruction::Lt, CompareOp::Gt),
            (Instruction::Ge, CompareOp::Le),
            (Instruction::Le, CompareOp::Ge),
            (Instruction::Eq, CompareOp::Eq),
            (Instruction::Ne, CompareOp::Ne),
        ];

        for (instruction, expected) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(300.0),
                    Instruction::PushTemperature,
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "cmp-literal-lhs",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant LHS comparison has a direct native lowering");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{instruction_name} should not allocate a second XMM stack slot for LHS constants"
            );
            assert!(
                !lowered.ops().iter().any(
                    |op| matches!(op, NativeOp::Const(value) if value.to_bits() == 300.0_f64.to_bits())
                ),
                "{expected:?} should consume the LHS literal in the compare op"
            );
            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::CompareConst(expected, 300.0)
                ]
            );
        }
    }

    #[test]
    fn folds_constant_comparisons_to_exact_literals() {
        let cases = [
            ("gt-true", Instruction::Gt, 5.0, 4.0, 1.0),
            ("gt-unordered", Instruction::Gt, f64::NAN, 4.0, 0.0),
            ("ge-equal", Instruction::Ge, 4.0, 4.0, 1.0),
            ("lt-true", Instruction::Lt, 3.0, 4.0, 1.0),
            ("le-equal", Instruction::Le, 4.0, 4.0, 1.0),
            ("le-unordered", Instruction::Le, 4.0, f64::NAN, 0.0),
            ("eq-within-epsilon", Instruction::Eq, 0.0, 0.5e-15, 1.0),
            ("eq-at-epsilon", Instruction::Eq, 0.0, 1.0e-15, 0.0),
            ("eq-unordered", Instruction::Eq, f64::NAN, 0.0, 0.0),
            ("ne-within-epsilon", Instruction::Ne, 0.0, 0.5e-15, 0.0),
            ("ne-at-epsilon", Instruction::Ne, 0.0, 1.0e-15, 1.0),
            ("ne-unordered", Instruction::Ne, 0.0, f64::NAN, 0.0),
        ];

        for (case, instruction, left, right, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-compare",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant comparison folds to a literal");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{case} should only need the folded literal"
            );
            assert_eq!(
                lowered.ops(),
                &[NativeOp::Const(expected)],
                "{case} should fold to an exact boolean literal"
            );
        }
    }

    #[test]
    fn lowers_equality_comparisons_as_native_binary_ops() {
        let cases = [
            (Instruction::Eq, CompareOp::Eq),
            (Instruction::Ne, CompareOp::Ne),
        ];

        for (instruction, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushVariable(0),
                    instruction,
                ],
            };

            let lowered =
                NativeProgram::from_bytecode("eq", EntryKind::Assignment, &program, limits(0, 0))
                    .expect("equality comparison has a direct native x64 lowering");

            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::LoadVariable(0),
                    NativeOp::Compare(expected),
                ]
            );
            assert_eq!(lowered.max_stack_depth(), 2);
        }
    }

    #[test]
    fn lowers_logical_ops_as_native_ops() {
        let cases = [
            (Instruction::And, LogicalOp::And),
            (Instruction::Or, LogicalOp::Or),
        ];

        for (instruction, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushVariable(0),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "logic",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("logical binary op has a direct native x64 lowering");

            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::LoadVariable(0),
                    NativeOp::Logical(expected),
                ]
            );
            assert_eq!(lowered.max_stack_depth(), 2);
        }

        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::Not],
        };

        let lowered =
            NativeProgram::from_bytecode("not", EntryKind::Assignment, &program, limits(0, 0))
                .expect("logical not has a direct native x64 lowering");

        assert_eq!(
            lowered.ops(),
            &[NativeOp::LoadTemperature, NativeOp::Logical(LogicalOp::Not)]
        );
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_constant_rhs_logical_ops_without_extra_stack_slot() {
        let cases = [
            (Instruction::And, 2.0e-15, LogicalOp::And, true),
            (Instruction::And, 1.0e-15, LogicalOp::And, false),
            (Instruction::And, f64::NAN, LogicalOp::And, false),
            (Instruction::Or, -2.0e-15, LogicalOp::Or, true),
            (Instruction::Or, 0.5e-15, LogicalOp::Or, false),
            (Instruction::Or, f64::NAN, LogicalOp::Or, false),
        ];

        for (instruction, rhs, expected_op, expected_truthy) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(rhs),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "logic-literal",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant RHS logical op has a direct native lowering");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{instruction_name} should not allocate a second XMM stack slot for RHS constants"
            );
            assert!(
                !lowered.ops().iter().any(
                    |op| matches!(op, NativeOp::Const(value) if value.to_bits() == rhs.to_bits())
                ),
                "{expected_op:?} should consume the RHS literal in the logical op"
            );
            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::LogicalConst(expected_op, expected_truthy)
                ]
            );
        }
    }

    #[test]
    fn lowers_constant_lhs_logical_ops_without_extra_stack_slot() {
        let cases = [
            (Instruction::And, 2.0e-15, LogicalOp::And, true),
            (Instruction::And, 1.0e-15, LogicalOp::And, false),
            (Instruction::And, f64::NAN, LogicalOp::And, false),
            (Instruction::Or, -2.0e-15, LogicalOp::Or, true),
            (Instruction::Or, 0.5e-15, LogicalOp::Or, false),
            (Instruction::Or, f64::NAN, LogicalOp::Or, false),
        ];

        for (instruction, lhs, expected_op, expected_truthy) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(lhs),
                    Instruction::PushTemperature,
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "logic-literal-lhs",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant LHS logical op has a direct native lowering");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{instruction_name} should not allocate a second XMM stack slot for LHS constants"
            );
            assert!(
                !lowered.ops().iter().any(
                    |op| matches!(op, NativeOp::Const(value) if value.to_bits() == lhs.to_bits())
                ),
                "{expected_op:?} should consume the LHS literal in the logical op"
            );
            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::LogicalConst(expected_op, expected_truthy)
                ]
            );
        }
    }

    #[test]
    fn folds_constant_logical_ops_to_exact_literals() {
        let cases = [
            ("and-both-true", Instruction::And, 2.0e-15, -2.0e-15, 1.0),
            (
                "and-left-at-epsilon",
                Instruction::And,
                1.0e-15,
                2.0e-15,
                0.0,
            ),
            (
                "and-right-unordered",
                Instruction::And,
                2.0e-15,
                f64::NAN,
                0.0,
            ),
            ("or-right-true", Instruction::Or, 0.5e-15, -2.0e-15, 1.0),
            ("or-both-false", Instruction::Or, 1.0e-15, 0.5e-15, 0.0),
            ("or-left-unordered", Instruction::Or, f64::NAN, 0.5e-15, 0.0),
        ];

        for (case, instruction, left, right, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-logic",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant logical op folds to a literal");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{case} should only need the folded literal"
            );
            assert_eq!(
                lowered.ops(),
                &[NativeOp::Const(expected)],
                "{case} should fold to an exact boolean literal"
            );
        }
    }

    #[test]
    fn lowers_ifelse_as_native_stack_select_op() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(7.0),
                Instruction::PushConst(3.0),
                Instruction::IfElse,
            ],
        };

        let lowered =
            NativeProgram::from_bytecode("ifelse", EntryKind::Assignment, &program, limits(0, 0))
                .expect("ifelse has a direct native x64 lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadTemperature,
                NativeOp::Const(7.0),
                NativeOp::Const(3.0),
                NativeOp::IfElse,
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 3);
    }

    #[test]
    fn lowers_min_max_as_native_binary_ops() {
        let cases = [
            (Instruction::Min, ExtremumOp::Min),
            (Instruction::Max, ExtremumOp::Max),
        ];

        for (instruction, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushVariable(0),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "minmax",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("min/max have direct native x64 lowering");

            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::LoadVariable(0),
                    NativeOp::Extremum(expected),
                ]
            );
            assert_eq!(lowered.max_stack_depth(), 2);
        }
    }

    #[test]
    fn lowers_constant_rhs_min_max_without_extra_stack_slot() {
        let cases = [
            (Instruction::Min, ExtremumOp::Min),
            (Instruction::Max, ExtremumOp::Max),
        ];

        for (instruction, expected) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(300.0),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "minmax-literal",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant RHS min/max has a direct native lowering");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{instruction_name} should not allocate a second XMM stack slot for RHS constants"
            );
            assert!(
                !lowered
                    .ops()
                    .iter()
                    .any(|op| matches!(op, NativeOp::Const(value) if value.to_bits() == 300.0_f64.to_bits())),
                "{expected:?} should consume the RHS literal in the min/max op"
            );
            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::ExtremumConst(expected, 300.0)
                ]
            );
        }
    }

    #[test]
    fn lowers_transcendental_functions_as_native_unary_math_ops() {
        let cases = [
            (Instruction::Exp, UnaryMathOp::Exp),
            (Instruction::Log, UnaryMathOp::Log),
            (Instruction::Log10, UnaryMathOp::Log10),
            (Instruction::Sin, UnaryMathOp::Sin),
            (Instruction::Cos, UnaryMathOp::Cos),
            (Instruction::Tan, UnaryMathOp::Tan),
            (Instruction::Sinh, UnaryMathOp::Sinh),
            (Instruction::Cosh, UnaryMathOp::Cosh),
            (Instruction::Tanh, UnaryMathOp::Tanh),
            (Instruction::Limexp, UnaryMathOp::Limexp),
            (Instruction::Asin, UnaryMathOp::Asin),
            (Instruction::Acos, UnaryMathOp::Acos),
            (Instruction::Atan, UnaryMathOp::Atan),
            (Instruction::Floor, UnaryMathOp::Floor),
            (Instruction::Ceil, UnaryMathOp::Ceil),
        ];

        for (instruction, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![Instruction::PushTemperature, instruction],
            };

            let lowered =
                NativeProgram::from_bytecode("math", EntryKind::Assignment, &program, limits(0, 0))
                    .expect("transcendental functions have native x64 helper-call lowering");

            assert_eq!(
                lowered.ops(),
                &[NativeOp::LoadTemperature, NativeOp::UnaryMath(expected)]
            );
            assert_eq!(lowered.max_stack_depth(), 1);
        }
    }

    #[test]
    fn lowers_binary_math_functions_as_native_binary_math_ops() {
        let cases = [
            (Instruction::Pow, 2.25, BinaryMathOp::Pow),
            (Instruction::FnPow, 2.25, BinaryMathOp::Pow),
            (Instruction::Atan2, 2.0, BinaryMathOp::Atan2),
            (Instruction::Mod, 2.0, BinaryMathOp::Mod),
        ];

        for (instruction, exponent, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(exponent),
                    instruction,
                ],
            };

            let lowered =
                NativeProgram::from_bytecode("math", EntryKind::Assignment, &program, limits(0, 0))
                    .expect("binary math functions have native x64 helper-call lowering");

            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::Const(exponent),
                    NativeOp::BinaryMath(expected),
                ]
            );
            assert_eq!(lowered.max_stack_depth(), 2);
        }
    }

    #[test]
    fn folds_constant_binary_math_to_exact_literals() {
        let cases = [
            (
                "pow-operator",
                Instruction::Pow,
                2.0,
                3.0,
                2.0_f64.powf(3.0),
            ),
            ("fn-pow", Instruction::FnPow, 4.0, 0.5, 4.0_f64.powf(0.5)),
            (
                "pow-domain-nan",
                Instruction::Pow,
                -4.0,
                0.5,
                (-4.0_f64).powf(0.5),
            ),
            ("atan2", Instruction::Atan2, 0.5, 0.25, 0.5_f64.atan2(0.25)),
            (
                "atan2-signed-zero",
                Instruction::Atan2,
                -0.0,
                -0.0,
                (-0.0_f64).atan2(-0.0),
            ),
            ("mod", Instruction::Mod, 5.25, 2.0, 5.25_f64 % 2.0),
            (
                "mod-negative-dividend",
                Instruction::Mod,
                -5.25,
                2.0,
                -5.25_f64 % 2.0,
            ),
            (
                "mod-zero-divisor",
                Instruction::Mod,
                5.25,
                0.0,
                5.25_f64 % 0.0,
            ),
        ];

        for (case, instruction, left, right, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-binary-math",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant binary math folds to a literal");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{case} should only need the folded literal"
            );
            match lowered.ops() {
                [NativeOp::Const(value)] => assert_eq!(
                    value.to_bits(),
                    expected.to_bits(),
                    "{case} should fold to the exact helper-equivalent bit pattern"
                ),
                other => panic!("{case} should fold to one literal op, got {other:?}"),
            }
        }
    }

    #[test]
    fn lowers_constant_square_power_as_native_square_op() {
        for instruction in [Instruction::Pow, Instruction::FnPow] {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(2.0),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "square",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant-square power has direct native x64 lowering");

            assert_eq!(
                lowered.ops(),
                &[NativeOp::LoadTemperature, NativeOp::Square]
            );
            assert_eq!(lowered.max_stack_depth(), 1);
        }
    }

    #[test]
    fn lowers_integer_binary_functions_as_native_integer_ops() {
        let cases = [
            (Instruction::Shl, IntegerBinaryOp::Shl),
            (Instruction::Shr, IntegerBinaryOp::Shr),
            (Instruction::BitAnd, IntegerBinaryOp::BitAnd),
            (Instruction::BitOr, IntegerBinaryOp::BitOr),
            (Instruction::BitXor, IntegerBinaryOp::BitXor),
        ];

        for (instruction, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(2.0),
                    instruction,
                ],
            };

            let lowered =
                NativeProgram::from_bytecode("bits", EntryKind::Assignment, &program, limits(0, 0))
                    .expect("integer binary ops have native x64 helper-call lowering");

            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::Const(2.0),
                    NativeOp::IntegerBinary(expected),
                ]
            );
            assert_eq!(lowered.max_stack_depth(), 2);
        }
    }

    #[test]
    fn lowers_table_model_ops_as_native_table_ops() {
        let cases = [
            (Instruction::TableLookup(2), NativeOp::TableLookup(2)),
            (
                Instruction::TableDerivative(3),
                NativeOp::TableDerivative(3),
            ),
        ];

        for (instruction, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![Instruction::PushTemperature, instruction],
            };

            let lowered = NativeProgram::from_bytecode(
                "table",
                EntryKind::Assignment,
                &program,
                limits(0, 0).with_lookup_table_count(4),
            )
            .expect("table model ops have native x64 helper-call lowering");

            assert_eq!(lowered.ops(), &[NativeOp::LoadTemperature, expected]);
            assert_eq!(lowered.max_stack_depth(), 1);
        }
    }

    #[test]
    fn lowers_limit_state_as_native_stateful_op() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(0.5),
                Instruction::LimitState(2),
            ],
        };

        let lowered =
            NativeProgram::from_bytecode("limit", EntryKind::Assignment, &program, limits(0, 0))
                .expect("limit state has native x64 lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadTemperature,
                NativeOp::Const(0.5),
                NativeOp::LimitState(2),
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 2);
    }

    #[test]
    fn lowering_rejects_limit_state_without_value_and_step() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::LimitState(0)],
        };

        let error = NativeProgram::from_bytecode(
            "bad-limit",
            EntryKind::Assignment,
            &program,
            limits(0, 0),
        )
        .expect_err("limit state requires value and step operands");
        let msg = error.to_string();
        assert!(
            msg.contains("LimitState requires stack depth 2"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowers_above_state_as_native_ordered_compare() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(300.0),
                Instruction::AboveState(7),
            ],
        };

        let lowered =
            NativeProgram::from_bytecode("above", EntryKind::Assignment, &program, limits(0, 0))
                .expect("above state has native x64 lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadTemperature,
                NativeOp::CompareConst(CompareOp::Gt, 300.0),
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_timer_state_as_native_context_event() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushConst(1.0),
                Instruction::PushConst(0.5),
                Instruction::TimerState(3),
            ],
        };

        let lowered =
            NativeProgram::from_bytecode("timer", EntryKind::Assignment, &program, limits(0, 0))
                .expect("timer state has native x64 helper-call lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::Const(1.0),
                NativeOp::Const(0.5),
                NativeOp::TimerState(3),
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 2);
    }

    #[test]
    fn lowering_rejects_timer_state_without_start_and_period() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushConst(1.0), Instruction::TimerState(0)],
        };

        let error = NativeProgram::from_bytecode(
            "bad-timer",
            EntryKind::Assignment,
            &program,
            limits(0, 0),
        )
        .expect_err("timer state requires start and period operands");
        let msg = error.to_string();
        assert!(
            msg.contains("TimerState requires stack depth 2"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_timer_state_in_restricted_entry_kinds() {
        for (entry_kind, expected) in [
            (EntryKind::StaticCondition, "StaticCondition TimerState"),
            (EntryKind::ParameterDefault, "ParameterDefault TimerState"),
        ] {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(0.5),
                    Instruction::TimerState(0),
                ],
            };

            let error =
                NativeProgram::from_bytecode("bad-timer-entry", entry_kind, &program, limits(0, 0))
                    .expect_err("timer state must stay out of restricted entries");
            let msg = error.to_string();
            assert!(msg.contains(expected), "{entry_kind:?}: got {msg}");
            assert!(
                msg.contains("no interpreter fallback"),
                "{entry_kind:?}: got {msg}"
            );
        }
    }

    #[test]
    fn lowers_transition_state_as_native_context_filter() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(0.2),
                Instruction::PushConst(0.4),
                Instruction::PushConst(0.4),
                Instruction::TransitionState(3),
            ],
        };

        let lowered = NativeProgram::from_bytecode(
            "transition",
            EntryKind::Assignment,
            &program,
            limits(0, 0),
        )
        .expect("transition state has native x64 helper-call lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadTemperature,
                NativeOp::Const(0.2),
                NativeOp::Const(0.4),
                NativeOp::Const(0.4),
                NativeOp::TransitionState(3),
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 4);
    }

    #[test]
    fn lowering_rejects_transition_state_without_all_operands() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(0.2),
                Instruction::PushConst(0.4),
                Instruction::TransitionState(0),
            ],
        };

        let error = NativeProgram::from_bytecode(
            "bad-transition",
            EntryKind::Assignment,
            &program,
            limits(0, 0),
        )
        .expect_err("transition state requires input, delay, rise, and fall operands");
        let msg = error.to_string();
        assert!(
            msg.contains("TransitionState requires stack depth 4"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_transition_state_in_restricted_entry_kinds() {
        for (entry_kind, expected) in [
            (
                EntryKind::StaticCondition,
                "StaticCondition TransitionState",
            ),
            (
                EntryKind::ParameterDefault,
                "ParameterDefault TransitionState",
            ),
        ] {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(0.2),
                    Instruction::PushConst(0.4),
                    Instruction::PushConst(0.4),
                    Instruction::TransitionState(0),
                ],
            };

            let error = NativeProgram::from_bytecode(
                "bad-transition-entry",
                entry_kind,
                &program,
                limits(0, 0),
            )
            .expect_err("transition state must stay out of restricted entries");
            let msg = error.to_string();
            assert!(msg.contains(expected), "{entry_kind:?}: got {msg}");
            assert!(
                msg.contains("no interpreter fallback"),
                "{entry_kind:?}: got {msg}"
            );
        }
    }

    #[test]
    fn lowers_slew_state_as_native_context_filter() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(2.0),
                Instruction::PushConst(3.0),
                Instruction::SlewState(4),
            ],
        };

        let lowered =
            NativeProgram::from_bytecode("slew", EntryKind::Assignment, &program, limits(0, 0))
                .expect("slew state has native x64 helper-call lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadTemperature,
                NativeOp::Const(2.0),
                NativeOp::Const(3.0),
                NativeOp::SlewState(4),
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 3);
    }

    #[test]
    fn lowering_rejects_slew_state_without_all_operands() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(2.0),
                Instruction::SlewState(0),
            ],
        };

        let error =
            NativeProgram::from_bytecode("bad-slew", EntryKind::Assignment, &program, limits(0, 0))
                .expect_err("slew state requires input, positive slew, and negative slew operands");
        let msg = error.to_string();
        assert!(
            msg.contains("SlewState requires stack depth 3"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_slew_state_in_restricted_entry_kinds() {
        for (entry_kind, expected) in [
            (EntryKind::StaticCondition, "StaticCondition SlewState"),
            (EntryKind::ParameterDefault, "ParameterDefault SlewState"),
        ] {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(3.0),
                    Instruction::SlewState(0),
                ],
            };

            let error =
                NativeProgram::from_bytecode("bad-slew-entry", entry_kind, &program, limits(0, 0))
                    .expect_err("slew state must stay out of restricted entries");
            let msg = error.to_string();
            assert!(msg.contains(expected), "{entry_kind:?}: got {msg}");
            assert!(
                msg.contains("no interpreter fallback"),
                "{entry_kind:?}: got {msg}"
            );
        }
    }

    #[test]
    fn lowers_absdelay_state_as_native_context_buffer() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(0.5),
                Instruction::AbsDelayState(5),
            ],
        };

        let lowered =
            NativeProgram::from_bytecode("absdelay", EntryKind::Assignment, &program, limits(0, 0))
                .expect("absdelay state has native x64 helper-call lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadTemperature,
                NativeOp::Const(0.5),
                NativeOp::AbsDelayState(5),
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 2);
    }

    #[test]
    fn lowering_rejects_absdelay_state_without_value_and_delay() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::AbsDelayState(0)],
        };

        let error = NativeProgram::from_bytecode(
            "bad-absdelay",
            EntryKind::Assignment,
            &program,
            limits(0, 0),
        )
        .expect_err("absdelay state requires input and delay operands");
        let msg = error.to_string();
        assert!(
            msg.contains("AbsDelayState requires stack depth 2"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_absdelay_state_in_restricted_entry_kinds() {
        for (entry_kind, expected) in [
            (EntryKind::StaticCondition, "StaticCondition AbsDelayState"),
            (
                EntryKind::ParameterDefault,
                "ParameterDefault AbsDelayState",
            ),
        ] {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(0.5),
                    Instruction::AbsDelayState(0),
                ],
            };

            let error = NativeProgram::from_bytecode(
                "bad-absdelay-entry",
                entry_kind,
                &program,
                limits(0, 0),
            )
            .expect_err("absdelay state must stay out of restricted entries");
            let msg = error.to_string();
            assert!(msg.contains(expected), "{entry_kind:?}: got {msg}");
            assert!(
                msg.contains("no interpreter fallback"),
                "{entry_kind:?}: got {msg}"
            );
        }
    }

    #[test]
    fn lowers_cross_state_as_native_context_detector() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(1.0),
                Instruction::CrossState(4),
            ],
        };

        let lowered =
            NativeProgram::from_bytecode("cross", EntryKind::Assignment, &program, limits(0, 0))
                .expect("cross state has native x64 helper-call lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadTemperature,
                NativeOp::Const(1.0),
                NativeOp::CrossState(4),
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 2);
    }

    #[test]
    fn lowering_rejects_cross_state_without_value_and_direction() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::CrossState(0)],
        };

        let error = NativeProgram::from_bytecode(
            "bad-cross",
            EntryKind::Assignment,
            &program,
            limits(0, 0),
        )
        .expect_err("cross state requires value and direction operands");
        let msg = error.to_string();
        assert!(
            msg.contains("CrossState requires stack depth 2"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_cross_state_in_restricted_entry_kinds() {
        for (entry_kind, expected) in [
            (EntryKind::StaticCondition, "StaticCondition CrossState"),
            (EntryKind::ParameterDefault, "ParameterDefault CrossState"),
        ] {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(1.0),
                    Instruction::CrossState(0),
                ],
            };

            let error =
                NativeProgram::from_bytecode("bad-cross-entry", entry_kind, &program, limits(0, 0))
                    .expect_err("cross state must stay out of restricted entries");
            let msg = error.to_string();
            assert!(msg.contains(expected), "{entry_kind:?}: got {msg}");
            assert!(
                msg.contains("no interpreter fallback"),
                "{entry_kind:?}: got {msg}"
            );
        }
    }

    #[test]
    fn lowers_laplace_state_when_filter_count_is_known() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::LaplaceState(0)],
        };

        let lowered = NativeProgram::from_bytecode(
            "laplace",
            EntryKind::StampValue,
            &program,
            limits(0, 0).with_laplace_filter_count(1),
        )
        .expect("Laplace filter state has native helper lowering");

        assert_eq!(
            lowered.ops(),
            &[NativeOp::LoadTemperature, NativeOp::LaplaceState(0)]
        );
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn lowering_rejects_laplace_state_outside_known_filters() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::LaplaceState(1)],
        };

        let error = NativeProgram::from_bytecode(
            "bad-laplace",
            EntryKind::StampValue,
            &program,
            limits(0, 0).with_laplace_filter_count(1),
        )
        .expect_err("Laplace filter id outside compiled table must fail closed");
        let msg = error.to_string();
        assert!(msg.contains("LaplaceState filter 1"), "got: {msg}");
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowers_zi_state_when_filter_count_is_known() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::ZiState(0)],
        };

        let lowered = NativeProgram::from_bytecode(
            "zi",
            EntryKind::StampValue,
            &program,
            limits(0, 0).with_zi_filter_count(1),
        )
        .expect("zi filter state has native helper lowering");

        assert_eq!(
            lowered.ops(),
            &[NativeOp::LoadTemperature, NativeOp::ZiState(0)]
        );
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn lowering_rejects_zi_state_outside_known_filters() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::ZiState(1)],
        };

        let error = NativeProgram::from_bytecode(
            "bad-zi",
            EntryKind::StampValue,
            &program,
            limits(0, 0).with_zi_filter_count(1),
        )
        .expect_err("zi filter id outside compiled table must fail closed");
        let msg = error.to_string();
        assert!(msg.contains("ZiState filter 1"), "got: {msg}");
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowers_large_signal_noise_sources_as_native_zero_ops() {
        let cases = [
            (
                "white",
                vec![Instruction::PushTemperature, Instruction::WhiteNoise],
                &[NativeOp::LoadTemperature, NativeOp::WhiteNoise][..],
                1,
            ),
            (
                "flicker",
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(1.0),
                    Instruction::FlickerNoise,
                ],
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::Const(1.0),
                    NativeOp::FlickerNoise,
                ][..],
                2,
            ),
        ];

        for (name, instructions, expected, expected_depth) in cases {
            let program = BytecodeProgram { instructions };

            let lowered = NativeProgram::from_bytecode(
                format!("noise-{name}"),
                EntryKind::StampValue,
                &program,
                limits(0, 0),
            )
            .expect("large-signal noise source has native x64 zero lowering");

            assert_eq!(lowered.ops(), expected, "{name}");
            assert_eq!(lowered.max_stack_depth(), expected_depth, "{name}");
        }
    }

    #[test]
    fn lowering_rejects_noise_sources_without_required_operands() {
        let cases = [
            (
                "WhiteNoise",
                vec![Instruction::WhiteNoise],
                "WhiteNoise requires stack depth 1",
            ),
            (
                "FlickerNoise",
                vec![Instruction::PushTemperature, Instruction::FlickerNoise],
                "FlickerNoise requires stack depth 2",
            ),
        ];

        for (name, instructions, expected) in cases {
            let program = BytecodeProgram { instructions };

            let error = NativeProgram::from_bytecode(
                "bad-noise",
                EntryKind::StampValue,
                &program,
                limits(0, 0),
            )
            .expect_err("malformed noise bytecode must fail closed");
            let msg = error.to_string();
            assert!(msg.contains(expected), "{name}: got {msg}");
            assert!(msg.contains("no interpreter fallback"), "{name}: got {msg}");
        }
    }

    #[test]
    fn lowering_rejects_table_model_ops_outside_known_tables() {
        let cases = [
            (
                "TableLookup",
                Instruction::TableLookup(1),
                "TableLookup table 1",
            ),
            (
                "TableDerivative",
                Instruction::TableDerivative(1),
                "TableDerivative table 1",
            ),
        ];

        for (name, instruction, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![Instruction::PushTemperature, instruction],
            };
            let limits = NativeLoweringLimits::new(0, 0, 0, 0, 0).with_lookup_table_count(1);

            let error =
                NativeProgram::from_bytecode("bad", EntryKind::Assignment, &program, limits)
                    .expect_err("table id outside registered lookup tables must fail closed");
            let msg = error.to_string();
            assert!(msg.contains(expected), "{name}: got {msg}");
            assert!(msg.contains("no interpreter fallback"), "{name}: got {msg}");
        }
    }

    #[test]
    fn lowers_terminal_to_ground_voltage_without_usize_sentinel() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushVoltage(0, usize::MAX)],
        };

        let lowered =
            NativeProgram::from_bytecode("res", EntryKind::StampValue, &program, limits(1, 0))
                .expect("lower terminal-to-ground voltage");

        assert_eq!(
            lowered.ops(),
            &[NativeOp::LoadVoltage {
                pos: VoltageNode::Terminal(0),
                neg: VoltageNode::Ground,
            }]
        );
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_ground_to_terminal_voltage_without_usize_sentinel() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushVoltage(usize::MAX, 0)],
        };

        let lowered =
            NativeProgram::from_bytecode("res", EntryKind::StampValue, &program, limits(1, 0))
                .expect("lower ground-to-terminal voltage");

        assert_eq!(
            lowered.ops(),
            &[NativeOp::LoadVoltage {
                pos: VoltageNode::Ground,
                neg: VoltageNode::Terminal(0),
            }]
        );
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_unified_internal_voltage_index_when_internal_count_is_known() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushVoltage(1, 2)],
        };

        let lowered =
            NativeProgram::from_bytecode("int", EntryKind::StampValue, &program, limits(2, 1))
                .expect("lower terminal-to-internal voltage");

        assert_eq!(
            lowered.ops(),
            &[NativeOp::LoadVoltage {
                pos: VoltageNode::Terminal(1),
                neg: VoltageNode::Internal(0),
            }]
        );
    }

    #[test]
    fn lowering_rejects_unified_voltage_index_outside_known_nodes() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushVoltage(3, usize::MAX)],
        };

        let error =
            NativeProgram::from_bytecode("bad", EntryKind::StampValue, &program, limits(2, 1))
                .expect_err("node outside terminals plus internals must fail closed");
        let msg = error.to_string();
        assert!(msg.contains("PushVoltage unified node 3"), "got: {msg}");
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_direct_internal_voltage_outside_known_nodes() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushInternalVoltage(1)],
        };

        let error =
            NativeProgram::from_bytecode("bad", EntryKind::StampValue, &program, limits(0, 1))
                .expect_err("direct internal voltage outside known internals must fail closed");
        let msg = error.to_string();
        assert!(
            msg.contains("PushInternalVoltage internal node 1"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowers_terminal_pair_current_probe_when_terminals_are_known() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushCurrent(0, 1)],
        };
        let available = [1];

        let lowered = NativeProgram::from_bytecode(
            "probe",
            EntryKind::StampValue,
            &program,
            limits(2, 0).with_available_current_pairs(&available),
        )
        .expect("terminal-pair current probes are native-loadable");

        assert_eq!(lowered.ops(), &[NativeOp::LoadCurrent(1)]);
        assert_eq!(lowered.max_stack_depth(), 1);
        assert_eq!(lowered.current_pair_dependencies(), &[1]);
    }

    #[test]
    fn lowering_rejects_current_probe_before_pair_is_available() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushCurrent(0, 1)],
        };

        let error =
            NativeProgram::from_bytecode("probe", EntryKind::StampValue, &program, limits(2, 0))
                .expect_err("current probes must not read unavailable terminal-pair slots");
        let msg = error.to_string();
        assert!(
            msg.contains("PushCurrent terminal pair 0,1 unavailable"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_current_probe_outside_terminal_pairs() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushCurrent(0, 2)],
        };

        let error =
            NativeProgram::from_bytecode("probe", EntryKind::StampValue, &program, limits(2, 1))
                .expect_err("current probes outside terminal pair matrix must fail closed");
        let msg = error.to_string();
        assert!(msg.contains("PushCurrent terminal pair 0,2"), "got: {msg}");
        assert!(msg.contains("native JIT"));
        assert!(msg.contains("no interpreter fallback"));
    }

    #[test]
    fn lowering_rejects_param_given_outside_known_parameters() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushParamGiven(1)],
        };
        let limits = NativeLoweringLimits::new(0, 0, 1, 0, 0);

        let error = NativeProgram::from_bytecode("bad", EntryKind::StampValue, &program, limits)
            .expect_err("parameter-given index outside known parameters must fail closed");
        let msg = error.to_string();
        assert!(msg.contains("PushParamGiven parameter 1"), "got: {msg}");
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_port_connected_outside_known_terminals() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushPortConnected(2)],
        };
        let limits = NativeLoweringLimits::new(2, 0, 0, 0, 0);

        let error = NativeProgram::from_bytecode("bad", EntryKind::StampValue, &program, limits)
            .expect_err("port-connected index outside known terminals must fail closed");
        let msg = error.to_string();
        assert!(msg.contains("PushPortConnected terminal 2"), "got: {msg}");
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_direct_indexed_loads_outside_known_storage() {
        let cases = [
            (
                "PushParam",
                Instruction::PushParam(1),
                NativeLoweringLimits::new(0, 0, 1, 0, 0),
                "PushParam parameter 1",
            ),
            (
                "PushVariable",
                Instruction::PushVariable(1),
                NativeLoweringLimits::new(0, 0, 0, 1, 0),
                "PushVariable variable 1",
            ),
            (
                "PushBranchCurrent",
                Instruction::PushBranchCurrent(1),
                NativeLoweringLimits::new(0, 0, 0, 0, 1),
                "PushBranchCurrent branch unknown 1",
            ),
        ];

        for (name, instruction, limits, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![instruction],
            };

            let error =
                NativeProgram::from_bytecode("bad", EntryKind::StampValue, &program, limits)
                    .unwrap_err();
            let msg = error.to_string();
            assert!(msg.contains(expected), "{name}: {msg}");
            assert!(msg.contains("no interpreter fallback"), "{name}: {msg}");
        }
    }

    #[test]
    fn lowers_dynamic_variable_read_when_range_is_known() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushParam(0),
                Instruction::PushVariableDyn {
                    base: 2,
                    len: 3,
                    lower: 1,
                },
            ],
        };

        let lowered = NativeProgram::from_bytecode(
            "dyn-read",
            EntryKind::StampValue,
            &program,
            NativeLoweringLimits::new(0, 0, 1, 5, 0),
        )
        .expect("dynamic variable reads inside known storage have native lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadParam(0),
                NativeOp::LoadVariableDyn {
                    base: 2,
                    len: 3,
                    lower: 1
                }
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn lowering_rejects_dynamic_variable_read_outside_known_storage() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushParam(0),
                Instruction::PushVariableDyn {
                    base: 3,
                    len: 3,
                    lower: 1,
                },
            ],
        };

        let error = NativeProgram::from_bytecode(
            "bad-dyn-read",
            EntryKind::StampValue,
            &program,
            NativeLoweringLimits::new(0, 0, 1, 5, 0),
        )
        .expect_err("dynamic variable range outside storage must fail closed");
        let msg = error.to_string();
        assert!(
            msg.contains("PushVariableDyn variable range 3..6 outside storage length 5"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_unbalanced_stack() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::Add],
        };

        let error =
            NativeProgram::from_bytecode("bad", EntryKind::StampValue, &program, limits(0, 0))
                .expect_err("binary op without operands must fail");
        assert!(error.to_string().contains("stack"));
    }
}
