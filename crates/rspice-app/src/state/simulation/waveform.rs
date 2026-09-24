//! Waveform data.
//!
//! The sampled traces a run produces, their complex form for AC-domain
//! results, and the display cache that keeps redraw off the raw arrays.

use super::*;
use std::sync::Arc;

pub type SharedWaveformValues = Arc<Vec<Value>>;
pub const DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES: usize = 4_096;

/// Bounded presentation cache derived from authoritative f64 samples.
///
/// The cache deliberately stores extrema pairs rather than uniform samples,
/// so narrow spikes remain visible while the source vectors keep their exact
/// f64/complex128 values for measurement, export, and persistence.
#[derive(Debug, Clone, PartialEq)]
pub struct DisplayWaveformCache {
    pub x: Arc<Vec<f32>>,
    pub y: Arc<Vec<f32>>,
    pub source_sample_count: usize,
}

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

/// Waveform trace data
#[derive(Debug, Clone, PartialEq)]
pub struct WaveformData {
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

    /// Trace color (hex string)
    pub color: String,

    /// Optional original complex samples for export and downstream analysis.
    pub complex: Option<ComplexWaveformComponents>,

    /// Whether this trace is visible
    pub visible: bool,

    /// Optional bounded f32 presentation cache. It is derived data and is
    /// intentionally omitted from project persistence.
    pub display_cache: Option<DisplayWaveformCache>,
}

impl WaveformData {
    /// Create a new waveform trace
    pub fn new(
        name: impl Into<String>,
        x: impl Into<SharedWaveformValues>,
        y: impl Into<SharedWaveformValues>,
        color: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            x: x.into(),
            y: y.into(),
            unit: None,
            color: color.into(),
            complex: None,
            visible: true,
            display_cache: None,
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

    /// Build an extrema-preserving f32 cache without modifying source data.
    #[must_use]
    #[cfg(test)]
    pub fn with_display_cache(mut self, maximum_samples: usize) -> Self {
        self.rebuild_display_cache(maximum_samples);
        self
    }

    pub fn rebuild_display_cache(&mut self, maximum_samples: usize) {
        let count = self.x.len().min(self.y.len());
        if count == 0 || maximum_samples == 0 {
            self.display_cache = None;
            return;
        }
        let limit = maximum_samples.max(2).min(count);
        let indices = extrema_cache_indices(&self.y[..count], limit);
        self.display_cache = Some(DisplayWaveformCache {
            x: Arc::new(indices.iter().map(|&index| self.x[index] as f32).collect()),
            y: Arc::new(indices.iter().map(|&index| self.y[index] as f32).collect()),
            source_sample_count: count,
        });
    }

    /// A bounded live preview consists of exact source knots. Display-cache
    /// f32 coordinates cannot be promoted back into measurement inputs: they
    /// can collapse distinct abscissas or erase very small signal values.
    pub(crate) fn into_bounded_preview(mut self, maximum_samples: usize) -> Result<Self, String> {
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
        self.rebuild_display_cache(maximum_samples);
        if let Some(cache) = &mut self.display_cache {
            cache.source_sample_count = count;
        }
        Ok(self)
    }

    /// Get the X range (min, max)
    pub fn x_range(&self) -> (Value, Value) {
        let min = self.x.iter().copied().fold(Value::INFINITY, Value::min);
        let max = self.x.iter().copied().fold(Value::NEG_INFINITY, Value::max);
        (min, max)
    }

    /// Get the Y range (min, max)
    pub fn y_range(&self) -> (Value, Value) {
        let min = self.y.iter().copied().fold(Value::INFINITY, Value::min);
        let max = self.y.iter().copied().fold(Value::NEG_INFINITY, Value::max);
        (min, max)
    }
}

fn extrema_cache_indices(values: &[Value], limit: usize) -> Vec<usize> {
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

#[cfg(test)]
mod display_cache_tests {
    use super::*;

    #[test]
    fn complex_preview_retains_exact_shared_knots_and_component_extrema() {
        let x = (0..10_000)
            .map(|index| 1e-4 + index as f64 * 1e-18)
            .collect::<Vec<_>>();
        let mut real = vec![1e-120; x.len()];
        let mut imag = vec![0.0; x.len()];
        real[1234] = -1e-120;
        imag[6789] = 1e-120;
        let source = WaveformData::new("signal", x.clone(), vec![1e-120; x.len()], "#fff")
            .with_complex_components("V(out)", real.clone(), imag.clone());
        let preview = source.clone().into_bounded_preview(256).unwrap();
        assert!(preview.x.len() <= 256);
        assert!(preview.x.windows(2).all(|pair| pair[0] < pair[1]));
        let complex = preview.complex.as_ref().unwrap();
        assert!(complex.real.contains(&-1e-120));
        assert!(complex.imag.contains(&1e-120));
        for (index, &point) in preview.x.iter().enumerate() {
            let source_index = x
                .binary_search_by(|candidate| candidate.total_cmp(&point))
                .unwrap();
            assert_eq!(preview.y[index].to_bits(), source.y[source_index].to_bits());
            assert_eq!(complex.real[index].to_bits(), real[source_index].to_bits());
            assert_eq!(complex.imag[index].to_bits(), imag[source_index].to_bits());
        }
        assert_eq!(source.x.len(), 10_000);
        assert_eq!(preview.display_cache.unwrap().source_sample_count, 10_000);
    }

    #[test]
    fn preview_rejects_misaligned_sources_before_slicing() {
        for malformed_complex in [false, true] {
            let mut wave = WaveformData::new("signal", vec![0.0, 1.0], vec![0.0], "#fff");
            if malformed_complex {
                wave.y = vec![0.0; 2].into();
                wave = wave.with_complex_components("V(out)", vec![0.0; 2], vec![0.0]);
            }
            assert!(wave.into_bounded_preview(2).is_err());
        }
    }

    #[test]
    fn cache_is_bounded_preserves_endpoints_and_narrow_extrema() {
        let x = (0..10_000).map(|value| value as f64).collect::<Vec<_>>();
        let mut y = vec![0.0; x.len()];
        y[4_321] = 123.0;
        y[7_654] = -91.0;
        let waveform =
            WaveformData::new("V(out)", x.clone(), y.clone(), "#fff").with_display_cache(256);
        let cache = waveform.display_cache.expect("display cache");
        assert_eq!(cache.x.len(), 256);
        assert_eq!(cache.x.first().copied(), Some(x[0] as f32));
        assert_eq!(cache.x.last().copied(), Some(x[x.len() - 1] as f32));
        assert!(cache.y.contains(&123.0));
        assert!(cache.y.contains(&-91.0));
        assert_eq!(waveform.y[4_321].to_bits(), y[4_321].to_bits());
    }

    #[test]
    fn flat_cache_has_deterministic_exact_capacity() {
        let waveform = WaveformData::new(
            "V(out)",
            (0..10_000).map(|value| value as f64).collect::<Vec<_>>(),
            vec![1.0; 10_000],
            "#fff",
        )
        .with_display_cache(255);
        let cache = waveform.display_cache.expect("display cache");
        assert_eq!(cache.x.len(), 255);
        assert_eq!(cache.y.len(), 255);
    }
}
