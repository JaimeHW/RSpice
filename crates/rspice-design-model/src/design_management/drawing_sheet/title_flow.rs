//! The flowed grid the compact and organization blocks draw: the fields a
//! sheet shows, poured into straight columns.
//!
//! ISO 7200's block is a designed form (see the `iso_7200_form` module).
//! The compact block is kept only so the sheets saved on it load, and an
//! organization's block is laid out from the field order it was sealed
//! with; both are drawn exactly as they were before the form existed, by the
//! rule below.
//!
//! # What a cell needs
//!
//! Each field needs the width that holds what it typically holds, lettered at
//! [`DRAWING_SHEET_TITLE_BLOCK_LETTERING`]: its typical value at the value
//! height or its caption at the caption height, whichever is wider, plus
//! [`DRAWING_SHEET_TITLE_CELL_PADDING_UM`] on either side. A field whose
//! value has a fixed form needs all of it — Page `12 of 12`, a `YYYY-MM-DD`
//! date, the longest release status, a paper size such as `ANSI B` — so a
//! cell given its need never cuts one short. A free-text field needs a
//! typical length chosen for it, and a longer value is elided.
//!
//! # How the grid is drawn
//!
//! The cells form columns whose rules run straight down the block. Fields
//! keep the order the sheet lists them in and fill the rows from the top,
//! left to right, as many to a row as it takes to seat them all, while
//! leaving every row at least one: a sheet showing fewer fields than the
//! template has rows gives each its own row. The last cell of a row reaches
//! the block's edge, across any columns a short row leaves unfilled, and a
//! row left without a field is a blank strip across the grid.
//!
//! The columns are as even as they can be while each is as wide as its cells
//! need, so a block with room for everything its fields typically hold is an
//! even grid. A block without that room gives way a step at a time: each
//! column keeps its fixed-form values, its document type and its captions
//! whole; then its fixed-form values and captions; then those but the
//! document status; then its captions alone — a page number or a date cut
//! short says nothing, where long free text cut short still names what it
//! is. The columns are as even as those
//! floors allow, and only a block too narrow even for its captions shares its
//! width in proportion to them. The last column takes what integer rounding
//! leaves, so every full row spans the grid exactly.

use super::*;

impl DrawingSheetTitleFieldId {
    /// Characters of the value this field typically holds in a flowed grid.
    /// The lengths are the ones these blocks were sized with before ISO
    /// 7200's form, and stay so that they draw as they always did; the form
    /// is sized to the standard's own (see [`iso_7200_typical_chars`]).
    const fn typical_value_chars(self) -> u64 {
        match self {
            // Fixed forms, held whole.
            Self::Page => 8,
            Self::Revision => 3,
            // The longest designation, ARCH E1.
            Self::PaperSize => 7,
            Self::Date => 10,
            Self::DocumentStatus => longest_release_status_chars(),
            Self::Classification => 12,
            Self::Language => 3,
            // Free text. A project name is a short identifier, and in a
            // single-library workspace it also leads `Cell / view`.
            Self::Project => 10,
            Self::CellView | Self::SheetTitle => 14,
            Self::DocumentType => DRAWING_SHEET_DEFAULT_DOCUMENT_TYPE.len() as u64,
            Self::DrawnBy | Self::CheckedBy | Self::ApprovedBy => 16,
            Self::Organization => 16,
            // ISO 7200's document identifier is 16 characters.
            Self::DocumentId => 16,
            Self::Scale => 3,
        }
    }

    /// Whether the field's value has a fixed form, held whole by a cell given
    /// its need, rather than free text a long value overruns.
    const fn has_fixed_form(self) -> bool {
        matches!(
            self,
            Self::Page
                | Self::Revision
                | Self::PaperSize
                | Self::Date
                | Self::DocumentStatus
                | Self::Classification
                | Self::Language
        )
    }

    /// Width of the flowed cell that holds this field's typical value and its
    /// caption whole, padding included, as the module documentation sets
    /// out.
    #[must_use]
    pub const fn title_cell_need_um(self) -> u64 {
        let value_um = self.typical_value_chars()
            * drawing_sheet_mono_advance_um(DRAWING_SHEET_TITLE_BLOCK_LETTERING.data_um)
            + DRAWING_SHEET_TITLE_CELL_PADDING_UM * 2;
        let caption_um = self.title_caption_need_um();
        if value_um > caption_um {
            value_um
        } else {
            caption_um
        }
    }

    /// Width of the flowed cell that holds this field's caption whole,
    /// padding included.
    const fn title_caption_need_um(self) -> u64 {
        // Every caption is ASCII, so its length is its character count.
        self.grid_caption().len() as u64
            * drawing_sheet_mono_advance_um(DRAWING_SHEET_TITLE_BLOCK_LETTERING.caption_um)
            + DRAWING_SHEET_TITLE_CELL_PADDING_UM * 2
    }
}

const fn longest_release_status_chars() -> u64 {
    let mut longest = 0;
    let mut index = 0;
    while index < DrawingSheetReleaseStatus::ALL.len() {
        let chars = DrawingSheetReleaseStatus::ALL[index].label().len();
        if chars > longest {
            longest = chars;
        }
        index += 1;
    }
    longest as u64
}

/// Lay out the flowed grid of `template` on a block `width_um` by
/// `height_um`, by the rule the module documentation sets out.
pub(super) fn flowed_title_layout(
    format: &SchematicSheetFormat,
    template: DrawingSheetTitleBlockTemplate,
    width_um: u64,
    height_um: u64,
) -> Option<DrawingSheetTitleLayout> {
    let rows = format.title_block_rows(template)?.max(1);
    let grid_x_um = format
        .title_block_logo(template)
        .map_or(0, DrawingSheetManagedLogo::reserved_width_um)
        .min(width_um);
    let grid_width_um = width_um - grid_x_um;
    let fields = format
        .title_block_field_order()
        .into_iter()
        .filter(|id| format.title_field_shown(*id))
        .collect::<Vec<_>>();
    let places = grid_places(fields.len(), rows);
    let columns = places
        .iter()
        .map(|&(_, column)| column + 1)
        .max()
        .unwrap_or(1);
    let widths = column_widths(&fields, &places, columns, grid_width_um);
    let row_top_um = |row: usize| height_um * row as u64 / rows as u64;
    let lettering = DRAWING_SHEET_TITLE_BLOCK_LETTERING;
    let padding_um = DRAWING_SHEET_TITLE_CELL_PADDING_UM;
    let mut cells = fields
        .iter()
        .zip(&places)
        .enumerate()
        .map(|(index, (&id, &(row, column)))| {
            let x_um = grid_x_um + widths[..column].iter().sum::<u64>();
            // The last cell of a row reaches the block's edge, across any
            // columns a short row leaves unfilled.
            let ends_row = places.get(index + 1).is_none_or(|&(next, _)| next != row);
            let cell_width_um = if ends_row {
                width_um - x_um
            } else {
                widths[column]
            };
            let (y_um, bottom_um) = (row_top_um(row), row_top_um(row + 1));
            let cell_height_um = bottom_um - y_um;
            let seat = |(numerator, denominator): (u64, u64)| {
                y_um + cell_height_um * numerator / denominator
            };
            let lettered_um = cell_width_um.saturating_sub(padding_um * 2);
            DrawingSheetTitleCell {
                x_um,
                y_um,
                width_um: cell_width_um,
                height_um: cell_height_um,
                caption: Some(DrawingSheetTitleCaption {
                    text: id.grid_caption(),
                    placement: DrawingSheetTitleCaptionPlacement::Over,
                    x_um: x_um + padding_um,
                    seat_um: seat(DRAWING_SHEET_TITLE_CAPTION_POSITION),
                    width_um: lettered_um,
                    lettering_um: lettering.caption_um,
                }),
                values: vec![DrawingSheetTitleValue {
                    text: DrawingSheetTitleText::Field(id),
                    x_um: x_um + padding_um,
                    seat_um: seat(DRAWING_SHEET_TITLE_VALUE_POSITION),
                    width_um: lettered_um,
                    lettering_um: lettering.data_um,
                    emphasised: id.emphasised(),
                    max_lines: 1,
                    line_pitch_um: 0,
                }],
            }
        })
        .collect::<Vec<_>>();
    // A row the fields leave empty is a strip across the grid, ruled as the
    // rows above it are; a block with no field to show is its outline alone.
    let filled = places.last().map_or(0, |&(row, _)| row + 1);
    if filled > 0 {
        cells.extend((filled..rows).map(|row| DrawingSheetTitleCell {
            x_um: grid_x_um,
            y_um: row_top_um(row),
            width_um: grid_width_um,
            height_um: row_top_um(row + 1) - row_top_um(row),
            caption: None,
            values: Vec::new(),
        }));
    }
    Some(DrawingSheetTitleLayout::from_cells(
        width_um, height_um, grid_x_um, cells,
    ))
}

/// The row and column of each of `count` fields on a grid of `rows` rows,
/// by the rule the module documentation sets out.
fn grid_places(count: usize, rows: usize) -> Vec<(usize, usize)> {
    let columns = count.div_ceil(rows).max(1);
    let mut places = Vec::with_capacity(count);
    let mut remaining = count;
    for row in 0..rows {
        // As many as a row holds, while leaving each row below one of its own.
        let taken = columns
            .min(remaining.saturating_sub(rows - row - 1))
            .max(1)
            .min(remaining);
        places.extend((0..taken).map(|column| (row, column)));
        remaining -= taken;
    }
    places
}

/// The width a column keeps for `id` at `step` of giving way: from 0, all
/// it typically holds, down to 4, its caption alone.
fn kept_um(id: DrawingSheetTitleFieldId, step: usize) -> u64 {
    let whole = match step {
        0 => true,
        // Document type is a term from a short list, such as IEC 61082's
        // "Circuit diagram", and the first a crowded block lets go.
        1 => id.has_fixed_form() || id == DrawingSheetTitleFieldId::DocumentType,
        2 => id.has_fixed_form(),
        // ISO 7200's status wording is the longest fixed form, and a status
        // cut short still names where the document stands, where a page
        // number or a date cut short says nothing.
        3 => id.has_fixed_form() && id != DrawingSheetTitleFieldId::DocumentStatus,
        _ => false,
    };
    if whole {
        id.title_cell_need_um()
    } else {
        id.title_caption_need_um()
    }
}

/// The width of each column, by the rule the module documentation sets out.
fn column_widths(
    fields: &[DrawingSheetTitleFieldId],
    places: &[(usize, usize)],
    columns: usize,
    grid_width_um: u64,
) -> Vec<u64> {
    let floors_at = |step| {
        let mut floors = vec![0_u64; columns];
        for (&id, &(_, column)) in fields.iter().zip(places) {
            floors[column] = floors[column].max(kept_um(id, step));
        }
        floors
    };
    let floors = (0..4)
        .map(floors_at)
        .find(|floors| floors.iter().sum::<u64>() <= grid_width_um)
        .unwrap_or_else(|| floors_at(4));
    level_widths(&floors, grid_width_um)
}

/// Widths that span `total_um` exactly and are as even as they can be while
/// none is narrower than its floor: every column below a common level is
/// raised to it. Floors that overrun `total_um` share it in proportion.
fn level_widths(floors: &[u64], total_um: u64) -> Vec<u64> {
    let floor_um = floors.iter().sum::<u64>();
    let mut widths = if floor_um > total_um {
        floors
            .iter()
            .map(|floor| total_um * floor / floor_um)
            .collect::<Vec<_>>()
    } else {
        // Columns whose floor stands above the level keep it; the rest share
        // what is left alike.
        let mut descending = floors.to_vec();
        descending.sort_unstable_by(|left, right| right.cmp(left));
        let (mut remaining_um, mut sharing) = (total_um, floors.len() as u64);
        for floor in descending {
            if floor * sharing <= remaining_um {
                break;
            }
            remaining_um -= floor;
            sharing -= 1;
        }
        let level_um = remaining_um / sharing.max(1);
        floors
            .iter()
            .map(|floor| (*floor).max(level_um))
            .collect::<Vec<_>>()
    };
    if let Some((last, others)) = widths.split_last_mut() {
        *last = total_um - others.iter().sum::<u64>();
    }
    widths
}
