//! Mean uncertainty estimators. Student-t limits follow the NIST engineering
//! statistics handbook: <https://www.itl.nist.gov/div898/handbook/eda/section3/eda352.htm>.
//! The incomplete beta evaluation uses the continued fraction in DLMF 8.17:
//! <https://dlmf.nist.gov/8.17#v>.

use super::{
    CompensatedSum, MonteCarloResult, VariableStatistics, Xorshift128Plus,
    statistical_location_scale,
};
use crate::Value;
use crate::abort_signal::AbortSignal;
use crate::analysis::error::SimulationError;
use crate::config::SimulationConfigError;
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};

/// Estimator for a two-sided confidence interval on the population mean.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MeanConfidenceMethod {
    /// Exact for independent normal observations; otherwise an approximation.
    #[default]
    StudentT,
    /// Empirical percentile bootstrap for independent observations. Its finite
    /// resampling resolution and small-sample coverage must be considered.
    /// The seed belongs to the estimator and never advances the circuit sampler.
    PercentileBootstrap { resamples: usize, seed: u64 },
}

impl MeanConfidenceMethod {
    pub const fn tag(self) -> &'static str {
        match self {
            Self::StudentT => "student-t-mean-v1",
            Self::PercentileBootstrap { .. } => "percentile-bootstrap-mean-v1",
        }
    }
}

/// A computed interval, or the reason finite limits could not be published.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeanConfidenceInterval {
    Available { lower: Value, upper: Value },
    InsufficientSamples,
    Unrepresentable,
}

/// Assumptions and population associated with every variable's mean interval.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MonteCarloConfidence {
    pub level_pct: Value,
    pub method: MeanConfidenceMethod,
    pub successful_samples: usize,
    /// Failures censor the original population. Intervals then describe only
    /// successful trials, with no claim about unconditional manufacturing yield.
    pub conditional_on_successful_trials: bool,
}

pub(super) fn validate_request(
    level_pct: Value,
    method: MeanConfidenceMethod,
    samples: usize,
    variables: usize,
    limits: ResourceLimits,
) -> Result<(), SimulationError> {
    if !level_pct.is_finite() || level_pct <= 0.0 || level_pct >= 100.0 {
        return Err(SimulationConfigError::InvalidValue {
            field: "monte_carlo.confidence_pct",
            value: level_pct,
            requirement: "finite and strictly between 0 and 100",
        }
        .into());
    }
    if let MeanConfidenceMethod::PercentileBootstrap { resamples, .. } = method {
        if resamples < 2 {
            return Err(SimulationError::Circuit(
                "Mean bootstrap confidence requires at least two resamples".to_owned(),
            ));
        }
        let draws = samples
            .checked_mul(variables)
            .and_then(|n| n.checked_mul(resamples))
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "Mean bootstrap confidence draw count overflowed".to_owned(),
                )
            })?;
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            draws,
            limits.max_analysis_points,
        )?;
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            resamples,
            limits.max_result_values,
        )?;
    }
    Ok(())
}

impl MonteCarloResult {
    /// Estimate uncertainty in each mean. This never treats a population
    /// percentile or a convergence rate as a confidence interval.
    ///
    /// All methods require independent trials. Student-t is exact only for
    /// normal observations; bootstrap coverage depends on sample size and the
    /// chosen resampling count. Updates are atomic on errors or cancellation.
    pub fn compute_mean_confidence(
        &mut self,
        level_pct: Value,
        method: MeanConfidenceMethod,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::from_abort(abort));
        }
        if self.num_runs == 0 {
            return Err(SimulationConfigError::InvalidCount {
                field: "monte_carlo.num_runs",
                value: 0,
            }
            .into());
        }
        if self.all_converged != (self.num_failures == 0) {
            return Err(SimulationError::Circuit(
                "Monte Carlo convergence flag disagrees with failed run count".to_owned(),
            ));
        }
        let samples = self
            .num_runs
            .checked_sub(self.num_failures)
            .ok_or_else(|| {
                SimulationError::Circuit("Monte Carlo failed runs exceed attempted runs".to_owned())
            })?;
        validate_request(level_pct, method, samples, self.variables.len(), limits)?;
        let retained_values = self
            .variables
            .values()
            .try_fold(0usize, |count, variable| {
                if variable.samples.len() != samples {
                    return Err(SimulationError::Circuit(format!(
                        "Monte Carlo variable '{}' has invalid successful-trial samples",
                        variable.name,
                    )));
                }
                for chunk in variable.samples.chunks(256) {
                    if abort.is_aborted() {
                        return Err(SimulationError::from_abort(abort));
                    }
                    if chunk.iter().any(|sample| !sample.is_finite()) {
                        return Err(SimulationError::Circuit(format!(
                            "Monte Carlo variable '{}' has nonfinite successful-trial samples",
                            variable.name,
                        )));
                    }
                }
                Ok(count
                    .saturating_add(variable.samples.len())
                    .saturating_add(variable.histogram.len())
                    .saturating_add(variable.bin_edges.len())
                    .saturating_add(7))
            })?;
        let scratch = match method {
            MeanConfidenceMethod::StudentT => 0,
            MeanConfidenceMethod::PercentileBootstrap { resamples, .. } => resamples,
        };
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            retained_values.saturating_add(scratch),
            limits.max_result_values,
        )?;
        let critical = if method == MeanConfidenceMethod::StudentT && samples >= 2 {
            Some(student_t_critical(level_pct, samples - 1).ok_or_else(|| {
                SimulationError::Circuit(
                    "Student-t confidence quantile did not converge".to_owned(),
                )
            })?)
        } else {
            None
        };
        let mut intervals = Vec::with_capacity(self.variables.len());
        for (name, variable) in &self.variables {
            if abort.is_aborted() {
                return Err(SimulationError::from_abort(abort));
            }
            let interval = if samples < 2 {
                MeanConfidenceInterval::InsufficientSamples
            } else {
                match method {
                    MeanConfidenceMethod::StudentT => {
                        let half_width = (variable.std_dev / (samples as Value).sqrt())
                            * critical.expect("Student-t quantile was resolved");
                        finite_interval(variable.mean - half_width, variable.mean + half_width)
                    }
                    MeanConfidenceMethod::PercentileBootstrap { resamples, seed } => {
                        bootstrap_interval(variable, level_pct, resamples, seed, abort)?
                    }
                }
            };
            intervals.push((name.clone(), interval));
        }
        if abort.is_aborted() {
            return Err(SimulationError::from_abort(abort));
        }
        for (name, interval) in intervals {
            self.variables
                .get_mut(&name)
                .expect("variable set is unchanged")
                .mean_confidence = Some(interval);
        }
        self.confidence = Some(MonteCarloConfidence {
            level_pct,
            method,
            successful_samples: samples,
            conditional_on_successful_trials: self.num_failures != 0,
        });
        Ok(())
    }
}

fn finite_interval(lower: Value, upper: Value) -> MeanConfidenceInterval {
    if lower.is_finite() && upper.is_finite() && lower <= upper {
        MeanConfidenceInterval::Available { lower, upper }
    } else {
        MeanConfidenceInterval::Unrepresentable
    }
}

fn bootstrap_interval(
    variable: &VariableStatistics,
    level_pct: Value,
    resamples: usize,
    seed: u64,
    abort: &dyn AbortSignal,
) -> Result<MeanConfidenceInterval, SimulationError> {
    let n = variable.samples.len();
    let (anchor, scale) = statistical_location_scale(&variable.samples, variable.min, variable.max);
    if scale == 0.0 {
        return Ok(finite_interval(anchor, anchor));
    }
    let mut rng = Xorshift128Plus::new(seed);
    let mut means = Vec::with_capacity(resamples);
    // Rejection sampling avoids modulo bias for non-power-of-two lengths.
    let modulus = n as u64;
    let threshold = modulus.wrapping_neg() % modulus;
    for _ in 0..resamples {
        let mut sum = CompensatedSum::default();
        for index in 0..n {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::from_abort(abort));
            }
            let draw = loop {
                let draw = rng.next_u64();
                if draw >= threshold {
                    break draw;
                }
            };
            let value = (variable.samples[(draw % modulus) as usize] - anchor) / scale;
            sum.add(value);
        }
        means.push(anchor + (sum.total() / n as Value) * scale);
    }
    means.sort_by(Value::total_cmp);
    let lower_probability = (100.0 - level_pct) / 200.0;
    let percentile = |probability: Value| {
        let position = probability * (resamples - 1) as Value;
        let lower = position.floor() as usize;
        let upper = position.ceil() as usize;
        let fraction = position - lower as Value;
        let (left, right) = (means[lower], means[upper]);
        if left == right {
            return left;
        }
        let span = right - left;
        if span.is_finite() {
            left + fraction * span
        } else {
            (1.0 - fraction) * left + fraction * right
        }
    };
    Ok(finite_interval(
        percentile(lower_probability),
        percentile(1.0 - lower_probability),
    ))
}

/// Positive Student-t quantile enclosing level_pct of the central population.
fn student_t_critical(level_pct: Value, degrees: usize) -> Option<Value> {
    let nu = degrees as Value;
    let central = level_pct / 100.0;
    let tail = (100.0 - level_pct) / 100.0;
    if central <= 0.0 || tail <= 0.0 || degrees == 0 {
        return None;
    }
    if degrees == 1 {
        return Some(if level_pct < 50.0 {
            (std::f64::consts::FRAC_PI_2 * central).tan()
        } else {
            1.0 / (std::f64::consts::FRAC_PI_2 * tail).tan()
        });
    }
    let log_beta = log_beta_half(0.5 * nu);
    if central < 1e-6 {
        let first = central * (0.5 * nu.sqrt() * log_beta.exp());
        return Some(first * (1.0 + ((nu + 1.0) / (6.0 * nu)) * first * first));
    }
    let below_target = |t: Value| -> Option<bool> {
        let square = t * t;
        if level_pct < 50.0 {
            let x = square / (nu + square);
            let complement = nu / (nu + square);
            Some(regularized_beta(x, complement, 0.5, nu * 0.5, log_beta)? < central)
        } else {
            let x = nu / (nu + square);
            let complement = square / (nu + square);
            Some(regularized_beta(x, complement, nu * 0.5, 0.5, log_beta)? > tail)
        }
    };
    let mut lower = 0.0;
    let mut upper = 1.0;
    while below_target(upper)? {
        lower = upper;
        upper *= 2.0;
        if !upper.is_finite() {
            return None;
        }
    }
    for _ in 0..128 {
        let midpoint = lower + (upper - lower) * 0.5;
        if midpoint == lower || midpoint == upper {
            return Some(midpoint);
        }
        if below_target(midpoint)? {
            lower = midpoint;
        } else {
            upper = midpoint;
        }
    }
    None
}

fn log_beta_half(b: Value) -> Value {
    let log_sqrt_pi = 0.5 * std::f64::consts::PI.ln();
    if b < 10_000.0 {
        log_sqrt_pi + libm::lgamma(b) - libm::lgamma(b + 0.5)
    } else {
        let inverse = 1.0 / b;
        // Gamma-ratio asymptotics avoid subtracting two large log-gammas.
        log_sqrt_pi - 0.5 * b.ln() + inverse / 8.0 - inverse.powi(3) / 192.0
            + inverse.powi(5) / 640.0
    }
}

fn regularized_beta(
    x: Value,
    complement: Value,
    a: Value,
    b: Value,
    log_beta: Value,
) -> Option<Value> {
    if x <= 0.0 {
        return Some(0.0);
    }
    if complement <= 0.0 {
        return Some(1.0);
    }
    let log_x = if x < 0.5 {
        x.ln()
    } else {
        (-complement).ln_1p()
    };
    let log_complement = if complement < 0.5 {
        complement.ln()
    } else {
        (-x).ln_1p()
    };
    let front = (a * log_x + b * log_complement - log_beta).exp();
    let value = if x < (a + 1.0) / (a + b + 2.0) {
        front * beta_fraction(x, a, b)? / a
    } else {
        1.0 - front * beta_fraction(complement, b, a)? / b
    };
    value.is_finite().then_some(value.clamp(0.0, 1.0))
}

fn beta_fraction(x: Value, a: Value, b: Value) -> Option<Value> {
    let protect = |value: Value| {
        if value.abs() < 1e-300 {
            1e-300_f64.copysign(value)
        } else {
            value
        }
    };
    let mut c = 1.0;
    let mut d = 1.0 / protect(1.0 - (a + b) * x / (a + 1.0));
    let mut fraction = d;
    for index in 1..=512 {
        let m = index as Value;
        let first = m * (b - m) * x / ((a + 2.0 * m - 1.0) * (a + 2.0 * m));
        let second = -(a + m) * (a + b + m) * x / ((a + 2.0 * m) * (a + 2.0 * m + 1.0));
        for coefficient in [first, second] {
            d = 1.0 / protect(1.0 + coefficient * d);
            c = protect(1.0 + coefficient / c);
            fraction *= d * c;
        }
        if (d * c - 1.0).abs() <= 8.0 * Value::EPSILON {
            return fraction.is_finite().then_some(fraction);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn student_t_quantiles_match_high_precision_reference_values() {
        // mpmath 1.3.0, 60 decimal digits, independently invert the regularized
        // incomplete beta integral. Includes the gamma-ratio transition and
        // central/tail probabilities across small and large sample counts.
        for (degrees, level, expected) in [
            (1, 95.0, 12.706204736174705),
            (2, 95.0, 4.302652729749464),
            (3, 99.0, 5.840909309733357),
            (9, 95.0, 2.2621571627982053),
            (29, 95.0, 2.0452296421327043),
            (99, 99.9, 3.391528833363668),
            (999, 95.0, 1.96234146113345),
            (19999, 95.0, 1.9600826110898155),
            (20000, 95.0, 1.9600826051581353),
            (999999, 95.0, 1.9599663568164793),
            (7, 1e-7, 1.2987301378228252e-9),
            (7, 1.0, 0.012987718654428904),
            (7, 20.0, 0.26316686135202283),
            (7, 50.0, 0.7111417780817864),
            (7, 99.9999, 15.767009437405942),
        ] {
            let actual = student_t_critical(level, degrees).expect("quantile converges");
            assert!(
                (actual / expected - 1.0).abs() < 3e-11,
                "df={degrees}, level={level}: {actual}, expected {expected}"
            );
        }
    }
}
