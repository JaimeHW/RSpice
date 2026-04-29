use super::*;

pub(super) fn render_trace(
    painter: &Painter,
    layout: &ViewerLayout,
    viewer_state: &WaveformViewerState,
    trace: &TraceData,
    clip: Rect,
) {
    if trace.is_empty() {
        return;
    }

    let view = &viewer_state.view;
    TRACE_RENDER_SCRATCH.with(|scratch| {
        let mut scratch = scratch.borrow_mut();
        let visible_samples = build_trace_polyline_in_scratch(layout, view, trace, &mut scratch);
        if scratch.points.len() < 2 {
            return;
        }

        let color = trace.style.to_color32();
        let width = if trace.highlighted {
            trace.style.width * 2.0
        } else {
            trace.style.width
        };
        let stroke = Stroke::new(width, color);
        let clipped_painter = painter.with_clip_rect(clip);
        clipped_painter.add(Shape::line(scratch.points.clone(), stroke));

        if trace.style.show_markers && visible_samples <= 200 {
            for point in scratch.points.iter().copied() {
                clipped_painter.circle_filled(point, trace.style.marker_size / 2.0, color);
            }
        }
    });
}

#[derive(Debug, Clone, Copy)]
struct TraceScreenSample {
    sample_index: usize,
    data_y: f64,
    pos: Pos2,
}

#[derive(Debug, Clone, Default)]
struct TraceBucket {
    first: Option<TraceScreenSample>,
    last: Option<TraceScreenSample>,
    min: Option<TraceScreenSample>,
    max: Option<TraceScreenSample>,
}

#[derive(Debug, Default)]
struct TraceRenderScratch {
    points: Vec<Pos2>,
    buckets: Vec<TraceBucket>,
}

thread_local! {
    static TRACE_RENDER_SCRATCH: RefCell<TraceRenderScratch> =
        RefCell::new(TraceRenderScratch::default());
}

fn trace_screen_pos(
    layout: &ViewerLayout,
    view: &ViewTransform,
    data_x: f64,
    data_y: f64,
) -> Option<Pos2> {
    if !data_x.is_finite() || !data_y.is_finite() {
        return None;
    }
    if view.x_range() <= 0.0 || view.y_range() <= 0.0 {
        return None;
    }
    let screen_x =
        layout.plot.min.x + ((data_x - view.x_min) / view.x_range()) as f32 * layout.plot.width();
    let screen_y =
        layout.plot.min.y + ((view.y_max - data_y) / view.y_range()) as f32 * layout.plot.height();
    if !screen_x.is_finite() || !screen_y.is_finite() {
        return None;
    }
    Some(Pos2::new(screen_x, screen_y))
}

fn visible_trace_index_window(trace: &TraceData, view: &ViewTransform) -> Option<(usize, usize)> {
    if trace.is_empty() || trace.x.is_empty() || view.x_max <= view.x_min {
        return None;
    }

    let first_x = *trace.x.first()?;
    let last_x = *trace.x.last()?;
    if !first_x.is_finite() || !last_x.is_finite() {
        return Some((0, trace.len()));
    }
    if view.x_max < first_x || view.x_min > last_x {
        return None;
    }

    let start = trace
        .x
        .partition_point(|x| *x < view.x_min)
        .saturating_sub(1);
    let end = (trace.x.partition_point(|x| *x <= view.x_max) + 1).min(trace.len());
    if end <= start {
        return None;
    }
    Some((start, end))
}

fn should_render_trace_directly(plot_width_px: usize, visible_samples: usize) -> bool {
    let direct_budget =
        (plot_width_px * DIRECT_RENDER_POINTS_PER_PIXEL).max(DIRECT_RENDER_MIN_SAMPLES);
    visible_samples <= direct_budget.min(DECIMATION_THRESHOLD)
}

fn push_unique_point(points: &mut Vec<Pos2>, point: Pos2) {
    if points.last().copied() == Some(point) {
        return;
    }
    points.push(point);
}

fn update_trace_bucket(bucket: &mut TraceBucket, sample: TraceScreenSample) {
    if bucket
        .first
        .map(|existing| sample.sample_index < existing.sample_index)
        .unwrap_or(true)
    {
        bucket.first = Some(sample);
    }
    if bucket
        .last
        .map(|existing| sample.sample_index > existing.sample_index)
        .unwrap_or(true)
    {
        bucket.last = Some(sample);
    }
    if bucket
        .min
        .map(|existing| sample.data_y < existing.data_y)
        .unwrap_or(true)
    {
        bucket.min = Some(sample);
    }
    if bucket
        .max
        .map(|existing| sample.data_y > existing.data_y)
        .unwrap_or(true)
    {
        bucket.max = Some(sample);
    }
}

fn collect_bucket_points(points: &mut Vec<Pos2>, bucket: &TraceBucket) {
    let mut pending = [bucket.first, bucket.min, bucket.max, bucket.last];
    let mut last_index = None;

    for _ in 0..pending.len() {
        let mut selected_slot = None;
        for (slot, sample) in pending.iter().enumerate() {
            let Some(sample) = sample else {
                continue;
            };
            let replace = selected_slot
                .map(|existing: usize| {
                    sample.sample_index < pending[existing].unwrap().sample_index
                })
                .unwrap_or(true);
            if replace {
                selected_slot = Some(slot);
            }
        }

        let Some(slot) = selected_slot else {
            break;
        };
        let Some(sample) = pending[slot].take() else {
            continue;
        };
        if last_index == Some(sample.sample_index) {
            continue;
        }
        last_index = Some(sample.sample_index);
        push_unique_point(points, sample.pos);
    }
}

fn build_trace_polyline_in_scratch(
    layout: &ViewerLayout,
    view: &ViewTransform,
    trace: &TraceData,
    scratch: &mut TraceRenderScratch,
) -> usize {
    let Some((start, end)) = visible_trace_index_window(trace, view) else {
        scratch.points.clear();
        return 0;
    };

    let visible_samples = end.saturating_sub(start);
    if visible_samples == 0 {
        scratch.points.clear();
        return 0;
    }

    let plot_width_px = layout.plot.width().max(1.0).ceil() as usize;
    scratch.points.clear();

    if should_render_trace_directly(plot_width_px, visible_samples) {
        scratch.points.reserve(visible_samples);
        for idx in start..end {
            let Some((&x, &y)) = trace.x.get(idx).zip(trace.y.get(idx)) else {
                continue;
            };
            if let Some(pos) = trace_screen_pos(layout, view, x, y) {
                push_unique_point(&mut scratch.points, pos);
            }
        }
        return visible_samples;
    }

    let bucket_count = plot_width_px.max(1);
    scratch.buckets.clear();
    scratch.buckets.resize(bucket_count, TraceBucket::default());

    for idx in start..end {
        let Some((&x, &y)) = trace.x.get(idx).zip(trace.y.get(idx)) else {
            continue;
        };
        let Some(pos) = trace_screen_pos(layout, view, x, y) else {
            continue;
        };

        // Ignore samples far outside the clip lane to avoid skewing bucket picks.
        if pos.x < layout.plot.min.x - 1.0 || pos.x > layout.plot.max.x + 1.0 {
            continue;
        }

        let bucket_index = ((pos.x - layout.plot.min.x).floor() as isize)
            .clamp(0, bucket_count as isize - 1) as usize;
        update_trace_bucket(
            &mut scratch.buckets[bucket_index],
            TraceScreenSample {
                sample_index: idx,
                data_y: y,
                pos,
            },
        );
    }

    scratch.points.reserve(bucket_count * 2);
    for bucket in &scratch.buckets {
        collect_bucket_points(&mut scratch.points, bucket);
    }

    visible_samples
}
