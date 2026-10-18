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
//! # How the fields are split into rows
//!
//! Fields keep the order the sheet lists them in and fill the rows from the
//! top, each row taking the next run of them. A row with room for its cells'
//! needs gives every cell a share of the field grid in proportion to its
//! need, so the width to spare is spread across the row. A row without room
//! gives each fixed-form field its whole need first — a page number, a date
//! or a status cut short says nothing — and the free-text fields share what
//! is left in proportion to theirs; only a row whose fixed-form fields alone
//! overrun the grid shares all of it in proportion. Either way the last cell
//! takes what integer rounding leaves, so a row spans the grid exactly.
//!
//! Of every way to cut the list into the template's rows, the layout takes
//! the one whose fullest row overruns the grid by least. That is no overrun
//! at all whenever some cut lets every row fit, and then no cell is narrower
//! than its need. Among cuts that tie, it takes the one whose row needs are
//! most even — the least sum of their squares — so the width a block has to
//! spare is shared alike rather than heaped on one row; and among cuts that
//! tie on both, the one that fills its earlier rows first. A sheet showing
//! fewer fields than the template has rows gives each field a row of its own
//! and leaves the rest empty.
//!
//! The layout depends on which fields a sheet shows and never on what they
//! hold, so a cell does not move while a value is typed into it, and a
//! preview that has no values draws the same grid the sheet does.
//!
//! Lettering never shrinks to fit: a value or caption longer than its cell
//! holds is cut with an ellipsis.

use std::ops::Range;

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
        let lettering = DRAWING_SHEET_TITLE_BLOCK_LETTERING;
        let value_um =
            self.typical_value_chars() * drawing_sheet_mono_advance_um(lettering.data_um);
        // Every caption is ASCII, so its length is its character count.
        let caption_um =
            self.display_label().len() as u64 * drawing_sheet_mono_advance_um(lettering.caption_um);
        let lettered_um = if value_um > caption_um {
            value_um
        } else {
            caption_um
        };
        lettered_um + DRAWING_SHEET_TITLE_CELL_PADDING_UM * 2
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
    let needs = fields
        .iter()
        .map(|id| id.title_cell_need_um())
        .collect::<Vec<_>>();
    let mut cells = Vec::with_capacity(fields.len());
    for (row, run) in split_into_rows(&needs, rows, grid_width_um)
        .into_iter()
        .enumerate()
    {
        let widths = share_row(&fields[run.clone()], &needs[run.clone()], grid_width_um);
        let mut x_um = grid_x_um;
        for (index, cell_width_um) in run.zip(widths) {
            cells.push(DrawingSheetTitleCell {
                id: fields[index],
                row,
                x_um,
                width_um: cell_width_um,
            });
            x_um += cell_width_um;
        }
    }
    Some(DrawingSheetTitleLayout {
        width_um,
        height_um,
        grid_x_um,
        rows,
        cells,
    })
}

/// The widths of one row's cells, which together span `grid_width_um`
/// exactly, by the rule the module documentation sets out.
fn share_row(fields: &[DrawingSheetTitleFieldId], needs: &[u64], grid_width_um: u64) -> Vec<u64> {
    let total_um = needs.iter().sum::<u64>().max(1);
    let fixed_um = fields
        .iter()
        .zip(needs)
        .filter(|(id, _)| id.has_fixed_form())
        .map(|(_, need)| need)
        .sum::<u64>();
    let free_um = total_um.saturating_sub(fixed_um);
    let mut widths = if total_um <= grid_width_um || fixed_um >= grid_width_um || free_um == 0 {
        needs
            .iter()
            .map(|need| grid_width_um * need / total_um)
            .collect::<Vec<_>>()
    } else {
        let free_room_um = grid_width_um - fixed_um;
        fields
            .iter()
            .zip(needs)
            .map(|(id, need)| {
                if id.has_fixed_form() {
                    *need
                } else {
                    free_room_um * need / free_um
                }
            })
            .collect::<Vec<_>>()
    };
    if let Some((last, others)) = widths.split_last_mut() {
        *last = grid_width_um - others.iter().sum::<u64>();
    }
    widths
}

/// Cut `needs` into at most `rows` contiguous runs, by the rule the module
/// documentation sets out.
fn split_into_rows(needs: &[u64], rows: usize, grid_width_um: u64) -> Vec<Range<usize>> {
    let count = needs.len();
    if count == 0 {
        return Vec::new();
    }
    let runs = rows.min(count);
    let mut prefix = Vec::with_capacity(count + 1);
    prefix.push(0_u64);
    for need in needs {
        prefix.push(prefix[prefix.len() - 1] + need);
    }
    let score = |cuts: &[usize]| {
        run_bounds(cuts, count).fold((0_u64, 0_u128), |(overrun, squares), (start, end)| {
            let need = prefix[end] - prefix[start];
            (
                overrun.max(need.saturating_sub(grid_width_um)),
                squares + u128::from(need) * u128::from(need),
            )
        })
    };
    // The cuts between runs, strictly increasing, visited in lexicographic
    // order: a later cut set fills its earlier rows more, so taking the last
    // of a tie fills earlier rows first.
    let mut cuts = (1..runs).collect::<Vec<_>>();
    let mut best = (score(&cuts), cuts.clone());
    while advance_cuts(&mut cuts, count) {
        let candidate = score(&cuts);
        if candidate <= best.0 {
            best = (candidate, cuts.clone());
        }
    }
    run_bounds(&best.1, count)
        .map(|(start, end)| start..end)
        .collect()
}

/// The start and end of every run the `cuts` make in a list of `count`.
fn run_bounds(cuts: &[usize], count: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
    std::iter::once(0)
        .chain(cuts.iter().copied())
        .zip(cuts.iter().copied().chain(std::iter::once(count)))
}

/// Step `cuts` to the next strictly increasing set drawn from `1..count`, or
/// return `false` once every set has been visited.
fn advance_cuts(cuts: &mut [usize], count: usize) -> bool {
    let picked = cuts.len();
    for index in (0..picked).rev() {
        if cuts[index] < count - picked + index {
            cuts[index] += 1;
            for later in index + 1..picked {
                cuts[later] = cuts[later - 1] + 1;
            }
            return true;
        }
    }
    false
}
