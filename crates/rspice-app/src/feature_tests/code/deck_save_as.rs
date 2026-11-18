//! Save deck as… writes the open deck to a new file, in its own encoding
//! and line endings, and the project reads it from there; the old file stays
//! as it was. Save netlist as deck… writes the generated netlist to a new
//! deck, its first line naming the design and when, and opens it as a deck
//! of its own.

#![cfg(not(target_arch = "wasm32"))]

use super::netlist_workflow::{Folder, app_with_deck};
use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame, themed_context};
use rspice_app_code::workbench::code::commands::CodeCommand;
use rspice_app_code::workbench::documents::deck_files::LineEnding;
use rspice_app_code::workbench::documents::netlist_document::{
    ActiveNetlistDocument, edit_deck_file,
};
use rspice_app_state::state::TextEncoding;
use rspice_app_state::time_compat::with_unix_epoch;

const LEGACY: &[u8] = b"* r\xe9sistance\r\nV1 out 0 1\r\nR1 out 0 1k\r\n.op\r\n.end\r\n";
const LEGACY_EDITED: &[u8] = b"* r\xe9sistance\r\nV1 out 0 1\r\nR1 out 0 2k\r\n.op\r\n.end\r\n";

/// 2026-11-18 12:00 UTC.
const NOON: std::time::Duration = std::time::Duration::from_secs(1_795_003_200);

/// Save deck as… writes the deck with its edit to the new file, Latin-1
/// with CRLF as it was read, and the project reads it from there; the old
/// file keeps what it held.
#[test]
fn save_deck_as_writes_the_deck_where_asked_and_the_project_reads_it_from_there() {
    let folder = Folder::new();
    let old = folder.write("legacy.cir", LEGACY);
    let mut app = app_with_deck(&old);
    assert!(app.command_key_enabled(CodeCommand::SaveDeckAs.key()));
    assert!(edit_deck_file(
        &mut app.state.model,
        &mut app.state.code.netlist,
        "* r\u{e9}sistance\nV1 out 0 1\nR1 out 0 2k\n.op\n.end\n".to_owned()
    ));
    std::fs::create_dir_all(folder.0.join("kept")).expect("the folder");
    let new = folder.0.join("kept").join("legacy_2k.cir");

    assert!(
        app.state.save_deck_as_at(&new),
        "{:?}",
        app.state.prompt_text()
    );

    assert_eq!(std::fs::read(&new).expect("the new deck"), LEGACY_EDITED);
    assert_eq!(std::fs::read(&old).expect("the old deck"), LEGACY);
    let deck = app.state.code.netlist.open_deck.as_ref().expect("open");
    assert_eq!(deck.root.path, new);
    assert!(!deck.root.dirty);
    assert_eq!(deck.root.encoding, TextEncoding::Latin1);
    assert_eq!(deck.root.line_ending, LineEnding::CrLf);
    let content = &app.state.model.workspace.content;
    assert_eq!(content.decks.len(), 1, "the deck moved, it was not copied");
    assert!(content.decks[0].is_file(&new, content.project.data_root()));
    assert_eq!(
        app.state.prompt_text(),
        Some("Saved the deck as legacy_2k.cir; it is read from there now.")
    );
}

/// The generated netlist saved as a deck is a file whose first line names
/// the design and when, in UTC, and whose lines after it are the netlist as
/// generated; it opens in front as a deck of the project.
#[test]
fn the_generated_netlist_saved_as_a_deck_names_the_design_and_opens_as_a_deck() {
    let reference = crate::workbench::examples::hierarchy_reference::build();
    let mut app = RSpiceApp::test_instance();
    app.state = reference.state;
    assert!(!app.command_key_enabled(CodeCommand::SaveNetlistAsDeck.key()));
    app.execute_command_key(CodeCommand::GenerateNetlist.key());
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    let generated = app.state.model.netlist_artifacts.generated_source.clone();
    assert!(!generated.trim().is_empty(), "the netlist is generated");
    assert!(app.command_key_enabled(CodeCommand::SaveNetlistAsDeck.key()));
    let design = app
        .state
        .model
        .export_file_stem(app.state.model.workspace.content.project.name());
    assert!(
        design.contains(&reference.top.cell),
        "{design:?} names the design {:?}",
        reference.top.cell
    );
    let folder = Folder::new();
    let path = folder.0.join("amp_deck.cir");

    let saved = with_unix_epoch(Ok(NOON), || app.state.save_generated_as_deck_at(&path));

    assert!(saved, "{:?}", app.state.prompt_text());
    let text = std::fs::read_to_string(&path).expect("the deck");
    let (first, rest) = text.split_once('\n').expect("a first line");
    assert_eq!(
        first,
        format!("* {design} netlist, saved as a deck 2026-11-18 12:00 UTC")
    );
    assert_eq!(rest.trim_end(), generated.trim_end());
    let deck = app.state.code.netlist.open_deck.as_ref().expect("open");
    assert_eq!(deck.root.path, path);
    assert_eq!(
        app.state.code.netlist.active_document,
        ActiveNetlistDocument::OwnedSource
    );
    assert_eq!(app.state.model.simulation.source.netlist_content, text);
    let content = &app.state.model.workspace.content;
    assert!(
        content
            .decks
            .iter()
            .any(|deck| deck.is_file(&path, content.project.data_root()))
    );
}

/// A deck open with edits stops Save netlist as deck… before any file is
/// written: the new deck would put the edited one away.
#[test]
fn a_deck_holding_edits_stops_save_netlist_as_deck_before_a_file_is_written() {
    let folder = Folder::new();
    let open = folder.write("amp.cir", "* amp\nR1 a 0 1k\n.op\n.end\n");
    let mut app = app_with_deck(&open);
    assert!(edit_deck_file(
        &mut app.state.model,
        &mut app.state.code.netlist,
        "* amp\nR1 a 0 2k\n.op\n.end\n".to_owned()
    ));
    app.state.model.netlist_artifacts.generated_source = "* g\nR1 a 0 1k\n.op\n.end\n".to_owned();
    let path = folder.0.join("generated.cir");

    assert!(!app.state.save_generated_as_deck_at(&path));

    assert!(!path.exists());
    assert_eq!(
        app.state.prompt_text(),
        Some("Save amp.cir before opening another deck.")
    );
}
