//! Viewer-image (PNG) export.
//!
//! Native flow: a one-shot request sends `ViewportCommand::Screenshot`;
//! the frame after, the screenshot arrives as an input event, gets cropped
//! to the results document well, gets a save dialog, and is encoded with
//! the `png` crate at physical-pixel resolution. The web build points at
//! the browser's own capture instead of pretending.

use egui::Context;

use super::{ConsoleMessage, RSpiceApp};

impl RSpiceApp {
    /// Pump the export request and any screenshot that arrived this frame.
    pub(super) fn handle_image_export(&mut self, ctx: &Context) {
        if std::mem::take(&mut self.state.shell.export_png_requested) {
            #[cfg(not(target_arch = "wasm32"))]
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
            #[cfg(target_arch = "wasm32")]
            self.state.shell.toasts.warn(
                ctx,
                "PNG export needs the desktop build — use the browser's screenshot instead",
            );
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
        let cropped = self.state.shell.results.well_rect.and_then(|well| {
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

        let viewer = self.state.shell.results.viewer.label().to_lowercase();
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
                self.state.shell.toasts.info(ctx, &message);
                self.state.push_user_message(ConsoleMessage::info(message));
            }
            Err(error) => {
                let message = format!("PNG export failed: {error}");
                self.state.shell.toasts.error(ctx, &message);
                self.state.push_user_message(ConsoleMessage::error(message));
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
    writer
        .write_image_data(&data)
        .map_err(|e| e.to_string())?;
    Ok(())
}
