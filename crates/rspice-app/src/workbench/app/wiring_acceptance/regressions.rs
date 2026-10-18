//! Defects the acceptance suite found, each pinned at its shortest
//! reproduction.

use super::Bench;
use super::property::{Drawing, Pick, Step::*, Turn, p, replay, replay_on};
use crate::state::{ComponentType, MoveSelectionMode, Point, Rotation};
use rspice_schematic_editor::view::pointer_target::PointerTarget;

/// Replay a shrunk sequence the property test reported, from the wiring
/// fixture it was found on.
fn holds(steps: &[super::property::Step]) {
    if let Err(failure) = replay(steps) {
        panic!("{failure}");
    }
}

/// Replay a shrunk sequence the property test reported from the annotated
/// fixture.
fn holds_annotated(steps: &[super::property::Step]) {
    if let Err(failure) = replay_on(Drawing::Annotated, steps) {
        panic!("{failure}");
    }
}

/// Both fixture sheets draw a T at (200, 100). Deleting the T's right
/// branch on Sheet 1 leaves two wire ends there, which no dot marks; the
/// deletion's own dot cleanup counted Sheet 2's T at the same point as well
/// and kept Sheet 1's dot.
#[test]
fn deleting_a_t_branch_drops_its_dot_though_another_sheet_has_a_t_there() {
    holds(&[Delete(Pick::Wire(8))]);
}

/// R3 hangs upright under the T's stem. Turned, flipped and moved, then
/// dragged up and right, its top pin came to rest on the stem's line short
/// of the stem's lower end, and the stem ran down past the pin and back up
/// to it: [(240, 140), (200, 140), (200, 220), (200, 200)].
#[test]
fn a_dragged_part_never_leaves_its_wire_doubled_back() {
    holds(&[
        Transform(
            vec![Pick::Part(4), Pick::Wire(8), Pick::Part(2)],
            Turn::Clockwise,
        ),
        Transform(vec![Pick::Part(3), Pick::Part(4)], Turn::MirrorHorizontal),
        Move(
            vec![Pick::Wire(8), Pick::Part(3), Pick::Part(1)],
            p(-20, 0),
            false,
        ),
        Drag(3, vec![p(60, -20)]),
    ]);
}

/// The L right of R2 is two wires: a run from R2's pin to the corner and a
/// leg down from it. Flipping the leg top to bottom twice puts it back
/// where it was drawn; the run followed both flips, and after the second
/// it ran down to (360, 160) and back up over itself:
/// [(320, 100), (360, 100), (360, 160), (360, 100)].
#[test]
fn a_wire_flipped_back_leaves_its_neighbour_square() {
    holds(&[
        Transform(vec![Pick::Wire(12)], Turn::MirrorVertical),
        Transform(vec![Pick::Wire(12)], Turn::MirrorVertical),
    ]);
}

/// A wire drawn from R2's pin on the T's right branch to the crossing
/// wire, then R1 mirrored top to bottom with two wires: the mirror moved a
/// dot onto a point that already had one, leaving two dots on (140, 100).
#[test]
fn a_mirrored_selection_leaves_one_dot_per_point() {
    holds(&[
        Sheet,
        Sheet,
        Draw(vec![p(280, 100), p(140, 180)]),
        Transform(
            vec![Pick::Part(1), Pick::Wire(38), Pick::Wire(29)],
            Turn::MirrorVertical,
        ),
    ]);
}

/// A resistor dropped with its `-` pin on R2's `+` pin, where the T's
/// right branch ends: a pin, a pin and a wire end meet there, and the dot
/// was never placed because no wire had to split.
#[test]
fn a_part_dropped_onto_a_pin_and_a_wire_end_gets_its_dot() {
    holds(&[
        Draw(vec![p(240, 20), p(240, 140)]),
        Place(p(240, 100), Rotation::R0),
    ]);
}

/// R1 flipped down onto R4 brought wire 29 up onto the run of wire 27.
/// Flipping the crossing wire with R3 and R4 then moved both ends of wire
/// 27, which was squared again before wire 29 followed, and it ran out to
/// (120, 100) and back over wire 29:
/// [(200, 180), (200, 100), (120, 100), (180, 100)].
#[test]
fn a_followed_wire_never_doubles_back_over_a_wire_beside_it() {
    holds(&[
        Sheet,
        Sheet,
        Draw(vec![p(280, 100), p(140, 180)]),
        Draw(vec![p(280, 180), p(240, 100)]),
        Transform(
            vec![Pick::Part(1), Pick::Wire(38), Pick::Wire(29)],
            Turn::MirrorVertical,
        ),
        Transform(
            vec![Pick::Wire(35), Pick::Part(3), Pick::Part(4)],
            Turn::MirrorHorizontal,
        ),
    ]);
}

/// R4 flipped left to right put a pin on the crossing wire where the T's
/// stem crosses it, a crossing no dot is added to. Deleting the stem left
/// the pin on the wire with nothing crossing there, which earns a dot, and
/// the deletion placed none: it only takes dots away.
#[test]
fn deleting_a_wire_places_the_dot_it_makes_due() {
    holds(&[
        Draw(vec![p(220, 100), p(180, 100)]),
        Transform(vec![Pick::Part(4), Pick::Wire(25)], Turn::MirrorHorizontal),
        Delete(Pick::Wire(6)),
    ]);
}

/// A wire drawn along Sheet 2's top wire where wires already run adds
/// nothing, but the merge laid one of those wires again at another place
/// in the drawing order: a step nobody can see, which Undo then took back
/// instead of the flip before it.
#[test]
fn a_route_laid_over_existing_wires_adds_no_step() {
    holds(&[
        Sheet,
        Transform(
            vec![Pick::Part(15), Pick::Wire(23), Pick::Part(14)],
            Turn::MirrorHorizontal,
        ),
        Draw(vec![p(120, 100), p(240, 100)]),
        Undo,
    ]);
}

/// A wire drawn from R4's right pin out over R4 holds that pin at one of
/// its corners. R4 dragged down and right carried the corner with it, and
/// the run beyond was squared the way it was drawn, straight back from the
/// pin over R4: [(200, 140), (120, 140), (120, 260), (140, 260), (100, 260)].
#[test]
fn a_dragged_pin_at_a_wire_corner_never_doubles_the_wire_back() {
    holds(&[
        Transform(
            vec![Pick::Wire(8), Pick::Part(3), Pick::Wire(7)],
            Turn::Clockwise,
        ),
        Draw(vec![p(120, 180), p(40, 180)]),
        Drag(3, vec![p(40, 20), p(80, -40)]),
        Drag(4, vec![p(40, 60), p(80, 100), p(20, 80)]),
    ]);
}

/// On Sheet 2, the wire drawn from R2's pin back over R1 turns down at
/// R1's left pin. Dragging R1 up carried that corner up the wire's own
/// downward run, which was squared out to the corner and straight back
/// down: [(140, 40), (140, 100), (80, 100), (80, 80), (80, 180)].
#[test]
fn a_pin_dragged_onto_its_own_wire_never_doubles_it_back() {
    holds(&[
        Sheet,
        Draw(vec![p(280, 100), p(80, 180)]),
        Transform(vec![Pick::Wire(19), Pick::Wire(25)], Turn::Clockwise),
        Drag(13, vec![p(60, 40), p(0, -20)]),
    ]);
}

/// A resistor dropped across the T's left branch, then R1 and the new
/// resistor dragged in turn, left a wire from the pins they share out to
/// the new resistor's other pin and straight back to where it began:
/// [(160, 120), (200, 120), (160, 120)].
#[test]
fn a_wire_never_runs_out_and_back_to_where_it_began() {
    holds(&[
        Place(p(160, 100), Rotation::R180),
        Drag(1, vec![p(40, 20)]),
        Drag(25, vec![p(60, -40), p(40, 20)]),
    ]);
}

/// The T's right branch turned, R2 and R4 moved, then R3 flipped top to
/// bottom with a wire: the stem, which ends on R3's pin and starts at the
/// T, was laid up to the T and straight back down to the pin, and the T
/// still needs the stretch it doubles over:
/// [(240, 60), (240, 260), (200, 260), (200, 100), (200, 140)].
#[test]
fn a_wire_doubled_back_to_a_t_is_laid_as_two_wires() {
    holds(&[
        Transform(vec![Pick::Wire(8)], Turn::Clockwise),
        Move(vec![Pick::Part(2), Pick::Part(4)], p(-40, 0), false),
        Transform(
            vec![Pick::Part(3), Pick::Wire(39), Pick::Part(2)],
            Turn::MirrorVertical,
        ),
    ]);
}

/// A junction-tool click on the dot where a wire tees into R3's moved
/// stem took the dot away, though the wires still meet there and automatic
/// junctions mark that point.
#[test]
fn a_junction_click_keeps_a_dot_the_wires_need() {
    holds(&[
        Draw(vec![p(200, 260), p(220, 120)]),
        Drag(3, vec![p(60, -60), p(80, -40)]),
        Move(vec![Pick::Wire(33), Pick::Wire(6)], p(20, 0), false),
        Junction(p(220, 180)),
    ]);
}

/// Undoing the switch to Sheet 2 and then moving a part on Sheet 1 is a
/// new edit after the undo: the switch can no longer be redone. Redo
/// replayed it anyway.
#[test]
fn an_edit_after_undo_leaves_nothing_to_redo_in_either_history() {
    holds(&[
        Sheet,
        Undo,
        Move(vec![Pick::Part(1)], p(40, 0), false),
        Redo,
    ]);
}

/// The other way round: opening Sheet 2 after undoing a move is a new step,
/// and the move can no longer be redone.
#[test]
fn a_sheet_switch_after_undo_leaves_the_undone_edit_unredoable() {
    holds(&[
        Move(vec![Pick::Part(1)], p(40, 0), false),
        Undo,
        Sheet,
        Redo,
    ]);
}

/// Both annotated sheets tap their bus onto the L's corner at (360, 100).
/// Moving the L's leg on Sheet 1 carried Sheet 2's tap along with it, to
/// (380, 60).
#[test]
fn moving_a_wire_leaves_another_sheets_tap_on_its_own_wire() {
    holds_annotated(&[Move(vec![Pick::Wire(12)], p(20, -40), false)]);
}

/// Sheet 2 draws what Sheet 1 does, a wire from (100, 100) to (240, 100),
/// and taps a bus onto it at (160, 100). Moving, dragging or stretching
/// Sheet 1's wire carried Sheet 2's tap along: those edits find a tap by
/// where it ends, and the sheet scope left every sheet's taps in reach.
#[test]
fn a_wire_edited_on_one_sheet_leaves_another_sheets_tap_where_it_was() {
    let mut wrong = Vec::new();
    for edit in ["move", "drag", "stretch"] {
        let mut bench = Bench::new(&[]);
        bench.draw_wire(&[p(100, 100), p(240, 100)]);
        bench.activate_sheet(1);
        bench.draw_wire(&[p(100, 100), p(240, 100)]);
        let bus = bench.draw_bus(&[p(100, 40), p(240, 40)], "DATA[7:0]");
        let tap = bench.place_tap(bus, p(160, 40), p(160, 100), "DATA[3]");
        assert_ne!(tap, 0, "no tap on Sheet 2\n{}", bench.describe());
        bench.activate_sheet(0);
        let document = bench.state().schematic.document();
        let wire = document
            .wires
            .iter()
            .map(|wire| wire.id)
            .find(|id| bench.sheet_of(*id) == 0)
            .expect("Sheet 1's wire");
        let taps = document.bus_taps.clone();
        match edit {
            "move" => {
                bench.select(|selection| selection.select_wire(wire));
                bench.move_selection(p(0, 40), MoveSelectionMode::Connected);
            }
            "drag" => {
                bench.select(|selection| selection.select_wire(wire));
                bench.drag(PointerTarget::Wire(wire), p(120, 100), &[p(0, 40)]);
            }
            _ => {
                bench.select(|selection| selection.select_only_wire_segment(wire, 0));
                bench.stretch_selection(p(0, 40));
            }
        }
        let document = bench.state().schematic.document();
        let moved = document.wires.iter().find(|candidate| candidate.id == wire);
        if moved.map(|wire| wire.points.as_slice()) != Some(&[p(100, 140), p(240, 140)][..]) {
            wrong.push(format!("the {edit} left Sheet 1's wire at {moved:?}"));
        }
        if document.bus_taps != taps {
            wrong.push(format!(
                "the {edit} on Sheet 1 carried Sheet 2's tap:\n{}",
                bench.describe()
            ));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// A dot copied alone pastes only where two of the open sheet's wires
/// meet, and not onto a dot that sheet already has. Sheet 2 draws a
/// crossing at (300, 100); pasted there on Sheet 1, which draws nothing
/// there, the dot was placed on Sheet 1 and only its cleanup took it away
/// again, and the paste said it had pasted it. With Sheet 1 drawing the
/// crossing too and Sheet 2 dotting it, the paste on Sheet 1 was refused:
/// the paste read every sheet's wires and dots.
#[test]
fn a_dot_pasted_alone_reads_only_the_open_sheets_wires_and_dots() {
    let mut wrong = Vec::new();
    for sheet_one_crosses_there in [false, true] {
        let mut bench = Bench::new(&[]);
        let cross = |bench: &mut Bench, x: i32| {
            bench.draw_wire(&[p(x - 40, 100), p(x + 40, 100)]);
            bench.draw_wire(&[p(x, 60), p(x, 140)]);
        };
        cross(&mut bench, 100);
        bench.click_junction(p(100, 100));
        if sheet_one_crosses_there {
            cross(&mut bench, 300);
        }
        bench.activate_sheet(1);
        cross(&mut bench, 300);
        if sheet_one_crosses_there {
            bench.click_junction(p(300, 100));
        }
        bench.activate_sheet(0);
        bench.select(|selection| selection.select_junction(p(100, 100)));
        bench.copy_selection();
        let mut expected = bench.dots();
        if sheet_one_crosses_there {
            expected.insert((0, 300, 100));
        }

        bench.paste_at(p(300, 100));

        let prompt = bench.state().prompt_text();
        if !sheet_one_crosses_there && prompt != Some("Nothing was pasted.") {
            wrong.push(format!(
                "with Sheet 1 empty at (300, 100), the paste said {prompt:?}"
            ));
        }
        if bench.dots() != expected {
            wrong.push(format!(
                "with Sheet 1 {} at (300, 100), the paste left dots {:?}, not {expected:?}\n{}",
                if sheet_one_crosses_there {
                    "crossing"
                } else {
                    "empty"
                },
                bench.dots(),
                bench.describe()
            ));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// Opening Sheet 2 is a step in the one undo order. A part placed there
/// gave the part its sheet, which moved the project's revision, and the
/// switch could never be undone after that: undo stopped with "the latest
/// project edit's document or references changed", stranding every step
/// before it.
#[test]
fn undo_steps_back_past_a_sheet_switch_once_the_new_sheet_is_edited() {
    let mut bench = Bench::new(&[]);
    bench.draw_wire(&[Point::new(0, 0), Point::new(100, 0)]);
    bench.activate_sheet(1);
    bench.place(ComponentType::Resistor, Point::new(100, 100), Rotation::R0);

    bench.undo();
    assert!(bench.state().schematic.document().components.is_empty());
    bench.undo();
    assert_eq!(
        bench.active_sheet(),
        0,
        "the switch was not undone: {:?}",
        bench.state().prompt_text()
    );
    bench.undo();
    assert!(
        bench.state().schematic.document().wires.is_empty(),
        "the wire drawn before the switch was not undone: {:?}",
        bench.state().prompt_text()
    );

    bench.redo();
    bench.redo();
    assert_eq!(bench.active_sheet(), 1, "{:?}", bench.state().prompt_text());
    bench.redo();
    let document = bench.state().schematic.document();
    assert_eq!(document.wires.len(), 1);
    assert_eq!(document.components.len(), 1);
    let part = document.components[0].id;
    let wire = document.wires[0].id;
    assert_eq!(bench.sheet_of(part), 1, "the part is back on Sheet 2");
    assert_eq!(bench.sheet_of(wire), 0, "the wire is back on Sheet 1");
}
