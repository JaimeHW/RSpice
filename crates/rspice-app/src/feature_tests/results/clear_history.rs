//! Clear result history… through a whole application: it asks first, and
//! keeps what a project relies on unless told otherwise.

use std::collections::{HashMap, HashSet};

use crate::workbench::RSpiceApp;
use rspice_app_results::workbench::documents::result_document::create_document;
use rspice_app_results::workbench::results::commands::ResultsCommand;
use rspice_app_state::product::DatasetId;
use rspice_app_state::state::{
    AnalysisResult, AnalysisType, RetainedTrace, RunRetention, SimulationRun,
    SimulationRunLifecycle,
};
use rspice_results_ui::create_document::CreateResultDocumentDialogState;

fn completed_run(id: u64) -> SimulationRun {
    let mut run = SimulationRun::new(id);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            RetainedTrace::new(
                "V(out)",
                vec![0.0, 0.5, 1.0],
                vec![0.0, 1.0, 0.0],
                "#55aaff",
            ),
        ]),
    );
    run
}

/// Draw one frame of the whole application, and say where each labelled
/// control landed.
fn frame(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    events: Vec<egui::Event>,
) -> HashMap<String, egui::Rect> {
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1440.0, 900.0),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            app.prepare_frame(ui.ctx());
            app.render_frame_chrome(ui);
            app.render_frame_dialogs(ui.ctx());
        },
    );
    let mut controls = HashMap::new();
    if let Some(update) = output.platform_output.accesskit_update {
        for (_, node) in update.nodes {
            if let (Some(label), Some(bounds)) = (node.label(), node.bounds()) {
                controls.insert(
                    label.to_owned(),
                    egui::Rect::from_min_max(
                        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                    ),
                );
            }
        }
    }
    controls
}

fn click(ctx: &egui::Context, app: &mut RSpiceApp, at: egui::Rect) {
    let pos = at.center();
    frame(
        ctx,
        app,
        vec![
            egui::Event::PointerMoved(pos),
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    frame(
        ctx,
        app,
        vec![egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
}

fn retained(app: &RSpiceApp) -> HashSet<DatasetId> {
    app.state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .map(|run| run.dataset_id)
        .collect()
}

/// One click of Clear result history deleted every run, and nothing could
/// bring them back. It now opens a dialog that counts the runs; deleting
/// from it keeps the pinned golden baseline and the run a saved result
/// document reads.
#[test]
fn clearing_keeps_a_pinned_run_and_a_run_a_saved_document_reads() {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    let runs = (1..=4).rev().map(completed_run).collect::<Vec<_>>();
    let pinned = runs[1].clone();
    let read = runs[2].clone();
    app.state.model.simulation.retained.runs = runs.into();
    assert!(app.state.model.simulation.select_run(0));
    assert!(
        app.state
            .model
            .simulation
            .set_run_retention(pinned.run_id, RunRetention::GoldenBaseline)
    );
    app.state.results.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name: "Saved review".to_owned(),
        name_touched: true,
        dataset_id: Some(read.dataset_id),
        family_id: "waveform-worksheet".to_owned(),
        viewer_id: "viewer-waveform".to_owned(),
        layout_id: "single-pane".to_owned(),
        validation_error: None,
    };
    create_document::commit(&mut app.state.results_parts()).expect("the document commits");
    app.state.results.create_result_document.open = false;

    app.execute_command_key(ResultsCommand::ClearResults.key());
    assert!(
        app.state.results.clear_history.is_some(),
        "the command asks first"
    );
    assert_eq!(
        app.state.model.simulation.retained.runs.len(),
        4,
        "and deletes nothing yet"
    );

    frame(&ctx, &mut app, Vec::new());
    frame(&ctx, &mut app, Vec::new());
    let controls = frame(&ctx, &mut app, Vec::new());
    let delete = *controls
        .get("Delete 2 runs")
        .unwrap_or_else(|| panic!("the dialog offers to delete the two others: {controls:?}"));
    click(&ctx, &mut app, delete);

    assert!(
        app.state.results.clear_history.is_none(),
        "deleting closes the dialog"
    );
    assert_eq!(
        retained(&app),
        HashSet::from([pinned.dataset_id, read.dataset_id])
    );
}

/// Cleared runs used to be gone for good. They now wait until the project
/// closes, and Clear result history offers them back.
#[test]
fn clearing_then_restoring_brings_back_every_run_with_its_samples() {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    let runs = (1..=3).rev().map(completed_run).collect::<Vec<_>>();
    let digests: Vec<_> = runs
        .iter()
        .map(SimulationRun::dataset_content_digest)
        .collect();
    app.state.model.simulation.retained.runs = runs.into();
    assert!(app.state.model.simulation.select_run(0));

    app.execute_command_key(ResultsCommand::ClearResults.key());
    frame(&ctx, &mut app, Vec::new());
    frame(&ctx, &mut app, Vec::new());
    let controls = frame(&ctx, &mut app, Vec::new());
    assert!(
        !controls.keys().any(|label| label.starts_with("Restore")),
        "nothing to restore yet: {controls:?}"
    );
    click(&ctx, &mut app, controls["Delete 3 runs"]);
    assert!(app.state.model.simulation.retained.runs.is_empty());

    app.execute_command_key(ResultsCommand::ClearResults.key());
    frame(&ctx, &mut app, Vec::new());
    frame(&ctx, &mut app, Vec::new());
    let controls = frame(&ctx, &mut app, Vec::new());
    let restore = *controls
        .get("Restore 3 deleted runs")
        .unwrap_or_else(|| panic!("the dialog offers the cleared runs back: {controls:?}"));
    click(&ctx, &mut app, restore);

    assert!(app.state.results.clear_history.is_none());
    let back: Vec<_> = app
        .state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .map(SimulationRun::dataset_content_digest)
        .collect();
    assert_eq!(
        back, digests,
        "every run is back, newest first, with its samples"
    );
    assert!(app.state.model.simulation.active_run().is_some());
    assert!(app.state.model.simulation.retained.deleted_runs.is_empty());
}
