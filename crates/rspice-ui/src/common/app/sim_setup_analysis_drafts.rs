//! Compatibility boundary between the retired singleton draft layout and the
//! stable per-instance simulation-plan domain.
//!
//! Project migration reads legacy fields through this adapter. New execution
//! and presentation code must operate on `AnalysisDraft` values and must not
//! treat these singleton fields as plan identity.

use std::collections::HashSet;

use uuid::Uuid;

use crate::product::{AnalysisInstanceId, ObjectRevision, ProjectId, SimulationPlanId};
use crate::simulation::plan::{
    AnalysisDependency, AnalysisDraft, AnalysisInstance, AnalysisKind, DistoDraft, NoiseDraft,
    SimulationPlan,
};

use super::{NoiseSetup, SimSetupState};

const LEGACY_ANALYSIS_PLAN_NAMESPACE: Uuid =
    Uuid::from_u128(0x4fca_8534_7bd6_52fb_a3f9_5ca4_d8e3_b127);

impl SimSetupState {
    /// Enabled analysis instances in their authoritative presentation and
    /// execution order. An unmigrated legacy payload intentionally yields an
    /// empty iterator so callers fail closed until restoration completes.
    pub(crate) fn enabled_analysis_instances(&self) -> impl Iterator<Item = &AnalysisInstance> {
        self.analysis_plan
            .iter()
            .flat_map(|plan| plan.instances())
            .filter(|instance| instance.enabled())
    }

    /// Number of enabled stable instances, including multiple instances of
    /// the same analysis kind.
    pub(crate) fn enabled_analysis_instance_count(&self) -> usize {
        self.enabled_analysis_instances().count()
    }

    /// Whether at least one enabled instance has the requested canonical
    /// analysis kind.
    pub(crate) fn has_enabled_analysis_kind(&self, kind: AnalysisKind) -> bool {
        self.enabled_analysis_instances()
            .any(|instance| instance.kind() == kind)
    }

    /// Deterministically migrate the schema-3 singleton layout into stable,
    /// project-scoped analysis identities. Replaying the same legacy project
    /// bytes produces the same plan and instance IDs.
    pub(crate) fn migrate_legacy_analysis_plan(
        &mut self,
        project_id: ProjectId,
    ) -> Result<bool, String> {
        if let Some(plan) = &mut self.analysis_plan {
            plan.prepare_after_restore();
            return Ok(false);
        }

        let analysis_count = AnalysisKind::ALL.len();
        if let Some(index) = self
            .enabled
            .iter()
            .chain(self.listed.iter())
            .copied()
            .find(|index| *index >= analysis_count)
        {
            return Err(format!(
                "legacy simulation plan contains unsupported analysis index {index}"
            ));
        }

        let mut seen = HashSet::with_capacity(analysis_count);
        let mut ordered = Vec::with_capacity(analysis_count);
        for index in self.analysis_order.iter().copied() {
            if index >= analysis_count {
                return Err(format!(
                    "legacy simulation plan order contains unsupported analysis index {index}"
                ));
            }
            if !seen.insert(index) {
                return Err(format!(
                    "legacy simulation plan order contains duplicate analysis index {index}"
                ));
            }
            if !self.enabled.contains(&index) {
                return Err(format!(
                    "legacy simulation plan order contains disabled analysis index {index}"
                ));
            }
            ordered.push(index);
        }
        let mut missing_enabled = self
            .enabled
            .iter()
            .copied()
            .filter(|index| !seen.contains(index))
            .collect::<Vec<_>>();
        missing_enabled.sort_unstable();
        for index in missing_enabled {
            seen.insert(index);
            ordered.push(index);
        }
        for index in 0..analysis_count {
            if seen.insert(index) {
                ordered.push(index);
            }
        }

        let project_scope = project_id.to_string();
        let plan_id = SimulationPlanId::from_namespace(
            LEGACY_ANALYSIS_PLAN_NAMESPACE,
            format!("{project_scope}/simulation-plan/schema-3").as_bytes(),
        );
        let revision = ObjectRevision::INITIAL;
        let staged = ordered
            .into_iter()
            .map(|index| {
                let kind = AnalysisKind::from_legacy_index(index)
                    .expect("validated legacy analysis indices map exactly");
                let id = AnalysisInstanceId::from_namespace(
                    LEGACY_ANALYSIS_PLAN_NAMESPACE,
                    format!(
                        "{project_scope}/simulation-plan/schema-3/{}/slot-0",
                        kind.stable_id()
                    )
                    .as_bytes(),
                );
                (
                    id,
                    kind,
                    self.legacy_analysis_draft(kind),
                    self.enabled.contains(&index),
                )
            })
            .collect::<Vec<_>>();

        let mut instances = Vec::with_capacity(staged.len());
        for (position, (id, kind, draft, enabled)) in staged.iter().cloned().enumerate() {
            let dependencies = if enabled {
                kind.prerequisites()
                    .iter()
                    .filter_map(|prerequisite| {
                        staged[..position]
                            .iter()
                            .rev()
                            .find(|(_, candidate_kind, _, candidate_enabled)| {
                                *candidate_enabled && candidate_kind == prerequisite
                            })
                            .map(|(target, _, _, _)| {
                                AnalysisDependency::new(*prerequisite, *target)
                            })
                    })
                    .collect()
            } else {
                Vec::new()
            };
            instances.push(
                AnalysisInstance::supplied(
                    id,
                    kind,
                    draft,
                    enabled,
                    dependencies,
                    revision,
                    revision,
                )
                .map_err(|error| format!("legacy analysis migration failed: {error}"))?,
            );
        }

        let mut plan = SimulationPlan::from_ordered_instances(plan_id, revision, instances)
            .map_err(|error| format!("legacy analysis migration failed: {error}"))?;
        plan.prepare_after_restore();
        self.analysis_plan = Some(plan);
        Ok(true)
    }

    /// Stable plan access used by preflight and presentation after migration.
    pub(crate) fn stable_analysis_plan(&self) -> Result<&SimulationPlan, String> {
        self.analysis_plan.as_ref().ok_or_else(|| {
            "simulation plan has not been migrated to stable analysis-instance identity".to_owned()
        })
    }

    /// Mutable stable plan access used only by its transaction-owning editor.
    pub(crate) fn stable_analysis_plan_mut(&mut self) -> Result<&mut SimulationPlan, String> {
        self.analysis_plan.as_mut().ok_or_else(|| {
            "simulation plan has not been migrated to stable analysis-instance identity".to_owned()
        })
    }

    /// Rebuild the retired singleton projections from the first instance of
    /// each kind. This exists only for controller paths not yet converted to
    /// accept an explicit frozen instance. Multiple same-kind instances remain
    /// authoritative and distinct in `analysis_plan`.
    pub(crate) fn refresh_legacy_analysis_projections(&mut self) {
        let Some(plan) = self.analysis_plan.as_ref() else {
            return;
        };
        let mut seen = HashSet::new();
        let drafts = plan
            .instances()
            .iter()
            .filter(|instance| seen.insert(instance.kind()))
            .map(|instance| (instance.enabled(), instance.draft().clone()))
            .collect::<Vec<_>>();

        self.enabled.clear();
        self.analysis_order.clear();
        self.listed.clear();
        for (enabled, draft) in &drafts {
            let index = draft.legacy_index();
            self.listed.insert(index);
            if *enabled {
                self.enabled.insert(index);
                self.analysis_order.push(index);
            }
        }

        // Apply compatibility-only variants first, then restore AC as the
        // primary legacy AC projection so Noise/DISTO never alias it again.
        for (_, draft) in drafts
            .iter()
            .filter(|(_, draft)| !matches!(draft, AnalysisDraft::Ac(_)))
        {
            self.apply_analysis_draft_projection(draft);
        }
        if let Some((_, ac)) = drafts
            .iter()
            .find(|(_, draft)| matches!(draft, AnalysisDraft::Ac(_)))
        {
            self.apply_analysis_draft_projection(ac);
        }
    }

    /// Validate one raw instance draft with the same parser used by the
    /// existing controller, without mutating the authoritative plan.
    pub(crate) fn analysis_draft_validation_error(&self, draft: &AnalysisDraft) -> Option<String> {
        let mut projection = self.clone();
        projection.apply_analysis_draft_projection(draft);
        if matches!(draft, AnalysisDraft::OperatingPoint(_)) {
            projection.op.temperature = self.reference_pvt.temperature_celsius.to_string();
        }
        projection.validation_error(draft.legacy_index())
    }

    /// Render the existing concise summary from one exact instance draft.
    pub(crate) fn analysis_draft_summary(&self, draft: &AnalysisDraft) -> String {
        let mut projection = self.clone();
        projection.apply_analysis_draft_projection(draft);
        if matches!(draft, AnalysisDraft::OperatingPoint(_)) {
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
        let mut projection = self.clone();
        for dependency in instance.dependencies() {
            let target = frozen
                .instances()
                .iter()
                .find(|candidate| candidate.id() == dependency.target())
                .ok_or_else(|| {
                    format!(
                        "frozen analysis {} references missing prerequisite {}",
                        instance.id(),
                        dependency.target()
                    )
                })?;
            if target.kind() != dependency.prerequisite() {
                return Err(format!(
                    "frozen analysis {} prerequisite {} targets {}, not {}",
                    instance.id(),
                    dependency.prerequisite(),
                    target.kind(),
                    dependency.prerequisite()
                ));
            }
            projection.apply_analysis_draft_projection(target.draft());
        }
        projection.apply_analysis_draft_projection(instance.draft());
        if instance.kind() == AnalysisKind::OperatingPoint {
            projection.op.temperature = self.reference_pvt.temperature_celsius.to_string();
        }
        projection.enabled.clear();
        projection.enabled.insert(instance.kind().legacy_index());
        projection.analysis_order = vec![instance.kind().legacy_index()];
        Ok(projection)
    }

    /// Capture one exact legacy draft without parsing or normalizing user text.
    pub(crate) fn legacy_analysis_draft(&self, kind: AnalysisKind) -> AnalysisDraft {
        match kind {
            AnalysisKind::OperatingPoint => AnalysisDraft::OperatingPoint(self.op.clone()),
            AnalysisKind::Transient => AnalysisDraft::Transient(self.tran.clone()),
            AnalysisKind::Ac => AnalysisDraft::Ac(self.ac.clone()),
            AnalysisKind::DcSweep => AnalysisDraft::DcSweep(self.dc.clone()),
            AnalysisKind::Noise => AnalysisDraft::Noise(NoiseDraft {
                output: self.noise.output.clone(),
                reference: self.noise.reference.clone(),
                input: self.noise.input.clone(),
                fstart: self.noise.fstart.clone(),
                fstop: self.noise.fstop.clone(),
                points: self.ac.points.clone(),
                sweep: self.ac.sweep,
            }),
            AnalysisKind::PoleZero => AnalysisDraft::PoleZero(self.pz.clone()),
            AnalysisKind::Sensitivity => AnalysisDraft::Sensitivity(self.sens.clone()),
            AnalysisKind::MonteCarlo => AnalysisDraft::MonteCarlo(self.mc.clone()),
            AnalysisKind::Pss => AnalysisDraft::Pss(self.pss.clone()),
            AnalysisKind::Stb => AnalysisDraft::Stb(self.stb.clone()),
            AnalysisKind::Temperature => AnalysisDraft::Temperature(self.temp.clone()),
            AnalysisKind::HarmonicBalance => AnalysisDraft::HarmonicBalance(self.hb.clone()),
            AnalysisKind::SParameter => AnalysisDraft::SParameter(self.sp.clone()),
            AnalysisKind::Pac => AnalysisDraft::Pac(self.pac.clone()),
            AnalysisKind::Pnoise => AnalysisDraft::Pnoise(self.pnoise.clone()),
            AnalysisKind::Pxf => AnalysisDraft::Pxf(self.pxf.clone()),
            AnalysisKind::Pstb => AnalysisDraft::Pstb(self.pstb.clone()),
            AnalysisKind::TransferFunction => AnalysisDraft::TransferFunction(self.xf.clone()),
            AnalysisKind::Corner => AnalysisDraft::Corner(self.corner.clone()),
            AnalysisKind::Envelope => AnalysisDraft::Envelope(self.envelope.clone()),
            AnalysisKind::Fourier => AnalysisDraft::Fourier(self.fourier.clone()),
            AnalysisKind::Reliability => AnalysisDraft::Reliability(self.reliability.clone()),
            AnalysisKind::Optimization => AnalysisDraft::Optimization(self.optimization.clone()),
            AnalysisKind::Soa => AnalysisDraft::Soa(self.soa.clone()),
            AnalysisKind::Disto => AnalysisDraft::Disto(DistoDraft {
                sweep: self.ac.clone(),
                f2_over_f1: self.disto_f2_over_f1.clone(),
            }),
        }
    }

    /// Materialize one instance draft into the legacy controller/form view.
    ///
    /// Noise and DISTO deliberately copy their owned sweep into the legacy AC
    /// slots because the retired controller API reads those slots. Callers
    /// must use a short-lived projection, never the authoritative plan state.
    pub(crate) fn apply_analysis_draft_projection(&mut self, draft: &AnalysisDraft) {
        match draft {
            AnalysisDraft::OperatingPoint(value) => self.op = value.clone(),
            AnalysisDraft::Transient(value) => self.tran = value.clone(),
            AnalysisDraft::Ac(value) => self.ac = value.clone(),
            AnalysisDraft::DcSweep(value) => self.dc = value.clone(),
            AnalysisDraft::Noise(value) => {
                self.noise = NoiseSetup {
                    output: value.output.clone(),
                    reference: value.reference.clone(),
                    input: value.input.clone(),
                    fstart: value.fstart.clone(),
                    fstop: value.fstop.clone(),
                };
                self.ac.points.clone_from(&value.points);
                self.ac.sweep = value.sweep;
            }
            AnalysisDraft::PoleZero(value) => self.pz = value.clone(),
            AnalysisDraft::Sensitivity(value) => self.sens = value.clone(),
            AnalysisDraft::MonteCarlo(value) => self.mc = value.clone(),
            AnalysisDraft::Pss(value) => self.pss = value.clone(),
            AnalysisDraft::Stb(value) => self.stb = value.clone(),
            AnalysisDraft::Temperature(value) => self.temp = value.clone(),
            AnalysisDraft::HarmonicBalance(value) => self.hb = value.clone(),
            AnalysisDraft::SParameter(value) => self.sp = value.clone(),
            AnalysisDraft::Pac(value) => self.pac = value.clone(),
            AnalysisDraft::Pnoise(value) => self.pnoise = value.clone(),
            AnalysisDraft::Pxf(value) => self.pxf = value.clone(),
            AnalysisDraft::Pstb(value) => self.pstb = value.clone(),
            AnalysisDraft::TransferFunction(value) => self.xf = value.clone(),
            AnalysisDraft::Corner(value) => self.corner = value.clone(),
            AnalysisDraft::Envelope(value) => self.envelope = value.clone(),
            AnalysisDraft::Fourier(value) => self.fourier = value.clone(),
            AnalysisDraft::Reliability(value) => self.reliability = value.clone(),
            AnalysisDraft::Optimization(value) => self.optimization = value.clone(),
            AnalysisDraft::Soa(value) => self.soa = value.clone(),
            AnalysisDraft::Disto(value) => {
                self.ac = value.sweep.clone();
                self.disto_f2_over_f1.clone_from(&value.f2_over_f1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn legacy_setup() -> SimSetupState {
        let mut setup = SimSetupState::new();
        setup.analysis_plan = None;
        setup.enabled.extend([0, 2]);
        setup.analysis_order = vec![0, 1, 2];
        setup.ac.points = "53".to_owned();
        setup.disto_f2_over_f1 = "1.07".to_owned();
        setup
    }

    #[test]
    fn compatibility_projection_preserves_noise_and_disto_owned_sweeps() {
        let mut setup = SimSetupState::new();
        let noise = AnalysisDraft::Noise(NoiseDraft {
            points: "77".to_owned(),
            sweep: 2,
            ..NoiseDraft::default()
        });
        setup.apply_analysis_draft_projection(&noise);
        assert_eq!(setup.ac.points, "77");
        assert_eq!(setup.ac.sweep, 2);
        let captured = setup.legacy_analysis_draft(AnalysisKind::Noise);
        let AnalysisDraft::Noise(captured) = captured else {
            panic!("noise projection must retain its kind");
        };
        assert_eq!(captured.points, "77");

        let mut disto = DistoDraft::default();
        disto.sweep.points = "31".to_owned();
        disto.f2_over_f1 = "1.1".to_owned();
        setup.apply_analysis_draft_projection(&AnalysisDraft::Disto(disto));
        let captured = setup.legacy_analysis_draft(AnalysisKind::Disto);
        let AnalysisDraft::Disto(captured) = captured else {
            panic!("DISTO projection must retain its kind");
        };
        assert_eq!(captured.sweep.points, "31");
        assert_eq!(captured.f2_over_f1, "1.1");
    }

    #[test]
    fn legacy_migration_is_project_scoped_reproducible_and_lossless() {
        let namespace = Uuid::from_u128(0xcb81_371c_b443_519d_88ec_0b38_a288_371f);
        let project = ProjectId::from_namespace(namespace, b"legacy-project");
        let mut first = legacy_setup();
        let mut replay = legacy_setup();

        assert!(
            first
                .migrate_legacy_analysis_plan(project)
                .expect("first migration succeeds")
        );
        assert!(
            replay
                .migrate_legacy_analysis_plan(project)
                .expect("replayed migration succeeds")
        );
        let first_json = serde_json::to_string(first.stable_analysis_plan().unwrap()).unwrap();
        let replay_json = serde_json::to_string(replay.stable_analysis_plan().unwrap()).unwrap();
        assert_eq!(first_json, replay_json);

        let plan = first.stable_analysis_plan().unwrap();
        assert_eq!(plan.instances().len(), AnalysisKind::ALL.len());
        assert_eq!(plan.instances()[0].kind(), AnalysisKind::OperatingPoint);
        assert_eq!(plan.instances()[1].kind(), AnalysisKind::Transient);
        assert_eq!(plan.instances()[2].kind(), AnalysisKind::Ac);
        let ac = &plan.instances()[2];
        assert_eq!(ac.dependencies().len(), 1);
        assert_eq!(ac.dependencies()[0].target(), plan.instances()[0].id());
        let AnalysisDraft::Ac(ac_draft) = ac.draft() else {
            panic!("migrated AC retains its exact draft");
        };
        assert_eq!(ac_draft.points, "53");
        let noise = plan
            .instances()
            .iter()
            .find(|instance| instance.kind() == AnalysisKind::Noise)
            .unwrap();
        let AnalysisDraft::Noise(noise) = noise.draft() else {
            panic!("migrated Noise owns its sweep");
        };
        assert_eq!(noise.points, "53");
        let disto = plan
            .instances()
            .iter()
            .find(|instance| instance.kind() == AnalysisKind::Disto)
            .unwrap();
        let AnalysisDraft::Disto(disto) = disto.draft() else {
            panic!("migrated DISTO owns its sweep");
        };
        assert_eq!(disto.sweep.points, "53");
        assert_eq!(disto.f2_over_f1, "1.07");

        let other_project = ProjectId::from_namespace(namespace, b"other-project");
        let mut other = legacy_setup();
        other
            .migrate_legacy_analysis_plan(other_project)
            .expect("other project migrates");
        assert_ne!(
            plan.id(),
            other.stable_analysis_plan().expect("other plan").id()
        );
        assert_ne!(
            plan.instances()[0].id(),
            other.stable_analysis_plan().unwrap().instances()[0].id()
        );
    }

    #[test]
    fn malformed_legacy_order_fails_without_creating_a_plan() {
        let namespace = Uuid::from_u128(0xcb81_371c_b443_519d_88ec_0b38_a288_371f);
        let project = ProjectId::from_namespace(namespace, b"malformed-project");
        let mut setup = legacy_setup();
        setup.analysis_order = vec![1, 1, 0, 2];

        let error = setup
            .migrate_legacy_analysis_plan(project)
            .expect_err("duplicate legacy order must fail");
        assert!(error.contains("duplicate analysis index 1"));
        assert!(setup.analysis_plan.is_none());
    }
}
