use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::monte_carlo::{Distribution, MeanConfidenceMethod};
use rspice_core::engine::{MonteCarloEnvironment, MonteCarloStudyConfig};
use rspice_core::{Engine, Netlist, SimulationConfig, SimulationError};

fn config(workers: usize) -> SimulationConfig {
    let mut config = SimulationConfig::default();
    config.resource_limits.max_parallel_workers = workers;
    config
}

#[test]
fn configured_dc_and_ac_measurements_reuse_the_trial_circuit_and_environment() {
    let netlist = Netlist::parse("RC study\n.param rval=1k\nV1 in 0 DC 1 AC 1\nR1 in out {rval}\nR2 out 0 1k\nC1 out 0 1u\n.end\n").unwrap();
    let mut study = MonteCarloStudyConfig::new(6, 42, vec!["bias".into(), "gain_1k".into()]);
    study.distribution = Distribution::Uniform { tolerance: 0.2 };
    study.parameter_filter = vec!["rval".into()];
    study.histogram_bins = 3;
    study.confidence_pct = 90.0;
    study.environment = Some(MonteCarloEnvironment {
        temperature_celsius: 75.0,
        supply_voltage: Some(1.8),
        nominal_supply_voltage: Some(1.0),
        supply_source_names: vec!["V1".into()],
    });
    let mut reference = None;
    for workers in [1, 2] {
        let engine = Engine::new(config(workers));
        let measured = engine
            .run_monte_carlo_measurements_with_abort(
                &netlist,
                &study,
                &NoAbort,
                |engine, trial, _, abort| {
                    let available = std::thread::available_parallelism().map_or(1, usize::from);
                    let expected_workers = if available > 1 { 1 } else { workers };
                    assert_eq!(
                        engine.config().resource_limits.max_parallel_workers,
                        expected_workers
                    );
                    assert_eq!(trial.options.temp, Some(75.0));
                    let r = trial.params.get("rval").unwrap();
                    let dc = engine.run_dc_op_with_abort(trial, abort)?;
                    let node = dc
                        .node_names
                        .iter()
                        .position(|name| name.eq_ignore_ascii_case("out"))
                        .unwrap();
                    let ac = engine.run_ac_with_abort(trial, &[1000.0], abort)?;
                    let ac_node = ac[0]
                        .node_names
                        .iter()
                        .position(|name| name.eq_ignore_ascii_case("out"))
                        .unwrap();
                    let gain = ac[0].voltages[ac_node].norm();
                    let expected = 1000.0 / (r + 1000.0).hypot(std::f64::consts::TAU * r);
                    assert!((gain - expected).abs() < 1e-12, "{gain} != {expected}");
                    assert!((dc.node_voltages[node] - 1.8 * 1000.0 / (r + 1000.0)).abs() < 1e-12);
                    Ok(vec![dc.node_voltages[node], gain])
                },
            )
            .unwrap();
        let legacy = engine
            .run_monte_carlo_with_options_environment_and_abort(
                &netlist,
                study.num_runs,
                study.seed,
                study.distribution,
                Some(&study.parameter_filter),
                study.environment.clone(),
                &NoAbort,
            )
            .unwrap();
        assert_eq!(
            measured.variables["bias"].samples,
            legacy.variables["V(OUT)"].samples
        );
        assert_eq!(measured.sampling, legacy.sampling);
        assert_eq!(measured.successful_trial_indices, Some((0..6).collect()));
        assert_eq!(measured.confidence.unwrap().level_pct, 90.0);
        assert_eq!(measured.variables["gain_1k"].histogram.len(), 3);
        assert_eq!(
            measured.variables.len(),
            2,
            "measurements do not acquire voltage aliases"
        );
        let samples = measured.variables["gain_1k"].samples.clone();
        assert!(samples.windows(2).any(|pair| pair[0] != pair[1]));
        if let Some(reference) = &reference {
            assert_eq!(&samples, reference);
        } else {
            reference = Some(samples);
        }
    }
}

#[test]
fn missing_nonfinite_and_failed_measurements_share_original_trial_indices() {
    let netlist = Netlist::parse("study\nV1 in 0 1\n.end\n").unwrap();
    let study = MonteCarloStudyConfig::new(6, 42, vec!["delay".into(), "power".into()]);
    for workers in [1, 2] {
        let engine = Engine::new(config(workers));
        let result = engine
            .run_monte_carlo_measurements_with_abort(
                &netlist,
                &study,
                &NoAbort,
                |_, _, index, _| match index {
                    1 => Ok(vec![1.0]),
                    2 => Ok(vec![2.0, f64::NAN]),
                    3 => Err(SimulationError::Circuit("analysis did not converge".into())),
                    _ => Ok(vec![index as f64, index as f64 * 2.0]),
                },
            )
            .unwrap();
        assert_eq!(result.successful_trial_indices, Some(vec![0, 4, 5]));
        assert_eq!(
            (result.num_runs, result.num_failures, result.all_converged),
            (6, 3, false)
        );
        assert_eq!(result.variables["delay"].samples, vec![0.0, 4.0, 5.0]);
        assert_eq!(result.variables["power"].samples, vec![0.0, 8.0, 10.0]);
        assert!(result.confidence.unwrap().conditional_on_successful_trials);
        let failed = engine
            .run_monte_carlo_measurements_with_abort(&netlist, &study, &NoAbort, |_, _, _, _| {
                Ok(vec![])
            })
            .unwrap();
        assert_eq!(failed.num_failures, 6);
        assert_eq!(failed.successful_trial_indices, Some(vec![]));
        assert!(failed.variables.is_empty());
    }
}

#[test]
fn invalid_measurement_configuration_and_budgets_stop_before_evaluation() {
    let netlist = Netlist::parse("study\nV1 in 0 1\n.end\n").unwrap();
    for case in 0..9 {
        let mut study = MonteCarloStudyConfig::new(3, 42, vec!["gain".into()]);
        let mut engine_config = config(1);
        match case {
            0 => study.measurements.clear(),
            1 => study.measurements.push("GAIN".into()),
            2 => study.histogram_bins = 0,
            3 => study.confidence_pct = 100.0,
            4 => engine_config.resource_limits.max_result_values = 20,
            5 => study.num_runs = 0,
            6 => study.measurements[0] = "gain\n".into(),
            7 => {
                study.confidence_method = MeanConfidenceMethod::PercentileBootstrap {
                    resamples: 1,
                    seed: 5,
                }
            }
            8 => {
                study.confidence_method = MeanConfidenceMethod::PercentileBootstrap {
                    resamples: 100,
                    seed: 5,
                };
                engine_config.resource_limits.max_analysis_points = 200;
            }
            _ => unreachable!(),
        }
        let error = Engine::new(engine_config)
            .run_monte_carlo_measurements_with_abort(&netlist, &study, &NoAbort, |_, _, _, _| {
                panic!("invalid case {case} executed a trial")
            })
            .unwrap_err();
        if matches!(case, 4 | 8) {
            assert!(
                matches!(error, SimulationError::ResourceLimit(_)),
                "{error}"
            );
        }
    }
}

#[test]
fn measurement_execution_preserves_fatal_errors_and_deadline_cancellation() {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    struct Deadline(AtomicBool);
    impl rspice_core::AbortSignal for Deadline {
        fn is_aborted(&self) -> bool {
            self.0.load(Ordering::Relaxed)
        }
        fn abort_reason(&self) -> rspice_core::AbortReason {
            if self.0.load(Ordering::Relaxed) {
                rspice_core::AbortReason::TimeLimit
            } else {
                rspice_core::AbortReason::Cancelled
            }
        }
    }
    let netlist = Netlist::parse("study\nV1 in 0 1\n.end\n").unwrap();
    let study = MonteCarloStudyConfig::new(6, 42, vec!["gain".into()]);
    for workers in [1, 2] {
        for case in 0..4 {
            let deadline = Deadline(AtomicBool::new(false));
            let calls = AtomicUsize::new(0);
            let error = Engine::new(config(workers))
                .run_monte_carlo_measurements_with_abort(
                    &netlist,
                    &study,
                    &deadline,
                    |_, _, _, _| {
                        calls.fetch_add(1, Ordering::Relaxed);
                        Err(match case {
                            0 => {
                                deadline.0.store(true, Ordering::Relaxed);
                                SimulationError::Circuit("interrupted analysis".into())
                            }
                            1 => SimulationError::ResourceLimit(
                                rspice_core::resource::ResourceLimitError {
                                    resource: rspice_core::resource::ResourceKind::AnalysisPoints,
                                    requested: 10,
                                    limit: 1,
                                },
                            ),
                            2 => SimulationError::Configuration(
                                rspice_core::config::SimulationConfigError::InvalidCount {
                                    field: "test.analysis_points",
                                    value: 0,
                                },
                            ),
                            3 => SimulationError::Aborted,
                            _ => unreachable!(),
                        })
                    },
                )
                .unwrap_err();
            assert!(calls.load(Ordering::Relaxed) <= workers);
            assert!(
                match case {
                    0 => matches!(error, SimulationError::TimeLimitExceeded),
                    1 => matches!(error, SimulationError::ResourceLimit(_)),
                    2 => matches!(error, SimulationError::Configuration(_)),
                    3 => matches!(error, SimulationError::Aborted),
                    _ => false,
                },
                "case {case}: {error}"
            );
        }
    }
}

#[test]
fn native_statistics_coordinate_is_shared_by_every_analysis_in_a_trial() {
    use rspice_core::netlist::SpectreStatisticalCoordinate;
    struct Fixture(std::path::PathBuf);
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let fixture = Fixture(std::env::temp_dir().join(format!(
        "rspice_mc_measurements_{}_{nonce}",
        std::process::id(),
    )));
    std::fs::create_dir(&fixture.0).unwrap();
    std::fs::write(
        fixture.0.join("process.scs"),
        "simulator lang=spectre\nstatistics {\nprocess {\nvary rtop dist=gauss std=100\n}\n}\n",
    )
    .unwrap();
    let mut netlist = Netlist::parse_with_path(
        "native statistics study\n.param rtop=1k\n.include \"process.scs\"\nV1 in 0 DC 1 AC 1\nR1 in out {rtop}\nR2 out 0 1k\n.end\n",
        &fixture.0.join("study.cir"),
    ).unwrap();
    assert!(!netlist.spectre_statistics.variations.is_empty());
    netlist.spectre_statistical_coordinate = Some(SpectreStatisticalCoordinate {
        axes: vec![("outer_step".into(), 2.0)],
        ..Default::default()
    });
    let study = MonteCarloStudyConfig::new(4, 73, vec!["dc".into(), "ac".into()]);
    let mut reference = None;
    for workers in [1, 2] {
        let result = Engine::new(config(workers))
            .run_monte_carlo_measurements_with_abort(
                &netlist,
                &study,
                &NoAbort,
                |engine, trial, index, abort| {
                    let coordinate = trial.spectre_statistical_coordinate.as_ref().unwrap();
                    assert_eq!(
                        (coordinate.seed, coordinate.monte_carlo_run),
                        (73, index as u64)
                    );
                    assert_eq!(coordinate.axes, vec![("outer_step".into(), 2.0)]);
                    let dc = engine.run_dc_op_with_abort(trial, abort)?;
                    let dc_node = dc
                        .node_names
                        .iter()
                        .position(|name| name.eq_ignore_ascii_case("out"))
                        .unwrap();
                    let ac = engine.run_ac_with_abort(trial, &[1000.0], abort)?;
                    let ac_node = ac[0]
                        .node_names
                        .iter()
                        .position(|name| name.eq_ignore_ascii_case("out"))
                        .unwrap();
                    Ok(vec![dc.node_voltages[dc_node], ac[0].voltages[ac_node].re])
                },
            )
            .unwrap();
        let samples = &result.variables["dc"].samples;
        assert!(samples.windows(2).any(|pair| pair[0] != pair[1]));
        for (dc, ac) in samples.iter().zip(&result.variables["ac"].samples) {
            assert!(
                (dc - ac).abs() < 1e-12,
                "analysis redrew the trial: {dc} != {ac}"
            );
        }
        assert_eq!(
            result.sampling.unwrap().policy,
            "spectre-coordinate-splitmix64-v1"
        );
        if let Some(reference) = &reference {
            assert_eq!(samples, reference);
        } else {
            reference = Some(samples.clone());
        }
    }
}

#[test]
fn monte_carlo_ranges_replay_original_streams_without_solving_skipped_trials() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let netlist =
        Netlist::parse("range\n.param X=1 Y=2\nV1 out 0 {X+Y}\nR1 out 0 1k\n.end\n").unwrap();
    for distribution in [
        Distribution::Gaussian { sigma: 0.1 },
        Distribution::Uniform { tolerance: 0.1 },
        Distribution::WorstCase { tolerance: 0.1 },
    ] {
        let mut study = MonteCarloStudyConfig::new(8, u64::MAX, vec!["bias".into()]);
        study.distribution = distribution;
        let run = |study: &MonteCarloStudyConfig, workers| {
            let count = AtomicUsize::new(0);
            let result = Engine::new(config(workers))
                .run_monte_carlo_measurements_with_abort(
                    &netlist,
                    study,
                    &NoAbort,
                    |engine, trial, index, abort| {
                        count.fetch_add(1, Ordering::Relaxed);
                        assert!(
                            (study.first_trial..study.first_trial + study.num_runs)
                                .contains(&index)
                        );
                        if index == 3 {
                            return Err(SimulationError::Circuit(
                                "deliberate missing observation".into(),
                            ));
                        }
                        let result = engine.run_dc_op_with_abort(trial, abort)?;
                        let node = result
                            .node_names
                            .iter()
                            .position(|name| name.eq_ignore_ascii_case("out"))
                            .unwrap();
                        Ok(vec![result.node_voltages[node]])
                    },
                )
                .unwrap();
            assert_eq!(count.load(Ordering::Relaxed), study.num_runs);
            result
        };
        let full = run(&study, 1);
        study.first_trial = 2;
        study.num_runs = 4;
        let batch = run(&study, 2);
        assert_eq!(batch.sampling.unwrap().first_trial, 2);
        assert_eq!(
            batch.successful_trial_indices.as_deref(),
            Some([2, 4, 5].as_slice())
        );
        assert_eq!((batch.num_runs, batch.num_failures), (4, 1));
        let full_indices = full.successful_trial_indices.as_ref().unwrap();
        for (sample, index) in batch.variables["bias"]
            .samples
            .iter()
            .zip(batch.successful_trial_indices.as_ref().unwrap())
        {
            let position = full_indices
                .iter()
                .position(|candidate| candidate == index)
                .unwrap();
            assert_eq!(*sample, full.variables["bias"].samples[position]);
        }
        study.first_trial = usize::MAX;
        assert!(
            Engine::default()
                .run_monte_carlo_measurements_with_abort(
                    &netlist,
                    &study,
                    &NoAbort,
                    |_, _, _, _| panic!("invalid range must not solve")
                )
                .is_err()
        );
    }
    for spelling in ["START 37", "START=37"] {
        let parsed = Netlist::parse(&format!("range\n.mc 3 {spelling} SEED 42\n.end\n")).unwrap();
        let rspice_core::netlist::AnalysisCommand::MonteCarlo(command) = &parsed.analyses[0] else {
            panic!("MC");
        };
        assert_eq!((command.first_trial, command.runs), (37, 3));
    }
    for card in [
        ".mc 1 START -1",
        ".mc 1 START 1.5",
        ".mc 1 START 2 START 3",
        ".mc 2 START 18446744073709551615",
    ] {
        assert!(
            Netlist::parse(&format!("invalid range\n{card}\n.end\n")).is_err(),
            "{card}"
        );
    }
}
