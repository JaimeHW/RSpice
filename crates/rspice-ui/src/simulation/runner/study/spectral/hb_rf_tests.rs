use super::*;
use crate::simulation::multi_run::{
    FrequencySweep, HbToneSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable,
    SpPort,
};
use crate::simulation::runner::{
    SpecExecutionOptions, worker_contract::WorkerSpecExecutionOptions,
};
use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

const CIRCUIT: &str = "HB RF study\n.param R=100 ACTUAL={2*R}\n.temp 12\nP1 p1 0 PORT=1 Z0=50\nP2 p2 0 PORT=2 Z0=50\nRS p1 p2 {ACTUAL}\nVIN in 0 DC 0 AC 1\nRNOISE in out {ACTUAL}\n";

fn base(noise: bool) -> StudyRunConfig {
    let request = if noise {
        AnalysisSpec::Hbnoise {
            input_sideband: -1,
            output_sideband: -1,
            noise_reference: Some(services::HbNoiseReference {
                source_resistor: "RNOISE".into(),
                temperature_kelvin: 300.15,
            }),
            start_freq: 1e4,
            stop_freq: 2e4,
            // Legacy AC grids retain one point for LIN 2; integration needs LIN 3.
            points_per_unit: 3,
            sweep: FrequencySweep::Linear,
            output_node: "out".into(),
            output_ref: "0".into(),
            input_source: "VIN".into(),
            max_sideband: 1,
            integrated_noise: true,
            noise_figure: true,
            contributor_ranking: true,
        }
    } else {
        AnalysisSpec::Hbsp {
            start_freq: 1e4,
            stop_freq: 2e4,
            points_per_unit: 2,
            sweep: FrequencySweep::Linear,
            ports: ["p1", "p2"]
                .into_iter()
                .map(|node| SpPort {
                    node_pos: node.into(),
                    node_neg: "0".into(),
                    z0: Some(50.0),
                })
                .collect(),
            max_sideband: 1,
            mixed_mode: false,
            noise_parameters: false,
            noise_reference: None,
        }
    };
    StudyRunConfig {
        instance_id: AnalysisInstanceId::new(), source_revision: ObjectRevision::INITIAL,
        analysis: StudyAnalysis::Hb(Box::new(StudyHbConfig { request: AnalysisSpec::HarmonicBalance {
            tones: vec![HbToneSpec::new(1e6, 3)], reltol: 1e-8, abstol: 1e-12,
            max_iterations: 40, damping: 1.0, min_damping: 0.02, oversample: 3,
            collocation_points: Some(9), max_mixing_order: 3, use_krylov: false,
            gmres_restart: 12, source_stepping: false, use_exact_jacobian: true, verbose: false,
        }, operating_point: StudyOperatingPoint {
            instance_id: AnalysisInstanceId::new(), source_revision: ObjectRevision::INITIAL,
            config: crate::simulation::dialog::OpConfig {
                temperature_mode: crate::simulation::dialog::OpTemperatureMode::Explicit,
                temperature_celsius: 27.0, ..Default::default()
            }, numeric_options: ".options GMIN=1e-7".into(),
        } })),
        postprocess: Some(StudyPostprocess {
            periodic_options: None,
            producer_instance_id: AnalysisInstanceId::new(), producer_source_revision: ObjectRevision::INITIAL,
            producer_analysis_line: ".hb 1meg HARMS=3 POINTS=9".into(),
            producer_numeric_options: ".options GMIN=0".into(), request,
        }),
        analysis_line: if noise { "* RSPICE HBNOISE LIN 3 1e4 2e4 OUT=out REF=0 IN=VIN MAXSIDEBAND=1 INTEGRATED=true CONTRIBUTORS=true" } else { "* RSPICE HBSP LIN 2 1e4 2e4 MAXSIDEBAND=1" }.into(),
        numeric_options: String::new(),
        measurements: if noise { vec!["bin:0:real:output_noise", "bin:1:real:input_noise", "scalar:noise.output_rms", "scalar:noise.input_rms", "bin:0:real:noise_figure_db"] } else { vec!["bin:0:real:S11", "bin:1:magnitude:S21[k=+0,m=+0]"] }.into_iter().map(str::to_owned).collect(),
        histogram_bins: 5, constraints: vec![], objective_terms: vec![],
    }
}

fn dispatch(spec: AnalysisSpec, base: StudyRunConfig, deck: &str) -> SimulationResult {
    let options = SpecExecutionOptions {
        study_base: Some(base),
        ..Default::default()
    };
    let StudyAnalysis::Hb(expected) = &options.study_base.as_ref().unwrap().analysis else {
        unreachable!()
    };
    let expected = expected.clone();
    let wire = WorkerSpecExecutionOptions::from(&options);
    let wire: WorkerSpecExecutionOptions =
        serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
    let restored: SpecExecutionOptions = wire.into();
    let StudyAnalysis::Hb(actual) = &restored.study_base.as_ref().unwrap().analysis else {
        panic!("configured HB")
    };
    assert_eq!(actual, &expected);
    crate::simulation::runner::spec::run_spec_request(
        &EngineBridge::new(),
        spec,
        restored,
        deck,
        None,
        &Default::default(),
        &NoAbort,
    )
    .unwrap()
}

#[test]
fn hb_rf_study_monte_carlo_uses_fresh_hb_and_varied_circuit_for_both_consumers() {
    let deck = format!("{CIRCUIT}.mc 3 START=2 SEED=31 DIST UNIFORM SPREAD .2 PARAMS R\n.end\n");
    let nominal = rspice_core::Netlist::parse(&deck).unwrap();
    let mut oracle = MonteCarloStudyConfig::new(3, 31, vec!["resistance".into()]);
    oracle.first_trial = 2;
    oracle.distribution = Distribution::Uniform { tolerance: 0.2 };
    oracle.parameter_filter = vec!["R".into()];
    let expected = rspice_core::Engine::default()
        .run_monte_carlo_measurements_with_abort(&nominal, &oracle, &NoAbort, |_, trial, _, _| {
            Ok(vec![trial.params.get("ACTUAL").unwrap()])
        })
        .unwrap()
        .variables["resistance"]
        .samples
        .clone();
    assert!(expected.windows(2).any(|pair| pair[0] != pair[1]));
    for noise in [false, true] {
        let base = base(noise);
        let result = dispatch(
            AnalysisSpec::MonteCarlo {
                variation_source: McVariationSource::ParameterTolerance,
                params: vec!["R".into()],
            },
            base.clone(),
            &deck,
        );
        let SimulationResult::MonteCarlo {
            variables,
            runs_completed,
            member_measurements,
            ..
        } = result
        else {
            panic!("MC result")
        };
        assert_eq!(runs_completed, 3);
        assert_eq!(
            member_measurements
                .iter()
                .map(|member| member.member.index())
                .collect::<Vec<_>>(),
            [2, 3, 4]
        );
        for variable in variables {
            for (&actual, &resistance) in variable.samples.iter().zip(&expected) {
                let psd = 4.0 * 1.380649e-23 * 300.15 * resistance;
                let expected = match variable.name.as_str() {
                    "bin:0:real:S11" => resistance / (resistance + 100.0),
                    "bin:1:magnitude:S21[k=+0,m=+0]" => 100.0 / (resistance + 100.0),
                    "bin:0:real:output_noise" | "bin:1:real:input_noise" => psd,
                    "scalar:noise.output_rms" | "scalar:noise.input_rms" => (psd * 1e4).sqrt(),
                    "bin:0:real:noise_figure_db" => 0.0,
                    name => panic!("unexpected {name}"),
                };
                let tolerance = if expected == 0.0 {
                    1e-9
                } else {
                    expected.abs() * 1e-6
                };
                assert!(
                    (actual - expected).abs() <= tolerance,
                    "{}: {actual} != {expected}",
                    variable.name
                );
            }
        }
        assert!(matches!(
            super::super::run_monte_carlo(
                &base,
                McVariationSource::ParameterTolerance,
                &deck,
                None,
                None,
                &ImmediateAbort
            ),
            Err(SimulationError::Aborted)
        ));
    }
}

#[test]
fn hb_rf_study_optimizer_reaches_scattering_and_integrated_noise_targets() {
    for noise in [false, true] {
        let mut base = base(noise);
        base.measurements = vec![
            if noise {
                "scalar:noise.output_rms"
            } else {
                "bin:0:real:S11"
            }
            .into(),
        ];
        let target = if noise {
            (4.0_f64 * 1.380649e-23 * 300.15 * 150.0 * 1e4).sqrt()
        } else {
            0.6
        };
        let tolerance = if noise { 1e-28 } else { 1e-16 };
        let result = dispatch(
            AnalysisSpec::Optimization {
                search: Default::default(),
                variables: vec![OptimizationVariable {
                    name: "R".into(),
                    min: 25.0,
                    max: 150.0,
                    initial: 100.0,
                }],
                objective_expression: None,
                objective_node: "out".into(),
                objective_ref: "0".into(),
                goal: OptimizationGoal::Target,
                target: Some(target),
                algorithm: OptimizationAlgorithm::PatternSearch,
                max_iterations: 48,
                cost_tolerance: tolerance,
                fd_step: 1e-4,
                initial_step: 25.0,
                min_step: 1e-8,
            },
            base,
            &format!("{CIRCUIT}.end\n"),
        );
        let SimulationResult::Optimization {
            best_variables,
            best_cost,
            converged,
            ..
        } = result
        else {
            panic!("optimizer result")
        };
        assert!(
            converged && best_cost <= tolerance,
            "noise={noise}: {best_cost}, {best_variables:?}"
        );
        assert!(
            (best_variables["R"] - 75.0).abs() < 1e-3,
            "noise={noise}: {best_variables:?}"
        );
    }
}
