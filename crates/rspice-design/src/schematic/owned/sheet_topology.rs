//! Temporary sheet isolation used by editor operations.
//!
//! Every sheet of a cell view lives in one document and shares its
//! coordinates. An edit that finds objects by where they stand -- a tap whose
//! end lies on a moved wire, a label anchoring a stretched segment, a pin a
//! wire end touches -- finds another sheet's as readily as its own. So the
//! scope takes every object of the other sheets out of the document, of every
//! kind, and puts each back where it stood: no edit inside can see them.
use super::super::{
    bus::{Bus, BusTap},
    component::Component,
    design_note::DesignNote,
    documentation_shape::DocumentationShape,
    net_label::{Junction, NetLabel},
    probe::SchematicProbe,
    wire::{Wire, WireConnection},
};
use super::Schematic;

/// The other sheets' objects, each with the place it held in its list.
pub struct HiddenWireTopology {
    wires: Vec<(usize, Wire)>,
    junctions: Vec<(usize, Junction)>,
    /// Parts taken out with their sheet's wires: their pins decide dots.
    components: Vec<(usize, Component)>,
    buses: Vec<(usize, Bus)>,
    /// Taps are found by where their ends lie, on any wire or bus there.
    bus_taps: Vec<(usize, BusTap)>,
    /// Labels anchor the segments they stand on and keep wires apart.
    net_labels: Vec<(usize, NetLabel)>,
    probes: Vec<(usize, SchematicProbe)>,
    design_notes: Vec<(usize, DesignNote)>,
    documentation_shapes: Vec<(usize, DocumentationShape)>,
    /// A pin record that names a hidden wire or a hidden part: a wire end on
    /// another sheet's pin must not hold this sheet's wire in place.
    connections: Vec<(usize, WireConnection)>,
}

impl HiddenWireTopology {
    pub fn restore(self, design: &mut Schematic) {
        let document = &mut design.document;
        restore_hidden(&mut document.wires, self.wires);
        restore_hidden(&mut document.junctions, self.junctions);
        restore_hidden(&mut document.components, self.components);
        restore_hidden(&mut document.buses, self.buses);
        restore_hidden(&mut document.bus_taps, self.bus_taps);
        restore_hidden(&mut document.net_labels, self.net_labels);
        restore_hidden(&mut document.probes, self.probes);
        restore_hidden(&mut document.design_notes, self.design_notes);
        restore_hidden(
            &mut document.documentation_shapes,
            self.documentation_shapes,
        );
        restore_hidden(&mut document.connections, self.connections);
    }
}

impl Schematic {
    /// Take every object `hidden` names out of the document, of every kind,
    /// until the returned value is restored.
    pub fn take_hidden_wire_topology(
        &mut self,
        hidden: impl Fn(u64) -> bool,
    ) -> HiddenWireTopology {
        let document = &mut self.document;
        HiddenWireTopology {
            wires: take_hidden(&mut document.wires, |wire| hidden(wire.id)),
            junctions: take_hidden(&mut document.junctions, |junction| hidden(junction.id)),
            components: take_hidden(&mut document.components, |component| hidden(component.id)),
            buses: take_hidden(&mut document.buses, |bus| hidden(bus.id)),
            bus_taps: take_hidden(&mut document.bus_taps, |tap| hidden(tap.id)),
            net_labels: take_hidden(&mut document.net_labels, |label| hidden(label.id)),
            probes: take_hidden(&mut document.probes, |probe| hidden(probe.id)),
            design_notes: take_hidden(&mut document.design_notes, |note| hidden(note.id)),
            documentation_shapes: take_hidden(&mut document.documentation_shapes, |shape| {
                hidden(shape.id)
            }),
            connections: take_hidden(&mut document.connections, |connection| {
                hidden(connection.wire_id) || hidden(connection.component_id)
            }),
        }
    }
}

/// Take the items `hidden` names out of `items`, each with the place it held.
///
/// Each taken item moves once, into the returned list. The kept items stay in
/// the list's own storage, closing up behind each taken one, so those ahead
/// of the first taken one never move: a part is close to a kilobyte, and a
/// drag takes the other sheets out on every frame.
fn take_hidden<T>(items: &mut Vec<T>, hidden: impl Fn(&T) -> bool) -> Vec<(usize, T)> {
    // The filter sees each item in order, just before the item it takes is
    // handed over, so the place it last took is that item's.
    let next = std::cell::Cell::new(0);
    let taken_at = std::cell::Cell::new(0);
    let mut removed = Vec::new();
    for item in items.extract_if(.., |item| {
        let place = next.get();
        next.set(place + 1);
        let take = hidden(item);
        if take {
            taken_at.set(place);
        }
        take
    }) {
        removed.push((taken_at.get(), item));
    }
    removed
}

fn restore_hidden<T>(items: &mut Vec<T>, hidden: Vec<(usize, T)>) {
    for (index, item) in hidden {
        items.insert(index.min(items.len()), item);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::super::super::{
        bus::{Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation},
        component::Component,
        document::SchematicDocument,
        movement::{DragRubberBand, MoveSelectionMode, MovementSelection},
        net_label::NetLabel,
        stretch::{StretchOrthogonalPolicy, StretchTarget},
        wire::Wire,
    };
    use super::Schematic;
    use rspice_design_model::Point;

    /// The ids Sheet 2 owns; everything else is on Sheet 1, which is open.
    const SHEET_TWO: [u64; 4] = [2, 3, 4, 5];

    fn on_sheet_two(id: u64) -> bool {
        SHEET_TWO.contains(&id)
    }

    fn no_pins(_: &Component) -> Vec<Point> {
        Vec::new()
    }

    /// Both sheets run a wire from (100, 100) to (240, 100). Sheet 2 also
    /// draws a bus above its wire and taps it onto the wire at (160, 100);
    /// with `labelled`, it names its wire there as well.
    fn two_sheets(labelled: bool) -> Schematic {
        let p = Point::new;
        let declaration = BusDeclaration::parse("DATA[7:0]").expect("declaration");
        let bus = Bus::new(3, vec![p(100, 40), p(240, 40)], Some(declaration)).expect("bus");
        let slice = BusSlice::parse("DATA[3]").expect("slice");
        let tap = BusTap::new(
            4,
            &bus,
            p(160, 40),
            p(160, 100),
            slice,
            BusTapOrientation::Down,
        )
        .expect("tap");
        Schematic::from_document(SchematicDocument {
            wires: vec![
                Wire::new(1, vec![p(100, 100), p(240, 100)]),
                Wire::new(2, vec![p(100, 100), p(240, 100)]),
            ],
            buses: vec![bus],
            bus_taps: vec![tap],
            net_labels: if labelled {
                vec![NetLabel::new(5, p(160, 100), "SHEET2")]
            } else {
                Vec::new()
            },
            ..SchematicDocument::default()
        })
    }

    #[derive(Debug, Clone, Copy)]
    enum Edit {
        Move,
        Drag,
        Stretch,
    }

    /// `edit` Sheet 1's wire down by 40 with Sheet 2 out of reach, as the
    /// editor's sheet scope runs it, and whether the edit was made.
    fn edit_sheet_one_wire(schematic: &mut Schematic, edit: Edit) -> bool {
        let empty = HashSet::new();
        let wire = HashSet::from([1]);
        let selection = MovementSelection {
            components: &empty,
            wires: &wire,
            buses: &empty,
            bus_taps: &empty,
            net_labels: &empty,
            design_notes: &empty,
            documentation_shapes: &empty,
            probes: &empty,
        };
        let down = Point::new(0, 40);
        schematic.begin_operation("edit Sheet 1");
        let hidden = schematic.take_hidden_wire_topology(on_sheet_two);
        let made = match edit {
            Edit::Move => schematic
                .move_selection_with_mode_resolved(
                    selection,
                    down,
                    MoveSelectionMode::Connected,
                    no_pins,
                )
                .is_ok_and(|impact| impact.changed),
            Edit::Drag => {
                schematic
                    .drag_selection_with_rubber_band_resolved(
                        selection,
                        down,
                        DragRubberBand::Orthogonal,
                        no_pins,
                    )
                    .changed
            }
            Edit::Stretch => schematic
                .stretch_target_resolved(
                    down,
                    StretchTarget::WireSegment {
                        wire_id: 1,
                        segment_index: 0,
                    },
                    StretchOrthogonalPolicy::PreserveOrthogonal,
                    no_pins,
                    |_| (0, 0, 0, 0),
                )
                .unwrap_or(false),
        };
        hidden.restore(schematic);
        schematic.end_operation();
        made
    }

    /// Sheet 2's tap ends at (160, 100), on Sheet 2's wire and on the same
    /// point of Sheet 1's. Moving, dragging or stretching Sheet 1's wire
    /// found taps by where they end and carried Sheet 2's along with it.
    #[test]
    fn another_sheets_tap_stays_put_while_this_sheets_wire_moves() {
        let mut wrong = Vec::new();
        for edit in [Edit::Move, Edit::Drag, Edit::Stretch] {
            let mut schematic = two_sheets(false);
            let before = schematic.document().clone();
            let made = edit_sheet_one_wire(&mut schematic, edit);
            let after = schematic.document();
            if !made || after.wires[0].points != [Point::new(100, 140), Point::new(240, 140)] {
                wrong.push(format!(
                    "{edit:?} left Sheet 1's wire at {:?}",
                    after.wires[0]
                ));
            }
            if after.wires[1] != before.wires[1] || after.buses != before.buses {
                wrong.push(format!("{edit:?} moved Sheet 2's wire or bus"));
            }
            if after.bus_taps != before.bus_taps {
                wrong.push(format!(
                    "{edit:?} on Sheet 1 carried Sheet 2's tap to {:?}",
                    after.bus_taps[0].connection_point
                ));
            }
        }
        assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    }

    /// A label Sheet 2 puts on its own wire at (160, 100) anchored Sheet 1's
    /// wire through the same point, and the stretch was refused.
    #[test]
    fn another_sheets_label_never_anchors_this_sheets_wire() {
        let mut schematic = two_sheets(true);
        let labels = schematic.document().net_labels.clone();
        assert!(
            edit_sheet_one_wire(&mut schematic, Edit::Stretch),
            "Sheet 2's label held Sheet 1's wire"
        );
        assert_eq!(schematic.document().net_labels, labels);
    }

    /// Every hidden object goes back to the place it held in its list.
    #[test]
    fn restoring_puts_every_object_back_where_it_stood() {
        let mut schematic = two_sheets(true);
        let before = schematic.document().clone();
        let hidden = schematic.take_hidden_wire_topology(on_sheet_two);
        let document = schematic.document();
        assert_eq!(document.wires.len(), 1);
        assert!(document.buses.is_empty());
        assert!(document.bus_taps.is_empty());
        assert!(document.net_labels.is_empty());
        hidden.restore(&mut schematic);
        let after = schematic.document();
        assert_eq!(after.wires, before.wires);
        assert_eq!(after.buses, before.buses);
        assert_eq!(after.bus_taps, before.bus_taps);
        assert_eq!(after.net_labels, before.net_labels);
    }
}
