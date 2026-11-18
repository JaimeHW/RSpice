//! Run and Check and netlist while a deck is what Run runs: on the Netlist
//! workspace, the deck tab Run runs; on every other, the project's deck,
//! while the deck is the project's simulation.
//!
//! Away from the Netlist workspace the press first opens the project's deck
//! as the Netlist workspace does and makes it what Run runs there; a deck
//! whose file cannot be read holds the press, and the notice says why.
//!
//! The press checks the deck as a run reads it, includes read where they are,
//! and starts the run in the same press when the check finds nothing to stop
//! it. A deck the check stops does not run: the deck comes to the front,
//! where the editor marks its errors, and the notice names the deck and why.
//! A check that would read a Verilog-A source still compiling on another
//! thread waits for that compile beside the frame instead of holding it: the
//! status bar says so, the frame loop runs the deck once the compile ends,
//! and Stop drops it.
//!
//! Check and netlist checks the same deck the same way and starts nothing:
//! the notice says that nothing stops it running, or names the deck, the
//! line and why, with the deck in front.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::MessageId;
use crate::workbench::code::CodeFeature;
use crate::workbench::documents::deck_files;
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::run_view::{RunCodeFacts, RunView};
use crate::workbench::slots::{manual_deck_in, veriloga_compilation_in};
use crate::workbench::state::Workspace;
use crate::workbench::workflows::netlist_workflow::record_include_resolutions;
use rspice_app_runs::workbench::waiting;
use rspice_simulation::execution::PreparedRunMetadata;
use rspice_simulation::preparation::PreparationError;

/// The deck Run runs, checked: its name, its text and what the check
/// found.
struct CheckedDeck {
    name: String,
    text: String,
    result: Result<PreparedRunMetadata, PreparationError>,
}

/// What a check of the deck Run runs came to.
enum DeckCheck {
    /// There was no deck to check, or its files held the check, which the
    /// notices have said.
    Held,
    /// The check reached a Verilog-A source still compiling.
    Compiling,
    /// The deck checked.
    Checked(Box<CheckedDeck>),
}

/// Check the deck Run runs as a run reads it: its files looked at on disk
/// and saved first, so the check reads what their tabs show; where the check
/// found its includes recorded. What it prepared is retained for the run
/// that follows when `for_run`.
fn check_deck_run(cx: &mut Cx<'_, CodeFeature>, for_run: bool) -> DeckCheck {
    if cx.wb.workspace != Workspace::Netlist && !bring_project_deck(cx) {
        return DeckCheck::Held;
    }
    // A file changed on disk under edits holds the check until the reader
    // answers for it.
    match deck_files::sync_before_run(cx.model, &mut cx.state.netlist) {
        Ok(notices) => {
            for notice in notices {
                cx.wb.prompt(notice);
            }
        }
        Err(why) => {
            cx.wb.prompt(why);
            return DeckCheck::Held;
        }
    }
    let Some((name, text)) =
        manual_deck_in(&cx.state.netlist).map(|deck| (deck.name.to_owned(), deck.text.to_owned()))
    else {
        return DeckCheck::Held;
    };
    let result = {
        let code = RunCodeFacts {
            veriloga_compilation: veriloga_compilation_in(&cx.state.code_workspace),
            manual_deck: manual_deck_in(&cx.state.netlist),
        };
        let host = RunView::new(
            &mut *cx.model,
            &mut *cx.runs,
            &mut *cx.wb,
            &*cx.platform,
            &mut cx.peers,
            code,
            cx.commands,
        );
        let controller = &mut *cx.handles.run_controller;
        waiting::unless_compiling(|| {
            if for_run {
                controller.prepare_deck_run(&host)
            } else {
                crate::simulation::controller::SimulationController::build_prepared_snapshot(
                    &host,
                    crate::state::SimulationRunIntent::ManualDeck,
                )
                .map(|snapshot| snapshot.metadata())
            }
        })
    };
    let Some(result) = result else {
        return DeckCheck::Compiling;
    };
    if let Ok(metadata) = &result {
        record_include_resolutions(
            &mut cx.state.code_workspace.include_resolutions,
            &metadata.sealed_source_dependencies,
        );
    }
    DeckCheck::Checked(Box::new(CheckedDeck { name, text, result }))
}

/// Make the project's deck what Run runs, as Run away from the Netlist
/// workspace runs it: opened as the Netlist workspace opens it and brought
/// to the front there. Whether it is open; a deck whose file cannot be read
/// is not, and the notice says why, in the words Run's own reason uses.
fn bring_project_deck(cx: &mut Cx<'_, CodeFeature>) -> bool {
    deck_files::show_project_deck(
        cx.model,
        &mut cx.state.netlist,
        &mut cx.state.netlist_open_documents,
        cx.runs,
    );
    if cx.state.netlist.open_deck.is_some() {
        return true;
    }
    let messages = cx.wb.messages();
    let netlist = &cx.state.netlist;
    let said = |id: MessageId, deck: deck_files::MissingDeck| {
        messages.format(
            id,
            &[
                ("name", &deck.name),
                ("path", &deck.path.display().to_string()),
            ],
        )
    };
    let notice = deck_files::missing_deck(cx.model, netlist)
        .map(|deck| said(MessageId::NetlistDeckMissing, deck))
        .or_else(|| {
            deck_files::desktop_deck(cx.model, netlist)
                .map(|deck| said(MessageId::NetlistDeckOnTheDesktop, deck))
        })
        .or_else(|| {
            let id = netlist.unreadable_deck?;
            let deck = cx
                .model
                .workspace
                .content
                .decks
                .iter()
                .find(|deck| deck.id == id)?;
            Some(messages.format(MessageId::NetlistDeckUnreadable, &[("name", &deck.name())]))
        });
    if let Some(notice) = notice {
        cx.wb.prompt(notice);
    }
    false
}

/// Check the deck Run runs and run it, or say why it did not run.
pub fn request_deck_run(cx: &mut Cx<'_, CodeFeature>) {
    cx.runs.waiting.deck_run = false;
    if waiting::compiling() {
        cx.runs.waiting.deck_run = true;
        return;
    }
    let CheckedDeck { name, text, result } = match check_deck_run(cx, true) {
        DeckCheck::Held => return,
        DeckCheck::Compiling => {
            cx.runs.waiting.deck_run = true;
            return;
        }
        DeckCheck::Checked(checked) => *checked,
    };
    match result {
        Ok(_) => {
            cx.wb.run_press_route = Some(cx.wb.current_route());
            cx.model.simulation.execution.request_manual_deck_run();
        }
        Err(error) => refuse(cx, &name, Verdict::DidNotRun, &text, &error),
    }
}

/// Check and netlist while a deck is what Run runs: check the deck as Run
/// would and start nothing. The notice says the deck has nothing that stops
/// it running, or names the line and why, with the deck in front.
pub fn check_deck(cx: &mut Cx<'_, CodeFeature>) {
    let messages = cx.wb.messages();
    let CheckedDeck { name, text, result } = match check_deck_run(cx, false) {
        DeckCheck::Held => return,
        DeckCheck::Compiling => {
            cx.wb
                .prompt(messages.text(MessageId::NetlistDeckCheckWaitsForCompile));
            return;
        }
        DeckCheck::Checked(checked) => *checked,
    };
    match result {
        Ok(metadata) => {
            let warnings = messages.counted(
                metadata.advisories.len(),
                MessageId::NetlistDeckCheckedWarning,
                MessageId::NetlistDeckCheckedWarnings,
            );
            let tasks = messages.counted(
                metadata.task_count,
                MessageId::NetlistDeckCheckedTask,
                MessageId::NetlistDeckCheckedTasks,
            );
            let notice = messages.format(
                MessageId::NetlistDeckChecked,
                &[("name", &name), ("warnings", &warnings), ("tasks", &tasks)],
            );
            cx.model
                .record(LogSeverity::Info, LogSource::Netlist, notice.clone());
            cx.wb.prompt(notice);
        }
        Err(error) => refuse(cx, &name, Verdict::CannotRun, &text, &error),
    }
}

/// What a deck the check stopped is said to have done.
#[derive(Clone, Copy)]
enum Verdict {
    /// Run was pressed: the deck did not run.
    DidNotRun,
    /// Check and netlist was pressed: the deck cannot run.
    CannotRun,
}

/// Bring the deck `name` the check stopped to the front, where the editor
/// marks its errors, and say what came of it (`verdict`), on which line of
/// `text` and why: once the line is named, why is what the line's finding
/// says, as its Problems row says it.
fn refuse(
    cx: &mut Cx<'_, CodeFeature>,
    name: &str,
    verdict: Verdict,
    text: &str,
    error: &PreparationError,
) {
    deck_files::show_deck(cx.model, &mut cx.state.netlist);
    let messages = cx.wb.messages();
    let (at_line, without_line) = match verdict {
        Verdict::DidNotRun => (
            MessageId::NetlistDeckDidNotRunAt,
            MessageId::NetlistDeckDidNotRun,
        ),
        Verdict::CannotRun => (
            MessageId::NetlistDeckCannotRunAt,
            MessageId::NetlistDeckCannotRun,
        ),
    };
    let notice = match failure_line(error, text) {
        Some(line) => messages.format(
            at_line,
            &[
                ("name", name),
                ("line", &line.to_string()),
                ("error", error.finding()),
            ],
        ),
        None => messages.format(without_line, &[("name", name), ("error", error.message())]),
    };
    cx.model
        .record(LogSeverity::Error, LogSource::Netlist, notice.clone());
    cx.wb.prompt(notice);
}

/// The line of the deck the check's failure is on: the line it names, or
/// else the card of the instance it names first, as "Q1 (BJT) references
/// unknown model" names Q1.
fn failure_line(error: &PreparationError, text: &str) -> Option<usize> {
    error.line().or_else(|| {
        let instance = error.message().split_whitespace().next()?;
        text.lines()
            .position(|card| {
                card.split_whitespace()
                    .next()
                    .is_some_and(|first| first.eq_ignore_ascii_case(instance))
            })
            .map(|index| index + 1)
    })
}

/// Run the deck whose run waits, once no source its check reads is
/// compiling. Called once per frame; does nothing while the compile runs.
pub fn poll_waiting_deck_run(cx: &mut Cx<'_, CodeFeature>) {
    if cx.runs.waiting.deck_run && !waiting::compiling() {
        request_deck_run(cx);
    }
}
