//! Preferences — application-level settings on the VOLTA modal grammar.
//!
//! Every row edits live state directly (theme, density, and grid changes
//! are safe to preview), so the dialog needs only a Close action; nothing
//! here is a draft awaiting an Apply.

use egui::Context;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize, check_row, choice_row, kv_row};
use crate::ui::{Density, Direction, Mode};

use super::RSpiceApp;

impl RSpiceApp {
    pub(super) fn render_preferences_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.preferences_open {
            return;
        }

        let state = &mut self.state;
        let choice = Dialog::new("Application", "Preferences", "Close")
            .size(DialogSize::Md)
            .hint("changes apply immediately")
            .show(ctx, |ui| {
                section_label(ui, "Canvas");

                let pitches = ["5", "10", "20"];
                let mut pitch_index = match state.schematic.grid_size {
                    5 => 0,
                    20 => 2,
                    _ => 1,
                };
                if choice_row(ui, "Grid pitch", &pitches, &mut pitch_index) {
                    state.schematic.grid_size = [5, 10, 20][pitch_index];
                    state.schematic.is_dirty = true;
                }

                let grid_labels: Vec<&str> = crate::shell::GridStyle::ALL
                    .iter()
                    .map(|style| style.label())
                    .collect();
                let mut grid_index = crate::shell::GridStyle::ALL
                    .iter()
                    .position(|style| *style == state.shell.grid)
                    .unwrap_or(0);
                if choice_row(ui, "Grid", &grid_labels, &mut grid_index) {
                    state.shell.grid = crate::shell::GridStyle::ALL[grid_index];
                }

                ui.add_space(tokens::SP_3);
                section_label(ui, "Appearance");

                let direction_labels: Vec<&str> =
                    Direction::ALL.iter().map(|d| d.label()).collect();
                let mut direction_index = Direction::ALL
                    .iter()
                    .position(|d| *d == state.shell.theme.direction)
                    .unwrap_or(0);
                if choice_row(ui, "Direction", &direction_labels, &mut direction_index) {
                    state.shell.theme.direction = Direction::ALL[direction_index];
                }

                let mode_labels: Vec<&str> = Mode::ALL.iter().map(|m| m.label()).collect();
                let mut mode_index = Mode::ALL
                    .iter()
                    .position(|m| *m == state.shell.theme.mode)
                    .unwrap_or(0);
                if choice_row(ui, "Mode", &mode_labels, &mut mode_index) {
                    state.shell.theme.mode = Mode::ALL[mode_index];
                }

                let density_labels: Vec<&str> = Density::ALL.iter().map(|d| d.label()).collect();
                let mut density_index = Density::ALL
                    .iter()
                    .position(|d| *d == state.shell.theme.density)
                    .unwrap_or(0);
                if choice_row(ui, "Density", &density_labels, &mut density_index) {
                    state.shell.theme.density = Density::ALL[density_index];
                }
                check_row(
                    ui,
                    "Colorblind-safe trace colors",
                    &mut state.shell.theme.colorblind_traces,
                );

                ui.add_space(tokens::SP_3);
                section_label(ui, "Files");

                let count = state.recent_files.len();
                let label = match count {
                    0 => "none".to_owned(),
                    1 => "1 entry".to_owned(),
                    n => format!("{n} entries"),
                };
                kv_row(ui, "Recent files", &label);
                if count > 0 && Button::new("Clear recent files").show(ui).clicked() {
                    state.recent_files.clear();
                }
            });

        if choice != DialogChoice::None {
            self.state.dialogs.preferences_open = false;
        }
    }
}

/// A mono uppercase section header inside the dialog body.
fn section_label(ui: &mut egui::Ui, title: &str) {
    let t = Tokens::get(ui.ctx());
    let mut job = egui::text::LayoutJob::default();
    job.append(
        &title.to_uppercase(),
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Medium),
            color: t.color.text_dim,
            extra_letter_spacing: 0.08 * tokens::FS_0,
            ..Default::default()
        },
    );
    ui.label(job);
    ui.add_space(2.0);
}
