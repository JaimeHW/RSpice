//! The Simulation Studio over the whole application: a committed workflow
//! answers on the status bar, a restored session starts from the default
//! lifecycle outcome, and a refusal restated on every frame is one event.

use crate::ui::widgets::DialogChoice;
use crate::workbench::RSpiceApp;
use crate::workbench::state::ClonePlanDraft;
use crate::workbench::surfaces::simulate::show_on_stage;
use crate::workbench::surfaces::simulate::workflow_dialog_tests::show_workflow_dialogs_on_stage;
use crate::workbench::surfaces::simulate::workflows::commit_clone_plan;
use crate::workbench::surfaces::simulate::workflows::finish_workflow_choice;
use egui::Rect;
use egui::vec2;

/// A committed plan edit is answered on the status bar: the reader made the
/// gesture and is looking at its result, so nothing is toasted.
#[test]
fn a_committed_simulation_workflow_answers_on_the_status_bar() {
    let mut app = RSpiceApp::test_instance();
    let mut draft = ClonePlanDraft::for_source(
        app.state
            .model
            .sim_setup
            .stable_analysis_plan()
            .unwrap()
            .id(),
        app.state.model.sim_setup.active_plan_name().as_str(),
    );
    draft.name = "Committed clone".to_owned();

    finish_workflow_choice(
        &mut crate::workbench::features::cx(&mut app),
        DialogChoice::Primary,
        draft,
        commit_clone_plan,
    );
    app.state.drain_notices(&egui::Context::default());

    assert!(app.state.simulate.simulation_workflow.is_none());
    assert!(app.state.prompt_text().is_some());
    assert!(app.state.workbench.toasts.activity().is_empty());
}

/// Run one frame: the studio on `page`, or the overlay host alone when there
/// is no page because the reader is not on this workspace at all.
fn simulate_frame(app: &mut RSpiceApp, page: Option<crate::state::SimulationPage>) {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    if let Some(page) = page {
        app.state.simulate.simulation_page = page;
    }
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1_280.0, 900.0))),
            ..egui::RawInput::default()
        },
        |ctx| {
            if page.is_some() {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| show_on_stage(ui, app));
            }
            show_workflow_dialogs_on_stage(ctx, app);
        },
    );
}

/// The session wire does not keep the field, so a restore starts from its
/// `Default`.
///
/// A restore must not carry a receipt from the session that wrote it — a
/// lifecycle outcome is an event, and a restored one would announce a change
/// nobody just made.
#[test]
fn a_restored_session_starts_from_the_default_lifecycle_outcome() {
    use crate::workbench::AppState;
    use crate::workbench::app_state::model::AnalysisLifecycleOutcome;
    use crate::workbench::app_state::session::wire::{WorkbenchWire, WorkbenchWireRef};

    assert!(!AnalysisLifecycleOutcome::default().message().is_empty());

    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .analysis_lifecycle_status
        .record_receipt("Receipt #9 · Insert committed for instance 5.");
    let saved = serde_json::to_value(WorkbenchWireRef::capture(&app.state))
        .expect("the workbench serializes");
    let mut restored = AppState::default();
    serde_json::from_value::<WorkbenchWire>(saved)
        .expect("the workbench restores")
        .scatter(&mut restored);

    assert_eq!(
        restored.model.analysis_lifecycle_status,
        AnalysisLifecycleOutcome::default(),
        "the outcome is runtime-only, so a restore starts from the default line"
    );
}

/// Three announcing sites sit on the render path rather than in a click
/// handler: the surface resolves the active instance, the editor resolves the
/// selection, and the editor re-serializes the draft, each of them every frame.
/// A plan that stays unavailable therefore restates the same refusal forever,
/// and that must stay one event.
#[test]
fn a_render_path_refusal_restated_every_frame_is_one_event() {
    use crate::state::SimulationPage;

    let mut app = RSpiceApp::test_instance();
    app.state.model.sim_setup.analysis_plan = None;
    for _ in 0..4 {
        simulate_frame(&mut app, Some(SimulationPage::Analyses));
    }
    app.state.drain_notices(&egui::Context::default());

    let outcome = &app.state.model.analysis_lifecycle_status;
    assert!(outcome.is_refusal(), "{}", outcome.message());
    assert!(
        app.state.workbench.toasts.activity().is_empty(),
        "a refusal raises no notice"
    );
    assert_eq!(
        app.state
            .model
            .log_buffer
            .entries()
            .filter(|entry| entry.source == crate::diagnostics::LogSource::Plan)
            .count(),
        1,
        "and one Console record, not one per frame"
    );
}
