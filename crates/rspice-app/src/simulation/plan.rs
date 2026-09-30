//! Stable, presentation-independent simulation-plan domain.
//!
//! This module owns analysis-instance identity, editable drafts, dependency
//! edges, lifecycle transactions, and frozen plan projections. Engine request
//! construction and egui presentation consume this model but are not part of
//! its authority boundary.

mod config;
#[cfg(test)]
mod model_app_tests;
mod numeric_override;

pub use config::{
    AcDataDraft, AnalysisDependencyRepairContext, AnalysisDraft, DcMismatchDraft, DistoDraft,
    FftDraft, FrequencySweepDraft, HbNoiseDraft, NetworkPortDraft, NoiseDraft,
    PeriodicNetworkDraft, QpnoiseLatticeSelection, QpnoiseOutputDraft, QpnoiseSourceSelection,
    QpssDraft, QpxfSidebandSelection, QpxfSourceSelection, QuasiPeriodicAcDraft,
    QuasiPeriodicNoiseDraft, QuasiPeriodicTransferDraft, TransientNoiseDraft,
};
pub use numeric_override::{
    AnalysisNumericOverride, NumericOverrideOption, OverrideSection, OverrideValue,
    OverrideValueKind, SolverOwnership,
};
pub use rspice_simulation_contract::analysis_kind::{AnalysisAvailability, AnalysisKind};
pub use rspice_simulation_contract::plan_model::{
    AnalysisDependency, AnalysisInstance, AnalysisLifecycleCommand, AnalysisLifecycleReceipt,
    AnalysisLifecycleState, AnalysisPlanError, AnalysisPlanIssue, FrozenAnalysisInstance,
    FrozenSimulationPlan, SimulationPlan,
};
