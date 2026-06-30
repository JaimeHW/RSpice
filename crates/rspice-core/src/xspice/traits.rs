//! XSPICE Code Model Traits
//!
//! Defines the core interfaces for XSPICE code models.
//! All built-in and external code models implement these traits.

use crate::{Complex64, Value};
use std::fmt;

//=============================================================================
// Error Types
//=============================================================================

/// Result type for code model operations
pub type CmResult<T> = Result<T, CmError>;

/// Error type for code model operations
#[derive(Debug, Clone)]
pub enum CmError {
    /// Invalid parameter value
    InvalidParameter { name: String, message: String },
    /// Port type mismatch
    PortTypeMismatch {
        port: String,
        expected: PortType,
        actual: PortType,
    },
    /// Port count mismatch
    PortCountMismatch { expected: usize, actual: usize },
    /// Missing required parameter
    MissingParameter(String),
    /// Invalid port connection
    InvalidPortConnection(String),
    /// Model evaluation error
    EvaluationError(String),
    /// Convergence failure in model
    ConvergenceFailure(String),
    /// Internal model error
    Internal(String),
}

impl fmt::Display for CmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CmError::InvalidParameter { name, message } => {
                write!(f, "Invalid parameter '{}': {}", name, message)
            }
            CmError::PortTypeMismatch {
                port,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Port '{}' type mismatch: expected {:?}, got {:?}",
                    port, expected, actual
                )
            }
            CmError::PortCountMismatch { expected, actual } => {
                write!(
                    f,
                    "Port count mismatch: expected {}, got {}",
                    expected, actual
                )
            }
            CmError::MissingParameter(name) => {
                write!(f, "Missing required parameter: {}", name)
            }
            CmError::InvalidPortConnection(msg) => {
                write!(f, "Invalid port connection: {}", msg)
            }
            CmError::EvaluationError(msg) => {
                write!(f, "Model evaluation error: {}", msg)
            }
            CmError::ConvergenceFailure(msg) => {
                write!(f, "Convergence failure: {}", msg)
            }
            CmError::Internal(msg) => {
                write!(f, "Internal error: {}", msg)
            }
        }
    }
}

impl std::error::Error for CmError {}

//=============================================================================
// Port Types
//=============================================================================

/// Port connection type for XSPICE code models
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortType {
    /// Single-ended analog voltage (v)
    Voltage,
    /// Differential analog voltage pair (vd)
    DifferentialVoltage,
    /// Single-ended analog conductance terminal (g)
    Conductance,
    /// Differential analog conductance terminal pair (gd)
    DifferentialConductance,
    /// Single-ended hybrid/resistance terminal (h): current input, voltage output
    Hybrid,
    /// Differential hybrid/resistance terminal pair (hd)
    DifferentialHybrid,
    /// Single-ended analog current (i)
    Current,
    /// Differential analog current pair (id)
    DifferentialCurrent,
    /// Voltage source name for current sensing (vnam)
    VoltageName,
    /// Digital signal (d) - 12-state logic
    Digital,
    /// Real-valued event-driven signal (real)
    Real,
    /// Integer event-driven signal (int)
    Integer,
    /// User-defined node type
    UserDefined,
}

impl PortType {
    /// Check if this port type is analog (continuous-time)
    pub fn is_analog(&self) -> bool {
        matches!(
            self,
            PortType::Voltage
                | PortType::DifferentialVoltage
                | PortType::Conductance
                | PortType::DifferentialConductance
                | PortType::Hybrid
                | PortType::DifferentialHybrid
                | PortType::Current
                | PortType::DifferentialCurrent
        )
    }

    /// Check if this port type is event-driven
    pub fn is_event_driven(&self) -> bool {
        matches!(self, PortType::Digital | PortType::Real | PortType::Integer)
    }
}

/// Port direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortDirection {
    /// Input port (read-only)
    In,
    /// Output port (write-only)
    Out,
    /// Bidirectional port
    InOut,
}

/// Port specification for a code model
#[derive(Debug, Clone)]
pub struct PortSpec {
    /// Port name
    pub name: String,
    /// Port direction
    pub direction: PortDirection,
    /// Default port type (used if not explicitly specified)
    pub default_type: PortType,
    /// Allowed port types
    pub allowed_types: Vec<PortType>,
    /// Whether this is a vector port (multiple connections)
    pub is_vector: bool,
    /// Null connection allowed (port can be unconnected)
    pub null_allowed: bool,
    /// Minimum vector connection length, when this is a vector port
    pub vector_min_len: Option<usize>,
    /// Maximum vector connection length, when this is a vector port
    pub vector_max_len: Option<usize>,
    /// Description for documentation
    pub description: String,
}

impl PortSpec {
    /// Create a simple input port
    pub fn input(name: impl Into<String>, port_type: PortType) -> Self {
        Self {
            name: name.into(),
            direction: PortDirection::In,
            default_type: port_type,
            allowed_types: vec![port_type],
            is_vector: false,
            null_allowed: false,
            vector_min_len: None,
            vector_max_len: None,
            description: String::new(),
        }
    }

    /// Create a simple output port
    pub fn output(name: impl Into<String>, port_type: PortType) -> Self {
        Self {
            name: name.into(),
            direction: PortDirection::Out,
            default_type: port_type,
            allowed_types: vec![port_type],
            is_vector: false,
            null_allowed: false,
            vector_min_len: None,
            vector_max_len: None,
            description: String::new(),
        }
    }

    /// Create a vector input port
    pub fn vector_input(name: impl Into<String>, port_type: PortType) -> Self {
        Self {
            name: name.into(),
            direction: PortDirection::In,
            default_type: port_type,
            allowed_types: vec![port_type],
            is_vector: true,
            null_allowed: false,
            vector_min_len: None,
            vector_max_len: None,
            description: String::new(),
        }
    }

    /// Create a vector output port  
    pub fn vector_output(name: impl Into<String>, port_type: PortType) -> Self {
        Self {
            name: name.into(),
            direction: PortDirection::Out,
            default_type: port_type,
            allowed_types: vec![port_type],
            is_vector: true,
            null_allowed: false,
            vector_min_len: None,
            vector_max_len: None,
            description: String::new(),
        }
    }

    /// Mark port as allowing null connections
    pub fn nullable(mut self) -> Self {
        self.null_allowed = true;
        self
    }

    /// Add description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Require at least this many elements when the port is connected as a vector
    pub fn with_vector_min_len(mut self, min_len: usize) -> Self {
        self.vector_min_len = Some(min_len);
        self
    }

    /// Require at most this many elements when the port is connected as a vector
    pub fn with_vector_max_len(mut self, max_len: usize) -> Self {
        self.vector_max_len = Some(max_len);
        self
    }

    /// Require a bounded vector length range when the port is connected as a vector
    pub fn with_vector_len_range(mut self, min_len: usize, max_len: usize) -> Self {
        self.vector_min_len = Some(min_len);
        self.vector_max_len = Some(max_len);
        self
    }
}

//=============================================================================
// Parameter Types
//=============================================================================

/// Parameter value type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParamType {
    /// Real number (f64)
    Real,
    /// Integer
    Integer,
    /// Boolean
    Boolean,
    /// String (path, identifier, etc.)
    String,
    /// Vector of strings
    StringVector,
    /// Vector of real numbers
    RealVector,
    /// Vector of integers
    IntegerVector,
}

/// Parameter specification for a code model
#[derive(Debug, Clone)]
pub struct ParamSpec {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: ParamType,
    /// Default value (as f64 for numeric, stored differently for strings)
    pub default: Value,
    /// Default string value for string-typed parameters
    pub string_default: Option<String>,
    /// Default string-vector value for string-vector parameters
    pub string_vector_default: Option<Vec<String>>,
    /// Default real-vector value for vector-typed parameters
    pub real_vector_default: Option<Vec<Value>>,
    /// Default integer-vector value for vector-typed parameters
    pub integer_vector_default: Option<Vec<i64>>,
    /// Minimum value (for numeric types)
    pub min: Option<Value>,
    /// Maximum value (for numeric types)
    pub max: Option<Value>,
    /// Minimum vector length (for vector types)
    pub vector_min_len: Option<usize>,
    /// Maximum vector length (for vector types)
    pub vector_max_len: Option<usize>,
    /// Whether this parameter is required
    pub required: bool,
    /// Description for documentation
    pub description: String,
}

impl ParamSpec {
    /// Create a real parameter with default
    pub fn real(name: impl Into<String>, default: Value) -> Self {
        Self {
            name: name.into(),
            param_type: ParamType::Real,
            default,
            string_default: None,
            string_vector_default: None,
            real_vector_default: None,
            integer_vector_default: None,
            min: None,
            max: None,
            vector_min_len: None,
            vector_max_len: None,
            required: false,
            description: String::new(),
        }
    }

    /// Create an integer parameter with default
    pub fn integer(name: impl Into<String>, default: i64) -> Self {
        Self {
            name: name.into(),
            param_type: ParamType::Integer,
            default: default as f64,
            string_default: None,
            string_vector_default: None,
            real_vector_default: None,
            integer_vector_default: None,
            min: None,
            max: None,
            vector_min_len: None,
            vector_max_len: None,
            required: false,
            description: String::new(),
        }
    }

    /// Create a boolean parameter with default
    pub fn boolean(name: impl Into<String>, default: bool) -> Self {
        Self {
            name: name.into(),
            param_type: ParamType::Boolean,
            default: if default { 1.0 } else { 0.0 },
            string_default: None,
            string_vector_default: None,
            real_vector_default: None,
            integer_vector_default: None,
            min: None,
            max: None,
            vector_min_len: None,
            vector_max_len: None,
            required: false,
            description: String::new(),
        }
    }

    /// Create a string parameter with default
    pub fn string(name: impl Into<String>, default: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            param_type: ParamType::String,
            default: 0.0,
            string_default: Some(default.into()),
            string_vector_default: None,
            real_vector_default: None,
            integer_vector_default: None,
            min: None,
            max: None,
            vector_min_len: None,
            vector_max_len: None,
            required: false,
            description: String::new(),
        }
    }

    /// Create a real-vector parameter with default
    pub fn real_vector(name: impl Into<String>, default: Vec<Value>) -> Self {
        Self {
            name: name.into(),
            param_type: ParamType::RealVector,
            default: 0.0,
            string_default: None,
            string_vector_default: None,
            real_vector_default: Some(default),
            integer_vector_default: None,
            min: None,
            max: None,
            vector_min_len: None,
            vector_max_len: None,
            required: false,
            description: String::new(),
        }
    }

    /// Create an integer-vector parameter with default
    pub fn integer_vector(name: impl Into<String>, default: Vec<i64>) -> Self {
        Self {
            name: name.into(),
            param_type: ParamType::IntegerVector,
            default: 0.0,
            string_default: None,
            string_vector_default: None,
            real_vector_default: None,
            integer_vector_default: Some(default),
            min: None,
            max: None,
            vector_min_len: None,
            vector_max_len: None,
            required: false,
            description: String::new(),
        }
    }

    /// Create a string-vector parameter with default
    pub fn string_vector(name: impl Into<String>, default: Vec<String>) -> Self {
        Self {
            name: name.into(),
            param_type: ParamType::StringVector,
            default: 0.0,
            string_default: None,
            string_vector_default: Some(default),
            real_vector_default: None,
            integer_vector_default: None,
            min: None,
            max: None,
            vector_min_len: None,
            vector_max_len: None,
            required: false,
            description: String::new(),
        }
    }

    /// Set minimum value
    pub fn with_min(mut self, min: Value) -> Self {
        self.min = Some(min);
        self
    }

    /// Set maximum value
    pub fn with_max(mut self, max: Value) -> Self {
        self.max = Some(max);
        self
    }

    /// Set value range
    pub fn with_range(mut self, min: Value, max: Value) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    /// Set minimum vector length
    pub fn with_vector_min_len(mut self, min_len: usize) -> Self {
        self.vector_min_len = Some(min_len);
        self
    }

    /// Set maximum vector length
    pub fn with_vector_max_len(mut self, max_len: usize) -> Self {
        self.vector_max_len = Some(max_len);
        self
    }

    /// Set vector length range
    pub fn with_vector_len_range(mut self, min_len: usize, max_len: usize) -> Self {
        self.vector_min_len = Some(min_len);
        self.vector_max_len = Some(max_len);
        self
    }

    /// Mark as required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Add description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
}

//=============================================================================
// Code Model Trait
//=============================================================================

/// The main trait for XSPICE code models
///
/// All code models (built-in and external) implement this trait.
/// The trait provides introspection for ports and parameters,
/// and the `evaluate` method is called during simulation.
pub trait CodeModel: Send + Sync {
    /// Get the model name (e.g., "d_source", "gain")
    fn name(&self) -> &str;

    /// Get the model description
    fn description(&self) -> &str {
        ""
    }

    /// Get port specifications
    fn ports(&self) -> &[PortSpec];

    /// Get parameter specifications
    fn parameters(&self) -> &[ParamSpec];

    /// Check if this model is analog-only (no event-driven ports)
    fn is_analog_only(&self) -> bool {
        self.ports().iter().all(|p| p.default_type.is_analog())
    }

    /// Check if this model is digital-only (all event-driven ports)
    fn is_digital_only(&self) -> bool {
        self.ports()
            .iter()
            .all(|p| p.default_type.is_event_driven())
    }

    /// Whether this code model needs global conservative Newton damping.
    ///
    /// Most XSPICE models are behavioral sources with explicit operating-point
    /// linearization. Treating them like compact semiconductor devices can
    /// prevent ideal source branch equations from taking their exact Newton
    /// step. Models that implement stiff physical device equations can opt in.
    fn requires_conservative_newton_damping(&self) -> bool {
        false
    }

    /// Initialize instance state
    ///
    /// Called once when the instance is created.
    /// Can be used to allocate internal state, validate parameters, etc.
    fn init(&self, ctx: &mut super::CmContext) -> CmResult<()>;

    /// Evaluate the model
    ///
    /// Called during simulation to compute outputs from inputs.
    /// May also modify internal state for stateful models.
    fn evaluate(&self, ctx: &mut super::CmContext) -> CmResult<()>;

    /// Get AC gain (for small-signal analysis)
    ///
    /// Returns the linearized transfer function for each output.
    /// Default implementation returns unity gain.
    fn ac_gain(&self, _ctx: &super::CmContext) -> Vec<Value> {
        vec![
            1.0;
            self.ports()
                .iter()
                .filter(|p| p.direction == PortDirection::Out)
                .count()
        ]
    }

    /// Whether a voltage output should be excluded from generic node-voltage
    /// LTE control during transient analysis.
    ///
    /// Explicit step-history code models can intentionally return accepted
    /// sample values rather than continuous-time state. Their output branch
    /// equation is still enforced by Newton residual stamping, but generic
    /// voltage extrapolation treats the sample history as physical curvature
    /// and can drive timesteps to zero.
    fn excludes_output_from_transient_voltage_lte(&self, _output_port: &str) -> bool {
        false
    }

    /// Linearized control partials for a voltage output branch equation.
    ///
    /// Each tuple is `(input_port_name, d(output)/d(input_port))`. The circuit
    /// stamper uses these to form `v(out) - f(inputs) = 0` instead of treating
    /// controlled voltage outputs as iteration-only independent sources.
    fn output_input_partials(
        &self,
        _ctx: &super::CmContext,
        _output_port: &str,
    ) -> Vec<(String, Value)> {
        Vec::new()
    }

    /// Linearized control partials for analog vector input ports.
    ///
    /// Each tuple is `(input_port_name, element_index, d(output)/d(input[element_index]))`.
    /// This is required for official vector-input code models such as
    /// `multi_input_pwl`, where only one vector element may be the controlling
    /// input for the current operating point.
    fn output_input_vector_partials(
        &self,
        _ctx: &super::CmContext,
        _output_port: &str,
    ) -> Vec<(String, usize, Value)> {
        Vec::new()
    }

    /// Linearized scalar-input control partials for one element of an analog
    /// vector output port.
    ///
    /// Existing scalar-output models remain compatible through the default
    /// delegation. Vector-output models that need different Jacobian rows per
    /// output element should override this method.
    fn output_vector_input_partials(
        &self,
        ctx: &super::CmContext,
        output_port: &str,
        _output_index: usize,
    ) -> Vec<(String, Value)> {
        self.output_input_partials(ctx, output_port)
    }

    /// Linearized vector-input control partials for one element of an analog
    /// vector output port.
    fn output_vector_input_vector_partials(
        &self,
        ctx: &super::CmContext,
        output_port: &str,
        _output_index: usize,
    ) -> Vec<(String, usize, Value)> {
        self.output_input_vector_partials(ctx, output_port)
    }

    /// Complex small-signal control partials for a scalar analog output.
    ///
    /// Most code models are real-valued in AC and use the default adapter from
    /// the DC/Newton partials. Frequency-domain models such as `xfer` and
    /// `s_xfer` override this to stamp phase-bearing transfer functions.
    fn output_input_ac_partials(
        &self,
        ctx: &super::CmContext,
        output_port: &str,
        _frequency: Value,
    ) -> Vec<(String, Complex64)> {
        self.output_input_partials(ctx, output_port)
            .into_iter()
            .map(|(port, partial)| (port, Complex64::new(partial, 0.0)))
            .collect()
    }

    /// Complex small-signal vector-input partials for a scalar analog output.
    fn output_input_vector_ac_partials(
        &self,
        ctx: &super::CmContext,
        output_port: &str,
        _frequency: Value,
    ) -> Vec<(String, usize, Complex64)> {
        self.output_input_vector_partials(ctx, output_port)
            .into_iter()
            .map(|(port, index, partial)| (port, index, Complex64::new(partial, 0.0)))
            .collect()
    }

    /// Complex small-signal scalar-input partials for a vector output element.
    fn output_vector_input_ac_partials(
        &self,
        ctx: &super::CmContext,
        output_port: &str,
        output_index: usize,
        _frequency: Value,
    ) -> Vec<(String, Complex64)> {
        self.output_vector_input_partials(ctx, output_port, output_index)
            .into_iter()
            .map(|(port, partial)| (port, Complex64::new(partial, 0.0)))
            .collect()
    }

    /// Complex small-signal vector-input partials for a vector output element.
    fn output_vector_input_vector_ac_partials(
        &self,
        ctx: &super::CmContext,
        output_port: &str,
        output_index: usize,
        _frequency: Value,
    ) -> Vec<(String, usize, Complex64)> {
        self.output_vector_input_vector_partials(ctx, output_port, output_index)
            .into_iter()
            .map(|(port, index, partial)| (port, index, Complex64::new(partial, 0.0)))
            .collect()
    }

    /// Absolute transient breakpoint times owned by this code model instance.
    fn transient_breakpoints(&self, _ctx: &super::CmContext) -> CmResult<Vec<Value>> {
        Ok(Vec::new())
    }
}

//=============================================================================
// Tests
//=============================================================================
