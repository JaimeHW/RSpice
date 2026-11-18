//! Library folders on disk: bytes, round trips, unreadable files and locks.

use super::*;
use crate::file::{decode_project_text, serialize_project_file};
use crate::{LibraryReference, ProjectFile, ProjectLibraries, ProjectWorkspace};
use rspice_design::library::folder::{LibraryPath, encode_library_file};
use rspice_design::library::{Cell, Library, View, ViewType};
use rspice_design::schematic::owned::Schematic;
use rspice_design_model::cell_view::CellViewRef;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A folder of its own under the system's temporary folder, removed when
/// the test ends.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "rspice-library-folder-{name}-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

const LIBRARY: &str = "shared_parts";

/// A workspace whose library `shared_parts` holds an amplifier drawn as a
/// schematic and a symbol.
fn workspace_with_library() -> (ProjectWorkspace, ProjectLibraries) {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = ProjectLibraries::default();
    let mut top = Cell::new(workspace.project.top_cell.clone());
    top.add_view(View::new(
        workspace.active_view.view.clone(),
        ViewType::Schematic,
    ));
    let mut root = Library::new(workspace.project.root_library.clone());
    root.add_cell(top);
    libraries.add_library(root);

    let mut amplifier = Cell::new("folded_cascode");
    amplifier.description = "Two-stage amplifier".to_owned();
    amplifier.add_view(View::new("schematic", ViewType::Schematic));
    amplifier.add_view(View::new("symbol", ViewType::Symbol));
    let mut library = Library::new(LIBRARY);
    library.technology = "gpdk045".to_owned();
    library.add_cell(amplifier);
    libraries.add_library(library);
    workspace.schematic_buffers.insert(
        CellViewRef::new(LIBRARY, "folded_cascode", "schematic").key(),
        Schematic::default(),
    );
    (workspace, libraries)
}

fn write_image(folder: &Path, image: &LibraryImage) -> BTreeMap<LibraryFile, Vec<u8>> {
    let mut written = BTreeMap::new();
    for file in image.files() {
        let encoded = image.encode(&file, 1).unwrap().unwrap();
        write_library_file(folder, &file, &encoded.bytes).unwrap();
        written.insert(file, encoded.bytes);
    }
    written
}

fn lock(user: &str, process: u32, project: &str) -> LibraryFileLock {
    LibraryFileLock {
        user: user.to_owned(),
        computer: "LAB-PC3".to_owned(),
        process,
        project: project.to_owned(),
        since_unix_ms: 1_700_000_000_000,
    }
}

fn amplifier_schematic() -> LibraryFile {
    LibraryFile::cell_view(
        "folded_cascode",
        &View::new("schematic", ViewType::Schematic),
    )
}

#[test]
fn a_library_written_to_its_folder_reads_back_to_the_same_bytes() {
    let scratch = Scratch::new("bytes");
    let (workspace, libraries) = workspace_with_library();
    let image = workspace
        .library_image(&libraries, LIBRARY, uuid::Uuid::nil())
        .unwrap();
    let written = write_image(&scratch.0, &image);
    assert!(scratch.0.join(LIBRARY_MANIFEST_FILE).is_file());
    assert!(
        scratch
            .0
            .join("folded_cascode/schematic/schematic.json")
            .is_file()
    );

    let read = read_library_folder(&scratch.0).unwrap();
    let again = read.image();
    for (file, bytes) in &written {
        let encoded = again.encode(file, 1).unwrap().unwrap();
        assert_eq!(&encoded.bytes, bytes, "{file} reads back to other bytes");
    }
    for (file, revision) in read.files() {
        let (revision, stamp) = revision.unwrap();
        assert_eq!(revision.revision, 1);
        assert_eq!(revision.digest, digest_file_bytes(&written[&file]));
        assert_eq!(stamp.unwrap().len, written[&file].len() as u64);
    }
    // The same files given as bytes, as a browser's folder upload gives them.
    let uploaded = written
        .iter()
        .map(|(file, bytes)| (file.to_string(), bytes.clone()))
        .collect();
    let uploaded = read_library_files(uploaded).unwrap().image();
    assert_eq!(uploaded.files(), again.files());
}

#[test]
fn a_library_moved_to_its_folder_and_embedded_again_holds_the_same_parts() {
    let scratch = Scratch::new("embed");
    let (mut workspace, mut libraries) = workspace_with_library();
    let id = uuid::Uuid::from_u128(7);
    let image = workspace.library_image(&libraries, LIBRARY, id).unwrap();
    let written = write_image(&scratch.0, &image);

    workspace.remove_library_parts(&libraries, LIBRARY);
    libraries.remove_library(LIBRARY);
    assert!(
        !workspace
            .schematic_buffers
            .keys()
            .any(|key| key.starts_with("shared_parts/"))
    );

    let read = read_library_folder(&scratch.0).unwrap().image();
    let left_out = workspace
        .insert_library_image(&mut libraries, &read, false, &|_| true)
        .unwrap();
    assert!(left_out.is_empty());
    let library = libraries.get_library(LIBRARY).unwrap();
    assert_eq!(library.technology, "gpdk045");
    assert_eq!(
        library.cells["folded_cascode"].description,
        "Two-stage amplifier"
    );
    let rebuilt = workspace.library_image(&libraries, LIBRARY, id).unwrap();
    for (file, bytes) in &written {
        assert_eq!(&rebuilt.encode(file, 1).unwrap().unwrap().bytes, bytes);
    }
}

#[test]
fn a_file_that_cannot_be_read_is_named_and_the_rest_still_read() {
    let scratch = Scratch::new("damaged");
    let (workspace, libraries) = workspace_with_library();
    let image = workspace
        .library_image(&libraries, LIBRARY, uuid::Uuid::nil())
        .unwrap();
    write_image(&scratch.0, &image);
    let damaged = amplifier_schematic();
    std::fs::write(scratch.0.join(damaged.relative_path()), b"{ not json").unwrap();

    let read = read_library_folder(&scratch.0).unwrap();
    assert!(matches!(
        read.cellviews[&damaged],
        Err(FileProblem::Unreadable(_))
    ));
    let symbol = LibraryFile::cell_view("folded_cascode", &View::new("symbol", ViewType::Symbol));
    assert!(read.cellviews[&symbol].is_ok());
    assert!(read.cells["folded_cascode"].is_ok());
    assert!(!read.image().cellviews.contains_key(&damaged));

    assert_eq!(
        read_library_folder(&scratch.0.join("nowhere")).unwrap_err(),
        FolderProblem::NoFolder
    );
    std::fs::remove_file(scratch.0.join(LIBRARY_MANIFEST_FILE)).unwrap();
    assert!(matches!(
        read_library_folder(&scratch.0),
        Err(FolderProblem::Unreadable(reason)) if reason.contains("not a library")
    ));
}

#[test]
fn writing_one_file_leaves_every_other_file_as_it_was() {
    let scratch = Scratch::new("atomic");
    let (workspace, libraries) = workspace_with_library();
    let image = workspace
        .library_image(&libraries, LIBRARY, uuid::Uuid::nil())
        .unwrap();
    let written = write_image(&scratch.0, &image);
    let schematic = amplifier_schematic();
    let manifest_before = std::fs::read(scratch.0.join(LIBRARY_MANIFEST_FILE)).unwrap();

    let next = image.encode(&schematic, 2).unwrap().unwrap();
    write_library_file(&scratch.0, &schematic, &next.bytes).unwrap();

    assert_eq!(
        std::fs::read(scratch.0.join(schematic.relative_path())).unwrap(),
        next.bytes
    );
    assert_eq!(
        std::fs::read(scratch.0.join(LIBRARY_MANIFEST_FILE)).unwrap(),
        manifest_before
    );
    for (file, bytes) in written.iter().filter(|(file, _)| **file != schematic) {
        assert_eq!(
            &std::fs::read(scratch.0.join(file.relative_path())).unwrap(),
            bytes
        );
    }
    let staging = std::fs::read_dir(scratch.0.join("folded_cascode/schematic"))
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".tmp"))
        .count();
    assert_eq!(staging, 0, "no staging file stays behind");
    assert_eq!(
        read_library_folder(&scratch.0).unwrap().files().len(),
        written.len()
    );
}

#[test]
fn two_projects_in_one_process_cannot_both_hold_one_cellview() {
    let scratch = Scratch::new("locks");
    let library = scratch.0.join("shared");
    let first_project = scratch.0.join("first").join("design.rspiceproj");
    let second_project = scratch.0.join("second").join("design.rspiceproj");
    let process = std::process::id();
    let first = lock("jsmith", process, &first_project.to_string_lossy());
    let second = lock("jsmith", process, &second_project.to_string_lossy());
    let schematic = amplifier_schematic();
    let alive = |_: u32| Some(true);

    assert_eq!(
        take_library_lock(&library, &schematic, &first, &alive).unwrap(),
        LockOutcome::Taken
    );
    assert_eq!(
        take_library_lock(&library, &schematic, &first, &alive).unwrap(),
        LockOutcome::AlreadyHeld
    );
    let LockOutcome::HeldBy(holder) =
        take_library_lock(&library, &schematic, &second, &alive).unwrap()
    else {
        panic!("the second project took a held cellview");
    };
    assert_eq!(holder, first);
    assert_eq!(holder.holder(), "jsmith on LAB-PC3");

    // Another cellview of the same library is free to the second project.
    let symbol = LibraryFile::cell_view("folded_cascode", &View::new("symbol", ViewType::Symbol));
    assert_eq!(
        take_library_lock(&library, &symbol, &second, &alive).unwrap(),
        LockOutcome::Taken
    );

    // Releasing someone else's lock leaves it in place.
    release_library_lock(&library, &schematic, &second).unwrap();
    assert_eq!(read_library_lock(&library, &schematic), Some(first.clone()));
    release_library_lock(&library, &schematic, &first).unwrap();
    assert_eq!(read_library_lock(&library, &schematic), None);
    assert_eq!(
        take_library_lock(&library, &schematic, &second, &alive).unwrap(),
        LockOutcome::Taken
    );
}

#[test]
fn a_lock_left_by_a_process_gone_from_this_computer_is_taken_over() {
    let scratch = Scratch::new("takeover");
    let schematic = amplifier_schematic();
    let gone = lock(
        "jsmith",
        std::process::id().wrapping_add(1),
        "old.rspiceproj",
    );
    let mine = lock("jsmith", std::process::id(), "design.rspiceproj");
    take_library_lock(&scratch.0, &schematic, &gone, &|_| Some(true)).unwrap();

    // While the process may still be running, the lock stands.
    assert_eq!(
        take_library_lock(&scratch.0, &schematic, &mine, &|_| None).unwrap(),
        LockOutcome::HeldBy(gone.clone())
    );
    assert_eq!(
        take_library_lock(&scratch.0, &schematic, &mine, &|_| Some(false)).unwrap(),
        LockOutcome::TakenOver(gone)
    );
    assert_eq!(
        read_library_lock(&scratch.0, &schematic),
        Some(mine.clone())
    );

    // A lock from another computer is never judged by this one's processes.
    let elsewhere = LibraryFileLock {
        computer: "LAB-PC9".to_owned(),
        ..lock("akumar", 12, "theirs.rspiceproj")
    };
    let symbol = LibraryFile::cell_view("folded_cascode", &View::new("symbol", ViewType::Symbol));
    take_library_lock(&scratch.0, &symbol, &elsewhere, &|_| Some(true)).unwrap();
    assert_eq!(
        take_library_lock(&scratch.0, &symbol, &mine, &|_| Some(false)).unwrap(),
        LockOutcome::HeldBy(elsewhere.clone())
    );
    assert_eq!(
        break_library_lock(&scratch.0, &symbol).unwrap(),
        Some(elsewhere)
    );
    assert_eq!(
        take_library_lock(&scratch.0, &symbol, &mine, &|_| Some(false)).unwrap(),
        LockOutcome::Taken
    );
    // Locks are never read as library files.
    let names = read_library_files(vec![
        (
            LIBRARY_MANIFEST_FILE.to_owned(),
            encode_library_file(
                &rspice_design::library::folder::LibraryManifest::of_library(
                    &Library::new(LIBRARY),
                    uuid::Uuid::nil(),
                    1,
                ),
            )
            .unwrap()
            .bytes,
        ),
        (symbol.lock_path(), b"{}".to_vec()),
    ])
    .unwrap();
    assert!(names.cellviews.is_empty());
}

struct NoSourceFiles;
impl rspice_design::hierarchy::HierarchySourceFiles for NoSourceFiles {
    fn source_paths_match(&self, _: &Path, _: &Path) -> bool {
        false
    }
    fn configured_source_identity(&self, path: &Path) -> String {
        path.display().to_string()
    }
    fn validate_source_file(
        &self,
        _: &Path,
        _: ViewType,
        _: &rspice_design::schematic::component::LibraryCellInstance,
    ) -> Result<(), String> {
        Ok(())
    }
}

#[test]
fn a_project_file_holds_only_the_reference_to_a_library_in_a_folder() {
    let (workspace, mut libraries) = workspace_with_library();
    libraries.insert_reference(LibraryReference::new(
        LIBRARY,
        LibraryPath::new("../shared_parts").unwrap(),
        uuid::Uuid::from_u128(7),
    ));
    let project = ProjectFile::new(workspace, libraries);
    let text = serialize_project_file(&project).unwrap();
    assert!(text.contains("../shared_parts"));
    assert!(
        !text.contains("folded_cascode"),
        "the folder's cells stay in the folder"
    );

    let decoded = decode_project_text(&text, None, NoSourceFiles)
        .unwrap()
        .file;
    assert!(decoded.libraries.get_library(LIBRARY).is_none());
    assert_eq!(
        decoded.libraries.reference(LIBRARY).unwrap().path.as_str(),
        "../shared_parts"
    );
    assert_eq!(serialize_project_file(&decoded).unwrap(), text);
}
