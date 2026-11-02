//! What the New project transaction is allowed to create.
//!
//! The mirror is the interesting case: it exists so the common path needs one
//! field, and it must stop the instant the reader disagrees with it, because a
//! library name that silently reverts is worse than one that was never
//! suggested.

use super::*;

fn opened() -> NewProjectDialogState {
    let mut state = AppState::default();
    open_new_project_dialog(&mut state);
    state.project.new_project.clone()
}

#[test]
fn library_mirrors_the_project_name_until_the_reader_edits_it() {
    let mut dialog = opened();
    assert!(dialog.open);
    assert_eq!(dialog.library, DEFAULT_PROJECT_LIBRARY);
    assert_eq!(dialog.top_cell, DEFAULT_TOP_CELL);
    assert!(dialog.name.is_empty());

    dialog.name = "Precision Sensor AFE".to_owned();
    dialog.mirror_library_from_name();
    assert_eq!(dialog.library, "precision_sensor_afe");

    // Editing Library retires the mirror; later name edits leave it alone.
    dialog.library_edited = true;
    dialog.library = "afe_top".to_owned();
    dialog.name = "Precision Sensor AFE rev B".to_owned();
    dialog.mirror_library_from_name();
    assert_eq!(dialog.library, "afe_top");
}

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

/// An empty field is not filled in yet, which is not an error: Create
/// stays disabled and the footer says, as a fact, what it waits for.
#[test]
fn an_empty_field_says_what_create_waits_for_instead_of_refusing_it() {
    let mut dialog = opened();
    assert!(dialog.params().validate().is_err(), "Create waits");
    assert_eq!(dialog.footer(), Ok("Type a project name."));

    dialog.name = "Sensor bridge".to_owned();
    dialog.library = "  ".to_owned();
    assert_eq!(dialog.footer(), Ok("Type a library name."));
    dialog.library = "afe".to_owned();
    dialog.top_cell = String::new();
    assert_eq!(dialog.footer(), Ok("Type a top cell name."));

    dialog.top_cell = "core".to_owned();
    assert!(dialog.params().validate().is_ok());
    assert_eq!(
        dialog.footer(),
        Ok("Nothing is written to disk until you save.")
    );
}

/// A refusal is one sentence that names its field once, in the reader's
/// words rather than the project file's.
#[test]
fn a_refused_name_is_one_sentence_naming_its_field_once() {
    let mut dialog = opened();
    let long = "x".repeat(121);
    let mut refusals = Vec::new();
    for (name, library) in [
        ("Sensor/bridge", "afe"),
        ("Tab\tname", "afe"),
        (long.as_str(), "afe"),
        ("Sensor bridge", "afe-top"),
    ] {
        dialog.name = name.to_owned();
        dialog.library = library.to_owned();
        let refusal = dialog
            .footer()
            .expect_err("a filled, invalid field refuses");
        assert_eq!(dialog.params().validate(), Err(refusal.clone()));
        refusals.push(refusal);
    }
    assert_eq!(
        refusals,
        [
            "The project name cannot contain / or \\.",
            "The project name cannot contain a control character.",
            "The project name is 121 characters long; the limit is 120.",
            "The library name can use only letters, numbers and underscores.",
        ]
    );
    for refusal in &refusals {
        assert!(!refusal.contains("grapheme"), "{refusal}");
        assert_eq!(refusal.matches("name").count(), 1, "{refusal}");
    }
}

#[test]
fn invalid_library_and_top_cell_segments_are_refused_by_field() {
    let mut dialog = opened();
    dialog.name = "Sensor bridge".to_owned();

    for invalid in ["afe top", "afe-top", ""] {
        dialog.library = invalid.to_owned();
        let error = dialog
            .params()
            .validate()
            .expect_err("an invalid library segment is refused");
        assert!(error.contains("library name"), "{invalid}: {error}");
    }

    dialog.library = "afe".to_owned();
    for invalid in ["top cell", "top-cell", ""] {
        dialog.top_cell = invalid.to_owned();
        let error = dialog
            .params()
            .validate()
            .expect_err("an invalid top cell segment is refused");
        assert!(error.contains("top cell name"), "{invalid}: {error}");
    }

    dialog.top_cell = "core".to_owned();
    assert!(dialog.params().validate().is_ok());
}

#[test]
fn a_reviewed_identity_is_trimmed_exactly_as_creation_records_it() {
    let mut dialog = opened();
    dialog.name = "  Sensor bridge  ".to_owned();
    dialog.library = " afe ".to_owned();
    dialog.top_cell = " core ".to_owned();

    let params = dialog.params();

    assert_eq!(params.name, "Sensor bridge");
    assert_eq!(params.root_library, "afe");
    assert_eq!(params.top_cell, "core");
    assert!(params.validate().is_ok());
}
