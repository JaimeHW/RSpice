//! Distribution plot geometry shared by the interactive sheet and publication.

use super::HistogramDisplayMode;
use rspice_results::histogram::{Histogram, HistogramDistribution};
use std::ops::Deref;

#[derive(Debug, Clone)]
pub(crate) struct HistogramDisplay {
    data: HistogramDistribution,
}

impl Deref for HistogramDisplay {
    type Target = HistogramDistribution;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl HistogramDisplay {
    pub fn new(
        histogram: &Histogram,
        samples: &[f64],
        mode: HistogramDisplayMode,
    ) -> Result<Self, &'static str> {
        HistogramDistribution::new(histogram, samples, mode).map(|data| Self { data })
    }

    pub fn y_max(&self) -> f64 {
        if self.cdf.is_some() {
            return 1.08;
        }
        let peak = self.ordinates.iter().copied().fold(0.0, f64::max);
        if self.mode == HistogramDisplayMode::Count {
            (peak * 1.18).ceil().max(4.0)
        } else if peak == 0.0 {
            1.0
        } else {
            (peak * 1.18)
                .min(f64::MAX)
                .max(peak.next_up().min(f64::MAX))
        }
    }

    /// Data-space outlines for printing. Artificial width for a point bar is
    /// a presentation aid; its exact observation remains in source_points.
    pub fn paths(&self, histogram: &Histogram, x0: f64, x1: f64) -> Vec<Vec<(f64, f64)>> {
        if let Some(cdf) = &self.cdf {
            let start = cdf.x.partition_point(|value| *value < x0);
            let end = cdf.x.partition_point(|value| *value <= x1);
            let mut points = vec![(x0, start.checked_sub(1).map_or(0.0, |i| cdf.y[i]))];
            points.extend(
                cdf.x[start..end]
                    .iter()
                    .copied()
                    .zip(cdf.y[start..end].iter().copied()),
            );
            points.push((x1, cdf.value_at(x1)));
            return vec![points];
        }
        histogram
            .bins
            .iter()
            .zip(&self.ordinates)
            .filter(|(_, value)| **value > 0.0)
            .map(|(bin, &value)| {
                let (left, right) = if bin.lower == bin.upper {
                    let half = (x1 - x0) * 0.09;
                    (
                        (bin.lower - half).max(-f64::MAX),
                        (bin.upper + half).min(f64::MAX),
                    )
                } else {
                    (bin.lower, bin.upper)
                };
                vec![
                    (left, 0.0),
                    (left, value),
                    (right, value),
                    (right, 0.0),
                    (left, 0.0),
                ]
            })
            .collect()
    }
}

/// The common auto-fit window for both screen and hardcopy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct HistAxis {
    pub x0: f64,
    pub x1: f64,
    pub degenerate_at: Option<f64>,
}

pub(crate) fn hist_axis(histogram: &Histogram) -> HistAxis {
    let (min, max) = histogram.range();
    let span = max - min;
    if min < max {
        let pad = if span.is_finite() { span * 0.06 } else { 0.0 };
        return HistAxis {
            x0: (min - pad).max(-f64::MAX),
            x1: (max + pad).min(f64::MAX),
            degenerate_at: None,
        };
    }
    let value = min;
    let pad = if value == 0.0 {
        1.0
    } else {
        value.abs() * 1.0e-3
    };
    HistAxis {
        x0: (value - pad).min(value.next_down()).max(-f64::MAX),
        x1: (value + pad).max(value.next_up()).min(f64::MAX),
        degenerate_at: Some(value),
    }
}

#[cfg(test)]
mod tests {
    use super::super::HistogramBuilder;
    use super::*;

    #[test]
    fn empirical_cdf_uses_exact_observations_and_inclusive_ties() {
        let samples = [3.0, 1.0, -1.0, 1.0];
        let histogram = HistogramBuilder::new()
            .bin_count(1)
            .range(0.0, 2.0)
            .build(&samples);
        let display =
            HistogramDisplay::new(&histogram, &samples, HistogramDisplayMode::Cdf).unwrap();
        assert_eq!(
            display.source_points(&histogram),
            vec![
                (-1.0, 0.0),
                (-1.0, 0.25),
                (1.0, 0.25),
                (1.0, 0.75),
                (3.0, 0.75),
                (3.0, 1.0)
            ]
        );
        for (x, expected) in [
            (-2.0, 0.0),
            (0.0, 0.25),
            (1.0, 0.75),
            (2.0, 0.75),
            (3.0, 1.0),
        ] {
            assert_eq!(display.value_at(&histogram, x), expected);
        }
        assert_eq!(
            display.paths(&histogram, 0.0, 2.0),
            vec![vec![(0.0, 0.25), (1.0, 0.25), (1.0, 0.75), (2.0, 0.75),]]
        );
        assert_eq!(display.paths(&histogram, 1.0, 2.0)[0][0], (1.0, 0.25));
    }

    #[test]
    fn bars_keep_bin_width_and_zero_baseline_in_print_geometry() {
        let samples = [0.5, 1.5, 1.5];
        let histogram = HistogramBuilder::new()
            .bin_count(2)
            .range(0.0, 2.0)
            .build(&samples);
        let display =
            HistogramDisplay::new(&histogram, &samples, HistogramDisplayMode::Count).unwrap();
        assert_eq!(
            display.paths(&histogram, 0.0, 2.0),
            vec![
                vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.0)],
                vec![(1.0, 0.0), (1.0, 2.0), (2.0, 2.0), (2.0, 0.0), (1.0, 0.0)],
            ]
        );
        let samples = [1.0; 3];
        let histogram = HistogramBuilder::new().build(&samples);
        let display =
            HistogramDisplay::new(&histogram, &samples, HistogramDisplayMode::Count).unwrap();
        assert_eq!(display.value_at(&histogram, 1.0), 3.0);
        assert_eq!(display.value_at(&histogram, 1.0_f64.next_up()), 0.0);
    }
}
