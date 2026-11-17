//! The rows every place on the Dependencies page reads: one order and one
//! filter over both catalogues.

use rspice_app_state::workbench::workspace::restore_model_library_manager;

use crate::state::Library;
use crate::state::model_library::ModelLibrary;
use crate::state::pdk_config::PdkConfig;
use crate::workbench::app_state::Model;

use super::*;

fn model() -> Model {
    Model::session_start(PdkConfig::default(), None, restore_model_library_manager)
}

#[test]
fn a_row_is_found_by_anything_it_shows() {
    let row = DependencyRow {
        library: DependencyLibrary::model("models_tt"),
        version: "2.1".to_owned(),
        source: "/models/tt.lib".to_owned(),
        digest: "12ab".to_owned(),
        status: LibraryStatus::neutral("Found", None),
    };
    for shown in [
        "models_tt",
        "model library",
        "2.1",
        "tt.lib",
        "12ab",
        "found",
    ] {
        assert!(row.matches(shown), "{shown}");
    }
    assert!(!row.matches("missing"));
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

/// A selection naming no library of the project is replaced by the first
/// row shown; one naming a library is kept, even under a filter.
#[test]
fn a_selection_is_kept_only_while_it_names_a_library() {
    let mut model = model();
    model.library_manager.add_library(Library::new("cells"));
    let rows = build_rows(&model);
    let visible: Vec<usize> = (0..rows.len()).collect();

    let mut selection = Some(DependencyLibrary::design("gone"));
    settle_selection(&rows, &visible, &mut selection);
    assert_eq!(selection, Some(rows[0].library.clone()));

    let kept = DependencyLibrary::design("cells");
    let mut selection = Some(kept.clone());
    settle_selection(&rows, &[], &mut selection);
    assert_eq!(selection, Some(kept));
}
