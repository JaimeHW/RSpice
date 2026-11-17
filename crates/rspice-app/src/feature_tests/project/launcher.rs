//! The project launcher through the whole application: its page painted with
//! and without projects and in safe mode, and a dismissal returning where
//! the route history says.

use egui::{Context, Rect, Vec2, vec2};

use crate::workbench::RSpiceApp;
use crate::workbench::session::project_transactions::{SafeModeStart, start_local_safe_mode};
use crate::workbench::shell_host::project::launcher::dismiss_launcher;
use crate::workbench::shell_host::project::launcher::show;
use rspice_app_project::workbench::project::workspace_state::ProjectLauncherPage;
use rspice_app_project::workbench::tools::project_launcher::{
    LAUNCHER_VERSION_LABEL, LauncherLayout,
};
use rspice_app_state::workbench::workspace::Workspace;

#[test]
fn launcher_dismissal_returns_through_canonical_history() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Results);
    rspice_app_project::workbench::project::open_project_launcher(
        &mut app.state.project,
        &mut app.state.workbench,
        &mut app.state.platform,
    );

    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId::ProjectLauncher
    );
    assert!(app.state.project.project_launcher_open);

    dismiss_launcher(&mut app);

    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId::Results
    );
    assert!(!app.state.project.project_launcher_open);
}

#[test]
fn deep_linked_launcher_without_history_dismisses_to_owning_workspace() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .workbench
        .replace_route(
            rspice_app_workbench::workbench::routing::surface_route::SurfaceRoute::surface(rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId::ProjectLauncher),
            rspice_app_workbench::workbench::routing::navigation::RouteTransitionSource::BrowserPop,
        )
        .expect("Project Launcher has a canonical executor");
    app.state.project.project_launcher_open = true;

    dismiss_launcher(&mut app);

    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId::Design
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
        Some(rspice_app_workbench::workbench::routing::navigation::BrowserHistoryEffect::Push(_))
    ));
    rspice_app_project::workbench::project::open_project_launcher(
        &mut app.state.project,
        &mut app.state.workbench,
        &mut app.state.platform,
    );
    assert!(matches!(
        app.state.workbench.take_browser_history_effect(),
        Some(rspice_app_workbench::workbench::routing::navigation::BrowserHistoryEffect::Push(_))
    ));
    app.state.close_project_session();

    dismiss_launcher(&mut app);

    let destination =
        rspice_app_workbench::workbench::routing::surface_route::SurfaceRoute::surface(
            rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId::Project,
        );
    assert_eq!(app.state.workbench.current_route(), destination);
    assert_eq!(app.state.workbench.workspace, Workspace::Project);
    assert_eq!(
        app.state.workbench.take_browser_history_effect(),
        Some(
            rspice_app_workbench::workbench::routing::navigation::BrowserHistoryEffect::Replace(
                destination
            )
        )
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
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    rspice_app_project::workbench::project::open_project_launcher(
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

/// Starting safe mode over a project with unsaved work asks first, with the
/// close review Close project uses, and keeps the project open until the
/// review answers. Cancel leaves everything as it was; Don't save starts
/// safe mode on the no-project landing.
#[test]
fn safe_mode_over_unsaved_work_goes_through_the_close_review() {
    use rspice_app_services::workbench::lifecycle::project_lifecycle::ProjectCloseDestination;
    use rspice_app_state::state::write_policy::LocalSafeModeOptions;
    use rspice_app_workbench::workbench::app::dialogs::confirmation_state::ProjectReviewRequest;

    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .workspace
        .content
        .project
        .rename("edited")
        .expect("valid name");
    rspice_app_project::workbench::project::open_project_launcher(
        &mut app.state.project,
        &mut app.state.workbench,
        &mut app.state.platform,
    );
    let options = LocalSafeModeOptions::default();

    assert_eq!(
        start_local_safe_mode(&mut app.state, options),
        Ok(SafeModeStart::AwaitingReview)
    );
    let Some(ProjectReviewRequest::CloseProject(destination)) =
        app.state.workbench.project_review_dialog.request.clone()
    else {
        panic!("the close review is asked for");
    };
    assert_eq!(destination, ProjectCloseDestination::SafeMode(options));
    assert!(!app.state.model.safe_mode.active);
    assert!(app.state.platform.project_lifecycle.is_open());
    assert_eq!(app.state.model.workspace.content.project.name(), "edited");

    // Cancel: the review closes and nothing else moves.
    app.state.workbench.project_review_dialog.close();
    assert!(!app.state.model.safe_mode.active);
    assert!(app.state.platform.project_lifecycle.is_open());

    // Don't save: the review's discard starts safe mode.
    assert_eq!(
        start_local_safe_mode(&mut app.state, options),
        Ok(SafeModeStart::AwaitingReview)
    );
    app.state.workbench.project_review_dialog.close();
    assert!(app.state.close_project_discard(destination));
    assert!(app.state.model.safe_mode.active);
    assert!(!app.state.platform.project_lifecycle.is_open());
    assert_eq!(app.state.workbench.workspace, Workspace::Project);
    assert!(!app.state.project.project_launcher_open);
}

/// The Recovery page's way to Safe mode says where it goes.
#[test]
fn the_recovery_page_names_its_way_to_safe_mode() {
    let painted = painted_launcher_page(vec2(1280.0, 800.0), false, ProjectLauncherPage::Recovery);
    assert!(
        painted.iter().any(|(text, _)| text == "Safe mode\u{2026}"),
        "{painted:?}"
    );
    assert!(
        !painted.iter().any(|(text, _)| text == "Recovery options"),
        "{painted:?}"
    );
}

/// A project with nothing unsaved closes and safe mode starts at once.
#[test]
fn safe_mode_over_a_clean_project_starts_at_once() {
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.closing_loses_nothing());

    assert_eq!(
        start_local_safe_mode(
            &mut app.state,
            rspice_app_state::state::write_policy::LocalSafeModeOptions::default()
        ),
        Ok(SafeModeStart::Started)
    );
    assert!(app.state.model.safe_mode.active);
    assert!(app.state.workbench.project_review_dialog.request.is_none());
    assert!(!app.state.platform.project_lifecycle.is_open());
}

/// Texts with the rects they stand in.
#[cfg(not(target_arch = "wasm32"))]
type Placed = Vec<(String, Rect)>;

/// One launcher frame through the shell: the strings it painted with their
/// rects, and every control's bounds by its accessible label.
#[cfg(not(target_arch = "wasm32"))]
fn launcher_frame(
    ctx: &Context,
    app: &mut RSpiceApp,
    events: Vec<egui::Event>,
) -> (Placed, Placed) {
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1440.0, 900.0))),
            events,
            ..Default::default()
        },
        |ui| show(ui.ctx(), app),
    );
    let painted = output
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::epaint::Shape::Text(text) => Some((
                text.galley.text().to_owned(),
                Rect::from_min_size(text.pos, text.galley.size()),
            )),
            _ => None,
        })
        .collect();
    let controls = output
        .platform_output
        .accesskit_update
        .map(|update| {
            update
                .nodes
                .into_iter()
                .filter_map(|(_, node)| {
                    let bounds = node.bounds()?;
                    Some((
                        node.label()?.to_owned(),
                        Rect::from_min_max(
                            egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                            egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                        ),
                    ))
                })
                .collect()
        })
        .unwrap_or_default();
    (painted, controls)
}

/// A press at the middle of `rect`.
#[cfg(not(target_arch = "wasm32"))]
fn press(rect: Rect) -> Vec<egui::Event> {
    let at = rect.center();
    vec![
        egui::Event::PointerMoved(at),
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::default(),
        },
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        },
    ]
}

/// Opening a recent project whose file is gone keeps the launcher open on
/// its row, marked missing, with Locate... and Remove under it; Remove takes
/// it off the list. Every row's date stands in one column, however short
/// its path.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn a_missing_recent_project_keeps_the_launcher_open_on_its_row() {
    use rspice_app_services::services::recent_files::{RecentFile, RecentKind};

    let root = rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir(
        "launcher-missing-recent",
    );
    let present = root
        .join("a much longer folder name than the other")
        .join("bandgap.rspiceproj");
    std::fs::create_dir_all(present.parent().expect("a folder")).expect("folder");
    std::fs::write(&present, b"{}").expect("a project file");
    let gone = root.join("ldo.rspiceproj");
    let recent = |path: &std::path::Path| RecentFile {
        kind: RecentKind::Project,
        path: path.to_path_buf(),
        opened_at_unix_ms: 1_760_000_000_000,
        pinned: false,
        missing: false,
    };
    let mut app = RSpiceApp::test_instance();
    app.state.platform.recent_files = vec![recent(&present), recent(&gone)];
    rspice_app_project::workbench::project::open_project_launcher(
        &mut app.state.project,
        &mut app.state.workbench,
        &mut app.state.platform,
    );
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let control = |controls: &[(String, Rect)], label: &str| {
        controls
            .iter()
            .find(|(name, _)| name == label)
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| panic!("no control {label:?} in {controls:?}"))
    };

    let mut frame = launcher_frame(&ctx, &mut app, Vec::new());
    for _ in 0..2 {
        frame = launcher_frame(&ctx, &mut app, Vec::new());
    }
    let row = control(&frame.1, "Open project ldo");
    launcher_frame(&ctx, &mut app, press(row));
    let (painted, controls) = launcher_frame(&ctx, &mut app, Vec::new());

    assert!(app.state.project.project_launcher_open);
    assert!(app.state.platform.recent_files[1].missing);
    let notices = app.state.model.notices.take();
    assert!(
        notices
            .iter()
            .any(|notice| notice.title == "Could not open ldo"),
        "{notices:?}"
    );
    let shows = |wanted: &str| painted.iter().any(|(text, _)| text == wanted);
    assert!(shows("missing"), "{painted:?}");
    let dates = painted
        .iter()
        .filter(|(text, _)| text.starts_with("20") && text.len() == 16)
        .map(|(_, rect)| rect.left())
        .collect::<Vec<_>>();
    assert_eq!(dates.len(), 2, "{painted:?}");
    assert!((dates[0] - dates[1]).abs() < 0.5, "{dates:?}");

    let remove = control(&controls, "Remove");
    let _ = control(&controls, "Locate\u{2026}");
    launcher_frame(&ctx, &mut app, press(remove));
    assert_eq!(app.state.platform.recent_files.len(), 1);
    assert_eq!(app.state.platform.recent_files[0].path, present);
    assert!(app.state.project.project_launcher_open);
    let _ = std::fs::remove_dir_all(root);
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
            assert!(shows("Do not reopen prior documents"), "{context}");
            for (text, _) in &painted {
                for retired in [
                    "STARTUP ISOLATION",
                    "Start with optional subsystems disabled",
                    "Safe mode is active for this launch",
                    "crash isolation",
                    // Options nothing enforces are not offered.
                    "Disable third-party extensions",
                    "Disable GPU acceleration",
                ] {
                    assert!(!text.contains(retired), "{retired:?} painted; {context}");
                }
            }
        }
    }
}

/// What the project's open, save, close, recovery and revision history say
/// is in the reader's words: none of their messages speaks of a canonical
/// file, an authority, a journal, a baseline or a working tree.
#[test]
fn lifecycle_messages_use_the_readers_words() {
    const SOURCES: [(&str, &str); 10] = [
        (
            "rspice-project/src/lifecycle.rs",
            include_str!("../../../../rspice-project/src/lifecycle.rs"),
        ),
        (
            "rspice-project/src/lifecycle/save.rs",
            include_str!("../../../../rspice-project/src/lifecycle/save.rs"),
        ),
        (
            "rspice-project/src/lifecycle/dirty.rs",
            include_str!("../../../../rspice-project/src/lifecycle/dirty.rs"),
        ),
        (
            "revision_history.rs",
            include_str!(
                "../../../../rspice-app-project/src/workbench/project/revision_history.rs"
            ),
        ),
        (
            "autosave.rs",
            include_str!(
                "../../../../rspice-app-services/src/workbench/app_state/session/autosave.rs"
            ),
        ),
        (
            "lifecycle/recovery.rs",
            include_str!("../../../../rspice-app-services/src/workbench/lifecycle/recovery.rs"),
        ),
        (
            "project_launcher.rs",
            include_str!("../../../../rspice-app-project/src/workbench/tools/project_launcher.rs"),
        ),
        (
            "project_launcher/recovery.rs",
            include_str!(
                "../../../../rspice-app-project/src/workbench/tools/project_launcher/recovery.rs"
            ),
        ),
        (
            "project_launcher/recent_rows.rs",
            include_str!(
                "../../../../rspice-app-project/src/workbench/tools/project_launcher/recent_rows.rs"
            ),
        ),
        (
            "surfaces/project/landing.rs",
            include_str!(
                "../../../../rspice-app-project/src/workbench/surfaces/project/landing.rs"
            ),
        ),
    ];
    const WORDS: [&str; 5] = [
        "canonical",
        "authority",
        "journal",
        "baseline",
        "working tree",
    ];

    let mut found = Vec::new();
    for (name, source) in SOURCES {
        let source = source.replace("\r\n", "\n");
        let shipped = source
            .split("\n#[cfg(test)]\nmod tests")
            .next()
            .unwrap_or_default();
        for (number, line) in shipped.lines().enumerate() {
            if line.trim_start().starts_with("//") {
                continue;
            }
            // A message is a string literal with a space in it; a key or an
            // identifier has none.
            for literal in string_literals(line).filter(|text| text.contains(' ')) {
                let lower = literal.to_lowercase();
                for word in WORDS.iter().filter(|word| lower.contains(**word)) {
                    found.push(format!("{name}:{}: {word}: {literal}", number + 1));
                }
            }
        }
    }
    assert!(found.is_empty(), "{}", found.join("\n"));
}

/// The string literals on one line of source, escapes left as written.
fn string_literals(line: &str) -> impl Iterator<Item = &str> {
    let bytes = line.as_bytes();
    let mut at = 0;
    std::iter::from_fn(move || {
        let start = at + line.get(at..)?.find('"')? + 1;
        let mut end = start;
        while end < bytes.len() && bytes[end] != b'"' {
            end += if bytes[end] == b'\\' { 2 } else { 1 };
        }
        at = (end + 1).min(bytes.len());
        line.get(start..end.min(bytes.len()))
    })
}
