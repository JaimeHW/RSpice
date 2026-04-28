//! XSPICE Instance
//!
//! Represents an instantiated code model in a circuit.
//! Handles port connections, parameter binding, and matrix stamping.

use super::{
    AnalysisType, CallType, CmContext, CmError, CmResult, CodeModel, DigitalValue, EventQueue,
    PortSpec, PortType,
};
use crate::Value;
use std::collections::HashMap;
use std::sync::Arc;

//=============================================================================
// Port Connection
//=============================================================================

/// Connection for a single port
#[derive(Debug, Clone)]
pub enum PortConnection {
    /// Analog node connection (circuit node index, 0 = ground)
    Analog(usize),
    /// Differential analog connection (positive node, negative node)
    Differential(usize, usize),
    /// Digital node connection
    Digital(usize),
    /// Vector of analog nodes
    AnalogVector(Vec<usize>),
    /// Vector of digital nodes
    DigitalVector(Vec<usize>),
    /// Null connection (unconnected)
    Null,
}

impl PortConnection {
    /// Get the primary node (for single connections)
    pub fn primary_node(&self) -> Option<usize> {
        match self {
            PortConnection::Analog(n) => Some(*n),
            PortConnection::Differential(p, _) => Some(*p),
            PortConnection::Digital(n) => Some(*n),
            PortConnection::AnalogVector(v) => v.first().copied(),
            PortConnection::DigitalVector(v) => v.first().copied(),
            PortConnection::Null => None,
        }
    }

    /// Check if this is a null connection
    pub fn is_null(&self) -> bool {
        matches!(self, PortConnection::Null)
    }
}

//=============================================================================
// XSPICE Instance
//=============================================================================

/// An instantiated XSPICE code model in a circuit
pub struct XspiceInstance {
    /// Instance name (e.g., "A1")
    pub name: String,
    /// Reference to the code model
    model: Arc<dyn CodeModel>,
    /// Port connections (indexed by port spec order)
    connections: Vec<PortConnection>,
    /// Port name to index mapping
    port_indices: HashMap<String, usize>,
    /// Execution context
    context: CmContext,
    /// Optional MNA branch variable per port (for voltage-type outputs)
    output_branches: Vec<Option<usize>>,
    /// Has been initialized
    initialized: bool,
}

impl std::fmt::Debug for XspiceInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XspiceInstance")
            .field("name", &self.name)
            .field("model", &self.model.name())
            .field("connections", &self.connections)
            .field("output_branches", &self.output_branches)
            .field("initialized", &self.initialized)
            .finish()
    }
}

impl XspiceInstance {
    /// Create a new XSPICE instance
    ///
    /// # Arguments
    /// * `name` - Instance name
    /// * `model` - Code model reference
    /// * `connections` - Port connections (must match model port count)
    /// * `params` - Instance parameter overrides
    pub fn new(
        name: impl Into<String>,
        model: Arc<dyn CodeModel>,
        connections: Vec<PortConnection>,
        params: &[(String, Value)],
        string_params: &[(String, String)],
    ) -> CmResult<Self> {
        let name = name.into();
        let ports = model.ports();
        let port_count = ports.len();

        // Validate connection count
        if connections.len() != ports.len() {
            return Err(CmError::PortCountMismatch {
                expected: ports.len(),
                actual: connections.len(),
            });
        }

        // Build port index map
        let mut port_indices = HashMap::new();
        for (i, port) in ports.iter().enumerate() {
            port_indices.insert(port.name.clone(), i);
        }

        // Create context and initialize parameters
        let mut context = CmContext::new();

        // Set default parameter values
        for param_spec in model.parameters() {
            match param_spec.param_type {
                super::ParamType::String => {
                    if let Some(default) = &param_spec.string_default {
                        context.set_string_param(&param_spec.name, default);
                    }
                }
                _ => context.set_param(&param_spec.name, param_spec.default),
            }
        }

        // Override with instance parameters
        for (name, value) in params {
            context.set_param(name, *value);
        }
        for (name, value) in string_params {
            context.set_string_param(name, value);
        }

        // Initialize output ports in context
        for port in ports {
            if port.direction == super::PortDirection::Out
                || port.direction == super::PortDirection::InOut
            {
                context.init_output(&port.name, port.default_type);
            }
        }

        Ok(Self {
            name,
            model,
            connections,
            port_indices,
            context,
            output_branches: vec![None; port_count],
            initialized: false,
        })
    }

    /// Get the model name
    pub fn model_name(&self) -> &str {
        self.model.name()
    }

    /// Get port specifications
    pub fn ports(&self) -> &[PortSpec] {
        self.model.ports()
    }

    /// Get connection for a port by name
    pub fn connection(&self, port_name: &str) -> Option<&PortConnection> {
        self.port_indices
            .get(port_name)
            .and_then(|&i| self.connections.get(i))
    }

    /// Get connection for a port by index
    pub fn connection_at(&self, index: usize) -> Option<&PortConnection> {
        self.connections.get(index)
    }

    /// Get parameter value
    pub fn param(&self, name: &str) -> Value {
        self.context.param(name)
    }

    /// Get string parameter value
    pub fn string_param(&self, name: &str) -> Option<&str> {
        self.context.string_param(name)
    }

    /// Set parameter value
    pub fn set_param(&mut self, name: &str, value: Value) {
        self.context.set_param(name, value);
    }

    /// Initialize the instance
    ///
    /// Called once at simulation start.
    pub fn init(&mut self) -> CmResult<()> {
        if self.initialized {
            return Ok(());
        }

        self.context.call_type = CallType::Init;
        self.model.init(&mut self.context)?;
        self.initialized = true;
        Ok(())
    }

    /// Update input values from circuit solution
    ///
    /// # Arguments
    /// * `voltages` - Circuit node voltages (index 0 = node 1)
    /// * `digital_values` - Digital node values
    pub fn update_inputs(
        &mut self,
        voltages: &[Value],
        digital_values: &HashMap<usize, DigitalValue>,
    ) {
        let ports = self.model.ports();

        for (i, port) in ports.iter().enumerate() {
            if port.direction != super::PortDirection::In
                && port.direction != super::PortDirection::InOut
            {
                continue;
            }

            match &self.connections[i] {
                PortConnection::Analog(node) => {
                    let v = if *node == 0 {
                        0.0
                    } else {
                        voltages.get(node - 1).copied().unwrap_or(0.0)
                    };
                    self.context.set_input_analog(&port.name, v);
                }
                PortConnection::Differential(pos, neg) => {
                    let v_pos = if *pos == 0 {
                        0.0
                    } else {
                        voltages.get(pos - 1).copied().unwrap_or(0.0)
                    };
                    let v_neg = if *neg == 0 {
                        0.0
                    } else {
                        voltages.get(neg - 1).copied().unwrap_or(0.0)
                    };
                    self.context.set_input_analog(&port.name, v_pos - v_neg);
                }
                PortConnection::Digital(node) => {
                    let val = digital_values.get(node).copied().unwrap_or_default();
                    self.context.set_input_digital(&port.name, val);
                }
                PortConnection::AnalogVector(nodes) => {
                    let values: Vec<Value> = nodes
                        .iter()
                        .map(|n| {
                            if *n == 0 {
                                0.0
                            } else {
                                voltages.get(n - 1).copied().unwrap_or(0.0)
                            }
                        })
                        .collect();
                    // Store as analog vector input
                    use super::context::{AnalogValue, InputValue};
                    self.context.set_input(
                        &port.name,
                        InputValue::AnalogVector(
                            values.into_iter().map(AnalogValue::new).collect(),
                        ),
                    );
                }
                PortConnection::DigitalVector(nodes) => {
                    let values: Vec<DigitalValue> = nodes
                        .iter()
                        .map(|n| digital_values.get(n).copied().unwrap_or_default())
                        .collect();
                    use super::context::InputValue;
                    self.context
                        .set_input(&port.name, InputValue::DigitalVector(values));
                }
                PortConnection::Null => {}
            }
        }
    }

    /// Evaluate the code model
    ///
    /// # Arguments
    /// * `time` - Current simulation time
    /// * `timestep` - Current timestep
    /// * `analysis` - Type of analysis
    pub fn evaluate(
        &mut self,
        time: Value,
        timestep: Value,
        analysis: AnalysisType,
    ) -> CmResult<()> {
        self.context.clear_stamps();
        self.context.clear_port_nodes();
        for (port, connection) in self.model.ports().iter().zip(self.connections.iter()) {
            if let PortConnection::Analog(node) = connection {
                self.context.set_port_node(&port.name, *node);
            }
        }

        self.context.time = time;
        self.context.timestep = timestep;
        self.context.analysis = analysis;
        self.context.call_type = match analysis {
            AnalysisType::DcOp | AnalysisType::DcSweep => CallType::DcAnalysis,
            AnalysisType::Ac => CallType::AcAnalysis,
            AnalysisType::Transient => CallType::TransientAnalysis,
            _ => CallType::DcAnalysis,
        };

        self.model.evaluate(&mut self.context)
    }

    /// Get output value for stamping
    pub fn output(&self, port_name: &str) -> Value {
        self.context.output(port_name)
    }

    /// Stamp the instance into the circuit matrix
    ///
    /// # Arguments
    /// * `matrix_add` - Callback to add conductance at (row, col)
    /// * `rhs_add` - Callback to add current/voltage to RHS at node
    pub fn stamp<M, R>(&mut self, mut matrix_add: M, mut rhs_add: R)
    where
        M: FnMut(usize, usize, Value),
        R: FnMut(usize, Value),
    {
        let ports = self.model.ports();

        for (i, port) in ports.iter().enumerate() {
            if port.direction != super::PortDirection::Out
                && port.direction != super::PortDirection::InOut
            {
                continue;
            }

            // Get output value and partial derivative
            let output_value = self.context.output(&port.name);

            match port.default_type {
                PortType::Voltage => {
                    // Voltage source output - stamps like a dependent voltage source
                    if let PortConnection::Analog(node) = &self.connections[i]
                        && *node > 0
                    {
                        // Would need branch equation - for now, treat as current source
                        rhs_add(*node - 1, output_value);
                    }
                }
                PortType::Current => {
                    // Current source output
                    if let PortConnection::Analog(node) = &self.connections[i]
                        && *node > 0
                    {
                        rhs_add(*node - 1, output_value);
                    }
                }
                _ => {}
            }
        }

        // Process any queued stamps from the model
        for (row, col, value) in self.context.take_stamps() {
            matrix_add(row, col, value);
        }
        for (node, value) in self.context.take_rhs() {
            rhs_add(node, value);
        }
    }

    /// Get pending digital events
    pub fn take_pending_events(&mut self) -> Vec<(String, DigitalValue, Value)> {
        self.context.take_pending_events()
    }

    /// Process digital events scheduled by this instance
    pub fn schedule_events(&mut self, event_queue: &mut EventQueue, current_time: Value) {
        let _ports = self.model.ports(); // Reserved for future port validation
        let events = self.context.take_pending_events();

        for (port_name, value, delay) in events {
            // Find the node for this port
            if let Some(&port_idx) = self.port_indices.get(&port_name)
                && let Some(PortConnection::Digital(node)) = self.connections.get(port_idx)
            {
                event_queue.schedule_delayed(
                    current_time,
                    delay,
                    *node,
                    &port_name,
                    &self.name,
                    value,
                );
            }
        }
    }

    /// Advance state for new timestep
    pub fn advance_state(&mut self) {
        self.context.advance_state();
    }

    /// Set simulation temperature
    pub fn set_temperature(&mut self, temp_k: Value) {
        self.context.temperature = temp_k;
    }

    //=========================================================================
    // Circuit Integration Methods
    //=========================================================================

    /// Get port connections
    #[inline]
    pub fn connections(&self) -> &[PortConnection] {
        &self.connections
    }

    /// Remap circuit node IDs after a topology-level reference-node rewrite.
    pub fn remap_circuit_nodes(&mut self, mut remap: impl FnMut(usize) -> usize) {
        for connection in &mut self.connections {
            match connection {
                PortConnection::Analog(node) | PortConnection::Digital(node) => {
                    *node = remap(*node);
                }
                PortConnection::Differential(pos, neg) => {
                    *pos = remap(*pos);
                    *neg = remap(*neg);
                }
                PortConnection::AnalogVector(nodes) | PortConnection::DigitalVector(nodes) => {
                    for node in nodes {
                        *node = remap(*node);
                    }
                }
                PortConnection::Null => {}
            }
        }
    }

    /// Set analog input value for a port
    ///
    /// Called by CircuitData::evaluate_xspice to provide node voltages.
    pub fn set_input_analog(&mut self, port_idx: usize, value: Value) {
        if let Some(port) = self.model.ports().get(port_idx) {
            self.context.set_input_analog(&port.name, value);
        }
    }

    /// Assign an MNA branch ordinal to a voltage-type output port.
    pub fn set_output_branch(&mut self, port_idx: usize, branch_ordinal: usize) -> CmResult<()> {
        let ports = self.model.ports();
        let Some(port) = ports.get(port_idx) else {
            return Err(CmError::Internal(format!(
                "Invalid port index {} for instance {}",
                port_idx, self.name
            )));
        };
        let is_output = port.direction == super::PortDirection::Out;
        let is_voltage_port = matches!(
            port.default_type,
            PortType::Voltage | PortType::DifferentialVoltage
        );
        if !is_output || !is_voltage_port {
            return Err(CmError::Internal(format!(
                "Port '{}' on instance {} is not a voltage output",
                port.name, self.name
            )));
        }
        if port_idx >= self.output_branches.len() {
            return Err(CmError::Internal(format!(
                "Branch storage out of bounds for port {} on instance {}",
                port_idx, self.name
            )));
        }
        self.output_branches[port_idx] = Some(branch_ordinal);
        Ok(())
    }

    /// Get assigned branch ordinal for a port, if any.
    #[inline]
    pub fn branch_ordinal_at(&self, port_idx: usize) -> Option<usize> {
        self.output_branches
            .get(port_idx)
            .and_then(|entry| entry.as_ref().copied())
    }

    /// Get analog contribution (conductance, current) for stamping
    ///
    /// Returns Some((conductance, current)) for output ports that produce
    /// analog contributions, None for inputs or digital ports.
    pub fn get_analog_contribution(&self, port_idx: usize) -> Option<(Value, Value)> {
        let ports = self.model.ports();
        if let Some(port) = ports.get(port_idx) {
            let is_output = port.direction == super::PortDirection::Out;
            if is_output && port.default_type.is_analog() {
                // Get output value and partial derivative
                let output = self.context.output(&port.name);
                let partial = self.context.partial(&port.name);

                // Return linearized contribution terms consumed by circuit stamping.
                Some((partial, output))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Drain deferred matrix stamps queued by the code model.
    pub fn take_deferred_stamps(&mut self) -> Vec<(usize, usize, Value)> {
        self.context.take_stamps()
    }

    /// Drain deferred RHS contributions queued by the code model.
    pub fn take_deferred_rhs(&mut self) -> Vec<(usize, Value)> {
        self.context.take_rhs()
    }

    /// Accept the current timestep
    ///
    /// Called after successful convergence to commit state changes.
    pub fn accept_timestep(&mut self) {
        self.context.advance_state();
    }

    /// Check if the instance has converged
    ///
    /// Compares current output to previous iteration.
    pub fn is_converged(&self, tolerance: Value) -> bool {
        let tol = if tolerance.is_finite() && tolerance > 0.0 {
            tolerance
        } else {
            1e-12
        };

        self.model
            .ports()
            .iter()
            .filter(|port| {
                port.direction == super::PortDirection::Out
                    || port.direction == super::PortDirection::InOut
            })
            .all(|port| {
                let curr = self.context.output(&port.name);
                let prev = self.context.output_prev(&port.name);
                (curr - prev).abs() <= tol + tol * curr.abs().max(prev.abs())
            })
    }
}

//=============================================================================
// Tests
//=============================================================================

