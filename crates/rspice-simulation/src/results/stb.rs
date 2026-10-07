//! Compatibility projections are derived from the complete STB evidence on demand.
use super::*;
use rspice_core::analysis::stb::StbResult;

impl SimulationResult {
    pub fn stb_waveforms(response: &StbResult) -> Result<HashMap<String, WaveformData>, String> {
        let mut waveforms = HashMap::new();
        for trace in
            rspice_results::analysis_payload::AnalysisResultPayload::stb_waveforms(response)?
        {
            waveforms.insert(
                trace.name.clone(),
                WaveformData {
                    name: trace.name,
                    x_values: Arc::unwrap_or_clone(trace.x),
                    y_values: Arc::unwrap_or_clone(trace.y),
                    y_unit: trace.unit.unwrap_or_default(),
                    is_complex: false,
                    y_imag: None,
                },
            );
        }
        if !response.nyquist_points.is_empty() {
            waveforms.insert(
                STB_NYQUIST_CONTOUR_WAVEFORM.into(),
                WaveformData::new_complex_in_unit(
                    STB_NYQUIST_CONTOUR_WAVEFORM,
                    response
                        .nyquist_points
                        .iter()
                        .map(|p| p.frequency)
                        .collect(),
                    response.nyquist_points.iter().map(|p| p.real).collect(),
                    response.nyquist_points.iter().map(|p| p.imag).collect(),
                    "1",
                ),
            );
        }
        Ok(waveforms)
    }

    pub(super) fn stb_measurement_projection(&self) -> Option<Self> {
        let Self::Stb {
            response,
            measurements,
        } = self
        else {
            return None;
        };
        Some(Self::Ac {
            frequencies: response.bode_points.iter().map(|p| p.frequency).collect(),
            waveforms: Self::stb_waveforms(response).ok()?,
            measurements: measurements.clone(),
            convergence: None,
            reference_impedances_ohm: None,
            noise_reference_temperature_kelvin: None,
        })
    }
}
use std::sync::Arc;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stb_studies_keep_defined_raw_zero_but_no_phase_at_zero() {
        let response = rspice_core::analysis::stb::StbAnalyzer::new(Default::default())
            .analyze(&[1.0], &[rspice_core::Complex64::new(0.0, 0.0)])
            .unwrap();
        let result = SimulationResult::Stb {
            response: Arc::new(response),
            measurements: Vec::new(),
        };
        assert!(result.study_measurement("bin:0:phase").is_none());
        let real = result.study_measurement("bin:0:real").unwrap();
        assert_eq!(real.value, Some(0.0));
        let magnitude = result.study_measurement("bin:0:magnitude").unwrap();
        assert_eq!(magnitude.value, Some(0.0));
    }
}
