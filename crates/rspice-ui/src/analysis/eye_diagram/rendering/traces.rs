use egui::{Color32, Painter, Pos2, Rect, Rounding, Stroke, Vec2};

use super::super::data::EyeTrace;
use super::super::state::{EyeDiagramState, EyePersistenceCache, EyePersistenceCacheKey};
use super::axes::{
    eye_time_to_x, eye_view_time_bounds, eye_view_voltage_bounds, eye_voltage_to_y,
    trace_time_to_seconds,
};
use super::style::{highlight_color, trace_color};

pub(super) fn render_traces_overlay(painter: &Painter, rect: Rect, state: &EyeDiagramState) {
    let data = &state.data;
    let stroke = Stroke::new(1.0, trace_color());

    for trace in &data.traces {
        render_single_eye_trace(painter, rect, state, trace, stroke);
    }
}

pub(super) fn render_traces_persistence(
    painter: &Painter,
    rect: Rect,
    state: &mut EyeDiagramState,
) {
    let color_map = state.color_map;
    let intensity_exponent = persistence_intensity_exponent(state.persistence_decay);
    let Some(grid) = ensure_persistence_grid(rect, state) else {
        return;
    };
    let cell_width = rect.width() / grid.width as f32;
    let cell_height = rect.height() / grid.height as f32;
    if !cell_width.is_finite()
        || !cell_height.is_finite()
        || cell_width <= 0.0
        || cell_height <= 0.0
    {
        return;
    }

    for (x, y, count) in &grid.nonzero_bins {
        let base = *count as f32 / grid.max_count.max(1) as f32;
        let intensity = base.powf(intensity_exponent).clamp(0.0, 1.0);
        let (r, g, b) = color_map.map(intensity);
        let alpha = (32.0 + intensity * 223.0).clamp(0.0, 255.0) as u8;
        let color = Color32::from_rgba_unmultiplied(r, g, b, alpha);
        let min = Pos2::new(
            rect.min.x + *x as f32 * cell_width,
            rect.min.y + *y as f32 * cell_height,
        );
        painter.rect_filled(
            Rect::from_min_size(min, Vec2::new(cell_width + 0.5, cell_height + 0.5)),
            Rounding::ZERO,
            color,
        );
    }
}

#[derive(Debug, Clone)]
struct PersistenceGrid {
    width: usize,
    height: usize,
    nonzero_bins: Vec<(usize, usize, u32)>,
    max_count: u32,
}

pub(super) fn persistence_intensity_exponent(decay: f32) -> f32 {
    // Higher decay => longer persistence tail => brighter low-density bins.
    let clamped = decay.clamp(0.50, 0.999);
    0.35 + (1.0 - clamped) * 3.3
}

pub(super) fn ensure_persistence_grid(
    plot_rect: Rect,
    state: &mut EyeDiagramState,
) -> Option<&EyePersistenceCache> {
    let key = persistence_cache_key(plot_rect, state)?;
    let cache_hit = state
        .persistence_cache
        .as_ref()
        .map(|cache| cache.key == key)
        .unwrap_or(false);
    if !cache_hit {
        let rebuilt = build_persistence_cache(state, key)?;
        state.persistence_cache = Some(rebuilt);
    }
    state.persistence_cache.as_ref()
}

pub(super) fn persistence_cache_key(
    plot_rect: Rect,
    state: &EyeDiagramState,
) -> Option<EyePersistenceCacheKey> {
    let width = (plot_rect.width().round() as usize).clamp(96, 480);
    let height = (plot_rect.height().round() as usize).clamp(72, 320);
    if width == 0 || height == 0 {
        return None;
    }

    Some(EyePersistenceCacheKey {
        width,
        height,
        trace_count: state.data.traces.len(),
        total_points: state.data.total_points(),
        ui_count: state.ui_count.max(1),
        time_min_s: state.view.time_min_s,
        time_max_s: state.view.time_max_s,
        voltage_min: state.view.voltage_min,
        voltage_max: state.view.voltage_max,
        decay_quantized: (state.persistence_decay.clamp(0.0, 1.0) * 1000.0).round() as u16,
    })
}

pub(super) fn build_persistence_cache(
    state: &EyeDiagramState,
    key: EyePersistenceCacheKey,
) -> Option<EyePersistenceCache> {
    let grid = build_persistence_grid(state, key.width, key.height)?;
    Some(EyePersistenceCache {
        key,
        width: grid.width,
        height: grid.height,
        nonzero_bins: grid.nonzero_bins,
        max_count: grid.max_count,
    })
}

fn build_persistence_grid(
    state: &EyeDiagramState,
    width: usize,
    height: usize,
) -> Option<PersistenceGrid> {
    let data = &state.data;
    if data.traces.is_empty() {
        return None;
    }

    let mut counts = vec![0u32; width * height];
    let mut max_count = 0u32;
    let (time_min, time_max) = eye_view_time_bounds(state);
    let time_range = (time_max - time_min).max(1e-18);
    if !time_range.is_finite() || time_range <= 0.0 {
        return None;
    }
    let (v_min, v_max) = eye_view_voltage_bounds(state);
    let v_range = (v_max - v_min).max(1e-18);

    for trace in &data.traces {
        let n = trace.time.len().min(trace.amplitude.len());
        for i in 0..n {
            let t = trace.time[i];
            let v = trace.amplitude[i];
            if !t.is_finite() || !v.is_finite() {
                continue;
            }

            let time_seconds = trace_time_to_seconds(state, t);
            let x_norm = ((time_seconds - time_min) / time_range).clamp(0.0, 1.0);
            let y_norm = ((v_max - v) / v_range).clamp(0.0, 1.0);

            let xi = ((x_norm * (width as f64 - 1.0)).round() as usize).min(width - 1);
            let yi = ((y_norm * (height as f64 - 1.0)).round() as usize).min(height - 1);
            let idx = yi * width + xi;
            counts[idx] = counts[idx].saturating_add(1);
            max_count = max_count.max(counts[idx]);
        }
    }

    if max_count == 0 {
        return None;
    }

    let mut nonzero_bins = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let count = counts[y * width + x];
            if count > 0 {
                nonzero_bins.push((x, y, count));
            }
        }
    }

    Some(PersistenceGrid {
        width,
        height,
        nonzero_bins,
        max_count,
    })
}

pub(super) fn render_single_trace(painter: &Painter, rect: Rect, state: &EyeDiagramState) {
    let data = &state.data;

    if let Some(idx) = state.selected_trace {
        if let Some(trace) = data.traces.get(idx) {
            let stroke = Stroke::new(2.0, highlight_color());
            render_single_eye_trace(painter, rect, state, trace, stroke);
        }
    } else if let Some(trace) = data.traces.first() {
        let stroke = Stroke::new(2.0, trace_color());
        render_single_eye_trace(painter, rect, state, trace, stroke);
    }
}

pub(super) fn render_single_eye_trace(
    painter: &Painter,
    rect: Rect,
    state: &EyeDiagramState,
    trace: &EyeTrace,
    stroke: Stroke,
) {
    let n = trace.time.len().min(trace.amplitude.len());

    if n < 2 {
        return;
    }

    let mut prev: Option<Pos2> = None;
    for i in 0..n {
        let t = trace.time[i];
        let v = trace.amplitude[i];
        if !t.is_finite() || !v.is_finite() {
            prev = None;
            continue;
        }

        let time_seconds = trace_time_to_seconds(state, t);
        let point = Pos2::new(
            eye_time_to_x(time_seconds, rect, state),
            eye_voltage_to_y(v, rect, state),
        );

        if let Some(start) = prev
            && let Some([clipped_start, clipped_end]) =
                clip_line_segment_to_rect(start, point, rect)
        {
            painter.line_segment([clipped_start, clipped_end], stroke);
        }
        prev = Some(point);
    }
}

pub(super) fn clip_line_segment_to_rect(start: Pos2, end: Pos2, rect: Rect) -> Option<[Pos2; 2]> {
    let mut t0 = 0.0f32;
    let mut t1 = 1.0f32;
    let dx = end.x - start.x;
    let dy = end.y - start.y;

    let checks = [
        (-dx, start.x - rect.min.x),
        (dx, rect.max.x - start.x),
        (-dy, start.y - rect.min.y),
        (dy, rect.max.y - start.y),
    ];

    for (p, q) in checks {
        if p.abs() <= f32::EPSILON {
            if q < 0.0 {
                return None;
            }
            continue;
        }

        let r = q / p;
        if p < 0.0 {
            if r > t1 {
                return None;
            }
            if r > t0 {
                t0 = r;
            }
        } else if p > 0.0 {
            if r < t0 {
                return None;
            }
            if r < t1 {
                t1 = r;
            }
        }
    }

    if t0 > t1 {
        return None;
    }

    Some([
        Pos2::new(start.x + t0 * dx, start.y + t0 * dy),
        Pos2::new(start.x + t1 * dx, start.y + t1 * dy),
    ])
}
