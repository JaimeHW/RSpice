//! XSPICE Instance
//!
//! Represents an instantiated code model in a circuit.
//! Handles port connections, parameter binding, and matrix stamping.

use super::context::PendingDigitalEvent;
use super::{
    AnalysisType, CallType, CmContext, CmError, CmResult, CodeModel, DigitalValue, EventQueue,
    ParamSpec, ParamType, PortSpec, PortType,
};
use crate::Value;
use std::any::Any;
use std::collections::HashMap;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;

fn canonical_param_key(name: &str) -> String {
    name.to_ascii_lowercase()
}

fn validate_numeric_param(spec: &ParamSpec, value: Value) -> CmResult<()> {
    if !value.is_finite() {
        return Err(CmError::InvalidParameter {
            name: spec.name.clone(),
            message: format!("value must be finite, got {value}"),
        });
    }

    if matches!(
        spec.param_type,
        ParamType::Integer | ParamType::IntegerVector
    ) && (value.round() - value).abs() > 1.0e-12
    {
        return Err(CmError::InvalidParameter {
            name: spec.name.clone(),
            message: format!("expected integer value, got {value}"),
        });
    }

    if let Some(min) = spec.min
        && value < min
    {
        return Err(CmError::InvalidParameter {
            name: spec.name.clone(),
            message: format!("value {value} is below minimum {min}"),
        });
    }

    if let Some(max) = spec.max
        && value > max
    {
        return Err(CmError::InvalidParameter {
            name: spec.name.clone(),
            message: format!("value {value} is above maximum {max}"),
        });
    }

    Ok(())
}

fn invalid_vector_param_type(spec: &ParamSpec, actual: &str) -> CmError {
    CmError::InvalidParameter {
        name: spec.name.clone(),
        message: format!(
            "expected {:?} parameter, got {} parameter",
            spec.param_type, actual
        ),
    }
}

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
#[derive(Clone)]
pub struct XspiceInstance {
    /// Instance name (e.g., "A1")
    pub name: String,
    /// Reference to the code model
    model: Arc<dyn CodeModel>,
    /// Immutable port contract captured when the instance is constructed.
    ports: Vec<PortSpec>,
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
            .field("ports", &self.ports)
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
    /// * `params` - Numeric instance/model parameter overrides
    /// * `string_params` - String instance/model parameter overrides
    /// * `real_vector_params` - Real-vector instance/model parameter overrides
    /// * `integer_vector_params` - Integer-vector instance/model parameter overrides
    pub fn new(
        name: impl Into<String>,
        model: Arc<dyn CodeModel>,
        connections: Vec<PortConnection>,
        params: &[(String, Value)],
        string_params: &[(String, String)],
        real_vector_params: &[(String, Vec<Value>)],
        integer_vector_params: &[(String, Vec<i64>)],
    ) -> CmResult<Self> {
        let name = name.into();
        let ports = model.ports().to_vec();
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
                super::ParamType::RealVector => {
                    if let Some(default) = &param_spec.real_vector_default {
                        context.set_real_vector_param(&param_spec.name, default.clone());
                    }
                }
                super::ParamType::IntegerVector => {
                    if let Some(default) = &param_spec.integer_vector_default {
                        context.set_integer_vector_param(&param_spec.name, default.clone());
                    }
                }
                _ => context.set_param(&param_spec.name, param_spec.default),
            }
        }

        let param_specs: HashMap<String, &ParamSpec> = model
            .parameters()
            .iter()
            .map(|spec| (canonical_param_key(&spec.name), spec))
            .collect();

        // Override with instance parameters
        for (name, value) in params {
            if let Some(spec) = param_specs.get(&canonical_param_key(name)) {
                if matches!(
                    spec.param_type,
                    ParamType::RealVector | ParamType::IntegerVector
                ) {
                    return Err(CmError::InvalidParameter {
                        name: spec.name.clone(),
                        message: "expected vector parameter, got scalar parameter".to_string(),
                    });
                }
                validate_numeric_param(spec, *value)?;
            }
            context.set_param(name, *value);
        }
        for (name, value) in string_params {
            if let Some(spec) = param_specs.get(&canonical_param_key(name))
                && matches!(
                    spec.param_type,
                    ParamType::RealVector | ParamType::IntegerVector
                )
            {
                return Err(CmError::InvalidParameter {
                    name: spec.name.clone(),
                    message: "expected vector parameter, got string parameter".to_string(),
                });
            }
            context.set_string_param(name, value);
        }
        for (name, values) in real_vector_params {
            if let Some(spec) = param_specs.get(&canonical_param_key(name)) {
                match spec.param_type {
                    ParamType::RealVector => {
                        for value in values {
                            validate_numeric_param(spec, *value)?;
                        }
                    }
                    ParamType::IntegerVector => {
                        let mut integer_values = Vec::with_capacity(values.len());
                        for value in values {
                            validate_numeric_param(spec, *value)?;
                            integer_values.push(value.round() as i64);
                        }
                        context.set_integer_vector_param(name, integer_values);
                        continue;
                    }
                    _ => return Err(invalid_vector_param_type(spec, "real-vector")),
                }
            }
            context.set_real_vector_param(name, values.clone());
        }
        for (name, values) in integer_vector_params {
            if let Some(spec) = param_specs.get(&canonical_param_key(name)) {
                match spec.param_type {
                    ParamType::IntegerVector => {
                        for value in values {
                            validate_numeric_param(spec, *value as Value)?;
                        }
                    }
                    ParamType::RealVector => {
                        let real_values: Vec<Value> =
                            values.iter().map(|value| *value as Value).collect();
                        for value in &real_values {
                            validate_numeric_param(spec, *value)?;
                        }
                        context.set_real_vector_param(name, real_values);
                        continue;
                    }
                    _ => return Err(invalid_vector_param_type(spec, "integer-vector")),
                }
            }
            context.set_integer_vector_param(name, values.clone());
        }

        for (port, connection) in ports.iter().zip(connections.iter()) {
            let width = match connection {
                PortConnection::AnalogVector(nodes) | PortConnection::DigitalVector(nodes) => {
                    nodes.len()
                }
                PortConnection::Null => 0,
                _ => 1,
            };
            context.set_port_width(&port.name, width);
        }

        // Initialize output ports in context
        for port in &ports {
            if port.direction == super::PortDirection::Out
                || port.direction == super::PortDirection::InOut
            {
                context.init_output(&port.name, port.default_type);
            }
        }

        Ok(Self {
            name,
            model,
            ports,
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
        &self.ports
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

    /// Get real-vector parameter value
    pub fn real_vector_param(&self, name: &str) -> Option<&[Value]> {
        self.context.real_vector_param(name)
    }

    /// Get integer-vector parameter value
    pub fn integer_vector_param(&self, name: &str) -> Option<&[i64]> {
        self.context.integer_vector_param(name)
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
        let context_before_init = self.context.clone();
        match catch_unwind(AssertUnwindSafe(|| self.model.init(&mut self.context))) {
            Ok(result) => result?,
            Err(payload) => {
                self.context = context_before_init;
                return Err(self.model_panic_error("initialization", payload));
            }
        }
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
        digital_event_times: &HashMap<usize, Value>,
    ) {
        let ports = &self.ports;

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
                    if let Some(time) = digital_event_times.get(node).copied() {
                        self.context.set_input_digital_event_time(&port.name, time);
                    }
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
        for (port, connection) in self.ports.iter().zip(self.connections.iter()) {
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

        let context_before_evaluate = self.context.clone();
        match catch_unwind(AssertUnwindSafe(|| self.model.evaluate(&mut self.context))) {
            Ok(result) => result,
            Err(payload) => {
                self.context = context_before_evaluate;
                Err(self.model_panic_error("evaluation", payload))
            }
        }
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
        let ports = &self.ports;

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
    pub(crate) fn take_pending_events(&mut self) -> Vec<PendingDigitalEvent> {
        self.context.take_pending_events()
    }

    /// Process digital events scheduled by this instance
    pub fn schedule_events(&mut self, event_queue: &mut EventQueue, current_time: Value) {
        let events = self.context.take_pending_events();

        for PendingDigitalEvent {
            port_name,
            values,
            delay,
        } in events
        {
            let Some(&port_idx) = self.port_indices.get(&port_name) else {
                continue;
            };
            let Some(connection) = self.connections.get(port_idx) else {
                continue;
            };

            match connection {
                PortConnection::Digital(node) => {
                    if let Some(value) = values.first().copied() {
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
                PortConnection::DigitalVector(nodes) => {
                    for (node, value) in nodes.iter().zip(values.into_iter()) {
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
                _ => {}
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
        if let Some(port) = self.ports.get(port_idx) {
            self.context.set_input_analog(&port.name, value);
        }
    }

    /// Assign an MNA branch ordinal to a voltage-type output port.
    pub fn set_output_branch(&mut self, port_idx: usize, branch_ordinal: usize) -> CmResult<()> {
        let ports = &self.ports;
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
        let ports = &self.ports;
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

        self.ports
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

    fn model_panic_error(&self, phase: &str, payload: Box<dyn Any + Send + 'static>) -> CmError {
        CmError::EvaluationError(format!(
            "XSPICE code model '{}' on instance '{}' panicked during {}: {}",
            self.model.name(),
            self.name,
            phase,
            panic_payload_message(payload.as_ref())
        ))
    }
}

fn panic_payload_message(payload: &(dyn Any + Send + 'static)) -> String {
    if let Some(message) = payload.downcast_ref::<&'static str>() {
        (*message).to_string()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "unknown panic payload".to_string()
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::{DigitalState, DigitalStrength, ParamSpec, PortDirection};
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::atomic::{AtomicBool, Ordering};

    struct PanicModel {
        panic_in_init: bool,
        panic_in_evaluate: bool,
        panic_via_context_accessor: bool,
        ports: Vec<PortSpec>,
        params: Vec<ParamSpec>,
    }

    struct MutablePortsModel {
        expanded: Arc<AtomicBool>,
        initial_ports: Vec<PortSpec>,
        expanded_ports: Vec<PortSpec>,
        params: Vec<ParamSpec>,
    }

    impl MutablePortsModel {
        fn new(expanded: Arc<AtomicBool>) -> Self {
            Self {
                expanded,
                initial_ports: vec![PortSpec {
                    name: "out".to_string(),
                    direction: PortDirection::Out,
                    default_type: PortType::Voltage,
                    allowed_types: vec![PortType::Voltage],
                    is_vector: false,
                    null_allowed: false,
                    description: String::new(),
                }],
                expanded_ports: vec![
                    PortSpec {
                        name: "out".to_string(),
                        direction: PortDirection::Out,
                        default_type: PortType::Voltage,
                        allowed_types: vec![PortType::Voltage],
                        is_vector: false,
                        null_allowed: false,
                        description: String::new(),
                    },
                    PortSpec {
                        name: "late_input".to_string(),
                        direction: PortDirection::In,
                        default_type: PortType::Voltage,
                        allowed_types: vec![PortType::Voltage],
                        is_vector: false,
                        null_allowed: false,
                        description: String::new(),
                    },
                ],
                params: Vec::new(),
            }
        }
    }

    impl CodeModel for MutablePortsModel {
        fn name(&self) -> &str {
            "mutable_ports_model"
        }

        fn ports(&self) -> &[PortSpec] {
            if self.expanded.load(Ordering::SeqCst) {
                &self.expanded_ports
            } else {
                &self.initial_ports
            }
        }

        fn parameters(&self) -> &[ParamSpec] {
            &self.params
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
            ctx.set_output("out", 1.0);
            Ok(())
        }
    }

    impl PanicModel {
        fn new() -> Self {
            Self {
                panic_in_init: false,
                panic_in_evaluate: false,
                panic_via_context_accessor: false,
                ports: vec![PortSpec {
                    name: "out".to_string(),
                    direction: PortDirection::Out,
                    default_type: PortType::Voltage,
                    allowed_types: vec![PortType::Voltage],
                    is_vector: false,
                    null_allowed: false,
                    description: String::new(),
                }],
                params: Vec::new(),
            }
        }
    }

    impl CodeModel for PanicModel {
        fn name(&self) -> &str {
            "panic_model"
        }

        fn ports(&self) -> &[PortSpec] {
            &self.ports
        }

        fn parameters(&self) -> &[ParamSpec] {
            &self.params
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            if self.panic_in_init {
                panic!("init exploded");
            }
            Ok(())
        }

        fn evaluate(&self, _ctx: &mut CmContext) -> CmResult<()> {
            if self.panic_in_evaluate {
                panic!("evaluate exploded");
            }
            if self.panic_via_context_accessor {
                let digital = super::super::context::InputValue::Digital(DigitalValue::new(
                    DigitalState::One,
                    DigitalStrength::Strong,
                ));
                let _ = digital.analog();
            }
            Ok(())
        }
    }

    fn instance_with(model: PanicModel) -> XspiceInstance {
        XspiceInstance::new(
            "Apanic",
            Arc::new(model),
            vec![PortConnection::Analog(1)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("panic-model instance should construct")
    }

    fn model_with_params(params: Vec<ParamSpec>) -> PanicModel {
        let mut model = PanicModel::new();
        model.params = params;
        model
    }

    #[test]
    fn instance_rejects_out_of_range_known_numeric_parameter() {
        let model = model_with_params(vec![ParamSpec::integer("ic", 2).with_range(0.0, 2.0)]);
        let err = XspiceInstance::new(
            "Aparam",
            Arc::new(model),
            vec![PortConnection::Analog(1)],
            &[("ic".to_string(), 3.0)],
            &[],
            &[],
            &[],
        )
        .expect_err("known integer parameter outside its range must be rejected");

        assert!(matches!(
            err,
            CmError::InvalidParameter { ref name, .. } if name == "ic"
        ));
    }

    #[test]
    fn instance_rejects_fractional_known_integer_parameter() {
        let model = model_with_params(vec![ParamSpec::integer("select_value", 1)]);
        let err = XspiceInstance::new(
            "Aparam",
            Arc::new(model),
            vec![PortConnection::Analog(1)],
            &[("select_value".to_string(), 1.5)],
            &[],
            &[],
            &[],
        )
        .expect_err("known integer parameter must not accept fractional values");

        assert!(matches!(
            err,
            CmError::InvalidParameter { ref name, .. } if name == "select_value"
        ));
    }

    #[test]
    fn instance_preserves_unknown_numeric_parameters_for_compatibility() {
        let model = PanicModel::new();
        let instance = XspiceInstance::new(
            "Aparam",
            Arc::new(model),
            vec![PortConnection::Analog(1)],
            &[("vendor_extra".to_string(), -1.25)],
            &[],
            &[],
            &[],
        )
        .expect("unknown compatibility parameters should still be stored");

        assert_eq!(instance.param("vendor_extra"), -1.25);
    }

    #[test]
    fn instance_uses_real_vector_parameter_default() {
        let model = model_with_params(vec![ParamSpec::real_vector("points", vec![1.0, 2.5])]);
        let instance = XspiceInstance::new(
            "Avec",
            Arc::new(model),
            vec![PortConnection::Analog(1)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("real-vector default should construct");

        assert_eq!(instance.real_vector_param("POINTS").unwrap(), &[1.0, 2.5]);
    }

    #[test]
    fn instance_applies_real_vector_parameter_override() {
        let model = model_with_params(vec![ParamSpec::real_vector("points", vec![1.0, 2.0])]);
        let instance = XspiceInstance::new(
            "Avec",
            Arc::new(model),
            vec![PortConnection::Analog(1)],
            &[],
            &[],
            &[("points".to_string(), vec![3.0, 4.0, 5.0])],
            &[],
        )
        .expect("real-vector override should construct");

        assert_eq!(
            instance.real_vector_param("points").unwrap(),
            &[3.0, 4.0, 5.0]
        );
    }

    #[test]
    fn instance_applies_integer_vector_parameter_override() {
        let model = model_with_params(vec![ParamSpec::integer_vector("bits", vec![1, 0])]);
        let instance = XspiceInstance::new(
            "Avec",
            Arc::new(model),
            vec![PortConnection::Analog(1)],
            &[],
            &[],
            &[],
            &[("bits".to_string(), vec![1, 1, 0, 1])],
        )
        .expect("integer-vector override should construct");

        assert_eq!(
            instance.integer_vector_param("bits").unwrap(),
            &[1, 1, 0, 1]
        );
    }

    #[test]
    fn instance_rejects_scalar_override_for_known_vector_parameter() {
        let model = model_with_params(vec![ParamSpec::real_vector("points", vec![1.0, 2.0])]);
        let err = XspiceInstance::new(
            "Avec",
            Arc::new(model),
            vec![PortConnection::Analog(1)],
            &[("points".to_string(), 3.0)],
            &[],
            &[],
            &[],
        )
        .expect_err("known vector parameter must not accept scalar override");

        assert!(matches!(
            err,
            CmError::InvalidParameter { ref name, .. } if name.eq_ignore_ascii_case("points")
        ));
    }

    #[test]
    fn instance_rejects_string_override_for_known_vector_parameter() {
        let model = model_with_params(vec![ParamSpec::real_vector("points", vec![1.0, 2.0])]);
        let err = XspiceInstance::new(
            "Avec",
            Arc::new(model),
            vec![PortConnection::Analog(1)],
            &[],
            &[("points".to_string(), "bad".to_string())],
            &[],
            &[],
        )
        .expect_err("known vector parameter must not accept string override");

        assert!(matches!(
            err,
            CmError::InvalidParameter { ref name, .. } if name.eq_ignore_ascii_case("points")
        ));
    }

    fn assert_evaluation_error(result: CmResult<()>, expected: &str) {
        match result {
            Err(CmError::EvaluationError(message)) => {
                assert!(
                    message.contains(expected),
                    "expected message containing `{expected}`, got `{message}`"
                );
            }
            other => panic!("expected evaluation error, got {other:?}"),
        }
    }

    #[test]
    fn evaluate_converts_code_model_panic_to_error() {
        let mut model = PanicModel::new();
        model.panic_in_evaluate = true;
        let mut instance = instance_with(model);

        let result = catch_unwind(AssertUnwindSafe(|| {
            instance.evaluate(0.0, 1e-9, AnalysisType::Transient)
        }));

        let result = result.expect("XSPICE evaluate panic must not unwind past instance");
        assert_evaluation_error(result, "evaluate exploded");
    }

    #[test]
    fn init_converts_code_model_panic_to_error() {
        let mut model = PanicModel::new();
        model.panic_in_init = true;
        let mut instance = instance_with(model);

        let result = catch_unwind(AssertUnwindSafe(|| instance.init()));

        let result = result.expect("XSPICE init panic must not unwind past instance");
        assert_evaluation_error(result, "init exploded");
    }

    #[test]
    fn evaluate_converts_context_accessor_panic_to_error() {
        let mut model = PanicModel::new();
        model.panic_via_context_accessor = true;
        let mut instance = instance_with(model);

        let result = catch_unwind(AssertUnwindSafe(|| {
            instance.evaluate(0.0, 1e-9, AnalysisType::Transient)
        }));

        let result = result.expect("XSPICE context helper panic must not unwind past instance");
        assert_evaluation_error(result, "Expected analog value");
    }

    #[test]
    fn instance_uses_construction_time_port_contract() {
        let expanded = Arc::new(AtomicBool::new(false));
        let model = MutablePortsModel::new(Arc::clone(&expanded));
        let mut instance = XspiceInstance::new(
            "Amutable",
            Arc::new(model),
            vec![PortConnection::Analog(1)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("mutable-ports instance should construct from initial port contract");

        assert_eq!(instance.ports().len(), 1);

        expanded.store(true, Ordering::SeqCst);

        assert_eq!(
            instance.ports().len(),
            1,
            "the instance must not observe model port mutations after construction"
        );
        let result = catch_unwind(AssertUnwindSafe(|| {
            instance.update_inputs(&[0.0], &HashMap::new(), &HashMap::new());
            instance
                .evaluate(0.0, 1e-9, AnalysisType::Transient)
                .expect("evaluation should use the stable port contract");
            instance.stamp(|_, _, _| {}, |_, _| {});
        }));
        result.expect("stable port contract must avoid connection/index panics");
    }
}
