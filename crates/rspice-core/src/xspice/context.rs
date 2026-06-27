//! XSPICE Code Model Execution Context
//!
//! Provides the runtime context passed to code models during evaluation.
//! Handles port value access, parameter lookup, and state management.

use super::{DigitalValue, PortType};
use crate::Value;
use std::collections::HashMap;

//=============================================================================
// Analysis Type
//=============================================================================

/// Type of analysis being performed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisType {
    /// DC operating point
    DcOp,
    /// DC sweep
    DcSweep,
    /// AC small-signal
    Ac,
    /// Transient time-domain
    Transient,
    /// Pole-zero analysis
    PoleZero,
    /// Sensitivity analysis
    Sensitivity,
    /// Noise analysis
    Noise,
}

/// Reason for model evaluation call
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallType {
    /// Initialization call at simulation start
    Init,
    /// DC operating point analysis
    DcAnalysis,
    /// AC analysis (small-signal)
    AcAnalysis,
    /// Transient analysis
    TransientAnalysis,
    /// Event-driven update (digital state change)
    EventDriven,
    /// Model probing (get outputs without side effects)
    Probe,
}

//=============================================================================
// Port Values
//=============================================================================

/// Analog port value (voltage or current)
#[derive(Debug, Clone, Copy, Default)]
pub struct AnalogValue {
    /// Current value
    pub value: Value,
    /// Previous value (for computing derivatives)
    pub prev_value: Value,
    /// Partial derivative contribution to matrix
    pub partial: Value,
}

impl AnalogValue {
    /// Create a new analog value
    pub fn new(value: Value) -> Self {
        Self {
            value,
            prev_value: value,
            partial: 0.0,
        }
    }
}

/// Input port value container
#[derive(Debug, Clone)]
pub enum InputValue {
    /// Analog voltage/current value
    Analog(AnalogValue),
    /// Vector of analog values
    AnalogVector(Vec<AnalogValue>),
    /// Digital value (12-state)
    Digital(DigitalValue),
    /// Vector of digital values
    DigitalVector(Vec<DigitalValue>),
}

impl InputValue {
    /// Try to get an analog scalar value.
    pub fn try_analog(&self) -> Option<Value> {
        match self {
            InputValue::Analog(v) => Some(v.value),
            _ => None,
        }
    }

    /// Get analog value, panics if not analog
    pub fn analog(&self) -> Value {
        self.try_analog().expect("Expected analog value")
    }

    /// Get analog value or default
    pub fn analog_or(&self, default: Value) -> Value {
        self.try_analog().unwrap_or(default)
    }

    /// Try to get an analog vector.
    pub fn try_analog_vector(&self) -> Option<&[AnalogValue]> {
        match self {
            InputValue::AnalogVector(v) => Some(v),
            _ => None,
        }
    }

    /// Get analog vector
    pub fn analog_vector(&self) -> &[AnalogValue] {
        self.try_analog_vector().expect("Expected analog vector")
    }

    /// Try to get a digital scalar value.
    pub fn try_digital(&self) -> Option<DigitalValue> {
        match self {
            InputValue::Digital(v) => Some(*v),
            _ => None,
        }
    }

    /// Get digital value
    pub fn digital(&self) -> DigitalValue {
        self.try_digital().expect("Expected digital value")
    }

    /// Try to get a digital vector.
    pub fn try_digital_vector(&self) -> Option<&[DigitalValue]> {
        match self {
            InputValue::DigitalVector(v) => Some(v),
            _ => None,
        }
    }

    /// Get digital vector
    pub fn digital_vector(&self) -> &[DigitalValue] {
        self.try_digital_vector().expect("Expected digital vector")
    }
}

/// Output port value container
#[derive(Debug, Clone)]
pub enum OutputValue {
    /// Analog voltage/current to stamp
    Analog(AnalogValue),
    /// Vector of analog outputs
    AnalogVector(Vec<AnalogValue>),
    /// Digital event to schedule
    Digital(DigitalValue),
    /// Vector of digital outputs
    DigitalVector(Vec<DigitalValue>),
}

/// Digital event emitted by a code model output port.
#[derive(Debug, Clone)]
pub(crate) struct PendingDigitalEvent {
    /// Output port name as declared by the code model.
    pub port_name: String,
    /// One or more values emitted by the port.
    pub values: Vec<DigitalValue>,
    /// Delay relative to the current evaluation time.
    pub delay: Value,
}

impl Default for OutputValue {
    fn default() -> Self {
        OutputValue::Analog(AnalogValue::default())
    }
}

impl OutputValue {
    /// Create analog output initialized to zero
    pub fn analog() -> Self {
        OutputValue::Analog(AnalogValue::default())
    }

    /// Create digital output initialized to unknown
    pub fn digital() -> Self {
        OutputValue::Digital(DigitalValue::default())
    }

    /// Set analog output value
    pub fn set_analog(&mut self, value: Value) {
        if let OutputValue::Analog(v) = self {
            v.value = value;
        }
    }

    /// Set analog partial derivative
    pub fn set_partial(&mut self, partial: Value) {
        if let OutputValue::Analog(v) = self {
            v.partial = partial;
        }
    }

    /// Set digital output value
    pub fn set_digital(&mut self, value: DigitalValue) {
        *self = OutputValue::Digital(value);
    }
}

//=============================================================================
// Code Model Context
//=============================================================================

/// Runtime context for code model evaluation
///
/// Provides access to:
/// - Input port values (read)
/// - Output port values (write)
/// - Instance parameters
/// - Simulation state (time, timestep, temperature)
/// - Internal state variables
/// - Event scheduling
#[derive(Debug, Clone)]
pub struct CmContext {
    //-------------------------------------------------------------------------
    // Simulation State
    //-------------------------------------------------------------------------
    /// Current simulation time
    pub time: Value,
    /// Previous simulation time
    pub time_prev: Value,
    /// Current timestep
    pub timestep: Value,
    /// Temperature in Kelvin
    pub temperature: Value,
    /// Type of analysis being performed
    pub analysis: AnalysisType,
    /// Reason for this evaluation call
    pub call_type: CallType,
    /// Current iteration count (for convergence tracking)
    pub iteration: usize,

    //-------------------------------------------------------------------------
    // Port Values
    //-------------------------------------------------------------------------
    /// Input port values by name
    inputs: HashMap<String, InputValue>,
    /// Last event time for scalar digital input ports.
    input_event_times: HashMap<String, Value>,
    /// Output port values by name
    outputs: HashMap<String, OutputValue>,
    /// Connected analog node index per scalar analog port (0 = ground).
    port_nodes: HashMap<String, usize>,
    /// Connected width per port. Scalar ports have width 1.
    port_widths: HashMap<String, usize>,

    //-------------------------------------------------------------------------
    // Parameters
    //-------------------------------------------------------------------------
    /// Instance parameters by name
    params: HashMap<String, Value>,
    /// String parameters (paths, etc.)
    string_params: HashMap<String, String>,
    /// Real-vector parameters by name
    real_vector_params: HashMap<String, Vec<Value>>,
    /// Integer-vector parameters by name
    integer_vector_params: HashMap<String, Vec<i64>>,

    //-------------------------------------------------------------------------
    // Internal State
    //-------------------------------------------------------------------------
    /// State variables (persistent across calls)
    state: Vec<Value>,
    /// Previous state values
    state_prev: Vec<Value>,
    /// Integer state variables
    int_state: Vec<i64>,

    //-------------------------------------------------------------------------
    // Event Scheduling
    //-------------------------------------------------------------------------
    /// Scheduled output events.
    pending_events: Vec<PendingDigitalEvent>,

    //-------------------------------------------------------------------------
    // Matrix Stamping
    //-------------------------------------------------------------------------
    /// Conductance stamps (row, col, value)
    stamps: Vec<(usize, usize, Value)>,
    /// RHS contributions (node, value)
    rhs: Vec<(usize, Value)>,
}

impl Default for CmContext {
    fn default() -> Self {
        Self::new()
    }
}

impl CmContext {
    #[inline]
    fn canonical_param_key(name: &str) -> String {
        name.to_ascii_lowercase()
    }

    /// Create a new empty context
    pub fn new() -> Self {
        Self {
            time: 0.0,
            time_prev: 0.0,
            timestep: 1e-9,
            temperature: 300.15, // 27°C
            analysis: AnalysisType::DcOp,
            call_type: CallType::Init,
            iteration: 0,
            inputs: HashMap::new(),
            input_event_times: HashMap::new(),
            outputs: HashMap::new(),
            port_nodes: HashMap::new(),
            port_widths: HashMap::new(),
            params: HashMap::new(),
            string_params: HashMap::new(),
            real_vector_params: HashMap::new(),
            integer_vector_params: HashMap::new(),
            state: Vec::new(),
            state_prev: Vec::new(),
            int_state: Vec::new(),
            pending_events: Vec::new(),
            stamps: Vec::new(),
            rhs: Vec::new(),
        }
    }

    //-------------------------------------------------------------------------
    // Input Access
    //-------------------------------------------------------------------------

    /// Get analog input value
    pub fn input(&self, name: &str) -> Value {
        self.inputs
            .get(name)
            .map(|v| v.analog_or(0.0))
            .unwrap_or(0.0)
    }

    /// Get analog input with explicit port type
    pub fn input_analog(&self, name: &str) -> Option<Value> {
        self.inputs.get(name).and_then(|v| match v {
            InputValue::Analog(a) => Some(a.value),
            _ => None,
        })
    }

    /// Get digital input value
    pub fn input_digital(&self, name: &str) -> Option<DigitalValue> {
        self.inputs.get(name).and_then(|v| match v {
            InputValue::Digital(d) => Some(*d),
            _ => None,
        })
    }

    /// Get the last event time for a scalar digital input.
    pub fn input_digital_event_time(&self, name: &str) -> Option<Value> {
        self.input_event_times.get(name).copied()
    }

    /// Get analog input vector
    pub fn input_vector(&self, name: &str) -> Vec<Value> {
        self.inputs
            .get(name)
            .map(|v| match v {
                InputValue::AnalogVector(vec) => vec.iter().map(|a| a.value).collect(),
                _ => Vec::new(),
            })
            .unwrap_or_default()
    }

    /// Get digital input vector
    pub fn input_digital_vector(&self, name: &str) -> Vec<DigitalValue> {
        self.inputs
            .get(name)
            .map(|v| match v {
                InputValue::DigitalVector(vec) => vec.clone(),
                _ => Vec::new(),
            })
            .unwrap_or_default()
    }

    /// Set an input value (used by circuit integration)
    pub fn set_input(&mut self, name: &str, value: InputValue) {
        self.inputs.insert(name.to_string(), value);
    }

    /// Set analog input by name
    pub fn set_input_analog(&mut self, name: &str, value: Value) {
        self.inputs.insert(
            name.to_string(),
            InputValue::Analog(AnalogValue::new(value)),
        );
    }

    /// Set digital input by name
    pub fn set_input_digital(&mut self, name: &str, value: DigitalValue) {
        self.inputs
            .insert(name.to_string(), InputValue::Digital(value));
    }

    /// Set last event time for a scalar digital input.
    pub fn set_input_digital_event_time(&mut self, name: &str, time: Value) {
        self.input_event_times.insert(name.to_string(), time);
    }

    //-------------------------------------------------------------------------
    // Output Access
    //-------------------------------------------------------------------------

    /// Get analog output value
    pub fn output(&self, name: &str) -> Value {
        self.outputs
            .get(name)
            .map(|v| match v {
                OutputValue::Analog(a) => a.value,
                _ => 0.0,
            })
            .unwrap_or(0.0)
    }

    /// Get previous analog output value
    pub fn output_prev(&self, name: &str) -> Value {
        self.outputs
            .get(name)
            .map(|v| match v {
                OutputValue::Analog(a) => a.prev_value,
                _ => 0.0,
            })
            .unwrap_or(0.0)
    }

    /// Get analog output partial derivative
    pub fn partial(&self, name: &str) -> Value {
        self.outputs
            .get(name)
            .map(|v| match v {
                OutputValue::Analog(a) => a.partial,
                _ => 0.0,
            })
            .unwrap_or(0.0)
    }

    /// Set analog output value
    pub fn set_output(&mut self, name: &str, value: Value) {
        match self.outputs.get_mut(name) {
            Some(OutputValue::Analog(a)) => {
                a.prev_value = a.value;
                a.value = value;
            }
            Some(_) => {}
            None => {
                self.outputs.insert(
                    name.to_string(),
                    OutputValue::Analog(AnalogValue::new(value)),
                );
            }
        }
    }

    /// Set analog output with partial derivative
    pub fn set_output_with_partial(&mut self, name: &str, value: Value, partial: Value) {
        let out = self
            .outputs
            .entry(name.to_string())
            .or_insert_with(OutputValue::analog);
        if let OutputValue::Analog(a) = out {
            a.prev_value = a.value;
            a.value = value;
            a.partial = partial;
        }
    }

    /// Set digital output value (schedules event)
    pub fn set_output_digital(&mut self, name: &str, value: DigitalValue, delay: Value) {
        self.outputs
            .insert(name.to_string(), OutputValue::Digital(value));
        self.pending_events.push(PendingDigitalEvent {
            port_name: name.to_string(),
            values: vec![value],
            delay,
        });
    }

    /// Set digital vector output value (schedules one event per connected bit).
    pub fn set_output_digital_vector(
        &mut self,
        name: &str,
        values: Vec<DigitalValue>,
        delay: Value,
    ) {
        self.outputs
            .insert(name.to_string(), OutputValue::DigitalVector(values.clone()));
        self.pending_events.push(PendingDigitalEvent {
            port_name: name.to_string(),
            values,
            delay,
        });
    }

    /// Initialize output ports
    pub fn init_output(&mut self, name: &str, port_type: PortType) {
        match port_type {
            PortType::Voltage | PortType::Current | PortType::DifferentialVoltage => {
                self.outputs.insert(name.to_string(), OutputValue::analog());
            }
            PortType::Digital => {
                self.outputs
                    .insert(name.to_string(), OutputValue::digital());
            }
            _ => {}
        }
    }

    /// Register connected node for an analog scalar port (0 = ground).
    pub fn set_port_node(&mut self, name: &str, node: usize) {
        self.port_nodes.insert(name.to_string(), node);
    }

    /// Get connected node for an analog scalar port (0 = ground).
    pub fn port_node(&self, name: &str) -> Option<usize> {
        self.port_nodes.get(name).copied()
    }

    /// Register connected width for a port.
    pub fn set_port_width(&mut self, name: &str, width: usize) {
        self.port_widths.insert(name.to_string(), width);
    }

    /// Get connected width for a port, defaulting to scalar.
    pub fn port_width(&self, name: &str) -> usize {
        self.port_widths.get(name).copied().unwrap_or(1)
    }

    /// Clear port-node mapping for current evaluation pass.
    pub fn clear_port_nodes(&mut self) {
        self.port_nodes.clear();
    }

    /// Get all pending events and clear the queue
    pub(crate) fn take_pending_events(&mut self) -> Vec<PendingDigitalEvent> {
        std::mem::take(&mut self.pending_events)
    }

    //-------------------------------------------------------------------------
    // Parameter Access
    //-------------------------------------------------------------------------

    /// Get parameter value
    pub fn param(&self, name: &str) -> Value {
        self.params
            .get(&Self::canonical_param_key(name))
            .copied()
            .unwrap_or(0.0)
    }

    /// Get parameter with default
    pub fn param_or(&self, name: &str, default: Value) -> Value {
        self.params
            .get(&Self::canonical_param_key(name))
            .copied()
            .unwrap_or(default)
    }

    /// Get string parameter
    pub fn string_param(&self, name: &str) -> Option<&str> {
        self.string_params
            .get(&Self::canonical_param_key(name))
            .map(|s| s.as_str())
    }

    /// Get real-vector parameter
    pub fn real_vector_param(&self, name: &str) -> Option<&[Value]> {
        self.real_vector_params
            .get(&Self::canonical_param_key(name))
            .map(|values| values.as_slice())
    }

    /// Get integer-vector parameter
    pub fn integer_vector_param(&self, name: &str) -> Option<&[i64]> {
        self.integer_vector_params
            .get(&Self::canonical_param_key(name))
            .map(|values| values.as_slice())
    }

    /// Set parameter value
    pub fn set_param(&mut self, name: &str, value: Value) {
        self.params.insert(Self::canonical_param_key(name), value);
    }

    /// Set string parameter
    pub fn set_string_param(&mut self, name: &str, value: &str) {
        self.string_params
            .insert(Self::canonical_param_key(name), value.to_string());
    }

    /// Set real-vector parameter
    pub fn set_real_vector_param(&mut self, name: &str, value: Vec<Value>) {
        self.real_vector_params
            .insert(Self::canonical_param_key(name), value);
    }

    /// Set integer-vector parameter
    pub fn set_integer_vector_param(&mut self, name: &str, value: Vec<i64>) {
        self.integer_vector_params
            .insert(Self::canonical_param_key(name), value);
    }

    //-------------------------------------------------------------------------
    // State Variable Access
    //-------------------------------------------------------------------------

    /// Allocate state variables
    pub fn allocate_states(&mut self, count: usize) {
        self.state.resize(count, 0.0);
        self.state_prev.resize(count, 0.0);
    }

    /// Allocate integer state variables
    pub fn allocate_int_states(&mut self, count: usize) {
        self.int_state.resize(count, 0);
    }

    /// Get state variable
    pub fn state(&self, index: usize) -> Value {
        self.state.get(index).copied().unwrap_or(0.0)
    }

    /// Get previous state variable
    pub fn state_prev(&self, index: usize) -> Value {
        self.state_prev.get(index).copied().unwrap_or(0.0)
    }

    /// Set state variable
    pub fn set_state(&mut self, index: usize, value: Value) {
        if index < self.state.len() {
            self.state[index] = value;
        }
    }

    /// Set both current and accepted state during model initialization.
    pub fn set_initial_state(&mut self, index: usize, value: Value) {
        if index < self.state.len() {
            self.state[index] = value;
        }
        if index < self.state_prev.len() {
            self.state_prev[index] = value;
        }
    }

    /// Get integer state variable
    pub fn int_state(&self, index: usize) -> i64 {
        self.int_state.get(index).copied().unwrap_or(0)
    }

    /// Set integer state variable
    pub fn set_int_state(&mut self, index: usize, value: i64) {
        if index < self.int_state.len() {
            self.int_state[index] = value;
        }
    }

    /// Advance state for new timestep
    pub fn advance_state(&mut self) {
        self.state_prev.clone_from(&self.state);
        for output in self.outputs.values_mut() {
            if let OutputValue::Analog(value) = output {
                value.prev_value = value.value;
            }
        }
        self.time_prev = self.time;
    }

    //-------------------------------------------------------------------------
    // Matrix Stamping
    //-------------------------------------------------------------------------

    /// Add a conductance stamp
    pub fn stamp_conductance(&mut self, row: usize, col: usize, value: Value) {
        self.stamps.push((row, col, value));
    }

    /// Add RHS contribution
    pub fn stamp_rhs(&mut self, node: usize, value: Value) {
        self.rhs.push((node, value));
    }

    /// Get all conductance stamps and clear
    pub fn take_stamps(&mut self) -> Vec<(usize, usize, Value)> {
        std::mem::take(&mut self.stamps)
    }

    /// Get all RHS contributions and clear
    pub fn take_rhs(&mut self) -> Vec<(usize, Value)> {
        std::mem::take(&mut self.rhs)
    }

    /// Clear queued matrix and RHS contributions.
    pub fn clear_stamps(&mut self) {
        self.stamps.clear();
        self.rhs.clear();
    }

    //-------------------------------------------------------------------------
    // Convenience Methods
    //-------------------------------------------------------------------------

    /// Get thermal voltage (kT/q)
    pub fn thermal_voltage(&self) -> Value {
        const BOLTZMANN: Value = 1.380649e-23;
        const CHARGE: Value = 1.602176634e-19;
        BOLTZMANN * self.temperature / CHARGE
    }

    /// Check if this is the first call (initialization)
    pub fn is_init(&self) -> bool {
        self.call_type == CallType::Init
    }

    /// Check if this is DC analysis
    pub fn is_dc(&self) -> bool {
        matches!(self.analysis, AnalysisType::DcOp | AnalysisType::DcSweep)
    }

    /// Check if this is transient analysis
    pub fn is_transient(&self) -> bool {
        self.analysis == AnalysisType::Transient
    }

    /// Check if this is AC analysis
    pub fn is_ac(&self) -> bool {
        self.analysis == AnalysisType::Ac
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::{DigitalState, DigitalStrength};

    #[test]
    fn scalar_analog_input_accessor_defaults_for_non_analog_ports() {
        let mut ctx = CmContext::new();
        ctx.set_input_digital(
            "in",
            DigitalValue::new(DigitalState::One, DigitalStrength::Strong),
        );

        assert_eq!(ctx.input("in"), 0.0);
        assert_eq!(ctx.input_analog("in"), None);
    }

    #[test]
    fn input_value_try_accessors_return_none_for_wrong_type() {
        let digital = InputValue::Digital(DigitalValue::new(
            DigitalState::One,
            DigitalStrength::Strong,
        ));
        assert_eq!(digital.try_analog(), None);
        assert!(digital.try_analog_vector().is_none());

        let analog = InputValue::Analog(AnalogValue::new(1.25));
        assert_eq!(analog.try_digital(), None);
        assert!(analog.try_digital_vector().is_none());
    }

    #[test]
    fn input_value_try_accessors_return_matching_values() {
        let analog = InputValue::Analog(AnalogValue::new(1.25));
        assert_eq!(analog.try_analog(), Some(1.25));

        let digital_value = DigitalValue::new(DigitalState::One, DigitalStrength::Strong);
        let digital = InputValue::Digital(digital_value);
        assert_eq!(digital.try_digital(), Some(digital_value));
    }
}
