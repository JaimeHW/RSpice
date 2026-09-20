//! QPXF plots expose unit transfers and finite sampled delay on the physical output axis.
use super::*;
use rspice_core::engine::QpxfAnalysisResult;
use std::sync::Arc;
impl SimulationResult {
    pub(crate) fn from_qpxf_response(response: Arc<QpxfAnalysisResult>) -> Result<Self, String> {
        let waveforms = Self::qpxf_waveforms(&response)?;
        Ok(Self::Qpxf {
            frequencies: response.metadata.output_frequencies_hz.clone(),
            waveforms,
            response,
        })
    }
    pub(crate) fn qpxf_waveforms(
        response: &QpxfAnalysisResult,
    ) -> Result<HashMap<String, WaveformData>, String> {
        let mut waveforms = HashMap::new();
        for trace in crate::state::AnalysisResultPayload::qpxf_display_traces(response)? {
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
#[cfg(test)]
impl SimulationResult {
    pub(crate) fn qpxf_retained_test_fixture() -> crate::state::AnalysisResult {
        crate::simulation::controller::SimulationController::new()
            .convert_to_analysis_result_with_metadata_owned(
                Self::qpxf_test_fixture(),
                crate::state::AnalysisType::Qpxf,
                "QPXF",
            )
    }
    pub(crate) fn qpxf_test_fixture() -> Self {
        use rspice_core::engine::*;
        let netlist = rspice_core::Netlist::parse(
            "QPXF response\nV1 in 0 DC 1\nIprobe 0 out DC 0\nR1 in out 1k\nC1 out 0 100n\n.end\n",
        )
        .unwrap();
        let engine = Engine::new(Default::default());
        let point = engine
            .run_qpss(
                &netlist,
                QpssConfig::new(vec![1000.0, 1414.2135623730951], vec![1, 1]),
            )
            .unwrap();
        let request = QpxfRequest {
            frequencies_hz: vec![-37.0, 0.0, 127.0],
            frequency_axis: QpxfFrequencyAxis::Output,
            input_sources: QpxfSources::AllIndependent,
            input_lattices: QpxfInputLattices::Explicit(vec![vec![1, -1], vec![0, 0]]),
            output: QpxfOutput::Voltage {
                positive: "out".into(),
                negative: "0".into(),
            },
            output_lattice: vec![1, -1],
            linear: Default::default(),
            group_delay: true,
            group_delay_magnitude_floor: 1e-8,
        };
        Self::from_qpxf_response(Arc::new(
            engine
                .run_qpxf_from_qpss(&netlist, request, &point)
                .unwrap(),
        ))
        .unwrap()
    }
}
