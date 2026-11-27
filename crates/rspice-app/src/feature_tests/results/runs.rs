//! A retained run as the reader handles it in Results: picked in the data
//! browser beside the saved documents it lists, stepped through, and kept
//! across a save and reopen.

use super::browser_rows;
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::commands::vocabulary::ShellCommand;
use egui::accesskit::{Action, ActionRequest, Node, NodeId, Role, TreeId};
use rspice_app_results::workbench::chrome::document_bar::results::{show_run, shown_document};
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
use rspice_app_workbench::workbench::framework::command_registry::{CommandAvailability, ids};
use rspice_results_ui::document_dialogs::ResultDocumentDialog;
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

    /// The accessibility tree of a settled frame.
    fn nodes(&mut self) -> Vec<(NodeId, Node)> {
        let output = self.settle();
        output
            .platform_output
            .accesskit_update
            .expect("accessibility tree")
            .nodes
    }

    /// Click `node`, as AccessKit's default action does, and draw the
    /// frames it takes.
    fn click(&mut self, node: NodeId) {
        self.frame(vec![egui::Event::AccessKitActionRequest(ActionRequest {
            action: Action::Click,
            target_tree: TreeId::ROOT,
            target_node: node,
            data: None,
        })]);
        self.settle();
    }

    /// Right-click at `at` with the mouse and draw the frames it takes.
    fn right_click(&mut self, at: egui::Pos2) {
        let button = |pressed| egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Secondary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        self.frame(vec![egui::Event::PointerMoved(at)]);
        self.frame(vec![button(true)]);
        self.frame(vec![button(false)]);
        self.settle();
    }

    /// Choose the button labelled `label`: a menu's item or a form's.
    fn choose(&mut self, label: &str) {
        let nodes = self.nodes();
        let (button, _) = nodes
            .iter()
            .find(|(_, node)| node.role() == Role::Button && node.label() == Some(label))
            .unwrap_or_else(|| panic!("{label} is offered"));
        self.click(*button);
    }

    /// The run Results draws.
    fn shown(&self) -> Option<DatasetId> {
        let simulation = &self.app.state.model.simulation;
        simulation.active_run().map(|run| run.dataset_id)
    }

    /// The saved document Results shows, if one is.
    fn shown_document(&self) -> Option<ResultDocumentId> {
        shown_document(self.app.state.results_reads())
    }

    /// The label the data browser gives the run of `dataset`.
    fn label_of(&self, dataset: DatasetId) -> String {
        let simulation = &self.app.state.model.simulation;
        let run = simulation.retained.run_by_dataset_id(dataset);
        run.expect("a retained run").label.clone()
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

/// Results draws no document tabs: neither the run the browser shows nor a
/// saved document is a tab, and the commands that stepped through or closed
/// tabs say why they do nothing here.
#[test]
fn results_draws_no_document_tabs() {
    let Bench {
        mut desk,
        first,
        other,
        ..
    } = bench();
    let names = [
        desk.name_of(first[1]),
        desk.name_of(other),
        "Bias".to_owned(),
    ];
    let nodes = desk.nodes();
    let tabbed = nodes.iter().find(|(_, node)| {
        node.role() == Role::Tab
            && node
                .label()
                .is_some_and(|label| names.iter().any(|name| label.starts_with(name.as_str())))
    });
    assert!(
        tabbed.is_none(),
        "a document is a tab: {:?}",
        tabbed.map(|(_, node)| node.label())
    );
    for command in [
        ShellCommand::PreviousDocument,
        ShellCommand::NextDocument,
        ShellCommand::CloseOtherDocuments,
        ShellCommand::CloseAllDocuments,
    ] {
        assert_eq!(
            command.availability(&desk.app),
            CommandAvailability::Disabled("Results picks what it shows in the data browser"),
            "{command:?}"
        );
    }
}

/// The saved documents are listed above every run, each by its name.
#[test]
fn a_saved_document_is_listed_above_the_runs() {
    let Bench {
        mut desk,
        first,
        other,
        ..
    } = bench();
    let nodes = desk.nodes();
    let documents = browser_rows::documents(&nodes);
    assert_eq!(browser_rows::document_names(&nodes), ["Bias"]);
    for run in [first[0], first[1], other] {
        let head = browser_rows::run_head(&nodes, &desk.label_of(run));
        assert!(
            documents[0].top < head.top,
            "the document is listed below {}",
            head.label
        );
    }
}

/// A document's row shows the document, and then it alone reads as shown;
/// a run's analysis shows the run again, and the document's row reads as
/// shown no more.
#[test]
fn a_documents_row_shows_it_and_a_runs_analysis_shows_the_run() {
    let Bench {
        mut desk,
        saved,
        other,
        ..
    } = bench();
    let bias = browser_rows::documents(&desk.nodes())[0].clone();
    assert!(!bias.shown, "the browser shows a run");
    desk.click(bias.id);
    assert_eq!(desk.shown_document(), Some(saved));
    assert_eq!(
        browser_rows::shown(&desk.nodes()),
        ["Bias"],
        "only the document's row reads as shown"
    );

    // The other plan's run is folded: open it, then pick its analysis.
    let label = desk.label_of(other);
    let head = browser_rows::run_head(&desk.nodes(), &label);
    desk.click(head.id);
    let analysis = browser_rows::analysis_under(&desk.nodes(), &label);
    desk.click(analysis.id);
    assert_eq!(desk.shown_document(), None);
    assert_eq!(desk.shown(), Some(other));
    let nodes = desk.nodes();
    assert!(!browser_rows::documents(&nodes)[0].shown);
    assert!(browser_rows::run_head(&nodes, &label).shown);
}

/// A document's row menu renames the document and deletes it. Deleting asks
/// nothing, since Undo puts the document back, and Results goes back to the
/// run the browser showed.
#[test]
fn a_documents_row_menu_renames_and_deletes_it() {
    let Bench {
        mut desk,
        first,
        saved,
        ..
    } = bench();
    let bias = browser_rows::documents(&desk.nodes())[0].clone();
    desk.right_click(bias.centre);
    desk.choose("Rename\u{2026}");
    let Some(ResultDocumentDialog::Rename(mut draft)) =
        desk.app.state.results.result_document_dialog.clone()
    else {
        panic!("Rename opens its form");
    };
    assert_eq!(draft.from, "Bias");
    draft.title = "Bias sweep".to_owned();
    desk.app.state.results.result_document_dialog = Some(ResultDocumentDialog::Rename(draft));
    desk.choose("Rename");
    assert_eq!(browser_rows::document_names(&desk.nodes()), ["Bias sweep"]);

    let row = browser_rows::documents(&desk.nodes())[0].clone();
    desk.right_click(row.centre);
    desk.choose("Delete");
    let content = &desk.app.state.model.workspace.content;
    assert!(content.visualization_document(saved).is_none());
    assert!(browser_rows::documents(&desk.nodes()).is_empty());
    assert_eq!(desk.shown_document(), None);
    assert_eq!(
        desk.shown(),
        Some(first[1]),
        "the browser's run shows again"
    );
    assert_eq!(
        desk.app.state.prompt_text(),
        Some("Deleted Bias sweep. Undo puts it back.")
    );

    desk.app.execute_command_key(ids::UNDO);
    assert_eq!(browser_rows::document_names(&desk.nodes()), ["Bias sweep"]);
}

/// A signal's menu makes a new result document over the signal's own run
/// and analysis, not over the run Results showed, with that signal ticked and
/// no other, and Results then shows the signal's run.
#[test]
fn a_signals_new_result_document_opens_on_its_run_and_analysis_with_it_ticked() {
    let Bench {
        mut desk,
        first,
        other,
        ..
    } = bench();
    let [_, newer] = first;
    desk.pick(other);
    assert_eq!(desk.shown(), Some(other));
    // A query opens every run, so the newer run's signals are listed under
    // its head while the other plan's run is shown.
    desk.app.state.workbench.set_navigator_filter("out");
    let nodes = desk.nodes();
    let head = browser_rows::run_head(&nodes, &desk.label_of(newer));
    let mut signals = nodes
        .iter()
        .filter(|(_, node)| {
            node.label()
                .is_some_and(|label| label.starts_with("V(out), "))
        })
        .filter_map(|(_, node)| node.bounds())
        .filter(|bounds| bounds.y0 > head.top)
        .collect::<Vec<_>>();
    signals.sort_by(|above, below| above.y0.total_cmp(&below.y0));
    let signal = signals.first().expect("the newer run lists V(out)");
    desk.right_click(egui::pos2(
        ((signal.x0 + signal.x1) * 0.5) as f32,
        ((signal.y0 + signal.y1) * 0.5) as f32,
    ));
    // The sheet bar offers the same words; the menu's entry is the one
    // drawn under the signal.
    let nodes = desk.nodes();
    let (entry, _) = nodes
        .iter()
        .filter(|(_, node)| {
            node.role() == Role::Button && node.label() == Some("New result document…")
        })
        .filter(|(_, node)| node.bounds().is_some_and(|bounds| bounds.y0 >= signal.y0))
        .min_by(|(_, above), (_, below)| {
            let top = |node: &Node| node.bounds().map_or(f64::MAX, |bounds| bounds.y0);
            top(above).total_cmp(&top(below))
        })
        .expect("the signal's menu offers New result document…");
    desk.click(*entry);

    let draft = &desk.app.state.results.create_result_document;
    assert!(draft.open, "the form opens");
    assert_eq!(draft.dataset_id, Some(newer), "on the signal's run");
    assert_eq!(draft.analysis, Some(0), "and on its analysis");
    assert_eq!(draft.signals, ["V(out)"], "with the signal ticked");
    assert_eq!(desk.shown(), Some(newer), "the signal's run shows");
    assert_eq!(desk.shown_document(), None);
}

/// The Datasets tab draws the rows in view and no others: of a hundred and
/// twenty runs, the newest's row is drawn and the oldest's is not.
#[test]
fn the_datasets_tab_draws_only_the_rows_in_view() {
    const RUNS: usize = 120;
    let mut state = session_with_a_run();
    state.model.simulation.retained.retained_dataset_limit = Some(RUNS);
    state.model.sim_setup.save_policy.retained_dataset_limit = RUNS;
    for _ in 1..RUNS {
        run_plan(&mut state, [0.0, 0.2, 0.4, 1.0, 0.1]);
    }
    let runs = newest_first(&state);
    assert_eq!(runs.len(), RUNS);
    let mut desk = Desk::new(state);
    let nodes = desk.nodes();
    let (tab, _) = nodes
        .iter()
        .find(|(_, node)| {
            node.label()
                .is_some_and(|label| label.starts_with("Datasets,"))
        })
        .expect("the Datasets tab");
    desk.click(*tab);
    // A run's row answers to its run's id, whatever its place.
    let drawn = |desk: &Desk, dataset: DatasetId| {
        desk.ctx
            .read_response(egui::Id::new(("workbench.results.run-row", dataset)))
            .is_some()
    };
    assert!(drawn(&desk, runs[0]), "the newest run's row is not drawn");
    assert!(
        !drawn(&desk, runs[RUNS - 1]),
        "the oldest run's row is drawn out of view"
    );
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

    desk.pick(first);
    run_plan(&mut desk.app.state, [0.0, 0.1, 0.2, 0.3, 0.4]);
    desk.settle();
    let third = newest_first(&desk.app.state)[0];
    assert_ne!(third, second);
    assert_eq!(desk.shown(), Some(first), "the run picked stays");

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
