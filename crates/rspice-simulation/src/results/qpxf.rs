//! QPXF plots expose unit transfers and finite sampled delay on the physical output axis.
use super::*;
use rspice_core::engine::QpxfAnalysisResult;
use std::sync::Arc;
impl SimulationResult {
    pub fn from_qpxf_response(response: Arc<QpxfAnalysisResult>) -> Result<Self, String> {
        let waveforms = Self::qpxf_waveforms(&response)?;
        Ok(Self::Qpxf {
            frequencies: response.metadata.output_frequencies_hz.clone(),
            waveforms,
            response,
        })
    }
    pub fn qpxf_waveforms(
        response: &QpxfAnalysisResult,
    ) -> Result<HashMap<String, WaveformData>, String> {
        let mut waveforms = HashMap::new();
        for trace in
            rspice_results::analysis_payload::AnalysisResultPayload::qpxf_display_traces(response)?
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
                return Err("QPXF returned duplicate trace identities".into());
            }
        }
        Ok(waveforms)
    }
}
