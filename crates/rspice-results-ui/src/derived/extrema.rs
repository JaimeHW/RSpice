//! Finite extremes of any run of a series, read without rescanning it.
//!
//! A pane fits its Y axis to what the X window shows, and the window moves
//! on every frame of a pan or a zoom. Reading every sample in it on every
//! frame is a pass over millions of values per trace on a long transient, so
//! the series is summarised once: its extremes per block of [`BLOCK`]
//! samples, those blocks' per block of blocks, and so on up. A run's
//! extremes are then its ragged ends read sample by sample at each level and
//! the whole blocks between them read as one value each, which bounds a
//! query by `2 · BLOCK` reads per level whatever the run's length.

use rspice_results::sampling::{SweepClass, SweepShape, XOrientation};

/// Entries per block at every level. The summary is a sixty-third of the
/// series' length, and a query reads at most two partial blocks per level.
const BLOCK: usize = 64;

/// The least and greatest finite value of a run, and its least positive one,
/// which is what a logarithmic axis fits to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Extremes {
    pub min: f64,
    pub max: f64,
    pub min_positive: f64,
}

impl Extremes {
    const EMPTY: Self = Self {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
        min_positive: f64::INFINITY,
    };

    /// Widen these extremes to take `other` in.
    pub fn absorb(&mut self, other: Self) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.min_positive = self.min_positive.min(other.min_positive);
    }

    fn absorb_value(&mut self, value: f64) {
        if value.is_finite() {
            self.min = self.min.min(value);
            self.max = self.max.max(value);
            if value > 0.0 {
                self.min_positive = self.min_positive.min(value);
            }
        }
    }

    /// The finite (min, max), when the run held a finite value.
    pub fn range(self) -> Option<(f64, f64)> {
        (self.min <= self.max).then_some((self.min, self.max))
    }

    /// The (least, greatest) positive value, when the run held one.
    pub fn positive_range(self) -> Option<(f64, f64)> {
        (self.min_positive <= self.max).then_some((self.min_positive, self.max))
    }
}

impl Default for Extremes {
    /// The extremes of a run with no finite value.
    fn default() -> Self {
        Self::EMPTY
    }
}

/// The block summary of one series.
#[derive(Debug)]
pub struct ExtremaIndex {
    /// `levels[0]` holds one entry per [`BLOCK`] samples, and each level
    /// above it one per [`BLOCK`] entries of the level below.
    levels: Vec<Vec<Extremes>>,
    /// The series it was built from, so a lookup under a reused key cannot
    /// answer for different values.
    source: (usize, usize),
    whole: Extremes,
}

impl ExtremaIndex {
    /// Summarise `values` in one pass.
    pub fn build(values: &[f64]) -> Self {
        let mut levels: Vec<Vec<Extremes>> = Vec::new();
        let mut below: Vec<Extremes> = values
            .chunks(BLOCK)
            .map(|block| {
                let mut extremes = Extremes::EMPTY;
                for &value in block {
                    extremes.absorb_value(value);
                }
                extremes
            })
            .collect();
        let mut whole = Extremes::EMPTY;
        loop {
            let next: Vec<Extremes> = below
                .chunks(BLOCK)
                .map(|block| {
                    let mut extremes = Extremes::EMPTY;
                    for entry in block {
                        extremes.absorb(*entry);
                    }
                    extremes
                })
                .collect();
            let top = below.len() <= BLOCK;
            if top {
                for entry in &below {
                    whole.absorb(*entry);
                }
            }
            levels.push(below);
            if top {
                break;
            }
            below = next;
        }
        Self {
            levels,
            source: (values.as_ptr() as usize, values.len()),
            whole,
        }
    }

    /// Whether this summary was built from `values`.
    pub fn describes(&self, values: &[f64]) -> bool {
        self.source == (values.as_ptr() as usize, values.len())
    }

    /// The extremes of the whole series.
    pub fn whole(&self) -> Extremes {
        self.whole
    }

    /// The extremes of `values[from..to]`.
    pub fn run(&self, values: &[f64], from: usize, to: usize) -> Extremes {
        let mut extremes = Extremes::EMPTY;
        let (mut from, mut to) = (from, to.min(values.len()));
        if from >= to {
            return extremes;
        }
        // The samples first, then each level of summary, reading only the
        // ragged ends of the run at each and handing the whole blocks
        // between them up to the level that holds them as single entries.
        let first_whole = from.div_ceil(BLOCK);
        let end_whole = to / BLOCK;
        if first_whole >= end_whole {
            for &value in &values[from..to] {
                extremes.absorb_value(value);
            }
            return extremes;
        }
        for &value in values[from..first_whole * BLOCK]
            .iter()
            .chain(&values[end_whole * BLOCK..to])
        {
            extremes.absorb_value(value);
        }
        (from, to) = (first_whole, end_whole);
        for (depth, level) in self.levels.iter().enumerate() {
            let first_whole = from.div_ceil(BLOCK);
            let end_whole = to / BLOCK;
            if depth + 1 == self.levels.len() || first_whole >= end_whole {
                for entry in &level[from..to] {
                    extremes.absorb(*entry);
                }
                break;
            }
            for entry in level[from..first_whole * BLOCK]
                .iter()
                .chain(&level[end_whole * BLOCK..to])
            {
                extremes.absorb(*entry);
            }
            (from, to) = (first_whole, end_whole);
        }
        extremes
    }

    /// The extremes of what a polyline through `(x, y)` draws between `x0`
    /// and `x1`: every sample in the window, and the curve's value where it
    /// crosses each edge of it, on every monotone branch of the abscissa.
    ///
    /// An abscissa with no branch structure to search, a locus, answers with
    /// the whole series.
    pub fn window(
        &self,
        x: &[f64],
        y: &[f64],
        shape: &SweepShape,
        (x0, x1): (f64, f64),
    ) -> Extremes {
        if shape.class() == SweepClass::NonSweep {
            return self.whole;
        }
        let (x0, x1) = (x0.min(x1), x0.max(x1));
        let len = x.len().min(y.len());
        let mut extremes = Extremes::EMPTY;
        for run in shape.runs() {
            let (start, end) = (run.start.min(len), run.end.min(len));
            if start >= end {
                continue;
            }
            let branch = &x[start..end];
            // The half-open stretch of the branch inside the window.
            let (first, last) = match run.orientation {
                XOrientation::Ascending => (
                    branch.partition_point(|value| *value < x0),
                    branch.partition_point(|value| *value <= x1),
                ),
                XOrientation::Descending => (
                    branch.partition_point(|value| *value > x1),
                    branch.partition_point(|value| *value >= x0),
                ),
            };
            let (first, last) = (start + first, start + last.max(first));
            extremes.absorb(self.run(y, first, last));
            // Where the drawn segment crosses each edge, unless the branch
            // has no sample beyond that edge.
            for (edge, at) in [(x0, first), (x1, last), (x1, first), (x0, last)] {
                if at > start
                    && at < end
                    && let Some(value) = crossing(x, y, at - 1, at, edge)
                {
                    extremes.absorb_value(value);
                }
            }
        }
        extremes
    }
}

/// The value the segment from sample `a` to sample `b` takes at `edge`, when
/// the edge falls on it.
fn crossing(x: &[f64], y: &[f64], a: usize, b: usize, edge: f64) -> Option<f64> {
    let (xa, xb, ya, yb) = (x[a], x[b], y[a], y[b]);
    let inside = (xa.min(xb)..=xa.max(xb)).contains(&edge);
    (inside && xa != xb && ya.is_finite() && yb.is_finite())
        .then(|| (edge - xa).mul_add((yb - ya) / (xb - xa), ya))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Lcg(u64);

    impl Lcg {
        fn unit(&mut self) -> f64 {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (self.0 >> 11) as f64 / (1u64 << 53) as f64
        }
    }

    fn scanned(values: &[f64]) -> Extremes {
        let mut extremes = Extremes::EMPTY;
        for &value in values {
            extremes.absorb_value(value);
        }
        extremes
    }

    /// Every run of a series, at every length and alignment against the
    /// blocks, reads the extremes a scan of its samples reads, gaps and all.
    #[test]
    fn any_run_reads_what_scanning_it_reads() {
        let mut random = Lcg(7);
        for len in [0, 1, 63, 64, 65, 4_095, 4_096, 4_097, 300_001] {
            let values: Vec<f64> = (0..len)
                .map(|index| {
                    if index % 977 == 3 {
                        f64::NAN
                    } else {
                        random.unit().mul_add(2.0, -1.0) * 1.0e-3
                    }
                })
                .collect();
            let index = ExtremaIndex::build(&values);
            assert!(index.describes(&values));
            assert_eq!(index.whole(), scanned(&values), "{len}: whole");
            for _ in 0..200 {
                let a = (random.unit() * (len + 1) as f64) as usize;
                let b = (random.unit() * (len + 1) as f64) as usize;
                let (from, to) = (a.min(b), a.max(b));
                assert_eq!(
                    index.run(&values, from, to),
                    scanned(&values[from..to]),
                    "{len}: {from}..{to}"
                );
            }
        }
    }

    /// A window between two samples reads the segment that crosses it, and a
    /// window over a ramp reads the ramp's values at its edges, not the
    /// samples outside it.
    #[test]
    fn a_window_reads_the_curve_at_its_edges() {
        let x = [0.0, 1.0, 2.0, 3.0];
        let y = [0.0, 10.0, 20.0, 30.0];
        let shape = SweepShape::of(&x);
        let index = ExtremaIndex::build(&y);
        assert_eq!(
            index.window(&x, &y, &shape, (1.25, 1.75)).range(),
            Some((12.5, 17.5))
        );
        assert_eq!(
            index.window(&x, &y, &shape, (0.5, 2.5)).range(),
            Some((5.0, 25.0))
        );
        // Past the data on either side only what is drawn counts.
        assert_eq!(
            index.window(&x, &y, &shape, (2.5, 9.0)).range(),
            Some((25.0, 30.0))
        );
        assert_eq!(index.window(&x, &y, &shape, (5.0, 9.0)).range(), None);
    }

    /// A reverse sweep and a loop are searched branch by branch.
    #[test]
    fn a_window_reads_every_branch_of_a_loop() {
        let x = [0.0, 1.0, 2.0, 3.0, 2.0, 1.0, 0.0];
        let y = [0.0, 1.0, 2.0, 3.0, -2.0, -1.0, 0.0];
        let shape = SweepShape::of(&x);
        let index = ExtremaIndex::build(&y);
        assert_eq!(
            index.window(&x, &y, &shape, (0.5, 1.5)).range(),
            Some((-1.5, 1.5))
        );
        let reverse: Vec<f64> = x[3..].to_vec();
        let values: Vec<f64> = y[3..].to_vec();
        let shape = SweepShape::of(&reverse);
        let index = ExtremaIndex::build(&values);
        assert_eq!(
            index.window(&reverse, &values, &shape, (0.5, 1.5)).range(),
            Some((-1.5, -0.5))
        );
    }

    /// A logarithmic axis fits the positive values only.
    #[test]
    fn the_positive_range_leaves_out_zero_and_below() {
        let values = [0.0, -1.0, 1.0e-6, 1.0e-3, f64::INFINITY];
        let index = ExtremaIndex::build(&values);
        assert_eq!(index.whole().range(), Some((-1.0, 1.0e-3)));
        assert_eq!(index.whole().positive_range(), Some((1.0e-6, 1.0e-3)));
        assert_eq!(
            ExtremaIndex::build(&[0.0, -2.0]).whole().positive_range(),
            None
        );
    }
}
