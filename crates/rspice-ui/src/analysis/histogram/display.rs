//! Shared distribution ordinates for the interactive sheet and publication.

use super::{HistogramDisplayMode, data::Histogram};

/// A right-continuous empirical CDF. Each distinct observation has a vertical
/// jump from the preceding population fraction to the inclusive fraction.
#[derive(Debug, Clone)]
pub(crate) struct EmpiricalCdf {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

impl EmpiricalCdf {
    fn new(samples: &[f64]) -> Self {
        let mut sorted = samples.to_vec();
        sorted.sort_unstable_by(f64::total_cmp);
        let mut x = Vec::new();
        let mut y = Vec::new();
        let mut index = 0;
        while index < sorted.len() {
            let value = sorted[index];
            let start = index;
            index += 1;
            while index < sorted.len() && sorted[index] == value {
                index += 1;
            }
            x.extend([value, value]);
            y.extend([
                start as f64 / samples.len() as f64,
                index as f64 / samples.len() as f64,
            ]);
        }
        Self { x, y }
    }

    pub fn value_at(&self, x: f64) -> f64 {
        let end = self.x.partition_point(|value| *value <= x);
        end.checked_sub(1).map_or(0.0, |index| self.y[index])
    }
}

#[derive(Debug, Clone)]
pub(crate) struct HistogramDisplay {
    pub mode: HistogramDisplayMode,
    /// One ordinate per bin for Count, PDF and Percent. Empty for the CDF,
    /// whose jumps are observations rather than bin edges or centers.
    pub ordinates: Vec<f64>,
    pub cdf: Option<EmpiricalCdf>,
}

impl HistogramDisplay {
    pub fn new(
        histogram: &Histogram,
        samples: &[f64],
        mode: HistogramDisplayMode,
    ) -> Result<Self, &'static str> {
        if samples.is_empty()
            || samples.len() != histogram.total_count
            || samples.iter().any(|value| !value.is_finite())
        {
            return Err("The distribution requires a finite retained sample population");
        }
        let mut ordinates = Vec::new();
        if mode != HistogramDisplayMode::Cdf {
            for bin in &histogram.bins {
                let fraction = bin.count as f64 / histogram.total_count as f64;
                let value = match mode {
                    HistogramDisplayMode::Count => bin.count as f64,
                    HistogramDisplayMode::Percent => fraction * 100.0,
                    HistogramDisplayMode::Pdf => {
                        if bin.lower == bin.upper {
                            return Err(
                                "A point population has no finite probability density. Choose a finite custom range, Count, Percent, or Empirical CDF.",
                            );
                        }
                        // Normalize before dividing by width. A full-width
                        // subtraction can itself overflow for opposite signs.
                        let width = bin.width();
                        let density = if width.is_finite() {
                            fraction / width
                        } else {
                            (fraction * 0.5) / (bin.upper * 0.5 - bin.lower * 0.5)
                        };
                        if !density.is_finite() {
                            return Err(
                                "The density exceeds the numeric display range. Choose wider bins, Count, Percent, or Empirical CDF.",
                            );
                        }
                        density
                    }
                    HistogramDisplayMode::Cdf => unreachable!(),
                };
                ordinates.push(value);
            }
        }
        Ok(Self {
            mode,
            ordinates,
            cdf: (mode == HistogramDisplayMode::Cdf).then(|| EmpiricalCdf::new(samples)),
        })
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

    /// Exact data-space coordinates underlying the displayed bars or steps.
    pub fn source_points(&self, histogram: &Histogram) -> Vec<(f64, f64)> {
        if let Some(cdf) = &self.cdf {
            cdf.x.iter().copied().zip(cdf.y.iter().copied()).collect()
        } else {
            histogram
                .bins
                .iter()
                .zip(&self.ordinates)
                .map(|(bin, &value)| (bin.center(), value))
                .collect()
        }
    }

    pub fn value_at(&self, histogram: &Histogram, x: f64) -> f64 {
        if let Some(cdf) = &self.cdf {
            return cdf.value_at(x);
        }
        let end = histogram.range().1;
        histogram
            .bins
            .iter()
            .zip(&self.ordinates)
            .find_map(|(bin, &value)| {
                (x >= bin.lower && (x < bin.upper || x == end && x == bin.upper)).then_some(value)
            })
            .unwrap_or(0.0)
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
    fn normalization_includes_samples_outside_the_custom_range() {
        let samples = [-1.0, 0.0, 1.0, 2.0, 3.0];
        let histogram = HistogramBuilder::new()
            .bin_count(2)
            .range(0.0, 2.0)
            .build(&samples);
        let density =
            HistogramDisplay::new(&histogram, &samples, HistogramDisplayMode::Pdf).unwrap();
        assert_eq!(density.ordinates, vec![0.2, 0.4]);
        let percent =
            HistogramDisplay::new(&histogram, &samples, HistogramDisplayMode::Percent).unwrap();
        assert_eq!(percent.ordinates, vec![20.0, 40.0]);
        assert_eq!(percent.value_at(&histogram, 1.0), 40.0);
        assert_eq!(percent.value_at(&histogram, 2.0), 40.0);
        assert_eq!(percent.value_at(&histogram, 3.0), 0.0);
    }

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

    #[test]
    fn density_preserves_extreme_widths_and_rejects_point_mass() {
        let samples = [1e308; 4];
        let histogram = HistogramBuilder::new()
            .bin_count(1)
            .range(0.0, 1.5e308)
            .build(&samples);
        let density =
            HistogramDisplay::new(&histogram, &samples, HistogramDisplayMode::Pdf).unwrap();
        assert!(density.ordinates[0] > 0.0);
        assert!((density.ordinates[0] * 1.5e308 - 1.0).abs() < 1e-14);
        let histogram = HistogramBuilder::new()
            .bin_count(1)
            .range(-1.5e308, 1.5e308)
            .build(&samples);
        let density =
            HistogramDisplay::new(&histogram, &samples, HistogramDisplayMode::Pdf).unwrap();
        assert!((density.ordinates[0] * 1.5e308 - 0.5).abs() < 1e-14);
        let histogram = HistogramBuilder::new().build(&samples);
        assert!(HistogramDisplay::new(&histogram, &samples, HistogramDisplayMode::Pdf).is_err());
        let cdf = HistogramDisplay::new(&histogram, &samples, HistogramDisplayMode::Cdf).unwrap();
        assert_eq!(cdf.value_at(&histogram, 1e308_f64.next_down()), 0.0);
        assert_eq!(cdf.value_at(&histogram, 1e308), 1.0);
    }
}
