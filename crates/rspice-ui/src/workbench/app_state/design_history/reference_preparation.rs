//! Prepare reference edits across configured occurrences and buffered documents.

use super::*;
use crate::state::workspace::HierarchyResolution;
use crate::state::{InstancePath, remap_instance_probes_many};

type PathMappings = Vec<(InstancePath, InstancePath)>;

impl AppState {
    pub(super) fn schematic_reference_sources(&self) -> BTreeMap<String, &SchematicState> {
        let active = self.workspace.active_schematic_reference().key();
        let mut sources: BTreeMap<_, _> = self
            .workspace
            .schematic_buffers
            .iter()
            .filter(|(key, _)| !key.eq_ignore_ascii_case(&active))
            .map(|(key, source)| (key.clone(), source))
            .collect();
        sources.insert(active, &self.schematic);
        sources
    }

    /// Callers first validate complete component candidates, including their
    /// local structural references. This adds every affected hierarchy path,
    /// configuration, saved output, probe and open-document occurrence.
    pub(super) fn prepare_schematic_reference_transaction(
        &self,
        mut before: BTreeMap<String, SchematicState>,
        mut after: BTreeMap<String, SchematicState>,
    ) -> Result<SchematicReferenceTransaction, String> {
        if !before.keys().eq(after.keys()) {
            return Err("Reference edit documents do not match their original sources.".to_owned());
        }
        for (key, source) in &before {
            validate_reference_document(self, key, source)?;
        }
        let active = self.workspace.active_schematic_reference();
        let projected = self.schematic_reference_sources();
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
                let resolution = self.workspace.resolve_hierarchy_for_reference(
                    &self.library_manager,
                    &root,
                    None,
                    &active,
                    &self.schematic,
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
                if source.probes.is_empty() {
                    continue;
                }
                let root = reference_document_root(self, key)?;
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
        for (key, source) in &projected {
            let root = reference_document_root(self, key)?;
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
                validate_reference_document(self, key, source)?;
                before
                    .entry(key.clone())
                    .or_insert_with(|| (*source).clone());
                let candidate = after
                    .entry(key.clone())
                    .or_insert_with(|| (*source).clone());
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
        Ok(SchematicReferenceTransaction {
            before,
            after,
            references,
            prepared_references,
        })
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

pub(super) fn validate_reference_document(
    state: &AppState,
    key: &str,
    source: &SchematicState,
) -> Result<(), String> {
    let reference = reference_from_key(key)?;
    if !state.project_lifecycle.project_open || document_read_only(state, &reference) {
        return Err(format!(
            "Reference editing requires an open, writable schematic '{key}'."
        ));
    }
    if source.has_pending_operation() {
        return Err(format!(
            "Finish or cancel the schematic gesture in '{key}' before changing references."
        ));
    }
    Ok(())
}

fn local_reference_paths(
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

fn hierarchy_reference_paths(
    root: &CellViewRef,
    resolution: &HierarchyResolution,
    before: &BTreeMap<String, SchematicState>,
    after: &BTreeMap<String, SchematicState>,
    emitted: bool,
) -> Result<PathMappings, String> {
    let mut paths = local_reference_paths(root, before, after, emitted)?;
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
                "Reference editing resolves '{from}' to incompatible final names."
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

fn reference_document_root(state: &AppState, key: &str) -> Result<CellViewRef, String> {
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
