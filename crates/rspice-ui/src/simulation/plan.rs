//! Stable, presentation-independent simulation-plan domain.
//!
//! This module owns analysis-instance identity, editable drafts, dependency
//! edges, lifecycle transactions, and frozen plan projections. Engine request
//! construction and egui presentation consume this model but are not part of
//! its authority boundary.

mod config;
mod kind;
mod model;

pub use config::{
    AnalysisDependencyRepairContext, AnalysisDraft, DistoDraft, FrequencySweepDraft,
    NetworkPortDraft, NoiseDraft, PeriodicNetworkDraft,
};
pub use kind::{AnalysisAvailability, AnalysisKind};
pub use model::{
    AnalysisDependency, AnalysisInstance, AnalysisLifecycleCommand,
    AnalysisLifecycleReceipt, AnalysisLifecycleState, AnalysisPlanError, AnalysisPlanIssue,
    FrozenAnalysisInstance, FrozenSimulationPlan, SimulationPlan,
    SimulationPlanConfigurationReceipt,
};
