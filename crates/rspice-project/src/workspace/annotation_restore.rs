//! Restore approved annotation receipts through the project reference transaction.

use super::{ProjectHierarchy, ProjectReferenceTransaction, ProjectWorkspace, reference_from_key};
use rspice_design::{hierarchy::HierarchySourceFiles, projection::ProjectionSources};
use std::collections::BTreeMap;

/// A fully validated restoration of an already-approved annotation journal.
pub struct PreparedAnnotation {
    count: usize,
    transaction: ProjectReferenceTransaction,
}

impl PreparedAnnotation {
    pub fn changed_documents(&self) -> impl Iterator<Item = &str> {
        self.transaction.after.keys().map(String::as_str)
    }

    pub fn publish(self, workspace: &mut ProjectWorkspace) -> usize {
        self.transaction.prepared_references.publish(workspace);
        for (key, schematic) in self.transaction.after {
            for open in &mut workspace.open_views {
                if open.reference.key().eq_ignore_ascii_case(&key) {
                    open.dirty = true;
                }
            }
            workspace.schematic_buffers.insert(key, schematic);
        }
        self.count
    }
}

impl<S: ProjectionSources, F: HierarchySourceFiles> ProjectHierarchy<'_, S, F> {
    /// Prepare every affected document and reference before publishing any change.
    pub fn prepare_pending_annotation(self) -> Result<Option<PreparedAnnotation>, String> {
        let annotation = self.workspace.design_management.annotation();
        if annotation.journal().is_empty() {
            return Ok(None);
        }
        self.workspace
            .design_management
            .validate()
            .map_err(|error| error.to_string())?;
        let sources = self
            .workspace
            .schematic_buffers
            .iter()
            .flat_map(|(key, schematic)| {
                schematic
                    .document()
                    .components
                    .iter()
                    .map(move |component| {
                        rspice_design_model::design_management::SchematicObjectKey::new(
                            key,
                            component.id,
                        )
                        .map(|object| (object, component.name.as_str()))
                    })
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())?;
        let assignments = annotation
            .projected_reference_assignments(sources)
            .map_err(|error| error.to_string())?;
        if assignments.is_empty() {
            return Ok(None);
        }
        let count = assignments.len();
        let mut by_document: BTreeMap<String, BTreeMap<u64, String>> = BTreeMap::new();
        for (object, name) in assignments {
            let key = self
                .workspace
                .schematic_buffers
                .keys()
                .find(|key| key.eq_ignore_ascii_case(object.cell_view_key()))
                .expect("annotation targets were collected from these buffers");
            by_document
                .entry(key.clone())
                .or_default()
                .insert(object.object_id(), name);
        }
        let mut before = BTreeMap::new();
        let mut after = BTreeMap::new();
        for (key, names) in by_document {
            let source = self
                .workspace
                .schematic_buffers
                .get(&key)
                .expect("annotation source");
            let candidate = source
                .renamed_reference_candidate(&names)
                .map_err(|reason| {
                    format!("Cannot restore reference annotation in '{key}': {reason}")
                })?;
            before.insert(key.clone(), source.clone());
            after.insert(key, candidate);
        }
        // The overlay must be an actual source, including projects last saved
        // with a non-schematic document active. It never changes navigation.
        let requested = self.workspace.active_schematic_reference();
        let active_key = self
            .workspace
            .schematic_buffers
            .keys()
            .find(|key| key.eq_ignore_ascii_case(&requested.key()))
            .unwrap_or_else(|| {
                before
                    .keys()
                    .next()
                    .expect("at least one pending annotation")
            });
        let active_reference = reference_from_key(active_key)?;
        let source = self
            .workspace
            .schematic_buffers
            .get(active_key)
            .expect("annotation overlay");
        let transaction = ProjectHierarchy {
            workspace: self.workspace,
            libraries: self.libraries,
            active_overlay: Some((
                &active_reference,
                self.sources.schematic_source(active_key, source),
            )),
            sources: AnnotationSources(&self.sources),
            source_files: AnnotationFiles(&self.source_files),
        }
        .prepare_reference_transaction(before, after)?;
        Ok(Some(PreparedAnnotation { count, transaction }))
    }
}

struct AnnotationSources<'a, S>(&'a S);
impl<S: ProjectionSources> ProjectionSources for AnnotationSources<'_, S> {
    fn schematic_source<'a>(
        &'a self,
        key: &str,
        schematic: &'a rspice_design::schematic::owned::Schematic,
    ) -> rspice_design::projection::SchematicSource<'a> {
        self.0.schematic_source(key, schematic)
    }
}
struct AnnotationFiles<'a, F>(&'a F);
impl<F: HierarchySourceFiles> HierarchySourceFiles for AnnotationFiles<'_, F> {
    fn source_paths_match(&self, left: &std::path::Path, right: &std::path::Path) -> bool {
        self.0.source_paths_match(left, right)
    }
    fn configured_source_identity(&self, path: &std::path::Path) -> String {
        self.0.configured_source_identity(path)
    }
    fn validate_source_file(
        &self,
        path: &std::path::Path,
        view_type: rspice_design::library::ViewType,
        binding: &rspice_design::schematic::component::LibraryCellInstance,
    ) -> Result<(), String> {
        self.0.validate_source_file(path, view_type, binding)
    }
}
