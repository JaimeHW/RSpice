#![forbid(unsafe_code)]

//! Stable domain vocabulary shared by RSpice cloud modules.
//!
//! This crate intentionally has no database, HTTP, or runtime dependencies.

mod execution;
mod ids;
mod policy;
mod revision;
mod simulation;

pub use execution::{
    CURRENT_SIMULATION_EXECUTION_MANIFEST_VERSION, LEGACY_SIMULATION_EXECUTION_MANIFEST_VERSION,
    MAX_SIMULATION_ATTEMPTS, MAX_SIMULATION_EXECUTION_ARTIFACT_BYTES,
    MAX_SIMULATION_EXECUTION_ARTIFACTS, MAX_SIMULATION_EXECUTION_MANIFEST_BYTES,
    SimulationExecutionArtifact, SimulationExecutionEngine, SimulationExecutionManifest,
    SimulationExecutionRequest, SimulationExecutionRevision, SimulationExecutionRuntimeMode,
    VERIFIED_ADAPTER_SIMULATION_EXECUTION_MANIFEST_VERSION, is_valid_simulation_execution_manifest,
};
pub use ids::{CircuitId, PrincipalId, RevisionId, ShareId, SimulationRunId, WorkspaceId};
pub use policy::{
    CircuitVisibility, EntitlementStatus, SharePermission, SimulationRunStatus, WorkspaceRole,
};
pub use revision::{
    CURRENT_REVISION_CONTENT_DIGEST_VERSION, LEGACY_REVISION_CONTENT_DIGEST_VERSION,
    RevisionArtifactIntegrity, RevisionDigestError, revision_content_digest,
};
pub use simulation::{
    CURRENT_SIMULATION_REQUEST_DIGEST_VERSION, SimulationRequestDigestError,
    simulation_request_digest,
};
