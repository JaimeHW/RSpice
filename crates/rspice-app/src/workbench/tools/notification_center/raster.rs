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
//! The test compares reviewed fingerprints and never writes files.

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
    "3c4543ddf3df9c40c77dd894acc1e0eaf9039bb7444249e581edd63127871f8d",
    "875b393b42bd80c3444218ce3cf2e2745496531e68662cadfe2cce598c34a0ce",
    "966cd532fefcdcda849a8e20e2fd7ea5b59fe4cb60d3059a9bce7dcc4a6038c3",
    "5083b78010945add3eeb66230ab62d3ff5a615c0c9bc16a789c4d87af7047d2f",
    "ff5aafe25fab968698bbcb1cd008d2090e6ed8cdee74d3302108424f352fb741",
    "07913ccfd9e88a89fd3b35ccbe610e2863e5ebd21b58944768a7dd2a8f316a6c",
    "15a2973b8619110dfc681e6df0289ed3c8c8f3bf5b87836d54bc8fb266c0d3ba",
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
            super::tests::seed_session(ui.ctx(), &mut app.state.workbench.toasts);
            seeded = true;
        }
        let layout = LayoutSpec::resolve(size.x, size.y, &app.state.workbench);
        crate::workbench::chrome::title_bar::show(ui, &mut app, layout);
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(background))
            .show(ui, |_| {});
        super::show(ui.ctx(), &mut app, layout.title_bar_height);
        if scene == Scene::Toasts {
            let large = super::large_targets(ui.ctx(), &app.state.workbench);
            let _ = app.state.workbench.toasts.show(
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
    // The earlier rows tell the time of day, so the clock and its zone are
    // pinned: a render reviewed in one zone is the render every machine draws.
    super::tests::with_pinned_clock(|| {
        if scene == Scene::PanelHover {
            crate::ui::raster::render_with_pointer(size, FIRST_ROW, pass)
        } else {
            crate::ui::raster::render_themed(theme, size, pass)
        }
    })
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
