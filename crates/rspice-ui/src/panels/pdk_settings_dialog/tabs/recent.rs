use std::path::PathBuf;

use super::super::model::PdkSettingsDialogState;
use crate::ui::tokens::Tokens;
use egui::{RichText, Ui};

/// Render the Recent Files tab
pub(in crate::panels::pdk_settings_dialog) fn render_recent_files_tab(
    ui: &mut Ui,
    state: &mut PdkSettingsDialogState,
) -> Option<PathBuf> {
    let c = Tokens::get(ui.ctx()).color;
    let mut load_file: Option<PathBuf> = None;

    ui.heading("Recent Files");
    ui.add_space(4.0);

    let recent = state.config.recent_files();

    if recent.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label(
                RichText::new("No recent files.")
                    .color(c.text_dim)
                    .italics(),
            );
        });
    } else {
        let mut clear_all = false;

        for file_path in recent.iter() {
            ui.horizontal(|ui| {
                if ui.link(file_path).clicked() {
                    load_file = Some(PathBuf::from(file_path));
                }

                if !std::path::Path::new(file_path).exists() {
                    ui.label(
                        RichText::new("(not found)")
                            .color(c.err)
                            .size(10.0),
                    );
                }
            });
        }

        ui.add_space(8.0);
        if ui.button("Clear Recent Files").clicked() {
            clear_all = true;
        }

        if clear_all {
            state.config.clear_recent_files();
        }
    }

    load_file
}
