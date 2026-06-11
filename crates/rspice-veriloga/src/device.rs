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
//! When the `native` feature is enabled and a C compiler is available,
//! the device will automatically compile to native code for maximum performance.
//! Falls back gracefully to the bytecode VM interpreter otherwise.

use crate::codegen::{CompiledModel, Instruction, StampIndex};
use crate::vm::{Vm, VmContext};
use smol_str::SmolStr;

#[cfg(feature = "native")]
use crate::native::NativeModel;

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
    /// Native compiled model (if compilation succeeded)
    #[cfg(feature = "native")]
    native_model: Option<std::sync::Arc<NativeModel>>,
    /// Variable storage for native execution
    #[cfg(feature = "native")]
    native_vars: Vec<f64>,
}

// NativeModel contains raw pointers but is safe to send across threads
// because the pointers are only used via the evaluate functions
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
    /// * `model` - Compiled Verilog-A model
    /// * `nodes` - Circuit node IDs for each terminal (0 = ground)
    pub fn new(name: impl Into<SmolStr>, model: CompiledModel, nodes: &[usize]) -> Self {
        let num_terminals = model.num_terminals;

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
        Self::preallocate_vm_runtime_state(&mut context, &model);

        // Attempt native compilation (if feature enabled)
        #[cfg(feature = "native")]
        let (native_model, native_vars) = Self::try_native_compile(&model);

        let num_branch_unknowns = model.branch_sources.len();
        let num_stamp_programs = model.stamp_programs.len();
        let mut device = Self {
            name: name.into(),
            model: std::sync::Arc::new(model),
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
            #[cfg(feature = "native")]
            native_vars,
        };
        device.context.branch_current_values = vec![0.0; num_branch_unknowns];
        device.rebuild_matrix_indices();
        device.resolve_parameter_defaults();
        device
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

        scan_steps(&model.assignment_steps, &mut scan_program);

        for stamp in &model.stamp_programs {
            scan_program(&stamp.value_program);
            for jac in &stamp.jacobian_programs {
                scan_program(&jac.program);
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

    /// Attempt to compile the model to native code using Cranelift JIT
    #[cfg(feature = "native")]
    fn try_native_compile(
        model: &CompiledModel,
    ) -> (Option<std::sync::Arc<NativeModel>>, Vec<f64>) {
        use crate::native::try_compile_native;

        let num_vars = model.num_variables;

        match try_compile_native(model) {
            Some(native) => {
                log::info!("[JIT] Model '{}' compiled to native code", model.name);
                #[cfg(debug_assertions)]
                eprintln!("[JIT] Model '{}' compiled to native code", model.name);
                (Some(std::sync::Arc::new(native)), vec![0.0; num_vars])
            }
            None => {
                log::debug!("[JIT] Model '{}' using interpreter", model.name);
                #[cfg(debug_assertions)]
                eprintln!("[JIT] Model '{}' using interpreter", model.name);
                (None, vec![0.0; num_vars])
            }
        }
    }

    /// Check if this device is using native compiled code
    ///
    /// Returns true if native compilation succeeded and the device
    /// will use native code for evaluation. Returns false if using
    /// the VM interpreter.
    #[cfg(feature = "native")]
    pub fn is_using_native(&self) -> bool {
        self.native_model.is_some()
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
    pub fn set_parameter(&mut self, name: &str, value: f64) -> bool {
        for (i, param) in self.model.parameters.iter().enumerate() {
            if param.name == name {
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
                return true;
            }
        }
        false
    }

    /// Evaluate dependent parameter defaults for parameters the instance
    /// did not set, in declaration order.
    ///
    /// Must be called after all instance parameters have been applied;
    /// calling it again is harmless (it is idempotent for a fixed set of
    /// given parameters).
    pub fn resolve_parameter_defaults(&mut self) {
        for i in 0..self.model.parameters.len() {
            if self.context.is_param_given(i) {
                continue;
            }
            let Some(program) = self.model.parameters[i].default_program.clone() else {
                continue;
            };
            let value = {
                let mut vm = Vm::new(&mut self.context);
                vm.execute(&program).unwrap_or(0.0)
            };
            let (min, max) = (
                self.model.parameters[i].min,
                self.model.parameters[i].max,
            );
            let clamped = match (min, max) {
                (Some(min), Some(max)) => value.clamp(min, max),
                (Some(min), None) => value.max(min),
                (None, Some(max)) => value.min(max),
                (None, None) => value,
            };
            self.context.set_param(i, clamped);
        }

        // Topology guards depend on final parameter values
        self.refresh_static_conditions();
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
        self.context.analysis_type = analysis;
    }

    /// Commit integrator state after an accepted timestep
    pub fn advance_state(&mut self) {
        self.context.advance_state();
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
    fn refresh_static_conditions(&mut self) {
        let model = &self.model;
        let mut program_active = vec![true; model.stamp_programs.len()];
        let mut branch_active = vec![false; model.branch_sources.len()];

        {
            let mut vm = Vm::new(&mut self.context);
            // Static guards may reference instance-static variables (e.g.
            // BSIM4rdsMod derived from the rdsmod parameter); run the
            // evaluation stream once so those variables hold their values.
            // Node voltages are irrelevant to instance-static expressions.
            Self::execute_assignment_programs(&mut vm, model);
            for (idx, program) in model.stamp_programs.iter().enumerate() {
                let active = match &program.static_condition {
                    Some(condition) => vm.execute(condition).map(|v| v != 0.0).unwrap_or(true),
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
        self.update_all_voltages(circuit_voltages);

        let context = &mut self.context;
        let model = &self.model;
        let matrix_indices = &self.matrix_indices;
        let program_active = &self.program_active;

        context.clear_currents();

        let mut vm = Vm::new(context);
        Self::execute_assignment_programs(&mut vm, model);

        for (program_idx, program) in model.stamp_programs.iter().enumerate() {
            if !program_active.get(program_idx).copied().unwrap_or(true) {
                continue;
            }

            for entry in &matrix_indices.reactive[program_idx] {
                let model_entry = &program.reactive_jacobians[entry.jacobian_idx];
                let deriv = match vm.execute(&model_entry.program) {
                    Ok(v) if v.is_finite() => v,
                    _ => continue,
                };
                if let (Some(row), Some(col)) = (entry.row, entry.col) {
                    matrix_add(row, col, entry.sign * deriv);
                }
            }
        }
    }

    /// Update terminal voltages from circuit solution
    ///
    /// Called before evaluating device equations.
    pub fn update_voltages(&mut self, circuit_voltages: &[f64]) {
        for (terminal, &node) in self.node_mapping.iter().enumerate() {
            if terminal < self.context.voltages.len() {
                let v = if node == 0 {
                    0.0
                } else if node <= circuit_voltages.len() {
                    circuit_voltages[node - 1]
                } else {
                    0.0
                };
                self.context.voltages[terminal] = v;
            }
        }
    }

    /// Update both terminal and internal node voltages from circuit solution
    ///
    /// This is the full-featured method for solver integration.
    pub fn update_all_voltages(&mut self, circuit_voltages: &[f64]) {
        // Update terminal voltages
        self.update_voltages(circuit_voltages);

        // Update internal node voltages
        for (internal_idx, &circuit_node) in self.internal_node_indices.iter().enumerate() {
            if internal_idx < self.context.internal_voltages.len() {
                let v = if circuit_node == 0 {
                    0.0
                } else if circuit_node <= circuit_voltages.len() {
                    circuit_voltages[circuit_node - 1]
                } else {
                    0.0
                };
                self.context.internal_voltages[internal_idx] = v;
            }
        }

        // Update branch-current unknown values
        for (ordinal, &circuit_node) in self.branch_current_indices.iter().enumerate() {
            if ordinal < self.context.branch_current_values.len() {
                let v = if circuit_node == 0 {
                    0.0
                } else if circuit_node <= circuit_voltages.len() {
                    circuit_voltages[circuit_node - 1]
                } else {
                    0.0
                };
                self.context.branch_current_values[ordinal] = v;
            }
        }
    }

    /// Evaluate the device: compute branch current
    ///
    /// Returns the current for each branch equation.
    /// Uses native compiled code if available, otherwise falls back to VM interpreter.
    pub fn evaluate(&mut self) -> Vec<f64> {
        // Try native evaluation if available
        #[cfg(feature = "native")]
        if self.native_model.is_some() {
            return self.evaluate_native();
        }

        // Fall back to VM interpreter
        self.evaluate_interpreter()
    }

    /// Evaluate using the bytecode VM interpreter
    fn evaluate_interpreter(&mut self) -> Vec<f64> {
        self.context.clear_currents();

        let program_active = &self.program_active;
        let mut vm = Vm::new(&mut self.context);
        Self::execute_assignment_programs(&mut vm, &self.model);
        let mut currents = Vec::with_capacity(self.model.stamp_programs.len());

        for (program_idx, program) in self.model.stamp_programs.iter().enumerate() {
            if !program_active.get(program_idx).copied().unwrap_or(true) {
                currents.push(0.0);
                vm.context.currents.push(0.0);
                continue;
            }
            match vm.execute(&program.value_program) {
                Ok(value) => {
                    currents.push(value);
                    vm.context.currents.push(value);
                    if program.branch_ordinal.is_none()
                        && let Some((pos, neg)) = Self::infer_current_terminal_pair(program)
                    {
                        vm.context.set_branch_current(pos, neg, value);
                    }
                }
                Err(_) => {
                    currents.push(0.0);
                    vm.context.currents.push(0.0);
                }
            }
        }

        currents
    }

    /// Build a native evaluation context snapshot.
    #[cfg(feature = "native")]
    fn native_eval_context(&mut self) -> crate::native::EvalContext {
        crate::native::EvalContext {
            voltages: self.context.voltages.as_ptr(),
            internal_voltages: self.context.internal_voltages.as_ptr(),
            params: self.context.parameters.as_ptr(),
            branch_currents: self.context.terminal_pair_currents_ptr(),
            branch_currents_len: self.context.terminal_pair_currents_len(),
            currents: self.context.currents.as_ptr(),
            currents_len: self.context.currents.len(),
            num_terminals: self.context.terminal_count(),
            temperature: self.context.temperature,
            time: self.context.time,
            timestep: self.context.timestep,
            // Pass null for empty vecs - as_ptr() on empty vec gives dangling non-null pointer
            state_prev: if self.context.state_values_prev.is_empty() {
                std::ptr::null()
            } else {
                self.context.state_values_prev.as_ptr()
            },
            lookup_tables: if self.context.lookup_tables.is_empty() {
                std::ptr::null()
            } else {
                self.context.lookup_tables.as_ptr()
            },
            lookup_tables_len: self.context.lookup_tables.len(),
            laplace_filters: if self.context.laplace_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                self.context.laplace_filters.as_mut_ptr()
            },
            laplace_filters_len: self.context.laplace_filters.len(),
        }
    }

    /// Evaluate using Cranelift JIT compiled code
    #[cfg(feature = "native")]
    fn evaluate_native(&mut self) -> Vec<f64> {
        let native = std::sync::Arc::clone(self.native_model.as_ref().unwrap());

        self.context.clear_currents();
        // Pre-reserve so currents pointer remains stable across stamp pushes.
        self.context
            .currents
            .reserve(self.model.stamp_programs.len());

        // First, compute all variable assignments
        let assignment_ctx = self.native_eval_context();
        native.evaluate_assignments(&assignment_ctx, &mut self.native_vars);

        // Then evaluate each stamp program
        let mut stamp_values = Vec::with_capacity(native.num_stamps);
        for i in 0..native.num_stamps {
            let stamp_ctx = self.native_eval_context();
            let value = native.evaluate_stamp(i, &stamp_ctx, &self.native_vars);
            stamp_values.push(value);
            self.context.currents.push(value);
            if let Some(program) = self.model.stamp_programs.get(i)
                && let Some((pos, neg)) = Self::infer_current_terminal_pair(program)
            {
                self.context.set_branch_current(pos, neg, value);
            }
        }

        stamp_values
    }

    /// Safety cap on runtime-loop iterations per evaluation (a model bug
    /// must not hang the Newton loop)
    const MAX_RUNTIME_LOOP_ITERATIONS: usize = 100_000;

    /// Execute assignment programs and update VM variable storage.
    fn execute_assignment_programs(vm: &mut Vm<'_>, model: &CompiledModel) {
        if vm.context.variables.len() < model.num_variables {
            vm.context.variables.resize(model.num_variables, 0.0);
        }

        Self::execute_assignment_steps(vm, &model.assignment_steps, &model.name);
    }

    /// Execute a sequence of evaluation steps (assignments and runtime
    /// loops), recursively
    fn execute_assignment_steps(
        vm: &mut Vm<'_>,
        steps: &[crate::codegen::AssignmentStep],
        model_name: &str,
    ) {
        for step in steps {
            match step {
                crate::codegen::AssignmentStep::Assign(assignment) => {
                    let value = vm.execute(&assignment.program).unwrap_or(0.0);
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
                    match slot {
                        Ok(slot) => {
                            let value = vm.execute(value).unwrap_or(0.0);
                            if slot < vm.context.variables.len() {
                                vm.context.variables[slot] = value;
                            }
                        }
                        Err(err) => {
                            log::warn!(
                                "Verilog-A model '{model_name}': skipping array element \
                                 assignment: {err}"
                            );
                        }
                    }
                }
                crate::codegen::AssignmentStep::Loop { condition, body } => {
                    let mut iterations = 0usize;
                    loop {
                        let active = vm.execute(condition).unwrap_or(0.0);
                        if active == 0.0 {
                            break;
                        }
                        Self::execute_assignment_steps(vm, body, model_name);
                        iterations += 1;
                        if iterations >= Self::MAX_RUNTIME_LOOP_ITERATIONS {
                            log::warn!(
                                "Verilog-A model '{}': runtime loop exceeded {} iterations; \
                                 aborting the loop (check the loop bounds)",
                                model_name,
                                Self::MAX_RUNTIME_LOOP_ITERATIONS
                            );
                            break;
                        }
                    }
                }
            }
        }
    }

    /// Compute Jacobian entries
    ///
    /// Returns (value, row_terminal, col_terminal, is_current) for each derivative.
    pub fn compute_jacobian(&mut self) -> Vec<JacobianEntry> {
        let context = &mut self.context;
        let model = &self.model;

        context.clear_currents();

        let program_active = &self.program_active;
        let mut vm = Vm::new(context);
        Self::execute_assignment_programs(&mut vm, model);
        let mut entries = Vec::new();

        for (prog_idx, program) in model.stamp_programs.iter().enumerate() {
            if !program_active.get(prog_idx).copied().unwrap_or(true) {
                vm.context.currents.push(0.0);
                continue;
            }
            let value = vm.execute(&program.value_program).unwrap_or(0.0);
            vm.context.currents.push(value);
            if program.branch_ordinal.is_none()
                && let Some((pos, neg)) = Self::infer_current_terminal_pair(program)
            {
                vm.context.set_branch_current(pos, neg, value);
            }

            for (jac_idx, jac_entry) in program.jacobian_programs.iter().enumerate() {
                if let Ok(value) = vm.execute(&jac_entry.program) {
                    entries.push(JacobianEntry {
                        value: jac_entry.sign * value,
                        row: jac_entry.row.clone(),
                        col: jac_entry.col.clone(),
                        program_idx: prog_idx,
                        jacobian_idx: jac_idx,
                    });
                }
            }
        }

        entries
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
        // Update context with the full solution (terminals, internal
        // nodes, and branch-current unknowns)
        self.update_all_voltages(circuit_voltages);

        // Extract disjoint fields to satisfy borrow checker
        let context = &mut self.context;
        let model = &self.model;
        let matrix_indices = &self.matrix_indices;

        context.clear_currents();

        let mut vm = Vm::new(context);
        Self::execute_assignment_programs(&mut vm, model);

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

            if let Some(p) = pos {
                matrix_add(p, br, 1.0);
                matrix_add(br, p, 1.0);
            }
            if let Some(n) = neg {
                matrix_add(n, br, -1.0);
                matrix_add(br, n, -1.0);
            }
        }

        for (program_idx, program) in model.stamp_programs.iter().enumerate() {
            if !self.program_active.get(program_idx).copied().unwrap_or(true) {
                continue;
            }

            // Compute the contribution value (branch current for current
            // contributions, source voltage for potential contributions).
            // Non-finite values would poison the whole system; skip the
            // program and let Newton damping recover.
            let value = match vm.execute(&program.value_program) {
                Ok(v) if v.is_finite() => v,
                _ => continue,
            };

            vm.context.currents.push(value);
            if program.branch_ordinal.is_none()
                && let Some((pos, neg)) = Self::infer_current_terminal_pair(program)
            {
                vm.context.set_branch_current(pos, neg, value);
            }

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
            let mut eq_value = value;

            for jacobian_entry in &matrix_indices.jacobian[program_idx] {
                let model_entry = &program.jacobian_programs[jacobian_entry.jacobian_idx];
                // A non-finite derivative (a model kink such as
                // d(sqrt(x))/dx at x=0) is treated as zero: the residual
                // stays exact and Newton proceeds on the remaining slope.
                let deriv = match vm.execute(&model_entry.program) {
                    Ok(v) if v.is_finite() => v,
                    _ => continue,
                };

                // Accumulate the companion RHS term once per derivative
                // column. Current contributions duplicate entries per KCL
                // row (+1/-1): count only the positive copy. Potential
                // contributions carry single -1-signed entries whose sign
                // already encodes the subtraction.
                match program.branch_ordinal {
                    None => {
                        if model_entry.sign > 0.0 {
                            let x_col =
                                Self::axis_value(vm.context, &model_entry.col_axis);
                            eq_value -= deriv * x_col;
                        }
                    }
                    Some(_) => {
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
    }

    /// Value of a differentiation axis: a unified node voltage or a
    /// branch-current unknown
    fn axis_value(context: &VmContext, axis: &crate::codegen::ColumnAxis) -> f64 {
        match axis {
            crate::codegen::ColumnAxis::Node(node) => {
                Self::unified_node_voltage(context, *node)
            }
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
        let mut pos_terminal = None;
        let mut neg_terminal = None;

        for loc in &program.stamp_locations {
            let terminal = match loc.row {
                StampIndex::Terminal(term) => term,
                _ => continue,
            };

            if loc.sign < 0.0 {
                if pos_terminal.replace(terminal).is_some() {
                    return None;
                }
            } else if loc.sign > 0.0 && neg_terminal.replace(terminal).is_some() {
                return None;
            }
        }

        match (pos_terminal, neg_terminal) {
            (Some(pos), Some(neg)) if pos != neg => Some((pos, neg)),
            _ => None,
        }
    }
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
