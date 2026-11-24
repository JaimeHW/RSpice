//! Run and Check and netlist while a deck is what Run runs: on the Netlist
//! workspace, the deck tab Run runs; on every other, the project's deck,
//! while the deck is the project's simulation.
//!
//! Away from the Netlist workspace the press first opens the project's deck
//! as the Netlist workspace does and makes it what Run runs there; a deck
//! whose file cannot be read holds the press, and the notice says why.
//!
//! Either press takes the deck as it stands, its text shared rather than
//! copied, and has it prepared away from the frame as a run reads it: its
//! files holding edits saved, includes read where they are, the deck parsed,
//! flattened and sealed. The status bar says it is preparing meanwhile, Stop
//! drops it, and an edit made meanwhile is not what is prepared. A save that
//! fails holds the press, and the notice says why.
//!
//! The press looks at the deck's files on disk first, by their length and
//! time. A file whose length or time moved since it was read is read away
//! from the frame, and the press waits for it where a deck being prepared
//! waits: a file changed on disk whose tab holds no edits is read again
//! into its tab, and the deck is taken as it then stands. A file changed on
//! disk while its tab held edits holds the press there, the notice saying
//! so, until the reader reloads it or keeps theirs above the deck, and the
//! press then goes on with the text they chose; Stop drops it.
//!
//! Run starts the run once it is prepared. A deck its preparation stops does
//! not run: the deck comes to the front, where the editor marks its errors,
//! and the notice names the deck and why. A preparation that would read a
//! Verilog-A source still compiling on another thread waits for that compile
//! beside the frame instead of holding it: the status bar says so, the frame
//! loop runs the deck once the compile ends, and Stop drops it.
//!
//! Check and netlist starts nothing: the notice says that nothing stops the
//! deck running, or names the deck, the line and why, with the deck in front.
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
//! the run prepared; Stop drops it as it drops a run being prepared. Where
//! no autosave keeps that work, the band says to save first. A run that
//! needs more than the machine has is refused before it runs: the status
//! line says it in short, and the band above the deck says what it needs
//! against what the machine has until the deck is edited or pressed again.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::simulation::controller::{BeforePreparing, DeckAction, DeckRunVerdict};
use crate::workbench::MessageId;
use crate::workbench::code::SourceFeature;
use crate::workbench::documents::deck_files;
use crate::workbench::documents::netlist_document::MemoryRefusal;
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::run_view::{RunCodeFacts, RunView};
use crate::workbench::slots::{manual_deck_in, veriloga_compilation_in};
use crate::workbench::state::Workspace;
use crate::workbench::workflows::netlist_workflow::record_include_resolutions;
use rspice_app_runs::simulation::ManualDeckCopy;
use rspice_app_runs::workbench::slots::runs::{DiskWait, PreparingDeck};
use rspice_app_runs::workbench::waiting;
use rspice_simulation::execution::run_memory::{MemoryWarning, RunOf};
use rspice_simulation::preparation::PreparationError;

/// Take the deck Run runs as it stands for `action`: its files looked at on
/// disk when `look`, and those holding edits as the press took them, to be
/// saved before the deck is prepared. `None` when there is no deck to take,
/// or its files held the press, which the notices have said, or the press
/// waits for them on disk; an empty deck is not taken either, and the
/// notice says it is empty.
fn take_deck(
    cx: &mut Cx<'_, SourceFeature>,
    action: DeckAction,
    look: bool,
) -> Option<(ManualDeckCopy, Vec<deck_files::EditedFile>)> {
    // A deck still being read is the deck Run runs once it is read.
    if let Some(refusal) = land_deck_read(cx) {
        cx.wb.prompt(refusal);
        return None;
    }
    if cx.wb.workspace != Workspace::Netlist && !bring_project_deck(cx) {
        return None;
    }
    // A file whose length or time moved since it was read holds the press
    // until it is read, and one changed on disk under edits until the reader
    // answers for it.
    let edited = match deck_files::sync_before_run(
        cx.model,
        &mut cx.state.netlist,
        &mut cx.state.disk_looks,
        look,
        cx.wb.messages(),
    ) {
        Ok(deck_files::BeforeRun {
            notices,
            edited,
            waits,
        }) => {
            for notice in notices {
                cx.wb.prompt(notice);
            }
            if let Some(waits) = waits {
                wait_on_disk(cx, action, waits);
                return None;
            }
            edited
        }
        Err(why) => {
            cx.wb.prompt(why);
            return None;
        }
    };
    // The deck's text is shared with the deck, not copied.
    let deck = manual_deck_in(&cx.state.netlist).map(ManualDeckCopy::of)?;
    // An empty deck has nothing to read: it comes to the front and the
    // notice says it is empty, and nothing more.
    if is_blank(&deck.text) {
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
    Some((deck, edited))
}

/// Hold the press of `action` on the deck Run runs until what it waits for
/// on the deck's files on disk is answered: it waits where a deck being
/// prepared waits, nothing of it prepared and Stop dropping it, and
/// [`poll_deck_run`] presses again once the reads are taken in or the reader
/// has answered. A file changed on disk under edits says so now.
fn wait_on_disk(
    cx: &mut Cx<'_, SourceFeature>,
    action: DeckAction,
    waits: deck_files::WaitsOnDisk,
) {
    let Some(id) = cx.state.netlist.open_deck.as_ref().map(|deck| deck.id) else {
        return;
    };
    let Some(ManualDeckCopy { name, text, .. }) =
        manual_deck_in(&cx.state.netlist).map(ManualDeckCopy::of)
    else {
        return;
    };
    let disk = match waits {
        deck_files::WaitsOnDisk::Reading => DiskWait::Reading(id),
        deck_files::WaitsOnDisk::Answer(why) => {
            cx.wb.prompt(why);
            DiskWait::Answer(id)
        }
    };
    cx.runs.preparing_deck = Some(Box::new(PreparingDeck {
        name,
        text,
        action,
        memory: None,
        disk: Some(disk),
    }));
    if action == DeckAction::Run {
        cx.wb.run_press_route = Some(cx.wb.current_route());
    }
}

/// Whether `text` holds nothing but blanks, read a line at a time from its
/// start, so a text an editor holds in pieces is not put in one piece on
/// the frame to find its first word.
fn is_blank(text: &crate::state::RevisedText) -> bool {
    let mut at = 0;
    while at < text.len() {
        let end = text.line_end_at(at);
        if !text.slice(at..end).trim().is_empty() {
            return false;
        }
        at = end + 1;
    }
    true
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

/// What the worker does before it prepares the deck: save `edited`, the
/// deck's files a press took with edits, whose save the frame loop takes in
/// ([`land_deck_saves`]) whether or not the preparation goes on. The
/// preparation goes on once every file is written.
fn save_before_preparing(
    cx: &mut Cx<'_, SourceFeature>,
    edited: Vec<deck_files::EditedFile>,
) -> BeforePreparing {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (cx, edited);
        Box::new(|| true)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let messages = cx.wb.messages();
        let (sender, saves) = std::sync::mpsc::channel();
        if !edited.is_empty() {
            cx.state.deck_saves.push(saves);
        }
        // A deck with no edits still waits for a save in flight, so it reads
        // its files once they are written.
        Box::new(move || {
            let saved = deck_files::save_edited_files(edited, messages);
            let written = saved.refusal().is_none();
            let _ = sender.send(saved);
            written
        })
    }
}

/// Take in the looks at the deck's files on disk that have ended: a file
/// changed on disk is read again into a tab with no edits, or shown on one
/// holding edits, as when the page is shown, and the notice says so.
fn land_disk_looks(cx: &mut Cx<'_, SourceFeature>) {
    for notice in deck_files::land_disk_looks(
        cx.model,
        &mut cx.state.netlist,
        &mut cx.state.disk_looks,
        cx.wb.messages(),
    ) {
        cx.wb.prompt(notice);
    }
}

/// Take in what the saves of decks' files made before their preparation
/// came to: the files are held to what was written, and a save that failed
/// says why, as a save does.
fn land_deck_saves(cx: &mut Cx<'_, SourceFeature>) {
    for saved in cx.state.deck_saves.take_landed() {
        if let Some(why) = deck_files::land_saved_before_run(&mut cx.state.netlist, saved) {
            cx.wb.prompt(why);
        }
    }
}

/// The press of Run or of Check and netlist: take the deck Run runs and have
/// it prepared for `action` away from the frame; [`poll_deck_run`] acts on
/// how its preparation ends. Where the deck is prepared in the press, that
/// is now.
fn press(cx: &mut Cx<'_, SourceFeature>, action: DeckAction) {
    // A refusal for memory stands until the deck is pressed again.
    cx.state.netlist.memory_refusal = None;
    press_looking(cx, action, true);
}

/// [`press`], the deck's files looked at on disk when `look`; else a file
/// whose length or time moved since it was read is taken as its tab holds
/// it.
fn press_looking(cx: &mut Cx<'_, SourceFeature>, action: DeckAction, look: bool) {
    let Some((deck, edited)) = take_deck(cx, action, look) else {
        return;
    };
    let before = save_before_preparing(cx, edited);
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
            .start_deck_preparation(&host, deck.clone(), action, before);
    }
    let ManualDeckCopy { name, text, .. } = deck;
    cx.runs.preparing_deck = Some(Box::new(PreparingDeck {
        name,
        text,
        action,
        memory: None,
        disk: None,
    }));
    if action == DeckAction::Run {
        cx.wb.run_press_route = Some(cx.wb.current_route());
    }
    poll_deck_run(cx);
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
    press(cx, DeckAction::Run);
}

/// Check and netlist while a deck is what Run runs: check the deck as Run
/// would and start nothing. The notice says the deck has nothing that stops
/// it running, or names the line and why, with the deck in front.
pub fn check_deck(cx: &mut Cx<'_, SourceFeature>) {
    press(cx, DeckAction::Check);
}

/// Take how the deck's preparation ended, once it has: Run's run starts,
/// Check says what it found, or the deck comes to the front and the notice
/// says why it cannot run, as when the press prepared it. Called once per
/// frame; takes in the saves made before preparations and the looks at the
/// disk that ended, presses again for a press that waited on disk once it
/// is answered, and does nothing more while nothing is prepared or the
/// preparation goes on, or while a run prepared waits for the reader to run
/// it anyway.
pub fn poll_deck_run(cx: &mut Cx<'_, SourceFeature>) {
    land_deck_saves(cx);
    land_disk_looks(cx);
    if let Some(disk) = cx.runs.preparing_deck.as_ref().and_then(|deck| deck.disk) {
        answer_disk_wait(cx, disk);
        return;
    }
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
    let Some(PreparingDeck {
        name, text, action, ..
    }) = cx.runs.preparing_deck.take().map(|deck| *deck)
    else {
        return;
    };
    match verdict {
        // A save that held the preparation said why as it was taken in.
        None | Some(DeckRunVerdict::Held) => return,
        Some(DeckRunVerdict::Compiling) => {
            match action {
                DeckAction::Run => cx.runs.waiting.deck_run = true,
                DeckAction::Check => cx.wb.prompt(
                    cx.wb
                        .messages()
                        .text(MessageId::NetlistDeckCheckWaitsForCompile),
                ),
            }
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
        Some(DeckRunVerdict::Checked(metadata)) => {
            record_include_resolutions(
                &mut cx.state.source_editing.include_resolutions,
                &metadata.sealed_source_dependencies,
            );
            let messages = cx.wb.messages();
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
        Some(DeckRunVerdict::Refused(error)) => {
            let verdict = match action {
                DeckAction::Run => Verdict::DidNotRun,
                DeckAction::Check => Verdict::CannotRun,
            };
            refuse(cx, &name, verdict, &text, &error);
        }
    }
    bring_findings_current(cx);
}

/// Press again for the press that waits on the deck's files on disk
/// (`disk`) once it is answered: the reads were taken in, a file that did
/// not read taken as its tab holds it, or the reader reloaded the file or
/// kept theirs, and the disk is looked at again. A press whose deck is no
/// longer open is dropped.
fn answer_disk_wait(cx: &mut Cx<'_, SourceFeature>, disk: DiskWait) {
    let (deck, answered, look) = match disk {
        DiskWait::Reading(deck) => (deck, cx.state.disk_looks.is_empty(), false),
        DiskWait::Answer(deck) => (deck, !deck_files::waits_for_answer(&cx.state.netlist), true),
    };
    let open = cx
        .state
        .netlist
        .open_deck
        .as_ref()
        .is_some_and(|open| open.id == deck);
    if open && !answered {
        return;
    }
    let Some(PreparingDeck { action, .. }) = cx.runs.preparing_deck.take().map(|deck| *deck) else {
        return;
    };
    if open {
        press_looking(cx, action, look);
    }
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
        format!("{name}: {}", warning.message_of(RunOf::Deck)),
    );
    cx.runs.preparing_deck = Some(Box::new(PreparingDeck {
        name,
        text,
        action: DeckAction::Run,
        memory: Some(warning),
        disk: None,
    }));
    if cx.wb.workspace != Workspace::Netlist {
        cx.execute(crate::workbench::framework::command_registry::ids::NETLIST_WORKSPACE);
    }
}

/// Run anyway: start the run held for memory, the run its press prepared,
/// as Run starts a run. The shell keeps the project's unsaved work as an
/// autosave as this returns, before the run dispatches. Whether a run was
/// held.
pub fn run_anyway(cx: &mut Cx<'_, SourceFeature>) -> bool {
    let Some(PreparingDeck { name, .. }) = cx
        .runs
        .preparing_deck
        .take_if(|deck| deck.memory.is_some())
        .map(|deck| *deck)
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
    text: &crate::state::RevisedText,
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
    // A run that needs more memory than the machine has is said of the deck:
    // in short on the status line, whole in the log, and in the band above
    // the deck until it is edited or pressed again.
    if let Some(&shortfall) = error.memory_shortfall() {
        let whole = messages.format(
            without_line,
            &[
                ("name", name),
                ("error", &shortfall.message_of(RunOf::Deck)),
            ],
        );
        let short = messages.format(
            without_line,
            &[("name", name), ("error", &shortfall.status())],
        );
        cx.model
            .record(LogSeverity::Error, LogSource::Netlist, whole);
        cx.wb.prompt(short);
        cx.state.netlist.memory_refusal = Some(MemoryRefusal {
            revision: text.revision(),
            shortfall,
        });
        return;
    }
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
