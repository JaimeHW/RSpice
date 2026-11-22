//! The compact title block: what a schematic's reader looks for, in a block
//! about the size of KiCad's default one.
//!
//! ISO 7200's 180 × 45 mm block takes two thirds of an A4 sheet's width,
//! and ASME Y14.1's is as large, because both are forms for formal release.
//! A schematic needs its title, where it lives, who owns it, its number,
//! revision and date, who drew it and which sheet of how many. The compact
//! block holds those in 110 × 32 mm, five rows of 6.4 mm, each caption
//! lettered inline before its value so a full-width row gives the value
//! nearly all of its width:
//!
//! ```text
//! |<---------------------------- 110 ---------------------------->|
//! | Title: …                                                      |  row 1
//! | Cell: …                                                       |  row 2
//! | Owner: …                                                      |  row 3
//! | Number: …                        | Rev: …  | Date: …          |  row 4
//! | Created by: …                              | Sheet: …         |  row 5
//! |<-------------- 54 -------------->|<- 21 ->|<------ 35 ------->|
//! ```
//!
//! At the form's lettering, captions 1.8 mm and values 2.5 mm with 2 mm of
//! padding inside each rule, the title holds 44 characters, the cell / view
//! path 45 and the legal owner 44; the identification number 17, the
//! revision 4, the date of issue 10, the creator 24 and the sheet `12 of
//! 12`. The rule between the revision and the date runs on as the rule
//! before the sheet, so the two right-hand cells stand in one column.
//!
//! Every field on the form is always on it, its value empty until someone
//! fills it in: a compact block that dropped a row would no longer be the
//! size it says.

use super::form_table::*;
use super::*;

use DrawingSheetTitleFieldId as Field;

/// The form's width and depth.
const COMPACT_FORM_WIDTH_UM: u64 = 110_000;
const COMPACT_FORM_HEIGHT_UM: u64 = 32_000;

/// The form: five rows, captions inline before their values.
const FORM: FormTable = FormTable {
    width_um: COMPACT_FORM_WIDTH_UM,
    height_um: COMPACT_FORM_HEIGHT_UM,
    rows: 5,
    captions: DrawingSheetTitleCaptionPlacement::Inline,
    cells: &FORM_CELLS,
};

/// A cell across the whole form.
const fn full_row(caption: &'static str, row: u64, lines: &'static [FormLine]) -> FormCell {
    FormCell {
        caption,
        x_um: 0,
        width_um: COMPACT_FORM_WIDTH_UM,
        row,
        rows: 1,
        lines,
    }
}

/// The form's cells, row by row and left to right.
const FORM_CELLS: [FormCell; 8] = [
    // The title is what the sheet is read by, so it is set in the heavier
    // weight.
    full_row(
        "Title:",
        0,
        &[FormLine {
            emphasised: true,
            ..line(Field::SheetTitle)
        }],
    ),
    full_row("Cell:", 1, &[line(Field::CellView)]),
    full_row("Owner:", 2, &[line(Field::Organization)]),
    FormCell {
        caption: "Number:",
        x_um: 0,
        width_um: 54_000,
        row: 3,
        rows: 1,
        lines: &[line(Field::DocumentId)],
    },
    FormCell {
        caption: "Rev:",
        x_um: 54_000,
        width_um: 21_000,
        row: 3,
        rows: 1,
        lines: &[line(Field::Revision)],
    },
    FormCell {
        caption: "Date:",
        x_um: 75_000,
        width_um: 35_000,
        row: 3,
        rows: 1,
        lines: &[line(Field::Date)],
    },
    FormCell {
        caption: "Created by:",
        x_um: 0,
        width_um: 75_000,
        row: 4,
        rows: 1,
        lines: &[line(Field::DrawnBy)],
    },
    FormCell {
        caption: "Sheet:",
        x_um: 75_000,
        width_um: 35_000,
        row: 4,
        rows: 1,
        lines: &[line(Field::Page)],
    },
];

/// The fields on the compact form, in the order it is read: row by row,
/// left to right.
pub const COMPACT_FORM_FIELDS: [DrawingSheetTitleFieldId; 8] = [
    Field::SheetTitle,
    Field::CellView,
    Field::Organization,
    Field::DocumentId,
    Field::Revision,
    Field::Date,
    Field::DrawnBy,
    Field::Page,
];

/// Lay out the compact form on a block `width_um` by `height_um`.
pub(super) fn compact_form_layout(
    format: &SchematicSheetFormat,
    width_um: u64,
    height_um: u64,
) -> DrawingSheetTitleLayout {
    form_layout(&FORM, format, width_um, height_um)
}
