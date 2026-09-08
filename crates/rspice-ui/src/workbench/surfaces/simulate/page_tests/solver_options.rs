//! Effective solver-policy precision and transaction validation.

use super::super::page_solver;
use super::{OptionsDialogState, RSpiceApp, SimulationOptions};

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
