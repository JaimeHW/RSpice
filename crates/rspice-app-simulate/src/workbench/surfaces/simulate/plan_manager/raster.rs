//! Deterministic visual regressions for every plan-manager route and gated
//! viewport.
//!
//! The contract tests next door read what the manager paints and how wide it
//! came out; nothing in them sees the result. This runs the real dialog through
//! [`crate::ui::raster`] — headless, software-filled, no GPU, no window, and no
//! dependence on the wasm build.
//!
//! Every route at every gated viewport — the same eight modes and the same
//! three shapes the contract tests gate, held here as [`EVERY_ROUTE`] and
//! [`GATED_VIEWPORTS`] and read by both rather than restated, so a render is
//! of the arrangement that was actually asserted about. Each canvas is
//! exactly its viewport: the product rule is that the whole surface fits
//! without scrolling, and a taller canvas would let a surface that overflows
//! the reader's screen look complete here.
//!
//! Read the renders for layout, not for wording — the rasterizer samples the
//! font atlas without filtering, and its own header says what that costs. A
//! glyph read off one of these images is not a defect report; check the string
//! in `tests.rs`, which reads the galley rather than the pixels.
//!
//! The tests compare reviewed fingerprints and never write files.

// Native-only, like the render harnesses in `tests.rs` it shares fixtures with:
// it rasterizes to a file for a person to open.
#![cfg(not(target_arch = "wasm32"))]

use egui::Vec2;

use crate::workbench::state::SimulationPlanManagerMode;

/// James's real viewport. Display scaling makes the usable area smaller than
/// a naive test window, and the product rule is that a dialog fits without
/// scrolling — so this, not a comfortable size, is what fit means here.
pub const REAL_VIEWPORT: Vec2 = egui::Vec2::new(1024.0, 640.0);

/// The three viewports every route is gated at.
///
/// 1024x640 is the real desktop one — display scaling makes the usable area
/// smaller than a comfortable test window. 820 is where `WideWorkflow` stops
/// being a fixed-width panel and becomes the whole viewport, and 560x900 is the
/// portrait shape a global command can open this dialog into. The names are the
/// ones the contract tests' two browse gates already use for the same three
/// shapes.
pub const GATED_VIEWPORTS: [(&str, Vec2); 3] = [
    ("desktop", REAL_VIEWPORT),
    ("full-viewport landscape", egui::Vec2::new(820.0, 640.0)),
    ("portrait", egui::Vec2::new(560.0, 900.0)),
];

/// Every route the manager dispatches to, in dispatch order; the contract
/// tests' `route_position` keeps it complete.
pub const EVERY_ROUTE: [SimulationPlanManagerMode; 8] = [
    SimulationPlanManagerMode::Browse,
    SimulationPlanManagerMode::Create,
    SimulationPlanManagerMode::Rename,
    SimulationPlanManagerMode::ConfirmArchive,
    SimulationPlanManagerMode::Compare,
    SimulationPlanManagerMode::Export,
    SimulationPlanManagerMode::Import,
    SimulationPlanManagerMode::Campaign,
];

/// Every source file of the plan manager, which the elision gate reads to tell
/// a string the module wrote with an ellipsis from one elided to fit.
pub const PLAN_MANAGER_SOURCES: &[(&str, &str)] = &[
    ("plan_manager.rs", include_str!("../plan_manager.rs")),
    ("tests.rs", include_str!("tests.rs")),
    ("campaign.rs", include_str!("campaign.rs")),
    ("compare.rs", include_str!("compare.rs")),
    ("create.rs", include_str!("create.rs")),
    ("exchange.rs", include_str!("exchange.rs")),
    ("kit.rs", include_str!("kit.rs")),
    ("lifecycle.rs", include_str!("lifecycle.rs")),
    ("records.rs", include_str!("records.rs")),
];

/// Route-major, then viewport-major, matching `EVERY_ROUTE × GATED_VIEWPORTS`.
pub const PLAN_MANAGER_FINGERPRINTS: [&str; 24] = [
    "971e9114163e82e3cbdc847975376905a9c5f06ad38933eb6df9e990ccc6cbe8",
    "300542b8ae0754d63447c64af248b4f2818dc440ab10acfe904583f08ac5600d",
    "3ec5bb9630edcfcff1f457791762b797e36e192b98f6d8ca162ccb7b093ef800",
    "cfa24fa39922b3032fa637a2b8be4a854482e3bb29e9da78d75e6587039f9c93",
    "2c527be37fbdbc1942ab9a938d96b53bc36b7bfce7d4131f10856dbf3499c186",
    "aad51a2e34cd9ce1c1d3366d22dead552b973f4425e8e3ae37de5722e6b90001",
    "602f10c2e7a1d961056c936677ca3c99507d2225a3ceb0a14f0a0873686d1d5e",
    "e8a645a5f8f38643f4591dc59e6b181a811e7bdef69157f3cc11abe9cdb25d2e",
    "11fca86814df5f56ef9b0d8abce4b8e41c24d0c69f1f59eba64a23ae923d8a36",
    "7fd0c3514d5a1c8723ce42a9fb76d66a2fc4098bde9de61d4cfdd9af581e510b",
    "566eb6ec94a4e84d925dbf61b7c86d33ba4d2d395ad9df4564c682a0c0056b52",
    "36e1f1f1f18514b569f3f4b9d75ec56d61325326824fbd35d34b062dfea26682",
    "a28b24b2618b21ce8868f9605ce2ab37e329792bddd3fc92e421b767b1b408dc",
    "b8334333a6f25aa26d5044f796770ea74a739f93b9227335be733cfb4979698b",
    "db7a10195b0d67c9ab8b75579303946e574805a787a4632f43b0c3a86fe4c3b8",
    "33723aba2177a90e34e7c1c53414c5df476d5f36f2a6b0cb6a1b4fa637167b46",
    "04e1d85b9756e39fadfef24746150df0eb3e2c4661d16b5d62783ee49f743df8",
    "bb4866076e01564a9a148fd3eed19d24c0bfb06f3654df03b81281e3fd743b70",
    "99f0846967185dad5f93285c1b8349510f75d7c09909a6c7e432f251fd7dbffe",
    "a10b59cebbaefb720cd0dd6a027d61b05ca4a144a99590cdae0e67e051ba69ac",
    "79da1e075a733ae74655d349545d16a3e99018fde10ac186375aa6c06417c311",
    "6a286285e54f9d9941ccaf29c03edc14208672a4f76163f7411d18a63531aa74",
    "5161b2a06e9115a951d7f589e6ce00e6957d6a74314e033dcd7346f72928b76a",
    "efb5e9a1900742c19c66113f919f9ff1c8b6dbc4157300258012e5fd7b5aed13",
];

/// A route's name in a file name: its variant, lowercased.
///
/// Taken from `Debug` rather than authored beside the enum, so a renamed
/// variant renames its renders instead of leaving a file named after a mode
/// that no longer exists.
pub fn file_stem(mode: SimulationPlanManagerMode, viewport: &str) -> String {
    format!(
        "plan-manager-{}-{}",
        format!("{mode:?}").to_ascii_lowercase(),
        viewport.replace(' ', "-")
    )
}

pub fn baseline_cases()
-> impl Iterator<Item = (usize, SimulationPlanManagerMode, &'static str, Vec2)> {
    EVERY_ROUTE
        .into_iter()
        .flat_map(|mode| {
            GATED_VIEWPORTS
                .into_iter()
                .map(move |(viewport, screen)| (mode, viewport, screen))
        })
        .enumerate()
        .map(|(index, (mode, viewport, screen))| (index, mode, viewport, screen))
}

#[test]
fn plan_manager_visual_baselines_cover_the_route_viewport_product_and_are_unique() {
    assert_eq!(
        PLAN_MANAGER_FINGERPRINTS.len(),
        EVERY_ROUTE.len() * GATED_VIEWPORTS.len(),
        "the visual baseline table must cover every route at every gated viewport"
    );
    let names = baseline_cases()
        .map(|(_, mode, viewport, _)| file_stem(mode, viewport))
        .collect::<std::collections::HashSet<_>>();
    let fingerprints = PLAN_MANAGER_FINGERPRINTS
        .into_iter()
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(
        names.len(),
        PLAN_MANAGER_FINGERPRINTS.len(),
        "duplicate plan-manager visual baseline identity"
    );
    assert_eq!(
        fingerprints.len(),
        PLAN_MANAGER_FINGERPRINTS.len(),
        "each route/viewport pair must have an independently reviewed fingerprint"
    );
}
