//! Native Code Generation for Verilog-A
//!
//! Provides JIT compilation of Verilog-A models to native machine code
//! using Cranelift. This gives near-C performance without requiring
//! external compilers.

#[cfg(feature = "native")]
pub mod cranelift_jit;

#[cfg(feature = "native")]
pub use cranelift_jit::{EvalContext, JitCompiler, JitError, JitResult, NativeModel};

use crate::codegen::CompiledModel;

/// Try to compile a model to native code
///
/// Returns `Some(NativeModel)` if compilation succeeds, `None` otherwise.
/// The caller should fall back to bytecode interpretation if this returns `None`.
#[cfg(feature = "native")]
pub fn try_compile_native(model: &CompiledModel) -> Option<NativeModel> {
    log::debug!("[JIT] Compiling model '{}' with Cranelift...", model.name);

    match JitCompiler::new() {
        Ok(compiler) => match compiler.compile(model) {
            Ok(native_model) => {
                log::info!(
                    "[JIT] Successfully compiled '{}': {} assignments, {} stamps",
                    model.name,
                    model.assignment_programs.len(),
                    model.stamp_programs.len()
                );
                Some(native_model)
            }
            Err(e) => {
                log::warn!("[JIT] Compilation failed for '{}': {}", model.name, e);
                None
            }
        },
        Err(e) => {
            log::warn!("[JIT] Failed to create compiler: {}", e);
            None
        }
    }
}

/// Stub for when native feature is disabled
#[cfg(not(feature = "native"))]
pub fn try_compile_native(_model: &CompiledModel) -> Option<()> {
    None
}

#[cfg(test)]
mod tests;
