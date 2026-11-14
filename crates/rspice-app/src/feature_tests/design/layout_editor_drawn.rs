//! The layout editor drawn through a whole application.

use crate::state::CellViewRef;
use crate::workbench::RSpiceApp;
use crate::workbench::design::surface::layout_editor::*;
use egui::{Pos2, Rect, Vec2};

/// One frame of the layout editor over a whole application's canvas, at
/// `size`.
fn render(app: &mut RSpiceApp, size: Vec2) -> egui::FullOutput {
    let context = egui::Context::default();
    crate::ui::Theme::default().apply(&context);
    context.enable_accesskit();
    context.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
            ..Default::default()
        },
        |context| {
            egui::CentralPanel::default().show(context, |ui| {
                show(
                    ui,
                    &mut crate::schematic::view::canvas_frame(&mut app.state),
                );
            });
        },
    )
}

fn app_with_layout() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.provision_test_project_technology_contract();
    let owner = CellViewRef::new("user", "top", "layout");
    app.state
        .model
        .library_manager
        .get_library_mut("user")
        .expect("default project library")
        .get_or_create_cell("top")
        .add_view(crate::state::View::new(
            "layout",
            crate::state::ViewType::Layout,
        ));
    app.state
        .model
        .workspace
        .open_view(owner.clone(), crate::state::ViewType::Layout);
    app.state
        .model
        .initialize_physical_layout_document(owner)
        .expect("initialize governed physical layout");
    app
}

/// A pointer over the canvas puts its database-unit point where the
/// status bar reads its coordinate readout from, and leaving clears it.
#[test]
fn the_cursor_over_the_canvas_reaches_the_status_bar() {
    let mut app = app_with_layout();
    let context = egui::Context::default();
    crate::ui::Theme::default().apply(&context);
    // Hover is resolved against the previous frame's widgets, so the
    // pointer rests over the canvas for a few frames before it leaves.
    let over = egui::Event::PointerMoved(Pos2::new(400.0, 450.0));
    let steps = [over.clone(), over.clone(), over, egui::Event::PointerGone];
    for (step, event) in steps.into_iter().enumerate() {
        let _ = context.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0))),
                events: vec![event],
                ..Default::default()
            },
            |context| {
                egui::CentralPanel::default().show(context, |ui| {
                    show(
                        ui,
                        &mut crate::schematic::view::canvas_frame(&mut app.state),
                    );
                });
            },
        );
        if step == 2 {
            assert!(
                app.state.design.canvas_hover.is_some(),
                "a pointer over the layout canvas has a coordinate"
            );
        }
    }
    assert_eq!(app.state.design.canvas_hover, None);
}

#[test]
fn governed_layout_editor_renders_with_accessible_controls_at_phone_width() {
    let mut app = app_with_layout();
    let output = render(&mut app, Vec2::new(390.0, 844.0));
    assert!(!output.shapes.is_empty());
    let nodes = &output
        .platform_output
        .accesskit_update
        .expect("physical layout accessibility tree")
        .nodes;
    for label in [
        "Physical layout tools",
        "Select",
        "Rectangle",
        "Active physical layout layer and purpose",
        "Fit",
        "Delete selected",
        "Physical layout canvas",
    ] {
        assert!(
            nodes.iter().any(|(_, node)| node.label() == Some(label)),
            "missing accessible physical-layout control {label}"
        );
    }
}
