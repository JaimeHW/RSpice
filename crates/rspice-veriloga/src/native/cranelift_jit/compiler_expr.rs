use super::*;

impl JitCompiler {
    /// Compile a bytecode program to Cranelift IR, returning the result value
    pub(super) fn compile_expression(
        &self,
        builder: &mut FunctionBuilder,
        program: &BytecodeProgram,
        ctx_ptr: Value,
        vars_ptr: Value,
        module: &mut JITModule,
        math_funcs: &HashMap<&'static str, FuncId>,
        num_terminals: usize,
    ) -> JitResult<Value> {
        let mut stack: Vec<Value> = Vec::new();

        // Load the potential of a unified node index (terminals first, then
        // internal nodes; usize::MAX is ground). The dispatch happens at
        // compile time since node indices are constants.
        let load_node_potential = |builder: &mut FunctionBuilder, node: usize| -> Value {
            if node == usize::MAX {
                builder.ins().f64const(0.0)
            } else if node < num_terminals {
                let voltages_ptr = builder.ins().load(
                    self.isa.pointer_type(),
                    MemFlags::new(),
                    ctx_ptr,
                    EVAL_CTX_OFFSET_VOLTAGES,
                );
                builder
                    .ins()
                    .load(types::F64, MemFlags::new(), voltages_ptr, (node * 8) as i32)
            } else {
                let internal_ptr = builder.ins().load(
                    self.isa.pointer_type(),
                    MemFlags::new(),
                    ctx_ptr,
                    EVAL_CTX_OFFSET_INTERNAL_VOLTAGES,
                );
                builder.ins().load(
                    types::F64,
                    MemFlags::new(),
                    internal_ptr,
                    ((node - num_terminals) * 8) as i32,
                )
            }
        };

        for instr in &program.instructions {
            match instr {
                Instruction::PushConst(v) => {
                    let val = builder.ins().f64const(*v);
                    stack.push(val);
                }
                Instruction::PushParam(idx) => {
                    let params_ptr = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::new(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_PARAMS,
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
                    let v_pos = load_node_potential(builder, *pos);
                    let v_neg = load_node_potential(builder, *neg);
                    let diff = builder.ins().fsub(v_pos, v_neg);
                    stack.push(diff);
                }
                Instruction::PushInternalVoltage(idx) => {
                    let internal_ptr = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::new(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_INTERNAL_VOLTAGES,
                    );
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
                    let val = builder.ins().load(
                        types::F64,
                        MemFlags::new(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_TEMPERATURE,
                    );
                    stack.push(val);
                }
                Instruction::PushVt => {
                    // Vt = kT/q, compute from temperature
                    let temp = builder.ins().load(
                        types::F64,
                        MemFlags::new(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_TEMPERATURE,
                    );
                    let k = builder.ins().f64const(1.380649e-23);
                    let q = builder.ins().f64const(1.602176634e-19);
                    let kt = builder.ins().fmul(k, temp);
                    let vt = builder.ins().fdiv(kt, q);
                    stack.push(vt);
                }
                Instruction::PushTime => {
                    let val = builder.ins().load(
                        types::F64,
                        MemFlags::new(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_TIME,
                    );
                    stack.push(val);
                }
                Instruction::PushCurrent(pos, neg) => {
                    let branch_currents_ptr = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::trusted(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_BRANCH_CURRENTS,
                    );
                    let branch_currents_len = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::trusted(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_BRANCH_CURRENTS_LEN,
                    );
                    let currents_ptr = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::trusted(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_CURRENTS,
                    );
                    let currents_len = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::trusted(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_CURRENTS_LEN,
                    );
                    let num_terminals = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::trusted(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_NUM_TERMINALS,
                    );
                    let pos_idx = builder.ins().iconst(self.isa.pointer_type(), *pos as i64);
                    let neg_idx = builder.ins().iconst(self.isa.pointer_type(), *neg as i64);

                    let func_id = math_funcs.get("rspice_current_lookup").ok_or_else(|| {
                        JitError::FunctionNotFound("rspice_current_lookup".to_string())
                    })?;
                    let func_ref = module.declare_func_in_func(*func_id, builder.func);
                    let call = builder.ins().call(
                        func_ref,
                        &[
                            branch_currents_ptr,
                            branch_currents_len,
                            currents_ptr,
                            currents_len,
                            num_terminals,
                            pos_idx,
                            neg_idx,
                        ],
                    );
                    let result = builder.inst_results(call)[0];
                    stack.push(result);
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
                    // Limited exponential - prevents overflow for large inputs
                    // Uses linear extrapolation beyond exp(40) for numerical stability
                    let a = stack.pop().unwrap();
                    let result =
                        self.call_math1(builder, module, math_funcs, "rspice_limexp", a)?;
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

                // Stateful operators mutate per-instance history; compiling
                // them with DC-only semantics silently diverged from the
                // interpreter in transient analysis. Refuse instead - the
                // device falls back to the bytecode interpreter.
                Instruction::DdtState(_) => {
                    return Err(JitError::UnsupportedInstruction("DdtState"));
                }
                Instruction::IdtState(_) => {
                    return Err(JitError::UnsupportedInstruction("IdtState"));
                }
                Instruction::DdtJacobian => {
                    return Err(JitError::UnsupportedInstruction("DdtJacobian"));
                }
                Instruction::IdtJacobian => {
                    return Err(JitError::UnsupportedInstruction("IdtJacobian"));
                }
                Instruction::TableDerivative(_) => {
                    return Err(JitError::UnsupportedInstruction("TableDerivative"));
                }
                Instruction::LimitState(_) => {
                    return Err(JitError::UnsupportedInstruction("LimitState"));
                }

                // TableLookup: call rspice_table_lookup helper function
                // Signature: fn(tables_ptr, tables_len, table_id, input) -> f64
                Instruction::TableLookup(table_id) => {
                    let input = stack.pop().unwrap();

                    let tables_ptr = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::trusted(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_LOOKUP_TABLES,
                    );

                    let tables_len = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::trusted(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_LOOKUP_TABLES_LEN,
                    );

                    // Create table_id as pointer-sized integer
                    let table_idx = builder
                        .ins()
                        .iconst(self.isa.pointer_type(), *table_id as i64);

                    // Call rspice_table_lookup helper
                    let func_id = math_funcs.get("rspice_table_lookup").ok_or_else(|| {
                        JitError::FunctionNotFound("rspice_table_lookup".to_string())
                    })?;
                    let func_ref = module.declare_func_in_func(*func_id, builder.func);
                    let call = builder
                        .ins()
                        .call(func_ref, &[tables_ptr, tables_len, table_idx, input]);
                    let result = builder.inst_results(call)[0];
                    stack.push(result);
                }

                // Stateful filters and detectors require per-instance
                // history; refuse so the interpreter handles them
                Instruction::AbsDelayState(_) => {
                    return Err(JitError::UnsupportedInstruction("AbsDelayState"));
                }
                Instruction::TransitionState(_) => {
                    return Err(JitError::UnsupportedInstruction("TransitionState"));
                }
                Instruction::SlewState(_) => {
                    return Err(JitError::UnsupportedInstruction("SlewState"));
                }
                Instruction::CrossState(_) => {
                    return Err(JitError::UnsupportedInstruction("CrossState"));
                }

                // WhiteNoise: noise source
                // In time domain, returns 0
                Instruction::WhiteNoise => {
                    let _power = stack.pop().unwrap();
                    stack.push(builder.ins().f64const(0.0));
                }

                // FlickerNoise: 1/f noise
                // In time domain, returns 0
                Instruction::FlickerNoise => {
                    let _exponent = stack.pop().unwrap();
                    let _power = stack.pop().unwrap();
                    stack.push(builder.ins().f64const(0.0));
                }

                // The evaluation context does not expose the analysis type
                // to native code yet; assuming DC silently diverged from the
                // interpreter during transient analysis
                Instruction::Analysis(_) => {
                    return Err(JitError::UnsupportedInstruction("Analysis"));
                }

                // AboveState: level crossing event
                // In DC, compare value > threshold
                Instruction::AboveState(_detector_id) => {
                    let threshold = stack.pop().unwrap();
                    let value = stack.pop().unwrap();
                    // Compare value > threshold
                    let cmp = builder.ins().fcmp(FloatCC::GreaterThan, value, threshold);
                    let one = builder.ins().f64const(1.0);
                    let zero = builder.ins().f64const(0.0);
                    let result = builder.ins().select(cmp, one, zero);
                    stack.push(result);
                }

                // TimerState depends on simulation time bookkeeping
                Instruction::TimerState(_) => {
                    return Err(JitError::UnsupportedInstruction("TimerState"));
                }

                // Laplace filters carry per-instance integration state that
                // the native path cannot manage consistently with the
                // interpreter (the helper mutated the shared model filters)
                Instruction::LaplaceState(_) => {
                    return Err(JitError::UnsupportedInstruction("LaplaceState"));
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
