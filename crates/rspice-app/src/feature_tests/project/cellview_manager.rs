//! The library cellview manager through the whole application: the symbol
//! form's actions on the selected symbol answered by the shell, and a press
//! on the manager answered in the frame it was clicked in. The manager's own
//! drawing is tested with it, in the project feature.

use egui::{Modifiers, Rect, vec2};

use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::workspace_state::LibraryCellviewPage;
use rspice_app_project::workbench::surfaces::library_cellview_manager::tests::typed_symbol;
use rspice_app_state::state::{Cell, CellViewRef, Library, View, ViewType};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::requests::OpenSymbolAuthoring;
use rspice_app_workbench::workbench::routing::{
    navigation::RouteTransitionSource, surface_catalog::SurfaceId, surface_route::SurfaceRoute,
};

fn app_with_typed_symbol() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let definition = typed_symbol();
    let mut view = View::new("symbol", ViewType::Symbol);
    rspice_app_state::state::store_model_bound_symbol(&definition, &mut view)
        .expect("typed symbol stores");
    let mut cell = Cell::new("amp");
    cell.add_view(view);
    let mut library = Library::new("authoring");
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state
        .model
        .library_manager
        .select_view("authoring", "amp", "symbol");
    app.state.project.library_cellview_page = LibraryCellviewPage::SymbolForm;
    app
}

#[test]
fn explicit_form_action_targets_the_selected_symbol() {
    let mut app = app_with_typed_symbol();
    app.state
        .workbench
        .outbox
        .push(OpenSymbolAuthoring::ParameterForm(Some(CellViewRef::new(
            "authoring",
            "amp",
            "symbol",
        ))));
    app.after_contribution();
    assert_eq!(
        app.state
            .design
            .symbol_parameter_form
            .target
            .as_ref()
            .map(CellViewRef::display_path),
        Some("authoring/amp/symbol".to_owned())
    );
    assert_eq!(
        app.state
            .design
            .symbol_parameter_form
            .draft_form
            .as_ref()
            .map(|form| form.revision),
        Some(4)
    );
}

/// What one frame told assistive technology: every node it drew.
type FrameNodes = Vec<(egui::accesskit::NodeId, egui::accesskit::Node)>;

/// Press the manager's `button` on the Symbol & form page, in whole frames
/// over an open project: the application after the press, and what the
/// frame that drew the press drew.
fn press_in_the_manager(button: &str) -> (RSpiceApp, FrameNodes) {
    let mut app = app_with_typed_symbol();
    app.state.open_project_session();
    app.state
        .workbench
        .navigate(
            SurfaceRoute::surface(SurfaceId::LibraryCellviewManager),
            RouteTransitionSource::User,
        )
        .expect("the manager's route is executable");
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut frame = eframe::Frame::_new_kittest();
    let mut run = |app: &mut RSpiceApp, pass: u32, events: Vec<egui::Event>| {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1440.0, 900.0))),
                time: Some(f64::from(pass) * 0.1),
                focused: true,
                events,
                ..Default::default()
            },
            |ui| eframe::App::ui(app, ui, &mut frame),
        )
        .platform_output
        .accesskit_update
        .map(|update| update.nodes)
        .unwrap_or_default()
    };
    // The fonts exist from the second frame on, and the page lays out
    // against them on the third.
    let mut nodes = Vec::new();
    for pass in 1..=3 {
        nodes = run(&mut app, pass, Vec::new());
    }
    let bounds = nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| node.label() == Some(button))
        .and_then(|node| node.bounds())
        .unwrap_or_else(|| panic!("the page draws {button:?}"));
    let at = egui::pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    );
    let press = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: Modifiers::default(),
    };
    let pressed = run(
        &mut app,
        4,
        vec![egui::Event::PointerMoved(at), press(true), press(false)],
    );
    (app, pressed)
}

/// Whether a frame drew the dialog titled `title`.
fn drew_dialog(nodes: &FrameNodes, title: &str) -> bool {
    nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Dialog && node.label() == Some(title)
    })
}

/// Edit component form… raises the form designer for the symbol the page
/// shows, and the shell opens it as the manager returns: the frame that
/// drew the press draws the dialog.
#[test]
fn edit_component_form_draws_the_form_designer_in_the_frame_that_pressed_it() {
    let (app, pressed) = press_in_the_manager("Edit component form\u{2026}");
    assert!(
        drew_dialog(&pressed, "Open component form designer"),
        "the frame that drew the press draws the form designer"
    );
    assert_eq!(
        app.state.design.symbol_parameter_form.target,
        Some(CellViewRef::new("authoring", "amp", "symbol"))
    );
}

/// Import symbol… raises the importer bound to the symbol the page shows,
/// and the shell opens it as the manager returns: the frame that drew the
/// press draws the dialog.
#[test]
fn import_symbol_draws_the_importer_in_the_frame_that_pressed_it() {
    let (app, pressed) = press_in_the_manager("Import symbol\u{2026}");
    assert!(
        drew_dialog(&pressed, "Import symbol definition"),
        "the frame that drew the press draws the importer"
    );
    assert_eq!(
        app.state.design.symbol_import.binding_source,
        Some(CellViewRef::new("authoring", "amp", "symbol"))
    );
}

/// Open symbol editor takes the reader to the symbol the page shows, in
/// Design, before the frame that drew the press is over: that frame leaves
/// Design in front with the symbol open, off the manager's route.
#[test]
fn open_symbol_editor_lands_in_design_in_the_frame_that_pressed_it() {
    let (app, _) = press_in_the_manager("Open symbol editor");
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new("authoring", "amp", "symbol")
    );
    assert_ne!(
        app.state.workbench.current_route().surface_id(),
        SurfaceId::LibraryCellviewManager
    );
}
