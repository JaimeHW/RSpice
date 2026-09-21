use super::*;
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::results::SimulationResult;
use crate::simulation::runner::{
    SpecExecutionOptions,
    monte_carlo_checkpoint::{MonteCarloCheckpointInput, MonteCarloCheckpointRequest},
    worker_contract::WorkerSpecExecutionOptions,
};
use rspice_core::{NoAbort, ResourceLimits};

fn execute(
    source: &str,
    variation_source: McVariationSource,
    options: SpecExecutionOptions,
    environment: Option<AnalysisExecutionEnvironment>,
) -> (SimulationResult, Vec<Vec<u8>>) {
    let publications = Mutex::new(Vec::new());
    let result = spec::run_spec_request_with_environment_and_checkpoint_observer(
        &EngineBridge::new(),
        AnalysisSpec::MonteCarlo {
            variation_source,
            params: vec![],
        },
        options,
        source,
        None,
        &Default::default(),
        environment,
        &NoAbort,
        Some(&|bytes| {
            publications.lock().unwrap().push(bytes.to_vec());
            Ok(())
        }),
    )
    .unwrap();
    (result, publications.into_inner().unwrap())
}

#[test]
fn all_node_monte_carlo_checkpoint_matches_legacy_and_resumes_with_new_histograms() {
    let deck = "Voltage continuation\n.param r=1k\nV1 in 0 1\nR1 in out {r}\nR2 out 0 1k\nV2 2 0 2\nR3 2 0 1k\na_adc [out] [digital] adc\n.model adc adc_bridge(in_low=0.4 in_high=0.6)\n.mc 4 uniform 0.2 seed 37 START=2\n.end\n";
    let environment = AnalysisExecutionEnvironment {
        temperature_celsius: 75.0,
        supply_voltage: Some(1.8),
        nominal_supply_voltage: Some(1.0),
        supply_source_names: vec!["V1".into()],
    };
    for variation_source in McVariationSource::ALL {
        let source = if variation_source == McVariationSource::DeckStatistics {
            deck.replace("r=1k", "r={unif(1k,0.2)}")
        } else {
            deck.into()
        };
        let mut options = SpecExecutionOptions {
            mc_histogram_bins: Some(5),
            mc_checkpoint: Some(MonteCarloCheckpointRequest {
                publish_every: 1.try_into().unwrap(),
                trial_range: Some(2..4),
                resume: None,
            }),
            ..Default::default()
        };
        // Exercise the same serialized options used by a browser worker.
        let wire = WorkerSpecExecutionOptions::from(&options);
        let restored: WorkerSpecExecutionOptions =
            serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
        assert_eq!(wire, restored);
        let (_, publications) = execute(
            &source,
            variation_source,
            restored.into(),
            Some(environment.clone()),
        );
        assert_eq!(publications.len(), 2);
        let bytes = publications.last().unwrap().clone();
        let checkpoint = StudyMonteCarloCheckpoint::from_bytes_with_limits(
            &bytes,
            ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
        assert_eq!(checkpoint.completed_indices().collect::<Vec<_>>(), [2, 3]);
        options.mc_histogram_bins = Some(7);
        options.mc_checkpoint.as_mut().unwrap().trial_range = None;
        options.mc_checkpoint.as_mut().unwrap().resume =
            Some(MonteCarloCheckpointInput::from_bytes(bytes).unwrap());
        let source = source.replace(
            "START=2",
            "START=2 CONFIDENCE=80 CI=BOOTSTRAP RESAMPLES=32 BOOTSEED=77",
        );
        assert_eq!(
            super::super::prepared_population_identity(
                None,
                7,
                variation_source,
                None,
                &source,
                Some(environment.clone())
            )
            .unwrap(),
            checkpoint.population_identity()
        );
        let (resumed, publications) = execute(
            &source,
            variation_source,
            options.clone(),
            Some(environment.clone()),
        );
        assert_eq!(publications.len(), 2, "only the two missing trials solve");
        options.mc_checkpoint = None;
        let (fresh, publications) = execute(
            &source,
            variation_source,
            options,
            Some(environment.clone()),
        );
        assert!(publications.is_empty());
        let (
            SimulationResult::MonteCarlo {
                variables,
                member_measurements,
                ..
            },
            SimulationResult::MonteCarlo {
                variables: expected,
                member_measurements: fresh_members,
                ..
            },
        ) = (resumed, fresh)
        else {
            panic!("MC results")
        };
        assert_eq!(member_measurements, fresh_members);
        let legacy = match variation_source {
            McVariationSource::ParameterTolerance => {
                services::run_monte_carlo_analysis_with_environment_and_source_path_and_abort(
                    &source,
                    None,
                    Some(75.0),
                    Some(1.8),
                    Some(1.0),
                    &["V1".into()],
                    &NoAbort,
                )
            }
            McVariationSource::DeckStatistics => {
                services::run_statistical_monte_carlo_with_environment_and_source_path_and_abort(
                    &source,
                    None,
                    Some(75.0),
                    Some(1.8),
                    Some(1.0),
                    &["V1".into()],
                    &NoAbort,
                )
            }
        }
        .unwrap();
        assert_eq!(member_measurements, legacy.trial_measurements);
        assert_eq!(variables.len(), legacy.variables.len());
        assert!(!variables.iter().any(|value| value.name == "V(DIGITAL)"));
        assert_eq!(
            variables
                .iter()
                .find(|value| value.name == "V(2)")
                .unwrap()
                .samples,
            vec![2.0; 4]
        );
        for ((actual, fresh), legacy) in variables.iter().zip(&expected).zip(&legacy.variables) {
            assert_eq!(actual.name, legacy.name);
            assert_eq!(actual.samples, legacy.samples);
            assert_eq!(actual.samples, fresh.samples);
            assert_eq!(actual.histogram, fresh.histogram);
            if actual.name == "V(OUT)" {
                assert_eq!(actual.histogram.len(), 7);
            }
            assert_eq!(actual.mean_confidence, fresh.mean_confidence);
            assert_eq!(actual.mean_confidence, legacy.mean_confidence);
        }
    }
}

#[test]
fn all_node_monte_carlo_roster_does_not_require_a_nominal_operating_point() {
    // x=x^2+0.3 has no real solution. Elaboration still defines its voltage
    // roster, so drawing parameters can proceed without solving this circuit.
    let source = "Unsolvable nominal\n.param offset=0.3\nB1 out 0 V=V(out)^2+{offset}\nR1 out 0 1k\n.mc 2 uniform 1 seed 7 params offset\n.end\n";
    let prepared = prepare(
        source,
        None,
        McVariationSource::ParameterTolerance,
        5,
        None,
        &NoAbort,
    )
    .unwrap();
    assert!(prepared.study.measurements.contains(&"V(OUT)".into()));
    let mut changed = rspice_core::SimulationResult::new(1, 0);
    changed.node_names = vec!["0".into(), "different".into()];
    changed.node_voltages = vec![0.0, 1.0];
    assert!(matches!(
        prepared.basis.observe(changed),
        Err(SimulationError::InvalidConfig(_))
    ));
    let mut draft = crate::simulation::dialog::McDialogState::from_config(
        &crate::simulation::dialog::mc::McConfig::default(),
    );
    draft.checkpoint.retain_trials = true;
    draft.histogram_bins = "7".into();
    let config = draft.to_config().unwrap();
    assert!(config.base_analysis.is_none());
    assert!(config.checkpoint.is_some());
    assert_eq!(config.histogram_bins, 7);
    draft.histogram_bins = "0".into();
    assert!(draft.to_config().is_err());
}
