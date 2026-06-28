#![allow(dead_code)]

use super::{JitError, JitResult};
use crate::codegen::{BytecodeProgram, CompiledModel, Instruction};
use smol_str::SmolStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EntryKind {
    Assignment,
    StampValue,
    Jacobian,
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
    LoadBranchUnknown(usize),
    LoadTemperature,
    LoadThermalVoltage,
    LoadTime,
    LoadMfactor,
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Abs,
    Sqrt,
    Compare(CompareOp),
    Logical(LogicalOp),
    IfElse,
    Extremum(ExtremumOp),
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
            available_current_pairs,
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
                Instruction::PushMfactor => {
                    ops.push(NativeOp::LoadMfactor);
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::Add => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::Add);
                }
                Instruction::Sub => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::Sub);
                }
                Instruction::Mul => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::Mul);
                }
                Instruction::Div => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::Div);
                }
                Instruction::Neg => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
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
                    ops.push(NativeOp::Compare(compare_op(instruction)));
                }
                Instruction::And | Instruction::Or => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::Logical(logical_op(instruction)));
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
                    ops.push(NativeOp::Extremum(extremum_op(instruction)));
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
                _ => {
                    return Err(JitError::unsupported_program_op(
                        model,
                        instruction_name(instruction),
                    ));
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

        Ok(Self {
            ops,
            max_stack_depth,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn limits(terminal_count: usize, internal_node_count: usize) -> NativeLoweringLimits<'static> {
        NativeLoweringLimits::new(terminal_count, internal_node_count, 8, 8, 8)
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
                NativeOp::Const(2.0),
                NativeOp::Mul,
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 2);
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
                    Instruction::PushConst(300.0),
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
                    NativeOp::Const(300.0),
                    NativeOp::Compare(expected),
                ]
            );
            assert_eq!(lowered.max_stack_depth(), 2);
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
                    Instruction::PushConst(300.0),
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
                    NativeOp::Const(300.0),
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
                    Instruction::PushConst(300.0),
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
                    NativeOp::Const(300.0),
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
                    Instruction::PushConst(300.0),
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
                    NativeOp::Const(300.0),
                    NativeOp::Extremum(expected),
                ]
            );
            assert_eq!(lowered.max_stack_depth(), 2);
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
