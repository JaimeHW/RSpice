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

use crate::codegen::{AssignmentStep, BytecodeProgram, CompiledModel, Instruction};
pub use abi::{
    EvalContext, rspice_current_lookup, rspice_laplace_step, rspice_limexp, rspice_limit,
    rspice_table_lookup,
};
pub(crate) use compiler_expr::program_is_jitable;

type AssignmentFn = extern "C" fn(*const EvalContext, *mut f64);
/// Value-returning native program: fn(ctx, vars) -> f64
pub type StampFn = extern "C" fn(*const EvalContext, *const f64) -> f64;

/// One node of the hybrid assignment execution plan. The plan mirrors the
/// model's assignment-step tree: contiguous runs of plain assignments
/// become native chunk functions, everything the JIT refuses stays on the
/// bytecode interpreter, and runtime loops keep interpreted conditions
/// around hybrid bodies.
#[derive(Debug)]
pub enum PlanStep {
    /// Run the chunk function `id`, covering original steps `[from, to)`
    Chunk { id: usize, from: usize, to: usize },
    /// Interpret the original steps `[from, to)` at this tree level
    Interpret { from: usize, to: usize },
    /// Runtime loop at step `index` of this level: the condition program
    /// is interpreted before every iteration; the body runs hybrid
    Loop { index: usize, body: Vec<PlanStep> },
}

/// Hybrid-plan composition counters (diagnostics)
#[derive(Debug, Default, Clone, Copy)]
pub struct PlanStats {
    /// Native chunk functions
    pub chunks: usize,
    /// Assignment steps covered by native chunks
    pub chunked_steps: usize,
    /// Assignment steps left on the interpreter
    pub interpreted_steps: usize,
    /// Runtime loops (conditions always interpret)
    pub loops: usize,
}

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
    /// Instruction the JIT cannot compile faithfully; the caller falls
    /// back to the bytecode interpreter
    UnsupportedInstruction(&'static str),
}

impl std::fmt::Display for JitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JitError::Module(msg) => write!(f, "JIT module error: {}", msg),
            JitError::Codegen(msg) => write!(f, "JIT codegen error: {}", msg),
            JitError::FunctionNotFound(name) => write!(f, "Function not found: {}", name),
            JitError::UnsupportedInstruction(name) => {
                write!(f, "Instruction not supported by the JIT: {}", name)
            }
        }
    }
}

impl std::error::Error for JitError {}

/// Compiled native model with JIT-generated functions
pub struct NativeModel {
    /// Number of variables
    pub num_variables: usize,
    /// Hybrid execution plan over the model's assignment steps
    pub plan: Vec<PlanStep>,
    /// Chunk functions referenced by [`PlanStep::Chunk`]
    chunk_fns: Vec<AssignmentFn>,
    /// Per-stamp native value programs (None falls back to the interpreter)
    stamp_value_fns: Vec<Option<StampFn>>,
    /// Per-stamp, per-entry native resistive Jacobian programs
    jacobian_fns: Vec<Vec<Option<StampFn>>>,
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
            .field("chunks", &self.chunk_fns.len())
            .field(
                "native_stamps",
                &self.stamp_value_fns.iter().filter(|f| f.is_some()).count(),
            )
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
const EVAL_CTX_OFFSET_LOOKUP_TABLES: i32 = std::mem::offset_of!(EvalContext, lookup_tables) as i32;
const EVAL_CTX_OFFSET_LOOKUP_TABLES_LEN: i32 =
    std::mem::offset_of!(EvalContext, lookup_tables_len) as i32;
const EVAL_CTX_OFFSET_PARAM_GIVEN: i32 = std::mem::offset_of!(EvalContext, param_given) as i32;
const EVAL_CTX_OFFSET_BRANCH_UNKNOWNS: i32 =
    std::mem::offset_of!(EvalContext, branch_unknowns) as i32;
const EVAL_CTX_OFFSET_ANALYSIS_TYPE: i32 = std::mem::offset_of!(EvalContext, analysis_type) as i32;
const EVAL_CTX_OFFSET_TIMESTEP: i32 = std::mem::offset_of!(EvalContext, timestep) as i32;
const EVAL_CTX_OFFSET_STATE_PREV: i32 = std::mem::offset_of!(EvalContext, state_prev) as i32;
const EVAL_CTX_OFFSET_STATE_VALUES: i32 = std::mem::offset_of!(EvalContext, state_values) as i32;
const EVAL_CTX_OFFSET_MULTIPLICITY: i32 = std::mem::offset_of!(EvalContext, multiplicity) as i32;

impl NativeModel {
    unsafe fn cast_assignment_fn(ptr: *const u8) -> AssignmentFn {
        // Safety: the JIT compiler emits chunk functions using the
        // `extern "C" fn(*const EvalContext, *mut f64)` ABI and keeps the
        // owning `JITModule` alive for the lifetime of the function pointer.
        unsafe { std::mem::transmute(ptr) }
    }

    unsafe fn cast_stamp_fn(ptr: *const u8) -> StampFn {
        // Safety: the JIT compiler emits each program entry using the
        // `extern "C" fn(*const EvalContext, *const f64) -> f64` ABI and keeps
        // the owning `JITModule` alive for the lifetime of the function pointer.
        unsafe { std::mem::transmute(ptr) }
    }

    /// Run one assignment chunk, writing results into `vars`.
    ///
    /// Safety contract: every raw pointer in `ctx` is valid and `vars`
    /// covers the model's full variable storage.
    pub fn run_chunk(&self, chunk: usize, ctx: &EvalContext, vars: *mut f64) {
        if let Some(&fn_ptr) = self.chunk_fns.get(chunk) {
            fn_ptr(ctx as *const EvalContext, vars);
        }
    }

    /// Native value program of a stamp, when one compiled
    pub fn stamp_value_fn(&self, index: usize) -> Option<StampFn> {
        self.stamp_value_fns.get(index).copied().flatten()
    }

    /// Native resistive Jacobian program of a stamp entry, when one
    /// compiled
    pub fn jacobian_fn(&self, stamp: usize, entry: usize) -> Option<StampFn> {
        self.jacobian_fns
            .get(stamp)
            .and_then(|fns| fns.get(entry))
            .copied()
            .flatten()
    }

    /// Number of compiled assignment chunks
    pub fn chunk_count(&self) -> usize {
        self.chunk_fns.len()
    }

    /// Number of stamp value programs that compiled to native code
    pub fn native_stamp_count(&self) -> usize {
        self.stamp_value_fns.iter().filter(|f| f.is_some()).count()
    }

    /// Composition of the hybrid plan (all nesting levels)
    pub fn plan_stats(&self) -> PlanStats {
        fn walk(plan: &[PlanStep], stats: &mut PlanStats) {
            for step in plan {
                match step {
                    PlanStep::Chunk { from, to, .. } => {
                        stats.chunks += 1;
                        stats.chunked_steps += to - from;
                    }
                    PlanStep::Interpret { from, to } => {
                        stats.interpreted_steps += to - from;
                    }
                    PlanStep::Loop { body, .. } => {
                        stats.loops += 1;
                        walk(body, stats);
                    }
                }
            }
        }
        let mut stats = PlanStats::default();
        walk(&self.plan, &mut stats);
        stats
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

    /// Upper bound on bytecode instructions per chunk function. Cranelift
    /// compile time grows superlinearly with function size, and regalloc2
    /// aborts past ~2M SSA values; autodiffed model assignments can carry
    /// thousands of instructions each, so the budget counts instructions
    /// rather than steps.
    const MAX_CHUNK_INSTRUCTIONS: usize = 20_000;

    /// Compile a model to native code.
    ///
    /// Compilation is per-program: assignment runs become chunk functions,
    /// and each stamp value / Jacobian program compiles independently.
    /// Anything the JIT refuses stays on the bytecode interpreter — a
    /// partial result is normal, not an error.
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

        // Build the hybrid assignment plan, compiling chunk functions
        let mut chunk_fn_ids: Vec<FuncId> = Vec::new();
        let plan = self.build_plan(
            &model.assignment_steps,
            model,
            &mut module,
            &mut ctx,
            &math_funcs,
            &mut chunk_fn_ids,
        )?;

        // Compile stamp value and Jacobian programs individually
        let mut stamp_value_ids: Vec<Option<FuncId>> = Vec::new();
        let mut jacobian_ids: Vec<Vec<Option<FuncId>>> = Vec::new();
        for (i, stamp) in model.stamp_programs.iter().enumerate() {
            let value_id = if program_is_jitable(&stamp.value_program) {
                self.compile_program(
                    &mut module,
                    &mut ctx,
                    model,
                    &stamp.value_program,
                    &format!("{}_stamp_{}", model.name, i),
                    &math_funcs,
                )
                .ok()
            } else {
                None
            };
            stamp_value_ids.push(value_id);

            let mut entries = Vec::with_capacity(stamp.jacobian_programs.len());
            for (j, jacobian) in stamp.jacobian_programs.iter().enumerate() {
                let id = if program_is_jitable(&jacobian.program) {
                    self.compile_program(
                        &mut module,
                        &mut ctx,
                        model,
                        &jacobian.program,
                        &format!("{}_jac_{}_{}", model.name, i, j),
                        &math_funcs,
                    )
                    .ok()
                } else {
                    None
                };
                entries.push(id);
            }
            jacobian_ids.push(entries);
        }

        // Finalize the module
        module
            .finalize_definitions()
            .map_err(|e| JitError::Module(e.to_string()))?;

        // Get function pointers
        let chunk_fns: Vec<_> = chunk_fn_ids
            .iter()
            .map(|&id| unsafe {
                NativeModel::cast_assignment_fn(module.get_finalized_function(id))
            })
            .collect();
        let stamp_value_fns: Vec<_> = stamp_value_ids
            .iter()
            .map(|id| {
                id.map(|id| unsafe {
                    NativeModel::cast_stamp_fn(module.get_finalized_function(id))
                })
            })
            .collect();
        let jacobian_fns: Vec<Vec<_>> = jacobian_ids
            .iter()
            .map(|entries| {
                entries
                    .iter()
                    .map(|id| {
                        id.map(|id| unsafe {
                            NativeModel::cast_stamp_fn(module.get_finalized_function(id))
                        })
                    })
                    .collect()
            })
            .collect();

        Ok(NativeModel {
            num_variables: model.num_variables,
            plan,
            chunk_fns,
            stamp_value_fns,
            jacobian_fns,
            _module: module,
        })
    }

    /// Build the hybrid plan for one level of the assignment-step tree:
    /// contiguous runs of JIT-compilable plain assignments become chunk
    /// functions; refused steps stay interpreted; loops recurse.
    #[allow(clippy::too_many_arguments)]
    fn build_plan(
        &self,
        steps: &[AssignmentStep],
        model: &CompiledModel,
        module: &mut JITModule,
        ctx: &mut cranelift::prelude::codegen::Context,
        math_funcs: &HashMap<&'static str, FuncId>,
        chunk_fn_ids: &mut Vec<FuncId>,
    ) -> JitResult<Vec<PlanStep>> {
        enum Pending {
            None,
            Chunk(usize),
            Interpret(usize),
        }

        let mut plan = Vec::new();
        let mut pending = Pending::None;

        let emit_chunk = |this: &Self,
                          plan: &mut Vec<PlanStep>,
                          module: &mut JITModule,
                          ctx: &mut cranelift::prelude::codegen::Context,
                          chunk_fn_ids: &mut Vec<FuncId>,
                          from: usize,
                          to: usize|
         -> JitResult<()> {
            let programs: Vec<&crate::codegen::AssignmentProgram> = steps[from..to]
                .iter()
                .map(|step| match step {
                    AssignmentStep::Assign(assign) => assign,
                    _ => unreachable!("chunk runs contain only plain assignments"),
                })
                .collect();
            let chunk_id = chunk_fn_ids.len();
            match this.compile_assignment_chunk(module, ctx, model, &programs, chunk_id, math_funcs)
            {
                Ok(fn_id) => {
                    chunk_fn_ids.push(fn_id);
                    plan.push(PlanStep::Chunk {
                        id: chunk_id,
                        from,
                        to,
                    });
                    Ok(())
                }
                // The pre-scan should make refusals rare; degrade the run
                // to interpretation rather than losing the whole model
                Err(JitError::UnsupportedInstruction(what)) => {
                    log::debug!(
                        "[JIT] '{}': chunk over steps {from}..{to} refused ({what}); \
                         interpreting",
                        model.name
                    );
                    plan.push(PlanStep::Interpret { from, to });
                    Ok(())
                }
                Err(err) => Err(err),
            }
        };

        let mut chunk_instructions = 0usize;
        for (i, step) in steps.iter().enumerate() {
            match step {
                AssignmentStep::Assign(assign)
                    if compiler_expr::program_is_jitable(&assign.program) =>
                {
                    let cost = assign.program.instructions.len();
                    match pending {
                        Pending::Chunk(start)
                            if chunk_instructions + cost > Self::MAX_CHUNK_INSTRUCTIONS =>
                        {
                            emit_chunk(self, &mut plan, module, ctx, chunk_fn_ids, start, i)?;
                            pending = Pending::Chunk(i);
                            chunk_instructions = cost;
                        }
                        Pending::Chunk(_) => chunk_instructions += cost,
                        Pending::Interpret(start) => {
                            plan.push(PlanStep::Interpret { from: start, to: i });
                            pending = Pending::Chunk(i);
                            chunk_instructions = cost;
                        }
                        Pending::None => {
                            pending = Pending::Chunk(i);
                            chunk_instructions = cost;
                        }
                    }
                }
                AssignmentStep::Loop { body, .. } => {
                    match pending {
                        Pending::Chunk(start) => {
                            emit_chunk(self, &mut plan, module, ctx, chunk_fn_ids, start, i)?;
                        }
                        Pending::Interpret(start) => {
                            plan.push(PlanStep::Interpret { from: start, to: i });
                        }
                        Pending::None => {}
                    }
                    pending = Pending::None;
                    let body_plan =
                        self.build_plan(body, model, module, ctx, math_funcs, chunk_fn_ids)?;
                    plan.push(PlanStep::Loop {
                        index: i,
                        body: body_plan,
                    });
                }
                // Indexed writes and refused assignments interpret
                _ => match pending {
                    Pending::Interpret(_) => {}
                    Pending::Chunk(start) => {
                        emit_chunk(self, &mut plan, module, ctx, chunk_fn_ids, start, i)?;
                        pending = Pending::Interpret(i);
                    }
                    Pending::None => pending = Pending::Interpret(i),
                },
            }
        }

        match pending {
            Pending::Chunk(start) => {
                emit_chunk(
                    self,
                    &mut plan,
                    module,
                    ctx,
                    chunk_fn_ids,
                    start,
                    steps.len(),
                )?;
            }
            Pending::Interpret(start) => {
                plan.push(PlanStep::Interpret {
                    from: start,
                    to: steps.len(),
                });
            }
            Pending::None => {}
        }

        Ok(plan)
    }
}
