//! A printed trace holds what its device can draw.
//!
//! The screen draws a trace as the least and greatest value of each pixel
//! column; the page does the same at the resolution it is printed at. A
//! million-point transient printed point for point carried every sample into
//! the PDF and SVG and through the rasteriser, which needed gigabytes at
//! 150 dpi and was killed at 300 dpi. Kept to the first, least, greatest and
//! last sample of each column, it draws the same ink at a few points a column.

use rspice_results::visualization_document::{figure_points, tile_points};
use rspice_results::waveform::{ColumnTile, WaveformValues};

use super::AxisScale;

/// The resolution a printed plot is read and drawn at, in columns per inch
/// of its natural width: the finest a desktop printer lays ink at, so a
/// 300 dpi page draws each of its columns from two of these.
pub const PRINT_COLUMNS_PER_INCH: i64 = 600;

/// The columns a plot `plot_width_um` wide is read and drawn at.
#[must_use]
pub fn print_columns(plot_width_um: i64) -> usize {
    usize::try_from(plot_width_um.saturating_mul(PRINT_COLUMNS_PER_INCH) / 25_400)
        .unwrap_or(1)
        .max(1)
}

/// The indices of `points` a page `columns` columns wide over x in
/// `x0 ..= x1` draws: in each column the first, least, greatest and last
/// sample, in their order, so the line between two columns runs exactly as
/// it would through every sample.
///
/// A point whose y is NaN is kept, since it is the gap the line breaks at. A
/// run that steps back in x — a folded eye starts each unit interval again
/// at its left edge — starts its columns afresh, so no fold is merged into
/// another. Points left and right of the window keep their own extremes,
/// which is what a line clipped at the window's edge needs.
pub(crate) fn column_extremes(
    points: &[(f64, f64)],
    x0: f64,
    x1: f64,
    columns: usize,
) -> Vec<usize> {
    let span = x1 - x0;
    if points.len() <= 4 || columns == 0 || !(span.is_finite() && span > 0.0) {
        return (0..points.len()).collect();
    }
    let scale = columns as f64 / span;
    let column_of = |x: f64| -> i64 {
        if x < x0 {
            -1
        } else if x > x1 {
            columns as i64
        } else {
            (((x - x0) * scale) as i64).min(columns as i64 - 1)
        }
    };
    let mut kept = Vec::with_capacity(points.len().min(4 * columns + 8));
    let mut bucket: Option<Bucket> = None;
    let mut previous_x = f64::NEG_INFINITY;
    for (index, &(x, y)) in points.iter().enumerate() {
        if y.is_nan() || x < previous_x {
            if let Some(open) = bucket.take() {
                open.flush(&mut kept);
            }
            if y.is_nan() {
                kept.push(index);
                previous_x = x;
                continue;
            }
        }
        previous_x = x;
        let column = column_of(x);
        match &mut bucket {
            Some(open) if open.column == column => open.take(index, y),
            _ => {
                if let Some(open) = bucket.take() {
                    open.flush(&mut kept);
                }
                bucket = Some(Bucket::new(column, index, y));
            }
        }
    }
    if let Some(open) = bucket {
        open.flush(&mut kept);
    }
    kept
}

/// One column's first, least, greatest and last sample.
struct Bucket {
    column: i64,
    first: usize,
    least: (usize, f64),
    greatest: (usize, f64),
    last: usize,
}

impl Bucket {
    fn new(column: i64, index: usize, y: f64) -> Self {
        Self {
            column,
            first: index,
            least: (index, y),
            greatest: (index, y),
            last: index,
        }
    }

    fn take(&mut self, index: usize, y: f64) {
        if y < self.least.1 {
            self.least = (index, y);
        }
        if y > self.greatest.1 {
            self.greatest = (index, y);
        }
        self.last = index;
    }

    fn flush(self, kept: &mut Vec<usize>) {
        let mut indices = [self.first, self.least.0, self.greatest.0, self.last];
        indices.sort_unstable();
        let mut previous = None;
        for index in indices {
            if previous != Some(index) {
                kept.push(index);
                previous = Some(index);
            }
        }
    }
}

/// `y` over `x` as a printed plot reads it: over `window` (the whole run when
/// `None`) at `columns` columns, from the store's tiles for a stored window
/// and from the samples in the window for one in memory — never the whole
/// column of a stored run. Each point is a sample, or a column's least or
/// greatest value at the column's centre; NaN is an unavailable value.
pub(crate) fn print_read(
    x: &WaveformValues,
    y: &WaveformValues,
    scale: AxisScale,
    window: Option<(f64, f64)>,
    columns: usize,
) -> Vec<(f64, f64)> {
    if x.len() != y.len() || x.is_empty() || columns == 0 {
        return Vec::new();
    }
    let window = window.or_else(|| x.ends());
    if scale == AxisScale::Logarithmic
        && let Some((x0, x1)) = window
        && x0 > 0.0
        && x0 < x1
        && let Some(tile) = y.log_tile_over(x, x0, x1, columns)
    {
        return log_tile_points(tile, x0, x1);
    }
    if scale != AxisScale::Logarithmic
        && let Some((xs, ys)) = figure_points(x, y, window, columns)
    {
        return xs.into_iter().zip(ys).collect();
    }
    // An axis that cannot tile (an imported sweep that does not ascend, or a
    // decade axis over an in-memory run) is read over the window's rows;
    // the page keeps each column's extremes of those.
    let (first, end) = match window {
        Some((x0, x1)) if x.is_ascending_axis() => {
            let first = x.partition_point(|value| value < x0).saturating_sub(1);
            let end = (x.partition_point(|value| value <= x1) + 1).min(x.len());
            (first, end)
        }
        _ => (0, x.len()),
    };
    x.samples(first..end).zip(y.samples(first..end)).collect()
}

/// The points of a tile read on a decade axis: [`tile_points`] for its rows,
/// and each touched column's least and greatest value at its centre in
/// decades for an envelope.
fn log_tile_points(tile: ColumnTile, x0: f64, x1: f64) -> Vec<(f64, f64)> {
    match tile {
        tile @ ColumnTile::Samples { .. } => {
            let (xs, ys) = tile_points(tile, x0, x1);
            xs.into_iter().zip(ys).collect()
        }
        ColumnTile::Envelope { min, max, touched } => {
            let (l0, l1) = (x0.log10(), x1.log10());
            let width = (l1 - l0) / min.len() as f64;
            let mut points = Vec::with_capacity(2 * min.len());
            for column in (0..min.len()).filter(|&column| touched[column]) {
                let at = 10f64.powf(l0 + (column as f64 + 0.5) * width);
                if min[column].is_nan() {
                    points.push((at, f64::NAN));
                    continue;
                }
                points.push((at, min[column]));
                if max[column] != min[column] {
                    points.push((at, max[column]));
                }
            }
            points
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A column keeps its first, least, greatest and last sample, in order,
    /// and nothing else: a million samples over a page are a few per column.
    #[test]
    fn a_dense_trace_keeps_four_samples_a_column_in_their_order() {
        let points = (0..1_000_000)
            .map(|index| {
                let x = f64::from(index) / 1.0e6;
                (x, (x * 7919.0).sin())
            })
            .collect::<Vec<_>>();
        let kept = column_extremes(&points, 0.0, 1.0, 1_000);
        assert!(kept.len() <= 4 * 1_000, "{} kept", kept.len());
        assert!(kept.windows(2).all(|pair| pair[0] < pair[1]));
        let column = |index: usize| (points[index].0 * 1_000.0) as usize;
        for at in [0, 499, 999] {
            let inside = kept.iter().filter(|&&index| column(index) == at);
            let least = inside
                .clone()
                .map(|&index| points[index].1)
                .fold(f64::INFINITY, f64::min);
            let truth = points
                .iter()
                .filter(|point| (point.0 * 1_000.0) as usize == at)
                .map(|point| point.1)
                .fold(f64::INFINITY, f64::min);
            assert_eq!(least, truth, "column {at} keeps its least sample");
        }
    }

    /// A gap stays a gap, and a fold is not merged into the one before it.
    #[test]
    fn a_gap_and_a_fold_each_survive_the_columns() {
        let mut points = (0..100)
            .map(|index| (f64::from(index), f64::from(index)))
            .collect::<Vec<_>>();
        points.push((100.0, f64::NAN));
        points.extend((0..100).map(|index| (f64::from(index), -f64::from(index))));
        let kept = column_extremes(&points, 0.0, 100.0, 10);
        assert!(kept.contains(&100), "the unavailable sample is kept");
        assert!(kept.contains(&101), "the fold starts its own columns");
        assert!(kept.contains(&200), "the fold ends on its own last sample");
    }
}
