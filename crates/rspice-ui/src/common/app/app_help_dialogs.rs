//! Help dialogs on the modal primitive: About, keyboard shortcuts, and the
//! waveform calculator host.

use egui::Context;

use super::{
    RSpiceApp,
    app_shortcuts::{ShortcutCategory, ShortcutCommand},
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, kv_row};

fn shortcut_help_row(command: ShortcutCommand) -> (&'static str, &'static str) {
    (command.shortcut_string(), command.display_name())
}

impl RSpiceApp {
    pub(super) fn render_about_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.about {
            return;
        }

        let choice = Dialog::new("RSpice", "About", "Close")
            .size(DialogSize::Sm)
            .show(ctx, |ui| {
                let t = Tokens::get(ui.ctx());
                let c = t.color;
                ui.label(
                    egui::RichText::new("RSpice")
                        .font(theme::sans(tokens::FS_4, FontWeight::SemiBold))
                        .color(c.text),
                );
                ui.label(
                    egui::RichText::new("Analog circuit design and simulation")
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(c.text_dim),
                );
                ui.add_space(10.0);
                ui.spacing_mut().item_spacing.y = 0.0;
                kv_row(ui, "Version", env!("CARGO_PKG_VERSION"));
                kv_row(ui, "Engine", concat!("rspice-core ", env!("CARGO_PKG_VERSION")));
                kv_row(ui, "Matrix pkg", "faer sparse");
            });

        if choice != DialogChoice::None {
            self.state.dialogs.about = false;
        }
    }

    pub(super) fn render_waveform_calculator_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.waveform_calculator_dialog {
            return;
        }

        let choice = Dialog::new("Results", "Waveform calculator", "Close")
            .size(DialogSize::Md)
            .show(ctx, |ui| {
                self.state.calculator_panel.show(ui, &self.state.simulation);
            });

        if choice != DialogChoice::None {
            self.state.dialogs.waveform_calculator_dialog = false;
        }
    }

    pub(super) fn render_shortcuts_help_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.shortcuts_help {
            return;
        }

        let choice = Dialog::new("Help", "Keyboard shortcuts", "Close")
            .size(DialogSize::Md)
            .show(ctx, |ui| {
                let t = Tokens::get(ui.ctx());
                let c = t.color;
                ui.spacing_mut().item_spacing.y = 0.0;

                for category in ShortcutCategory::ALL {
                    ui.add_space(8.0);
                    let mut header = egui::text::LayoutJob::default();
                    header.append(
                        &category.display_name().to_uppercase(),
                        0.0,
                        egui::TextFormat {
                            font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
                            color: c.text_faint,
                            extra_letter_spacing: 0.08 * tokens::FS_0,
                            ..Default::default()
                        },
                    );
                    ui.label(header);
                    ui.add_space(2.0);

                    for command in category.commands() {
                        let (shortcut, display_name) = shortcut_help_row(*command);
                        // Description left, key chord right in mono — the
                        // kv pattern with the value rendered as a key cap.
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(ui.available_width(), 24.0),
                            egui::Sense::hover(),
                        );
                        let painter = ui.painter();
                        painter.text(
                            egui::pos2(rect.left(), rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            display_name,
                            theme::sans(tokens::FS_1, FontWeight::Regular),
                            c.text_dim,
                        );
                        let key_galley = ui.fonts(|f| {
                            f.layout_no_wrap(
                                shortcut.to_owned(),
                                theme::mono(tokens::FS_0, FontWeight::Regular),
                                c.text,
                            )
                        });
                        let pad = egui::vec2(7.0, 3.0);
                        let key_rect = egui::Rect::from_min_size(
                            egui::pos2(
                                rect.right() - key_galley.size().x - 2.0 * pad.x,
                                rect.center().y - key_galley.size().y * 0.5 - pad.y,
                            ),
                            key_galley.size() + 2.0 * pad,
                        );
                        painter.rect(
                            key_rect,
                            t.radius,
                            c.bg_inset,
                            egui::Stroke::new(1.0, c.border_strong),
                        );
                        painter.galley(
                            key_rect.min + pad,
                            key_galley,
                            c.text,
                        );
                    }
                }
            });

        if choice != DialogChoice::None {
            self.state.dialogs.shortcuts_help = false;
        }
    }
}
