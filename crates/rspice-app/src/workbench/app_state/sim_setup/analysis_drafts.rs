//! Compatibility boundary between the retired singleton draft layout and the
//! stable per-instance simulation-plan domain.
//!
//! Project migration reads legacy fields through this adapter. New execution
//! and presentation code must operate on `AnalysisDraft` values and must not
//! treat these singleton fields as plan identity.

use std::collections::HashSet;

use crate::product::AnalysisInstanceId;
use rspice_simulation_contract::analysis_draft::AnalysisDraft;
use rspice_simulation_contract::analysis_kind::AnalysisKind;

use crate::workbench::app_state::SimSetupState;

impl SimSetupState {
    /// Validate one raw instance draft with the same parser used by the
    /// existing controller, without mutating the authoritative plan.
    pub(crate) fn analysis_draft_validation_error(&self, draft: &AnalysisDraft) -> Option<String> {
        if let Some(error) = draft.manifest_configuration_error() {
            return Some(error);
        }
        if let Some(blocker) = draft.kind().execution_blocker() {
            return Some(format!("Execution unavailable: {blocker}"));
        }
        let mut projection = self.clone();
        projection.apply_analysis_draft_projection(draft);
        if matches!(draft, AnalysisDraft::OperatingPoint(state) if matches!(state.temperature_mode_idx, 0 | 3))
        {
            projection.op.temperature = self.reference_pvt.temperature_celsius.to_string();
        }
        projection.validation_error(draft.legacy_index())
    }

    /// Render the existing concise summary from one exact instance draft.
    pub(crate) fn analysis_draft_summary(&self, draft: &AnalysisDraft) -> String {
        if let Some(summary) = draft.manifest_summary() {
            return summary;
        }
        let mut projection = self.clone();
        projection.apply_analysis_draft_projection(draft);
        if matches!(draft, AnalysisDraft::OperatingPoint(state) if matches!(state.temperature_mode_idx, 0 | 3))
        {
            projection.op.temperature = self.reference_pvt.temperature_celsius.to_string();
        }
        projection.summary(draft.legacy_index())
    }

    /// Build a short-lived legacy controller view from an immutable frozen
    /// instance and its exact bound prerequisites.
    pub(crate) fn frozen_instance_projection(
        &self,
        frozen: &crate::simulation::plan::FrozenSimulationPlan,
        instance: &crate::simulation::plan::FrozenAnalysisInstance,
    ) -> Result<Self, String> {
        fn apply_dependency_closure(
            projection: &mut SimSetupState,
            frozen: &crate::simulation::plan::FrozenSimulationPlan,
            consumer: &crate::simulation::plan::FrozenAnalysisInstance,
            expected_kind: AnalysisKind,
            target_id: AnalysisInstanceId,
            applied: &mut HashSet<AnalysisInstanceId>,
            visiting: &mut Vec<AnalysisInstanceId>,
        ) -> Result<(), String> {
            let target = frozen
                .instances()
                .iter()
                .find(|candidate| candidate.id() == target_id)
                .ok_or_else(|| {
                    format!(
                        "frozen analysis {} references missing prerequisite {}",
                        consumer.id(),
                        target_id
                    )
                })?;
            if target.kind() != expected_kind {
                return Err(format!(
                    "frozen analysis {} prerequisite {} targets {}, not {}",
                    consumer.id(),
                    expected_kind,
                    target.kind(),
                    expected_kind
                ));
            }
            if target.order() >= consumer.order() {
                return Err(format!(
                    "frozen analysis {} prerequisite {} does not appear earlier",
                    consumer.id(),
                    target.id()
                ));
            }
            if applied.contains(&target.id()) {
                return Ok(());
            }
            if visiting.contains(&target.id()) {
                return Err(format!(
                    "frozen analysis {} has a cyclic prerequisite closure through {}",
                    consumer.id(),
                    target.id()
                ));
            }
            visiting.push(target.id());
            for dependency in target.dependencies() {
                apply_dependency_closure(
                    projection,
                    frozen,
                    target,
                    dependency.prerequisite(),
                    dependency.target(),
                    applied,
                    visiting,
                )?;
            }
            visiting.pop();
            projection.apply_analysis_draft_projection(target.draft());
            applied.insert(target.id());
            Ok(())
        }

        let base = instance.draft().pvt_base_analysis().map(|id| {
            let base = frozen.instances().iter().find(|candidate| candidate.id() == id)
                .ok_or_else(|| format!("Study base analysis {id} is missing or disabled"))?;
            if !base.kind().supports_pvt_base() {
                return Err(format!("{} cannot be used as a Temperature/Corner base; select operating point, transient, AC or DC sweep", base.display_name()));
            }
            Ok(base)
        }).transpose()?;
        let mut projection = match base {
            Some(base) => self.frozen_instance_projection(frozen, base)?,
            None => self.clone(),
        };
        let mut applied = HashSet::new();
        let mut visiting = vec![instance.id()];
        for dependency in instance.dependencies() {
            apply_dependency_closure(
                &mut projection,
                frozen,
                instance,
                dependency.prerequisite(),
                dependency.target(),
                &mut applied,
                &mut visiting,
            )?;
        }
        projection.apply_analysis_draft_projection(instance.draft());
        if let Some(base) = base {
            // The legacy builders still read a type index. Derive it from the
            // bound card; the hidden legacy choice cannot change the request.
            match instance.draft() {
                AnalysisDraft::Temperature(_) => {
                    projection.temp.base_idx = match base.kind() {
                        AnalysisKind::OperatingPoint => 0,
                        AnalysisKind::Transient => 1,
                        AnalysisKind::Ac => 2,
                        _ => 3,
                    }
                }
                AnalysisDraft::Corner(_) => {
                    projection.corner.base_analysis_idx = match base.kind() {
                        AnalysisKind::Transient => 0,
                        AnalysisKind::Ac => 1,
                        AnalysisKind::DcSweep => 2,
                        _ => 3,
                    }
                }
                _ => unreachable!(),
            }
        }
        if matches!(instance.draft(), AnalysisDraft::OperatingPoint(state) if matches!(state.temperature_mode_idx, 0 | 3))
        {
            projection.op.temperature = self.reference_pvt.temperature_celsius.to_string();
        }
        projection.enabled.clear();
        projection.enabled.insert(instance.kind().legacy_index());
        projection.analysis_order = vec![instance.kind().legacy_index()];
        Ok(projection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_projection_applies_the_exact_transitive_prerequisite_closure() {
        let mut setup = SimSetupState::new();
        let plan = setup.analysis_plan.as_mut().expect("stable plan exists");
        let (op, _) = plan
            .insert_at(AnalysisKind::OperatingPoint, 0)
            .expect("OP inserts");
        plan.edit(op, |draft| {
            let AnalysisDraft::OperatingPoint(draft) = draft else {
                panic!("expected OP draft");
            };
            draft.temperature = "88".to_owned();
        })
        .expect("OP edits");
        let (pss, _) = plan.insert(AnalysisKind::Pss).expect("PSS inserts");
        plan.edit(pss, |draft| {
            let AnalysisDraft::Pss(draft) = draft else {
                panic!("expected PSS draft");
            };
            draft.fund_freq = "7Meg".to_owned();
            // A driven solve needs a tone, and only the design can name one.
            draft.tone_sources = "VSRC".to_owned();
        })
        .expect("PSS edits");
        plan.bind_dependency(pss, AnalysisKind::OperatingPoint, op)
            .expect("PSS binds OP");
        let (pac, _) = plan.insert(AnalysisKind::Pac).expect("PAC inserts");
        plan.bind_dependency(pac, AnalysisKind::Pss, pss)
            .expect("PAC binds PSS");
        let frozen = plan.freeze().expect("plan freezes");
        let frozen_pac = frozen
            .instances()
            .iter()
            .find(|instance| instance.id() == pac)
            .expect("PAC freezes");

        let projection = setup
            .frozen_instance_projection(&frozen, frozen_pac)
            .expect("transitive closure projects");

        assert_eq!(projection.op.temperature, "88");
        assert_eq!(projection.pss.fund_freq, "7Meg");
        assert_eq!(
            projection.analysis_order,
            vec![AnalysisKind::Pac.legacy_index()]
        );
    }
}
