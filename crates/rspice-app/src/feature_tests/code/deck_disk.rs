//! A deck's files against the disk. A file something else changed is read
//! again into a tab that holds no edits; a tab that holds edits keeps them,
//! shows the change and asks: Reload, Keep mine or Compare. Run saves the
//! deck's files first, so an edited include is what the run reads, and it
//! waits while a file changed under edits. A deck whose file is gone can be
//! found again, or taken out of the project, which leaves its file alone.

#![cfg(not(target_arch = "wasm32"))]

use super::netlist_document::painted_text;
use super::netlist_workflow::{Folder, app_with_deck, dispatched, opened};
use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame, themed_context};
use crate::workbench::app_state::AppState;
use rspice_app_code::workbench::documents::deck_files;
use rspice_app_code::workbench::documents::netlist_document::edit_deck_file;
use rspice_app_state::state::ProjectDeck;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;

const DECK: &str = "* amp\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n";
const EDITED: &str = "* amp\nV1 in 0 1\nR1 in 0 2k\n.op\n.end\n";
const ON_DISK: &str = "* amp\nV1 in 0 1\nR1 in 0 10k\n.op\n.end\n";

/// The deck's own tab.
fn root(state: &AppState) -> &deck_files::OpenFile {
    &state
        .code
        .netlist
        .open_deck
        .as_ref()
        .expect("a deck is open")
        .root
}

/// Show the Netlist page again, after a frame that showed another page.
fn show_the_page_again(ctx: &egui::Context, app: &mut RSpiceApp) -> egui::FullOutput {
    app.state.workbench.activate(Workspace::Design);
    frame(ctx, app, DESKTOP);
    app.state.workbench.activate(Workspace::Netlist);
    frame(ctx, app, DESKTOP)
}

/// The deck at `path`, open with `EDITED` in its tab, then rewritten on
/// disk by something else.
fn edited_then_changed_on_disk(path: &std::path::Path) -> AppState {
    let mut state = opened(path);
    assert!(edit_deck_file(
        &mut state.model,
        &mut state.code.netlist,
        EDITED.to_owned()
    ));
    std::fs::write(path, ON_DISK).expect("another writer");
    assert_eq!(
        deck_files::check_disk(&mut state.model, &mut state.code.netlist),
        Vec::<String>::new()
    );
    state
}

/// A file changed on disk is read again into a tab with no edits when the
/// page is shown, and the status bar says so; a frame of the page already
/// in front does not look.
#[test]
fn a_file_changed_on_disk_is_read_again_into_a_clean_tab_when_the_page_shows() {
    let folder = Folder::new();
    let path = folder.write("amp.cir", DECK);
    let mut app = app_with_deck(&path);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    std::fs::write(&path, ON_DISK).expect("another writer");

    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(app.state.model.simulation.source.netlist_content, DECK);

    show_the_page_again(&ctx, &mut app);

    assert_eq!(app.state.model.simulation.source.netlist_content, ON_DISK);
    assert_eq!(root(&app.state).text, ON_DISK);
    assert!(!root(&app.state).dirty);
    assert_eq!(
        app.state.prompt_text(),
        Some("Reloaded amp.cir: changed on disk.")
    );
}

/// A tab holding edits keeps them when its file changes on disk, and the
/// band over it says so and offers the three answers.
#[test]
fn a_tab_holding_edits_keeps_them_and_shows_the_change_on_disk() {
    let folder = Folder::new();
    let path = folder.write("amp.cir", DECK);
    let mut app = app_with_deck(&path);
    assert!(edit_deck_file(
        &mut app.state.model,
        &mut app.state.code.netlist,
        EDITED.to_owned()
    ));
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    std::fs::write(&path, ON_DISK).expect("another writer");

    show_the_page_again(&ctx, &mut app);
    let output = frame(&ctx, &mut app, DESKTOP);

    assert_eq!(app.state.model.simulation.source.netlist_content, EDITED);
    assert!(root(&app.state).dirty);
    let painted = painted_text(&output);
    for words in [
        "amp.cir changed on disk since you opened it.",
        "Reload",
        "Keep mine",
        "Compare",
    ] {
        assert!(painted.contains(words), "{words}: {painted}");
    }
}

/// Run waits while a file of the deck changed on disk under edits, and
/// writes nothing.
#[test]
fn run_waits_while_a_file_changed_on_disk_holds_edits() {
    let folder = Folder::new();
    let path = folder.write("amp.cir", DECK);
    let mut app = app_with_deck(&path);
    app.state.workbench.activate(Workspace::Netlist);
    assert!(edit_deck_file(
        &mut app.state.model,
        &mut app.state.code.netlist,
        EDITED.to_owned()
    ));
    std::fs::write(&path, ON_DISK).expect("another writer");

    app.execute_command_key(ids::RUN_SIMULATION);

    let state = &app.state;
    assert!(!state.model.simulation.execution.trigger_simulation);
    assert_eq!(
        state.prompt_text(),
        Some("Run is waiting: amp.cir changed on disk. Reload it, keep yours or compare the two.")
    );
    assert_eq!(std::fs::read_to_string(&path).expect("the deck"), ON_DISK);
    assert_eq!(root(state).text, EDITED);
}

/// Reload drops the tab's edits for the file on disk.
#[test]
fn reload_takes_the_file_on_disk_for_the_edits() {
    let folder = Folder::new();
    let path = folder.write("amp.cir", DECK);
    let mut state = edited_then_changed_on_disk(&path);

    deck_files::reload_from_disk(&mut state.model, &mut state.code.netlist, None)
        .expect("reloaded");

    assert_eq!(state.model.simulation.source.netlist_content, ON_DISK);
    assert_eq!(root(&state).text, ON_DISK);
    assert!(!root(&state).dirty);
    assert!(root(&state).disk_change.is_none());
}

/// Keep mine keeps the edits, and the next Save writes them over the file.
#[test]
fn keep_mine_keeps_the_edits_and_save_writes_them_over_the_file() {
    let folder = Folder::new();
    let path = folder.write("amp.cir", DECK);
    let mut state = edited_then_changed_on_disk(&path);

    deck_files::keep_mine(&mut state.code.netlist, None);
    deck_files::save_shown_file(&mut state.code.netlist).expect("saved");

    assert_eq!(std::fs::read_to_string(&path).expect("the deck"), EDITED);
    assert!(!root(&state).dirty);
}

/// Compare shows the file on disk against the tab, and the tab keeps its
/// edits.
#[test]
fn compare_shows_the_file_on_disk_against_the_tab() {
    let folder = Folder::new();
    let path = folder.write("amp.cir", DECK);
    let mut state = edited_then_changed_on_disk(&path);

    deck_files::compare_with_disk(&mut state.model, &mut state.code.netlist, None)
        .expect("compared");

    assert!(
        state.code.netlist.front.is_comparison(),
        "the comparison is in front"
    );
    let shown = &state.model.simulation.source.netlist_content;
    for line in [
        "--- amp.cir on disk",
        "+++ amp.cir in RSpice",
        "-R1 in 0 10k",
        "+R1 in 0 2k",
    ] {
        assert!(shown.contains(line), "{line}: {shown}");
    }
    assert_eq!(root(&state).text, EDITED);
    assert!(root(&state).dirty);
}

/// An include edited on its tab is saved by Run, and the run reads the
/// edit (NL-10).
#[test]
fn run_saves_an_edited_include_and_the_run_reads_it() {
    let folder = Folder::new();
    let include = folder.write("models.inc", ".model dmod d is=1e-14\n");
    let deck = folder.write(
        "inc.cir",
        "* include\n.include models.inc\nV1 in 0 1\nR1 in a 1k\nD1 a 0 dmod\n.op\n.end\n",
    );
    let mut app = app_with_deck(&deck);
    app.state.workbench.activate(Workspace::Netlist);
    let state = &mut app.state;
    deck_files::show_include(
        &mut state.model,
        &mut state.code.netlist,
        &mut state.code.netlist_open_documents,
        &mut state.runs,
        &include,
    )
    .expect("the include opens");
    assert!(edit_deck_file(
        &mut state.model,
        &mut state.code.netlist,
        ".model dmod d is=3e-14\n".to_owned()
    ));
    assert!(deck_files::show_deck(
        &mut state.model,
        &mut state.code.netlist
    ));

    app.execute_command_key(ids::RUN_SIMULATION);

    assert_eq!(
        std::fs::read_to_string(&include).expect("the include"),
        ".model dmod d is=3e-14\n"
    );
    let netlist = dispatched(&mut app.state).expect("the deck runs");
    assert!(netlist.contains("3e-14"), "{netlist}");
}

/// A deck whose file is not where the project says is named with where it
/// was looked for, and Locate reads it from where it is now.
#[test]
fn a_deck_whose_file_is_gone_is_found_again_with_locate() {
    let folder = Folder::new();
    let moved = folder.write("found/amp.cir", DECK);
    let gone = folder.0.join("amp.cir");
    let mut state = AppState::default();
    state
        .model
        .workspace
        .content
        .decks
        .push(ProjectDeck::for_file(&gone, None));
    let id = state.model.workspace.content.decks[0].id;
    deck_files::open_project_deck(
        &mut state.model,
        &mut state.code.netlist,
        &mut state.code.netlist_open_documents,
        &mut state.runs,
    );
    let missing =
        deck_files::missing_deck(&state.model, &state.code.netlist).expect("the deck is missing");
    assert_eq!((missing.id, missing.name.as_str()), (id, "amp.cir"));
    assert_eq!(missing.path, gone);

    deck_files::locate_deck(
        &mut state.model,
        &mut state.code.netlist,
        &mut state.code.netlist_open_documents,
        &mut state.runs,
        id,
        &moved,
    )
    .expect("located");

    assert_eq!(state.model.simulation.source.netlist_content, DECK);
    assert_eq!(
        state.model.workspace.content.decks[0].file_path(None),
        Some(moved)
    );
    assert!(deck_files::missing_deck(&state.model, &state.code.netlist).is_none());
}

/// Remove from project takes the deck out of the project and closes it;
/// its file stays as it was. A deck holding edits is not removed.
#[test]
fn remove_from_project_leaves_the_file() {
    let folder = Folder::new();
    let path = folder.write("amp.cir", DECK);
    let mut state = opened(&path);
    let id = state.model.workspace.content.decks[0].id;
    assert!(edit_deck_file(
        &mut state.model,
        &mut state.code.netlist,
        EDITED.to_owned()
    ));
    assert_eq!(
        deck_files::remove_deck(
            &mut state.model,
            &mut state.code.netlist,
            &mut state.code.netlist_open_documents,
            id
        ),
        Err("Save amp.cir before removing amp.cir from the project.".to_owned())
    );
    deck_files::save_shown_file(&mut state.code.netlist).expect("saved");

    let removed = deck_files::remove_deck(
        &mut state.model,
        &mut state.code.netlist,
        &mut state.code.netlist_open_documents,
        id,
    );

    assert_eq!(removed, Ok("amp.cir".to_owned()));
    assert_eq!(std::fs::read_to_string(&path).expect("the deck"), EDITED);
    assert!(state.model.workspace.content.decks.is_empty());
    assert!(state.code.netlist.open_deck.is_none());
}

/// A library two lines name for two sections is what the check read for
/// each line, two rows of the Includes list; opening it from either is one
/// tab (NL-3).
#[test]
fn a_library_named_twice_is_two_lines_of_the_check_and_one_tab() {
    let folder = Folder::new();
    let library = folder.write(
        "m.lib",
        "* corners\n.lib tt\n.model dmod d is=1e-14\n.endl tt\n\
         .lib res\n.model dbig d is=1e-12\n.endl res\n",
    );
    let deck = folder.write(
        "two.cir",
        "* two sections\n.lib m.lib tt\n.lib m.lib res\nV1 in 0 1\nR1 in a 1k\n\
         D1 a 0 dmod\nD2 a 0 dbig\n.op\n.end\n",
    );
    let mut app = app_with_deck(&deck);
    frame(&themed_context(), &mut app, DESKTOP);

    let includes = app.state.code.netlist.diagnostics.includes();
    let lines = includes
        .iter()
        .map(|include| (include.line, include.section.as_deref()))
        .collect::<Vec<_>>();
    assert_eq!(lines, [(2, Some("tt")), (3, Some("res"))]);
    assert_eq!(includes[0].path, includes[1].path);
    let sections = includes[0]
        .sections
        .iter()
        .map(|section| section.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(sections, ["tt", "res"]);

    let state = &mut app.state;
    for _ in 0..2 {
        deck_files::show_include(
            &mut state.model,
            &mut state.code.netlist,
            &mut state.code.netlist_open_documents,
            &mut state.runs,
            &library,
        )
        .expect("the library opens");
        assert!(deck_files::show_deck(
            &mut state.model,
            &mut state.code.netlist
        ));
    }
    assert_eq!(
        state
            .code
            .netlist
            .open_deck
            .as_ref()
            .unwrap()
            .includes
            .len(),
        1
    );
}

/// An include found only in one of the project's include folders is what
/// the check reads and the run reads (NL-7).
#[test]
fn an_include_found_only_in_the_projects_include_folders_runs() {
    let folder = Folder::new();
    let models = folder.0.join("pdk");
    folder.write("pdk/diodes.inc", ".model dpdk d is=2e-14\n");
    let deck = folder.write(
        "decks/pdk.cir",
        "* pdk include\n.include diodes.inc\nV1 in 0 1\nR1 in a 1k\nD1 a 0 dpdk\n.op\n.end\n",
    );
    let mut app = app_with_deck(&deck);
    app.state
        .model
        .workspace
        .content
        .project
        .set_include_search_paths(vec![models])
        .expect("the include folder");
    app.state.model.invalidate_source_evidence();
    app.state.code.netlist.revision = app.state.code.netlist.revision.wrapping_add(1);
    frame(&themed_context(), &mut app, DESKTOP);

    let includes = app.state.code.netlist.diagnostics.includes();
    assert_eq!(includes.len(), 1, "{includes:?}");
    assert!(includes[0].path.ends_with("diodes.inc"));
    let netlist = dispatched(&mut app.state).expect("the deck runs");
    assert!(netlist.to_lowercase().contains("dpdk"), "{netlist}");
}

/// New deck writes a file the deck is read from; a relative and an
/// absolute include written in it are read from there, and nothing asks
/// where the deck comes from (NL-17).
#[test]
fn a_new_deck_with_a_relative_and_an_absolute_include_runs() {
    let folder = Folder::new();
    let absolute = folder.write("lib/abs.inc", ".model dabs d is=3e-14\n");
    folder.write("rel.inc", ".model drel d is=4e-14\n");
    let path = folder.0.join("new.cir");
    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state.new_deck_at(&path, true),
        "{:?}",
        app.state.prompt_text()
    );
    let text = format!(
        "* new deck\n.include rel.inc\n.include \"{}\"\nV1 in 0 1\nR1 in a 1k\n\
         D1 a 0 drel\nD2 a 0 dabs\n.op\n.end\n",
        absolute.display()
    );
    assert!(edit_deck_file(
        &mut app.state.model,
        &mut app.state.code.netlist,
        text.clone()
    ));
    deck_files::save_shown_file(&mut app.state.code.netlist).expect("saved");
    frame(&themed_context(), &mut app, DESKTOP);

    assert_eq!(std::fs::read_to_string(&path).expect("the deck"), text);
    assert_eq!(app.state.code.netlist.diagnostics.includes().len(), 2);
    let said = app
        .state
        .code
        .netlist
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.message.clone())
        .collect::<Vec<_>>();
    assert!(
        said.iter().all(|message| !message.contains("origin")),
        "{said:?}"
    );
    let netlist = dispatched(&mut app.state)
        .expect("the deck runs")
        .to_lowercase();
    assert!(
        netlist.contains("drel") && netlist.contains("dabs"),
        "{netlist}"
    );
}

/// A project saved to another folder keeps its deck where it is: by its
/// path from the new folder when the deck is under it, else by its whole
/// path; and the session reads the deck from the same file.
#[test]
fn a_project_saved_elsewhere_keeps_its_deck_where_it_is() {
    let folder = Folder::new();
    let deck = folder.write("one/decks/amp.cir", DECK);
    let first = folder.0.join("one").join("amp.rspiceproj");
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.save_project_to_path(&first), "the project saves");
    assert!(app.state.open_deck_at(&deck, false));
    assert!(app.state.save_project_to_path(&first), "the project saves");
    let stored = |path: &std::path::Path| -> String {
        std::fs::read_to_string(path)
            .expect("the project file")
            .split_whitespace()
            .collect()
    };
    assert!(stored(&first).contains(r#"{"kind":"file","path":"decks/amp.cir"}"#));

    let parent = folder.0.join("amp.rspiceproj");
    assert!(app.state.save_project_to_path(&parent), "the project saves");

    assert!(
        stored(&parent).contains(r#"{"kind":"file","path":"one/decks/amp.cir"}"#),
        "{}",
        stored(&parent)
    );
    let reads = |app: &RSpiceApp| {
        let content = &app.state.model.workspace.content;
        content.decks[0].is_file(&deck, content.project.data_root())
    };
    assert!(reads(&app));

    std::fs::create_dir_all(folder.0.join("two")).expect("the folder");
    let elsewhere = folder.0.join("two").join("amp.rspiceproj");
    assert!(
        app.state.save_project_to_path(&elsewhere),
        "the project saves"
    );

    assert!(!stored(&elsewhere).contains(r#""path":"one/decks/amp.cir""#));
    assert!(!stored(&elsewhere).contains(r#""path":"decks/amp.cir""#));
    assert!(reads(&app));
}
