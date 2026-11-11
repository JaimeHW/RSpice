//! The Solver page offers no control that authors an override: the ledger
//! reports and hops, and the accessibility tree of the drawn page is what
//! says so.

use super::accessibility::{studio_route, studio_route_nodes};

/// The Solver page offers no way to author an override.
///
/// It carried a single-row editor over the same record the Advanced options
/// section authors: two editors, two pages, one fact, and the two surfaces drew
/// their own sets of authorable fields to do it. The ledger reports and hops
/// now, so the controls that opened, retargeted and removed a draft here are
/// gone — and a control a reader can still reach is a control the page still
/// has, which is why this asks the accessibility tree rather than the source.
#[test]
fn the_solver_page_publishes_no_override_authoring_control() {
    use crate::state::SimulationPage;

    let names: Vec<String> = studio_route_nodes(studio_route(SimulationPage::Solver, None), 1280.0)
        .into_iter()
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .collect();
    // A sweep of an undrawn route would pass this forever. The named policy
    // chooser is the first control on the page, so its presence is what says
    // the route was actually laid out.
    assert!(
        names.iter().any(|name| name == "Balanced"),
        "the route has to have been drawn for this to mean anything: {names:?}"
    );
    assert!(
        names
            .iter()
            .any(|name| name.contains("Simulation compatibility")),
        "compatibility selector must be accessible: {names:?}"
    );
    for gone in [
        "Add override\u{2026}",
        "Override value",
        "Analysis to override",
    ] {
        assert!(
            !names.iter().any(|name| name == gone),
            "the Solver page still publishes {gone:?}: {names:?}"
        );
    }
}
