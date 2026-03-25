use egui::Context;

use super::{
    RSpiceApp,
    app_shortcuts::{ShortcutCategory, ShortcutCommand},
};

fn shortcut_help_row(command: ShortcutCommand) -> (&'static str, &'static str) {
    (command.shortcut_string(), command.display_name())
}

#[cfg(test)]
fn total_shortcut_help_rows() -> usize {
    ShortcutCategory::ALL
        .iter()
        .map(|category| category.commands().len())
        .sum()
}

impl RSpiceApp {
    pub(super) fn render_about_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.about {
            return;
        }

        egui::Window::new("About RSpice")
            .collapsible(false)
            .resizable(false)
            .default_width(300.0)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.heading("RSpice");
                ui.label("Commercial-grade SPICE simulator");
                ui.add_space(10.0);
                ui.label("Version 0.1.0");
                ui.add_space(20.0);
                if ui.button("Close").clicked() {
                    self.state.dialogs.about = false;
                }
            });
    }

    pub(super) fn render_waveform_calculator_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.waveform_calculator_dialog {
            return;
        }

        egui::Window::new("Calculator")
            .open(&mut self.state.dialogs.waveform_calculator_dialog)
            .default_width(400.0)
            .show(ctx, |ui| {
                self.state.calculator_panel.show(ui, &self.state.simulation);
            });
    }

    pub(super) fn render_shortcuts_help_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.shortcuts_help {
            return;
        }

        let mut close_requested = false;

        egui::Window::new("Keyboard Shortcuts")
            .collapsible(false)
            .resizable(true)
            .default_width(560.0)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("shortcuts_help_scroll")
                    .max_height(420.0)
                    .show(ui, |ui| {
                        for category in ShortcutCategory::ALL {
                            ui.strong(category.display_name());
                            ui.add_space(4.0);

                            egui::Grid::new(format!("shortcuts_grid_{:?}", category))
                                .num_columns(2)
                                .spacing([40.0, 6.0])
                                .show(ui, |ui| {
                                    for command in category.commands() {
                                        let (shortcut, display_name) = shortcut_help_row(*command);
                                        ui.label(shortcut);
                                        ui.label(display_name);
                                        ui.end_row();
                                    }
                                });

                            ui.add_space(8.0);
                        }
                    });

                ui.separator();
                ui.add_space(6.0);
                if ui.button("Close").clicked() {
                    close_requested = true;
                }
            });

        if close_requested {
            self.state.dialogs.shortcuts_help = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_total_shortcut_help_rows_matches_registry() {
        let expected: usize = ShortcutCategory::ALL
            .iter()
            .map(|category| category.commands().len())
            .sum();
        assert_eq!(total_shortcut_help_rows(), expected);
        assert!(total_shortcut_help_rows() > 0);
    }

    #[test]
    fn test_shortcut_help_rows_have_non_empty_text() {
        for category in ShortcutCategory::ALL {
            for command in category.commands() {
                let (shortcut, display_name) = shortcut_help_row(*command);
                assert!(!shortcut.trim().is_empty());
                assert!(!display_name.trim().is_empty());
            }
        }
    }

    #[test]
    fn test_shortcut_help_commands_are_unique_across_categories() {
        let mut seen = HashSet::new();
        for category in ShortcutCategory::ALL {
            for command in category.commands() {
                assert!(
                    seen.insert(*command),
                    "shortcut command {:?} appears in multiple categories",
                    command
                );
            }
        }
        assert_eq!(seen.len(), total_shortcut_help_rows());
    }
}
