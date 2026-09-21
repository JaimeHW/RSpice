use super::*;
use crate::simulation::multi_run::{
    HbToneSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable,
};
use crate::simulation::results::SimulationResult;
use crate::simulation::runner::{
    SimulationRequest, SpecExecutionOptions, worker_contract::WorkerSpecExecutionOptions,
};
use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

fn base() -> StudyRunConfig {
    StudyRunConfig {
        postprocess: None,
        constraints: vec![],
        objective_terms: vec![],
        instance_id: AnalysisInstanceId::new(),
        source_revision: ObjectRevision::INITIAL,
        analysis: StudyAnalysis::Native(AnalysisSpec::HarmonicBalance {
            tones: vec![HbToneSpec::new(1000.0, 3).with_source("V1")],
            reltol: 1e-8,
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
        }),
        analysis_line: ".hb 1k HARMS=3 SOURCE1=V1 POINTS=9".into(),
        numeric_options: ".options GMIN=0".into(),
        measurements: vec!["bin:1:magnitude:V(out)".into()],
        histogram_bins: 5,
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
const CIRCUIT: &str = "HB study\n.param AMP=1 ACTUAL={2*AMP}\nV1 out 0 SIN(0 {ACTUAL} 1k)\nR1 out 0 1k\nVDD vdd 0 1.2\nR2 vdd 0 1k\n";

#[test]
fn hb_current_study_monte_carlo_runs_each_materialized_circuit_and_run_set_point() {
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
    let mut base = base();
    base.measurements.push("bin:0:real:V(vdd)".into());
    base.measurements.push("bin:1:imag:I(V1)".into());
    base.measurements.push("bin:0:real:I(VDD)".into());
    let options = SpecExecutionOptions {
        study_base: Some(base.clone()),
        ..Default::default()
    };
    let wire = WorkerSpecExecutionOptions::from(&options);
    let wire: WorkerSpecExecutionOptions =
        serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
    let restored: SpecExecutionOptions = wire.into();
    let environment = Some(AnalysisExecutionEnvironment {
        temperature_celsius: 85.0,
        supply_voltage: Some(2.4),
        nominal_supply_voltage: Some(1.2),
        supply_source_names: vec!["VDD".into()],
    });
    let data = super::super::run_monte_carlo(
        restored.study_base.as_ref().unwrap(),
        McVariationSource::ParameterTolerance,
        &deck,
        None,
        environment,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(data.runs_completed, 3);
    for (actual, expected) in data
        .variables
        .iter()
        .find(|variable| variable.name == "bin:1:magnitude:V(out)")
        .unwrap()
        .samples
        .iter()
        .zip(&expected)
    {
        assert!((actual - expected).abs() < 1e-7, "{actual} != {expected}");
    }
    for (actual, expected) in data
        .variables
        .iter()
        .find(|variable| variable.name == "bin:1:imag:I(V1)")
        .unwrap()
        .samples
        .iter()
        .zip(&expected)
    {
        assert!(
            (actual - expected / 1000.0).abs() < 1e-10,
            "current {actual} != {expected}/1000"
        );
    }
    assert!(
        data.variables
            .iter()
            .find(|variable| variable.name == "bin:0:real:I(VDD)")
            .unwrap()
            .samples
            .iter()
            .all(|value| (value + 0.0024).abs() < 1e-12)
    );
    assert!(
        data.variables
            .iter()
            .find(|variable| variable.name == "bin:0:real:V(vdd)")
            .unwrap()
            .samples
            .iter()
            .all(|value| (value - 2.4).abs() < 1e-9)
    );
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
    base.measurements = vec!["scalar:V(out)".into()];
    assert!(
        super::super::run_monte_carlo(
            &base,
            McVariationSource::ParameterTolerance,
            &deck,
            None,
            None,
            &NoAbort
        )
        .unwrap_err()
        .to_string()
        .contains("bin:index")
    );
}

#[test]
fn hb_current_study_optimization_reaches_a_current_target_and_retains_verbose() {
    let spec = AnalysisSpec::Optimization {
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
        cost_tolerance: 1e-16,
        fd_step: 1e-4,
        initial_step: 0.25,
        min_step: 1e-8,
    };
    let mut base = base();
    base.measurements = vec!["bin:1:magnitude:I(V1)".into()];
    let result = dispatch(spec.clone(), base.clone(), &format!("{CIRCUIT}.end\n"));
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
        converged && best_cost <= 1e-16,
        "{best_cost}, {best_variables:?}"
    );
    assert!(
        (best_variables["AMP"] - 0.75).abs() < 1e-4,
        "{best_variables:?}"
    );
    for verbose in [false, true] {
        let StudyAnalysis::Native(AnalysisSpec::HarmonicBalance { verbose: flag, .. }) =
            &mut base.analysis
        else {
            unreachable!()
        };
        *flag = verbose;
        let request = SimulationRequest::Spec {
            spec: Box::new(spec.clone()),
            options: Box::new(SpecExecutionOptions {
                study_base: Some(base.clone()),
                ..Default::default()
            }),
        };
        assert_eq!(
            crate::simulation::runner::request_asked_for_verbose(&request),
            verbose
        );
    }
}
