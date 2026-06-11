//! The plot painter — turns a [`PlotSpec`] into egui shapes.

use egui::{Align2, Color32, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, pos2, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;

use super::cursor::CursorPair;
use super::decimate::DecimationCache;
use super::spec::{PlotSpec, YSide};

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
        pos2(rect.right() - right_margin(spec), rect.bottom() - MARGIN_BOTTOM),
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
    Rect::from_center_size(
        avail.center(),
        vec2(inner + h_margins, inner + v_margins),
    )
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
        || !(spec.x.max > spec.x.min)
        || !(spec.y.max > spec.y.min)
    {
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
        let (axis, _) = spec.y_right.as_ref().expect("right axis");
        plot_rect.bottom()
            - (((y - axis.min) / (axis.max - axis.min)) as f32) * plot_rect.height()
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
    let x_unit_left = if spec.x.unit.is_empty() {
        f32::INFINITY
    } else {
        let unit = painter.layout_no_wrap(spec.x.unit.to_owned(), tick_font.clone(), c.text_dim);
        plot_rect.right() - unit.size().x - 8.0
    };
    for (xv, label) in &spec.x.ticks {
        let px = mx(*xv);
        painter.vline(px, plot_rect.y_range(), grid);
        let galley = painter.layout_no_wrap(label.clone(), tick_font.clone(), c.text_dim);
        if px + galley.size().x * 0.5 <= x_unit_left {
            painter.galley(
                pos2(
                    px - galley.size().x * 0.5,
                    rect.bottom() - 9.0 - galley.size().y * 0.5,
                ),
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
        for trace in &spec.traces {
            if trace.x.is_empty() {
                continue;
            }
            let stroke = Stroke::new(trace.width, trace.color);
            let points: Vec<Pos2> = match trace.cache_key {
                Some(key) => cache
                    .envelope(
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
                None => trace
                    .x
                    .iter()
                    .zip(trace.y.iter())
                    .map(|(&x, &y)| pos2(mx(x), map_y(y, trace.side)))
                    .collect(),
            };
            if points.len() < 2 {
                continue;
            }
            // Dashing a dense min/max envelope would emit one shape per
            // dash along a path that zig-zags every column — thousands of
            // segments reading as noise. Sparse curves dash normally.
            if trace.dashed && points.len() < columns {
                clipped.extend(Shape::dashed_line(&points, stroke, 5.0, 4.0));
            } else {
                clipped.add(Shape::line(points, stroke));
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
            spec.y.unit,
            tick_font.clone(),
            c.text_dim,
        );
    }
    if let Some((axis, tint)) = &spec.y_right {
        if !axis.unit.is_empty() {
            painter.text(
                pos2(plot_rect.right() + 8.0, rect.top() + 8.0),
                Align2::LEFT_CENTER,
                axis.unit,
                tick_font.clone(),
                *tint,
            );
        }
    }
    if !spec.x.unit.is_empty() {
        painter.text(
            pos2(plot_rect.right(), rect.bottom() - 9.0),
            Align2::RIGHT_CENTER,
            spec.x.unit,
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
        painter.circle(pos2(px, py), 3.0, c.canvas_bg, Stroke::new(1.5, marker.color));

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
        painter.rect(tag_rect, t.radius, c.bg_elevated, Stroke::new(1.0, c.border_strong));
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
    if let Some(pointer) = out.response.hover_pos() {
        if plot_rect.contains(pointer) {
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
    }

    // ---- navigation gestures: wheel zoom, drag pan, zoom box, fit
    handle_navigation(ui, spec, &painter, plot_rect, &mut out, &t);

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
    if out.response.double_clicked() {
        out.view.reset = true;
        return;
    }

    let shift = ui.input(|i| i.modifiers.shift);
    let ctrl = ui.input(|i| i.modifiers.ctrl);
    let box_id = out.response.id.with("plot.zoombox");

    // Zoom box: Shift+primary drag or right drag. The anchor survives
    // across frames in egui memory; the box zooms both axes on release.
    let box_drag_started = (out.response.drag_started_by(egui::PointerButton::Primary) && shift)
        || out.response.drag_started_by(egui::PointerButton::Secondary);
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
    if out.response.dragged_by(egui::PointerButton::Primary) && !shift {
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
    if let Some(pointer) = out.response.hover_pos()
        && plot_rect.contains(pointer)
    {
        let scroll = ui.input(|i| i.smooth_scroll_delta.y);
        if scroll != 0.0 {
            // Consume the wheel so an enclosing ScrollArea doesn't also
            // scroll the strip list while the user zooms a plot.
            ui.input_mut(|i| {
                i.raw_scroll_delta = egui::Vec2::ZERO;
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
    origin.y = origin
        .y
        .clamp(plot_rect.top(), (rect.bottom() - box_h - 4.0).max(plot_rect.top()));

    let bg = Rect::from_min_size(origin, vec2(box_w, box_h));
    painter.rect(bg, t.radius, c.bg_elevated, Stroke::new(1.0, c.border_strong));
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
