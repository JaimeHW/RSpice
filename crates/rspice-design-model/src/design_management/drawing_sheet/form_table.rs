//! A title block drawn as a designed form: a table of cells, each with its
//! caption and the fields it letters, laid out on a block of any size.
//!
//! ISO 7200's form and the compact form are both such tables (see the
//! `iso_7200_form` and `compact_form` modules). A form letters its captions
//! one of two ways: over the value, on a line of their own, or inline before
//! it on the value's line.
//!
//! An optional field the sheet does not show merges its cell into the one on
//! its left, or the one on its right when it starts its row, so the rules
//! that remain stand where the form puts them and no rule appears anywhere
//! the form does not have one. Nothing in any other row moves.

use super::*;

/// ISO 3098 heights a form letters at: captions a step below the values
/// they name.
pub(super) const CAPTION_UM: u64 = 1_800;
pub(super) const VALUE_UM: u64 = 2_500;

/// A form as designed: its size, its rows and its cells.
pub(super) struct FormTable {
    pub(super) width_um: u64,
    pub(super) height_um: u64,
    /// Rows of equal height the form is drawn on.
    pub(super) rows: u64,
    /// Where every cell letters its caption.
    pub(super) captions: DrawingSheetTitleCaptionPlacement,
    /// The cells, row by row and left to right.
    pub(super) cells: &'static [FormCell],
}

impl FormTable {
    const fn row_um(&self) -> u64 {
        self.height_um / self.rows
    }
}

/// One cell of a form as designed.
pub(super) struct FormCell {
    pub(super) caption: &'static str,
    pub(super) x_um: u64,
    pub(super) width_um: u64,
    /// The first row the cell covers, from the top, and how many it spans.
    pub(super) row: u64,
    pub(super) rows: u64,
    /// What it letters, the first being the field the cell is for.
    pub(super) lines: &'static [FormLine],
}

/// A field a form cell letters.
pub(super) struct FormLine {
    pub(super) field: DrawingSheetTitleFieldId,
    /// The row of the cell's span the value is seated on, from its first.
    pub(super) row: u64,
    pub(super) lettering_um: u64,
    /// Whether the value is set in the heavier weight.
    pub(super) emphasised: bool,
    /// Lines a long value wraps onto.
    pub(super) max_lines: u8,
}

/// A line lettering `field` at the form's value height, on the first row of
/// its cell.
pub(super) const fn line(field: DrawingSheetTitleFieldId) -> FormLine {
    FormLine {
        field,
        row: 0,
        lettering_um: VALUE_UM,
        emphasised: field.emphasised(),
        max_lines: 1,
    }
}

/// A form cell where the sheet's fields place it, in the form's own frame.
struct Placed {
    cell: &'static FormCell,
    x_um: u64,
    width_um: u64,
    /// Whether the cell letters nothing: a run of hidden fields with no cell
    /// beside it in the row to take its width.
    blank: bool,
}

/// Lay out `table` on a block `width_um` by `height_um`: a block stretched
/// across the drawing area keeps the form's proportions, and its lettering
/// keeps its heights.
pub(super) fn form_layout(
    table: &FormTable,
    format: &SchematicSheetFormat,
    width_um: u64,
    height_um: u64,
) -> DrawingSheetTitleLayout {
    let shown = |field| format.title_field_shown(field);
    let across = |x_um: u64| x_um * width_um / table.width_um;
    let down = |y_um: u64| y_um * height_um / table.height_um;
    let row_um = table.row_um();
    let padding_um = DRAWING_SHEET_TITLE_CELL_PADDING_UM;
    let cells = place_form_cells(table, &shown)
        .into_iter()
        .map(|placed| {
            let cell = placed.cell;
            let (left_um, right_um) = (across(placed.x_um), across(placed.x_um + placed.width_um));
            let row_top_um = cell.row * row_um;
            let (top_um, bottom_um) = (down(row_top_um), down(row_top_um + cell.rows * row_um));
            let lettered_um = (right_um - left_um).saturating_sub(padding_um * 2);
            let (caption, values) = if placed.blank {
                (None, Vec::new())
            } else {
                match table.captions {
                    DrawingSheetTitleCaptionPlacement::Over => {
                        // Captions and values sit on the fractions of a row
                        // every title block seats them on.
                        let seat = |row: u64, (numerator, denominator): (u64, u64)| {
                            down(row * row_um + row_um * numerator / denominator)
                        };
                        (
                            Some(DrawingSheetTitleCaption {
                                text: cell.caption,
                                placement: DrawingSheetTitleCaptionPlacement::Over,
                                x_um: left_um + padding_um,
                                seat_um: seat(cell.row, DRAWING_SHEET_TITLE_CAPTION_POSITION),
                                width_um: lettered_um,
                                lettering_um: CAPTION_UM,
                            }),
                            cell.lines
                                .iter()
                                .filter(|line| shown(line.field))
                                .map(|line| DrawingSheetTitleValue {
                                    text: DrawingSheetTitleText::Field(line.field),
                                    x_um: left_um + padding_um,
                                    seat_um: seat(
                                        cell.row + line.row,
                                        DRAWING_SHEET_TITLE_VALUE_POSITION,
                                    ),
                                    width_um: lettered_um,
                                    lettering_um: line.lettering_um,
                                    emphasised: line.emphasised,
                                    max_lines: line.max_lines,
                                    // ISO 3098's least baseline spacing for
                                    // lettering that may carry diacritics,
                                    // 19/10 of its height.
                                    line_pitch_um: down(line.lettering_um * 19 / 10),
                                    upper_case: false,
                                })
                                .collect(),
                        )
                    }
                    DrawingSheetTitleCaptionPlacement::Inline => {
                        inline_lines(cell, &shown, left_um, right_um, top_um, bottom_um)
                    }
                }
            };
            DrawingSheetTitleCell {
                x_um: left_um,
                y_um: top_um,
                width_um: right_um - left_um,
                height_um: bottom_um - top_um,
                caption,
                values,
            }
        })
        .collect();
    DrawingSheetTitleLayout::from_cells(width_um, height_um, 0, cells)
}

/// A one-row cell whose caption is lettered before its value, one caption
/// space ahead of it, both on the baseline that centres the value's capitals
/// in the row: `Title: …` reads as one line, and the value has the rest of
/// the cell to itself.
fn inline_lines(
    cell: &FormCell,
    shown: &impl Fn(DrawingSheetTitleFieldId) -> bool,
    left_um: u64,
    right_um: u64,
    top_um: u64,
    bottom_um: u64,
) -> (
    Option<DrawingSheetTitleCaption>,
    Vec<DrawingSheetTitleValue>,
) {
    let padding_um = DRAWING_SHEET_TITLE_CELL_PADDING_UM;
    let start_um = left_um + padding_um;
    let end_um = right_um.saturating_sub(padding_um);
    let caption_advance_um = drawing_sheet_mono_advance_um(CAPTION_UM);
    let caption_width_um =
        (caption_advance_um * cell.caption.chars().count() as u64).min(end_um - start_um);
    let caption = DrawingSheetTitleCaption {
        text: cell.caption,
        placement: DrawingSheetTitleCaptionPlacement::Inline,
        x_um: start_um,
        seat_um: drawing_sheet_seat_on_centred_baseline(top_um, bottom_um, VALUE_UM, CAPTION_UM),
        width_um: caption_width_um,
        lettering_um: CAPTION_UM,
    };
    let value_x_um = (start_um + caption_width_um + caption_advance_um).min(end_um);
    let values = cell
        .lines
        .iter()
        .filter(|line| shown(line.field))
        .map(|line| DrawingSheetTitleValue {
            text: DrawingSheetTitleText::Field(line.field),
            x_um: value_x_um,
            seat_um: drawing_sheet_seat_on_centred_baseline(
                top_um,
                bottom_um,
                VALUE_UM,
                line.lettering_um,
            ),
            width_um: end_um - value_x_um,
            lettering_um: line.lettering_um,
            emphasised: line.emphasised,
            max_lines: 1,
            line_pitch_um: 0,
            upper_case: false,
        })
        .collect();
    (Some(caption), values)
}

/// Every cell of `table` the sheet's fields leave on it, row by row, with
/// the width each takes once hidden fields have given theirs away.
fn place_form_cells(
    table: &FormTable,
    shown: &impl Fn(DrawingSheetTitleFieldId) -> bool,
) -> Vec<Placed> {
    let mut placed = Vec::with_capacity(table.cells.len());
    for row in 0..table.rows {
        // A cell spanning rows stands in each of them, and the cells of a
        // row share width only within the runs it divides the row into.
        let mut run = Vec::new();
        for cell in table
            .cells
            .iter()
            .filter(|cell| cell.row <= row && row < cell.row + cell.rows)
        {
            if cell.rows == 1 {
                run.push(cell);
                continue;
            }
            share_run(&run, shown, &mut placed);
            run.clear();
            if cell.row == row {
                placed.push(Placed {
                    cell,
                    x_um: cell.x_um,
                    width_um: cell.width_um,
                    blank: false,
                });
            }
        }
        share_run(&run, shown, &mut placed);
    }
    placed
}

/// Place a run of side-by-side cells in one row: each hidden cell merges
/// into the shown cell on its left, or on its right when it starts the row,
/// so every rule that remains stands where the form puts it.
fn share_run(
    run: &[&'static FormCell],
    shown: &impl Fn(DrawingSheetTitleFieldId) -> bool,
    placed: &mut Vec<Placed>,
) {
    let Some(first) = run.first() else {
        return;
    };
    let visible = run
        .iter()
        .map(|cell| shown(cell.lines[0].field))
        .collect::<Vec<_>>();
    if !visible.contains(&true) {
        placed.extend(run.iter().map(|cell| Placed {
            cell,
            x_um: cell.x_um,
            width_um: cell.width_um,
            blank: true,
        }));
        return;
    }
    let mut widths = run.iter().map(|cell| cell.width_um).collect::<Vec<_>>();
    let mut index = 0;
    while index < run.len() {
        if visible[index] {
            index += 1;
            continue;
        }
        let start = index;
        while index < run.len() && !visible[index] {
            index += 1;
        }
        let given_um = run[start..index]
            .iter()
            .map(|cell| cell.width_um)
            .sum::<u64>();
        // Into the cell on the left, or the one on the right for the first
        // cells of a row; a run that is hidden whole has none and stays
        // blank.
        if let Some(neighbour) = start
            .checked_sub(1)
            .or((index < run.len()).then_some(index))
        {
            widths[neighbour] += given_um;
        }
    }
    let mut x_um = first.x_um;
    for ((cell, width_um), visible) in run.iter().zip(widths).zip(visible) {
        if visible {
            placed.push(Placed {
                cell,
                x_um,
                width_um,
                blank: false,
            });
            x_um += width_um;
        }
    }
}
