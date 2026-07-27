//! Strict, deterministic Automation/CI workflow contracts.
//!
//! This module deliberately contains no UI or execution code.  It accepts the
//! small declarative language exposed by the Automation workspace, compiles it
//! into an immutable plan, and renders requested artifacts from completed,
//! typed evidence supplied by the execution layer.

mod artifacts;
mod parser;
mod regression;

pub use artifacts::{
    ArtifactRenderError, CheckEvidence, CheckOutcome, ComparisonEvidence, CompletedEvidence,
    EvidenceError, RenderedArtifact, render_requested_artifacts,
};
pub use parser::{
    ArtifactKind, AutomationPlan, CompareStageAst, Diagnostic, DiagnosticCode, DiagnosticSet,
    ExecuteStageAst, ExportStageAst, MAX_SOURCE_BYTES, PlanStageAst, RequireStageAst, SourceDigest,
    SourceLocation, SourceSpan, SpannedString, WorkflowAst, compile_workflow, parse_workflow,
};
pub(crate) use regression::compare_governed_waveforms;

#[cfg(test)]
mod tests;
