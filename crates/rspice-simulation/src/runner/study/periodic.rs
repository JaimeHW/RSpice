//! Periodic consumer execution on a freshly solved trial.
use super::*;
use crate::results::SimulationResult;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;

pub(super) fn run_periodic(
    request_config: &StudyPostprocess,
    engine: &rspice_core::Engine,
    analysis: &StudyAnalysis,
    circuit: &rspice_core::Netlist,
    numeric_options: &str,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let context = service_context(engine, abort);
    request_config.periodic_execution_options()?;
    let (physical, result) = match analysis {
        StudyAnalysis::Pss(pss) => super::pss::run_with_circuit(
            pss,
            engine,
            circuit,
            &request_config.producer_numeric_options,
            abort,
        )?,
        StudyAnalysis::Qpss(qpss) => super::qpss::run_with_circuit(
            qpss,
            engine,
            circuit,
            &request_config.producer_numeric_options,
            abort,
        )?,
        StudyAnalysis::Hb(hb) => super::hb::run_with_circuit(
            hb,
            engine,
            circuit,
            &request_config.producer_numeric_options,
            abort,
        )?,
        StudyAnalysis::Native(
            producer @ (AnalysisSpec::HarmonicBalance { .. } | AnalysisSpec::Qpss { .. }),
        ) => {
            let physical = super::pss::circuit_with_options(
                circuit,
                &request_config.producer_numeric_options,
                context,
            )?;
            let result = super::super::spec::run_native_study_on_materialized(
                producer.clone(),
                &physical,
                context,
            )?;
            (physical, result)
        }
        _ => {
            return Err(SimulationError::InvalidConfig(
                "Periodic study requires its configured PSS, HB or QPSS producer".into(),
            ));
        }
    };
    let consumer = super::pss::circuit_with_options(&physical, numeric_options, context)?;
    if let SimulationResult::Qpss {
        operating_point, ..
    } = &result
    {
        return super::super::spec::run_qp_study_consumer(
            request_config.request.clone(),
            &consumer,
            operating_point,
            context,
        );
    }
    let carrier = match &result {
        SimulationResult::Transient {
            periodic_state: Some(point),
            ..
        } => services::PeriodicCarrierState::Shooting(point),
        SimulationResult::HarmonicBalance {
            operating_point: point,
            ..
        } => services::PeriodicCarrierState::HarmonicBalance(point),
        _ => {
            return Err(SimulationError::SolverError(
                "Study producer returned no retained periodic state".into(),
            ));
        }
    };
    super::super::spec::run_periodic_study_consumer(
        request_config.request.clone(),
        request_config.periodic_options.as_ref(),
        &consumer,
        carrier,
        context,
    )
}

#[cfg(test)]
mod tests;
