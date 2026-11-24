//! Run on a deck weighed against the memory of the machine it runs on, a
//! machine the tests pretend: a run that needs more than is free waits where
//! a run being prepared waits, for Run anyway or Stop, its numbers on the
//! status line and in the band above the deck, which says when no autosave
//! keeps the unsaved work first; one that needs more than is installed is
//! refused before it runs, in short on the status line and whole in the band
//! until the deck is edited or run again; and the check of a deck is held to
//! the limits its run is held to.

#![cfg(not(target_arch = "wasm32"))]

use std::fmt::Write as _;

use egui::accesskit::Role;
use rspice_app_code::workbench::documents::deck_files;
use rspice_app_runs::simulation::controller::DeckPreparationHold;
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::{ProjectDeck, SimulationRunIntent, StoredDeck, TextEncoding};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;
use rspice_simulation::execution::run_memory::{
    FieldPolicy, MachineMemory, MemoryShortfall, MemoryWarning, POLICY, RunOf, deck_limits,
    forget_machine_memory_here, pretend_machine_memory_here,
};

use crate::feature_tests::replay::Replay;
use crate::workbench::RSpiceApp;

const MB: u64 = 1024 * 1024;
const GB: u64 = 1024 * MB;

/// A deck of `resistors` resistors in a chain, driven by one source.
fn chain(resistors: usize) -> String {
    let mut deck = String::from("chain\nV1 n0 0 1\n");
    for i in 0..resistors {
        let _ = writeln!(deck, "R{i} n{i} n{} 1k", i + 1);
    }
    deck.push_str(".op\n.end\n");
    deck
}

/// The application on the Netlist workspace with `source` open as the deck
/// file `name`, the schematic's components gone, on the machine `memory`.
fn session(source: &str, name: &str, memory: MachineMemory) -> Replay {
    pretend_machine_memory_here(memory);
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    assert!(app.state.open_test_deck(source.to_owned(), None, name));
    app.state.workbench.workspace = Workspace::Netlist;
    Replay::of(app)
}

/// The application on the Netlist workspace with `source` kept in the
/// project as the deck `name` and open, the project saved to `file`, on the
/// machine `memory`.
fn kept_session(source: &str, name: &str, file: &std::path::Path, memory: MachineMemory) -> Replay {
    pretend_machine_memory_here(memory);
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let deck = ProjectDeck::stored(StoredDeck {
        name: name.to_owned(),
        text: source.into(),
        encoding: TextEncoding::Utf8,
        files: Vec::new(),
    });
    let id = deck.id;
    app.state.model.workspace.content.decks.push(deck);
    let messages = app.state.workbench.messages();
    deck_files::open_deck_by_id(
        &mut app.state.model,
        &mut app.state.code.netlist,
        &mut app.state.code.netlist_open_documents,
        &mut app.state.runs,
        id,
        messages,
    )
    .expect("the deck opens");
    app.state.workbench.activate(Workspace::Netlist);
    assert!(app.state.save_project_to_path(file), "the project saves");
    Replay::of(app)
}

/// Whether a run of the deck was asked for or has started.
fn run_started(app: &RSpiceApp) -> bool {
    let simulation = &app.state.model.simulation;
    (simulation.execution.trigger_simulation
        && simulation.execution.run_intent == SimulationRunIntent::ManualDeck)
        || simulation.execution.has_active_execution()
        || !simulation.retained.runs.is_empty()
}

/// Whether a frame's accessibility tree says `text`.
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

/// Everything a frame's accessibility tree says that holds `part`, for a
/// failure to show what was said instead.
fn said_with(output: &egui::FullOutput, part: &str) -> Vec<String> {
    output
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| update.nodes.iter())
        .flat_map(|(_, node)| [node.label(), node.value()])
        .flatten()
        .filter(|said| said.contains(part))
        .map(str::to_owned)
        .collect()
}

/// What the run of the deck being prepared needs beyond the memory free,
/// when it waits for Run anyway, with the deck's name.
fn held(app: &RSpiceApp) -> Option<(String, MemoryWarning)> {
    let deck = app.state.runs.preparing_deck.as_ref()?;
    Some((deck.name.clone(), deck.memory?))
}

/// What the deck's run, refused for memory, needs against what the machine
/// has, while the band above the deck says it.
fn refused(app: &RSpiceApp) -> Option<MemoryShortfall> {
    Some(app.state.code.netlist.memory_refusal?.shortfall)
}

/// Type `text` at the start of the deck in front, as the reader types it.
fn type_at_start(replay: &mut Replay, text: &str) {
    let editor = rspice_app_code::workbench::documents::netlist_document::editor_id_in(
        &replay.app.state.model,
        &replay.app.state.code.netlist,
    );
    rspice_app_code::workbench::documents::text_editor_commands::queue_caret_char_index(
        replay.ctx(),
        editor,
        0,
    );
    replay
        .ctx()
        .memory_mut(|memory| memory.request_focus(editor));
    replay.frame(Vec::new());
    replay.frame(Vec::new());
    replay.frame(vec![egui::Event::Text(text.to_owned())]);
    replay.frame(Vec::new());
}

/// A machine with memory installed for the deck and almost none free: Run
/// prepares the run, and it waits where a run being prepared waits. Nothing
/// dispatches; the status line says what the run needs and what is free,
/// the band above the deck says it, and Run anyway there starts the run
/// prepared.
#[test]
fn a_run_needing_more_than_is_free_waits_for_run_anyway() {
    let busy = MachineMemory {
        installed_bytes: 16 * GB,
        free_bytes: MB,
    };
    let mut replay = session(&chain(600), "chain.cir", busy);

    replay.app.execute_command_key(ids::RUN_SIMULATION);
    let (name, warning) = held(&replay.app).expect("the run waits for an answer");
    assert_eq!(name, "chain.cir");
    assert_eq!(warning.memory, busy);
    assert!(warning.needed_bytes > busy.free_bytes);
    assert!(!run_started(&replay.app), "nothing dispatches");
    assert!(replay.app.command_key_enabled(ids::STOP_SIMULATION));
    assert!(!replay.app.command_key_enabled(ids::RUN_SIMULATION));
    let output = replay.frame(Vec::new());
    // The status line marks it a warning, and says so to a screen reader.
    let status = format!("Warning: {}", warning.status());
    assert!(
        says(&output, &status),
        "the status line says {status:?}; said {:?}",
        said_with(&output, "free")
    );
    let band = warning.numbers_of(RunOf::Deck);
    assert!(
        band.starts_with("This deck's 601 devices need about "),
        "{band}"
    );
    assert!(
        !said_with(&output, &band).is_empty(),
        "the band says it; said {:?}",
        said_with(&output, "devices")
    );
    for _ in 0..3 {
        replay.frame(Vec::new());
    }
    assert!(!run_started(&replay.app), "still nothing dispatches");

    replay.click("Run anyway", Role::Button);
    assert!(replay.app.state.runs.preparing_deck.is_none());
    assert!(run_started(&replay.app), "Run anyway starts the run");
    forget_machine_memory_here();
}

/// The same run prepared away from the frame, as the application prepares
/// one: once its worker has prepared it, it waits in Preparing for Run
/// anyway instead of starting.
#[test]
fn a_run_prepared_away_from_the_frame_waits_for_run_anyway() {
    let busy = MachineMemory {
        installed_bytes: 16 * GB,
        free_bytes: MB,
    };
    let mut replay = session(&chain(600), "chain.cir", busy);
    let DeckPreparationHold { release, ended } = replay
        .app
        .simulation_controller
        .hold_next_deck_run_preparation();

    replay.app.execute_command_key(ids::RUN_SIMULATION);
    assert!(replay.app.state.runs.preparing_deck.is_some());
    assert_eq!(held(&replay.app), None, "it is still preparing");
    drop(release);
    // Disconnects once the worker has ended.
    let _ = ended.recv();
    replay.frame(Vec::new());

    let (_, warning) = held(&replay.app).expect("the prepared run waits for an answer");
    assert_eq!(warning.memory, busy);
    for _ in 0..3 {
        replay.frame(Vec::new());
    }
    assert!(!run_started(&replay.app), "nothing dispatches");
    replay.click("Run anyway", Role::Button);
    assert!(run_started(&replay.app), "Run anyway starts the run");
    forget_machine_memory_here();
}

/// Stop, in the band or as the command, drops the run held for memory:
/// nothing of it ever runs, and the notice says so.
#[test]
fn stop_drops_a_run_held_for_memory() {
    let busy = MachineMemory {
        installed_bytes: 16 * GB,
        free_bytes: MB,
    };
    let mut replay = session(&chain(600), "chain.cir", busy);

    replay.app.execute_command_key(ids::RUN_SIMULATION);
    assert!(held(&replay.app).is_some());
    replay.click("Stop", Role::Button);

    assert!(replay.app.state.runs.preparing_deck.is_none());
    assert_eq!(
        replay.app.state.prompt_text(),
        Some("Stopped chain.cir before it ran.")
    );
    for _ in 0..3 {
        replay.frame(Vec::new());
    }
    assert!(!run_started(&replay.app), "a stopped run never starts");
    assert!(replay.app.command_key_enabled(ids::RUN_SIMULATION));
    forget_machine_memory_here();
}

/// A machine with less memory installed than the run needs: Run refuses the
/// deck before anything runs. The status line says in short what it needs
/// and what the machine has; the band above the deck says it of the deck in
/// a whole sentence, and the log keeps the whole refusal. The band goes once
/// the deck is edited, and once it runs again.
#[test]
fn a_run_needing_more_than_is_installed_is_refused_before_it_runs() {
    let small = MachineMemory {
        installed_bytes: MB,
        free_bytes: MB / 2,
    };
    let mut replay = session(&chain(600), "chain.cir", small);

    replay.app.execute_command_key(ids::RUN_SIMULATION);

    assert!(replay.app.state.runs.preparing_deck.is_none());
    assert!(!run_started(&replay.app), "nothing runs");
    let shortfall = refused(&replay.app).expect("the band says the run was refused");
    assert_eq!(shortfall.devices, 601);
    assert_eq!(shortfall.installed_bytes, MB);
    assert_eq!(
        replay.app.state.prompt_text().unwrap_or_default(),
        format!("chain.cir did not run: {}", shortfall.status())
    );
    let band = shortfall.numbers_of(RunOf::Deck);
    assert!(
        band.starts_with("This deck's 601 devices need about ")
            && band.ends_with("of memory, more than the 1 MB this computer has."),
        "{band}"
    );
    let output = replay.frame(Vec::new());
    assert!(
        says(&output, &band),
        "the band says it; said {:?}",
        said_with(&output, "devices")
    );
    let whole = format!(
        "chain.cir did not run: {}",
        shortfall.message_of(RunOf::Deck)
    );
    assert!(
        whole.ends_with(
            "more than the 1 MB this computer has. Run it on a computer with more memory."
        ),
        "{whole}"
    );
    assert!(
        replay
            .app
            .state
            .model
            .log_buffer
            .entries()
            .any(|entry| entry.message == whole),
        "the log keeps {whole:?}"
    );

    type_at_start(&mut replay, "*");
    let output = replay.frame(Vec::new());
    assert!(
        refused(&replay.app).is_none(),
        "the band goes once the deck is edited"
    );
    assert!(!says(&output, &band));

    replay.app.execute_command_key(ids::RUN_SIMULATION);
    assert!(refused(&replay.app).is_some(), "refused again");
    pretend_machine_memory_here(MachineMemory {
        installed_bytes: 16 * GB,
        free_bytes: 8 * GB,
    });
    replay.app.execute_command_key(ids::RUN_SIMULATION);
    assert!(
        refused(&replay.app).is_none(),
        "the band goes as it runs again"
    );
    assert!(
        run_started(&replay.app),
        "the deck runs on a machine that holds it"
    );
    forget_machine_memory_here();
}

/// Run anyway keeps the project's unsaved work as an autosave first (Run
/// saves a deck file's own edits before it takes the deck). With autosave
/// off nothing keeps it: while a deck the project keeps holds unsaved typing,
/// the band of a run held for memory says to save first, and with none it
/// says nothing of it. With autosave on, the project saved to a file
/// keeps it.
#[test]
fn a_run_held_for_memory_says_when_unsaved_work_is_not_kept() {
    let busy = MachineMemory {
        installed_bytes: 16 * GB,
        free_bytes: MB,
    };
    let folder = unique_temp_dir("held-unsaved-work");
    let mut replay = kept_session(
        &chain(600),
        "chain.cir",
        &folder.join("held.rspiceproj"),
        busy,
    );
    replay.app.state.platform.autosave_minutes = 0;

    replay.app.execute_command_key(ids::RUN_SIMULATION);
    let (_, warning) = held(&replay.app).expect("the run waits for an answer");
    let band = warning.numbers_of(RunOf::Deck);
    let unkept = format!("{band} Save first.");
    let output = replay.frame(Vec::new());
    assert!(
        says(&output, &band),
        "nothing unsaved, so nothing said of it; said {:?}",
        said_with(&output, "devices")
    );

    replay.app.execute_command_key(ids::STOP_SIMULATION);
    assert!(held(&replay.app).is_none());
    type_at_start(&mut replay, "*");
    replay.app.execute_command_key(ids::RUN_SIMULATION);
    assert!(held(&replay.app).is_some(), "the edited deck waits too");
    replay.frame(Vec::new());
    let output = replay.frame(Vec::new());
    assert!(
        says(&output, &unkept),
        "the band says to save first; said {:?}",
        said_with(&output, "devices")
    );

    replay.app.state.platform.autosave_minutes = 5;
    let output = replay.frame(Vec::new());
    assert!(
        says(&output, &band),
        "an autosave keeps it; said {:?}",
        said_with(&output, "devices")
    );
    forget_machine_memory_here();
    let _ = std::fs::remove_dir_all(folder);
}

/// The Netlist check of a deck and a run of it are held to the same limits,
/// sized to the machine. A deck past the line count the engine ships with
/// checks with no finding on a machine that holds it, and runs; on a machine
/// whose memory holds fewer lines, the check's finding and Run's refusal
/// say the same: it needs more memory than the machine has.
#[test]
fn the_check_and_the_run_hold_a_deck_to_the_same_limits() {
    let shipped = rspice_core::ResourceLimits::default().max_netlist_lines;
    let per_line = POLICY
        .iter()
        .find_map(|(kind, policy)| match (kind, policy) {
            (rspice_core::ResourceKind::NetlistLines, FieldPolicy::Sized { bytes_each }) => {
                Some(*bytes_each)
            }
            _ => None,
        })
        .expect("the line count is sized");
    // A machine that holds a fifth again the shipped line count, and a deck
    // one line past what it holds.
    let small = MachineMemory {
        installed_bytes: (shipped as u64 + shipped as u64 / 5) * per_line,
        free_bytes: GB,
    };
    let lines = deck_limits(Some(small)).max_netlist_lines + 1;
    assert!(lines > shipped + 1);
    let deck = format!(
        "tall\n{}V1 a 0 1\nR1 a 0 1k\n.op\n.end\n",
        "*\n".repeat(lines)
    );

    let roomy = MachineMemory {
        installed_bytes: 32 * GB,
        free_bytes: 16 * GB,
    };
    let mut replay = session(&deck, "tall.cir", roomy);
    replay.app.execute_command_key(ids::RUN_SIMULATION);
    let findings = &replay.app.state.code.netlist.diagnostics;
    assert!(
        findings
            .iter()
            .all(|finding| !finding.message.contains("at most")
                && !finding.message.contains("memory")),
        "{findings:#?}"
    );
    assert!(
        run_started(&replay.app),
        "the deck runs: {:?}; held {:?}",
        replay.app.state.prompt_text(),
        held(&replay.app)
    );
    drop(replay);

    let mut replay = session(&deck, "tall.cir", small);
    replay.app.execute_command_key(ids::RUN_SIMULATION);
    assert!(!run_started(&replay.app), "the deck does not run");
    let found = replay
        .app
        .state
        .code
        .netlist
        .diagnostics
        .iter()
        .map(|finding| finding.message.clone())
        .find(|message| message.contains("of memory this computer has"))
        .expect("the check finds the deck too large for the machine");
    let notice = replay.app.state.prompt_text().unwrap_or_default();
    assert!(
        notice.starts_with("tall.cir did not run: ") && notice.ends_with(&found),
        "Run refuses it as the check found it: {notice}"
    );
    forget_machine_memory_here();
}
