//! The Netlist workspace's tabs, in whole applications: the generated
//! netlist and the Added lines pinned first, what Run runs as tabs come to
//! the front and close, the Added lines' check, and the tab a run's input
//! opens on.

use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{DESKTOP, themed_context};
use crate::workbench::examples::hierarchy_reference;
use rspice_app_code::workbench::documents::netlist_document::{self, CompareSide, NetlistTab};
use rspice_app_state::state::SimulationRunIntent;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::chrome::document_bar::{
    WorkspaceDocumentDescriptor, activate_document, close_document, document_descriptors,
};
use rspice_app_workbench::workbench::framework::command_registry::ids;
use rspice_app_workbench::workbench::state::workbench_session::WorkspaceDocumentId;
use rspice_vocabulary::diagnostics::DiagnosticSeverity;

const DECK: &str = "standalone\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";

/// The hierarchical reference design on the Netlist workspace, with its
/// generated netlist built and the engineer's deck.cir open in front.
fn plan_and_deck_session() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state = hierarchy_reference::build().state;
    app.state.seed_test_deck(DECK);
    app.state.workbench.activate(Workspace::Netlist);
    application_frame(&mut app);
    app
}

/// One whole application frame on a desktop window.
fn application_frame(app: &mut RSpiceApp) {
    application_frame_at(app, 0.0);
}

/// One whole application frame on a desktop window, `seconds` into the
/// session.
fn application_frame_at(app: &mut RSpiceApp, seconds: f64) {
    let _ = drawn_frame_at(app, seconds);
}

/// One whole application frame `seconds` into the session, and every string
/// it painted, one per line.
fn drawn_frame_at(app: &mut RSpiceApp, seconds: f64) -> String {
    fn collect(shape: &egui::epaint::Shape, text: &mut String) {
        match shape {
            egui::epaint::Shape::Text(shape) => {
                text.push_str(shape.galley.text());
                text.push('\n');
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, text);
                }
            }
            _ => {}
        }
    }
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, DESKTOP)),
        time: Some(seconds),
        ..Default::default()
    };
    let output = themed_context().run_ui(input, |ui| {
        app.prepare_frame(ui.ctx());
        app.render_frame_chrome(ui);
        app.render_frame_dialogs(ui.ctx());
    });
    let mut text = String::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut text);
    }
    text
}

/// The tab strip's tabs, by label and whether each closes.
fn tabs(app: &RSpiceApp) -> Vec<(String, bool)> {
    document_descriptors(&app.state)
        .into_iter()
        .map(|tab| (tab.label, tab.closable))
        .collect()
}

fn tab_labelled(app: &RSpiceApp, label: &str) -> WorkspaceDocumentDescriptor {
    document_descriptors(&app.state)
        .into_iter()
        .find(|tab| tab.label == label)
        .unwrap_or_else(|| panic!("no tab is labelled {label}: {:?}", tabs(app)))
}

/// Press F5 and say what it ran: the deck as written, or the plan.
fn press_run(app: &mut RSpiceApp) -> &'static str {
    app.execute_command_key(ids::RUN_SIMULATION);
    let execution = &app.state.model.simulation.execution;
    let deck =
        execution.trigger_simulation && execution.run_intent == SimulationRunIntent::ManualDeck;
    let plan = app.state.runs.preflight.take_run_and_queue_request();
    match (deck, plan) {
        (true, false) => "deck",
        (false, true) => "plan",
        (false, false) => "nothing",
        (true, true) => "both",
    }
}

/// The generated netlist and the Added lines come first and stay: neither
/// closes. The deck beside them does.
#[test]
fn the_generated_netlist_and_the_added_lines_are_pinned_first() {
    let app = plan_and_deck_session();

    assert_eq!(
        tabs(&app),
        [
            ("Generated netlist".to_owned(), false),
            ("Added lines".to_owned(), false),
            ("deck.cir".to_owned(), true),
        ]
    );
}

/// The Added lines run with the plan: their tab in front makes the plan
/// what F5 runs, where the deck in front ran the deck.
#[test]
fn the_added_lines_in_front_make_the_plan_what_runs() {
    let mut app = plan_and_deck_session();

    assert!(activate_document(
        &mut app.state,
        &WorkspaceDocumentId::NetlistAddedLines
    ));

    assert_eq!(app.state.code.netlist.front, NetlistTab::AddedLines);
    assert_eq!(press_run(&mut app), "plan");
}

/// Closing the deck's tab while it is what runs takes the tab, brings the
/// tab beside it to the front, and F5 runs the plan.
#[test]
fn closing_the_deck_tab_that_runs_makes_the_plan_what_runs() {
    let mut app = plan_and_deck_session();
    let deck = tab_labelled(&app, "deck.cir");
    assert!(deck.active, "the deck opened in front");
    let documents =
        rspice_app_code::workbench::chrome::document_bar::code::netlist_workspace_documents(
            app.state
                .code
                .netlist_tabs(&app.state.model, app.state.workbench.messages()),
        );

    assert!(close_document(&mut app.state, &deck.id, &documents));
    application_frame(&mut app);

    assert_eq!(
        tabs(&app),
        [
            ("Generated netlist".to_owned(), false),
            ("Added lines".to_owned(), false),
        ],
        "the deck's tab has gone and stays gone"
    );
    assert_eq!(app.state.code.netlist.front, NetlistTab::AddedLines);
    assert_eq!(press_run(&mut app), "plan");
}

/// A line the plan owns is refused on its own line of the Added lines, and
/// what was typed is kept with the project.
#[test]
fn an_added_line_the_plan_owns_is_refused_on_its_line() {
    let mut app = plan_and_deck_session();
    assert!(activate_document(
        &mut app.state,
        &WorkspaceDocumentId::NetlistAddedLines
    ));
    let typed = "R9 out 0 10k\n.tran 1n 1u\n";

    assert!(netlist_document::edit_added_lines(
        &mut app.state.model,
        &mut app.state.code.netlist,
        typed.to_owned(),
    ));
    // The check reads the lines once typing has paused.
    application_frame_at(&mut app, 1.0);

    let errors = app
        .state
        .code
        .netlist
        .diagnostics
        .iter()
        .filter(|finding| finding.severity == DiagnosticSeverity::Error)
        .map(|finding| (finding.line, finding.message.clone()))
        .collect::<Vec<_>>();
    assert_eq!(
        errors,
        [(
            Some(1),
            "Analyses belong to the plan: add .tran in Simulate ▸ Analyses".to_owned()
        )],
        "the second line is refused, the first is not"
    );
    assert_eq!(netlist_document::added_lines_text(&app.state.model), typed);
    assert!(
        app.state.model.workspace.content.project_metadata_dirty,
        "the project has the edit to save"
    );
}

/// A run's input opened from Results is a tab named for the run, holding
/// the text the run read through the frames after, and the inspector says
/// which run it is and what that run ran.
#[test]
fn a_run_input_opened_from_results_is_a_tab_that_keeps_its_text() {
    use crate::workbench::commands::tests::{RunOrigin, app_with_prepared_run};
    use rspice_app_results::workbench::results::commands::ResultsCommand;
    use rspice_app_state::state::{ExecutedDeck, ExecutedDeckPoint};

    const INPUT: &str = "task deck\nV1 1 0 1\nR1 1 0 1k\n.op\n.end\n";
    let mut app = app_with_prepared_run(RunOrigin::ActivePlan);
    let run = app
        .state
        .model
        .simulation
        .active_run()
        .expect("the fixture selected a run")
        .id;
    app.state
        .model
        .simulation
        .retained
        .executed_decks
        .retain(ExecutedDeck {
            run_id: run,
            points: vec![ExecutedDeckPoint {
                label: "nominal".to_owned(),
                deck: INPUT.into(),
                model_sources: Vec::new(),
            }],
        });

    app.execute_command_key(ResultsCommand::OpenTaskDeck.key());
    application_frame(&mut app);
    application_frame(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);
    let tab = tab_labelled(&app, &format!("Run {run} input"));
    assert!(tab.active, "{tab:?}");
    assert_eq!(app.state.model.simulation.source.netlist_content, INPUT);

    // The inspector states whose input it is.
    let painted = drawn_frame_at(&mut app, 0.0);
    for row in ["Run input", "Run", &run.to_string(), "Ran", "The plan"] {
        assert!(
            painted.lines().any(|line| line == row),
            "the inspector shows {row}: {painted}"
        );
    }
}

/// The deck compared with its file on disk shows what was typed since, on
/// a tab naming both sides; closing the view brings the deck back with the
/// edit, and the deck is still what F5 runs.
#[test]
fn the_deck_compared_with_its_file_shows_the_edit_on_a_tab_naming_both() {
    let folder = std::env::temp_dir().join(format!(
        "rspice-deck-disk-comparison-{}",
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(&folder).expect("fixture folder");
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.open_test_deck(
        DECK.to_owned(),
        Some(folder.join("amp_tb.cir")),
        "amp_tb.cir"
    ));
    app.state.workbench.workspace = Workspace::Netlist;
    let edited = DECK.replace("1k", "2k");
    assert!(netlist_document::edit_deck_file(
        &mut app.state.model,
        &mut app.state.code.netlist,
        edited.clone(),
    ));
    let deck = app
        .state
        .code
        .netlist
        .open_deck
        .as_ref()
        .expect("the deck is open")
        .id;

    netlist_document::compare(
        &mut app.state.model,
        &mut app.state.code.netlist,
        &app.state.runs,
        CompareSide::DiskFile(deck),
        CompareSide::Deck(deck),
    )
    .expect("the deck and its file compare");
    application_frame(&mut app);

    let view = tab_labelled(&app, "amp_tb.cir on disk vs amp_tb.cir");
    assert!(view.active && view.closable, "{view:?}");
    let shown = app.state.model.simulation.source.netlist_content.clone();
    assert!(
        shown.starts_with("--- amp_tb.cir on disk\n+++ amp_tb.cir\n"),
        "{shown}"
    );
    assert!(shown.contains("\n-R1 out 0 1k\n+R1 out 0 2k\n"), "{shown}");
    assert_eq!(press_run(&mut app), "deck", "a view leaves what runs");

    let documents =
        rspice_app_code::workbench::chrome::document_bar::code::netlist_workspace_documents(
            app.state
                .code
                .netlist_tabs(&app.state.model, app.state.workbench.messages()),
        );
    assert!(close_document(&mut app.state, &view.id, &documents));
    assert_eq!(app.state.code.netlist.front, NetlistTab::Deck(deck));
    assert_eq!(app.state.model.simulation.source.netlist_content, edited);
    std::fs::remove_dir_all(folder).expect("fixture removed");
}

/// The deck compared with the generated netlist, and the generated netlist
/// with its previous build, each name their sides on the tab.
#[test]
fn a_comparison_tab_names_the_two_texts_it_compares() {
    let mut app = plan_and_deck_session();
    let deck = app
        .state
        .code
        .netlist
        .open_deck
        .as_ref()
        .expect("the deck is open")
        .id;

    netlist_document::compare(
        &mut app.state.model,
        &mut app.state.code.netlist,
        &app.state.runs,
        CompareSide::Deck(deck),
        CompareSide::Generated,
    )
    .expect("the deck and the generated netlist compare");

    assert!(tab_labelled(&app, "deck.cir vs Generated netlist").active);
    let shown = &app.state.model.simulation.source.netlist_content;
    assert!(
        shown.starts_with("--- deck.cir\n+++ Generated netlist\n"),
        "{shown}"
    );
}

/// With the Added lines in front, the inspector names the design they
/// belong to and says they run with the plan.
#[test]
fn the_inspector_says_whose_added_lines_are_in_front() {
    let mut app = plan_and_deck_session();
    assert!(activate_document(
        &mut app.state,
        &WorkspaceDocumentId::NetlistAddedLines
    ));
    let design = netlist_document::added_lines_design(&app.state.model)
        .expect("the reference design has a root")
        .display_path();

    let painted = drawn_frame_at(&mut app, 0.0);

    for row in ["Design", design.as_str(), "Runs with", "The plan"] {
        assert!(
            painted.lines().any(|line| line == row),
            "the inspector shows {row}: {painted}"
        );
    }
}

/// A save dialog that answers with `self.0`, and writes that reach the disk.
struct SavesTo(std::path::PathBuf);

impl rspice_app_services::workbench::workflows::export_workflow::ExportWorkflowIo for SavesTo {
    fn show_save_dialog(
        &self,
        _config: rspice_app_services::workbench::workflows::export_workflow::SaveDialogConfig<'_>,
    ) -> Result<Option<std::path::PathBuf>, String> {
        Ok(Some(self.0.clone()))
    }

    fn write_text_file(&self, path: &std::path::Path, contents: &str) -> Result<(), String> {
        std::fs::write(path, contents).map_err(|error| error.to_string())
    }

    fn write_bytes_file_observed(
        &self,
        destination: &rspice_app_services::workbench::workflows::export_workflow::ObservedExportDestination,
        contents: &[u8],
        _mime_type: &str,
    ) -> Result<(), String> {
        std::fs::write(destination.path(), contents).map_err(|error| error.to_string())
    }

    fn write_waveform_csv(
        &self,
        _dataset: &rspice_app_services::io::WaveformDataset,
        _path: &std::path::Path,
    ) -> Result<(), String> {
        Ok(())
    }
}

/// Export with the deck in front writes the deck as it is in the editor, not
/// the generated netlist, and opens no dialog; with the generated netlist in
/// front it opens the generated netlist's export dialog.
#[test]
fn export_writes_the_deck_in_front_and_the_generated_netlist_behind_its_dialog() {
    use rspice_app_code::workbench::code::commands::CodeCommand;

    let mut app = plan_and_deck_session();
    let edited = DECK.replace("1k", "4k7");
    assert!(netlist_document::edit_deck_file(
        &mut app.state.model,
        &mut app.state.code.netlist,
        edited.clone(),
    ));
    let folder = std::env::temp_dir().join(format!("rspice-deck-export-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&folder).expect("fixture folder");
    let copy = folder.join("copy.cir");
    app.export_workflow_io = Box::new(SavesTo(copy.clone()));

    app.execute_command_key(CodeCommand::ExportNetlist.key());

    assert_eq!(
        std::fs::read_to_string(&copy).expect("the export wrote the deck"),
        edited
    );
    assert!(
        !app.state.code.netlist.export_dialog.open,
        "no dialog for a deck"
    );

    let generated = tab_labelled(&app, "Generated netlist").id;
    assert!(activate_document(&mut app.state, &generated));
    app.execute_command_key(CodeCommand::ExportNetlist.key());
    assert!(
        app.state.code.netlist.export_dialog.open,
        "the generated netlist exports through its dialog"
    );
    std::fs::remove_dir_all(folder).expect("fixture removed");
}
