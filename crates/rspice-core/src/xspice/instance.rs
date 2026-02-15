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
            context.set_param(&param_spec.name, param_spec.default);
        }

        // Override with instance parameters
        for (name, value) in params {
            context.set_param(name, *value);
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
                    if let PortConnection::Analog(node) = &self.connections[i] {
                        if *node > 0 {
                            // Would need branch equation - for now, treat as current source
                            rhs_add(*node - 1, output_value);
                        }
                    }
                }
                PortType::Current => {
                    // Current source output
                    if let PortConnection::Analog(node) = &self.connections[i] {
                        if *node > 0 {
                            rhs_add(*node - 1, output_value);
                        }
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
            if let Some(&port_idx) = self.port_indices.get(&port_name) {
                if let Some(PortConnection::Digital(node)) = self.connections.get(port_idx) {
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
        let is_output = port.direction == super::PortDirection::Out
            || port.direction == super::PortDirection::InOut;
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
            let is_output = port.direction == super::PortDirection::Out
                || port.direction == super::PortDirection::InOut;
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

#[cfg(test)]
mod tests {
    use super::super::{CodeModel, ParamSpec, PortDirection, PortSpec, PortType};
    use super::*;
    use std::sync::Arc;

    #[derive(Debug)]
    struct TestGainModel;

    impl CodeModel for TestGainModel {
        fn name(&self) -> &str {
            "test_gain"
        }

        fn ports(&self) -> &[PortSpec] {
            use std::sync::OnceLock;
            static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
            PORTS.get_or_init(|| {
                vec![
                    PortSpec::input("in", PortType::Voltage),
                    PortSpec::output("out", PortType::Voltage),
                    PortSpec {
                        name: "dout".to_string(),
                        direction: PortDirection::Out,
                        default_type: PortType::Digital,
                        allowed_types: vec![PortType::Digital],
                        is_vector: false,
                        null_allowed: false,
                        description: String::new(),
                    },
                ]
            })
        }

        fn parameters(&self) -> &[ParamSpec] {
            &[]
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
            let v = ctx.input("in");
            ctx.set_output_with_partial("out", v, 1.0);
            Ok(())
        }
    }

    #[test]
    fn test_port_connection() {
        let conn = PortConnection::Analog(5);
        assert_eq!(conn.primary_node(), Some(5));
        assert!(!conn.is_null());

        let null = PortConnection::Null;
        assert!(null.is_null());
        assert_eq!(null.primary_node(), None);
    }

    #[test]
    fn test_set_output_branch_for_voltage_output_port() {
        let model: Arc<dyn CodeModel> = Arc::new(TestGainModel);
        let mut instance = XspiceInstance::new(
            "A1",
            model,
            vec![
                PortConnection::Analog(1),
                PortConnection::Analog(2),
                PortConnection::Digital(3),
            ],
            &[],
        )
        .expect("instance should build");

        instance
            .set_output_branch(1, 7)
            .expect("voltage output branch assignment should succeed");
        assert_eq!(instance.branch_ordinal_at(1), Some(7));
    }

    #[test]
    fn test_set_output_branch_rejects_non_voltage_or_non_output_ports() {
        let model: Arc<dyn CodeModel> = Arc::new(TestGainModel);
        let mut instance = XspiceInstance::new(
            "A1",
            model,
            vec![
                PortConnection::Analog(1),
                PortConnection::Analog(2),
                PortConnection::Digital(3),
            ],
            &[],
        )
        .expect("instance should build");

        let err_input = instance
            .set_output_branch(0, 3)
            .expect_err("input port should reject branch assignment");
        assert!(
            err_input.to_string().contains("not a voltage output"),
            "unexpected input-port error: {}",
            err_input
        );

        let err_digital = instance
            .set_output_branch(2, 4)
            .expect_err("digital output should reject branch assignment");
        assert!(
            err_digital.to_string().contains("not a voltage output"),
            "unexpected digital-port error: {}",
            err_digital
        );
    }
}
