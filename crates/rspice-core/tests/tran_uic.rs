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

/// The first accepted point after t=0 of a `.TRAN ... UIC` run, per node.
///
/// UIC records t=0 as the assigned startup state (zero everywhere the deck
/// named nothing); the state a device-line `IC=` seeded shows up in the first
/// integrated point, exactly as it does in ngspice's own printout.
fn first_integrated_point(deck: &str, tstop: f64, tstep: f64, nodes: &[&str]) -> Vec<f64> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, tstop, tstep)
        .expect("transient solves");
    let index = result
        .time
        .iter()
        .position(|&t| t > 0.0)
        .expect("at least one integrated point");
    nodes
        .iter()
        .map(|node| {
            result
                .try_voltage_waveform_named(node)
                .unwrap_or_else(|| panic!("missing waveform for {node}"))[index]
        })
        .collect()
}

fn bjt_ic_deck(tail: &str, startup: &str) -> String {
    format!(
        "bjt device ic under uic\n\
         vcc cc 0 dc 5\n\
         rc cc c 1k\n\
         q1 c b 0 qnpn{tail}\n\
         rb b 0 1meg\n\
         cb b 0 1p\n\
         .model qnpn npn is=1e-16 bf=100 cje=2p cjc=1p tf=1n\n\
         .tran 1n 20n{startup}\n\
         .end\n"
    )
}

#[test]
fn bjt_instance_ic_vector_opens_the_uic_transient_at_the_authored_junction_state() {
    // `bjtload.c:245-252` evaluates the UIC transient operating point at
    // `vbe = type*BJTicVBE`, `vce = type*BJTicVCE`; ngspice-46 reports
    // v(b) = 5.425358e-01 and v(c) = 2.849371e+00 at its first integrated
    // point for this deck, against 3.322830e-03 and 1.331804e-02 without the
    // vector.
    let seeded =
        first_integrated_point(&bjt_ic_deck(" ic=0.7,3", " uic"), 20e-9, 1e-9, &["b", "c"]);
    for (label, got, expected) in [
        ("v(b)", seeded[0], 5.425358e-01),
        ("v(c)", seeded[1], 2.849371e+00),
    ] {
        let rel = (got - expected).abs() / expected.abs();
        assert!(
            rel < 1.0e-5,
            "{label}: rspice={got:.9e} ngspice={expected:.9e} rel={rel:.3e}"
        );
    }

    // The vector, not the deck, produces that state. An unseeded instance
    // still opens its first UIC load at tVcrit here rather than at the node
    // bias ngspice's `BJTgetic` would have filled in, which is a separate and
    // wider gap in RSpice's UIC startup — pinned only as "materially different"
    // so closing it later does not have to touch this test.
    let bare = first_integrated_point(&bjt_ic_deck("", " uic"), 20e-9, 1e-9, &["b"]);
    assert!(
        (bare[0] - 5.425358e-01).abs() > 1.0e-1,
        "the seeded state must come from the vector, not the deck, got {}",
        bare[0]
    );
}

#[test]
fn instance_ic_vectors_stay_out_of_an_ordinary_transient() {
    // The arm is gated on `MODEUIC` in every ngspice family that has one
    // (`bjtload.c:246`, `dioload.c:154`, `jfetload.c:108`), and ngspice-46
    // agrees: the same deck without the keyword reports the identical
    // waveform with and without `IC=`.
    let bare = first_integrated_point(&bjt_ic_deck("", ""), 20e-9, 1e-9, &["b", "c"]);
    let seeded = first_integrated_point(&bjt_ic_deck(" ic=0.7,3", ""), 20e-9, 1e-9, &["b", "c"]);
    assert_eq!(
        bare, seeded,
        "an ordinary .TRAN must ignore a device-line IC vector"
    );
}

#[test]
fn jfet_instance_ic_vector_opens_the_uic_transient_at_the_authored_channel_state() {
    // `jfetload.c:106-111`: `vds = type*icVDS`, `vgs = type*icVGS`.
    // ngspice-46 on this deck reports v(g) = -5.97642e-01 and
    // v(d) = 1.407991e+00 at its first integrated point, against
    // 3.287650e-03 and 1.318037e-02 without the vector. The few-percent
    // residual is RSpice's gate depletion charge at reverse bias, which the
    // vector exposes rather than causes: the zero-gate-bias deck below agrees
    // with ngspice to six figures.
    let deck = |tail: &str| {
        format!(
            "jfet device ic under uic\n\
             vdd dd 0 dc 5\n\
             rd dd d 1k\n\
             j1 d g 0 jmod{tail}\n\
             rg g 0 1meg\n\
             cg g 0 1p\n\
             .model jmod njf vto=-2 beta=1m cgs=2p cgd=1p\n\
             .tran 1n 20n uic\n\
             .end\n"
        )
    };

    let zero_gate = first_integrated_point(&deck(" ic=3,0"), 20e-9, 1e-9, &["d"]);
    let rel = (zero_gate[0] - 2.990738e+00).abs() / 2.990738e+00;
    assert!(
        rel < 1.0e-5,
        "v(d) with IC=3,0: {zero_gate:?} rel={rel:.3e}"
    );

    let reverse_gate = first_integrated_point(&deck(" ic=1,-1"), 20e-9, 1e-9, &["g", "d"]);
    for (label, got, expected) in [
        ("v(g)", reverse_gate[0], -5.97642e-01),
        ("v(d)", reverse_gate[1], 1.407991e+00),
    ] {
        let rel = (got - expected).abs() / expected.abs();
        assert!(
            rel < 5.0e-2,
            "{label}: rspice={got:.9e} ngspice={expected:.9e} rel={rel:.3e}"
        );
    }
}

#[test]
fn mosfet_instance_ic_vector_opens_the_uic_transient_at_the_authored_bias() {
    // `mos1load.c:396-408` assigns `vds/vgs/vbs` from the vector and skips the
    // limiter block entirely. ngspice-46 reports v(d) = 1.957231e+00 at its
    // first integrated point for this deck, against 9.912537e-03 without the
    // vector; the drain carries the bulk junction charge the vector sets.
    let deck = |tail: &str| {
        format!(
            "mos device ic under uic\n\
             vdd dd 0 dc 5\n\
             rd dd d 1k\n\
             m1 d g 0 0 nm w=10u l=1u{tail}\n\
             rg g 0 1meg\n\
             cg g 0 1p\n\
             .model nm nmos level=1 vto=1 kp=20u cgso=1n cgdo=1n cbd=1p cbs=1p\n\
             .tran 1n 20n uic\n\
             .end\n"
        )
    };

    let seeded = first_integrated_point(&deck(" ic=2,3,0"), 20e-9, 1e-9, &["d"]);
    let rel = (seeded[0] - 1.957231e+00).abs() / 1.957231e+00;
    assert!(rel < 1.0e-3, "v(d): rspice={:.9e} rel={rel:.3e}", seeded[0]);

    let bare = first_integrated_point(&deck(""), 20e-9, 1e-9, &["d"]);
    let rel = (bare[0] - 9.912537e-03).abs() / 9.912537e-03;
    assert!(
        rel < 1.0e-5,
        "without the vector the deck must keep its ngspice answer: {:.9e}",
        bare[0]
    );
}

#[test]
fn diode_instance_ic_reaches_the_uic_startup_state() {
    // The diode's `IC` is a scalar junction voltage in both references
    // (`dio/dio.c:16` declares `IF_REAL`; `N_DEV_Diode.C:79` a plain
    // `addPar`), and both document the same UIC arm: `dioload.c:153-157`
    // assigns `vd = DIOinitCond` and `N_DEV_Diode.C:1151-1154` assigns
    // `Vd = InitCond`.
    //
    // There is no ngspice oracle for this one. `dio/dioparam.c:62-64` sets
    // `DIOinitCond` without ever setting `DIOinitCondGiven` — the flag is
    // declared at `diodefs.h:113`, read at `diogetic.c:28` and set nowhere —
    // so ngspice-46 overwrites the authored value with the node difference
    // before its own arm can read it, and measurably reports the same
    // waveform with and without `IC=`. Xyce binds the same parameter through
    // `setGivenMember(&InitCondGiven)` and does reach the arm, so the value
    // is honoured here.
    let deck = |tail: &str, startup: &str| {
        format!(
            "diode device ic under uic\n\
             v1 a 0 dc 0\n\
             r1 a n 1meg\n\
             d1 n 0 dmod{tail}\n\
             c1 n 0 1p\n\
             .model dmod d is=1e-14 cjo=2p tt=1n rs=0\n\
             .tran 1n 20n{startup}\n\
             .end\n"
        )
    };

    let bare = first_integrated_point(&deck("", " uic"), 20e-9, 1e-9, &["n"]);
    let seeded = first_integrated_point(&deck(" ic=0.7", " uic"), 20e-9, 1e-9, &["n"]);
    assert!(
        bare[0].abs() < 1.0e-9,
        "an unseeded diode starts the UIC run at its own zero bias, got {}",
        bare[0]
    );
    assert!(
        seeded[0] > 1.0e-3,
        "IC=0.7 must charge the junction the UIC run starts from, got {}",
        seeded[0]
    );

    assert_eq!(
        first_integrated_point(&deck("", ""), 20e-9, 1e-9, &["n"]),
        first_integrated_point(&deck(" ic=0.7", ""), 20e-9, 1e-9, &["n"]),
        "an ordinary .TRAN must ignore the diode IC"
    );
}
