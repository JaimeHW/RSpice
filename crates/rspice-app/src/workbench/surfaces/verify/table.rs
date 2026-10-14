//! The one result table every Verify page reads its rows in.
//!
//! It is the grid of an ADE results view, not a card: 24 px heads and rows,
//! heads as written (sentence case, no tracking), names in sans, numbers in
//! mono and right-aligned so their digits line up, hairlines between rows and
//! columns. The only colour in a row is a verdict cell's — a failure in the
//! error tone, a value near its limit in the warning tone, a pass neutral — and
//! the accent wash of the row the reader selected. A problem the reader must
//! notice in a cell is a mark in front of neutral words, never coloured words.
//!
//! The first column's words start on the panel inset and the last column's end
//! on it, so a table's text lines up with the toolbar title above it. Columns
//! that do not fit scroll sideways rather than squeezing their numbers.

use egui::{Rect, Sense, Ui, pos2, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Problem, paint_problem_mark};

/// Height of a table's head band.
pub(super) const TABLE_HEAD_H: f32 = 24.0;
/// Height of one table row.
pub(super) const TABLE_ROW_H: f32 = 24.0;
/// Where the first column's words start and the last column's end: the panel
/// inset, so a table's text lines up with the toolbar title over it.
pub(super) const TABLE_EDGE_INSET: f32 = crate::workbench::design_system::PANEL_INSET;
/// Space between a cell's words and the rule on either side of it.
const CELL_PAD: f32 = 8.0;
/// A problem mark in a cell, and the advance from its left edge to the words.
const MARK_SIDE: f32 = 11.0;
const MARK_ADVANCE: f32 = 17.0;

/// Which edge of its cell a column's words sit on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Align {
    Left,
    Right,
}

/// One column: its head, how wide it is, and where its words sit.
#[derive(Debug, Clone)]
pub(super) struct Column {
    pub(super) head: String,
    /// What the head stands for, shown when the pointer rests on it.
    pub(super) tooltip: Option<String>,
    /// The column's width, or its least width when it takes a share of what
    /// the fixed columns leave.
    width: f32,
    /// Share of the spare width; zero for a fixed column.
    flex: f32,
    pub(super) align: Align,
}

impl Column {
    /// A column of names that takes the width the fixed columns leave.
    pub(super) fn name(head: impl Into<String>, min_width: f32) -> Self {
        Self {
            head: head.into(),
            tooltip: None,
            width: min_width,
            flex: 1.0,
            align: Align::Left,
        }
    }

    /// A fixed-width column of right-aligned numbers.
    pub(super) fn number(head: impl Into<String>, width: f32) -> Self {
        Self {
            head: head.into(),
            tooltip: None,
            width,
            flex: 0.0,
            align: Align::Right,
        }
    }

    /// A fixed-width column of left-aligned words: a limit, a verdict.
    pub(super) fn words(head: impl Into<String>, width: f32) -> Self {
        Self {
            head: head.into(),
            tooltip: None,
            width,
            flex: 0.0,
            align: Align::Left,
        }
    }

    pub(super) fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
}

/// The widths `columns` take across `available` points: fixed columns at their
/// width, the spare shared among the flexible ones by their share. When the
/// fixed widths and the flexible minimums exceed `available`, every column
/// keeps its least width and the table scrolls sideways.
pub(super) fn column_widths(columns: &[Column], available: f32) -> Vec<f32> {
    let least: f32 = columns.iter().map(|column| column.width).sum();
    let shares: f32 = columns.iter().map(|column| column.flex).sum();
    let spare = (available - least).max(0.0);
    let mut widths = columns
        .iter()
        .map(|column| {
            column.width
                + if shares > 0.0 {
                    spare * column.flex / shares
                } else {
                    0.0
                }
        })
        .collect::<Vec<_>>();
    if shares <= 0.0
        && let Some(last) = widths.last_mut()
    {
        *last += spare;
    }
    widths
}

/// A result's judgment of one value.
///
/// A failure is painted in the error tone and a value near its limit in the
/// warning tone. A pass is neutral: success is the absence of a problem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Verdict {
    Pass,
    Near,
    Fail,
}

impl Verdict {
    /// The word a pass/fail column states.
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Pass => "Pass",
            Self::Near => "Near",
            Self::Fail => "Fail",
        }
    }

    /// The worse of two verdicts.
    pub(super) fn worst(self, other: Self) -> Self {
        match (self, other) {
            (Self::Fail, _) | (_, Self::Fail) => Self::Fail,
            (Self::Near, _) | (_, Self::Near) => Self::Near,
            _ => Self::Pass,
        }
    }
}

/// How a cell's words are inked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ink {
    Plain,
    /// The metadata register: a value the run does not hold.
    Faint,
    Verdict(Verdict),
    /// A problem's mark in front of neutral words.
    Problem(Problem),
}

/// One cell's words and how they are set.
#[derive(Debug, Clone)]
pub(super) struct Cell {
    pub(super) text: String,
    mono: bool,
    ink: Ink,
}

impl Cell {
    /// Words in sans.
    pub(super) fn text(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            mono: false,
            ink: Ink::Plain,
        }
    }

    /// A number in mono.
    pub(super) fn number(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            mono: true,
            ink: Ink::Plain,
        }
    }

    /// A value the run does not hold: an em dash in the faint register.
    pub(super) fn absent() -> Self {
        Self {
            text: "\u{2014}".to_owned(),
            mono: true,
            ink: Ink::Faint,
        }
    }

    /// Nothing at all: a cell a row has no business filling.
    pub(super) fn empty() -> Self {
        Self::text("")
    }

    /// The same words, judged: coloured only when the verdict is not a pass.
    pub(super) fn judged(mut self, verdict: Verdict) -> Self {
        self.ink = Ink::Verdict(verdict);
        self
    }

    /// A pass/fail column's word for `verdict`.
    pub(super) fn verdict(verdict: Verdict) -> Self {
        Self::text(verdict.label()).judged(verdict)
    }

    /// Neutral words behind a problem's mark.
    pub(super) fn problem(text: impl Into<String>, problem: Problem) -> Self {
        Self {
            text: text.into(),
            mono: false,
            ink: Ink::Problem(problem),
        }
    }
}

/// What a table reported back this frame.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct TableResponse {
    /// The row the reader clicked.
    pub(super) clicked: Option<usize>,
}

/// One table: its columns, its rows, and where it may grow.
pub(super) struct DataTable<'a> {
    id: &'a str,
    columns: &'a [Column],
    rows: usize,
    selected: Option<usize>,
    clickable: bool,
    max_height: f32,
}

impl<'a> DataTable<'a> {
    pub(super) fn new(id: &'a str, columns: &'a [Column], rows: usize) -> Self {
        Self {
            id,
            columns,
            rows,
            selected: None,
            clickable: false,
            max_height: f32::INFINITY,
        }
    }

    /// The row painted with the selection wash.
    pub(super) fn selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    /// Rows answer a click.
    pub(super) fn clickable(mut self) -> Self {
        self.clickable = true;
        self
    }

    /// The tallest the table may be, head included; its rows scroll inside.
    pub(super) fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height;
        self
    }

    pub(super) fn show(
        self,
        ui: &mut Ui,
        mut row_at: impl FnMut(usize) -> Vec<Cell>,
    ) -> TableResponse {
        let visible = ui.available_rect_before_wrap().intersect(ui.clip_rect());
        let available = visible.width().max(1.0);
        let least: f32 = self.columns.iter().map(|column| column.width).sum();
        let width = available.max(least);
        let widths = column_widths(self.columns, width);
        let body_height = (self.max_height - TABLE_HEAD_H).max(TABLE_ROW_H);
        let mut response = TableResponse::default();
        let table = |ui: &mut Ui| {
            ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
            ui.set_min_width(width);
            self.head(ui, width, &widths);
            egui::ScrollArea::vertical()
                .id_salt((self.id, "rows"))
                .max_height(body_height)
                .auto_shrink([false, true])
                .show_rows(ui, TABLE_ROW_H, self.rows, |ui, range| {
                    ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
                    for index in range {
                        let cells = row_at(index);
                        if self.row(ui, index, width, &widths, &cells) {
                            response.clicked = Some(index);
                        }
                    }
                });
        };
        if least > available + 0.5 {
            egui::ScrollArea::horizontal()
                .id_salt((self.id, "columns"))
                .auto_shrink([false, true])
                .show(ui, table);
        } else {
            ui.scope(table);
        }
        response
    }

    fn head(&self, ui: &mut Ui, width: f32, widths: &[f32]) {
        let t = Tokens::get(ui.ctx());
        let (rect, response) = ui.allocate_exact_size(vec2(width, TABLE_HEAD_H), Sense::hover());
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Label,
                ui.is_enabled(),
                format!(
                    "Table columns: {}",
                    self.columns
                        .iter()
                        .map(|column| column.head.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            )
        });
        #[cfg(test)]
        super::raster::probe(ui, "table.head", rect);
        if !ui.is_rect_visible(rect) {
            return;
        }
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
        let mut x = rect.left();
        for (index, (column, column_width)) in self.columns.iter().zip(widths).enumerate() {
            let cell = Rect::from_min_size(pos2(x, rect.top()), vec2(*column_width, rect.height()));
            let _ = paint_words(
                ui,
                cell,
                self.text_span(index, cell),
                Words {
                    text: &column.head,
                    font: theme::sans(tokens::FS_0, FontWeight::Medium),
                    align: column.align,
                    color: t.color.text_dim,
                    problem: None,
                },
            );
            if let Some(tooltip) = &column.tooltip {
                let _ = ui
                    .interact(cell, ui.id().with((self.id, "head", index)), Sense::hover())
                    .on_hover_text(tooltip);
            }
            if index > 0 {
                ui.painter().vline(
                    cell.left(),
                    cell.y_range(),
                    egui::Stroke::new(1.0, t.color.border),
                );
            }
            x += column_width;
        }
        ui.painter().hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            egui::Stroke::new(1.0, t.color.border),
        );
    }

    /// Paint one row and report whether it was clicked.
    fn row(&self, ui: &mut Ui, index: usize, width: f32, widths: &[f32], cells: &[Cell]) -> bool {
        let t = Tokens::get(ui.ctx());
        let (rect, response) = ui.allocate_exact_size(
            vec2(width, TABLE_ROW_H),
            if self.clickable {
                Sense::click()
            } else {
                Sense::hover()
            },
        );
        let selected = self.selected == Some(index);
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
        #[cfg(test)]
        super::raster::probe(ui, "table.row", rect);
        if ui.is_rect_visible(rect) {
            if selected {
                ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
            } else if self.clickable && response.hovered() {
                ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
            }
            let mut x = rect.left();
            for (column_index, ((column, column_width), cell)) in
                self.columns.iter().zip(widths).zip(cells).enumerate()
            {
                let cell_rect =
                    Rect::from_min_size(pos2(x, rect.top()), vec2(*column_width, rect.height()));
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
                    self.text_span(column_index, cell_rect),
                    Words {
                        text: &cell.text,
                        font,
                        align: column.align,
                        color,
                        problem,
                    },
                );
                #[cfg(test)]
                if let Some(text) = _painted
                    && column.align == Align::Right
                {
                    super::raster::probe(ui, &format!("table.right.{column_index}"), text);
                }
                if column_index > 0 {
                    ui.painter().vline(
                        cell_rect.left(),
                        cell_rect.y_range(),
                        egui::Stroke::new(1.0, t.color.border),
                    );
                }
                x += column_width;
            }
            ui.painter().hline(
                rect.x_range(),
                rect.bottom() - 0.5,
                egui::Stroke::new(1.0, t.color.border),
            );
            if theme::focus_is_visible(ui.ctx(), &response) {
                theme::paint_focus_ring(ui, &response, rect);
            }
        }
        self.clickable && response.clicked()
    }

    /// The horizontal span a column's words may occupy: the cell less its
    /// padding, with the table's outer columns held to the panel inset.
    fn text_span(&self, column: usize, cell: Rect) -> egui::Rangef {
        let left = if column == 0 {
            TABLE_EDGE_INSET
        } else {
            CELL_PAD
        };
        let right = if column + 1 == self.columns.len() {
            TABLE_EDGE_INSET
        } else {
            CELL_PAD
        };
        egui::Rangef::new(
            cell.left() + left,
            (cell.right() - right).max(cell.left() + left),
        )
    }
}

/// How one cell's words are set.
struct Words<'a> {
    text: &'a str,
    font: egui::FontId,
    align: Align,
    color: egui::Color32,
    problem: Option<Problem>,
}

/// Paint one cell's words on its line, elided to `span`, and return where the
/// words landed.
fn paint_words(ui: &Ui, cell: Rect, span: egui::Rangef, words: Words<'_>) -> Option<Rect> {
    let Words {
        text,
        font,
        align,
        color,
        problem,
    } = words;
    if text.is_empty() {
        return None;
    }
    let mark_room = if problem.is_some() { MARK_ADVANCE } else { 0.0 };
    let shown = elide(ui, text, &font, (span.span() - mark_room).max(0.0));
    let galley = ui.painter().layout_no_wrap(shown, font, color);
    let text_width = galley.size().x;
    let left = match align {
        Align::Left => span.min + mark_room,
        Align::Right => span.max - text_width,
    };
    let top = cell.center().y - galley.size().y * 0.5;
    let painter = ui.painter().with_clip_rect(cell.intersect(ui.clip_rect()));
    if let Some(problem) = problem {
        paint_problem_mark(
            &painter,
            Rect::from_center_size(
                pos2(left - mark_room + MARK_SIDE * 0.5, cell.center().y),
                egui::Vec2::splat(MARK_SIDE),
            ),
            problem,
        );
    }
    let landed = Rect::from_min_size(pos2(left, top), galley.size());
    painter.galley(landed.min, galley, color);
    Some(landed)
}

/// `text` shortened with an ellipsis until it fits `max_width`.
pub(super) fn elide(ui: &Ui, text: &str, font: &egui::FontId, max_width: f32) -> String {
    crate::workbench::design_system::elide_text(ui, text, font, max_width)
}
