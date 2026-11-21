//! Latest tracking of a project-owned Results document, against the
//! project's authored source and the runs it authorizes.

use crate::workbench::AppState;
use crate::workbench::RSpiceApp;
use egui::Ui;
use rspice_app_results::workbench::documents::result_document::ResultViewer;
use rspice_app_results::workbench::documents::result_document::create_document;
use rspice_app_results::workbench::documents::result_document::persistent_document::*;
use rspice_app_results::workbench::documents::result_document::persistent_document::{
    LatestBinding, current_result_source_digest, refresh_latest_binding, show,
};
use rspice_app_results::workbench::documents::result_document::test_fixtures::{
    completed_prepared_run, digest,
};
use rspice_app_state::product::{
    AnalysisInstanceId, ContentDigest, ObjectRevision, ResultDocumentId, SimulationPlanId,
};
use rspice_app_state::results::visualization_document::AxisOrientation;
use rspice_app_state::results::visualization_document::AxisRange;
use rspice_app_state::results::visualization_document::DocumentEdit;
use rspice_app_state::results::visualization_document::Marker;
use rspice_app_state::results::visualization_document::PaneId;
use rspice_app_state::results::visualization_document::TypedValue;
use rspice_app_state::state::SimulationRunLifecycle;
use rspice_app_state::state::{
    AnalysisResult, AnalysisResultProvenance, AnalysisType, SimulationRun,
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
    authored_run_with(
        run_sequence,
        plan_id,
        project_revision,
        source_digest,
        authored_analysis_id,
        trace_name,
        (vec![0.0, 0.5, 1.0], vec![0.0, 1.0, 0.0]),
    )
}

/// [`authored_run`], with the one trace's samples given.
fn authored_run_with(
    run_sequence: u64,
    plan_id: SimulationPlanId,
    project_revision: ObjectRevision,
    source_digest: ContentDigest,
    authored_analysis_id: AnalysisInstanceId,
    trace_name: &str,
    (x, y): (Vec<f64>, Vec<f64>),
) -> SimulationRun {
    let mut run = completed_prepared_run(plan_id, project_revision, source_digest);
    run.id = run_sequence;
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_waveforms(vec![rspice_app_state::state::RetainedTrace::new(
                trace_name, x, y, "#fff",
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
    use rspice_app_state::results::visualization_document::{
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
    let source_digest = current_result_source_digest(app.state.results_reads())
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
    let document_id = create_document::commit(&mut app.state.results_parts())
        .expect("persistent document commits");
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

/// A signal name no run reference can catalogue: the newest run carrying it
/// cannot build the retained source a document follows.
const UNCATALOGUED_SIGNAL: &str = "V(out)\u{7}";

/// A Latest document whose newest run cannot build its retained source
/// keeps drawing the dataset it last resolved, under a banner naming the
/// cause and the control that settles it. Blanking the surface withheld the
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
    // The newest run of the same authored analysis names its signal in a
    // way no run reference can catalogue.
    app.state.model.simulation.retained.runs.push(authored_run(
        2,
        plan_id,
        project_revision,
        source_digest,
        authored_analysis_id,
        UNCATALOGUED_SIGNAL,
    ));
    let stale_dataset = app.state.model.simulation.retained.runs[1].dataset_id;
    assert_ne!(good_dataset, stale_dataset);

    let LatestBinding::Degraded(reason) =
        refresh_latest_binding(&mut app.state.results_parts(), document_id)
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
    drive_frame(|ui| {
        super::with_surface_parts(&mut app, |state, run_controller| {
            show(ui, state, run_controller, document_id)
        })
    });
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

/// A Latest document whose newest run renamed the net it plots moves onto
/// that run all the same: the trace keeps its place, marked missing, and
/// nothing is refused.
#[test]
fn a_latest_document_moves_onto_a_run_that_lacks_its_signal() {
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
    let newest = app.state.model.simulation.retained.runs[1].dataset_id;

    assert!(matches!(
        refresh_latest_binding(&mut app.state.results_parts(), document_id),
        LatestBinding::Current
    ));
    let document = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("retained document");
    assert!(
        document
            .panes()
            .iter()
            .filter_map(|pane| pane.binding)
            .all(|binding| binding.dataset.dataset_id == newest),
        "the view moved onto the newest run"
    );
    assert!(!document.traces().is_empty());
    assert!(
        document.traces().iter().all(|trace| trace.missing),
        "the trace the run lacks stays, marked missing"
    );
    assert!(
        app.state
            .results
            .results
            .latest_retarget_failure(document_id, newest)
            .is_none(),
        "nothing was refused"
    );
}

/// A step taken before the view followed a re-run still undoes, on the
/// new run: undo addresses the view, not the run it read.
#[test]
fn undo_survives_the_view_following_a_rerun() {
    use rspice_app_state::results::visualization_document::DocumentEdit;
    use rspice_app_workbench::workbench::framework::command_registry::ids;
    use rspice_app_workbench::workbench::state::{Workspace, WorkspaceDocumentId};

    let LatestFixture {
        mut app,
        document_id,
        plan_id,
        authored_analysis_id,
        source_digest,
    } = latest_tracking_fixture();
    app.state.workbench.activate(Workspace::Results);
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::VisualizationDocument(document_id));
    let trace_id = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("retained document")
        .traces()[0]
        .id;
    rspice_app_results::workbench::documents::result_document::document_edit::transact(
        &mut app.state.results_parts(),
        document_id,
        vec![DocumentEdit::SetTraceVisibility {
            trace_id,
            visible: false,
        }],
    )
    .expect("the reader hides the trace");
    let project_revision = app.state.model.workspace.content.project.revision();
    app.state.model.simulation.retained.runs.push(authored_run(
        2,
        plan_id,
        project_revision,
        source_digest,
        authored_analysis_id,
        "V(out)",
    ));
    let rerun = app.state.model.simulation.retained.runs[1].dataset_id;
    assert!(matches!(
        refresh_latest_binding(&mut app.state.results_parts(), document_id),
        LatestBinding::Current
    ));

    app.execute_command_key(ids::UNDO);
    let document = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("retained document");
    let trace = &document.traces()[0];
    assert!(trace.visible, "the hidden trace shows again");
    assert_eq!(trace.binding.dataset_id, rerun, "on the re-run");
    assert!(
        app.state.workbench.prompt.is_none(),
        "nothing was dropped: {:?}",
        app.state
            .workbench
            .prompt
            .as_ref()
            .map(|prompt| &prompt.text)
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
        UNCATALOGUED_SIGNAL,
    ));
    let stale_dataset = app.state.model.simulation.retained.runs[1].dataset_id;

    assert!(matches!(
        refresh_latest_binding(&mut app.state.results_parts(), document_id),
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
            refresh_latest_binding(&mut app.state.results_parts(), document_id),
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
            .latest_retarget_failure(document_id, rspice_app_state::product::DatasetId::new()),
        None
    );
}

/// Save the document and load it back, as a project round trip does.
fn reload(app: &mut RSpiceApp, document_id: ResultDocumentId) {
    let documents = &mut app.state.model.workspace.content.visualization_documents;
    let index = documents
        .iter()
        .position(|document| document.id() == document_id)
        .expect("the project retains the document");
    let saved = serde_json::to_string(&documents[index]).expect("the document saves");
    documents[index] = serde_json::from_str(&saved).expect("the document loads");
}

/// Draw `pane` as a frame does, from the snapshot it was projected in, with
/// `edit` standing in for what the reader does in its viewer.
fn draw_pane(app: &mut RSpiceApp, pane: &PaneProjection, edit: impl FnOnce(&mut AppState)) {
    select_pane_binding(&mut app.state.results_parts(), pane).expect("pane binding");
    project_pane_presentation(&mut app.state.results_parts(), pane, ResultViewer::Waves)
        .expect("presentation projects");
    edit(&mut app.state);
    capture_pane_presentation(&mut app.state.results_parts(), pane, ResultViewer::Waves);
}

fn saved_x_range(
    app: &RSpiceApp,
    document_id: ResultDocumentId,
    pane_id: PaneId,
) -> Option<AxisRange> {
    app.state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("the project retains the document")
        .axes()
        .iter()
        .find(|axis| axis.pane_id == pane_id && axis.orientation == AxisOrientation::Horizontal)
        .and_then(|axis| axis.range)
}

fn saved_cursor_a(app: &RSpiceApp, document_id: ResultDocumentId, pane_id: PaneId) -> Option<f64> {
    app.state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("the project retains the document")
        .cursors()
        .iter()
        .find(|cursor| cursor.pane_id == pane_id && cursor.label == "A")
        .and_then(|cursor| match cursor.position {
            TypedValue::Real(position) => Some(position),
            _ => None,
        })
}

fn persistent_transient_fixture() -> (RSpiceApp, ResultDocumentId) {
    transient_fixture_with_layout("single-pane")
}

/// Project the fixture's one pane, exactly as a drawn frame does, and
/// hand back the pane so the test can keep transacting against it.
fn projected_pane(state: &mut AppState, document_id: ResultDocumentId) -> PaneProjection {
    let mut projected =
        projection(state.results_reads(), document_id).expect("document projection");
    let mut page = projected.pages.remove(0);
    let pane = page.panes.remove(0);
    select_pane_binding(&mut state.results_parts(), &pane).expect("pane binding");
    project_pane_presentation(&mut state.results_parts(), &pane, ResultViewer::Waves)
        .expect("presentation projects");
    pane
}

fn active_analysis_anchor(
    state: &mut AppState,
) -> (
    rspice_app_results::workbench::documents::result_document::AnalysisPresentationKey,
    rspice_app_results::workbench::documents::result_document::WaveformPresentationKey,
) {
    rspice_app_results::workbench::documents::result_document::waves::source_waveform_anchor(
        &mut state.results_parts(),
        "V(out)",
    )
    .expect("the fixture retains V(out)")
}

fn placement(
    analysis: rspice_app_results::workbench::documents::result_document::AnalysisPresentationKey,
    anchor: &rspice_app_results::workbench::documents::result_document::WaveformPresentationKey,
    trace_name: &str,
    x: f64,
) -> rspice_app_results::workbench::documents::result_document::MarkerPlacement<'static> {
    rspice_app_results::workbench::documents::result_document::MarkerPlacement {
        analysis,
        anchor: anchor.clone(),
        trace_name: trace_name.to_owned(),
        x,
        samples: Box::leak(Box::new(FIXTURE_SAMPLES.to_vec().into())),
    }
}

fn transient_fixture_with_layout(layout_id: &str) -> (RSpiceApp, ResultDocumentId) {
    let mut app = RSpiceApp::test_instance();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            rspice_app_state::state::RetainedTrace::new(
                "V(out)",
                vec![0.0, 0.5, 1.0],
                vec![0.0, 1.0, 0.0],
                "#fff",
            ),
        ]),
    );
    let dataset_id = run.dataset_id;
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run(0));
    assert!(app.state.model.simulation.select_analysis(0));
    app.state.results.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name: "Durable transient review".to_owned(),
        name_touched: true,
        dataset_id: Some(dataset_id),
        family_id: "waveform-worksheet".to_owned(),
        viewer_id: "viewer-waveform".to_owned(),
        layout_id: layout_id.to_owned(),
        validation_error: None,
    };
    let document_id =
        rspice_app_results::workbench::documents::result_document::create_document::commit(
            &mut app.state.results_parts(),
        )
        .expect("persistent document commits");
    (app, document_id)
}

fn retained_markers(app: &RSpiceApp, document_id: ResultDocumentId) -> Vec<Marker> {
    app.state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("retained document")
        .markers()
        .to_vec()
}

/// Drive one whole frame of a surface, with the product theme applied so
/// token lookups and font metrics resolve exactly as they do on screen.
pub(crate) fn drive_frame(body: impl FnMut(&mut Ui)) {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut body = body;
    let _ = ctx.run_ui(egui::RawInput::default(), |ui| body(ui));
}

/// Re-selecting the binding a persistent pane already holds must be inert.
///
/// `select_run` resynchronizes the displayed waveform set and advances the
/// simulation data version; every version change retires cursors, the
/// selected trace, the active pane, pinned readouts and the renderer
/// caches. This path runs on every frame the document draws, so an
/// unguarded re-selection made all of those states impossible to hold.
#[test]
fn idle_frames_of_an_open_persistent_document_never_advance_the_data_version() {
    let (mut app, document_id) = persistent_transient_fixture();
    drive_frame(|ui| {
        super::with_surface_parts(&mut app, |state, run_controller| {
            show(ui, state, run_controller, document_id)
        })
    });
    let settled = app.state.model.simulation.view.data_version;

    for frame in 0..4 {
        drive_frame(|ui| {
            super::with_surface_parts(&mut app, |state, run_controller| {
                show(ui, state, run_controller, document_id)
            })
        });
        assert_eq!(
            app.state.model.simulation.view.data_version, settled,
            "idle frame {frame} of an open persistent document advanced the data version"
        );
    }
}

/// The state an advancing data version retires has to survive an idle
/// frame, or a reader can never hold a cursor, a trace selection or a
/// pinned readout inside a project-owned document at all.
#[test]
fn idle_frames_of_an_open_persistent_document_hold_cursors_and_trace_selection() {
    let (mut app, document_id) = persistent_transient_fixture();
    drive_frame(|ui| {
        super::with_surface_parts(&mut app, |state, run_controller| {
            show(ui, state, run_controller, document_id)
        })
    });
    let analysis =
        rspice_app_results::workbench::documents::result_document::AnalysisPresentationKey::new(
            app.state.model.simulation.retained.runs[0].dataset_id,
            &app.state.model.simulation.retained.runs[0].analyses[0],
        );
    app.state.results.results.session.selected_trace =
        Some(rspice_results_ui::selection::SelectedResultTrace::from_identity(analysis, "V(out)"));
    app.state
        .results
        .results
        .session
        .rf_pin
        .insert(ResultViewer::Smith, (0, 3));

    for frame in 0..3 {
        drive_frame(|ui| {
            super::with_surface_parts(&mut app, |state, run_controller| {
                show(ui, state, run_controller, document_id)
            })
        });
        assert!(
            app.state.results.results.session.selected_trace.is_some(),
            "idle frame {frame} cleared the selected trace"
        );
        assert!(
            app.state
                .results
                .results
                .session
                .rf_pin
                .contains_key(&ResultViewer::Smith),
            "idle frame {frame} cleared the pinned readout"
        );
    }
}

/// Projecting a pane that already holds a retained marker draws it in the
/// overlay, under the pane that keeps it.
#[test]
fn projecting_a_pane_draws_its_retained_marker() {
    let (mut app, document_id) = persistent_transient_fixture();
    let pane = projected_pane(&mut app.state, document_id);
    let (trace_id, revision) = {
        let document = app
            .state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .expect("retained document");
        (document.traces()[0].id, document.revision())
    };
    app.state
        .model
        .workspace
        .content
        .transact_visualization_document(
            document_id,
            revision,
            vec![DocumentEdit::AddTypedMarker {
                pane_id: pane.id,
                trace_id,
                coordinate: TypedValue::Real(0.5),
                label: "retained".to_owned(),
                kind: rspice_app_state::results::visualization_document::PlotMarkerKind::PointNote,
                scope: rspice_app_state::results::visualization_document::PlotMarkerScope::Pane,
                source_specification: None,
            }],
        )
        .expect("the document retains a marker");

    let pane = projected_pane(&mut app.state, document_id);

    assert_eq!(app.state.results.results.session.document_markers.len(), 1);
    assert_eq!(
        app.state.results.results.session.document_markers[0].note,
        "retained"
    );
    assert_eq!(
        app.state.results.results.session.document_markers[0].pane_id,
        pane.id
    );
}

/// A click inside a persistent pane is retained by the document.
#[test]
fn placing_a_marker_on_a_persistent_pane_transacts_against_the_document() {
    let (mut app, document_id) = persistent_transient_fixture();
    let pane = projected_pane(&mut app.state, document_id);
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);
    let revision_before = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("retained document")
        .revision();

    let selector = rspice_app_results::workbench::documents::result_document::place_marker(
        &mut app.state.results_parts(),
        placement(analysis, &anchor, "V(out)", 0.5),
    )
    .expect("the placement resolves a store");

    let rspice_results_ui::session::MarkerSelector {
        document_id: routed_document,
        pane_id,
        marker_id,
    } = selector;
    assert_eq!(routed_document, document_id);
    assert_eq!(pane_id, pane.id);
    let retained = retained_markers(&app, document_id);
    assert_eq!(retained.len(), 1);
    assert_eq!(retained[0].id, marker_id);
    assert_eq!(retained[0].coordinate, TypedValue::Real(0.5));
    assert_ne!(
        app.state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .expect("retained document")
            .revision(),
        revision_before,
        "retaining a marker advances the document revision"
    );
    // It is addressable in the frame it was placed in, not one frame later.
    assert!(
        app.state
            .results
            .results
            .session
            .document_marker(marker_id)
            .is_some()
    );
}

/// A saved document places no marker on a trace it does not keep, and
/// says so, rather than keeping the marker somewhere it does not.
#[test]
fn a_trace_the_document_does_not_keep_places_no_marker_and_says_so() {
    let (mut app, document_id) = persistent_transient_fixture();
    let _pane = projected_pane(&mut app.state, document_id);
    let (analysis, _) = active_analysis_anchor(&mut app.state);
    let anchor = rspice_app_results::workbench::documents::result_document::marker_anchor_for(
        analysis,
        "V(a)+V(b)",
    );

    let selector = rspice_app_results::workbench::documents::result_document::place_marker(
        &mut app.state.results_parts(),
        placement(analysis, &anchor, "V(a)+V(b)", 0.25),
    );

    assert_eq!(selector, None);
    assert!(retained_markers(&app, document_id).is_empty());
    assert!(
        app.state
            .prompt_text()
            .is_some_and(|text| text.contains("not a trace this view keeps")),
        "the refusal has to be stated, not silent"
    );
}

/// The quick view's frame around nothing: the shown run's working view is
/// made or followed, drawn, and kept.
fn quick_frame(state: &mut AppState) {
    state.results.results.session.leave_persistent_document();
    let mut parts = state.results_parts();
    rspice_app_results::workbench::documents::result_document::working_view::sync(&mut parts);
    rspice_app_results::workbench::documents::result_document::working_view::capture(&mut parts);
}

/// The markers the shown run's working view keeps.
fn working_view_markers(app: &RSpiceApp) -> Vec<Marker> {
    let run = app
        .state
        .model
        .simulation
        .active_run()
        .expect("a shown run")
        .dataset_id;
    let view =
        rspice_app_results::workbench::documents::result_document::working_view::view_of_run(
            app.state.results_reads(),
            run,
        )
        .expect("the shown run's working view");
    retained_markers(app, view)
}

/// A working view's markers are what the reader made of the run. Opening,
/// drawing and leaving a saved document draws the document's own and
/// leaves the working view's as they were.
#[test]
fn a_working_views_markers_are_untouched_by_opening_and_leaving_a_saved_document() {
    let (mut app, document_id) = persistent_transient_fixture();
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);
    quick_frame(&mut app.state);
    let quick = rspice_app_results::workbench::documents::result_document::place_marker(
        &mut app.state.results_parts(),
        placement(analysis, &anchor, "V(out)", 0.75),
    )
    .expect("the working view keeps the marker");
    rspice_app_results::workbench::documents::result_document::commit_marker_edit(
        &mut app.state.results_parts(),
        quick,
        "dataset note",
        rspice_app_results::workbench::documents::result_document::MarkerKind::Note,
    )
    .expect("the working view takes the note");
    let pane = projected_pane(&mut app.state, document_id);
    rspice_app_results::workbench::documents::result_document::place_marker(
        &mut app.state.results_parts(),
        placement(analysis, &anchor, "V(out)", 0.5),
    )
    .expect("the document retains its own marker");

    for _ in 0..3 {
        drive_frame(|ui| {
            super::with_surface_parts(&mut app, |state, run_controller| {
                show(ui, state, run_controller, document_id)
            })
        });
    }

    let kept = working_view_markers(&app);
    assert_eq!(kept.len(), 1);
    assert_eq!(kept[0].id, quick.marker_id);
    assert_eq!(kept[0].label, "dataset note");
    let overlay = &app.state.results.results.session.document_markers;
    assert_eq!(
        overlay.len(),
        1,
        "the drawn pane's retained markers project into the overlay"
    );
    assert_eq!(overlay[0].pane_id, pane.id);

    // Back on a quick surface the overlay is the working view's again.
    drive_frame(|ui| {
        super::with_surface_parts(&mut app, |state, run_controller| {
            rspice_app_results::workbench::documents::result_document::show_compact_split(
                ui,
                state,
                run_controller,
            )
        })
    });
    assert!(
        app.state
            .results
            .results
            .session
            .persistent_pane_context
            .is_none()
    );
    let overlay = &app.state.results.results.session.document_markers;
    assert_eq!(overlay.len(), 1);
    assert_eq!(overlay[0].retained_id, quick.marker_id);
    assert_eq!(overlay[0].note, "dataset note");
}

/// Apply routes to the view the dialog opened on: an edit of the saved
/// document's marker leaves the working view's, and the other way round.
#[test]
fn the_marker_dialog_applies_to_the_view_it_opened_on() {
    let (mut app, document_id) = persistent_transient_fixture();
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);
    quick_frame(&mut app.state);
    let quick = rspice_app_results::workbench::documents::result_document::place_marker(
        &mut app.state.results_parts(),
        placement(analysis, &anchor, "V(out)", 0.75),
    )
    .expect("the working view keeps the marker");
    let _pane = projected_pane(&mut app.state, document_id);
    let retained = rspice_app_results::workbench::documents::result_document::place_marker(
        &mut app.state.results_parts(),
        placement(analysis, &anchor, "V(out)", 0.5),
    )
    .expect("the document retains its own marker");

    rspice_app_results::workbench::documents::result_document::commit_marker_edit(
        &mut app.state.results_parts(),
        retained,
        "retained",
        rspice_app_results::workbench::documents::result_document::MarkerKind::Peak,
    )
    .expect("the document edit commits");

    let in_document = retained_markers(&app, document_id);
    assert_eq!(in_document.len(), 1);
    assert_eq!(in_document[0].label, "retained");
    assert_eq!(
        in_document[0].kind,
        rspice_app_state::results::visualization_document::PlotMarkerKind::Peak
    );
    assert_eq!(
        working_view_markers(&app)[0].label,
        "V(out)",
        "the working view's marker is untouched"
    );

    rspice_app_results::workbench::documents::result_document::commit_marker_edit(
        &mut app.state.results_parts(),
        quick,
        "edited quick",
        rspice_app_results::workbench::documents::result_document::MarkerKind::Spec,
    )
    .expect("the working view edit commits");

    assert_eq!(working_view_markers(&app)[0].label, "edited quick");
    assert_eq!(
        retained_markers(&app, document_id)[0].label,
        "retained",
        "editing the working view's marker must not reach the document"
    );
}

/// Removing a row removes it from the view that keeps it, and only that
/// view changes.
#[test]
fn removing_a_marker_row_reaches_only_the_view_that_keeps_it() {
    let (mut app, document_id) = persistent_transient_fixture();
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);
    quick_frame(&mut app.state);
    let quick = rspice_app_results::workbench::documents::result_document::place_marker(
        &mut app.state.results_parts(),
        placement(analysis, &anchor, "V(out)", 0.9),
    )
    .expect("the working view keeps the marker");
    let _pane = projected_pane(&mut app.state, document_id);
    let document_selector =
        rspice_app_results::workbench::documents::result_document::place_marker(
            &mut app.state.results_parts(),
            placement(analysis, &anchor, "V(out)", 0.5),
        )
        .expect("the document retains its own marker");
    let revision_before = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("retained document")
        .revision();

    rspice_app_results::workbench::documents::result_document::remove_marker(
        &mut app.state.results_parts(),
        quick,
    );

    assert!(working_view_markers(&app).is_empty());
    assert_eq!(retained_markers(&app, document_id).len(), 1);
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .expect("retained document")
            .revision(),
        revision_before,
        "removing the working view's marker is not a change to the document"
    );

    rspice_app_results::workbench::documents::result_document::remove_marker(
        &mut app.state.results_parts(),
        document_selector,
    );

    assert!(retained_markers(&app, document_id).is_empty());
    assert!(
        app.state
            .results
            .results
            .session
            .document_markers
            .is_empty()
    );
    assert_ne!(
        app.state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .expect("retained document")
            .revision(),
        revision_before
    );
}

#[test]
fn projection_carries_every_document_owned_pane_entity() {
    let (app, document_id) = persistent_transient_fixture();
    let projected =
        projection(app.state.results_reads(), document_id).expect("document projection");
    let pane = &projected.pages[0].panes[0];
    let document = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("retained document");

    assert_eq!(pane.axes, document.axes());
    assert_eq!(pane.traces, document.traces());
    assert_eq!(pane.cursors, document.cursors());
    assert_eq!(pane.markers, document.markers());
    assert_eq!(pane.measurements, document.measurements());
    assert_eq!(pane.annotations, document.annotations());
}

#[test]
fn persistent_trace_axis_and_cursor_interactions_commit_without_mutating_results() {
    let (mut app, document_id) = persistent_transient_fixture();
    let mut projected =
        projection(app.state.results_reads(), document_id).expect("document projection");
    let mut page = projected.pages.remove(0);
    let pane = page.panes.remove(0);
    select_pane_binding(&mut app.state.results_parts(), &pane).expect("pane binding");
    project_pane_presentation(&mut app.state.results_parts(), &pane, ResultViewer::Waves)
        .expect("presentation projects");
    let retained_default =
        app.state.model.simulation.retained.runs[0].analyses[0].waveforms[0].visible;

    rspice_app_results::workbench::documents::result_document::waves::toggle_visibility(
        &mut app.state.results_parts(),
        0,
        0,
    );
    assert_eq!(
        app.state.model.simulation.retained.runs[0].analyses[0].waveforms[0].visible,
        retained_default
    );
    assert!(
        !app.state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .expect("document remains retained")
            .traces()[0]
            .visible
    );

    let analysis =
        rspice_app_results::workbench::documents::result_document::AnalysisPresentationKey::new(
            app.state.model.simulation.retained.runs[0].dataset_id,
            &app.state.model.simulation.retained.runs[0].analyses[0],
        );
    let view = app
        .state
        .results
        .results
        .session
        .analysis_plot_view_pane_mut(ResultViewer::Waves, analysis, 0);
    view.x = Some((0.2, 0.8));
    view.y = Some((-0.25, 1.25));
    app.state.results.results.session.cursors.a = Some(0.5);
    capture_pane_presentation(&mut app.state.results_parts(), &pane, ResultViewer::Waves);

    let document = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("captured document");
    assert_eq!(
        document
            .axes()
            .iter()
            .find(|axis| axis.orientation == AxisOrientation::Horizontal)
            .and_then(|axis| axis.range),
        Some(AxisRange::new(0.2, 0.8).unwrap())
    );
    assert!(
        document
            .cursors()
            .iter()
            .any(|cursor| { cursor.label == "A" && cursor.position == TypedValue::Real(0.5) })
    );
    assert!(
        app.state
            .model
            .workspace
            .content
            .visualization_documents_dirty
    );
}

/// Which pane a reader is working in is a fact about one document.
///
/// A single global selection meant activating a pane in one document
/// selected the pane with the same serial in every other open one, and
/// coming back to a document forgot where the reader had been.
#[test]
fn pane_selection_belongs_to_the_document_it_was_made_in() {
    let (mut app, first) = persistent_transient_fixture();
    app.state.results.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name: "Second transient review".to_owned(),
        name_touched: true,
        dataset_id: Some(app.state.model.simulation.retained.runs[0].dataset_id),
        family_id: "waveform-worksheet".to_owned(),
        viewer_id: "viewer-waveform".to_owned(),
        layout_id: "two-linked-panes".to_owned(),
        validation_error: None,
    };
    let second =
        rspice_app_results::workbench::documents::result_document::create_document::commit(
            &mut app.state.results_parts(),
        )
        .expect("a second document commits");

    drive_frame(|ui| {
        super::with_surface_parts(&mut app, |state, run_controller| {
            show(ui, state, run_controller, first)
        })
    });
    let first_pane = app
        .state
        .results
        .results
        .session
        .persistent_document_pane(first)
        .expect("the first document selected a pane");

    // The reader works in the second pane of the two-pane document.
    let second_panes: Vec<PaneId> = app
        .state
        .model
        .workspace
        .content
        .visualization_document(second)
        .expect("the second document is retained")
        .panes()
        .iter()
        .map(|pane| pane.id)
        .collect();
    assert_eq!(second_panes.len(), 2);
    let working_pane = second_panes[1];
    app.state
        .results
        .results
        .session
        .select_persistent_document_pane(second, working_pane);
    drive_frame(|ui| {
        super::with_surface_parts(&mut app, |state, run_controller| {
            show(ui, state, run_controller, second)
        })
    });
    assert_eq!(
        app.state
            .results
            .results
            .session
            .persistent_document_pane(second),
        Some(working_pane)
    );
    assert_eq!(
        app.state
            .results
            .results
            .session
            .persistent_document_pane(first),
        Some(first_pane),
        "drawing another document must not restate this one's selection"
    );

    // Coming back to each finds the reader where they were, rather than
    // wherever the other document last left one global selection.
    drive_frame(|ui| {
        super::with_surface_parts(&mut app, |state, run_controller| {
            show(ui, state, run_controller, first)
        })
    });
    assert_eq!(
        app.state
            .results
            .results
            .session
            .persistent_document_pane(first),
        Some(first_pane)
    );
    drive_frame(|ui| {
        super::with_surface_parts(&mut app, |state, run_controller| {
            show(ui, state, run_controller, second)
        })
    });
    assert_eq!(
        app.state
            .results
            .results
            .session
            .persistent_document_pane(second),
        Some(working_pane),
        "the second document went back to the pane it was left on"
    );
}

/// (7) Closing a document and reopening it holds the reader's place: the
/// trace they selected, the readout they pinned, and the page and pane
/// they were on.
#[test]
fn closing_and_reopening_a_document_holds_the_readers_place() {
    let (mut app, document_id) = persistent_transient_fixture();
    drive_frame(|ui| {
        super::with_surface_parts(&mut app, |state, run_controller| {
            show(ui, state, run_controller, document_id)
        })
    });
    let pane = app
        .state
        .results
        .results
        .session
        .persistent_document_pane(document_id)
        .expect("the document selected a pane");
    let page = app
        .state
        .results
        .results
        .session
        .persistent_document_page(document_id)
        .expect("the document selected a page");
    let analysis =
        rspice_app_results::workbench::documents::result_document::AnalysisPresentationKey::new(
            app.state.model.simulation.retained.runs[0].dataset_id,
            &app.state.model.simulation.retained.runs[0].analyses[0],
        );
    app.state.results.results.session.selected_trace =
        Some(rspice_results_ui::selection::SelectedResultTrace::from_identity(analysis, "V(out)"));
    app.state
        .results
        .results
        .session
        .rf_pin
        .insert(ResultViewer::Smith, (0, 3));

    // Close: the reader goes back to the dataset quick view, which leaves
    // the persistent projection behind.
    drive_frame(|ui| {
        super::with_surface_parts(&mut app, |state, run_controller| {
            rspice_app_results::workbench::documents::result_document::show_compact_split(
                ui,
                state,
                run_controller,
            )
        })
    });
    assert!(
        app.state
            .results
            .results
            .session
            .persistent_pane_context
            .is_none()
    );

    // Re-open, and hold it open.
    for frame in 0..3 {
        drive_frame(|ui| {
            super::with_surface_parts(&mut app, |state, run_controller| {
                show(ui, state, run_controller, document_id)
            })
        });
        assert!(
            app.state.results.results.session.selected_trace.is_some(),
            "frame {frame} after re-opening lost the selected trace"
        );
        assert!(
            app.state
                .results
                .results
                .session
                .rf_pin
                .contains_key(&ResultViewer::Smith),
            "frame {frame} after re-opening lost the pinned readout"
        );
    }
    assert_eq!(
        app.state
            .results
            .results
            .session
            .persistent_document_pane(document_id),
        Some(pane)
    );
    assert_eq!(
        app.state
            .results
            .results
            .session
            .persistent_document_page(document_id),
        Some(page)
    );
}

/// A retained pane holds one vertical axis while the waveform stack draws
/// a unit pane per quantity. Zooming the amps pane of a V+I document must
/// land on the amps pane, and must not be written into the document's one
/// vertical axis in place of the volts pane the document actually states.
#[test]
fn zooming_one_unit_pane_of_a_document_never_restates_another_pane_axis() {
    let (mut app, document_id) = persistent_transient_fixture();
    let pane = projected_pane(&mut app.state, document_id);
    let analysis =
        rspice_app_results::workbench::documents::result_document::AnalysisPresentationKey::new(
            app.state.model.simulation.retained.runs[0].dataset_id,
            &app.state.model.simulation.retained.runs[0].analyses[0],
        );

    // The volts pane — the one the document's vertical axis states.
    let volts = app
        .state
        .results
        .results
        .session
        .analysis_plot_view_pane_mut(ResultViewer::Waves, analysis, 0);
    volts.x = Some((0.2, 0.8));
    volts.y = Some((-1.0, 1.0));
    // The amps pane, whose scale the document cannot also state.
    let amps = app
        .state
        .results
        .results
        .session
        .analysis_plot_view_pane_mut(ResultViewer::Waves, analysis, 1);
    amps.y = Some((-5.0e-3, 5.0e-3));

    capture_pane_presentation(&mut app.state.results_parts(), &pane, ResultViewer::Waves);

    let vertical = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("captured document")
        .axes()
        .iter()
        .find(|axis| axis.orientation == AxisOrientation::VerticalLeft)
        .and_then(|axis| axis.range);
    assert_eq!(
        vertical,
        Some(AxisRange::new(-1.0, 1.0).expect("a valid range")),
        "the document states its own pane's vertical axis, not whichever \
         unit pane a hash map yielded first"
    );

    // Re-projecting must leave the amps pane's zoom alone: it is the only
    // owner of that window, and wiping it every frame made a multi-pane
    // document unable to hold a zoom on anything but its first pane.
    let _pane = projected_pane(&mut app.state, document_id);
    assert_eq!(
        app.state
            .results
            .results
            .session
            .analysis_plot_view_pane(ResultViewer::Waves, analysis, 1)
            .y,
        Some((-5.0e-3, 5.0e-3))
    );
    assert_eq!(
        app.state
            .results
            .results
            .session
            .analysis_plot_view_pane(ResultViewer::Waves, analysis, 0)
            .y,
        Some((-1.0, 1.0)),
        "the retained pane's own window comes back from the document"
    );
}

/// A saved X link makes its panes one X range: zooming one zooms the other.
/// The viewer wrote only the zoomed pane's own axis, so the link did nothing.
#[test]
fn zooming_one_pane_of_a_saved_x_link_zooms_the_other() {
    let (mut app, document_id) = transient_fixture_with_layout("two-linked-panes");
    reload(&mut app, document_id);
    drive_frame(|ui| {
        super::with_surface_parts(&mut app, |state, run_controller| {
            show(ui, state, run_controller, document_id)
        })
    });
    let page = projection(app.state.results_reads(), document_id)
        .expect("document projection")
        .pages
        .remove(0);
    let (first, second) = (&page.panes[0], &page.panes[1]);
    let analysis =
        rspice_app_results::workbench::documents::result_document::AnalysisPresentationKey::new(
            app.state.model.simulation.retained.runs[0].dataset_id,
            &app.state.model.simulation.retained.runs[0].analyses[0],
        );

    draw_pane(&mut app, first, |state| {
        state
            .results
            .results
            .session
            .analysis_plot_view_pane_mut(ResultViewer::Waves, analysis, 0)
            .x = Some((0.2, 0.8));
    });
    let zoomed = Some(AxisRange::new(0.2, 0.8).expect("a valid range"));
    assert_eq!(saved_x_range(&app, document_id, first.id), zoomed);
    assert_eq!(
        saved_x_range(&app, document_id, second.id),
        zoomed,
        "the linked pane follows the zoom"
    );

    // The second pane draws later in the same frame, from the frame's
    // snapshot. Holding that stale range must not undo the link.
    draw_pane(&mut app, second, |_| {});
    assert_eq!(saved_x_range(&app, document_id, first.id), zoomed);
    assert_eq!(saved_x_range(&app, document_id, second.id), zoomed);
}

/// One frame of the Results workspace over `document_id`, `time` seconds
/// into the session.
fn document_frame(app: &mut RSpiceApp, document_id: ResultDocumentId, time: f64) {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let input = egui::RawInput {
        time: Some(time),
        ..Default::default()
    };
    let _ = ctx.run_ui(input, |ui| {
        super::with_surface_parts(app, |state, run_controller| {
            rspice_app_results::workbench::documents::result_document::show_persistent_document(
                ui,
                state,
                run_controller,
                document_id,
            );
        });
    });
}

/// Zoom previous steps a result document back through the windows it
/// showed, and the window the document keeps in its own axes comes back
/// with it, not only the session's copy.
#[test]
fn zoom_previous_steps_a_result_document_back_through_its_kept_windows() {
    use rspice_app_workbench::workbench::framework::command_registry::ids;
    use rspice_app_workbench::workbench::framework::host::Host;
    let (mut app, document_id) = persistent_transient_fixture();
    app.state
        .workbench
        .activate(rspice_app_state::workbench::workspace::Workspace::Results);
    let key = ids::ZOOM_PREVIOUS;
    document_frame(&mut app, document_id, 0.0);
    let pane_id = projection(app.state.results_reads(), document_id)
        .expect("document projection")
        .pages[0]
        .panes[0]
        .id;
    let run = &app.state.model.simulation.retained.runs[0];
    let analysis =
        rspice_app_results::workbench::documents::result_document::AnalysisPresentationKey::new(
            run.dataset_id,
            &run.analyses[0],
        );
    app.state.results.results.session.active_wave_pane = Some(
        rspice_app_results::workbench::documents::result_document::WavePanePresentationKey {
            analysis,
            unit: "V".to_owned(),
        },
    );
    let opened = saved_x_range(&app, document_id, pane_id);
    assert!(!app.is_enabled(key));

    app.execute(ids::ZOOM_IN);
    document_frame(&mut app, document_id, 1.0);
    let nearer = saved_x_range(&app, document_id, pane_id);
    app.execute(ids::ZOOM_IN);
    document_frame(&mut app, document_id, 2.0);
    assert_ne!(saved_x_range(&app, document_id, pane_id), nearer);
    assert_ne!(nearer, opened);

    assert!(app.is_enabled(key));
    app.execute(key);
    assert_eq!(saved_x_range(&app, document_id, pane_id), nearer);
    document_frame(&mut app, document_id, 3.0);
    assert_eq!(saved_x_range(&app, document_id, pane_id), nearer);
    app.execute(key);
    document_frame(&mut app, document_id, 4.0);
    assert_eq!(saved_x_range(&app, document_id, pane_id), opened);
    assert!(!app.is_enabled(key));
}

/// Linked cursors over a document are the document's: they survive a save,
/// and moving the A cursor in one pane moves it in the other.
#[test]
fn linked_cursors_are_kept_by_the_document_and_move_together() {
    let (mut app, document_id) = transient_fixture_with_layout("two-linked-panes");
    drive_frame(|ui| {
        super::with_surface_parts(&mut app, |state, run_controller| {
            show(ui, state, run_controller, document_id)
        })
    });
    let page = projection(app.state.results_reads(), document_id)
        .expect("document projection")
        .pages
        .remove(0);
    let (first, second) = (page.panes[0].clone(), page.panes[1].clone());
    draw_pane(&mut app, &first, |state| {
        state.results.results.session.cursors.a = Some(0.25)
    });
    draw_pane(&mut app, &second, |state| {
        state.results.results.session.cursors.a = Some(0.75)
    });
    let linked_before = app.state.results.results.session.linked_cursors;

    rspice_app_results::workbench::documents::result_document::toggle_linked_cursors(
        &mut app.state.results_parts(),
    );
    assert_eq!(
        app.state.results.results.session.linked_cursors, linked_before,
        "a document's cursor links are not the quick view's toggle"
    );
    let reference = saved_cursor_a(&app, document_id, first.id);
    assert_eq!(saved_cursor_a(&app, document_id, second.id), reference);

    reload(&mut app, document_id);
    drive_frame(|ui| {
        super::with_surface_parts(&mut app, |state, run_controller| {
            show(ui, state, run_controller, document_id)
        })
    });
    let page = projection(app.state.results_reads(), document_id)
        .expect("document projection")
        .pages
        .remove(0);
    draw_pane(&mut app, &page.panes[0], |state| {
        state.results.results.session.cursors.a = Some(0.5);
    });
    draw_pane(&mut app, &page.panes[1], |_| {});
    assert_eq!(saved_cursor_a(&app, document_id, first.id), Some(0.5));
    assert_eq!(
        saved_cursor_a(&app, document_id, second.id),
        Some(0.5),
        "the linked cursor follows"
    );

    rspice_app_results::workbench::documents::result_document::toggle_linked_cursors(
        &mut app.state.results_parts(),
    );
    let document = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("the project retains the document");
    assert!(
        !document.link_groups().iter().any(|group| group.kind
            == rspice_app_state::results::visualization_document::LinkKind::CursorPosition),
        "the second toggle takes the links away"
    );
}

/// The fixture's one trace's X samples, as the drawn pane knows them.
static FIXTURE_SAMPLES: [f64; 3] = [0.0, 0.5, 1.0];

/// Undo and Redo in Results step the shown view's own edits, one reader's
/// edit at a time, and never reach the schematic's history.
#[test]
fn undo_in_results_steps_the_view_and_leaves_the_schematic_alone() {
    use rspice_app_state::state::{ComponentType, Point};
    use rspice_app_workbench::workbench::framework::command_registry::{CommandAvailability, ids};
    use rspice_app_workbench::workbench::state::Workspace;

    let (mut app, document_id) = persistent_transient_fixture();
    app.state.model.schematic.init_undo_history();
    app.state
        .model
        .schematic
        .with_undo("place R1", |schematic| {
            schematic.add_component(ComponentType::Resistor, Point::new(0, 0));
        });
    assert!(app.state.model.schematic.can_undo());
    let placed = app.state.model.schematic.document().components.len();
    app.state.workbench.activate(Workspace::Results);
    let _pane = projected_pane(&mut app.state, document_id);
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);
    assert_ne!(
        app.command_availability(ids::UNDO),
        CommandAvailability::Available,
        "nothing the reader did in this view yet"
    );

    for x in [0.0, 1.0] {
        rspice_app_results::workbench::documents::result_document::place_marker(
            &mut app.state.results_parts(),
            placement(analysis, &anchor, "V(out)", x),
        )
        .expect("the placement resolves a store");
    }
    assert_eq!(retained_markers(&app, document_id).len(), 2);

    app.execute_command_key(ids::UNDO);
    assert_eq!(retained_markers(&app, document_id).len(), 1);
    app.execute_command_key(ids::UNDO);
    assert!(retained_markers(&app, document_id).is_empty());
    assert_eq!(
        app.state.model.schematic.document().components.len(),
        placed,
        "Undo in Results never takes back a schematic edit"
    );
    assert!(app.state.model.schematic.can_undo());

    app.execute_command_key(ids::REDO);
    let redone = retained_markers(&app, document_id);
    assert_eq!(redone.len(), 1);
    assert_eq!(redone[0].coordinate, TypedValue::Real(0.0));
}

/// A project opened while Results shows a saved view opens on its own
/// Results, not on the view of the project it replaced.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_project_opened_over_a_shown_view_does_not_show_that_view() {
    use rspice_app_workbench::workbench::state::{Workspace, WorkspaceDocumentId};

    let path = std::env::temp_dir().join(format!(
        "rspice-opened-over-results-{}-{}.rspiceproj",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    ));
    let mut other = RSpiceApp::test_instance();
    assert!(
        other.state.save_project_to_path(&path),
        "the other project saves"
    );
    drop(other);
    let LatestFixture {
        mut app,
        document_id,
        ..
    } = latest_tracking_fixture();
    app.state.workbench.activate(Workspace::Results);
    assert_eq!(
        app.state.workbench.documents.active(Workspace::Results),
        Some(&WorkspaceDocumentId::VisualizationDocument(document_id))
    );

    // The fixture's runs stand in for a simulation without one, which a
    // project cannot save; the view stays shown without them.
    app.state.model.simulation.retained.runs = Vec::new().into();
    let opened = app.state.load_project_from_path(&path);
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
    let mut lock = path.as_os_str().to_os_string();
    lock.push(".rspice.lock");
    let _ = std::fs::remove_file(std::path::PathBuf::from(lock));
    assert!(
        opened,
        "the other project opens: {:?} {:?}",
        app.state
            .workbench
            .prompt
            .as_ref()
            .map(|prompt| &prompt.text),
        app.state
            .model
            .notices
            .take()
            .into_iter()
            .map(|notice| format!("{}: {}", notice.title, notice.detail))
            .collect::<Vec<_>>()
    );

    assert!(
        app.state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .is_none()
    );
    assert_ne!(
        app.state.workbench.documents.active(Workspace::Results),
        Some(&WorkspaceDocumentId::VisualizationDocument(document_id)),
        "Results no longer names the replaced project's view"
    );
}
