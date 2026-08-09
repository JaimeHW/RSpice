//! Backend-independent coverage checks for executable JIT targets.

use super::{JitError, JitResult};
use crate::codegen::{AssignmentStep, CompiledModel};

pub(crate) fn validate_jit_coverage(model: &CompiledModel) -> JitResult<()> {
    for step in &model.assignment_steps {
        validate_assignment_coverage(model, step)?;
    }
    Ok(())
}

fn validate_assignment_coverage(model: &CompiledModel, step: &AssignmentStep) -> JitResult<()> {
    match step {
        AssignmentStep::Assign(_) => Ok(()),
        AssignmentStep::AssignIndexed { base, len, .. } => {
            validate_assignment_range(model, *base, *len)
        }
        AssignmentStep::Loop { body, .. } => {
            for step in body {
                validate_assignment_coverage(model, step)?;
            }
            Ok(())
        }
    }
}

pub(crate) fn validate_assignment_range(
    model: &CompiledModel,
    base: usize,
    len: usize,
) -> JitResult<()> {
    let Some(end) = base.checked_add(len) else {
        return Err(JitError::unsupported_native_coverage(
            model.name.clone(),
            "AssignIndexedRangeOverflow",
        ));
    };
    if len == 0 || end > model.num_variables {
        return Err(JitError::unsupported_native_coverage(
            model.name.clone(),
            "AssignIndexedRange",
        ));
    }
    Ok(())
}
