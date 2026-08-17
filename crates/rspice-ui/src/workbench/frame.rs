//! One workbench frame: chrome, docks, and the central surface.
//!
//! This is the shell's rendering entry point. It lived in `workbench.rs`
//! itself, which meant the module root declared 58 submodules and then also
//! painted the application — so the file that should answer "what is the
//! shell made of" was 470 lines of layout code with the answer buried at the
//! top.

use std::time::Duration;

use egui::{CentralPanel, Context, Frame};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::ui::tokens::Tokens;

// The shell's submodules and re-exports, as this code saw them when it lived
// in `workbench.rs` itself.
use super::layout::LayoutSpec;
use super::state::Workspace;
use super::*;

pub(crate) fn enter_full_screen_presentation(
    app: &mut RSpiceApp,
    scope: crate::workbench::app::FullScreenScope,
    panels: crate::workbench::app::FullScreenPanels,
) {
    let platform_full_screen = scope == crate::workbench::app::FullScreenScope::ApplicationWindow;
    app.state.workbench.full_screen_presentation = true;
    app.state.workbench.full_screen_hide_context_panels =
        panels == crate::workbench::app::FullScreenPanels::HideNavigatorAndInspector;
    app.state.workbench.full_screen = platform_full_screen;
    if platform_full_screen {
        app.state.ui.request_full_screen(true);
    }
}

pub(crate) fn exit_full_screen_presentation(app: &mut RSpiceApp) {
    clear_full_screen_presentation(app, true);
}

fn clear_full_screen_presentation(app: &mut RSpiceApp, request_platform_exit: bool) {
    let platform_full_screen = app.state.workbench.full_screen;
    app.state.workbench.full_screen = false;
    app.state.workbench.full_screen_presentation = false;
    app.state.workbench.full_screen_hide_context_panels = false;
    if request_platform_exit && platform_full_screen {
        app.state.ui.request_full_screen(false);
    }
}

/// Render one complete workbench frame.
///
/// `root` is the frame `Ui` eframe hands to [`eframe::App::ui`]. Panels are
/// allocated out of it in order: egui subtracts each panel's edge from the
/// remaining rect, so call order is the layout.
pub fn show(root: &mut egui::Ui, app: &mut RSpiceApp) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    reconcile_platform_full_screen(app);
    synchronize_activity_stream(ctx, app);
    app.state.workbench.coarse_pointer = pointer_is_coarse(ctx, app.state.workbench.coarse_pointer);
    let viewport = ctx.content_rect().size();
    let context_docks_enabled = app.state.workbench.current_route().surface_id().archetype()
        != SurfaceArchetype::SpecialistWorkspace;
    let layout = LayoutSpec::resolve_for_shell(
        viewport.x,
        viewport.y,
        app.state.workbench.coarse_pointer,
        chrome::document_bar::is_visible(app),
        app.state.project_lifecycle.project_open,
        context_docks_enabled,
        &app.state.workbench,
    );
    // egui persists panel rectangles independently of application state.
    // Reconcile that cache before any command or panel is rendered so
    // responsive defaults and Reset Layout remain authoritative.
    docks::synchronize_panel_memory(ctx, app, layout);
    // Touch target inflation is capability-driven. A fine-pointer browser
    // resized through tablet/phone widths retains the compact mockup metrics.
    let large_targets = app.state.workbench.coarse_pointer;
    let requested_target = if app
        .state
        .ui
        .preferences
        .choice(ChoicePreference::MinimumTouchTarget)
        == 1
    {
        48.0
    } else {
        44.0
    };
    app.state
        .ui
        .theme
        .apply_responsive_metrics_with_target(ctx, large_targets.then_some(requested_target));
    let reduced_motion = app
        .state
        .ui
        .preferences
        .toggle(TogglePreference::ReducedMotion);
    // Custom widgets read this same policy instead of hard-coding their own
    // durations, so reduced motion is an application-wide contract rather
    // than an egui-default-only hint.
    ctx.global_style_mut(|style| style.animation_time = if reduced_motion { 0.0 } else { 0.16 });
    app.state.workbench.reconcile_drawer_mode(
        layout.navigator_uses_drawer,
        layout.inspector_uses_drawer,
        layout.workspaces_uses_drawer,
    );
    if app.state.workbench.full_screen_presentation
        && !app.state.application_modal_open()
        && ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
    {
        exit_full_screen_presentation(app);
    }
    if app.state.workbench.full_screen_presentation {
        show_full_screen_presentation(root, app, layout);
        return;
    }

    // Global chrome is allocated first. Workbench columns are then allocated
    // before the document strip and console so both rows remain confined to
    // the center stack exactly as in the mockup grid.
    if layout.show_status_bar {
        chrome::status_bar::show(root, app, layout);
    }
    if layout.show_phone_navigation {
        chrome::phone_navigation::show(root, app, layout);
    }
    chrome::title_bar::show(root, app, layout);
    chrome::toolbar::show(root, app, layout);
    // egui allocates same-edge side panels from the outside inward in call
    // order. Reserve the canonical 51 px activity rail first, then place the
    // navigator between that rail and the active document surface.
    if layout.show_activity_rail {
        chrome::activity_rail::show(root, app);
    }
    if layout.show_navigator_dock {
        docks::show_navigator(root, app, layout);
    }
    if layout.show_inspector_dock {
        docks::show_inspector(root, app, layout);
    }
    chrome::document_bar::show(root, app, layout);
    docks::show_console(root, app, layout);

    let t = Tokens::get(ctx);
    CentralPanel::default()
        .frame(Frame::new().fill(t.color.bg_app))
        .show(root, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            surfaces::show(ui, app);
        });

    if layout.has_overlay_drawer {
        docks::show_drawers(root, app, layout);
    }

    // The launcher is a canonical route even though its mockup composition is
    // an application-modal manager. Browser navigation and restored history
    // therefore own its lifetime; the transient flag only retains modal focus
    // state for the current frame.
    let launcher_route_active =
        app.state.workbench.current_route().surface_id() == SurfaceId::ProjectLauncher;
    if launcher_route_active && !app.state.workbench.project_launcher_open {
        app.state.workbench.open_project_launcher();
    } else if !launcher_route_active && app.state.workbench.project_launcher_open {
        app.state.workbench.project_launcher_open = false;
    }
    tools::project_launcher::show(ctx, app);
    // Shortcuts and dialogs record the intent to run; the pass itself belongs
    // to the frame loop, so a caller never has to name the workflow to ask
    // for it. Consumed before the dialog so a blocked report opens this frame.
    if app.state.workbench.preflight.take_run_and_queue_request() {
        preflight::run_and_queue(app);
    }
    preflight::show(ctx, app);
    tools::notification_center::show(ctx, app);
    // Export requests originate in retained result-document engines but IO is
    // owned by the app boundary.
    if let Some(keys) = app.state.ui.export_result_quantities_requested.take() {
        crate::workbench::menu_bar::action_export_result_selection_with_io(
            &mut app.state,
            app.export_workflow_io.as_ref(),
            &keys,
        );
    }
    if std::mem::take(&mut app.state.ui.export_csv_requested) {
        crate::workbench::menu_bar::action_export_csv_with_io(
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

fn show_full_screen_presentation(root: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    let hide_context_panels = app.state.workbench.full_screen_hide_context_panels;
    if !hide_context_panels && layout.show_navigator_dock {
        docks::show_navigator(root, app, layout);
    }
    if !hide_context_panels && layout.show_inspector_dock {
        docks::show_inspector(root, app, layout);
    }
    docks::show_console(root, app, layout);

    let t = Tokens::get(ctx);
    CentralPanel::default()
        .frame(Frame::new().fill(t.color.bg_app))
        .show(root, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            surfaces::show(ui, app);
        });

    if !hide_context_panels && layout.has_overlay_drawer {
        docks::show_drawers(root, app, layout);
    }

    let mut exit_requested = false;
    egui::Area::new(egui::Id::new("rspice-full-screen-exit"))
        .order(egui::Order::Foreground)
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-12.0, 12.0))
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(t.color.bg_panel)
                .stroke(egui::Stroke::new(1.0, t.color.border_strong))
                .corner_radius(t.radius)
                .inner_margin(egui::Margin::symmetric(8, 6))
                .show(ui, |ui| {
                    exit_requested = ui.button("Exit full screen  \u{00b7}  F11 / Esc").clicked();
                });
        });
    if exit_requested {
        exit_full_screen_presentation(app);
    }

    if let Some(keys) = app.state.ui.export_result_quantities_requested.take() {
        crate::workbench::menu_bar::action_export_result_selection_with_io(
            &mut app.state,
            app.export_workflow_io.as_ref(),
            &keys,
        );
    }
    if std::mem::take(&mut app.state.ui.export_csv_requested) {
        crate::workbench::menu_bar::action_export_csv_with_io(
            &mut app.state,
            app.export_workflow_io.as_ref(),
        );
    }
    if app.state.workbench.workspace != Workspace::Design {
        app.state.ui.canvas_hover = None;
        app.state.ui.canvas_view_center = None;
    }
    app.state.ui.toasts.show(ctx, 0.0, 0.0);
    apply_platform_full_screen_request(ctx, app);
}

/// Render one secondary native application viewport.
///
/// Application-global modal owners, notifications, exports, and toasts remain
/// in the primary viewport. The window still receives the complete workbench
/// chrome, dock composition, active document, and workspace surface.
pub(crate) fn show_secondary(root: &mut egui::Ui, app: &mut RSpiceApp) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    reconcile_platform_full_screen(app);
    app.state.workbench.coarse_pointer = pointer_is_coarse(ctx, app.state.workbench.coarse_pointer);
    let viewport = ctx.content_rect().size();
    let context_docks_enabled = app.state.workbench.current_route().surface_id().archetype()
        != SurfaceArchetype::SpecialistWorkspace;
    let layout = LayoutSpec::resolve_for_shell(
        viewport.x,
        viewport.y,
        app.state.workbench.coarse_pointer,
        chrome::document_bar::is_visible(app),
        app.state.project_lifecycle.project_open,
        context_docks_enabled,
        &app.state.workbench,
    );
    docks::synchronize_panel_memory(ctx, app, layout);
    app.state.workbench.reconcile_drawer_mode(
        layout.navigator_uses_drawer,
        layout.inspector_uses_drawer,
        layout.workspaces_uses_drawer,
    );

    if layout.show_status_bar {
        chrome::status_bar::show(root, app, layout);
    }
    if layout.show_phone_navigation {
        chrome::phone_navigation::show(root, app, layout);
    }
    chrome::title_bar::show(root, app, layout);
    chrome::toolbar::show(root, app, layout);
    if layout.show_activity_rail {
        chrome::activity_rail::show(root, app);
    }
    if layout.show_navigator_dock {
        docks::show_navigator(root, app, layout);
    }
    if layout.show_inspector_dock {
        docks::show_inspector(root, app, layout);
    }
    chrome::document_bar::show(root, app, layout);
    docks::show_console(root, app, layout);

    let t = Tokens::get(ctx);
    CentralPanel::default()
        .frame(Frame::new().fill(t.color.bg_app))
        .show(root, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            surfaces::show(ui, app);
        });
    if layout.has_overlay_drawer {
        docks::show_drawers(root, app, layout);
    }
    apply_platform_full_screen_request(ctx, app);
}

/// Truthful browser/backend fallback for a secondary logical window. The host
/// embeds it in a movable egui window, so nested top-level panels would escape
/// that frame; render the owned document surface directly instead.
pub(crate) fn show_embedded_secondary(ui: &mut egui::Ui, app: &mut RSpiceApp, title: &str) -> bool {
    let mut reattach = false;
    ui.horizontal(|ui| {
        ui.strong(title);
        ui.label(format!(
            "{} workspace \u{00b7} embedded single-host fallback",
            app.state.workbench.workspace.label()
        ));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            reattach = ui.button("Reattach to main window").clicked();
        });
    });
    ui.separator();
    surfaces::show(ui, app);
    reattach
}

/// Render route-owned overlays after application dialogs so a workflow opened
/// from Preferences is the top modal while the originating dialog remains
/// retained underneath and can be restored on Close.
pub(crate) fn show_route_overlays(ctx: &Context, app: &mut RSpiceApp) {
    account_organization::show(ctx, app);
    tools::jobs_manager::show(ctx, app);
    tools::specialist_tool_browser::show(ctx, app);
    feature_availability::show(ctx, app);
}

fn synchronize_activity_stream(ctx: &Context, app: &mut RSpiceApp) {
    use crate::ui::widgets::{NotificationCategory, ToastKind};

    let revision = app.state.log_buffer.revision();
    let observed = app.state.ui.toasts.observed_log_revision();
    if revision == observed {
        return;
    }
    let now = ctx.input(|input| input.time);
    let session_elapsed = app.state.log_buffer.session_elapsed();
    let attention = app
        .state
        .ui
        .preferences
        .workspace()
        .map(WorkspacePreferences::background_task_attention)
        .unwrap_or_default();
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
            if !attention.retains(matches!(kind, ToastKind::Error)) {
                return None;
            }
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
        let platform_full_screen = document.fullscreen_element().is_some();
        let platform_exit_ended_presentation =
            app.state.workbench.full_screen && !platform_full_screen;
        app.state.workbench.full_screen = platform_full_screen;
        if platform_exit_ended_presentation {
            clear_full_screen_presentation(app, false);
        }
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
        clear_full_screen_presentation(app, false);
        log::error!("Fullscreen request failed: browser document is unavailable");
        return;
    };

    if enabled {
        let Some(root) = document.document_element() else {
            clear_full_screen_presentation(app, false);
            log::error!("Fullscreen request failed: document root is unavailable");
            return;
        };
        if let Err(error) = root.request_fullscreen() {
            clear_full_screen_presentation(app, false);
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
    fn active_canvas_full_screen_never_requests_host_window_mutation() {
        let mut app = RSpiceApp::test_instance();
        enter_full_screen_presentation(
            &mut app,
            crate::workbench::app::FullScreenScope::ActiveCanvasOnly,
            crate::workbench::app::FullScreenPanels::HideNavigatorAndInspector,
        );

        assert!(app.state.workbench.full_screen_presentation);
        assert!(app.state.workbench.full_screen_hide_context_panels);
        assert!(!app.state.workbench.full_screen);
        assert_eq!(app.state.ui.take_full_screen_request(), None);

        exit_full_screen_presentation(&mut app);
        assert!(!app.state.workbench.full_screen_presentation);
        assert!(!app.state.workbench.full_screen_hide_context_panels);
        assert_eq!(app.state.ui.take_full_screen_request(), None);
    }

    #[test]
    fn application_full_screen_has_exactly_one_enter_and_exit_request() {
        let mut app = RSpiceApp::test_instance();
        enter_full_screen_presentation(
            &mut app,
            crate::workbench::app::FullScreenScope::ApplicationWindow,
            crate::workbench::app::FullScreenPanels::KeepCurrent,
        );

        assert!(app.state.workbench.full_screen_presentation);
        assert!(app.state.workbench.full_screen);
        assert_eq!(app.state.ui.take_full_screen_request(), Some(true));
        assert_eq!(app.state.ui.take_full_screen_request(), None);

        exit_full_screen_presentation(&mut app);
        assert!(!app.state.workbench.full_screen_presentation);
        assert!(!app.state.workbench.full_screen);
        assert_eq!(app.state.ui.take_full_screen_request(), Some(false));
        assert_eq!(app.state.ui.take_full_screen_request(), None);
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

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn background_attention_filters_the_live_activity_stream() {
        let ctx = Context::default();
        let mut app = RSpiceApp::test_instance();
        app.state
            .ui
            .preferences
            .workspace_mut()
            .unwrap()
            .set_background_task_attention(BackgroundTaskAttention::NotifyOnFailureOnly);
        app.state
            .log_buffer
            .log(LogSeverity::Info, LogSource::Simulation, "completed", None);
        app.state
            .log_buffer
            .log(LogSeverity::Error, LogSource::Simulation, "failed", None);

        synchronize_activity_stream(&ctx, &mut app);
        assert_eq!(app.state.ui.toasts.activity().len(), 1);
        assert_eq!(app.state.ui.toasts.activity()[0].message(), "failed");

        app.state
            .ui
            .preferences
            .workspace_mut()
            .unwrap()
            .set_background_task_attention(BackgroundTaskAttention::Silent);
        app.state.log_buffer.log(
            LogSeverity::Error,
            LogSource::Simulation,
            "second failure",
            None,
        );
        synchronize_activity_stream(&ctx, &mut app);
        assert_eq!(app.state.ui.toasts.activity().len(), 1);
    }
}
