//! Model Library Browser
//!
//! PDK model file navigation with corner/process selection.
//!
//! # Architecture
//!
//! Matches Cadence's model library management:
//! - **Model Library**: Collection of device models (e.g., `tsmc180.lib`)
//! - **Section/Corner**: Process corner within library (tt, ff, ss, etc.)
//! - **Model**: Individual device model (nmos, pmos, npn, etc.)

mod compatibility;
mod manager;
mod part_index;
mod source_label;

pub(crate) use compatibility::placement_component_for_model;
pub use compatibility::{
    models_have_compatible_device_family, validate_component_model_compatibility,
};
pub use rspice_model_library::ProjectModelDefinition;
pub use rspice_model_library::RetainedClosure;
pub use rspice_model_library::device_class::{SUBCIRCUIT_CLASS, card_device};
pub use rspice_model_library::{
    CornerSectionBinding, CornerSectionDomain, ProcessCorner, stated_temperatures,
};
pub use rspice_model_library::{
    CorrelationMatrix, FiniteBounds, FiniteF64, LookupInterpolation, ModelDefinitionMetadata,
    ModelFileIdentity, ModelSectionDefinition, ParameterDataType, ParameterDefinition,
    ParameterSource, ParameterValue, StatisticalDistribution, StatisticalHierarchyScope,
    TemperatureExtrapolationPolicy, TemperatureLawDefinition, TemperatureLawRepresentation,
};
// Metadata fixtures used by application integration tests.
pub use manager::model_library_source_digest;
pub use manager::{ModelLibraryManager, ProjectModelCommit, SealedModelExecutionSources};
pub use part_index::{LibraryPart, LibraryPartKind, library_part_index};
pub use rspice_model_library::DeviceModel;
pub use rspice_model_library::ProjectModelRevisionDefinition;
pub use rspice_model_library::first_unreachable_source;
#[cfg(not(target_arch = "wasm32"))]
pub use rspice_model_library::is_foreign_platform_absolute_path;
#[cfg(any(test, feature = "test-support"))]
pub use rspice_model_library::is_portable_absolute_path;
pub use rspice_model_library::{BinFamily, BindableModel, model_bin_family};
pub use rspice_model_library::{ClosureFacts, closure_facts, envelope_is_invalid};
pub use rspice_model_library::{
    ModelConsumerScope, ModelResolutionRecord, ModelValidationFinding,
    ModelValidationFindingSeverity, SimulationPlanModelBinding,
};
pub use rspice_model_library::{ModelLevel, ModelType};
pub use rspice_model_library::{ModelLibrary, ModelSourceAuthority};
#[cfg(any(test, feature = "test-support"))]
pub use rspice_model_library::{
    ModelSourceContent, ModelSourceEdge, ModelSourcePin, ModelSubcircuitInterface,
};
#[cfg(any(test, feature = "test-support"))]
pub use rspice_model_library::{StatisticalDefinition, StatisticalVariableDefinition};
pub use source_label::short_digest;
