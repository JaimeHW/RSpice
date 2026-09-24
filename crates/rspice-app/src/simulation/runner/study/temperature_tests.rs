//! Selected physical temperature must reach parsing and every varied trial.
use super::*;
use crate::simulation::dialog::{
    OpConfig, OpInitialGuess, OpNodeInitialization, OpPreviousState, OpTemperatureMode,
};
use crate::simulation::multi_run::{
    AnalysisSpec, HbToneSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable,
    PssMethod,
};
use crate::simulation::results::SimulationResult;
use rspice_core::NoAbort;

fn base(family: u8, inherited: bool) -> StudyRunConfig {
    let mut config = OpConfig {
        temperature_celsius: 37.0,
        temperature_mode: if inherited {
            OpTemperatureMode::PvtRunSet
        } else {
            OpTemperatureMode::Explicit
        },
        ..Default::default()
    };
    config.run_point.supply_voltage = Some(2.0);
    config.run_point.nominal_supply_voltage = Some(1.0);
    config.run_point.supply_source_names = vec!["VDD".into()];
    let op = StudyOperatingPoint {
        instance_id: AnalysisInstanceId::new(),
        source_revision: ObjectRevision::INITIAL,
        config: config.clone(),
        numeric_options: ".options GMIN=0".into(),
    };
    let (analysis, line, prefix) = match family {
        0 => (
            StudyAnalysis::Basic(AnalysisConfig::DcOp(config)),
            ".op".into(),
            "scalar",
        ),
        1 => (
            StudyAnalysis::Hb(Box::new(StudyHbConfig {
                request: AnalysisSpec::HarmonicBalance {
                    tones: vec![HbToneSpec::new(1000.0, 3)],
                    reltol: 1e-8,
                    abstol: 1e-12,
                    max_iterations: 40,
                    damping: 1.0,
                    min_damping: 0.01,
                    oversample: 4,
                    collocation_points: None,
                    max_mixing_order: 3,
                    use_krylov: false,
                    gmres_restart: 12,
                    source_stepping: false,
                    use_exact_jacobian: true,
                    verbose: false,
                },
                operating_point: op,
            })),
            ".hb 1000 HARMS=3".into(),
            "bin:0:real",
        ),
        2 => (
            StudyAnalysis::Pss(Box::new(StudyPssConfig {
                request: AnalysisSpec::Pss {
                    method: PssMethod::Shooting,
                    fundamental_freq: 1000.0,
                    tone_sources: vec!["VIN".into()],
                    tstab_periods: 0,
                    points_per_period: 64,
                    tolerance: 1e-7,
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
                operating_point: op,
            })),
            ".pss fund=1000 harms=3 points=64 tstabperiods=0".into(),
            "bin:0:real",
        ),
        _ => {
            let request = crate::simulation::plan::QpssDraft {
                tones: "1000,1414.2135623730951".into(),
                harmonics: "1,1".into(),
                dc_initialization: true,
                ..Default::default()
            }
            .to_spec()
            .unwrap();
            let line = request.driven_qpss_config().unwrap().to_spice().unwrap();
            (
                StudyAnalysis::Qpss(Box::new(StudyQpssConfig {
                    request,
                    operating_point: op,
                })),
                line,
                "tuple:0,0:real",
            )
        }
    };
    StudyRunConfig {
        instance_id: AnalysisInstanceId::new(),
        source_revision: ObjectRevision::INITIAL,
        analysis,
        analysis_line: line,
        numeric_options: ".options GMIN=0".into(),
        postprocess: None,
        measurements: ["out", "rprobe"]
            .map(|node| format!("{prefix}:V({node})"))
            .to_vec(),
        histogram_bins: 3,
        constraints: vec![],
        objective_terms: vec![],
    }
}
fn environment() -> AnalysisExecutionEnvironment {
    AnalysisExecutionEnvironment {
        temperature_celsius: 52.0,
        supply_voltage: Some(3.0),
        nominal_supply_voltage: Some(1.0),
        supply_source_names: vec!["VDD".into()],
    }
}
fn deck(statistics: bool) -> String {
    format!(
        "Thermal study\n.param R={}\nVIN drive 0 SIN(0 .01 1k) AC .01\nVDD in drive DC 1\nRS in out {{R*(1+TEMP/100)}}\n.if (TEMP>40)\nRL out 0 2k\n.else\nRL out 0 1k\n.endif\nC1 out 0 100n\nVR rprobe 0 {{R}}\nRR rprobe 0 1meg\n.options TEMP=12 GMIN=0\n.mc 2 START=2 SEED=31 DIST UNIFORM SPREAD .2 {}\n.end\n",
        if statistics {
            "{aunif(1000,200)}"
        } else {
            "1000"
        },
        if statistics { "" } else { "PARAMS R" }
    )
}
fn bind_compatible_previous(base: &mut StudyRunConfig, source: &str) {
    let previous_source = source.replace("R=1000", "R=1500");
    let SimulationResult::DcOp(point) = EngineBridge::new()
        .run(&AnalysisConfig::dc_op(), &previous_source)
        .unwrap()
    else {
        panic!("baseline OP")
    };
    let op = match &mut base.analysis {
        StudyAnalysis::Basic(AnalysisConfig::DcOp(op)) => op,
        StudyAnalysis::Hb(config) => &mut config.operating_point.config,
        StudyAnalysis::Pss(config) => &mut config.operating_point.config,
        StudyAnalysis::Qpss(config) => &mut config.operating_point.config,
        _ => unreachable!(),
    };
    op.initial_guess = OpInitialGuess::PreviousCompatible;
    op.node_initialization = OpNodeInitialization::IgnoreIcAndNodeset;
    op.previous_state = Some(OpPreviousState {
        source_content_digest:
            crate::simulation::execution::operating_point_effective_source_digest(
                &previous_source,
                Default::default(),
            ),
        producer_snapshot_digest: crate::product::ContentDigest::from_bytes([2; 32]),
        producer_result_digest: crate::product::ContentDigest::from_bytes([3; 32]),
        node_names: point.mna_node_names,
        branch_names: point.mna_branch_names,
        solution: point.mna_solution,
    });
}
#[test]
fn study_temperature_precedes_parameter_statistics_and_optimization_replay() {
    for family in 0..4 {
        for inherited in [false, true] {
            let mut base = base(family, inherited);
            let previous_source = if family == 2 {
                deck(false).replace("C1 out 0 100n\n", "")
            } else {
                deck(false)
            };
            bind_compatible_previous(&mut base, &previous_source);
            for statistics in [false, true] {
                let mut source = deck(statistics);
                if family == 2 {
                    // PSS must retain a varied driven waveform even without
                    // independent charge/flux storage in the circuit.
                    source = source.replace("C1 out 0 100n\n", "");
                }
                let result = run_monte_carlo(
                    &base,
                    if statistics {
                        McVariationSource::DeckStatistics
                    } else {
                        McVariationSource::ParameterTolerance
                    },
                    &source,
                    None,
                    Some(environment()),
                    &NoAbort,
                )
                .unwrap();
                assert_eq!(result.num_failures, 0);
                let output = result
                    .variables
                    .iter()
                    .find(|v| v.name == base.measurements[0])
                    .unwrap();
                let draws = result
                    .variables
                    .iter()
                    .find(|v| v.name == base.measurements[1])
                    .unwrap();
                assert!(draws.samples[0] != draws.samples[1]);
                let temperature = if inherited { 52.0 } else { 37.0 };
                let load = if inherited { 2000.0 } else { 1000.0 };
                for (&actual, &r) in output.samples.iter().zip(&draws.samples) {
                    let expected = 3.0 * load / (load + r * (1.0 + temperature / 100.0));
                    assert!(
                        (actual - expected).abs() < 1e-5,
                        "family={family} inherited={inherited} statistics={statistics}: {actual} != {expected}"
                    );
                }
            }
        }
    }
    let mut base = base(1, false);
    bind_compatible_previous(&mut base, &deck(false));
    base.measurements.truncate(1);
    let target = 3.0 * 1000.0 / (1000.0 + 800.0 * 1.37);
    let result = super::super::spec::run_spec_request_with_environment(
        &EngineBridge::new(),
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
            max_iterations: 40,
            cost_tolerance: 1e-12,
            fd_step: 1e-4,
            initial_step: 0.25,
            min_step: 1e-8,
        },
        super::super::SpecExecutionOptions {
            study_base: Some(base),
            ..Default::default()
        },
        &deck(false),
        None,
        &Default::default(),
        Some(environment()),
        &NoAbort,
    )
    .unwrap();
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
        "{best_variables:?}: {best_cost}"
    );
    assert!((best_variables["R"] - 800.0).abs() < 0.01);
}
