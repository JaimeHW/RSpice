//! Serializable summaries of parser diagnostics and the readiness probe.
//!
//! Analysis results are deliberately not here. There is one result document
//! in this crate and it is `rspice_core::execution::AnalysisResultDocument`,
//! projected for JavaScript by [`crate::document`].

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetlistSummary {
    pub title: String,
    pub element_count: usize,
    pub analysis_count: usize,
    pub model_count: usize,
    pub subcircuit_count: usize,
    pub parameter_count: usize,
    pub diagnostics: Vec<WasmDiagnostic>,
    #[serde(default)]
    pub startup_diagnostics: Vec<WasmStartupDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmDiagnostic {
    pub line: usize,
    pub severity: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmSourceLocation {
    pub source: Option<String>,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmStartupDirectiveScope {
    pub kind: String,
    pub qualified_definition: Option<String>,
    pub qualified_instances: Vec<String>,
}

/// Stable structured representation of a non-fatal `.IC`/`.NODESET`
/// semantic diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmStartupDiagnostic {
    pub code: String,
    pub stage: String,
    pub directive: String,
    pub origins: Vec<WasmSourceLocation>,
    pub scopes: Vec<WasmStartupDirectiveScope>,
    pub canonical_nodes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WasmHealthReport {
    pub status: String,
    pub ready: bool,
    pub duration_seconds: f64,
    pub element_count: usize,
    pub node_count: usize,
    pub branch_count: usize,
    pub output_voltage: f64,
}
