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
