//! Exact migration and singleton projections for portable simulation setup.

use super::SimulationSetup;
use crate::analysis_draft::AnalysisDraft;
use crate::analysis_kind::AnalysisKind;
use crate::drafts::{DistoDraft, NoiseDraft};
use crate::legacy_plan_migration::NoiseSetup;
use crate::plan_model::{AnalysisInstance, SimulationPlan};
use rspice_app_types::product::ProjectId;
use std::collections::HashSet;

impl SimulationSetup {
    /// Enabled analysis instances in their authoritative presentation and
    /// execution order. An unmigrated legacy payload intentionally yields an
    /// empty iterator so callers fail closed until restoration completes.
    pub fn enabled_analysis_instances(&self) -> impl Iterator<Item = &AnalysisInstance> {
        self.analysis_plan
            .iter()
            .flat_map(|plan| plan.instances())
            .filter(|instance| instance.enabled())
    }

    /// Number of enabled stable instances, including multiple instances of
    /// the same analysis kind.
    pub fn enabled_analysis_instance_count(&self) -> usize {
        self.enabled_analysis_instances().count()
    }

    /// Whether at least one enabled instance has the requested canonical
    /// analysis kind.
    pub fn has_enabled_analysis_kind(&self, kind: AnalysisKind) -> bool {
        self.enabled_analysis_instances()
            .any(|instance| instance.kind() == kind)
    }

    /// Fold every pre-unification Corner run space into the plan-global one.
    ///
    /// Both shapes a project can carry one in are handled: the schema-3
    /// singleton corner state, and the Corner drafts of a stable analysis plan.
    /// The singleton goes first, because a schema-3 project has no plan yet and
    /// its corner state is the only declaration it has.
    ///
    /// Where the plan-global set is empty the legacy space becomes it, so a
    /// project keeps running the space it was saved with. Where both declare a
    /// space and they differ, the plan-global one wins — it is the one the Run
    /// Set page edits and every other analysis already crosses — and the
    /// declaration that lost is written to [`SimulationSetup::legacy_run_set_notes`]
    /// rather than vanishing.
    fn adopt_legacy_corner_run_sets(&mut self) {
        let mut notes = Vec::new();

        if let Some(note) = self
            .corner
            .adopt_legacy_run_set(&mut self.document.run_set)
            .dropped_declaration_note()
        {
            notes.push(note);
        }
        if let Some(plan) = &mut self.document.analysis_plan {
            for (shown_as, migration) in
                plan.adopt_legacy_corner_run_sets(&mut self.document.run_set)
            {
                if let Some(note) = migration.dropped_declaration_note() {
                    notes.push(format!("{shown_as}: {note}"));
                }
            }
        }

        self.legacy_run_set_notes = notes;
    }

    /// Deterministically migrate the schema-3 singleton layout into stable,
    /// project-scoped analysis identities. Replaying the same legacy project
    /// bytes produces the same plan and instance IDs.
    pub fn migrate_legacy_analysis_plan(&mut self, project_id: ProjectId) -> Result<bool, String> {
        // A project saved before the run space had one owner carries a second
        // declaration inside its Corner drafts. Fold it in here, on the one
        // path every load takes, so the space a plan runs is settled before
        // anything validates, edits or executes against it.
        self.adopt_legacy_corner_run_sets();
        if let Some(plan) = &mut self.analysis_plan {
            plan.prepare_after_restore();
            return Ok(false);
        }

        let plan = crate::legacy_plan_migration::migrate_schema3_analysis_plan(
            project_id,
            &self.enabled,
            &self.listed,
            &self.analysis_order,
            |kind| self.legacy_analysis_draft(kind),
        )?;
        self.analysis_plan = Some(plan);
        Ok(true)
    }

    /// Stable plan access used by preflight and presentation after migration.
    pub fn stable_analysis_plan(&self) -> Result<&SimulationPlan, String> {
        self.analysis_plan.as_ref().ok_or_else(|| {
            "simulation plan has not been migrated to stable analysis-instance identity".to_owned()
        })
    }

    /// Mutable stable plan access used only by its transaction-owning editor.
    pub fn stable_analysis_plan_mut(&mut self) -> Result<&mut SimulationPlan, String> {
        self.analysis_plan.as_mut().ok_or_else(|| {
            "simulation plan has not been migrated to stable analysis-instance identity".to_owned()
        })
    }

    /// Rebuild the retired singleton projections from the first instance of
    /// each kind. This exists only for controller paths not yet converted to
    /// accept an explicit frozen instance. Multiple same-kind instances remain
    /// authoritative and distinct in `analysis_plan`.
    pub fn refresh_legacy_analysis_projections(&mut self) {
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

    /// Capture one exact legacy draft without parsing or normalizing user text.
    pub fn legacy_analysis_draft(&self, kind: AnalysisKind) -> AnalysisDraft {
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
                sweep: crate::config::NoiseSweepType::from_selection_index(self.ac.sweep),
                ..NoiseDraft::default()
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
            AnalysisKind::Envelope => AnalysisDraft::Envelope(Box::new(self.envelope.clone())),
            AnalysisKind::Fourier => AnalysisDraft::Fourier(self.fourier.clone()),
            AnalysisKind::Optimization => AnalysisDraft::Optimization(self.optimization.clone()),
            AnalysisKind::Soa => AnalysisDraft::Soa(self.soa.clone()),
            AnalysisKind::Disto => AnalysisDraft::Disto(DistoDraft {
                sweep: self.ac.clone(),
                f2_over_f1: self.disto_f2_over_f1.clone(),
            }),
            AnalysisKind::Qpss
            | AnalysisKind::Hbsp
            | AnalysisKind::Hbnoise
            | AnalysisKind::Psp
            | AnalysisKind::Qpac
            | AnalysisKind::Qpnoise
            | AnalysisKind::Qpxf
            | AnalysisKind::TransientNoise
            | AnalysisKind::DcMismatch
            | AnalysisKind::AcData
            | AnalysisKind::Fft => AnalysisDraft::for_kind(kind),
        }
    }

    /// Materialize one instance draft into the legacy controller/form view.
    ///
    /// Noise and DISTO deliberately copy their owned sweep into the legacy AC
    /// slots because the retired controller API reads those slots. Callers
    /// must use a short-lived projection, never the authoritative plan state.
    pub fn apply_analysis_draft_projection(&mut self, draft: &AnalysisDraft) {
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
                self.ac.sweep = value.sweep.legacy_index().unwrap_or(usize::MAX);
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
            AnalysisDraft::Envelope(value) => self.envelope = value.as_ref().clone(),
            AnalysisDraft::Fourier(value) => self.fourier = value.clone(),
            AnalysisDraft::Optimization(value) => self.optimization = value.clone(),
            AnalysisDraft::Soa(value) => self.soa = value.clone(),
            AnalysisDraft::Disto(value) => {
                self.ac = value.sweep.clone();
                self.disto_f2_over_f1.clone_from(&value.f2_over_f1);
            }
            AnalysisDraft::Qpss(_)
            | AnalysisDraft::Hbsp(_)
            | AnalysisDraft::Hbnoise(_)
            | AnalysisDraft::Psp(_)
            | AnalysisDraft::Qpac(_)
            | AnalysisDraft::Qpnoise(_)
            | AnalysisDraft::Qpxf(_)
            | AnalysisDraft::TransientNoise(_)
            | AnalysisDraft::DcMismatch(_)
            | AnalysisDraft::AcData(_)
            | AnalysisDraft::Fft(_) => {}
        }
    }
}
