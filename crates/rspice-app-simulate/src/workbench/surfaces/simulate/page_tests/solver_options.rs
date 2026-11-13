//! Effective solver-policy precision and transaction validation.

use super::super::page_solver;
use super::{OptionsDialogState, SimulationOptions};
use crate::workbench::simulate::test_host::SimulateTestHost;

#[test]
fn solver_options_iteration_edit_preserves_precise_temperatures() {
    let mut harness = SimulateTestHost::harness();
    let options = SimulationOptions {
        temp: 27.123_456_789,
        tnom: -40.123_456_789,
        ..harness.model.sim_setup.options.clone()
    };
    page_solver::commit_options_transaction(&mut harness.cx(), &options)
        .expect("valid temperatures commit through the plan transaction");
    harness.model.sim_setup.session.options_draft.itl1 = "73".to_owned();
    page_solver::commit_draft(&mut harness.cx());

    let mut expected = options;
    expected.itl1 = 73;
    assert_eq!(
        serde_json::to_value(&harness.model.sim_setup.options).unwrap(),
        serde_json::to_value(&expected).unwrap(),
        "editing the iteration budget changed an unrelated authored option"
    );
    assert!(!harness.model.analysis_lifecycle_status.is_refusal());
    assert_eq!(
        harness
            .model
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
            let mut harness = SimulateTestHost::harness();
            let before = serde_json::to_value(&harness.model.sim_setup.options).unwrap();
            edit(&mut harness.model.sim_setup.session.options_draft, value);
            page_solver::commit_draft(&mut harness.cx());
            assert!(
                harness.model.analysis_lifecycle_status.is_refusal(),
                "{name}={value} was accepted"
            );
            assert!(
                harness
                    .model
                    .analysis_lifecycle_status
                    .message()
                    .contains(name),
                "the refusal must identify {name}"
            );
            assert_eq!(
                serde_json::to_value(&harness.model.sim_setup.options).unwrap(),
                before,
                "invalid {name} changed effective options"
            );
        }
    }
}

#[test]
fn solver_options_transaction_rejects_invalid_policy_before_mutation() {
    let mut harness = SimulateTestHost::harness();
    let before = serde_json::to_value(&harness.model.sim_setup.options).unwrap();
    let invalid = SimulationOptions {
        tnom: f64::NAN,
        ..harness.model.sim_setup.options.clone()
    };
    let error = page_solver::commit_options_transaction(&mut harness.cx(), &invalid)
        .expect_err("the effective-options transaction must validate its own input");
    assert!(
        error.contains("tnom"),
        "the refusal must identify the invalid field: {error}"
    );
    assert_eq!(
        serde_json::to_value(&harness.model.sim_setup.options).unwrap(),
        before
    );
}

#[test]
fn simulation_compatibility_commits_through_the_solver_transaction() {
    use crate::simulation::dialog::SimulationCompatibility;
    let mut harness = SimulateTestHost::harness();
    harness.model.sim_setup.session.options_draft.compatibility = SimulationCompatibility::Xyce;
    page_solver::commit_draft(&mut harness.cx());
    assert!(!harness.model.analysis_lifecycle_status.is_refusal());
    assert_eq!(
        harness.model.sim_setup.options.compatibility,
        SimulationCompatibility::Xyce
    );
    let deck = rspice_simulation::analysis_preparation::apply_simulation_options_to_netlist(
        "solver choice\nV1 in 0 1\nR1 in 0 1k\n.end\n",
        &harness.model.sim_setup.options,
    );
    let parsed = rspice_core::Netlist::parse(&deck).unwrap();
    assert_eq!(
        rspice_simulation::netlist_preparation::build_engine_config(&parsed, None).spice_dialect,
        rspice_core::SpiceDialect::Xyce
    );
    harness.model.sim_setup.session.options_draft.itl1 = "81".into();
    page_solver::commit_draft(&mut harness.cx());
    assert_eq!(
        harness.model.sim_setup.options.compatibility,
        SimulationCompatibility::Xyce
    );
    assert_eq!(harness.model.sim_setup.options.itl1, 81);
}
