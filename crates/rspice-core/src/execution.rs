//! Target-neutral deck planning and typed result-schema contracts.
//!
//! Frontends may format these types, but they must not independently decide
//! analysis identity, Cartesian coordinate order, topology identity, or how a
//! signal that is absent at one coordinate is represented.

mod plan;
mod schema;
mod topology;

pub use plan::{
    AnalysisInstanceId, AnalysisKind, AnalysisRequest, AxisAssignment, AxisKind, DataBinding,
    DeckPlan, DeckPlanError, PlannedAnalysis, RunAxis, RunAxisValue, RunCoordinate,
    RunCoordinateId, StepAxisTarget,
};
pub use schema::{
    CoordinateSchema, SchemaUnion, SignalDescriptor, SignalKind, SignalOwner, SignalSchema,
    SignalSchemaError, SignalShape, SignalUnit, SignalValueType,
};
pub use topology::{TopologyComponent, TopologyFingerprint, TopologyFingerprintError};
