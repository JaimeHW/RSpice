//! Console panel — filterable structured log with severity tabs, counts,
//! clear, and collapse, docked above the status bar.

use egui::{Context, Frame, ScrollArea, Sense, TopBottomPanel, Ui, vec2};

use crate::common::AppState;
use crate::panels::{LogSeverity, LogSource};
use crate::shell::state::ConsoleFilter;
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::IconButton;

/// Height of the console header strip (the collapsed height).
const HEADER_HEIGHT: f32 = 29.0;

/// Render the console panel.
pub fn show(ctx: &Context, state: &mut AppState) {
    let t = Tokens::get(ctx);
    let c = t.color;

    let collapsed = state.shell.console.collapsed;
    let panel = TopBottomPanel::bottom("volta.console")
        .frame(Frame::none().fill(c.bg_panel))
        .show_separator_line(false);
    let panel = if collapsed {
        panel.exact_height(HEADER_HEIGHT).resizable(false)
    } else {
        panel
            .resizable(true)
            .default_height(state.shell.console.height)
            .height_range(egui::Rangef::new(HEADER_HEIGHT + 60.0, 480.0))
    };

    panel.show(ctx, |ui| {
        let panel_rect = ui.max_rect();
        ui.painter().hline(
            panel_rect.x_range(),
            panel_rect.top() + 0.5,
            egui::Stroke::new(1.0, c.border),
        );
        if !collapsed {
            state.shell.console.height = panel_rect.height();
        }

        header(ui, state);
        if !state.shell.console.collapsed {
            ui.painter().hline(
                panel_rect.x_range(),
                panel_rect.top() + HEADER_HEIGHT - 0.5,
                egui::Stroke::new(1.0, c.border),
            );
            if state.shell.console.filter == ConsoleFilter::Script {
                script_body(ui, state);
            } else {
                log_body(ui, state);
            }
        }
    });
}

fn header(ui: &mut Ui, state: &mut AppState) {
    let error_count = state.log_buffer.count_by_severity(LogSeverity::Error);
    let warn_count = state.log_buffer.count_by_severity(LogSeverity::Warning);

    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), HEADER_HEIGHT),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.add_space(6.0);
            ui.spacing_mut().item_spacing.x = 2.0;

            let filter = &mut state.shell.console.filter;
            if console_tab(ui, "Console", None, *filter == ConsoleFilter::All) {
                *filter = ConsoleFilter::All;
            }
            if console_tab(
                ui,
                "Errors",
                Some(error_count),
                *filter == ConsoleFilter::Errors,
            ) {
                *filter = ConsoleFilter::Errors;
            }
            if console_tab(
                ui,
                "Warnings",
                Some(warn_count),
                *filter == ConsoleFilter::Warnings,
            ) {
                *filter = ConsoleFilter::Warnings;
            }
            if console_tab(ui, "Automation", None, *filter == ConsoleFilter::Script) {
                *filter = ConsoleFilter::Script;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(6.0);
                let collapsed = state.shell.console.collapsed;
                let chevron = if collapsed {
                    Icon::ChevronUp
                } else {
                    Icon::ChevronDown
                };
                let toggle_tip = if collapsed {
                    "Expand console"
                } else {
                    "Collapse console"
                };
                if IconButton::new(chevron)
                    .side(24.0)
                    .tooltip(toggle_tip)
                    .show(ui)
                    .clicked()
                {
                    state.shell.console.collapsed = !collapsed;
                }
                if IconButton::new(Icon::Trash)
                    .side(24.0)
                    .tooltip("Clear console")
                    .show(ui)
                    .clicked()
                {
                    if state.shell.console.filter == ConsoleFilter::Script {
                        state.script_console.history.clear();
                    } else {
                        state.clear_primary_log();
                    }
                }
            });
        },
    );
}

/// One console filter tab with an optional count. Returns `true` on click.
fn console_tab(ui: &mut Ui, label: &str, count: Option<usize>, active: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let label_galley = ui.fonts(|f| {
        f.layout_no_wrap(
            label.to_owned(),
            theme::sans(tokens::FS_1, FontWeight::Regular),
            c.text,
        )
    });
    let count_galley = count.map(|n| {
        ui.fonts(|f| {
            f.layout_no_wrap(
                n.to_string(),
                theme::mono(10.0, FontWeight::Regular),
                c.text_faint,
            )
        })
    });

    let mut width = 20.0 + label_galley.size().x;
    if let Some(g) = &count_galley {
        width += 4.0 + g.size().x;
    }
    let (rect, response) = ui.allocate_exact_size(vec2(width, 22.0), Sense::click());
    if !ui.is_rect_visible(rect) {
        return false;
    }

    let hover = ui
        .ctx()
        .animate_bool_with_time(response.id, response.hovered() && !active, 0.16);
    let painter = ui.painter();
    if active {
        painter.rect_filled(rect, t.radius, c.bg_active);
    } else if hover > 0.0 {
        painter.rect_filled(
            rect,
            t.radius,
            mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
        );
    }
    let text_color = if active {
        c.text
    } else {
        mix(c.text_dim, c.text, hover)
    };
    let mut x = rect.left() + 10.0;
    painter.galley(
        egui::pos2(x, rect.center().y - label_galley.size().y * 0.5),
        label_galley.clone(),
        text_color,
    );
    x += label_galley.size().x + 4.0;
    if let Some(g) = count_galley {
        painter.galley(
            egui::pos2(x, rect.center().y - g.size().y * 0.5),
            g,
            c.text_faint,
        );
    }

    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}

/// Level column: label text + color per severity/source.
fn level_style(
    entry_severity: LogSeverity,
    entry_source: LogSource,
    c: &crate::ui::palette::Palette,
) -> (&'static str, egui::Color32) {
    match entry_severity {
        LogSeverity::Error => ("error", c.err),
        LogSeverity::Warning => ("warn", c.warn),
        // Design-check results read as green "check" lines, like the design.
        LogSeverity::Info if entry_source == LogSource::Drc => ("check", c.ok),
        LogSeverity::Info => ("info", c.text_dim),
        LogSeverity::Debug => ("debug", c.text_faint),
        LogSeverity::Trace => ("trace", c.text_faint),
    }
}

/// The interactive automation console: mono history above, a command
/// input pinned at the bottom.
///
/// Laid out bottom-up with the history filling all remaining space — the
/// panel re-fits to its content each frame, so any gap between content
/// and panel height would ratchet the resizable panel smaller.
fn script_body(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let row_font = theme::mono(11.5, FontWeight::Regular);

    ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
        // Command input, pinned to the panel bottom.
        ui.add_space(5.0);
        let response = ui
            .horizontal(|ui| {
                ui.add_space(8.0);
                ui.add_sized(
                    vec2(ui.available_width() - 8.0, t.metrics.ctl_h),
                    egui::TextEdit::singleline(&mut state.script_console.input_buffer)
                        .font(egui::TextStyle::Monospace)
                        .hint_text("run tran · plot v(out) · help")
                        .margin(egui::Margin::symmetric(8.0, 4.0))
                        .lock_focus(true),
                )
            })
            .inner;
        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            let command = state.script_console.input_buffer.trim().to_string();
            if !command.is_empty() {
                let output = state
                    .script_console
                    .executor
                    .execute_command(&command, &mut state.simulation);
                state
                    .script_console
                    .history
                    .push(crate::panels::ConsoleHistoryItem { command, output });
                state.script_console.input_buffer.clear();
                response.request_focus();
            }
        }
        ui.add_space(4.0);

        // History fills everything above the input exactly.
        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            ScrollArea::vertical()
                .id_salt("volta.console.script")
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    if state.script_console.history.is_empty() {
                        let (rect, _) = ui
                            .allocate_exact_size(vec2(ui.available_width(), 20.0), Sense::hover());
                        ui.painter().text(
                            egui::pos2(rect.left() + 12.0, rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            "Type a command — 'help' lists the automation verbs",
                            row_font.clone(),
                            c.text_faint,
                        );
                    }
                    for item in &state.script_console.history {
                        let width = ui.available_width();
                        let (rect, _) = ui.allocate_exact_size(vec2(width, 20.0), Sense::hover());
                        let painter = ui.painter();
                        painter.text(
                            egui::pos2(rect.left() + 12.0, rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            format!("> {}", item.command),
                            row_font.clone(),
                            c.text,
                        );
                        let color = if item.output.success { c.ok } else { c.err };
                        for line in item.output.message.lines() {
                            let (rect, _) =
                                ui.allocate_exact_size(vec2(width, 20.0), Sense::hover());
                            ui.painter().text(
                                egui::pos2(rect.left() + 24.0, rect.center().y),
                                egui::Align2::LEFT_CENTER,
                                line,
                                row_font.clone(),
                                color,
                            );
                        }
                    }
                });
        });
    });
}

fn log_body(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let filter = state.shell.console.filter;

    // Row → entry map for the active filter, rebuilt only when the buffer
    // or the filter changes — never per frame.
    let key = (filter, state.log_buffer.revision(), state.log_buffer.len());
    if state.shell.console.rows_key != Some(key) {
        let rows: Vec<u32> = state
            .log_buffer
            .entries()
            .enumerate()
            .filter(|(_, entry)| match filter {
                ConsoleFilter::All | ConsoleFilter::Script => true,
                ConsoleFilter::Errors => entry.severity == LogSeverity::Error,
                ConsoleFilter::Warnings => entry.severity == LogSeverity::Warning,
            })
            .map(|(index, _)| index as u32)
            .collect();
        state.shell.console.rows = rows;
        state.shell.console.rows_key = Some(key);
    }

    let row_height = 20.0;
    let total_rows = state.shell.console.rows.len();
    let row_font = theme::mono(11.5, FontWeight::Regular);

    // Anchored rows are clickable; the jump applies after the loop so the
    // buffer borrow ends first.
    let mut jump: Option<crate::panels::LogAnchor> = None;

    // Virtualized: only the rows in view are laid out and painted, so a
    // full 10k-entry buffer costs the same as thirty rows.
    ui.spacing_mut().item_spacing.y = 0.0;
    ScrollArea::vertical()
        .id_salt("volta.console.scroll")
        .auto_shrink([false, false])
        .stick_to_bottom(true)
        .show_rows(ui, row_height, total_rows, |ui, range| {
            for row in range {
                let Some(&entry_index) = state.shell.console.rows.get(row) else {
                    continue;
                };
                let Some(entry) = state.log_buffer.entry(entry_index as usize) else {
                    continue;
                };
                let (level_label, level_color) = level_style(entry.severity, entry.source, &c);

                let width = ui.available_width();
                let sense = if entry.anchor.is_some() {
                    Sense::click()
                } else {
                    Sense::hover()
                };
                let (rect, response) = ui.allocate_exact_size(vec2(width, row_height), sense);
                let painter = ui.painter();
                if entry.anchor.is_some() {
                    if response.hovered() {
                        painter.rect_filled(rect, 2.0, c.bg_hover);
                    }
                    if response
                        .clone()
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        jump = entry.anchor.clone();
                    }
                }
                let cy = rect.center().y;
                painter.text(
                    egui::pos2(rect.left() + 12.0, cy),
                    egui::Align2::LEFT_CENTER,
                    entry.format_timestamp(),
                    row_font.clone(),
                    c.text_faint,
                );
                painter.text(
                    egui::pos2(rect.left() + 110.0, cy),
                    egui::Align2::LEFT_CENTER,
                    level_label,
                    row_font.clone(),
                    level_color,
                );
                painter.text(
                    egui::pos2(rect.left() + 160.0, cy),
                    egui::Align2::LEFT_CENTER,
                    &entry.message,
                    row_font.clone(),
                    c.text,
                );
            }
        });

    // Click-to-source: jump the canvas to the finding's anchor and select
    // the offending object. The console stays open — the row is the map,
    // the canvas is the territory.
    if let Some(anchor) = jump {
        state.jump_to_log_anchor(anchor);
    }
}
