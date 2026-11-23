//! Run on a deck weighed against the memory of the machine it runs on, a
//! machine the tests pretend: a run that needs more than is free waits for
//! Run anyway or Stop, its numbers on the status line and in the band above
//! the deck; one that needs more than is installed is refused before it
//! runs; and the check of a deck is held to the limits its run is held to.

#![cfg(not(target_arch = "wasm32"))]

use std::fmt::Write as _;

use egui::accesskit::Role;
use rspice_app_state::state::SimulationRunIntent;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;
use rspice_simulation::execution::run_memory::{
    FieldPolicy, MachineMemory, POLICY, deck_limits, forget_machine_memory_here,
    pretend_machine_memory_here,
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

/// A machine with memory installed for the deck and almost none free: Run
/// prepares the run and holds it. Nothing dispatches; the status line says
/// what the run needs and what is free, the band above the deck says it in
/// full, and Run anyway there starts the run prepared.
#[test]
fn a_run_needing_more_than_is_free_waits_for_run_anyway() {
    let busy = MachineMemory {
        installed_bytes: 16 * GB,
        free_bytes: MB,
    };
    let mut replay = session(&chain(600), "chain.cir", busy);

    replay.app.execute_command_key(ids::RUN_SIMULATION);
    let hold = replay
        .app
        .state
        .runs
        .memory_hold
        .clone()
        .expect("the run waits for an answer");
    assert_eq!(hold.name, "chain.cir");
    assert_eq!(hold.warning.memory, busy);
    assert!(hold.warning.needed_bytes > busy.free_bytes);
    assert!(!run_started(&replay.app), "nothing dispatches");
    assert!(replay.app.command_key_enabled(ids::STOP_SIMULATION));
    let output = replay.frame(Vec::new());
    // The status line marks it a warning, and says so to a screen reader.
    let status = format!("Warning: {}", hold.warning.status());
    assert!(
        says(&output, &status),
        "the status line says {status:?}; said {:?}",
        said_with(&output, "free")
    );
    assert!(says(&output, &hold.warning.numbers()), "the band says it");
    for _ in 0..3 {
        replay.frame(Vec::new());
    }
    assert!(!run_started(&replay.app), "still nothing dispatches");

    replay.click("Run anyway", Role::Button);
    assert_eq!(replay.app.state.runs.memory_hold, None);
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
    assert!(replay.app.state.runs.memory_hold.is_some());
    replay.click("Stop", Role::Button);

    assert_eq!(replay.app.state.runs.memory_hold, None);
    assert_eq!(
        replay.app.state.prompt_text(),
        Some("Stopped the run of chain.cir. Nothing ran.")
    );
    for _ in 0..3 {
        replay.frame(Vec::new());
    }
    assert!(!run_started(&replay.app), "a stopped run never starts");
    assert!(replay.app.command_key_enabled(ids::RUN_SIMULATION));
    forget_machine_memory_here();
}

/// A machine with less memory installed than the run needs: Run refuses the
/// deck before anything runs, saying what it needs and what the machine has.
#[test]
fn a_run_needing_more_than_is_installed_is_refused_before_it_runs() {
    let small = MachineMemory {
        installed_bytes: MB,
        free_bytes: MB / 2,
    };
    let mut replay = session(&chain(600), "chain.cir", small);

    replay.app.execute_command_key(ids::RUN_SIMULATION);

    assert_eq!(replay.app.state.runs.memory_hold, None);
    assert!(!run_started(&replay.app), "nothing runs");
    let notice = replay
        .app
        .state
        .prompt_text()
        .unwrap_or_default()
        .to_owned();
    assert!(
        notice.starts_with(
            "chain.cir did not run: This design has 601 devices. Running it needs about "
        ),
        "{notice}"
    );
    assert!(
        notice.ends_with("of memory, more than the 1 MB this computer has. Run it on a computer with more memory."),
        "{notice}"
    );
    forget_machine_memory_here();
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
        replay.app.state.runs.memory_hold
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
