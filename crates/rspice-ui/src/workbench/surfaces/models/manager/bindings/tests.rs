//! Every unresolved instance reaches the reader, not just the first one.

use super::*;

use crate::state::model_library::ModelType;
use crate::state::{Component, ComponentType, Point};
use crate::workbench::app_state::AppState;

/// A design with `count` instances naming a model nothing provides.
fn design_with_unresolved(count: u64) -> AppState {
    let mut state = AppState::default();
    state.model_library_manager.clear();
    let mut library = ModelLibrary::new("alpha");
    library.add_model(DeviceModel::new("nch", ModelType::Nmos));
    state.model_library_manager.add_library(library);

    state.workspace.ensure_active_buffer();
    let mut schematic = state
        .workspace
        .active_schematic()
        .cloned()
        .expect("an active schematic exists");
    for index in 0..count {
        let mut component = Component::new(index + 1, ComponentType::Nmos, Point::origin());
        component.name = format!("M{index}");
        component.params = format!("model=absent{index}");
        schematic.components.push(component);
    }
    state.workspace.save_active_schematic(&schematic);
    state.schematic = schematic.clone();
    state.workspace.save_active_schematic(&schematic);
    state
}

fn diagnostics(state: &mut AppState) -> Vec<BindingDiagnostic> {
    let mut pending = Vec::new();
    let render = ManagerRenderContext {
        state,
        pending_actions: &mut pending,
    };
    ConsumerIndex::build(&render).diagnostics
}

#[test]
fn every_unresolved_instance_is_a_row_of_its_own() {
    let mut state = design_with_unresolved(11);
    let diagnostics = diagnostics(&mut state);
    assert_eq!(
        diagnostics.len(),
        11,
        "the page used to keep the first finding and discard the rest"
    );
    let first = &diagnostics[0];
    assert_eq!(first.instance, "M0");
    assert_eq!(first.reference, "absent0");
    assert_eq!(
        first.reason,
        "no library in this project provides an executable definition"
    );
    assert!(
        !first.sheet.is_empty(),
        "a row names the sheet the instance sits on"
    );
    assert!(
        diagnostics
            .iter()
            .map(|diagnostic| diagnostic.component_id)
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            == 11,
        "each row identifies exactly one instance, which is what makes it navigable"
    );
}

/// A declared provider that is not the executable one is its own finding.
#[test]
fn an_instance_bound_to_the_wrong_provider_says_which_one_wins() {
    let mut state = AppState::default();
    state.model_library_manager.clear();
    for name in ["alpha", "beta"] {
        let mut library = ModelLibrary::new(name);
        library.add_model(DeviceModel::new("nch", ModelType::Nmos));
        state.model_library_manager.add_library(library);
    }
    state
        .model_library_manager
        .resolve_definition_provider(
            ModelConsumerScope::PrimitiveModel,
            "nch",
            "alpha",
            "provider-aware binding test",
        )
        .expect("the contested provider can be resolved");
    state.workspace.ensure_active_buffer();
    let mut schematic = state
        .workspace
        .active_schematic()
        .cloned()
        .expect("an active schematic exists");
    let mut component = Component::new(1, ComponentType::Nmos, Point::origin());
    component.name = "M1".to_owned();
    component.params = "model=nch model_library=beta".to_owned();
    schematic.components.push(component);
    state.workspace.save_active_schematic(&schematic);

    let diagnostics = diagnostics(&mut state);
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].instance, "M1");
    assert_eq!(diagnostics[0].reference, "nch");
    assert!(
        diagnostics[0].reason.contains("beta") && diagnostics[0].reason.contains("alpha"),
        "the row names both the declared provider and the one that executes: {}",
        diagnostics[0].reason
    );
}

/// Rendering: the card is the exception, and every row carries both routes out.
#[test]
fn the_card_offers_both_routes_out_and_vanishes_when_there_is_nothing_to_say() {
    fn nodes(
        state: &mut AppState,
        diagnostics: &[BindingDiagnostic],
    ) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
        let ctx = egui::Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut pending = Vec::new();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 700.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut app = ManagerRenderContext {
                        state,
                        pending_actions: &mut pending,
                    };
                    unresolved_card(ui, &mut app, diagnostics);
                });
            },
        );
        output
            .platform_output
            .accesskit_update
            .expect("an access tree")
            .nodes
    }

    fn button(
        nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)],
        label: &str,
    ) -> Option<egui::accesskit::Node> {
        nodes
            .iter()
            .find(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
            })
            .map(|(_, node)| node.clone())
    }

    let mut state = design_with_unresolved(3);
    let findings = diagnostics(&mut state);
    let tree = nodes(&mut state, &findings);
    assert!(
        button(&tree, "Show instance").is_some(),
        "a row can be navigated to"
    );
    let bind = button(&tree, "Bind to selection…").expect("a row offers the binding action");
    assert!(
        bind.is_disabled(),
        "with no model selected in the catalog there is nothing to bind to, and the \
         control says so rather than failing after being pressed"
    );

    // Nothing to report renders nothing at all.
    let mut clean = AppState::default();
    let tree = nodes(&mut clean, &[]);
    assert!(button(&tree, "Show instance").is_none());
    assert!(button(&tree, "Bind to selection…").is_none());
}
