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
use crate::workbench::design_system::WorkbenchIcon;

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

    /// A failure, named by whatever named it.
    ///
    /// The canonical description says only that "the requested viewer
    /// operation failed", which is true of every failure and identifies none
    /// of them. This banner sits directly above the control that acts on the
    /// failure, so the sentence over that control has to say what failed.
    fn failed(run: &SimulationRun, blocks_visuals: bool) -> Self {
        Self {
            state: ResultOperationalState::Failed,
            detail: failed_analysis_detail(run),
            blocks_visuals,
            dismissible: false,
        }
    }
}

/// What actually failed in `run`, in the words of the owner that produced it.
///
/// The engine's own attribution headline first: that is the classification the
/// engine committed to, and the same one the failure-site control below the
/// banner offers to mark. Otherwise the analysis's error message verbatim.
/// Never a sentence assembled here out of parts — a failure this surface
/// invented would be worse than a generic one.
///
/// A live partial is written `success == false` with a reserved message, so it
/// is told from a real failure before the success bit is read at all: a run
/// still streaming has not failed.
fn failed_analysis_detail(run: &SimulationRun) -> Option<String> {
    let analysis = run
        .analyses
        .iter()
        .find(|analysis| !analysis.success && !analysis.is_live_partial())?;
    let name = analysis.label.as_str();
    if let Some(attribution) = analysis.failure_attribution.as_ref() {
        return Some(format!("{name}: {}", attribution.class.headline()));
    }
    analysis
        .error_message
        .as_deref()
        .map(|message| format!("{name}: {message}"))
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
            return ResultOperationalStatus::failed(run, !run_has_data);
        }
        SimulationRunLifecycle::Completed | SimulationRunLifecycle::LegacyUnknown => {}
    }

    if !run_has_data {
        return if run.success {
            ResultOperationalStatus::canonical(ResultOperationalState::NoDataset, true)
        } else {
            ResultOperationalStatus::failed(run, true)
        };
    }
    if !run.success || run.analyses.iter().any(|analysis| !analysis.success) {
        return ResultOperationalStatus::failed(run, false);
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
    // Derived before the card borrows `state` mutably for the dismiss path.
    let offer = failure_site_offer(state);
    let marked = offer
        .as_ref()
        .is_some_and(|offer| state.ui.results.marked_failure_run == Some(offer.run_sequence));
    let card = OperationalCard {
        status,
        accent,
        offer: offer.as_ref(),
        marked,
        floating: status.blocks_visuals,
    };
    let mut actions = OperationalCardActions::default();
    let response = if status.blocks_visuals {
        card.show_centered(ui, &mut actions)
    } else {
        card.show(ui, &mut actions)
    };
    let OperationalCardActions { dismiss, highlight } = actions;
    let accessible = format!(
        "{} status, {}: {} {}",
        status.state.id(),
        status.state.label(),
        status.detail.as_deref().unwrap_or(status.state.message()),
        status.state.recovery()
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, true, accessible.as_str()));
    ui.ctx().accesskit_node_builder(response.id, |node| {
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

/// The card's own geometry, as `.viewer-operational-state` defines it: a
/// bounded column centred in the stage, never a strip as long as whichever
/// sentence the state happens to carry.
const OPERATIONAL_CARD_MAX_WIDTH: f32 = 560.0;
const OPERATIONAL_CARD_GUTTER: f32 = 16.0;
const OPERATIONAL_CARD_PADDING: i8 = 14;
const OPERATIONAL_CARD_ICON: f32 = 20.0;
const OPERATIONAL_CARD_ICON_GAP: f32 = 10.0;
/// What the banner form keeps clear of the bar above and the viewer below.
const OPERATIONAL_BANNER_INSET: i8 = 8;

/// The glyph that stands for a category before its sentence is read.
const fn operational_icon(category: ResultOperationalCategory) -> WorkbenchIcon {
    match category {
        ResultOperationalCategory::Error
        | ResultOperationalCategory::Warning
        | ResultOperationalCategory::Partial => WorkbenchIcon::Warning,
        ResultOperationalCategory::Loading => WorkbenchIcon::Refresh,
        ResultOperationalCategory::Recovery | ResultOperationalCategory::Normal => {
            WorkbenchIcon::Success
        }
        ResultOperationalCategory::Empty => WorkbenchIcon::Info,
    }
}

/// What the reader pressed while the card was on screen.
#[derive(Default)]
struct OperationalCardActions {
    dismiss: bool,
    highlight: bool,
}

/// One operational card: the category glyph, then the state's label, detail,
/// message and recovery, then whatever this state lets the reader act on.
struct OperationalCard<'a> {
    status: &'a ResultOperationalStatus,
    accent: egui::Color32,
    offer: Option<&'a FailureSiteOffer>,
    marked: bool,
    /// The card that owns the well — elevated and shadowed over the canvas —
    /// rather than the banner attached above a still-usable viewer.
    floating: bool,
}

impl OperationalCard<'_> {
    /// Draw the card across the width its caller already fixed.
    fn show(&self, ui: &mut Ui, actions: &mut OperationalCardActions) -> egui::Response {
        let t = Tokens::get(ui.ctx());
        // Severity outlines the card; an empty or loading state is not an
        // alarm and keeps the ordinary strong border under its heading.
        let border = match self.status.state.category() {
            ResultOperationalCategory::Normal
            | ResultOperationalCategory::Empty
            | ResultOperationalCategory::Loading => t.color.border_strong,
            ResultOperationalCategory::Partial
            | ResultOperationalCategory::Warning
            | ResultOperationalCategory::Error
            | ResultOperationalCategory::Recovery => self.accent,
        };
        let mut frame = egui::Frame::NONE
            .fill(if self.floating {
                t.color.bg_elevated
            } else {
                t.color.bg_panel
            })
            .stroke(egui::Stroke::new(1.0, border))
            .corner_radius(t.radius)
            .inner_margin(egui::Margin::same(OPERATIONAL_CARD_PADDING));
        if self.floating {
            frame = frame.shadow(t.shadow());
        } else {
            // The banner is inset from the well it warns about, so it neither
            // butts against the bar above it nor against the viewer below.
            frame = frame.outer_margin(egui::Margin::same(OPERATIONAL_BANNER_INSET));
        }
        frame
            .show(ui, |ui| {
                // Both forms are laid out to a width their caller fixed, so
                // the copy wraps inside the card rather than setting its
                // width — a state's longest sentence is not a layout.
                let width = ui.available_width();
                ui.set_width(width);
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = OPERATIONAL_CARD_ICON_GAP;
                    let (glyph, _) = ui.allocate_exact_size(
                        egui::Vec2::splat(OPERATIONAL_CARD_ICON),
                        egui::Sense::hover(),
                    );
                    operational_icon(self.status.state.category()).paint(
                        ui.painter(),
                        glyph,
                        self.accent,
                    );
                    let copy = (width - OPERATIONAL_CARD_ICON - OPERATIONAL_CARD_ICON_GAP).max(1.0);
                    ui.allocate_ui_with_layout(
                        egui::vec2(copy, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_width(copy);
                            ui.spacing_mut().item_spacing.y = 3.0;
                            ui.label(
                                egui::RichText::new(self.status.state.label())
                                    .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                                    .color(self.accent),
                            );
                            if let Some(detail) = self.status.detail.as_deref() {
                                ui.label(
                                    egui::RichText::new(detail)
                                        .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                                        .color(t.color.text),
                                );
                            }
                            ui.label(
                                egui::RichText::new(self.status.state.message())
                                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                                    .color(t.color.text_dim),
                            );
                            ui.label(
                                egui::RichText::new(self.status.state.recovery())
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text_faint),
                            );
                        },
                    );
                });
                if self.offer.is_some() || self.status.dismissible {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if let Some(offer) = self.offer {
                            actions.highlight = show_failure_site_control(ui, offer, self.marked);
                        }
                        if self.status.dismissible {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    actions.dismiss = crate::ui::widgets::Button::new("Dismiss")
                                        .show(ui)
                                        .on_hover_text("Dismiss this recorded runtime notice")
                                        .clicked();
                                },
                            );
                        }
                    });
                }
            })
            .response
    }

    /// Centre the card in the document well it has taken over.
    ///
    /// The card is measured at its final width first, so it lands on the
    /// well's own centre. What this replaces was a guessed 172 pt height
    /// halved against the well's left edge, which put the card somewhere
    /// different for every state and in the middle for none of them.
    fn show_centered(&self, ui: &mut Ui, actions: &mut OperationalCardActions) -> egui::Response {
        let well = ui.available_rect_before_wrap();
        let width =
            (well.width() - 2.0 * OPERATIONAL_CARD_GUTTER).clamp(1.0, OPERATIONAL_CARD_MAX_WIDTH);

        let mut probe = ui.new_child(
            egui::UiBuilder::new()
                .id_salt("result-operational-state-measure")
                .max_rect(egui::Rect::from_min_size(
                    well.min,
                    egui::vec2(width, well.height().max(OPERATIONAL_CARD_MAX_WIDTH)),
                ))
                .layout(egui::Layout::top_down(egui::Align::Min))
                .sizing_pass()
                .invisible(),
        );
        // The measuring pass presses nothing; what it reports is thrown away.
        self.show(&mut probe, &mut OperationalCardActions::default());
        let height = probe.min_rect().height();

        let top = (well.center().y - height * 0.5).max(well.top() + OPERATIONAL_CARD_GUTTER);
        let mut card = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(egui::Rect::from_min_size(
                    egui::pos2((well.center().x - width * 0.5).round(), top.round()),
                    egui::vec2(width, height),
                ))
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        self.show(&mut card, actions)
    }
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
/// The workbench's own Button: it already marks its response disabled and
/// paints its own focus ring, so nothing is given up by drawing it in the
/// chrome every other control on this surface uses.
fn show_failure_site_control(ui: &mut Ui, offer: &FailureSiteOffer, marked: bool) -> bool {
    // "Objects", not "nodes": a site is a node *or* a branch current, which the
    // schematic draws as a device (`ConvergenceSiteKind`). The console's anchor
    // hint has always said "objects", and this control marks the same set.
    let label = if marked {
        "Clear highlighted sites".to_owned()
    } else if offer.named == 1 {
        "Highlight the 1 object this run named".to_owned()
    } else {
        format!("Highlight the {} objects this run named", offer.named)
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
    let clicked = crate::ui::widgets::Button::new(&label)
        .show(ui)
        .on_hover_text(&hint)
        .clicked();
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

#[cfg(test)]
mod presentation_tests {
    use super::*;
    use egui::Rect;

    /// The well the Results workspace hands a blocking state on a desktop.
    const WELL: egui::Vec2 = egui::vec2(1280.0, 720.0);

    /// Lay one blocking card out headlessly and report the well it was given
    /// alongside the rectangle it actually took.
    fn centered_card(state: ResultOperationalState, well_size: egui::Vec2) -> (Rect, Rect) {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let status = ResultOperationalStatus::canonical(state, true);
        let mut well = Rect::NOTHING;
        let mut card = Rect::NOTHING;
        // Fonts build on the first pass and sizing settles on the second, so
        // the geometry asserted on is the third pass's.
        for _ in 0..3 {
            let input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, well_size)),
                ..Default::default()
            };
            let _ = ctx.run_ui(input, |ui| {
                well = ui.available_rect_before_wrap();
                card = OperationalCard {
                    status: &status,
                    accent: Tokens::get(ui.ctx()).color.info,
                    offer: None,
                    marked: false,
                    floating: true,
                }
                .show_centered(ui, &mut OperationalCardActions::default())
                .rect;
            });
        }
        (well, card)
    }

    /// A state that owns the well is centred in it, on both axes. The offset
    /// this replaced was computed from a guessed card height against the
    /// well's left edge, so the card landed neither centred nor predictably.
    #[test]
    fn a_blocking_card_is_centred_in_the_well_it_owns() {
        let (well, card) = centered_card(ResultOperationalState::NoDataset, WELL);

        assert!(
            (card.center().x - well.center().x).abs() <= 1.0,
            "card centre {:?} is not the well centre {:?}",
            card.center(),
            well.center()
        );
        assert!(
            (card.center().y - well.center().y).abs() <= 1.0,
            "card centre {:?} is not the well centre {:?}",
            card.center(),
            well.center()
        );
    }

    /// Every state gets the same bounded column. Width used to be whatever
    /// the state's longest sentence happened to measure, so each state drew a
    /// differently sized card.
    #[test]
    fn card_width_is_the_bounded_column_and_not_the_length_of_the_sentence() {
        let (_, empty) = centered_card(ResultOperationalState::NoDataset, WELL);
        let (_, partial) = centered_card(ResultOperationalState::Partial, WELL);
        let (_, offline) = centered_card(ResultOperationalState::Offline, WELL);

        assert_eq!(empty.width(), OPERATIONAL_CARD_MAX_WIDTH);
        assert_eq!(partial.width(), empty.width());
        assert_eq!(offline.width(), empty.width());
    }

    /// Below the bounded column the card keeps its gutter rather than
    /// touching, or overflowing, the well's edges.
    #[test]
    fn a_narrow_well_keeps_the_card_inside_its_gutter() {
        let narrow = egui::vec2(360.0, 540.0);
        let (well, card) = centered_card(ResultOperationalState::NoDataset, narrow);

        assert_eq!(card.width(), well.width() - 2.0 * OPERATIONAL_CARD_GUTTER);
        assert!(
            well.contains_rect(card),
            "{card:?} escaped the well {well:?}"
        );
    }

    /// A card taller than its well is pinned to the top of it, so the copy
    /// that explains the state is the part that stays on screen.
    #[test]
    fn a_card_taller_than_its_well_starts_at_the_top() {
        let short = egui::vec2(420.0, 90.0);
        let (well, card) = centered_card(ResultOperationalState::NoDataset, short);

        assert!(card.height() > well.height(), "the case under test");
        assert_eq!(card.top(), (well.top() + OPERATIONAL_CARD_GUTTER).round());
    }
}
