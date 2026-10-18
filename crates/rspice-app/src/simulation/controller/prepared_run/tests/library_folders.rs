//! Library folders: the host's own include search, after the project's.
//!
//! A deck's include that only a library folder holds resolves in a run, the
//! project's own search paths win over the folders, the receipt names the
//! folder a file was read from, and a dispatch prepared under one list of
//! folders is refused under another.

use std::path::{Path, PathBuf};

use super::*;
use rspice_core::netlist::IncludeSearchStage;

const DECK: &str = "deck\n.include device.inc\nV1 out 0 1\n.op\n.end\n";

/// A manual deck in its own folder whose one include is in none of the
/// places a deck is searched from by default.
fn deck_in(directory: &Path) -> AppState {
    let deck = directory.join("deck");
    fs::create_dir_all(&deck).expect("create deck folder");
    let origin = deck.join("deck.cir");
    fs::write(&origin, DECK).expect("write deck");
    let mut state = AppState::default();
    state.simulation.execution.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.content.netlist_source = Some(DECK.to_owned());
    state.workspace.content.netlist_source_path = Some(origin);
    state
}

fn library(directory: &Path, name: &str, body: &str) -> PathBuf {
    let folder = directory.join(name);
    fs::create_dir_all(&folder).expect("create library folder");
    fs::write(folder.join("device.inc"), body).expect("write library include");
    folder
}

/// Whether `file` sits directly in `folder`. A sealed path is the host's
/// canonical one, which on Windows is the verbatim `\\?\` form.
fn read_from(file: &Path, folder: &Path) -> bool {
    let canonical = |path: &Path| fs::canonicalize(path).expect("an existing fixture path");
    file.parent().map(canonical) == Some(canonical(folder))
}

fn set_library_folders(state: &mut AppState, folders: Vec<PathBuf>) {
    state
        .ui
        .preferences
        .set_folders(FolderListPreference::LibraryFolders, folders)
        .expect("full library folder paths");
}

#[test]
fn a_run_resolves_an_include_only_a_library_folder_holds() {
    let directory = fixture_dir("library-folder-run");
    let folder = library(&directory, "shared", "R1 out 0 1k\n");
    let mut state = deck_in(&directory);
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect_err("nothing the deck is searched from holds device.inc");

    set_library_folders(&mut state, vec![folder.clone()]);
    let metadata = controller
        .validate_manual_deck_document(&state)
        .expect("the library folder holds device.inc");
    let [dependency] = metadata.sealed_source_dependencies.as_slice() else {
        panic!("one include: {:?}", metadata.sealed_source_dependencies);
    };
    assert!(read_from(dependency.resolved_path(), &folder));
    assert_eq!(
        dependency.resolution().stage(),
        IncludeSearchStage::LibraryPath(0)
    );
    let dispatch = controller
        .consume_snapshot_for_dispatch(&state)
        .expect("the validated deck dispatches");
    assert!(dispatch.executable_netlist().contains("R1 out 0 1k"));

    fs::remove_dir_all(directory).expect("remove library fixture");
}

#[test]
fn the_projects_own_search_path_wins_over_a_library_folder() {
    let directory = fixture_dir("library-folder-precedence");
    let project_models = library(&directory, "project-models", "R1 out 0 1k\n");
    let shared = library(&directory, "shared", "R1 out 0 9k\n");
    let mut state = deck_in(&directory);
    state
        .workspace
        .content
        .project
        .set_include_search_paths(vec![project_models.clone()])
        .expect("an absolute project search path");
    set_library_folders(&mut state, vec![shared.clone()]);

    let metadata = SimulationController::new()
        .validate_manual_deck_document(&state)
        .expect("both hold device.inc");
    let [dependency] = metadata.sealed_source_dependencies.as_slice() else {
        panic!("one include: {:?}", metadata.sealed_source_dependencies);
    };
    assert!(read_from(dependency.resolved_path(), &project_models));
    assert!(dependency.source().contains("1k"));
    assert!(
        dependency
            .resolution()
            .shadowed()
            .any(|candidate| read_from(candidate.path(), &shared)),
        "the library folder's copy is the one shadowed"
    );

    fs::remove_dir_all(directory).expect("remove library fixture");
}

/// Two folders holding the same bytes give the same deck, so only the
/// receipt can say which folder a run read. It does.
#[test]
fn a_receipt_names_the_library_folder_its_include_was_read_from() {
    let directory = fixture_dir("library-folder-receipt");
    let first = library(&directory, "first", "R1 out 0 1k\n");
    let second = library(&directory, "second", "R1 out 0 1k\n");
    let mut state = deck_in(&directory);

    set_library_folders(&mut state, vec![first.clone(), second.clone()]);
    let from_first = SimulationController::new()
        .validate_manual_deck_document(&state)
        .expect("the first folder holds device.inc");
    set_library_folders(&mut state, vec![second.clone(), first.clone()]);
    let from_second = SimulationController::new()
        .validate_manual_deck_document(&state)
        .expect("the second folder holds device.inc");

    assert_eq!(from_first.source_digest, from_second.source_digest);
    assert_ne!(from_first.receipt_digest, from_second.receipt_digest);
    assert!(read_from(
        from_first.sealed_source_dependencies[0].resolved_path(),
        &first
    ));
    assert!(read_from(
        from_second.sealed_source_dependencies[0].resolved_path(),
        &second
    ));

    fs::remove_dir_all(directory).expect("remove library fixture");
}

#[test]
fn changing_the_library_folders_after_validation_refuses_the_dispatch() {
    let directory = fixture_dir("library-folder-identity");
    let folder = library(&directory, "shared", "R1 out 0 1k\n");
    let mut state = deck_in(&directory);
    set_library_folders(&mut state, vec![folder]);
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("the library folder holds device.inc");

    set_library_folders(&mut state, Vec::new());
    let error = controller
        .consume_snapshot_for_dispatch(&state)
        .expect_err("the deck was prepared under other folders");
    assert_eq!(error.stage(), PreparationStage::Authorization);
    assert!(
        error.message().contains("the library folders changed"),
        "{}",
        error.message()
    );

    fs::remove_dir_all(directory).expect("remove library fixture");
}
