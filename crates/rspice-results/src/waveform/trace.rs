//! Display choices over exact retained waveform samples.

use super::{OutputWaveform, RetainedWaveform, SharedWaveformValues};
use std::ops::{Deref, DerefMut};

/// Samples a live preview keeps of each waveform while its run is solving.
pub const DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES: usize = 4_096;

/// A retained waveform with its display choices.
#[derive(Debug, Clone, PartialEq)]
pub struct RetainedTrace {
    pub data: RetainedWaveform,
    /// Persisted trace color (hex string).
    pub color: String,
    /// Persisted visibility selection.
    pub visible: bool,
}

impl Deref for RetainedTrace {
    type Target = RetainedWaveform;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for RetainedTrace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl AsRef<RetainedWaveform> for RetainedTrace {
    fn as_ref(&self) -> &RetainedWaveform {
        &self.data
    }
}

impl AsMut<RetainedWaveform> for RetainedTrace {
    fn as_mut(&mut self) -> &mut RetainedWaveform {
        &mut self.data
    }
}

impl RetainedTrace {
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

    pub fn into_bounded_preview(mut self, maximum_samples: usize) -> Result<Self, String> {
        self.data = self.data.into_bounded_preview(maximum_samples)?;
        Ok(self)
    }
}

impl OutputWaveform for RetainedTrace {
    fn from_retained(data: RetainedWaveform) -> Self {
        Self {
            data,
            color: "#f5b700".to_owned(),
            visible: true,
        }
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn adopt_presentation(&mut self, source: Self) {
        self.visible = source.visible;
    }

    fn into_output_preview(self, maximum_samples: usize) -> Result<Self, String> {
        self.into_bounded_preview(maximum_samples)
    }
}

#[cfg(test)]
mod preview_tests {
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
        let source = RetainedTrace::new("signal", x.clone(), vec![1e-120; x.len()], "#fff")
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
    }

    #[test]
    fn preview_rejects_misaligned_sources_before_slicing() {
        for malformed_complex in [false, true] {
            let mut wave = RetainedTrace::new("signal", vec![0.0, 1.0], vec![0.0], "#fff");
            if malformed_complex {
                wave.y = vec![0.0; 2].into();
                wave = wave.with_complex_components("V(out)", vec![0.0; 2], vec![0.0]);
            }
            assert!(wave.into_bounded_preview(2).is_err());
        }
    }
}
