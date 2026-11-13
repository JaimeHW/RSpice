//! Saved views that reference their runs: large results save, reopen,
//! compare and refresh; a deleted or changed run is named and never drawn as
//! the original; an archived view reopens with its run gone.

use std::collections::HashSet;

use rspice_results_ui::document_dialogs::ResultDocumentDialog;
use rspice_results_ui::document_dialogs::compare::CompareDraft;

use super::*;
use crate::results::visualization_document::{
    DocumentEdit, PaneDataBinding, ResultDocumentTracking, ResultDocumentTrackingMode,
};
use crate::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRunLifecycle, SimulationRunProvenance,
};
use crate::workbench::documents::result_document::test_host::ResultsTestHost;
use crate::workbench::documents::result_document::{create_document, document_edit};
use crate::workbench::state::{Workspace, WorkspaceDocumentId};

/// A finished transient run of `signals` signals of `points` samples each.
fn run(sequence: u64, signals: usize, points: usize, offset: f64) -> SimulationRun {
    let x = (0..points)
        .map(|sample| sample as f64 * 1.0e-9)
        .collect::<Vec<_>>();
    let traces = (0..signals)
        .map(|signal| {
            let y = (0..points)
                .map(|sample| ((sample % 1_000) as f64).mul_add(1.0e-3, signal as f64) + offset)
                .collect::<Vec<_>>();
            RetainedTrace::new(format!("V(n{signal})"), x.clone(), y, "#55aaff")
        })
        .collect();
    let mut run = SimulationRun::new(sequence);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(traces),
    );
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .expect("legacy provenance");
    run.restore_lifecycle(SimulationRunLifecycle::LegacyUnknown, 0.0)
        .expect("legacy lifecycle");
    run
}

fn host(runs: &[&SimulationRun]) -> ResultsTestHost {
    let mut state = ResultsTestHost::default();
    for run in runs {
        state.model.simulation.retained.runs.push((*run).clone());
    }
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state
}

/// Save the active run as a single-pane view, as Create Result Document does.
fn save_view(state: &mut ResultsTestHost) -> ResultDocumentId {
    let mut parts = state.results_parts();
    create_document::open(&mut parts);
    parts.create_result_document.layout_id =
        rspice_results::document_creation::ResultDocumentLayout::SinglePane
            .id()
            .to_owned();
    create_document::commit(&mut parts).expect("the view is saved")
}

fn document(state: &ResultsTestHost, document_id: ResultDocumentId) -> VisualizationDocument {
    state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("the project keeps the document")
        .clone()
}

fn pane_binding(document: &VisualizationDocument) -> PaneDataBinding {
    document.panes()[0].binding.expect("the pane reads a run")
}

/// Write the document out, read it back, and open its tab in Results in a
/// session holding `runs`, as choosing the document in the project does.
fn reopen(
    document: &VisualizationDocument,
    runs: &[&SimulationRun],
) -> (ResultsTestHost, ResultDocumentId, usize) {
    let json = serde_json::to_string(document).expect("the document writes out");
    let reopened: VisualizationDocument = serde_json::from_str(&json).expect("it reads back");
    let mut state = host(runs);
    let document_id = state
        .model
        .workspace
        .content
        .insert_visualization_document(reopened)
        .expect("the reopened document is kept");
    state.workbench.activate(Workspace::Results);
    assert!(
        super::super::activate(&mut state.results_parts(), document_id),
        "the document opens even when its runs are gone"
    );
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::VisualizationDocument(document_id));
    (state, document_id, json.len())
}

fn compare_with(state: &mut ResultsTestHost, baseline: &SimulationRun) {
    let mut parts = state.results_parts();
    *parts.result_document_dialog = Some(ResultDocumentDialog::Compare(CompareDraft {
        baseline: Some(baseline.dataset_id),
        ..CompareDraft::default()
    }));
    document_edit::commit_open_dialog(&mut parts);
    let ran = matches!(
        parts.result_document_dialog.as_ref(),
        Some(ResultDocumentDialog::Compare(draft)) if draft.outcome.is_some()
    );
    assert!(ran, "the comparison ran: {:?}", state.prompt_text());
}

/// Advance a document onto a newer run of its analysis, as Latest tracking
/// does when a rerun completes.
fn refresh_onto(state: &mut ResultsTestHost, document_id: ResultDocumentId, newer: &SimulationRun) {
    let document = document(state, document_id);
    let previous = pane_binding(&document);
    let next = create_document::source_reference(newer, &newer.analyses[0]).expect("reference");
    let analysis_id = create_document::source_analysis_identity(newer, &newer.analyses[0]);
    let tracking = ResultDocumentTracking::for_plan(
        ResultDocumentTrackingMode::Latest,
        crate::product::SimulationPlanId::new(),
        analysis_id,
    );
    state
        .model
        .workspace
        .content
        .transact_visualization_document(
            document_id,
            document.revision(),
            vec![
                DocumentEdit::SetTracking(tracking),
                DocumentEdit::RetargetTrackedDataset {
                    previous: previous.dataset,
                    next,
                    analysis_id,
                },
            ],
        )
        .expect("the document advances onto the newer run");
}

/// The whole round trip a saved view of a large result takes.
fn saves_reopens_compares_and_refreshes(signals: usize, points: usize) {
    let original = run(1, signals, points, 0.0);
    let baseline = run(2, signals, points, 1.0e-6);
    let newer = run(3, signals, points, 2.0e-6);
    let mut state = host(&[&original, &baseline, &newer]);

    let document_id = save_view(&mut state);
    let saved = document(&state, document_id);
    assert_eq!(saved.traces().len(), signals, "one trace per signal");
    assert_eq!(pane_binding(&saved).dataset.dataset_id, original.dataset_id);

    let (mut reopened, reopened_id, bytes) = reopen(&saved, &[&original, &baseline, &newer]);
    assert!(
        bytes < 64 * 1024,
        "{signals} x {points} samples saved as {bytes} bytes; the samples stay in the run"
    );
    assert!(unavailable_sources(reopened.results_reads(), reopened_id).is_empty());
    assert!(
        super::super::resolve_binding(
            reopened.results_reads(),
            pane_binding(&document(&reopened, reopened_id))
        )
        .is_ok(),
        "the reopened pane reads its run"
    );

    compare_with(&mut reopened, &baseline);
    let compared = document(&reopened, reopened_id);
    assert_eq!(compared.datasets().len(), 2);
    assert_eq!(compared.comparisons().len(), 1);

    refresh_onto(&mut reopened, reopened_id, &newer);
    let refreshed = document(&reopened, reopened_id);
    assert_eq!(
        pane_binding(&refreshed).dataset.dataset_id,
        newer.dataset_id
    );
    assert!(
        refreshed
            .traces()
            .iter()
            .all(|trace| trace.binding.dataset_id == newer.dataset_id)
    );
}

#[test]
fn a_view_of_twenty_signals_of_forty_thousand_points_saves_reopens_compares_and_refreshes() {
    saves_reopens_compares_and_refreshes(20, 40_000);
}

#[test]
fn a_view_of_one_signal_of_a_million_points_saves_reopens_compares_and_refreshes() {
    saves_reopens_compares_and_refreshes(1, 1_000_000);
}

#[test]
fn one_view_compares_three_runs() {
    let runs = [run(1, 3, 64, 0.0), run(2, 3, 64, 0.5), run(3, 3, 64, 1.0)];
    let mut state = host(&[&runs[0], &runs[1], &runs[2]]);
    let document_id = save_view(&mut state);
    compare_with(&mut state, &runs[1]);
    compare_with(&mut state, &runs[2]);
    let compared = document(&state, document_id);
    assert_eq!(compared.comparisons().len(), 2);
    let attached = compared
        .datasets()
        .iter()
        .map(|dataset| dataset.binding().dataset_id)
        .collect::<HashSet<_>>();
    assert_eq!(
        attached,
        runs.iter()
            .map(|run| run.dataset_id)
            .collect::<HashSet<_>>(),
        "the view holds a reference to each of the three runs"
    );
}

#[test]
fn a_deleted_run_is_named_and_the_view_repoints_to_a_run_with_its_signals() {
    let original = run(1, 4, 32, 0.0);
    let other = run(2, 4, 32, 1.0);
    let unrelated = {
        let mut unrelated = run(3, 1, 32, 0.0);
        unrelated.analyses[0].waveforms[0].name = "I(unrelated)".to_owned();
        unrelated
    };
    let mut state = host(&[&original, &other, &unrelated]);
    let document_id = save_view(&mut state);
    let saved = document(&state, document_id);
    let pages = saved.pages().len();
    let traces = saved.traces().len();

    // Reopened where its run is gone, the view still opens and says so.
    let (reopened, reopened_id, _) = reopen(&saved, &[&other, &unrelated]);
    let gone = unavailable_sources(reopened.results_reads(), reopened_id);
    assert_eq!(gone.len(), 1);
    assert_eq!(gone[0].problem, SourceProblem::Missing);

    state
        .model
        .simulation
        .retained
        .runs
        .retain(|run| run.dataset_id != original.dataset_id);
    let missing = unavailable_sources(state.results_reads(), document_id);
    assert_eq!(missing.len(), 1);
    assert_eq!(missing[0].problem, SourceProblem::Missing);
    assert_eq!(
        missing[0].reason(),
        format!(
            "{}, which this document reads, is no longer in the run history.",
            original.label
        )
    );
    assert_eq!(
        missing[0]
            .candidates
            .iter()
            .map(|(dataset_id, _)| *dataset_id)
            .collect::<Vec<_>>(),
        vec![other.dataset_id],
        "only a run with the same signals is offered"
    );
    assert!(
        super::super::resolve_binding(state.results_reads(), pane_binding(&saved)).is_err(),
        "nothing else is drawn in the missing run's place"
    );

    repoint(
        &mut state.results_parts(),
        document_id,
        missing[0].binding,
        other.dataset_id,
    )
    .expect("the view reads the other run");
    let repointed = document(&state, document_id);
    assert_eq!(
        (repointed.pages().len(), repointed.traces().len()),
        (pages, traces)
    );
    assert_eq!(
        pane_binding(&repointed).dataset.dataset_id,
        other.dataset_id
    );
    assert!(unavailable_sources(state.results_reads(), document_id).is_empty());
    assert!(super::super::resolve_binding(state.results_reads(), pane_binding(&repointed)).is_ok());
}

#[test]
fn a_run_whose_results_changed_is_never_shown_as_the_original() {
    let original = run(1, 2, 32, 0.0);
    let mut state = host(&[&original]);
    let document_id = save_view(&mut state);
    let saved = document(&state, document_id);

    let changed = run(1, 2, 32, 0.25);
    let retained = &mut state.model.simulation.retained.runs[0];
    retained.analyses = changed.analyses.clone();
    let problems = unavailable_sources(state.results_reads(), document_id);
    assert_eq!(problems.len(), 1);
    assert_eq!(problems[0].problem, SourceProblem::Changed);
    assert!(problems[0].reason().contains("no longer holds the results"));
    assert!(
        problems[0].candidates.is_empty(),
        "the changed run is not offered as itself"
    );
    assert!(
        super::super::resolve_binding(state.results_reads(), pane_binding(&saved))
            .unwrap_err()
            .contains("no longer match"),
    );
    assert!(matches!(
        archive_availability(state.results_reads(), document_id),
        ArchiveAvailability::Unavailable(_)
    ));
}

#[test]
fn an_archived_view_reopens_with_its_run_deleted() {
    let original = run(1, 3, 256, 0.0);
    let mut state = host(&[&original]);
    let document_id = save_view(&mut state);
    assert_eq!(
        archive_availability(state.results_reads(), document_id),
        ArchiveAvailability::Available
    );
    archive_document(&mut state.results_parts(), document_id).expect("the view is archived");
    assert_eq!(
        archive_availability(state.results_reads(), document_id),
        ArchiveAvailability::Archived
    );
    let archived = document(&state, document_id);
    assert!(archived.is_archived());

    // Opening the document in a session without the run brings it back.
    let (mut reopened, reopened_id, _) = reopen(&archived, &[]);
    let restored = reopened
        .model
        .simulation
        .retained
        .runs
        .iter()
        .find(|run| run.dataset_id == original.dataset_id)
        .expect("the archived run is back in the history");
    assert_eq!(
        restored.dataset_content_digest(),
        original.dataset_content_digest()
    );
    assert!(unavailable_sources(reopened.results_reads(), reopened_id).is_empty());
    assert!(
        super::super::resolve_binding(
            reopened.results_reads(),
            pane_binding(&document(&reopened, reopened_id))
        )
        .is_ok()
    );
    // Restoring again with the run back brings no second copy.
    restore_archived_runs(&mut reopened.results_parts(), reopened_id);
    assert_eq!(reopened.model.simulation.retained.runs.len(), 1);
}

#[test]
fn an_archive_that_is_not_the_bound_run_is_refused() {
    let original = run(1, 2, 32, 0.0);
    let mut state = host(&[&original]);
    let document_id = save_view(&mut state);
    archive_document(&mut state.results_parts(), document_id).expect("archived");
    let mut archived = serde_json::to_value(document(&state, document_id)).unwrap();
    // Swap the archived run for another one under the same binding.
    let impostor = run(1, 2, 32, 0.5);
    let record = serde_json::to_string(&ProjectSimulationRun::from(&{
        let mut impostor = impostor;
        impostor.dataset_id = original.dataset_id;
        impostor
    }))
    .unwrap();
    archived["datasets"][0]["run"]["archive"]["record"] = serde_json::Value::String(record);
    let archived: VisualizationDocument = serde_json::from_value(archived).unwrap();

    let (mut reopened, reopened_id, _) = reopen(&archived, &[]);
    restore_archived_runs(&mut reopened.results_parts(), reopened_id);
    assert!(reopened.model.simulation.retained.runs.is_empty());
    let binding = pane_binding(&archived).dataset;
    assert!(
        reopened
            .results_reads()
            .results
            .archive_restore_failure(binding)
            .is_some_and(|reason| reason.contains("not the run it was made from"))
    );
}
