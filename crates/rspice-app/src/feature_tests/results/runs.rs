//! A retained run as the reader handles it in Results: the tab and the run
//! selector that show it, and its identity across a save and reopen.

use crate::workbench::app_state::AppState;
use rspice_app_results::workbench::chrome::document_bar::results::result_documents;
use rspice_app_results::workbench::chrome::toolbar::results::run_selector;
use rspice_app_results::workbench::documents::result_document::working_view;
use rspice_app_runs::simulation::controller::SimulationController;
use rspice_app_runs::simulation::controller::test_execution::{
    run_generated_batches, state_with_current_clean_drc,
};
use rspice_app_state::product::DatasetId;
use rspice_app_state::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRunLifecycle,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::layout::LayoutSpec;
use rspice_app_workbench::workbench::state::workbench_session::WorkspaceDocumentId;

const TIME: [f64; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];

/// Run the session's plan once, its transient answering `out`, and finish
/// the run, its samples held in memory.
fn run_plan(state: &mut AppState, out: [f64; 5]) {
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
                    RetainedTrace::new("V(in)", TIME.to_vec(), vec![1.0; 5], "#ffffff"),
                    RetainedTrace::new("V(out)", TIME.to_vec(), out.to_vec(), "#ffffff"),
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

/// The quick view's frame around nothing: the shown run's working view is
/// made or followed, drawn, and kept.
fn quick_frame(state: &mut AppState) {
    let mut parts = state.results_parts();
    working_view::sync(&mut parts);
    working_view::capture(&mut parts);
}

/// The datasets of the retained runs, newest first.
fn newest_first(state: &AppState) -> Vec<DatasetId> {
    let mut runs: Vec<_> = state.model.simulation.retained.runs.iter().collect();
    runs.sort_by_key(|run| std::cmp::Reverse(run.id));
    runs.into_iter().map(|run| run.dataset_id).collect()
}

/// What the run selector reads: the run shown and whether it follows the
/// plan's newest.
fn selector(state: &AppState) -> (DatasetId, bool) {
    let choice =
        run_selector::run_choice(state.results_reads()).expect("Results shows a run of the plan");
    (choice.shown.dataset_id, choice.following)
}

/// The Results tabs: what each shows and what it is called.
fn tabs(state: &AppState) -> Vec<(WorkspaceDocumentId, String)> {
    result_documents(&state.model)
        .into_iter()
        .map(|tab| (tab.id, tab.label))
        .collect()
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

/// A plan's runs are chosen, not tabbed: three runs of one plan are one tab,
/// named for the plan, on the newest run.
#[test]
fn a_plans_runs_share_one_tab_named_for_the_plan() {
    let mut state = session_with_a_run();
    quick_frame(&mut state);
    for out in [[0.0, 0.2, 0.4, 1.0, 0.1], [0.0, 0.1, 0.2, 0.3, 0.4]] {
        run_plan(&mut state, out);
        quick_frame(&mut state);
    }
    let plan = state.model.sim_setup.active_plan_name().to_string();
    assert_eq!(
        tabs(&state),
        vec![(
            WorkspaceDocumentId::ResultDataset(newest_first(&state)[0]),
            plan
        )]
    );
}

/// A run chosen from the selector stays shown, frame after frame, until
/// Latest is chosen or the plan runs again, as Assembler's Results tab
/// turns to the history a run makes.
#[test]
fn a_run_chosen_from_the_selector_stays_shown_until_the_plan_runs_again() {
    let mut state = session_with_a_run();
    quick_frame(&mut state);
    run_plan(&mut state, [0.0, 0.2, 0.4, 1.0, 0.1]);
    quick_frame(&mut state);
    let [second, first] = newest_first(&state)[..] else {
        panic!("two runs");
    };
    assert_eq!(selector(&state), (second, true), "Latest shows the newest");
    let listed: Vec<DatasetId> = run_selector::run_choice(state.results_reads())
        .expect("the plan's runs")
        .runs
        .iter()
        .map(|run| run.dataset_id)
        .collect();
    assert_eq!(listed, vec![second, first], "newest first");

    run_selector::choose(&mut state.results_parts(), Some(first)).expect("the first run shows");
    for _ in 0..3 {
        quick_frame(&mut state);
    }
    assert_eq!(selector(&state), (first, false));
    let plan = state.model.sim_setup.active_plan_name().to_string();
    assert_eq!(
        tabs(&state),
        vec![(WorkspaceDocumentId::ResultDataset(first), plan.clone())],
        "the plan's tab shows the run chosen"
    );

    run_selector::choose(&mut state.results_parts(), None).expect("Latest");
    quick_frame(&mut state);
    assert_eq!(selector(&state), (second, true));

    run_selector::choose(&mut state.results_parts(), Some(first)).expect("the first run shows");
    quick_frame(&mut state);
    run_plan(&mut state, [0.0, 0.1, 0.2, 0.3, 0.4]);
    quick_frame(&mut state);
    let third = newest_first(&state)[0];
    assert_eq!(
        selector(&state),
        (third, true),
        "the plan's new run takes the view back to Latest"
    );
    assert_eq!(
        tabs(&state),
        vec![(WorkspaceDocumentId::ResultDataset(third), plan)]
    );
}

/// The selector lists two hundred runs and lays out only the rows in view.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_run_selector_lists_two_hundred_runs_and_lays_out_a_page_of_them() {
    let mut state = session_with_a_run();
    quick_frame(&mut state);
    let template = state.model.simulation.retained.runs[0].clone();
    let mut runs: Vec<_> = (2..=200)
        .rev()
        .map(|id| {
            let mut run = template.clone();
            run.id = id;
            run.dataset_id = DatasetId::new();
            run.rename(&format!("Run {id}"));
            run
        })
        .collect();
    runs.push(template);
    state.model.simulation.retained.runs = runs.into();
    assert!(state.model.simulation.select_run(0));
    quick_frame(&mut state);
    assert_eq!(
        run_selector::run_choice(state.results_reads())
            .expect("the plan's runs")
            .runs
            .len(),
        200
    );

    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let layout = LayoutSpec::resolve(1_600.0, 900.0, &state.workbench);
    egui::Popup::open_id(&ctx, run_selector::list_id());
    let mut frame = || {
        ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| run_selector::show(ui, &mut state.results_parts(), layout));
            });
        })
    };
    let _ = frame();
    let output = frame();
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("an accessibility tree")
        .nodes;
    // The update lists its nodes in no order, so the rows are read top to
    // bottom by where they stand.
    let mut placed: Vec<(f64, &str)> = nodes
        .iter()
        .filter_map(|(_, node)| Some((node.bounds()?.y0, node.label()?)))
        .filter(|(_, label)| label.starts_with("Run "))
        .collect();
    placed.sort_by(|a, b| a.0.total_cmp(&b.0));
    let rows: Vec<&str> = placed.into_iter().map(|(_, label)| label).collect();
    assert_eq!(
        rows.first(),
        Some(&"Run 200"),
        "the newest run heads the list"
    );
    assert!(
        rows.len() <= run_selector::VISIBLE_ROWS + 1,
        "only the rows in view are laid out, not {} of 200",
        rows.len()
    );
}
