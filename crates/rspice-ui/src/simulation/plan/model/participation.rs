//! The plan transaction that moves one analysis's run-set participation.
//!
//! It sits beside the other lifecycle edits rather than among them because the
//! value it writes is run-set vocabulary, not plan vocabulary: the plan stores
//! which points an instance visits, but what a point *is* belongs to
//! [`crate::simulation::run_set`], and this file is the only part of the plan
//! model that has to name it.
//!
//! Like every other edit here it goes through `transact`, so a participation
//! change moves the plan revision and leaves a receipt. That is what makes the
//! `FrameSimulationInput` staleness contract hold by construction: an
//! authorized preflight cannot be followed by a narrowing and then a dispatch
//! of the queue that was never checked.

use super::{
    AnalysisInstanceId, AnalysisLifecycleCommand, AnalysisLifecycleReceipt, AnalysisLifecycleState,
    AnalysisPlanError, AnalysisRunAt, SimulationPlan,
};

impl SimulationPlan {
    /// Atomically replace which run-set points one analysis is executed at.
    ///
    /// A selection that names nothing is refused rather than stored: an
    /// instance that runs nowhere is a disabled instance, and the plan already
    /// has a word for that. The refusal leaves the plan byte-for-byte
    /// unchanged, so a rejected edit cannot move the revision and invalidate a
    /// preflight for nothing.
    ///
    /// The keys are *not* checked against the run set here. The plan does not
    /// hold the run set — the two are edited on separate pages against separate
    /// revisions — and a check against a space this type cannot see would be a
    /// guess. The run-set page owns that rule at the moment the space changes,
    /// where the points that are about to disappear can actually be named.
    pub fn set_run_at(
        &mut self,
        id: AnalysisInstanceId,
        run_at: AnalysisRunAt,
    ) -> Result<AnalysisLifecycleReceipt, AnalysisPlanError> {
        let instance = self
            .instance(id)
            .ok_or(AnalysisPlanError::InstanceNotFound(id))?;
        let kind = instance.kind();
        let outcome = if instance.enabled() {
            AnalysisLifecycleState::Draft
        } else {
            AnalysisLifecycleState::Disabled
        };
        run_at
            .validate()
            .map_err(|reason| AnalysisPlanError::RunSetParticipationInvalid { id, kind, reason })?;
        let detail = match &run_at {
            AnalysisRunAt::AllPoints => format!(
                "{} analysis {id} runs at every point of the run set again.",
                kind.label()
            ),
            AnalysisRunAt::NominalPoint => format!(
                "{} analysis {id} runs at the nominal run-set point only.",
                kind.label()
            ),
            AnalysisRunAt::SelectedPoints(keys) => format!(
                "{} analysis {id} runs at {} selected run-set point{}.",
                kind.label(),
                keys.len(),
                if keys.len() == 1 { "" } else { "s" }
            ),
        };
        let ((), receipt) = self.transact(
            AnalysisLifecycleCommand::Edit,
            id,
            None,
            outcome,
            detail,
            move |candidate, revision| {
                let index = candidate.index_of(id)?;
                candidate.ensure_editable(index)?;
                let instance = &mut candidate.instances[index];
                instance.run_at = run_at;
                instance.modified_revision = revision;
                Ok(())
            },
        )?;
        Ok(receipt)
    }
}
