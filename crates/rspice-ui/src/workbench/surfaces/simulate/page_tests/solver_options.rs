//! Effective solver-policy precision and transaction validation.

use super::super::page_solver;
use super::accessibility::{studio_route, studio_route_nodes};
use super::{OptionsDialogState, RSpiceApp, SimulationOptions};

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
    use crate::workbench::state::SimulationPage;

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

#[test]
fn solver_options_iteration_edit_preserves_precise_temperatures() {
    let mut app = RSpiceApp::test_instance();
    let options = SimulationOptions {
        temp: 27.123_456_789,
        tnom: -40.123_456_789,
        ..app.state.sim_setup.options.clone()
    };
    page_solver::commit_options_transaction(&mut app, &options)
        .expect("valid temperatures commit through the plan transaction");
    app.state.sim_setup.options_draft.itl1 = "73".to_owned();
    page_solver::commit_draft(&mut app);

    let mut expected = options;
    expected.itl1 = 73;
    assert_eq!(
        serde_json::to_value(&app.state.sim_setup.options).unwrap(),
        serde_json::to_value(&expected).unwrap(),
        "editing the iteration budget changed an unrelated authored option"
    );
    assert!(!app.state.workbench.analysis_lifecycle_status.is_refusal());
    assert_eq!(
        app.state
            .sim_setup
            .op
            .temperature
            .parse::<f64>()
            .unwrap()
            .to_bits(),
        expected.temp.to_bits(),
        "the operating-point editor must retain the precise reference temperature"
    );
}

#[test]
fn solver_options_invalid_drafts_cannot_replace_effective_policy() {
    type Edit = fn(&mut OptionsDialogState, &str);
    let fields: [(&str, Edit, &[&str]); 7] = [
        (
            "tnom",
            |draft, value| draft.tnom = value.to_owned(),
            &["NaN", "inf", "-273.15"],
        ),
        (
            "trtol",
            |draft, value| draft.trtol = value.to_owned(),
            &["0", "-1"],
        ),
        (
            "transient_lte_reltol",
            |draft, value| draft.transient_lte_reltol = value.to_owned(),
            &["0", "-1"],
        ),
        (
            "transient_lte_abstol",
            |draft, value| draft.transient_lte_abstol = value.to_owned(),
            &["0", "-1"],
        ),
        (
            "gmin",
            |draft, value| draft.gmin = value.to_owned(),
            &["-1"],
        ),
        (
            "bypass_reltol",
            |draft, value| draft.bypass_reltol = value.to_owned(),
            &["-1"],
        ),
        (
            "bypass_abstol",
            |draft, value| draft.bypass_abstol = value.to_owned(),
            &["-1"],
        ),
    ];
    for (name, edit, values) in fields {
        for value in values {
            let mut app = RSpiceApp::test_instance();
            let before = serde_json::to_value(&app.state.sim_setup.options).unwrap();
            edit(&mut app.state.sim_setup.options_draft, value);
            page_solver::commit_draft(&mut app);
            assert!(
                app.state.workbench.analysis_lifecycle_status.is_refusal(),
                "{name}={value} was accepted"
            );
            assert!(
                app.state
                    .workbench
                    .analysis_lifecycle_status
                    .message()
                    .contains(name),
                "the refusal must identify {name}"
            );
            assert_eq!(
                serde_json::to_value(&app.state.sim_setup.options).unwrap(),
                before,
                "invalid {name} changed effective options"
            );
        }
    }
}

#[test]
fn solver_options_transaction_rejects_invalid_policy_before_mutation() {
    let mut app = RSpiceApp::test_instance();
    let before = serde_json::to_value(&app.state.sim_setup.options).unwrap();
    let invalid = SimulationOptions {
        tnom: f64::NAN,
        ..app.state.sim_setup.options.clone()
    };
    let error = page_solver::commit_options_transaction(&mut app, &invalid)
        .expect_err("the effective-options transaction must validate its own input");
    assert!(
        error.contains("tnom"),
        "the refusal must identify the invalid field: {error}"
    );
    assert_eq!(
        serde_json::to_value(&app.state.sim_setup.options).unwrap(),
        before
    );
}
