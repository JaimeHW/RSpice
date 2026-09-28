//! Resolve hierarchy against project authority and explicit source metadata.

use super::ProjectWorkspace;
use crate::ProjectLibraries;
use rspice_design::configuration_set::{ConfigurationSet, ConfigurationSetId};
use rspice_design::hierarchy::{
    self, HierarchyContext, HierarchyDocuments, HierarchyResolution, HierarchySchematic,
    HierarchySourceFiles,
};
use rspice_design::projection::{ProjectionSources, SchematicSource};
use rspice_design::schematic::owned::Schematic;
use rspice_design_model::cell_view::CellViewRef;
use std::collections::BTreeMap;

/// Borrowed project authority plus host metadata for one hierarchy operation.
pub struct ProjectHierarchy<'a, S, F> {
    pub workspace: &'a ProjectWorkspace,
    pub libraries: &'a ProjectLibraries,
    pub sources: S,
    pub source_files: F,
    pub active_overlay: Option<(&'a CellViewRef, SchematicSource<'a>)>,
}

impl<S: ProjectionSources, F: HierarchySourceFiles> ProjectHierarchy<'_, S, F> {
    pub fn resolve(&self) -> HierarchyResolution {
        self.with_authority(
            self.workspace.simulation_root_reference(),
            self.workspace.configuration_sets.active(),
        )
    }

    /// Inspect an exact root/configuration without changing the active plan.
    pub fn resolve_for_reference(
        &self,
        root: &CellViewRef,
        configuration: Option<ConfigurationSetId>,
    ) -> Result<HierarchyResolution, String> {
        root.validate_name_segments()
            .map_err(|error| error.to_string())?;
        let configuration = configuration
            .map(|id| {
                self.workspace
                    .configuration_sets
                    .find(id)
                    .ok_or_else(|| "The hierarchy configuration no longer exists.".to_owned())
            })
            .transpose()?;
        if let Some(configuration) = configuration
            && !configuration.root().key().eq_ignore_ascii_case(&root.key())
        {
            return Err("The hierarchy configuration belongs to a different root.".to_owned());
        }
        Ok(self.with_authority(root.clone(), configuration))
    }

    fn with_authority(
        &self,
        root: CellViewRef,
        configuration: Option<&ConfigurationSet>,
    ) -> HierarchyResolution {
        let context = HierarchyContext {
            documents: self,
            libraries: self.libraries.catalog(),
            project_id: self.workspace.project.id(),
            project_sources: &self.workspace.project_sources,
            source_files: &self.source_files,
        };
        hierarchy::resolve_hierarchy(context, root, configuration).0
    }

    pub(crate) fn schematic_reference_sources(&self) -> BTreeMap<String, &Schematic> {
        let active = self.active_overlay.map(|(reference, _)| reference.key());
        let mut sources: BTreeMap<_, _> = self
            .workspace
            .schematic_buffers
            .iter()
            .filter(|(key, _)| {
                !active
                    .as_ref()
                    .is_some_and(|active| key.eq_ignore_ascii_case(active))
            })
            .map(|(key, design)| (key.clone(), design))
            .collect();
        if let Some((reference, source)) = self.active_overlay {
            sources.insert(reference.key(), source.schematic);
        }
        sources
    }
}

impl<S: ProjectionSources, F: HierarchySourceFiles> HierarchyDocuments
    for ProjectHierarchy<'_, S, F>
{
    fn find_schematic(&self, reference: &CellViewRef) -> Option<HierarchySchematic<'_>> {
        let schematic = if let Some((overlay_reference, schematic)) = self.active_overlay
            && overlay_reference
                .key()
                .eq_ignore_ascii_case(&reference.key())
        {
            Some(schematic)
        } else {
            self.workspace
                .schematic_buffers
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(&reference.key()))
                .map(|(key, design)| self.sources.schematic_source(key, design))
        }?;
        Some(HierarchySchematic {
            document: schematic.schematic.document(),
            modified: schematic.modified,
        })
    }
}
