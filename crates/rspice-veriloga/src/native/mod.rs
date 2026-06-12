//! Native Code Generation for Verilog-A
//!
//! Provides JIT compilation of Verilog-A models to native machine code
//! using Cranelift. This gives near-C performance without requiring
//! external compilers.

#[cfg(feature = "native")]
pub mod cranelift_jit;

#[cfg(feature = "native")]
pub use cranelift_jit::{
    EvalContext, JitCompiler, JitError, JitResult, NativeModel, PlanStats, PlanStep, StampFn,
};

use crate::codegen::CompiledModel;

/// Try to compile a model to native code
///
/// Returns `Some(NativeModel)` if compilation succeeds, `None` otherwise.
/// The caller should fall back to bytecode interpretation if this returns `None`.
#[cfg(feature = "native")]
pub fn try_compile_native(model: &CompiledModel) -> Option<NativeModel> {
    log::debug!("[JIT] Compiling model '{}' with Cranelift...", model.name);

    // A compiler panic (e.g. a register-allocator limit on a pathological
    // function) must degrade to the interpreter, never kill a simulation
    let compiled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        match JitCompiler::new() {
            Ok(compiler) => match compiler.compile(model) {
                Ok(native_model) => {
                    log::info!(
                        "[JIT] Compiled '{}': {} assignment chunks, {}/{} native stamps",
                        model.name,
                        native_model.chunk_count(),
                        native_model.native_stamp_count(),
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
    }));
    match compiled {
        Ok(result) => result,
        Err(_) => {
            log::warn!(
                "[JIT] Compiler panicked on '{}'; falling back to the interpreter",
                model.name
            );
            None
        }
    }
}

/// Stub for when native feature is disabled
#[cfg(not(feature = "native"))]
pub fn try_compile_native(_model: &CompiledModel) -> Option<()> {
    None
}
