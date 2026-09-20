//! A deterministic, validated display of the full independent-tone solution.
use super::*;
use rspice_core::{NoAbort, engine::QpssOperatingPoint};
use std::sync::Arc;

impl SimulationResult {
    pub(crate) fn from_qpss_operating_point(
        point: Arc<QpssOperatingPoint>,
    ) -> Result<Self, String> {
        let data = crate::services::simulation_runner::qpss_data_from_operating_point_with_abort(
            point, &NoAbort,
        )
        .map_err(|error| error.to_string())?;
        let mut waveforms = HashMap::new();
        for (name, coefficients) in data.spectra {
            let (real, imaginary) = coefficients
                .iter()
                .map(|value| (value.re, value.im))
                .unzip();
            let mut waveform =
                WaveformData::new_complex(&name, data.frequencies.clone(), real, imaginary);
            waveform.y_unit = if name.starts_with("I(") { "A" } else { "V" }.into();
            if waveforms.insert(name, waveform).is_some() {
                return Err("QPSS returned duplicate spectral trace names".into());
            }
        }
        Ok(Self::Qpss {
            frequencies: data.frequencies,
            tuples: data.tuples,
            waveforms,
            operating_point: data.operating_point,
        })
    }
}
