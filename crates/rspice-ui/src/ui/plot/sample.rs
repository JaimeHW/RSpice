//! What the X column of a series actually is.
//!
//! Every reduction and every readout in the plot engine used to assume one
//! thing about X: that it increases. Most sweeps do. A reverse DC sweep does
//! not, a hysteresis loop turns around in the middle, and a locus abandons the
//! idea of ordering altogether. Each of those assumptions failed silently —
//! a descending sweep clamped every cursor query to its first sample and
//! vanished when zoomed, and a two-branch loop lost whichever branch fell
//! outside a contiguous index window.
//!
//! [`SweepShape`] is the one O(n) answer the rest of the engine asks for: how
//! many monotone branches this X column has, which way each one runs, and
//! where each one lives in the source arrays. It is pure — caching it is the
//! caller's business, because only the caller knows when its data changed.

use std::ops::Range;

use super::decimate::{SampleInterpolation, sample_at_with};

/// Which way a monotone run of samples travels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XOrientation {
    Ascending,
    Descending,
}

/// One maximal monotone run: half-open `[start, end)` into the source arrays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonotoneRun {
    pub start: usize,
    pub end: usize,
    pub orientation: XOrientation,
}

/// What the X column is, coarsely enough to route a reduction by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SweepClass {
    /// No samples carry a position at all.
    Empty,
    /// One run, increasing — the ordinary case every reduction assumes.
    Ascending,
    /// One run, decreasing — a reverse sweep.
    Descending,
    /// Several monotone branches: a hysteresis loop, a retrace, a sawtooth.
    MultiBranch,
    /// More reversals than a branch model can describe. A locus, or noise.
    NonSweep,
}

/// More reversals than this and the series stops being a set of branches;
/// storing them would cost more than the reduction they were meant to route.
pub const SHAPE_RUN_CAP: usize = 512;

/// The monotone structure of one X column.
#[derive(Debug, Clone, PartialEq)]
pub struct SweepShape {
    class: SweepClass,
    runs: Vec<MonotoneRun>,
}

impl SweepShape {
    /// Classify `x` in one pass.
    ///
    /// Segmentation rules, each of them a case the instrument actually meets:
    /// a non-finite abscissa is a hole, so it breaks the run and belongs to
    /// none; equal consecutive samples extend the run they are in (a plateau
    /// is not a turnaround); a run's direction is decided by its first strict
    /// comparison; and two consecutive runs share the sample they turn around
    /// on, so no sample and no drawn segment is lost between them.
    #[must_use]
    pub fn of(x: &[f64]) -> Self {
        let mut runs: Vec<MonotoneRun> = Vec::new();
        let mut index = 0usize;
        while index < x.len() {
            if !x[index].is_finite() {
                index += 1;
                continue;
            }
            let start = index;
            while index < x.len() && x[index].is_finite() {
                index += 1;
            }
            if !segment_runs(x, start, index, &mut runs) {
                return Self {
                    class: SweepClass::NonSweep,
                    runs: Vec::new(),
                };
            }
        }
        let class = match runs.as_slice() {
            [] => SweepClass::Empty,
            [only] => match only.orientation {
                XOrientation::Ascending => SweepClass::Ascending,
                XOrientation::Descending => SweepClass::Descending,
            },
            _ => SweepClass::MultiBranch,
        };
        Self { class, runs }
    }

    #[must_use]
    pub fn class(&self) -> SweepClass {
        self.class
    }

    #[must_use]
    pub fn runs(&self) -> &[MonotoneRun] {
        &self.runs
    }

    /// How many monotone branches a reader would see. Zero for a series with
    /// no positions and for one past [`SHAPE_RUN_CAP`].
    #[must_use]
    pub fn branch_count(&self) -> usize {
        self.runs.len()
    }

    /// The case every X-ordered reduction was written for.
    #[must_use]
    pub fn is_single_ascending(&self) -> bool {
        self.class == SweepClass::Ascending
    }

    /// One run, either direction: a sweep with a single answer per X.
    #[must_use]
    pub fn is_monotone(&self) -> bool {
        matches!(self.class, SweepClass::Ascending | SweepClass::Descending)
    }

    /// The half-open index ranges, one per branch, whose samples fall inside
    /// the closed window `[lo, hi]`.
    ///
    /// A series past the run cap keeps no branches, so its ranges are found by
    /// a scan instead: the answer stays exact, it just costs O(n).
    #[must_use]
    pub fn window_ranges(&self, x: &[f64], lo: f64, hi: f64) -> Vec<Range<usize>> {
        if !(hi >= lo) {
            return Vec::new();
        }
        if self.class == SweepClass::NonSweep {
            return scanned_window_ranges(x, lo, hi);
        }
        self.runs
            .iter()
            .filter_map(|run| {
                let end = run.end.min(x.len());
                if run.start >= end {
                    return None;
                }
                let slice = &x[run.start..end];
                let (first, last) = match run.orientation {
                    XOrientation::Ascending => (
                        slice.partition_point(|&value| value < lo),
                        slice.partition_point(|&value| value <= hi),
                    ),
                    XOrientation::Descending => (
                        slice.partition_point(|&value| value > hi),
                        slice.partition_point(|&value| value >= lo),
                    ),
                };
                (first < last).then(|| (run.start + first)..(run.start + last))
            })
            .collect()
    }
}

/// Split one all-finite segment into monotone runs, appending them to `runs`.
/// Returns `false` once the run cap is passed, which makes the whole series a
/// [`SweepClass::NonSweep`].
fn segment_runs(x: &[f64], start: usize, end: usize, runs: &mut Vec<MonotoneRun>) -> bool {
    let mut run_start = start;
    let mut orientation: Option<XOrientation> = None;
    for index in (start + 1)..end {
        let (previous, current) = (x[index - 1], x[index]);
        if current == previous {
            continue;
        }
        let direction = if current > previous {
            XOrientation::Ascending
        } else {
            XOrientation::Descending
        };
        match orientation {
            None => orientation = Some(direction),
            Some(established) if established == direction => {}
            Some(established) => {
                if runs.len() >= SHAPE_RUN_CAP {
                    return false;
                }
                runs.push(MonotoneRun {
                    start: run_start,
                    end: index,
                    orientation: established,
                });
                // The turnaround sample belongs to both branches: it is where
                // one ends and the other begins, and dropping it from either
                // would open a gap in the drawn curve.
                run_start = index - 1;
                orientation = Some(direction);
            }
        }
    }
    if runs.len() >= SHAPE_RUN_CAP {
        return false;
    }
    runs.push(MonotoneRun {
        start: run_start,
        end,
        // A flat segment has no direction to read; ascending is the identity
        // the rest of the engine already assumes.
        orientation: orientation.unwrap_or(XOrientation::Ascending),
    });
    true
}

fn scanned_window_ranges(x: &[f64], lo: f64, hi: f64) -> Vec<Range<usize>> {
    let mut ranges: Vec<Range<usize>> = Vec::new();
    let mut open: Option<usize> = None;
    for (index, value) in x.iter().enumerate() {
        if value.is_finite() && (lo..=hi).contains(value) {
            open.get_or_insert(index);
        } else if let Some(start) = open.take() {
            ranges.push(start..index);
        }
    }
    if let Some(start) = open {
        ranges.push(start..x.len());
    }
    ranges
}

/// One branch's answer at a queried X.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BranchSample {
    /// Index into [`SweepShape::runs`].
    pub run: usize,
    pub orientation: XOrientation,
    pub value: f64,
}

/// Evaluate every branch whose closed X span contains `xq`.
///
/// A branch that does not reach `xq` is absent from the output rather than
/// clamped to its endpoint: a loop that has not got there yet has no value
/// there, and inventing one is the whole failure this replaces. `out` is
/// cleared first, so a per-frame readout can keep one scratch vector and
/// allocate nothing.
///
/// A [`SweepClass::NonSweep`] series keeps no branches and reports none; its
/// honest single answer is [`nearest_sample`].
pub fn sample_branches_into(
    x: &[f64],
    y: &[f64],
    shape: &SweepShape,
    xq: f64,
    interpolation: SampleInterpolation,
    out: &mut Vec<BranchSample>,
) {
    out.clear();
    let n = x.len().min(y.len());
    for (index, run) in shape.runs.iter().enumerate() {
        let end = run.end.min(n);
        if run.start >= end {
            continue;
        }
        let (first, last) = (x[run.start], x[end - 1]);
        let (lo, hi) = if first <= last {
            (first, last)
        } else {
            (last, first)
        };
        if !(xq >= lo && xq <= hi) {
            continue;
        }
        out.push(BranchSample {
            run: index,
            orientation: run.orientation,
            value: sample_at_with(&x[run.start..end], &y[run.start..end], xq, interpolation),
        });
    }
}

/// One value for `xq`, whatever the shape.
///
/// A monotone sweep answers exactly as [`sample_at_with`] does, endpoint
/// clamping included. A multi-branch series answers with its first covering
/// branch, which is presentation-lossy by construction — a surface that owes
/// the reader every branch uses [`sample_branches_into`].
#[must_use]
pub fn sample_at_with_shape(
    x: &[f64],
    y: &[f64],
    shape: &SweepShape,
    xq: f64,
    interpolation: SampleInterpolation,
) -> f64 {
    match shape.class {
        SweepClass::Empty | SweepClass::Ascending | SweepClass::Descending => {
            sample_at_with(x, y, xq, interpolation)
        }
        SweepClass::NonSweep => nearest_sample(x, y, xq),
        SweepClass::MultiBranch => {
            let n = x.len().min(y.len());
            for run in &shape.runs {
                let end = run.end.min(n);
                if run.start >= end {
                    continue;
                }
                let (first, last) = (x[run.start], x[end - 1]);
                let (lo, hi) = if first <= last {
                    (first, last)
                } else {
                    (last, first)
                };
                if xq >= lo && xq <= hi {
                    return sample_at_with(
                        &x[run.start..end],
                        &y[run.start..end],
                        xq,
                        interpolation,
                    );
                }
            }
            nearest_sample(x, y, xq)
        }
    }
}

/// The sample nearest `xq` in source order, with no assumption about ordering
/// at all. O(n), and the only honest answer for a series with no branch model.
#[must_use]
pub fn nearest_sample(x: &[f64], y: &[f64], xq: f64) -> f64 {
    let n = x.len().min(y.len());
    let mut best: Option<(f64, f64)> = None;
    for index in 0..n {
        let distance = (x[index] - xq).abs();
        if !distance.is_finite() {
            continue;
        }
        if best.is_none_or(|(closest, _)| distance < closest) {
            best = Some((distance, y[index]));
        }
    }
    best.map_or(0.0, |(_, value)| value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn runs_of(x: &[f64]) -> Vec<(usize, usize, XOrientation)> {
        SweepShape::of(x)
            .runs()
            .iter()
            .map(|run| (run.start, run.end, run.orientation))
            .collect()
    }

    /// Oracle 2: the closed-form segmentation of every shape the instrument
    /// produces.
    #[test]
    fn segmentation_matches_its_closed_forms() {
        use XOrientation::{Ascending, Descending};

        // A retrace shares the sample it turns around on.
        let retrace = [0.0, 0.5, 1.0, 0.5, 0.0];
        assert_eq!(
            runs_of(&retrace),
            vec![(0, 3, Ascending), (2, 5, Descending)]
        );
        assert_eq!(SweepShape::of(&retrace).class(), SweepClass::MultiBranch);
        assert_eq!(SweepShape::of(&retrace).branch_count(), 2);

        // A hole breaks the run and belongs to neither side of it.
        assert_eq!(
            runs_of(&[0.0, 1.0, f64::NAN, 2.0, 3.0]),
            vec![(0, 2, Ascending), (3, 5, Ascending)]
        );

        // A plateau extends the run it is in; the turnaround is still shared.
        assert_eq!(
            runs_of(&[0.0, 0.0, 1.0, 1.0, 0.0]),
            vec![(0, 4, Ascending), (3, 5, Descending)]
        );

        // No strict comparison anywhere: one run, and it reads as ascending.
        let flat = [2.0; 6];
        assert_eq!(runs_of(&flat), vec![(0, 6, Ascending)]);
        assert_eq!(SweepShape::of(&flat).class(), SweepClass::Ascending);
        assert!(SweepShape::of(&flat).is_single_ascending());

        // Past the cap the branch model is abandoned outright.
        let sawtooth: Vec<f64> = (0..4_000)
            .map(|index| f64::from(index % 2) + f64::from(index) * 1.0e-6)
            .collect();
        let shape = SweepShape::of(&sawtooth);
        assert_eq!(shape.class(), SweepClass::NonSweep);
        assert!(shape.runs().is_empty());
        assert_eq!(shape.branch_count(), 0);

        // Nothing to classify.
        assert_eq!(SweepShape::of(&[]).class(), SweepClass::Empty);
        assert_eq!(SweepShape::of(&[f64::NAN; 4]).class(), SweepClass::Empty);

        // A plain reverse sweep is monotone, not multi-branch.
        let reverse = [5.0, 4.0, 3.0, 2.0];
        assert_eq!(SweepShape::of(&reverse).class(), SweepClass::Descending);
        assert!(SweepShape::of(&reverse).is_monotone());
        assert!(!SweepShape::of(&reverse).is_single_ascending());
    }

    /// Oracle 3: every branch covering the query answers, and only those.
    #[test]
    fn branch_queries_answer_from_every_covering_branch() {
        let x = [0.0, 0.5, 1.0, 0.5, 0.0];
        let y = [0.0, 1.0, 2.0, 3.0, 4.0];
        let shape = SweepShape::of(&x);
        let mut out = Vec::new();

        sample_branches_into(&x, &y, &shape, 0.25, SampleInterpolation::Linear, &mut out);
        assert_eq!(
            out,
            vec![
                BranchSample {
                    run: 0,
                    orientation: XOrientation::Ascending,
                    value: 0.5
                },
                BranchSample {
                    run: 1,
                    orientation: XOrientation::Descending,
                    value: 3.5
                },
            ]
        );

        // The turnaround is on both branches and reads the same from each.
        sample_branches_into(&x, &y, &shape, 1.0, SampleInterpolation::Linear, &mut out);
        assert_eq!(
            out.iter().map(|sample| sample.value).collect::<Vec<_>>(),
            vec![2.0, 2.0]
        );

        // Past the loop, no branch has a value — and none is invented.
        sample_branches_into(&x, &y, &shape, 1.5, SampleInterpolation::Linear, &mut out);
        assert!(out.is_empty());
    }

    /// Oracle 4: a shaped monotone series answers exactly as the unshaped
    /// kernel does, including outside the data.
    #[test]
    fn a_monotone_shape_answers_exactly_as_the_plain_kernel() {
        for reverse in [false, true] {
            let mut x: Vec<f64> = (0..40).map(|index| f64::from(index) * 0.25).collect();
            let mut y: Vec<f64> = x.iter().map(|value| (value * 0.7).sin()).collect();
            if reverse {
                x.reverse();
                y.reverse();
            }
            let shape = SweepShape::of(&x);
            assert!(shape.is_monotone());
            for mode in [
                SampleInterpolation::Nearest,
                SampleInterpolation::Linear,
                SampleInterpolation::MonotoneCubic,
            ] {
                for step in -4..44 {
                    let xq = f64::from(step) * 0.25 + 0.125;
                    assert_eq!(
                        sample_at_with_shape(&x, &y, &shape, xq, mode),
                        sample_at_with(&x, &y, xq, mode),
                        "reverse={reverse} mode={mode:?} xq={xq}"
                    );
                }
            }
        }
    }

    /// Oracle 5: a window over a loop selects a range per branch.
    #[test]
    fn window_ranges_split_per_branch() {
        let x = [0.0, 0.5, 1.0, 0.5, 0.0];
        let shape = SweepShape::of(&x);
        assert_eq!(shape.window_ranges(&x, 0.4, 0.6), vec![1..2, 3..4]);
        assert_eq!(shape.window_ranges(&x, -1.0, 2.0), vec![0..3, 2..5]);
        assert!(shape.window_ranges(&x, 2.0, 3.0).is_empty());
        assert!(shape.window_ranges(&x, 0.6, 0.4).is_empty());

        let ascending: Vec<f64> = (0..10).map(f64::from).collect();
        assert_eq!(
            SweepShape::of(&ascending).window_ranges(&ascending, 3.0, 5.0),
            vec![3..6]
        );

        // A series past the cap still answers exactly, by scan.
        let sawtooth: Vec<f64> = (0..4_000)
            .map(|index| f64::from(index % 2) + f64::from(index) * 1.0e-6)
            .collect();
        let shape = SweepShape::of(&sawtooth);
        assert_eq!(shape.class(), SweepClass::NonSweep);
        let ranges = shape.window_ranges(&sawtooth, 0.9, 1.1);
        assert!(!ranges.is_empty());
        assert!(
            ranges
                .iter()
                .flat_map(|range| range.clone())
                .all(|index| (0.9..=1.1).contains(&sawtooth[index]))
        );
    }

    #[test]
    fn the_nearest_sample_ignores_ordering_entirely() {
        let x = [0.0, 3.0, 1.0, f64::NAN, 2.0];
        let y = [10.0, 13.0, 11.0, 99.0, 12.0];
        assert_eq!(nearest_sample(&x, &y, 1.1), 11.0);
        assert_eq!(nearest_sample(&x, &y, 100.0), 13.0);
        assert_eq!(nearest_sample(&[], &[], 0.0), 0.0);
    }

    #[test]
    fn a_non_sweep_still_answers_one_value() {
        let sawtooth: Vec<f64> = (0..4_000)
            .map(|index| f64::from(index % 2) + f64::from(index) * 1.0e-6)
            .collect();
        let y: Vec<f64> = sawtooth.iter().map(|value| value * 2.0).collect();
        let shape = SweepShape::of(&sawtooth);
        let mut out = Vec::new();
        sample_branches_into(
            &sawtooth,
            &y,
            &shape,
            0.5,
            SampleInterpolation::Linear,
            &mut out,
        );
        assert!(out.is_empty(), "a non-sweep reports no branches");
        let value = sample_at_with_shape(&sawtooth, &y, &shape, 0.5, SampleInterpolation::Linear);
        assert!(value.is_finite());
    }
}
