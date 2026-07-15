//! Stable, presentation-independent simulation-plan domain.
//!
//! This module owns analysis-instance identity, editable drafts, dependency
//! edges, lifecycle transactions, and frozen plan projections. Engine request
//! construction and egui presentation consume this model but are not part of
//! its authority boundary.

mod config;
mod kind;
mod model;

pub use config::{AcDraft, AnalysisDraft, DistoDraft, NoiseDraft};
pub use kind::AnalysisKind;
pub use model::{
    AnalysisDependency, AnalysisInstance, AnalysisLifecycleCommand, AnalysisLifecycleReceipt,
    AnalysisLifecycleState, AnalysisPlanError, AnalysisPlanIssue, AnalysisTombstone,
    FrozenAnalysisInstance, FrozenSimulationPlan, SimulationPlan,
};
