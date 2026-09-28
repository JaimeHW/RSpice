//! Prepare component references independently of editor history and permissions.

use super::*;
use rspice_design::references::{PathMappings, ReferenceComponents, remap_instance_probes_many};

pub(crate) struct SchematicReferenceTransaction {
    pub(crate) before: BTreeMap<String, SchematicState>,
    pub(crate) after: BTreeMap<String, SchematicState>,
    pub(crate) references: ReferenceChanges,
    pub(crate) prepared_references: PreparedReferences,
}

impl ProjectWorkspace {
    pub(crate) fn schematic_reference_sources<'a>(
        &'a self,
        active_reference: &CellViewRef,
        active_schematic: &'a SchematicState,
    ) -> BTreeMap<String, &'a SchematicState> {
        let active = active_reference.key();
        let mut sources: BTreeMap<_, _> = self
            .schematic_buffers
            .iter()
            .filter(|(key, _)| !key.eq_ignore_ascii_case(&active))
            .map(|(key, source)| (key.clone(), source))
            .collect();
        sources.insert(active, active_schematic);
        sources
    }

    /// Callers first validate complete component candidates, including their
    /// local structural references. This adds every affected hierarchy path,
    /// configuration, saved output, probe and open-document occurrence.
    pub(crate) fn prepare_schematic_reference_transaction(
        &self,
        libraries: &LibraryManager,
        active_reference: &CellViewRef,
        active_schematic: &SchematicState,
        mut before: BTreeMap<String, SchematicState>,
        mut after: BTreeMap<String, SchematicState>,
    ) -> Result<SchematicReferenceTransaction, String> {
        if !before.keys().eq(after.keys()) {
            return Err("Reference edit documents do not match their original sources.".to_owned());
        }
        let active = active_reference.clone();
        let projected = self.schematic_reference_sources(active_reference, active_schematic);
        let mut configurations = self.configuration_sets.clone();
        let mut probe_roots: BTreeMap<String, PathMappings> = BTreeMap::new();
        if !before.is_empty() {
            for configuration in self.configuration_sets.configurations() {
                let resolution = self.resolve_hierarchy_for_reference(
                    libraries,
                    configuration.root(),
                    Some(configuration.id()),
                    &active,
                    active_schematic,
                )?;
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
                if self.configuration_sets.active_configuration_id() == Some(configuration.id()) {
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
            let root = self.simulation_root_reference();
            if let std::collections::btree_map::Entry::Vacant(entry) =
                probe_roots.entry(root.key().to_ascii_lowercase())
            {
                let resolution = self.resolve_hierarchy_for_reference(
                    libraries,
                    &root,
                    None,
                    &active,
                    active_schematic,
                )?;
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
                let root = reference_document_root(self, key)?;
                if probe_roots.contains_key(&root.key().to_ascii_lowercase()) {
                    continue;
                }
                let resolution = self.resolve_hierarchy_for_reference(
                    libraries,
                    &root,
                    None,
                    &active,
                    active_schematic,
                )?;
                probe_roots.insert(
                    root.key().to_ascii_lowercase(),
                    hierarchy_reference_paths(&root, &resolution, &before, &after, true)?,
                );
            }
        }
        let mut outputs = Vec::new();
        if let Some(mappings) =
            probe_roots.get(&self.simulation_root_reference().key().to_ascii_lowercase())
        {
            for record in &self.simulation_plan_payloads {
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
        for (key, source) in &projected {
            let root = reference_document_root(self, key)?;
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
            }
        }
        for schematic in after.values_mut() {
            schematic.clear_schematic_redo();
        }
        let mut references = ReferenceChanges::between(self, &configurations, outputs);
        for (key, source) in &before {
            references.add_instance_renames(
                &reference_from_key(key)?,
                &source.document().components,
                &after[key].document().components,
            );
        }
        let prepared_references = references.prepare(self, true)?;
        Ok(SchematicReferenceTransaction {
            before,
            after,
            references,
            prepared_references,
        })
    }
}

pub(crate) fn reference_from_key(key: &str) -> Result<CellViewRef, String> {
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
    before: &BTreeMap<String, SchematicState>,
    after: &BTreeMap<String, SchematicState>,
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
