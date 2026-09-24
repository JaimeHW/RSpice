//! Portable model definitions, metadata, process corners, and model evidence.
//!
//! Retained source authority, corner resolution, and project-model revision validation
//! share the same definitions and evidence contracts.

mod authoring;
mod binding;
mod catalog;
mod corner;
mod corner_expansion;
pub mod correlation;
mod definition_metadata;
mod execution;
mod facts;
mod library;
mod model;
mod project_revision;
mod projection;
pub mod qualification;
mod resolution;
pub mod source_bundle;
mod types;
mod validation;

pub use authoring::*;
pub use binding::CornerModelBinding;
pub use catalog::{CapturedHdlSources, ModelCatalog, ProjectModelCommit, ProjectModelTarget};
pub use corner::*;
pub use definition_metadata::*;
pub use model::*;
pub use types::*;

pub use corner_expansion::RetainedClosure;
pub use facts::{ClosureFacts, closure_facts, envelope_is_invalid};
pub use library::*;
pub use project_revision::*;

pub use resolution::*;
pub use validation::*;

pub use execution::{
    MaterializedPlanBinding, ModelExecutionPlan, resolve_materialized_definition_namespace,
};
