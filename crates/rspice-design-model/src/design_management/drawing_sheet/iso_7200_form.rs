//! ISO 7200's title block as a designed form.
//!
//! ISO 7200 fixes the block's width at 180 mm, "to fit an A4 sheet", and the
//! same block serves every paper size. Its figures are examples rather than
//! a required arrangement; RSpice's form follows Figure 2, which puts the
//! people's names on a line of their own above the rest and so leaves a
//! large cell for the legal owner and a free area at the upper right for the
//! classification:
//!
//! ```text
//! |<----- 47.5 ----->|<----- 47.5 ----->|<----- 47.5 ----->|<-- 37.5 -->|
//! | Created by       | Checked by       | Approved by      | Classific. |  row 1
//! |------------------+-------------------------+-------------------------|
//! | Legal owner      | Document type    62.5   | Document status    70    |  row 2
//! |                  |-------------------------+-------------------------|
//! |                  | Title                   | Identification number   |  row 3
//! |                  |  Sheet title            |-----+-------+------+----|
//! |                  |  Cell / view            |Rev. |Date of|Sheet |Lang|  row 4
//! |                  |                         |10.5 |issue26| 21.5 | 12 |
//! ```
//!
//! Four rows of 11.25 mm make the block 45 mm deep. The widths hold what ISO
//! 7200's table recommends each field hold (see [`iso_7200_typical_chars`])
//! at the form's lettering — captions 1.8 mm, values 2.5 mm, the
//! identification number 3.5 mm — with 2 mm of padding inside each rule: the
//! people's names 20 characters, the title and supplementary title 27, the
//! identification number 21, the date of issue 10, the sheet `12 of 12`.
//! The vertical rules stand on one grid: the legal owner's rule runs from
//! the top of the block to its foot, the title's from the second row down,
//! and each rule within a row stands at least 10 mm from any in the rows
//! beside it, so no two rules nearly meet.
//!
//! An optional field the sheet does not show merges its cell into the one on
//! its left, or the one on its right when it starts its row, so the rules
//! that remain stand where the form puts them and no rule appears anywhere
//! the form does not have one. Nothing in any other row moves. The fields
//! ISO 7200 makes mandatory are always on the form; their values may stay
//! empty until the drawing is signed.

use super::*;

use DrawingSheetTitleFieldId as Field;

/// The form's width: ISO 7200's 180 mm.
const FORM_WIDTH_UM: u64 = 180_000;
/// The form's depth: four rows.
const FORM_HEIGHT_UM: u64 = 45_000;
const ROW_UM: u64 = FORM_HEIGHT_UM / 4;

/// ISO 3098 heights the form letters at.
const CAPTION_UM: u64 = 1_800;
const VALUE_UM: u64 = 2_500;
/// The identification number stands a step above the other values, as the
/// figures of ISO 7200 set it apart.
const IDENTIFIER_UM: u64 = 3_500;

/// One cell of the form as designed.
struct FormCell {
    caption: &'static str,
    x_um: u64,
    width_um: u64,
    /// The first row the cell covers, from the top, and how many it spans.
    row: u64,
    rows: u64,
    /// What it letters, the first being the field the cell is for.
    lines: &'static [FormLine],
}

/// A field a form cell letters.
struct FormLine {
    field: DrawingSheetTitleFieldId,
    /// The row of the cell's span the value is seated on, from its first.
    row: u64,
    lettering_um: u64,
    /// Lines a long value wraps onto.
    max_lines: u8,
}

const fn line(field: DrawingSheetTitleFieldId) -> FormLine {
    FormLine {
        field,
        row: 0,
        lettering_um: VALUE_UM,
        max_lines: 1,
    }
}

/// The form, row by row and left to right.
const FORM: [FormCell; 13] = [
    FormCell {
        caption: "Created by",
        x_um: 0,
        width_um: 47_500,
        row: 0,
        rows: 1,
        lines: &[line(Field::DrawnBy)],
    },
    FormCell {
        caption: "Checked by",
        x_um: 47_500,
        width_um: 47_500,
        row: 0,
        rows: 1,
        lines: &[line(Field::CheckedBy)],
    },
    FormCell {
        caption: "Approved by",
        x_um: 95_000,
        width_um: 47_500,
        row: 0,
        rows: 1,
        lines: &[line(Field::ApprovedBy)],
    },
    FormCell {
        caption: "Classification",
        x_um: 142_500,
        width_um: 37_500,
        row: 0,
        rows: 1,
        lines: &[line(Field::Classification)],
    },
    // The legal owner's name wraps rather than being cut: the cell is as
    // large as it is to carry it.
    FormCell {
        caption: "Legal owner",
        x_um: 0,
        width_um: 47_500,
        row: 1,
        rows: 3,
        lines: &[FormLine {
            max_lines: 3,
            ..line(Field::Organization)
        }],
    },
    FormCell {
        caption: "Document type",
        x_um: 47_500,
        width_um: 62_500,
        row: 1,
        rows: 1,
        lines: &[line(Field::DocumentType)],
    },
    FormCell {
        caption: "Document status",
        x_um: 110_000,
        width_um: 70_000,
        row: 1,
        rows: 1,
        lines: &[line(Field::DocumentStatus)],
    },
    // The title, and under it the supplementary title on the value line of
    // the row beside it.
    FormCell {
        caption: "Title",
        x_um: 47_500,
        width_um: 62_500,
        row: 2,
        rows: 2,
        lines: &[
            line(Field::SheetTitle),
            FormLine {
                row: 1,
                ..line(Field::CellView)
            },
        ],
    },
    FormCell {
        caption: "Identification number",
        x_um: 110_000,
        width_um: 70_000,
        row: 2,
        rows: 1,
        lines: &[FormLine {
            lettering_um: IDENTIFIER_UM,
            ..line(Field::DocumentId)
        }],
    },
    FormCell {
        caption: "Rev.",
        x_um: 110_000,
        width_um: 10_500,
        row: 3,
        rows: 1,
        lines: &[line(Field::Revision)],
    },
    FormCell {
        caption: "Date of issue",
        x_um: 120_500,
        width_um: 26_000,
        row: 3,
        rows: 1,
        lines: &[line(Field::Date)],
    },
    FormCell {
        caption: "Sheet",
        x_um: 146_500,
        width_um: 21_500,
        row: 3,
        rows: 1,
        lines: &[line(Field::Page)],
    },
    FormCell {
        caption: "Lang.",
        x_um: 168_000,
        width_um: 12_000,
        row: 3,
        rows: 1,
        lines: &[line(Field::Language)],
    },
];

/// The fields on ISO 7200's form, in the order it is read: row by row, left
/// to right, a cell's lines top to bottom. Project and Paper size are not on
/// it — the standard has no project field, and ISO 5457 letters the paper
/// size in the border.
pub const ISO_7200_FORM_FIELDS: [DrawingSheetTitleFieldId; 14] = [
    Field::DrawnBy,
    Field::CheckedBy,
    Field::ApprovedBy,
    Field::Classification,
    Field::Organization,
    Field::DocumentType,
    Field::DocumentStatus,
    Field::SheetTitle,
    Field::CellView,
    Field::DocumentId,
    Field::Revision,
    Field::Date,
    Field::Page,
    Field::Language,
];

/// The fields ISO 7200 makes mandatory: legal owner, identification number,
/// date of issue, sheet number, title, approval person, creator and
/// document type.
pub const ISO_7200_MANDATORY_FIELDS: [DrawingSheetTitleFieldId; 8] = [
    Field::Organization,
    Field::DocumentId,
    Field::Date,
    Field::Page,
    Field::SheetTitle,
    Field::ApprovedBy,
    Field::DrawnBy,
    Field::DocumentType,
];

/// Characters ISO 7200's table recommends a field hold, where it gives a
/// number: the title 25 and the supplementary title 25 a line, the
/// identification number 16, the revision index 2, the date of issue 10, the
/// creator and approval person 20, the document type 30, the document
/// status 20, the paper size 4 and the language code (ISO 639) 2. A sheet
/// number reads `12 of 12`, the standard's sheet number and number of
/// sheets together. The standard gives no length for the legal owner or the
/// classification, and has no project field.
#[must_use]
pub const fn iso_7200_typical_chars(field: DrawingSheetTitleFieldId) -> Option<usize> {
    match field {
        Field::SheetTitle | Field::CellView => Some(25),
        Field::DocumentId => Some(16),
        Field::Revision | Field::Language => Some(2),
        Field::Date => Some(10),
        Field::Page => Some(8),
        Field::DrawnBy | Field::CheckedBy | Field::ApprovedBy | Field::DocumentStatus => Some(20),
        Field::DocumentType => Some(30),
        Field::PaperSize => Some(4),
        Field::Organization
        | Field::Classification
        | Field::Project
        | Field::Scale
        | Field::CageCode
        | Field::Engineer
        | Field::DrawnDate
        | Field::CheckedDate
        | Field::EngineerDate
        | Field::ApprovedDate
        | Field::ContractNumber => None,
    }
}

impl DrawingSheetTitleBlockTemplate {
    /// Whether a sheet on this template must show `field`: the fields every
    /// block shows, on ISO 7200's form the fields the standard makes
    /// mandatory, and on ASME Y14.1's every field of its blocks A to L, whose
    /// cells the form always draws. Page Setup locks their visibility; their
    /// values may stay empty.
    #[must_use]
    pub fn requires_field(self, field: DrawingSheetTitleFieldId) -> bool {
        field.policy().required_visible
            || (self == Self::Standard && ISO_7200_MANDATORY_FIELDS.contains(&field))
            || (self == Self::AsmeY141 && ASME_Y14_1_FORM_FIELDS.contains(&field))
    }

    /// Why Page Setup locks a field this template requires.
    #[must_use]
    pub const fn required_field_note(self) -> &'static str {
        match self {
            Self::Standard => "ISO 7200 makes this field mandatory on its form.",
            Self::AsmeY141 => "ASME Y14.1's form always carries this field.",
            Self::Compact | Self::OrganizationManaged | Self::None => {
                "Every title block carries this field."
            }
        }
    }
}

impl SchematicSheetFormat {
    /// Whether the sheet's title block shows `field`: it is on the block's
    /// form, and the sheet shows it or the form requires it. A field the
    /// form requires is shown whatever a saved sheet says, so a sheet
    /// authored before the form existed draws it complete without its saved
    /// choices being rewritten.
    #[must_use]
    pub fn title_field_shown(&self, field: DrawingSheetTitleFieldId) -> bool {
        let template = self.title_block.template;
        let on_form = match template {
            DrawingSheetTitleBlockTemplate::Standard => ISO_7200_FORM_FIELDS.contains(&field),
            DrawingSheetTitleBlockTemplate::AsmeY141 => ASME_Y14_1_FORM_FIELDS.contains(&field),
            _ => true,
        };
        if !on_form {
            return false;
        }
        template.requires_field(field)
            || self
                .title_block
                .fields
                .get(&field)
                .is_some_and(|state| state.visible)
    }

    /// Whether the sheet's paper is one ISO 5457 frames — the ISO A series
    /// and the JIS B series JIS Z 8311 frames alike. ISO 7200's block is the
    /// same on every such sheet, so it never gives way to another block on
    /// one: a drawing area too small for it carries none.
    pub(super) const fn framed_by_iso_5457(&self) -> bool {
        matches!(
            self.authored_size,
            AuthoredDrawingSheetSize::Standard { standard }
                if matches!(
                    standard.series(),
                    DrawingSheetStandardSeries::Iso | DrawingSheetStandardSeries::Jis
                )
        )
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

/// Lay out ISO 7200's form on a block `width_um` by `height_um`: the form
/// is designed at 180 × 45 mm, and a block stretched across the drawing
/// area keeps its proportions.
pub(super) fn iso_7200_form_layout(
    format: &SchematicSheetFormat,
    width_um: u64,
    height_um: u64,
) -> DrawingSheetTitleLayout {
    let shown = |field| format.title_field_shown(field);
    let across = |x_um: u64| x_um * width_um / FORM_WIDTH_UM;
    let down = |y_um: u64| y_um * height_um / FORM_HEIGHT_UM;
    let padding_um = DRAWING_SHEET_TITLE_CELL_PADDING_UM;
    let cells = place_form_cells(&shown)
        .into_iter()
        .map(|placed| {
            let cell = placed.cell;
            let (left_um, right_um) = (across(placed.x_um), across(placed.x_um + placed.width_um));
            let row_top_um = cell.row * ROW_UM;
            let (top_um, bottom_um) = (down(row_top_um), down(row_top_um + cell.rows * ROW_UM));
            let lettered_um = (right_um - left_um).saturating_sub(padding_um * 2);
            // Captions and values sit on the fractions of a row every title
            // block seats them on.
            let seat = |row: u64, (numerator, denominator): (u64, u64)| {
                down(row * ROW_UM + ROW_UM * numerator / denominator)
            };
            let (caption, values) = if placed.blank {
                (None, Vec::new())
            } else {
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
                            seat_um: seat(cell.row + line.row, DRAWING_SHEET_TITLE_VALUE_POSITION),
                            width_um: lettered_um,
                            lettering_um: line.lettering_um,
                            emphasised: line.field.emphasised(),
                            max_lines: line.max_lines,
                            // ISO 3098's least baseline spacing for lettering
                            // that may carry diacritics, 19/10 of its height.
                            line_pitch_um: down(line.lettering_um * 19 / 10),
                            upper_case: false,
                        })
                        .collect(),
                )
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

/// Every cell of the form the sheet's fields leave on it, row by row, with
/// the width each takes once hidden fields have given theirs away.
fn place_form_cells(shown: &impl Fn(DrawingSheetTitleFieldId) -> bool) -> Vec<Placed> {
    let mut placed = Vec::with_capacity(FORM.len());
    for row in 0..FORM_HEIGHT_UM / ROW_UM {
        // A cell spanning rows stands in each of them, and the cells of a
        // row share width only within the runs it divides the row into.
        let mut run = Vec::new();
        for cell in FORM
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
