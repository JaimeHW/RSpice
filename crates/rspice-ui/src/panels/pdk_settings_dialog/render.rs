use super::model::{PdkSettingsDialogResult, PdkSettingsDialogState, PdkSettingsTab};
use super::tabs::{
    render_discovered_files_tab, render_environment_tab, render_library_paths_tab,
    render_recent_files_tab,
};
use egui::{Context, RichText, Window};

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

    Window::new("PDK Settings")
        .resizable(true)
        .collapsible(false)
        .default_width(650.0)
        .default_height(500.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 6.0;

            ui.horizontal(|ui| {
                for tab in PdkSettingsTab::all() {
                    let selected = state.selected_tab == *tab;
                    if ui.selectable_label(selected, tab.display_name()).clicked() {
                        state.selected_tab = *tab;
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if state.scanning {
                        ui.spinner();
                        ui.label("Scanning...");
                    } else if ui.button("🔄 Rescan").clicked() {
                        should_rescan = true;
                    }
                });
            });

            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .max_height(380.0)
                .show(ui, |ui| match state.selected_tab {
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
                });

            ui.add_space(8.0);
            ui.separator();
            ui.horizontal(|ui| {
                let file_count = state.config.discovered_files().len();
                let path_count = state.config.library_paths().len();
                let enabled_count = state
                    .config
                    .library_paths()
                    .iter()
                    .filter(|p| p.enabled)
                    .count();

                ui.label(
                    RichText::new(format!(
                        "{} files discovered | {} paths ({} enabled)",
                        file_count, path_count, enabled_count
                    ))
                    .color(egui::Color32::GRAY)
                    .size(11.0),
                );

                if state.has_changes() {
                    ui.label(
                        RichText::new("• Modified")
                            .color(egui::Color32::YELLOW)
                            .size(11.0),
                    );
                }
            });

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Cancel").clicked() {
                        should_close = true;
                        result = PdkSettingsDialogResult::Cancelled;
                    }

                    let apply_enabled = state.has_changes();
                    if ui
                        .add_enabled(apply_enabled, egui::Button::new("Apply"))
                        .clicked()
                    {
                        let config = state.apply();
                        result = PdkSettingsDialogResult::Applied(config);
                        should_close = true;
                    }

                    if ui.button("OK").clicked() {
                        if state.has_changes() {
                            let config = state.apply();
                            result = PdkSettingsDialogResult::Applied(config);
                        }
                        should_close = true;
                    }
                });
            });
        });

    if should_rescan {
        state.rescan();
    }

    if should_close {
        state.close();
    }

    result
}
