//! The reviewed visual baseline of every Simulation Studio setup page: one
//! fingerprint per page, in the navigation's order.
//!
//! The simulate feature's whole-application tests rasterize each page and
//! compare it with its fingerprint here.

use crate::state::SimulationPage;

#[derive(Clone, Copy)]
pub(crate) struct StudioBaseline {
    pub(crate) name: &'static str,
    pub(crate) page: SimulationPage,
    pub(crate) fingerprint: &'static str,
}

pub(crate) const STUDIO_BASELINES: [StudioBaseline; 9] = [
    StudioBaseline {
        name: "studio-analyses",
        page: SimulationPage::Analyses,
        fingerprint: "11525a810208160953c7b0cc186fb9fd2a9346db366b5217547fdc6f0e8166c9",
    },
    StudioBaseline {
        name: "studio-excitations",
        page: SimulationPage::Excitations,
        fingerprint: "f84375226b79dbd8bed7549779eee7837c155ffcfc447aa233fae84f1ab0f1dc",
    },
    StudioBaseline {
        name: "studio-variables",
        page: SimulationPage::Variables,
        fingerprint: "ecd4668ad35ae83b107c984de336572f04c320bfef762dffd3276f4cff0412ed",
    },
    StudioBaseline {
        name: "studio-outputs",
        page: SimulationPage::Outputs,
        fingerprint: "69210575d312229df9cf8c5a64691e1e11eb62ee4b8e0f5c315f784d96e796a7",
    },
    StudioBaseline {
        name: "studio-specifications",
        page: SimulationPage::Specifications,
        fingerprint: "eda43e5163dc325b52524075dc53b48a8c29c228b76d8cd6ac860cca7b1c6100",
    },
    StudioBaseline {
        name: "studio-run-set",
        page: SimulationPage::RunSet,
        fingerprint: "760f3aebb6e5da01ba74336ba3a51f4a1908b6d06a91f262a03c36845020cbd7",
    },
    StudioBaseline {
        name: "studio-models",
        page: SimulationPage::Models,
        fingerprint: "2c901b6b98338a7da2c72ff6c56ac624a254ca8cc59caead46ebc5f5056d14f7",
    },
    StudioBaseline {
        name: "studio-solver",
        page: SimulationPage::Solver,
        fingerprint: "ba9fb2d2b166e280f8eef76f7a3490d46eacfa6583213cffa6b99b78eb6b9a7c",
    },
    StudioBaseline {
        name: "studio-save",
        page: SimulationPage::Save,
        fingerprint: "71820118ead56a1a4b6fd735f4b247e4f14b887c1312a06bf5bd5c160185ca93",
    },
];

#[test]
fn studio_visual_baselines_cover_every_page_once_and_are_unique() {
    assert_eq!(STUDIO_BASELINES.len(), SimulationPage::NAVIGATION.len());
    assert_eq!(
        STUDIO_BASELINES.map(|baseline| baseline.page),
        SimulationPage::NAVIGATION,
        "the visual baseline table must follow the complete Studio navigation catalog"
    );
    let names = STUDIO_BASELINES
        .iter()
        .map(|baseline| baseline.name)
        .collect::<std::collections::HashSet<_>>();
    let fingerprints = STUDIO_BASELINES
        .iter()
        .map(|baseline| baseline.fingerprint)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(
        names.len(),
        STUDIO_BASELINES.len(),
        "duplicate Studio baseline name"
    );
    assert_eq!(
        fingerprints.len(),
        STUDIO_BASELINES.len(),
        "Studio pages must have independently reviewed fingerprints"
    );
}
