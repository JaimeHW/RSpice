//! Conditional moments of the same Gaussian copula used by the sampler.
//!
//! Genz separation of variables maps each conditional normal interval onto a
//! unit interval. Positive probability weights retain joint conditioning; in
//! particular, bounding one variable also changes its unbounded partners.
//! Eight independently shifted Halton rules estimate integration error. These
//! points integrate the statistical model; they do not execute circuit trials.
//! See https://people.cs.kuleuven.be/~dirk.nuyens/mcqmc2014_proceedings_preprints/223.pdf.

use super::*;
use crate::ResourceLimits;
use crate::abort_signal::AbortSignal;

const REPLICATES: usize = 8;

/// Numerical integration policy for conditional statistical moments.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct StatisticalMomentOptions {
    /// Relative error estimate on covariance entries, scaled by the product
    /// of their marginal standard deviations. Also checks the conditional means.
    /// This is a numerical convergence estimate, not a rigorous error bound.
    pub relative_tolerance: Value,
    /// Total integrand evaluations available to one statistical scope.
    pub max_points: usize,
}
impl Default for StatisticalMomentOptions {
    fn default() -> Self {
        Self {
            relative_tolerance: 1e-3,
            max_points: 262_144,
        }
    }
}
impl StatisticalMomentOptions {
    pub fn validate(self) -> Result<(), SpectreStatisticsError> {
        if !self.relative_tolerance.is_finite()
            || self.relative_tolerance <= 0.0
            || self.relative_tolerance > 0.1
        {
            return Err(invalid(
                0,
                "Moment relative tolerance must be finite and in (0, 0.1]".into(),
            ));
        }
        if self.max_points < 1024 {
            return Err(invalid(
                0,
                "Moment integration requires a budget of at least 1024 points".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Default)]
pub(crate) struct SpectreScopeMoments {
    pub sigmas: Vec<SpectreVariationSigma>,
    pub correlation: Option<SpectreCorrelationMatrix>,
    pub evaluated_points: usize,
    pub relative_error_estimate: Value,
}

fn check_abort(abort: &dyn AbortSignal) -> Result<(), SpectreStatisticsError> {
    if abort.is_aborted() {
        Err(SpectreStatisticsError::Aborted)
    } else {
        Ok(())
    }
}

impl SpectreStatisticsPlan {
    pub(crate) fn scope_moments_with_abort(
        &self,
        scope: SpectreVariationScope,
        params: &ParamContext,
        process: &BTreeMap<String, Value>,
        options: StatisticalMomentOptions,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<SpectreScopeMoments, SpectreStatisticsError> {
        options.validate()?;
        check_abort(abort)?;
        let variations = self.resolve_scope(scope, params, process)?;
        let count = variations.len();
        if count > limits.max_result_values {
            return Err(invalid(
                0,
                "Statistical moment vector exceeds the result-value limit".into(),
            ));
        }
        let correlated = self.correlations.iter().any(|entry| entry.scope == scope);
        // Independent scopes remain linear in storage, even with many bounds.
        if correlated && count.saturating_mul(count).saturating_mul(32) > limits.max_result_values {
            return Err(invalid(
                0,
                "Statistical moment matrices exceed the result-value limit".into(),
            ));
        }
        let mut target = if correlated {
            let target = self.target_correlation_matrix(scope, &variations, params)?;
            SpectreCorrelationMatrix::new(target.clone())?;
            Some(target)
        } else {
            None
        };
        let groups = target.as_ref().map_or_else(
            || (0..count).map(|index| vec![index]).collect(),
            |target| bounded_groups(target),
        );
        let mut result = SpectreScopeMoments {
            sigmas: variations
                .iter()
                .map(|variation| SpectreVariationSigma {
                    parameter: variation.source.parameter.to_ascii_uppercase(),
                    nominal: variation.nominal,
                    standard_deviation: 0.0,
                })
                .collect(),
            ..Default::default()
        };
        let budget = options.max_points.min(limits.max_analysis_points);
        for mut group in groups {
            check_abort(abort)?;
            // Constrain early coordinates first to reduce integration variance.
            group.sort_by(|&a, &b| {
                variations[b]
                    .bounds
                    .is_some()
                    .cmp(&variations[a].bounds.is_some())
            });
            let local = group
                .iter()
                .map(|&index| variations[index].clone())
                .collect::<Vec<_>>();
            let local_target = group
                .iter()
                .map(|&a| {
                    group
                        .iter()
                        .map(|&b| {
                            target
                                .as_ref()
                                .map_or(if a == b { 1.0 } else { 0.0 }, |rows| rows[a][b])
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            // Validate feasibility in the sampler's latent space even when no
            // bounds are active: target PSD alone is insufficient for a copula.
            let factor =
                SpectreCorrelationMatrix::new(latent_correlation_matrix(&local, &local_target)?)?;
            if !local
                .iter()
                .any(|variation| variation.bounds.is_some() && variation.spread > 0.0)
            {
                for (position, &index) in group.iter().enumerate() {
                    result.sigmas[index].standard_deviation = unbounded_sigma(&local[position])?;
                }
                continue;
            }
            let (moments, error) = integrate(
                &local,
                &factor.lower,
                options.relative_tolerance,
                budget,
                &mut result.evaluated_points,
                abort,
            )?;
            result.relative_error_estimate = result.relative_error_estimate.max(error);
            let covariance = moments.covariance();
            for (a, &index) in group.iter().enumerate() {
                let sigma = libm::sqrt(covariance[a][a].max(0.0));
                result.sigmas[index].standard_deviation = sigma * scale(&local[a]);
                if !result.sigmas[index].standard_deviation.is_finite() {
                    return Err(invalid(
                        local[a].source.line,
                        "Conditional standard deviation is not finite".into(),
                    ));
                }
                if let Some(target) = &mut target {
                    for (b, &other) in group.iter().enumerate() {
                        let denominator = sigma * libm::sqrt(covariance[b][b].max(0.0));
                        target[index][other] = if a == b {
                            1.0
                        } else if denominator == 0.0 {
                            0.0
                        } else if local_target[a][b].abs() == 1.0 {
                            // Perfect Pearson correlation means an affine
                            // relationship, preserved by joint conditioning.
                            local_target[a][b]
                        } else {
                            (covariance[a][b] / denominator).clamp(-1.0, 1.0)
                        };
                    }
                }
            }
        }
        result.correlation = target.map(SpectreCorrelationMatrix::new).transpose()?;
        Ok(result)
    }
}

fn unbounded_sigma(variation: &ResolvedVariation<'_>) -> Result<Value, SpectreStatisticsError> {
    let sigma = match variation.source.distribution {
        SpectreDistribution::Gaussian => variation.spread,
        SpectreDistribution::Uniform => variation.spread / libm::sqrt(3.0),
        SpectreDistribution::Lognormal => {
            let variance = variation.spread * variation.spread;
            variation.nominal * libm::exp(variance * 0.5) * libm::sqrt(libm::expm1(variance))
        }
    };
    if sigma.is_finite() {
        Ok(sigma)
    } else {
        Err(invalid(
            variation.source.line,
            "Statistical variation has no finite standard deviation".into(),
        ))
    }
}

fn scale(variation: &ResolvedVariation<'_>) -> Value {
    if variation.source.distribution == SpectreDistribution::Lognormal {
        variation.nominal * variation.spread
    } else {
        variation.spread
    }
}
fn normalized(variation: &ResolvedVariation<'_>, score: Value) -> Value {
    if variation.spread == 0.0 {
        return 0.0;
    }
    match variation.source.distribution {
        SpectreDistribution::Gaussian => score,
        SpectreDistribution::Uniform => 2.0 * standard_normal_cdf(score) - 1.0,
        SpectreDistribution::Lognormal => libm::expm1(variation.spread * score) / variation.spread,
    }
}
fn score_bounds(variation: &ResolvedVariation<'_>) -> (Value, Value) {
    let Some(bounds) = &variation.bounds else {
        return (Value::NEG_INFINITY, Value::INFINITY);
    };
    if variation.spread == 0.0 {
        return (Value::NEG_INFINITY, Value::INFINITY);
    }
    let score = |value: Value| match variation.source.distribution {
        SpectreDistribution::Gaussian => (value - variation.nominal) / variation.spread,
        SpectreDistribution::Lognormal => {
            (libm::log(value) - libm::log(variation.nominal)) / variation.spread
        }
        SpectreDistribution::Uniform => {
            let probability = 0.5 * (1.0 + (value - variation.nominal) / variation.spread);
            if probability <= 0.0 {
                Value::NEG_INFINITY
            } else if probability >= 1.0 {
                Value::INFINITY
            } else {
                inverse_standard_normal_cdf(probability)
            }
        }
    };
    (score(bounds.lower), score(bounds.upper))
}

// Use survival probabilities in the positive tail to avoid subtracting from1.
fn cdf(value: Value) -> Value {
    0.5 * libm::erfc(-value / std::f64::consts::SQRT_2)
}
fn quantile(probability: Value) -> Value {
    if probability > 0.5 {
        return -quantile(1.0 - probability);
    }
    let mut score = inverse_standard_normal_cdf(probability);
    for _ in 0..2 {
        let density = libm::exp(-0.5 * score * score) / libm::sqrt(2.0 * std::f64::consts::PI);
        let error = cdf(score) - probability;
        score -= error / (density + 0.5 * score * error);
    }
    score
}
fn interval(lower: Value, upper: Value, unit: Value) -> Option<(Value, Value)> {
    if lower >= upper {
        return None;
    }
    let positive = lower > 0.0;
    let (low, high) = if positive {
        (cdf(-upper), cdf(-lower))
    } else {
        (cdf(lower), cdf(upper))
    };
    let mass = high - low;
    if mass <= 0.0 {
        return None;
    }
    let probability = low + mass * unit;
    let score = if positive {
        -quantile(probability)
    } else {
        quantile(probability)
    };
    score.is_finite().then_some((score, libm::log(mass)))
}

#[derive(Clone)]
struct WeightedMoments {
    log_scale: Value,
    weight: Value,
    mean: Vec<Value>,
    scatter: Vec<Vec<Value>>,
}
impl WeightedMoments {
    fn new(count: usize) -> Self {
        Self {
            log_scale: Value::NEG_INFINITY,
            weight: 0.0,
            mean: vec![0.0; count],
            scatter: vec![vec![0.0; count]; count],
        }
    }
    fn rescale(&mut self, log_scale: Value) {
        if log_scale <= self.log_scale {
            return;
        }
        let factor = libm::exp(self.log_scale - log_scale);
        self.weight *= factor;
        for row in &mut self.scatter {
            for value in row {
                *value *= factor;
            }
        }
        self.log_scale = log_scale;
    }
    fn add(&mut self, values: &[Value], log_weight: Value) {
        if self.weight == 0.0 {
            self.log_scale = log_weight;
            self.weight = 1.0;
            self.mean.copy_from_slice(values);
            return;
        }
        self.rescale(log_weight);
        let weight = libm::exp(log_weight - self.log_scale);
        let total = self.weight + weight;
        let delta = values
            .iter()
            .zip(&self.mean)
            .map(|(value, mean)| value - mean)
            .collect::<Vec<_>>();
        let product = weight * (self.weight / total);
        for a in 0..values.len() {
            self.mean[a] += weight / total * delta[a];
            for b in 0..values.len() {
                self.scatter[a][b] += product * delta[a] * delta[b];
            }
        }
        self.weight = total;
    }
    fn merge(&mut self, other: &Self) {
        if other.weight == 0.0 {
            return;
        }
        if self.weight == 0.0 {
            *self = other.clone();
            return;
        }
        self.rescale(other.log_scale);
        let factor = libm::exp(other.log_scale - self.log_scale);
        let weight = other.weight * factor;
        let total = self.weight + weight;
        let delta = other
            .mean
            .iter()
            .zip(&self.mean)
            .map(|(a, b)| a - b)
            .collect::<Vec<_>>();
        for a in 0..self.mean.len() {
            self.mean[a] += weight / total * delta[a];
            for b in 0..self.mean.len() {
                self.scatter[a][b] += factor * other.scatter[a][b]
                    + weight * (self.weight / total) * delta[a] * delta[b];
            }
        }
        self.weight = total;
    }
    fn covariance(&self) -> Vec<Vec<Value>> {
        self.scatter
            .iter()
            .map(|row| row.iter().map(|value| value / self.weight).collect())
            .collect()
    }
}

fn radical_inverse(mut index: usize, prime: usize) -> Value {
    let mut place = 1.0 / prime as Value;
    let mut result = 0.0;
    while index > 0 {
        result += (index % prime) as Value * place;
        index /= prime;
        place /= prime as Value;
    }
    result
}
fn primes(count: usize) -> Vec<usize> {
    let mut result = Vec::with_capacity(count);
    let mut candidate = 2;
    while result.len() < count {
        if result
            .iter()
            .take_while(|&&p| p <= candidate / p)
            .all(|p| candidate % p != 0)
        {
            result.push(candidate);
        }
        candidate += 1;
    }
    result
}

fn integrate(
    variations: &[ResolvedVariation<'_>],
    lower: &[Vec<Value>],
    tolerance: Value,
    budget: usize,
    evaluated: &mut usize,
    abort: &dyn AbortSignal,
) -> Result<(WeightedMoments, Value), SpectreStatisticsError> {
    let count = variations.len();
    let bounds = variations.iter().map(score_bounds).collect::<Vec<_>>();
    let mut independent = vec![(Value::NEG_INFINITY, Value::INFINITY); count];
    for row in 0..count {
        if lower[row][row] > PSD_TOLERANCE && lower[row][..row].iter().all(|value| *value == 0.0) {
            independent[row] = (
                bounds[row].0 / lower[row][row],
                bounds[row].1 / lower[row][row],
            );
        }
    }
    // A singular row depending on only one independent score can tighten that
    // score before drawing it. This also detects contradictory rank-one bounds.
    for (row, coefficients) in lower.iter().enumerate() {
        if coefficients[row] > PSD_TOLERANCE {
            continue;
        }
        let nonzero = coefficients
            .iter()
            .enumerate()
            .filter(|(_, value)| **value != 0.0)
            .collect::<Vec<_>>();
        if let [(column, coefficient)] = nonzero.as_slice() {
            let (a, b) = (bounds[row].0 / **coefficient, bounds[row].1 / **coefficient);
            independent[*column].0 = independent[*column].0.max(a.min(b));
            independent[*column].1 = independent[*column].1.min(a.max(b));
            if independent[*column].0 >= independent[*column].1 {
                return Err(invalid(
                    0,
                    "Correlated statistical bounds have no joint probability mass".into(),
                ));
            }
        }
    }
    let primes = primes(count);
    let mut replicas = (0..REPLICATES)
        .map(|_| WeightedMoments::new(count))
        .collect::<Vec<_>>();
    let shifts = (0..REPLICATES)
        .map(|replica| {
            (0..count)
                .map(|dimension| {
                    let bits = splitmix64(
                        variation_identity(&variations[dimension].source.parameter)
                            ^ ((replica as u64 + 1).wrapping_mul(0x9e37_79b9_7f4a_7c15)),
                    );
                    (bits >> 11) as Value * (1.0 / (1_u64 << 53) as Value)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut previous = None;
    let mut first = 1;
    let mut end = 65usize;
    loop {
        let work = (end - first).saturating_mul(REPLICATES);
        if evaluated.saturating_add(work) > budget {
            return Err(invalid(
                0,
                format!(
                    "Conditional statistical moments did not converge within {budget} integration points; increase the moment budget or relax its relative tolerance"
                ),
            ));
        }
        for (replica, accumulator) in replicas.iter_mut().enumerate() {
            for index in first..end {
                if index % 32 == 1 {
                    check_abort(abort)?;
                }
                *evaluated += 1;
                let mut independent_scores = vec![0.0; count];
                let mut values = vec![0.0; count];
                let mut weight = 0.0;
                let mut accepted = true;
                for row in 0..count {
                    let shift = (0..row)
                        .map(|column| lower[row][column] * independent_scores[column])
                        .sum::<Value>();
                    let diagonal = lower[row][row];
                    let score = if diagonal <= PSD_TOLERANCE {
                        if shift < bounds[row].0 || shift > bounds[row].1 {
                            accepted = false;
                            break;
                        }
                        shift
                    } else {
                        let low = ((bounds[row].0 - shift) / diagonal).max(independent[row].0);
                        let high = ((bounds[row].1 - shift) / diagonal).min(independent[row].1);
                        let unit = (radical_inverse(index, primes[row]) + shifts[replica][row])
                            .fract()
                            .clamp(Value::EPSILON, 1.0 - Value::EPSILON);
                        let Some((draw, mass)) = interval(low, high, unit) else {
                            accepted = false;
                            break;
                        };
                        independent_scores[row] = draw;
                        weight += mass;
                        shift + diagonal * draw
                    };
                    values[row] = normalized(&variations[row], score);
                    if !values[row].is_finite() {
                        return Err(invalid(
                            0,
                            "Conditional moment integrand is not finite".into(),
                        ));
                    }
                }
                if accepted {
                    accumulator.add(&values, weight);
                }
            }
        }
        let mut pooled = WeightedMoments::new(count);
        for replica in &replicas {
            pooled.merge(replica);
        }
        if replicas.iter().all(|replica| replica.weight > 0.0) {
            let covariance = pooled.covariance();
            if pooled
                .mean
                .iter()
                .chain(covariance.iter().flatten())
                .any(|value| !value.is_finite())
            {
                return Err(invalid(
                    0,
                    "Conditional moments exceed numerical range".into(),
                ));
            }
            if variations
                .iter()
                .enumerate()
                .all(|(index, variation)| variation.spread == 0.0 || covariance[index][index] > 0.0)
            {
                let error = relative_error(&pooled, &replicas, previous.as_ref());
                if error <= tolerance {
                    return Ok((pooled, error));
                }
            }
        }
        previous = (pooled.weight > 0.0).then_some(pooled);
        first = end;
        end = (end - 1).saturating_mul(2).saturating_add(1);
    }
}

fn relative_error(
    pooled: &WeightedMoments,
    replicas: &[WeightedMoments],
    previous: Option<&WeightedMoments>,
) -> Value {
    let Some(previous) = previous else {
        return Value::INFINITY;
    };
    let covariance = pooled.covariance();
    let old = previous.covariance();
    let rows = replicas
        .iter()
        .map(WeightedMoments::covariance)
        .collect::<Vec<_>>();
    let mut error = 0.0_f64;
    for a in 0..pooled.mean.len() {
        let sigma = libm::sqrt(covariance[a][a].max(0.0));
        let mean_error = libm::sqrt(
            replicas
                .iter()
                .map(|replica| (replica.mean[a] - pooled.mean[a]).powi(2))
                .sum::<Value>()
                / (REPLICATES * (REPLICATES - 1)) as Value,
        ) * 3.0;
        if sigma > 0.0 {
            error = error.max(mean_error.max((previous.mean[a] - pooled.mean[a]).abs()) / sigma);
        }
        for b in 0..pooled.mean.len() {
            let scale = sigma * libm::sqrt(covariance[b][b].max(0.0));
            if scale == 0.0 {
                continue;
            }
            let estimate = libm::sqrt(
                rows.iter()
                    .map(|row| (row[a][b] - covariance[a][b]).powi(2))
                    .sum::<Value>()
                    / (REPLICATES * (REPLICATES - 1)) as Value,
            ) * 3.0;
            error = error.max(estimate.max((old[a][b] - covariance[a][b]).abs()) / scale);
        }
    }
    error
}

#[cfg(test)]
mod tests;
