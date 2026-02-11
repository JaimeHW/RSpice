use super::*;

impl JitCompiler {
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

}
