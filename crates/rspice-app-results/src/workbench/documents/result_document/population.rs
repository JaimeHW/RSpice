//! Application source keys, requirement authority and cache lifetime for population viewers.

use super::{AnalysisPresentationKey, ResultsState, SheetContext};
use crate::state::{
    AnalysisResult, PreparedSpecification, ProjectWorkspace, RunHistoryRevision, SimulationState,
    SpecEntry, SpecPointScope,
};
use rspice_results::population::{self as projection, PopulationData, PopulationRequirements};
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

impl PopulationPlan {
    /// The analysis this population was resolved from.
    pub(super) fn analysis(&self) -> AnalysisPresentationKey {
        self.analysis
    }
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
    let analysis = context.simulation.active_analysis()?;
    let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    let source = source_of(context.simulation);
    if let Some(plan) = context.results.plans.population.as_ref()
        && plan.is_current(&source, key, context.workspace)
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

/// The population of the shown analysis for a reader that cannot keep it:
/// the memo when it is current, otherwise one resolved for this call.
pub(super) fn shown(
    simulation: &SimulationState,
    workspace: &ProjectWorkspace,
    results: &ResultsState,
) -> Option<Arc<PopulationPlan>> {
    let run = simulation.active_run()?;
    let analysis = simulation.active_analysis()?;
    let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    let source = source_of(simulation);
    if let Some(plan) = results.plans.population.as_ref()
        && plan.is_current(&source, key, workspace)
    {
        return Some(Arc::clone(plan));
    }
    build(
        analysis,
        key,
        source,
        workspace,
        run.prepared_receipt()
            .map(|receipt| receipt.specifications()),
    )
    .map(Arc::new)
}

fn source_of(simulation: &SimulationState) -> (RunHistoryRevision, u64) {
    (
        simulation.retained.runs.revision(),
        simulation.view.data_version,
    )
}

impl PopulationPlan {
    /// Whether this population is still the shown analysis's under the
    /// workspace's requirements.
    pub(super) fn is_shown(
        &self,
        simulation: &SimulationState,
        workspace: &ProjectWorkspace,
    ) -> bool {
        simulation
            .active_run()
            .zip(simulation.active_analysis())
            .is_some_and(|(run, analysis)| {
                self.is_current(
                    &source_of(simulation),
                    AnalysisPresentationKey::new(run.dataset_id, analysis),
                    workspace,
                )
            })
    }

    /// Whether this population still describes `key` at `source` under the
    /// workspace's requirements.
    fn is_current(
        &self,
        source: &(RunHistoryRevision, u64),
        key: AnalysisPresentationKey,
        workspace: &ProjectWorkspace,
    ) -> bool {
        self.source == *source
            && self.analysis == key
            && self.requirements.as_ref().is_none_or(|requirements| {
                requirements.len() == workspace.content.specs.len()
                    && requirements
                        .iter()
                        .zip(&workspace.content.specs)
                        .all(|(retained, spec)| retained.matches(spec))
            })
    }
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
mod tests;
