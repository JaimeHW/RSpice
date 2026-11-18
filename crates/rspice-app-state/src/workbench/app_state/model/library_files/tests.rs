//! Libraries kept in folders of their own, through the commands a page
//! calls: two projects in one process share one folder.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::*;
use crate::state::pdk_config::PdkConfig;
use crate::state::{Cell, CellViewRef, View, ViewType};
use crate::workbench::app_state::design_history::HistoryEffects;
use rspice_project::ProjectFile;

const LIBRARY: &str = "shared_parts";

/// A folder of its own under the system's temporary folder, removed when
/// the test ends.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "rspice-library-files-{name}-{}-{}",
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

/// An open project whose file is `project` (not yet written).
fn project(project: &Path) -> Model {
    let mut model = Model::session_start(
        PdkConfig::default(),
        None,
        crate::workbench::workspace::restore_model_library_manager,
    );
    model.project_open.set_from_lifecycle(true);
    model.workspace.content.project.path = Some(project.to_path_buf());
    model
}

/// `model` with the embedded library `shared_parts`: one amplifier with a
/// schematic and a symbol.
fn with_library(model: &mut Model) {
    model.create_library(LIBRARY).unwrap();
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    cell.add_view(View::new("symbol", ViewType::Symbol));
    model
        .library_manager
        .get_library_mut(LIBRARY)
        .unwrap()
        .add_cell(cell);
    model.workspace.content.schematic_buffers.insert(
        CellViewRef::new(LIBRARY, "amp", "schematic").key(),
        Default::default(),
    );
}

fn schematic_file() -> LibraryFile {
    LibraryFile::cell_view("amp", &View::new("schematic", ViewType::Schematic))
}

/// The project as its file holds it, but for the catalog's revision, which
/// only moves forward, through Undo too.
fn project_bytes(model: &Model) -> String {
    let file = working_file(model);
    let text = rspice_project::serialize_project_file(&file).unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&text).unwrap();
    value["libraries"]["revision"] = serde_json::Value::from(0);
    serde_json::to_string_pretty(&value).unwrap()
}

/// The open project as a file, without the simulation setup a session
/// starts with.
fn working_file(model: &Model) -> ProjectFile {
    let mut workspace = (*model.workspace.content).clone();
    workspace.simulation_plan_payloads.clear();
    ProjectFile::new(workspace, model.library_manager.clone())
}

/// Change what the amplifier's schematic view records, as an edit would.
fn annotate(model: &mut Model, note: &str) {
    model
        .library_manager
        .get_library_mut(LIBRARY)
        .unwrap()
        .cells
        .get_mut("amp")
        .unwrap()
        .views
        .get_mut("schematic")
        .unwrap()
        .metadata
        .insert("note".to_owned(), note.to_owned());
}

fn note(model: &Model) -> Option<String> {
    model.library_manager.get_library(LIBRARY)?.cells["amp"].views["schematic"]
        .metadata
        .get("note")
        .cloned()
}

fn step(model: &mut Model, forward: bool) {
    let staged = if forward {
        model.stage_project_redo()
    } else {
        model.stage_project_undo()
    }
    .unwrap()
    .expect("a step to take");
    let mut effects = HistoryEffects::default();
    let applied = model
        .apply_project_step(staged, None, &mut effects)
        .unwrap();
    model.finish_project_step(applied).unwrap();
}

/// The library `shared_parts` moved out of a first project into `folder`,
/// which then holds it Editable.
fn moved_out(scratch: &Scratch) -> (Model, PathBuf) {
    let folder = scratch.0.join("libraries").join(LIBRARY);
    let mut owner = project(&scratch.0.join("first").join("design.rspiceproj"));
    with_library(&mut owner);
    owner.move_library_to_folder(LIBRARY, &folder).unwrap();
    (owner, folder)
}

#[test]
fn moving_a_library_to_its_folder_writes_its_files_and_undo_puts_it_back_byte_for_byte() {
    let scratch = Scratch::new("move");
    let folder = scratch.0.join("libraries").join(LIBRARY);
    let mut model = project(&scratch.0.join("first").join("design.rspiceproj"));
    with_library(&mut model);
    let embedded = project_bytes(&model);

    model.move_library_to_folder(LIBRARY, &folder).unwrap();
    assert!(folder.join("library.rspicelib").is_file());
    assert!(folder.join("amp/schematic/schematic.json").is_file());
    let reference = model.library_manager.reference(LIBRARY).unwrap();
    assert_eq!(reference.path.as_str(), "../libraries/shared_parts");
    assert_eq!(reference.mode, LibraryMode::Editable);
    assert!(matches!(
        model.reference_status(LIBRARY),
        Some(ReferenceStatus::Found {
            mode: LibraryMode::Editable,
            moved_on: 0,
            ..
        })
    ));
    let moved = project_bytes(&model);
    assert!(!moved.contains("\"amp\""), "the cells are the folder's now");

    step(&mut model, false);
    assert_eq!(project_bytes(&model), embedded);
    assert_eq!(
        model.reference_status(LIBRARY),
        Some(ReferenceStatus::InProject)
    );
    step(&mut model, true);
    assert_eq!(project_bytes(&model), moved);
}

#[test]
fn attaching_a_library_reads_its_folder_and_undo_gives_back_the_project_byte_for_byte() {
    let scratch = Scratch::new("attach");
    let (_owner, folder) = moved_out(&scratch);
    let mut model = project(&scratch.0.join("second").join("design.rspiceproj"));
    let before = project_bytes(&model);

    let choice = model.library_path_choice(&folder).unwrap();
    assert_eq!(choice.stored.as_str(), "../libraries/shared_parts");
    assert_eq!(
        model.attach_library(&folder, choice.stored).unwrap(),
        LIBRARY
    );
    assert!(
        model
            .library_manager
            .get_library(LIBRARY)
            .unwrap()
            .read_only
    );
    assert!(
        model
            .workspace
            .content
            .schematic_buffers
            .contains_key(&CellViewRef::new(LIBRARY, "amp", "schematic").key())
    );
    assert_eq!(
        model.library_file_status(LIBRARY, &schematic_file()),
        Some(CellViewStatus::Found {
            revision: 1,
            saved_with: None
        })
    );
    let attached = project_bytes(&model);
    assert_ne!(attached, before);
    assert!(
        model
            .attach_library(
                &folder,
                LibraryPath::new("../libraries/shared_parts").unwrap()
            )
            .unwrap_err()
            .contains("already attached")
    );

    step(&mut model, false);
    assert_eq!(project_bytes(&model), before);
    assert_eq!(model.reference_status(LIBRARY), None);
    step(&mut model, true);
    assert_eq!(project_bytes(&model), attached);

    assert_eq!(model.detach_library(LIBRARY).unwrap(), Vec::<String>::new());
    assert_eq!(project_bytes(&model), before);
}

#[test]
fn two_projects_in_one_process_cannot_edit_one_cellview_at_once() {
    let scratch = Scratch::new("locks");
    let (mut first, folder) = moved_out(&scratch);
    let mut second = project(&scratch.0.join("second").join("design.rspiceproj"));
    let path = second.library_path_choice(&folder).unwrap().stored;
    second.attach_library(&folder, path).unwrap();
    assert!(
        second
            .claim_library_file(LIBRARY, &schematic_file())
            .unwrap_err()
            .contains("read-only")
    );
    second
        .set_library_mode(LIBRARY, LibraryMode::Editable)
        .unwrap();

    first
        .claim_library_file(LIBRARY, &schematic_file())
        .unwrap();
    first
        .claim_library_file(LIBRARY, &schematic_file())
        .unwrap();
    let refusal = second
        .claim_library_file(LIBRARY, &schematic_file())
        .unwrap_err();
    assert!(refusal.starts_with("Being edited by "), "{refusal}");
    // Another cellview of the same library is free.
    let symbol = LibraryFile::cell_view("amp", &View::new("symbol", ViewType::Symbol));
    second.claim_library_file(LIBRARY, &symbol).unwrap();

    first.release_library_locks();
    second
        .claim_library_file(LIBRARY, &schematic_file())
        .unwrap();
    assert!(
        second
            .held_library_lock(LIBRARY, &schematic_file())
            .is_some()
    );
    second
        .break_library_lock(LIBRARY, &schematic_file())
        .unwrap();
    assert!(
        second
            .held_library_lock(LIBRARY, &schematic_file())
            .is_none()
    );
}

#[test]
fn a_file_changed_on_disk_is_named_never_overwritten_and_refreshed_on_request() {
    let scratch = Scratch::new("changed");
    let (mut first, folder) = moved_out(&scratch);
    let mut second = project(&scratch.0.join("second").join("design.rspiceproj"));
    let path = second.library_path_choice(&folder).unwrap().stored;
    second.attach_library(&folder, path).unwrap();
    second
        .set_library_mode(LIBRARY, LibraryMode::Editable)
        .unwrap();

    // The first project saves an edit: the file moves to revision 2.
    annotate(&mut first, "first");
    let mut file = working_file(&first);
    first.write_library_files(&mut file).unwrap();
    assert_eq!(
        file.libraries.reference(LIBRARY).unwrap().files[&schematic_file()].revision,
        2
    );
    assert_eq!(
        first.library_file_status(LIBRARY, &schematic_file()),
        Some(CellViewStatus::Found {
            revision: 2,
            saved_with: None
        })
    );

    // The second project edited the same file meanwhile.
    annotate(&mut second, "second");
    assert_eq!(second.check_library_files(), 1);
    assert_eq!(
        second.library_file_status(LIBRARY, &schematic_file()),
        Some(CellViewStatus::ChangedSinceEditing {
            loaded: 1,
            on_disk: 2
        })
    );
    assert!(matches!(
        second.reference_status(LIBRARY),
        Some(ReferenceStatus::ChangedSinceEditing { cellviews: 1 })
    ));
    let mut file = working_file(&second);
    let refusal = second.write_library_files(&mut file).unwrap_err();
    assert!(
        refusal.contains("Changed on disk since you started editing"),
        "{refusal}"
    );
    assert!(
        second
            .refresh_library_files(&[(LIBRARY.to_owned(), schematic_file())])
            .is_err()
    );
    let on_disk = std::fs::read(folder.join(schematic_file().relative_path())).unwrap();

    second
        .keep_my_library_file(LIBRARY, &schematic_file())
        .unwrap();
    second.write_library_files(&mut file).unwrap();
    assert_ne!(
        std::fs::read(folder.join(schematic_file().relative_path())).unwrap(),
        on_disk
    );
    assert_eq!(
        second.library_file_status(LIBRARY, &schematic_file()),
        Some(CellViewStatus::Found {
            revision: 3,
            saved_with: None
        })
    );

    // The first project, unedited since, takes the file on request.
    assert_eq!(first.check_library_files(), 1);
    assert_eq!(first.refresh_rows().len(), 1);
    let (reloaded, _) = first
        .refresh_library_files(&[(LIBRARY.to_owned(), schematic_file())])
        .unwrap();
    assert_eq!(reloaded, 1);
    assert_eq!(note(&first).as_deref(), Some("second"));
    assert_eq!(first.check_library_files(), 0);
}

#[test]
fn a_file_that_cannot_be_written_is_named_and_the_rest_are_written() {
    let scratch = Scratch::new("unwritable");
    let (mut owner, folder) = moved_out(&scratch);
    annotate(&mut owner, "edited");
    let target = folder.join(schematic_file().relative_path());
    std::fs::remove_file(&target).unwrap();
    std::fs::create_dir_all(target.join("in-the-way")).unwrap();
    let mut file = working_file(&owner);
    let refusal = owner.write_library_files(&mut file).unwrap_err();
    assert!(
        refusal.contains("shared_parts/amp/schematic/schematic.json"),
        "{refusal}"
    );
    assert!(refusal.starts_with("The project file was left as it was"));
}

#[test]
fn a_project_whose_library_folder_is_gone_opens_with_the_library_not_found() {
    let scratch = Scratch::new("missing");
    let (owner, folder) = moved_out(&scratch);
    let mut file = working_file(&owner).without_referenced_libraries();
    std::fs::remove_dir_all(&folder).unwrap();
    let reader = project(&scratch.0.join("first").join("design.rspiceproj"));
    let session = reader.read_referenced_libraries(&mut file);
    assert!(file.libraries.get_library(LIBRARY).is_none());
    assert!(matches!(
        session.library(LIBRARY).map(|session| &session.load),
        Some(ReferenceLoad::NotFound(Missing::NoFolder(_)))
    ));

    // Put back, the folder reads again, every file at the revision saved.
    let (_again, _) = moved_out(&scratch);
    let mut file = working_file(&owner).without_referenced_libraries();
    let session = reader.read_referenced_libraries(&mut file);
    assert!(
        matches!(
            session.library(LIBRARY).map(|session| &session.load),
            Some(ReferenceLoad::DifferentLibrary(_))
        ),
        "a folder written again holds a library of another identity"
    );
}
