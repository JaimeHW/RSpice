use rspice_core::analysis::monte_carlo::{MeanConfidenceInterval, MeanConfidenceMethod};
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
        runner
            .run(|variation| Ok::<_, ()>(variation.values.clone()))
            .unwrap()
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

#[test]
fn callback_host_seed_can_replay_the_exact_samples() {
    let run = |config| {
        let mut runner = MonteCarloRunner::new(config);
        runner.add_component("R1", -1000.0, Tolerance::uniform(5.0));
        runner
            .run(|variation| Ok::<_, ()>(variation.values.clone()))
            .unwrap()
    };
    let original = run(MonteCarloConfig::new(12));
    let sampling = original.sampling.expect("host seed is retained");
    let replay = run(MonteCarloConfig::new(12).with_seed(sampling.seed));
    assert_eq!(replay.sampling, original.sampling);
    assert_eq!(
        replay.variables["R1"].samples,
        original.variables["R1"].samples
    );
}

#[test]
fn callback_configuration_is_rejected_before_simulation() {
    for case in 0..6 {
        let mut config = MonteCarloConfig::new(3).with_seed(1);
        match case {
            0 => config.num_runs = 0,
            1 => config.histogram_bins = 0,
            2 => config.histogram_bins = usize::MAX,
            3 => config.confidence_pct = f64::NAN,
            4 => config.resource_limits.max_batch_runs = 2,
            _ => config.resource_limits.max_result_values = 2,
        }
        let runner = MonteCarloRunner::new(config);
        assert!(
            runner
                .run::<_, ()>(|_| panic!("invalid configuration invoked callback"))
                .is_err()
        );
    }
    for (nominal, spread) in [(f64::NAN, 0.1), (1.0, -0.1), (1.0, f64::INFINITY)] {
        let mut runner = MonteCarloRunner::new(MonteCarloConfig::new(3).with_seed(1));
        runner.add_component(
            "R1",
            nominal,
            Tolerance {
                lot: None,
                dev: Some(Distribution::Uniform { tolerance: spread }),
            },
        );
        assert!(
            runner
                .run::<_, ()>(|_| panic!("invalid component invoked callback"))
                .is_err()
        );
    }
}

#[test]
fn callback_result_budget_accounts_for_samples_and_histograms() {
    let mut config = MonteCarloConfig::new(4).with_seed(1);
    config.histogram_bins = 1;
    config.resource_limits.max_result_values = 12;
    let runner = MonteCarloRunner::new(config);
    let mut calls = 0;
    let error = runner
        .run::<_, ()>(|_| {
            calls += 1;
            Ok(std::collections::HashMap::from([(
                "out".to_owned(),
                calls as f64,
            )]))
        })
        .unwrap_err();
    assert_eq!(calls, 3);
    assert!(matches!(
        error,
        rspice_core::SimulationError::ResourceLimit(_)
    ));
}

#[test]
fn callback_cancellation_preserves_the_deadline_reason() {
    struct Deadline(std::sync::atomic::AtomicBool);
    impl rspice_core::AbortSignal for Deadline {
        fn is_aborted(&self) -> bool {
            self.0.load(std::sync::atomic::Ordering::Relaxed)
        }
        fn abort_reason(&self) -> rspice_core::AbortReason {
            rspice_core::AbortReason::TimeLimit
        }
    }
    let deadline = Deadline(std::sync::atomic::AtomicBool::new(false));
    let runner = MonteCarloRunner::new(MonteCarloConfig::new(2).with_seed(1));
    let error = runner
        .run_with_abort::<_, ()>(
            |_| {
                deadline.0.store(true, std::sync::atomic::Ordering::Relaxed);
                Err(())
            },
            &deadline,
        )
        .unwrap_err();
    assert!(matches!(
        error,
        rspice_core::SimulationError::TimeLimitExceeded
    ));
    let netlist = Netlist::parse(PARAMETRIC_DIVIDER).unwrap();
    let error = Engine::default()
        .run_monte_carlo_with_options_and_abort(
            &netlist,
            2,
            1,
            Distribution::uniform(0.1),
            None,
            &deadline,
        )
        .unwrap_err();
    assert!(matches!(
        error,
        rspice_core::SimulationError::TimeLimitExceeded
    ));
}

#[test]
fn confidence_level_changes_the_student_t_mean_interval() {
    let run = |level| {
        let mut config = MonteCarloConfig::new(5).with_seed(1);
        config.confidence_pct = level;
        let mut value = 0;
        MonteCarloRunner::new(config)
            .run::<_, ()>(|_| {
                value += 1;
                Ok(std::collections::HashMap::from([(
                    "out".to_owned(),
                    value as f64,
                )]))
            })
            .unwrap()
    };
    let intervals = [run(80.0), run(95.0)];
    let [
        MeanConfidenceInterval::Available {
            lower: lower80,
            upper: upper80,
        },
        MeanConfidenceInterval::Available {
            lower: lower95,
            upper: upper95,
        },
    ] = intervals
        .each_ref()
        .map(|result| result.variables["out"].mean_confidence.unwrap())
    else {
        panic!("five finite samples must have mean confidence limits")
    };
    // df=4, Student-t 0.975 quantile = 2.7764451051977987.
    assert!((lower95 - 1.036756838522439).abs() < 1e-11);
    assert!((upper95 - 4.963243161477561).abs() < 1e-11);
    assert!(lower95 < lower80 && upper95 > upper80);
    assert_eq!(intervals[1].confidence.unwrap().successful_samples, 5);
    assert!(
        !intervals[1]
            .confidence
            .unwrap()
            .conditional_on_successful_trials
    );
}

#[test]
fn failed_trials_qualify_mean_confidence_and_insufficient_samples_stay_unavailable() {
    let mut attempted = 0;
    let result = MonteCarloRunner::new(MonteCarloConfig::new(3).with_seed(1))
        .run(|_| {
            attempted += 1;
            if attempted == 1 {
                Ok(std::collections::HashMap::from([("out".to_owned(), 2.0)]))
            } else {
                Err(())
            }
        })
        .unwrap();
    let confidence = result.confidence.unwrap();
    assert_eq!(confidence.successful_samples, 1);
    assert!(confidence.conditional_on_successful_trials);
    assert_eq!(
        result.variables["out"].mean_confidence,
        Some(MeanConfidenceInterval::InsufficientSamples)
    );
}

#[test]
fn bootstrap_is_repeatable_and_separate_from_circuit_sampling() {
    let run = |method| {
        let mut config = MonteCarloConfig::new(2).with_seed(1);
        config.confidence_method = method;
        let mut runner = MonteCarloRunner::new(config);
        runner.add_component("R1", 1000.0, Tolerance::uniform(5.0));
        let mut draws = Vec::new();
        let result = runner
            .run::<_, ()>(|variation| {
                draws.push(variation.get("R1", 0.0));
                Ok(std::collections::HashMap::from([(
                    "out".to_owned(),
                    if draws.len() == 1 { 0.0 } else { 2.0 },
                )]))
            })
            .unwrap();
        (result, draws)
    };
    let method = MeanConfidenceMethod::PercentileBootstrap {
        resamples: 256,
        seed: 42,
    };
    let (first, first_draws) = run(method);
    let (second, second_draws) = run(method);
    let (_, student_draws) = run(MeanConfidenceMethod::StudentT);
    assert_eq!(first_draws, second_draws);
    assert_eq!(first_draws, student_draws);
    assert_eq!(
        first.variables["out"].mean_confidence,
        second.variables["out"].mean_confidence
    );
    // For [0,2], the exact empirical two-draw mean distribution is
    // {0: 1/4, 1: 1/2, 2: 1/4}; its central 95% interval is [0,2].
    assert_eq!(
        first.variables["out"].mean_confidence,
        Some(MeanConfidenceInterval::Available {
            lower: 0.0,
            upper: 2.0
        })
    );
}

#[test]
fn bootstrap_work_is_bounded_before_callback_execution() {
    let mut config = MonteCarloConfig::new(1000).with_seed(1);
    config.confidence_method = MeanConfidenceMethod::PercentileBootstrap {
        resamples: 10_000,
        seed: 1,
    };
    let error = MonteCarloRunner::new(config)
        .run::<_, ()>(|_| {
            panic!("an excessive bootstrap workload must be refused before simulation")
        })
        .unwrap_err();
    assert!(matches!(
        error,
        rspice_core::SimulationError::ResourceLimit(_)
    ));
}

#[test]
fn interrupted_confidence_recalculation_keeps_the_previous_estimate() {
    let mut value = 0;
    let mut result = MonteCarloRunner::new(MonteCarloConfig::new(4).with_seed(1))
        .run::<_, ()>(|_| {
            value += 1;
            Ok(std::collections::HashMap::from([(
                "out".to_owned(),
                value as f64,
            )]))
        })
        .unwrap();
    let previous_method = result.confidence;
    let previous_interval = result.variables["out"].mean_confidence;
    let error = result
        .compute_mean_confidence(
            99.0,
            MeanConfidenceMethod::PercentileBootstrap {
                resamples: 1000,
                seed: 2,
            },
            rspice_core::resource::ResourceLimits::default(),
            &rspice_core::abort_signal::CountingAbort::new(30),
        )
        .unwrap_err();
    assert!(matches!(error, rspice_core::SimulationError::Aborted));
    assert_eq!(result.confidence, previous_method);
    assert_eq!(result.variables["out"].mean_confidence, previous_interval);
}

#[test]
fn confidence_reports_unrepresentable_student_t_limits_without_losing_bootstrap_limits() {
    let mut index = 0;
    let mut result = MonteCarloRunner::new(MonteCarloConfig::new(2).with_seed(1))
        .run::<_, ()>(|_| {
            index += 1;
            Ok(std::collections::HashMap::from([
                (
                    "out".to_owned(),
                    if index == 1 { -f64::MAX } else { f64::MAX },
                ),
                ("constant".to_owned(), f64::MAX),
            ]))
        })
        .unwrap();
    assert_eq!(
        result.variables["out"].mean_confidence,
        Some(MeanConfidenceInterval::Unrepresentable)
    );
    result
        .compute_mean_confidence(
            95.0,
            MeanConfidenceMethod::PercentileBootstrap {
                resamples: 256,
                seed: 42,
            },
            rspice_core::resource::ResourceLimits::default(),
            &rspice_core::NoAbort,
        )
        .unwrap();
    assert_eq!(
        result.variables["out"].mean_confidence,
        Some(MeanConfidenceInterval::Available {
            lower: -f64::MAX,
            upper: f64::MAX
        })
    );
    assert_eq!(
        result.variables["constant"].mean_confidence,
        Some(MeanConfidenceInterval::Available {
            lower: f64::MAX,
            upper: f64::MAX
        })
    );
}
