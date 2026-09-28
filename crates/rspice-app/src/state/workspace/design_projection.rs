//! Bind project authority and the live editor to the headless design projection.
use super::hierarchy::WorkspaceSourceFiles;
use super::*;
pub use rspice_design::projection::{
    ConfigurationExecutionPlanError, ConfigurationExecutionProjection, DesignProjection,
    DesignProjectionKey,
};
use rspice_design::projection::{
    ProjectionContext, ProjectionSource, ProjectionSources, SchematicSource,
};
#[cfg(test)]
use rspice_design::projection::{materialization_count, reset_materialization_count};
use std::sync::Arc;
#[cfg(test)]
mod tests;

impl ProjectWorkspace {
    pub fn design_projection(
        &self,
        libraries: &LibraryManager,
        active_reference: &CellViewRef,
        active_schematic: &impl ProjectionSource,
    ) -> Result<Arc<DesignProjection>, ConfigurationExecutionPlanError> {
        self.inspect_design_projection(libraries, active_reference, active_schematic)?
            .into_execution()
    }

    pub fn inspect_design_projection(
        &self,
        libraries: &LibraryManager,
        active_reference: &CellViewRef,
        active_schematic: &impl ProjectionSource,
    ) -> Result<Arc<DesignProjection>, ConfigurationExecutionPlanError> {
        if let Some(error) = self.annotation_restoration_error() {
            return Err(ConfigurationExecutionPlanError::DesignManagement(format!(
                "reference annotation restoration failed: {error}. Correct the references, then check the design to retry."
            )));
        }
        self.projection_context().inspect_design_projection(
            libraries.catalog(),
            active_reference,
            active_schematic,
        )
    }

    pub(crate) fn design_projection_key(
        &self,
        libraries: &LibraryManager,
        active_reference: &CellViewRef,
        active_schematic: &impl ProjectionSource,
    ) -> Option<DesignProjectionKey> {
        if self.annotation_restoration_error().is_some() {
            return None;
        }
        self.projection_context().design_projection_key(
            libraries.catalog(),
            active_reference,
            active_schematic,
        )
    }

    pub fn configuration_execution_projection(
        &self,
        libraries: &LibraryManager,
        active_reference: &CellViewRef,
        active_schematic: &impl ProjectionSource,
    ) -> Result<ConfigurationExecutionProjection, ConfigurationExecutionPlanError> {
        self.design_projection(libraries, active_reference, active_schematic)
    }

    fn projection_context(
        &self,
    ) -> ProjectionContext<'_, WorkspaceProjectionSources<'_>, WorkspaceSourceFiles> {
        ProjectionContext {
            schematic_buffers: &self.content.schematic_buffers,
            sources: WorkspaceProjectionSources(&self.session.schematic_sessions),
            configuration_sets: &self.content.configuration_sets,
            design_management: &self.content.design_management,
            connectivity: &self.content.connectivity,
            project_id: self.content.project.id(),
            project_revision: self.content.project.revision(),
            project_sources: &self.content.project_sources,
            root: self.content.simulation_root_reference(),
            source_files: &WorkspaceSourceFiles,
            cache: &self.session.design_projection_cache,
        }
    }
}

pub(super) struct WorkspaceProjectionSources<'a>(
    pub(super) &'a HashMap<String, crate::state::schematic::SchematicSession>,
);

impl ProjectionSources for WorkspaceProjectionSources<'_> {
    fn schematic_source<'a>(
        &'a self,
        key: &str,
        schematic: &'a rspice_design::schematic::owned::Schematic,
    ) -> SchematicSource<'a> {
        let session = self.0.get(key);
        SchematicSource {
            schematic,
            current_file: session.and_then(|session| session.current_file.as_deref()),
            read_only: session.is_some_and(|session| session.read_only),
            modified: session.is_some_and(|session| session.is_dirty),
        }
    }
}
