//! The rows every place on the Dependencies page reads: one order and one
//! filter over everything the project reads.

use rspice_app_state::workbench::workspace::restore_model_library_manager;

use crate::state::Library;
use crate::state::model_library::ModelLibrary;
use crate::state::pdk_config::PdkConfig;
use crate::workbench::app_state::Model;

use super::*;

fn model() -> Model {
    Model::session_start(PdkConfig::default(), None, restore_model_library_manager)
}

/// A row is found by anything it shows: its contents, and its status only
/// when that is a problem, since a sound row says none.
#[test]
fn a_row_is_found_by_anything_it_shows() {
    let mut row = DependencyRow {
        library: DependencyLibrary::model("models_tt"),
        version: "2.1".to_owned(),
        source: "/models/tt.lib".to_owned(),
        location: "../models/tt.lib".to_owned(),
        resolved: Some("/models/tt.lib".to_owned()),
        access: "Read-only",
        digest: "12ab".to_owned(),
        contents: Some(Contents::of(21, "model", "models").and(3, "subckt", "subckts")),
        status: LibraryStatus::neutral("Found", None),
    };
    for shown in [
        "models_tt",
        "model library",
        "2.1",
        "tt.lib",
        "../models",
        "12ab",
        "21 models",
        "3 subckts",
    ] {
        assert!(row.matches(shown), "{shown}");
    }
    assert!(!row.matches("missing"));
    assert!(!row.matches("found"), "a sound row says no status");
    row.status = LibraryStatus {
        mark: Tone::Error,
        label: "Not found",
        detail: None,
    };
    assert!(row.matches("not found"));
}

/// Design libraries come first, then model libraries, each by name without
/// regard to case, so "alpha" is not listed after "Zeta" and a design and a
/// model library of one name are two rows.
#[test]
fn rows_list_design_then_model_libraries_by_name_without_regard_to_case() {
    let mut model = model();
    for name in ["Zeta", "alpha", "shared"] {
        model.library_manager.add_library(Library::new(name));
    }
    model
        .model_library_manager
        .add_library(ModelLibrary::new("shared"));
    let rows = build_rows(&model);
    let listed: Vec<_> = rows
        .iter()
        .map(|row| (row.library.kind, row.library.name.as_str()))
        .collect();
    let design: Vec<_> = listed
        .iter()
        .filter(|(kind, _)| *kind == DependencyKind::Design)
        .map(|(_, name)| *name)
        .collect();
    let first_model = listed
        .iter()
        .position(|(kind, _)| *kind == DependencyKind::Model)
        .expect("a model library is listed");
    assert!(
        listed[..first_model]
            .iter()
            .all(|(kind, _)| *kind == DependencyKind::Design),
        "{listed:?}"
    );
    let alpha = design.iter().position(|name| *name == "alpha");
    let zeta = design.iter().position(|name| *name == "Zeta");
    assert!(alpha < zeta, "{design:?}");
    assert!(listed.contains(&(DependencyKind::Design, "shared")));
    assert!(listed.contains(&(DependencyKind::Model, "shared")));
}

/// A canonical Windows path is shown as the reader wrote it; a device path,
/// which has no other spelling, keeps its prefix.
#[test]
fn a_path_is_shown_without_the_prefix_canonicalising_adds() {
    for (stored, shown) in [
        (r"\\?\C:\models\tt.lib", r"C:\models\tt.lib"),
        (r"\\?\UNC\server\share\tt.lib", r"\\server\share\tt.lib"),
        (
            r"\\?\GLOBALROOT\Device\tt.lib",
            r"\\?\GLOBALROOT\Device\tt.lib",
        ),
        ("/models/tt.lib", "/models/tt.lib"),
    ] {
        assert_eq!(shown_path(std::path::Path::new(stored)), shown);
    }
}

/// A selection naming nothing the project reads is dropped, so the
/// inspector states the project; one naming a dependency is kept.
#[test]
fn a_selection_is_kept_only_while_it_names_a_dependency() {
    let mut model = model();
    model.library_manager.add_library(Library::new("cells"));
    let rows = build_rows(&model);

    let mut selection = Some(DependencyLibrary::design("gone"));
    settle_selection(&rows, &mut selection);
    assert_eq!(selection, None);

    let kept = DependencyLibrary::design("cells");
    let mut selection = Some(kept.clone());
    settle_selection(&rows, &mut selection);
    assert_eq!(selection, Some(kept));
}

/// An include search path is listed by the folder it names, the path the
/// project stores telling two of one name apart, in the order the folders
/// are searched rather than by name; one no folder answers to is not found.
#[test]
fn include_search_paths_are_rows_in_search_order() {
    let folder = std::env::temp_dir().join(format!("rspice-include-rows-{}", std::process::id()));
    std::fs::create_dir_all(folder.join("zeta")).expect("the folder is made");
    std::fs::create_dir_all(folder.join("alpha")).expect("the folder is made");
    let mut model = model();
    model
        .workspace
        .content
        .project
        .set_include_search_paths(vec![
            folder.join("zeta"),
            folder.join("alpha"),
            folder.join("gone"),
        ])
        .expect("three folders are a chain");
    let rows = build_rows(&model);
    let paths: Vec<_> = rows
        .iter()
        .filter(|row| row.library.kind == DependencyKind::IncludePath)
        .map(|row| (row.library.shown_name(), row.status.label))
        .collect();
    assert_eq!(
        paths,
        [("zeta", "Found"), ("alpha", "Found"), ("gone", "Not found")]
    );
    let _ = std::fs::remove_dir_all(folder);
}

/// A path is read from the project folder when it is in it or beside it,
/// and whole when it is farther away.
#[test]
fn a_location_is_read_from_the_project_folder_when_it_is_near() {
    let project = std::path::Path::new("/work/chip/design");
    for (path, shown) in [
        ("/work/chip/design/models/tt.lib", "models/tt.lib"),
        ("/work/chip/design", "."),
        ("/work/chip/libraries/vendor", "../libraries/vendor"),
        ("/work/chip/design/./../models/tt.lib", "../models/tt.lib"),
        ("/opt/pdk/demo180/models.lib", "/opt/pdk/demo180/models.lib"),
    ] {
        let found = project_relative(std::path::Path::new(path), Some(project));
        assert_eq!(
            found.replace(std::path::MAIN_SEPARATOR, "/"),
            shown,
            "{path}"
        );
    }
}

/// A deck kept as a file is a row, and each file its include lines name is
/// a row of its own: found beside the deck or along the include search
/// paths, or not found, with the line that names it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_deck_file_lists_the_files_it_includes_found_or_not() {
    use crate::state::{DeckSource, ProjectDeck};

    let folder = std::env::temp_dir().join(format!("rspice-deck-rows-{}", std::process::id()));
    std::fs::create_dir_all(folder.join("models")).expect("the folder is made");
    std::fs::write(folder.join("models").join("tt.lib"), "* corners\n").expect("it writes");
    std::fs::write(
        folder.join("amp.cir"),
        "* amp\n.include models/tt.lib\n.include gone.lib\n.end\n",
    )
    .expect("it writes");
    let mut model = model();
    model.workspace.content.decks.push(ProjectDeck {
        id: rspice_design::deck::DeckId::new(),
        source: DeckSource::File {
            path: folder.join("amp.cir"),
        },
    });
    let rows = build_rows(&model);
    let files: Vec<_> = rows
        .iter()
        .filter(|row| {
            matches!(
                row.library.kind,
                DependencyKind::Deck | DependencyKind::Include
            )
        })
        .map(|row| (row.library.kind, row.library.shown_name(), row.status.label))
        .collect();
    assert_eq!(
        files,
        [
            (DependencyKind::Deck, "amp.cir", "Found"),
            (DependencyKind::Include, "gone.lib", "Not found"),
            (DependencyKind::Include, "tt.lib", "Found"),
        ]
    );
    let gone = rows
        .iter()
        .find(|row| row.library.name == "gone.lib")
        .expect("the missing include is a row");
    assert!(
        gone.status
            .detail
            .as_deref()
            .is_some_and(|detail| detail.starts_with("amp.cir line 3")),
        "{:?}",
        gone.status.detail
    );
    let _ = std::fs::remove_dir_all(folder);
}
