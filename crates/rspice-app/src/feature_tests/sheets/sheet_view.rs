//! What a sheet's canvas shows: where its first frame stands and where a fit
//! puts it, drawn through the schematic canvas.

#![cfg(not(target_arch = "wasm32"))]

use egui::{Rect, Vec2};
use rspice_app_design::workbench::design::commands::DesignCommand;
use rspice_app_sheets::workbench::app::actions::sheets::activate_sheet;

use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use rspice_app_sheets::workbench::app::drawing_sheet_edit::DrawingSheetEdit;
use rspice_app_state::state::{
    Component, ComponentType, DrawingSheetStandard, DrawingSheetTitleFieldId, Point,
    SchematicPageOrientation, SheetDefinition, SheetId, SheetPortPolicy,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::WorkbenchCommand;

const CANVAS: Vec2 = Vec2::new(1_200.0, 800.0);

/// Draw the schematic canvas for one frame at `time` seconds, filling a
/// [`CANVAS`]-sized window from its origin, and return every text it paints.
fn draw_canvas(state: &mut AppState, time: f64) -> Vec<String> {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, CANVAS)),
            time: Some(time),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    rspice_app_design::schematic::view::render_schematic_view(
                        ui,
                        &mut state.canvas_frame(),
                        None,
                    );
                });
        },
    );
    output
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
            _ => None,
        })
        .collect()
}

/// Where the open sheet's paper lands on the canvas.
fn paper_on_screen(state: &AppState) -> Rect {
    let (min_x, min_y, max_x, max_y) = state.model.active_drawing_sheet().geometry.paper.as_tuple();
    let editor = &state.model.schematic.session.editor;
    let at = |x: f64, y: f64| {
        egui::pos2(
            (editor.pan.0 + x * editor.zoom) as f32,
            (editor.pan.1 + y * editor.zoom) as f32,
        )
    };
    Rect::from_two_pos(at(min_x, min_y), at(max_x, max_y))
}

/// The whole paper is on the canvas and spans most of it one way: framed,
/// not shown at 100 % from a corner of the sheet.
fn assert_framed(state: &AppState, what: &str) {
    let paper = paper_on_screen(state);
    let canvas = Rect::from_min_size(egui::Pos2::ZERO, CANVAS);
    assert!(
        canvas.expand(0.5).contains_rect(paper),
        "{what}: the paper {paper:?} runs off the canvas"
    );
    assert!(
        paper.width() > 0.85 * CANVAS.x || paper.height() > 0.85 * CANVAS.y,
        "{what}: the paper {paper:?} is not framed"
    );
}

/// A new schematic opens framed on its sheet, as KiCad and Altium open one
/// fitted to the page, and an empty file that asks for a content fit is
/// framed on its sheet too, never reset to 100 %.
#[test]
fn an_empty_schematic_opens_framed_on_its_sheet() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    draw_canvas(&mut app.state, 0.0);
    assert_framed(&app.state, "a new schematic");

    let editor = &mut app.state.model.schematic.session.editor;
    (editor.zoom, editor.pan) = (3.0, (-500.0, -200.0));
    editor.needs_fit = true;
    draw_canvas(&mut app.state, 1.0);
    assert_framed(&app.state, "an empty file just opened");
}

/// A new size or orientation frames the new sheet: going from A4 to A1, the
/// new right and bottom part of the paper and the title block that moved to
/// its corner are on screen without a fit.
#[test]
fn a_new_sheet_size_frames_the_new_sheet() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    draw_canvas(&mut app.state, 0.0);
    assert_framed(&app.state, "the sheet as it opened");

    super::apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::Size(DrawingSheetStandard::IsoA1),
    )
    .expect("the size change publishes");
    draw_canvas(&mut app.state, 1.0);
    assert_framed(&app.state, "the sheet made A1");

    super::apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::Orientation(SchematicPageOrientation::Portrait),
    )
    .expect("the orientation change publishes");
    draw_canvas(&mut app.state, 2.0);
    assert_framed(&app.state, "the sheet turned upright");
}

/// One cell view with two sheets, Sheet 2 open: a part far out at
/// (3000, 2000) and one near the origin at (200, 150), each on Sheet 1 but
/// those `on_second` names.
fn two_sheet_app(on_second: &[u64]) -> (RSpiceApp, SheetId, SheetId) {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
    app.state.model.schematic.document_mut_for_test().components = vec![
        Component::new(10, ComponentType::Resistor, Point::new(3_000, 2_000)),
        Component::new(20, ComponentType::Resistor, Point::new(200, 150)),
    ];
    let key = app
        .state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let designs = &mut app.state.model.workspace.content.design_management;
    let first = designs
        .bootstrap_for_cell_view(&key, "Sheet 1", [10, 20])
        .expect("first sheet");
    let catalog = designs.sheet_catalog_mut(&key).expect("sheet catalog");
    let second = catalog
        .create_sheet(
            SheetDefinition {
                name: "Sheet 2".to_owned(),
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(first),
        )
        .expect("second sheet");
    if !on_second.is_empty() {
        catalog
            .assign_objects(catalog.revision(), second, on_second.iter().copied())
            .expect("second sheet assignment");
    }
    catalog.set_active(second).expect("active sheet");
    (app, first, second)
}

/// Where world point `(x, y)` lands on the canvas.
fn on_screen(state: &AppState, (x, y): (f64, f64)) -> egui::Pos2 {
    let editor = &state.model.schematic.session.editor;
    egui::pos2(
        (editor.pan.0 + x * editor.zoom) as f32,
        (editor.pan.1 + y * editor.zoom) as f32,
    )
}

/// Fit content frames the objects of the sheet in front, as KiCad's Zoom to
/// Objects does: each sheet is its own page, so Sheet 1's part far away
/// neither shrinks Sheet 2's part into a corner nor pulls the view off it.
#[test]
fn fit_content_frames_the_open_sheet_only() {
    let (mut app, _, _) = two_sheet_app(&[20]);
    draw_canvas(&mut app.state, 0.0);
    app.execute_command_key(DesignCommand::FitSchematicContent.key());
    draw_canvas(&mut app.state, 1.0);

    let canvas = Rect::from_min_size(egui::Pos2::ZERO, CANVAS);
    let near = on_screen(&app.state, (200.0, 150.0));
    assert!(
        near.distance(canvas.center()) < 0.3 * CANVAS.y,
        "Sheet 2's part {near:?} is framed in the middle of the canvas"
    );
    let far = on_screen(&app.state, (3_000.0, 2_000.0));
    assert!(
        !canvas.contains(far),
        "Sheet 1's part {far:?} plays no part in Sheet 2's fit"
    );
}

/// The open camera, as Zoom previous keeps it.
fn camera(state: &AppState) -> (f64, (f64, f64)) {
    let editor = &state.model.schematic.session.editor;
    (editor.zoom, editor.pan)
}

fn open_sheet(app: &mut RSpiceApp, sheet: SheetId) {
    let state = &mut app.state;
    activate_sheet(
        &mut state.model,
        &mut state.workbench,
        &mut *state.design,
        sheet,
    )
    .expect("activation publishes");
}

/// Zoom previous steps back through the views of the sheet in front. Right
/// after a switch it has nothing earlier to show on the new sheet, so it
/// never paints the old sheet's camera onto it; back on the first sheet it
/// returns the view that sheet was left at.
#[test]
fn zoom_previous_steps_back_through_its_own_sheet() {
    let (mut app, first, second) = two_sheet_app(&[20]);
    open_sheet(&mut app, first);
    draw_canvas(&mut app.state, 0.0);
    let left_at = (2.0, (-300.0, -100.0));
    let editor = &mut app.state.model.schematic.session.editor;
    (editor.zoom, editor.pan) = left_at;
    draw_canvas(&mut app.state, 2.0);

    open_sheet(&mut app, second);
    draw_canvas(&mut app.state, 4.0);
    let framed = camera(&app.state);
    app.execute_command_key(WorkbenchCommand::ZoomPrevious.key());
    draw_canvas(&mut app.state, 6.0);
    assert_eq!(camera(&app.state), framed, "Sheet 2 has no earlier view");

    open_sheet(&mut app, first);
    draw_canvas(&mut app.state, 8.0);
    app.execute_command_key(WorkbenchCommand::ZoomPrevious.key());
    draw_canvas(&mut app.state, 10.0);
    assert_eq!(
        camera(&app.state),
        left_at,
        "Sheet 1 goes back to where it was left"
    );
}

/// The first-use hint is for an empty drawing: a design's blank next sheet
/// shows only its paper, while a new drawing with nothing on it says what to
/// do first.
#[test]
fn a_blank_second_sheet_shows_no_first_use_hint() {
    let (mut app, _, _) = two_sheet_app(&[]);
    let texts = draw_canvas(&mut app.state, 0.0);
    assert!(
        !texts.iter().any(|text| text == "Empty schematic"),
        "{texts:?}"
    );

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    let texts = draw_canvas(&mut app.state, 0.0);
    assert!(
        texts.iter().any(|text| text == "Empty schematic"),
        "{texts:?}"
    );
}

/// A double-click on the title block opens the sheet in the inspector with
/// the cursor in its Title, as KiCad's double-click on the block opens its
/// page settings: what is typed next is the sheet's Title.
#[test]
fn a_double_click_on_the_title_block_puts_the_cursor_in_its_title() {
    use crate::feature_tests::replay::{Replay, only};
    use egui::accesskit::Role;

    let mut replay = Replay::new();
    let controls = replay.frames(Vec::new());
    let canvas = only(&controls, "Schematic canvas", Role::Canvas).center;
    let sheet = replay.app.state.model.active_drawing_sheet();
    let (min_x, min_y, max_x, max_y) = sheet.geometry.paper.as_tuple();
    let (block_min_x, block_min_y, block_max_x, block_max_y) = sheet
        .geometry
        .title_block
        .expect("the sheet has a title block")
        .as_tuple();
    // A new schematic opens framed on its sheet, the paper's centre on the
    // canvas's.
    let zoom = replay.app.state.model.schematic.session.editor.zoom;
    let at = canvas
        + egui::vec2(
            (((block_min_x + block_max_x) - (min_x + max_x)) / 2.0 * zoom) as f32,
            (((block_min_y + block_max_y) - (min_y + max_y)) / 2.0 * zoom) as f32,
        );
    let press = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    replay.frame(vec![egui::Event::PointerMoved(at)]);
    for pressed in [true, false, true, false] {
        replay.frame(vec![press(pressed)]);
    }
    replay.frames(Vec::new());
    replay.type_text("Loop control\n");

    let sheet = replay.app.state.model.active_drawing_sheet();
    assert_eq!(
        sheet.format.title_block.fields[&DrawingSheetTitleFieldId::SheetTitle].value,
        "Loop control"
    );
}
