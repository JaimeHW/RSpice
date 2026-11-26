//! A retained run as the reader handles it in Results: picked in the data
//! browser and shown under the strip's first tab, stepped through, and kept
//! across a save and reopen.

use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use rspice_app_results::workbench::chrome::document_bar::results::{result_documents, show_run};
use rspice_app_results::workbench::documents::result_document::{
    AnalysisPresentationKey, ResultViewer, working_view,
};
use rspice_app_runs::simulation::controller::SimulationController;
use rspice_app_runs::simulation::controller::test_execution::{
    run_generated_batches, state_with_current_clean_drc,
};
use rspice_app_state::product::{DatasetId, ResultDocumentId};
use rspice_app_state::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRunLifecycle,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::chrome::document_bar::{
    activate_document, close_all_documents, close_document, close_other_documents, cycle_document,
    document_descriptors, visible_documents,
};
use rspice_app_workbench::workbench::state::workbench_session::WorkspaceDocumentId;
use rspice_simulation_contract::plan_catalog::SimulationPlanCloneOptions;

const TIME: [f64; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];

/// Run the session's active plan once, its transient answering `out` over
/// `time`, and finish the run, its samples held in memory.
fn run_plan_over(state: &mut AppState, time: [f64; 5], out: [f64; 5]) {
    let mut controller = SimulationController::new();
    controller
        .preflight_run_set(state)
        .expect("clean plan preflight");
    state.model.simulation.execution.request_simulate_run_set();
    controller.start_simulation(state);
    let provenance = controller
        .current_provenance
        .clone()
        .expect("the plan's prepared task");
    state
        .model
        .simulation
        .active_run_mut()
        .expect("the plan's run")
        .add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
                .with_provenance(provenance)
                .with_waveforms(vec![
                    RetainedTrace::new("V(in)", time.to_vec(), vec![1.0; 5], "#ffffff"),
                    RetainedTrace::new("V(out)", time.to_vec(), out.to_vec(), "#ffffff"),
                ]),
        );
    controller.abort();
    state
        .model
        .simulation
        .active_run_mut()
        .expect("the plan's run")
        .finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("the run finishes");
}

/// Run the session's active plan once over the first second.
fn run_plan(state: &mut AppState, out: [f64; 5]) {
    run_plan_over(state, TIME, out);
}

/// A session with one finished run of its plan, Results in front.
fn session_with_a_run() -> AppState {
    let host = state_with_current_clean_drc();
    let mut state = AppState::default();
    state.model = host.model;
    state.runs = host.runs;
    state.workbench.activate(Workspace::Results);
    run_plan(&mut state, [0.0, 0.4, 1.0, 0.4, 0.0]);
    state
}

/// The datasets of the retained runs, newest first.
fn newest_first(state: &AppState) -> Vec<DatasetId> {
    let mut runs: Vec<_> = state.model.simulation.retained.runs.iter().collect();
    runs.sort_by_key(|run| std::cmp::Reverse(run.id));
    runs.into_iter().map(|run| run.dataset_id).collect()
}

/// Save `state`'s project to a scratch folder and open it again in a new
/// session. The reopened runs read their samples from the results folder
/// beside the project file, so the folder stays while the test holds it.
fn save_and_reopen(mut state: AppState) -> (AppState, ScratchFolder) {
    let root = std::env::temp_dir().join(format!("rspice-runs-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&root).expect("a scratch folder");
    let path = root.join("runs.rspiceproj");
    assert!(state.save_project_to_path(&path), "the project saves");
    drop(state);
    let mut state = AppState::default();
    let reopened = state.load_project_from_path(&path);
    assert!(reopened, "the project reopens");
    (state, ScratchFolder(root))
}

/// A scratch folder removed when the test lets go of it.
struct ScratchFolder(std::path::PathBuf);

impl Drop for ScratchFolder {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_run_saved_from_memory_reopens_with_the_digest_it_had() {
    let state = session_with_a_run();
    let run = &state.model.simulation.retained.runs[0];
    let (dataset, before) = (run.dataset_id, run.dataset_content_digest());

    let (state, _folder) = save_and_reopen(state);
    let run = state
        .model
        .simulation
        .retained
        .run_by_dataset_id(dataset)
        .expect("the run reopens");
    assert_eq!(
        run.dataset_content_digest(),
        before,
        "the same samples have the same digest wherever they are held"
    );
}

/// A run's name and note are the reader's, kept with the project as
/// Assembler's History keeps a run's name and comment; neither is result
/// content, so neither moves the digest.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_renamed_and_annotated_run_reopens_as_the_reader_left_it() {
    let mut state = session_with_a_run();
    let run = state
        .model
        .simulation
        .retained
        .runs
        .first_mut()
        .expect("the run");
    let (dataset, digest) = (run.dataset_id, run.dataset_content_digest());
    let stamp = run.start_stamp().expect("the run's start");
    run.rename("Bias at 1.8 V");
    run.note = "Supply ripple fixed".to_owned();
    assert_eq!(run.label, format!("Bias at 1.8 V ({stamp})"));
    assert_eq!(
        run.dataset_content_digest(),
        digest,
        "a name is not content"
    );

    let (state, _folder) = save_and_reopen(state);
    let run = state
        .model
        .simulation
        .retained
        .run_by_dataset_id(dataset)
        .expect("the run reopens");
    assert_eq!(run.name(), "Bias at 1.8 V");
    assert_eq!(run.note, "Supply ripple fixed");
    assert_eq!(run.dataset_content_digest(), digest);
}

/// A run the engine solved keeps its samples in the store from the start,
/// and the save copies its segments beside the project file.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_solved_run_reopens_with_the_digest_it_had() {
    let host = run_generated_batches(state_with_current_clean_drc(), 1);
    let mut state = AppState::default();
    state.model = host.model;
    state.runs = host.runs;
    let run = &state.model.simulation.retained.runs[0];
    assert!(
        !run.holds_samples_in_memory(),
        "a solved run is in the store"
    );
    let (dataset, before) = (run.dataset_id, run.dataset_content_digest());

    let (state, _folder) = save_and_reopen(state);
    let run = state
        .model
        .simulation
        .retained
        .run_by_dataset_id(dataset)
        .expect("the run reopens");
    assert_eq!(
        run.dataset_content_digest(),
        before,
        "the same samples have the same digest wherever they are held"
    );
}

/// The application on a session, drawn whole, frame after frame.
struct Desk {
    ctx: egui::Context,
    app: RSpiceApp,
    time: f64,
}

impl Desk {
    fn new(state: AppState) -> Self {
        let ctx = egui::Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state = state;
        app.state.workbench.activate(Workspace::Results);
        let mut desk = Self {
            ctx,
            app,
            time: 0.0,
        };
        desk.settle();
        desk
    }

    /// Draw one frame of the whole application with `events`.
    fn frame(&mut self, events: Vec<egui::Event>) -> egui::FullOutput {
        self.time += 0.02;
        let app = &mut self.app;
        self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1600.0, 900.0),
                )),
                time: Some(self.time),
                focused: true,
                events,
                ..Default::default()
            },
            |ui| {
                app.prepare_frame(ui.ctx());
                app.render_frame_chrome(ui);
                app.render_frame_dialogs(ui.ctx());
            },
        )
    }

    fn settle(&mut self) -> egui::FullOutput {
        self.frame(Vec::new());
        self.frame(Vec::new());
        self.frame(Vec::new())
    }

    /// Press `key` and draw the frames it takes.
    fn press(&mut self, key: egui::Key) {
        self.frame(vec![egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }]);
        self.settle();
    }

    /// Pick the run of `dataset` in the data browser, as clicking its row
    /// does, and draw the frames it takes.
    fn pick(&mut self, dataset: DatasetId) {
        show_run(&mut self.app.state.results_parts(), dataset).expect("the run shows");
        self.settle();
    }

    /// The Results tabs: what each shows and what it is called.
    fn tabs(&self) -> Vec<(WorkspaceDocumentId, String)> {
        result_documents(self.app.state.results_reads())
            .into_iter()
            .map(|tab| (tab.id, tab.label))
            .collect()
    }

    /// The document the strip has in front.
    fn front(&self) -> Option<WorkspaceDocumentId> {
        document_descriptors(&self.app.state)
            .into_iter()
            .find(|document| document.active)
            .map(|document| document.id)
    }

    /// The run Results draws.
    fn shown(&self) -> Option<DatasetId> {
        let simulation = &self.app.state.model.simulation;
        simulation.active_run().map(|run| run.dataset_id)
    }

    fn name_of(&self, dataset: DatasetId) -> String {
        let simulation = &self.app.state.model.simulation;
        let run = simulation.retained.run_by_dataset_id(dataset);
        run.expect("a retained run").name().to_owned()
    }
}

/// A second plan, cloned from the first, as Clone plan does, and made the
/// active one.
fn clone_plan(state: &mut AppState, name: &str) {
    let model = &mut state.model;
    let source = model
        .sim_setup
        .stable_analysis_plan()
        .expect("a plan is active")
        .id();
    model.workspace.content.migrate_active_plan_data(source);
    let outcome = model
        .sim_setup
        .clone_active_plan(name, SimulationPlanCloneOptions::default())
        .expect("the plan clones");
    model
        .workspace
        .content
        .clone_plan_data(
            outcome.source_plan_id,
            outcome.cloned_plan_id,
            outcome.contents.copy_variables_outputs_and_specifications,
            outcome.contents.copy_regression_baseline_ownership,
            &outcome.analysis_identity_map,
        )
        .expect("its data clones");
}

/// Two runs of the first plan, the newer over two seconds; a run of a
/// second plan; and the first plan's view saved as a document. The browser
/// shows the first plan's newer run.
struct Bench {
    desk: Desk,
    /// The first plan's runs, older first.
    first: [DatasetId; 2],
    /// The second plan's run.
    other: DatasetId,
    saved: ResultDocumentId,
}

fn bench() -> Bench {
    let mut state = session_with_a_run();
    run_plan_over(
        &mut state,
        [0.0, 0.5, 1.0, 1.5, 2.0],
        [0.0, 0.2, 0.4, 1.0, 0.1],
    );
    let [newer, older] = newest_first(&state)[..] else {
        panic!("two runs");
    };
    let mut desk = Desk::new(state);
    assert_eq!(desk.shown(), Some(newer));
    let view =
        working_view::view_of_run(desk.app.state.results_reads(), newer).expect("the plan's view");
    let saved = working_view::save_as_document(&mut desk.app.state.results_parts(), view, "Bias")
        .expect("the view saves as a document");
    clone_plan(&mut desk.app.state, "Corner");
    run_plan(&mut desk.app.state, [0.0, 0.1, 0.2, 0.3, 0.4]);
    desk.settle();
    let other = newest_first(&desk.app.state)[0];
    Bench {
        desk,
        first: [older, newer],
        other,
        saved,
    }
}

/// Runs are picked in the data browser, not tabbed: two plans' runs and a
/// saved document are the browser's tab and the document's.
#[test]
fn two_plans_runs_and_a_saved_document_take_two_tabs() {
    let Bench {
        desk, first, saved, ..
    } = bench();
    let expected = vec![
        (
            WorkspaceDocumentId::ResultDataset(first[1]),
            desk.name_of(first[1]),
        ),
        (
            WorkspaceDocumentId::VisualizationDocument(saved),
            "Bias".to_owned(),
        ),
    ];
    assert_eq!(desk.tabs(), expected);
    let strip: Vec<_> = visible_documents(&desk.app.state)
        .into_iter()
        .map(|tab| (tab.id, tab.label))
        .collect();
    assert_eq!(strip, expected, "the strip draws the same two");
}

/// A run picked in the browser shows, and the first tab takes its name.
#[test]
fn a_run_picked_in_the_browser_shows_under_the_first_tab() {
    let Bench {
        mut desk, other, ..
    } = bench();
    desk.pick(other);
    assert_eq!(desk.shown(), Some(other));
    assert_eq!(
        desk.front(),
        Some(WorkspaceDocumentId::ResultDataset(other))
    );
    assert_eq!(
        desk.tabs()[0],
        (
            WorkspaceDocumentId::ResultDataset(other),
            desk.name_of(other)
        )
    );
}

/// Stepping to the plan's other run keeps the sheet, the view's panes and
/// the X range, and each cursor the other run's X still reaches.
#[test]
fn stepping_through_a_plans_runs_keeps_the_panes_the_x_range_and_the_cursors() {
    let Bench {
        mut desk, first, ..
    } = bench();
    let [older, newer] = first;
    let analysis = |desk: &Desk| {
        let run = desk.app.state.model.simulation.active_run().expect("a run");
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
    };
    let panes = |desk: &Desk| {
        let reads = desk.app.state.results_reads();
        let view = working_view::view_of_run(reads, desk.shown().expect("a run"))
            .expect("the plan's view");
        let content = &desk.app.state.model.workspace.content;
        let document = content.visualization_document(view);
        let panes = document.expect("the view").panes();
        (view, panes.iter().map(|pane| pane.id).collect::<Vec<_>>())
    };
    let x_range = |desk: &Desk| {
        let session = &desk.app.state.results.results.session;
        session
            .analysis_plot_view_pane(ResultViewer::Waves, analysis(desk), 0)
            .x
    };
    let cursors = |desk: &Desk| {
        let session = &desk.app.state.results.results.session;
        (session.cursors.a, session.cursors.b)
    };
    assert_eq!(desk.shown(), Some(newer));
    let key = analysis(&desk);
    let session = &mut desk.app.state.results.results.session;
    session.viewer = ResultViewer::Waves;
    session
        .analysis_plot_view_pane_mut(ResultViewer::Waves, key, 0)
        .x = Some((0.25, 0.75));
    session.cursors.a = Some(0.5);
    session.cursors.b = Some(1.5);
    session.cursor_strip = Some(0);
    desk.settle();
    let kept = panes(&desk);

    desk.press(egui::Key::PageDown);
    assert_eq!(
        desk.shown(),
        Some(older),
        "Page Down steps to the older run"
    );
    assert_eq!(
        desk.front(),
        Some(WorkspaceDocumentId::ResultDataset(older))
    );
    assert_eq!(
        desk.app.state.results.results.session.viewer,
        ResultViewer::Waves
    );
    assert_eq!(panes(&desk), kept);
    assert_eq!(x_range(&desk), Some((0.25, 0.75)));
    assert_eq!(
        cursors(&desk),
        (Some(0.5), None),
        "B at 1.5 s is past the older run's first second"
    );

    desk.press(egui::Key::PageUp);
    assert_eq!(desk.shown(), Some(newer), "Page Up steps back");
    assert_eq!(panes(&desk), kept);
    assert_eq!(x_range(&desk), Some((0.25, 0.75)));
    assert_eq!(cursors(&desk), (Some(0.5), None));
}

/// With a saved document in front, a run picked in the browser brings the
/// browser's tab forward on it; the browser's tab, chosen again, goes back
/// to the run it showed.
#[test]
fn a_run_picked_over_a_saved_document_shows_under_the_browsers_tab() {
    let Bench {
        mut desk,
        first,
        saved,
        ..
    } = bench();
    let document = WorkspaceDocumentId::VisualizationDocument(saved);
    assert!(activate_document(&mut desk.app.state, &document));
    desk.settle();
    assert_eq!(desk.front(), Some(document.clone()));

    let browser = WorkspaceDocumentId::ResultDataset(first[1]);
    assert!(activate_document(&mut desk.app.state, &browser));
    desk.settle();
    assert_eq!(
        desk.front(),
        Some(browser),
        "the browser's tab is the way back"
    );
    assert_eq!(desk.shown(), Some(first[1]));

    assert!(activate_document(&mut desk.app.state, &document));
    desk.settle();
    desk.pick(first[0]);
    assert_eq!(
        desk.front(),
        Some(WorkspaceDocumentId::ResultDataset(first[0]))
    );
    assert_eq!(desk.shown(), Some(first[0]));
}

/// The browser's tab has no close control and no close takes it; closing
/// saved documents' tabs only hides them.
#[test]
fn the_browsers_tab_never_closes() {
    let Bench {
        mut desk,
        first,
        saved,
        ..
    } = bench();
    let browser = WorkspaceDocumentId::ResultDataset(first[1]);
    let document = WorkspaceDocumentId::VisualizationDocument(saved);
    let closable: Vec<_> = document_descriptors(&desk.app.state)
        .into_iter()
        .map(|tab| (tab.id, tab.closable))
        .collect();
    assert_eq!(
        closable,
        vec![(browser.clone(), false), (document.clone(), true)]
    );
    let output = desk.settle();
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("accessibility tree")
        .nodes;
    let label = desk.name_of(first[1]);
    let tab = nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| {
            node.role() == egui::accesskit::Role::Tab && node.label() == Some(label.as_str())
        })
        .expect("the browser's tab is drawn");
    assert!(
        !tab.description().unwrap_or_default().contains("close"),
        "the tab offers no close: {:?}",
        tab.description()
    );

    let strip = visible_documents(&desk.app.state);
    assert!(!close_document(&mut desk.app.state, &browser, &strip));
    assert_eq!(close_all_documents(&mut desk.app.state), 1);
    desk.settle();
    assert_eq!(desk.tabs()[0].0, browser);
    assert_eq!(desk.front(), Some(browser.clone()));
    assert_eq!(visible_documents(&desk.app.state).len(), 1);

    assert!(activate_document(&mut desk.app.state, &document));
    desk.settle();
    assert!(cycle_document(&mut desk.app.state, false));
    assert_eq!(desk.front(), Some(browser));
    assert_eq!(close_other_documents(&mut desk.app.state), 1);
    let content = &desk.app.state.model.workspace.content;
    assert!(
        content.visualization_document(saved).is_some(),
        "closing a tab keeps its document"
    );
    assert_eq!(desk.app.state.model.simulation.retained.runs.len(), 3);
}

/// A finished run of the plan shown shows, unless the reader picked an
/// older run of it.
#[test]
fn a_new_run_of_the_plan_shown_shows_unless_an_older_run_was_picked() {
    let mut desk = Desk::new(session_with_a_run());
    let first = desk.shown().expect("the first run");
    run_plan(&mut desk.app.state, [0.0, 0.2, 0.4, 1.0, 0.1]);
    desk.settle();
    let second = newest_first(&desk.app.state)[0];
    assert_eq!(desk.shown(), Some(second), "the new run shows");
    assert_eq!(desk.tabs()[0].0, WorkspaceDocumentId::ResultDataset(second));

    desk.pick(first);
    run_plan(&mut desk.app.state, [0.0, 0.1, 0.2, 0.3, 0.4]);
    desk.settle();
    let third = newest_first(&desk.app.state)[0];
    assert_ne!(third, second);
    assert_eq!(desk.shown(), Some(first), "the run picked stays");
    assert_eq!(desk.tabs()[0].0, WorkspaceDocumentId::ResultDataset(first));

    desk.pick(third);
    run_plan(&mut desk.app.state, [0.0, 0.3, 0.6, 0.9, 1.0]);
    desk.settle();
    let fourth = newest_first(&desk.app.state)[0];
    assert_eq!(
        desk.shown(),
        Some(fourth),
        "the newest run picked, the plan's next run shows"
    );
}
