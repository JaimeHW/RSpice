//! Complete QPNOISE evidence with typed physical-frequency display traces.
use super::*;
use rspice_core::engine::QpnoiseAnalysisResult;
use std::sync::Arc;
impl SimulationResult {
    pub(crate) fn from_qpnoise_response(
        response: Arc<QpnoiseAnalysisResult>,
    ) -> Result<Self, String> {
        let waveforms = Self::qpnoise_waveforms(&response)?;
        Ok(Self::Qpnoise {
            frequencies: response.outputs[0].frequencies_hz.clone(),
            waveforms,
            response,
        })
    }
    pub(crate) fn qpnoise_waveforms(
        response: &QpnoiseAnalysisResult,
    ) -> Result<HashMap<String, WaveformData>, String> {
        let mut waveforms = HashMap::new();
        for trace in crate::state::AnalysisResultPayload::qpnoise_display_traces(response)? {
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

#[cfg(test)]
impl SimulationResult {
    pub(crate) fn qpnoise_retained_test_fixture() -> crate::state::AnalysisResult {
        crate::simulation::controller::SimulationController::new()
            .convert_to_analysis_result_with_metadata_owned(
                Self::qpnoise_test_fixture(),
                crate::state::AnalysisType::Qpnoise,
                "QPNOISE",
            )
    }
    pub(crate) fn qpnoise_test_fixture() -> Self {
        let netlist=rspice_core::Netlist::parse("Noise outputs\nV1 in 0 DC 1\nRs in out 1k\nRl out 0 2k\nL1 out sense 1m\nR3 sense 0 100\nC1 out 0 100n\n.end\n").unwrap();
        let engine = rspice_core::engine::Engine::default();
        let point = engine
            .run_qpss(
                &netlist,
                rspice_core::engine::QpssConfig::new(vec![1000.0, 1414.213562373095], vec![1, 1]),
            )
            .unwrap();
        let request = rspice_core::engine::QpnoiseRequest::from_qpnoise_card(
            &crate::simulation::plan::QuasiPeriodicNoiseDraft {
                explicit_frequencies: "100,300,700".into(),
                additional_outputs: vec![
                    crate::simulation::plan::QpnoiseOutputDraft {
                        current: true,
                        branch: "L1".into(),
                        ..Default::default()
                    },
                    crate::simulation::plan::QpnoiseOutputDraft {
                        lattice: "1,-1".into(),
                        ..Default::default()
                    },
                ],
                noise_figure: true,
                source_resistor: "Rs".into(),
                integration_method: rspice_core::engine::QpnoiseIntegrationMethod::LogLog,
                ..Default::default()
            }
            .to_spec()
            .unwrap()
            .qpnoise_card()
            .unwrap(),
        )
        .unwrap();
        Self::from_qpnoise_response(Arc::new(
            engine
                .run_qpnoise_from_qpss(&netlist, request, &point)
                .unwrap(),
        ))
        .unwrap()
    }
}
