#[cfg(test)]
use super::super::super::{BusDeclaration, BusSlice, BusTapOrientation};
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
        if self.selection.is_empty() {
            return;
        }

        let selected_comps: Vec<Component> = self
            .components
            .iter()
            .filter(|c| self.selection.has_component(c.id))
            .cloned()
            .collect();

        // Get all terminal positions for selected components
        let selected_terminals: Vec<Point> = selected_comps
            .iter()
            .flat_map(|c| c.terminal_positions().into_iter().map(|(_, pos)| pos))
            .collect();

        // Find wires that have both endpoints at selected component terminals
        let mut wires_to_copy: Vec<Wire> = Vec::new();

        for wire in &self.wires {
            // Check if explicitly selected
            if self.selection.has_wire(wire.id) {
                if wire.points.len() >= 2 {
                    wires_to_copy.push(wire.clone());
                }
                continue;
            }

            // Check if both endpoints connect to selected components
            if wire.points.len() >= 2 {
                let start = wire.points[0];
                let end = *wire.points.last().unwrap();

                let start_connected = selected_terminals.contains(&start);
                let end_connected = selected_terminals.contains(&end);

                if start_connected && end_connected {
                    wires_to_copy.push(wire.clone());
                }
            }
        }

        // Junction dots that sit on a copied wire travel with the selection;
        // a pasted multi-way joint must keep its explicit connection dots.
        let mut junctions_to_copy: Vec<Point> = self
            .junctions
            .iter()
            .map(|j| j.pos)
            .filter(|pos| {
                self.selection.has_junction(*pos)
                    || wires_to_copy.iter().any(|wire| wire.contains_point(*pos))
            })
            .collect();
        junctions_to_copy.sort_by_key(|point| (point.x, point.y));
        junctions_to_copy.dedup();

        let explicitly_selected_bus_ids = self.selection.buses.clone();
        let mut bus_ids_to_copy = explicitly_selected_bus_ids.clone();
        bus_ids_to_copy.extend(
            self.bus_taps
                .iter()
                .filter(|tap| self.selection.has_bus_tap(tap.id))
                .map(|tap| tap.bus_id),
        );
        let buses_to_copy: Vec<Bus> = self
            .buses
            .iter()
            .filter(|bus| bus_ids_to_copy.contains(&bus.id))
            .cloned()
            .collect();
        let bus_taps_to_copy: Vec<BusTap> = self
            .bus_taps
            .iter()
            .filter(|tap| {
                self.selection.has_bus_tap(tap.id)
                    || explicitly_selected_bus_ids.contains(&tap.bus_id)
            })
            .cloned()
            .collect();
        let net_labels_to_copy: Vec<NetLabel> = self
            .net_labels
            .iter()
            .filter(|label| self.selection.has_net_label(label.id))
            .cloned()
            .collect();

        self.clipboard = ClipboardData::from_selection_with_labels_and_buses(
            selected_comps,
            wires_to_copy,
            junctions_to_copy,
            net_labels_to_copy,
            buses_to_copy,
            bus_taps_to_copy,
        );
    }

    /// Check if clipboard has content
    pub fn can_paste(&self) -> bool {
        self.clipboard.has_content()
    }

    /// Paste clipboard contents at the given position (one undo entry)
    pub fn paste_at(&mut self, pos: Point) -> bool {
        if self.read_only || !self.can_paste() {
            return false;
        }

        let junction_only = self.clipboard.components.is_empty()
            && self.clipboard.wires.is_empty()
            && self.clipboard.buses.is_empty()
            && self.clipboard.bus_taps.is_empty()
            && self.clipboard.net_labels.is_empty();
        // A junction-only clipboard is a connectivity edit, not decoration.
        // Snap its anchor through the same ambiguous-crossing candidate set as
        // the junction tool, then reject it before opening an undo transaction
        // unless at least one translated marker would create a new connection.
        let paste_pos = if junction_only {
            let Some(candidate) = self.nearest_junction_candidate(pos, self.grid_size) else {
                return false;
            };
            candidate
        } else {
            pos
        };
        if junction_only {
            let offset_x = paste_pos.x.saturating_sub(self.clipboard.origin.x);
            let offset_y = paste_pos.y.saturating_sub(self.clipboard.origin.y);
            let has_valid_target = self.clipboard.junctions.iter().any(|junction| {
                let target = Point::new(
                    junction.x.saturating_add(offset_x),
                    junction.y.saturating_add(offset_y),
                );
                !self.has_junction(target)
                    && self.nearest_junction_candidate(target, 0) == Some(target)
            });
            if !has_valid_target {
                return false;
            }
        }

        self.with_undo("paste", |s| {
            let clipboard_components = s.clipboard.components.clone();
            let clipboard_wires: Vec<Wire> = s
                .clipboard
                .wires
                .iter()
                .filter(|wire| wire.points.len() >= 2)
                .cloned()
                .collect();
            let clipboard_junctions = s.clipboard.junctions.clone();
            let clipboard_net_labels = s.clipboard.net_labels.clone();
            let clipboard_buses = s.clipboard.buses.clone();
            let clipboard_bus_taps = s.clipboard.bus_taps.clone();
            let origin = s.clipboard.origin;

            if clipboard_components.is_empty()
                && clipboard_wires.is_empty()
                && clipboard_junctions.is_empty()
                && clipboard_net_labels.is_empty()
                && clipboard_buses.is_empty()
                && clipboard_bus_taps.is_empty()
            {
                return;
            }

            let offset_x = paste_pos.x.saturating_sub(origin.x);
            let offset_y = paste_pos.y.saturating_sub(origin.y);

            let mut committed = false;

            // Paste components with new IDs
            for comp in clipboard_components {
                if !committed {
                    s.selection.clear();
                    committed = true;
                }
                let new_id = s.next_id();
                let mut new_comp = comp;
                new_comp.id = new_id;
                new_comp.pos.x = new_comp.pos.x.saturating_add(offset_x);
                new_comp.pos.y = new_comp.pos.y.saturating_add(offset_y);
                new_comp.name = s.generate_name(new_comp.kind);
                s.components.push(new_comp);
                s.selection.select_component(new_id);
            }

            // Paste wires with new IDs
            for wire in clipboard_wires {
                if !committed {
                    s.selection.clear();
                    committed = true;
                }
                let new_id = s.next_id();
                let new_points: Vec<Point> = wire
                    .points
                    .iter()
                    .map(|p| Point::new(p.x.saturating_add(offset_x), p.y.saturating_add(offset_y)))
                    .collect();
                s.wires.push(Wire::new(new_id, new_points));
                s.selection.select_wire(new_id);
            }

            // Labels retain their user-facing net names while receiving new
            // document identities and translated attachment anchors.
            for mut label in clipboard_net_labels {
                if !committed {
                    s.selection.clear();
                    committed = true;
                }
                let new_id = s.next_id();
                label.id = new_id;
                label.pos = Point::new(
                    label.pos.x.saturating_add(offset_x),
                    label.pos.y.saturating_add(offset_y),
                );
                s.net_labels.push(label);
                s.selection.select_net_label(new_id);
            }

            // Paste buses before taps so every source reference can be
            // remapped to a fresh stable document identity.
            let mut bus_id_map = std::collections::HashMap::new();
            for mut bus in clipboard_buses {
                let old_id = bus.id;
                bus.translate(Point::new(offset_x, offset_y));
                if bus.validate().is_err() {
                    continue;
                }
                if !committed {
                    s.selection.clear();
                    committed = true;
                }
                let new_id = s.next_id();
                bus.id = new_id;
                s.buses.push(bus);
                bus_id_map.entry(old_id).or_insert(new_id);
                s.selection.select_bus(new_id);
            }

            for mut tap in clipboard_bus_taps {
                let Some(&new_bus_id) = bus_id_map.get(&tap.bus_id) else {
                    continue;
                };
                tap.bus_id = new_bus_id;
                tap.translate(Point::new(offset_x, offset_y));
                let Some(source) = s.buses.iter().find(|bus| bus.id == new_bus_id) else {
                    continue;
                };
                if tap.validate_against_bus(source).is_ok() {
                    if !committed {
                        s.selection.clear();
                        committed = true;
                    }
                    tap.id = s.next_id();
                    let id = tap.id;
                    s.bus_taps.push(tap);
                    s.selection.select_bus_tap(id);
                }
            }

            // Re-create junction dots only where at least two distinct wires
            // meet. This makes junction-only copy/paste useful without ever
            // manufacturing an electrically meaningless floating marker.
            for junction in clipboard_junctions {
                let target = Point::new(
                    junction.x.saturating_add(offset_x),
                    junction.y.saturating_add(offset_y),
                );
                let valid_target = if junction_only {
                    s.nearest_junction_candidate(target, 0) == Some(target)
                } else {
                    s.wires
                        .iter()
                        .filter(|wire| wire.contains_point(target))
                        .map(|wire| wire.id)
                        .collect::<std::collections::HashSet<_>>()
                        .len()
                        >= 2
                };
                if valid_target && !s.has_junction(target) {
                    if !committed {
                        s.selection.clear();
                        committed = true;
                    }
                    s.add_junction(target);
                    s.selection.select_junction(target);
                }
            }

            if committed {
                s.is_dirty = true;
                s.bump_topology_version();
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clipboard_drops_malformed_wires_from_corrupt_import_state() {
        let mut schematic = SchematicState::default();
        schematic.wires.push(Wire::new(10, Vec::new()));
        schematic.wires.push(Wire::new(11, vec![Point::new(5, 5)]));
        schematic.selection.select_wire(10);
        schematic.selection.select_wire(11);
        let original_wire_count = schematic.wires.len();

        schematic.copy_selection();

        assert!(
            schematic.clipboard.wires.is_empty(),
            "malformed wires must not propagate into clipboard state"
        );
        schematic.paste_at(Point::new(20, 20));
        assert_eq!(
            schematic.wires.len(),
            original_wire_count,
            "paste must not create additional invalid wires"
        );
    }

    #[test]
    fn junction_only_clipboard_pastes_only_on_a_valid_intersection() {
        let source = Point::new(20, 20);
        let target = Point::new(80, 80);
        let mut schematic = SchematicState::default();
        schematic.wires = vec![
            Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
            Wire::new(2, vec![Point::new(20, 0), Point::new(20, 40)]),
            Wire::new(3, vec![Point::new(60, 80), Point::new(100, 80)]),
            Wire::new(4, vec![Point::new(80, 60), Point::new(80, 100)]),
        ];
        schematic.add_junction(source);
        schematic.selection.select_only_junction(source);
        schematic.copy_selection();

        assert!(schematic.can_paste());
        assert_eq!(schematic.clipboard.origin, source);
        assert!(schematic.paste_at(Point::new(target.x + 1, target.y - 1)));

        assert!(schematic.has_junction(target));
        assert!(schematic.selection.has_junction(target));
        assert!(schematic.can_undo());
    }

    #[test]
    fn junction_only_paste_rejects_empty_space_without_an_undo_step() {
        let source = Point::new(20, 20);
        let mut schematic = SchematicState::default();
        schematic.wires = vec![
            Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
            Wire::new(2, vec![Point::new(20, 0), Point::new(20, 40)]),
        ];
        schematic.add_junction(source);
        schematic.selection.select_only_junction(source);
        schematic.copy_selection();

        assert!(!schematic.paste_at(Point::new(200, 200)));
        assert!(!schematic.can_undo());
        assert_eq!(schematic.junctions.len(), 1);
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
        schematic.buses.push(bus);
        schematic.bus_taps.push(tap);
        schematic.recalculate_runtime_state();
        let original_bus_id = schematic.buses[0].id;
        schematic.selection.select_only_bus(original_bus_id);
        schematic.copy_selection();
        let clipboard_origin = schematic.clipboard.origin;
        schematic.clear_undo_history();

        assert!(schematic.paste_at(Point::new(100, 100)));
        assert_eq!(schematic.buses.len(), 2);
        assert_eq!(schematic.bus_taps.len(), 2);
        let pasted_bus_id = schematic
            .buses
            .iter()
            .find(|bus| bus.id != original_bus_id)
            .unwrap()
            .id;
        assert!(schematic.bus_taps.iter().any(|tap| {
            tap.bus_id == pasted_bus_id
                && tap.connection_point
                    == Point::new(10 + 100 - clipboard_origin.x, 5 + 100 - clipboard_origin.y)
        }));
        assert!(schematic.undo());
        assert_eq!(schematic.buses.len(), 1);
        assert_eq!(schematic.bus_taps.len(), 1);
    }

    #[test]
    fn fully_rejected_typed_payload_is_a_true_no_op() {
        let mut schematic = SchematicState::default();
        let component_id = schematic.add_component(ComponentType::Resistor, Point::origin());
        schematic.selection.select_only_component(component_id);
        schematic.is_dirty = false;
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
        schematic.clipboard = ClipboardData {
            buses: vec![invalid_bus],
            bus_taps: vec![orphan_tap],
            junctions: vec![Point::new(100, 100)],
            origin: Point::origin(),
            ..ClipboardData::default()
        };

        assert!(!schematic.paste_at(Point::new(20, 20)));
        assert!(!schematic.is_dirty);
        assert_eq!(schematic.topology_version(), topology_before);
        assert!(!schematic.can_undo());
        assert!(schematic.selection.has_component(component_id));
        assert!(schematic.buses.is_empty());
        assert!(schematic.bus_taps.is_empty());
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
        schematic.buses.push(bus);
        schematic.bus_taps.push(tap);
        schematic
            .wires
            .push(Wire::segment(62, Point::new(0, 10), Point::new(20, 10)));
        schematic.selection.select_only_bus_tap(61);

        schematic.copy_selection();

        assert_eq!(schematic.clipboard.buses.len(), 1);
        assert_eq!(schematic.clipboard.bus_taps.len(), 1);
        assert!(
            schematic.clipboard.wires.is_empty(),
            "an external target is a selection boundary, not an implicit copy"
        );
        assert!(schematic.paste_at(Point::new(100, 100)));
        assert_eq!(schematic.buses.len(), 2);
        assert_eq!(schematic.bus_taps.len(), 2);
        assert_eq!(schematic.wires.len(), 1);
    }

    #[test]
    fn label_copy_paste_preserves_name_offsets_position_and_remaps_identity() {
        let original = NetLabel::new(70, Point::new(10, 20), "sense_out");
        let mut schematic = SchematicState::default();
        schematic.net_labels.push(original.clone());
        schematic.recalculate_runtime_state();
        schematic.selection.select_only_net_label(original.id);
        schematic.copy_selection();
        schematic.clear_undo_history();

        assert_eq!(schematic.clipboard.net_labels, vec![original.clone()]);
        assert_eq!(schematic.clipboard.origin, original.pos);
        assert!(schematic.paste_at(Point::new(110, 220)));

        assert_eq!(schematic.net_labels.len(), 2);
        let pasted = schematic
            .net_labels
            .iter()
            .find(|label| label.id != original.id)
            .unwrap();
        assert_eq!(pasted.name, original.name);
        assert_eq!(pasted.pos, Point::new(110, 220));
        assert!(schematic.selection.has_net_label(pasted.id));
        assert_eq!(schematic.selection.single_net_label(), Some(pasted.id));
        assert_eq!(schematic.undo_description(), Some("paste"));

        assert!(schematic.undo());
        assert_eq!(schematic.net_labels, vec![original]);
        assert!(!schematic.can_undo(), "paste must create one undo step");
        assert!(schematic.redo());
        assert_eq!(schematic.net_labels.len(), 2);
    }

    #[test]
    fn label_cut_pattern_updates_clipboard_and_deletes_in_one_undo_step() {
        let label = NetLabel::new(80, Point::new(-5, 15), "cut_me");
        let mut schematic = SchematicState::default();
        schematic.net_labels.push(label.clone());
        schematic.selection.select_only_net_label(label.id);
        schematic.init_undo_history();

        schematic.copy_selection();
        assert!(schematic.delete_selection());

        assert_eq!(schematic.clipboard.net_labels, vec![label.clone()]);
        assert!(schematic.net_labels.is_empty());
        assert_eq!(schematic.undo_description(), Some("delete selection"));
        assert!(schematic.undo());
        assert_eq!(schematic.net_labels, vec![label]);
        assert!(!schematic.can_undo(), "cut must create one undo step");
    }

    #[test]
    fn label_duplicate_pattern_creates_fresh_identity_in_one_undo_step() {
        let label = NetLabel::new(90, Point::new(3, 7), "duplicated_net");
        let mut schematic = SchematicState::default();
        schematic.net_labels.push(label.clone());
        schematic.recalculate_runtime_state();
        schematic.selection.select_only_net_label(label.id);
        schematic.init_undo_history();

        schematic.copy_selection();
        assert!(schematic.paste_at(Point::new(13, 17)));

        assert_eq!(schematic.net_labels.len(), 2);
        let duplicate = schematic
            .net_labels
            .iter()
            .find(|candidate| candidate.id != label.id)
            .unwrap();
        assert_eq!(duplicate.pos, Point::new(13, 17));
        assert_eq!(duplicate.name, label.name);
        assert_ne!(duplicate.id, label.id);
        assert!(schematic.undo());
        assert_eq!(schematic.net_labels, vec![label]);
        assert!(!schematic.can_undo(), "duplicate must create one undo step");
    }

    #[test]
    fn label_only_paste_is_not_constrained_by_junction_candidates() {
        let label = NetLabel::new(100, Point::new(0, 0), "floating_name");
        let mut schematic = SchematicState::default();
        schematic.clipboard = ClipboardData::from_selection_with_labels_and_buses(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec![label],
            Vec::new(),
            Vec::new(),
        );

        assert!(schematic.paste_at(Point::new(123, 456)));
        assert_eq!(schematic.net_labels[0].pos, Point::new(123, 456));
    }
}
