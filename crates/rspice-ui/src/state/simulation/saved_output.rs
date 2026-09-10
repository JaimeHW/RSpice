//! Saved outputs.
//!
//! The outputs a plan asked to keep, and whether each was actually
//! materialized by the run. An output that was requested but not produced is
//! reported rather than silently missing.

use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SavedOutputId};
use crate::state::{
    SavedOutputDisplayIntent, SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision,
    SavedOutputStreaming,
};

mod source_bindings;
mod validation;
pub(crate) use source_bindings::saved_output_references;
pub use source_bindings::{SavedOutputAxis, SavedOutputBoundSource, SavedOutputSourceBindings};

/// Durable outcome of applying one immutable saved-output contract to an
/// analysis result. Receipts are persisted with the dataset so result viewers
/// never have to infer which live project configuration produced a trace.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SavedOutputMaterializationStatus {
    /// The named waveform was materialized into `AnalysisResult::waveforms`.
    Materialized {
        waveform_name: String,
        sample_count: u64,
    },
    /// One authored output evaluated independently for every retained DC
    /// member. Indices refer to this analysis' exact DC sweep evidence.
    MaterializedDcFamily { members: Vec<SavedOutputDcMember> },
    /// The exact recipe is retained and can be evaluated against the source
    /// data in this same immutable analysis result.
    Deferred,
    /// A failure-only contract was intentionally inactive on a successful
    /// analysis.
    SuppressedOnSuccess,
    /// The contract applied, but its required source evidence was absent.
    Unavailable { reason: String },
}

/// One member's samples, with no copy of its coordinate or numerical arrays.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedOutputDcMember {
    pub member: usize,
    pub waveform_name: String,
    pub sample_count: u64,
}

impl SavedOutputMaterializationStatus {
    pub fn materialized_waveforms(&self) -> impl Iterator<Item = (&str, u64)> {
        let single = match self {
            Self::Materialized {
                waveform_name,
                sample_count,
            } => Some((waveform_name.as_str(), *sample_count)),
            _ => None,
        };
        let members = match self {
            Self::MaterializedDcFamily { members } => members.as_slice(),
            _ => &[],
        };
        single.into_iter().chain(
            members
                .iter()
                .map(|member| (member.waveform_name.as_str(), member.sample_count)),
        )
    }
}

/// Immutable provenance for a single saved output in one retained analysis.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedOutputReceipt {
    pub output_id: SavedOutputId,
    pub output_revision: ObjectRevision,
    pub analysis_id: AnalysisInstanceId,
    pub contract_digest: ContentDigest,
    pub name: String,
    pub source_expression: String,
    pub output_kind: SavedOutputKind,
    pub save_policy: SavedOutputPolicy,
    pub stored_precision: SavedOutputPrecision,
    pub streaming: SavedOutputStreaming,
    #[serde(default)]
    pub display_intent: SavedOutputDisplayIntent,
    /// Absent only for historical receipts whose physical bindings are unknown.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "source_bindings::deserialize_bindings"
    )]
    pub source_bindings: Option<SavedOutputSourceBindings>,
    pub status: SavedOutputMaterializationStatus,
}
