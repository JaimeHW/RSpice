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
mod preflight;
mod project_launcher;
mod recovery;
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
    reconcile_platform_full_screen(app);
    if !app.state.workbench.application_modal_open() {
        handle_workbench_shortcuts(ctx, app);
    }
    app.state.workbench.coarse_pointer = pointer_is_coarse(ctx, app.state.workbench.coarse_pointer);
    let viewport = ctx.content_rect().size();
    let layout = LayoutSpec::resolve_with_pointer(
        viewport.x,
        viewport.y,
        app.state.workbench.coarse_pointer,
        &app.state.workbench,
    );
    app.state.workbench.reconcile_drawer_mode(
        layout.navigator_uses_drawer,
        layout.inspector_uses_drawer,
        layout.workspaces_uses_drawer,
    );

    // Global chrome is allocated first. Workbench columns are then allocated
    // before the document strip and console so both rows remain confined to
    // the center stack exactly as in the mockup grid.
    if layout.show_status_bar {
        chrome::status_bar::show(ctx, app, layout);
    }
    if layout.show_phone_navigation {
        chrome::phone_navigation::show(ctx, app, layout);
    }
    chrome::title_bar::show(ctx, app, layout);
    chrome::toolbar::show(ctx, app, layout);
    if layout.show_activity_rail {
        chrome::activity_rail::show(ctx, app);
    }
    if layout.show_navigator_dock {
        docks::show_navigator(ctx, app, layout);
    }
    if layout.show_inspector_dock {
        docks::show_inspector(ctx, app, layout);
    }
    chrome::document_bar::show(ctx, app, layout);
    docks::show_console(ctx, app, layout);

    let t = Tokens::get(ctx);
    CentralPanel::default()
        .frame(Frame::new().fill(t.color.bg_app))
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            surfaces::show(ui, app);
        });

    if layout.has_overlay_drawer {
        docks::show_drawers(ctx, app, layout);
    }

    project_launcher::show(ctx, app);
    preflight::show(ctx, app);

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
    apply_platform_full_screen_request(ctx, app);
}

fn pointer_is_coarse(ctx: &Context, retained_touch_capability: bool) -> bool {
    let touch_event = ctx.input(|input| {
        input
            .events
            .iter()
            .any(|event| matches!(event, egui::Event::Touch { .. }))
    });
    retained_touch_capability || touch_event || platform_pointer_is_coarse()
}

#[cfg(not(target_arch = "wasm32"))]
const fn platform_pointer_is_coarse() -> bool {
    cfg!(any(target_os = "android", target_os = "ios"))
}

#[cfg(target_arch = "wasm32")]
fn platform_pointer_is_coarse() -> bool {
    use wasm_bindgen::{JsCast as _, JsValue};

    let Some(window) = web_sys::window() else {
        return false;
    };
    let Ok(callable) = js_sys::Reflect::get(window.as_ref(), &JsValue::from_str("matchMedia"))
    else {
        return false;
    };
    let Ok(match_media) = callable.dyn_into::<js_sys::Function>() else {
        return false;
    };
    let Ok(result) = match_media.call1(window.as_ref(), &JsValue::from_str("(pointer: coarse)"))
    else {
        return false;
    };
    js_sys::Reflect::get(&result, &JsValue::from_str("matches"))
        .ok()
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
}

#[cfg(not(target_arch = "wasm32"))]
fn reconcile_platform_full_screen(_app: &mut RSpiceApp) {}

#[cfg(target_arch = "wasm32")]
fn reconcile_platform_full_screen(app: &mut RSpiceApp) {
    // Browser chrome (notably Escape) may leave fullscreen without routing a
    // command through egui. Reflect the platform truth back into retained UI
    // state so the menu checkmark and next toggle remain accurate.
    if app.state.ui.full_screen_request.is_none()
        && let Some(document) = web_sys::window().and_then(|window| window.document())
    {
        app.state.workbench.full_screen = document.fullscreen_element().is_some();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn apply_platform_full_screen_request(ctx: &Context, app: &mut RSpiceApp) {
    if let Some(enabled) = app.state.ui.take_full_screen_request() {
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(enabled));
    }
}

#[cfg(target_arch = "wasm32")]
fn apply_platform_full_screen_request(_ctx: &Context, app: &mut RSpiceApp) {
    let Some(enabled) = app.state.ui.take_full_screen_request() else {
        return;
    };
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        app.state.workbench.full_screen = false;
        log::error!("Fullscreen request failed: browser document is unavailable");
        return;
    };

    if enabled {
        let Some(root) = document.document_element() else {
            app.state.workbench.full_screen = false;
            log::error!("Fullscreen request failed: document root is unavailable");
            return;
        };
        if let Err(error) = root.request_fullscreen() {
            app.state.workbench.full_screen = false;
            log::warn!("Browser rejected fullscreen request: {error:?}");
        }
    } else {
        document.exit_fullscreen();
    }
}

fn handle_workbench_shortcuts(ctx: &Context, app: &mut RSpiceApp) {
    let command = ctx.input(|input| {
        if input.key_pressed(egui::Key::Escape) && app.state.workbench.drawer.is_some() {
            return Some(None);
        }
        if input.key_pressed(egui::Key::F11) {
            return Some(Some(Command::ToggleFullScreen));
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
        Some(None) => app.state.workbench.close_drawer(),
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
