//! The stimulus surface drawn through a whole application.

use super::stimulus_surface::seeded;
use crate::workbench::surfaces::stimulus::tests::adopter;
use rspice_app_state::state::ComponentType;
use rspice_app_state::state::stimulus_library::definition::StimulusFamily;

#[test]
fn a_family_switch_resets_the_shape_parameters_and_undo_restores_them() {
    let mut app = seeded("bridge_cal_step");
    let before = app
        .state
        .model
        .workspace
        .content
        .stimulus_library
        .get("bridge_cal_step")
        .expect("fixture")
        .params
        .clone();

    crate::workbench::app::actions::stimulus::edit_family(
        app.state.design.selected_stimulus_definition.as_deref(),
        &mut app.state.design.stimulus_editor,
        &app.state.model,
        StimulusFamily::Sin,
    );
    let draft = app
        .state
        .design
        .stimulus_editor
        .draft("bridge_cal_step")
        .expect("draft");
    assert_eq!(
        draft.working().component_type(),
        ComponentType::VoltageSourceSin
    );
    assert_eq!(draft.working().params, "");

    assert!(crate::workbench::app::actions::stimulus::undo_draft(
        app.state.design.selected_stimulus_definition.as_deref(),
        &mut app.state.design.stimulus_editor,
        &app.state.model,
    ));
    let draft = app
        .state
        .design
        .stimulus_editor
        .draft("bridge_cal_step")
        .expect("draft");
    assert_eq!(draft.working().params, before);
    assert!(!draft.is_dirty());
}

/// A rename is published as a rename of the library's record, and every
/// adopter keeps the receipt it took. Nothing reads "definition removed"
/// because of a corrected spelling, and nothing reaches into the design to
/// make that true — which is what keeps an adopter on an unloaded sheet as
/// safe as the one on this one.
#[test]
fn applying_a_rename_repoints_every_adopter() {
    let mut app = seeded("bridge_cal_step");
    let id = adopter(&mut app.state.model, "bridge_cal_step", "V1");
    crate::workbench::app::actions::stimulus::edit_name(
        app.state.design.selected_stimulus_definition.as_deref(),
        &mut app.state.design.stimulus_editor,
        &app.state.model,
        "bridge_cal_pulse",
    );
    crate::workbench::app::actions::stimulus::apply_draft(
        &mut app.state.design.selected_stimulus_definition,
        &mut app.state.design.stimulus_editor,
        &mut app.state.model,
        &mut app.state.workbench,
    );

    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .stimulus_library
            .definitions()
            .iter()
            .filter(|definition| definition.name().starts_with("bridge_cal"))
            .count(),
        1,
        "a rename moves one record rather than leaving two"
    );
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .stimulus_library
            .get("bridge_cal_step")
            .map(|definition| definition.name()),
        Some("bridge_cal_pulse"),
        "the name an adopter copied still resolves to the record"
    );
    let definition = app
        .state
        .model
        .workspace
        .content
        .stimulus_library
        .get("bridge_cal_pulse")
        .expect("the renamed definition");
    assert_eq!(definition.revision(), 2);
    let component = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("the adopter");
    assert_eq!(
        component
            .stimulus_provenance
            .as_ref()
            .map(|provenance| provenance.definition.as_str()),
        Some("bridge_cal_step"),
        "the receipt keeps the name it copied; the library resolves it"
    );
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .stimulus_library
            .provenance_state(component)
            .label(),
        "behind \u{b7} library r2"
    );
}
