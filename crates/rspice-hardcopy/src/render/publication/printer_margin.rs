//! Whether a printer can lay down every mark on a page.
//!
//! A driver reports the part of the paper it can reach, and nothing outside
//! it prints. The page's planned margins do not decide whether that matters;
//! its ink does. A drawing sheet printed as drawn has no margin at all, yet its
//! frame stands well inside the paper's edge. So the rule is measured on the
//! raster the printer receives: a page is refused only when its ink reaches
//! into the strip the printer cannot print.

use std::fmt;

#[cfg(not(target_arch = "wasm32"))]
use super::*;

/// The box around every pixel of a printer page that is not bare paper, in
/// device pixels: `left` and `top` inclusive, `right` and `bottom` exclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InkBounds {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

/// A page whose ink reaches into the printer's unprintable margin, on the
/// side where it reaches furthest: how wide the margin is there, and how far
/// from the paper's edge the ink comes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrinterMarginShortfall {
    margin_um: u64,
    ink_um: u64,
}

impl PrinterMarginShortfall {
    #[must_use]
    pub const fn margin_um(self) -> u64 {
        self.margin_um
    }

    #[must_use]
    pub const fn ink_um(self) -> u64 {
        self.ink_um
    }
}

impl fmt::Display for PrinterMarginShortfall {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let millimetres = |micrometres: u64| micrometres as f64 / 1_000.0;
        write!(
            formatter,
            "The printer can't print within {:.1} mm of the paper's edge, but this page has ink \
             {:.1} mm from it. Print it scaled to fit, or choose a larger paper.",
            millimetres(self.margin_um),
            millimetres(self.ink_um)
        )
    }
}

/// Where the ink of a printer page lies, read in the same pass that proves
/// the page opaque, so measuring it costs no second walk over the page.
/// `None` is a page with no ink at all.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn opaque_page_ink(
    rgba: &[u8],
    width: u32,
    paper: Rgb8,
) -> Result<Option<InkBounds>, HardcopyRenderError> {
    let paper = u32::from_ne_bytes([paper.red, paper.green, paper.blue, u8::MAX]);
    let mut bounds: Option<InkBounds> = None;
    let row_bytes = width as usize * 4;
    if row_bytes == 0 {
        return Ok(None);
    }
    for (y, row) in (0_u32..).zip(rgba.chunks_exact(row_bytes)) {
        let mut first = None;
        let mut last = 0;
        for (x, pixel) in (0_u32..).zip(row.chunks_exact(4)) {
            if u32::from_ne_bytes([pixel[0], pixel[1], pixel[2], pixel[3]]) != paper {
                if pixel[3] != u8::MAX {
                    return Err(HardcopyRenderError::TransparentPrinterPage);
                }
                first.get_or_insert(x);
                last = x;
            }
        }
        if let Some(first) = first {
            let ink = bounds.get_or_insert(InkBounds {
                left: first,
                top: y,
                right: last + 1,
                bottom: y + 1,
            });
            ink.left = ink.left.min(first);
            ink.right = ink.right.max(last + 1);
            ink.bottom = y + 1;
        }
    }
    Ok(bounds)
}

/// The worst side on which `ink` reaches into the strip outside the driver's
/// printable rectangle (`x`, `y`, `width`, `height` in device pixels) of a
/// `raster_width` × `raster_height` page at `dpi`, or `None` when every mark
/// prints.
#[must_use]
pub fn printer_margin_shortfall(
    ink: Option<InkBounds>,
    (raster_width, raster_height): (u32, u32),
    (x, y, width, height): (u32, u32, u32, u32),
    dpi: u16,
) -> Option<PrinterMarginShortfall> {
    let ink = ink?;
    let printable_right = x.saturating_add(width);
    let printable_bottom = y.saturating_add(height);
    // Each side as (unprintable margin, ink's distance from the edge).
    let sides = [
        (x, ink.left),
        (y, ink.top),
        (
            raster_width.saturating_sub(printable_right),
            raster_width.saturating_sub(ink.right),
        ),
        (
            raster_height.saturating_sub(printable_bottom),
            raster_height.saturating_sub(ink.bottom),
        ),
    ];
    let (margin, ink) = sides
        .into_iter()
        .filter(|(margin, ink)| ink < margin)
        .max_by_key(|(margin, ink)| margin - ink)?;
    let micrometres = |pixels: u32| (u64::from(pixels) * 25_400).div_ceil(u64::from(dpi.max(1)));
    Some(PrinterMarginShortfall {
        margin_um: micrometres(margin),
        ink_um: micrometres(ink),
    })
}
