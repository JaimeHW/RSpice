use super::*;
use crate::simulation::multi_run::{
    AnalysisSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable,
};
use crate::simulation::plan::{
    QpnoiseOutputDraft, QpnoiseSourceSelection, QpssDraft, QpxfSidebandSelection,
    QpxfSourceSelection, QuasiPeriodicAcDraft, QuasiPeriodicNoiseDraft, QuasiPeriodicTransferDraft,
};
use crate::simulation::results::SimulationResult;
use crate::simulation::runner::{
    SpecExecutionOptions, worker_contract::WorkerSpecExecutionOptions,
};
use rspice_core::abort_signal::{ImmediateAbort, NoAbort};
use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod;

const CIRCUIT: &str = "Quasi periodic study\n.param R=1000 ACTUAL={2*R}\nV1 in 0 DC 0.2 AC 0.3 30\nI1 0 out DC 0 AC 0.001 -20\nRS in out {ACTUAL} TC1=0.01\nRL out 0 2k\nC1 out 0 100n\n.options TEMP=12 TNOM=27 GMIN=1e-10\n";
fn producer() -> AnalysisSpec {
    QpssDraft {
        tones: "1k, 1414.2135623730951".into(),
        harmonics: "1,1".into(),
        max_iterations: "19".into(),
        relative_tolerance: "1e-8".into(),
        current_absolute_tolerance: "2e-13".into(),
        voltage_absolute_tolerance: "3e-10".into(),
        max_backtracks: "7".into(),
        max_mixing_order: "2".into(),
        collocation_points: "8,8".into(),
        source_tones: "V1=1; I1=2".into(),
        dc_initialization: true,
        linear_method: QuasiPeriodicLinearMethod::Direct,
        ..Default::default()
    }
    .to_spec()
    .unwrap()
}
fn consumer(kind: usize) -> AnalysisSpec {
    match kind {
        1 => QuasiPeriodicAcDraft {
            explicit_offsets: "100,300,700".into(),
            input_source: "I1".into(),
            input_lattice: "1,-1".into(),
            output_lattice: "1,-1".into(),
            magnitude: "0.002".into(),
            phase_degrees: "73".into(),
            linear_method: QuasiPeriodicLinearMethod::Krylov,
            krylov_restart: "16".into(),
            krylov_cycles: "12".into(),
            linear_tolerance: "2e-11".into(),
            ..Default::default()
        }
        .to_spec()
        .unwrap(),
        2 => QuasiPeriodicTransferDraft {
            explicit_frequencies: "-100,0,117".into(),
            source_selection: QpxfSourceSelection::Named,
            input_sources: "I1\nV1".into(),
            sideband_selection: QpxfSidebandSelection::Explicit,
            input_lattices: "1,-1;0,0".into(),
            output_lattice: "1,-1".into(),
            group_delay: true,
            group_delay_magnitude_floor: "1e-8".into(),
            linear_method: QuasiPeriodicLinearMethod::Krylov,
            krylov_restart: "16".into(),
            ..Default::default()
        }
        .to_spec()
        .unwrap(),
        _ => QuasiPeriodicNoiseDraft {
            explicit_frequencies: "100,300,700".into(),
            source_selection: QpnoiseSourceSelection::Only,
            source_names: "RS thermal".into(),
            additional_outputs: vec![QpnoiseOutputDraft {
                current: true,
                branch: "V1".into(),
                ..Default::default()
            }],
            integrated_noise: true,
            band_start: "150".into(),
            band_stop: "600".into(),
            contributor_ranking: true,
            noise_figure: true,
            source_resistor: "RS".into(),
            reference_temperature: "310.15".into(),
            linear_method: QuasiPeriodicLinearMethod::Krylov,
            krylov_restart: "16".into(),
            ..Default::default()
        }
        .to_spec()
        .unwrap(),
    }
}
fn base(kind: usize) -> StudyRunConfig {
    let producer = producer();
    let line = producer.driven_qpss_config().unwrap().to_spice().unwrap();
    StudyRunConfig {
        instance_id: AnalysisInstanceId::new(),
        source_revision: ObjectRevision::INITIAL,
        analysis: StudyAnalysis::Qpss(Box::new(StudyQpssConfig {
            request: producer,
            operating_point: StudyOperatingPoint {
                instance_id: AnalysisInstanceId::new(),
                source_revision: ObjectRevision::INITIAL,
                config: crate::simulation::dialog::OpConfig {
                    temperature_mode: crate::simulation::dialog::OpTemperatureMode::Explicit,
                    temperature_celsius: 37.0,
                    initial_guess: crate::simulation::dialog::OpInitialGuess::ZeroState,
                    node_initialization:
                        crate::simulation::dialog::OpNodeInitialization::IgnoreIcAndNodeset,
                    ..Default::default()
                },
                numeric_options: ".options GMIN=1e-7 RELTOL=1e-8".into(),
            },
        })),
        analysis_line: if kind == 0 {
            line.clone()
        } else {
            "* configured QP consumer".into()
        },
        numeric_options: if kind == 0 {
            ".options GMIN=0 TEMP=12".into()
        } else {
            String::new()
        },
        postprocess: (kind != 0).then(|| StudyPostprocess {
            producer_instance_id: AnalysisInstanceId::new(),
            producer_source_revision: ObjectRevision::INITIAL,
            producer_analysis_line: line,
            producer_numeric_options: ".options GMIN=0 TEMP=12".into(),
            request: consumer(kind),
            periodic_options: None,
        }),
        measurements: match kind {
            0 => vec![
                "tuple:1,0:real:V(out)",
                "tuple:1,0:imag:V(out)",
                "tuple:-1,0:imag:V(out)",
                "tuple:0,1:magnitude:V(out)",
                "tuple:0,0:real:V(out)",
                "scalar:qpss.normalized_residual",
            ],
            1 => vec![
                "bin:0:real:V(out,0) [k=[1, -1]]",
                "bin:0:imag:V(out,0) [k=[1, -1]]",
            ],
            2 => vec![
                "bin:0:real:H(V(out,0)/V(V1)) [in=[1, -1]; out=[1, -1]]",
                "bin:0:imag:H(V(out,0)/I(I1)) [in=[1, -1]; out=[1, -1]]",
            ],
            _ => vec![
                "bin:0:real:PSD(output 1: V(out,0) [0, 0])",
                "scalar:qpnoise.input_rms(1)",
                "scalar:qpnoise.output_rms(1)",
                "scalar:qpnoise.contributor_rms(1,RS thermal)",
                "scalar:qpnoise.contributor_share_percent(1,RS thermal)",
                "bin:0:real:NF(output 1: V(out,0) [0, 0])",
            ],
        }
        .into_iter()
        .map(str::to_owned)
        .collect(),
        histogram_bins: 5,
        objective_terms: vec![],
        constraints: vec![],
    }
}
fn dispatch(spec: AnalysisSpec, base: StudyRunConfig, deck: &str) -> SimulationResult {
    let editor = crate::simulation::dialog::McDialogState::from_config(
        &crate::simulation::dialog::mc::McConfig {
            base_analysis: Some(base.instance_id),
            measurements: base.measurements.clone(),
            ..Default::default()
        },
    );
    assert_eq!(editor.to_config().unwrap().measurements, base.measurements);
    let StudyAnalysis::Qpss(expected_producer) = &base.analysis else {
        unreachable!()
    };
    let expected_producer = expected_producer.clone();
    let expected_consumer = base.postprocess.clone();
    let wire = WorkerSpecExecutionOptions::from(&SpecExecutionOptions {
        study_base: Some(base),
        ..Default::default()
    });
    let wire =
        serde_json::from_str::<WorkerSpecExecutionOptions>(&serde_json::to_string(&wire).unwrap())
            .unwrap();
    let options: SpecExecutionOptions = wire.into();
    let restored = options.study_base.as_ref().unwrap();
    let StudyAnalysis::Qpss(actual) = &restored.analysis else {
        panic!("native")
    };
    assert_eq!(*actual, expected_producer);
    assert_eq!(restored.postprocess, expected_consumer);
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
fn impedance(r: f64, f: f64) -> num_complex::Complex64 {
    num_complex::Complex64::new(1.0 / r + 1.0 / 2000.0, std::f64::consts::TAU * f * 1e-7).inv()
}
fn drive(magnitude: f64, phase: f64) -> num_complex::Complex64 {
    num_complex::Complex64::from_polar(magnitude, phase.to_radians())
}
fn integrated_output(r: f64) -> f64 {
    let psd = |f| 4.0 * 1.380649e-23 * 310.15 / r * impedance(r, f).norm_sqr();
    let p100 = psd(100.0);
    let p300 = psd(300.0);
    let p700 = psd(700.0);
    let p150 = p100 + (p300 - p100) * 0.25;
    let p600 = p300 + (p700 - p300) * 0.75;
    ((p150 + p300) * 0.5 * 150.0 + (p300 + p600) * 0.5 * 300.0).sqrt()
}
#[test]
fn qp_study_all_families_use_the_varied_circuit_and_retained_lattice() {
    let deck = format!("{CIRCUIT}.mc 2 START=2 SEED=31 DIST UNIFORM SPREAD 0.2 PARAMS R\n.end\n");
    let nominal = rspice_core::Netlist::parse(&deck).unwrap();
    let mut oracle = MonteCarloStudyConfig::new(2, 31, vec!["r".into()]);
    oracle.first_trial = 2;
    oracle.distribution = Distribution::Uniform { tolerance: 0.2 };
    oracle.parameter_filter = vec!["R".into()];
    let expected = rspice_core::Engine::default()
        .run_monte_carlo_measurements_with_abort(&nominal, &oracle, &NoAbort, |_, trial, _, _| {
            Ok(vec![trial.params.get("ACTUAL").unwrap()])
        })
        .unwrap()
        .variables["r"]
        .samples
        .clone();
    assert_ne!(expected[0], expected[1]);
    for kind in 0..4 {
        let base = base(kind);
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
        assert_eq!(runs_completed, 2, "kind={kind}");
        for variable in variables {
            for (&actual, &r) in variable.samples.iter().zip(&expected) {
                let r = r * 1.1;
                let fundamental = drive(0.3, 30.0) * impedance(r, 1000.0) / r;
                let expected = match variable.name.as_str() {
                    "tuple:1,0:real:V(out)" => fundamental.re,
                    "tuple:1,0:imag:V(out)" => fundamental.im,
                    "tuple:-1,0:imag:V(out)" => -fundamental.im,
                    "tuple:0,1:magnitude:V(out)" => 0.001 * impedance(r, 1414.2135623730951).norm(),
                    "tuple:0,0:real:V(out)" => 0.2 * impedance(r, 0.0).re / r,
                    "scalar:qpss.normalized_residual" => {
                        assert!(actual <= 1.0);
                        continue;
                    }
                    "bin:0:real:V(out,0) [k=[1, -1]]" => {
                        (drive(0.002, 73.0) * impedance(r, 100.0 + 1000.0 - 1414.2135623730951)).re
                    }
                    "bin:0:imag:V(out,0) [k=[1, -1]]" => {
                        (drive(0.002, 73.0) * impedance(r, 100.0 + 1000.0 - 1414.2135623730951)).im
                    }
                    "bin:0:real:H(V(out,0)/V(V1)) [in=[1, -1]; out=[1, -1]]" => {
                        impedance(r, -100.0).re / r
                    }
                    "bin:0:imag:H(V(out,0)/I(I1)) [in=[1, -1]; out=[1, -1]]" => {
                        impedance(r, -100.0).im
                    }
                    "bin:0:real:PSD(output 1: V(out,0) [0, 0])" => {
                        4.0 * 1.380649e-23 * 310.15 / r * impedance(r, 100.0).norm_sqr()
                    }
                    "scalar:qpnoise.input_rms(1)" => {
                        (4.0 * 1.380649e-23 * 310.15 * r * 450.0).sqrt()
                    }
                    "scalar:qpnoise.output_rms(1)"
                    | "scalar:qpnoise.contributor_rms(1,RS thermal)" => integrated_output(r),
                    "scalar:qpnoise.contributor_share_percent(1,RS thermal)" => 100.0,
                    "bin:0:real:NF(output 1: V(out,0) [0, 0])" => 0.0,
                    name => panic!("{name}"),
                };
                let tolerance = if expected == 0.0 {
                    1e-9
                } else {
                    expected.abs() * 1e-6
                };
                assert!(
                    (actual - expected).abs() < tolerance,
                    "kind={kind}, {}: {actual} != {expected}",
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
    let mut invalid = base(0);
    invalid.measurements = vec!["tuple:2,0:magnitude:V(out)".into()];
    assert!(
        run_monte_carlo(
            &invalid,
            McVariationSource::ParameterTolerance,
            &deck,
            None,
            None,
            &NoAbort
        )
        .unwrap_err()
        .to_string()
        .contains("does not retain")
    );
}
#[test]
fn qp_study_optimization_reaches_an_explicit_lattice_target() {
    let mut base = base(0);
    base.measurements = vec!["tuple:1,0:magnitude:V(out)".into()];
    let target = 0.3 * impedance(1760.0, 1000.0).norm() / 1760.0;
    let result = dispatch(
        AnalysisSpec::Optimization {
            search: Default::default(),
            variables: vec![OptimizationVariable {
                name: "R".into(),
                min: 500.0,
                max: 1500.0,
                initial: 1000.0,
            }],
            objective_unit: String::new(),
            objective_expression: None,
            objective_node: "out".into(),
            objective_ref: "0".into(),
            goal: OptimizationGoal::Target,
            target: Some(target),
            algorithm: OptimizationAlgorithm::PatternSearch,
            max_iterations: 32,
            cost_tolerance: 1e-12,
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
        panic!("optimizer")
    };
    assert!(
        converged && best_cost <= 1e-12,
        "{best_cost}, {best_variables:?}"
    );
    assert!((best_variables["R"] - 800.0).abs() < 0.1);
}

#[test]
fn qp_study_uses_exact_dc_seed_and_preserves_zero_start_and_supply_environment() {
    use rspice_core::engine::{PeriodicDcOperatingPointSeed, QpssConfig, QpssInitialState};
    let circuit =
        rspice_core::Netlist::parse("Seed basis\nV1 out 0 1\nR1 out 0 1k\n.options GMIN=0\n.end\n")
            .unwrap();
    let mut config = QpssConfig::new(vec![1000.0, 1414.2135623730951], vec![1, 1]);
    config.initial_state = QpssInitialState::DcOperatingPoint;
    let exact = PeriodicDcOperatingPointSeed::try_new(
        vec!["OUT".into()],
        vec!["V1".into()],
        vec![1.0, -0.001],
    )
    .unwrap();
    let engine = rspice_core::Engine::default();
    let solved = engine
        .run_qpss_with_dc_seed_and_abort(&circuit, config.clone(), &exact, &NoAbort)
        .unwrap();
    assert_eq!(
        solved.iterations(),
        0,
        "exact full MNA seed already satisfies the DC-only circuit"
    );
    // Incorrect current alone needs a correction. An internal OP recompute
    // would erase this input and report zero corrections instead.
    let displaced = PeriodicDcOperatingPointSeed::try_new(
        vec!["OUT".into()],
        vec!["V1".into()],
        vec![1.0, -0.002],
    )
    .unwrap();
    let corrected = engine
        .run_qpss_with_dc_seed_and_abort(&circuit, config.clone(), &displaced, &NoAbort)
        .unwrap();
    assert!(corrected.iterations() > 0);
    let stale = PeriodicDcOperatingPointSeed::try_new(
        vec!["other".into()],
        vec!["V1".into()],
        vec![1.0, -0.001],
    )
    .unwrap();
    assert!(
        engine
            .run_qpss_with_dc_seed_and_abort(&circuit, config.clone(), &stale, &NoAbort)
            .is_err()
    );
    assert!(matches!(
        engine.run_qpss_with_dc_seed_and_abort(&circuit, config.clone(), &exact, &ImmediateAbort),
        Err(rspice_core::SimulationError::Aborted)
    ));
    config.initial_state = QpssInitialState::Zero;
    assert!(
        engine
            .run_qpss_with_dc_seed_and_abort(&circuit, config, &exact, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("requires DC")
    );

    let mut base = base(0);
    let StudyAnalysis::Qpss(qpss) = &mut base.analysis else {
        unreachable!()
    };
    qpss.operating_point.config.run_point.supply_voltage = Some(2.0);
    qpss.operating_point.config.run_point.nominal_supply_voltage = Some(1.0);
    qpss.operating_point.config.run_point.supply_source_names = vec!["V1".into()];
    let circuit = rspice_core::Netlist::parse(&format!("{CIRCUIT}.end\n")).unwrap();
    for zero in [false, true] {
        let AnalysisSpec::Qpss { controls, .. } = &mut qpss.request else {
            unreachable!()
        };
        controls.initial_state = if zero {
            QpssInitialState::Zero
        } else {
            QpssInitialState::DcOperatingPoint
        };
        // In zero mode even invalid OP-only numerical options must not be run.
        qpss.operating_point.numeric_options = if zero {
            ".options invalid_option=1"
        } else {
            ".options GMIN=1e-7"
        }
        .into();
        let (_, result) = qpss
            .run_with_circuit(&engine, &circuit, &base.numeric_options, &NoAbort)
            .unwrap();
        let actual = result
            .study_measurement("tuple:0,0:real:V(out)")
            .unwrap()
            .value
            .unwrap();
        let expected = 0.4 * impedance(2200.0, 0.0).re / 2200.0;
        assert!(
            (actual - expected).abs() < 1e-9,
            "supply must scale once; zero={zero}"
        );
    }
    let environment = MonteCarloEnvironment {
        temperature_celsius: 52.0,
        supply_voltage: Some(3.0),
        nominal_supply_voltage: Some(1.0),
        supply_source_names: vec!["V1".into()],
    };
    let StudyAnalysis::Qpss(explicit) = analysis_for_environment(&base, Some(&environment)) else {
        unreachable!()
    };
    assert_eq!(explicit.operating_point.config.temperature_celsius, 37.0);
    assert_eq!(
        explicit.operating_point.config.run_point.supply_voltage,
        None
    );
    let StudyAnalysis::Qpss(qpss) = &mut base.analysis else {
        unreachable!()
    };
    qpss.operating_point.config.temperature_mode =
        crate::simulation::dialog::OpTemperatureMode::PvtRunSet;
    let StudyAnalysis::Qpss(inherited) = analysis_for_environment(&base, Some(&environment)) else {
        unreachable!()
    };
    assert_eq!(inherited.operating_point.config.temperature_celsius, 52.0);
}
