//! The live editor resolves a deck's includes through the library folders
//! set in Preferences, as a run of the same deck does.

use super::*;
use crate::workbench::FolderListPreference;
use crate::workbench::code::tests::{CodeHarness, harness};

/// The errors the editor last published.
fn errors(harness: &CodeHarness) -> Vec<String> {
    harness
        .state
        .netlist
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
        .map(|diagnostic| diagnostic.message.clone())
        .collect()
}

#[test]
fn the_editor_resolves_an_include_only_a_library_folder_holds() {
    let directory = crate::fixture_root::canonical_temp_dir().join(format!(
        "rspice-editor-library-folder-{}",
        uuid::Uuid::new_v4()
    ));
    let deck = directory.join("deck");
    let shared = directory.join("shared");
    for folder in [&deck, &shared] {
        std::fs::create_dir_all(folder).expect("create fixture folder");
    }
    std::fs::write(
        shared.join("device.inc"),
        ".subckt cell a b\nR1 a b 1k\n.ends cell\n",
    )
    .expect("write library include");
    let path = deck.join("top.cir");
    let source = "deck\n.include device.inc\nX1 out 0 cell\nV1 out 0 1\n.op\n.end\n";
    std::fs::write(&path, source).expect("write deck");

    let mut harness = harness();
    harness.model.workspace.content.netlist_source_path = Some(path);
    harness.state.netlist.active_document = super::super::ActiveNetlistDocument::OwnedSource;
    harness.model.simulation.source.netlist_content = source.to_owned();
    let revision = harness.state.netlist.revision;
    validate_visible_source_in(
        &harness.model,
        &mut harness.state.netlist,
        &harness.state.code_workspace.include_resolutions,
        revision,
    );
    assert!(
        !errors(&harness).is_empty(),
        "nothing the deck is searched from holds device.inc"
    );

    // Nothing is edited: the folders changing is reason enough to parse the
    // deck again, once the edit debounce has long passed.
    harness
        .model
        .preferences
        .set_folders(FolderListPreference::LibraryFolders, vec![shared])
        .expect("a full path");
    harness.state.netlist.last_edit_time = f64::NEG_INFINITY;
    let ctx = egui::Context::default();
    let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
        refresh_diagnostics(
            ui,
            &harness.model,
            &mut harness.state.netlist,
            &harness.state.code_workspace.include_resolutions,
        );
    });
    assert_eq!(errors(&harness), Vec::<String>::new());

    std::fs::remove_dir_all(&directory).ok();
}
