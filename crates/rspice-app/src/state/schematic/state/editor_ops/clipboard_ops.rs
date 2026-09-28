//! Cut, copy, and paste.
//!
//! Capturing a selection with everything it needs to stand alone — the wires
//! between the copied objects, their junctions, and their labels — and
//! re-anchoring it at the paste point with fresh object identities.

#[cfg(test)]
use super::super::super::SchematicProbe;
use super::super::super::clipboard_edit;
#[cfg(test)]
use super::super::super::{BusDeclaration, BusSlice, BusTapOrientation, DesignNoteKind};
use super::super::*;

impl SchematicState {
    // =========================================================================
    // Clipboard Operations
    // =========================================================================

    /// Copy selected components, wires, explicit junctions, net labels, buses,
    /// and bus taps to the typed schematic clipboard.
    ///
    /// In addition to explicitly selected wires, automatically includes
    /// any wires that have both endpoints connected to selected components.
    /// This preserves circuit connectivity when copying/pasting.
    pub fn copy_selection(&mut self) {
        if self.session.selection.is_empty() {
            return;
        }

        self.session.clipboard = self.capture_complete_selection_resolved(|component| {
            component
                .terminal_positions()
                .into_iter()
                .map(|(_, point)| point)
                .collect()
        });
    }

    /// Capture the current complete-object selection without modifying the
    /// application clipboard.
    ///
    /// This is the common selection authority used by copy and by repeated
    /// structure transactions.  The caller supplies resolved terminal
    /// geometry so authored cell symbols are treated identically to generated
    /// primitives.  Whole conductors explicitly selected by the user are
    /// retained, as are conductors whose two endpoints terminate on selected
    /// components.  Bus-tap ownership and explicit junction intent are closed
    /// over in the same deterministic way as Copy.
    pub(crate) fn capture_complete_selection_resolved(
        &self,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) -> ClipboardData {
        clipboard_edit::capture_complete_selection(
            &self.design.document(),
            clipboard_edit::CopySelection {
                components: &self.session.selection.components,
                wires: &self.session.selection.wires,
                net_labels: &self.session.selection.net_labels,
                design_notes: &self.session.selection.design_notes,
                documentation_shapes: &self.session.selection.documentation_shapes,
                probes: &self.session.selection.probes,
                buses: &self.session.selection.buses,
                bus_taps: &self.session.selection.bus_taps,
            },
            self.session
                .selection
                .junctions
                .iter()
                .map(|junction| junction.pos),
            terminal_points_for,
        )
    }

    /// Check if clipboard has content
    pub fn can_paste(&self) -> bool {
        self.session.clipboard.has_content()
    }

    /// Paste clipboard contents at the given position (one undo entry)
    pub fn paste_at(&mut self, pos: Point) -> bool {
        self.paste_at_checked(pos).unwrap_or(false)
    }

    /// Paste with a diagnostic when structural references cannot be preserved.
    pub fn paste_at_checked(&mut self, pos: Point) -> Result<bool, String> {
        if self.session.read_only || !self.can_paste() {
            return Ok(false);
        }
        let Some(edit) = self.design.paste_at(&self.session.clipboard, pos)? else {
            return Ok(false);
        };
        let pasted = edit.value;
        let committed = edit.committed;
        let has_content = pasted.has_content();
        if has_content {
            self.session.selection.clear();
            for object in pasted.components {
                self.session.selection.select_component(object.id);
            }
            for object in pasted.wires {
                self.session.selection.select_wire(object.id);
            }
            for object in pasted.net_labels {
                self.session.selection.select_net_label(object.id);
            }
            for object in pasted.design_notes {
                self.session.selection.select_design_note(object.id);
            }
            for object in pasted.documentation_shapes {
                self.session.selection.select_documentation_shape(object.id);
            }
            for object in pasted.probes {
                self.session.selection.select_probe(object.id);
            }
            for object in pasted.buses {
                self.session.selection.select_bus(object.id);
            }
            for object in pasted.bus_taps {
                self.session.selection.select_bus_tap(object.id);
            }
            for object in pasted.junctions {
                self.session.selection.select_junction(object.pos);
            }
        }
        if has_content || committed {
            self.finish_document_edit(committed);
        }
        Ok(committed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::DocumentationShapeGeometry;

    #[test]
    fn note_only_clipboard_pastes_at_exact_target_without_junction_constraints() {
        let mut schematic = SchematicState::default();
        let note = DesignNote::new(
            77,
            Point::new(10, 20),
            DesignNoteKind::ReviewNote,
            "Check bias path",
        )
        .unwrap();
        schematic
            .design
            .document_mut_for_test()
            .design_notes
            .push(note.clone());
        schematic.session.selection.select_only_design_note(note.id);
        schematic.copy_selection();
        assert_eq!(schematic.session.clipboard.origin, note.pos);
        assert_eq!(schematic.session.clipboard.design_notes, vec![note.clone()]);

        let topology = schematic.topology_version();
        assert!(schematic.paste_at(Point::new(100, 120)));
        assert_eq!(schematic.design.document().design_notes.len(), 2);
        let pasted = schematic.design.document().design_notes.last().unwrap();
        assert_ne!(pasted.id, note.id);
        assert_eq!(pasted.pos, Point::new(100, 120));
        assert_eq!(pasted.kind, DesignNoteKind::ReviewNote);
        assert_eq!(pasted.text, note.text);
        assert_eq!(
            pasted.review.as_ref().unwrap().record_id,
            format!("NOTE-{:04}", pasted.id)
        );
        assert_eq!(schematic.topology_version(), topology);
        assert!(schematic.undo());
        assert_eq!(schematic.design.document().design_notes, vec![note]);
        assert_eq!(schematic.topology_version(), topology);
    }

    #[test]
    fn clipboard_drops_malformed_wires_from_corrupt_import_state() {
        let mut schematic = SchematicState::default();
        schematic
            .design
            .document_mut_for_test()
            .wires
            .push(Wire::new(10, Vec::new()));
        schematic
            .design
            .document_mut_for_test()
            .wires
            .push(Wire::new(11, vec![Point::new(5, 5)]));
        schematic.session.selection.select_wire(10);
        schematic.session.selection.select_wire(11);
        let original_wire_count = schematic.design.document().wires.len();

        schematic.copy_selection();

        assert!(
            schematic.session.clipboard.wires.is_empty(),
            "malformed wires must not propagate into clipboard state"
        );
        schematic.paste_at(Point::new(20, 20));
        assert_eq!(
            schematic.design.document().wires.len(),
            original_wire_count,
            "paste must not create additional invalid wires"
        );
    }

    #[test]
    fn junction_only_clipboard_pastes_only_on_a_valid_intersection() {
        let source = Point::new(20, 20);
        let target = Point::new(80, 80);
        let mut schematic = SchematicState::default();
        schematic.design.document_mut_for_test().wires = vec![
            Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
            Wire::new(2, vec![Point::new(20, 0), Point::new(20, 40)]),
            Wire::new(3, vec![Point::new(60, 80), Point::new(100, 80)]),
            Wire::new(4, vec![Point::new(80, 60), Point::new(80, 100)]),
        ];
        schematic.add_junction(source);
        schematic.session.selection.select_only_junction(source);
        schematic.copy_selection();

        assert!(schematic.can_paste());
        assert_eq!(schematic.session.clipboard.origin, source);
        assert!(schematic.paste_at(Point::new(target.x + 1, target.y - 1)));

        assert!(schematic.has_junction(target));
        assert!(schematic.session.selection.has_junction(target));
        assert!(schematic.can_undo());
    }

    #[test]
    fn junction_only_paste_rejects_empty_space_without_an_undo_step() {
        let source = Point::new(20, 20);
        let mut schematic = SchematicState::default();
        schematic.design.document_mut_for_test().wires = vec![
            Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
            Wire::new(2, vec![Point::new(20, 0), Point::new(20, 40)]),
        ];
        schematic.add_junction(source);
        schematic.session.selection.select_only_junction(source);
        schematic.copy_selection();

        assert!(!schematic.paste_at(Point::new(200, 200)));
        assert!(!schematic.can_undo());
        assert_eq!(schematic.design.document().junctions.len(), 1);
    }

    #[test]
    fn bus_copy_paste_remaps_tap_ownership_and_undoes_atomically() {
        let declaration = BusDeclaration::parse("DATA[3:0]").unwrap();
        let bus = Bus::segment(50, Point::new(0, 0), Point::new(20, 0), Some(declaration)).unwrap();
        let tap = BusTap::new(
            51,
            &bus,
            Point::new(10, 0),
            Point::new(10, 5),
            BusSlice::parse("DATA[2]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        let mut schematic = SchematicState::default();
        schematic.design.document_mut_for_test().buses.push(bus);
        schematic.design.document_mut_for_test().bus_taps.push(tap);
        schematic.recalculate_runtime_state();
        let original_bus_id = schematic.design.document().buses[0].id;
        schematic.session.selection.select_only_bus(original_bus_id);
        schematic.copy_selection();
        let clipboard_origin = schematic.session.clipboard.origin;
        schematic.clear_undo_history();

        assert!(schematic.paste_at(Point::new(100, 100)));
        assert_eq!(schematic.design.document().buses.len(), 2);
        assert_eq!(schematic.design.document().bus_taps.len(), 2);
        let pasted_bus_id = schematic
            .design
            .document()
            .buses
            .iter()
            .find(|bus| bus.id != original_bus_id)
            .unwrap()
            .id;
        assert!(schematic.design.document().bus_taps.iter().any(|tap| {
            tap.bus_id == pasted_bus_id
                && tap.connection_point
                    == Point::new(10 + 100 - clipboard_origin.x, 5 + 100 - clipboard_origin.y)
        }));
        assert!(schematic.undo());
        assert_eq!(schematic.design.document().buses.len(), 1);
        assert_eq!(schematic.design.document().bus_taps.len(), 1);
    }

    #[test]
    fn fully_rejected_typed_payload_is_a_true_no_op() {
        let mut schematic = SchematicState::default();
        let component_id = schematic.add_component(ComponentType::Resistor, Point::origin());
        schematic
            .session
            .selection
            .select_only_component(component_id);
        schematic.session.is_dirty = false;
        schematic.clear_undo_history();
        let topology_before = schematic.topology_version();
        let invalid_bus = Bus {
            id: 90,
            points: vec![Point::new(1, 1)],
            declaration: None,
        };
        let orphan_tap = BusTap {
            id: 91,
            bus_id: 90,
            bus_point: Point::new(1, 1),
            connection_point: Point::new(2, 1),
            slice: BusSlice::parse("DATA[0]").unwrap(),
            orientation: BusTapOrientation::Right,
        };
        schematic.session.clipboard = ClipboardData {
            buses: vec![invalid_bus],
            bus_taps: vec![orphan_tap],
            junctions: vec![Point::new(100, 100)],
            origin: Point::origin(),
            ..ClipboardData::default()
        };

        assert!(!schematic.paste_at(Point::new(20, 20)));
        assert!(!schematic.session.is_dirty);
        assert_eq!(schematic.topology_version(), topology_before);
        assert!(!schematic.can_undo());
        assert!(schematic.session.selection.has_component(component_id));
        assert!(schematic.design.document().buses.is_empty());
        assert!(schematic.design.document().bus_taps.is_empty());
    }

    #[test]
    fn tap_only_selection_copies_required_source_not_external_target() {
        let declaration = BusDeclaration::parse("DATA[3:0]").unwrap();
        let bus = Bus::segment(60, Point::new(0, 0), Point::new(20, 0), Some(declaration)).unwrap();
        let tap = BusTap::new(
            61,
            &bus,
            Point::new(10, 0),
            Point::new(10, 10),
            BusSlice::parse("DATA[1]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        let mut schematic = SchematicState::default();
        schematic.design.document_mut_for_test().buses.push(bus);
        schematic.design.document_mut_for_test().bus_taps.push(tap);
        schematic
            .design
            .document_mut_for_test()
            .wires
            .push(Wire::segment(62, Point::new(0, 10), Point::new(20, 10)));
        schematic.session.selection.select_only_bus_tap(61);

        schematic.copy_selection();

        assert_eq!(schematic.session.clipboard.buses.len(), 1);
        assert_eq!(schematic.session.clipboard.bus_taps.len(), 1);
        assert!(
            schematic.session.clipboard.wires.is_empty(),
            "an external target is a selection boundary, not an implicit copy"
        );
        assert!(schematic.paste_at(Point::new(100, 100)));
        assert_eq!(schematic.design.document().buses.len(), 2);
        assert_eq!(schematic.design.document().bus_taps.len(), 2);
        assert_eq!(schematic.design.document().wires.len(), 1);
    }

    #[test]
    fn label_copy_paste_preserves_name_offsets_position_and_remaps_identity() {
        let original = NetLabel::new(70, Point::new(10, 20), "sense_out");
        let mut schematic = SchematicState::default();
        schematic
            .design
            .document_mut_for_test()
            .net_labels
            .push(original.clone());
        schematic.recalculate_runtime_state();
        schematic
            .session
            .selection
            .select_only_net_label(original.id);
        schematic.copy_selection();
        schematic.clear_undo_history();

        assert_eq!(
            schematic.session.clipboard.net_labels,
            vec![original.clone()]
        );
        assert_eq!(schematic.session.clipboard.origin, original.pos);
        assert!(schematic.paste_at(Point::new(110, 220)));

        assert_eq!(schematic.design.document().net_labels.len(), 2);
        let pasted = schematic
            .design
            .document()
            .net_labels
            .iter()
            .find(|label| label.id != original.id)
            .unwrap();
        assert_eq!(pasted.name, original.name);
        assert_eq!(pasted.pos, Point::new(110, 220));
        assert!(schematic.session.selection.has_net_label(pasted.id));
        assert_eq!(
            schematic.session.selection.single_net_label(),
            Some(pasted.id)
        );
        assert_eq!(schematic.undo_description(), Some("paste"));

        assert!(schematic.undo());
        assert_eq!(schematic.design.document().net_labels, vec![original]);
        assert!(!schematic.can_undo(), "paste must create one undo step");
        assert!(schematic.redo());
        assert_eq!(schematic.design.document().net_labels.len(), 2);
    }

    #[test]
    fn label_cut_pattern_updates_clipboard_and_deletes_in_one_undo_step() {
        let label = NetLabel::new(80, Point::new(-5, 15), "cut_me");
        let mut schematic = SchematicState::default();
        schematic
            .design
            .document_mut_for_test()
            .net_labels
            .push(label.clone());
        schematic.session.selection.select_only_net_label(label.id);
        schematic.init_undo_history();

        schematic.copy_selection();
        assert!(schematic.delete_selection());

        assert_eq!(schematic.session.clipboard.net_labels, vec![label.clone()]);
        assert!(schematic.design.document().net_labels.is_empty());
        assert_eq!(schematic.undo_description(), Some("delete selection"));
        assert!(schematic.undo());
        assert_eq!(schematic.design.document().net_labels, vec![label]);
        assert!(!schematic.can_undo(), "cut must create one undo step");
    }

    #[test]
    fn label_duplicate_pattern_creates_fresh_identity_in_one_undo_step() {
        let label = NetLabel::new(90, Point::new(3, 7), "duplicated_net");
        let mut schematic = SchematicState::default();
        schematic
            .design
            .document_mut_for_test()
            .net_labels
            .push(label.clone());
        schematic.recalculate_runtime_state();
        schematic.session.selection.select_only_net_label(label.id);
        schematic.init_undo_history();

        schematic.copy_selection();
        assert!(schematic.paste_at(Point::new(13, 17)));

        assert_eq!(schematic.design.document().net_labels.len(), 2);
        let duplicate = schematic
            .design
            .document()
            .net_labels
            .iter()
            .find(|candidate| candidate.id != label.id)
            .unwrap();
        assert_eq!(duplicate.pos, Point::new(13, 17));
        assert_eq!(duplicate.name, label.name);
        assert_ne!(duplicate.id, label.id);
        assert!(schematic.undo());
        assert_eq!(schematic.design.document().net_labels, vec![label]);
        assert!(!schematic.can_undo(), "duplicate must create one undo step");
    }

    #[test]
    fn label_only_paste_is_not_constrained_by_junction_candidates() {
        let label = NetLabel::new(100, Point::new(0, 0), "floating_name");
        let mut schematic = SchematicState::default();
        schematic.session.clipboard = ClipboardData::from_selection_with_labels_and_buses(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec![label],
            Vec::new(),
            Vec::new(),
        );

        assert!(schematic.paste_at(Point::new(123, 456)));
        assert_eq!(
            schematic.design.document().net_labels[0].pos,
            Point::new(123, 456)
        );
    }

    #[test]
    fn documentation_shape_copy_paste_translates_exactly_with_fresh_identity_and_one_undo() {
        let original = DocumentationShape::new(
            110,
            DocumentationShapeGeometry::Rectangle {
                first: Point::new(10, 20),
                opposite: Point::new(30, 40),
            },
        )
        .unwrap();
        let mut schematic = SchematicState::default();
        schematic
            .design
            .document_mut_for_test()
            .documentation_shapes
            .push(original.clone());
        schematic.recalculate_runtime_state();
        schematic
            .session
            .selection
            .select_only_documentation_shape(original.id);
        schematic.init_undo_history();
        let topology = schematic.topology_version();

        schematic.copy_selection();

        assert_eq!(
            schematic.session.clipboard.documentation_shapes,
            vec![original.clone()]
        );
        assert_eq!(schematic.session.clipboard.origin, Point::new(20, 30));
        assert!(schematic.paste_at(Point::new(100, 120)));
        assert_eq!(schematic.design.document().documentation_shapes.len(), 2);
        let pasted = schematic
            .design
            .document()
            .documentation_shapes
            .last()
            .unwrap();
        assert_ne!(pasted.id, original.id);
        assert_eq!(pasted.layer, original.layer);
        assert_eq!(
            pasted.geometry,
            DocumentationShapeGeometry::Rectangle {
                first: Point::new(90, 110),
                opposite: Point::new(110, 130),
            }
        );
        assert_eq!(
            schematic.session.selection.single_documentation_shape(),
            Some(pasted.id)
        );
        assert_eq!(schematic.topology_version(), topology);
        assert_eq!(schematic.undo_description(), Some("paste"));

        assert!(schematic.undo());
        assert_eq!(
            schematic.design.document().documentation_shapes,
            vec![original]
        );
        assert_eq!(schematic.topology_version(), topology);
        assert!(
            !schematic.can_undo(),
            "paste must create exactly one undo step"
        );
    }

    #[test]
    fn bound_probe_copy_paste_preserves_expression_without_changing_topology() {
        let original =
            SchematicProbe::new(120, Point::new(10, 20), "V(out)", Some("V(out)".to_owned()))
                .unwrap();
        let mut schematic = SchematicState::default();
        schematic
            .design
            .document_mut_for_test()
            .probes
            .push(original.clone());
        schematic.recalculate_runtime_state();
        schematic.session.selection.select_only_probe(original.id);
        schematic.copy_selection();
        schematic.clear_undo_history();
        let topology = schematic.topology_version();

        assert_eq!(schematic.session.clipboard.probes, vec![original.clone()]);
        assert_eq!(schematic.session.clipboard.origin, original.position);
        assert!(schematic.paste_at(Point::new(110, 220)));

        assert_eq!(schematic.design.document().probes.len(), 2);
        let pasted = schematic
            .design
            .document()
            .probes
            .iter()
            .find(|probe| probe.id != original.id)
            .unwrap();
        assert_eq!(pasted.position, Point::new(110, 220));
        assert_eq!(pasted.reference, original.reference);
        assert_eq!(pasted.source_expression, original.source_expression);
        assert_eq!(schematic.session.selection.single_probe(), Some(pasted.id));
        assert_eq!(schematic.topology_version(), topology);
        assert_eq!(schematic.undo_description(), Some("paste"));

        assert!(schematic.undo());
        assert_eq!(schematic.design.document().probes, vec![original]);
        assert_eq!(schematic.topology_version(), topology);
        assert!(!schematic.can_undo());
    }
}
