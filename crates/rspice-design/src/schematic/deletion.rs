//! Deletion of complete schematic objects and their dependent markers.

use super::document::SchematicDocument;
use super::junction_edit;
use rspice_design_model::Point;
use std::collections::HashSet;

/// Complete object identities to delete. Wire handles are editor state and do
/// not imply deletion of their parent wire. Junctions are identified by position.
pub struct DeletionSelection<'a, J> {
    pub components: &'a HashSet<u64>,
    pub wires: &'a HashSet<u64>,
    pub junctions: J,
    pub net_labels: &'a HashSet<u64>,
    pub buses: &'a HashSet<u64>,
    pub bus_taps: &'a HashSet<u64>,
    pub design_notes: &'a HashSet<u64>,
    pub documentation_shapes: &'a HashSet<u64>,
    pub probes: &'a HashSet<u64>,
}

/// A deletion bound to the document checked during preparation. Dropping it
/// leaves the document unchanged; consuming it commits to that same document.
pub struct ObjectDeletion<'document, 'selection, J> {
    document: &'document mut SchematicDocument,
    selection: DeletionSelection<'selection, J>,
    removes_electrical_object: bool,
}

impl<'document, 'selection, J> ObjectDeletion<'document, 'selection, J>
where
    J: Iterator<Item = Point> + Clone,
{
    /// Return no edit when none of the requested complete objects exist.
    pub fn prepare(
        document: &'document mut SchematicDocument,
        selection: DeletionSelection<'selection, J>,
    ) -> Option<Self> {
        let has_live_object = document
            .components
            .iter()
            .any(|component| selection.components.contains(&component.id))
            || document
                .wires
                .iter()
                .any(|wire| selection.wires.contains(&wire.id))
            || document
                .junctions
                .iter()
                .any(|junction| selection.junctions.clone().any(|pos| pos == junction.pos))
            || document
                .net_labels
                .iter()
                .any(|label| selection.net_labels.contains(&label.id))
            || document
                .buses
                .iter()
                .any(|bus| selection.buses.contains(&bus.id))
            || document
                .bus_taps
                .iter()
                .any(|tap| selection.bus_taps.contains(&tap.id))
            || document
                .design_notes
                .iter()
                .any(|note| selection.design_notes.contains(&note.id))
            || document
                .documentation_shapes
                .iter()
                .any(|shape| selection.documentation_shapes.contains(&shape.id))
            || document
                .probes
                .iter()
                .any(|probe| selection.probes.contains(&probe.id));
        if !has_live_object {
            return None;
        }
        let removes_electrical_object = document
            .components
            .iter()
            .any(|component| selection.components.contains(&component.id))
            || document
                .wires
                .iter()
                .any(|wire| selection.wires.contains(&wire.id))
            || document
                .junctions
                .iter()
                .any(|junction| selection.junctions.clone().any(|pos| pos == junction.pos))
            || document
                .net_labels
                .iter()
                .any(|label| selection.net_labels.contains(&label.id))
            || document
                .buses
                .iter()
                .any(|bus| selection.buses.contains(&bus.id))
            || document
                .bus_taps
                .iter()
                .any(|tap| selection.bus_taps.contains(&tap.id));

        Some(Self {
            document,
            selection,
            removes_electrical_object,
        })
    }

    /// The unchanged document, available for history capture before commit.
    pub fn document(&self) -> &SchematicDocument {
        self.document
    }

    /// Remove the prepared objects, preserving survivor order. Returns whether
    /// electrical objects were removed and topology caches must be invalidated.
    pub fn commit(self) -> bool {
        let Self {
            document,
            selection,
            removes_electrical_object,
        } = self;
        document
            .components
            .retain(|component| !selection.components.contains(&component.id));
        document
            .wires
            .retain(|wire| !selection.wires.contains(&wire.id));
        let removed_bus_ids: std::collections::HashSet<u64> = document
            .buses
            .iter()
            .filter(|bus| selection.buses.contains(&bus.id))
            .map(|bus| bus.id)
            .collect();
        document
            .buses
            .retain(|bus| !removed_bus_ids.contains(&bus.id));
        document.bus_taps.retain(|tap| {
            !removed_bus_ids.contains(&tap.bus_id) && !selection.bus_taps.contains(&tap.id)
        });
        // A wire deletion may invalidate connection markers that were not
        // explicitly selected. Keep that lifecycle cleanup inside this
        // same undo transaction and topology update.
        if removes_electrical_object {
            junction_edit::remove_orphan_junctions(document);
        }
        document
            .junctions
            .retain(|junction| !selection.junctions.clone().any(|pos| pos == junction.pos));
        document
            .net_labels
            .retain(|label| !selection.net_labels.contains(&label.id));
        document
            .design_notes
            .retain(|note| !selection.design_notes.contains(&note.id));
        document
            .documentation_shapes
            .retain(|shape| !selection.documentation_shapes.contains(&shape.id));
        document
            .probes
            .retain(|probe| !selection.probes.contains(&probe.id));

        removes_electrical_object
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        history::SchematicSnapshot,
        net_label::{Junction, NetLabel},
    };
    use super::*;

    #[test]
    fn discarded_deletion_preserves_document_and_stale_targets_do_not_prepare() {
        let mut document = SchematicDocument {
            net_labels: vec![
                NetLabel::new(1, Point::new(0, 0), "remove"),
                NetLabel::new(2, Point::new(1, 1), "retain"),
            ],
            junctions: vec![Junction::new(3, Point::new(10, 10))],
            ..SchematicDocument::default()
        };
        let empty = HashSet::new();
        let stale = HashSet::from([999]);
        let selected = HashSet::from([1]);
        let selection = |labels| DeletionSelection {
            components: &empty,
            wires: &empty,
            junctions: std::iter::empty(),
            net_labels: labels,
            buses: &empty,
            bus_taps: &empty,
            design_notes: &empty,
            documentation_shapes: &empty,
            probes: &empty,
        };
        let before = SchematicSnapshot::capture(&document);
        assert!(ObjectDeletion::prepare(&mut document, selection(&stale)).is_none());
        assert!(before.is_equal_document(&document));
        {
            let deletion = ObjectDeletion::prepare(&mut document, selection(&selected)).unwrap();
            assert!(before.is_equal_document(deletion.document()));
        }
        assert!(before.is_equal_document(&document));
        assert!(
            ObjectDeletion::prepare(&mut document, selection(&selected))
                .unwrap()
                .commit()
        );
        assert_eq!(
            document.net_labels,
            vec![NetLabel::new(2, Point::new(1, 1), "retain")]
        );
        assert!(document.junctions.is_empty());
    }
}
