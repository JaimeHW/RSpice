//! Physical AC-operator regressions.
//!
//! These cases deliberately exercise weak, ideal, floating, and dependent
//! equations through the public engine boundary. A solver conditioning term
//! must never become a hidden shunt or series element that changes them.

use rspice_core::Complex64;
use rspice_core::analysis::ac::AcResult;
use rspice_core::engine::{Engine, SimulationConfig, SimulationErrorCode};
use rspice_core::netlist::Netlist;

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).expect("AC regression deck parses")
}

fn solve_one(deck: &str) -> AcResult {
    Engine::new(SimulationConfig::default())
        .run_ac(&parse(deck), &[1.0e3])
        .expect("physical AC operator solves")
        .pop()
        .expect("one requested AC point")
}

fn voltage(point: &AcResult, node: &str) -> Complex64 {
    let index = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("missing node {node:?} in {:?}", point.node_names));
    point.voltages[index]
}

fn branch_current(point: &AcResult, branch: &str) -> Complex64 {
    let index = point
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(branch))
        .unwrap_or_else(|| panic!("missing branch {branch:?} in {:?}", point.branch_names));
    point.currents[index]
}

fn assert_solver_singular(deck: &str) {
    let error = Engine::new(SimulationConfig::default())
        .run_ac(&parse(deck), &[1.0e3])
        .expect_err("an underdetermined physical AC operator must not be regularized");
    assert_eq!(
        error.descriptor().code,
        SimulationErrorCode::SolverError,
        "singularity must cross the public API as a solver error: {error}"
    );
    assert!(
        error.to_string().to_ascii_lowercase().contains("singular"),
        "solver error must identify the singular operator: {error}"
    );
}

#[test]
fn one_ampere_through_one_hundred_teraohms_produces_one_hundred_teravolts() {
    let point = solve_one(
        "one-ampere weak-conductance transimpedance\n\
         I1 0 out DC 0 AC 1\n\
         R1 out 0 1e14\n\
         .AC LIN 1 1k 1k\n\
         .END\n",
    );
    let actual = voltage(&point, "out");
    let expected = Complex64::new(1.0e14, 0.0);
    let tolerance = 64.0 * f64::EPSILON * expected.norm();

    assert!(
        (actual - expected).norm() <= tolerance,
        "1 A through 1e14 ohm must produce 1e14 V, got {actual:?} (tolerance {tolerance:.3e})"
    );
}

#[test]
fn exact_and_near_zero_resistors_preserve_their_branch_constitutive_equation() {
    for resistance in ["0", "1e-101"] {
        let point = solve_one(&format!(
            "branch-form short AC constitutive equation\n\
             V1 a 0 DC 0 AC 1e14\n\
             RSHORT a b {resistance}\n\
             RLOAD b 0 1\n\
             .AC LIN 1 1k 1k\n\
             .END\n"
        ));
        let va = voltage(&point, "a");
        let vb = voltage(&point, "b");
        let current = branch_current(&point, "RSHORT");
        let resistance = resistance.parse::<f64>().unwrap();
        let constitutive_residual = va - vb - resistance * current;
        let voltage_tolerance = 2.0 * f64::EPSILON * va.norm().max(vb.norm()).max(1.0);
        let current_tolerance = 64.0 * f64::EPSILON * 1.0e14;

        assert!(
            constitutive_residual.norm() <= voltage_tolerance,
            "RSHORT={resistance:.1e} violates Va-Vb=R*I: Va={va:?}, Vb={vb:?}, I={current:?}, residual={constitutive_residual:?}"
        );
        assert!(
            (current - Complex64::new(1.0e14, 0.0)).norm() <= current_tolerance,
            "RSHORT={resistance:.1e} must carry the one-ohm load current, got {current:?}"
        );
    }
}

#[test]
fn genuinely_floating_differential_network_is_singular() {
    assert_solver_singular(
        "floating differential AC component\n\
         VREF reference 0 DC 0 AC 0\n\
         RREF reference 0 1k\n\
         VDIFF left right DC 0 AC 1\n\
         RDIFF left right 1k\n\
         .AC LIN 1 1k 1k\n\
         .END\n",
    );
}

#[test]
fn frequency_activated_behavioral_voltage_identity_is_singular() {
    // At DC, HERTZ is zero and BID prescribes V(out)=0, so the bias point is
    // well-defined. At an AC frequency the exact small-signal equation becomes
    // V(out)=V(out), leaving the behavioral branch current underdetermined.
    assert_solver_singular(
        "underdetermined behavioral AC identity\n\
         IEX 0 out DC 0 AC 1\n\
         BID out 0 V={V(out)*(HERTZ>0)}\n\
         RLOAD out 0 1k\n\
         .AC LIN 1 1k 1k\n\
         .END\n",
    );
}
