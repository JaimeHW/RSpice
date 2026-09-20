//! Frozen configured-analysis execution on each sampled circuit.

use super::{AnalysisExecutionEnvironment, SimulationError};
use crate::product::{AnalysisInstanceId, ObjectRevision};
use crate::services::simulation_runner as services;
use crate::simulation::dialog::McVariationSource;
use crate::simulation::{config::AnalysisConfig, engine_bridge::EngineBridge, plan::AnalysisKind};
use rspice_core::abort_signal::{AbortReason, AbortSignal, ModelRunControl};
use rspice_core::analysis::monte_carlo::Distribution;
use rspice_core::engine::{
    MonteCarloEnvironment, MonteCarloStudyConfig, MonteCarloVariationSource,
};
use rspice_core::netlist::{AnalysisCommand, MonteCarloDistribution};
use std::path::Path;
use std::sync::{
    Mutex,
    atomic::{AtomicBool, Ordering},
};

/// A study captures the selected instance's configuration before dispatch.
/// Its Run Set point belongs to the study; its solver controls belong to the base.
#[derive(Debug, Clone)]
pub struct StudyRunConfig {
    pub instance_id: AnalysisInstanceId,
    pub source_revision: ObjectRevision,
    pub analysis: AnalysisConfig,
    pub analysis_line: String,
    pub numeric_options: String,
    pub measurements: Vec<String>,
    pub histogram_bins: usize,
}

pub(crate) fn supports_kind(kind: AnalysisKind) -> bool {
    matches!(
        kind,
        AnalysisKind::OperatingPoint
            | AnalysisKind::DcSweep
            | AnalysisKind::Transient
            | AnalysisKind::Ac
            | AnalysisKind::Noise
            | AnalysisKind::PoleZero
            | AnalysisKind::Sensitivity
    )
}

pub(crate) fn validate_measurements(names: &[String]) -> Result<(), String> {
    if names.is_empty() {
        return Err("Select at least one study measurement".into());
    }
    let mut seen = std::collections::HashSet::new();
    for name in names {
        let (mode, key) = name.split_once(':').unwrap_or(("meas", name));
        if key.trim().is_empty()
            || !["meas", "scalar", "last"]
                .iter()
                .any(|value| mode.eq_ignore_ascii_case(value))
        {
            return Err(format!(
                "Invalid study measurement {name:?}; use a .MEAS name, scalar:name, or last:signal"
            ));
        }
        if !seen.insert(name.to_ascii_lowercase()) {
            return Err(format!("Repeated study measurement {name:?}"));
        }
    }
    Ok(())
}

pub(crate) fn run_monte_carlo(
    base: &StudyRunConfig,
    variation_source: McVariationSource,
    source: &str,
    source_path: Option<&Path>,
    environment: Option<AnalysisExecutionEnvironment>,
    abort: &dyn AbortSignal,
) -> Result<services::MonteCarloData, SimulationError> {
    super::spec::ensure_not_aborted(abort)?;
    validate_measurements(&base.measurements).map_err(SimulationError::InvalidConfig)?;
    base.analysis
        .validate()
        .map_err(|errors| SimulationError::InvalidConfig(errors.join("; ")))?;
    // Preserve these cards in the parser's retained source, so expression and
    // native-statistics replay observes the same analysis and solver policy.
    let source = services::splice_before_terminal_end_card(
        source,
        &format!("{}\n{}", base.analysis_line, base.numeric_options),
    );
    let bridge = EngineBridge::new();
    let circuit = bridge.parse_netlist_with_abort_and_source_path(&source, source_path, abort)?;
    let command = circuit
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::MonteCarlo(command) => Some(command),
            _ => None,
        })
        .ok_or_else(|| {
            SimulationError::InvalidConfig("Configured Monte Carlo requires a .MC command".into())
        })?;
    let family = match base.analysis {
        AnalysisConfig::Ac(_) => "AC",
        AnalysisConfig::Transient(_) => "TRAN",
        AnalysisConfig::DcSweep(_) => "DC",
        AnalysisConfig::Noise(_) => "NOISE",
        _ => "",
    };
    for request in &base.measurements {
        let (mode, name) = request.split_once(':').unwrap_or(("meas", request));
        if mode.eq_ignore_ascii_case("meas")
            && !circuit.measurements.iter().any(|measurement| {
                measurement.name.eq_ignore_ascii_case(name)
                    && measurement.analysis.eq_ignore_ascii_case(family)
            })
        {
            return Err(SimulationError::InvalidConfig(format!(
                "Study measurement {name:?} has no .MEAS {family} declaration for the selected base"
            )));
        }
        if mode.eq_ignore_ascii_case("scalar")
            && !matches!(
                base.analysis,
                AnalysisConfig::DcOp(_)
                    | AnalysisConfig::PoleZero(_)
                    | AnalysisConfig::Sensitivity(_)
            )
        {
            return Err(SimulationError::InvalidConfig(format!(
                "{request:?} requires a scalar analysis; use a .MEAS name or last:signal for a waveform"
            )));
        }
        if mode.eq_ignore_ascii_case("last") && family.is_empty() {
            return Err(SimulationError::InvalidConfig(format!(
                "{request:?} requires an analysis with waveforms"
            )));
        }
    }
    let mut study = MonteCarloStudyConfig::new(
        command.runs,
        command.seed.unwrap_or(0x5EED_5EED),
        base.measurements.clone(),
    );
    study.distribution = match command.distribution {
        MonteCarloDistribution::Gaussian => Distribution::Gaussian {
            sigma: command.relative_spread,
        },
        MonteCarloDistribution::Uniform => Distribution::Uniform {
            tolerance: command.relative_spread,
        },
        MonteCarloDistribution::WorstCase => Distribution::WorstCase {
            tolerance: command.relative_spread,
        },
    };
    study.variation_source = match variation_source {
        McVariationSource::ParameterTolerance => MonteCarloVariationSource::ParameterTolerance,
        McVariationSource::DeckStatistics => MonteCarloVariationSource::DeckStatistics,
    };
    study.parameter_filter = command.params.clone();
    study.histogram_bins = base.histogram_bins;
    study.confidence_pct = command.confidence_pct;
    study.confidence_method = command.confidence_method.into();
    study.environment = environment.map(|point| MonteCarloEnvironment {
        temperature_celsius: point.temperature_celsius,
        supply_voltage: point.supply_voltage,
        nominal_supply_voltage: point.nominal_supply_voltage,
        supply_source_names: point.supply_source_names,
    });
    let mut analysis = base.analysis.clone();
    if let AnalysisConfig::DcOp(op) = &mut analysis {
        // The study applies the supply exactly once, after statistical replay.
        // Outside a Run Set retain the selected OP's explicit supply point.
        if let Some(environment) = &study.environment {
            if matches!(
                op.temperature_mode,
                crate::simulation::dialog::OpTemperatureMode::PvtRunSet
                    | crate::simulation::dialog::OpTemperatureMode::ActiveRunSetAxis
            ) {
                op.temperature_celsius = environment.temperature_celsius;
            }
            op.run_point.supply_voltage = None;
            op.run_point.nominal_supply_voltage = None;
            op.run_point.supply_source_names = environment.supply_source_names.clone();
        }
    }
    if let (AnalysisConfig::Noise(noise), Some(environment)) = (&mut analysis, &study.environment) {
        noise.temperature_kelvin =
            rspice_core::constants::celsius_to_kelvin(environment.temperature_celsius);
    }
    let engine = rspice_core::Engine::default().resolved_for_netlist(&circuit);
    let signal = StudyAbort {
        parent: abort,
        failed: AtomicBool::new(false),
    };
    let fatal = Mutex::new(None);
    let first_trial_failure = Mutex::new(None);
    let measurement_verdicts = Mutex::new(std::collections::HashMap::new());
    let result = engine.run_monte_carlo_measurements_with_abort(
        &circuit,
        &study,
        &signal,
        |engine, trial, trial_index, abort| {
            let result = EngineBridge::run_materialized_with_abort(engine, &analysis, trial, abort)
                .map_err(|error| match error {
                    SimulationError::SolverError(_)
                    | SimulationError::ConvergenceFailed { .. }
                    | SimulationError::Attributed { .. }
                    | SimulationError::CircuitError(_) => {
                        let message = error.to_string();
                        first_trial_failure
                            .lock()
                            .unwrap()
                            .get_or_insert_with(|| message.clone());
                        rspice_core::SimulationError::Circuit(message)
                    }
                    other => {
                        let mut failure = fatal.lock().unwrap();
                        if failure.is_none() {
                            *failure = Some(other);
                        }
                        signal.failed.store(true, Ordering::Release);
                        rspice_core::SimulationError::Aborted
                    }
                })?;
            let mut values = Vec::with_capacity(study.measurements.len());
            let mut verdicts = Vec::new();
            for name in &study.measurements {
                let observation = result.study_measurement(name).ok_or_else(|| {
                    let message = format!("Study measurement {name:?} is unavailable or failed");
                    first_trial_failure
                        .lock()
                        .unwrap()
                        .get_or_insert_with(|| message.clone());
                    rspice_core::SimulationError::Circuit(message)
                })?;
                values.push(observation.value.expect("observed study value"));
                if !observation.passed {
                    verdicts.push(observation);
                }
            }
            if !verdicts.is_empty() {
                measurement_verdicts
                    .lock()
                    .unwrap()
                    .insert(trial_index, verdicts);
            }
            Ok(values)
        },
    );
    if let Some(error) = fatal.into_inner().unwrap() {
        return Err(error);
    }
    super::spec::ensure_not_aborted(abort)?;
    let result = result.map_err(|error| bridge.translate_error(error))?;
    if result.num_failures == result.num_runs {
        return Err(SimulationError::CircuitError(format!(
            "All {} Monte Carlo trials failed: {}",
            result.num_runs,
            first_trial_failure
                .into_inner()
                .unwrap()
                .unwrap_or_else(|| "no valid measurements".into())
        )));
    }
    let mut data = super::spec::run_abort_aware_service(abort, || {
        services::finish_monte_carlo_result(
            result,
            engine.config().resource_limits.max_result_values,
            abort,
        )
    })?;
    let mut verdicts = measurement_verdicts.into_inner().unwrap();
    for member in &mut data.trial_measurements {
        super::spec::ensure_not_aborted(abort)?;
        if let Some(observations) = verdicts.remove(&member.member.index()) {
            for observation in observations {
                if let Some(retained) = member
                    .measurements
                    .iter_mut()
                    .find(|value| value.name == observation.name)
                {
                    *retained = observation;
                }
            }
        }
    }
    Ok(data)
}

struct StudyAbort<'a> {
    parent: &'a dyn AbortSignal,
    failed: AtomicBool,
}
impl AbortSignal for StudyAbort<'_> {
    fn is_aborted(&self) -> bool {
        self.failed.load(Ordering::Acquire) || self.parent.is_aborted()
    }
    fn abort_reason(&self) -> AbortReason {
        self.parent.abort_reason()
    }
    fn model_control(&self) -> Option<&ModelRunControl> {
        self.parent.model_control()
    }
    fn observe_progress(&self, fraction: f64) {
        self.parent.observe_progress(fraction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::config::{AcAnalysisConfig, AcSweepType, TransientAnalysisConfig};
    use crate::simulation::runner::{
        SpecExecutionOptions, worker_contract::WorkerSpecExecutionOptions,
    };
    use crate::simulation::{multi_run::AnalysisSpec, results::SimulationResult};
    use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

    fn base(analysis: AnalysisConfig, names: &[&str]) -> StudyRunConfig {
        StudyRunConfig {
            instance_id: AnalysisInstanceId::new(),
            source_revision: ObjectRevision::INITIAL,
            analysis_line: analysis.to_spice(),
            analysis,
            numeric_options: ".OPTIONS RELTOL=1e-5".into(),
            measurements: names.iter().map(|name| (*name).to_owned()).collect(),
            histogram_bins: 7,
        }
    }
    fn ac() -> AnalysisConfig {
        AnalysisConfig::Ac(AcAnalysisConfig {
            start_freq: 1000.0,
            stop_freq: 1000.0,
            num_points: 1,
            sweep_type: AcSweepType::Linear,
            ..Default::default()
        })
    }
    fn spec(source: McVariationSource) -> AnalysisSpec {
        AnalysisSpec::MonteCarlo {
            variation_source: source,
            params: vec![],
        }
    }

    #[test]
    fn configured_study_worker_dispatch_runs_each_selected_base_on_varied_circuits() {
        let parameter_deck = "Configured study\n.param rval=1k\nV1 in 0 DC 1 AC 1\nR1 in out {rval}\nC1 out 0 1u\n.ic V(out)=0\n.meas AC gain FIND VM(out) AT=1k\n.meas AC bounded FIND VM(out) AT=1k GOAL=1000 TOL=0.01\n.meas TRAN settled FIND V(out) AT=1m\n.mc 3 uniform 0.2 seed 37\n.end\n";
        for (analysis, names) in [
            (ac(), vec!["gain", "last:V(out)", "bounded"]),
            (
                AnalysisConfig::Transient(TransientAnalysisConfig {
                    stop_time: 1e-3,
                    step_time: 1e-5,
                    start_time: 2e-4,
                    max_timestep: Some(1e-5),
                    uic: true,
                }),
                vec!["settled", "last:V(out)"],
            ),
            (AnalysisConfig::dc_op(), vec!["scalar:V(in)"]),
        ] {
            for source in McVariationSource::ALL {
                let deck = if source == McVariationSource::DeckStatistics {
                    parameter_deck.replace(".param rval=1k", ".param rval={unif(1k,0.2)}")
                } else {
                    parameter_deck.to_owned()
                };
                let options = SpecExecutionOptions {
                    study_base: Some(base(analysis.clone(), &names)),
                    ..Default::default()
                };
                let wire = WorkerSpecExecutionOptions::from(&options);
                let encoded = serde_json::to_string(&wire).unwrap();
                let decoded: WorkerSpecExecutionOptions = serde_json::from_str(&encoded).unwrap();
                assert_eq!(wire, decoded);
                let result = super::super::spec::run_spec_request_with_environment(
                    &EngineBridge::new(),
                    spec(source),
                    decoded.into(),
                    &deck,
                    None,
                    &crate::simulation::execution::ResolvedExecutionDependencies::default(),
                    Some(AnalysisExecutionEnvironment {
                        temperature_celsius: 75.0,
                        supply_voltage: Some(1.8),
                        nominal_supply_voltage: Some(1.0),
                        supply_source_names: vec!["V1".into()],
                    }),
                    &NoAbort,
                )
                .unwrap();
                let SimulationResult::MonteCarlo {
                    variables,
                    runs_completed,
                    num_failures,
                    member_measurements,
                    ..
                } = result
                else {
                    panic!("MC result")
                };
                assert_eq!(runs_completed, 3);
                assert_eq!(num_failures, 0);
                assert_eq!(variables.len(), names.len());
                assert_eq!(member_measurements.len(), 3);
                crate::state::FamilyMemberMeasurements::validate_monte_carlo_sequence(
                    &member_measurements,
                    37,
                    3,
                    3,
                    0,
                    variables
                        .iter()
                        .map(|variable| (variable.name.as_str(), variable.samples.as_slice())),
                )
                .unwrap();
                if matches!(analysis, AnalysisConfig::Ac(_)) {
                    let bounded = variables
                        .iter()
                        .find(|variable| variable.name == "bounded")
                        .unwrap();
                    let gain = variables
                        .iter()
                        .find(|variable| variable.name == "gain")
                        .unwrap();
                    assert_eq!(bounded.samples, gain.samples);
                    for trial in &member_measurements {
                        let measured = trial
                            .measurements
                            .iter()
                            .find(|measurement| measurement.name == "bounded")
                            .unwrap();
                        assert!(measured.value.is_some());
                        assert!(!measured.passed);
                        assert!(
                            measured
                                .error
                                .as_ref()
                                .is_some_and(|error| error.contains("GOAL"))
                        );
                    }
                }
                for variable in &variables {
                    assert_eq!(
                        variable.histogram.len(),
                        if variable.min == variable.max { 1 } else { 7 }
                    );
                    assert_eq!(variable.samples.len(), 3);
                    assert!(variable.mean_confidence.is_some());
                }
                let first = variables
                    .iter()
                    .find(|variable| variable.name == names[0])
                    .unwrap();
                match &analysis {
                    AnalysisConfig::DcOp(_) => assert!(
                        first
                            .samples
                            .iter()
                            .all(|value| (*value - 1.8).abs() < 1e-10)
                    ),
                    AnalysisConfig::Ac(_) => {
                        assert!(first.samples.windows(2).any(|pair| pair[0] != pair[1]));
                        let real = variables
                            .iter()
                            .find(|variable| variable.name == "last:V(out)")
                            .unwrap();
                        for (magnitude, real) in first.samples.iter().zip(&real.samples) {
                            assert!((magnitude * magnitude - real).abs() < 1e-10);
                            assert!(*magnitude > 0.12 && *magnitude < 0.21);
                        }
                    }
                    AnalysisConfig::Transient(_) => {
                        assert!(first.samples.windows(2).any(|pair| pair[0] != pair[1]));
                        let last = variables
                            .iter()
                            .find(|variable| variable.name == "last:V(out)")
                            .unwrap();
                        for (measured, last) in first.samples.iter().zip(&last.samples) {
                            assert!((measured - last).abs() < 1e-7);
                            assert!(*measured > 0.9 && *measured < 1.4);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    #[test]
    fn configured_study_rejects_missing_measurements_and_preserves_abort_and_limits() {
        let deck =
            "Study\n.param r=1k\nV1 in 0 AC 1\nR1 in 0 {r}\n.mc 2 uniform 0.01 seed 7\n.end\n";
        let mut configured = base(ac(), &["absent"]);
        let error = run_monte_carlo(
            &configured,
            McVariationSource::ParameterTolerance,
            deck,
            None,
            None,
            &NoAbort,
        )
        .unwrap_err();
        assert!(
            matches!(error, SimulationError::InvalidConfig(ref message) if message.contains("absent")),
            "{error}"
        );
        configured.measurements = vec!["last:V(in)".into()];
        configured.histogram_bins = rspice_core::SimulationConfig::default()
            .resource_limits
            .max_analysis_points
            + 1;
        assert!(matches!(
            run_monte_carlo(
                &configured,
                McVariationSource::ParameterTolerance,
                deck,
                None,
                None,
                &NoAbort
            ),
            Err(SimulationError::ResourceLimit { .. })
        ));
        assert!(matches!(
            run_monte_carlo(
                &configured,
                McVariationSource::ParameterTolerance,
                deck,
                None,
                None,
                &ImmediateAbort
            ),
            Err(SimulationError::Aborted)
        ));
        configured.histogram_bins = 7;
        configured.measurements = vec!["in".into()];
        let failed = deck.replace(".end", ".meas AC in FIND VM(in) AT=9k\n.end");
        let error = run_monte_carlo(
            &configured,
            McVariationSource::ParameterTolerance,
            &failed,
            None,
            None,
            &NoAbort,
        )
        .unwrap_err();
        assert!(
            matches!(error, SimulationError::CircuitError(ref message) if message.contains("All 2") && message.contains("in")),
            "{error}"
        );
    }
}
