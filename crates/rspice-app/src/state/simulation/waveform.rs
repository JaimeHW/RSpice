//! Application presentation of exact retained waveform samples.

use super::SharedWaveformValues;
use rspice_results::waveform::RetainedWaveform;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

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

/// A retained waveform with its application display choices and derived cache.
#[derive(Debug, Clone, PartialEq)]
pub struct WaveformData {
    pub data: RetainedWaveform,
    /// Persisted trace color (hex string).
    pub color: String,
    /// Persisted visibility selection.
    pub visible: bool,
    /// Disposable f32 drawing data, intentionally omitted from persistence.
    pub display_cache: Option<DisplayWaveformCache>,
}

impl Deref for WaveformData {
    type Target = RetainedWaveform;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for WaveformData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl WaveformData {
    pub fn new(
        name: impl Into<String>,
        x: impl Into<SharedWaveformValues>,
        y: impl Into<SharedWaveformValues>,
        color: impl Into<String>,
    ) -> Self {
        Self {
            data: RetainedWaveform::new(name, x, y),
            color: color.into(),
            visible: true,
            display_cache: None,
        }
    }

    #[must_use]
    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.data = self.data.with_unit(unit);
        self
    }

    pub fn with_complex_components(
        mut self,
        source_name: impl Into<String>,
        real: impl Into<SharedWaveformValues>,
        imag: impl Into<SharedWaveformValues>,
    ) -> Self {
        self.data = self.data.with_complex_components(source_name, real, imag);
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
        let indices = self.data.display_sample_indices(maximum_samples);
        if indices.is_empty() {
            self.display_cache = None;
            return;
        }
        self.display_cache = Some(DisplayWaveformCache {
            x: Arc::new(
                indices
                    .iter()
                    .map(|&index| self.data.x[index] as f32)
                    .collect(),
            ),
            y: Arc::new(
                indices
                    .iter()
                    .map(|&index| self.data.y[index] as f32)
                    .collect(),
            ),
            source_sample_count: self.data.x.len().min(self.data.y.len()),
        });
    }

    pub(crate) fn into_bounded_preview(mut self, maximum_samples: usize) -> Result<Self, String> {
        let count = self.data.x.len();
        self.data = self.data.into_bounded_preview(maximum_samples)?;
        self.rebuild_display_cache(maximum_samples);
        if let Some(cache) = &mut self.display_cache {
            cache.source_sample_count = count;
        }
        Ok(self)
    }
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
        assert_eq!(waveform.data.y[4_321].to_bits(), y[4_321].to_bits());
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
