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

