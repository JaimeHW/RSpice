//! What an edit costs the frames after it, counted where the work is done:
//! whether the project holds unsaved work is compared once per edit, over
//! the documents the edit moved; a frame that only paints compares nothing
//! and writes nothing back from the editor; and a canvas gesture holds the
//! answer, the Design navigator its hierarchy and the project its drawing,
//! until the drop. None of this shows in what a frame paints, which is why
//! it is counted.

#![cfg(not(target_arch = "wasm32"))]

use std::time::Duration;

use egui::{Event, Modifiers, PointerButton, Pos2};

use crate::feature_tests::replay::Replay;
use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_project::workbench::surfaces::project::overview::CELL_ROW_BUILDS;
use rspice_app_services::workbench::lifecycle::project_lifecycle::{
    DIRTY_AUDITS, DIRTY_RECOMPUTES, active_document_is_dirty, has_unsaved_changes,
};
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::{
    Cell, ComponentType, ConfigurationBlackBoxPolicy, ConfigurationModelProfile,
    ConfigurationPlatform, ConfigurationSetDefinition, ConfigurationSetOverride, InstancePath,
    Library, Point, UnresolvedBindingPolicy, View, ViewType,
};
use rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticInputs;
use rspice_app_state::workbench::app_state::model::DRAWING_WRITES;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_design::projection::{
    document_digest_counts, materialization_count, reset_document_digest_counts,
    reset_materialization_count,
};
use rspice_project::registry::{DOCUMENT_DIGESTS, DRAWING_DIGESTS, PROJECT_CAPTURES};

/// Frames a surface is given to lay itself out before counting starts.
const WARM_UP_FRAMES: usize = 3;
/// Frames counted after an edit, and while nothing happens.
const FRAMES: usize = 6;
/// Longer than a drawing waits to stand still before its live check runs,
/// so the check's own work falls inside what an edit is counted for.
const LIVE_CHECK_SETTLED: Duration = Duration::from_millis(400);

/// The work the frames did since the last [`take`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct EditWork {
    /// Comparisons of the working project with the saved one.
    comparisons: usize,
    /// Project documents digested from their content.
    documents_digested: usize,
    /// Drawings digested from their content, rather than read from where
    /// they stand in their history.
    drawings_digested: usize,
    /// Drawings digested to key the design projection, rather than keyed by
    /// where they stand in their history.
    projection_sources_digested: u64,
    /// Whole working-project captures.
    captures: usize,
    /// Copies of the editor's drawing written back to the project.
    drawing_writes: usize,
}

fn take() -> EditWork {
    EditWork {
        comparisons: DIRTY_RECOMPUTES.with(|count| count.replace(0)),
        documents_digested: DOCUMENT_DIGESTS.with(|count| count.replace(0)),
        drawings_digested: DRAWING_DIGESTS.with(|count| count.replace(0)),
        projection_sources_digested: {
            let (sources, _) = document_digest_counts();
            reset_document_digest_counts();
            sources
        },
        captures: PROJECT_CAPTURES.with(|count| count.replace(0)),
        drawing_writes: DRAWING_WRITES.with(|count| count.replace(0)),
    }
}

/// A saved, clean project in a folder of its own: three resistors on the
/// root schematic, and a library of three cells beside it.
fn saved_project(label: &str) -> (Replay, std::path::PathBuf) {
    let folder = unique_temp_dir(label);
    let mut app = RSpiceApp::test_instance();
    app.symbol_library = rspice_app_design::schematic::symbols::SymbolLibrary::load_embedded().ok();
    app.state.model.schematic.with_undo("Fill", |schematic| {
        for index in 0..3 {
            schematic.add_component(ComponentType::Resistor, Point::new(200 * index, 0));
        }
    });
    let mut cells = Library::new("cells");
    for index in 0..3 {
        let mut cell = Cell::new(format!("cell{index}"));
        cell.add_view(View::new("schematic", ViewType::Schematic));
        cells.add_cell(cell);
    }
    app.state.model.library_manager.add_library(cells);
    assert!(
        app.state
            .save_project_to_path(&folder.join("edits.rspiceproj")),
        "the project saves"
    );
    (Replay::of(app), folder)
}

fn idle(replay: &mut Replay, frames: usize) {
    for _ in 0..frames {
        replay.frame(Vec::new());
    }
}

/// The frames after `edit`, its live check among them, and what they did.
fn work_of(replay: &mut Replay, edit: impl FnOnce(&mut RSpiceApp)) -> EditWork {
    let _ = take();
    edit(&mut replay.app);
    idle(replay, FRAMES);
    std::thread::sleep(LIVE_CHECK_SETTLED);
    idle(replay, FRAMES);
    take()
}

fn place(app: &mut RSpiceApp) {
    app.state.model.schematic.with_undo("Place", |schematic| {
        schematic.add_component(ComponentType::Capacitor, Point::new(0, 400));
    });
}

/// A step of an edit history: its name, what it does, and whether the
/// project reads unsaved after it.
type HistoryStep<'a> = (&'a str, &'a dyn Fn(&mut RSpiceApp), bool);

/// Idle frames compare nothing and write nothing back, on the canvas and on
/// the Project pages; a part placed on the drawing is compared once, from
/// that one drawing, read unsaved from where it stands in its history with
/// no digest of its content, and written back once. The design projection
/// is keyed by that same position, so it digests no drawing either.
#[test]
fn an_edit_is_compared_once_from_its_own_document_and_idle_frames_do_nothing() {
    let (mut replay, folder) = saved_project("edit-work-idle");
    for (surface, bring_up) in [
        ("the canvas", None),
        ("the Overview", Some(ProjectPage::Overview)),
        ("the Library page", Some(ProjectPage::Library)),
    ] {
        match bring_up {
            None => replay.app.state.workbench.activate(Workspace::Design),
            Some(page) => {
                replay.app.state.workbench.activate(Workspace::Project);
                replay.app.state.project.project_page = page;
            }
        }
        idle(&mut replay, WARM_UP_FRAMES);
        let _ = take();
        idle(&mut replay, FRAMES);
        assert_eq!(
            take(),
            EditWork::default(),
            "{surface}: idle frames compare, digest, capture and write back nothing"
        );

        assert_eq!(
            work_of(&mut replay, place),
            EditWork {
                comparisons: 1,
                documents_digested: 1,
                drawings_digested: 0,
                projection_sources_digested: 0,
                captures: 0,
                drawing_writes: 1,
            },
            "{surface}: a placed part is compared once, from its drawing alone, which is not \
             digested, and written back once"
        );
        assert!(
            replay.app.last_window_title.contains('*'),
            "{surface}: the title marks the edit: {}",
            replay.app.last_window_title
        );
    }
    drop(replay);
    let _ = std::fs::remove_dir_all(folder);
}

/// Whether the drawing is unsaved is read from where it stands in its
/// history: an edit, its undo back to where the save left it and its redo
/// are each read without digesting the drawing, and the undo reads clean in
/// the title, the toolbar and the drawing's tab. Saved where a redo leaves
/// it, an undo past the save reads unsaved and the redo back clean.
#[test]
fn an_edit_and_its_undo_are_read_from_the_history_without_a_digest() {
    let (mut replay, folder) = saved_project("edit-work-undo");
    replay.app.state.workbench.activate(Workspace::Design);
    idle(&mut replay, WARM_UP_FRAMES);
    let undo = |app: &mut RSpiceApp| assert!(app.state.model.schematic.undo(), "undone");
    let redo = |app: &mut RSpiceApp| assert!(app.state.model.schematic.redo(), "redone");
    let read = |replay: &Replay, step: &str, unsaved: bool| {
        let app = &replay.app;
        assert_eq!(
            (
                has_unsaved_changes(&app.state),
                active_document_is_dirty(&app.state),
                app.state.model.workspace.content.active_dirty(),
                app.last_window_title.contains('*'),
            ),
            (unsaved, unsaved, Some(unsaved), unsaved),
            "{step}: the toolbar, the drawing's tab and the title read {}: {}",
            if unsaved { "unsaved" } else { "clean" },
            app.last_window_title
        );
    };
    let steps: [HistoryStep<'_>; 4] = [
        ("place", &place, true),
        ("undo", &undo, false),
        ("redo", &redo, true),
        ("undo", &undo, false),
    ];
    for (step, edit, unsaved) in steps {
        let work = work_of(&mut replay, edit);
        assert_eq!(
            work.drawings_digested, 0,
            "{step}: the drawing is read from its history, not digested: {work:?}"
        );
        read(&replay, step, unsaved);
    }

    work_of(&mut replay, redo);
    assert!(
        replay
            .app
            .state
            .save_project_to_path(&folder.join("edits.rspiceproj")),
        "the project saves"
    );
    idle(&mut replay, WARM_UP_FRAMES);
    read(&replay, "the save", false);
    let steps: [HistoryStep<'_>; 2] = [
        ("undo past the save", &undo, true),
        ("redo back to it", &redo, false),
    ];
    for (step, edit, unsaved) in steps {
        let work = work_of(&mut replay, edit);
        assert_eq!(
            work.drawings_digested, 0,
            "{step}: the drawing is read from its history, not digested: {work:?}"
        );
        read(&replay, step, unsaved);
    }
    drop(replay);
    let _ = std::fs::remove_dir_all(folder);
}

/// A project closed while an edit waits for its live check lands without
/// checking it: the first frame of the landing projects no design.
#[test]
fn a_project_closed_mid_edit_lands_without_checking_the_edit() {
    let (mut replay, folder) = saved_project("edit-work-close");
    replay.app.state.workbench.activate(Workspace::Design);
    idle(&mut replay, WARM_UP_FRAMES);
    place(&mut replay.app);
    replay.frame(Vec::new());
    replay.app.state.close_project_session();
    reset_materialization_count();
    replay.frame(Vec::new());
    assert_eq!(
        materialization_count(),
        0,
        "the landing's first frame projects no design for the edit left behind"
    );
    drop(replay);
    let _ = std::fs::remove_dir_all(folder);
}

/// The Overview's Cells table counts each schematic's parts: a placed part
/// derives it once, on the frame of the edit, and not again when the editor
/// writes the drawing back a frame later.
#[test]
fn a_placed_part_derives_the_overview_cells_once() {
    let (mut replay, folder) = saved_project("edit-work-overview-cells");
    replay.app.state.workbench.activate(Workspace::Project);
    replay.app.state.project.project_page = ProjectPage::Overview;
    idle(&mut replay, WARM_UP_FRAMES);
    // The first edit after the save also marks the drawing's tab unsaved,
    // which the table shows; the second is the one counted.
    let _ = work_of(&mut replay, place);
    CELL_ROW_BUILDS.with(|count| count.set(0));
    let _ = work_of(&mut replay, |app| {
        app.state.model.schematic.with_undo("Place", |schematic| {
            schematic.add_component(ComponentType::Capacitor, Point::new(200, 400));
        });
    });
    assert_eq!(
        CELL_ROW_BUILDS.with(std::cell::Cell::get),
        1,
        "the Cells table is derived once for the placed part"
    );
    drop(replay);
    let _ = std::fs::remove_dir_all(folder);
}

/// A library change from the Library page is compared once: the frames
/// after it do not compare it again.
#[test]
fn a_library_change_is_compared_once() {
    type Change = fn(&mut RSpiceApp) -> Result<usize, String>;
    let changes: [(&str, Change); 3] = [
        ("rename a cell", |app| {
            app.state.rename_cell("cells", "cell1", "renamed")
        }),
        ("rename a library", |app| {
            app.state.rename_library("cells", "parts")
        }),
        ("delete a library", |app| app.state.delete_library("cells")),
    ];
    for (name, change) in changes {
        let (mut replay, folder) = saved_project("edit-work-library");
        replay.app.state.workbench.activate(Workspace::Project);
        replay.app.state.project.project_page = ProjectPage::Library;
        idle(&mut replay, WARM_UP_FRAMES);
        let work = work_of(&mut replay, |app| {
            change(app).unwrap_or_else(|error| panic!("{name}: {error}"));
        });
        assert_eq!(
            work.comparisons, 1,
            "{name}: the change is compared once: {work:?}"
        );
        drop(replay);
        let _ = std::fs::remove_dir_all(folder);
    }
}

fn button(pos: Pos2, pressed: bool) -> Event {
    Event::PointerButton {
        pos,
        button: PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    }
}

/// What one frame of a drag did.
#[derive(Debug)]
struct DragFrame {
    name: String,
    work: EditWork,
    /// Cell views projected whole.
    projections: u64,
    /// Whether the project reads as holding unsaved work after the frame,
    /// as the window title and the toolbar read it.
    unsaved: bool,
}

/// A part dragged across the canvas: no frame of the drag compares the
/// project, projects the design again or copies the drawing into the
/// project until the drop, the project reads as edited from the first move,
/// and the drop compares it once and writes the drawing back once.
#[test]
fn a_drag_is_compared_and_projected_at_the_drop_not_on_each_move() {
    let (replay, folder) = saved_project("edit-work-drag");
    drag_a_part(replay, folder, true);
}

/// A drag over a configured design that does not resolve is held the same
/// way: the reason it does not resolve stands through the drag, and the
/// design is projected again at the drop.
#[test]
fn a_design_that_does_not_resolve_is_projected_at_the_drop_not_on_each_move() {
    let (mut replay, folder) = saved_project("edit-work-drag-unresolved");
    let content = &mut replay.app.state.model.workspace.content;
    let mut configurations = (*content.configuration_sets).clone();
    configurations
        .create(ConfigurationSetDefinition {
            name: "Unresolved".to_owned(),
            root: content.active_view.clone(),
            dut_path: InstancePath::root().to_string(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: vec![ConfigurationSetOverride {
                instance_path: "/XMISSING".to_owned(),
                executable_views: vec!["schematic".to_owned()],
                stop_view: None,
                model_section: None,
                eligible_platforms: vec![ConfigurationPlatform::Desktop],
                target: None,
            }],
            cell_bindings: Vec::new(),
            model_profile: ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Local project".to_owned(),
        })
        .expect("the configuration is well formed");
    content
        .replace_configuration_sets(configurations)
        .expect("the configuration publishes");
    drag_a_part(replay, folder, false);
}

/// Drags the middle part of the saved drawing four grid steps across the
/// canvas in whole application frames, and checks what each frame did.
/// `resolves` says whether the configured design resolves, and so whether a
/// live check stands on the drawing through the drag.
fn drag_a_part(mut replay: Replay, folder: std::path::PathBuf, resolves: bool) {
    replay.app.state.workbench.activate(Workspace::Design);
    let editor = &mut replay.app.state.model.schematic.session.editor;
    (editor.needs_fit, editor.needs_drawing_sheet_fit) = (false, false);
    editor.zoom = 1.0;
    // A live check stands on the drawing, as one does once any edit has
    // settled, and whether its findings are current is asked every frame.
    idle(&mut replay, WARM_UP_FRAMES);
    place(&mut replay.app);
    idle(&mut replay, WARM_UP_FRAMES);
    std::thread::sleep(LIVE_CHECK_SETTLED);
    idle(&mut replay, WARM_UP_FRAMES);
    let model = &replay.app.state.model;
    let subject = model.workspace.content.active_view.clone();
    assert_eq!(
        model.design_projection_through_gesture().is_ok(),
        resolves,
        "whether the configured design resolves"
    );
    if resolves {
        assert!(
            model
                .diagnostics
                .batch(&DiagnosticInputs::LiveCheck { subject })
                .is_some(),
            "the drawing's live check has run"
        );
    }
    assert!(
        replay
            .app
            .state
            .save_project_to_path(&folder.join("edits.rspiceproj")),
        "the project saves"
    );
    idle(&mut replay, WARM_UP_FRAMES);
    let canvas = replay
        .ctx()
        .read_response(egui::Id::new("rspice-schematic-canvas-interaction"))
        .expect("the canvas was drawn")
        .rect;
    let document = replay.app.state.model.schematic.document();
    let (part, at, grid) = (
        document.components[1].id,
        document.components[1].pos,
        document.grid_size,
    );
    replay.app.state.model.schematic.session.editor.pan = (
        f64::from(canvas.width()) * 0.5 - f64::from(at.x),
        f64::from(canvas.height()) * 0.5 - f64::from(at.y),
    );
    let mut pointer = canvas.center();
    let mut script = vec![
        ("hover".to_owned(), vec![Event::PointerMoved(pointer)]),
        ("hover".to_owned(), vec![Event::PointerMoved(pointer)]),
        ("press".to_owned(), vec![button(pointer, true)]),
    ];
    for index in 1..=4 {
        pointer.x += grid as f32;
        script.push((format!("move {index}"), vec![Event::PointerMoved(pointer)]));
    }
    script.push(("drop".to_owned(), vec![button(pointer, false)]));
    for index in 1..=3 {
        script.push((format!("after {index}"), Vec::new()));
    }
    let frames = script
        .into_iter()
        .map(|(name, events)| {
            let _ = take();
            reset_materialization_count();
            replay.frame(events);
            let projections = materialization_count();
            let unsaved = has_unsaved_changes(&replay.app.state);
            DragFrame {
                name,
                work: take(),
                projections,
                unsaved,
            }
        })
        .collect::<Vec<_>>();

    let dropped = replay
        .app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == part)
        .map(|component| component.pos);
    assert_eq!(
        dropped,
        Some(Point::new(at.x + 4 * grid, at.y)),
        "the part follows the pointer: {frames:#?}"
    );
    let (gesture, settled) = frames.split_at(frames.len() - 4);
    for frame in gesture {
        assert_eq!(
            (frame.work.comparisons, frame.projections),
            (0, 0),
            "{}: no frame of the drag compares the project or projects the design: {frames:#?}",
            frame.name
        );
        assert_eq!(
            frame.work.drawing_writes, 0,
            "{}: no frame of the drag copies the drawing into the project: {frames:#?}",
            frame.name
        );
        assert_eq!(
            frame.unsaved,
            frame.name.starts_with("move"),
            "{}: the project reads as edited from the first move on: {frames:#?}",
            frame.name
        );
    }
    assert_eq!(
        settled
            .iter()
            .map(|frame| frame.work.comparisons)
            .sum::<usize>(),
        1,
        "the drop is compared once: {frames:#?}"
    );
    assert_eq!(
        settled
            .iter()
            .map(|frame| frame.work.drawing_writes)
            .sum::<usize>(),
        1,
        "the drop writes the drawing back once: {frames:#?}"
    );
    assert!(
        settled.iter().any(|frame| frame.projections > 0),
        "the drop projects the design again: {frames:#?}"
    );
    assert!(
        settled.iter().all(|frame| frame.unsaved),
        "the project still reads as edited after the drop: {frames:#?}"
    );
    assert!(
        replay.app.last_window_title.contains('*'),
        "the title marks the edit: {}",
        replay.app.last_window_title
    );
    drop(replay);
    let _ = std::fs::remove_dir_all(folder);
}

/// An idle frame on any Project page of a project of five thousand cells
/// derives nothing per cell: it compares, digests, captures and writes back
/// nothing, and the debug audit of the answer it reads captures the project
/// again ever more rarely: a page idle as long again as it was shown takes
/// that capture at most once.
#[test]
fn an_idle_frame_on_a_project_of_five_thousand_cells_derives_nothing_per_cell() {
    let (mut replay, folder) = saved_project("edit-work-idle-cells");
    let mut cells = Library::new("bulk");
    for index in 0..5_000 {
        let mut cell = Cell::new(format!("cell{index:04}"));
        cell.add_view(View::new("schematic", ViewType::Schematic));
        cells.add_cell(cell);
    }
    replay.app.state.model.library_manager.add_library(cells);
    replay.app.state.workbench.activate(Workspace::Project);
    for page in [
        ProjectPage::Overview,
        ProjectPage::Library,
        ProjectPage::Configuration,
        ProjectPage::Dependencies,
    ] {
        replay.app.state.project.project_page = page;
        idle(&mut replay, FRAMES);
        let _ = take();
        DIRTY_AUDITS.with(|count| count.set(0));
        idle(&mut replay, FRAMES);
        assert_eq!(
            take(),
            EditWork::default(),
            "{page:?}: idle frames compare, digest, capture and write back nothing"
        );
        assert!(
            DIRTY_AUDITS.with(std::cell::Cell::get) <= 1,
            "{page:?}: idle as long again as it was shown, the page captures the project \
             for the audit at most once"
        );
    }
    drop(replay);
    let _ = std::fs::remove_dir_all(folder);
}
