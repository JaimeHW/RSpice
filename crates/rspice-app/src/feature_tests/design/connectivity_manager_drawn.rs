//! The connectivity manager drawn through a whole application.

use crate::workbench::RSpiceApp;
use egui::Context;
use rspice_app_design::workbench::app::dialogs::connectivity_manager::*;
use std::collections::HashMap;

/// Every node the manager publishes, after enough passes for the surface to
/// settle. Access updates are incremental, so the latest of each is kept.
fn rendered_nodes(mut app: RSpiceApp) -> HashMap<egui::accesskit::NodeId, egui::accesskit::Node> {
    let ctx = Context::default();
    ctx.enable_accesskit();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut nodes = HashMap::new();
    for _ in 0..4 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                ConnectivityManager {
                    dialog: &mut app.state.design.connectivity_manager,
                    model: &mut app.state.model,
                    workbench: &mut app.state.workbench,
                }
                .render_connectivity_manager_dialog(ctx);
            },
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
    }
    nodes
}

fn node_text(node: &egui::accesskit::Node) -> String {
    format!(
        "{} {}",
        node.label().unwrap_or_default(),
        node.value().unwrap_or_default()
    )
}

/// The surface's left and right edges. The modal's own node is the whole
/// screen, so they are read off the primary button, which ends 12 pt in
/// from the right edge of a surface centred on the 1440 pt screen.
fn surface_edges(
    nodes: &HashMap<egui::accesskit::NodeId, egui::accesskit::Node>,
    primary: &str,
) -> (f64, f64) {
    let right = nodes
        .values()
        .find(|node| node.role() == egui::accesskit::Role::Button && node.label() == Some(primary))
        .and_then(egui::accesskit::Node::bounds)
        .expect("the primary button has bounds")
        .x1
        + 12.0;
    (1440.0 - right, right)
}

/// The manager's rows start on the dialog's left edge, as its title does,
/// not flush against the frame; and no section header carries a constant
/// kicker.
#[test]
fn the_manager_rows_start_on_the_dialog_edge_without_kickers() {
    let mut app = RSpiceApp::test_instance();
    open_connectivity_manager(
        &mut app.state.design.connectivity_manager,
        &mut app.state.model,
        &mut app.state.workbench,
        &app.state.platform,
    );
    let nodes = rendered_nodes(app);
    let (left, _) = surface_edges(&nodes, PRIMARY);
    let signal = nodes
        .values()
        .find(|node| node_text(node).trim() == "Signal")
        .and_then(egui::accesskit::Node::bounds)
        .expect("the Signal heading has bounds");
    assert!(
        signal.x0 - left >= 11.5,
        "the table starts {} pt in from the frame",
        signal.x0 - left
    );
    assert!(
        !nodes
            .values()
            .any(|node| node_text(node).contains("hierarchy-resolved")),
        "a header still carries the constant kicker"
    );
}

/// Creating a bus is a command form: a 420 pt card asking for the
/// declaration, with no row that says the same thing for every bus.
#[test]
fn creating_a_bus_is_a_command_form_without_constant_rows() {
    let mut app = RSpiceApp::test_instance();
    open_connectivity_manager(
        &mut app.state.design.connectivity_manager,
        &mut app.state.model,
        &mut app.state.workbench,
        &app.state.platform,
    );
    app.state.design.connectivity_manager.page = ConnectivityManagerPage::CreateBus;
    let nodes = rendered_nodes(app);
    let (left, right) = surface_edges(&nodes, CREATE_BUS_PRIMARY);
    assert!(
        right - left <= 424.0,
        "the form is {} pt wide",
        right - left
    );
    assert!(nodes.values().any(|node| {
        node.role() == egui::accesskit::Role::TextInput && node.label() == Some("Declaration")
    }));
    assert!(
        !nodes
            .values()
            .any(|node| node_text(node).contains("Discipline conversion")),
        "the constant discipline row is back"
    );
}
