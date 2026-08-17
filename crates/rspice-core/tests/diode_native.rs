//! Native SPICE diode validation against ngspice 46.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use rspice_core::solver::SimulationResult;

fn branch_current(result: &SimulationResult, branch: &str) -> f64 {
    result
        .branch_current_named(branch)
        .unwrap_or_else(|| panic!("missing branch {branch} in {:?}", result.branch_names))
}

fn ac_branch_current(deck: &str, branch: &str, frequency: f64) -> rspice_core::Complex64 {
    let netlist = Netlist::parse(deck).expect("diode AC deck parses");
    let point = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[frequency])
        .expect("diode AC converges")
        .pop()
        .expect("one AC point");
    let index = point
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(branch))
        .unwrap_or_else(|| panic!("missing branch {branch} in {:?}", point.branch_names));
    point.currents[index]
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

#[test]
fn diode_grading_above_one_retains_reverse_bias_capacitance_like_ngspice46() {
    let frequency = 1.0e6;
    let deck = "* M > 1 diode depletion-capacitance oracle\n\
                V1 anode 0 DC -4 AC 1\n\
                D1 anode 0 DM\n\
                .model DM D(IS=1e-14 N=1 RS=0 CJO=463.53p VJ=9.99 M=1.2861 TT=0 TNOM=25)\n\
                .temp 25\n\
                .end\n";
    let current = ac_branch_current(deck, "V1", frequency);
    let capacitance = current.im.abs() / (std::f64::consts::TAU * frequency);
    let expected = 463.53e-12 * (1.0_f64 + 4.0 / 9.99).powf(-1.2861);

    assert_close(
        "M > 1 reverse-bias capacitance",
        capacitance,
        expected,
        2.0e-10,
        1.0e-21,
    );
}

/// A diode across a negative resistance has two DC roots: the junction's
/// low-forward-bias root and a deeply reverse-biased one where the reverse
/// saturation current balances the same line. Which root the solve reports is
/// exactly what the `OFF` instance keyword is there to decide.
fn negative_resistance_bistable_deck(off: bool) -> String {
    format!(
        "* diode bistable steered by the OFF keyword\n\
         vs vs 0 dc -0.3\n\
         rn n vs -1k\n\
         d1 n 0 dmod{}\n\
         .model dmod D(IS=1e-3 N=1)\n\
         .op\n\
         .end\n",
        if off { " OFF" } else { "" }
    )
}

fn op_node_voltage(deck: &str, node: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("diode deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("diode bistable operating point converges");
    result
        .try_voltage_named(node)
        .unwrap_or_else(|| panic!("missing voltage for node {node}"))
}

#[test]
fn diode_off_keyword_selects_the_bistable_operating_point_branch() {
    // dioload.c evaluates an OFF instance at exactly vd = 0 on MODEINITJCT, in
    // every compatibility mode, which lands this network on its forward root.
    // ngspice-46 reports v(n) = 6.923413e-03 there. An instance whose IS is
    // large enough for the pnjlim reference to still conduct is where that
    // differs from merely limiting the raw bias against zero, and a simulator
    // that drops the keyword reports the reverse root near -1.3 V instead.
    let off_root = op_node_voltage(&negative_resistance_bistable_deck(true), "n");
    assert_close("OFF diode bistable v(n)", off_root, 6.923_413e-3, 1.0e-5, 0.0);
}

/// The same network at a default saturation current, where `tVcrit` is a
/// three-quarter-volt forward bias rather than the 75 mV a milliamp `IS`
/// gives. Here the two MODEINITJCT arms land on genuinely different roots, so
/// the deck reads the startup bias directly rather than inferring it.
fn standard_bistable_deck(off: bool) -> String {
    format!(
        "* diode bistable steered by the MODEINITJCT startup bias\n\
         vs vs 0 dc -1.5\n\
         rn n vs -1k\n\
         d1 n 0 dmod{}\n\
         .model dmod D(IS=1e-14 N=1)\n\
         .op\n\
         .end\n",
        if off { " OFF" } else { "" }
    )
}

#[test]
fn diode_startup_bias_selects_the_bistable_operating_point_branch() {
    // Both roots are genuine equilibria of this network: the junction's
    // forward root near 0.68 V, and the one where the resistor line meets the
    // reverse saturation current a hair below the supply. Which one a solve
    // reports is decided entirely by where dioload.c's MODEINITJCT arms open
    // the junction — `vd = tVcrit` (dioload.c:162-166) for an unmarked
    // instance, `vd = 0` (dioload.c:158-161) for one the deck marked OFF.
    //
    // ngspice-46 reports v(n) = 6.752190e-01 and -1.50000e+00 respectively.
    // A simulator that instead limits a zero-referenced raw bias opens both
    // instances at cutoff and reports the reverse root for both, which makes
    // the unmarked diode's operating point disagree with every other SPICE.
    let unmarked = op_node_voltage(&standard_bistable_deck(false), "n");
    assert_close(
        "unmarked diode bistable v(n)",
        unmarked,
        6.752_190e-1,
        1.0e-5,
        0.0,
    );

    let off = op_node_voltage(&standard_bistable_deck(true), "n");
    assert_close("OFF diode bistable v(n)", off, -1.5, 1.0e-6, 1.0e-9);
}
