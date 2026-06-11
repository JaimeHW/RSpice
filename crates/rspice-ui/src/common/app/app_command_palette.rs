//! Command palette (Ctrl+K) — keyboard-first access to every command.
//!
//! A floating top-center surface, lighter than a modal: mono filter input
//! over a ranked command list with the shortcut hints right-aligned.
//! Arrows move, Enter runs, Esc closes, click runs. Filtering prefers
//! prefix matches, then word starts, then subsequences.

use egui::{Context, Key};

use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};

use super::RSpiceApp;
use super::app_shortcuts::ShortcutCommand;

/// Most rows shown at once; the rest are reachable by typing.
const MAX_ROWS: usize = 12;

/// How well a command matches the query (lower sorts first).
fn match_rank(name: &str, query: &str) -> Option<u8> {
    if query.is_empty() {
        return Some(3);
    }
    let name_lower = name.to_lowercase();
    let query_lower = query.to_lowercase();
    if name_lower.starts_with(&query_lower) {
        return Some(0);
    }
    if name_lower
        .split_whitespace()
        .any(|word| word.starts_with(&query_lower))
    {
        return Some(1);
    }
    if name_lower.contains(&query_lower) {
        return Some(2);
    }
    // Subsequence: every query char appears in order.
    let mut chars = name_lower.chars();
    if query_lower
        .chars()
        .all(|q| chars.by_ref().any(|n| n == q))
    {
        return Some(4);
    }
    None
}

/// The filtered, ranked command list for a query.
fn filtered_commands(query: &str) -> Vec<ShortcutCommand> {
    let mut ranked: Vec<(u8, ShortcutCommand)> = ShortcutCommand::ALL
        .iter()
        .copied()
        // Cancel and the palette itself are pointless from inside the palette.
        .filter(|c| {
            !matches!(
                c,
                ShortcutCommand::EscapeCancel | ShortcutCommand::OpenCommandPalette
            )
        })
        .filter_map(|c| match_rank(c.display_name(), query).map(|rank| (rank, c)))
        .collect();
    ranked.sort_by_key(|(rank, c)| (*rank, c.display_name()));
    ranked.into_iter().map(|(_, c)| c).collect()
}

impl RSpiceApp {
    pub(super) fn render_command_palette(&mut self, ctx: &Context) {
        if !self.state.dialogs.command_palette.open {
            return;
        }
        let t = Tokens::get(ctx);
        let c = t.color;

        let commands = filtered_commands(&self.state.dialogs.command_palette.query);
        let visible = commands.len().min(MAX_ROWS);
        let palette = &mut self.state.dialogs.command_palette;
        palette.selected = palette.selected.min(visible.saturating_sub(1));

        // Keyboard before UI: the input keeps focus, so navigation reads the
        // raw key state rather than competing for events.
        let (down, up, enter, escape) = ctx.input(|i| {
            (
                i.key_pressed(Key::ArrowDown),
                i.key_pressed(Key::ArrowUp),
                i.key_pressed(Key::Enter),
                i.key_pressed(Key::Escape),
            )
        });
        if escape {
            self.state.dialogs.command_palette.open = false;
            return;
        }
        if down && visible > 0 {
            let palette = &mut self.state.dialogs.command_palette;
            palette.selected = (palette.selected + 1) % visible;
        }
        if up && visible > 0 {
            let palette = &mut self.state.dialogs.command_palette;
            palette.selected = palette.selected.checked_sub(1).unwrap_or(visible - 1);
        }

        let mut run: Option<ShortcutCommand> = None;
        let screen = ctx.screen_rect();
        let width = 520.0_f32.min(screen.width() - 48.0);

        egui::Area::new(egui::Id::new("volta.command_palette"))
            .order(egui::Order::Foreground)
            .fixed_pos(egui::pos2(
                screen.center().x - width * 0.5,
                screen.top() + 96.0,
            ))
            .show(ctx, |ui| {
                ui.set_width(width);
                egui::Frame::none()
                    .fill(c.bg_panel)
                    .stroke(egui::Stroke::new(1.0, c.border_strong))
                    .rounding(6.0)
                    .shadow(egui::epaint::Shadow {
                        offset: egui::vec2(0.0, 10.0),
                        blur: 32.0,
                        spread: 0.0,
                        color: egui::Color32::from_black_alpha(110),
                    })
                    .inner_margin(egui::Margin::same(8.0))
                    .show(ui, |ui| {
                        let palette = &mut self.state.dialogs.command_palette;
                        let response = ui.add(
                            egui::TextEdit::singleline(&mut palette.query)
                                .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                                .hint_text("Type a command…")
                                .desired_width(f32::INFINITY),
                        );
                        if palette.want_focus {
                            response.request_focus();
                            palette.want_focus = false;
                        }
                        if response.changed() {
                            palette.selected = 0;
                        }
                        // Click-away dismisses.
                        if response.clicked_elsewhere()
                            && ui.input(|i| i.pointer.any_pressed())
                            && !ui.rect_contains_pointer(ui.min_rect().expand(8.0))
                        {
                            palette.open = false;
                        }

                        ui.add_space(6.0);
                        let selected = palette.selected;

                        if commands.is_empty() {
                            ui.label(
                                egui::RichText::new("No matching command")
                                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                                    .color(c.text_faint),
                            );
                        }
                        for (index, command) in commands.iter().take(MAX_ROWS).enumerate() {
                            if command_row(ui, command, index == selected) {
                                run = Some(*command);
                            }
                        }
                        let hidden = commands.len().saturating_sub(MAX_ROWS);
                        if hidden > 0 {
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new(format!("{hidden} more — keep typing"))
                                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                    .color(c.text_faint),
                            );
                        }
                    });
            });

        if enter && run.is_none() {
            run = commands
                .get(self.state.dialogs.command_palette.selected)
                .copied();
        }
        if let Some(command) = run {
            self.state.dialogs.command_palette.open = false;
            self.execute_shortcut_command(command);
        }
    }
}

/// One palette row: name left, shortcut hint right; accent edge when
/// selected. Returns true when clicked.
fn command_row(ui: &mut egui::Ui, command: &ShortcutCommand, selected: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 28.0),
        egui::Sense::click(),
    );
    if !ui.is_rect_visible(rect) {
        return false;
    }

    let hover = ui
        .ctx()
        .animate_bool_with_time(response.id, response.hovered(), 0.12);
    let painter = ui.painter();
    if selected || hover > 0.0 {
        let fill = if selected {
            c.bg_hover
        } else {
            mix(egui::Color32::TRANSPARENT, c.bg_hover, hover)
        };
        painter.rect_filled(rect, t.radius, fill);
    }
    if selected {
        painter.rect_filled(
            egui::Rect::from_min_size(rect.min, egui::vec2(2.0, rect.height())),
            1.0,
            c.accent,
        );
    }

    painter.text(
        egui::pos2(rect.left() + 12.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        command.display_name(),
        theme::sans(tokens::FS_1, FontWeight::Regular),
        if selected { c.text } else { c.text_dim },
    );
    let shortcut = command.shortcut_string();
    if !shortcut.is_empty() {
        painter.text(
            egui::pos2(rect.right() - 10.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            shortcut,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        );
    }

    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}
