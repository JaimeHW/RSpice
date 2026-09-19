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

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::state::{MonteCarloMeanInterval, MonteCarloMeanMethod};

    #[test]
    fn both_monte_carlo_sources_apply_confidence_without_changing_trials() {
        type Run = fn(&str) -> ServiceRunResult<MonteCarloData>;
        fn parameters(deck: &str) -> ServiceRunResult<MonteCarloData> {
            run_monte_carlo_analysis_with_abort(deck, &NoAbort)
        }
        fn statistics(deck: &str) -> ServiceRunResult<MonteCarloData> {
            run_statistical_monte_carlo_with_source_path_and_abort(deck, None, &NoAbort)
        }
        for (run, parameter) in [
            (parameters as Run, "1000"),
            (statistics as Run, "agauss(1000,100,1)"),
        ] {
            let deck = |options: &str| {
                format!(
                    "MC confidence\n.param rload={parameter}\nV1 in 0 1\nR1 in out {{rload}}\nR2 out 0 1k\n.mc 16 uniform 0.1 seed 17 {options}\n.end\n"
                )
            };
            let default = run(&deck("")).unwrap();
            let narrower = run(&deck("CONFIDENCE 80")).unwrap();
            let bootstrap = run(&deck(
                "CONFIDENCE 90 CI BOOTSTRAP RESAMPLES 257 BOOTSEED 18446744073709551615",
            ))
            .unwrap();
            let repeated = run(&deck(
                "CONFIDENCE 90 CI BOOTSTRAP RESAMPLES 257 BOOTSEED 18446744073709551615",
            ))
            .unwrap();
            let find = |data: MonteCarloData| {
                data.variables
                    .into_iter()
                    .find(|v| v.name == "V(OUT)")
                    .unwrap()
            };
            let a = find(default);
            let b = find(narrower);
            let c = find(bootstrap);
            let d = find(repeated);
            assert_eq!(a.samples, b.samples);
            assert_eq!(a.samples, c.samples);
            let MonteCarloMeanInterval::Available {
                lower: low95,
                upper: high95,
            } = a.mean_confidence.unwrap().interval
            else {
                panic!("limits")
            };
            let b = b.mean_confidence.unwrap();
            let MonteCarloMeanInterval::Available {
                lower: low80,
                upper: high80,
            } = b.interval
            else {
                panic!("limits")
            };
            assert_eq!(b.level_pct, 80.0);
            assert!(low80 > low95 && high80 < high95);
            assert_eq!(c.mean_confidence, d.mean_confidence);
            let retained = c.mean_confidence.unwrap();
            assert_eq!(retained.level_pct, 90.0);
            assert_eq!(
                retained.method,
                MonteCarloMeanMethod::PercentileBootstrap {
                    resamples: 257,
                    seed: u64::MAX
                }
            );
            assert_eq!(retained.successful_samples, 16);
        }
    }
}
