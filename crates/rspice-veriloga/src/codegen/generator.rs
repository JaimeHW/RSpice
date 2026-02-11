use super::*;

impl<'a> CodeGenerator<'a> {
    /// Create a new code generator
    pub fn new(options: &'a CompilerOptions) -> Self {
        Self {
            options,
            laplace_filters: std::cell::RefCell::new(Vec::new()),
            lookup_tables: std::cell::RefCell::new(Vec::new()),
            limit_state_count: std::cell::Cell::new(0),
            delay_buffer_count: std::cell::Cell::new(0),
            transition_filter_count: std::cell::Cell::new(0),
            slew_filter_count: std::cell::Cell::new(0),
            cross_detector_count: std::cell::Cell::new(0),
            above_detector_count: std::cell::Cell::new(0),
            timer_state_count: std::cell::Cell::new(0),
        }
    }

    /// Generate compiled model from analyzed file
    pub fn generate(&self, analyzed: &AnalyzedFile) -> CompileResult<CompiledModel> {
        // Get the first module (for now, single module per file)
        let module = analyzed.modules.values().next().ok_or_else(|| {
            CodeGenError::new(CodeGenErrorKind::Internal("No modules found".into()))
        })?;

        // Build IR
        let ir = DeviceIR::from_analyzed(module);

        // Generate code from IR
        self.generate_from_ir(&ir)
    }

    /// Generate from IR
    fn generate_from_ir(&self, ir: &DeviceIR) -> CompileResult<CompiledModel> {
        self.lookup_tables.borrow_mut().clear();
        self.laplace_filters.borrow_mut().clear();
        self.limit_state_count.set(0);
        self.delay_buffer_count.set(0);
        self.transition_filter_count.set(0);
        self.slew_filter_count.set(0);
        self.cross_detector_count.set(0);
        self.above_detector_count.set(0);
        self.timer_state_count.set(0);

        let mut model = CompiledModel {
            name: ir.name.clone(),
            num_terminals: ir.terminals.len(),
            terminal_names: ir.terminals.iter().map(|t| t.name.clone()).collect(),
            parameters: ir
                .parameters
                .iter()
                .map(|p| CompiledParameter {
                    name: p.name.clone(),
                    default: p.default,
                    min: p.min,
                    max: p.max,
                })
                .collect(),
            num_variables: ir.variables.len(),
            assignment_programs: Vec::new(),
            stamp_programs: Vec::new(),
            lookup_tables: Vec::new(),
            internal_nodes: ir.internal_nodes.len(),
            branch_currents: 0,
            laplace_filters: Vec::new(),
        };

        // Generate assignment programs (executed in order before contributions)
        for assign in &ir.assignments {
            let program = self.compile_expr(&assign.expr, ir)?;
            model.assignment_programs.push(AssignmentProgram {
                var_index: assign.var_index,
                program,
            });
        }

        // Generate stamp programs for each equation
        for eq in &ir.equations {
            let program = self.compile_equation(eq, ir)?;
            model.stamp_programs.push(program);
        }

        model.laplace_filters = self.laplace_filters.take();
        model.lookup_tables = self.lookup_tables.take();

        Ok(model)
    }

    /// Compile a branch equation to a stamp program
    fn compile_equation(&self, eq: &BranchEquation, ir: &DeviceIR) -> CompileResult<StampProgram> {
        let value_program = self.compile_expr(&eq.expr, ir)?;

        let mut jacobian_programs = Vec::new();
        for deriv in &eq.derivatives {
            let program = self.compile_expr(&deriv.expr, ir)?;
            let (row, col) = self.derivative_indices(&eq.branch, &deriv.wrt, eq.is_current);
            jacobian_programs.push(JacobianEntry { row, col, program });
        }

        // Build stamp locations for the contribution
        let pos = eq.branch.pos_terminal;
        let neg = eq.branch.neg_terminal;

        let stamp_locations = if eq.is_current {
            // Current contribution: stamps to RHS at pos and neg
            vec![
                StampLocation {
                    row: StampIndex::Terminal(pos),
                    col: StampIndex::Ground,
                    sign: -1.0,
                },
                StampLocation {
                    row: StampIndex::Terminal(neg),
                    col: StampIndex::Ground,
                    sign: 1.0,
                },
            ]
        } else {
            // Voltage contribution would need branch equation
            vec![]
        };

        Ok(StampProgram {
            stamp_locations,
            value_program,
            jacobian_programs,
        })
    }

    /// Get row/col for a derivative
    fn derivative_indices(
        &self,
        branch: &crate::ir::BranchRef,
        wrt: &DerivativeWrt,
        is_current: bool,
    ) -> (StampIndex, StampIndex) {
        match wrt {
            DerivativeWrt::Voltage(node) => {
                if is_current {
                    (
                        StampIndex::Terminal(branch.pos_terminal),
                        StampIndex::Terminal(*node),
                    )
                } else {
                    (StampIndex::Internal(0), StampIndex::Terminal(*node))
                }
            }
            DerivativeWrt::Current(p, _n) => (StampIndex::Terminal(*p), StampIndex::Internal(0)),
            DerivativeWrt::Time => (StampIndex::Ground, StampIndex::Ground),
        }
    }

    /// Compile an IR expression to bytecode
    pub(super) fn compile_expr(
        &self,
        expr: &IrExpr,
        ir: &DeviceIR,
    ) -> CompileResult<BytecodeProgram> {
        let mut program = BytecodeProgram::default();
        self.emit_expr(expr, ir, &mut program)?;
        Ok(program)
    }

    #[inline]
    fn allocate_slot(counter: &std::cell::Cell<usize>) -> usize {
        let id = counter.get();
        counter.set(id + 1);
        id
    }

    /// Emit bytecode for an expression
    fn emit_expr(
        &self,
        expr: &IrExpr,
        ir: &DeviceIR,
        program: &mut BytecodeProgram,
    ) -> CompileResult<()> {
        match expr {
            IrExpr::Const(v) => {
                program.instructions.push(Instruction::PushConst(*v));
            }
            IrExpr::Param(name) => {
                let idx = ir
                    .parameters
                    .iter()
                    .position(|p| &p.name == name)
                    .ok_or_else(|| {
                        CodeGenError::new(CodeGenErrorKind::Internal(format!(
                            "Unknown parameter: {}",
                            name
                        )))
                    })?;
                program.instructions.push(Instruction::PushParam(idx));
            }
            IrExpr::Var(name) => {
                let idx = ir
                    .variables
                    .iter()
                    .position(|v| &v.name == name)
                    .ok_or_else(|| {
                        CodeGenError::new(CodeGenErrorKind::Internal(format!(
                            "Unknown variable: {}",
                            name
                        )))
                    })?;
                program.instructions.push(Instruction::PushVariable(idx));
            }
            IrExpr::Voltage(p, n) => {
                program.instructions.push(Instruction::PushVoltage(*p, *n));
            }
            IrExpr::Current(p, n) => {
                program.instructions.push(Instruction::PushCurrent(*p, *n));
            }
            IrExpr::Temperature => {
                program.instructions.push(Instruction::PushTemperature);
            }
            IrExpr::Vt => {
                program.instructions.push(Instruction::PushVt);
            }
            IrExpr::Time => {
                program.instructions.push(Instruction::PushTime);
            }
            IrExpr::Binary(op, left, right) => {
                self.emit_expr(left, ir, program)?;
                self.emit_expr(right, ir, program)?;
                program.instructions.push(match op {
                    // Arithmetic
                    BinaryOp::Add => Instruction::Add,
                    BinaryOp::Sub => Instruction::Sub,
                    BinaryOp::Mul => Instruction::Mul,
                    BinaryOp::Div => Instruction::Div,
                    BinaryOp::Pow => Instruction::Pow,
                    // Comparisons
                    BinaryOp::Gt => Instruction::Gt,
                    BinaryOp::Lt => Instruction::Lt,
                    BinaryOp::Ge => Instruction::Ge,
                    BinaryOp::Le => Instruction::Le,
                    BinaryOp::Eq => Instruction::Eq,
                    BinaryOp::Ne => Instruction::Ne,
                    // Logical
                    BinaryOp::And => Instruction::And,
                    BinaryOp::Or => Instruction::Or,
                    _ => {
                        return Err(CompileError::CodeGen(CodeGenError::new(
                            CodeGenErrorKind::UnsupportedFeature(format!("Binary op {:?}", op)),
                        )));
                    }
                });
            }
            IrExpr::Unary(crate::ast::UnaryOp::Neg, inner) => {
                self.emit_expr(inner, ir, program)?;
                program.instructions.push(Instruction::Neg);
            }
            IrExpr::Call(func, args) => {
                for arg in args {
                    self.emit_expr(arg, ir, program)?;
                }
                program.instructions.push(match func {
                    IrFunction::Abs => Instruction::Abs,
                    IrFunction::Sqrt => Instruction::Sqrt,
                    IrFunction::Exp => Instruction::Exp,
                    IrFunction::Log => Instruction::Log,
                    IrFunction::Log10 => Instruction::Log10,
                    IrFunction::Sin => Instruction::Sin,
                    IrFunction::Cos => Instruction::Cos,
                    IrFunction::Tan => Instruction::Tan,
                    IrFunction::Sinh => Instruction::Sinh,
                    IrFunction::Cosh => Instruction::Cosh,
                    IrFunction::Tanh => Instruction::Tanh,
                    IrFunction::Min => Instruction::Min,
                    IrFunction::Max => Instruction::Max,
                    // Inverse trig
                    IrFunction::Asin => Instruction::Asin,
                    IrFunction::Acos => Instruction::Acos,
                    IrFunction::Atan => Instruction::Atan,
                    IrFunction::Atan2 => Instruction::Atan2,
                    // Rounding
                    IrFunction::Floor => Instruction::Floor,
                    IrFunction::Ceil => Instruction::Ceil,
                    // Power
                    IrFunction::Pow => Instruction::FnPow,
                });
            }
            IrExpr::Limexp(inner) => {
                self.emit_expr(inner, ir, program)?;
                program.instructions.push(Instruction::Limexp);
            }
            IrExpr::Conditional(cond, then_expr, else_expr) => {
                self.emit_expr(cond, ir, program)?;
                self.emit_expr(then_expr, ir, program)?;
                self.emit_expr(else_expr, ir, program)?;
                program.instructions.push(Instruction::IfElse);
            }
            IrExpr::Unary(crate::ast::UnaryOp::Not, inner) => {
                self.emit_expr(inner, ir, program)?;
                program.instructions.push(Instruction::Not);
            }
            IrExpr::Ddt(inner) => {
                // For DC analysis, ddt = 0. For transient, would need state tracking.
                // For now, emit 0 for DC compatibility
                let _ = inner; // Mark as intentionally unused for now
                program.instructions.push(Instruction::PushConst(0.0));
            }
            IrExpr::Idt(inner, ic) => {
                // For DC analysis, idt behavior depends on context
                // For now, use initial condition if provided, else 0
                if let Some(ic_expr) = ic {
                    self.emit_expr(ic_expr, ir, program)?;
                } else {
                    let _ = inner; // Mark as intentionally unused for now
                    program.instructions.push(Instruction::PushConst(0.0));
                }
            }
            IrExpr::Limit(inner, step) => {
                // $limit(expr, step) - bounds value change per Newton iteration
                // For DC, we track previous value and limit the step
                self.emit_expr(inner, ir, program)?;
                if let Some(step_expr) = step {
                    self.emit_expr(step_expr, ir, program)?;
                } else {
                    // Default step limit for pn-junction type limiting
                    program.instructions.push(Instruction::PushConst(0.7)); // ~2*Vt
                }
                let state_id = Self::allocate_slot(&self.limit_state_count);
                program.instructions.push(Instruction::LimitState(state_id));
            }
            IrExpr::TableLookup {
                input,
                x_data,
                y_data,
            } => {
                // $table_model lookup with linear interpolation
                // Emit input expression, then TableLookup instruction referencing the table
                self.emit_expr(input, ir, program)?;
                let table_id = self.register_lookup_table(x_data, y_data)?;
                program
                    .instructions
                    .push(Instruction::TableLookup(table_id));
            }
            IrExpr::AbsDelay { expr, delay_time } => {
                // absdelay(expr, delay_time) - transport delay
                // Emit expression value, then delay time, then AbsDelayState instruction
                self.emit_expr(expr, ir, program)?;
                self.emit_expr(delay_time, ir, program)?;
                let buffer_id = Self::allocate_slot(&self.delay_buffer_count);
                program
                    .instructions
                    .push(Instruction::AbsDelayState(buffer_id));
            }
            IrExpr::Transition {
                expr,
                delay,
                rise_time,
                fall_time,
            } => {
                // transition(expr, delay, rise_time, fall_time)
                self.emit_expr(expr, ir, program)?;
                // Emit delay (default 0)
                if let Some(d) = delay {
                    self.emit_expr(d, ir, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                // Emit rise_time (default 0 = instantaneous)
                if let Some(r) = rise_time {
                    self.emit_expr(r, ir, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                // Emit fall_time (default to rise_time)
                if let Some(f) = fall_time {
                    self.emit_expr(f, ir, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                let filter_id = Self::allocate_slot(&self.transition_filter_count);
                program
                    .instructions
                    .push(Instruction::TransitionState(filter_id));
            }
            IrExpr::Slew {
                expr,
                max_pos_slew,
                max_neg_slew,
            } => {
                // slew(expr, max_pos_slew, max_neg_slew)
                self.emit_expr(expr, ir, program)?;
                // Emit max_pos_slew (default infinity = no limit)
                if let Some(p) = max_pos_slew {
                    self.emit_expr(p, ir, program)?;
                } else {
                    program
                        .instructions
                        .push(Instruction::PushConst(f64::INFINITY));
                }
                // Emit max_neg_slew (default to max_pos_slew)
                if let Some(n) = max_neg_slew {
                    self.emit_expr(n, ir, program)?;
                } else {
                    program
                        .instructions
                        .push(Instruction::PushConst(f64::INFINITY));
                }
                let filter_id = Self::allocate_slot(&self.slew_filter_count);
                program.instructions.push(Instruction::SlewState(filter_id));
            }
            IrExpr::Cross {
                expr,
                direction,
                time_tol: _,
            } => {
                // cross(expr, direction, time_tol)
                self.emit_expr(expr, ir, program)?;
                // Push direction constant (-1, 0, or +1)
                let dir = direction.unwrap_or(0);
                program
                    .instructions
                    .push(Instruction::PushConst(dir as f64));
                let detector_id = Self::allocate_slot(&self.cross_detector_count);
                program
                    .instructions
                    .push(Instruction::CrossState(detector_id));
            }
            IrExpr::WhiteNoise { power, name: _ } => {
                // $white_noise(power, name)
                // In time domain, noise returns 0
                // Contributes to AC noise analysis
                self.emit_expr(power, ir, program)?;
                program.instructions.push(Instruction::WhiteNoise);
            }
            IrExpr::FlickerNoise {
                power,
                exponent,
                name: _,
            } => {
                // $flicker_noise(power, exponent, name)
                self.emit_expr(power, ir, program)?;
                self.emit_expr(exponent, ir, program)?;
                program.instructions.push(Instruction::FlickerNoise);
            }
            IrExpr::Analysis(name) => {
                // analysis(name) - check current analysis type
                let analysis_id = match name.to_lowercase().as_str() {
                    "dc" => 0,
                    "ac" => 1,
                    "tran" | "transient" => 2,
                    "noise" => 3,
                    "ic" => 4,
                    _ => 255, // Unknown = always false
                };
                program
                    .instructions
                    .push(Instruction::Analysis(analysis_id));
            }
            IrExpr::Above {
                expr,
                threshold,
                time_tol: _,
            } => {
                // above(expr, threshold) - level crossing
                self.emit_expr(expr, ir, program)?;
                self.emit_expr(threshold, ir, program)?;
                let detector_id = Self::allocate_slot(&self.above_detector_count);
                program
                    .instructions
                    .push(Instruction::AboveState(detector_id));
            }
            IrExpr::Timer { start_time, period } => {
                // timer(start, period) - periodic trigger
                self.emit_expr(start_time, ir, program)?;
                if let Some(p) = period {
                    self.emit_expr(p, ir, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                let timer_id = Self::allocate_slot(&self.timer_state_count);
                program.instructions.push(Instruction::TimerState(timer_id));
            }
            IrExpr::Unary(op, _) => {
                return Err(CompileError::CodeGen(CodeGenError::new(
                    CodeGenErrorKind::UnsupportedFeature(format!("Unary op {:?}", op)),
                )));
            }
            IrExpr::LaplaceZP {
                expr,
                zeros,
                poles,
                gain,
            } => {
                self.emit_expr(expr, ir, program)?;

                let p_complex: Vec<Complex> = poles
                    .iter()
                    .map(|(re, im)| Complex::new(*re, *im))
                    .collect();
                let z_complex: Vec<Complex> = zeros
                    .iter()
                    .map(|(re, im)| Complex::new(*re, *im))
                    .collect();

                let filter = StateSpaceFilter::from_poles_zeros(&p_complex, &z_complex, *gain);
                let filter_id = self.laplace_filters.borrow().len();
                self.laplace_filters.borrow_mut().push(filter);

                program
                    .instructions
                    .push(Instruction::LaplaceState(filter_id));
            }
            IrExpr::LaplaceND {
                expr,
                numerator,
                denominator,
            } => {
                self.emit_expr(expr, ir, program)?;

                // IR has ascending powers: n0 + n1*s + ...
                // StateSpaceFilter expects descending: n_k*s^k + ... + n0
                let mut num_desc = numerator.clone();
                num_desc.reverse();
                let mut den_desc = denominator.clone();
                den_desc.reverse();

                let filter = StateSpaceFilter::from_transfer_function(&num_desc, &den_desc);
                let filter_id = self.laplace_filters.borrow().len();
                self.laplace_filters.borrow_mut().push(filter);

                program
                    .instructions
                    .push(Instruction::LaplaceState(filter_id));
            }
        }
        Ok(())
    }

    fn register_lookup_table(&self, x_data: &[f64], y_data: &[f64]) -> CompileResult<usize> {
        if x_data.len() != y_data.len() {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "$table_model x/y table length mismatch".into(),
            ))
            .into());
        }
        if x_data.len() < 2 {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "$table_model requires at least two table points".into(),
            ))
            .into());
        }

        let mut tables = self.lookup_tables.borrow_mut();
        if let Some((existing_idx, _)) = tables
            .iter()
            .enumerate()
            .find(|(_, table)| table.x_data == x_data && table.y_data == y_data)
        {
            return Ok(existing_idx);
        }

        let table = LookupTable::from_data(x_data.to_vec(), y_data.to_vec());
        tables.push(table);
        Ok(tables.len() - 1)
    }
}
