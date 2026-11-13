//! The project launcher through the whole application: its page painted with
//! and without projects and in safe mode, and a dismissal returning where
//! the route history says.

use egui::{Context, Rect, Vec2, vec2};

use crate::workbench::RSpiceApp;
use crate::workbench::project::workspace_state::ProjectLauncherPage;
use crate::workbench::state::Workspace;
use crate::workbench::tools::project_launcher::{
    LAUNCHER_VERSION_LABEL, LauncherLayout, dismiss_launcher, show,
};

#[test]
fn launcher_dismissal_returns_through_canonical_history() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Results);
    crate::workbench::project::open_project_launcher(
        &mut app.state.project,
        &mut app.state.workbench,
        &mut app.state.platform,
    );

    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        crate::workbench::SurfaceId::ProjectLauncher
    );
    assert!(app.state.project.project_launcher_open);

    dismiss_launcher(&mut app);

    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        crate::workbench::SurfaceId::Results
    );
    assert!(!app.state.project.project_launcher_open);
}

#[test]
fn deep_linked_launcher_without_history_dismisses_to_owning_workspace() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .workbench
        .replace_route(
            crate::workbench::SurfaceRoute::surface(crate::workbench::SurfaceId::ProjectLauncher),
            crate::workbench::RouteTransitionSource::BrowserPop,
        )
        .expect("Project Launcher has a canonical executor");
    app.state.project.project_launcher_open = true;

    dismiss_launcher(&mut app);

    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        crate::workbench::SurfaceId::Design
    );
    assert!(!app.state.project.project_launcher_open);
}

#[test]
fn dismissing_with_no_project_open_replaces_the_launcher_with_the_landing() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.clear_browser_history_effects();
    app.state.workbench.activate(Workspace::Results);
    assert!(matches!(
        app.state.workbench.take_browser_history_effect(),
        Some(crate::workbench::BrowserHistoryEffect::Push(_))
    ));
    crate::workbench::project::open_project_launcher(
        &mut app.state.project,
        &mut app.state.workbench,
        &mut app.state.platform,
    );
    assert!(matches!(
        app.state.workbench.take_browser_history_effect(),
        Some(crate::workbench::BrowserHistoryEffect::Push(_))
    ));
    app.state.close_project_session();

    dismiss_launcher(&mut app);

    let destination = crate::workbench::SurfaceRoute::surface(crate::workbench::SurfaceId::Project);
    assert_eq!(app.state.workbench.current_route(), destination);
    assert_eq!(app.state.workbench.workspace, Workspace::Project);
    assert_eq!(
        app.state.workbench.take_browser_history_effect(),
        Some(crate::workbench::BrowserHistoryEffect::Replace(destination))
    );
    assert!(app.state.workbench.take_browser_history_effect().is_none());
    assert!(!app.state.project.project_launcher_open);
}

/// Every string one launcher page paints, with the rect it was painted in.
fn painted_launcher_page(
    size: Vec2,
    safe_mode: bool,
    page: ProjectLauncherPage,
) -> Vec<(String, Rect)> {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    crate::workbench::project::open_project_launcher(
        &mut app.state.project,
        &mut app.state.workbench,
        &mut app.state.platform,
    );
    app.state.project.project_launcher_page = page;
    app.state.model.safe_mode.active = safe_mode;
    let mut output = None;
    for _ in 0..3 {
        output = Some(ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, size)),
                ..Default::default()
            },
            |ui| show(ui.ctx(), &mut app),
        ));
    }
    output
        .expect("three passes ran")
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::epaint::Shape::Text(text) => Some((
                text.galley.text().to_owned(),
                Rect::from_min_size(text.pos, text.galley.size()),
            )),
            _ => None,
        })
        .collect()
}

/// With no project the list area keeps the launcher's size and holds its
/// message and the two ways to a project in its middle, not two lines at
/// the top of an empty shell.
#[test]
fn an_empty_project_list_centres_its_message_and_actions() {
    let size = vec2(1440.0, 900.0);
    let layout = LauncherLayout::resolve(Rect::from_min_size(egui::Pos2::ZERO, size));
    let painted = painted_launcher_page(size, false, ProjectLauncherPage::Projects);
    let rects = |wanted: &str| {
        painted
            .iter()
            .filter(|(text, _)| text == wanted)
            .map(|(_, rect)| *rect)
            .collect::<Vec<_>>()
    };
    let message = rects("No recent projects");
    assert_eq!(message.len(), 1, "{painted:?}");
    let message = message[0];
    let body = layout.surface.y_range();
    assert!(
        message.center().y > body.min + 0.35 * body.span()
            && message.center().y < body.min + 0.65 * body.span(),
        "the message stands at {message:?} in a surface over {body:?}"
    );
    // The heading's actions, and the empty list's under its message.
    for action in ["Browse\u{2026}", "New project"] {
        let under = rects(action)
            .into_iter()
            .filter(|rect| rect.top() > message.bottom())
            .collect::<Vec<_>>();
        assert_eq!(under.len(), 1, "{action} under the message: {painted:?}");
    }
}

#[test]
fn launcher_header_names_the_product_and_states_safe_mode_only_while_active() {
    for size in [vec2(1280.0, 800.0), vec2(390.0, 844.0)] {
        let layout = LauncherLayout::resolve(Rect::from_min_size(egui::Pos2::ZERO, size));
        let header_bottom = layout.surface.top() + layout.header_height;
        for safe_mode in [false, true] {
            let painted = painted_launcher_page(size, safe_mode, ProjectLauncherPage::Projects);
            let context = format!("{}x{}, safe mode {safe_mode}: {painted:?}", size.x, size.y);
            let header: Vec<&str> = painted
                .iter()
                .filter(|(_, rect)| rect.center().y < header_bottom)
                .map(|(text, _)| text.as_str())
                .collect();

            assert!(header.contains(&"RSpice"), "{context}");
            assert_eq!(header.contains(&"Safe mode"), safe_mode, "{context}");
            assert_eq!(
                header.contains(&LAUNCHER_VERSION_LABEL),
                !layout.compact,
                "{context}"
            );
            assert!(
                painted
                    .iter()
                    .any(|(text, rect)| text == "Projects" && rect.center().y > header_bottom),
                "{context}"
            );
            for (text, _) in &painted {
                for retired in [
                    "Continue without a project",
                    "Activation required",
                    "COMMERCIAL",
                    "Start RSpice",
                    "Open engineering work",
                ] {
                    assert!(!text.contains(retired), "{retired:?} painted; {context}");
                }
            }
        }
    }
}

/// The Safe mode page states its launch scope in one line under a plain
/// title, and never offers to start safe mode while it is already on.
#[test]
fn safe_mode_page_heading_is_one_title_and_its_start_is_truthful() {
    for size in [vec2(1280.0, 800.0), vec2(390.0, 844.0)] {
        for active in [false, true] {
            let painted = painted_launcher_page(size, active, ProjectLauncherPage::SafeMode);
            let context = format!("{}x{}, active {active}: {painted:?}", size.x, size.y);
            let shows = |wanted: &str| painted.iter().any(|(text, _)| text == wanted);

            assert!(shows("Changes apply to this launch only."), "{context}");
            assert_eq!(shows("Safe mode active"), active, "{context}");
            assert_eq!(shows("Start RSpice in safe mode"), !active, "{context}");
            for (text, _) in &painted {
                for retired in [
                    "STARTUP ISOLATION",
                    "Start with optional subsystems disabled",
                    "Safe mode is active for this launch",
                    "crash isolation",
                ] {
                    assert!(!text.contains(retired), "{retired:?} painted; {context}");
                }
            }
        }
    }
}
