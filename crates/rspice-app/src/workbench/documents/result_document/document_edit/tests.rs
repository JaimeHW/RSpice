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

#[test]
fn the_last_pane_on_a_page_is_not_removable() {
    let (mut app, document_id) = document_fixture("single-pane");
    let pane = ordered_panes(&app, document_id)[0].id;
    assert!(!pane_menu(&app.state, document_id, pane).removable);
    remove_pane(&mut app.state, document_id, pane);
    assert_eq!(ordered_panes(&app, document_id).len(), 1);
}
