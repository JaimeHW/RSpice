pub(crate) mod codegen;
pub mod encoder;

use super::expr::{EntryKind, NativeLoweringLimits, NativeProgram};
use super::model::{CodeOffset, NativeCurrentDependencies, NativeEntryOffsets, NativeModel};
use super::runtime::ExecutableMemory;
use super::{JitError, JitResult};
use crate::codegen::{AssignmentStep, CompiledModel, StampIndex, StampProgram};

pub(crate) fn compile_model(model: &CompiledModel) -> JitResult<NativeModel> {
    super::validate_native_coverage(model)?;
    let base_limits = NativeLoweringLimits::for_model(model);

    let mut image = Vec::new();
    let assignment = CodeOffset::new(image.len());
    append_assignment_entry(model, &mut image)?;

    let mut stamp_values = Vec::with_capacity(model.stamp_programs.len());
    let mut stamp_value_current_dependencies = Vec::with_capacity(model.stamp_programs.len());
    let mut jacobians = Vec::with_capacity(model.stamp_programs.len());
    let mut jacobian_current_dependencies = Vec::with_capacity(model.stamp_programs.len());
    let mut available_current_pairs = Vec::new();

    for stamp in &model.stamp_programs {
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
        executable,
        NativeEntryOffsets {
            assignment,
            stamp_values,
            jacobians,
            reactive_jacobians: vec![Vec::new(); model.stamp_programs.len()],
        },
        NativeCurrentDependencies {
            stamp_values: stamp_value_current_dependencies,
            jacobians: jacobian_current_dependencies,
            reactive_jacobians: vec![Vec::new(); model.stamp_programs.len()],
        },
    )
}

fn append_assignment_entry(model: &CompiledModel, image: &mut Vec<u8>) -> JitResult<()> {
    let assignments = model
        .assignment_steps
        .iter()
        .map(|step| {
            let AssignmentStep::Assign(assignment) = step else {
                unreachable!("unsupported assignment step was rejected before lowering");
            };
            let program = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::Assignment,
                &assignment.program,
                NativeLoweringLimits::for_model(model),
            )?;
            Ok((assignment.var_index, program))
        })
        .collect::<JitResult<Vec<_>>>()?;

    let bytes = if assignments.is_empty() {
        vec![0xC3]
    } else {
        codegen::compile_assignment_pass_function(&assignments)?
    };
    image.extend_from_slice(&bytes);
    Ok(())
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
