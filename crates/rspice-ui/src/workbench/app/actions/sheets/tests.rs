//! What the sheet actions promise the strip, the chords and the menus.

use super::*;
use crate::state::{
    Component, ComponentType, DrawingSheetTitleFieldId, Point, SheetDefinition, SheetPortPolicy,
    SheetTemplate,
};

/// One cell view with two governed sheets, and one component on each.
fn two_sheet_state() -> (AppState, SheetId, SheetId) {
    let mut state = AppState::default();
    state.schematic.components = vec![
        Component::new(10, ComponentType::Resistor, Point::new(10, 10)),
        Component::new(20, ComponentType::Capacitor, Point::new(40, 10)),
    ];
    let key = state.workspace.active_schematic_reference().key();
    let first = state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Sheet 1", [10, 20])
        .expect("first sheet");
    let catalog = state
        .workspace
        .design_management
        .sheet_catalog_mut(&key)
        .expect("sheet catalog");
    let second = catalog
        .create_sheet(
            SheetDefinition {
                name: "Sheet 2".to_owned(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(first),
        )
        .expect("second sheet");
    catalog
        .assign_objects(catalog.revision(), second, [20])
        .expect("second sheet assignment");
    catalog.set_active(first).expect("active sheet");
    (state, first, second)
}

fn delete_behavior(state: &mut AppState, behavior: SheetDeleteBehavior) {
    let key = state.workspace.active_schematic_reference().key();
    let catalog = state
        .workspace
        .design_management
        .sheet_catalog_mut(&key)
        .expect("sheet catalog");
    let mut settings = catalog.settings().clone();
    if settings.delete_behavior == behavior {
        return;
    }
    settings.delete_behavior = behavior;
    let revision = catalog.revision();
    catalog
        .set_settings(revision, settings)
        .expect("delete behavior");
}

#[test]
fn the_strip_projects_every_sheet_in_catalog_order() {
    let (state, first, second) = two_sheet_state();
    let entries = sheet_entries(&state);

    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].id, first);
    assert_eq!(entries[0].name, "Sheet 1");
    assert_eq!(entries[0].page, 1);
    assert_eq!(entries[1].id, second);
    assert_eq!(entries[1].page, 2);
    assert_eq!(active_sheet_id(&state), Some(first));
}

#[test]
fn activating_a_sheet_retires_the_selection_and_frames_the_new_drawing() {
    let (mut state, _, second) = two_sheet_state();
    state.schematic.selection.select_component(10);
    state.schematic.needs_drawing_sheet_fit = false;

    activate_sheet(&mut state, second).expect("activation publishes");

    assert_eq!(active_sheet_id(&state), Some(second));
    assert!(state.schematic.selection.is_empty());
    assert!(!state.schematic.net_highlight.active);
    assert!(state.schematic.needs_drawing_sheet_fit);
    assert!(!state.schematic.needs_fit);
}

#[test]
fn re_activating_the_current_sheet_publishes_nothing() {
    let (mut state, first, _) = two_sheet_state();
    let revision = state.workspace.design_management.revision();
    state.schematic.selection.select_component(10);

    activate_sheet(&mut state, first).expect("an inert activation still succeeds");

    assert_eq!(state.workspace.design_management.revision(), revision);
    assert!(
        state.schematic.selection.has_component(10),
        "a chip redrawn every frame must not clear the selection"
    );
}

#[test]
fn page_navigation_wraps_around_both_ends() {
    let (mut state, first, second) = two_sheet_state();

    next_sheet(&mut state).expect("forward");
    assert_eq!(active_sheet_id(&state), Some(second));
    next_sheet(&mut state).expect("forward wraps");
    assert_eq!(active_sheet_id(&state), Some(first));
    previous_sheet(&mut state).expect("backward wraps");
    assert_eq!(active_sheet_id(&state), Some(second));
    go_to_sheet(&mut state, 0).expect("indexed");
    assert_eq!(active_sheet_id(&state), Some(first));
}

#[test]
fn a_single_sheet_catalog_offers_no_navigation() {
    let mut state = AppState::default();
    let key = state.workspace.active_schematic_reference().key();
    state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Sheet 1", [10])
        .expect("first sheet");

    assert_eq!(sheet_count(&state), 1);
    assert!(next_sheet(&mut state).is_err());
    assert!(previous_sheet(&mut state).is_err());
}

#[test]
fn a_new_sheet_lands_after_the_active_one_and_becomes_current() {
    let (mut state, first, second) = two_sheet_state();

    new_sheet(&mut state).expect("creation publishes");

    let entries = sheet_entries(&state);
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].id, first);
    assert_eq!(entries[1].name, "Sheet 3");
    assert_eq!(entries[2].id, second);
    assert_eq!(active_sheet_id(&state), Some(entries[1].id));
}

#[test]
fn renaming_a_sheet_moves_the_printed_title_with_it() {
    let (mut state, first, _) = two_sheet_state();

    rename_sheet(&mut state, first, "  Input stage  ".to_owned()).expect("rename publishes");

    let catalog = active_sheet_catalog(&state).expect("catalog");
    let sheet = catalog.find(first).expect("renamed sheet");
    assert_eq!(sheet.name(), "Input stage");
    assert_eq!(
        sheet
            .page_format()
            .title_block
            .fields
            .get(&DrawingSheetTitleFieldId::SheetTitle)
            .map(|field| field.value.as_str()),
        Some("Input stage"),
        "the sheet name and the printed title are one authored fact"
    );
}

#[test]
fn a_blocking_delete_policy_refuses_and_states_what_the_sheet_holds() {
    let (mut state, _, second) = two_sheet_state();
    delete_behavior(&mut state, SheetDeleteBehavior::BlockWhileReferenced);

    assert_eq!(
        plan_delete(&state, second),
        SheetDeletePlan::Blocked { objects: 1 }
    );
    let error = delete_sheet(&mut state, second).expect_err("the sheet still owns an object");
    assert!(error.contains("still owns"), "{error}");
    assert_eq!(sheet_count(&state), 2);
}

#[test]
fn a_moving_delete_policy_relocates_the_objects_and_undo_restores_them() {
    let (mut state, first, second) = two_sheet_state();
    delete_behavior(
        &mut state,
        SheetDeleteBehavior::MoveReferencesToReviewedReplacement,
    );

    assert_eq!(
        plan_delete(&state, second),
        SheetDeletePlan::MovesObjects {
            objects: 1,
            destination: first,
            destination_name: "Sheet 1".to_owned(),
        }
    );
    delete_sheet(&mut state, second).expect("delete publishes");

    let catalog = active_sheet_catalog(&state).expect("catalog");
    assert_eq!(catalog.sheets().len(), 1);
    assert_eq!(catalog.sheet_for_object(20), Some(first));

    state
        .undo_project_design()
        .expect("one guarded transaction");
    let catalog = active_sheet_catalog(&state).expect("catalog");
    assert_eq!(catalog.sheets().len(), 2);
    assert_eq!(catalog.sheet_for_object(20), Some(second));
}

#[test]
fn the_last_sheet_of_a_catalog_is_retained() {
    let mut state = AppState::default();
    let key = state.workspace.active_schematic_reference().key();
    let only = state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Sheet 1", [10])
        .expect("first sheet");

    assert_eq!(plan_delete(&state, only), SheetDeletePlan::LastSheet);
    let error = delete_sheet(&mut state, only).expect_err("the catalog keeps a sheet");
    assert!(error.contains("last sheet"), "{error}");
}

#[test]
fn moving_a_sheet_publishes_the_whole_order() {
    let (mut state, first, second) = two_sheet_state();

    move_sheet(&mut state, second, 0).expect("reorder publishes");

    let entries = sheet_entries(&state);
    assert_eq!(entries[0].id, second);
    assert_eq!(entries[1].id, first);
    assert_eq!(entries[0].page, 1, "print order follows catalog order");
    assert_eq!(entries[1].page, 2);
}
