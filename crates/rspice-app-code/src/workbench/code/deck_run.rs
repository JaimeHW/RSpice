//! Run and Check and netlist while a deck is what Run runs: on the Netlist
//! workspace, the deck tab Run runs; on every other, the project's deck,
//! while the deck is the project's simulation.
//!
//! Away from the Netlist workspace the press first opens the project's deck
//! as the Netlist workspace does and makes it what Run runs there; a deck
//! whose file cannot be read holds the press, and the notice says why.
//!
//! Run takes the deck as it stands at the press, its text shared rather than
//! copied, and has its run prepared away from the frame: includes read where
//! they are, the deck parsed, flattened and sealed. The status bar says the
//! run is preparing meanwhile, Stop drops it, and an edit made meanwhile is
//! not the run's. The run starts once it is prepared. A deck its preparation
//! stops does not run: the deck comes to the front, where the editor marks
//! its errors, and the notice names the deck and why. A preparation that
//! would read a Verilog-A source still compiling on another thread waits for
//! that compile beside the frame instead of holding it: the status bar says
//! so, the frame loop runs the deck once the compile ends, and Stop drops it.
//!
//! Check and netlist checks the deck the same way, in the press, and starts
//! nothing: the notice says that nothing stops it running, or names the
//! deck, the line and why, with the deck in front.
//!
//! Either leaves Problems current: it lists what the deck's text holds once
//! the check or the preparation has read it, whichever workspace is in front
//! afterwards.
//!
//! A run that needs more memory than the machine has free, and fits the
//! memory it has, waits once prepared where a run being prepared waits:
//! nothing dispatches, the status line and the band above the deck say what
//! it needs and what is free, and the band offers Run anyway and Stop. Run
//! anyway keeps the project's unsaved work as an autosave first, then starts
//! the run prepared; Stop drops it as it drops a run being prepared. A run
//! that needs more than the machine has is refused before it runs, as any
//! deck its preparation stops is.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::simulation::controller::DeckRunVerdict;
use crate::workbench::MessageId;
use crate::workbench::code::SourceFeature;
use crate::workbench::documents::deck_files;
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::run_view::{RunCodeFacts, RunView};
use crate::workbench::slots::{manual_deck_in, veriloga_compilation_in};
use crate::workbench::state::Workspace;
use crate::workbench::workflows::netlist_workflow::record_include_resolutions;
use rspice_app_runs::simulation::ManualDeckCopy;
use rspice_app_runs::workbench::slots::runs::PreparingDeck;
use rspice_app_runs::workbench::waiting;
use rspice_simulation::execution::PreparedRunMetadata;
use rspice_simulation::execution::run_memory::MemoryWarning;
use rspice_simulation::preparation::PreparationError;

/// The deck Run runs, checked: its name, its text and what the check
/// found.
struct CheckedDeck {
    name: String,
    text: crate::state::RevisedText,
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

/// Take the deck Run runs as it stands: its files looked at on disk and
/// saved first, so what is taken is what their tabs show. `None` when there
/// is no deck to take, or its files held the press, which the notices have
/// said; an empty deck is not taken either, and the notice says it is empty.
fn take_deck(cx: &mut Cx<'_, SourceFeature>) -> Option<ManualDeckCopy> {
    // A deck still being read is the deck Run runs once it is read.
    if let Some(refusal) = land_deck_read(cx) {
        cx.wb.prompt(refusal);
        return None;
    }
    if cx.wb.workspace != Workspace::Netlist && !bring_project_deck(cx) {
        return None;
    }
    // A file changed on disk under edits holds the press until the reader
    // answers for it.
    match deck_files::sync_before_run(cx.model, &mut cx.state.netlist, cx.wb.messages()) {
        Ok(notices) => {
            for notice in notices {
                cx.wb.prompt(notice);
            }
        }
        Err(why) => {
            cx.wb.prompt(why);
            return None;
        }
    }
    // The deck's text is shared with the deck, not copied.
    let deck = manual_deck_in(&cx.state.netlist).map(ManualDeckCopy::of)?;
    // An empty deck has nothing to read: it comes to the front and the
    // notice says it is empty, and nothing more.
    if deck.text.trim().is_empty() {
        deck_files::show_deck(cx.model, &mut cx.state.netlist);
        let notice = cx
            .wb
            .messages()
            .format(MessageId::NetlistDeckEmpty, &[("name", &deck.name)]);
        cx.model
            .record(LogSeverity::Error, LogSource::Netlist, notice.clone());
        cx.wb.prompt(notice);
        bring_findings_current(cx);
        return None;
    }
    Some(deck)
}

/// Check the deck Run runs as a run reads it, in the press, and record
/// where the check found its includes.
fn check_deck_run(cx: &mut Cx<'_, SourceFeature>) -> DeckCheck {
    let Some(deck) = take_deck(cx) else {
        return DeckCheck::Held;
    };
    let result = {
        let code = RunCodeFacts {
            veriloga_compilation: veriloga_compilation_in(&cx.state.source_editing),
            manual_deck: Some(deck.deck()),
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
        waiting::unless_compiling(|| {
            crate::simulation::controller::SimulationController::build_prepared_snapshot(
                &host,
                crate::state::SimulationRunIntent::ManualDeck,
            )
            .map(|snapshot| snapshot.metadata())
        })
    };
    let Some(result) = result else {
        return DeckCheck::Compiling;
    };
    if let Ok(metadata) = &result {
        record_include_resolutions(
            &mut cx.state.source_editing.include_resolutions,
            &metadata.sealed_source_dependencies,
        );
    }
    let ManualDeckCopy { name, text, .. } = deck;
    DeckCheck::Checked(Box::new(CheckedDeck { name, text, result }))
}

/// Make the project's deck what Run runs, as Run away from the Netlist
/// workspace runs it: opened as the Netlist workspace opens it and brought
/// to the front there. Whether it is open; a deck whose file cannot be read
/// is not, and the notice says why, in the words Run's own reason uses.
fn bring_project_deck(cx: &mut Cx<'_, SourceFeature>) -> bool {
    deck_files::show_project_deck(
        cx.model,
        &mut cx.state.netlist,
        &mut cx.state.netlist_open_documents,
        cx.runs,
        cx.wb.messages(),
    );
    if let Some(refusal) = land_deck_read(cx) {
        cx.wb.prompt(refusal);
    }
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

/// Take the deck being read, waiting for its read: why it did not read,
/// when it did not. What the page says of a deck that read is said.
fn land_deck_read(cx: &mut Cx<'_, SourceFeature>) -> Option<String> {
    match deck_files::land_deck_read(
        cx.model,
        &mut cx.state.netlist,
        &mut cx.state.netlist_open_documents,
        cx.runs,
        true,
        cx.wb.messages(),
    )? {
        Ok(said) => {
            cx.wb.prompt(said);
            None
        }
        Err(refusal) => Some(refusal),
    }
}

/// Make Problems say what the text the check read holds, whichever
/// workspace the press leaves in front.
fn bring_findings_current(cx: &mut Cx<'_, SourceFeature>) {
    crate::workbench::documents::netlist_document::bring_findings_current(
        cx.model,
        &mut cx.state.netlist,
        &cx.state.source_editing.include_resolutions,
        cx.wb.messages(),
    );
}

/// Take the deck Run runs and have its run prepared away from the frame
/// from it; [`poll_deck_run`] runs it once it is prepared, or says why it
/// did not run. Where the run is prepared in the press, that is now.
pub fn request_deck_run(cx: &mut Cx<'_, SourceFeature>) {
    cx.runs.waiting.deck_run = false;
    if waiting::compiling() {
        cx.runs.waiting.deck_run = true;
        return;
    }
    let Some(deck) = take_deck(cx) else {
        return;
    };
    {
        let code = RunCodeFacts {
            veriloga_compilation: veriloga_compilation_in(&cx.state.source_editing),
            manual_deck: Some(deck.deck()),
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
        cx.handles
            .run_controller
            .start_deck_run_preparation(&host, deck.clone());
    }
    let ManualDeckCopy { name, text, .. } = deck;
    cx.runs.preparing_deck = Some(PreparingDeck {
        name,
        text,
        memory: None,
    });
    cx.wb.run_press_route = Some(cx.wb.current_route());
    poll_deck_run(cx);
}

/// Take the verdict on the deck whose run is being prepared, once there is
/// one: the run starts, or the deck comes to the front and the notice says
/// why it did not run, as when the press prepared it. Called once per frame;
/// does nothing while nothing is prepared or the preparation goes on, and
/// while a run prepared waits for the reader to run it anyway.
pub fn poll_deck_run(cx: &mut Cx<'_, SourceFeature>) {
    if cx
        .runs
        .preparing_deck
        .as_ref()
        .is_none_or(|deck| deck.memory.is_some())
    {
        return;
    }
    let controller = &mut *cx.handles.run_controller;
    // A preparation the controller dropped, as a replaced design drops it,
    // has no verdict to wait for.
    let verdict = if controller.deck_run_preparing() {
        match controller.poll_deck_run_preparation() {
            Some(verdict) => Some(verdict),
            None => return,
        }
    } else {
        None
    };
    let Some(PreparingDeck { name, text, .. }) = cx.runs.preparing_deck.take() else {
        return;
    };
    match verdict {
        None => return,
        Some(DeckRunVerdict::Compiling) => {
            cx.runs.waiting.deck_run = true;
            return;
        }
        Some(DeckRunVerdict::Prepared(metadata)) => {
            record_include_resolutions(
                &mut cx.state.source_editing.include_resolutions,
                &metadata.sealed_source_dependencies,
            );
            cx.model.simulation.execution.request_manual_deck_run();
        }
        Some(DeckRunVerdict::NeedsMemory { metadata, warning }) => {
            record_include_resolutions(
                &mut cx.state.source_editing.include_resolutions,
                &metadata.sealed_source_dependencies,
            );
            hold_for_memory(cx, name, text, warning);
        }
        Some(DeckRunVerdict::Refused(error)) => {
            refuse(cx, &name, Verdict::DidNotRun, &text, &error);
        }
    }
    bring_findings_current(cx);
}

/// Hold the run of the deck `name`, prepared from `text` and retained, which
/// needs more memory than the machine has free (`warning`): it waits where a
/// run being prepared waits, and nothing of it dispatches until the reader
/// answers. The Netlist workspace comes to the front, where the band above
/// the deck asks; the log keeps what it said.
fn hold_for_memory(
    cx: &mut Cx<'_, SourceFeature>,
    name: String,
    text: crate::state::RevisedText,
    warning: MemoryWarning,
) {
    cx.model.record(
        LogSeverity::Warning,
        LogSource::Run,
        format!("{name}: {}", warning.message()),
    );
    cx.runs.preparing_deck = Some(PreparingDeck {
        name,
        text,
        memory: Some(warning),
    });
    if cx.wb.workspace != Workspace::Netlist {
        cx.execute(crate::workbench::framework::command_registry::ids::NETLIST_WORKSPACE);
    }
}

/// Run anyway: start the run held for memory, the run its press prepared,
/// as Run starts a run. The shell keeps the project's unsaved work as an
/// autosave as this returns, before the run dispatches. Whether a run was
/// held.
pub fn run_anyway(cx: &mut Cx<'_, SourceFeature>) -> bool {
    let Some(PreparingDeck { name, .. }) =
        cx.runs.preparing_deck.take_if(|deck| deck.memory.is_some())
    else {
        return false;
    };
    let said = cx
        .wb
        .messages()
        .format(MessageId::NetlistMemoryRunningAnyway, &[("name", &name)]);
    cx.model.record(LogSeverity::Info, LogSource::Run, said);
    cx.model.simulation.execution.request_manual_deck_run();
    true
}

/// Check and netlist while a deck is what Run runs: check the deck as Run
/// would and start nothing. The notice says the deck has nothing that stops
/// it running, or names the line and why, with the deck in front.
pub fn check_deck(cx: &mut Cx<'_, SourceFeature>) {
    let messages = cx.wb.messages();
    let CheckedDeck { name, text, result } = match check_deck_run(cx) {
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
    bring_findings_current(cx);
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
    cx: &mut Cx<'_, SourceFeature>,
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
    // A deck past one of the engine's limits is said to be, in the words
    // its finding says it in; past one sized to the machine, in the words
    // the preparation said it in.
    let refusal = error
        .catalogued_limit()
        .map(|refusal| messages.resource_refusal(refusal));
    let notice = match failure_line(error, text) {
        Some(line) => messages.format(
            at_line,
            &[
                ("name", name),
                ("line", &line.to_string()),
                ("error", refusal.as_deref().unwrap_or(error.finding())),
            ],
        ),
        None => messages.format(
            without_line,
            &[
                ("name", name),
                ("error", refusal.as_deref().unwrap_or(error.message())),
            ],
        ),
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
pub fn poll_waiting_deck_run(cx: &mut Cx<'_, SourceFeature>) {
    if cx.runs.waiting.deck_run && !waiting::compiling() {
        request_deck_run(cx);
    }
}
