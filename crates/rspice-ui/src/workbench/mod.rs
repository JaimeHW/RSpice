//! Contract-driven, responsive RSpice application workbench.
//!
//! This is the sole owner of application chrome and top-level task layout.
//! Document engines such as the schematic canvas and precision result viewers
//! render inside its central surface but do not own navigation or chrome.

use std::time::Duration;

pub mod availability;
pub mod browser_navigation;
pub mod capability_workflow;
pub mod commands;
pub mod design_system;
mod feature_availability;
pub mod feature_availability_data;
pub mod navigation;
pub mod netlist_document;
pub mod result_document;
pub mod state;
pub mod surface_catalog;
pub mod surface_route;

mod chrome;
mod docks;
mod layout;
mod notification_center;
mod preflight;
mod project_launcher;
mod recovery;
mod session;
mod surfaces;

pub use availability::{
    SurfaceExecutionAvailability, SurfaceRouteUnavailable, route_availability, surface_availability,
};
pub use capability_workflow::{
    CapabilityWorkflowId, CapabilityWorkflowIdParseError, CapabilityWorkflowMetadata,
};
pub use navigation::{
    BrowserHistoryEffect, RouteTransition, RouteTransitionSource, SurfaceNavigation,
};
pub use result_document::{ResultViewer, ResultsState};
pub use session::{
    GridStyle, InspectorEdit, SymbolClipboard, SymbolDocumentSnapshot, SymbolSelection, SymbolTool,
    SymbolUiState, UiSessionState, UiSessionStateSer, mirror_point_h_about, mirror_point_v_about,
    mirror_shape_h_about, mirror_shape_v_about, rotate_point_cw_about, rotate_shape_cw_about,
    symbol_shape_bounds,
};
pub use state::{EngineeringProfile, WorkbenchState};
pub use surface_catalog::{
    CanonicalTier, NonPrimarySurface, ReleaseStatus, SurfaceArchetype, SurfaceId,
    SurfaceIdParseError, SurfaceMetadata,
};
pub use surface_route::{SurfaceRoute, SurfaceRouteParseError};

use egui::{CentralPanel, Context, Frame};

use crate::common::RSpiceApp;
use crate::ui::tokens::Tokens;

use layout::LayoutSpec;
use state::Workspace;

/// Render one complete workbench frame.
pub fn show(ctx: &Context, app: &mut RSpiceApp) {
    reconcile_platform_full_screen(app);
    synchronize_activity_stream(ctx, app);
    app.state.workbench.coarse_pointer = pointer_is_coarse(ctx, app.state.workbench.coarse_pointer);
    let viewport = ctx.content_rect().size();
    let layout = LayoutSpec::resolve_with_pointer_and_document_strip(
        viewport.x,
        viewport.y,
        app.state.workbench.coarse_pointer,
        chrome::document_bar::is_visible(app),
        &app.state.workbench,
    );
    // egui persists panel rectangles independently of application state.
    // Reconcile that cache before any command or panel is rendered so
    // responsive defaults and Reset Layout remain authoritative.
    docks::synchronize_panel_memory(ctx, app, layout);
    // Match the mockup's responsive interaction contract without mutating the
    // persisted Compact/Relaxed preference. Content controls become 44 px at
    // narrow widths or for coarse input; explicit chrome controls continue to
    // use LayoutSpec's more specific row/control dimensions.
    app.state.ui.theme.apply_responsive_metrics(
        ctx,
        viewport.x <= 820.0 || app.state.workbench.coarse_pointer,
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
    notification_center::show(ctx, app);
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
    app.state
        .ui
        .toasts
        .show(ctx, layout.title_bar_height, layout.toolbar_height);
    apply_platform_full_screen_request(ctx, app);
}

/// Render route-owned overlays after application dialogs so a workflow opened
/// from Preferences is the top modal while the originating dialog remains
/// retained underneath and can be restored on Close.
pub(crate) fn show_route_overlays(ctx: &Context, app: &mut RSpiceApp) {
    feature_availability::show(ctx, app);
}

fn synchronize_activity_stream(ctx: &Context, app: &mut RSpiceApp) {
    use crate::panels::{LogSeverity, LogSource};
    use crate::ui::widgets::{NotificationCategory, ToastKind};

    let revision = app.state.log_buffer.revision();
    let observed = app.state.ui.toasts.observed_log_revision();
    if revision == observed {
        return;
    }
    let now = ctx.input(|input| input.time);
    let session_elapsed = app.state.log_buffer.session_elapsed();
    let entries = app
        .state
        .log_buffer
        .entries()
        .filter(|entry| entry.id >= observed)
        .filter_map(|entry| {
            let category = match entry.source {
                LogSource::Simulation | LogSource::Engine | LogSource::Netlist | LogSource::Drc => {
                    NotificationCategory::Job
                }
                LogSource::User | LogSource::System => NotificationCategory::System,
            };
            let kind = match entry.severity {
                LogSeverity::Error => ToastKind::Error,
                LogSeverity::Warning => ToastKind::Warn,
                LogSeverity::Info => ToastKind::Info,
                LogSeverity::Debug | LogSeverity::Trace => return None,
            };
            Some((
                entry.id,
                category,
                kind,
                entry.message.clone(),
                project_log_timestamp(now, session_elapsed, entry.timestamp),
            ))
        })
        .collect::<Vec<_>>();
    app.state.ui.toasts.synchronize_activity(revision, entries);
}

/// Project a timestamp from the log buffer's session clock onto egui's
/// monotonic clock while preserving the event's original age.
fn project_log_timestamp(now: f64, session_elapsed: Duration, timestamp: Duration) -> f64 {
    now - session_elapsed.saturating_sub(timestamp).as_secs_f64()
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

    #[test]
    fn log_timestamp_projection_preserves_original_event_age() {
        assert_eq!(
            project_log_timestamp(120.0, Duration::from_secs(45), Duration::from_secs(15)),
            90.0
        );
    }

    #[test]
    fn log_timestamp_projection_bounds_future_source_timestamps_to_now() {
        assert_eq!(
            project_log_timestamp(120.0, Duration::from_secs(10), Duration::from_secs(15)),
            120.0
        );
    }
}
