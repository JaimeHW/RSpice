//! Trace decimation — per-pixel-column min/max envelopes, cached.
//!
//! Stroking a million-point transient directly would tessellate a million
//! segments per frame. Instead each trace is reduced to at most two points
//! per pixel column (the column's min and max, in encounter order, which
//! preserves the drawn envelope exactly). The reduction is a single O(n)
//! pass and is cached keyed by (data identity, column count, x-range), so a
//! steady view costs only the cheap data→screen transform per frame.

use std::collections::HashMap;
use std::sync::Arc;

use super::scale::XScale;

/// Viewer-only sampling policy. None of these modes mutate or replace the
/// source waveform arrays.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum DisplayDecimation {
    #[default]
    EnvelopeExtrema,
    Uniform,
    FullResolution,
}

/// Evaluation method used for cursor readouts between accepted source points.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SampleInterpolation {
    #[default]
    MonotoneCubic,
    Linear,
    Nearest,
}

/// Cache of decimated envelopes. Owned by the results workspace state;
/// cleared wholesale when the data version changes.
#[derive(Debug, Clone)]
pub struct DecimationCache {
    map: HashMap<CacheKey, Entry>,
    /// Data version the cache contents belong to.
    version: u64,
    /// Frame tick, advanced once per frame by `ensure_version` — eviction
    /// keeps entries the current frame touched instead of nuking the hot
    /// working set.
    tick: u64,
    /// Hard resident-memory budget for reconstructable display envelopes.
    byte_capacity: usize,
    resident_bytes: usize,
}

#[derive(Debug, Clone)]
struct Entry {
    envelope: Arc<[[f64; 2]]>,
    last_used: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct CacheKey {
    trace: u64,
    mode: DisplayDecimation,
    columns: u32,
    x0_bits: u64,
    x1_bits: u64,
    /// Screen quantization of the Y axis. Only a parametric reduction reads
    /// the Y projection, so a monotone trace deliberately keys these to zero:
    /// including a live autoscale bound would invalidate every envelope each
    /// time a pane refits, which is exactly the cost this cache exists to
    /// avoid.
    rows: u32,
    y0_bits: u64,
    y1_bits: u64,
    parametric: bool,
}

/// The data→screen projection a decimated series is quantized against.
///
/// This mirrors `render`'s `mx`/`my` exactly. A reduction is only valid for
/// the projection that produced it, so every field belongs in the cache key.
#[derive(Debug, Clone, Copy)]
pub struct TraceView {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub x_scale: XScale,
    pub y_scale: XScale,
    pub columns: usize,
    pub rows: usize,
}

/// Entry-count bound; old views evict once it's exceeded.
const CACHE_CAP: usize = 256;

/// Default resident budget for reconstructable display envelopes.
///
/// Every entry here is derived data that costs one O(n) pass to rebuild, so
/// the budget buys latency, not correctness. A workstation can spare half a
/// gigabyte for it; a phone browser cannot — the whole WebAssembly heap is
/// often smaller than that, and the retained waveforms have to live in it too.
#[cfg(not(target_arch = "wasm32"))]
pub const DEFAULT_DISPLAY_CACHE_MIB: u32 = 512;
#[cfg(target_arch = "wasm32")]
pub const DEFAULT_DISPLAY_CACHE_MIB: u32 = 96;

const DEFAULT_BYTE_CAPACITY: usize = DEFAULT_DISPLAY_CACHE_MIB as usize * 1024 * 1024;

impl Default for DecimationCache {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
            version: 0,
            tick: 0,
            byte_capacity: DEFAULT_BYTE_CAPACITY,
            resident_bytes: 0,
        }
    }
}

impl DecimationCache {
    /// Drop everything if `version` differs from the cached one (new run /
    /// new analysis selection), and advance the frame tick.
    pub fn ensure_version(&mut self, version: u64) {
        if self.version != version {
            self.map.clear();
            self.resident_bytes = 0;
            self.version = version;
        }
        self.tick = self.tick.wrapping_add(1);
    }

    /// Configure the hard resident cache budget. Existing least-recently-used
    /// entries are evicted immediately when the budget shrinks.
    pub fn set_memory_budget_mib(&mut self, mebibytes: u32) {
        self.byte_capacity = (mebibytes.clamp(64, 16_384) as usize).saturating_mul(1024 * 1024);
        self.evict_to_fit(0);
    }

    fn evict_to_fit(&mut self, incoming_bytes: usize) {
        while (!self.map.is_empty())
            && (self.map.len() >= CACHE_CAP
                || self.resident_bytes.saturating_add(incoming_bytes) > self.byte_capacity)
        {
            let Some(oldest) = self
                .map
                .iter()
                .min_by_key(|(_, entry)| entry.last_used)
                .map(|(key, _)| *key)
            else {
                break;
            };
            if let Some(removed) = self.map.remove(&oldest) {
                self.resident_bytes = self
                    .resident_bytes
                    .saturating_sub(removed.envelope.len() * size_of::<[f64; 2]>());
            }
        }
    }

    /// Fetch or compute a bounded display series for one trace at the given
    /// view. Full-resolution rendering deliberately bypasses this cache.
    ///
    /// `parametric` selects the reduction, and it is a property of the data,
    /// never of the user's display preference: a locus revisits X values, so
    /// both the X-ordered reductions below would be answering a question it
    /// has no single answer to. See [`decimate_pixel_cells`].
    pub fn series(
        &mut self,
        mode: DisplayDecimation,
        trace_key: u64,
        x: &[f64],
        y: &[f64],
        view: TraceView,
        parametric: bool,
    ) -> Arc<[[f64; 2]]> {
        debug_assert!(!matches!(mode, DisplayDecimation::FullResolution));
        let key = CacheKey {
            trace: trace_key,
            mode,
            columns: view.columns as u32,
            x0_bits: view.x0.to_bits(),
            x1_bits: view.x1.to_bits(),
            rows: if parametric { view.rows as u32 } else { 0 },
            y0_bits: if parametric { view.y0.to_bits() } else { 0 },
            y1_bits: if parametric { view.y1.to_bits() } else { 0 },
            parametric,
        };
        if let Some(hit) = self.map.get_mut(&key) {
            hit.last_used = self.tick;
            return Arc::clone(&hit.envelope);
        }
        let envelope: Arc<[[f64; 2]]> = if parametric {
            decimate_pixel_cells(x, y, view)
        } else {
            match mode {
                DisplayDecimation::EnvelopeExtrema => {
                    decimate_minmax(x, y, view.x0, view.x1, view.x_scale, view.columns)
                }
                DisplayDecimation::Uniform => {
                    decimate_uniform(x, y, view.x0, view.x1, view.columns)
                }
                DisplayDecimation::FullResolution => {
                    unreachable!("full resolution bypasses cache")
                }
            }
        }
        .into();
        let bytes = envelope.len() * size_of::<[f64; 2]>();
        if bytes <= self.byte_capacity {
            self.evict_to_fit(bytes);
            self.map.insert(
                key,
                Entry {
                    envelope: Arc::clone(&envelope),
                    last_used: self.tick,
                },
            );
            self.resident_bytes = self.resident_bytes.saturating_add(bytes);
        }
        envelope
    }
}

/// Uniformly sample the visible source range to at most `columns` points,
/// retaining both visible endpoints. This is intentionally a presentation
/// alternative to the extrema-preserving envelope.
pub fn decimate_uniform(x: &[f64], y: &[f64], x0: f64, x1: f64, columns: usize) -> Vec<[f64; 2]> {
    let n = x.len().min(y.len());
    if n == 0 || columns == 0 || !matches!(x1.partial_cmp(&x0), Some(std::cmp::Ordering::Greater)) {
        return Vec::new();
    }
    let start = x[..n]
        .partition_point(|&value| value < x0)
        .saturating_sub(1);
    let end = (x[..n].partition_point(|&value| value <= x1) + 1).min(n);
    let visible = end.saturating_sub(start);
    if columns == 1 {
        return vec![[x[start], y[start]]];
    }
    if visible <= columns {
        return (start..end).map(|index| [x[index], y[index]]).collect();
    }
    let last = visible - 1;
    (0..columns)
        .map(|slot| {
            let offset = slot * last / (columns - 1);
            let index = start + offset;
            [x[index], y[index]]
        })
        .collect()
}

/// Reduce `(x, y)` to a per-column min/max envelope over `[x0, x1]` split
/// into `columns` equal screen columns. Returns the raw points when the
/// range already holds fewer than `2 × columns` samples.
///
/// # Precondition
///
/// `x` must be non-decreasing. Both the visible-window binary search and the
/// per-column reduction below depend on it, and neither fails loudly without
/// it — a locus fed through here loses whichever branch falls outside the
/// contiguous index window. Parametric data belongs in
/// [`decimate_pixel_cells`].
pub fn decimate_minmax(
    x: &[f64],
    y: &[f64],
    x0: f64,
    x1: f64,
    x_scale: XScale,
    columns: usize,
) -> Vec<[f64; 2]> {
    let n = x.len().min(y.len());
    if n == 0 || columns == 0 || !matches!(x1.partial_cmp(&x0), Some(std::cmp::Ordering::Greater)) {
        return Vec::new();
    }
    // Visible index range (x is sorted ascending).
    let start = x[..n].partition_point(|&v| v < x0);
    let end = x[..n].partition_point(|&v| v <= x1);
    // Keep one sample of margin so strokes continue to the plot edge.
    let start = start.saturating_sub(1);
    let end = (end + 1).min(n);
    let visible = end - start;

    if visible <= columns * 2 {
        return (start..end).map(|i| [x[i], y[i]]).collect();
    }

    // Past this point the ordering decides the output, so a violation stops
    // being a small inaccuracy and starts deleting whole branches of the
    // curve. Non-finite abscissae are ordinary — a diverged run carries them
    // and `finite_runs` breaks the stroke around them downstream — so the
    // precondition is only about the samples that do carry a position.
    debug_assert!(
        x[start..end]
            .iter()
            .copied()
            .filter(|value| value.is_finite())
            .try_fold(f64::NEG_INFINITY, |previous, value| (previous <= value)
                .then_some(value))
            .is_some(),
        "decimate_minmax requires non-decreasing x; a locus belongs in decimate_pixel_cells"
    );

    let mut out: Vec<[f64; 2]> = Vec::with_capacity(columns * 2 + 4);
    let mut column = 0usize;
    let mut min_v = f64::INFINITY;
    let mut max_v = f64::NEG_INFINITY;
    let mut min_i = 0usize;
    let mut max_i = 0usize;
    let mut col_x = 0.0f64;
    let mut has = false;

    let flush = |out: &mut Vec<[f64; 2]>,
                 col_x: f64,
                 min_v: f64,
                 max_v: f64,
                 min_i: usize,
                 max_i: usize| {
        // Emit min/max in encounter order so the stroke zig-zags the way the
        // raw data would, not always downward.
        if min_i <= max_i {
            out.push([col_x, min_v]);
            if max_i != min_i {
                out.push([col_x, max_v]);
            }
        } else {
            out.push([col_x, max_v]);
            out.push([col_x, min_v]);
        }
    };

    for i in start..end {
        let t = x_scale.normalize(x[i], x0, x1).clamp(0.0, 1.0);
        let c = ((t * columns as f64) as usize).min(columns - 1);
        if c != column && has {
            flush(&mut out, col_x, min_v, max_v, min_i, max_i);
            min_v = f64::INFINITY;
            max_v = f64::NEG_INFINITY;
        }
        column = c;
        if y[i] < min_v {
            min_v = y[i];
            min_i = i;
        }
        if y[i] > max_v {
            max_v = y[i];
            max_i = i;
        }
        col_x = x[i];
        has = true;
    }
    if has {
        flush(&mut out, col_x, min_v, max_v, min_i, max_i);
    }
    out
}

/// Screen cell index of one sample under `view`'s projection.
///
/// Deliberately unclamped, so a sample outside the viewport keeps a distinct
/// cell and the path still enters and leaves the plot where the data does.
/// The normalized coordinate is bounded before the cast because an outlier
/// far beyond the window would otherwise overflow the index; anything that
/// far off screen is indistinguishable from anything else that far off.
fn screen_cell(x: f64, y: f64, view: TraceView) -> (i64, i64) {
    const LIMIT: f64 = 1.0e9;
    let column = (view
        .x_scale
        .normalize(x, view.x0, view.x1)
        .clamp(-LIMIT, LIMIT)
        * view.columns as f64) as i64;
    let row = (view
        .y_scale
        .normalize(y, view.y0, view.y1)
        .clamp(-LIMIT, LIMIT)
        * view.rows as f64) as i64;
    (column, row)
}

/// Reduce a parametric series — one whose X is not monotone — to at most a
/// few points per screen cell, in source order.
///
/// A locus (a Smith or Nyquist curve) revisits X values as the sweep runs, so
/// neither a binary search for the visible index window nor a per-X-column
/// min/max envelope describes it: the first assumes the visible samples are
/// contiguous in index order, and the second assumes one column holds one
/// span of the curve. A circle violates both, and the failure is silent —
/// whichever branch falls outside the contiguous window simply is not drawn.
///
/// This walks the samples in the order the sweep produced them, projects each
/// to an integer screen cell, and keeps a sample whenever the cell changes,
/// plus the last sample of the run it is leaving. The drawn path therefore
/// follows the sweep, every visible excursion survives, and the output is
/// bounded by the number of distinct cells the curve actually visits.
///
/// Non-finite samples pass through unchanged so `render`'s `finite_runs`
/// still splits the stroke exactly where the data stops.
pub fn decimate_pixel_cells(x: &[f64], y: &[f64], view: TraceView) -> Vec<[f64; 2]> {
    let n = x.len().min(y.len());
    if n == 0
        || view.columns == 0
        || view.rows == 0
        || !matches!(
            view.x1.partial_cmp(&view.x0),
            Some(std::cmp::Ordering::Greater)
        )
        || !matches!(
            view.y1.partial_cmp(&view.y0),
            Some(std::cmp::Ordering::Greater)
        )
    {
        return Vec::new();
    }

    let mut out: Vec<[f64; 2]> = Vec::new();
    let mut current: Option<(i64, i64)> = None;
    // The most recent sample of the cell run in progress. Emitting it as the
    // run ends keeps the segment leaving a cell anchored where the curve
    // actually was, not where it first arrived.
    let mut trailing: Option<[f64; 2]> = None;

    for index in 0..n {
        let (xv, yv) = (x[index], y[index]);
        if !xv.is_finite() || !yv.is_finite() {
            if let Some(point) = trailing.take() {
                out.push(point);
            }
            out.push([xv, yv]);
            current = None;
            continue;
        }
        let cell = screen_cell(xv, yv, view);
        if current == Some(cell) {
            trailing = Some([xv, yv]);
            continue;
        }
        if let Some(point) = trailing.take() {
            out.push(point);
        }
        out.push([xv, yv]);
        current = Some(cell);
    }
    if let Some(point) = trailing.take() {
        out.push(point);
    }
    out
}

/// Linearly interpolated sample of `(x, y)` at `xq` (x sorted ascending).
/// Clamps outside the data range.
pub fn sample_at(x: &[f64], y: &[f64], xq: f64) -> f64 {
    sample_at_with(x, y, xq, SampleInterpolation::Linear)
}

/// Evaluate a source series at `xq` using the selected cursor policy. Values
/// outside the accepted range clamp to the first/last accepted sample.
pub fn sample_at_with(x: &[f64], y: &[f64], xq: f64, interpolation: SampleInterpolation) -> f64 {
    let n = x.len().min(y.len());
    if n == 0 {
        return 0.0;
    }
    if xq <= x[0] {
        return y[0];
    }
    if xq >= x[n - 1] {
        return y[n - 1];
    }
    let hi = x[..n].partition_point(|&v| v < xq).max(1);
    let lo = hi - 1;
    let span = x[hi] - x[lo];
    if span <= 0.0 {
        return y[lo];
    }
    if matches!(interpolation, SampleInterpolation::Nearest) {
        return if xq - x[lo] <= x[hi] - xq {
            y[lo]
        } else {
            y[hi]
        };
    }
    let t = (xq - x[lo]) / span;
    let linear = y[lo] + t * (y[hi] - y[lo]);
    if !matches!(interpolation, SampleInterpolation::MonotoneCubic) || n < 3 {
        return linear;
    }
    let Some(m0) = monotone_slope(x, y, n, lo) else {
        return linear;
    };
    let Some(m1) = monotone_slope(x, y, n, hi) else {
        return linear;
    };
    let t2 = t * t;
    let t3 = t2 * t;
    let value = (2.0 * t3 - 3.0 * t2 + 1.0) * y[lo]
        + (t3 - 2.0 * t2 + t) * span * m0
        + (-2.0 * t3 + 3.0 * t2) * y[hi]
        + (t3 - t2) * span * m1;
    if value.is_finite() { value } else { linear }
}

fn monotone_slope(x: &[f64], y: &[f64], n: usize, index: usize) -> Option<f64> {
    let secant = |left: usize, right: usize| {
        let h = x[right] - x[left];
        (h.is_finite() && h > 0.0 && y[left].is_finite() && y[right].is_finite())
            .then(|| (h, (y[right] - y[left]) / h))
    };
    if index == 0 {
        let (h0, d0) = secant(0, 1)?;
        let (h1, d1) = secant(1, 2)?;
        return Some(endpoint_slope(h0, h1, d0, d1));
    }
    if index + 1 == n {
        let (h0, d0) = secant(n - 2, n - 1)?;
        let (h1, d1) = secant(n - 3, n - 2)?;
        return Some(endpoint_slope(h0, h1, d0, d1));
    }
    let (h_previous, d_previous) = secant(index - 1, index)?;
    let (h_next, d_next) = secant(index, index + 1)?;
    if d_previous == 0.0 || d_next == 0.0 || d_previous.signum() != d_next.signum() {
        return Some(0.0);
    }
    let w1 = 2.0 * h_next + h_previous;
    let w2 = h_next + 2.0 * h_previous;
    Some((w1 + w2) / (w1 / d_previous + w2 / d_next))
}

fn endpoint_slope(h0: f64, h1: f64, d0: f64, d1: f64) -> f64 {
    let mut slope = ((2.0 * h0 + h1) * d0 - h0 * d1) / (h0 + h1);
    if slope.signum() != d0.signum() {
        slope = 0.0;
    } else if d0.signum() != d1.signum() && slope.abs() > 3.0 * d0.abs() {
        slope = 3.0 * d0;
    }
    slope
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_data_passes_through() {
        let x: Vec<f64> = (0..10).map(|i| i as f64).collect();
        let y: Vec<f64> = (0..10).map(|i| (i * i) as f64).collect();
        let env = decimate_minmax(&x, &y, 0.0, 9.0, XScale::Linear, 100);
        assert_eq!(env.len(), 10);
    }

    #[test]
    fn dense_data_bounded_by_columns() {
        let n = 100_000;
        let x: Vec<f64> = (0..n).map(|i| i as f64 / n as f64).collect();
        let y: Vec<f64> = (0..n).map(|i| (i as f64 * 0.1).sin()).collect();
        let env = decimate_minmax(&x, &y, 0.0, 1.0, XScale::Linear, 800);
        assert!(env.len() <= 800 * 2 + 4, "len = {}", env.len());
        // Envelope must preserve the global extremes.
        let max = env.iter().map(|p| p[1]).fold(f64::MIN, f64::max);
        assert!(max > 0.999);
    }

    #[test]
    fn interpolation() {
        let x = [0.0, 1.0, 2.0];
        let y = [0.0, 10.0, 20.0];
        assert!((sample_at(&x, &y, 0.5) - 5.0).abs() < 1e-12);
        assert!((sample_at(&x, &y, 5.0) - 20.0).abs() < 1e-12);
    }

    #[test]
    fn cursor_interpolation_modes_are_distinct_and_monotone_is_shape_preserving() {
        let x = [0.0, 1.0, 2.0, 3.0];
        let y = [0.0, 1.0, 1.5, 1.75];
        let nearest = sample_at_with(&x, &y, 1.25, SampleInterpolation::Nearest);
        let linear = sample_at_with(&x, &y, 1.25, SampleInterpolation::Linear);
        let cubic = sample_at_with(&x, &y, 1.25, SampleInterpolation::MonotoneCubic);
        assert_eq!(nearest, 1.0);
        assert_eq!(linear, 1.125);
        assert!((1.0..=1.5).contains(&cubic));
        assert_ne!(cubic, linear);
    }

    /// A Smith/Nyquist locus at a sweep density that decimates: 4001 samples
    /// around a circle, the shape `smith.rs` and `nyquist.rs` actually hand
    /// the renderer.
    fn locus(samples: usize) -> (Vec<f64>, Vec<f64>) {
        let angle = |i: usize| i as f64 / (samples - 1) as f64 * std::f64::consts::TAU;
        (
            (0..samples).map(|i| angle(i).cos() * 0.9).collect(),
            (0..samples).map(|i| angle(i).sin() * 0.9).collect(),
        )
    }

    fn locus_view(x0: f64, x1: f64) -> TraceView {
        TraceView {
            x0,
            x1,
            y0: -1.12,
            y1: 1.12,
            x_scale: XScale::Linear,
            y_scale: XScale::Linear,
            columns: 1216,
            rows: 704,
        }
    }

    #[test]
    fn a_zoomed_locus_keeps_every_branch_that_crosses_the_window() {
        // The defect this pins: the X-ordered reduction finds its visible
        // window with a binary search, so on a locus it returned one
        // contiguous index run and the other branch — half the curve — was
        // silently not drawn. Each of these windows is crossed twice.
        let (re, im) = locus(4001);
        for (x0, x1) in [(0.2, 0.5), (-0.6, -0.2), (0.0, 0.3)] {
            let drawn = decimate_pixel_cells(&re, &im, locus_view(x0, x1));
            let upper = drawn
                .iter()
                .filter(|point| (x0..=x1).contains(&point[0]) && point[1] > 0.0)
                .count();
            let lower = drawn
                .iter()
                .filter(|point| (x0..=x1).contains(&point[0]) && point[1] < 0.0)
                .count();
            assert!(
                upper > 0 && lower > 0,
                "window [{x0}, {x1}] drew upper={upper} lower={lower}"
            );
        }
    }

    #[test]
    fn a_locus_reduction_stays_bounded_and_ordered() {
        let (re, im) = locus(40_001);
        let view = locus_view(-1.12, 1.12);
        let drawn = decimate_pixel_cells(&re, &im, view);
        assert!(drawn.len() < re.len() / 4, "no reduction: {}", drawn.len());
        // Source order, not X order: consecutive output points must stay
        // within a cell step of each other, which an X-sorted reduction of a
        // circle could not satisfy.
        for pair in drawn.windows(2) {
            let (a, b) = (
                screen_cell(pair[0][0], pair[0][1], view),
                screen_cell(pair[1][0], pair[1][1], view),
            );
            assert!(
                (a.0 - b.0).abs() <= 1 && (a.1 - b.1).abs() <= 1,
                "path jumped from {a:?} to {b:?}"
            );
        }
    }

    #[test]
    fn a_locus_preserves_the_gaps_in_its_own_data() {
        let mut re = vec![0.0, 0.1, 0.2, 0.3, 0.4];
        let im = vec![0.0, 0.1, 0.2, 0.3, 0.4];
        re[2] = f64::NAN;
        let drawn = decimate_pixel_cells(&re, &im, locus_view(-1.12, 1.12));
        assert_eq!(
            drawn.iter().filter(|point| !point[0].is_finite()).count(),
            1,
            "the hole must survive so finite_runs can split the stroke"
        );
    }

    #[test]
    fn a_degenerate_locus_window_draws_nothing_rather_than_guessing() {
        let (re, im) = locus(64);
        let mut view = locus_view(0.0, 0.0);
        assert!(decimate_pixel_cells(&re, &im, view).is_empty());
        view = locus_view(-1.0, 1.0);
        view.rows = 0;
        assert!(decimate_pixel_cells(&re, &im, view).is_empty());
        assert!(decimate_pixel_cells(&[], &[], locus_view(-1.0, 1.0)).is_empty());
    }

    #[test]
    fn uniform_decimation_retains_visible_endpoints() {
        let x = (0..100).map(f64::from).collect::<Vec<_>>();
        let y = x.iter().map(|value| value * value).collect::<Vec<_>>();
        let points = decimate_uniform(&x, &y, 10.0, 89.0, 8);
        assert_eq!(points.len(), 8);
        assert_eq!(points.first().copied(), Some([9.0, 81.0]));
        assert_eq!(points.last().copied(), Some([90.0, 8100.0]));
        assert_eq!(decimate_uniform(&x, &y, 10.0, 89.0, 1).len(), 1);
    }
}
