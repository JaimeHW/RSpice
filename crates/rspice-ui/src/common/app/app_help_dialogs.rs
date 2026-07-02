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

fn shortcut_row_matches(command: ShortcutCommand, filter: &str) -> bool {
    filter.is_empty()
        || command.display_name().to_lowercase().contains(filter)
        || command.shortcut_string().to_lowercase().contains(filter)
}

impl RSpiceApp {
    pub(super) fn render_about_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.about {
            return;
        }

        let license_line = match &self.state.license {
            Some(info) => format!("{} · until {}", info.tier, info.updates_until),
            None => "—".to_owned(),
        };

        let license_for_body = license_line.clone();
        let choice = Dialog::new("RSpice", "About", "Close")
            .size(DialogSize::Sm)
            .ghost("Copy diagnostics")
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
                kv_row(
                    ui,
                    "Version",
                    concat!(env!("CARGO_PKG_VERSION"), " · ", env!("RSPICE_BUILD_HASH")),
                );
                kv_row(
                    ui,
                    "Engine",
                    concat!("rspice-core ", env!("CARGO_PKG_VERSION")),
                );
                kv_row(ui, "Matrix pkg", "faer sparse");
                kv_row(ui, "License", &license_for_body);
            });

        match choice {
            DialogChoice::None => {}
            DialogChoice::Ghost => {
                // The first thing support asks for, one click to produce.
                let diagnostics = format!(
                    "RSpice {} ({}) · rspice-core {} · faer sparse · license {} · {}-{}",
                    env!("CARGO_PKG_VERSION"),
                    env!("RSPICE_BUILD_HASH"),
                    env!("CARGO_PKG_VERSION"),
                    license_line,
                    std::env::consts::OS,
                    std::env::consts::ARCH,
                );
                ctx.copy_text(diagnostics);
                self.state.shell.toasts.info(ctx, "Diagnostics copied");
            }
            _ => self.state.dialogs.about = false,
        }
    }

    pub(super) fn render_waveform_calculator_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.waveform_calculator_dialog {
            return;
        }

        let hint = self
            .state
            .calculator_panel
            .context_hint(&self.state.simulation);

        let state = &mut self.state;
        let choice = Dialog::new("Results", "Waveform calculator", "Evaluate")
            .size(DialogSize::Lg)
            .secondary("Plot result")
            .ghost("Clear")
            .hint(&hint)
            .show(ctx, |ui| {
                let (panel, simulation) = (&mut state.calculator_panel, &state.simulation);
                panel.show_body(ui, simulation);
            });

        match choice {
            DialogChoice::None => {}
            DialogChoice::Primary => {
                let (panel, simulation) =
                    (&mut self.state.calculator_panel, &self.state.simulation);
                panel.evaluate(simulation);
            }
            DialogChoice::Ghost => self.state.calculator_panel.clear(),
            DialogChoice::Secondary => self.plot_calculator_expression(ctx),
            DialogChoice::Cancelled => {
                self.state.dialogs.waveform_calculator_dialog = false;
            }
        }
    }

    /// "Plot result": hand the calculator expression to the waves strips as
    /// an expression trace on the active analysis and jump to Results.
    fn plot_calculator_expression(&mut self, ctx: &Context) {
        let expression = self.state.calculator_panel.expression.trim().to_owned();
        if expression.is_empty() {
            return;
        }
        let analysis = self.state.simulation.active_analysis_idx.unwrap_or(0);
        let traces = self.state.shell.results.exprs.entry(analysis).or_default();
        if !traces.iter().any(|t| t.text == expression) {
            traces.push(crate::shell::results::ExprTrace {
                text: expression.clone(),
                visible: true,
            });
        }
        self.state.shell.results.viewer = crate::shell::results::ResultViewer::Waves;
        self.state.shell.view = crate::shell::WorkspaceView::Results;
        self.state.dialogs.waveform_calculator_dialog = false;
        self.state
            .shell
            .toasts
            .info(ctx, format!("Plotted {expression}"));
    }

    pub(super) fn render_shortcuts_help_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.shortcuts_help {
            return;
        }

        // Count against the filter as entered last frame — immediate mode
        // corrects the hint one frame after each keystroke.
        let filter = self.state.dialogs.shortcuts_filter.trim().to_lowercase();
        let total: usize = ShortcutCategory::ALL
            .iter()
            .map(|c| c.commands().len())
            .sum();
        let shown: usize = ShortcutCategory::ALL
            .iter()
            .flat_map(|c| c.commands())
            .filter(|c| shortcut_row_matches(**c, &filter))
            .count();
        let hint = format!("{shown} of {total} shortcuts");

        let filter_text = &mut self.state.dialogs.shortcuts_filter;
        let choice = Dialog::new("Help", "Keyboard shortcuts", "Close")
            .size(DialogSize::Md)
            .hint(&hint)
            .show(ctx, |ui| {
                let t = Tokens::get(ui.ctx());
                let c = t.color;

                ui.add(
                    egui::TextEdit::singleline(filter_text)
                        .hint_text("Filter — try 'mirror' or 'zoom'")
                        .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                        .desired_width(f32::INFINITY),
                );
                let filter = filter_text.trim().to_lowercase();

                ui.spacing_mut().item_spacing.y = 0.0;
                let mut any_shown = false;

                for category in ShortcutCategory::ALL {
                    let commands: Vec<_> = category
                        .commands()
                        .iter()
                        .copied()
                        .filter(|c| shortcut_row_matches(*c, &filter))
                        .collect();
                    if commands.is_empty() {
                        continue;
                    }
                    any_shown = true;

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

                    for command in commands {
                        let (shortcut, display_name) = shortcut_help_row(command);
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
                        let key_galley = ui.fonts_mut(|f| {
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
                            egui::StrokeKind::Inside,
                        );
                        painter.galley(key_rect.min + pad, key_galley, c.text);
                    }
                }

                if !any_shown {
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("nothing matches — clear the filter")
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(c.text_faint),
                    );
                }
            });

        if choice != DialogChoice::None {
            self.state.dialogs.shortcuts_help = false;
            self.state.dialogs.shortcuts_filter.clear();
        }
    }
}
