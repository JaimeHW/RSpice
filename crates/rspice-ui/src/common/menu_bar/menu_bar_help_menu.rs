use egui::Ui;

use crate::common::app::{AppState, ConsoleMessage};

const DOC_REFERENCES: [(&str, &str); 4] = [
    ("User Guide", "docs/user_guide.md"),
    ("SPICE Reference", "docs/spice_reference.md"),
    ("Analysis Guide", "docs/analysis_guide.md"),
    ("Model Library", "docs/models.md"),
];

pub(super) fn render_help_menu(ui: &mut Ui, state: &mut AppState) {
    if ui.button("Keyboard Shortcuts    F1").clicked() {
        open_keyboard_shortcuts(state);
        ui.close_menu();
    }

    ui.separator();

    ui.menu_button("Documentation", |ui| {
        for &(title, path) in &DOC_REFERENCES {
            if ui.button(title).clicked() {
                open_documentation_reference(state, title, path);
                ui.close_menu();
            }
        }
    });

    ui.menu_button("Examples", |ui| {
        for shortcut in [
            "RC Low-Pass Filter",
            "Inverter Chain",
            "Operational Amplifier",
            "PLL Circuit",
        ] {
            if ui.button(shortcut).clicked() {
                load_help_example_shortcut(state, shortcut);
                ui.close_menu();
            }
        }
    });

    ui.separator();

    if ui.button("Check for Updates...").clicked() {
        check_for_updates(state);
        ui.close_menu();
    }

    if ui.button("Report Issue...").clicked() {
        open_issue_report_link(state);
        ui.close_menu();
    }

    ui.separator();

    if ui.button("About RSpice").clicked() {
        open_about_dialog(state);
        ui.close_menu();
    }
}

fn open_keyboard_shortcuts(state: &mut AppState) {
    state.dialogs.shortcuts_help = true;
}

fn open_documentation_reference(state: &mut AppState, title: &str, relative_path: &str) {
    state.push_user_message(ConsoleMessage::info(format!(
        "{}: See {}",
        title, relative_path
    )));
}

fn load_help_example_shortcut(state: &mut AppState, shortcut: &str) -> bool {
    let mapped_example = match shortcut {
        "RC Low-Pass Filter" => Some("RC Lowpass Filter"),
        "Inverter Chain" => Some("CMOS Inverter"),
        "Operational Amplifier" => Some("Opamp Inverting Amplifier"),
        "PLL Circuit" => None,
        _ => None,
    };

    let Some(mapped_example) = mapped_example else {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Example '{}' is not yet available",
            shortcut
        )));
        return false;
    };

    super::menu_bar_examples_menu::load_named_example(state, mapped_example)
}

fn check_for_updates(state: &mut AppState) {
    state.push_user_message(ConsoleMessage::info("You are running the latest version"));
}

fn open_issue_report_link(state: &mut AppState) {
    state.push_user_message(ConsoleMessage::info(
        "Please report issues at: https://github.com/rspice/issues",
    ));
}

fn open_about_dialog(state: &mut AppState) {
    state.dialogs.about = true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, Point};

    #[test]
    fn test_open_keyboard_shortcuts_sets_dialog_visibility() {
        let mut state = AppState::default();
        state.dialogs.shortcuts_help = false;

        open_keyboard_shortcuts(&mut state);

        assert!(state.dialogs.shortcuts_help);
    }

    #[test]
    fn test_open_documentation_reference_logs_help_message() {
        let mut state = AppState::default();

        open_documentation_reference(&mut state, "User Guide", "docs/user_guide.md");

        assert!(
            state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("User Guide: See docs/user_guide.md")),
            "expected documentation reference message"
        );
    }

    #[test]
    fn test_load_help_example_shortcut_maps_alias_to_registered_example() {
        let mut state = AppState::default();

        let loaded = load_help_example_shortcut(&mut state, "Inverter Chain");

        assert!(loaded);
        assert!(
            !state.schematic.components.is_empty(),
            "expected mapped example to populate schematic"
        );
        assert!(
            state.console_messages.iter().any(|msg| msg
                .message
                .contains("Loaded example: CMOS Inverter (Digital)")),
            "expected mapped example load message"
        );
    }

    #[test]
    fn test_load_help_example_shortcut_pll_warns_without_clearing_schematic() {
        let mut state = AppState::default();
        state.schematic.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::new(5, 5),
        ));
        let component_count_before = state.schematic.components.len();

        let loaded = load_help_example_shortcut(&mut state, "PLL Circuit");

        assert!(!loaded);
        assert_eq!(
            state.schematic.components.len(),
            component_count_before,
            "unimplemented shortcut should not change the current schematic"
        );
        assert!(
            state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("not yet available")),
            "expected warning for unavailable shortcut"
        );
    }

    #[test]
    fn test_open_about_dialog_sets_about_visibility() {
        let mut state = AppState::default();
        state.dialogs.about = false;

        open_about_dialog(&mut state);

        assert!(state.dialogs.about);
    }
}
