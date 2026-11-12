//! Deterministic visual regressions for every plan-manager route and gated
//! viewport.
//!
//! The contract tests next door read what the manager paints and how wide it
//! came out; nothing in them sees the result. This runs the real dialog through
//! [`crate::ui::raster`] — headless, software-filled, no GPU, no window, and no
//! dependence on the wasm build.
//!
//! Every route at every gated viewport — the same eight modes and the same
//! three shapes the contract tests gate, taken from
//! [`tests::GATED_VIEWPORTS`](super::tests::GATED_VIEWPORTS) rather than
//! restated, so a render is of the arrangement that was actually asserted
//! about. Each canvas is exactly its viewport: the product rule is that the
//! whole surface fits without scrolling, and a taller canvas would let a
//! surface that overflows the reader's screen look complete here.
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

/// Route-major, then viewport-major, matching `EVERY_ROUTE × GATED_VIEWPORTS`.
pub(crate) const PLAN_MANAGER_FINGERPRINTS: [&str; 24] = [
    "0c9b5df9c7ea43878f19b58222a2962dfb09c49981941d317dd361d5a69dbd5a",
    "54f4db23d37d14f366b06dd9d17774457f3f6613cd1aea4eb4fd10e494bd2b20",
    "2b238132dffaec754d4b79e7578f10ab146d1dda2c2f36722132775998737e2f",
    "cfa24fa39922b3032fa637a2b8be4a854482e3bb29e9da78d75e6587039f9c93",
    "2c527be37fbdbc1942ab9a938d96b53bc36b7bfce7d4131f10856dbf3499c186",
    "aad51a2e34cd9ce1c1d3366d22dead552b973f4425e8e3ae37de5722e6b90001",
    "602f10c2e7a1d961056c936677ca3c99507d2225a3ceb0a14f0a0873686d1d5e",
    "e8a645a5f8f38643f4591dc59e6b181a811e7bdef69157f3cc11abe9cdb25d2e",
    "11fca86814df5f56ef9b0d8abce4b8e41c24d0c69f1f59eba64a23ae923d8a36",
    "7fd0c3514d5a1c8723ce42a9fb76d66a2fc4098bde9de61d4cfdd9af581e510b",
    "566eb6ec94a4e84d925dbf61b7c86d33ba4d2d395ad9df4564c682a0c0056b52",
    "36e1f1f1f18514b569f3f4b9d75ec56d61325326824fbd35d34b062dfea26682",
    "cc30aaa6a627f2f17ebd36d6491c6264445abae4a651a66bbb1a56b1dc9fcfd2",
    "0e13750c755a8d4d2d374af3b08dd830dbf5a181a1d38aac1e335260938ee042",
    "5e86cf69207e58c3ff18336b9911e48ac158df36573c6790e8f6b7999ea0a3a6",
    "33723aba2177a90e34e7c1c53414c5df476d5f36f2a6b0cb6a1b4fa637167b46",
    "04e1d85b9756e39fadfef24746150df0eb3e2c4661d16b5d62783ee49f743df8",
    "bb4866076e01564a9a148fd3eed19d24c0bfb06f3654df03b81281e3fd743b70",
    "dcb13a236012f87caaafd5b223c60c65a98ae5b5d8b203f47ae3438da31759d1",
    "26c23da20bce5500f3137a0fc71c13cb0dc449b05e9e23334c3e37e46a72fcdd",
    "c6bae7ee65ecba5ebcdb944fdc50ebca1439a4ca986375c20827cb5ba644d72e",
    "54a7f6c9cd97bccaef964664dfc5426f4c5f0d95c37ce80b3bceccc71e9ae384",
    "ada1cf4f292f485d9fe7d001bee4c6f5af79e998851a694d6b2e2488475023c2",
    "cdc9c2e9d733905ea42175693f4aa48a3ce613c315325e4af86c6f2023b01fbe",
];

/// A route's name in a file name: its variant, lowercased.
///
/// Taken from `Debug` rather than authored beside the enum, so a renamed
/// variant renames its renders instead of leaving a file named after a mode
/// that no longer exists.
pub(crate) fn file_stem(mode: SimulationPlanManagerMode, viewport: &str) -> String {
    format!(
        "plan-manager-{}-{}",
        format!("{mode:?}").to_ascii_lowercase(),
        viewport.replace(' ', "-")
    )
}

pub(crate) fn baseline_cases()
-> impl Iterator<Item = (usize, SimulationPlanManagerMode, &'static str, Vec2)> {
    super::tests::EVERY_ROUTE
        .into_iter()
        .flat_map(|mode| {
            super::tests::GATED_VIEWPORTS
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
        super::tests::EVERY_ROUTE.len() * super::tests::GATED_VIEWPORTS.len(),
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
