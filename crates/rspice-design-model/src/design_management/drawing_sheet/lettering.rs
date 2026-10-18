//! What a drawing sheet's lettering measures, derived from the face that
//! letters it.
//!
//! Every renderer letters a sheet in IBM Plex Mono, so the numbers a layout
//! needs — the em that sets a given capital height, the advance of one
//! character, the drop from a line's centre to its baseline — are all
//! measurements of that one face rather than estimates. They live together
//! here so a layout cannot take one of them from the face and another from a
//! guess.

use super::*;

/// Capital height of IBM Plex Mono, in units per 1000-unit em, as the face's
/// `OS/2.sCapHeight` records it.
const MONO_CAP_HEIGHT_PER_EM: u64 = 698;

/// Em size that letters IBM Plex Mono at the given ISO 3098 height.
///
/// ISO 3098 names a height `h` by the height of the capitals, and so does
/// every CAD system that asks a drafter for a text height. A font engine
/// instead sizes a line by its em, of which the capitals occupy only part.
/// Handing a renderer an em of `h` therefore letters the sheet at 0.698 `h`
/// — an A4 caption lands at 1.26 mm, below the smallest height the standard
/// permits — so every renderer converts here before asking for a size.
#[must_use]
pub const fn drawing_sheet_mono_em_um(lettering_um: u64) -> u64 {
    let em_um = lettering_um * 1_000 / MONO_CAP_HEIGHT_PER_EM;
    if em_um == 0 { 1 } else { em_um }
}

/// Advance width of one IBM Plex Mono character at the given ISO 3098
/// lettering height. The face advances 600 units per 1000-unit em for every
/// glyph, so this is a width measurement, not an approximation of one.
#[must_use]
pub const fn drawing_sheet_mono_advance_um(lettering_um: u64) -> u64 {
    let advance_um = drawing_sheet_mono_em_um(lettering_um) * 3 / 5;
    if advance_um == 0 { 1 } else { advance_um }
}

/// Distance from the vertical centre of a line of IBM Plex Mono down to its
/// alphabetic baseline.
///
/// The canvas centres a laid-out line on a point; publication backends place
/// the baseline on one. The face ascends 1025 and descends 275 units per
/// 1000-unit em, so the two conventions differ by exactly 3/8 of the em the
/// line is set at, and converting between them keeps a field's ink in the
/// same place on screen and on paper.
#[must_use]
pub const fn drawing_sheet_mono_baseline_offset_um(lettering_um: u64) -> u64 {
    drawing_sheet_mono_em_um(lettering_um) * 3 / 8
}

/// The ISO 3098 lettering height series, smallest first. Lettering only ever
/// steps between these, never to a height between two of them.
pub(in crate::design_management) const ISO_3098_HEIGHTS_UM: [u64; 8] =
    [1_800, 2_500, 3_500, 5_000, 7_000, 10_000, 14_000, 20_000];

/// The heights a title block letters its fields at, on every sheet: 1.8 mm
/// captions and 2.5 mm values, the row ISO 3098 gives A4.
///
/// ISO 7200 uses the same title block for every paper size, and the block is
/// a fixed form, the same 32 to 50 mm tall whichever sheet carries it. The
/// heights a sheet recommends rise with the paper because the drawing grows
/// with it; the block does not, so lettering it at an A0 sheet's heights only
/// overfills rows no taller than an A4 block's. A caption stands a step of
/// the series below the value it names.
pub const DRAWING_SHEET_TITLE_BLOCK_LETTERING: DrawingSheetLettering =
    DrawingSheetLettering::for_sheet(210_000, 297_000);

/// How far a line's ink reaches below the point it is seated on, in
/// twentieths of its em.
///
/// Canvas and hardcopy both seat a line on a fraction of the row: the canvas
/// centres the laid-out line there, publication puts the baseline 3/8 of an
/// em lower. Below that baseline the deepest glyph a title field can hold is
/// `g`, which the face draws to 212 units per 1000-unit em — so ink reaches
/// 0.587 of an em below the seat, and 12/20 is the next twentieth clear of
/// it. The line box reaches further, to 13/20, but a cell is clipped to its
/// rules by the ink that crosses them and not by the box around it.
const TITLE_LINE_REACH_PER_EM: u64 = 12;

/// How far a line's capitals rise above the point it is seated on, in
/// twentieths of its em.
///
/// The baseline sits 3/8 of an em below the seat and the face's capitals
/// stand 698 units per 1000-unit em above it, so they top out 0.323 of an em
/// above the seat, and 7/20 is the next twentieth clear of it.
const TITLE_LINE_RISE_PER_EM: u64 = 7;

/// Whether a title row of the given height seats
/// [`DRAWING_SHEET_TITLE_BLOCK_LETTERING`].
///
/// The value's ink must stop above the rule under it, and the caption's
/// descenders must stop short of the value's capitals by the gap ISO 3098's
/// own line spacing leaves between a descender and the next line's capitals:
/// one stroke width, a tenth of the taller line's height. Clearing the rule
/// at the foot of the cell is not enough, because the value lies between.
#[must_use]
pub(in crate::design_management) fn drawing_sheet_title_row_holds_lettering(
    row_height_um: u64,
) -> bool {
    let lettering = DRAWING_SHEET_TITLE_BLOCK_LETTERING;
    let seat_um = |(numerator, denominator): (u64, u64)| row_height_um * numerator / denominator;
    let value_seat_um = seat_um(DRAWING_SHEET_TITLE_VALUE_POSITION);
    let value_em_um = drawing_sheet_mono_em_um(lettering.data_um);
    let caption_foot_um = seat_um(DRAWING_SHEET_TITLE_CAPTION_POSITION)
        + drawing_sheet_mono_em_um(lettering.caption_um) * TITLE_LINE_REACH_PER_EM / 20;
    value_seat_um + value_em_um * TITLE_LINE_REACH_PER_EM / 20 <= row_height_um
        && caption_foot_um + lettering.data_um / 10 + value_em_um * TITLE_LINE_RISE_PER_EM / 20
            <= value_seat_um
}
