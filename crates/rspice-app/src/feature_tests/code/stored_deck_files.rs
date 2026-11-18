//! Decks the project keeps as text, with the files their include lines
//! read: SPICE files picked together or a ZIP of them, stored or deflated,
//! come in as decks with their files kept by their paths from each deck;
//! such a deck checks and runs reading those files and nothing on a disk,
//! so an absolute path is the error on the line that writes it (NL-8); it
//! goes out again as one ZIP; and on the desktop it is saved as files, and
//! the project keeps it as the file from then on.

use std::io::Write as _;

use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame, themed_context};
use crate::workbench::app_state::AppState;
use rspice_app_code::workbench::documents::deck_files;
use rspice_app_code::workbench::workflows::deck_import;
use rspice_app_runs::simulation::controller::SimulationController;
use rspice_app_state::workbench::workspace::Workspace;

const TESTBENCH: &str = "* amplifier testbench\n.include \"models/opamp.inc\"\n.lib \"../lib/corners.lib\" tt\nVDD vdd 0 1.8\nVIN in 0 0.5\nXU1 in out vdd 0 opamp\nRL out 0 {rload}\n.op\n.end\n";
const OPAMP: &str = "* op-amp\n.subckt opamp inp out vdd vss\nRIN inp vss 1meg\nE1 out vss inp vss 2\n.ends opamp\n";
const CORNERS: &str =
    "* corners\n.lib tt\n.param rload=10k\n.endl tt\n.lib ff\n.param rload=5k\n.endl ff\n";

/// The deck in front prepared as Run deck prepares it and handed to the
/// engine: the netlist the engine reads, or why it was refused.
fn dispatched(state: &mut AppState) -> Result<String, String> {
    state.model.simulation.execution.run_intent =
        rspice_app_state::state::SimulationRunIntent::ManualDeck;
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(state)
        .map_err(|error| error.to_string())?;
    let dispatch = controller
        .consume_snapshot_for_dispatch(state)
        .map_err(|error| error.to_string())?;
    Ok(dispatch.executable_netlist().to_owned())
}

/// A deflated ZIP of `files`, each a path in it and its text.
fn deflated(files: &[(&str, &str)]) -> Vec<u8> {
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    for (path, text) in files {
        writer.start_file(*path, options).expect("a member");
        writer.write_all(text.as_bytes()).expect("its text");
    }
    writer.finish().expect("the archive").into_inner()
}

/// The testbench's ZIP: the deck in a folder, its op-amp beside it and its
/// corners library in a folder next to the deck's.
fn testbench_zip() -> Vec<u8> {
    deflated(&[
        ("amp/amp_tb.cir", TESTBENCH),
        ("amp/models/opamp.inc", OPAMP),
        ("lib/corners.lib", CORNERS),
        ("amp/notes.md", "run at 1.8 V\n"),
    ])
}

/// A deflated ZIP comes in as its one deck, the file no other includes,
/// with every other file kept by its path from the deck; it is in front,
/// and runs reading its op-amp and the section of its library it names.
#[test]
fn a_deflated_zip_comes_in_as_its_deck_with_the_files_it_reads_and_runs() {
    let mut state = AppState::default();

    assert!(
        state.import_deck_files(vec![("amp.zip".to_owned(), testbench_zip())]),
        "{:?}",
        state.prompt_text()
    );

    assert_eq!(state.workbench.workspace, Workspace::Netlist);
    let decks = &state.model.workspace.content.decks;
    assert_eq!(decks.len(), 1);
    let stored = decks[0].stored_deck().expect("kept as text");
    assert_eq!(stored.name, "amp_tb.cir");
    assert_eq!(stored.text, TESTBENCH);
    let mut files = stored
        .files
        .iter()
        .map(|file| file.path.as_str())
        .collect::<Vec<_>>();
    files.sort_unstable();
    assert_eq!(files, ["lib/corners.lib", "models/opamp.inc", "notes.md"]);
    assert_eq!(state.model.simulation.source.netlist_content, TESTBENCH);

    let netlist = dispatched(&mut state).expect("the deck runs");

    assert!(netlist.contains("E1 out vss inp vss 2"), "{netlist}");
    assert!(netlist.contains(".param rload=10k"), "{netlist}");
    assert!(!netlist.contains("rload=5k"), "{netlist}");
}

/// An absolute include of a deck kept as text is the check's error on that
/// line, and the run refuses with it: nothing is read from a disk (NL-8).
#[test]
fn an_absolute_include_of_a_deck_kept_as_text_is_the_error_on_its_line() {
    let mut app = RSpiceApp::test_instance();
    let deck = "* amp\nV1 a 0 1\n.include \"C:/pdk/models.lib\"\nR1 a 0 1k\n.op\n.end\n";
    assert!(
        app.state
            .import_deck_files(vec![("amp.cir".to_owned(), deck.as_bytes().to_vec())]),
        "{:?}",
        app.state.prompt_text()
    );
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    let said =
        "a deck kept in the project cannot read C:/pdk/models.lib; add the file to this deck";

    let error = app
        .state
        .code
        .netlist
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.message.contains(said))
        .expect("the check finds the line");
    // Lines count from zero: the third line, the include.
    assert_eq!(error.line, Some(2), "{error:?}");
    assert!(error.message.contains(said), "{error:?}");
    let refused = dispatched(&mut app.state).expect_err("nothing is read from a disk");
    assert!(refused.contains(said), "{refused}");
}

/// Files picked together come in as the decks among them, each a file no
/// other includes, and each keeps the files that are no deck: a deck one of
/// them includes is no deck of its own.
#[test]
fn picked_files_come_in_as_the_decks_no_other_file_includes() {
    let decks = deck_import::import_files(vec![
        (
            "tb.cir".to_owned(),
            b"* tb\n.include \"stage.cir\"\n.op\n.end\n".to_vec(),
        ),
        ("stage.cir".to_owned(), b"R1 a 0 1k\n".to_vec()),
        (
            "other.sp".to_owned(),
            b"* other\nV1 a 0 1\n.op\n.end\n".to_vec(),
        ),
    ])
    .expect("decks");

    let names = decks
        .iter()
        .map(|deck| deck.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(names, ["tb.cir", "other.sp"]);
    for deck in &decks {
        let files = deck
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        assert_eq!(files, ["stage.cir"], "{}", deck.name);
    }
}

/// A deck kept as text goes out alone, as its own text, or as one ZIP of
/// itself and its files, which comes in again as the same deck.
#[test]
fn a_deck_kept_as_text_exports_alone_or_as_a_zip_of_it_and_its_files() {
    let mut state = AppState::default();
    assert!(state.import_deck_files(vec![("amp.zip".to_owned(), testbench_zip())]));

    let (name, alone) =
        deck_files::export_stored_deck(&state.model, &state.code.netlist, false).expect("the deck");
    let (zip_name, zip) = deck_files::export_stored_deck(&state.model, &state.code.netlist, true)
        .expect("the deck and its files");

    assert_eq!(name, "amp_tb.cir");
    assert_eq!(alone, TESTBENCH.as_bytes());
    assert_eq!(zip_name, "amp_tb.zip");
    let kept = state.model.workspace.content.decks[0]
        .stored_deck()
        .expect("kept as text")
        .clone();
    assert_eq!(
        deck_import::import_files(vec![(zip_name, zip)]).expect("in again"),
        vec![kept]
    );
}

/// In the browser a deck of the project that is a file on the desktop is
/// named with its path, for the desktop to edit and run; on the desktop it
/// is the file itself.
#[test]
fn a_deck_that_is_a_file_is_named_with_its_path_in_the_browser() {
    let mut state = AppState::default();
    let path = std::env::temp_dir().join("amp_tb.cir");
    let deck = rspice_app_state::state::ProjectDeck::for_file(&path, None);
    let id = deck.id;
    state.model.workspace.content.decks.push(deck);
    state.code.netlist.unreadable_deck = Some(id);

    let shown = deck_files::desktop_deck_in(&state.model, &state.code.netlist, true)
        .expect("the browser names it");

    assert_eq!(shown.name, "amp_tb.cir");
    assert_eq!(shown.path, path);
    assert!(deck_files::desktop_deck_in(&state.model, &state.code.netlist, false).is_none());
}

/// On the desktop, Save as files writes the deck and each file kept with
/// it at its path beside the deck; the project then keeps the deck as the
/// file, and the run reads it and its files from the disk.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn save_as_files_writes_the_deck_and_its_files_and_the_project_keeps_the_file() {
    let deck = "* amp\n.include \"models/opamp.inc\"\nVDD vdd 0 1.8\nVIN in 0 0.5\nXU1 in out vdd 0 opamp\n.op\n.end\n";
    let mut state = AppState::default();
    assert!(state.import_deck_files(vec![(
        "amp.zip".to_owned(),
        deflated(&[("amp_tb.cir", deck), ("models/opamp.inc", OPAMP)]),
    )]));
    let folder = rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir("decks");
    let path = folder.join("amp_tb.cir");

    assert!(state.save_deck_as_at(&path), "{:?}", state.prompt_text());

    assert_eq!(std::fs::read_to_string(&path).expect("the deck"), deck);
    assert_eq!(
        std::fs::read_to_string(folder.join("models").join("opamp.inc")).expect("the op-amp"),
        OPAMP
    );
    let content = &state.model.workspace.content;
    assert!(content.decks[0].is_file(&path, content.project.data_root()));
    let open = state.code.netlist.open_deck.as_ref().expect("open");
    assert!(!open.is_stored());
    assert_eq!(open.root.path, path);
    let netlist = dispatched(&mut state).expect("the deck runs from the disk");
    assert!(netlist.contains("E1 out vss inp vss 2"), "{netlist}");
}
