use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SavedOutputId};
use crate::state::{
    SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming,
};

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
    /// The exact recipe is retained and can be evaluated against the source
    /// data in this same immutable analysis result.
    Deferred,
    /// A failure-only contract was intentionally inactive on a successful
    /// analysis.
    SuppressedOnSuccess,
    /// The contract applied, but its required source evidence was absent.
    Unavailable { reason: String },
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
    pub status: SavedOutputMaterializationStatus,
}
