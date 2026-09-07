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
    AnalysisResult, AnalysisResultFamilyMetadata, FamilyMemberId, PreparedSpecification,
    ProjectWorkspace, RunHistoryRevision, SpecEntry, SpecPointScope,
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
    specification: PreparedSpecification,
    /// The bound spelled the way every other surface spells it.
    pub(super) text: String,
}

impl PopulationLimit {
    pub(super) fn passes(&self, value: f64) -> bool {
        value.is_finite() && self.signed_margin(value).is_none_or(|margin| margin >= 0.0)
    }

    /// Signed distance to the nearest bound: positive inside, negative out.
    /// The same quantity the Specs sheet calls the margin.
    pub(super) fn signed_margin(&self, value: f64) -> Option<f64> {
        self.specification.signed_margin(value)
    }

    /// Approximate plot coordinates of the acceptance edges. Verdicts always
    /// use the exact margin above, not these rounded display coordinates.
    pub(super) fn min(&self) -> Option<f64> {
        self.specification
            .entry()
            .min
            .map(|value| value + self.specification.guard_band())
            .filter(|value| value.is_finite())
    }

    pub(super) fn max(&self) -> Option<f64> {
        self.specification
            .entry()
            .max
            .map(|value| value - self.specification.guard_band())
            .filter(|value| value.is_finite())
    }

    pub(super) fn normalized(&self) -> Self {
        let entry = SpecEntry {
            min: Some(0.0),
            max: None,
            ..self.specification.entry().clone()
        };
        Self {
            specification: PreparedSpecification::new(entry)
                .expect("a zero bound preserves validated requirement fields"),
            text: self.text.clone(),
        }
    }

    #[cfg(test)]
    pub(super) fn for_test(min: Option<f64>, max: Option<f64>) -> Self {
        population_limit(
            &PreparedSpecification::new(SpecEntry {
                measurement: "test".into(),
                expression: String::new(),
                min,
                max,
                unit: String::new(),
                scope: SpecPointScope::AllPoints,
            })
            .unwrap(),
        )
        .unwrap()
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
        let reference = match (
            self.specification.entry().min,
            self.specification.entry().max,
        ) {
            (Some(min), Some(max)) => (max - min) / 2.0 - self.specification.guard_band(),
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
    /// No applicable bounded requirement judges this trial.
    NotEvaluated,
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
    source: (RunHistoryRevision, u64),
    analysis: AnalysisPresentationKey,
    requirements: Option<Vec<PopulationRequirement>>,
    pub(super) requirement_note: String,
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

/// Resolve the population from the retained source, selected analysis, and
/// exact requirement inputs, then hand back a shared handle.
pub(super) fn plan(context: &mut SheetContext<'_>) -> Option<Arc<PopulationPlan>> {
    let run = context.simulation.active_run()?;
    let dataset_id = run.dataset_id;
    let analysis = context.simulation.active_analysis()?;
    let key = AnalysisPresentationKey::new(dataset_id, analysis);
    let source = (
        context.simulation.runs.revision(),
        context.simulation.data_version,
    );
    if let Some(plan) = context.results.plans.population.as_ref()
        && plan.source == source
        && plan.analysis == key
        && plan.requirements.as_ref().is_none_or(|requirements| {
            requirements.len() == context.workspace.specs.len()
                && requirements
                    .iter()
                    .zip(&context.workspace.specs)
                    .all(|(retained, spec)| retained.matches(spec))
        })
    {
        return Some(Arc::clone(plan));
    }
    context.results.plans.population = None;
    let built = Arc::new(build(
        analysis,
        key,
        source,
        context.workspace,
        run.prepared_receipt()
            .map(|receipt| receipt.specifications()),
    )?);
    context.results.plans.population = Some(Arc::clone(&built));
    Some(built)
}

/// Exactly the authored fields this projection reads, in requirement order.
/// Comparing the small requirement list allocates nothing on a cache hit and
/// avoids hash collisions. Bound bits preserve both optionality and NaN
/// identity while a requirement is being edited. This snapshot is needed only
/// for legacy runs; a prepared run's contract belongs to its history revision.
#[derive(Debug, Clone)]
struct PopulationRequirement {
    measurement: String,
    expression: String,
    min: Option<u64>,
    max: Option<u64>,
    unit: String,
    scope: SpecPointScope,
}

impl PopulationRequirement {
    fn capture(spec: &SpecEntry) -> Self {
        let SpecEntry {
            measurement,
            expression,
            min,
            max,
            unit,
            scope,
        } = spec;
        Self {
            measurement: measurement.clone(),
            expression: expression.clone(),
            min: min.map(f64::to_bits),
            max: max.map(f64::to_bits),
            unit: unit.clone(),
            scope: scope.clone(),
        }
    }

    fn matches(&self, spec: &SpecEntry) -> bool {
        let SpecEntry {
            measurement,
            expression,
            min,
            max,
            unit,
            scope,
        } = spec;
        self.measurement == *measurement
            && self.expression == *expression
            && self.min == min.map(f64::to_bits)
            && self.max == max.map(f64::to_bits)
            && self.unit == *unit
            && self.scope == *scope
    }
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
    source: (RunHistoryRevision, u64),
    workspace: &ProjectWorkspace,
    prepared: Option<&[PreparedSpecification]>,
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

    let requirements = applicable_requirements(analysis, &workspace.specs, prepared);
    let requirements_valid = requirements.is_ok();
    let requirement_note = match &requirements {
        Err(error) => {
            format!("Requirements unavailable: {error}. No pass/fail verdict is available.")
        }
        Ok(_) if prepared.is_some() => "Requirements retained with this run.".to_owned(),
        Ok(_) => {
            "Current workspace requirements; this legacy run has no retained requirement contract."
                .to_owned()
        }
    };
    let resolved = requirements.unwrap_or_default();

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
    for requirement in &resolved {
        let name = &requirement.entry().measurement;
        if !measured_names
            .iter()
            .any(|measured| measured.eq_ignore_ascii_case(name))
        {
            measured_names.push(name.clone());
        }
    }
    for name in measured_names {
        let spec = resolved
            .iter()
            .find(|spec| spec.entry().measurement.eq_ignore_ascii_case(&name));
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
            unit: spec
                .map(|spec| spec.entry().unit.clone())
                .unwrap_or_default(),
            limit: spec.and_then(population_limit),
            values,
        });
    }

    let status = (0..row_count)
        .map(|row| {
            if requirements_valid {
                trial_status(&columns, row)
            } else {
                TrialStatus::Unmeasured
            }
        })
        .collect();

    Some(PopulationPlan {
        source,
        analysis: key,
        requirements: prepared.is_none().then(|| {
            workspace
                .specs
                .iter()
                .map(PopulationRequirement::capture)
                .collect()
        }),
        requirement_note,
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

fn applicable_requirements(
    analysis: &AnalysisResult,
    entries: &[SpecEntry],
    prepared: Option<&[PreparedSpecification]>,
) -> Result<Vec<PreparedSpecification>, String> {
    if let Some(prepared) = prepared {
        return Ok(prepared
            .iter()
            .filter(|spec| spec.admits_analysis(analysis))
            .cloned()
            .collect());
    }
    let mut names = std::collections::HashSet::new();
    let mut applicable = Vec::new();
    for entry in entries {
        let spec = PreparedSpecification::new(entry.clone())
            .map_err(|error| format!("{}: {error}", entry.measurement))?;
        if !names.insert(entry.measurement.to_ascii_lowercase()) {
            return Err(format!("duplicate measurement '{}'", entry.measurement));
        }
        if entry.scope.admits(
            analysis
                .provenance()
                .and_then(|provenance| provenance.pvt_point()),
        ) {
            applicable.push(spec);
        }
    }
    Ok(applicable)
}

fn population_limit(spec: &PreparedSpecification) -> Option<PopulationLimit> {
    let entry = spec.entry();
    (entry.min.is_some() || entry.max.is_some()).then(|| {
        let mut text = entry.limit_text();
        if spec.guard_band() > 0.0 {
            text.push_str(&format!(
                " · guard band {} {}",
                crate::state::format_engineering_display(spec.guard_band()),
                entry.unit
            ));
        }
        PopulationLimit {
            specification: spec.clone(),
            text,
        }
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
    if unmeasured > 0 {
        TrialStatus::Unmeasured
    } else if bounded == 0 {
        TrialStatus::NotEvaluated
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
    limit
        .signed_margin(mean)
        .map(|margin| margin / (3.0 * sigma))
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
mod tests;
