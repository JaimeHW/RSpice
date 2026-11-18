use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::*;
use crate::library::{Cell, Library, View, ViewType};

const WINDOWS: HostPlatform = HostPlatform::Windows;
const UNIX: HostPlatform = HostPlatform::Unix;

fn no_environment(_: &str) -> Option<String> {
    None
}

fn written(choice: &LibraryPathChoice) -> (String, Vec<String>) {
    (
        choice.stored.as_str().to_owned(),
        choice
            .others
            .iter()
            .map(|path| path.as_str().to_owned())
            .collect(),
    )
}

#[test]
fn attach_writes_a_folder_inside_the_project_relative_to_it() {
    let roots = [("RSPICE_LIBS".to_owned(), "C:/proj".to_owned())];
    let choice = choose_library_path(
        Path::new(r"C:\proj\libs\analog"),
        Some(Path::new(r"C:\proj")),
        &roots,
        WINDOWS,
    )
    .unwrap();
    assert_eq!(
        written(&choice),
        (
            "libs/analog".to_owned(),
            vec![
                "$RSPICE_LIBS/libs/analog".to_owned(),
                "C:/proj/libs/analog".to_owned()
            ]
        )
    );
}

#[test]
fn attach_writes_a_folder_under_a_defined_root_by_the_root() {
    let roots = [
        ("EDA".to_owned(), "C:/eda".to_owned()),
        ("RSPICE_LIBS".to_owned(), r"C:\eda\libs".to_owned()),
    ];
    let choice = choose_library_path(
        Path::new("C:/eda/libs/analog"),
        Some(Path::new("C:/work/amp")),
        &roots,
        WINDOWS,
    )
    .unwrap();
    // The deepest root names it; the same drive still offers relative.
    assert_eq!(choice.stored.as_str(), "$RSPICE_LIBS/analog");
    assert_eq!(choice.stored.form(), LibraryPathForm::Root);
    assert!(
        choice
            .others
            .iter()
            .any(|path| path.as_str() == "../../eda/libs/analog")
    );
}

#[test]
fn attach_writes_relative_on_the_same_drive_and_absolute_across_drives() {
    let same = choose_library_path(
        Path::new("C:/shared/analog"),
        Some(Path::new("C:/work/amp")),
        &[],
        WINDOWS,
    )
    .unwrap();
    assert_eq!(same.stored.as_str(), "../../shared/analog");
    let across = choose_library_path(
        Path::new(r"\\server\share\analog"),
        Some(Path::new("C:/work/amp")),
        &[],
        WINDOWS,
    )
    .unwrap();
    assert_eq!(across.stored.as_str(), "//server/share/analog");
    assert_eq!(across.stored.form(), LibraryPathForm::Absolute);
    assert!(across.others.is_empty());
}

#[test]
fn a_typed_backslash_and_a_verbatim_prefix_are_never_stored() {
    assert_eq!(
        LibraryPath::new(r"\\?\C:\eda\analog\").unwrap().as_str(),
        "C:/eda/analog"
    );
    assert_eq!(
        LibraryPath::new(r"\\?\UNC\server\share\analog")
            .unwrap()
            .as_str(),
        "//server/share/analog"
    );
    assert_eq!(
        LibraryPath::new(r"..\shared\.\analog").unwrap().as_str(),
        "../shared/analog"
    );
    assert_eq!(
        LibraryPath::new("~/libs/analog"),
        Err(LibraryPathError::HomeFolder)
    );
    assert_eq!(
        LibraryPath::new("$1BAD/analog"),
        Err(LibraryPathError::BadRootName("1BAD".to_owned()))
    );
}

#[test]
fn a_preferences_value_beats_the_environment() {
    let preferences = BTreeMap::from([("RSPICE_LIBS".to_owned(), "/opt/team".to_owned())]);
    let environment = |name: &str| (name == "RSPICE_LIBS").then(|| "/opt/elsewhere".to_owned());
    let roots = RootTable {
        preferences: &preferences,
        environment: &environment,
    };
    let path = LibraryPath::new("$RSPICE_LIBS/analog").unwrap();
    assert_eq!(
        path.resolve(None, &roots, UNIX),
        Ok(PathBuf::from("/opt/team/analog"))
    );
    let blank = BTreeMap::from([("RSPICE_LIBS".to_owned(), " ".to_owned())]);
    let roots = RootTable {
        preferences: &blank,
        environment: &environment,
    };
    assert_eq!(
        path.resolve(None, &roots, UNIX),
        Ok(PathBuf::from("/opt/elsewhere/analog"))
    );
}

#[test]
fn an_unset_root_and_a_path_from_another_platform_name_no_folder_with_the_reason() {
    let preferences = BTreeMap::new();
    let roots = RootTable {
        preferences: &preferences,
        environment: &no_environment,
    };
    let unset = LibraryPath::new("$RSPICE_LIBS/analog")
        .unwrap()
        .resolve(None, &roots, UNIX)
        .unwrap_err();
    assert_eq!(
        unset.to_string(),
        "$RSPICE_LIBS is not set on this computer"
    );
    let drive = LibraryPath::new("C:/eda/analog")
        .unwrap()
        .resolve(None, &roots, UNIX)
        .unwrap_err();
    assert_eq!(drive.to_string(), "Written on another kind of computer");
    let posix = LibraryPath::new("/opt/libs/analog")
        .unwrap()
        .resolve(None, &roots, WINDOWS);
    assert_eq!(posix, Err(LibraryPathError::OtherPlatform));
    let nested = BTreeMap::from([("A".to_owned(), "$B/x".to_owned())]);
    let roots = RootTable {
        preferences: &nested,
        environment: &no_environment,
    };
    assert_eq!(
        LibraryPath::new("$A/lib")
            .unwrap()
            .resolve(None, &roots, UNIX),
        Err(LibraryPathError::RootNamesRoot("A".to_owned()))
    );
    let relative = LibraryPath::new("lib").unwrap().resolve(None, &roots, UNIX);
    assert_eq!(relative, Err(LibraryPathError::NoProjectFolder));
}

#[test]
fn a_relative_path_resolves_against_the_project_folder_and_follows_a_save_as() {
    let preferences = BTreeMap::new();
    let roots = RootTable {
        preferences: &preferences,
        environment: &no_environment,
    };
    let path = LibraryPath::new("../shared/analog").unwrap();
    assert_eq!(
        path.resolve(Some(Path::new("/home/a/work")), &roots, UNIX),
        Ok(PathBuf::from("/home/a/shared/analog"))
    );
    let moved = path
        .rebased(
            Path::new("/home/a/work"),
            Path::new("/home/a/archive/2026"),
            UNIX,
        )
        .unwrap();
    assert_eq!(moved.as_str(), "../../shared/analog");
    assert_eq!(
        moved.resolve(Some(Path::new("/home/a/archive/2026")), &roots, UNIX),
        Ok(PathBuf::from("/home/a/shared/analog"))
    );
    let rooted = LibraryPath::new("$RSPICE_LIBS/analog").unwrap();
    assert_eq!(
        rooted
            .rebased(Path::new("/a"), Path::new("/b"), UNIX)
            .unwrap(),
        rooted
    );
}

#[test]
fn a_library_file_is_named_by_its_place_in_the_folder() {
    let view = View::new("behavioral", ViewType::VerilogA);
    let files = [
        (LibraryFile::Manifest, "library.rspicelib"),
        (LibraryFile::cell("opamp"), "opamp/cell.json"),
        (
            LibraryFile::cell_view("opamp", &view),
            "opamp/behavioral/veriloga.json",
        ),
    ];
    for (file, text) in files {
        assert_eq!(file.relative_path(), text);
        assert_eq!(text.parse::<LibraryFile>().unwrap(), file);
    }
    assert_eq!(
        LibraryFile::cell("opamp").lock_path(),
        "opamp/cell.json.lock"
    );
    assert!(
        "opamp/schematic/nonsense.json"
            .parse::<LibraryFile>()
            .is_err()
    );
}

#[test]
fn one_content_is_one_byte_string_whatever_order_its_maps_were_built_in() {
    let mut first = Cell::new("opamp");
    let mut second = Cell::new("opamp");
    for (key, value) in [("a", "1"), ("b", "2"), ("c", "3"), ("d", "4")] {
        first.metadata.insert(key.to_owned(), value.to_owned());
    }
    for (key, value) in [("d", "4"), ("c", "3"), ("b", "2"), ("a", "1")] {
        second.metadata.insert(key.to_owned(), value.to_owned());
    }
    let mut view = View::new("symbol", ViewType::Symbol);
    view.metadata.insert("pins".to_owned(), "3".to_owned());
    view.metadata.insert("body".to_owned(), "box".to_owned());
    view.modified = true;
    view.saved_at = Some(7);
    let left = encode_library_file(&CellFile::of_cell(&first, 3)).unwrap();
    let right = encode_library_file(&CellFile::of_cell(&second, 3)).unwrap();
    assert_eq!(left, right);
    let encoded = encode_library_file(&CellViewFile::of_view(&view, 1)).unwrap();
    let decoded = CellViewFile::decode(&encoded.bytes).unwrap();
    // Whether a view has unsaved edits, and when it was saved, describe a
    // session, not the file.
    let mut clean = view.clone();
    clean.modified = false;
    clean.saved_at = None;
    assert_eq!(decoded.view, ViewRecord::of_view(&clean));
    let restored = decoded.view.view();
    assert!(!restored.modified);
    assert_eq!(restored.saved_at, None);
    assert_eq!(
        encode_library_file(&decoded).unwrap(),
        encoded,
        "a file read and written again is the same bytes"
    );
    assert_eq!(encoded.digest, digest_file_bytes(&encoded.bytes));
}

#[test]
fn a_manifest_names_its_library_and_a_newer_format_says_so() {
    let mut library = Library::new("analog");
    library.technology = "gpdk045".to_owned();
    library
        .metadata
        .insert("display_name".to_owned(), "Analog parts".to_owned());
    let id = uuid::Uuid::from_u128(7);
    let manifest = LibraryManifest::of_library(&library, id, 2);
    let encoded = encode_library_file(&manifest).unwrap();
    let decoded = LibraryManifest::decode(&encoded.bytes).unwrap();
    assert_eq!(decoded, manifest);
    assert_eq!(decoded.display_name(), "Analog parts");
    let rebuilt = decoded.library(true);
    assert_eq!(rebuilt.name, "analog");
    assert!(rebuilt.read_only);
    let mut newer = serde_json::to_value(&manifest).unwrap();
    newer["version"] = serde_json::json!(LIBRARY_FILE_VERSION + 1);
    let error = LibraryManifest::decode(newer.to_string().as_bytes()).unwrap_err();
    assert_eq!(
        error,
        LibraryFileError::NewerFormat(LIBRARY_FILE_VERSION + 1)
    );
    let error = LibraryManifest::decode(b"{\"name\": \"x\"}").unwrap_err();
    assert!(matches!(error, LibraryFileError::NotALibrary(_)));
}

#[test]
fn each_status_says_its_word_mark_and_detail() {
    let found = ReferenceStatus::Found {
        mode: LibraryMode::Editable,
        moved_on: 2,
        read_for_session: None,
    };
    assert_eq!(found.label(), "Found");
    assert_eq!(found.mark(), StatusMark::None);
    assert_eq!(
        found.detail().as_deref(),
        Some("Editable \u{00b7} 2 cellviews newer than the project's save")
    );
    let changed = ReferenceStatus::Changed { cellviews: 3 };
    assert_eq!(changed.detail().as_deref(), Some("3 cellviews changed"));
    assert_eq!(changed.mark(), StatusMark::Caution);
    let different = ReferenceStatus::DifferentLibrary {
        holds: "digital".to_owned(),
    };
    assert_eq!(
        different.detail().as_deref(),
        Some("The folder holds library 'digital'")
    );
    let cellview = CellViewStatus::Found {
        revision: 7,
        saved_with: Some(5),
    };
    assert_eq!(
        cellview.detail().as_deref(),
        Some("Revision 7 \u{00b7} project saved with 5")
    );
    let editing = CellViewStatus::ChangedSinceEditing {
        loaded: 7,
        on_disk: 8,
    };
    assert_eq!(editing.label(), CHANGED_SINCE_EDITING);
    assert_eq!(editing.mark(), StatusMark::Error);
    assert_eq!(
        editing.detail().as_deref(),
        Some("Revision 8 on disk \u{00b7} 7 loaded")
    );
}
