//! Units retained at the native result boundary, never inferred on history load.
use super::*;
use rspice_core::analysis::MeasurementUnit;

impl AnalysisResult {
    /// Called only when materializing a new runner result. A missing map on
    /// historical evidence preserves its original numeric limit interpretation.
    pub(crate) fn retain_native_scalar_units(&mut self) {
        self.native_scalar_units = self.result_payload.as_ref().and_then(|payload| {
            let names = payload.scalar_evidence_names();
            (!names.is_empty()).then(|| {
                names
                    .into_iter()
                    .map(|name| {
                        let unit = payload.produced_scalar_unit(&name);
                        (name, unit)
                    })
                    .collect()
            })
        });
    }

    pub(super) fn native_scalar_unit(&self, name: &str) -> Option<MeasurementUnit> {
        let units = self.native_scalar_units.as_ref()?;
        let payload = self.result_payload.as_ref()?;
        units
            .iter()
            .find(|(canonical, _)| {
                canonical.eq_ignore_ascii_case(name)
                    || !matches!(payload, AnalysisResultPayload::ScalarMeasurements { .. })
                        && native_scalar_name_matches(
                            name,
                            canonical,
                            &canonical.replacen('_', ".", 1),
                        )
            })
            .map(|(_, unit)| unit.clone())
    }

    pub(super) fn validate_native_scalar_units(&self) -> Result<(), String> {
        let Some(units) = &self.native_scalar_units else {
            return Ok(());
        };
        let names = self
            .result_payload
            .as_ref()
            .ok_or("native scalar units require an analysis payload")?
            .scalar_evidence_names();
        if names.is_empty()
            || units.len() != names.len()
            || names.iter().any(|name| !units.contains_key(name))
        {
            return Err("native scalar units do not match the payload's scalar roster".into());
        }
        for unit in units.values() {
            unit.validate()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rspice_results::fft::FftSpectrumFormatEvidence;
    use rspice_results::fft::spectrum::{
        FftMetricsEvidence, FftSpectrumEvidence, FftSpectrumModeEvidence, FftSpectrumStatusEvidence,
    };

    fn fixture() -> FftSpectrumEvidence {
        FftSpectrumEvidence {
            status: FftSpectrumStatusEvidence::Complete,
            output: "V(OUT)".to_owned(),
            physical_type: "voltage".to_owned(),
            start_time_s: 0.0,
            stop_time_s: 8.0e-3,
            sample_interval_s: 8.0e-3 / 256.0,
            point_count: 256,
            accurate_sampling: true,
            format: FftSpectrumFormatEvidence::Unnormalized,
            mode: FftSpectrumModeEvidence::HspiceCompatible,
            window: "RECT".to_owned(),
            alpha: 3.0,
            coherent_gain: 1.0,
            frequency_resolution_hz: 125.0,
            fundamental_bin: 1,
            minimum_metric_bin: 1,
            maximum_metric_bin: 128,
            metrics: None,
        }
    }

    #[test]
    fn native_scalar_units_keep_fft_normalization_and_unknown_quantities() {
        use crate::state::{AnalysisResult, AnalysisResultPayload, AnalysisType};
        for (physical, format, target, expected) in [
            (
                "voltage",
                FftSpectrumFormatEvidence::Unnormalized,
                "mV",
                Some(250.0),
            ),
            (
                "current",
                FftSpectrumFormatEvidence::Unnormalized,
                "mA",
                Some(250.0),
            ),
            (
                "voltage",
                FftSpectrumFormatEvidence::Normalized,
                "%",
                Some(25.0),
            ),
            (
                "parameter",
                FftSpectrumFormatEvidence::Unnormalized,
                "V",
                None,
            ),
        ] {
            let mut spectrum = fixture();
            spectrum.physical_type = physical.into();
            spectrum.format = format;
            spectrum.metrics = Some(FftMetricsEvidence {
                fundamental_magnitude: 0.25,
                thd_ratio: 0.01,
                thd_db: -40.0,
                sndr_db: 40.0,
                enob_bits: 6.35,
                snr_db: 41.0,
                sfdr_db: 45.0,
                sfdr_spur_bin: None,
                sfdr_spur_frequency_hz: None,
                largest_harmonics: vec![],
            });
            spectrum.validate().unwrap();
            let mut result = AnalysisResult::new(1, AnalysisType::Fourier, "FFT")
                .with_result_payload(AnalysisResultPayload::FftSpectrum { spectrum });
            result.retain_native_scalar_units();
            let scalar = result.scalar_evidence("FFT.fundamental_magnitude");
            match expected {
                Some(expected) => {
                    assert_eq!(scalar[0].value_in_unit(target).unwrap(), Some(expected))
                }
                None => assert!(scalar[0].value_in_unit(target).is_err()),
            }
            assert_eq!(
                result.scalar_evidence("fft_enob_bits")[0]
                    .value_in_unit("bits")
                    .unwrap(),
                Some(6.35)
            );
            assert!(
                result.scalar_evidence("fft_thd_db")[0]
                    .value_in_unit("V")
                    .is_err()
            );
        }
    }
}
