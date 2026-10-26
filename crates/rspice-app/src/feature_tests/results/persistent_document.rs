//! Latest tracking of a project-owned Results document, against the
//! project's authored source and the runs it authorizes.

use crate::product::{
    AnalysisInstanceId, ContentDigest, ObjectRevision, ResultDocumentId, SimulationPlanId,
};
use crate::state::{AnalysisResult, AnalysisResultProvenance, AnalysisType, SimulationRun};
use crate::workbench::RSpiceApp;
use crate::workbench::documents::result_document::create_document;
use crate::workbench::documents::result_document::persistent_document::tests::{
    completed_prepared_run, digest, drive_frame,
};
use crate::workbench::documents::result_document::persistent_document::{
    LatestBinding, current_result_source_digest, refresh_latest_binding, show,
};
use rspice_results_ui::create_document::CreateResultDocumentDialogState;

/// A document that follows the newest run of one authored analysis, plus
/// the identities a second run needs to be a candidate for it.
struct LatestFixture {
    app: RSpiceApp,
    document_id: ResultDocumentId,
    plan_id: SimulationPlanId,
    authored_analysis_id: AnalysisInstanceId,
    source_digest: ContentDigest,
}

fn authored_run(
    run_sequence: u64,
    plan_id: SimulationPlanId,
    project_revision: ObjectRevision,
    source_digest: ContentDigest,
    authored_analysis_id: AnalysisInstanceId,
    trace_name: &str,
) -> SimulationRun {
    let mut run = completed_prepared_run(plan_id, project_revision, source_digest);
    run.id = run_sequence;
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_waveforms(vec![crate::state::RetainedTrace::new(
                trace_name,
                vec![0.0, 0.5, 1.0],
                vec![0.0, 1.0, 0.0],
                "#fff",
            )])
            .with_provenance(
                AnalysisResultProvenance::new(
                    authored_analysis_id,
                    ObjectRevision::INITIAL,
                    digest(0x51),
                    Vec::new(),
                )
                .expect("analysis provenance"),
            ),
    );
    run
}

fn latest_tracking_fixture() -> LatestFixture {
    use crate::results::visualization_document::{
        DocumentEdit, ResultDocumentTracking, ResultDocumentTrackingMode,
    };

    let mut app = RSpiceApp::test_instance();
    // Latest tracking only follows runs the current authored source
    // authorizes, so the fixture has to be that source.
    app.state.model.simulation.source.netlist_content =
        "* latest tracking\nV1 out 0 1\n.end\n".to_owned();
    let input_digest = digest(0x60);
    app.state.model.netlist_artifacts.generation_error = None;
    app.state.model.netlist_artifacts.generated_input_digest = Some(input_digest);
    app.state
        .model
        .netlist_artifacts
        .current_generation_input_digest = Some(input_digest);
    let source_digest = current_result_source_digest(&app.state)
        .expect("the fixture states a current authored source");

    let plan_id = SimulationPlanId::new();
    let authored_analysis_id = AnalysisInstanceId::new();
    let project_revision = app.state.model.workspace.content.project.revision();
    let run = authored_run(
        1,
        plan_id,
        project_revision,
        source_digest,
        authored_analysis_id,
        "V(out)",
    );
    let dataset_id = run.dataset_id;
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run(0));
    assert!(app.state.model.simulation.select_analysis(0));
    app.state.results.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name: "Latest transient review".to_owned(),
        name_touched: true,
        dataset_id: Some(dataset_id),
        family_id: "waveform-worksheet".to_owned(),
        viewer_id: "viewer-waveform".to_owned(),
        layout_id: "single-pane".to_owned(),
        validation_error: None,
    };
    let document_id = create_document::commit(&mut app).expect("persistent document commits");
    let revision = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("retained document")
        .revision();
    app.state
        .model
        .workspace
        .content
        .transact_visualization_document(
            document_id,
            revision,
            vec![DocumentEdit::SetTracking(ResultDocumentTracking::for_plan(
                ResultDocumentTrackingMode::Latest,
                plan_id,
                authored_analysis_id,
            ))],
        )
        .expect("the document tracks the latest run");

    LatestFixture {
        app,
        document_id,
        plan_id,
        authored_analysis_id,
        source_digest,
    }
}

/// A Latest document whose newest run renamed the net it plots keeps
/// drawing the dataset it last resolved, under a banner naming the cause
/// and the control that settles it. Blanking the surface withheld the
/// evidence the reader already had.
#[test]
fn a_latest_document_that_cannot_retarget_keeps_its_last_good_binding() {
    let LatestFixture {
        mut app,
        document_id,
        plan_id,
        authored_analysis_id,
        source_digest,
    } = latest_tracking_fixture();
    let good_dataset = app.state.model.simulation.retained.runs[0].dataset_id;
    let project_revision = app.state.model.workspace.content.project.revision();
    // The newest run of the same authored analysis no longer carries the
    // signal this document's traces name.
    app.state.model.simulation.retained.runs.push(authored_run(
        2,
        plan_id,
        project_revision,
        source_digest,
        authored_analysis_id,
        "V(renamed)",
    ));
    let stale_dataset = app.state.model.simulation.retained.runs[1].dataset_id;
    assert_ne!(good_dataset, stale_dataset);

    let LatestBinding::Degraded(reason) = refresh_latest_binding(&mut app.state, document_id)
    else {
        panic!("a document that cannot retarget must degrade, not advance or blank");
    };
    assert!(
        reason.contains("Latest / Pinned"),
        "the banner has to name the control that settles this: {reason}"
    );
    assert!(
        reason.contains("could not be retargeted onto the newest run")
            || reason.contains("no longer builds its retained source"),
        "the banner has to name the cause, not just the remedy: {reason}"
    );

    // The document still draws: the pane resolves, against the binding it
    // last had.
    drive_frame(|ui| show(ui, &mut app, document_id));
    let context = app
        .state
        .results
        .results
        .session
        .persistent_pane_context
        .expect("the document still projects a pane");
    assert_eq!(context.document_id, document_id);
    assert_eq!(
        app.state
            .model
            .simulation
            .active_run()
            .expect("an active run")
            .dataset_id,
        good_dataset,
        "the last binding that resolved is the one still on screen"
    );
}

/// A refused retarget rebuilds the retained source dataset. Re-trying it
/// every frame would pay for that rebuild every frame, so the refusal is
/// held against the candidate that produced it.
#[test]
fn a_refused_latest_retarget_is_not_re_attempted_every_frame() {
    let LatestFixture {
        mut app,
        document_id,
        plan_id,
        authored_analysis_id,
        source_digest,
    } = latest_tracking_fixture();
    let project_revision = app.state.model.workspace.content.project.revision();
    app.state.model.simulation.retained.runs.push(authored_run(
        2,
        plan_id,
        project_revision,
        source_digest,
        authored_analysis_id,
        "V(renamed)",
    ));
    let stale_dataset = app.state.model.simulation.retained.runs[1].dataset_id;

    assert!(matches!(
        refresh_latest_binding(&mut app.state, document_id),
        LatestBinding::Degraded(_)
    ));
    let first = app
        .state
        .results
        .results
        .latest_retarget_failure(document_id, stale_dataset)
        .expect("the refusal is held against the candidate that caused it")
        .to_owned();
    let revision_after_first = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("retained document")
        .revision();

    for _ in 0..3 {
        assert!(matches!(
            refresh_latest_binding(&mut app.state, document_id),
            LatestBinding::Degraded(_)
        ));
    }

    assert_eq!(
        app.state
            .results
            .results
            .latest_retarget_failure(document_id, stale_dataset),
        Some(first.as_str())
    );
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .expect("retained document")
            .revision(),
        revision_after_first,
        "a held refusal must not keep re-transacting against the document"
    );

    // A genuinely new candidate is tried again rather than inheriting the
    // previous refusal.
    assert_eq!(
        app.state
            .results
            .results
            .latest_retarget_failure(document_id, crate::product::DatasetId::new()),
        None
    );
}
