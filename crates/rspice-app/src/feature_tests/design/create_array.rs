//! Create array through the whole application: when it is offered, what it
//! names the copies, and what it reports.

use crate::feature_tests::replay::{PRIMARY, Replay};
use crate::workbench::app::RSpiceApp;
use egui::accesskit::Role;
use rspice_app_design::workbench::design::commands::DesignCommand;
use rspice_app_state::state::{
    ComponentType, Point, SchematicArrayCount, SchematicArrayKind, SchematicProbe,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::CommandAvailability;

/// A selection holding a probe greys Create array and says why, rather than
/// opening a form whose first canvas click is refused for that reason.
#[test]
fn create_array_is_greyed_with_its_reason_while_a_probe_is_selected() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    let schematic = &mut app.state.model.schematic;
    let part = schematic.add_component(ComponentType::Resistor, Point::origin());
    schematic.document_mut_for_test().probes.push(
        SchematicProbe::new(900, Point::new(0, 40), "P1", None).expect("a probe is well formed"),
    );
    let selection = &mut schematic.session.editor.selection;
    selection.select_only_component(part);
    selection.select_probe(900);

    assert_eq!(
        app.command_availability(DesignCommand::ArraySelection.key()),
        CommandAvailability::Disabled(
            "probes are not copied into an array; leave them out of the selection"
        )
    );

    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(part);
    assert_eq!(
        app.command_availability(DesignCommand::ArraySelection.key()),
        CommandAvailability::Available
    );
}

/// A replay with one resistor selected and Create array open over it.
fn create_array_open() -> Replay {
    let mut replay = Replay::of(RSpiceApp::test_instance());
    replay.app.state.workbench.activate(Workspace::Design);
    let schematic = &mut replay.app.state.model.schematic;
    let part = schematic.add_component(ComponentType::Resistor, Point::origin());
    schematic
        .session
        .editor
        .selection
        .select_only_component(part);
    replay
        .app
        .execute_command_key(DesignCommand::ArraySelection.key());
    replay
}

/// The count alone says whether the copies make a row, a column or a grid:
/// the form has no second control that could disagree with it, and a count
/// of 4 x 2 arms a rectangular array.
#[test]
fn the_count_alone_sets_a_row_or_a_grid() {
    let mut replay = create_array_open();
    let controls = replay.frames(Vec::new());
    assert!(
        !controls
            .iter()
            .any(|control| control.label == "Array" && control.role == Role::ComboBox),
        "a part's array offers no arrangement to choose"
    );
    assert_eq!(
        replay.app.state.design.array_selection.kind,
        SchematicArrayKind::Linear
    );
    replay.click("Count", Role::TextInput);
    replay.key(egui::Key::A, PRIMARY);
    replay.type_text("4x2");
    replay.click("Create", Role::Button);

    let draft = &replay.app.state.design.array_selection;
    assert!(draft.armed, "a 4 x 2 count arms the canvas");
    assert_eq!(draft.kind, SchematicArrayKind::Rectangular);
}

/// A pitch typed in millimetres places the array at once, the copies that
/// far apart, with no clicks on the canvas.
#[test]
fn a_typed_pitch_places_the_array_at_once() {
    let mut replay = create_array_open();
    replay.click("Count", Role::TextInput);
    replay.key(egui::Key::A, PRIMARY);
    replay.type_text("3x1");
    replay.click("Pitch (mm) X", Role::TextInput);
    replay.type_text("25");
    replay.click("Create", Role::Button);
    replay.frames(Vec::new());

    let state = &replay.app.state;
    assert!(
        !state.design.array_selection.armed,
        "nothing waits on the canvas: {:?}",
        state.prompt_text()
    );
    let mut xs: Vec<i32> = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .map(|part| part.pos.x)
        .collect();
    xs.sort_unstable();
    // Four drawing units to the millimetre.
    assert_eq!(xs, [0, 100, 200]);
}

/// Esc on the armed canvas goes back to the form as it was filled in; the
/// form opens next time with the count Create was last pressed with; and a
/// placement reports the copies it added and the members the array has.
#[test]
fn esc_keeps_the_form_and_create_reports_copies_and_members() {
    let mut replay = create_array_open();
    replay.click("Count", Role::TextInput);
    replay.key(egui::Key::A, PRIMARY);
    replay.type_text("3x1");
    replay.click("Create", Role::Button);
    assert!(replay.app.state.design.array_selection.armed);

    replay.key(egui::Key::Escape, egui::Modifiers::NONE);
    let draft = &replay.app.state.design.array_selection;
    assert!(draft.open && !draft.armed, "Esc went back to the form");
    assert_eq!(draft.count, "3\u{00d7}1");

    replay.click("Cancel", Role::Button);
    assert!(!replay.app.state.design.array_selection.open);
    replay
        .app
        .execute_command_key(DesignCommand::ArraySelection.key());
    assert_eq!(
        SchematicArrayCount::parse(&replay.app.state.design.array_selection.count).ok(),
        SchematicArrayCount::new(3, 1).ok(),
        "the form remembers its count"
    );

    replay.click("Pitch (mm) X", Role::TextInput);
    replay.type_text("25");
    replay.click("Create", Role::Button);
    replay.frames(Vec::new());
    assert_eq!(
        replay.app.state.prompt_text(),
        Some("Added 2 copies (3 members).")
    );
}

/// A placement the drawing refuses, copies too close to touch, comes back
/// to the form as it was filled in and says why there.
#[test]
fn a_refused_placement_comes_back_to_the_form() {
    let mut replay = create_array_open();
    replay.click("Pitch (mm) X", Role::TextInput);
    replay.type_text("10");
    replay.click("Create", Role::Button);
    replay.frames(Vec::new());

    let draft = &replay.app.state.design.array_selection;
    assert!(draft.open && !draft.armed, "the form is back");
    assert_eq!(draft.pitch_x, "10");
    assert!(
        draft
            .refusal
            .as_deref()
            .is_some_and(|refusal| refusal.starts_with("Array not created:")),
        "{:?}",
        draft.refusal
    );
    assert_eq!(
        replay.app.state.model.schematic.document().components.len(),
        1
    );
}
