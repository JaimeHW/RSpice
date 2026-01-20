//! Cranelift JIT Compiler for Verilog-A
//!
//! Compiles Verilog-A bytecode to native machine code using Cranelift.
//! This provides professional, dependency-free native compilation.

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{FuncId, Linkage, Module};
use std::collections::HashMap;

use crate::codegen::{BytecodeProgram, CompiledModel, Instruction, StampProgram};

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
    assignment_fn: Option<*const u8>,
    /// Stamp evaluation functions: fn(*const EvalContext, *const f64 vars) -> f64
    stamp_fns: Vec<*const u8>,
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

/// Evaluation context passed to JIT-compiled functions
#[repr(C)]
pub struct EvalContext {
    /// Terminal voltages array
    pub voltages: *const f64,
    /// Internal node voltages
    pub internal_voltages: *const f64,
    /// Parameter values
    pub params: *const f64,
    /// Temperature in Kelvin
    pub temperature: f64,
    /// Simulation time
    pub time: f64,
    /// Time step (for transient)
    pub timestep: f64,
    /// Previous state values (for ddt/idt)
    pub state_prev: *const f64,
}

impl NativeModel {
    /// Evaluate all assignments, storing results in vars array
    pub fn evaluate_assignments(&self, ctx: &EvalContext, vars: &mut [f64]) {
        if let Some(fn_ptr) = self.assignment_fn {
            // Safety: function signature matches, arrays are properly sized
            unsafe {
                let func: extern "C" fn(*const EvalContext, *mut f64) = std::mem::transmute(fn_ptr);
                func(ctx as *const EvalContext, vars.as_mut_ptr());
            }
        }
    }

    /// Evaluate a single stamp program
    pub fn evaluate_stamp(&self, index: usize, ctx: &EvalContext, vars: &[f64]) -> f64 {
        if let Some(&fn_ptr) = self.stamp_fns.get(index) {
            // Safety: function signature matches
            unsafe {
                let func: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                    std::mem::transmute(fn_ptr);
                func(ctx as *const EvalContext, vars.as_ptr())
            }
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
        let builder =
            JITBuilder::with_isa(self.isa.clone(), cranelift_module::default_libcall_names());

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
        let assignment_fn = assignment_fn_id.map(|id| module.get_finalized_function(id));
        let stamp_fns: Vec<_> = stamp_fn_ids
            .iter()
            .map(|&id| module.get_finalized_function(id))
            .collect();

        Ok(NativeModel {
            num_variables: model.num_variables,
            num_stamps: model.stamp_programs.len(),
            assignment_fn,
            stamp_fns,
            _module: module,
        })
    }

    /// Import standard math functions
    fn import_math_functions(
        &self,
        module: &mut JITModule,
    ) -> JitResult<HashMap<&'static str, FuncId>> {
        let mut funcs = HashMap::new();
        let ptr_type = self.isa.pointer_type();
        let _ = ptr_type; // Suppress warning

        // Define signatures for math functions
        let math_sig = {
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F64));
            sig.returns.push(AbiParam::new(types::F64));
            sig
        };

        let math2_sig = {
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F64));
            sig.params.push(AbiParam::new(types::F64));
            sig.returns.push(AbiParam::new(types::F64));
            sig
        };

        // Import single-arg math functions
        for name in [
            "exp", "log", "log10", "sqrt", "sin", "cos", "tan", "sinh", "cosh", "tanh", "asin",
            "acos", "atan", "floor", "ceil", "fabs",
        ] {
            let id = module
                .declare_function(name, Linkage::Import, &math_sig)
                .map_err(|e| JitError::Module(e.to_string()))?;
            funcs.insert(name, id);
        }

        // Import two-arg math functions
        for name in ["pow", "atan2", "fmin", "fmax"] {
            let id = module
                .declare_function(name, Linkage::Import, &math2_sig)
                .map_err(|e| JitError::Module(e.to_string()))?;
            funcs.insert(name, id);
        }

        Ok(funcs)
    }

    /// Compile all assignments into a single function
    fn compile_assignments(
        &self,
        module: &mut JITModule,
        ctx: &mut cranelift::prelude::codegen::Context,
        model: &CompiledModel,
        math_funcs: &HashMap<&'static str, FuncId>,
    ) -> JitResult<FuncId> {
        let ptr_type = self.isa.pointer_type();

        // Function signature: fn(ctx: *const EvalContext, vars: *mut f64)
        let mut sig = module.make_signature();
        sig.params.push(AbiParam::new(ptr_type)); // ctx
        sig.params.push(AbiParam::new(ptr_type)); // vars

        let func_id = module
            .declare_function(&format!("{}_assignments", model.name), Linkage::Local, &sig)
            .map_err(|e| JitError::Module(e.to_string()))?;

        ctx.func.signature = sig;
        ctx.func.name = cranelift_codegen::ir::UserFuncName::user(0, func_id.as_u32());

        let mut builder_ctx = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_ctx);

        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        let ctx_ptr = builder.block_params(entry_block)[0];
        let vars_ptr = builder.block_params(entry_block)[1];

        // Compile each assignment
        for assign in &model.assignment_programs {
            let value = self.compile_expression(
                &mut builder,
                &assign.program,
                ctx_ptr,
                vars_ptr,
                module,
                math_funcs,
            )?;

            // Store to vars[var_index]
            let offset = (assign.var_index * 8) as i32;
            builder
                .ins()
                .store(MemFlags::new(), value, vars_ptr, offset);
        }

        builder.ins().return_(&[]);
        builder.finalize();

        module
            .define_function(func_id, ctx)
            .map_err(|e| JitError::Module(e.to_string()))?;

        ctx.clear();
        Ok(func_id)
    }

    /// Compile a single stamp program
    fn compile_stamp(
        &self,
        module: &mut JITModule,
        ctx: &mut cranelift::prelude::codegen::Context,
        model: &CompiledModel,
        stamp: &StampProgram,
        index: usize,
        math_funcs: &HashMap<&'static str, FuncId>,
    ) -> JitResult<FuncId> {
        let ptr_type = self.isa.pointer_type();

        // Function signature: fn(ctx: *const EvalContext, vars: *const f64) -> f64
        let mut sig = module.make_signature();
        sig.params.push(AbiParam::new(ptr_type)); // ctx
        sig.params.push(AbiParam::new(ptr_type)); // vars
        sig.returns.push(AbiParam::new(types::F64));

        let func_id = module
            .declare_function(
                &format!("{}_stamp_{}", model.name, index),
                Linkage::Local,
                &sig,
            )
            .map_err(|e| JitError::Module(e.to_string()))?;

        ctx.func.signature = sig;
        ctx.func.name = cranelift_codegen::ir::UserFuncName::user(0, func_id.as_u32());

        let mut builder_ctx = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_ctx);

        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        let ctx_ptr = builder.block_params(entry_block)[0];
        let vars_ptr = builder.block_params(entry_block)[1];

        let value = self.compile_expression(
            &mut builder,
            &stamp.value_program,
            ctx_ptr,
            vars_ptr,
            module,
            math_funcs,
        )?;

        builder.ins().return_(&[value]);
        builder.finalize();

        module
            .define_function(func_id, ctx)
            .map_err(|e| JitError::Module(e.to_string()))?;

        ctx.clear();
        Ok(func_id)
    }

    /// Compile a bytecode program to Cranelift IR, returning the result value
    fn compile_expression(
        &self,
        builder: &mut FunctionBuilder,
        program: &BytecodeProgram,
        ctx_ptr: Value,
        vars_ptr: Value,
        module: &mut JITModule,
        math_funcs: &HashMap<&'static str, FuncId>,
    ) -> JitResult<Value> {
        let mut stack: Vec<Value> = Vec::new();

        for instr in &program.instructions {
            match instr {
                Instruction::PushConst(v) => {
                    let val = builder.ins().f64const(*v);
                    stack.push(val);
                }
                Instruction::PushParam(idx) => {
                    // ctx->params is at offset 16 (after voltages, internal_voltages)
                    let params_ptr = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::new(),
                        ctx_ptr,
                        16, // offset of params in EvalContext
                    );
                    let val = builder.ins().load(
                        types::F64,
                        MemFlags::new(),
                        params_ptr,
                        (*idx * 8) as i32,
                    );
                    stack.push(val);
                }
                Instruction::PushVoltage(pos, neg) => {
                    // ctx->voltages is at offset 0
                    let voltages_ptr =
                        builder
                            .ins()
                            .load(self.isa.pointer_type(), MemFlags::new(), ctx_ptr, 0);
                    let v_pos = builder.ins().load(
                        types::F64,
                        MemFlags::new(),
                        voltages_ptr,
                        (*pos * 8) as i32,
                    );
                    let v_neg = builder.ins().load(
                        types::F64,
                        MemFlags::new(),
                        voltages_ptr,
                        (*neg * 8) as i32,
                    );
                    let diff = builder.ins().fsub(v_pos, v_neg);
                    stack.push(diff);
                }
                Instruction::PushInternalVoltage(idx) => {
                    // ctx->internal_voltages is at offset 8
                    let internal_ptr =
                        builder
                            .ins()
                            .load(self.isa.pointer_type(), MemFlags::new(), ctx_ptr, 8);
                    let val = builder.ins().load(
                        types::F64,
                        MemFlags::new(),
                        internal_ptr,
                        (*idx * 8) as i32,
                    );
                    stack.push(val);
                }
                Instruction::PushVariable(idx) => {
                    let val = builder.ins().load(
                        types::F64,
                        MemFlags::new(),
                        vars_ptr,
                        (*idx * 8) as i32,
                    );
                    stack.push(val);
                }
                Instruction::PushTemperature => {
                    // ctx->temperature is at offset 24
                    let val = builder.ins().load(types::F64, MemFlags::new(), ctx_ptr, 24);
                    stack.push(val);
                }
                Instruction::PushVt => {
                    // Vt = kT/q, compute from temperature
                    let temp = builder.ins().load(types::F64, MemFlags::new(), ctx_ptr, 24);
                    let k = builder.ins().f64const(1.380649e-23);
                    let q = builder.ins().f64const(1.602176634e-19);
                    let kt = builder.ins().fmul(k, temp);
                    let vt = builder.ins().fdiv(kt, q);
                    stack.push(vt);
                }
                Instruction::PushTime => {
                    // ctx->time is at offset 32
                    let val = builder.ins().load(types::F64, MemFlags::new(), ctx_ptr, 32);
                    stack.push(val);
                }
                Instruction::PushCurrent(_, _) => {
                    // Placeholder - currents need circuit solution
                    stack.push(builder.ins().f64const(0.0));
                }

                // Binary operations
                Instruction::Add => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(builder.ins().fadd(a, b));
                }
                Instruction::Sub => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(builder.ins().fsub(a, b));
                }
                Instruction::Mul => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(builder.ins().fmul(a, b));
                }
                Instruction::Div => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(builder.ins().fdiv(a, b));
                }
                Instruction::Pow | Instruction::FnPow => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let result = self.call_math2(builder, module, math_funcs, "pow", a, b)?;
                    stack.push(result);
                }

                // Unary operations
                Instruction::Neg => {
                    let a = stack.pop().unwrap();
                    stack.push(builder.ins().fneg(a));
                }
                Instruction::Abs => {
                    let a = stack.pop().unwrap();
                    let result = self.call_math1(builder, module, math_funcs, "fabs", a)?;
                    stack.push(result);
                }
                Instruction::Sqrt => {
                    let a = stack.pop().unwrap();
                    stack.push(builder.ins().sqrt(a));
                }
                Instruction::Exp => {
                    let a = stack.pop().unwrap();
                    let result = self.call_math1(builder, module, math_funcs, "exp", a)?;
                    stack.push(result);
                }
                Instruction::Limexp => {
                    // Limited exponential - for now just use exp
                    // TODO: Implement proper limexp
                    let a = stack.pop().unwrap();
                    let result = self.call_math1(builder, module, math_funcs, "exp", a)?;
                    stack.push(result);
                }
                Instruction::Log => {
                    let a = stack.pop().unwrap();
                    let result = self.call_math1(builder, module, math_funcs, "log", a)?;
                    stack.push(result);
                }
                Instruction::Log10 => {
                    let a = stack.pop().unwrap();
                    let result = self.call_math1(builder, module, math_funcs, "log10", a)?;
                    stack.push(result);
                }
                Instruction::Sin => {
                    let a = stack.pop().unwrap();
                    let result = self.call_math1(builder, module, math_funcs, "sin", a)?;
                    stack.push(result);
                }
                Instruction::Cos => {
                    let a = stack.pop().unwrap();
                    let result = self.call_math1(builder, module, math_funcs, "cos", a)?;
                    stack.push(result);
                }
                Instruction::Tan => {
                    let a = stack.pop().unwrap();
                    let result = self.call_math1(builder, module, math_funcs, "tan", a)?;
                    stack.push(result);
                }
                Instruction::Sinh => {
                    let a = stack.pop().unwrap();
                    let result = self.call_math1(builder, module, math_funcs, "sinh", a)?;
                    stack.push(result);
                }
                Instruction::Cosh => {
                    let a = stack.pop().unwrap();
                    let result = self.call_math1(builder, module, math_funcs, "cosh", a)?;
                    stack.push(result);
                }
                Instruction::Tanh => {
                    let a = stack.pop().unwrap();
                    let result = self.call_math1(builder, module, math_funcs, "tanh", a)?;
                    stack.push(result);
                }
                Instruction::Asin => {
                    let a = stack.pop().unwrap();
                    let result = self.call_math1(builder, module, math_funcs, "asin", a)?;
                    stack.push(result);
                }
                Instruction::Acos => {
                    let a = stack.pop().unwrap();
                    let result = self.call_math1(builder, module, math_funcs, "acos", a)?;
                    stack.push(result);
                }
                Instruction::Atan => {
                    let a = stack.pop().unwrap();
                    let result = self.call_math1(builder, module, math_funcs, "atan", a)?;
                    stack.push(result);
                }
                Instruction::Atan2 => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let result = self.call_math2(builder, module, math_funcs, "atan2", a, b)?;
                    stack.push(result);
                }
                Instruction::Floor => {
                    let a = stack.pop().unwrap();
                    let result = self.call_math1(builder, module, math_funcs, "floor", a)?;
                    stack.push(result);
                }
                Instruction::Ceil => {
                    let a = stack.pop().unwrap();
                    let result = self.call_math1(builder, module, math_funcs, "ceil", a)?;
                    stack.push(result);
                }
                Instruction::Min => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let result = self.call_math2(builder, module, math_funcs, "fmin", a, b)?;
                    stack.push(result);
                }
                Instruction::Max => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let result = self.call_math2(builder, module, math_funcs, "fmax", a, b)?;
                    stack.push(result);
                }

                // Comparison operations
                Instruction::Gt => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let cmp = builder.ins().fcmp(FloatCC::GreaterThan, a, b);
                    let one = builder.ins().f64const(1.0);
                    let zero = builder.ins().f64const(0.0);
                    let result = builder.ins().select(cmp, one, zero);
                    stack.push(result);
                }
                Instruction::Lt => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let cmp = builder.ins().fcmp(FloatCC::LessThan, a, b);
                    let one = builder.ins().f64const(1.0);
                    let zero = builder.ins().f64const(0.0);
                    let result = builder.ins().select(cmp, one, zero);
                    stack.push(result);
                }
                Instruction::Ge => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let cmp = builder.ins().fcmp(FloatCC::GreaterThanOrEqual, a, b);
                    let one = builder.ins().f64const(1.0);
                    let zero = builder.ins().f64const(0.0);
                    let result = builder.ins().select(cmp, one, zero);
                    stack.push(result);
                }
                Instruction::Le => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let cmp = builder.ins().fcmp(FloatCC::LessThanOrEqual, a, b);
                    let one = builder.ins().f64const(1.0);
                    let zero = builder.ins().f64const(0.0);
                    let result = builder.ins().select(cmp, one, zero);
                    stack.push(result);
                }
                Instruction::Eq => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let cmp = builder.ins().fcmp(FloatCC::Equal, a, b);
                    let one = builder.ins().f64const(1.0);
                    let zero = builder.ins().f64const(0.0);
                    let result = builder.ins().select(cmp, one, zero);
                    stack.push(result);
                }
                Instruction::Ne => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let cmp = builder.ins().fcmp(FloatCC::NotEqual, a, b);
                    let one = builder.ins().f64const(1.0);
                    let zero = builder.ins().f64const(0.0);
                    let result = builder.ins().select(cmp, one, zero);
                    stack.push(result);
                }

                // Logical operations
                Instruction::And => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let zero = builder.ins().f64const(0.0);
                    let one = builder.ins().f64const(1.0);
                    let a_true = builder.ins().fcmp(FloatCC::NotEqual, a, zero);
                    let b_true = builder.ins().fcmp(FloatCC::NotEqual, b, zero);
                    let both = builder.ins().band(a_true, b_true);
                    let result = builder.ins().select(both, one, zero);
                    stack.push(result);
                }
                Instruction::Or => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let zero = builder.ins().f64const(0.0);
                    let one = builder.ins().f64const(1.0);
                    let a_true = builder.ins().fcmp(FloatCC::NotEqual, a, zero);
                    let b_true = builder.ins().fcmp(FloatCC::NotEqual, b, zero);
                    let either = builder.ins().bor(a_true, b_true);
                    let result = builder.ins().select(either, one, zero);
                    stack.push(result);
                }
                Instruction::Not => {
                    let a = stack.pop().unwrap();
                    let zero = builder.ins().f64const(0.0);
                    let one = builder.ins().f64const(1.0);
                    let is_zero = builder.ins().fcmp(FloatCC::Equal, a, zero);
                    let result = builder.ins().select(is_zero, one, zero);
                    stack.push(result);
                }

                // Conditional
                Instruction::IfElse => {
                    let else_val = stack.pop().unwrap();
                    let then_val = stack.pop().unwrap();
                    let cond = stack.pop().unwrap();
                    let zero = builder.ins().f64const(0.0);
                    let is_true = builder.ins().fcmp(FloatCC::NotEqual, cond, zero);
                    let result = builder.ins().select(is_true, then_val, else_val);
                    stack.push(result);
                }

                // State operations (transient analysis)
                Instruction::DdtState(idx) | Instruction::IdtState(idx) => {
                    // For now, push 0 - proper implementation needs state tracking
                    let _ = idx;
                    let _ = stack.pop();
                    stack.push(builder.ins().f64const(0.0));
                }
            }
        }

        // Return the top of stack (or 0 if empty)
        Ok(stack.pop().unwrap_or_else(|| builder.ins().f64const(0.0)))
    }

    /// Call a single-argument math function
    fn call_math1(
        &self,
        builder: &mut FunctionBuilder,
        module: &mut JITModule,
        math_funcs: &HashMap<&'static str, FuncId>,
        name: &str,
        arg: Value,
    ) -> JitResult<Value> {
        let func_id = math_funcs
            .get(name)
            .ok_or_else(|| JitError::FunctionNotFound(name.to_string()))?;

        let func_ref = module.declare_func_in_func(*func_id, builder.func);
        let call = builder.ins().call(func_ref, &[arg]);
        Ok(builder.inst_results(call)[0])
    }

    /// Call a two-argument math function
    fn call_math2(
        &self,
        builder: &mut FunctionBuilder,
        module: &mut JITModule,
        math_funcs: &HashMap<&'static str, FuncId>,
        name: &str,
        arg1: Value,
        arg2: Value,
    ) -> JitResult<Value> {
        let func_id = math_funcs
            .get(name)
            .ok_or_else(|| JitError::FunctionNotFound(name.to_string()))?;

        let func_ref = module.declare_func_in_func(*func_id, builder.func);
        let call = builder.ins().call(func_ref, &[arg1, arg2]);
        Ok(builder.inst_results(call)[0])
    }
}

impl Default for JitCompiler {
    fn default() -> Self {
        Self::new().expect("Failed to create JIT compiler")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::{CompiledParameter, StampIndex, StampLocation};

    #[test]
    fn test_jit_compiler_creation() {
        let compiler = JitCompiler::new();
        assert!(compiler.is_ok(), "JIT compiler should initialize");
    }

    #[test]
    fn test_simple_model_compilation() {
        let model = CompiledModel {
            name: "test".into(),
            num_terminals: 2,
            terminal_names: vec!["p".into(), "n".into()],
            parameters: vec![CompiledParameter {
                name: "g".into(),
                default: 0.001,
                min: Some(0.0),
                max: None,
            }],
            num_variables: 0,
            assignment_programs: vec![],
            stamp_programs: vec![StampProgram {
                stamp_locations: vec![StampLocation {
                    row: StampIndex::Terminal(0),
                    col: StampIndex::Ground,
                    sign: 1.0,
                }],
                value_program: BytecodeProgram {
                    instructions: vec![
                        Instruction::PushParam(0),
                        Instruction::PushVoltage(0, 1),
                        Instruction::Mul,
                    ],
                },
                jacobian_programs: vec![],
            }],
            internal_nodes: 0,
            branch_currents: 0,
        };

        let compiler = JitCompiler::new().unwrap();
        let result = compiler.compile(&model);
        assert!(
            result.is_ok(),
            "Simple model should compile: {:?}",
            result.err()
        );
    }
}
