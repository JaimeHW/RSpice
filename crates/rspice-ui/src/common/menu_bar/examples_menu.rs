use egui::Ui;

use crate::common::app::{AppState, ConsoleMessage};
use crate::common::examples::{EXAMPLES, load_example};

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

