//! Versioned, portable study inputs over a circuit and explicitly named tasks.
//!
//! This format owns typed requests, never generated netlist snippets or editor
//! drafts. Decoding rejects unknown fields even inside the shared analysis
//! specifications. Lowering and source preparation use the same contracts as
//! application execution; decoding alone does not authorize a run.

use std::path::PathBuf;

use rspice_app_types::product::SimulationPlanId;
use rspice_core::abort_signal::AbortSignal;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_simulation_contract::numeric_override::AnalysisNumericOverride;
use rspice_simulation_contract::output_policy::SimulationSavePolicy;
use rspice_simulation_contract::saved_output::SavedOutput;
use serde::{Deserialize, Serialize};

use crate::periodic::{PacRunConfig, PnoiseRunConfig, PstbRunConfig, PxfRunConfig};

mod lowering;
#[cfg(test)]
mod tests;

pub const STUDY_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StudyDocument {
    pub schema_version: u32,
    /// Stable namespace for task identities. Editing settings does not rename
    /// the task; its authenticated configuration digest records that change.
    pub id: SimulationPlanId,
    pub circuit: StudyCircuit,
    pub tasks: Vec<StudyTask>,
    #[serde(default)]
    pub numeric: AnalysisNumericOverride,
    #[serde(default)]
    pub saved_outputs: Vec<SavedOutput>,
    #[serde(default)]
    pub save_policy: Option<SimulationSavePolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StudyCircuit {
    /// Resolved relative to the study document by its host, never the process
    /// working directory. Browser hosts supply the corresponding sealed bytes.
    pub path: PathBuf,
    /// Search paths relative to the circuit's directory, in priority order.
    #[serde(default)]
    pub include_search_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StudyTask {
    /// A case-sensitive name, independent of array position or display label.
    pub id: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub depends_on: Vec<String>,
    pub analysis: AnalysisSpec,
    #[serde(default)]
    pub periodic: Option<StudyPeriodicSettings>,
    #[serde(default)]
    pub numeric: AnalysisNumericOverride,
}

/// Complete controls for the periodic request kinds whose shared spec names
/// the analysis family and whose numerical settings live in a separate type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "settings",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum StudyPeriodicSettings {
    Pac(PacRunConfig),
    Pxf(PxfRunConfig),
    Pnoise(PnoiseRunConfig),
    Pstb(PstbRunConfig),
}

#[derive(Debug, thiserror::Error)]
pub enum StudyDocumentError {
    #[error("Study document is {bytes} bytes; the configured limit is {limit}")]
    InputTooLarge { bytes: usize, limit: usize },
    #[error("Study preparation aborted")]
    Aborted,
    #[error("Invalid study JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Unknown study field: {0}")]
    UnknownField(String),
    #[error("Unsupported study schema version {0}; supported version is 1")]
    UnsupportedVersion(u32),
    #[error("Invalid study: {0}")]
    Invalid(String),
    #[error(transparent)]
    ResourceLimit(#[from] rspice_core::ResourceLimitError),
}

impl StudyDocument {
    /// Decode one bounded JSON document. Shared persisted types can be lenient
    /// when migrating projects; a study request must never ignore a typo.
    pub fn from_json(
        source: &str,
        byte_limit: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Self, StudyDocumentError> {
        check_abort(abort)?;
        if source.len() > byte_limit {
            return Err(StudyDocumentError::InputTooLarge {
                bytes: source.len(),
                limit: byte_limit,
            });
        }
        let mut decoder = serde_json::Deserializer::from_str(source);
        let mut ignored = None;
        let document: Self = serde_ignored::deserialize(&mut decoder, |path| {
            if ignored.is_none() {
                ignored = Some(path.to_string());
            }
        })?;
        decoder.end()?;
        check_abort(abort)?;
        if let Some(path) = ignored {
            return Err(StudyDocumentError::UnknownField(path));
        }
        document.validate_header()?;
        Ok(document)
    }

    fn validate_header(&self) -> Result<(), StudyDocumentError> {
        if self.schema_version != STUDY_SCHEMA_VERSION {
            return Err(StudyDocumentError::UnsupportedVersion(self.schema_version));
        }
        if self.circuit.path.as_os_str().is_empty() {
            return Err(invalid("circuit.path must not be empty"));
        }
        if self
            .circuit
            .include_search_paths
            .iter()
            .any(|path| path.as_os_str().is_empty())
        {
            return Err(invalid("include search paths must not be empty"));
        }
        if let Some(policy) = self.save_policy {
            policy.validate().map_err(invalid)?;
        }
        Ok(())
    }
}

fn invalid(message: impl Into<String>) -> StudyDocumentError {
    StudyDocumentError::Invalid(message.into())
}

fn check_abort(abort: &dyn AbortSignal) -> Result<(), StudyDocumentError> {
    if abort.is_aborted() {
        Err(StudyDocumentError::Aborted)
    } else {
        Ok(())
    }
}
