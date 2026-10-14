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
