//! Where each field of a title block sits, and how wide its cell is.
//!
//! A title block is a form of fixed size: each template has its dimensions
//! and its number of rows. The canvas, the plot and the Page Setup preview
//! all draw the cells laid out here rather than working out a grid of their
//! own, so a field cannot sit in one place on screen and another on paper,
//! or be cut short on one and whole on the other.
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
//! the block's edge, across any columns a short row leaves unfilled.
//!
//! The columns are as even as they can be while each is as wide as its cells
//! need, so a block with room for everything its fields typically hold is an
//! even grid. A block without that room gives way a step at a time: each
//! column keeps its fixed-form values, its document type and its captions
//! whole; then its fixed-form values and captions; then its captions alone —
//! a page number, a date or a status cut short says nothing, where long free
//! text cut short still names what it is. The columns are as even as those
//! floors allow, and only a block too narrow even for its captions shares its
//! width in proportion to them. The last column takes what integer rounding
//! leaves, so every full row spans the grid exactly.
//!
//! The layout depends on which fields a sheet shows and never on what they
//! hold, so a cell does not move while a value is typed into it, and a
//! preview that has no values draws the same grid the sheet does.
//!
//! Lettering never shrinks to fit: a value or caption longer than its cell
//! holds is cut with an ellipsis.

use super::*;

impl DrawingSheetTitleFieldId {
    /// Characters of the value this field typically holds.
    const fn typical_value_chars(self) -> u64 {
        match self {
            // Fixed forms, held whole.
            Self::Page => 8,
            Self::Revision => 3,
            Self::PaperSize => 6,
            Self::Date => 10,
            Self::DocumentStatus => longest_release_status_chars(),
            Self::Classification => 12,
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
        )
    }

    /// Width of the cell that holds this field's typical value and its
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

    /// Width of the cell that holds this field's caption whole, padding
    /// included.
    const fn title_caption_need_um(self) -> u64 {
        // Every caption is ASCII, so its length is its character count.
        self.display_label().len() as u64
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

/// One visible field's cell, in the block's upright, authored frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawingSheetTitleCell {
    pub id: DrawingSheetTitleFieldId,
    /// The row it sits in, counted from the top from zero.
    pub row: usize,
    /// Distance from the block's left edge to the cell's left rule.
    pub x_um: u64,
    pub width_um: u64,
}

impl DrawingSheetTitleCell {
    /// Characters lettered at `lettering_um` that fit between the cell's
    /// padding. IBM Plex Mono advances every glyph alike, so this is a
    /// measurement, and it holds on every renderer that letters the height.
    #[must_use]
    pub const fn capacity(self, lettering_um: u64) -> usize {
        let lettered_um = self
            .width_um
            .saturating_sub(DRAWING_SHEET_TITLE_CELL_PADDING_UM * 2);
        let chars = lettered_um / drawing_sheet_mono_advance_um(lettering_um);
        if chars == 0 { 1 } else { chars as usize }
    }

    /// The field's caption as the cell letters it.
    #[must_use]
    pub fn letter_caption(self) -> String {
        elide_title_text(
            self.id.display_label(),
            self.capacity(DRAWING_SHEET_TITLE_BLOCK_LETTERING.caption_um),
        )
    }

    /// `value` as the cell letters it: whole when it fits, and otherwise cut
    /// with an ellipsis to the characters the cell holds.
    #[must_use]
    pub fn letter_value(self, value: &str) -> String {
        elide_title_text(
            value,
            self.capacity(DRAWING_SHEET_TITLE_BLOCK_LETTERING.data_um),
        )
    }

    /// Whether the cell cuts `value` short.
    #[must_use]
    pub fn elides(self, value: &str) -> bool {
        value.chars().count() > self.capacity(DRAWING_SHEET_TITLE_BLOCK_LETTERING.data_um)
    }
}

fn elide_title_text(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_owned();
    }
    let mut kept = text
        .chars()
        .take(max_chars.saturating_sub(1))
        .collect::<String>();
    kept.push('…');
    kept
}

/// A title block's field grid: its rows, and the cell of every field the
/// sheet shows. Distances are from the top-left corner of the block in its
/// upright, authored frame; a rotated block is laid out upright and turned
/// whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawingSheetTitleLayout {
    pub width_um: u64,
    pub height_um: u64,
    /// Where the field grid starts: the width a managed logo reserves at the
    /// block's left edge, or zero.
    pub grid_x_um: u64,
    pub rows: usize,
    /// Every visible field's cell, in the order the sheet's fields resolve.
    pub cells: Vec<DrawingSheetTitleCell>,
}

impl DrawingSheetTitleLayout {
    #[must_use]
    pub const fn grid_width_um(&self) -> u64 {
        self.width_um.saturating_sub(self.grid_x_um)
    }

    /// The rule above `row`, from the block's top edge; the rule above row
    /// `rows` is the block's foot.
    #[must_use]
    pub const fn row_top_um(&self, row: usize) -> u64 {
        self.height_um * row as u64 / self.rows as u64
    }

    #[must_use]
    pub fn cell(&self, id: DrawingSheetTitleFieldId) -> Option<DrawingSheetTitleCell> {
        self.cells.iter().copied().find(|cell| cell.id == id)
    }
}

/// Lay out the title block `geometry` places on the sheet `format` authors,
/// or `None` when it carries none.
#[must_use]
pub fn drawing_sheet_title_layout(
    format: &SchematicSheetFormat,
    geometry: &DrawingSheetGeometry,
) -> Option<DrawingSheetTitleLayout> {
    let block = geometry.title_block?;
    let template = geometry.effective_title_block_template;
    let rows = format.title_block_rows(template)?.max(1);
    let (width_um, height_um) = match format.title_block.rotation {
        DrawingSheetTitleBlockRotation::Upright => (block.width_um, block.height_um),
        DrawingSheetTitleBlockRotation::Clockwise90
        | DrawingSheetTitleBlockRotation::CounterClockwise90 => (block.height_um, block.width_um),
    };
    let grid_x_um = format
        .title_block_logo(template)
        .map_or(0, DrawingSheetManagedLogo::reserved_width_um)
        .min(width_um);
    let grid_width_um = width_um - grid_x_um;
    let fields = format
        .title_block_field_order()
        .into_iter()
        .filter(|id| {
            format
                .title_block
                .fields
                .get(id)
                .is_some_and(|field| field.visible)
        })
        .collect::<Vec<_>>();
    let places = grid_places(fields.len(), rows);
    let columns = places
        .iter()
        .map(|&(_, column)| column + 1)
        .max()
        .unwrap_or(1);
    let widths = column_widths(&fields, &places, columns, grid_width_um);
    let cells = fields
        .iter()
        .zip(&places)
        .enumerate()
        .map(|(index, (&id, &(row, column)))| {
            let x_um = grid_x_um + widths[..column].iter().sum::<u64>();
            // The last cell of a row reaches the block's edge, across any
            // columns a short row leaves unfilled.
            let ends_row = places.get(index + 1).is_none_or(|&(next, _)| next != row);
            DrawingSheetTitleCell {
                id,
                row,
                x_um,
                width_um: if ends_row {
                    width_um - x_um
                } else {
                    widths[column]
                },
            }
        })
        .collect();
    Some(DrawingSheetTitleLayout {
        width_um,
        height_um,
        grid_x_um,
        rows,
        cells,
    })
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
/// it typically holds, down to 3, its caption alone.
fn kept_um(id: DrawingSheetTitleFieldId, step: usize) -> u64 {
    let whole = match step {
        0 => true,
        // Document type is a term from a short list, such as IEC 61082's
        // "Circuit diagram", and the first a crowded block lets go.
        1 => id.has_fixed_form() || id == DrawingSheetTitleFieldId::DocumentType,
        2 => id.has_fixed_form(),
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
    let floors = (0..3)
        .map(floors_at)
        .find(|floors| floors.iter().sum::<u64>() <= grid_width_um)
        .unwrap_or_else(|| floors_at(3));
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
