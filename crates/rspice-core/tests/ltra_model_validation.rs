//! Fail-closed scalar-LTRA model classification regressions.

use rspice_core::Netlist;
use rspice_core::engine::{Engine, SimulationConfig};

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
