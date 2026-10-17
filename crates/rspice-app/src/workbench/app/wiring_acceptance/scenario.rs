//! One scripted session, step by step: every wiring edit a user makes in the
//! first hour with the editor, then undo to the empty start and redo to the
//! end. After each step the deck's nets and the painted dots are exactly
//! what the step should leave.

use super::{Bench, Dots, Nets, dots, nets};
use crate::schematic::view::selection_transform::SelectionTransform;
use crate::state::{ComponentType, MoveSelectionMode, Point, Rotation};

fn p(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

/// The id of the wire with a vertex at `point` on the active document.
fn wire_at(bench: &Bench, point: Point) -> u64 {
    bench
        .state()
        .schematic
        .document()
        .wires
        .iter()
        .find(|wire| wire.points.contains(&point))
        .unwrap_or_else(|| panic!("no wire has a vertex at {point:?}\n{}", bench.describe()))
        .id
}

fn part(bench: &Bench, name: &str) -> u64 {
    bench
        .state()
        .schematic
        .document()
        .components
        .iter()
        .find(|part| part.name == name)
        .unwrap_or_else(|| panic!("no part {name}\n{}", bench.describe()))
        .id
}

struct Expected {
    step: &'static str,
    nets: Nets,
    dots: Dots,
}

fn check(bench: &Bench, expected: &Expected, phase: &str) {
    assert_eq!(
        bench.nets(),
        expected.nets,
        "{phase} `{}`: the deck's nets\n{}",
        expected.step,
        bench.describe()
    );
    assert_eq!(
        bench.dots(),
        expected.dots,
        "{phase} `{}`: the painted dots\n{}",
        expected.step,
        bench.describe()
    );
}

/// The parts the session starts with on Sheet 1. Resistors at R0 have their
/// `+` pin 20 left of the body and `-` 20 right; at R90, `+` is 20 above.
fn starting_parts() -> Vec<(&'static str, ComponentType, Point, Rotation)> {
    let resistor = ComponentType::Resistor;
    vec![
        ("R1", resistor, p(100, 100), Rotation::R0),
        ("R2", resistor, p(300, 100), Rotation::R0),
        ("R3", resistor, p(200, 200), Rotation::R90),
        ("R4", resistor, p(100, 140), Rotation::R0),
        ("R5", resistor, p(300, 140), Rotation::R0),
        ("R6", resistor, p(500, 100), Rotation::R0),
        ("R7", resistor, p(580, 100), Rotation::R0),
        ("R8", resistor, p(700, 100), Rotation::R0),
    ]
}

/// Copy an upright resistor with a wire falling 40 from its lower pin, out
/// of cell `parts`, and come back to `top`.
fn copy_from_parts_cell(bench: &mut Bench) {
    bench.open_cell("parts");
    let resistor = bench.place(ComponentType::Resistor, p(0, 0), Rotation::R90);
    bench.draw_wire(&[p(0, 20), p(0, 60)]);
    let wire = wire_at(bench, p(0, 60));
    bench.select(|selection| {
        selection.select_component(resistor);
        selection.select_wire(wire);
    });
    bench.copy_selection();
    bench.open_cell("top");
}

#[test]
fn a_wiring_session_keeps_the_deck_and_the_dots_right_through_undo_and_redo() {
    let mut bench = Bench::new(&starting_parts());
    // The group the session turns, already wired: R6 to R7, and R7 on to R8.
    bench.draw_wire(&[p(520, 100), p(560, 100)]);
    bench.draw_wire(&[p(600, 100), p(680, 100)]);
    copy_from_parts_cell(&mut bench);
    bench.state_mut().schematic.clear_undo_history();
    assert_eq!(bench.active_sheet(), 0);

    let mut expected = vec![Expected {
        step: "start",
        nets: nets(&[&["R6.-", "R7.+"], &["R7.-", "R8.+"]]),
        dots: dots(&[]),
    }];
    let mut done = |bench: &Bench,
                    step: &'static str,
                    net_list: &[&[&str]],
                    dot_list: &[(usize, i32, i32)]| {
        let next = Expected {
            step,
            nets: nets(net_list),
            dots: dots(dot_list),
        };
        check(bench, &next, "after");
        expected.push(next);
    };

    // A wire from R1 to R2.
    bench.draw_wire(&[p(120, 100), p(280, 100)]);
    done(
        &bench,
        "draw a wire",
        &[&["R1.-", "R2.+"], &["R6.-", "R7.+"], &["R7.-", "R8.+"]],
        &[],
    );

    // A stem from the middle of that wire down to R3 makes a T.
    bench.draw_wire(&[p(200, 100), p(200, 180)]);
    done(
        &bench,
        "make a T",
        &[
            &["R1.-", "R2.+", "R3.+"],
            &["R6.-", "R7.+"],
            &["R7.-", "R8.+"],
        ],
        &[(0, 200, 100)],
    );

    // R4 to R5 crosses the stem: a crossing joins nothing.
    bench.draw_wire(&[p(120, 140), p(280, 140)]);
    done(
        &bench,
        "cross the stem",
        &[
            &["R1.-", "R2.+", "R3.+"],
            &["R4.-", "R5.+"],
            &["R6.-", "R7.+"],
            &["R7.-", "R8.+"],
        ],
        &[(0, 200, 100)],
    );

    // A junction placed on the crossing joins the two.
    bench.click_junction(p(200, 140));
    done(
        &bench,
        "mark the crossing",
        &[
            &["R1.-", "R2.+", "R3.+", "R4.-", "R5.+"],
            &["R6.-", "R7.+"],
            &["R7.-", "R8.+"],
        ],
        &[(0, 200, 100), (0, 200, 140)],
    );

    // R9 dropped upright with its lower pin inside the R1-R2 wire.
    bench.place(ComponentType::Resistor, p(250, 80), Rotation::R90);
    done(
        &bench,
        "place a part on a wire",
        &[
            &["R1.-", "R2.+", "R3.+", "R4.-", "R5.+", "R9.-"],
            &["R6.-", "R7.+"],
            &["R7.-", "R8.+"],
        ],
        &[(0, 200, 100), (0, 200, 140), (0, 250, 100)],
    );

    // R1 moves up; its wire follows.
    let r1 = part(&bench, "R1");
    bench.select(|selection| selection.select_component(r1));
    bench.move_selection(p(0, -40), MoveSelectionMode::Connected);
    done(
        &bench,
        "move",
        &[
            &["R1.-", "R2.+", "R3.+", "R4.-", "R5.+", "R9.-"],
            &["R6.-", "R7.+"],
            &["R7.-", "R8.+"],
        ],
        &[(0, 200, 100), (0, 200, 140), (0, 250, 100)],
    );

    // R6, R7 and the wire between them turn as one body; R7's wire on to R8
    // is not selected and follows.
    let (r6, r7) = (part(&bench, "R6"), part(&bench, "R7"));
    let between = wire_at(&bench, p(520, 100));
    bench.select(|selection| {
        selection.select_component(r6);
        selection.select_component(r7);
        selection.select_wire(between);
    });
    bench.turn_selection(SelectionTransform::RotateClockwise);
    done(
        &bench,
        "rotate a group",
        &[
            &["R1.-", "R2.+", "R3.+", "R4.-", "R5.+", "R9.-"],
            &["R6.-", "R7.+"],
            &["R7.-", "R8.+"],
        ],
        &[(0, 200, 100), (0, 200, 140), (0, 250, 100)],
    );

    // The turn followed R7's lower pin round a corner at (600, 100): the
    // wire's upright middle segment stretched 20 right takes its two corners
    // with it, and both ends stay where they are.
    let followed = wire_at(&bench, p(600, 100));
    bench.select(|selection| selection.select_only_wire_segment(followed, 1));
    bench.stretch_selection(p(20, 0));
    assert!(
        bench
            .state()
            .schematic
            .document()
            .wires
            .iter()
            .any(|wire| wire.id == followed && wire.points.contains(&p(620, 100))),
        "the segment did not stretch\n{}",
        bench.describe()
    );
    done(
        &bench,
        "stretch",
        &[
            &["R1.-", "R2.+", "R3.+", "R4.-", "R5.+", "R9.-"],
            &["R6.-", "R7.+"],
            &["R7.-", "R8.+"],
        ],
        &[(0, 200, 100), (0, 200, 140), (0, 250, 100)],
    );

    // The part copied in the other cell floats with its middle on the
    // pointer and lands upright at (230, 40), its wire's end inside the R1-R2
    // wire: a T, joined and dotted. It takes this cell's next free name.
    bench.paste_at(p(230, 50));
    assert!(
        bench
            .state()
            .schematic
            .document()
            .components
            .iter()
            .any(|part| part.name == "R10" && part.pos == p(230, 40)),
        "the copy did not land at (230, 40)\n{}",
        bench.describe()
    );
    done(
        &bench,
        "paste across cells",
        &[
            &["R1.-", "R10.-", "R2.+", "R3.+", "R4.-", "R5.+", "R9.-"],
            &["R6.-", "R7.+"],
            &["R7.-", "R8.+"],
        ],
        &[(0, 200, 100), (0, 200, 140), (0, 230, 100), (0, 250, 100)],
    );

    // Sheet 2 draws R11 to R12 over the very coordinates of Sheet 1's T,
    // its paste and its part on the wire. They are another page: nothing on
    // Sheet 1 splits, joins or gains a dot.
    bench.activate_sheet(1);
    done(
        &bench,
        "open the second sheet",
        &[
            &["R1.-", "R10.-", "R2.+", "R3.+", "R4.-", "R5.+", "R9.-"],
            &["R6.-", "R7.+"],
            &["R7.-", "R8.+"],
        ],
        &[(0, 200, 100), (0, 200, 140), (0, 230, 100), (0, 250, 100)],
    );
    bench.place(ComponentType::Resistor, p(100, 100), Rotation::R0);
    done(
        &bench,
        "place on the second sheet",
        &[
            &["R1.-", "R10.-", "R2.+", "R3.+", "R4.-", "R5.+", "R9.-"],
            &["R6.-", "R7.+"],
            &["R7.-", "R8.+"],
        ],
        &[(0, 200, 100), (0, 200, 140), (0, 230, 100), (0, 250, 100)],
    );
    bench.place(ComponentType::Resistor, p(300, 100), Rotation::R0);
    done(
        &bench,
        "place over a pin of the first sheet",
        &[
            &["R1.-", "R10.-", "R2.+", "R3.+", "R4.-", "R5.+", "R9.-"],
            &["R6.-", "R7.+"],
            &["R7.-", "R8.+"],
        ],
        &[(0, 200, 100), (0, 200, 140), (0, 230, 100), (0, 250, 100)],
    );
    bench.draw_wire(&[p(120, 100), p(280, 100)]);
    done(
        &bench,
        "wire the second sheet",
        &[
            &["R1.-", "R10.-", "R2.+", "R3.+", "R4.-", "R5.+", "R9.-"],
            &["R11.-", "R12.+"],
            &["R6.-", "R7.+"],
            &["R7.-", "R8.+"],
        ],
        &[(0, 200, 100), (0, 200, 140), (0, 230, 100), (0, 250, 100)],
    );

    // Undo every step back to the start, then redo every step to the end.
    for index in (0..expected.len() - 1).rev() {
        bench.undo();
        check(&bench, &expected[index], "undo to");
    }
    assert_eq!(bench.active_sheet(), 0, "undo returns to the first sheet");
    for next in &expected[1..] {
        bench.redo();
        check(&bench, next, "redo of");
    }
    assert_eq!(bench.active_sheet(), 1, "redo returns to the second sheet");
}
