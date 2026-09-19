//! Preserve the engine's mean-confidence evidence at the Studio boundary.

use crate::state::{MonteCarloMeanConfidence, MonteCarloMeanInterval, MonteCarloMeanMethod};
use rspice_core::analysis::monte_carlo::{
    MeanConfidenceInterval, MeanConfidenceMethod, MonteCarloConfidence,
};

pub(super) fn retain(
    population: Option<MonteCarloConfidence>,
    interval: Option<MeanConfidenceInterval>,
) -> Option<MonteCarloMeanConfidence> {
    let population = population?;
    let interval = match interval? {
        MeanConfidenceInterval::Available { lower, upper } => {
            MonteCarloMeanInterval::Available { lower, upper }
        }
        MeanConfidenceInterval::InsufficientSamples => MonteCarloMeanInterval::InsufficientSamples,
        MeanConfidenceInterval::Unrepresentable => MonteCarloMeanInterval::Unrepresentable,
    };
    Some(MonteCarloMeanConfidence {
        level_pct: population.level_pct,
        method: match population.method {
            MeanConfidenceMethod::StudentT => MonteCarloMeanMethod::StudentT,
            MeanConfidenceMethod::PercentileBootstrap { resamples, seed } => {
                MonteCarloMeanMethod::PercentileBootstrap { resamples, seed }
            }
        },
        successful_samples: population.successful_samples,
        conditional_on_successful_trials: population.conditional_on_successful_trials,
        interval,
    })
}
