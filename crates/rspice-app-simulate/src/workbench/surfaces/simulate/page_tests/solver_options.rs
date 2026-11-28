//! Effective solver-policy precision and transaction validation.

use super::super::page_solver;
use super::{OptionsDialogState, SimulationOptions};
use crate::workbench::simulate::test_host::SimulateTestHost;

/// A preset decides tolerances and convergence aids. The plan's temperature,
/// TNOM and seed are the engineer's own and reach the deck unchanged, and the
/// page still names the preset whose tolerances the plan holds.
#[test]
fn a_solver_preset_leaves_the_temperature_tnom_and_seed_alone() {
    let mut harness = SimulateTestHost::harness();
    crate::workbench::simulate::chrome::commit_reference_pvt(
        &mut harness.cx(),
        crate::product::ProcessCorner::TT,
        85.0,
    )
    .expect("the run set takes 85 degrees");
    harness.model.sim_setup.session.options_draft.tnom = "25".to_owned();
    harness
        .model
        .sim_setup
        .session
        .options_draft
        .statistical_seed = "42".to_owned();
    page_solver::commit_draft(&mut harness.cx());

    page_solver::apply_preset(&mut harness.cx(), "Accurate");

    let options = &harness.model.sim_setup.options;
    assert_eq!(options.preset_name(), Some("Accurate"));
    let deck = rspice_simulation::analysis_preparation::apply_simulation_options_to_netlist(
        "preset\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n",
        &harness.model.sim_setup.options_block(),
    );
    let lines: Vec<&str> = deck.lines().map(str::trim).collect();
    for line in ["+ TEMP=85", "+ TNOM=25", "+ SEED=42", "+ RELTOL=1e-4"] {
        assert!(lines.contains(&line), "the deck lacks {line}:\n{deck}");
    }
}

/// The plan holds one current tolerance: the number the page shows, the deck
/// states and the engine's convergence test reads, whichever preset came first.
#[test]
fn the_current_tolerance_is_one_stored_number_the_engine_reads() {
    let mut harness = SimulateTestHost::harness();
    page_solver::apply_preset(&mut harness.cx(), "Fast");
    harness.model.sim_setup.session.options_draft.iabstol = "1e-10".to_owned();
    page_solver::commit_draft(&mut harness.cx());

    let options = &harness.model.sim_setup.options;
    let stored = serde_json::to_value(options).unwrap();
    assert!(stored.get("abstol").is_none(), "{stored}");
    assert_eq!(options.iabstol, 1e-10);
    let deck = rspice_simulation::analysis_preparation::apply_simulation_options_to_netlist(
        "current tolerance\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n",
        &harness.model.sim_setup.options_block(),
    );
    let stated: Vec<&str> = deck
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("+ ABSTOL") || line.starts_with("+ IABSTOL"))
        .collect();
    assert_eq!(stated, ["+ IABSTOL=1e-10"], "{deck}");
    let parsed = rspice_core::Netlist::parse(&deck).expect("the deck parses");
    assert_eq!(
        rspice_simulation::netlist_preparation::build_engine_config(&parsed)
            .convergence_config
            .current_abstol,
        1e-10
    );
}

/// A plan states no step ceiling until one is typed: a 100 s transient with a
/// 10 ms step steps at up to 10 ms, not at a hidden 1 ms. A typed value is a
/// real ceiling.
#[test]
fn a_plan_states_no_step_ceiling_until_one_is_typed() {
    const DECK: &str = "start-up\nV1 in 0 PULSE(0 1 0 1 1 100 200)\nR1 in out 1k\nC1 out 0 1m\n.tran 10m 100\n.end\n";
    let mut harness = SimulateTestHost::harness();
    assert_eq!(
        harness.model.sim_setup.session.options_draft.max_timestep,
        ""
    );

    let deck = rspice_simulation::analysis_preparation::apply_simulation_options_to_netlist(
        DECK,
        &harness.model.sim_setup.options_block(),
    );
    assert!(!deck.contains("MAXTIMESTEP"), "{deck}");
    let parsed = rspice_core::Netlist::parse(&deck).expect("the deck parses");
    let config = rspice_simulation::netlist_preparation::build_engine_config(&parsed);
    assert!(
        config.max_timestep.is_infinite(),
        "the engine's ceiling is the transient's own step, got {}",
        config.max_timestep
    );

    harness.model.sim_setup.session.options_draft.max_timestep = "2m".to_owned();
    page_solver::commit_draft(&mut harness.cx());
    let deck = rspice_simulation::analysis_preparation::apply_simulation_options_to_netlist(
        DECK,
        &harness.model.sim_setup.options_block(),
    );
    assert!(
        deck.lines().any(|line| line.trim() == "+ MAXTIMESTEP=2e-3"),
        "{deck}"
    );
    let parsed = rspice_core::Netlist::parse(&deck).expect("the deck parses");
    assert_eq!(
        rspice_simulation::netlist_preparation::build_engine_config(&parsed).max_timestep,
        2e-3
    );
}

/// A plan's temperature is one stored value, the run set's reference point:
/// the options record keeps no copy for an edit to drift, and the deck states
/// the reference.
#[test]
fn the_plan_temperature_is_the_run_sets_reference_alone() {
    let mut harness = SimulateTestHost::harness();
    crate::workbench::simulate::chrome::commit_reference_pvt(
        &mut harness.cx(),
        crate::product::ProcessCorner::TT,
        85.0,
    )
    .expect("the run set takes 85 degrees");
    harness.model.sim_setup.session.options_draft.itl1 = "73".to_owned();
    page_solver::commit_draft(&mut harness.cx());

    let stored = serde_json::to_value(&harness.model.sim_setup.options).unwrap();
    assert!(stored.get("temp").is_none(), "{stored}");
    assert_eq!(
        harness.model.sim_setup.reference_pvt.temperature_celsius,
        85.0
    );
    let deck = rspice_simulation::analysis_preparation::apply_simulation_options_to_netlist(
        "temperature\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n",
        &harness.model.sim_setup.options_block(),
    );
    assert!(
        deck.lines().any(|line| line.trim() == "+ TEMP=85"),
        "{deck}"
    );
    let parsed = rspice_core::Netlist::parse(&deck).expect("the deck parses");
    assert_eq!(
        rspice_simulation::netlist_preparation::build_engine_config(&parsed).temperature,
        85.0 + 273.15
    );
}

#[test]
fn solver_options_iteration_edit_preserves_precise_temperatures() {
    const REFERENCE: f64 = 27.123_456_789;
    let mut harness = SimulateTestHost::harness();
    crate::workbench::simulate::chrome::commit_reference_pvt(
        &mut harness.cx(),
        crate::product::ProcessCorner::TT,
        REFERENCE,
    )
    .expect("the run set takes a precise reference");
    let options = SimulationOptions {
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
        REFERENCE.to_bits(),
        "the operating-point editor must retain the precise reference temperature"
    );
    assert_eq!(
        harness
            .model
            .sim_setup
            .reference_pvt
            .temperature_celsius
            .to_bits(),
        REFERENCE.to_bits()
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
        &harness.model.sim_setup.options_block(),
    );
    let parsed = rspice_core::Netlist::parse(&deck).unwrap();
    assert_eq!(
        rspice_simulation::netlist_preparation::build_engine_config(&parsed).spice_dialect,
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
