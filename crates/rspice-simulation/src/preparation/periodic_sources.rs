//! Validate periodic-source contracts against the prepared executable netlist.

use super::{PreparationError, PreparationStage};
use rspice_app_types::product::AnalysisInstanceId;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;

pub fn validate_prepared_periodic_sources<'a>(
    tasks: impl Iterator<Item = (AnalysisInstanceId, &'a AnalysisSpec)> + Clone,
    executable_netlist: &str,
) -> Result<(), PreparationError> {
    if !tasks
        .clone()
        .any(|(_, spec)| matches!(spec, AnalysisSpec::Pss { .. }))
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

    for (instance_id, spec) in tasks {
        let AnalysisSpec::Pss {
            fundamental_freq,
            tone_sources,
            points_per_period,
            num_harmonics,
            oscillator_mode,
            ..
        } = spec
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
                        "PSS instance {instance_id} does not match the prepared circuit: {error}"
                    ),
                )
            })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_simulation_contract::analysis_spec_values::PssMethod;

    fn prepared_pss_spec(
        tone_sources: impl IntoIterator<Item = &'static str>,
        oscillator_mode: bool,
    ) -> AnalysisSpec {
        AnalysisSpec::Pss {
            method: PssMethod::Shooting,
            fundamental_freq: 1.0e3,
            tone_sources: tone_sources.into_iter().map(str::to_owned).collect(),
            tstab_periods: 20,
            points_per_period: 512,
            tolerance: 1.0e-7,
            oscillator_mode,
            oscillator_node: oscillator_mode.then(|| "out".to_owned()),
            num_harmonics: 20,
            integration_method: None,
            tstab: 0.0,
            max_iterations: 100,
            abstol: 1.0e-12,
            damping: 1.0,
            max_period_change: 0.1,
            verbose: false,
        }
    }

    #[test]
    fn prepared_pss_authenticates_the_complete_executable_source_set() {
        let deck = "periodic sources\nVLO lo 0 SIN(0 1 1k)\nVCLK clk 0 PULSE(0 1 0 1u 1u 200u 500u)\nR1 lo 0 1k\nR2 clk 0 1k\n.end\n";
        let complete = prepared_pss_spec(["vclk", "VLO"], false);
        validate_prepared_periodic_sources(
            std::iter::once((AnalysisInstanceId::new(), &complete)),
            deck,
        )
        .expect("the complete commensurate source set is accepted");

        let incomplete = prepared_pss_spec(["VLO"], false);
        let error = validate_prepared_periodic_sources(
            std::iter::once((AnalysisInstanceId::new(), &incomplete)),
            deck,
        )
        .expect_err("an omitted periodic source fails preflight");
        assert_eq!(error.stage(), PreparationStage::AnalysisPlan);
        assert!(error.message().contains("omitted: VCLK"));
    }

    #[test]
    fn prepared_autonomous_pss_accepts_an_exact_empty_driven_source_set() {
        let deck = "autonomous oscillator\nR1 out 0 1k\nC1 out 0 1n\n.end\n";
        let autonomous = prepared_pss_spec([], true);
        validate_prepared_periodic_sources(
            std::iter::once((AnalysisInstanceId::new(), &autonomous)),
            deck,
        )
        .expect("a source-free autonomous circuit has an exact empty source set");
    }
}
