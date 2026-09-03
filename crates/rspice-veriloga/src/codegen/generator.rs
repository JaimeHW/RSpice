//! Emitting bytecode programs from the device-equation IR.
//!
//! Walks the IR and produces the programs a [`CompiledModel`] is made of: the
//! assignment program, a value and Jacobian program per stamp, the reactive
//! (charge) programs, and the noise metadata. Derivatives are read off the
//! shadow assignments the autodiff pass already produced, so nothing here
//! differentiates — it schedules and serializes what exists.

use super::*;
use std::collections::HashMap;

struct EmitContext {
    parameter_indices: HashMap<SmolStr, usize>,
    variable_indices: HashMap<SmolStr, usize>,
}

#[cfg(test)]
mod absdelay_derivative_tests {
    use super::*;
    use crate::ir::{AbsDelaySiteId, DerivativeWrt, IrExpr, autodiff};

    fn primal(site: AbsDelaySiteId, with_max: bool) -> IrExpr {
        IrExpr::AbsDelay {
            site,
            expr: Box::new(IrExpr::Voltage(0, usize::MAX)),
            delay_time: Box::new(IrExpr::Voltage(1, usize::MAX)),
            max_delay: with_max.then(|| Box::new(IrExpr::Const(2.0))),
        }
    }

    #[test]
    fn absdelay_maxdelay_and_exact_derivative_share_one_slot() {
        let generator = CodeGenerator::new();
        let emit_context = EmitContext {
            parameter_indices: HashMap::new(),
            variable_indices: HashMap::new(),
        };
        let site = AbsDelaySiteId::from_span(crate::source::Span::dummy());
        let primal = primal(site, true);
        let derivative = autodiff::differentiate(&primal, &DerivativeWrt::Voltage(0));
        let derivative_program = generator
            .compile_expr(&derivative, &emit_context)
            .expect("compile absdelay derivative first");
        let primal_program = generator
            .compile_expr(&primal, &emit_context)
            .expect("compile absdelay primal second");

        assert!(matches!(
            derivative_program.instructions.last(),
            Some(Instruction::AbsDelayStateDerivativeMax(0))
        ));
        assert!(matches!(
            primal_program.instructions.last(),
            Some(Instruction::AbsDelayStateMax(0))
        ));
        assert_eq!(generator.delay_buffer_count.get(), 1);
    }

    #[test]
    fn absdelay_second_derivative_fails_closed() {
        let site = AbsDelaySiteId::from_span(crate::source::Span::dummy());
        let first = autodiff::differentiate(&primal(site, false), &DerivativeWrt::Voltage(0));
        let second = autodiff::differentiate(&first, &DerivativeWrt::Voltage(0));
        let error = CodeGenerator::new()
            .compile_expr(
                &second,
                &EmitContext {
                    parameter_indices: HashMap::new(),
                    variable_indices: HashMap::new(),
                },
            )
            .expect_err("unsupported absdelay Hessian must fail compilation");
        assert!(error.to_string().contains("higher-order derivatives"));
    }

    #[test]
    fn source_compiler_retains_absdelay_maxdelay_and_exact_jacobian_action() {
        let source = r#"
`include "disciplines.vams"
module absdelay_max(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ absdelay(V(p, n), 1.0e-9, 2.0e-9);
endmodule
"#;
        let model = crate::VerilogACompiler::new(crate::CompilerOptions::default())
            .compile(source)
            .expect("compile absdelay with maxdelay");
        assert!(matches!(
            model.stamp_programs[0].value_program.instructions.last(),
            Some(Instruction::AbsDelayStateMax(0))
        ));
        assert!(
            model.stamp_programs[0]
                .jacobian_programs
                .iter()
                .any(|entry| matches!(
                    entry.program.instructions.last(),
                    Some(Instruction::AbsDelayStateDerivativeMax(0))
                ))
        );
    }

    #[test]
    fn source_compiler_rejects_invalid_absdelay_arity_and_dynamic_maxdelay() {
        let invalid_arity = r#"
`include "disciplines.vams"
module bad_absdelay_arity(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ absdelay(V(p, n));
endmodule
"#;
        let error = crate::VerilogACompiler::new(crate::CompilerOptions::default())
            .compile(invalid_arity)
            .expect_err("absdelay requires td");
        assert!(error.to_string().contains("2..3"));

        let dynamic_maxdelay = r#"
`include "disciplines.vams"
module bad_absdelay_max(p, n);
    inout p, n;
    electrical p, n;
    real dynamic_max;
    analog begin
        dynamic_max = V(p, n);
        I(p, n) <+ absdelay(V(p, n), 1.0e-9, dynamic_max);
    end
endmodule
"#;
        let error = crate::VerilogACompiler::new(crate::CompilerOptions::default())
            .compile(dynamic_maxdelay)
            .expect_err("absdelay maxdelay must be simulation invariant");
        assert!(error.to_string().contains("constant for the duration"));
    }
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
            laplace_sites: std::cell::RefCell::new(HashMap::new()),
            lookup_tables: std::cell::RefCell::new(Vec::new()),
            limit_state_count: std::cell::Cell::new(0),
            delay_buffer_count: std::cell::Cell::new(0),
            absdelay_sites: std::cell::RefCell::new(HashMap::new()),
            transition_filter_count: std::cell::Cell::new(0),
            transition_sites: std::cell::RefCell::new(HashMap::new()),
            slew_filter_count: std::cell::Cell::new(0),
            slew_sites: std::cell::RefCell::new(HashMap::new()),
            cross_detector_count: std::cell::Cell::new(0),
            timer_state_count: std::cell::Cell::new(0),
            zi_filters: std::cell::RefCell::new(Vec::new()),
            zi_filter_definitions: std::cell::RefCell::new(Vec::new()),
            zi_sites: std::cell::RefCell::new(HashMap::new()),
        }
    }

    /// Generate compiled model from analyzed file
    ///
    /// The file must contain exactly one module; files declaring several
    /// require an explicit selection via [`Self::generate_module`].
    pub fn generate(&self, analyzed: &AnalyzedFile) -> CompileResult<CompiledModel> {
        self.generate_module(analyzed, None)
    }

    /// Generate compiled model from analyzed file with a stable source digest.
    ///
    /// Use this when the resulting model may be paired with a canonical IR
    /// artifact for native JIT compilation. The digest must identify the same
    /// preprocessed source text used to produce the canonical artifact.
    pub fn generate_with_source_digest(
        &self,
        analyzed: &AnalyzedFile,
        source_digest: impl Into<SmolStr>,
    ) -> CompileResult<CompiledModel> {
        self.generate_module_with_source_digest(analyzed, None, source_digest)
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
        self.generate_module_inner(analyzed, module_name, SmolStr::default())
    }

    /// Generate compiled model for one module with a stable source digest.
    ///
    /// Use this when the resulting model may be paired with a canonical IR
    /// artifact for native JIT compilation. The digest must identify the same
    /// preprocessed source text used to produce the canonical artifact.
    pub fn generate_module_with_source_digest(
        &self,
        analyzed: &AnalyzedFile,
        module_name: Option<&str>,
        source_digest: impl Into<SmolStr>,
    ) -> CompileResult<CompiledModel> {
        self.generate_module_inner(analyzed, module_name, source_digest.into())
    }

    fn generate_module_inner(
        &self,
        analyzed: &AnalyzedFile,
        module_name: Option<&str>,
        source_digest: SmolStr,
    ) -> CompileResult<CompiledModel> {
        let selected = Self::select_module(analyzed, module_name)?;
        let module = crate::semantic::elaborate_executable_module(analyzed, selected)?;
        self.generate_analyzed_module_with_source_digest(&module, source_digest)
    }

    /// Lower one already-selected and already-elaborated analyzed module.
    /// Runtime compilation uses this internal boundary so bytecode and
    /// canonical IR consume the exact same owned hierarchy artifact.
    pub(crate) fn generate_analyzed_module_with_source_digest(
        &self,
        module: &AnalyzedModule,
        source_digest: impl Into<SmolStr>,
    ) -> CompileResult<CompiledModel> {
        self.generate_analyzed_module_inner(module, source_digest.into(), false)
    }

    /// Lower only the analog half after a mixed host has taken ownership of
    /// the module's canonical digital plan.
    pub(crate) fn generate_mixed_analog_half_with_source_digest(
        &self,
        module: &AnalyzedModule,
        source_digest: impl Into<SmolStr>,
    ) -> CompileResult<CompiledModel> {
        self.generate_analyzed_module_inner(module, source_digest.into(), true)
    }

    fn generate_analyzed_module_inner(
        &self,
        module: &AnalyzedModule,
        source_digest: SmolStr,
        mixed_host_owns_digital: bool,
    ) -> CompileResult<CompiledModel> {
        let timings = compile_timings_enabled();

        // Build IR
        let phase_start = web_time::Instant::now();
        let ir = if mixed_host_owns_digital {
            DeviceIR::from_analyzed_mixed_analog_half(module)?
        } else {
            DeviceIR::from_analyzed(module)?
        };
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
        let phase_start = web_time::Instant::now();
        let mut model = self.generate_from_ir(&ir)?;
        model.source_digest = source_digest;
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
        self.laplace_sites.borrow_mut().clear();
        self.zi_filters.borrow_mut().clear();
        self.zi_filter_definitions.borrow_mut().clear();
        self.zi_sites.borrow_mut().clear();
        self.limit_state_count.set(0);
        self.delay_buffer_count.set(0);
        self.absdelay_sites.borrow_mut().clear();
        self.transition_filter_count.set(0);
        self.transition_sites.borrow_mut().clear();
        self.slew_filter_count.set(0);
        self.slew_sites.borrow_mut().clear();
        self.cross_detector_count.set(0);
        self.timer_state_count.set(0);

        let phase_start = web_time::Instant::now();
        let parameters = ir
            .parameters
            .iter()
            .map(|p| {
                let resolve_bound = |name: &SmolStr| {
                    emit_ctx
                        .parameter_indices
                        .get(name)
                        .copied()
                        .ok_or_else(|| {
                            crate::error::CodeGenError::new(
                                crate::error::CodeGenErrorKind::Internal(format!(
                                    "parameter '{}' range references unknown parameter '{name}'",
                                    p.name
                                )),
                            )
                            .into()
                        })
                };
                let default_program = p
                    .default_expr
                    .as_ref()
                    .map(|expr| self.compile_expr(expr, &emit_ctx))
                    .transpose()?;
                Ok(CompiledParameter {
                    name: p.name.clone(),
                    is_public: p.is_public,
                    aliases: p.aliases.clone(),
                    default: p.default,
                    default_program,
                    is_integer: p.is_integer,
                    min: p.min,
                    max: p.max,
                    min_parameter: p.min_parameter.as_ref().map(resolve_bound).transpose()?,
                    max_parameter: p.max_parameter.as_ref().map(resolve_bound).transpose()?,
                    min_program: p
                        .min_expr
                        .as_ref()
                        .map(|expr| self.compile_expr(expr, &emit_ctx))
                        .transpose()?,
                    max_program: p
                        .max_expr
                        .as_ref()
                        .map(|expr| self.compile_expr(expr, &emit_ctx))
                        .transpose()?,
                    min_exclusive: p.min_exclusive,
                    max_exclusive: p.max_exclusive,
                    exclude: p.exclude.clone(),
                    exclude_parameters: p
                        .exclude_parameters
                        .iter()
                        .map(resolve_bound)
                        .collect::<CompileResult<Vec<_>>>()?,
                    exclude_programs: p
                        .exclude_exprs
                        .iter()
                        .map(|expr| self.compile_expr(expr, &emit_ctx))
                        .collect::<CompileResult<Vec<_>>>()?,
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
            source_digest: SmolStr::default(),
            num_terminals: ir.terminals.len(),
            terminal_names: ir.terminals.iter().map(|t| t.name.clone()).collect(),
            parameters,
            num_variables: ir.variables.len(),
            variable_names: ir.variables.iter().map(|v| v.name.clone()).collect(),
            event_state_variables: ir.event_state_variables.clone(),
            assignment_steps: Vec::new(),
            noise_assignment_steps: Vec::new(),
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
            zi_filter_definitions: Vec::new(),
            noise_process_schema: 1,
            noise_sources: Vec::new(),
        };

        // Generate evaluation steps (executed in order before contributions)
        let phase_start = web_time::Instant::now();
        model.assignment_steps = self.compile_assignment_items(&ir.assignments, &emit_ctx)?;
        model.noise_assignment_steps =
            self.compile_assignment_items(&ir.noise_assignments, &emit_ctx)?;
        if timings {
            eprintln!(
                "timing codegen.assignments module={} elapsed={:.3}s steps={}",
                ir.name,
                phase_start.elapsed().as_secs_f64(),
                count_assignment_steps_for_timing(&model.assignment_steps)
            );
        }

        // Generate stamp programs for each equation
        let phase_start = web_time::Instant::now();
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
        let phase_start = web_time::Instant::now();
        for source in &ir.noise_sources {
            let psd_program = self.compile_expr(&source.psd, &emit_ctx)?;
            let exponent_program = source
                .exponent
                .as_ref()
                .map(|e| self.compile_expr(e, &emit_ctx))
                .transpose()?;
            let injections = source
                .injections
                .iter()
                .map(|injection| {
                    let equation = &ir.equations[injection.equation_index];
                    let rhs_sign = if equation.indirect || equation.is_current {
                        -1.0
                    } else {
                        1.0
                    };
                    Ok(crate::codegen::CompiledNoiseInjection {
                        pos: Self::node_stamp_index(ir, injection.branch.pos_terminal),
                        neg: Self::node_stamp_index(ir, injection.branch.neg_terminal),
                        is_current: injection.is_current,
                        branch_ordinal: injection.branch_ordinal,
                        program_idx: injection.equation_index,
                        rhs_sign,
                        gain_program: self.compile_expr(&injection.gain, &emit_ctx)?,
                    })
                })
                .collect::<CompileResult<Vec<_>>>()?;
            model.noise_sources.push(CompiledNoiseSource {
                process_id: source.process_id,
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
                injections,
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
        model.zi_filter_definitions = self.zi_filter_definitions.take();

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
            DerivativeWrt::Noise(_) => {
                unreachable!("noise-process derivatives are not matrix Jacobian columns")
            }
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

    fn laplace_site_slot(
        &self,
        site: crate::ir::LaplaceSiteId,
        construct: impl FnOnce() -> CompileResult<StateSpaceFilter>,
    ) -> CompileResult<usize> {
        if let Some(slot) = self.laplace_sites.borrow().get(&site).copied() {
            return Ok(slot);
        }

        let filter = construct()?;
        let slot = self.laplace_filters.borrow().len();
        self.laplace_filters.borrow_mut().push(filter);
        self.laplace_sites.borrow_mut().insert(site, slot);
        Ok(slot)
    }

    fn slew_site_slot(&self, site: crate::ir::SlewSiteId) -> usize {
        if let Some(slot) = self.slew_sites.borrow().get(&site).copied() {
            return slot;
        }
        let slot = Self::allocate_slot(&self.slew_filter_count);
        self.slew_sites.borrow_mut().insert(site, slot);
        slot
    }

    fn transition_site_slot(&self, site: crate::ir::TransitionSiteId) -> usize {
        if let Some(slot) = self.transition_sites.borrow().get(&site).copied() {
            return slot;
        }
        let slot = Self::allocate_slot(&self.transition_filter_count);
        self.transition_sites.borrow_mut().insert(site, slot);
        slot
    }

    fn absdelay_site_slot(&self, site: crate::ir::AbsDelaySiteId) -> usize {
        if let Some(slot) = self.absdelay_sites.borrow().get(&site).copied() {
            return slot;
        }
        let slot = Self::allocate_slot(&self.delay_buffer_count);
        self.absdelay_sites.borrow_mut().insert(site, slot);
        slot
    }

    fn compile_zi_polynomial(
        &self,
        definition: &crate::ir::ZiPolynomialDefinition,
        emit_ctx: &EmitContext,
    ) -> CompileResult<CompiledZiPolynomial> {
        Ok(match definition {
            crate::ir::ZiPolynomialDefinition::Coefficients(values) => {
                CompiledZiPolynomial::Coefficients(
                    values
                        .iter()
                        .map(|value| self.compile_expr(value, emit_ctx))
                        .collect::<CompileResult<Vec<_>>>()?,
                )
            }
            crate::ir::ZiPolynomialDefinition::Roots(values) => CompiledZiPolynomial::Roots(
                values
                    .iter()
                    .map(|(real, imaginary)| {
                        Ok((
                            self.compile_expr(real, emit_ctx)?,
                            self.compile_expr(imaginary, emit_ctx)?,
                        ))
                    })
                    .collect::<CompileResult<Vec<_>>>()?,
            ),
        })
    }

    fn emit_zi_polynomial_operands(
        &self,
        definition: &crate::ir::ZiPolynomialDefinition,
        emit_ctx: &EmitContext,
        program: &mut BytecodeProgram,
    ) -> CompileResult<ZiPolynomialLayout> {
        match definition {
            crate::ir::ZiPolynomialDefinition::Coefficients(values) => {
                for value in values {
                    self.emit_expr(value, emit_ctx, program)?;
                }
                Ok(ZiPolynomialLayout::Coefficients { len: values.len() })
            }
            crate::ir::ZiPolynomialDefinition::Roots(values) => {
                for (real, imaginary) in values {
                    self.emit_expr(real, emit_ctx, program)?;
                    self.emit_expr(imaginary, emit_ctx, program)?;
                }
                Ok(ZiPolynomialLayout::Roots { len: values.len() })
            }
        }
    }

    fn zi_site_slot(
        &self,
        site: crate::ir::ZiSiteId,
        numerator: &crate::ir::ZiPolynomialDefinition,
        denominator: &crate::ir::ZiPolynomialDefinition,
        period: &IrExpr,
        first_transition: &IrExpr,
        emit_ctx: &EmitContext,
    ) -> CompileResult<usize> {
        let polynomial_layout = |definition: &crate::ir::ZiPolynomialDefinition| match definition {
            crate::ir::ZiPolynomialDefinition::Coefficients(values) => {
                ZiPolynomialLayout::Coefficients { len: values.len() }
            }
            crate::ir::ZiPolynomialDefinition::Roots(values) => {
                ZiPolynomialLayout::Roots { len: values.len() }
            }
        };
        let numerator_layout = polynomial_layout(numerator);
        let denominator_layout = polynomial_layout(denominator);
        let numerator_scalars = numerator_layout.checked_value_count().ok_or_else(|| {
            CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "Zi numerator root scalar count overflows usize".into(),
            ))
        })?;
        let denominator_scalars = denominator_layout.checked_value_count().ok_or_else(|| {
            CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "Zi denominator root scalar count overflows usize".into(),
            ))
        })?;
        crate::zfilter::validate_zi_runtime_operand_budget(
            "Zi filter",
            numerator_scalars,
            denominator_scalars,
        )
        .map_err(|error| {
            CodeGenError::new(CodeGenErrorKind::InvalidExpression(error.to_string()))
        })?;
        if let Some(slot) = self.zi_sites.borrow().get(&site).copied() {
            return Ok(slot);
        }
        let definition = CompiledZiFilterDefinition {
            numerator: self.compile_zi_polynomial(numerator, emit_ctx)?,
            denominator: self.compile_zi_polynomial(denominator, emit_ctx)?,
            period: self.compile_expr(period, emit_ctx)?,
            first_transition: self.compile_expr(first_transition, emit_ctx)?,
        };
        let slot = self.zi_filter_definitions.borrow().len();
        self.zi_filter_definitions.borrow_mut().push(definition);
        // The placeholder carries the site's declared coefficient widths so a
        // compiled model's Zi shape is the same before and after the first
        // evaluation freezes the per-instance values. A resume validates a
        // checkpoint against a rebuilt device that has not evaluated yet.
        let placeholder = crate::zfilter::ZiFilter::unfrozen_placeholder(
            numerator_layout.coefficient_count(),
            denominator_layout.coefficient_count(),
        )
        .map_err(|error| {
            CodeGenError::new(CodeGenErrorKind::Internal(format!(
                "failed to create internal Zi placeholder: {error}"
            )))
        })?;
        self.zi_filters.borrow_mut().push(placeholder);
        self.zi_sites.borrow_mut().insert(site, slot);
        Ok(slot)
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
            // Bitwise complement is represented through the shared integer
            // conversion and 32-bit XOR contract: ~x == x ^ -1.
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
                    IrFunction::LimitedExp => Instruction::LimitedExp,
                    // Inverse trig
                    IrFunction::Asin => Instruction::Asin,
                    IrFunction::Acos => Instruction::Acos,
                    IrFunction::Atan => Instruction::Atan,
                    IrFunction::Asinh => Instruction::Asinh,
                    IrFunction::Acosh => Instruction::Acosh,
                    IrFunction::Atanh => Instruction::Atanh,
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
            IrExpr::CanonicalLimit(inner) => {
                self.emit_expr(inner, emit_ctx, program)?;
                let state_id = Self::allocate_slot(&self.limit_state_count);
                program
                    .instructions
                    .push(Instruction::CanonicalLimitState(state_id));
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
            IrExpr::AbsDelay {
                site,
                expr,
                delay_time,
                max_delay,
            } => {
                self.emit_expr(expr, emit_ctx, program)?;
                self.emit_expr(delay_time, emit_ctx, program)?;
                if let Some(max_delay) = max_delay {
                    self.emit_expr(max_delay, emit_ctx, program)?;
                }
                let buffer_id = self.absdelay_site_slot(*site);
                program.instructions.push(if max_delay.is_some() {
                    Instruction::AbsDelayStateMax(buffer_id)
                } else {
                    Instruction::AbsDelayState(buffer_id)
                });
            }
            IrExpr::AbsDelayDerivative {
                site,
                input,
                input_derivative,
                delay_time,
                delay_derivative,
                max_delay,
                derivative_order,
            } => {
                if *derivative_order != 1 {
                    return Err(CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(
                        "absdelay higher-order derivatives are not implemented".into(),
                    ))
                    .into());
                }
                self.emit_expr(input, emit_ctx, program)?;
                self.emit_expr(input_derivative, emit_ctx, program)?;
                self.emit_expr(delay_time, emit_ctx, program)?;
                self.emit_expr(delay_derivative, emit_ctx, program)?;
                if let Some(max_delay) = max_delay {
                    self.emit_expr(max_delay, emit_ctx, program)?;
                }
                let buffer_id = self.absdelay_site_slot(*site);
                program.instructions.push(if max_delay.is_some() {
                    Instruction::AbsDelayStateDerivativeMax(buffer_id)
                } else {
                    Instruction::AbsDelayStateDerivative(buffer_id)
                });
            }
            IrExpr::Transition {
                site,
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
                // Semantic lowering normally materializes the module-scoped
                // rise default; retain an instantaneous defensive default for
                // directly constructed IR.
                if let Some(r) = rise_time {
                    self.emit_expr(r, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                // An omitted fall time reuses the effective rise expression.
                if let Some(f) = fall_time {
                    self.emit_expr(f, emit_ctx, program)?;
                } else if let Some(r) = rise_time {
                    self.emit_expr(r, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                let filter_id = self.transition_site_slot(*site);
                program
                    .instructions
                    .push(Instruction::TransitionState(filter_id));
            }
            IrExpr::TransitionDerivative {
                site,
                input,
                input_derivative,
                delay,
                rise_time,
                fall_time,
            } => {
                self.emit_expr(input, emit_ctx, program)?;
                self.emit_expr(input_derivative, emit_ctx, program)?;
                if let Some(delay) = delay {
                    self.emit_expr(delay, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(rise_time) = rise_time {
                    self.emit_expr(rise_time, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(fall_time) = fall_time {
                    self.emit_expr(fall_time, emit_ctx, program)?;
                } else if let Some(rise_time) = rise_time {
                    self.emit_expr(rise_time, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                let filter_id = self.transition_site_slot(*site);
                program
                    .instructions
                    .push(Instruction::TransitionStateDerivative(filter_id));
            }
            IrExpr::Slew {
                site,
                expr,
                max_pos_slew,
                max_neg_slew,
            } => {
                // With no authored rates the LRM defines an exact passthrough;
                // do not allocate or touch state in that form.
                if max_pos_slew.is_none() && max_neg_slew.is_none() {
                    self.emit_expr(expr, emit_ctx, program)?;
                    return Ok(());
                }
                self.emit_expr(expr, emit_ctx, program)?;
                let positive = max_pos_slew.as_ref().ok_or_else(|| {
                    CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "slew negative rate cannot be authored without a positive rate".into(),
                    ))
                })?;
                self.emit_expr(positive, emit_ctx, program)?;
                if let Some(n) = max_neg_slew {
                    self.emit_expr(n, emit_ctx, program)?;
                } else {
                    self.emit_expr(positive, emit_ctx, program)?;
                    program.instructions.push(Instruction::Neg);
                }
                let filter_id = self.slew_site_slot(*site);
                program.instructions.push(Instruction::SlewState(filter_id));
            }
            IrExpr::SlewDerivative {
                site,
                input,
                input_derivative,
                max_pos_slew,
                max_pos_slew_derivative,
                max_neg_slew,
                max_neg_slew_derivative,
            } => {
                if max_pos_slew.is_none() && max_neg_slew.is_none() {
                    self.emit_expr(input_derivative, emit_ctx, program)?;
                    return Ok(());
                }
                let positive = max_pos_slew.as_ref().ok_or_else(|| {
                    CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "slew negative rate cannot be authored without a positive rate".into(),
                    ))
                })?;
                let positive_derivative = max_pos_slew_derivative.as_ref().ok_or_else(|| {
                    CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "slew positive-rate derivative is missing".into(),
                    ))
                })?;
                self.emit_expr(input, emit_ctx, program)?;
                self.emit_expr(input_derivative, emit_ctx, program)?;
                self.emit_expr(positive, emit_ctx, program)?;
                self.emit_expr(positive_derivative, emit_ctx, program)?;
                if let (Some(negative), Some(negative_derivative)) =
                    (max_neg_slew, max_neg_slew_derivative)
                {
                    self.emit_expr(negative, emit_ctx, program)?;
                    self.emit_expr(negative_derivative, emit_ctx, program)?;
                } else {
                    self.emit_expr(positive, emit_ctx, program)?;
                    program.instructions.push(Instruction::Neg);
                    self.emit_expr(positive_derivative, emit_ctx, program)?;
                    program.instructions.push(Instruction::Neg);
                }
                let filter_id = self.slew_site_slot(*site);
                program
                    .instructions
                    .push(Instruction::SlewStateDerivative(filter_id));
            }
            IrExpr::Cross {
                expr,
                direction,
                time_tol,
                expr_tol,
                enable,
            } => {
                // cross(expr, direction, time_tol, expr_tol, enable)
                self.emit_expr(expr, emit_ctx, program)?;
                if let Some(direction) = direction {
                    self.emit_expr(direction, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(tolerance) = time_tol {
                    self.emit_expr(tolerance, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(tolerance) = expr_tol {
                    self.emit_expr(tolerance, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(enable) = enable {
                    self.emit_expr(enable, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(1.0));
                }
                let detector_id = Self::allocate_slot(&self.cross_detector_count);
                program
                    .instructions
                    .push(Instruction::CrossState(detector_id));
            }
            IrExpr::LastCrossing { expr, direction } => {
                self.emit_expr(expr, emit_ctx, program)?;
                program
                    .instructions
                    .push(Instruction::PushConst(direction.unwrap_or(0) as f64));
                let detector_id = Self::allocate_slot(&self.cross_detector_count);
                program
                    .instructions
                    .push(Instruction::LastCrossingState(detector_id));
            }
            IrExpr::WhiteNoise {
                power: _, name: _, ..
            } => {
                // The large-signal contribution is zero. The PSD operand is
                // compiled separately into model.noise_sources for noise
                // analysis and must not create stamp-time dependencies.
                program.instructions.push(Instruction::PushConst(0.0));
            }
            IrExpr::NoiseTable { .. } => {
                // Like the other noise functions, the large-signal value
                // is zero; the table feeds the noise-analysis sources
                program.instructions.push(Instruction::PushConst(0.0));
            }
            IrExpr::ZiFilter {
                site,
                expr,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                direct_assignment,
            } => {
                let filter_id = self.zi_site_slot(
                    *site,
                    numerator,
                    denominator,
                    period,
                    first_transition,
                    emit_ctx,
                )?;
                let numerator = self.emit_zi_polynomial_operands(numerator, emit_ctx, program)?;
                let denominator =
                    self.emit_zi_polynomial_operands(denominator, emit_ctx, program)?;
                self.emit_expr(period, emit_ctx, program)?;
                self.emit_expr(first_transition, emit_ctx, program)?;
                self.emit_expr(expr, emit_ctx, program)?;
                self.emit_expr(transition, emit_ctx, program)?;
                program
                    .instructions
                    .push(Instruction::ZiState(ZiRuntimeLayout {
                        filter_id,
                        numerator,
                        denominator,
                        direct_assignment: *direct_assignment,
                    }));
            }
            IrExpr::ZiFilterDerivative {
                site,
                expr,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                direct_assignment,
            } => {
                let filter_id = self.zi_site_slot(
                    *site,
                    numerator,
                    denominator,
                    period,
                    first_transition,
                    emit_ctx,
                )?;
                let numerator = self.emit_zi_polynomial_operands(numerator, emit_ctx, program)?;
                let denominator =
                    self.emit_zi_polynomial_operands(denominator, emit_ctx, program)?;
                self.emit_expr(period, emit_ctx, program)?;
                self.emit_expr(first_transition, emit_ctx, program)?;
                self.emit_expr(expr, emit_ctx, program)?;
                self.emit_expr(transition, emit_ctx, program)?;
                program
                    .instructions
                    .push(Instruction::ZiStateDerivative(ZiRuntimeLayout {
                        filter_id,
                        numerator,
                        denominator,
                        direct_assignment: *direct_assignment,
                    }));
            }
            IrExpr::FlickerNoise {
                power: _,
                exponent: _,
                name: _,
                ..
            } => {
                // The large-signal contribution is zero. PSD and exponent
                // programs are compiled separately for noise analysis.
                program.instructions.push(Instruction::PushConst(0.0));
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
                    "__rspice_initial_step" => 7,
                    "__rspice_final_step" => 8,
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
                time_tol,
                expr_tol,
                enable,
            } => {
                // above(expr, time_tol, expr_tol, enable)
                self.emit_expr(expr, emit_ctx, program)?;
                if let Some(tolerance) = time_tol {
                    self.emit_expr(tolerance, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(tolerance) = expr_tol {
                    self.emit_expr(tolerance, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(enable) = enable {
                    self.emit_expr(enable, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(1.0));
                }
                let detector_id = Self::allocate_slot(&self.cross_detector_count);
                program
                    .instructions
                    .push(Instruction::AboveState(detector_id));
            }
            IrExpr::Timer {
                start_time,
                period,
                time_tol,
                enable,
            } => {
                // timer(start, period, time_tol, enable)
                self.emit_expr(start_time, emit_ctx, program)?;
                if let Some(p) = period {
                    self.emit_expr(p, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(tolerance) = time_tol {
                    self.emit_expr(tolerance, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(enable) = enable {
                    self.emit_expr(enable, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(1.0));
                }
                let timer_id = Self::allocate_slot(&self.timer_state_count);
                program.instructions.push(Instruction::TimerState(timer_id));
            }
            IrExpr::LaplaceZP {
                site,
                expr,
                zeros,
                poles,
                gain,
            } => {
                self.emit_expr(expr, emit_ctx, program)?;

                let p_complex: Vec<Complex64> = poles
                    .iter()
                    .map(|(re, im)| Complex64::new(*re, *im))
                    .collect();
                let z_complex: Vec<Complex64> = zeros
                    .iter()
                    .map(|(re, im)| Complex64::new(*re, *im))
                    .collect();

                let filter_id = self.laplace_site_slot(*site, || {
                    StateSpaceFilter::from_poles_zeros(&p_complex, &z_complex, *gain).map_err(
                        |error| {
                            CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                                "laplace_zp: {error}"
                            )))
                            .into()
                        },
                    )
                })?;

                program
                    .instructions
                    .push(Instruction::LaplaceState(filter_id));
            }
            IrExpr::LaplaceND {
                site,
                expr,
                numerator,
                denominator,
            } => {
                self.emit_expr(expr, emit_ctx, program)?;

                let filter_id = self.laplace_site_slot(*site, || {
                    // IR has ascending powers: n0 + n1*s + ...
                    // StateSpaceFilter expects descending: n_k*s^k + ... + n0
                    let mut num_desc = numerator.clone();
                    num_desc.reverse();
                    let mut den_desc = denominator.clone();
                    den_desc.reverse();
                    StateSpaceFilter::from_transfer_function(&num_desc, &den_desc).map_err(
                        |error| {
                            CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                                "laplace coefficient form: {error}"
                            )))
                            .into()
                        },
                    )
                })?;

                program
                    .instructions
                    .push(Instruction::LaplaceState(filter_id));
            }
            IrExpr::LaplaceZPDerivative {
                site,
                expr,
                zeros,
                poles,
                gain,
            } => {
                self.emit_expr(expr, emit_ctx, program)?;

                let p_complex = poles
                    .iter()
                    .map(|(re, im)| Complex64::new(*re, *im))
                    .collect::<Vec<_>>();
                let z_complex = zeros
                    .iter()
                    .map(|(re, im)| Complex64::new(*re, *im))
                    .collect::<Vec<_>>();
                let filter_id = self.laplace_site_slot(*site, || {
                    StateSpaceFilter::from_poles_zeros(&p_complex, &z_complex, *gain).map_err(
                        |error| {
                            CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                                "laplace_zp derivative: {error}"
                            )))
                            .into()
                        },
                    )
                })?;
                program
                    .instructions
                    .push(Instruction::LaplaceStateDerivative(filter_id));
            }
            IrExpr::LaplaceNDDerivative {
                site,
                expr,
                numerator,
                denominator,
            } => {
                self.emit_expr(expr, emit_ctx, program)?;

                let filter_id = self.laplace_site_slot(*site, || {
                    let mut num_desc = numerator.clone();
                    num_desc.reverse();
                    let mut den_desc = denominator.clone();
                    den_desc.reverse();
                    StateSpaceFilter::from_transfer_function(&num_desc, &den_desc).map_err(
                        |error| {
                            CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                                "laplace coefficient derivative: {error}"
                            )))
                            .into()
                        },
                    )
                })?;
                program
                    .instructions
                    .push(Instruction::LaplaceStateDerivative(filter_id));
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

#[cfg(test)]
mod laplace_derivative_tests {
    use super::*;
    use crate::ir::{IrExpr, LaplaceSiteId};

    fn laplace_nd(site: LaplaceSiteId, derivative: bool) -> IrExpr {
        let expr = Box::new(IrExpr::Voltage(0, usize::MAX));
        if derivative {
            IrExpr::LaplaceNDDerivative {
                site,
                expr,
                numerator: vec![1.0],
                denominator: vec![1.0, 1.0],
            }
        } else {
            IrExpr::LaplaceND {
                site,
                expr,
                numerator: vec![1.0],
                denominator: vec![1.0, 1.0],
            }
        }
    }

    #[test]
    fn laplace_derivative_and_primal_share_a_slot_when_derivative_compiles_first() {
        let generator = CodeGenerator::new();
        let emit_context = EmitContext {
            parameter_indices: HashMap::new(),
            variable_indices: HashMap::new(),
        };
        let site = LaplaceSiteId::from_span(crate::source::Span::dummy());

        let derivative = generator
            .compile_expr(&laplace_nd(site, true), &emit_context)
            .expect("compile derivative first");
        let primal = generator
            .compile_expr(&laplace_nd(site, false), &emit_context)
            .expect("compile primal second");

        assert!(matches!(
            derivative.instructions.last(),
            Some(Instruction::LaplaceStateDerivative(0))
        ));
        assert!(matches!(
            primal.instructions.last(),
            Some(Instruction::LaplaceState(0))
        ));
        assert_eq!(generator.laplace_filters.borrow().len(), 1);
        assert_eq!(generator.laplace_sites.borrow().get(&site), Some(&0));
    }

    #[test]
    fn source_compiler_emits_explicit_laplace_jacobian_action() {
        let source = r#"
`include "disciplines.vams"
module laplace_jacobian(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ laplace_nd(V(p, n), '{1.0}, '{1.0, 1.0});
endmodule
"#;
        let model = crate::VerilogACompiler::new(crate::CompilerOptions::default())
            .compile(source)
            .expect("compile Laplace contribution");

        assert_eq!(model.laplace_filters.len(), 1);
        assert!(matches!(
            model.stamp_programs[0].value_program.instructions.last(),
            Some(Instruction::LaplaceState(0))
        ));
        assert!(
            model.stamp_programs[0]
                .jacobian_programs
                .iter()
                .any(|entry| matches!(
                    entry.program.instructions.last(),
                    Some(Instruction::LaplaceStateDerivative(0))
                ))
        );
    }
}

#[cfg(test)]
mod transition_derivative_tests {
    use super::*;
    use crate::ir::{IrExpr, TransitionSiteId};

    fn nonconstant_rise_time() -> IrExpr {
        IrExpr::Binary(
            BinaryOp::Add,
            Box::new(IrExpr::Voltage(1, usize::MAX)),
            Box::new(IrExpr::Const(0.5)),
        )
    }

    fn transition(site: TransitionSiteId, derivative: bool) -> IrExpr {
        if derivative {
            IrExpr::TransitionDerivative {
                site,
                input: Box::new(IrExpr::Voltage(0, usize::MAX)),
                input_derivative: Box::new(IrExpr::Const(1.0)),
                delay: Some(Box::new(IrExpr::Const(0.25))),
                rise_time: Some(Box::new(nonconstant_rise_time())),
                fall_time: None,
            }
        } else {
            IrExpr::Transition {
                site,
                expr: Box::new(IrExpr::Voltage(0, usize::MAX)),
                delay: Some(Box::new(IrExpr::Const(0.25))),
                rise_time: Some(Box::new(nonconstant_rise_time())),
                fall_time: None,
            }
        }
    }

    #[test]
    fn transition_derivative_and_primal_share_slot_and_fall_default() {
        let generator = CodeGenerator::new();
        let emit_context = EmitContext {
            parameter_indices: HashMap::new(),
            variable_indices: HashMap::new(),
        };
        let site = TransitionSiteId::from_span(crate::source::Span::dummy());

        let derivative = generator
            .compile_expr(&transition(site, true), &emit_context)
            .expect("compile transition derivative first");
        let primal = generator
            .compile_expr(&transition(site, false), &emit_context)
            .expect("compile transition primal second");

        assert!(matches!(
            derivative.instructions.last(),
            Some(Instruction::TransitionStateDerivative(0))
        ));
        assert!(matches!(
            primal.instructions.last(),
            Some(Instruction::TransitionState(0))
        ));
        assert_eq!(generator.transition_filter_count.get(), 1);
        assert_eq!(generator.transition_sites.borrow().get(&site), Some(&0));

        // An omitted fall time re-emits the rise expression, so the slots
        // between the delay and the terminating filter instruction are the
        // rise expression's own emission twice, back to back.
        let rise_ops: Vec<String> = generator
            .compile_expr(&nonconstant_rise_time(), &emit_context)
            .expect("compile the rise expression on its own")
            .instructions
            .iter()
            .map(|instruction| format!("{instruction:?}"))
            .collect();
        let expected: Vec<String> = rise_ops.iter().chain(rise_ops.iter()).cloned().collect();
        for (label, instructions) in [
            ("primal", &primal.instructions),
            ("derivative", &derivative.instructions),
        ] {
            let end = instructions.len() - 1;
            let start = end - expected.len();
            let actual: Vec<String> = instructions[start..end]
                .iter()
                .map(|instruction| format!("{instruction:?}"))
                .collect();
            assert_eq!(actual, expected, "{label} rise and fall slots");
            assert!(
                matches!(instructions[start - 1], Instruction::PushConst(0.25)),
                "{label} delay must sit immediately before the rise slot"
            );
        }
    }
}

#[cfg(test)]
mod slew_derivative_tests {
    use super::*;
    use crate::ast::BinaryOp;
    use crate::ir::{DerivativeWrt, IrExpr, SlewSiteId, autodiff};

    fn slew(site: SlewSiteId, derivative: bool) -> IrExpr {
        if derivative {
            IrExpr::SlewDerivative {
                site,
                input: Box::new(IrExpr::Voltage(0, usize::MAX)),
                input_derivative: Box::new(IrExpr::Const(1.0)),
                max_pos_slew: Some(Box::new(IrExpr::Const(2.0))),
                max_pos_slew_derivative: Some(Box::new(IrExpr::Const(0.0))),
                max_neg_slew: None,
                max_neg_slew_derivative: None,
            }
        } else {
            IrExpr::Slew {
                site,
                expr: Box::new(IrExpr::Voltage(0, usize::MAX)),
                max_pos_slew: Some(Box::new(IrExpr::Const(2.0))),
                max_neg_slew: None,
            }
        }
    }

    #[test]
    fn slew_derivative_and_primal_share_slot_when_derivative_compiles_first() {
        let generator = CodeGenerator::new();
        let emit_context = EmitContext {
            parameter_indices: HashMap::new(),
            variable_indices: HashMap::new(),
        };
        let site = SlewSiteId::from_span(crate::source::Span::dummy());

        let derivative = generator
            .compile_expr(&slew(site, true), &emit_context)
            .expect("compile derivative first");
        let primal = generator
            .compile_expr(&slew(site, false), &emit_context)
            .expect("compile primal second");

        assert!(matches!(
            derivative.instructions.last(),
            Some(Instruction::SlewStateDerivative(0))
        ));
        assert!(matches!(
            primal.instructions.last(),
            Some(Instruction::SlewState(0))
        ));
        assert_eq!(generator.slew_filter_count.get(), 1);
        assert_eq!(
            derivative
                .instructions
                .iter()
                .filter(|instruction| matches!(instruction, Instruction::Neg))
                .count(),
            2,
            "omitted negative rate and its derivative inherit the negated positive values"
        );
    }

    #[test]
    fn slew_without_rates_is_compiled_as_exact_passthrough_without_state() {
        let generator = CodeGenerator::new();
        let emit_context = EmitContext {
            parameter_indices: HashMap::new(),
            variable_indices: HashMap::new(),
        };
        let program = generator
            .compile_expr(
                &IrExpr::Slew {
                    site: SlewSiteId::from_span(crate::source::Span::dummy()),
                    expr: Box::new(IrExpr::Const(3.0)),
                    max_pos_slew: None,
                    max_neg_slew: None,
                },
                &emit_context,
            )
            .expect("compile passthrough slew");

        assert!(matches!(
            program.instructions.as_slice(),
            [Instruction::PushConst(3.0)]
        ));
        assert_eq!(generator.slew_filter_count.get(), 0);
    }

    #[test]
    fn slew_higher_derivatives_preserve_dynamic_rate_dependence() {
        let site = SlewSiteId::from_span(crate::source::Span::dummy());
        let voltage = IrExpr::Voltage(0, usize::MAX);
        let nonlinear_rate =
            IrExpr::Binary(BinaryOp::Mul, Box::new(voltage.clone()), Box::new(voltage));
        let primal = IrExpr::Slew {
            site,
            // Deliberately independent of the differentiation axis: the rate
            // is the only source of the saturated-branch Jacobian.
            expr: Box::new(IrExpr::Const(10.0)),
            max_pos_slew: Some(Box::new(nonlinear_rate)),
            max_neg_slew: None,
        };
        let first = autodiff::differentiate(&primal, &DerivativeWrt::Voltage(0));
        let second = autodiff::differentiate(&first, &DerivativeWrt::Voltage(0));

        let IrExpr::SlewDerivative {
            input_derivative,
            max_pos_slew_derivative,
            ..
        } = &first
        else {
            panic!("first slew derivative must retain a branch action");
        };
        assert!(matches!(input_derivative.as_ref(), IrExpr::Const(0.0)));
        assert!(
            !matches!(max_pos_slew_derivative.as_deref(), Some(IrExpr::Const(0.0))),
            "dynamic rate derivative must not be optimized to zero"
        );
        assert!(matches!(second, IrExpr::SlewDerivative { .. }));

        let generator = CodeGenerator::new();
        let emit_context = EmitContext {
            parameter_indices: HashMap::new(),
            variable_indices: HashMap::new(),
        };
        for derivative in [&first, &second] {
            let program = generator
                .compile_expr(derivative, &emit_context)
                .expect("compile branch-exact slew derivative");
            assert!(matches!(
                program.instructions.last(),
                Some(Instruction::SlewStateDerivative(0))
            ));
        }
    }
}
