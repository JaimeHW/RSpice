//! The Library page, the cell-view manager and the Configuration page
//! through whole application frames: a frame that changes nothing lists,
//! sorts, scans, projects and resolves nothing, and a change to what a page
//! reads shows on the next frame.

#![cfg(not(target_arch = "wasm32"))]

use egui::{Context, Pos2, Rect, Vec2, vec2};

use crate::workbench::RSpiceApp;
use crate::workbench::examples::hierarchy_reference;
use rspice_app_project::workbench::project::configuration_state::{
    CONFIGURATION_SCANS, ConfigurationTab,
};
use rspice_app_project::workbench::project::workspace_state::{LibraryCellviewPage, ProjectPage};
use rspice_app_project::workbench::surfaces::library_cellview_manager::CELLVIEW_PROJECTION_BUILDS;
use rspice_app_project::workbench::surfaces::library_cellview_manager::tests::typed_symbol;
use rspice_app_project::workbench::surfaces::project::library::LIBRARY_ROW_BUILDS;
use rspice_app_state::state::{
    Cell, ConfigurationCellBinding, Library, LibraryCellInstance, Point, SchematicState, View,
    ViewType,
};
use rspice_app_state::workbench::app_state::cell_usage::CELL_USAGE_SCANS;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::routing::{
    navigation::RouteTransitionSource, surface_catalog::SurfaceId, surface_route::SurfaceRoute,
};

const IDLE_FRAMES: usize = 30;
const SIZE: Vec2 = vec2(1600.0, 900.0);

/// One whole application frame, as the shell paints it.
fn app_frame(ctx: &Context, app: &mut RSpiceApp) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, SIZE)),
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

fn offscreen() -> Context {
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx
}

/// Every string a frame painted, one per line.
fn painted(output: &egui::FullOutput) -> String {
    fn collect(shape: &egui::epaint::Shape, text: &mut String) {
        match shape {
            egui::epaint::Shape::Text(shape) => {
                text.push_str(shape.galley.text());
                text.push('\n');
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, text);
                }
            }
            _ => {}
        }
    }
    let mut text = String::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut text);
    }
    text
}

/// Paint `IDLE_FRAMES` frames that change nothing, and the last of them.
fn idle(ctx: &Context, app: &mut RSpiceApp) -> egui::FullOutput {
    let mut last = app_frame(ctx, app);
    for _ in 1..IDLE_FRAMES {
        last = app_frame(ctx, app);
    }
    last
}

fn take(counter: &'static std::thread::LocalKey<std::cell::Cell<usize>>) -> usize {
    counter.with(|count| count.replace(0))
}

/// An open project whose library `big` holds 5,000 cells, each with a
/// symbol, the first of them selected and placed in the open schematic.
fn big_library_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let mut library = Library::new("big");
    for index in 0..5_000 {
        let mut cell = Cell::new(format!("c{index:04}"));
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
    }
    app.state.model.library_manager.add_library(library);
    app.state
        .model
        .library_manager
        .select_view("big", "c0000", "symbol");
    app.state.model.schematic.add_library_cell_component(
        Point::origin(),
        LibraryCellInstance::new("big", "c0000", "symbol"),
    );
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Library;
    app
}

/// With a 5,000-cell library open, a frame that changes nothing neither
/// lists nor sorts a column, nor walks the drawings for the inspector's
/// Used by; a renamed cell is listed in its new place, and a new placement
/// is named, on the very next frame. The new drawing's path sorts before
/// the open one's, so Used by names it first.
#[test]
fn an_idle_library_page_lists_and_scans_nothing() {
    let mut app = big_library_app();
    let ctx = offscreen();
    take(&LIBRARY_ROW_BUILDS);
    take(&CELL_USAGE_SCANS);

    let first = app_frame(&ctx, &mut app);
    assert!(painted(&first).contains("c0000"), "{}", painted(&first));
    assert!(
        take(&LIBRARY_ROW_BUILDS) > 0,
        "the first frame lists the columns"
    );
    assert!(
        take(&CELL_USAGE_SCANS) > 0,
        "the first frame finds the placements"
    );
    let shown = idle(&ctx, &mut app);
    assert!(painted(&shown).contains("Used by"), "{}", painted(&shown));
    assert_eq!(
        take(&LIBRARY_ROW_BUILDS),
        0,
        "an idle frame lists no column"
    );
    assert_eq!(take(&CELL_USAGE_SCANS), 0, "an idle frame walks no drawing");

    let renamed = app
        .state
        .model
        .library_manager
        .edit_library("big")
        .expect("the library is there")
        .rename_cell("c0001", "a_renamed");
    assert!(renamed);
    let next = app_frame(&ctx, &mut app);
    assert!(take(&LIBRARY_ROW_BUILDS) > 0);
    let text = painted(&next);
    let at = |name: &str| {
        text.find(name)
            .unwrap_or_else(|| panic!("{name} is drawn:\n{text}"))
    };
    assert!(
        at("a_renamed") < at("c0002"),
        "the renamed cell sorts first"
    );
    assert!(!text.contains("c0001"));

    let mut bench = SchematicState::default();
    bench.add_library_cell_component(
        Point::origin(),
        LibraryCellInstance::new("big", "c0000", "symbol"),
    );
    app.state
        .model
        .workspace
        .insert_schematic_editor("a/bench/schematic".to_owned(), bench);
    let next = app_frame(&ctx, &mut app);
    assert_eq!(take(&CELL_USAGE_SCANS), 1, "the new drawing is read once");
    assert!(
        painted(&next).contains("Used by\na/bench/schematic"),
        "{}",
        painted(&next)
    );
    idle(&ctx, &mut app);
    assert_eq!(take(&LIBRARY_ROW_BUILDS), 0);
    assert_eq!(take(&CELL_USAGE_SCANS), 0);
}

/// The manager's Symbol & form page decodes, checks and draws the selected
/// symbol once, and its header counts the catalog once; a new revision of
/// the symbol's form shows on the next frame.
#[test]
fn an_idle_cellview_manager_projects_nothing() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let mut definition = typed_symbol();
    let store = |definition: &rspice_app_state::state::ModelBoundSymbolDefinition| {
        let mut view = View::new("symbol", ViewType::Symbol);
        rspice_app_state::state::store_model_bound_symbol(definition, &mut view)
            .expect("the typed symbol stores");
        view
    };
    let mut cell = Cell::new("amp");
    cell.add_view(store(&definition));
    let mut library = Library::new("authoring");
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state
        .model
        .library_manager
        .select_view("authoring", "amp", "symbol");
    app.state.project.library_cellview_page = LibraryCellviewPage::SymbolForm;
    app.state
        .workbench
        .navigate(
            SurfaceRoute::surface(SurfaceId::LibraryCellviewManager),
            RouteTransitionSource::User,
        )
        .expect("the manager's route is executable");
    let ctx = offscreen();
    take(&CELLVIEW_PROJECTION_BUILDS);

    let first = app_frame(&ctx, &mut app);
    assert!(
        painted(&first).contains("Form revision r3"),
        "{}",
        painted(&first)
    );
    assert!(take(&CELLVIEW_PROJECTION_BUILDS) > 0);
    idle(&ctx, &mut app);
    assert_eq!(
        take(&CELLVIEW_PROJECTION_BUILDS),
        0,
        "an idle frame projects nothing"
    );

    definition.parameter_form.revision = 4;
    app.state
        .model
        .library_manager
        .edit_library("authoring")
        .expect("the library is there")
        .add_view("amp", store(&definition))
        .expect("the cell is there");
    let next = app_frame(&ctx, &mut app);
    assert!(
        painted(&next).contains("Form revision r4"),
        "{}",
        painted(&next)
    );
    assert!(take(&CELLVIEW_PROJECTION_BUILDS) > 0);
    idle(&ctx, &mut app);
    assert_eq!(take(&CELLVIEW_PROJECTION_BUILDS), 0);
}

/// The Configuration page over a configuration with 200 cell bindings
/// resolves the design and builds its tables once: idle frames on either
/// table, and switching between them, resolve and walk nothing. An edit
/// resolves again, once, and shows on the next frame.
#[test]
fn an_idle_configuration_page_resolves_and_walks_nothing() {
    let mut app = RSpiceApp::test_instance();
    app.state = hierarchy_reference::build().state;
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Configuration;
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    let id = catalog
        .active_configuration_id()
        .expect("the reference uses a configuration");
    let retire = |definition: &mut rspice_app_state::state::ConfigurationSetDefinition, index| {
        definition.cell_bindings.push(ConfigurationCellBinding {
            library: "user".to_owned(),
            cell: format!("retired_{index:03}"),
            executable_views: vec!["schematic".to_owned()],
            stop_view: None,
        });
    };
    app.state
        .project
        .configuration
        .edit(&catalog, id, |definition| {
            for index in 0..200 {
                retire(definition, index);
            }
        });
    let ctx = offscreen();
    take(&CONFIGURATION_SCANS);

    app_frame(&ctx, &mut app);
    let builds = app.state.project.configuration.resolution_builds();
    assert!(take(&CONFIGURATION_SCANS) > 0);
    idle(&ctx, &mut app);
    app.state.project.configuration.tab = ConfigurationTab::Cells;
    let cells = idle(&ctx, &mut app);
    assert!(
        painted(&cells).contains("200 bindings not in this design"),
        "{}",
        painted(&cells)
    );
    assert_eq!(app.state.project.configuration.resolution_builds(), builds);
    assert_eq!(take(&CONFIGURATION_SCANS), 0, "an idle frame walks nothing");

    app.state
        .project
        .configuration
        .edit(&catalog, id, |definition| retire(definition, 200));
    let next = app_frame(&ctx, &mut app);
    assert_eq!(
        app.state.project.configuration.resolution_builds(),
        builds + 1
    );
    assert!(
        painted(&next).contains("201 bindings not in this design"),
        "{}",
        painted(&next)
    );
    take(&CONFIGURATION_SCANS);
    idle(&ctx, &mut app);
    assert_eq!(
        app.state.project.configuration.resolution_builds(),
        builds + 1
    );
    assert_eq!(take(&CONFIGURATION_SCANS), 0);
}
