//! Native JIT backend for Verilog-A models.
//!
//! Native mode is full JIT or error. The bytecode interpreter is not a
//! fallback path when this module is asked to compile a model.

mod abi;
mod error;
mod model;
mod runtime;
mod target;
pub mod x64;

pub use abi::{
    rspice_current_lookup, rspice_laplace_step, rspice_limexp, rspice_limit, rspice_table_lookup,
    EvalContext,
};
pub use error::{JitError, JitResult};
pub use model::{AssignmentFn, NativeModel, PlanStats, PlanStep, StampFn};
pub use runtime::ExecutableMemory;
pub use target::{Architecture, TargetSpec};

use crate::codegen::CompiledModel;

pub fn compile_native(model: &CompiledModel) -> JitResult<NativeModel> {
    let target = TargetSpec::host().ok_or_else(|| JitError::UnsupportedTarget {
        target: "unknown".into(),
        reason: "host architecture is not supported".into(),
    })?;

    match target.arch {
        Architecture::X64 => Err(JitError::unsupported_current_optir(model.name.clone())),
        Architecture::AArch64 => Err(JitError::UnsupportedTarget {
            target: target.display_name().into(),
            reason: "AArch64 backend boundary exists but is not enabled".into(),
        }),
    }
}

/// Compatibility shim for the pre-foundation device integration. New native
/// callers must use [`compile_native`] so unsupported coverage is surfaced as
/// a typed hard-fail error.
pub fn try_compile_native(model: &CompiledModel) -> Option<NativeModel> {
    match compile_native(model) {
        Ok(native) => Some(native),
        Err(error) => {
            log::debug!(
                "[JIT] Native compilation failed for '{}': {}",
                model.name,
                error
            );
            None
        }
    }
}
