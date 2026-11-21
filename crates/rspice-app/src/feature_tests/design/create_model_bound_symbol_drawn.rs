//! Creating a model-bound symbol, drawn through a whole application.

use crate::feature_tests::design::create_model_bound_symbol::{
    open_create_model_bound_symbol_dialog, state_with_bound_mos_model,
};
use crate::workbench::app::RSpiceApp;
use egui::{Context, Rect, vec2};
use rspice_app_design::workbench::app::dialogs::create_model_bound_symbol::state::*;
use rspice_app_design::workbench::app::dialogs::create_model_bound_symbol::tests::{
    self as dialog,
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
