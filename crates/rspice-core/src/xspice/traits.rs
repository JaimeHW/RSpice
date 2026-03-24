//! XSPICE Code Model Traits
//!
//! Defines the core interfaces for XSPICE code models.
//! All built-in and external code models implement these traits.

use crate::Value;
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
    /// Analog current (i)
    Current,
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
            PortType::Voltage | PortType::DifferentialVoltage | PortType::Current
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
    /// Minimum value (for numeric types)
    pub min: Option<Value>,
    /// Maximum value (for numeric types)
    pub max: Option<Value>,
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
            min: None,
            max: None,
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
            min: None,
            max: None,
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
            min: None,
            max: None,
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
            min: None,
            max: None,
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
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_spec_creation() {
        let port = PortSpec::input("in", PortType::Voltage);
        assert_eq!(port.name, "in");
        assert_eq!(port.direction, PortDirection::In);
        assert_eq!(port.default_type, PortType::Voltage);
        assert!(!port.is_vector);

        let vector_port = PortSpec::vector_input("data", PortType::Digital);
        assert!(vector_port.is_vector);
    }

    #[test]
    fn test_param_spec_creation() {
        let param = ParamSpec::real("gain", 1.0)
            .with_range(0.0, 1000.0)
            .with_description("Voltage gain factor");
        assert_eq!(param.name, "gain");
        assert_eq!(param.default, 1.0);
        assert_eq!(param.min, Some(0.0));
        assert_eq!(param.max, Some(1000.0));
    }

    #[test]
    fn test_port_type_classification() {
        assert!(PortType::Voltage.is_analog());
        assert!(PortType::Current.is_analog());
        assert!(!PortType::Digital.is_analog());

        assert!(PortType::Digital.is_event_driven());
        assert!(PortType::Real.is_event_driven());
        assert!(!PortType::Voltage.is_event_driven());
    }

    #[test]
    fn test_cm_error_display() {
        let err = CmError::InvalidParameter {
            name: "gain".to_string(),
            message: "must be positive".to_string(),
        };
        assert!(err.to_string().contains("gain"));
        assert!(err.to_string().contains("must be positive"));
    }
}
