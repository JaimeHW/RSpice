//! Configured QPSS execution on each materialized study circuit.
use super::*;
use crate::results::SimulationResult;
use rspice_core::engine::QpssInitialState;

pub(super) fn run_with_circuit(
    request_config: &StudyQpssConfig,
    engine: &rspice_core::Engine,
    circuit: &rspice_core::Netlist,
    numeric_options: &str,
    abort: &dyn AbortSignal,
) -> Result<(rspice_core::Netlist, SimulationResult), SimulationError> {
    request_config
        .validate()
        .map_err(SimulationError::InvalidConfig)?;
    let config = request_config
        .request
        .qpss_config()
        .map_err(SimulationError::InvalidConfig)?;
    let (physical, seed) = match config.initial_state {
        QpssInitialState::DcOperatingPoint => {
            let (physical, seed) = super::pss::run_operating_point(
                &request_config.operating_point,
                engine,
                circuit,
                abort,
            )?;
            (physical, Some(seed))
        }
        QpssInitialState::Zero => {
            let (physical, _) =
                super::pss::physical_circuit(&request_config.operating_point, circuit, abort)?;
            (physical, None)
        }
    };
    let mut physical = super::pss::circuit_with_options(&physical, numeric_options, abort)?;
    // OP's resolved temperature defines this physical run point, even if
    // the analysis has its own independent numerical-options overlay.
    physical.options.temp = Some(request_config.operating_point.config.temperature_celsius);
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
