pub(crate) mod codegen;
pub mod encoder;

use super::expr::{EntryKind, NativeProgram};
use super::model::{CodeOffset, NativeEntryOffsets, NativeModel};
use super::runtime::ExecutableMemory;
use super::{JitError, JitResult};
use crate::codegen::{AssignmentStep, CompiledModel};

pub(crate) fn compile_model(model: &CompiledModel) -> JitResult<NativeModel> {
    validate_model_coverage(model)?;

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
        )?;
        stamp_values.push(append_value_entry(&mut image, &program)?);

        let mut stamp_jacobians = Vec::with_capacity(stamp.jacobian_programs.len());
        for jacobian in &stamp.jacobian_programs {
            let program = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::Jacobian,
                &jacobian.program,
                model.num_terminals,
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

fn validate_model_coverage(model: &CompiledModel) -> JitResult<()> {
    if model
        .parameters
        .iter()
        .any(|parameter| parameter.default_program.is_some())
    {
        return Err(JitError::unsupported_native_coverage(
            model.name.clone(),
            "DependentParameterDefaults",
        ));
    }

    if model
        .stamp_programs
        .iter()
        .any(|stamp| stamp.static_condition.is_some())
    {
        return Err(JitError::unsupported_native_coverage(
            model.name.clone(),
            "StaticConditionPrograms",
        ));
    }

    if !model.noise_sources.is_empty() {
        return Err(JitError::unsupported_native_coverage(
            model.name.clone(),
            "NoiseSources",
        ));
    }

    if model
        .stamp_programs
        .iter()
        .any(|stamp| !stamp.reactive_jacobians.is_empty())
    {
        return Err(JitError::unsupported_native_coverage(
            model.name.clone(),
            "ReactiveJacobians",
        ));
    }

    for step in &model.assignment_steps {
        validate_assignment_step(model, step)?;
    }

    Ok(())
}

fn validate_assignment_step(model: &CompiledModel, step: &AssignmentStep) -> JitResult<()> {
    match step {
        AssignmentStep::Assign(_) => Ok(()),
        AssignmentStep::AssignIndexed { .. } => Err(JitError::unsupported_native_coverage(
            model.name.clone(),
            "AssignIndexed",
        )),
        AssignmentStep::Loop { .. } => Err(JitError::unsupported_native_coverage(
            model.name.clone(),
            "Loop",
        )),
    }
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
