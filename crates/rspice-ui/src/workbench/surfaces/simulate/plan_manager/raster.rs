//! Offscreen renders of the plan manager, so its layout can be looked at
//! rather than only asserted about.
//!
//! The contract tests next door read what the manager paints and how wide it
//! came out; nothing in them sees the result. This runs the real dialog through
//! a headless context, tessellates, and software-rasterizes to a PNG — no GPU,
//! no window, and no dependence on the wasm build.
//!
//! Every route at every gated viewport — the same eight modes and the same
//! three shapes the contract tests gate, taken from
//! [`tests::GATED_VIEWPORTS`](super::tests::GATED_VIEWPORTS) rather than
//! restated, so a render is of the arrangement that was actually asserted
//! about. Each canvas is exactly its viewport: the product rule is that the
//! whole surface fits without scrolling, and a taller canvas would let a
//! surface that overflows the reader's screen look complete here.
//!
//! Run with `--ignored`; the renders go to `RSPICE_RASTER_DIR` (default: the
//! system temp directory).
//!
//! # What these images are evidence of, and what they are not
//!
//! **Trust the layout; do not trust the text.** [`atlas_coverage`] samples the
//! font atlas with nearest-neighbour rounding and no filtering, so a glyph
//! feature one pixel thick can land between two samples and vanish. The visible
//! symptom is a capital `T` losing its crossbar — `PVT` rasterizes as `PVI` —
//! and thin strokes elsewhere thinning or breaking. Nothing is wrong with the
//! surface when that happens.
//!
//! So these renders are sound evidence about geometry: column alignment,
//! spacing and rhythm, where a rule falls, whether a row is clipped, whether a
//! surface overflows its viewport. They are unreliable evidence about wording,
//! and a glyph read off one of them is not a defect report — check the string
//! in `tests.rs`, which reads the galley rather than the pixels.
//!
//! The defect is inherited along with the rasterizer: `simulate/page_raster.rs`
//! samples the same way. Fixing it means bilinear sampling in both copies, or
//! the one shared rasterizer the note below already asks for, and neither is
//! this module's to make.
//!
//! The triangle fill and the PNG encoder are the recipe `simulate/page_raster.rs`
//! established for the studio pages. That module is a sibling rather than an
//! ancestor, so its rasterizer is not reachable from here and this carries its
//! own copy; extracting one shared test-only rasterizer under `simulate/` is
//! the right fix and belongs to whoever owns that file.

// Native-only, like the render harnesses in `tests.rs` it shares fixtures with:
// it rasterizes to a file for a person to open.
#![cfg(not(target_arch = "wasm32"))]

use egui::{Rect, Vec2};

use crate::workbench::state::SimulationPlanManagerMode;

struct Canvas {
    width: usize,
    height: usize,
    /// Premultiplied sRGBA, matching `Color32`.
    pixels: Vec<[u8; 4]>,
}

impl Canvas {
    fn new(width: usize, height: usize, fill: egui::Color32) -> Self {
        Self {
            width,
            height,
            pixels: vec![fill.to_array(); width * height],
        }
    }

    fn blend(&mut self, x: usize, y: usize, src: [f32; 4]) {
        let dst = &mut self.pixels[y * self.width + x];
        let inv = 1.0 - src[3] / 255.0;
        for channel in 0..4 {
            dst[channel] = (src[channel] + f32::from(dst[channel]) * inv)
                .round()
                .clamp(0.0, 255.0) as u8;
        }
    }

    /// The last row anything painted on, so a surface shorter than the canvas
    /// is not reported as acres of empty space.
    fn content_height(&self, background: egui::Color32) -> usize {
        let background = background.to_array();
        (0..self.height)
            .rev()
            .find(|row| {
                self.pixels[row * self.width..(row + 1) * self.width]
                    .iter()
                    .any(|pixel| *pixel != background)
            })
            .map_or(0, |row| row + 1)
    }
}

/// Render one route at one gated viewport and rasterize it.
///
/// The fixture and the draft are the contract tests' own — a catalog with one
/// plan of each lifecycle state, and the entry state
/// [`route_draft`](super::tests::route_draft) documents each route is reachable
/// in. A render seeded any other way would show an arrangement no reader can
/// get to, which is worse than no render at all.
fn raster(mode: SimulationPlanManagerMode, screen: Vec2) -> (Canvas, egui::Color32) {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let background = crate::ui::tokens::Tokens::get(&ctx).color.bg_app;

    let (mut app, active, available, _) = super::tests::app_with_every_lifecycle_state();
    let draft = super::tests::route_draft(&app, mode, active, available);

    let input = || egui::RawInput {
        screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, screen)),
        ..Default::default()
    };
    // The dialog owns the draft for the frame and re-arms it in
    // `simulation_workflow`; each pass hands it a fresh one so the three
    // passes render the same state rather than accumulating it.
    //
    // The application is captured rather than taken as a parameter on purpose.
    // `tests/module_layering.rs` counts every whole-application mutable
    // parameter in the crate, test-only files included, and a render harness is
    // not a reason for that ceiling to move.
    let mut pass = || {
        ctx.run_ui(input(), |ctx| {
            super::plan_manager_dialog(ctx, &mut app, draft.clone());
        })
    };

    // Fonts build on the first pass and the surface settles on the second, so
    // the third is the one worth looking at.
    let _ = pass();
    let _ = pass();
    let output = pass();

    let atlas = ctx.fonts(|fonts| fonts.image());
    let primitives = ctx.tessellate(output.shapes, 1.0);

    let mut canvas = Canvas::new(screen.x as usize, screen.y as usize, background);
    for primitive in &primitives {
        let egui::epaint::Primitive::Mesh(mesh) = &primitive.primitive else {
            continue;
        };
        let clip = primitive.clip_rect;
        for triangle in mesh.indices.chunks_exact(3) {
            let vertices = [
                mesh.vertices[triangle[0] as usize],
                mesh.vertices[triangle[1] as usize],
                mesh.vertices[triangle[2] as usize],
            ];
            fill_triangle(&mut canvas, &atlas, &vertices, clip);
        }
    }
    (canvas, background)
}

fn fill_triangle(
    canvas: &mut Canvas,
    atlas: &egui::ColorImage,
    vertices: &[egui::epaint::Vertex; 3],
    clip: Rect,
) {
    let positions = vertices.map(|vertex| vertex.pos);
    let min_x = positions
        .iter()
        .fold(f32::MAX, |acc, p| acc.min(p.x))
        .max(clip.min.x)
        .max(0.0)
        .floor() as usize;
    let max_x = (positions
        .iter()
        .fold(f32::MIN, |acc, p| acc.max(p.x))
        .min(clip.max.x)
        .min(canvas.width as f32 - 1.0)
        .ceil()) as isize;
    let min_y = positions
        .iter()
        .fold(f32::MAX, |acc, p| acc.min(p.y))
        .max(clip.min.y)
        .max(0.0)
        .floor() as usize;
    let max_y = (positions
        .iter()
        .fold(f32::MIN, |acc, p| acc.max(p.y))
        .min(clip.max.y)
        .min(canvas.height as f32 - 1.0)
        .ceil()) as isize;
    if max_x < min_x as isize || max_y < min_y as isize {
        return;
    }

    let area = edge(positions[0], positions[1], positions[2]);
    if area.abs() < f32::EPSILON {
        return;
    }

    for y in min_y..=(max_y as usize) {
        for x in min_x..=(max_x as usize) {
            let point = egui::pos2(x as f32 + 0.5, y as f32 + 0.5);
            let w0 = edge(positions[1], positions[2], point) / area;
            let w1 = edge(positions[2], positions[0], point) / area;
            let w2 = edge(positions[0], positions[1], point) / area;
            if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                continue;
            }
            let weights = [w0, w1, w2];
            let mut color = [0.0f32; 4];
            let mut uv = egui::Vec2::ZERO;
            for (vertex, weight) in vertices.iter().zip(weights) {
                let channels = vertex.color.to_array();
                for channel in 0..4 {
                    color[channel] += f32::from(channels[channel]) * weight;
                }
                uv += vertex.uv.to_vec2() * weight;
            }
            let coverage = atlas_coverage(atlas, uv);
            for channel in &mut color {
                *channel *= coverage;
            }
            if color[3] <= 0.5 {
                continue;
            }
            canvas.blend(x, y, color);
        }
    }
}

fn edge(a: egui::Pos2, b: egui::Pos2, c: egui::Pos2) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

/// Glyph alpha at a normalized atlas coordinate.
///
/// The tessellator points untextured geometry at the atlas's reserved white
/// pixel, so sampling unconditionally is correct for both cases.
fn atlas_coverage(atlas: &egui::ColorImage, uv: egui::Vec2) -> f32 {
    let [width, height] = atlas.size;
    if width == 0 || height == 0 {
        return 1.0;
    }
    let x = ((uv.x * width as f32).round() as isize).clamp(0, width as isize - 1) as usize;
    let y = ((uv.y * height as f32).round() as isize).clamp(0, height as isize - 1) as usize;
    f32::from(atlas.pixels[y * width + x].a()) / 255.0
}

fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

fn adler32(bytes: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for byte in bytes {
        a = (a + u32::from(*byte)) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let mut body = kind.to_vec();
    body.extend_from_slice(data);
    out.extend_from_slice(&body);
    out.extend_from_slice(&crc32(&body).to_be_bytes());
}

/// A PNG with stored (uncompressed) deflate blocks. The crate has no image
/// dependency and this is a review artifact, so size does not matter.
fn png(canvas: &Canvas, height: usize) -> Vec<u8> {
    let mut raw = Vec::with_capacity(height * (canvas.width * 4 + 1));
    for y in 0..height {
        raw.push(0); // filter: none
        for x in 0..canvas.width {
            let [r, g, b, a] = canvas.pixels[y * canvas.width + x];
            // PNG wants straight alpha; `Color32` is premultiplied.
            let straight = |channel: u8| {
                if a == 0 {
                    0
                } else {
                    ((f32::from(channel) * 255.0 / f32::from(a)).round()).clamp(0.0, 255.0) as u8
                }
            };
            raw.extend_from_slice(&[straight(r), straight(g), straight(b), a]);
        }
    }

    let mut zlib = vec![0x78, 0x01];
    for (index, block) in raw.chunks(65535).enumerate() {
        let last = (index + 1) * 65535 >= raw.len();
        zlib.push(u8::from(last));
        zlib.extend_from_slice(&(block.len() as u16).to_le_bytes());
        zlib.extend_from_slice(&(!(block.len() as u16)).to_le_bytes());
        zlib.extend_from_slice(block);
    }
    zlib.extend_from_slice(&adler32(&raw).to_be_bytes());

    let mut out = vec![137, 80, 78, 71, 13, 10, 26, 10];
    let mut header = Vec::new();
    header.extend_from_slice(&(canvas.width as u32).to_be_bytes());
    header.extend_from_slice(&(height as u32).to_be_bytes());
    header.extend_from_slice(&[8, 6, 0, 0, 0]);
    chunk(&mut out, b"IHDR", &header);
    chunk(&mut out, b"IDAT", &zlib);
    chunk(&mut out, b"IEND", &[]);
    out
}

/// A route's name in a file name: its variant, lowercased.
///
/// Taken from `Debug` rather than authored beside the enum, so a renamed
/// variant renames its renders instead of leaving a file named after a mode
/// that no longer exists.
fn file_stem(mode: SimulationPlanManagerMode, viewport: &str) -> String {
    format!(
        "plan-manager-{}-{}",
        format!("{mode:?}").to_ascii_lowercase(),
        viewport.replace(' ', "-")
    )
}

/// Write every route at every gated viewport to PNGs so the design can be
/// reviewed.
///
/// Twenty-four images: eight modes at three shapes. The list of modes is
/// [`tests::EVERY_ROUTE`](super::tests::EVERY_ROUTE), which the coverage claim
/// beside it holds to the mode enum, so a ninth route arrives here through the
/// same edit that dispatches it rather than whenever someone remembers this
/// file.
///
/// Read them for layout, not for wording — the module header says why.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_every_route_at_every_gated_viewport() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report_output = stderr.lock();

    for mode in super::tests::EVERY_ROUTE {
        for (viewport, screen) in super::tests::GATED_VIEWPORTS {
            let (canvas, background) = raster(mode, screen);
            // Cropped to the last row anything painted on, so a surface shorter
            // than its viewport is not reported as acres of empty space. A
            // surface *taller* than its viewport cannot hide here: the canvas
            // is the viewport, so the overflow is simply cut off, and the fit
            // gates are what prove there is none.
            let height = canvas.content_height(background).max(1);
            let bytes = png(&canvas, height);
            let path = directory.join(format!("{}.png", file_stem(mode, viewport)));
            std::fs::write(&path, &bytes).expect("write plan-manager render");
            writeln!(
                report_output,
                "{} {}x{} {} bytes",
                path.display(),
                canvas.width,
                height,
                bytes.len()
            )
            .expect("write raster qualification report");
        }
    }
}
