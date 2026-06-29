pub(crate) mod codegen;
pub mod encoder;

use super::expr::{
    EntryKind, NativeLoweringLimits, NativeOp, NativeProgram, constant_dynamic_variable_slot,
};
use super::model::{CodeOffset, NativeCurrentDependencies, NativeEntryOffsets, NativeModel};
use super::runtime::ExecutableMemory;
use super::{JitError, JitResult};
use crate::codegen::{AssignmentStep, CompiledModel, StampIndex, StampProgram};
use crate::native::x64::codegen::NativeAssignment;

pub(crate) fn compile_model(model: &CompiledModel) -> JitResult<NativeModel> {
    super::validate_native_coverage(model)?;
    let base_limits = NativeLoweringLimits::for_model(model);

    let mut image = Vec::new();
    let assignment = CodeOffset::new(image.len());
    append_assignment_entry(model, &mut image)?;

    let mut parameter_defaults = Vec::with_capacity(model.parameters.len());
    for parameter in &model.parameters {
        let default_entry = if let Some(program) = &parameter.default_program {
            let program = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::ParameterDefault,
                program,
                base_limits,
            )?;
            Some(append_value_entry(&mut image, &program)?)
        } else {
            None
        };
        parameter_defaults.push(default_entry);
    }

    let mut static_conditions = Vec::with_capacity(model.stamp_programs.len());
    let mut stamp_values = Vec::with_capacity(model.stamp_programs.len());
    let mut stamp_value_current_dependencies = Vec::with_capacity(model.stamp_programs.len());
    let mut jacobians = Vec::with_capacity(model.stamp_programs.len());
    let mut jacobian_current_dependencies = Vec::with_capacity(model.stamp_programs.len());
    let mut reactive_jacobians = Vec::with_capacity(model.stamp_programs.len());
    let mut reactive_jacobian_current_dependencies = Vec::with_capacity(model.stamp_programs.len());
    let mut available_current_pairs = Vec::new();

    for stamp in &model.stamp_programs {
        let static_condition = if let Some(condition) = &stamp.static_condition {
            let program = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::StaticCondition,
                condition,
                base_limits,
            )?;
            Some(append_value_entry(&mut image, &program)?)
        } else {
            None
        };
        static_conditions.push(static_condition);

        let value_limits = base_limits.with_available_current_pairs(&available_current_pairs);
        let program = NativeProgram::from_bytecode(
            model.name.clone(),
            EntryKind::StampValue,
            &stamp.value_program,
            value_limits,
        )?;
        stamp_value_current_dependencies.push(program.current_pair_dependencies().to_vec());
        stamp_values.push(append_value_entry(&mut image, &program)?);

        let mut jacobian_current_pairs = available_current_pairs.clone();
        if let Some((pos, neg)) = infer_current_terminal_pair(stamp) {
            push_current_pair_indices(
                model,
                &mut jacobian_current_pairs,
                model.num_terminals,
                pos,
                neg,
            )?;
        }
        let jacobian_limits = base_limits.with_available_current_pairs(&jacobian_current_pairs);

        let mut stamp_jacobians = Vec::with_capacity(stamp.jacobian_programs.len());
        let mut stamp_jacobian_current_dependencies =
            Vec::with_capacity(stamp.jacobian_programs.len());
        for jacobian in &stamp.jacobian_programs {
            let program = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::Jacobian,
                &jacobian.program,
                jacobian_limits,
            )?;
            stamp_jacobian_current_dependencies.push(program.current_pair_dependencies().to_vec());
            stamp_jacobians.push(append_value_entry(&mut image, &program)?);
        }
        jacobians.push(stamp_jacobians);
        jacobian_current_dependencies.push(stamp_jacobian_current_dependencies);

        let mut stamp_reactive_jacobians = Vec::with_capacity(stamp.reactive_jacobians.len());
        let mut stamp_reactive_jacobian_current_dependencies =
            Vec::with_capacity(stamp.reactive_jacobians.len());
        for reactive_jacobian in &stamp.reactive_jacobians {
            let program = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::ReactiveJacobian,
                &reactive_jacobian.program,
                base_limits,
            )?;
            stamp_reactive_jacobian_current_dependencies
                .push(program.current_pair_dependencies().to_vec());
            stamp_reactive_jacobians.push(append_value_entry(&mut image, &program)?);
        }
        reactive_jacobians.push(stamp_reactive_jacobians);
        reactive_jacobian_current_dependencies.push(stamp_reactive_jacobian_current_dependencies);

        if let Some((pos, neg)) = infer_current_terminal_pair(stamp) {
            push_current_pair_indices(
                model,
                &mut available_current_pairs,
                model.num_terminals,
                pos,
                neg,
            )?;
        }
    }

    let executable = ExecutableMemory::allocate(&image)?;
    NativeModel::from_executable_image_with_dependencies(
        model.num_variables,
        model.parameters.len(),
        executable,
        NativeEntryOffsets {
            assignment,
            parameter_defaults,
            static_conditions,
            stamp_values,
            jacobians,
            reactive_jacobians,
        },
        NativeCurrentDependencies {
            stamp_values: stamp_value_current_dependencies,
            jacobians: jacobian_current_dependencies,
            reactive_jacobians: reactive_jacobian_current_dependencies,
        },
    )
}

fn append_assignment_entry(model: &CompiledModel, image: &mut Vec<u8>) -> JitResult<()> {
    let assignments = model
        .assignment_steps
        .iter()
        .map(|step| lower_assignment_step(model, step))
        .collect::<JitResult<Vec<_>>>()?;

    let bytes = if assignments.is_empty() {
        vec![0xC3]
    } else {
        codegen::compile_assignment_pass_function(&assignments)?
    };
    image.extend_from_slice(&bytes);
    Ok(())
}

fn lower_assignment_step(
    model: &CompiledModel,
    step: &AssignmentStep,
) -> JitResult<NativeAssignment> {
    let limits = NativeLoweringLimits::for_model(model);
    match step {
        AssignmentStep::Assign(assignment) => {
            let program = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::Assignment,
                &assignment.program,
                limits,
            )?;
            Ok(NativeAssignment::Direct {
                var_index: assignment.var_index,
                program,
            })
        }
        AssignmentStep::AssignIndexed {
            base,
            len,
            lower,
            index,
            value,
        } => {
            let index = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::Assignment,
                index,
                limits,
            )?;
            let value = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::Assignment,
                value,
                limits,
            )?;
            if let Some(var_index) = constant_indexed_assignment_slot(&index, *base, *len, *lower) {
                return Ok(NativeAssignment::Direct {
                    var_index,
                    program: value,
                });
            }
            Ok(NativeAssignment::Indexed {
                base: *base,
                len: *len,
                lower: *lower,
                index,
                value,
            })
        }
        AssignmentStep::Loop { condition, body } => {
            let condition = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::Assignment,
                condition,
                limits,
            )?;
            let body = body
                .iter()
                .map(|step| lower_assignment_step(model, step))
                .collect::<JitResult<Vec<_>>>()?;
            Ok(NativeAssignment::Loop { condition, body })
        }
    }
}

fn constant_indexed_assignment_slot(
    index: &NativeProgram,
    base: usize,
    len: usize,
    lower: i64,
) -> Option<usize> {
    match index.ops() {
        [NativeOp::Const(raw_index)] => {
            constant_dynamic_variable_slot(*raw_index, base, len, lower)
        }
        _ => None,
    }
}

fn append_value_entry(image: &mut Vec<u8>, program: &NativeProgram) -> JitResult<CodeOffset> {
    let offset = CodeOffset::new(image.len());
    let bytes = codegen::compile_value_function(program)?;
    image.extend_from_slice(&bytes);
    Ok(offset)
}

fn infer_current_terminal_pair(program: &StampProgram) -> Option<(usize, usize)> {
    let mut pos_terminal = None;
    let mut neg_terminal = None;

    for loc in &program.stamp_locations {
        let terminal = match loc.row {
            StampIndex::Terminal(term) => term,
            _ => continue,
        };

        if loc.sign < 0.0 {
            if pos_terminal.replace(terminal).is_some() {
                return None;
            }
        } else if loc.sign > 0.0 && neg_terminal.replace(terminal).is_some() {
            return None;
        }
    }

    match (pos_terminal, neg_terminal) {
        (Some(pos), Some(neg)) if pos != neg => Some((pos, neg)),
        _ => None,
    }
}

fn push_current_pair_indices(
    model: &CompiledModel,
    available_current_pairs: &mut Vec<usize>,
    terminal_count: usize,
    pos: usize,
    neg: usize,
) -> JitResult<()> {
    let forward = pos
        .checked_mul(terminal_count)
        .and_then(|base| base.checked_add(neg))
        .ok_or_else(|| current_pair_overflow(model, pos, neg))?;
    if !available_current_pairs.contains(&forward) {
        available_current_pairs.push(forward);
    }

    let reverse = neg
        .checked_mul(terminal_count)
        .and_then(|base| base.checked_add(pos))
        .ok_or_else(|| current_pair_overflow(model, neg, pos))?;
    if !available_current_pairs.contains(&reverse) {
        available_current_pairs.push(reverse);
    }

    Ok(())
}

fn current_pair_overflow(model: &CompiledModel, pos: usize, neg: usize) -> JitError {
    JitError::InvalidCanonicalIr {
        model: model.name.clone(),
        detail: format!("PushCurrent terminal pair {pos},{neg} index overflow").into(),
    }
}

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::lower_assignment_step;
    use crate::codegen::{AssignmentStep, BytecodeProgram, CompiledModel, Instruction};
    use crate::native::expr::NativeOp;
    use crate::native::x64::codegen::NativeAssignment;
    use smol_str::SmolStr;

    #[test]
    fn lower_assignment_step_folds_constant_indexed_write_to_direct_assignment() {
        let model = compiled_model_with_variables(4);
        let step = indexed_assignment_step(
            1,
            3,
            1,
            vec![Instruction::PushConst(2.49)],
            vec![Instruction::PushConst(11.0)],
        );

        let assignment = lower_assignment_step(&model, &step).expect("lower indexed assignment");

        match assignment {
            NativeAssignment::Direct { var_index, program } => {
                assert_eq!(var_index, 2);
                assert_eq!(program.ops(), &[NativeOp::Const(11.0)]);
            }
            other => panic!("expected direct assignment, got {other:?}"),
        }
    }

    #[test]
    fn lower_assignment_step_preserves_unsafe_indexed_writes_on_helper_path() {
        let cases = [
            ("dynamic", vec![Instruction::PushVariable(0)], 0),
            ("nan", vec![Instruction::PushConst(f64::NAN)], 0),
            ("infinity", vec![Instruction::PushConst(f64::INFINITY)], 0),
            (
                "huge finite",
                vec![Instruction::PushConst(1.0e300)],
                i64::MAX,
            ),
            ("out of range", vec![Instruction::PushConst(2.0)], 0),
        ];

        for (name, index, lower) in cases {
            let model = compiled_model_with_variables(1);
            let step = indexed_assignment_step(
                0,
                1,
                lower,
                index.clone(),
                vec![Instruction::PushConst(11.0)],
            );

            let assignment =
                lower_assignment_step(&model, &step).expect("lower indexed assignment");

            match assignment {
                NativeAssignment::Indexed {
                    base,
                    len,
                    lower: actual_lower,
                    index: index_program,
                    value,
                } => {
                    assert_eq!(base, 0, "{name}");
                    assert_eq!(len, 1, "{name}");
                    assert_eq!(actual_lower, lower, "{name}");
                    assert_eq!(value.ops(), &[NativeOp::Const(11.0)], "{name}");
                    assert!(
                        !index_program.ops().is_empty(),
                        "{name}: index program must remain on helper path"
                    );
                }
                other => panic!("{name}: expected indexed helper path, got {other:?}"),
            }
        }
    }

    fn indexed_assignment_step(
        base: usize,
        len: usize,
        lower: i64,
        index: Vec<Instruction>,
        value: Vec<Instruction>,
    ) -> AssignmentStep {
        AssignmentStep::AssignIndexed {
            base,
            len,
            lower,
            index: BytecodeProgram {
                instructions: index,
            },
            value: BytecodeProgram {
                instructions: value,
            },
        }
    }

    fn compiled_model_with_variables(num_variables: usize) -> CompiledModel {
        CompiledModel {
            name: SmolStr::new("native_x64_assignment_test"),
            num_terminals: 0,
            terminal_names: Vec::new(),
            parameters: Vec::new(),
            num_variables,
            variable_names: Vec::new(),
            assignment_steps: Vec::new(),
            stamp_programs: Vec::new(),
            lookup_tables: Vec::new(),
            internal_nodes: 0,
            branch_sources: Vec::new(),
            laplace_filters: Vec::new(),
            zi_filters: Vec::new(),
            noise_sources: Vec::new(),
        }
    }
}
