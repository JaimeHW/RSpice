//! Deleting the selection as one undoable edit.

use super::super::deletion::DeletionSelection;
#[cfg(test)]
use super::super::{BusDeclaration, BusSlice, BusTapOrientation, DesignNoteKind};
use super::*;

impl SchematicState {
    /// Atomically remove selected complete objects, including net labels.
    ///
    /// This state-layer entry point owns the undo transaction so keyboard,
    /// menu, and context-menu deletion all have identical behavior. Wire
    /// segment and vertex selections remain editing handles and are not
    /// interpreted as requests to delete their parent wire.
    pub fn delete_selection(&mut self) -> bool {
        if self.session.read_only {
            return false;
        }

        let Some(edit) = self.design.delete_objects(DeletionSelection {
            components: &self.session.editor.selection.components,
            wires: &self.session.editor.selection.wires,
            junctions: self
                .session
                .editor
                .selection
                .junctions
                .iter()
                .map(|junction| junction.pos),
            net_labels: &self.session.editor.selection.net_labels,
            buses: &self.session.editor.selection.buses,
            bus_taps: &self.session.editor.selection.bus_taps,
            design_notes: &self.session.editor.selection.design_notes,
            documentation_shapes: &self.session.editor.selection.documentation_shapes,
            probes: &self.session.editor.selection.probes,
        }) else {
            return false;
        };
        self.session.editor.selection.clear();
        self.finish_document_edit(edit.committed);
        edit.committed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{DocumentationShapeGeometry, SchematicProbe};

    #[test]
    fn junction_deletion_is_one_undoable_transaction() {
        let point = Point::new(12, 8);
        let mut schematic = SchematicState::default();
        schematic
            .design
            .document_mut_for_test()
            .junctions
            .push(Junction::new(7, point));
        schematic.init_undo_history();
        schematic
            .session
            .editor
            .selection
            .select_only_junction(point);

        assert!(schematic.delete_selection());
        assert!(schematic.design.document().junctions.is_empty());
        assert!(schematic.session.editor.selection.is_empty());
        assert_eq!(schematic.undo_description(), Some("delete selection"));

        assert!(schematic.undo());
        assert_eq!(
            schematic.design.document().junctions,
            vec![Junction::new(7, point)]
        );
        assert!(
            !schematic.can_undo(),
            "deletion must create only one undo step"
        );
    }

    #[test]
    fn deleting_a_stale_selection_is_a_no_op() {
        let mut schematic = SchematicState::default();
        schematic.init_undo_history();
        schematic
            .session
            .editor
            .selection
            .select_only_junction(Point::new(9, 9));

        assert!(!schematic.delete_selection());
        assert!(!schematic.can_undo());
    }

    #[test]
    fn deletion_preserves_read_only_state_and_joins_outer_cancellation() {
        let label = NetLabel::new(1, Point::new(2, 2), "remove");
        let mut schematic = SchematicState::default();
        schematic
            .design
            .document_mut_for_test()
            .net_labels
            .push(label.clone());
        schematic
            .session
            .editor
            .selection
            .select_only_net_label(label.id);
        schematic.session.is_dirty = false;
        schematic.init_undo_history();
        let selection = schematic.session.editor.selection.clone();
        let topology = schematic.topology_version();
        let content = schematic.content_version();

        schematic.session.read_only = true;
        assert!(!schematic.delete_selection());
        assert_eq!(schematic.design.document().net_labels, vec![label.clone()]);
        assert_eq!(schematic.session.editor.selection, selection);
        assert_eq!(schematic.topology_version(), topology);
        assert_eq!(schematic.content_version(), content);
        assert!(!schematic.session.is_dirty);
        assert!(!schematic.can_undo());
        assert!(!schematic.has_pending_operation());

        schematic.session.read_only = false;
        schematic.begin_operation("outer edit");
        assert!(!schematic.delete_selection(), "the outer scope owns commit");
        assert!(schematic.design.document().net_labels.is_empty());
        assert!(schematic.session.editor.selection.is_empty());
        assert_eq!(schematic.topology_version(), topology + 1);
        assert_eq!(schematic.content_version(), content);
        assert!(schematic.session.is_dirty);
        assert!(schematic.has_pending_operation());
        assert!(!schematic.can_undo());
        assert!(schematic.cancel_operation());
        assert_eq!(schematic.design.document().net_labels, vec![label]);
        assert_eq!(schematic.session.editor.selection, selection);
        assert!(!schematic.session.is_dirty);
        assert!(!schematic.can_undo());
        assert!(!schematic.has_pending_operation());
    }

    #[test]
    fn wire_deletion_removes_newly_orphaned_junction_in_same_undo_step() {
        let point = Point::new(10, 10);
        let mut schematic = SchematicState::default();
        schematic.design.document_mut_for_test().wires = vec![
            Wire::new(1, vec![Point::new(0, 10), point]),
            Wire::new(2, vec![point, Point::new(10, 20)]),
        ];
        schematic.design.document_mut_for_test().junctions = vec![Junction::new(3, point)];
        schematic.init_undo_history();
        schematic.session.editor.selection.select_only_wire(1);

        assert!(schematic.delete_selection());
        assert_eq!(schematic.design.document().wires.len(), 1);
        assert!(schematic.design.document().junctions.is_empty());

        assert!(schematic.undo());
        assert_eq!(schematic.design.document().wires.len(), 2);
        assert_eq!(
            schematic.design.document().junctions,
            vec![Junction::new(3, point)]
        );
        assert!(!schematic.can_undo(), "cleanup belongs to the delete step");
    }

    #[test]
    fn deleting_bus_cascades_taps_and_undo_restores_both() {
        let declaration = BusDeclaration::parse("DATA[3:0]").unwrap();
        let bus = Bus::segment(21, Point::new(0, 0), Point::new(10, 0), Some(declaration)).unwrap();
        let tap = BusTap::new(
            22,
            &bus,
            Point::new(5, 0),
            Point::new(5, 5),
            BusSlice::parse("DATA[1]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        let mut schematic = SchematicState::default();
        schematic.design.document_mut_for_test().buses.push(bus);
        schematic.design.document_mut_for_test().bus_taps.push(tap);
        schematic.session.editor.selection.select_only_bus(21);
        assert!(schematic.delete_selection());
        assert!(
            schematic.design.document().buses.is_empty()
                && schematic.design.document().bus_taps.is_empty()
        );
        assert!(schematic.undo());
        assert_eq!(schematic.design.document().buses.len(), 1);
        assert_eq!(schematic.design.document().bus_taps.len(), 1);
    }

    #[test]
    fn label_deletion_is_one_undoable_redoable_transaction() {
        let retained = NetLabel::new(40, Point::new(1, 1), "retained");
        let removed = NetLabel::new(41, Point::new(2, 2), "removed");
        let mut schematic = SchematicState::default();
        schematic.design.document_mut_for_test().net_labels =
            vec![retained.clone(), removed.clone()];
        schematic.init_undo_history();
        schematic
            .session
            .editor
            .selection
            .select_only_net_label(removed.id);

        assert!(schematic.delete_selection());
        assert_eq!(
            schematic.design.document().net_labels,
            vec![retained.clone()]
        );
        assert_eq!(schematic.undo_description(), Some("delete selection"));
        assert!(schematic.undo());
        assert_eq!(
            schematic.design.document().net_labels,
            vec![retained, removed]
        );
        assert!(!schematic.can_undo());
        assert!(schematic.redo());
        assert_eq!(schematic.design.document().net_labels.len(), 1);
    }

    #[test]
    fn design_note_deletion_never_cleans_or_versions_electrical_objects() {
        let point = Point::new(90, 90);
        let note = DesignNote::new(
            42,
            Point::new(2, 2),
            DesignNoteKind::PlainText,
            "Bias network",
        )
        .unwrap();
        let mut schematic = SchematicState::default();
        schematic
            .design
            .document_mut_for_test()
            .junctions
            .push(Junction::new(7, point));
        schematic
            .design
            .document_mut_for_test()
            .design_notes
            .push(note.clone());
        schematic.init_undo_history();
        schematic
            .session
            .editor
            .selection
            .select_only_design_note(note.id);
        let topology = schematic.topology_version();

        assert!(schematic.delete_selection());
        assert!(schematic.design.document().design_notes.is_empty());
        assert_eq!(
            schematic.design.document().junctions,
            vec![Junction::new(7, point)]
        );
        assert_eq!(schematic.topology_version(), topology);
        assert!(schematic.undo());
        assert_eq!(schematic.design.document().design_notes, vec![note]);
        assert_eq!(
            schematic.design.document().junctions,
            vec![Junction::new(7, point)]
        );
        assert_eq!(schematic.topology_version(), topology);
    }

    #[test]
    fn documentation_shape_delete_and_undo_are_non_electrical() {
        let retained = DocumentationShape::new(
            43,
            DocumentationShapeGeometry::Line {
                start: Point::new(0, 0),
                end: Point::new(10, 10),
            },
        )
        .unwrap();
        let removed = DocumentationShape::new(
            44,
            DocumentationShapeGeometry::Callout {
                tip: Point::new(20, 20),
                elbow: Point::new(30, 20),
                box_corner: Point::new(40, 30),
            },
        )
        .unwrap();
        let marker = Point::new(90, 90);
        let mut schematic = SchematicState::default();
        schematic
            .design
            .document_mut_for_test()
            .documentation_shapes = vec![retained.clone(), removed.clone()];
        schematic
            .design
            .document_mut_for_test()
            .junctions
            .push(Junction::new(7, marker));

        schematic
            .session
            .editor
            .selection
            .select_only_documentation_shape(removed.id);
        schematic.init_undo_history();
        let topology = schematic.topology_version();
        assert!(schematic.delete_selection());
        assert_eq!(
            schematic.design.document().documentation_shapes,
            vec![retained.clone()]
        );
        assert_eq!(
            schematic.design.document().junctions,
            vec![Junction::new(7, marker)]
        );
        assert_eq!(schematic.topology_version(), topology);
        assert_eq!(schematic.undo_description(), Some("delete selection"));

        assert!(schematic.undo());
        assert_eq!(
            schematic.design.document().documentation_shapes,
            vec![retained, removed]
        );
        assert_eq!(
            schematic.design.document().junctions,
            vec![Junction::new(7, marker)]
        );
        assert_eq!(schematic.topology_version(), topology);
        assert!(!schematic.can_undo(), "deletion must create one undo step");
    }

    #[test]
    fn probe_delete_and_undo_are_non_electrical() {
        let probe =
            SchematicProbe::new(45, Point::new(10, 20), "V(out)", Some("V(out)".to_owned()))
                .unwrap();
        let marker = Point::new(90, 90);
        let mut schematic = SchematicState::default();
        schematic
            .design
            .document_mut_for_test()
            .probes
            .push(probe.clone());
        schematic
            .design
            .document_mut_for_test()
            .junctions
            .push(Junction::new(7, marker));

        schematic
            .session
            .editor
            .selection
            .select_only_probe(probe.id);
        schematic.init_undo_history();
        let topology = schematic.topology_version();
        assert!(schematic.delete_selection());
        assert!(schematic.design.document().probes.is_empty());
        assert_eq!(
            schematic.design.document().junctions,
            vec![Junction::new(7, marker)]
        );
        assert_eq!(schematic.topology_version(), topology);
        assert_eq!(schematic.undo_description(), Some("delete selection"));

        assert!(schematic.undo());
        assert_eq!(schematic.design.document().probes, vec![probe]);
        assert_eq!(
            schematic.design.document().junctions,
            vec![Junction::new(7, marker)]
        );
        assert_eq!(schematic.topology_version(), topology);
        assert!(!schematic.can_undo());
    }
}
