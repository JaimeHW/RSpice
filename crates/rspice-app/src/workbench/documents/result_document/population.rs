//! Application source keys, requirement authority and cache lifetime for population viewers.

use super::{AnalysisPresentationKey, SheetContext};
use crate::state::{
    AnalysisResult, PreparedSpecification, ProjectWorkspace, RunHistoryRevision, SpecEntry,
    SpecPointScope,
};
pub(crate) use rspice_results::population::Whiskers;
use rspice_results::population::{self as projection, PopulationData, PopulationRequirements};
pub(super) use rspice_results::population::{
    BoxStatistics, ColumnKind, PopulationColumn, PopulationLimit, TrialStatus, UNPAIRED_REASON,
    box_statistics, cpk, is_a_population, kernel_density, least_squares, mean, pearson,
    silverman_bandwidth, sorted, std_dev, wilson_interval,
};
use std::ops::Deref;
use std::sync::Arc;

/// A population and the exact source/legacy-requirement inputs that keep it current.
#[derive(Debug, Clone)]
pub(super) struct PopulationPlan {
    source: (RunHistoryRevision, u64),
    analysis: AnalysisPresentationKey,
    requirements: Option<Vec<PopulationRequirement>>,
    data: PopulationData,
}

impl Deref for PopulationPlan {
    type Target = PopulationData;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

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
            requirements.len() == context.workspace.content.specs.len()
                && requirements
                    .iter()
                    .zip(&context.workspace.content.specs)
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

fn build(
    analysis: &AnalysisResult,
    key: AnalysisPresentationKey,
    source: (RunHistoryRevision, u64),
    workspace: &ProjectWorkspace,
    prepared: Option<&[PreparedSpecification]>,
) -> Option<PopulationPlan> {
    let requirements = match prepared {
        Some(specs) => PopulationRequirements::Retained(specs),
        None => PopulationRequirements::Legacy(&workspace.content.specs),
    };
    let data = projection::build(analysis, requirements)?;
    Some(PopulationPlan {
        source,
        analysis: key,
        requirements: prepared.is_none().then(|| {
            workspace
                .content
                .specs
                .iter()
                .map(PopulationRequirement::capture)
                .collect()
        }),
        data,
    })
}

#[cfg(test)]
pub(super) fn test_limit(min: Option<f64>, max: Option<f64>) -> PopulationLimit {
    PopulationLimit::from_spec(
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

#[cfg(test)]
mod tests;
