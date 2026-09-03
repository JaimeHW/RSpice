//! Target-neutral deck planning and typed result-schema contracts.
//!
//! Frontends may format these types, but they must not independently decide
//! analysis identity, Cartesian coordinate order, topology identity, or how a
//! signal that is absent at one coordinate is represented.

pub mod bounded_io;
mod capability;
mod fingerprint;
mod materialized;
mod plan;
mod post_process;
mod projection;
pub mod result_document;
mod schema;
mod seed;
mod topology;
mod transient;

pub use crate::identity::{AnalysisInstanceId, AnalysisKind, RunCoordinateId};
pub use bounded_io::{BoundedAbortWriter, BoundedWriteFailure};
pub use capability::{
    ANALYSIS_CAPABILITY_MATRIX, AnalysisResultCapability, AnalysisResultKind, MappingStatus,
    NonUiSurface, SIGNAL_CAPABILITY_MATRIX, SignalCapability, SurfaceCapability,
    analysis_result_capability, analysis_result_kind, signal_capability,
};
pub use fingerprint::{topology_fingerprint, topology_fingerprint_with_abort};
pub use materialized::{
    ArtifactNamespace, DeckPlanMaterializer, MaterializedAnalysis, MaterializedRun,
    MaterializedRunError,
};
pub use plan::{
    AnalysisRequest, AxisAssignment, AxisKind, DataBinding, DeckPlan, DeckPlanError,
    PlannedAnalysis, PlannedPostProcess, PostProcessSource, RunAxis, RunAxisValue, RunCoordinate,
    StepAxisTarget, numeric_run_coordinate_id,
};
pub use post_process::{
    PlannedFourierResult, evaluate_planned_fourier_with_abort, transient_output_unit,
};
pub use projection::{
    ProjectedSignal, ProjectedSignals, ProjectionSource, ProjectionSourceSignal, ProjectionValues,
    SignalProjection, dc_sweep_observable_series, observable_lookup,
    operating_point_observable_series, operating_point_projection_signals, probe_names_nothing,
    probe_registry_name, probe_specification_error, projection_analysis_kind, raw_variable_type,
    signal_descriptor, transient_projection_signals,
};
pub use result_document::{
    ANALYSIS_RESULT_DOCUMENT_SCHEMA, ANALYSIS_RESULT_DOCUMENT_VERSION, AnalysisResultDocument,
    AnalysisResultDocumentBuilder, ResultAxis, ResultAxisKind, ResultCoordinate,
    ResultDocumentError, ResultNamespaces, ResultPayload, ResultScalar, ResultSignal, ResultWindow,
};
pub use schema::{
    CoordinateSchema, SchemaUnion, SignalDescriptor, SignalKind, SignalOwner, SignalSchema,
    SignalSchemaError, SignalShape, SignalUnit, SignalValueType,
};
pub use seed::monte_carlo_seed_at_coordinate;
pub use topology::{TopologyComponent, TopologyFingerprint, TopologyFingerprintError};
pub use transient::{TransientMaximumStepError, resolve_transient_maximum_step};
