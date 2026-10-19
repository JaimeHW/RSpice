//! The work a part drag does on each frame of a large sheet, counted.
//!
//! The frame-cost probe ([`super::frame_cost`]) times the same drag against
//! its budgets, by hand and in release, because a debug build's clock says
//! nothing about a frame. This gate runs in every test run instead. It drives
//! the drag through whole canvas frames with real pointer events and counts,
//! through [`rspice_design_model::work_tally`], what each frame copies,
//! extracts, projects, cleans up and looks up, and how that work grows when
//! the sheet doubles. Each count is a cost a large-sheet drag once paid on
//! every frame it moved: the undo copy on the first move, connectivity over
//! every sheet, the projection rebuilt, a catalog lookup per object, loops
//! over pairs of objects.

use rspice_design::projection::{materialization_count, reset_materialization_count};
use rspice_design_model::work_tally::{self, WorkTally};

use super::AppState;
use super::frame_cost::{COLUMNS, ROWS, scale_fixture_of};
use crate::state::{Point, SchematicAnnotationVisibility};

const SCREEN: egui::Vec2 = egui::vec2(1600.0, 1000.0);
/// Move frames per drag; each one moves the part a grid step further.
const MOVES: usize = 4;

/// What one frame did.
#[derive(Debug)]
struct FrameWork {
    tally: WorkTally,
    /// Cell views materialized: whole-document projections.
    projections: u64,
    /// The objects the drawing holds after the frame, and how many of them
    /// are on the open sheet.
    document_objects: usize,
    open_sheet_objects: usize,
}

/// The press, each move and the drop of one part drag.
#[derive(Debug)]
struct DragWork {
    press: FrameWork,
    moves: Vec<FrameWork>,
    drop: FrameWork,
}

impl DragWork {
    /// Every frame of the drag, named, in order.
    fn frames(&self) -> impl Iterator<Item = (String, &FrameWork)> {
        std::iter::once(("the press".to_owned(), &self.press))
            .chain(
                self.moves
                    .iter()
                    .enumerate()
                    .map(|(index, frame)| (format!("move {}", index + 1), frame)),
            )
            .chain(std::iter::once(("the drop".to_owned(), &self.drop)))
    }
}

fn drawing_objects(state: &AppState) -> usize {
    let document = state.schematic.document();
    document.components.len()
        + document.wires.len()
        + document.buses.len()
        + document.bus_taps.len()
        + document.junctions.len()
        + document.net_labels.len()
}

/// Drag a part in the middle of the open sheet of a two-sheet cell view,
/// each sheet `columns` by `rows` parts, at 100 %: the press, [`MOVES`]
/// moves of one grid step each, and the drop.
fn drag_work(columns: usize, rows: usize) -> DragWork {
    let mut fixture = scale_fixture_of(columns, rows);
    fixture.state.ui.schematic_visibility.annotations = SchematicAnnotationVisibility::Hidden;
    // The drag never reaches the other sheet, so it keeps what it was built
    // with.
    let other_sheet = fixture.objects[1];
    let part = fixture.components[0][columns * (rows / 2) + columns / 2];
    let mut state = fixture.state;
    let at = state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == part)
        .map(|component| component.pos)
        .expect("the part is on the sheet");
    let grid = state.schematic.document().grid_size;
    let editor = &mut state.schematic.session.editor;
    editor.needs_fit = false;
    editor.zoom = 1.0;
    editor.pan = (
        f64::from(SCREEN.x) * 0.5 - f64::from(at.x),
        f64::from(SCREEN.y) * 0.5 - f64::from(at.y),
    );
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let symbols = super::super::symbols::SymbolLibrary::load_embedded().ok();
    let mut clock = 0.0;
    let mut frame = |state: &mut AppState, event: egui::Event| {
        clock += 1.0 / 60.0;
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN)),
            events: vec![event],
            time: Some(clock),
            ..Default::default()
        };
        let _ = work_tally::take();
        reset_materialization_count();
        let _ = ctx.run_ui(input, |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    super::render_schematic_view(ui, state, symbols.as_ref())
                });
        });
        let tally = work_tally::take();
        let projections = materialization_count();
        let document_objects = drawing_objects(state);
        FrameWork {
            tally,
            projections,
            document_objects,
            open_sheet_objects: document_objects - other_sheet,
        }
    };
    let button = |pos, pressed| egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };

    let mut pointer = egui::pos2(SCREEN.x * 0.5, SCREEN.y * 0.5);
    for _ in 0..2 {
        frame(&mut state, egui::Event::PointerMoved(pointer));
    }
    let press = frame(&mut state, button(pointer, true));
    let moves = (0..MOVES)
        .map(|_| {
            pointer.x += grid as f32;
            frame(&mut state, egui::Event::PointerMoved(pointer))
        })
        .collect();
    let drop = frame(&mut state, button(pointer, false));

    let steps = i32::try_from(MOVES).expect("a few moves");
    let dropped = state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == part)
        .map(|component| component.pos);
    assert_eq!(
        dropped,
        Some(Point::new(at.x + steps * grid, at.y)),
        "the part follows the pointer and drops where it was released"
    );
    assert!(
        !state.schematic_drag_in_progress(),
        "the drop ends the drag"
    );
    DragWork { press, moves, drop }
}

/// Hold each frame of a drag on `sheet` to the work that frame needs.
fn hold_each_frame_to_its_work(sheet: &str, drag: &DragWork) {
    let DragWork { press, moves, drop } = drag;

    // Undo copies the document twice a drag: at the press, as the baseline
    // the drag takes when it starts, and at the drop, as the step's
    // after-state. A move copies nothing.
    assert_eq!(
        press.tally.document_snapshots, 1,
        "{sheet}: the press copies the undo baseline once: {press:?}"
    );
    assert_eq!(
        drop.tally.document_snapshots, 1,
        "{sheet}: the drop copies the undo step's after-state once: {drop:?}"
    );
    for (index, frame) in moves.iter().enumerate() {
        let tally = &frame.tally;
        let index = index + 1;
        assert_eq!(
            tally.document_snapshots, 0,
            "{sheet}: move {index} copies no document: {frame:?}"
        );
        assert_eq!(
            (frame.projections, tally.sheet_reconciliations),
            (0, 0),
            "{sheet}: move {index} neither projects the cell view nor reconciles its sheets: \
             {frame:?}"
        );
        assert_eq!(
            (tally.collinear_merges, tally.junction_updates),
            (0, 0),
            "{sheet}: move {index} leaves the wire cleanup to the drop: {frame:?}"
        );
    }
    assert!(
        drop.projections <= 1 && drop.tally.sheet_reconciliations <= 1,
        "{sheet}: the drop projects the cell view and reconciles its sheets at most once \
         each: {drop:?}"
    );
    assert_eq!(
        (drop.tally.collinear_merges, drop.tally.junction_updates),
        (1, 1),
        "{sheet}: the drop joins collinear wires and updates the junctions once: {drop:?}"
    );

    // The open pins are judged at most once a frame, on the open sheet's
    // objects alone, never on the other sheet's as well.
    for (name, frame) in drag.frames() {
        let tally = &frame.tally;
        assert!(
            tally.connectivity_passes <= 1,
            "{sheet}: {name} extracts connectivity at most once: {frame:?}"
        );
        assert_eq!(
            tally.connectivity_objects,
            tally.connectivity_passes * frame.open_sheet_objects,
            "{sheet}: {name} extracts connectivity from the open sheet's {} objects, not the \
             drawing's {}: {frame:?}",
            frame.open_sheet_objects,
            frame.document_objects
        );
    }
    assert!(
        moves
            .iter()
            .any(|frame| frame.tally.connectivity_passes == 1),
        "{sheet}: the moves re-judge the open pins, or this gate checks nothing: {moves:?}"
    );
}

/// Hold the work that grows with the sheet to the sheet's own growth.
/// Doubling the sheet may multiply it by 2.5 at most, where a pass over each
/// object grows twice and a loop over pairs of them four times; and a lookup
/// by key must not grow at all.
fn hold_growth_to_the_sheets(small: &DragWork, large: &DragWork) {
    let (few, many) = (
        small.press.open_sheet_objects,
        large.press.open_sheet_objects,
    );
    assert!(
        10 * many >= 19 * few,
        "the large sheet holds twice the small one's objects: {many} against {few}"
    );
    let in_step = |what: &str, small: usize, large: usize| {
        assert!(small > 0, "the small sheet's drag counted no {what}");
        assert!(
            2 * large <= 5 * small,
            "{what} grew from {small} to {large} when the sheet doubled, more than 2.5 times"
        );
    };
    let busiest_move = |drag: &DragWork| {
        drag.moves
            .iter()
            .map(|frame| frame.tally.connectivity_steps)
            .max()
            .unwrap_or_default()
    };
    in_step(
        "connectivity steps on a move frame",
        busiest_move(small),
        busiest_move(large),
    );
    in_step(
        "connectivity steps on the drop",
        small.drop.tally.connectivity_steps,
        large.drop.tally.connectivity_steps,
    );
    in_step(
        "wires sorted onto lines by the drop's collinear merge",
        small.drop.tally.collinear_wires_sorted,
        large.drop.tally.collinear_wires_sorted,
    );
    for ((name, small), (_, large)) in small.frames().zip(large.frames()) {
        assert_eq!(
            large.tally.sheet_catalog_lookups, small.tally.sheet_catalog_lookups,
            "{name} looks the sheet catalog up as often on a sheet twice the size, not once \
             per object"
        );
    }
}

/// A part drag on the scale fixture's open sheet does each frame's work once
/// and on that sheet alone, and the work grows in step with the sheet.
#[test]
fn a_part_drag_on_a_large_sheet_does_only_the_work_each_frame_needs() {
    // Half the scale fixture's parts on each sheet, then the scale fixture.
    let small = drag_work(28, 18);
    let large = drag_work(COLUMNS, ROWS);
    hold_each_frame_to_its_work("the half-size sheet", &small);
    hold_each_frame_to_its_work("the scale fixture", &large);
    hold_growth_to_the_sheets(&small, &large);
}
