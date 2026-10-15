//! Deterministic visual regressions for the notification panel and its toasts.
//!
//! The contract tests next door read what the panel paints and press what it
//! announces; nothing in them sees the result. This runs the real panel under
//! the real title bar through [`crate::ui::raster`] — headless, software
//! filled, no GPU, no window — so the bell it hangs from, the caret that
//! points at it and the corner the toasts land in are the product's own.
//!
//! Read the renders for layout, not for wording: the rasterizer samples the
//! font atlas unfiltered, and its own header says what that costs.
//!
//! The default test compares reviewed fingerprints and never writes files.
//! The ignored review tools print replacement fingerprints or write PNGs to
//! `RSPICE_RASTER_DIR` (default: the system temp directory).

#![cfg(not(target_arch = "wasm32"))]

use egui::{Pos2, Vec2, pos2, vec2};

use crate::ui::raster::Canvas;
use crate::ui::tokens::Mode;
use crate::workbench::RSpiceApp;
use crate::workbench::layout::LayoutSpec;

/// What is on screen when the render is taken.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Scene {
    /// The seeded session with the panel open.
    Panel,
    /// The same, with the pointer on the first row.
    PanelHover,
    /// A session in which nothing has happened.
    PanelEmpty,
    /// The panel closed, and the seeded session's notices still arriving.
    Toasts,
}

struct Case {
    stem: &'static str,
    scene: Scene,
    mode: Mode,
    size: Vec2,
}

const CASES: [Case; 7] = [
    Case {
        stem: "notifications-panel-desktop",
        scene: Scene::Panel,
        mode: Mode::Dark,
        size: vec2(1280.0, 760.0),
    },
    Case {
        stem: "notifications-panel-tablet",
        scene: Scene::Panel,
        mode: Mode::Dark,
        size: vec2(820.0, 640.0),
    },
    Case {
        stem: "notifications-panel-phone",
        scene: Scene::Panel,
        mode: Mode::Dark,
        size: vec2(390.0, 780.0),
    },
    Case {
        stem: "notifications-panel-desktop-light",
        scene: Scene::Panel,
        mode: Mode::Light,
        size: vec2(1280.0, 760.0),
    },
    Case {
        stem: "notifications-panel-desktop-hover",
        scene: Scene::PanelHover,
        mode: Mode::Dark,
        size: vec2(1280.0, 760.0),
    },
    Case {
        stem: "notifications-panel-desktop-empty",
        scene: Scene::PanelEmpty,
        mode: Mode::Dark,
        size: vec2(1280.0, 760.0),
    },
    Case {
        stem: "notifications-toasts-desktop",
        scene: Scene::Toasts,
        mode: Mode::Dark,
        size: vec2(1280.0, 760.0),
    },
];

/// Index-aligned with [`CASES`].
const FINGERPRINTS: [&str; 7] = [
    "f0aa67652644d86b044f879837db87a5aca210c042c9afdc5dce852b89e261a4",
    "132ac865a2f1e161029fd985cc145ba883eb1e3ad06efe2907fdef79c5543fbc",
    "02718f260182c6dcafca85c016f1e434f668692e7b7963c69494629700530ca1",
    "174f5428505f98a39cd3060f18d31f7781075efbc1785f672ef089167fe2e6e1",
    "ac17c80e65d89f69552024cc3112d5a1d757babe97f1ef3bef094a1d32843184",
    "727cbac096615fe41589f18b3bc911cd246f6dc74006369023ba5baec183c971",
    "c0368eefeb9026cdc7395dae8ad6d32668bdd9d9c99b20f961b7d8edfbcedee1",
];

/// Where the first row sits under the desktop bell: inside the panel's list,
/// clear of the head and the filters above it.
const FIRST_ROW: Pos2 = pos2(1080.0, 190.0);

fn raster(case: &Case) -> Canvas {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.notification_center_open = case.scene != Scene::Toasts;
    let mut seeded = case.scene == Scene::PanelEmpty;
    let size = case.size;
    let scene = case.scene;

    let pass = move |ui: &mut egui::Ui, background: egui::Color32| {
        // An entrance half-played is a render of a moment nobody reviews. With
        // motion off the panel and the toasts arrive whole, as they do for a
        // reader who asked for reduced motion.
        ui.ctx()
            .global_style_mut(|style| style.animation_time = 0.0);
        if !seeded {
            super::tests::seed_session(ui.ctx(), &mut app.state.ui.toasts);
            seeded = true;
        }
        let layout = LayoutSpec::resolve(size.x, size.y, &app.state.workbench);
        crate::workbench::chrome::title_bar::show(ui, &mut app, layout);
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(background))
            .show(ui, |_| {});
        super::show(ui.ctx(), &mut app, layout.title_bar_height);
        if scene == Scene::Toasts {
            let large = super::large_targets(ui.ctx(), &app.state);
            let _ = app.state.ui.toasts.show(
                ui.ctx(),
                layout.title_bar_height,
                layout.toolbar_height,
                large,
            );
        }
    };
    let theme = crate::ui::Theme {
        mode: case.mode,
        ..crate::ui::Theme::default()
    };
    if scene == Scene::PanelHover {
        crate::ui::raster::render_with_pointer(size, FIRST_ROW, pass)
    } else {
        crate::ui::raster::render_themed(theme, size, pass)
    }
}

fn regression_height(canvas: &Canvas) -> usize {
    canvas.content_height().max(1)
}

#[test]
fn notification_visual_baselines_are_index_aligned_and_distinct() {
    let stems = CASES
        .iter()
        .map(|case| case.stem)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(stems.len(), CASES.len(), "duplicate render identity");
    let fingerprints = FINGERPRINTS
        .into_iter()
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(
        fingerprints.len(),
        FINGERPRINTS.len(),
        "each scene must have an independently reviewed fingerprint"
    );
}

#[test]
fn the_notification_surfaces_match_their_reviewed_baselines() {
    for (case, fingerprint) in CASES.iter().zip(FINGERPRINTS) {
        let canvas = raster(case);
        canvas.assert_regression(case.stem, regression_height(&canvas), fingerprint);
    }
}

#[test]
#[ignore = "prints source-ready visual fingerprints after explicit review"]
fn print_notification_visual_fingerprints_for_review() {
    use std::io::Write as _;

    let mut stdout = std::io::stdout().lock();
    for case in &CASES {
        let canvas = raster(case);
        let height = regression_height(&canvas);
        writeln!(
            stdout,
            "{} {}x{} {}",
            case.stem,
            canvas.width(),
            height,
            canvas.regression_fingerprint(height)
        )
        .expect("write visual fingerprint");
    }
}

/// Write every scene to a PNG so the design can be looked at.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_every_notification_scene() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();
    for case in &CASES {
        let canvas = raster(case);
        let height = regression_height(&canvas);
        let path = directory.join(format!("{}.png", case.stem));
        std::fs::write(&path, canvas.png(height)).expect("write notification render");
        writeln!(report, "{} {}x{}", path.display(), canvas.width(), height)
            .expect("write raster report");
    }
}
