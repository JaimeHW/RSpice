//! Configured operating-point initialization for each quasiperiodic study trial.
use super::*;
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::results::SimulationResult;
use rspice_core::engine::QpssInitialState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StudyQpssConfig {
    pub request: AnalysisSpec,
    pub operating_point: StudyOperatingPoint,
}

impl StudyQpssConfig {
    pub(super) fn validate(&self) -> Result<(), String> {
        self.request.driven_qpss_config()?;
        self.operating_point.config.validate()
    }

    pub(super) fn run_with_circuit(
        &self,
        engine: &rspice_core::Engine,
        circuit: &rspice_core::Netlist,
        numeric_options: &str,
        abort: &dyn AbortSignal,
    ) -> Result<(rspice_core::Netlist, SimulationResult), SimulationError> {
        self.validate().map_err(SimulationError::InvalidConfig)?;
        let config = self
            .request
            .driven_qpss_config()
            .map_err(SimulationError::InvalidConfig)?;
        let (physical, seed) = match config.initial_state {
            QpssInitialState::DcOperatingPoint => {
                let (physical, seed) = self.operating_point.run(engine, circuit, abort)?;
                (physical, Some(seed))
            }
            QpssInitialState::Zero => {
                let (physical, _) = self.operating_point.physical_circuit(circuit, abort)?;
                (physical, None)
            }
        };
        let mut physical = super::pss::circuit_with_options(&physical, numeric_options, abort)?;
        // OP's resolved temperature defines this physical run point, even if
        // the analysis has its own independent numerical-options overlay.
        physical.options.temp = Some(self.operating_point.config.temperature_celsius);
        let data = super::super::spec::run_abort_aware_service(abort, || {
            services::run_qpss_analysis_with_dc_seed_on_materialized_with_abort(
                &physical,
                config,
                seed.as_ref(),
                abort,
            )
        })?;
        let result = SimulationResult::from_qpss_operating_point(data.operating_point)
            .map_err(SimulationError::InvalidConfig)?;
        Ok((physical, result))
    }
}
