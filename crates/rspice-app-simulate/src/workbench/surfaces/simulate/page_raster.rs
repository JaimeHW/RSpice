//! The reviewed visual baseline of every Simulation Studio setup page: one
//! fingerprint per page, in the navigation's order.
//!
//! The simulate feature's whole-application tests rasterize each page and
//! compare it with its fingerprint here.

use crate::state::SimulationPage;

#[derive(Clone, Copy)]
pub struct StudioBaseline {
    pub name: &'static str,
    pub page: SimulationPage,
    pub fingerprint: &'static str,
}

pub const STUDIO_BASELINES: [StudioBaseline; 9] = [
    StudioBaseline {
        name: "studio-analyses",
        page: SimulationPage::Analyses,
        fingerprint: "68ca5bc00a5de34276433908f9bcda6abcbcad1b32d47e340c147c154074d044",
    },
    StudioBaseline {
        name: "studio-excitations",
        page: SimulationPage::Excitations,
        fingerprint: "9f35eb70395dd23d40520c92e7094110f65176b3d07b986622973a7e13fb8436",
    },
    StudioBaseline {
        name: "studio-variables",
        page: SimulationPage::Variables,
        fingerprint: "b8e8cbcd1adffa1b57c800e8f069118844091a4a864501afcd710025123891b5",
    },
    StudioBaseline {
        name: "studio-outputs",
        page: SimulationPage::Outputs,
        fingerprint: "7727faa5b616929fd0f9eadb1975c263bbb8eab74c89344c5658e7bfe93b2696",
    },
    StudioBaseline {
        name: "studio-specifications",
        page: SimulationPage::Specifications,
        fingerprint: "6b5c7511be459de36fe19331209a7bef37b33e1b2878d376bb6324f41fba74f5",
    },
    StudioBaseline {
        name: "studio-run-set",
        page: SimulationPage::RunSet,
        fingerprint: "e48c546aa22615783d3c728dcd0d54330e38f28098bfc3567192931f277b6588",
    },
    StudioBaseline {
        name: "studio-models",
        page: SimulationPage::Models,
        fingerprint: "a12633e9281a251972c64ef6e998319fca1660e03b21158419c3a0da9f34496c",
    },
    StudioBaseline {
        name: "studio-solver",
        page: SimulationPage::Solver,
        fingerprint: "0965c98875e854f630c5698a9abb7570c6e3a253d4e27a0394111fc79fbce4c8",
    },
    StudioBaseline {
        name: "studio-save",
        page: SimulationPage::Save,
        fingerprint: "250abe4ae813f366ef7079e0f346c106f14dda1ceeec1072d81f472fc7520d77",
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
