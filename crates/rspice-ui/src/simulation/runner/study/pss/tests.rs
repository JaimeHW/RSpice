//! PSS study tests keep operating-point and shooting controls independent.

use super::*;
use crate::simulation::multi_run::{OptimizationAlgorithm, OptimizationGoal, OptimizationVariable};
use crate::simulation::runner::{
    SpecExecutionOptions, worker_contract::WorkerSpecExecutionOptions,
};
use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

const CIRCUIT: &str = "PSS study\n.param AMP=1 ACTUAL={2*AMP}\nV1 out 0 SIN(-0.25 {ACTUAL} 1k)\nR1 out 0 1k TC1=0.01\nR2 out aux 1k\nC1 aux 0 1n\n.options GMIN=1e-10 RELTOL=.01 TEMP=12 TNOM=27\n";

// Real input admittance: temperature-adjusted load plus the series RC branch.
fn input_conductance() -> f64 {
    let omega_c = std::f64::consts::TAU * 1000.0 * 1e-9;
    1.0 / 1100.0 + 1000.0 * omega_c * omega_c / (1.0 + (1000.0 * omega_c).powi(2))
}

fn base() -> StudyRunConfig {
    StudyRunConfig {
        instance_id: AnalysisInstanceId::new(),
        source_revision: ObjectRevision::INITIAL,
        analysis: StudyAnalysis::Pss(Box::new(StudyPssConfig {
            request: AnalysisSpec::Pss {
                method: PssMethod::Shooting,
                fundamental_freq: 1000.0,
                tone_sources: vec!["V1".into()],
                tstab_periods: 0,
                points_per_period: 128,
                tolerance: 1e-6,
                oscillator_mode: false,
                oscillator_node: None,
                num_harmonics: 3,
                integration_method: Some(crate::simulation::dialog::IntegrationMethod::Gear2),
                tstab: 0.0,
                max_iterations: 30,
                abstol: 1e-10,
                damping: 0.8,
                max_period_change: 0.05,
                verbose: false,
            },
            operating_point: StudyOperatingPoint {
                instance_id: AnalysisInstanceId::new(),
                source_revision: ObjectRevision::INITIAL,
                config: OpConfig {
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
        analysis_line:
            ".pss fund=1k autonomous=no tstabperiods=0 points=128 harms=3 tol=1e-6 maxiter=30"
                .into(),
        numeric_options: ".options GMIN=0 RELTOL=1e-6".into(),
        postprocess: None,
        measurements: vec![
            "bin:1:magnitude:V(out)",
            "bin:1:imag:I(V1)",
            "bin:0:real:V(out)",
            "scalar:pss.period",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect(),
        histogram_bins: 5,
        constraints: vec![],
        objective_terms: vec![],
    }
}
fn dispatch(spec: AnalysisSpec, base: StudyRunConfig, deck: &str) -> SimulationResult {
    let options = SpecExecutionOptions {
        study_base: Some(base),
        ..Default::default()
    };
    let wire = WorkerSpecExecutionOptions::from(&options);
    let wire: WorkerSpecExecutionOptions =
        serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
    crate::simulation::runner::spec::run_spec_request(
        &EngineBridge::new(),
        spec,
        wire.into(),
        deck,
        None,
        &Default::default(),
        &NoAbort,
    )
    .unwrap()
}
#[test]
fn pss_study_varies_the_circuit_and_keeps_op_and_shooting_options_separate() {
    let deck = format!("{CIRCUIT}.mc 3 START=2 SEED=31 DIST UNIFORM SPREAD .2 PARAMS AMP\n.end\n");
    let nominal = rspice_core::Netlist::parse(&deck).unwrap();
    let mut oracle = MonteCarloStudyConfig::new(3, 31, vec!["amplitude".into()]);
    oracle.first_trial = 2;
    oracle.distribution = Distribution::Uniform { tolerance: 0.2 };
    oracle.parameter_filter = vec!["AMP".into()];
    let expected = rspice_core::Engine::default()
        .run_monte_carlo_measurements_with_abort(&nominal, &oracle, &NoAbort, |_, trial, _, _| {
            Ok(vec![trial.params.get("ACTUAL").unwrap()])
        })
        .unwrap()
        .variables["amplitude"]
        .samples
        .clone();
    let base = base();
    let StudyAnalysis::Pss(pss) = &base.analysis else {
        unreachable!()
    };
    let op =
        circuit_with_options(&nominal, &pss.operating_point.numeric_options, &NoAbort).unwrap();
    let shooting = circuit_with_options(&nominal, &base.numeric_options, &NoAbort).unwrap();
    assert_eq!(op.options.gmin, Some(1e-7));
    assert_eq!(shooting.options.gmin, Some(0.0));
    assert_eq!(nominal.options.gmin, Some(1e-10));
    assert!(
        circuit_with_options(&nominal, ".options GMIN=1 INVALID_STUDY_OPTION=2", &NoAbort).is_err()
    );
    assert_eq!(nominal.options.gmin, Some(1e-10));
    let result = dispatch(
        AnalysisSpec::MonteCarlo {
            variation_source: McVariationSource::ParameterTolerance,
            params: vec!["AMP".into()],
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
    assert_eq!(runs_completed, 3);
    for variable in variables {
        for (actual, amplitude) in variable.samples.iter().zip(&expected) {
            let expected = match variable.name.as_str() {
                "bin:1:magnitude:V(out)" => *amplitude,
                "bin:1:imag:I(V1)" => amplitude * input_conductance(),
                "bin:0:real:V(out)" => -0.25,
                "scalar:pss.period" => 0.001,
                _ => unreachable!(),
            };
            assert!(
                (actual - expected).abs() < expected.abs() * 5e-4,
                "{}: {actual} != {expected}",
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
#[test]
fn pss_study_optimizer_reaches_a_harmonic_current_target() {
    let mut base = base();
    base.measurements = vec!["bin:1:imag:I(V1)".into()];
    let result = dispatch(
        AnalysisSpec::Optimization {
            search: Default::default(),
            variables: vec![OptimizationVariable {
                name: "AMP".into(),
                min: 0.25,
                max: 1.5,
                initial: 1.0,
            }],
            objective_unit: String::new(),
            objective_expression: None,
            objective_node: "out".into(),
            objective_ref: "0".into(),
            goal: OptimizationGoal::Target,
            target: Some(0.0015),
            algorithm: OptimizationAlgorithm::PatternSearch,
            max_iterations: 32,
            cost_tolerance: 1e-14,
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
        converged && best_cost <= 1e-14,
        "{best_cost}, {best_variables:?}"
    );
    assert!(
        (best_variables["AMP"] - 0.0015 / (2.0 * input_conductance())).abs() < 1e-3,
        "{best_variables:?}"
    );
}
