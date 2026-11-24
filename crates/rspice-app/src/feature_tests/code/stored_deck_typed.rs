//! A key typed in a deck the project keeps as text: the project holds the
//! editor's own text rather than a copy of it, and every reader of the
//! deck's text reads what was typed, with the deck's own line separators.
//! A deck typed back to the text it was saved with reads as saved.

#![cfg(not(target_arch = "wasm32"))]

use std::time::{Duration, Instant};

use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame, frame_with, themed_context};
use crate::workbench::app_state::AppState;
use rspice_app_code::workbench::documents::deck_files;
use rspice_app_runs::simulation::controller::SimulationController;
use rspice_app_services::workbench::lifecycle::project_checkpoint;
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::{DeckId, ProjectDeck, StoredDeck, TextEncoding};
use rspice_app_state::workbench::workspace::Workspace;

/// The deck typed in, written with CRLF, its card's brace unclosed.
const UNCLOSED: &str = "* typed\r\nV1 in 0 1\r\nR1 in 0 {1k\r\n.op\r\n.end\r\n";
/// The deck once the brace is typed, with its separators.
const TYPED: &str = "* typed\r\nV1 in 0 1\r\nR1 in 0 {1k}\r\n.op\r\n.end\r\n";
/// Another deck the project keeps.
const OTHER: &str = "* other\nV1 in 0 1\nR1 in 0 2k\n.op\n.end\n";

/// Keep `text` in the project as the deck `name`.
fn keep(state: &mut AppState, name: &str, text: &str) -> DeckId {
    let deck = ProjectDeck::stored(StoredDeck {
        name: name.to_owned(),
        text: text.into(),
        encoding: TextEncoding::Utf8,
        files: Vec::new(),
    });
    let id = deck.id;
    state.model.workspace.content.decks.push(deck);
    id
}

/// The deck `id` brought to the front of the Netlist workspace.
fn open(state: &mut AppState, id: DeckId) {
    let messages = state.workbench.messages();
    deck_files::open_deck_by_id(
        &mut state.model,
        &mut state.code.netlist,
        &mut state.code.netlist_open_documents,
        &mut state.runs,
        id,
        messages,
    )
    .expect("the deck opens");
    state.workbench.activate(Workspace::Netlist);
}

/// The text the project keeps for the deck `id`.
fn kept(state: &AppState, id: DeckId) -> String {
    state
        .model
        .workspace
        .content
        .decks
        .iter()
        .find(|deck| deck.id == id)
        .and_then(ProjectDeck::stored_deck)
        .map(|deck| deck.text.as_str().to_owned())
        .expect("the project keeps the deck")
}

/// The deck in front prepared as Run deck prepares it and handed to the
/// engine: the netlist the engine reads, or why it was refused.
fn dispatched(state: &mut AppState) -> Result<String, String> {
    state.model.simulation.execution.run_intent =
        rspice_app_state::state::SimulationRunIntent::ManualDeck;
    let mut controller = SimulationController::new();
    controller
        .prepare_deck_run(state)
        .map_err(|error| error.to_string())?;
    let dispatch = controller
        .consume_snapshot_for_dispatch(state)
        .map_err(|error| error.to_string())?;
    Ok(dispatch.executable_netlist().to_owned())
}

/// The findings the check of the deck in front states on the card's line.
fn on_the_card(app: &RSpiceApp) -> usize {
    app.state
        .code
        .netlist
        .diagnostics
        .iter()
        .filter(|finding| finding.is_current() && finding.line == Some(2))
        .count()
}

/// A key typed in a deck the project keeps, written with CRLF: the project
/// then holds the typed text with the deck's own separators, and its check,
/// its run, its export, the project's autosave and its save each read it.
/// Another deck brought to the front and this one brought back shows it.
#[test]
fn a_key_typed_in_a_kept_deck_is_what_every_reader_of_its_text_reads() {
    let folder = unique_temp_dir("kept-deck-typed");
    let file = folder.join("typed.rspiceproj");
    let mut app = RSpiceApp::test_instance();
    let typed = keep(&mut app.state, "typed.cir", UNCLOSED);
    let other = keep(&mut app.state, "other.cir", OTHER);
    assert!(app.state.save_project_to_path(&file), "the project saves");
    open(&mut app.state, typed);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    assert_ne!(on_the_card(&app), 0, "the open brace is found");

    let editor = rspice_app_code::workbench::documents::netlist_document::editor_id_in(
        &app.state.model,
        &app.state.code.netlist,
    );
    let shown = UNCLOSED.replace("\r\n", "\n");
    let caret = shown.find("{1k").expect("the card") + "{1k".len();
    rspice_app_code::workbench::documents::text_editor_commands::queue_caret_char_index(
        &ctx, editor, caret,
    );
    ctx.memory_mut(|memory| memory.request_focus(editor));
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    frame_with(
        &ctx,
        &mut app,
        DESKTOP,
        vec![egui::Event::Text("}".to_owned())],
    );
    frame(&ctx, &mut app, DESKTOP);

    assert_eq!(
        app.state.model.simulation.source.netlist_content,
        TYPED.replace("\r\n", "\n"),
        "the editor shows the key"
    );
    assert_eq!(
        kept(&app.state, typed),
        TYPED,
        "the project keeps the typed text with its separators"
    );
    assert_eq!(on_the_card(&app), 0, "the check read the closed brace");
    let run = dispatched(&mut app.state).expect("the run reads the closed brace");
    assert!(run.contains("R1 in 0"), "{run}");
    let (_, exported) = deck_files::export_stored_deck(
        &app.state.model,
        &app.state.code.netlist,
        false,
        app.state.workbench.messages(),
    )
    .expect("the deck exports");
    assert_eq!(exported, TYPED.as_bytes(), "the export");

    app.state.platform.project_autosave.arm_due();
    let deadline = Instant::now() + Duration::from_secs(60);
    while app.state.platform.project_autosave.written().is_empty() {
        assert!(Instant::now() < deadline, "the autosave is never written");
        frame(&ctx, &mut app, DESKTOP);
        std::thread::sleep(Duration::from_millis(2));
    }
    let autosaved = project_checkpoint::load(&app.state.platform.project_autosave.written()[0])
        .expect("the autosave reads");
    let autosaved = autosaved
        .file
        .workspace
        .decks
        .iter()
        .find(|deck| deck.id == typed)
        .and_then(ProjectDeck::stored_deck)
        .map(|deck| deck.text.as_str().to_owned());
    assert_eq!(autosaved.as_deref(), Some(TYPED), "the autosave");

    assert!(app.state.save_project_to_path(&file), "the project saves");
    let saved = rspice_app_services::io::project_io::load_project_text(
        &std::fs::read_to_string(&file).expect("the project file"),
        None,
    )
    .expect("the project file loads");
    let saved = saved
        .file
        .workspace
        .decks
        .iter()
        .find(|deck| deck.id == typed)
        .and_then(ProjectDeck::stored_deck)
        .map(|deck| deck.text.as_str().to_owned());
    assert_eq!(saved.as_deref(), Some(TYPED), "the saved project file");
    assert!(
        !app.state.model.workspace.content.decks_dirty,
        "the save leaves the decks saved"
    );

    open(&mut app.state, other);
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(app.state.model.simulation.source.netlist_content, OTHER);
    open(&mut app.state, typed);
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(
        app.state.model.simulation.source.netlist_content,
        TYPED.replace("\r\n", "\n"),
        "the deck brought back shows the key"
    );
    let _ = std::fs::remove_dir_all(folder);
}

/// Type `x` at the start of the deck in front, then take it back.
fn type_and_take_back(ctx: &egui::Context, app: &mut RSpiceApp) {
    let editor = rspice_app_code::workbench::documents::netlist_document::editor_id_in(
        &app.state.model,
        &app.state.code.netlist,
    );
    rspice_app_code::workbench::documents::text_editor_commands::queue_caret_char_index(
        ctx, editor, 0,
    );
    ctx.memory_mut(|memory| memory.request_focus(editor));
    frame(ctx, app, DESKTOP);
    frame(ctx, app, DESKTOP);
    frame_with(ctx, app, DESKTOP, vec![egui::Event::Text("x".to_owned())]);
    frame(ctx, app, DESKTOP);
    frame_with(
        ctx,
        app,
        DESKTOP,
        vec![egui::Event::Key {
            key: egui::Key::Backspace,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
}

/// Frames until `saved` holds.
fn until(ctx: &egui::Context, app: &mut RSpiceApp, saved: impl Fn(&RSpiceApp) -> bool) {
    let deadline = Instant::now() + Duration::from_secs(60);
    while !saved(app) {
        assert!(Instant::now() < deadline, "it never reads as saved");
        frame(ctx, app, DESKTOP);
        std::thread::sleep(Duration::from_millis(2));
    }
}

/// A deck the project keeps, written with CRLF, and then a deck file, each
/// typed in and typed back to the text it was saved with: each reads as
/// saved again without a save.
#[test]
fn a_deck_typed_back_to_its_saved_text_reads_as_saved() {
    use rspice_app_services::workbench::lifecycle::project_lifecycle::has_unsaved_changes;

    let folder = unique_temp_dir("kept-deck-typed-back");
    let file = folder.join("typed.rspiceproj");
    let mut app = RSpiceApp::test_instance();
    let typed = keep(&mut app.state, "typed.cir", TYPED);
    keep(&mut app.state, "other.cir", OTHER);
    assert!(app.state.save_project_to_path(&file), "the project saves");
    open(&mut app.state, typed);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    assert!(!has_unsaved_changes(&app.state));

    type_and_take_back(&ctx, &mut app);
    assert!(
        app.state.model.workspace.content.decks_dirty,
        "the edits read as unsaved"
    );
    until(&ctx, &mut app, |app| {
        !app.state.model.workspace.content.decks_dirty && !has_unsaved_changes(&app.state)
    });
    assert_eq!(kept(&app.state, typed), TYPED);

    let deck = folder.join("file.cir");
    assert!(
        app.state
            .open_test_deck(OTHER.to_owned(), Some(deck.clone()), "file.cir"),
        "{:?}",
        app.state.prompt_text()
    );
    frame(&ctx, &mut app, DESKTOP);
    type_and_take_back(&ctx, &mut app);
    let unsaved = |app: &RSpiceApp| {
        app.state
            .code
            .netlist
            .open_deck
            .as_ref()
            .and_then(deck_files::OpenDeck::unsaved)
            .is_some()
    };
    assert!(unsaved(&app), "the edits read as unsaved");
    until(&ctx, &mut app, |app| !unsaved(app));
    assert_eq!(
        std::fs::read_to_string(&deck).expect("the deck file"),
        OTHER
    );
    let _ = std::fs::remove_dir_all(folder);
}
