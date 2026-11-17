//! What the New project transaction is allowed to create.
//!
//! The mirror is the interesting case: it exists so the common path needs one
//! field, and it must stop the instant the reader disagrees with it, because a
//! library name that silently reverts is worse than one that was never
//! suggested.

use super::*;

#[test]
fn the_slug_collapses_runs_and_never_yields_an_invalid_segment() {
    assert_eq!(library_slug("Sensor  bridge -- v2"), "sensor_bridge_v2");
    assert_eq!(
        library_slug("__leading and trailing__"),
        "leading_and_trailing"
    );
    assert_eq!(library_slug("///"), DEFAULT_PROJECT_LIBRARY);
    assert_eq!(library_slug(""), DEFAULT_PROJECT_LIBRARY);
    for name in ["Sensor  bridge -- v2", "__x__", "///", "\u{c9}tage"] {
        let slug = library_slug(name);
        assert!(
            crate::state::workspace::validate_cell_view_name_segment(&slug).is_ok(),
            "{name} -> {slug}"
        );
    }
}

/// A project name becomes a file and folder name every system takes, and the
/// file lands in a folder of that name inside the location.
#[test]
fn the_project_file_is_named_for_the_project_in_a_folder_of_its_own() {
    assert_eq!(project_file_stem("Sensor bridge"), "Sensor bridge");
    assert_eq!(project_file_stem("A: B? <C>"), "A_ B_ _C_");
    assert_eq!(project_file_stem("Trailing dots.."), "Trailing dots");
    assert_eq!(project_file_stem("con"), "con_");
    assert_eq!(project_file_stem("COM1.v2"), "COM1_.v2");
    assert_eq!(project_file_stem("..."), "project");

    let location = std::env::temp_dir();
    let dialog = NewProjectDialogState {
        name: "  Amp: rev B ".to_owned(),
        location: location.display().to_string(),
        ..NewProjectDialogState::default()
    };
    assert_eq!(
        dialog.project_file(),
        Ok(location.join("Amp_ rev B").join("Amp_ rev B.rspiceproj"))
    );
}
