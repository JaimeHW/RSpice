//! Dispatch for swept and statistical analyses.

use std::collections::HashMap;
use std::path::Path;

use rspice_core::abort_signal::AbortSignal;

use crate::services::simulation_runner as svc_runner;
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::results::{MonteCarloVariableResult, SimulationResult, WaveformData};
use crate::simulation::runner::{
    AnalysisExecutionEnvironment, SimulationError, SpecExecutionOptions,
};

pub(super) fn run_sweep_spec(
    spec: AnalysisSpec,
    options: SpecExecutionOptions,
    netlist: &str,
    source_path: Option<&Path>,
    environment: Option<AnalysisExecutionEnvironment>,
    abort: &dyn AbortSignal,
    checkpoint_observer: Option<&(dyn Fn(&[u8]) -> Result<(), SimulationError> + Sync)>,
) -> Result<SimulationResult, SimulationError> {
    super::ensure_not_aborted(abort)?;
    match spec {
        // The varied subset reaches the engine on the `.MC` card's `PARAMS`
        // list, which this deck already carries, so the specification's copy
        // is read for identity rather than re-applied here.
        AnalysisSpec::MonteCarlo {
            variation_source, ..
        } => {
            let augmented = crate::simulation::runner::study::monte_carlo::source_with_statistics(
                netlist,
                variation_source,
                options.mc_statistics.as_ref(),
            )?;
            let netlist = augmented.as_ref();
            run_monte_carlo(
                variation_source,
                options.study_base.as_ref(),
                options.mc_checkpoint.as_ref(),
                options.mc_histogram_bins.unwrap_or(20),
                checkpoint_observer,
                netlist,
                source_path,
                environment,
                abort,
            )
        }
        AnalysisSpec::Parametric => run_parametric(netlist, options, source_path, abort),
        // A corner declaration is expanded into one task per declared point
        // before the run is authorized, and its plotting family is assembled
        // from those results. Nothing solves the declaration itself, so a
        // corner spec arriving here is a routing fault, not a request.
        other => Err(super::misrouted_spec_error("sweep", &other)),
    }
}

fn run_monte_carlo(
    variation_source: crate::simulation::dialog::McVariationSource,
    base: Option<&crate::simulation::runner::study::StudyRunConfig>,
    checkpoint: Option<
        &crate::simulation::runner::monte_carlo_checkpoint::MonteCarloCheckpointRequest,
    >,
    histogram_bins: usize,
    checkpoint_observer: Option<&(dyn Fn(&[u8]) -> Result<(), SimulationError> + Sync)>,
    netlist: &str,
    source_path: Option<&Path>,
    environment: Option<AnalysisExecutionEnvironment>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    use crate::simulation::runner::study::monte_carlo::{self, MonteCarloContinuation};
    let run = |continuation| match base {
        Some(base) => monte_carlo::run_monte_carlo_with_continuation(
            base,
            variation_source,
            netlist,
            source_path,
            environment.clone(),
            abort,
            continuation,
        ),
        None => monte_carlo::voltages::run(
            netlist,
            source_path,
            variation_source,
            histogram_bins,
            environment.clone(),
            abort,
            continuation,
        ),
    };
    let data = if let Some(request) = checkpoint {
        let observer = checkpoint_observer.ok_or_else(|| {
            SimulationError::InvalidConfig("Monte Carlo checkpoint destination is missing".into())
        })?;
        let mut retained = request
            .resume
            .as_ref()
            .map(|input| input.decode())
            .transpose()?;
        let publish = |value: &monte_carlo::checkpoint::StudyMonteCarloCheckpoint| {
            let bytes = value.to_bytes_with_limits(
                rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )?;
            observer(&bytes)
        };
        run(Some(MonteCarloContinuation {
            checkpoint: &mut retained,
            trial_range: request.trial_range.clone(),
            publish_every: request.publish_every,
            publish: &publish,
        }))?
    } else {
        run(None)?
    };
    let mut variables = Vec::with_capacity(data.variables.len());
    for variable in data.variables {
        super::ensure_not_aborted(abort)?;
        variables.push(MonteCarloVariableResult {
            mean_confidence: variable.mean_confidence,
            name: variable.name,
            samples: variable.samples,
            mean: variable.mean,
            std_dev: variable.std_dev,
            min: variable.min,
            max: variable.max,
            histogram: variable.histogram,
            bin_edges: variable.bin_edges,
        });
    }

    Ok(SimulationResult::MonteCarlo {
        seed: data.seed,
        runs_requested: data.runs_requested,
        runs_completed: data.runs_completed,
        num_failures: data.num_failures,
        all_converged: data.all_converged,
        variables,
        member_measurements: data.trial_measurements,
    })
}

/// Step a design parameter through the `.STEP` command the deck declares.
///
/// A temperature step is not this. It declares a PVT axis, expands into one
/// task per temperature before the run is authorized, and has its family
/// assembled from those results — so a temperature contract arriving here means
/// a declaration reached an executor instead of being expanded.
fn run_parametric(
    netlist: &str,
    options: SpecExecutionOptions,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    if options.temp.is_some() {
        return Err(SimulationError::InvalidConfig(
            "sweep runner received a temperature step, which is solved one declared point at a time"
                .to_owned(),
        ));
    }
    let data = super::run_abort_aware_service(abort, || {
        if let Some(base_mode) = options.parametric_base.as_ref() {
            svc_runner::run_parametric_analysis_with_base_and_source_path_and_abort(
                netlist,
                source_path,
                base_mode,
                abort,
            )
        } else {
            svc_runner::run_parametric_analysis_with_source_path_and_abort(
                netlist,
                source_path,
                abort,
            )
        }
    })?;
    let sweep_values = data.sweep_values;
    let member_measurements = point_measurements(&sweep_values, &data.voltages);
    let mut waveforms = HashMap::with_capacity(data.voltages.len());
    for (name, values) in data.voltages {
        super::ensure_not_aborted(abort)?;
        waveforms.insert(
            name.clone(),
            WaveformData::new_time_domain(name, sweep_values.clone(), values),
        );
    }

    Ok(SimulationResult::Parametric {
        target: data.target,
        sweep_values,
        waveforms,
        num_failures: data.num_failures,
        member_measurements,
    })
}

/// What each swept point measured, attributed to the point that measured it.
///
/// The runner already solved every point and reduced each to one terminal
/// value per traced quantity; those values arrive here transposed — one series
/// per quantity, one entry per point. Reading them back per point costs nothing
/// beyond the transpose and is the difference between a limit judged against
/// the sweep and a limit judged against whichever point happened to be last.
///
/// A quantity whose series is shorter than the sweep is skipped for the points
/// it does not reach rather than padded: a point that produced no value for a
/// name did not measure it, and inventing one would put a fabricated sample
/// into a yield.
fn point_measurements(
    sweep_values: &[f64],
    voltages: &[(String, Vec<f64>)],
) -> Vec<crate::state::FamilyMemberMeasurements> {
    use crate::state::{FamilyMeasurementEvidence, FamilyMemberId, FamilyMemberMeasurements};

    sweep_values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let measurements = voltages
                .iter()
                .filter_map(|(name, series)| {
                    let measured = series.get(index).copied()?;
                    Some(FamilyMeasurementEvidence {
                        unit: Some(rspice_core::analysis::MeasurementUnit::Known("V".into())),
                        name: name.clone(),
                        value: measured.is_finite().then_some(measured),
                        passed: measured.is_finite(),
                        error: (!measured.is_finite())
                            .then(|| "sweep point produced a non-finite value".to_owned()),
                    })
                })
                .collect();
            FamilyMemberMeasurements::new(
                FamilyMemberId::SweepPoint {
                    index,
                    value: *value,
                },
                measurements,
            )
        })
        .collect()
}
