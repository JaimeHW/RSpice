//! The plot painter — turns a [`PlotSpec`] into egui shapes.

use egui::{
    Align2, Color32, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, WidgetInfo, WidgetType, pos2,
    vec2,
};

use crate::theme::{self, FontWeight};
use crate::tokens::Tokens;

use super::cursor::CursorPair;
use super::decimate::DecimationCache;
use super::scale::XScale;
use super::spec::{Axis, MarkerShape, PlotSpec, Trace};
use crate::palette::TRACE_HUES;

/// Interaction contract selected by the owning result surface.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum InteractionMode {
    /// Standard Results behavior: pan, box zoom, wheel zoom, and fit.
    #[default]
    All,
    /// Preserve pointer selection/readout without changing the view.
    Select,
    /// Primary drag pans; wheel zoom remains available.
    Pan,
    /// Primary drag draws a zoom box; wheel zoom remains available.
    Zoom,
}

const INTERACTION_MODE_ID: &str = "rspice.plot.interaction-mode";

/// Set the plot interaction mode for subsequently rendered plots this frame.
pub fn set_interaction_mode(ctx: &egui::Context, mode: InteractionMode) {
    ctx.data_mut(|data| data.insert_temp(egui::Id::new(INTERACTION_MODE_ID), mode));
}

fn interaction_mode(ctx: &egui::Context) -> InteractionMode {
    ctx.data(|data| {
        data.get_temp(egui::Id::new(INTERACTION_MODE_ID))
            .unwrap_or_default()
    })
}

/// A view-range change requested by a navigation gesture this frame.
/// The caller owns the view state; the engine only reports what the
/// gesture means in data space against the ranges it was handed.
#[derive(Debug, Clone, Copy, Default)]
pub struct ViewChange {
    /// New X range.
    pub x: Option<(f64, f64)>,
    /// New Y range.
    pub y: Option<(f64, f64)>,
    /// Double-click: restore the automatic (fit-to-data) view.
    pub reset: bool,
}

impl ViewChange {
    /// Whether the gesture produced any change this frame.
    pub fn any(&self) -> bool {
        self.reset || self.x.is_some() || self.y.is_some()
    }
}

/// What the plot reported back for this frame.
pub struct PlotResponse {
    /// The interaction response over the whole well.
    pub response: Response,
    /// Data-space X of a primary click inside the plot area, this frame.
    pub clicked_x: Option<f64>,
    /// A horizontal cursor a click or drag placed or moved: its slot (0 is
    /// H1, 1 is H2) and its new left-axis value.
    pub horizontal_cursor: Option<(usize, f64)>,
    /// The inner plot rectangle (inside margins), for overlays.
    pub plot_rect: Rect,
    /// Zoom/pan/fit gesture result: wheel zooms X about the cursor
    /// (Ctrl+wheel zooms Y), drag pans, Shift- or right-drag draws a
    /// zoom box, double-click fits.
    pub view: ViewChange,
    /// The X and Y intervals this frame actually drew, whether they came
    /// from a pinned viewport or from fitting the data. A surface offering
    /// explicit axis limits has to seed itself from what the reader can
    /// see, and only the renderer knows that for certain.
    pub axes: ((f64, f64), (f64, f64)),
    /// Where the X axis' own label was painted, so a surface can let the
    /// reader click it to change what the axis is drawn against.
    pub x_label_rect: Option<Rect>,
    /// Where the Y axis' caption was drawn, for a caller that explains it.
    pub y_caption: Option<Rect>,
}

/// A row of the hover readout: (label, value).
pub type ReadoutRow = (String, String);

/// The inner plot rectangle `show` will use for the current available
/// space — for callers that pre-bake size-dependent resources (textures)
/// before handing the spec over.
pub fn plot_rect(ui: &Ui, spec: &PlotSpec<'_>) -> Rect {
    inner_rect(ui.available_rect_before_wrap(), spec)
}

/// Margins around the inner plot area (tick labels, axis units).
const MARGIN_TOP: f32 = 12.0;
/// The top margin of a plot whose Y unit has a row of its own: the caption's
/// row, centred on [`Y_CAPTION_CENTER`], then room for half of a tick label
/// standing on the plot's top edge, so the two never touch.
const MARGIN_TOP_CAPTIONED: f32 = 24.0;
/// Where the Y caption's middle sits below the well's top.
const Y_CAPTION_CENTER: f32 = 8.0;
/// How far a drag must travel along an axis before it pans that axis.
const PAN_DEAD_ZONE: f32 = 6.0;
/// The size of the tick labels' font.
const Y_TICK_FONT_SIZE: f32 = 10.0;
/// The least gap between two Y tick labels that are both drawn.
const Y_TICK_LABEL_GAP: f32 = 2.0;
const MARGIN_BOTTOM: f32 = 26.0;
const MARGIN_RIGHT_PLAIN: f32 = 16.0;

/// A point marker a curve carries besides its colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraceMarkerShape {
    Circle,
    Square,
    Diamond,
    Triangle,
    Cross,
    Plus,
    TriangleDown,
    Asterisk,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TraceRedundancy {
    dash: Option<(f32, f32)>,
    marker: TraceMarkerShape,
}

/// One (dash, marker) pair per palette hue, so a colour-safe curve is told
/// apart from the seven others without its colour.
const COLOR_SAFE_STYLES: [TraceRedundancy; TRACE_HUES] = [
    TraceRedundancy {
        dash: None,
        marker: TraceMarkerShape::Circle,
    },
    TraceRedundancy {
        dash: Some((7.0, 4.0)),
        marker: TraceMarkerShape::Square,
    },
    TraceRedundancy {
        dash: Some((2.0, 4.0)),
        marker: TraceMarkerShape::Diamond,
    },
    TraceRedundancy {
        dash: Some((10.0, 4.0)),
        marker: TraceMarkerShape::Triangle,
    },
    TraceRedundancy {
        dash: Some((5.0, 2.0)),
        marker: TraceMarkerShape::Cross,
    },
    TraceRedundancy {
        dash: Some((12.0, 3.0)),
        marker: TraceMarkerShape::Plus,
    },
    TraceRedundancy {
        dash: Some((3.0, 3.0)),
        marker: TraceMarkerShape::TriangleDown,
    },
    TraceRedundancy {
        dash: Some((9.0, 2.0)),
        marker: TraceMarkerShape::Asterisk,
    },
];

/// The dash each further pass through the palette's hues draws in: solid
/// first, then dashed, dotted and long-dashed, so the ninth curve is not the
/// first one again.
const PASS_DASHES: [Option<(f32, f32)>; 4] =
    [None, Some((6.0, 4.0)), Some((2.0, 3.0)), Some((12.0, 4.0))];

/// The gap each later pass through the hues adds to its hue's first-pass
/// dash, in points. The first pass's gaps are 2 to 4 points, so every pass
/// draws its dashes from a set no other pass draws.
const COLOR_SAFE_PASS_GAP: f32 = 4.0;

/// The dash a solid first-pass curve is broken into on a later pass.
const COLOR_SAFE_SOLID_AS_DASH: (f32, f32) = (12.0, 2.0);

/// The dash a colour-safe curve on pass `pass` (the second is 1) through the
/// hues draws in: its hue's own first-pass dash with a wider gap, so it is
/// always dashed, keeps its hue's marker, and is told from every first-pass
/// curve — the one in its own hue by the gap, the others by the marker too.
///
/// Borrowing another hue's dash instead, as a later pass once did, handed
/// the eighth hue's second pass the first hue's solid stroke.
fn color_safe_pass_dash(hue: usize, pass: usize) -> (f32, f32) {
    let (dash, gap) = COLOR_SAFE_STYLES[hue % TRACE_HUES]
        .dash
        .unwrap_or(COLOR_SAFE_SOLID_AS_DASH);
    (dash, gap + COLOR_SAFE_PASS_GAP * pass as f32)
}

fn color_safe_trace_style(index: usize, semantic_dashed: bool) -> TraceRedundancy {
    let mut style = COLOR_SAFE_STYLES[index % TRACE_HUES];
    if semantic_dashed && style.dash.is_none() {
        style.dash = Some((5.0, 4.0));
    }
    style
}

/// How a curve is drawn besides its colour.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TraceLook {
    /// Dash and gap lengths, in points; `None` is a solid stroke.
    pub dash: Option<(f32, f32)>,
    /// The marker the curve carries, if any.
    pub marker: Option<TraceMarkerShape>,
}

impl Trace<'_> {
    /// How this trace is drawn besides its colour, as the `ordinal`-th trace
    /// of its plot, with color-safe traces on or off.
    ///
    /// A trace placed in the palette's cycle takes its look from that slot:
    /// its marker (color-safe) and, once the hues have all been used, a dash
    /// of its own pass. A legend draws its swatch from the same answer, so
    /// the swatch is the curve.
    pub fn look(&self, ordinal: usize, color_safe: bool) -> TraceLook {
        let slot = self.palette_slot.unwrap_or(ordinal);
        let pass = self.palette_slot.map_or(0, |slot| slot / TRACE_HUES);
        let redundancy = color_safe.then(|| {
            let mut style = color_safe_trace_style(slot, self.dashed);
            if pass > 0 {
                style.dash = Some(color_safe_pass_dash(slot, pass));
            }
            style
        });
        let dash = self
            .dash_style
            .and_then(|ordinal| color_safe_trace_style(ordinal, false).dash)
            .or_else(|| redundancy.and_then(|style| style.dash))
            .or_else(|| self.dashed.then_some((5.0, 4.0)))
            .or_else(|| {
                if color_safe {
                    None
                } else {
                    PASS_DASHES[pass % PASS_DASHES.len()]
                }
            });
        let marker = self
            .marker_style
            .map(|ordinal| color_safe_trace_style(ordinal, false).marker)
            .or_else(|| redundancy.map(|style| style.marker));
        TraceLook { dash, marker }
    }
}

/// A legend swatch that is the curve it stands for: a short stroke in the
/// curve's dash, and its marker at the middle.
pub fn paint_trace_swatch(
    painter: &egui::Painter,
    rect: Rect,
    color: Color32,
    look: TraceLook,
    background: Color32,
) {
    let y = rect.center().y;
    let ends = [pos2(rect.left() + 1.0, y), pos2(rect.right() - 1.0, y)];
    let stroke = Stroke::new(2.0, color);
    match look.dash {
        // A swatch is a handful of points long: a long dash shrunk to fit
        // still reads as a dash, not as a solid line with a gap at its end.
        Some((dash, gap)) => {
            let scale = (rect.width() / (2.0 * (dash + gap))).min(1.0);
            painter.extend(Shape::dashed_line(
                &ends,
                stroke,
                (dash * scale).max(1.5),
                (gap * scale).max(1.5),
            ));
        }
        None => {
            painter.line_segment(ends, stroke);
        }
    }
    if let Some(marker) = look.marker {
        paint_trace_marker(painter, rect.center(), marker, color, background);
    }
}

/// How much of its colour a measured band's fill carries. Low enough that the
/// grid and anything ruled across the band stay readable through it, dark
/// enough that it reads as one filled range rather than as two unrelated
/// curves.
const BAND_FILL_ALPHA: f32 = 0.28;

/// A filled band between a low and a high boundary, with both boundaries
/// outlined.
///
/// `columns` carries one `(high, low)` pair per position, already projected,
/// and the band is drawn in `color`: filled at [`BAND_FILL_ALPHA`] and outlined
/// at full strength.
///
/// The fill is one mesh rather than a filled rectangle per column: two
/// translucent rectangles sharing an edge blend twice along it, and a fill that
/// darkens every few pixels reads as a grid nobody drew. The two boundaries are
/// separate polylines for the same reason — a closed path around the band would
/// trace the two ends twice and leave a hairline at each.
///
/// It lives here because two surfaces draw a band of measured extremes — the
/// stimulus instrument's proof band and the properties dialog's preview card —
/// and a band that read as a fill on one and as a stack of quads on the other
/// would be two answers to what a range looks like in this product.
pub fn paint_min_max_band(
    painter: &egui::Painter,
    columns: &[(Pos2, Pos2)],
    color: Color32,
    width: f32,
) {
    let fill = color.gamma_multiply(BAND_FILL_ALPHA);
    let stroke = Stroke::new(width, color);
    let mut mesh = egui::Mesh::default();
    mesh.reserve_vertices(columns.len() * 2);
    mesh.reserve_triangles(columns.len().saturating_sub(1) * 2);
    let mut high = Vec::with_capacity(columns.len());
    let mut low = Vec::with_capacity(columns.len());
    for (ordinal, (top, bottom)) in columns.iter().enumerate() {
        let index = (ordinal * 2) as u32;
        mesh.colored_vertex(*top, fill);
        mesh.colored_vertex(*bottom, fill);
        if index >= 2 {
            mesh.add_triangle(index - 2, index - 1, index);
            mesh.add_triangle(index - 1, index, index + 1);
        }
        high.push(*top);
        low.push(*bottom);
    }
    painter.add(Shape::mesh(mesh));
    painter.add(Shape::line(high, stroke));
    painter.add(Shape::line(low, stroke));
}

fn paint_trace_marker(
    painter: &egui::Painter,
    center: Pos2,
    shape: TraceMarkerShape,
    color: Color32,
    background: Color32,
) {
    let radius = 3.4;
    let stroke = Stroke::new(1.35, color);
    match shape {
        TraceMarkerShape::Circle => {
            painter.circle(center, radius, background, stroke);
        }
        TraceMarkerShape::Square => {
            painter.rect(
                Rect::from_center_size(center, vec2(radius * 2.0, radius * 2.0)),
                0.0,
                background,
                stroke,
                egui::StrokeKind::Inside,
            );
        }
        TraceMarkerShape::Diamond => {
            painter.add(Shape::convex_polygon(
                vec![
                    pos2(center.x, center.y - radius - 0.5),
                    pos2(center.x + radius + 0.5, center.y),
                    pos2(center.x, center.y + radius + 0.5),
                    pos2(center.x - radius - 0.5, center.y),
                ],
                background,
                stroke,
            ));
        }
        TraceMarkerShape::Triangle => {
            painter.add(Shape::convex_polygon(
                vec![
                    pos2(center.x, center.y - radius - 0.8),
                    pos2(center.x + radius + 0.8, center.y + radius),
                    pos2(center.x - radius - 0.8, center.y + radius),
                ],
                background,
                stroke,
            ));
        }
        TraceMarkerShape::TriangleDown => {
            painter.add(Shape::convex_polygon(
                vec![
                    pos2(center.x - radius - 0.8, center.y - radius),
                    pos2(center.x + radius + 0.8, center.y - radius),
                    pos2(center.x, center.y + radius + 0.8),
                ],
                background,
                stroke,
            ));
        }
        TraceMarkerShape::Asterisk => {
            paint_trace_marker(painter, center, TraceMarkerShape::Plus, color, background);
            paint_trace_marker(painter, center, TraceMarkerShape::Cross, color, background);
        }
        TraceMarkerShape::Cross | TraceMarkerShape::Plus => {
            let diagonal = shape == TraceMarkerShape::Cross;
            let segments = if diagonal {
                [
                    [
                        pos2(center.x - radius, center.y - radius),
                        pos2(center.x + radius, center.y + radius),
                    ],
                    [
                        pos2(center.x - radius, center.y + radius),
                        pos2(center.x + radius, center.y - radius),
                    ],
                ]
            } else {
                [
                    [
                        pos2(center.x - radius, center.y),
                        pos2(center.x + radius, center.y),
                    ],
                    [
                        pos2(center.x, center.y - radius),
                        pos2(center.x, center.y + radius),
                    ],
                ]
            };
            for segment in segments {
                painter.line_segment(segment, Stroke::new(3.4, background));
                painter.line_segment(segment, stroke);
            }
        }
    }
}

/// The contiguous runs of finite points in a projected series.
///
/// A non-finite sample is a hole in the data, not a coordinate: a step that
/// failed to converge, or a division by zero inside a plotted expression.
/// The curve has to break there. Drawing straight through the gap would
/// assert a value the run never produced, and handing the raw coordinate to
/// the tessellator puts a vertex at infinity, which degenerates the whole
/// mesh — one bad sample would blank the trace rather than a single point of
/// it. Markers already skip non-finite points individually.
fn finite_runs(points: &[Pos2]) -> impl Iterator<Item = &[Pos2]> {
    points
        .split(|point| !(point.x.is_finite() && point.y.is_finite()))
        .filter(|run| !run.is_empty())
}

/// The distance between one trace's markers along X, in points: about an
/// inch of screen, so a pane carries a handful per curve.
const MARKER_PITCH: f32 = 96.0;

/// The least distance between the markers of two staggered traces. A pane
/// with more marked curves than fit in [`MARKER_PITCH`] widens its pitch.
const MARKER_STAGGER_MIN: f32 = 12.0;

/// Half a marker's width with its outline and its background halo, and a
/// point of air: a transition nearer than this to a marker's centre runs
/// under or against the glyph.
const MARKER_CLEARANCE: f32 = 5.5;

/// Half the width of a marker's centre. On a tread narrower than the glyph
/// — a fast clock zoomed out — no position clears both risers, and the
/// marker stands at the tread's middle with this much of it on the tread.
const MARKER_CORE: f32 = 1.5;

/// The farthest a marker leaves its slot to clear a transition, in points:
/// far enough to step off a riser a few columns wide onto the tread beside
/// it, and a small part of a pitch, so the rhythm still reads.
const MARKER_REACH: u8 = 8;

/// The rise across one pixel column, in points, above which the curve there
/// is a transition rather than a slope: steeper than about 72°.
const EDGE_SLOPE: f32 = 3.0;

/// The least height a run of steep columns spans to be a transition. A
/// smaller step lies under the glyph wherever the marker stands on it.
const EDGE_MIN_RISE: f32 = 6.0;

/// Where on its pane's X grid one trace's markers stand.
#[derive(Debug, Clone, Copy, PartialEq)]
struct MarkerSlots {
    /// The screen X of one of this trace's slots; the rest lie whole
    /// pitches either side of it.
    origin: f32,
    /// The distance between this trace's slots.
    pitch: f32,
}

impl MarkerSlots {
    /// The slots of each of `spec`'s traces, in order: `None` for a trace
    /// that carries no marker.
    fn for_pane(spec: &PlotSpec<'_>, plot_rect: Rect, color_safe: bool) -> Vec<Option<Self>> {
        let marked: Vec<bool> = spec
            .traces
            .iter()
            .enumerate()
            .map(|(index, trace)| {
                !trace.x.is_empty() && trace.look(index, color_safe).marker.is_some()
            })
            .collect();
        let count = marked.iter().filter(|marked| **marked).count();
        let mut rank = 0;
        marked
            .into_iter()
            .map(|marked| {
                marked.then(|| {
                    rank += 1;
                    Self::on_pane(spec, plot_rect, rank - 1, count)
                })
            })
            .collect()
    }

    /// The slots of the `rank`-th of a pane's `marked` marked traces.
    ///
    /// Every trace's slots are the pane's one grid, shifted by an even share
    /// of a pitch per rank: the curves' markers interleave instead of
    /// stacking, and each curve's stand in one regular rhythm. The grid is
    /// pinned to the data, not to the pane — X = 0, or 1 on a log axis, is a
    /// grid line — so a pan carries the markers with their curves instead of
    /// sliding the curves under them.
    fn on_pane(spec: &PlotSpec<'_>, plot_rect: Rect, rank: usize, marked: usize) -> Self {
        let marked = marked.max(1);
        let pitch = MARKER_PITCH.max(MARKER_STAGGER_MIN * marked as f32);
        let anchor = match spec.x_scale {
            XScale::Linear => 0.0,
            XScale::Log10 => 1.0,
        };
        let anchor =
            spec.x_scale.normalize(anchor, spec.x.min, spec.x.max) * f64::from(plot_rect.width());
        let phase = if anchor.is_finite() {
            anchor.rem_euclid(f64::from(pitch)) as f32
        } else {
            0.0
        };
        Self {
            origin: plot_rect.left() + phase + pitch * (rank % marked) as f32 / marked as f32,
            pitch,
        }
    }
}

/// Where a trace's categorical markers sit.
///
/// On an ordinary left-to-right curve, at its slots on the pane's X grid
/// (see [`MarkerSlots`]), so every marked curve carries its symbols in one
/// visible rhythm. Spacing them by arc length, as this once did, reads as
/// random on a stepped or digital trace: risers are long and treads short,
/// so symbols crowded the edges and thinned out on the flats.
///
/// Never on a transition. A riser is a step's jump between two held values,
/// and a marker on it stands at a value the trace never held: 101 on a
/// 128 → 64 step. Any run of pixel columns steeper than [`EDGE_SLOPE`] is a
/// transition however many columns it spans, not only one narrower than a
/// column. A marker whose glyph would cover one moves along the tread beside
/// it, at most [`MARKER_REACH`] points; on treads narrower than the glyph it
/// stands at a tread's middle; with no tread in reach it is left out.
///
/// A locus — Smith, Nyquist, an XY loop — doubles back on X, so it has no X
/// grid to stand on; its markers are spaced along the curve instead.
fn trace_marker_positions(points: &[Pos2], plot_rect: Rect, slots: MarkerSlots) -> Vec<Pos2> {
    if points.len() < 2 || plot_rect.width() < 72.0 {
        return Vec::new();
    }
    if doubles_back(points) {
        locus_marker_positions(points, plot_rect)
    } else {
        abscissa_marker_positions(points, plot_rect, slots)
    }
}

fn finite_point(point: Pos2) -> bool {
    point.x.is_finite() && point.y.is_finite()
}

/// Whether a projected curve runs right to left anywhere, by more than half
/// a pixel. A min/max envelope keeps each column's samples in encounter
/// order, which never steps back.
fn doubles_back(points: &[Pos2]) -> bool {
    points
        .windows(2)
        .any(|pair| finite_point(pair[0]) && finite_point(pair[1]) && pair[1].x < pair[0].x - 0.5)
}

/// A left-to-right curve's markers, one per slot that has a tread within
/// reach: the nearest position whose whole glyph clears every transition,
/// or failing that, on treads narrower than the glyph, the nearest whose
/// centre stands on one.
fn abscissa_marker_positions(points: &[Pos2], plot_rect: Rect, slots: MarkerSlots) -> Vec<Pos2> {
    let profile = ColumnProfile::of(points, plot_rect);
    let bounds = plot_rect.shrink(4.0);
    let offsets = std::iter::once(0.0).chain((1..=2 * MARKER_REACH).flat_map(|step| {
        let offset = f32::from(step) * 0.5;
        [-offset, offset]
    }));
    let mut placed = Vec::new();
    let mut cursor = 0;
    let mut slot =
        slots.origin - ((slots.origin - bounds.left()) / slots.pitch).floor() * slots.pitch;
    while slot <= bounds.right() {
        let nearest = |clearance: f32| {
            offsets
                .clone()
                .map(|offset| slot + offset)
                .find(|x| bounds.x_range().contains(*x) && profile.clear_at(*x, clearance))
        };
        let clear = nearest(MARKER_CLEARANCE).or_else(|| nearest(MARKER_CORE));
        if let Some(x) = clear
            && let Some(y) = height_at(points, &mut cursor, x)
            && bounds.y_range().contains(y)
        {
            placed.push(pos2(x, y));
        }
        slot += slots.pitch;
    }
    placed
}

/// The curve's height at screen X `x`, read along a left-to-right polyline
/// from `cursor`, which only moves forward. `None` in a hole in the data.
fn height_at(points: &[Pos2], cursor: &mut usize, x: f32) -> Option<f32> {
    while *cursor + 1 < points.len() {
        let (a, b) = (points[*cursor], points[*cursor + 1]);
        if finite_point(a) && finite_point(b) {
            if a.x > x {
                return None;
            }
            if b.x >= x && b.x > a.x {
                return Some(a.y + (b.y - a.y) * (x - a.x) / (b.x - a.x));
            }
        }
        *cursor += 1;
    }
    None
}

/// Which pixel columns of a plot a left-to-right curve passes through, and
/// which of those belong to a transition.
struct ColumnProfile {
    left: f32,
    covered: Vec<bool>,
    edge: Vec<bool>,
}

impl ColumnProfile {
    fn of(points: &[Pos2], plot_rect: Rect) -> Self {
        let left = plot_rect.left();
        let columns = plot_rect.width().ceil().max(0.0) as usize;
        let mut low = vec![f32::INFINITY; columns];
        let mut high = vec![f32::NEG_INFINITY; columns];
        let last = columns.saturating_sub(1) as f32;
        for pair in points.windows(2) {
            let (mut a, mut b) = (pair[0], pair[1]);
            if !(finite_point(a) && finite_point(b)) {
                continue;
            }
            if b.x < a.x {
                std::mem::swap(&mut a, &mut b);
            }
            let (from, to) = (a.x - left, b.x - left);
            if columns == 0 || to < 0.0 || from >= columns as f32 {
                continue;
            }
            let height = |x: f32| {
                if to > from {
                    a.y + (b.y - a.y) * (x - from) / (to - from)
                } else if x > from {
                    b.y
                } else {
                    a.y
                }
            };
            let first = from.floor().clamp(0.0, last) as usize;
            let final_column = to.floor().clamp(0.0, last) as usize;
            for column in first..=final_column {
                // A vertical segment is all of its rise inside one column.
                let (x0, x1) = if to > from {
                    ((column as f32).max(from), (column as f32 + 1.0).min(to))
                } else {
                    (from, from + 1.0)
                };
                let (y0, y1) = (height(x0), height(x1));
                low[column] = low[column].min(y0.min(y1));
                high[column] = high[column].max(y0.max(y1));
            }
        }
        let covered: Vec<bool> = low
            .iter()
            .zip(&high)
            .map(|(low, high)| low <= high)
            .collect();
        let steep = |column: usize| covered[column] && high[column] - low[column] > EDGE_SLOPE;
        let mut edge = vec![false; columns];
        let mut column = 0;
        while column < columns {
            if !steep(column) {
                column += 1;
                continue;
            }
            let start = column;
            let (mut top, mut bottom) = (f32::INFINITY, f32::NEG_INFINITY);
            while column < columns && steep(column) {
                top = top.min(low[column]);
                bottom = bottom.max(high[column]);
                column += 1;
            }
            if bottom - top >= EDGE_MIN_RISE {
                edge[start..column].fill(true);
            }
        }
        Self {
            left,
            covered,
            edge,
        }
    }

    /// Whether the columns within `clearance` of screen X `x` all carry the
    /// curve and none of them belongs to a transition.
    fn clear_at(&self, x: f32, clearance: f32) -> bool {
        let first = (x - clearance - self.left).floor();
        let last = (x + clearance - self.left).floor();
        if !(first >= 0.0 && last < self.covered.len() as f32) {
            return false;
        }
        (first as usize..=last as usize).all(|column| self.covered[column] && !self.edge[column])
    }
}

/// A locus's markers, evenly spaced along the curve.
///
/// Along the curve, not along the X axis. Walking X assumes the drawn path
/// advances left to right, which a locus does not — it doubles back, so the
/// search that only ever moved forward stuck at the turn and stamped the same
/// marker repeatedly on one point while the whole return branch got none.
/// Arc position keeps the count bounded by the pane's width.
///
/// A marker the walk puts on a vertical segment moves to the nearer end, and
/// one that would then crowd the marker before it is dropped.
fn locus_marker_positions(points: &[Pos2], plot_rect: Rect) -> Vec<Pos2> {
    const MIN_SPACING: f32 = MARKER_PITCH;
    let length = |a: Pos2, b: Pos2| (b - a).length();
    let total: f32 = points
        .windows(2)
        .filter(|pair| finite_point(pair[0]) && finite_point(pair[1]))
        .map(|pair| length(pair[0], pair[1]))
        .sum();
    // Only finite pairs contribute, so every term is a non-negative real and
    // the sum is either a non-negative real or `+inf` — never NaN. `<= 0.0`
    // therefore rejects exactly what `!(total > 0.0)` rejected: a polyline
    // with no drawable length.
    if total <= 0.0 {
        return Vec::new();
    }
    let budget = ((plot_rect.width() - 72.0) / MIN_SPACING).floor().max(1.0);
    let spacing = (total / budget).max(MIN_SPACING);
    let bounds = plot_rect.shrink(4.0);
    let mut placed = Vec::new();
    let mut travelled = 0.0f32;
    let mut next = spacing * 0.5;
    for pair in points.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        if !(finite_point(a) && finite_point(b)) {
            continue;
        }
        let segment = length(a, b);
        if segment <= 0.0 {
            continue;
        }
        // Within one pixel column: an edge, not a slope.
        let edge = (b.x - a.x).abs() < 1.0;
        while next <= travelled + segment {
            let along = ((next - travelled) / segment).clamp(0.0, 1.0);
            let point = if !edge {
                a + (b - a) * along
            } else if along < 0.5 {
                a
            } else {
                b
            };
            let crowded = placed
                .last()
                .is_some_and(|last: &Pos2| (point - *last).length() < MIN_SPACING * 0.5);
            if bounds.contains(point) && !crowded {
                placed.push(point);
            }
            next += spacing;
        }
        travelled += segment;
    }
    placed
}

fn paint_trace_markers(
    painter: &egui::Painter,
    points: &[Pos2],
    plot_rect: Rect,
    slots: MarkerSlots,
    shape: TraceMarkerShape,
    color: Color32,
    background: Color32,
) {
    for point in trace_marker_positions(points, plot_rect, slots) {
        paint_trace_marker(painter, point, shape, color, background);
    }
}

fn right_margin(spec: &PlotSpec<'_>) -> f32 {
    spec.right_margin.unwrap_or(MARGIN_RIGHT_PLAIN)
}

fn bottom_margin(spec: &PlotSpec<'_>) -> f32 {
    if spec.x_axis_chrome {
        MARGIN_BOTTOM
    } else {
        MARGIN_TOP
    }
}

/// What the Y axis states above its ticks: the anchor its ticks are offsets
/// from, which carries the unit itself, else its unit, else what a caption
/// row says for an axis without one.
fn y_caption_text(spec: &PlotSpec<'_>) -> Option<String> {
    spec.y
        .offset_anchor
        .clone()
        .or_else(|| (!spec.y.unit.is_empty()).then(|| spec.y.unit.clone()))
        .or_else(|| {
            spec.y_caption_row
                .as_ref()
                .and_then(|row| row.without_unit.clone())
        })
}

/// How close two Y tick labels may stand and both be drawn: a label's
/// height and the gap the renderer keeps between neighbours.
pub fn y_tick_pitch(ui: &Ui) -> f32 {
    ui.painter()
        .layout_no_wrap(
            "0".to_owned(),
            theme::mono(Y_TICK_FONT_SIZE, FontWeight::Regular),
            Color32::PLACEHOLDER,
        )
        .size()
        .y
        + Y_TICK_LABEL_GAP
}

fn top_margin(spec: &PlotSpec<'_>) -> f32 {
    if spec.y_caption_row.is_some() && y_caption_text(spec).is_some() {
        MARGIN_TOP_CAPTIONED
    } else {
        MARGIN_TOP
    }
}

fn inner_rect(rect: Rect, spec: &PlotSpec<'_>) -> Rect {
    Rect::from_min_max(
        pos2(
            rect.left() + spec.left_margin,
            rect.top() + top_margin(spec),
        ),
        pos2(
            rect.right() - right_margin(spec),
            rect.bottom() - bottom_margin(spec),
        ),
    )
}

fn axis_accessibility_range(axis: &super::spec::Axis) -> String {
    let unit = if axis.unit.is_empty() {
        String::new()
    } else {
        format!(" {}", axis.unit)
    };
    let label = axis
        .label
        .as_deref()
        .map_or_else(String::new, |label| format!("{label}, "));
    format!(
        "{label}{} to {}{}",
        axis.format_display_value(axis.min),
        axis.format_display_value(axis.max),
        unit
    )
}

fn plot_accessibility_label(spec: &PlotSpec<'_>, cursors: Option<&CursorPair>) -> String {
    use crate::accessibility::counted;
    let trace_count = spec
        .traces
        .iter()
        .filter(|trace| !trace.x.is_empty() && !trace.y.is_empty())
        .count();
    let trace_summary = if trace_count == 0 && spec.accessible_detail.is_some() {
        "custom-rendered engineering data".to_owned()
    } else {
        counted(trace_count, "visible trace", "visible traces")
    };
    let cursor_summary = cursors.map_or_else(String::new, |pair| match (pair.a, pair.b) {
        (Some(a), Some(b)) => format!(
            " Cursor A {}, cursor B {}, delta {}.",
            spec.x.format_display_value(a),
            spec.x.format_display_value(b),
            spec.x.format_display_delta(b - a)
        ),
        (Some(a), None) => format!(" Cursor A {}.", spec.x.format_display_value(a)),
        _ => String::new(),
    });
    let horizontal_cursor = match spec.horizontal_cursors {
        [Some(h1), Some(h2)] => format!(
            " Horizontal cursor H1 {}, H2 {}, delta {}.",
            spec.y.format_display_value(h1),
            spec.y.format_display_value(h2),
            spec.y.format_display_delta(h2 - h1)
        ),
        [Some(value), None] | [None, Some(value)] => {
            format!(" Horizontal cursor {}.", spec.y.format_display_value(value))
        }
        [None, None] => String::new(),
    };
    let custom_detail = spec
        .accessible_detail
        .map_or_else(String::new, |detail| format!(" {detail}."));
    format!(
        "{}.{} {}. X axis {}. Y axis {}. {}. {}.{}{} Drag to pan, use the mouse wheel to zoom, Shift-drag or right-drag to zoom a region, and double-click to fit the data.",
        spec.accessible_name,
        custom_detail,
        trace_summary,
        axis_accessibility_range(&spec.x),
        axis_accessibility_range(&spec.y),
        counted(spec.markers.len(), "marker", "markers"),
        counted(spec.limit_lines.len(), "project limit", "project limits"),
        cursor_summary,
        horizontal_cursor,
    )
}

/// A rect centered in `avail` whose INNER plot area (after this spec's
/// margins) is square. The XY viewers (Smith, Nyquist, pole-zero) need
/// circle grids and trace geometry on identical X/Y scales — a square
/// OUTER rect leaves the inner area taller than wide and the unit circle
/// would lie about |Γ| = 1.
pub fn square_outer_rect(avail: Rect, spec: &PlotSpec<'_>) -> Rect {
    let h_margins = spec.left_margin + right_margin(spec);
    let v_margins = top_margin(spec) + bottom_margin(spec);
    let inner = (avail.width() - h_margins)
        .min(avail.height() - v_margins)
        .max(48.0);
    Rect::from_center_size(avail.center(), vec2(inner + h_margins, inner + v_margins))
}

/// Render a plot filling the remaining space of `ui`.
///
/// `cursors` draws the A/B verticals when placed (in this plot's X domain).
/// `readout` supplies hover rows for the floating readout; `None` disables
/// the crosshair.
pub fn show(
    ui: &mut Ui,
    spec: &PlotSpec<'_>,
    cache: &mut DecimationCache,
    cursors: Option<&CursorPair>,
    readout: Option<&dyn Fn(f64) -> Vec<ReadoutRow>>,
) -> PlotResponse {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let rect = ui.available_rect_before_wrap();
    let response = ui.allocate_rect(rect, Sense::click_and_drag());
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Image,
            ui.is_enabled(),
            plot_accessibility_label(spec, cursors),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::GraphicsDocument);
    });
    let plot_rect = inner_rect(rect, spec);

    let mut out = PlotResponse {
        response,
        clicked_x: None,
        horizontal_cursor: None,
        plot_rect,
        view: ViewChange::default(),
        axes: ((spec.x.min, spec.x.max), (spec.y.min, spec.y.max)),
        x_label_rect: None,
        y_caption: None,
    };
    if plot_rect.width() < 24.0 || plot_rect.height() < 24.0 {
        theme::paint_focus_ring(ui, &out.response, rect);
        return out;
    }
    if !matches!(
        spec.x.max.partial_cmp(&spec.x.min),
        Some(std::cmp::Ordering::Greater)
    ) || !matches!(
        spec.y.max.partial_cmp(&spec.y.min),
        Some(std::cmp::Ordering::Greater)
    ) {
        // A view collapsed to a point has nothing to draw, but the gesture
        // that would rescue it still has to arrive. Returning before the
        // navigation left a stuck viewport with no way out but closing the
        // pane: every zoom and pan is stated against the span it is handed,
        // and a double-click to fit was being swallowed with them.
        let painter = ui.painter_at(rect);
        handle_navigation(ui, spec, &painter, plot_rect, &mut out, &t);
        theme::paint_focus_ring(ui, &out.response, rect);
        return out;
    }

    let mx = |x: f64| -> f32 {
        plot_rect.left()
            + (spec.x_scale.normalize(x, spec.x.min, spec.x.max) as f32) * plot_rect.width()
    };
    let my = |y: f64| -> f32 {
        plot_rect.bottom()
            - (spec.y_scale.normalize(y, spec.y.min, spec.y.max) as f32) * plot_rect.height()
    };
    let painter = ui.painter_at(rect);
    // Everything that describes a position in the data belongs inside the
    // plot area. Painted past it, a band or a limit line stops being a
    // statement about the data and becomes one about the tick gutter.
    let inside = ui.painter_at(plot_rect.expand(1.0));
    let tick_font = theme::mono(Y_TICK_FONT_SIZE, FontWeight::Regular);
    let grid = Stroke::new(1.0, c.canvas_grid);
    let frame = Stroke::new(1.0, c.canvas_frame);

    // ---- bands (under everything)
    for band in &spec.bands {
        let (left, right) = (mx(band.x0), mx(band.x1));
        let band_rect = Rect::from_min_max(
            pos2(left.min(right).max(plot_rect.left()), plot_rect.top()),
            pos2(left.max(right).min(plot_rect.right()), plot_rect.bottom()),
        );
        if band_rect.width() > 0.0 && band_rect.is_finite() {
            inside.rect_filled(band_rect, 0.0, c.accent_dim.gamma_multiply(0.4));
        }
    }

    // ---- grid + ticks
    // The x-axis unit owns the right end of the tick row; a tick label that
    // would run into it is dropped (its gridline stays).
    let x_end_label = if spec.x_axis_chrome {
        spec.x.end_label()
    } else {
        String::new()
    };
    let x_unit_left = if x_end_label.is_empty() {
        f32::INFINITY
    } else {
        let unit =
            painter.layout_no_wrap(x_end_label.clone(), tick_font.clone(), c.canvas_text_dim);
        plot_rect.right() - unit.size().x - 8.0
    };
    // Labels skip when they would collide with the previous label (dense
    // log decades at deep zoom) — every gridline still draws.
    // A logarithmic axis is read against its mantissas, 2 … 9 in each decade,
    // lighter than the decades, as ViVA and ADS rule one; a linear axis
    // subdivides only when the reader asks for a minor grid.
    let minor = Stroke::new(1.0, c.canvas_grid.gamma_multiply(0.45));
    let minors = |scale: XScale, axis: &Axis| {
        if spec.minor_grid || scale == XScale::Log10 {
            super::scale::minor_grid_values(scale, &axis.ticks, axis.min, axis.max)
        } else {
            Vec::new()
        }
    };
    for value in minors(spec.x_scale, &spec.x) {
        painter.vline(mx(value), plot_rect.y_range(), minor);
    }
    for value in minors(spec.y_scale, &spec.y) {
        painter.hline(plot_rect.x_range(), my(value), minor);
    }
    // The Y column's chrome is either its unit or, when the window is too
    // narrow to label absolutely, the anchor its ticks are offsets from —
    // which carries the unit itself. Without a row of its own it owns the
    // top of the column, so tick labels give way to it; with one, the plot
    // starts below it and every label is drawn.
    let y_chrome = y_caption_text(spec)
        .map(|text| painter.layout_no_wrap(text, tick_font.clone(), c.canvas_text_dim));
    let y_chrome_bottom = match (&y_chrome, &spec.y_caption_row) {
        (Some(galley), None) => rect.top() + Y_CAPTION_CENTER + galley.size().y * 0.5,
        _ => f32::NEG_INFINITY,
    };
    let mut last_label_right = f32::NEG_INFINITY;
    if spec.x_axis_chrome
        && let Some(anchor) = &spec.x.offset_anchor
    {
        // Stated once, in the gutter the Y labels leave free, and brighter
        // than the ticks: it is the value, and they are only the differences.
        let galley = painter.layout_no_wrap(anchor.clone(), tick_font.clone(), c.canvas_text);
        let width = galley.size().x;
        painter.galley(
            pos2(
                rect.left() + 2.0,
                rect.bottom() - 9.0 - galley.size().y * 0.5,
            ),
            galley,
            c.canvas_text,
        );
        last_label_right = rect.left() + 2.0 + width;
    }
    for (xv, label) in &spec.x.ticks {
        let px = mx(*xv);
        painter.vline(px, plot_rect.y_range(), grid);
        let galley = painter.layout_no_wrap(label.clone(), tick_font.clone(), c.canvas_text_dim);
        let half = galley.size().x * 0.5;
        if spec.x_axis_chrome && px + half <= x_unit_left && px - half >= last_label_right + 6.0 {
            last_label_right = px + half;
            painter.galley(
                pos2(px - half, rect.bottom() - 9.0 - galley.size().y * 0.5),
                galley,
                c.canvas_text_dim,
            );
        }
    }
    // Y labels skip on collision exactly as the X row does. A pane a few
    // rows tall carries the same tick count as a full-height one, and
    // stacked labels are less readable than none.
    let mut last_label_top = f32::INFINITY;
    for (yv, label) in &spec.y.ticks {
        let py = my(*yv);
        painter.hline(plot_rect.x_range(), py, grid);
        let galley = painter.layout_no_wrap(label.clone(), tick_font.clone(), c.canvas_text_dim);
        let (height, top) = (galley.size().y, py - galley.size().y * 0.5);
        if top + height <= last_label_top - Y_TICK_LABEL_GAP
            && top >= y_chrome_bottom + Y_TICK_LABEL_GAP
        {
            last_label_top = top;
            painter.galley(
                pos2(plot_rect.left() - 7.0 - galley.size().x, top),
                galley,
                c.canvas_text_dim,
            );
        }
    }
    // ---- reference lines
    let ref_stroke = Stroke::new(1.0, c.canvas_text_faint);
    for line in &spec.ref_lines {
        if !(line.y >= spec.y.min && line.y <= spec.y.max) {
            continue;
        }
        let py = my(line.y);
        inside.extend(Shape::dashed_line(
            &[pos2(plot_rect.left(), py), pos2(plot_rect.right(), py)],
            ref_stroke,
            4.0,
            3.0,
        ));
    }
    for line in &spec.limit_lines {
        if !(line.y >= spec.y.min && line.y <= spec.y.max) {
            continue;
        }
        let py = my(line.y);
        inside.extend(Shape::dashed_line(
            &[pos2(plot_rect.left(), py), pos2(plot_rect.right(), py)],
            Stroke::new(1.0, line.color),
            6.0,
            4.0,
        ));
        inside.text(
            pos2(
                plot_rect.right() - 4.0,
                (py - 3.0).max(plot_rect.top() + 11.0),
            ),
            Align2::RIGHT_BOTTOM,
            &line.label,
            theme::mono(9.0, FontWeight::Medium),
            line.color,
        );
    }

    // ---- custom underlay (histogram bars, eye acquisitions)
    if let Some(underlay) = &spec.underlay {
        let mapper = super::spec::PlotMapper {
            rect: plot_rect,
            x0: spec.x.min,
            x1: spec.x.max,
            x_scale: spec.x_scale,
            y0: spec.y.min,
            y1: spec.y.max,
        };
        let clipped = ui.painter_at(plot_rect.expand(1.0));
        underlay(&clipped, &mapper);
    }

    // ---- traces (clipped to the plot area)
    {
        let clipped = ui.painter_at(plot_rect.expand(1.0));
        // Round the column count up to 64-px buckets: a live window resize
        // then reuses cached envelopes instead of re-decimating every trace
        // every frame (the envelope is always at least pixel-dense).
        let columns = (plot_rect.width().ceil() as usize).next_multiple_of(64);
        // A parametric reduction quantizes both axes, so it needs the vertical
        // resolution too. Bucketed for the same reason as `columns`.
        let rows = (plot_rect.height().ceil() as usize).next_multiple_of(64);
        let marker_slots = MarkerSlots::for_pane(spec, plot_rect, t.color_safe_traces);
        for (trace_index, trace) in spec.traces.iter().enumerate() {
            if trace.x.is_empty() {
                continue;
            }
            let stroke = Stroke::new(trace.width, trace.color);
            let points: Vec<Pos2> = match trace.cache_key {
                Some(key) => cache
                    .series(
                        key,
                        trace.x,
                        trace.y,
                        super::decimate::TraceView {
                            x0: spec.x.min,
                            x1: spec.x.max,
                            y0: spec.y.min,
                            y1: spec.y.max,
                            x_scale: spec.x_scale,
                            y_scale: spec.y_scale,
                            columns,
                            rows,
                        },
                        trace.parametric,
                        trace.shape,
                    )
                    .iter()
                    .map(|p| pos2(mx(p[0]), my(p[1])))
                    .collect(),
                None => trace
                    .x
                    .iter()
                    .zip(trace.y.iter())
                    .map(|(&x, &y)| pos2(mx(x), my(y)))
                    .collect(),
            };
            let look = trace.look(trace_index, t.color_safe_traces);
            let dash = look.dash;
            // Dashing a dense min/max envelope would emit one shape per
            // dash along a path that zig-zags every column — thousands of
            // segments reading as noise. Sparse curves dash normally.
            if points.len() >= 2 {
                let dash = dash.filter(|_| points.len() < columns);
                for run in finite_runs(&points) {
                    if run.len() < 2 {
                        continue;
                    }
                    if let Some((dash_length, gap_length)) = dash {
                        clipped.extend(Shape::dashed_line(run, stroke, dash_length, gap_length));
                    } else {
                        clipped.add(Shape::line(run.to_vec(), stroke));
                    }
                }
            }
            let marker = look.marker.or_else(|| {
                (trace.show_single_point && points.len() == 1).then_some(TraceMarkerShape::Circle)
            });
            if let Some(marker) = marker {
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
                        paint_trace_marker(&clipped, point, marker, trace.color, c.canvas_bg);
                    }
                } else {
                    paint_trace_markers(
                        &clipped,
                        &points,
                        plot_rect,
                        marker_slots[trace_index]
                            .unwrap_or_else(|| MarkerSlots::on_pane(spec, plot_rect, 0, 1)),
                        marker,
                        trace.color,
                        c.canvas_bg,
                    );
                }
            }
        }
    }

    // ---- frame + axis units
    if spec.frame {
        painter.vline(plot_rect.left(), plot_rect.y_range(), frame);
        painter.hline(plot_rect.x_range(), plot_rect.bottom(), frame);
    }
    if let Some(galley) = y_chrome {
        let caption = Rect::from_min_size(
            pos2(
                plot_rect.left() - 7.0 - galley.size().x,
                rect.top() + Y_CAPTION_CENTER - galley.size().y * 0.5,
            ),
            galley.size(),
        );
        painter.galley(caption.min, galley, c.canvas_text_dim);
        out.y_caption = Some(caption);
    }
    if !x_end_label.is_empty() {
        out.x_label_rect = Some(painter.text(
            pos2(plot_rect.right(), rect.bottom() - 9.0),
            Align2::RIGHT_CENTER,
            x_end_label,
            tick_font.clone(),
            c.canvas_text_dim,
        ));
    }

    // ---- markers
    let tag_font = theme::mono(9.5, FontWeight::Regular);
    for marker in &spec.markers {
        // A marker states where something is. Off the window there is no
        // "where" to draw, and a dot pinned to the frame would claim a
        // position the data does not have.
        if !(marker.x >= spec.x.min && marker.x <= spec.x.max) {
            continue;
        }
        let px = mx(marker.x);
        let limit_line = marker.shape == MarkerShape::LimitLine;
        // A limit line spans the pane, so it is never clipped by Y; anything
        // else has to land inside the window. Stated positively because the
        // pair of negations it replaces said the same thing less directly.
        let y_is_drawable = limit_line || (marker.y >= spec.y.min && marker.y <= spec.y.max);
        if !y_is_drawable {
            continue;
        }
        // A limit line is a callout about the X position alone, so it spans
        // the plot and tags at the top; its `y` carries no meaning.
        let py = if limit_line {
            plot_rect.top()
        } else {
            my(marker.y)
        };
        if limit_line {
            inside.extend(Shape::dashed_line(
                &[pos2(px, plot_rect.top()), pos2(px, plot_rect.bottom())],
                Stroke::new(1.0, marker.color),
                5.0,
                4.0,
            ));
        } else {
            if marker.drop_line {
                inside.extend(Shape::dashed_line(
                    &[pos2(px, py), pos2(px, plot_rect.bottom())],
                    ref_stroke,
                    4.0,
                    3.0,
                ));
            }
            inside.circle(
                pos2(px, py),
                3.0,
                c.canvas_bg,
                Stroke::new(1.5, marker.color),
            );
        }

        let galley = painter.layout_no_wrap(marker.label.clone(), tag_font.clone(), marker.color);
        let (pad, tag_h) = (6.0, 16.0);
        let tag_w = galley.size().x + pad * 2.0;
        let mut tx = px + 9.0;
        let mut ty = if limit_line {
            plot_rect.top() + 4.0 + marker.label_dy
        } else {
            py - tag_h - 7.0 + marker.label_dy
        };
        if tx + tag_w > plot_rect.right() - 4.0 {
            tx = px - tag_w - 9.0;
        }
        if ty < plot_rect.top() + 2.0 {
            ty = py + 9.0;
        }
        // The tag belongs to the plot area even when its anchor sits at the
        // very edge of it, and a staggering offset can push it past either
        // end. Keep the whole tag inside rather than letting a corner of it
        // spill over the axis chrome.
        let tx = tx.clamp(
            plot_rect.left() + 2.0,
            (plot_rect.right() - tag_w - 2.0).max(plot_rect.left() + 2.0),
        );
        let ty = ty.clamp(
            plot_rect.top() + 2.0,
            (plot_rect.bottom() - tag_h - 2.0).max(plot_rect.top() + 2.0),
        );
        let tag_rect = Rect::from_min_size(pos2(tx, ty), vec2(tag_w, tag_h));
        inside.rect(
            tag_rect,
            t.radius,
            c.bg_elevated,
            Stroke::new(1.0, c.border_strong),
            egui::StrokeKind::Inside,
        );
        inside.galley(
            pos2(tx + pad, ty + (tag_h - galley.size().y) * 0.5),
            galley,
            marker.color,
        );
    }

    // ---- A/B cursors
    if let Some(pair) = cursors {
        let flag_font = theme::mono(9.0, FontWeight::Medium);
        let draw_cursor = |x: f64, letter: &str, color: Color32| {
            if x < spec.x.min || x > spec.x.max {
                return;
            }
            let px = mx(x);
            painter.vline(px, plot_rect.y_range(), Stroke::new(1.0, color));
            let flag = Rect::from_min_size(pos2(px - 7.0, plot_rect.top() - 1.0), vec2(14.0, 13.0));
            painter.rect_filled(flag, 1.0, color);
            painter.text(
                flag.center(),
                Align2::CENTER_CENTER,
                letter,
                flag_font.clone(),
                c.canvas_bg,
            );
        };
        if let Some(a) = pair.a {
            draw_cursor(a, "A", c.canvas_cursor_a);
        }
        if let Some(b) = pair.b {
            draw_cursor(b, "B", c.canvas_cursor_b);
        }
    }

    // ---- horizontal cursors: H1 in A's colour, H2 in B's, and the ΔY
    // between them dimensioned at the right edge when both are placed.
    let horizontal_colors = [c.canvas_cursor_a, c.canvas_cursor_b];
    let in_view = |y: f64| y.is_finite() && y >= spec.y.min && y <= spec.y.max;
    for (slot, y) in spec.horizontal_cursors.iter().enumerate() {
        let Some(y) = y.filter(|y| in_view(*y)) else {
            continue;
        };
        let py = my(y);
        let color = horizontal_colors[slot];
        painter.hline(plot_rect.x_range(), py, Stroke::new(1.0, color));
        painter.text(
            pos2(plot_rect.left() + 4.0, py - 3.0),
            Align2::LEFT_BOTTOM,
            format!("H{} {}", slot + 1, spec.y.format_display_value(y)),
            theme::mono(9.0, FontWeight::Medium),
            color,
        );
    }
    if let [Some(h1), Some(h2)] = spec.horizontal_cursors
        && h1.is_finite()
        && h2.is_finite()
    {
        let clamp = |y: f64| my(y).clamp(plot_rect.top(), plot_rect.bottom());
        let (top, bottom) = (clamp(h1).min(clamp(h2)), clamp(h1).max(clamp(h2)));
        let px = plot_rect.right() - 10.0;
        let stroke = Stroke::new(1.0, c.canvas_text);
        painter.vline(px, top..=bottom, stroke);
        for y in [top, bottom] {
            painter.hline(px - 3.0..=px + 3.0, y, stroke);
        }
        painter.text(
            pos2(px - 5.0, (top + bottom) * 0.5),
            Align2::RIGHT_CENTER,
            format!("\u{0394}Y {}", spec.y.format_display_delta(h2 - h1)),
            theme::mono(9.0, FontWeight::Medium),
            c.canvas_text,
        );
    }

    // ---- pointer: crosshair, readout, clicks
    if let Some(pointer) = out.response.hover_pos()
        && plot_rect.contains(pointer)
    {
        let frac = ((pointer.x - plot_rect.left()) / plot_rect.width()) as f64;
        let data_x = spec.x_scale.denormalize(frac, spec.x.min, spec.x.max);

        if let Some(readout) = readout {
            painter.extend(Shape::dashed_line(
                &[
                    pos2(pointer.x, plot_rect.top()),
                    pos2(pointer.x, plot_rect.bottom()),
                ],
                Stroke::new(1.0, c.canvas_text_faint),
                3.0,
                3.0,
            ));
            draw_readout(ui, &t, rect, plot_rect, pointer, &readout(data_x));
        }
        if out.response.clicked() {
            out.clicked_x = Some(data_x);
        }
        if spec.horizontal_cursor_interactive {
            // The slot is chosen where the gesture starts and kept for the
            // whole drag, so dragging H2 across H1 keeps moving H2.
            let grab = out.response.id.with("horizontal-cursor-slot");
            let starting = out.response.clicked()
                || out.response.drag_started_by(egui::PointerButton::Primary);
            if starting {
                let slot = horizontal_cursor_slot(spec.horizontal_cursors, pointer.y, &my);
                ui.data_mut(|data| data.insert_temp(grab, slot));
            }
            if out.response.clicked() || out.response.dragged_by(egui::PointerButton::Primary) {
                let fraction =
                    ((plot_rect.bottom() - pointer.y) / plot_rect.height()).clamp(0.0, 1.0);
                let slot = ui
                    .data(|data| data.get_temp::<usize>(grab))
                    .unwrap_or_else(|| {
                        horizontal_cursor_slot(spec.horizontal_cursors, pointer.y, &my)
                    });
                out.horizontal_cursor = Some((
                    slot.min(1),
                    spec.y_scale
                        .denormalize(f64::from(fraction), spec.y.min, spec.y.max),
                ));
            }
        }
    }

    // ---- navigation gestures: wheel zoom, drag pan, zoom box, fit
    handle_navigation(ui, spec, &painter, plot_rect, &mut out, &t);

    theme::paint_focus_ring(ui, &out.response, rect);

    out
}

/// Which horizontal cursor a press at screen height `pointer_y` takes: one
/// already placed within grabbing distance of it, else the first unplaced
/// slot, else the nearer of the two.
fn horizontal_cursor_slot(
    cursors: [Option<f64>; 2],
    pointer_y: f32,
    to_screen: &impl Fn(f64) -> f32,
) -> usize {
    const GRAB: f32 = 6.0;
    let nearest = cursors
        .iter()
        .enumerate()
        .filter_map(|(slot, y)| {
            y.filter(|y| y.is_finite())
                .map(|y| (slot, (to_screen(y) - pointer_y).abs()))
        })
        .min_by(|a, b| a.1.total_cmp(&b.1));
    match nearest {
        Some((slot, distance)) if distance <= GRAB => slot,
        _ => cursors
            .iter()
            .position(Option::is_none)
            .unwrap_or_else(|| nearest.map_or(0, |(slot, _)| slot)),
    }
}

/// A range wide enough to still be a view, or nothing.
///
/// Screen coordinates are `f32`, so a window narrower than about a billionth
/// of its own magnitude is already finer than the projection that draws it.
/// A window of exactly zero is worse than useless: every gesture states its
/// result as a fraction of the span it was handed, so a stored zero span is a
/// viewport nothing but a fit can ever widen again.
fn floored_span(range: (f64, f64), scale: XScale) -> Option<(f64, f64)> {
    const RELATIVE_FLOOR: f64 = 1.0e-9;
    let (low, high) = range;
    if !(low.is_finite() && high.is_finite()) {
        return None;
    }
    match scale {
        XScale::Log10 => {
            if !(low > 0.0 && high > 0.0) {
                return None;
            }
            let (low, high) = if high > low { (low, high) } else { (high, low) };
            if high / low >= 1.0 + RELATIVE_FLOOR {
                return Some((low, high));
            }
            let centre = (low * high).sqrt();
            let half = (1.0 + RELATIVE_FLOOR).sqrt();
            (centre > 0.0 && centre.is_finite()).then(|| (centre / half, centre * half))
        }
        XScale::Linear => {
            let (low, high) = if high > low { (low, high) } else { (high, low) };
            let centre = (low + high) * 0.5;
            let floor = (centre.abs() * RELATIVE_FLOOR).max(f64::MIN_POSITIVE);
            if high - low >= floor {
                return Some((low, high));
            }
            let (low, high) = (centre - floor * 0.5, centre + floor * 0.5);
            (high > low).then_some((low, high))
        }
    }
}

/// Interpret wheel/drag gestures against the spec's current ranges and
/// fill `out.view` with the resulting data-space ranges. The caller owns
/// the view state (this engine is stateless across frames except for the
/// zoom-box anchor, which lives in egui memory keyed by the widget id).
fn handle_navigation(
    ui: &mut Ui,
    spec: &PlotSpec<'_>,
    painter: &egui::Painter,
    plot_rect: Rect,
    out: &mut PlotResponse,
    t: &Tokens,
) {
    let c = t.color;
    let interaction = interaction_mode(ui.ctx());

    // Fraction coordinates: fx in 0..1 left→right, fy in 0..1 bottom→top.
    let fx_of = |px: f32| ((px - plot_rect.left()) / plot_rect.width()) as f64;
    let fy_of = |py: f32| ((plot_rect.bottom() - py) / plot_rect.height()) as f64;
    let denorm_x = |frac: f64| spec.x_scale.denormalize(frac, spec.x.min, spec.x.max);
    let denorm_y = |frac: f64| spec.y_scale.denormalize(frac, spec.y.min, spec.y.max);
    // Every range this reports is about to become the next frame's view, so
    // none of them may be degenerate: a stored zero span is a viewport no
    // later gesture can widen, because every gesture is relative to it.
    let store_x = |range: (f64, f64)| floored_span(range, spec.x_scale);
    let store_y = |range: (f64, f64)| floored_span(range, spec.y_scale);

    // Double-click restores the automatic fit.
    if interaction != InteractionMode::Select && out.response.double_clicked() {
        out.view.reset = true;
        return;
    }

    let shift = ui.input(|i| i.modifiers.shift);
    let ctrl = ui.input(|i| i.modifiers.ctrl);
    let box_id = out.response.id.with("plot.zoombox");

    // Zoom box: Shift+primary drag or right drag. The anchor survives
    // across frames in egui memory; the box zooms both axes on release.
    //
    // Select mode arms none of it. Its contract is that the view does not
    // move, and a modifier held for a selection gesture was rescaling the
    // plot underneath the selection being made.
    let box_drag_started = interaction != InteractionMode::Select
        && ((out.response.drag_started_by(egui::PointerButton::Primary)
            && (shift || interaction == InteractionMode::Zoom))
            || out.response.drag_started_by(egui::PointerButton::Secondary));
    if box_drag_started
        && let Some(pos) = out.response.interact_pointer_pos()
        && plot_rect.contains(pos)
    {
        ui.memory_mut(|m| m.data.insert_temp(box_id, pos));
    }
    let anchor = ui.memory(|m| m.data.get_temp::<Pos2>(box_id));
    if let Some(anchor) = anchor {
        let dragging = out.response.dragged_by(egui::PointerButton::Primary)
            || out.response.dragged_by(egui::PointerButton::Secondary);
        let stopped = out.response.drag_stopped_by(egui::PointerButton::Primary)
            || out.response.drag_stopped_by(egui::PointerButton::Secondary);
        let corner = out
            .response
            .interact_pointer_pos()
            .map(|p| plot_rect.clamp(p));

        if dragging && let Some(corner) = corner {
            let band = Rect::from_two_pos(anchor, corner);
            painter.rect(
                band,
                0.0,
                c.canvas_cursor_a.gamma_multiply(0.08),
                Stroke::new(1.0, c.canvas_cursor_a),
                egui::StrokeKind::Inside,
            );
        }
        if stopped {
            ui.memory_mut(|m| m.data.remove::<Pos2>(box_id));
            if let Some(corner) = corner {
                let band = Rect::from_two_pos(anchor, corner);
                if band.width() > 6.0 && band.height() > 6.0 {
                    let (fx0, fx1) = (fx_of(band.left()).max(0.0), fx_of(band.right()).min(1.0));
                    out.view.x = store_x((denorm_x(fx0), denorm_x(fx1)));
                    let (fy0, fy1) = (fy_of(band.bottom()).max(0.0), fy_of(band.top()).min(1.0));
                    out.view.y = store_y((denorm_y(fy0), denorm_y(fy1)));
                }
            }
        }
        return;
    }

    // Drag pan (primary, unmodified): the content follows the pointer, along
    // each axis once the drag has travelled past a dead zone on it. A pan in
    // time wobbles by a pixel or two; moving Y by that pinned an axis that
    // was following the data in view, and a fit the reader never asked to
    // leave stopped refitting.
    if out.response.dragged_by(egui::PointerButton::Primary)
        && !shift
        && matches!(interaction, InteractionMode::All | InteractionMode::Pan)
    {
        let delta = out.response.drag_delta();
        let travelled = ui
            .input(|input| Some(input.pointer.interact_pos()? - input.pointer.press_origin()?))
            .unwrap_or(egui::Vec2::INFINITY);
        if delta != egui::Vec2::ZERO {
            let dfx = -f64::from(delta.x) / f64::from(plot_rect.width());
            if dfx != 0.0 && travelled.x.abs() > PAN_DEAD_ZONE {
                out.view.x = store_x((denorm_x(dfx), denorm_x(1.0 + dfx)));
            }
            let dy = f64::from(delta.y) / f64::from(plot_rect.height());
            if dy != 0.0 && travelled.y.abs() > PAN_DEAD_ZONE {
                out.view.y = store_y((denorm_y(dy), denorm_y(1.0 + dy)));
            }
        }
        return;
    }

    // Wheel zoom about the cursor: X by default, Y with Ctrl held.
    if interaction != InteractionMode::Select
        && let Some(pointer) = out.response.hover_pos()
        && plot_rect.contains(pointer)
    {
        let scroll = ui.input(|i| i.smooth_scroll_delta.y);
        if scroll != 0.0 {
            // Consume the wheel so an enclosing ScrollArea doesn't also
            // scroll the strip list while the user zooms a plot.
            ui.input_mut(|i| {
                i.smooth_scroll_delta = egui::Vec2::ZERO;
            });
            let factor = (f64::from(-scroll) * 0.002).exp().clamp(0.05, 20.0);
            if ctrl {
                let fy = fy_of(pointer.y);
                let (f0, f1) = (fy * (1.0 - factor), fy + (1.0 - fy) * factor);
                out.view.y = store_y((denorm_y(f0), denorm_y(f1)));
            } else {
                let fx = fx_of(pointer.x);
                let (f0, f1) = (fx * (1.0 - factor), fx + (1.0 - fx) * factor);
                out.view.x = store_x((denorm_x(f0), denorm_x(f1)));
            }
        }
    }
}

/// The floating hover readout: elevated box, mono rows, k dim / v bright.
fn draw_readout(
    ui: &Ui,
    t: &Tokens,
    rect: Rect,
    plot_rect: Rect,
    pointer: Pos2,
    rows: &[ReadoutRow],
) {
    if rows.is_empty() {
        return;
    }
    let c = t.color;
    let font = theme::mono(11.0, FontWeight::Regular);
    let painter = ui.painter();

    let mut key_w = 0.0f32;
    let mut val_w = 0.0f32;
    let galleys: Vec<_> = rows
        .iter()
        .map(|(k, v)| {
            let kg = painter.layout_no_wrap(k.clone(), font.clone(), c.text_dim);
            let vg = painter.layout_no_wrap(v.clone(), font.clone(), c.text);
            key_w = key_w.max(kg.size().x);
            val_w = val_w.max(vg.size().x);
            (kg, vg)
        })
        .collect();

    let (pad_x, pad_y, gap, line_h) = (8.0, 5.0, 10.0, 16.0);
    let box_w = pad_x * 2.0 + key_w + gap + val_w;
    let box_h = pad_y * 2.0 + line_h * rows.len() as f32;
    let mut origin = pointer + vec2(14.0, -14.0);
    if origin.x + box_w > rect.right() - 4.0 {
        origin.x = pointer.x - box_w - 14.0;
    }
    origin.y = origin.y.clamp(
        plot_rect.top(),
        (rect.bottom() - box_h - 4.0).max(plot_rect.top()),
    );

    let bg = Rect::from_min_size(origin, vec2(box_w, box_h));
    painter.rect(
        bg,
        t.radius,
        c.bg_elevated,
        Stroke::new(1.0, c.border_strong),
        egui::StrokeKind::Inside,
    );
    for (i, (kg, vg)) in galleys.into_iter().enumerate() {
        let y = origin.y + pad_y + i as f32 * line_h;
        painter.galley(pos2(origin.x + pad_x, y), kg, c.text_dim);
        painter.galley(
            pos2(origin.x + pad_x + key_w + gap + val_w - vg.size().x, y),
            vg,
            c.text,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::format::tick_label;
    use crate::plot::{Axis, Marker, Trace, XScale};

    #[test]
    fn an_ordinary_curve_stays_a_single_unbroken_run() {
        // Every plot in the workbench goes through this, so the common case
        // has to stay exactly one stroke. Splitting a clean curve into
        // fragments would still tessellate finitely and pass every test
        // that only asks whether the vertices are finite.
        let points = (0..64)
            .map(|i| pos2(i as f32, (i as f32 * 0.1).sin()))
            .collect::<Vec<_>>();
        let runs = finite_runs(&points).collect::<Vec<_>>();

        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].len(), points.len());
    }

    #[test]
    fn a_hole_breaks_the_curve_without_swallowing_the_data_around_it() {
        let mut points = (0..10).map(|i| pos2(i as f32, 1.0)).collect::<Vec<_>>();
        points[4] = pos2(4.0, f32::NAN);
        points[5] = pos2(f32::INFINITY, 1.0);
        let runs = finite_runs(&points).collect::<Vec<_>>();

        // Two strokes, and only the two bad samples are missing — the curve
        // resumes rather than ending at the hole.
        assert_eq!(runs.len(), 2);
        assert_eq!(runs[0].len(), 4);
        assert_eq!(runs[1].len(), 4);
    }

    #[test]
    fn a_wholly_undefined_series_strokes_nothing() {
        let points = (0..8).map(|_| pos2(f32::NAN, f32::NAN)).collect::<Vec<_>>();

        assert_eq!(finite_runs(&points).count(), 0);
    }

    #[test]
    fn color_safe_trace_cycle_has_a_marker_shape_per_hue_and_preserves_phase_dashing() {
        let styles = (0..TRACE_HUES)
            .map(|index| color_safe_trace_style(index, false))
            .collect::<Vec<_>>();
        let marker_shapes = styles
            .iter()
            .map(|style| style.marker)
            .collect::<std::collections::HashSet<_>>();

        assert_eq!(marker_shapes.len(), TRACE_HUES);
        assert!(styles[0].dash.is_none());
        assert!(styles[1..].iter().all(|style| style.dash.is_some()));
        assert!(color_safe_trace_style(0, true).dash.is_some());
        assert_eq!(color_safe_trace_style(TRACE_HUES, false), styles[0]);
    }

    /// The ninth trace takes the first one's hue in a dash of its own, with
    /// color-safe traces off or on, and a legend reading the same look draws
    /// the same swatch.
    #[test]
    fn a_second_pass_through_the_hues_draws_in_another_dash() {
        let (x, y) = ([0.0, 1.0], [0.0, 1.0]);
        let trace = |slot| Trace::new(&x, &y, Color32::RED).palette_slot(slot);
        for color_safe in [false, true] {
            let first = trace(0).look(0, color_safe);
            let ninth = trace(TRACE_HUES).look(0, color_safe);
            assert_ne!(first.dash, ninth.dash, "color-safe {color_safe}");
            assert_eq!(first.marker, ninth.marker, "color-safe {color_safe}");
            for slot in 1..TRACE_HUES {
                let pass = trace(slot + TRACE_HUES).look(0, color_safe);
                assert_ne!(trace(slot).look(0, color_safe), pass, "slot {slot}");
            }
        }
        assert_eq!(trace(3).look(0, false).dash, None);
        assert_eq!(trace(3).look(0, false).marker, None);
    }

    /// A fresh theme draws every first-pass curve as a plain line in its
    /// hue: no dash, no symbol.
    #[test]
    fn the_default_theme_draws_traces_solid() {
        let (x, y) = ([0.0, 1.0], [0.0, 1.0]);
        let solid = TraceLook {
            dash: None,
            marker: None,
        };
        let color_safe = crate::Theme::default().tokens().color_safe_traces;
        for slot in 0..TRACE_HUES {
            let placed = Trace::new(&x, &y, Color32::RED).palette_slot(slot);
            assert_eq!(placed.look(slot, color_safe), solid, "slot {slot}");
            let unplaced = Trace::new(&x, &y, Color32::RED);
            assert_eq!(unplaced.look(slot, color_safe), solid, "ordinal {slot}");
        }
    }

    /// With colour-safe traces on, every curve past the eighth is dashed and
    /// no two curves share a dash and a marker. The first pass's dashes are
    /// never reused, so a second-pass curve cannot pass for a first-pass one
    /// even where a reader cannot tell the hues apart.
    #[test]
    fn a_colour_safe_later_pass_is_always_dashed_and_never_a_first_pass_look() {
        let (x, y) = ([0.0, 1.0], [0.0, 1.0]);
        let trace = |slot| Trace::new(&x, &y, Color32::RED).palette_slot(slot);
        let first: Vec<TraceLook> = (0..TRACE_HUES)
            .map(|slot| trace(slot).look(0, true))
            .collect();
        let later: Vec<TraceLook> = (TRACE_HUES..4 * TRACE_HUES)
            .map(|slot| trace(slot).look(0, true))
            .collect();
        for (index, look) in later.iter().enumerate() {
            let slot = index + TRACE_HUES;
            assert!(look.dash.is_some(), "slot {slot} is drawn solid");
            assert!(
                first.iter().all(|first| first.dash != look.dash),
                "slot {slot} reuses a first-pass dash"
            );
        }
        let looks: Vec<TraceLook> = first.iter().chain(&later).copied().collect();
        for (index, look) in looks.iter().enumerate() {
            for (other_index, other) in looks.iter().enumerate().skip(index + 1) {
                assert_ne!(look, other, "slots {index} and {other_index} draw alike");
            }
        }
        // A semantically dashed curve stays dashed on its first pass.
        assert!(
            Trace::new(&x, &y, Color32::RED)
                .palette_slot(0)
                .dashed()
                .look(0, true)
                .dash
                .is_some()
        );
    }

    /// Sixteen curves on one colour-safe plot, read back from the frame: only
    /// the first is stroked solid, and each curve on the second pass through
    /// the hues is dashed with a gap its first-pass twin does not draw.
    #[test]
    fn a_colour_safe_frame_draws_the_second_pass_dashed() {
        let x = [0.0, 1.0];
        let levels: Vec<[f64; 2]> = (0..2 * TRACE_HUES)
            .map(|slot| [slot as f64, slot as f64])
            .collect();
        let color = |slot: usize| Color32::from_rgb(40 + 12 * slot as u8, 90, 160);
        let mut spec = PlotSpec::new(
            Axis::linear(0.0, 1.0, "s"),
            XScale::Linear,
            Axis::linear(-1.0, 2.0 * TRACE_HUES as f64, "V"),
        );
        for (slot, y) in levels.iter().enumerate() {
            spec.traces
                .push(Trace::new(&x, y, color(slot)).palette_slot(slot));
        }
        let ctx = egui::Context::default();
        crate::Theme {
            colorblind_traces: true,
            ..crate::Theme::default()
        }
        .apply(&ctx);
        let mut cache = DecimationCache::default();
        let mut shapes = Vec::new();
        for _ in 0..2 {
            let input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(640.0, 480.0))),
                ..Default::default()
            };
            shapes = ctx
                .run_ui(input, |ui| {
                    show(ui, &spec, &mut cache, None, None);
                })
                .shapes;
        }
        fn flatten(shape: &Shape, out: &mut Vec<Shape>) {
            match shape {
                Shape::Vec(inner) => inner.iter().for_each(|shape| flatten(shape, out)),
                shape => out.push(shape.clone()),
            }
        }
        let mut painted = Vec::new();
        for clipped in &shapes {
            flatten(&clipped.shape, &mut painted);
        }
        // Each curve as the frame drew it: stroked solid across the plot, or
        // as horizontal dashes whose median gap is measured.
        let drawn = |slot: usize| -> (bool, Option<f32>) {
            let mut solid = false;
            let mut dashes: Vec<(f32, f32)> = Vec::new();
            for shape in &painted {
                match shape {
                    Shape::Path(path)
                        if !path.closed
                            && path.stroke.color == egui::epaint::ColorMode::Solid(color(slot))
                            && path.points.len() >= 2
                            && (path.points[path.points.len() - 1].x - path.points[0].x)
                                > 200.0 =>
                    {
                        solid = true;
                    }
                    Shape::LineSegment { points, stroke }
                        if stroke.color == color(slot)
                            && (points[0].y - points[1].y).abs() < 0.01
                            && (points[1].x - points[0].x).abs() > 0.5 =>
                    {
                        let (a, b) = (points[0].x.min(points[1].x), points[0].x.max(points[1].x));
                        dashes.push((a, b));
                    }
                    _ => {}
                }
            }
            dashes.sort_by(|a, b| a.0.total_cmp(&b.0));
            let mut gaps: Vec<f32> = dashes
                .windows(2)
                .map(|pair| pair[1].0 - pair[0].1)
                .collect();
            gaps.sort_by(f32::total_cmp);
            (solid, gaps.get(gaps.len() / 2).copied())
        };
        let solid: Vec<usize> = (0..2 * TRACE_HUES).filter(|slot| drawn(*slot).0).collect();
        assert_eq!(solid, [0], "the curves stroked solid");
        for hue in 0..TRACE_HUES {
            let (_, second_gap) = drawn(hue + TRACE_HUES);
            let second_gap =
                second_gap.unwrap_or_else(|| panic!("slot {} is not dashed", hue + TRACE_HUES));
            if let (_, Some(first_gap)) = drawn(hue) {
                assert!(
                    second_gap > first_gap + 2.0,
                    "hue {hue}: second-pass gap {second_gap} beside the first pass's {first_gap}"
                );
            }
        }
    }

    #[test]
    fn a_family_marker_is_kept_and_none_is_invented() {
        let (x, y) = ([0.0], [0.0]);
        assert_eq!(Trace::new(&x, &y, Color32::RED).look(0, false).marker, None);
        assert_eq!(
            Trace::new(&x, &y, Color32::RED)
                .marker_style(2)
                .look(0, false)
                .marker,
            Some(TraceMarkerShape::Diamond)
        );
    }

    #[test]
    fn accessibility_label_reports_axes_traces_markers_and_cursors() {
        let x = [1.0, 10.0];
        let y = [-3.0, 4.0];
        let mut spec = PlotSpec::new(
            Axis::log_decades(1.0, 10.0, "Hz"),
            XScale::Log10,
            Axis::linear(-5.0, 5.0, "dB"),
        )
        .accessible_name("Bode plot");
        spec.traces.push(Trace::new(&x, &y, egui::Color32::WHITE));
        spec.markers.push(Marker {
            x: 10.0,
            y: 4.0,
            color: egui::Color32::WHITE,
            label: "UGF".to_owned(),
            drop_line: true,
            label_dy: 0.0,
            shape: MarkerShape::Point,
        });

        let label = plot_accessibility_label(
            &spec,
            Some(&CursorPair {
                a: Some(2.0),
                b: Some(5.0),
            }),
        );

        assert!(label.starts_with("Bode plot. 1 visible trace."));
        assert!(label.contains("X axis 1 to 10 Hz."));
        assert!(label.contains(&format!("Y axis {} to 5 dB.", tick_label(-5.0))));
        assert!(label.contains("1 marker."));
        assert!(label.contains("Cursor A 2, cursor B 5, delta 3."));
    }

    #[test]
    fn accessibility_label_describes_custom_underlay_data_without_claiming_zero_data() {
        let spec = PlotSpec::new(
            Axis::linear(0.0, 2.0, "UI"),
            XScale::Linear,
            Axis::linear(-1.0, 1.0, "V"),
        )
        .accessible_name("Eye diagram")
        .accessible_detail("34 folded acquisitions; compliance mask visible");

        let label = plot_accessibility_label(&spec, None);

        assert!(label.contains("34 folded acquisitions; compliance mask visible"));
        assert!(label.contains("custom-rendered engineering data"));
        assert!(!label.contains("0 visible traces"));
    }

    #[test]
    fn shared_axis_panes_keep_aligned_margins_without_repeated_x_chrome() {
        let ordinary = PlotSpec::new(
            Axis::linear(0.0, 1.0, "s"),
            XScale::Linear,
            Axis::linear(0.0, 1.0, "V"),
        );
        let shared = PlotSpec::new(
            Axis::linear(0.0, 1.0, "s"),
            XScale::Linear,
            Axis::linear(0.0, 1.0, "V"),
        )
        .without_x_axis_chrome()
        .with_right_margin(54.0);

        assert_eq!(right_margin(&shared), 54.0);
        assert_eq!(bottom_margin(&ordinary), MARGIN_BOTTOM);
        assert_eq!(bottom_margin(&shared), MARGIN_TOP);
        let outer = Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 300.0));
        assert_eq!(inner_rect(outer, &shared).right(), 746.0);
        assert!(inner_rect(outer, &shared).height() > inner_rect(outer, &ordinary).height());
    }

    #[test]
    fn logarithmic_y_spec_uses_decade_geometry() {
        let spec = PlotSpec::new(
            Axis::linear(0.0, 1.0, "s"),
            XScale::Linear,
            Axis::log_decades(1.0, 100.0, "V"),
        )
        .with_log_y();
        assert_eq!(spec.y_scale, XScale::Log10);
        assert!((spec.y_scale.normalize(10.0, spec.y.min, spec.y.max) - 0.5).abs() < 1.0e-12);
        assert!((spec.y_scale.denormalize(0.5, spec.y.min, spec.y.max) - 10.0).abs() < 1.0e-12);
    }

    /// Drive `show` for a sequence of input frames and hand back what the
    /// last frame painted plus the navigation it reported. Painted shapes are
    /// the only honest record of what a reader sees.
    fn plot_frames(
        spec: &PlotSpec<'_>,
        size: egui::Vec2,
        frames: &[Vec<egui::Event>],
    ) -> (Vec<egui::epaint::ClippedShape>, ViewChange) {
        let ctx = egui::Context::default();
        crate::Theme::default().apply(&ctx);
        let mut cache = DecimationCache::default();
        let mut view = ViewChange::default();
        let mut shapes = Vec::new();
        for events in frames {
            let input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), size)),
                events: events.clone(),
                ..Default::default()
            };
            let output = ctx.run_ui(input, |ui| {
                let out = show(ui, spec, &mut cache, None, None);
                view = out.view;
            });
            shapes = output.shapes;
        }
        (shapes, view)
    }

    fn press(at: Pos2, button: egui::PointerButton, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos: at,
            button,
            pressed,
            modifiers: egui::Modifiers::default(),
        }
    }

    fn double_click(at: Pos2) -> Vec<egui::Event> {
        vec![
            egui::Event::PointerMoved(at),
            press(at, egui::PointerButton::Primary, true),
            press(at, egui::PointerButton::Primary, false),
            press(at, egui::PointerButton::Primary, true),
            press(at, egui::PointerButton::Primary, false),
        ]
    }

    fn painted_text(shapes: &[egui::epaint::ClippedShape]) -> Vec<(String, Rect)> {
        shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                Shape::Text(text) => Some((
                    text.galley.text().to_owned(),
                    Rect::from_min_size(text.pos, text.galley.size()),
                )),
                _ => None,
            })
            .collect()
    }

    fn painted_fills(shapes: &[egui::epaint::ClippedShape]) -> Vec<Rect> {
        shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                Shape::Rect(rect) if rect.fill != Color32::TRANSPARENT => Some(rect.rect),
                _ => None,
            })
            .collect()
    }

    /// The gridlines a frame painted across the whole plot, as (position,
    /// ink): vertical ones at their x, horizontal ones at their y.
    fn rulings(
        shapes: &[egui::epaint::ClippedShape],
        plot: Rect,
        vertical: bool,
    ) -> Vec<(f32, Color32)> {
        shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                Shape::LineSegment { points, stroke } => {
                    let [a, b] = *points;
                    let spans = if vertical {
                        a.x == b.x && (a.y - b.y).abs() >= plot.height() - 0.5
                    } else {
                        a.y == b.y && (a.x - b.x).abs() >= plot.width() - 0.5
                    };
                    spans.then_some((if vertical { a.x } else { a.y }, stroke.color))
                }
                _ => None,
            })
            .collect()
    }

    /// A logarithmic axis is ruled at 2 … 9 in every decade, lighter than its
    /// decades, whether or not the reader turned the minor grid on: it is
    /// how a log scale is read, as ViVA and ADS draw it. Before, a Bode
    /// plot's frequency axis carried its decade lines and nothing between.
    #[test]
    fn a_log_axis_is_ruled_at_every_mantissa_lighter_than_its_decades() {
        let x = [10.0, 1.0e5];
        let y = [1.0e-3, 1.0];
        let mut spec = PlotSpec::new(
            Axis::log_decades(10.0, 1.0e5, "Hz"),
            XScale::Log10,
            Axis::log_decades(1.0e-3, 1.0, "V"),
        )
        .with_log_y();
        spec.traces.push(Trace::new(&x, &y, Color32::WHITE));
        assert!(!spec.minor_grid, "the reader's minor grid is off");
        let size = vec2(640.0, 400.0);
        let plot = inner_rect(Rect::from_min_size(pos2(0.0, 0.0), size), &spec);
        let (shapes, _) = plot_frames(&spec, size, &[Vec::new()]);
        let decade = Tokens::get(&{
            let ctx = egui::Context::default();
            crate::Theme::default().apply(&ctx);
            ctx
        })
        .color
        .canvas_grid;

        for (vertical, (min, max), decades) in [(true, (10.0, 1.0e5), 4), (false, (1.0e-3, 1.0), 3)]
        {
            let lines = rulings(&shapes, plot, vertical);
            let minors: Vec<f32> = lines
                .iter()
                .filter(|(_, ink)| ink.a() < decade.a())
                .map(|(at, _)| *at)
                .collect();
            assert_eq!(
                minors.len(),
                8 * decades,
                "vertical {vertical}: eight mantissa lines per decade, {minors:?}"
            );
            assert!(
                lines.iter().any(|(_, ink)| *ink == decade),
                "vertical {vertical}: the decades keep the full grid ink"
            );
            let (low, high) = (f64::log10(min), f64::log10(max));
            for exponent in (low as i32)..(high as i32) {
                for mantissa in 2..=9 {
                    let value = f64::from(mantissa) * 10f64.powi(exponent);
                    let fraction = ((value.log10() - low) / (high - low)) as f32;
                    let at = if vertical {
                        plot.left() + fraction * plot.width()
                    } else {
                        plot.bottom() - fraction * plot.height()
                    };
                    assert!(
                        minors.iter().any(|line| (line - at).abs() <= 0.5),
                        "vertical {vertical}: no line at {value} ({at})"
                    );
                }
            }
        }
    }

    /// A first-order low-pass on log-log axes, both canvases, for review.
    /// Set `RSPICE_REVIEW_DIR` to choose where.
    #[test]
    #[ignore = "writes PNGs for review"]
    fn write_log_grid_pngs() {
        let directory = std::env::var_os("RSPICE_REVIEW_DIR")
            .map_or_else(std::env::temp_dir, std::path::PathBuf::from);
        let x: Vec<f64> = (0..=400)
            .map(|step| 10f64.powf(1.0 + f64::from(step) / 100.0))
            .collect();
        let y: Vec<f64> = x
            .iter()
            .map(|f| 1.0 / (1.0 + (f / 1.0e3).powi(2)).sqrt())
            .collect();
        for (canvas_theme, name) in [
            (crate::EngineeringCanvasTheme::Dark, "log-grid-dark.png"),
            (crate::EngineeringCanvasTheme::Light, "log-grid-light.png"),
        ] {
            let theme = crate::Theme {
                canvas_theme,
                mode: if canvas_theme == crate::EngineeringCanvasTheme::Dark {
                    crate::Mode::Dark
                } else {
                    crate::Mode::Light
                },
                ..crate::Theme::default()
            };
            let canvas = crate::raster::render_themed(theme, vec2(720.0, 420.0), |ui, _| {
                let well = ui.available_rect_before_wrap();
                ui.painter()
                    .rect_filled(well, 0.0, Tokens::get(ui.ctx()).color.canvas_bg);
                let mut spec = PlotSpec::new(
                    Axis::log_decades(10.0, 1.0e5, "Hz"),
                    XScale::Log10,
                    Axis::log_decades(1.0e-3, 1.0, "V"),
                )
                .with_log_y();
                spec.traces
                    .push(Trace::new(&x, &y, Tokens::get(ui.ctx()).color.traces[0]));
                let mut cache = DecimationCache::default();
                show(ui, &spec, &mut cache, None, None);
            });
            let path = directory.join(name);
            std::fs::write(&path, canvas.png(canvas.height())).expect("write render");
            println!("{}", path.display());
        }
    }

    fn degenerate_spec<'a>(x: &'a [f64], y: &'a [f64], min: f64, max: f64) -> PlotSpec<'a> {
        let mut spec = PlotSpec::new(
            Axis::linear(min, max, "s"),
            XScale::Linear,
            Axis::linear(min, max, "V"),
        );
        spec.traces
            .push(Trace::new(x, y, egui::Color32::WHITE).cache_key(1));
        spec
    }

    /// A view that has collapsed to a point cannot draw, but it must still
    /// listen: the only way out of a stuck viewport is a gesture, and the
    /// early return swallowed every one of them.
    #[test]
    fn a_degenerate_view_still_accepts_the_gesture_that_would_rescue_it() {
        let x = [1.0, 1.0];
        let y = [2.0, 2.0];
        let spec = degenerate_spec(&x, &y, 1.0, 1.0);
        let size = vec2(400.0, 240.0);
        let inside = inner_rect(Rect::from_min_size(pos2(0.0, 0.0), size), &spec).center();
        let (_, view) = plot_frames(
            &spec,
            size,
            &[
                vec![egui::Event::PointerMoved(inside)],
                double_click(inside),
            ],
        );

        assert!(view.reset, "a collapsed view ignored double-click to fit");
    }

    /// Degenerate data must paint quietly rather than panic: zero span, one
    /// sample, and a wholly undefined trace all reach the painter in practice.
    #[test]
    fn degenerate_views_paint_without_panicking() {
        let nan = [f64::NAN; 8];
        let single = [1.0];
        let cases: Vec<(Vec<f64>, Vec<f64>, f64, f64)> = vec![
            (vec![1.0, 1.0], vec![2.0, 2.0], 1.0, 1.0),
            (single.to_vec(), vec![0.5], 0.0, 1.0),
            (nan.to_vec(), nan.to_vec(), 0.0, 1.0),
            (vec![0.0, 1.0], vec![f64::NAN, f64::INFINITY], 0.0, 1.0),
        ];
        for (x, y, min, max) in cases {
            let spec = degenerate_spec(&x, &y, min, max);
            let (shapes, _) = plot_frames(&spec, vec2(320.0, 200.0), &[Vec::new()]);
            assert!(
                shapes
                    .iter()
                    .all(|clipped| clipped.shape.visual_bounding_rect().is_finite()
                        || clipped.shape.visual_bounding_rect().is_negative())
            );
        }
    }

    /// Tick labels are chrome, and chrome that overlaps is unreadable. The X
    /// row already skipped colliding labels; the Y column never did, so a
    /// short pane stacked them on top of one another.
    #[test]
    fn y_tick_labels_never_overlap_however_short_the_pane_gets() {
        let x = [0.0, 1.0];
        let y = [0.0, 1000.0];
        let mut spec = PlotSpec::new(
            Axis::linear(0.0, 1.0, "s"),
            XScale::Linear,
            Axis::linear_with(0.0, 1000.0, "V", 40),
        );
        spec.traces.push(Trace::new(&x, &y, egui::Color32::WHITE));
        let size = vec2(420.0, 96.0);
        let plot = inner_rect(Rect::from_min_size(pos2(0.0, 0.0), size), &spec);
        let (shapes, _) = plot_frames(&spec, size, &[Vec::new()]);

        let mut rows: Vec<Rect> = painted_text(&shapes)
            .into_iter()
            .filter(|(_, at)| at.right() <= plot.left())
            .map(|(_, at)| at)
            .collect();
        rows.sort_by(|a, b| a.top().total_cmp(&b.top()));
        for pair in rows.windows(2) {
            assert!(
                pair[1].top() >= pair[0].bottom(),
                "y labels overlap: {:?} then {:?} ({} labels)",
                pair[0],
                pair[1],
                rows.len()
            );
        }
    }

    /// A band is a statement about a region of the data. Painted outside the
    /// plot area it is a statement about the tick gutter.
    #[test]
    fn a_band_outside_the_window_never_paints_over_the_axis_gutter() {
        let x = [0.0, 1.0];
        let y = [0.0, 1.0];
        let mut spec = PlotSpec::new(
            Axis::linear(0.0, 1.0, "s"),
            XScale::Linear,
            Axis::linear(0.0, 1.0, "V"),
        );
        spec.traces.push(Trace::new(&x, &y, egui::Color32::WHITE));
        spec.bands
            .push(super::super::spec::Band { x0: -4.0, x1: -2.0 });
        spec.bands
            .push(super::super::spec::Band { x0: 0.4, x1: 3.0 });
        let size = vec2(420.0, 260.0);
        let plot = inner_rect(Rect::from_min_size(pos2(0.0, 0.0), size), &spec);
        let (shapes, _) = plot_frames(&spec, size, &[Vec::new()]);

        for fill in painted_fills(&shapes) {
            assert!(
                plot.expand(1.0).contains_rect(fill),
                "a band painted {fill:?} outside the plot area {plot:?}"
            );
        }
    }

    /// One Shift-drag, run in both modes. It has to zoom where zooming is the
    /// contract and do nothing where preserving the view is: Select was
    /// arming the zoom box off the same modifier, so a key held for a
    /// selection gesture rescaled the plot underneath it.
    #[test]
    fn shift_drag_zooms_where_the_mode_allows_it_and_nowhere_else() {
        let x = [0.0, 1.0];
        let y = [0.0, 1.0];
        let mut spec = PlotSpec::new(
            Axis::linear(0.0, 1.0, "s"),
            XScale::Linear,
            Axis::linear(0.0, 1.0, "V"),
        );
        spec.traces.push(Trace::new(&x, &y, egui::Color32::WHITE));
        let size = vec2(420.0, 260.0);
        let plot = inner_rect(Rect::from_min_size(pos2(0.0, 0.0), size), &spec);
        let (from, to) = (
            plot.left_top() + vec2(20.0, 20.0),
            plot.left_top() + vec2(180.0, 140.0),
        );
        let shift = egui::Modifiers {
            shift: true,
            ..Default::default()
        };
        // The box anchors where the drag threshold is crossed, so the press
        // needs a frame of travel past that threshold of its own before the
        // sweep to the far corner.
        let drag_frames = |modifiers: egui::Modifiers| -> Vec<Vec<egui::Event>> {
            vec![
                vec![egui::Event::PointerMoved(from)],
                vec![
                    egui::Event::PointerMoved(from),
                    egui::Event::PointerButton {
                        pos: from,
                        button: egui::PointerButton::Primary,
                        pressed: true,
                        modifiers,
                    },
                ],
                vec![egui::Event::PointerMoved(from + vec2(30.0, 20.0))],
                vec![egui::Event::PointerMoved(to)],
                vec![
                    egui::Event::PointerMoved(to),
                    egui::Event::PointerButton {
                        pos: to,
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        modifiers,
                    },
                ],
            ]
        };

        let outcome = |mode: InteractionMode| {
            let ctx = egui::Context::default();
            crate::Theme::default().apply(&ctx);
            let mut cache = DecimationCache::default();
            let mut view = ViewChange::default();
            for events in drag_frames(shift) {
                set_interaction_mode(&ctx, mode);
                let input = egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), size)),
                    events,
                    modifiers: shift,
                    ..Default::default()
                };
                let _ = ctx.run_ui(input, |ui| {
                    let out = show(ui, &spec, &mut cache, None, None);
                    if out.view.any() {
                        view = out.view;
                    }
                });
            }
            view
        };

        let zoomed = outcome(InteractionMode::All);
        assert!(
            zoomed.x.is_some() && zoomed.y.is_some(),
            "the gesture never reached the zoom box: {zoomed:?}"
        );
        let selected = outcome(InteractionMode::Select);
        assert!(
            !selected.any(),
            "Select mode changed the view: {selected:?}"
        );
    }

    /// One trace's slots on a pane of its own, starting at its left edge.
    fn slots(plot: Rect) -> MarkerSlots {
        MarkerSlots {
            origin: plot.left(),
            pitch: MARKER_PITCH,
        }
    }

    /// A locus's markers follow the curve, not the abscissa. On a locus an X
    /// walk stalled at the first turn and stamped one point over and over.
    #[test]
    fn trace_markers_walk_a_locus_rather_than_its_abscissa() {
        let plot = Rect::from_min_size(pos2(0.0, 0.0), vec2(600.0, 400.0));
        let circle: Vec<Pos2> = (0..=720)
            .map(|step: u16| {
                let angle = f32::from(step) * std::f32::consts::TAU / 720.0;
                plot.center() + vec2(angle.cos(), angle.sin()) * 150.0
            })
            .collect();
        let placed = trace_marker_positions(&circle, plot, slots(plot));

        assert!(placed.len() >= 4, "{} markers on a locus", placed.len());
        for pair in placed.windows(2) {
            assert!(
                (pair[1] - pair[0]).length() > 24.0,
                "markers stacked at {:?}",
                pair[0]
            );
        }
        assert!(placed.iter().all(|point| plot.contains(*point)));

        // An ordinary left-to-right trace stands its markers one pitch apart.
        let ramp: Vec<Pos2> = (0..=600u16)
            .map(|step| pos2(f32::from(step), 200.0 + f32::from(step) * 0.1))
            .collect();
        let placed = trace_marker_positions(&ramp, plot, slots(plot));
        assert!(placed.len() >= 4 && placed.len() <= 8, "{placed:?}");
        assert!(
            placed
                .windows(2)
                .all(|pair| (pair[1].x - pair[0].x - MARKER_PITCH).abs() < 1.0e-3),
            "{placed:?}"
        );
    }

    /// A step trace whose transitions each span three pixel columns carries
    /// no marker on any of them, wherever the grid falls: every marker stands
    /// on a tread, at a level the trace held, with its glyph clear of both
    /// risers, and within reach of its slot. The guard this replaced only
    /// knew a riser narrower than one column, so a marker stood partway up
    /// every wider one the walk happened to stop on.
    #[test]
    fn markers_stay_off_transitions_three_columns_wide() {
        let plot = Rect::from_min_size(pos2(0.0, 0.0), vec2(960.0, 300.0));
        let levels = [
            250.0f32, 60.0, 180.0, 40.0, 220.0, 100.0, 260.0, 80.0, 200.0, 120.0, 240.0, 50.0,
        ];
        let (tread, riser) = (77.0f32, 3.0f32);
        let mut points = Vec::new();
        let mut risers = Vec::new();
        for (index, level) in levels.iter().enumerate() {
            let start = index as f32 * (tread + riser);
            points.extend([pos2(start, *level), pos2(start + tread, *level)]);
            if index + 1 < levels.len() {
                risers.push((start + tread, start + tread + riser));
            }
        }
        for phase in 0..80u8 {
            let slots = MarkerSlots {
                origin: f32::from(phase),
                pitch: MARKER_PITCH,
            };
            let placed = trace_marker_positions(&points, plot, slots);
            assert!(placed.len() >= 6, "phase {phase}: {placed:?}");
            for marker in &placed {
                assert!(
                    levels.iter().any(|level| (marker.y - level).abs() < 1.0e-3),
                    "phase {phase}: a marker at {marker:?} stands between levels"
                );
                assert!(
                    risers.iter().all(|(start, end)| {
                        marker.x + MARKER_CLEARANCE <= *start || marker.x - MARKER_CLEARANCE >= *end
                    }),
                    "phase {phase}: a marker at {marker:?} covers a riser"
                );
                let from_slot = (marker.x - slots.origin).rem_euclid(MARKER_PITCH);
                assert!(
                    from_slot.min(MARKER_PITCH - from_slot) <= f32::from(MARKER_REACH),
                    "phase {phase}: a marker at {marker:?} left its slot"
                );
            }
        }
    }

    /// A clock zoomed out until its half-periods are narrower than a glyph
    /// still carries its markers, each at the middle of a high or low tread,
    /// never on an edge. Clearing the whole glyph alone would have left the
    /// curve with none while its legend swatch showed one.
    #[test]
    fn a_clock_narrower_than_a_glyph_carries_markers_mid_tread() {
        let plot = Rect::from_min_size(pos2(0.0, 0.0), vec2(960.0, 300.0));
        let (tread, edge, high, low) = (5.0f32, 1.0f32, 60.0f32, 240.0f32);
        let mut points = Vec::new();
        let mut edges = Vec::new();
        let mut start = 0.0f32;
        let mut level = high;
        while start < plot.width() {
            points.extend([pos2(start, level), pos2(start + tread, level)]);
            edges.push((start + tread, start + tread + edge));
            start += tread + edge;
            level = if level == high { low } else { high };
        }
        for phase in 0..12u8 {
            let slots = MarkerSlots {
                origin: f32::from(phase) * 0.5,
                pitch: MARKER_PITCH,
            };
            let placed = trace_marker_positions(&points, plot, slots);
            assert!(placed.len() >= 8, "phase {phase}: {placed:?}");
            for marker in &placed {
                assert!(
                    marker.y == high || marker.y == low,
                    "phase {phase}: a marker at {marker:?} is off both levels"
                );
                assert!(
                    edges.iter().all(|(start, end)| {
                        marker.x + MARKER_CORE <= *start || marker.x - MARKER_CORE >= *end
                    }),
                    "phase {phase}: a marker at {marker:?} stands on an edge"
                );
            }
        }
    }

    /// Two marked curves on one pane stand their markers on one X grid, the
    /// second half a pitch after the first, so neither curve's symbols stack
    /// on the other's and each keeps one rhythm. Read from the frame.
    #[test]
    fn two_marked_curves_share_one_staggered_grid() {
        let x = [0.0, 10.0];
        let (low, high) = ([1.0, 1.0], [3.0, 3.0]);
        let mut spec = PlotSpec::new(
            Axis::linear(0.0, 10.0, "s"),
            XScale::Linear,
            Axis::linear(0.0, 4.0, "V"),
        );
        spec.traces
            .push(Trace::new(&x, &low, Color32::RED).marker_style(0));
        spec.traces
            .push(Trace::new(&x, &high, Color32::GREEN).marker_style(1));
        let (shapes, _) = plot_frames(&spec, vec2(960.0, 360.0), &[Vec::new()]);
        let mut circles = Vec::new();
        let mut squares = Vec::new();
        for clipped in &shapes {
            match &clipped.shape {
                Shape::Circle(circle) if circle.stroke.color == Color32::RED => {
                    circles.push(circle.center.x);
                }
                Shape::Rect(rect) if rect.stroke.color == Color32::GREEN => {
                    squares.push(rect.rect.center().x);
                }
                _ => {}
            }
        }
        assert!(
            circles.len() >= 6 && squares.len() >= 6,
            "{circles:?} {squares:?}"
        );
        let pitch = |marks: &[f32]| {
            marks
                .windows(2)
                .all(|pair| (pair[1] - pair[0] - MARKER_PITCH).abs() < 1.0e-3)
        };
        assert!(pitch(&circles), "{circles:?}");
        assert!(pitch(&squares), "{squares:?}");
        for square in &squares {
            let after = (square - circles[0]).rem_euclid(MARKER_PITCH);
            assert!(
                (after - MARKER_PITCH / 2.0).abs() < 1.0e-3,
                "a square at {square} is {after} after the circles' grid"
            );
        }
    }

    /// The grid is pinned to the data, so a pan carries the markers with
    /// their curves: moving the view ten columns moves every slot ten.
    #[test]
    fn a_pan_carries_the_marker_grid_with_the_data() {
        let plot = Rect::from_min_size(pos2(40.0, 0.0), vec2(960.0, 300.0));
        let origin = |min: f64| {
            let spec = PlotSpec::new(
                Axis::linear(min, min + 9.6, "s"),
                XScale::Linear,
                Axis::linear(0.0, 1.0, "V"),
            );
            MarkerSlots::on_pane(&spec, plot, 0, 1).origin
        };
        let moved = (origin(0.0) - origin(0.1)).rem_euclid(MARKER_PITCH);
        assert!(
            (moved - 10.0).abs() < 1.0e-2,
            "the slots moved {moved} for a ten-column pan"
        );
    }

    /// A staircase's markers stand on its treads, never partway up a riser.
    /// Read from the frame: every marker a step trace paints sits at one of
    /// the levels the trace held. Before, the walk along the curve stopped
    /// wherever its spacing ran out, and a marker stood at 101 on a 128 → 64
    /// step.
    #[test]
    fn a_staircase_carries_its_markers_on_its_treads() {
        let codes = [
            128.0, 64.0, 96.0, 32.0, 160.0, 8.0, 200.0, 72.0, 136.0, 24.0,
        ];
        let (mut x, mut y) = (Vec::new(), Vec::new());
        for (step, code) in codes.iter().enumerate() {
            // Each code holds for one microsecond: two samples, one at each
            // end of its tread, so the risers are vertical.
            x.extend([step as f64, step as f64 + 1.0]);
            y.extend([*code, *code]);
        }
        let mut spec = PlotSpec::new(
            Axis::linear(0.0, codes.len() as f64, "µs"),
            XScale::Linear,
            Axis::linear(0.0, 256.0, ""),
        );
        spec.traces
            .push(Trace::new(&x, &y, Color32::WHITE).marker_style(0));
        let size = vec2(720.0, 400.0);
        let plot = inner_rect(Rect::from_min_size(pos2(0.0, 0.0), size), &spec);
        let (shapes, _) = plot_frames(&spec, size, &[Vec::new()]);
        let tread = |code: f64| plot.bottom() - (code / 256.0) as f32 * plot.height();
        let markers: Vec<Pos2> = shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                Shape::Circle(circle) if circle.stroke.color == Color32::WHITE => {
                    Some(circle.center)
                }
                _ => None,
            })
            .collect();
        assert!(markers.len() >= 4, "{} markers", markers.len());
        for marker in &markers {
            assert!(
                codes
                    .iter()
                    .any(|code| (marker.y - tread(*code)).abs() <= 0.5),
                "a marker at {marker:?} stands between treads"
            );
        }
    }

    /// The ten-code staircase of the test above, with its markers, on the
    /// dark canvas, for review. Set `RSPICE_REVIEW_DIR` to choose where.
    #[test]
    #[ignore = "writes a PNG for review"]
    fn write_staircase_marker_png() {
        let directory = std::env::var_os("RSPICE_REVIEW_DIR")
            .map_or_else(std::env::temp_dir, std::path::PathBuf::from);
        let codes = [
            128.0, 64.0, 96.0, 32.0, 160.0, 8.0, 200.0, 72.0, 136.0, 24.0,
        ];
        let (mut x, mut y) = (Vec::new(), Vec::new());
        for (step, code) in codes.iter().enumerate() {
            x.extend([step as f64, step as f64 + 1.0]);
            y.extend([*code, *code]);
        }
        let theme = crate::Theme {
            canvas_theme: crate::EngineeringCanvasTheme::Dark,
            mode: crate::Mode::Dark,
            ..crate::Theme::default()
        };
        let canvas = crate::raster::render_themed(theme, vec2(720.0, 400.0), |ui, _| {
            let well = ui.available_rect_before_wrap();
            ui.painter()
                .rect_filled(well, 0.0, Tokens::get(ui.ctx()).color.canvas_bg);
            let mut spec = PlotSpec::new(
                Axis::linear(0.0, codes.len() as f64, "µs"),
                XScale::Linear,
                Axis::linear(0.0, 256.0, ""),
            );
            spec.traces
                .push(Trace::new(&x, &y, Tokens::get(ui.ctx()).color.traces[0]).marker_style(0));
            let mut cache = DecimationCache::default();
            show(ui, &spec, &mut cache, None, None);
        });
        let path = directory.join("staircase-markers.png");
        std::fs::write(&path, canvas.png(canvas.height())).expect("write render");
        println!("{}", path.display());
    }

    /// A trace with no drawable length carries no markers.
    ///
    /// The length guard reads `total <= 0.0` where it once read `!(total >
    /// 0.0)`. The two agree here because `total` sums only the finite pairs,
    /// so every term is a non-negative real and the sum is never NaN — a
    /// non-finite sample cannot reach the arithmetic to make one. This pins
    /// both ends of that argument: the degenerate cases still place nothing,
    /// and a curve whose only length lies between two non-finite neighbours
    /// is one of them.
    #[test]
    fn a_trace_with_no_drawable_length_places_no_markers() {
        let plot = Rect::from_min_size(pos2(0.0, 0.0), vec2(600.0, 400.0));

        // Every sample on one spot: the walk has nowhere to advance.
        let stacked = vec![plot.center(); 64];
        assert!(trace_marker_positions(&stacked, plot, slots(plot)).is_empty());

        // Every sample undefined: nothing survives the finite filter, so the
        // sum is the empty sum rather than a NaN.
        let undefined = vec![pos2(f32::NAN, f32::NAN); 64];
        assert!(trace_marker_positions(&undefined, plot, slots(plot)).is_empty());

        // Real positions, but every segment straddles a hole, so no pair
        // contributes and the total is still zero.
        let straddled: Vec<Pos2> = (0..64u16)
            .map(|index| {
                if index % 2 == 0 {
                    pos2(f32::from(index) * 8.0, 200.0)
                } else {
                    pos2(f32::INFINITY, 200.0)
                }
            })
            .collect();
        assert!(trace_marker_positions(&straddled, plot, slots(plot)).is_empty());

        // The guard has not swallowed the ordinary case.
        let ramp: Vec<Pos2> = (0..=600u16)
            .map(|step| pos2(f32::from(step), 200.0))
            .collect();
        assert!(!trace_marker_positions(&ramp, plot, slots(plot)).is_empty());
    }

    /// No gesture may store a view a later gesture cannot undo.
    #[test]
    fn a_stored_view_span_is_never_degenerate() {
        for scale in [XScale::Linear, XScale::Log10] {
            for range in [(1.0, 1.0), (1.0e-6, 1.0e-6), (5.0, 5.0)] {
                let (low, high) = floored_span(range, scale).expect("a usable span");
                assert!(high > low, "{scale:?} {range:?} stored {low}..{high}");
            }
        }
        let (low, high) = floored_span((0.0, 0.0), XScale::Linear).expect("a usable span");
        assert!(high > low);
        assert_eq!(floored_span((-1.0, 2.0), XScale::Log10), None);
        assert_eq!(floored_span((f64::NAN, 1.0), XScale::Linear), None);
        // An ordinary range passes through untouched.
        assert_eq!(floored_span((0.0, 1.0), XScale::Linear), Some((0.0, 1.0)));
    }

    #[test]
    fn plot_publishes_graphics_document_role_and_scene_summary() {
        let x = [0.0, 1.0];
        let y = [0.0, 1.0];
        let mut spec = PlotSpec::new(
            Axis::linear(0.0, 1.0, "s"),
            XScale::Linear,
            Axis::linear(0.0, 1.0, "V"),
        )
        .accessible_name("Transient waveform plot");
        spec.traces.push(Trace::new(&x, &y, egui::Color32::WHITE));
        let ctx = egui::Context::default();
        crate::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut cache = DecimationCache::default();
                show(ui, &spec, &mut cache, None, None);
            });
        });
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;

        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::GraphicsDocument
                && node
                    .label()
                    .is_some_and(|label| label.starts_with("Transient waveform plot. 1 visible"))
        }));
    }
}
