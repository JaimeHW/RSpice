//! Distribution presentation settings. Samples belong to retained results;
//! derived bins and descriptive moments belong to the result view plan.

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

/// Histogram presentation settings, independent of the active population.
#[derive(Debug, Clone)]
pub struct HistogramState {
    /// Display mode
    pub mode: HistogramDisplayMode,
    /// Exact measurement name; ordering is not a measurement identity.
    pub selected: Option<String>,
    /// Number of bins (for rebuilding)
    pub bin_count: usize,
    /// Custom range enabled
    pub custom_range: bool,
    /// Custom range values
    pub custom_min: f64,
    pub custom_max: f64,
}

impl Default for HistogramState {
    fn default() -> Self {
        Self {
            mode: HistogramDisplayMode::Count,
            selected: None,
            bin_count: 50,
            custom_range: false,
            custom_min: 0.0,
            custom_max: 1.0,
        }
    }
}

impl HistogramState {
    /// Forget the selected measurement when the result context is cleared.
    pub fn clear_selection(&mut self) {
        self.selected = None;
    }
}

/// Name-based selection shared by interactive and prepared print consumers.
/// The index fallback is only for initial selection and older print snapshots.
pub(crate) fn measurement_index(
    selected: Option<&str>,
    legacy_index: usize,
    names: &[&str],
) -> Option<usize> {
    let target = selected.or_else(|| {
        names
            .get(legacy_index.min(names.len().saturating_sub(1)))
            .copied()
    })?;
    let mut matches = names
        .iter()
        .enumerate()
        .filter(|(_, name)| **name == target);
    let index = matches.next()?.0;
    matches.next().is_none().then_some(index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_measurement_survives_reordering_and_never_substitutes_a_missing_name() {
        assert_eq!(
            measurement_index(Some("gain"), 0, &["gain", "offset"]),
            Some(0)
        );
        assert_eq!(
            measurement_index(Some("gain"), 0, &["offset", "gain"]),
            Some(1)
        );
        assert_eq!(measurement_index(Some("gain"), 0, &["offset"]), None);
        assert_eq!(measurement_index(Some("gain"), 0, &["gain", "gain"]), None);
        assert_eq!(measurement_index(None, 0, &["offset", "gain"]), Some(0));
        assert_eq!(
            measurement_index(None, usize::MAX, &["offset", "gain"]),
            Some(1)
        );
        assert_eq!(measurement_index(None, 0, &[]), None);
    }
}
