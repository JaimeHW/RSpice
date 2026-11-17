//! The New project dialog's fields as the whole state opens them: the name
//! offered, the library following it, and what each field refuses and why.

use crate::workbench::app::project_transactions::open_new_project_dialog;
use crate::workbench::app_state::AppState;
use rspice_app_project::workbench::app::dialogs::project::new_project::{
    LOCATION_NOT_WHOLE, LOCATION_WAIT, NewProjectDialogState, NewProjectField,
};
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::workspace::DEFAULT_TOP_CELL;

/// One edit to the fields.
type Edit = fn(&mut NewProjectDialogState);

fn opened() -> NewProjectDialogState {
    let mut state = AppState::default();
    open_new_project_dialog(&mut state);
    let mut dialog = state.project.new_project.clone();
    dialog.location = unique_temp_dir("new-project-fields").display().to_string();
    dialog
}

/// The dialog offers a name that can be created as it is, and the library
/// follows the name until the reader edits the library.
#[test]
fn library_follows_the_project_name_until_the_reader_edits_it() {
    let mut dialog = opened();
    assert!(dialog.open);
    assert_eq!(dialog.name, "Untitled Project");
    assert_eq!(dialog.library, "untitled_project");
    assert_eq!(dialog.top_cell, DEFAULT_TOP_CELL);
    assert_eq!(dialog.creatable(), Ok(()));
    assert_eq!(
        dialog.footer(),
        None,
        "the footer is silent while Create can go"
    );

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

/// The name offered is numbered past a project folder of that name already
/// in the location.
#[test]
fn the_name_offered_is_one_the_location_does_not_hold() {
    let mut dialog = opened();
    let location = std::path::PathBuf::from(&dialog.location);
    std::fs::create_dir(location.join("Untitled Project")).unwrap();
    std::fs::create_dir(location.join("Untitled Project 2")).unwrap();
    dialog.offer_free_name();
    assert_eq!(dialog.name, "Untitled Project 3");
    assert_eq!(dialog.library, "untitled_project_3");
}

/// Each field says why it refuses what it holds, and Create waits on the
/// first refusal in the dialog's order, which the footer states.
#[test]
fn each_field_says_why_it_refuses_what_it_holds() {
    let mut dialog = opened();
    let cases: [(NewProjectField, Edit, &str); 5] = [
        (
            NewProjectField::Name,
            |dialog| dialog.name = "  ".to_owned(),
            "Type a project name.",
        ),
        (
            NewProjectField::Location,
            |dialog| dialog.location.clear(),
            LOCATION_WAIT,
        ),
        (
            NewProjectField::Location,
            |dialog| dialog.location = "projects".to_owned(),
            LOCATION_NOT_WHOLE,
        ),
        (
            NewProjectField::Library,
            |dialog| dialog.library = "  ".to_owned(),
            "Type a library name.",
        ),
        (
            NewProjectField::TopCell,
            |dialog| dialog.top_cell.clear(),
            "Type a top cell name.",
        ),
    ];
    for (field, edit, refusal) in cases {
        let mut edited = dialog.clone();
        edit(&mut edited);
        assert_eq!(edited.refusal(field).as_deref(), Some(refusal), "{field:?}");
        for other in NewProjectField::ALL
            .into_iter()
            .filter(|other| *other != field)
        {
            assert_eq!(
                edited.refusal(other),
                None,
                "{field:?} leaves {other:?} alone"
            );
        }
        assert_eq!(edited.creatable(), Err(refusal.to_owned()));
        assert_eq!(edited.footer().as_deref(), Some(refusal));
    }

    dialog.name.clear();
    dialog.top_cell.clear();
    assert_eq!(
        dialog.footer().as_deref(),
        Some("Type a project name."),
        "the first refusal in the dialog's order"
    );
}

/// The location is refused when it cannot take the project's folder: not
/// there, a file, or already holding a folder of the project's name with
/// something in it. An empty folder of that name is taken.
#[test]
fn a_location_that_cannot_take_the_project_folder_is_refused() {
    let mut dialog = opened();
    let location = std::path::PathBuf::from(&dialog.location);
    dialog.name = "Sensor bridge".to_owned();
    let refusal = |dialog: &NewProjectDialogState| dialog.refusal(NewProjectField::Location);

    let occupied = location.join("Sensor bridge");
    std::fs::create_dir(&occupied).unwrap();
    assert_eq!(
        refusal(&dialog),
        None,
        "an empty folder of the name is taken"
    );
    std::fs::write(occupied.join("notes.txt"), "kept").unwrap();
    assert_eq!(
        refusal(&dialog).as_deref(),
        Some("\u{201c}Sensor bridge\u{201d} already exists here and is not empty.")
    );
    assert!(dialog.creatable().is_err());

    dialog.location = location.join("missing").display().to_string();
    assert_eq!(
        refusal(&dialog).as_deref(),
        Some("This folder does not exist.")
    );
    dialog.location = occupied.join("notes.txt").display().to_string();
    assert_eq!(
        refusal(&dialog).as_deref(),
        Some("This is a file, not a folder.")
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
        let refusal = dialog.footer().expect("a filled, invalid field refuses");
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
fn invalid_library_and_top_cell_names_are_refused_by_their_own_field() {
    let mut dialog = opened();
    dialog.name = "Sensor bridge".to_owned();

    for invalid in ["afe top", "afe-top", ""] {
        dialog.library = invalid.to_owned();
        let refusal = dialog
            .refusal(NewProjectField::Library)
            .expect("an invalid library name is refused");
        assert!(refusal.contains("library name"), "{invalid}: {refusal}");
        assert_eq!(dialog.refusal(NewProjectField::TopCell), None);
    }

    dialog.library = "afe".to_owned();
    for invalid in ["top cell", "top-cell", ""] {
        dialog.top_cell = invalid.to_owned();
        let refusal = dialog
            .refusal(NewProjectField::TopCell)
            .expect("an invalid top cell name is refused");
        assert!(refusal.contains("top cell name"), "{invalid}: {refusal}");
        assert_eq!(dialog.refusal(NewProjectField::Library), None);
    }

    dialog.top_cell = "core".to_owned();
    assert_eq!(dialog.creatable(), Ok(()));
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
