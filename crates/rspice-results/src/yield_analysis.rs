//! Portable yield specifications, retained population evidence and statistics.
use rspice_app_types::product::{DatasetId, RunId};
use serde::{Deserialize, Serialize};

// =============================================================================
// Yield Specifications
// =============================================================================

/// Type of specification limit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecLimitType {
    /// Lower specification limit (LSL)
    Lower,
    /// Upper specification limit (USL)
    Upper,
    /// Both LSL and USL (Range)
    Range,
}

/// A target specification for a measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldSpec {
    /// Name of the measurement/signal
    pub target: String,
    /// Type of limit
    pub limit_type: SpecLimitType,
    /// Lower limit value
    pub min: Option<f64>,
    /// Upper limit value
    pub max: Option<f64>,
    /// Target nominal value
    pub target_val: Option<f64>,
    /// Unit (e.g., "V", "ns")
    pub unit: String,
    /// Weight/Priority for optimization (0.0 - 1.0)
    pub weight: f32,
}

impl YieldSpec {
    /// Create a lower limit spec
    pub fn lower(target: impl Into<String>, min: f64, unit: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            limit_type: SpecLimitType::Lower,
            min: Some(min),
            max: None,
            target_val: None,
            unit: unit.into(),
            weight: 1.0,
        }
    }

    /// Create an upper limit spec
    pub fn upper(target: impl Into<String>, max: f64, unit: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            limit_type: SpecLimitType::Upper,
            min: None,
            max: Some(max),
            target_val: None,
            unit: unit.into(),
            weight: 1.0,
        }
    }

    /// Create a range spec
    pub fn range(target: impl Into<String>, min: f64, max: f64, unit: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            limit_type: SpecLimitType::Range,
            min: Some(min),
            max: Some(max),
            target_val: Some((min + max) / 2.0),
            unit: unit.into(),
            weight: 1.0,
        }
    }

    /// Check if a value passes the specification
    pub fn evaluates(&self, value: f64) -> bool {
        match self.limit_type {
            SpecLimitType::Lower => self.min.is_none_or(|m| value >= m),
            SpecLimitType::Upper => self.max.is_none_or(|m| value <= m),
            SpecLimitType::Range => {
                let lower = self.min.is_none_or(|m| value >= m);
                let upper = self.max.is_none_or(|m| value <= m);
                lower && upper
            }
        }
    }
}

// =============================================================================
// Statistical Results
// =============================================================================

/// Statistical distribution data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DistributionStats {
    pub count: usize,
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub median: f64,
    pub skewness: f64,
    pub kurtosis: f64,
    /// Process capability index
    pub cp: Option<f64>,
    /// Process capability index (centered)
    pub cpk: Option<f64>,
}

/// Yield result for a single specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldResult {
    pub spec: YieldSpec,
    /// The yield denominator: every trial the engine reported, whether or not
    /// it produced a finite observation.
    ///
    /// This is the one convention, and it is stated wherever the percentage is
    /// shown. A trial that diverged is counted here and counted as a failure
    /// — a specification a run could not evaluate is not a specification it
    /// met — so the figure is the conservative one. It is *not* the number of
    /// runs requested: trials the engine never completed are not in the
    /// retained population at all, and the distribution panel names them
    /// separately rather than folding them into this count.
    pub total_runs: usize,
    pub pass_count: usize,
    pub fail_count: usize,
    pub yield_percent: f64,
    pub stats: DistributionStats,
    /// Pass/Fail list for each iteration
    pub trail: Vec<bool>,
    /// Exact finite sample values used for the distribution statistics. This
    /// keeps verification plots evidence-backed instead of reconstructing a
    /// synthetic distribution from summary moments.
    #[serde(default)]
    pub samples: Vec<f64>,
}

/// Immutable identity of the retained result dataset used for the current
/// yield evidence. Stable product IDs prevent a history reorder from silently
/// retargeting verification results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct YieldAnalysisProvenance {
    pub source_run_id: RunId,
    pub source_dataset_id: DatasetId,
    pub seed: u64,
    pub runs_requested: usize,
    pub runs_completed: usize,
    pub sampling_mode: MonteCarloSamplingMode,
}

/// Sampling algorithm used by the current Monte Carlo engine. The engine uses
/// a deterministic pseudo-random number generator; stratified and Latin
/// hypercube modes are not represented until they are actually implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MonteCarloSamplingMode {
    PseudoRandom,
}

impl MonteCarloSamplingMode {
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::PseudoRandom => "Pseudo-random",
        }
    }
}

/// Calculate comprehensive statistical metrics
pub fn calculate_distribution_stats(values: &[f64], spec: &YieldSpec) -> DistributionStats {
    if values.is_empty() {
        return DistributionStats::default();
    }

    // Defensive filtering so direct callers cannot poison statistics with NaN/Inf.
    let mut finite_values = values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect::<Vec<_>>();
    if finite_values.is_empty() {
        return DistributionStats::default();
    }

    if finite_values.len() == 1 {
        let value = finite_values[0];
        return DistributionStats {
            count: 1,
            mean: value,
            std_dev: 0.0,
            min: value,
            max: value,
            median: value,
            skewness: 0.0,
            kurtosis: 0.0,
            cp: None,
            cpk: None,
        };
    }

    let n = finite_values.len() as f64;
    let mean = finite_values.iter().sum::<f64>() / n;
    let variance_num = finite_values
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f64>();
    let variance = variance_num / (n - 1.0);
    let std_dev = if variance.is_finite() && variance > 0.0 {
        variance.sqrt()
    } else {
        0.0
    };

    finite_values.sort_by(f64::total_cmp);
    let lower_mid = finite_values[(finite_values.len() - 1) / 2];
    let upper_mid = finite_values[finite_values.len() / 2];
    let median = (lower_mid + upper_mid) * 0.5;
    let min = finite_values[0];
    let max = finite_values[finite_values.len() - 1];

    // Skewness and kurtosis remain centered moments around the mean.
    let (skewness, kurtosis) = if std_dev > 0.0 {
        let m3 = finite_values
            .iter()
            .map(|value| (value - mean).powi(3))
            .sum::<f64>()
            / n;
        let m4 = finite_values
            .iter()
            .map(|value| (value - mean).powi(4))
            .sum::<f64>()
            / n;
        let skewness = m3 / std_dev.powi(3);
        let kurtosis = (m4 / std_dev.powi(4)) - 3.0;
        if skewness.is_finite() && kurtosis.is_finite() {
            (skewness, kurtosis)
        } else {
            (0.0, 0.0)
        }
    } else {
        (0.0, 0.0)
    };

    // Capability indices.
    //
    // Cp is the ratio of the specification width to the process width and
    // therefore exists only for a two-sided specification. Cpk is the
    // distance from the mean to the nearer bound in three-sigma units, and
    // a bound that does not exist is not a nearer one: a one-sided
    // specification reports the one-sided form — Cpu against an upper
    // limit, Cpl against a lower one — which is what a capability study
    // of a process with a single limit publishes. Refusing to compute it
    // left the majority of specifications showing an em dash for the one
    // index they can actually have.
    let mut cp = None;
    let mut cpk = None;

    if std_dev > 0.0 {
        // Exactly the bounds `YieldSpec::evaluates` judges against, so an
        // index can never be computed from a limit the verdict ignores.
        let finite = |value: &f64| value.is_finite();
        let (lsl, usl) = match spec.limit_type {
            SpecLimitType::Lower => (spec.min.filter(finite), None),
            SpecLimitType::Upper => (None, spec.max.filter(finite)),
            SpecLimitType::Range => (spec.min.filter(finite), spec.max.filter(finite)),
        };
        if let (Some(lsl), Some(usl)) = (lsl, usl)
            && usl > lsl
        {
            let cp_val = (usl - lsl) / (6.0 * std_dev);
            if cp_val.is_finite() {
                cp = Some(cp_val);
            }
        }
        let cpu = usl.map(|usl| (usl - mean) / (3.0 * std_dev));
        let cpl = lsl.map(|lsl| (mean - lsl) / (3.0 * std_dev));
        let cpk_val = match (cpu, cpl) {
            (Some(cpu), Some(cpl)) => Some(cpu.min(cpl)),
            (Some(one), None) | (None, Some(one)) => Some(one),
            (None, None) => None,
        };
        cpk = cpk_val.filter(|value| value.is_finite());
    }

    DistributionStats {
        count: finite_values.len(),
        mean,
        std_dev,
        min,
        max,
        median,
        skewness,
        kurtosis,
        cp,
        cpk,
    }
}

#[cfg(test)]
#[test]
fn distribution_stats_keep_finite_samples_and_capability_limits() {
    let values = [9.0, f64::NAN, 10.0, 11.0];
    let lower = calculate_distribution_stats(&values, &YieldSpec::lower("gain", 7.0, ""));
    assert_eq!(lower.count, 3);
    assert_eq!((lower.mean, lower.std_dev, lower.median), (10.0, 1.0, 10.0));
    assert_eq!(lower.cp, None);
    assert!(lower.cpk.is_some_and(|cpk| (cpk - 1.0).abs() < 1e-12));

    let upper = calculate_distribution_stats(&values, &YieldSpec::upper("gain", 16.0, ""));
    assert_eq!(upper.cp, None);
    assert!(upper.cpk.is_some_and(|cpk| (cpk - 2.0).abs() < 1e-12));

    let range = calculate_distribution_stats(&values, &YieldSpec::range("gain", 7.0, 16.0, ""));
    assert!(range.cp.is_some_and(|cp| (cp - 1.5).abs() < 1e-12));
    assert!(range.cpk.is_some_and(|cpk| (cpk - 1.0).abs() < 1e-12));
}

mod manager;
pub use manager::{YieldAnalysisManager, YieldInput};
mod evidence;
pub use evidence::YieldEvidence;
