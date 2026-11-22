//! HIST — distributions rebuilt from retained samples, with descriptive
//! moments, shape and specification evidence bound to the same population.

#[cfg(test)]
mod source_tests;

use egui::Ui;
use std::borrow::Cow;
use std::sync::Arc;

use super::population::{self, PopulationPlan};
use super::{ResultsParts, ResultsReads, SheetContext};
use crate::product::DatasetId;
use crate::results::visualization_document::{HistogramReading, SheetSettingValue};
use crate::source_revision::SourceRevision;
use crate::state::{
    AnalysisResultFamilyMetadata, AnalysisType, MonteCarloVariableMetadata, SimulationState,
};
use rspice_results::histogram::{Histogram, SampleMoments};
use rspice_results::histogram::{HistogramBuilder, HistogramDisplayMode};
use rspice_results::population::{ColumnKind, PopulationColumn};
use rspice_results::yield_analysis::{SpecLimitType, YieldResult};
use rspice_results_ui::histogram::display::HistogramDisplay;
use rspice_results_ui::histogram::view::{
    self, HistogramInspector, HistogramPlot, HistogramUnavailable, LimitSource, MonteCarloMethod,
    SampleShape, SpecLimits, YieldCount,
};

use super::frame_work::{self, DatasetWalk};

/// Return the one yield result authorized by both the active immutable
/// dataset and the selected Monte-Carlo measurement. Yield results are stored
/// as a flat collection, so selecting the first entry can silently apply the
/// limits and verdict for another variable. Duplicate target evidence is also
/// rejected: the UI must not choose an arbitrary result when retained state is
/// malformed or came from an older, less constrained schema.
///
/// This half is cheap — name matching over the retained result list. Whether
/// the result is internally consistent walks the whole retained population,
/// and that verdict comes from the plan instead.
fn matching_yield_result<'a>(
    simulation: &'a SimulationState,
    histogram_name: &str,
) -> Option<&'a YieldResult> {
    let results = simulation.yield_results_for_active_dataset()?;
    let mut matches = results
        .iter()
        .filter(|result| yield_target_measurement(&result.spec.target) == histogram_name);
    let result = matches.next()?;
    matches.next().is_none().then_some(result)
}

#[cfg(test)]
fn selected_yield_result<'a>(
    simulation: &'a SimulationState,
    histogram_name: &str,
    consistent: bool,
) -> Option<&'a YieldResult> {
    consistent
        .then(|| matching_yield_result(simulation, histogram_name))
        .flatten()
}

/// `mean(name)` is an accepted Monte-Carlo yield target spelling and refers
/// to the complete retained sample population named `name`. Keep this
/// canonicalization byte-for-byte compatible with the yield engine so display
/// authority never broadens beyond evidence the engine actually evaluated.
fn yield_target_measurement(target: &str) -> &str {
    let target = target.trim();
    const PREFIX: &str = "mean";
    if target.len() > PREFIX.len() + 2
        && target
            .get(..PREFIX.len())
            .is_some_and(|head| head.eq_ignore_ascii_case(PREFIX))
        && target
            .get(PREFIX.len()..)
            .is_some_and(|tail| tail.starts_with('('))
        && target.ends_with(')')
    {
        &target[PREFIX.len() + 1..target.len() - 1]
    } else {
        target
    }
}

fn nearly_equal(left: f64, right: f64) -> bool {
    if left == right {
        return left.is_finite();
    }
    let scale = left.abs().max(right.abs());
    (left / scale - right / scale).abs() <= 128.0 * f64::EPSILON
}

fn moments_from_samples(samples: &[f64]) -> Option<SampleMoments> {
    frame_work::note(DatasetWalk::HistMoments);
    SampleMoments::from_samples(samples)
}

fn moments_match(
    moments: SampleMoments,
    count: usize,
    mean: f64,
    std_dev: f64,
    min: f64,
    max: f64,
) -> bool {
    moments.count == count
        && nearly_equal(moments.mean, mean)
        && nearly_equal(moments.std_dev, std_dev)
        && nearly_equal(moments.min, min)
        && nearly_equal(moments.max, max)
}

fn yield_result_is_consistent(result: &YieldResult) -> bool {
    frame_work::note(DatasetWalk::HistMoments);
    let spec = &result.spec;
    let limits_valid = match spec.limit_type {
        SpecLimitType::Lower => spec.min.is_some_and(f64::is_finite) && spec.max.is_none(),
        SpecLimitType::Upper => spec.min.is_none() && spec.max.is_some_and(f64::is_finite),
        SpecLimitType::Range => matches!(
            (spec.min, spec.max),
            (Some(min), Some(max)) if min.is_finite() && max.is_finite() && min < max
        ),
    };
    if spec.target.trim().is_empty()
        || !limits_valid
        || spec.target_val.is_some_and(|value| !value.is_finite())
        || !spec.weight.is_finite()
        || !(0.0..=1.0).contains(&spec.weight)
        || result.total_runs == 0
        || result.pass_count.saturating_add(result.fail_count) != result.total_runs
        || result.trail.len() != result.total_runs
        || result.trail.iter().filter(|passed| **passed).count() != result.pass_count
        || result.samples.len() != result.total_runs
        || result
            .samples
            .iter()
            .zip(&result.trail)
            .any(|(sample, passed)| spec.evaluates(*sample) != *passed)
        || result
            .samples
            .iter()
            .filter(|value| spec.evaluates(**value))
            .count()
            != result.pass_count
        || !result.yield_percent.is_finite()
        || !(0.0..=100.0).contains(&result.yield_percent)
        || !result.stats.median.is_finite()
        || result
            .stats
            .skewness
            .is_some_and(|value| !value.is_finite())
        || result
            .stats
            .kurtosis
            .is_some_and(|value| !value.is_finite())
        || result.stats.cp.is_some_and(|value| !value.is_finite())
        || result.stats.cpk.is_some_and(|value| !value.is_finite())
    {
        return false;
    }

    let expected_yield = result.pass_count as f64 * 100.0 / result.total_runs as f64;
    let Some(moments) = moments_from_samples(&result.samples) else {
        return false;
    };
    nearly_equal(result.yield_percent, expected_yield)
        && moments_match(
            moments,
            result.stats.count,
            result.stats.mean,
            result.stats.std_dev,
            result.stats.min,
            result.stats.max,
        )
}

#[derive(Debug, Clone, Copy)]
struct MonteCarloAuthority<'a> {
    seed: u64,
    runs_requested: usize,
    runs_completed: usize,
    failures: usize,
    variable: &'a MonteCarloVariableMetadata,
}

/// Whether the shown analysis is a successful family whose retained
/// evidence still describes the shown run.
fn family_evidence_is_valid(state: ResultsReads<'_>) -> bool {
    let simulation = &state.model.simulation;
    simulation.active_analysis().is_some_and(|analysis| {
        analysis.success
            && analysis.family_metadata.is_some()
            && simulation.active_run().is_some_and(|run| {
                super::analysis_evidence_is_valid(state, run.dataset_id, analysis)
            })
    })
}

fn active_monte_carlo_authority<'a>(
    state: ResultsReads<'a>,
    histogram_name: &str,
) -> Option<MonteCarloAuthority<'a>> {
    if !family_evidence_is_valid(state) {
        return None;
    }
    let analysis = state.model.simulation.active_analysis()?;
    let AnalysisResultFamilyMetadata::MonteCarlo {
        seed,
        runs_requested,
        runs_completed,
        failures,
        variables,
        ..
    } = analysis.family_metadata.as_ref()?
    else {
        return None;
    };
    let mut matches = variables
        .iter()
        .filter(|variable| variable.name == histogram_name);
    let variable = matches.next()?;
    if matches.next().is_some() {
        return None;
    }
    Some(MonteCarloAuthority {
        seed: *seed,
        runs_requested: *runs_requested,
        runs_completed: *runs_completed,
        failures: *failures,
        variable,
    })
}

fn exact_moments(state: ResultsReads<'_>, histogram_name: &str) -> Option<SampleMoments> {
    if state
        .model
        .simulation
        .active_analysis()
        .and_then(|analysis| analysis.family_metadata.as_ref())
        .is_some()
    {
        let authority = active_monte_carlo_authority(state, histogram_name)?;
        let variable = authority.variable;
        return moments_from_samples(&variable.samples).filter(|moments| {
            moments_match(
                *moments,
                variable.samples.len(),
                variable.mean,
                variable.std_dev,
                variable.min,
                variable.max,
            )
        });
    }

    if !legacy_yield_is_eligible(state) {
        return None;
    }
    let result = matching_yield_result(&state.model.simulation, histogram_name)
        .filter(|result| yield_result_is_consistent(result))?;
    moments_from_samples(&result.samples)
}

/// Specification limits from the reader's side, judged only where the run
/// states none: a measurement on the reading pane, else the reader's own.
/// Held as bits, so a limit being typed keys the plan exactly.
type FallbackLimits = (Option<u64>, Option<u64>, LimitSource);

fn fallback_limits(state: ResultsReads<'_>, name: &str) -> Option<FallbackLimits> {
    let bits = |(lower, upper): (Option<f64>, Option<f64>), source| {
        (lower.is_some() || upper.is_some())
            .then(|| (lower.map(f64::to_bits), upper.map(f64::to_bits), source))
    };
    kept_yield_limits(state, name)
        .and_then(|limits| bits(limits, LimitSource::Pane))
        .or_else(|| {
            let settings = &state.analysis.histogram_state;
            bits(
                (settings.lower_limit, settings.upper_limit),
                LimitSource::Reader,
            )
        })
}

/// The descriptive moments, shape and yield behind one distribution.
///
/// Each walks the retained population — the moments are four passes, the
/// shape sorts it, the yield verdict counts, filters and then recomputes the
/// moments to check the retained summary against them. The sheet and its
/// panel each asked for them on every frame.
#[derive(Debug, Clone)]
pub(super) struct HistPlan {
    version: u64,
    source: SourceRevision,
    yield_source: SourceRevision,
    dataset: Option<DatasetId>,
    analysis: Option<super::AnalysisPresentationKey>,
    histogram_name: String,
    bin_count: usize,
    range: Option<(u64, u64)>,
    population: Option<Arc<PopulationPlan>>,
    fallback: Option<FallbackLimits>,
    histogram: Option<Arc<Histogram>>,
    mode: HistogramDisplayMode,
    display: Result<Arc<HistogramDisplay>, &'static str>,
    display_source: SourceRevision,
    moments: Option<SampleMoments>,
    shape: Option<SampleShape>,
    /// The binned quantity's unit; empty when the run states none.
    unit: String,
    limits: Option<SpecLimits>,
    yield_count: Option<YieldCount>,
    method: Option<MonteCarloMethod>,
    /// The values by population row, with the analysis whose member
    /// selection those rows index, when the rows are the trials the scatter
    /// sheet brushes.
    rows: Option<(super::AnalysisPresentationKey, Vec<Option<f64>>)>,
}

/// The population's measured column named `name`, spelled as the
/// requirement spells it in any case.
fn measured_column<'a>(population: &'a PopulationPlan, name: &str) -> Option<&'a PopulationColumn> {
    population.columns.iter().find(|column| {
        column.kind == ColumnKind::Measurement && column.name.eq_ignore_ascii_case(name)
    })
}

/// Whether `column` holds `samples` row for row, so that its unit and its
/// requirement describe them. A requirement may state a measurement in
/// another unit ("mV", "dB"); a column converted to it is not these numbers.
fn column_holds(column: &PopulationColumn, samples: &[f64]) -> bool {
    column.values.len() == samples.len()
        && column.values.iter().any(Option::is_some)
        && column
            .values
            .iter()
            .zip(samples)
            .all(|(value, sample)| value.is_none_or(|value| nearly_equal(value, *sample)))
}

/// How many of `values` pass, out of every trial row. A row with no value
/// counts against the yield; trials the run requested and never completed
/// are named beside it instead.
fn count_passing(
    values: &[Option<f64>],
    passes: impl Fn(f64) -> bool,
    runs_requested: Option<usize>,
) -> YieldCount {
    frame_work::note(DatasetWalk::HistMoments);
    let total = values.len();
    YieldCount {
        passing: values
            .iter()
            .filter(|value| value.is_some_and(&passes))
            .count(),
        total,
        excluded: runs_requested.map_or(0, |requested| requested.saturating_sub(total)),
    }
}

/// Resolve bins and statistics together. Source tokens also cover nested edits,
/// wholesale replacement and clones whose numeric generations happen to match.
fn hist_plan(state: ResultsReads<'_>, histogram: &str) -> Arc<HistPlan> {
    let simulation = &state.model.simulation;
    let workspace = &state.model.workspace;
    let version = simulation.view.data_version;
    let source = simulation.retained.runs.revision();
    let yield_source = simulation.retained.yield_evidence.revision();
    let dataset = simulation.active_run().map(|run| run.dataset_id);
    let analysis = dataset
        .zip(simulation.active_analysis())
        .map(|(dataset, analysis)| super::AnalysisPresentationKey::new(dataset, analysis));
    let settings = &state.analysis.histogram_state;
    let bin_count = settings.bin_count.clamp(1, 1000);
    let range = settings
        .custom_range
        .then_some((settings.custom_min.to_bits(), settings.custom_max.to_bits()));
    let fallback = fallback_limits(state, histogram);
    if let Some(plan) = state.results.plans.hist.borrow().as_ref()
        && plan.version == version
        && plan.source == source
        && plan.yield_source == yield_source
        && plan.dataset == dataset
        && plan.analysis == analysis
        && plan.histogram_name == histogram
        && plan.mode == settings.mode
        && plan.bin_count == bin_count
        && plan.range == range
        && plan.fallback == fallback
        && plan
            .population
            .as_deref()
            .is_none_or(|population| population.is_shown(simulation, workspace))
    {
        return Arc::clone(plan);
    }
    let population = population::shown(simulation, workspace, state.results);
    let authority = active_monte_carlo_authority(state, histogram);
    let yield_result = matching_yield_result(simulation, histogram);
    let yield_is_consistent = yield_result.is_some_and(|result| {
        if !yield_result_is_consistent(result) {
            return false;
        }
        match authority {
            Some(authority) => simulation
                .retained
                .yield_provenance()
                .is_some_and(|provenance| {
                    provenance.seed == authority.seed
                        && provenance.runs_requested == authority.runs_requested
                        && provenance.runs_completed == authority.runs_completed
                        && result.samples.len() == authority.variable.samples.len()
                        && result
                            .samples
                            .iter()
                            .zip(&authority.variable.samples)
                            .all(|(a, b)| a.to_bits() == b.to_bits())
                }),
            None => legacy_yield_is_eligible(state),
        }
    });
    let engine_yield = yield_result.filter(|_| yield_is_consistent);
    let paired = |samples: &[f64]| {
        population.as_deref().is_some_and(|population| {
            population.variables_paired && population.trial_count() == samples.len()
        })
    };
    // A retained variable is the population. A measured column of the same
    // name lends it a unit and a requirement only when it holds the same
    // numbers; a name only the trials measured is that column.
    let variable = authority
        .map(|authority| authority.variable.samples.as_slice())
        .or_else(|| engine_yield.map(|result| result.samples.as_slice()));
    let column = population
        .as_deref()
        .and_then(|population| measured_column(population, histogram))
        .filter(|column| {
            variable.is_none_or(|samples| paired(samples) && column_holds(column, samples))
        });
    let (samples, values, moments): (Option<Cow<'_, [f64]>>, Vec<Option<f64>>, _) =
        match (variable, column) {
            (Some(samples), _) => (
                Some(Cow::Borrowed(samples)),
                samples.iter().copied().map(Some).collect(),
                exact_moments(state, histogram),
            ),
            (None, Some(column)) => {
                let samples: Vec<f64> = column.values.iter().flatten().copied().collect();
                let moments = moments_from_samples(&samples);
                (Some(Cow::Owned(samples)), column.values.clone(), moments)
            }
            (None, None) => (None, Vec::new(), None),
        };
    let unit = column
        .map(|column| column.unit.trim())
        .filter(|unit| !unit.is_empty())
        .or_else(|| engine_yield.map(|result| result.spec.unit.trim()))
        .unwrap_or_default()
        .to_owned();
    let method = match (authority, population.as_deref()) {
        (Some(authority), _) => Some(MonteCarloMethod {
            seed: authority.seed,
            runs_requested: authority.runs_requested,
            runs_completed: authority.runs_completed,
            failures: authority.failures,
            mean_confidence: authority.variable.mean_confidence,
        }),
        (None, Some(population)) if variable.is_none() && column.is_some() => {
            Some(MonteCarloMethod {
                seed: population.seed,
                runs_requested: population.runs_requested,
                runs_completed: population.runs_completed,
                failures: population.failures,
                mean_confidence: None,
            })
        }
        _ => None,
    };
    let requested = method.map(|method| method.runs_requested);
    let requirement = column.and_then(|column| column.limit.as_ref());
    let (limits, yield_count) = if samples.is_none() {
        (None, None)
    } else if let Some(result) = engine_yield {
        (
            Some(SpecLimits {
                lower: result.spec.min,
                upper: result.spec.max,
                source: LimitSource::Yield,
            }),
            Some(YieldCount {
                passing: result.pass_count,
                total: result.total_runs,
                excluded: requested
                    .map_or(0, |requested| requested.saturating_sub(result.total_runs)),
            }),
        )
    } else if let Some(limit) = requirement {
        (
            Some(SpecLimits {
                lower: limit.min(),
                upper: limit.max(),
                source: LimitSource::Requirement,
            }),
            Some(count_passing(
                &values,
                |value| limit.passes(value),
                requested,
            )),
        )
    } else if let Some((lower, upper, source)) = fallback {
        let (lower, upper) = (lower.map(f64::from_bits), upper.map(f64::from_bits));
        let passes = |value: f64| {
            value.is_finite()
                && lower.is_none_or(|lower| value >= lower)
                && upper.is_none_or(|upper| value <= upper)
        };
        (
            Some(SpecLimits {
                lower,
                upper,
                source,
            }),
            Some(count_passing(&values, passes, requested)),
        )
    } else {
        (None, None)
    };
    let rows = population.as_deref().and_then(|population| {
        let rows = match (variable, column) {
            (Some(samples), _) if paired(samples) => values.clone(),
            (None, Some(_)) => values.clone(),
            _ => return None,
        };
        Some((population.analysis(), rows))
    });
    let shape = samples.as_deref().and_then(|samples| {
        frame_work::note(DatasetWalk::HistMoments);
        SampleShape::of(samples)
    });
    let derived = samples.as_deref().and_then(|samples| {
        let mut builder = HistogramBuilder::new().name(histogram).bin_count(bin_count);
        if let Some((min, max)) = range {
            let (min, max) = (f64::from_bits(min), f64::from_bits(max));
            if !min.is_finite() || !max.is_finite() || min >= max {
                return None;
            }
            builder = builder.range(min, max);
        }
        frame_work::note(DatasetWalk::HistMoments);
        Some(Arc::new(builder.build(samples)))
    });
    let display = derived
        .as_deref()
        .zip(samples.as_deref())
        .ok_or("The selected distribution is unavailable")
        .and_then(|(histogram, samples)| HistogramDisplay::new(histogram, samples, settings.mode))
        .map(Arc::new);
    let built = Arc::new(HistPlan {
        version,
        source,
        yield_source,
        dataset,
        analysis,
        histogram_name: histogram.to_owned(),
        bin_count,
        range,
        population: population.clone(),
        fallback,
        histogram: derived,
        mode: settings.mode,
        display,
        display_source: SourceRevision::default(),
        moments,
        shape,
        unit,
        limits,
        yield_count,
        method,
        rows,
    });
    *state.results.plans.hist.borrow_mut() = Some(Arc::clone(&built));
    built
}

fn legacy_yield_is_eligible(state: ResultsReads<'_>) -> bool {
    state.model.simulation.active_run().is_some()
        && state
            .model
            .simulation
            .active_analysis()
            .is_none_or(|analysis| {
                analysis.success
                    && analysis.analysis_type == AnalysisType::MonteCarlo
                    && analysis.family_metadata.is_none()
            })
}

/// The names the retained run states itself: its Monte Carlo variables, or
/// a legacy run's yield targets. Never the mutable legacy bins.
fn retained_names(state: ResultsReads<'_>) -> Vec<&str> {
    if let Some(analysis) = state.model.simulation.active_analysis()
        && analysis.family_metadata.is_some()
    {
        if !family_evidence_is_valid(state) {
            return Vec::new();
        }
        return match &analysis.family_metadata {
            Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. }) => variables
                .iter()
                .map(|variable| variable.name.as_str())
                .collect(),
            _ => Vec::new(),
        };
    }
    if !legacy_yield_is_eligible(state) {
        return Vec::new();
    }
    state
        .model
        .simulation
        .yield_results_for_active_dataset()
        .unwrap_or_default()
        .iter()
        .map(|result| yield_target_measurement(&result.spec.target))
        .collect()
}

/// Every distribution the sheet offers: the run's variables, then each
/// measurement its trials produced that no variable already names.
fn histogram_names(state: ResultsReads<'_>) -> Vec<String> {
    let mut names: Vec<String> = retained_names(state)
        .into_iter()
        .map(str::to_owned)
        .collect();
    if family_evidence_is_valid(state)
        && let Some(population) = population::shown(
            &state.model.simulation,
            &state.model.workspace,
            state.results,
        )
    {
        for column in &population.columns {
            if column.kind == ColumnKind::Measurement
                && !names
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(&column.name))
            {
                names.push(column.name.clone());
            }
        }
    }
    names
}

/// The reader's significant digits, which the histogram's values read at.
fn significant_digits(state: ResultsReads<'_>) -> usize {
    usize::from(
        state
            .presentation_policy()
            .displayed_significant_digits()
            .get(),
    )
}

/// Whether the shown run has a distribution to draw. Called for every frame
/// the sheet's tab is drawn, so it reads the retained evidence directly
/// rather than resolving the population.
pub(super) fn histogram_is_available(state: ResultsReads<'_>) -> bool {
    let names = retained_names(state);
    let retained = names.iter().any(|name| {
        active_monte_carlo_authority(state, name)
            .is_some_and(|authority| !authority.variable.samples.is_empty())
            || (legacy_yield_is_eligible(state)
                && matching_yield_result(&state.model.simulation, name)
                    .is_some_and(|result| !result.samples.is_empty()))
    });
    retained
        || (family_evidence_is_valid(state)
            && state
                .model
                .simulation
                .active_analysis()
                .and_then(|analysis| analysis.family_metadata.as_ref())
                .is_some_and(|metadata| {
                    matches!(
                        metadata,
                        AnalysisResultFamilyMetadata::MonteCarlo { member_measurements, .. }
                            if member_measurements.iter().any(|member| {
                                member.measurements.iter().any(|evidence| {
                                    evidence.is_measured()
                                        && evidence.value.is_some_and(f64::is_finite)
                                })
                            })
                    )
                }))
}

pub(super) fn active_histogram(state: ResultsReads<'_>) -> Option<Arc<Histogram>> {
    let name = selected_histogram_name(state)?;
    hist_plan(state, &name).histogram.clone()
}

pub(super) fn active_histogram_display(state: ResultsReads<'_>) -> Option<Arc<HistogramDisplay>> {
    let name = selected_histogram_name(state)?;
    hist_plan(state, &name)
        .display
        .as_ref()
        .ok()
        .map(Arc::clone)
}

/// The distribution the reader has selected, by name.
fn selected_histogram_name(state: ResultsReads<'_>) -> Option<String> {
    selected_name(state, &histogram_names(state))
}

fn selected_name(state: ResultsReads<'_>, names: &[String]) -> Option<String> {
    let names: Vec<&str> = names.iter().map(String::as_str).collect();
    let index = rspice_results_ui::histogram::state::measurement_index(
        state.analysis.histogram_state.selected.as_deref(),
        0,
        &names,
    )?;
    Some(names[index].to_owned())
}

/// The limits of the yield the reader last kept on the reading pane for
/// `name` itself: a family measurement of exactly that variable, judged
/// against a lower and/or upper limit. An engine yield's limits, when the
/// run judged one, are drawn instead.
fn kept_yield_limits(state: ResultsReads<'_>, name: &str) -> Option<(Option<f64>, Option<f64>)> {
    let document_id = super::document_edit::active_document_id(state)?;
    let (_, pane_id) = super::persistent_document::reading_position(state, document_id)?;
    state
        .model
        .workspace
        .content
        .visualization_document(document_id)?
        .measurements()
        .iter()
        .rev()
        .filter(|measurement| measurement.pane_id == pane_id)
        .filter(|measurement| {
            measurement
                .expression
                .as_deref()
                .is_some_and(|expression| expression.trim().eq_ignore_ascii_case(name))
        })
        .filter_map(|measurement| measurement.family.as_ref())
        .find(|family| family.lower.is_some() || family.upper.is_some())
        .map(|family| (family.lower, family.upper))
}

/// The trial rows a pick in the histogram selects: the trials whose value
/// the picked bin counts, by the same edge rule that counted them.
fn picked_rows(
    values: &[Option<f64>],
    histogram: &Histogram,
    picked: view::HistogramPick,
) -> Vec<usize> {
    match picked {
        view::HistogramPick::Bin(bin) => values
            .iter()
            .enumerate()
            .filter(|(_, value)| value.is_some_and(|value| histogram.bin_of(value) == Some(bin)))
            .map(|(row, _)| row)
            .collect(),
        view::HistogramPick::Clear => Vec::new(),
    }
}

/// Resolve the shown population once for the frame, into the memo the
/// scatter and box sheets share, so every read below finds it current.
fn resolve_population(state: &mut ResultsParts<'_>) {
    let _ = population::plan(&mut SheetContext::of(state));
}

/// Take the reading the view keeps for the shown analysis.
fn take_kept_reading(state: &mut ResultsParts<'_>) {
    if let Some(SheetSettingValue::Histogram(reading)) =
        super::sheet_settings::kept(state.results_reads(), super::ResultViewer::Hist)
        && reading != state.analysis.histogram_state.reading()
    {
        state.analysis.histogram_state.apply(&reading);
    }
}

/// Keep what the reader changed since `before`, so the view reopens on it.
fn keep_reading(state: &mut ResultsParts<'_>, before: &HistogramReading) {
    let reading = state.analysis.histogram_state.reading();
    if reading != *before {
        super::sheet_settings::keep(
            state,
            super::ResultViewer::Hist,
            Some(SheetSettingValue::Histogram(reading)),
        );
    }
}

/// The sheet's controls in its bar; `false` when there is no distribution
/// to choose from.
pub(super) fn domain_bar(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    resolve_population(state);
    take_kept_reading(state);
    let names = histogram_names(state.results_reads());
    if names.is_empty() {
        return false;
    }
    if state.analysis.histogram_state.selected.is_none() {
        state.analysis.histogram_state.selected = names.first().cloned();
    }
    let before = state.analysis.histogram_state.reading();
    if view::domain_bar(ui, &mut state.analysis.histogram_state, &names) {
        state
            .results
            .session
            .reset_plot_view(super::ResultViewer::Hist, 0);
    }
    keep_reading(state, &before);
    true
}

/// Resolve the current population and draw it, applying local plot outcomes.
pub fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    resolve_population(state);
    take_kept_reading(state);
    let names = histogram_names(state.results_reads());
    let name = selected_name(state.results_reads(), &names);
    let plan = name
        .as_deref()
        .map(|name| hist_plan(state.results_reads(), name));
    let selected: Vec<f64> = plan
        .as_ref()
        .and_then(|plan| plan.rows.as_ref())
        .map(|(key, values)| {
            state
                .results
                .session
                .members
                .rows_on(*key)
                .iter()
                .filter_map(|row| values.get(*row).copied().flatten())
                .collect()
        })
        .unwrap_or_default();
    let significant_digits = significant_digits(state.results_reads());
    let settings = &state.analysis.histogram_state;
    let (normal_fit, normal_quantile) = (settings.normal_fit, settings.normal_quantile);
    let model = match &plan {
        None => Err(HistogramUnavailable::NoSelection {
            has_measurements: !names.is_empty(),
        }),
        Some(plan) => plan
            .histogram
            .as_deref()
            .ok_or(HistogramUnavailable::Invalid)
            .map(|histogram| HistogramPlot {
                histogram,
                display: plan
                    .display
                    .as_ref()
                    .map(Arc::as_ref)
                    .map_err(|reason| *reason),
                moments: plan.moments,
                shape: plan.shape.as_ref(),
                unit: &plan.unit,
                limits: plan.limits,
                source: &plan.display_source,
                selected: &selected,
                significant_digits,
                normal_fit,
                normal_quantile,
            }),
    };
    let plot_view = state
        .results
        .session
        .plot_view(super::ResultViewer::Hist, 0);
    let out = view::show(ui, model, plot_view, &mut state.results.session.cache);
    if let (Some(picked), Some(plan)) = (out.picked, plan.as_ref())
        && let (Some((key, values)), Some(histogram)) = (&plan.rows, plan.histogram.as_deref())
    {
        *state.results.session.members.on(*key) = picked_rows(values, histogram, picked);
    }
    if out.fit_requested {
        state
            .results
            .session
            .reset_plot_view(super::ResultViewer::Hist, 0);
    }
    if out.open_specification {
        super::open_in_verify(
            state,
            crate::state::VerificationPage::Specifications,
            name.clone(),
        );
    }
    if let Some(response) = out.plot {
        super::record_drawn_axes(state.results, super::ResultViewer::Hist, &response);
        if response.view.any() {
            state
                .results
                .session
                .plot_view_mut(super::ResultViewer::Hist, 0)
                .apply(&response.view);
        }
    }
}

/// Render the qualified facts beside the sheet, and keep the range and the
/// limits the reader set there.
pub fn right_panel(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    resolve_population(state);
    take_kept_reading(state);
    let Some(name) = selected_histogram_name(state.results_reads()) else {
        view::right_panel(
            ui,
            Err(HistogramUnavailable::NoSelection {
                has_measurements: false,
            }),
        );
        return;
    };
    let before = state.analysis.histogram_state.reading();
    let current_range = active_histogram(state.results_reads()).map(|histogram| histogram.range());
    if view::range_controls(ui, &mut state.analysis.histogram_state, current_range) {
        state
            .results
            .session
            .reset_plot_view(super::ResultViewer::Hist, 0);
    }
    let plan = hist_plan(state.results_reads(), &name);
    let settings = &state.analysis.histogram_state;
    let model = plan
        .histogram
        .as_deref()
        .ok_or(HistogramUnavailable::Invalid)
        .map(|histogram| HistogramInspector {
            histogram,
            moments: plan.moments,
            shape: plan.shape.as_ref(),
            unit: &plan.unit,
            method: plan.method,
            limits: plan.limits,
            yield_count: plan.yield_count,
            reader_limits: (settings.lower_limit, settings.upper_limit),
            significant_digits: significant_digits(state.results_reads()),
        });
    let response = view::right_panel(ui, model);
    if let Some((lower, upper)) = response.reader_limits {
        let settings = &mut state.analysis.histogram_state;
        settings.lower_limit = lower;
        settings.upper_limit = upper;
    }
    keep_reading(state, &before);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisResultFamilyMetadata, AnalysisType, MonteCarloVariableMetadata,
        SimulationRun,
    };
    use crate::workbench::documents::result_document::test_host::ResultsTestHost;
    use rspice_results::yield_analysis::{
        DistributionStats, MonteCarloSamplingMode, YieldAnalysisProvenance, YieldSpec,
    };

    pub(super) fn result(target: &str, yield_percent: f64) -> YieldResult {
        let total_runs = 100;
        let pass_count = yield_percent.round() as usize;
        let mut samples = vec![1.0; pass_count];
        samples.resize(total_runs, 0.0);
        let moments = moments_from_samples(&samples).expect("finite test samples");
        YieldResult {
            spec: YieldSpec::lower(target, 0.9, "V"),
            total_runs,
            pass_count,
            fail_count: total_runs - pass_count,
            yield_percent,
            stats: DistributionStats {
                count: moments.count,
                mean: moments.mean,
                std_dev: moments.std_dev,
                min: moments.min,
                max: moments.max,
                median: 0.0,
                skewness: None,
                kurtosis: None,
                cp: None,
                cpk: None,
            },
            trail: (0..total_runs).map(|index| index < pass_count).collect(),
            samples,
        }
    }

    pub(super) fn provenance(run: &SimulationRun) -> YieldAnalysisProvenance {
        YieldAnalysisProvenance {
            source_run_id: run.run_id,
            source_dataset_id: run.dataset_id,
            seed: 7,
            runs_requested: 100,
            runs_completed: 100,
            sampling_mode: MonteCarloSamplingMode::PseudoRandom,
        }
    }

    #[test]
    fn selected_measurement_never_uses_first_result_for_another_measurement() {
        let run = SimulationRun::new(1);
        let mut simulation = SimulationState::default();
        simulation.retained.runs.push(run.clone());
        simulation.view.active_run_idx = Some(0);
        simulation.replace_yield_evidence(
            vec![result("gain", 12.0), result("V(out)", 97.0)],
            Some(provenance(&run)),
        );

        let selected = selected_yield_result(&simulation, "V(out)", true)
            .expect("the exact selected measurement is authoritative");
        assert_eq!(selected.spec.target, "V(out)");
        assert_eq!(selected.yield_percent, 97.0);
    }

    #[test]
    fn stale_dataset_yield_evidence_is_rejected() {
        let stale_run = SimulationRun::new(1);
        let active_run = SimulationRun::new(2);
        let mut simulation = SimulationState::default();
        simulation.retained.runs = vec![stale_run.clone(), active_run].into();
        simulation.view.active_run_idx = Some(1);
        simulation
            .replace_yield_evidence(vec![result("V(out)", 23.0)], Some(provenance(&stale_run)));

        assert!(selected_yield_result(&simulation, "V(out)", true).is_none());
    }

    #[test]
    fn absent_or_ambiguous_measurement_evidence_fails_closed() {
        let run = SimulationRun::new(1);
        let mut simulation = SimulationState::default();
        simulation.retained.runs.push(run.clone());
        simulation.view.active_run_idx = Some(0);
        simulation.replace_yield_evidence(vec![result("gain", 90.0)], Some(provenance(&run)));
        assert!(selected_yield_result(&simulation, "V(out)", true).is_none());

        simulation.replace_yield_evidence(
            vec![result("V(out)", 90.0), result("mean(V(out))", 10.0)],
            Some(provenance(&run)),
        );
        assert!(selected_yield_result(&simulation, "V(out)", true).is_none());
    }

    #[test]
    fn mean_wrapper_resolves_only_its_exact_sample_population() {
        let run = SimulationRun::new(1);
        let mut simulation = SimulationState::default();
        simulation.retained.runs.push(run.clone());
        simulation.view.active_run_idx = Some(0);
        simulation
            .replace_yield_evidence(vec![result("mean(V(out))", 88.0)], Some(provenance(&run)));

        assert_eq!(
            selected_yield_result(&simulation, "V(out)", true).map(|result| result.yield_percent),
            Some(88.0)
        );
        assert!(selected_yield_result(&simulation, "v(out)", true).is_none());
    }

    pub(super) fn mc_variable(name: &str) -> MonteCarloVariableMetadata {
        MonteCarloVariableMetadata {
            mean_confidence: None,
            name: name.to_owned(),
            samples: vec![1.0, 2.0, 3.0],
            mean: 2.0,
            std_dev: 1.0,
            min: 1.0,
            max: 3.0,
        }
    }

    #[test]
    fn monte_carlo_method_authority_is_bound_to_the_selected_measurement() {
        let analysis = AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
            AnalysisResultFamilyMetadata::MonteCarlo {
                member_measurements: Vec::new(),
                seed: 0x73a4,
                runs_requested: 4,
                runs_completed: 3,
                failures: 1,
                all_converged: false,
                variables: vec![mc_variable("gain"), mc_variable("offset")],
            },
        );
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));

        let authority = active_monte_carlo_authority(state.results_reads(), "offset")
            .expect("exact selected variable authority");
        assert_eq!(authority.seed, 0x73a4);
        assert_eq!(authority.runs_completed, 3);
        assert_eq!(authority.failures, 1);
        assert_eq!(
            exact_moments(state.results_reads(), "offset").unwrap().mean,
            2.0
        );
        assert!(active_monte_carlo_authority(state.results_reads(), "missing").is_none());
    }

    #[test]
    fn duplicate_monte_carlo_variable_authority_fails_closed() {
        let mut analysis = AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC");
        analysis.family_metadata = Some(AnalysisResultFamilyMetadata::MonteCarlo {
            member_measurements: Vec::new(),
            seed: 9,
            runs_requested: 3,
            runs_completed: 3,
            failures: 0,
            all_converged: true,
            variables: vec![mc_variable("gain"), mc_variable("gain")],
        });
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));

        assert!(active_monte_carlo_authority(state.results_reads(), "gain").is_none());
    }

    #[test]
    fn exact_moments_fail_closed_when_retained_summary_disagrees_with_samples() {
        let mut variable = mc_variable("gain");
        variable.mean = 200.0;
        let analysis = AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
            AnalysisResultFamilyMetadata::MonteCarlo {
                member_measurements: Vec::new(),
                seed: 9,
                runs_requested: 3,
                runs_completed: 3,
                failures: 0,
                all_converged: true,
                variables: vec![variable],
            },
        );
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));

        assert!(exact_moments(state.results_reads(), "gain").is_none());
    }

    #[test]
    fn inconsistent_yield_counts_and_statistics_are_rejected() {
        let valid = result("V(out)", 90.0);
        assert!(yield_result_is_consistent(&valid));

        let mut bad_counts = valid.clone();
        bad_counts.fail_count += 1;
        assert!(!yield_result_is_consistent(&bad_counts));

        let mut bad_stats = valid;
        bad_stats.stats.mean += 0.25;
        assert!(!yield_result_is_consistent(&bad_stats));
    }

    /// The population is walked once per (generation, distribution), and the
    /// answer must move when either does.
    #[test]
    fn the_distribution_statistics_are_resolved_once_per_generation() {
        let analysis = AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
            AnalysisResultFamilyMetadata::MonteCarlo {
                member_measurements: Vec::new(),
                seed: 3,
                runs_requested: 3,
                runs_completed: 3,
                failures: 0,
                all_converged: true,
                variables: vec![mc_variable("gain"), mc_variable("offset")],
            },
        );
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));

        let first = hist_plan(state.results_reads(), "gain");
        assert!(std::sync::Arc::ptr_eq(
            &first,
            &hist_plan(state.results_reads(), "gain")
        ));
        assert_eq!(first.moments.expect("retained moments").mean, 2.0);
        // The memo is the projection it replaced.
        assert_eq!(
            first.moments.expect("retained moments").std_dev,
            exact_moments(state.results_reads(), "gain")
                .expect("retained moments")
                .std_dev
        );

        // Another distribution is another answer, at the same generation.
        let other = hist_plan(state.results_reads(), "offset");
        assert!(!std::sync::Arc::ptr_eq(&first, &other));

        // A new generation of the population is a new answer.
        let AnalysisResultFamilyMetadata::MonteCarlo { variables, .. } =
            state.model.simulation.retained.runs[0].analyses[0]
                .family_metadata
                .as_mut()
                .expect("Monte Carlo metadata")
        else {
            panic!("the fixture retains Monte Carlo metadata");
        };
        variables[0].samples = vec![10.0, 20.0, 30.0];
        variables[0].mean = 20.0;
        variables[0].std_dev = 10.0;
        variables[0].min = 10.0;
        variables[0].max = 30.0;
        state.model.simulation.view.data_version =
            state.model.simulation.view.data_version.wrapping_add(1);

        let after = hist_plan(state.results_reads(), "gain");
        assert_eq!(
            after.moments.expect("retained moments").mean,
            20.0,
            "the sheet reported the previous population's mean"
        );
    }
}
