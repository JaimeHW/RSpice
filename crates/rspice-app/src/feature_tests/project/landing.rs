//! The landing page drawn with no project open: what its rows say and what
//! pressing them asks for, at every supported width.

use egui::{Context, Pos2, Rect, Vec2, vec2};

use crate::workbench::RSpiceApp;
use crate::workbench::features;
use crate::workbench::shell_host::project::landing::execute_action;
use rspice_app_project::workbench::surfaces::project::landing::{LandingAction, landing};
use rspice_app_project::workbench::tools::project_launcher::recent_rows::RecentRowAction;
use rspice_app_services::services::recent_files::{RecentFile, RecentKind};

const SIZES: [Vec2; 3] = [vec2(1440.0, 900.0), vec2(820.0, 1180.0), vec2(390.0, 844.0)];

/// An app with no project open and two recent projects on its list.
fn landing_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.close_project_session();
    app.state.platform.recovery_catalog = Default::default();
    let recent = |path: &str, pinned| RecentFile {
        kind: RecentKind::Project,
        path: std::path::PathBuf::from(path),
        opened_at_unix_ms: 1_760_000_000_000,
        pinned,
        missing: false,
    };
    app.state.platform.recent_files = vec![
        recent("C:/Engineering/Analog/afe.rspiceproj", false),
        recent("D:/Lab/rf_pa.rspiceproj", true),
    ];
    app
}

/// One landing frame: what it asked for, the strings it painted with their
/// rects, and the bounds of every control by its accessible label.
struct Frame {
    action: Option<LandingAction>,
    painted: Vec<(String, Rect)>,
    controls: Vec<(String, Rect)>,
}

impl Frame {
    fn shows(&self, wanted: &str) -> bool {
        self.painted.iter().any(|(text, _)| text == wanted)
    }

    fn control(&self, label: &str) -> Rect {
        self.controls
            .iter()
            .find(|(name, _)| name == label)
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| panic!("no control {label:?} in {:?}", self.controls))
    }

    fn text(&self, wanted: &str) -> Rect {
        self.painted
            .iter()
            .find(|(text, _)| text == wanted)
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| panic!("{wanted:?} not painted: {:?}", self.painted))
    }
}

fn frame(ctx: &Context, app: &mut RSpiceApp, size: Vec2, events: Vec<egui::Event>) -> Frame {
    let mut action = None;
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
            events,
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                action = landing(ui, &mut features::cx(app));
            });
        },
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
                .iter()
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
    Frame {
        action,
        painted,
        controls,
    }
}

/// Settle the landing, then press and release at `at`: the action the
/// release asked for.
fn click(ctx: &Context, app: &mut RSpiceApp, size: Vec2, at: Pos2) -> Option<LandingAction> {
    frame(ctx, app, size, vec![egui::Event::PointerMoved(at)]);
    let press = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    frame(ctx, app, size, vec![press(true)]);
    frame(ctx, app, size, vec![press(false)]).action
}

fn settled(app: &mut RSpiceApp, size: Vec2) -> (Context, Frame) {
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    frame(&ctx, app, size, Vec::new());
    let settled = frame(&ctx, app, size, Vec::new());
    (ctx, settled)
}

/// The landing lists the recent projects by name with the date each was
/// last opened, under a one-line heading. The columns nothing fills are
/// gone, and so are the filters nothing writes.
#[test]
fn the_landing_lists_each_recent_project_under_one_heading() {
    for size in SIZES {
        let mut app = landing_app();
        let (_ctx, landing) = settled(&mut app, size);
        let context = format!("{}x{}: {:?}", size.x, size.y, landing.painted);

        assert!(landing.shows("No project open"), "{context}");
        assert!(landing.shows("afe"), "{context}");
        assert!(landing.shows("rf_pa"), "{context}");
        assert!(landing.shows("All") && landing.shows("Pinned"), "{context}");
        for retired in [
            "RSpice Workbench",
            "Nothing is loaded",
            "Owner",
            "Availability",
            "Shared",
            "local",
        ] {
            assert!(
                !landing
                    .painted
                    .iter()
                    .any(|(text, _)| text.contains(retired)),
                "{retired:?} painted; {context}"
            );
        }
        assert!(!app.state.platform.project_lifecycle.is_open());
    }
}

/// Pressing a row asks to open that project; pressing its pin asks to pin
/// it, and the shell then writes the pin.
#[test]
fn a_row_opens_its_project_and_its_pin_writes_the_pin() {
    for size in [vec2(1440.0, 900.0), vec2(390.0, 844.0)] {
        let mut app = landing_app();
        let (ctx, landing) = settled(&mut app, size);
        let context = format!("{}x{}", size.x, size.y);

        let opened = click(&ctx, &mut app, size, landing.text("afe").center());
        assert_eq!(
            opened,
            Some(LandingAction::OpenRecent(
                app.state.platform.recent_files[0].clone()
            )),
            "{context}"
        );

        let pin = landing.control("Pin afe");
        let pinned = click(&ctx, &mut app, size, pin.center());
        let path = std::path::PathBuf::from("C:/Engineering/Analog/afe.rspiceproj");
        assert_eq!(
            pinned,
            Some(LandingAction::Recent(RecentRowAction::TogglePin(
                path.clone()
            ))),
            "{context}"
        );
        execute_action(&mut app, pinned.expect("pin pressed"));
        let afe = app
            .state
            .platform
            .recent_files
            .iter()
            .find(|recent| recent.path == path)
            .expect("still listed");
        assert!(afe.pinned, "{context}");
    }
}

/// Opening a recent project whose file is gone keeps its row, marks it
/// missing and says so in a notice. The row then offers Locate… and Remove,
/// and Remove takes it off the list.
#[test]
fn a_missing_recent_project_stays_listed_with_locate_and_remove() {
    let size = vec2(1440.0, 900.0);
    let mut app = landing_app();
    let gone = std::env::temp_dir().join("rspice-landing-gone/afe.rspiceproj");
    app.state.platform.recent_files[0].path = gone.clone();
    let (ctx, landing) = settled(&mut app, size);

    let opened = click(&ctx, &mut app, size, landing.text("afe").center());
    execute_action(&mut app, opened.expect("row pressed"));

    let afe = &app.state.platform.recent_files[0];
    assert_eq!(afe.path, gone);
    assert!(afe.missing);
    let notices = app.state.model.notices.take();
    assert!(
        notices
            .iter()
            .any(|notice| notice.title == "Could not open afe"),
        "{notices:?}"
    );
    assert!(!app.state.platform.project_lifecycle.is_open());

    let marked = frame(&ctx, &mut app, size, Vec::new());
    assert!(marked.shows("missing"), "{:?}", marked.painted);
    assert!(marked.shows("Locate\u{2026}"), "{:?}", marked.painted);
    let remove = marked.text("Remove").center();
    let removed = click(&ctx, &mut app, size, remove);
    assert_eq!(
        removed,
        Some(LandingAction::Recent(RecentRowAction::Remove(gone.clone())))
    );
    execute_action(&mut app, removed.expect("remove pressed"));
    assert!(
        !app.state
            .platform
            .recent_files
            .iter()
            .any(|recent| recent.path == gone)
    );
}

/// Locate… points the row at where the file went and keeps its pin.
#[test]
fn a_located_recent_project_keeps_its_place_and_pin() {
    let mut app = landing_app();
    let from = std::path::PathBuf::from("D:/Lab/rf_pa.rspiceproj");
    let to = std::path::PathBuf::from("E:/Moved/rf_pa.rspiceproj");
    app.state.platform.recent_files[1].missing = true;

    let located = app
        .locate_recent_file(&from, &to)
        .expect("the row is listed");

    assert_eq!(located.path, to);
    assert!(located.pinned);
    assert!(!located.missing);
    assert_eq!(app.state.platform.recent_files[1].path, to);
}
