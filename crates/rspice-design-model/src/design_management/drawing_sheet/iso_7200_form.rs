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
//! 7200's table recommends each field hold (see `iso_7200_typical_chars`)
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

use super::form_table::*;
use super::*;

use DrawingSheetTitleFieldId as Field;

/// The form's width: ISO 7200's 180 mm.
const FORM_WIDTH_UM: u64 = 180_000;
/// The form's depth: four rows.
const FORM_HEIGHT_UM: u64 = 45_000;

/// The identification number stands a step above the other values, as the
/// figures of ISO 7200 set it apart.
const IDENTIFIER_UM: u64 = 3_500;

/// The form: four rows of 11.25 mm, captions over their values.
const FORM: FormTable = FormTable {
    width_um: FORM_WIDTH_UM,
    height_um: FORM_HEIGHT_UM,
    rows: 4,
    captions: DrawingSheetTitleCaptionPlacement::Over,
    cells: &FORM_CELLS,
};

/// The form's cells, row by row and left to right.
const FORM_CELLS: [FormCell; 13] = [
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
#[cfg(test)]
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
    /// mandatory, on ASME Y14.1's every field of its blocks A to L, whose
    /// cells the form always draws, and on the compact form every field it
    /// carries. No sheet hides them; their values may stay empty.
    #[must_use]
    pub fn requires_field(self, field: DrawingSheetTitleFieldId) -> bool {
        field.policy().required_visible
            || (self == Self::Standard && ISO_7200_MANDATORY_FIELDS.contains(&field))
            || (self == Self::Compact && COMPACT_FORM_FIELDS.contains(&field))
            || (self == Self::AsmeY141 && ASME_Y14_1_FORM_FIELDS.contains(&field))
    }

    /// Why a sheet cannot hide a field this template requires.
    #[must_use]
    pub const fn required_field_note(self) -> &'static str {
        match self {
            Self::Standard => "ISO 7200 makes this field mandatory on its form.",
            Self::Compact => "The compact block always carries this field.",
            Self::AsmeY141 => "ASME Y14.1's form always carries this field.",
            Self::OrganizationManaged | Self::None => "Every title block carries this field.",
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
            DrawingSheetTitleBlockTemplate::Compact => COMPACT_FORM_FIELDS.contains(&field),
            DrawingSheetTitleBlockTemplate::AsmeY141 => {
                ASME_Y14_1_FORM_FIELDS.contains(&field)
                    || ASME_Y14_1_SUPPLEMENTARY_FIELDS.contains(&field)
            }
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
}

/// Lay out ISO 7200's form on a block `width_um` by `height_um`: the form
/// is designed at 180 × 45 mm, and a block stretched across the drawing
/// area keeps its proportions.
pub(super) fn iso_7200_form_layout(
    format: &SchematicSheetFormat,
    width_um: u64,
    height_um: u64,
) -> DrawingSheetTitleLayout {
    form_layout(&FORM, format, width_um, height_um)
}
