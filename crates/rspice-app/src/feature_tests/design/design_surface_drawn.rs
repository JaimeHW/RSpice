//! The design surface drawn through a whole application: breadcrumbs and the occurrence
//! watermark.

use crate::workbench::RSpiceApp;
use crate::workbench::design::surface::tests::descended;
use crate::workbench::design::surface::{
    breadcrumb, hierarchy_breadcrumb_segments, master_occurrence_paths, occurrence_scope_banner,
};
use egui::{Rect, Vec2};
use rspice_app_state::state::ViewType;

/// The canvas every render in this module uses. Wide enough that the crumb
/// row is measured rather than elided.
const RASTER_VIEWPORT: Vec2 = egui::vec2(720.0, 60.0);

/// Every level is a place you can go, and going there opens exactly that
/// occurrence rather than the deepest one that shares its master.
#[test]
fn a_breadcrumb_level_opens_the_occurrence_it_names() {
    let mut app = RSpiceApp::test_instance();
    descended(&mut app.state.model, "XAFE", "afe_core");
    descended(&mut app.state.model, "XBIAS", "bias");

    let crumbs = hierarchy_breadcrumb_segments(&app.state.model);
    let levels: Vec<Option<usize>> = crumbs.iter().map(|crumb| crumb.level).collect();
    assert_eq!(
        levels,
        vec![None, Some(0), Some(1), Some(2), None],
        "the library and the view name no level; every occurrence level does"
    );

    let intermediate = crumbs
        .iter()
        .find(|crumb| crumb.label.starts_with("XAFE"))
        .and_then(|crumb| crumb.level)
        .expect("the descended level is a crumb");
    app.state
        .navigation()
        .focus_workspace_breadcrumb(intermediate);

    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .occurrence_path()
            .to_string(),
        "/XAFE"
    );
    assert_eq!(
        app.state.model.workspace.content.active_view.cell,
        "afe_core"
    );

    let root = hierarchy_breadcrumb_segments(&app.state.model)
        .into_iter()
        .find(|crumb| crumb.label == "top")
        .and_then(|crumb| crumb.level)
        .expect("the design root is a crumb");
    app.state.navigation().focus_workspace_breadcrumb(root);

    assert!(
        app.state
            .model
            .workspace
            .content
            .occurrence_path()
            .is_root()
    );
    assert_eq!(app.state.model.workspace.content.active_view.cell, "top");
}

/// A crumb states the instance *and* its master. Two occurrences of one
/// master must not paint the same crumb, and one instance name under two
/// masters must not either — which is what a crumb truncated to either half
/// would do.
#[test]
fn breadcrumb_segments_render_instance_and_master() {
    fn render(instance: &str, master: &str) -> rspice_app_workbench::ui::raster::Canvas {
        let mut app = RSpiceApp::test_instance();
        descended(&mut app.state.model, instance, master);
        rspice_app_workbench::ui::raster::render(RASTER_VIEWPORT, |ui, _| {
            breadcrumb(
                ui.ctx(),
                &mut app.state.canvas_frame(),
                Rect::from_min_size(egui::Pos2::ZERO, RASTER_VIEWPORT),
            );
        })
    }

    let band = Rect::from_min_size(egui::Pos2::ZERO, RASTER_VIEWPORT);
    let ink = |canvas: &rspice_app_workbench::ui::raster::Canvas| -> Vec<egui::Color32> {
        canvas.pixels_in(band).collect()
    };

    let first = render("X1", "amp");
    let second = render("X2", "amp");
    let elsewhere = render("X1", "bias");

    assert!(
        ink(&first).iter().any(|pixel| *pixel != first.background()),
        "the breadcrumb painted nothing at all"
    );
    assert_ne!(
        ink(&first),
        ink(&second),
        "a crumb that dropped the instance would paint X1 and X2 identically"
    );
    assert_ne!(
        ink(&first),
        ink(&elsewhere),
        "a crumb that dropped the master would paint amp and bias identically"
    );
}

/// A project whose root sheet instantiates one cell twice, with the design
/// root open. Descending into the shared cell is what puts a document with
/// two occurrences in front.
fn app_with_a_twice_instantiated_cell() -> (RSpiceApp, rspice_app_state::state::CellViewRef) {
    use rspice_app_state::state::{
        CellViewRef, ComponentType, Library, LibraryCellInstance, Point, SchematicState, View,
    };

    let mut app = RSpiceApp::test_instance();
    let root = app.state.model.workspace.content.active_view.clone();
    let shared = CellViewRef::new("user", "pad", "schematic");

    if app
        .state
        .model
        .library_manager
        .get_library(&shared.library)
        .is_none()
    {
        app.state
            .model
            .library_manager
            .add_library(Library::new(&shared.library));
    }
    let cell = app
        .state
        .model
        .library_manager
        .get_library_mut(&shared.library)
        .expect("the project library")
        .get_or_create_cell(&shared.cell);
    if cell.get_view(&shared.view).is_none() {
        cell.add_view(View::new(&shared.view, ViewType::Schematic));
    }

    let mut master = SchematicState::default();
    master.add_component(ComponentType::Resistor, Point::new(30, 0));
    app.state
        .model
        .workspace
        .insert_schematic_editor(shared.key(), master);

    let mut binding = LibraryCellInstance::new(&shared.library, &shared.cell, &shared.view);
    binding.bind_interface(&[]);
    let mut root_sheet = SchematicState::default();
    root_sheet.add_library_cell_component(Point::new(400, 400), binding.clone());
    root_sheet.add_library_cell_component(Point::new(600, 400), binding);
    app.state.model.schematic = root_sheet.clone();
    app.state
        .model
        .workspace
        .insert_schematic_editor(root.key(), root_sheet);
    (app, shared)
}

/// The strip states a scope, so it appears exactly where the scope is wider
/// than the sheet in front of the reader: never at a design root, never on a
/// master placed once, always on one placed more than once.
#[test]
fn occurrence_watermark_appears_only_above_one_instance() {
    fn render(app: &RSpiceApp) -> rspice_app_workbench::ui::raster::Canvas {
        rspice_app_workbench::ui::raster::render(RASTER_VIEWPORT, |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| occurrence_scope_banner(ui, &app.state.model));
        })
    }

    let (mut app, shared) = app_with_a_twice_instantiated_cell();
    assert_eq!(
        master_occurrence_paths(&app.state.model).len(),
        1,
        "the design root is instantiated once"
    );
    assert_eq!(
        render(&app).content_height(),
        0,
        "a design root must carry no occurrence marking"
    );

    let master = app
        .state
        .model
        .workspace
        .clone_schematic_editor(&shared.key())
        .expect("the shared master buffer");
    app.state
        .model
        .workspace
        .descend_into("X1".to_owned(), shared.clone(), ViewType::Schematic);
    app.state.model.schematic = master;

    assert_eq!(
        master_occurrence_paths(&app.state.model).len(),
        2,
        "the shared master is instantiated twice"
    );
    assert!(
        render(&app).content_height() > 0,
        "a master placed twice must say so before the edit"
    );
}
