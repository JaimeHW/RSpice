//! Configured Monte Carlo execution and lossless trial continuation.

use super::*;
use crate::simulation::runner::spec;
use checkpoint::StudyMonteCarloCheckpoint;
use rspice_core::analysis::monte_carlo::Distribution;
use rspice_core::engine::{MonteCarloStudyConfig, MonteCarloVariationSource};
use rspice_core::netlist::{AnalysisCommand, MonteCarloDistribution};
use std::sync::atomic::AtomicUsize;

pub(crate) mod checkpoint;
pub(crate) mod voltages;

/// A continuation always evaluates the same frozen source. An explicit range
/// changes only which original trial indices contribute to this result; rows
/// outside it remain available in the journal. The callback receives complete,
/// bounded snapshots and may persist or forward their portable bytes.
pub(crate) struct MonteCarloContinuation<'a> {
    pub checkpoint: &'a mut Option<StudyMonteCarloCheckpoint>,
    pub trial_range: Option<std::ops::Range<usize>>,
    pub publish_every: std::num::NonZeroUsize,
    pub publish: &'a (dyn Fn(&StudyMonteCarloCheckpoint) -> Result<(), SimulationError> + Sync),
}

#[allow(dead_code, reason = "retained Monte Carlo study adapter used by tests")]
pub(crate) fn run_monte_carlo(
    base: &StudyRunConfig,
    variation_source: McVariationSource,
    source: &str,
    source_path: Option<&Path>,
    environment: Option<AnalysisExecutionEnvironment>,
    abort: &dyn AbortSignal,
) -> Result<services::MonteCarloData, SimulationError> {
    run_monte_carlo_with_continuation(
        base,
        variation_source,
        source,
        source_path,
        environment,
        abort,
        None,
    )
}

pub(crate) fn run_monte_carlo_with_continuation(
    base: &StudyRunConfig,
    variation_source: McVariationSource,
    source: &str,
    source_path: Option<&Path>,
    environment: Option<AnalysisExecutionEnvironment>,
    abort: &dyn AbortSignal,
    continuation: Option<MonteCarloContinuation<'_>>,
) -> Result<services::MonteCarloData, SimulationError> {
    let PreparedStudy {
        analysis,
        circuit,
        study,
        engine,
    } = prepare_study(
        base,
        variation_source,
        source,
        source_path,
        environment,
        abort,
    )?;
    let evaluation_identity =
        *crate::simulation::execution::monte_carlo_evaluator_digest(base).as_bytes();
    run_prepared(
        circuit,
        study,
        engine,
        evaluation_identity,
        abort,
        continuation,
        |engine, trial, abort| {
            let result = base.run_trial(engine, &analysis, trial, abort)?;
            base.measurements
                .iter()
                .map(|name| {
                    result.study_measurement(name).ok_or_else(|| {
                        SimulationError::CircuitError(format!(
                            "Study measurement {name:?} is unavailable or failed"
                        ))
                    })
                })
                .collect()
        },
    )
}

fn run_prepared<F>(
    circuit: rspice_core::Netlist,
    mut study: MonteCarloStudyConfig,
    engine: rspice_core::Engine,
    evaluation_identity: [u8; 32],
    abort: &dyn AbortSignal,
    continuation: Option<MonteCarloContinuation<'_>>,
    evaluate_trial: F,
) -> Result<services::MonteCarloData, SimulationError>
where
    F: Fn(
            &rspice_core::Engine,
            &rspice_core::Netlist,
            &dyn AbortSignal,
        ) -> Result<Vec<crate::state::FamilyMeasurementEvidence>, SimulationError>
        + Sync,
{
    let bridge = EngineBridge::new();
    if let Some(range) = continuation
        .as_ref()
        .and_then(|value| value.trial_range.as_ref())
    {
        if range.is_empty() {
            return Err(SimulationError::InvalidConfig(
                "Monte Carlo continuation requires a nonempty trial range".into(),
            ));
        }
        study.first_trial = range.start;
        study.num_runs = range.end - range.start;
    }
    let signal = StudyAbort {
        parent: abort,
        failed: AtomicBool::new(false),
    };
    let fatal = Mutex::new(None);
    let first_trial_failure = Mutex::new(None);
    let limits = engine.config().resource_limits;
    let mut numerical = if let Some(continuation) = &continuation {
        if let Some(checkpoint) = continuation.checkpoint.as_ref() {
            checkpoint.validate(limits, abort)?;
            checkpoint.validate_for_resume()?;
            let expected = engine
                .new_monte_carlo_checkpoint(&circuit, &study, evaluation_identity, abort)
                .map_err(|error| bridge.translate_error(error))?;
            if checkpoint.population_identity() != expected.population_identity() {
                return Err(SimulationError::InvalidConfig(
                    "Monte Carlo checkpoint does not match the frozen study population".into(),
                ));
            }
            Some(checkpoint.numerical.clone())
        } else {
            Some(
                engine
                    .new_monte_carlo_checkpoint(&circuit, &study, evaluation_identity, abort)
                    .map_err(|error| bridge.translate_error(error))?,
            )
        }
    } else {
        None
    };
    let measurement_verdicts = Mutex::new(
        continuation
            .as_ref()
            .and_then(|value| value.checkpoint.as_ref())
            .map(|value| value.observations.clone())
            .unwrap_or_default(),
    );
    let retaining = continuation.is_some();
    let evaluate = |engine: &rspice_core::Engine,
                    trial: &rspice_core::Netlist,
                    trial_index: usize,
                    abort: &dyn AbortSignal| {
        let observations = evaluate_trial(engine, trial, abort).map_err(|error| match error {
            SimulationError::SolverError(_)
            | SimulationError::ConvergenceFailed { .. }
            | SimulationError::Attributed { .. }
            | SimulationError::CircuitError(_) => {
                let message = error.to_string();
                first_trial_failure
                    .lock()
                    .unwrap()
                    .get_or_insert_with(|| message.clone());
                if retaining {
                    measurement_verdicts.lock().unwrap().insert(
                        trial_index,
                        checkpoint::failed_observations(&study.measurements, &message),
                    );
                }
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
        for observation in observations {
            values.push(observation.value.expect("observed study value"));
            // The numerical summary cannot reconstruct physical units. Keep
            // typed successful observations even when no checkpoint is requested.
            if retaining || !observation.passed || observation.unit.is_some() {
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
    };
    let result = if let Some(continuation) = continuation {
        let numerical = numerical.as_mut().expect("checkpoint request");
        let initial = numerical.completed_trials();
        let last_published = AtomicUsize::new(initial);
        let publication_failed = AtomicBool::new(false);
        let capture = |numerical: &rspice_core::engine::MonteCarloCheckpoint| {
            // Once accepted, a trial remains durable even if cancellation has
            // just arrived. Capture is bounded by the already checked limits.
            StudyMonteCarloCheckpoint::capture(
                numerical,
                &study.measurements,
                &measurement_verdicts.lock().unwrap(),
                limits,
                &rspice_core::NoAbort,
            )
        };
        let result = engine.run_monte_carlo_measurements_checkpointed_with_abort(
            &circuit,
            &study,
            evaluation_identity,
            numerical,
            &signal,
            evaluate,
            |numerical| {
                if numerical
                    .completed_trials()
                    .saturating_sub(last_published.load(Ordering::Relaxed))
                    < continuation.publish_every.get()
                {
                    return Ok(());
                }
                match capture(numerical).and_then(|value| (continuation.publish)(&value)) {
                    Ok(()) => {
                        last_published.store(numerical.completed_trials(), Ordering::Relaxed);
                        Ok(())
                    }
                    Err(error) => {
                        publication_failed.store(true, Ordering::Release);
                        fatal.lock().unwrap().get_or_insert(error);
                        signal.failed.store(true, Ordering::Release);
                        Err(rspice_core::SimulationError::Aborted)
                    }
                }
            },
        );
        let checkpoint = capture(numerical)?;
        // Update the caller-owned journal before returning a terminal error.
        // Do not retry a failed publication behind the consumer's back.
        // A fully cached run still needs to deliver its retained journal to
        // this run's result; no newly evaluated trial means no cadence callback.
        *continuation.checkpoint = Some(checkpoint);
        if !publication_failed.load(Ordering::Acquire)
            && numerical.completed_trials() > 0
            && (numerical.completed_trials() > last_published.load(Ordering::Relaxed)
                || last_published.load(Ordering::Relaxed) == initial)
            && let Err(error) =
                (continuation.publish)(continuation.checkpoint.as_ref().expect("captured"))
        {
            fatal.lock().unwrap().get_or_insert(error);
        }
        result
    } else {
        engine.run_monte_carlo_measurements_with_abort(&circuit, &study, &signal, evaluate)
    };
    if let Some(error) = fatal.into_inner().unwrap() {
        return Err(error);
    }
    spec::ensure_not_aborted(abort)?;
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
    let mut data = spec::run_abort_aware_service(abort, || {
        services::finish_monte_carlo_result(
            result,
            engine.config().resource_limits.max_result_values,
            abort,
        )
    })?;
    let mut verdicts = measurement_verdicts.into_inner().unwrap();
    for member in &mut data.trial_measurements {
        spec::ensure_not_aborted(abort)?;
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

#[cfg(test)]
mod tests;

struct PreparedStudy {
    analysis: StudyAnalysis,
    circuit: rspice_core::Netlist,
    study: MonteCarloStudyConfig,
    engine: rspice_core::Engine,
}

fn prepare_study(
    base: &StudyRunConfig,
    variation_source: McVariationSource,
    source: &str,
    source_path: Option<&Path>,
    environment: Option<AnalysisExecutionEnvironment>,
    abort: &dyn AbortSignal,
) -> Result<PreparedStudy, SimulationError> {
    spec::ensure_not_aborted(abort)?;
    if !base.objective_terms.is_empty() || !base.constraints.is_empty() {
        return Err(SimulationError::InvalidConfig(
            "Objectives and constraints apply only to optimization".into(),
        ));
    }
    validate_measurements(&base.measurements).map_err(SimulationError::InvalidConfig)?;
    base.analysis
        .validate()
        .map_err(|errors| SimulationError::InvalidConfig(errors.join("; ")))?;
    // Preserve these cards in the parser's retained source, so expression and
    // native-statistics replay observes the same analysis and solver policy.
    let (analysis, environment) = resolved_study_environment(base, environment);
    let source = study_source_at_environment(base, source, environment.as_ref(), abort)?;
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
    validate_base_measurements(base, &circuit)?;
    let mut study = MonteCarloStudyConfig::new(
        command.runs,
        command.seed.unwrap_or(0x5EED_5EED),
        base.measurements.clone(),
    );
    study.first_trial = command.first_trial;
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
    study.environment = environment;
    let engine = rspice_core::Engine::default().resolved_for_netlist(&circuit);
    Ok(PreparedStudy {
        analysis,
        circuit,
        study,
        engine,
    })
}

/// The same population contract the runner checks, without executing a trial.
/// Prepared decks already seal include contents and therefore have no source path.
pub(crate) fn prepared_population_identity(
    base: Option<&StudyRunConfig>,
    histogram_bins: usize,
    variation_source: McVariationSource,
    statistics: Option<&crate::simulation::dialog::mc::statistics::McStatisticsConfig>,
    source: &str,
    environment: Option<AnalysisExecutionEnvironment>,
) -> Result<[u8; 32], SimulationError> {
    let source = source_with_statistics(source, variation_source, statistics)?;
    let Some(base) = base else {
        return voltages::population_identity(
            &source,
            variation_source,
            histogram_bins,
            environment,
        );
    };
    let prepared = prepare_study(
        base,
        variation_source,
        &source,
        None,
        environment,
        &rspice_core::NoAbort,
    )?;
    let evaluation = *crate::simulation::execution::monte_carlo_evaluator_digest(base).as_bytes();
    prepared
        .engine
        .new_monte_carlo_checkpoint(
            &prepared.circuit,
            &prepared.study,
            evaluation,
            &rspice_core::NoAbort,
        )
        .map(|checkpoint| checkpoint.population_identity())
        .map_err(|error| EngineBridge::new().translate_error(error))
}

pub(crate) fn source_with_statistics<'a>(
    source: &'a str,
    variation_source: McVariationSource,
    statistics: Option<&crate::simulation::dialog::mc::statistics::McStatisticsConfig>,
) -> Result<std::borrow::Cow<'a, str>, SimulationError> {
    let Some(statistics) = statistics else {
        return Ok(source.into());
    };
    if variation_source != McVariationSource::DeckStatistics {
        return Err(SimulationError::InvalidConfig(
            "Custom statistics require the native statistics sampler".into(),
        ));
    }
    let directive = statistics
        .parser_directive()
        .map_err(SimulationError::InvalidConfig)?;
    Ok(services::splice_before_terminal_end_card(source, &directive).into())
}
