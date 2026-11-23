//! A run of any size has a working view, and the view keeps every tick: all
//! the signals of a run of a hundred thousand stay plotted through a save
//! and a reopen, and taking them all off is one step to undo. The view of a
//! run of a million signals names the signals it plots and none of the rest,
//! so it saves in a few kilobytes. A new document of a run, and a pane added
//! to one, plot what the run's deck names and nothing else.

use super::{plan_view, quick_frame, run_plan_with, show_newest, view_plots};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use rspice_app_results::workbench::docks::navigator::results_actions::select_result_dataset;
use rspice_app_results::workbench::documents::result_document::AnalysisPresentationKey;
use rspice_app_results::workbench::documents::result_document::create_document;
use rspice_app_results::workbench::documents::result_document::document_edit::{
    add_pane_choices, commit_open_dialog, open_add_pane,
};
use rspice_app_runs::simulation::controller::test_execution::state_with_current_clean_drc;
use rspice_app_state::product::ResultDocumentId;
use rspice_app_state::state::RetainedTrace;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::WorkbenchCommand;
use rspice_results::document_creation::{ResultDocumentFamily, ResultDocumentLayout};
use rspice_results::result_presentation::SourceWaveformPresentationKey;
use rspice_results_ui::create_document::CreateResultDocumentDialogState;
use rspice_results_ui::document_dialogs::ResultDocumentDialog;

/// Results over the session's plan, which has not run yet.
fn app_on_the_plan() -> RSpiceApp {
    let host = state_with_current_clean_drc();
    let mut app = RSpiceApp::test_instance();
    app.state.model = host.model;
    app.state.runs = host.runs;
    app.state.workbench.activate(Workspace::Results);
    app
}

/// `count` signals, none of them marked to plot.
fn signals(count: usize) -> Vec<RetainedTrace> {
    (0..count)
        .map(|index| {
            let mut waveform = RetainedTrace::new(
                format!("V(n{index})"),
                vec![0.0, 1.0],
                vec![0.0, 1.0],
                "#ffffff",
            );
            waveform.visible = false;
            waveform
        })
        .collect()
}

/// The shown run's analysis, as the data browser keys it.
fn shown_analysis(state: &AppState) -> AnalysisPresentationKey {
    let run = state.model.simulation.active_run().expect("a shown run");
    AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
}

/// Tick or untick `names` in the data browser, in one gesture.
fn tick(state: &mut AppState, names: impl IntoIterator<Item = String>, ticked: bool) {
    let analysis = shown_analysis(state);
    let session = &mut state.results.results.session;
    for name in names {
        session
            .waveform_visibility
            .insert(SourceWaveformPresentationKey::new(analysis, name), ticked);
    }
    quick_frame(state);
}

/// Every signal of the shown run, by name.
fn every_signal(state: &AppState) -> Vec<String> {
    let run = state.model.simulation.active_run().expect("a shown run");
    run.analyses[0]
        .waveforms
        .iter()
        .map(|waveform| waveform.name.clone())
        .collect()
}

/// How many signals the data browser shows ticked, and how many the view
/// plots.
fn ticked_and_plotted(state: &AppState) -> (usize, usize) {
    let analysis = shown_analysis(state);
    let run = state.model.simulation.active_run().expect("a shown run");
    let session = &state.results.results.session;
    let ticked = run.analyses[0]
        .waveforms
        .iter()
        .filter(|waveform| {
            session.waveform_visibility(
                &SourceWaveformPresentationKey::new(analysis, waveform.name.clone()),
                waveform.visible,
            )
        })
        .count();
    let plotted = view_plots(state).iter().filter(|(_, drawn)| *drawn).count();
    (ticked, plotted)
}

/// Save `state`'s project, reopen it and show its newest run.
fn save_and_reopen(mut state: AppState) -> AppState {
    let path = std::env::temp_dir().join(format!(
        "rspice-large-view-{}.rspiceproj",
        uuid::Uuid::new_v4()
    ));
    assert!(state.save_project_to_path(&path), "the project saves");
    drop(state);
    let mut state = AppState::default();
    let reopened = state.load_project_from_path(&path);
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
    let mut lock = path.into_os_string();
    lock.push(".rspice.lock");
    let _ = std::fs::remove_file(std::path::PathBuf::from(lock));
    assert!(reopened, "the project reopens");
    show_newest(&mut state);
    quick_frame(&mut state);
    state
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_signal_of_a_hundred_thousand_stays_plotted_and_taking_all_off_is_one_undo() {
    const SIGNALS: usize = 100_000;
    let mut app = app_on_the_plan();
    run_plan_with(&mut app.state, signals(SIGNALS));
    // Shown from the data browser, as a reader shows a run.
    assert!(select_result_dataset(&mut app.state.results_parts(), 0));
    quick_frame(&mut app.state);
    assert_eq!(ticked_and_plotted(&app.state), (0, 0), "nothing is marked");

    let names = every_signal(&app.state);
    tick(&mut app.state, names.clone(), true);
    assert_eq!(
        ticked_and_plotted(&app.state),
        (SIGNALS, SIGNALS),
        "every tick is kept, in the browser and on the plot"
    );

    tick(&mut app.state, names, false);
    assert_eq!(ticked_and_plotted(&app.state), (0, 0), "all taken off");
    assert!(app.command_key_available(WorkbenchCommand::Undo.key()));
    app.execute_command_key(WorkbenchCommand::Undo.key());
    quick_frame(&mut app.state);
    assert_eq!(
        ticked_and_plotted(&app.state),
        (SIGNALS, SIGNALS),
        "one undo puts every signal back"
    );

    let state = save_and_reopen(std::mem::take(&mut app.state));
    assert_eq!(
        ticked_and_plotted(&state),
        (SIGNALS, SIGNALS),
        "the reopened project plots every signal"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_run_of_a_million_signals_has_a_view_that_names_only_what_it_plots() {
    const SIGNALS: usize = 1_000_000;
    let mut app = app_on_the_plan();
    run_plan_with(&mut app.state, signals(SIGNALS));
    show_newest(&mut app.state);
    quick_frame(&mut app.state);
    let plotted = ["V(n0)", "V(n500000)", "V(n999999)"].map(str::to_owned);
    tick(&mut app.state, plotted.clone(), true);
    assert_eq!(ticked_and_plotted(&app.state), (3, 3));

    // The view as the project file holds it, read back the way the project
    // reads it, in place of the one the project held. (The run itself is
    // kept beside it in the file, at nearly a gigabyte; the view is not.)
    let view_id = plan_view(&app.state);
    let documents = &mut app.state.model.workspace.content.visualization_documents;
    let index = documents
        .iter()
        .position(|document| document.id() == view_id)
        .expect("the plan's view");
    let saved = serde_json::to_vec(&documents[index]).expect("the view saves");
    documents[index] = serde_json::from_slice(&saved).expect("the view reopens");
    quick_frame(&mut app.state);
    assert_eq!(
        view_plots(&app.state),
        plotted.map(|name| (name, true)).to_vec(),
        "the reopened view plots what it plotted"
    );
    assert_eq!(ticked_and_plotted(&app.state), (3, 3));
    assert!(
        saved.len() < SIGNALS,
        "the view holds nothing for a signal it does not plot: {} bytes",
        saved.len()
    );
}

/// The labels of the signals `document_id` plots, pane by pane.
fn document_plots(state: &AppState, document_id: ResultDocumentId) -> Vec<Vec<String>> {
    let document = state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("the new document");
    document
        .panes()
        .iter()
        .map(|pane| {
            document
                .traces()
                .iter()
                .filter(|trace| trace.pane_id == pane.id && trace.visible)
                .map(|trace| trace.label.clone())
                .collect()
        })
        .collect()
}

/// Make a one-pane waveform document of the shown run, as Create does.
fn create_document(state: &mut AppState) -> ResultDocumentId {
    let dataset_id = state
        .model
        .simulation
        .active_run()
        .expect("a shown run")
        .dataset_id;
    state.results.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name: "Review".to_owned(),
        name_touched: true,
        dataset_id: Some(dataset_id),
        family_id: ResultDocumentFamily::WaveformWorksheet.id().to_owned(),
        viewer_id: "viewer-waveform".to_owned(),
        layout_id: ResultDocumentLayout::SinglePane.id().to_owned(),
        validation_error: None,
    };
    create_document::commit(&mut state.results_parts()).expect("the document is made")
}

/// Add a pane of the shown analysis after the selected one, as Add pane does.
fn add_pane(state: &mut AppState) {
    open_add_pane(&mut state.results_parts());
    let Some(ResultDocumentDialog::AddPane(mut draft)) =
        state.results.result_document_dialog.clone()
    else {
        panic!("Add pane opens its form");
    };
    assert_eq!(
        add_pane_choices(state.results_reads(), &mut draft).refusal,
        None
    );
    state.results.result_document_dialog = Some(ResultDocumentDialog::AddPane(draft));
    commit_open_dialog(&mut state.results_parts());
    assert!(
        state.results.result_document_dialog.is_none(),
        "the pane is added"
    );
}

/// A new document, and a pane added to it, plot what the run's deck names
/// to plot, however many signals the run has: never every signal.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_new_document_and_an_added_pane_plot_what_the_deck_names_of_a_million_signals() {
    const SIGNALS: usize = 1_000_000;
    let mut waveforms = signals(SIGNALS);
    for index in [7, SIGNALS - 1] {
        waveforms[index].visible = true;
    }
    let mut app = app_on_the_plan();
    run_plan_with(&mut app.state, waveforms);
    show_newest(&mut app.state);
    quick_frame(&mut app.state);

    let started = std::time::Instant::now();
    let document_id = create_document(&mut app.state);
    println!(
        "a new document of a million-signal run in {:?}",
        started.elapsed()
    );
    let named = vec!["V(n7)".to_owned(), format!("V(n{})", SIGNALS - 1)];
    assert_eq!(document_plots(&app.state, document_id), [named.clone()]);

    add_pane(&mut app.state);
    assert_eq!(
        document_plots(&app.state, document_id),
        [named.clone(), named],
        "an added pane plots the same"
    );
}

/// A run whose deck names nothing to plot gives a new document and an added
/// pane that plot nothing until the reader ticks a signal.
#[test]
fn a_run_that_names_nothing_gives_a_document_that_plots_nothing() {
    let mut app = app_on_the_plan();
    run_plan_with(&mut app.state, signals(20));
    show_newest(&mut app.state);
    quick_frame(&mut app.state);

    let document_id = create_document(&mut app.state);
    add_pane(&mut app.state);
    assert_eq!(
        document_plots(&app.state, document_id),
        [Vec::<String>::new(), Vec::new()]
    );
}
