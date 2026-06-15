use crate::common::app::{AppState, ConsoleMessage};
use crate::common::examples::{EXAMPLES, load_example};

pub(crate) fn load_named_example(state: &mut AppState, name: &str) -> bool {
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
