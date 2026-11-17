//! One line of a table's body: a row of cells, or a group's head.
//!
//! A row is laid out across the whole width of its columns, so it scrolls with
//! them, and painted only where it shows: the frozen cells at their own place,
//! the scrolled cells in view right of the freeze edge. The open cell is a
//! field in its cell's place.

use egui::{Key, Modifiers, Rect, Sense, Ui, pos2, vec2};

use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::design_system::elide_text;

use super::geometry::Columns;
use super::{
    Align, CELL_PAD, Cell, CellEditor, Column, DataTable, EditOutcome, Ink, TABLE_EDGE_INSET,
    Verdict, View, Words, paint_words,
};

/// Side of the fold mark in front of a group's label.
const FOLD_MARK_SIDE: f32 = 12.0;

/// Where one row goes and how it is marked.
pub(super) struct RowPlace<'g> {
    /// The row's index in the page's own order.
    pub(super) index: usize,
    pub(super) view: View,
    pub(super) geometry: &'g Columns,
    pub(super) height: f32,
    pub(super) selected: bool,
    pub(super) selected_column: Option<usize>,
    /// The table has state: it, not each row, holds the keyboard.
    pub(super) stateful: bool,
}

/// What one row reported back.
#[derive(Debug, Clone, Copy)]
pub(super) struct RowResponse {
    pub(super) rect: Rect,
    pub(super) clicked: bool,
    /// The editable column the reader opened.
    pub(super) edit: Option<usize>,
    pub(super) edited: Option<EditOutcome>,
    pub(super) double_clicked: bool,
    /// Where the reader asked for the row's context menu.
    pub(super) context: Option<egui::Pos2>,
    /// The column under the pointer when the row was pressed.
    pub(super) column: Option<usize>,
}

impl DataTable<'_> {
    /// Paint one row and report what the reader did to it.
    pub(super) fn row(
        &self,
        ui: &mut Ui,
        place: RowPlace<'_>,
        cells: &[Cell],
        mut editor: Option<&mut CellEditor<'_>>,
    ) -> RowResponse {
        let t = Tokens::get(ui.ctx());
        let RowPlace {
            index,
            view,
            geometry,
            height,
            selected,
            selected_column,
            stateful,
        } = place;
        let sense = match (self.clickable, stateful) {
            (false, _) => Sense::hover(),
            // One keyboard stop for the whole table: a row is not one.
            (true, true) => Sense::CLICK,
            (true, false) => Sense::click(),
        };
        let (rect, response) =
            ui.allocate_exact_size(vec2(geometry.content_width(), height), sense);
        response.widget_info(|| {
            let spoken = self
                .columns
                .iter()
                .zip(cells)
                .filter(|(_, cell)| !cell.text.is_empty())
                .map(|(column, cell)| match cell.ink {
                    Ink::Problem(problem) => {
                        format!("{}: {}", column.head, problem.announce(&cell.text))
                    }
                    _ => format!("{}: {}", column.head, cell.text),
                })
                .collect::<Vec<_>>()
                .join(", ");
            if self.clickable {
                egui::WidgetInfo::selected(
                    egui::WidgetType::Button,
                    ui.is_enabled(),
                    selected,
                    spoken,
                )
            } else {
                egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), spoken)
            }
        });
        #[cfg(any(test, feature = "test-support"))]
        {
            super::super::raster::probe(ui, "table.row", rect);
            super::super::raster::probe(ui, &format!("table.line.{index}"), rect);
            if selected {
                super::super::raster::probe(ui, "table.selected", rect);
            }
        }
        let pressed_column = response
            .interact_pointer_pos()
            .and_then(|pointer| geometry.column_at(pointer.x - view.left, view.offset));
        let mut answered = RowResponse {
            rect,
            clicked: self.clickable && response.clicked(),
            edit: None,
            edited: None,
            double_clicked: self.clickable && response.double_clicked(),
            context: if self.clickable && response.secondary_clicked() {
                response.interact_pointer_pos()
            } else {
                None
            },
            column: pressed_column,
        };
        if !ui.is_rect_visible(rect) {
            return answered;
        }
        if selected {
            ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
        } else if self.clickable && response.hovered() {
            ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
        }
        let y = rect.y_range();
        // The open cell stays drawn, clipped away, while it is scrolled out
        // of view, so its field keeps the keyboard.
        let open_column = editor
            .as_deref()
            .map(|editor| editor.column)
            .filter(|column| {
                !geometry
                    .in_view(view.offset, view.width)
                    .any(|shown| shown == *column)
            });
        for column_index in geometry.in_view(view.offset, view.width).chain(open_column) {
            let (Some(column), Some(cell)) =
                (self.columns.get(column_index), cells.get(column_index))
            else {
                continue;
            };
            let (cell_rect, region) = view.cell(geometry, column_index, y);
            let clip = region.intersect(ui.clip_rect());
            #[cfg(any(test, feature = "test-support"))]
            super::super::raster::probe(
                ui,
                &format!("table.cell.{}", self.page_column(column_index)),
                cell_rect,
            );
            if column_index > 0 {
                ui.painter().with_clip_rect(clip).vline(
                    cell_rect.left(),
                    cell_rect.y_range(),
                    egui::Stroke::new(1.0, t.color.border),
                );
            }
            if let Some(open) = editor
                .as_deref_mut()
                .filter(|editor| editor.column == column_index)
            {
                let (outcome, onward) = self.edit_cell(ui, index, column, cell_rect, clip, open);
                answered.edited = outcome;
                if let Some(next) =
                    onward.and_then(|onward| next_editable(cells, column_index, onward))
                {
                    answered.edit = Some(next);
                }
                continue;
            }
            if cell.editable || cell.hint.is_some() {
                let cell_response = ui.interact(
                    cell_rect.intersect(clip),
                    ui.id().with((self.id, "cell", index, column_index)),
                    if cell.editable {
                        Sense::click()
                    } else {
                        Sense::hover()
                    },
                );
                if cell.editable {
                    cell_response.widget_info(|| {
                        egui::WidgetInfo::labeled(
                            egui::WidgetType::Button,
                            ui.is_enabled(),
                            format!("Edit {}: {}", column.head, cell.text),
                        )
                    });
                    if cell_response.hovered() {
                        ui.painter().with_clip_rect(clip).rect_filled(
                            cell_rect.shrink(1.0),
                            0.0,
                            t.color.bg_hover,
                        );
                        ui.ctx().set_cursor_icon(egui::CursorIcon::Text);
                    }
                    if theme::focus_is_visible(ui.ctx(), &cell_response) {
                        theme::paint_focus_ring(ui, &cell_response, cell_rect.shrink(1.0));
                    }
                    if cell_response.clicked() {
                        answered.edit = Some(column_index);
                    }
                }
                if let Some(hint) = &cell.hint {
                    let _ = cell_response.on_hover_text(hint);
                }
            }
            let font = if cell.mono {
                theme::mono(tokens::FS_0, FontWeight::Regular)
            } else {
                theme::sans(tokens::FS_0, FontWeight::Regular)
            };
            let (color, problem) = match cell.ink {
                Ink::Plain | Ink::Verdict(Verdict::Pass) => (t.color.text, None),
                Ink::Faint => (t.color.text_faint, None),
                Ink::Verdict(Verdict::Near) => (t.color.warn, None),
                Ink::Verdict(Verdict::Fail) => (t.color.err, None),
                Ink::Problem(problem) => (t.color.text_dim, Some(problem)),
            };
            let _painted = paint_words(
                ui,
                cell_rect,
                clip,
                self.text_span(column_index, cell_rect),
                Words {
                    text: &cell.text,
                    font,
                    align: column.align,
                    color,
                    problem,
                },
            );
            #[cfg(any(test, feature = "test-support"))]
            if let Some(text) = _painted
                && column.align == Align::Right
            {
                super::super::raster::probe(
                    ui,
                    &format!("table.right.{}.{column_index}", self.id),
                    text,
                );
            }
            // The row's wash is the selection. A cell is boxed only where it
            // edits, so the box says what Enter opens; a cell that only reads
            // is marked by its row alone.
            if selected_column == Some(column_index) && cell.editable {
                #[cfg(any(test, feature = "test-support"))]
                super::super::raster::probe(ui, "table.cell_box", cell_rect);
                ui.painter().with_clip_rect(clip).rect_stroke(
                    cell_rect.shrink(0.5),
                    0.0,
                    egui::Stroke::new(1.0, t.color.accent),
                    egui::StrokeKind::Inside,
                );
            }
        }
        ui.painter().hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            egui::Stroke::new(1.0, t.color.border),
        );
        if !stateful && theme::focus_is_visible(ui.ctx(), &response) {
            theme::paint_focus_ring(ui, &response, rect);
        }
        answered
    }

    /// Paint the open cell as a field in its place and report how it closed,
    /// and, when Tab or Shift+Tab closed it, which way the reader moved on.
    ///
    /// The field's words sit where the cell's did: the same mono face, the same
    /// edge, the same padding, so opening a cell moves nothing but the caret.
    fn edit_cell(
        &self,
        ui: &mut Ui,
        index: usize,
        column: &Column,
        cell: Rect,
        clip: Rect,
        editor: &mut CellEditor<'_>,
    ) -> (Option<EditOutcome>, Option<isize>) {
        let id = ui.id().with((self.id, "edit", index, editor.column));
        if editor.focus {
            ui.memory_mut(|memory| memory.request_focus(id));
            let mut state = egui::text_edit::TextEditState::load(ui.ctx(), id).unwrap_or_default();
            state
                .cursor
                .set_char_range(Some(egui::text::CCursorRange::two(
                    egui::text::CCursor::new(0),
                    egui::text::CCursor::new(editor.text.chars().count()),
                )));
            state.store(ui.ctx(), id);
        }
        // Tab moves on to the next editable cell of the row, Shift+Tab back.
        let onward = if ui.memory(|memory| memory.has_focus(id)) {
            ui.input_mut(|input| {
                if input.consume_key(Modifiers::NONE, Key::Tab) {
                    Some(1)
                } else if input.consume_key(Modifiers::SHIFT, Key::Tab) {
                    Some(-1)
                } else {
                    None
                }
            })
        } else {
            None
        };
        let field = cell.shrink(1.0);
        #[cfg(any(test, feature = "test-support"))]
        super::super::raster::probe(ui, "table.edit", field);
        let mut place = ui.new_child(egui::UiBuilder::new().max_rect(field));
        place.set_clip_rect(clip);
        let response = place.put(
            field,
            egui::TextEdit::singleline(editor.text)
                .id(id)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .horizontal_align(match column.align {
                    Align::Left => egui::Align::LEFT,
                    Align::Right => egui::Align::RIGHT,
                })
                // One point under the cell's padding, which the field's own
                // inset takes up.
                .margin(egui::Margin::symmetric(7, 2))
                .desired_width(field.width()),
        );
        if onward.is_some() {
            ui.memory_mut(|memory| memory.surrender_focus(id));
            return (Some(EditOutcome::Commit), onward);
        }
        let outcome = response.lost_focus().then(|| {
            if ui.input(|input| input.key_pressed(Key::Escape)) {
                EditOutcome::Cancel
            } else {
                EditOutcome::Commit
            }
        });
        (outcome, None)
    }

    /// Paint one group's head: its fold mark, label and the number of its
    /// rows shown, held at the viewport's left edge. Returns whether the
    /// reader folded or opened it.
    pub(super) fn group_head(
        &self,
        ui: &mut Ui,
        view: View,
        geometry: &Columns,
        height: f32,
        head: GroupHead<'_>,
    ) -> bool {
        let t = Tokens::get(ui.ctx());
        let GroupHead {
            label,
            rows,
            folded,
            foldable,
        } = head;
        let (rect, response) = ui.allocate_exact_size(
            vec2(geometry.content_width(), height),
            if foldable {
                Sense::CLICK
            } else {
                Sense::hover()
            },
        );
        let spoken = format!("{label}, {rows} {}", if rows == 1 { "row" } else { "rows" });
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                if foldable {
                    egui::WidgetType::CollapsingHeader
                } else {
                    egui::WidgetType::Label
                },
                ui.is_enabled(),
                &spoken,
            )
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_expanded(!folded);
        });
        #[cfg(any(test, feature = "test-support"))]
        super::super::raster::probe(ui, &format!("table.group.{label}"), rect);
        if ui.is_rect_visible(rect) {
            let shown = Rect::from_x_y_ranges(view.left..=view.left + view.width, rect.y_range());
            let painter = ui.painter().with_clip_rect(shown.intersect(ui.clip_rect()));
            painter.rect_filled(
                shown,
                0.0,
                if foldable && response.hovered() {
                    t.color.bg_hover
                } else {
                    t.color.bg_panel_2
                },
            );
            let mark = Rect::from_center_size(
                pos2(
                    shown.left() + TABLE_EDGE_INSET + FOLD_MARK_SIDE * 0.5,
                    shown.center().y,
                ),
                egui::Vec2::splat(FOLD_MARK_SIDE),
            );
            if foldable {
                let icon = if folded {
                    Icon::ChevronRight
                } else {
                    Icon::ChevronDown
                };
                icon.paint(&painter, mark, t.color.text_dim);
            }
            let count_font = theme::mono(tokens::FS_0, FontWeight::Regular);
            let count = painter.layout_no_wrap(rows.to_string(), count_font, t.color.text_faint);
            let words_left = if foldable {
                mark.right() + CELL_PAD * 0.5
            } else {
                shown.left() + TABLE_EDGE_INSET
            };
            let room = (shown.right() - TABLE_EDGE_INSET - count.size().x - CELL_PAD - words_left)
                .max(0.0);
            let label_font = theme::sans(tokens::FS_0, FontWeight::Medium);
            let words = elide_text(ui, label, &label_font, room);
            let words = painter.layout_no_wrap(words, label_font, t.color.text);
            let words_at = pos2(words_left, shown.center().y - words.size().y * 0.5);
            let count_at = pos2(
                words_at.x + words.size().x + CELL_PAD,
                shown.center().y - count.size().y * 0.5,
            );
            painter.galley(words_at, words, t.color.text);
            painter.galley(count_at, count, t.color.text_faint);
            painter.hline(
                shown.x_range(),
                rect.bottom() - 0.5,
                egui::Stroke::new(1.0, t.color.border),
            );
        }
        foldable && response.clicked()
    }
}

/// A group head's words and whether a click folds it.
pub(super) struct GroupHead<'a> {
    pub(super) label: &'a str,
    /// How many of its rows the filter keeps.
    pub(super) rows: usize,
    pub(super) folded: bool,
    pub(super) foldable: bool,
}

/// The next editable cell of `cells` from `column`, one way or the other.
fn next_editable(cells: &[Cell], column: usize, onward: isize) -> Option<usize> {
    if onward > 0 {
        (column + 1..cells.len()).find(|index| cells[*index].editable)
    } else {
        (0..column).rev().find(|index| cells[*index].editable)
    }
}
