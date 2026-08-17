//! `.TRAN ... UIC`: skip the operating point and integrate from user
//! initial conditions, ngspice-style. Without `UIC` the same `.IC` card
//! instead constrains the bias solution that starts the transient.

use rspice_core::engine::{
    ConvergenceConfig, Engine, SimulationConfig, SpiceDialect, TransientStartupMode,
};
use rspice_core::netlist::{AnalysisCommand, Netlist};
use rspice_core::numerics::integration::IntegrationMethod;

fn out_waveform(deck: &str) -> (Vec<f64>, Vec<f64>) {
    out_waveform_with_config(deck, SimulationConfig::default())
}

fn out_waveform_with_config(deck: &str, config: SimulationConfig) -> (Vec<f64>, Vec<f64>) {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(config);
    let result = engine
        .run_tran(&netlist, 5e-3, 1e-5)
        .expect("transient solves");
    let out_idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node present");
    (result.time.clone(), result.voltages[out_idx].clone())
}

const RC_DECK_UIC: &str = "\
* rc charge from zero (uic)
v1 in 0 dc 5
r1 in out 1k
c1 out 0 1u
.tran 10u 5m uic
.end
";

const RC_DECK_OP: &str = "\
* rc at operating point
v1 in 0 dc 5
r1 in out 1k
c1 out 0 1u
.tran 10u 5m
.end
";

#[test]
fn uic_keyword_parses() {
    let netlist = Netlist::parse(RC_DECK_UIC).expect("deck parses");
    assert!(
        netlist
            .analyses
            .iter()
            .any(|a| matches!(a, AnalysisCommand::Tran { uic: true, .. })),
        "UIC keyword must reach the analysis command"
    );
    let plain = Netlist::parse(RC_DECK_OP).expect("deck parses");
    assert!(
        plain
            .analyses
            .iter()
            .any(|a| matches!(a, AnalysisCommand::Tran { uic: false, .. })),
        "without the keyword the flag stays clear"
    );
}

#[test]
fn uic_starts_from_zero_and_charges() {
    let (time, v_out) = out_waveform(RC_DECK_UIC);
    assert!(
        v_out[0].abs() < 1e-6,
        "UIC must start the capacitor at 0 V (no operating point), got {}",
        v_out[0]
    );
    // tau = 1 ms: at t = 5 ms the capacitor sits at 5*(1 - e^-5) = 4.966 V.
    let v_end = *v_out.last().unwrap();
    let expected = 5.0 * (1.0 - (-5.0_f64).exp());
    assert!(
        (v_end - expected).abs() < 0.05,
        "RC charge from zero: expected ~{expected:.3} V at t=5ms, got {v_end:.3}"
    );
    // And the trajectory matches the analytic charge curve mid-way.
    let mid_idx = time.iter().position(|&t| t >= 1e-3).unwrap();
    let expected_mid = 5.0 * (1.0 - (-time[mid_idx] / 1e-3).exp());
    assert!(
        (v_out[mid_idx] - expected_mid).abs() < 0.05,
        "at t={:.3e}: expected {expected_mid:.3} V, got {:.3}",
        time[mid_idx],
        v_out[mid_idx]
    );
}

#[test]
fn uic_first_accepted_rc_point_contains_the_integrated_state() {
    let netlist = Netlist::parse(
        "first UIC RC step\n\
         v0 1 0 dc 1\n\
         r1 1 2 1k\n\
         c1 2 0 1u\n\
         .tran 100u 10m uic\n\
         .print tran all\n\
         .control\n\
         tran 100u 10m uic\n\
         plot v(2)\n\
         .endc\n\
         .end\n",
    )
    .expect("deck parses");
    let result = Engine::new(SimulationConfig {
        max_iterations: SimulationConfig::default().max_iterations.max(1200),
        convergence_config: ConvergenceConfig::robust(),
        integration_method: IntegrationMethod::Trapezoidal,
        min_timestep: 1e-12,
        spice_dialect: SpiceDialect::Ngspice,
        ..SimulationConfig::default()
    })
    .run_tran(&netlist, 10e-3, 100e-6)
    .expect("transient solves");
    let out = result
        .try_voltage_waveform_named("2")
        .expect("node 2 waveform exists");
    let first = result
        .time
        .iter()
        .position(|time| *time > 0.0)
        .expect("positive-time result point exists");
    let dt = result.time[first];
    let expected_backward_euler = dt / (1e-3 + dt);
    assert!(
        (out[first] - expected_backward_euler).abs() <= 1e-9,
        "at first accepted t={dt:e}, expected {expected_backward_euler:e}, got {:e}",
        out[first]
    );
}

#[test]
fn without_uic_the_operating_point_applies() {
    let (_, v_out) = out_waveform(RC_DECK_OP);
    assert!(
        (v_out[0] - 5.0).abs() < 1e-6,
        "without UIC the transient starts at the DC operating point, got {}",
        v_out[0]
    );
}

#[test]
fn uic_honors_element_ic_value() {
    let deck = "\
* element IC under uic
v1 in 0 dc 5
r1 in out 1k
c1 out 0 1u ic=2
.tran 10u 1m uic
.end
";
    let (_, v_out) = out_waveform(deck);
    assert!(
        (v_out[0] - 2.0).abs() < 0.01,
        "IC=2 must seed the capacitor under UIC, got {}",
        v_out[0]
    );
}

#[test]
fn xyce_without_uic_honors_capacitor_ic_value() {
    let deck = "\
* Xyce capacitor IC without uic
r1 out 0 1k
c1 out 0 1u ic=1
.tran 10u 1m
.end
";
    let (_, v_out) = out_waveform_with_config(
        deck,
        SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce),
    );
    assert!(
        (v_out[0] - 1.0).abs() < 0.01,
        "Xyce dialect should seed capacitor IC=1 without UIC, got {}",
        v_out[0]
    );
}

#[test]
fn default_dialect_without_uic_uses_operating_point_over_capacitor_ic() {
    let deck = "\
* ngspice-style capacitor IC without uic
r1 out 0 1k
c1 out 0 1u ic=1
.tran 10u 1m
.end
";
    let (_, v_out) = out_waveform(deck);
    assert!(
        v_out[0].abs() < 0.01,
        "default dialect should keep the operating-point startup without UIC, got {}",
        v_out[0]
    );
}

#[test]
fn uic_honors_dot_ic_node_voltage() {
    let deck = "\
* .ic node voltage under uic
v1 in 0 dc 5
r1 in out 1k
c1 out 0 1u
.ic v(out)=3
.tran 10u 1m uic
.end
";
    let (_, v_out) = out_waveform(deck);
    assert!(
        (v_out[0] - 3.0).abs() < 0.01,
        ".ic v(out)=3 must seed the node under UIC, got {}",
        v_out[0]
    );
}

#[test]
fn without_uic_dot_ic_constrains_the_bias_solution() {
    // ngspice forces the `.ic` node voltages during the bias solution that
    // starts an ordinary `.tran`, then releases them. V(n1) discriminates the
    // three candidate semantics: 1.0 V is the clamped bias solution, 2.0 V is
    // a post-solve overlay onto an unconstrained solve, 0.0 V is UIC.
    let deck = "\
IC clamp scope discriminator
V1 in 0 DC 2
R1 in n1 1k
R2 n1 n2 1k
C1 n2 0 1n
.ic v(n2)=0
.tran 1u 5u
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 5e-6, 1e-6)
        .expect("IC-constrained transient solves");
    let n1 = result
        .try_voltage_waveform_named("n1")
        .expect("n1 waveform exists")[0];
    let n2 = result
        .try_voltage_waveform_named("n2")
        .expect("n2 waveform exists")[0];
    assert_eq!(n2.to_bits(), 0.0f64.to_bits());
    assert!(
        (n1 - 1.0).abs() <= 1e-9,
        ".ic v(n2)=0 must constrain the t=0 bias solution (ngspice-46 reports V(n1)=1), got {n1}"
    );
}

#[test]
fn without_uic_an_ideal_source_outvotes_dot_ic() {
    // A clamped node keeps the branch equations that pass through it, so an
    // ideal source still owns the node and the clamp only shows up as the
    // current it takes to hold it. ngspice-46 reports in=2, n1=1 and
    // v1#branch=-1.5e10 for this deck.
    let deck = "\
IC clamp on a source-driven node
V1 in 0 DC 2
R1 in n1 1k
R2 n1 0 1k
.ic v(in)=0.5
.tran 1u 5u
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 5e-6, 1e-6)
        .expect("outvoted IC transient solves");
    let node = |name: &str| {
        result
            .try_voltage_waveform_named(name)
            .unwrap_or_else(|| panic!("{name} waveform exists"))[0]
    };
    assert!(
        (node("in") - 2.0).abs() <= 1e-9,
        "V1 owns node in; .ic must not overwrite it, got {}",
        node("in")
    );
    assert!(
        (node("n1") - 1.0).abs() <= 1e-9,
        "the rest of the t=0 solution must stay consistent, got {}",
        node("n1")
    );
    let branch = result
        .try_branch_current_waveform_named("v1")
        .expect("v1 branch current waveform exists")[0];
    assert!(
        (branch + 1.5e10).abs() <= 1.0,
        "the clamp current the source takes must reach the branch, got {branch}"
    );
}

#[test]
fn uic_honors_inductor_ic_branch_current() {
    let deck = "\
* inductor IC under uic
i1 in 0 10
r1 in mid 3
r2 mid 0 3
l1 mid 0 1 ic=2
.tran 10m 20m uic
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_tran(&netlist, 20e-3, 10e-3)
        .expect("transient solves");
    let initial = result
        .try_branch_current_waveform_named("l1")
        .and_then(|waveform| waveform.first().copied())
        .expect("inductor branch current waveform exists");

    assert!(
        (initial - 2.0).abs() < 1e-12,
        "inductor IC=2 must seed the branch current under UIC, got {initial}"
    );
}

#[test]
fn mixed_tran_cards_require_and_honor_explicit_selected_startup_mode() {
    let netlist = Netlist::parse(
        "selected transient startup mode\n\
         v1 in 0 5\n\
         r1 in out 1k\n\
         c1 out 0 1u\n\
         .tran 10u 1m\n\
         .tran 10u 1m uic\n\
         .end\n",
    )
    .expect("mixed transient deck parses");
    let engine = Engine::default();

    let error = engine
        .run_tran(&netlist, 1.0e-3, 10.0e-6)
        .expect_err("the compatibility API must not guess between mixed startup modes");
    assert!(error.to_string().contains("explicit TransientStartupMode"));

    let operating_point = engine
        .run_tran_with_startup_mode(
            &netlist,
            1.0e-3,
            10.0e-6,
            TransientStartupMode::OperatingPoint,
        )
        .expect("the selected ordinary .TRAN executes");
    let uic = engine
        .run_tran_with_startup_mode(&netlist, 1.0e-3, 10.0e-6, TransientStartupMode::Uic)
        .expect("the selected UIC .TRAN executes");
    let op_out = operating_point
        .try_voltage_waveform_named("out")
        .expect("ordinary output waveform");
    let uic_out = uic
        .try_voltage_waveform_named("out")
        .expect("UIC output waveform");
    assert!((op_out[0] - 5.0).abs() <= 1.0e-9);
    assert!(uic_out[0].abs() <= 1.0e-12);
}
