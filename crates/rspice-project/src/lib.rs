//! Canonical project documents, accepted content and persistence contracts.

mod accepted;
mod candidate;
pub use accepted::AcceptedProject;

mod execution_context;

pub use execution_context::{
    PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION, ProjectExecutionContext, ProjectModelLibrary,
    persisted_active_model_section_names,
};
pub use rspice_project_contract::*;

pub mod results;

pub mod registry;

mod libraries;
mod open_view;
pub use libraries::ProjectLibraries;
pub use open_view::OpenCellView;

mod workspace;
pub use workspace::{
    HardcopySourceSetPersistenceError, MAX_PROJECT_HARDCOPY_SOURCE_SETS,
    MAX_PROJECT_VISUALIZATION_DOCUMENTS, PreparedAnnotation, PreparedPhysicalLayoutCatalog,
    PreparedReferences, ProjectConfigurationMutationError, ProjectHierarchy,
    ProjectReferenceTransaction, ProjectWorkspace, ReferenceChanges, SimulationConfigurationError,
    VisualizationDocumentPersistenceError, bind_generated_netlist_provenance, reference_from_key,
};

mod file;
pub use file::{
    DecodedProject, MAX_PROJECT_FILE_BYTES, ProjectFile, ProjectIoError, ProjectVersion,
    decode_project_text, serialize_project_file,
};

pub mod lifecycle;
pub mod persistence;

mod working;
pub use working::{ProjectWorkingSet, SnapshotContent, SnapshotSessions};
