pub(crate) mod codegen;
pub mod encoder;

use super::JitResult;
use super::expr::{EntryKind, NativeProgram};
use super::model::{CodeOffset, NativeEntryOffsets, NativeModel};
use super::runtime::ExecutableMemory;
use crate::codegen::{AssignmentStep, CompiledModel};

pub(crate) fn compile_model(model: &CompiledModel) -> JitResult<NativeModel> {
    super::validate_native_coverage(model)?;

    let mut image = Vec::new();
    let assignment = CodeOffset::new(image.len());
    append_assignment_entry(model, &mut image)?;

    let mut stamp_values = Vec::with_capacity(model.stamp_programs.len());
    let mut jacobians = Vec::with_capacity(model.stamp_programs.len());

    for stamp in &model.stamp_programs {
        let program = NativeProgram::from_bytecode(
            model.name.clone(),
            EntryKind::StampValue,
            &stamp.value_program,
            model.num_terminals,
            model.internal_nodes,
        )?;
        stamp_values.push(append_value_entry(&mut image, &program)?);

        let mut stamp_jacobians = Vec::with_capacity(stamp.jacobian_programs.len());
        for jacobian in &stamp.jacobian_programs {
            let program = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::Jacobian,
                &jacobian.program,
                model.num_terminals,
                model.internal_nodes,
            )?;
            stamp_jacobians.push(append_value_entry(&mut image, &program)?);
        }
        jacobians.push(stamp_jacobians);
    }

    let executable = ExecutableMemory::allocate(&image)?;
    NativeModel::from_executable_image(
        model.num_variables,
        executable,
        NativeEntryOffsets {
            assignment,
            stamp_values,
            jacobians,
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
                model.num_terminals,
                model.internal_nodes,
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
