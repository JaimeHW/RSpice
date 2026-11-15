//! Creating a hierarchy block, drawn through a whole application.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::hierarchy::create::*;
use crate::workbench::app::open_create_hierarchy_dialog;
use egui::Context;

/// Create hierarchy is a command form: its ports table on the labels'
/// edge, and none of the explainer lines under its labels.
#[test]
fn create_hierarchy_is_a_command_form_without_explainers() {
    let mut app = RSpiceApp::test_instance();
    rspice_app_state::workbench::examples::load_example(
        "RC Lowpass Filter",
        &mut app.state.model.schematic,
    );
    let r1 = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.name == "R1")
        .expect("the example places R1")
        .id;
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(r1);
    open_create_hierarchy_dialog(
        &mut app.state.design.create_hierarchy,
        &app.state.model,
        &mut app.state.workbench,
    );
    assert!(app.state.design.create_hierarchy.open);
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut nodes = Vec::new();
    for _ in 0..2 {
        nodes = ctx
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    CreateHierarchy {
                        dialog: &mut app.state.design.create_hierarchy,
                        model: &app.state.model,
                        workbench: &mut app.state.workbench,
                    }
                    .render_create_hierarchy_dialog(ui.ctx());
                },
            )
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;
    }
    let texts = nodes
        .iter()
        .flat_map(|(_, node)| [node.label(), node.value()])
        .flatten()
        .collect::<Vec<_>>();
    assert!(
        texts.iter().any(|text| text.starts_with("Port: ")),
        "the ports table is published: {texts:?}"
    );
    let explainers = texts
        .iter()
        .filter(|text| text.contains("Stable library identity") || text.contains("transactionally"))
        .collect::<Vec<_>>();
    assert!(
        explainers.is_empty(),
        "the explainers are back: {explainers:?}"
    );
    // View's two choices span the column Cell name spans.
    let control = |name: &str| {
        nodes
            .iter()
            .map(|(_, node)| node)
            .find(|node| node.label() == Some(name) && node.role() != egui::accesskit::Role::Label)
            .and_then(egui::accesskit::Node::bounds)
            .unwrap_or_else(|| panic!("{name} is published"))
    };
    let (cell, schematic, symbol) = (
        control("Cell name"),
        control(VIEW_OPTIONS[0]),
        control(VIEW_OPTIONS[1]),
    );
    assert!(
        (schematic.x0 - cell.x0).abs() < 1.0 && (symbol.x1 - cell.x1).abs() < 1.0,
        "{schematic:?}..{symbol:?} against {cell:?}"
    );
}
