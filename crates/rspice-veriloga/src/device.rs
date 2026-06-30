//! Verilog-A Device Interface
//!
//! This module provides the runtime device interface for compiled Verilog-A models.
//! Devices can be instantiated in a circuit and stamped into the solver matrix.
//!
//! # Architecture
//!
//! ```text
//! CompiledModel (bytecode) + VmContext (runtime state)
//!         ↓
//! VerilogADevice (instance in circuit)
//!         ↓
//! stamp() → Matrix + RHS
//! ```
//!
//! # Native Compilation
//!
//! When the `native` feature is enabled, model construction requires a complete
//! native JIT image. Unsupported native compilation returns a typed error rather
//! than silently running the bytecode VM interpreter.
//!
//! The native path is a performance backend, not a separate public ABI. Keep all
//! raw-pointer and Send/Sync safety reasoning in the native module and require a
//! targeted audit before expanding that boundary.

use crate::canonical_ir::CanonicalIrArtifact;
use crate::codegen::{CompiledModel, Instruction, StampIndex};
#[cfg(feature = "native")]
use crate::vm::terminal_pair_current_endpoints;
use crate::vm::{CURRENT_PAIR_GROUND, Vm, VmContext, VmError};
use smol_str::SmolStr;

#[cfg(feature = "native")]
use crate::native::{
    NativeModel, NativeRequiredStorage, clear_native_runtime_error, take_native_runtime_error,
};

#[cfg(feature = "native")]
#[derive(Clone, Copy)]
enum NativeValueEntry {
    ParameterDefault(usize),
    StaticCondition(usize),
    StampValue(usize),
    Jacobian { stamp: usize, entry: usize },
    ReactiveJacobian { stamp: usize, entry: usize },
    NoisePsd(usize),
    NoiseExponent(usize),
}

#[cfg(feature = "native")]
#[derive(Clone, PartialEq, Eq)]
enum NativeCompileCacheKey {
    Bytecode,
    CanonicalMir(SmolStr),
}

/// A Verilog-A device instance in a circuit
///
/// Holds the compiled model, runtime context, and circuit connectivity.
#[derive(Debug, Clone)]
pub struct VerilogADevice {
    /// Device instance name
    pub name: SmolStr,
    /// Compiled model, shared between clones. Newton line-search snapshots
    /// clone devices per probe; sharing the (potentially megabyte-scale)
    /// program keeps that clone proportional to runtime state only.
    model: std::sync::Arc<CompiledModel>,
    /// Runtime execution context
    context: VmContext,
    /// Mapping from terminal index to circuit node ID (0 = ground)
    node_mapping: Vec<usize>,
    /// Mapping from internal node index to circuit node ID
    /// When the solver allocates circuit nodes for internal nodes, this maps them
    internal_node_indices: Vec<usize>,
    /// Number of internal nodes in this device
    num_internal_nodes: usize,
    /// Mapping from branch-current unknown ordinal to circuit node ID
    /// (the engine allocates one extra system unknown per potential
    /// contribution branch)
    branch_current_indices: Vec<usize>,
    /// Per stamp program: instance-static activation (parameter-only
    /// guards evaluated after parameter resolution)
    program_active: Vec<bool>,
    /// Per branch unknown: whether any potential contribution drives it
    /// (an undriven branch is forced to zero current)
    branch_active: Vec<bool>,
    /// Pre-computed matrix indices for O(1) stamping
    matrix_indices: MatrixIndices,
    /// Native compiled model. In native mode this is required: construction
    /// fails if a complete native image cannot be produced.
    #[cfg(feature = "native")]
    native_model: std::sync::Arc<NativeModel>,
    /// $discontinuity level at the last accepted timestep (edge detector)
    prev_discontinuity: bool,
}

// Safety: VerilogADevice owns per-instance VmContext and solver mapping state.
// The NativeModel is shared through Arc and immutable after native image
// construction. Calls supply mutable evaluation state through the device
// instance, so cloned devices used by line-search probes do not share
// VmContext mutation.
#[cfg(feature = "native")]
unsafe impl Send for VerilogADevice {}
#[cfg(feature = "native")]
unsafe impl Sync for VerilogADevice {}

/// Pre-computed matrix indices for fast stamping
#[derive(Debug, Clone, Default)]
pub struct MatrixIndices {
    /// Jacobian mappings grouped per stamp program.
    pub jacobian: Vec<Vec<JacobianIndex>>,
    /// Reactive (charge) Jacobian mappings grouped per stamp program.
    pub reactive: Vec<Vec<JacobianIndex>>,
    /// RHS mappings grouped per stamp program.
    pub rhs: Vec<Vec<RhsIndex>>,
}

/// Single Jacobian matrix entry index
#[derive(Debug, Clone)]
pub struct JacobianIndex {
    /// Row in circuit matrix (None = ground)
    pub row: Option<usize>,
    /// Column in circuit matrix (None = ground)
    pub col: Option<usize>,
    /// Index into stamp programs
    pub program_idx: usize,
    /// Index into Jacobian programs
    pub jacobian_idx: usize,
    /// Sign multiplier
    pub sign: f64,
}

/// Single RHS vector entry index
#[derive(Debug, Clone)]
pub struct RhsIndex {
    /// Node in circuit (None = ground)
    pub node: Option<usize>,
    /// Sign multiplier
    pub sign: f64,
    /// Index into stamp programs
    pub program_idx: usize,
}

impl VerilogADevice {
    /// Create a new device instance
    ///
    /// # Arguments
    /// * `name` - Instance name (e.g., "D1")
    /// * `model` - Compiled Verilog-A model. Pass an `Arc<CompiledModel>`
    ///   when instantiating a model many times: instances then share the
    ///   program and one JIT compilation.
    /// * `nodes` - Circuit node IDs for each terminal (0 = ground)
    pub fn new(
        name: impl Into<SmolStr>,
        model: impl Into<std::sync::Arc<CompiledModel>>,
        nodes: &[usize],
    ) -> Self {
        Self::try_new(name, model, nodes).unwrap_or_else(|err| {
            panic!("Verilog-A device construction failed: {}", err);
        })
    }

    /// Checked constructor for callers that can surface dependent-parameter
    /// default failures as diagnostics instead of panicking or accepting a
    /// zero default.
    pub fn try_new(
        name: impl Into<SmolStr>,
        model: impl Into<std::sync::Arc<CompiledModel>>,
        nodes: &[usize],
    ) -> Result<Self, VmError> {
        let model: std::sync::Arc<CompiledModel> = model.into();
        Self::try_new_inner(name, model, nodes, None)
    }

    /// Checked constructor that compiles stamp values from canonical MIR when
    /// the native backend is available. Unsupported MIR is a construction
    /// error; the bytecode stamp path is not used as a fallback.
    #[cfg(feature = "native")]
    pub fn try_new_with_canonical_ir(
        name: impl Into<SmolStr>,
        model: impl Into<std::sync::Arc<CompiledModel>>,
        artifact: &CanonicalIrArtifact,
        nodes: &[usize],
    ) -> Result<Self, VmError> {
        let model: std::sync::Arc<CompiledModel> = model.into();
        Self::try_new_inner(name, model, nodes, Some(artifact))
    }

    fn try_new_inner(
        name: impl Into<SmolStr>,
        model: std::sync::Arc<CompiledModel>,
        nodes: &[usize],
        canonical_artifact: Option<&CanonicalIrArtifact>,
    ) -> Result<Self, VmError> {
        #[cfg(not(feature = "native"))]
        let _ = canonical_artifact;

        let num_terminals = model.num_terminals;
        let supplied_terminals = nodes.len().min(num_terminals);

        // Build node mapping
        let mut node_mapping = vec![0; num_terminals];
        for (i, &node) in nodes.iter().enumerate() {
            if i < num_terminals {
                node_mapping[i] = node;
            }
        }

        // Create context with terminal count and internal nodes
        let num_internal_nodes = model.internal_nodes;
        let mut context = VmContext::with_internal_nodes(num_terminals, num_internal_nodes);
        context.port_connected = (0..num_terminals)
            .map(|terminal| u8::from(terminal < supplied_terminals))
            .collect();

        // Initialize parameters to their constant defaults; dependent
        // defaults are resolved after instance parameters are applied
        for (i, param) in model.parameters.iter().enumerate() {
            context.set_param(i, param.default);
        }
        context.param_given = vec![false; model.parameters.len()];
        context.variables.resize(model.num_variables, 0.0);
        // Stateful runtime data referenced by the bytecode lives in the
        // per-instance context (the model stays immutable and shared)
        context.lookup_tables = model.lookup_tables.clone();
        context.laplace_filters = model.laplace_filters.clone();
        context.zi_filters = model.zi_filters.clone();
        Self::preallocate_vm_runtime_state(&mut context, &model);

        // Attempt native compilation (if feature enabled)
        #[cfg(feature = "native")]
        let native_model = match canonical_artifact {
            Some(artifact) => Self::try_native_compile_with_canonical_ir(&model, artifact)?,
            None => Self::try_native_compile(&model)?,
        };

        let num_branch_unknowns = model.branch_sources.len();
        let num_stamp_programs = model.stamp_programs.len();
        let mut device = Self {
            name: name.into(),
            model,
            context,
            node_mapping,
            internal_node_indices: vec![0; num_internal_nodes],
            num_internal_nodes,
            branch_current_indices: vec![0; num_branch_unknowns],
            program_active: vec![true; num_stamp_programs],
            branch_active: vec![true; num_branch_unknowns],
            matrix_indices: MatrixIndices::default(),
            #[cfg(feature = "native")]
            native_model,
            prev_discontinuity: false,
        };
        device.context.branch_current_values = vec![0.0; num_branch_unknowns];
        device.rebuild_matrix_indices();
        device.try_resolve_parameter_defaults()?;
        Ok(device)
    }

    /// Preallocate interpreter runtime state vectors from bytecode instruction IDs.
    ///
    /// This avoids repeated dynamic growth during simulation hot paths and ensures
    /// stateful operators have stable dedicated slots.
    fn preallocate_vm_runtime_state(context: &mut VmContext, model: &CompiledModel) {
        #[inline]
        fn update_max(max_slot: &mut Option<usize>, idx: usize) {
            *max_slot = Some(max_slot.map_or(idx, |prev| prev.max(idx)));
        }

        let mut max_state = None;
        let mut max_delay_buffer = None;
        let mut max_transition_filter = None;
        let mut max_slew_filter = None;
        let mut max_cross_detector = None;

        let mut scan_program = |program: &crate::codegen::BytecodeProgram| {
            for instruction in &program.instructions {
                match instruction {
                    Instruction::DdtState(idx)
                    | Instruction::IdtState(idx)
                    | Instruction::IdtModState(idx)
                    | Instruction::LimitState(idx) => update_max(&mut max_state, *idx),
                    Instruction::AbsDelayState(idx) => update_max(&mut max_delay_buffer, *idx),
                    Instruction::TransitionState(idx) => {
                        update_max(&mut max_transition_filter, *idx)
                    }
                    Instruction::SlewState(idx) => update_max(&mut max_slew_filter, *idx),
                    Instruction::CrossState(idx) => update_max(&mut max_cross_detector, *idx),
                    _ => {}
                }
            }
        };

        fn scan_steps(
            steps: &[crate::codegen::AssignmentStep],
            scan_program: &mut impl FnMut(&crate::codegen::BytecodeProgram),
        ) {
            for step in steps {
                match step {
                    crate::codegen::AssignmentStep::Assign(assignment) => {
                        scan_program(&assignment.program);
                    }
                    crate::codegen::AssignmentStep::AssignIndexed { index, value, .. } => {
                        scan_program(index);
                        scan_program(value);
                    }
                    crate::codegen::AssignmentStep::Loop { condition, body } => {
                        scan_program(condition);
                        scan_steps(body, scan_program);
                    }
                }
            }
        }

        for parameter in &model.parameters {
            if let Some(program) = &parameter.default_program {
                scan_program(program);
            }
        }

        scan_steps(&model.assignment_steps, &mut scan_program);

        for stamp in &model.stamp_programs {
            if let Some(condition) = &stamp.static_condition {
                scan_program(condition);
            }
            scan_program(&stamp.value_program);
            for jac in &stamp.jacobian_programs {
                scan_program(&jac.program);
            }
            for jac in &stamp.reactive_jacobians {
                scan_program(&jac.program);
            }
        }

        for source in &model.noise_sources {
            scan_program(&source.psd_program);
            if let Some(program) = &source.exponent_program {
                scan_program(program);
            }
        }

        if let Some(max_idx) = max_state {
            context.allocate_states(max_idx + 1);
        }
        if let Some(max_idx) = max_delay_buffer {
            context.allocate_delay_buffers(max_idx + 1);
        }
        if let Some(max_idx) = max_transition_filter {
            context.allocate_transition_filters(max_idx + 1);
        }
        if let Some(max_idx) = max_slew_filter {
            context.allocate_slew_filters(max_idx + 1);
        }
        if let Some(max_idx) = max_cross_detector {
            context.allocate_cross_detectors(max_idx + 1);
        }
    }

    /// Attempt to compile the model to native code.
    ///
    /// Compilations are shared process-wide per model `Arc`: a thousand
    /// instances of one model compile once. The result (including a
    /// failed attempt) is cached so construction stays O(1) after the
    /// first instance.
    #[cfg(feature = "native")]
    fn try_native_compile(
        model: &std::sync::Arc<CompiledModel>,
    ) -> Result<std::sync::Arc<NativeModel>, VmError> {
        Self::try_native_compile_cached(model, NativeCompileCacheKey::Bytecode, |model| {
            crate::native::compile_native(model)
        })
    }

    #[cfg(feature = "native")]
    fn try_native_compile_with_canonical_ir(
        model: &std::sync::Arc<CompiledModel>,
        artifact: &CanonicalIrArtifact,
    ) -> Result<std::sync::Arc<NativeModel>, VmError> {
        Self::try_native_compile_cached(
            model,
            NativeCompileCacheKey::CanonicalMir(artifact.mir_digest.clone()),
            |model| crate::native::compile_native_with_canonical_ir(model, artifact),
        )
    }

    #[cfg(feature = "native")]
    fn try_native_compile_cached(
        model: &std::sync::Arc<CompiledModel>,
        cache_key: NativeCompileCacheKey,
        compile: impl FnOnce(&CompiledModel) -> crate::native::JitResult<NativeModel>,
    ) -> Result<std::sync::Arc<NativeModel>, VmError> {
        use std::sync::{Arc, Mutex, Weak};

        type CacheEntry = (
            Weak<CompiledModel>,
            NativeCompileCacheKey,
            Result<Arc<NativeModel>, String>,
        );
        static NATIVE_CACHE: Mutex<Vec<CacheEntry>> = Mutex::new(Vec::new());

        let mut cache = NATIVE_CACHE.lock().unwrap_or_else(|e| e.into_inner());
        cache.retain(|(weak, _, _)| weak.strong_count() > 0);
        if let Some((_, _, cached)) = cache
            .iter()
            .find(|(weak, key, _)| weak.as_ptr() == Arc::as_ptr(model) && *key == cache_key)
        {
            return cached.clone().map_err(VmError::NativeJit);
        }

        let compiled = match compile(model.as_ref()) {
            Ok(native) => {
                log::info!("[JIT] Model '{}' compiled to native code", model.name);
                #[cfg(debug_assertions)]
                eprintln!("[JIT] Model '{}' compiled to native code", model.name);
                Ok(std::sync::Arc::new(native))
            }
            Err(error) => {
                let msg = error.to_string();
                log::warn!(
                    "[JIT] Native compilation failed for '{}': {}",
                    model.name,
                    msg
                );
                #[cfg(debug_assertions)]
                eprintln!(
                    "[JIT] Native compilation failed for '{}': {}",
                    model.name, msg
                );
                Err(msg)
            }
        };
        cache.push((Arc::downgrade(model), cache_key, compiled.clone()));
        compiled.map_err(VmError::NativeJit)
    }

    /// Check if this device is using native compiled code.
    ///
    /// In native-feature builds, construction succeeds only with a complete
    /// native image, so every constructed device reports true.
    #[cfg(feature = "native")]
    pub fn is_using_native(&self) -> bool {
        true
    }

    /// Set the instance multiplicity (`m=` / $mfactor): the device stamps
    /// as m parallel copies. Non-positive values are rejected.
    pub fn set_multiplicity(&mut self, m: f64) {
        if m.is_finite() && m > 0.0 {
            self.context.multiplicity = m;
            // Static guards may reference $mfactor.
            self.refresh_static_conditions();
        } else {
            log::warn!(
                "Verilog-A instance '{}': ignoring non-positive multiplicity {m}",
                self.name
            );
        }
    }

    /// Instance multiplicity ($mfactor)
    pub fn multiplicity(&self) -> f64 {
        self.context.multiplicity
    }

    /// Maximum next transient step requested by `$bound_step` during the
    /// latest evaluation (None when unbounded or the model never calls it)
    pub fn transient_bound_step(&self) -> Option<f64> {
        let bound = self.variable("$bound_step")?;
        (bound.is_finite() && bound > 0.0).then_some(bound)
    }

    /// Whether `$discontinuity` fired during the latest evaluation
    pub fn discontinuity_pending(&self) -> bool {
        self.variable("$discontinuity").is_some_and(|v| v != 0.0)
    }

    /// Number of native assignment chunks the JIT produced for this model
    #[cfg(feature = "native")]
    pub fn native_chunk_count(&self) -> usize {
        self.native_model.chunk_count()
    }

    /// Native entry-point composition diagnostics.
    #[cfg(feature = "native")]
    pub fn native_plan_stats(&self) -> crate::native::PlanStats {
        self.native_model.plan_stats()
    }

    /// Check if this device is using native compiled code
    #[cfg(not(feature = "native"))]
    pub fn is_using_native(&self) -> bool {
        false
    }

    /// Get the number of terminals
    pub fn num_terminals(&self) -> usize {
        self.model.num_terminals
    }

    /// Get the number of internal nodes
    pub fn num_internal_nodes(&self) -> usize {
        self.num_internal_nodes
    }

    /// Get terminal names
    pub fn terminal_names(&self) -> &[SmolStr] {
        &self.model.terminal_names
    }

    /// Get the circuit node for a terminal
    pub fn node_for_terminal(&self, terminal: usize) -> usize {
        self.node_mapping.get(terminal).copied().unwrap_or(0)
    }

    /// Set a parameter value by name
    ///
    /// aliasparam names resolve to their target parameter, so setting an
    /// alias is identical to setting the target directly.
    pub fn set_parameter(&mut self, name: &str, value: f64) -> bool {
        // Verilog-A is case-sensitive but SPICE decks are not: prefer an
        // exact match (parameter, then alias), then accept a
        // case-insensitive one (industry netlists write PSP's TOXO as
        // toxo). Aliases cannot collide with parameter names, so the
        // ordering only arbitrates between case-insensitive candidates.
        let params = &self.model.parameters;
        let index = params
            .iter()
            .position(|p| p.name == name)
            .or_else(|| {
                params
                    .iter()
                    .position(|p| p.aliases.iter().any(|a| a.as_str() == name))
            })
            .or_else(|| {
                params
                    .iter()
                    .position(|p| p.name.eq_ignore_ascii_case(name))
            })
            .or_else(|| {
                params
                    .iter()
                    .position(|p| p.aliases.iter().any(|a| a.eq_ignore_ascii_case(name)))
            });
        let Some(i) = index else { return false };
        let param = &self.model.parameters[i];

        // Apply min/max clamping
        let clamped = match (param.min, param.max) {
            (Some(min), Some(max)) => value.clamp(min, max),
            (Some(min), None) => value.max(min),
            (None, Some(max)) => value.min(max),
            (None, None) => value,
        };
        if (clamped - value).abs() > 0.0 {
            log::warn!(
                "Parameter '{}' of '{}' clamped from {} to {} (range bound)",
                name,
                self.name,
                value,
                clamped
            );
        }
        self.context.set_param(i, clamped);
        self.context.mark_param_given(i);
        true
    }

    /// Evaluate dependent parameter defaults for parameters the instance
    /// did not set, in declaration order.
    ///
    /// Must be called after all instance parameters have been applied;
    /// calling it again is harmless (it is idempotent for a fixed set of
    /// given parameters).
    pub fn resolve_parameter_defaults(&mut self) {
        self.try_resolve_parameter_defaults().unwrap_or_else(|err| {
            panic!(
                "Verilog-A device '{}' model '{}' parameter default resolution failed: {}",
                self.name, self.model.name, err
            )
        });
    }

    /// Checked dependent-parameter default evaluation. A malformed or stale
    /// compiled default program must not become a numeric zero.
    pub fn try_resolve_parameter_defaults(&mut self) -> Result<(), VmError> {
        for i in 0..self.model.parameters.len() {
            if self.context.is_param_given(i) {
                continue;
            }
            if self.model.parameters[i].default_program.is_none() {
                continue;
            }

            #[cfg(feature = "native")]
            let value = self.run_native_parameter_default(i)?;

            #[cfg(not(feature = "native"))]
            let value = {
                let default_program = self.model.parameters[i]
                    .default_program
                    .clone()
                    .expect("default program checked above");
                let context = &mut self.context;
                let mut vm = Vm::new(context);
                vm.execute(&default_program)?
            };

            #[cfg(not(feature = "native"))]
            {
                let (min, max) = (self.model.parameters[i].min, self.model.parameters[i].max);
                let clamped = match (min, max) {
                    (Some(min), Some(max)) => value.clamp(min, max),
                    (Some(min), None) => value.max(min),
                    (None, Some(max)) => value.min(max),
                    (None, None) => value,
                };
                self.context.set_param(i, clamped);
            }

            #[cfg(feature = "native")]
            {
                let (min, max) = (self.model.parameters[i].min, self.model.parameters[i].max);
                let clamped = match (min, max) {
                    (Some(min), Some(max)) => value.clamp(min, max),
                    (Some(min), None) => value.max(min),
                    (None, Some(max)) => value.min(max),
                    (None, None) => value,
                };
                self.context.set_param(i, clamped);
            }
        }

        // Topology guards depend on final parameter values.
        #[cfg(feature = "native")]
        self.try_refresh_static_conditions()?;

        #[cfg(not(feature = "native"))]
        self.refresh_static_conditions();

        Ok(())
    }

    #[cfg(feature = "native")]
    fn run_native_parameter_default(&mut self, index: usize) -> Result<f64, VmError> {
        if self.context.variables.len() < self.model.num_variables {
            self.context.variables.resize(self.model.num_variables, 0.0);
        }

        let default_program = self.model.parameters[index]
            .default_program
            .clone()
            .expect("default program checked above");
        let native = std::sync::Arc::clone(&self.native_model);
        let context = &mut self.context;
        let mut vm = Vm::new(context);
        Self::run_value_program(
            &mut vm,
            &default_program,
            native.as_ref(),
            NativeValueEntry::ParameterDefault(index),
        )
    }

    /// Set simulation temperature in Kelvin
    pub fn set_temperature(&mut self, temp_k: f64) {
        self.context.temperature = temp_k;
        // Static guards may reference $temperature
        self.refresh_static_conditions();
    }

    /// Set simulation time
    pub fn set_time(&mut self, time: f64) {
        self.context.time = time;
    }

    /// Set the transient timestep (0 selects DC semantics for ddt/idt)
    pub fn set_timestep(&mut self, dt: f64) {
        self.context.set_timestep(dt);
    }

    /// Set the analysis type (0=dc, 1=ac, 2=tran, 3=noise, 4=ic)
    pub fn set_analysis_type(&mut self, analysis: u8) {
        self.try_set_analysis_type(analysis).unwrap_or_else(|err| {
            panic!(
                "Verilog-A device '{}' model '{}' analysis update failed: {}",
                self.name, self.model.name, err
            )
        });
    }

    /// Checked analysis update for callers that can surface native static
    /// guard refresh failures as diagnostics instead of panicking.
    pub fn try_set_analysis_type(&mut self, analysis: u8) -> Result<(), VmError> {
        if self.context.analysis_type == analysis {
            return Ok(());
        }

        self.context.analysis_type = analysis;

        #[cfg(feature = "native")]
        self.try_refresh_static_conditions()?;

        #[cfg(not(feature = "native"))]
        self.refresh_static_conditions();

        Ok(())
    }

    /// Commit integrator state after an accepted timestep
    pub fn advance_state(&mut self) {
        // Snapshot the $discontinuity level so the next step reports only
        // rising edges (a level-true region must not pin tiny steps)
        self.prev_discontinuity = self.discontinuity_pending();
        self.context.advance_state();
    }

    /// Whether `$discontinuity` newly fired since the last accepted step
    pub fn discontinuity_rising(&self) -> bool {
        self.discontinuity_pending() && !self.prev_discontinuity
    }

    /// Set the circuit node indices for internal nodes
    ///
    /// Called during circuit setup when the solver allocates nodes for internal nodes.
    pub fn set_internal_node_indices(&mut self, indices: &[usize]) {
        for (i, &idx) in indices.iter().enumerate() {
            if i < self.internal_node_indices.len() {
                self.internal_node_indices[i] = idx;
            }
        }
        self.rebuild_matrix_indices();
    }

    /// Number of branch-current unknowns required by this device's
    /// potential contributions (the engine allocates one extra system
    /// unknown per entry)
    pub fn num_branch_unknowns(&self) -> usize {
        self.model.branch_sources.len()
    }

    /// Set the circuit node indices allocated for branch-current unknowns
    pub fn set_branch_current_indices(&mut self, indices: &[usize]) {
        for (i, &idx) in indices.iter().enumerate() {
            if i < self.branch_current_indices.len() {
                self.branch_current_indices[i] = idx;
            }
        }
        self.rebuild_matrix_indices();
    }

    /// Re-evaluate instance-static activation conditions (mode guards
    /// peeled from contributions: parameter expressions or variables
    /// derived purely from parameters). A potential contribution whose
    /// guard is false leaves its branch open; a branch driven by no
    /// active potential contribution is forced to zero current.
    #[cfg(feature = "native")]
    fn refresh_static_conditions(&mut self) {
        self.try_refresh_static_conditions().unwrap_or_else(|err| {
            panic!(
                "Verilog-A device '{}' model '{}' static-condition evaluation failed: {}",
                self.name, self.model.name, err
            )
        });
    }

    #[cfg(feature = "native")]
    fn try_refresh_static_conditions(&mut self) -> Result<(), VmError> {
        let model = &self.model;
        let native = self.native_model.as_ref();
        let mut program_active = vec![true; model.stamp_programs.len()];
        let mut branch_active = vec![false; model.branch_sources.len()];
        let has_static_conditions = model
            .stamp_programs
            .iter()
            .any(|program| program.static_condition.is_some());

        if has_static_conditions {
            let context = &mut self.context;
            let mut vm = Vm::new(context);
            Self::run_assignment_pass(&mut vm, model, native)?;

            for (idx, program) in model.stamp_programs.iter().enumerate() {
                let active = if program.static_condition.is_some() {
                    Self::run_value_program(
                        &mut vm,
                        program
                            .static_condition
                            .as_ref()
                            .expect("static condition checked above"),
                        native,
                        NativeValueEntry::StaticCondition(idx),
                    )? != 0.0
                } else {
                    true
                };
                program_active[idx] = active;
                if active
                    && let Some(ordinal) = program.branch_ordinal
                    && ordinal < branch_active.len()
                {
                    branch_active[ordinal] = true;
                }
            }
        } else {
            for program in &model.stamp_programs {
                if let Some(ordinal) = program.branch_ordinal
                    && ordinal < branch_active.len()
                {
                    branch_active[ordinal] = true;
                }
            }
        }

        self.program_active = program_active;
        self.branch_active = branch_active;
        Ok(())
    }

    #[cfg(feature = "native")]
    fn missing_native_static_condition_entry(index: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT missing static-condition entry for stamp {index}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_parameter_default_entry(index: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT missing parameter-default entry for parameter {index}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_noise_exponent_entry(index: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT missing noise exponent entry for source {index}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_terminal_pair_current_slot(pair_index: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT missing terminal-pair current slot {pair_index}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_parameter_storage(required: usize, available: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT parameter storage has {available} slot(s), but compiled image requires {required}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_param_given_storage(required: usize, available: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT parameter-given storage has {available} slot(s), but compiled image requires {required}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_variable_storage(required: usize, available: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT variable storage has {available} slot(s), but compiled image requires {required}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_voltage_storage(required: usize, available: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT voltage storage has {available} terminal slot(s), but compiled image requires {required}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_internal_voltage_storage(required: usize, available: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT internal-voltage storage has {available} slot(s), but compiled image requires {required}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_port_connected_storage(required: usize, available: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT port-connected storage has {available} slot(s), but compiled image requires {required}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_runtime_storage(label: &str, required: usize, available: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT {label} has {available} slot(s), but compiled image requires {required}; no interpreter fallback"
        ))
    }

    #[cfg(not(feature = "native"))]
    fn refresh_static_conditions(&mut self) {
        let model = &self.model;
        let mut program_active = vec![true; model.stamp_programs.len()];
        let mut branch_active = vec![false; model.branch_sources.len()];

        {
            let context = &mut self.context;
            let mut bytecode_vm = Vm::new(context);
            // Static guards may reference instance-static variables (e.g.
            // BSIM4rdsMod derived from the rdsmod parameter); run the
            // evaluation stream once so those variables hold their values.
            // Node voltages are irrelevant to instance-static expressions.
            Self::execute_assignment_programs(&mut bytecode_vm, model).unwrap_or_else(|err| {
                panic!(
                    "Verilog-A device '{}' model '{}' static-condition evaluation failed: {}",
                    self.name, model.name, err
                )
            });
            for (idx, program) in model.stamp_programs.iter().enumerate() {
                let active = match &program.static_condition {
                    Some(condition) => bytecode_vm
                        .execute(condition)
                        .map(|v| v != 0.0)
                        .unwrap_or_else(|err| {
                            panic!(
                                "Verilog-A device '{}' model '{}' static condition failed: {}",
                                self.name, model.name, err
                            )
                        }),
                    None => true,
                };
                program_active[idx] = active;
                if active
                    && let Some(ordinal) = program.branch_ordinal
                    && ordinal < branch_active.len()
                {
                    branch_active[ordinal] = true;
                }
            }
        }

        self.program_active = program_active;
        self.branch_active = branch_active;
    }

    /// Get the circuit node index for an internal node
    pub fn internal_node_index(&self, internal_idx: usize) -> Option<usize> {
        self.internal_node_indices.get(internal_idx).copied()
    }

    /// Get the circuit node index allocated for a branch-current unknown
    pub fn branch_current_index(&self, ordinal: usize) -> Option<usize> {
        self.branch_current_indices.get(ordinal).copied()
    }

    /// Read an internal model variable by name (operating-point
    /// inspection). Returns the value from the most recent evaluation.
    pub fn variable(&self, name: &str) -> Option<f64> {
        let idx = self
            .model
            .variable_names
            .iter()
            .position(|n| n.as_str() == name)?;
        self.context.variables.get(idx).copied()
    }

    /// Iterate (name, value) over all internal model variables
    pub fn variables(&self) -> impl Iterator<Item = (&str, f64)> {
        self.model
            .variable_names
            .iter()
            .map(|n| n.as_str())
            .zip(self.context.variables.iter().copied())
    }

    /// Remap circuit node IDs after an external topology rewrite.
    pub fn remap_circuit_nodes(&mut self, mut remap: impl FnMut(usize) -> usize) {
        for node in &mut self.node_mapping {
            *node = remap(*node);
        }
        for node in &mut self.internal_node_indices {
            *node = remap(*node);
        }
        self.rebuild_matrix_indices();
    }

    /// Build mapped RHS stamp rows for each stamp program.
    ///
    /// Returns one entry per stamp program; each program entry contains
    /// `(node_index, sign)` pairs for non-ground RHS rows.
    pub fn mapped_rhs_rows(&self) -> Vec<Vec<(usize, f64)>> {
        self.matrix_indices
            .rhs
            .iter()
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|entry| entry.node.map(|node| (node, entry.sign)))
                    .collect()
            })
            .collect()
    }

    /// Build mapped Jacobian matrix locations for each stamp program.
    ///
    /// Returns one entry per stamp program; each program entry contains
    /// `(row, col)` locations for each Jacobian program where `None` means ground.
    pub fn mapped_jacobian_locations(&self) -> Vec<Vec<(Option<usize>, Option<usize>)>> {
        self.matrix_indices
            .jacobian
            .iter()
            .map(|entries| entries.iter().map(|entry| (entry.row, entry.col)).collect())
            .collect()
    }

    /// Recompute cached matrix/RHS node mappings after topology changes.
    fn rebuild_matrix_indices(&mut self) {
        let mut rhs = vec![Vec::new(); self.model.stamp_programs.len()];
        let mut jacobian = vec![Vec::new(); self.model.stamp_programs.len()];
        let mut reactive = vec![Vec::new(); self.model.stamp_programs.len()];

        let map_entries = |entries: &[crate::codegen::JacobianEntry], program_idx: usize| {
            entries
                .iter()
                .enumerate()
                .map(|(jacobian_idx, jac_entry)| JacobianIndex {
                    row: Self::index_to_node(
                        &jac_entry.row,
                        &self.node_mapping,
                        &self.internal_node_indices,
                        &self.branch_current_indices,
                    ),
                    col: Self::index_to_node(
                        &jac_entry.col,
                        &self.node_mapping,
                        &self.internal_node_indices,
                        &self.branch_current_indices,
                    ),
                    program_idx,
                    jacobian_idx,
                    sign: jac_entry.sign,
                })
                .collect::<Vec<_>>()
        };

        for (program_idx, program) in self.model.stamp_programs.iter().enumerate() {
            rhs[program_idx] = program
                .stamp_locations
                .iter()
                .map(|loc| RhsIndex {
                    node: Self::index_to_node(
                        &loc.row,
                        &self.node_mapping,
                        &self.internal_node_indices,
                        &self.branch_current_indices,
                    ),
                    sign: loc.sign,
                    program_idx,
                })
                .collect();

            jacobian[program_idx] = map_entries(&program.jacobian_programs, program_idx);
            reactive[program_idx] = map_entries(&program.reactive_jacobians, program_idx);
        }

        self.matrix_indices = MatrixIndices {
            jacobian,
            reactive,
            rhs,
        };
    }

    /// Stamp the reactive (charge/flux) Jacobian dQ/dx.
    ///
    /// AC analysis multiplies these entries by the angular frequency and
    /// adds them to the imaginary part of the system matrix: capacitances
    /// for current contributions, inductive terms on the branch rows of
    /// potential contributions.
    pub fn stamp_reactive<M>(&mut self, circuit_voltages: &[f64], mut matrix_add: M)
    where
        M: FnMut(usize, usize, f64),
    {
        if let Err(err) = self.try_stamp_reactive(circuit_voltages, &mut matrix_add) {
            panic!(
                "Verilog-A device '{}' model '{}' reactive stamping failed: {}",
                self.name, self.model.name, err
            );
        }
    }

    /// Checked reactive stamping path for callers that can report Verilog-A
    /// runtime diagnostics instead of unwinding.
    pub fn try_stamp_reactive<M>(
        &mut self,
        circuit_voltages: &[f64],
        mut matrix_add: M,
    ) -> Result<(), VmError>
    where
        M: FnMut(usize, usize, f64),
    {
        self.try_update_all_voltages(circuit_voltages)?;

        let context = &mut self.context;
        let model = &self.model;
        let matrix_indices = &self.matrix_indices;
        let program_active = &self.program_active;
        #[cfg(feature = "native")]
        let native = self.native_model.as_ref();

        context.clear_currents();

        let mut vm = Vm::new(context);
        Self::run_assignment_pass(
            &mut vm,
            model,
            #[cfg(feature = "native")]
            native,
        )?;

        let m = vm.context.multiplicity;
        for (program_idx, program) in model.stamp_programs.iter().enumerate() {
            if !program_active.get(program_idx).copied().unwrap_or(true) {
                continue;
            }
            // Charge of m parallel copies scales the same way as current
            let scale = if program.branch_ordinal.is_none() {
                m
            } else {
                1.0
            };

            for entry in &matrix_indices.reactive[program_idx] {
                let model_entry = &program.reactive_jacobians[entry.jacobian_idx];
                let deriv = Self::run_value_program(
                    &mut vm,
                    &model_entry.program,
                    #[cfg(feature = "native")]
                    native,
                    #[cfg(feature = "native")]
                    NativeValueEntry::ReactiveJacobian {
                        stamp: program_idx,
                        entry: entry.jacobian_idx,
                    },
                )?;
                let deriv = match deriv {
                    v if v.is_finite() => v * scale,
                    _ => continue,
                };
                if let (Some(row), Some(col)) = (entry.row, entry.col) {
                    matrix_add(row, col, entry.sign * deriv);
                }
            }
        }
        Ok(())
    }

    /// Update terminal voltages from circuit solution
    ///
    /// Called before evaluating device equations.
    pub fn update_voltages(&mut self, circuit_voltages: &[f64]) {
        self.try_update_voltages(circuit_voltages)
            .unwrap_or_else(|err| {
                panic!(
                    "Verilog-A device '{}' model '{}' terminal voltage update failed: {}",
                    self.name, self.model.name, err
                )
            });
    }

    /// Checked terminal voltage update from circuit solution.
    pub fn try_update_voltages(&mut self, circuit_voltages: &[f64]) -> Result<(), VmError> {
        for (terminal, &node) in self.node_mapping.iter().enumerate() {
            if terminal < self.context.voltages.len() {
                let v =
                    Self::solution_value(circuit_voltages, node, "missing terminal solution slot")?;
                self.context.voltages[terminal] = v;
            }
        }
        Ok(())
    }

    /// Update both terminal and internal node voltages from circuit solution
    ///
    /// This is the full-featured method for solver integration.
    pub fn update_all_voltages(&mut self, circuit_voltages: &[f64]) {
        self.try_update_all_voltages(circuit_voltages)
            .unwrap_or_else(|err| {
                panic!(
                    "Verilog-A device '{}' model '{}' solution update failed: {}",
                    self.name, self.model.name, err
                )
            });
    }

    /// Checked update of terminals, internal nodes, and branch-current
    /// unknowns from a circuit solution.
    pub fn try_update_all_voltages(&mut self, circuit_voltages: &[f64]) -> Result<(), VmError> {
        // Update terminal voltages
        self.try_update_voltages(circuit_voltages)?;

        // Update internal node voltages
        for (internal_idx, &circuit_node) in self.internal_node_indices.iter().enumerate() {
            if internal_idx < self.context.internal_voltages.len() {
                let v = Self::solution_value(
                    circuit_voltages,
                    circuit_node,
                    "missing internal-node solution slot",
                )?;
                self.context.internal_voltages[internal_idx] = v;
            }
        }

        // Update branch-current unknown values
        for (ordinal, &circuit_node) in self.branch_current_indices.iter().enumerate() {
            if ordinal < self.context.branch_current_values.len() {
                let v = Self::solution_value(
                    circuit_voltages,
                    circuit_node,
                    "missing branch-current solution slot",
                )?;
                self.context.branch_current_values[ordinal] = v;
            }
        }
        Ok(())
    }

    fn solution_value(
        circuit_voltages: &[f64],
        circuit_node: usize,
        missing_message: &'static str,
    ) -> Result<f64, VmError> {
        if circuit_node == 0 {
            Ok(0.0)
        } else {
            circuit_voltages
                .get(circuit_node - 1)
                .copied()
                .ok_or(VmError::InvalidInstruction(missing_message))
        }
    }

    /// Evaluate the device: compute branch current
    ///
    /// Returns the current for each branch equation. Native builds require
    /// complete assignment and stamp-value entry points; non-native builds run
    /// the bytecode programs.
    pub fn evaluate(&mut self) -> Vec<f64> {
        self.try_evaluate().unwrap_or_else(|err| {
            panic!(
                "Verilog-A device '{}' model '{}' evaluation failed: {}",
                self.name, self.model.name, err
            )
        })
    }

    /// Checked evaluation path for callers that can surface runtime model
    /// errors as diagnostics instead of panicking.
    pub fn try_evaluate(&mut self) -> Result<Vec<f64>, VmError> {
        self.context.clear_currents();
        // Pre-reserve so the currents pointer stays stable while native
        // snapshots reference it across pushes
        self.context
            .currents
            .reserve(self.model.stamp_programs.len());

        let program_active = &self.program_active;
        #[cfg(feature = "native")]
        let native = self.native_model.as_ref();
        let context = &mut self.context;
        let mut vm = Vm::new(context);
        Self::run_assignment_pass(
            &mut vm,
            &self.model,
            #[cfg(feature = "native")]
            native,
        )?;
        let mut currents = Vec::with_capacity(self.model.stamp_programs.len());

        for (program_idx, program) in self.model.stamp_programs.iter().enumerate() {
            if !program_active.get(program_idx).copied().unwrap_or(true) {
                currents.push(0.0);
                vm.context.currents.push(0.0);
                continue;
            }
            let value = Self::run_value_program(
                &mut vm,
                &program.value_program,
                #[cfg(feature = "native")]
                native,
                #[cfg(feature = "native")]
                NativeValueEntry::StampValue(program_idx),
            )?;
            currents.push(value);
            vm.context.currents.push(value);
            if program.branch_ordinal.is_none()
                && let Some((pos, neg)) = Self::infer_current_terminal_pair(program)
            {
                vm.context.set_branch_current(pos, neg, value);
            }
        }

        Ok(currents)
    }

    /// Build a native evaluation-context snapshot over the VM context.
    /// Raw pointers only — rebuild it after anything that may reallocate
    /// the underlying vectors.
    #[cfg(feature = "native")]
    fn eval_context_from(context: &mut VmContext) -> crate::native::EvalContext {
        crate::native::EvalContext {
            voltages: context.voltages.as_ptr(),
            internal_voltages: context.internal_voltages.as_ptr(),
            params: context.parameters.as_ptr(),
            branch_currents: context.terminal_pair_currents_ptr(),
            branch_currents_len: context.terminal_pair_currents_len(),
            currents: context.currents.as_ptr(),
            currents_len: context.currents.len(),
            num_terminals: context.terminal_count(),
            port_connected: context.port_connected.as_ptr(),
            port_connected_len: context.port_connected.len(),
            temperature: context.temperature,
            time: context.time,
            timestep: context.timestep,
            // Pass null for empty vecs - as_ptr() on empty vec gives dangling non-null pointer
            state_prev: if context.state_values_prev.is_empty() {
                std::ptr::null()
            } else {
                context.state_values_prev.as_ptr()
            },
            state_values: if context.state_values.is_empty() {
                std::ptr::null_mut()
            } else {
                context.state_values.as_mut_ptr()
            },
            state_initialized: if context.state_initialized.is_empty() {
                std::ptr::null_mut()
            } else {
                context.state_initialized.as_mut_ptr() as *mut u8
            },
            state_initialized_len: context.state_initialized.len(),
            lookup_tables: if context.lookup_tables.is_empty() {
                std::ptr::null()
            } else {
                context.lookup_tables.as_ptr()
            },
            lookup_tables_len: context.lookup_tables.len(),
            laplace_filters: if context.laplace_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                context.laplace_filters.as_mut_ptr()
            },
            laplace_filters_len: context.laplace_filters.len(),
            param_given: context.param_given.as_ptr() as *const u8,
            param_given_len: context.param_given.len(),
            branch_unknowns: if context.branch_current_values.is_empty() {
                std::ptr::null()
            } else {
                context.branch_current_values.as_ptr()
            },
            analysis_type: context.analysis_type,
            multiplicity: context.multiplicity,
            zi_filters: if context.zi_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                context.zi_filters.as_mut_ptr()
            },
            zi_filters_len: context.zi_filters.len(),
            transition_filters: if context.transition_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                context.transition_filters.as_mut_ptr()
            },
            transition_filters_len: context.transition_filters.len(),
            slew_filters: if context.slew_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                context.slew_filters.as_mut_ptr()
            },
            slew_filters_len: context.slew_filters.len(),
            delay_buffers: if context.delay_buffers.is_empty() {
                std::ptr::null_mut()
            } else {
                context.delay_buffers.as_mut_ptr()
            },
            delay_buffers_len: context.delay_buffers.len(),
            cross_detectors: if context.cross_detectors.is_empty() {
                std::ptr::null_mut()
            } else {
                context.cross_detectors.as_mut_ptr()
            },
            cross_detectors_len: context.cross_detectors.len(),
            state_prev_len: context.state_values_prev.len(),
            state_values_len: context.state_values.len(),
        }
    }

    /// Run one value-returning native entry point.
    #[cfg(feature = "native")]
    fn run_value_program(
        vm: &mut Vm<'_>,
        _program: &crate::codegen::BytecodeProgram,
        native: &NativeModel,
        entry: NativeValueEntry,
    ) -> Result<f64, VmError> {
        if let NativeValueEntry::NoiseExponent(index) = entry
            && !native.has_noise_exponent_entry(index)
        {
            return Err(Self::missing_native_noise_exponent_entry(index));
        }

        let current_pairs = match entry {
            NativeValueEntry::ParameterDefault(_) => &[],
            NativeValueEntry::StaticCondition(_) => &[],
            NativeValueEntry::StampValue(index) => native.stamp_value_current_pairs(index),
            NativeValueEntry::Jacobian { stamp, entry } => {
                native.jacobian_current_pairs(stamp, entry)
            }
            NativeValueEntry::ReactiveJacobian { stamp, entry } => {
                native.reactive_jacobian_current_pairs(stamp, entry)
            }
            NativeValueEntry::NoisePsd(index) => native.noise_psd_current_pairs(index),
            NativeValueEntry::NoiseExponent(index) => native.noise_exponent_current_pairs(index),
        };
        let prior_currents = match entry {
            NativeValueEntry::StampValue(index) => native.stamp_value_prior_currents(index),
            _ => &[],
        };
        Self::validate_native_storage(vm.context, native)?;
        Self::validate_native_current_pairs(vm.context, current_pairs)?;
        Self::validate_native_prior_currents(vm.context, prior_currents)?;

        let ctx = Self::eval_context_from(vm.context);
        let vars_ptr = vm.context.variables.as_ptr();
        clear_native_runtime_error();
        let value = match entry {
            NativeValueEntry::ParameterDefault(index) => native
                .run_parameter_default(index, &ctx, vars_ptr)
                .ok_or_else(|| Self::missing_native_parameter_default_entry(index))?,
            NativeValueEntry::StaticCondition(index) => native
                .run_static_condition(index, &ctx, vars_ptr)
                .ok_or_else(|| Self::missing_native_static_condition_entry(index))?,
            NativeValueEntry::StampValue(index) => native.run_stamp_value(index, &ctx, vars_ptr),
            NativeValueEntry::Jacobian { stamp, entry } => {
                native.run_jacobian(stamp, entry, &ctx, vars_ptr)
            }
            NativeValueEntry::ReactiveJacobian { stamp, entry } => {
                native.run_reactive_jacobian(stamp, entry, &ctx, vars_ptr)
            }
            NativeValueEntry::NoisePsd(index) => native.run_noise_psd(index, &ctx, vars_ptr),
            NativeValueEntry::NoiseExponent(index) => native
                .run_noise_exponent(index, &ctx, vars_ptr)
                .ok_or_else(|| Self::missing_native_noise_exponent_entry(index))?,
        };
        if let Some(error) = take_native_runtime_error() {
            return Err(VmError::NativeJit(error));
        }
        Ok(value)
    }

    #[cfg(feature = "native")]
    fn validate_native_current_pairs(
        context: &VmContext,
        current_pairs: &[usize],
    ) -> Result<(), VmError> {
        if current_pairs.is_empty() {
            return Ok(());
        }

        let terminal_count = context.terminal_count();
        if terminal_count == 0 {
            return Err(Self::missing_native_terminal_pair_current_slot(0));
        }
        for pair_index in current_pairs {
            let Some((pos, neg)) = terminal_pair_current_endpoints(*pair_index, terminal_count)
            else {
                return Err(Self::missing_native_terminal_pair_current_slot(*pair_index));
            };
            context
                .try_current(pos, neg)
                .map_err(|_| Self::missing_native_terminal_pair_current_slot(*pair_index))?;
        }

        Ok(())
    }

    #[cfg(feature = "native")]
    fn validate_native_storage(context: &VmContext, native: &NativeModel) -> Result<(), VmError> {
        Self::validate_native_voltage_storage(
            context,
            native.num_terminals,
            native.num_internal_nodes,
        )?;
        Self::validate_native_parameter_storage(context, native.num_parameters)?;
        Self::validate_native_variable_storage(context, native.num_variables)?;
        Self::validate_native_runtime_storage(context, native.required_storage())
    }

    #[cfg(feature = "native")]
    fn validate_native_voltage_storage(
        context: &VmContext,
        required_terminals: usize,
        required_internal_nodes: usize,
    ) -> Result<(), VmError> {
        if context.voltages.len() < required_terminals {
            return Err(Self::missing_native_voltage_storage(
                required_terminals,
                context.voltages.len(),
            ));
        }
        if context.internal_voltages.len() < required_internal_nodes {
            return Err(Self::missing_native_internal_voltage_storage(
                required_internal_nodes,
                context.internal_voltages.len(),
            ));
        }
        if context.port_connected.len() < required_terminals {
            return Err(Self::missing_native_port_connected_storage(
                required_terminals,
                context.port_connected.len(),
            ));
        }

        Ok(())
    }

    #[cfg(feature = "native")]
    fn validate_native_parameter_storage(
        context: &VmContext,
        required: usize,
    ) -> Result<(), VmError> {
        if context.parameters.len() < required {
            return Err(Self::missing_native_parameter_storage(
                required,
                context.parameters.len(),
            ));
        }
        if context.param_given.len() < required {
            return Err(Self::missing_native_param_given_storage(
                required,
                context.param_given.len(),
            ));
        }

        Ok(())
    }

    #[cfg(feature = "native")]
    fn validate_native_variable_storage(
        context: &VmContext,
        required: usize,
    ) -> Result<(), VmError> {
        if context.variables.len() < required {
            return Err(Self::missing_native_variable_storage(
                required,
                context.variables.len(),
            ));
        }

        Ok(())
    }

    #[cfg(feature = "native")]
    fn validate_native_runtime_storage(
        context: &VmContext,
        required: NativeRequiredStorage,
    ) -> Result<(), VmError> {
        Self::validate_native_runtime_storage_len(
            "state-value storage",
            required.state_values,
            context.state_values.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "prior state-value storage",
            required.state_values_prev,
            context.state_values_prev.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "state-initialization flag storage",
            required.state_initialized,
            context.state_initialized.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "lookup-table storage",
            required.lookup_tables,
            context.lookup_tables.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "Laplace filter storage",
            required.laplace_filters,
            context.laplace_filters.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "zi filter storage",
            required.zi_filters,
            context.zi_filters.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "transition filter storage",
            required.transition_filters,
            context.transition_filters.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "slew filter storage",
            required.slew_filters,
            context.slew_filters.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "delay-buffer storage",
            required.delay_buffers,
            context.delay_buffers.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "cross-detector storage",
            required.cross_detectors,
            context.cross_detectors.len(),
        )
    }

    #[cfg(feature = "native")]
    fn validate_native_runtime_storage_len(
        label: &str,
        required: usize,
        available: usize,
    ) -> Result<(), VmError> {
        if available < required {
            return Err(Self::missing_native_runtime_storage(
                label, required, available,
            ));
        }

        Ok(())
    }

    #[cfg(feature = "native")]
    fn validate_native_prior_currents(
        context: &VmContext,
        prior_currents: &[usize],
    ) -> Result<(), VmError> {
        for current_index in prior_currents {
            if *current_index >= context.currents.len() {
                return Err(VmError::NativeJit(format!(
                    "native stamp requires prior contribution current {current_index}, but only {} current(s) have been evaluated; no interpreter fallback",
                    context.currents.len()
                )));
            }
        }

        Ok(())
    }

    /// Run one value-returning bytecode program.
    #[cfg(not(feature = "native"))]
    fn run_value_program(
        vm: &mut Vm<'_>,
        program: &crate::codegen::BytecodeProgram,
    ) -> Result<f64, VmError> {
        vm.execute(program)
    }

    /// Execute the assignment pass through the required native entry point.
    #[cfg(feature = "native")]
    fn run_assignment_pass(
        vm: &mut Vm<'_>,
        model: &CompiledModel,
        native: &NativeModel,
    ) -> Result<(), VmError> {
        if vm.context.variables.len() < model.num_variables {
            vm.context.variables.resize(model.num_variables, 0.0);
        }
        Self::validate_native_storage(vm.context, native)?;
        let ctx = Self::eval_context_from(vm.context);
        let vars_ptr = vm.context.variables.as_mut_ptr();
        clear_native_runtime_error();
        native.run_assignments(&ctx, vars_ptr);
        if let Some(error) = take_native_runtime_error() {
            return Err(VmError::NativeJit(error));
        }
        Ok(())
    }

    /// Execute the assignment pass through the bytecode interpreter.
    #[cfg(not(feature = "native"))]
    fn run_assignment_pass(vm: &mut Vm<'_>, model: &CompiledModel) -> Result<(), VmError> {
        if vm.context.variables.len() < model.num_variables {
            vm.context.variables.resize(model.num_variables, 0.0);
        }
        Self::execute_assignment_steps(vm, &model.assignment_steps)
    }

    #[cfg(feature = "native")]
    fn populate_noise_current_probe_cache(
        vm: &mut Vm<'_>,
        model: &CompiledModel,
        program_active: &[bool],
        native: &NativeModel,
    ) -> Result<(), VmError> {
        for (program_idx, program) in model.stamp_programs.iter().enumerate() {
            let active = program_active.get(program_idx).copied().unwrap_or(true);
            let value = if active {
                Self::run_value_program(
                    vm,
                    &program.value_program,
                    native,
                    NativeValueEntry::StampValue(program_idx),
                )?
            } else {
                0.0
            };
            Self::cache_current_probe_value(vm.context, program, value, active);
        }
        Ok(())
    }

    #[cfg(not(feature = "native"))]
    fn populate_noise_current_probe_cache(
        vm: &mut Vm<'_>,
        model: &CompiledModel,
        program_active: &[bool],
    ) -> Result<(), VmError> {
        for (program_idx, program) in model.stamp_programs.iter().enumerate() {
            let active = program_active.get(program_idx).copied().unwrap_or(true);
            let value = if active {
                vm.execute(&program.value_program)?
            } else {
                0.0
            };
            Self::cache_current_probe_value(vm.context, program, value, active);
        }
        Ok(())
    }

    fn cache_current_probe_value(
        context: &mut VmContext,
        program: &crate::codegen::StampProgram,
        value: f64,
        active: bool,
    ) {
        context.currents.push(value);
        if active
            && program.branch_ordinal.is_none()
            && let Some((pos, neg)) = Self::infer_current_terminal_pair(program)
        {
            context.set_branch_current(pos, neg, value);
        }
    }

    /// Safety cap on runtime-loop iterations per evaluation (a model bug
    /// must not hang the Newton loop)
    #[cfg(not(feature = "native"))]
    const MAX_RUNTIME_LOOP_ITERATIONS: usize = 100_000;

    /// Execute assignment programs and update VM variable storage.
    #[cfg(not(feature = "native"))]
    fn execute_assignment_programs(vm: &mut Vm<'_>, model: &CompiledModel) -> Result<(), VmError> {
        if vm.context.variables.len() < model.num_variables {
            vm.context.variables.resize(model.num_variables, 0.0);
        }

        Self::execute_assignment_steps(vm, &model.assignment_steps)
    }

    /// Execute a sequence of evaluation steps (assignments and runtime
    /// loops), recursively
    #[cfg(not(feature = "native"))]
    fn execute_assignment_steps(
        vm: &mut Vm<'_>,
        steps: &[crate::codegen::AssignmentStep],
    ) -> Result<(), VmError> {
        for step in steps {
            match step {
                crate::codegen::AssignmentStep::Assign(assignment) => {
                    let value = vm.execute(&assignment.program)?;
                    if assignment.var_index < vm.context.variables.len() {
                        vm.context.variables[assignment.var_index] = value;
                    }
                }
                crate::codegen::AssignmentStep::AssignIndexed {
                    base,
                    len,
                    lower,
                    index,
                    value,
                } => {
                    let slot = vm
                        .execute(index)
                        .and_then(|raw| Vm::array_slot(raw, *base, *len, *lower));
                    let slot = slot?;
                    let value = vm.execute(value)?;
                    if slot < vm.context.variables.len() {
                        vm.context.variables[slot] = value;
                    }
                }
                crate::codegen::AssignmentStep::Loop { condition, body } => {
                    let mut iterations = 0usize;
                    loop {
                        let active = vm.execute(condition)?;
                        if active == 0.0 {
                            break;
                        }
                        Self::execute_assignment_steps(vm, body)?;
                        iterations += 1;
                        if iterations >= Self::MAX_RUNTIME_LOOP_ITERATIONS {
                            return Err(VmError::InvalidInstruction(
                                "runtime loop iteration limit exceeded",
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Compute Jacobian entries
    ///
    /// Returns (value, row_terminal, col_terminal, is_current) for each derivative.
    pub fn compute_jacobian(&mut self) -> Vec<JacobianEntry> {
        self.try_compute_jacobian().unwrap_or_else(|err| {
            panic!(
                "Verilog-A device '{}' model '{}' Jacobian evaluation failed: {}",
                self.name, self.model.name, err
            )
        })
    }

    /// Checked Jacobian evaluation path for callers that can surface
    /// runtime model errors as diagnostics instead of panicking.
    pub fn try_compute_jacobian(&mut self) -> Result<Vec<JacobianEntry>, VmError> {
        let context = &mut self.context;
        let model = &self.model;
        #[cfg(feature = "native")]
        let native = self.native_model.as_ref();

        context.clear_currents();
        context.currents.reserve(model.stamp_programs.len());

        let program_active = &self.program_active;
        let mut vm = Vm::new(context);
        Self::run_assignment_pass(
            &mut vm,
            model,
            #[cfg(feature = "native")]
            native,
        )?;
        let mut entries = Vec::new();

        for (prog_idx, program) in model.stamp_programs.iter().enumerate() {
            if !program_active.get(prog_idx).copied().unwrap_or(true) {
                vm.context.currents.push(0.0);
                continue;
            }
            let value = Self::run_value_program(
                &mut vm,
                &program.value_program,
                #[cfg(feature = "native")]
                native,
                #[cfg(feature = "native")]
                NativeValueEntry::StampValue(prog_idx),
            )?;
            let value = match value {
                v if v.is_finite() => v,
                _ => {
                    vm.context.currents.push(0.0);
                    continue;
                }
            };
            vm.context.currents.push(value);
            if program.branch_ordinal.is_none()
                && let Some((pos, neg)) = Self::infer_current_terminal_pair(program)
            {
                vm.context.set_branch_current(pos, neg, value);
            }

            for (jac_idx, jac_entry) in program.jacobian_programs.iter().enumerate() {
                let value = Self::run_value_program(
                    &mut vm,
                    &jac_entry.program,
                    #[cfg(feature = "native")]
                    native,
                    #[cfg(feature = "native")]
                    NativeValueEntry::Jacobian {
                        stamp: prog_idx,
                        entry: jac_idx,
                    },
                )?;
                let value = match value {
                    v if v.is_finite() => v,
                    _ => continue,
                };
                entries.push(JacobianEntry {
                    value: jac_entry.sign * value,
                    row: jac_entry.row.clone(),
                    col: jac_entry.col.clone(),
                    program_idx: prog_idx,
                    jacobian_idx: jac_idx,
                });
            }
        }

        Ok(entries)
    }

    /// Stamp device into matrix and RHS
    ///
    /// This is the main interface for circuit simulation.
    ///
    /// # Arguments
    /// * `matrix_add` - Callback to add value at (row, col) in circuit matrix
    /// * `rhs_add` - Callback to add value at (node) in RHS vector
    /// * `circuit_voltages` - Current voltage solution
    pub fn stamp<M, R>(&mut self, circuit_voltages: &[f64], mut matrix_add: M, mut rhs_add: R)
    where
        M: FnMut(usize, usize, f64),
        R: FnMut(usize, f64),
    {
        if let Err(err) = self.try_stamp(circuit_voltages, &mut matrix_add, &mut rhs_add) {
            panic!(
                "Verilog-A device '{}' model '{}' stamping failed: {}",
                self.name, self.model.name, err
            );
        }
    }

    /// Checked stamping path for callers that can turn Verilog-A runtime
    /// faults into simulator diagnostics.
    pub fn try_stamp<M, R>(
        &mut self,
        circuit_voltages: &[f64],
        mut matrix_add: M,
        mut rhs_add: R,
    ) -> Result<(), VmError>
    where
        M: FnMut(usize, usize, f64),
        R: FnMut(usize, f64),
    {
        // Update context with the full solution (terminals, internal
        // nodes, and branch-current unknowns)
        self.try_update_all_voltages(circuit_voltages)?;

        // Extract disjoint fields to satisfy borrow checker
        let context = &mut self.context;
        let model = &self.model;
        let matrix_indices = &self.matrix_indices;
        #[cfg(feature = "native")]
        let native = self.native_model.as_ref();
        // Instance multiplicity: m parallel copies scale every flow
        // (current) stamp by m; potential and constraint rows stay
        // per-copy, as do probed currents and internal node voltages
        let m = context.multiplicity;

        context.clear_currents();
        // Native snapshots hold a raw pointer into `currents` while values
        // push; pre-reserve so it never reallocates mid-pass
        context.currents.reserve(model.stamp_programs.len());

        let mut vm = Vm::new(context);
        Self::run_assignment_pass(
            &mut vm,
            model,
            #[cfg(feature = "native")]
            native,
        )?;

        // Structural stamps of the branch-current unknowns: the KCL rows of
        // the source nodes couple to the branch column, and the branch row
        // reads the node potentials. An undriven branch (all its potential
        // contributions mode-disabled) is pinned to zero current so its row
        // stays non-singular while the branch itself is open.
        for (ordinal, source) in model.branch_sources.iter().enumerate() {
            let br = Self::index_to_node(
                &StampIndex::Branch(ordinal),
                &self.node_mapping,
                &self.internal_node_indices,
                &self.branch_current_indices,
            );
            let Some(br) = br else { continue };

            if !self.branch_active.get(ordinal).copied().unwrap_or(false) {
                matrix_add(br, br, 1.0);
                continue;
            }

            let pos = Self::index_to_node(
                &source.pos,
                &self.node_mapping,
                &self.internal_node_indices,
                &self.branch_current_indices,
            );
            let neg = Self::index_to_node(
                &source.neg,
                &self.node_mapping,
                &self.internal_node_indices,
                &self.branch_current_indices,
            );

            // The unknown is the per-copy branch current; m copies inject
            // m times that current into the external KCL rows
            if let Some(p) = pos {
                matrix_add(p, br, m);
                // Indirect branches replace the V(p)-V(n)-E row with the
                // constraint equation; only the KCL couplings are
                // structural
                if !source.indirect {
                    matrix_add(br, p, 1.0);
                }
            }
            if let Some(n) = neg {
                matrix_add(n, br, -m);
                if !source.indirect {
                    matrix_add(br, n, -1.0);
                }
            }
        }

        for (program_idx, program) in model.stamp_programs.iter().enumerate() {
            if !self
                .program_active
                .get(program_idx)
                .copied()
                .unwrap_or(true)
            {
                continue;
            }

            // Compute the contribution value (branch current for current
            // contributions, source voltage for potential contributions).
            // Non-finite values would poison the whole system; skip the
            // program and let Newton damping recover.
            let value = Self::run_value_program(
                &mut vm,
                &program.value_program,
                #[cfg(feature = "native")]
                native,
                #[cfg(feature = "native")]
                NativeValueEntry::StampValue(program_idx),
            )?;
            let value = match value {
                v if v.is_finite() => v,
                _ => continue,
            };

            // Probed currents stay per-copy; only the stamps scale
            vm.context.currents.push(value);
            if program.branch_ordinal.is_none()
                && let Some((pos, neg)) = Self::infer_current_terminal_pair(program)
            {
                vm.context.set_branch_current(pos, neg, value);
            }

            // Flow contributions of m parallel copies inject m times the
            // per-copy current; potential and constraint rows are per-copy
            let scale = if program.branch_ordinal.is_none() {
                m
            } else {
                1.0
            };

            // Companion model: solve A*x_new = z with the device linearized
            // at x_old.
            //
            // Current contributions: each KCL row receives the equivalent
            // current Ieq = I(x_old) - sum_col dI/dx_col * x_col_old and
            // the Jacobian stamps both KCL rows (entry sign tracks the row).
            //
            // Potential contributions: the branch row carries
            // V(p) - V(n) - E(x) = 0; the entries hold -dE/dx (sign -1)
            // and the RHS receives Eeq = E - sum dE/dx * x_old, which is
            // exactly value + sum(sign * deriv * x_old).
            let mut eq_value = value * scale;

            for jacobian_entry in &matrix_indices.jacobian[program_idx] {
                let model_entry = &program.jacobian_programs[jacobian_entry.jacobian_idx];
                // A non-finite derivative (a model kink such as
                // d(sqrt(x))/dx at x=0) is treated as zero: the residual
                // stays exact and Newton proceeds on the remaining slope.
                let deriv = Self::run_value_program(
                    &mut vm,
                    &model_entry.program,
                    #[cfg(feature = "native")]
                    native,
                    #[cfg(feature = "native")]
                    NativeValueEntry::Jacobian {
                        stamp: program_idx,
                        entry: jacobian_entry.jacobian_idx,
                    },
                )?;
                let deriv = match deriv {
                    v if v.is_finite() => v * scale,
                    _ => continue,
                };

                // Accumulate the companion RHS term once per derivative
                // column. Current contributions (and indirect constraint
                // rows, which stamp the same way onto the branch row)
                // duplicate entries per row with +1/-1 signs: count only
                // the positive copy. Potential contributions carry single
                // -1-signed entries whose sign already encodes the
                // subtraction.
                match (program.branch_ordinal, program.indirect) {
                    (None, _) | (Some(_), true) => {
                        if model_entry.sign > 0.0 {
                            let x_col = Self::axis_value(vm.context, &model_entry.col_axis);
                            eq_value -= deriv * x_col;
                        }
                    }
                    (Some(_), false) => {
                        let x_col = Self::axis_value(vm.context, &model_entry.col_axis);
                        eq_value += model_entry.sign * deriv * x_col;
                    }
                }

                if let (Some(row), Some(col)) = (jacobian_entry.row, jacobian_entry.col) {
                    matrix_add(row, col, jacobian_entry.sign * deriv);
                }
            }

            // RHS: current contributions stamp -/+ Ieq at the KCL rows;
            // potential contributions stamp +Eeq at the branch row
            for entry in &matrix_indices.rhs[program_idx] {
                if let Some(row) = entry.node {
                    rhs_add(row, entry.sign * eq_value);
                }
            }
        }
        Ok(())
    }

    /// Evaluate the model's noise sources at an operating point.
    ///
    /// PSDs come from the contribution expressions' `white_noise` /
    /// `flicker_noise` terms (amplitude-squared scaling folded in at
    /// compile time). Current-contribution sources inject across their
    /// node pair; potential-contribution sources inject at the branch
    /// row as a series EMF, so their PSD is in V²/Hz. Node ids follow the
    /// engine convention (0 = ground). Mode-disabled contributions
    /// contribute nothing.
    pub fn noise_sources(&mut self, circuit_voltages: &[f64]) -> Vec<EvaluatedNoiseSource> {
        self.try_noise_sources(circuit_voltages)
            .unwrap_or_else(|err| {
                panic!(
                    "Verilog-A device '{}' model '{}' noise evaluation failed: {}",
                    self.name, self.model.name, err
                )
            })
    }

    /// Checked noise-source evaluation path for callers that can surface
    /// runtime model diagnostics instead of panicking or dropping sources.
    #[cfg(feature = "native")]
    pub fn try_noise_sources(
        &mut self,
        circuit_voltages: &[f64],
    ) -> Result<Vec<EvaluatedNoiseSource>, VmError> {
        self.try_update_all_voltages(circuit_voltages)?;

        let context = &mut self.context;
        let model = &self.model;
        let program_active = &self.program_active;
        let native = self.native_model.as_ref();

        context.clear_currents();
        context.currents.reserve(model.stamp_programs.len());
        let mut vm = Vm::new(context);
        Self::run_assignment_pass(&mut vm, model, native)?;
        Self::populate_noise_current_probe_cache(&mut vm, model, program_active, native)?;

        let circuit_node = |index: &StampIndex| -> usize {
            match index {
                StampIndex::Terminal(t) => self.node_mapping.get(*t).copied().unwrap_or(0),
                StampIndex::Internal(i) => self.internal_node_indices.get(*i).copied().unwrap_or(0),
                StampIndex::Branch(k) => self.branch_current_indices.get(*k).copied().unwrap_or(0),
                StampIndex::Ground => 0,
            }
        };

        let mut sources = Vec::with_capacity(model.noise_sources.len());
        for (idx, source) in model.noise_sources.iter().enumerate() {
            if !program_active
                .get(source.program_idx)
                .copied()
                .unwrap_or(true)
            {
                continue;
            }
            let psd = Self::run_value_program(
                &mut vm,
                &source.psd_program,
                native,
                NativeValueEntry::NoisePsd(idx),
            )?;
            if !psd.is_finite() {
                continue;
            }
            let psd = psd.max(0.0);
            if psd == 0.0 {
                continue;
            }
            let m = vm.context.multiplicity;
            let psd = if source.is_current { psd * m } else { psd / m };
            let exponent = source
                .exponent_program
                .as_ref()
                .map(|program| {
                    Self::run_value_program(
                        &mut vm,
                        program,
                        native,
                        NativeValueEntry::NoiseExponent(idx),
                    )
                })
                .transpose()?;

            let (node_pos, node_neg) = match (source.is_current, source.branch_ordinal) {
                (false, Some(ordinal)) => (
                    self.branch_current_indices
                        .get(ordinal)
                        .copied()
                        .unwrap_or(0),
                    0,
                ),
                _ => (circuit_node(&source.pos), circuit_node(&source.neg)),
            };

            sources.push(EvaluatedNoiseSource {
                node_pos,
                node_neg,
                psd,
                exponent,
                table: source.table.clone(),
                name: source
                    .name
                    .as_ref()
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| format!("noise{idx}")),
            });
        }
        Ok(sources)
    }

    /// Checked noise-source evaluation path for callers that can surface
    /// runtime model diagnostics instead of panicking or dropping sources.
    #[cfg(not(feature = "native"))]
    pub fn try_noise_sources(
        &mut self,
        circuit_voltages: &[f64],
    ) -> Result<Vec<EvaluatedNoiseSource>, VmError> {
        self.try_update_all_voltages(circuit_voltages)?;

        let context = &mut self.context;
        let model = &self.model;
        let program_active = &self.program_active;

        context.clear_currents();
        context.currents.reserve(model.stamp_programs.len());
        let mut vm = Vm::new(context);
        Self::run_assignment_pass(&mut vm, model)?;
        Self::populate_noise_current_probe_cache(&mut vm, model, program_active)?;

        let circuit_node = |index: &StampIndex| -> usize {
            match index {
                StampIndex::Terminal(t) => self.node_mapping.get(*t).copied().unwrap_or(0),
                StampIndex::Internal(i) => self.internal_node_indices.get(*i).copied().unwrap_or(0),
                StampIndex::Branch(k) => self.branch_current_indices.get(*k).copied().unwrap_or(0),
                StampIndex::Ground => 0,
            }
        };

        let mut sources = Vec::with_capacity(model.noise_sources.len());
        for (idx, source) in model.noise_sources.iter().enumerate() {
            if !program_active
                .get(source.program_idx)
                .copied()
                .unwrap_or(true)
            {
                continue;
            }
            let psd_program = &source.psd_program;
            let psd = vm.execute(psd_program)?;
            if !psd.is_finite() {
                continue;
            }
            let psd = psd.max(0.0);
            if psd == 0.0 {
                continue;
            }
            // m uncorrelated parallel copies: current-noise powers add
            // (x m); series voltage-noise EMFs average (/ m)
            let m = vm.context.multiplicity;
            let psd = if source.is_current { psd * m } else { psd / m };
            let exponent = source
                .exponent_program
                .as_ref()
                .map(|p| vm.execute(p))
                .transpose()?;

            // Potential-contribution noise is a series EMF on the branch
            // equation row; current noise injects across the node pair
            let (node_pos, node_neg) = match (source.is_current, source.branch_ordinal) {
                (false, Some(ordinal)) => (
                    self.branch_current_indices
                        .get(ordinal)
                        .copied()
                        .unwrap_or(0),
                    0,
                ),
                _ => (circuit_node(&source.pos), circuit_node(&source.neg)),
            };

            sources.push(EvaluatedNoiseSource {
                node_pos,
                node_neg,
                psd,
                exponent,
                table: source.table.clone(),
                name: source
                    .name
                    .as_ref()
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| format!("noise{idx}")),
            });
        }
        Ok(sources)
    }

    /// Value of a differentiation axis: a unified node voltage or a
    /// branch-current unknown
    fn axis_value(context: &VmContext, axis: &crate::codegen::ColumnAxis) -> f64 {
        match axis {
            crate::codegen::ColumnAxis::Node(node) => Self::unified_node_voltage(context, *node),
            crate::codegen::ColumnAxis::Branch(k) => context
                .branch_current_values
                .get(*k)
                .copied()
                .unwrap_or(0.0),
        }
    }

    /// Voltage of a unified node index (terminals first, then internal
    /// nodes; usize::MAX is the global reference)
    fn unified_node_voltage(context: &VmContext, node: usize) -> f64 {
        let num_terminals = context.terminal_count();
        if node == usize::MAX {
            0.0
        } else if node < num_terminals {
            context.voltages.get(node).copied().unwrap_or(0.0)
        } else {
            context
                .internal_voltages
                .get(node - num_terminals)
                .copied()
                .unwrap_or(0.0)
        }
    }

    /// Convert a StampIndex to circuit node
    fn index_to_node(
        index: &StampIndex,
        node_mapping: &[usize],
        internal_node_indices: &[usize],
        branch_current_indices: &[usize],
    ) -> Option<usize> {
        match index {
            StampIndex::Terminal(t) => {
                let node = node_mapping.get(*t).copied().unwrap_or(0);
                if node > 0 { Some(node - 1) } else { None }
            }
            StampIndex::Internal(i) => {
                let node = internal_node_indices.get(*i).copied().unwrap_or(0);
                if node > 0 { Some(node - 1) } else { None }
            }
            StampIndex::Branch(k) => {
                let node = branch_current_indices.get(*k).copied().unwrap_or(0);
                if node > 0 { Some(node - 1) } else { None }
            }
            StampIndex::Ground => None,
        }
    }

    /// Convert a stamp index to matrix node index for this device instance.
    pub fn stamp_index_to_node(&self, index: &StampIndex) -> Option<usize> {
        match index {
            StampIndex::Internal(i) => {
                let mapped = self.internal_node_indices.get(*i).copied().unwrap_or(0);
                if mapped > 0 {
                    Some(mapped - 1)
                } else {
                    Some(self.model.num_terminals + *i)
                }
            }
            _ => Self::index_to_node(
                index,
                &self.node_mapping,
                &self.internal_node_indices,
                &self.branch_current_indices,
            ),
        }
    }

    fn infer_current_terminal_pair(
        program: &crate::codegen::StampProgram,
    ) -> Option<(usize, usize)> {
        let mut pos_endpoint = None;
        let mut neg_endpoint = None;

        for loc in &program.stamp_locations {
            let endpoint = match loc.row {
                StampIndex::Terminal(term) => term,
                StampIndex::Ground => CURRENT_PAIR_GROUND,
                _ => continue,
            };

            if loc.sign < 0.0 {
                if pos_endpoint.replace(endpoint).is_some() {
                    return None;
                }
            } else if loc.sign > 0.0 && neg_endpoint.replace(endpoint).is_some() {
                return None;
            }
        }

        match (pos_endpoint, neg_endpoint) {
            (Some(pos), Some(neg)) if pos != neg => Some((pos, neg)),
            _ => None,
        }
    }
}

/// One noise source evaluated at an operating point
#[derive(Debug, Clone)]
pub struct EvaluatedNoiseSource {
    /// Positive injection circuit node (0 = ground); for potential
    /// contributions this is the branch-equation row's unknown
    pub node_pos: usize,
    /// Negative injection circuit node (0 = ground)
    pub node_neg: usize,
    /// Power spectral density at the operating point (A²/Hz for current
    /// contributions, V²/Hz for potential contributions). For table
    /// sources this is the scale applied to the interpolated value.
    pub psd: f64,
    /// Flicker frequency exponent: S(f) = psd / f^exp (None = white)
    pub exponent: Option<f64>,
    /// Frequency-interpolated PSD table: sorted (f, p) points and whether
    /// interpolation runs in log-log coordinates
    pub table: Option<(Vec<(f64, f64)>, bool)>,
    /// Source label
    pub name: String,
}

/// Result of Jacobian computation
#[derive(Debug, Clone)]
pub struct JacobianEntry {
    /// Computed derivative value
    pub value: f64,
    /// Row stamp index
    pub row: StampIndex,
    /// Column stamp index
    pub col: StampIndex,
    /// Index of the stamp program
    pub program_idx: usize,
    /// Index within Jacobian programs
    pub jacobian_idx: usize,
}

/// Builder for creating device instances with parameter overrides
pub struct DeviceBuilder {
    model: CompiledModel,
    name: SmolStr,
    nodes: Vec<usize>,
    params: Vec<(String, f64)>,
    temperature: f64,
}

impl DeviceBuilder {
    /// Create a new builder
    pub fn new(model: CompiledModel, name: impl Into<SmolStr>) -> Self {
        Self {
            model,
            name: name.into(),
            nodes: Vec::new(),
            params: Vec::new(),
            temperature: 300.15, // 27°C
        }
    }

    /// Set terminal connections
    pub fn nodes(mut self, nodes: &[usize]) -> Self {
        self.nodes = nodes.to_vec();
        self
    }

    /// Set a parameter
    pub fn param(mut self, name: &str, value: f64) -> Self {
        self.params.push((name.to_string(), value));
        self
    }

    /// Set temperature
    pub fn temperature(mut self, temp_k: f64) -> Self {
        self.temperature = temp_k;
        self
    }

    /// Build the device
    pub fn build(self) -> VerilogADevice {
        let mut device = VerilogADevice::new(self.name, self.model, &self.nodes);
        device.set_temperature(self.temperature);

        for (name, value) in self.params {
            device.set_parameter(&name, value);
        }
        device.resolve_parameter_defaults();

        device
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::*;
    use crate::codegen::{
        AssignmentProgram, AssignmentStep, BytecodeProgram, ColumnAxis, CompiledNoiseSource,
        JacobianEntry,
    };
    use crate::{CompilerOptions, VerilogACompiler};
    use std::sync::Arc;

    fn compile(source: &str) -> CompiledModel {
        VerilogACompiler::new(CompilerOptions::default())
            .compile(source)
            .expect("Verilog-A source must compile")
    }

    fn native_test_device(model: CompiledModel) -> VerilogADevice {
        let model = Arc::new(model);
        let num_terminals = model.num_terminals;
        let num_internal_nodes = model.internal_nodes;
        let num_branch_unknowns = model.branch_sources.len();
        let num_stamp_programs = model.stamp_programs.len();

        let mut context = VmContext::with_internal_nodes(num_terminals, num_internal_nodes);
        context.port_connected = vec![1; num_terminals];
        for (i, param) in model.parameters.iter().enumerate() {
            context.set_param(i, param.default);
        }
        context.param_given = vec![false; model.parameters.len()];
        context.variables.resize(model.num_variables, 0.0);
        context.lookup_tables = model.lookup_tables.clone();
        context.laplace_filters = model.laplace_filters.clone();
        context.zi_filters = model.zi_filters.clone();
        VerilogADevice::preallocate_vm_runtime_state(&mut context, &model);

        let mut device = VerilogADevice {
            name: SmolStr::new("NTEST"),
            model,
            context,
            node_mapping: vec![0; num_terminals],
            internal_node_indices: vec![0; num_internal_nodes],
            num_internal_nodes,
            branch_current_indices: vec![0; num_branch_unknowns],
            program_active: vec![true; num_stamp_programs],
            branch_active: vec![true; num_branch_unknowns],
            matrix_indices: MatrixIndices::default(),
            native_model: Arc::new(NativeModel::new_for_test(
                0,
                num_stamp_programs,
                vec![0; num_stamp_programs],
                vec![0; num_stamp_programs],
            )),
            prev_discontinuity: false,
        };
        device.context.branch_current_values = vec![0.0; num_branch_unknowns];
        device.rebuild_matrix_indices();
        device
    }

    fn assert_native_hard_fail(err: VmError, feature: &str) {
        let msg = err.to_string();
        assert!(
            msg.contains("native JIT"),
            "error must identify native JIT failure, got: {msg}"
        );
        assert!(
            msg.contains(feature),
            "error must identify {feature}, got: {msg}"
        );
        assert!(
            msg.contains("no interpreter fallback"),
            "error must state the hard-fail contract, got: {msg}"
        );
    }

    #[test]
    fn preallocates_runtime_state_from_all_native_entry_surfaces() {
        let mut model = compile(
            r#"
`include "disciplines.vams"
module prealloc_reactive_noise(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 1.0;
    analog I(p, n) <+ V(p, n) * 0.0;
endmodule
"#,
        );
        assert_eq!(model.stamp_programs.len(), 1);

        model.parameters[0].default_program = Some(BytecodeProgram {
            instructions: vec![Instruction::LimitState(8)],
        });
        model.stamp_programs[0].static_condition = Some(BytecodeProgram {
            instructions: vec![Instruction::AbsDelayState(6)],
        });
        model.stamp_programs[0]
            .reactive_jacobians
            .push(JacobianEntry {
                row: StampIndex::Terminal(0),
                col: StampIndex::Terminal(0),
                col_axis: ColumnAxis::Node(0),
                sign: 1.0,
                program: BytecodeProgram {
                    instructions: vec![
                        Instruction::DdtState(3),
                        Instruction::AbsDelayState(2),
                        Instruction::TransitionState(1),
                    ],
                },
            });
        model.noise_sources.push(CompiledNoiseSource {
            pos: StampIndex::Terminal(0),
            neg: StampIndex::Ground,
            is_current: true,
            branch_ordinal: None,
            program_idx: 0,
            psd_program: BytecodeProgram {
                instructions: vec![Instruction::IdtState(5), Instruction::SlewState(4)],
            },
            exponent_program: Some(BytecodeProgram {
                instructions: vec![Instruction::LimitState(6), Instruction::CrossState(5)],
            }),
            table: None,
            name: None,
        });

        let mut context = VmContext::with_internal_nodes(model.num_terminals, model.internal_nodes);
        VerilogADevice::preallocate_vm_runtime_state(&mut context, &model);

        assert_eq!(context.state_values.len(), 9);
        assert_eq!(context.state_values_prev.len(), 9);
        assert_eq!(context.state_initialized.len(), 9);
        assert_eq!(context.delay_buffers.len(), 7);
        assert_eq!(context.transition_filters.len(), 2);
        assert_eq!(context.slew_filters.len(), 5);
        assert_eq!(context.cross_detectors.len(), 6);
    }

    #[test]
    fn native_runtime_helper_error_reaches_device_as_native_jit_error() {
        let model = compile(
            r#"
`include "disciplines.vams"
module native_laplace_error_path(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ laplace_nd(V(p, n), {1.0}, {1.0, 1.0});
endmodule
"#,
        );

        let mut device = VerilogADevice::try_new("LERR1", model, &[1, 0])
            .expect("laplace model uses native JIT");
        device.update_voltages(&[1.0]);
        assert_eq!(
            device.context.laplace_filters.len(),
            1,
            "fixture must allocate one Laplace filter"
        );
        device.context.laplace_filters.clear();

        let err = device
            .try_evaluate()
            .expect_err("native helper metadata error must hard-fail");

        assert_native_hard_fail(err, "Laplace");
    }

    #[test]
    fn native_state_storage_preflights_before_dispatch() {
        let source = r#"
`include "disciplines.vams"
module state_storage_preflight(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ ddt(V(p, n));
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile state model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile state canonical IR");
        let mut device =
            VerilogADevice::try_new_with_canonical_ir("STATEPRE1", model, &artifact, &[1, 0])
                .expect("state model uses native JIT");
        assert_eq!(
            device.context.state_values.len(),
            1,
            "fixture must allocate one state slot"
        );

        device.context.state_values.clear();
        let err = device
            .try_evaluate()
            .expect_err("missing state storage must preflight before native dispatch");

        assert_native_hard_fail(err, "state-value storage");
    }

    #[test]
    fn native_transition_storage_preflights_before_dispatch() {
        let source = r#"
`include "disciplines.vams"
module transition_storage_preflight(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ transition(V(p, n), 0.0, 1.0e-9, 1.0e-9);
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile transition model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile transition canonical IR");
        let mut device =
            VerilogADevice::try_new_with_canonical_ir("TRANPRE1", model, &artifact, &[1, 0])
                .expect("transition model uses native JIT");
        assert_eq!(
            device.context.transition_filters.len(),
            1,
            "fixture must allocate one transition filter"
        );

        device.context.transition_filters.clear();
        let err = device
            .try_evaluate()
            .expect_err("missing transition filter storage must preflight before native dispatch");

        assert_native_hard_fail(err, "transition filter storage");
    }

    #[test]
    fn native_current_pair_preflight_errors_use_hard_fail_contract() {
        let context = VmContext::with_internal_nodes(0, 0);
        let err = VerilogADevice::validate_native_current_pairs(&context, &[0])
            .expect_err("missing terminal-pair storage must hard-fail in native mode");

        assert_native_hard_fail(err, "terminal-pair current slot 0");
    }

    #[test]
    fn native_missing_noise_exponent_entry_uses_hard_fail_contract() {
        let err = VerilogADevice::missing_native_noise_exponent_entry(2);

        assert_native_hard_fail(err, "noise exponent entry");
    }

    #[test]
    fn native_noise_exponent_missing_entry_preflights_before_dependency_tables() {
        let native = NativeModel::new_for_test(0, 0, vec![], vec![]);
        let mut context = VmContext::with_internal_nodes(0, 0);
        let mut vm = Vm::new(&mut context);
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushConst(1.0)],
        };

        let err = VerilogADevice::run_value_program(
            &mut vm,
            &program,
            &native,
            NativeValueEntry::NoiseExponent(0),
        )
        .expect_err("missing optional native noise exponent entry must not index dependencies");

        assert_native_hard_fail(err, "noise exponent entry");
    }

    #[test]
    fn native_missing_parameter_default_entry_hard_fails_without_bytecode_execution() {
        let model = compile(
            r#"
`include "disciplines.vams"
module dependent_default(p, n);
    inout p, n;
    electrical p, n;
    parameter real base = 2.0;
    parameter real derived = base * 3.0;
    analog I(p, n) <+ V(p, n) * derived;
endmodule
"#,
        );
        assert!(
            model
                .parameters
                .iter()
                .any(|param| param.default_program.is_some()),
            "fixture must contain a dependent parameter default program"
        );

        let mut device = native_test_device(model);
        let err = device
            .try_resolve_parameter_defaults()
            .expect_err("native mode must not execute dependent default bytecode fallback");

        assert_native_hard_fail(err, "missing parameter-default entry");
    }

    #[test]
    fn native_static_condition_refresh_uses_native_entries_without_bytecode_execution() {
        let mut model = compile(
            r#"
`include "disciplines.vams"
module static_condition(p, n);
    inout p, n;
    electrical p, n;
    parameter real enabled = 1.0;
    real guard;
    analog begin
        guard = enabled;
        if (guard)
            I(p, n) <+ V(p, n) * 1.0e-3;
    end
endmodule
"#,
        );
        assert!(
            model
                .stamp_programs
                .iter()
                .any(|program| program.static_condition.is_some()),
            "fixture must contain a static condition program"
        );
        assert!(
            !model.assignment_steps.is_empty(),
            "fixture must contain assignment bytecode"
        );
        model.assignment_steps = vec![AssignmentStep::Assign(AssignmentProgram {
            var_index: 0,
            program: BytecodeProgram {
                instructions: vec![Instruction::PushParam(999)],
            },
        })];
        for program in &mut model.stamp_programs {
            if program.static_condition.is_some() {
                program.static_condition = Some(BytecodeProgram {
                    instructions: vec![Instruction::PushParam(999)],
                });
            }
        }

        let mut device = native_test_device(model);
        device
            .try_resolve_parameter_defaults()
            .expect("native mode must refresh static conditions without bytecode fallback");

        assert!(device.program_active.iter().all(|active| *active));
    }

    #[test]
    fn native_analysis_static_condition_refreshes_when_analysis_type_changes() {
        let source = r#"
`include "disciplines.vams"
module analysis_static_condition(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if (analysis("static"))
            I(p, n) <+ V(p, n);
    end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler
            .compile(source)
            .expect("compile analysis static-condition model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile analysis static-condition canonical IR");
        assert!(
            model
                .stamp_programs
                .iter()
                .any(|program| program.static_condition.is_some()),
            "analysis(\"static\") guard should be peeled into a topology guard"
        );

        let mut device =
            VerilogADevice::try_new_with_canonical_ir("ANSTATIC1", model, &artifact, &[1, 0])
                .expect("analysis static condition compiles natively");

        assert!(
            device.program_active.iter().all(|active| *active),
            "default DC analysis should activate analysis(\"static\") guarded stamps"
        );

        device
            .try_set_analysis_type(2)
            .expect("transient analysis change refreshes native static conditions");
        assert!(
            device.program_active.iter().all(|active| !*active),
            "transient analysis should deactivate analysis(\"static\") guarded stamps"
        );

        device
            .try_set_analysis_type(4)
            .expect("IC analysis change refreshes native static conditions");
        assert!(
            device.program_active.iter().all(|active| *active),
            "IC analysis should reactivate analysis(\"static\") guarded stamps"
        );
    }

    #[test]
    fn native_static_refresh_without_conditions_does_not_execute_assignment_bytecode() {
        let mut model = compile(
            r#"
`include "disciplines.vams"
module unconditional_vsource(p, n);
    inout p, n;
    electrical p, n;
    real value;
    analog begin
        value = 1.0;
        V(p, n) <+ value;
    end
endmodule
"#,
        );
        assert!(
            !model.assignment_steps.is_empty(),
            "fixture must contain assignment bytecode"
        );
        assert!(
            !model.branch_sources.is_empty(),
            "fixture must contain a branch-current unknown"
        );
        assert!(
            model
                .stamp_programs
                .iter()
                .all(|program| program.static_condition.is_none()),
            "fixture must not contain static condition programs"
        );
        model.assignment_steps = vec![AssignmentStep::Assign(AssignmentProgram {
            var_index: 0,
            program: BytecodeProgram {
                instructions: vec![Instruction::PushParam(999)],
            },
        })];

        let mut device = native_test_device(model);
        device.refresh_static_conditions();

        assert!(device.program_active.iter().all(|active| *active));
        assert!(device.branch_active.iter().all(|active| *active));
    }
}
