//! The canvas scene composed for a whole application.

use crate::state::Point;
use crate::state::{Component, ComponentType};
use crate::workbench::app_state::AppState;
use egui::Rect;

/// The centres of the open-pin squares one canvas frame paints, in
/// schematic units: the view is at 100 % with the origin at the canvas's
/// top-left corner.
fn open_pin_marks(ctx: &egui::Context, state: &mut AppState) -> Vec<Point> {
    let editor = &mut state.model.schematic.session.editor;
    (editor.needs_fit, editor.needs_drawing_sheet_fit) = (false, false);
    (editor.pan, editor.zoom) = ((0.0, 0.0), 1.0);
    let mut canvas = Rect::NOTHING;
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 600.0),
            )),
            ..Default::default()
        },
        |ui| {
            canvas = ui.available_rect_before_wrap();
            crate::schematic::view::render_schematic_view(
                ui,
                &mut crate::schematic::view::canvas_frame(state),
                None,
            );
        },
    );
    let mut marks = output
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Rect(square)
                if square.fill == egui::Color32::TRANSPARENT
                    && square.rect.width() < 10.0
                    && (square.rect.width() - square.rect.height()).abs() < 0.01 =>
            {
                let centre = square.rect.center() - canvas.min;
                Some(Point::new(centre.x.round() as i32, centre.y.round() as i32))
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    marks.sort_by_key(|point| (point.x, point.y));
    marks
}

/// An open pin is marked on the terminal it belongs to, on every sheet.
/// The marks used to be read off the whole cell's projection, which lays
/// the sheet at catalog index k a stride along x, so Sheet 2's marks were
/// drawn a stride away from its pins, off the canvas. Switching sheets
/// changes no topology, and the marks still follow the sheet.
#[test]
fn an_open_pin_on_sheet_two_is_marked_at_its_own_point() {
    let mut state = crate::schematic::view::sheet_visibility::two_sheet_state(&[6], &[7]);
    state.design.grid = crate::state::GridStyle::Off;
    let first = Component::new(6, ComponentType::Resistor, Point::new(100, 100));
    let second = Component::new(7, ComponentType::Capacitor, Point::new(300, 200));
    let terminals = |component: &Component| {
        let mut points = component
            .terminal_positions()
            .into_iter()
            .map(|(_, point)| point)
            .collect::<Vec<_>>();
        points.sort_by_key(|point| (point.x, point.y));
        points
    };
    let (first_pins, second_pins) = (terminals(&first), terminals(&second));
    let document = state.model.schematic.document_mut_for_test();
    document.components.extend([first, second]);
    state.model.schematic.bump_topology_version();
    state.sync_active_schematic_to_workspace();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);

    assert_eq!(open_pin_marks(&ctx, &mut state), first_pins, "Sheet 1");

    let key = state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let catalog = state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .expect("two sheets");
    let sheet_two = catalog.sheets()[1].id();
    catalog.set_active(sheet_two).expect("Sheet 2 opens");
    assert_eq!(
        open_pin_marks(&ctx, &mut state),
        second_pins,
        "Sheet 2's open pins are marked where the canvas draws them"
    );
}
