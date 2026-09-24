//! Engine-bridge study tests verify per-trial analyses and worker resource limits.

use super::*;
use crate::simulation::config::{AcAnalysisConfig, AcSweepType, TransientAnalysisConfig};
use rspice_core::analysis::monte_carlo::Distribution;
use rspice_core::engine::{MonteCarloStudyConfig, MonteCarloVariationSource};

#[test]
fn monte_carlo_bases_execute_configured_ac_and_transient_measurements_on_each_trial() {
    let deck = "Configured study\n.param rval=1k\nV1 in 0 DC 1 AC 1\nR1 in out {rval}\nC1 out 0 1u\n.ic V(out)=0\n.meas AC gain FIND VM(out) AT=1k\n.meas TRAN settled FIND V(out) AT=1m\n.end\n";
    let netlist = rspice_core::Netlist::parse(deck).unwrap();
    let cases = [
        (
            AnalysisConfig::Ac(AcAnalysisConfig {
                start_freq: 1000.0,
                stop_freq: 1000.0,
                num_points: 1,
                sweep_type: AcSweepType::Linear,
            }),
            "gain",
        ),
        (
            AnalysisConfig::Transient(TransientAnalysisConfig {
                stop_time: 1e-3,
                step_time: 1e-5,
                start_time: 2e-4,
                max_timestep: Some(1e-5),
                uic: true,
            }),
            "settled",
        ),
    ];
    for (config, name) in cases {
        let mut study = MonteCarloStudyConfig::new(3, 37, vec![name.into()]);
        study.distribution = Distribution::Uniform { tolerance: 0.2 };
        let result = rspice_core::Engine::default()
            .run_monte_carlo_measurements_with_abort(
                &netlist,
                &study,
                &NoAbort,
                |engine, circuit, _, abort| {
                    let result =
                        EngineBridge::run_materialized_with_abort(engine, &config, circuit, abort)
                            .map_err(|error| {
                                rspice_core::SimulationError::Circuit(error.to_string())
                            })?;
                    let value = result
                        .measurement(name)
                        .expect("configured .MEAS is evaluated");
                    let r = circuit.params.get("rval").unwrap();
                    let expected = if name == "gain" {
                        1.0 / 1.0_f64.hypot(std::f64::consts::TAU * 1000.0 * r * 1e-6)
                    } else {
                        1.0 - (-1e-3 / (r * 1e-6)).exp()
                    };
                    assert!(
                        (value - expected).abs() < 2e-3,
                        "{name}: {value} != {expected}"
                    );
                    Ok(vec![value])
                },
            )
            .unwrap();
        assert_eq!(result.num_failures, 0);
        let samples = &result.variables[name].samples;
        assert_eq!(samples.len(), 3);
        assert!(samples.windows(2).any(|pair| pair[0] != pair[1]));
    }
}

#[test]
fn materialized_studies_retain_worker_resource_limits() {
    let netlist =
        rspice_core::Netlist::parse("Bounded study\nV1 in 0 AC 1\nR1 in 0 1k\n.end\n").unwrap();
    let mut config = rspice_core::SimulationConfig::default();
    config.resource_limits.max_analysis_points = 2;
    let error = EngineBridge::run_materialized_with_abort(
        &rspice_core::Engine::new(config),
        &AnalysisConfig::Ac(AcAnalysisConfig {
            start_freq: 1.0,
            stop_freq: 10.0,
            num_points: 10,
            sweep_type: AcSweepType::Linear,
        }),
        &netlist,
        &NoAbort,
    )
    .unwrap_err();
    assert!(
        matches!(error, SimulationError::ResourceLimit { .. }),
        "{error}"
    );
}

#[test]
fn deck_statistics_measurements_replay_model_draws_across_worker_counts() {
    let deck = "Model statistics study\n.model nm nmos level=1 vto={agauss(0.7,0.1,1)} kp=200u\nM1 d g 0 0 nm w=10u l=1u\nR1 vdd d 10k\nV1 vdd 0 3\nV2 g 0 1.2\n.end\n";
    let netlist = rspice_core::Netlist::parse(deck).unwrap();
    let mut study = MonteCarloStudyConfig::new(4, 5, vec!["drain".into()]);
    study.variation_source = MonteCarloVariationSource::DeckStatistics;
    let mut reference = None;
    for workers in [1, 2] {
        let mut configuration = rspice_core::SimulationConfig::default();
        configuration.resource_limits.max_parallel_workers = workers;
        let result = rspice_core::Engine::new(configuration)
            .run_monte_carlo_measurements_with_abort(
                &netlist,
                &study,
                &NoAbort,
                |engine, circuit, index, abort| {
                    let measured = EngineBridge::run_materialized_with_abort(
                        engine,
                        &AnalysisConfig::dc_op(),
                        circuit,
                        abort,
                    )
                    .map_err(|error| rspice_core::SimulationError::Circuit(error.to_string()))?;
                    let value = measured.measurement("V(d)").unwrap();
                    assert_eq!(measured.measurement("v(D)"), Some(value));
                    assert_eq!(measured.measurement("d"), Some(value));
                    assert_eq!(measured.measurement("電圧"), None);
                    let replay = rspice_core::Netlist::parse_with_options(
                        deck,
                        rspice_core::netlist::NetlistParseOptions {
                            statistical_mode: rspice_core::netlist::StatisticalParamMode::Sample,
                            statistical_seed: Some(
                                rspice_core::engine::monte_carlo_deck_trial_seed(5, index),
                            ),
                            ..Default::default()
                        },
                    )
                    .unwrap();
                    let expected = engine
                        .run_dc_op_with_abort(&replay, abort)?
                        .try_voltage_named("d")
                        .unwrap();
                    assert_eq!(value.to_bits(), expected.to_bits());
                    Ok(vec![value])
                },
            )
            .unwrap();
        let samples = result.variables["drain"].samples.clone();
        assert!(samples.windows(2).any(|pair| pair[0] != pair[1]));
        if let Some(reference) = &reference {
            assert_eq!(&samples, reference);
        } else {
            reference = Some(samples);
        }
    }
}

#[test]
fn studio_deck_statistics_executes_native_process_and_expression_variations() {
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
    let fixture = Fixture(
        std::env::temp_dir().join(format!("rspice_studio_mc_{}_{nonce}", std::process::id())),
    );
    std::fs::create_dir(&fixture.0).unwrap();
    std::fs::write(
        fixture.0.join("process.scs"),
        "simulator lang=spectre\nstatistics {\nprocess {\nvary rtop dist=gauss std=100\n}\n}\n",
    )
    .unwrap();
    let path = fixture.0.join("study.cir");
    let deck = "Studio statistics\n.param rtop=1k rbottom={agauss(1k,50,1)}\n.include \"process.scs\"\nV1 in 0 1\nR1 in top {rtop}\nR2 top 0 1k\nR3 in bottom {rbottom}\nR4 bottom 0 1k\n.mc 4 seed 73\n.end\n";
    let run = || {
        crate::services::simulation_runner::run_statistical_monte_carlo_with_environment_and_source_path_and_abort(
        deck, Some(&path), Some(75.0), Some(1.8), Some(1.0), &["V1".into()], &NoAbort,
    ).unwrap()
    };
    let first = run();
    let second = run();
    assert_eq!(first.runs_completed, 4);
    for name in ["V(TOP)", "V(BOTTOM)"] {
        let a = &first
            .variables
            .iter()
            .find(|v| v.name == name)
            .unwrap()
            .samples;
        let b = &second
            .variables
            .iter()
            .find(|v| v.name == name)
            .unwrap()
            .samples;
        assert_eq!(a, b);
        assert!(
            a.windows(2).any(|pair| pair[0] != pair[1]),
            "{name} was nominal"
        );
        assert!(a.iter().all(|value| *value > 0.6 && *value < 1.2));
    }
    for (index, member) in first.trial_measurements.iter().enumerate() {
        assert!(matches!(&member.member,
            crate::state::FamilyMemberId::MonteCarloSequenceTrial {
                index: actual, seed: 73, policy,
            } if *actual == index && policy == "deck-expressions-and-spectre-coordinate-v1"
        ));
    }
}
