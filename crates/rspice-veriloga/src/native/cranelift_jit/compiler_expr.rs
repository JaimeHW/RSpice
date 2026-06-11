use super::*;

/// Whether every instruction of a program is compilable by
/// [`JitCompiler::compile_expression`]. Mirrors the bail arms below — kept
/// adjacent so the two stay in sync. A stale entry here only costs
/// performance: the chunk compiler degrades a refused chunk back to the
/// interpreter, never correctness.
pub(crate) fn program_is_jitable(program: &BytecodeProgram) -> bool {
    program.instructions.iter().all(|instr| {
        !matches!(
            instr,
            Instruction::PushVariableDyn { .. }
                | Instruction::IdtModState(_)
                | Instruction::TableDerivative(_)
                | Instruction::LimitState(_)
                | Instruction::AbsDelayState(_)
                | Instruction::TransitionState(_)
                | Instruction::SlewState(_)
                | Instruction::CrossState(_)
                | Instruction::TimerState(_)
                | Instruction::LaplaceState(_)
        )
    })
}

impl JitCompiler {
    /// Compile a bytecode program to Cranelift IR, returning the result value
    #[allow(clippy::too_many_arguments)]
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
                // Runtime-indexed reads need a bounds-error path the JIT
                // cannot report faithfully; the interpreter handles them
                Instruction::PushVariableDyn { .. } => {
                    return Err(JitError::UnsupportedInstruction("PushVariableDyn"));
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

                // ddt(): record the operand in the state slot, return
                // (operand - prev) / dt during transient and 0 at DC,
                // exactly mirroring the interpreter (same state arrays)
                Instruction::DdtState(idx) => {
                    let q = stack.pop().unwrap();
                    let state_ptr = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::trusted(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_STATE_VALUES,
                    );
                    builder
                        .ins()
                        .store(MemFlags::new(), q, state_ptr, (*idx * 8) as i32);
                    let prev = self.load_state_prev_or(builder, ctx_ptr, *idx, q);
                    let dt = builder.ins().load(
                        types::F64,
                        MemFlags::trusted(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_TIMESTEP,
                    );
                    let dt_live = Self::timestep_is_live(builder, dt);
                    let diff = builder.ins().fsub(q, prev);
                    let deriv = builder.ins().fdiv(diff, dt);
                    let zero = builder.ins().f64const(0.0);
                    let result = builder.ins().select(dt_live, deriv, zero);
                    stack.push(result);
                }
                // idt(): integral = prev + operand * dt during transient,
                // pinned to the initial condition at DC; the result seeds
                // the state slot either way
                Instruction::IdtState(idx) => {
                    let ic = stack.pop().unwrap();
                    let q = stack.pop().unwrap();
                    let prev = self.load_state_prev_or(builder, ctx_ptr, *idx, ic);
                    let dt = builder.ins().load(
                        types::F64,
                        MemFlags::trusted(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_TIMESTEP,
                    );
                    let dt_live = Self::timestep_is_live(builder, dt);
                    let step = builder.ins().fmul(q, dt);
                    let advanced = builder.ins().fadd(prev, step);
                    let result = builder.ins().select(dt_live, advanced, ic);
                    let state_ptr = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::trusted(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_STATE_VALUES,
                    );
                    builder
                        .ins()
                        .store(MemFlags::new(), result, state_ptr, (*idx * 8) as i32);
                    stack.push(result);
                }
                Instruction::IdtModState(_) => {
                    return Err(JitError::UnsupportedInstruction("IdtModState"));
                }
                // Companion Jacobian factors: operand / dt (ddt) or
                // operand * dt (idt) during transient, 0 at DC
                Instruction::DdtJacobian | Instruction::IdtJacobian => {
                    let a = stack.pop().unwrap();
                    let dt = builder.ins().load(
                        types::F64,
                        MemFlags::trusted(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_TIMESTEP,
                    );
                    let dt_live = Self::timestep_is_live(builder, dt);
                    let factor = match instr {
                        Instruction::DdtJacobian => builder.ins().fdiv(a, dt),
                        _ => builder.ins().fmul(a, dt),
                    };
                    let zero = builder.ins().f64const(0.0);
                    let result = builder.ins().select(dt_live, factor, zero);
                    stack.push(result);
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

                // analysis("..."): compare the context's analysis type code
                // (0=dc, 1=ac, 2=tran, 3=noise, 4=ic; 5 queries "static" =
                // dc or ic), mirroring the interpreter's table
                Instruction::Analysis(query_id) => {
                    let current = builder.ins().load(
                        types::I8,
                        MemFlags::new(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_ANALYSIS_TYPE,
                    );
                    let current = builder.ins().uextend(types::I32, current);
                    let one = builder.ins().f64const(1.0);
                    let zero = builder.ins().f64const(0.0);
                    let result = match *query_id {
                        0..=4 => {
                            let cmp =
                                builder.ins().icmp_imm(IntCC::Equal, current, *query_id as i64);
                            builder.ins().select(cmp, one, zero)
                        }
                        5 => {
                            let is_dc = builder.ins().icmp_imm(IntCC::Equal, current, 0);
                            let is_ic = builder.ins().icmp_imm(IntCC::Equal, current, 4);
                            let either = builder.ins().bor(is_dc, is_ic);
                            builder.ins().select(either, one, zero)
                        }
                        _ => zero,
                    };
                    stack.push(result);
                }

                // $param_given: one byte per parameter in the context
                Instruction::PushParamGiven(idx) => {
                    let flags_ptr = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::new(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_PARAM_GIVEN,
                    );
                    let flag =
                        builder
                            .ins()
                            .load(types::I8, MemFlags::new(), flags_ptr, *idx as i32);
                    let cmp = builder.ins().icmp_imm(IntCC::NotEqual, flag, 0);
                    let one = builder.ins().f64const(1.0);
                    let zero = builder.ins().f64const(0.0);
                    let result = builder.ins().select(cmp, one, zero);
                    stack.push(result);
                }

                // Branch-current unknown values (the device sizes the array
                // to the model's branch unknown count before native calls)
                Instruction::PushBranchCurrent(k) => {
                    let unknowns_ptr = builder.ins().load(
                        self.isa.pointer_type(),
                        MemFlags::new(),
                        ctx_ptr,
                        EVAL_CTX_OFFSET_BRANCH_UNKNOWNS,
                    );
                    let val = builder.ins().load(
                        types::F64,
                        MemFlags::new(),
                        unknowns_ptr,
                        (*k * 8) as i32,
                    );
                    stack.push(val);
                }

                // Modulus follows fmod semantics on reals (LRM 4.2.3)
                Instruction::Mod => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let result = self.call_math2(builder, module, math_funcs, "fmod", a, b)?;
                    stack.push(result);
                }
                // Bitwise/shift operations truncate the operands to i64
                // (saturating, matching Rust `as i64` in the interpreter)
                // and convert the result back
                Instruction::Shl
                | Instruction::Shr
                | Instruction::BitAnd
                | Instruction::BitOr
                | Instruction::BitXor => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let ia = builder.ins().fcvt_to_sint_sat(types::I64, a);
                    let ib = builder.ins().fcvt_to_sint_sat(types::I64, b);
                    let res = match instr {
                        Instruction::Shl => builder.ins().ishl(ia, ib),
                        Instruction::Shr => builder.ins().sshr(ia, ib),
                        Instruction::BitAnd => builder.ins().band(ia, ib),
                        Instruction::BitOr => builder.ins().bor(ia, ib),
                        Instruction::BitXor => builder.ins().bxor(ia, ib),
                        _ => unreachable!(),
                    };
                    let result = builder.ins().fcvt_from_sint(types::F64, res);
                    stack.push(result);
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

    /// `|dt| > 1e-20`: whether a transient timestep is active (matches the
    /// interpreter's DC gate on stateful operators)
    fn timestep_is_live(builder: &mut FunctionBuilder, dt: Value) -> Value {
        let dt_abs = builder.ins().fabs(dt);
        let tiny = builder.ins().f64const(1e-20);
        builder.ins().fcmp(FloatCC::GreaterThan, dt_abs, tiny)
    }

    /// Load `state_prev[idx]`, falling back to `fallback` when no previous
    /// state exists yet (null pointer before the first accepted step) —
    /// mirroring the interpreter's `.get(idx).unwrap_or(fallback)`
    fn load_state_prev_or(
        &self,
        builder: &mut FunctionBuilder,
        ctx_ptr: Value,
        idx: usize,
        fallback: Value,
    ) -> Value {
        let prev_ptr = builder.ins().load(
            self.isa.pointer_type(),
            MemFlags::trusted(),
            ctx_ptr,
            EVAL_CTX_OFFSET_STATE_PREV,
        );
        let null = builder.ins().iconst(self.isa.pointer_type(), 0);
        let is_null = builder.ins().icmp(IntCC::Equal, prev_ptr, null);

        let load_block = builder.create_block();
        let merge_block = builder.create_block();
        builder.append_block_param(merge_block, types::F64);

        builder
            .ins()
            .brif(is_null, merge_block, &[fallback], load_block, &[]);

        builder.switch_to_block(load_block);
        builder.seal_block(load_block);
        let loaded =
            builder
                .ins()
                .load(types::F64, MemFlags::new(), prev_ptr, (idx * 8) as i32);
        builder.ins().jump(merge_block, &[loaded]);

        builder.switch_to_block(merge_block);
        builder.seal_block(merge_block);
        builder.block_params(merge_block)[0]
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
