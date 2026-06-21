use crate::common::app::{AppState, ConfirmationAction, ConsoleMessage};
use crate::common::examples::{EXAMPLES, load_example_into_app};

pub(crate) fn request_load_named_example(state: &mut AppState, name: &str) -> bool {
    let Some(example) = EXAMPLES.iter().find(|example| example.name == name) else {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Example '{}' is not registered",
            name
        )));
        return false;
    };

    if state.schematic.is_dirty || state.workspace.any_dirty() {
        state
            .dialogs
            .confirmation_dialog
            .show_with_example(ConfirmationAction::OpenExample, example.name.to_owned());
        return true;
    }

    load_named_example(state, example.name)
}

pub(crate) fn load_named_example(state: &mut AppState, name: &str) -> bool {
    let Some(example) = EXAMPLES.iter().find(|example| example.name == name) else {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Example '{}' is not registered",
            name
        )));
        return false;
    };

    load_example_into_app(example.name, state);
    state.push_user_message(ConsoleMessage::info(format!(
        "Loaded example: {} ({})",
        example.name, example.category
    )));
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, Point, SimulationRun};
    use std::path::PathBuf;

    #[test]
    fn request_load_named_example_prompts_when_dirty_without_replacing_schematic() {
        let mut state = AppState::default();
        state.schematic.components.push(Component::new(
            99,
            ComponentType::Resistor,
            Point::new(10, 10),
        ));
        state.schematic.is_dirty = true;

        assert!(request_load_named_example(&mut state, "RC Lowpass Filter"));

        assert_eq!(state.schematic.components.len(), 1);
        assert_eq!(state.schematic.components[0].id, 99);
        assert!(
            state
                .dialogs
                .confirmation_dialog
                .is_showing(ConfirmationAction::OpenExample)
        );
        assert_eq!(
            state.dialogs.confirmation_dialog.pending_example.as_deref(),
            Some("RC Lowpass Filter")
        );
    }

    #[test]
    fn load_named_example_resets_document_identity_and_execution_context() {
        let mut state = AppState::default();
        state.schematic.current_file = Some(PathBuf::from("old_design.rsch"));
        state.schematic.read_only = true;
        state.workspace.netlist_source = Some("old manual deck\n.end\n".to_owned());
        state.simulation.netlist_content = "old generated deck\n.end\n".to_owned();
        state.simulation.runs.push(SimulationRun::new(1));
        state.simulation.active_run_idx = Some(0);

        assert!(load_named_example(&mut state, "Voltage Divider"));

        assert!(!state.schematic.components.is_empty());
        assert!(state.schematic.current_file.is_none());
        assert!(!state.schematic.read_only);
        assert!(state.workspace.netlist_source.is_none());
        assert!(state.simulation.netlist_content.is_empty());
        assert!(!state.simulation.has_results());
    }
}
