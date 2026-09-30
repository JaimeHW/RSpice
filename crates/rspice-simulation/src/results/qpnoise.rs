//! Complete QPNOISE evidence with typed physical-frequency display traces.
use super::*;
use rspice_core::engine::QpnoiseAnalysisResult;
use std::sync::Arc;
impl SimulationResult {
    pub fn from_qpnoise_response(response: Arc<QpnoiseAnalysisResult>) -> Result<Self, String> {
        let waveforms = Self::qpnoise_waveforms(&response)?;
        Ok(Self::Qpnoise {
            frequencies: response.outputs[0].frequencies_hz.clone(),
            waveforms,
            response,
        })
    }
    pub fn qpnoise_waveforms(
        response: &QpnoiseAnalysisResult,
    ) -> Result<HashMap<String, WaveformData>, String> {
        let mut waveforms = HashMap::new();
        for trace in
            rspice_results::analysis_payload::AnalysisResultPayload::qpnoise_display_traces(
                response,
            )?
        {
            let waveform = WaveformData {
                name: trace.name.clone(),
                x_values: trace.frequencies,
                y_values: trace.real,
                y_unit: trace.unit.into(),
                is_complex: trace.imaginary.is_some(),
                y_imag: trace.imaginary,
            };
            if waveforms.insert(trace.name, waveform).is_some() {
                return Err("QPNOISE returned duplicate trace identities".into());
            }
        }
        Ok(waveforms)
    }
}
