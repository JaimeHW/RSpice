//! A deck read away from the frame, as the application reads one: its tab
//! comes to the front at once, saying it is reading, and the first frame
//! after the read opens it. A file that is not there is refused at once.

#![cfg(not(target_arch = "wasm32"))]

use std::time::{Duration, Instant};

use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame, themed_context};
use rspice_app_code::workbench::documents::deck_files;
use rspice_app_code::workbench::documents::netlist_document::NetlistTab;
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::{ProjectDeck, StoredDeck, TextEncoding};
use rspice_app_state::workbench::workspace::Workspace;

const FILE_DECK: &str = "* read away\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n";
const KEPT_DECK: &str = "* kept\nV1 in 0 1\nR1 in 0 2k\n.op\n.end\n";

/// Frames until the deck being read is open.
fn until_open(ctx: &egui::Context, app: &mut RSpiceApp) {
    let deadline = Instant::now() + Duration::from_secs(60);
    while app.state.code.netlist.open_deck.is_none() {
        assert!(Instant::now() < deadline, "the deck never opened");
        frame(ctx, app, DESKTOP);
        std::thread::sleep(Duration::from_millis(2));
    }
}

/// A deck file opened in a project of its own, then a deck the project
/// keeps opened from its list: each tab is in front at once, reading, and a
/// later frame shows its text.
#[test]
fn a_deck_read_away_from_the_frame_opens_on_a_later_frame() {
    deck_files::read_decks_away_here();
    let folder = unique_temp_dir("deck-read-away");
    let file = folder.join("away.cir");
    std::fs::write(&file, FILE_DECK).expect("the deck file");
    let mut app = RSpiceApp::test_instance();
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);

    assert!(
        !app.state.open_deck_at(&folder.join("absent.cir"), true),
        "a file that is not there is refused"
    );
    assert!(app.state.model.workspace.content.decks.is_empty());

    assert!(
        app.state.open_deck_at(&file, true),
        "{:?}",
        app.state.prompt_text()
    );
    let id = app.state.model.workspace.content.decks[0].id;
    assert!(
        app.state.code.netlist.open_deck.is_none(),
        "read on the frame"
    );
    assert!(deck_files::reading_deck_in_front(&app.state.code.netlist));
    assert_eq!(app.state.code.netlist.front, NetlistTab::Deck(id));
    until_open(&ctx, &mut app);
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(
        app.state.model.simulation.source.netlist_content.as_str(),
        FILE_DECK
    );
    assert_eq!(app.state.code.netlist.front, NetlistTab::Deck(id));
    assert!(app.state.code.netlist.reading_deck.is_none());

    let deck = ProjectDeck::stored(StoredDeck {
        name: "kept.cir".to_owned(),
        text: KEPT_DECK.into(),
        encoding: TextEncoding::Utf8,
        files: Vec::new(),
    });
    let kept = deck.id;
    app.state.model.workspace.content.decks.push(deck);
    let messages = app.state.workbench.messages();
    let state = &mut app.state;
    deck_files::open_deck_by_id(
        &mut state.model,
        &mut state.code.netlist,
        &mut state.code.netlist_open_documents,
        &mut state.runs,
        kept,
        messages,
    )
    .expect("the kept deck opens");
    app.state.workbench.activate(Workspace::Netlist);
    assert!(
        app.state.code.netlist.open_deck.is_none(),
        "read on the frame"
    );
    assert_eq!(app.state.code.netlist.front, NetlistTab::Deck(kept));
    until_open(&ctx, &mut app);
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(
        app.state.model.simulation.source.netlist_content.as_str(),
        KEPT_DECK
    );
    assert_eq!(app.state.code.netlist.front, NetlistTab::Deck(kept));
    let _ = std::fs::remove_dir_all(folder);
}
