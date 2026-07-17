//! The plot painter — turns a [`PlotSpec`] into egui shapes.

use egui::{
    Align2, Color32, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, WidgetInfo, WidgetType, pos2,
    vec2,
};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;

use super::cursor::CursorPair;
use super::decimate::{DecimationCache, DisplayDecimation};
use super::spec::{PlotSpec, YSide};

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
    /// New left-Y range.
    pub y: Option<(f64, f64)>,
    /// New right-Y range (only emitted when the spec had a right axis).
    pub y_right: Option<(f64, f64)>,
    /// Double-click: restore the automatic (fit-to-data) view.
    pub reset: bool,
}

impl ViewChange {
    /// Whether the gesture produced any change this frame.
    pub fn any(&self) -> bool {
        self.reset || self.x.is_some() || self.y.is_some() || self.y_right.is_some()
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
    /// The inner plot rectangle (inside margins), for overlays.
    pub plot_rect: Rect,
    /// Zoom/pan/fit gesture result: wheel zooms X about the cursor
    /// (Ctrl+wheel zooms Y), drag pans, Shift- or right-drag draws a
    /// zoom box, double-click fits.
    pub view: ViewChange,
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
const MARGIN_RIGHT_AXIS: f32 = 54.0;

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

fn paint_trace_markers(
    painter: &egui::Painter,
    points: &[Pos2],
    plot_rect: Rect,
    shape: TraceMarkerShape,
    color: Color32,
    background: Color32,
) {
    if points.len() < 2 || plot_rect.width() < 72.0 {
        return;
    }

    let mut point_index = 0;
    let mut target_x = plot_rect.left() + 48.0;
    while target_x < plot_rect.right() - 24.0 {
        while point_index + 1 < points.len() && points[point_index].x < target_x {
            point_index += 1;
        }
        let point = points[point_index];
        if point.x.is_finite() && point.y.is_finite() && plot_rect.shrink(4.0).contains(point) {
            paint_trace_marker(painter, point, shape, color, background);
        }
        target_x += 96.0;
    }
}

fn right_margin(spec: &PlotSpec<'_>) -> f32 {
    if spec.y_right.is_some() {
        MARGIN_RIGHT_AXIS
    } else {
        MARGIN_RIGHT_PLAIN
    }
}

fn inner_rect(rect: Rect, spec: &PlotSpec<'_>) -> Rect {
    Rect::from_min_max(
        pos2(rect.left() + spec.left_margin, rect.top() + MARGIN_TOP),
        pos2(
            rect.right() - right_margin(spec),
            rect.bottom() - MARGIN_BOTTOM,
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
    let right_axis = spec.y_right.as_ref().map_or_else(String::new, |(axis, _)| {
        format!(" Right Y axis {}.", axis_accessibility_range(axis))
    });
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
    format!(
        "{}. {}. X axis {}. Left Y axis {}.{} {}.{} Drag to pan, use the mouse wheel to zoom, Shift-drag or right-drag to zoom a region, and double-click to fit the data.",
        spec.accessible_name,
        counted(trace_count, "visible trace", "visible traces"),
        axis_accessibility_range(&spec.x),
        axis_accessibility_range(&spec.y),
        right_axis,
        counted(spec.markers.len(), "marker", "markers"),
        cursor_summary,
    )
}

/// A rect centered in `avail` whose INNER plot area (after this spec's
/// margins) is square. The XY viewers (Smith, Nyquist, pole-zero) need
/// circle grids and trace geometry on identical X/Y scales — a square
/// OUTER rect leaves the inner area taller than wide and the unit circle
/// would lie about |Γ| = 1.
pub fn square_outer_rect(avail: Rect, spec: &PlotSpec<'_>) -> Rect {
    let h_margins = spec.left_margin + right_margin(spec);
    let v_margins = MARGIN_TOP + MARGIN_BOTTOM;
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
        plot_rect,
        view: ViewChange::default(),
    };
    if plot_rect.width() < 24.0
        || plot_rect.height() < 24.0
        || !matches!(
            spec.x.max.partial_cmp(&spec.x.min),
            Some(std::cmp::Ordering::Greater)
        )
        || !matches!(
            spec.y.max.partial_cmp(&spec.y.min),
            Some(std::cmp::Ordering::Greater)
        )
    {
        theme::paint_focus_ring(ui, &out.response, rect);
        return out;
    }

    let mx = |x: f64| -> f32 {
        plot_rect.left()
            + (spec.x_scale.normalize(x, spec.x.min, spec.x.max) as f32) * plot_rect.width()
    };
    let my = |y: f64| -> f32 {
        plot_rect.bottom()
            - (((y - spec.y.min) / (spec.y.max - spec.y.min)) as f32) * plot_rect.height()
    };
    let my_r = |y: f64| -> f32 {
        // A right-side trace with no right axis is an inconsistent spec;
        // map it against the left axis rather than panicking mid-frame.
        match spec.y_right.as_ref() {
            Some((axis, _)) => {
                plot_rect.bottom()
                    - (((y - axis.min) / (axis.max - axis.min)) as f32) * plot_rect.height()
            }
            None => my(y),
        }
    };
    let map_y = |y: f64, side: YSide| -> f32 {
        match side {
            YSide::Left => my(y),
            YSide::Right => my_r(y),
        }
    };

    let painter = ui.painter_at(rect);
    let tick_font = theme::mono(10.0, FontWeight::Regular);
    let grid = Stroke::new(1.0, c.canvas_grid);
    let frame = Stroke::new(1.0, c.border_strong);

    // ---- bands (under everything)
    for band in &spec.bands {
        let band_rect = Rect::from_min_max(
            pos2(mx(band.x0), plot_rect.top()),
            pos2(mx(band.x1), plot_rect.bottom()),
        );
        painter.rect_filled(band_rect, 0.0, c.accent_dim.gamma_multiply(0.4));
    }

    // ---- grid + ticks
    // The x-axis unit owns the right end of the tick row; a tick label that
    // would run into it is dropped (its gridline stays).
    let x_end_label = spec.x.end_label();
    let x_unit_left = if x_end_label.is_empty() {
        f32::INFINITY
    } else {
        let unit = painter.layout_no_wrap(x_end_label.clone(), tick_font.clone(), c.text_dim);
        plot_rect.right() - unit.size().x - 8.0
    };
    // Labels skip when they would collide with the previous label (dense
    // log decades at deep zoom) — every gridline still draws.
    let mut last_label_right = f32::NEG_INFINITY;
    for (xv, label) in &spec.x.ticks {
        let px = mx(*xv);
        painter.vline(px, plot_rect.y_range(), grid);
        let galley = painter.layout_no_wrap(label.clone(), tick_font.clone(), c.text_dim);
        let half = galley.size().x * 0.5;
        if px + half <= x_unit_left && px - half >= last_label_right + 6.0 {
            last_label_right = px + half;
            painter.galley(
                pos2(px - half, rect.bottom() - 9.0 - galley.size().y * 0.5),
                galley,
                c.text_dim,
            );
        }
    }
    for (yv, label) in &spec.y.ticks {
        let py = my(*yv);
        painter.hline(plot_rect.x_range(), py, grid);
        painter.text(
            pos2(plot_rect.left() - 7.0, py),
            Align2::RIGHT_CENTER,
            label,
            tick_font.clone(),
            c.text_dim,
        );
    }
    if let Some((axis, tint)) = &spec.y_right {
        for (yv, label) in &axis.ticks {
            painter.text(
                pos2(plot_rect.right() + 8.0, my_r(*yv)),
                Align2::LEFT_CENTER,
                label,
                tick_font.clone(),
                *tint,
            );
        }
    }

    // ---- reference lines
    let ref_stroke = Stroke::new(1.0, c.text_faint);
    for line in &spec.ref_lines {
        let py = my(line.y);
        painter.extend(Shape::dashed_line(
            &[pos2(plot_rect.left(), py), pos2(plot_rect.right(), py)],
            ref_stroke,
            4.0,
            3.0,
        ));
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
                        spec.x.min,
                        spec.x.max,
                        spec.x_scale,
                        columns,
                    )
                    .iter()
                    .map(|p| pos2(mx(p[0]), map_y(p[1], trace.side)))
                    .collect(),
                _ => trace
                    .x
                    .iter()
                    .zip(trace.y.iter())
                    .map(|(&x, &y)| pos2(mx(x), map_y(y, trace.side)))
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
                if let Some((dash_length, gap_length)) = dash.filter(|_| points.len() < columns) {
                    clipped.extend(Shape::dashed_line(&points, stroke, dash_length, gap_length));
                } else {
                    clipped.add(Shape::line(points.clone(), stroke));
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
    if spec.y_right.is_some() {
        painter.vline(plot_rect.right(), plot_rect.y_range(), frame);
    }
    if !spec.y.unit.is_empty() {
        painter.text(
            pos2(plot_rect.left() - 7.0, rect.top() + 8.0),
            Align2::RIGHT_CENTER,
            spec.y.unit.as_str(),
            tick_font.clone(),
            c.text_dim,
        );
    }
    if let Some((axis, tint)) = &spec.y_right
        && !axis.unit.is_empty()
    {
        painter.text(
            pos2(plot_rect.right() + 8.0, rect.top() + 8.0),
            Align2::LEFT_CENTER,
            axis.unit.as_str(),
            tick_font.clone(),
            *tint,
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
        let px = mx(marker.x);
        let py = map_y(marker.y, marker.side);
        if marker.drop_line {
            painter.extend(Shape::dashed_line(
                &[pos2(px, py), pos2(px, plot_rect.bottom())],
                ref_stroke,
                4.0,
                3.0,
            ));
        }
        painter.circle(
            pos2(px, py),
            3.0,
            c.canvas_bg,
            Stroke::new(1.5, marker.color),
        );

        let galley = painter.layout_no_wrap(marker.label.clone(), tag_font.clone(), marker.color);
        let (pad, tag_h) = (6.0, 16.0);
        let tag_w = galley.size().x + pad * 2.0;
        let mut tx = px + 9.0;
        let mut ty = py - tag_h - 7.0 + marker.label_dy;
        if tx + tag_w > plot_rect.right() - 4.0 {
            tx = px - tag_w - 9.0;
        }
        if ty < plot_rect.top() + 2.0 {
            ty = py + 9.0;
        }
        let tag_rect = Rect::from_min_size(pos2(tx, ty), vec2(tag_w, tag_h));
        painter.rect(
            tag_rect,
            t.radius,
            c.bg_elevated,
            Stroke::new(1.0, c.border_strong),
            egui::StrokeKind::Inside,
        );
        painter.galley(
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
    }

    // ---- navigation gestures: wheel zoom, drag pan, zoom box, fit
    handle_navigation(ui, spec, &painter, plot_rect, &mut out, &t);

    theme::paint_focus_ring(ui, &out.response, rect);

    out
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
    let denorm_y = |frac: f64| spec.y.min + frac * (spec.y.max - spec.y.min);
    let denorm_yr = |frac: f64| {
        spec.y_right
            .as_ref()
            .map(|(axis, _)| axis.min + frac * (axis.max - axis.min))
    };

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
    let box_drag_started = (out.response.drag_started_by(egui::PointerButton::Primary)
        && (shift || interaction == InteractionMode::Zoom))
        || (interaction != InteractionMode::Select
            && out.response.drag_started_by(egui::PointerButton::Secondary));
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
                    out.view.x = Some((fx_of(band.left()).max(0.0), fx_of(band.right()).min(1.0)))
                        .map(|(a, b)| (denorm_x(a), denorm_x(b)));
                    let (fy0, fy1) = (fy_of(band.bottom()).max(0.0), fy_of(band.top()).min(1.0));
                    out.view.y = Some((denorm_y(fy0), denorm_y(fy1)));
                    if let (Some(a), Some(b)) = (denorm_yr(fy0), denorm_yr(fy1)) {
                        out.view.y_right = Some((a, b));
                    }
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
                out.view.x = Some((denorm_x(dfx), denorm_x(1.0 + dfx)));
            }
            let dy = f64::from(delta.y) / f64::from(plot_rect.height());
            if dy != 0.0 {
                let span = spec.y.max - spec.y.min;
                out.view.y = Some((spec.y.min + dy * span, spec.y.max + dy * span));
                if let Some((axis, _)) = &spec.y_right {
                    let span_r = axis.max - axis.min;
                    out.view.y_right = Some((axis.min + dy * span_r, axis.max + dy * span_r));
                }
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
                out.view.y = Some((denorm_y(f0), denorm_y(f1)));
                if let (Some(a), Some(b)) = (denorm_yr(f0), denorm_yr(f1)) {
                    out.view.y_right = Some((a, b));
                }
            } else {
                let fx = fx_of(pointer.x);
                let (f0, f1) = (fx * (1.0 - factor), fx + (1.0 - fx) * factor);
                out.view.x = Some((denorm_x(f0), denorm_x(f1)));
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
            side: YSide::Left,
            color: egui::Color32::WHITE,
            label: "UGF".to_owned(),
            drop_line: true,
            label_dy: 0.0,
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
        assert!(label.contains(&format!("Left Y axis {} to 5 dB.", tick_label(-5.0))));
        assert!(label.contains("1 marker."));
        assert!(label.contains("Cursor A 2, cursor B 5, delta 3."));
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
        let output = ctx.run(Default::default(), |ctx| {
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
