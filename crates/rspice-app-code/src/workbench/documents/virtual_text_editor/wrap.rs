//! The rows a view lays a source's visible lines into: one a line, or, with
//! word wrap on, as many as each line needs to fit the view's width.

use std::ops::Range;

use super::super::text_document_model::TextDocumentModel;

/// The fewest cells a wrapped row holds, however narrow the view.
const MINIMUM_WRAP_CELLS: usize = 16;

/// The rows of one view.
#[derive(Clone, Debug, Default)]
pub(super) struct Rows {
    /// The cells a wrapped row holds; `None` while lines run on unwrapped.
    width: Option<usize>,
    /// What the rows were laid out for.
    laid_out_for: Option<LaidOutFor>,
    /// The first row of each visible line, and after them the row count.
    first_rows: Vec<usize>,
    /// The columns the visible lines break at, each line's in turn.
    breaks: Vec<usize>,
}

/// The text's revision, the width and the folds rows were laid out for.
#[derive(Clone, Debug, PartialEq, Eq)]
struct LaidOutFor {
    revision: u64,
    width: usize,
    folds: Vec<(usize, usize)>,
}

/// One row: the line it shows and the columns of it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct RowSpan {
    pub line: usize,
    /// The columns the row shows; a line's last row runs to its end.
    pub columns: Range<usize>,
    /// Whether the row is the line's first, which carries its number.
    pub first: bool,
    pub last: bool,
}

impl RowSpan {
    /// The column of the row nearest display cell `cell` along it.
    pub(super) fn column_at(&self, model: &TextDocumentModel, cell: f32) -> usize {
        let start_cell = if self.columns.start == 0 {
            0
        } else {
            model.cell_at(self.line, self.columns.start)
        };
        let column = model.column_nearest_cell(self.line, cell + start_cell as f32);
        // A row a line runs on from ends before its break: the break's
        // column is the start of the next row.
        if self.last {
            column.max(self.columns.start)
        } else {
            column.clamp(self.columns.start, self.columns.end.saturating_sub(1))
        }
    }
}

/// The cells a row of a view `view_width` wide holds, the gutter and the
/// margins taken off.
pub(super) fn wrap_cells(
    view_width: f32,
    code_left: f32,
    right_margin: f32,
    char_width: f32,
) -> usize {
    (((view_width - code_left - right_margin) / char_width.max(1.0))
        .floor()
        .max(0.0) as usize)
        .max(MINIMUM_WRAP_CELLS)
}

impl Rows {
    pub(super) fn wraps(&self) -> bool {
        self.width.is_some()
    }

    /// Lay the rows out `width` cells wide, or one a line for `None`, for
    /// the text as it stands.
    pub(super) fn fit(&mut self, model: &TextDocumentModel, width: Option<usize>) {
        self.width = width;
        self.refresh(model);
    }

    /// Lay the rows out again if the text or its folds changed since.
    pub(super) fn refresh(&mut self, model: &TextDocumentModel) {
        let Some(width) = self.width else {
            if self.laid_out_for.take().is_some() {
                self.first_rows = Vec::new();
                self.breaks = Vec::new();
            }
            return;
        };
        let folds = model.folds();
        if self.laid_out_for.as_ref().is_some_and(|laid| {
            laid.revision == model.revision()
                && laid.width == width
                && laid.folds.len() == folds.len()
                && laid
                    .folds
                    .iter()
                    .zip(folds)
                    .all(|(laid, fold)| *laid == (fold.start_line, fold.end_line))
        }) {
            return;
        }
        self.first_rows.clear();
        self.breaks.clear();
        let visible = model.visible_line_count();
        self.first_rows.reserve(visible + 1);
        let mut row = 0;
        for visible_row in 0..visible {
            self.first_rows.push(row);
            let before = self.breaks.len();
            if let Some(line) = model.logical_line_for_visible_row(visible_row) {
                model.wrap_columns(line, width, &mut self.breaks);
            }
            row += 1 + self.breaks.len() - before;
        }
        self.first_rows.push(row);
        self.laid_out_for = Some(LaidOutFor {
            revision: model.revision(),
            width,
            folds: folds
                .iter()
                .map(|fold| (fold.start_line, fold.end_line))
                .collect(),
        });
    }

    /// How many rows the view has.
    pub(super) fn count(&self, model: &TextDocumentModel) -> usize {
        if self.wraps() {
            self.first_rows.last().copied().unwrap_or(0)
        } else {
            model.visible_line_count()
        }
    }

    /// The columns visible line `visible_row` breaks at.
    fn breaks_of(&self, visible_row: usize) -> Option<&[usize]> {
        let first = *self.first_rows.get(visible_row)?;
        let next = *self.first_rows.get(visible_row + 1)?;
        self.breaks
            .get(first - visible_row..next.checked_sub(visible_row + 1)?)
    }

    /// The first row of visible line `visible_row`.
    pub(super) fn first_row_of(&self, visible_row: usize) -> usize {
        if self.wraps() {
            self.first_rows
                .get(visible_row)
                .copied()
                .unwrap_or_else(|| self.first_rows.last().copied().unwrap_or(0))
        } else {
            visible_row
        }
    }

    /// The row column `column` of `line` is drawn on, and the column that
    /// row starts at; `None` while the line is folded away.
    pub(super) fn row_of(
        &self,
        model: &TextDocumentModel,
        line: usize,
        column: usize,
    ) -> Option<(usize, usize)> {
        let visible_row = model.visible_row_for_logical_line(line)?;
        if !self.wraps() {
            return Some((visible_row, 0));
        }
        let breaks = self.breaks_of(visible_row)?;
        let index = breaks.partition_point(|at| *at <= column);
        Some((
            self.first_rows[visible_row] + index,
            index.checked_sub(1).map_or(0, |before| breaks[before]),
        ))
    }

    /// What row `row` shows; `None` past the last.
    pub(super) fn span(&self, model: &TextDocumentModel, row: usize) -> Option<RowSpan> {
        if !self.wraps() {
            let line = model.logical_line_for_visible_row(row)?;
            return Some(RowSpan {
                line,
                columns: 0..usize::MAX,
                first: true,
                last: true,
            });
        }
        if row >= self.count(model) {
            return None;
        }
        let visible_row = self.first_rows.partition_point(|first| *first <= row) - 1;
        let line = model.logical_line_for_visible_row(visible_row)?;
        let breaks = self.breaks_of(visible_row)?;
        let index = row - self.first_rows[visible_row];
        let start = index.checked_sub(1).map_or(0, |before| breaks[before]);
        let end = breaks.get(index).copied();
        Some(RowSpan {
            line,
            columns: start..end.unwrap_or(usize::MAX),
            first: index == 0,
            last: end.is_none(),
        })
    }
}
