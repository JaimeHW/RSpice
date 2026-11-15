//! Renaming a selection, drawn through a whole application.

use crate::feature_tests::design::rename::{apply_commit, component_app};
use crate::state::Component;
use crate::state::{ComponentType, NetLabel, Point, Wire};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::selection::rename::tests::{self as dialog, key_event};
use crate::workbench::app::dialogs::selection::rename::*;
use egui::Context;

/// Draw the dialog as its dialog step does.
fn render(app: &mut RSpiceApp, ctx: &Context) {
    let state = &mut app.state;
    dialog::render(
        &mut state.design.rename_selection,
        &mut state.model,
        &mut state.workbench,
        ctx,
    );
}

fn dialog_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1_100.0, 850.0),
        )),
        events,
        ..Default::default()
    }
}

/// The form names what it is renaming and how far the rename reaches,
/// and states no internal ID for either.
#[test]
fn the_object_row_names_the_object_without_its_internal_id() {
    let (app, _) = component_app();
    let target = selected_rename_target(&app.state.model).expect("one renameable object");
    let context = object_context(&target);
    assert_eq!(context, "Resistor R1");
    assert!(!context.contains("COMPONENT-"));
    assert_eq!(rename_scope(&app.state.model.schematic, &target), None);
}

#[test]
fn availability_requires_one_live_renameable_object_and_write_access() {
    let (mut app, id) = component_app();
    assert!(rename_selection_available(&app.state.model));

    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(id + 999);
    assert!(!rename_selection_available(&app.state.model));
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(id);
    app.state.model.schematic.session.read_only = true;
    assert!(!rename_selection_available(&app.state.model));
}

#[test]
fn named_wire_and_interface_port_open_the_net_scoped_rename_target() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::new(81, vec![Point::new(0, 0), Point::new(40, 0)]));
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(82, Point::new(20, 0), "sense"));
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_wire(81);
    assert!(open_selected_object_rename(
        &mut app.state.design.rename_selection,
        app.state.design.object_properties.open || app.state.design.tabbed_property_dialog.open,
        &app.state.model
    ));
    assert!(matches!(
        app.state.design.rename_selection.target,
        Some(RenameSelectionTarget::NamedNet(ref target))
            if target.name == "sense" && target.labels[0].id == 82
    ));
    app.state.design.rename_selection.close();

    let port =
        Component::new(83, ComponentType::Port, Point::new(0, 0)).with_name_value("P83", "sense");
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(port);
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(83);
    assert!(open_selected_object_rename(
        &mut app.state.design.rename_selection,
        app.state.design.object_properties.open || app.state.design.tabbed_property_dialog.open,
        &app.state.model
    ));
    assert!(matches!(
        app.state.design.rename_selection.target,
        Some(RenameSelectionTarget::NamedNet(ref target))
            if target.name == "sense"
                && target.ports.iter().any(|port| port.id == 83)
    ));
}

#[test]
fn component_validation_enforces_prefix_and_case_insensitive_uniqueness() {
    let (mut app, id) = component_app();
    let other = app
        .state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(10, 0));
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == other)
        .unwrap()
        .name = "RLOAD".to_owned();
    let target = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .unwrap()
        .clone();

    assert!(matches!(
        validate_draft(
            &app.state.model.schematic,
            &RenameSelectionTarget::Component(Box::new(target.clone())),
            "C7",
            &|_| true,
        ),
        RenameValidation::Invalid(_)
    ));
    assert!(matches!(
        validate_draft(
            &app.state.model.schematic,
            &RenameSelectionTarget::Component(Box::new(target.clone())),
            "rload",
            &|_| true,
        ),
        RenameValidation::Invalid(_)
    ));
    assert!(matches!(
        validate_draft(
            &app.state.model.schematic,
            &RenameSelectionTarget::Component(Box::new(target)),
            "R_GAIN_2",
            &|_| true,
        ),
        RenameValidation::Valid(_)
    ));
}

#[test]
fn stale_target_and_cancel_never_mutate() {
    let (mut app, id) = component_app();
    let original = app.state.model.schematic.document().components[0].clone();
    assert!(open_selected_object_rename(
        &mut app.state.design.rename_selection,
        app.state.design.object_properties.open || app.state.design.tabbed_property_dialog.open,
        &app.state.model
    ));
    app.state.design.rename_selection.draft = "R_CANCELLED".to_owned();
    app.state.design.rename_selection.close();
    assert_eq!(app.state.model.schematic.document().components[0], original);
    assert!(!app.state.model.schematic.can_undo());

    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(id);
    assert!(open_selected_object_rename(
        &mut app.state.design.rename_selection,
        app.state.design.object_properties.open || app.state.design.tabbed_property_dialog.open,
        &app.state.model
    ));
    let target = app.state.design.rename_selection.target.clone().unwrap();
    let pending_commit =
        match validate_draft(&app.state.model.schematic, &target, "R_STALE", &|_| true) {
            RenameValidation::Valid(commit) => commit,
            other => panic!("expected initially valid rename, got {other:?}"),
        };
    app.state.model.schematic.document_mut_for_test().components[0].value = "2k".to_owned();
    let before = app.state.model.schematic.document().components[0].clone();
    assert!(matches!(
        validate_draft(&app.state.model.schematic, &target, "R_STALE", &|_| true),
        RenameValidation::Invalid(_)
    ));
    assert!(apply_commit(&mut app.state, *pending_commit).is_err());
    assert_eq!(app.state.model.schematic.document().components[0], before);
    assert!(!app.state.model.schematic.can_undo());
}

#[test]
fn rendered_enter_commits_and_escape_cancels() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let (mut app, id) = component_app();
    assert!(open_selected_object_rename(
        &mut app.state.design.rename_selection,
        app.state.design.object_properties.open || app.state.design.tabbed_property_dialog.open,
        &app.state.model
    ));
    app.state.design.rename_selection.draft = "R_ENTER".to_owned();
    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
        render(&mut app, ctx)
    });
    assert!(!app.state.design.rename_selection.open);
    assert_eq!(app.state.model.schematic.document().components[0].id, id);
    assert_eq!(
        app.state.model.schematic.document().components[0].name,
        "R_ENTER"
    );

    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(id);
    assert!(open_selected_object_rename(
        &mut app.state.design.rename_selection,
        app.state.design.object_properties.open || app.state.design.tabbed_property_dialog.open,
        &app.state.model
    ));
    app.state.design.rename_selection.draft = "R_ESCAPE".to_owned();
    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
        render(&mut app, ctx)
    });
    assert!(!app.state.design.rename_selection.open);
    assert_eq!(
        app.state.model.schematic.document().components[0].name,
        "R_ENTER"
    );
}
