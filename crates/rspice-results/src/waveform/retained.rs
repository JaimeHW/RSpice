//! Exact shared waveform samples, independent of application presentation.

use std::sync::Arc;

pub type SharedWaveformValues = Arc<Vec<f64>>;

/// Original complex samples associated with a display trace.
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexWaveformComponents {
    /// Source signal name before display transformations such as magnitude.
    pub source_name: String,

    /// Real component samples.
    pub real: SharedWaveformValues,

    /// Imaginary component samples.
    pub imag: SharedWaveformValues,
}

/// Exact retained samples and their producer-stated physical meaning.
#[derive(Debug, Clone, PartialEq)]
pub struct RetainedWaveform {
    /// Trace name (e.g., "V(out)")
    pub name: String,

    /// X-axis values (time or frequency)
    pub x: SharedWaveformValues,

    /// Y-axis values
    pub y: SharedWaveformValues,

    /// The engineering unit the retained Y samples are measured in, as the
    /// producer stated it.
    ///
    /// `None` does not mean dimensionless — it means unstated, which is what
    /// every project written before this field carried and what a conversion
    /// that invents a series has no right to claim. Surfaces that label a
    /// waveform fall back to reading its name, then the analysis' own
    /// quantity, exactly as they did before.
    pub unit: Option<String>,

    /// Optional original complex samples for export and downstream analysis.
    pub complex: Option<ComplexWaveformComponents>,
}

impl RetainedWaveform {
    /// Create a new waveform trace
    pub fn new(
        name: impl Into<String>,
        x: impl Into<SharedWaveformValues>,
        y: impl Into<SharedWaveformValues>,
    ) -> Self {
        Self {
            name: name.into(),
            x: x.into(),
            y: y.into(),
            unit: None,
            complex: None,
        }
    }

    /// State the unit the retained samples are measured in.
    ///
    /// The producer contract spells "no unit for this quantity" as an empty
    /// string, so that case is folded into `None` here rather than at each
    /// conversion: a waveform must never claim `""` as its unit and have a
    /// reader take that for a real one.
    #[must_use]
    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        let unit = unit.into();
        self.unit = (!unit.is_empty()).then_some(unit);
        self
    }

    /// Attach original complex samples to a derived display trace.
    pub fn with_complex_components(
        mut self,
        source_name: impl Into<String>,
        real: impl Into<SharedWaveformValues>,
        imag: impl Into<SharedWaveformValues>,
    ) -> Self {
        self.complex = Some(ComplexWaveformComponents {
            source_name: source_name.into(),
            real: real.into(),
            imag: imag.into(),
        });
        self
    }

    /// Select exact source knots for a bounded drawing cache without changing
    /// the retained samples. Empty input or a zero budget produces no cache.
    pub fn display_sample_indices(&self, maximum_samples: usize) -> Vec<usize> {
        let count = self.x.len().min(self.y.len());
        if count == 0 || maximum_samples == 0 {
            return Vec::new();
        }
        let limit = maximum_samples.max(2).min(count);
        extrema_cache_indices(&self.y[..count], limit)
    }

    /// A bounded live preview consists of exact source knots. Display-cache
    /// f32 coordinates cannot be promoted back into measurement inputs: they
    /// can collapse distinct abscissas or erase very small signal values.
    pub fn into_bounded_preview(mut self, maximum_samples: usize) -> Result<Self, String> {
        let count = self.x.len();
        if count != self.y.len()
            || self
                .complex
                .as_ref()
                .is_some_and(|value| value.real.len() != count || value.imag.len() != count)
        {
            return Err("preview source arrays are not aligned".to_owned());
        }
        let limit = maximum_samples.max(2).min(count);
        if count > limit {
            let indices = if let Some(complex) = &self.complex {
                // Include extrema of both rectangular components even where
                // their magnitude is constant. Each list shares endpoints.
                let part = (limit / 3).max(2);
                let mut indices = extrema_cache_indices(&self.y, part);
                indices.extend(extrema_cache_indices(&complex.real, part));
                indices.extend(extrema_cache_indices(&complex.imag, part));
                indices.sort_unstable();
                indices.dedup();
                indices
            } else {
                extrema_cache_indices(&self.y, limit)
            };
            let selected = |values: &SharedWaveformValues| -> SharedWaveformValues {
                indices
                    .iter()
                    .map(|&index| values[index])
                    .collect::<Vec<_>>()
                    .into()
            };
            self.x = selected(&self.x);
            self.y = selected(&self.y);
            if let Some(complex) = &mut self.complex {
                complex.real = selected(&complex.real);
                complex.imag = selected(&complex.imag);
            }
        }
        Ok(self)
    }

    /// Get the X range (min, max)
    pub fn x_range(&self) -> (f64, f64) {
        let min = self.x.iter().copied().fold(f64::INFINITY, f64::min);
        let max = self.x.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        (min, max)
    }

    /// Get the Y range (min, max)
    pub fn y_range(&self) -> (f64, f64) {
        let min = self.y.iter().copied().fold(f64::INFINITY, f64::min);
        let max = self.y.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        (min, max)
    }
}

fn extrema_cache_indices(values: &[f64], limit: usize) -> Vec<usize> {
    if values.len() <= limit {
        return (0..values.len()).collect();
    }
    if limit <= 2 {
        return vec![0, values.len() - 1];
    }

    let interior_len = values.len() - 2;
    let pair_budget = (limit - 2) / 2;
    let bucket_count = pair_budget.max(1);
    let mut indices = Vec::with_capacity(limit);
    indices.push(0);
    for bucket in 0..bucket_count {
        let start = 1 + bucket * interior_len / bucket_count;
        let end = 1 + (bucket + 1) * interior_len / bucket_count;
        if start >= end {
            continue;
        }
        let mut minimum = start;
        let mut maximum = start;
        for index in start + 1..end {
            if values[index].total_cmp(&values[minimum]).is_lt() {
                minimum = index;
            }
            if values[index].total_cmp(&values[maximum]).is_gt() {
                maximum = index;
            }
        }
        if minimum <= maximum {
            indices.push(minimum);
            if maximum != minimum {
                indices.push(maximum);
            }
        } else {
            indices.push(maximum);
            indices.push(minimum);
        }
    }
    indices.push(values.len() - 1);
    indices.sort_unstable();
    indices.dedup();
    if indices.len() > limit {
        indices.truncate(limit - 1);
        indices.push(values.len() - 1);
    }
    // Flat or repeated-valued buckets can collapse min/max to one index. Fill
    // the remaining cache capacity with deterministic, evenly spaced source
    // samples so a preflight byte estimate is exact rather than data-shaped.
    for slot in 1..limit - 1 {
        if indices.len() == limit {
            break;
        }
        let index = slot * (values.len() - 1) / (limit - 1);
        if let Err(position) = indices.binary_search(&index) {
            indices.insert(position, index);
        }
    }
    debug_assert_eq!(indices.len(), limit);
    indices
}
