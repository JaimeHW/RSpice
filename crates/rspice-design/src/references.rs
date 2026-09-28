//! Hierarchy reference renames and lossless probe-expression updates.

use crate::hierarchy::HierarchyResolution;
use crate::schematic::{
    component::Component, component_type::ComponentType, probe::SchematicProbe,
};
use rspice_app_types::hierarchy_path::{InstancePath, ProbeTarget};
use rspice_design_model::cell_view::CellViewRef;
use std::collections::BTreeMap;

mod probe_rewrite;
pub use probe_rewrite::remap_instance_probes_many;

pub type PathMappings = Vec<(InstancePath, InstancePath)>;

/// Before/after components for a validated reference edit. Every original
/// component identity must be present in the replacement slice.
pub struct ReferenceComponents<'a> {
    pub before: &'a [Component],
    pub after: &'a [Component],
}

fn local_reference_paths<'a>(
    root: &CellViewRef,
    components: &impl Fn(&CellViewRef) -> Option<ReferenceComponents<'a>>,
    emitted: bool,
) -> Result<PathMappings, String> {
    let mut paths = Vec::new();
    append_document_paths(root, &InstancePath::root(), components, emitted, &mut paths)?;
    Ok(paths)
}

fn append_document_paths<'a>(
    reference: &CellViewRef,
    parent: &InstancePath,
    components: &impl Fn(&CellViewRef) -> Option<ReferenceComponents<'a>>,
    emitted: bool,
    paths: &mut PathMappings,
) -> Result<(), String> {
    let Some(edit) = components(reference) else {
        return Ok(());
    };
    let candidates: BTreeMap<_, _> = edit
        .after
        .iter()
        .map(|component| (component.id, component))
        .collect();
    for component in edit.before {
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

/// Compose simultaneous renames in the resolved hierarchy's original namespace.
pub fn hierarchy_reference_paths<'a>(
    root: &CellViewRef,
    resolution: &HierarchyResolution,
    components: &impl Fn(&CellViewRef) -> Option<ReferenceComponents<'a>>,
    emitted: bool,
) -> Result<PathMappings, String> {
    let mut paths = local_reference_paths(root, components, emitted)?;
    for binding in &resolution.bindings {
        if !binding.status.is_resolved() {
            continue;
        }
        for path in &binding.instance_paths {
            let parent = InstancePath::parse(path).map_err(|error| error.to_string())?;
            if parent.is_root() {
                continue;
            }
            append_document_paths(&binding.reference, &parent, components, emitted, &mut paths)?;
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

/// Prepare rewritten flags without changing the source document.
pub fn remap_schematic_probes(
    source: &[SchematicProbe],
    mappings: &[(InstancePath, InstancePath)],
) -> Result<Option<Vec<SchematicProbe>>, String> {
    if mappings.is_empty() || source.is_empty() {
        return Ok(None);
    }
    let mut probes = source.to_vec();
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
    Ok(changed.then_some(probes))
}

#[cfg(test)]
mod tests;
