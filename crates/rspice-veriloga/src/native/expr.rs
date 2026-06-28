#![allow(dead_code)]

use super::{JitError, JitResult};
use crate::codegen::{BytecodeProgram, Instruction};
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
    Ground,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum NativeOp {
    Const(f64),
    LoadParam(usize),
    LoadVoltage { pos: VoltageNode, neg: VoltageNode },
    LoadInternalVoltage(usize),
    LoadVariable(usize),
    LoadBranchUnknown(usize),
    Add,
    Sub,
    Mul,
    Div,
    Neg,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NativeProgram {
    ops: Vec<NativeOp>,
    max_stack_depth: usize,
}

impl NativeProgram {
    pub(crate) fn from_bytecode(
        model: impl Into<SmolStr>,
        entry_kind: EntryKind,
        program: &BytecodeProgram,
        terminal_count: usize,
    ) -> JitResult<Self> {
        let model = model.into();
        let mut ops = Vec::with_capacity(program.instructions.len());
        let mut depth = 0usize;
        let mut max_stack_depth = 0usize;

        for instruction in &program.instructions {
            match instruction {
                Instruction::PushConst(value) => {
                    ops.push(NativeOp::Const(*value));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushParam(index) => {
                    ops.push(NativeOp::LoadParam(*index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushVoltage(pos, neg) => {
                    ops.push(NativeOp::LoadVoltage {
                        pos: lower_voltage_node(model.clone(), *pos, terminal_count)?,
                        neg: lower_voltage_node(model.clone(), *neg, terminal_count)?,
                    });
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushInternalVoltage(index) => {
                    ops.push(NativeOp::LoadInternalVoltage(*index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushVariable(index) => {
                    ops.push(NativeOp::LoadVariable(*index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushBranchCurrent(index) => {
                    ops.push(NativeOp::LoadBranchUnknown(*index));
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
                Instruction::PushCurrent(_, _) => {
                    return Err(JitError::unsupported_program_op(
                        model,
                        instruction_name(instruction),
                    ));
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
        })
    }

    pub(crate) fn ops(&self) -> &[NativeOp] {
        &self.ops
    }

    pub(crate) fn max_stack_depth(&self) -> usize {
        self.max_stack_depth
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
) -> JitResult<VoltageNode> {
    if node == usize::MAX {
        return Ok(VoltageNode::Ground);
    }

    if node >= terminal_count {
        return Err(JitError::unsupported_program_op(
            model,
            format!("PushVoltage unified node {node}"),
        ));
    }

    Ok(VoltageNode::Terminal(node))
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

#[cfg(test)]
mod tests {
    use super::*;

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

        let lowered = NativeProgram::from_bytecode("res", EntryKind::StampValue, &program, 2)
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
    fn lowers_terminal_to_ground_voltage_without_usize_sentinel() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushVoltage(0, usize::MAX)],
        };

        let lowered = NativeProgram::from_bytecode("res", EntryKind::StampValue, &program, 1)
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

        let lowered = NativeProgram::from_bytecode("res", EntryKind::StampValue, &program, 1)
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
    fn lowering_rejects_unified_internal_voltage_index_when_terminal_count_is_known() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushVoltage(1, usize::MAX)],
        };

        let error = NativeProgram::from_bytecode("res", EntryKind::StampValue, &program, 1)
            .expect_err("unified internal node index is outside this native slice");
        let msg = error.to_string();
        assert!(msg.contains("PushVoltage"));
        assert!(msg.contains("native JIT"));
        assert!(msg.contains("no interpreter fallback"));
    }

    #[test]
    fn lowering_rejects_current_probe_without_fallback() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushCurrent(0, 1)],
        };

        let error = NativeProgram::from_bytecode("probe", EntryKind::StampValue, &program, 0)
            .expect_err("current probe is outside this slice");
        let msg = error.to_string();
        assert!(msg.contains("PushCurrent"));
        assert!(msg.contains("native JIT"));
        assert!(msg.contains("no interpreter fallback"));
    }

    #[test]
    fn lowering_rejects_unbalanced_stack() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::Add],
        };

        let error = NativeProgram::from_bytecode("bad", EntryKind::StampValue, &program, 0)
            .expect_err("binary op without operands must fail");
        assert!(error.to_string().contains("stack"));
    }
}
