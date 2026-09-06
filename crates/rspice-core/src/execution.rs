//! Target-neutral deck planning and typed result-schema contracts.
//!
//! Frontends may format these types, but they must not independently decide
//! analysis identity, Cartesian coordinate order, topology identity, or how a
//! signal that is absent at one coordinate is represented.

// Plan rule 2: no authored-input panic. Every index, name and shape this layer
// reads comes from a deck, so an unchecked access here is a crash a user can
// author. A proven invariant keeps its `expect` under a function-scope allow
// that names the test constructing the boundary case; everything else returns
// a typed error. Clippy's in-test allowances (crates/rspice-core/clippy.toml)
// keep the denial off the tests' own preconditions.
#![deny(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::unreachable,
    clippy::unwrap_used
)]

pub mod bounded_io;
mod capability;
mod event_bus;
mod event_export;
mod event_projection;
mod fingerprint;
mod materialized;
mod plan;
mod post_process;
mod post_process_fft;
mod projection;
pub mod result_document;
mod schema;
mod seed;
mod sole_analysis;
mod sweep_axis;
mod topology;
mod transient;

pub use crate::identity::{AnalysisInstanceId, AnalysisKind, RunCoordinateId};
pub use bounded_io::{BoundedAbortWriter, BoundedWriteFailure};
pub use capability::{
    ANALYSIS_CAPABILITY_MATRIX, AnalysisResultCapability, AnalysisResultKind, MappingStatus,
    NonUiSurface, SIGNAL_CAPABILITY_MATRIX, SignalCapability, SurfaceCapability,
    analysis_result_capability, analysis_result_kind, signal_capability,
};
pub use event_bus::{
    BusMemberHistory, BusReassemblyTooLarge, MAX_BUS_EVENT_CELLS, bus_events, bus_value_at,
    split_bus_notation,
};
pub use event_export::{
    EventPlotError, RawEventTraces, decode_event_plots, transient_bus_plots, transient_event_plots,
};
pub use event_projection::{
    EventProjectionError, VcdEventHistories, digital_value_from_vcd_bit, digital_value_to_vcd_bit,
    event_code_to_vcd_bit, event_vcd_document, vcd_event_histories,
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
pub use post_process::{PlannedFourierResult, evaluate_planned_fourier_with_abort};
pub use post_process_fft::{
    PlannedFftSpectrum, planned_transient_fft_spectra, transient_fft_output_unit,
};
pub use projection::{
    ProjectedSignal, ProjectedSignals, ProjectionSource, ProjectionSourceSignal, ProjectionValues,
    SignalProjection, dc_sweep_observable_series, observable_lookup,
    operating_point_observable_series, operating_point_projection_signals, probe_names_nothing,
    probe_registry_name, projection_analysis_kind, raw_variable_type, signal_descriptor,
    transient_projection_signals,
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
pub use sole_analysis::{analysis_instance_identity, sole_analysis_identity};
pub use sweep_axis::sweep_axis_unit;
pub use topology::{TopologyComponent, TopologyFingerprint, TopologyFingerprintError};
pub use transient::{TransientMaximumStepError, resolve_transient_maximum_step};
