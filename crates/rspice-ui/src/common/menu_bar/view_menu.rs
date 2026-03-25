use egui::Ui;

use crate::common::app::{AppState, BottomPanelTab, ConsoleMessage};
use crate::viewers::ActiveViewer;

pub(super) fn render_view_menu(ui: &mut Ui, state: &mut AppState) {
    if ui.button("Zoom In     Ctrl++").clicked() {
        state.schematic.zoom = (state.schematic.zoom * 1.25).min(4.0);
        ui.close_menu();
    }
    if ui.button("Zoom Out    Ctrl+-").clicked() {
        state.schematic.zoom = (state.schematic.zoom / 1.25).max(0.25);
        ui.close_menu();
    }
    if ui.button("Zoom to Fit").clicked() {
        state.schematic.zoom_to_fit(800.0, 600.0);
        ui.close_menu();
    }
    if ui.button("Zoom 100%   Ctrl+0").clicked() {
        state.schematic.zoom = 1.0;
        ui.close_menu();
    }

    ui.separator();

    if ui
        .button(checkmark_label(
            state.panels.project_browser,
            "Library Browser",
        ))
        .clicked()
    {
        state.panels.project_browser = !state.panels.project_browser;
        ui.close_menu();
    }

    if ui
        .button(checkmark_label(
            state.panels.bottom_panel && state.panels.active_bottom_tab == BottomPanelTab::Waveform,
            "Waveform Viewer",
        ))
        .clicked()
    {
        toggle_bottom_tab_panel(state, BottomPanelTab::Waveform);
        ui.close_menu();
    }

    if ui
        .button(
            if state.panels.bottom_panel && state.panels.active_bottom_tab == BottomPanelTab::Log {
                "[x] Log".to_string()
            } else {
                "[ ] Log".to_string()
            },
        )
        .clicked()
    {
        toggle_bottom_tab_panel(state, BottomPanelTab::Log);
        ui.close_menu();
    }

    if ui
        .button(checkmark_label(state.panels.properties, "Properties"))
        .clicked()
    {
        state.panels.properties = !state.panels.properties;
        ui.close_menu();
    }

    if ui
        .button(checkmark_label(
            state.panels.signal_browser,
            "Signal Browser",
        ))
        .clicked()
    {
        state.panels.signal_browser = !state.panels.signal_browser;
        ui.close_menu();
    }

    if ui
        .button(checkmark_label(
            state.panels.script_console,
            "Automation Console",
        ))
        .clicked()
    {
        state.panels.script_console = !state.panels.script_console;
        ui.close_menu();
    }

    ui.separator();

    ui.menu_button("Specialized Viewers", |ui| {
        for viewer in ActiveViewer::all() {
            let is_active = state.active_viewer() == *viewer;
            let capability = state.viewer_capability(*viewer);
            let enabled = capability.available;
            let label = checkmark_label(is_active, viewer.name());

            let mut response = ui.add_enabled(enabled, egui::Button::new(&label));
            if !enabled {
                response = response.on_hover_text(capability.reason);
            }

            if response.clicked() {
                activate_specialized_viewer(state, *viewer);
                ui.close_menu();
            }
        }
    });
}

fn checkmark_label(enabled: bool, label: &str) -> String {
    if enabled {
        format!("[x] {}", label)
    } else {
        format!("[ ] {}", label)
    }
}

fn toggle_bottom_tab_panel(state: &mut AppState, tab: BottomPanelTab) {
    if state.panels.bottom_panel && state.panels.active_bottom_tab == tab {
        state.panels.bottom_panel = false;
    } else {
        state.panels.bottom_panel = true;
        state.panels.active_bottom_tab = tab;
    }
}

fn activate_specialized_viewer(state: &mut AppState, viewer: ActiveViewer) {
    let opened = state.open_viewer_in_tab(viewer, BottomPanelTab::Waveform);
    state.push_user_message(ConsoleMessage::info(format!(
        "Switched to {} viewer",
        opened.name()
    )));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed_eye_data(state: &mut AppState) {
        let mut eye = crate::analysis::eye_diagram::data::EyeData::default();
        eye.add_trace(crate::analysis::eye_diagram::data::EyeTrace::new(
            vec![0.0, 1.0],
            vec![0.0, 1.0],
        ));
        state.analysis.eye_diagram_state.load_data(eye);
    }

    #[test]
    fn test_toggle_bottom_tab_panel_hides_when_same_tab_is_active() {
        let mut state = AppState::default();
        state.panels.bottom_panel = true;
        state.panels.active_bottom_tab = BottomPanelTab::Waveform;

        toggle_bottom_tab_panel(&mut state, BottomPanelTab::Waveform);

        assert!(!state.panels.bottom_panel);
        assert_eq!(state.panels.active_bottom_tab, BottomPanelTab::Waveform);
    }

    #[test]
    fn test_toggle_bottom_tab_panel_activates_tab_when_hidden() {
        let mut state = AppState::default();
        state.panels.bottom_panel = false;
        state.panels.active_bottom_tab = BottomPanelTab::Log;

        toggle_bottom_tab_panel(&mut state, BottomPanelTab::Waveform);

        assert!(state.panels.bottom_panel);
        assert_eq!(state.panels.active_bottom_tab, BottomPanelTab::Waveform);
    }

    #[test]
    fn test_activate_specialized_viewer_sets_waveform_tab_and_logs_message() {
        let mut state = AppState::default();

        activate_specialized_viewer(&mut state, ActiveViewer::EyeDiagram);

        assert_eq!(state.active_viewer(), ActiveViewer::Waveform);
        assert!(state.panels.bottom_panel);
        assert_eq!(state.panels.active_bottom_tab, BottomPanelTab::Waveform);
        assert!(
            state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("Switched to Waveform viewer")),
            "expected viewer-switch message"
        );
    }

    #[test]
    fn test_activate_specialized_viewer_opens_requested_viewer_when_available() {
        let mut state = AppState::default();
        seed_eye_data(&mut state);

        activate_specialized_viewer(&mut state, ActiveViewer::EyeDiagram);

        assert_eq!(state.active_viewer(), ActiveViewer::EyeDiagram);
        assert!(
            state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("Switched to Eye Diagram viewer"))
        );
    }

    #[test]
    fn test_checkmark_label_formats_ascii_state_prefixes() {
        assert_eq!(
            checkmark_label(true, "Signal Browser"),
            "[x] Signal Browser"
        );
        assert_eq!(
            checkmark_label(false, "Signal Browser"),
            "[ ] Signal Browser"
        );
    }
}
