//! QPAC phasors plotted against probe offset with explicit tuple identities.
use super::*;
use rspice_core::engine::QpacAnalysisResult;
use std::sync::Arc;

impl SimulationResult {
    pub(crate) fn from_qpac_response(response: Arc<QpacAnalysisResult>) -> Result<Self, String> {
        let waveforms = Self::qpac_waveforms(&response)?;
        Ok(Self::Qpac {
            frequencies: response.metadata.request.offsets_hz.clone(),
            waveforms,
            response,
        })
    }

    pub(crate) fn qpac_waveforms(
        response: &QpacAnalysisResult,
    ) -> Result<HashMap<String, WaveformData>, String> {
        let traces = crate::state::AnalysisResultPayload::qpac_display_traces(response)?;
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

#[cfg(test)]
impl SimulationResult {
    pub(crate) fn qpac_retained_test_fixture() -> crate::state::AnalysisResult {
        crate::simulation::controller::SimulationController::new()
            .convert_to_analysis_result_with_metadata_owned(
                Self::qpac_test_fixture(),
                crate::state::AnalysisType::Qpac,
                "QPAC",
            )
    }

    pub(crate) fn qpac_test_fixture() -> Self {
        let deck = "QPAC retained response\nV1 in 0 DC 1\nIprobe 0 out DC 0\nR1 in out 1k\nC1 out 0 100n\n.end\n";
        let netlist = rspice_core::Netlist::parse(deck).unwrap();
        let engine = rspice_core::engine::Engine::new(Default::default());
        let point = engine
            .run_qpss(
                &netlist,
                rspice_core::engine::QpssConfig::new(vec![1000.0, 1414.213562373095], vec![1, 1]),
            )
            .unwrap();
        let request = rspice_core::engine::QpacRequest {
            offsets_hz: vec![-37.0, 0.0, 127.0],
            input_source: "Iprobe".into(),
            input_lattice: vec![1, -1],
            output_node: "out".into(),
            output_ref: "0".into(),
            output_lattice: vec![1, -1],
            magnitude: 0.002,
            phase_degrees: 73.0,
            solver: Default::default(),
        };
        let result = engine
            .run_qpac_from_qpss(&netlist, request, &point)
            .unwrap();
        Self::from_qpac_response(Arc::new(result)).unwrap()
    }
}
