//! Fail-closed scalar-LTRA model classification regressions.

use rspice_core::Netlist;
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};

fn deck(model: &str) -> Netlist {
    Netlist::parse(&format!(
        "LTRA model validation\nVIN in 0 AC 1\nO1 in 0 out 0 line\nRLOAD out 0 50\n{model}\n.end\n"
    ))
    .expect("validation deck parses")
}

#[test]
fn invalid_and_tiny_g_ltra_cards_fail_during_build_before_small_signal_stamping() {
    let engine = Engine::new(SimulationConfig::default());
    for (model, evidence) in [
        (
            ".model line ltra r=1 l=1n g=1e-30 c=1p len=1e30",
            "finite nonzero G",
        ),
        (
            ".model line ltra r=1 l=1n g=-1e-30 c=1p len=1",
            "invalid G=",
        ),
        (
            ".model line ltra r=1 l=1n len=1",
            "unsupported or ambiguous RLGC combination",
        ),
        (
            ".model line ltra z0=50 td=1n",
            "outside the distributed RLGC model contract",
        ),
    ] {
        let error = engine
            .run_ac(&deck(model), &[1.0e3])
            .expect_err("invalid LTRA model must fail before an AC matrix is stamped");
        let text = error.to_string();
        assert!(
            text.to_ascii_lowercase().contains("ltra model 'line'"),
            "{model}: {error}"
        );
        assert!(text.contains(evidence), "{model}: {error}");
        assert!(
            !text.contains("singular") && !text.contains("solve failed"),
            "semantic rejection must precede solver diagnostics: {model}: {error}"
        );
    }
}

/// The finite-length RG line is the one nonzero-`G` case ngspice and Xyce
/// implement, so it executes natively. `crates/rspice-core/tests/ltra_rg_line.rs`
/// owns its numerical qualification; this only pins that the classifier routes
/// it to execution instead of to a rejection.
#[test]
fn finite_rg_executes_natively_while_reactive_nonzero_g_stays_rejected() {
    let engine = Engine::new(SimulationConfig::default());
    engine
        .run_ac(&deck(".model line ltra r=3 g=1u len=10"), &[1.0e3])
        .expect("a finite-length RG line has native execution stamps");

    let error = engine
        .run_ac(
            &deck(".model line ltra r=3 g=1u l=1n c=1p len=10"),
            &[1.0e3],
        )
        .expect_err("RLGC with nonzero G has no reference implementation to match");
    assert!(
        error.to_string().contains("finite nonzero G"),
        "the rejection must name the parameter that has no reference semantics: {error}"
    );
}

#[test]
fn zero_length_rc_and_rg_remain_exact_small_signal_through_connections() {
    let netlist = Netlist::parse(
        r#"zero-length LTRA through cases
VIN in 0 AC 1
ORC in 0 out_rc 0 rc
ORG in 0 out_rg 0 rg
RRC out_rc 0 50
RRG out_rg 0 75
.model rc ltra r=0.05 c=20p len=0
.model rg ltra r=0.05 g0=20 len=0
.end
"#,
    )
    .expect("zero-length deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[1.0, 1.0e9])
        .expect("qualified zero-length RC/RG cards still execute");

    for point in result {
        for node in ["in", "out_rc", "out_rg"] {
            let index = point
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(node))
                .unwrap_or_else(|| panic!("missing node {node}"));
            assert!(
                (point.voltages[index].re - 1.0).abs() < 1.0e-12
                    && point.voltages[index].im.abs() < 1.0e-12,
                "{node} at {} Hz was {:?}",
                point.frequency,
                point.voltages[index]
            );
        }
    }
}

#[test]
fn matched_lossless_ltra_halves_the_drive_at_both_ends() {
    // A 50 ohm LC line (r=0, g=0) between a 50 ohm source and a 50 ohm load
    // sees the drive split exactly in half at its near end, and delivers that
    // half TD = len*sqrt(L*C) = 5 ns later at its far end. ngspice-46,
    // Xyce 7.10 and LTspice 26.1.1 all print 0.5000000 on both plateaus. The
    // RLC kernel's beta = 0 h1dash term used to double the line's admittance,
    // which put 1/3 V on both.
    let deck = Netlist::parse(
        "matched lossless ltra
         v1 in 0 pulse(0 1 1n 1n 1n 5n 20n)
         r1 in a 50
         o1 a 0 b 0 lline
         r2 b 0 50
         .model lline ltra r=0 g=0 l=250n c=100p len=1
         .tran 0.1n 20n
         .end
",
    )
    .expect("matched line deck parses");
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        let result = engine
            .run_tran(&deck, 20.0e-9, 0.1e-9)
            .expect("the matched line transient completes");
        let near = result.try_voltage_waveform_named("a").expect("v(a)");
        let far = result.try_voltage_waveform_named("b").expect("v(b)");
        let mut checked = 0;
        for (index, &time) in result.time.iter().enumerate() {
            for (plateau, wave) in [((2.2e-9, 6.8e-9), near), ((7.2e-9, 11.8e-9), far)] {
                if time > plateau.0 && time < plateau.1 {
                    checked += 1;
                    assert!(
                        (wave[index] - 0.5).abs() <= 0.5e-6,
                        "{dialect:?} at {time:e}: {:e}",
                        wave[index]
                    );
                }
            }
        }
        assert!(checked > 40, "{dialect:?}: {checked} plateau points");
    }
}

#[test]
fn xyce_dialect_coupled_lines_run_through_a_diode_reverse_recovery_snap() {
    // Xyce's LTRA3 coupled-line deck cut to its diode load. d1's stored charge
    // (TT=0.75n, CJO=0) holds node 1 above the 2.71 V the diode chain sets,
    // until it runs out at 7.33 ns and the node snaps down within attoseconds.
    // Those steps left exactly zero RLC convolution coefficients that cut the
    // line's own history, and the Xyce-dialect run stalled at 7.3317 ns.
    // LTspice 26.1.1: v(x3:1) = 3.1097 V at 7.0 ns, 3.3966 V at 7.3 ns, then
    // 2.703-2.715 V. Xyce 7.10 stays at 2.707 V from 6.8 ns because it drops
    // TT charge when CJO=0 (N_DEV_Diode.C:1351-1412).
    let deck = Netlist::parse(
        "coupled lossy lines into a recovering diode load
         v1 1 0 pulse(0 4 1ns 1ns 1ns 20ns 40ns)
         vcc 10 0 5
         r1 1 2 50
         rt1 3 0 50
         x2 2 3 4 5 conv2wetcmodel
         x3 4 dioload
         rt2 5 0 50
         .model diod1 d(tt=0.75ns vj=0.6 rs=909 bv=10)
         .subckt dioload 28
         vccint 5 0 5
         c1 28 0 5pF
         r503 0 4 5.55
         r4 0 28 120k
         r5 1 5 7.5k
         d1 1 28 diod1
         d4 2 0 diod1
         d3 3 2 diod1
         d2 1 3 diod1
         .ends dioload
         .model mod1 ltra rel=1.2 r=0.2 l=4.72933999088e-09 g=0 c=7.25000000373e-12 len=20
         .model mod2 ltra rel=1.2 r=0.2 l=1.35306599818e-08 g=0 c=3.65000000746e-12 len=20
         .subckt modal 1 2 3 4
         v1 5 0 0
         v2 6 0 0
         f1 0 3 v1 0.707106779721
         f2 0 3 v2 -0.707106782652
         f3 0 4 v1 0.707106781919
         f4 0 4 v2 0.707106780454
         e1 7 5 3 0 0.707106780454
         e2 1 7 4 0 0.707106782652
         e3 8 6 3 0 -0.707106781919
         e4 2 8 4 0 0.707106779721
         .ends modal
         .subckt coupled 1 2 3 4
         x1 1 2 5 6 modal
         o1 5 0 7 0 mod1
         o2 6 0 8 0 mod2
         x2 3 4 7 8 modal
         .ends coupled
         .model short ltra r=0.2 l=9.13e-9 c=3.65e-12 len=2.0 rel=1.2 nocontrol
         .subckt conv2wetcmodel 1 2 3 4
         o1 1 0 5 0 short
         x1 5 2 6 4 coupled
         o2 6 0 3 0 short
         .ends conv2wetcmodel
         .tran 0.1ns 9ns 0 0.1ns
         .end
",
    )
    .expect("coupled line deck parses");
    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
    let result = engine
        .run_tran(&deck, 9.0e-9, 0.1e-9)
        .expect("the Xyce-dialect run passes the snap");
    let node = result.try_voltage_waveform_named("x3.1").expect("v(x3:1)");
    let at = |time: f64| {
        let k = result
            .time
            .partition_point(|t| *t < time)
            .clamp(1, result.time.len() - 1);
        let (t0, t1) = (result.time[k - 1], result.time[k]);
        node[k - 1] + (time - t0) / (t1 - t0) * (node[k] - node[k - 1])
    };
    assert!(
        (at(7.0e-9) - 3.1097).abs() < 0.01,
        "stored-charge rise {}",
        at(7.0e-9)
    );
    assert!(
        (at(7.3e-9) - 3.3966).abs() < 0.02,
        "before the snap {}",
        at(7.3e-9)
    );
    for (index, &time) in result.time.iter().enumerate() {
        if time >= 7.6e-9 {
            assert!(
                (node[index] - 2.709).abs() < 0.01,
                "after the snap at {time:e}: {}",
                node[index]
            );
        }
    }
}
