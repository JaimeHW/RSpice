//! QPAC phasors plotted against probe offset with explicit tuple identities.
use super::*;
use rspice_core::engine::QpacAnalysisResult;
use std::sync::Arc;

impl SimulationResult {
    pub fn from_qpac_response(response: Arc<QpacAnalysisResult>) -> Result<Self, String> {
        let waveforms = Self::qpac_waveforms(&response)?;
        Ok(Self::Qpac {
            frequencies: response.metadata.request.offsets_hz.clone(),
            waveforms,
            response,
        })
    }

    pub fn qpac_waveforms(
        response: &QpacAnalysisResult,
    ) -> Result<HashMap<String, WaveformData>, String> {
        let traces =
            rspice_results::analysis_payload::AnalysisResultPayload::qpac_display_traces(response)?;
        let frequencies = response.metadata.request.offsets_hz.clone();
        let mut waveforms = HashMap::new();
        for (name, unit, values) in traces {
            let (real, imaginary) = values.into_iter().map(|v| (v.re, v.im)).unzip();
            let mut waveform =
                WaveformData::new_complex(&name, frequencies.clone(), real, imaginary);
            waveform.y_unit = unit.into();
            if waveforms.insert(name, waveform).is_some() {
                return Err("QPAC returned duplicate trace identities".into());
            }
        }
        Ok(waveforms)
    }
}
