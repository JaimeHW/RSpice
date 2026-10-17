//! Defects the acceptance suite found, each pinned at its shortest
//! reproduction.

use super::Bench;
use super::property::{Pick, Step::*, p, replay};
use crate::state::{ComponentType, Point, Rotation};

/// Replay a shrunk sequence the property test reported, from its fixture.
fn holds(steps: &[super::property::Step]) {
    if let Err(failure) = replay(steps) {
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
