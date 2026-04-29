use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// =============================================================================
// AST Types
// =============================================================================

/// A parsed library section (corner)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LibrarySection {
    /// Section name (e.g., "tt", "ff")
    pub name: String,
    /// Models defined in this section
    pub models: HashMap<String, ModelDef>,
    /// Subcircuits defined in this section
    pub subcircuits: HashMap<String, SubcircuitDef>,
    /// Parameters defined in this section
    pub parameters: HashMap<String, ParamValue>,
    /// Include directives
    pub includes: Vec<IncludeDirective>,
}

/// Model definition
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelDef {
    /// Model name
    pub name: String,
    /// Model type (nmos, pmos, npn, pnp, r, c, d, etc.)
    pub model_type: String,
    /// Model level
    pub level: Option<i32>,
    /// Version
    pub version: Option<String>,
    /// Parameters
    pub parameters: HashMap<String, ParamValue>,
    /// Source line number
    pub line: usize,
}

/// Subcircuit definition
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubcircuitDef {
    /// Subcircuit name
    pub name: String,
    /// Port names
    pub ports: Vec<String>,
    /// Parameters with defaults
    pub parameters: HashMap<String, ParamValue>,
    /// Internal content (as string for now)
    pub content: String,
    /// Source line number
    pub line: usize,
}

/// Include directive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncludeDirective {
    /// Include type (.include or .lib)
    pub directive_type: IncludeType,
    /// File path
    pub path: PathBuf,
    /// Section name (for .lib)
    pub section: Option<String>,
    /// Source line number
    pub line: usize,
}

/// Type of include directive
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncludeType {
    /// .include - full file inclusion
    Include,
    /// .lib - section-specific inclusion
    Lib,
}

/// Parameter value (can be numeric or expression)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParamValue {
    /// Numeric value
    Number(f64),
    /// String/expression value
    Expression(String),
}

impl ParamValue {
    /// Get as number if possible
    pub fn as_number(&self) -> Option<f64> {
        match self {
            ParamValue::Number(n) => Some(*n),
            ParamValue::Expression(s) => s.parse().ok(),
        }
    }

    /// Get as string
    pub fn as_string(&self) -> String {
        match self {
            ParamValue::Number(n) => n.to_string(),
            ParamValue::Expression(s) => s.clone(),
        }
    }
}
