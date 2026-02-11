use egui::Ui;

use crate::common::app::{AppState, ConsoleMessage};
use crate::common::examples::{load_example, EXAMPLES};

pub(super) fn render_examples_menu(ui: &mut Ui, state: &mut AppState) {
    for example in EXAMPLES {
        if ui
            .button(example.name)
            .on_hover_text(example.description)
            .clicked()
        {
            load_named_example(state, example.name);
            ui.close_menu();
        }
    }
}

pub(super) fn load_named_example(state: &mut AppState, name: &str) -> bool {
    let Some(example) = EXAMPLES.iter().find(|example| example.name == name) else {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Example '{}' is not registered",
            name
        )));
        return false;
    };

    load_example(example.name, &mut state.schematic);
    state.push_user_message(ConsoleMessage::info(format!(
        "Loaded example: {} ({})",
        example.name, example.category
    )));
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, Point};

    #[test]
    fn test_load_named_example_populates_schematic_and_logs_info_message() {
        let mut state = AppState::default();

        let loaded = load_named_example(&mut state, "Voltage Divider");

        assert!(loaded);
        assert!(
            !state.schematic.components.is_empty(),
            "expected example to populate schematic"
        );
        assert!(
            state.console_messages.iter().any(|msg| msg
                .message
                .contains("Loaded example: Voltage Divider (Basics)")),
            "expected example load info message"
        );
    }

    #[test]
    fn test_load_named_example_unknown_name_warns_and_preserves_schematic() {
        let mut state = AppState::default();
        state.schematic.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::new(10, 10),
        ));
        let component_count_before = state.schematic.components.len();

        let loaded = load_named_example(&mut state, "Unknown Example");

        assert!(!loaded);
        assert_eq!(
            state.schematic.components.len(),
            component_count_before,
            "unknown example should not clear the current schematic"
        );
        assert!(
            state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("is not registered")),
            "expected warning for unknown example name"
        );
    }
}
