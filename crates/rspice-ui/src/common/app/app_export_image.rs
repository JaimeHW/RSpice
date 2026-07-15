//! Viewer-image (PNG) export.
//!
//! Native flow: a one-shot request sends `ViewportCommand::Screenshot`;
//! the frame after, the screenshot arrives as an input event, gets cropped
//! to the results document well, gets a save dialog, and is encoded with
//! the `png` crate at physical-pixel resolution. The web build exports the
//! active canvas through the browser download path, cropped to the same well
//! when the browser permits it.

use egui::Context;

#[cfg(not(target_arch = "wasm32"))]
use super::ConsoleMessage;
use super::RSpiceApp;

impl RSpiceApp {
    /// Pump the export request and any screenshot that arrived this frame.
    pub(super) fn handle_image_export(&mut self, ctx: &Context) {
        if std::mem::take(&mut self.state.ui.export_png_requested) {
            #[cfg(not(target_arch = "wasm32"))]
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
            #[cfg(target_arch = "wasm32")]
            self.save_browser_canvas_png(ctx);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let screenshot = ctx.input(|input| {
                input.events.iter().find_map(|event| match event {
                    egui::Event::Screenshot { image, .. } => Some(image.clone()),
                    _ => None,
                })
            });
            if let Some(image) = screenshot {
                self.save_screenshot_png(ctx, &image);
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save_screenshot_png(&mut self, ctx: &Context, image: &egui::ColorImage) {
        // Crop the window capture to the document well so the export is
        // the viewer, not the chrome around it. Falls back to the full
        // window when no well rect was recorded.
        let pixels_per_point = ctx.pixels_per_point();
        let cropped = self.state.ui.results.well_rect.and_then(|well| {
            let image_rect = egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(
                    image.size[0] as f32 / pixels_per_point,
                    image.size[1] as f32 / pixels_per_point,
                ),
            );
            let well = well.intersect(image_rect);
            (well.width() > 1.0 && well.height() > 1.0)
                .then(|| image.region(&well, Some(pixels_per_point)))
        });
        let image = cropped.as_ref().unwrap_or(image);

        let viewer = self.state.ui.results.viewer.label().to_lowercase();
        let Some(path) = rfd::FileDialog::new()
            .add_filter("PNG image", &["png"])
            .set_file_name(format!("rspice-{viewer}.png"))
            .save_file()
        else {
            return;
        };

        let result = write_png(&path, image);
        match result {
            Ok(()) => {
                let message = format!("Exported window image: {}", path.display());
                self.state.ui.toasts.info(ctx, &message);
                self.state.push_user_message(ConsoleMessage::info(message));
            }
            Err(error) => {
                let message = format!("PNG export failed: {error}");
                self.state.ui.toasts.error(ctx, &message);
                self.state.push_user_message(ConsoleMessage::error(message));
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn save_browser_canvas_png(&mut self, ctx: &Context) {
        let viewer = self.state.ui.results.viewer.label().to_lowercase();
        let filename = format!("rspice-{viewer}.png");
        let result = download_browser_canvas_png(ctx, &filename, self.state.ui.results.well_rect);
        match result {
            Ok(()) => {
                let message = format!(
                    "Viewer image download started: {filename} (confirm the browser accepted the download)"
                );
                self.state.ui.toasts.info(ctx, &message);
                self.state
                    .push_user_message(super::ConsoleMessage::info(message));
            }
            Err(error) => {
                let message = format!("PNG export failed: {error}");
                self.state.ui.toasts.error(ctx, &message);
                self.state
                    .push_user_message(super::ConsoleMessage::error(message));
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn write_png(path: &std::path::Path, image: &egui::ColorImage) -> Result<(), String> {
    let expected = crate::io::durable_file::observe_expected_content(path)
        .map_err(|error| format!("failed to authorize PNG destination: {error}"))?;
    let encoded = encode_png(image)?;
    publish_png(path, expected, &encoded)
}

#[cfg(not(target_arch = "wasm32"))]
fn encode_png(image: &egui::ColorImage) -> Result<Vec<u8>, String> {
    let width = u32::try_from(image.size[0])
        .map_err(|_| "PNG width exceeds the format limit".to_string())?;
    let height = u32::try_from(image.size[1])
        .map_err(|_| "PNG height exceeds the format limit".to_string())?;
    if width == 0 || height == 0 {
        return Err("PNG dimensions must be non-zero".to_string());
    }
    let expected_pixels = image.size[0]
        .checked_mul(image.size[1])
        .ok_or_else(|| "PNG pixel count overflowed".to_string())?;
    if image.pixels.len() != expected_pixels {
        return Err(format!(
            "PNG pixel buffer has {} pixels for a {}x{} image",
            image.pixels.len(),
            image.size[0],
            image.size[1]
        ));
    }
    let rgba_len = expected_pixels
        .checked_mul(4)
        .ok_or_else(|| "PNG RGBA buffer size overflowed".to_string())?;
    let mut rgba = Vec::new();
    rgba.try_reserve_exact(rgba_len)
        .map_err(|error| format!("failed to allocate PNG RGBA buffer: {error}"))?;
    for pixel in &image.pixels {
        rgba.extend_from_slice(&pixel.to_array());
    }

    let mut encoded = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut encoded, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
        writer.write_image_data(&rgba).map_err(|e| e.to_string())?;
        writer.finish().map_err(|e| e.to_string())?;
    }
    Ok(encoded)
}

#[cfg(not(target_arch = "wasm32"))]
fn publish_png(
    path: &std::path::Path,
    expected: crate::io::durable_file::ExpectedContent,
    encoded: &[u8],
) -> Result<(), String> {
    crate::io::durable_file::compare_exchange_bytes(path, expected, encoded)
        .map_err(|error| format!("failed to publish PNG: {error}"))
}

#[cfg(target_arch = "wasm32")]
fn download_browser_canvas_png(
    ctx: &Context,
    filename: &str,
    well_rect: Option<egui::Rect>,
) -> Result<(), String> {
    use wasm_bindgen::JsCast as _;

    let window = web_sys::window().ok_or_else(|| "Browser window is unavailable".to_string())?;
    let document = window
        .document()
        .ok_or_else(|| "Browser document is unavailable".to_string())?;
    let source_canvas = document
        .get_element_by_id("rspice_canvas")
        .ok_or_else(|| "RSpice canvas is unavailable".to_string())?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| "RSpice canvas element has the wrong type".to_string())?;

    let crop = well_rect.and_then(|well| {
        browser_canvas_crop_rect(
            [source_canvas.width(), source_canvas.height()],
            well,
            ctx.pixels_per_point(),
        )
    });
    let export_canvas = match crop {
        Some(crop) => crop_canvas(&document, &source_canvas, crop)?,
        None => source_canvas,
    };
    let data_url = export_canvas
        .to_data_url_with_type("image/png")
        .map_err(js_error_message)?;
    crate::common::browser_download::download_href(filename, &data_url)
}

#[cfg(target_arch = "wasm32")]
fn crop_canvas(
    document: &web_sys::Document,
    source_canvas: &web_sys::HtmlCanvasElement,
    crop: CanvasCropRect,
) -> Result<web_sys::HtmlCanvasElement, String> {
    use wasm_bindgen::JsCast as _;

    let export_canvas = document
        .create_element("canvas")
        .map_err(js_error_message)?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| "Browser canvas export target has the wrong type".to_string())?;
    export_canvas.set_width(crop.width);
    export_canvas.set_height(crop.height);

    let context = export_canvas
        .get_context("2d")
        .map_err(js_error_message)?
        .ok_or_else(|| "Browser 2D canvas context is unavailable".to_string())?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|_| "Browser 2D canvas context has the wrong type".to_string())?;
    context
        .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            source_canvas,
            crop.x as f64,
            crop.y as f64,
            crop.width as f64,
            crop.height as f64,
            0.0,
            0.0,
            crop.width as f64,
            crop.height as f64,
        )
        .map_err(js_error_message)?;

    Ok(export_canvas)
}

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CanvasCropRect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[cfg(any(test, target_arch = "wasm32"))]
fn browser_canvas_crop_rect(
    canvas_size: [u32; 2],
    well: egui::Rect,
    pixels_per_point: f32,
) -> Option<CanvasCropRect> {
    if canvas_size[0] == 0 || canvas_size[1] == 0 || pixels_per_point <= 0.0 {
        return None;
    }

    let canvas_width = canvas_size[0] as f32;
    let canvas_height = canvas_size[1] as f32;
    let min_x = (well.min.x * pixels_per_point)
        .floor()
        .clamp(0.0, canvas_width);
    let min_y = (well.min.y * pixels_per_point)
        .floor()
        .clamp(0.0, canvas_height);
    let max_x = (well.max.x * pixels_per_point)
        .ceil()
        .clamp(0.0, canvas_width);
    let max_y = (well.max.y * pixels_per_point)
        .ceil()
        .clamp(0.0, canvas_height);

    if max_x <= min_x || max_y <= min_y {
        return None;
    }

    let width = (max_x - min_x).round() as u32;
    let height = (max_y - min_y).round() as u32;
    (width > 1 && height > 1).then(|| CanvasCropRect {
        x: min_x.round() as u32,
        y: min_y.round() as u32,
        width,
        height,
    })
}

#[cfg(target_arch = "wasm32")]
fn js_error_message(error: wasm_bindgen::JsValue) -> String {
    error
        .as_string()
        .unwrap_or_else(|| "Browser PNG export failed".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browser_png_crop_uses_well_rect_in_physical_pixels() {
        let crop = browser_canvas_crop_rect(
            [1600, 900],
            egui::Rect::from_min_size(egui::pos2(50.0, 25.0), egui::vec2(700.0, 300.0)),
            2.0,
        )
        .expect("crop is inside canvas");

        assert_eq!(
            crop,
            CanvasCropRect {
                x: 100,
                y: 50,
                width: 1400,
                height: 600,
            }
        );
    }

    #[test]
    fn browser_png_crop_clamps_to_canvas_bounds() {
        let crop = browser_canvas_crop_rect(
            [800, 600],
            egui::Rect::from_min_size(egui::pos2(-10.0, 250.0), egui::vec2(500.0, 200.0)),
            2.0,
        )
        .expect("crop overlaps canvas");

        assert_eq!(
            crop,
            CanvasCropRect {
                x: 0,
                y: 500,
                width: 800,
                height: 100,
            }
        );
    }

    #[test]
    fn browser_png_crop_rejects_non_overlapping_or_degenerate_rects() {
        assert_eq!(
            browser_canvas_crop_rect(
                [800, 600],
                egui::Rect::from_min_size(egui::pos2(900.0, 20.0), egui::vec2(100.0, 100.0)),
                1.0,
            ),
            None
        );
        assert_eq!(
            browser_canvas_crop_rect(
                [800, 600],
                egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(0.5, 40.0)),
                1.0,
            ),
            None
        );
        assert_eq!(
            browser_canvas_crop_rect(
                [800, 600],
                egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(40.0, 40.0)),
                0.0,
            ),
            None
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn png_encode_failure_preserves_existing_destination() {
        let root = unique_temp_dir("png-encode-failure");
        let path = root.join("plot.png");
        std::fs::write(&path, b"predecessor").expect("write predecessor");
        let invalid = egui::ColorImage {
            size: [2, 2],
            source_size: egui::vec2(2.0, 2.0),
            pixels: vec![egui::Color32::RED],
        };

        let result = write_png(&path, &invalid);

        assert!(result.is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"predecessor");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn png_publication_rejects_late_external_change() {
        let root = unique_temp_dir("png-late-change");
        let path = root.join("plot.png");
        std::fs::write(&path, b"authorized predecessor").expect("write predecessor");
        let expected =
            crate::io::durable_file::observe_expected_content(&path).expect("observe destination");
        let image = egui::ColorImage::filled([2, 2], egui::Color32::BLUE);
        let encoded = encode_png(&image).expect("encode PNG");
        std::fs::write(&path, b"late external edit").expect("race destination");

        let result = publish_png(&path, expected, &encoded);

        assert!(result.is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"late external edit");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn unique_temp_dir(label: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "rspice-image-export-{label}-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&root).expect("create fixture");
        root
    }
}
