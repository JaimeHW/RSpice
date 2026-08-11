//! Viewer-figure export.
//!
//! Native builds hand this to the hardcopy publication pipeline — the one
//! that already renders every result sheet and every schematic as PDF/A, PDF,
//! SVG or PNG at a chosen resolution, attaches provenance, and which Print has
//! always used. What was here instead was a second, worse export beside it: it
//! sent `ViewportCommand::Screenshot`, waited a frame, cropped the window
//! capture to the document well and encoded that. A "figure" was therefore the
//! pixels already on screen, at whatever the display happened to be, with the
//! chrome removed by rectangle — no resolution, no vector format, no page
//! size, no colour profile, no provenance.
//!
//! The browser build keeps its canvas capture, because the pipeline's
//! export-artifact finalization (`dialogs::hardcopy::finalize`) is native-only:
//! the renderer itself compiles for WebAssembly, but nothing yet carries its
//! bytes to a download. Routing the browser at a workflow whose primary action
//! cannot complete would be worse than the crude capture it replaced. That
//! boundary is the remaining work, not a design.

use egui::Context;

use crate::workbench::app::RSpiceApp;

impl RSpiceApp {
    /// Pump a pending figure-export request.
    pub(in crate::workbench) fn handle_image_export(&mut self, ctx: &Context) {
        if !std::mem::take(&mut self.state.ui.export_figure_requested) {
            return;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = ctx;
            crate::workbench::app::open_hardcopy_workflow(
                self,
                crate::workbench::app::HardcopyWorkflow::Export,
            );
        }
        #[cfg(target_arch = "wasm32")]
        self.save_browser_canvas_png(ctx);
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
                self.state.ui.toasts.success(
                    ctx,
                    "Viewer image download started",
                    format!(
                        "{filename} was handed to the browser. Confirm the download completed."
                    ),
                );
                self.state
                    .push_user_message(crate::diagnostics::ConsoleMessage::info(message));
            }
            Err(error) => {
                let message = format!("PNG export failed: {error}");
                self.state
                    .ui
                    .toasts
                    .error_with_title(ctx, "PNG export failed", error.to_string());
                self.state
                    .push_user_message(crate::diagnostics::ConsoleMessage::error(message));
            }
        }
    }
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
    crate::workbench::browser::download::download_href(filename, &data_url)
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
