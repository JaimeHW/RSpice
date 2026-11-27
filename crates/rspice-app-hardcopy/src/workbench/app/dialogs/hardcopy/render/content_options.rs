//! What the Print and Export settings may offer: whether a printer's own
//! properties are reachable, and the raster resolution a file is written at.
//!
//! A raster resolution is bounded by what this host can afford for the chosen
//! page, so the field's text is only committed once it is whole: on Enter, on
//! leaving the field, or on the pass that publishes. None of this refreshes
//! the preview itself; the body compares the whole draft before and after the
//! pass.

use super::*;

/// Whether this host has system printers whose properties the dialog can set:
/// Windows through its drivers, and macOS and Linux through CUPS. The browser
/// selects tray, duplex and collation in its own print dialog, so there is
/// nothing behind this row. Where it leads nowhere it is absent rather than
/// dimmed: a row advertising capabilities the host does not have reads as a
/// feature withheld instead of one that was never there.
pub(super) const fn printer_properties_reachable() -> bool {
    cfg!(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux"
    ))
}

/// The resolution field's fixed identity, so whether it is being edited can be
/// read on the passes where the section that draws it is no longer on screen.
pub(super) fn raster_resolution_id() -> egui::Id {
    egui::Id::new("hardcopy-raster-resolution")
}

/// What this page and this host will accept for the format in the draft, or the
/// contract's own range while no plan has compiled.
pub(super) fn raster_dpi_ceiling(draft: &HardcopyDialogState) -> Option<u16> {
    raster_dpi_ceiling_for(draft, draft.format)
}

/// The same question asked of a format the draft does not currently hold, which
/// is what the target list has to ask before it offers one.
pub(super) fn raster_dpi_ceiling_for(
    draft: &HardcopyDialogState,
    format: OutputFormat,
) -> Option<u16> {
    draft
        .preview_plan
        .as_ref()
        .and_then(|plan| max_raster_dpi(plan, format))
}

/// The resolution in force for a raster target: the one the format carries,
/// never above what this host can raster for the page in front of it.
pub(super) fn resolution_in_force(format: OutputFormat, ceiling: Option<u16>) -> u16 {
    let dpi = format.raster_dpi().unwrap_or(DEFAULT_RASTER_DPI);
    ceiling.map_or(dpi, |ceiling| dpi.min(ceiling))
}

/// `format` carrying a resolution this host can actually raster for it here.
pub fn bounded_raster(draft: &HardcopyDialogState, format: OutputFormat) -> OutputFormat {
    format.with_raster_dpi(resolution_in_force(
        format,
        raster_dpi_ceiling_for(draft, format),
    ))
}

/// Take what the field holds into the draft. An entry that names no resolution
/// this page admits reverts to the one in force, because the field states the
/// committed value whenever it is not being typed into — and the value it
/// reverts to is bounded, since reverting to a resolution the publisher would
/// itself refuse is no refusal at all.
pub(super) fn commit_raster_resolution(draft: &mut HardcopyDialogState, ceiling: Option<u16>) {
    let dpi = parse_raster_dpi(&draft.raster_dpi_draft, ceiling).unwrap_or_else(|_| {
        let dpi = resolution_in_force(draft.format, ceiling);
        draft.raster_dpi_draft = dpi.to_string();
        dpi
    });
    draft.format = draft.format.with_raster_dpi(dpi);
}

/// Settle a resolution the operator typed and then walked away from.
///
/// The field's text outlives the widget that edits it. Selecting another
/// section, or the page instead of the setup, replaces what is drawn in the
/// same pass the choice is made, so the field is never drawn again to notice
/// its editing session ended — and a publication started from wherever the
/// operator landed would carry the previously committed resolution while the
/// field still showed the typed one. The body settles it once, on the path
/// every pass takes, rather than from the surfaces a layout may decide not to
/// draw.
///
/// The ceiling settles on the same path and for the same kind of reason. It
/// belongs to the page rather than to the field, so media, orientation, scale
/// and tiling all move it under a resolution that was legal when it was
/// committed. Bringing the committed value back inside it here is what keeps
/// the rail summary, the estimate strip and the enabled primary describing a
/// publication this host will accept, rather than one it refuses a render round
/// trip later.
pub(super) fn settle_raster_resolution(ui: &Ui, draft: &mut HardcopyDialogState) {
    if draft.format.raster_dpi().is_none() {
        return;
    }
    let ceiling = raster_dpi_ceiling(draft);
    draft.format = bounded_raster(draft, draft.format);
    if ui
        .ctx()
        .memory(|memory| memory.has_focus(raster_resolution_id()))
    {
        return;
    }
    commit_raster_resolution(draft, ceiling);
}

/// Accept a resolution only if both the contract and this host admit it.
pub(super) fn parse_raster_dpi(text: &str, ceiling: Option<u16>) -> Result<u16, String> {
    let dpi: u16 = text
        .trim()
        .parse()
        .map_err(|_| "enter a whole number of dots per inch".to_owned())?;
    if dpi < MIN_RASTER_DPI {
        return Err(format!(
            "{MIN_RASTER_DPI} dpi is the lowest this contract accepts"
        ));
    }
    if dpi > ceiling.unwrap_or(MAX_RASTER_DPI) {
        return Err(match ceiling {
            Some(ceiling) => format!("this page cannot be rastered above {ceiling} dpi here"),
            None => format!("{MAX_RASTER_DPI} dpi is the highest this contract accepts"),
        });
    }
    Ok(dpi)
}

pub(super) type NoteTone = fn(&Ui) -> egui::Color32;

/// What the field says under itself. Only the text being typed can be out of
/// range: the committed value is settled inside the ceiling on every pass, so
/// the note states the bound rather than reporting the draft against it.
pub(super) fn raster_resolution_note(text: &str, ceiling: Option<u16>) -> (String, NoteTone) {
    let warn: NoteTone = |ui| ui.visuals().warn_fg_color;
    let dim: NoteTone = |ui| Tokens::get(ui.ctx()).color.text_dim;
    match parse_raster_dpi(text, ceiling) {
        Err(reason) => (reason, warn),
        Ok(_) => match ceiling {
            Some(ceiling) => (format!("up to {ceiling} dpi for this page"), dim),
            None => (format!("{MIN_RASTER_DPI}–{MAX_RASTER_DPI} dpi"), dim),
        },
    }
}
