//! Latest tracking of a project-owned Results document, against the
//! project's authored source and the runs it authorizes.

use crate::product::{
    AnalysisInstanceId, ContentDigest, ObjectRevision, ResultDocumentId, SimulationPlanId,
};
use crate::results::visualization_document::AxisOrientation;
use crate::results::visualization_document::AxisRange;
use crate::results::visualization_document::DocumentEdit;
use crate::results::visualization_document::Marker;
use crate::results::visualization_document::PaneId;
use crate::results::visualization_document::TypedValue;
use crate::state::AnalysisResultSourceDomain;
use crate::state::PreparedRunReceipt;
use crate::state::PreparedRunTaskReceipt;
use crate::state::PreparedSourceCheckReceipt;
use crate::state::SimulationRunLifecycle;
use crate::state::SimulationRunProvenance;
use crate::state::{AnalysisResult, AnalysisResultProvenance, AnalysisType, SimulationRun};
use crate::workbench::AppState;
use crate::workbench::RSpiceApp;
use crate::workbench::documents::result_document::ResultViewer;
use crate::workbench::documents::result_document::create_document;
use crate::workbench::documents::result_document::persistent_document::*;
use crate::workbench::documents::result_document::persistent_document::{
    LatestBinding, current_result_source_digest, refresh_latest_binding, show,
};
use egui::Ui;
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
        super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
            show(
                ui,
                state,
                run_controller,
                selected_specification,
                document_id,
            )
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
            .latest_retarget_failure(document_id, crate::product::DatasetId::new()),
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
    crate::workbench::documents::result_document::AnalysisPresentationKey,
    crate::workbench::documents::result_document::WaveformPresentationKey,
) {
    crate::workbench::documents::result_document::waves::source_waveform_anchor(
        &mut state.results_parts(),
        "V(out)",
    )
    .expect("the fixture retains V(out)")
}

fn placement(
    analysis: crate::workbench::documents::result_document::AnalysisPresentationKey,
    anchor: &crate::workbench::documents::result_document::WaveformPresentationKey,
    trace_name: &str,
    x: f64,
) -> crate::workbench::documents::result_document::MarkerPlacement<'static> {
    crate::workbench::documents::result_document::MarkerPlacement {
        analysis,
        anchor: anchor.clone(),
        trace_name: trace_name.to_owned(),
        x,
        samples: &FIXTURE_SAMPLES,
    }
}

fn transient_fixture_with_layout(layout_id: &str) -> (RSpiceApp, ResultDocumentId) {
    let mut app = RSpiceApp::test_instance();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            crate::state::RetainedTrace::new(
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
    let document_id = crate::workbench::documents::result_document::create_document::commit(
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
    crate::ui::Theme::default().apply(&ctx);
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
        super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
            show(
                ui,
                state,
                run_controller,
                selected_specification,
                document_id,
            )
        })
    });
    let settled = app.state.model.simulation.view.data_version;

    for frame in 0..4 {
        drive_frame(|ui| {
            super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
                show(
                    ui,
                    state,
                    run_controller,
                    selected_specification,
                    document_id,
                )
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
        super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
            show(
                ui,
                state,
                run_controller,
                selected_specification,
                document_id,
            )
        })
    });
    let analysis = crate::workbench::documents::result_document::AnalysisPresentationKey::new(
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
            super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
                show(
                    ui,
                    state,
                    run_controller,
                    selected_specification,
                    document_id,
                )
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

/// (h.2) Projecting a pane that already holds a retained marker must not
/// adopt it into the project's quick-view list.
///
/// The projection used to write the document's markers straight into
/// `ResultsState::markers` — the list the project file saves — so a
/// document's own annotation was saved a second time as a dataset marker,
/// under an id truncated out of the document's entity serial.
#[test]
fn projecting_a_retained_marker_never_adopts_it_into_the_quick_list() {
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
                kind: crate::results::visualization_document::PlotMarkerKind::PointNote,
                scope: crate::results::visualization_document::PlotMarkerScope::Pane,
                source_specification: None,
            }],
        )
        .expect("the document retains a marker");

    let pane = projected_pane(&mut app.state, document_id);

    assert!(
        app.state.results.results.session.markers.is_empty(),
        "a retained document marker leaked into the project's quick-view list: {:?}",
        app.state
            .results
            .results
            .session
            .markers
            .iter()
            .map(|marker| (marker.id, marker.note.clone()))
            .collect::<Vec<_>>()
    );
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

/// (h.7) A click inside a persistent pane is retained by the document,
/// not copied into the project's quick-view marker list.
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

    let selector = crate::workbench::documents::result_document::place_marker(
        &mut app.state.results_parts(),
        placement(analysis, &anchor, "V(out)", 0.5),
    )
    .expect("the placement resolves a store");

    let rspice_results_ui::session::MarkerSelector::Document {
        document_id: routed_document,
        pane_id,
        marker_id,
    } = selector
    else {
        panic!("a persistent pane must retain its own markers");
    };
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
    assert!(
        app.state.results.results.session.markers.is_empty(),
        "a document marker must never enter the project's quick-view list"
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

/// (h.7) A document cannot retain a marker on a trace it does not own, so
/// the click still lands — on the dataset — and says so.
#[test]
fn a_trace_the_document_does_not_retain_falls_back_to_a_quick_marker() {
    let (mut app, document_id) = persistent_transient_fixture();
    let _pane = projected_pane(&mut app.state, document_id);
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);

    let selector = crate::workbench::documents::result_document::place_marker(
        &mut app.state.results_parts(),
        placement(analysis, &anchor, "V(a)+V(b)", 0.25),
    )
    .expect("the placement resolves a store");

    assert!(matches!(
        selector,
        rspice_results_ui::session::MarkerSelector::Quick(_)
    ));
    assert_eq!(app.state.results.results.session.markers.len(), 1);
    assert!(retained_markers(&app, document_id).is_empty());
    assert!(
        app.state
            .prompt_text()
            .is_some_and(|text| text.contains("not a retained trace")),
        "the fallback has to be stated, not silent"
    );
}

/// (h.1) Quick markers are a fact about the dataset. Opening, drawing and
/// leaving a project-owned document must not adopt, rewrite or drop them.
#[test]
fn quick_markers_are_untouched_by_opening_and_leaving_a_persistent_document() {
    let (mut app, document_id) = persistent_transient_fixture();
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);
    let quick = app
        .state
        .results
        .results
        .session
        .add_marker(analysis, anchor.clone(), "V(out)".to_owned(), 0.75)
        .unwrap();
    if let Some(marker) = app.state.results.results.session.marker_mut(quick) {
        marker.note = "dataset note".to_owned();
    }
    let pane = projected_pane(&mut app.state, document_id);
    crate::workbench::documents::result_document::place_marker(
        &mut app.state.results_parts(),
        placement(analysis, &anchor, "V(out)", 0.5),
    )
    .expect("the document retains its own marker");

    for _ in 0..3 {
        drive_frame(|ui| {
            super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
                show(
                    ui,
                    state,
                    run_controller,
                    selected_specification,
                    document_id,
                )
            })
        });
    }

    assert_eq!(app.state.results.results.session.markers.len(), 1);
    assert_eq!(app.state.results.results.session.markers[0].id, quick);
    assert_eq!(
        app.state.results.results.session.markers[0].note,
        "dataset note"
    );
    assert_eq!(
        app.state.results.results.session.document_markers.len(),
        1,
        "the drawn pane's retained markers project into the overlay"
    );
    assert_eq!(
        app.state.results.results.session.document_markers[0].pane_id,
        pane.id
    );

    // Back on a quick surface the overlay is gone and the quick marker is
    // exactly what it was.
    drive_frame(|ui| {
        super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
            crate::workbench::documents::result_document::show_compact_split(
                ui,
                state,
                run_controller,
                selected_specification,
            )
        })
    });
    assert!(
        app.state
            .results
            .results
            .session
            .document_markers
            .is_empty()
    );
    assert!(
        app.state
            .results
            .results
            .session
            .persistent_pane_context
            .is_none()
    );
    assert_eq!(app.state.results.results.session.markers.len(), 1);
    assert_eq!(
        app.state.results.results.session.markers[0].note,
        "dataset note"
    );
}

/// (h.2, h.3) The two stores never contend: a document serial cannot
/// advance the quick allocator, and the project save carries exactly the
/// quick markers.
#[test]
fn document_marker_serials_never_reach_the_project_save_or_the_quick_allocator() {
    let (mut app, document_id) = persistent_transient_fixture();
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);
    let _pane = projected_pane(&mut app.state, document_id);
    let selector = crate::workbench::documents::result_document::place_marker(
        &mut app.state.results_parts(),
        placement(analysis, &anchor, "V(out)", 0.5),
    )
    .expect("the document retains its own marker");
    let rspice_results_ui::session::MarkerSelector::Document { marker_id, .. } = selector else {
        panic!("a persistent pane retains its own markers");
    };

    let quick = app
        .state
        .results
        .results
        .session
        .add_marker(analysis, anchor, "V(out)".to_owned(), 0.9)
        .unwrap();

    assert_eq!(
        quick,
        1,
        "the quick allocator counts quick markers alone, whatever serial \
         the document handed its own marker ({})",
        marker_id.get()
    );
    assert_eq!(
        app.state.results.results.session.markers.len(),
        1,
        "the project's saved marker list is exactly the quick markers"
    );
    assert_eq!(app.state.results.results.session.markers[0].id, quick);
}

/// (h.5) Apply routes by the store the dialog opened on, even when a quick
/// id and a document serial are the same number — the case the old shared
/// `u32` space could not tell apart.
#[test]
fn the_marker_dialog_applies_to_the_store_it_opened_on() {
    let (mut app, document_id) = persistent_transient_fixture();
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);
    let _pane = projected_pane(&mut app.state, document_id);
    let selector = crate::workbench::documents::result_document::place_marker(
        &mut app.state.results_parts(),
        placement(analysis, &anchor, "V(out)", 0.5),
    )
    .expect("the document retains its own marker");
    let rspice_results_ui::session::MarkerSelector::Document { marker_id, .. } = selector else {
        panic!("a persistent pane retains its own markers");
    };
    // Give the quick store a marker whose id is the document serial, so
    // the two identities collide as integers and can only be told apart
    // by the store they name.
    let colliding = u32::try_from(marker_id.get()).expect("a small test serial");
    app.state.results.results.session.adopt_markers(
        vec![crate::workbench::documents::result_document::ResultMarker {
            id: colliding,
            analysis,
            anchor,
            trace_name: "V(out)".to_owned(),
            x: 0.9,
            kind: crate::workbench::documents::result_document::MarkerKind::Note,
            note: "quick".to_owned(),
        }],
        0,
    );

    crate::workbench::documents::result_document::commit_marker_edit(
        &mut app.state.results_parts(),
        selector,
        "retained",
        crate::workbench::documents::result_document::MarkerKind::Peak,
    )
    .expect("the document edit commits");

    let retained = retained_markers(&app, document_id);
    assert_eq!(retained.len(), 1);
    assert_eq!(retained[0].label, "retained");
    assert_eq!(
        retained[0].kind,
        crate::results::visualization_document::PlotMarkerKind::Peak
    );
    assert_eq!(
        app.state.results.results.session.markers[0].note, "quick",
        "the quick marker that shares the number must be untouched"
    );
    assert_eq!(
        app.state.results.results.session.markers[0].kind,
        crate::workbench::documents::result_document::MarkerKind::Note
    );

    crate::workbench::documents::result_document::commit_marker_edit(
        &mut app.state.results_parts(),
        rspice_results_ui::session::MarkerSelector::Quick(colliding),
        "edited quick",
        crate::workbench::documents::result_document::MarkerKind::Spec,
    )
    .expect("the quick edit commits");

    assert_eq!(
        app.state.results.results.session.markers[0].note,
        "edited quick"
    );
    assert_eq!(
        retained_markers(&app, document_id)[0].label,
        "retained",
        "editing the quick marker must not reach the document"
    );
}

/// (h.6) Removing a row removes it from the store that owns it, and only
/// a document row advances the document.
#[test]
fn removing_a_marker_row_reaches_only_the_store_that_owns_it() {
    let (mut app, document_id) = persistent_transient_fixture();
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);
    let _pane = projected_pane(&mut app.state, document_id);
    let document_selector = crate::workbench::documents::result_document::place_marker(
        &mut app.state.results_parts(),
        placement(analysis, &anchor, "V(out)", 0.5),
    )
    .expect("the document retains its own marker");
    let quick = app
        .state
        .results
        .results
        .session
        .add_marker(analysis, anchor, "V(out)".to_owned(), 0.9)
        .unwrap();
    let revision_before = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("retained document")
        .revision();

    crate::workbench::documents::result_document::remove_marker(
        &mut app.state.results_parts(),
        rspice_results_ui::session::MarkerSelector::Quick(quick),
    );

    assert!(app.state.results.results.session.markers.is_empty());
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
        "removing a quick marker is not a change to the document"
    );

    crate::workbench::documents::result_document::remove_marker(
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

/// (h.4) A project saved before markers had one owner captured the
/// document's markers into the quick list as well. Reopening must keep the
/// genuine quick marker and drop the duplicate rather than draw both.
#[test]
fn a_saved_projection_of_a_retained_marker_is_dropped_on_load() {
    let (mut app, document_id) = persistent_transient_fixture();
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);
    let _pane = projected_pane(&mut app.state, document_id);
    crate::workbench::documents::result_document::place_marker(
        &mut app.state.results_parts(),
        placement(analysis, &anchor, "V(out)", 0.5),
    )
    .expect("the document retains its own marker");
    let marker_id = retained_markers(&app, document_id)[0].id;
    crate::workbench::documents::result_document::commit_marker_edit(
        &mut app.state.results_parts(),
        rspice_results_ui::session::MarkerSelector::Document {
            document_id,
            pane_id: _pane.id,
            marker_id,
        },
        "overshoot",
        crate::workbench::documents::result_document::MarkerKind::Peak,
    )
    .expect("the document edit commits");

    let duplicate = crate::workbench::documents::result_document::ResultMarker {
        id: 4,
        analysis,
        anchor: anchor.clone(),
        trace_name: "V(out)".to_owned(),
        x: 0.5,
        kind: crate::workbench::documents::result_document::MarkerKind::Peak,
        note: "overshoot".to_owned(),
    };
    let genuine = crate::workbench::documents::result_document::ResultMarker {
        id: 5,
        analysis,
        anchor,
        trace_name: "V(out)".to_owned(),
        x: 0.9,
        kind: crate::workbench::documents::result_document::MarkerKind::Note,
        note: "settling".to_owned(),
    };

    crate::workbench::documents::result_document::restore_markers(
        &mut app.state.results_parts(),
        vec![duplicate.clone(), genuine.clone()],
        None,
    );

    assert_eq!(app.state.results.results.session.markers.len(), 1);
    assert_eq!(
        app.state.results.results.session.markers[0].note,
        "settling"
    );

    // Current files explicitly own quick markers. Identical annotation text
    // and coordinates no longer identify a legacy document projection.
    crate::workbench::documents::result_document::restore_presentation(
        &mut app.state.results_parts(),
        crate::state::result_presentation::ResultPresentation {
            markers: vec![duplicate, genuine],
            marker_id_high_water: Some(5),
            ..Default::default()
        },
    );
    assert_eq!(app.state.results.results.session.markers.len(), 2);
    assert_eq!(
        app.state.results.results.session.markers[0].note,
        "overshoot"
    );
}

#[test]
fn matching_marker_text_on_another_dataset_is_preserved_on_load() {
    let (mut app, document_id) = persistent_transient_fixture();
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);
    projected_pane(&mut app.state, document_id);
    crate::workbench::documents::result_document::place_marker(
        &mut app.state.results_parts(),
        placement(analysis, &anchor, "V(out)", 0.5),
    )
    .expect("document marker");
    let retained = retained_markers(&app, document_id).remove(0);
    let mut another_run = SimulationRun::new(2);
    another_run.add_analysis(app.state.model.simulation.retained.runs[0].analyses[0].clone());
    let another_analysis =
        crate::workbench::documents::result_document::AnalysisPresentationKey::new(
            another_run.dataset_id,
            &another_run.analyses[0],
        );
    app.state.model.simulation.retained.runs.push(another_run);
    let mut another_anchor = anchor;
    another_anchor.analysis = another_analysis;
    let marker = crate::workbench::documents::result_document::ResultMarker {
        id: 5,
        analysis: another_analysis,
        anchor: another_anchor,
        trace_name: "V(out)".to_owned(),
        x: 0.5,
        kind: crate::workbench::documents::result_document::marker_kind_of_retained(retained.kind),
        note: retained.label,
    };
    crate::workbench::documents::result_document::restore_markers(
        &mut app.state.results_parts(),
        vec![marker],
        None,
    );
    assert_eq!(app.state.results.results.session.markers.len(), 1);
    assert_eq!(
        app.state.results.results.session.markers[0].analysis,
        another_analysis
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

    crate::workbench::documents::result_document::waves::toggle_visibility(
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

    let analysis = crate::workbench::documents::result_document::AnalysisPresentationKey::new(
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
    let second = crate::workbench::documents::result_document::create_document::commit(
        &mut app.state.results_parts(),
    )
    .expect("a second document commits");

    drive_frame(|ui| {
        super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
            show(ui, state, run_controller, selected_specification, first)
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
        super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
            show(ui, state, run_controller, selected_specification, second)
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
        super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
            show(ui, state, run_controller, selected_specification, first)
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
        super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
            show(ui, state, run_controller, selected_specification, second)
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
        super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
            show(
                ui,
                state,
                run_controller,
                selected_specification,
                document_id,
            )
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
    let analysis = crate::workbench::documents::result_document::AnalysisPresentationKey::new(
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
        super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
            crate::workbench::documents::result_document::show_compact_split(
                ui,
                state,
                run_controller,
                selected_specification,
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
            super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
                show(
                    ui,
                    state,
                    run_controller,
                    selected_specification,
                    document_id,
                )
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
    let analysis = crate::workbench::documents::result_document::AnalysisPresentationKey::new(
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
        super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
            show(
                ui,
                state,
                run_controller,
                selected_specification,
                document_id,
            )
        })
    });
    let page = projection(app.state.results_reads(), document_id)
        .expect("document projection")
        .pages
        .remove(0);
    let (first, second) = (&page.panes[0], &page.panes[1]);
    let analysis = crate::workbench::documents::result_document::AnalysisPresentationKey::new(
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

/// Linked cursors over a document are the document's: they survive a save,
/// and moving the A cursor in one pane moves it in the other.
#[test]
fn linked_cursors_are_kept_by_the_document_and_move_together() {
    let (mut app, document_id) = transient_fixture_with_layout("two-linked-panes");
    drive_frame(|ui| {
        super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
            show(
                ui,
                state,
                run_controller,
                selected_specification,
                document_id,
            )
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

    crate::workbench::documents::result_document::toggle_linked_cursors(
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
        super::with_surface_parts(&mut app, |state, run_controller, selected_specification| {
            show(
                ui,
                state,
                run_controller,
                selected_specification,
                document_id,
            )
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

    crate::workbench::documents::result_document::toggle_linked_cursors(
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
        !document
            .link_groups()
            .iter()
            .any(|group| group.kind
                == crate::results::visualization_document::LinkKind::CursorPosition),
        "the second toggle takes the links away"
    );
}

pub(crate) fn digest(byte: u8) -> ContentDigest {
    ContentDigest::from_bytes([byte; 32])
}

pub(crate) fn completed_prepared_run(
    plan_id: SimulationPlanId,
    project_revision: ObjectRevision,
    source_digest: ContentDigest,
) -> SimulationRun {
    let task = PreparedRunTaskReceipt::new(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        Vec::new(),
        5,
        digest(0x44),
    )
    .expect("task receipt");
    let receipt = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
        source_domain: AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(plan_id),
        project_revision,
        prepared_snapshot_digest: digest(0x41),
        source_content_digest: source_digest,
        source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0x43)),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        tasks: vec![task],
    })
    .expect("run receipt");
    let mut run = SimulationRun::new(1);
    run.restore_provenance(SimulationRunProvenance::Prepared(Box::new(receipt)))
        .expect("run provenance");
    run.mark_running().expect("running lifecycle");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("completed lifecycle");
    run
}

/// The fixture's one trace's X samples, as the drawn pane knows them.
static FIXTURE_SAMPLES: [f64; 3] = [0.0, 0.5, 1.0];
