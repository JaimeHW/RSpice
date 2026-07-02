//! `.TRAN ... UIC`: skip the operating point and integrate from user
//! initial conditions, ngspice-style.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{AnalysisCommand, Netlist};

fn out_waveform(deck: &str) -> (Vec<f64>, Vec<f64>) {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
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
