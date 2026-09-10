//! Exact source rows and coordinates supplied before expression evaluation.

use super::EvaluationError;
use crate::state::{SharedWaveformValues, WaveformData};

#[derive(Clone, Copy)]
pub(super) struct SampleProjection<'a> {
    indices: &'a [usize],
    axis: Option<&'a [f64]>,
}

impl<'a> SampleProjection<'a> {
    pub(super) fn new(
        indices: &'a [usize],
        axis: Option<&'a [f64]>,
    ) -> Result<Self, EvaluationError> {
        if indices.is_empty() {
            return Err(invalid("the expression selection contains no samples"));
        }
        if indices.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(invalid("selected source rows must be unique and ascending"));
        }
        if let Some(axis) = axis {
            super::interpolation::validate_samples(axis, indices)
                .map_err(|error| invalid(&error.to_string()))?;
        }
        Ok(Self { indices, axis })
    }

    pub(super) fn apply(&self, source: &WaveformData) -> Result<WaveformData, EvaluationError> {
        let length = source.x.len();
        if source.y.len() != length
            || source
                .complex
                .as_ref()
                .is_some_and(|complex| complex.real.len() != length || complex.imag.len() != length)
        {
            return Err(invalid(&format!(
                "{} has mismatched sample columns",
                source.name
            )));
        }
        if self.indices.last().is_some_and(|index| *index >= length) {
            return Err(invalid(&format!(
                "selected rows exceed {} samples",
                source.name
            )));
        }
        let select = |values: &SharedWaveformValues| -> SharedWaveformValues {
            self.indices
                .iter()
                .map(|index| values[*index])
                .collect::<Vec<_>>()
                .into()
        };
        let mut projected = source.clone();
        projected.x = self
            .axis
            .map_or_else(|| select(&source.x), |axis| axis.to_vec().into());
        projected.y = select(&source.y);
        if let Some(complex) = &mut projected.complex {
            complex.real = select(&complex.real);
            complex.imag = select(&complex.imag);
        }
        projected.display_cache = None;
        Ok(projected)
    }
}

fn invalid(message: &str) -> EvaluationError {
    EvaluationError::WaveformMismatch(message.to_owned())
}

#[cfg(test)]
mod tests {
    use super::super::{CalcValue, EvaluationContext, RealValue, WaveformsContext};
    use super::*;

    #[test]
    fn row_projection_preserves_complex_evidence_and_the_declared_coordinate() {
        let source = WaveformData::new(
            "V(out)",
            vec![100.0, 101.0, 102.0],
            vec![5.0, 13.0, 25.0],
            "#fff",
        )
        .with_complex_components("V(out)", vec![3.0, 5.0, 7.0], vec![4.0, 12.0, 24.0]);
        let before = source.clone();
        let projected = SampleProjection::new(&[0, 2], Some(&[0.0, 0.5]))
            .unwrap()
            .apply(&source)
            .unwrap();
        assert_eq!(projected.x.as_slice(), &[0.0, 0.5]);
        assert_eq!(projected.y.as_slice(), &[5.0, 25.0]);
        let complex = projected.complex.unwrap();
        assert_eq!(complex.real.as_slice(), &[3.0, 7.0]);
        assert_eq!(complex.imag.as_slice(), &[4.0, 24.0]);
        assert_eq!(source, before);
    }

    #[test]
    fn row_projection_rejects_ambiguous_maps_and_malformed_source_columns() {
        for indices in [vec![], vec![1, 1], vec![1, 0]] {
            assert!(SampleProjection::new(&indices, None).is_err());
        }
        assert!(SampleProjection::new(&[0, 1], Some(&[0.0])).is_err());
        assert!(SampleProjection::new(&[0], Some(&[f64::NAN])).is_err());
        let source = WaveformData::new("V(out)", vec![0.0, 1.0], vec![2.0, 3.0], "#fff");
        assert!(
            SampleProjection::new(&[2], None)
                .unwrap()
                .apply(&source)
                .is_err()
        );
        let short = source.with_complex_components("V(out)", vec![1.0], vec![0.0, 0.0]);
        assert!(
            SampleProjection::new(&[0], None)
                .unwrap()
                .apply(&short)
                .is_err()
        );
    }

    #[test]
    fn projected_context_keeps_ground_coordinates_and_magnitude_fallback_in_scope() {
        let source = [WaveformData::new(
            "|V(out)|",
            vec![100.0, 101.0, 102.0],
            vec![10.0, 1.0, 20.0],
            "#fff",
        )];
        let ctx = WaveformsContext::new(&source)
            .with_sample_projection(&[1], Some(&[4.0]))
            .unwrap();
        for (signal, expected) in [("TIME", 4.0), ("V(0)", 0.0)] {
            assert_eq!(
                ctx.get_waveform(signal, None).unwrap(),
                CalcValue::Real(RealValue::Waveform(vec![4.0], vec![expected]))
            );
        }
        assert_eq!(
            ctx.get_magnitude("V(out)", None).unwrap(),
            CalcValue::Real(RealValue::Waveform(vec![4.0], vec![1.0]))
        );
        assert!(
            ctx.get_waveform("V(out)", None).is_err(),
            "magnitude cannot invent phase"
        );
        assert!(
            WaveformsContext::new(&source)
                .with_sample_projection(&[3], Some(&[4.0]))
                .is_err()
        );
    }
}
