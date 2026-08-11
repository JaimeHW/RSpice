//! Rasterizing a page to PNG or TIFF.
//!
//! Pixel dimensions are derived from the page's exact micrometre size and the
//! requested DPI, and the total is checked against the working-set budget
//! before any buffer is allocated — an oversized raster is refused rather than
//! attempted and killed. Soft-proof preview is applied to the copy that is
//! displayed, never to the bytes that are written out.

use super::*;

#[derive(Debug)]
pub(super) struct RasterPage {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) rgba: Vec<u8>,
}

/// Peak working-set cost per pixel, per raster format. PNG holds the RGBA
/// pixmap and the encoder's window; TIFF additionally materialises an RGB8
/// copy of the page before compressing it. These are the numbers the publisher
/// charges, so anything that predicts a refusal has to charge the same ones.
pub(super) const PNG_BYTES_PER_PIXEL: u64 = 8;
/// See [`PNG_BYTES_PER_PIXEL`].
pub(super) const TIFF_BYTES_PER_PIXEL: u64 = 12;

/// The greatest resolution this plan's pages can be rastered at on this host,
/// or `None` when the format is not raster or even [`MIN_RASTER_DPI`] does not
/// fit.
///
/// Derived by asking the same predicates the publisher runs, never by
/// inverting them: `raster_dimensions` rounds each axis up independently, the
/// two formats charge different bytes per pixel, and a closed form would drift
/// from all of that the first time any of it changed — which is precisely how
/// a dialog comes to offer a setting its own renderer refuses. Pixel counts
/// rise monotonically with resolution, so bisecting the contract's range finds
/// the exact boundary.
pub(crate) fn max_raster_dpi(plan: &HardcopyPlan, format: OutputFormat) -> Option<u16> {
    let bytes_per_pixel = match format {
        OutputFormat::Png { .. } => PNG_BYTES_PER_PIXEL,
        OutputFormat::Tiff { .. } => TIFF_BYTES_PER_PIXEL,
        _ => return None,
    };
    let admits = |dpi| raster_budget_admits(plan, dpi, bytes_per_pixel);
    if !admits(MIN_RASTER_DPI) {
        return None;
    }
    if admits(MAX_RASTER_DPI) {
        return Some(MAX_RASTER_DPI);
    }
    let (mut admitted, mut refused) = (MIN_RASTER_DPI, MAX_RASTER_DPI);
    while admitted + 1 < refused {
        let midpoint = admitted + (refused - admitted) / 2;
        if admits(midpoint) {
            admitted = midpoint;
        } else {
            refused = midpoint;
        }
    }
    Some(admitted)
}

/// Every budget a raster publication must satisfy, evaluated together.
/// `aggregate_raster_pixels` already refuses a single oversized page, so the
/// remaining checks are the aggregate and the working set.
fn raster_budget_admits(plan: &HardcopyPlan, dpi: u16, bytes_per_pixel: u64) -> bool {
    let Ok((total_pixels, largest_page)) = aggregate_raster_pixels(plan, dpi) else {
        return false;
    };
    total_pixels <= MAX_RASTER_PIXELS_TOTAL
        && validate_raster_working_set(largest_page, 1, bytes_per_pixel).is_ok()
}

pub(super) fn raster_dimensions(
    page: &PreviewPage,
    dpi: u16,
) -> Result<(u32, u32, u64), HardcopyRenderError> {
    let (width, height) = page.geometry().physical_size();
    let pixels = |length: Length| -> Result<u32, HardcopyRenderError> {
        let numerator = u128::from(length.micrometres()) * u128::from(dpi);
        let value = numerator.div_ceil(u128::from(MICROMETRES_PER_INCH));
        u32::try_from(value).map_err(|_| HardcopyRenderError::RasterDimensionOverflow)
    };
    let width = pixels(width)?;
    let height = pixels(height)?;
    let count = u64::from(width)
        .checked_mul(u64::from(height))
        .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
    if width == 0 || height == 0 || count > MAX_RASTER_PIXELS_PER_PAGE {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "raster pixels per page",
            maximum: MAX_RASTER_PIXELS_PER_PAGE,
        });
    }
    Ok((width, height, count))
}

pub(super) fn aggregate_raster_pixels(
    plan: &HardcopyPlan,
    dpi: u16,
) -> Result<(u64, u64), HardcopyRenderError> {
    let mut total = 0_u64;
    let mut largest = 0_u64;
    for page in plan.pagination().pages() {
        let (_, _, pixels) = raster_dimensions(page, dpi)?;
        total = total
            .checked_add(pixels)
            .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
        largest = largest.max(pixels);
    }
    Ok((total, largest))
}

pub(super) fn rasterize_page(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    page: &PreviewPage,
    dpi: u16,
) -> Result<RasterPage, HardcopyRenderError> {
    let (width, height, _) = raster_dimensions(page, dpi)?;
    rasterize_page_at_dimensions(plan, scene, page, dpi, width, height)
}

pub(super) fn rasterize_page_at_dimensions(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    page: &PreviewPage,
    dpi: u16,
    width: u32,
    height: u32,
) -> Result<RasterPage, HardcopyRenderError> {
    let svg = render_page_svg(plan, scene, page)?;
    let mut options = usvg::Options {
        dpi: f32::from(dpi),
        font_family: "RSpice Plex Sans".to_owned(),
        ..usvg::Options::default()
    };
    options
        .fontdb_mut()
        .load_font_data(IBM_PLEX_SANS_REGULAR.to_vec());
    options
        .fontdb_mut()
        .load_font_data(IBM_PLEX_SANS_SEMIBOLD.to_vec());
    options
        .fontdb_mut()
        .load_font_data(IBM_PLEX_MONO_REGULAR.to_vec());
    let tree = usvg::Tree::from_str(&svg, &options)
        .map_err(|error| HardcopyRenderError::SvgParse(error.to_string()))?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height)
        .ok_or(HardcopyRenderError::RasterAllocation)?;
    let transform = resvg::tiny_skia::Transform::from_scale(
        width as f32 / tree.size().width(),
        height as f32 / tree.size().height(),
    );
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    Ok(RasterPage {
        width,
        height,
        rgba: pixmap.take(),
    })
}

pub(super) fn apply_soft_proof_preview(rgba: &mut [u8]) {
    // Deterministic, deliberately conservative uncoated-paper proof. This is
    // a dialog aid, not a color-management claim: it compresses chroma and
    // dynamic range against a warm paper white while retaining alpha.
    for pixel in rgba.chunks_exact_mut(4) {
        let red = u16::from(pixel[0]);
        let green = u16::from(pixel[1]);
        let blue = u16::from(pixel[2]);
        let luminance = (red * 54 + green * 183 + blue * 19) / 256;
        let proof = |channel: u16, paper: u16| -> u8 {
            let desaturated = (channel * 3 + luminance) / 4;
            let compressed = 18 + desaturated * 217 / 255;
            u8::try_from(compressed * paper / 255).unwrap_or(u8::MAX)
        };
        pixel[0] = proof(red, 250);
        pixel[1] = proof(green, 247);
        pixel[2] = proof(blue, 238);
    }
}

pub(super) fn encode_png(page: &RasterPage, dpi: u16) -> Result<Vec<u8>, HardcopyRenderError> {
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, page.width, page.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::Balanced);
        encoder.set_filter(png::Filter::Adaptive);
        let pixels_per_metre = u32::from(dpi)
            .checked_mul(10_000)
            .ok_or(HardcopyRenderError::RasterDimensionOverflow)?
            .div_ceil(254);
        encoder.set_pixel_dims(Some(png::PixelDimensions {
            xppu: pixels_per_metre,
            yppu: pixels_per_metre,
            unit: png::Unit::Meter,
        }));
        let mut writer = encoder
            .write_header()
            .map_err(|error| HardcopyRenderError::Encoding {
                format: "PNG",
                message: error.to_string(),
            })?;
        writer
            .write_image_data(&page.rgba)
            .map_err(|error| HardcopyRenderError::Encoding {
                format: "PNG",
                message: error.to_string(),
            })?;
        writer
            .finish()
            .map_err(|error| HardcopyRenderError::Encoding {
                format: "PNG",
                message: error.to_string(),
            })?;
    }
    Ok(bytes)
}

pub(super) fn render_tiff(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    dpi: u16,
) -> Result<Vec<u8>, HardcopyRenderError> {
    let (total_pixels, largest_page) = aggregate_raster_pixels(plan, dpi)?;
    if total_pixels > MAX_RASTER_PIXELS_TOTAL {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "aggregate raster pixels",
            maximum: MAX_RASTER_PIXELS_TOTAL,
        });
    }
    validate_raster_working_set(largest_page, 1, TIFF_BYTES_PER_PIXEL)?;
    let mut bytes = Vec::new();
    let cursor = Cursor::new(&mut bytes);
    let mut encoder = TiffEncoder::new(cursor)
        .map_err(|error| HardcopyRenderError::Encoding {
            format: "TIFF",
            message: error.to_string(),
        })?
        .with_compression(Compression::Deflate(DeflateLevel::Balanced));
    for page in plan.pagination().pages() {
        let raster = rasterize_page(plan, scene, page, dpi)?;
        let rgb_capacity = raster
            .rgba
            .len()
            .checked_div(4)
            .and_then(|pixels| pixels.checked_mul(3))
            .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
        let mut rgb = Vec::with_capacity(rgb_capacity);
        for pixel in raster.rgba.chunks_exact(4) {
            rgb.extend_from_slice(&pixel[..3]);
        }
        let mut image = encoder
            .new_image::<colortype::RGB8>(raster.width, raster.height)
            .map_err(|error| HardcopyRenderError::Encoding {
                format: "TIFF",
                message: error.to_string(),
            })?;
        image.resolution(
            ResolutionUnit::Inch,
            Rational {
                n: u32::from(dpi),
                d: 1,
            },
        );
        image
            .write_data(&rgb)
            .map_err(|error| HardcopyRenderError::Encoding {
                format: "TIFF",
                message: error.to_string(),
            })?;
    }
    Ok(bytes)
}
