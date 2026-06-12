use super::*;

impl JitCompiler {
    /// Compile a contiguous run of plain assignments into one function:
    /// fn(ctx: *const EvalContext, vars: *mut f64)
    pub(super) fn compile_assignment_chunk(
        &self,
        module: &mut JITModule,
        ctx: &mut cranelift::prelude::codegen::Context,
        model: &CompiledModel,
        chunk: &[&crate::codegen::AssignmentProgram],
        chunk_id: usize,
        math_funcs: &HashMap<&'static str, FuncId>,
    ) -> JitResult<FuncId> {
        let ptr_type = self.isa.pointer_type();

        let mut sig = module.make_signature();
        sig.params.push(AbiParam::new(ptr_type)); // ctx
        sig.params.push(AbiParam::new(ptr_type)); // vars

        let func_id = module
            .declare_function(
                &format!("{}_chunk_{}", model.name, chunk_id),
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

        for assign in chunk {
            let value = match self.compile_expression(
                &mut builder,
                &assign.program,
                ctx_ptr,
                vars_ptr,
                module,
                math_funcs,
                model.num_terminals,
            ) {
                Ok(value) => value,
                Err(err) => {
                    // Reset the partial function so the context can be
                    // reused for the next chunk
                    ctx.clear();
                    return Err(err);
                }
            };

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

    /// Compile a single value-returning program:
    /// fn(ctx: *const EvalContext, vars: *const f64) -> f64
    pub(super) fn compile_program(
        &self,
        module: &mut JITModule,
        ctx: &mut cranelift::prelude::codegen::Context,
        model: &CompiledModel,
        program: &BytecodeProgram,
        name: &str,
        math_funcs: &HashMap<&'static str, FuncId>,
    ) -> JitResult<FuncId> {
        let ptr_type = self.isa.pointer_type();

        let mut sig = module.make_signature();
        sig.params.push(AbiParam::new(ptr_type)); // ctx
        sig.params.push(AbiParam::new(ptr_type)); // vars
        sig.returns.push(AbiParam::new(types::F64));

        let func_id = module
            .declare_function(name, Linkage::Local, &sig)
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

        let value = match self.compile_expression(
            &mut builder,
            program,
            ctx_ptr,
            vars_ptr,
            module,
            math_funcs,
            model.num_terminals,
        ) {
            Ok(value) => value,
            Err(err) => {
                ctx.clear();
                return Err(err);
            }
        };

        builder.ins().return_(&[value]);
        builder.finalize();

        module
            .define_function(func_id, ctx)
            .map_err(|e| JitError::Module(e.to_string()))?;

        ctx.clear();
        Ok(func_id)
    }
}
