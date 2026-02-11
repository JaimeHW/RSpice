use super::*;

impl JitCompiler {
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
                    let voltages_ptr = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::new(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_VOLTAGES,
                    );
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

                // State operations (transient analysis)
                Instruction::DdtState(_idx) | Instruction::IdtState(_idx) => {
                    // For now, push 0 - proper ddt/idt implementation needs state tracking
                    // This is a fallback for DC analysis where ddt/idt return 0
                    let _ = stack.pop();
                    stack.push(builder.ins().f64const(0.0));
                }

                // LimitState: call rspice_limit helper function
                // Signature: fn(state_prev, state_idx, new_value, step_limit) -> f64
                Instruction::LimitState(idx) => {
                    let step_limit = stack.pop().unwrap();
                    let new_value = stack.pop().unwrap();

                    let state_prev = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::trusted(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_STATE_PREV,
                    );

                    // Create state index as pointer-sized integer
                    let state_idx = builder.ins().iconst(self.isa.pointer_type(), *idx as i64);

                    // Call rspice_limit helper
                    let func_id = math_funcs
                        .get("rspice_limit")
                        .ok_or_else(|| JitError::FunctionNotFound("rspice_limit".to_string()))?;
                    let func_ref = module.declare_func_in_func(*func_id, builder.func);
                    let call = builder
                        .ins()
                        .call(func_ref, &[state_prev, state_idx, new_value, step_limit]);
                    let result = builder.inst_results(call)[0];
                    stack.push(result);
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

                // AbsDelayState: transport delay
                // For DC/JIT, delay is typically not applicable - return current value
                // Full transient delay would need buffer pointer in context
                Instruction::AbsDelayState(_buffer_id) => {
                    let _delay_time = stack.pop().unwrap();
                    let current_value = stack.pop().unwrap();
                    // In DC analysis, absdelay returns current value
                    stack.push(current_value);
                }

                // TransitionState: piecewise-linear smoothing
                // In DC/JIT, transition is instantaneous - return input
                Instruction::TransitionState(_filter_id) => {
                    let _fall_time = stack.pop().unwrap();
                    let _rise_time = stack.pop().unwrap();
                    let _delay = stack.pop().unwrap();
                    let input = stack.pop().unwrap();
                    // In DC, transition returns input
                    stack.push(input);
                }

                // SlewState: slew rate limiting
                // In DC/JIT, slew is instantaneous - return input
                Instruction::SlewState(_filter_id) => {
                    let _max_neg_slew = stack.pop().unwrap();
                    let _max_pos_slew = stack.pop().unwrap();
                    let input = stack.pop().unwrap();
                    // In DC, slew returns input
                    stack.push(input);
                }

                // CrossState: threshold crossing detection
                // In DC, cross never fires - return 0
                Instruction::CrossState(_detector_id) => {
                    let _direction = stack.pop().unwrap();
                    let _value = stack.pop().unwrap();
                    // DC: no crossing events
                    stack.push(builder.ins().f64const(0.0));
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

                // Analysis: check current analysis type
                // In JIT, we assume DC analysis by default (return 1 for dc check, 0 others)
                Instruction::Analysis(analysis_str_id) => {
                    // For JIT DC analysis: dc=1, ac=0, tran=0
                    let result = if *analysis_str_id == 0 {
                        1.0 // DC check returns true
                    } else {
                        0.0 // Non-DC checks return false
                    };
                    stack.push(builder.ins().f64const(result));
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

                // TimerState: periodic timer
                // In DC, timer never fires - return 0
                Instruction::TimerState(_timer_id) => {
                    let _period = stack.pop().unwrap();
                    let _start_time = stack.pop().unwrap();
                    // DC: no timer events
                    stack.push(builder.ins().f64const(0.0));
                }

                // LaplaceState: Laplace state-space filter step
                // Calls rspice_laplace_step(filters_ptr, filters_len, filter_id, input, timestep)
                Instruction::LaplaceState(filter_id) => {
                    let input = stack.pop().unwrap();

                    let filters_ptr = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::new(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_LAPLACE_FILTERS,
                    );
                    let filters_len = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::new(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_LAPLACE_FILTERS_LEN,
                    );

                    let timestep = builder.ins().load(
                        types::F64,
                        MemFlags::new(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_TIMESTEP,
                    );

                    // Create filter_id as a constant
                    let filter_idx = builder
                        .ins()
                        .iconst(self.isa.pointer_type(), *filter_id as i64);

                    // Call rspice_laplace_step(filters_ptr, filters_len, filter_id, input, timestep)
                    let func_id = math_funcs.get("rspice_laplace_step").ok_or_else(|| {
                        JitError::FunctionNotFound("rspice_laplace_step".to_string())
                    })?;
                    let func_ref = module.declare_func_in_func(*func_id, builder.func);
                    let call = builder.ins().call(
                        func_ref,
                        &[filters_ptr, filters_len, filter_idx, input, timestep],
                    );
                    let result = builder.inst_results(call)[0];
                    stack.push(result);
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
}
