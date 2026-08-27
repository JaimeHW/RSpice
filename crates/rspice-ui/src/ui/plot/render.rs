//! The plot painter — turns a [`PlotSpec`] into egui shapes.

use egui::{
    Align2, Color32, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, WidgetInfo, WidgetType, pos2,
    vec2,
};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;

use super::cursor::CursorPair;
use super::decimate::{DecimationCache, DisplayDecimation};
use super::scale::XScale;
use super::spec::{MarkerShape, PlotSpec};

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
    /// Pointer X in data space while hovering inside the plot area.
    pub hover_x: Option<f64>,
    /// Data-space X of a primary click inside the plot area, this frame.
    pub clicked_x: Option<f64>,
    /// Updated left-axis horizontal cursor value from a click or drag.
    pub horizontal_cursor_y: Option<f64>,
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
const MARGIN_BOTTOM: f32 = 26.0;
const MARGIN_RIGHT_PLAIN: f32 = 16.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TraceMarkerShape {
    Circle,
    Square,
    Diamond,
    Triangle,
    Cross,
    Plus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TraceRedundancy {
    dash: Option<(f32, f32)>,
    marker: TraceMarkerShape,
}

fn color_safe_trace_style(index: usize, semantic_dashed: bool) -> TraceRedundancy {
    let mut style = match index % 6 {
        0 => TraceRedundancy {
            dash: None,
            marker: TraceMarkerShape::Circle,
        },
        1 => TraceRedundancy {
            dash: Some((7.0, 4.0)),
            marker: TraceMarkerShape::Square,
        },
        2 => TraceRedundancy {
            dash: Some((2.0, 4.0)),
            marker: TraceMarkerShape::Diamond,
        },
        3 => TraceRedundancy {
            dash: Some((10.0, 4.0)),
            marker: TraceMarkerShape::Triangle,
        },
        4 => TraceRedundancy {
            dash: Some((5.0, 2.0)),
            marker: TraceMarkerShape::Cross,
        },
        _ => TraceRedundancy {
            dash: Some((12.0, 3.0)),
            marker: TraceMarkerShape::Plus,
        },
    };
    if semantic_dashed && style.dash.is_none() {
        style.dash = Some((5.0, 4.0));
    }
    style
}

fn resolved_trace_marker(
    marker_style: Option<usize>,
    redundancy: Option<TraceRedundancy>,
    show_single_point: bool,
    point_count: usize,
) -> Option<TraceMarkerShape> {
    marker_style
        .map(|ordinal| color_safe_trace_style(ordinal, false).marker)
        .or_else(|| redundancy.map(|style| style.marker))
        .or_else(|| (show_single_point && point_count == 1).then_some(TraceMarkerShape::Circle))
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

/// Where a trace's categorical markers sit: evenly spaced along the curve.
///
/// Along the curve, not along the X axis. Walking X assumes the drawn path
/// advances left to right, which a locus does not — it doubles back, so the
/// search that only ever moved forward stuck at the turn and stamped the same
/// marker repeatedly on one point while the whole return branch got none.
/// Arc position is the same walk for an ordinary trace and the right one for
/// every other, and it keeps the count bounded by the pane's width.
fn trace_marker_positions(points: &[Pos2], plot_rect: Rect) -> Vec<Pos2> {
    const MIN_SPACING: f32 = 96.0;
    if points.len() < 2 || plot_rect.width() < 72.0 {
        return Vec::new();
    }
    let finite = |point: &Pos2| point.x.is_finite() && point.y.is_finite();
    let length = |a: Pos2, b: Pos2| (b - a).length();
    let total: f32 = points
        .windows(2)
        .filter(|pair| finite(&pair[0]) && finite(&pair[1]))
        .map(|pair| length(pair[0], pair[1]))
        .sum();
    if !(total > 0.0) {
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
        if !(finite(&a) && finite(&b)) {
            continue;
        }
        let segment = length(a, b);
        if segment <= 0.0 {
            continue;
        }
        while next <= travelled + segment {
            let along = ((next - travelled) / segment).clamp(0.0, 1.0);
            let point = a + (b - a) * along;
            if bounds.contains(point) {
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
    shape: TraceMarkerShape,
    color: Color32,
    background: Color32,
) {
    for point in trace_marker_positions(points, plot_rect) {
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

fn inner_rect(rect: Rect, spec: &PlotSpec<'_>) -> Rect {
    Rect::from_min_max(
        pos2(rect.left() + spec.left_margin, rect.top() + MARGIN_TOP),
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
    use crate::ui::accessibility::counted;
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
    let horizontal_cursor = spec.horizontal_cursor.map_or_else(String::new, |value| {
        format!(" Horizontal cursor {}.", spec.y.format_display_value(value))
    });
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
    let v_margins = MARGIN_TOP + bottom_margin(spec);
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
        hover_x: None,
        clicked_x: None,
        horizontal_cursor_y: None,
        plot_rect,
        view: ViewChange::default(),
        axes: ((spec.x.min, spec.x.max), (spec.y.min, spec.y.max)),
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
    let tick_font = theme::mono(10.0, FontWeight::Regular);
    let grid = Stroke::new(1.0, c.canvas_grid);
    let frame = Stroke::new(1.0, c.border_strong);

    // ---- bands (under everything)
    for band in &spec.bands {
        let (left, right) = (mx(band.x0), mx(band.x1));
        let band_rect = Rect::from_min_max(
            pos2(left.min(right).max(plot_rect.left()), plot_rect.top()),
            pos2(
                left.max(right).min(plot_rect.right()),
                plot_rect.bottom(),
            ),
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
        let unit = painter.layout_no_wrap(x_end_label.clone(), tick_font.clone(), c.text_dim);
        plot_rect.right() - unit.size().x - 8.0
    };
    // Labels skip when they would collide with the previous label (dense
    // log decades at deep zoom) — every gridline still draws.
    if spec.minor_grid {
        let minor = Stroke::new(1.0, c.canvas_grid.gamma_multiply(0.45));
        for value in
            super::scale::minor_grid_values(spec.x_scale, &spec.x.ticks, spec.x.min, spec.x.max)
        {
            painter.vline(mx(value), plot_rect.y_range(), minor);
        }
        for value in
            super::scale::minor_grid_values(spec.y_scale, &spec.y.ticks, spec.y.min, spec.y.max)
        {
            painter.hline(plot_rect.x_range(), my(value), minor);
        }
    }
    // The Y column's chrome is either its unit or, when the window is too
    // narrow to label absolutely, the anchor its ticks are offsets from —
    // which carries the unit itself. Either way it owns the top of the
    // column, so tick labels give way to it.
    let y_chrome = spec
        .y
        .offset_anchor
        .clone()
        .unwrap_or_else(|| spec.y.unit.clone());
    let y_chrome = (!y_chrome.is_empty())
        .then(|| painter.layout_no_wrap(y_chrome, tick_font.clone(), c.text_dim));
    let y_chrome_bottom = y_chrome
        .as_ref()
        .map_or(f32::NEG_INFINITY, |galley| rect.top() + 8.0 + galley.size().y * 0.5);
    let mut last_label_right = f32::NEG_INFINITY;
    if spec.x_axis_chrome
        && let Some(anchor) = &spec.x.offset_anchor
    {
        // Stated once, in the gutter the Y labels leave free, and brighter
        // than the ticks: it is the value, and they are only the differences.
        let galley = painter.layout_no_wrap(anchor.clone(), tick_font.clone(), c.text);
        let width = galley.size().x;
        painter.galley(
            pos2(
                rect.left() + 2.0,
                rect.bottom() - 9.0 - galley.size().y * 0.5,
            ),
            galley,
            c.text,
        );
        last_label_right = rect.left() + 2.0 + width;
    }
    for (xv, label) in &spec.x.ticks {
        let px = mx(*xv);
        painter.vline(px, plot_rect.y_range(), grid);
        let galley = painter.layout_no_wrap(label.clone(), tick_font.clone(), c.text_dim);
        let half = galley.size().x * 0.5;
        if spec.x_axis_chrome && px + half <= x_unit_left && px - half >= last_label_right + 6.0 {
            last_label_right = px + half;
            painter.galley(
                pos2(px - half, rect.bottom() - 9.0 - galley.size().y * 0.5),
                galley,
                c.text_dim,
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
        let galley = painter.layout_no_wrap(label.clone(), tick_font.clone(), c.text_dim);
        let (height, top) = (galley.size().y, py - galley.size().y * 0.5);
        if top + height <= last_label_top - 2.0 && top >= y_chrome_bottom + 2.0 {
            last_label_top = top;
            painter.galley(
                pos2(plot_rect.left() - 7.0 - galley.size().x, top),
                galley,
                c.text_dim,
            );
        }
    }
    // ---- reference lines
    let ref_stroke = Stroke::new(1.0, c.text_faint);
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
            pos2(plot_rect.right() - 4.0, (py - 3.0).max(plot_rect.top() + 11.0)),
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
        for (trace_index, trace) in spec.traces.iter().enumerate() {
            if trace.x.is_empty() {
                continue;
            }
            let stroke = Stroke::new(trace.width, trace.color);
            let points: Vec<Pos2> = match (trace.cache_key, spec.display_decimation) {
                (
                    Some(key),
                    mode @ (DisplayDecimation::EnvelopeExtrema | DisplayDecimation::Uniform),
                ) => cache
                    .series(
                        mode,
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
                _ => trace
                    .x
                    .iter()
                    .zip(trace.y.iter())
                    .map(|(&x, &y)| pos2(mx(x), my(y)))
                    .collect(),
            };
            let redundancy = t
                .color_safe_traces
                .then(|| color_safe_trace_style(trace_index, trace.dashed));
            let dash = trace
                .dash_style
                .and_then(|ordinal| color_safe_trace_style(ordinal, false).dash)
                .or_else(|| redundancy.and_then(|style| style.dash))
                .or_else(|| trace.dashed.then_some((5.0, 4.0)));
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
            let marker = resolved_trace_marker(
                trace.marker_style,
                redundancy,
                trace.show_single_point,
                points.len(),
            );
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
                        marker,
                        trace.color,
                        c.canvas_bg,
                    );
                }
            }
        }
    }

    // ---- frame + axis units
    painter.vline(plot_rect.left(), plot_rect.y_range(), frame);
    painter.hline(plot_rect.x_range(), plot_rect.bottom(), frame);
    if let Some(galley) = y_chrome {
        painter.galley(
            pos2(
                plot_rect.left() - 7.0 - galley.size().x,
                rect.top() + 8.0 - galley.size().y * 0.5,
            ),
            galley,
            c.text_dim,
        );
    }
    if !x_end_label.is_empty() {
        painter.text(
            pos2(plot_rect.right(), rect.bottom() - 9.0),
            Align2::RIGHT_CENTER,
            x_end_label,
            tick_font.clone(),
            c.text_dim,
        );
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
        if !limit_line && !(marker.y >= spec.y.min && marker.y <= spec.y.max) {
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
            draw_cursor(a, "A", c.accent);
        }
        if let Some(b) = pair.b {
            draw_cursor(b, "B", c.traces[4]);
        }
    }

    if let Some(y) = spec
        .horizontal_cursor
        .filter(|y| y.is_finite() && *y >= spec.y.min && *y <= spec.y.max)
    {
        let py = my(y);
        painter.hline(plot_rect.x_range(), py, Stroke::new(1.0, c.accent));
        painter.text(
            pos2(plot_rect.left() + 4.0, py - 3.0),
            Align2::LEFT_BOTTOM,
            format!("H {}", spec.y.format_display_value(y)),
            theme::mono(9.0, FontWeight::Medium),
            c.accent,
        );
    }

    // ---- pointer: crosshair, readout, clicks
    if let Some(pointer) = out.response.hover_pos()
        && plot_rect.contains(pointer)
    {
        let frac = ((pointer.x - plot_rect.left()) / plot_rect.width()) as f64;
        let data_x = spec.x_scale.denormalize(frac, spec.x.min, spec.x.max);
        out.hover_x = Some(data_x);

        if let Some(readout) = readout {
            painter.extend(Shape::dashed_line(
                &[
                    pos2(pointer.x, plot_rect.top()),
                    pos2(pointer.x, plot_rect.bottom()),
                ],
                Stroke::new(1.0, c.text_faint),
                3.0,
                3.0,
            ));
            draw_readout(ui, &t, rect, plot_rect, pointer, &readout(data_x));
        }
        if out.response.clicked() {
            out.clicked_x = Some(data_x);
        }
        if spec.horizontal_cursor_interactive
            && (out.response.clicked() || out.response.dragged_by(egui::PointerButton::Primary))
        {
            let fraction = ((plot_rect.bottom() - pointer.y) / plot_rect.height()).clamp(0.0, 1.0);
            out.horizontal_cursor_y = Some(spec.y_scale.denormalize(
                f64::from(fraction),
                spec.y.min,
                spec.y.max,
            ));
        }
    }

    // ---- navigation gestures: wheel zoom, drag pan, zoom box, fit
    handle_navigation(ui, spec, &painter, plot_rect, &mut out, &t);

    theme::paint_focus_ring(ui, &out.response, rect);

    out
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
                c.accent.gamma_multiply(0.08),
                Stroke::new(1.0, c.accent),
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

    // Drag pan (primary, unmodified): the content follows the pointer.
    if out.response.dragged_by(egui::PointerButton::Primary)
        && !shift
        && matches!(interaction, InteractionMode::All | InteractionMode::Pan)
    {
        let delta = out.response.drag_delta();
        if delta != egui::Vec2::ZERO {
            let dfx = -f64::from(delta.x) / f64::from(plot_rect.width());
            if dfx != 0.0 {
                out.view.x = store_x((denorm_x(dfx), denorm_x(1.0 + dfx)));
            }
            let dy = f64::from(delta.y) / f64::from(plot_rect.height());
            if dy != 0.0 {
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
    use crate::ui::plot::format::tick_label;
    use crate::ui::plot::{Axis, Marker, Trace, XScale};

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
    fn color_safe_trace_cycle_has_six_marker_shapes_and_preserves_phase_dashing() {
        let styles = (0..6)
            .map(|index| color_safe_trace_style(index, false))
            .collect::<Vec<_>>();
        let marker_shapes = styles
            .iter()
            .map(|style| style.marker)
            .collect::<std::collections::HashSet<_>>();

        assert_eq!(marker_shapes.len(), 6);
        assert!(styles[0].dash.is_none());
        assert!(styles[1..].iter().all(|style| style.dash.is_some()));
        assert!(color_safe_trace_style(0, true).dash.is_some());
        assert_eq!(color_safe_trace_style(6, false), styles[0]);
    }

    #[test]
    fn isolated_family_point_uses_neutral_marker_without_category_invention() {
        assert_eq!(
            resolved_trace_marker(None, None, true, 1),
            Some(TraceMarkerShape::Circle)
        );
        assert_eq!(resolved_trace_marker(None, None, true, 2), None);
        assert_eq!(
            resolved_trace_marker(Some(2), None, true, 1),
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
        crate::ui::Theme::default().apply(&ctx);
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

    fn degenerate_spec<'a>(x: &'a [f64], y: &'a [f64], min: f64, max: f64) -> PlotSpec<'a> {
        let mut spec = PlotSpec::new(
            Axis::linear(min, max, "s"),
            XScale::Linear,
            Axis::linear(min, max, "V"),
        );
        spec.traces.push(Trace::new(x, y, egui::Color32::WHITE).cache_key(1));
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
            assert!(shapes.iter().all(|clipped| clipped.shape.visual_bounding_rect().is_finite()
                || clipped.shape.visual_bounding_rect().is_negative()));
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
        spec.bands.push(super::super::spec::Band { x0: -4.0, x1: -2.0 });
        spec.bands.push(super::super::spec::Band { x0: 0.4, x1: 3.0 });
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
            crate::ui::Theme::default().apply(&ctx);
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
        assert!(!selected.any(), "Select mode changed the view: {selected:?}");
    }

    /// Markers follow the curve, not the abscissa. On a locus the X walk
    /// stalled at the first turn and stamped one point over and over.
    #[test]
    fn trace_markers_walk_the_curve_rather_than_the_abscissa() {
        let plot = Rect::from_min_size(pos2(0.0, 0.0), vec2(600.0, 400.0));
        let circle: Vec<Pos2> = (0..=720)
            .map(|step: u16| {
                let angle = f32::from(step) * std::f32::consts::TAU / 720.0;
                plot.center() + vec2(angle.cos(), angle.sin()) * 150.0
            })
            .collect();
        let placed = trace_marker_positions(&circle, plot);

        assert!(placed.len() >= 4, "{} markers on a locus", placed.len());
        for pair in placed.windows(2) {
            assert!(
                (pair[1] - pair[0]).length() > 24.0,
                "markers stacked at {:?}",
                pair[0]
            );
        }
        assert!(placed.iter().all(|point| plot.contains(*point)));

        // An ordinary left-to-right trace keeps its even cadence.
        let ramp: Vec<Pos2> = (0..=600u16)
            .map(|step| pos2(f32::from(step), 200.0 + f32::from(step) * 0.1))
            .collect();
        let placed = trace_marker_positions(&ramp, plot);
        assert!(placed.len() >= 4 && placed.len() <= 8, "{placed:?}");
        assert!(placed.windows(2).all(|pair| pair[1].x > pair[0].x));
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
        assert_eq!(
            floored_span((0.0, 1.0), XScale::Linear),
            Some((0.0, 1.0))
        );
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
        crate::ui::Theme::default().apply(&ctx);
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
