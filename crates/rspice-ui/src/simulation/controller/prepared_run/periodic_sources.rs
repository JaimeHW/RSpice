//! Validate periodic-source contracts against the prepared executable netlist.

use crate::simulation::execution::{PreparationError, PreparationStage, PreparedTask};
use crate::simulation::multi_run::AnalysisSpec;
pub(super) fn validate_prepared_periodic_sources(
    tasks: &[PreparedTask],
    executable_netlist: &str,
) -> Result<(), PreparationError> {
    if !tasks
        .iter()
        .any(|task| matches!(task.queued_analysis().spec, AnalysisSpec::Pss { .. }))
    {
        return Ok(());
    }

    let parsed = rspice_core::Netlist::parse(executable_netlist).map_err(|error| {
        PreparationError::new(
            PreparationStage::Netlist,
            format!("Could not authenticate periodic sources in the executable netlist: {error}"),
        )
    })?;
    let engine = rspice_core::Engine::new(rspice_core::SimulationConfig::default());

    for task in tasks {
        let AnalysisSpec::Pss {
            fundamental_freq,
            tone_sources,
            points_per_period,
            num_harmonics,
            oscillator_mode,
            ..
        } = &task.queued_analysis().spec
        else {
            continue;
        };
        if *oscillator_mode {
            continue;
        }
        engine
            .validate_pss_source_contract_with_abort(
                &parsed,
                tone_sources,
                &rspice_core::analysis::PssConfig::new(*fundamental_freq)
                    .with_points_per_period(*points_per_period)
                    .with_harmonics((*num_harmonics).max(1)),
                &rspice_core::abort_signal::NoAbort,
            )
            .map_err(|error| {
                PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!(
                        "PSS instance {} does not match the prepared circuit: {error}",
                        task.instance_id()
                    ),
                )
            })?;
    }
    Ok(())
}
