//! A plot's curves, drawn once and kept while nothing they show changes.
//!
//! A frame that draws the curves, view, rect and looks of the picture the plot
//! drew last reuses that picture: it reads no envelope, maps no point and
//! strokes no curve. Hover, cursors and readouts draw over it. A picture holds
//! its curves as the shapes they were stroked into, or, when those would
//! outnumber the plot's own pixels, as the pixels.
//!
//! A picture costing more than a frame to draw is drawn off the frame. Until
//! it is ready the plot keeps the picture it has, carried along with the view
//! and marked as updating; a newer request supersedes an older one, whose
//! picture is never shown.
//!
//! The web build has no thread to draw on, so it draws every picture on the
//! frame, and the off-frame drawing is compiled there but never called.
#![cfg_attr(target_arch = "wasm32", allow(dead_code))]

mod raster;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use egui::emath::GuiRounding as _;
use egui::epaint::TessellationOptions;
use egui::{Color32, Pos2, Rect, Shape, Stroke, pos2};
use rspice_results::waveform::WaveformValues;

use super::decimate::{self, DecimationCache, EnvelopeKey, Kept, TraceView};
use super::render::{
    MarkerSlots, TraceMarkerShape, column_dashes, finite_runs, held_steps, run_is_a_point,
    runs_forward, trace_marker_positions, trace_marker_shapes,
};
use super::sample::SweepShape;
use super::scale::XScale;
use super::spec::{DrawMode, PlotSpec, StoredSamples, Trace};

/// What a frame can afford to draw itself: points stroked, a sample reduced
/// counting a sixteenth of one. About what a frame maps and strokes in a
/// sixtieth of a second; a picture of more is drawn off the frame.
const FRAME_WORK: usize = 400_000;

/// Where a plot's curves are drawn and through what view.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Geometry {
    pub plot_rect: Rect,
    pub pixels_per_point: f32,
    pub x: (f64, f64, XScale),
    /// The plot's own Y axis; a curve may read against another.
    pub y: (f64, f64, XScale),
    /// The columns and rows a view is reduced to.
    pub columns: usize,
    pub rows: usize,
    /// What markers are cut out of.
    pub background: Color32,
}

impl Geometry {
    fn view(&self, y: (f64, f64, XScale)) -> TraceView {
        TraceView {
            x0: self.x.0,
            x1: self.x.1,
            y0: y.0,
            y1: y.1,
            x_scale: self.x.2,
            y_scale: y.2,
            columns: self.columns,
            rows: self.rows,
        }
    }

    fn screen_x(&self, x: f64) -> f32 {
        self.plot_rect.left()
            + (self.x.2.normalize(x, self.x.0, self.x.1) as f32) * self.plot_rect.width()
    }

    fn screen_y(&self, y: f64, axis: (f64, f64, XScale)) -> f32 {
        self.plot_rect.bottom()
            - (axis.2.normalize(y, axis.0, axis.1) as f32) * self.plot_rect.height()
    }

    /// What the curves may stroke into: the plot area and the pixel around
    /// it.
    fn area(&self) -> Rect {
        self.plot_rect.expand(1.0)
    }

    /// The device pixels of the plot area.
    fn pixels(&self) -> usize {
        let area = self.area();
        let side = |length: f32| (length * self.pixels_per_point).ceil().max(0.0) as usize;
        side(area.width()).saturating_mul(side(area.height()))
    }

    fn hash_into(&self, hasher: &mut impl Hasher) {
        for value in [
            self.plot_rect.min.x,
            self.plot_rect.min.y,
            self.plot_rect.max.x,
            self.plot_rect.max.y,
            self.pixels_per_point,
        ] {
            value.to_bits().hash(hasher);
        }
        for (min, max, scale) in [self.x, self.y] {
            min.to_bits().hash(hasher);
            max.to_bits().hash(hasher);
            scale.hash(hasher);
        }
        (self.columns, self.rows, self.background.to_array()).hash(hasher);
    }

    /// How a point of a picture drawn in `self` lies in `to`: the same data
    /// at the same place, carried along by a pan or zoom. `None` when the
    /// scales differ, so no straight carry exists.
    fn carry(&self, to: &Self) -> Option<Carry> {
        if self.x.2 != to.x.2 || self.y.2 != to.y.2 {
            return None;
        }
        let (left, right) = (to.screen_x(self.x.0), to.screen_x(self.x.1));
        let (bottom, top) = (to.screen_y(self.y.0, to.y), to.screen_y(self.y.1, to.y));
        let from = self.plot_rect;
        let scale_x = (right - left) / from.width();
        let scale_y = (top - bottom) / (from.top() - from.bottom());
        let carry = Carry {
            scale: egui::vec2(scale_x, scale_y),
            offset: egui::vec2(
                left - scale_x * from.left(),
                bottom - scale_y * from.bottom(),
            ),
        };
        (carry.scale.is_finite() && carry.offset.is_finite()).then_some(carry)
    }
}

/// A straight carry of the screen: each axis scaled and shifted.
#[derive(Debug, Clone, Copy)]
struct Carry {
    scale: egui::Vec2,
    offset: egui::Vec2,
}

impl Carry {
    fn apply(self, point: Pos2) -> Pos2 {
        pos2(
            point.x.mul_add(self.scale.x, self.offset.x),
            point.y.mul_add(self.scale.y, self.offset.y),
        )
    }

    fn shape(self, shape: &mut Shape) {
        match shape {
            Shape::Vec(shapes) => {
                for shape in shapes {
                    self.shape(shape);
                }
            }
            Shape::Path(path) => {
                for point in &mut path.points {
                    *point = self.apply(*point);
                }
            }
            Shape::LineSegment { points, .. } => {
                for point in points {
                    *point = self.apply(*point);
                }
            }
            Shape::Circle(circle) => circle.center = self.apply(circle.center),
            Shape::Rect(rect) => {
                let center = self.apply(rect.rect.center());
                rect.rect = Rect::from_center_size(center, rect.rect.size());
            }
            _ => {}
        }
    }
}

/// Everything about how one curve is drawn that its picture keeps.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Look {
    pub color: Color32,
    /// The width the trace asks for, which sizes its dots.
    pub width: f32,
    /// The stroke's width: a hairline's in whole device pixels.
    pub stroke_width: f32,
    /// A hairline runs through pixel centres.
    pub hairline: bool,
    pub dash: Option<(f32, f32)>,
    pub marker: Option<TraceMarkerShape>,
    pub slots: MarkerSlots,
    pub draw: DrawMode,
    pub show_single_point: bool,
    /// The Y axis the curve reads against.
    pub y: (f64, f64, XScale),
}

impl Look {
    fn of(
        trace: &Trace<'_>,
        index: usize,
        color_safe: bool,
        slots: MarkerSlots,
        geometry: &Geometry,
    ) -> Self {
        let hairline = trace.width <= 1.0;
        let look = trace.look(index, color_safe);
        Self {
            color: trace.color,
            width: trace.width,
            stroke_width: if hairline {
                super::render::hairline(trace.width, geometry.pixels_per_point)
            } else {
                trace.width
            },
            hairline,
            dash: look.dash,
            marker: look.marker,
            slots,
            draw: trace.draw,
            show_single_point: trace.show_single_point,
            y: trace
                .y_axis
                .map_or(geometry.y, |axis| (axis.min, axis.max, axis.scale)),
        }
    }

    fn hash_into(&self, hasher: &mut impl Hasher) {
        self.color.to_array().hash(hasher);
        for value in [
            self.width,
            self.stroke_width,
            self.slots.origin,
            self.slots.pitch,
        ] {
            value.to_bits().hash(hasher);
        }
        self.dash
            .map(|(dash, gap)| (dash.to_bits(), gap.to_bits()))
            .hash(hasher);
        (
            self.marker,
            self.draw,
            self.show_single_point,
            self.hairline,
        )
            .hash(hasher);
        (self.y.0.to_bits(), self.y.1.to_bits(), self.y.2).hash(hasher);
    }
}

/// Where a curve drawn off the frame takes its envelope from.
pub(super) enum Source {
    /// An envelope the frame holds, and the key the cache holds it under
    /// when it does.
    Held {
        envelope: Arc<[[f64; 2]]>,
        key: Option<EnvelopeKey>,
    },
    /// Samples in memory, reduced off the frame.
    Memory {
        key: u64,
        x: WaveformValues,
        y: WaveformValues,
        parametric: bool,
        shape: Option<SweepShape>,
    },
    /// Samples a results store keeps, read off the frame.
    Stored {
        key: u64,
        x: WaveformValues,
        y: WaveformValues,
    },
}

/// An envelope the cache holds, with the key it holds it under.
type HeldEnvelope = (EnvelopeKey, Arc<[[f64; 2]]>);

/// One curve of a picture drawn off the frame.
pub(super) struct Curve {
    source: Source,
    look: Look,
}

/// A picture to draw off the frame.
struct Job {
    curves: Vec<Curve>,
    geometry: Geometry,
    /// Drawn as pixels rather than shapes.
    pixels: bool,
    options: TessellationOptions,
    /// The envelopes it may hand back for the frame to keep, in bytes.
    keep_budget: usize,
}

/// A picture drawn, before the frame takes it.
pub(super) struct Built {
    drawing: Drawing,
    /// Every envelope is the view's own, not a coarser one drawn while it is
    /// read.
    complete: bool,
    /// The envelopes it reads, for the frame's cache to keep.
    kept: Kept,
    curves: usize,
}

enum Drawing {
    Shapes(Vec<Shape>),
    /// The device pixels of `rect`, ready for the renderer.
    Image {
        rect: Rect,
        image: egui::ColorImage,
    },
}

impl Drawing {
    /// `canvas` as the renderer takes it.
    fn of(canvas: &raster::Canvas) -> Self {
        Self::Image {
            rect: canvas.rect(),
            image: canvas.image(),
        }
    }
}

/// A picture the plot shows.
struct Picture {
    key: u64,
    complete: bool,
    geometry: Geometry,
    content: Content,
}

enum Content {
    Shapes(Vec<Shape>),
    Pixels {
        texture: egui::TextureHandle,
        rect: Rect,
    },
}

/// Where a picture drawn off the frame is left for the frame to take.
type Arrival = Arc<Mutex<Option<Built>>>;

/// A picture being drawn off the frame. Dropped, it is called off.
struct Pending {
    key: u64,
    geometry: Geometry,
    cancel: Arc<AtomicBool>,
    arrival: Arrival,
}

impl Drop for Pending {
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}

/// One plot's pictures: the one it shows and the one being drawn.
#[derive(Default)]
struct Pictures {
    shown: Option<Picture>,
    pending: Option<Pending>,
    /// The frame that last drew the plot.
    drawn: u64,
}

impl Pictures {
    /// Ask for the picture `key` off the frame: the one pending is called
    /// off, and only what arrives for `key` is taken.
    fn request(&mut self, key: u64, geometry: Geometry) -> (Arc<AtomicBool>, Arrival) {
        let cancel = Arc::new(AtomicBool::new(false));
        let arrival = Arc::new(Mutex::new(None));
        self.pending = Some(Pending {
            key,
            geometry,
            cancel: Arc::clone(&cancel),
            arrival: Arc::clone(&arrival),
        });
        (cancel, arrival)
    }

    /// The picture asked for last, if it has arrived, with its key and
    /// geometry.
    fn arrived(&mut self) -> Option<(u64, Geometry, Built)> {
        let built = self
            .pending
            .as_ref()?
            .arrival
            .lock()
            .ok()
            .and_then(|mut slot| slot.take())?;
        let pending = self.pending.take()?;
        Some((pending.key, pending.geometry, built))
    }
}

/// Every plot's pictures, kept in the context.
type Store = Arc<Mutex<HashMap<egui::Id, Pictures>>>;

fn store(ctx: &egui::Context) -> Store {
    ctx.data_mut(|data| {
        data.get_temp_mut_or_default::<Store>(egui::Id::new("rspice plot pictures"))
            .clone()
    })
}

/// Whether a plot drawn in `ctx` has a picture being drawn off the frame
/// that no frame has yet taken.
#[cfg(any(test, feature = "test-support"))]
pub fn drawing_off_frame(ctx: &egui::Context) -> bool {
    store(ctx)
        .lock()
        .is_ok_and(|plots| plots.values().any(|pictures| pictures.pending.is_some()))
}

/// Draw `spec`'s curves into `painter`, from the picture the plot keeps when
/// nothing they show has changed. Says whether the plot shows an older
/// picture while a new one is drawn.
pub(super) fn draw(
    ctx: &egui::Context,
    id: egui::Id,
    spec: &PlotSpec<'_>,
    cache: &mut DecimationCache,
    painter: &egui::Painter,
    geometry: Geometry,
    color_safe: bool,
) -> bool {
    let slots = MarkerSlots::for_pane(spec, geometry.plot_rect, color_safe);
    let looks: Vec<Option<Look>> = spec
        .traces
        .iter()
        .enumerate()
        .map(|(index, trace)| {
            (!trace.is_empty()).then(|| {
                let slots = slots[index]
                    .unwrap_or_else(|| MarkerSlots::on_pane(spec, geometry.plot_rect, 0, 1));
                Look::of(trace, index, color_safe, slots, &geometry)
            })
        })
        .collect();
    // A curve with no identity cannot be known unchanged, so a plot of one
    // is drawn afresh each frame.
    if spec
        .traces
        .iter()
        .any(|trace| !trace.is_empty() && trace.cache_key.is_none())
    {
        let built = build_on_frame(spec, &looks, &[], cache, &geometry, None);
        if let Drawing::Shapes(shapes) = built.drawing {
            painter.extend(shapes);
        }
        return false;
    }
    let key = picture_key(spec, &looks, &geometry, cache.identity());
    let store = store(ctx);
    let Ok(mut plots) = store.lock() else {
        return false;
    };
    let frame = ctx.cumulative_frame_nr();
    // A plot no frame drew since the last lets its pictures go.
    plots.retain(|other, pictures| *other == id || pictures.drawn.saturating_add(1) >= frame);
    let pictures = plots.entry(id).or_default();
    pictures.drawn = frame;
    if let Some((arrived, drawn_in, built)) = pictures.arrived() {
        pictures.shown = Some(take(ctx, arrived, drawn_in, built, cache));
    }
    if let Some(shown) = &pictures.shown
        && shown.key == key
        && shown.complete
    {
        paint(shown, painter, &geometry);
        return false;
    }
    if pictures
        .pending
        .as_ref()
        .is_some_and(|pending| pending.key == key)
    {
        if let Some(shown) = &pictures.shown {
            paint(shown, painter, &geometry);
        }
        return true;
    }
    // A picture to draw: what the cache holds of it, and what drawing the
    // rest costs.
    let mut work = 0usize;
    let mut points = 0usize;
    let held: Vec<Option<HeldEnvelope>> = spec
        .traces
        .iter()
        .zip(&looks)
        .map(|(trace, look)| {
            let (Some(look), Some(trace_key)) = (look, trace.cache_key) else {
                return None;
            };
            let view = geometry.view(look.y);
            let envelope_key = match trace.stored {
                Some(_) => EnvelopeKey::stored(trace_key, view),
                None => EnvelopeKey::series(trace_key, view, trace.parametric, trace.shape),
            };
            let held = cache.held(envelope_key);
            let (cost, drawn) = match (&held, trace.stored) {
                (Some(envelope), _) => (envelope.len(), envelope.len()),
                (None, Some(_)) => (2 * geometry.columns, 2 * geometry.columns),
                (None, None) => {
                    let samples = trace.x.len().min(trace.y.len());
                    let drawn = samples.min(2 * geometry.columns);
                    (drawn + samples / 16, drawn)
                }
            };
            work = work.saturating_add(cost);
            points = points.saturating_add(drawn);
            held.map(|envelope| (envelope_key, envelope))
        })
        .collect();
    let off_frame = cfg!(not(target_arch = "wasm32")) && work > FRAME_WORK;
    if !off_frame {
        pictures.pending = None;
        let built = build_on_frame(
            spec,
            &looks,
            &held,
            cache,
            &geometry,
            Some(ctx.tessellation_options(|options| *options)),
        );
        let picture = take(ctx, key, geometry, built, cache);
        paint(&picture, painter, &geometry);
        pictures.shown = Some(picture);
        return false;
    }
    let curves: Vec<Curve> = spec
        .traces
        .iter()
        .zip(&looks)
        .zip(held)
        .filter_map(|((trace, look), held)| {
            let look = (*look)?;
            Some(Curve {
                source: source_of(trace, held, cache, &geometry, &look),
                look,
            })
        })
        .collect();
    let job = Job {
        curves,
        geometry,
        pixels: points > geometry.pixels(),
        options: ctx.tessellation_options(|options| *options),
        keep_budget: cache.capacity(),
    };
    let (cancel, arrival) = pictures.request(key, geometry);
    if !spawn(ctx.clone(), job, cancel, arrival) {
        // Without a thread to draw on, the next frame asks again.
        pictures.pending = None;
    }
    if let Some(shown) = &pictures.shown {
        paint(shown, painter, &geometry);
    }
    true
}

/// What identifies a picture: the cache's samples, where and through what
/// view it is drawn, and each curve's identity and look.
fn picture_key(
    spec: &PlotSpec<'_>,
    looks: &[Option<Look>],
    geometry: &Geometry,
    identity: u64,
) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    identity.hash(&mut hasher);
    geometry.hash_into(&mut hasher);
    for (trace, look) in spec.traces.iter().zip(looks) {
        let Some(look) = look else {
            0u8.hash(&mut hasher);
            continue;
        };
        (
            trace.cache_key,
            trace.stored.is_some(),
            trace.parametric,
            trace
                .shape
                .map(|shape| std::mem::discriminant(&shape.class())),
        )
            .hash(&mut hasher);
        look.hash_into(&mut hasher);
    }
    hasher.finish()
}

/// Where a curve drawn off the frame reads its envelope: the one the frame
/// holds, the shared columns it reduces, the store it reads, or, for
/// borrowed samples nobody shares, the envelope the frame reduces now.
fn source_of(
    trace: &Trace<'_>,
    held: Option<HeldEnvelope>,
    cache: &mut DecimationCache,
    geometry: &Geometry,
    look: &Look,
) -> Source {
    if let Some((key, envelope)) = held {
        return Source::Held {
            envelope,
            key: Some(key),
        };
    }
    let key = trace.cache_key.unwrap_or_default();
    if let Some(stored) = trace.stored {
        return Source::Stored {
            key,
            x: stored.x.clone(),
            y: stored.y.clone(),
        };
    }
    // Columns shared from memory are reduced off the frame; an axis read
    // from a store is borrowed for this frame only.
    if let Some(shared) = trace.shared
        && shared.x.in_memory().is_some()
        && shared.y.in_memory().is_some()
    {
        return Source::Memory {
            key,
            x: shared.x.clone(),
            y: shared.y.clone(),
            parametric: trace.parametric,
            shape: trace.shape.cloned(),
        };
    }
    Source::Held {
        envelope: cache.series(
            key,
            trace.x,
            trace.y,
            geometry.view(look.y),
            trace.parametric,
            trace.shape,
        ),
        key: None,
    }
}

/// Draw the picture now, reducing through the frame's cache what it does
/// not hold. Without tessellation options it is drawn as shapes.
fn build_on_frame(
    spec: &PlotSpec<'_>,
    looks: &[Option<Look>],
    held: &[Option<HeldEnvelope>],
    cache: &mut DecimationCache,
    geometry: &Geometry,
    options: Option<TessellationOptions>,
) -> Built {
    let mut shapes = Vec::new();
    let mut complete = true;
    let mut curves = 0;
    let mut points = 0usize;
    for (index, (trace, look)) in spec.traces.iter().zip(looks).enumerate() {
        let Some(look) = look else {
            continue;
        };
        let view = geometry.view(look.y);
        let held = held.get(index).cloned().flatten();
        let envelope = match (held.map(|(_, envelope)| envelope), trace.stored) {
            (Some(envelope), _) => Some(envelope),
            (None, Some(stored)) => {
                let (envelope, done) = cache.stored_series_read(trace.cache_key, stored, view);
                complete &= done;
                Some(envelope)
            }
            (None, None) => trace.cache_key.map(|key| {
                cache.series(key, trace.x, trace.y, view, trace.parametric, trace.shape)
            }),
        };
        let projected = match envelope {
            Some(envelope) => project(&envelope, geometry, look),
            None => {
                let raw: Vec<[f64; 2]> =
                    trace.x.iter().zip(trace.y).map(|(&x, &y)| [x, y]).collect();
                project(&raw, geometry, look)
            }
        };
        points = points.saturating_add(projected.len());
        curve_shapes(projected, look, geometry, &mut shapes);
        curves += 1;
    }
    cache.stroked(curves);
    let drawing = match options {
        Some(options) if points > geometry.pixels() => {
            let mut canvas =
                raster::Canvas::new(geometry.area(), geometry.pixels_per_point, options);
            for shape in &shapes {
                canvas.paint(shape);
            }
            Drawing::of(&canvas)
        }
        _ => Drawing::Shapes(shapes),
    };
    Built {
        drawing,
        complete,
        kept: Kept::default(),
        curves: 0,
    }
}

/// Turn a drawn picture into one the plot shows: its pixels handed to the
/// renderer, its envelopes kept whole and its curves counted. Nothing here
/// is done once per curve.
fn take(
    ctx: &egui::Context,
    key: u64,
    geometry: Geometry,
    built: Built,
    cache: &mut DecimationCache,
) -> Picture {
    cache.keep_all(built.kept);
    cache.stroked(built.curves);
    let content = match built.drawing {
        Drawing::Shapes(shapes) => Content::Shapes(shapes),
        Drawing::Image { rect, image } => Content::Pixels {
            rect,
            texture: ctx.load_texture("plot curves", image, egui::TextureOptions::NEAREST),
        },
    };
    Picture {
        key,
        complete: built.complete,
        geometry,
        content,
    }
}

/// Paint `picture` for a plot drawn in `geometry`: where it was drawn, or
/// carried along to the plot's view when it was drawn for another.
fn paint(picture: &Picture, painter: &egui::Painter, geometry: &Geometry) {
    let carry = (picture.geometry != *geometry)
        .then(|| picture.geometry.carry(geometry))
        .flatten();
    match &picture.content {
        Content::Shapes(shapes) => match carry {
            None => painter.extend(shapes.iter().cloned()),
            Some(carry) => painter.extend(shapes.iter().map(|shape| {
                let mut shape = shape.clone();
                carry.shape(&mut shape);
                shape
            })),
        },
        Content::Pixels { texture, rect } => {
            let rect = carry.map_or(*rect, |carry| {
                Rect::from_two_pos(carry.apply(rect.min), carry.apply(rect.max))
            });
            painter.image(
                texture.id(),
                rect,
                Rect::from_min_max(Pos2::ZERO, pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        }
    }
}

/// Start drawing `job` off the frame; what it draws arrives in `arrival`,
/// unless `cancel` calls it off first.
#[cfg(not(target_arch = "wasm32"))]
fn spawn(ctx: egui::Context, job: Job, cancel: Arc<AtomicBool>, arrival: Arrival) -> bool {
    std::thread::Builder::new()
        .name("plot curves".to_owned())
        .spawn(move || {
            if let Some(built) = build_off_frame(&job, &cancel)
                && !cancel.load(Ordering::Relaxed)
                && let Ok(mut slot) = arrival.lock()
            {
                *slot = Some(built);
                drop(slot);
                ctx.request_repaint();
            }
        })
        .is_ok()
}

#[cfg(target_arch = "wasm32")]
fn spawn(_ctx: egui::Context, _job: Job, _cancel: Arc<AtomicBool>, _arrival: Arrival) -> bool {
    false
}

/// Draw `job`, its curves shared among the machine's threads in runs, each
/// thread drawing its run whole, and the runs laid over one another in
/// order. `None` once called off.
fn build_off_frame(job: &Job, cancel: &AtomicBool) -> Option<Built> {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
    // Every thread drawing pixels holds a canvas of its own, and the
    // canvases of one picture stay within the display cache's budget.
    let canvases = if job.pixels {
        (job.keep_budget
            / raster::canvas_bytes(job.geometry.area(), job.geometry.pixels_per_point).max(1))
        .max(1)
    } else {
        usize::MAX
    };
    let threads = cores
        .saturating_sub(1)
        .max(1)
        .min(canvases)
        .min(job.curves.len().max(1));
    let run = job.curves.len().div_ceil(threads).max(1);
    let kept_bytes = AtomicUsize::new(0);
    let parts: Vec<Option<Run>> = std::thread::scope(|scope| {
        let handles: Vec<_> = job
            .curves
            .chunks(run)
            .map(|curves| scope.spawn(|| draw_run(curves, job, cancel, &kept_bytes)))
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().ok().flatten())
            .collect()
    });
    let mut parts = parts.into_iter();
    let mut built = parts.next().flatten()?;
    for part in parts {
        let part = part?;
        built.complete &= part.complete;
        built.kept.extend(part.kept);
        built.curves += part.curves;
        match (&mut built.drawing, part.drawing) {
            (Drawn::Shapes(shapes), Drawn::Shapes(more)) => shapes.extend(more),
            (Drawn::Pixels(canvas), Drawn::Pixels(top)) => canvas.cover_with(&top),
            _ => return None,
        }
    }
    Some(Built {
        drawing: match built.drawing {
            Drawn::Shapes(shapes) => Drawing::Shapes(shapes),
            Drawn::Pixels(canvas) => Drawing::of(&canvas),
        },
        complete: built.complete,
        kept: built.kept,
        curves: built.curves,
    })
}

/// One run of a job's curves, drawn.
struct Run {
    drawing: Drawn,
    complete: bool,
    kept: Kept,
    curves: usize,
}

enum Drawn {
    Shapes(Vec<Shape>),
    Pixels(Box<raster::Canvas>),
}

/// Draw one run of a job's curves.
fn draw_run(
    curves: &[Curve],
    job: &Job,
    cancel: &AtomicBool,
    kept_bytes: &AtomicUsize,
) -> Option<Run> {
    let geometry = &job.geometry;
    let mut canvas = job
        .pixels
        .then(|| raster::Canvas::new(geometry.area(), geometry.pixels_per_point, job.options));
    let mut shapes = Vec::new();
    let mut kept = Kept::default();
    let mut complete = true;
    for curve in curves {
        if cancel.load(Ordering::Relaxed) {
            return None;
        }
        let view = geometry.view(curve.look.y);
        let (envelope, done, envelope_key) = match &curve.source {
            Source::Held { envelope, key } => (Arc::clone(envelope), true, *key),
            Source::Memory {
                key,
                x,
                y,
                parametric,
                shape,
            } => match (x.in_memory(), y.in_memory()) {
                (Some(xs), Some(ys)) => (
                    decimate::reduce(xs, ys, view, *parametric, shape.as_ref()),
                    true,
                    Some(EnvelopeKey::series(*key, view, *parametric, shape.as_ref())),
                ),
                _ => (Arc::from([]), true, None),
            },
            Source::Stored { key, x, y } => {
                let (envelope, done) =
                    decimate::stored_envelope_waiting(StoredSamples { x, y }, view);
                (
                    envelope,
                    done,
                    done.then(|| EnvelopeKey::stored(*key, view)),
                )
            }
        };
        complete &= done;
        if let Some(envelope_key) = envelope_key {
            // The frame's cache keeps what fits its budget, the first
            // envelopes drawn; the rest are not held on to.
            let bytes = envelope.len() * size_of::<[f64; 2]>();
            if kept_bytes.fetch_add(bytes, Ordering::Relaxed) + bytes <= job.keep_budget {
                kept.keep(envelope_key, Arc::clone(&envelope));
            }
        }
        let start = shapes.len();
        curve_shapes(
            project(&envelope, geometry, &curve.look),
            &curve.look,
            geometry,
            &mut shapes,
        );
        if let Some(canvas) = &mut canvas {
            for shape in shapes.drain(start..) {
                canvas.paint(&shape);
            }
        }
    }
    Some(Run {
        drawing: match canvas {
            Some(canvas) => Drawn::Pixels(Box::new(canvas)),
            None => Drawn::Shapes(shapes),
        },
        complete,
        kept,
        curves: curves.len(),
    })
}

/// An envelope's points on the screen; a hairline's through pixel centres,
/// so its flat stretches stay sharp.
fn project(envelope: &[[f64; 2]], geometry: &Geometry, look: &Look) -> Vec<Pos2> {
    let ppp = geometry.pixels_per_point;
    envelope
        .iter()
        .map(|point| {
            let at = pos2(
                geometry.screen_x(point[0]),
                geometry.screen_y(point[1], look.y),
            );
            if look.hairline {
                pos2(
                    at.x.round_to_pixel_center(ppp),
                    at.y.round_to_pixel_center(ppp),
                )
            } else {
                at
            }
        })
        .collect()
}

/// The shapes one curve is drawn as: its strokes, dots and markers.
fn curve_shapes(points: Vec<Pos2>, look: &Look, geometry: &Geometry, out: &mut Vec<Shape>) {
    let stroke = Stroke::new(look.stroke_width, look.color);
    let marker = look.marker.or_else(|| {
        (look.show_single_point && points.len() == 1).then_some(TraceMarkerShape::Circle)
    });
    let plot_rect = geometry.plot_rect;
    match look.draw {
        DrawMode::Line | DrawMode::Steps => {}
        DrawMode::Points => {
            let radius = (look.width * 0.9).max(1.5);
            out.extend(
                points
                    .iter()
                    .filter(|point| point.is_finite())
                    .map(|point| Shape::circle_filled(*point, radius, look.color)),
            );
            return;
        }
        DrawMode::Stems => {
            let base = geometry.screen_y(0.0, look.y);
            let base = if base.is_finite() {
                base.clamp(plot_rect.top(), plot_rect.bottom())
            } else {
                plot_rect.bottom()
            };
            for point in points.iter().filter(|point| point.is_finite()) {
                out.push(Shape::line_segment([pos2(point.x, base), *point], stroke));
                out.push(Shape::circle_filled(
                    *point,
                    (look.width * 0.9).max(1.5),
                    look.color,
                ));
            }
            return;
        }
    }
    let points = if look.draw == DrawMode::Steps {
        held_steps(&points)
    } else {
        points
    };
    let dense = points.len() >= geometry.columns;
    for run in finite_runs(&points) {
        // A sample with a hole either side, or a run that folds into one
        // pixel, has no length to stroke; it is still data, so it draws as a
        // dot the width of the curve. A lone sample that carries a marker
        // already has its mark.
        if run_is_a_point(run) {
            if !(points.len() == 1 && marker.is_some()) {
                out.push(Shape::circle_filled(
                    run[0],
                    (look.width * 0.5).max(1.25),
                    look.color,
                ));
            }
            continue;
        }
        match look.dash {
            // A dense envelope zig-zags through every column, so dashing it
            // along its length would cut it into thousands of slivers reading
            // as noise. It is dashed across: drawn over stretches of X and
            // left out between them, which keeps the dash a curve is told
            // apart by.
            Some((dash, gap)) if dense && runs_forward(run) => {
                for stretch in column_dashes(run, dash, gap) {
                    out.push(Shape::line(stretch.to_vec(), stroke));
                }
            }
            Some((dash, gap)) if !dense => out.extend(Shape::dashed_line(run, stroke, dash, gap)),
            _ => out.push(Shape::line(run.to_vec(), stroke)),
        }
    }
    let Some(marker) = marker else {
        return;
    };
    if points.len() == 1 {
        let source_point = points[0];
        if source_point.x.is_finite()
            && source_point.y.is_finite()
            && plot_rect.contains(source_point)
        {
            let marker_bounds = plot_rect.shrink(4.0);
            let point = pos2(
                source_point
                    .x
                    .clamp(marker_bounds.left(), marker_bounds.right()),
                source_point
                    .y
                    .clamp(marker_bounds.top(), marker_bounds.bottom()),
            );
            trace_marker_shapes(point, marker, look.color, geometry.background, out);
        }
    } else {
        for point in trace_marker_positions(&points, plot_rect, look.slots) {
            trace_marker_shapes(point, marker, look.color, geometry.background, out);
        }
    }
}
