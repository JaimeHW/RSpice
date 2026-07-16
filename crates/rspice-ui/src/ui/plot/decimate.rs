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
#[derive(Debug, Default, Clone)]
pub struct DecimationCache {
    map: HashMap<CacheKey, Entry>,
    /// Data version the cache contents belong to.
    version: u64,
    /// Frame tick, advanced once per frame by `ensure_version` — eviction
    /// keeps entries the current frame touched instead of nuking the hot
    /// working set.
    tick: u64,
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
}

/// Entry-count bound; old views evict once it's exceeded.
const CACHE_CAP: usize = 256;

impl DecimationCache {
    /// Drop everything if `version` differs from the cached one (new run /
    /// new analysis selection), and advance the frame tick.
    pub fn ensure_version(&mut self, version: u64) {
        if self.version != version {
            self.map.clear();
            self.version = version;
        }
        self.tick = self.tick.wrapping_add(1);
    }

    /// Fetch or compute a bounded display series for one trace at the given
    /// view. Full-resolution rendering deliberately bypasses this cache.
    #[allow(clippy::too_many_arguments)]
    pub fn series(
        &mut self,
        mode: DisplayDecimation,
        trace_key: u64,
        x: &[f64],
        y: &[f64],
        x0: f64,
        x1: f64,
        x_scale: XScale,
        columns: usize,
    ) -> Arc<[[f64; 2]]> {
        debug_assert!(!matches!(mode, DisplayDecimation::FullResolution));
        let key = CacheKey {
            trace: trace_key,
            mode,
            columns: columns as u32,
            x0_bits: x0.to_bits(),
            x1_bits: x1.to_bits(),
        };
        if let Some(hit) = self.map.get_mut(&key) {
            hit.last_used = self.tick;
            return Arc::clone(&hit.envelope);
        }
        // Resize/zoom mint new keys; evict stale views, keep this frame's.
        if self.map.len() > CACHE_CAP {
            let tick = self.tick;
            self.map.retain(|_, entry| entry.last_used == tick);
            if self.map.len() > CACHE_CAP {
                self.map.clear();
            }
        }
        let envelope: Arc<[[f64; 2]]> = match mode {
            DisplayDecimation::EnvelopeExtrema => decimate_minmax(x, y, x0, x1, x_scale, columns),
            DisplayDecimation::Uniform => decimate_uniform(x, y, x0, x1, columns),
            DisplayDecimation::FullResolution => unreachable!("full resolution bypasses cache"),
        }
        .into();
        self.map.insert(
            key,
            Entry {
                envelope: Arc::clone(&envelope),
                last_used: self.tick,
            },
        );
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
