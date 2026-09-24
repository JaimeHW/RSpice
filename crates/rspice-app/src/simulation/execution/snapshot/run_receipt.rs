//! The one builder of a prepared-run receipt, and its two callers.
//!
//! A snapshot exists to be dispatched, and dispatch's first act is to seal a
//! receipt from it. Preflight therefore has to seal the same receipt before it
//! reports a plan runnable — otherwise a plan clears preflight and is refused
//! at Run, which is exactly what happened while only dispatch built one.
//!
//! Both sides project their own task representation into the same facts and
//! call [`seal_prepared_run_receipt`], so what preflight validates is
//! bit-for-bit what dispatch seals rather than a second opinion about it.

use crate::product::{
    AnalysisInstanceId, ContentDigest, DerivedAnalysisIdentity, ObjectRevision, SimulationPlanId,
};
use crate::simulation::multi_run::AnalysisSpec;
use crate::state::{
    AnalysisResultSourceDomain, HierarchyMapRow, PreparedModelSourceIdentity, PreparedRunReceipt,
    PreparedRunTaskReceipt, PreparedSpecification, PreparedSpecificationPolicy,
    SimulationRunIntent,
};

use super::super::canonical::analysis_kind_tag;
use super::{
    AuthorizedRunDispatch, PreparationError, PreparationStage, PreparedRunSnapshot,
    RunSourceReceipt,
};

/// The run-wide facts a prepared-run receipt authenticates.
struct RunReceiptFacts<'a> {
    source_domain: AnalysisResultSourceDomain,
    simulation_plan_id: Option<SimulationPlanId>,
    project_revision: u64,
    snapshot_digest: ContentDigest,
    source_digest: ContentDigest,
    source_receipt: RunSourceReceipt,
    project_model_sources: &'a [PreparedModelSourceIdentity],
    specifications: &'a [PreparedSpecification],
    specification_policy: &'a PreparedSpecificationPolicy,
}

/// The exact facts one task contributes to a prepared-run receipt.
struct ReceiptTaskFacts<'a> {
    instance_id: AnalysisInstanceId,
    /// How the run derived this task's identity, for a task the plan did not
    /// author. Sealed into the receipt so a restored project can re-derive the
    /// identity from the authored instance instead of failing to find it.
    derivation: Option<&'a DerivedAnalysisIdentity>,
    source_revision: ObjectRevision,
    dependencies: &'a [AnalysisInstanceId],
    spec: &'a AnalysisSpec,
    config_digest: ContentDigest,
}

fn seal_prepared_run_receipt<'a>(
    run: RunReceiptFacts<'_>,
    tasks: impl Iterator<Item = ReceiptTaskFacts<'a>>,
    hierarchy_map: Vec<HierarchyMapRow>,
) -> Result<PreparedRunReceipt, PreparationError> {
    let project_revision = ObjectRevision::new(run.project_revision).map_err(|error| {
        PreparationError::new(
            PreparationStage::Authorization,
            format!("Authorized run has invalid project revision: {error}"),
        )
    })?;
    let tasks = tasks
        .map(|task| {
            match task.derivation {
                Some(derivation) => PreparedRunTaskReceipt::new_derived(
                    derivation.clone(),
                    task.source_revision,
                    task.dependencies.to_vec(),
                    analysis_kind_tag(task.spec),
                    task.config_digest,
                ),
                None => PreparedRunTaskReceipt::new(
                    task.instance_id,
                    task.source_revision,
                    task.dependencies.to_vec(),
                    analysis_kind_tag(task.spec),
                    task.config_digest,
                ),
            }
            .map_err(|error| PreparationError::new(PreparationStage::Authorization, error))
        })
        .collect::<Result<Vec<_>, _>>()?;
    PreparedRunReceipt::new_with_project_model_sources_specifications_and_policy(
        run.source_domain,
        run.simulation_plan_id,
        project_revision,
        run.snapshot_digest,
        run.source_digest,
        run.source_receipt.durable(),
        run.project_model_sources.to_vec(),
        run.specifications.to_vec(),
        run.specification_policy.clone(),
        tasks,
    )
    .and_then(|receipt| receipt.with_hierarchy_map(hierarchy_map))
    .map_err(|error| PreparationError::new(PreparationStage::Authorization, error))
}

/// The result domain a run of this intent produces.
///
/// Dispatch and preflight both need it, and it decides which receipt shape is
/// legal, so it is stated once.
pub(in crate::simulation) const fn result_source_domain(
    intent: SimulationRunIntent,
) -> AnalysisResultSourceDomain {
    match intent {
        SimulationRunIntent::SimulateRunSet => AnalysisResultSourceDomain::SimulationPlan,
        SimulationRunIntent::ManualDeck => AnalysisResultSourceDomain::ManualDeck,
    }
}

impl PreparedRunSnapshot {
    /// The exact receipt dispatch will seal for this snapshot.
    ///
    /// Preflight runs this before reporting a plan runnable. Every check the
    /// receipt layer performs — the closed canonical tag protocol, dependency
    /// ordering, model-source and specification uniqueness, the hierarchy map
    /// — therefore fails as a named preflight blocker rather than as a refusal
    /// after the operator has already been told the run was queued.
    pub(in crate::simulation) fn prepared_run_receipt(
        &self,
    ) -> Result<PreparedRunReceipt, PreparationError> {
        let hierarchy_map = self
            .receipt_hierarchy_map()
            .map_err(|error| PreparationError::new(PreparationStage::Authorization, error))?;
        seal_prepared_run_receipt(
            RunReceiptFacts {
                source_domain: result_source_domain(self.intent),
                simulation_plan_id: self.simulation_plan_id,
                project_revision: self.project_revision,
                snapshot_digest: self.digest,
                source_digest: self.source_digest,
                source_receipt: self.receipt,
                project_model_sources: &self.project_model_sources,
                specifications: &self.specifications,
                specification_policy: &self.specification_policy,
            },
            self.tasks.iter().map(|task| ReceiptTaskFacts {
                instance_id: task.instance_id,
                derivation: task.derivation.as_ref(),
                source_revision: task.source_revision,
                dependencies: &task.dependencies,
                spec: &task.task.spec,
                config_digest: task.config_digest,
            }),
            hierarchy_map,
        )
    }
}

impl AuthorizedRunDispatch {
    pub(in crate::simulation) fn prepared_run_receipt(
        &self,
        source_domain: AnalysisResultSourceDomain,
    ) -> Result<PreparedRunReceipt, PreparationError> {
        seal_prepared_run_receipt(
            RunReceiptFacts {
                source_domain,
                simulation_plan_id: self.simulation_plan_id,
                project_revision: self.project_revision,
                snapshot_digest: self.digest,
                source_digest: self.source_digest,
                source_receipt: self.source_receipt,
                project_model_sources: &self.project_model_sources,
                specifications: &self.specifications,
                specification_policy: &self.specification_policy,
            },
            self.tasks.iter().map(|task| ReceiptTaskFacts {
                instance_id: task.instance_id,
                derivation: task.derivation.as_ref(),
                source_revision: task.source_revision,
                dependencies: &task.dependencies,
                spec: &task.task.spec,
                config_digest: task.config_digest,
            }),
            self.hierarchy_map.clone(),
        )
    }
}
