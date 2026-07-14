//! Contract-driven, responsive RSpice application workbench.
//!
//! This is the sole owner of application chrome and top-level task layout.
//! Document engines such as the schematic canvas and precision result viewers
//! render inside its central surface but do not own navigation or chrome.

pub mod commands;
pub mod design_system;
pub mod netlist_document;
pub mod result_document;
pub mod state;

mod chrome;
mod docks;
mod layout;
pub(crate) mod menu;
mod project_launcher;
mod session;
mod surfaces;

pub use result_document::{ResultViewer, ResultsState};
pub use session::{
    GridStyle, InspectorEdit, SymbolClipboard, SymbolDocumentSnapshot, SymbolSelection, SymbolTool,
    SymbolUiState, UiSessionState, UiSessionStateSer, mirror_point_h_about, mirror_point_v_about,
    mirror_shape_h_about, mirror_shape_v_about, rotate_point_cw_about, rotate_shape_cw_about,
    symbol_shape_bounds,
};
pub use state::WorkbenchState;

use egui::{CentralPanel, Context, Frame};

use crate::common::RSpiceApp;
use crate::ui::tokens::Tokens;

use commands::Command;
use layout::LayoutSpec;
use state::Workspace;

/// Render one complete workbench frame.
pub fn show(ctx: &Context, app: &mut RSpiceApp) {
    handle_workbench_shortcuts(ctx, app);
    ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(
        app.state.workbench.full_screen,
    ));
    let layout = LayoutSpec::resolve(ctx.content_rect().width(), &app.state.workbench);

    // Outer bars first, then activity/docks, then the owner surface.
    chrome::status_bar::show(ctx, app, layout);
    if layout.show_phone_navigation {
        chrome::phone_navigation::show(ctx, app);
    }
    chrome::title_bar::show(ctx, app, layout);
    chrome::toolbar::show(ctx, app, layout);
    chrome::document_bar::show(ctx, app);

    if layout.show_console_body {
        docks::show_console(ctx, app, layout);
    }
    if layout.show_activity_rail {
        chrome::activity_rail::show(ctx, app);
    }
    if layout.show_navigator_dock {
        docks::show_navigator(ctx, app, layout);
    }
    if layout.show_inspector_dock {
        docks::show_inspector(ctx, app, layout);
    }

    let t = Tokens::get(ctx);
    CentralPanel::default()
        .frame(Frame::new().fill(t.color.bg_app))
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            surfaces::show(ui, app);
        });

    if layout.width_class.uses_drawers() {
        docks::show_drawers(ctx, app, layout);
    }

    project_launcher::show(ctx, app);

    // Export requests originate in retained result-document engines but IO is
    // owned by the app boundary.
    if std::mem::take(&mut app.state.ui.export_csv_requested) {
        crate::common::menu_bar::action_export_csv_with_io(
            &mut app.state,
            app.export_workflow_io.as_ref(),
        );
    }

    if app.state.workbench.workspace != Workspace::Design {
        app.state.ui.canvas_hover = None;
        app.state.ui.canvas_view_center = None;
    }
    app.state.ui.toasts.show(ctx);
}

fn handle_workbench_shortcuts(ctx: &Context, app: &mut RSpiceApp) {
    let command = ctx.input(|input| {
        if input.key_pressed(egui::Key::Escape) && app.state.workbench.drawer.is_some() {
            return Some(None);
        }
        if !input.modifiers.alt || input.modifiers.command || input.modifiers.shift {
            return None;
        }
        let workspace = [
            (egui::Key::Num1, Workspace::Project),
            (egui::Key::Num2, Workspace::Design),
            (egui::Key::Num3, Workspace::Simulate),
            (egui::Key::Num4, Workspace::Results),
            (egui::Key::Num5, Workspace::Verify),
            (egui::Key::Num6, Workspace::Models),
            (egui::Key::Num7, Workspace::Netlist),
        ]
        .into_iter()
        .find_map(|(key, workspace)| input.key_pressed(key).then_some(workspace));
        workspace.map(|workspace| Some(Command::OpenWorkspace(workspace)))
    });

    match command {
        Some(Some(command)) => command.execute(app),
        Some(None) => app.state.workbench.drawer = None,
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::state::Drawer;

    #[test]
    fn every_canonical_workspace_has_a_surface_owner() {
        assert_eq!(Workspace::ALL.len(), 7);
        assert_eq!(Workspace::PHONE_PRIMARY.len(), 4);
        assert!(Workspace::ALL.contains(&Workspace::Netlist));
    }

    #[test]
    fn drawer_state_is_transient_navigation_not_a_workspace() {
        let mut state = WorkbenchState::default();
        state.toggle_drawer(Drawer::Navigator);
        assert_eq!(state.workspace, Workspace::Design);
        assert_eq!(state.drawer, Some(Drawer::Navigator));
    }
}
