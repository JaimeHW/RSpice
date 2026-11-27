//! Evidence-backed operational state for the Results workspace.
//!
//! The mockup contract names states; this module decides when RSpice may
//! truthfully claim each one. Source currentness is derived from immutable
//! provenance and the current simulation-plan revision, never from which run
//! row happens to be selected.

use egui::Ui;
use rspice_results_ui::operational::{
    self, CardOffers, ResultOperationalCategory, ResultOperationalState, ResultOperationalStatus,
    RunAgainControl, StatusWording,
};

use super::{AnalysisPresentationKey, ResultViewer, ResultsParts, ResultsReads, ResultsState};
use crate::state::{
    AnalysisResult, AnalysisResultSourceDomain, AnalysisType, SimulationRun, SimulationRunLifecycle,
};
#[cfg(test)]
use crate::workbench::documents::result_document::test_host::ResultsTestHost;
use crate::workbench::framework::command_registry::{CommandAvailability, ids};
use crate::workbench::framework::feature::Execute;
use crate::workbench::framework::requests::MarkFailureSites;
use rspice_app_state::workbench::app_state::model::Model;

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
    Cancelled,
}

impl ResultRuntimeConditionKind {
    pub(crate) const ALL: [Self; 9] = [
        Self::IntegrityVerifying,
        Self::OfflineCached,
        Self::SourcePermissionDenied,
        Self::EntitlementDenied,
        Self::StorageDenied,
        Self::LowMemory,
        Self::RendererLoss,
        Self::InterruptedOperation,
        Self::Cancelled,
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
            Self::Cancelled => "cancelled",
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

    pub(crate) fn dismiss_runtime_condition(&mut self) {
        self.operational_condition = None;
    }
}

/// Source relationship published by the Data Browser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultCurrentness {
    Current,
    Stale,
    Partial,
    Superseded,
    Unresolved,
    Corrupted,
}

impl ResultCurrentness {
    pub const ALL: [Self; 6] = [
        Self::Current,
        Self::Stale,
        Self::Partial,
        Self::Superseded,
        Self::Unresolved,
        Self::Corrupted,
    ];

    pub(crate) const fn id(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Partial => "partial",
            Self::Superseded => "superseded",
            Self::Unresolved => "unresolved",
            Self::Corrupted => "corrupted",
        }
    }
}

pub(crate) fn analysis_currentness(
    state: ResultsReads<'_>,
    run: &SimulationRun,
    analysis: &AnalysisResult,
    evidence_valid: bool,
) -> ResultCurrentness {
    debug_assert_eq!(ResultCurrentness::ALL.len(), 6);
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

    let Ok(plan) = state.model.sim_setup.stable_analysis_plan() else {
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
    state: ResultsReads<'_>,
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
    } else {
        currentness
    }
}

const fn currentness_rank(currentness: ResultCurrentness) -> u8 {
    match currentness {
        ResultCurrentness::Current => 0,
        ResultCurrentness::Unresolved => 1,
        ResultCurrentness::Superseded => 2,
        ResultCurrentness::Stale => 3,
        ResultCurrentness::Partial => 4,
        ResultCurrentness::Corrupted => 5,
    }
}

/// A failure, named by whatever named it.
///
/// The canonical description says only that "the requested viewer
/// operation failed", which is true of every failure and identifies none
/// of them. This banner sits directly above the control that acts on the
/// failure, so the sentence over that control has to say what failed.
/// What a running analysis that is not drawn yet says: whether it is drawn as it runs, and how
/// far it has got when the engine reports that.
fn running_words(drawn_live: bool, progress: Option<f64>) -> String {
    let mut words = if drawn_live {
        "The plot starts as soon as the run writes its first results.".to_owned()
    } else {
        "This analysis is not drawn while it runs; its results appear when it ends.".to_owned()
    };
    if let Some(progress) = progress {
        words.push_str(&format!(" {:.0}% done.", progress * 100.0));
    }
    words
}

fn failed_status(run: &SimulationRun, blocks_visuals: bool) -> ResultOperationalStatus {
    ResultOperationalStatus {
        detail: failed_analysis_detail(run),
        ..ResultOperationalStatus::canonical(ResultOperationalState::Failed, blocks_visuals)
    }
}

/// Whether `analysis` failed. A live partial is written `success == false`
/// with a reserved message, so it is told from a real failure first: an
/// analysis still streaming has not failed.
fn analysis_failed(analysis: &AnalysisResult) -> bool {
    !analysis.success && !analysis.is_live_partial()
}

/// What a run's own mark says of a run whose data reads as failed. A run
/// some of whose analyses failed completed: it reads as how many failed, in
/// the warning tone, and as Failed only when every analysis failed.
pub(crate) fn run_failure_status(run: &SimulationRun) -> ResultOperationalStatus {
    let failed = run
        .analyses
        .iter()
        .filter(|analysis| analysis_failed(analysis))
        .count();
    let of = run.analyses.len();
    ResultOperationalStatus {
        wording: (failed > 0 && failed < of).then(|| StatusWording {
            label: format!("{failed} of {of} failed"),
            category: ResultOperationalCategory::Warning,
            message: "The run completed; each analysis that failed is marked in its listing, with its reason.",
        }),
        ..failed_status(run, false)
    }
}

/// What became of `run`, as Run details and the Details' State row say it:
/// its lifecycle, and for a run that completed with some of its analyses
/// failed, how many. A run any of whose analyses failed ends Failed, so a
/// failed run with an analysis that completed reads as completed too. A run
/// its results storage stopped says where and why, as the status bar does,
/// in this session or reopened.
pub(crate) fn run_state_words(run: &SimulationRun) -> String {
    if matches!(
        run.lifecycle,
        SimulationRunLifecycle::Completed | SimulationRunLifecycle::Failed
    ) && let Some(stop) = run.results_storage_stop()
    {
        return storage_stop_state_words(stop);
    }
    let failed = run
        .analyses
        .iter()
        .filter(|analysis| analysis_failed(analysis))
        .count();
    let completed = match run.lifecycle {
        SimulationRunLifecycle::Completed => failed > 0,
        SimulationRunLifecycle::Failed => failed > 0 && failed < run.analyses.len(),
        _ => false,
    };
    if completed {
        return if failed == 1 {
            "Completed, one analysis failed".to_owned()
        } else {
            format!("Completed, {failed} analyses failed")
        };
    }
    match run.lifecycle {
        SimulationRunLifecycle::LegacyUnknown => "Not recorded",
        SimulationRunLifecycle::Preparing => "Preparing",
        SimulationRunLifecycle::Running => "Running",
        SimulationRunLifecycle::Cancelling => "Stopping",
        SimulationRunLifecycle::Completed => "Completed",
        SimulationRunLifecycle::Failed => "Failed",
        SimulationRunLifecycle::Aborted => "Stopped",
        SimulationRunLifecycle::Interrupted => "Interrupted",
    }
    .to_owned()
}

/// How the results storage stopped a run, as its State row says it: where
/// the run stopped and why, or that none of it was kept.
fn storage_stop_state_words(stop: &crate::state::ResultsStorageStop) -> String {
    if stop.kept_nothing() {
        return "Results not kept: storage full".to_owned();
    }
    let at = stop
        .at
        .map(|at| {
            format!(
                " at {}s",
                rspice_app_state::quantity::engineering::format_engineering_display_with(
                    at,
                    rspice_app_state::quantity::engineering::EngineeringPrecision::Adaptive,
                )
            )
        })
        .unwrap_or_default();
    let cause = if stop.budget.is_some() {
        "storage budget reached"
    } else {
        "results storage full"
    };
    format!("Stopped{at}: {cause}")
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
    analysis_failure_detail(
        run.analyses
            .iter()
            .find(|analysis| analysis_failed(analysis))?,
    )
}

/// What failed in `analysis`, as [`failed_analysis_detail`] says it.
fn analysis_failure_detail(analysis: &AnalysisResult) -> Option<String> {
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
    state: &mut ResultsParts<'_>,
    viewer: ResultViewer,
) -> ResultOperationalStatus {
    debug_assert_eq!(ResultOperationalState::ALL.len(), 16);
    if !state.platform.project_lifecycle.is_open() {
        return ResultOperationalStatus::canonical(ResultOperationalState::NoProject, true);
    }

    let Some(run_index) = state
        .model
        .simulation
        .view
        .active_run_idx
        .filter(|index| state.model.simulation.retained.runs.get(*index).is_some())
    else {
        return ResultOperationalStatus::canonical(ResultOperationalState::NoDataset, true);
    };

    let (run_has_data, analysis_keys) = {
        let run = &state.model.simulation.retained.runs[run_index];
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
        .any(|key| !super::retained_evidence_is_valid(state.results_reads(), key))
    {
        return ResultOperationalStatus::canonical(ResultOperationalState::Corrupted, true);
    }

    let run = &state.model.simulation.retained.runs[run_index];

    if let Some(condition) = state.results.operational_condition.clone()
        && condition.data_version == state.model.simulation.view.data_version
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
            wording: None,
        };
    }

    match run.lifecycle {
        SimulationRunLifecycle::Preparing => {
            return ResultOperationalStatus {
                detail: Some("The run is being prepared.".to_owned()),
                ..ResultOperationalStatus::canonical(ResultOperationalState::Loading, true)
            };
        }
        SimulationRunLifecycle::Running => {
            let execution = &state.model.simulation.execution;
            if run
                .analyses
                .iter()
                .any(|analysis| analysis.is_live_partial())
            {
                return ResultOperationalStatus {
                    detail: execution.live_preview_note.clone(),
                    ..ResultOperationalStatus::canonical(
                        ResultOperationalState::Streaming,
                        !run_has_data,
                    )
                };
            }
            return ResultOperationalStatus {
                detail: Some(running_words(
                    execution.live_window.is_some(),
                    execution.reported_progress,
                )),
                ..ResultOperationalStatus::canonical(ResultOperationalState::Loading, !run_has_data)
            };
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
        SimulationRunLifecycle::Failed
        | SimulationRunLifecycle::Completed
        | SimulationRunLifecycle::LegacyUnknown => {}
    }

    if !run_has_data {
        return if run.success {
            ResultOperationalStatus::canonical(ResultOperationalState::NoDataset, true)
        } else {
            failed_status(run, true)
        };
    }
    // A sheet reads the analysis it shows, not the run's worst: a transient
    // that completed is not failed because the run's AC analysis did. The
    // failure takes the well over when the sheet has nothing of it to draw.
    let shown = state
        .model
        .simulation
        .view
        .active_analysis_idx
        .and_then(|index| run.analyses.get(index));
    let failure = match shown {
        Some(analysis) if analysis_failed(analysis) => Some(ResultOperationalStatus {
            detail: analysis_failure_detail(analysis),
            ..ResultOperationalStatus::canonical(ResultOperationalState::Failed, false)
        }),
        Some(_) => None,
        // No analysis chosen: the sheet draws the run, failed when every
        // analysis did, or when the run failed with none to blame.
        None if run.analyses.iter().all(analysis_failed)
            || !run.analyses.iter().any(analysis_failed) && !run.success =>
        {
            Some(failed_status(run, false))
        }
        None => None,
    };
    if let Some(failure) = failure {
        let drawable = super::viewer_availability(state.results_reads(), viewer).available;
        return ResultOperationalStatus {
            blocks_visuals: !drawable,
            ..failure
        };
    }

    let currentness = run_currentness(state.results_reads(), run, |_| true);
    if matches!(
        currentness,
        ResultCurrentness::Stale | ResultCurrentness::Superseded
    ) {
        return ResultOperationalStatus::canonical(ResultOperationalState::Stale, false);
    }

    if !super::viewer_availability(state.results_reads(), viewer).available {
        return ResultOperationalStatus {
            detail: super::viewer_unavailability_reason(state.results_reads(), viewer)
                .map(str::to_owned),
            ..ResultOperationalStatus::canonical(ResultOperationalState::Unsupported, true)
        };
    }

    ResultOperationalStatus::canonical(ResultOperationalState::Complete, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{ContentDigest, ObjectRevision};
    use crate::simulation::plan::AnalysisKind;
    use crate::state::{AnalysisResultProvenance, AnalysisType, RetainedTrace};

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
                "cancelled",
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
        let state = ResultsTestHost::default();
        let plan = state
            .model
            .sim_setup
            .stable_analysis_plan()
            .expect("default plan");
        let source = plan.instances()[0].id();
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![RetainedTrace::new(
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
            analysis_currentness(state.results_reads(), &run, &analysis, true),
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
            .model
            .sim_setup
            .stable_analysis_plan_mut()
            .expect("mutable plan")
            .insert(AnalysisKind::Ac)
            .expect("plan revision advances");
        let stale_run = completed_run_with(stale.clone());
        assert_eq!(
            analysis_currentness(revised.results_reads(), &stale_run, &stale, true),
            ResultCurrentness::Stale
        );
        assert_eq!(
            analysis_currentness(revised.results_reads(), &stale_run, &stale, false),
            ResultCurrentness::Corrupted
        );
    }

    /// A running run's card says when its plot starts and how far the run got; once the plot
    /// is drawn, it says what the live plot left out, in the live plot's own words.
    #[test]
    fn a_running_runs_card_says_what_its_live_plot_left_out() {
        let mut state = ResultsTestHost::default();
        state.platform.project_lifecycle.authority.open_session();
        fn run(state: &mut ResultsTestHost, sequence: u64) -> &mut SimulationRun {
            state
                .model
                .simulation
                .retained
                .run_by_sequence_mut(sequence)
                .expect("the run is retained")
        }
        let sequence = state.model.simulation.start_run().id;
        run(&mut state, sequence)
            .mark_running()
            .expect("the run starts");
        state.model.simulation.execution.live_window = Some((0.0, 1e-3));
        state.model.simulation.execution.reported_progress = Some(0.25);

        let waiting = classify_viewer(&mut state.results_parts(), ResultViewer::Waves);
        assert_eq!(waiting.state, ResultOperationalState::Loading);
        assert_eq!(
            waiting.detail.as_deref(),
            Some("The plot starts as soon as the run writes its first results. 25% done.")
        );

        let note = "The live plot could not keep up and left out 6 points; the trace breaks \
                    where they were (1 gap). The finished result has every point.";
        state.model.simulation.execution.live_preview_note = Some(note.to_owned());
        // The live plot breaks its trace where the queue let points go.
        run(&mut state, sequence).add_analysis(
            AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "TRAN")
                .with_waveforms(vec![RetainedTrace::new(
                    "V(out)",
                    vec![0.0, 1e-4, 2e-4, 9e-4],
                    vec![0.0, 0.1, f64::NAN, 0.6],
                    "",
                )]),
        );

        let drawn = classify_viewer(&mut state.results_parts(), ResultViewer::Waves);
        assert_eq!(drawn.state, ResultOperationalState::Streaming);
        assert_eq!(drawn.detail.as_deref(), Some(note));
        assert!(!drawn.blocks_visuals, "the plot so far stays in view");
    }

    /// A sheet reads the analysis it shows, not the run's worst. In a run
    /// whose transient completed and whose AC analysis failed, the
    /// transient's sheet carries no state at all, so nothing marks its tab
    /// or its well; the AC sheet, with nothing of its own to draw, is the
    /// failure, which takes its well. The run's own mark says the run
    /// completed with one of its two analyses failed, in the warning tone,
    /// and reads Failed only once every analysis failed.
    #[test]
    fn a_sheet_reads_its_own_analysis_not_the_runs_worst() {
        let mut state = ResultsTestHost::default();
        state.platform.project_lifecycle.authority.open_session();
        let run = state.model.simulation.start_run();
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#00aaff"),
            ]),
        );
        let mut ac = AnalysisResult::new(2, AnalysisType::Ac, "AC");
        ac.success = false;
        ac.error_message = Some("singular matrix at node n3".to_owned());
        run.add_analysis(ac);
        run.mark_running().expect("the run starts");
        run.finish_lifecycle(SimulationRunLifecycle::Failed)
            .expect("the run ends");

        state.model.simulation.view.active_analysis_idx = Some(0);
        let tran = classify_viewer(&mut state.results_parts(), ResultViewer::Waves);
        assert_eq!(tran.state, ResultOperationalState::Complete);
        assert!(notable_status(&mut state.results_parts(), ResultViewer::Waves).is_none());

        state.model.simulation.view.active_analysis_idx = Some(1);
        let ac = classify_viewer(&mut state.results_parts(), ResultViewer::Waves);
        assert_eq!(ac.state, ResultOperationalState::Failed);
        assert!(ac.blocks_visuals, "the AC sheet has nothing to draw");
        assert_eq!(ac.detail.as_deref(), Some("AC: singular matrix at node n3"));
        // Its own failure: the reason, then what to do. Nothing earlier is
        // on show, so nothing is said to be kept.
        assert_eq!(
            rspice_results_ui::operational::sentence(&ac),
            "Failed · AC: singular matrix at node n3 · Fix the cause and run the plan again."
        );

        let mark = RunMark::of(&state.results_parts(), 0, ResultOperationalState::Failed);
        let run = &state.model.simulation.retained.runs[0];
        assert_eq!(mark.status.label(), "1 of 2 failed");
        assert_eq!(mark.status.category(), ResultOperationalCategory::Warning);
        assert_eq!(run_state_words(run), "Completed, one analysis failed");
        assert_eq!(
            mark.in_run_details_words(run).status.label(),
            "Completed, one analysis failed"
        );

        state.model.simulation.retained.runs[0].analyses[0].success = false;
        let mark = RunMark::of(&state.results_parts(), 0, ResultOperationalState::Failed);
        let run = &state.model.simulation.retained.runs[0];
        assert_eq!(run_state_words(run), "Failed");
        assert_eq!(mark.status.label(), "Failed");
        assert_eq!(mark.status.category(), ResultOperationalCategory::Error);
    }

    /// A run its results storage stopped says where and why in its State
    /// row, not that it completed: the volume filled, the plan's budget was
    /// reached, or the store kept none of it.
    #[test]
    fn a_run_its_results_storage_stopped_says_so_in_its_state() {
        let mut state = ResultsTestHost::default();
        let run = state.model.simulation.start_run();
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                RetainedTrace::new("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.0], "#00aaff"),
            ]),
        );
        run.mark_running().expect("the run starts");
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("the run ends");
        let mut stop = crate::state::ResultsStorageStop {
            analysis: "TRAN".to_owned(),
            at: Some(1.0e-6),
            rows: 2,
            bytes: 64,
            free: Some(0),
            folder: "store".to_owned(),
            budget: None,
        };
        let run = &mut state.model.simulation.retained.runs[0];
        assert_eq!(run_state_words(run), "Completed");
        run.stop_on_results_storage(stop.clone());
        assert_eq!(run_state_words(run), "Stopped at 1µs: results storage full");
        stop.budget = Some(crate::state::ResultsStorageBudget {
            budget: 64,
            written: 64,
            point: None,
        });
        run.stop_on_results_storage(stop.clone());
        assert_eq!(
            run_state_words(run),
            "Stopped at 1µs: storage budget reached"
        );
        stop.budget = None;
        stop.rows = 0;
        run.stop_on_results_storage(stop);
        assert_eq!(run_state_words(run), "Results not kept: storage full");
    }
}

// ---------------------------------------------------------------------------
// presentation
// ---------------------------------------------------------------------------

/// Present one canonical Results operational state without replacing valid
/// retained evidence. Blocking states own the well; warnings and recovery
/// notices spend none of it, marked on the Plot page's band and the data
/// browser's run row instead.
pub(super) fn show_result_operational_status(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    status: &ResultOperationalStatus,
) -> bool {
    if states_its_own_failure(state.results_reads(), status) {
        return false;
    }
    let mut offered = None;
    let run_plan = state.run_plan;
    let response = operational::show(ui, status, || {
        offered = offered_failure_sites(state);
        CardOffers {
            failure_site: offered.as_ref().map(OfferedFailureSites::control),
            run_again: run_again_control(run_plan),
        }
    });
    apply_operational_response(state, &response, offered.as_ref());
    response.blocks_visuals
}

/// The state the quick view's sheet stays readable under, when it is one a
/// mark beside the viewer carries.
fn status_beside_viewer(state: &mut ResultsParts<'_>) -> Option<ResultOperationalStatus> {
    let viewer = state.results.session.viewer;
    let status = classify_viewer(state, viewer);
    (operational::marks_beside_viewer(&status)
        && !states_its_own_failure(state.results_reads(), &status))
    .then_some(status)
}

/// The state the shown result is in on `viewer` when it is one a reader
/// should notice: anything but Complete or an empty well, unless the sheet
/// states its own failure on its first row.
pub(crate) fn notable_status(
    state: &mut ResultsParts<'_>,
    viewer: ResultViewer,
) -> Option<ResultOperationalStatus> {
    let status = classify_viewer(state, viewer);
    (status.state != ResultOperationalState::Complete
        && status.state.category() != operational::ResultOperationalCategory::Empty
        && !states_its_own_failure(state.results_reads(), &status))
    .then_some(status)
}

/// Draw the quick view's state as a mark on the Plot page's band, and carry
/// out what its popup asked. Returns whether there was a state to mark.
pub(super) fn show_status_mark(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    let Some(status) = status_beside_viewer(state) else {
        return false;
    };
    let mut offered = None;
    let run_plan = state.run_plan;
    let response = operational::show_mark(ui, &status, crate::ui::tokens::FS_1, || {
        offered = offered_failure_sites(state);
        CardOffers {
            failure_site: offered.as_ref().map(OfferedFailureSites::control),
            run_again: run_again_control(run_plan),
        }
    });
    apply_operational_response(state, &response, offered.as_ref());
    true
}

/// The mark a run carries in the data browser's row and the Details' State
/// row: what became of the run, in the state's own words (Out of date,
/// Running, Failed), or for a run that completed with some of its analyses
/// failed, how many failed. The shown run's mark is the one its sheet reads
/// when that is the run's reading, detail and all, and its popup can act on
/// it: mark the failure's sites, dismiss a notice, or run the plan again.
pub(crate) struct RunMark {
    status: ResultOperationalStatus,
    /// The failure's sites the shown run's popup offers to mark.
    offered: Option<OfferedFailureSites>,
    /// What Run plan answered, for the popup's Run again.
    run_plan: CommandAvailability,
    shown: bool,
}

impl RunMark {
    /// The mark of the retained run at `run_index`, whose data reads as
    /// `reads_as`.
    pub(crate) fn of(
        state: &ResultsParts<'_>,
        run_index: usize,
        reads_as: ResultOperationalState,
    ) -> Self {
        let run = state.model.simulation.retained.runs.get(run_index);
        let status = match run {
            Some(run) if reads_as == ResultOperationalState::Failed => run_failure_status(run),
            _ => ResultOperationalStatus::canonical(reads_as, false),
        };
        Self {
            status,
            offered: None,
            run_plan: state.run_plan,
            shown: false,
        }
    }

    /// The shown run's mark. A failed run's is the run's own reading, which
    /// says how many of its analyses failed whatever the sheet shows; any
    /// other's is the state its sheet reads, when that is the reading its
    /// data has, else the reading alone.
    pub(crate) fn shown(state: &mut ResultsParts<'_>, reads_as: ResultOperationalState) -> Self {
        let Some(run_index) = state.model.simulation.view.active_run_idx else {
            return Self {
                status: ResultOperationalStatus::canonical(reads_as, false),
                offered: None,
                run_plan: state.run_plan,
                shown: false,
            };
        };
        if reads_as == ResultOperationalState::Failed {
            return Self {
                offered: offered_failure_sites(state),
                shown: true,
                ..Self::of(state, run_index, reads_as)
            };
        }
        let status = classify_viewer(state, state.results.session.viewer);
        if !same_reading(status.state, reads_as) {
            return Self::of(state, run_index, reads_as);
        }
        Self {
            offered: offered_failure_sites(state),
            status,
            run_plan: state.run_plan,
            shown: true,
        }
    }

    /// The mark as the Details' State row reads it: a run that completed
    /// with some of its analyses failed, in the words Run details uses.
    pub(crate) fn in_run_details_words(mut self, run: &SimulationRun) -> Self {
        if let Some(wording) = self.status.wording.as_mut() {
            wording.label = run_state_words(run);
        }
        self
    }

    pub(crate) fn width(&self, ui: &Ui) -> f32 {
        operational::mark_width(ui, &self.status, RUN_MARK_SIZE)
    }

    /// Draw the mark in `slot`, at the slot's height, and say what its popup
    /// asked.
    pub(crate) fn show_in(
        &self,
        ui: &mut Ui,
        slot: egui::Rect,
        id: impl std::hash::Hash + std::fmt::Debug,
    ) -> operational::OperationalResponse {
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .id_salt(("run-mark", id))
                .max_rect(slot)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        child.spacing_mut().interact_size.y = slot.height();
        operational::show_mark(&mut child, &self.status, RUN_MARK_SIZE, || CardOffers {
            failure_site: self.offered.as_ref().map(OfferedFailureSites::control),
            run_again: run_again_control(self.run_plan),
        })
    }

    /// Carry out what the popup asked: Run again from any run's, and what
    /// acts on the shown result from the shown run's.
    pub(crate) fn apply(
        &self,
        state: &mut ResultsParts<'_>,
        response: &operational::OperationalResponse,
    ) {
        if self.shown {
            apply_operational_response(state, response, self.offered.as_ref());
        } else if response.run_again {
            run_again(state);
        }
    }
}

/// The card's Run again as Run plan answered: offered while Run plan is
/// offered at all, and unavailable for its reason.
fn run_again_control(run_plan: CommandAvailability) -> Option<RunAgainControl> {
    match run_plan {
        CommandAvailability::Available => Some(RunAgainControl { unavailable: None }),
        CommandAvailability::Disabled(reason) => Some(RunAgainControl {
            unavailable: Some(reason),
        }),
        CommandAvailability::Hidden => None,
    }
}

/// Run the plan again through Run plan's own command, once the frame's
/// contribution returns.
fn run_again(state: &mut ResultsParts<'_>) {
    state.workbench.outbox.push(Execute(ids::RUN_SIMULATION));
}

/// The face a run's mark names its state in: the run's own row size.
const RUN_MARK_SIZE: f32 = crate::ui::tokens::FS_0;

/// Whether the state a sheet reads and the state a run's data reads as are
/// one reading: the same state, or a run still in progress either way.
fn same_reading(sheet: ResultOperationalState, data: ResultOperationalState) -> bool {
    use ResultOperationalState::{Loading, Streaming};
    sheet == data || matches!((sheet, data), (Loading | Streaming, Loading | Streaming))
}

/// A failed solve on its own operating-point sheet: the sheet says so on
/// its first row, so neither a banner nor a mark repeats it.
fn states_its_own_failure(state: ResultsReads<'_>, status: &ResultOperationalStatus) -> bool {
    status.state == ResultOperationalState::Failed
        && !status.blocks_visuals
        && op_sheet_states_the_failure(state, state.results.session.viewer)
}

fn apply_operational_response(
    state: &mut ResultsParts<'_>,
    response: &operational::OperationalResponse,
    offered: Option<&OfferedFailureSites>,
) {
    if response.dismiss {
        state.results.dismiss_runtime_condition();
    }
    if response.highlight {
        toggle_failure_site_marking(state, offered);
    }
    if response.run_again {
        run_again(state);
    }
}

/// Whether `viewer` is the operating-point sheet of the solve that failed.
/// The sheet says so on its own first row, so a banner above it would be the
/// same sentence twice.
fn op_sheet_states_the_failure(state: ResultsReads<'_>, viewer: ResultViewer) -> bool {
    viewer == ResultViewer::Op
        && state
            .model
            .simulation
            .active_analysis()
            .is_some_and(|analysis| {
                analysis.analysis_type == AnalysisType::DcOp
                    && !analysis.success
                    && !analysis.is_live_partial()
            })
}

/// The failure on display's offer to mark what it named, and whether that
/// marking is on the drawing now.
pub(super) struct OfferedFailureSites {
    offer: FailureSiteOffer,
    marked: bool,
}

impl OfferedFailureSites {
    pub(super) fn control(&self) -> operational::FailureSiteControl<'static> {
        operational::FailureSiteControl {
            named: self.offer.named,
            elided: self.offer.elided,
            headline: self.offer.headline,
            marked: self.marked,
        }
    }
}

pub(super) fn offered_failure_sites(state: &ResultsParts<'_>) -> Option<OfferedFailureSites> {
    let offer = failure_site_offer(state.results_reads())?;
    let marked = state.results.marked_failure_run == Some(offer.run_sequence);
    Some(OfferedFailureSites { offer, marked })
}

pub(super) fn toggle_failure_site_marking(
    state: &mut ResultsParts<'_>,
    offered: Option<&OfferedFailureSites>,
) {
    if offered.is_some_and(|offered| offered.marked) {
        clear_failure_site_marking(state.model);
        state.results.marked_failure_run = None;
    } else {
        state.workbench.outbox.push(MarkFailureSites(
            offered.map(|offered| offered.offer.run_sequence),
        ));
    }
}

/// Whether the analysis on display named objects worth marking.
///
/// A surface offering "highlight non-converged nodes" asks this to decide
/// whether the control belongs on screen at all. It answers about the
/// evidence, not about the schematic: whether the marking will land is
/// `AppState::highlight_active_failure_sites`'s answer, and it is a
/// different question with a different sentence.
pub fn active_failure_names_objects(model: &Model) -> bool {
    model
        .simulation
        .active_analysis()
        .and_then(|analysis| analysis.failure_attribution.as_ref())
        .is_some_and(|attribution| !attribution.sites.is_empty())
}

/// Take back a marking `AppState::highlight_active_failure_sites` made.
///
/// The exact inverse of what the marking did, and no more: the highlight
/// and the selection go, the view stays where the reader left it. Scrolling
/// them back to wherever they were before would be a second surprise
/// rather than an undo of the first.
fn clear_failure_site_marking(model: &mut Model) {
    model.schematic.session.editor.net_highlight.clear();
    model.schematic.session.editor.selection.clear();
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
fn failure_site_offer(state: ResultsReads<'_>) -> Option<FailureSiteOffer> {
    if !active_failure_names_objects(state.model) {
        return None;
    }
    let run_sequence = state.model.simulation.active_run()?.id;
    let analysis = state.model.simulation.active_analysis()?;
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

#[cfg(test)]
mod failure_site_control_tests {
    use super::*;
    use crate::state::ConvergenceAttribution;

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
        rspice_results::convergence_attribution::from_core(&core::ConvergenceDiagnostic {
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
    ) -> ResultsTestHost {
        let mut state = ResultsTestHost::default();
        let mut analysis =
            AnalysisResult::failed(1, AnalysisType::Transient, "tran", rendered_error);
        analysis.failure_attribution = attribution;
        state.model.simulation.start_run().add_analysis(analysis);
        state.model.simulation.select_run(0);
        state
    }

    /// Only the failed solve's own sheet goes without the banner: the sheet
    /// of a solve that finished, and every other page, still need it.
    #[test]
    fn the_operating_point_sheet_states_its_own_failed_solve() {
        let mut failed = ResultsTestHost::default();
        failed
            .model
            .simulation
            .start_run()
            .add_analysis(AnalysisResult::failed(1, AnalysisType::DcOp, "op", FAILURE));
        failed.model.simulation.select_run(0);
        assert!(op_sheet_states_the_failure(
            failed.results_reads(),
            ResultViewer::Op
        ));
        assert!(!op_sheet_states_the_failure(
            failed.results_reads(),
            ResultViewer::Waves
        ));

        let elsewhere = state_showing_failure(None, FAILURE);
        assert!(!op_sheet_states_the_failure(
            elsewhere.results_reads(),
            ResultViewer::Op
        ));
    }

    /// The control is offered for the failure it was recorded against.
    #[test]
    fn a_matching_attribution_offers_the_control_and_counts_what_it_marks() {
        let state = state_showing_failure(Some(attribution(FAILURE, 9)), FAILURE);

        let offer = failure_site_offer(state.results_reads())
            .expect("the attribution describes this failure");
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
            active_failure_names_objects(&state.model),
            "there is an attribution naming objects — that gate alone is not enough"
        );
        assert!(
            failure_site_offer(state.results_reads()).is_none(),
            "but it does not describe the failure on display, so nothing is offered"
        );
    }

    /// A project written before the failure message was retained cannot prove
    /// the pairing, and an unprovable pairing must not mark anything.
    #[test]
    fn an_attribution_that_cannot_prove_its_pairing_offers_nothing() {
        let state = state_showing_failure(Some(attribution("", 0)), FAILURE);

        assert!(failure_site_offer(state.results_reads()).is_none());
    }

    /// No attribution at all is the ordinary case for a failure the engine
    /// could not attribute, and it carries no control.
    #[test]
    fn a_failure_naming_no_objects_offers_nothing() {
        let state = state_showing_failure(None, FAILURE);

        assert!(!active_failure_names_objects(&state.model));
        assert!(failure_site_offer(state.results_reads()).is_none());
    }

    /// The control takes its own marking back, and nothing else.
    #[test]
    fn clearing_removes_the_marking_without_moving_the_view() {
        let mut state = state_showing_failure(Some(attribution(FAILURE, 0)), FAILURE);
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_wire(91);
        state
            .model
            .schematic
            .session
            .editor
            .net_highlight
            .highlight_wires(std::iter::once(91).collect());
        state.model.schematic.session.editor.center_request = None;

        clear_failure_site_marking(&mut state.model);

        assert!(
            state
                .model
                .schematic
                .session
                .editor
                .selection
                .wires
                .is_empty()
        );
        assert!(!state.model.schematic.session.editor.net_highlight.active);
        assert!(
            state
                .model
                .schematic
                .session
                .editor
                .center_request
                .is_none(),
            "clearing a marking must not scroll the drawing"
        );
    }
}
