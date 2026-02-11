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
            let is_active = state.active_viewer == *viewer;
            let label = if is_active {
                format!("âœ“ {}", viewer.name())
            } else {
                format!("  {}", viewer.name())
            };

            if ui.button(&label).clicked() {
                activate_specialized_viewer(state, *viewer);
                ui.close_menu();
            }
        }
    });
}

fn checkmark_label(enabled: bool, label: &str) -> String {
    if enabled {
        format!("âœ“ {}", label)
    } else {
        format!("  {}", label)
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
    state.active_viewer = viewer;
    state.panels.bottom_panel = true;
    state.panels.active_bottom_tab = BottomPanelTab::Waveform;
    state.push_user_message(ConsoleMessage::info(format!(
        "Switched to {} viewer",
        viewer.name()
    )));
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(state.active_viewer, ActiveViewer::EyeDiagram);
        assert!(state.panels.bottom_panel);
        assert_eq!(state.panels.active_bottom_tab, BottomPanelTab::Waveform);
        assert!(
            state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("Switched to Eye Diagram viewer")),
            "expected viewer-switch message"
        );
    }
}
