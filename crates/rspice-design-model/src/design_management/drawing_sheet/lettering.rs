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

/// Height ISO 5457 letters a grid reference at, on every sheet size.
const ZONE_REFERENCE_UM: u64 = 3_500;

/// The ISO 3098 height a zone reference in a border band of the given width
/// is lettered at.
///
/// ISO 5457 letters grid references 3.5 mm high whatever the paper: they
/// label a frame of fixed width rather than the drawing inside it, so unlike
/// the drawing's own lettering they do not grow with the sheet. A band too
/// narrow for that steps down the series to the largest height whose
/// capitals clear both of its rules by a stroke width, a tenth of the
/// height, and no lower than the series goes. Canvas and hardcopy both
/// letter a reference at this height, centred in the band.
#[must_use]
pub fn drawing_sheet_zone_lettering_um(border_band_um: u64) -> u64 {
    let mut fitted_um = ISO_3098_HEIGHTS_UM[0];
    for height_um in ISO_3098_HEIGHTS_UM {
        if height_um > ZONE_REFERENCE_UM {
            break;
        }
        if height_um * 12 / 10 <= border_band_um {
            fitted_um = height_um;
        }
    }
    fitted_um
}

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

/// The heights a title block can actually letter its rows at.
///
/// The lettering a sheet recommends rises with the paper, but a title block
/// does not: it is a fixed form, the same 32 to 50 mm tall whichever sheet
/// carries it, so a row of an A0 block is no taller than a row of an A4 one.
/// Letter that row at the height A0 recommends and the value's descenders
/// cross the rule below it — on the canvas, which clips a cell to its rules,
/// that is the field truncated outright.
///
/// The value therefore steps down the standard's own series until its line
/// fits between its seat in the row and the rule under it, and the caption
/// steps down until it sits below that value: a step of the series lower,
/// with its descenders clear of the value's capitals. A clamped field is
/// still lettered at ISO heights rather than at heights derived from the
/// furniture, and its caption never draws level with the value it names.
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
    lettering.data_um = fit_title_line_um(
        row_height_um,
        DRAWING_SHEET_TITLE_VALUE_POSITION,
        lettering.data_um,
    );
    lettering.caption_um =
        fit_title_caption_um(row_height_um, lettering.caption_um, lettering.data_um);
    Some(lettering)
}

/// Largest height at or below `requested_um` that letters a caption beneath
/// the value it names.
///
/// The caption takes a lower step of the series than the value — unless the
/// value already stands on the lowest, which has none below it — and its
/// descenders must stop short of the value's capitals by the gap ISO 3098's
/// own line spacing leaves between a descender and the next line's capitals:
/// one stroke width, a tenth of the taller line's height. Clearing the rule at
/// the foot of the cell is not enough, because the value lies between.
fn fit_title_caption_um(row_height_um: u64, requested_um: u64, value_um: u64) -> u64 {
    let seat_um = |(numerator, denominator): (u64, u64)| row_height_um * numerator / denominator;
    let value_capitals_um = seat_um(DRAWING_SHEET_TITLE_VALUE_POSITION)
        .saturating_sub(drawing_sheet_mono_em_um(value_um) * TITLE_LINE_RISE_PER_EM / 20);
    let caption_seat_um = seat_um(DRAWING_SHEET_TITLE_CAPTION_POSITION);
    let mut fitted_um = ISO_3098_HEIGHTS_UM[0];
    for height_um in ISO_3098_HEIGHTS_UM {
        if height_um > requested_um || height_um >= value_um {
            break;
        }
        let ink_foot_um =
            caption_seat_um + drawing_sheet_mono_em_um(height_um) * TITLE_LINE_REACH_PER_EM / 20;
        if ink_foot_um + value_um / 10 <= value_capitals_um {
            fitted_um = height_um;
        }
    }
    fitted_um
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
