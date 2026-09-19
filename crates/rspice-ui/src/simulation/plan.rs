//! Stable, presentation-independent simulation-plan domain.
//!
//! This module owns analysis-instance identity, editable drafts, dependency
//! edges, lifecycle transactions, and frozen plan projections. Engine request
//! construction and egui presentation consume this model but are not part of
//! its authority boundary.

mod config;
mod kind;
mod model;
mod numeric_override;

/// One account of the share threshold's two equivalent spellings, read by the
/// draft validator and by the specification builder alike.
pub(crate) use config::dc_mismatch_share_threshold;
pub use config::{
    AcDataDraft, AnalysisDependencyRepairContext, AnalysisDraft, DcMismatchDraft, DistoDraft,
    FftDraft, FrequencySweepDraft, HbNoiseDraft, NetworkPortDraft, NoiseDraft,
    PeriodicNetworkDraft, QpssDraft, QuasiPeriodicAcDraft, QuasiPeriodicNoiseDraft,
    QuasiPeriodicTransferDraft, TransientNoiseDraft,
};
pub use kind::{AnalysisAvailability, AnalysisKind};
pub use model::{
    AnalysisDependency, AnalysisInstance, AnalysisLifecycleCommand, AnalysisLifecycleReceipt,
    AnalysisLifecycleState, AnalysisPlanError, AnalysisPlanIssue, FrozenAnalysisInstance,
    FrozenSimulationPlan, SimulationPlan, SimulationPlanConfigurationReceipt,
};
pub use numeric_override::{
    AnalysisNumericOverride, NumericOverrideOption, OverrideSection, OverrideValue,
    OverrideValueKind, SolverOwnership,
};
