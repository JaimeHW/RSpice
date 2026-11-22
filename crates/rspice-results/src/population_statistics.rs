//! Descriptive statistics of a sampled population that every reader of one
//! states alike: the shape (G1, G2), process capability, and the normal
//! distribution a histogram is fitted with and a probability plot is read
//! against.

/// The mean of a non-empty set.
fn mean(values: &[f64]) -> Option<f64> {
    (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
}

/// Pearson's r over paired samples; `None` where either has no spread.
fn correlation(xs: &[f64], ys: &[f64]) -> Option<f64> {
    if xs.len() != ys.len() || xs.len() < 2 {
        return None;
    }
    let (mean_x, mean_y) = (mean(xs)?, mean(ys)?);
    let (mut covariance, mut variance_x, mut variance_y) = (0.0, 0.0, 0.0);
    for (x, y) in xs.iter().zip(ys) {
        let (dx, dy) = (x - mean_x, y - mean_y);
        covariance += dx * dy;
        variance_x += dx * dx;
        variance_y += dy * dy;
    }
    let denominator = (variance_x * variance_y).sqrt();
    (denominator > 0.0).then(|| (covariance / denominator).clamp(-1.0, 1.0))
}

/// The mean and the second, third and fourth central moments over n, when
/// the set has any spread at all.
fn central_moments(values: &[f64]) -> Option<(f64, f64, f64)> {
    let mean = mean(values)?;
    let (min, max) = values
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), value| {
            (lo.min(*value), hi.max(*value))
        });
    if !(max > min) {
        return None;
    }
    let n = values.len() as f64;
    let (mut m2, mut m3, mut m4) = (0.0, 0.0, 0.0);
    for value in values {
        let d = value - mean;
        let d2 = d * d;
        m2 += d2;
        m3 += d2 * d;
        m4 += d2 * d2;
    }
    let (m2, m3, m4) = (m2 / n, m3 / n, m4 / n);
    (m2 > 0.0 && m2.is_finite() && m3.is_finite() && m4.is_finite()).then_some((m2, m3, m4))
}

/// Sample skewness as the adjusted Fisher-Pearson coefficient G1, the one
/// Excel's SKEW, SAS, SPSS and `scipy.stats.skew(bias=False)` print:
/// g1 · √(n(n − 1)) / (n − 2). `None` below three samples, where G1 is not
/// defined, and for a set with no spread.
pub fn skewness(values: &[f64]) -> Option<f64> {
    let n = values.len();
    if n < 3 {
        return None;
    }
    let (m2, m3, _) = central_moments(values)?;
    let n = n as f64;
    let g1 = m3 / m2.powf(1.5);
    let value = g1 * (n * (n - 1.0)).sqrt() / (n - 2.0);
    value.is_finite().then_some(value)
}

/// Sample excess kurtosis as G2, the one Excel's KURT, SAS, SPSS and
/// `scipy.stats.kurtosis(bias=False)` print: ((n + 1) g2 + 6)(n − 1) /
/// ((n − 2)(n − 3)), with g2 = m4 / m2² − 3. Zero for a normal population.
/// `None` below four samples, where G2 is not defined, and for a set with no
/// spread.
pub fn excess_kurtosis(values: &[f64]) -> Option<f64> {
    let n = values.len();
    if n < 4 {
        return None;
    }
    let (m2, _, m4) = central_moments(values)?;
    let n = n as f64;
    let g2 = m4 / (m2 * m2) - 3.0;
    let value = ((n + 1.0) * g2 + 6.0) * (n - 1.0) / ((n - 2.0) * (n - 3.0));
    value.is_finite().then_some(value)
}

/// Process capability against a lower and an upper limit, either optional.
///
/// Cp is the specification width over six sigma and exists only for a
/// two-sided specification. Cpk is the distance from the mean to the nearer
/// limit in three-sigma units; with one limit it is that limit's one-sided
/// index (Cpu or Cpl), which is what a capability study of a single-limit
/// process publishes.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Capability {
    pub cp: Option<f64>,
    pub cpk: Option<f64>,
}

pub fn capability(mean: f64, sigma: f64, lower: Option<f64>, upper: Option<f64>) -> Capability {
    if !(sigma > 0.0) || !sigma.is_finite() || !mean.is_finite() {
        return Capability::default();
    }
    let lower = lower.filter(|value| value.is_finite());
    let upper = upper.filter(|value| value.is_finite());
    let cp = match (lower, upper) {
        (Some(lower), Some(upper)) if upper > lower => Some((upper - lower) / (6.0 * sigma)),
        _ => None,
    };
    let cpu = upper.map(|upper| (upper - mean) / (3.0 * sigma));
    let cpl = lower.map(|lower| (mean - lower) / (3.0 * sigma));
    let cpk = match (cpu, cpl) {
        (Some(cpu), Some(cpl)) => Some(cpu.min(cpl)),
        (one, None) | (None, one) => one,
    };
    Capability {
        cp: cp.filter(|value| value.is_finite()),
        cpk: cpk.filter(|value| value.is_finite()),
    }
}

/// The standard normal density.
pub fn normal_pdf(z: f64) -> f64 {
    const NORMALIZER: f64 = 0.398_942_280_401_432_7; // 1 / sqrt(2π)
    NORMALIZER * (-0.5 * z * z).exp()
}

/// The standard normal cumulative distribution, Φ(z), to a double's
/// precision in both tails.
pub fn normal_cdf(z: f64) -> f64 {
    0.5 * libm::erfc(-z / std::f64::consts::SQRT_2)
}

/// The standard normal quantile, Φ⁻¹(p), for 0 < p < 1: Acklam's rational
/// approximation polished by one Halley step against [`normal_cdf`], which
/// leaves it at a double's precision.
pub fn normal_quantile(p: f64) -> Option<f64> {
    if !(p > 0.0 && p < 1.0) {
        return None;
    }
    const A: [f64; 6] = [
        -3.969_683_028_665_376e1,
        2.209_460_984_245_205e2,
        -2.759_285_104_469_687e2,
        1.383_577_518_672_69e2,
        -3.066_479_806_614_716e1,
        2.506_628_277_459_239,
    ];
    const B: [f64; 5] = [
        -5.447_609_879_822_406e1,
        1.615_858_368_580_409e2,
        -1.556_989_798_598_866e2,
        6.680_131_188_771_972e1,
        -1.328_068_155_288_572e1,
    ];
    const C: [f64; 6] = [
        -7.784_894_002_430_293e-3,
        -3.223_964_580_411_365e-1,
        -2.400_758_277_161_838,
        -2.549_732_539_343_734,
        4.374_664_141_464_968,
        2.938_163_982_698_783,
    ];
    const D: [f64; 4] = [
        7.784_695_709_041_462e-3,
        3.224_671_290_700_398e-1,
        2.445_134_137_142_996,
        3.754_408_661_907_416,
    ];
    const LOW: f64 = 0.024_25;
    let tail = |q: f64| {
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    };
    let mut x = if p < LOW {
        tail((-2.0 * p.ln()).sqrt())
    } else if p > 1.0 - LOW {
        -tail((-2.0 * (1.0 - p).ln()).sqrt())
    } else {
        let q = p - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    };
    let error = normal_cdf(x) - p;
    let u = error / normal_pdf(x);
    x -= u / (1.0 + 0.5 * x * u);
    x.is_finite().then_some(x)
}

/// A normal probability plot: each sorted sample against the standard
/// normal quantile of its plotting position, and how straight the points
/// lie.
#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilityPlot {
    /// (theoretical quantile, sample), the samples ascending.
    pub points: Vec<(f64, f64)>,
    /// The probability-plot correlation coefficient: Pearson's r of the
    /// points. 1 is a perfectly normal sample. `None` with no spread.
    pub correlation: Option<f64>,
}

/// Filliben's plotting positions, the ones `scipy.stats.probplot` uses:
/// 1 − 0.5^(1/n) for the smallest sample, 0.5^(1/n) for the largest, and
/// (i − 0.3175) / (n + 0.365) between.
pub fn plotting_positions(n: usize) -> Vec<f64> {
    if n == 0 {
        return Vec::new();
    }
    let last = 0.5_f64.powf(1.0 / n as f64);
    (1..=n)
        .map(|i| {
            if i == n {
                last
            } else if i == 1 {
                1.0 - last
            } else {
                (i as f64 - 0.3175) / (n as f64 + 0.365)
            }
        })
        .collect()
}

/// The normal probability plot of an already sorted, finite sample set.
pub fn normal_probability_plot(sorted: &[f64]) -> Option<ProbabilityPlot> {
    if sorted.is_empty() {
        return None;
    }
    let quantiles = plotting_positions(sorted.len())
        .into_iter()
        .map(normal_quantile)
        .collect::<Option<Vec<_>>>()?;
    let correlation = correlation(&quantiles, sorted);
    Some(ProbabilityPlot {
        points: quantiles.into_iter().zip(sorted.iter().copied()).collect(),
        correlation,
    })
}

#[cfg(test)]
mod tests;
