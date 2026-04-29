use std::path::PathBuf;

use super::super::model::PdkSettingsDialogState;
use egui::{RichText, Ui};

/// Render the Discovered Files tab
pub(in crate::panels::pdk_settings_dialog) fn render_discovered_files_tab(
    ui: &mut Ui,
    state: &mut PdkSettingsDialogState,
) -> Option<PathBuf> {
    let mut load_file: Option<PathBuf> = None;

    ui.heading("Discovered Model Files");
    ui.add_space(4.0);

    ui.horizontal(|ui| {
        ui.label("Filter:");
        ui.add(
            egui::TextEdit::singleline(&mut state.file_filter)
                .desired_width(200.0)
                .hint_text("Search files..."),
        );

        if !state.file_filter.is_empty() && ui.small_button("✖").clicked() {
            state.file_filter.clear();
        }
    });

    ui.add_space(4.0);

    let filtered = state.filtered_files();

    if filtered.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label(
                RichText::new("No model files discovered.")
                    .color(egui::Color32::GRAY)
                    .italics(),
            );
            ui.add_space(8.0);
            ui.label(
                RichText::new("Add library paths and click 'Rescan' to discover files.")
                    .color(egui::Color32::GRAY)
                    .size(11.0),
            );
        });
    } else {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Type").strong().size(11.0));
            ui.add_space(40.0);
            ui.label(RichText::new("Path").strong().size(11.0));
        });
        ui.separator();

        for file in filtered {
            ui.horizontal(|ui| {
                let (type_color, type_text) = match file.file_type() {
                    "lib" => (egui::Color32::from_rgb(100, 180, 100), "LIB"),
                    "scs" => (egui::Color32::from_rgb(100, 150, 200), "SCS"),
                    "mod" => (egui::Color32::from_rgb(200, 150, 100), "MOD"),
                    "sp" | "cir" => (egui::Color32::from_rgb(150, 150, 150), "SP"),
                    _ => (egui::Color32::GRAY, "???"),
                };
                ui.label(
                    RichText::new(format!("[{}]", type_text))
                        .color(type_color)
                        .monospace()
                        .size(10.0),
                );

                let path_display = file.path_str();
                if ui.link(&path_display).clicked() {
                    load_file = Some(file.path.clone());
                }

                if !file.sections.is_empty() {
                    ui.label(
                        RichText::new(format!("({})", file.sections.join(", ")))
                            .color(egui::Color32::GRAY)
                            .size(10.0),
                    );
                }
            });
        }
    }

    load_file
}
