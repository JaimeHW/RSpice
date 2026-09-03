//! Target-neutral deck planning and typed result-schema contracts.
//!
//! Frontends may format these types, but they must not independently decide
//! analysis identity, Cartesian coordinate order, topology identity, or how a
//! signal that is absent at one coordinate is represented.

pub mod bounded_io;
mod capability;
mod materialized;
mod plan;
pub mod result_document;
mod schema;
mod topology;
mod transient;

pub use bounded_io::{BoundedAbortWriter, BoundedWriteFailure};
pub use capability::{
    ANALYSIS_CAPABILITY_MATRIX, AnalysisResultCapability, AnalysisResultKind, MappingStatus,
    NonUiSurface, SIGNAL_CAPABILITY_MATRIX, SignalCapability, SurfaceCapability,
    analysis_result_capability, analysis_result_kind, render_non_ui_capability_matrix,
    signal_capability,
};
pub use materialized::{
    ArtifactNamespace, DeckPlanMaterializer, MaterializedAnalysis, MaterializedRun,
    MaterializedRunError,
};
pub use plan::{
    AnalysisInstanceId, AnalysisKind, AnalysisRequest, AxisAssignment, AxisKind, DataBinding,
    DeckPlan, DeckPlanError, PlannedAnalysis, RunAxis, RunAxisValue, RunCoordinate,
    RunCoordinateId, StepAxisTarget, numeric_run_coordinate_id,
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
pub use topology::{TopologyComponent, TopologyFingerprint, TopologyFingerprintError};
pub use transient::{TransientMaximumStepError, resolve_transient_maximum_step};
