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
    let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    let writer = std::io::BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, image.size[0] as u32, image.size[1] as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(|e| e.to_string())?;

    let mut data = Vec::with_capacity(image.pixels.len() * 4);
    for pixel in &image.pixels {
        data.extend_from_slice(&pixel.to_array());
    }
    writer.write_image_data(&data).map_err(|e| e.to_string())?;
    Ok(())
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
}
