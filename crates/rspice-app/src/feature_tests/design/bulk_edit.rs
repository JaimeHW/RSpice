//! The bulk-edit dialog drawn and committed through a whole application.

use crate::workbench::app::RSpiceApp;
use egui::Context;
use rspice_app_design::workbench::app::dialogs::selection::bulk_edit::tests::component;
use rspice_app_design::workbench::app::dialogs::selection::bulk_edit::*;

/// Draw the dialog as its dialog step does.
fn render(app: &mut RSpiceApp, ctx: &Context) {
    let state = &mut app.state;
    render_selection_bulk_edit_dialog(
        &mut state.design.selection_bulk_edit,
        &mut state.design.schematic_bulk_edit_filters,
        &mut state.model,
        &mut state.workbench,
        ctx,
    );
}

#[test]
fn accessibility_tree_exposes_only_the_mockup_bulk_edit_surface_actions() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state.model.schematic.document_mut_for_test().components =
        vec![component(1, "X1", Some("tt"))];
    open_selection_bulk_edit_dialog(
        &mut app.state.design.selection_bulk_edit,
        &app.state.design.schematic_bulk_edit_filters,
        &app.state.model,
    );

    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 900.0),
            )),
            ..egui::RawInput::default()
        },
        |ctx| render(&mut app, ctx),
    );
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("bulk-edit access tree")
        .nodes;
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Dialog && node.label() == Some(TITLE)
    }));
    for label in [PRIMARY, "Cancel", "Save filter\u{2026}"] {
        assert!(
            nodes.iter().any(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
            }),
            "missing action {label}"
        );
    }
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::CheckBox
            && node.label() == Some("Select X1")
            && node.toggled() == Some(egui::accesskit::Toggled::True)
    }));
}
