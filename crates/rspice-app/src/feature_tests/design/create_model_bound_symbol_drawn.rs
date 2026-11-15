//! Creating a model-bound symbol, drawn through a whole application.

use crate::feature_tests::design::create_model_bound_symbol::{
    open_create_model_bound_symbol_dialog, state_with_bound_mos_model,
};
use crate::workbench::app::RSpiceApp;
use egui::{Context, Rect, vec2};
use rspice_app_design::workbench::app::dialogs::create_model_bound_symbol::state::*;
use rspice_app_design::workbench::app::dialogs::create_model_bound_symbol::tests::{
    self as dialog, node_bounds,
};

/// Draw the dialog as its dialog step does.
fn render(app: &mut RSpiceApp, ctx: &Context) {
    let state = &mut app.state;
    dialog::render(
        &mut state.design.create_model_bound_symbol,
        &mut state.model,
        &mut state.workbench,
        ctx,
    );
}

fn render_accessibility_frame(
    app: &mut RSpiceApp,
    ctx: &Context,
    size: egui::Vec2,
) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, size)),
            ..egui::RawInput::default()
        },
        |ctx| render(app, ctx),
    )
    .platform_output
    .accesskit_update
    .expect("create-symbol access tree")
    .nodes
}

fn matching_bounds(
    nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)],
    role: egui::accesskit::Role,
    label: &str,
) -> Vec<egui::accesskit::Rect> {
    nodes
        .iter()
        .filter(|(_, node)| node.role() == role && node.label() == Some(label))
        .filter_map(|(_, node)| node.bounds())
        .collect()
}

fn rects_overlap(a: egui::accesskit::Rect, b: egui::accesskit::Rect) -> bool {
    a.x0 < b.x1 && b.x0 < a.x1 && a.y0 < b.y1 && b.y0 < a.y1
}

#[test]
fn accessibility_tree_exposes_the_mockup_dialog_and_complete_actions() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state = state_with_bound_mos_model();
    open_create_model_bound_symbol_dialog(&mut app.state);
    app.state
        .design
        .create_model_bound_symbol
        .pin_contract_reviewed = true;
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1280.0, 900.0))),
            ..egui::RawInput::default()
        },
        |ctx| render(&mut app, ctx),
    );
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("create-symbol access tree")
        .nodes;
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Dialog && node.label() == Some(TITLE)
    }));
    for label in [PRIMARY, "Cancel", "Add pin", "Remove pin"] {
        assert!(
            nodes.iter().any(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
            }),
            "missing action {label}"
        );
    }
}

#[test]
fn mockup_width_keeps_pin_actions_and_generated_views_in_separate_tracks() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state = state_with_bound_mos_model();
    open_create_model_bound_symbol_dialog(&mut app.state);

    // A warm-up frame lets the dialog settle its measured body height before
    // the production-sized frame is inspected.
    let size = vec2(772.0, 477.0);
    let _ = render_accessibility_frame(&mut app, &ctx, size);
    let nodes = render_accessibility_frame(&mut app, &ctx, size);
    let dialog = node_bounds(&nodes, egui::accesskit::Role::Dialog, TITLE);

    let earlier = matching_bounds(
        &nodes,
        egui::accesskit::Role::Button,
        "Move pin earlier in netlist order",
    );
    let later = matching_bounds(
        &nodes,
        egui::accesskit::Role::Button,
        "Move pin later in netlist order",
    );
    assert_eq!(
        earlier.len(),
        app.state.design.create_model_bound_symbol.pins.len()
    );
    assert_eq!(later.len(), earlier.len());
    for (up, down) in earlier.into_iter().zip(later) {
        assert!((up.x1 - up.x0 - 24.0).abs() <= 0.5);
        assert!((down.x1 - down.x0 - 24.0).abs() <= 0.5);
        assert!(!rects_overlap(up, down));
        assert!(up.x0 >= dialog.x0 && down.x1 <= dialog.x1);
    }

    let generated = ["symbol", "parameter form", "simulation test fixture"]
        .map(|label| node_bounds(&nodes, egui::accesskit::Role::CheckBox, label));
    assert!(!rects_overlap(generated[0], generated[1]));
    assert!(!rects_overlap(generated[1], generated[2]));
    assert!(generated[0].x0 >= dialog.x0);
    assert!(generated[2].x1 <= dialog.x1);
}

#[test]
fn validation_status_reserves_height_and_desktop_generated_views_do_not_clip() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state = state_with_bound_mos_model();
    open_create_model_bound_symbol_dialog(&mut app.state);

    let size = vec2(1440.0, 900.0);
    let _ = render_accessibility_frame(&mut app, &ctx, size);
    let invalid_nodes = render_accessibility_frame(&mut app, &ctx, size);
    let invalid_add = node_bounds(&invalid_nodes, egui::accesskit::Role::Button, "Add pin");

    app.state
        .design
        .create_model_bound_symbol
        .pin_contract_reviewed = true;
    let valid_nodes = render_accessibility_frame(&mut app, &ctx, size);
    let valid_add = node_bounds(&valid_nodes, egui::accesskit::Role::Button, "Add pin");
    assert!(
        (invalid_add.y0 - valid_add.y0).abs() <= 1.0,
        "validation status changed body placement: invalid={invalid_add:?}, valid={valid_add:?}"
    );

    let dialog = node_bounds(&valid_nodes, egui::accesskit::Role::Dialog, TITLE);
    let generated = ["symbol", "parameter form", "simulation test fixture"]
        .map(|label| node_bounds(&valid_nodes, egui::accesskit::Role::CheckBox, label));
    assert!(!rects_overlap(generated[0], generated[1]));
    assert!(!rects_overlap(generated[1], generated[2]));
    assert!(generated[0].x0 >= dialog.x0);
    assert!(generated[2].x1 <= dialog.x1);
}
