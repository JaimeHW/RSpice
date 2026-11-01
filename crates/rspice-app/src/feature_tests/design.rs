//! The design feature observed through a whole application: each dialog it
//! draws, opened as its command opens it and drawn by the real frame through
//! the feature's own context.

use egui::accesskit::{Node, Role};
use egui::{Context, Rect, pos2, vec2};

use crate::state::{ComponentType, Point};
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::Workspace;

/// A whole application on the Design workspace of an open project.
fn design_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
    app
}

/// Place a resistor on the open schematic; its id.
fn resistor(app: &mut RSpiceApp, at: Point) -> u64 {
    app.state
        .model
        .schematic
        .add_component(ComponentType::Resistor, at)
}

/// Three frames of the whole application; the last one's AccessKit nodes.
fn frame_nodes(app: &mut RSpiceApp) -> Vec<Node> {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut frame = eframe::Frame::_new_kittest();
    let mut nodes = Vec::new();
    for pass in 1..=3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(1440.0, 900.0))),
                time: Some(f64::from(pass) * 0.1),
                focused: true,
                ..Default::default()
            },
            |ui| eframe::App::ui(app, ui, &mut frame),
        );
        nodes = output
            .platform_output
            .accesskit_update
            .map(|update| update.nodes)
            .unwrap_or_default();
    }
    nodes.into_iter().map(|(_, node)| node).collect()
}

/// Three frames of the whole application; whether the last one drew a
/// dialog titled `title`.
fn draws_dialog(app: &mut RSpiceApp, title: &str) -> bool {
    frame_nodes(app)
        .iter()
        .any(|node| node.role() == Role::Dialog && node.label() == Some(title))
}

/// How many of the last of three frames' nodes are labelled `label`.
fn labelled(app: &mut RSpiceApp, label: &str) -> usize {
    frame_nodes(app)
        .iter()
        .filter(|node| node.label() == Some(label))
        .count()
}

#[test]
fn place_bus_tap_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    Command::PlaceBusTap.execute(&mut app);
    assert!(draws_dialog(&mut app, "Place bus tap"));
    assert!(app.state.design.bus_tap.open);
}

/// The net label is an entry on the canvas, not a dialog: the card it draws
/// carries its title.
#[test]
fn net_label_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    let before = labelled(&mut app, "Net label");
    assert!(crate::workbench::app::open_net_label_placement(
        &mut app.state,
        Point::new(0, 0)
    ));
    assert_eq!(labelled(&mut app, "Net label"), before + 1);
    assert!(app.state.design.net_label_placement.open);
}

#[test]
fn create_pins_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    Command::PlacePin.execute(&mut app);
    assert!(draws_dialog(&mut app, "Create pins"));
    assert!(app.state.design.pin_port.open);
}

#[test]
fn place_text_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    Command::PlaceText.execute(&mut app);
    assert!(draws_dialog(&mut app, "Place text or design note"));
    assert!(app.state.design.design_note.open);
}

#[test]
fn draw_documentation_shape_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    Command::PlaceShape.execute(&mut app);
    assert!(draws_dialog(&mut app, "Draw documentation shape"));
    assert!(app.state.design.documentation_shape.open);
}

#[test]
fn create_array_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    let id = resistor(&mut app, Point::new(0, 0));
    let selection = &mut app.state.model.schematic.session.editor.selection;
    selection.select_only_component(id);
    Command::ArraySelection.execute(&mut app);
    assert!(draws_dialog(&mut app, "Create array"));
    assert!(app.state.design.array_selection.open);
}

#[test]
fn net_label_properties_open_and_draw_in_the_frame() {
    let mut app = design_app();
    let label = app
        .state
        .model
        .schematic
        .add_net_label(Point::new(0, 0), "sense".to_owned());
    let selection = &mut app.state.model.schematic.session.editor.selection;
    selection.select_only_net_label(label);
    Command::ObjectProperties.execute(&mut app);
    assert!(draws_dialog(&mut app, "Net label properties"));
    assert!(app.state.design.object_properties.open);
}

#[test]
fn rename_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    let id = resistor(&mut app, Point::new(0, 0));
    let selection = &mut app.state.model.schematic.session.editor.selection;
    selection.select_only_component(id);
    Command::RenameSelection.execute(&mut app);
    assert!(draws_dialog(&mut app, "Rename"));
    assert!(app.state.design.rename_selection.open);
}

#[test]
fn find_and_replace_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    Command::FindReplace.execute(&mut app);
    assert!(draws_dialog(&mut app, "Find and replace"));
    assert!(app.state.design.find_replace.open);
}

#[test]
fn shared_properties_open_and_draw_in_the_frame() {
    let mut app = design_app();
    let first = resistor(&mut app, Point::new(0, 0));
    let second = resistor(&mut app, Point::new(100, 0));
    let selection = &mut app.state.model.schematic.session.editor.selection;
    selection.select_component(first);
    selection.select_component(second);
    Command::ObjectProperties.execute(&mut app);
    assert!(draws_dialog(&mut app, "Properties"));
    assert!(app.state.design.shared_properties.open);
}

#[test]
fn grid_snap_and_routing_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    Command::GridSnapRouting.execute(&mut app);
    assert!(draws_dialog(&mut app, "Grid, snap and wire routing"));
    assert!(app.state.design.grid_snap_routing.open);
}

#[test]
fn display_options_open_and_draw_in_the_frame() {
    let mut app = design_app();
    Command::VisibilityOptions.execute(&mut app);
    assert!(draws_dialog(&mut app, "Display options"));
    assert!(app.state.design.schematic_visibility_dialog.open);
}
