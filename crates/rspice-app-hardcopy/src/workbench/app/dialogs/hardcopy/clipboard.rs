//! Edit ▸ Copy as image: the selection, or the whole view, drawn through the
//! hardcopy raster path and handed to the platform clipboard as a picture.
//!
//! The picture is the export's drawing, not a screenshot: the same resolved
//! source, ink mapping and renderer the Export dialog previews, at 1:1 on a
//! page cut to the content, without the header, ink legend or provenance
//! band a printed page carries. A schematic with a selection copies the
//! selection; otherwise the view's active extent is copied (the sheet in
//! view, the symbol, the plot).

use super::{HardcopyDialogState, HardcopyParts, HardcopyWorkflow, publish};
use crate::workbench::SurfaceId;
use crate::workbench::hardcopy_adapters::sources::{
    enumerate_retained_hardcopy_sources, prepare_retained_hardcopy_resolution,
};
use rspice_hardcopy::render::{
    HardcopyPublicationTimestamp, HardcopyRenderer, HardcopySceneMetadata,
};
use rspice_hardcopy_contract::{
    BackgroundMode, Bleed, ContentExtent, CustomPaper, DecorationSetup, FontPolicy, HardcopyError,
    HardcopyPlan, HardcopyScope, HardcopySetup, Length, LengthUnit, MIN_RASTER_DPI, Orientation,
    OutputFormat, OutsideSheetContentPolicy, PageMargins, PaperSize, PhysicalPageSetup,
    RenderSetup, RenderTarget, ScaleMode, SchematicHardcopySetup, TilingMode, TilingSetup,
    Watermark,
};

/// The resolution a picture is drawn at: twice a 96 dpi screen's, so it
/// stays sharp pasted into a document or shown on a high-density display.
const PICTURE_DPI: u16 = 192;
/// The longest edge a picture may have; a larger drawing is drawn at a lower
/// resolution instead.
const LONGEST_EDGE_PX: u64 = 4_096;
/// The white border kept around the drawing: an eighth of an inch.
const BORDER_UM: u64 = 3_175;
/// The smallest page side the hardcopy contract lays out.
const MIN_PAGE_UM: u64 = 10_000;
const MICROMETRES_PER_INCH: u64 = 25_400;

/// A picture Copy as image drew, and what it is of.
pub struct CopiedImage {
    pub image: egui::ColorImage,
    /// Whether it is of the selection rather than the whole view.
    pub selection: bool,
}

impl CopiedImage {
    /// What the status bar says once the picture is on the clipboard.
    #[must_use]
    pub fn summary(&self) -> String {
        let [width, height] = self.image.size;
        let what = if self.selection { "selection" } else { "view" };
        format!("Copied the {what} as a {width} \u{00d7} {height} picture.")
    }
}

/// Draw the schematic's selection, or the active view, as a picture.
pub fn copy_as_image(parts: &HardcopyParts<'_>) -> Result<CopiedImage, String> {
    let (source_key, active_extent) = publish::active_retained_source_selection(parts)?;
    let selection = parts.workbench.current_route().surface_id() == SurfaceId::Design
        && !parts.model.schematic.session.editor.selection.is_empty()
        && enumerate_retained_hardcopy_sources(parts.source_reads())
            .iter()
            .any(|candidate| {
                candidate.source_key == source_key
                    && candidate.supports_scope(&HardcopyScope::Selection)
            });
    let scope = if selection {
        HardcopyScope::Selection
    } else {
        active_extent
    };
    let document = prepare_retained_hardcopy_resolution(parts.source_reads(), &source_key, scope)
        .map_err(|error| error.to_string())?
        .resolve_owned()
        .map_err(|error| error.to_string())?;
    let saved = parts
        .model
        .workspace
        .content
        .hardcopy_setups
        .setup_for(document.authority())
        .map_err(|error| error.to_string())?
        .map(|saved| saved.setup().clone());
    // The setup the Export dialog would open with says what the drawing
    // shows and in which inks; the picture takes those and its own page.
    let mut draft = HardcopyDialogState::default();
    draft
        .open_resolved(HardcopyWorkflow::Export, document, saved.as_ref())
        .map_err(|error| error.to_string())?;
    // A picture of the view holds everything drawn on it, on the sheet or
    // off it: there is no paper to keep it to.
    if draft.outside_sheet_content == OutsideSheetContentPolicy::Ask {
        draft.outside_sheet_content = OutsideSheetContentPolicy::ExtendOutput;
    }
    let drawing = draft.build_setup().map_err(|error| error.to_string())?;
    let document = draft
        .resolved_document
        .take()
        .ok_or_else(|| "the view did not resolve".to_owned())?;
    let content = document
        .content_extent_for_setup(drawing.schematic())
        .map_err(|error| error.to_string())?;
    if !document
        .hardcopy_sections_for_setup(drawing.schematic())
        .map_err(|error| error.to_string())?
        .is_empty()
    {
        return Err("the view prints as several pages; export it instead".to_owned());
    }
    let (width, height) = picture_page(content);
    let dpi = picture_dpi(width, height);
    let setup = picture_setup(&drawing, width, height, dpi).map_err(|error| error.to_string())?;
    let plan = HardcopyPlan::compile(document.authority().clone(), setup, content)
        .map_err(|error| error.to_string())?;
    if plan.pagination().pages().len() != 1 {
        return Err("the view prints as several pages; export it instead".to_owned());
    }
    let metadata =
        metadata(document.authority().display_name()).map_err(|error| error.to_string())?;
    let page = HardcopyRenderer::render_preview_pages_resolved(
        &plan,
        &document,
        metadata,
        &[0],
        dpi,
        || false,
    )
    .map_err(|error| error.to_string())?
    .pop()
    .ok_or_else(|| "nothing was drawn".to_owned())?;
    let image = cropped(page.width(), page.height(), page.rgba(), u32::from(dpi) / 8)
        .ok_or_else(|| "there is nothing to draw".to_owned())?;
    Ok(CopiedImage { image, selection })
}

/// The page a picture of `content` is drawn on: the content at 1:1 with a
/// border all round, never smaller than the contract's smallest page.
fn picture_page(content: ContentExtent) -> (Length, Length) {
    let side = |length: Length| {
        Length::from_micrometres((length.micrometres() + 2 * BORDER_UM).max(MIN_PAGE_UM))
    };
    (side(content.width()), side(content.height()))
}

/// The resolution a page of this size is drawn at: [`PICTURE_DPI`], or less
/// where that would make the picture's longer edge exceed
/// [`LONGEST_EDGE_PX`].
fn picture_dpi(width: Length, height: Length) -> u16 {
    let longest = width.micrometres().max(height.micrometres()).max(1);
    let fitting = LONGEST_EDGE_PX * MICROMETRES_PER_INCH / longest;
    u16::try_from(fitting)
        .unwrap_or(u16::MAX)
        .clamp(MIN_RASTER_DPI, PICTURE_DPI)
}

/// The drawing's own setup on the picture's page: what it shows and its inks
/// stay; the page is cut to the content at 1:1 on white, with no decoration
/// band, crop mark or tile overlap.
fn picture_setup(
    drawing: &HardcopySetup,
    width: Length,
    height: Length,
    dpi: u16,
) -> Result<HardcopySetup, HardcopyError> {
    let orientation = if width > height {
        Orientation::Landscape
    } else {
        Orientation::Portrait
    };
    let render = drawing.render();
    let schematic = drawing.schematic();
    HardcopySetup::try_new_with_schematic(
        PhysicalPageSetup::try_new(
            PaperSize::Custom(CustomPaper::try_new(
                "Picture",
                width,
                height,
                LengthUnit::Millimetres,
            )?),
            PageMargins::uniform(Length::from_micrometres(BORDER_UM)),
            Bleed::None,
            orientation,
        )?,
        ScaleMode::EngineeringOneToOne,
        TilingSetup::try_new(TilingMode::Automatic, Length::ZERO, false)?,
        RenderSetup::try_new(
            RenderTarget::ExportArtifact,
            OutputFormat::Png { dpi },
            render.color_mapping(),
            BackgroundMode::White,
            // A picture holds no text to search or fonts to embed.
            FontPolicy::new(false, false),
            false,
        )?,
        DecorationSetup::try_new(false, false, false, Watermark::None)?,
        SchematicHardcopySetup::new(
            schematic.extent(),
            schematic.outside_content(),
            false,
            schematic.includes_paper(),
            schematic.includes_border(),
            schematic.includes_title_block(),
            schematic.includes_zones(),
            schematic.includes_grid(),
        ),
        drawing.print_mapping().clone(),
    )
}

/// The identity a page's scene needs; no band on the picture prints it.
fn metadata(
    title: &str,
) -> Result<HardcopySceneMetadata, rspice_hardcopy::render::HardcopyRenderError> {
    let mut metadata = HardcopySceneMetadata::try_new(title, "RSpice")?;
    metadata.set_publication_timestamp(HardcopyPublicationTimestamp::from_unix_seconds(
        crate::time_compat::unix_epoch().as_secs(),
    )?);
    Ok(metadata)
}

/// The `width` × `height` RGBA page cut to what is drawn on it, with `border`
/// pixels of the page kept round it; `None` for a blank page. The page's
/// corner is its background: the page has a border there, never ink.
fn cropped(width: u32, height: u32, rgba: &[u8], border: u32) -> Option<egui::ColorImage> {
    let (width, height) = (width as usize, height as usize);
    let pixel = |x: usize, y: usize| &rgba[(y * width + x) * 4..(y * width + x) * 4 + 4];
    let background = pixel(0, 0);
    let mut inked: Option<(usize, usize, usize, usize)> = None;
    for y in 0..height {
        for x in 0..width {
            if pixel(x, y) != background {
                let (left, top, right, bottom) = inked.unwrap_or((x, y, x, y));
                inked = Some((left.min(x), top.min(y), right.max(x), bottom.max(y)));
            }
        }
    }
    let (left, top, right, bottom) = inked?;
    let border = border as usize;
    let (left, top) = (left.saturating_sub(border), top.saturating_sub(border));
    let (right, bottom) = (
        (right + border).min(width - 1),
        (bottom + border).min(height - 1),
    );
    let mut pixels = Vec::with_capacity((right - left + 1) * (bottom - top + 1) * 4);
    for y in top..=bottom {
        pixels.extend_from_slice(&rgba[(y * width + left) * 4..(y * width + right + 1) * 4]);
    }
    Some(egui::ColorImage::from_rgba_unmultiplied(
        [right - left + 1, bottom - top + 1],
        &pixels,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A page is cut to its ink with the asked border kept, and a blank page
    /// is nothing to copy.
    #[test]
    fn a_page_is_cut_to_its_ink_with_a_border() {
        let (width, height) = (20_u32, 10_u32);
        let mut rgba = vec![255_u8; (width * height * 4) as usize];
        assert!(cropped(width, height, &rgba, 2).is_none());
        for (x, y) in [(5, 3), (12, 6)] {
            let at = ((y * width + x) * 4) as usize;
            rgba[at..at + 3].copy_from_slice(&[0, 0, 0]);
        }
        let image = cropped(width, height, &rgba, 2).expect("ink");
        assert_eq!(image.size, [12, 8]);
        assert_eq!(image.pixels[2 * 12 + 2], egui::Color32::BLACK);
        assert_eq!(image.pixels[0], egui::Color32::WHITE);
        let image = cropped(width, height, &rgba, 100).expect("ink");
        assert_eq!(image.size, [20, 10]);
    }

    /// A page is drawn at the picture resolution until its longer edge would
    /// pass the cap, then at the resolution that meets it, never under the
    /// contract's least.
    #[test]
    fn a_large_page_is_drawn_at_a_lower_resolution() {
        let inches = |value: u64| Length::from_micrometres(value * MICROMETRES_PER_INCH);
        assert_eq!(picture_dpi(inches(10), inches(5)), PICTURE_DPI);
        assert_eq!(picture_dpi(inches(32), inches(5)), 128);
        assert_eq!(picture_dpi(inches(5), inches(200)), MIN_RASTER_DPI);
    }
}
