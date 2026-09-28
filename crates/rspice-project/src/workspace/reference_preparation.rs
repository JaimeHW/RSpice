//! Prepare reference edits across project configurations and document occurrences.

use super::{PreparedReferences, ProjectHierarchy, ProjectWorkspace, ReferenceChanges};
use rspice_design::{
    hierarchy::{HierarchyResolution, HierarchySourceFiles},
    projection::ProjectionSources,
    references::{PathMappings, ReferenceComponents, remap_instance_probes_many},
    schematic::owned::Schematic,
};
use rspice_design_model::cell_view::CellViewRef;
use std::collections::BTreeMap;

pub struct ProjectReferenceTransaction {
    pub before: BTreeMap<String, Schematic>,
    pub after: BTreeMap<String, Schematic>,
    pub references: ReferenceChanges,
    pub prepared_references: PreparedReferences,
    /// Documents whose probe expressions changed during preparation.
    pub probe_documents: Vec<String>,
}

impl<S: ProjectionSources, F: HierarchySourceFiles> ProjectHierarchy<'_, S, F> {
    /// Callers first validate complete component candidates, including their
    /// local structural references. This adds every affected hierarchy path,
    /// configuration, saved output, probe and open-document occurrence.
    pub fn prepare_reference_transaction(
        &self,
        mut before: BTreeMap<String, Schematic>,
        mut after: BTreeMap<String, Schematic>,
    ) -> Result<ProjectReferenceTransaction, String> {
        if !before.keys().eq(after.keys()) {
            return Err("Reference edit documents do not match their original sources.".to_owned());
        }
        let projected = self.schematic_reference_sources();
        let mut configurations = self.workspace.configuration_sets.clone();
        let mut probe_roots: BTreeMap<String, PathMappings> = BTreeMap::new();
        if !before.is_empty() {
            for configuration in self.workspace.configuration_sets.configurations() {
                let resolution =
                    self.resolve_for_reference(configuration.root(), Some(configuration.id()))?;
                let mappings = hierarchy_reference_paths(
                    configuration.root(),
                    &resolution,
                    &before,
                    &after,
                    false,
                )?;
                configurations
                    .remap_configuration_instance_paths(configuration.id(), &mappings)
                    .map_err(|error| error.to_string())?;
                if self.workspace.configuration_sets.active_configuration_id()
                    == Some(configuration.id())
                {
                    probe_roots.insert(
                        configuration.root().key().to_ascii_lowercase(),
                        hierarchy_reference_paths(
                            configuration.root(),
                            &resolution,
                            &before,
                            &after,
                            true,
                        )?,
                    );
                }
            }
            let root = self.workspace.simulation_root_reference();
            if let std::collections::btree_map::Entry::Vacant(entry) =
                probe_roots.entry(root.key().to_ascii_lowercase())
            {
                let resolution = self.resolve_for_reference(&root, None)?;
                entry.insert(hierarchy_reference_paths(
                    &root,
                    &resolution,
                    &before,
                    &after,
                    true,
                )?);
            }
            for (key, source) in &projected {
                if source.document().probes.is_empty() {
                    continue;
                }
                let root = reference_document_root(self.workspace, key)?;
                if probe_roots.contains_key(&root.key().to_ascii_lowercase()) {
                    continue;
                }
                let resolution = self.resolve_for_reference(&root, None)?;
                probe_roots.insert(
                    root.key().to_ascii_lowercase(),
                    hierarchy_reference_paths(&root, &resolution, &before, &after, true)?,
                );
            }
        }
        let mut outputs = Vec::new();
        if let Some(mappings) = probe_roots.get(
            &self
                .workspace
                .simulation_root_reference()
                .key()
                .to_ascii_lowercase(),
        ) {
            for record in &self.workspace.simulation_plan_payloads {
                for output in &record.payload.saved_outputs {
                    if let Some(expression) =
                        remap_instance_probes_many(&output.source_expression, mappings)?
                    {
                        let mut replacement = output.clone();
                        replacement.source_expression = expression;
                        outputs.push((record.plan_id, replacement));
                    }
                }
            }
        }
        // A probe belongs to its document's recorded occurrence, even while a
        // different document is active. Rooted unopened buffers use their own
        // local instance namespace.
        let mut probe_documents = Vec::new();
        for (key, source) in &projected {
            let root = reference_document_root(self.workspace, key)?;
            let Some(mappings) = probe_roots.get(&root.key().to_ascii_lowercase()) else {
                continue;
            };
            if let Some(probes) = source.prepare_probe_reference_update(mappings)? {
                before
                    .entry(key.clone())
                    .or_insert_with(|| (*source).clone());
                let candidate = after
                    .entry(key.clone())
                    .or_insert_with(|| (*source).clone());
                probes.apply_to(candidate);
                probe_documents.push(key.clone());
            }
        }
        for schematic in after.values_mut() {
            schematic.clear_redo();
        }
        let mut references = ReferenceChanges::between(self.workspace, &configurations, outputs);
        for (key, source) in &before {
            references.add_instance_renames(
                &reference_from_key(key)?,
                &source.document().components,
                &after[key].document().components,
            );
        }
        let prepared_references = references.prepare(self.workspace, true)?;
        Ok(ProjectReferenceTransaction {
            before,
            after,
            references,
            prepared_references,
            probe_documents,
        })
    }
}

pub fn reference_from_key(key: &str) -> Result<CellViewRef, String> {
    let segments = key.split('/').collect::<Vec<_>>();
    let [library, cell, view] = segments.as_slice() else {
        return Err(format!("Invalid schematic document key '{key}'."));
    };
    let reference = CellViewRef::new(*library, *cell, *view);
    reference
        .validate_name_segments()
        .map_err(|error| error.to_string())?;
    Ok(reference)
}

fn hierarchy_reference_paths(
    root: &CellViewRef,
    resolution: &HierarchyResolution,
    before: &BTreeMap<String, Schematic>,
    after: &BTreeMap<String, Schematic>,
    emitted: bool,
) -> Result<PathMappings, String> {
    rspice_design::references::hierarchy_reference_paths(
        root,
        resolution,
        &|reference| {
            before
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(&reference.key()))
                .map(|(key, source)| ReferenceComponents {
                    before: &source.document().components,
                    after: &after[key].document().components,
                })
        },
        emitted,
    )
}

fn reference_document_root(workspace: &ProjectWorkspace, key: &str) -> Result<CellViewRef, String> {
    if let Some(open) = workspace
        .open_views
        .iter()
        .find(|open| open.reference.key().eq_ignore_ascii_case(key))
    {
        return Ok(if open.occurrence.is_unrooted() {
            &open.reference
        } else {
            &open.occurrence.root
        }
        .clone());
    }
    reference_from_key(key)
}
