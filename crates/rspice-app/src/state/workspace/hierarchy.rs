//! Project authority and editor projections for the design hierarchy resolver.

use super::hierarchy_resolver as hierarchy_core;
use super::*;
use hierarchy_core::HierarchySourceFiles;
pub use hierarchy_core::{
    ConfigurationExecutionBinding, ConfigurationExecutionPlan, ConfigurationVerilogABinding,
    HierarchyResolution, MasterKey, ResolvedHierarchyBinding,
};
#[cfg(test)]
pub use hierarchy_core::{HierarchyBindingStatus, MasterRecord};
pub(crate) use hierarchy_core::{assign_master_names, master_closure_digest};

#[cfg(test)]
mod tests;

impl ProjectWorkspace {
    pub(super) fn project_hierarchy<'a>(
        &'a self,
        libraries: &'a LibraryManager,
        active_overlay: Option<(&'a CellViewRef, SchematicEditorRef<'a>)>,
    ) -> rspice_project::ProjectHierarchy<
        'a,
        super::design_projection::WorkspaceProjectionSources<'a>,
        WorkspaceSourceFiles,
    > {
        self.session
            .project_hierarchy(&self.content, libraries, active_overlay)
    }
}

impl super::WorkspaceSession {
    pub(super) fn project_hierarchy<'a>(
        &'a self,
        content: &'a rspice_project::ProjectWorkspace,
        libraries: &'a LibraryManager,
        active_overlay: Option<(&'a CellViewRef, SchematicEditorRef<'a>)>,
    ) -> rspice_project::ProjectHierarchy<
        'a,
        super::design_projection::WorkspaceProjectionSources<'a>,
        WorkspaceSourceFiles,
    > {
        rspice_project::ProjectHierarchy {
            workspace: content,
            libraries,
            sources: super::design_projection::WorkspaceProjectionSources(&self.schematic_sessions),
            source_files: WorkspaceSourceFiles,
            active_overlay: active_overlay.map(|(reference, source)| {
                (
                    reference,
                    rspice_design::projection::SchematicSource {
                        schematic: source.design,
                        current_file: source
                            .session
                            .and_then(|session| session.current_file.as_deref()),
                        read_only: source.read_only(),
                        modified: source.is_dirty(),
                    },
                )
            }),
        }
    }
}

pub(crate) struct WorkspaceSourceFiles;

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

pub(super) fn find_schematic<'a>(
    workspace: &'a ProjectWorkspace,
    reference: &CellViewRef,
) -> Option<SchematicEditorRef<'a>> {
    workspace
        .content
        .schematic_buffers
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(&reference.key()))
        .map(|(key, design)| SchematicEditorRef {
            design,
            session: workspace.session.schematic_sessions.get(key),
        })
}
