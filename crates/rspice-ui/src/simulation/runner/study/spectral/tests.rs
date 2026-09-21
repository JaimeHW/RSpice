use super::*;
use crate::simulation::config::{FftFormatChoice, FftRequest, TransientAnalysisConfig};
use crate::simulation::multi_run::{OptimizationAlgorithm, OptimizationGoal, OptimizationVariable};
use crate::simulation::runner::{
    SpecExecutionOptions, worker_contract::WorkerSpecExecutionOptions,
};
use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

fn study(fft: bool) -> StudyRunConfig {
    let request = if fft {
        AnalysisSpec::Fft {
            request: FftRequest {
                output: "V(out)".into(),
                points: 64,
                start: Some(0.0),
                stop: Some(0.001),
                format: Some(FftFormatChoice::Unnormalized),
                ..Default::default()
            },
        }
    } else {
        AnalysisSpec::Fourier {
            fundamental_freq: 1000.0,
            num_harmonics: 5,
            num_periods: 1,
            output_node: "out".into(),
            output_ref: "0".into(),
            additional_outputs: vec![],
            start_time: 0.0,
            stop_time: 0.001,
            compute_thd: true,
            normalize: false,
        }
    };
    let line = match &request {
        AnalysisSpec::Fft { request } => request.to_card(),
        _ => ".four 1k 5 V(out) PERIODS=1 FROM=0 TO=1m".into(),
    };
    StudyRunConfig {
        postprocess: Some(StudyPostprocess {
            periodic_options: None,
            producer_instance_id: AnalysisInstanceId::new(),
            producer_source_revision: ObjectRevision::INITIAL,
            producer_analysis_line: ".tran 2u 1m".into(),
            producer_numeric_options: ".options RELTOL=1e-6".into(),
            request,
        }),
        analysis: AnalysisConfig::Transient(TransientAnalysisConfig {
            stop_time: 0.001,
            step_time: 2e-6,
            start_time: 0.0,
            max_timestep: Some(2e-6),
            uic: false,
        })
        .into(),
        analysis_line: line,
        numeric_options: String::new(),
        instance_id: AnalysisInstanceId::new(),
        source_revision: ObjectRevision::INITIAL,
        measurements: vec!["bin:1:magnitude".into()],
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
const CIRCUIT: &str = "Spectral study\n.param AMP=1 ACTUAL={2*AMP}\nV1 out 0 SIN(0 {ACTUAL} 1k)\nR1 out 0 1k\n.fft V(out) NP=16 STOP=1\n";

#[test]
fn spectral_monte_carlo_uses_each_trial_circuit_and_exact_frozen_transient() {
    let deck = format!("{CIRCUIT}.mc 4 START=3 SEED=19 DIST GAUSS SPREAD 0.1 PARAMS AMP\n.end\n");
    let nominal = rspice_core::Netlist::parse(&deck).unwrap();
    let mut oracle = MonteCarloStudyConfig::new(4, 19, vec!["amplitude".into()]);
    oracle.first_trial = 3;
    oracle.distribution = Distribution::Gaussian { sigma: 0.1 };
    oracle.parameter_filter = vec!["AMP".into()];
    let expected = rspice_core::Engine::default()
        .run_monte_carlo_measurements_with_abort(&nominal, &oracle, &NoAbort, |_, trial, _, _| {
            Ok(vec![trial.params.get("ACTUAL").unwrap()])
        })
        .unwrap()
        .variables["amplitude"]
        .samples
        .clone();
    for fft in [false, true] {
        let mut base = study(fft);
        base.measurements.push("bin:1:phase".into());
        if !fft {
            base.measurements.push("scalar:DC".into());
        }
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
            member_measurements,
            runs_completed,
            ..
        } = result
        else {
            panic!("MC result")
        };
        assert_eq!(runs_completed, 4);
        for member in &member_measurements {
            let amplitude = member.evidence_for("bin:1:magnitude").unwrap();
            let millivolts = amplitude.value_in_unit("mV").unwrap().unwrap();
            assert!((millivolts - amplitude.value.unwrap() * 1000.0).abs() < 1e-9);
            assert!(amplitude.value_in_unit("A").is_err());
            assert!(
                member
                    .evidence_for("bin:1:phase")
                    .unwrap()
                    .value_in_unit("rad")
                    .unwrap()
                    .is_some()
            );
        }
        assert_eq!(
            member_measurements
                .iter()
                .map(|member| member.member.index())
                .collect::<Vec<_>>(),
            [3, 4, 5, 6]
        );
        let magnitudes = &variables
            .iter()
            .find(|variable| variable.name == "bin:1:magnitude")
            .unwrap()
            .samples;
        for (actual, expected) in magnitudes.iter().zip(&expected) {
            assert!(
                (actual - expected).abs() < 2e-4,
                "fft={fft}: actual={actual}, expected={expected}"
            );
        }
        let phase = &variables
            .iter()
            .find(|variable| variable.name == "bin:1:phase")
            .unwrap()
            .samples;
        assert!(
            phase.iter().all(|value| (value + 90.0).abs() < 0.02),
            "{fft}: {phase:?}"
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
        base.postprocess.as_mut().unwrap().producer_instance_id = base.instance_id;
        assert!(base.execution_source(&deck).is_err());
    }
}

#[test]
fn spectral_optimization_changes_candidates_and_finds_the_amplitude_target() {
    let deck = format!("{CIRCUIT}.end\n");
    for fft in [false, true] {
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
            target: Some(1.5),
            algorithm: OptimizationAlgorithm::PatternSearch,
            max_iterations: 32,
            cost_tolerance: 1e-8,
            fd_step: 1e-4,
            initial_step: 0.25,
            min_step: 1e-8,
        };
        let result = dispatch(spec, study(fft), &deck);
        let SimulationResult::Optimization {
            best_variables,
            best_cost,
            converged,
            ..
        } = result
        else {
            panic!("optimization result")
        };
        assert!(
            converged && best_cost <= 1e-8,
            "{fft}: {best_cost}, {best_variables:?}"
        );
        assert!(
            (best_variables["AMP"] - 0.75).abs() < 1e-3,
            "{fft}: {best_variables:?}"
        );
    }
}
