//! Evidence-backed operational state for the Results workspace.
//!
//! The mockup contract names states; this module decides when RSpice may
//! truthfully claim each one. Source currentness is derived from immutable
//! provenance and the current simulation-plan revision, never from which run
//! row happens to be selected.

use egui::{Ui, WidgetInfo, WidgetType};

use super::{AnalysisPresentationKey, ResultViewer, ResultsState};
use crate::state::{
    AnalysisResult, AnalysisResultSourceDomain, SimulationRun, SimulationRunLifecycle,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app_state::AppState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResultOperationalCategory {
    Normal,
    Empty,
    Loading,
    Partial,
    Warning,
    Error,
    Recovery,
}

/// Canonical viewer-state vocabulary from `result-data-contract.json`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResultOperationalState {
    Complete,
    NoProject,
    NoDataset,
    Loading,
    Streaming,
    Partial,
    Stale,
    Failed,
    Corrupted,
    Unsupported,
    Offline,
    StorageDenied,
    LowMemory,
    RendererLoss,
    InterruptedOperation,
    Recovered,
}

impl ResultOperationalState {
    pub(crate) const ALL: [Self; 16] = [
        Self::Complete,
        Self::NoProject,
        Self::NoDataset,
        Self::Loading,
        Self::Streaming,
        Self::Partial,
        Self::Stale,
        Self::Failed,
        Self::Corrupted,
        Self::Unsupported,
        Self::Offline,
        Self::StorageDenied,
        Self::LowMemory,
        Self::RendererLoss,
        Self::InterruptedOperation,
        Self::Recovered,
    ];

    pub(crate) const fn id(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::NoProject => "no-project",
            Self::NoDataset => "no-dataset",
            Self::Loading => "loading",
            Self::Streaming => "streaming",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Failed => "failed",
            Self::Corrupted => "corrupted",
            Self::Unsupported => "unsupported",
            Self::Offline => "offline",
            Self::StorageDenied => "storage-denied",
            Self::LowMemory => "low-memory",
            Self::RendererLoss => "renderer-loss",
            Self::InterruptedOperation => "interrupted-operation",
            Self::Recovered => "recovered",
        }
    }

    pub(crate) const fn category(self) -> ResultOperationalCategory {
        match self {
            Self::Complete => ResultOperationalCategory::Normal,
            Self::NoProject | Self::NoDataset => ResultOperationalCategory::Empty,
            Self::Loading | Self::Streaming => ResultOperationalCategory::Loading,
            Self::Partial => ResultOperationalCategory::Partial,
            Self::Stale
            | Self::Unsupported
            | Self::Offline
            | Self::LowMemory
            | Self::InterruptedOperation => ResultOperationalCategory::Warning,
            Self::Failed | Self::Corrupted | Self::StorageDenied | Self::RendererLoss => {
                ResultOperationalCategory::Error
            }
            Self::Recovered => ResultOperationalCategory::Recovery,
        }
    }

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Complete => "Complete",
            Self::NoProject => "No project",
            Self::NoDataset => "No compatible dataset",
            Self::Loading => "Loading metadata",
            Self::Streaming => "Streaming",
            Self::Partial => "Partial result",
            Self::Stale => "Stale source",
            Self::Failed => "Result operation failed",
            Self::Corrupted => "Integrity failure",
            Self::Unsupported => "Unsupported viewer contract",
            Self::Offline => "Offline",
            Self::StorageDenied => "Storage unavailable",
            Self::LowMemory => "Memory pressure",
            Self::RendererLoss => "Renderer unavailable",
            Self::InterruptedOperation => "Operation interrupted",
            Self::Recovered => "Recovered",
        }
    }

    pub(crate) const fn message(self) -> &'static str {
        match self {
            Self::Complete => {
                "All declared source scopes are available and exact-data consumers resolve normally."
            }
            Self::NoProject => "No project owns a result document or dataset binding.",
            Self::NoDataset => {
                "The viewer document exists but has no compatible immutable dataset binding."
            }
            Self::Loading => {
                "Manifest and schema metadata are loading; numeric access has not been claimed."
            }
            Self::Streaming => {
                "Verified chunks are arriving progressively; unavailable ranges remain explicit."
            }
            Self::Partial => {
                "Only the disclosed verified scope is available; absent scope is not inferred or interpolated."
            }
            Self::Stale => "A dependency changed after this immutable result revision was created.",
            Self::Failed => {
                "The requested viewer operation failed without replacing the prior valid state."
            }
            Self::Corrupted => {
                "Dataset bytes or structure failed verification and numeric access is quarantined."
            }
            Self::Unsupported => {
                "The required analysis, producer, transform, or representation is unavailable."
            }
            Self::Offline => {
                "Remote-only source chunks or services are unavailable; cached verified scope remains identified."
            }
            Self::StorageDenied => {
                "Quota, permission, or durable storage policy blocked the requested operation."
            }
            Self::LowMemory => {
                "Rendering detail was reduced under policy; exact engineering queries remain source-backed."
            }
            Self::RendererLoss => {
                "The visual renderer was lost; source data and exact structured access remain intact."
            }
            Self::InterruptedOperation => {
                "An import, comparison, derivation, or publication operation stopped before commit."
            }
            Self::Recovered => {
                "The viewer was reconstructed from verified document and dataset state after a recorded failure."
            }
        }
    }

    pub(crate) const fn recovery(self) -> &'static str {
        match self {
            Self::Complete => "No recovery is required.",
            Self::NoProject => {
                "Open or create a project, then return to the preserved viewer route."
            }
            Self::NoDataset => {
                "Bind a compatible dataset or choose another viewer without fabricating values."
            }
            Self::Loading => "Cancel or retry while retaining the document and source identities.",
            Self::Streaming => "Pause, cancel, or resume from the last verified chunk.",
            Self::Partial => "Continue acquisition or inspect the exact available-scope table.",
            Self::Stale => {
                "Keep historical review, compare revisions, or run again from a reviewed current plan."
            }
            Self::Failed => "Inspect diagnostics and retry the exact failed boundary.",
            Self::Corrupted => {
                "Verify another retained copy, restore from a trusted artifact, or keep quarantined diagnostics."
            }
            Self::Unsupported => {
                "Install or select an exact compatible capability; no fallback viewer is substituted."
            }
            Self::Offline => {
                "Reconnect or continue with the exact cached scope without implying completeness."
            }
            Self::StorageDenied => {
                "Choose an authorized destination, free scoped storage, or cancel without source mutation."
            }
            Self::LowMemory => {
                "Release cached views, reduce visible scope, or retry after pressure clears."
            }
            Self::RendererLoss => {
                "Restart the renderer and restore viewport, selection, and presentation from the document revision."
            }
            Self::InterruptedOperation => {
                "Resume from the retained checkpoint or remove the incomplete candidate."
            }
            Self::Recovered => "Review the recovery receipt before continuing or publishing.",
        }
    }
}

/// Runtime boundaries that cannot be inferred from a retained result alone.
///
/// Permission and entitlement are deliberately distinct here even though the
/// canonical viewer contract groups unavailable capabilities under
/// `unsupported`: diagnostics must identify which authority refused access.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResultRuntimeConditionKind {
    IntegrityVerifying,
    OfflineCached,
    SourcePermissionDenied,
    EntitlementDenied,
    StorageDenied,
    LowMemory,
    RendererLoss,
    InterruptedOperation,
    Failed,
    Cancelled,
    Recovered,
}

impl ResultRuntimeConditionKind {
    pub(crate) const ALL: [Self; 11] = [
        Self::IntegrityVerifying,
        Self::OfflineCached,
        Self::SourcePermissionDenied,
        Self::EntitlementDenied,
        Self::StorageDenied,
        Self::LowMemory,
        Self::RendererLoss,
        Self::InterruptedOperation,
        Self::Failed,
        Self::Cancelled,
        Self::Recovered,
    ];

    pub(crate) const fn browser_state_id(self) -> &'static str {
        match self {
            Self::IntegrityVerifying => "integrity-verifying",
            Self::OfflineCached => "offline-cached",
            Self::SourcePermissionDenied => "permission-denied",
            Self::EntitlementDenied => "entitlement-denied",
            Self::StorageDenied => "storage-denied",
            Self::LowMemory => "low-memory",
            Self::RendererLoss => "renderer-loss",
            Self::InterruptedOperation => "interrupted-operation",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Recovered => "recovered",
        }
    }

    const fn operational_state(self) -> ResultOperationalState {
        match self {
            Self::IntegrityVerifying => ResultOperationalState::Loading,
            Self::OfflineCached => ResultOperationalState::Offline,
            Self::SourcePermissionDenied | Self::EntitlementDenied => {
                ResultOperationalState::Unsupported
            }
            Self::StorageDenied => ResultOperationalState::StorageDenied,
            Self::LowMemory => ResultOperationalState::LowMemory,
            Self::RendererLoss => ResultOperationalState::RendererLoss,
            Self::InterruptedOperation | Self::Cancelled => {
                ResultOperationalState::InterruptedOperation
            }
            Self::Failed => ResultOperationalState::Failed,
            Self::Recovered => ResultOperationalState::Recovered,
        }
    }

    const fn always_blocks_visuals(self) -> bool {
        matches!(self, Self::RendererLoss)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ResultRuntimeCondition {
    kind: ResultRuntimeConditionKind,
    detail: String,
    data_version: u64,
}

impl ResultsState {
    pub(crate) fn record_runtime_condition(
        &mut self,
        kind: ResultRuntimeConditionKind,
        detail: impl Into<String>,
        data_version: u64,
    ) {
        debug_assert!(ResultRuntimeConditionKind::ALL.contains(&kind));
        debug_assert!(!kind.browser_state_id().is_empty());
        self.operational_condition = Some(ResultRuntimeCondition {
            kind,
            detail: detail.into(),
            data_version,
        });
    }

    pub(crate) fn clear_runtime_condition(&mut self, kind: ResultRuntimeConditionKind) {
        if self
            .operational_condition
            .as_ref()
            .is_some_and(|condition| condition.kind == kind)
        {
            self.operational_condition = None;
        }
    }

    pub(crate) fn record_runtime_recovery_if(
        &mut self,
        failed_kind: ResultRuntimeConditionKind,
        detail: impl Into<String>,
        data_version: u64,
    ) {
        if self
            .operational_condition
            .as_ref()
            .is_some_and(|condition| {
                condition.kind == failed_kind && condition.data_version == data_version
            })
        {
            self.record_runtime_condition(
                ResultRuntimeConditionKind::Recovered,
                detail,
                data_version,
            );
        }
    }

    pub(crate) fn dismiss_runtime_condition(&mut self) {
        self.operational_condition = None;
    }
}

/// Source relationship published by the Data Browser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResultCurrentness {
    Current,
    Stale,
    Partial,
    Superseded,
    Unresolved,
    Corrupted,
    Recovered,
}

impl ResultCurrentness {
    pub(crate) const ALL: [Self; 7] = [
        Self::Current,
        Self::Stale,
        Self::Partial,
        Self::Superseded,
        Self::Unresolved,
        Self::Corrupted,
        Self::Recovered,
    ];

    pub(crate) const fn id(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Partial => "partial",
            Self::Superseded => "superseded",
            Self::Unresolved => "unresolved",
            Self::Corrupted => "corrupted",
            Self::Recovered => "recovered",
        }
    }
}

pub(crate) fn analysis_currentness(
    state: &AppState,
    run: &SimulationRun,
    analysis: &AnalysisResult,
    evidence_valid: bool,
) -> ResultCurrentness {
    debug_assert_eq!(ResultCurrentness::ALL.len(), 7);
    if !evidence_valid {
        return ResultCurrentness::Corrupted;
    }
    if analysis.is_live_partial()
        || matches!(
            run.lifecycle,
            SimulationRunLifecycle::Preparing
                | SimulationRunLifecycle::Running
                | SimulationRunLifecycle::Cancelling
                | SimulationRunLifecycle::Aborted
                | SimulationRunLifecycle::Interrupted
        )
    {
        return ResultCurrentness::Partial;
    }

    let Some(provenance) = analysis.provenance() else {
        // Imported and pre-provenance datasets are still exact readable
        // evidence. Their relationship to the active source is unknown; it is
        // not legitimate to call them either stale or recovered.
        return ResultCurrentness::Unresolved;
    };
    match provenance.source_domain() {
        AnalysisResultSourceDomain::ManualDeck | AnalysisResultSourceDomain::LegacyUnclassified => {
            return ResultCurrentness::Unresolved;
        }
        AnalysisResultSourceDomain::SimulationPlan => {}
    }

    let Ok(plan) = state.sim_setup.stable_analysis_plan() else {
        return ResultCurrentness::Unresolved;
    };
    if run
        .prepared_receipt()
        .and_then(|receipt| receipt.simulation_plan_id())
        .is_some_and(|plan_id| plan_id != plan.id())
    {
        return ResultCurrentness::Superseded;
    }

    let authored = provenance.authored_source_instance_id();
    if plan
        .instances()
        .iter()
        .any(|instance| instance.id() == authored)
    {
        if provenance.source_revision() == plan.revision() {
            ResultCurrentness::Current
        } else {
            ResultCurrentness::Stale
        }
    } else if plan
        .tombstones()
        .iter()
        .any(|tombstone| tombstone.id() == authored)
    {
        ResultCurrentness::Superseded
    } else {
        ResultCurrentness::Unresolved
    }
}

pub(crate) fn run_currentness(
    state: &AppState,
    run: &SimulationRun,
    evidence_valid: impl Fn(&AnalysisResult) -> bool,
) -> ResultCurrentness {
    let mut currentness = ResultCurrentness::Current;
    for analysis in &run.analyses {
        let candidate = analysis_currentness(state, run, analysis, evidence_valid(analysis));
        if currentness_rank(candidate) > currentness_rank(currentness) {
            currentness = candidate;
        }
    }
    if run.analyses.is_empty() {
        ResultCurrentness::Unresolved
    } else if currentness_rank(currentness) <= currentness_rank(ResultCurrentness::Unresolved)
        && state
            .ui
            .results
            .operational_condition
            .as_ref()
            .is_some_and(|condition| {
                condition.kind == ResultRuntimeConditionKind::Recovered
                    && condition.data_version == state.simulation.data_version
                    && state
                        .simulation
                        .active_run()
                        .is_some_and(|active| active.dataset_id == run.dataset_id)
            })
    {
        ResultCurrentness::Recovered
    } else {
        currentness
    }
}

const fn currentness_rank(currentness: ResultCurrentness) -> u8 {
    match currentness {
        ResultCurrentness::Current => 0,
        ResultCurrentness::Recovered => 1,
        ResultCurrentness::Unresolved => 2,
        ResultCurrentness::Superseded => 3,
        ResultCurrentness::Stale => 4,
        ResultCurrentness::Partial => 5,
        ResultCurrentness::Corrupted => 6,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResultOperationalStatus {
    pub(crate) state: ResultOperationalState,
    pub(crate) detail: Option<String>,
    pub(crate) blocks_visuals: bool,
    pub(crate) dismissible: bool,
}

impl ResultOperationalStatus {
    fn canonical(state: ResultOperationalState, blocks_visuals: bool) -> Self {
        Self {
            state,
            detail: None,
            blocks_visuals,
            dismissible: false,
        }
    }
}

pub(crate) fn classify_viewer(
    state: &mut AppState,
    viewer: ResultViewer,
) -> ResultOperationalStatus {
    debug_assert_eq!(ResultOperationalState::ALL.len(), 16);
    if !state.project_lifecycle.project_open {
        return ResultOperationalStatus::canonical(ResultOperationalState::NoProject, true);
    }

    // Specifications owns an authoring route before any result exists.
    let authoring_without_dataset = viewer == ResultViewer::Specs
        && (state.ui.results.spec_drafts.is_some() || !state.workspace.specs.is_empty());
    let Some(run_index) = state
        .simulation
        .active_run_idx
        .filter(|index| state.simulation.runs.get(*index).is_some())
    else {
        return if authoring_without_dataset {
            ResultOperationalStatus::canonical(ResultOperationalState::Complete, false)
        } else {
            ResultOperationalStatus::canonical(ResultOperationalState::NoDataset, true)
        };
    };

    let (run_has_data, analysis_keys) = {
        let run = &state.simulation.runs[run_index];
        (
            !run.analyses.is_empty(),
            run.analyses
                .iter()
                .map(|analysis| AnalysisPresentationKey::new(run.dataset_id, analysis))
                .collect::<Vec<_>>(),
        )
    };
    if analysis_keys
        .into_iter()
        .any(|key| !super::retained_evidence_is_valid(state, key))
    {
        return ResultOperationalStatus::canonical(ResultOperationalState::Corrupted, true);
    }

    let run = &state.simulation.runs[run_index];

    if let Some(condition) = state.ui.results.operational_condition.clone()
        && condition.data_version == state.simulation.data_version
    {
        let operational_state = condition.kind.operational_state();
        return ResultOperationalStatus {
            state: operational_state,
            detail: Some(condition.detail),
            blocks_visuals: condition.kind.always_blocks_visuals() || !run_has_data,
            dismissible: !matches!(
                condition.kind,
                ResultRuntimeConditionKind::IntegrityVerifying
                    | ResultRuntimeConditionKind::RendererLoss
            ),
        };
    }

    match run.lifecycle {
        SimulationRunLifecycle::Preparing => {
            return ResultOperationalStatus::canonical(ResultOperationalState::Loading, true);
        }
        SimulationRunLifecycle::Running => {
            return ResultOperationalStatus::canonical(
                if run.analyses.iter().any(AnalysisResult::is_live_partial) {
                    ResultOperationalState::Streaming
                } else {
                    ResultOperationalState::Loading
                },
                !run_has_data,
            );
        }
        SimulationRunLifecycle::Cancelling => {
            return ResultOperationalStatus::canonical(
                ResultOperationalState::InterruptedOperation,
                !run_has_data,
            );
        }
        SimulationRunLifecycle::Aborted => {
            return ResultOperationalStatus::canonical(
                if run_has_data {
                    ResultOperationalState::Partial
                } else {
                    ResultOperationalState::InterruptedOperation
                },
                !run_has_data,
            );
        }
        SimulationRunLifecycle::Interrupted => {
            return ResultOperationalStatus::canonical(
                ResultOperationalState::InterruptedOperation,
                !run_has_data,
            );
        }
        SimulationRunLifecycle::Failed => {
            return ResultOperationalStatus::canonical(
                ResultOperationalState::Failed,
                !run_has_data,
            );
        }
        SimulationRunLifecycle::Completed | SimulationRunLifecycle::LegacyUnknown => {}
    }

    if !run_has_data {
        return ResultOperationalStatus::canonical(
            if run.success {
                ResultOperationalState::NoDataset
            } else {
                ResultOperationalState::Failed
            },
            true,
        );
    }
    if !run.success || run.analyses.iter().any(|analysis| !analysis.success) {
        return ResultOperationalStatus::canonical(ResultOperationalState::Failed, false);
    }

    let currentness = run_currentness(state, run, |_| true);
    if matches!(
        currentness,
        ResultCurrentness::Stale | ResultCurrentness::Superseded
    ) {
        return ResultOperationalStatus::canonical(ResultOperationalState::Stale, false);
    }

    if super::viewer_requires_retained_results(viewer)
        && !super::viewer_availability(state, viewer).available
    {
        return ResultOperationalStatus {
            state: ResultOperationalState::Unsupported,
            detail: super::viewer_unavailability_reason(state, viewer).map(str::to_owned),
            blocks_visuals: true,
            dismissible: false,
        };
    }

    ResultOperationalStatus::canonical(ResultOperationalState::Complete, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{ContentDigest, ObjectRevision};
    use crate::simulation::plan::AnalysisKind;
    use crate::state::{AnalysisResultProvenance, AnalysisType, WaveformData};

    #[test]
    fn canonical_operational_registry_matches_the_result_contract() {
        assert_eq!(
            ResultOperationalState::ALL.map(ResultOperationalState::id),
            [
                "complete",
                "no-project",
                "no-dataset",
                "loading",
                "streaming",
                "partial",
                "stale",
                "failed",
                "corrupted",
                "unsupported",
                "offline",
                "storage-denied",
                "low-memory",
                "renderer-loss",
                "interrupted-operation",
                "recovered",
            ]
        );
        for state in ResultOperationalState::ALL {
            assert!(!state.label().is_empty());
            assert!(!state.message().is_empty());
            assert!(!state.recovery().is_empty());
        }
    }

    #[test]
    fn browser_runtime_registry_keeps_access_and_verification_failures_distinct() {
        assert_eq!(
            ResultRuntimeConditionKind::ALL.map(ResultRuntimeConditionKind::browser_state_id),
            [
                "integrity-verifying",
                "offline-cached",
                "permission-denied",
                "entitlement-denied",
                "storage-denied",
                "low-memory",
                "renderer-loss",
                "interrupted-operation",
                "failed",
                "cancelled",
                "recovered",
            ]
        );
        assert_eq!(
            ResultRuntimeConditionKind::SourcePermissionDenied.operational_state(),
            ResultOperationalState::Unsupported
        );
        assert_eq!(
            ResultRuntimeConditionKind::EntitlementDenied.operational_state(),
            ResultOperationalState::Unsupported
        );
    }

    #[test]
    fn runtime_recovery_requires_the_exact_failure_kind_and_dataset_generation() {
        let mut results = ResultsState::default();
        results.record_runtime_condition(ResultRuntimeConditionKind::Failed, "export failed", 7);
        results.record_runtime_recovery_if(
            ResultRuntimeConditionKind::StorageDenied,
            "wrong boundary",
            7,
        );
        assert_eq!(
            results
                .operational_condition
                .as_ref()
                .map(|condition| condition.kind),
            Some(ResultRuntimeConditionKind::Failed)
        );
        results.record_runtime_recovery_if(
            ResultRuntimeConditionKind::Failed,
            "export retry succeeded",
            8,
        );
        assert_eq!(
            results
                .operational_condition
                .as_ref()
                .map(|condition| condition.kind),
            Some(ResultRuntimeConditionKind::Failed)
        );
        results.record_runtime_recovery_if(
            ResultRuntimeConditionKind::Failed,
            "export retry succeeded",
            7,
        );
        assert_eq!(
            results
                .operational_condition
                .as_ref()
                .map(|condition| condition.kind),
            Some(ResultRuntimeConditionKind::Recovered)
        );
    }

    fn completed_run_with(analysis: AnalysisResult) -> SimulationRun {
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        run.mark_running().expect("fixture starts");
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("fixture completes");
        run
    }

    #[test]
    fn source_currentness_uses_plan_identity_and_revision_not_selection() {
        let state = AppState::default();
        let plan = state
            .sim_setup
            .stable_analysis_plan()
            .expect("default plan");
        let source = plan.instances()[0].id();
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![WaveformData::new(
                "V(out)",
                vec![0.0, 1.0],
                vec![0.0, 1.0],
                "#00aaff",
            )])
            .with_provenance(
                AnalysisResultProvenance::new(
                    source,
                    plan.revision(),
                    ContentDigest::from_bytes([0x42; 32]),
                    Vec::new(),
                )
                .expect("valid provenance"),
            );
        let run = completed_run_with(analysis.clone());
        assert_eq!(
            analysis_currentness(&state, &run, &analysis, true),
            ResultCurrentness::Current
        );

        let stale = analysis.with_provenance(
            AnalysisResultProvenance::new(
                source,
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x43; 32]),
                Vec::new(),
            )
            .expect("valid stale provenance"),
        );
        let mut revised = state;
        revised
            .sim_setup
            .stable_analysis_plan_mut()
            .expect("mutable plan")
            .insert(AnalysisKind::Ac)
            .expect("plan revision advances");
        let stale_run = completed_run_with(stale.clone());
        assert_eq!(
            analysis_currentness(&revised, &stale_run, &stale, true),
            ResultCurrentness::Stale
        );
        assert_eq!(
            analysis_currentness(&revised, &stale_run, &stale, false),
            ResultCurrentness::Corrupted
        );
    }
}

// ---------------------------------------------------------------------------
// presentation
// ---------------------------------------------------------------------------

/// Present one canonical Results operational state without replacing valid
/// retained evidence. Blocking states own the well; warnings and recovery
/// notices consume only a bounded banner above the still-usable viewer.
pub(super) fn show_result_operational_status(
    ui: &mut Ui,
    state: &mut AppState,
    status: &ResultOperationalStatus,
) -> bool {
    // The classifier and its banner are the same concern: the states are
    // named here, so what each one looks like is named here too.

    if status.state == ResultOperationalState::Complete {
        return false;
    }
    let t = Tokens::get(ui.ctx());
    let accent = match status.state.category() {
        ResultOperationalCategory::Normal | ResultOperationalCategory::Recovery => t.color.ok,
        ResultOperationalCategory::Empty | ResultOperationalCategory::Loading => t.color.info,
        ResultOperationalCategory::Partial | ResultOperationalCategory::Warning => t.color.warn,
        ResultOperationalCategory::Error => t.color.err,
    };
    if status.blocks_visuals {
        let available = ui.available_rect_before_wrap();
        ui.add_space(((available.height() - 172.0) * 0.5).max(12.0));
    }
    // Derived before the frame borrows `state` mutably for the dismiss path.
    let offer = failure_site_offer(state);
    let marked = offer
        .as_ref()
        .is_some_and(|offer| state.ui.results.marked_failure_run == Some(offer.run_sequence));
    let mut highlight = false;
    let mut dismiss = false;
    let response = egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(egui::Stroke::new(1.0, accent))
        .corner_radius(t.radius)
        .inner_margin(egui::Margin::symmetric(14, 11))
        .show(ui, |ui| {
            ui.set_max_width(if status.blocks_visuals {
                680.0_f32.min(ui.available_width())
            } else {
                ui.available_width()
            });
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(status.state.label())
                            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                            .color(accent),
                    );
                    if let Some(detail) = status.detail.as_deref() {
                        ui.label(
                            egui::RichText::new(detail)
                                .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                                .color(t.color.text),
                        );
                    }
                    ui.label(
                        egui::RichText::new(status.state.message())
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                    ui.label(
                        egui::RichText::new(status.state.recovery())
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_faint),
                    );
                });
                if status.dismissible {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        dismiss = ui
                            .button("Dismiss")
                            .on_hover_text("Dismiss this recorded runtime notice")
                            .clicked();
                    });
                }
            });
            if let Some(offer) = offer.as_ref() {
                ui.add_space(7.0);
                highlight = show_failure_site_control(ui, offer, marked);
            }
        });
    let accessible = format!(
        "{} status, {}: {} {}",
        status.state.id(),
        status.state.label(),
        status.detail.as_deref().unwrap_or(status.state.message()),
        status.state.recovery()
    );
    response
        .response
        .widget_info(|| WidgetInfo::labeled(WidgetType::Label, true, accessible.as_str()));
    ui.ctx()
        .accesskit_node_builder(response.response.id, |node| {
            node.set_role(
                if status.state.category() == ResultOperationalCategory::Error {
                    egui::accesskit::Role::Alert
                } else {
                    egui::accesskit::Role::Status
                },
            );
            node.set_label(accessible);
        });
    if dismiss {
        state.ui.results.dismiss_runtime_condition();
    }
    if highlight {
        if marked {
            state.clear_failure_site_marking();
            state.ui.results.marked_failure_run = None;
        } else if state.highlight_active_failure_sites() {
            state.ui.results.marked_failure_run = offer.as_ref().map(|offer| offer.run_sequence);
        }
    }
    status.blocks_visuals
}

/// What the failure on display named, when it named anything markable.
struct FailureSiteOffer {
    run_sequence: u64,
    /// Objects the attribution named, and that the control will mark.
    named: usize,
    /// Objects the engine measured but did not name.
    elided: usize,
    /// The class headline, for the control's own explanation.
    headline: &'static str,
}

/// Whether the analysis on display has attributed sites worth offering.
///
/// Two gates, and they answer different questions. `active_failure_names_objects`
/// asks whether there is an attribution naming anything at all. `describes`
/// asks whether that attribution belongs to *this* failure: the engine records
/// one at the moment a solve gives up, and a convergence aid may still rescue
/// that solve afterwards, so the freshest attribution is not automatically the
/// one behind the error on screen. Offering a control built on the wrong
/// attribution would mark objects that converged.
fn failure_site_offer(state: &AppState) -> Option<FailureSiteOffer> {
    if !state.active_failure_names_objects() {
        return None;
    }
    let run_sequence = state.simulation.active_run()?.id;
    let analysis = state.simulation.active_analysis()?;
    let attribution = analysis.failure_attribution.as_ref()?;
    let error = analysis.error_message.as_deref()?;
    if !attribution.describes(error) {
        return None;
    }
    Some(FailureSiteOffer {
        run_sequence,
        named: attribution.sites.len(),
        elided: attribution.elided_sites,
        headline: attribution.class.headline(),
    })
}

/// The control that marks what the failure named, or takes the marking back.
///
/// It says how many objects it will mark before it is pressed, because "mark
/// the offending nodes" is a different proposition at four nodes than at
/// thirty — and it says how many the engine measured but did not name, so the
/// marking is not read as the complete set.
///
/// A plain `Button` on purpose: it is one of egui's own widgets, so its
/// disabled and focus states are handled for it. A self-painted row here
/// would have to clear the `ENABLED` accessibility bit itself.
fn show_failure_site_control(ui: &mut Ui, offer: &FailureSiteOffer, marked: bool) -> bool {
    let label = if marked {
        "Clear highlighted sites".to_owned()
    } else if offer.named == 1 {
        "Highlight the 1 node this run named".to_owned()
    } else {
        format!("Highlight the {} nodes this run named", offer.named)
    };
    let hint = if marked {
        "Remove the marking from the drawing".to_owned()
    } else if offer.elided > 0 {
        format!(
            "{} — {} named, {} more measured but not named",
            offer.headline, offer.named, offer.elided
        )
    } else {
        format!(
            "{} — marks all {} on the drawing",
            offer.headline, offer.named
        )
    };
    let clicked = ui.button(&label).on_hover_text(&hint).clicked();
    if !marked && offer.elided > 0 {
        let t = Tokens::get(ui.ctx());
        ui.label(
            egui::RichText::new(format!(
                "{} more were measured but not named.",
                offer.elided
            ))
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_faint),
        );
    }
    clicked
}

#[cfg(test)]
mod failure_site_control_tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, ConvergenceAttribution};

    const FAILURE: &str = "Newton did not converge after 100 iterations";

    /// Built through the engine's own record, so the test exercises the path
    /// a real attribution takes rather than a hand-assembled stand-in.
    fn attribution(failure_message: &str, elided: usize) -> ConvergenceAttribution {
        use rspice_core::diagnostics as core;

        let site = |name: &str, residual: f64| core::ConvergenceSite {
            name: name.to_owned(),
            kind: core::ConvergenceSiteKind::Node,
            residual: Some(residual),
        };
        ConvergenceAttribution::from(&core::ConvergenceDiagnostic {
            class: core::ConvergenceFailureClass::NewtonNonConvergence,
            sites: vec![site("OUT", 4.0), site("MID", 2.0)],
            elided_sites: elided,
            failure_message: failure_message.to_owned(),
        })
    }

    /// A failed analysis on display, whose recorded attribution says whether
    /// it belongs to the error the analysis actually carries.
    fn state_showing_failure(
        attribution: Option<ConvergenceAttribution>,
        rendered_error: &str,
    ) -> AppState {
        let mut state = AppState::default();
        let mut analysis =
            AnalysisResult::failed(1, AnalysisType::Transient, "tran", rendered_error);
        analysis.failure_attribution = attribution;
        state.simulation.start_run().add_analysis(analysis);
        state.simulation.select_run(0);
        state
    }

    /// The control is offered for the failure it was recorded against.
    #[test]
    fn a_matching_attribution_offers_the_control_and_counts_what_it_marks() {
        let state = state_showing_failure(Some(attribution(FAILURE, 9)), FAILURE);

        let offer = failure_site_offer(&state).expect("the attribution describes this failure");
        assert_eq!(
            offer.named, 2,
            "it must state how many objects it will mark"
        );
        assert_eq!(offer.elided, 9, "and how many the engine did not name");
        assert_eq!(offer.headline, "Did not converge");
    }

    /// The engine records an attribution whenever a solve gives up, and a
    /// convergence aid may still rescue that solve. An attribution recorded
    /// against some other failure must not offer to mark objects for this one.
    #[test]
    fn an_attribution_for_another_failure_offers_nothing() {
        let state =
            state_showing_failure(Some(attribution("Singular matrix at row 4", 0)), FAILURE);

        assert!(
            state.active_failure_names_objects(),
            "there is an attribution naming objects — that gate alone is not enough"
        );
        assert!(
            failure_site_offer(&state).is_none(),
            "but it does not describe the failure on display, so nothing is offered"
        );
    }

    /// A project written before the failure message was retained cannot prove
    /// the pairing, and an unprovable pairing must not mark anything.
    #[test]
    fn an_attribution_that_cannot_prove_its_pairing_offers_nothing() {
        let state = state_showing_failure(Some(attribution("", 0)), FAILURE);

        assert!(failure_site_offer(&state).is_none());
    }

    /// No attribution at all is the ordinary case for a failure the engine
    /// could not attribute, and it carries no control.
    #[test]
    fn a_failure_naming_no_objects_offers_nothing() {
        let state = state_showing_failure(None, FAILURE);

        assert!(!state.active_failure_names_objects());
        assert!(failure_site_offer(&state).is_none());
    }

    /// The control takes its own marking back, and nothing else.
    #[test]
    fn clearing_removes_the_marking_without_moving_the_view() {
        let mut state = state_showing_failure(Some(attribution(FAILURE, 0)), FAILURE);
        state.schematic.selection.select_wire(91);
        state
            .schematic
            .net_highlight
            .highlight_wires(std::iter::once(91).collect());
        state.schematic.center_request = None;

        state.clear_failure_site_marking();

        assert!(state.schematic.selection.wires.is_empty());
        assert!(!state.schematic.net_highlight.active);
        assert!(
            state.schematic.center_request.is_none(),
            "clearing a marking must not scroll the drawing"
        );
    }
}
