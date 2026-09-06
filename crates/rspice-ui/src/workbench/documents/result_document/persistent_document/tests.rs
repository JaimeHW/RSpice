//! Lifecycle, projection, marker-ownership and Latest-tracking tests for
//! the project-owned Results document projection.

use super::*;
use crate::state::{
    AnalysisResultProvenance, AnalysisResultSourceDomain, PreparedRunReceipt,
    PreparedRunTaskReceipt, PreparedSourceCheckReceipt, SimulationRunLifecycle,
    SimulationRunProvenance,
};
use crate::workbench::state::CreateResultDocumentDialogState;

fn digest(byte: u8) -> ContentDigest {
    ContentDigest::from_bytes([byte; 32])
}

fn persistent_transient_fixture() -> (RSpiceApp, ResultDocumentId) {
    let mut app = RSpiceApp::test_instance();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            crate::state::WaveformData::new(
                "V(out)",
                vec![0.0, 0.5, 1.0],
                vec![0.0, 1.0, 0.0],
                "#fff",
            ),
        ]),
    );
    let dataset_id = run.dataset_id;
    app.state.simulation.runs = vec![run];
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));
    app.state.workbench.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name: "Durable transient review".to_owned(),
        name_touched: true,
        dataset_id: Some(dataset_id),
        family_id: "waveform-worksheet".to_owned(),
        viewer_id: "viewer-waveform".to_owned(),
        layout_id: "single-pane".to_owned(),
        validation_error: None,
    };
    let document_id =
        super::super::create_document::commit(&mut app).expect("persistent document commits");
    (app, document_id)
}

/// Drive one whole frame of a surface, with the product theme applied so
/// token lookups and font metrics resolve exactly as they do on screen.
fn drive_frame(body: impl FnMut(&mut Ui)) {
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
    drive_frame(|ui| show(ui, &mut app, document_id));
    let settled = app.state.simulation.data_version;

    for frame in 0..4 {
        drive_frame(|ui| show(ui, &mut app, document_id));
        assert_eq!(
            app.state.simulation.data_version, settled,
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
    drive_frame(|ui| show(ui, &mut app, document_id));
    let analysis = super::super::AnalysisPresentationKey::new(
        app.state.simulation.runs[0].dataset_id,
        &app.state.simulation.runs[0].analyses[0],
    );
    app.state.ui.results.selected_trace = Some(super::super::SelectedResultTrace::from_identity(
        analysis, "V(out)",
    ));
    app.state
        .ui
        .results
        .rf_pin
        .insert(ResultViewer::Smith, (0, 3));

    for frame in 0..3 {
        drive_frame(|ui| show(ui, &mut app, document_id));
        assert!(
            app.state.ui.results.selected_trace.is_some(),
            "idle frame {frame} cleared the selected trace"
        );
        assert!(
            app.state
                .ui
                .results
                .rf_pin
                .contains_key(&ResultViewer::Smith),
            "idle frame {frame} cleared the pinned readout"
        );
    }
}

// -----------------------------------------------------------------
// marker ownership
// -----------------------------------------------------------------

/// Project the fixture's one pane, exactly as a drawn frame does, and
/// hand back the pane so the test can keep transacting against it.
fn projected_pane(state: &mut AppState, document_id: ResultDocumentId) -> PaneProjection {
    let mut projected = projection(state, document_id).expect("document projection");
    let mut page = projected.pages.remove(0);
    let pane = page.panes.remove(0);
    select_pane_binding(state, &pane).expect("pane binding");
    project_pane_presentation(state, &pane, ResultViewer::Waves).expect("presentation projects");
    pane
}

fn active_analysis_anchor(
    state: &mut AppState,
) -> (
    super::super::AnalysisPresentationKey,
    super::super::WaveformPresentationKey,
) {
    super::super::waves::source_waveform_anchor(state, "V(out)")
        .expect("the fixture retains V(out)")
}

/// The fixture's one trace's X samples, as the drawn pane knows them.
static FIXTURE_SAMPLES: [f64; 3] = [0.0, 0.5, 1.0];

fn placement(
    analysis: super::super::AnalysisPresentationKey,
    anchor: &super::super::WaveformPresentationKey,
    trace_name: &str,
    x: f64,
) -> super::super::MarkerPlacement<'static> {
    super::super::MarkerPlacement {
        analysis,
        anchor: anchor.clone(),
        trace_name: trace_name.to_owned(),
        x,
        samples: &FIXTURE_SAMPLES,
    }
}

fn retained_markers(app: &RSpiceApp, document_id: ResultDocumentId) -> Vec<Marker> {
    app.state
        .workspace
        .visualization_document(document_id)
        .expect("retained document")
        .markers()
        .to_vec()
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
            .workspace
            .visualization_document(document_id)
            .expect("retained document");
        (document.traces()[0].id, document.revision())
    };
    app.state
        .workspace
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
        app.state.ui.results.markers.is_empty(),
        "a retained document marker leaked into the project's quick-view list: {:?}",
        app.state
            .ui
            .results
            .markers
            .iter()
            .map(|marker| (marker.id, marker.note.clone()))
            .collect::<Vec<_>>()
    );
    assert_eq!(app.state.ui.results.document_markers.len(), 1);
    assert_eq!(app.state.ui.results.document_markers[0].note, "retained");
    assert_eq!(app.state.ui.results.document_markers[0].pane_id, pane.id);
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
        .workspace
        .visualization_document(document_id)
        .expect("retained document")
        .revision();

    let selector =
        super::super::place_marker(&mut app.state, placement(analysis, &anchor, "V(out)", 0.5))
            .expect("the placement resolves a store");

    let super::super::MarkerSelector::Document {
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
            .workspace
            .visualization_document(document_id)
            .expect("retained document")
            .revision(),
        revision_before,
        "retaining a marker advances the document revision"
    );
    assert!(
        app.state.ui.results.markers.is_empty(),
        "a document marker must never enter the project's quick-view list"
    );
    // It is addressable in the frame it was placed in, not one frame later.
    assert!(app.state.ui.results.document_marker(marker_id).is_some());
}

/// (h.7) A document cannot retain a marker on a trace it does not own, so
/// the click still lands — on the dataset — and says so.
#[test]
fn a_trace_the_document_does_not_retain_falls_back_to_a_quick_marker() {
    let (mut app, document_id) = persistent_transient_fixture();
    let _pane = projected_pane(&mut app.state, document_id);
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);

    let selector = super::super::place_marker(
        &mut app.state,
        placement(analysis, &anchor, "V(a)+V(b)", 0.25),
    )
    .expect("the placement resolves a store");

    assert!(matches!(selector, super::super::MarkerSelector::Quick(_)));
    assert_eq!(app.state.ui.results.markers.len(), 1);
    assert!(retained_markers(&app, document_id).is_empty());
    assert!(
        app.state
            .log_buffer
            .entries()
            .any(|entry| entry.message.contains("not a retained trace")),
        "the fallback has to be stated, not silent"
    );
}

/// (h.1) Quick markers are a fact about the dataset. Opening, drawing and
/// leaving a project-owned document must not adopt, rewrite or drop them.
#[test]
fn quick_markers_are_untouched_by_opening_and_leaving_a_persistent_document() {
    let (mut app, document_id) = persistent_transient_fixture();
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);
    let quick =
        app.state
            .ui
            .results
            .add_marker(analysis, anchor.clone(), "V(out)".to_owned(), 0.75);
    if let Some(marker) = app.state.ui.results.marker_mut(quick) {
        marker.note = "dataset note".to_owned();
    }
    let pane = projected_pane(&mut app.state, document_id);
    super::super::place_marker(&mut app.state, placement(analysis, &anchor, "V(out)", 0.5))
        .expect("the document retains its own marker");

    for _ in 0..3 {
        drive_frame(|ui| show(ui, &mut app, document_id));
    }

    assert_eq!(app.state.ui.results.markers.len(), 1);
    assert_eq!(app.state.ui.results.markers[0].id, quick);
    assert_eq!(app.state.ui.results.markers[0].note, "dataset note");
    assert_eq!(
        app.state.ui.results.document_markers.len(),
        1,
        "the drawn pane's retained markers project into the overlay"
    );
    assert_eq!(app.state.ui.results.document_markers[0].pane_id, pane.id);

    // Back on a quick surface the overlay is gone and the quick marker is
    // exactly what it was.
    drive_frame(|ui| super::super::show_compact_split(ui, &mut app));
    assert!(app.state.ui.results.document_markers.is_empty());
    assert!(app.state.ui.results.persistent_pane_context.is_none());
    assert_eq!(app.state.ui.results.markers.len(), 1);
    assert_eq!(app.state.ui.results.markers[0].note, "dataset note");
}

/// (h.8) The Studio stage embeds the renderer against the global
/// projection, so it clears the pane context and its overlay too.
#[test]
fn the_studio_stage_clears_the_persistent_pane_context_and_overlay() {
    let (mut app, document_id) = persistent_transient_fixture();
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);
    let _pane = projected_pane(&mut app.state, document_id);
    super::super::place_marker(&mut app.state, placement(analysis, &anchor, "V(out)", 0.5))
        .expect("the document retains its own marker");
    assert!(!app.state.ui.results.document_markers.is_empty());

    drive_frame(|ui| {
        super::super::show_embedded_with_sample_selection(ui, &mut app, None);
    });

    assert!(app.state.ui.results.persistent_pane_context.is_none());
    assert!(app.state.ui.results.document_markers.is_empty());
}

/// (h.2, h.3) The two stores never contend: a document serial cannot
/// advance the quick allocator, and the project save carries exactly the
/// quick markers.
#[test]
fn document_marker_serials_never_reach_the_project_save_or_the_quick_allocator() {
    let (mut app, document_id) = persistent_transient_fixture();
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);
    let _pane = projected_pane(&mut app.state, document_id);
    let selector =
        super::super::place_marker(&mut app.state, placement(analysis, &anchor, "V(out)", 0.5))
            .expect("the document retains its own marker");
    let super::super::MarkerSelector::Document { marker_id, .. } = selector else {
        panic!("a persistent pane retains its own markers");
    };

    let quick = app
        .state
        .ui
        .results
        .add_marker(analysis, anchor, "V(out)".to_owned(), 0.9);

    assert_eq!(
        quick,
        1,
        "the quick allocator counts quick markers alone, whatever serial \
         the document handed its own marker ({})",
        marker_id.get()
    );
    assert_eq!(
        app.state.ui.results.markers.len(),
        1,
        "the project's saved marker list is exactly the quick markers"
    );
    assert_eq!(app.state.ui.results.markers[0].id, quick);
}

/// (h.5) Apply routes by the store the dialog opened on, even when a quick
/// id and a document serial are the same number — the case the old shared
/// `u32` space could not tell apart.
#[test]
fn the_marker_dialog_applies_to_the_store_it_opened_on() {
    let (mut app, document_id) = persistent_transient_fixture();
    let (analysis, anchor) = active_analysis_anchor(&mut app.state);
    let _pane = projected_pane(&mut app.state, document_id);
    let selector =
        super::super::place_marker(&mut app.state, placement(analysis, &anchor, "V(out)", 0.5))
            .expect("the document retains its own marker");
    let super::super::MarkerSelector::Document { marker_id, .. } = selector else {
        panic!("a persistent pane retains its own markers");
    };
    // Give the quick store a marker whose id is the document serial, so
    // the two identities collide as integers and can only be told apart
    // by the store they name.
    let colliding = u32::try_from(marker_id.get()).expect("a small test serial");
    app.state
        .ui
        .results
        .adopt_markers(vec![super::super::ResultMarker {
            id: colliding,
            analysis,
            anchor,
            trace_name: "V(out)".to_owned(),
            x: 0.9,
            kind: super::super::MarkerKind::Note,
            note: "quick".to_owned(),
        }]);

    super::super::commit_marker_edit(
        &mut app.state,
        selector,
        "retained",
        super::super::MarkerKind::Peak,
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
        app.state.ui.results.markers[0].note, "quick",
        "the quick marker that shares the number must be untouched"
    );
    assert_eq!(
        app.state.ui.results.markers[0].kind,
        super::super::MarkerKind::Note
    );

    super::super::commit_marker_edit(
        &mut app.state,
        super::super::MarkerSelector::Quick(colliding),
        "edited quick",
        super::super::MarkerKind::Spec,
    )
    .expect("the quick edit commits");

    assert_eq!(app.state.ui.results.markers[0].note, "edited quick");
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
    let document_selector =
        super::super::place_marker(&mut app.state, placement(analysis, &anchor, "V(out)", 0.5))
            .expect("the document retains its own marker");
    let quick = app
        .state
        .ui
        .results
        .add_marker(analysis, anchor, "V(out)".to_owned(), 0.9);
    let revision_before = app
        .state
        .workspace
        .visualization_document(document_id)
        .expect("retained document")
        .revision();

    super::super::remove_marker(&mut app.state, super::super::MarkerSelector::Quick(quick));

    assert!(app.state.ui.results.markers.is_empty());
    assert_eq!(retained_markers(&app, document_id).len(), 1);
    assert_eq!(
        app.state
            .workspace
            .visualization_document(document_id)
            .expect("retained document")
            .revision(),
        revision_before,
        "removing a quick marker is not a change to the document"
    );

    super::super::remove_marker(&mut app.state, document_selector);

    assert!(retained_markers(&app, document_id).is_empty());
    assert!(app.state.ui.results.document_markers.is_empty());
    assert_ne!(
        app.state
            .workspace
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
    super::super::place_marker(&mut app.state, placement(analysis, &anchor, "V(out)", 0.5))
        .expect("the document retains its own marker");
    let marker_id = retained_markers(&app, document_id)[0].id;
    super::super::commit_marker_edit(
        &mut app.state,
        super::super::MarkerSelector::Document {
            document_id,
            pane_id: _pane.id,
            marker_id,
        },
        "overshoot",
        super::super::MarkerKind::Peak,
    )
    .expect("the document edit commits");

    let duplicate = super::super::ResultMarker {
        id: 4,
        analysis,
        anchor: anchor.clone(),
        trace_name: "V(out)".to_owned(),
        x: 0.5,
        kind: super::super::MarkerKind::Peak,
        note: "overshoot".to_owned(),
    };
    let genuine = super::super::ResultMarker {
        id: 5,
        analysis,
        anchor,
        trace_name: "V(out)".to_owned(),
        x: 0.9,
        kind: super::super::MarkerKind::Note,
        note: "settling".to_owned(),
    };

    super::super::restore_markers(&mut app.state, vec![duplicate, genuine]);

    assert_eq!(app.state.ui.results.markers.len(), 1);
    assert_eq!(app.state.ui.results.markers[0].note, "settling");
}

#[test]
fn projection_carries_every_document_owned_pane_entity() {
    let (app, document_id) = persistent_transient_fixture();
    let projected = projection(&app.state, document_id).expect("document projection");
    let pane = &projected.pages[0].panes[0];
    let document = app
        .state
        .workspace
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
    let mut projected = projection(&app.state, document_id).expect("document projection");
    let mut page = projected.pages.remove(0);
    let pane = page.panes.remove(0);
    select_pane_binding(&mut app.state, &pane).expect("pane binding");
    project_pane_presentation(&mut app.state, &pane, ResultViewer::Waves)
        .expect("presentation projects");
    let retained_default = app.state.simulation.runs[0].analyses[0].waveforms[0].visible;

    super::super::waves::toggle_visibility(&mut app.state, 0, 0);
    assert_eq!(
        app.state.simulation.runs[0].analyses[0].waveforms[0].visible,
        retained_default
    );
    assert!(
        !app.state
            .workspace
            .visualization_document(document_id)
            .expect("document remains retained")
            .traces()[0]
            .visible
    );

    let analysis = super::super::AnalysisPresentationKey::new(
        app.state.simulation.runs[0].dataset_id,
        &app.state.simulation.runs[0].analyses[0],
    );
    let view = app
        .state
        .ui
        .results
        .analysis_plot_view_pane_mut(ResultViewer::Waves, analysis, 0);
    view.x = Some((0.2, 0.8));
    view.y = Some((-0.25, 1.25));
    app.state.ui.results.cursors.a = Some(0.5);
    capture_pane_presentation(&mut app.state, &pane, ResultViewer::Waves);

    let document = app
        .state
        .workspace
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
    assert!(app.state.workspace.visualization_documents_dirty);
}

fn completed_prepared_run(
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
    let receipt = PreparedRunReceipt::new(
        AnalysisResultSourceDomain::SimulationPlan,
        Some(plan_id),
        project_revision,
        digest(0x41),
        source_digest,
        PreparedSourceCheckReceipt::SchematicDrc(digest(0x43)),
        vec![task],
    )
    .expect("run receipt");
    let mut run = SimulationRun::new(1);
    run.restore_provenance(SimulationRunProvenance::Prepared(Box::new(receipt)))
        .expect("run provenance");
    run.mark_running().expect("running lifecycle");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("completed lifecycle");
    run
}

// -----------------------------------------------------------------
// per-document selection and lifecycle
// -----------------------------------------------------------------

/// Which pane a reader is working in is a fact about one document.
///
/// A single global selection meant activating a pane in one document
/// selected the pane with the same serial in every other open one, and
/// coming back to a document forgot where the reader had been.
#[test]
fn pane_selection_belongs_to_the_document_it_was_made_in() {
    let (mut app, first) = persistent_transient_fixture();
    app.state.workbench.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name: "Second transient review".to_owned(),
        name_touched: true,
        dataset_id: Some(app.state.simulation.runs[0].dataset_id),
        family_id: "waveform-worksheet".to_owned(),
        viewer_id: "viewer-waveform".to_owned(),
        layout_id: "two-linked-panes".to_owned(),
        validation_error: None,
    };
    let second =
        super::super::create_document::commit(&mut app).expect("a second document commits");

    drive_frame(|ui| show(ui, &mut app, first));
    let first_pane = app
        .state
        .ui
        .results
        .persistent_document_pane(first)
        .expect("the first document selected a pane");

    // The reader works in the second pane of the two-pane document.
    let second_panes: Vec<PaneId> = app
        .state
        .workspace
        .visualization_document(second)
        .expect("the second document is retained")
        .panes()
        .iter()
        .map(|pane| pane.id)
        .collect();
    assert_eq!(second_panes.len(), 2);
    let working_pane = second_panes[1];
    app.state
        .ui
        .results
        .select_persistent_document_pane(second, working_pane);
    drive_frame(|ui| show(ui, &mut app, second));
    assert_eq!(
        app.state.ui.results.persistent_document_pane(second),
        Some(working_pane)
    );
    assert_eq!(
        app.state.ui.results.persistent_document_pane(first),
        Some(first_pane),
        "drawing another document must not restate this one's selection"
    );

    // Coming back to each finds the reader where they were, rather than
    // wherever the other document last left one global selection.
    drive_frame(|ui| show(ui, &mut app, first));
    assert_eq!(
        app.state.workbench.visualization_studio.active_pane,
        Some(first_pane.get())
    );
    drive_frame(|ui| show(ui, &mut app, second));
    assert_eq!(
        app.state.workbench.visualization_studio.active_pane,
        Some(working_pane.get()),
        "the second document went back to the pane it was left on"
    );
}

/// (7) Closing a document and reopening it holds the reader's place: the
/// trace they selected, the readout they pinned, and the page and pane
/// they were on.
#[test]
fn closing_and_reopening_a_document_holds_the_readers_place() {
    let (mut app, document_id) = persistent_transient_fixture();
    drive_frame(|ui| show(ui, &mut app, document_id));
    let pane = app
        .state
        .ui
        .results
        .persistent_document_pane(document_id)
        .expect("the document selected a pane");
    let page = app
        .state
        .ui
        .results
        .persistent_document_page(document_id)
        .expect("the document selected a page");
    let analysis = super::super::AnalysisPresentationKey::new(
        app.state.simulation.runs[0].dataset_id,
        &app.state.simulation.runs[0].analyses[0],
    );
    app.state.ui.results.selected_trace = Some(super::super::SelectedResultTrace::from_identity(
        analysis, "V(out)",
    ));
    app.state
        .ui
        .results
        .rf_pin
        .insert(ResultViewer::Smith, (0, 3));

    // Close: the reader goes back to the dataset quick view, which leaves
    // the persistent projection behind.
    drive_frame(|ui| super::super::show_compact_split(ui, &mut app));
    assert!(app.state.ui.results.persistent_pane_context.is_none());

    // Re-open, and hold it open.
    for frame in 0..3 {
        drive_frame(|ui| show(ui, &mut app, document_id));
        assert!(
            app.state.ui.results.selected_trace.is_some(),
            "frame {frame} after re-opening lost the selected trace"
        );
        assert!(
            app.state
                .ui
                .results
                .rf_pin
                .contains_key(&ResultViewer::Smith),
            "frame {frame} after re-opening lost the pinned readout"
        );
    }
    assert_eq!(
        app.state.ui.results.persistent_document_pane(document_id),
        Some(pane)
    );
    assert_eq!(
        app.state.ui.results.persistent_document_page(document_id),
        Some(page)
    );
}

// -----------------------------------------------------------------
// per-pane viewport retention
// -----------------------------------------------------------------

/// A retained pane holds one vertical axis while the waveform stack draws
/// a unit pane per quantity. Zooming the amps pane of a V+I document must
/// land on the amps pane, and must not be written into the document's one
/// vertical axis in place of the volts pane the document actually states.
#[test]
fn zooming_one_unit_pane_of_a_document_never_restates_another_pane_axis() {
    let (mut app, document_id) = persistent_transient_fixture();
    let pane = projected_pane(&mut app.state, document_id);
    let analysis = super::super::AnalysisPresentationKey::new(
        app.state.simulation.runs[0].dataset_id,
        &app.state.simulation.runs[0].analyses[0],
    );

    // The volts pane — the one the document's vertical axis states.
    let volts = app
        .state
        .ui
        .results
        .analysis_plot_view_pane_mut(ResultViewer::Waves, analysis, 0);
    volts.x = Some((0.2, 0.8));
    volts.y = Some((-1.0, 1.0));
    // The amps pane, whose scale the document cannot also state.
    let amps = app
        .state
        .ui
        .results
        .analysis_plot_view_pane_mut(ResultViewer::Waves, analysis, 1);
    amps.y = Some((-5.0e-3, 5.0e-3));

    capture_pane_presentation(&mut app.state, &pane, ResultViewer::Waves);

    let vertical = app
        .state
        .workspace
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
            .ui
            .results
            .analysis_plot_view_pane(ResultViewer::Waves, analysis, 1)
            .y,
        Some((-5.0e-3, 5.0e-3))
    );
    assert_eq!(
        app.state
            .ui
            .results
            .analysis_plot_view_pane(ResultViewer::Waves, analysis, 0)
            .y,
        Some((-1.0, 1.0)),
        "the retained pane's own window comes back from the document"
    );
}

// -----------------------------------------------------------------
// Latest tracking
// -----------------------------------------------------------------

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
            .with_waveforms(vec![crate::state::WaveformData::new(
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
    app.state.simulation.netlist_content = "* latest tracking\nV1 out 0 1\n.end\n".to_owned();
    let input_digest = digest(0x60);
    app.state.ui.netlist.generation_error = None;
    app.state.ui.netlist.generated_input_digest = Some(input_digest);
    app.state.ui.netlist.current_generation_input_digest = Some(input_digest);
    let source_digest = current_result_source_digest(&app.state)
        .expect("the fixture states a current authored source");

    let plan_id = SimulationPlanId::new();
    let authored_analysis_id = AnalysisInstanceId::new();
    let project_revision = app.state.workspace.project.revision();
    let run = authored_run(
        1,
        plan_id,
        project_revision,
        source_digest,
        authored_analysis_id,
        "V(out)",
    );
    let dataset_id = run.dataset_id;
    app.state.simulation.runs = vec![run];
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));
    app.state.workbench.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name: "Latest transient review".to_owned(),
        name_touched: true,
        dataset_id: Some(dataset_id),
        family_id: "waveform-worksheet".to_owned(),
        viewer_id: "viewer-waveform".to_owned(),
        layout_id: "single-pane".to_owned(),
        validation_error: None,
    };
    let document_id =
        super::super::create_document::commit(&mut app).expect("persistent document commits");
    let revision = app
        .state
        .workspace
        .visualization_document(document_id)
        .expect("retained document")
        .revision();
    app.state
        .workspace
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
    let good_dataset = app.state.simulation.runs[0].dataset_id;
    let project_revision = app.state.workspace.project.revision();
    // The newest run of the same authored analysis no longer carries the
    // signal this document's traces name.
    app.state.simulation.runs.push(authored_run(
        2,
        plan_id,
        project_revision,
        source_digest,
        authored_analysis_id,
        "V(renamed)",
    ));
    let stale_dataset = app.state.simulation.runs[1].dataset_id;
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
        .ui
        .results
        .persistent_pane_context
        .expect("the document still projects a pane");
    assert_eq!(context.document_id, document_id);
    assert_eq!(
        app.state
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
    let project_revision = app.state.workspace.project.revision();
    app.state.simulation.runs.push(authored_run(
        2,
        plan_id,
        project_revision,
        source_digest,
        authored_analysis_id,
        "V(renamed)",
    ));
    let stale_dataset = app.state.simulation.runs[1].dataset_id;

    assert!(matches!(
        refresh_latest_binding(&mut app.state, document_id),
        LatestBinding::Degraded(_)
    ));
    let first = app
        .state
        .ui
        .results
        .latest_retarget_failure(document_id, stale_dataset)
        .expect("the refusal is held against the candidate that caused it")
        .to_owned();
    let revision_after_first = app
        .state
        .workspace
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
            .ui
            .results
            .latest_retarget_failure(document_id, stale_dataset),
        Some(first.as_str())
    );
    assert_eq!(
        app.state
            .workspace
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
            .ui
            .results
            .latest_retarget_failure(document_id, crate::product::DatasetId::new()),
        None
    );
}

#[test]
fn latest_candidate_requires_completed_success_and_current_project_source_authority() {
    let plan_id = SimulationPlanId::new();
    let revision = ObjectRevision::INITIAL;
    let source = digest(0x52);
    let mut run = completed_prepared_run(plan_id, revision, source);

    assert!(run_matches_current_authority(
        &run,
        plan_id,
        revision,
        Some(source)
    ));
    assert!(!run_matches_current_authority(
        &run,
        plan_id,
        ObjectRevision::new(revision.get() + 1).expect("next revision"),
        Some(source)
    ));
    assert!(!run_matches_current_authority(
        &run,
        plan_id,
        revision,
        Some(digest(0x53))
    ));
    assert!(!run_matches_current_authority(
        &run, plan_id, revision, None
    ));

    run.success = false;
    assert!(!run_matches_current_authority(
        &run,
        plan_id,
        revision,
        Some(source)
    ));
    run.success = true;
    run.lifecycle = SimulationRunLifecycle::Running;
    assert!(!run_matches_current_authority(
        &run,
        plan_id,
        revision,
        Some(source)
    ));
}

#[test]
fn latest_authored_analysis_uses_the_final_expanded_execution_identity() {
    let authored = AnalysisInstanceId::new();
    let first_execution = AnalysisInstanceId::new();
    let final_execution = AnalysisInstanceId::new();
    let mut run = SimulationRun::new(1);
    for (label, execution) in [
        ("PVT point 1/2", first_execution),
        ("PVT point 2/2", final_execution),
    ] {
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, label).with_provenance(
                AnalysisResultProvenance::new_with_authored_source_domain(
                    AnalysisResultSourceDomain::SimulationPlan,
                    execution,
                    authored,
                    ObjectRevision::INITIAL,
                    digest(0x61),
                    Vec::new(),
                )
                .expect("expanded analysis provenance"),
            ),
        );
    }

    let selected = latest_successful_authored_analysis(&run, authored)
        .expect("the authored analysis has expanded results");
    assert_eq!(selected.label, "PVT point 2/2");
    assert_eq!(analysis_identity(&run, selected), final_execution);
}

fn assert_slots_are_finite_and_bounded(stage: Rect, slots: &[Rect], expected: usize) {
    assert_eq!(slots.len(), expected);
    for slot in slots {
        for value in [
            slot.min.x,
            slot.min.y,
            slot.max.x,
            slot.max.y,
            slot.width(),
            slot.height(),
        ] {
            assert!(value.is_finite(), "pane slot must remain finite: {slot:?}");
        }
        assert!(slot.width() >= 0.0 && slot.height() >= 0.0, "{slot:?}");
        assert!(
            slot.left() >= stage.left() && slot.top() >= stage.top(),
            "{slot:?}"
        );
        assert!(
            slot.right() <= stage.right() && slot.bottom() <= stage.bottom(),
            "{slot:?} exceeds {stage:?}"
        );
    }
}

#[test]
fn multi_pane_slots_never_exceed_narrow_or_short_result_stages() {
    let short = Rect::from_min_size(pos2(17.0, 23.0), vec2(480.0, 19.0));
    let rows = bounded_pane_slots(short, PageLayout::Rows, 7);
    assert_slots_are_finite_and_bounded(short, &rows, 7);

    let narrow = Rect::from_min_size(pos2(3.0, 5.0), vec2(13.0, 360.0));
    let columns = bounded_pane_slots(narrow, PageLayout::Columns, 8);
    assert_slots_are_finite_and_bounded(narrow, &columns, 8);

    let compact = Rect::from_min_size(pos2(11.0, 13.0), vec2(29.0, 17.0));
    let grid = bounded_pane_slots(compact, PageLayout::Grid { columns: 3 }, 11);
    assert_slots_are_finite_and_bounded(compact, &grid, 11);
}

#[test]
fn pane_slot_geometry_sanitizes_nonfinite_available_extents() {
    let size = finite_stage_size(vec2(f32::INFINITY, f32::NAN));
    assert_eq!(size, vec2(0.0, 0.0));
    let stage = Rect::from_min_size(pos2(0.0, 0.0), size);
    let slots = bounded_pane_slots(stage, PageLayout::Grid { columns: 2 }, 4);
    assert_slots_are_finite_and_bounded(stage, &slots, 4);
}

#[test]
fn exact_renderer_mapping_exposes_only_implemented_catalog_viewers() {
    assert_eq!(
        ResultViewer::from_viewer_document_id("viewer-waveform"),
        Some(ResultViewer::Waves)
    );
    assert_eq!(
        ResultViewer::from_viewer_document_id("viewer-table"),
        Some(ResultViewer::Table)
    );
    assert_eq!(
        ResultViewer::from_viewer_document_id("viewer-phase-noise"),
        Some(ResultViewer::PhaseNoise)
    );
    assert_eq!(
        ResultViewer::from_viewer_document_id("viewer-manifest"),
        None
    );
    assert_eq!(ResultViewer::from_viewer_document_id("manifest"), None);
    assert_eq!(
        ResultViewer::from_viewer_document_id("field-viewer-3d"),
        None
    );
}

#[test]
fn every_release_target_has_an_exact_native_renderer_identity() {
    let release_targets = crate::results::viewer_catalog::VIEWER_DOCUMENTS
        .iter()
        .filter(|viewer| viewer.release == ViewerReleaseClass::ReleaseTarget)
        .collect::<Vec<_>>();
    assert_eq!(release_targets.len(), 14);
    for viewer in release_targets {
        let native = ResultViewer::from_viewer_document_id(viewer.id);
        assert!(
            native.is_some(),
            "release-target viewer {} has no native Results renderer",
            viewer.id
        );
        assert_eq!(
            native.and_then(ResultViewer::viewer_document_id),
            Some(viewer.id),
            "release-target viewer {} does not round-trip its canonical identity",
            viewer.id
        );
    }
}

#[test]
fn active_pane_identity_is_retained_only_when_it_belongs_to_the_page() {
    assert_eq!(resolved_active_pane_id(Some(7), [3_u64, 7, 11]), Some(7));
    assert_eq!(resolved_active_pane_id(Some(99), [3_u64, 7, 11]), Some(3));
    assert_eq!(resolved_active_pane_id(None, [3_u64, 7, 11]), Some(3));
    assert_eq!(resolved_active_pane_id(Some(7), []), None);
}

#[test]
fn inactive_panes_use_retained_viewers_and_never_the_global_viewer() {
    assert_eq!(
        pane_viewer(true, Some(ResultViewer::Eye), "viewer-waveform"),
        Some(ResultViewer::Eye)
    );
    assert_eq!(
        pane_viewer(false, Some(ResultViewer::Eye), "viewer-waveform"),
        Some(ResultViewer::Waves)
    );
    assert_eq!(
        pane_viewer(false, Some(ResultViewer::Eye), "viewer-manifest"),
        None
    );
}

#[test]
fn retained_frequency_document_restores_its_noise_projection() {
    let noise = AnalysisResult::new(3, AnalysisType::Noise, "noise").with_waveforms(vec![
        crate::state::WaveformData::new("inoise", vec![1.0, 10.0], vec![1.0e-9, 2.0e-9], "#fff"),
    ]);
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(noise);
    state.simulation.runs = vec![run];
    assert!(state.simulation.select_run(0));

    assert_eq!(
        bound_viewer_projection(&state, ResultViewer::Bode),
        ResultViewer::NoiseContrib
    );
}

#[test]
fn interactive_result_viewers_keep_canonical_document_identity() {
    for viewer in [
        ResultViewer::Waves,
        ResultViewer::DcSweep,
        ResultViewer::Bode,
        ResultViewer::Fft,
        ResultViewer::HarmonicBalance,
        ResultViewer::PhaseNoise,
        ResultViewer::Eye,
        ResultViewer::Hist,
        ResultViewer::Op,
        ResultViewer::NoiseContrib,
        ResultViewer::Contribution,
        ResultViewer::TransferFunction,
        ResultViewer::Specs,
        ResultViewer::Table,
        ResultViewer::Nyquist,
        ResultViewer::Smith,
        ResultViewer::PoleZero,
        ResultViewer::Events,
        ResultViewer::Soa,
        ResultViewer::Reliability,
        ResultViewer::Optimization,
    ] {
        let document_id = viewer
            .viewer_document_id()
            .expect("interactive viewers have catalog identities");
        assert!(
            viewer_document(document_id).is_some(),
            "{viewer:?} mapped to unknown canonical document {document_id}"
        );
    }
}

#[test]
fn persistent_renderer_contract_never_substitutes_a_broader_catalog_mode() {
    let transient = AnalysisResult::new(1, AnalysisType::Transient, "tran").with_waveforms(vec![
        crate::state::WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
    ]);
    let hb = AnalysisResult::new(2, AnalysisType::HarmonicBalance, "hb").with_waveforms(vec![
        crate::state::WaveformData::new("V(out)", vec![1.0, 2.0], vec![1.0, 0.5], "#fff"),
    ]);
    let noise = AnalysisResult::new(3, AnalysisType::Noise, "noise").with_waveforms(vec![
        crate::state::WaveformData::new("onoise", vec![1.0, 10.0], vec![1.0e-18, 1.0e-19], "#fff"),
    ]);
    let invalid_noise = AnalysisResult::new(4, AnalysisType::Noise, "invalid noise")
        .with_waveforms(vec![crate::state::WaveformData::new(
            "onoise",
            vec![1.0, 10.0],
            vec![1.0e-18, -1.0e-19],
            "#fff",
        )]);

    assert!(renderer_supports_analysis("viewer-spectrum", &transient));
    assert!(!renderer_supports_analysis("viewer-spectrum", &hb));
    assert!(renderer_supports_analysis("viewer-bode", &noise));
    assert!(!renderer_supports_analysis("viewer-bode", &invalid_noise));
    assert!(!renderer_supports_analysis("viewer-phase-noise", &noise));
}
