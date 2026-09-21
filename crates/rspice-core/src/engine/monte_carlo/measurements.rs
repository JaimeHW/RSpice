//! Named observations of arbitrary analyses on the engine's trial circuits.

use super::{MonteCarloEnvironment, MonteCarloRunConfig, MonteCarloVariationSource};
use crate::abort_signal::AbortSignal;
use crate::analysis::monte_carlo::{
    Distribution, MeanConfidenceMethod, MonteCarloResult, VariableStatistics,
    validate_mean_confidence_request,
};
use crate::engine::{Engine, SimulationError};
use crate::{Netlist, Value};
use std::collections::HashSet;

/// Sampling and reporting for a configured analysis evaluated once per trial.
/// The engine's resource limits apply to the batch and each analysis invocation.
#[derive(Debug, Clone)]
pub struct MonteCarloStudyConfig {
    pub first_trial: usize,
    pub num_runs: usize,
    pub seed: u64,
    pub distribution: Distribution,
    pub variation_source: MonteCarloVariationSource,
    /// Empty selects every eligible generic parameter. Native Spectre
    /// statistics instead use the variations declared by the circuit.
    pub parameter_filter: Vec<String>,
    pub environment: Option<MonteCarloEnvironment>,
    /// Ordered identities corresponding to the evaluator's scalar outputs.
    /// These are measurement names, without voltage aliases or implicit units.
    pub measurements: Vec<String>,
    pub histogram_bins: usize,
    pub confidence_pct: Value,
    pub confidence_method: MeanConfidenceMethod,
}

impl MonteCarloStudyConfig {
    pub fn new(num_runs: usize, seed: u64, measurements: Vec<String>) -> Self {
        Self {
            first_trial: 0,
            num_runs,
            seed,
            distribution: Distribution::Gaussian { sigma: 0.01 },
            variation_source: MonteCarloVariationSource::ParameterTolerance,
            parameter_filter: Vec::new(),
            environment: None,
            measurements,
            histogram_bins: 20,
            confidence_pct: 95.0,
            confidence_method: MeanConfidenceMethod::StudentT,
        }
    }
}

impl Engine {
    /// Evaluate configured analyses and their measurements on each sampled circuit.
    ///
    /// The callback receives the bounded worker engine, materialized trial circuit,
    /// original zero-based trial index, and cancellation signal. It must run all
    /// prerequisites against this same circuit, preserving its statistical
    /// coordinate and environment; reparsing the nominal source would lose them.
    /// Return one finite scalar per configured measurement, in the declared order.
    /// A solve error, missing value, or nonfinite value fails the entire trial so
    /// every retained measurement has the same population and trial identities.
    /// Cancellation, deadlines, configuration errors, and resource limits stop the
    /// batch rather than becoming statistical failures.
    pub fn run_monte_carlo_measurements_with_abort<F>(
        &self,
        netlist: &Netlist,
        study: &MonteCarloStudyConfig,
        abort: &dyn AbortSignal,
        evaluate: F,
    ) -> Result<MonteCarloResult, SimulationError>
    where
        F: Fn(&Engine, &Netlist, usize, &dyn AbortSignal) -> Result<Vec<Value>, SimulationError>
            + Sync,
    {
        self.run_monte_carlo_measurements_journaled_with_abort(
            netlist, study, abort, None, evaluate,
        )
    }

    pub(super) fn run_monte_carlo_measurements_journaled_with_abort<F>(
        &self,
        netlist: &Netlist,
        study: &MonteCarloStudyConfig,
        abort: &dyn AbortSignal,
        journal: Option<super::MonteCarloTrialJournal<'_, Vec<Value>>>,
        evaluate: F,
    ) -> Result<MonteCarloResult, SimulationError>
    where
        F: Fn(&Engine, &Netlist, usize, &dyn AbortSignal) -> Result<Vec<Value>, SimulationError>
            + Sync,
    {
        if abort.is_aborted() {
            return Err(SimulationError::from_abort(abort));
        }
        self.ensure_valid_configuration()?;
        self.ensure_batch_runs(study.num_runs)?;
        if study.histogram_bins == 0 || study.measurements.is_empty() {
            return Err(SimulationError::Circuit(
                "Monte Carlo measurement studies require measurements and positive histogram bins"
                    .into(),
            ));
        }
        self.ensure_analysis_points(study.histogram_bins)?;
        let mut names = HashSet::new();
        for name in &study.measurements {
            if abort.is_aborted() {
                return Err(SimulationError::from_abort(abort));
            }
            if name.is_empty()
                || name.trim() != name
                || name.chars().any(char::is_control)
                || !names.insert(name.to_ascii_uppercase())
            {
                return Err(SimulationError::Circuit(format!(
                    "Monte Carlo measurement identity {name:?} is empty, malformed, or duplicated"
                )));
            }
        }
        validate_mean_confidence_request(
            study.confidence_pct,
            study.confidence_method,
            study.num_runs,
            study.measurements.len(),
            self.config.resource_limits,
        )?;
        // Reserve for trial rows, transposed samples, histogram edges/counts,
        // summary/interval scalars, and original successful-trial identities
        // before any solver callback can allocate the batch's retained outputs.
        let per_measurement = study
            .num_runs
            .saturating_mul(2)
            .saturating_add(study.histogram_bins.saturating_mul(2))
            .saturating_add(8);
        let retained = per_measurement
            .saturating_mul(study.measurements.len())
            .saturating_add(study.num_runs);
        let confidence_scratch = match study.confidence_method {
            MeanConfidenceMethod::StudentT => 0,
            MeanConfidenceMethod::PercentileBootstrap { resamples, .. } => resamples,
        };
        self.ensure_result_values(retained.saturating_add(confidence_scratch))?;
        let options = MonteCarloRunConfig {
            first_trial: study.first_trial,
            num_runs: study.num_runs,
            seed: study.seed,
            distribution: study.distribution,
            variation_source: study.variation_source,
            parameter_filter: Some(&study.parameter_filter),
            environment: study.environment.as_ref(),
        };
        let (outcomes, sampling) = self.run_monte_carlo_trials_journaled_with_abort(
            netlist,
            &options,
            abort,
            journal,
            |engine, trial, index, abort| {
                let values = evaluate(engine, trial, index, abort)?;
                if values.len() != study.measurements.len()
                    || values.iter().any(|value| !value.is_finite())
                {
                    return Err(SimulationError::Circuit(format!(
                        "Monte Carlo trial {} did not produce every requested finite measurement",
                        index + 1
                    )));
                }
                Ok(values)
            },
        )?;
        let successful_trial_indices: Vec<_> = outcomes
            .iter()
            .enumerate()
            .filter_map(|(index, result)| result.as_ref().map(|_| study.first_trial + index))
            .collect();
        let successes = successful_trial_indices.len();
        let mut result = MonteCarloResult {
            successful_trial_indices: Some(successful_trial_indices),
            num_runs: study.num_runs,
            variables: Default::default(),
            all_converged: successes == study.num_runs,
            num_failures: study.num_runs - successes,
            sampling: Some(sampling),
            confidence: None,
        };
        if successes > 0 {
            for (column, name) in study.measurements.iter().enumerate() {
                let mut samples = Vec::with_capacity(successes);
                for outcome in &outcomes {
                    if abort.is_aborted() {
                        return Err(SimulationError::from_abort(abort));
                    }
                    if let Some(values) = outcome {
                        samples.push(values[column]);
                    }
                }
                result.variables.insert(
                    name.clone(),
                    VariableStatistics::from_samples(name, samples, study.histogram_bins),
                );
            }
        }
        drop(outcomes);
        result.compute_mean_confidence(
            study.confidence_pct,
            study.confidence_method,
            self.config.resource_limits,
            abort,
        )?;
        Ok(result)
    }
}
