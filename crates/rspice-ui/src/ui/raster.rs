//! Software rasterizer for offscreen renders, so what a surface paints can be
//! asserted about rather than only what it says.
//!
//! Test-only, and deliberately independent of every route the application
//! normally takes to a screen: no GPU, no window, no wasm build, no
//! screenshotting. A caller's pass runs through a headless [`egui::Context`],
//! is tessellated, and is filled here in software, so a render costs nothing
//! but CPU and comes out the same on every machine.
//!
//! # What a render is evidence of
//!
//! Geometry. Column alignment, spacing and rhythm, where a rule falls, whether
//! a row is clipped, whether a surface overflows its viewport.
//!
//! Not wording. [`atlas_coverage`] samples the font atlas with
//! nearest-neighbour rounding and no filtering, so a glyph feature one pixel
//! thick can land between two samples and vanish — a capital `T` losing its
//! crossbar rasterizes `PVT` as `PVI`. A glyph read off a render is not a
//! defect report; assert on the galley instead.
//!
//! # PNGs are for people
//!
//! [`Canvas::png`] encodes a render for someone to open. Every test that writes
//! one is `#[ignore]`: an image nobody looks at proves nothing, and writing
//! files is not what the suite is for. Tests that assert stay in the suite and
//! read pixels through [`Canvas::pixels_in`].

use egui::{Color32, Rect, Vec2};

/// A rasterized render, and the colour it was cleared to.
pub(crate) struct Canvas {
    width: usize,
    height: usize,
    /// What every pixel started as, so "nothing painted here" is a comparison
    /// rather than a guess about which token the caller used.
    background: Color32,
    /// Premultiplied sRGBA, row-major.
    pixels: Vec<Color32>,
}

impl Canvas {
    fn new(width: usize, height: usize, background: Color32) -> Self {
        Self {
            width,
            height,
            background,
            pixels: vec![background; width * height],
        }
    }

    pub(crate) fn width(&self) -> usize {
        self.width
    }

    pub(crate) fn background(&self) -> Color32 {
        self.background
    }

    fn blend(&mut self, x: usize, y: usize, src: [f32; 4]) {
        let dst = &mut self.pixels[y * self.width + x];
        let inv = 1.0 - src[3] / 255.0;
        let mut channels = dst.to_array();
        for channel in 0..4 {
            channels[channel] = (src[channel] + f32::from(channels[channel]) * inv)
                .round()
                .clamp(0.0, 255.0) as u8;
        }
        *dst = Color32::from_rgba_premultiplied(channels[0], channels[1], channels[2], channels[3]);
    }

    /// The last row that anything painted on, so a surface shorter than the
    /// canvas is not reported as acres of empty space.
    pub(crate) fn content_height(&self) -> usize {
        (0..self.height)
            .rev()
            .find(|row| {
                self.pixels[row * self.width..(row + 1) * self.width]
                    .iter()
                    .any(|pixel| *pixel != self.background)
            })
            .map_or(0, |row| row + 1)
    }

    /// Every pixel whose centre lies inside `rect`, row-major.
    ///
    /// Clamped to the canvas, and empty when the rect falls outside it — so an
    /// assertion phrased over this iterator must also check that it yielded
    /// something, or a rect placed off-canvas passes it vacuously.
    pub(crate) fn pixels_in(&self, rect: Rect) -> impl Iterator<Item = Color32> + '_ {
        let columns = covered(rect.min.x, rect.max.x, self.width);
        let rows = covered(rect.min.y, rect.max.y, self.height);
        rows.flat_map(move |y| {
            columns
                .clone()
                .map(move |x| self.pixels[y * self.width + x])
        })
    }

    /// A PNG with stored (uncompressed) deflate blocks, cropped to `height`
    /// rows. The crate has no image dependency and this is a review artifact,
    /// so size does not matter.
    pub(crate) fn png(&self, height: usize) -> Vec<u8> {
        let mut raw = Vec::with_capacity(height * (self.width * 4 + 1));
        for y in 0..height {
            raw.push(0); // filter: none
            for x in 0..self.width {
                let [r, g, b, a] = self.pixels[y * self.width + x].to_array();
                // PNG wants straight alpha; `Color32` is premultiplied.
                let straight = |channel: u8| {
                    if a == 0 {
                        0
                    } else {
                        ((f32::from(channel) * 255.0 / f32::from(a)).round()).clamp(0.0, 255.0)
                            as u8
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
        header.extend_from_slice(&(self.width as u32).to_be_bytes());
        header.extend_from_slice(&(height as u32).to_be_bytes());
        header.extend_from_slice(&[8, 6, 0, 0, 0]);
        chunk(&mut out, b"IHDR", &header);
        chunk(&mut out, b"IDAT", &zlib);
        chunk(&mut out, b"IEND", &[]);
        out
    }
}

/// The pixel indices along one axis whose centres lie within `min..=max`,
/// clamped to `extent`.
fn covered(min: f32, max: f32, extent: usize) -> std::ops::Range<usize> {
    let first = ((min - 0.5).ceil().max(0.0) as usize).min(extent);
    let last = (max - 0.5).floor();
    let end = if last < 0.0 {
        0
    } else {
        (last as usize).saturating_add(1).min(extent)
    };
    // `max` rather than an assertion: a rect that misses the canvas yields no
    // pixels, which the doc comment above makes the caller's problem.
    first..end.max(first)
}

/// Render `pass` at `size` and rasterize the result.
///
/// `pass` is handed the root [`egui::Ui`] and the colour the canvas was cleared
/// to, so a panel that fills its own background fills it with the same one.
///
/// # Cost
///
/// Three egui passes run: fonts build on the first, sizing settles on the
/// second, and the third is the one rasterized. Each is a full pass over the
/// whole surface followed by a software fill of every triangle it tessellates
/// to, so a render is far from free. Render once and assert over cropped
/// regions with [`Canvas::pixels_in`] rather than rendering once per claim.
pub(crate) fn render(size: Vec2, pass: impl FnMut(&mut egui::Ui, Color32)) -> Canvas {
    render_at_pointer(size, None, pass)
}

/// Render `pass` with the pointer parked at `pointer`, so a hover state and
/// any tooltip it opens are part of what gets rasterized.
///
/// The tooltip delay and the still-pointer requirement are cleared: a headless
/// pass has no wall clock for either to elapse against, and a pointer that
/// never moves is as still as one gets.
pub(crate) fn render_with_pointer(
    size: Vec2,
    pointer: egui::Pos2,
    pass: impl FnMut(&mut egui::Ui, Color32),
) -> Canvas {
    render_at_pointer(size, Some(pointer), pass)
}

fn render_at_pointer(
    size: Vec2,
    pointer: Option<egui::Pos2>,
    mut pass: impl FnMut(&mut egui::Ui, Color32),
) -> Canvas {
    let ctx = egui::Context::default();
    super::Theme::default().apply(&ctx);
    if pointer.is_some() {
        ctx.all_styles_mut(|style| {
            style.interaction.tooltip_delay = 0.0;
            style.interaction.show_tooltips_only_when_still = false;
        });
    }
    let background = super::tokens::Tokens::get(&ctx).color.bg_app;

    let input = || egui::RawInput {
        screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, size)),
        events: pointer
            .map(|pointer| vec![egui::Event::PointerMoved(pointer)])
            .unwrap_or_default(),
        ..Default::default()
    };
    let mut run = || ctx.run_ui(input(), |ui| pass(ui, background));

    let _ = run();
    let _ = run();
    let output = run();

    let atlas = ctx.fonts(|fonts| fonts.image());
    let primitives = ctx.tessellate(output.shapes, 1.0);

    let mut canvas = Canvas::new(size.x as usize, size.y as usize, background);
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
    canvas
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
            // Geometry is either untextured or glyph coverage from the font
            // atlas; icons are vector strokes, which arrive as geometry.
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

/// The chain every render-based assertion in the crate rests on: a rect painted
/// at a position lands at that position, in the colour it was painted, and
/// nowhere else.
///
/// Without this, a break anywhere between the caller's closure and the pixels —
/// a lost pass, a clip rect, an off-by-one in the fill — reads as a layout
/// defect in whatever surface was being rendered.
#[test]
fn a_painted_rect_covers_its_own_pixels_and_nothing_else() {
    let painted = Rect::from_min_size(egui::pos2(24.0, 18.0), egui::vec2(40.0, 26.0));
    let ink = Color32::from_rgb(220, 40, 120);
    let canvas = render(egui::vec2(120.0, 80.0), |ui, background| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |ui| ui.painter().rect_filled(painted, 0.0, ink));
    });

    // Inset past the tessellator's one-pixel feathering, which blends the
    // outermost row of the rect with what is behind it.
    let region = painted.shrink(2.0);
    let inside: Vec<Color32> = canvas.pixels_in(region).collect();
    // The region's bounds are whole pixels, so it covers exactly its area.
    assert_eq!(
        inside.len(),
        (region.width() * region.height()) as usize,
        "pixels_in covered {} pixels of a {}x{} region",
        inside.len(),
        region.width(),
        region.height()
    );
    assert!(
        inside.iter().all(|pixel| *pixel != canvas.background()),
        "the painted rect left background pixels inside itself"
    );
    assert!(
        inside.iter().all(|pixel| *pixel == ink),
        "the painted rect is not the colour it was painted"
    );

    let below = Rect::from_min_max(
        egui::pos2(0.0, painted.max.y + 2.0),
        egui::pos2(120.0, 80.0),
    );
    let outside: Vec<Color32> = canvas.pixels_in(below).collect();
    assert!(!outside.is_empty(), "no pixels below the painted rect");
    assert!(
        outside.iter().all(|pixel| *pixel == canvas.background()),
        "something painted below the rect"
    );
}
