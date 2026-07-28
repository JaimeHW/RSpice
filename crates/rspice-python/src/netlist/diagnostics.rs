//! Structured diagnostics a parse or startup check produces.
//!
//! Each carries the source location it came from, so a caller can point at the
//! offending line rather than re-scanning the deck to find it. These are
//! `frozen` pyclasses: a diagnostic describes something that already happened
//! and must not be editable after the fact.

use super::*;

/// Non-fatal parser diagnostic attached to a parsed netlist.
#[pyclass(name = "ParseDiagnostic", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyParseDiagnostic {
    #[pyo3(get)]
    pub line: usize,
    #[pyo3(get)]
    pub severity: String,
    #[pyo3(get)]
    pub code: String,
    #[pyo3(get)]
    pub message: String,
}

impl From<&rspice_core::netlist::ParseDiagnostic> for PyParseDiagnostic {
    fn from(diagnostic: &rspice_core::netlist::ParseDiagnostic) -> Self {
        Self {
            line: diagnostic.line,
            severity: match diagnostic.severity {
                rspice_core::netlist::DiagnosticSeverity::Warning => "warning".to_string(),
            },
            code: diagnostic.code.clone(),
            message: diagnostic.message.clone(),
        }
    }
}

/// Source location retained by a structured netlist diagnostic.
#[pyclass(
    name = "NetlistSourceLocation",
    module = "rspice",
    frozen,
    from_py_object
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyNetlistSourceLocation {
    #[pyo3(get)]
    pub line: usize,
    #[pyo3(get)]
    pub source: Option<String>,
}

impl From<&rspice_core::netlist::NetlistSourceLocation> for PyNetlistSourceLocation {
    fn from(location: &rspice_core::netlist::NetlistSourceLocation) -> Self {
        Self {
            line: location.line,
            source: location
                .path
                .as_ref()
                .map(|path| crate::errors::public_path_string(path)),
        }
    }
}

/// Scope affected by a structured startup-directive diagnostic.
#[pyclass(
    name = "StartupDirectiveScope",
    module = "rspice",
    frozen,
    from_py_object
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyStartupDirectiveScope {
    #[pyo3(get)]
    pub kind: String,
    #[pyo3(get)]
    pub qualified_definition: Option<String>,
    #[pyo3(get)]
    pub qualified_instances: Vec<String>,
}

impl From<&rspice_core::netlist::StartupDirectiveScope> for PyStartupDirectiveScope {
    fn from(scope: &rspice_core::netlist::StartupDirectiveScope) -> Self {
        match scope {
            rspice_core::netlist::StartupDirectiveScope::TopLevel => Self {
                kind: "top_level".to_string(),
                qualified_definition: None,
                qualified_instances: Vec::new(),
            },
            rspice_core::netlist::StartupDirectiveScope::Subcircuit {
                qualified_definition,
                qualified_instances,
            } => Self {
                kind: "subcircuit".to_string(),
                qualified_definition: Some(qualified_definition.clone()),
                qualified_instances: qualified_instances.clone(),
            },
        }
    }
}

/// Structured, non-fatal `.IC`/`.NODESET` semantic diagnostic.
///
/// String tags are stable API values so callers never need to parse the
/// human-readable compatibility warning in `Netlist.diagnostics`.
#[pyclass(name = "StartupDiagnostic", module = "rspice", frozen, from_py_object)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyStartupDiagnostic {
    #[pyo3(get)]
    pub code: String,
    #[pyo3(get)]
    pub stage: String,
    #[pyo3(get)]
    pub directive: String,
    #[pyo3(get)]
    pub origins: Vec<PyNetlistSourceLocation>,
    #[pyo3(get)]
    pub scopes: Vec<PyStartupDirectiveScope>,
    #[pyo3(get)]
    pub canonical_nodes: Vec<String>,
}

impl From<&rspice_core::netlist::StartupDiagnostic> for PyStartupDiagnostic {
    fn from(diagnostic: &rspice_core::netlist::StartupDiagnostic) -> Self {
        use rspice_core::netlist::{StartupDiagnosticStage, StartupDirectiveKind};

        Self {
            code: diagnostic.code.as_str().to_string(),
            stage: match diagnostic.stage {
                StartupDiagnosticStage::Parse => "parse",
                StartupDiagnosticStage::StartupTopology => "startup_topology",
            }
            .to_string(),
            directive: match diagnostic.kind {
                StartupDirectiveKind::Ic => "ic",
                StartupDirectiveKind::NodeSet => "nodeset",
            }
            .to_string(),
            origins: diagnostic.origins.iter().map(Into::into).collect(),
            scopes: diagnostic.scopes.iter().map(Into::into).collect(),
            canonical_nodes: diagnostic.canonical_nodes.clone(),
        }
    }
}
