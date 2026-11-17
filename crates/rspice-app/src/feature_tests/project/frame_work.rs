//! Every Project page over a large project, thirty idle frames each: a frame
//! that only paints does no whole-project work. Nothing a frame paints shows
//! that work, so each kind is counted where it is done: fingerprint passes,
//! whole-project captures, hashes of model sources, project files read,
//! checkpoint directories listed and files stat'ed, whole-catalog sorts, net
//! extractions, library columns listed and drawings or bindings walked.

#![cfg(not(target_arch = "wasm32"))]

use std::path::{Path, PathBuf};

use egui::Context;

use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::configuration_state::CONFIGURATION_SCANS;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_project::workbench::surfaces::library_cellview_manager::CELLVIEW_PROJECTION_BUILDS;
use rspice_app_project::workbench::surfaces::project::library::LIBRARY_ROW_BUILDS;
use rspice_app_services::workbench::lifecycle::project_checkpoint::{
    ProjectCheckpointReason, create,
};
use rspice_app_services::workbench::lifecycle::project_lifecycle::dirty_document_count;
use rspice_app_services::workbench::lifecycle::recovery::{take_disk_reads, unique_temp_dir};
use rspice_app_state::product::ContentDigest;
use rspice_app_state::state::{
    Cell, CellViewRef, ComponentType, ConfigurationBlackBoxPolicy, ConfigurationModelProfile,
    ConfigurationPlatform, ConfigurationSetDefinition, ConfigurationSetOverride, InstancePath,
    Library, Point, ProjectTechnologyBinding, ProjectTechnologyChangeAuthority,
    ProjectTechnologyChangeContext, UnresolvedBindingPolicy, View, ViewType,
};
use rspice_app_state::workbench::app_state::cell_usage::CELL_USAGE_SCANS;
use rspice_app_state::workbench::workspace::Workspace;

use super::recovery_frames::{IDLE_FRAMES, app_frame, offscreen, paint_until, painted};

/// Frames painted before counting starts: a page lays out its first frame
/// and derives what it shows then.
const WARM_UP_FRAMES: usize = 3;
const COMPONENTS: usize = 8_000;
const CELLS: usize = 5_000;
const BINDINGS: usize = 200;
const CHECKPOINTS: usize = 8;

/// The whole-project work one thread did.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct FrameWork {
    /// Every document of the project fingerprinted.
    fingerprint_passes: usize,
    /// The working project copied out whole, to compare or to save.
    project_captures: usize,
    /// Model-library content hashed: retained sources proven against their
    /// pins, sources re-read for drift, libraries serialized for a digest.
    model_library_hashes: usize,
    /// Saved projects and checkpoint snapshots read.
    project_file_reads: usize,
    /// Checkpoint directories listed and files whose metadata was read.
    checkpoint_reads: usize,
    /// Every library of a catalog, or every cell of a library, sorted.
    catalog_sorts: usize,
    /// Nets extracted from a schematic.
    net_extractions: usize,
    /// Library browser columns listed and sorted, and the cell-view
    /// manager's catalog projected.
    library_rows: usize,
    /// Drawings walked for a cell's placements, and a configuration's
    /// bindings, instance paths or views walked.
    design_scans: usize,
}

impl FrameWork {
    const fn counts(self) -> [usize; 9] {
        [
            self.fingerprint_passes,
            self.project_captures,
            self.model_library_hashes,
            self.project_file_reads,
            self.checkpoint_reads,
            self.catalog_sorts,
            self.net_extractions,
            self.library_rows,
            self.design_scans,
        ]
    }

    const fn from_counts(counts: [usize; 9]) -> Self {
        let [
            fingerprint_passes,
            project_captures,
            model_library_hashes,
            project_file_reads,
            checkpoint_reads,
            catalog_sorts,
            net_extractions,
            library_rows,
            design_scans,
        ] = counts;
        Self {
            fingerprint_passes,
            project_captures,
            model_library_hashes,
            project_file_reads,
            checkpoint_reads,
            catalog_sorts,
            net_extractions,
            library_rows,
            design_scans,
        }
    }

    /// What the page itself did, leaving out the checks of the held
    /// unsaved-document answer.
    ///
    /// The toolbar, the window title and the Overview ask on every frame
    /// whether the project holds unsaved work, and read the answer held for
    /// the working state. A debug build checks every read: it captures the
    /// project again, outside the capture and fingerprint counts, and that
    /// capture proves the model libraries. A release build does not.
    /// `per_read` is what one checked read costs on this project; the frames'
    /// work must be a whole number of reads, and what is left is the page's.
    fn page_work(self, per_read: Self) -> Self {
        let (frames, each) = (self.counts(), per_read.counts());
        let reads = frames
            .iter()
            .zip(each)
            .find(|(_, each)| *each > 0)
            .map_or(0, |(total, each)| total / each);
        Self::from_counts(std::array::from_fn(|kind| {
            frames[kind]
                .checked_sub(reads * each[kind])
                .unwrap_or_else(|| {
                    panic!("{reads} checked reads of {per_read:?} cost more than the frames did: {self:?}")
                })
        }))
    }
}

/// The work counted on this thread since the last call, and start again.
fn take_work() -> FrameWork {
    let disk = take_disk_reads();
    let design = rspice_design_model::work_tally::take();
    FrameWork {
        fingerprint_passes: rspice_project::registry::FINGERPRINT_PASSES.with(|n| n.replace(0)),
        project_captures: rspice_project::registry::PROJECT_CAPTURES.with(|n| n.replace(0)),
        model_library_hashes: rspice_model_library::MODEL_SOURCE_HASHES.with(|n| n.replace(0))
            + rspice_model_library::CATALOG_LIBRARY_SERIALIZATIONS.with(|n| n.replace(0))
            + rspice_app_state::workbench::app_state::model::SOURCE_DRIFT_HASHES
                .with(|n| n.replace(0)),
        project_file_reads: disk.project_files,
        checkpoint_reads: disk.checkpoint_directories + disk.file_stats,
        catalog_sorts: rspice_model_library::CATALOG_SORTS.with(|n| n.replace(0))
            + design.catalog_sorts,
        net_extractions: design.connectivity_passes,
        library_rows: LIBRARY_ROW_BUILDS.with(|n| n.replace(0))
            + CELLVIEW_PROJECTION_BUILDS.with(|n| n.replace(0)),
        design_scans: CELL_USAGE_SCANS.with(|n| n.replace(0))
            + CONFIGURATION_SCANS.with(|n| n.replace(0)),
    }
}

/// A saved project of `components` resistors on its root schematic, a
/// library of 5,000 cells, a configuration binding the first 200 instances,
/// an attached technology and eight checkpoints, in a folder of its own.
fn large_project(label: &str, components: usize) -> (RSpiceApp, PathBuf) {
    let folder = unique_temp_dir(label);
    let mut app = RSpiceApp::test_instance();
    let content = &mut app.state.model.workspace.content;
    let root = CellViewRef::new(
        &content.project.root_library,
        &content.project.top_cell,
        rspice_app_state::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    );
    content.active_view = root.clone();
    app.state.model.schematic.with_undo("Fill", |schematic| {
        for index in 0..components {
            let index = i32::try_from(index).expect("a component index fits");
            schematic.add_component(
                ComponentType::Resistor,
                Point::new(40 * (index % 200), 40 * (index / 200)),
            );
        }
    });

    let mut cells = Library::new("cells");
    for index in 0..CELLS {
        let mut cell = Cell::new(format!("cell{index:04}"));
        cell.add_view(View::new("schematic", ViewType::Schematic));
        cells.add_cell(cell);
    }
    app.state.model.library_manager.add_library(cells);

    let mut overrides = app
        .state
        .model
        .schematic
        .editor_ref()
        .design
        .document()
        .components
        .iter()
        .take(BINDINGS)
        .map(|component| ConfigurationSetOverride {
            instance_path: InstancePath::root()
                .child(&component.name)
                .expect("a placed name is a path segment")
                .to_string(),
            executable_views: vec!["schematic".to_owned()],
            stop_view: None,
            model_section: None,
            eligible_platforms: vec![ConfigurationPlatform::Desktop],
        })
        .collect::<Vec<_>>();
    overrides.sort_by_key(|binding| binding.instance_path.to_lowercase());
    let mut configurations = (*app.state.model.workspace.content.configuration_sets).clone();
    configurations
        .create(ConfigurationSetDefinition {
            name: "Large".to_owned(),
            root,
            dut_path: InstancePath::root().to_string(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides,
            cell_bindings: Vec::new(),
            model_profile: ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Frame gate".to_owned(),
        })
        .expect("the configuration is well formed");
    app.state
        .model
        .workspace
        .content
        .replace_configuration_sets(configurations)
        .expect("the configuration publishes");

    app.state.model.provision_test_project_technology_contract();
    assert!(
        app.state
            .save_project_to_path(&folder.join("large.rspiceproj")),
        "the large project saves"
    );
    for _ in 0..CHECKPOINTS {
        create(&mut app.state, ProjectCheckpointReason::Manual).expect("checkpoint written");
    }
    (app, folder)
}

/// The page's share of the work thirty idle frames do. On no page does an
/// idle frame capture or fingerprint the project: the unsaved-document
/// answer is held for the working state until it changes.
fn idle_frames(ctx: &Context, app: &mut RSpiceApp) -> FrameWork {
    take_work();
    for _ in 0..IDLE_FRAMES {
        app_frame(ctx, app, Vec::new());
    }
    let frames = take_work();
    assert_eq!(
        (frames.fingerprint_passes, frames.project_captures),
        (0, 0),
        "an idle frame neither captures nor fingerprints the project: {frames:?}"
    );
    let _ = dirty_document_count(&app.state);
    let per_read = take_work();
    frames.page_work(per_read)
}

/// The page's share of the work thirty idle frames of `page` do, counted
/// from the first frame after the page has shown what it shows.
fn idle_work(page: ProjectPage) -> FrameWork {
    let (mut app, folder) = large_project("frame-work", COMPONENTS);
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = page;
    let ctx = offscreen();
    for _ in 0..WARM_UP_FRAMES {
        app_frame(&ctx, &mut app, Vec::new());
    }
    if page == ProjectPage::Recovery {
        paint_until(&ctx, &mut app, |text| text.contains("8 checkpoints"));
    }
    let work = idle_frames(&ctx, &mut app);
    let _ = std::fs::remove_dir_all(folder);
    work
}

#[test]
fn the_overview_does_no_project_work_on_an_idle_frame() {
    assert_eq!(idle_work(ProjectPage::Overview), FrameWork::default());
}

#[test]
fn the_library_page_does_no_project_work_on_an_idle_frame() {
    assert_eq!(idle_work(ProjectPage::Library), FrameWork::default());
}

#[test]
fn the_configuration_page_does_no_project_work_on_an_idle_frame() {
    assert_eq!(idle_work(ProjectPage::Configuration), FrameWork::default());
}

#[test]
fn the_dependencies_page_does_no_project_work_on_an_idle_frame() {
    assert_eq!(idle_work(ProjectPage::Dependencies), FrameWork::default());
}

/// The checkpoints are listed on a background thread: the first frame says
/// so, and neither it nor any frame after reads a checkpoint.
#[test]
fn the_recovery_page_lists_its_checkpoints_off_the_frame_and_idles() {
    let (mut app, folder) = large_project("frame-work-recovery", COMPONENTS);
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Recovery;
    let ctx = offscreen();

    take_work();
    let first = app_frame(&ctx, &mut app, Vec::new());
    assert!(
        painted(&first).contains("Listing checkpoints\u{2026}"),
        "{}",
        painted(&first)
    );
    paint_until(&ctx, &mut app, |text| text.contains("8 checkpoints"));
    let listing = take_work();
    assert_eq!(
        (listing.project_file_reads, listing.checkpoint_reads),
        (0, 0),
        "the frames that waited for the list read nothing"
    );
    assert_eq!(idle_frames(&ctx, &mut app), FrameWork::default());
    let _ = std::fs::remove_dir_all(folder);
}

/// `count` and the noun for it, as the Overview states a count.
fn counted(count: usize, one: &str, many: &str) -> String {
    format!("{count} {}", if count == 1 { one } else { many })
}

/// A project on the Overview with its root schematic in the editor.
fn overview_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let content = &mut app.state.model.workspace.content;
    content.active_view = CellViewRef::new(
        &content.project.root_library,
        &content.project.top_cell,
        rspice_app_state::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    );
    show_overview(&mut app);
    app
}

fn show_overview(app: &mut RSpiceApp) {
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Overview;
}

/// The Overview keeps what it counted between frames, so what it keeps has
/// to follow the project: a cell, a component and a model library added
/// since the last frame are all counted on the next one.
#[test]
fn the_overview_counts_what_changed_since_its_last_frame() {
    let mut app = overview_app();
    let ctx = offscreen();
    let cells = app.state.model.library_manager.total_cell_count();
    let first = painted(&app_frame(&ctx, &mut app, Vec::new()));
    assert!(
        first.contains(&format!("\u{b7} {}", counted(cells, "cell", "cells"))),
        "{first}"
    );

    let mut late = Library::new("late");
    let mut cell = Cell::new("added");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    late.add_cell(cell);
    app.state.model.library_manager.add_library(late);
    app.state.model.schematic.with_undo("Place", |schematic| {
        schematic.add_component(ComponentType::Resistor, Point::new(0, 0));
    });
    app.state
        .model
        .model_library_manager
        .load_library_bytes(
            "late.lib",
            b".model nlate nmos level=1 vto=0.5".to_vec(),
            None,
        )
        .expect("the model library imports");

    let next = painted(&app_frame(&ctx, &mut app, Vec::new()));
    assert!(
        next.contains(&format!("\u{b7} {}", counted(cells + 1, "cell", "cells"))),
        "the added cell is counted: {next}"
    );
    let design = app.state.model.schematic.editor_ref().design;
    let instances = design.document().components.len();
    let nets = rspice_design::connectivity::summary::design_nets(design).len();
    assert!(
        next.contains(&format!(
            "{} \u{b7} {}",
            counted(instances, "instance", "instances"),
            counted(nets, "net", "nets")
        )),
        "the placed component and its nets are counted: {next}"
    );
    let models = app.state.model.model_library_manager.total_model_count();
    assert!(
        next.contains(&format!("\u{b7} {models} models \u{b7}")),
        "the imported model is counted: {next}"
    );
}

/// A source deck's line count is kept beside the deck's digest: the deck
/// replaced since the last frame is counted on the next.
#[test]
fn the_overview_counts_the_lines_of_the_deck_in_front() {
    let mut app = overview_app();
    assert!(app.state.apply_imported_netlist(
        "V1 out 0 1\n.op\n.end\n".to_owned(),
        None,
        "front_end.sp"
    ));
    // Importing a deck opens it in the netlist editor.
    show_overview(&mut app);
    let ctx = offscreen();
    let first = painted(&app_frame(&ctx, &mut app, Vec::new()));
    assert!(first.contains("\u{b7} 3 lines \u{b7}"), "{first}");

    assert!(app.state.apply_imported_netlist(
        "* front end\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned(),
        None,
        "front_end.sp"
    ));
    show_overview(&mut app);
    let next = painted(&app_frame(&ctx, &mut app, Vec::new()));
    assert!(next.contains("\u{b7} 5 lines \u{b7}"), "{next}");
}

/// The model library at `path`, imported from disk as the Models workspace
/// imports it, attached with the test PDK package as the project's
/// technology.
fn attach_technology_from(app: &mut RSpiceApp, path: &Path) {
    let model = &mut app.state.model;
    let (archive, authority) = rspice_app_state::state::pdk_config::pdk_technology_test_fixture();
    model
        .pdk_config
        .technology_registry
        .install_archive_bytes(&archive, &authority, "Install the test technology")
        .expect("the test PDK installs");
    let package = model.pdk_config.technology_registry.validated_packages()[0].clone();
    let name = model
        .model_library_manager
        .load_library_file(path, None)
        .expect("the model file imports");
    let library = model
        .model_library_manager
        .get_library(&name)
        .expect("the imported library is in the catalogue");
    let binding = ProjectTechnologyBinding::from_model_library(library)
        .and_then(|binding| binding.with_pdk_package_metadata(package.metadata()))
        .expect("the library and the package form one binding");
    let context = ProjectTechnologyChangeContext::new(
        ProjectTechnologyChangeAuthority::new(
            "frames@rspice.invalid",
            "test:frame-work",
            "Attach a technology read from disk",
        )
        .expect("the authority is well formed"),
        uuid::Uuid::new_v4(),
        model.workspace.content.project.revision(),
        1,
        ContentDigest::from_bytes([0x7a; 32]),
        1,
    )
    .expect("the evidence is well formed");
    model
        .workspace
        .content
        .project
        .attach_technology_audited(binding, context)
        .expect("the technology attaches");
    model
        .validate_project_technology_contract()
        .expect("the attached technology resolves");
}

/// Dependencies keeps the technology verdict until what it was proven
/// against changes, so a refresh has to reach it: the attached model file
/// edited on disk and refreshed into the catalogue is stale on the next
/// frame.
#[test]
fn dependencies_states_a_stale_technology_once_its_model_file_is_refreshed() {
    let folder = unique_temp_dir("frame-work-drift");
    let model_file = folder.join("demo180.lib");
    std::fs::write(&model_file, ".model nmos_drift nmos level=1 vto=0.55\n")
        .expect("the model file writes");
    let mut app = RSpiceApp::test_instance();
    attach_technology_from(&mut app, &model_file);
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Dependencies;
    let ctx = offscreen();
    let stale = "Attached technology contract is stale";

    let first = painted(&app_frame(&ctx, &mut app, Vec::new()));
    assert!(first.contains("demo180"), "{first}");
    assert!(!first.contains(stale), "{first}");

    std::fs::write(&model_file, ".model nmos_drift nmos level=1 vto=0.60\n")
        .expect("the model file rewrites");
    app.state
        .model
        .model_library_manager
        .load_library_file(&model_file, None)
        .expect("the refresh reads the edited file");
    let next = painted(&app_frame(&ctx, &mut app, Vec::new()));
    assert!(next.contains(stale), "{next}");
    let _ = std::fs::remove_dir_all(folder);
}

/// The dependency ledger and the rail beside it keep their rows until a
/// catalogue changes: a library added since the last frame is a row of both
/// on the next.
#[test]
fn a_library_added_since_the_last_frame_is_a_dependency() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Dependencies;
    let ctx = offscreen();
    let first = painted(&app_frame(&ctx, &mut app, Vec::new()));
    assert!(!first.contains("aa-late"), "{first}");

    app.state
        .model
        .library_manager
        .add_library(Library::new("aa-late"));
    let next = painted(&app_frame(&ctx, &mut app, Vec::new()));
    assert!(
        next.matches("aa-late").count() >= 2,
        "the rail and the ledger both list it: {next}"
    );
}

/// One whole application frame on a screen of `size`.
fn sized_frame(ctx: &Context, app: &mut RSpiceApp, size: egui::Vec2) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            ..Default::default()
        },
        |ui| {
            let ctx = ui.ctx().clone();
            app.prepare_frame(&ctx);
            app.render_frame_chrome(ui);
            app.render_frame_dialogs(&ctx);
        },
    )
}

/// The ledger lays out the rows the reader can see, not every row: three
/// hundred libraries, beside the technology panel or stacked under it on a
/// narrow window, draw a screenful of ledger rows.
#[test]
fn the_dependency_ledger_draws_only_the_rows_in_view() {
    let mut app = RSpiceApp::test_instance();
    for index in 0..300 {
        app.state
            .model
            .library_manager
            .add_library(Library::new(format!("dep-{index:03}")));
    }
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Dependencies;
    for size in [egui::vec2(1600.0, 900.0), egui::vec2(600.0, 1000.0)] {
        let ctx = offscreen();
        sized_frame(&ctx, &mut app, size);
        let text = painted(&sized_frame(&ctx, &mut app, size));
        // Every design library's ledger row names where it lives.
        let rows = text.matches("Project file").count();
        assert!(
            (1..60).contains(&rows),
            "{size:?}: {rows} ledger rows drawn\n{text}"
        );
    }
}

/// The attach dialog proves its candidate libraries when the model
/// catalogue changes, not on each frame it stays open: a library imported
/// while it is open is offered on the next frame, and the frames after that
/// hash nothing.
#[test]
fn the_attach_dialog_proves_its_candidates_when_the_catalogue_changes() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Dependencies;
    app.state.project.technology_attachment.open = true;
    let ctx = offscreen();
    paint_until(&ctx, &mut app, |text| {
        text.contains("No model library can be attached")
    });

    app.state
        .model
        .model_library_manager
        .load_library_bytes(
            "late.lib",
            b".model nlate nmos level=1 vto=0.5".to_vec(),
            None,
        )
        .expect("the model library imports");
    let offered = painted(&app_frame(&ctx, &mut app, Vec::new()));
    assert!(
        offered.contains("\u{b7} 1 model \u{b7} 1 source file")
            && !offered.contains("No model library can be attached"),
        "{offered}"
    );
    assert_eq!(idle_frames(&ctx, &mut app).model_library_hashes, 0);
}

/// How long an idle Overview frame takes on an empty project and on the
/// large one, and how long a read of the held unsaved-document answer
/// takes. Numbers to read from a release build, not a rule: the rule is the
/// work counted above.
///
/// `cargo test --release -p rspice-app --lib -- overview_idle_frame_time
/// --ignored --nocapture`
#[test]
#[ignore = "a release-build timing to read, not a rule"]
#[allow(
    clippy::print_stdout,
    reason = "run by hand with --nocapture, the numbers are what it is for"
)]
fn overview_idle_frame_time() {
    for components in [0, COMPONENTS] {
        let (mut app, folder) = large_project("frame-time", components);
        show_overview(&mut app);
        let ctx = offscreen();
        for _ in 0..WARM_UP_FRAMES {
            app_frame(&ctx, &mut app, Vec::new());
        }
        let frames = u32::try_from(IDLE_FRAMES).expect("the frame count fits");
        let started = std::time::Instant::now();
        for _ in 0..IDLE_FRAMES {
            app_frame(&ctx, &mut app, Vec::new());
        }
        let frame = started.elapsed() / frames;
        let started = std::time::Instant::now();
        for _ in 0..IDLE_FRAMES {
            let _ = dirty_document_count(&app.state);
        }
        let read = started.elapsed() / frames;
        println!(
            "Overview, {components} components: {:.2} ms a frame; a read of the unsaved-document answer {:.3} ms",
            frame.as_secs_f64() * 1e3,
            read.as_secs_f64() * 1e3,
        );
        let _ = std::fs::remove_dir_all(folder);
    }
}
