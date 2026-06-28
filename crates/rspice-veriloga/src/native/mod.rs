//! Native JIT backend for Verilog-A models.
//!
//! Native mode is full JIT or error. The bytecode interpreter is not a
//! fallback path when this module is asked to compile a model.

mod abi;
mod error;
mod expr;
mod model;
mod runtime;
mod target;
pub mod x64;

pub use abi::{
    EvalContext, rspice_current_lookup, rspice_dynamic_variable_load_native, rspice_laplace_step,
    rspice_laplace_step_native, rspice_limexp, rspice_limit, rspice_table_lookup,
    rspice_zi_step_native,
};
pub(crate) use abi::{clear_native_runtime_error, take_native_runtime_error};
pub use error::{JitError, JitResult};
pub use model::{NativeModel, PlanStats};
pub use target::{Architecture, TargetSpec};

use crate::codegen::{AssignmentStep, CompiledModel};

pub fn compile_native(model: &CompiledModel) -> JitResult<NativeModel> {
    validate_native_coverage(model)?;

    let target = TargetSpec::host().ok_or_else(|| JitError::UnsupportedTarget {
        target: "unknown".into(),
        reason: "host architecture is not supported".into(),
    })?;
    match target.arch {
        Architecture::X64 => x64::compile_model(model),
        Architecture::AArch64 => Err(JitError::UnsupportedTarget {
            target: target.display_name().into(),
            reason: "AArch64 backend boundary exists but is not enabled".into(),
        }),
    }
}

fn validate_native_coverage(model: &CompiledModel) -> JitResult<()> {
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
        AssignmentStep::Loop { .. } => Err(JitError::unsupported_native_coverage(
            model.name.clone(),
            "Loop",
        )),
    }
}

fn validate_assignment_range(model: &CompiledModel, base: usize, len: usize) -> JitResult<()> {
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
