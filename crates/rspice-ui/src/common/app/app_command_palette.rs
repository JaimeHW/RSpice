//! Command palette (Ctrl+K) — keyboard-first access to every command.
//!
//! A floating top-center surface, lighter than a modal: mono filter input
//! over a ranked command list with the shortcut hints right-aligned
//! as a product command surface. Arrows move, Enter runs,
//! Esc closes, click runs. Filtering prefers prefix matches, then word
//! starts, then substrings, then subsequences; ties keep the canonical
//! order so related commands stay grouped. Matched letters render in
//! accent so subsequence hits explain themselves. An empty query leads
//! with the last five commands run from here, under a RECENT header.
//! Context verbs (descend/ascend) stay listed when they cannot run,
//! dimmed, with the reason in place of the shortcut.

use egui::{Context, Key};

use crate::state::ComponentType;
use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};

use super::RSpiceApp;
use super::app_shortcuts::ShortcutCommand;

/// Most rows shown at once; the rest are reachable by typing.
const MAX_ROWS: usize = 12;

/// How many executed commands the RECENT section keeps.
const MAX_RECENT: usize = 5;

/// How well a command matches the query (lower sorts first), plus which
/// display-name chars matched (char indices, ascending).
fn match_spans(name: &str, query: &str) -> Option<(u8, Vec<usize>)> {
    if query.is_empty() {
        return Some((3, Vec::new()));
    }
    let name_lower: Vec<char> = name.to_lowercase().chars().collect();
    let query_lower: Vec<char> = query.to_lowercase().chars().collect();

    if let Some(at) = find_chars(&name_lower, &query_lower) {
        let rank = if at == 0 {
            0
        } else if name_lower[at - 1] == ' ' {
            1
        } else {
            2
        };
        return Some((rank, (at..at + query_lower.len()).collect()));
    }

    // Subsequence: every query char appears in order.
    let mut marks = Vec::with_capacity(query_lower.len());
    let mut next = 0;
    for (index, ch) in name_lower.iter().enumerate() {
        if next < query_lower.len() && *ch == query_lower[next] {
            marks.push(index);
            next += 1;
        }
    }
    (next == query_lower.len()).then_some((4, marks))
}

fn find_chars(haystack: &[char], needle: &[char]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    (0..=haystack.len() - needle.len()).find(|&at| haystack[at..at + needle.len()] == *needle)
}

/// One renderable palette row.
struct PaletteRow {
    command: ShortcutCommand,
    /// Matched display-name char indices (accent-rendered).
    marks: Vec<usize>,
    /// None = runnable; Some(reason) renders dimmed with the reason in
    /// place of the shortcut, and refuses to run.
    blocked: Option<&'static str>,
}

/// Commands the palette offers — everything except Cancel and the palette
/// itself, which are pointless from inside the palette.
fn palette_commands() -> impl Iterator<Item = ShortcutCommand> {
    ShortcutCommand::ALL.iter().copied().filter(|c| {
        !matches!(
            c,
            ShortcutCommand::EscapeCancel | ShortcutCommand::OpenCommandPalette
        )
    })
}

impl RSpiceApp {
    /// Why a context verb cannot run right now (None = it can).
    fn palette_blocker(&self, command: ShortcutCommand) -> Option<&'static str> {
        match command {
            ShortcutCommand::DescendIntoSelected => {
                let schematic = &self.state.schematic;
                let descendable = schematic
                    .selection
                    .single_component()
                    .and_then(|id| schematic.components.iter().find(|c| c.id == id))
                    .is_some_and(|c| {
                        c.kind == ComponentType::CellInstance && c.library_cell.is_some()
                    });
                (!descendable).then_some("select one cell instance")
            }
            ShortcutCommand::AscendHierarchy => {
                (self.state.workspace.hierarchy_stack.len() < 2).then_some("already at top")
            }
            _ => None,
        }
    }

    /// The filtered, ranked rows plus how many of them are RECENT entries.
    fn palette_rows(&self, query: &str) -> (Vec<PaletteRow>, usize) {
        let row = |command: ShortcutCommand, marks: Vec<usize>| PaletteRow {
            command,
            marks,
            blocked: self.palette_blocker(command),
        };

        if query.is_empty() {
            let recents = &self.state.dialogs.command_palette.recent;
            let mut rows: Vec<PaletteRow> = recents
                .iter()
                .take(MAX_RECENT)
                .map(|c| row(*c, Vec::new()))
                .collect();
            let recent_count = rows.len();
            rows.extend(
                palette_commands()
                    .filter(|c| !recents.contains(c))
                    .map(|c| row(c, Vec::new())),
            );
            return (rows, recent_count);
        }

        let mut ranked: Vec<(u8, PaletteRow)> = palette_commands()
            .filter_map(|c| match_spans(c.display_name(), query).map(|(r, m)| (r, row(c, m))))
            .collect();
        // Stable by rank only — ties keep the canonical ALL order, so the
        // Place family stays a family instead of alphabet soup.
        ranked.sort_by_key(|(rank, _)| *rank);
        (ranked.into_iter().map(|(_, r)| r).collect(), 0)
    }

    pub(super) fn render_command_palette(&mut self, ctx: &Context) {
        if !self.state.dialogs.command_palette.open {
            return;
        }
        let t = Tokens::get(ctx);
        let c = t.color;

        let query = self.state.dialogs.command_palette.query.clone();
        let (rows, recent_count) = self.palette_rows(&query);
        let visible = rows.len().min(MAX_ROWS);
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
        let query_empty = query.is_empty();

        egui::Area::new(egui::Id::new("volta.command_palette"))
            .order(egui::Order::Foreground)
            .fixed_pos(egui::pos2(
                screen.center().x - width * 0.5,
                screen.top() + 96.0,
            ))
            .show(ctx, |ui| {
                ui.set_width(width);
                egui::Frame::NONE
                    .fill(c.bg_panel)
                    .stroke(egui::Stroke::new(1.0, c.border_strong))
                    .rounding(6.0)
                    .shadow(egui::epaint::Shadow {
                        offset: [0, 10],
                        blur: 32,
                        spread: 0,
                        color: egui::Color32::from_black_alpha(110),
                    })
                    .inner_margin(egui::Margin::same(8))
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

                        if rows.is_empty() {
                            ui.label(
                                egui::RichText::new("No matching command")
                                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                                    .color(c.text_faint),
                            );
                        }
                        for (index, row) in rows.iter().take(MAX_ROWS).enumerate() {
                            if query_empty && recent_count > 0 {
                                if index == 0 {
                                    section_header(ui, "RECENT");
                                }
                                if index == recent_count {
                                    ui.add_space(4.0);
                                }
                            }
                            if command_row(ui, row, index == selected) {
                                run = Some(row.command);
                            }
                        }
                        let hidden = rows.len().saturating_sub(MAX_ROWS);
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
            run = rows
                .get(self.state.dialogs.command_palette.selected)
                .filter(|row| row.blocked.is_none())
                .map(|row| row.command);
        }
        if let Some(command) = run {
            let recent = &mut self.state.dialogs.command_palette.recent;
            recent.retain(|c| *c != command);
            recent.insert(0, command);
            recent.truncate(MAX_RECENT);
            self.state.dialogs.command_palette.open = false;
            self.execute_shortcut_command(command);
        }
    }
}

/// Small mono section header inside the list (RECENT).
fn section_header(ui: &mut egui::Ui, label: &str) {
    let c = Tokens::get(ui.ctx()).color;
    let mut job = egui::text::LayoutJob::default();
    job.append(
        label,
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
            color: c.text_faint,
            extra_letter_spacing: 0.1 * tokens::FS_0,
            ..Default::default()
        },
    );
    ui.add_space(2.0);
    ui.label(job);
    ui.add_space(2.0);
}

/// One palette row: name left (matched chars in accent), shortcut hint —
/// or the blocked reason — right; accent edge when selected. Returns true
/// when a runnable row is clicked.
fn command_row(ui: &mut egui::Ui, row: &PaletteRow, selected: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 28.0), egui::Sense::click());
    if !ui.is_rect_visible(rect) {
        return false;
    }

    let blocked = row.blocked.is_some();
    let hover = ui
        .ctx()
        .animate_bool_with_time(response.id, response.hovered() && !blocked, 0.12);
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

    let base_color = if blocked {
        c.text_faint
    } else if selected {
        c.text
    } else {
        c.text_dim
    };
    let name = row.command.display_name();
    let mut job = egui::text::LayoutJob::default();
    let mut next_mark = 0;
    for (index, ch) in name.chars().enumerate() {
        let marked = next_mark < row.marks.len() && row.marks[next_mark] == index;
        if marked {
            next_mark += 1;
        }
        let format = if marked {
            egui::TextFormat {
                font_id: theme::sans(tokens::FS_1, FontWeight::Medium),
                color: c.accent,
                ..Default::default()
            }
        } else {
            egui::TextFormat {
                font_id: theme::sans(tokens::FS_1, FontWeight::Regular),
                color: base_color,
                ..Default::default()
            }
        };
        let mut buffer = [0u8; 4];
        job.append(ch.encode_utf8(&mut buffer), 0.0, format);
    }
    let galley = ui.fonts_mut(|f| f.layout_job(job));
    painter.galley(
        egui::pos2(rect.left() + 12.0, rect.center().y - galley.size().y * 0.5),
        galley,
        base_color,
    );

    let right_text = row.blocked.unwrap_or_else(|| row.command.shortcut_string());
    if !right_text.is_empty() {
        painter.text(
            egui::pos2(rect.right() - 10.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            right_text,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        );
    }

    if blocked {
        return false;
    }
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}
