//! Restore approved annotation receipts into canonical project documents.

use super::*;

impl ProjectWorkspace {
    /// Prepare the complete current reference closure before replacing any
    /// source. Reading a project may materialize its already-approved journal;
    /// it must neither invent an edit nor partially update its consumers.
    pub(crate) fn restore_pending_annotation(
        &mut self,
        libraries: &LibraryManager,
    ) -> Result<usize, String> {
        let annotation = self.design_management.annotation();
        if annotation.journal().is_empty() {
            return Ok(0);
        }
        self.design_management
            .validate()
            .map_err(|error| error.to_string())?;
        let sources = self
            .schematic_buffers
            .iter()
            .flat_map(|(key, schematic)| {
                schematic.components.iter().map(move |component| {
                    crate::state::SchematicObjectKey::new(key, component.id)
                        .map(|object| (object, component.name.as_str()))
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())?;
        let assignments = annotation
            .projected_reference_assignments(sources)
            .map_err(|error| error.to_string())?;
        if assignments.is_empty() {
            return Ok(0);
        }
        let count = assignments.len();
        let mut by_document: BTreeMap<String, BTreeMap<u64, String>> = BTreeMap::new();
        for (object, name) in assignments {
            let key = self
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
            let source = &self.schematic_buffers[&key];
            let mut candidate = source.clone();
            candidate.components = source.prepare_component_renames(&names).map_err(|reason| {
                format!("Cannot restore reference annotation in '{key}': {reason}")
            })?;
            candidate.is_dirty = true;
            candidate.bump_topology_version();
            before.insert(key.clone(), source.clone());
            after.insert(key, candidate);
        }
        // The overlay must be an actual source, including projects last saved
        // with a non-schematic document active. It never changes navigation.
        let requested = self.active_schematic_reference();
        let active_key = self
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
        let transaction = self.prepare_schematic_reference_transaction(
            libraries,
            &active_reference,
            &self.schematic_buffers[active_key],
            before,
            after,
        )?;
        transaction.prepared_references.publish(self);
        for (key, schematic) in transaction.after {
            for open in &mut self.open_views {
                if open.reference.key().eq_ignore_ascii_case(&key) {
                    open.dirty = true;
                }
            }
            self.schematic_buffers.insert(key, schematic);
        }
        Ok(count)
    }
}
