//! The plot painter — turns a [`PlotSpec`] into egui shapes.

use egui::{Align2, Color32, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, pos2, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;

use super::cursor::CursorPair;
use super::decimate::DecimationCache;
use super::spec::{PlotSpec, YSide};

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
}

/// A row of the hover readout: (label, value).
pub type ReadoutRow = (String, String);

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
    let response = ui.allocate_rect(rect, Sense::click());

    let left = spec.left_margin;
    let right = if spec.y_right.is_some() { 54.0 } else { 16.0 };
    let (top, bottom) = (12.0, 26.0);
    let plot_rect = Rect::from_min_max(
        pos2(rect.left() + left, rect.top() + top),
        pos2(rect.right() - right, rect.bottom() - bottom),
    );

    let mut out = PlotResponse {
        response,
        hover_x: None,
        clicked_x: None,
        plot_rect,
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
        let columns = plot_rect.width().ceil() as usize;
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
            if trace.dashed {
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

    out
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
