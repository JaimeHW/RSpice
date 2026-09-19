//! Retained uncertainty in a Monte Carlo population mean.
//!
//! These intervals describe independent successful trials. They are distinct
//! from population percentiles and from the confidence limits on yield.

/// The estimator and, for resampling, its independent random stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "method", rename_all = "snake_case", deny_unknown_fields)]
pub enum MonteCarloMeanMethod {
    StudentT,
    PercentileBootstrap { resamples: usize, seed: u64 },
}

/// A finite two-sided interval, or a retained reason it was unavailable.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum MonteCarloMeanInterval {
    Available { lower: f64, upper: f64 },
    InsufficientSamples,
    Unrepresentable,
}

/// The complete authority behind one variable's mean interval.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MonteCarloMeanConfidence {
    pub level_pct: f64,
    pub method: MonteCarloMeanMethod,
    pub successful_samples: usize,
    pub conditional_on_successful_trials: bool,
    pub interval: MonteCarloMeanInterval,
}

impl MonteCarloMeanConfidence {
    pub(crate) fn validate(&self, samples: usize, failures: usize) -> Result<(), String> {
        if !self.level_pct.is_finite() || self.level_pct <= 0.0 || self.level_pct >= 100.0 {
            return Err(
                "mean confidence level must be finite and between 0 and 100 percent".into(),
            );
        }
        if self.successful_samples != samples
            || self.conditional_on_successful_trials != (failures != 0)
        {
            return Err("mean confidence population disagrees with the retained trials".into());
        }
        if matches!(self.method, MonteCarloMeanMethod::PercentileBootstrap { resamples, .. } if resamples < 2)
        {
            return Err("mean bootstrap confidence requires at least two resamples".into());
        }
        match self.interval {
            MonteCarloMeanInterval::Available { lower, upper }
                if samples >= 2 && lower.is_finite() && upper.is_finite() && lower <= upper =>
            {
                Ok(())
            }
            MonteCarloMeanInterval::InsufficientSamples if samples < 2 => Ok(()),
            MonteCarloMeanInterval::Unrepresentable if samples >= 2 => Ok(()),
            _ => {
                Err("mean confidence interval contradicts its sample count or finite limits".into())
            }
        }
    }

    pub(crate) fn estimator_label(&self) -> String {
        match self.method {
            MonteCarloMeanMethod::StudentT => "Student t".into(),
            MonteCarloMeanMethod::PercentileBootstrap { resamples, seed } => {
                format!("Percentile bootstrap · {resamples} resamples · seed {seed}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_confidence_refuses_invalid_limits_and_population_claims() {
        let evidence = MonteCarloMeanConfidence {
            level_pct: 95.0,
            method: MonteCarloMeanMethod::StudentT,
            successful_samples: 3,
            conditional_on_successful_trials: true,
            interval: MonteCarloMeanInterval::Available {
                lower: 0.9,
                upper: 1.1,
            },
        };
        assert!(evidence.validate(3, 1).is_ok());
        assert!(evidence.validate(2, 1).is_err());
        assert!(evidence.validate(3, 0).is_err());
        for interval in [
            MonteCarloMeanInterval::Available {
                lower: f64::NAN,
                upper: 1.1,
            },
            MonteCarloMeanInterval::Available {
                lower: 1.2,
                upper: 1.1,
            },
            MonteCarloMeanInterval::InsufficientSamples,
        ] {
            assert!(
                MonteCarloMeanConfidence {
                    interval,
                    ..evidence
                }
                .validate(3, 1)
                .is_err()
            );
        }
        assert!(
            MonteCarloMeanConfidence {
                successful_samples: 1,
                interval: MonteCarloMeanInterval::InsufficientSamples,
                ..evidence
            }
            .validate(1, 1)
            .is_ok()
        );
    }
}
