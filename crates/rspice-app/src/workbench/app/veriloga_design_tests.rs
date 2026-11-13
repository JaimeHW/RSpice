//! The helpers the Verilog-A design tests share with the shell's own
//! whole-frame tests: a project application, a Verilog-A cell added to it
//! and opened in the Design workspace, and the themed frames that draw it.
//! The design tests themselves are the code feature's, in
//! feature_tests/code/veriloga_design.

use super::*;
use crate::state::{
    Cell, CellViewRef, Library, ProjectSourceBundle, ProjectSourceId, ProjectSourceLanguage,
    ProjectSourceOwner, View, ViewType,
};
use crate::workbench::state::Workspace;

const LIBRARY: &str = "behavioral";

/// A library cell whose `veriloga` view owns a one-file bundle at `root`.
pub(crate) fn add_veriloga_cell(
    state: &mut AppState,
    cell: &str,
    root: &str,
    module: &str,
) -> (CellViewRef, ProjectSourceId) {
    if state.model.library_manager.get_library(LIBRARY).is_none() {
        state
            .model
            .library_manager
            .add_library(Library::new(LIBRARY));
    }
    let reference = CellViewRef::new(LIBRARY, cell, "veriloga");
    let mut view = View::new("veriloga", ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), module.to_owned());
    view.metadata
        .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
    let mut library_cell = Cell::new(cell);
    library_cell.add_view(view);
    state
        .model
        .library_manager
        .get_library_mut(LIBRARY)
        .expect("the behavioral library")
        .add_cell(library_cell);
    let bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::cell_view(reference.clone()),
        ProjectSourceLanguage::VerilogA,
        root,
        module_source(module, ""),
        [],
        [],
    )
    .expect("a valid cell-view bundle");
    let id = bundle.id();
    state
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle)
        .expect("one bundle per cell view");
    (reference, id)
}

pub(crate) fn module_source(module: &str, extra: &str) -> String {
    format!(
        "// Behavioral conductance.\nmodule {module}(p, n);\n  inout p, n;\n  electrical p, n;\n  parameter real g = 1m;\n{extra}  analog I(p, n) <+ g * V(p, n);\nendmodule\n"
    )
}

pub(crate) fn open_project_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app
}

/// Open `reference` the way the library and the schematic do, then show it.
pub(crate) fn open_in_design(state: &mut AppState, reference: &CellViewRef) {
    state.open_workspace_view(reference.clone());
    state.workbench.activate(Workspace::Design);
}

/// One whole application frame: the frame loop's preparation, the chrome,
/// the docks and the surface, and the dialogs.
pub(crate) fn frame(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    size: egui::Vec2,
) -> egui::FullOutput {
    frame_with(ctx, app, size, Vec::new())
}

/// One whole application frame with `events` delivered to it.
pub(crate) fn frame_with(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    size: egui::Vec2,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            events,
            ..Default::default()
        },
        |ui| {
            app.prepare_frame(ui.ctx());
            app.render_frame_chrome(ui);
            app.render_frame_dialogs(ui.ctx());
        },
    )
}

pub(crate) fn themed_context() -> egui::Context {
    let ctx = egui::Context::default();
    // egui names a shortcut's modifier after the host's operating system, so
    // a context that inherits it asserts "Ctrl+F" on one machine and "Cmd+F"
    // on another. These tests read menus, so the context states its platform.
    ctx.set_os(egui::os::OperatingSystem::Nix);
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    ctx
}

pub(crate) const DESKTOP: egui::Vec2 = egui::vec2(1440.0, 900.0);
