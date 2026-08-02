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
    CheckEvidence, CheckOutcome, ComparisonEvidence, CompletedEvidence, RenderedArtifact,
    render_requested_artifacts,
};
pub use parser::{ArtifactKind, AutomationPlan, DiagnosticSet, compile_workflow};
// The error vocabularies and the standalone parser entry point are asserted
// on by this module's tests; `artifacts` and `parser` are private.
#[cfg(test)]
pub use artifacts::{ArtifactRenderError, EvidenceError};
#[cfg(test)]
pub use parser::{DiagnosticCode, parse_workflow};
pub(crate) use regression::compare_governed_waveforms;

#[cfg(test)]
mod tests;
