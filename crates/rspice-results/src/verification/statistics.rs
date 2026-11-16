//! Monte Carlo trials as a sheet states them: the yield the acceptance gate
//! counts, the distribution behind it, and the trials that failed.

use super::points::RunPoints;
use super::sheet::Row;
use crate::family_measurements::FamilyMemberId;
use crate::population::wilson_interval;
use crate::specification_verdict::{Candidate, candidate_is_passing, trial_count};
use crate::yield_analysis::{
    DistributionStats, SpecLimitType, YieldSpec, calculate_distribution_stats,
};

/// One Monte Carlo trial, as one specification judged it.
#[derive(Debug, Clone, PartialEq)]
pub struct Trial {
    /// The position in the run of the analysis that ran it.
    pub analysis: usize,
    /// The point the trial ran at.
    pub point: Option<usize>,
    /// The trial, with the seed that reproduces it; `None` for a value the
    /// analysis measured as a whole.
    pub member: Option<FamilyMemberId>,
    /// Its finite value in the specification's unit; `None` when it gave none.
    pub value: Option<f64>,
    pub passing: bool,
}

impl Trial {
    /// The trial's number, as the analysis requested it.
    #[must_use]
    pub fn index(&self) -> Option<usize> {
        self.member.as_ref().map(FamilyMemberId::index)
    }

    /// The seed that reproduces the trial.
    #[must_use]
    pub fn seed(&self) -> Option<u64> {
        match self.member.as_ref()? {
            FamilyMemberId::MonteCarloTrial { seed, .. }
            | FamilyMemberId::MonteCarloSequenceTrial { seed, .. } => Some(*seed),
            FamilyMemberId::SweepPoint { .. } | FamilyMemberId::Corner { .. } => None,
        }
    }
}

/// How many trials held, out of how many.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct YieldCount {
    pub passing: usize,
    pub total: usize,
    /// The Wilson 95 % interval on the yield, in percent.
    pub interval: Option<(f64, f64)>,
}

impl YieldCount {
    fn new(passing: usize, total: usize) -> Self {
        Self {
            passing,
            total,
            interval: wilson_interval(passing, total),
        }
    }

    /// The yield in percent.
    #[must_use]
    pub fn percent(&self) -> f64 {
        self.passing as f64 / self.total as f64 * 100.0
    }
}

/// The trials at one point, when trials ran at more than one.
#[derive(Debug, Clone)]
pub struct PointStatistics {
    pub point: Option<usize>,
    pub trials: usize,
    pub yield_count: Option<YieldCount>,
    pub distribution: DistributionStats,
}

/// Every Monte Carlo trial that answered one specification.
#[derive(Debug, Clone)]
pub struct Statistics {
    /// In retained order, the order the yield gate counts them in.
    pub trials: Vec<Trial>,
    /// `None` for a specification with no limit, or a run that does not
    /// answer this plan's limits.
    pub yield_count: Option<YieldCount>,
    /// Mean, sigma, extremes and capability of the trials' finite values.
    pub distribution: DistributionStats,
    /// Statistics at each point, in point order, when trials ran at more
    /// than one; empty otherwise.
    pub per_point: Vec<PointStatistics>,
}

impl Statistics {
    /// The trials that did not hold, with their numbers, seeds and values.
    pub fn failing_trials(&self) -> impl Iterator<Item = &Trial> {
        self.trials.iter().filter(|trial| !trial.passing)
    }

    /// The trials' finite values, for a histogram.
    #[must_use]
    pub fn samples(&self) -> Vec<f64> {
        self.trials.iter().filter_map(|trial| trial.value).collect()
    }
}

/// Trials that passed every specification with a yield.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointYield {
    /// Over the trials every such specification was judged on.
    Counted {
        passing: usize,
        total: usize,
        specifications: usize,
    },
    /// The specifications were judged over different trials, so no trial
    /// can be said to pass them all.
    DifferentTrials,
}

/// The statistics of one row's Monte Carlo candidates, when it has any.
///
/// `limits` are the bounds the yield and capability are stated against;
/// `None` states neither.
pub(super) fn row_statistics(
    measurement: &str,
    limits: Option<(Option<f64>, Option<f64>)>,
    candidates: &[Candidate],
    points: &RunPoints,
) -> Option<Statistics> {
    let trials = candidates
        .iter()
        .filter(|candidate| candidate.is_monte_carlo)
        .map(|candidate| Trial {
            analysis: candidate.analysis,
            point: points.of_candidate(candidate.analysis, candidate.member.as_ref()),
            member: candidate.member.clone(),
            value: candidate.value,
            passing: candidate_is_passing(candidate),
        })
        .collect::<Vec<_>>();
    if trials.is_empty() {
        return None;
    }
    let spec = yield_spec(measurement, limits);
    let count = trial_count(candidates);
    let mut at_points: Vec<Option<usize>> = Vec::new();
    for trial in &trials {
        if !at_points.contains(&trial.point) {
            at_points.push(trial.point);
        }
    }
    at_points.sort();
    let per_point = if at_points.len() > 1 {
        at_points
            .into_iter()
            .map(|point| {
                let here = trials
                    .iter()
                    .filter(|trial| trial.point == point)
                    .collect::<Vec<_>>();
                let passing = here.iter().filter(|trial| trial.passing).count();
                PointStatistics {
                    point,
                    trials: here.len(),
                    yield_count: limits.map(|_| YieldCount::new(passing, here.len())),
                    distribution: distribution(here.iter().copied(), &spec),
                }
            })
            .collect()
    } else {
        Vec::new()
    };
    Some(Statistics {
        yield_count: limits.map(|_| YieldCount::new(count.passing, count.total)),
        distribution: distribution(trials.iter(), &spec),
        trials,
        per_point,
    })
}

/// The limit the capability is computed against, as the yield judge states
/// it; with no limits, a specification that has no capability.
fn yield_spec(measurement: &str, limits: Option<(Option<f64>, Option<f64>)>) -> YieldSpec {
    let (min, max) = limits.unwrap_or((None, None));
    YieldSpec {
        target: measurement.to_owned(),
        limit_type: match (min, max) {
            (Some(_), None) => SpecLimitType::Lower,
            (None, Some(_)) => SpecLimitType::Upper,
            (Some(_), Some(_)) | (None, None) => SpecLimitType::Range,
        },
        min,
        max,
        target_val: None,
        unit: String::new(),
        weight: 1.0,
    }
}

fn distribution<'a>(
    trials: impl Iterator<Item = &'a Trial>,
    spec: &YieldSpec,
) -> DistributionStats {
    let samples = trials.filter_map(|trial| trial.value).collect::<Vec<_>>();
    calculate_distribution_stats(&samples, spec)
}

/// Trials that passed every row with a yield, counted only when every such
/// row was judged over the same trials.
pub(super) fn joint_yield(rows: &[Row]) -> Option<JointYield> {
    let judged = rows
        .iter()
        .filter_map(|row| {
            let statistics = row.statistics.as_ref()?;
            statistics.yield_count.map(|_| &statistics.trials)
        })
        .collect::<Vec<_>>();
    let first = judged.first()?;
    let same_trials = |trials: &&Vec<Trial>| {
        trials.len() == first.len()
            && trials
                .iter()
                .zip(first.iter())
                .all(|(left, right)| left.analysis == right.analysis && left.member == right.member)
    };
    if !judged.iter().all(same_trials) {
        return Some(JointYield::DifferentTrials);
    }
    let passing = (0..first.len())
        .filter(|index| judged.iter().all(|trials| trials[*index].passing))
        .count();
    Some(JointYield::Counted {
        passing,
        total: first.len(),
        specifications: judged.len(),
    })
}
