//! Type System for Verilog-A/AMS
//!
//! Provides type representation, inference, and compatibility checking
//! following the Verilog-A LRM 2.4 type rules.

use smol_str::SmolStr;
use std::fmt;

/// Value types in Verilog-A
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueType {
    /// IEEE 754 double precision floating point
    Real,
    /// Signed 32-bit integer
    Integer,
    /// String literal
    String,
    /// Boolean (result of comparisons)
    Boolean,
    /// Nature access (V, I, etc.)
    NatureAccess,
    /// Void (for statements without value)
    Void,
    /// Type could not be determined
    Unknown,
    /// Type error occurred
    Error,
}

impl ValueType {
    /// Check if this type can be coerced to another
    pub fn can_coerce_to(&self, target: &ValueType) -> bool {
        if self == target {
            return true;
        }
        match (self, target) {
            // Integer can coerce to Real
            (ValueType::Integer, ValueType::Real) => true,
            // Boolean can coerce to Integer (0/1) then to Real
            (ValueType::Boolean, ValueType::Integer) => true,
            (ValueType::Boolean, ValueType::Real) => true,
            // Error and Unknown are compatible with anything
            (ValueType::Error, _) | (_, ValueType::Error) => true,
            (ValueType::Unknown, _) | (_, ValueType::Unknown) => true,
            _ => false,
        }
    }

    /// Get the common type for binary operations
    pub fn common_type(self, other: ValueType) -> ValueType {
        if self == other {
            return self;
        }
        match (self, other) {
            // Errors propagate
            (ValueType::Error, _) | (_, ValueType::Error) => ValueType::Error,
            // Unknown defers to the known type
            (ValueType::Unknown, t) | (t, ValueType::Unknown) => t,
            // Integer + Real = Real
            (ValueType::Integer, ValueType::Real) | (ValueType::Real, ValueType::Integer) => {
                ValueType::Real
            }
            // Boolean promotes to Integer
            (ValueType::Boolean, ValueType::Integer) | (ValueType::Integer, ValueType::Boolean) => {
                ValueType::Integer
            }
            // Boolean promotes to Real through Integer
            (ValueType::Boolean, ValueType::Real) | (ValueType::Real, ValueType::Boolean) => {
                ValueType::Real
            }
            // Incompatible types
            _ => ValueType::Error,
        }
    }

    /// Check if this type is numeric (can participate in arithmetic)
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            ValueType::Real | ValueType::Integer | ValueType::Boolean | ValueType::NatureAccess
        )
    }

    /// Check if this type is a valid condition
    pub fn is_condition(&self) -> bool {
        matches!(
            self,
            ValueType::Boolean
                | ValueType::Real
                | ValueType::Integer
                | ValueType::Unknown
                | ValueType::Error
        )
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Real => write!(f, "real"),
            ValueType::Integer => write!(f, "integer"),
            ValueType::String => write!(f, "string"),
            ValueType::Boolean => write!(f, "boolean"),
            ValueType::NatureAccess => write!(f, "nature_access"),
            ValueType::Void => write!(f, "void"),
            ValueType::Unknown => write!(f, "unknown"),
            ValueType::Error => write!(f, "<error>"),
        }
    }
}

/// Array type information
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
    pub element_type: ValueType,
    pub dimensions: Vec<ArrayDimension>,
}

/// Array dimension
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayDimension {
    pub lower: i32,
    pub upper: i32,
}

impl ArrayDimension {
    pub fn size(&self) -> usize {
        (self.upper - self.lower + 1).max(0) as usize
    }
}

/// Parameter range constraint
#[derive(Debug, Clone)]
pub struct ParameterRange {
    /// Minimum value (exclusive if `min_exclusive`)
    pub min: Option<f64>,
    /// Maximum value (exclusive if `max_exclusive`)
    pub max: Option<f64>,
    /// Whether minimum is exclusive (uses `from (min:max)` vs `from [min:max]`)
    pub min_exclusive: bool,
    /// Whether maximum is exclusive
    pub max_exclusive: bool,
    /// Excluded values
    pub exclude: Vec<f64>,
}

impl ParameterRange {
    /// Create an unrestricted range
    pub fn unrestricted() -> Self {
        Self {
            min: None,
            max: None,
            min_exclusive: false,
            max_exclusive: false,
            exclude: Vec::new(),
        }
    }

    /// Create a range (min, max) exclusive
    pub fn open(min: f64, max: f64) -> Self {
        Self {
            min: Some(min),
            max: Some(max),
            min_exclusive: true,
            max_exclusive: true,
            exclude: Vec::new(),
        }
    }

    /// Create a range [min, max] inclusive
    pub fn closed(min: f64, max: f64) -> Self {
        Self {
            min: Some(min),
            max: Some(max),
            min_exclusive: false,
            max_exclusive: false,
            exclude: Vec::new(),
        }
    }

    /// Create a range (min, max] left-exclusive
    pub fn left_open(min: f64, max: f64) -> Self {
        Self {
            min: Some(min),
            max: Some(max),
            min_exclusive: true,
            max_exclusive: false,
            exclude: Vec::new(),
        }
    }

    /// Check if a value is within this range
    pub fn contains(&self, value: f64) -> bool {
        // NaN is never a valid parameter value (commercial simulator behavior)
        if value.is_nan() {
            return false;
        }

        // Check minimum
        if let Some(min) = self.min {
            if self.min_exclusive {
                if value <= min {
                    return false;
                }
            } else if value < min {
                return false;
            }
        }

        // Check maximum
        if let Some(max) = self.max {
            if self.max_exclusive {
                if value >= max {
                    return false;
                }
            } else if value > max {
                return false;
            }
        }

        // Check exclusions
        if self.exclude.contains(&value) {
            return false;
        }

        true
    }

    /// Check if a value is positive infinity indicator
    pub fn is_inf(v: f64) -> bool {
        v == f64::INFINITY
    }

    /// Check if a value is negative infinity indicator
    pub fn is_neg_inf(v: f64) -> bool {
        v == f64::NEG_INFINITY
    }
}

impl fmt::Display for ParameterRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "from ")?;

        let open = if self.min_exclusive { "(" } else { "[" };
        let close = if self.max_exclusive { ")" } else { "]" };

        match (self.min, self.max) {
            (Some(min), Some(max)) => write!(f, "{open}{min}:{max}{close}")?,
            (Some(min), None) => write!(f, "{open}{min}:inf)")?,
            (None, Some(max)) => write!(f, "(-inf:{max}{close}")?,
            (None, None) => write!(f, "(-inf:inf)")?,
        }

        if !self.exclude.is_empty() {
            write!(f, " exclude ")?;
            for (i, v) in self.exclude.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{v}")?;
            }
        }

        Ok(())
    }
}

/// Function signature for built-in and user-defined functions
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: SmolStr,
    pub params: Vec<FunctionParam>,
    pub return_type: ValueType,
    pub is_analog_operator: bool,
}

/// Function parameter specification
#[derive(Debug, Clone)]
pub struct FunctionParam {
    pub name: SmolStr,
    pub param_type: ValueType,
    pub is_optional: bool,
    pub default: Option<f64>,
}

impl FunctionSignature {
    /// Create a simple math function signature (one real arg, returns real)
    pub fn math_unary(name: &str) -> Self {
        Self {
            name: name.into(),
            params: vec![FunctionParam {
                name: "x".into(),
                param_type: ValueType::Real,
                is_optional: false,
                default: None,
            }],
            return_type: ValueType::Real,
            is_analog_operator: false,
        }
    }

    /// Create a binary math function signature
    pub fn math_binary(name: &str) -> Self {
        Self {
            name: name.into(),
            params: vec![
                FunctionParam {
                    name: "x".into(),
                    param_type: ValueType::Real,
                    is_optional: false,
                    default: None,
                },
                FunctionParam {
                    name: "y".into(),
                    param_type: ValueType::Real,
                    is_optional: false,
                    default: None,
                },
            ],
            return_type: ValueType::Real,
            is_analog_operator: false,
        }
    }

    /// Create an analog operator signature (e.g., ddt, idt)
    pub fn analog_operator(name: &str, num_args: usize) -> Self {
        let params: Vec<_> = (0..num_args)
            .map(|i| FunctionParam {
                name: format!("arg{}", i).into(),
                param_type: ValueType::Real,
                is_optional: i > 0,
                default: None,
            })
            .collect();

        Self {
            name: name.into(),
            params,
            return_type: ValueType::Real,
            is_analog_operator: true,
        }
    }

    /// Get the minimum required argument count
    pub fn min_args(&self) -> usize {
        self.params.iter().filter(|p| !p.is_optional).count()
    }

    /// Get the maximum allowed argument count
    pub fn max_args(&self) -> usize {
        self.params.len()
    }
}

/// Registry of built-in function signatures
#[derive(Debug, Clone)]
pub struct FunctionRegistry {
    functions: std::collections::HashMap<SmolStr, FunctionSignature>,
}

impl FunctionRegistry {
    /// Create a new registry with all standard Verilog-A functions
    pub fn new() -> Self {
        let mut functions = std::collections::HashMap::new();

        // Math functions
        for name in &[
            "abs", "sqrt", "exp", "ln", "log", "sin", "cos", "tan", "asin", "acos", "atan", "sinh",
            "cosh", "tanh", "asinh", "acosh", "atanh", "floor", "ceil", "limexp",
        ] {
            functions.insert((*name).into(), FunctionSignature::math_unary(name));
        }

        // Binary math functions
        for name in &["pow", "hypot", "atan2", "min", "max"] {
            functions.insert((*name).into(), FunctionSignature::math_binary(name));
        }

        // Analog operators
        functions.insert("ddt".into(), FunctionSignature::analog_operator("ddt", 2));
        functions.insert("idt".into(), FunctionSignature::analog_operator("idt", 4));
        functions.insert(
            "idtmod".into(),
            FunctionSignature::analog_operator("idtmod", 5),
        );
        functions.insert("ddx".into(), FunctionSignature::analog_operator("ddx", 2));
        functions.insert(
            "laplace_zp".into(),
            FunctionSignature::analog_operator("laplace_zp", 4),
        );
        functions.insert(
            "laplace_zd".into(),
            FunctionSignature::analog_operator("laplace_zd", 4),
        );
        functions.insert(
            "laplace_np".into(),
            FunctionSignature::analog_operator("laplace_np", 4),
        );
        functions.insert(
            "laplace_nd".into(),
            FunctionSignature::analog_operator("laplace_nd", 4),
        );

        // Noise functions
        functions.insert(
            "white_noise".into(),
            FunctionSignature {
                name: "white_noise".into(),
                params: vec![
                    FunctionParam {
                        name: "pwr".into(),
                        param_type: ValueType::Real,
                        is_optional: false,
                        default: None,
                    },
                    FunctionParam {
                        name: "name".into(),
                        param_type: ValueType::String,
                        is_optional: true,
                        default: None,
                    },
                ],
                return_type: ValueType::Real,
                is_analog_operator: true,
            },
        );

        functions.insert(
            "flicker_noise".into(),
            FunctionSignature {
                name: "flicker_noise".into(),
                params: vec![
                    FunctionParam {
                        name: "pwr".into(),
                        param_type: ValueType::Real,
                        is_optional: false,
                        default: None,
                    },
                    FunctionParam {
                        name: "exp".into(),
                        param_type: ValueType::Real,
                        is_optional: false,
                        default: None,
                    },
                    FunctionParam {
                        name: "name".into(),
                        param_type: ValueType::String,
                        is_optional: true,
                        default: None,
                    },
                ],
                return_type: ValueType::Real,
                is_analog_operator: true,
            },
        );

        Self { functions }
    }

    /// Look up a function signature
    pub fn get(&self, name: &str) -> Option<&FunctionSignature> {
        self.functions.get(name)
    }

    /// Check if a function exists
    pub fn contains(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

