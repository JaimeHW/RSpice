//! Project identity, execution context, metadata and revision receipts.

mod descriptor;
mod execution_context;
mod library_publication;

pub use descriptor::*;
pub use execution_context::{
    PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION, ProjectExecutionContext, ProjectModelLibrary,
    persisted_active_model_section_names,
};
pub use library_publication::*;

pub mod results;

pub mod registry;

mod libraries;
mod open_view;
pub use libraries::ProjectLibraries;
pub use open_view::OpenCellView;

mod workspace;
pub use workspace::{
    HardcopySourceSetPersistenceError, MAX_PROJECT_HARDCOPY_SOURCE_SETS,
    MAX_PROJECT_VISUALIZATION_DOCUMENTS, PreparedPhysicalLayoutCatalog,
    ProjectConfigurationMutationError, ProjectWorkspace, SimulationConfigurationError,
    VisualizationDocumentPersistenceError,
};
