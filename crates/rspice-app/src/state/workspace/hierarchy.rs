//! Project authority and editor projections for the design hierarchy resolver.

use super::hierarchy_resolver as hierarchy_core;
use super::*;
pub use hierarchy_core::{
    ConfigurationExecutionBinding, ConfigurationExecutionPlan, ConfigurationVerilogABinding,
    HierarchyResolution, MasterKey, ResolvedHierarchyBinding,
};
#[cfg(test)]
pub use hierarchy_core::{HierarchyBindingStatus, MasterRecord};
use hierarchy_core::{
    HierarchyContext, HierarchyDocuments, HierarchySchematic, HierarchySourceFiles,
};
pub(crate) use hierarchy_core::{assign_master_names, master_closure_digest};
pub(super) use hierarchy_core::{find_cell, find_view};

#[cfg(test)]
mod tests;

impl ProjectWorkspace {
    /// Inspect an exact root/configuration without changing the active plan
    /// or cloning retained project data. Reference-changing edits use the
    /// same resolver as netlisting for each affected executable authority.
    pub(crate) fn resolve_hierarchy_for_reference<'a>(
        &'a self,
        libraries: &'a LibraryManager,
        root: &CellViewRef,
        configuration: Option<crate::state::ConfigurationSetId>,
        active_reference: &'a CellViewRef,
        active_schematic: &'a SchematicState,
    ) -> Result<HierarchyResolution, String> {
        root.validate_name_segments()
            .map_err(|error| error.to_string())?;
        let configuration = configuration
            .map(|id| {
                self.configuration_sets
                    .find(id)
                    .ok_or_else(|| "The hierarchy configuration no longer exists.".to_owned())
            })
            .transpose()?;
        if let Some(configuration) = configuration
            && !configuration.root().key().eq_ignore_ascii_case(&root.key())
        {
            return Err("The hierarchy configuration belongs to a different root.".to_owned());
        }
        Ok(HierarchyResolver::with_authority(
            self,
            libraries,
            Some((active_reference, active_schematic)),
            root.clone(),
            configuration,
        )
        .resolve())
    }
}

pub(super) struct HierarchyResolver<'a> {
    workspace: &'a ProjectWorkspace,
    libraries: &'a LibraryManager,
    active_overlay: Option<(&'a CellViewRef, &'a SchematicState)>,
    projected_buffers: Option<&'a HashMap<String, SchematicState>>,
    root: CellViewRef,
    configuration: Option<&'a crate::state::ConfigurationSet>,
}

impl<'a> HierarchyResolver<'a> {
    pub(super) fn new(
        workspace: &'a ProjectWorkspace,
        libraries: &'a LibraryManager,
        active_overlay: Option<(&'a CellViewRef, &'a SchematicState)>,
    ) -> Self {
        Self::with_authority(
            workspace,
            libraries,
            active_overlay,
            workspace.simulation_root_reference(),
            workspace.configuration_sets.active(),
        )
    }

    pub(super) fn with_authority(
        workspace: &'a ProjectWorkspace,
        libraries: &'a LibraryManager,
        active_overlay: Option<(&'a CellViewRef, &'a SchematicState)>,
        root: CellViewRef,
        configuration: Option<&'a crate::state::ConfigurationSet>,
    ) -> Self {
        Self {
            workspace,
            libraries,
            active_overlay,
            projected_buffers: None,
            root,
            configuration,
        }
    }

    pub(super) fn with_projected_buffers(
        mut self,
        buffers: &'a HashMap<String, SchematicState>,
    ) -> Self {
        self.projected_buffers = Some(buffers);
        self
    }

    pub(super) fn resolve(self) -> HierarchyResolution {
        self.resolve_all().0
    }

    pub(super) fn resolve_all(self) -> (HierarchyResolution, ConfigurationExecutionPlan) {
        let documents = WorkspaceHierarchyDocuments {
            buffers: &self.workspace.schematic_buffers,
            active_overlay: self.active_overlay,
            projected_buffers: self.projected_buffers,
        };
        let context = HierarchyContext {
            documents: &documents,
            libraries: self.libraries.catalog(),
            project_id: self.workspace.project.id(),
            project_sources: &self.workspace.project_sources,
            source_files: &WorkspaceSourceFiles,
        };
        hierarchy_core::resolve_hierarchy(context, self.root, self.configuration)
    }
}

struct WorkspaceHierarchyDocuments<'a> {
    buffers: &'a HashMap<String, SchematicState>,
    active_overlay: Option<(&'a CellViewRef, &'a SchematicState)>,
    projected_buffers: Option<&'a HashMap<String, SchematicState>>,
}

impl HierarchyDocuments for WorkspaceHierarchyDocuments<'_> {
    fn find_schematic(&self, reference: &CellViewRef) -> Option<HierarchySchematic<'_>> {
        let schematic = if let Some(buffers) = self.projected_buffers {
            buffers
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(&reference.key()))
                .map(|(_, schematic)| schematic)
        } else if let Some((overlay_reference, schematic)) = self.active_overlay
            && overlay_reference
                .key()
                .eq_ignore_ascii_case(&reference.key())
        {
            Some(schematic)
        } else {
            self.buffers
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(&reference.key()))
                .map(|(_, schematic)| schematic)
        }?;
        Some(HierarchySchematic {
            document: &schematic.document,
            modified: schematic.is_dirty,
        })
    }
}

struct WorkspaceSourceFiles;

impl HierarchySourceFiles for WorkspaceSourceFiles {
    fn source_paths_match(&self, left: &Path, right: &Path) -> bool {
        source_paths_match(left, right)
    }

    fn configured_source_identity(&self, path: &Path) -> String {
        configured_source_identity(path)
    }

    fn validate_source_file(
        &self,
        source_path: &Path,
        view_type: ViewType,
        binding: &LibraryCellInstance,
    ) -> Result<(), String> {
        validate_source_file(source_path, view_type, binding)
    }
}

pub(super) fn find_library<'a>(libraries: &'a LibraryManager, name: &str) -> Option<&'a Library> {
    hierarchy_core::find_library(libraries.catalog(), name)
}

pub(super) fn find_schematic<'a>(
    workspace: &'a ProjectWorkspace,
    reference: &CellViewRef,
) -> Option<&'a SchematicState> {
    workspace
        .schematic_buffers
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(&reference.key()))
        .map(|(_, schematic)| schematic)
}
