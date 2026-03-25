//! Cranelift JIT Compiler for Verilog-A
//!
//! Compiles Verilog-A bytecode to native machine code using Cranelift.
//! This provides dependency-free native compilation.

#![allow(unsafe_attr_outside_unsafe)]

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{FuncId, Linkage, Module};
use std::collections::HashMap;
mod compiler_emit;
mod compiler_expr;
mod compiler_imports;

mod abi;

use crate::codegen::{BytecodeProgram, CompiledModel, Instruction, StampProgram};
pub use abi::{
    EvalContext, rspice_current_lookup, rspice_laplace_step, rspice_limexp, rspice_limit,
    rspice_table_lookup,
};

type AssignmentFn = extern "C" fn(*const EvalContext, *mut f64);
type StampFn = extern "C" fn(*const EvalContext, *const f64) -> f64;

/// Result type for JIT operations
pub type JitResult<T> = Result<T, JitError>;

/// JIT compilation errors
#[derive(Debug)]
pub enum JitError {
    /// Cranelift module error
    Module(String),
    /// Cranelift codegen error
    Codegen(String),
    /// Function not found
    FunctionNotFound(String),
}

impl std::fmt::Display for JitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JitError::Module(msg) => write!(f, "JIT module error: {}", msg),
            JitError::Codegen(msg) => write!(f, "JIT codegen error: {}", msg),
            JitError::FunctionNotFound(name) => write!(f, "Function not found: {}", name),
        }
    }
}

impl std::error::Error for JitError {}

/// Compiled native model with JIT-generated functions
pub struct NativeModel {
    /// Number of variables
    pub num_variables: usize,
    /// Number of stamp programs
    pub num_stamps: usize,
    /// Assignment evaluation function: fn(*const EvalContext, *mut f64 vars)
    assignment_fn: Option<AssignmentFn>,
    /// Stamp evaluation functions: fn(*const EvalContext, *const f64 vars) -> f64
    stamp_fns: Vec<StampFn>,
    /// Keep module alive
    _module: JITModule,
}

// Safety: The raw function pointers are only called with proper arguments
unsafe impl Send for NativeModel {}
unsafe impl Sync for NativeModel {}

impl std::fmt::Debug for NativeModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeModel")
            .field("num_variables", &self.num_variables)
            .field("num_stamps", &self.num_stamps)
            .field("has_assignment_fn", &self.assignment_fn.is_some())
            .field("stamp_fns_count", &self.stamp_fns.len())
            .finish()
    }
}

const EVAL_CTX_OFFSET_VOLTAGES: i32 = std::mem::offset_of!(EvalContext, voltages) as i32;
const EVAL_CTX_OFFSET_INTERNAL_VOLTAGES: i32 =
    std::mem::offset_of!(EvalContext, internal_voltages) as i32;
const EVAL_CTX_OFFSET_PARAMS: i32 = std::mem::offset_of!(EvalContext, params) as i32;
const EVAL_CTX_OFFSET_BRANCH_CURRENTS: i32 =
    std::mem::offset_of!(EvalContext, branch_currents) as i32;
const EVAL_CTX_OFFSET_BRANCH_CURRENTS_LEN: i32 =
    std::mem::offset_of!(EvalContext, branch_currents_len) as i32;
const EVAL_CTX_OFFSET_CURRENTS: i32 = std::mem::offset_of!(EvalContext, currents) as i32;
const EVAL_CTX_OFFSET_CURRENTS_LEN: i32 = std::mem::offset_of!(EvalContext, currents_len) as i32;
const EVAL_CTX_OFFSET_NUM_TERMINALS: i32 = std::mem::offset_of!(EvalContext, num_terminals) as i32;
const EVAL_CTX_OFFSET_TEMPERATURE: i32 = std::mem::offset_of!(EvalContext, temperature) as i32;
const EVAL_CTX_OFFSET_TIME: i32 = std::mem::offset_of!(EvalContext, time) as i32;
const EVAL_CTX_OFFSET_TIMESTEP: i32 = std::mem::offset_of!(EvalContext, timestep) as i32;
const EVAL_CTX_OFFSET_STATE_PREV: i32 = std::mem::offset_of!(EvalContext, state_prev) as i32;
const EVAL_CTX_OFFSET_LOOKUP_TABLES: i32 = std::mem::offset_of!(EvalContext, lookup_tables) as i32;
const EVAL_CTX_OFFSET_LOOKUP_TABLES_LEN: i32 =
    std::mem::offset_of!(EvalContext, lookup_tables_len) as i32;
const EVAL_CTX_OFFSET_LAPLACE_FILTERS: i32 =
    std::mem::offset_of!(EvalContext, laplace_filters) as i32;
const EVAL_CTX_OFFSET_LAPLACE_FILTERS_LEN: i32 =
    std::mem::offset_of!(EvalContext, laplace_filters_len) as i32;

impl NativeModel {
    unsafe fn cast_assignment_fn(ptr: *const u8) -> AssignmentFn {
        // Safety: the JIT compiler emits `assignment_fn` using the
        // `extern "C" fn(*const EvalContext, *mut f64)` ABI and keeps the
        // owning `JITModule` alive for the lifetime of the function pointer.
        unsafe { std::mem::transmute(ptr) }
    }

    unsafe fn cast_stamp_fn(ptr: *const u8) -> StampFn {
        // Safety: the JIT compiler emits each stamp entry using the
        // `extern "C" fn(*const EvalContext, *const f64) -> f64` ABI and keeps
        // the owning `JITModule` alive for the lifetime of the function pointer.
        unsafe { std::mem::transmute(ptr) }
    }

    /// Evaluate all assignments, storing results in vars array
    pub fn evaluate_assignments(&self, ctx: &EvalContext, vars: &mut [f64]) {
        if let Some(fn_ptr) = self.assignment_fn {
            fn_ptr(ctx as *const EvalContext, vars.as_mut_ptr());
        }
    }

    /// Evaluate a single stamp program
    pub fn evaluate_stamp(&self, index: usize, ctx: &EvalContext, vars: &[f64]) -> f64 {
        if let Some(&fn_ptr) = self.stamp_fns.get(index) {
            fn_ptr(ctx as *const EvalContext, vars.as_ptr())
        } else {
            0.0
        }
    }
}

/// JIT compiler using Cranelift
pub struct JitCompiler {
    /// Cranelift target ISA
    isa: cranelift_codegen::isa::OwnedTargetIsa,
}

impl JitCompiler {
    /// Create a new JIT compiler for the host platform
    pub fn new() -> JitResult<Self> {
        let mut flag_builder = settings::builder();
        // Enable optimizations
        flag_builder
            .set("opt_level", "speed")
            .map_err(|e| JitError::Codegen(e.to_string()))?;

        let isa_builder =
            cranelift_native::builder().map_err(|e| JitError::Codegen(e.to_string()))?;

        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .map_err(|e| JitError::Codegen(e.to_string()))?;

        Ok(Self { isa })
    }

    /// Compile a model to native code
    pub fn compile(&self, model: &CompiledModel) -> JitResult<NativeModel> {
        let mut builder =
            JITBuilder::with_isa(self.isa.clone(), cranelift_module::default_libcall_names());

        // Register rspice helper function symbols for JIT linking
        // These must be registered before module creation so Cranelift can resolve them
        builder.symbol("rspice_table_lookup", rspice_table_lookup as *const u8);
        builder.symbol("rspice_limit", rspice_limit as *const u8);
        builder.symbol("rspice_limexp", rspice_limexp as *const u8);
        builder.symbol("rspice_laplace_step", rspice_laplace_step as *const u8);
        builder.symbol("rspice_current_lookup", rspice_current_lookup as *const u8);

        let mut module = JITModule::new(builder);
        let mut ctx = module.make_context();

        // Import math functions
        let math_funcs = self.import_math_functions(&mut module)?;

        // Compile assignment function if there are assignments
        let assignment_fn_id = if !model.assignment_programs.is_empty() {
            Some(self.compile_assignments(&mut module, &mut ctx, model, &math_funcs)?)
        } else {
            None
        };

        // Compile stamp functions
        let mut stamp_fn_ids = Vec::new();
        for (i, stamp) in model.stamp_programs.iter().enumerate() {
            let fn_id = self.compile_stamp(&mut module, &mut ctx, model, stamp, i, &math_funcs)?;
            stamp_fn_ids.push(fn_id);
        }

        // Finalize the module
        module
            .finalize_definitions()
            .map_err(|e| JitError::Module(e.to_string()))?;

        // Get function pointers
        let assignment_fn = assignment_fn_id.map(|id| unsafe {
            NativeModel::cast_assignment_fn(module.get_finalized_function(id))
        });
        let stamp_fns: Vec<_> = stamp_fn_ids
            .iter()
            .map(|&id| unsafe { NativeModel::cast_stamp_fn(module.get_finalized_function(id)) })
            .collect();

        Ok(NativeModel {
            num_variables: model.num_variables,
            num_stamps: model.stamp_programs.len(),
            assignment_fn,
            stamp_fns,
            _module: module,
        })
    }
}

impl Default for JitCompiler {
    fn default() -> Self {
        Self::new().expect("Failed to create JIT compiler")
    }
}

#[cfg(test)]
mod tests;
