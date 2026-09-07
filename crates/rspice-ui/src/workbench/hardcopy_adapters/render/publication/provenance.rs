//! Measured provenance rows shared by vector, raster, preview, and printer output.

use unicode_segmentation::UnicodeSegmentation as _;

use super::*;

pub(super) const PROVENANCE_TEXT_UM: u64 = 2_200;
const INSET_UM: u64 = 300;
const ROW_GAP_UM: u64 = 400;

pub(super) struct ProvenanceRow {
    pub(super) text: String,
    pub(super) x_um: u64,
    pub(super) baseline_um: u64,
}

/// Preserve every character of the identity, including long unbroken keys.
/// The fixed band has a finite capacity: refuse excess text before rendering.
pub(super) fn provenance_rows(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    page: &PreviewPage,
) -> Result<Vec<ProvenanceRow>, HardcopyRenderError> {
    if !plan.setup().decorations().includes_provenance() {
        return Ok(Vec::new());
    }
    let face = FontCoverage::load()?.mono;
    let units = u64::from(face.units_per_em());
    let to_um = |value: u64| (value * PROVENANCE_TEXT_UM).div_ceil(units);
    let bounds = face.global_bounding_box();
    let ascent = to_um(i64::from(face.ascender().max(bounds.y_max)).max(0) as u64);
    let descent = to_um((-i64::from(face.descender().min(bounds.y_min))).max(0) as u64);
    let left_inset = INSET_UM + to_um((-i64::from(bounds.x_min)).max(0) as u64);
    let row_height = ascent + descent;
    let pitch = row_height + ROW_GAP_UM;
    let geometry = page.geometry();
    let printable = geometry.printable_rect();
    let available_height = geometry
        .provenance_band()
        .micrometres()
        .saturating_sub(2 * INSET_UM);
    let maximum = ((available_height + ROW_GAP_UM) / pitch) as usize;
    let width = printable
        .width
        .micrometres()
        .saturating_sub(left_inset + INSET_UM);
    let width_units = width * units / PROVENANCE_TEXT_UM;
    let fallback;
    let lines = if scene.metadata.provenance_lines.is_empty() {
        fallback = vec![format!(
            "source {} · plan {}",
            plan.source().content_digest(),
            plan.content_digest()
        )];
        &fallback
    } else {
        &scene.metadata.provenance_lines
    };
    let overflow = |actual| HardcopyRenderError::DecorationOverflow {
        decoration: "provenance rows",
        actual,
        maximum,
    };
    if lines.len() > maximum {
        return Err(overflow(lines.len()));
    }
    let mut wrapped = Vec::new();
    for line in lines {
        let mut remaining = line.as_str();
        // An authored empty row still consumes space.
        loop {
            if wrapped.len() == maximum {
                return Err(overflow(wrapped.len() + 1));
            }
            let mut end = 0;
            let mut word_break = None;
            let mut advance = 0;
            for (offset, grapheme) in remaining.grapheme_indices(true) {
                let next = advance + grapheme_width(&face, grapheme)?;
                if next > width_units {
                    break;
                }
                advance = next;
                end = offset + grapheme.len();
                if grapheme.chars().all(char::is_whitespace) {
                    word_break = Some(end);
                }
            }
            if end == 0 && !remaining.is_empty() {
                return Err(overflow(maximum + 1));
            }
            if end < remaining.len() {
                end = word_break.unwrap_or(end);
            }
            wrapped.push(remaining[..end].to_owned());
            remaining = &remaining[end..];
            if remaining.is_empty() {
                break;
            }
        }
    }
    let last_baseline =
        printable.y.micrometres() + printable.height.micrometres() - INSET_UM - descent;
    let first_baseline = last_baseline - (wrapped.len().saturating_sub(1) as u64) * pitch;
    Ok(wrapped
        .into_iter()
        .enumerate()
        .map(|(index, text)| ProvenanceRow {
            text,
            x_um: printable.x.micrometres() + left_inset,
            baseline_um: first_baseline + index as u64 * pitch,
        })
        .collect())
}

/// Include glyph overhangs as well as advances. Graphemes stay intact when a
/// key must wrap; conservative widths also cover decomposed accented names.
fn grapheme_width(face: &ttf_parser::Face<'_>, text: &str) -> Result<u64, HardcopyRenderError> {
    let mut advance = 0i64;
    let mut left = 0;
    let mut right = 0;
    for character in text.chars() {
        let glyph = face
            .glyph_index(character)
            .ok_or(HardcopyRenderError::UnsupportedGlyph {
                codepoint: character as u32,
                context: "page provenance",
            })?;
        if let Some(bounds) = face.glyph_bounding_box(glyph) {
            left = left.min(advance + i64::from(bounds.x_min));
            right = right.max(advance + i64::from(bounds.x_max));
        }
        advance += i64::from(face.glyph_hor_advance(glyph).ok_or(
            HardcopyRenderError::InvalidEmbeddedFont("IBM Plex Mono Regular"),
        )?);
        right = right.max(advance);
    }
    Ok((right - left) as u64)
}
