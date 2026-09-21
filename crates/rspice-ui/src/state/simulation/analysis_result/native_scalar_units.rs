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

impl AnalysisResultPayload {
    fn produced_scalar_unit(&self, name: &str) -> MeasurementUnit {
        let symbol = match self {
            Self::PssFloquet { .. } => match name {
                "pss_period" => "s",
                "pss_fundamental_frequency" => "Hz",
                "pss_mode_count" => "count",
                _ => return MeasurementUnit::Unknown,
            },
            Self::Pstb { .. } => match name {
                "pstb_period" => "s",
                "pstb_fundamental_frequency" => "Hz",
                "pstb_mode_count" | "pstb_unstable_mode_count" => "count",
                "pstb_max_multiplier_magnitude" => "1",
                "pstb_min_stability_margin_db" => "dB",
                _ => return MeasurementUnit::Unknown,
            },
            Self::DcMismatch { evidence } => &evidence.output_unit,
            Self::FftSpectrum { spectrum } => match name {
                "fft_thd_db" | "fft_sndr_db" | "fft_snr_db" | "fft_sfdr_db" => "dB",
                "fft_enob_bits" => "bits",
                "fft_fundamental_magnitude" => {
                    return rspice_core::execution::transient_fft_output_unit(
                        &spectrum.physical_type,
                        spectrum.format.core(),
                    )
                    .ok()
                    .and_then(|unit| MeasurementUnit::known(&unit.symbol()).ok())
                    .unwrap_or(MeasurementUnit::Unknown);
                }
                _ => return MeasurementUnit::Unknown,
            },
            // Generic scalar results have no producer quantity metadata.
            _ => return MeasurementUnit::Unknown,
        };
        MeasurementUnit::known(symbol).unwrap_or(MeasurementUnit::Unknown)
    }
}
