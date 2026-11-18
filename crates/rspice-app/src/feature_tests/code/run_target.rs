//! Run on the Netlist workspace, in whole applications: what Run runs as
//! tabs come to the front, the check that is Run's first step, where that
//! check found the deck's includes, how the toolbar names what runs, and
//! what Check and netlist checks there.

use crate::workbench::RSpiceApp;
use rspice_app_code::workbench::code::commands::CodeCommand;
use rspice_app_state::state::SimulationRunIntent;
use rspice_app_state::workbench::preferences::FolderListPreference;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::{CommandAvailability, ids};

const DECK: &str = "standalone\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";

/// A session on the Netlist workspace with the deck `source` open from a
/// file called `name`, and the schematic's components gone.
fn netlist_session_with_deck(source: &str, name: &str) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    assert!(app.state.open_test_deck(source.to_owned(), None, name));
    app.state.workbench.workspace = Workspace::Netlist;
    app
}

/// Press F5: run Run as the key does.
fn press_run(app: &mut RSpiceApp) {
    app.execute_command_key(ids::RUN_SIMULATION);
}

/// Whether the press started a run of the deck as written.
fn deck_run_started(app: &RSpiceApp) -> bool {
    let execution = &app.state.model.simulation.execution;
    execution.trigger_simulation && execution.run_intent == SimulationRunIntent::ManualDeck
}

/// With a deck's tab in front, F5 checks the deck and starts a run of it as
/// written, in one press, and nothing of the plan's.
#[test]
fn a_deck_in_front_runs_as_written_in_one_press() {
    let mut app = netlist_session_with_deck(DECK, "amp_tb.cir");

    assert_eq!(
        app.command_availability(ids::RUN_SIMULATION),
        CommandAvailability::Available
    );
    press_run(&mut app);

    assert!(deck_run_started(&app), "F5 runs the deck");
    assert!(
        !app.state.runs.preflight.take_run_and_queue_request(),
        "the plan's run is not asked for"
    );
}

/// An include, a run's input and a comparison in front leave the deck as
/// what Run runs.
#[test]
fn an_include_in_front_leaves_the_deck_as_what_runs() {
    let folder = std::env::temp_dir().join(format!(
        "rspice-run-target-include-{}",
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(&folder).expect("fixture folder");
    std::fs::write(folder.join("load.inc"), "R2 out 0 2k\n").expect("include written");
    let source = "standalone\n.include \"load.inc\"\nV1 out 0 1\n.op\n.end\n";
    let root = folder.join("amp_tb.cir");
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    assert!(
        app.state
            .open_test_deck(source.to_owned(), Some(root), "amp_tb.cir")
    );
    app.state.workbench.workspace = Workspace::Netlist;

    rspice_app_code::workbench::documents::deck_files::show_include(
        &mut app.state.model,
        &mut app.state.code.netlist,
        &mut app.state.code.netlist_open_documents,
        &mut app.state.runs,
        &folder.join("load.inc"),
        app.state.workbench.messages(),
    )
    .expect("the include opens");
    press_run(&mut app);

    assert!(deck_run_started(&app), "the deck runs, not its include");
    std::fs::remove_dir_all(folder).expect("fixture removed");
}

/// The generated netlist in front makes the plan what Run runs: F5 asks for
/// Simulate's own run, and the deck does not run.
#[test]
fn the_generated_netlist_in_front_runs_the_plan() {
    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state
            .open_test_deck(DECK.to_owned(), None, "amp_tb.cir")
    );
    app.state.workbench.workspace = Workspace::Netlist;
    rspice_app_code::workbench::documents::netlist_document::show_generated_deck(
        &mut app.state.model,
        &mut app.state.code.netlist,
        &mut app.state.workbench,
    );

    press_run(&mut app);

    assert!(
        app.state.runs.preflight.take_run_and_queue_request(),
        "F5 is Simulate's Run of the plan"
    );
    assert!(!deck_run_started(&app), "the deck does not run");
}

/// A deck the check stops starts nothing, comes to the front, and the notice
/// names the deck and the line.
#[test]
fn a_deck_with_an_error_does_not_run_and_the_notice_names_the_line() {
    let source = "standalone\nV1 out 0 1\nQ1 out out 0 qn2\n.op\n.end\n";
    let mut app = netlist_session_with_deck(source, "amp_tb.cir");

    press_run(&mut app);

    assert!(!deck_run_started(&app), "nothing starts");
    let notice = app.state.prompt_text().unwrap_or_default().to_owned();
    assert!(
        notice.starts_with("amp_tb.cir did not run: "),
        "the notice names the deck: {notice}"
    );
    assert!(
        notice.contains("line 3"),
        "the notice names the line: {notice}"
    );
}

/// Nothing anywhere asks for a separate validation before Run: no command is
/// called Validate.
#[test]
fn no_command_asks_for_a_validation_before_run() {
    for command in CodeCommand::ALL {
        let label = command.declared_spec().label;
        assert!(!label.contains("Validat"), "{label}");
    }
}

/// While a run is going, Run on the Netlist workspace is refused in Simulate's
/// words, through the part the code feature takes in Run.
#[test]
fn run_is_refused_while_a_run_is_going_on_the_netlist_workspace() {
    let mut app = netlist_session_with_deck(DECK, "amp_tb.cir");
    app.state.model.simulation.execution.trigger_simulation = true;

    assert_eq!(
        app.routed_command_key_enabled(ids::RUN_SIMULATION),
        Some(false)
    );
    assert_eq!(
        app.command_availability(ids::RUN_SIMULATION),
        CommandAvailability::Disabled("a run is already in progress")
    );
}

/// Show deck brings the deck Run runs to the front from one of its includes.
#[test]
fn show_deck_brings_the_deck_that_runs_to_the_front() {
    let mut app = netlist_session_with_deck(DECK, "amp_tb.cir");
    assert_eq!(
        app.command_availability(CodeCommand::ShowRunDeck.key()),
        CommandAvailability::Available
    );
    rspice_app_code::workbench::documents::netlist_document::show_generated_deck(
        &mut app.state.model,
        &mut app.state.code.netlist,
        &mut app.state.workbench,
    );
    assert!(
        !app.command_availability(CodeCommand::ShowRunDeck.key())
            .is_available(),
        "with the plan as what runs, there is no deck to show"
    );
}

#[test]
fn run_finds_direct_and_transitive_includes_where_they_are() {
    let dir = std::env::temp_dir().join(format!(
        "rspice-netlist-dependency-closure-{}",
        uuid::Uuid::new_v4()
    ));
    let models = dir.join("models");
    std::fs::create_dir_all(&models).expect("create dependency fixture");
    let root = dir.join("root.cir");
    std::fs::write(
        models.join("first.inc"),
        ".include \"nested.inc\"\nRmodel out 0 2k\n",
    )
    .expect("write direct include");
    std::fs::write(models.join("nested.inc"), "Cmodel out 0 1p\n")
        .expect("write transitive include");
    let source = "standalone\n.include \"models/first.inc\"\nV1 out 0 1\n.op\n.end\n";
    std::fs::write(&root, source).expect("write root deck");

    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state
            .open_test_deck(source.to_owned(), Some(root.clone()), "root.cir")
    );
    app.state.workbench.workspace = Workspace::Netlist;
    press_run(&mut app);
    assert!(deck_run_started(&app));

    let resolutions = &app.state.code.code_workspace.include_resolutions;
    let canonical = |path: &std::path::Path| std::fs::canonicalize(path).expect("fixture path");
    assert_eq!(
        resolutions.keys().map(String::as_str).collect::<Vec<_>>(),
        ["models/first.inc", "nested.inc"],
        "the direct include and the include of an include, as their lines name them"
    );
    assert_eq!(
        canonical(resolutions["models/first.inc"].resolved()),
        canonical(&models.join("first.inc"))
    );
    assert_eq!(
        canonical(resolutions["nested.inc"].resolved()),
        canonical(&models.join("nested.inc")),
        "an include's own include is found beside it"
    );

    std::fs::remove_dir_all(dir).expect("remove dependency fixture");
}

#[test]
fn run_distinguishes_inline_lib_sections_from_external_lib_paths() {
    let dir = std::env::temp_dir().join(format!(
        "rspice-netlist-library-closure-{}",
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(&dir).expect("create library fixture");
    let root = dir.join("root.cir");
    let library_source =
        ".lib TT\n.model DMOD D IS=1e-14\n.endl TT\n.lib SS\n.model DMOD D IS=2e-14\n.endl SS\n";
    std::fs::write(dir.join("corners.lib"), library_source).expect("write library");
    let source = "standalone\n.lib LOCAL\n.model LOCALMOD D IS=3e-14\n.endl LOCAL\n.lib \"corners.lib\" TT\nD1 out 0 DMOD\n.op\n.end\n";
    std::fs::write(&root, source).expect("write root deck");

    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state
            .open_test_deck(source.to_owned(), Some(root), "root.cir")
    );
    app.state.workbench.workspace = Workspace::Netlist;
    press_run(&mut app);
    assert!(deck_run_started(&app));

    assert_eq!(
        app.state
            .code
            .code_workspace
            .include_resolutions
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["corners.lib"],
        "an inline section is part of the deck; only the library file is read"
    );

    std::fs::remove_dir_all(dir).expect("remove library fixture");
}

#[test]
fn run_resolves_an_include_only_a_library_folder_holds() {
    let directory = std::env::temp_dir().join(format!(
        "rspice-netlist-library-folder-{}",
        uuid::Uuid::new_v4()
    ));
    let deck = directory.join("deck");
    let shared = directory.join("shared");
    std::fs::create_dir_all(&deck).expect("create deck folder");
    std::fs::create_dir_all(&shared).expect("create library folder");
    std::fs::write(shared.join("device.inc"), "Rmodel out 0 2k\n").expect("write library include");
    let root = deck.join("root.cir");
    let source = "standalone\n.include \"device.inc\"\nV1 out 0 1\n.op\n.end\n";
    std::fs::write(&root, source).expect("write root deck");

    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state
            .open_test_deck(source.to_owned(), Some(root), "root.cir")
    );
    app.state.workbench.workspace = Workspace::Netlist;
    press_run(&mut app);
    assert!(
        !deck_run_started(&app),
        "nothing the deck is searched from holds device.inc"
    );
    let notice = app.state.prompt_text().unwrap_or_default().to_owned();
    assert!(
        notice.contains("device.inc"),
        "the notice says which include is missing: {notice}"
    );

    app.state
        .model
        .preferences
        .set_folders(FolderListPreference::LibraryFolders, vec![shared.clone()])
        .expect("a full path");
    press_run(&mut app);
    assert!(deck_run_started(&app));
    let resolution = app
        .state
        .code
        .code_workspace
        .include_resolutions
        .get("device.inc")
        .expect("the run retains where the include resolved");
    assert_eq!(
        resolution.stage(),
        rspice_core::netlist::IncludeSearchStage::LibraryPath(0)
    );
    let canonical = |path: &std::path::Path| std::fs::canonicalize(path).expect("fixture path");
    assert_eq!(
        resolution.resolved().parent().map(canonical),
        Some(canonical(&shared))
    );

    std::fs::remove_dir_all(directory).expect("remove library fixture");
}

/// A Run pressed while a Verilog-A source the deck's check reads is
/// compiling on another thread holds no frame: it waits, the status bar says
/// so, and frames keep coming while the compile runs. The first frame after
/// the compile ends runs the deck. Stop drops a run that waits.
#[test]
fn a_run_pressed_mid_compile_waits_while_frames_keep_coming() {
    use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame, themed_context};
    use rspice_app_runs::workbench::waiting::hold_compiling;

    const WAITING: &str = "Running the deck \u{b7} waiting for the Verilog-A compile";
    fn says(output: &egui::FullOutput, text: &str) -> bool {
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .is_some_and(|update| {
                update
                    .nodes
                    .iter()
                    .any(|(_, node)| node.label() == Some(text) || node.value() == Some(text))
            })
    }
    fn deck_runs(app: &RSpiceApp) -> usize {
        app.state
            .model
            .simulation
            .retained
            .executed_decks
            .iter()
            .count()
    }

    let mut app = netlist_session_with_deck(DECK, "amp_tb.cir");
    let ctx = themed_context();

    let compile = hold_compiling();
    press_run(&mut app);
    assert!(app.state.runs.waiting.deck_run);
    for _ in 0..3 {
        let output = frame(&ctx, &mut app, DESKTOP);
        assert!(says(&output, WAITING), "the status bar says what waits");
        assert!(!deck_run_started(&app));
    }
    drop(compile);
    let output = frame(&ctx, &mut app, DESKTOP);
    assert!(app.state.runs.waiting.is_empty());
    assert!(!says(&output, WAITING));
    assert!(
        deck_run_started(&app) || deck_runs(&app) == 1,
        "the deck's run starts once the compile ends"
    );

    let mut app = netlist_session_with_deck(DECK, "amp_tb.cir");
    let compile = hold_compiling();
    press_run(&mut app);
    assert!(app.state.runs.waiting.deck_run);
    assert!(app.command_key_enabled(ids::STOP_SIMULATION));
    app.execute_command_key(ids::STOP_SIMULATION);
    assert!(app.state.runs.waiting.is_empty());
    drop(compile);
    frame(&ctx, &mut app, DESKTOP);
    assert!(!deck_run_started(&app), "a stopped run never starts");
    assert_eq!(deck_runs(&app), 0, "a stopped run never starts");
}

/// Check and netlist on the Netlist workspace, while a deck is what Run
/// runs, checks that deck as Run would and starts nothing: neither the
/// plan's check of the schematic nor a run.
#[test]
fn check_and_netlist_checks_the_deck_in_front_and_starts_nothing() {
    let mut app = netlist_session_with_deck(DECK, "amp_tb.cir");
    assert_eq!(
        app.command_availability(ids::CHECK_AND_NETLIST),
        CommandAvailability::Available
    );

    app.execute_command_key(ids::CHECK_AND_NETLIST);

    let notice = app.state.prompt_text().unwrap_or_default().to_owned();
    assert!(
        notice.starts_with("amp_tb.cir checked \u{b7} 0 blocking errors"),
        "the notice names the deck it checked: {notice}"
    );
    assert!(
        app.state.runs.preflight.checking.is_none(),
        "the schematic is not checked"
    );
    assert!(!deck_run_started(&app), "the deck does not run");
    assert!(
        !app.state.runs.preflight.take_run_and_queue_request(),
        "the plan's run is not asked for"
    );
}

/// Check and netlist names the line a deck cannot run from, as Run would,
/// and starts nothing.
#[test]
fn check_and_netlist_names_the_line_a_deck_cannot_run_from() {
    let source = "standalone\nV1 out 0 1\nQ1 out out 0 qn2\n.op\n.end\n";
    let mut app = netlist_session_with_deck(source, "amp_tb.cir");

    app.execute_command_key(ids::CHECK_AND_NETLIST);

    let notice = app.state.prompt_text().unwrap_or_default().to_owned();
    assert!(
        notice.starts_with("amp_tb.cir cannot run: line 3"),
        "the notice names the deck and the line: {notice}"
    );
    assert!(!deck_run_started(&app), "nothing starts");
}

/// A library the deck names and nothing holds reads the same in the deck's
/// finding, Run's refusal and Check and netlist's, at the line that names it:
/// the file, then each folder looked in once.
#[test]
fn a_missing_library_reads_the_same_in_its_finding_and_its_refusals() {
    let source = "standalone\n.lib \"corners.lib\" tt\nV1 out 0 1\n.op\n.end\n";
    let said = "corners.lib not found (looked in the deck's folder, lib/, models/, ../lib/, \
                ../models/)";
    let mut replay =
        crate::feature_tests::replay::Replay::of(netlist_session_with_deck(source, "amp_tb.cir"));
    replay.frames(Vec::new());
    let findings = replay
        .app
        .state
        .code
        .netlist
        .diagnostics
        .iter()
        .map(|finding| (finding.source_line, finding.message.clone()))
        .collect::<Vec<_>>();
    assert_eq!(findings, [(Some(1), said.to_owned())]);

    press_run(&mut replay.app);
    assert!(!deck_run_started(&replay.app), "nothing starts");
    assert_eq!(
        replay.app.state.prompt_text().unwrap_or_default(),
        format!("amp_tb.cir did not run: line 2: {said}")
    );

    replay.app.execute_command_key(ids::CHECK_AND_NETLIST);
    assert_eq!(
        replay.app.state.prompt_text().unwrap_or_default(),
        format!("amp_tb.cir cannot run: line 2: {said}")
    );
}

/// Every file the deck names and nothing holds is a problem of its own at
/// the line that names it, and its include in the outline is marked
/// missing; Run refuses on the first.
#[test]
fn every_missing_file_is_a_problem_at_its_own_line() {
    let source = "standalone\nV1 out 0 1\nR1 out 0 1k\n.op\n.lib \"corners.lib\" tt\n\
                  .include models/missing.inc\n.end\n";
    let looked = "(looked in the deck's folder, lib/, models/, ../lib/, ../models/)";
    let mut replay =
        crate::feature_tests::replay::Replay::of(netlist_session_with_deck(source, "amp_tb.cir"));
    replay.frames(Vec::new());
    let painted = super::netlist_document::painted_text(&replay.frame(Vec::new()));

    let listed = rspice_app_state::workbench::app_state::diagnostic_store::listing::listing(
        &replay.app.state,
    );
    let mut rows = (0..listed.len())
        .filter_map(|index| listed.row(index))
        .map(|row| (row.location.clone(), row.message.to_owned()))
        .collect::<Vec<_>>();
    rows.sort();
    assert_eq!(
        rows,
        [
            (
                "line 5".to_owned(),
                format!("corners.lib not found {looked}")
            ),
            (
                "line 6".to_owned(),
                format!("models/missing.inc not found {looked}")
            ),
        ]
    );
    assert_eq!(
        painted.lines().filter(|line| *line == "missing").count(),
        2,
        "both includes are marked missing in the outline: {painted}"
    );

    press_run(&mut replay.app);
    assert_eq!(
        replay.app.state.prompt_text().unwrap_or_default(),
        format!("amp_tb.cir did not run: line 5: corners.lib not found {looked}")
    );
}

/// With the generated netlist in front the plan is what runs, and Check and
/// netlist is Simulate's check of it: the code feature leaves it alone.
#[test]
fn with_the_generated_netlist_in_front_check_and_netlist_is_the_plans() {
    let mut app = netlist_session_with_deck(DECK, "amp_tb.cir");
    assert_eq!(
        app.routed_command_key_enabled(ids::CHECK_AND_NETLIST),
        Some(true)
    );
    rspice_app_code::workbench::documents::netlist_document::show_generated_deck(
        &mut app.state.model,
        &mut app.state.code.netlist,
        &mut app.state.workbench,
    );

    assert_eq!(app.routed_command_key_enabled(ids::CHECK_AND_NETLIST), None);
}
