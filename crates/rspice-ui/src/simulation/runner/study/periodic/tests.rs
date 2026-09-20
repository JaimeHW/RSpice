use super::*;
use crate::simulation::dialog::{OpConfig, OpTemperatureMode};
use crate::simulation::multi_run::{
    FrequencySweep, HbToneSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable,
    PssMethod, SpPort,
};
use crate::simulation::runner::worker_contract::WorkerSpecExecutionOptions;
use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

const CIRCUIT: &str = "Periodic RF study\n.param R=1000 ACTUAL={2*R}\nVIN in 0 SIN(0 0.1 1k) AC 1\nRVAR in out {ACTUAL} TC1=.01\nRL out 0 2k\nC1 out 0 100n\nLPROBE lp 0 .01\nRPROBE lp 0 {ACTUAL/1000}\nP1 p1 0 PORT=1 Z0=50\nP2 p2 0 PORT=2 Z0=50\nRSP p1 p2 {ACTUAL}\n.options TEMP=12 TNOM=27 GMIN=1e-10\n";

fn base(kind: usize, hb: bool) -> StudyRunConfig {
    use services::*;
    let carrier = if hb {
        PeriodicCarrier::Hb
    } else {
        PeriodicCarrier::Pss
    };
    let (request, options, observations) = match kind {
        0 => (
            AnalysisSpec::Pac,
            Some(StudyPeriodicOptions::Pac(PacRunConfig {
                pss_fundamental_freq: 1000.0,
                pss_num_harmonics: 3,
                pss_tolerance: 1e-6,
                start_freq: 100.0,
                stop_freq: 200.0,
                points_per_unit: 3,
                sweep: PacFrequencySweep::Linear,
                sideband_min: -1,
                sideband_max: 0,
                input_source: "VIN".into(),
                output_node: "out".into(),
                output_ref: Some("0".into()),
                pac_magnitude: 2.5,
                include_dc: true,
                reltol: 2e-7,
                abstol: 3e-12,
                carrier,
            })),
            vec!["bin:0:real:V(out)[sb=+0]", "bin:0:imag:V(out)[sb=+0]"],
        ),
        1 => (
            AnalysisSpec::Pxf,
            Some(StudyPeriodicOptions::Pxf(PxfRunConfig {
                pss_fundamental_freq: 1000.0,
                pss_num_harmonics: 3,
                pss_tolerance: 1e-6,
                start_freq: 100.0,
                stop_freq: 200.0,
                points_per_unit: 3,
                sweep: PxfFrequencySweep::Linear,
                input_source: "VIN".into(),
                output_node: "out".into(),
                output_ref: None,
                input_sideband: 1,
                output_sideband: 1,
                max_sideband: 1,
                reltol: 2e-7,
                abstol: 3e-12,
                carrier,
            })),
            vec![
                "bin:0:real:H(sb1->sb1, V(out))",
                "bin:0:imag:H(sb1->sb1, V(out))",
                "bin:0:real:Converted Output Frequency",
            ],
        ),
        2 => (
            AnalysisSpec::Pnoise,
            Some(StudyPeriodicOptions::Pnoise(PnoiseRunConfig {
                pss_fundamental_freq: 1000.0,
                pss_num_harmonics: 3,
                pss_tolerance: 1e-6,
                start_freq: 100.0,
                stop_freq: 200.0,
                points_per_unit: 3,
                sweep: PnoiseFrequencySweep::Linear,
                input_source: "VIN".into(),
                output_node: "out".into(),
                output_ref: None,
                input_sideband: 0,
                output_sideband: 0,
                max_sideband: 1,
                noise_ref: PnoiseReference::Input,
                integrated_noise: true,
                noise_summary: true,
                reltol: 2e-7,
                abstol: 3e-12,
                carrier,
            })),
            vec![
                "bin:0:real:output_noise",
                "bin:0:real:input_noise",
                "scalar:noise.output_rms",
            ],
        ),
        3 => (
            AnalysisSpec::Pstb,
            Some(StudyPeriodicOptions::Pstb(PstbRunConfig {
                pss_fundamental_freq: 1000.0,
                pss_num_harmonics: 3,
                pss_tolerance: 1e-6,
                probe_instance: "LPROBE".into(),
                max_harmonics: 2,
                num_multipliers: 2,
                stability_threshold: 1.01,
                detect_subharmonics: false,
                eigenvalue_tolerance: 3e-10,
            })),
            vec![
                "scalar:pstb.max_multiplier_magnitude",
                "scalar:pstb.stability_threshold",
                "scalar:pstb.mode_count",
            ],
        ),
        _ => (
            AnalysisSpec::Psp {
                start_freq: 100.0,
                stop_freq: 200.0,
                points_per_unit: 3,
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
            },
            None,
            vec!["bin:0:real:S11", "bin:1:magnitude:S21[k=+0,m=+0]"],
        ),
    };
    let analysis = if hb {
        StudyAnalysis::Native(AnalysisSpec::HarmonicBalance {
            tones: vec![HbToneSpec::new(1000.0, 3)],
            reltol: 1e-6,
            abstol: 1e-12,
            max_iterations: 40,
            damping: 1.0,
            min_damping: 0.02,
            oversample: 3,
            collocation_points: Some(9),
            max_mixing_order: 3,
            use_krylov: false,
            gmres_restart: 12,
            source_stepping: false,
            use_exact_jacobian: true,
            verbose: false,
        })
    } else {
        StudyAnalysis::Pss(Box::new(StudyPssConfig {
            request: AnalysisSpec::Pss {
                method: PssMethod::Shooting,
                fundamental_freq: 1000.0,
                tone_sources: vec!["VIN".into(), "P1".into(), "P2".into()],
                tstab_periods: 0,
                points_per_period: 128,
                tolerance: 1e-6,
                oscillator_mode: false,
                oscillator_node: None,
                num_harmonics: 3,
                integration_method: None,
                tstab: 0.0,
                max_iterations: 30,
                abstol: 1e-10,
                damping: 1.0,
                max_period_change: 0.05,
                verbose: false,
            },
            operating_point: StudyOperatingPoint {
                instance_id: AnalysisInstanceId::new(),
                source_revision: ObjectRevision::INITIAL,
                config: OpConfig {
                    temperature_mode: OpTemperatureMode::Explicit,
                    temperature_celsius: 37.0,
                    ..Default::default()
                },
                numeric_options: ".options GMIN=1e-7".into(),
            },
        }))
    };
    StudyRunConfig {
        instance_id: AnalysisInstanceId::new(),
        source_revision: ObjectRevision::INITIAL,
        analysis,
        postprocess: Some(StudyPostprocess {
            producer_instance_id: AnalysisInstanceId::new(),
            producer_source_revision: ObjectRevision::INITIAL,
            producer_analysis_line: if hb {
                ".hb 1k HARMS=3 POINTS=9"
            } else {
                ".pss fund=1k harms=3 tstabperiods=0 points=128"
            }
            .into(),
            producer_numeric_options: ".options GMIN=0 TEMP=37".into(),
            request,
            periodic_options: options,
        }),
        analysis_line: "* configured periodic RF consumer".into(),
        numeric_options: String::new(),
        measurements: observations.into_iter().map(str::to_owned).collect(),
        histogram_bins: 5,
        constraints: vec![],
        objective_terms: vec![],
    }
}

fn dispatch(spec: AnalysisSpec, base: StudyRunConfig, deck: &str) -> SimulationResult {
    let original = base.postprocess.clone();
    let wire = WorkerSpecExecutionOptions::from(&SpecExecutionOptions {
        study_base: Some(base),
        ..Default::default()
    });
    let wire =
        serde_json::from_str::<WorkerSpecExecutionOptions>(&serde_json::to_string(&wire).unwrap())
            .unwrap();
    let options: SpecExecutionOptions = wire.into();
    assert_eq!(options.study_base.as_ref().unwrap().postprocess, original);
    crate::simulation::runner::spec::run_spec_request(
        &EngineBridge::new(),
        spec,
        options,
        deck,
        None,
        &Default::default(),
        &NoAbort,
    )
    .unwrap()
}

fn gain(resistance: f64, frequency: f64) -> num_complex::Complex64 {
    num_complex::Complex64::new(
        1.0 + resistance / 2000.0,
        std::f64::consts::TAU * frequency * resistance * 1e-7,
    )
    .inv()
}

#[test]
fn periodic_rf_study_consumers_use_varied_circuits_and_complete_options() {
    let deck = format!("{CIRCUIT}.mc 2 START=2 SEED=31 DIST UNIFORM SPREAD .2 PARAMS R\n.end\n");
    let nominal = rspice_core::Netlist::parse(&deck).unwrap();
    let mut oracle = MonteCarloStudyConfig::new(2, 31, vec!["resistance".into()]);
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
    assert_ne!(expected[0], expected[1]);
    for (kind, hb) in (0..5)
        .map(|kind| (kind, false))
        .chain((0..3).map(|kind| (kind, true)))
    {
        let base = base(kind, hb);
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
            ..
        } = result
        else {
            panic!("MC")
        };
        assert_eq!(runs_completed, 2, "kind={kind}, hb={hb}");
        for variable in variables {
            for (&actual, &resistance) in variable.samples.iter().zip(&expected) {
                let r = resistance * 1.1;
                let parallel = 1.0 / (1.0 / r + 1.0 / 2000.0);
                let psd = 4.0 * 1.380649e-23 * 310.15 * parallel
                    / (1.0 + (std::f64::consts::TAU * 100.0 * parallel * 1e-7).powi(2));
                let expected = match variable.name.as_str() {
                    "bin:0:real:V(out)[sb=+0]" => 2.5 * gain(r, 100.0).re,
                    "bin:0:imag:V(out)[sb=+0]" => 2.5 * gain(r, 100.0).im,
                    "bin:0:real:H(sb1->sb1, V(out))" => gain(r, 1100.0).re,
                    "bin:0:imag:H(sb1->sb1, V(out))" => gain(r, 1100.0).im,
                    "bin:0:real:Converted Output Frequency" => 1100.0,
                    "bin:0:real:output_noise" => psd,
                    "bin:0:real:input_noise" => psd / gain(r, 100.0).norm_sqr(),
                    "scalar:noise.output_rms" => {
                        assert!(actual > 0.0);
                        continue;
                    }
                    "scalar:pstb.max_multiplier_magnitude" => (-resistance / 10000.0).exp(),
                    "scalar:pstb.stability_threshold" => 1.01,
                    "scalar:pstb.mode_count" => 2.0,
                    "bin:0:real:S11" => resistance / (resistance + 100.0),
                    "bin:1:magnitude:S21[k=+0,m=+0]" => 100.0 / (resistance + 100.0),
                    name => panic!("{name}"),
                };
                assert!(
                    (actual - expected).abs() < expected.abs() * 2e-3,
                    "kind={kind}, hb={hb}, {}: {actual} != {expected}",
                    variable.name
                );
            }
        }
        assert!(matches!(
            run_monte_carlo(
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
fn periodic_rf_study_optimization_reaches_a_configured_transfer_target() {
    let mut base = base(0, false);
    base.measurements = vec!["bin:0:real:V(out)[sb=+0]".into()];
    let target = 2.5 * gain(2.0 * 800.0 * 1.1, 100.0).re;
    let result = dispatch(
        AnalysisSpec::Optimization {
            search: Default::default(),
            variables: vec![OptimizationVariable {
                name: "R".into(),
                min: 500.0,
                max: 1500.0,
                initial: 1000.0,
            }],
            objective_expression: None,
            objective_node: "out".into(),
            objective_ref: "0".into(),
            goal: OptimizationGoal::Target,
            target: Some(target),
            algorithm: OptimizationAlgorithm::PatternSearch,
            max_iterations: 32,
            cost_tolerance: 1e-10,
            fd_step: 1e-4,
            initial_step: 0.25,
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
        panic!("optimization")
    };
    assert!(
        converged && best_cost <= 1e-10,
        "{best_cost}, {best_variables:?}"
    );
    assert!((best_variables["R"] - 800.0).abs() < 0.1);
}
