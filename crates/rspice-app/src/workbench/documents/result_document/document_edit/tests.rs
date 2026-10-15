//! Pins for the edits the result viewer makes to a project result document.

use super::*;
use crate::results::visualization_document::{LinkKind, Pane};
use crate::state::{AnalysisType, SimulationRun, SimulationRunLifecycle, WaveformData};
use rspice_results_ui::create_document::CreateResultDocumentDialogState;

fn transient_run(id: u64) -> SimulationRun {
    let mut run = SimulationRun::new(id);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 0.5, 1.0], vec![0.0, 1.0, 0.0], "#fff"),
            WaveformData::new("V(in)", vec![0.0, 0.5, 1.0], vec![1.0, 0.0, 1.0], "#0ff"),
        ]),
    );
    run
}

/// A project result document over one retained transient, created through
/// the same commit the Create dialog uses, and open in the Results workspace.
fn document_fixture(layout_id: &str) -> (RSpiceApp, ResultDocumentId) {
    let mut app = RSpiceApp::test_instance();
    let run = transient_run(1);
    let dataset_id = run.dataset_id;
    app.state.simulation.retained.runs = vec![run].into();
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));
    app.state.workbench.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name: "Transient review".to_owned(),
        name_touched: true,
        dataset_id: Some(dataset_id),
        family_id: "waveform-worksheet".to_owned(),
        viewer_id: "viewer-waveform".to_owned(),
        layout_id: layout_id.to_owned(),
        validation_error: None,
    };
    let document_id =
        super::super::create_document::commit(&mut app).expect("the document commits");
    (app, document_id)
}

fn retained(app: &RSpiceApp, document_id: ResultDocumentId) -> &VisualizationDocument {
    app.state
        .workspace
        .content
        .visualization_document(document_id)
        .expect("the project retains the document")
}

fn ordered_panes(app: &RSpiceApp, document_id: ResultDocumentId) -> Vec<Pane> {
    let document = retained(app, document_id);
    let page = &document.pages()[0];
    super::super::persistent_document::ordered_page_panes(document.panes(), page)
        .into_iter()
        .cloned()
        .collect()
}

/// Drive one whole frame with the product theme applied.
fn drive_frame(body: impl FnMut(&mut egui::Ui)) {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut body = body;
    let _ = ctx.run_ui(egui::RawInput::default(), |ui| body(ui));
}

fn show_document(app: &mut RSpiceApp, document_id: ResultDocumentId) {
    drive_frame(|ui| super::super::show_persistent_document(ui, app, document_id));
}

fn commit_add_pane(app: &mut RSpiceApp) {
    open_add_pane(&mut app.state);
    let Some(ResultDocumentDialog::AddPane(mut draft)) =
        app.state.workbench.result_document_dialog.clone()
    else {
        panic!("Add pane opens its form");
    };
    let choices = add_pane_choices(&app.state, &mut draft);
    assert_eq!(choices.refusal, None);
    app.state.workbench.result_document_dialog = Some(ResultDocumentDialog::AddPane(draft));
    commit_open_dialog(&mut app.state);
    assert!(
        app.state.workbench.result_document_dialog.is_none(),
        "a landed Add closes its form"
    );
}

#[test]
fn adding_a_pane_binds_it_after_the_selected_pane_and_selects_it() {
    let (mut app, document_id) = document_fixture("single-pane");
    show_document(&mut app, document_id);
    let anchor = ordered_panes(&app, document_id)[0].clone();

    commit_add_pane(&mut app);

    let panes = ordered_panes(&app, document_id);
    assert_eq!(panes.len(), 2);
    assert_eq!(panes[0].id, anchor.id);
    assert_eq!(panes[1].viewer_id, "viewer-waveform");
    assert_eq!(panes[1].binding, anchor.binding);
    // A single-pane page cannot hold a second pane, so it becomes rows.
    assert_eq!(
        retained(&app, document_id).pages()[0].layout,
        PageLayout::Rows
    );
    assert_eq!(
        app.state
            .ui
            .results
            .session
            .persistent_document_pane(document_id),
        Some(panes[1].id)
    );
    // The new pane draws on the next frame without disturbing the document.
    show_document(&mut app, document_id);
    assert_eq!(ordered_panes(&app, document_id).len(), 2);
}

#[test]
fn a_pane_from_another_run_attaches_that_run_to_the_document() {
    let (mut app, document_id) = document_fixture("single-pane");
    let second = transient_run(2);
    let second_dataset = second.dataset_id;
    app.state.simulation.retained.runs.push(second);
    show_document(&mut app, document_id);

    open_add_pane(&mut app.state);
    let Some(ResultDocumentDialog::AddPane(mut draft)) =
        app.state.workbench.result_document_dialog.clone()
    else {
        panic!("Add pane opens its form");
    };
    draft.dataset = Some(second_dataset);
    draft.analysis = None;
    let choices = add_pane_choices(&app.state, &mut draft);
    assert_eq!(choices.datasets.len(), 2);
    assert_eq!(choices.refusal, None);
    app.state.workbench.result_document_dialog = Some(ResultDocumentDialog::AddPane(draft));
    commit_open_dialog(&mut app.state);

    let document = retained(&app, document_id);
    assert!(
        document
            .datasets()
            .iter()
            .any(|dataset| dataset.binding().dataset_id == second_dataset),
        "the second run is attached"
    );
    let panes = ordered_panes(&app, document_id);
    assert_eq!(
        panes[1].binding.map(|binding| binding.dataset.dataset_id),
        Some(second_dataset)
    );
}

#[test]
fn the_add_pane_form_draws_over_the_document() {
    let (mut app, document_id) = document_fixture("two-linked-panes");
    show_document(&mut app, document_id);
    open_add_pane(&mut app.state);
    drive_frame(|ui| show(ui.ctx(), &mut app));
    assert!(matches!(
        app.state.workbench.result_document_dialog,
        Some(ResultDocumentDialog::AddPane(_))
    ));
}

#[test]
fn add_pane_is_offered_only_over_a_project_result_document() {
    let mut app = RSpiceApp::test_instance();
    app.state.simulation.retained.runs = vec![transient_run(1)].into();
    assert!(!add_pane_available(&app.state));
    open_add_pane(&mut app.state);
    assert!(app.state.workbench.result_document_dialog.is_none());
}

fn horizontal_link_members(app: &RSpiceApp, document_id: ResultDocumentId) -> Vec<Vec<EntityRef>> {
    retained(app, document_id)
        .link_groups()
        .iter()
        .filter(|group| group.kind == LinkKind::HorizontalViewport)
        .map(|group| group.members.clone())
        .collect()
}

#[test]
fn removing_a_linked_pane_takes_it_out_of_its_link_group() {
    let (mut app, document_id) = document_fixture("two-linked-panes");
    show_document(&mut app, document_id);
    // The new pane follows a linked pane, so it joins that pane's X group.
    commit_add_pane(&mut app);
    assert_eq!(horizontal_link_members(&app, document_id)[0].len(), 3);
    let panes = ordered_panes(&app, document_id);

    remove_pane(&mut app.state, document_id, panes[2].id);
    assert_eq!(ordered_panes(&app, document_id).len(), 2);
    assert_eq!(horizontal_link_members(&app, document_id)[0].len(), 2);

    // A group cannot link one axis, so it goes with the second-last member.
    remove_pane(&mut app.state, document_id, panes[1].id);
    assert_eq!(ordered_panes(&app, document_id).len(), 1);
    assert!(horizontal_link_members(&app, document_id).is_empty());
    show_document(&mut app, document_id);
}

/// Save the document and load it back, as a project round trip does.
fn reload(app: &mut RSpiceApp, document_id: ResultDocumentId) {
    let documents = &mut app.state.workspace.content.visualization_documents;
    let index = documents
        .iter()
        .position(|document| document.id() == document_id)
        .expect("the project retains the document");
    let saved = serde_json::to_string(&documents[index]).expect("the document saves");
    documents[index] = serde_json::from_str(&saved).expect("the document loads");
}

/// The fixture run's one analysis, as a pane binding.
fn fixture_binding(app: &RSpiceApp) -> PaneDataBinding {
    let run = &app.state.simulation.retained.runs[0];
    PaneDataBinding {
        analysis_id: super::super::create_document::source_analysis_identity(run, &run.analyses[0]),
        dataset: DatasetBinding::new(run.dataset_id, run.dataset_content_digest()),
    }
}

/// A saved document can hold pages after the first; the Studio authored
/// them. The result viewer showed only the first page and offered no way to
/// another, so such a page was unreachable.
#[test]
fn a_saved_document_shows_the_second_page_once_it_is_picked() {
    use crate::results::visualization_document::{
        NewPage, NewPagePane, PageUpdatePolicy, PaneKind,
    };
    let (mut app, document_id) = document_fixture("single-pane");
    let binding = fixture_binding(&app);
    transact(
        &mut app.state,
        document_id,
        vec![DocumentEdit::AddPaneOnNewPage {
            page: NewPage {
                title: "Corners".to_owned(),
                layout: PageLayout::Rows,
                template_id: "engineering-dark".to_owned(),
                update_policy: PageUpdatePolicy::RefreshLinkedFigures,
            },
            pane: NewPagePane {
                title: "Corner waveform".to_owned(),
                kind: PaneKind::Cartesian,
                viewer_id: "viewer-waveform".to_owned(),
                binding: Some(binding),
            },
        }],
    )
    .expect("a second page commits");
    reload(&mut app, document_id);
    show_document(&mut app, document_id);
    let document = retained(&app, document_id);
    let second_page = document.pages()[1].id;
    let second_pane = document
        .panes()
        .iter()
        .find(|pane| pane.page_id == second_page)
        .expect("the second page holds a pane")
        .id;
    let session = &app.state.ui.results.session;
    assert_ne!(
        session.persistent_document_page(document_id),
        Some(second_page)
    );

    super::super::persistent_document::select_page(&mut app.state, document_id, 1);
    show_document(&mut app, document_id);

    let session = &app.state.ui.results.session;
    assert_eq!(
        session.persistent_document_page(document_id),
        Some(second_page)
    );
    assert_eq!(
        session.persistent_document_pane(document_id),
        Some(second_pane)
    );
}

/// A pane moved earlier keeps its new place through a save and reload, and
/// the menu stops offering moves past either end of the page.
#[test]
fn a_moved_pane_keeps_its_place_in_the_saved_document() {
    let (mut app, document_id) = document_fixture("two-linked-panes");
    show_document(&mut app, document_id);
    let before = ordered_panes(&app, document_id);
    let (first, second) = (before[0].id, before[1].id);
    let menu = pane_menu(&app.state, document_id, second);
    assert!(menu.earlier && !menu.later);

    move_pane(&mut app.state, document_id, second, PaneMove::Earlier);
    reload(&mut app, document_id);
    show_document(&mut app, document_id);

    let after = ordered_panes(&app, document_id)
        .iter()
        .map(|pane| pane.id)
        .collect::<Vec<_>>();
    assert_eq!(after, vec![second, first]);
    let menu = pane_menu(&app.state, document_id, second);
    assert!(!menu.earlier && menu.later);
    let revision = retained(&app, document_id).revision();
    move_pane(&mut app.state, document_id, second, PaneMove::Earlier);
    assert_eq!(
        retained(&app, document_id).revision(),
        revision,
        "the first pane cannot move earlier"
    );
}

#[test]
fn the_last_pane_on_a_page_is_not_removable() {
    let (mut app, document_id) = document_fixture("single-pane");
    let pane = ordered_panes(&app, document_id)[0].id;
    assert!(!pane_menu(&app.state, document_id, pane).removable);
    remove_pane(&mut app.state, document_id, pane);
    assert_eq!(ordered_panes(&app, document_id).len(), 1);
}

// ---------------------------------------------------------------------------
// Compare datasets
// ---------------------------------------------------------------------------

/// A second transient on the fixture's axis whose V(out) peaks 0.2 V higher.
fn baseline_run(id: u64) -> SimulationRun {
    let mut run = SimulationRun::new(id);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 0.5, 1.0], vec![0.0, 1.2, 0.0], "#fff"),
            WaveformData::new("V(in)", vec![0.0, 0.5, 1.0], vec![1.0, 0.0, 1.0], "#0ff"),
        ]),
    );
    run
}

/// Open Compare datasets, compare by X value within 100 mV, and run it.
fn run_comparison(
    app: &mut RSpiceApp,
) -> rspice_results_ui::document_dialogs::compare::CompareOutcome {
    app.state.workbench.workspace = Workspace::Results;
    assert!(compare_available(&app.state));
    open_compare(&mut app.state);
    let Some(ResultDocumentDialog::Compare(mut draft)) =
        app.state.workbench.result_document_dialog.clone()
    else {
        panic!("Compare datasets opens its form");
    };
    draft.alignment = rspice_results::waveform_comparison::ComparisonAlignmentDraft::AbsoluteXAxis;
    draft.absolute_tolerance = "100m".to_owned();
    let choices = compare::choices(&app.state, &mut draft);
    assert_eq!(choices.refusal, None);
    app.state.workbench.result_document_dialog = Some(ResultDocumentDialog::Compare(draft));
    commit_open_dialog(&mut app.state);
    let Some(ResultDocumentDialog::Compare(draft)) =
        app.state.workbench.result_document_dialog.clone()
    else {
        panic!("the form stays open on its outcome");
    };
    draft.outcome.expect("the comparison ran")
}

fn verdict_of(
    outcome: &rspice_results_ui::document_dialogs::compare::CompareOutcome,
    name: &str,
) -> rspice_results_ui::document_dialogs::compare::Verdict {
    outcome
        .signals
        .iter()
        .find(|signal| signal.name == name)
        .unwrap_or_else(|| panic!("the outcome names {name}"))
        .verdict
}

/// Compare datasets over a document records the receipt with it and
/// attaches the baseline, and both survive a save. The Studio kept this;
/// the result viewer could not compare at all.
#[test]
fn a_comparison_over_a_document_is_kept_with_it() {
    use rspice_results_ui::document_dialogs::compare::Verdict;
    let (mut app, document_id) = document_fixture("single-pane");
    let baseline = baseline_run(2);
    let baseline_id = baseline.dataset_id;
    app.state.simulation.retained.runs.push(baseline);
    show_document(&mut app, document_id);

    let outcome = run_comparison(&mut app);
    assert_eq!(outcome.rows_compared, 3);
    assert_eq!(verdict_of(&outcome, "V(out)"), Verdict::Fail);
    assert_eq!(verdict_of(&outcome, "V(in)"), Verdict::Pass);

    reload(&mut app, document_id);
    let document = retained(&app, document_id);
    assert_eq!(document.comparisons().len(), 1);
    assert_eq!(document.comparisons()[0].baseline.dataset_id, baseline_id);
    assert!(
        document
            .datasets()
            .iter()
            .any(|dataset| dataset.binding().dataset_id == baseline_id),
        "the baseline is attached with the receipt"
    );
    show_document(&mut app, document_id);

    // Done closes the outcome.
    commit_open_dialog(&mut app.state);
    assert!(app.state.workbench.result_document_dialog.is_none());
}

/// Over a quick view the outcome shows and nothing is kept: no document
/// appears and neither dataset changes.
#[test]
fn a_comparison_over_a_quick_view_keeps_nothing() {
    let mut app = RSpiceApp::test_instance();
    app.state.simulation.retained.runs = vec![transient_run(1), baseline_run(2)].into();
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));
    let digests = app
        .state
        .simulation
        .retained
        .runs
        .iter()
        .map(SimulationRun::dataset_content_digest)
        .collect::<Vec<_>>();

    let outcome = run_comparison(&mut app);
    assert_eq!(outcome.signals.len(), 2);
    assert!(
        app.state
            .workspace
            .content
            .visualization_documents
            .is_empty()
    );
    assert_eq!(
        app.state
            .simulation
            .retained
            .runs
            .iter()
            .map(SimulationRun::dataset_content_digest)
            .collect::<Vec<_>>(),
        digests
    );
}

/// With no retained run that holds the same analysis on the same axis,
/// Compare datasets is unavailable and opens nothing.
#[test]
fn compare_is_refused_without_a_compatible_baseline() {
    let mut app = RSpiceApp::test_instance();
    let mut incompatible = SimulationRun::new(2);
    incompatible.lifecycle = SimulationRunLifecycle::Completed;
    incompatible.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            WaveformData::new(
                "V(other)",
                vec![0.0, 0.25, 1.0],
                vec![0.0, 1.0, 0.0],
                "#fff",
            ),
        ]),
    );
    app.state.simulation.retained.runs = vec![transient_run(1), incompatible].into();
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));
    app.state.workbench.workspace = Workspace::Results;

    assert!(!compare_available(&app.state));
    open_compare(&mut app.state);
    assert!(app.state.workbench.result_document_dialog.is_none());
}

// ---------------------------------------------------------------------------
// Add measurement
// ---------------------------------------------------------------------------

/// Open Add measurement, type `expression`, and return what the form shows.
fn measurement_preview(
    app: &mut RSpiceApp,
    expression: &str,
) -> rspice_results_ui::document_dialogs::measurement::MeasurementPreview {
    app.state.workbench.workspace = Workspace::Results;
    open_measurement(&mut app.state);
    let Some(ResultDocumentDialog::Measurement(mut draft)) =
        app.state.workbench.result_document_dialog.clone()
    else {
        panic!("Add measurement opens its form");
    };
    draft.expression = expression.to_owned();
    let preview = measurement::preview(&app.state, &draft);
    app.state.workbench.result_document_dialog = Some(ResultDocumentDialog::Measurement(draft));
    preview
}

/// A measurement added in the result viewer is kept on the selected pane,
/// with the trace it reads, and survives a save. Only the Studio could add
/// one; the result viewer showed them and could not make them.
#[test]
fn a_measurement_added_to_a_document_is_kept_with_its_pane() {
    let (mut app, document_id) = document_fixture("two-linked-panes");
    show_document(&mut app, document_id);
    let panes = ordered_panes(&app, document_id);
    app.state
        .ui
        .results
        .session
        .select_persistent_document_pane(document_id, panes[1].id);

    let preview = measurement_preview(&mut app, "rms(V(out))");
    assert_eq!(preview.refusal, None);
    assert!(matches!(preview.value, Ok(Some(_))), "{:?}", preview.value);
    commit_open_dialog(&mut app.state);
    assert!(
        app.state.workbench.result_document_dialog.is_none(),
        "a kept measurement closes its form"
    );

    reload(&mut app, document_id);
    let document = retained(&app, document_id);
    let [measurement] = document.measurements() else {
        panic!("one measurement is kept");
    };
    assert_eq!(measurement.pane_id, panes[1].id);
    assert_eq!(measurement.expression.as_deref(), Some("rms(V(out))"));
    // ∫y² over the two linear segments 0 → 1 → 0 on [0, 1] is 1/3.
    let value = measurement.value.expect("the value is kept");
    assert!((value - (1.0_f64 / 3.0).sqrt()).abs() < 1e-14, "{value}");
    let trace = document
        .traces()
        .iter()
        .find(|trace| trace.id == measurement.trace_ids[0])
        .expect("the measurement references a kept trace");
    assert_eq!(
        (trace.pane_id, trace.label.as_str()),
        (panes[1].id, "V(out)")
    );
    show_document(&mut app, document_id);
}

/// An expression that is still a trace, or names a signal the pane does not
/// show, is refused under the field and keeps nothing.
#[test]
fn a_measurement_must_reduce_to_a_number() {
    let (mut app, document_id) = document_fixture("single-pane");
    show_document(&mut app, document_id);
    let preview = measurement_preview(&mut app, "V(out)");
    assert!(
        preview
            .value
            .as_ref()
            .is_err_and(|reason| reason.contains("reduce it")),
        "{:?}",
        preview.value
    );
    commit_open_dialog(&mut app.state);
    assert!(retained(&app, document_id).measurements().is_empty());
}

/// Over a quick view the form still evaluates, so the value can be read,
/// but Add is refused: a quick view has no document to keep it.
#[test]
fn a_measurement_over_a_quick_view_is_evaluated_and_not_kept() {
    let mut app = RSpiceApp::test_instance();
    app.state.simulation.retained.runs = vec![transient_run(1)].into();
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));

    let preview = measurement_preview(&mut app, "max(V(out))");
    assert_eq!(
        preview.refusal.as_deref(),
        Some("Create a result document to keep measurements.")
    );
    assert!(matches!(preview.value, Ok(Some(_))), "{:?}", preview.value);
    commit_open_dialog(&mut app.state);
    assert!(
        matches!(
            app.state.workbench.result_document_dialog,
            Some(ResultDocumentDialog::Measurement(_))
        ),
        "a refused Add leaves the form open"
    );
    assert!(
        app.state
            .workspace
            .content
            .visualization_documents
            .is_empty()
    );
}

// ---------------------------------------------------------------------------
// Family slicing
// ---------------------------------------------------------------------------

const SOA_TIMES: [f64; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];

/// A retained SOA run: a time-domain analysis whose every sample is one
/// point of a family over time, a family the waveform view draws.
fn soa_run(id: u64) -> SimulationRun {
    let mut run = SimulationRun::new(id);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Soa, "SOA")
            .with_waveforms(vec![WaveformData::new(
                "V(out)",
                SOA_TIMES.to_vec(),
                vec![0.0, 1.0, 2.0, 1.0, 0.0],
                "#fff",
            )])
            .with_family_metadata(crate::state::AnalysisResultFamilyMetadata::Soa {
                time: SOA_TIMES.to_vec(),
            }),
    );
    run
}

/// Open Family slicing, narrow it to `filter`, and apply it.
fn apply_family_slice(app: &mut RSpiceApp, filter: &str) {
    app.state.workbench.workspace = Workspace::Results;
    app.state.ui.results.session.viewer = ResultViewer::Waves;
    assert!(family_available(&app.state));
    open_family(&mut app.state);
    let Some(ResultDocumentDialog::Family(mut draft)) =
        app.state.workbench.result_document_dialog.clone()
    else {
        panic!("Family slicing opens its form");
    };
    draft.filter = filter.to_owned();
    let choices = family::choices(&app.state, &draft);
    assert_eq!(choices.refusal, None);
    assert_eq!(choices.points, Ok("3 of 5 points".to_owned()));
    app.state.workbench.result_document_dialog = Some(ResultDocumentDialog::Family(draft));
    commit_open_dialog(&mut app.state);
    assert!(
        app.state.workbench.result_document_dialog.is_none(),
        "an applied slice closes its form"
    );
}

/// The family rows the drawn sheet shows, when it shows a slice.
fn drawn_rows(app: &RSpiceApp) -> Option<Vec<usize>> {
    app.state
        .ui
        .results
        .session
        .sample_selection
        .as_ref()
        .map(|selection| selection.source_indices.clone())
}

/// Over a quick view a family slice is kept for the session, per analysis,
/// and the waveform view draws only the rows it keeps.
#[test]
fn a_family_slice_over_a_quick_view_draws_only_the_rows_it_keeps() {
    let mut app = RSpiceApp::test_instance();
    app.state.simulation.retained.runs = vec![soa_run(1)].into();
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));

    apply_family_slice(&mut app, "time <= 0.5");
    assert_eq!(app.state.ui.results.session.family_policies.len(), 1);
    drive_frame(|ui| super::super::show(ui, &mut app));
    assert_eq!(drawn_rows(&app), Some(vec![0, 1, 2]));
}

/// A project result document over `run` whose copy of the result keeps each
/// row's time and sample, the dimensions a family slice is kept over.
fn family_document(app: &mut RSpiceApp) -> ResultDocumentId {
    use crate::results::visualization_document::{
        ColumnRole, SourceColumn, SourceDataset, SourceRow, TypedValue, ValueType,
    };
    let run = &app.state.simulation.retained.runs[0];
    let binding = DatasetBinding::new(run.dataset_id, run.dataset_content_digest());
    let analysis_id =
        super::super::create_document::source_analysis_identity(run, &run.analyses[0]);
    let column = |key: &str, label: &str, value_type, role, unit: Option<&str>| {
        SourceColumn::new(key, label, value_type, role, unit.map(str::to_owned))
            .expect("the column is valid")
    };
    let columns = vec![
        column(
            "time",
            "Time",
            ValueType::Real,
            ColumnRole::Coordinate,
            Some("s"),
        ),
        column(
            "sample",
            "Sample",
            ValueType::Integer,
            ColumnRole::Coordinate,
            None,
        ),
        column(
            "v(out)",
            "V(out)",
            ValueType::Real,
            ColumnRole::Signal,
            Some("V"),
        ),
    ];
    let rows = SOA_TIMES
        .iter()
        .zip([0.0, 1.0, 2.0, 1.0, 0.0])
        .zip(1_i64..)
        .map(|((time, value), sample)| {
            SourceRow::new(vec![
                TypedValue::Real(*time),
                TypedValue::Integer(sample),
                TypedValue::Real(value),
            ])
        })
        .collect();
    let source = SourceDataset::new(binding, columns, rows).expect("the source is valid");
    let mut document =
        VisualizationDocument::new("SOA review", vec![source]).expect("the document is valid");
    let pane_id = document.panes()[0].id;
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetPaneSource {
                pane_id,
                viewer_id: "viewer-waveform".to_owned(),
                binding: Some(PaneDataBinding {
                    analysis_id,
                    dataset: binding,
                }),
            }],
        )
        .expect("the pane binds the run");
    let document_id = app
        .state
        .workspace
        .content
        .insert_visualization_document(document)
        .expect("the project keeps the document");
    app.state.workbench.activate(Workspace::Results);
    assert!(super::super::activate_persistent_document(
        &mut app.state,
        document_id
    ));
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::VisualizationDocument(document_id));
    document_id
}

/// A family slice kept with a pane survives a save, and the pane draws only
/// the rows it keeps. The result viewer ignored a saved pane's family policy
/// and drew every row.
#[test]
fn a_saved_pane_family_slice_is_drawn_as_saved() {
    let mut app = RSpiceApp::test_instance();
    app.state.simulation.retained.runs = vec![soa_run(1)].into();
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));
    let document_id = family_document(&mut app);
    app.state.ui.results.session.viewer = ResultViewer::Waves;
    show_document(&mut app, document_id);
    assert_eq!(drawn_rows(&app), None);

    apply_family_slice(&mut app, "time <= 0.5");
    reload(&mut app, document_id);
    assert!(
        retained(&app, document_id).panes()[0]
            .family_policy
            .is_some(),
        "the slice is kept with the pane"
    );
    assert!(app.state.ui.results.session.family_policies.is_empty());
    show_document(&mut app, document_id);
    assert_eq!(drawn_rows(&app), Some(vec![0, 1, 2]));
}

/// Drawing a pane's slice changes the session's selection, which clears its
/// cursors. The pane's cursors were projected from the document and have to
/// survive that, or capturing the pane deletes them from the document.
#[test]
fn drawing_a_pane_family_slice_keeps_the_pane_cursors() {
    let mut app = RSpiceApp::test_instance();
    app.state.simulation.retained.runs = vec![soa_run(1)].into();
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));
    let document_id = family_document(&mut app);
    let pane_id = retained(&app, document_id).panes()[0].id;
    let edits = links::cursor_pair_edits(
        retained(&app, document_id),
        pane_id,
        (Some(0.25), Some(0.5)),
    )
    .expect("the pane keeps cursors on its X axis");
    transact(&mut app.state, document_id, edits).expect("the cursors are kept");
    app.state.ui.results.session.viewer = ResultViewer::Waves;
    show_document(&mut app, document_id);

    apply_family_slice(&mut app, "time <= 0.5");
    for _ in 0..2 {
        show_document(&mut app, document_id);
    }
    assert_eq!(drawn_rows(&app), Some(vec![0, 1, 2]));
    let mut kept = retained(&app, document_id)
        .cursors()
        .iter()
        .map(|cursor| (cursor.label.clone(), cursor.position.clone()))
        .collect::<Vec<_>>();
    kept.sort_by(|left, right| left.0.cmp(&right.0));
    assert_eq!(
        kept,
        vec![
            (
                "A".to_owned(),
                crate::results::visualization_document::TypedValue::Real(0.25)
            ),
            (
                "B".to_owned(),
                crate::results::visualization_document::TypedValue::Real(0.5)
            ),
        ]
    );
}

/// A document keeps a slice only over dimensions its copy of the result
/// declares. The documents the result viewer creates declare none, so the
/// form refuses before Apply instead of failing on it.
#[test]
fn a_slice_a_document_cannot_keep_is_refused_before_apply() {
    let mut app = RSpiceApp::test_instance();
    app.state.simulation.retained.runs = vec![soa_run(1)].into();
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));
    let document_id = family_document(&mut app);
    app.state.ui.results.session.viewer = ResultViewer::Waves;
    show_document(&mut app, document_id);
    open_family(&mut app.state);
    let Some(ResultDocumentDialog::Family(mut draft)) =
        app.state.workbench.result_document_dialog.clone()
    else {
        panic!("Family slicing opens its form");
    };
    // Status is not a column of this document's copy of the result.
    draft.filter = "status != not-run".to_owned();
    let choices = family::choices(&app.state, &draft);
    assert_eq!(
        choices.refusal.as_deref(),
        Some("The document keeps no Status values for this pane to slice by.")
    );
}

// ---------------------------------------------------------------------------
// Plot properties
// ---------------------------------------------------------------------------

/// Open Plot properties through its command and set `digits` and `phase`.
fn apply_properties(app: &mut RSpiceApp, digits: u8, phase_continuous: bool) {
    crate::workbench::commands::vocabulary::Command::VisualizationDocumentProperties.execute(app);
    let Some(ResultDocumentDialog::Properties(mut draft)) =
        app.state.workbench.result_document_dialog.clone()
    else {
        panic!("Plot properties opens its form");
    };
    draft.significant_digits = digits;
    draft.phase_continuous = phase_continuous;
    app.state.workbench.result_document_dialog = Some(ResultDocumentDialog::Properties(draft));
    commit_open_dialog(&mut app.state);
    assert!(
        app.state.workbench.result_document_dialog.is_none(),
        "applied properties close their form"
    );
}

fn shown_digits(app: &RSpiceApp) -> u8 {
    app.state
        .ui
        .result_presentation_policy()
        .displayed_significant_digits()
        .get()
}

/// A document's digits and phase are kept with it and applied whenever it
/// is shown, in place of the reader's own, which come back on leaving it.
/// The result viewer ignored both; only the Studio honored them.
#[test]
fn a_document_s_digits_and_phase_are_applied_as_saved() {
    let (mut app, document_id) = document_fixture("single-pane");
    app.state.workbench.workspace = Workspace::Results;
    show_document(&mut app, document_id);
    let own_digits = shown_digits(&app);
    assert_ne!(own_digits, 4);

    apply_properties(&mut app, 4, true);
    reload(&mut app, document_id);
    let presentation = retained(&app, document_id).presentation();
    assert_eq!(
        (
            presentation.significant_digits,
            presentation.phase_continuous
        ),
        (4, true)
    );
    show_document(&mut app, document_id);
    assert_eq!(shown_digits(&app), 4);
    assert!(app.state.ui.results.session.presented_phase_continuous());
    assert!(
        !app.state.ui.results.session.phase_continuous,
        "the reader's own phase setting is left alone"
    );

    drive_frame(|ui| super::super::show(ui, &mut app));
    assert_eq!(shown_digits(&app), own_digits);
    assert!(!app.state.ui.results.session.presented_phase_continuous());
}

/// Over a quick view the properties are the reader's own: the digits
/// preference and the session's phase setting. No document appears.
#[test]
fn quick_view_properties_are_the_reader_s_own() {
    let mut app = RSpiceApp::test_instance();
    app.state.simulation.retained.runs = vec![transient_run(1)].into();
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));
    app.state.workbench.workspace = Workspace::Results;

    apply_properties(&mut app, 5, true);
    assert_eq!(shown_digits(&app), 5);
    assert!(app.state.ui.results.session.phase_continuous);
    assert!(
        app.state
            .workspace
            .content
            .visualization_documents
            .is_empty()
    );
}

// ---------------------------------------------------------------------------
// Review renders
// ---------------------------------------------------------------------------

/// Write one review render of what `pass` paints at `size`, with its measured
/// geometry beside it.
fn review(stem: &str, size: egui::Vec2, mut pass: impl FnMut(&mut egui::Ui)) {
    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    crate::ui::raster::write_panel_review(
        &directory,
        stem,
        crate::ui::Theme::default(),
        size,
        0.0,
        |ui, _| pass(ui),
    );
}

/// The result viewer's document forms and a two-pane linked document, as
/// PNGs with their measured geometry. Run with `--ignored`; the renders go
/// to `RSPICE_RASTER_DIR` (default: the system temp directory).
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_result_document_forms_for_review() {
    let form = egui::vec2(760.0, 560.0);

    let (mut app, document_id) = document_fixture("single-pane");
    show_document(&mut app, document_id);
    measurement_preview(&mut app, "rms(V(out))");
    review("measurement", form, |ui| show(ui.ctx(), &mut app));

    let mut app = RSpiceApp::test_instance();
    app.state.simulation.retained.runs = vec![soa_run(1)].into();
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));
    app.state.workbench.workspace = Workspace::Results;
    app.state.ui.results.session.viewer = ResultViewer::Waves;
    open_family(&mut app.state);
    review("family", form, |ui| show(ui.ctx(), &mut app));

    let (mut app, document_id) = document_fixture("single-pane");
    app.state.workbench.workspace = Workspace::Results;
    show_document(&mut app, document_id);
    open_properties(&mut app.state);
    review("properties", form, |ui| show(ui.ctx(), &mut app));

    let (mut app, document_id) = document_fixture("single-pane");
    app.state.simulation.retained.runs.push(baseline_run(2));
    show_document(&mut app, document_id);
    app.state.workbench.workspace = Workspace::Results;
    open_compare(&mut app.state);
    review("compare", form, |ui| show(ui.ctx(), &mut app));
    run_comparison(&mut app);
    review("compare-outcome", form, |ui| show(ui.ctx(), &mut app));

    let (mut app, document_id) = document_fixture("two-linked-panes");
    app.state.workbench.workspace = Workspace::Results;
    show_document(&mut app, document_id);
    // Cursors come before the link: linking is what makes the panes agree on
    // a position they already have, so a document with none refuses it.
    for pane in ordered_panes(&app, document_id) {
        let edits = links::cursor_pair_edits(
            retained(&app, document_id),
            pane.id,
            (Some(0.25), Some(0.75)),
        )
        .expect("the pane keeps cursors");
        transact(&mut app.state, document_id, edits).expect("the cursors are kept");
    }
    links::set_cursor_links(&mut app.state, document_id, true).expect("the cursors link");
    review("two-pane-linked", egui::vec2(1100.0, 640.0), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            super::super::show_persistent_document(ui, &mut app, document_id);
        });
    });
}
