//! Prepare annotation names and their references before project publication.

use super::*;
use crate::state::workspace::HierarchyResolution;
use crate::state::{InstancePath, remap_instance_probes_many};

#[cfg(test)]
mod tests;

type PathMappings = Vec<(InstancePath, InstancePath)>;

impl AppState {
    pub(crate) fn prepare_design_management_schematic_transaction(
        &self,
        candidate: &DesignManagementCatalog,
    ) -> Result<DesignManagementSchematicTransaction, String> {
        candidate.validate().map_err(|error| error.to_string())?;
        let active = self.workspace.active_schematic_reference();
        let mut projected = self.workspace.schematic_buffers.clone();
        projected.insert(active.key(), self.schematic.clone());
        let existing = self
            .workspace
            .design_management
            .annotation()
            .effective_mappings();
        let mut names: BTreeMap<String, BTreeMap<u64, String>> = BTreeMap::new();
        for (object, mapping) in candidate.annotation().effective_mappings() {
            if existing.get(&object) == Some(&mapping) {
                continue;
            }
            let key = projected
                .keys()
                .find(|key| key.eq_ignore_ascii_case(object.cell_view_key()))
                .ok_or_else(|| {
                    format!(
                        "Annotation schematic '{}' is unavailable.",
                        object.cell_view_key()
                    )
                })?;
            let component = projected[key]
                .components
                .iter()
                .find(|component| component.id == object.object_id())
                .ok_or_else(|| {
                    format!(
                        "Annotation object {} no longer exists in '{key}'.",
                        object.object_id()
                    )
                })?;
            if component.name == mapping.new_reference {
                continue;
            }
            if component.name != mapping.old_reference {
                return Err(format!(
                    "Annotation cannot be published because {} in '{key}' changed from '{}'.",
                    component.name, mapping.old_reference
                ));
            }
            names
                .entry(key.clone())
                .or_default()
                .insert(component.id, mapping.new_reference);
        }
        let mut before = BTreeMap::new();
        let mut after = BTreeMap::new();
        for (key, names) in names {
            let source = &projected[&key];
            validate_annotation_document(self, &key, source)?;
            let mut candidate = source.clone();
            candidate.components = source.prepare_component_renames(&names)?;
            candidate.is_dirty = true;
            candidate.bump_topology_version();
            before.insert(key.clone(), source.clone());
            after.insert(key, candidate);
        }
        let mut configurations = self.workspace.configuration_sets.clone();
        let mut probe_roots: BTreeMap<String, PathMappings> = BTreeMap::new();
        if !before.is_empty() {
            for configuration in self.workspace.configuration_sets.configurations() {
                let resolution = self.workspace.resolve_hierarchy_for_reference(
                    &self.library_manager,
                    configuration.root(),
                    Some(configuration.id()),
                    &active,
                    &self.schematic,
                )?;
                let mappings =
                    annotation_paths(configuration.root(), &resolution, &before, &after, false)?;
                configurations
                    .remap_configuration_instance_paths(configuration.id(), &mappings)
                    .map_err(|error| error.to_string())?;
                if self.workspace.configuration_sets.active_configuration_id()
                    == Some(configuration.id())
                {
                    probe_roots.insert(
                        configuration.root().key().to_ascii_lowercase(),
                        annotation_paths(configuration.root(), &resolution, &before, &after, true)?,
                    );
                }
            }
            let root = self.workspace.simulation_root_reference();
            if let std::collections::btree_map::Entry::Vacant(entry) =
                probe_roots.entry(root.key().to_ascii_lowercase())
            {
                let resolution = self.workspace.resolve_hierarchy_for_reference(
                    &self.library_manager,
                    &root,
                    None,
                    &active,
                    &self.schematic,
                )?;
                entry.insert(annotation_paths(&root, &resolution, &before, &after, true)?);
            }
            for (key, source) in &projected {
                if source.probes.is_empty() {
                    continue;
                }
                let root = annotation_document_root(self, key)?;
                if probe_roots.contains_key(&root.key().to_ascii_lowercase()) {
                    continue;
                }
                let resolution = self.workspace.resolve_hierarchy_for_reference(
                    &self.library_manager,
                    &root,
                    None,
                    &active,
                    &self.schematic,
                )?;
                probe_roots.insert(
                    root.key().to_ascii_lowercase(),
                    annotation_paths(&root, &resolution, &before, &after, true)?,
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
        for (key, source) in &projected {
            let root = annotation_document_root(self, key)?;
            let Some(mappings) = probe_roots.get(&root.key().to_ascii_lowercase()) else {
                continue;
            };
            if mappings.is_empty() || source.probes.is_empty() {
                continue;
            }
            let mut probes = source.probes.clone();
            let mut changed = false;
            for probe in &mut probes {
                if let Some(expression) = &probe.source_expression
                    && let Some(rewritten) = remap_instance_probes_many(expression, mappings)?
                {
                    if probe.reference == *expression {
                        probe.reference = rewritten.clone();
                    }
                    probe.source_expression = Some(rewritten);
                    probe.validate()?;
                    changed = true;
                }
            }
            if changed {
                validate_annotation_document(self, key, source)?;
                before.entry(key.clone()).or_insert_with(|| source.clone());
                let candidate = after.entry(key.clone()).or_insert_with(|| source.clone());
                candidate.probes = probes;
                candidate.is_dirty = true;
            }
        }
        for schematic in after.values_mut() {
            schematic.undo_history.clear_redo();
        }
        let mut references = ReferenceChanges::between(self, &configurations, outputs);
        for (key, source) in &before {
            references.add_instance_renames(
                &reference_from_key(key)?,
                &source.components,
                &after[key].components,
            );
        }
        let prepared_references = references.prepare(self, true)?;
        Ok(DesignManagementSchematicTransaction {
            before,
            after,
            references,
            prepared_references,
        })
    }

    pub(crate) fn apply_design_management_schematic_transaction(
        &mut self,
        transaction: &DesignManagementSchematicTransaction,
    ) {
        transaction.prepared_references.clone().publish(self);
        let active_key = self.workspace.active_schematic_reference().key();
        for (key, schematic) in &transaction.after {
            if key.eq_ignore_ascii_case(&active_key) {
                self.schematic = schematic.clone();
                self.workspace
                    .schematic_buffers
                    .insert(active_key.clone(), schematic.clone());
            } else if let Some(existing_key) = self
                .workspace
                .schematic_buffers
                .keys()
                .find(|candidate| candidate.eq_ignore_ascii_case(key))
                .cloned()
            {
                self.workspace
                    .schematic_buffers
                    .insert(existing_key, schematic.clone());
            }
            if let Some(open) = self
                .workspace
                .open_views
                .iter_mut()
                .find(|open| open.reference.key().eq_ignore_ascii_case(key))
            {
                open.dirty = true;
            }
        }
    }
}

pub(super) fn reference_from_key(key: &str) -> Result<CellViewRef, String> {
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

pub(super) fn validate_annotation_document(
    state: &AppState,
    key: &str,
    source: &SchematicState,
) -> Result<(), String> {
    let reference = reference_from_key(key)?;
    if !state.project_lifecycle.project_open || document_read_only(state, &reference) {
        return Err(format!(
            "Annotation requires an open, writable schematic '{key}'."
        ));
    }
    if source.has_pending_operation() {
        return Err(format!(
            "Finish or cancel the schematic gesture in '{key}' before applying annotation."
        ));
    }
    Ok(())
}

fn annotation_local_paths(
    root: &CellViewRef,
    before: &BTreeMap<String, SchematicState>,
    after: &BTreeMap<String, SchematicState>,
    emitted: bool,
) -> Result<PathMappings, String> {
    let mut paths = Vec::new();
    append_document_paths(
        root,
        &InstancePath::root(),
        before,
        after,
        emitted,
        &mut paths,
    )?;
    Ok(paths)
}

fn append_document_paths(
    reference: &CellViewRef,
    parent: &InstancePath,
    before: &BTreeMap<String, SchematicState>,
    after: &BTreeMap<String, SchematicState>,
    emitted: bool,
    paths: &mut PathMappings,
) -> Result<(), String> {
    let Some((key, source)) = before
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(&reference.key()))
    else {
        return Ok(());
    };
    let candidates: BTreeMap<_, _> = after[key]
        .components
        .iter()
        .map(|component| (component.id, component))
        .collect();
    for component in &source.components {
        let candidate = candidates[&component.id];
        if component.name == candidate.name || component.kind.spice_prefix().is_empty() {
            continue;
        }
        // Hierarchy resolution names placements by their authored instance
        // names. Primitive current probes instead name emitted SPICE cards.
        let emitted = emitted && component.kind != ComponentType::CellInstance;
        let from = if emitted {
            component.emitted_instance_name()
        } else {
            component.name.clone()
        };
        let to = if emitted {
            candidate.emitted_instance_name()
        } else {
            candidate.name.clone()
        };
        paths.push((
            parent.child(&from).map_err(|error| error.to_string())?,
            parent.child(&to).map_err(|error| error.to_string())?,
        ));
    }
    Ok(())
}

fn annotation_paths(
    root: &CellViewRef,
    resolution: &HierarchyResolution,
    before: &BTreeMap<String, SchematicState>,
    after: &BTreeMap<String, SchematicState>,
    emitted: bool,
) -> Result<PathMappings, String> {
    let mut paths = annotation_local_paths(root, before, after, emitted)?;
    for binding in &resolution.bindings {
        if !binding.status.is_resolved() {
            continue;
        }
        for path in &binding.instance_paths {
            let parent = InstancePath::parse(path).map_err(|error| error.to_string())?;
            if parent.is_root() {
                continue;
            }
            append_document_paths(
                &binding.reference,
                &parent,
                before,
                after,
                emitted,
                &mut paths,
            )?;
        }
    }
    let mut unique = BTreeMap::new();
    for (from, to) in paths {
        if let Some((_, previous)) = unique.insert(from.fold_key(), (from.clone(), to.clone()))
            && previous != to
        {
            return Err(format!(
                "Annotation resolves '{from}' to incompatible final names."
            ));
        }
    }
    // Compose ancestor and descendant renames from original prefixes, never
    // by applying one rename to a previous rename's destination.
    unique
        .values()
        .map(|(from, _)| {
            let mut prefix = InstancePath::root();
            let mut destination = InstancePath::root();
            for segment in from.segments() {
                prefix = prefix.child(segment).map_err(|error| error.to_string())?;
                let name = unique
                    .get(&prefix.fold_key())
                    .and_then(|(_, to)| to.segments().last())
                    .unwrap_or(segment);
                destination = destination.child(name).map_err(|error| error.to_string())?;
            }
            Ok((from.clone(), destination))
        })
        .collect()
}

fn annotation_document_root(state: &AppState, key: &str) -> Result<CellViewRef, String> {
    if let Some(open) = state
        .workspace
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
