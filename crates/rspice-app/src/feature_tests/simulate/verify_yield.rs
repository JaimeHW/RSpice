//! The Monte Carlo yield page over the whole application: Edit specifications
//! pressed mid-edit opens on the committed limit through the contribution step,
//! and a restored session and a reopened project state the same yield.

use super::verify_fixtures::monte_carlo_app;
use crate::state::VerificationPage;
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::state::{YieldLimitBound, YieldLimitDraft};
use crate::workbench::surfaces::verify::raster::{page_input, page_pass};
use crate::workbench::surfaces::verify::show;
use crate::workbench::surfaces::verify::tests::yield_histogram::fixture_yield_results;

/// Edit specifications pressed while a limit is being typed: the press takes
/// the cell's focus, so the typed limit is the plan's in that frame, and the
/// specification editor the press opens holds it rather than the limit the
/// plan had before.
#[test]
fn edit_specifications_pressed_mid_edit_opens_on_the_committed_limit() {
    let mut app = monte_carlo_app();
    app.state.simulate.verification_page = VerificationPage::Yield;
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    app.state.simulate.verification.yield_limit_draft = Some(YieldLimitDraft {
        output: "gain".to_owned(),
        bound: YieldLimitBound::Lower,
        text: "41 dB".to_owned(),
        seed: String::new(),
        focus: true,
    });
    let frame = |app: &mut RSpiceApp, events: Vec<egui::Event>| {
        let (_, geometry) = page_pass(&ctx, page_input(events), |ui| {
            show(ui, &mut crate::workbench::features::cx(app));
        });
        app.after_contribution();
        geometry
    };
    // The cell opens and takes the keyboard.
    let button = frame(&mut app, Vec::new()).all("toolbar.action")[0].center();
    let pointer = |pressed| egui::Event::PointerButton {
        pos: button,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    frame(
        &mut app,
        vec![egui::Event::PointerMoved(button), pointer(true)],
    );
    assert!(
        app.state.simulate.verification.yield_limit_draft.is_some(),
        "the cell is still open while the button is held"
    );
    assert!(app.state.results.results.session.spec_drafts.is_none());
    frame(&mut app, vec![pointer(false)]);

    assert_eq!(app.state.simulate.verification.yield_limit_draft, None);
    let drafts = app
        .state
        .results
        .results
        .session
        .spec_drafts
        .clone()
        .expect("the press opened the specification editor");
    let gain = drafts
        .iter()
        .find(|draft| draft.measurement == "gain")
        .expect("the editor holds the gain specification");
    assert!(
        gain.primary_limit.trim().starts_with("41"),
        "the editor opens on the committed limit, not {:?}",
        gain.primary_limit
    );
}

/// Yield is derived, never stored: the session eframe restores, the same
/// session read strictly, and the project saved and reopened each judge the
/// retained trials against the limits they carried, and state the same yield.
#[test]
fn a_restored_session_and_a_reopened_project_state_the_same_yield() {
    struct Stored(String);

    impl eframe::Storage for Stored {
        fn get_string(&self, key: &str) -> Option<String> {
            (key == eframe::APP_KEY).then(|| self.0.clone())
        }

        fn set_string(&mut self, _: &str, _: String) {}

        fn remove_string(&mut self, _: &str) {}

        fn flush(&mut self) {}
    }

    let app = monte_carlo_app();
    let results = fixture_yield_results(&app.state.model);
    assert_eq!((results[0].pass_count, results[0].total_runs), (37, 40));
    let judged = serde_json::to_value(&results).expect("the yield serializes");

    let session = serde_json::to_string(&app.state).expect("the session serializes");
    let restored =
        AppState::restore_eframe_session(&Stored(session.clone())).expect("a stored session");
    let strict =
        serde_json::from_str::<Box<AppState>>(&session).expect("the session reads strictly");
    for (route, mut state) in [("eframe", Box::new(restored)), ("strict", strict)] {
        let model = &mut state.model;
        assert!(
            model
                .simulation
                .yield_results_for_active_dataset()
                .is_none(),
            "{route}: the yield is not stored"
        );
        model
            .simulation
            .refresh_yield_evidence(&model.sim_setup, &model.workspace.content);
        let results = model
            .simulation
            .yield_results_for_active_dataset()
            .expect("the restored run is judged");
        assert_eq!(
            serde_json::to_value(results).expect("the yield serializes"),
            judged,
            "{route}"
        );
    }

    let project =
        rspice_app_services::workbench::lifecycle::project_lifecycle::snapshot(&app.state)
            .expect("the project captures");
    let text = rspice_app_services::io::project_io::serialize_project_file(&project)
        .expect("the project serializes");
    let file = rspice_app_services::io::project_io::load_project_text(&text, None)
        .expect("the project reopens")
        .file;
    let (setup, _, _) = rspice_app_services::io::restore_execution_context(
        file.execution_context.expect("the project saves its plan"),
        file.workspace.project.id(),
    )
    .expect("the plan restores");
    let mut simulation =
        rspice_app_services::io::simulation_state_from_results(file.simulation_results)
            .expect("the runs restore");
    simulation.refresh_yield_evidence(&setup, &file.workspace);
    let results = simulation
        .yield_results_for_active_dataset()
        .expect("the reopened run is judged");
    assert_eq!(
        serde_json::to_value(results).expect("the yield serializes"),
        judged,
        "project file"
    );
}
