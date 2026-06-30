//! Native SPICE diode validation against ngspice 46.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use rspice_core::solver::SimulationResult;

fn branch_current(result: &SimulationResult, branch: &str) -> f64 {
    result
        .branch_current_named(branch)
        .unwrap_or_else(|| panic!("missing branch {branch} in {:?}", result.branch_names))
}

fn op_branch_current(model_tail: &str, voltage: f64) -> f64 {
    let deck = format!(
        "* diode high-injection knee oracle\n\
         .options gmin=0\n\
         V1 anode 0 {voltage:.15e}\n\
         D1 anode 0 DK\n\
         .model DK D(IS=1e-14 N=1 RS=0 CJO=0 {model_tail})\n\
         .op\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("diode deck parses");
    let mut config = SimulationConfig::default();
    // Isolate the model-card high-injection knee from RSpice's internal
    // nodal conditioning floor; the deck already sets junction GMIN to zero.
    config.convergence_config.gmin_target = 0.0;
    let result = Engine::new(config)
        .run_dc_op(&netlist)
        .expect("diode op converges");
    branch_current(&result, "v1")
}

fn assert_close(label: &str, got: f64, expected: f64, rel_tol: f64, abs_tol: f64) {
    let abs = (got - expected).abs();
    let tol = abs_tol.max(rel_tol * expected.abs().max(got.abs()));
    assert!(
        abs <= tol,
        "{label}: rspice={got:.12e} ngspice46={expected:.12e} abs={abs:.3e} tol={tol:.3e}"
    );
}

#[test]
fn diode_ikf_limits_forward_high_injection_current_like_ngspice46() {
    // ngspice-46, same one-diode .op deck, `IKF=1e-3`:
    // i(V1) = -1.551419616134975e-02.  Without the high-injection knee
    // rolloff this bias point is -2.708299612085749e-01 A.
    assert_close(
        "IKF-limited diode source current",
        op_branch_current("IKF=1e-3", 0.8),
        -1.551_419_616_134_975e-2,
        2.0e-8,
        1.0e-10,
    );
}

#[test]
fn diode_tiny_ikf_is_disabled_like_ngspice46() {
    // ngspice-46 prints `Warning: ... IKF too small - model effect disabled!`
    // and returns the no-knee current for this vendor-model corner.
    assert_close(
        "tiny IKF disabled diode source current",
        op_branch_current("IKF=1e-186", 0.8),
        -2.708_299_612_085_749e-1,
        2.0e-8,
        1.0e-10,
    );
}

#[test]
fn diode_ik_alias_limits_forward_high_injection_current_like_ngspice46() {
    assert_close(
        "IK alias-limited diode source current",
        op_branch_current("IK=1e-3", 0.8),
        -1.551_419_616_134_975e-2,
        2.0e-8,
        1.0e-10,
    );
}

#[test]
fn diode_ikr_limits_reverse_high_injection_current_like_ngspice46() {
    // ngspice-46, same one-diode .op deck, `IKR=1e-14` at Vd=-0.2:
    // i(V1) = 4.989091488361717e-15.  Without the reverse knee this is
    // 9.970924740527615e-15 A.
    assert_close(
        "IKR-limited diode source current",
        op_branch_current("IKR=1e-14", -0.2),
        4.989_091_488_361_717e-15,
        2.0e-6,
        1.0e-20,
    );
}
