//! Bind project authority and the live editor to the headless design projection.
use super::hierarchy::WorkspaceSourceFiles;
use super::*;
use rspice_design::projection::ProjectionContext;
pub use rspice_design::projection::{
    ConfigurationExecutionPlanError, ConfigurationExecutionProjection, DesignProjection,
    DesignProjectionKey,
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
        active_schematic: &SchematicState,
    ) -> Result<Arc<DesignProjection>, ConfigurationExecutionPlanError> {
        self.inspect_design_projection(libraries, active_reference, active_schematic)?
            .into_execution()
    }

    pub fn inspect_design_projection(
        &self,
        libraries: &LibraryManager,
        active_reference: &CellViewRef,
        active_schematic: &SchematicState,
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
        active_schematic: &SchematicState,
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
        active_schematic: &SchematicState,
    ) -> Result<ConfigurationExecutionProjection, ConfigurationExecutionPlanError> {
        self.design_projection(libraries, active_reference, active_schematic)
    }

    fn projection_context(&self) -> ProjectionContext<'_, SchematicState, WorkspaceSourceFiles> {
        ProjectionContext {
            schematic_buffers: &self.schematic_buffers,
            configuration_sets: &self.configuration_sets,
            design_management: &self.design_management,
            connectivity: &self.connectivity,
            project_id: self.project.id(),
            project_revision: self.project.revision(),
            project_sources: &self.project_sources,
            root: self.simulation_root_reference(),
            source_files: &WorkspaceSourceFiles,
            cache: &self.design_projection_cache,
        }
    }
}
