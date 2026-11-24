//! A deck's files looked at on disk as the application looks: by their
//! length and time on the frame, a file whose length or time moved read
//! away from it. A page shown after a file changed on disk reads it again
//! into a clean tab on a later frame. Run with a file changed on disk is
//! preparing at once; once the file is read, a tab holding edits holds the
//! run with the band's three answers, and the run goes on with the text the
//! reader chose. A file whose length and time did not move is not read.

#![cfg(not(target_arch = "wasm32"))]

use std::time::{Duration, Instant};

use super::netlist_document::painted_text;
use super::netlist_workflow::{Folder, app_with_deck};
use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame, themed_context};
use rspice_app_code::workbench::documents::deck_files;
use rspice_app_code::workbench::documents::netlist_document::edit_deck_file;
use rspice_app_state::state::SimulationRunIntent;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;

const DECK: &str = "* amp\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n";
const EDITED: &str = "* amp\nV1 in 0 1\nR1 in 0 2k\n.op\n.end\n";
const ON_DISK: &str = "* amp\nV1 in 0 1\nR1 in 0 10k\n.op\n.end\n";

/// The deck's own tab.
fn root(app: &RSpiceApp) -> &deck_files::OpenFile {
    &app.state
        .code
        .netlist
        .open_deck
        .as_ref()
        .expect("a deck is open")
        .root
}

/// `DECK` as the file `amp.cir` in `folder`, open on the Netlist workspace,
/// its files looked at on disk away from the frame from now on.
fn session(folder: &Folder) -> (RSpiceApp, egui::Context, std::path::PathBuf) {
    let path = folder.write("amp.cir", DECK);
    let mut app = app_with_deck(&path);
    app.state.workbench.activate(Workspace::Netlist);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    deck_files::read_decks_away_here();
    (app, ctx, path)
}

/// [`session`] with `EDITED` typed over the deck, then `ON_DISK` written
/// to its file by something else.
fn edited_then_changed_on_disk(folder: &Folder) -> (RSpiceApp, egui::Context, std::path::PathBuf) {
    let (mut app, ctx, path) = session(folder);
    assert!(edit_deck_file(
        &mut app.state.model,
        &mut app.state.code.netlist,
        EDITED.to_owned()
    ));
    std::fs::write(&path, ON_DISK).expect("another writer");
    (app, ctx, path)
}

/// Frames until `until` holds, the last frame's output.
fn frames_until(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    what: &str,
    until: impl Fn(&RSpiceApp) -> bool,
) -> egui::FullOutput {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        let output = frame(ctx, app, DESKTOP);
        if until(app) {
            return output;
        }
        assert!(Instant::now() < deadline, "{what} never came");
        std::thread::sleep(Duration::from_millis(2));
    }
}

/// Whether a run of the deck was asked for or has started.
fn deck_run_started(app: &RSpiceApp) -> bool {
    let simulation = &app.state.model.simulation;
    (simulation.execution.trigger_simulation
        && simulation.execution.run_intent == SimulationRunIntent::ManualDeck)
        || simulation.retained.executed_decks.iter().count() > 0
}

/// The text the deck's run ran, once it is dispatched.
fn text_run(app: &RSpiceApp) -> Option<String> {
    let handoff = &app.state.runs.manual_run;
    handoff
        .pending_run_buffer
        .as_ref()
        .or(handoff.last_run_buffer.as_ref())
        .map(|text| text.as_str().to_owned())
}

/// Run pressed on the edited deck whose file changed on disk, then frames
/// until the run holds for the reader's answer.
fn held(ctx: &egui::Context, app: &mut RSpiceApp) {
    app.execute_command_key(ids::RUN_SIMULATION);
    frames_until(ctx, app, "the hold", |app| root(app).disk_change.is_some());
    assert!(app.state.runs.preparing_deck.is_some(), "the run waits");
}

/// The page shown after its file changed on disk reads the file away from
/// the frame that shows it: the tab is as it was on that frame, and a later
/// frame reads the file into it and says so.
#[test]
fn the_page_shown_reads_a_changed_file_on_a_later_frame() {
    let folder = Folder::new();
    let (mut app, ctx, path) = session(&folder);
    std::fs::write(&path, ON_DISK).expect("another writer");

    app.state.workbench.activate(Workspace::Design);
    frame(&ctx, &mut app, DESKTOP);
    app.state.workbench.activate(Workspace::Netlist);
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(
        root(&app).text,
        DECK,
        "the frame that shows the page reads nothing"
    );

    frames_until(&ctx, &mut app, "the read", |app| root(app).text == ON_DISK);
    assert_eq!(app.state.model.simulation.source.netlist_content, ON_DISK);
    assert!(!root(&app).dirty);
    assert_eq!(
        app.state.prompt_text(),
        Some("Reloaded amp.cir: changed on disk.")
    );
}

/// Run with the deck's file changed on disk under edits is preparing at
/// once, the file not yet read; once it is read the run waits with the
/// words and the three answers it waits with when the page found the
/// change, runs nothing and writes nothing.
#[test]
fn run_is_preparing_at_once_and_holds_once_the_changed_file_is_read() {
    let folder = Folder::new();
    let (mut app, ctx, path) = edited_then_changed_on_disk(&folder);

    app.execute_command_key(ids::RUN_SIMULATION);
    assert!(
        app.state.runs.preparing_deck.is_some(),
        "the run is preparing"
    );
    assert!(
        root(&app).disk_change.is_none(),
        "the press reads nothing of the file"
    );

    frames_until(&ctx, &mut app, "the hold", |app| {
        root(app).disk_change.is_some()
    });
    let output = frame(&ctx, &mut app, DESKTOP);
    assert_eq!(
        app.state.prompt_text(),
        Some("Run is waiting: amp.cir changed on disk. Reload it, keep yours or compare the two.")
    );
    let painted = painted_text(&output);
    for words in [
        "amp.cir changed on disk since you opened it.",
        "Reload",
        "Keep mine",
        "Compare",
    ] {
        assert!(painted.contains(words), "{words}: {painted}");
    }
    assert!(app.state.runs.preparing_deck.is_some(), "the run waits");
    assert!(!deck_run_started(&app), "nothing runs");
    assert_eq!(std::fs::read_to_string(&path).expect("the deck"), ON_DISK);
    assert_eq!(root(&app).text, EDITED);
}

/// Keep mine on the held run runs the editor's text, saved over the file
/// first.
#[test]
fn keep_mine_runs_the_editors_text() {
    let folder = Folder::new();
    let (mut app, ctx, path) = edited_then_changed_on_disk(&folder);
    held(&ctx, &mut app);

    deck_files::keep_mine(&mut app.state.code.netlist, None);
    frames_until(&ctx, &mut app, "the run", |app| text_run(app).is_some());

    assert_eq!(text_run(&app).as_deref(), Some(EDITED));
    assert_eq!(std::fs::read_to_string(&path).expect("the deck"), EDITED);
    assert!(!root(&app).dirty);
}

/// Reload on the held run runs the file's text as it is on disk.
#[test]
fn reload_runs_the_text_on_disk() {
    let folder = Folder::new();
    let (mut app, ctx, path) = edited_then_changed_on_disk(&folder);
    held(&ctx, &mut app);

    let messages = app.state.workbench.messages();
    deck_files::reload_from_disk(
        &mut app.state.model,
        &mut app.state.code.netlist,
        None,
        messages,
    )
    .expect("reloaded");
    frames_until(&ctx, &mut app, "the run", |app| text_run(app).is_some());

    assert_eq!(text_run(&app).as_deref(), Some(ON_DISK));
    assert_eq!(root(&app).text, ON_DISK);
    assert_eq!(std::fs::read_to_string(&path).expect("the deck"), ON_DISK);
}

/// Run with the file changed on disk and its tab clean runs the text on
/// disk, read into the tab first.
#[test]
fn run_on_a_clean_tab_runs_the_changed_file() {
    let folder = Folder::new();
    let (mut app, ctx, path) = session(&folder);
    std::fs::write(&path, ON_DISK).expect("another writer");

    app.execute_command_key(ids::RUN_SIMULATION);
    assert_eq!(root(&app).text, DECK, "the press reads nothing of the file");
    frames_until(&ctx, &mut app, "the run", |app| text_run(app).is_some());

    assert_eq!(text_run(&app).as_deref(), Some(ON_DISK));
    assert_eq!(root(&app).text, ON_DISK);
}

/// A file whose length and time are as when it was read is not read, when
/// the page is shown or by Run: other text of the same length written with
/// the old time stays on disk, and the run runs the tab's.
#[test]
fn a_file_whose_length_and_time_did_not_move_is_not_read() {
    let folder = Folder::new();
    let (mut app, ctx, path) = session(&folder);
    let modified = std::fs::metadata(&path)
        .and_then(|metadata| metadata.modified())
        .expect("the file's time");
    assert_eq!(EDITED.len(), DECK.len());
    std::fs::write(&path, EDITED).expect("another writer");
    std::fs::File::options()
        .write(true)
        .open(&path)
        .and_then(|file| file.set_modified(modified))
        .expect("the old time");

    app.state.workbench.activate(Workspace::Design);
    frame(&ctx, &mut app, DESKTOP);
    app.state.workbench.activate(Workspace::Netlist);
    frame(&ctx, &mut app, DESKTOP);
    app.execute_command_key(ids::RUN_SIMULATION);
    assert!(
        app.state.code.disk_looks.is_empty(),
        "nothing is read to look"
    );
    frames_until(&ctx, &mut app, "the run", |app| text_run(app).is_some());

    assert_eq!(text_run(&app).as_deref(), Some(DECK));
    assert_eq!(root(&app).text, DECK);
}
