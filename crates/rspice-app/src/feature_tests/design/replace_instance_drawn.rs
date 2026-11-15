//! Replacing an instance, drawn through a whole application.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use egui::Context;
use rspice_app_design::workbench::app::dialogs::replace_instance::open_replace_instance_dialog;
use rspice_app_design::workbench::app::dialogs::replace_instance::*;
use rspice_app_state::state::ComponentType;
use rspice_app_state::state::Point;

/// Replace instance is a wide command form: Current, a pick list of the
/// masters that can stand in (not a free-text field), and the mapping
/// table. None of the wizard it was drawn as survives: no step tabs,
/// purpose line or "Context" side table.
#[test]
fn replace_instance_is_a_command_form_with_a_pick_list_and_a_mapping_table() {
    let mut app = RSpiceApp::test_instance();
    app.state = placed_source("");
    open_replace_instance_dialog(
        &mut app.state.design.replace_instance,
        &app.state.design.property_registry,
        &app.state.model,
        &mut app.state.workbench,
    );
    assert!(app.state.design.replace_instance.open);
    let candidates = &app.state.design.replace_instance.candidates;
    assert!(
        candidates.contains(&app.state.design.replace_instance.replacement),
        "the opening choice is one of the picks"
    );
    assert_eq!(
        candidates.first(),
        Some(&app.state.design.replace_instance.replacement),
        "the proposed master leads the picks"
    );

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
                    ReplaceInstance {
                        dialog: &mut app.state.design.replace_instance,
                        registry: &app.state.design.property_registry,
                        model: &mut app.state.model,
                        workbench: &mut app.state.workbench,
                    }
                    .render_replace_instance_dialog(ui.ctx());
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
        nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::ComboBox && node.label() == Some("Replace with")
        }),
        "Replace with is a pick list"
    );
    assert!(
        texts.iter().any(|text| text.starts_with("Name: ")),
        "the mapping table is published"
    );
    let wizard = texts
        .iter()
        .filter(|text| {
            text.contains("Scope") || text.contains("Context") || text.contains("Mapping")
        })
        .collect::<Vec<_>>();
    assert!(wizard.is_empty(), "the wizard is back: {wizard:?}");
}

pub(crate) fn placed_source(params: &str) -> AppState {
    let mut state = AppState::default();
    let id = state
        .model
        .schematic
        .add_component(ComponentType::VoltageSource, Point::origin());
    if !params.is_empty() {
        state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("the placed source is retained")
            .params = params.to_owned();
    }
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(id);
    state
}
