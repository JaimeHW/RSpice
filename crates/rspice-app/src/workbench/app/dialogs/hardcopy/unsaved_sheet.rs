//! A drawing sheet with nothing saved for it follows its target until the
//! user changes the page.
//!
//! Exported, it is laid out as drawn: its own paper, at 1:1. Sent to a
//! printer, it is fitted to the printable area of the printer's paper (its
//! own when the printer offers it), inside the widest margin the driver
//! cannot print, so a print with nothing saved is never refused for ink the
//! printer cannot reach. Only a 1:1 the user chose can be. The dialog lays
//! the page out again only while it is exactly as the dialog last laid it;
//! anything the user changed stays.

use super::*;
use crate::workbench::hardcopy_adapters::print::{
    PrinterCapabilitySnapshot, PrinterPaperCapability,
};
use rspice_hardcopy_contract::PrinterRasterGeometry;

/// A sheet's paper and a printer's are the same paper within this, as the
/// printer selection matches them.
const PAPER_MATCH_UM: u64 = 100;

/// How a drawing sheet with nothing saved was drawn, and how the dialog last
/// laid it out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UnsavedSheetLayout {
    as_drawn: HardcopySetup,
    laid_out: HardcopySetup,
}

impl UnsavedSheetLayout {
    pub(super) const fn new(as_drawn: HardcopySetup, laid_out: HardcopySetup) -> Self {
        Self { as_drawn, laid_out }
    }
}

impl HardcopyDialogState {
    /// Fit a sheet with nothing saved to the printable area of the paper it
    /// prints on with this printer, in the sheet's own orientation, centred,
    /// with the header, legend and provenance bands still off. Its margin is
    /// set once the driver's printable rectangle for that paper is known; see
    /// [`Self::keep_unsaved_sheet_inside`]. Returns whether the page was laid
    /// out again.
    pub(crate) fn fit_unsaved_sheet_to_printer(
        &mut self,
        capabilities: &PrinterCapabilitySnapshot,
        default_paper: Option<i16>,
    ) -> bool {
        let Some(sheet) = self.followed_unsaved_sheet() else {
            return false;
        };
        let Some(paper) =
            unsaved_sheet_paper(&sheet.as_drawn, capabilities.papers(), default_paper)
        else {
            return false;
        };
        let Ok(fitted) = fitted_to(&sheet.as_drawn, paper, Length::ZERO) else {
            return false;
        };
        self.lay_out_unsaved_sheet(&fitted)
    }

    /// Keep a fitted sheet with nothing saved inside the widest margin the
    /// driver cannot print, the same on every side so the sheet stays centred
    /// on its paper.
    pub(crate) fn keep_unsaved_sheet_inside(&mut self, geometry: PrinterRasterGeometry, dpi: u16) {
        let Some(sheet) = self.followed_unsaved_sheet() else {
            return;
        };
        let physical = sheet.laid_out.physical_page();
        let Ok(page) = PhysicalPageSetup::try_new(
            physical.paper().clone(),
            PageMargins::uniform(widest_unprintable_margin(geometry, dpi)),
            physical.bleed(),
            physical.orientation(),
        ) else {
            return;
        };
        let laid_out = &sheet.laid_out;
        let Ok(inside) = HardcopySetup::try_new_with_schematic(
            page,
            laid_out.scale(),
            laid_out.tiling(),
            laid_out.render().clone(),
            laid_out.decorations().clone(),
            laid_out.schematic(),
            laid_out.print_mapping().clone(),
        ) else {
            return;
        };
        self.lay_out_unsaved_sheet(&inside);
    }

    /// The unsaved sheet, while the page is still as the dialog last laid it.
    fn followed_unsaved_sheet(&self) -> Option<UnsavedSheetLayout> {
        let sheet = self.unsaved_sheet.as_ref()?;
        let current = self.page_layout().ok()?;
        same_page(&current, &sheet.laid_out).then(|| UnsavedSheetLayout::clone(sheet))
    }

    /// Lay the page out as `layout` does, leaving where it is sent, its
    /// colour, fonts and print mapping as they are, and remember the page as
    /// the draft now builds it.
    fn lay_out_unsaved_sheet(&mut self, layout: &HardcopySetup) -> bool {
        let unit = self.display_unit;
        let physical = layout.physical_page();
        let margins = physical.margins();
        self.paper = PaperDraft::from_paper(physical.paper(), unit);
        self.orientation = physical.orientation();
        self.margin_top = format_length(margins.top, unit);
        self.margin_right = format_length(margins.right, unit);
        self.margin_bottom = format_length(margins.bottom, unit);
        self.margin_left = format_length(margins.left, unit);
        self.bleed = match physical.bleed() {
            Bleed::None => "0".to_owned(),
            Bleed::Uniform(value) => format_length(value, unit),
        };
        self.scale = layout.scale();
        let decorations = layout.decorations();
        self.include_legends = decorations.includes_legends();
        self.include_header = decorations.includes_header();
        self.include_provenance = decorations.includes_provenance();
        self.watermark = decorations.watermark().clone();
        let laid_out = self.page_layout().ok();
        match (self.unsaved_sheet.as_mut(), laid_out) {
            (Some(sheet), Some(laid_out)) => {
                sheet.laid_out = laid_out;
                true
            }
            _ => {
                self.unsaved_sheet = None;
                false
            }
        }
    }
}

/// Whether two setups lay out the same page, whatever each is sent to.
fn same_page(left: &HardcopySetup, right: &HardcopySetup) -> bool {
    left.physical_page() == right.physical_page()
        && left.scale() == right.scale()
        && left.tiling() == right.tiling()
        && left.decorations() == right.decorations()
        && left.schematic() == right.schematic()
}

/// The paper a sheet with nothing saved prints on: its own when the printer
/// offers it, otherwise the driver's default paper. When the driver names
/// none, the offered paper of the sheet's own family (A4 for an ISO sheet,
/// US Letter for any other), and failing that the first paper offered.
fn unsaved_sheet_paper<'a>(
    as_drawn: &HardcopySetup,
    papers: &'a [PrinterPaperCapability],
    default_paper: Option<i16>,
) -> Option<&'a PrinterPaperCapability> {
    let offered = |(width, height): (Length, Length)| {
        papers.iter().find(|paper| {
            let (paper_width, paper_height) = paper.portrait_dimensions_um();
            paper_width.abs_diff(width.micrometres()) <= PAPER_MATCH_UM
                && paper_height.abs_diff(height.micrometres()) <= PAPER_MATCH_UM
        })
    };
    let sheet = match as_drawn.physical_page().paper() {
        PaperSize::MatchAuthoredSheets(media) => media
            .first()
            .filter(|first| {
                media
                    .iter()
                    .all(|other| other.portrait_dimensions() == first.portrait_dimensions())
            })
            .map(AuthoredSheetMedia::portrait_dimensions),
        _ => None,
    };
    let family = sheet.map_or(StandardPaper::Letter, |(width, height)| {
        let ratio = height.micrometres() as f64 / width.micrometres().max(1) as f64;
        if (ratio - std::f64::consts::SQRT_2).abs() < 0.01 {
            StandardPaper::A4
        } else {
            StandardPaper::Letter
        }
    });
    sheet
        .and_then(offered)
        .or_else(|| {
            default_paper.and_then(|id| papers.iter().find(|paper| paper.platform_id() == id))
        })
        .or_else(|| offered(family.portrait_dimensions()))
        .or_else(|| papers.first())
}

/// The sheet as drawn, fitted to the printable area of `paper` inside
/// `margin`, in the sheet's own orientation.
fn fitted_to(
    as_drawn: &HardcopySetup,
    paper: &PrinterPaperCapability,
    margin: Length,
) -> Result<HardcopySetup, HardcopyError> {
    let (width_um, height_um) = paper.portrait_dimensions_um();
    let (width, height) = (
        Length::from_micrometres(width_um),
        Length::from_micrometres(height_um),
    );
    let standard = [
        StandardPaper::Letter,
        StandardPaper::Legal,
        StandardPaper::Tabloid,
        StandardPaper::A4,
        StandardPaper::A3,
        StandardPaper::A2,
        StandardPaper::A1,
        StandardPaper::A0,
    ]
    .into_iter()
    .find(|standard| {
        let (standard_width, standard_height) = standard.portrait_dimensions();
        standard_width.micrometres().abs_diff(width_um) <= PAPER_MATCH_UM
            && standard_height.micrometres().abs_diff(height_um) <= PAPER_MATCH_UM
    });
    let paper = match standard {
        Some(standard) => PaperSize::Standard(standard),
        None => PaperSize::Custom(CustomPaper::try_new(
            paper.display_name(),
            width,
            height,
            LengthUnit::Millimetres,
        )?),
    };
    HardcopySetup::try_new_with_schematic(
        PhysicalPageSetup::try_new(
            paper,
            PageMargins::uniform(margin),
            Bleed::None,
            as_drawn.physical_page().orientation(),
        )?,
        ScaleMode::FitPrintableArea,
        as_drawn.tiling(),
        as_drawn.render().clone(),
        as_drawn.decorations().clone(),
        as_drawn.schematic(),
        as_drawn.print_mapping().clone(),
    )
}

/// The widest strip on any side of the page the driver cannot print,
/// rounded up to a tenth of a millimetre.
fn widest_unprintable_margin(geometry: PrinterRasterGeometry, dpi: u16) -> Length {
    let (width, height) = geometry.physical_size_px();
    let (x, y, printable_width, printable_height) = geometry.printable_rect_px();
    let widest = [
        x,
        y,
        width.saturating_sub(x.saturating_add(printable_width)),
        height.saturating_sub(y.saturating_add(printable_height)),
    ]
    .into_iter()
    .max()
    .unwrap_or(0);
    let micrometres = (u64::from(widest) * 25_400).div_ceil(u64::from(dpi.max(1)));
    Length::from_micrometres(micrometres.div_ceil(100) * 100)
}
