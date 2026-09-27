//! Exact count, density, percentage and empirical cumulative distributions.

use super::Histogram;

/// Histogram display mode
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum HistogramDisplayMode {
    /// Bar chart (count per bin)
    #[default]
    Count,
    /// Probability density function
    Pdf,
    /// Empirical cumulative distribution from exact retained observations
    Cdf,
    /// Percent of total
    Percent,
}

impl HistogramDisplayMode {
    pub const ALL: [Self; 4] = [Self::Count, Self::Pdf, Self::Cdf, Self::Percent];

    pub fn label(self) -> &'static str {
        match self {
            Self::Count => "Count",
            Self::Pdf => "Probability density",
            Self::Cdf => "Empirical CDF",
            Self::Percent => "Percent",
        }
    }

    pub fn unit(self) -> &'static str {
        match self {
            Self::Count => "n",
            Self::Pdf => "1/x",
            Self::Cdf => "P(X ≤ x)",
            Self::Percent => "%",
        }
    }
}

/// A right-continuous empirical CDF. Each distinct observation has a vertical
/// jump from the preceding population fraction to the inclusive fraction.
#[derive(Debug, Clone)]
pub struct EmpiricalCdf {
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
pub struct HistogramDistribution {
    pub mode: HistogramDisplayMode,
    /// One ordinate per bin for Count, PDF and Percent. Empty for the CDF,
    /// whose jumps are observations rather than bin edges or centers.
    pub ordinates: Vec<f64>,
    pub cdf: Option<EmpiricalCdf>,
}

impl HistogramDistribution {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::histogram::HistogramBuilder;

    #[test]
    fn normalization_includes_samples_outside_the_custom_range() {
        let samples = [-1.0, 0.0, 1.0, 2.0, 3.0];
        let histogram = HistogramBuilder::new()
            .bin_count(2)
            .range(0.0, 2.0)
            .build(&samples);
        let density =
            HistogramDistribution::new(&histogram, &samples, HistogramDisplayMode::Pdf).unwrap();
        assert_eq!(density.ordinates, vec![0.2, 0.4]);
        let percent =
            HistogramDistribution::new(&histogram, &samples, HistogramDisplayMode::Percent)
                .unwrap();
        assert_eq!(percent.ordinates, vec![20.0, 40.0]);
        assert_eq!(percent.value_at(&histogram, 1.0), 40.0);
        assert_eq!(percent.value_at(&histogram, 2.0), 40.0);
        assert_eq!(percent.value_at(&histogram, 3.0), 0.0);
    }

    #[test]
    fn density_preserves_extreme_widths_and_rejects_point_mass() {
        let samples = [1e308; 4];
        let histogram = HistogramBuilder::new()
            .bin_count(1)
            .range(0.0, 1.5e308)
            .build(&samples);
        let density =
            HistogramDistribution::new(&histogram, &samples, HistogramDisplayMode::Pdf).unwrap();
        assert!(density.ordinates[0] > 0.0);
        assert!((density.ordinates[0] * 1.5e308 - 1.0).abs() < 1e-14);
        let histogram = HistogramBuilder::new()
            .bin_count(1)
            .range(-1.5e308, 1.5e308)
            .build(&samples);
        let density =
            HistogramDistribution::new(&histogram, &samples, HistogramDisplayMode::Pdf).unwrap();
        assert!((density.ordinates[0] * 1.5e308 - 0.5).abs() < 1e-14);
        let histogram = HistogramBuilder::new().build(&samples);
        assert!(
            HistogramDistribution::new(&histogram, &samples, HistogramDisplayMode::Pdf).is_err()
        );
        let cdf =
            HistogramDistribution::new(&histogram, &samples, HistogramDisplayMode::Cdf).unwrap();
        assert_eq!(cdf.value_at(&histogram, 1e308_f64.next_down()), 0.0);
        assert_eq!(cdf.value_at(&histogram, 1e308), 1.0);
    }
}
