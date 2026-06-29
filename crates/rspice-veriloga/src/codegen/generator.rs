use super::*;
use std::collections::HashMap;

struct EmitContext {
    parameter_indices: HashMap<SmolStr, usize>,
    variable_indices: HashMap<SmolStr, usize>,
}

impl EmitContext {
    fn from_ir(ir: &DeviceIR) -> Self {
        Self {
            parameter_indices: ir
                .parameters
                .iter()
                .enumerate()
                .map(|(idx, param)| (param.name.clone(), idx))
                .collect(),
            variable_indices: ir
                .variables
                .iter()
                .enumerate()
                .map(|(idx, var)| (var.name.clone(), idx))
                .collect(),
        }
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeGenerator {
    /// Create a new code generator
    pub fn new() -> Self {
        Self {
            laplace_filters: std::cell::RefCell::new(Vec::new()),
            lookup_tables: std::cell::RefCell::new(Vec::new()),
            limit_state_count: std::cell::Cell::new(0),
            delay_buffer_count: std::cell::Cell::new(0),
            transition_filter_count: std::cell::Cell::new(0),
            slew_filter_count: std::cell::Cell::new(0),
            cross_detector_count: std::cell::Cell::new(0),
            above_detector_count: std::cell::Cell::new(0),
            timer_state_count: std::cell::Cell::new(0),
            zi_filters: std::cell::RefCell::new(Vec::new()),
        }
    }

    /// Generate compiled model from analyzed file
    ///
    /// The file must contain exactly one module; files declaring several
    /// require an explicit selection via [`Self::generate_module`].
    pub fn generate(&self, analyzed: &AnalyzedFile) -> CompileResult<CompiledModel> {
        self.generate_module(analyzed, None)
    }

    /// Generate compiled model for one module of an analyzed file
    ///
    /// `module_name` selects the module to compile (foundry releases ship
    /// several modules per file). Without a name the file must contain
    /// exactly one module: picking an arbitrary one would be
    /// nondeterministic, so that case is an error naming the candidates.
    pub fn generate_module(
        &self,
        analyzed: &AnalyzedFile,
        module_name: Option<&str>,
    ) -> CompileResult<CompiledModel> {
        let module = Self::select_module(analyzed, module_name)?;
        let timings = compile_timings_enabled();

        // Build IR
        let phase_start = std::time::Instant::now();
        let ir = DeviceIR::from_analyzed(module)?;
        if timings {
            eprintln!(
                "timing codegen.ir module={} elapsed={:.3}s variables={} assignments={} equations={} branch_unknowns={}",
                ir.name,
                phase_start.elapsed().as_secs_f64(),
                ir.variables.len(),
                count_ir_assignment_items(&ir.assignments),
                ir.equations.len(),
                ir.branch_unknowns.len()
            );
        }

        // Generate code from IR
        let phase_start = std::time::Instant::now();
        let model = self.generate_from_ir(&ir)?;
        if timings {
            eprintln!(
                "timing codegen.bytecode module={} elapsed={:.3}s assignment_steps={} stamp_programs={} variables={}",
                model.name,
                phase_start.elapsed().as_secs_f64(),
                count_assignment_steps_for_timing(&model.assignment_steps),
                model.stamp_programs.len(),
                model.num_variables
            );
        }
        Ok(model)
    }

    /// Resolve which analyzed module to compile
    fn select_module<'a>(
        analyzed: &'a AnalyzedFile,
        module_name: Option<&str>,
    ) -> CompileResult<&'a AnalyzedModule> {
        // The modules map iterates in arbitrary order; list candidates in
        // declaration order so diagnostics are deterministic
        let declared: Vec<&str> = analyzed
            .source
            .items
            .iter()
            .filter_map(|item| match item {
                crate::ast::Item::Module(module) => Some(module.name.as_str()),
                _ => None,
            })
            .collect();

        match module_name {
            Some(name) => analyzed.modules.get(name).ok_or_else(|| {
                let candidates = if declared.is_empty() {
                    "none".to_string()
                } else {
                    declared.join(", ")
                };
                CompileError::ModuleSelection(format!(
                    "module '{}' not found; the file declares: {}",
                    name, candidates
                ))
            }),
            None => match declared.as_slice() {
                [] => Err(CompileError::ModuleSelection(
                    "no modules found in source".into(),
                )),
                [name] => analyzed.modules.get(*name).ok_or_else(|| {
                    CodeGenError::new(CodeGenErrorKind::Internal(format!(
                        "module '{}' was parsed but not analyzed",
                        name
                    )))
                    .into()
                }),
                names => Err(CompileError::ModuleSelection(format!(
                    "the file declares multiple modules: {}; select one by name",
                    names.join(", ")
                ))),
            },
        }
    }

    /// Generate from IR
    fn generate_from_ir(&self, ir: &DeviceIR) -> CompileResult<CompiledModel> {
        let timings = compile_timings_enabled();
        let emit_ctx = EmitContext::from_ir(ir);
        self.lookup_tables.borrow_mut().clear();
        self.laplace_filters.borrow_mut().clear();
        self.zi_filters.borrow_mut().clear();
        self.limit_state_count.set(0);
        self.delay_buffer_count.set(0);
        self.transition_filter_count.set(0);
        self.slew_filter_count.set(0);
        self.cross_detector_count.set(0);
        self.above_detector_count.set(0);
        self.timer_state_count.set(0);

        let phase_start = std::time::Instant::now();
        let parameters = ir
            .parameters
            .iter()
            .map(|p| {
                let default_program = p
                    .default_expr
                    .as_ref()
                    .map(|expr| self.compile_expr(expr, &emit_ctx))
                    .transpose()?;
                Ok(CompiledParameter {
                    name: p.name.clone(),
                    aliases: p.aliases.clone(),
                    default: p.default,
                    default_program,
                    min: p.min,
                    max: p.max,
                })
            })
            .collect::<CompileResult<Vec<_>>>()?;
        if timings {
            eprintln!(
                "timing codegen.parameters module={} elapsed={:.3}s count={}",
                ir.name,
                phase_start.elapsed().as_secs_f64(),
                parameters.len()
            );
        }

        let mut model = CompiledModel {
            name: ir.name.clone(),
            num_terminals: ir.terminals.len(),
            terminal_names: ir.terminals.iter().map(|t| t.name.clone()).collect(),
            parameters,
            num_variables: ir.variables.len(),
            variable_names: ir.variables.iter().map(|v| v.name.clone()).collect(),
            assignment_steps: Vec::new(),
            stamp_programs: Vec::new(),
            lookup_tables: Vec::new(),
            internal_nodes: ir.internal_nodes.len(),
            branch_sources: ir
                .branch_unknowns
                .iter()
                .map(|b| CompiledBranchSource {
                    pos: Self::node_stamp_index(ir, b.pos),
                    neg: Self::node_stamp_index(ir, b.neg),
                    indirect: b.indirect,
                })
                .collect(),
            laplace_filters: Vec::new(),
            zi_filters: Vec::new(),
            noise_sources: Vec::new(),
        };

        // Generate evaluation steps (executed in order before contributions)
        let phase_start = std::time::Instant::now();
        model.assignment_steps = self.compile_assignment_items(&ir.assignments, &emit_ctx)?;
        if timings {
            eprintln!(
                "timing codegen.assignments module={} elapsed={:.3}s steps={}",
                ir.name,
                phase_start.elapsed().as_secs_f64(),
                count_assignment_steps_for_timing(&model.assignment_steps)
            );
        }

        // Generate stamp programs for each equation
        let phase_start = std::time::Instant::now();
        for eq in &ir.equations {
            let program = self.compile_equation(eq, ir, &emit_ctx)?;
            model.stamp_programs.push(program);
        }
        if timings {
            eprintln!(
                "timing codegen.equations module={} elapsed={:.3}s equations={} programs={}",
                ir.name,
                phase_start.elapsed().as_secs_f64(),
                ir.equations.len(),
                model.stamp_programs.len()
            );
        }

        // Compile noise-source PSD programs (evaluated at the operating
        // point during noise analysis)
        let phase_start = std::time::Instant::now();
        for source in &ir.noise_sources {
            let psd_program = self.compile_expr(&source.psd, &emit_ctx)?;
            let exponent_program = source
                .exponent
                .as_ref()
                .map(|e| self.compile_expr(e, &emit_ctx))
                .transpose()?;
            model.noise_sources.push(CompiledNoiseSource {
                pos: Self::node_stamp_index(ir, source.branch.pos_terminal),
                neg: Self::node_stamp_index(ir, source.branch.neg_terminal),
                is_current: source.is_current,
                branch_ordinal: source.branch_ordinal,
                program_idx: source.equation_index,
                psd_program,
                exponent_program,
                table: source
                    .table
                    .as_ref()
                    .map(|t| (t.points.clone(), t.log_interp)),
                name: source.name.clone(),
            });
        }
        if timings {
            eprintln!(
                "timing codegen.noise module={} elapsed={:.3}s sources={}",
                ir.name,
                phase_start.elapsed().as_secs_f64(),
                model.noise_sources.len()
            );
        }

        model.laplace_filters = self.laplace_filters.take();
        model.lookup_tables = self.lookup_tables.take();
        model.zi_filters = self.zi_filters.take();

        Ok(model)
    }

    /// Compile assignment items (assignments and runtime loops) to steps
    fn compile_assignment_items(
        &self,
        items: &[crate::ir::IrAssignmentItem],
        emit_ctx: &EmitContext,
    ) -> CompileResult<Vec<AssignmentStep>> {
        items
            .iter()
            .map(|item| match item {
                crate::ir::IrAssignmentItem::Assign(assign) => {
                    let program = self.compile_expr(&assign.expr, emit_ctx)?;
                    match &assign.index {
                        Some(target) => Ok(AssignmentStep::AssignIndexed {
                            base: assign.var_index,
                            len: target.len,
                            lower: target.lower,
                            index: self.compile_expr(&target.index, emit_ctx)?,
                            value: program,
                        }),
                        None => Ok(AssignmentStep::Assign(AssignmentProgram {
                            var_index: assign.var_index,
                            program,
                        })),
                    }
                }
                crate::ir::IrAssignmentItem::Loop { condition, body } => {
                    let condition = self.compile_expr(condition, emit_ctx)?;
                    let body = self.compile_assignment_items(body, emit_ctx)?;
                    Ok(AssignmentStep::Loop { condition, body })
                }
            })
            .collect()
    }

    /// Map a unified node index (terminals, then internal nodes, ground
    /// sentinel) to a stamp index
    fn node_stamp_index(ir: &DeviceIR, node: usize) -> StampIndex {
        if node == crate::expr_converter::GROUND_NODE {
            StampIndex::Ground
        } else if node < ir.terminals.len() {
            StampIndex::Terminal(node)
        } else {
            StampIndex::Internal(node - ir.terminals.len())
        }
    }

    /// Map a derivative axis to its stamp column and column-axis record
    fn axis_stamp_column(ir: &DeviceIR, wrt: &DerivativeWrt) -> (StampIndex, ColumnAxis) {
        match wrt {
            DerivativeWrt::Voltage(node) => {
                (Self::node_stamp_index(ir, *node), ColumnAxis::Node(*node))
            }
            DerivativeWrt::BranchCurrent(k) => (StampIndex::Branch(*k), ColumnAxis::Branch(*k)),
        }
    }

    /// Compile a branch equation to a stamp program
    ///
    /// Current contributions use the standard SPICE companion form: the
    /// Jacobian G stamps both KCL rows and the RHS receives -/+ Ieq where
    /// Ieq = I - sum(G*x) is computed by the device at stamp time.
    ///
    /// Potential contributions define a branch-current unknown i_br with
    /// the equation V(p) - V(n) - E(...) = 0:
    /// the device stamps the structural +-1 coupling (KCL rows gain the
    /// branch column; the branch row gains the node columns), each
    /// -dE/dx into the branch row, and Eeq = E - sum(dE/dx * x) into the
    /// branch RHS.
    fn compile_equation(
        &self,
        eq: &BranchEquation,
        ir: &DeviceIR,
        emit_ctx: &EmitContext,
    ) -> CompileResult<StampProgram> {
        let value_program = self.compile_expr(&eq.expr, emit_ctx)?;
        let static_condition = eq
            .static_condition
            .as_ref()
            .map(|cond| self.compile_expr(cond, emit_ctx))
            .transpose()?;

        let pos = Self::node_stamp_index(ir, eq.branch.pos_terminal);
        let neg = Self::node_stamp_index(ir, eq.branch.neg_terminal);

        let mut jacobian_programs = Vec::new();

        if eq.indirect {
            // Indirect contribution: the branch row carries the constraint
            // f(x) = 0, stamped exactly like a single KCL row at the
            // branch unknown's equation (+df/dx entries; the device's
            // companion RHS gives rhs[br] -= f - sum df/dx * x)
            let ordinal = eq.branch_ordinal.ok_or_else(|| {
                CodeGenError::new(CodeGenErrorKind::Internal(
                    "indirect equation without a branch unknown".into(),
                ))
            })?;
            let branch_row = StampIndex::Branch(ordinal);
            for deriv in &eq.derivatives {
                let (col, col_axis) = Self::axis_stamp_column(ir, &deriv.wrt);
                let program = self.compile_expr(&deriv.expr, emit_ctx)?;
                jacobian_programs.push(JacobianEntry {
                    row: branch_row.clone(),
                    col,
                    col_axis,
                    sign: 1.0,
                    program,
                });
            }

            let mut reactive_jacobians = Vec::new();
            for deriv in &eq.reactive_derivatives {
                let (col, col_axis) = Self::axis_stamp_column(ir, &deriv.wrt);
                let program = self.compile_expr(&deriv.expr, emit_ctx)?;
                reactive_jacobians.push(JacobianEntry {
                    row: branch_row.clone(),
                    col,
                    col_axis,
                    sign: 1.0,
                    program,
                });
            }

            let stamp_locations = vec![StampLocation {
                row: branch_row,
                col: StampIndex::Ground,
                sign: -1.0,
            }];

            return Ok(StampProgram {
                stamp_locations,
                value_program,
                jacobian_programs,
                reactive_jacobians,
                branch_ordinal: Some(ordinal),
                indirect: true,
                static_condition,
            });
        }

        if let Some(ordinal) = eq.branch_ordinal {
            // Potential contribution: constitutive row of the branch
            // unknown receives -dE/dx for every axis
            let branch_row = StampIndex::Branch(ordinal);
            for deriv in &eq.derivatives {
                let (col, col_axis) = Self::axis_stamp_column(ir, &deriv.wrt);
                let program = self.compile_expr(&deriv.expr, emit_ctx)?;
                jacobian_programs.push(JacobianEntry {
                    row: branch_row.clone(),
                    col,
                    col_axis,
                    sign: -1.0,
                    program,
                });
            }

            // Reactive part of the source (flux: V <+ ddt(L*i)) stamps
            // -jw * dQ/dx into the branch row in AC
            let mut reactive_jacobians = Vec::new();
            for deriv in &eq.reactive_derivatives {
                let (col, col_axis) = Self::axis_stamp_column(ir, &deriv.wrt);
                let program = self.compile_expr(&deriv.expr, emit_ctx)?;
                reactive_jacobians.push(JacobianEntry {
                    row: branch_row.clone(),
                    col,
                    col_axis,
                    sign: -1.0,
                    program,
                });
            }

            // The companion source Eeq stamps into the branch row
            let stamp_locations = vec![StampLocation {
                row: branch_row,
                col: StampIndex::Ground,
                sign: 1.0,
            }];

            return Ok(StampProgram {
                stamp_locations,
                value_program,
                jacobian_programs,
                reactive_jacobians,
                branch_ordinal: Some(ordinal),
                indirect: false,
                static_condition,
            });
        }

        // Current contribution
        for deriv in &eq.derivatives {
            let (col, col_axis) = Self::axis_stamp_column(ir, &deriv.wrt);
            let program = self.compile_expr(&deriv.expr, emit_ctx)?;

            // KCL row of the positive node gains +dI/dx, the negative node
            // row gains -dI/dx
            jacobian_programs.push(JacobianEntry {
                row: pos.clone(),
                col: col.clone(),
                col_axis,
                sign: 1.0,
                program: program.clone(),
            });
            jacobian_programs.push(JacobianEntry {
                row: neg.clone(),
                col,
                col_axis,
                sign: -1.0,
                program,
            });
        }

        // Reactive (capacitance) entries: AC stamps jw * dQ/dx with the
        // same KCL row pairing
        let mut reactive_jacobians = Vec::new();
        for deriv in &eq.reactive_derivatives {
            let (col, col_axis) = Self::axis_stamp_column(ir, &deriv.wrt);
            let program = self.compile_expr(&deriv.expr, emit_ctx)?;
            reactive_jacobians.push(JacobianEntry {
                row: pos.clone(),
                col: col.clone(),
                col_axis,
                sign: 1.0,
                program: program.clone(),
            });
            reactive_jacobians.push(JacobianEntry {
                row: neg.clone(),
                col,
                col_axis,
                sign: -1.0,
                program,
            });
        }

        // Current contribution: I leaves pos, enters neg.
        // The device computes Ieq = I - G*x and stamps rhs[pos] -= Ieq,
        // rhs[neg] += Ieq (signs recorded here).
        let stamp_locations = vec![
            StampLocation {
                row: pos,
                col: StampIndex::Ground,
                sign: -1.0,
            },
            StampLocation {
                row: neg,
                col: StampIndex::Ground,
                sign: 1.0,
            },
        ];

        Ok(StampProgram {
            stamp_locations,
            value_program,
            jacobian_programs,
            reactive_jacobians,
            branch_ordinal: None,
            indirect: false,
            static_condition,
        })
    }

    /// Compile an IR expression to bytecode
    fn compile_expr(
        &self,
        expr: &IrExpr,
        emit_ctx: &EmitContext,
    ) -> CompileResult<BytecodeProgram> {
        let mut program = BytecodeProgram::default();
        self.emit_expr(expr, emit_ctx, &mut program)?;
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
        emit_ctx: &EmitContext,
        program: &mut BytecodeProgram,
    ) -> CompileResult<()> {
        match expr {
            IrExpr::Const(v) => {
                program.instructions.push(Instruction::PushConst(*v));
            }
            IrExpr::Param(name) => {
                let idx = emit_ctx
                    .parameter_indices
                    .get(name)
                    .copied()
                    .ok_or_else(|| {
                        CodeGenError::new(CodeGenErrorKind::Internal(format!(
                            "Unknown parameter: {}",
                            name
                        )))
                    })?;
                program.instructions.push(Instruction::PushParam(idx));
            }
            IrExpr::ParamGiven(name) => {
                let idx = emit_ctx
                    .parameter_indices
                    .get(name)
                    .copied()
                    .ok_or_else(|| {
                        CodeGenError::new(CodeGenErrorKind::Internal(format!(
                            "Unknown parameter: {}",
                            name
                        )))
                    })?;
                program.instructions.push(Instruction::PushParamGiven(idx));
            }
            IrExpr::Var(name) => {
                let idx = emit_ctx
                    .variable_indices
                    .get(name)
                    .copied()
                    .ok_or_else(|| {
                        CodeGenError::new(CodeGenErrorKind::Internal(format!(
                            "Unknown variable: {}",
                            name
                        )))
                    })?;
                program.instructions.push(Instruction::PushVariable(idx));
            }
            IrExpr::VarIndexed {
                base,
                len,
                lower,
                index,
                ..
            } => {
                self.emit_expr(index, emit_ctx, program)?;
                program.instructions.push(Instruction::PushVariableDyn {
                    base: *base,
                    len: *len,
                    lower: *lower,
                });
            }
            IrExpr::Voltage(p, n) => {
                program.instructions.push(Instruction::PushVoltage(*p, *n));
            }
            IrExpr::Current(p, n) => {
                program.instructions.push(Instruction::PushCurrent(*p, *n));
            }
            IrExpr::BranchCurrent(k) => {
                program
                    .instructions
                    .push(Instruction::PushBranchCurrent(*k));
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
            IrExpr::Mfactor => {
                program.instructions.push(Instruction::PushMfactor);
            }
            IrExpr::PortConnected(index) => {
                program
                    .instructions
                    .push(Instruction::PushPortConnected(*index));
            }
            IrExpr::Binary(op, left, right) => {
                self.emit_expr(left, emit_ctx, program)?;
                self.emit_expr(right, emit_ctx, program)?;
                program.instructions.push(match op {
                    // Arithmetic
                    BinaryOp::Add => Instruction::Add,
                    BinaryOp::Sub => Instruction::Sub,
                    BinaryOp::Mul => Instruction::Mul,
                    BinaryOp::Div => Instruction::Div,
                    BinaryOp::Pow => Instruction::Pow,
                    BinaryOp::Mod => Instruction::Mod,
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
                    // Bitwise/shift
                    BinaryOp::Shl => Instruction::Shl,
                    BinaryOp::Shr => Instruction::Shr,
                    BinaryOp::BitAnd => Instruction::BitAnd,
                    BinaryOp::BitOr => Instruction::BitOr,
                    BinaryOp::BitXor => Instruction::BitXor,
                });
            }
            IrExpr::Unary(crate::ast::UnaryOp::Neg, inner) => {
                self.emit_expr(inner, emit_ctx, program)?;
                program.instructions.push(Instruction::Neg);
            }
            // Unary plus is the identity
            IrExpr::Unary(crate::ast::UnaryOp::Pos, inner) => {
                self.emit_expr(inner, emit_ctx, program)?;
            }
            // Bitwise complement truncates to integer: ~x = -x - 1
            IrExpr::Unary(crate::ast::UnaryOp::BitNot, inner) => {
                self.emit_expr(inner, emit_ctx, program)?;
                program.instructions.push(Instruction::PushConst(-1.0));
                program.instructions.push(Instruction::BitXor);
            }
            IrExpr::Call(func, args) => {
                for arg in args {
                    self.emit_expr(arg, emit_ctx, program)?;
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
                self.emit_expr(inner, emit_ctx, program)?;
                program.instructions.push(Instruction::Limexp);
            }
            IrExpr::Conditional(cond, then_expr, else_expr) => {
                self.emit_expr(cond, emit_ctx, program)?;
                self.emit_expr(then_expr, emit_ctx, program)?;
                self.emit_expr(else_expr, emit_ctx, program)?;
                program.instructions.push(Instruction::IfElse);
            }
            IrExpr::Unary(crate::ast::UnaryOp::Not, inner) => {
                self.emit_expr(inner, emit_ctx, program)?;
                program.instructions.push(Instruction::Not);
            }
            IrExpr::Ddt(inner) => {
                // Backward-Euler time derivative with a dedicated state slot:
                // (value - prev_value) / dt in transient, 0 at DC. The state
                // slot records the operand so the next step has its history.
                self.emit_expr(inner, emit_ctx, program)?;
                let state_id = Self::allocate_slot(&self.limit_state_count);
                program.instructions.push(Instruction::DdtState(state_id));
            }
            IrExpr::Idt(inner, ic) => {
                // Time integral: state + value*dt in transient; the initial
                // condition (default 0) seeds the integral at DC/IC.
                self.emit_expr(inner, emit_ctx, program)?;
                if let Some(ic_expr) = ic {
                    self.emit_expr(ic_expr, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                let state_id = Self::allocate_slot(&self.limit_state_count);
                program.instructions.push(Instruction::IdtState(state_id));
            }
            IrExpr::IdtMod {
                expr,
                ic,
                modulus,
                offset,
            } => {
                self.emit_expr(expr, emit_ctx, program)?;
                match ic {
                    Some(ic) => self.emit_expr(ic, emit_ctx, program)?,
                    None => program.instructions.push(Instruction::PushConst(0.0)),
                }
                self.emit_expr(modulus, emit_ctx, program)?;
                match offset {
                    Some(offset) => self.emit_expr(offset, emit_ctx, program)?,
                    None => program.instructions.push(Instruction::PushConst(0.0)),
                }
                let state_id = Self::allocate_slot(&self.limit_state_count);
                program
                    .instructions
                    .push(Instruction::IdtModState(state_id));
            }
            IrExpr::DdtCompanion(inner) => {
                // Jacobian companion factor: operand / dt (0 at DC)
                self.emit_expr(inner, emit_ctx, program)?;
                program.instructions.push(Instruction::DdtJacobian);
            }
            IrExpr::IdtCompanion(inner) => {
                // Jacobian companion factor: operand * dt (0 at DC)
                self.emit_expr(inner, emit_ctx, program)?;
                program.instructions.push(Instruction::IdtJacobian);
            }
            IrExpr::TableDerivative {
                input,
                x_data,
                y_data,
            } => {
                self.emit_expr(input, emit_ctx, program)?;
                let table_id = self.register_lookup_table(x_data, y_data)?;
                program
                    .instructions
                    .push(Instruction::TableDerivative(table_id));
            }
            IrExpr::Ddx { .. } => {
                return Err(CompileError::CodeGen(CodeGenError::new(
                    CodeGenErrorKind::Internal("unresolved ddx() reached code generation".into()),
                )));
            }
            IrExpr::Limit(inner, step) => {
                // $limit(expr, step) - bounds value change per Newton iteration
                // For DC, we track previous value and limit the step
                self.emit_expr(inner, emit_ctx, program)?;
                if let Some(step_expr) = step {
                    self.emit_expr(step_expr, emit_ctx, program)?;
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
                self.emit_expr(input, emit_ctx, program)?;
                let table_id = self.register_lookup_table(x_data, y_data)?;
                program
                    .instructions
                    .push(Instruction::TableLookup(table_id));
            }
            IrExpr::AbsDelay { expr, delay_time } => {
                // absdelay(expr, delay_time) - transport delay
                // Emit expression value, then delay time, then AbsDelayState instruction
                self.emit_expr(expr, emit_ctx, program)?;
                self.emit_expr(delay_time, emit_ctx, program)?;
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
                self.emit_expr(expr, emit_ctx, program)?;
                // Emit delay (default 0)
                if let Some(d) = delay {
                    self.emit_expr(d, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                // Emit rise_time (default 0 = instantaneous)
                if let Some(r) = rise_time {
                    self.emit_expr(r, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                // Emit fall_time (default to rise_time)
                if let Some(f) = fall_time {
                    self.emit_expr(f, emit_ctx, program)?;
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
                self.emit_expr(expr, emit_ctx, program)?;
                // Emit max_pos_slew (default infinity = no limit)
                if let Some(p) = max_pos_slew {
                    self.emit_expr(p, emit_ctx, program)?;
                } else {
                    program
                        .instructions
                        .push(Instruction::PushConst(f64::INFINITY));
                }
                // Emit max_neg_slew (default to max_pos_slew)
                if let Some(n) = max_neg_slew {
                    self.emit_expr(n, emit_ctx, program)?;
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
                self.emit_expr(expr, emit_ctx, program)?;
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
                self.emit_expr(power, emit_ctx, program)?;
                program.instructions.push(Instruction::WhiteNoise);
            }
            IrExpr::NoiseTable { .. } => {
                // Like the other noise functions, the large-signal value
                // is zero; the table feeds the noise-analysis sources
                program.instructions.push(Instruction::PushConst(0.0));
            }
            IrExpr::ZiFilter {
                expr,
                numerator,
                denominator,
                period,
            } => {
                self.emit_expr(expr, emit_ctx, program)?;
                let filter_id = {
                    let mut filters = self.zi_filters.borrow_mut();
                    filters.push(crate::zfilter::ZiFilter::new(
                        numerator.clone(),
                        denominator.clone(),
                        *period,
                    ));
                    filters.len() - 1
                };
                program.instructions.push(Instruction::ZiState(filter_id));
            }
            IrExpr::FlickerNoise {
                power,
                exponent,
                name: _,
            } => {
                // $flicker_noise(power, exponent, name)
                self.emit_expr(power, emit_ctx, program)?;
                self.emit_expr(exponent, emit_ctx, program)?;
                program.instructions.push(Instruction::FlickerNoise);
            }
            IrExpr::Analysis(name) => {
                // analysis(name) - check current analysis type
                let analysis_id = match name.to_lowercase().as_str() {
                    "dc" | "op" => 0,
                    "ac" => 1,
                    "tran" | "transient" => 2,
                    "noise" => 3,
                    "ic" => 4,
                    // "static" matches any equilibrium (DC or IC) analysis
                    "static" => 5,
                    // "smallsig" matches small-signal frequency-domain analyses.
                    "smallsig" | "smallsignal" | "small_signal" => 6,
                    _ => {
                        return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                            format!("analysis() unknown analysis name '{name}'"),
                        ))
                        .into());
                    }
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
                self.emit_expr(expr, emit_ctx, program)?;
                self.emit_expr(threshold, emit_ctx, program)?;
                let detector_id = Self::allocate_slot(&self.above_detector_count);
                program
                    .instructions
                    .push(Instruction::AboveState(detector_id));
            }
            IrExpr::Timer { start_time, period } => {
                // timer(start, period) - periodic trigger
                self.emit_expr(start_time, emit_ctx, program)?;
                if let Some(p) = period {
                    self.emit_expr(p, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                let timer_id = Self::allocate_slot(&self.timer_state_count);
                program.instructions.push(Instruction::TimerState(timer_id));
            }
            IrExpr::LaplaceZP {
                expr,
                zeros,
                poles,
                gain,
            } => {
                self.emit_expr(expr, emit_ctx, program)?;

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
                self.emit_expr(expr, emit_ctx, program)?;

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

fn compile_timings_enabled() -> bool {
    std::env::var_os("RSPICE_VERILOGA_COMPILE_TIMINGS").is_some()
}

fn count_ir_assignment_items(items: &[crate::ir::IrAssignmentItem]) -> usize {
    items
        .iter()
        .map(|item| match item {
            crate::ir::IrAssignmentItem::Assign(_) => 1,
            crate::ir::IrAssignmentItem::Loop { body, .. } => 1 + count_ir_assignment_items(body),
        })
        .sum()
}

fn count_assignment_steps_for_timing(items: &[AssignmentStep]) -> usize {
    items
        .iter()
        .map(|item| match item {
            AssignmentStep::Assign(_) | AssignmentStep::AssignIndexed { .. } => 1,
            AssignmentStep::Loop { body, .. } => 1 + count_assignment_steps_for_timing(body),
        })
        .sum()
}
