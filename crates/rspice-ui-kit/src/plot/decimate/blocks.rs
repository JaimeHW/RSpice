//! Block extrema over one retained trace.
//!
//! The X-ordered reduction reads every visible sample, so a pan or a zoom of
//! a long transient reads millions of samples per trace on every frame of the
//! gesture. A column's interior does not need that: its least and greatest
//! value are the least and greatest of the blocks inside it, and only the
//! ragged ends need reading sample by sample. The column boundaries are found
//! by bisection on the same predicate the reduction quantizes with, so the
//! envelope that comes out is the reference reduction's, point for point.

use super::super::sample::LogicalView;
use super::super::scale::XScale;

/// Samples per block. A column reads at most two partial blocks, so this
/// trades the index's size (about a twentieth of the trace's values) against
/// the ragged ends a column still reads one by one.
const BLOCK: usize = 64;

/// The least and greatest value over a run of samples, with where each was
/// first met.
#[derive(Debug, Clone, Copy)]
struct Extrema {
    min: f64,
    max: f64,
    min_at: u32,
    max_at: u32,
}

impl Extrema {
    const fn empty(at: u32) -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            min_at: at,
            max_at: at,
        }
    }

    /// Fold a later run in. Strict comparisons keep the first occurrence,
    /// exactly as reading the samples in order does.
    fn absorb(&mut self, later: Self) {
        if later.min < self.min {
            self.min = later.min;
            self.min_at = later.min_at;
        }
        if later.max > self.max {
            self.max = later.max;
            self.max_at = later.max_at;
        }
    }

    fn scan(&mut self, y: &[f64], from: usize, to: usize) {
        for (offset, &value) in y[from..to].iter().enumerate() {
            let at = (from + offset) as u32;
            if value < self.min {
                self.min = value;
                self.min_at = at;
            }
            if value > self.max {
                self.max = value;
                self.max_at = at;
            }
        }
    }
}

/// Block extrema of one trace, and the identity of the samples they were
/// read from.
#[derive(Debug)]
pub(super) struct BlockIndex {
    blocks: Vec<Extrema>,
    /// Blocks holding a non-finite abscissa or value. The reference reduction
    /// breaks the envelope at a hole, so a window holding one is left to it.
    holed: Vec<bool>,
    x_at: usize,
    y_at: usize,
    len: usize,
}

impl BlockIndex {
    /// Read `x` and `y` once into block extrema. `None` for a trace too long
    /// for the index's 32-bit positions.
    pub(super) fn build(x: &[f64], y: &[f64]) -> Option<Self> {
        let len = x.len().min(y.len());
        u32::try_from(len).ok()?;
        let mut blocks = Vec::with_capacity(len.div_ceil(BLOCK));
        let mut holed = Vec::with_capacity(len.div_ceil(BLOCK));
        for start in (0..len).step_by(BLOCK) {
            let end = (start + BLOCK).min(len);
            let mut extrema = Extrema::empty(start as u32);
            extrema.scan(y, start, end);
            blocks.push(extrema);
            holed.push(
                x[start..end]
                    .iter()
                    .zip(&y[start..end])
                    .any(|(x, y)| !(x.is_finite() && y.is_finite())),
            );
        }
        Some(Self {
            blocks,
            holed,
            x_at: x.as_ptr() as usize,
            y_at: y.as_ptr() as usize,
            len,
        })
    }

    /// Whether this index was read from these very samples.
    pub(super) fn describes(&self, x: &[f64], y: &[f64]) -> bool {
        self.x_at == x.as_ptr() as usize
            && self.y_at == y.as_ptr() as usize
            && self.len == x.len().min(y.len())
    }

    fn holed_between(&self, start: usize, end: usize) -> bool {
        start < end && self.holed[start / BLOCK..=(end - 1) / BLOCK].contains(&true)
    }

    /// Extrema of `y` over `[from, to)`, which holds no hole.
    fn extrema(&self, y: &[f64], from: usize, to: usize) -> Extrema {
        let mut extrema = Extrema::empty(from as u32);
        let (first_whole, end_whole) = (from.div_ceil(BLOCK), to / BLOCK);
        if first_whole >= end_whole {
            extrema.scan(y, from, to);
            return extrema;
        }
        extrema.scan(y, from, first_whole * BLOCK);
        for block in &self.blocks[first_whole..end_whole] {
            extrema.absorb(*block);
        }
        extrema.scan(y, end_whole * BLOCK, to);
        extrema
    }

    /// The envelope [`super::decimate_minmax`] draws of an ascending trace,
    /// read a block at a time. `None` where the reference has to read it
    /// sample by sample instead: a window short enough to draw raw, or one
    /// holding a hole.
    pub(super) fn decimate_ascending(
        &self,
        x: &[f64],
        y: &[f64],
        x0: f64,
        x1: f64,
        x_scale: XScale,
        columns: usize,
    ) -> Option<Vec<[f64; 2]>> {
        let view = LogicalView::new(x, y, false);
        let n = view.paired_len();
        if n == 0
            || columns == 0
            || !matches!(x1.partial_cmp(&x0), Some(std::cmp::Ordering::Greater))
        {
            return None;
        }
        // The reference's own window, margin and raw-draw threshold.
        let start = view.below(x0).saturating_sub(1);
        let end = (view.at_or_below(x1) + 1).min(n);
        if end.saturating_sub(start) <= columns * 2 || self.holed_between(start, end) {
            return None;
        }
        // The reference's column predicate, verbatim; it only rises with x.
        let column_of = |value: f64| {
            let t = x_scale.normalize(value, x0, x1).clamp(0.0, 1.0);
            ((t * columns as f64) as usize).min(columns - 1)
        };
        let mut out: Vec<[f64; 2]> = Vec::with_capacity(columns * 2 + 4);
        let mut first = start;
        while first < end {
            let column = column_of(x[first]);
            // Gallop to a sample past this column, then bisect back to the
            // first one: a column costs the logarithm of its width.
            let mut known = first;
            let mut probe = first + 1;
            let mut step = 1;
            while probe < end && column_of(x[probe]) == column {
                known = probe;
                step *= 2;
                probe = (probe + step).min(end);
            }
            let next = known + 1 + x[known + 1..probe].partition_point(|&v| column_of(v) == column);
            let extrema = self.extrema(y, first, next);
            let column_x = x[next - 1];
            // Encounter order, as the reference emits it.
            if extrema.min_at <= extrema.max_at {
                out.push([column_x, extrema.min]);
                if extrema.max_at != extrema.min_at {
                    out.push([column_x, extrema.max]);
                }
            } else {
                out.push([column_x, extrema.max]);
                out.push([column_x, extrema.min]);
            }
            first = next;
        }
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{XOrientation, decimate_minmax};
    use super::*;

    /// A small deterministic generator, so a failure names its seed.
    struct Lcg(u64);

    impl Lcg {
        fn next(&mut self) -> u64 {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            self.0 >> 33
        }

        fn below(&mut self, bound: u64) -> u64 {
            self.next() % bound
        }

        fn unit(&mut self) -> f64 {
            self.below(1 << 30) as f64 / f64::from(1u32 << 30)
        }
    }

    /// The block reduction is the reference reduction, bit for bit, over
    /// plateaus, repeated abscissae, ties on both extremes, partial and
    /// overhanging windows, both scales and many column counts.
    #[test]
    fn the_block_reduction_is_the_reference_reduction_point_for_point() {
        let mut compared = 0;
        for seed in 0..300u64 {
            let mut random = Lcg(seed);
            let len = 200 + random.below(20_000) as usize;
            let mut x = Vec::with_capacity(len);
            let mut position = 1.0 + random.unit();
            for _ in 0..len {
                // Repeated abscissae and uneven steps.
                if random.below(8) != 0 {
                    position += random.unit() * 0.01;
                }
                x.push(position);
            }
            // Quantized values: plateaus and ties on both extremes.
            let levels = 1 + random.below(40);
            let y: Vec<f64> = (0..len)
                .map(|_| random.below(levels) as f64 - levels as f64 / 2.0)
                .collect();
            let (low, high) = (x[0], x[len - 1]);
            let span = high - low;
            let x0 = low + span * (random.unit() * 0.6 - 0.1);
            let x1 = x0 + span * (0.05 + random.unit());
            let columns = 1 + random.below(2_000) as usize;
            let x_scale = if random.below(3) == 0 {
                XScale::Log10
            } else {
                XScale::Linear
            };
            let index = BlockIndex::build(&x, &y).expect("short trace");
            let reference =
                decimate_minmax(&x, &y, x0, x1, x_scale, columns, XOrientation::Ascending);
            if let Some(blocks) = index.decimate_ascending(&x, &y, x0, x1, x_scale, columns) {
                compared += 1;
                assert_eq!(blocks.len(), reference.len(), "seed {seed}");
                for (left, right) in blocks.iter().zip(&reference) {
                    assert!(
                        left[0].to_bits() == right[0].to_bits()
                            && left[1].to_bits() == right[1].to_bits(),
                        "seed {seed}: {left:?} != {right:?}"
                    );
                }
            }
        }
        assert!(compared > 150, "only {compared} cases took the block path");
    }

    /// A hole anywhere in the window leaves the reduction to the reference,
    /// which breaks the envelope there.
    #[test]
    fn a_window_holding_a_hole_is_left_to_the_reference() {
        let x: Vec<f64> = (0..10_000).map(f64::from).collect();
        let mut y: Vec<f64> = x.iter().map(|value| (value * 0.01).sin()).collect();
        y[5_000] = f64::NAN;
        let index = BlockIndex::build(&x, &y).expect("short trace");
        assert!(
            index
                .decimate_ascending(&x, &y, 0.0, 9_999.0, XScale::Linear, 512)
                .is_none()
        );
        assert!(
            index
                .decimate_ascending(&x, &y, 6_000.0, 9_999.0, XScale::Linear, 512)
                .is_some()
        );
    }
}
