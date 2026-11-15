//! The grid, snap and routing dialog drawn through a whole application.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::grid_snap_routing::*;
use egui::Context;

/// Draw the dialog as its dialog step does.
fn render(app: &mut RSpiceApp, ctx: &Context) {
    let state = &mut app.state;
    state
        .design
        .render_grid_snap_routing_dialog(&mut state.model, &mut state.workbench, ctx);
}

/// Grid, snap and wire routing is a command form: four rows whose
/// controls share one column, not a 760 pt card of tinted rows each
/// restating what its control is for.
#[test]
fn grid_snap_and_routing_is_a_command_form_without_restated_rows() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    assert!(open_grid_snap_routing_dialog(
        &mut app.state.design.grid_snap_routing,
        app.state.design.grid,
        &app.state.model,
        &mut app.state.workbench
    ));
    // Access updates are incremental: keep the latest of every node.
    let mut nodes = std::collections::HashMap::new();
    for _ in 0..4 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| render(&mut app, ctx),
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
    }
    // The modal's own node is the whole screen. The surface is centred
    // on it, and the primary ends 12 pt in from its right edge.
    let primary = nodes
        .values()
        .find(|node| node.role() == egui::accesskit::Role::Button && node.label() == Some(PRIMARY))
        .and_then(egui::accesskit::Node::bounds)
        .expect("the primary has bounds");
    let width = 2.0 * (primary.x1 + 12.0) - 1440.0;
    assert!(width <= 424.0, "the form is {width} pt wide");
    let says = |text: &str| {
        nodes.values().any(|node| {
            node.label().is_some_and(|label| label.contains(text))
                || node.value().is_some_and(|value| value.contains(text))
        })
    };
    assert!(
        !says("never changes snapping"),
        "a row restates its control"
    );
    assert!(says("Snap spacing"));
}

/// A routing the view cannot change is a read-only fact on the form's
/// grid, and it reads at the size of the choices above it.
#[test]
fn the_routing_fact_reads_at_the_size_of_the_choices_beside_it() {
    fn walk(shape: &egui::Shape, into: &mut Vec<(String, egui::FontId)>) {
        match shape {
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, into)),
            egui::Shape::Text(text) => into.push((
                text.galley.text().to_owned(),
                text.galley.job.sections[0].format.font_id.clone(),
            )),
            _ => {}
        }
    }
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    assert!(open_grid_snap_routing_dialog(
        &mut app.state.design.grid_snap_routing,
        app.state.design.grid,
        &app.state.model,
        &mut app.state.workbench
    ));
    let mut shapes = Vec::new();
    for _ in 0..4 {
        shapes = ctx
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..egui::RawInput::default()
                },
                |ctx| render(&mut app, ctx),
            )
            .shapes;
    }
    let mut texts = Vec::new();
    for clipped in &shapes {
        walk(&clipped.shape, &mut texts);
    }
    let font_of = |value: &str| {
        texts
            .iter()
            .find(|(text, _)| text == value)
            .map(|(_, font)| font.clone())
            .unwrap_or_else(|| panic!("{value:?} is painted"))
    };
    let choice = font_of(grid_style_label(app.state.design.grid));
    assert_eq!(font_of("Orthogonal"), choice, "the routing fact");
    assert_eq!(
        choice,
        rspice_app_workbench::ui::theme::field_sans(),
        "the field size"
    );
}
