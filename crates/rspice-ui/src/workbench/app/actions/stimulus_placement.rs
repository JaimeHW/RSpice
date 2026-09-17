//! Arming the placement cursor with a stimulus definition.
//!
//! The ADS sources-palette idiom: a definition is a part, so picking one arms
//! the cursor and the next click on the canvas draws an instance that has
//! already adopted it. Every door onto that — the Component shelf's Stimulus
//! library section, the Stimulus Library workspace, the schematic's own menu —
//! comes through here, so there is one refusal vocabulary and one console line
//! for all of them.
//!
//! The armed copy is the library's **saved** revision. A draft being edited in
//! the Stimulus Library workspace is not a revision anything can adopt: an
//! instance stamped from one would read `modified` against a definition the
//! library never published.

use crate::diagnostics::ConsoleMessage;
use crate::state::{PendingStimulusPlacement, Tool};
use crate::workbench::app_state::AppState;
use crate::workbench::state::Workspace;

/// Arm the next placement with the saved revision of one definition.
///
/// Returns the refusal sentence when there is nothing to arm: the caller shows
/// it where the reader asked, rather than this pushing a console line for a
/// failure the reader is standing in front of.
pub(crate) fn arm_placement_from_definition(
    state: &mut AppState,
    name: &str,
) -> Result<(), String> {
    if state.schematic_edit_read_only() {
        return Err(format!(
            "The active schematic view is read-only, so '{name}' cannot be placed on it. Reopen \
             the view in an editable context first."
        ));
    }
    let Some(definition) = state.workspace.stimulus_library.get(name) else {
        return Err(format!(
            "This project defines no '{name}', so there is no stimulus to place."
        ));
    };
    let placement = PendingStimulusPlacement::of(definition);
    let reference = definition.kind().letter().to_owned();
    let armed = format!("{} r{}", placement.definition(), placement.revision());

    state
        .schematic
        .arm_tool(Tool::Place(placement.component_type));
    state.schematic.pending_stimulus = Some(placement);
    state.workbench.activate(Workspace::Design);
    state.push_user_message(ConsoleMessage::info(format!(
        "{reference}\u{2026} from {armed} follows the pointer; click to place, Esc cancels."
    )));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::stimulus_library::definition::StimulusDefinition;
    use crate::state::stimulus_library::provenance::ProvenanceState;
    use crate::state::{ComponentType, Point};
    use crate::workbench::RSpiceApp;

    fn app_with_definition() -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        let mut definition =
            StimulusDefinition::new("sensor_drive", ComponentType::VoltageSourceSin)
                .expect("definition");
        definition.value = "0".to_owned();
        definition.params = "va=3m freq=1k".to_owned();
        app.state
            .workspace
            .stimulus_library
            .insert(definition)
            .expect("insert");
        app
    }

    #[test]
    fn arming_carries_the_saved_revision_onto_the_next_placement() {
        let mut app = app_with_definition();
        arm_placement_from_definition(&mut app.state, "sensor_drive").expect("armed");

        assert_eq!(
            app.state.schematic.tool,
            Tool::Place(ComponentType::VoltageSourceSin)
        );
        let armed = app
            .state
            .schematic
            .pending_stimulus
            .clone()
            .expect("a definition is armed");
        assert_eq!(armed.definition(), "sensor_drive");
        assert_eq!(armed.revision(), 1);

        let id = app
            .state
            .schematic
            .add_stimulus_component(&armed, Point::new(6, 6));
        let component = app
            .state
            .schematic
            .components
            .iter()
            .find(|component| component.id == id)
            .expect("placed");
        assert_eq!(
            app.state
                .workspace
                .stimulus_library
                .provenance_state(component),
            ProvenanceState::Adopted { revision: 1 }
        );
    }

    #[test]
    fn arming_refuses_a_definition_the_library_does_not_hold() {
        let mut app = app_with_definition();
        let refusal =
            arm_placement_from_definition(&mut app.state, "absent").expect_err("nothing to arm");
        assert!(refusal.contains("defines no 'absent'"), "{refusal}");
        assert!(app.state.schematic.pending_stimulus.is_none());
        assert_eq!(app.state.schematic.tool, Tool::Select);
    }

    #[test]
    fn arming_refuses_a_read_only_view() {
        let mut app = app_with_definition();
        app.state.schematic.read_only = true;
        let refusal = arm_placement_from_definition(&mut app.state, "sensor_drive")
            .expect_err("a read-only sheet takes no placement");
        assert!(refusal.contains("read-only"), "{refusal}");
        assert!(app.state.schematic.pending_stimulus.is_none());
    }
}
