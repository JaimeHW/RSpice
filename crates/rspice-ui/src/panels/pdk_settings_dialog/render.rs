//! PDK Settings on the modal primitive: underline tabs for paths,
//! environment, discovered and recent files; footer per the dialog
//! grammar with a live summary hint.

use super::model::{PdkSettingsDialogResult, PdkSettingsDialogState, PdkSettingsTab};
use super::tabs::{
    render_discovered_files_tab, render_environment_tab, render_library_paths_tab,
    render_recent_files_tab,
};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize, dialog_tabs};
use egui::Context;

/// Render the PDK Settings dialog
///
/// Returns the dialog result indicating user action.
pub fn render_pdk_settings_dialog(
    ctx: &Context,
    state: &mut PdkSettingsDialogState,
) -> PdkSettingsDialogResult {
    if !state.open {
        return PdkSettingsDialogResult::None;
    }

    let mut result = PdkSettingsDialogResult::None;
    let mut should_close = false;
    let mut should_rescan = false;

    let file_count = state.config.discovered_files().len();
    let path_count = state.config.library_paths().len();
    let enabled_count = state
        .config
        .library_paths()
        .iter()
        .filter(|p| p.enabled)
        .count();
    let hint = if state.has_changes() {
        format!("{file_count} files · {enabled_count}/{path_count} paths · modified")
    } else {
        format!("{file_count} files · {enabled_count}/{path_count} paths")
    };

    let choice = Dialog::new("PDK", "PDK settings", "OK")
        .size(DialogSize::Lg)
        .secondary("Apply")
        .ghost("Cancel")
        .hint(&hint)
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 4.0;

            // Rescan action above the tab strip.
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if state.scanning {
                        ui.spinner();
                    } else if Button::new("Rescan paths").ghost().show(ui).clicked() {
                        should_rescan = true;
                    }
                });
            });

            let tabs: Vec<&str> = PdkSettingsTab::all()
                .iter()
                .map(|tab| tab.display_name())
                .collect();
            let mut active = PdkSettingsTab::all()
                .iter()
                .position(|tab| *tab == state.selected_tab)
                .unwrap_or(0);
            dialog_tabs(ui, &tabs, &mut active);
            state.selected_tab = PdkSettingsTab::all()[active.min(tabs.len() - 1)];

            match state.selected_tab {
                PdkSettingsTab::LibraryPaths => {
                    render_library_paths_tab(ui, state);
                }
                PdkSettingsTab::Environment => {
                    render_environment_tab(ui, state);
                }
                PdkSettingsTab::DiscoveredFiles => {
                    if let Some(path) = render_discovered_files_tab(ui, state) {
                        result = PdkSettingsDialogResult::LoadFile(path);
                    }
                }
                PdkSettingsTab::RecentFiles => {
                    if let Some(path) = render_recent_files_tab(ui, state) {
                        result = PdkSettingsDialogResult::LoadFile(path);
                    }
                }
            }
        });

    match choice {
        DialogChoice::Primary => {
            if state.has_changes() {
                let config = state.apply();
                result = PdkSettingsDialogResult::Applied(config);
            }
            should_close = true;
        }
        DialogChoice::Secondary => {
            if state.has_changes() {
                let config = state.apply();
                result = PdkSettingsDialogResult::Applied(config);
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            should_close = true;
            result = PdkSettingsDialogResult::Cancelled;
        }
        DialogChoice::None => {}
    }

    if should_rescan {
        state.rescan();
    }

    if should_close {
        state.close();
    }

    result
}
