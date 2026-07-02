//! XSPICE Instance
//!
//! Represents an instantiated code model in a circuit.
//! Handles port connections, parameter binding, and matrix stamping.

use super::context::{AnalogValue, PendingDigitalEvent, PendingRealEvent};
use super::{
    AnalysisType, CallType, CmContext, CmError, CmResult, CodeModel, DigitalValue, EvaluationPhase,
    EventQueue, ParamSpec, ParamType, PortSpec, PortType,
};
use crate::{Complex64, Value};
use std::any::Any;
use std::collections::{HashMap, HashSet};
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

    let checked_value = if matches!(
        spec.param_type,
        ParamType::Integer | ParamType::IntegerVector
    ) {
        value.round()
    } else {
        value
    };

    if let Some(min) = spec.min
        && checked_value < min
    {
        return Err(CmError::InvalidParameter {
            name: spec.name.clone(),
            message: format!("value {value} is below minimum {min}"),
        });
    }

    if let Some(max) = spec.max
        && checked_value > max
    {
        return Err(CmError::InvalidParameter {
            name: spec.name.clone(),
            message: format!("value {value} is above maximum {max}"),
        });
    }

    Ok(())
}

fn validate_vector_param_len(spec: &ParamSpec, len: usize) -> CmResult<()> {
    if let Some(min_len) = spec.vector_min_len
        && len < min_len
    {
        return Err(CmError::InvalidParameter {
            name: spec.name.clone(),
            message: format!("expected at least {min_len} value(s), got {len}"),
        });
    }

    if let Some(max_len) = spec.vector_max_len
        && len > max_len
    {
        return Err(CmError::InvalidParameter {
            name: spec.name.clone(),
            message: format!("expected at most {max_len} value(s), got {len}"),
        });
    }

    Ok(())
}

fn validate_vector_default_len(spec: &ParamSpec, len: usize) -> CmResult<()> {
    if len == 0 && !spec.required {
        return Ok(());
    }
    validate_vector_param_len(spec, len)
}

fn invalid_param_channel_type(spec: &ParamSpec, actual: &str) -> CmError {
    CmError::InvalidParameter {
        name: spec.name.clone(),
        message: format!(
            "expected {:?} parameter, got {} parameter",
            spec.param_type, actual
        ),
    }
}

fn validate_complex_param(spec: &ParamSpec, value: Complex64) -> CmResult<()> {
    if value.re.is_finite() && value.im.is_finite() {
        Ok(())
    } else {
        Err(CmError::InvalidParameter {
            name: spec.name.clone(),
            message: format!(
                "complex value must be finite, got <{} {}>",
                value.re, value.im
            ),
        })
    }
}

fn parse_complex_param(spec: &ParamSpec, value: &str) -> CmResult<Complex64> {
    let trimmed = value.trim();
    let Some(inner) = trimmed
        .strip_prefix('<')
        .and_then(|value| value.strip_suffix('>'))
    else {
        return Err(CmError::InvalidParameter {
            name: spec.name.clone(),
            message: format!("expected complex literal '<real imag>', got '{value}'"),
        });
    };

    let normalized = inner.replace(',', " ");
    let mut parts = normalized.split_whitespace();
    let Some(real_token) = parts.next() else {
        return Err(CmError::InvalidParameter {
            name: spec.name.clone(),
            message: format!("expected complex literal '<real imag>', got '{value}'"),
        });
    };
    let Some(imag_token) = parts.next() else {
        return Err(CmError::InvalidParameter {
            name: spec.name.clone(),
            message: format!("expected complex literal '<real imag>', got '{value}'"),
        });
    };
    if parts.next().is_some() {
        return Err(CmError::InvalidParameter {
            name: spec.name.clone(),
            message: format!("expected exactly two complex components, got '{value}'"),
        });
    }

    let real = crate::netlist::lexer::parse_spice_value(real_token).map_err(|err| {
        CmError::InvalidParameter {
            name: spec.name.clone(),
            message: format!("invalid complex real component '{real_token}': {err}"),
        }
    })?;
    let imag = crate::netlist::lexer::parse_spice_value(imag_token).map_err(|err| {
        CmError::InvalidParameter {
            name: spec.name.clone(),
            message: format!("invalid complex imaginary component '{imag_token}': {err}"),
        }
    })?;
    let value = Complex64::new(real, imag);
    validate_complex_param(spec, value)?;
    Ok(value)
}

fn push_non_ground_node_index(nodes: &mut Vec<usize>, node: usize) {
    if let Some(index) = node.checked_sub(1) {
        nodes.push(index);
    }
}

fn port_declares_type(port: &PortSpec, predicate: impl Fn(PortType) -> bool) -> bool {
    if port.allowed_types.is_empty() {
        predicate(port.default_type)
    } else {
        port.allowed_types.iter().copied().any(predicate)
    }
}

fn analog_connection_allowed(port: &PortSpec) -> bool {
    port_declares_type(port, |port_type| port_type.is_analog())
        || port_declares_type(port, |port_type| port_type == PortType::VoltageName)
}

fn scalar_analog_connection_allowed(port: &PortSpec) -> bool {
    port_declares_type(port, |port_type| {
        matches!(
            port_type,
            PortType::Voltage
                | PortType::DifferentialVoltage
                | PortType::Conductance
                | PortType::Current
                | PortType::VoltageName
        )
    })
}

fn differential_analog_connection_allowed(port: &PortSpec) -> bool {
    port_declares_type(port, |port_type| {
        matches!(
            port_type,
            PortType::DifferentialVoltage | PortType::DifferentialConductance
        )
    })
}

fn current_probe_connection_allowed(port: &PortSpec) -> bool {
    port_declares_type(port, |port_type| {
        matches!(port_type, PortType::DifferentialCurrent)
    })
}

fn branch_current_connection_allowed(port: &PortSpec) -> bool {
    port_declares_type(port, |port_type| {
        matches!(port_type, PortType::Current | PortType::VoltageName)
    })
}

fn current_output_connection_allowed(port: &PortSpec) -> bool {
    port_declares_type(port, |port_type| {
        matches!(port_type, PortType::Current | PortType::DifferentialCurrent)
    })
}

fn hybrid_connection_allowed(port: &PortSpec) -> bool {
    port_declares_type(port, |port_type| {
        matches!(port_type, PortType::Hybrid | PortType::DifferentialHybrid)
    })
}

fn event_connection_allowed(port: &PortSpec) -> bool {
    port_declares_type(port, |port_type| port_type.is_event_driven())
}

fn typed_analog_vector_connection_allowed(
    port: &PortSpec,
    elements: &[AnalogInputConnection],
) -> bool {
    elements.iter().all(|element| match element {
        AnalogInputConnection::Node(_) => scalar_analog_connection_allowed(port),
        AnalogInputConnection::Differential(_, _) => differential_analog_connection_allowed(port),
        AnalogInputConnection::CurrentProbe { .. } => current_probe_connection_allowed(port),
        AnalogInputConnection::BranchCurrent { .. }
        | AnalogInputConnection::NamedBranchCurrent { .. }
        | AnalogInputConnection::NamedCurrentSource { .. } => {
            branch_current_connection_allowed(port)
        }
        AnalogInputConnection::CurrentOutput { .. } => current_output_connection_allowed(port),
        AnalogInputConnection::Hybrid { .. } => hybrid_connection_allowed(port),
    })
}

fn vector_connection_len(connection: &PortConnection) -> Option<usize> {
    match connection {
        PortConnection::AnalogVector(nodes)
        | PortConnection::DigitalVector(nodes)
        | PortConnection::RealVector(nodes) => Some(nodes.len()),
        PortConnection::DigitalVectorMapped(nodes) => Some(nodes.len()),
        PortConnection::TypedAnalogVector(elements) => Some(elements.len()),
        _ => None,
    }
}

fn validate_branch_ordinal(
    instance_name: &str,
    port_name: &str,
    branch_ordinal: usize,
) -> CmResult<()> {
    if branch_ordinal == 0 {
        return Err(CmError::InvalidPortConnection(format!(
            "Port '{port_name}' on instance '{instance_name}' uses branch ordinal 0; branch ordinals are 1-based"
        )));
    }
    Ok(())
}

fn validate_connection_branch_ordinals(
    instance_name: &str,
    port: &PortSpec,
    connection: &PortConnection,
) -> CmResult<()> {
    match connection {
        PortConnection::CurrentProbe { branch_ordinal, .. }
        | PortConnection::Hybrid { branch_ordinal, .. }
        | PortConnection::BranchCurrent { branch_ordinal } => {
            validate_branch_ordinal(instance_name, &port.name, *branch_ordinal)?;
        }
        PortConnection::NamedBranchCurrent {
            branch_ordinal: Some(branch_ordinal),
            ..
        } => {
            validate_branch_ordinal(instance_name, &port.name, *branch_ordinal)?;
        }
        PortConnection::TypedAnalogVector(elements) => {
            for element in elements {
                if let Some(branch_ordinal) = element.branch_ordinal() {
                    validate_branch_ordinal(instance_name, &port.name, branch_ordinal)?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_port_connection(
    instance_name: &str,
    port: &PortSpec,
    connection: &PortConnection,
) -> CmResult<()> {
    if connection.is_null() {
        if port.null_allowed {
            return Ok(());
        }
        return Err(CmError::InvalidPortConnection(format!(
            "Port '{}' on instance '{}' does not allow null connection",
            port.name, instance_name
        )));
    }

    let connection_is_vector = matches!(
        connection,
        PortConnection::AnalogVector(_)
            | PortConnection::TypedAnalogVector(_)
            | PortConnection::DigitalVector(_)
            | PortConnection::DigitalVectorMapped(_)
            | PortConnection::RealVector(_)
    );
    if port.is_vector != connection_is_vector {
        let expected = if port.is_vector { "vector" } else { "scalar" };
        let actual = if connection_is_vector {
            "vector"
        } else {
            "scalar"
        };
        return Err(CmError::InvalidPortConnection(format!(
            "Port '{}' on instance '{}' expects {} connection, got {} connection",
            port.name, instance_name, expected, actual
        )));
    }

    if let Some(len) = vector_connection_len(connection) {
        if let Some(min_len) = port.vector_min_len
            && len < min_len
        {
            return Err(CmError::InvalidPortConnection(format!(
                "Port '{}' on instance '{}' expects at least {min_len} connection(s), got {len}",
                port.name, instance_name
            )));
        }
        if let Some(max_len) = port.vector_max_len
            && len > max_len
        {
            return Err(CmError::InvalidPortConnection(format!(
                "Port '{}' on instance '{}' expects at most {max_len} connection(s), got {len}",
                port.name, instance_name
            )));
        }
    }

    validate_connection_branch_ordinals(instance_name, port, connection)?;

    let category_allowed = match connection {
        PortConnection::Analog(_) => scalar_analog_connection_allowed(port),
        PortConnection::Differential(_, _) => differential_analog_connection_allowed(port),
        PortConnection::CurrentProbe { .. } => current_probe_connection_allowed(port),
        PortConnection::BranchCurrent { .. }
        | PortConnection::NamedBranchCurrent { .. }
        | PortConnection::NamedCurrentSource { .. } => branch_current_connection_allowed(port),
        PortConnection::CurrentOutput { .. } => current_output_connection_allowed(port),
        PortConnection::Hybrid { .. } => hybrid_connection_allowed(port),
        PortConnection::AnalogVector(_) => analog_connection_allowed(port),
        PortConnection::TypedAnalogVector(elements) => {
            typed_analog_vector_connection_allowed(port, elements)
        }
        PortConnection::Digital(_)
        | PortConnection::DigitalInverted(_)
        | PortConnection::DigitalVector(_)
        | PortConnection::DigitalVectorMapped(_)
        | PortConnection::Real(_)
        | PortConnection::RealVector(_) => event_connection_allowed(port),
        PortConnection::Null => true,
    };
    if !category_allowed {
        return Err(CmError::InvalidPortConnection(format!(
            "Port '{}' on instance '{}' with default type {:?} does not allow {:?} connection",
            port.name, instance_name, port.default_type, connection
        )));
    }

    Ok(())
}

//=============================================================================
// Port Connection
//=============================================================================

/// Element of an analog vector connection whose resolved input type must be
/// preserved for current-sense and named-branch inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalogInputConnection {
    /// Single-ended voltage-style input.
    Node(usize),
    /// Differential voltage-style input.
    Differential(usize, usize),
    /// Current through an inserted zero-volt probe branch.
    CurrentProbe {
        pos: usize,
        neg: usize,
        branch_ordinal: usize,
    },
    /// Explicit differential current-source output element. Positive output
    /// current flows from `pos` to `neg`.
    CurrentOutput { pos: usize, neg: usize },
    /// Hybrid/resistance port element. The branch current is the input and the
    /// same branch imposes the model's output voltage.
    Hybrid {
        pos: usize,
        neg: usize,
        branch_ordinal: usize,
    },
    /// Current through an existing branch-bearing element.
    BranchCurrent { branch_ordinal: usize },
    /// Existing branch current to be resolved after all branch-bearing
    /// elements have been built.
    NamedBranchCurrent {
        source_name: String,
        branch_ordinal: Option<usize>,
    },
    /// Current through a named independent current source.
    NamedCurrentSource {
        source_name: String,
        source_index: usize,
    },
}

impl AnalogInputConnection {
    pub(crate) fn primary_node(&self) -> Option<usize> {
        match self {
            AnalogInputConnection::Node(node) => Some(*node),
            AnalogInputConnection::Differential(pos, _) => Some(*pos),
            AnalogInputConnection::CurrentProbe { pos, .. }
            | AnalogInputConnection::CurrentOutput { pos, .. }
            | AnalogInputConnection::Hybrid { pos, .. } => Some(*pos),
            AnalogInputConnection::BranchCurrent { .. }
            | AnalogInputConnection::NamedBranchCurrent { .. }
            | AnalogInputConnection::NamedCurrentSource { .. } => None,
        }
    }

    fn remap_circuit_nodes(&mut self, remap: &mut impl FnMut(usize) -> usize) {
        match self {
            AnalogInputConnection::Node(node) => *node = remap(*node),
            AnalogInputConnection::Differential(pos, neg)
            | AnalogInputConnection::CurrentProbe { pos, neg, .. }
            | AnalogInputConnection::CurrentOutput { pos, neg }
            | AnalogInputConnection::Hybrid { pos, neg, .. } => {
                *pos = remap(*pos);
                *neg = remap(*neg);
            }
            AnalogInputConnection::BranchCurrent { .. }
            | AnalogInputConnection::NamedBranchCurrent { .. }
            | AnalogInputConnection::NamedCurrentSource { .. } => {}
        }
    }

    fn value_from_solution(
        &self,
        solution: &[Value],
        num_nodes: usize,
        current_source_values: &[Value],
    ) -> Value {
        fn node_voltage(solution: &[Value], node: usize) -> Value {
            if node == 0 {
                0.0
            } else {
                solution.get(node - 1).copied().unwrap_or(0.0)
            }
        }

        fn branch_current(solution: &[Value], num_nodes: usize, branch_ordinal: usize) -> Value {
            branch_ordinal
                .checked_sub(1)
                .and_then(|branch_index| solution.get(num_nodes + branch_index))
                .copied()
                .unwrap_or(0.0)
        }

        match self {
            AnalogInputConnection::Node(node) => node_voltage(solution, *node),
            AnalogInputConnection::Differential(pos, neg) => {
                node_voltage(solution, *pos) - node_voltage(solution, *neg)
            }
            AnalogInputConnection::CurrentProbe { branch_ordinal, .. }
            | AnalogInputConnection::BranchCurrent { branch_ordinal } => {
                branch_current(solution, num_nodes, *branch_ordinal)
            }
            AnalogInputConnection::Hybrid { branch_ordinal, .. } => {
                branch_current(solution, num_nodes, *branch_ordinal)
            }
            AnalogInputConnection::CurrentOutput { .. } => 0.0,
            AnalogInputConnection::NamedBranchCurrent {
                branch_ordinal: Some(branch_ordinal),
                ..
            } => branch_current(solution, num_nodes, *branch_ordinal),
            AnalogInputConnection::NamedBranchCurrent {
                branch_ordinal: None,
                ..
            } => 0.0,
            AnalogInputConnection::NamedCurrentSource { source_index, .. } => current_source_values
                .get(*source_index)
                .copied()
                .unwrap_or(0.0),
        }
    }

    fn current_probe(&self) -> Option<(usize, usize, usize)> {
        match self {
            AnalogInputConnection::CurrentProbe {
                pos,
                neg,
                branch_ordinal,
            } => Some((*pos, *neg, *branch_ordinal)),
            _ => None,
        }
    }

    pub(crate) fn branch_ordinal(&self) -> Option<usize> {
        match self {
            AnalogInputConnection::CurrentProbe { branch_ordinal, .. }
            | AnalogInputConnection::BranchCurrent { branch_ordinal }
            | AnalogInputConnection::Hybrid { branch_ordinal, .. } => Some(*branch_ordinal),
            AnalogInputConnection::NamedBranchCurrent { branch_ordinal, .. } => *branch_ordinal,
            _ => None,
        }
    }
}

/// Resolved digital event-node reference with ngspice-compatible inversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DigitalPortConnection {
    pub node: usize,
    pub inverted: bool,
}

impl DigitalPortConnection {
    pub fn new(node: usize, inverted: bool) -> Self {
        Self { node, inverted }
    }

    fn input_value(&self, values: &HashMap<usize, DigitalValue>) -> DigitalValue {
        let value = values.get(&self.node).copied().unwrap_or_default();
        if self.inverted { value.invert() } else { value }
    }

    fn output_value(&self, value: DigitalValue) -> DigitalValue {
        if self.inverted { value.invert() } else { value }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum EventInputSignatureValue {
    Digital(DigitalValue),
    Real(u64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct EventInputSignatureEntry {
    event_time: Option<Value>,
    value: EventInputSignatureValue,
}

/// Connection for a single port
#[derive(Debug, Clone)]
pub enum PortConnection {
    /// Analog node connection (circuit node index, 0 = ground)
    Analog(usize),
    /// Differential analog connection (positive node, negative node)
    Differential(usize, usize),
    /// Digital node connection
    Digital(usize),
    /// Inverted digital node connection
    DigitalInverted(usize),
    /// Real-valued event node connection
    Real(usize),
    /// Vector of analog nodes
    AnalogVector(Vec<usize>),
    /// Vector of typed analog inputs.
    TypedAnalogVector(Vec<AnalogInputConnection>),
    /// Vector of digital nodes
    DigitalVector(Vec<usize>),
    /// Vector of digital nodes with per-entry inversion
    DigitalVectorMapped(Vec<DigitalPortConnection>),
    /// Vector of real-valued event nodes
    RealVector(Vec<usize>),
    /// Current through an inserted zero-volt probe branch.
    CurrentProbe {
        pos: usize,
        neg: usize,
        branch_ordinal: usize,
    },
    /// Explicit differential current-source output. Positive output current
    /// flows from `pos` to `neg`.
    CurrentOutput { pos: usize, neg: usize },
    /// Hybrid/resistance port. The branch current is the input and the same
    /// branch imposes the model's output voltage.
    Hybrid {
        pos: usize,
        neg: usize,
        branch_ordinal: usize,
    },
    /// Current through an existing branch-bearing element.
    BranchCurrent { branch_ordinal: usize },
    /// Existing branch current to be resolved after all branch-bearing
    /// elements have been built.
    NamedBranchCurrent {
        source_name: String,
        branch_ordinal: Option<usize>,
    },
    /// Current through a named independent current source.
    NamedCurrentSource {
        source_name: String,
        source_index: usize,
    },
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
            PortConnection::DigitalInverted(n) => Some(*n),
            PortConnection::Real(n) => Some(*n),
            PortConnection::AnalogVector(v) => v.first().copied(),
            PortConnection::TypedAnalogVector(v) => {
                v.first().and_then(AnalogInputConnection::primary_node)
            }
            PortConnection::DigitalVector(v) => v.first().copied(),
            PortConnection::DigitalVectorMapped(v) => v.first().map(|connection| connection.node),
            PortConnection::RealVector(v) => v.first().copied(),
            PortConnection::CurrentProbe { pos, .. }
            | PortConnection::CurrentOutput { pos, .. }
            | PortConnection::Hybrid { pos, .. } => Some(*pos),
            PortConnection::BranchCurrent { .. }
            | PortConnection::NamedBranchCurrent { .. }
            | PortConnection::NamedCurrentSource { .. } => None,
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
    /// Optional MNA branch variable per vector output element.
    output_vector_branches: HashMap<(usize, usize), usize>,
    /// Node-voltage count for the most recently supplied MNA solution.
    solution_num_nodes: usize,
    /// Node-voltage count used to build the current context port bindings.
    port_context_solution_num_nodes: Option<usize>,
    /// Last accepted event-input signature for combinational event models.
    last_event_input_signature: Option<Vec<EventInputSignatureEntry>>,
    /// Reusable scratch signature for transient event-input skip checks.
    event_input_signature_scratch: Vec<EventInputSignatureEntry>,
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
            .field("output_vector_branches", &self.output_vector_branches)
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
        Self::new_with_string_vectors(
            name,
            model,
            connections,
            params,
            string_params,
            &[],
            real_vector_params,
            integer_vector_params,
        )
    }

    /// Create a new XSPICE instance with all supported parameter channels.
    pub fn new_with_string_vectors(
        name: impl Into<String>,
        model: Arc<dyn CodeModel>,
        connections: Vec<PortConnection>,
        params: &[(String, Value)],
        string_params: &[(String, String)],
        string_vector_params: &[(String, Vec<String>)],
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

        for (port, connection) in ports.iter().zip(connections.iter()) {
            validate_port_connection(&name, port, connection)?;
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
            if param_spec.required {
                continue;
            }

            match param_spec.param_type {
                super::ParamType::Complex => {
                    if let Some(default) = param_spec.complex_default {
                        validate_complex_param(param_spec, default)?;
                        context.set_complex_param(&param_spec.name, default);
                    }
                }
                super::ParamType::String => {
                    if let Some(default) = &param_spec.string_default {
                        context.set_string_param(&param_spec.name, default);
                    }
                }
                super::ParamType::ComplexVector => {
                    if let Some(default) = &param_spec.complex_vector_default {
                        validate_vector_default_len(param_spec, default.len())?;
                        for value in default {
                            validate_complex_param(param_spec, *value)?;
                        }
                        context.set_complex_vector_param(&param_spec.name, default.clone());
                    }
                }
                super::ParamType::StringVector => {
                    if let Some(default) = &param_spec.string_vector_default {
                        validate_vector_default_len(param_spec, default.len())?;
                        context.set_string_vector_param(&param_spec.name, default.clone());
                    }
                }
                super::ParamType::RealVector => {
                    if let Some(default) = &param_spec.real_vector_default {
                        validate_vector_default_len(param_spec, default.len())?;
                        for value in default {
                            validate_numeric_param(param_spec, *value)?;
                        }
                        context.set_real_vector_param(&param_spec.name, default.clone());
                    }
                }
                super::ParamType::IntegerVector => {
                    if let Some(default) = &param_spec.integer_vector_default {
                        validate_vector_default_len(param_spec, default.len())?;
                        for value in default {
                            validate_numeric_param(param_spec, *value as Value)?;
                        }
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
        let provided_param_keys: HashSet<String> = params
            .iter()
            .map(|(name, _)| canonical_param_key(name))
            .chain(
                string_params
                    .iter()
                    .map(|(name, _)| canonical_param_key(name)),
            )
            .chain(
                string_vector_params
                    .iter()
                    .map(|(name, _)| canonical_param_key(name)),
            )
            .chain(
                real_vector_params
                    .iter()
                    .map(|(name, _)| canonical_param_key(name)),
            )
            .chain(
                integer_vector_params
                    .iter()
                    .map(|(name, _)| canonical_param_key(name)),
            )
            .collect();
        for key in &provided_param_keys {
            context.mark_param_provided(key);
        }

        // Override with instance parameters
        for (name, value) in params {
            if let Some(spec) = param_specs.get(&canonical_param_key(name)) {
                match spec.param_type {
                    ParamType::Real | ParamType::Integer | ParamType::Boolean => {
                        validate_numeric_param(spec, *value)?;
                    }
                    ParamType::Complex | ParamType::String => {
                        return Err(invalid_param_channel_type(spec, "scalar"));
                    }
                    ParamType::StringVector
                    | ParamType::RealVector
                    | ParamType::IntegerVector
                    | ParamType::ComplexVector => {
                        return Err(CmError::InvalidParameter {
                            name: spec.name.clone(),
                            message: "expected vector parameter, got scalar parameter".to_string(),
                        });
                    }
                }
            }
            let value = if param_specs
                .get(&canonical_param_key(name))
                .is_some_and(|spec| matches!(spec.param_type, ParamType::Integer))
            {
                value.round()
            } else {
                *value
            };
            context.set_param(name, value);
        }
        for (name, value) in string_params {
            if let Some(spec) = param_specs.get(&canonical_param_key(name)) {
                match spec.param_type {
                    ParamType::String => {}
                    ParamType::Complex => {
                        let value = parse_complex_param(spec, value)?;
                        context.set_complex_param(name, value);
                        continue;
                    }
                    ParamType::StringVector
                    | ParamType::RealVector
                    | ParamType::IntegerVector
                    | ParamType::ComplexVector => {
                        return Err(CmError::InvalidParameter {
                            name: spec.name.clone(),
                            message: "expected vector parameter, got string parameter".to_string(),
                        });
                    }
                    ParamType::Real | ParamType::Integer | ParamType::Boolean => {
                        return Err(invalid_param_channel_type(spec, "string"));
                    }
                }
            }
            context.set_string_param(name, value);
        }
        for (name, values) in string_vector_params {
            if let Some(spec) = param_specs.get(&canonical_param_key(name)) {
                match spec.param_type {
                    ParamType::StringVector => {
                        validate_vector_param_len(spec, values.len())?;
                    }
                    ParamType::ComplexVector => {
                        validate_vector_param_len(spec, values.len())?;
                        let mut complex_values = Vec::with_capacity(values.len());
                        for value in values {
                            complex_values.push(parse_complex_param(spec, value)?);
                        }
                        context.set_complex_vector_param(name, complex_values);
                        continue;
                    }
                    _ => return Err(invalid_param_channel_type(spec, "string-vector")),
                }
            }
            context.set_string_vector_param(name, values.clone());
        }
        for (name, values) in real_vector_params {
            if let Some(spec) = param_specs.get(&canonical_param_key(name)) {
                match spec.param_type {
                    ParamType::RealVector => {
                        validate_vector_param_len(spec, values.len())?;
                        for value in values {
                            validate_numeric_param(spec, *value)?;
                        }
                    }
                    ParamType::IntegerVector => {
                        validate_vector_param_len(spec, values.len())?;
                        let mut integer_values = Vec::with_capacity(values.len());
                        for value in values {
                            validate_numeric_param(spec, *value)?;
                            integer_values.push(value.round() as i64);
                        }
                        context.set_integer_vector_param(name, integer_values);
                        continue;
                    }
                    ParamType::StringVector => {
                        return Err(invalid_param_channel_type(spec, "real-vector"));
                    }
                    ParamType::ComplexVector => {
                        return Err(invalid_param_channel_type(spec, "real-vector"));
                    }
                    _ => return Err(invalid_param_channel_type(spec, "real-vector")),
                }
            }
            context.set_real_vector_param(name, values.clone());
        }
        for (name, values) in integer_vector_params {
            if let Some(spec) = param_specs.get(&canonical_param_key(name)) {
                match spec.param_type {
                    ParamType::IntegerVector => {
                        validate_vector_param_len(spec, values.len())?;
                        for value in values {
                            validate_numeric_param(spec, *value as Value)?;
                        }
                    }
                    ParamType::RealVector => {
                        validate_vector_param_len(spec, values.len())?;
                        let real_values: Vec<Value> =
                            values.iter().map(|value| *value as Value).collect();
                        for value in &real_values {
                            validate_numeric_param(spec, *value)?;
                        }
                        context.set_real_vector_param(name, real_values);
                        continue;
                    }
                    ParamType::StringVector => {
                        return Err(invalid_param_channel_type(spec, "integer-vector"));
                    }
                    ParamType::ComplexVector => {
                        return Err(invalid_param_channel_type(spec, "integer-vector"));
                    }
                    _ => return Err(invalid_param_channel_type(spec, "integer-vector")),
                }
            }
            context.set_integer_vector_param(name, values.clone());
        }
        for spec in model.parameters() {
            if spec.required && !provided_param_keys.contains(&canonical_param_key(&spec.name)) {
                return Err(CmError::MissingParameter(spec.name.clone()));
            }
        }

        for (port, connection) in ports.iter().zip(connections.iter()) {
            let width = match connection {
                PortConnection::AnalogVector(nodes)
                | PortConnection::DigitalVector(nodes)
                | PortConnection::RealVector(nodes) => nodes.len(),
                PortConnection::DigitalVectorMapped(nodes) => nodes.len(),
                PortConnection::TypedAnalogVector(elements) => elements.len(),
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
                let width = context.port_width(&port.name);
                if port.is_vector {
                    context.init_output_vector(&port.name, port.default_type, width);
                } else {
                    context.init_output(&port.name, port.default_type);
                }
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
            output_vector_branches: HashMap::new(),
            solution_num_nodes: 0,
            port_context_solution_num_nodes: None,
            last_event_input_signature: None,
            event_input_signature_scratch: Vec::new(),
            initialized: false,
        })
    }

    /// Get the model name
    pub fn model_name(&self) -> &str {
        self.model.name()
    }

    /// Whether this instance needs global conservative Newton damping.
    #[inline]
    pub fn requires_conservative_newton_damping(&self) -> bool {
        self.model.requires_conservative_newton_damping()
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

    /// Set transient run context for code models with ngspice run-context defaults.
    pub(crate) fn set_transient_run_context(&mut self, tstep: Option<Value>, tstop: Option<Value>) {
        self.context.set_transient_run_context(tstep, tstop);
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
            Ok(Ok(())) => {}
            Ok(Err(err)) => {
                self.context = context_before_init;
                return Err(err);
            }
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
    /// * `real_values` - Real-valued event node values
    pub fn update_inputs(
        &mut self,
        solution: &[Value],
        num_nodes: usize,
        digital_values: &HashMap<usize, DigitalValue>,
        digital_event_times: &HashMap<usize, Value>,
        real_values: &HashMap<usize, Value>,
        real_event_times: &HashMap<usize, Value>,
        current_source_values: &[Value],
    ) {
        if self.solution_num_nodes != num_nodes {
            self.port_context_solution_num_nodes = None;
            self.solution_num_nodes = num_nodes;
        }

        fn node_voltage(solution: &[Value], node: usize) -> Value {
            if node == 0 {
                0.0
            } else {
                solution.get(node - 1).copied().unwrap_or(0.0)
            }
        }

        fn branch_current(solution: &[Value], num_nodes: usize, branch_ordinal: usize) -> Value {
            branch_ordinal
                .checked_sub(1)
                .and_then(|branch_index| solution.get(num_nodes + branch_index))
                .copied()
                .unwrap_or(0.0)
        }

        let ports = &self.ports;

        for (i, port) in ports.iter().enumerate() {
            if port.direction != super::PortDirection::In
                && port.direction != super::PortDirection::InOut
            {
                continue;
            }

            match &self.connections[i] {
                PortConnection::Analog(node) => {
                    let v = node_voltage(solution, *node);
                    self.context.set_input_analog(&port.name, v);
                }
                PortConnection::Differential(pos, neg) => {
                    self.context.set_input_analog(
                        &port.name,
                        node_voltage(solution, *pos) - node_voltage(solution, *neg),
                    );
                }
                PortConnection::CurrentProbe { branch_ordinal, .. }
                | PortConnection::BranchCurrent { branch_ordinal } => {
                    self.context.set_input_analog(
                        &port.name,
                        branch_current(solution, num_nodes, *branch_ordinal),
                    );
                }
                PortConnection::Hybrid { branch_ordinal, .. } => {
                    self.context.set_input_analog(
                        &port.name,
                        branch_current(solution, num_nodes, *branch_ordinal),
                    );
                }
                PortConnection::CurrentOutput { .. } => {}
                PortConnection::NamedBranchCurrent {
                    branch_ordinal: Some(branch_ordinal),
                    ..
                } => {
                    self.context.set_input_analog(
                        &port.name,
                        branch_current(solution, num_nodes, *branch_ordinal),
                    );
                }
                PortConnection::NamedBranchCurrent {
                    branch_ordinal: None,
                    ..
                } => {
                    self.context.set_input_analog(&port.name, 0.0);
                }
                PortConnection::NamedCurrentSource { source_index, .. } => {
                    self.context.set_input_analog(
                        &port.name,
                        current_source_values
                            .get(*source_index)
                            .copied()
                            .unwrap_or(0.0),
                    );
                }
                PortConnection::Digital(node) => {
                    let val = digital_values.get(node).copied().unwrap_or_default();
                    self.context.set_input_digital(&port.name, val);
                    if let Some(time) = digital_event_times.get(node).copied() {
                        self.context.set_input_digital_event_time(&port.name, time);
                    }
                }
                PortConnection::DigitalInverted(node) => {
                    let val = digital_values
                        .get(node)
                        .copied()
                        .unwrap_or_default()
                        .invert();
                    self.context.set_input_digital(&port.name, val);
                    if let Some(time) = digital_event_times.get(node).copied() {
                        self.context.set_input_digital_event_time(&port.name, time);
                    }
                }
                PortConnection::Real(node) => {
                    let value = real_values.get(node).copied().unwrap_or(0.0);
                    self.context.set_input_real(&port.name, value);
                    if let Some(time) = real_event_times.get(node).copied() {
                        self.context.set_input_real_event_time(&port.name, time);
                    }
                }
                PortConnection::AnalogVector(nodes) => {
                    self.context.set_input_analog_vector_from_fn(
                        &port.name,
                        nodes.len(),
                        |index| AnalogValue::new(node_voltage(solution, nodes[index])),
                    );
                }
                PortConnection::TypedAnalogVector(elements) => {
                    self.context.set_input_analog_vector_from_fn(
                        &port.name,
                        elements.len(),
                        |index| {
                            AnalogValue::new(elements[index].value_from_solution(
                                solution,
                                num_nodes,
                                current_source_values,
                            ))
                        },
                    );
                }
                PortConnection::DigitalVector(nodes) => {
                    self.context.set_input_digital_vector_from_fn(
                        &port.name,
                        nodes.len(),
                        |index| {
                            digital_values
                                .get(&nodes[index])
                                .copied()
                                .unwrap_or_default()
                        },
                    );
                    self.context.set_input_digital_vector_event_times_from_fn(
                        &port.name,
                        nodes.len(),
                        |index| digital_event_times.get(&nodes[index]).copied(),
                    );
                }
                PortConnection::DigitalVectorMapped(nodes) => {
                    self.context.set_input_digital_vector_from_fn(
                        &port.name,
                        nodes.len(),
                        |index| nodes[index].input_value(digital_values),
                    );
                    self.context.set_input_digital_vector_event_times_from_fn(
                        &port.name,
                        nodes.len(),
                        |index| digital_event_times.get(&nodes[index].node).copied(),
                    );
                }
                PortConnection::RealVector(nodes) => {
                    self.context
                        .set_input_real_vector_from_fn(&port.name, nodes.len(), |index| {
                            real_values.get(&nodes[index]).copied().unwrap_or(0.0)
                        });
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
    /// * `phase` - Rollback/commit phase of this evaluation
    pub fn evaluate(
        &mut self,
        time: Value,
        timestep: Value,
        analysis: AnalysisType,
        phase: EvaluationPhase,
    ) -> CmResult<()> {
        self.context.clear_stamps();
        self.refresh_port_context_bindings();
        self.context.time = time;
        self.context.timestep = timestep;
        self.context.analysis = analysis;
        self.context.set_evaluation_phase(phase);
        self.context.call_type = match analysis {
            AnalysisType::DcOp | AnalysisType::DcSweep => CallType::DcAnalysis,
            AnalysisType::Ac => CallType::AcAnalysis,
            AnalysisType::Transient => CallType::TransientAnalysis,
            _ => CallType::DcAnalysis,
        };

        let track_event_input_signature = self.should_track_event_input_signature(analysis);
        if track_event_input_signature {
            self.refresh_event_input_signature();
            if self.last_event_input_signature.as_deref()
                == Some(self.event_input_signature_scratch.as_slice())
            {
                return Ok(());
            }
        }

        let context_before_evaluate = self.context.clone();
        match catch_unwind(AssertUnwindSafe(|| self.model.evaluate(&mut self.context))) {
            Ok(Ok(())) => {
                if track_event_input_signature && phase != EvaluationPhase::RollbackableProbe {
                    self.last_event_input_signature =
                        Some(self.event_input_signature_scratch.clone());
                }
                Ok(())
            }
            Ok(Err(err)) => {
                self.context = context_before_evaluate;
                self.port_context_solution_num_nodes = None;
                Err(err)
            }
            Err(payload) => {
                self.context = context_before_evaluate;
                Err(self.model_panic_error("evaluation", payload))
            }
        }
    }

    fn should_track_event_input_signature(&self, analysis: AnalysisType) -> bool {
        analysis == AnalysisType::Transient && self.model.can_skip_unchanged_event_inputs()
    }

    fn refresh_event_input_signature(&mut self) {
        self.event_input_signature_scratch.clear();
        for (port, connection) in self.ports.iter().zip(self.connections.iter()) {
            if port.direction != super::PortDirection::In
                && port.direction != super::PortDirection::InOut
            {
                continue;
            }

            match connection {
                PortConnection::Digital(_) | PortConnection::DigitalInverted(_) => {
                    self.event_input_signature_scratch
                        .push(EventInputSignatureEntry {
                            event_time: self.context.input_digital_event_time(&port.name),
                            value: EventInputSignatureValue::Digital(
                                self.context.input_digital(&port.name).unwrap_or_default(),
                            ),
                        });
                }
                PortConnection::DigitalVector(nodes) => {
                    let values = self.context.input_digital_vector_values(&port.name);
                    for index in 0..nodes.len() {
                        self.event_input_signature_scratch
                            .push(EventInputSignatureEntry {
                                event_time: self
                                    .context
                                    .input_digital_vector_event_time(&port.name, index),
                                value: EventInputSignatureValue::Digital(
                                    values
                                        .and_then(|values| values.get(index))
                                        .copied()
                                        .unwrap_or_default(),
                                ),
                            });
                    }
                }
                PortConnection::DigitalVectorMapped(nodes) => {
                    let values = self.context.input_digital_vector_values(&port.name);
                    for index in 0..nodes.len() {
                        self.event_input_signature_scratch
                            .push(EventInputSignatureEntry {
                                event_time: self
                                    .context
                                    .input_digital_vector_event_time(&port.name, index),
                                value: EventInputSignatureValue::Digital(
                                    values
                                        .and_then(|values| values.get(index))
                                        .copied()
                                        .unwrap_or_default(),
                                ),
                            });
                    }
                }
                PortConnection::Real(_) => {
                    self.event_input_signature_scratch
                        .push(EventInputSignatureEntry {
                            event_time: self.context.input_real_event_time(&port.name),
                            value: EventInputSignatureValue::Real(
                                self.context.input_real(&port.name).unwrap_or(0.0).to_bits(),
                            ),
                        });
                }
                PortConnection::RealVector(nodes) => {
                    let values = self.context.input_real_vector_values(&port.name);
                    for index in 0..nodes.len() {
                        self.event_input_signature_scratch
                            .push(EventInputSignatureEntry {
                                event_time: None,
                                value: EventInputSignatureValue::Real(
                                    values
                                        .and_then(|values| values.get(index))
                                        .copied()
                                        .unwrap_or(0.0)
                                        .to_bits(),
                                ),
                            });
                    }
                }
                _ => {}
            }
        }
    }

    fn refresh_port_context_bindings(&mut self) {
        if self.port_context_solution_num_nodes == Some(self.solution_num_nodes) {
            return;
        }

        self.context.clear_port_nodes();
        for (port, connection) in self.ports.iter().zip(self.connections.iter()) {
            match connection {
                PortConnection::Analog(node) => {
                    self.context.set_port_node(&port.name, *node);
                }
                PortConnection::Differential(pos, neg) => {
                    self.context.set_port_terminals(&port.name, *pos, *neg);
                }
                PortConnection::CurrentProbe {
                    pos,
                    neg,
                    branch_ordinal,
                } => {
                    self.context.set_port_terminals(&port.name, *pos, *neg);
                    self.context.set_port_control_column(
                        &port.name,
                        self.solution_num_nodes + *branch_ordinal - 1,
                    );
                }
                PortConnection::Hybrid {
                    pos,
                    neg,
                    branch_ordinal,
                } => {
                    self.context.set_port_terminals(&port.name, *pos, *neg);
                    self.context.set_port_control_column(
                        &port.name,
                        self.solution_num_nodes + *branch_ordinal - 1,
                    );
                }
                PortConnection::BranchCurrent { branch_ordinal } => {
                    self.context.set_port_control_column(
                        &port.name,
                        self.solution_num_nodes + *branch_ordinal - 1,
                    );
                }
                PortConnection::NamedBranchCurrent {
                    branch_ordinal: Some(branch_ordinal),
                    ..
                } => {
                    self.context.set_port_control_column(
                        &port.name,
                        self.solution_num_nodes + *branch_ordinal - 1,
                    );
                }
                PortConnection::CurrentOutput { pos, neg } => {
                    self.context.set_port_terminals(&port.name, *pos, *neg);
                }
                PortConnection::AnalogVector(nodes) => {
                    self.context.set_port_vector_terminals(
                        &port.name,
                        nodes.iter().copied().map(|node| (node, 0)).collect(),
                    );
                }
                PortConnection::TypedAnalogVector(elements) => {
                    self.context.set_port_vector_terminals(
                        &port.name,
                        elements
                            .iter()
                            .map(|element| match element {
                                AnalogInputConnection::Node(node) => (*node, 0),
                                AnalogInputConnection::Differential(pos, neg)
                                | AnalogInputConnection::CurrentOutput { pos, neg }
                                | AnalogInputConnection::CurrentProbe { pos, neg, .. }
                                | AnalogInputConnection::Hybrid { pos, neg, .. } => (*pos, *neg),
                                _ => (0, 0),
                            })
                            .collect(),
                    );
                }
                _ => {}
            }
        }

        self.port_context_solution_num_nodes = Some(self.solution_num_nodes);
    }

    /// Get output value for stamping
    pub fn output(&self, port_name: &str) -> Value {
        self.context.output(port_name)
    }

    /// Get vector output values for stamping.
    pub fn output_vector(&self, port_name: &str) -> Vec<Value> {
        self.context.output_vector(port_name)
    }

    /// Get the last analog input value supplied to a port.
    pub fn analog_input_value(&self, port_name: &str) -> Value {
        self.context.input(port_name)
    }

    /// Get one element from the last analog vector input supplied to a port.
    pub fn analog_vector_input_value(&self, port_name: &str, index: usize) -> Value {
        self.context
            .input_analog_vector_values(port_name)
            .and_then(|values| values.get(index))
            .map(|value| value.value)
            .unwrap_or(0.0)
    }

    /// Get model-provided control partials for a voltage output port.
    pub fn output_input_partials(&self, output_port: &str) -> Vec<(String, Value)> {
        self.model.output_input_partials(&self.context, output_port)
    }

    /// Get model-provided complex AC control partials for an analog output.
    pub fn output_input_ac_partials(
        &self,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, crate::Complex64)> {
        self.model
            .output_input_ac_partials(&self.context, output_port, frequency)
    }

    /// Get model-provided vector control partials for a voltage output port.
    pub fn output_input_vector_partials(&self, output_port: &str) -> Vec<(String, usize, Value)> {
        self.model
            .output_input_vector_partials(&self.context, output_port)
    }

    /// Get model-provided complex AC vector control partials for an analog output.
    pub fn output_input_vector_ac_partials(
        &self,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, usize, crate::Complex64)> {
        self.model
            .output_input_vector_ac_partials(&self.context, output_port, frequency)
    }

    /// Get model-provided scalar control partials for one vector output
    /// element.
    pub fn output_vector_input_partials(
        &self,
        output_port: &str,
        output_index: usize,
    ) -> Vec<(String, Value)> {
        self.model
            .output_vector_input_partials(&self.context, output_port, output_index)
    }

    /// Get model-provided complex AC scalar control partials for one vector
    /// output element.
    pub fn output_vector_input_ac_partials(
        &self,
        output_port: &str,
        output_index: usize,
        frequency: Value,
    ) -> Vec<(String, crate::Complex64)> {
        self.model.output_vector_input_ac_partials(
            &self.context,
            output_port,
            output_index,
            frequency,
        )
    }

    /// Get model-provided vector control partials for one vector output
    /// element.
    pub fn output_vector_input_vector_partials(
        &self,
        output_port: &str,
        output_index: usize,
    ) -> Vec<(String, usize, Value)> {
        self.model
            .output_vector_input_vector_partials(&self.context, output_port, output_index)
    }

    /// Get model-provided complex AC vector control partials for one vector
    /// output element.
    pub fn output_vector_input_vector_ac_partials(
        &self,
        output_port: &str,
        output_index: usize,
        frequency: Value,
    ) -> Vec<(String, usize, crate::Complex64)> {
        self.model.output_vector_input_vector_ac_partials(
            &self.context,
            output_port,
            output_index,
            frequency,
        )
    }

    /// Absolute transient breakpoints contributed by this code model.
    pub(crate) fn transient_breakpoints(&self) -> CmResult<Vec<Value>> {
        self.model.transient_breakpoints(&self.context)
    }

    /// Zero-based circuit node indices for voltage outputs that should not
    /// participate in generic transient voltage LTE control.
    pub fn transient_voltage_lte_excluded_nodes(&self) -> Vec<usize> {
        let mut nodes = Vec::new();
        for (port_idx, port) in self.ports.iter().enumerate() {
            if port.direction != super::PortDirection::Out
                || !port.default_type.is_analog()
                || !self
                    .model
                    .excludes_output_from_transient_voltage_lte(&port.name)
            {
                continue;
            }

            match self.connections.get(port_idx) {
                Some(PortConnection::Analog(node)) => {
                    push_non_ground_node_index(&mut nodes, *node);
                }
                Some(PortConnection::Differential(pos, neg))
                | Some(PortConnection::CurrentOutput { pos, neg })
                | Some(PortConnection::Hybrid { pos, neg, .. }) => {
                    push_non_ground_node_index(&mut nodes, *pos);
                    push_non_ground_node_index(&mut nodes, *neg);
                }
                Some(PortConnection::AnalogVector(output_nodes)) => {
                    for node in output_nodes {
                        push_non_ground_node_index(&mut nodes, *node);
                    }
                }
                Some(PortConnection::TypedAnalogVector(elements)) => {
                    for element in elements {
                        match element {
                            AnalogInputConnection::Node(node) => {
                                push_non_ground_node_index(&mut nodes, *node);
                            }
                            AnalogInputConnection::Differential(pos, neg)
                            | AnalogInputConnection::CurrentOutput { pos, neg }
                            | AnalogInputConnection::Hybrid { pos, neg, .. } => {
                                push_non_ground_node_index(&mut nodes, *pos);
                                push_non_ground_node_index(&mut nodes, *neg);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        nodes
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
        fn current_output_self_conductance(port: &PortSpec, conductance: Value) -> Value {
            match port.default_type {
                PortType::Current
                | PortType::DifferentialCurrent
                | PortType::Conductance
                | PortType::DifferentialConductance => conductance,
                _ => 0.0,
            }
        }

        fn stamp_current_output<M, R>(
            matrix_add: &mut M,
            rhs_add: &mut R,
            pos: usize,
            neg: usize,
            conductance: Value,
            current: Value,
        ) where
            M: FnMut(usize, usize, Value),
            R: FnMut(usize, Value),
        {
            if pos > 0 {
                let pos_row = pos - 1;
                matrix_add(pos_row, pos_row, conductance);
                if neg > 0 {
                    matrix_add(pos_row, neg - 1, -conductance);
                }
                rhs_add(pos_row, -current);
            }
            if neg > 0 {
                let neg_row = neg - 1;
                if pos > 0 {
                    matrix_add(neg_row, pos - 1, -conductance);
                }
                matrix_add(neg_row, neg_row, conductance);
                rhs_add(neg_row, current);
            }
        }

        fn stamp_voltage_output<M, R>(
            matrix_add: &mut M,
            rhs_add: &mut R,
            branch_row: usize,
            pos: usize,
            neg: usize,
            value: Value,
        ) where
            M: FnMut(usize, usize, Value),
            R: FnMut(usize, Value),
        {
            if pos > 0 {
                let pos_row = pos - 1;
                matrix_add(branch_row, pos_row, 1.0);
                matrix_add(pos_row, branch_row, 1.0);
            }
            if neg > 0 {
                let neg_row = neg - 1;
                matrix_add(branch_row, neg_row, -1.0);
                matrix_add(neg_row, branch_row, -1.0);
            }
            rhs_add(branch_row, value);
        }

        let ports = &self.ports;

        for (i, port) in ports.iter().enumerate() {
            if port.direction != super::PortDirection::Out
                && port.direction != super::PortDirection::InOut
            {
                continue;
            }

            let Some((conductance, value)) = self.get_analog_contribution(i) else {
                continue;
            };

            match (port.default_type, &self.connections[i]) {
                (
                    PortType::Voltage | PortType::DifferentialVoltage,
                    PortConnection::Analog(node),
                ) => {
                    if let Some(branch_ordinal) = self.branch_ordinal_at(i)
                        && self.solution_num_nodes > 0
                    {
                        let branch_row = self.solution_num_nodes + branch_ordinal - 1;
                        stamp_voltage_output(
                            &mut matrix_add,
                            &mut rhs_add,
                            branch_row,
                            *node,
                            0,
                            value,
                        );
                    }
                }
                (
                    PortType::Voltage | PortType::DifferentialVoltage,
                    PortConnection::Differential(pos, neg),
                )
                | (
                    PortType::Voltage | PortType::DifferentialVoltage,
                    PortConnection::Hybrid { pos, neg, .. },
                ) => {
                    if let Some(branch_ordinal) = self.branch_ordinal_at(i)
                        && self.solution_num_nodes > 0
                    {
                        let branch_row = self.solution_num_nodes + branch_ordinal - 1;
                        stamp_voltage_output(
                            &mut matrix_add,
                            &mut rhs_add,
                            branch_row,
                            *pos,
                            *neg,
                            value,
                        );
                    }
                }
                (
                    PortType::Current
                    | PortType::DifferentialCurrent
                    | PortType::Conductance
                    | PortType::DifferentialConductance,
                    PortConnection::Analog(node),
                ) => stamp_current_output(
                    &mut matrix_add,
                    &mut rhs_add,
                    *node,
                    0,
                    current_output_self_conductance(port, conductance),
                    value,
                ),
                (
                    PortType::Current
                    | PortType::DifferentialCurrent
                    | PortType::Conductance
                    | PortType::DifferentialConductance,
                    PortConnection::Differential(pos, neg)
                    | PortConnection::CurrentOutput { pos, neg },
                ) => stamp_current_output(
                    &mut matrix_add,
                    &mut rhs_add,
                    *pos,
                    *neg,
                    current_output_self_conductance(port, conductance),
                    value,
                ),
                _ => {}
            }
        }

        // Process any queued stamps from the model
        for (row, col, value) in self.context.drain_stamps() {
            matrix_add(row, col, value);
        }
        for (node, value) in self.context.drain_rhs() {
            rhs_add(node, value);
        }
    }

    /// Get pending digital events
    pub(crate) fn take_pending_events(&mut self) -> Vec<PendingDigitalEvent> {
        self.context.take_pending_events()
    }

    /// Get pending real-valued events
    pub(crate) fn take_pending_real_events(&mut self) -> Vec<PendingRealEvent> {
        self.context.take_pending_real_events()
    }

    /// Drain transient breakpoint requests while preserving context storage.
    pub(crate) fn drain_requested_breakpoints(&mut self) -> impl Iterator<Item = Value> + '_ {
        self.context.drain_requested_breakpoints()
    }

    /// Process digital events scheduled by this instance
    pub fn schedule_events(&mut self, event_queue: &mut EventQueue, current_time: Value) {
        for PendingDigitalEvent {
            port_name,
            start_index,
            values,
            delay,
        } in self.context.drain_pending_events()
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
                PortConnection::DigitalInverted(node) => {
                    if let Some(value) = values.first().copied() {
                        event_queue.schedule_delayed(
                            current_time,
                            delay,
                            *node,
                            &port_name,
                            &self.name,
                            value.invert(),
                        );
                    }
                }
                PortConnection::DigitalVector(nodes) => {
                    for (node, value) in nodes.iter().skip(start_index).zip(values.into_iter()) {
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
                PortConnection::DigitalVectorMapped(nodes) => {
                    for (connection, value) in
                        nodes.iter().skip(start_index).zip(values.into_iter())
                    {
                        event_queue.schedule_delayed(
                            current_time,
                            delay,
                            connection.node,
                            &port_name,
                            &self.name,
                            connection.output_value(value),
                        );
                    }
                }
                _ => {}
            }
        }

        for PendingRealEvent {
            port_name,
            start_index,
            values,
            delay,
        } in self.context.drain_pending_real_events()
        {
            let Some(&port_idx) = self.port_indices.get(&port_name) else {
                continue;
            };
            let Some(connection) = self.connections.get(port_idx) else {
                continue;
            };

            match connection {
                PortConnection::Real(node) => {
                    if let Some(value) = values.first().copied() {
                        event_queue.schedule_real_delayed(
                            current_time,
                            delay,
                            *node,
                            &port_name,
                            &self.name,
                            value,
                        );
                    }
                }
                PortConnection::RealVector(nodes) => {
                    for (node, value) in nodes.iter().skip(start_index).zip(values.into_iter()) {
                        event_queue.schedule_real_delayed(
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

    /// Set ngspice-compatible transient ramp time in seconds.
    pub fn set_ramptime(&mut self, ramptime: Value) {
        self.context.set_ramptime(ramptime);
    }

    //=========================================================================
    // Circuit Integration Methods
    //=========================================================================

    /// Get port connections
    #[inline]
    pub fn connections(&self) -> &[PortConnection] {
        &self.connections
    }

    /// Zero-volt probe branches that must be stamped so current input ports
    /// expose branch current to the code model.
    pub fn current_probe_branches(&self) -> Vec<(usize, usize, usize)> {
        let mut probes = Vec::new();
        for connection in &self.connections {
            match connection {
                PortConnection::CurrentProbe {
                    pos,
                    neg,
                    branch_ordinal,
                } => probes.push((*pos, *neg, *branch_ordinal)),
                PortConnection::TypedAnalogVector(elements) => {
                    probes.extend(
                        elements
                            .iter()
                            .filter_map(AnalogInputConnection::current_probe),
                    );
                }
                _ => {}
            }
        }
        probes
    }

    /// Branch ordinal for a scalar analog control port.
    pub fn branch_control_ordinal(&self, port_name: &str) -> Option<usize> {
        let index = *self.port_indices.get(port_name)?;
        match self.connections.get(index)? {
            PortConnection::CurrentProbe { branch_ordinal, .. }
            | PortConnection::BranchCurrent { branch_ordinal }
            | PortConnection::Hybrid { branch_ordinal, .. } => Some(*branch_ordinal),
            PortConnection::NamedBranchCurrent { branch_ordinal, .. } => *branch_ordinal,
            _ => None,
        }
    }

    /// Branch ordinal for one element of an analog vector control port.
    pub fn branch_vector_control_ordinal(&self, port_name: &str, index: usize) -> Option<usize> {
        let port_index = *self.port_indices.get(port_name)?;
        match self.connections.get(port_index)? {
            PortConnection::TypedAnalogVector(elements) => elements
                .get(index)
                .and_then(AnalogInputConnection::branch_ordinal),
            _ => None,
        }
    }

    /// Resolve `%vnam` inputs after branch-bearing elements and independent
    /// current sources have been allocated.
    pub fn resolve_branch_references(
        &mut self,
        mut branch_lookup: impl FnMut(&str) -> Option<usize>,
        mut current_source_lookup: impl FnMut(&str) -> Option<usize>,
    ) -> CmResult<()> {
        fn resolve_one(
            instance_name: &str,
            element: &mut AnalogInputConnection,
            branch_lookup: &mut impl FnMut(&str) -> Option<usize>,
            current_source_lookup: &mut impl FnMut(&str) -> Option<usize>,
        ) -> CmResult<()> {
            let AnalogInputConnection::NamedBranchCurrent {
                source_name,
                branch_ordinal,
            } = element
            else {
                return Ok(());
            };
            if branch_ordinal.is_some() {
                return Ok(());
            }
            if let Some(branch) = branch_lookup(source_name) {
                *branch_ordinal = Some(branch);
                return Ok(());
            }
            if let Some(source_index) = current_source_lookup(source_name) {
                *element = AnalogInputConnection::NamedCurrentSource {
                    source_name: source_name.clone(),
                    source_index,
                };
                return Ok(());
            }
            Err(CmError::InvalidPortConnection(format!(
                "XSPICE instance '{instance_name}' references unknown branch or current source '{source_name}'"
            )))
        }

        for connection in &mut self.connections {
            match connection {
                PortConnection::NamedBranchCurrent {
                    source_name,
                    branch_ordinal,
                } if branch_ordinal.is_none() => {
                    if let Some(branch) = branch_lookup(source_name) {
                        *branch_ordinal = Some(branch);
                    } else if let Some(source_index) = current_source_lookup(source_name) {
                        *connection = PortConnection::NamedCurrentSource {
                            source_name: source_name.clone(),
                            source_index,
                        };
                    } else {
                        return Err(CmError::InvalidPortConnection(format!(
                            "XSPICE instance '{}' references unknown branch or current source '{}'",
                            self.name, source_name
                        )));
                    }
                }
                PortConnection::TypedAnalogVector(elements) => {
                    for element in elements {
                        resolve_one(
                            &self.name,
                            element,
                            &mut branch_lookup,
                            &mut current_source_lookup,
                        )?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Remap circuit node IDs after a topology-level reference-node rewrite.
    pub fn remap_circuit_nodes(&mut self, mut remap: impl FnMut(usize) -> usize) {
        for connection in &mut self.connections {
            match connection {
                PortConnection::Analog(node)
                | PortConnection::Digital(node)
                | PortConnection::DigitalInverted(node)
                | PortConnection::Real(node) => {
                    *node = remap(*node);
                }
                PortConnection::Differential(pos, neg)
                | PortConnection::CurrentOutput { pos, neg }
                | PortConnection::Hybrid { pos, neg, .. } => {
                    *pos = remap(*pos);
                    *neg = remap(*neg);
                }
                PortConnection::CurrentProbe { pos, neg, .. } => {
                    *pos = remap(*pos);
                    *neg = remap(*neg);
                }
                PortConnection::AnalogVector(nodes)
                | PortConnection::DigitalVector(nodes)
                | PortConnection::RealVector(nodes) => {
                    for node in nodes {
                        *node = remap(*node);
                    }
                }
                PortConnection::TypedAnalogVector(elements) => {
                    for element in elements {
                        element.remap_circuit_nodes(&mut remap);
                    }
                }
                PortConnection::DigitalVectorMapped(nodes) => {
                    for connection in nodes {
                        connection.node = remap(connection.node);
                    }
                }
                PortConnection::BranchCurrent { .. }
                | PortConnection::NamedBranchCurrent { .. }
                | PortConnection::NamedCurrentSource { .. } => {}
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

    /// Assign an MNA branch ordinal to one element of a voltage-type vector
    /// output port.
    pub fn set_output_vector_branch(
        &mut self,
        port_idx: usize,
        element_idx: usize,
        branch_ordinal: usize,
    ) -> CmResult<()> {
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
        if !is_output || !port.is_vector || !is_voltage_port {
            return Err(CmError::Internal(format!(
                "Port '{}' on instance {} is not a voltage vector output",
                port.name, self.name
            )));
        }
        let width = self.context.port_width(&port.name);
        if element_idx >= width {
            return Err(CmError::Internal(format!(
                "Vector output element {} is out of bounds for port '{}' on instance {} with width {}",
                element_idx, port.name, self.name, width
            )));
        }
        self.output_vector_branches
            .insert((port_idx, element_idx), branch_ordinal);
        Ok(())
    }

    /// Get assigned branch ordinal for a port, if any.
    #[inline]
    pub fn branch_ordinal_at(&self, port_idx: usize) -> Option<usize> {
        if let Some(PortConnection::Hybrid { branch_ordinal, .. }) = self.connections.get(port_idx)
        {
            return Some(*branch_ordinal);
        }
        self.output_branches
            .get(port_idx)
            .and_then(|entry| entry.as_ref().copied())
    }

    /// Get assigned branch ordinal for one vector output element, if any.
    #[inline]
    pub fn branch_vector_output_ordinal(
        &self,
        port_idx: usize,
        element_idx: usize,
    ) -> Option<usize> {
        self.output_vector_branches
            .get(&(port_idx, element_idx))
            .copied()
    }

    /// Get analog contribution (conductance, current) for stamping
    ///
    /// Returns Some((conductance, current)) for output ports that produce
    /// analog contributions, None for inputs or digital ports.
    pub fn get_analog_contribution(&self, port_idx: usize) -> Option<(Value, Value)> {
        let ports = &self.ports;
        if let Some(port) = ports.get(port_idx) {
            let is_output = port.direction == super::PortDirection::Out
                || matches!(
                    self.connections.get(port_idx),
                    Some(PortConnection::Hybrid { .. })
                );
            if is_output && !port.is_vector && port.default_type.is_analog() {
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

    /// Whether a port produces analog vector contributions for stamping.
    pub fn has_analog_vector_contributions(&self, port_idx: usize) -> bool {
        self.ports.get(port_idx).is_some_and(|port| {
            port.direction == super::PortDirection::Out
                && port.is_vector
                && port.default_type.is_analog()
        })
    }

    /// Whether a port has analog vector small-signal output data available.
    ///
    /// DC stamping intentionally limits `has_analog_vector_contributions` to
    /// output ports because inout conductance models may already queue
    /// explicit matrix stamps. AC matrix assembly uses this broader predicate
    /// to read the converged linearization without re-evaluating the model.
    pub fn has_analog_vector_small_signal_contributions(&self, port_idx: usize) -> bool {
        self.ports.get(port_idx).is_some_and(|port| {
            matches!(
                port.direction,
                super::PortDirection::Out | super::PortDirection::InOut
            ) && port.is_vector
                && port.default_type.is_analog()
        })
    }

    /// Get one analog vector contribution without allocating the full vector.
    pub fn analog_vector_contribution_at(
        &self,
        port_idx: usize,
        output_index: usize,
    ) -> (Value, Value) {
        let Some(port) = self.ports.get(port_idx) else {
            return (0.0, 0.0);
        };
        if !self.has_analog_vector_contributions(port_idx) {
            return (0.0, 0.0);
        }

        (
            self.context.partial_vector_value(&port.name, output_index),
            self.context.output_vector_value(&port.name, output_index),
        )
    }

    /// Get one analog vector small-signal contribution for AC assembly.
    pub fn analog_vector_small_signal_contribution_at(
        &self,
        port_idx: usize,
        output_index: usize,
    ) -> (Value, Value) {
        let Some(port) = self.ports.get(port_idx) else {
            return (0.0, 0.0);
        };
        if !self.has_analog_vector_small_signal_contributions(port_idx) {
            return (0.0, 0.0);
        }

        (
            self.context.partial_vector_value(&port.name, output_index),
            self.context.output_vector_value(&port.name, output_index),
        )
    }

    /// Get analog vector contributions (conductance, current/value) for stamping.
    pub fn get_analog_vector_contributions(&self, port_idx: usize) -> Option<Vec<(Value, Value)>> {
        if !self.has_analog_vector_contributions(port_idx) {
            return None;
        }
        let port = self.ports.get(port_idx)?;
        let width = self.context.port_width(&port.name);
        Some(
            (0..width)
                .map(|index| self.analog_vector_contribution_at(port_idx, index))
                .collect(),
        )
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
                if port.is_vector {
                    let width = self.context.port_width(&port.name);
                    return (0..width).all(|index| {
                        let curr = self.context.output_vector_value(&port.name, index);
                        let prev = self.context.output_vector_prev_value(&port.name, index);
                        (curr - prev).abs() <= tol + tol * curr.abs().max(prev.abs())
                    });
                }
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
    use crate::xspice::{DigitalState, DigitalStrength, EventValue, ParamSpec, PortDirection};
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::Mutex;
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

    struct ControlColumnModel {
        observed_columns: Arc<Mutex<Vec<Option<usize>>>>,
        ports: Vec<PortSpec>,
    }

    struct ErrorAfterPortMutationModel {
        observed_columns: Arc<Mutex<Vec<Option<usize>>>>,
        fail_next: AtomicBool,
        ports: Vec<PortSpec>,
    }

    impl ControlColumnModel {
        fn new(observed_columns: Arc<Mutex<Vec<Option<usize>>>>) -> Self {
            Self {
                observed_columns,
                ports: vec![PortSpec::input("sense", PortType::DifferentialCurrent)],
            }
        }
    }

    impl CodeModel for ControlColumnModel {
        fn name(&self) -> &str {
            "control_column_model"
        }

        fn ports(&self) -> &[PortSpec] {
            &self.ports
        }

        fn parameters(&self) -> &[ParamSpec] {
            &[]
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
            self.observed_columns
                .lock()
                .expect("observed column lock should not be poisoned")
                .push(ctx.port_control_column("sense"));
            Ok(())
        }
    }

    impl ErrorAfterPortMutationModel {
        fn new(observed_columns: Arc<Mutex<Vec<Option<usize>>>>) -> Self {
            Self {
                observed_columns,
                fail_next: AtomicBool::new(true),
                ports: vec![PortSpec::input("sense", PortType::DifferentialCurrent)],
            }
        }
    }

    impl CodeModel for ErrorAfterPortMutationModel {
        fn name(&self) -> &str {
            "error_after_port_mutation_model"
        }

        fn ports(&self) -> &[PortSpec] {
            &self.ports
        }

        fn parameters(&self) -> &[ParamSpec] {
            &[]
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
            self.observed_columns
                .lock()
                .expect("observed column lock should not be poisoned")
                .push(ctx.port_control_column("sense"));
            if self.fail_next.swap(false, Ordering::SeqCst) {
                ctx.set_port_control_column("sense", 999);
                return Err(CmError::EvaluationError("synthetic failure".to_string()));
            }
            Ok(())
        }
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
                    vector_min_len: None,
                    vector_max_len: None,
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
                        vector_min_len: None,
                        vector_max_len: None,
                        description: String::new(),
                    },
                    PortSpec {
                        name: "late_input".to_string(),
                        direction: PortDirection::In,
                        default_type: PortType::Voltage,
                        allowed_types: vec![PortType::Voltage],
                        is_vector: false,
                        null_allowed: false,
                        vector_min_len: None,
                        vector_max_len: None,
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
                    vector_min_len: None,
                    vector_max_len: None,
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

    fn model_with_ports(ports: Vec<PortSpec>) -> PanicModel {
        let mut model = PanicModel::new();
        model.ports = ports;
        model
    }

    #[test]
    fn instance_converts_string_channels_to_complex_parameters() {
        let instance = XspiceInstance::new_with_string_vectors(
            "Acomplex",
            Arc::new(model_with_params(vec![
                ParamSpec::complex("pole", Complex64::new(0.0, 0.0)),
                ParamSpec::complex_vector("zeros", Vec::new()),
            ])),
            vec![PortConnection::Analog(1)],
            &[],
            &[("pole".to_string(), "<1k -2meg>".to_string())],
            &[(
                "zeros".to_string(),
                vec!["<3 4>".to_string(), "<5, -6>".to_string()],
            )],
            &[],
            &[],
        )
        .expect("complex parameters should be parsed from string channels");

        assert_eq!(
            instance.context.complex_param("POLE"),
            Some(Complex64::new(1.0e3, -2.0e6))
        );
        assert_eq!(
            instance.context.complex_vector_param("zeros"),
            Some([Complex64::new(3.0, 4.0), Complex64::new(5.0, -6.0)].as_slice())
        );
    }

    #[test]
    fn instance_rejects_out_of_range_known_numeric_parameter() {
        let rounded = XspiceInstance::new(
            "AparamRounded",
            Arc::new(model_with_params(vec![
                ParamSpec::integer("ic", 2).with_range(0.0, 2.0),
            ])),
            vec![PortConnection::Analog(1)],
            &[("ic".to_string(), 2.4)],
            &[],
            &[],
            &[],
        )
        .expect("integer parameter range should apply after rounding");
        assert_eq!(rounded.context.param("ic"), 2.0);

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
    fn instance_rounds_fractional_known_integer_parameter_like_ngspice() {
        let model = model_with_params(vec![ParamSpec::integer("select_value", 1)]);
        let instance = XspiceInstance::new(
            "Aparam",
            Arc::new(model),
            vec![PortConnection::Analog(1)],
            &[("select_value".to_string(), 1.5)],
            &[],
            &[],
            &[],
        )
        .expect("known integer parameter should round fractional values");

        assert_eq!(instance.context.param("select_value"), 2.0);
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
    fn instance_accepts_required_real_vector_override_with_bounded_placeholder_default() {
        let model = model_with_params(vec![
            ParamSpec::real_vector("points", Vec::new())
                .required()
                .with_vector_min_len(2),
        ]);
        let instance = XspiceInstance::new(
            "Avec",
            Arc::new(model),
            vec![PortConnection::Analog(1)],
            &[],
            &[],
            &[("points".to_string(), vec![3.0, 4.0])],
            &[],
        )
        .expect("required bounded vector override should construct");

        assert_eq!(instance.real_vector_param("points").unwrap(), &[3.0, 4.0]);
    }

    #[test]
    fn instance_accepts_optional_empty_real_vector_default_with_bounded_overrides() {
        let model = model_with_params(vec![
            ParamSpec::real_vector("points", Vec::new()).with_vector_min_len(2),
        ]);
        let instance = XspiceInstance::new(
            "Avec",
            Arc::new(model),
            vec![PortConnection::Analog(1)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("optional empty vector default should not trigger supplied-value bounds");

        assert_eq!(instance.real_vector_param("points").unwrap(), &[]);
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

    #[test]
    fn instance_rejects_string_override_for_known_numeric_parameter() {
        let model = model_with_params(vec![ParamSpec::real("gain", 2.0)]);
        let err = XspiceInstance::new(
            "Aparam",
            Arc::new(model),
            vec![PortConnection::Analog(1)],
            &[],
            &[("gain".to_string(), "bad".to_string())],
            &[],
            &[],
        )
        .expect_err("known numeric parameter must not accept string override");

        assert!(matches!(
            err,
            CmError::InvalidParameter { ref name, .. } if name.eq_ignore_ascii_case("gain")
        ));
    }

    #[test]
    fn instance_rejects_numeric_override_for_known_string_parameter() {
        let model = model_with_params(vec![ParamSpec::string("file", "default.txt")]);
        let err = XspiceInstance::new(
            "Aparam",
            Arc::new(model),
            vec![PortConnection::Analog(1)],
            &[("file".to_string(), 1.0)],
            &[],
            &[],
            &[],
        )
        .expect_err("known string parameter must not accept numeric override");

        assert!(matches!(
            err,
            CmError::InvalidParameter { ref name, .. } if name.eq_ignore_ascii_case("file")
        ));
    }

    #[test]
    fn instance_rejects_null_connection_for_required_port() {
        let model = model_with_ports(vec![PortSpec::input("in", PortType::Voltage)]);
        let err = XspiceInstance::new(
            "Aport",
            Arc::new(model),
            vec![PortConnection::Null],
            &[],
            &[],
            &[],
            &[],
        )
        .expect_err("required port must not accept null connection");

        assert!(matches!(err, CmError::InvalidPortConnection(_)));
        assert!(err.to_string().contains("in"));
    }

    #[test]
    fn instance_accepts_null_connection_for_nullable_port() {
        let model = model_with_ports(vec![
            PortSpec::input("optional", PortType::Voltage).nullable(),
        ]);
        let instance = XspiceInstance::new(
            "Aport",
            Arc::new(model),
            vec![PortConnection::Null],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("nullable port should accept null connection");

        assert!(matches!(
            instance.connection("optional"),
            Some(PortConnection::Null)
        ));
    }

    #[test]
    fn instance_rejects_scalar_connection_for_vector_port() {
        let model = model_with_ports(vec![PortSpec::vector_input("bus", PortType::Digital)]);
        let err = XspiceInstance::new(
            "Aport",
            Arc::new(model),
            vec![PortConnection::Digital(1)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect_err("vector port must not accept scalar connection");

        assert!(matches!(err, CmError::InvalidPortConnection(_)));
        assert!(err.to_string().contains("bus"));
    }

    #[test]
    fn instance_rejects_vector_connection_for_scalar_port() {
        let model = model_with_ports(vec![PortSpec::input("in", PortType::Voltage)]);
        let err = XspiceInstance::new(
            "Aport",
            Arc::new(model),
            vec![PortConnection::AnalogVector(vec![1, 2])],
            &[],
            &[],
            &[],
            &[],
        )
        .expect_err("scalar port must not accept vector connection");

        assert!(matches!(err, CmError::InvalidPortConnection(_)));
        assert!(err.to_string().contains("in"));
    }

    #[test]
    fn instance_rejects_typed_vector_element_not_allowed_by_port_contract() {
        let model = model_with_ports(vec![PortSpec::vector_output("out", PortType::Voltage)]);
        let err = XspiceInstance::new(
            "Aport",
            Arc::new(model),
            vec![PortConnection::TypedAnalogVector(vec![
                AnalogInputConnection::CurrentOutput { pos: 1, neg: 0 },
            ])],
            &[],
            &[],
            &[],
            &[],
        )
        .expect_err("voltage-only vector port must not accept current-output elements");

        assert!(matches!(err, CmError::InvalidPortConnection(_)));
        assert!(err.to_string().contains("out"));
    }

    #[test]
    fn analog_vector_contribution_accessor_avoids_full_vector_allocation() {
        let model = model_with_ports(vec![PortSpec::vector_output("out", PortType::Current)]);
        let mut instance = XspiceInstance::new(
            "Avecout",
            Arc::new(model),
            vec![PortConnection::AnalogVector(vec![1, 2])],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("analog vector output should construct");

        instance
            .context
            .set_output_vector_with_partials("out", vec![1.5, 2.5], vec![0.25, 0.5]);

        assert!(instance.has_analog_vector_contributions(0));
        assert_eq!(instance.analog_vector_contribution_at(0, 0), (0.25, 1.5));
        assert_eq!(instance.analog_vector_contribution_at(0, 1), (0.5, 2.5));
        assert_eq!(instance.analog_vector_contribution_at(0, 2), (0.0, 0.0));
        assert_eq!(
            instance
                .get_analog_vector_contributions(0)
                .expect("vector contributions should still be available"),
            vec![(0.25, 1.5), (0.5, 2.5)]
        );
    }

    #[test]
    fn standalone_stamp_uses_current_output_sign_convention() {
        let model = model_with_ports(vec![PortSpec::output("out", PortType::Current)]);
        let mut instance = XspiceInstance::new(
            "Acurrent",
            Arc::new(model),
            vec![PortConnection::Analog(2)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("current output should construct");
        instance.context.set_output_with_partial("out", 5.0, 0.25);

        let mut stamps = Vec::new();
        let mut rhs = Vec::new();
        instance.stamp(
            |row, col, value| stamps.push((row, col, value)),
            |row, value| rhs.push((row, value)),
        );

        assert_eq!(stamps, vec![(1, 1, 0.25)]);
        assert_eq!(rhs, vec![(1, -5.0)]);
    }

    #[test]
    fn standalone_stamp_uses_assigned_voltage_output_branch() {
        let model = model_with_ports(vec![PortSpec::output("out", PortType::Voltage)]);
        let mut instance = XspiceInstance::new(
            "Avoltage",
            Arc::new(model),
            vec![PortConnection::Analog(2)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("voltage output should construct");
        instance
            .set_output_branch(0, 3)
            .expect("voltage output branch assignment");
        instance.update_inputs(
            &[0.0, 0.0, 0.0, 0.0],
            4,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &[],
        );
        instance.context.set_output("out", 7.0);

        let mut stamps = Vec::new();
        let mut rhs = Vec::new();
        instance.stamp(
            |row, col, value| stamps.push((row, col, value)),
            |row, value| rhs.push((row, value)),
        );

        assert_eq!(stamps, vec![(6, 1, 1.0), (1, 6, 1.0)]);
        assert_eq!(rhs, vec![(6, 7.0)]);
    }

    #[test]
    fn vector_output_convergence_uses_element_accessors() {
        let model = model_with_ports(vec![PortSpec::vector_output("out", PortType::Voltage)]);
        let mut instance = XspiceInstance::new(
            "Avecout",
            Arc::new(model),
            vec![PortConnection::AnalogVector(vec![1, 2])],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("analog vector output should construct");

        instance.context.set_output_vector("out", vec![1.0, 2.0]);
        instance
            .context
            .set_output_vector("out", vec![1.0 + 1.0e-13, 2.0 - 1.0e-13]);
        assert!(instance.is_converged(1.0e-12));

        instance.context.set_output_vector("out", vec![1.1, 2.0]);
        assert!(!instance.is_converged(1.0e-12));
    }

    #[test]
    fn instance_rejects_digital_connection_for_analog_port() {
        let model = model_with_ports(vec![PortSpec::input("in", PortType::Voltage)]);
        let err = XspiceInstance::new(
            "Aport",
            Arc::new(model),
            vec![PortConnection::Digital(1)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect_err("analog port must not accept digital connection");

        assert!(matches!(err, CmError::InvalidPortConnection(_)));
        assert!(err.to_string().contains("in"));
    }

    #[test]
    fn instance_rejects_analog_connection_for_digital_port() {
        let model = model_with_ports(vec![PortSpec::input("in", PortType::Digital)]);
        let err = XspiceInstance::new(
            "Aport",
            Arc::new(model),
            vec![PortConnection::Analog(1)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect_err("digital port must not accept analog connection");

        assert!(matches!(err, CmError::InvalidPortConnection(_)));
        assert!(err.to_string().contains("in"));
    }

    #[test]
    fn inverted_digital_input_connection_feeds_inverted_value() {
        let model = model_with_ports(vec![PortSpec::input("in", PortType::Digital)]);
        let mut instance = XspiceInstance::new(
            "Ain",
            Arc::new(model),
            vec![PortConnection::DigitalInverted(1)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("inverted digital input should construct");
        let mut digital_values = HashMap::new();
        digital_values.insert(1, DigitalValue::zero());

        instance.update_inputs(
            &[],
            0,
            &digital_values,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &[],
        );

        assert_eq!(
            instance.context.input_digital("in"),
            Some(DigitalValue::one())
        );
    }

    #[test]
    fn update_inputs_reuses_vector_input_buffers() {
        let model = model_with_ports(vec![
            PortSpec::vector_input("ain", PortType::Voltage),
            PortSpec::vector_input("din", PortType::Digital),
            PortSpec::vector_input("rin", PortType::Real),
        ]);
        let mut instance = XspiceInstance::new(
            "Avecin",
            Arc::new(model),
            vec![
                PortConnection::AnalogVector(vec![1, 2]),
                PortConnection::DigitalVector(vec![3, 4]),
                PortConnection::RealVector(vec![5, 6]),
            ],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("vector input instance should construct");

        let mut digital_values = HashMap::new();
        digital_values.insert(3, DigitalValue::zero());
        digital_values.insert(4, DigitalValue::one());
        let mut digital_event_times = HashMap::new();
        digital_event_times.insert(4, 2.0e-9);
        let mut real_values = HashMap::new();
        real_values.insert(5, 10.0);
        real_values.insert(6, 11.0);

        instance.update_inputs(
            &[1.0, 2.0],
            2,
            &digital_values,
            &digital_event_times,
            &real_values,
            &HashMap::new(),
            &[],
        );

        let analog_ptr = instance
            .context
            .input_analog_vector_values("ain")
            .unwrap()
            .as_ptr();
        let digital_ptr = instance
            .context
            .input_digital_vector_values("din")
            .unwrap()
            .as_ptr();
        let real_ptr = instance
            .context
            .input_real_vector_values("rin")
            .unwrap()
            .as_ptr();

        digital_values.insert(3, DigitalValue::one());
        digital_values.insert(4, DigitalValue::zero());
        digital_event_times.clear();
        digital_event_times.insert(3, 3.0e-9);
        real_values.insert(5, 20.0);
        real_values.insert(6, 21.0);

        instance.update_inputs(
            &[3.0, 4.0],
            2,
            &digital_values,
            &digital_event_times,
            &real_values,
            &HashMap::new(),
            &[],
        );

        let analog = instance.context.input_analog_vector_values("ain").unwrap();
        assert_eq!(analog.as_ptr(), analog_ptr);
        assert_eq!(
            analog.iter().map(|value| value.value).collect::<Vec<_>>(),
            vec![3.0, 4.0]
        );

        let digital = instance.context.input_digital_vector_values("din").unwrap();
        assert_eq!(digital.as_ptr(), digital_ptr);
        assert_eq!(digital, &[DigitalValue::one(), DigitalValue::zero()]);
        assert_eq!(
            instance.context.input_digital_vector_event_time("din", 0),
            Some(3.0e-9)
        );
        assert_eq!(
            instance.context.input_digital_vector_event_time("din", 1),
            None
        );

        let real = instance.context.input_real_vector_values("rin").unwrap();
        assert_eq!(real.as_ptr(), real_ptr);
        assert_eq!(real, &[20.0, 21.0]);
    }

    #[test]
    fn inverted_digital_output_connection_schedules_inverted_value() {
        let model = model_with_ports(vec![PortSpec::output("out", PortType::Digital)]);
        let mut instance = XspiceInstance::new(
            "Aout",
            Arc::new(model),
            vec![PortConnection::DigitalInverted(1)],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("inverted digital output should construct");
        instance
            .context
            .set_output_digital("out", DigitalValue::one(), 0.0);

        let mut event_queue = EventQueue::new();
        instance.schedule_events(&mut event_queue, 0.0);
        let events = event_queue.pop_events_at(0.0);

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].node_id, 1);
        assert_eq!(events[0].value, EventValue::Digital(DigitalValue::zero()));
    }

    #[test]
    fn instance_rejects_zero_branch_ordinal_connections() {
        let model = model_with_ports(vec![PortSpec::input(
            "sense",
            PortType::DifferentialCurrent,
        )]);
        let err = XspiceInstance::new(
            "Aport",
            Arc::new(model),
            vec![PortConnection::CurrentProbe {
                pos: 1,
                neg: 0,
                branch_ordinal: 0,
            }],
            &[],
            &[],
            &[],
            &[],
        )
        .expect_err("branch ordinals are one-based and zero must be rejected");

        assert!(matches!(err, CmError::InvalidPortConnection(_)));
        assert!(err.to_string().contains("branch ordinal 0"));
    }

    #[test]
    fn cached_port_context_updates_control_columns_when_node_count_changes() {
        let observed_columns = Arc::new(Mutex::new(Vec::new()));
        let model = ControlColumnModel::new(Arc::clone(&observed_columns));
        let mut instance = XspiceInstance::new(
            "Acolumn",
            Arc::new(model),
            vec![PortConnection::CurrentProbe {
                pos: 1,
                neg: 0,
                branch_ordinal: 2,
            }],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("current-probe instance should construct");

        instance.update_inputs(
            &[0.0; 8],
            5,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &[],
        );
        instance
            .evaluate(
                0.0,
                1e-9,
                AnalysisType::Transient,
                EvaluationPhase::DirectEvaluation,
            )
            .expect("first evaluation should succeed");
        instance
            .evaluate(
                1e-9,
                1e-9,
                AnalysisType::Transient,
                EvaluationPhase::DirectEvaluation,
            )
            .expect("cached-port evaluation should succeed");
        instance.update_inputs(
            &[0.0; 10],
            7,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &[],
        );
        instance
            .evaluate(
                2e-9,
                1e-9,
                AnalysisType::Transient,
                EvaluationPhase::DirectEvaluation,
            )
            .expect("evaluation after node-count change should succeed");

        let observed_columns = observed_columns
            .lock()
            .expect("observed column lock should not be poisoned");
        assert_eq!(&*observed_columns, &[Some(6), Some(6), Some(8)]);
        assert_eq!(instance.port_context_solution_num_nodes, Some(7));
    }

    #[test]
    fn model_error_invalidates_cached_port_context_bindings() {
        let observed_columns = Arc::new(Mutex::new(Vec::new()));
        let model = ErrorAfterPortMutationModel::new(Arc::clone(&observed_columns));
        let mut instance = XspiceInstance::new(
            "Acolumn",
            Arc::new(model),
            vec![PortConnection::CurrentProbe {
                pos: 1,
                neg: 0,
                branch_ordinal: 2,
            }],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("current-probe instance should construct");

        instance.update_inputs(
            &[0.0; 8],
            5,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &[],
        );
        let err = instance
            .evaluate(
                0.0,
                1e-9,
                AnalysisType::Transient,
                EvaluationPhase::DirectEvaluation,
            )
            .expect_err("first evaluation should fail");
        assert!(err.to_string().contains("synthetic failure"));
        assert_eq!(instance.port_context_solution_num_nodes, None);

        instance
            .evaluate(
                1e-9,
                1e-9,
                AnalysisType::Transient,
                EvaluationPhase::DirectEvaluation,
            )
            .expect("second evaluation should rebuild cached port context");

        let observed_columns = observed_columns
            .lock()
            .expect("observed column lock should not be poisoned");
        assert_eq!(&*observed_columns, &[Some(6), Some(6)]);
        assert_eq!(instance.port_context_solution_num_nodes, Some(5));
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
            instance.evaluate(
                0.0,
                1e-9,
                AnalysisType::Transient,
                EvaluationPhase::DirectEvaluation,
            )
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
            instance.evaluate(
                0.0,
                1e-9,
                AnalysisType::Transient,
                EvaluationPhase::DirectEvaluation,
            )
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
            instance.update_inputs(
                &[0.0],
                1,
                &HashMap::new(),
                &HashMap::new(),
                &HashMap::new(),
                &HashMap::new(),
                &[],
            );
            instance
                .evaluate(
                    0.0,
                    1e-9,
                    AnalysisType::Transient,
                    EvaluationPhase::DirectEvaluation,
                )
                .expect("evaluation should use the stable port contract");
            instance.stamp(|_, _, _| {}, |_, _| {});
        }));
        result.expect("stable port contract must avoid connection/index panics");
    }
}
