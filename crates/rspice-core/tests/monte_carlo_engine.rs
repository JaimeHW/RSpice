use rspice_core::analysis::{Distribution, MonteCarloConfig, MonteCarloRunner, Tolerance};
use rspice_core::{Engine, Netlist, SimulationConfig};

const PARAMETRIC_DIVIDER: &str = "\
Monte Carlo divider
.param rval=1k
V1 in 0 1
R1 in out {rval}
R2 out 0 1k
.end
";

#[test]
fn monte_carlo_is_reproducible_and_preserves_run_order() {
    let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("fixture parses");
    let filter = ["RVAL".to_string()];
    let engine = Engine::default();

    let first = engine
        .run_monte_carlo_with_options(
            &netlist,
            32,
            0x5eed,
            Distribution::Gaussian { sigma: 0.1 },
            Some(&filter),
        )
        .expect("first run completes");
    let second = engine
        .run_monte_carlo_with_options(
            &netlist,
            32,
            0x5eed,
            Distribution::Gaussian { sigma: 0.1 },
            Some(&filter),
        )
        .expect("second run completes");

    assert_eq!(first.num_runs, 32);
    assert!(first.all_converged);
    assert_eq!(first.num_failures, 0);
    let first_samples = &first.variables["V(OUT)"].samples;
    let second_samples = &second.variables["V(OUT)"].samples;
    assert_eq!(first_samples, second_samples);
    assert!(first_samples.windows(2).any(|pair| pair[0] != pair[1]));
}

#[test]
fn monte_carlo_with_zero_spread_matches_the_nominal_solution() {
    let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("fixture parses");
    let filter = ["RVAL".to_string()];
    let result = Engine::default()
        .run_monte_carlo_with_options(
            &netlist,
            8,
            42,
            Distribution::Uniform { tolerance: 0.0 },
            Some(&filter),
        )
        .expect("analysis completes");

    let samples = &result.variables["V(OUT)"].samples;
    assert_eq!(samples.len(), 8);
    assert!(samples.iter().all(|value| (*value - 0.5).abs() < 1.0e-12));
}

#[test]
fn authored_numeric_node_names_take_precedence_over_internal_indices() {
    let netlist = Netlist::parse("numeric nodes\nVfirst 2 0 2\nVsecond out 0 5\n.end\n")
        .expect("fixture parses");
    let result = Engine::default()
        .run_monte_carlo_with_options(
            &netlist,
            3,
            42,
            Distribution::Uniform { tolerance: 0.0 },
            None,
        )
        .expect("analysis completes");
    assert_eq!(result.variables["V(2)"].samples, vec![2.0; 3]);
    assert_eq!(result.variables["V(OUT)"].samples, vec![5.0; 3]);
    assert_eq!(result.variables["V(1)"].samples, vec![2.0; 3]);
}

#[test]
fn aggregation_counts_attempts_and_preserves_all_failed_evidence() {
    let engine = Engine::default();
    let trial = (vec![0.0, 2.0], vec!["0".to_owned(), "OUT".to_owned()]);
    let result = engine
        .monte_carlo_result_from_trials([trial.clone()], 4)
        .unwrap();
    assert_eq!((result.num_runs, result.num_failures), (4, 3));
    assert!(!result.all_converged);
    assert_eq!(result.variables["V(OUT)"].samples.len(), 1);
    let failed = engine.monte_carlo_result_from_trials([], 4).unwrap();
    assert_eq!((failed.num_runs, failed.num_failures), (4, 4));
    assert!(!failed.all_converged);
    assert!(failed.variables.is_empty());
    assert!(
        engine
            .monte_carlo_result_from_trials([trial.clone(), trial], 1)
            .is_err()
    );
}

#[test]
fn aggregation_checks_the_batch_limit_before_reading_trials() {
    let mut config = SimulationConfig::default();
    config.resource_limits.max_batch_runs = 2;
    let trials = std::iter::from_fn(|| panic!("over-limit input must not be read"));
    let error = Engine::new(config)
        .monte_carlo_result_from_trials(trials, usize::MAX)
        .unwrap_err();
    assert!(matches!(
        error,
        rspice_core::engine::SimulationError::ResourceLimit(_)
    ));
}

#[test]
fn callback_sampling_is_independent_of_component_registration_order() {
    let sample = |reverse: bool| {
        let mut runner = MonteCarloRunner::new(MonteCarloConfig::new(8).with_seed(42));
        for index in 0..8 {
            let index = if reverse { 7 - index } else { index };
            runner.add_component(
                &format!("R{index}"),
                1000.0,
                Tolerance {
                    lot: Some(Distribution::Uniform { tolerance: 0.02 }),
                    dev: Some(Distribution::Gaussian { sigma: 0.01 }),
                },
            );
        }
        runner.run(|variation| Ok::<_, ()>(variation.values.clone()))
    };
    let forward = sample(false);
    let reverse = sample(true);
    for (name, statistics) in forward.variables {
        assert_eq!(
            statistics.samples, reverse.variables[&name].samples,
            "{name}"
        );
    }
}
