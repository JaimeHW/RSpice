//! What the library browser and the definition inspector are held to.
//!
//! Both docks are gated here because they are read together: a count in the
//! browser's footer and a count in the inspector's Adopters heading are the
//! same walk of the same design, and a test that only ever opened one of them
//! would not notice the two disagreeing.

use super::*;

use crate::state::ComponentType;
use crate::state::stimulus_library::fixtures;
use crate::workbench::app::actions::stimulus as stimulus_verbs;
use crate::workbench::state::PURPOSE_FIELD;
use crate::workbench::{AppState, RSpiceApp};

/// The dock's own minimum, at the fit the workspace is held to.
pub(crate) const NAVIGATOR: egui::Vec2 = egui::vec2(228.0, 640.0);

/// Whether the console holds a user line containing `fragment`.
fn prompt_says(state: &AppState, fragment: &str) -> bool {
    state
        .prompt_text()
        .is_some_and(|prompt| prompt.contains(fragment))
}

/// The mockup's fourteen definitions, with nothing adopted, in the workspace
/// whose navigator lists them.
pub(crate) fn library(state: &mut AppState) {
    state.model.workspace.content.stimulus_library = fixtures::library();
    state.workbench.activate(Workspace::Stimulus);
    state.design.selected_stimulus_definition = Some("sensor_diff_1k".to_owned());
}

/// Every row's three tracks stay positive at the dock's own minimum, so a name
/// is elided rather than painted over the mini or under the revision.
#[test]
fn every_row_track_survives_the_navigator_minimum() {
    let name_track = NAVIGATOR.x
        - row::INDENT
        - crate::properties::source_preview::MINI_SIZE.x
        - row::TEXT_GAP
        - row::RIGHT_INSET
        - row::DOT_TRACK;
    assert!(
        name_track >= 100.0,
        "a name row has only {name_track} points at the dock's minimum"
    );
    assert!(row::ROW_HEIGHT >= crate::properties::source_preview::MINI_SIZE.y + 8.0);
    assert!(SCOPE_BAND + FOOTER_BAND + row::ROW_HEIGHT * 2.0 < NAVIGATOR.y);
}

/// The purpose writes through the draft, so it is undoable and dirties the
/// definition exactly as a waveform field does.
#[test]
fn editing_the_purpose_goes_through_the_draft_and_is_undoable() {
    let mut app = RSpiceApp::test_instance();
    library(&mut app.state);
    stimulus_verbs::edit_field(
        app.state.design.selected_stimulus_definition.as_deref(),
        &mut app.state.design.stimulus_editor,
        &app.state.model,
        PURPOSE_FIELD,
        "differential sensor drive",
    );

    assert!(
        app.state.design.stimulus_editor.is_dirty("sensor_diff_1k"),
        "the purpose edit did not dirty the definition"
    );
    assert_eq!(
        app.state
            .design
            .stimulus_editor
            .draft("sensor_diff_1k")
            .map(|draft| draft.working().purpose.as_str()),
        Some("differential sensor drive")
    );
    assert!(stimulus_verbs::undo_draft(
        app.state.design.selected_stimulus_definition.as_deref(),
        &mut app.state.design.stimulus_editor,
        &app.state.model,
    ));
    assert_eq!(
        app.state
            .design
            .stimulus_editor
            .draft("sensor_diff_1k")
            .map(|draft| draft.working().purpose.as_str()),
        Some("")
    );
    // The library itself is untouched until Apply publishes, exactly as it is
    // for every other field.
    assert!(
        app.state
            .model
            .workspace
            .content
            .stimulus_library
            .get("sensor_diff_1k")
            .expect("held")
            .purpose
            .is_empty()
    );
}

/// Place arms the cursor with the library's saved revision, and says so when
/// the editor is holding a draft it is not placing.
#[test]
fn place_arms_the_saved_revision_and_warns_about_an_unapplied_draft() {
    use crate::state::Tool;

    let mut app = RSpiceApp::test_instance();
    library(&mut app.state);
    stimulus_verbs::place_selected_definition(
        app.state.design.selected_stimulus_definition.as_deref(),
        &app.state.design.stimulus_editor,
        &mut app.state.model,
        &mut app.state.workbench,
    );

    assert_eq!(
        app.state.model.schematic.session.editor.tool,
        Tool::Place(ComponentType::VoltageSourceSin)
    );
    let armed = app
        .state
        .model
        .schematic
        .session
        .editor
        .pending_stimulus
        .clone()
        .expect("a definition is armed");
    assert_eq!(armed.definition(), "sensor_diff_1k");
    assert_eq!(armed.revision(), 1);
    assert!(
        !prompt_says(&app.state, "apply the draft first"),
        "a clean definition was warned about"
    );

    let saved = app
        .state
        .model
        .workspace
        .content
        .stimulus_library
        .get("sensor_diff_1k")
        .cloned()
        .expect("held");
    app.state
        .design
        .stimulus_editor
        .draft_for(&saved)
        .edit(|working| working.params = "va=9m freq=1k".to_owned());
    stimulus_verbs::place_selected_definition(
        app.state.design.selected_stimulus_definition.as_deref(),
        &app.state.design.stimulus_editor,
        &mut app.state.model,
        &mut app.state.workbench,
    );
    assert!(
        prompt_says(
            &app.state,
            "Placement copies the saved r1 of 'sensor_diff_1k'; apply the draft first"
        ),
        "the reader was not told which revision the pointer is carrying"
    );
    assert_eq!(
        app.state
            .model
            .schematic
            .session
            .editor
            .pending_stimulus
            .as_ref()
            .map(crate::state::PendingStimulusPlacement::revision),
        Some(1),
        "the draft was armed instead of the published revision"
    );
}
