//! The retained statistical population both distribution sheets read.
//!
//! A Monte Carlo retains two independent things: the exact samples of every
//! swept variable, and what each retained trial measured. Neither is a
//! distribution on its own — the scatter sheet needs them side by side per
//! trial, and the box/violin sheet needs one measured column at a time
//! against the requirement that bounds it. Both projections are the same walk
//! over the same evidence, so it is done once here, keyed by the dataset
//! generation that produced it.
//!
//! The one thing this module refuses to do is pair evidence that the result
//! does not say is paired. Variable samples are indexed by *retained* trial
//! and member measurements by the index the driver *requested*; a Monte Carlo
//! that dropped a diverged trial has no correspondence between them, and
//! plotting a variable against a measurement across that gap would draw a
//! correlation nobody measured. See [`PopulationPlan::variables_paired`].

use std::sync::Arc;

use super::{AnalysisPresentationKey, SheetContext};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, FamilyMemberId, ProjectWorkspace, SpecEntry,
};

/// Where one column of the population came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ColumnKind {
    /// A swept statistical variable: the analysis drew these values.
    SampledVariable,
    /// A `.MEAS` result the trial produced.
    Measurement,
}

/// The authored requirement bounding one measured column.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct PopulationLimit {
    pub(super) min: Option<f64>,
    pub(super) max: Option<f64>,
    /// The bound spelled the way every other surface spells it.
    pub(super) text: String,
}

impl PopulationLimit {
    pub(super) fn passes(&self, value: f64) -> bool {
        self.min.is_none_or(|min| value >= min) && self.max.is_none_or(|max| value <= max)
    }

    /// Signed distance to the nearest bound: positive inside, negative out.
    /// The same quantity the Specs sheet calls the margin.
    pub(super) fn signed_margin(&self, value: f64) -> Option<f64> {
        match (self.min, self.max) {
            (Some(min), Some(max)) => Some((value - min).min(max - value)),
            (Some(min), None) => Some(value - min),
            (None, Some(max)) => Some(max - value),
            (None, None) => None,
        }
    }

    /// The margin as a percentage of the bound it is measured against.
    ///
    /// A two-sided requirement is measured against its half-width, so the
    /// centre of the window is 100 % and either bound is 0 %; a one-sided one
    /// is measured against the bound itself, which is what a datasheet
    /// margin means. A bound of zero has no percentage — stated as `None`
    /// rather than as an infinity nobody can read.
    pub(super) fn margin_percent(&self, value: f64) -> Option<f64> {
        let margin = self.signed_margin(value)?;
        let reference = match (self.min, self.max) {
            (Some(min), Some(max)) => (max - min) / 2.0,
            (Some(min), None) => min.abs(),
            (None, Some(max)) => max.abs(),
            (None, None) => return None,
        };
        (reference > 0.0).then(|| 100.0 * margin / reference)
    }
}

/// One column of per-trial values.
#[derive(Debug, Clone)]
pub(super) struct PopulationColumn {
    pub(super) name: String,
    pub(super) kind: ColumnKind,
    /// The producer's unit, when a requirement stated one. Empty is
    /// "unstated", never "dimensionless".
    pub(super) unit: String,
    /// One entry per trial row. `None` is a trial whose measurement ran and
    /// produced no number — never a zero.
    pub(super) values: Vec<Option<f64>>,
    pub(super) limit: Option<PopulationLimit>,
}

impl PopulationColumn {
    /// Every finite value in trial order, with the trial it came from.
    pub(super) fn measured(&self) -> impl Iterator<Item = (usize, f64)> + '_ {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(trial, value)| value.map(|value| (trial, value)))
    }

    pub(super) fn measured_values(&self) -> Vec<f64> {
        self.measured().map(|(_, value)| value).collect()
    }
}

/// What one trial's evidence says about it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TrialStatus {
    /// Every bounded measurement this trial carries is inside its bound.
    Passing,
    /// At least one bounded measurement is outside its bound.
    Failing,
    /// The trial's own measurement did not produce a number. This is not a
    /// failure — it is evidence the analysis could not take — and the two are
    /// never collapsed.
    Unmeasured,
}

/// One trial, named the way a verdict has to name it.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct TrialIdentity {
    /// The index the driver requested this trial under.
    pub(super) index: usize,
    /// The seed that reproduces it, when the family retained one.
    pub(super) seed: Option<u64>,
    pub(super) label: String,
}

/// The population projection, built once per dataset generation.
#[derive(Debug, Clone)]
pub(super) struct PopulationPlan {
    version: u64,
    analysis: AnalysisPresentationKey,
    specs_revision: u64,
    pub(super) trials: Vec<TrialIdentity>,
    pub(super) columns: Vec<PopulationColumn>,
    pub(super) status: Vec<TrialStatus>,
    pub(super) seed: u64,
    pub(super) runs_requested: usize,
    pub(super) runs_completed: usize,
    pub(super) failures: usize,
    /// Whether a sampled variable may be read against a measurement.
    ///
    /// False whenever the retained trial rows are not the sampled ordinals —
    /// a diverged trial dropped from the distribution leaves the two indexed
    /// differently, and no correspondence between them is retained.
    pub(super) variables_paired: bool,
}

impl PopulationPlan {
    pub(super) fn trial_count(&self) -> usize {
        self.trials.len()
    }

    /// Where a named column sits. Matched case-insensitively, the way every
    /// other measurement join in the product matches.
    pub(super) fn column_index(&self, name: &str) -> Option<usize> {
        self.columns
            .iter()
            .position(|column| column.name.eq_ignore_ascii_case(name))
    }

    pub(super) fn failing_count(&self) -> usize {
        self.status
            .iter()
            .filter(|status| **status == TrialStatus::Failing)
            .count()
    }

    /// Whether two columns may be read against one another.
    pub(super) fn columns_are_paired(
        &self,
        left: &PopulationColumn,
        right: &PopulationColumn,
    ) -> bool {
        self.variables_paired
            || left.kind == right.kind
            || left.kind == ColumnKind::Measurement && right.kind == ColumnKind::Measurement
    }
}

/// Why a measured column and a sampled variable cannot be read together.
pub(super) const UNPAIRED_REASON: &str = "This run dropped trials, so its sampled variables and its measurements are indexed \
     differently — no per-trial correspondence between them is retained.";

/// Resolve the population once per (dataset generation, analysis, requirement
/// revision) and hand back a shared handle.
pub(super) fn plan(context: &mut SheetContext<'_>) -> Option<Arc<PopulationPlan>> {
    let run = context.simulation.active_run()?;
    let dataset_id = run.dataset_id;
    let analysis = context.simulation.active_analysis()?;
    let key = AnalysisPresentationKey::new(dataset_id, analysis);
    let version = context.simulation.data_version;
    let specs_revision = specs_revision(context.workspace);
    if let Some(plan) = context.results.plans.population.as_ref()
        && plan.version == version
        && plan.analysis == key
        && plan.specs_revision == specs_revision
    {
        return Some(Arc::clone(plan));
    }
    let built = Arc::new(build(analysis, key, version, context.workspace)?);
    context.results.plans.population = Some(Arc::clone(&built));
    Some(built)
}

/// A cheap fingerprint of the authored requirements, so an edited bound
/// rebuilds the pass/fail column without a re-run.
fn specs_revision(workspace: &ProjectWorkspace) -> u64 {
    let mut revision = workspace.specs.len() as u64;
    for spec in &workspace.specs {
        revision = revision
            .wrapping_mul(0x100_0000_01b3)
            .wrapping_add(spec.measurement.len() as u64)
            .wrapping_add(spec.min.map_or(0, f64::to_bits))
            .wrapping_add(spec.max.map_or(0, f64::to_bits));
    }
    revision
}

/// Whether one retained analysis carries a population worth a distribution.
///
/// Deliberately blind to the authored requirements: whether a bound exists is
/// what the sheet reports, not what decides that there is a population.
pub(super) fn is_a_population(analysis: &AnalysisResult) -> bool {
    analysis.success
        && analysis.family_metadata.as_ref().is_some_and(|metadata| {
            matches!(
                metadata,
                AnalysisResultFamilyMetadata::MonteCarlo {
                    variables,
                    member_measurements,
                    ..
                } if member_measurements.len() >= 2
                    || variables.iter().any(|variable| variable.samples.len() >= 2)
            ) && metadata.validate_for(analysis.analysis_type).is_ok()
        })
}

fn build(
    analysis: &AnalysisResult,
    key: AnalysisPresentationKey,
    version: u64,
    workspace: &ProjectWorkspace,
) -> Option<PopulationPlan> {
    if !analysis.success {
        return None;
    }
    let metadata = analysis.family_metadata.as_ref()?;
    let AnalysisResultFamilyMetadata::MonteCarlo {
        seed,
        runs_requested,
        runs_completed,
        failures,
        variables,
        member_measurements,
        ..
    } = metadata
    else {
        return None;
    };
    if metadata.validate_for(analysis.analysis_type).is_err() {
        return None;
    }

    // The retained sample count every variable agrees on, if they agree.
    let sample_count = variables.first().map(|variable| variable.samples.len());
    let variables_agree = sample_count.is_some_and(|count| {
        variables
            .iter()
            .all(|variable| variable.samples.len() == count)
    });

    // Member rows are the trials that measured something; when there are
    // none, the sampled ordinals are the population.
    let (trials, row_count, rows_are_members) = if member_measurements.is_empty() {
        let count = sample_count.unwrap_or(0);
        (
            (0..count)
                .map(|index| TrialIdentity {
                    index,
                    seed: None,
                    label: format!("Trial {index}"),
                })
                .collect::<Vec<_>>(),
            count,
            false,
        )
    } else {
        (
            member_measurements
                .iter()
                .map(|member| TrialIdentity {
                    index: member.member.index(),
                    seed: match &member.member {
                        FamilyMemberId::MonteCarloTrial { seed, .. } => Some(*seed),
                        _ => None,
                    },
                    label: member.member.label(),
                })
                .collect(),
            member_measurements.len(),
            true,
        )
    };
    if row_count == 0 {
        return None;
    }

    // A sampled variable can be laid beside a measurement only when the
    // retained rows are exactly the sampled ordinals.
    let variables_paired = !rows_are_members
        || (variables_agree
            && sample_count == Some(row_count)
            && trials
                .iter()
                .enumerate()
                .all(|(row, trial)| trial.index == row));

    let mut columns = Vec::with_capacity(variables.len() + 4);
    for variable in variables {
        let values = if variables_paired && variable.samples.len() == row_count {
            variable.samples.iter().copied().map(Some).collect()
        } else if !rows_are_members {
            variable
                .samples
                .iter()
                .copied()
                .map(Some)
                .chain(std::iter::repeat_n(None, row_count))
                .take(row_count)
                .collect()
        } else {
            vec![None; row_count]
        };
        columns.push(PopulationColumn {
            name: variable.name.clone(),
            kind: ColumnKind::SampledVariable,
            unit: String::new(),
            values,
            limit: None,
        });
    }

    // Measurement columns, in the order the first trial that carries them
    // states them.
    let mut measured_names: Vec<String> = Vec::new();
    for member in member_measurements {
        for evidence in &member.measurements {
            if !measured_names
                .iter()
                .any(|name| name.eq_ignore_ascii_case(&evidence.name))
            {
                measured_names.push(evidence.name.clone());
            }
        }
    }
    for name in measured_names {
        let spec = workspace
            .specs
            .iter()
            .find(|spec| spec.measurement.eq_ignore_ascii_case(&name));
        let values = member_measurements
            .iter()
            .map(|member| {
                member
                    .evidence_for(&name)
                    .filter(|evidence| evidence.is_measured())
                    .and_then(|evidence| evidence.value)
                    .filter(|value| value.is_finite())
            })
            .collect::<Vec<_>>();
        columns.push(PopulationColumn {
            name,
            kind: ColumnKind::Measurement,
            unit: spec.map(|spec| spec.unit.clone()).unwrap_or_default(),
            limit: spec.and_then(population_limit),
            values,
        });
    }

    let status = (0..row_count)
        .map(|row| trial_status(&columns, row))
        .collect();

    Some(PopulationPlan {
        version,
        analysis: key,
        specs_revision: specs_revision(workspace),
        trials,
        columns,
        status,
        seed: *seed,
        runs_requested: *runs_requested,
        runs_completed: *runs_completed,
        failures: *failures,
        variables_paired,
    })
}

fn population_limit(spec: &SpecEntry) -> Option<PopulationLimit> {
    (spec.min.is_some() || spec.max.is_some()).then(|| PopulationLimit {
        min: spec.min,
        max: spec.max,
        text: spec.limit_text(),
    })
}

fn trial_status(columns: &[PopulationColumn], row: usize) -> TrialStatus {
    let mut bounded = 0_usize;
    let mut unmeasured = 0_usize;
    for column in columns {
        let Some(limit) = column.limit.as_ref() else {
            continue;
        };
        bounded += 1;
        match column.values.get(row).copied().flatten() {
            Some(value) if !limit.passes(value) => return TrialStatus::Failing,
            Some(_) => {}
            None => unmeasured += 1,
        }
    }
    if bounded > 0 && unmeasured == bounded {
        TrialStatus::Unmeasured
    } else {
        TrialStatus::Passing
    }
}

// ---------------------------------------------------------------------------
// descriptive statistics
// ---------------------------------------------------------------------------

/// Ascending copy of a sample set, non-finite values dropped.
pub(super) fn sorted(values: &[f64]) -> Vec<f64> {
    let mut sorted = values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect::<Vec<_>>();
    sorted.sort_by(f64::total_cmp);
    sorted
}

pub(super) fn mean(values: &[f64]) -> Option<f64> {
    (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
}

/// Sample standard deviation (n − 1). `None` below two samples, where the
/// quantity is not defined rather than zero.
pub(super) fn std_dev(values: &[f64]) -> Option<f64> {
    if values.len() < 2 {
        return None;
    }
    let mean = mean(values)?;
    let variance = values
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f64>()
        / (values.len() - 1) as f64;
    variance.is_finite().then(|| variance.sqrt())
}

/// Linear-interpolation quantile over an already sorted set (the type-7
/// definition R and NumPy both default to).
pub(super) fn quantile(sorted: &[f64], fraction: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    if sorted.len() == 1 {
        return Some(sorted[0]);
    }
    let position = fraction.clamp(0.0, 1.0) * (sorted.len() - 1) as f64;
    let lower = position.floor() as usize;
    let upper = position.ceil() as usize;
    let weight = position - lower as f64;
    Some(sorted[lower] * (1.0 - weight) + sorted[upper] * weight)
}

/// Quartiles, the inter-quartile range, and the whiskers of one column.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct BoxStatistics {
    pub(super) count: usize,
    pub(super) minimum: f64,
    pub(super) maximum: f64,
    pub(super) q1: f64,
    pub(super) median: f64,
    pub(super) q3: f64,
    pub(super) mean: f64,
    pub(super) whisker_low: f64,
    pub(super) whisker_high: f64,
}

impl BoxStatistics {
    pub(super) fn iqr(self) -> f64 {
        self.q3 - self.q1
    }
}

/// How far the whiskers reach.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum Whiskers {
    /// Tukey: the furthest sample within 1.5 IQR of the quartiles.
    #[default]
    Tukey,
    /// The extremes of the sample set; nothing is an outlier.
    Extremes,
}

pub(super) fn box_statistics(sorted: &[f64], whiskers: Whiskers) -> Option<BoxStatistics> {
    if sorted.is_empty() {
        return None;
    }
    let q1 = quantile(sorted, 0.25)?;
    let median = quantile(sorted, 0.5)?;
    let q3 = quantile(sorted, 0.75)?;
    let minimum = sorted[0];
    let maximum = sorted[sorted.len() - 1];
    let (whisker_low, whisker_high) = match whiskers {
        Whiskers::Extremes => (minimum, maximum),
        Whiskers::Tukey => {
            let reach = 1.5 * (q3 - q1);
            let low = sorted
                .iter()
                .copied()
                .find(|value| *value >= q1 - reach)
                .unwrap_or(minimum);
            let high = sorted
                .iter()
                .copied()
                .rev()
                .find(|value| *value <= q3 + reach)
                .unwrap_or(maximum);
            (low, high)
        }
    };
    Some(BoxStatistics {
        count: sorted.len(),
        minimum,
        maximum,
        q1,
        median,
        q3,
        mean: mean(sorted)?,
        whisker_low,
        whisker_high,
    })
}

/// The Wilson score interval for a proportion, at 95 %.
///
/// Not the normal approximation: a yield of 100 % over a thousand trials has
/// a zero-width normal interval, which reads as certainty the sample cannot
/// support. Wilson keeps a bound there, which is the whole reason a yield
/// figure carries an interval at all.
pub(super) fn wilson_interval(passing: usize, total: usize) -> Option<(f64, f64)> {
    if total == 0 {
        return None;
    }
    const Z: f64 = 1.959_963_984_540_054;
    let n = total as f64;
    let p = passing as f64 / n;
    let denominator = 1.0 + Z * Z / n;
    let centre = (p + Z * Z / (2.0 * n)) / denominator;
    let spread = Z * ((p * (1.0 - p) / n) + Z * Z / (4.0 * n * n)).sqrt() / denominator;
    Some((
        ((centre - spread) * 100.0).clamp(0.0, 100.0),
        ((centre + spread) * 100.0).clamp(0.0, 100.0),
    ))
}

/// Process capability against the retained bound: two-sided when the
/// requirement is, one-sided when it is not.
pub(super) fn cpk(values: &[f64], limit: &PopulationLimit) -> Option<f64> {
    let mean = mean(values)?;
    let sigma = std_dev(values)?;
    if sigma <= 0.0 {
        return None;
    }
    let lower = limit.min.map(|min| (mean - min) / (3.0 * sigma));
    let upper = limit.max.map(|max| (max - mean) / (3.0 * sigma));
    match (lower, upper) {
        (Some(lower), Some(upper)) => Some(lower.min(upper)),
        (Some(only), None) | (None, Some(only)) => Some(only),
        (None, None) => None,
    }
}

/// Pearson's r over paired samples.
pub(super) fn pearson(xs: &[f64], ys: &[f64]) -> Option<f64> {
    if xs.len() != ys.len() || xs.len() < 2 {
        return None;
    }
    let (mean_x, mean_y) = (mean(xs)?, mean(ys)?);
    let mut covariance = 0.0;
    let mut variance_x = 0.0;
    let mut variance_y = 0.0;
    for (x, y) in xs.iter().zip(ys.iter()) {
        let (dx, dy) = (x - mean_x, y - mean_y);
        covariance += dx * dy;
        variance_x += dx * dx;
        variance_y += dy * dy;
    }
    let denominator = (variance_x * variance_y).sqrt();
    (denominator > 0.0).then(|| (covariance / denominator).clamp(-1.0, 1.0))
}

/// Ordinary least squares: `(slope, intercept)` of y on x.
pub(super) fn least_squares(xs: &[f64], ys: &[f64]) -> Option<(f64, f64)> {
    if xs.len() != ys.len() || xs.len() < 2 {
        return None;
    }
    let (mean_x, mean_y) = (mean(xs)?, mean(ys)?);
    let mut covariance = 0.0;
    let mut variance_x = 0.0;
    for (x, y) in xs.iter().zip(ys.iter()) {
        covariance += (x - mean_x) * (y - mean_y);
        variance_x += (x - mean_x).powi(2);
    }
    (variance_x > 0.0).then(|| {
        let slope = covariance / variance_x;
        (slope, mean_y - slope * mean_x)
    })
}

/// Silverman's rule-of-thumb bandwidth for a Gaussian kernel.
pub(super) fn silverman_bandwidth(sorted: &[f64]) -> Option<f64> {
    let sigma = std_dev(sorted)?;
    let iqr = quantile(sorted, 0.75)? - quantile(sorted, 0.25)?;
    let spread = if iqr > 0.0 {
        sigma.min(iqr / 1.349)
    } else {
        sigma
    };
    let bandwidth = 0.9 * spread * (sorted.len() as f64).powf(-0.2);
    (bandwidth.is_finite() && bandwidth > 0.0).then_some(bandwidth)
}

/// Gaussian kernel density at one point.
pub(super) fn kernel_density(sorted: &[f64], bandwidth: f64, at: f64) -> f64 {
    if sorted.is_empty() || bandwidth <= 0.0 {
        return 0.0;
    }
    const NORMALIZER: f64 = 0.398_942_280_401_432_7; // 1 / sqrt(2π)
    let sum: f64 = sorted
        .iter()
        .map(|value| {
            let z = (at - value) / bandwidth;
            NORMALIZER * (-0.5 * z * z).exp()
        })
        .sum();
    sum / (sorted.len() as f64 * bandwidth)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisType, FamilyMeasurementEvidence, FamilyMemberMeasurements,
        MonteCarloVariableMetadata, SpecPointScope,
    };

    fn variable(name: &str, samples: Vec<f64>) -> MonteCarloVariableMetadata {
        let count = samples.len() as f64;
        let mean = samples.iter().sum::<f64>() / count;
        let variance = samples.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (count - 1.0);
        MonteCarloVariableMetadata {
            name: name.to_owned(),
            mean,
            std_dev: variance.sqrt(),
            min: samples.iter().copied().fold(f64::INFINITY, f64::min),
            max: samples.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            samples,
        }
    }

    fn trial(index: usize, gain: f64) -> FamilyMemberMeasurements {
        FamilyMemberMeasurements::new(
            FamilyMemberId::MonteCarloTrial {
                index,
                seed: 0x73a4 + index as u64,
            },
            vec![FamilyMeasurementEvidence {
                name: "gain_dc".to_owned(),
                value: Some(gain),
                passed: true,
                error: None,
            }],
        )
    }

    fn monte_carlo(members: Vec<FamilyMemberMeasurements>, samples: Vec<f64>) -> AnalysisResult {
        let completed = members.len();
        AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
            AnalysisResultFamilyMetadata::MonteCarlo {
                seed: 0x73a4,
                runs_requested: completed,
                runs_completed: completed,
                failures: 0,
                all_converged: true,
                variables: vec![variable("RGAIN.r", samples)],
                member_measurements: members,
            },
        )
    }

    fn workspace_with_limit(min: Option<f64>, max: Option<f64>) -> ProjectWorkspace {
        let mut workspace = ProjectWorkspace::default();
        workspace.specs.push(SpecEntry {
            measurement: "gain_dc".to_owned(),
            expression: String::new(),
            min,
            max,
            unit: "dB".to_owned(),
            scope: SpecPointScope::AllPoints,
        });
        workspace
    }

    fn key() -> AnalysisPresentationKey {
        AnalysisPresentationKey::new(
            crate::product::DatasetId::new(),
            &AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC"),
        )
    }

    /// A complete run pairs its sampled variables with its measurements, and
    /// the requirement decides which trials failed.
    #[test]
    fn a_complete_run_pairs_its_variables_with_its_measurements() {
        let analysis = monte_carlo(
            vec![trial(0, 40.0), trial(1, 39.0), trial(2, 41.0)],
            vec![1.0, 2.0, 3.0],
        );
        let plan = build(&analysis, key(), 1, &workspace_with_limit(Some(39.5), None))
            .expect("the fixture retains a population");

        assert!(plan.variables_paired);
        assert_eq!(plan.trial_count(), 3);
        assert_eq!(
            plan.columns[plan.column_index("RGAIN.r").expect("sampled column")].values,
            [Some(1.0), Some(2.0), Some(3.0)]
        );
        assert_eq!(
            plan.status,
            [
                TrialStatus::Passing,
                TrialStatus::Failing,
                TrialStatus::Passing
            ]
        );
        assert_eq!(plan.failing_count(), 1);
        assert_eq!(
            plan.columns[plan.column_index("gain_dc").expect("measured column")].unit,
            "dB",
            "the unit comes from the requirement that bounds it"
        );
    }

    /// A run that dropped a trial has no correspondence between its sampled
    /// variables and its measurements, and must not invent one.
    #[test]
    fn a_run_that_dropped_a_trial_refuses_to_pair_variables_with_measurements() {
        // The driver requested four and retained trials 0, 1 and 3.
        let analysis = monte_carlo(
            vec![trial(0, 40.0), trial(1, 40.5), trial(3, 41.0)],
            vec![1.0, 2.0, 3.0],
        );
        let plan = build(&analysis, key(), 1, &ProjectWorkspace::default())
            .expect("a population is built");

        assert!(
            !plan.variables_paired,
            "trial 3 is not sample 2, and the sheet must not pretend it is"
        );
        assert!(
            plan.columns[plan
                .column_index("RGAIN.r")
                .expect("the sampled column is still listed")]
            .values
            .iter()
            .all(Option::is_none),
            "an unpairable sampled column carries no per-trial value"
        );
        let measured = &plan.columns[plan.column_index("gain_dc").expect("measured column")];
        let variable = &plan.columns[plan.column_index("RGAIN.r").expect("sampled column")];
        assert!(!plan.columns_are_paired(variable, measured));
        assert!(plan.columns_are_paired(measured, measured));
    }

    /// A trial whose measurement produced no number is evidence the analysis
    /// could not take — never a failure.
    #[test]
    fn an_unmeasured_trial_is_not_a_failing_trial() {
        let mut members = vec![trial(0, 40.0), trial(1, 40.0)];
        members[1].measurements[0].value = None;
        members[1].measurements[0].passed = false;
        let analysis = monte_carlo(members, vec![1.0, 2.0]);
        let plan = build(&analysis, key(), 1, &workspace_with_limit(Some(39.5), None))
            .expect("a population is built");

        assert_eq!(plan.status, [TrialStatus::Passing, TrialStatus::Unmeasured]);
        assert_eq!(plan.failing_count(), 0);
    }

    /// The quartiles are the interpolated definition, and Tukey whiskers stop
    /// at the furthest sample inside 1.5 IQR rather than at the fence itself.
    #[test]
    fn the_box_is_the_interpolated_quartiles_with_tukey_whiskers() {
        let values = sorted(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 100.0]);
        let statistics = box_statistics(&values, Whiskers::Tukey).expect("ten samples");

        assert!((statistics.q1 - 3.25).abs() < 1.0e-12, "{statistics:?}");
        assert!((statistics.median - 5.5).abs() < 1.0e-12);
        assert!((statistics.q3 - 7.75).abs() < 1.0e-12, "{statistics:?}");
        assert!(
            (statistics.whisker_high - 9.0).abs() < 1.0e-12,
            "{statistics:?}"
        );
        assert!((statistics.whisker_low - 1.0).abs() < 1.0e-12);
        assert!((statistics.maximum - 100.0).abs() < 1.0e-12);

        let extremes = box_statistics(&values, Whiskers::Extremes).expect("ten samples");
        assert!((extremes.whisker_high - 100.0).abs() < 1.0e-12);
    }

    /// A perfect yield keeps a confidence bound: the interval is Wilson's,
    /// not the normal approximation that collapses to zero width there.
    #[test]
    fn a_perfect_yield_still_carries_an_interval() {
        let (low, high) = wilson_interval(1_000, 1_000).expect("a thousand trials");
        assert!(low > 99.0 && low < 100.0, "{low}");
        assert!((high - 100.0).abs() < 1.0e-9, "{high}");

        let (low, high) = wilson_interval(986, 1_000).expect("a thousand trials");
        assert!(low > 97.6 && low < 98.7, "{low}");
        assert!(high > 98.5 && high < 99.3, "{high}");
    }

    /// Three sigma of margin to the nearest bound is Cpk 1, in one direction
    /// or in both.
    #[test]
    fn three_sigma_of_margin_is_a_capability_of_one() {
        let values: Vec<f64> = (0..1_001)
            .map(|index| (index as f64 - 500.0) / 500.0)
            .collect();
        let sigma = std_dev(&values).expect("a spread");
        let limit = PopulationLimit {
            min: Some(-3.0 * sigma),
            max: None,
            text: String::new(),
        };
        let capability = cpk(&values, &limit).expect("a capability");
        assert!((capability - 1.0).abs() < 1.0e-9, "{capability}");

        let two_sided = PopulationLimit {
            min: Some(-3.0 * sigma),
            max: Some(6.0 * sigma),
            text: String::new(),
        };
        let capability = cpk(&values, &two_sided).expect("a capability");
        assert!((capability - 1.0).abs() < 1.0e-9, "the tighter side wins");
    }

    /// The margin percentage is measured against the half-width of a
    /// two-sided requirement and against the bound of a one-sided one.
    #[test]
    fn the_margin_percentage_is_measured_against_the_requirement_it_belongs_to() {
        let window = PopulationLimit {
            min: Some(-50.0),
            max: Some(50.0),
            text: String::new(),
        };
        assert_eq!(window.margin_percent(0.0), Some(100.0));
        assert_eq!(window.margin_percent(50.0), Some(0.0));
        assert_eq!(window.margin_percent(75.0), Some(-50.0));

        let floor = PopulationLimit {
            min: Some(95.0),
            max: None,
            text: String::new(),
        };
        assert_eq!(floor.margin_percent(95.0), Some(0.0));
        let above = floor.margin_percent(190.0).expect("a percentage");
        assert!((above - 100.0).abs() < 1.0e-12);
    }

    /// A straight line correlates perfectly and its fit recovers the line.
    #[test]
    fn the_fit_recovers_the_line_it_was_given() {
        let xs: Vec<f64> = (0..50).map(f64::from).collect();
        let ys: Vec<f64> = xs.iter().map(|x| 3.5 * x - 7.25).collect();
        assert!((pearson(&xs, &ys).expect("a correlation") - 1.0).abs() < 1.0e-12);
        let (slope, intercept) = least_squares(&xs, &ys).expect("a fit");
        assert!((slope - 3.5).abs() < 1.0e-12);
        assert!((intercept + 7.25).abs() < 1.0e-12);

        // A column with no spread has no correlation to report, and saying
        // "0" would read as "measured and uncorrelated".
        let flat = vec![2.0; 50];
        assert_eq!(pearson(&flat, &ys), None);
        assert_eq!(least_squares(&flat, &ys), None);
    }

    /// The kernel density integrates to one over the sample range, which is
    /// what makes one violin's width comparable with the next.
    #[test]
    fn the_kernel_density_is_a_density() {
        let values = sorted(
            &(0..201)
                .map(|i| f64::from(i) / 100.0 - 1.0)
                .collect::<Vec<_>>(),
        );
        let bandwidth = silverman_bandwidth(&values).expect("a bandwidth");
        let (low, high) = (-2.0, 2.0);
        let steps = 4_000;
        let step = (high - low) / f64::from(steps);
        let area: f64 = (0..steps)
            .map(|index| {
                let at = low + step * f64::from(index) + step / 2.0;
                kernel_density(&values, bandwidth, at) * step
            })
            .sum();
        assert!(
            (area - 1.0).abs() < 1.0e-3,
            "the density integrates to {area}"
        );
    }
}
