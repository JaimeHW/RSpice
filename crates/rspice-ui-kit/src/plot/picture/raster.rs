//! A picture's curves as pixels: one premultiplied colour for each device
//! pixel of the plot, for a picture that holds more curve than the plot has
//! pixels.
//!
//! A curve is stroked as the renderer strokes it: a solid core of the
//! stroke's width, feathered to nothing over the tessellator's feathering
//! width, or, thinner than that, the feathering's width faded in proportion.
//! Each curve covers a pixel once however often it passes through it, and
//! curves are laid over one another in order, as the renderer blends them.
//! Everything else a curve carries (dots, markers) is tessellated as the
//! renderer tessellates it and its triangles filled.

use egui::epaint::{ColorMode, Mesh, TessellationOptions, Tessellator};
use egui::{Color32, ColorImage, Pos2, Rect, Shape, Vec2, pos2};

/// The pixels of a picture's curves over one area of the screen.
pub(in crate::plot) struct Canvas {
    /// The device pixel of the canvas' first pixel.
    origin: [i32; 2],
    width: usize,
    height: usize,
    pixels_per_point: f32,
    /// Premultiplied colour of each pixel, row by row, as the renderer's
    /// eight-bit target holds it.
    pixels: Vec<[u8; 4]>,
    /// How much of each pixel the curve being stroked covers, valid where its
    /// stamp is the curve's.
    coverage: Vec<f32>,
    stamp: Vec<u32>,
    /// The pixels the curve being stroked covers, each once.
    touched: Vec<usize>,
    curve: u32,
    feathering: f32,
    tessellator: Tessellator,
    mesh: Mesh,
}

impl Canvas {
    /// An empty canvas over the device pixels `area` (in points) touches.
    pub(in crate::plot) fn new(
        area: Rect,
        pixels_per_point: f32,
        options: TessellationOptions,
    ) -> Self {
        let x0 = (area.min.x * pixels_per_point).floor() as i32;
        let y0 = (area.min.y * pixels_per_point).floor() as i32;
        let x1 = (area.max.x * pixels_per_point).ceil() as i32;
        let y1 = (area.max.y * pixels_per_point).ceil() as i32;
        let width = x1.saturating_sub(x0).max(0) as usize;
        let height = y1.saturating_sub(y0).max(0) as usize;
        let count = width * height;
        let feathering = if options.feathering {
            options.feathering_size_in_pixels.max(0.0)
        } else {
            0.0
        };
        Self {
            origin: [x0, y0],
            width,
            height,
            pixels_per_point,
            pixels: vec![[0; 4]; count],
            coverage: vec![0.0; count],
            stamp: vec![0; count],
            touched: Vec::new(),
            curve: 0,
            feathering,
            tessellator: Tessellator::new(pixels_per_point, options, [1, 1], Vec::new()),
            mesh: Mesh::default(),
        }
    }

    /// Where the canvas lies, in points: whole device pixels.
    pub(in crate::plot) fn rect(&self) -> Rect {
        let ppp = self.pixels_per_point;
        Rect::from_min_size(
            pos2(self.origin[0] as f32 / ppp, self.origin[1] as f32 / ppp),
            Vec2::new(self.width as f32 / ppp, self.height as f32 / ppp),
        )
    }

    /// The device pixels of the canvas, as the renderer takes an image.
    pub(in crate::plot) fn image(&self) -> ColorImage {
        let pixels = self
            .pixels
            .iter()
            .map(|&[r, g, b, a]| Color32::from_rgba_premultiplied(r, g, b, a))
            .collect();
        ColorImage::new([self.width, self.height], pixels)
    }

    /// Lay `top`, a canvas over the same area painted after this one, over
    /// it.
    pub(in crate::plot) fn cover_with(&mut self, top: &Self) {
        for (under, over) in self.pixels.iter_mut().zip(&top.pixels) {
            if over[3] == 0 {
                continue;
            }
            blend(under, over.map(|channel| f32::from(channel) / 255.0), 1.0);
        }
    }

    /// Paint `shape` over what the canvas holds.
    pub(in crate::plot) fn paint(&mut self, shape: &Shape) {
        match shape {
            Shape::Noop => {}
            Shape::Vec(shapes) => {
                for shape in shapes {
                    self.paint(shape);
                }
            }
            Shape::Path(path) if !path.closed && path.fill == Color32::TRANSPARENT => {
                match path.stroke.color {
                    ColorMode::Solid(color) => {
                        self.stroke(&path.points, path.stroke.width, color);
                    }
                    ColorMode::UV(_) => self.fill_tessellated(shape),
                }
            }
            Shape::LineSegment { points, stroke } => {
                self.stroke(points, stroke.width, stroke.color);
            }
            _ => self.fill_tessellated(shape),
        }
    }

    /// The canvas position of a point on the screen, in device pixels: pixel
    /// `(i, j)` spans `i..i + 1` by `j..j + 1`.
    fn place(&self, point: Pos2) -> Pos2 {
        pos2(
            point.x * self.pixels_per_point - self.origin[0] as f32,
            point.y * self.pixels_per_point - self.origin[1] as f32,
        )
    }

    /// Stroke a polyline `width` points wide in `color`.
    fn stroke(&mut self, points: &[Pos2], width: f32, color: Color32) {
        if color.a() == 0 || width <= 0.0 || points.is_empty() || self.pixels.is_empty() {
            return;
        }
        let profile = Profile::new(width * self.pixels_per_point, self.feathering);
        if profile.reach <= 0.0 {
            return;
        }
        self.curve = self.curve.wrapping_add(1);
        if self.curve == 0 {
            self.stamp.fill(0);
            self.curve = 1;
        }
        self.touched.clear();
        if let [only] = points {
            let at = self.place(*only);
            self.segment(at, at, &profile);
        }
        for pair in points.windows(2) {
            let (a, b) = (self.place(pair[0]), self.place(pair[1]));
            if a.is_finite() && b.is_finite() {
                self.segment(a, b, &profile);
            }
        }
        let color = premultiplied(color);
        let touched = std::mem::take(&mut self.touched);
        for &index in &touched {
            blend(&mut self.pixels[index], color, self.coverage[index]);
        }
        self.touched = touched;
    }

    /// Cover the pixels within reach of the segment `a`–`b` for the curve
    /// being stroked, each by the most any of its segments covers it.
    fn segment(&mut self, a: Pos2, b: Pos2, profile: &Profile) {
        let reach = profile.reach;
        let d = b - a;
        let length_sq = d.length_sq();
        let distance = |center: Pos2| {
            let t = if length_sq > 0.0 {
                ((center - a).dot(d) / length_sq).clamp(0.0, 1.0)
            } else {
                0.0
            };
            (center - (a + d * t)).length()
        };
        let (width, height) = (self.width as i64, self.height as i64);
        // Walk the segment's longer axis a pixel at a time, covering the
        // pixels across it within reach.
        let along_x = d.x.abs() >= d.y.abs();
        let (start, end) = if along_x {
            (a.x.min(b.x) - reach, a.x.max(b.x) + reach)
        } else {
            (a.y.min(b.y) - reach, a.y.max(b.y) + reach)
        };
        let limit = if along_x { width } else { height };
        let first = ((start - 0.5).floor() as i64).max(0);
        let last = ((end - 0.5).ceil() as i64).min(limit - 1);
        let (major, minor) = if along_x { (d.x, d.y) } else { (d.y, d.x) };
        let (major_origin, minor_origin) = if along_x { (a.x, a.y) } else { (a.y, a.x) };
        // How far across the segment reaches, measured along the minor axis.
        let across = if major.abs() > f32::EPSILON {
            reach * length_sq.sqrt() / major.abs()
        } else {
            reach
        };
        let minor_limit = if along_x { height } else { width };
        for step in first..=last {
            let center_major = step as f32 + 0.5;
            let t = if major.abs() > f32::EPSILON {
                ((center_major - major_origin) / major).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let middle = minor_origin + minor * t;
            let low = ((middle - across - 0.5).floor() as i64).max(0);
            let high = ((middle + across - 0.5).ceil() as i64).min(minor_limit - 1);
            for cross in low..=high {
                let (column, row) = if along_x {
                    (step, cross)
                } else {
                    (cross, step)
                };
                let center = pos2(column as f32 + 0.5, row as f32 + 0.5);
                let covered = profile.coverage(distance(center));
                if covered <= 0.0 {
                    continue;
                }
                let index = row as usize * self.width + column as usize;
                if self.stamp[index] == self.curve {
                    if covered > self.coverage[index] {
                        self.coverage[index] = covered;
                    }
                } else {
                    self.stamp[index] = self.curve;
                    self.coverage[index] = covered;
                    self.touched.push(index);
                }
            }
        }
    }

    /// Tessellate `shape` as the renderer does and fill its triangles.
    fn fill_tessellated(&mut self, shape: &Shape) {
        self.mesh.clear();
        self.tessellator
            .tessellate_shape(shape.clone(), &mut self.mesh);
        let mesh = std::mem::take(&mut self.mesh);
        for triangle in mesh.indices.chunks_exact(3) {
            let corner = |index: u32| {
                let vertex = &mesh.vertices[index as usize];
                (self.place(vertex.pos), premultiplied(vertex.color))
            };
            self.triangle(
                corner(triangle[0]),
                corner(triangle[1]),
                corner(triangle[2]),
            );
        }
        self.mesh = mesh;
    }

    /// Fill a triangle whose corners carry colours, blending each pixel whose
    /// centre lies inside it with the colour interpolated there.
    fn triangle(&mut self, a: (Pos2, [f32; 4]), b: (Pos2, [f32; 4]), c: (Pos2, [f32; 4])) {
        let edge =
            |p: Pos2, q: Pos2, r: Pos2| (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x);
        let area = edge(a.0, b.0, c.0);
        if area.abs() <= f32::EPSILON || !area.is_finite() {
            return;
        }
        let min_x = a.0.x.min(b.0.x).min(c.0.x);
        let max_x = a.0.x.max(b.0.x).max(c.0.x);
        let min_y = a.0.y.min(b.0.y).min(c.0.y);
        let max_y = a.0.y.max(b.0.y).max(c.0.y);
        let first_column = ((min_x - 0.5).ceil() as i64).max(0);
        let last_column = ((max_x - 0.5).floor() as i64).min(self.width as i64 - 1);
        let first_row = ((min_y - 0.5).ceil() as i64).max(0);
        let last_row = ((max_y - 0.5).floor() as i64).min(self.height as i64 - 1);
        for row in first_row..=last_row {
            for column in first_column..=last_column {
                let center = pos2(column as f32 + 0.5, row as f32 + 0.5);
                let wa = edge(b.0, c.0, center) / area;
                let wb = edge(c.0, a.0, center) / area;
                let wc = edge(a.0, b.0, center) / area;
                if wa < 0.0 || wb < 0.0 || wc < 0.0 {
                    continue;
                }
                let mut color = [0.0; 4];
                for (channel, value) in color.iter_mut().enumerate() {
                    *value = wa * a.1[channel] + wb * b.1[channel] + wc * c.1[channel];
                }
                let index = row as usize * self.width + column as usize;
                blend(&mut self.pixels[index], color, 1.0);
            }
        }
    }
}

/// The memory a canvas over `area` holds, in bytes.
pub(in crate::plot) fn canvas_bytes(area: Rect, pixels_per_point: f32) -> usize {
    let side =
        |length: f32| ((length * pixels_per_point).ceil().max(0.0) as usize).saturating_add(1);
    side(area.width())
        .saturating_mul(side(area.height()))
        .saturating_mul(size_of::<[u8; 4]>() + size_of::<f32>() + size_of::<u32>())
}

/// How much of a pixel a stroke covers at a distance from its middle.
struct Profile {
    /// Fully covered within this distance.
    core: f32,
    /// Covered not at all beyond this distance.
    reach: f32,
    /// The most a stroke thinner than the feathering covers anything.
    peak: f32,
}

impl Profile {
    /// The renderer's stroke `width` device pixels wide, feathered over
    /// `feathering` device pixels.
    fn new(width: f32, feathering: f32) -> Self {
        if feathering <= 0.0 {
            return Self {
                core: width * 0.5,
                reach: width * 0.5,
                peak: 1.0,
            };
        }
        if width <= feathering {
            // A thin line: the feathering's width, faded by how much
            // thinner the stroke is.
            return Self {
                core: 0.0,
                reach: feathering,
                peak: width / feathering,
            };
        }
        Self {
            core: (width - feathering) * 0.5,
            reach: (width + feathering) * 0.5,
            peak: 1.0,
        }
    }

    fn coverage(&self, distance: f32) -> f32 {
        if distance >= self.reach {
            return 0.0;
        }
        if distance <= self.core {
            return self.peak;
        }
        self.peak * (self.reach - distance) / (self.reach - self.core)
    }
}

/// A colour's premultiplied channels, from nothing to one.
fn premultiplied(color: Color32) -> [f32; 4] {
    let [r, g, b, a] = color.to_array();
    [r, g, b, a].map(|channel| f32::from(channel) / 255.0)
}

/// Lay `color`, covering `coverage` of the pixel, over it.
fn blend(pixel: &mut [u8; 4], color: [f32; 4], coverage: f32) {
    let keep = 1.0 - color[3] * coverage;
    for channel in 0..4 {
        let value = (color[channel] * coverage).mul_add(255.0, f32::from(pixel[channel]) * keep);
        pixel[channel] = value.round().clamp(0.0, 255.0) as u8;
    }
}
