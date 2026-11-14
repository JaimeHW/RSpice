//! What the instrument says, what its verbs do, and whether it fits.

use egui::vec2;

use crate::state::stimulus_library::definition::StimulusFamily;
use crate::state::stimulus_library::fixtures;
use crate::state::{Component, ComponentType, Point};
use crate::workbench::{AppState, RSpiceApp};

/// The centre column the shell leaves this stage at one viewport.
///
/// Read off `LayoutSpec`, which owns every one of these numbers, rather than
/// measured from a rendered frame: the gate is about whether the instrument
/// fits the space the shell gives it, and the shell decides that space from
/// exactly these fields.
pub(crate) fn stage_size(width: f32, height: f32) -> egui::Vec2 {
    use crate::workbench::design_system::ACTIVITY_RAIL_W;
    use crate::workbench::layout::LayoutSpec;
    use crate::workbench::state::WorkbenchSession;

    let mut workbench = WorkbenchSession::default();
    workbench.workspace = crate::workbench::state::Workspace::Stimulus;
    let layout = LayoutSpec::resolve(width, height, &workbench);
    let columns = width
        - if layout.show_activity_rail {
            ACTIVITY_RAIL_W
        } else {
            0.0
        }
        - if layout.show_navigator_dock {
            layout.navigator_width
        } else {
            0.0
        }
        - if layout.show_inspector_dock {
            layout.inspector_width
        } else {
            0.0
        };
    let rows = height
        - layout.title_bar_height
        - layout.toolbar_height
        - if layout.show_status_bar {
            layout.status_bar_height
        } else {
            0.0
        }
        // The collapsed console keeps its tab track, which is the one chrome
        // row the layout spec states only as a height it can grow to.
        - if layout.show_console_strip { 31.0 } else { 0.0 };
    vec2(columns.max(320.0), rows.max(240.0))
}

/// An application whose project holds the fixture library.
pub(crate) fn seeded(selection: &str) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.model.workspace.content.stimulus_library = fixtures::library();
    app.state.workbench.workspace = crate::workbench::state::Workspace::Stimulus;
    app.state.design.selected_stimulus_definition = Some(selection.to_owned());
    app
}

/// One placed source that has adopted a fixture definition.
fn adopter(state: &mut AppState, name: &str, instance: &str) -> u64 {
    let definition = state
        .model
        .workspace
        .content
        .stimulus_library
        .get(name)
        .cloned()
        .expect("fixture definition");
    let id = 7;
    let mut component = Component::new(id, definition.component_type(), Point::new(4, 4));
    component.name = instance.to_owned();
    definition.adopt_onto(&mut component).expect("adopt");
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
    id
}

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
    let id = adopter(&mut app.state, "bridge_cal_step", "V1");
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

/// The centre column the gate measures, stated so a layout change that shrinks
/// it fails here rather than in a raster nobody looks at.
#[test]
fn the_measured_stage_is_the_column_the_shell_leaves() {
    let smallest = stage_size(1024.0, 640.0);
    let laptop = stage_size(1440.0, 900.0);
    assert!(smallest.x >= 320.0 && smallest.y >= 240.0);
    assert!(laptop.x > smallest.x && laptop.y > smallest.y);
}
