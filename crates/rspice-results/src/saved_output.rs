//! Policies and source identities retained with saved engineering outputs.

use rspice_app_types::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SavedOutputId};
use serde::{Deserialize, Serialize};

#[cfg(feature = "engine-evidence")]
mod expressions;
#[cfg(feature = "engine-evidence")]
pub use expressions::{
    device_current_probe, parse_probe_target, raw_probe_unit, saved_output_references,
    validate_raw_probe,
};

/// Initial presentation intent, independent from whether and how the full
/// precision quantity is retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputDisplayIntent {
    #[default]
    Plot,
    DataBrowserOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputKind {
    RawVoltageOrCurrent,
    DerivedExpression,
    DeviceOperatingPointQuantity,
    NoiseContributor,
    RfPortQuantity,
}

impl SavedOutputKind {
    pub const ALL: [Self; 5] = [
        Self::RawVoltageOrCurrent,
        Self::DerivedExpression,
        Self::DeviceOperatingPointQuantity,
        Self::NoiseContributor,
        Self::RfPortQuantity,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::RawVoltageOrCurrent => "Raw voltage / current",
            Self::DerivedExpression => "Derived expression",
            Self::DeviceOperatingPointQuantity => "Device operating-point quantity",
            Self::NoiseContributor => "Noise contributor",
            Self::RfPortQuantity => "RF port quantity",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputPolicy {
    EveryAcceptedPoint,
    SelectedAndFinalPoints,
    OnDemandFromRetainedState,
    FailureDiagnosticsOnly,
}

impl SavedOutputPolicy {
    pub const ALL: [Self; 4] = [
        Self::EveryAcceptedPoint,
        Self::SelectedAndFinalPoints,
        Self::OnDemandFromRetainedState,
        Self::FailureDiagnosticsOnly,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::EveryAcceptedPoint => "Every accepted point",
            Self::SelectedAndFinalPoints => "Selected + final points",
            Self::OnDemandFromRetainedState => "On demand from retained state",
            Self::FailureDiagnosticsOnly => "Failure diagnostics only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputPrecision {
    FullSourcePrecision,
    DisplayCacheWithFullSourcePrecision,
}

impl SavedOutputPrecision {
    pub const ALL: [Self; 2] = [
        Self::FullSourcePrecision,
        Self::DisplayCacheWithFullSourcePrecision,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::FullSourcePrecision => "f64 / complex128",
            Self::DisplayCacheWithFullSourcePrecision => {
                "f32 display cache + full source precision"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputStreaming {
    LivePlotAdaptiveDisplayDecimation,
    StoreOnly,
}

impl SavedOutputStreaming {
    pub const ALL: [Self; 2] = [Self::LivePlotAdaptiveDisplayDecimation, Self::StoreOnly];

    pub const fn label(self) -> &'static str {
        match self {
            Self::LivePlotAdaptiveDisplayDecimation => "Live plot · adaptive display decimation",
            Self::StoreOnly => "Store only",
        }
    }
}

/// Interpretation of complex source signals in an authored expression.
/// Missing fields preserve historical magnitude arithmetic. New expressions
/// opt into rectangular arithmetic; reopening a recipe never changes it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComplexExpressionPolicy {
    #[default]
    LegacyMagnitude,
    Rectangular,
}

impl ComplexExpressionPolicy {
    pub const ALL: [Self; 2] = [Self::Rectangular, Self::LegacyMagnitude];

    pub const fn is_legacy(&self) -> bool {
        matches!(self, Self::LegacyMagnitude)
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::LegacyMagnitude => "Legacy · magnitude arithmetic",
            Self::Rectangular => "Complex · rectangular arithmetic",
        }
    }
}

mod source_bindings;
pub use source_bindings::{SavedOutputAxis, SavedOutputBoundSource, SavedOutputSourceBindings};
#[cfg(feature = "engine-evidence")]
mod validation;
#[cfg(feature = "engine-evidence")]
pub use validation::SavedOutputValidationRef;

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
    #[serde(default, skip_serializing_if = "ComplexExpressionPolicy::is_legacy")]
    pub complex_policy: ComplexExpressionPolicy,
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
