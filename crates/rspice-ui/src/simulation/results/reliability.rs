//! Lifetime display and primary calibrated mission evidence.

use super::*;
use rspice_core::engine::ReliabilityRunResult;
use std::sync::Arc;

impl SimulationResult {
    pub(crate) fn from_reliability_response(
        response: Arc<ReliabilityRunResult>,
    ) -> Result<Self, String> {
        let waveforms = Self::reliability_waveforms(&response)?;
        Ok(Self::ReliabilityMission {
            years: response.stress.request.target_years.clone(),
            waveforms,
            response,
        })
    }
    pub(crate) fn reliability_waveforms(
        response: &ReliabilityRunResult,
    ) -> Result<HashMap<String, WaveformData>, String> {
        crate::state::AnalysisResultPayload::reliability_display_traces(response)?
            .into_iter()
            .map(|trace| {
                let name = trace.name;
                Ok((
                    name.clone(),
                    WaveformData {
                        name,
                        x_values: trace.x.as_ref().clone(),
                        y_values: trace.y.as_ref().clone(),
                        y_unit: trace.unit.unwrap_or_default(),
                        is_complex: false,
                        y_imag: None,
                    },
                ))
            })
            .collect()
    }
}

#[cfg(test)]
impl SimulationResult {
    pub(crate) fn reliability_recovery_test_fixture() -> Self {
        use crate::simulation::reliability_engine::tests::{RECOVERY_DECK, recovery_fixture};
        use rspice_core::analysis::reliability::{ReliabilityRunRequest, SECONDS_PER_AGING_YEAR};
        let request = ReliabilityRunRequest {
            study: recovery_fixture(),
            target_years: vec![1.0 / SECONDS_PER_AGING_YEAR, 2.0 / SECONDS_PER_AGING_YEAR],
            enable_hci: false,
            enable_nbti: true,
            enable_em: false,
            min_stress_voltage: 0.5,
        };
        let response = crate::services::simulation_runner::run_reliability_analysis_with_source_path_and_abort(
            RECOVERY_DECK, &request, None, &rspice_core::NoAbort).unwrap();
        Self::from_reliability_response(Arc::new(response)).unwrap()
    }

    pub(crate) fn reliability_recovery_retained_test_fixture() -> crate::state::AnalysisResult {
        crate::simulation::controller::SimulationController::new()
            .convert_to_analysis_result_with_metadata_owned(
                Self::reliability_recovery_test_fixture(),
                crate::state::AnalysisType::Reliability,
                "NBTI recovery",
            )
    }

    pub(crate) fn reliability_mission_test_fixture() -> Self {
        let mut study = crate::simulation::reliability_engine::tests::fixture();
        study.transient_stress = None;
        study.mission[0].duration_s = 1.0;
        study.mission[1].duration_s = 1.0;
        let request = rspice_core::analysis::reliability::ReliabilityRunRequest {
            study,
            target_years: vec![
                1.0 / rspice_core::analysis::reliability::SECONDS_PER_AGING_YEAR,
                2.0 / rspice_core::analysis::reliability::SECONDS_PER_AGING_YEAR,
            ],
            enable_hci: false,
            enable_nbti: true,
            enable_em: false,
            min_stress_voltage: 0.0,
        };
        let result = crate::services::simulation_runner::run_reliability_analysis_with_source_path_and_abort(
            "mission fixture\n.param VDD=1\nVS s 0 {VDD}\nVG g 0 0\nVD d 0 0.1\nM1 d g s s PM W=10u L=1u\n.model PM PMOS (LEVEL=1 VTO=-0.2 KP=100u)\n.end\n", &request, None, &rspice_core::NoAbort).unwrap();
        Self::from_reliability_response(Arc::new(result)).unwrap()
    }
    pub(crate) fn reliability_mission_retained_test_fixture() -> crate::state::AnalysisResult {
        crate::simulation::controller::SimulationController::new()
            .convert_to_analysis_result_with_metadata_owned(
                Self::reliability_mission_test_fixture(),
                crate::state::AnalysisType::Reliability,
                "Reliability mission",
            )
    }
}
