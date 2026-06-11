use std::path::PathBuf;

use super::super::model::PdkSettingsDialogState;
use crate::ui::tokens::Tokens;
use egui::{RichText, Ui};

/// Render the Discovered Files tab
pub(in crate::panels::pdk_settings_dialog) fn render_discovered_files_tab(
    ui: &mut Ui,
    state: &mut PdkSettingsDialogState,
) -> Option<PathBuf> {
    let c = Tokens::get(ui.ctx()).color;
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

        if !state.file_filter.is_empty() && ui.small_button("clear").clicked() {
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
                    .color(c.text_dim)
                    .italics(),
            );
            ui.add_space(8.0);
            ui.label(
                RichText::new("Add library paths and click 'Rescan' to discover files.")
                    .color(c.text_faint)
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
                // File-type badges draw from the categorical trace palette —
                // theme-aware in both modes, unlike hand-picked RGB.
                let (type_color, type_text) = match file.file_type() {
                    "lib" => (c.traces[2], "LIB"),
                    "scs" => (c.traces[1], "SCS"),
                    "mod" => (c.traces[3], "MOD"),
                    "sp" | "cir" => (c.text_dim, "SP"),
                    _ => (c.text_faint, "???"),
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
                            .color(c.text_faint)
                            .size(10.0),
                    );
                }
            });
        }
    }

    load_file
}
