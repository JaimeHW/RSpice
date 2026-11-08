//! The References page: every place Find references found a name, one row
//! each with its file, line, column and text.
//!
//! Built as the Problems page's table is: one keyboard stop over the rows,
//! the arrows, Home, End and the page keys to move, Enter to open the row at
//! its column, Esc to let go. A click opens the row too.

use egui::{Align, Key, Layout, Modifiers, ScrollArea, Sense, Ui, Vec2};

use super::{
    CONSOLE_BODY_PADDING_X, CONSOLE_CELL_INSET, CONSOLE_FONT_SIZE, CONSOLE_ROW_MIN_HEIGHT,
    CONSOLE_TABLE_HEADER_HEIGHT, muted, paint_header_band, paint_row_fill, row_announcement,
    toolbar,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::workbench::app_state::diagnostic_store::DiagnosticRoute;
use crate::workbench::app_state::session::shortcuts::report_escape_owner;
use crate::workbench::framework::workbench_host::WorkbenchHost;
use crate::workbench::state::{ConsolePage, SourceReferences};

const HEADINGS: [&str; 4] = ["File", "Line", "Column", "Text"];

fn body_id() -> egui::Id {
    egui::Id::new("workbench.references.body")
}

fn row_id(index: usize) -> egui::Id {
    egui::Id::new(("workbench.references.row", index))
}

/// Drop a list whose bundle has left the project, and leave the page when
/// there is no list to show: a session restored on it holds none.
pub(super) fn settle(app: &mut impl WorkbenchHost) {
    let has_list = app
        .workbench()
        .source_references
        .as_ref()
        .is_some_and(|list| {
            app.model()
                .workspace
                .content
                .project_sources
                .get_bundle(list.bundle)
                .is_some()
        });
    if !has_list {
        let workbench = app.workbench_mut();
        workbench.source_references = None;
        if workbench.console_page == ConsolePage::References {
            workbench.console_page = ConsolePage::Console;
        }
    }
}

fn summary(list: &SourceReferences) -> String {
    let count = list.rows.len();
    let files = list
        .rows
        .iter()
        .map(|row| row.logical_path.as_str())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    format!(
        "{count} reference{} to {} in {files} file{}",
        if count == 1 { "" } else { "s" },
        list.name,
        if files == 1 { "" } else { "s" }
    )
}

pub(super) fn references(ui: &mut Ui, app: &mut impl WorkbenchHost) {
    let Some(list) = app.workbench().source_references.as_ref() else {
        return;
    };
    let stale = app
        .model()
        .workspace
        .content
        .project_sources
        .get_bundle(list.bundle)
        .is_some_and(|bundle| bundle.revision().get() != list.revision);
    let summary = summary(list);
    let clear = toolbar(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(summary)
                    .font(theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular))
                    .color(Tokens::get(ui.ctx()).color.text_dim),
            );
            if stale {
                muted(ui, "The source has changed since this list was made.");
            }
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                crate::ui::widgets::Button::new("Clear").show(ui).clicked()
            })
            .inner
        })
        .inner
    });
    if clear {
        let workbench = app.workbench_mut();
        workbench.source_references = None;
        workbench.console_page = ConsolePage::Console;
        return;
    }
    let Some(list) = app.workbench_mut().source_references.as_mut() else {
        return;
    };
    if let Some(route) = table(ui, list) {
        app.model_mut().diagnostics.request_route(route);
    }
}

/// The header band and the rows; returns the place to open, when one was
/// chosen.
fn table(ui: &mut Ui, list: &mut SourceReferences) -> Option<DiagnosticRoute> {
    let tokens = Tokens::get(ui.ctx());
    let font = theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular);
    let heading_font = theme::mono(CONSOLE_FONT_SIZE, FontWeight::SemiBold);
    let measure = |text: &str, font: &egui::FontId| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font.clone(), tokens.color.text)
            .size()
            .x
    };
    // One font, one width to a character: the longest text is the widest.
    let longest = list
        .rows
        .iter()
        .map(|row| row.logical_path.as_str())
        .max_by_key(|path| path.chars().count())
        .unwrap_or_default();
    let highest = |value: fn(&crate::workbench::state::SourceReference) -> usize| {
        list.rows.iter().map(value).max().unwrap_or(1).to_string()
    };
    let cells = [
        longest.to_owned(),
        highest(|row| row.line),
        highest(|row| row.column),
    ];
    let band = ui.available_rect_before_wrap().x_range();
    let mut starts = [0.0_f32; 4];
    let mut x = band.min + CONSOLE_BODY_PADDING_X - CONSOLE_CELL_INSET;
    for (index, cell) in cells.iter().enumerate() {
        starts[index] = x;
        let width = measure(cell, &font).max(measure(HEADINGS[index], &heading_font));
        x += width + 2.0 * CONSOLE_CELL_INSET;
    }
    starts[3] = x;

    // The header shows only over at least one whole row, and the body holds
    // whole rows, as the dock's other tables do.
    let pitch = CONSOLE_ROW_MIN_HEIGHT + ui.spacing().item_spacing.y;
    if ui.available_height() + 0.5 < CONSOLE_TABLE_HEADER_HEIGHT + CONSOLE_ROW_MIN_HEIGHT {
        return None;
    }
    let (_, header) = ui.allocate_space(Vec2::new(band.span(), CONSOLE_TABLE_HEADER_HEIGHT));
    let body_height =
        ((ui.available_height() + ui.spacing().item_spacing.y) / pitch).floor() * pitch;
    paint_header_band(ui, header, &starts[1..], None);
    for (start, heading) in starts.iter().zip(HEADINGS) {
        ui.painter().text(
            egui::pos2(start + CONSOLE_CELL_INSET, header.center().y),
            egui::Align2::LEFT_CENTER,
            heading,
            heading_font.clone(),
            tokens.color.text_faint,
        );
    }

    let per_page = ((body_height / pitch).floor() as usize)
        .saturating_sub(1)
        .max(1);
    let (moved, opened) = keyboard(ui, list, per_page);
    let mut follow = opened;
    let offset_id = egui::Id::new("workbench.references.offset");
    let mut area = ScrollArea::vertical()
        .id_salt("workbench.references")
        .auto_shrink([false, true])
        .max_height(body_height)
        .min_scrolled_height(0.0);
    if moved {
        let shown = ui
            .data(|data| data.get_temp::<f32>(offset_id))
            .unwrap_or(0.0);
        let top = list.selected as f32 * pitch;
        let bottom = top + CONSOLE_ROW_MIN_HEIGHT;
        area = area.vertical_scroll_offset(if top < shown {
            top
        } else if bottom > shown + body_height {
            bottom - body_height
        } else {
            shown
        });
    }
    let mut current = None;
    let mut pressed = None;
    let rows = &list.rows;
    let selected = list.selected;
    let output = area.show_rows(ui, CONSOLE_ROW_MIN_HEIGHT, rows.len(), |ui, range| {
        for index in range {
            let Some(row) = rows.get(index) else {
                continue;
            };
            let (_, rect) =
                ui.allocate_space(Vec2::new(ui.available_width(), CONSOLE_ROW_MIN_HEIGHT));
            let response = ui.interact(rect, row_id(index), Sense::click());
            let is_selected = index == selected;
            if is_selected {
                paint_row_fill(ui, rect, band.max, tokens.color.accent_dim);
                current = Some((response.id, rect));
            } else if response.hovered() {
                paint_row_fill(ui, rect, band.max, tokens.color.bg_hover);
            } else if index % 2 == 1 {
                paint_row_fill(ui, rect, band.max, ui.visuals().faint_bg_color);
            }
            let line = row.line.to_string();
            let column = row.column.to_string();
            let texts = [row.logical_path.as_str(), &line, &column, &row.text];
            for (at, (start, text)) in starts.iter().zip(texts).enumerate() {
                let cell = egui::Rect::from_x_y_ranges(
                    *start..=starts.get(at + 1).copied().unwrap_or(band.max),
                    rect.y_range(),
                );
                ui.painter()
                    .with_clip_rect(cell.intersect(ui.clip_rect()))
                    .text(
                        egui::pos2(start + CONSOLE_CELL_INSET, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        text,
                        font.clone(),
                        tokens.color.text,
                    );
            }
            let label = row_announcement(&texts);
            response.widget_info(|| {
                egui::WidgetInfo::selected(
                    egui::WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    is_selected,
                    label.clone(),
                )
            });
            if response.clicked() {
                pressed = Some(index);
            }
        }
    });
    ui.data_mut(|data| data.insert_temp(offset_id, output.state.offset.y));

    let response = body(ui, output.inner_rect, current);
    if std::mem::take(&mut list.take_focus) {
        response.request_focus();
    }
    report_escape_owner(&response);
    if let Some(index) = pressed {
        list.selected = index;
        follow = list.rows.get(index).map(|row| row.route.clone());
    }
    if follow.is_some() {
        response.surrender_focus();
    }
    follow
}

/// The table's one keyboard stop, laid over the rows it moves through.
fn body(ui: &mut Ui, rect: egui::Rect, current: Option<(egui::Id, egui::Rect)>) -> egui::Response {
    let id = body_id();
    let response = ui.interact(rect, id, Sense::FOCUSABLE);
    if response.has_focus() {
        ui.memory_mut(|memory| {
            memory.set_focus_lock_filter(
                id,
                egui::EventFilter {
                    tab: false,
                    horizontal_arrows: true,
                    vertical_arrows: true,
                    escape: true,
                },
            );
        });
    }
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Other, ui.is_enabled(), "References")
    });
    ui.ctx().accesskit_node_builder(id, |node| {
        node.set_role(egui::accesskit::Role::ListBox);
        if let Some((row, _)) = current {
            node.set_active_descendant(row.accesskit_id());
        }
    });
    if theme::focus_is_visible(ui.ctx(), &response) {
        let framed = current
            .map(|(_, row)| row.intersect(rect))
            .filter(|row| row.is_positive())
            .unwrap_or(rect);
        theme::paint_focus_ring(ui, &response, framed);
    }
    response
}

/// Move over the rows, open one, or let go. The keys are taken only while
/// the table holds focus; returns whether the row stood on moved and the
/// place Enter opened.
fn keyboard(
    ui: &Ui,
    list: &mut SourceReferences,
    per_page: usize,
) -> (bool, Option<DiagnosticRoute>) {
    let id = body_id();
    if !ui.memory(|memory| memory.has_focus(id)) {
        return (false, None);
    }
    let (down, up, home, end, page_down, page_up, enter, escape) = ui.input_mut(|input| {
        let mut take = |key| input.consume_key(Modifiers::NONE, key);
        (
            take(Key::ArrowDown),
            take(Key::ArrowUp),
            take(Key::Home),
            take(Key::End),
            take(Key::PageDown),
            take(Key::PageUp),
            take(Key::Enter),
            take(Key::Escape),
        )
    });
    if down || up {
        // On the first frame the table holds focus its lock filter is not
        // yet in force, and egui has read the arrow as a move to the widget
        // beside it; the table keeps the key, so the move is cancelled.
        ui.memory_mut(|memory| memory.move_focus(egui::FocusDirection::None));
    }
    if escape {
        ui.memory_mut(|memory| memory.surrender_focus(id));
        return (false, None);
    }
    let last = list.rows.len().saturating_sub(1);
    let at = list.selected;
    let next = if home {
        0
    } else if end {
        last
    } else if page_down {
        (at + per_page).min(last)
    } else if page_up {
        at.saturating_sub(per_page)
    } else if down {
        (at + 1).min(last)
    } else if up {
        at.saturating_sub(1)
    } else {
        at
    };
    list.selected = next;
    let opened = enter
        .then(|| list.rows.get(next).map(|row| row.route.clone()))
        .flatten();
    (next != at, opened)
}
