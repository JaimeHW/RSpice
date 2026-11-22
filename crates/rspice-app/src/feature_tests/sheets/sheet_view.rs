//! What a sheet's canvas shows: where its first frame stands and where a fit
//! puts it, drawn through the schematic canvas.

#![cfg(not(target_arch = "wasm32"))]

use egui::{Rect, Vec2};

use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use rspice_app_state::workbench::workspace::Workspace;

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
