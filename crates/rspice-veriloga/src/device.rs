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

use crate::codegen::{CompiledModel, StampIndex};
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
    /// Compiled model (shared reference would be better, but owned for simplicity)
    model: CompiledModel,
    /// Runtime execution context
    context: VmContext,
    /// Mapping from terminal index to circuit node ID (0 = ground)
    node_mapping: Vec<usize>,
    /// Mapping from internal node index to circuit node ID
    /// When the solver allocates circuit nodes for internal nodes, this maps them
    internal_node_indices: Vec<usize>,
    /// Number of internal nodes in this device
    num_internal_nodes: usize,
    /// Pre-computed matrix indices for O(1) stamping
    #[allow(dead_code)]
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
    /// (row, col, program_idx, jacobian_idx) for each Jacobian entry
    pub jacobian: Vec<JacobianIndex>,
    /// (node, sign) for RHS contributions
    pub rhs: Vec<RhsIndex>,
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

        // Initialize parameters to defaults
        for (i, param) in model.parameters.iter().enumerate() {
            context.set_param(i, param.default);
        }

        // Build matrix indices (will be set during circuit linking)
        let matrix_indices = MatrixIndices::default();

        // Attempt native compilation (if feature enabled)
        #[cfg(feature = "native")]
        let (native_model, native_vars) = Self::try_native_compile(&model);

        Self {
            name: name.into(),
            model,
            context,
            node_mapping,
            internal_node_indices: vec![0; num_internal_nodes],
            num_internal_nodes,
            matrix_indices,
            #[cfg(feature = "native")]
            native_model,
            #[cfg(feature = "native")]
            native_vars,
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
                self.context.set_param(i, clamped);
                return true;
            }
        }
        false
    }

    /// Set simulation temperature in Kelvin
    pub fn set_temperature(&mut self, temp_k: f64) {
        self.context.temperature = temp_k;
    }

    /// Set simulation time
    pub fn set_time(&mut self, time: f64) {
        self.context.time = time;
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
    }

    /// Get the circuit node index for an internal node
    pub fn internal_node_index(&self, internal_idx: usize) -> Option<usize> {
        self.internal_node_indices.get(internal_idx).copied()
    }

    /// Build mapped RHS stamp rows for each stamp program.
    ///
    /// Returns one entry per stamp program; each program entry contains
    /// `(node_index, sign)` pairs for non-ground RHS rows.
    pub fn mapped_rhs_rows(&self) -> Vec<Vec<(usize, f64)>> {
        self.model
            .stamp_programs
            .iter()
            .map(|program| {
                program
                    .stamp_locations
                    .iter()
                    .filter_map(|loc| {
                        Self::index_to_node(
                            &loc.row,
                            &self.node_mapping,
                            &self.internal_node_indices,
                        )
                        .map(|row| (row, loc.sign))
                    })
                    .collect()
            })
            .collect()
    }

    /// Build mapped Jacobian matrix locations for each stamp program.
    ///
    /// Returns one entry per stamp program; each program entry contains
    /// `(row, col)` locations for each Jacobian program where `None` means ground.
    pub fn mapped_jacobian_locations(&self) -> Vec<Vec<(Option<usize>, Option<usize>)>> {
        self.model
            .stamp_programs
            .iter()
            .map(|program| {
                program
                    .jacobian_programs
                    .iter()
                    .map(|jac| {
                        (
                            Self::index_to_node(
                                &jac.row,
                                &self.node_mapping,
                                &self.internal_node_indices,
                            ),
                            Self::index_to_node(
                                &jac.col,
                                &self.node_mapping,
                                &self.internal_node_indices,
                            ),
                        )
                    })
                    .collect()
            })
            .collect()
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
        let mut vm = Vm::new(&mut self.context);
        let mut currents = Vec::with_capacity(self.model.stamp_programs.len());

        for program in &self.model.stamp_programs {
            match vm.execute(&program.value_program) {
                Ok(value) => currents.push(value),
                Err(_) => currents.push(0.0),
            }
        }

        currents
    }

    /// Evaluate using Cranelift JIT compiled code
    #[cfg(feature = "native")]
    fn evaluate_native(&mut self) -> Vec<f64> {
        use crate::native::EvalContext;

        let native = self.native_model.as_ref().unwrap();

        // Build evaluation context
        let ctx = EvalContext {
            voltages: self.context.voltages.as_ptr(),
            internal_voltages: self.context.internal_voltages.as_ptr(),
            params: self.context.parameters.as_ptr(),
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
            laplace_filters: if self.model.laplace_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                self.model.laplace_filters.as_ptr() as *mut _
            },
            laplace_filters_len: self.model.laplace_filters.len(),
        };

        // First, compute all variable assignments
        native.evaluate_assignments(&ctx, &mut self.native_vars);

        // Then evaluate each stamp program
        let mut stamp_values = Vec::with_capacity(native.num_stamps);
        for i in 0..native.num_stamps {
            let value = native.evaluate_stamp(i, &ctx, &self.native_vars);
            stamp_values.push(value);
        }

        stamp_values
    }

    /// Compute Jacobian entries
    ///
    /// Returns (value, row_terminal, col_terminal, is_current) for each derivative.
    pub fn compute_jacobian(&mut self) -> Vec<JacobianEntry> {
        let context = &mut self.context;
        let model = &self.model;

        let mut vm = Vm::new(context);
        let mut entries = Vec::new();

        for (prog_idx, program) in model.stamp_programs.iter().enumerate() {
            for (jac_idx, jac_entry) in program.jacobian_programs.iter().enumerate() {
                match vm.execute(&jac_entry.program) {
                    Ok(value) => {
                        entries.push(JacobianEntry {
                            value,
                            row: jac_entry.row.clone(),
                            col: jac_entry.col.clone(),
                            program_idx: prog_idx,
                            jacobian_idx: jac_idx,
                        });
                    }
                    Err(_) => {}
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
        // Update context with current voltages
        self.update_voltages(circuit_voltages);

        // Extract disjoint fields to satisfy borrow checker
        let context = &mut self.context;
        let model = &self.model;
        let node_mapping = &self.node_mapping;
        let internal_node_indices = &self.internal_node_indices;

        let mut vm = Vm::new(context);

        for program in &model.stamp_programs {
            // Compute the branch current/value
            let value = match vm.execute(&program.value_program) {
                Ok(v) => v,
                Err(_) => continue,
            };

            // Stamp RHS contributions
            for loc in &program.stamp_locations {
                let row_node = Self::index_to_node(&loc.row, node_mapping, internal_node_indices);
                if let Some(row) = row_node {
                    rhs_add(row, loc.sign * value);
                }
            }

            // Stamp Jacobian entries
            for jac in &program.jacobian_programs {
                let deriv = match vm.execute(&jac.program) {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                let row_node = Self::index_to_node(&jac.row, node_mapping, internal_node_indices);
                let col_node = Self::index_to_node(&jac.col, node_mapping, internal_node_indices);

                if let (Some(row), Some(col)) = (row_node, col_node) {
                    matrix_add(row, col, deriv);
                }
            }
        }
    }

    /// Convert a StampIndex to circuit node
    fn index_to_node(
        index: &StampIndex,
        node_mapping: &[usize],
        internal_node_indices: &[usize],
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
            StampIndex::Ground => None,
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

        device
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::{
        BytecodeProgram, CompiledModel, CompiledParameter, Instruction, JacobianEntry as JacEntry,
        StampLocation, StampProgram,
    };

    fn create_simple_resistor_model() -> CompiledModel {
        // I = G * V(p, n)
        // Jacobian: dI/dVp = G, dI/dVn = -G

        let value_program = BytecodeProgram {
            instructions: vec![
                Instruction::PushParam(0),      // G (conductance)
                Instruction::PushVoltage(0, 1), // V(p, n)
                Instruction::Mul,               // G * V
            ],
        };

        let jacobian_pos = BytecodeProgram {
            instructions: vec![Instruction::PushParam(0)], // dI/dVp = G
        };

        let jacobian_neg = BytecodeProgram {
            instructions: vec![Instruction::PushParam(0), Instruction::Neg], // dI/dVn = -G
        };

        CompiledModel {
            name: "resistor".into(),
            num_terminals: 2,
            terminal_names: vec!["p".into(), "n".into()],
            parameters: vec![CompiledParameter {
                name: "g".into(),
                default: 0.001, // 1kΩ
                min: Some(0.0),
                max: None,
            }],
            num_variables: 0,
            assignment_programs: vec![],
            stamp_programs: vec![StampProgram {
                stamp_locations: vec![
                    StampLocation {
                        row: StampIndex::Terminal(0),
                        col: StampIndex::Ground,
                        sign: 1.0,
                    },
                    StampLocation {
                        row: StampIndex::Terminal(1),
                        col: StampIndex::Ground,
                        sign: -1.0,
                    },
                ],
                value_program,
                jacobian_programs: vec![
                    JacEntry {
                        row: StampIndex::Terminal(0),
                        col: StampIndex::Terminal(0),
                        program: jacobian_pos,
                    },
                    JacEntry {
                        row: StampIndex::Terminal(0),
                        col: StampIndex::Terminal(1),
                        program: jacobian_neg,
                    },
                ],
            }],
            lookup_tables: vec![],
            internal_nodes: 0,
            branch_currents: 0,
            laplace_filters: vec![],
        }
    }

    fn create_internal_node_model() -> CompiledModel {
        // Simple model with one internal node
        // I = V(terminal) - V(internal)
        let value_program = BytecodeProgram {
            instructions: vec![
                Instruction::PushVoltage(0, 1),      // V(terminal)
                Instruction::PushInternalVoltage(0), // V(internal)
                Instruction::Sub,
            ],
        };

        CompiledModel {
            name: "internal_node_test".into(),
            num_terminals: 2,
            terminal_names: vec!["p".into(), "n".into()],
            parameters: vec![],
            num_variables: 0,
            assignment_programs: vec![],
            stamp_programs: vec![StampProgram {
                stamp_locations: vec![StampLocation {
                    row: StampIndex::Terminal(0),
                    col: StampIndex::Ground,
                    sign: 1.0,
                }],
                value_program,
                jacobian_programs: vec![],
            }],
            lookup_tables: vec![],
            internal_nodes: 1,
            branch_currents: 0,
            laplace_filters: vec![],
        }
    }

    fn create_diode_model() -> CompiledModel {
        // I = Is * (exp(V/Vt) - 1)
        let value_program = BytecodeProgram {
            instructions: vec![
                Instruction::PushParam(0),      // Is
                Instruction::PushVoltage(0, 1), // V(anode, cathode)
                Instruction::PushVt,            // Vt
                Instruction::Div,               // V/Vt
                Instruction::Limexp,            // limexp(V/Vt)
                Instruction::PushConst(1.0),
                Instruction::Sub, // exp(V/Vt) - 1
                Instruction::Mul, // Is * (exp(V/Vt) - 1)
            ],
        };

        // dI/dV = Is/Vt * exp(V/Vt)
        let jacobian = BytecodeProgram {
            instructions: vec![
                Instruction::PushParam(0),      // Is
                Instruction::PushVt,            // Vt
                Instruction::Div,               // Is/Vt
                Instruction::PushVoltage(0, 1), // V
                Instruction::PushVt,            // Vt
                Instruction::Div,               // V/Vt
                Instruction::Limexp,            // exp(V/Vt)
                Instruction::Mul,               // (Is/Vt) * exp(V/Vt)
            ],
        };

        CompiledModel {
            name: "diode".into(),
            num_terminals: 2,
            terminal_names: vec!["anode".into(), "cathode".into()],
            parameters: vec![CompiledParameter {
                name: "is".into(),
                default: 1e-14,
                min: Some(0.0),
                max: None,
            }],
            num_variables: 0,
            assignment_programs: vec![],
            laplace_filters: vec![],
            stamp_programs: vec![StampProgram {
                stamp_locations: vec![
                    StampLocation {
                        row: StampIndex::Terminal(0),
                        col: StampIndex::Ground,
                        sign: -1.0,
                    },
                    StampLocation {
                        row: StampIndex::Terminal(1),
                        col: StampIndex::Ground,
                        sign: 1.0,
                    },
                ],
                value_program,
                jacobian_programs: vec![
                    JacEntry {
                        row: StampIndex::Terminal(0),
                        col: StampIndex::Terminal(0),
                        program: jacobian.clone(),
                    },
                    JacEntry {
                        row: StampIndex::Terminal(0),
                        col: StampIndex::Terminal(1),
                        program: BytecodeProgram {
                            instructions: vec![
                                Instruction::PushParam(0),
                                Instruction::PushVt,
                                Instruction::Div,
                                Instruction::PushVoltage(0, 1),
                                Instruction::PushVt,
                                Instruction::Div,
                                Instruction::Limexp,
                                Instruction::Mul,
                                Instruction::Neg,
                            ],
                        },
                    },
                ],
            }],
            lookup_tables: vec![],
            internal_nodes: 0,
            branch_currents: 0,
        }
    }

    #[test]
    fn test_device_creation() {
        let model = create_simple_resistor_model();
        let device = VerilogADevice::new("R1", model, &[1, 0]);

        assert_eq!(device.name.as_str(), "R1");
        assert_eq!(device.num_terminals(), 2);
        assert_eq!(device.node_for_terminal(0), 1);
        assert_eq!(device.node_for_terminal(1), 0);
    }

    #[test]
    fn test_device_builder() {
        let model = create_simple_resistor_model();
        let device = DeviceBuilder::new(model, "R1")
            .nodes(&[1, 2])
            .param("g", 0.01)
            .temperature(350.0)
            .build();

        assert_eq!(device.name.as_str(), "R1");
        assert_eq!(device.context.temperature, 350.0);
    }

    #[test]
    fn test_set_parameter() {
        let model = create_simple_resistor_model();
        let mut device = VerilogADevice::new("R1", model, &[1, 0]);

        assert!(device.set_parameter("g", 0.01));
        assert!(!device.set_parameter("unknown", 1.0));
    }

    #[test]
    fn test_update_voltages() {
        let model = create_simple_resistor_model();
        let mut device = VerilogADevice::new("R1", model, &[1, 2]);

        let voltages = vec![5.0, 3.0];
        device.update_voltages(&voltages);

        assert!((device.context.voltages[0] - 5.0).abs() < 1e-10);
        assert!((device.context.voltages[1] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_resistor() {
        let model = create_simple_resistor_model();
        let mut device = VerilogADevice::new("R1", model, &[1, 2]);
        device.set_parameter("g", 0.001); // 1kΩ

        // Set V(p) = 5V, V(n) = 3V → V(p,n) = 2V
        let voltages = vec![5.0, 3.0];
        device.update_voltages(&voltages);

        let currents = device.evaluate();
        assert_eq!(currents.len(), 1);
        // I = G * V = 0.001 * 2 = 0.002A = 2mA
        assert!((currents[0] - 0.002).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_diode() {
        let model = create_diode_model();
        let mut device = VerilogADevice::new("D1", model, &[1, 0]);
        device.set_parameter("is", 1e-14);

        // Forward bias 0.7V
        let voltages = vec![0.7];
        device.update_voltages(&voltages);

        let currents = device.evaluate();
        assert_eq!(currents.len(), 1);
        // Should be in mA range for a forward-biased diode
        assert!(currents[0] > 1e-4);
        assert!(currents[0] < 1.0);
    }

    #[test]
    fn test_compute_jacobian() {
        let model = create_simple_resistor_model();
        let mut device = VerilogADevice::new("R1", model, &[1, 0]);
        device.set_parameter("g", 0.001);

        let voltages = vec![1.0];
        device.update_voltages(&voltages);

        let jacobian = device.compute_jacobian();
        assert_eq!(jacobian.len(), 2);

        // dI/dVp = G = 0.001
        assert!((jacobian[0].value - 0.001).abs() < 1e-10);
        // dI/dVn = -G = -0.001
        assert!((jacobian[1].value + 0.001).abs() < 1e-10);
    }

    #[test]
    fn test_stamp() {
        let model = create_simple_resistor_model();
        let mut device = VerilogADevice::new("R1", model, &[1, 2]);
        device.set_parameter("g", 0.001);

        let voltages = vec![5.0, 3.0];

        let mut matrix: Vec<(usize, usize, f64)> = Vec::new();
        let mut rhs: Vec<(usize, f64)> = Vec::new();

        device.stamp(
            &voltages,
            |row, col, val| matrix.push((row, col, val)),
            |node, val| rhs.push((node, val)),
        );

        // Should have stamped:
        // - RHS at node 0 (terminal p) and node 1 (terminal n)
        // - Matrix entries for Jacobian
        assert!(!matrix.is_empty());
        assert!(!rhs.is_empty());
    }

    #[test]
    fn test_terminal_names() {
        let model = create_simple_resistor_model();
        let device = VerilogADevice::new("R1", model, &[1, 0]);

        let names = device.terminal_names();
        assert_eq!(names.len(), 2);
        assert_eq!(names[0].as_str(), "p");
        assert_eq!(names[1].as_str(), "n");
    }

    #[test]
    fn test_stamp_index_to_node() {
        let model = create_simple_resistor_model();
        let device = VerilogADevice::new("R1", model, &[1, 2]);

        // Terminal 0 → circuit node 1 → index 0
        assert_eq!(
            device.stamp_index_to_node(&StampIndex::Terminal(0)),
            Some(0)
        );
        // Terminal 1 → circuit node 2 → index 1
        assert_eq!(
            device.stamp_index_to_node(&StampIndex::Terminal(1)),
            Some(1)
        );
        // Ground → None
        assert_eq!(device.stamp_index_to_node(&StampIndex::Ground), None);
    }

    #[test]
    fn test_ground_terminal() {
        let model = create_simple_resistor_model();
        let device = VerilogADevice::new("R1", model, &[1, 0]);

        // Terminal 1 connected to ground (node 0)
        assert_eq!(device.node_for_terminal(1), 0);
        // Ground terminal should return None from stamp_index_to_node
        // Terminal 1 maps to node 0 (ground), which returns None
        let node = device.node_for_terminal(1);
        let stamp_node = if node > 0 { Some(node - 1) } else { None };
        assert_eq!(stamp_node, None);
    }

    // ========================================================================
    // Additional Comprehensive Tests
    // ========================================================================

    #[test]
    fn test_parameter_clamping() {
        let model = create_simple_resistor_model();
        let mut device = VerilogADevice::new("R1", model, &[1, 0]);

        // Try to set negative conductance (should be clamped to min=0)
        device.set_parameter("g", -0.001);
        let currents = device.evaluate();
        // Should clamp to 0, so current should be 0
        assert!(currents[0].abs() < 1e-15);
    }

    #[test]
    fn test_temperature_effect_on_diode() {
        let model = create_diode_model();

        // At low temperature
        let mut device_cold = VerilogADevice::new("D1", model.clone(), &[1, 0]);
        device_cold.set_temperature(250.0); // ~-23°C
        device_cold.update_voltages(&[0.6]);
        let current_cold = device_cold.evaluate()[0];

        // At high temperature
        let mut device_hot = VerilogADevice::new("D2", model, &[1, 0]);
        device_hot.set_temperature(400.0); // ~127°C
        device_hot.update_voltages(&[0.6]);
        let current_hot = device_hot.evaluate()[0];

        // At higher temperature, Vt = kT/q increases
        // So exp(V/Vt) decreases, meaning LESS current at same voltage (with constant Is)
        // Note: Real diodes show Is increase with T which dominates, but our simple model
        // only varies Vt, so current_cold > current_hot
        assert!(current_cold > current_hot);
    }

    #[test]
    fn test_diode_jacobian_values() {
        let model = create_diode_model();
        let mut device = VerilogADevice::new("D1", model, &[1, 0]);
        device.set_parameter("is", 1e-14);

        // Forward bias 0.7V
        device.update_voltages(&[0.7]);

        let jacobian = device.compute_jacobian();

        // Should have 2 Jacobian entries
        assert_eq!(jacobian.len(), 2);

        // First entry should be positive (dI/dVanode > 0)
        assert!(jacobian[0].value > 0.0);

        // Second entry should be negative (dI/dVcathode < 0)
        assert!(jacobian[1].value < 0.0);

        // Magnitudes should be equal
        assert!((jacobian[0].value + jacobian[1].value).abs() < 1e-15);
    }

    #[test]
    fn test_stamp_values_match_evaluate() {
        let model = create_simple_resistor_model();
        let mut device = VerilogADevice::new("R1", model, &[1, 2]);
        device.set_parameter("g", 0.005); // 200Ω

        let voltages = vec![10.0, 4.0]; // 6V across

        // First get evaluated current
        device.update_voltages(&voltages);
        let currents = device.evaluate();
        let expected_current = currents[0]; // 0.005 * 6 = 0.03A

        // Now stamp and collect RHS values
        let mut rhs: Vec<(usize, f64)> = Vec::new();
        device.stamp(&voltages, |_, _, _| {}, |node, val| rhs.push((node, val)));

        // RHS stamps should be related to current
        assert!(!rhs.is_empty());
        // The sum of absolute RHS should relate to 2x current (into and out of device)
        let total_rhs: f64 = rhs.iter().map(|(_, v)| v.abs()).sum();
        assert!((total_rhs - 2.0 * expected_current.abs()).abs() < 1e-10);
    }

    #[test]
    fn test_internal_node_mapping() {
        let model = create_simple_resistor_model();
        let device = VerilogADevice::new("R1", model, &[1, 2]);

        // Internal node 0 should map to offset from terminal count
        let internal = device.stamp_index_to_node(&StampIndex::Internal(0));
        assert_eq!(internal, Some(2)); // 2 terminals, so internal 0 → 2
    }

    #[test]
    fn test_set_time() {
        let model = create_simple_resistor_model();
        let mut device = VerilogADevice::new("R1", model, &[1, 0]);

        device.set_time(1.5e-9);
        assert!((device.context.time - 1.5e-9).abs() < 1e-20);
    }

    #[test]
    fn test_device_clone() {
        let model = create_simple_resistor_model();
        let device1 = VerilogADevice::new("R1", model, &[1, 2]);
        let device2 = device1.clone();

        assert_eq!(device1.name, device2.name);
        assert_eq!(device1.num_terminals(), device2.num_terminals());
    }

    #[test]
    fn test_multiple_parameters() {
        // Create a model with multiple parameters
        let value_program = BytecodeProgram {
            instructions: vec![
                Instruction::PushParam(0), // p1
                Instruction::PushParam(1), // p2
                Instruction::Mul,
                Instruction::PushVoltage(0, 1),
                Instruction::Mul,
            ],
        };

        let model = CompiledModel {
            name: "test".into(),
            num_terminals: 2,
            terminal_names: vec!["a".into(), "b".into()],
            parameters: vec![
                CompiledParameter {
                    name: "p1".into(),
                    default: 1.0,
                    min: None,
                    max: None,
                },
                CompiledParameter {
                    name: "p2".into(),
                    default: 2.0,
                    min: None,
                    max: None,
                },
            ],
            num_variables: 0,
            assignment_programs: vec![],
            stamp_programs: vec![StampProgram {
                stamp_locations: vec![],
                value_program,
                jacobian_programs: vec![],
            }],
            lookup_tables: vec![],
            internal_nodes: 0,
            branch_currents: 0,
            laplace_filters: vec![],
        };

        let mut device = VerilogADevice::new("T1", model, &[1, 0]);
        device.set_parameter("p1", 3.0);
        device.set_parameter("p2", 4.0);
        device.update_voltages(&[5.0]);

        let result = device.evaluate();
        // 3 * 4 * 5 = 60
        assert!((result[0] - 60.0).abs() < 1e-10);
    }

    #[test]
    fn test_set_internal_node_indices() {
        let model = create_internal_node_model();
        let mut device = VerilogADevice::new("D1", model, &[1, 0]);

        // Set internal node index (circuit assigns node 5 to internal node)
        device.set_internal_node_indices(&[5]);

        assert_eq!(device.internal_node_index(0), Some(5));
        assert_eq!(device.internal_node_index(1), None); // Only 1 internal node
        assert_eq!(device.internal_node_index(999), None);
    }

    #[test]
    fn test_update_all_voltages() {
        let model = create_internal_node_model();
        let mut device = VerilogADevice::new("D1", model, &[1, 2]);

        // Set internal node at circuit node 3
        device.set_internal_node_indices(&[3]);

        // Circuit voltages: node1=5V, node2=3V, node3=4V (internal)
        let voltages = vec![5.0, 3.0, 4.0];
        device.update_all_voltages(&voltages);

        // Terminal voltages
        assert!((device.context.voltages[0] - 5.0).abs() < 1e-10);
        assert!((device.context.voltages[1] - 3.0).abs() < 1e-10);

        // Internal node voltage
        assert!((device.context.internal_voltages[0] - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_update_all_voltages_with_ground() {
        let model = create_internal_node_model();
        let mut device = VerilogADevice::new("D1", model, &[1, 0]);

        // Internal node at ground (circuit node 0)
        device.set_internal_node_indices(&[0]);

        let voltages = vec![5.0, 3.0];
        device.update_all_voltages(&voltages);

        // Internal node voltage should be 0 (ground)
        assert!(device.context.internal_voltages[0].abs() < 1e-10);
    }

    #[test]
    fn test_internal_node_affects_evaluate() {
        // End-to-end: verify internal node voltage affects device output
        // Model: I = V(terminals) - V(internal)
        let model = create_internal_node_model();
        let mut device = VerilogADevice::new("D1", model, &[1, 2]);

        // Set internal node at circuit node 3
        device.set_internal_node_indices(&[3]);

        // Circuit: V(1)=5V, V(2)=3V (terminal difference: 2V), V(3)=1V (internal)
        device.update_all_voltages(&[5.0, 3.0, 1.0]);

        let result = device.evaluate();
        // Expected: (V(1)-V(2)) - V(internal) = (5-3) - 1 = 1
        assert!(
            (result[0] - 1.0).abs() < 1e-10,
            "Internal node should affect result: got {}, expected 1.0",
            result[0]
        );

        // Change internal node voltage only
        device.update_all_voltages(&[5.0, 3.0, 0.5]);
        let result2 = device.evaluate();
        // Expected: (V(1)-V(2)) - V(internal) = 2 - 0.5 = 1.5
        assert!(
            (result2[0] - 1.5).abs() < 1e-10,
            "Changed internal node should affect result: got {}, expected 1.5",
            result2[0]
        );
    }

    #[test]
    fn test_mapped_rhs_rows_skips_ground() {
        let model = create_simple_resistor_model();
        let device = VerilogADevice::new("R1", model, &[1, 0]);
        let rows = device.mapped_rhs_rows();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].len(), 1);
        assert_eq!(rows[0][0].0, 0);
        assert!((rows[0][0].1 - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_mapped_jacobian_locations_with_ground_terminal() {
        let model = create_simple_resistor_model();
        let device = VerilogADevice::new("R1", model, &[1, 0]);
        let locs = device.mapped_jacobian_locations();

        assert_eq!(locs.len(), 1);
        assert_eq!(locs[0].len(), 2);
        assert_eq!(locs[0][0], (Some(0), Some(0)));
        assert_eq!(locs[0][1], (Some(0), None));
    }
}
