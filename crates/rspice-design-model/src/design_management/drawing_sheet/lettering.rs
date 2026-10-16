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
pub const ISO_3098_HEIGHTS_UM: [u64; 8] =
    [1_800, 2_500, 3_500, 5_000, 7_000, 10_000, 14_000, 20_000];

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

/// The heights a title block can actually letter its rows at.
///
/// The lettering a sheet recommends rises with the paper, but a title block
/// does not: it is a fixed form, the same 32 to 50 mm tall whichever sheet
/// carries it, so a row of an A0 block is no taller than a row of an A4 one.
/// Letter that row at the height A0 recommends and the value's descenders
/// cross the rule below it — on the canvas, which clips a cell to its rules,
/// that is the field truncated outright.
///
/// Each height therefore steps down the standard's own series until the line
/// it sets fits between its seat in the row and the rule under it, so a
/// clamped field is still lettered at an ISO height rather than at some
/// height derived from the furniture.
#[must_use]
pub fn drawing_sheet_title_block_lettering(
    format: &SchematicSheetFormat,
    geometry: &DrawingSheetGeometry,
) -> Option<DrawingSheetLettering> {
    let block = geometry.title_block?;
    let rows = format.title_block_rows(geometry.effective_title_block_template)? as u64;
    let authored_height_um = match format.title_block.rotation {
        DrawingSheetTitleBlockRotation::Upright => block.height_um,
        DrawingSheetTitleBlockRotation::Clockwise90
        | DrawingSheetTitleBlockRotation::CounterClockwise90 => block.width_um,
    };
    let row_height_um = authored_height_um / rows.max(1);
    let mut lettering = format.authored_size.lettering();
    lettering.caption_um = fit_title_line_um(
        row_height_um,
        DRAWING_SHEET_TITLE_CAPTION_POSITION,
        lettering.caption_um,
    );
    lettering.data_um = fit_title_line_um(
        row_height_um,
        DRAWING_SHEET_TITLE_VALUE_POSITION,
        lettering.data_um,
    );
    Some(lettering)
}

/// Largest height at or below `requested_um` whose ink clears the rule below
/// its seat in the row.
fn fit_title_line_um(row_height_um: u64, position: (u64, u64), requested_um: u64) -> u64 {
    let (numerator, denominator) = position;
    let below_seat_um = row_height_um.saturating_sub(row_height_um * numerator / denominator);
    let mut fitted_um = ISO_3098_HEIGHTS_UM[0];
    for height_um in ISO_3098_HEIGHTS_UM {
        if height_um > requested_um {
            break;
        }
        if drawing_sheet_mono_em_um(height_um) * TITLE_LINE_REACH_PER_EM / 20 <= below_seat_um {
            fitted_um = height_um;
        }
    }
    fitted_um
}
