//! Emitting bytecode programs from the device-equation IR.
//!
//! Walks the IR and produces the programs a [`CompiledModel`] is made of: the
//! assignment program, a value and Jacobian program per stamp, the reactive
//! (charge) programs, and the noise metadata. Derivatives are read off the
//! shadow assignments the autodiff pass already produced, so nothing here
//! differentiates — it schedules and serializes what exists.

use super::*;
use crate::ir::arena::{ExprArena, Heavy, Node, NodeId, ZiPolynomial, unpack_index};
use std::collections::HashMap;

#[derive(Default)]
struct EmitContext {
    parameter_indices: HashMap<SmolStr, usize>,
    variable_indices: HashMap<SmolStr, usize>,
    // One arena node is one integration/limiting site, even when AD reads
    // its primal several times. This context belongs to a single IR arena.
    integration_sites: std::cell::RefCell<HashMap<NodeId, usize>>,
}

#[cfg(test)]
mod absdelay_derivative_tests {
    use super::*;
    use crate::ir::arena::Heavy;
    use crate::ir::{AbsDelaySiteId, DerivativeWrt, autodiff};

    fn primal(arena: &mut ExprArena, site: AbsDelaySiteId, with_max: bool) -> NodeId {
        let expr = arena.push(Node::Voltage(0, u32::MAX));
        let delay_time = arena.push(Node::Voltage(1, u32::MAX));
        let max_delay = with_max.then(|| arena.push(Node::Const(2.0)));
        arena.push_heavy(Heavy::AbsDelay {
            site,
            expr,
            delay_time,
            max_delay,
        })
    }

    #[test]
    fn absdelay_maxdelay_and_exact_derivative_share_one_slot() {
        let generator = CodeGenerator::new();
        let emit_context = empty_emit_context();
        let site = AbsDelaySiteId::from_span(crate::source::Span::dummy());
        let arena = &mut ExprArena::new();
        let primal = primal(arena, site, true);
        let derivative = autodiff::differentiate(arena, primal, &DerivativeWrt::Voltage(0));
        let derivative_program = generator
            .compile_expr(arena, derivative, &emit_context)
            .expect("compile absdelay derivative first");
        let primal_program = generator
            .compile_expr(arena, primal, &emit_context)
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
        let arena = &mut ExprArena::new();
        let primal = primal(arena, site, false);
        let first = autodiff::differentiate(arena, primal, &DerivativeWrt::Voltage(0));
        let second = autodiff::differentiate(arena, first, &DerivativeWrt::Voltage(0));
        let error = CodeGenerator::new()
            .compile_expr(arena, second, &empty_emit_context())
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
    fn integration_slot(&self, node: NodeId, counter: &std::cell::Cell<usize>) -> usize {
        *self
            .integration_sites
            .borrow_mut()
            .entry(node)
            .or_insert_with(|| CodeGenerator::allocate_slot(counter))
    }

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
            integration_sites: Default::default(),
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
            requires_nodeset_phase: std::cell::Cell::new(false),
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
        let mut ir = if mixed_host_owns_digital {
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
        let mut model = self.generate_from_ir(&mut ir)?;
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

    /// Generate compiled model from IR
    ///
    /// # The arena is the emitter's input
    ///
    /// Every family of expressions this function compiles — the parameter
    /// default and range programs, the assignment items, the branch equations
    /// and the noise PSD, exponent and gain programs — is already a [`NodeId`]
    /// into [`DeviceIR::exprs`] when it arrives. There is no import and no
    /// boxed tree anywhere in the front end to import from: the converter
    /// writes these nodes, the differentiation core rewrites them, and this
    /// function reads them.
    ///
    /// # Why the IR is taken by `&mut`
    ///
    /// The shadow-expanded assignment forest is the largest allocation of the
    /// whole compile — eighteen million nodes on `bsimcmg` — and nothing after
    /// the assignment pass reads it. Taking the arena out of the IR rather
    /// than borrowing it lets the forest be dropped with the IR while the
    /// bytecode is still being written, which is part of what lets the largest
    /// shipped models compile inside a 32 GB box.
    /// The generator's emission state ([`EmitContext`], the site maps, the
    /// per-emission counters) is owned by `self` and by a context built before
    /// the first take, so nothing borrows the IR across a drop.
    fn generate_from_ir(&self, ir: &mut DeviceIR) -> CompileResult<CompiledModel> {
        let timings = compile_timings_enabled();
        let emit_ctx = EmitContext::from_ir(ir);
        let num_terminals = ir.terminals.len();
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
        self.requires_nodeset_phase.set(false);

        // Every expression this module emits already lives in the IR's arena.
        // It is taken out rather than borrowed so the IR's own lists can be
        // consumed while the emitter reads it, and it dies with this call.
        let arena = std::mem::take(&mut ir.exprs);

        let phase_start = web_time::Instant::now();
        let parameters = std::mem::take(&mut ir.parameters)
            .into_iter()
            .map(|p| {
                let param_name = p.name.clone();
                let resolve_bound = |name: &SmolStr| {
                    emit_ctx
                        .parameter_indices
                        .get(name)
                        .copied()
                        .ok_or_else(|| {
                            crate::error::CodeGenError::new(
                                crate::error::CodeGenErrorKind::Internal(format!(
                                    "parameter '{param_name}' range references unknown parameter '{name}'"
                                )),
                            )
                            .into()
                        })
                };
                let default_program = p
                    .default_expr
                    .map(|expr| self.compile_expr(&arena, expr, &emit_ctx))
                    .transpose()?;
                Ok(CompiledParameter {
                    name: p.name,
                    is_public: p.is_public,
                    aliases: p.aliases,
                    default: p.default,
                    default_program,
                    is_integer: p.is_integer,
                    min: p.min,
                    max: p.max,
                    min_parameter: p.min_parameter.as_ref().map(resolve_bound).transpose()?,
                    max_parameter: p.max_parameter.as_ref().map(resolve_bound).transpose()?,
                    min_program: p
                        .min_expr
                        .map(|expr| self.compile_expr(&arena, expr, &emit_ctx))
                        .transpose()?,
                    max_program: p
                        .max_expr
                        .map(|expr| self.compile_expr(&arena, expr, &emit_ctx))
                        .transpose()?,
                    min_exclusive: p.min_exclusive,
                    max_exclusive: p.max_exclusive,
                    exclude: p.exclude,
                    exclude_parameters: p
                        .exclude_parameters
                        .iter()
                        .map(resolve_bound)
                        .collect::<CompileResult<Vec<_>>>()?,
                    exclude_programs: p
                        .exclude_exprs
                        .into_iter()
                        .map(|expr| self.compile_expr(&arena, expr, &emit_ctx))
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
            requires_nodeset_phase: false,
            num_terminals: ir.terminals.len(),
            terminal_names: ir.terminals.iter().map(|t| t.name.clone()).collect(),
            parameters,
            num_variables: ir.variables.len(),
            variable_names: ir.variables.iter().map(|v| v.name.clone()).collect(),
            event_state_variables: ir.event_state_variables.clone(),
            switch_branch_variables: ir.switch_branch_variables.clone(),
            initialization_prologue_variables: ir.initialization_prologue_variables.clone(),
            assignment_steps: Vec::new(),
            noise_assignment_steps: Vec::new(),
            stamp_programs: Vec::new(),
            lookup_tables: Vec::new(),
            internal_nodes: ir.internal_nodes.len(),
            internal_state_nodes: ir
                .internal_nodes
                .iter()
                .filter(|node| node.is_state)
                .map(|node| node.index)
                .collect(),
            branch_sources: ir
                .branch_unknowns
                .iter()
                .map(|b| {
                    Ok(CompiledBranchSource {
                        declared_name: b.declared_name.clone(),
                        pos: Self::node_stamp_index(num_terminals, b.pos),
                        neg: Self::node_stamp_index(num_terminals, b.neg),
                        indirect: b.indirect,
                        equation_abstol: b
                            .equation_abstol
                            .map(|expr| self.compile_expr(&arena, expr, &emit_ctx))
                            .transpose()?,
                    })
                })
                .collect::<CompileResult<Vec<_>>>()?,
            laplace_filters: Vec::new(),
            zi_filters: Vec::new(),
            zi_filter_definitions: Vec::new(),
            noise_process_schema: 1,
            noise_sources: Vec::new(),
            reaching_snapshots: ir.reaching_snapshots.clone(),
        };

        // Generate evaluation steps (executed in order before contributions)
        let phase_start = web_time::Instant::now();
        // The ordinary forest is the compile's largest allocation and nothing
        // after this pass reads it, so it is handed over rather than borrowed:
        // each item's boxed trees are freed the moment they are in the arena,
        // and the list itself is gone when the call returns. That is what lets
        // the largest shipped models compile inside a 32 GB box.
        model.assignment_steps =
            self.compile_assignment_items(std::mem::take(&mut ir.assignments), &arena, &emit_ctx)?;
        // A mirrored noise pass *is* the ordinary pass, so say so by leaving the
        // list empty rather than by carrying a copy of it
        // ([`CompiledModel::noise_assignment_steps`] documents the convention;
        // the replay's two callers resolve it). Emitting it a second time
        // produced the same instructions with a second set of per-emission slot
        // numbers, which `state_renumbering` then rewrote back onto the first
        // pass's sites — the two lists were identical in every model that left
        // this compiler, and the twin was half of a large model's retained size.
        // The site-keyed families (`laplace`, `zi`, `$table_model`, `absdelay`,
        // `transition`, `slew`) are untouched, because those dedupe by site and
        // a second emission never allocated from them either. What does move is
        // the per-emission numbering of everything compiled after this pass:
        // equations and noise sources now start where the ordinary pass ended
        // rather than where a second emission of it ended. That shift is uniform
        // across every per-emission family, so the k-th emission is still the
        // k-th site and the renumbered slots are the ones they always were.
        model.noise_assignment_steps = if ir.noise_assignments_mirror_ordinary {
            Vec::new()
        } else {
            self.compile_assignment_items(
                std::mem::take(&mut ir.noise_assignments),
                &arena,
                &emit_ctx,
            )?
        };
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
        // The equations are consumed here, one at a time, so the one fact the
        // noise pass still needs of them — the sign its residual stamps with —
        // is read off first.
        let equation_rhs_signs = ir
            .equations
            .iter()
            .map(|eq| {
                if eq.indirect || eq.is_current {
                    -1.0
                } else {
                    1.0
                }
            })
            .collect::<Vec<f64>>();
        let equations = std::mem::take(&mut ir.equations);
        let equation_count = equations.len();
        for eq in equations {
            let program = self.compile_equation(eq, num_terminals, &arena, &emit_ctx)?;
            model.stamp_programs.push(program);
        }
        if timings {
            eprintln!(
                "timing codegen.equations module={} elapsed={:.3}s equations={} programs={}",
                ir.name,
                phase_start.elapsed().as_secs_f64(),
                equation_count,
                model.stamp_programs.len()
            );
        }

        // Compile noise-source PSD programs (evaluated at the operating
        // point during noise analysis)
        let phase_start = web_time::Instant::now();
        for source in std::mem::take(&mut ir.noise_sources) {
            let psd_program = self.compile_expr(&arena, source.psd, &emit_ctx)?;
            let exponent_program = source
                .exponent
                .map(|e| self.compile_expr(&arena, e, &emit_ctx))
                .transpose()?;
            let injections = source
                .injections
                .into_iter()
                .map(|injection| {
                    let rhs_sign = equation_rhs_signs[injection.equation_index];
                    Ok(crate::codegen::CompiledNoiseInjection {
                        pos: Self::node_stamp_index(num_terminals, injection.branch.pos_terminal),
                        neg: Self::node_stamp_index(num_terminals, injection.branch.neg_terminal),
                        is_current: injection.is_current,
                        branch_ordinal: injection.branch_ordinal,
                        program_idx: injection.equation_index,
                        rhs_sign,
                        gain_program: self.compile_expr(&arena, injection.gain, &emit_ctx)?,
                    })
                })
                .collect::<CompileResult<Vec<_>>>()?;
            model.noise_sources.push(CompiledNoiseSource {
                process_id: source.process_id,
                pos: Self::node_stamp_index(num_terminals, source.branch.pos_terminal),
                neg: Self::node_stamp_index(num_terminals, source.branch.neg_terminal),
                is_current: source.is_current,
                branch_ordinal: source.branch_ordinal,
                program_idx: source.equation_index,
                psd_program,
                exponent_program,
                table: source.table.map(|t| (t.points, t.log_interp)),
                name: source.name,
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
        model.requires_nodeset_phase = self.requires_nodeset_phase.get();

        Ok(model)
    }

    /// Compile assignment items (assignments and runtime loops) to steps
    ///
    /// The list is taken by value and consumed item by item, so its spine is
    /// freed with the call; the expressions themselves are node ids into the
    /// IR's arena and are only read.
    fn compile_assignment_items(
        &self,
        items: Vec<crate::ir::IrAssignmentItem>,
        arena: &ExprArena,
        emit_ctx: &EmitContext,
    ) -> CompileResult<Vec<AssignmentStep>> {
        let mut steps = Vec::with_capacity(items.len());
        for item in items {
            steps.push(match item {
                crate::ir::IrAssignmentItem::Initialization { phase, body } => {
                    let before = self.requires_nodeset_phase.get();
                    let body = self.compile_assignment_items(body, arena, emit_ctx);
                    self.requires_nodeset_phase.set(before);
                    AssignmentStep::Initialization { phase, body: body? }
                }
                crate::ir::IrAssignmentItem::Task(task) => {
                    AssignmentStep::Task(task.try_map(task.span, |expression| {
                        self.compile_expr(arena, *expression, emit_ctx)
                    })?)
                }
                crate::ir::IrAssignmentItem::Assign(assign) => {
                    let program = self.compile_expr(arena, assign.expr, emit_ctx)?;
                    match assign.index {
                        Some(target) => AssignmentStep::AssignIndexed {
                            base: assign.var_index,
                            len: target.len,
                            lower: target.lower,
                            index: self.compile_expr(arena, target.index, emit_ctx)?,
                            value: program,
                        },
                        None => AssignmentStep::Assign(AssignmentProgram {
                            var_index: assign.var_index,
                            program,
                        }),
                    }
                }
                crate::ir::IrAssignmentItem::Loop { condition, body } => {
                    let condition = self.compile_expr(arena, condition, emit_ctx)?;
                    let body = self.compile_assignment_items(body, arena, emit_ctx)?;
                    AssignmentStep::Loop { condition, body }
                }
            });
        }
        Ok(steps)
    }

    /// Map a unified node index (terminals, then internal nodes, ground
    /// sentinel) to a stamp index
    ///
    /// The terminal count is the whole of the IR this needs, and passing it
    /// alone is what lets an equation's trees be moved out of the IR while
    /// its stamps are laid out.
    fn node_stamp_index(num_terminals: usize, node: usize) -> StampIndex {
        if node == crate::expr_converter::GROUND_NODE {
            StampIndex::Ground
        } else if node < num_terminals {
            StampIndex::Terminal(node)
        } else {
            StampIndex::Internal(node - num_terminals)
        }
    }

    /// Map a derivative axis to its stamp column and column-axis record
    fn axis_stamp_column(num_terminals: usize, wrt: &DerivativeWrt) -> (StampIndex, ColumnAxis) {
        match wrt {
            DerivativeWrt::Voltage(node) => (
                Self::node_stamp_index(num_terminals, *node),
                ColumnAxis::Node(*node),
            ),
            DerivativeWrt::BranchCurrent(k) => (StampIndex::Branch(*k), ColumnAxis::Branch(*k)),
            DerivativeWrt::Noise(_) | DerivativeWrt::LimiterCorrection => {
                unreachable!("auxiliary derivatives are not matrix Jacobian columns")
            }
        }
    }

    /// Compile a branch equation to a stamp program
    ///
    /// The equation is taken by value so its derivative list is freed one
    /// program at a time; the expressions are node ids into the IR's arena.
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
        eq: BranchEquation,
        num_terminals: usize,
        arena: &ExprArena,
        emit_ctx: &EmitContext,
    ) -> CompileResult<StampProgram> {
        let value_program = self.compile_expr(arena, eq.expr, emit_ctx)?;
        let limiter_correction = eq
            .limiter_correction
            .map(|expr| self.compile_expr(arena, expr, emit_ctx))
            .transpose()?;
        let static_condition = eq
            .static_condition
            .map(|cond| self.compile_expr(arena, cond, emit_ctx))
            .transpose()?;

        let pos = Self::node_stamp_index(num_terminals, eq.branch.pos_terminal);
        let neg = Self::node_stamp_index(num_terminals, eq.branch.neg_terminal);

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
            for deriv in eq.derivatives {
                let (col, col_axis) = Self::axis_stamp_column(num_terminals, &deriv.wrt);
                let program = self.compile_expr(arena, deriv.expr, emit_ctx)?;
                jacobian_programs.push(JacobianEntry {
                    row: branch_row.clone(),
                    col,
                    col_axis,
                    sign: 1.0,
                    program,
                });
            }

            let mut reactive_jacobians = Vec::new();
            for deriv in eq.reactive_derivatives {
                let (col, col_axis) = Self::axis_stamp_column(num_terminals, &deriv.wrt);
                let program = self.compile_expr(arena, deriv.expr, emit_ctx)?;
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
                limiter_correction,
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
            for deriv in eq.derivatives {
                let (col, col_axis) = Self::axis_stamp_column(num_terminals, &deriv.wrt);
                let program = self.compile_expr(arena, deriv.expr, emit_ctx)?;
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
            for deriv in eq.reactive_derivatives {
                let (col, col_axis) = Self::axis_stamp_column(num_terminals, &deriv.wrt);
                let program = self.compile_expr(arena, deriv.expr, emit_ctx)?;
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
                limiter_correction,
                jacobian_programs,
                reactive_jacobians,
                branch_ordinal: Some(ordinal),
                indirect: false,
                static_condition,
            });
        }

        // Current contribution
        for deriv in eq.derivatives {
            let (col, col_axis) = Self::axis_stamp_column(num_terminals, &deriv.wrt);
            let program = self.compile_expr(arena, deriv.expr, emit_ctx)?;

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
        for deriv in eq.reactive_derivatives {
            let (col, col_axis) = Self::axis_stamp_column(num_terminals, &deriv.wrt);
            let program = self.compile_expr(arena, deriv.expr, emit_ctx)?;
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
            limiter_correction,
            jacobian_programs,
            reactive_jacobians,
            branch_ordinal: None,
            indirect: false,
            static_condition,
        })
    }

    /// Compile one arena expression to bytecode
    fn compile_expr(
        &self,
        arena: &ExprArena,
        id: NodeId,
        emit_ctx: &EmitContext,
    ) -> CompileResult<BytecodeProgram> {
        let mut program = BytecodeProgram::default();
        self.emit_expr(arena, id, emit_ctx, &mut program)?;
        // Grown by `push`, a program's vector carries up to half its length
        // again in slack; over the eighteen million instructions of a large
        // compact model that was 0.4 GB per assignment pass held for the
        // model's whole lifetime.
        program.instructions.shrink_to_fit();
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
        arena: &ExprArena,
        definition: &ZiPolynomial,
        emit_ctx: &EmitContext,
    ) -> CompileResult<CompiledZiPolynomial> {
        Ok(match definition {
            ZiPolynomial::Coefficients(values) => CompiledZiPolynomial::Coefficients(
                values
                    .iter()
                    .map(|value| self.compile_expr(arena, *value, emit_ctx))
                    .collect::<CompileResult<Vec<_>>>()?,
            ),
            ZiPolynomial::Roots(values) => CompiledZiPolynomial::Roots(
                values
                    .iter()
                    .map(|(real, imaginary)| {
                        Ok((
                            self.compile_expr(arena, *real, emit_ctx)?,
                            self.compile_expr(arena, *imaginary, emit_ctx)?,
                        ))
                    })
                    .collect::<CompileResult<Vec<_>>>()?,
            ),
        })
    }

    fn emit_zi_polynomial_operands(
        &self,
        arena: &ExprArena,
        definition: &ZiPolynomial,
        emit_ctx: &EmitContext,
        program: &mut BytecodeProgram,
    ) -> CompileResult<ZiPolynomialLayout> {
        match definition {
            ZiPolynomial::Coefficients(values) => {
                for value in values {
                    self.emit_expr(arena, *value, emit_ctx, program)?;
                }
                Ok(ZiPolynomialLayout::Coefficients { len: values.len() })
            }
            ZiPolynomial::Roots(values) => {
                for (real, imaginary) in values {
                    self.emit_expr(arena, *real, emit_ctx, program)?;
                    self.emit_expr(arena, *imaginary, emit_ctx, program)?;
                }
                Ok(ZiPolynomialLayout::Roots { len: values.len() })
            }
        }
    }

    fn zi_site_slot(
        &self,
        arena: &ExprArena,
        site: crate::ir::ZiSiteId,
        numerator: &ZiPolynomial,
        denominator: &ZiPolynomial,
        period: NodeId,
        first_transition: NodeId,
        emit_ctx: &EmitContext,
    ) -> CompileResult<usize> {
        let polynomial_layout = |definition: &ZiPolynomial| match definition {
            ZiPolynomial::Coefficients(values) => {
                ZiPolynomialLayout::Coefficients { len: values.len() }
            }
            ZiPolynomial::Roots(values) => ZiPolynomialLayout::Roots { len: values.len() },
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
            numerator: self.compile_zi_polynomial(arena, numerator, emit_ctx)?,
            denominator: self.compile_zi_polynomial(arena, denominator, emit_ctx)?,
            period: self.compile_expr(arena, period, emit_ctx)?,
            first_transition: self.compile_expr(arena, first_transition, emit_ctx)?,
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

    /// The index one parameter reference resolves to
    fn parameter_index(name: &SmolStr, emit_ctx: &EmitContext) -> CompileResult<usize> {
        emit_ctx
            .parameter_indices
            .get(name)
            .copied()
            .ok_or_else(|| {
                CodeGenError::new(CodeGenErrorKind::Internal(format!(
                    "Unknown parameter: {}",
                    name
                )))
                .into()
            })
    }

    /// The instruction one built-in call lowers to
    fn call_instruction(func: IrFunction, argc: usize) -> CompileResult<Instruction> {
        Ok(match func {
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
            IrFunction::Hypot => Instruction::Hypot,
            IrFunction::SumProductsDiv => {
                if argc < 3 || argc.is_multiple_of(2) {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "sum-products quotient requires product pairs and one divisor".into(),
                    ))
                    .into());
                }
                Instruction::SumProductsDiv((argc - 1) / 2)
            }
            // Rounding
            IrFunction::Floor => Instruction::Floor,
            IrFunction::Ceil => Instruction::Ceil,
            // Power
            IrFunction::Pow => Instruction::FnPow,
        })
    }

    /// Emit bytecode for the arena expression rooted at `id`
    ///
    /// The traversal is the boxed tree's, slot for slot: every child is
    /// emitted in the field order it was declared in and the node's own
    /// instruction goes last. That order is an identity obligation rather
    /// than a convenience — the per-emission state slots
    /// (`limit_state_count`, `cross_detector_count`, `timer_state_count`) and
    /// a Zi site's sub-programs are handed out *at the visit*, so a subtree
    /// two parents share is emitted once per path, exactly as its two copies
    /// were emitted before the arena existed.
    ///
    /// The event, noise and companion operands are reached explicitly here.
    /// [`crate::ir::arena::for_each_child`] stops at them because the generic
    /// walks do — those operands are compiled into programs of their own — so
    /// this walk is deliberately not written over it.
    fn emit_expr(
        &self,
        arena: &ExprArena,
        id: NodeId,
        emit_ctx: &EmitContext,
        program: &mut BytecodeProgram,
    ) -> CompileResult<()> {
        // Walk arithmetic chains without reserving this emitter's full frame
        // per operand. A node is still emitted once per path, left before right;
        // no DAG deduplication may change state-slot allocation or evaluation.
        let mut pending = Vec::new();
        let mut next = Some((id, false));
        while let Some((id, children_emitted)) = next {
            match *arena.node(id) {
                Node::Const(v) => {
                    program.instructions.push(Instruction::PushConst(v));
                }
                Node::Param(name) => {
                    let idx = Self::parameter_index(arena.name(name), emit_ctx)?;
                    program.instructions.push(Instruction::PushParam(idx));
                }
                Node::ParamGiven(name) => {
                    let idx = Self::parameter_index(arena.name(name), emit_ctx)?;
                    program.instructions.push(Instruction::PushParamGiven(idx));
                }
                Node::Var(name) => {
                    let name = arena.name(name);
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
                Node::VarIndexed { payload, index } => {
                    self.emit_expr(arena, index, emit_ctx, program)?;
                    let read = arena.indexed(payload);
                    program.instructions.push(Instruction::PushVariableDyn {
                        base: read.base,
                        len: read.len,
                        lower: read.lower,
                    });
                }
                Node::Voltage(p, n) => {
                    program
                        .instructions
                        .push(Instruction::PushVoltage(unpack_index(p), unpack_index(n)));
                }
                Node::Current(p, n) => {
                    program
                        .instructions
                        .push(Instruction::PushCurrent(unpack_index(p), unpack_index(n)));
                }
                Node::BranchCurrent(k) => {
                    program
                        .instructions
                        .push(Instruction::PushBranchCurrent(unpack_index(k)));
                }
                Node::Temperature => {
                    program.instructions.push(Instruction::PushTemperature);
                }
                Node::Vt => {
                    program.instructions.push(Instruction::PushVt);
                }
                Node::Time => {
                    program.instructions.push(Instruction::PushTime);
                }
                Node::SimParamValue(parameter) => {
                    program
                        .instructions
                        .push(Instruction::PushSimParamValue(parameter));
                }
                Node::SimParamPresent(parameter) => {
                    program
                        .instructions
                        .push(Instruction::PushSimParamPresent(parameter));
                }
                Node::Mfactor => {
                    program.instructions.push(Instruction::PushMfactor);
                }
                Node::PortConnected(index) => {
                    program
                        .instructions
                        .push(Instruction::PushPortConnected(unpack_index(index)));
                }
                Node::Binary(_, left, right) if !children_emitted => {
                    pending.push((id, true));
                    pending.push((right, false));
                    next = Some((left, false));
                    continue;
                }
                Node::Binary(op, _, _) => {
                    program.instructions.push(match op {
                        // Arithmetic
                        BinaryOp::IntAdd
                        | BinaryOp::IntSub
                        | BinaryOp::IntMul
                        | BinaryOp::IntDiv
                        | BinaryOp::IntMod
                        | BinaryOp::IntPow => Instruction::IntegerArithmetic(
                            op.integer_arithmetic().expect("integer operation"),
                        ),
                        BinaryOp::CheckedValue => Instruction::CheckedValue,
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
                Node::Unary(_, inner) if !children_emitted => {
                    pending.push((id, true));
                    next = Some((inner, false));
                    continue;
                }
                Node::Unary(op, _) => {
                    match op {
                        crate::ast::UnaryOp::ToInteger => {
                            program.instructions.push(Instruction::PushConst(0.0));
                            program.instructions.push(Instruction::BitOr);
                        }
                        crate::ast::UnaryOp::Neg => program.instructions.push(Instruction::Neg),
                        // Unary plus is the identity
                        crate::ast::UnaryOp::Pos => {}
                        crate::ast::UnaryOp::Not => program.instructions.push(Instruction::Not),
                        // Bitwise complement is represented through the shared
                        // integer conversion and 32-bit XOR contract: ~x == x ^ -1.
                        crate::ast::UnaryOp::BitNot => {
                            program.instructions.push(Instruction::PushConst(-1.0));
                            program.instructions.push(Instruction::BitXor);
                        }
                    }
                }
                Node::Call { func, argc, a, b } => {
                    if let Some(a) = a {
                        self.emit_expr(arena, a, emit_ctx, program)?;
                    }
                    if let Some(b) = b {
                        self.emit_expr(arena, b, emit_ctx, program)?;
                    }
                    program
                        .instructions
                        .push(Self::call_instruction(func, usize::from(argc))?);
                }
                // Variable-arity internal arithmetic retains every operand.
                // Source-call arities are checked before reaching this IR.
                Node::CallSpilled { func, args } => {
                    for arg in arena.call_args(args) {
                        self.emit_expr(arena, *arg, emit_ctx, program)?;
                    }
                    program
                        .instructions
                        .push(Self::call_instruction(func, arena.call_args(args).len())?);
                }
                Node::Limexp(inner) => {
                    self.emit_expr(arena, inner, emit_ctx, program)?;
                    program.instructions.push(Instruction::Limexp);
                }
                Node::Conditional(cond, then_expr, else_expr) => {
                    self.emit_expr(arena, cond, emit_ctx, program)?;
                    let branch = program.instructions.len();
                    program.instructions.push(Instruction::JumpIfFalse(0));
                    self.emit_expr(arena, then_expr, emit_ctx, program)?;
                    let jump = program.instructions.len();
                    program.instructions.push(Instruction::Jump(0));
                    program.instructions[branch] = Instruction::JumpIfFalse(jump - branch);
                    self.emit_expr(arena, else_expr, emit_ctx, program)?;
                    program.instructions[jump] =
                        Instruction::Jump(program.instructions.len() - jump - 1);
                }
                Node::Ddt(inner) => {
                    // Backward-Euler time derivative with a dedicated state slot:
                    // (value - prev_value) / dt in transient, 0 at DC. The state
                    // slot records the operand so the next step has its history.
                    self.emit_expr(arena, inner, emit_ctx, program)?;
                    let state_id = emit_ctx.integration_slot(id, &self.limit_state_count);
                    program.instructions.push(Instruction::DdtState(state_id));
                }
                Node::Idt(inner, ic) => {
                    // Time integral: state + value*dt in transient; the initial
                    // condition (default 0) seeds the integral at DC/IC.
                    self.emit_expr(arena, inner, emit_ctx, program)?;
                    if let Some(ic_expr) = ic {
                        self.emit_expr(arena, ic_expr, emit_ctx, program)?;
                    } else {
                        program.instructions.push(Instruction::PushConst(0.0));
                    }
                    let state_id = emit_ctx.integration_slot(id, &self.limit_state_count);
                    program.instructions.push(Instruction::IdtState(state_id));
                }
                Node::IdtMod {
                    expr,
                    modulus,
                    payload,
                } => {
                    let (ic, offset) = arena.optional_pair(payload);
                    self.emit_expr(arena, expr, emit_ctx, program)?;
                    match ic {
                        Some(ic) => self.emit_expr(arena, ic, emit_ctx, program)?,
                        None => program.instructions.push(Instruction::PushConst(0.0)),
                    }
                    self.emit_expr(arena, modulus, emit_ctx, program)?;
                    match offset {
                        Some(offset) => self.emit_expr(arena, offset, emit_ctx, program)?,
                        None => program.instructions.push(Instruction::PushConst(0.0)),
                    }
                    let state_id = emit_ctx.integration_slot(id, &self.limit_state_count);
                    program
                        .instructions
                        .push(Instruction::IdtModState(state_id));
                }
                Node::FreezeDerivative(inner) => {
                    self.emit_expr(arena, inner, emit_ctx, program)?;
                }
                Node::DdtCompanion(inner) => {
                    // Jacobian companion factor: operand / dt (0 at DC)
                    self.emit_expr(arena, inner, emit_ctx, program)?;
                    program.instructions.push(Instruction::DdtJacobian);
                }
                Node::IdtCompanion(inner) => {
                    // Jacobian companion factor: operand * dt (0 at DC)
                    self.emit_expr(arena, inner, emit_ctx, program)?;
                    program.instructions.push(Instruction::IdtJacobian);
                }
                Node::TableDerivative { input, table } => {
                    self.emit_expr(arena, input, emit_ctx, program)?;
                    let (x_data, y_data) = arena.table(table);
                    let table_id = self.register_lookup_table(x_data, y_data)?;
                    program
                        .instructions
                        .push(Instruction::TableDerivative(table_id));
                }
                Node::Ddx { .. } => {
                    return Err(CompileError::CodeGen(CodeGenError::new(
                        CodeGenErrorKind::Internal(
                            "unresolved ddx() reached code generation".into(),
                        ),
                    )));
                }
                Node::Limit(inner, step) => {
                    // $limit(expr, step) - bounds value change per Newton iteration
                    // For DC, we track previous value and limit the step
                    self.emit_expr(arena, inner, emit_ctx, program)?;
                    if let Some(step_expr) = step {
                        self.emit_expr(arena, step_expr, emit_ctx, program)?;
                    } else {
                        // Default step limit for pn-junction type limiting
                        program.instructions.push(Instruction::PushConst(0.7)); // ~2*Vt
                    }
                    let state_id = emit_ctx.integration_slot(id, &self.limit_state_count);
                    program.instructions.push(Instruction::LimitState(state_id));
                }
                Node::CanonicalLimit(inner) => {
                    self.emit_expr(arena, inner, emit_ctx, program)?;
                    let state_id = emit_ctx.integration_slot(id, &self.limit_state_count);
                    program
                        .instructions
                        .push(Instruction::CanonicalLimitState(state_id));
                }
                Node::TableLookup { input, table } => {
                    // $table_model lookup with linear interpolation
                    // Emit input expression, then TableLookup instruction referencing the table
                    self.emit_expr(arena, input, emit_ctx, program)?;
                    let (x_data, y_data) = arena.table(table);
                    let table_id = self.register_lookup_table(x_data, y_data)?;
                    program
                        .instructions
                        .push(Instruction::TableLookup(table_id));
                }
                Node::LastCrossing { expr, direction } => {
                    self.emit_expr(arena, expr, emit_ctx, program)?;
                    if let Some(direction) = direction {
                        self.emit_expr(arena, direction, emit_ctx, program)?;
                    } else {
                        program.instructions.push(Instruction::PushConst(0.0));
                    }
                    let detector_id = Self::allocate_slot(&self.cross_detector_count);
                    program
                        .instructions
                        .push(Instruction::LastCrossingState(detector_id));
                }
                Node::Analysis(name) => {
                    // analysis(name) - check current analysis type
                    let name = arena.name(name);
                    if name.eq_ignore_ascii_case("nodeset") {
                        self.requires_nodeset_phase.set(true);
                    }
                    let instruction = rspice_veriloga_runtime::analysis_query_id(name)
                        .map(Instruction::Analysis)
                        .unwrap_or(Instruction::PushConst(0.0));
                    program.instructions.push(instruction);
                }
                Node::Heavy(_, heavy) => {
                    self.emit_heavy(arena, arena.heavy(heavy), emit_ctx, program)?
                }
            }
            next = pending.pop();
        }
        Ok(())
    }

    /// Emit bytecode for one site-bearing, event, noise or filter operator
    ///
    /// Split out of [`Self::emit_expr`] because these eighteen payloads live
    /// beside the nodes rather than inside them. The emission order within
    /// each is the boxed variant's field order, unchanged.
    fn emit_heavy(
        &self,
        arena: &ExprArena,
        heavy: &Heavy,
        emit_ctx: &EmitContext,
        program: &mut BytecodeProgram,
    ) -> CompileResult<()> {
        match heavy {
            Heavy::IntegralDerivative {
                primal,
                input_derivative,
                ic_derivative,
                modulus_derivative,
            } => {
                self.emit_expr(arena, *primal, emit_ctx, program)?;
                let state_id = emit_ctx.integration_slot(*primal, &self.limit_state_count);
                let wrapped = match (*arena.node(*primal), modulus_derivative) {
                    (Node::Idt(_, _), None) => false,
                    (
                        Node::IdtMod {
                            modulus, payload, ..
                        },
                        Some(_),
                    ) => {
                        self.emit_expr(arena, modulus, emit_ctx, program)?;
                        if let Some(offset) = arena.optional_pair(payload).1 {
                            self.emit_expr(arena, offset, emit_ctx, program)?;
                        } else {
                            program.instructions.push(Instruction::PushConst(0.0));
                        }
                        true
                    }
                    _ => {
                        return Err(CodeGenError::new(CodeGenErrorKind::Internal(
                            "integral derivative has an incompatible primal".into(),
                        ))
                        .into());
                    }
                };
                self.emit_expr(arena, *input_derivative, emit_ctx, program)?;
                self.emit_expr(arena, *ic_derivative, emit_ctx, program)?;
                if let Some(derivative) = modulus_derivative {
                    self.emit_expr(arena, *derivative, emit_ctx, program)?;
                }
                program.instructions.push(if wrapped {
                    Instruction::IdtModDerivativeState(state_id)
                } else {
                    Instruction::IdtDerivativeState(state_id)
                });
            }
            Heavy::AbsDelay {
                site,
                expr,
                delay_time,
                max_delay,
            } => {
                self.emit_expr(arena, *expr, emit_ctx, program)?;
                self.emit_expr(arena, *delay_time, emit_ctx, program)?;
                if let Some(max_delay) = max_delay {
                    self.emit_expr(arena, *max_delay, emit_ctx, program)?;
                }
                let buffer_id = self.absdelay_site_slot(*site);
                program.instructions.push(if max_delay.is_some() {
                    Instruction::AbsDelayStateMax(buffer_id)
                } else {
                    Instruction::AbsDelayState(buffer_id)
                });
            }
            Heavy::AbsDelayDerivative {
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
                self.emit_expr(arena, *input, emit_ctx, program)?;
                self.emit_expr(arena, *input_derivative, emit_ctx, program)?;
                self.emit_expr(arena, *delay_time, emit_ctx, program)?;
                self.emit_expr(arena, *delay_derivative, emit_ctx, program)?;
                if let Some(max_delay) = max_delay {
                    self.emit_expr(arena, *max_delay, emit_ctx, program)?;
                }
                let buffer_id = self.absdelay_site_slot(*site);
                program.instructions.push(if max_delay.is_some() {
                    Instruction::AbsDelayStateDerivativeMax(buffer_id)
                } else {
                    Instruction::AbsDelayStateDerivative(buffer_id)
                });
            }
            Heavy::Transition {
                site,
                expr,
                delay,
                rise_time,
                fall_time,
            } => {
                // transition(expr, delay, rise_time, fall_time)
                self.emit_expr(arena, *expr, emit_ctx, program)?;
                // Emit delay (default 0)
                if let Some(d) = delay {
                    self.emit_expr(arena, *d, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                // Semantic lowering normally materializes the module-scoped
                // rise default; retain an instantaneous defensive default for
                // directly constructed IR.
                if let Some(r) = rise_time {
                    self.emit_expr(arena, *r, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                // An omitted fall time reuses the effective rise expression.
                if let Some(f) = fall_time {
                    self.emit_expr(arena, *f, emit_ctx, program)?;
                } else if let Some(r) = rise_time {
                    self.emit_expr(arena, *r, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                let filter_id = self.transition_site_slot(*site);
                program
                    .instructions
                    .push(Instruction::TransitionState(filter_id));
            }
            Heavy::TransitionDerivative {
                site,
                input,
                input_derivative,
                delay,
                rise_time,
                fall_time,
            } => {
                self.emit_expr(arena, *input, emit_ctx, program)?;
                self.emit_expr(arena, *input_derivative, emit_ctx, program)?;
                if let Some(delay) = delay {
                    self.emit_expr(arena, *delay, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(rise_time) = rise_time {
                    self.emit_expr(arena, *rise_time, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(fall_time) = fall_time {
                    self.emit_expr(arena, *fall_time, emit_ctx, program)?;
                } else if let Some(rise_time) = rise_time {
                    self.emit_expr(arena, *rise_time, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                let filter_id = self.transition_site_slot(*site);
                program
                    .instructions
                    .push(Instruction::TransitionStateDerivative(filter_id));
            }
            Heavy::Slew {
                site,
                expr,
                max_pos_slew,
                max_neg_slew,
            } => {
                // With no authored rates the LRM defines an exact passthrough;
                // do not allocate or touch state in that form.
                if max_pos_slew.is_none() && max_neg_slew.is_none() {
                    self.emit_expr(arena, *expr, emit_ctx, program)?;
                    return Ok(());
                }
                self.emit_expr(arena, *expr, emit_ctx, program)?;
                let positive = max_pos_slew.ok_or_else(|| {
                    CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "slew negative rate cannot be authored without a positive rate".into(),
                    ))
                })?;
                self.emit_expr(arena, positive, emit_ctx, program)?;
                if let Some(n) = max_neg_slew {
                    self.emit_expr(arena, *n, emit_ctx, program)?;
                } else {
                    self.emit_expr(arena, positive, emit_ctx, program)?;
                    program.instructions.push(Instruction::Neg);
                }
                let filter_id = self.slew_site_slot(*site);
                program.instructions.push(Instruction::SlewState(filter_id));
            }
            Heavy::SlewDerivative {
                site,
                input,
                input_derivative,
                max_pos_slew,
                max_pos_slew_derivative,
                max_neg_slew,
                max_neg_slew_derivative,
            } => {
                if max_pos_slew.is_none() && max_neg_slew.is_none() {
                    self.emit_expr(arena, *input_derivative, emit_ctx, program)?;
                    return Ok(());
                }
                let positive = max_pos_slew.ok_or_else(|| {
                    CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "slew negative rate cannot be authored without a positive rate".into(),
                    ))
                })?;
                let positive_derivative = max_pos_slew_derivative.ok_or_else(|| {
                    CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "slew positive-rate derivative is missing".into(),
                    ))
                })?;
                self.emit_expr(arena, *input, emit_ctx, program)?;
                self.emit_expr(arena, *input_derivative, emit_ctx, program)?;
                self.emit_expr(arena, positive, emit_ctx, program)?;
                self.emit_expr(arena, positive_derivative, emit_ctx, program)?;
                if let (Some(negative), Some(negative_derivative)) =
                    (*max_neg_slew, *max_neg_slew_derivative)
                {
                    self.emit_expr(arena, negative, emit_ctx, program)?;
                    self.emit_expr(arena, negative_derivative, emit_ctx, program)?;
                } else {
                    self.emit_expr(arena, positive, emit_ctx, program)?;
                    program.instructions.push(Instruction::Neg);
                    self.emit_expr(arena, positive_derivative, emit_ctx, program)?;
                    program.instructions.push(Instruction::Neg);
                }
                let filter_id = self.slew_site_slot(*site);
                program
                    .instructions
                    .push(Instruction::SlewStateDerivative(filter_id));
            }
            Heavy::Cross {
                expr,
                direction,
                time_tol,
                expr_tol,
                enable,
            } => {
                // cross(expr, direction, time_tol, expr_tol, enable)
                self.emit_expr(arena, *expr, emit_ctx, program)?;
                if let Some(direction) = direction {
                    self.emit_expr(arena, *direction, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(tolerance) = time_tol {
                    self.emit_expr(arena, *tolerance, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(tolerance) = expr_tol {
                    self.emit_expr(arena, *tolerance, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(enable) = enable {
                    self.emit_expr(arena, *enable, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(1.0));
                }
                let detector_id = Self::allocate_slot(&self.cross_detector_count);
                program
                    .instructions
                    .push(Instruction::CrossState(detector_id));
            }
            Heavy::Above {
                expr,
                time_tol,
                expr_tol,
                enable,
            } => {
                // above(expr, time_tol, expr_tol, enable)
                self.emit_expr(arena, *expr, emit_ctx, program)?;
                if let Some(tolerance) = time_tol {
                    self.emit_expr(arena, *tolerance, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(tolerance) = expr_tol {
                    self.emit_expr(arena, *tolerance, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(enable) = enable {
                    self.emit_expr(arena, *enable, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(1.0));
                }
                let detector_id = Self::allocate_slot(&self.cross_detector_count);
                program
                    .instructions
                    .push(Instruction::AboveState(detector_id));
            }
            Heavy::Timer {
                start_time,
                period,
                time_tol,
                enable,
            } => {
                // timer(start, period, time_tol, enable)
                self.emit_expr(arena, *start_time, emit_ctx, program)?;
                if let Some(p) = period {
                    self.emit_expr(arena, *p, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(tolerance) = time_tol {
                    self.emit_expr(arena, *tolerance, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                if let Some(enable) = enable {
                    self.emit_expr(arena, *enable, emit_ctx, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(1.0));
                }
                let timer_id = Self::allocate_slot(&self.timer_state_count);
                program.instructions.push(Instruction::TimerState(timer_id));
            }
            Heavy::WhiteNoise { .. } => {
                // The large-signal contribution is zero. The PSD operand is
                // compiled separately into model.noise_sources for noise
                // analysis and must not create stamp-time dependencies.
                program.instructions.push(Instruction::PushConst(0.0));
            }
            Heavy::FlickerNoise { .. } => {
                // The large-signal contribution is zero. PSD and exponent
                // programs are compiled separately for noise analysis.
                program.instructions.push(Instruction::PushConst(0.0));
            }
            Heavy::NoiseTable { .. } => {
                // Like the other noise functions, the large-signal value
                // is zero; the table feeds the noise-analysis sources
                program.instructions.push(Instruction::PushConst(0.0));
            }
            Heavy::LaplaceZP {
                site,
                expr,
                zeros,
                poles,
                gain,
            } => {
                self.emit_expr(arena, *expr, emit_ctx, program)?;

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
            Heavy::LaplaceND {
                site,
                expr,
                numerator,
                denominator,
            } => {
                self.emit_expr(arena, *expr, emit_ctx, program)?;

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
            Heavy::LaplaceZPDerivative {
                site,
                expr,
                zeros,
                poles,
                gain,
            } => {
                self.emit_expr(arena, *expr, emit_ctx, program)?;

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
            Heavy::LaplaceNDDerivative {
                site,
                expr,
                numerator,
                denominator,
            } => {
                self.emit_expr(arena, *expr, emit_ctx, program)?;

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
            Heavy::ZiFilter {
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
                    arena,
                    *site,
                    numerator,
                    denominator,
                    *period,
                    *first_transition,
                    emit_ctx,
                )?;
                let numerator =
                    self.emit_zi_polynomial_operands(arena, numerator, emit_ctx, program)?;
                let denominator =
                    self.emit_zi_polynomial_operands(arena, denominator, emit_ctx, program)?;
                self.emit_expr(arena, *period, emit_ctx, program)?;
                self.emit_expr(arena, *first_transition, emit_ctx, program)?;
                self.emit_expr(arena, *expr, emit_ctx, program)?;
                self.emit_expr(arena, *transition, emit_ctx, program)?;
                program
                    .instructions
                    .push(Instruction::ZiState(ZiRuntimeLayout {
                        filter_id,
                        numerator,
                        denominator,
                        direct_assignment: *direct_assignment,
                    }));
            }
            Heavy::ZiFilterDerivative {
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
                    arena,
                    *site,
                    numerator,
                    denominator,
                    *period,
                    *first_transition,
                    emit_ctx,
                )?;
                let numerator =
                    self.emit_zi_polynomial_operands(arena, numerator, emit_ctx, program)?;
                let denominator =
                    self.emit_zi_polynomial_operands(arena, denominator, emit_ctx, program)?;
                self.emit_expr(arena, *period, emit_ctx, program)?;
                self.emit_expr(arena, *first_transition, emit_ctx, program)?;
                self.emit_expr(arena, *expr, emit_ctx, program)?;
                self.emit_expr(arena, *transition, emit_ctx, program)?;
                program
                    .instructions
                    .push(Instruction::ZiStateDerivative(ZiRuntimeLayout {
                        filter_id,
                        numerator,
                        denominator,
                        direct_assignment: *direct_assignment,
                    }));
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
            crate::ir::IrAssignmentItem::Assign(_) | crate::ir::IrAssignmentItem::Task(_) => 1,
            crate::ir::IrAssignmentItem::Loop { body, .. }
            | crate::ir::IrAssignmentItem::Initialization { body, .. } => {
                1 + count_ir_assignment_items(body)
            }
        })
        .sum()
}

/// An [`EmitContext`] with nothing named, which is what every fixture below
/// wants.
#[cfg(test)]
fn empty_emit_context() -> EmitContext {
    EmitContext::default()
}

fn count_assignment_steps_for_timing(items: &[AssignmentStep]) -> usize {
    items
        .iter()
        .map(|item| match item {
            AssignmentStep::Assign(_)
            | AssignmentStep::AssignIndexed { .. }
            | AssignmentStep::Task(_) => 1,
            AssignmentStep::Loop { body, .. } | AssignmentStep::Initialization { body, .. } => {
                1 + count_assignment_steps_for_timing(body)
            }
        })
        .sum()
}

#[cfg(test)]
mod laplace_derivative_tests {
    use super::*;
    use crate::ir::LaplaceSiteId;
    use crate::ir::arena::Heavy;

    fn laplace_nd(arena: &mut ExprArena, site: LaplaceSiteId, derivative: bool) -> NodeId {
        let expr = arena.push(Node::Voltage(0, u32::MAX));
        let numerator = vec![1.0];
        let denominator = vec![1.0, 1.0];
        arena.push_heavy(if derivative {
            Heavy::LaplaceNDDerivative {
                site,
                expr,
                numerator,
                denominator,
            }
        } else {
            Heavy::LaplaceND {
                site,
                expr,
                numerator,
                denominator,
            }
        })
    }

    #[test]
    fn laplace_derivative_and_primal_share_a_slot_when_derivative_compiles_first() {
        let generator = CodeGenerator::new();
        let emit_context = empty_emit_context();
        let site = LaplaceSiteId::from_span(crate::source::Span::dummy());

        let arena = &mut ExprArena::new();
        let derivative_id = laplace_nd(arena, site, true);
        let primal_id = laplace_nd(arena, site, false);
        let derivative = generator
            .compile_expr(arena, derivative_id, &emit_context)
            .expect("compile derivative first");
        let primal = generator
            .compile_expr(arena, primal_id, &emit_context)
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
    use crate::ir::TransitionSiteId;
    use crate::ir::arena::Heavy;

    fn nonconstant_rise_time(arena: &mut ExprArena) -> NodeId {
        let left = arena.push(Node::Voltage(1, u32::MAX));
        let right = arena.push(Node::Const(0.5));
        arena.push(Node::Binary(BinaryOp::Add, left, right))
    }

    fn transition(arena: &mut ExprArena, site: TransitionSiteId, derivative: bool) -> NodeId {
        let input = arena.push(Node::Voltage(0, u32::MAX));
        let delay = Some(arena.push(Node::Const(0.25)));
        let rise_time = Some(nonconstant_rise_time(arena));
        let payload = if derivative {
            let input_derivative = arena.push(Node::Const(1.0));
            Heavy::TransitionDerivative {
                site,
                input,
                input_derivative,
                delay,
                rise_time,
                fall_time: None,
            }
        } else {
            Heavy::Transition {
                site,
                expr: input,
                delay,
                rise_time,
                fall_time: None,
            }
        };
        arena.push_heavy(payload)
    }

    #[test]
    fn transition_derivative_and_primal_share_slot_and_fall_default() {
        let generator = CodeGenerator::new();
        let emit_context = empty_emit_context();
        let site = TransitionSiteId::from_span(crate::source::Span::dummy());

        let arena = &mut ExprArena::new();
        let derivative_id = transition(arena, site, true);
        let primal_id = transition(arena, site, false);
        let derivative = generator
            .compile_expr(arena, derivative_id, &emit_context)
            .expect("compile transition derivative first");
        let primal = generator
            .compile_expr(arena, primal_id, &emit_context)
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
        let rise_id = nonconstant_rise_time(arena);
        let rise_ops: Vec<String> = generator
            .compile_expr(arena, rise_id, &emit_context)
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
    use crate::ir::arena::Heavy;
    use crate::ir::{DerivativeWrt, SlewSiteId, autodiff};

    fn slew(arena: &mut ExprArena, site: SlewSiteId, derivative: bool) -> NodeId {
        let input = arena.push(Node::Voltage(0, u32::MAX));
        let max_pos_slew = Some(arena.push(Node::Const(2.0)));
        let payload = if derivative {
            let input_derivative = arena.push(Node::Const(1.0));
            let max_pos_slew_derivative = Some(arena.push(Node::Const(0.0)));
            Heavy::SlewDerivative {
                site,
                input,
                input_derivative,
                max_pos_slew,
                max_pos_slew_derivative,
                max_neg_slew: None,
                max_neg_slew_derivative: None,
            }
        } else {
            Heavy::Slew {
                site,
                expr: input,
                max_pos_slew,
                max_neg_slew: None,
            }
        };
        arena.push_heavy(payload)
    }

    #[test]
    fn slew_derivative_and_primal_share_slot_when_derivative_compiles_first() {
        let generator = CodeGenerator::new();
        let emit_context = empty_emit_context();
        let site = SlewSiteId::from_span(crate::source::Span::dummy());

        let arena = &mut ExprArena::new();
        let derivative_id = slew(arena, site, true);
        let primal_id = slew(arena, site, false);
        let derivative = generator
            .compile_expr(arena, derivative_id, &emit_context)
            .expect("compile derivative first");
        let primal = generator
            .compile_expr(arena, primal_id, &emit_context)
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
        let emit_context = empty_emit_context();
        let arena = &mut ExprArena::new();
        let expr = arena.push(Node::Const(3.0));
        let id = arena.push_heavy(Heavy::Slew {
            site: SlewSiteId::from_span(crate::source::Span::dummy()),
            expr,
            max_pos_slew: None,
            max_neg_slew: None,
        });
        let program = generator
            .compile_expr(arena, id, &emit_context)
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
        let arena = &mut ExprArena::new();
        let voltage = arena.push(Node::Voltage(0, u32::MAX));
        let nonlinear_rate = arena.push(Node::Binary(BinaryOp::Mul, voltage, voltage));
        // Deliberately independent of the differentiation axis: the rate is
        // the only source of the saturated-branch Jacobian.
        let expr = arena.push(Node::Const(10.0));
        let primal = arena.push_heavy(Heavy::Slew {
            site,
            expr,
            max_pos_slew: Some(nonlinear_rate),
            max_neg_slew: None,
        });
        let first = autodiff::differentiate(arena, primal, &DerivativeWrt::Voltage(0));
        let second = autodiff::differentiate(arena, first, &DerivativeWrt::Voltage(0));

        let heavy = |arena: &ExprArena, id: NodeId| match *arena.node(id) {
            Node::Heavy(_, heavy) => arena.heavy(heavy).clone(),
            other => panic!("expected a heavy operator, found {other:?}"),
        };
        let Heavy::SlewDerivative {
            input_derivative,
            max_pos_slew_derivative,
            ..
        } = heavy(arena, first)
        else {
            panic!("first slew derivative must retain a branch action");
        };
        assert!(matches!(*arena.node(input_derivative), Node::Const(0.0)));
        assert!(
            !matches!(
                max_pos_slew_derivative.map(|id| *arena.node(id)),
                Some(Node::Const(0.0))
            ),
            "dynamic rate derivative must not be optimized to zero"
        );
        assert!(matches!(heavy(arena, second), Heavy::SlewDerivative { .. }));

        let generator = CodeGenerator::new();
        let emit_context = empty_emit_context();
        for derivative in [first, second] {
            let program = generator
                .compile_expr(arena, derivative, &emit_context)
                .expect("compile branch-exact slew derivative");
            assert!(matches!(
                program.instructions.last(),
                Some(Instruction::SlewStateDerivative(0))
            ));
        }
    }
}

#[cfg(test)]
mod limiter_correction_tests {
    use super::*;
    use crate::ir::{DerivativeWrt, autodiff};
    use crate::vm::{Vm, VmContext};

    #[test]
    fn limiter_correction_bytecode_preserves_physical_values_and_companion_rhs() {
        for (outer_step, limited) in [(None, 0.25_f64), (Some(0.125), 0.125)] {
            let arena = &mut ExprArena::new();
            let proposed = arena.push(Node::Voltage(0, u32::MAX));
            let step = arena.push(Node::Const(0.25));
            let mut value = arena.push(Node::Limit(proposed, Some(step)));
            if let Some(step) = outer_step {
                let step = arena.push(Node::Const(step));
                value = arena.push(Node::Limit(value, Some(step)));
            }
            let value = arena.push(Node::Binary(BinaryOp::Mul, value, value));
            let jacobian = autodiff::differentiate(arena, value, &DerivativeWrt::Voltage(0));
            let correction =
                autodiff::differentiate(arena, value, &DerivativeWrt::LimiterCorrection);
            let generator = CodeGenerator::new();
            let emit_context = empty_emit_context();
            let value = generator.compile_expr(arena, value, &emit_context).unwrap();
            let jacobian = generator
                .compile_expr(arena, jacobian, &emit_context)
                .unwrap();
            let correction = generator
                .compile_expr(arena, correction, &emit_context)
                .unwrap();
            for correction_first in [false, true] {
                let mut context = VmContext::new(1);
                context.begin_stateful_evaluation();
                assert_eq!(Vm::new(&mut context).execute(&value).unwrap(), 0.0);
                context.voltages[0] = 1.0;
                context.begin_stateful_evaluation();
                if correction_first {
                    Vm::new(&mut context).execute(&correction).unwrap();
                }
                let f = Vm::new(&mut context).execute(&value).unwrap();
                let g = Vm::new(&mut context).execute(&jacobian).unwrap();
                let c = Vm::new(&mut context).execute(&correction).unwrap();
                assert_eq!(f, limited * limited);
                assert_eq!(g, 2.0 * limited);
                assert_eq!(c, g * (limited - 1.0));
                assert_eq!(g - (f - c), limited * limited);
            }
        }
    }
}
