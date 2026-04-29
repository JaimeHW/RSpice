use super::interactions::{freq_to_x_for_trace, mag_to_y};
use super::*;

pub(super) fn render_trace(painter: &egui::Painter, rect: Rect, data: &FftData, state: &FftState) {
    if data.is_empty() {
        return;
    }

    let stroke = Stroke::new(1.5, trace_color());
    let clipped_painter = painter.with_clip_rect(rect);
    FFT_RENDER_SCRATCH.with(|scratch| {
        let mut scratch = scratch.borrow_mut();
        build_spectrum_polyline_in_scratch(rect, data, state, &mut scratch);
        if scratch.points.len() < 2 {
            return;
        }
        clipped_painter.add(Shape::line(scratch.points.clone(), stroke));
    });
}

#[derive(Debug, Clone, Copy)]
struct SpectrumScreenSample {
    bin_index: usize,
    display_magnitude: f64,
    pos: Pos2,
}

#[derive(Debug, Clone, Default)]
struct SpectrumBucket {
    first: Option<SpectrumScreenSample>,
    last: Option<SpectrumScreenSample>,
    min: Option<SpectrumScreenSample>,
    max: Option<SpectrumScreenSample>,
}

#[derive(Debug, Default)]
struct SpectrumRenderScratch {
    points: Vec<Pos2>,
    buckets: Vec<SpectrumBucket>,
}

thread_local! {
    static FFT_RENDER_SCRATCH: RefCell<SpectrumRenderScratch> =
        RefCell::new(SpectrumRenderScratch::default());
}

fn visible_spectrum_index_window(data: &FftData, state: &FftState) -> Option<(usize, usize)> {
    if data.is_empty() || state.freq_max <= state.freq_min {
        return None;
    }

    let lower_bound = match state.freq_scale {
        FrequencyScale::Linear => state.freq_min,
        FrequencyScale::Log => state.freq_min.max(1e-12),
    };
    let upper_bound = state.freq_max;
    if upper_bound <= lower_bound {
        return None;
    }

    let start = data
        .points
        .partition_point(|point| point.frequency < lower_bound)
        .saturating_sub(1);
    let end = (data
        .points
        .partition_point(|point| point.frequency <= upper_bound)
        + 1)
    .min(data.points.len());

    (end > start).then_some((start, end))
}

fn should_render_spectrum_directly(plot_width_px: usize, visible_bins: usize) -> bool {
    let direct_budget =
        (plot_width_px * SPECTRUM_DIRECT_BINS_PER_PIXEL).max(SPECTRUM_DIRECT_MIN_BINS);
    visible_bins <= direct_budget.min(SPECTRUM_DECIMATION_THRESHOLD)
}

fn push_unique_spectrum_point(points: &mut Vec<Pos2>, point: Pos2) {
    if points.last().copied() == Some(point) {
        return;
    }
    points.push(point);
}

fn update_spectrum_bucket(bucket: &mut SpectrumBucket, sample: SpectrumScreenSample) {
    if bucket
        .first
        .map(|existing| sample.bin_index < existing.bin_index)
        .unwrap_or(true)
    {
        bucket.first = Some(sample);
    }
    if bucket
        .last
        .map(|existing| sample.bin_index > existing.bin_index)
        .unwrap_or(true)
    {
        bucket.last = Some(sample);
    }
    if bucket
        .min
        .map(|existing| sample.display_magnitude < existing.display_magnitude)
        .unwrap_or(true)
    {
        bucket.min = Some(sample);
    }
    if bucket
        .max
        .map(|existing| sample.display_magnitude > existing.display_magnitude)
        .unwrap_or(true)
    {
        bucket.max = Some(sample);
    }
}

fn collect_spectrum_bucket_points(points: &mut Vec<Pos2>, bucket: &SpectrumBucket) {
    let mut pending = [bucket.first, bucket.min, bucket.max, bucket.last];
    let mut last_index = None;

    for _ in 0..pending.len() {
        let mut selected_slot = None;
        for (slot, sample) in pending.iter().enumerate() {
            let Some(sample) = sample else {
                continue;
            };
            let replace = selected_slot
                .map(|existing: usize| sample.bin_index < pending[existing].unwrap().bin_index)
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
        if last_index == Some(sample.bin_index) {
            continue;
        }
        last_index = Some(sample.bin_index);
        push_unique_spectrum_point(points, sample.pos);
    }
}

fn build_spectrum_polyline_in_scratch(
    rect: Rect,
    data: &FftData,
    state: &FftState,
    scratch: &mut SpectrumRenderScratch,
) -> usize {
    let Some((start, end)) = visible_spectrum_index_window(data, state) else {
        scratch.points.clear();
        return 0;
    };

    let visible_bins = end.saturating_sub(start);
    if visible_bins == 0 {
        scratch.points.clear();
        return 0;
    }

    let plot_width_px = rect.width().max(1.0).ceil() as usize;
    scratch.points.clear();

    if should_render_spectrum_directly(plot_width_px, visible_bins) {
        scratch.points.reserve(visible_bins);
        for point in &data.points[start..end] {
            let Some(x) = freq_to_x_for_trace(point.frequency, rect, state) else {
                continue;
            };
            let y = mag_to_y(point, rect, state);
            if !(x.is_finite() && y.is_finite()) {
                continue;
            }
            if x < rect.min.x - 1.0 || x > rect.max.x + 1.0 {
                continue;
            }
            push_unique_spectrum_point(&mut scratch.points, Pos2::new(x, y));
        }
        return visible_bins;
    }

    let bucket_count = plot_width_px.max(1);
    scratch.buckets.clear();
    scratch
        .buckets
        .resize(bucket_count, SpectrumBucket::default());

    for (offset, point) in data.points[start..end].iter().enumerate() {
        let Some(x) = freq_to_x_for_trace(point.frequency, rect, state) else {
            continue;
        };
        let y = mag_to_y(point, rect, state);
        if !(x.is_finite() && y.is_finite()) {
            continue;
        }
        if x < rect.min.x - 1.0 || x > rect.max.x + 1.0 {
            continue;
        }

        let bucket_index =
            ((x - rect.min.x).floor() as isize).clamp(0, bucket_count as isize - 1) as usize;
        update_spectrum_bucket(
            &mut scratch.buckets[bucket_index],
            SpectrumScreenSample {
                bin_index: start + offset,
                display_magnitude: state.display_magnitude(point),
                pos: Pos2::new(x, y),
            },
        );
    }

    scratch.points.reserve(bucket_count * 2);
    for bucket in &scratch.buckets {
        collect_spectrum_bucket_points(&mut scratch.points, bucket);
    }

    visible_bins
}
