//! The plot painter — turns a [`PlotSpec`] into egui shapes.

use egui::{
    Align2, Color32, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, WidgetInfo, WidgetType, pos2,
    vec2,
};

use crate::theme::{self, FontWeight};
use crate::tokens::Tokens;

use super::cursor::{CursorId, CursorPair};
use super::decimate::DecimationCache;
use super::format::{fmt_reading, fmt_si_signed, fmt_signed_reading, tick_with_unit};
use super::scale::{LabelColumn, XScale, even_labels};
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
    /// Data-space X and left-axis Y of a right click inside the plot area,
    /// this frame: where the plot's context menu acts.
    pub context_at: Option<(f64, f64)>,
    /// A horizontal cursor a click or drag placed or moved: its slot (0 is
    /// H1, 1 is H2) and its new left-axis value.
    pub horizontal_cursor: Option<(usize, f64)>,
    /// A vertical cursor dragged by its line this frame, and where to.
    pub cursor_drag: Option<(CursorId, f64)>,
    /// The inner plot rectangle (inside margins), for overlays.
    pub plot_rect: Rect,
    /// Zoom/pan/fit gesture result. The wheel zooms X about the pointer,
    /// Ctrl+wheel Y, Ctrl+Shift+wheel both, Shift+wheel pans X, and a
    /// trackpad pinch zooms X (both axes on a chart that asks for it). Over
    /// the Y gutter the wheel zooms Y and a drag pans it. A drag pans, a
    /// Shift- or right-drag draws a zoom box whose shape picks its axes,
    /// double-click fits; with the plot focused the arrows pan and plus and
    /// minus zoom.
    pub view: ViewChange,
    /// The X and Y intervals this frame actually drew, whether they came
    /// from a pinned viewport or from fitting the data. A surface offering
    /// explicit axis limits has to seed itself from what the reader can
    /// see, and only the renderer knows that for certain.
    pub axes: ((f64, f64), (f64, f64)),
    /// The unit each axis states its interval in, X then Y, for a field that
    /// writes the drawn interval as the ticks read it: empty where the axis
    /// states none, or where its ticks label a transformed value rather than
    /// the data-space one `axes` holds.
    pub units: (String, String),
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
    on_whole_rows(
        inner_rect(ui.available_rect_before_wrap(), spec, left_margin(ui, spec)),
        ui.pixels_per_point(),
    )
}

/// The unit `axis` states a data-space value in: its own, unless its ticks
/// label a transformed value.
fn data_space_unit(axis: &Axis) -> String {
    if axis.display_scale == 1.0 && axis.display_offset == 0.0 {
        axis.unit.clone()
    } else {
        String::new()
    }
}

/// `plot` with its bottom edge on a pixel boundary at or above where it
/// was, so the frame's bottom line falls on the plot's own last row and
/// whatever stands under the plot (the X row's rule, the next strip) starts
/// on the row after it, crisp, however the layout divided the height.
fn on_whole_rows(mut plot: Rect, pixels_per_point: f32) -> Rect {
    plot.max.y = ((plot.max.y * pixels_per_point).floor() / pixels_per_point).max(plot.min.y);
    plot
}

/// What a chart read on its own grid (a Smith chart, a polar disc) keeps
/// above and below it. A framed plot keeps nothing: its frame stands on its
/// rect's edges and the fit's headroom keeps a curve off the frame.
const MARGIN_TOP: f32 = 12.0;
/// Where the Y caption's middle sits below the well's top.
const Y_CAPTION_CENTER: f32 = 8.0;
/// How far a drag must travel along an axis before it pans that axis.
const PAN_DEAD_ZONE: f32 = 6.0;
/// The size of the tick labels' font.
const Y_TICK_FONT_SIZE: f32 = 10.0;
/// How far a Y tick label stands from the axis line it labels.
pub const Y_LABEL_GAP: f32 = 7.0;
/// The least gap between two Y tick labels that are both drawn.
const Y_TICK_LABEL_GAP: f32 = 2.0;
const MARGIN_RIGHT_PLAIN: f32 = 16.0;
/// The least space between the widest Y label and the plot's left edge.
const GUTTER_EDGE: f32 = 2.0;

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
        TraceLook {
            dash: self.chosen_dash.unwrap_or(dash),
            marker: self.chosen_marker.unwrap_or(marker),
        }
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
    let mut shapes = Vec::new();
    trace_marker_shapes(center, shape, color, background, &mut shapes);
    painter.extend(shapes);
}

/// The shapes a curve's marker of `shape` at `center` is drawn as.
pub(super) fn trace_marker_shapes(
    center: Pos2,
    shape: TraceMarkerShape,
    color: Color32,
    background: Color32,
    out: &mut Vec<Shape>,
) {
    let radius = 3.4;
    let stroke = Stroke::new(1.35, color);
    match shape {
        TraceMarkerShape::Circle => {
            out.push(Shape::Circle(egui::epaint::CircleShape {
                center,
                radius,
                fill: background,
                stroke,
            }));
        }
        TraceMarkerShape::Square => {
            out.push(Shape::Rect(egui::epaint::RectShape::new(
                Rect::from_center_size(center, vec2(radius * 2.0, radius * 2.0)),
                0.0,
                background,
                stroke,
                egui::StrokeKind::Inside,
            )));
        }
        TraceMarkerShape::Diamond => {
            out.push(Shape::convex_polygon(
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
            out.push(Shape::convex_polygon(
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
            out.push(Shape::convex_polygon(
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
            trace_marker_shapes(center, TraceMarkerShape::Plus, color, background, out);
            trace_marker_shapes(center, TraceMarkerShape::Cross, color, background, out);
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
                out.push(Shape::line_segment(segment, Stroke::new(3.4, background)));
                out.push(Shape::line_segment(segment, stroke));
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
/// A hairline `width` points wide in whole device pixels: at least one and never a fraction, as
/// a browser draws a thin border, so a line snapped to the pixel grid stays sharp at 150 %.
/// The font a plot's Y tick labels and unit are set in, so an axis drawn
/// beside the plot reads as one of its own.
pub fn y_tick_font() -> egui::FontId {
    theme::mono(Y_TICK_FONT_SIZE, FontWeight::Regular)
}

/// A line `width` points wide, on whole device pixels: the plot's frame and
/// grid, and an axis drawn beside it in their ink.
pub fn hairline(width: f32, pixels_per_point: f32) -> f32 {
    (width * pixels_per_point).floor().max(1.0) / pixels_per_point
}

pub(super) fn finite_runs(points: &[Pos2]) -> impl Iterator<Item = &[Pos2]> {
    points
        .split(|point| !(point.x.is_finite() && point.y.is_finite()))
        .filter(|run| !run.is_empty())
}

/// Whether a run of points covers no more than a pixel either way: one
/// sample, or a dense column whose minimum and maximum coincide.
pub(super) fn run_is_a_point(run: &[Pos2]) -> bool {
    let first = run[0];
    run.iter()
        .all(|point| (point.x - first.x).abs() < 1.0 && (point.y - first.y).abs() < 1.0)
}

/// Whether a run never steps back along X by more than a pixel, as the
/// envelope of a curve against time or frequency does and a locus need not.
pub(super) fn runs_forward(run: &[Pos2]) -> bool {
    run.windows(2).all(|pair| pair[1].x >= pair[0].x - 1.0)
}

/// The stretches of a forward run that a dash of `dash` points with a gap of
/// `gap` points keeps, measured along X from the run's first point.
pub(super) fn column_dashes(run: &[Pos2], dash: f32, gap: f32) -> Vec<&[Pos2]> {
    let period = dash + gap;
    let origin = run[0].x;
    let mut stretches = Vec::new();
    let mut start = None;
    for (index, point) in run.iter().enumerate() {
        let drawn = (point.x - origin).rem_euclid(period) < dash;
        match (drawn, start) {
            (true, None) => start = Some(index),
            (false, Some(from)) => {
                stretches.push(&run[from..index]);
                start = None;
            }
            _ => {}
        }
    }
    if let Some(from) = start {
        stretches.push(&run[from..]);
    }
    stretches
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
pub(super) struct MarkerSlots {
    /// The screen X of one of this trace's slots; the rest lie whole
    /// pitches either side of it.
    pub(super) origin: f32,
    /// The distance between this trace's slots.
    pub(super) pitch: f32,
}

impl MarkerSlots {
    /// The slots of each of `traces`, in order, on a pane whose X axis is
    /// `x` (its least, its greatest and its scale): `None` for a trace that
    /// carries no marker.
    pub(super) fn for_pane(
        (x, traces): ((f64, f64, XScale), &[Trace<'_>]),
        plot_rect: Rect,
        color_safe: bool,
    ) -> Vec<Option<Self>> {
        let marked: Vec<bool> = traces
            .iter()
            .enumerate()
            .map(|(index, trace)| {
                !trace.is_empty() && trace.look(index, color_safe).marker.is_some()
            })
            .collect();
        let count = marked.iter().filter(|marked| **marked).count();
        let mut rank = 0;
        marked
            .into_iter()
            .map(|marked| {
                marked.then(|| {
                    rank += 1;
                    Self::on_pane(x, plot_rect, rank - 1, count)
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
    pub(super) fn on_pane(
        (x_min, x_max, x_scale): (f64, f64, XScale),
        plot_rect: Rect,
        rank: usize,
        marked: usize,
    ) -> Self {
        let marked = marked.max(1);
        let pitch = MARKER_PITCH.max(MARKER_STAGGER_MIN * marked as f32);
        let anchor = match x_scale {
            XScale::Linear => 0.0,
            XScale::Log10 => 1.0,
        };
        let anchor = x_scale.normalize(anchor, x_min, x_max) * f64::from(plot_rect.width());
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
pub(super) fn trace_marker_positions(
    points: &[Pos2],
    plot_rect: Rect,
    slots: MarkerSlots,
) -> Vec<Pos2> {
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

/// The plot's mark that the curves it shows are being drawn again: the
/// kit's busy mark and the words, in the plot's top right corner.
fn paint_updating(ui: &mut Ui, plot_rect: Rect, t: &Tokens) {
    const MARK: f32 = 12.0;
    const PAD: f32 = 4.0;
    let text = "Updating curves\u{2026}";
    let font = theme::sans(crate::tokens::FS_1, FontWeight::Regular);
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), font.clone(), t.color.text_dim);
    let size = vec2(
        MARK + PAD + galley.size().x + 2.0 * PAD,
        galley.size().y.max(MARK) + 2.0 * PAD,
    );
    let rect = Rect::from_min_size(
        pos2(plot_rect.right() - size.x - PAD, plot_rect.top() + PAD),
        size,
    );
    ui.painter().rect(
        rect,
        3.0,
        t.color.canvas_bg,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    let mut row = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect.shrink(PAD))
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    row.spacing_mut().item_spacing.x = PAD;
    crate::widgets::busy_mark(&mut row, MARK);
    row.add(
        egui::Label::new(egui::RichText::new(text).font(font).color(t.color.text_dim))
            .selectable(false),
    );
}

/// A framed plot runs to its rect's right edge unless it is told to keep a
/// gutter there (an axis standing beside it); the X row holds its last value
/// inside the plot rather than keeping room for half of it.
fn right_margin(spec: &PlotSpec<'_>) -> f32 {
    spec.right_margin
        .unwrap_or(if spec.frame { 0.0 } else { MARGIN_RIGHT_PLAIN })
}

fn bottom_margin(spec: &PlotSpec<'_>) -> f32 {
    if spec.x_axis_chrome {
        super::X_AXIS_ROW_HEIGHT
    } else if spec.frame {
        0.0
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
        .or_else(|| spec.y_without_unit.clone())
}

/// How a Y axis `length` points tall stands its tick labels: a label's
/// height and the gap the renderer keeps between neighbours.
pub fn y_label_column(painter: &egui::Painter, length: f32) -> LabelColumn {
    let label = painter
        .layout_no_wrap(
            "0".to_owned(),
            theme::mono(Y_TICK_FONT_SIZE, FontWeight::Regular),
            Color32::PLACEHOLDER,
        )
        .size()
        .y;
    LabelColumn {
        length: f64::from(length),
        label: f64::from(label),
        gap: f64::from(Y_TICK_LABEL_GAP),
    }
}

fn top_margin(spec: &PlotSpec<'_>) -> f32 {
    if spec.frame { 0.0 } else { MARGIN_TOP }
}

/// Whether the Y axis names a plain unit on its top tick label, as ViVA and
/// EZwave do, rather than at the head of its column: an anchor the ticks are
/// offsets from, or a caption for an axis without a unit, stands there
/// instead, and a chart read on its own grid keeps its own labelling.
fn unit_on_tick(spec: &PlotSpec<'_>) -> bool {
    spec.frame && spec.y.offset_anchor.is_none() && !spec.y.unit.is_empty()
}

/// The left gutter the plot draws with: the spec's, widened to its widest Y
/// label (the top one with its unit on it) or its column's caption, so a
/// long unit ("dBV rms") never clips or drops a label.
fn left_margin(ui: &Ui, spec: &PlotSpec<'_>) -> f32 {
    if !spec.frame {
        return spec.left_margin;
    }
    let font = theme::mono(Y_TICK_FONT_SIZE, FontWeight::Regular);
    let width = |text: String| {
        ui.painter()
            .layout_no_wrap(text, font.clone(), Color32::PLACEHOLDER)
            .size()
            .x
    };
    let top = spec
        .y
        .ticks
        .iter()
        .filter(|(value, _)| *value >= spec.y.min && *value <= spec.y.max)
        .max_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(value, _)| *value);
    let widest = spec
        .y
        .ticks
        .iter()
        .map(|(value, label)| {
            if unit_on_tick(spec) && Some(*value) == top {
                width(tick_with_unit(label, &spec.y.unit))
            } else {
                width(label.clone())
            }
        })
        .chain(
            y_caption_text(spec)
                .filter(|_| !unit_on_tick(spec))
                .map(width),
        )
        .fold(0.0, f32::max);
    spec.left_margin.max(widest + Y_LABEL_GAP + GUTTER_EDGE)
}

fn inner_rect(rect: Rect, spec: &PlotSpec<'_>, left: f32) -> Rect {
    Rect::from_min_max(
        pos2(rect.left() + left, rect.top() + top_margin(spec)),
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
    let trace_count = spec.traces.iter().filter(|trace| !trace.is_empty()).count()
        + spec.curves.as_ref().map_or(0, |curves| curves.count);
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

/// The ranges about the centres of `x` and `y` that put one data unit on
/// the same number of points along both axes of a plot area `plot` points
/// across: the axis with room to spare widens about its centre, and neither
/// narrows, so all that was asked for stays in view. A plane drawn this way
/// fills its rect and keeps a circle round, as MATLAB's `axis equal` with
/// adjustable limits does, where [`square_outer_rect`] would letterbox it.
///
/// A degenerate size or range is returned as it came.
#[must_use]
pub fn equal_scale_ranges(
    plot: egui::Vec2,
    x: (f64, f64),
    y: (f64, f64),
) -> ((f64, f64), (f64, f64)) {
    let (width, height) = (f64::from(plot.x), f64::from(plot.y));
    let (x_span, y_span) = ((x.1 - x.0).abs(), (y.1 - y.0).abs());
    if !(width > 0.0 && height > 0.0 && x_span.is_finite() && y_span.is_finite()) {
        return (x, y);
    }
    let per_point = (x_span / width).max(y_span / height);
    if per_point <= 0.0 {
        return (x, y);
    }
    let about = |(low, high): (f64, f64), points: f64| {
        let centre = (low + high) * 0.5;
        let half = per_point * points * 0.5;
        (centre - half, centre + half)
    };
    (about(x, width), about(y, height))
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
    let plot_rect = on_whole_rows(
        inner_rect(rect, spec, left_margin(ui, spec)),
        ui.pixels_per_point(),
    );

    let mut out = PlotResponse {
        response,
        clicked_x: None,
        context_at: None,
        horizontal_cursor: None,
        cursor_drag: None,
        plot_rect,
        view: ViewChange::default(),
        axes: ((spec.x.min, spec.x.max), (spec.y.min, spec.y.max)),
        units: (data_space_unit(&spec.x), data_space_unit(&spec.y)),
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
        handle_navigation(
            ui,
            spec,
            &painter,
            plot_rect,
            &mut out,
            &t,
            Navigation {
                cursors: cursors.copied(),
                cursor_grabbed: false,
            },
        );
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
    // Grid and reference lines are hairlines the painter centres on device pixels.
    let pixels_per_point = painter.pixels_per_point();
    let hair = hairline(1.0, pixels_per_point);
    let grid = Stroke::new(hair, c.canvas_grid);
    let frame = Stroke::new(hair, c.canvas_frame);

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
    // A linear Y ladder too dense for the plot's height to name every tick
    // coarsens until it can; ticks that still crowd (dense log decades at
    // deep zoom) are named at even strides from the top, and every gridline
    // still draws.
    let fitted_y = (spec.y_scale == XScale::Linear)
        .then(|| spec.y.fitted(y_label_column(&painter, plot_rect.height())))
        .flatten();
    let y_axis = fitted_y.as_ref().unwrap_or(&spec.y);
    // A logarithmic axis is read against its mantissas, 2 … 9 in each decade,
    // lighter than the decades, as ViVA and ADS rule one; a linear axis
    // subdivides only when the reader asks for a minor grid.
    let minor = Stroke::new(hair, c.canvas_grid.gamma_multiply(0.45));
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
    for value in minors(spec.y_scale, y_axis) {
        painter.hline(plot_rect.x_range(), my(value), minor);
    }
    // The Y column's chrome is either its unit or, when the window is too
    // narrow to label absolutely, the anchor its ticks are offsets from —
    // which carries the unit itself. Without a row of its own it owns the
    // top of the column, so tick labels give way to it; with one, the plot
    // starts below it and every label is drawn.
    let y_ink = spec.y_ink.unwrap_or(c.canvas_text_dim);
    let unit_on_tick = unit_on_tick(spec);
    let y_chrome = y_caption_text(spec)
        .filter(|_| !unit_on_tick)
        .map(|text| painter.layout_no_wrap(text, tick_font.clone(), y_ink));
    let y_chrome_bottom = y_chrome.as_ref().map_or(f32::NEG_INFINITY, |galley| {
        rect.top() + Y_CAPTION_CENTER + galley.size().y * 0.5
    });
    for (xv, _) in &spec.x.ticks {
        painter.vline(mx(*xv), plot_rect.y_range(), grid);
    }
    // Labels that still crowd are named at even strides from the top, the
    // one with the unit always among them: stacked labels are less readable
    // than none, and a hole in the middle of a scale reads as a missing
    // value.
    let mut y_labels = Vec::new();
    for (yv, label) in &y_axis.ticks {
        let py = my(*yv);
        painter.hline(plot_rect.x_range(), py, grid);
        let galley = painter.layout_no_wrap(label.clone(), tick_font.clone(), y_ink);
        let (height, mut top) = (galley.size().y, py - galley.size().y * 0.5);
        // A framed plot keeps no margin above or below its frame: the label
        // of a tick on its very edge is nudged inside it, so a short plot
        // keeps its scale and no label is clipped by what stands beside it.
        if spec.frame {
            top = top.clamp(
                plot_rect.top(),
                (plot_rect.bottom() - height).max(plot_rect.top()),
            );
        }
        if top >= y_chrome_bottom + Y_TICK_LABEL_GAP {
            y_labels.push((top, label, galley));
        }
    }
    let centres: Vec<f64> = y_labels
        .iter()
        .map(|(top, _, galley)| f64::from(plot_rect.bottom() - top - galley.size().y * 0.5))
        .collect();
    let pitch = y_labels
        .first()
        .map_or(0.0, |(_, _, galley)| galley.size().y + Y_TICK_LABEL_GAP);
    let mut named = even_labels(&centres, f64::from(pitch)).into_iter();
    y_labels.retain(|_| named.next().unwrap_or(false));
    let topmost = y_labels.len().checked_sub(1);
    for (index, (top, label, galley)) in y_labels.into_iter().enumerate() {
        let galley = if unit_on_tick && Some(index) == topmost {
            painter.layout_no_wrap(
                tick_with_unit(label, &spec.y.unit),
                tick_font.clone(),
                y_ink,
            )
        } else {
            galley
        };
        let at = pos2(plot_rect.left() - Y_LABEL_GAP - galley.size().x, top);
        if unit_on_tick && Some(index) == topmost {
            out.y_caption = Some(Rect::from_min_size(at, galley.size()));
        }
        painter.galley(at, galley, y_ink);
    }
    // After the grid: a grid line on the plot's bottom edge (a range that
    // starts at zero) falls on the row's rule, and the row stands over it.
    if spec.x_axis_chrome {
        let row = Rect::from_min_max(pos2(rect.left(), plot_rect.bottom()), rect.right_bottom());
        super::paint_x_axis_band(&painter, row, &t);
        out.x_label_rect = Some(super::paint_x_axis_row(
            &painter,
            row,
            super::XAxisRow {
                axis: &spec.x,
                scale: spec.x_scale,
                plot: plot_rect.x_range(),
                caption: &super::x_axis_caption(
                    spec.x.label.as_deref().unwrap_or(""),
                    &spec.x.unit,
                ),
                keep_clear: &[],
            },
            &t,
        ));
    }
    // ---- reference lines
    let ref_stroke = Stroke::new(hair, c.canvas_text_faint);
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
    let (x_low, x_high) = (spec.x.min.min(spec.x.max), spec.x.min.max(spec.x.max));
    for &x in &spec.x_ref_lines {
        if !(x >= x_low && x <= x_high) {
            continue;
        }
        let px = mx(x);
        inside.extend(Shape::dashed_line(
            &[pos2(px, plot_rect.top()), pos2(px, plot_rect.bottom())],
            ref_stroke,
            4.0,
            3.0,
        ));
    }
    // Where the limit labels sit, so a marker's tag keeps clear of them.
    let mut limit_labels: Vec<Rect> = Vec::new();
    for line in &spec.limit_lines {
        if !(line.y >= spec.y.min && line.y <= spec.y.max) {
            continue;
        }
        let py = my(line.y);
        inside.extend(Shape::dashed_line(
            &[pos2(plot_rect.left(), py), pos2(plot_rect.right(), py)],
            Stroke::new(hair, line.color),
            6.0,
            4.0,
        ));
        // Above its line, or below it when the line runs too near the top
        // for the words to stand clear: never across the dashes.
        let galley = painter.layout_no_wrap(
            line.label.clone(),
            theme::mono(9.0, FontWeight::Medium),
            line.color,
        );
        let size = galley.size();
        let top = if py - 3.0 - size.y >= plot_rect.top() + 1.0 {
            py - 3.0 - size.y
        } else {
            py + 3.0
        };
        let label = Rect::from_min_size(pos2(plot_rect.right() - 4.0 - size.x, top), size);
        inside.galley(label.min, galley, line.color);
        limit_labels.push(label);
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
    let updating = {
        let clipped = ui.painter_at(plot_rect.expand(1.0));
        // One column per device pixel, so a dense trace strokes every pixel at
        // any scaling.
        let columns = super::decimate::display_columns(plot_rect.width(), pixels_per_point);
        // A parametric reduction quantizes both axes, so it needs the vertical
        // resolution too, counted the same way.
        let rows = super::decimate::display_columns(plot_rect.height(), pixels_per_point);
        super::picture::draw(
            ui.ctx(),
            out.response.id,
            spec,
            cache,
            &clipped,
            super::picture::Geometry {
                plot_rect,
                pixels_per_point,
                x: (spec.x.min, spec.x.max, spec.x_scale),
                y: (spec.y.min, spec.y.max, spec.y_scale),
                columns,
                rows,
                background: c.canvas_bg,
            },
            t.color_safe_traces,
        )
    };
    if updating {
        paint_updating(ui, plot_rect, &t);
    }

    // ---- frame + axis units
    if spec.frame {
        painter.vline(plot_rect.left(), plot_rect.y_range(), frame);
        // On the plot's last row rather than the row under it: a stacked
        // strip's bottom line is the separator from the strip below, and an
        // X row's rule stands on that next row.
        painter.hline(plot_rect.x_range(), plot_rect.bottom() - 0.5, frame);
    }
    if let Some(galley) = y_chrome {
        let caption = Rect::from_min_size(
            pos2(
                plot_rect.left() - Y_LABEL_GAP - galley.size().x,
                rect.top() + Y_CAPTION_CENTER - galley.size().y * 0.5,
            ),
            galley.size(),
        );
        painter.galley(caption.min, galley, y_ink);
        out.y_caption = Some(caption);
    }

    // ---- markers
    let tag_font = theme::mono(9.5, FontWeight::Regular);
    // What a tag keeps clear of beside the curves: the limit labels, and
    // every tag already placed.
    let mut obstacles = limit_labels;
    // The curves a plot makes only for a new picture, made here once, and
    // only when a tag has to keep clear of them.
    let made = std::cell::OnceCell::new();
    let curves = || {
        made.get_or_init(|| {
            spec.curves
                .as_ref()
                .map_or_else(Vec::new, |curves| (curves.make)())
        })
        .as_slice()
    };
    for marker in &spec.markers {
        // A marker states where something is. Off the window there is no
        // "where" to draw, and a dot pinned to the frame would claim a
        // position the data does not have.
        if !(marker.x >= spec.x.min && marker.x <= spec.x.max) {
            continue;
        }
        let px = mx(marker.x);
        let limit_line = marker.shape == MarkerShape::LimitLine;
        let span = match marker.shape {
            MarkerShape::Span { to } => Some(to),
            _ => None,
        };
        let in_window = |y: f64| y >= spec.y.min && y <= spec.y.max;
        // A limit line spans the pane, so it is never clipped by Y; a span
        // is drawn as far as it stands in the window; anything else has to
        // land inside the window.
        let y_is_drawable = limit_line
            || span.map_or_else(
                || in_window(marker.y),
                |to| marker.y.max(to) >= spec.y.min && marker.y.min(to) <= spec.y.max,
            );
        if !y_is_drawable {
            continue;
        }
        // A limit line is a callout about the X position alone, so it spans
        // the plot and tags at the top; its `y` carries no meaning. A span
        // tags beside the middle of what the window shows of it.
        let clamp_y = |y: f64| my(y.clamp(spec.y.min, spec.y.max));
        let py = if limit_line {
            plot_rect.top()
        } else if let Some(to) = span {
            (clamp_y(marker.y) + clamp_y(to)) * 0.5
        } else {
            my(marker.y)
        };
        if let Some(to) = span {
            let stroke = Stroke::new(1.5, marker.color);
            inside.line_segment([pos2(px, clamp_y(marker.y)), pos2(px, clamp_y(to))], stroke);
            // Each end that stands in the window is capped, so the bar reads
            // as a measured distance rather than a line running on.
            for end in [marker.y, to].into_iter().filter(|&y| in_window(y)) {
                inside.line_segment([pos2(px - 4.0, my(end)), pos2(px + 4.0, my(end))], stroke);
            }
        } else if limit_line {
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
        } else if span.is_some() {
            py - tag_h * 0.5 + marker.label_dy
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
        let place = |tx: f32, ty: f32| {
            let tx = tx.clamp(
                plot_rect.left() + 2.0,
                (plot_rect.right() - tag_w - 2.0).max(plot_rect.left() + 2.0),
            );
            let ty = ty.clamp(
                plot_rect.top() + 2.0,
                (plot_rect.bottom() - tag_h - 2.0).max(plot_rect.top() + 2.0),
            );
            Rect::from_min_size(pos2(tx, ty), vec2(tag_w, tag_h))
        };
        let mut tag_rect = place(tx, ty);
        // A point's tag set on the curve it names, on a limit line, on a
        // limit's words or on another tag hides them: it takes the first
        // corner of the point that is clear, nearest first, and keeps its
        // usual place when none is.
        if !limit_line {
            let (right, left) = (px + 9.0, px - tag_w - 9.0);
            let lift = tag_h + 4.0;
            let clear =
                |rect: Rect| !tag_collides(rect, (spec, curves()), plot_rect, &mx, &my, &obstacles);
            if !clear(tag_rect)
                && let Some(rect) = [0.0, lift, 2.0 * lift]
                    .into_iter()
                    .flat_map(|farther| {
                        // A span's tag stays level with its bar, either side,
                        // before it moves up or down along it.
                        let (above, below) = if span.is_some() {
                            (py - tag_h * 0.5 - farther, py - tag_h * 0.5 + farther)
                        } else {
                            (py - tag_h - 7.0 - farther, py + 9.0 + farther)
                        };
                        [(right, above), (left, above), (right, below), (left, below)]
                    })
                    .map(|(x, y)| place(x, y))
                    .find(|rect| clear(*rect))
            {
                tag_rect = rect;
            }
        }
        obstacles.push(tag_rect);
        let (tx, ty) = (tag_rect.left(), tag_rect.top());
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

    // ---- A/B cursors: a line and a lettered flag where a cursor stands in
    // the window, and a chevron on the edge it left by where it does not.
    let flag_font = theme::mono(9.0, FontWeight::Medium);
    let mut cursor_lines: Vec<(CursorId, f32)> = Vec::new();
    let mut chevrons: Vec<(CursorId, f64, Rect, Color32)> = Vec::new();
    if let Some(pair) = cursors {
        for (id, color) in [
            (CursorId::A, c.canvas_cursor_a),
            (CursorId::B, c.canvas_cursor_b),
        ] {
            let Some(x) = pair.get(id).filter(|x| x.is_finite()) else {
                continue;
            };
            if x < spec.x.min || x > spec.x.max {
                let before = x < spec.x.min;
                let beside = chevrons
                    .iter()
                    .filter(|(_, other, ..)| (*other < spec.x.min) == before)
                    .count() as f32;
                let offset = beside * (CHEVRON_WIDTH + 2.0);
                let left = if before {
                    plot_rect.left() + offset
                } else {
                    plot_rect.right() - CHEVRON_WIDTH - offset
                };
                let chevron = Rect::from_min_size(
                    pos2(left, plot_rect.top() - 1.0),
                    vec2(CHEVRON_WIDTH, 13.0),
                );
                chevrons.push((id, x, chevron, color));
                continue;
            }
            let px = mx(x);
            let hair = hairline(1.0, painter.pixels_per_point());
            painter.vline(px, plot_rect.y_range(), Stroke::new(hair, color));
            let flag = Rect::from_min_size(pos2(px - 7.0, plot_rect.top() - 1.0), vec2(14.0, 13.0));
            painter.rect_filled(flag, 1.0, color);
            painter.text(
                flag.center(),
                Align2::CENTER_CENTER,
                id.letter(),
                flag_font.clone(),
                c.canvas_bg,
            );
            cursor_lines.push((id, px));
        }
    }
    for &(id, x, chevron, color) in &chevrons {
        let before = x < spec.x.min;
        painter.rect_filled(chevron, 1.0, color);
        painter.text(
            chevron.center(),
            Align2::CENTER_CENTER,
            if before {
                format!("\u{2039}{}", id.letter())
            } else {
                format!("{}\u{203a}", id.letter())
            },
            flag_font.clone(),
            c.canvas_bg,
        );
        let handle = ui.interact(
            chevron,
            out.response.id.with(("cursor-chevron", id.letter())),
            Sense::click(),
        );
        handle.widget_info(|| {
            WidgetInfo::labeled(
                WidgetType::Button,
                ui.is_enabled(),
                format!("Bring cursor {} into view", id.letter()),
            )
        });
        theme::paint_focus_ring(ui, &handle, chevron);
        if handle.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        // The window keeps its width and centres on the cursor.
        if handle.clicked() {
            let at = spec.x_scale.normalize(x, spec.x.min, spec.x.max);
            out.view.x = floored_span(
                (
                    spec.x_scale.denormalize(at - 0.5, spec.x.min, spec.x.max),
                    spec.x_scale.denormalize(at + 0.5, spec.x.min, spec.x.max),
                ),
                spec.x_scale,
            );
        }
        handle.on_hover_text(format!(
            "Cursor {} is {} the window: click to centre on it",
            id.letter(),
            if before { "before" } else { "after" }
        ));
    }

    // A cursor line is a handle along its whole height: a press within reach
    // of it moves that cursor, and the drag neither pans nor draws a box.
    let grab_id = out.response.id.with("plot.cursor-grab");
    let line_near = |x: f32| {
        cursor_lines
            .iter()
            .map(|&(id, px)| (id, (px - x).abs()))
            .filter(|&(_, distance)| distance <= CURSOR_GRAB)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(id, _)| id)
    };
    let shift_held = ui.input(|input| input.modifiers.shift);
    if out.response.drag_started_by(egui::PointerButton::Primary)
        && !shift_held
        && let Some(origin) = ui.input(|input| input.pointer.press_origin())
        && plot_rect.expand(2.0).contains(origin)
        && let Some(id) = line_near(origin.x)
    {
        ui.data_mut(|data| data.insert_temp(grab_id, id));
    }
    let grabbed = ui.data(|data| data.get_temp::<CursorId>(grab_id));
    if let Some(id) = grabbed {
        if out.response.dragged_by(egui::PointerButton::Primary)
            && let Some(pointer) = out.response.interact_pointer_pos()
        {
            let fraction = ((pointer.x.clamp(plot_rect.left(), plot_rect.right())
                - plot_rect.left())
                / plot_rect.width()) as f64;
            out.cursor_drag = Some((
                id,
                spec.x_scale.denormalize(fraction, spec.x.min, spec.x.max),
            ));
        }
        if !out.response.dragged() {
            ui.data_mut(|data| data.remove::<CursorId>(grab_id));
        }
        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
    } else if let Some(pointer) = out.response.hover_pos()
        && plot_rect.expand(2.0).contains(pointer)
        && line_near(pointer.x).is_some()
    {
        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
    }

    // ---- horizontal cursors: H1 in A's colour, H2 in B's, and the ΔY
    // between them dimensioned at the right edge when both are placed. Each
    // level is read in the axis' unit, as the readout strip reads it.
    let horizontal_colors = [c.canvas_cursor_a, c.canvas_cursor_b];
    let in_view = |y: f64| y.is_finite() && y >= spec.y.min && y <= spec.y.max;
    for (slot, y) in spec.horizontal_cursors.iter().enumerate() {
        let Some(y) = y.filter(|y| in_view(*y)) else {
            continue;
        };
        let py = my(y);
        let color = horizontal_colors[slot];
        let hair = hairline(1.0, painter.pixels_per_point());
        painter.hline(plot_rect.x_range(), py, Stroke::new(hair, color));
        let label = painter.layout_no_wrap(
            format!(
                "H{} {}",
                slot + 1,
                fmt_reading(
                    spec.y.display_value(y),
                    &spec.y.unit,
                    spec.significant_digits
                )
            ),
            theme::mono(9.0, FontWeight::Medium),
            color,
        );
        // Above its line, or under it where the plot's top would cut it.
        let height = label.size().y;
        let top = if py - 3.0 - height >= plot_rect.top() {
            py - 3.0 - height
        } else {
            py + 3.0
        };
        painter.galley(pos2(plot_rect.left() + 4.0, top), label, color);
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
            format!(
                "\u{0394}Y {}",
                if super::format::reads_at_fixed_places(&spec.y.unit) {
                    fmt_signed_reading(
                        (h2 - h1) * spec.y.display_scale,
                        &spec.y.unit,
                        spec.significant_digits,
                    )
                } else {
                    fmt_si_signed(
                        (h2 - h1) * spec.y.display_scale,
                        &spec.y.unit,
                        spec.significant_digits,
                    )
                }
            ),
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
        if out.response.secondary_clicked() {
            let fraction = ((plot_rect.bottom() - pointer.y) / plot_rect.height()).clamp(0.0, 1.0);
            out.context_at = Some((
                data_x,
                spec.y_scale
                    .denormalize(f64::from(fraction), spec.y.min, spec.y.max),
            ));
        }
        if spec.horizontal_cursor_interactive && grabbed.is_none() {
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

    // ---- navigation gestures: wheel, pinch, keys, drag pan, zoom box, fit
    handle_navigation(
        ui,
        spec,
        &painter,
        plot_rect,
        &mut out,
        &t,
        Navigation {
            cursors: cursors.copied(),
            cursor_grabbed: grabbed.is_some(),
        },
    );

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

/// A range wide enough to still be a view, or nothing. This is the one
/// floor every zoom in the results workspace stops at: a plot's wheel, box
/// and keys, and a ruler's wheel and handles.
///
/// Screen coordinates are `f32`, so a window narrower than about a billionth
/// of its own magnitude is already finer than the projection that draws it.
/// A window of exactly zero is worse than useless: every gesture states its
/// result as a fraction of the span it was handed, so a stored zero span is a
/// viewport nothing but a fit can ever widen again.
pub fn floored_span(range: (f64, f64), scale: XScale) -> Option<(f64, f64)> {
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

/// How near a cursor line, in points, a press takes hold of it.
const CURSOR_GRAB: f32 = 5.0;

/// The width of the chevron that stands for a cursor outside the window.
const CHEVRON_WIDTH: f32 = 20.0;

/// A zoom band thinner than this along one axis, in points, zooms the other
/// axis alone: a flat band is a stretch of X, a tall one a stretch of Y.
const ONE_AXIS_BAND: f32 = 12.0;

/// The least extent, in points, a band needs along an axis it zooms.
const LEAST_BAND: f32 = 6.0;

/// The share of the window one arrow key pans, and the factor plus zooms
/// the window by (minus by its reciprocal).
const KEY_PAN: f64 = 0.1;
const KEY_ZOOM_IN: f64 = 0.8;

/// How fast the wheel zooms: the window's span changes by `exp(-rate × d)`
/// for `d` points of wheel travel.
const WHEEL_ZOOM_RATE: f64 = 0.002;

/// The axes one zoom gesture changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ZoomAxes {
    X,
    Y,
    Both,
}

impl ZoomAxes {
    const fn x(self) -> bool {
        matches!(self, Self::X | Self::Both)
    }

    const fn y(self) -> bool {
        matches!(self, Self::Y | Self::Both)
    }
}

/// The axes a box zoom over `band` changes: the one the reader armed with X
/// or Y if any, else what the band's shape says, as EZwave reads a drag.
fn band_axes(band: Rect, armed: Option<ZoomAxes>) -> Option<ZoomAxes> {
    let (wide, tall) = (band.width(), band.height());
    match armed {
        Some(ZoomAxes::X) => (wide > LEAST_BAND).then_some(ZoomAxes::X),
        Some(ZoomAxes::Y) => (tall > LEAST_BAND).then_some(ZoomAxes::Y),
        Some(ZoomAxes::Both) | None => match (wide >= ONE_AXIS_BAND, tall >= ONE_AXIS_BAND) {
            (true, true) => Some(ZoomAxes::Both),
            (true, false) => Some(ZoomAxes::X),
            (false, true) => Some(ZoomAxes::Y),
            (false, false) => None,
        },
    }
}

/// Remember the axis the reader armed for the next zoom, or forget it.
fn keep_armed(ui: &Ui, id: egui::Id, armed: Option<ZoomAxes>) {
    ui.data_mut(|data| {
        if let Some(axis) = armed {
            data.insert_temp(id, axis);
        } else {
            data.remove::<ZoomAxes>(id);
        }
    });
}

/// What the plot needs besides its spec to read a gesture.
struct Navigation {
    cursors: Option<CursorPair>,
    /// A cursor line has the drag, which therefore is not a pan.
    cursor_grabbed: bool,
}

/// What the wheel and the trackpad did this frame, told apart.
///
/// egui folds a wheel turned with Ctrl held into the same zoom factor a
/// pinch arrives in, and reports no scroll for it, so a plot that read only
/// the scroll never saw Ctrl+wheel at all. A pinch reaches egui as a zoom
/// event (the browser turns a trackpad pinch, which it reports as a wheel
/// with a Ctrl it did not press, into one; macOS hands over its pinch
/// gesture) or as a two-finger touch. What the pinch does not explain of the
/// zoom factor is the wheel.
#[derive(Debug, Clone, Copy)]
struct WheelInput {
    /// The span factor a pinch asks for, 1 when there was none.
    pinch: f64,
    /// The span factor a Ctrl-held wheel asks for, at the rate the plain
    /// wheel zooms, 1 when there was none.
    ctrl_wheel: f64,
    /// Plain wheel travel, in points: Y from the wheel, X from a horizontal
    /// swipe or from the wheel with Shift held.
    scroll: egui::Vec2,
    shift: bool,
}

impl WheelInput {
    fn read(ui: &Ui) -> Self {
        let zoom_speed = ui
            .ctx()
            .options(|options| f64::from(options.input_options.scroll_zoom_speed));
        ui.input(|input| {
            let zoom_events: f32 = input
                .events
                .iter()
                .filter_map(|event| match event {
                    egui::Event::Zoom(factor) => Some(*factor),
                    _ => None,
                })
                .product();
            let (pinch, wheel) = match input.multi_touch() {
                Some(touch) => (touch.zoom_delta, 1.0),
                None => (zoom_events, input.zoom_delta() / zoom_events),
            };
            let span = |zoom: f32| f64::from(zoom).recip().clamp(0.05, 20.0);
            // Dividing the pinch back out leaves rounding behind; a wheel
            // that turned changes the factor by far more than that.
            let ctrl_wheel = if (wheel - 1.0).abs() > 1.0e-4 && zoom_speed > 0.0 {
                let travel = f64::from(wheel).ln() / zoom_speed;
                (-travel * WHEEL_ZOOM_RATE).exp().clamp(0.05, 20.0)
            } else {
                1.0
            };
            Self {
                pinch: span(pinch),
                ctrl_wheel,
                scroll: input.smooth_scroll_delta,
                shift: input.modifiers.shift,
            }
        })
    }
}

/// One axis' window as fractions of the span handed in: `(0, 1)` is the
/// window as it stands.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Window(f64, f64);

impl Window {
    const SAME: Self = Self(0.0, 1.0);

    /// Zoom by `factor` about the fraction `anchor`, which keeps its place.
    fn zoom(self, anchor: f64, factor: f64) -> Self {
        let at = self.0 + (self.1 - self.0) * anchor;
        Self(at + (self.0 - at) * factor, at + (self.1 - at) * factor)
    }

    /// Move by `delta` of the span handed in.
    fn pan(self, delta: f64) -> Self {
        Self(self.0 + delta, self.1 + delta)
    }
}

/// Interpret the frame's gestures against the spec's current ranges and
/// fill `out.view` with the resulting data-space ranges. The caller owns
/// the view state; the engine keeps only a gesture's own state between
/// frames (the zoom band's anchor, the armed axis) in egui memory keyed by
/// the widget id.
fn handle_navigation(
    ui: &mut Ui,
    spec: &PlotSpec<'_>,
    painter: &egui::Painter,
    plot_rect: Rect,
    out: &mut PlotResponse,
    t: &Tokens,
    navigation: Navigation,
) {
    let c = t.color;
    let interaction = interaction_mode(ui.ctx());
    let rect = out.response.rect;
    // The Y axis' gutter: the tick column beside the plot area.
    let gutter = Rect::from_min_max(
        pos2(rect.left(), plot_rect.top()),
        pos2(plot_rect.left(), plot_rect.bottom()),
    );

    // Fraction coordinates: fx in 0..1 left→right, fy in 0..1 bottom→top.
    let fx_of = |px: f32| ((px - plot_rect.left()) / plot_rect.width()) as f64;
    let fy_of = |py: f32| ((plot_rect.bottom() - py) / plot_rect.height()) as f64;
    let denorm_x = |frac: f64| spec.x_scale.denormalize(frac, spec.x.min, spec.x.max);
    let denorm_y = |frac: f64| spec.y_scale.denormalize(frac, spec.y.min, spec.y.max);
    // Every range this reports is about to become the next frame's view, so
    // none of them may be degenerate: a stored zero span is a viewport no
    // later gesture can widen, because every gesture is relative to it.
    let store_x = |w: Window| floored_span((denorm_x(w.0), denorm_x(w.1)), spec.x_scale);
    let store_y = |w: Window| floored_span((denorm_y(w.0), denorm_y(w.1)), spec.y_scale);

    // Double-click restores the automatic fit.
    if out.response.double_clicked() {
        out.view.reset = true;
        return;
    }
    if navigation.cursor_grabbed {
        return;
    }
    // Select's contract is that the pointer selects and the view stays: no
    // zoom box, no pan, no wheel. A modifier held for a selection gesture was
    // rescaling the plot underneath the selection being made. A pinch selects
    // nothing, so it still zooms there, and a double-click still fits.
    if interaction == InteractionMode::Select {
        if let Some(pointer) = out.response.hover_pos()
            && plot_rect.contains(pointer)
        {
            let pinch = WheelInput::read(ui).pinch;
            if pinch != 1.0 {
                out.view.x = store_x(Window::SAME.zoom(fx_of(pointer.x).clamp(0.0, 1.0), pinch));
                if spec.pinch_zooms_both_axes {
                    out.view.y =
                        store_y(Window::SAME.zoom(fy_of(pointer.y).clamp(0.0, 1.0), pinch));
                }
            }
        }
        return;
    }

    let id = out.response.id;
    let armed_id = id.with("plot.armed-axis");
    let mut armed = ui.data(|data| data.get_temp::<ZoomAxes>(armed_id));
    let focused = out.response.has_focus();
    // The plot's keys are its own while it holds the focus. X and Y, which
    // arm a one-axis zoom, and Enter, which zooms the band between the
    // cursors on the armed axis, are also taken while the pointer is over
    // it, so long as no other widget is taking keys.
    let keys_free = ui.memory(|memory| memory.focused().is_none_or(|focus| focus == id));
    if keys_free && (focused || out.response.contains_pointer()) {
        let plain = ui.input(|input| input.modifiers.is_none());
        let (x, y, enter, escape) = ui.input_mut(|input| {
            (
                plain && input.consume_key(egui::Modifiers::NONE, egui::Key::X),
                plain && input.consume_key(egui::Modifiers::NONE, egui::Key::Y),
                armed.is_some()
                    && plain
                    && input.consume_key(egui::Modifiers::NONE, egui::Key::Enter),
                armed.is_some() && input.consume_key(egui::Modifiers::NONE, egui::Key::Escape),
            )
        });
        for (pressed, axis) in [(x, ZoomAxes::X), (y, ZoomAxes::Y)] {
            if pressed {
                armed = (armed != Some(axis)).then_some(axis);
            }
        }
        if escape {
            armed = None;
        }
        if enter && let Some(axis) = armed.take() {
            let band = match axis {
                ZoomAxes::X => navigation
                    .cursors
                    .and_then(|pair| pair.a.zip(pair.b))
                    .map(|(a, b)| (a.min(b), a.max(b))),
                ZoomAxes::Y | ZoomAxes::Both => {
                    if let [Some(h1), Some(h2)] = spec.horizontal_cursors {
                        Some((h1.min(h2), h1.max(h2)))
                    } else {
                        None
                    }
                }
            };
            if let Some(band) = band {
                match axis {
                    ZoomAxes::X => out.view.x = floored_span(band, spec.x_scale),
                    ZoomAxes::Y | ZoomAxes::Both => {
                        out.view.y = floored_span(band, spec.y_scale);
                    }
                }
            }
        }
    }
    if focused {
        // The arrows pan the plot; egui must not also move the focus on
        // them.
        ui.memory_mut(|memory| {
            memory.set_focus_lock_filter(
                id,
                egui::EventFilter {
                    horizontal_arrows: true,
                    vertical_arrows: true,
                    ..Default::default()
                },
            );
        });
        let mut x = Window::SAME;
        let mut y = Window::SAME;
        let mut fit = false;
        ui.input_mut(|input| {
            let mut pressed = |modifiers, key| input.consume_key(modifiers, key);
            let command = egui::Modifiers::COMMAND;
            let none = egui::Modifiers::NONE;
            for (key, delta) in [
                (egui::Key::ArrowLeft, -KEY_PAN),
                (egui::Key::ArrowRight, KEY_PAN),
            ] {
                if pressed(none, key) {
                    x = x.pan(delta);
                }
            }
            for (key, delta) in [
                (egui::Key::ArrowDown, -KEY_PAN),
                (egui::Key::ArrowUp, KEY_PAN),
            ] {
                if pressed(none, key) {
                    y = y.pan(delta);
                }
            }
            for (key, factor) in [
                (egui::Key::Plus, KEY_ZOOM_IN),
                (egui::Key::Equals, KEY_ZOOM_IN),
                (egui::Key::Minus, KEY_ZOOM_IN.recip()),
            ] {
                // Ctrl is asked first: a bare pattern does not match a key
                // pressed with Ctrl, but the order states which wins.
                if pressed(command, key) {
                    y = y.zoom(0.5, factor);
                } else if pressed(none, key) {
                    x = x.zoom(0.5, factor);
                }
            }
            fit = pressed(none, egui::Key::F) || pressed(none, egui::Key::Home);
        });
        if fit {
            out.view.reset = true;
            return;
        }
        if x != Window::SAME {
            out.view.x = store_x(x);
        }
        if y != Window::SAME {
            out.view.y = store_y(y);
        }
    }

    // Zoom box: Shift+primary drag or right drag. The anchor survives
    // across frames in egui memory; the box zooms on release, along the
    // axes its shape (or the armed axis) picks.
    let shift = ui.input(|i| i.modifiers.shift);
    let box_id = id.with("plot.zoombox");
    let box_drag_started = (out.response.drag_started_by(egui::PointerButton::Primary)
        && (shift || interaction == InteractionMode::Zoom))
        || out.response.drag_started_by(egui::PointerButton::Secondary);
    if box_drag_started
        && let Some(pos) = out.response.interact_pointer_pos()
        && plot_rect.contains(pos)
    {
        ui.data_mut(|data| data.insert_temp(box_id, pos));
    }
    let anchor = ui.data(|data| data.get_temp::<Pos2>(box_id));
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
            // The band shows what it will zoom: a stretch of X spans the
            // plot's height, a stretch of Y its width.
            let band = Rect::from_two_pos(anchor, corner);
            let (shown, ink) = match band_axes(band, armed) {
                Some(ZoomAxes::X) => (
                    Rect::from_x_y_ranges(band.x_range(), plot_rect.y_range()),
                    c.canvas_cursor_a,
                ),
                Some(ZoomAxes::Y) => (
                    Rect::from_x_y_ranges(plot_rect.x_range(), band.y_range()),
                    c.canvas_cursor_a,
                ),
                Some(ZoomAxes::Both) => (band, c.canvas_cursor_a),
                None => (band, c.canvas_text_faint),
            };
            painter.rect(
                shown,
                0.0,
                ink.gamma_multiply(0.08),
                Stroke::new(1.0, ink),
                egui::StrokeKind::Inside,
            );
        }
        if stopped {
            ui.data_mut(|data| data.remove::<Pos2>(box_id));
            if let Some(corner) = corner {
                let band = Rect::from_two_pos(anchor, corner);
                if let Some(axes) = band_axes(band, armed) {
                    if axes.x() {
                        let window =
                            Window(fx_of(band.left()).max(0.0), fx_of(band.right()).min(1.0));
                        out.view.x = store_x(window);
                    }
                    if axes.y() {
                        let window =
                            Window(fy_of(band.bottom()).max(0.0), fy_of(band.top()).min(1.0));
                        out.view.y = store_y(window);
                    }
                    armed = None;
                }
            }
        }
        keep_armed(ui, armed_id, armed);
        return;
    }
    keep_armed(ui, armed_id, armed);
    if let Some(axis) = armed {
        // The armed axis is stated on the plot until a band or Enter uses
        // it, so the key that armed it is not a silent mode.
        painter.text(
            pos2(plot_rect.right() - 4.0, plot_rect.top() + 14.0),
            Align2::RIGHT_TOP,
            match axis {
                ZoomAxes::X => "zoom X",
                ZoomAxes::Y | ZoomAxes::Both => "zoom Y",
            },
            theme::mono(9.0, FontWeight::Medium),
            c.canvas_cursor_a,
        );
    }

    // Drag pan (primary, unmodified): the content follows the pointer, along
    // each axis once the drag has travelled past a dead zone on it. A pan in
    // time wobbles by a pixel or two; moving Y by that pinned an axis that
    // was following the data in view, and a fit the reader never asked to
    // leave stopped refitting. A drag that starts in the Y gutter pans Y
    // alone.
    if out.response.dragged_by(egui::PointerButton::Primary)
        && !shift
        && matches!(interaction, InteractionMode::All | InteractionMode::Pan)
    {
        let delta = out.response.drag_delta();
        let (origin, travelled) = ui.input(|input| {
            let origin = input.pointer.press_origin();
            let travelled = input
                .pointer
                .interact_pos()
                .zip(origin)
                .map_or(egui::Vec2::INFINITY, |(now, from)| now - from);
            (origin, travelled)
        });
        let in_gutter = origin.is_some_and(|origin| gutter.contains(origin));
        if delta != egui::Vec2::ZERO {
            let dfx = -f64::from(delta.x) / f64::from(plot_rect.width());
            if dfx != 0.0 && travelled.x.abs() > PAN_DEAD_ZONE && !in_gutter {
                out.view.x = store_x(Window::SAME.pan(dfx));
            }
            let dfy = f64::from(delta.y) / f64::from(plot_rect.height());
            if dfy != 0.0 && travelled.y.abs() > PAN_DEAD_ZONE {
                out.view.y = store_y(Window::SAME.pan(dfy));
            }
        }
        return;
    }

    // Wheel and pinch about the pointer.
    let Some(pointer) = out.response.hover_pos() else {
        return;
    };
    let over_plot = plot_rect.contains(pointer);
    let over_gutter = gutter.contains(pointer);
    if !(over_plot || over_gutter) {
        return;
    }
    let wheel = WheelInput::read(ui);
    let (fx, fy) = (
        fx_of(pointer.x).clamp(0.0, 1.0),
        fy_of(pointer.y).clamp(0.0, 1.0),
    );
    let plain_zoom = (f64::from(-wheel.scroll.y) * WHEEL_ZOOM_RATE)
        .exp()
        .clamp(0.05, 20.0);
    let mut x = Window::SAME;
    let mut y = Window::SAME;
    if over_gutter {
        // Over the gutter every zoom is the Y axis'.
        let factor = plain_zoom * wheel.ctrl_wheel * wheel.pinch;
        if factor != 1.0 {
            y = y.zoom(fy, factor);
        }
    } else {
        if wheel.pinch != 1.0 {
            x = x.zoom(fx, wheel.pinch);
            if spec.pinch_zooms_both_axes {
                y = y.zoom(fy, wheel.pinch);
            }
        }
        // Ctrl+wheel zooms Y, as in ViVA; with Shift as well, both axes.
        if wheel.ctrl_wheel != 1.0 {
            y = y.zoom(fy, wheel.ctrl_wheel);
            if wheel.shift {
                x = x.zoom(fx, wheel.ctrl_wheel);
            }
        }
        if wheel.scroll.y != 0.0 {
            x = x.zoom(fx, plain_zoom);
        }
        // A horizontal swipe, or the wheel with Shift held, which egui
        // turns into one, pans X; the content follows the swipe.
        if wheel.scroll.x != 0.0 {
            x = x.pan(-f64::from(wheel.scroll.x) / f64::from(plot_rect.width()));
        }
    }
    if wheel.scroll != egui::Vec2::ZERO {
        // Consume the wheel so an enclosing ScrollArea doesn't also scroll
        // the strip list while the user zooms a plot.
        ui.input_mut(|input| input.smooth_scroll_delta = egui::Vec2::ZERO);
    }
    if x != Window::SAME {
        out.view.x = store_x(x);
    }
    if y != Window::SAME {
        out.view.y = store_y(y);
    }
}

/// Whether a marker tag at `rect` covers a trace, a limit line or one of
/// the limit labels at `labels`. Only the samples under the tag are read;
/// a dense trace is read at a stride, which is fine enough for a tag.
fn tag_collides(
    rect: Rect,
    (spec, curves): (&PlotSpec, &[Trace<'_>]),
    plot_rect: Rect,
    mx: &dyn Fn(f64) -> f32,
    my: &dyn Fn(f64) -> f32,
    labels: &[Rect],
) -> bool {
    const MOST_SEGMENTS: usize = 2048;
    let near = rect.expand(2.0);
    if labels.iter().any(|label| label.intersects(near)) {
        return true;
    }
    if spec.limit_lines.iter().any(|line| {
        line.y >= spec.y.min && line.y <= spec.y.max && near.y_range().contains(my(line.y))
    }) {
        return true;
    }
    let data_x = |screen: f32| {
        let at = f64::from((screen - plot_rect.left()) / plot_rect.width());
        spec.x_scale.denormalize(at, spec.x.min, spec.x.max)
    };
    let (from, to) = (data_x(near.left()), data_x(near.right()));
    spec.traces.iter().chain(curves).any(|trace| {
        let (x, y) = (trace.x, trace.y);
        let n = x.len().min(y.len());
        // A trace that does not rise in x, or whose ends are not numbers,
        // has no span to search.
        if n < 2
            || !matches!(
                x[0].partial_cmp(&x[n - 1]),
                Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
            )
        {
            return false;
        }
        let first = x[..n].partition_point(|v| *v < from).saturating_sub(1);
        let last = (x[..n].partition_point(|v| *v <= to) + 1).min(n);
        if last <= first + 1 {
            return false;
        }
        let stride = ((last - first) / MOST_SEGMENTS).max(1);
        let mut previous: Option<Pos2> = None;
        for i in (first..last)
            .step_by(stride)
            .chain(std::iter::once(last - 1))
        {
            if !(x[i].is_finite() && y[i].is_finite()) {
                previous = None;
                continue;
            }
            let point = pos2(mx(x[i]), my(y[i]));
            if let Some(start) = previous
                && segment_meets_rect(start, point, near)
            {
                return true;
            }
            previous = Some(point);
        }
        false
    })
}

/// Whether the segment from `a` to `b` passes through `rect`.
fn segment_meets_rect(a: Pos2, b: Pos2, rect: Rect) -> bool {
    let (left, right) = (a.x.min(b.x), a.x.max(b.x));
    if right < rect.left() || left > rect.right() {
        return false;
    }
    // The segment's height over the part of it inside the rect's columns.
    let at = |x: f32| {
        if (b.x - a.x).abs() < f32::EPSILON {
            a.y
        } else {
            a.y + (b.y - a.y) * (x - a.x) / (b.x - a.x)
        }
    };
    let (x0, x1) = (left.max(rect.left()), right.min(rect.right()));
    let (y0, y1) = if (b.x - a.x).abs() < f32::EPSILON {
        (a.y.min(b.y), a.y.max(b.y))
    } else {
        (at(x0).min(at(x1)), at(x0).max(at(x1)))
    };
    y1 >= rect.top() && y0 <= rect.bottom()
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

/// The staircase a sample-and-hold draws through `points`: each sample held
/// level until the next one's X, where it steps. A hole (a non-finite point)
/// stays a hole.
pub(super) fn held_steps(points: &[Pos2]) -> Vec<Pos2> {
    let mut steps = Vec::with_capacity(points.len() * 2);
    for pair in points.windows(2) {
        let (from, to) = (pair[0], pair[1]);
        steps.push(from);
        if from.is_finite() && to.is_finite() {
            steps.push(pos2(to.x, from.y));
        }
    }
    if let Some(last) = points.last() {
        steps.push(*last);
    }
    steps
}

#[cfg(test)]
mod gesture_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::format::tick_label;
    use crate::plot::{Axis, Marker, Trace, XScale};

    /// One unit spans the same points on both axes of a wide plane and of a
    /// tall one, each range keeps its centre, and the unit square asked for
    /// stays in view.
    #[test]
    fn an_equal_scale_plane_keeps_one_unit_the_same_length_on_both_axes() {
        for (size, wider) in [(vec2(800.0, 400.0), true), (vec2(300.0, 600.0), false)] {
            let (x, y) = equal_scale_ranges(size, (-1.0, 1.0), (-0.5, 1.5));
            let along_x = f64::from(size.x) / (x.1 - x.0);
            let along_y = f64::from(size.y) / (y.1 - y.0);
            assert!((along_x / along_y - 1.0).abs() < 1e-12, "{size:?}");
            assert!(((x.0 + x.1) * 0.5).abs() < 1e-12);
            assert!(((y.0 + y.1) * 0.5 - 0.5).abs() < 1e-12);
            assert!(x.0 <= -1.0 && x.1 >= 1.0 && y.0 <= -0.5 && y.1 >= 1.5);
            // The axis with room to spare is the one that widened.
            if wider {
                assert_eq!(y, (-0.5, 1.5));
                assert_eq!(x, (-2.0, 2.0));
            } else {
                assert_eq!(x, (-1.0, 1.0));
                assert_eq!(y, (-1.5, 2.5));
            }
        }
    }

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
        let inside = inner_rect(
            Rect::from_min_size(pos2(0.0, 0.0), size),
            &spec,
            spec.left_margin,
        )
        .center();
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
        let plot = inner_rect(
            Rect::from_min_size(pos2(0.0, 0.0), size),
            &spec,
            spec.left_margin,
        );
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
