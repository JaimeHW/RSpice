//! XSPICE Code Model Execution Context
//!
//! Provides the runtime context passed to code models during evaluation.
//! Handles port value access, parameter lookup, and state management.

use super::{CmError, CmResult, DigitalValue, PortType};
use crate::{Complex64, Value};
use std::any::Any;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;
use std::vec::Drain;

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

/// Rollback/commit phase for a code-model evaluation.
///
/// Most built-in models are pure and do not need this distinction. External
/// co-simulation models need it to avoid mutating irreversible host state
/// during Newton trial evaluations that may be rolled back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvaluationPhase {
    /// Ordinary evaluation without transient rollback semantics.
    DirectEvaluation,
    /// A rollbackable trial evaluation used for residual/Jacobian probing.
    RollbackableProbe,
    /// Evaluation for an accepted transient timepoint.
    AcceptedStep,
}

impl Default for EvaluationPhase {
    fn default() -> Self {
        Self::DirectEvaluation
    }
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
    /// Real-valued event node
    Real(Value),
    /// Vector of real-valued event nodes
    RealVector(Vec<Value>),
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

    /// Try to get a real scalar value.
    pub fn try_real(&self) -> Option<Value> {
        match self {
            InputValue::Real(v) => Some(*v),
            _ => None,
        }
    }

    /// Get real value
    pub fn real(&self) -> Value {
        self.try_real().expect("Expected real value")
    }

    /// Try to get a real vector.
    pub fn try_real_vector(&self) -> Option<&[Value]> {
        match self {
            InputValue::RealVector(v) => Some(v),
            _ => None,
        }
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
    /// Real-valued event to schedule
    Real(Value),
    /// Vector of real-valued event outputs
    RealVector(Vec<Value>),
}

/// Digital event emitted by a code model output port.
#[derive(Debug, Clone)]
pub(crate) struct PendingDigitalEvent {
    /// Output port name as declared by the code model.
    pub port_name: String,
    /// First vector element index targeted by this event.
    pub start_index: usize,
    /// One or more values emitted by the port.
    pub values: Vec<DigitalValue>,
    /// Delay relative to the current evaluation time.
    pub delay: Value,
}

/// Real-valued event emitted by a code model output port.
#[derive(Debug, Clone)]
pub(crate) struct PendingRealEvent {
    /// Output port name as declared by the code model.
    pub port_name: String,
    /// First vector element index targeted by this event.
    pub start_index: usize,
    /// One or more values emitted by the port.
    pub values: Vec<Value>,
    /// Delay relative to the current evaluation time.
    pub delay: Value,
}

/// Per-output state used by official XSPICE inertial digital delays.
#[derive(Debug, Clone, Copy)]
struct InertialOutputState {
    /// Absolute time of the pending transition, or a negative value when idle.
    when: Value,
    /// Output value before the pending transition started.
    prev: DigitalValue,
}

/// Time-domain sample history owned by one code-model instance.
#[derive(Debug, Clone)]
struct TransientHistorySample {
    time: Value,
    values: Vec<Value>,
}

impl Default for OutputValue {
    fn default() -> Self {
        OutputValue::Analog(AnalogValue::default())
    }
}

fn same_transient_time(a: Value, b: Value) -> bool {
    let scale = a.abs().max(b.abs()).max(1.0);
    (a - b).abs() <= f64::EPSILON * scale
}

fn prune_transient_history(
    history: &mut Vec<TransientHistorySample>,
    time: Value,
    retention_window: Value,
) {
    if !retention_window.is_finite() || retention_window < 0.0 {
        return;
    }
    let oldest_kept = time - retention_window;
    let prune_count = history.partition_point(|sample| {
        sample.time < oldest_kept && !same_transient_time(sample.time, oldest_kept)
    });
    if prune_count > 0 {
        history.drain(0..prune_count);
    }
}

fn context_allocation_error(
    context: &str,
    len: usize,
    err: std::collections::TryReserveError,
) -> CmError {
    CmError::EvaluationError(format!(
        "{context}: unable to reserve {len} value(s): {err}"
    ))
}

fn refill_vec_from_fn<T, F>(
    context: &str,
    values: &mut Vec<T>,
    value_count: usize,
    mut value_at: F,
) -> CmResult<()>
where
    F: FnMut(usize) -> T,
{
    if values.len() > value_count {
        values.truncate(value_count);
    }

    let existing_len = values.len();
    for (index, value) in values.iter_mut().enumerate() {
        *value = value_at(index);
    }

    if existing_len < value_count {
        if values.capacity() < value_count {
            let additional = value_count - values.capacity();
            values
                .try_reserve_exact(additional)
                .map_err(|err| context_allocation_error(context, value_count, err))?;
        }
        for index in existing_len..value_count {
            values.push(value_at(index));
        }
    }
    Ok(())
}

fn vec_from_fn<T, F>(context: &str, value_count: usize, mut value_at: F) -> CmResult<Vec<T>>
where
    F: FnMut(usize) -> T,
{
    let mut values = Vec::new();
    values
        .try_reserve_exact(value_count)
        .map_err(|err| context_allocation_error(context, value_count, err))?;
    for index in 0..value_count {
        values.push(value_at(index));
    }
    Ok(values)
}

fn analog_vector_values(context: &str, width: usize) -> CmResult<Vec<AnalogValue>> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(width)
        .map_err(|err| context_allocation_error(context, width, err))?;
    values.resize(width, AnalogValue::default());
    Ok(values)
}

fn resize_analog_vector_values(
    context: &str,
    values: &mut Vec<AnalogValue>,
    width: usize,
) -> CmResult<()> {
    if values.capacity() < width {
        let additional = width - values.capacity();
        values
            .try_reserve_exact(additional)
            .map_err(|err| context_allocation_error(context, width, err))?;
    }
    values.resize(width, AnalogValue::default());
    Ok(())
}

impl OutputValue {
    /// Create analog output initialized to zero
    pub fn analog() -> Self {
        OutputValue::Analog(AnalogValue::default())
    }

    /// Create analog vector output initialized to zero.
    pub fn analog_vector(width: usize) -> Self {
        OutputValue::AnalogVector(vec![AnalogValue::default(); width])
    }

    /// Create digital output initialized to unknown
    pub fn digital() -> Self {
        OutputValue::Digital(DigitalValue::default())
    }

    /// Create real output initialized to zero
    pub fn real() -> Self {
        OutputValue::Real(0.0)
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

    /// Set real output value
    pub fn set_real(&mut self, value: Value) {
        *self = OutputValue::Real(value);
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
#[derive(Clone, Default)]
struct ContextResources {
    values: HashMap<String, Arc<dyn Any + Send + Sync>>,
}

impl fmt::Debug for ContextResources {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContextResources")
            .field("keys", &self.values.keys().collect::<Vec<_>>())
            .finish()
    }
}

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
    /// Transient print-step/max-step hint for code models that need run context.
    transient_step_hint: Option<Value>,
    /// Final transient stop time for code models that need circuit run context.
    transient_stop_time: Option<Value>,
    /// Temperature in Kelvin
    pub temperature: Value,
    /// ngspice-compatible transient ramp time for analog code models.
    ramptime: Value,
    /// ngspice XSPICE digital delay policy.
    digital_delay_type: Option<i64>,
    /// Type of analysis being performed
    pub analysis: AnalysisType,
    /// Reason for this evaluation call
    pub call_type: CallType,
    /// Rollback/commit phase for this evaluation call.
    evaluation_phase: EvaluationPhase,
    /// Current iteration count (for convergence tracking)
    pub iteration: usize,
    /// Resource policy inherited from the owning simulation engine.
    resource_limits: crate::resource::ResourceLimits,

    //-------------------------------------------------------------------------
    // Port Values
    //-------------------------------------------------------------------------
    /// Input port values by name
    inputs: HashMap<String, InputValue>,
    /// Last event time for scalar digital input ports.
    input_event_times: HashMap<String, Value>,
    /// Last event time for vector digital input ports, per element.
    input_vector_event_times: HashMap<String, Vec<Option<Value>>>,
    /// ngspice-style total event-node load per scalar event port.
    port_total_loads: HashMap<String, Value>,
    /// ngspice-style total event-node load per vector event port element.
    port_vector_total_loads: HashMap<String, Vec<Value>>,
    /// Output port values by name
    outputs: HashMap<String, OutputValue>,
    /// Connected analog node index per scalar analog port (0 = ground).
    port_nodes: HashMap<String, usize>,
    /// Connected terminal pair per scalar or differential analog port.
    port_terminals: HashMap<String, (usize, usize)>,
    /// Connected terminal pairs per analog vector port element.
    port_vector_terminals: HashMap<String, Vec<(usize, usize)>>,
    /// MNA matrix column for scalar branch-current control ports.
    port_control_columns: HashMap<String, usize>,
    /// Connected width per port. Scalar ports have width 1.
    port_widths: HashMap<String, usize>,

    //-------------------------------------------------------------------------
    // Parameters
    //-------------------------------------------------------------------------
    /// Instance parameters by name
    params: HashMap<String, Value>,
    /// Complex parameters by name
    complex_params: HashMap<String, Complex64>,
    /// String parameters (paths, etc.)
    string_params: HashMap<String, String>,
    /// Monotonic revisions for string parameters by canonical name.
    string_param_revisions: HashMap<String, u64>,
    /// Next revision assigned by `set_string_param`.
    next_string_param_revision: u64,
    /// String-vector parameters by name
    string_vector_params: HashMap<String, Vec<String>>,
    /// Complex-vector parameters by name
    complex_vector_params: HashMap<String, Vec<Complex64>>,
    /// Real-vector parameters by name
    real_vector_params: HashMap<String, Vec<Value>>,
    /// Monotonic revisions for real-vector parameters by canonical name.
    real_vector_param_revisions: HashMap<String, u64>,
    /// Next revision assigned by `set_real_vector_param`.
    next_real_vector_param_revision: u64,
    /// Integer-vector parameters by name
    integer_vector_params: HashMap<String, Vec<i64>>,
    /// Parameters explicitly supplied by the instance or model card.
    provided_params: HashSet<String>,

    //-------------------------------------------------------------------------
    // Internal State
    //-------------------------------------------------------------------------
    /// State variables (persistent across calls)
    state: Vec<Value>,
    /// Previous state values
    state_prev: Vec<Value>,
    /// Integer state variables
    int_state: Vec<i64>,
    /// Per-instance transient sample histories keyed by model-defined names.
    transient_histories: HashMap<String, Vec<TransientHistorySample>>,
    /// Host/runtime resources owned by the model instance.
    resources: ContextResources,

    //-------------------------------------------------------------------------
    // Event Scheduling
    //-------------------------------------------------------------------------
    /// Scheduled output events.
    pending_events: Vec<PendingDigitalEvent>,
    /// Scheduled real-valued output events.
    pending_real_events: Vec<PendingRealEvent>,
    /// Per-output inertial-delay state for digital code models.
    inertial_outputs: HashMap<String, InertialOutputState>,
    /// Absolute transient times requested by analog code models.
    requested_breakpoints: Vec<Value>,

    //-------------------------------------------------------------------------
    // Matrix Stamping
    //-------------------------------------------------------------------------
    /// Conductance stamps (row, col, value)
    stamps: Vec<(usize, usize, Value)>,
    /// RHS contributions (node, value)
    rhs: Vec<(usize, Value)>,
}

/// Serializable code-model context state captured in transient checkpoints.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CmContextCheckpoint {
    pub time: Value,
    pub time_prev: Value,
    pub state: Vec<Value>,
    pub state_prev: Vec<Value>,
    pub int_state: Vec<i64>,
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

    #[inline]
    fn canonical_param_lookup_key(name: &str) -> Cow<'_, str> {
        if name.bytes().any(|byte| byte.is_ascii_uppercase()) {
            Cow::Owned(Self::canonical_param_key(name))
        } else {
            Cow::Borrowed(name)
        }
    }

    /// Create a new empty context
    pub fn new() -> Self {
        Self {
            time: 0.0,
            time_prev: 0.0,
            timestep: 1e-9,
            transient_step_hint: None,
            transient_stop_time: None,
            temperature: 300.15, // 27°C
            ramptime: 0.0,
            digital_delay_type: None,
            analysis: AnalysisType::DcOp,
            call_type: CallType::Init,
            evaluation_phase: EvaluationPhase::DirectEvaluation,
            iteration: 0,
            resource_limits: crate::resource::ResourceLimits::default(),
            inputs: HashMap::new(),
            input_event_times: HashMap::new(),
            input_vector_event_times: HashMap::new(),
            port_total_loads: HashMap::new(),
            port_vector_total_loads: HashMap::new(),
            outputs: HashMap::new(),
            port_nodes: HashMap::new(),
            port_terminals: HashMap::new(),
            port_vector_terminals: HashMap::new(),
            port_control_columns: HashMap::new(),
            port_widths: HashMap::new(),
            params: HashMap::new(),
            complex_params: HashMap::new(),
            string_params: HashMap::new(),
            string_param_revisions: HashMap::new(),
            next_string_param_revision: 1,
            string_vector_params: HashMap::new(),
            complex_vector_params: HashMap::new(),
            real_vector_params: HashMap::new(),
            real_vector_param_revisions: HashMap::new(),
            next_real_vector_param_revision: 1,
            integer_vector_params: HashMap::new(),
            provided_params: HashSet::new(),
            state: Vec::new(),
            state_prev: Vec::new(),
            int_state: Vec::new(),
            transient_histories: HashMap::new(),
            resources: ContextResources::default(),
            pending_events: Vec::new(),
            pending_real_events: Vec::new(),
            inertial_outputs: HashMap::new(),
            requested_breakpoints: Vec::new(),
            stamps: Vec::new(),
            rhs: Vec::new(),
        }
    }

    /// Return the rollback/commit phase for the current model evaluation.
    pub fn evaluation_phase(&self) -> EvaluationPhase {
        self.evaluation_phase
    }

    /// Resource policy for file-backed and allocation-heavy code models.
    pub fn resource_limits(&self) -> crate::resource::ResourceLimits {
        self.resource_limits
    }

    /// Apply the owning engine's resource policy to this model context.
    pub(crate) fn set_resource_limits(&mut self, resource_limits: crate::resource::ResourceLimits) {
        self.resource_limits = resource_limits;
    }

    /// Final transient stop time, when evaluation is running inside `.tran`.
    pub fn transient_stop_time(&self) -> Option<Value> {
        self.transient_stop_time
    }

    /// Transient print-step/max-step hint, when evaluation is running inside `.tran`.
    pub fn transient_step_hint(&self) -> Option<Value> {
        self.transient_step_hint
    }

    /// Set transient run context for models with ngspice run-context defaults.
    pub(crate) fn set_transient_run_context(&mut self, tstep: Option<Value>, tstop: Option<Value>) {
        self.transient_step_hint = tstep.filter(|value| value.is_finite() && *value > 0.0);
        self.transient_stop_time = tstop.filter(|value| value.is_finite() && *value >= 0.0);
    }

    /// Set ngspice-compatible transient ramp time in seconds.
    pub fn set_ramptime(&mut self, ramptime: Value) {
        self.ramptime = ramptime;
    }

    /// Transient ramp time in seconds.
    pub fn ramptime(&self) -> Value {
        self.ramptime
    }

    /// Set ngspice-compatible XSPICE digital delay policy.
    pub fn set_digital_delay_type(&mut self, digital_delay_type: Option<i64>) {
        self.digital_delay_type = digital_delay_type;
    }

    /// XSPICE digital delay policy, when configured.
    pub fn digital_delay_type(&self) -> Option<i64> {
        self.digital_delay_type
    }

    /// ngspice `cm_analog_ramp_factor()` semantics for XSPICE analog models.
    pub fn analog_ramp_factor(&self) -> Value {
        if !self.is_transient() || !self.ramptime.is_finite() || self.ramptime <= 0.0 {
            return 1.0;
        }
        if self.time >= self.ramptime {
            return 1.0;
        }
        self.time / self.ramptime
    }

    /// Set the rollback/commit phase for the next model evaluation.
    pub(crate) fn set_evaluation_phase(&mut self, phase: EvaluationPhase) {
        self.evaluation_phase = phase;
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

    /// Get the last event time for one element of a digital input vector.
    pub fn input_digital_vector_event_time(&self, name: &str, index: usize) -> Option<Value> {
        self.input_vector_event_times
            .get(name)
            .and_then(|times| times.get(index).copied().flatten())
    }

    /// Get real input value
    pub fn input_real(&self, name: &str) -> Option<Value> {
        self.inputs.get(name).and_then(|v| match v {
            InputValue::Real(value) => Some(*value),
            _ => None,
        })
    }

    /// Get the last event time for a scalar real input.
    pub fn input_real_event_time(&self, name: &str) -> Option<Value> {
        self.input_event_times.get(name).copied()
    }

    /// ngspice-compatible `TOTAL_LOAD(port)` value for a scalar event port.
    pub fn port_total_load(&self, name: &str) -> Value {
        self.port_total_loads.get(name).copied().unwrap_or(0.0)
    }

    /// ngspice-compatible `TOTAL_LOAD(port[index])` value for an event vector port.
    pub fn port_vector_total_load(&self, name: &str, index: usize) -> Value {
        self.port_vector_total_loads
            .get(name)
            .and_then(|loads| loads.get(index).copied())
            .unwrap_or(0.0)
    }

    /// Borrow all per-element total loads for an event vector port.
    pub fn port_vector_total_loads(&self, name: &str) -> Option<&[Value]> {
        self.port_vector_total_loads.get(name).map(Vec::as_slice)
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

    /// Borrow an analog input vector without allocating.
    pub fn input_analog_vector_values(&self, name: &str) -> Option<&[AnalogValue]> {
        self.inputs
            .get(name)
            .and_then(InputValue::try_analog_vector)
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

    /// Borrow a digital input vector without allocating.
    pub fn input_digital_vector_values(&self, name: &str) -> Option<&[DigitalValue]> {
        self.inputs
            .get(name)
            .and_then(InputValue::try_digital_vector)
    }

    /// Get real input vector
    pub fn input_real_vector(&self, name: &str) -> Vec<Value> {
        self.inputs
            .get(name)
            .map(|v| match v {
                InputValue::RealVector(vec) => vec.clone(),
                _ => Vec::new(),
            })
            .unwrap_or_default()
    }

    /// Borrow a real input vector without allocating.
    pub fn input_real_vector_values(&self, name: &str) -> Option<&[Value]> {
        self.inputs.get(name).and_then(InputValue::try_real_vector)
    }

    /// Set an input value (used by circuit integration)
    pub fn set_input(&mut self, name: &str, value: InputValue) {
        match self.inputs.get_mut(name) {
            Some(existing) => *existing = value,
            None => {
                self.inputs.insert(name.to_string(), value);
            }
        }
    }

    /// Set analog input by name
    pub fn set_input_analog(&mut self, name: &str, value: Value) {
        let value = AnalogValue::new(value);
        match self.inputs.get_mut(name) {
            Some(InputValue::Analog(existing)) => *existing = value,
            Some(existing) => *existing = InputValue::Analog(value),
            None => {
                self.inputs
                    .insert(name.to_string(), InputValue::Analog(value));
            }
        }
    }

    /// Set analog vector input values by name.
    pub fn set_input_analog_vector(&mut self, name: &str, values: &[Value]) -> CmResult<()> {
        self.set_input_analog_vector_from_fn(name, values.len(), |index| {
            AnalogValue::new(values[index])
        })
    }

    /// Set digital input by name
    pub fn set_input_digital(&mut self, name: &str, value: DigitalValue) {
        match self.inputs.get_mut(name) {
            Some(InputValue::Digital(existing)) => *existing = value,
            Some(existing) => *existing = InputValue::Digital(value),
            None => {
                self.inputs
                    .insert(name.to_string(), InputValue::Digital(value));
            }
        }
    }

    /// Set real input by name
    pub fn set_input_real(&mut self, name: &str, value: Value) {
        match self.inputs.get_mut(name) {
            Some(InputValue::Real(existing)) => *existing = value,
            Some(existing) => *existing = InputValue::Real(value),
            None => {
                self.inputs
                    .insert(name.to_string(), InputValue::Real(value));
            }
        }
    }

    /// Set ngspice-style total event-node load for a scalar port.
    pub(crate) fn set_port_total_load(&mut self, name: &str, value: Value) {
        match self.port_total_loads.get_mut(name) {
            Some(existing) => *existing = value,
            None => {
                self.port_total_loads.insert(name.to_string(), value);
            }
        }
    }

    /// Set ngspice-style total event-node loads for a vector port.
    pub(crate) fn set_port_vector_total_loads_from_fn<F>(
        &mut self,
        name: &str,
        value_count: usize,
        value_at: F,
    ) -> CmResult<()>
    where
        F: FnMut(usize) -> Value,
    {
        match self.port_vector_total_loads.get_mut(name) {
            Some(values) => {
                refill_vec_from_fn("port vector total loads", values, value_count, value_at)?;
            }
            None => {
                let values = vec_from_fn("port vector total loads", value_count, value_at)?;
                self.port_vector_total_loads
                    .insert(name.to_string(), values);
            }
        }
        Ok(())
    }

    /// Set analog vector input values while reusing an existing vector buffer.
    pub(crate) fn set_input_analog_vector_from_fn<F>(
        &mut self,
        name: &str,
        value_count: usize,
        value_at: F,
    ) -> CmResult<()>
    where
        F: FnMut(usize) -> AnalogValue,
    {
        match self.inputs.get_mut(name) {
            Some(InputValue::AnalogVector(values)) => {
                refill_vec_from_fn("analog vector input", values, value_count, value_at)?;
            }
            Some(value) => {
                let values = vec_from_fn("analog vector input", value_count, value_at)?;
                *value = InputValue::AnalogVector(values);
            }
            None => {
                let values = vec_from_fn("analog vector input", value_count, value_at)?;
                self.inputs
                    .insert(name.to_string(), InputValue::AnalogVector(values));
            }
        }
        Ok(())
    }

    /// Set digital vector input values while reusing an existing vector buffer.
    pub(crate) fn set_input_digital_vector_from_fn<F>(
        &mut self,
        name: &str,
        value_count: usize,
        value_at: F,
    ) -> CmResult<()>
    where
        F: FnMut(usize) -> DigitalValue,
    {
        match self.inputs.get_mut(name) {
            Some(InputValue::DigitalVector(values)) => {
                refill_vec_from_fn("digital vector input", values, value_count, value_at)?;
            }
            Some(value) => {
                let values = vec_from_fn("digital vector input", value_count, value_at)?;
                *value = InputValue::DigitalVector(values);
            }
            None => {
                let values = vec_from_fn("digital vector input", value_count, value_at)?;
                self.inputs
                    .insert(name.to_string(), InputValue::DigitalVector(values));
            }
        }
        Ok(())
    }

    /// Set real vector input values while reusing an existing vector buffer.
    pub(crate) fn set_input_real_vector_from_fn<F>(
        &mut self,
        name: &str,
        value_count: usize,
        value_at: F,
    ) -> CmResult<()>
    where
        F: FnMut(usize) -> Value,
    {
        match self.inputs.get_mut(name) {
            Some(InputValue::RealVector(values)) => {
                refill_vec_from_fn("real vector input", values, value_count, value_at)?;
            }
            Some(value) => {
                let values = vec_from_fn("real vector input", value_count, value_at)?;
                *value = InputValue::RealVector(values);
            }
            None => {
                let values = vec_from_fn("real vector input", value_count, value_at)?;
                self.inputs
                    .insert(name.to_string(), InputValue::RealVector(values));
            }
        }
        Ok(())
    }

    /// Set last event time for a scalar digital input.
    pub fn set_input_digital_event_time(&mut self, name: &str, time: Value) {
        match self.input_event_times.get_mut(name) {
            Some(existing) => *existing = time,
            None => {
                self.input_event_times.insert(name.to_string(), time);
            }
        }
    }

    /// Set per-element event times for a digital input vector.
    pub fn set_input_digital_vector_event_times(&mut self, name: &str, times: Vec<Option<Value>>) {
        match self.input_vector_event_times.get_mut(name) {
            Some(existing) => {
                existing.clear();
                existing.extend(times);
            }
            None => {
                self.input_vector_event_times
                    .insert(name.to_string(), times);
            }
        }
    }

    /// Set per-element event times while reusing an existing vector buffer.
    pub(crate) fn set_input_digital_vector_event_times_from_fn<F>(
        &mut self,
        name: &str,
        value_count: usize,
        time_at: F,
    ) -> CmResult<()>
    where
        F: FnMut(usize) -> Option<Value>,
    {
        match self.input_vector_event_times.get_mut(name) {
            Some(times) => {
                refill_vec_from_fn("digital vector event times", times, value_count, time_at)?;
            }
            None => {
                let times = vec_from_fn("digital vector event times", value_count, time_at)?;
                self.input_vector_event_times
                    .insert(name.to_string(), times);
            }
        }
        Ok(())
    }

    /// Set last event time for a scalar real input.
    pub fn set_input_real_event_time(&mut self, name: &str, time: Value) {
        match self.input_event_times.get_mut(name) {
            Some(existing) => *existing = time,
            None => {
                self.input_event_times.insert(name.to_string(), time);
            }
        }
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

    /// Get analog vector output values.
    pub fn output_vector(&self, name: &str) -> Vec<Value> {
        self.outputs
            .get(name)
            .map(|v| match v {
                OutputValue::AnalogVector(values) => {
                    values.iter().map(|value| value.value).collect()
                }
                OutputValue::Analog(value) => vec![value.value],
                _ => Vec::new(),
            })
            .unwrap_or_default()
    }

    /// Get one analog vector output value without cloning the full vector.
    pub fn output_vector_value(&self, name: &str, index: usize) -> Value {
        match self.outputs.get(name) {
            Some(OutputValue::AnalogVector(values)) => {
                values.get(index).map(|value| value.value).unwrap_or(0.0)
            }
            Some(OutputValue::Analog(value)) if index == 0 => value.value,
            _ => 0.0,
        }
    }

    /// Get previous analog vector output values.
    pub fn output_vector_prev(&self, name: &str) -> Vec<Value> {
        self.outputs
            .get(name)
            .map(|v| match v {
                OutputValue::AnalogVector(values) => {
                    values.iter().map(|value| value.prev_value).collect()
                }
                OutputValue::Analog(value) => vec![value.prev_value],
                _ => Vec::new(),
            })
            .unwrap_or_default()
    }

    /// Get one previous analog vector output value without cloning the full vector.
    pub fn output_vector_prev_value(&self, name: &str, index: usize) -> Value {
        match self.outputs.get(name) {
            Some(OutputValue::AnalogVector(values)) => values
                .get(index)
                .map(|value| value.prev_value)
                .unwrap_or(0.0),
            Some(OutputValue::Analog(value)) if index == 0 => value.prev_value,
            _ => 0.0,
        }
    }

    /// Get real output value, if this output is a real-valued event port.
    pub fn output_real(&self, name: &str) -> Option<Value> {
        self.outputs.get(name).and_then(|v| match v {
            OutputValue::Real(value) => Some(*value),
            _ => None,
        })
    }

    /// Get digital vector output values.
    pub fn output_digital_vector(&self, name: &str) -> Vec<DigitalValue> {
        self.outputs
            .get(name)
            .map(|v| match v {
                OutputValue::DigitalVector(values) => values.clone(),
                OutputValue::Digital(value) => vec![*value],
                _ => Vec::new(),
            })
            .unwrap_or_default()
    }

    /// Borrow digital vector output values without cloning.
    pub fn output_digital_vector_values(&self, name: &str) -> Option<&[DigitalValue]> {
        match self.outputs.get(name) {
            Some(OutputValue::DigitalVector(values)) => Some(values),
            _ => None,
        }
    }

    /// Get one digital vector output element without cloning the full vector.
    pub fn output_digital_vector_value(&self, name: &str, index: usize) -> Option<DigitalValue> {
        match self.outputs.get(name) {
            Some(OutputValue::DigitalVector(values)) => values.get(index).copied(),
            Some(OutputValue::Digital(value)) if index == 0 => Some(*value),
            _ => None,
        }
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

    /// Get analog vector output partial derivatives.
    pub fn partial_vector(&self, name: &str) -> Vec<Value> {
        self.outputs
            .get(name)
            .map(|v| match v {
                OutputValue::AnalogVector(values) => {
                    values.iter().map(|value| value.partial).collect()
                }
                OutputValue::Analog(value) => vec![value.partial],
                _ => Vec::new(),
            })
            .unwrap_or_default()
    }

    /// Get one analog vector output partial without cloning the full vector.
    pub fn partial_vector_value(&self, name: &str, index: usize) -> Value {
        match self.outputs.get(name) {
            Some(OutputValue::AnalogVector(values)) => {
                values.get(index).map(|value| value.partial).unwrap_or(0.0)
            }
            Some(OutputValue::Analog(value)) if index == 0 => value.partial,
            _ => 0.0,
        }
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

    /// Set analog vector output values with zero direct partials.
    pub fn set_output_vector(&mut self, name: &str, values: Vec<Value>) -> CmResult<()> {
        self.set_output_vector_from_slice(name, &values)
    }

    /// Set analog vector output values from borrowed values with zero direct partials.
    pub fn set_output_vector_from_slice(&mut self, name: &str, values: &[Value]) -> CmResult<()> {
        let width = self.port_width(name).max(values.len());
        if !self.outputs.contains_key(name) {
            self.outputs.insert(
                name.to_string(),
                OutputValue::AnalogVector(analog_vector_values("analog vector output", width)?),
            );
        }
        match self.outputs.get_mut(name) {
            Some(OutputValue::AnalogVector(existing)) => {
                resize_analog_vector_values("analog vector output", existing, width)?;
                for (index, analog) in existing.iter_mut().enumerate() {
                    analog.prev_value = analog.value;
                    analog.value = values.get(index).copied().unwrap_or(0.0);
                    analog.partial = 0.0;
                }
            }
            Some(OutputValue::Analog(existing)) if width == 1 => {
                existing.prev_value = existing.value;
                existing.value = values.first().copied().unwrap_or(0.0);
                existing.partial = 0.0;
            }
            _ => {}
        }
        Ok(())
    }

    /// Set analog vector output values from a callback with zero direct partials.
    pub fn set_output_vector_from_fn<F>(
        &mut self,
        name: &str,
        value_count: usize,
        mut value_at: F,
    ) -> CmResult<()>
    where
        F: FnMut(usize) -> Value,
    {
        let width = self.port_width(name).max(value_count);
        if !self.outputs.contains_key(name) {
            self.outputs.insert(
                name.to_string(),
                OutputValue::AnalogVector(analog_vector_values("analog vector output", width)?),
            );
        }
        match self.outputs.get_mut(name) {
            Some(OutputValue::AnalogVector(existing)) => {
                resize_analog_vector_values("analog vector output", existing, width)?;
                for (index, analog) in existing.iter_mut().enumerate() {
                    analog.prev_value = analog.value;
                    analog.value = value_at(index);
                    analog.partial = 0.0;
                }
            }
            Some(OutputValue::Analog(existing)) if width == 1 => {
                existing.prev_value = existing.value;
                existing.value = value_at(0);
                existing.partial = 0.0;
            }
            _ => {}
        }
        Ok(())
    }

    /// Set analog vector output values and per-element direct partials.
    pub fn set_output_vector_with_partials(
        &mut self,
        name: &str,
        values: Vec<Value>,
        partials: Vec<Value>,
    ) -> CmResult<()> {
        let width = self.port_width(name).max(values.len()).max(partials.len());
        if !self.outputs.contains_key(name) {
            self.outputs.insert(
                name.to_string(),
                OutputValue::AnalogVector(analog_vector_values("analog vector output", width)?),
            );
        }
        match self.outputs.get_mut(name) {
            Some(OutputValue::AnalogVector(existing)) => {
                resize_analog_vector_values("analog vector output", existing, width)?;
                for (index, analog) in existing.iter_mut().enumerate() {
                    analog.prev_value = analog.value;
                    analog.value = values.get(index).copied().unwrap_or(0.0);
                    analog.partial = partials.get(index).copied().unwrap_or(0.0);
                }
            }
            Some(OutputValue::Analog(existing)) if width == 1 => {
                existing.prev_value = existing.value;
                existing.value = values.first().copied().unwrap_or(0.0);
                existing.partial = partials.first().copied().unwrap_or(0.0);
            }
            _ => {}
        }
        Ok(())
    }

    /// Set one analog vector output element with zero direct partial.
    pub fn set_output_vector_element(
        &mut self,
        name: &str,
        index: usize,
        value: Value,
    ) -> CmResult<()> {
        self.set_output_vector_element_with_partial(name, index, value, 0.0)
    }

    /// Set one analog vector output element and direct partial in place.
    pub fn set_output_vector_element_with_partial(
        &mut self,
        name: &str,
        index: usize,
        value: Value,
        partial: Value,
    ) -> CmResult<()> {
        let width = self.port_width(name).max(index + 1);
        if !self.outputs.contains_key(name) {
            self.outputs.insert(
                name.to_string(),
                OutputValue::AnalogVector(analog_vector_values("analog vector output", width)?),
            );
        }
        match self.outputs.get_mut(name) {
            Some(OutputValue::AnalogVector(existing)) => {
                resize_analog_vector_values("analog vector output", existing, width)?;
                if let Some(analog) = existing.get_mut(index) {
                    analog.prev_value = analog.value;
                    analog.value = value;
                    analog.partial = partial;
                }
            }
            Some(OutputValue::Analog(existing)) if index == 0 && width == 1 => {
                existing.prev_value = existing.value;
                existing.value = value;
                existing.partial = partial;
            }
            _ => {}
        }
        Ok(())
    }

    /// Set digital output value (schedules event)
    pub fn set_output_digital(&mut self, name: &str, value: DigitalValue, delay: Value) {
        match self.outputs.get_mut(name) {
            Some(output) => *output = OutputValue::Digital(value),
            None => {
                self.outputs
                    .insert(name.to_string(), OutputValue::Digital(value));
            }
        }
        self.push_pending_digital_event(name, value, delay);
    }

    /// Set real output value (schedules event)
    pub fn set_output_real(&mut self, name: &str, value: Value, delay: Value) {
        match self.outputs.get_mut(name) {
            Some(output) => *output = OutputValue::Real(value),
            None => {
                self.outputs
                    .insert(name.to_string(), OutputValue::Real(value));
            }
        }
        self.push_pending_real_event(name, value, delay);
    }

    /// Set digital output value using official XSPICE inertial delay semantics.
    pub fn set_output_digital_inertial(
        &mut self,
        name: &str,
        value: DigitalValue,
        delay: Value,
        previous: DigitalValue,
        unknown_transition_delays: Option<(Value, Value)>,
    ) {
        match self.outputs.get_mut(name) {
            Some(output) => *output = OutputValue::Digital(value),
            None => {
                self.outputs
                    .insert(name.to_string(), OutputValue::Digital(value));
            }
        }

        if !self.is_transient() {
            self.push_pending_digital_event(name, value, delay);
            return;
        }

        let state = self
            .inertial_outputs
            .entry(name.to_string())
            .or_insert(InertialOutputState {
                when: -1.0,
                prev: previous,
            });
        let mut effective_delay = delay;
        let mut reversion = None;

        if state.when <= self.time {
            state.prev = previous;
            state.when = self.time + delay;
        } else if value != state.prev {
            reversion = Some((state.prev, (state.when - self.time) / 2.0));
            if value.state.is_unknown() {
                if let Some((rise_delay, fall_delay)) = unknown_transition_delays {
                    effective_delay = if state.prev.state.is_low() {
                        rise_delay
                    } else {
                        fall_delay
                    };
                }
            }
            state.when = self.time + effective_delay;
        } else {
            effective_delay = (state.when - self.time) / 2.0;
            state.when = -1.0;
        }

        if let Some((reversion_value, reversion_delay)) = reversion {
            self.push_pending_digital_event(name, reversion_value, reversion_delay);
        }
        self.push_pending_digital_event(name, value, effective_delay);
    }

    fn push_pending_digital_event(&mut self, name: &str, value: DigitalValue, delay: Value) {
        self.pending_events.push(PendingDigitalEvent {
            port_name: name.to_string(),
            start_index: 0,
            values: vec![value],
            delay,
        });
    }

    fn push_pending_real_event(&mut self, name: &str, value: Value, delay: Value) {
        self.pending_real_events.push(PendingRealEvent {
            port_name: name.to_string(),
            start_index: 0,
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
        match self.outputs.get_mut(name) {
            Some(OutputValue::DigitalVector(existing)) => {
                existing.clear();
                existing.extend_from_slice(&values);
            }
            _ => {
                self.outputs
                    .insert(name.to_string(), OutputValue::DigitalVector(values.clone()));
            }
        }
        self.pending_events.push(PendingDigitalEvent {
            port_name: name.to_string(),
            start_index: 0,
            values,
            delay,
        });
    }

    /// Set a digital vector output from values derived from this context.
    pub fn set_output_digital_vector_from_context_fn<F>(
        &mut self,
        name: &str,
        value_count: usize,
        delay: Value,
        mut value_at: F,
    ) -> CmResult<()>
    where
        F: FnMut(&CmContext, usize) -> DigitalValue,
    {
        let event_values = vec_from_fn("digital vector output events", value_count, |index| {
            value_at(self, index)
        })?;
        let mut output_values = Vec::new();
        output_values
            .try_reserve_exact(value_count)
            .map_err(|err| {
                context_allocation_error("digital vector output values", value_count, err)
            })?;
        output_values.extend_from_slice(&event_values);

        match self.outputs.get_mut(name) {
            Some(OutputValue::DigitalVector(existing)) => {
                if existing.capacity() < value_count {
                    let additional = value_count - existing.capacity();
                    existing.try_reserve_exact(additional).map_err(|err| {
                        context_allocation_error("digital vector output values", value_count, err)
                    })?;
                }
                existing.clear();
                existing.extend_from_slice(&output_values);
            }
            _ => {
                self.outputs
                    .insert(name.to_string(), OutputValue::DigitalVector(output_values));
            }
        }
        self.pending_events.push(PendingDigitalEvent {
            port_name: name.to_string(),
            start_index: 0,
            values: event_values,
            delay,
        });
        Ok(())
    }

    /// Set digital vector output from borrowed values.
    pub fn set_output_digital_vector_from_slice(
        &mut self,
        name: &str,
        values: &[DigitalValue],
        delay: Value,
    ) {
        if values.is_empty() {
            return;
        }

        let event_values = values.to_vec();
        match self.outputs.get_mut(name) {
            Some(OutputValue::DigitalVector(existing)) => {
                existing.clear();
                existing.extend_from_slice(values);
            }
            _ => {
                self.outputs.insert(
                    name.to_string(),
                    OutputValue::DigitalVector(event_values.clone()),
                );
            }
        }
        self.pending_events.push(PendingDigitalEvent {
            port_name: name.to_string(),
            start_index: 0,
            values: event_values,
            delay,
        });
    }

    /// Set real vector output value (schedules one event per connected element).
    pub fn set_output_real_vector(&mut self, name: &str, values: Vec<Value>, delay: Value) {
        match self.outputs.get_mut(name) {
            Some(OutputValue::RealVector(existing)) => {
                existing.clear();
                existing.extend_from_slice(&values);
            }
            _ => {
                self.outputs
                    .insert(name.to_string(), OutputValue::RealVector(values.clone()));
            }
        }
        self.pending_real_events.push(PendingRealEvent {
            port_name: name.to_string(),
            start_index: 0,
            values,
            delay,
        });
    }

    /// Set one element of a digital vector output.
    pub fn set_output_digital_vector_element(
        &mut self,
        name: &str,
        index: usize,
        value: DigitalValue,
        delay: Value,
    ) {
        let width = self.port_width(name).max(index + 1);
        match self.outputs.get_mut(name) {
            Some(OutputValue::DigitalVector(values)) => {
                values.resize(width, DigitalValue::unknown());
                values[index] = value;
            }
            _ => {
                let mut values = vec![DigitalValue::unknown(); width];
                values[index] = value;
                self.outputs
                    .insert(name.to_string(), OutputValue::DigitalVector(values));
            }
        }
        self.pending_events.push(PendingDigitalEvent {
            port_name: name.to_string(),
            start_index: index,
            values: vec![value],
            delay,
        });
    }

    /// Initialize output ports
    pub fn init_output(&mut self, name: &str, port_type: PortType) {
        match port_type {
            PortType::Voltage
            | PortType::DifferentialVoltage
            | PortType::Conductance
            | PortType::DifferentialConductance
            | PortType::Hybrid
            | PortType::DifferentialHybrid
            | PortType::Current
            | PortType::DifferentialCurrent => {
                self.outputs.insert(name.to_string(), OutputValue::analog());
            }
            PortType::Digital => {
                self.outputs
                    .insert(name.to_string(), OutputValue::digital());
            }
            PortType::Real => {
                self.outputs.insert(name.to_string(), OutputValue::real());
            }
            _ => {}
        }
    }

    /// Initialize an output vector port.
    pub fn init_output_vector(&mut self, name: &str, port_type: PortType, width: usize) {
        match port_type {
            PortType::Voltage
            | PortType::DifferentialVoltage
            | PortType::Conductance
            | PortType::DifferentialConductance
            | PortType::Hybrid
            | PortType::DifferentialHybrid
            | PortType::Current
            | PortType::DifferentialCurrent => {
                self.outputs
                    .insert(name.to_string(), OutputValue::analog_vector(width));
            }
            PortType::Digital => {
                self.outputs.insert(
                    name.to_string(),
                    OutputValue::DigitalVector(vec![DigitalValue::default(); width]),
                );
            }
            PortType::Real => {
                self.outputs
                    .insert(name.to_string(), OutputValue::RealVector(vec![0.0; width]));
            }
            _ => {}
        }
    }

    /// Register connected node for an analog scalar port (0 = ground).
    pub fn set_port_node(&mut self, name: &str, node: usize) {
        self.port_nodes.insert(name.to_string(), node);
        self.port_terminals.insert(name.to_string(), (node, 0));
    }

    /// Register connected terminal pair for a scalar or differential analog port.
    pub fn set_port_terminals(&mut self, name: &str, pos: usize, neg: usize) {
        self.port_nodes.insert(name.to_string(), pos);
        self.port_terminals.insert(name.to_string(), (pos, neg));
    }

    /// Register connected terminal pairs for an analog vector port.
    pub fn set_port_vector_terminals(&mut self, name: &str, terminals: Vec<(usize, usize)>) {
        self.port_vector_terminals
            .insert(name.to_string(), terminals);
    }

    /// Register the matrix column for a branch-current input port.
    pub fn set_port_control_column(&mut self, name: &str, column: usize) {
        self.port_control_columns.insert(name.to_string(), column);
    }

    /// Get connected node for an analog scalar port (0 = ground).
    pub fn port_node(&self, name: &str) -> Option<usize> {
        self.port_nodes.get(name).copied()
    }

    /// Get connected terminal pair for a scalar or differential analog port.
    pub fn port_node_pair(&self, name: &str) -> Option<(usize, usize)> {
        self.port_terminals.get(name).copied()
    }

    /// Get connected terminal pair for one analog vector port element.
    pub fn port_vector_node_pair(&self, name: &str, index: usize) -> Option<(usize, usize)> {
        self.port_vector_terminals
            .get(name)
            .and_then(|pairs| pairs.get(index).copied())
    }

    /// Get matrix column for a branch-current input port.
    pub fn port_control_column(&self, name: &str) -> Option<usize> {
        self.port_control_columns.get(name).copied()
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
        self.port_terminals.clear();
        self.port_vector_terminals.clear();
        self.port_control_columns.clear();
    }

    /// Get all pending events and clear the queue
    pub(crate) fn take_pending_events(&mut self) -> Vec<PendingDigitalEvent> {
        std::mem::take(&mut self.pending_events)
    }

    /// Drain pending events while preserving the queue allocation.
    pub(crate) fn drain_pending_events(&mut self) -> Drain<'_, PendingDigitalEvent> {
        self.pending_events.drain(..)
    }

    /// Get all pending real events and clear the queue
    pub(crate) fn take_pending_real_events(&mut self) -> Vec<PendingRealEvent> {
        std::mem::take(&mut self.pending_real_events)
    }

    /// Drain pending real-valued events while preserving the queue allocation.
    pub(crate) fn drain_pending_real_events(&mut self) -> Drain<'_, PendingRealEvent> {
        self.pending_real_events.drain(..)
    }

    /// Request that the transient stepper place a breakpoint at an absolute time.
    pub fn request_breakpoint(&mut self, time: Value) {
        if time.is_finite() && time >= 0.0 {
            self.requested_breakpoints.push(time);
        }
    }

    /// Drain pending absolute transient breakpoint requests.
    pub fn take_requested_breakpoints(&mut self) -> Vec<Value> {
        std::mem::take(&mut self.requested_breakpoints)
    }

    /// Drain pending breakpoint requests while preserving the queue allocation.
    pub(crate) fn drain_requested_breakpoints(&mut self) -> Drain<'_, Value> {
        self.requested_breakpoints.drain(..)
    }

    //-------------------------------------------------------------------------
    // Parameter Access
    //-------------------------------------------------------------------------

    /// Get parameter value
    pub fn param(&self, name: &str) -> Value {
        let key = Self::canonical_param_lookup_key(name);
        self.params.get(key.as_ref()).copied().unwrap_or(0.0)
    }

    /// Get parameter with default
    pub fn param_or(&self, name: &str, default: Value) -> Value {
        let key = Self::canonical_param_lookup_key(name);
        self.params.get(key.as_ref()).copied().unwrap_or(default)
    }

    /// Get complex parameter
    pub fn complex_param(&self, name: &str) -> Option<Complex64> {
        let key = Self::canonical_param_lookup_key(name);
        self.complex_params.get(key.as_ref()).copied()
    }

    /// Get complex parameter with default
    pub fn complex_param_or(&self, name: &str, default: Complex64) -> Complex64 {
        let key = Self::canonical_param_lookup_key(name);
        self.complex_params
            .get(key.as_ref())
            .copied()
            .unwrap_or(default)
    }

    /// Get string parameter
    pub fn string_param(&self, name: &str) -> Option<&str> {
        let key = Self::canonical_param_lookup_key(name);
        self.string_params.get(key.as_ref()).map(|s| s.as_str())
    }

    /// Get the revision assigned when a string parameter was last set.
    pub(crate) fn string_param_revision(&self, name: &str) -> Option<u64> {
        let key = Self::canonical_param_lookup_key(name);
        self.string_param_revisions.get(key.as_ref()).copied()
    }

    /// Get string-vector parameter
    pub fn string_vector_param(&self, name: &str) -> Option<&[String]> {
        let key = Self::canonical_param_lookup_key(name);
        self.string_vector_params
            .get(key.as_ref())
            .map(|values| values.as_slice())
    }

    /// Get complex-vector parameter
    pub fn complex_vector_param(&self, name: &str) -> Option<&[Complex64]> {
        let key = Self::canonical_param_lookup_key(name);
        self.complex_vector_params
            .get(key.as_ref())
            .map(|values| values.as_slice())
    }

    /// Get real-vector parameter
    pub fn real_vector_param(&self, name: &str) -> Option<&[Value]> {
        let key = Self::canonical_param_lookup_key(name);
        self.real_vector_params
            .get(key.as_ref())
            .map(|values| values.as_slice())
    }

    /// Get the revision assigned when a real-vector parameter was last set.
    pub(crate) fn real_vector_param_revision(&self, name: &str) -> Option<u64> {
        let key = Self::canonical_param_lookup_key(name);
        self.real_vector_param_revisions.get(key.as_ref()).copied()
    }

    /// Get integer-vector parameter
    pub fn integer_vector_param(&self, name: &str) -> Option<&[i64]> {
        let key = Self::canonical_param_lookup_key(name);
        self.integer_vector_params
            .get(key.as_ref())
            .map(|values| values.as_slice())
    }

    /// Return true when a parameter was explicitly supplied instead of coming from a default.
    pub fn param_was_provided(&self, name: &str) -> bool {
        let key = Self::canonical_param_lookup_key(name);
        self.provided_params.contains(key.as_ref())
    }

    /// Mark a parameter as explicitly supplied by the instance or model card.
    pub fn mark_param_provided(&mut self, name: &str) {
        self.provided_params.insert(Self::canonical_param_key(name));
    }

    /// Set parameter value
    pub fn set_param(&mut self, name: &str, value: Value) {
        self.params.insert(Self::canonical_param_key(name), value);
    }

    /// Set complex parameter
    pub fn set_complex_param(&mut self, name: &str, value: Complex64) {
        self.complex_params
            .insert(Self::canonical_param_key(name), value);
    }

    /// Set string parameter
    pub fn set_string_param(&mut self, name: &str, value: &str) {
        let key = Self::canonical_param_key(name);
        let revision = self.next_string_param_revision;
        self.next_string_param_revision = self.next_string_param_revision.wrapping_add(1);
        if self.next_string_param_revision == 0 {
            self.next_string_param_revision = 1;
        }
        self.string_params.insert(key.clone(), value.to_string());
        self.string_param_revisions.insert(key, revision);
    }

    /// Set string-vector parameter
    pub fn set_string_vector_param(&mut self, name: &str, value: Vec<String>) {
        self.string_vector_params
            .insert(Self::canonical_param_key(name), value);
    }

    /// Set complex-vector parameter
    pub fn set_complex_vector_param(&mut self, name: &str, value: Vec<Complex64>) {
        self.complex_vector_params
            .insert(Self::canonical_param_key(name), value);
    }

    /// Set real-vector parameter
    pub fn set_real_vector_param(&mut self, name: &str, value: Vec<Value>) {
        let key = Self::canonical_param_key(name);
        let revision = self.next_real_vector_param_revision;
        self.next_real_vector_param_revision = self.next_real_vector_param_revision.wrapping_add(1);
        if self.next_real_vector_param_revision == 0 {
            self.next_real_vector_param_revision = 1;
        }
        self.real_vector_params.insert(key.clone(), value);
        self.real_vector_param_revisions.insert(key, revision);
    }

    /// Set integer-vector parameter
    pub fn set_integer_vector_param(&mut self, name: &str, value: Vec<i64>) {
        self.integer_vector_params
            .insert(Self::canonical_param_key(name), value);
    }

    //-------------------------------------------------------------------------
    // Resource Access
    //-------------------------------------------------------------------------

    /// Store a typed host resource in the model context.
    pub fn set_resource<T>(&mut self, key: impl Into<String>, resource: Arc<T>)
    where
        T: Any + Send + Sync + 'static,
    {
        self.resources.values.insert(key.into(), resource);
    }

    /// Fetch a typed host resource from the model context.
    pub fn resource<T>(&self, key: &str) -> Option<Arc<T>>
    where
        T: Any + Send + Sync + 'static,
    {
        self.resources
            .values
            .get(key)
            .and_then(|resource| Arc::clone(resource).downcast::<T>().ok())
    }

    /// Fetch a uniquely-owned typed host resource mutably.
    pub fn resource_mut<T>(&mut self, key: &str) -> Option<&mut T>
    where
        T: Any + Send + Sync + 'static,
    {
        self.resources
            .values
            .get_mut(key)
            .and_then(Arc::get_mut)
            .and_then(|resource| resource.downcast_mut::<T>())
    }

    /// Fetch a typed host resource mutably, cloning it first if shared.
    pub fn resource_make_mut<T>(&mut self, key: &str) -> Option<&mut T>
    where
        T: Any + Send + Sync + Clone + 'static,
    {
        let resource = self.resources.values.get_mut(key)?;
        if Arc::strong_count(resource) == 1 {
            return Arc::get_mut(resource).and_then(|resource| resource.downcast_mut::<T>());
        }

        let cloned = Arc::clone(resource)
            .downcast::<T>()
            .ok()
            .map(|typed| (*typed).clone())?;
        *resource = Arc::new(cloned);
        Arc::get_mut(resource).and_then(|resource| resource.downcast_mut::<T>())
    }

    /// Snapshot the serializable model-owned state arrays.
    pub(crate) fn checkpoint_state(&self) -> CmContextCheckpoint {
        CmContextCheckpoint {
            time: self.time,
            time_prev: self.time_prev,
            state: self.state.clone(),
            state_prev: self.state_prev.clone(),
            int_state: self.int_state.clone(),
        }
    }

    /// Restore model-owned state arrays into an initialized context.
    pub(crate) fn validate_checkpoint_state(
        &self,
        checkpoint: &CmContextCheckpoint,
    ) -> CmResult<()> {
        if self.state.len() != checkpoint.state.len()
            || self.state_prev.len() != checkpoint.state_prev.len()
            || self.int_state.len() != checkpoint.int_state.len()
        {
            return Err(CmError::EvaluationError(format!(
                "checkpoint state shape mismatch: context has real {}/{}, integer {}; \
                 checkpoint has real {}/{}, integer {}",
                self.state.len(),
                self.state_prev.len(),
                self.int_state.len(),
                checkpoint.state.len(),
                checkpoint.state_prev.len(),
                checkpoint.int_state.len()
            )));
        }
        Ok(())
    }

    /// Restore model-owned state arrays into an initialized context.
    pub(crate) fn restore_checkpoint_state(
        &mut self,
        checkpoint: &CmContextCheckpoint,
    ) -> CmResult<()> {
        self.validate_checkpoint_state(checkpoint)?;

        self.state.clone_from(&checkpoint.state);
        self.state_prev.clone_from(&checkpoint.state_prev);
        self.int_state.clone_from(&checkpoint.int_state);
        self.time = checkpoint.time;
        self.time_prev = checkpoint.time_prev;
        Ok(())
    }

    /// Whether this context owns any state that must be represented in a
    /// transient checkpoint before resume is safe.
    pub(crate) fn has_serializable_checkpoint_state(&self) -> bool {
        !self.state.is_empty() || !self.state_prev.is_empty() || !self.int_state.is_empty()
    }

    /// Summary of runtime-only state that is still not serialized in transient
    /// checkpoint files.
    pub(crate) fn checkpoint_nonserializable_runtime_state_summary(&self) -> Option<String> {
        let mut parts = Vec::new();
        if !self.transient_histories.is_empty() {
            parts.push(format!(
                "{} transient history buffer(s)",
                self.transient_histories.len()
            ));
        }
        if !self.resources.values.is_empty() {
            parts.push(format!("{} host resource(s)", self.resources.values.len()));
        }
        if !self.pending_events.is_empty() {
            parts.push(format!(
                "{} pending digital event(s)",
                self.pending_events.len()
            ));
        }
        if !self.pending_real_events.is_empty() {
            parts.push(format!(
                "{} pending real event(s)",
                self.pending_real_events.len()
            ));
        }
        if !self.inertial_outputs.is_empty() {
            parts.push(format!(
                "{} inertial output state(s)",
                self.inertial_outputs.len()
            ));
        }
        if !self.requested_breakpoints.is_empty() {
            parts.push(format!(
                "{} requested breakpoint(s)",
                self.requested_breakpoints.len()
            ));
        }

        (!parts.is_empty()).then(|| parts.join(", "))
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

    /// Record one rollback-aware transient sample for model-owned history.
    ///
    /// Samples at the same time replace the last sample, which avoids growing
    /// history during repeated Newton iterations. Samples newer than `time`
    /// are discarded before appending, matching transient rollback semantics.
    pub fn record_transient_history(
        &mut self,
        key: &str,
        time: Value,
        values: Vec<Value>,
        retention_window: Value,
    ) {
        if !time.is_finite() || values.iter().any(|value| !value.is_finite()) {
            return;
        }

        let history = self.transient_histories.entry(key.to_string()).or_default();

        while let Some(last) = history.last() {
            if last.time > time && !same_transient_time(last.time, time) {
                history.pop();
            } else {
                break;
            }
        }

        if let Some(last) = history.last_mut()
            && same_transient_time(last.time, time)
        {
            last.time = time;
            last.values = values;
            prune_transient_history(history, time, retention_window);
            return;
        }

        history.push(TransientHistorySample { time, values });
        prune_transient_history(history, time, retention_window);
    }

    /// Return the first recorded sample values at or after `time`.
    pub fn transient_history_values_at_or_after(&self, key: &str, time: Value) -> Option<&[Value]> {
        if !time.is_finite() {
            return None;
        }
        self.transient_histories.get(key).and_then(|history| {
            let index = history.partition_point(|sample| {
                sample.time < time && !same_transient_time(sample.time, time)
            });
            history.get(index).map(|sample| sample.values.as_slice())
        })
    }

    /// Return the first recorded sample at or after `time`.
    pub fn transient_history_at_or_after(&self, key: &str, time: Value) -> Option<Vec<Value>> {
        self.transient_history_values_at_or_after(key, time)
            .map(<[Value]>::to_vec)
    }

    /// Advance state for new timestep
    pub fn advance_state(&mut self) {
        self.state_prev.clone_from(&self.state);
        for output in self.outputs.values_mut() {
            match output {
                OutputValue::Analog(value) => {
                    value.prev_value = value.value;
                }
                OutputValue::AnalogVector(values) => {
                    for value in values {
                        value.prev_value = value.value;
                    }
                }
                _ => {}
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

    /// Drain conductance stamps while preserving the queue allocation.
    pub(crate) fn drain_stamps(&mut self) -> Drain<'_, (usize, usize, Value)> {
        self.stamps.drain(..)
    }

    /// Get all RHS contributions and clear
    pub fn take_rhs(&mut self) -> Vec<(usize, Value)> {
        std::mem::take(&mut self.rhs)
    }

    /// Drain RHS contributions while preserving the queue allocation.
    pub(crate) fn drain_rhs(&mut self) -> Drain<'_, (usize, Value)> {
        self.rhs.drain(..)
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
        assert_eq!(digital.try_real(), None);

        let analog = InputValue::Analog(AnalogValue::new(1.25));
        assert_eq!(analog.try_digital(), None);
        assert!(analog.try_digital_vector().is_none());
        assert_eq!(analog.try_real(), None);
    }

    #[test]
    fn input_value_try_accessors_return_matching_values() {
        let analog = InputValue::Analog(AnalogValue::new(1.25));
        assert_eq!(analog.try_analog(), Some(1.25));

        let digital_value = DigitalValue::new(DigitalState::One, DigitalStrength::Strong);
        let digital = InputValue::Digital(digital_value);
        assert_eq!(digital.try_digital(), Some(digital_value));

        let real = InputValue::Real(2.5);
        assert_eq!(real.try_real(), Some(2.5));
    }

    #[test]
    fn vector_input_borrow_accessors_do_not_require_clones() {
        let mut ctx = CmContext::new();
        ctx.set_input(
            "analog",
            InputValue::AnalogVector(vec![AnalogValue::new(1.0), AnalogValue::new(2.0)]),
        );
        ctx.set_input(
            "digital",
            InputValue::DigitalVector(vec![DigitalValue::zero(), DigitalValue::one()]),
        );
        ctx.set_input("real", InputValue::RealVector(vec![3.0, 4.0]));

        let analog = ctx
            .input_analog_vector_values("analog")
            .expect("analog vector is borrowed");
        assert_eq!(
            analog.iter().map(|value| value.value).collect::<Vec<_>>(),
            vec![1.0, 2.0]
        );
        assert_eq!(
            ctx.input_digital_vector_values("digital"),
            Some([DigitalValue::zero(), DigitalValue::one()].as_slice())
        );
        assert_eq!(
            ctx.input_real_vector_values("real"),
            Some([3.0, 4.0].as_slice())
        );
        assert!(ctx.input_analog_vector_values("missing").is_none());
    }

    #[test]
    fn analog_vector_element_setter_updates_in_place() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("out", 2);
        ctx.set_output_vector("out", vec![1.0, 2.0])
            .expect("set output vector");

        ctx.set_output_vector_element("out", 0, 1.5)
            .expect("set output vector element");
        ctx.set_output_vector_element_with_partial("out", 1, 3.0, 4.0)
            .expect("set output vector element partial");

        assert_eq!(ctx.output_vector("out"), vec![1.5, 3.0]);
        assert_eq!(ctx.output_vector_prev("out"), vec![1.0, 2.0]);
        assert_eq!(ctx.partial_vector("out"), vec![0.0, 4.0]);
    }

    #[test]
    fn analog_vector_function_setter_preserves_prev_values_once() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("out", 3);
        ctx.set_output_vector("out", vec![1.0, 2.0, 3.0])
            .expect("set output vector");
        ctx.set_output_vector_with_partials("out", vec![4.0, 5.0], vec![0.25, 0.5])
            .expect("set output vector partials");

        let mut calls = 0;
        ctx.set_output_vector_from_fn("out", 3, |index| {
            calls += 1;
            if index == 1 { 9.0 } else { 0.0 }
        })
        .expect("set output vector from function");

        assert_eq!(calls, 3);
        assert_eq!(ctx.output_vector("out"), vec![0.0, 9.0, 0.0]);
        assert_eq!(ctx.output_vector_prev("out"), vec![4.0, 5.0, 0.0]);
        assert_eq!(ctx.partial_vector("out"), vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn digital_vector_slice_setter_reuses_output_and_schedules_event() {
        let mut ctx = CmContext::new();
        ctx.set_output_digital_vector("out", vec![DigitalValue::zero(), DigitalValue::one()], 0.0);
        ctx.take_pending_events();

        let values = [DigitalValue::one(), DigitalValue::unknown()];
        ctx.set_output_digital_vector_from_slice("out", &values, 2.0e-9);

        assert_eq!(ctx.output_digital_vector("out"), values);
        let events = ctx.take_pending_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].values, values);
        assert_eq!(events[0].delay, 2.0e-9);
    }

    #[test]
    fn digital_vector_owned_setter_replaces_output_and_schedules_event() {
        let mut ctx = CmContext::new();
        ctx.set_output_digital_vector(
            "out",
            vec![
                DigitalValue::zero(),
                DigitalValue::one(),
                DigitalValue::unknown(),
            ],
            0.0,
        );
        ctx.take_pending_events();

        let values = vec![DigitalValue::one()];
        ctx.set_output_digital_vector("out", values.clone(), 4.0e-9);

        assert_eq!(ctx.output_digital_vector("out"), values);
        let events = ctx.take_pending_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].values, values);
        assert_eq!(events[0].delay, 4.0e-9);
    }

    #[test]
    fn scalar_event_output_setters_replace_values_without_growing_map() {
        let mut ctx = CmContext::new();

        ctx.set_output_digital("out", DigitalValue::zero(), 0.0);
        ctx.set_output_digital("out", DigitalValue::one(), 1.0e-9);
        assert_eq!(
            ctx.output_digital_vector_value("out", 0),
            Some(DigitalValue::one())
        );
        assert_eq!(ctx.outputs.len(), 1);

        ctx.set_output_real("out", 1.25, 2.0e-9);
        assert_eq!(ctx.output_real("out"), Some(1.25));
        assert_eq!(ctx.outputs.len(), 1);

        ctx.set_output_digital_inertial(
            "out",
            DigitalValue::unknown(),
            3.0e-9,
            DigitalValue::zero(),
            None,
        );
        assert_eq!(
            ctx.output_digital_vector_value("out", 0),
            Some(DigitalValue::unknown())
        );
        assert_eq!(ctx.outputs.len(), 1);

        let digital_events = ctx.take_pending_events();
        assert_eq!(digital_events.len(), 3);
        assert_eq!(digital_events[2].values, [DigitalValue::unknown()]);
        assert_eq!(digital_events[2].delay, 3.0e-9);

        let real_events = ctx.take_pending_real_events();
        assert_eq!(real_events.len(), 1);
        assert_eq!(real_events[0].values, [1.25]);
        assert_eq!(real_events[0].delay, 2.0e-9);
    }

    #[test]
    fn pending_event_drains_preserve_queue_capacity() {
        let mut ctx = CmContext::new();

        ctx.set_output_digital("dout", DigitalValue::zero(), 0.0);
        ctx.set_output_digital("dout", DigitalValue::one(), 1.0e-9);
        let digital_capacity = ctx.pending_events.capacity();
        let digital_events = ctx.drain_pending_events().collect::<Vec<_>>();
        assert_eq!(digital_events.len(), 2);
        assert!(ctx.pending_events.is_empty());
        assert_eq!(ctx.pending_events.capacity(), digital_capacity);

        ctx.set_output_real("rout", 1.0, 0.0);
        ctx.set_output_real("rout", 2.0, 1.0e-9);
        let real_capacity = ctx.pending_real_events.capacity();
        let real_events = ctx.drain_pending_real_events().collect::<Vec<_>>();
        assert_eq!(real_events.len(), 2);
        assert!(ctx.pending_real_events.is_empty());
        assert_eq!(ctx.pending_real_events.capacity(), real_capacity);
    }

    #[test]
    fn real_vector_owned_setter_reuses_output_buffer_and_schedules_event() {
        let mut ctx = CmContext::new();
        ctx.set_output_real_vector("out", vec![1.0, 2.0, 3.0], 0.0);
        ctx.take_pending_real_events();

        let vector_ptr = match ctx.outputs.get("out") {
            Some(OutputValue::RealVector(values)) => values.as_ptr(),
            other => panic!("expected real vector output, got {other:?}"),
        };

        let values = vec![4.0];
        ctx.set_output_real_vector("out", values.clone(), 4.0e-9);

        match ctx.outputs.get("out") {
            Some(OutputValue::RealVector(values)) => {
                assert_eq!(values.as_slice(), &[4.0]);
                assert_eq!(values.as_ptr(), vector_ptr);
            }
            other => panic!("expected reused real vector output, got {other:?}"),
        }
        let events = ctx.take_pending_real_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].values, values);
        assert_eq!(events[0].delay, 4.0e-9);
    }

    #[test]
    fn digital_vector_context_fn_reads_context_before_replacing_output() {
        let mut ctx = CmContext::new();
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![DigitalValue::zero(), DigitalValue::one()]),
        );
        ctx.set_output_digital_vector("out", vec![DigitalValue::unknown()], 0.0);
        ctx.take_pending_events();

        ctx.set_output_digital_vector_from_context_fn("out", 2, 8.0e-9, |ctx, index| {
            ctx.input_digital_vector_values("in")
                .and_then(|values| values.get(index).copied())
                .unwrap_or_default()
        })
        .expect("set digital vector output from context");

        let values = vec![DigitalValue::zero(), DigitalValue::one()];
        assert_eq!(ctx.output_digital_vector("out"), values);
        let events = ctx.take_pending_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].values, values);
        assert_eq!(events[0].delay, 8.0e-9);
    }

    #[test]
    fn scalar_input_setters_replace_values_without_growing_maps() {
        let mut ctx = CmContext::new();

        ctx.set_input_analog("in", 1.0);
        ctx.set_input_analog("in", 2.0);
        assert_eq!(ctx.input("in"), 2.0);
        assert_eq!(ctx.inputs.len(), 1);

        ctx.set_input_digital("in", DigitalValue::one());
        assert_eq!(ctx.input_digital("in"), Some(DigitalValue::one()));
        assert_eq!(ctx.inputs.len(), 1);

        ctx.set_input_real("in", 3.0);
        assert_eq!(ctx.input_real("in"), Some(3.0));
        assert_eq!(ctx.inputs.len(), 1);

        ctx.set_input("in", InputValue::Digital(DigitalValue::zero()));
        assert_eq!(ctx.input_digital("in"), Some(DigitalValue::zero()));
        assert_eq!(ctx.inputs.len(), 1);

        ctx.set_input_digital_event_time("clk", 1.0e-9);
        ctx.set_input_digital_event_time("clk", 2.0e-9);
        assert_eq!(ctx.input_digital_event_time("clk"), Some(2.0e-9));
        assert_eq!(ctx.input_event_times.len(), 1);

        ctx.set_input_real_event_time("clk", 3.0e-9);
        assert_eq!(ctx.input_real_event_time("clk"), Some(3.0e-9));
        assert_eq!(ctx.input_event_times.len(), 1);
    }

    #[test]
    fn vector_input_setters_reuse_existing_buffers() {
        let mut ctx = CmContext::new();

        ctx.set_input_digital_vector_from_fn("din", 4, |index| {
            if index % 2 == 0 {
                DigitalValue::zero()
            } else {
                DigitalValue::one()
            }
        })
        .expect("set digital vector input");
        let digital_ptr = match ctx.inputs.get("din") {
            Some(InputValue::DigitalVector(values)) => values.as_ptr(),
            other => panic!("expected digital vector input, got {other:?}"),
        };
        ctx.set_input_digital_vector_from_fn("din", 2, |_| DigitalValue::unknown())
            .expect("reuse digital vector input");
        assert_eq!(
            ctx.input_digital_vector_values("din").unwrap(),
            &[DigitalValue::unknown(), DigitalValue::unknown()]
        );
        match ctx.inputs.get("din") {
            Some(InputValue::DigitalVector(values)) => assert_eq!(values.as_ptr(), digital_ptr),
            other => panic!("expected reused digital vector input, got {other:?}"),
        }

        ctx.set_input_digital_vector_event_times(
            "din",
            vec![Some(0.0), Some(1.0e-9), Some(2.0e-9), Some(3.0e-9)],
        );
        let times_ptr = ctx.input_vector_event_times["din"].as_ptr();
        ctx.set_input_digital_vector_event_times_from_fn("din", 2, |index| {
            (index == 1).then_some(3.0e-9)
        })
        .expect("reuse digital vector event times");
        assert_eq!(ctx.input_digital_vector_event_time("din", 0), None);
        assert_eq!(ctx.input_digital_vector_event_time("din", 1), Some(3.0e-9));
        assert_eq!(ctx.input_vector_event_times["din"].as_ptr(), times_ptr);
        ctx.set_input_digital_vector_event_times("din", vec![Some(5.0e-9)]);
        assert_eq!(ctx.input_digital_vector_event_time("din", 0), Some(5.0e-9));
        assert_eq!(ctx.input_vector_event_times["din"].as_ptr(), times_ptr);

        ctx.set_input_analog_vector_from_fn("ain", 3, |index| AnalogValue::new(index as Value))
            .expect("set analog vector input");
        let analog_ptr = match ctx.inputs.get("ain") {
            Some(InputValue::AnalogVector(values)) => values.as_ptr(),
            other => panic!("expected analog vector input, got {other:?}"),
        };
        ctx.set_input_analog_vector_from_fn("ain", 2, |index| {
            AnalogValue::new(10.0 + index as Value)
        })
        .expect("reuse analog vector input");
        assert_eq!(
            ctx.input_analog_vector_values("ain")
                .unwrap()
                .iter()
                .map(|value| value.value)
                .collect::<Vec<_>>(),
            vec![10.0, 11.0]
        );
        match ctx.inputs.get("ain") {
            Some(InputValue::AnalogVector(values)) => assert_eq!(values.as_ptr(), analog_ptr),
            other => panic!("expected reused analog vector input, got {other:?}"),
        }

        ctx.set_input_real_vector_from_fn("rin", 3, |index| index as Value)
            .expect("set real vector input");
        let real_ptr = match ctx.inputs.get("rin") {
            Some(InputValue::RealVector(values)) => values.as_ptr(),
            other => panic!("expected real vector input, got {other:?}"),
        };
        ctx.set_input_real_vector_from_fn("rin", 2, |index| 20.0 + index as Value)
            .expect("reuse real vector input");
        assert_eq!(ctx.input_real_vector_values("rin").unwrap(), &[20.0, 21.0]);
        match ctx.inputs.get("rin") {
            Some(InputValue::RealVector(values)) => assert_eq!(values.as_ptr(), real_ptr),
            other => panic!("expected reused real vector input, got {other:?}"),
        }
    }

    #[test]
    fn digital_vector_scalar_output_accessor_avoids_full_clone() {
        let mut ctx = CmContext::new();
        ctx.set_output_digital_vector("out", vec![DigitalValue::zero(), DigitalValue::one()], 0.0);
        ctx.set_output_digital("scalar", DigitalValue::unknown(), 0.0);

        assert_eq!(
            ctx.output_digital_vector_value("out", 0),
            Some(DigitalValue::zero())
        );
        assert_eq!(
            ctx.output_digital_vector_value("out", 1),
            Some(DigitalValue::one())
        );
        assert_eq!(ctx.output_digital_vector_value("out", 2), None);
        assert_eq!(
            ctx.output_digital_vector_value("scalar", 0),
            Some(DigitalValue::unknown())
        );
        assert_eq!(ctx.output_digital_vector_value("scalar", 1), None);
    }

    #[test]
    fn analog_vector_scalar_output_accessors_avoid_full_clones() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("out", 2);
        ctx.set_output_vector("out", vec![1.0, 2.0])
            .expect("set output vector");
        ctx.set_output_vector_with_partials("out", vec![1.5, 2.5], vec![0.25, 0.5])
            .expect("set output vector partials");
        ctx.set_output_with_partial("scalar", 2.5, 0.25);
        ctx.set_output_with_partial("scalar", 3.5, 0.75);

        assert_eq!(ctx.output_vector_value("out", 0), 1.5);
        assert_eq!(ctx.output_vector_value("out", 1), 2.5);
        assert_eq!(ctx.output_vector_value("out", 2), 0.0);
        assert_eq!(ctx.output_vector_prev_value("out", 0), 1.0);
        assert_eq!(ctx.output_vector_prev_value("out", 1), 2.0);
        assert_eq!(ctx.output_vector_prev_value("out", 2), 0.0);
        assert_eq!(ctx.partial_vector_value("out", 0), 0.25);
        assert_eq!(ctx.partial_vector_value("out", 1), 0.5);
        assert_eq!(ctx.partial_vector_value("out", 2), 0.0);

        assert_eq!(ctx.output_vector_value("scalar", 0), 3.5);
        assert_eq!(ctx.output_vector_value("scalar", 1), 0.0);
        assert_eq!(ctx.output_vector_prev_value("scalar", 0), 2.5);
        assert_eq!(ctx.output_vector_prev_value("scalar", 1), 0.0);
        assert_eq!(ctx.partial_vector_value("scalar", 0), 0.75);
        assert_eq!(ctx.partial_vector_value("scalar", 1), 0.0);
    }

    #[test]
    fn parameter_lookup_remains_case_insensitive_for_all_channels() {
        let mut ctx = CmContext::new();
        ctx.set_param("Gain", 2.5);
        ctx.set_complex_param("Pole", Complex64::new(1.0, -2.0));
        ctx.set_string_param("File", "table.dat");
        ctx.set_string_vector_param("Labels", vec!["a".to_string(), "b".to_string()]);
        ctx.set_complex_vector_param(
            "Zeros",
            vec![Complex64::new(3.0, 4.0), Complex64::new(5.0, -6.0)],
        );
        ctx.set_real_vector_param("Points", vec![1.0, 2.0]);
        ctx.set_integer_vector_param("Bits", vec![1, 0]);
        ctx.mark_param_provided("Gain");

        assert_eq!(ctx.param("gain"), 2.5);
        assert_eq!(ctx.param("GAIN"), 2.5);
        assert_eq!(ctx.param_or("missing", 7.0), 7.0);
        assert_eq!(ctx.complex_param("POLE"), Some(Complex64::new(1.0, -2.0)));
        assert_eq!(
            ctx.complex_param_or("missing_complex", Complex64::new(7.0, 8.0)),
            Complex64::new(7.0, 8.0)
        );
        assert_eq!(ctx.string_param("file"), Some("table.dat"));
        let file_revision = ctx
            .string_param_revision("FILE")
            .expect("string parameter revision is tracked");
        ctx.set_string_param("file", "updated.tbl");
        assert_eq!(ctx.string_param("FILE"), Some("updated.tbl"));
        assert!(
            ctx.string_param_revision("file").unwrap() > file_revision,
            "string parameter revisions advance when a parameter is replaced"
        );
        assert_eq!(
            ctx.string_vector_param("LABELS"),
            Some(["a".to_string(), "b".to_string()].as_slice())
        );
        assert_eq!(
            ctx.complex_vector_param("zeros"),
            Some([Complex64::new(3.0, 4.0), Complex64::new(5.0, -6.0)].as_slice())
        );
        assert_eq!(ctx.real_vector_param("points"), Some([1.0, 2.0].as_slice()));
        let points_revision = ctx
            .real_vector_param_revision("POINTS")
            .expect("real-vector parameter revision is tracked");
        ctx.set_real_vector_param("points", vec![3.0, 4.0]);
        assert_eq!(ctx.real_vector_param("POINTS"), Some([3.0, 4.0].as_slice()));
        assert!(
            ctx.real_vector_param_revision("points").unwrap() > points_revision,
            "real-vector parameter revisions advance when a parameter is replaced"
        );
        assert_eq!(ctx.integer_vector_param("BITS"), Some([1, 0].as_slice()));
        assert!(ctx.param_was_provided("gain"));
        assert!(ctx.param_was_provided("GAIN"));
    }

    #[test]
    fn context_records_and_drains_transient_breakpoint_requests() {
        let mut ctx = CmContext::new();

        ctx.request_breakpoint(2.0e-9);
        ctx.request_breakpoint(f64::NAN);
        ctx.request_breakpoint(-1.0e-9);
        ctx.request_breakpoint(1.0e-9);

        assert_eq!(ctx.take_requested_breakpoints(), vec![2.0e-9, 1.0e-9]);
        assert!(ctx.take_requested_breakpoints().is_empty());
    }

    #[test]
    fn requested_breakpoint_drains_preserve_queue_capacity() {
        let mut ctx = CmContext::new();

        ctx.request_breakpoint(1.0e-9);
        ctx.request_breakpoint(2.0e-9);
        let capacity = ctx.requested_breakpoints.capacity();
        let breakpoints = ctx.drain_requested_breakpoints().collect::<Vec<_>>();

        assert_eq!(breakpoints, vec![1.0e-9, 2.0e-9]);
        assert!(ctx.requested_breakpoints.is_empty());
        assert_eq!(ctx.requested_breakpoints.capacity(), capacity);
    }

    #[test]
    fn matrix_stamp_drains_preserve_queue_capacity() {
        let mut ctx = CmContext::new();

        ctx.stamp_conductance(1, 2, 3.0);
        ctx.stamp_conductance(4, 5, 6.0);
        let stamp_capacity = ctx.stamps.capacity();
        let stamps = ctx.drain_stamps().collect::<Vec<_>>();
        assert_eq!(stamps, vec![(1, 2, 3.0), (4, 5, 6.0)]);
        assert!(ctx.stamps.is_empty());
        assert_eq!(ctx.stamps.capacity(), stamp_capacity);

        ctx.stamp_rhs(7, 8.0);
        ctx.stamp_rhs(9, 10.0);
        let rhs_capacity = ctx.rhs.capacity();
        let rhs = ctx.drain_rhs().collect::<Vec<_>>();
        assert_eq!(rhs, vec![(7, 8.0), (9, 10.0)]);
        assert!(ctx.rhs.is_empty());
        assert_eq!(ctx.rhs.capacity(), rhs_capacity);
    }

    #[test]
    fn analog_ramp_factor_matches_ngspice_transient_rules() {
        let mut ctx = CmContext::new();
        ctx.set_ramptime(10.0e-9);
        ctx.time = 5.0e-9;
        assert_eq!(ctx.analog_ramp_factor(), 1.0);

        ctx.analysis = AnalysisType::Transient;
        assert_eq!(ctx.analog_ramp_factor(), 0.5);

        ctx.time = 10.0e-9;
        assert_eq!(ctx.analog_ramp_factor(), 1.0);

        ctx.set_ramptime(0.0);
        ctx.time = 1.0e-12;
        assert_eq!(ctx.analog_ramp_factor(), 1.0);
    }

    #[test]
    fn transient_history_replaces_same_time_rolls_back_and_prunes_old_samples() {
        let mut ctx = CmContext::new();

        ctx.record_transient_history("line", 0.0, vec![0.0], 3.0);
        ctx.record_transient_history("line", 1.0, vec![1.0], 3.0);
        ctx.record_transient_history("line", 1.0, vec![1.5], 3.0);
        ctx.record_transient_history("line", 3.5, vec![3.5], 3.0);

        assert_eq!(
            ctx.transient_history_at_or_after("line", 0.0),
            Some(vec![1.5])
        );
        assert_eq!(
            ctx.transient_history_at_or_after("line", 2.0),
            Some(vec![3.5])
        );

        ctx.record_transient_history("line", 2.5, vec![2.5], 3.0);

        assert_eq!(ctx.transient_history_at_or_after("line", 3.0), None);
        assert_eq!(
            ctx.transient_history_at_or_after("line", 2.0),
            Some(vec![2.5])
        );
        assert_eq!(
            ctx.transient_history_values_at_or_after("line", 2.0),
            Some([2.5].as_slice())
        );
    }
}
