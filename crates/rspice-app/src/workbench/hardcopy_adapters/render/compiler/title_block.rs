//! Canonical title-block projection into hardcopy scene primitives.
//!
//! Rotation, clipping, managed logos, and field layout are resolved here so
//! every publication backend consumes identical geometry.

use super::*;

pub(in crate::workbench::hardcopy_adapters::render) fn authored_title_block_rect(
    block: DrawingSheetRect,
    rotation: DrawingSheetTitleBlockRotation,
) -> Result<DrawingSheetRect, HardcopyRenderError> {
    if rotation == DrawingSheetTitleBlockRotation::Upright {
        return Ok(block);
    }
    let block_width = i64::try_from(block.width_um)
        .map_err(|_| conversion_error("drawing-sheet title-block width overflow"))?;
    let block_height = i64::try_from(block.height_um)
        .map_err(|_| conversion_error("drawing-sheet title-block height overflow"))?;
    let x_twice = block
        .x_um
        .checked_mul(2)
        .and_then(|value| value.checked_add(block_width))
        .and_then(|value| value.checked_sub(block_height))
        .ok_or_else(|| conversion_error("drawing-sheet title-block X geometry overflow"))?;
    let y_twice = block
        .y_um
        .checked_mul(2)
        .and_then(|value| value.checked_add(block_height))
        .and_then(|value| value.checked_sub(block_width))
        .ok_or_else(|| conversion_error("drawing-sheet title-block Y geometry overflow"))?;
    if x_twice % 2 != 0 || y_twice % 2 != 0 {
        return Err(conversion_error(
            "drawing-sheet title-block geometry requires half-micrometre coordinates",
        ));
    }
    Ok(DrawingSheetRect {
        x_um: x_twice / 2,
        y_um: y_twice / 2,
        width_um: block.height_um,
        height_um: block.width_um,
    })
}

pub(in crate::workbench::hardcopy_adapters::render) fn zone_alpha_label(index: u8) -> String {
    // Match canvas and preview: engineering drawing zones omit ambiguous
    // letters and fall back to the numeric ordinal once the alphabet ends.
    const LETTERS: &[u8] = b"ABCDEFGHJKLMNPRSTUVWXY";
    LETTERS.get(usize::from(index)).map_or_else(
        || (usize::from(index) + 1).to_string(),
        |letter| char::from(*letter).to_string(),
    )
}

pub(in crate::workbench::hardcopy_adapters::render) fn midpoint_coordinate(
    start: i64,
    end: i64,
    context: &'static str,
) -> Result<i64, HardcopyRenderError> {
    end.checked_sub(start)
        .and_then(|distance| start.checked_add(distance / 2))
        .ok_or_else(|| conversion_error(format!("{context} coordinate overflow")))
}

pub(in crate::workbench::hardcopy_adapters::render) fn truncate_title_block_text(
    value: &str,
    max_chars: usize,
) -> String {
    let count = value.chars().count();
    if count <= max_chars {
        return value.to_owned();
    }
    let keep = max_chars.saturating_sub(1);
    let mut truncated = value.chars().take(keep).collect::<String>();
    truncated.push('…');
    truncated
}
