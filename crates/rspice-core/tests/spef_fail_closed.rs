//! Fail-closed contracts for path-backed SPEF parasitic annotation.
//!
//! Unsupported or malformed parasitic data must never yield a plausible
//! unannotated circuit.  These tests exercise the public netlist and engine
//! entry points rather than the private SPEF parser implementation.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct SpefFixture {
    directory: PathBuf,
    deck_path: PathBuf,
}

impl SpefFixture {
    fn new(label: &str, spef: &str) -> Self {
        let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!(
            "rspice-spef-fail-closed-{label}-{}-{sequence}",
            std::process::id()
        ));
        std::fs::create_dir_all(&directory).expect("create isolated SPEF fixture directory");
        let deck_path = directory.join("circuit.cir");
        std::fs::write(directory.join("parasitics.spef"), spef).expect("write SPEF fixture");
        Self {
            directory,
            deck_path,
        }
    }
}

impl Drop for SpefFixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

fn annotated_deck(spef_name: &str) -> String {
    format!(
        "\
SPEF fail-closed fixture
V1 in 0 dc 1
RLOAD in 0 1k
.spef_include \"{spef_name}\"
.op
.end
"
    )
}

fn assert_path_backed_spef_error(label: &str, spef: &str, expected: &str) {
    let fixture = SpefFixture::new(label, spef);
    let deck = annotated_deck("parasitics.spef");
    let error = Netlist::parse_with_path(&deck, &fixture.deck_path)
        .expect_err("unsupported or invalid SPEF must fail closed");
    let message = error.to_string();
    assert!(
        message.to_ascii_uppercase().contains("SPEF"),
        "{label}: error must identify SPEF input, got: {message}"
    );
    assert!(
        message.to_ascii_uppercase().contains(expected),
        "{label}: error must identify {expected}, got: {message}"
    );
}

#[test]
fn path_backed_reduced_resistor_and_capacitor_nets_are_not_silently_skipped() {
    for (label, section) in [("reduced-r", "*R_NET"), ("reduced-c", "*C_NET")] {
        let spef = format!(
            "*SPEF \"IEEE 1481-2009\"\n*C_UNIT 1 PF\n*R_UNIT 1 OHM\n{section} in 1\n*END\n"
        );
        assert_path_backed_spef_error(label, &spef, section);
    }
}

#[test]
fn unmatched_spef_instance_and_pin_connections_are_rejected() {
    for (label, connection, expected) in [
        ("missing-instance", "MISSING:P", "MISSING"),
        ("invalid-pin", "RLOAD:Q", "RLOAD"),
    ] {
        let spef =
            format!("*SPEF \"IEEE 1481-2009\"\n*D_NET in 0\n*CONN\n*I {connection} I\n*END\n");
        assert_path_backed_spef_error(label, &spef, expected);
    }
}

#[test]
fn invalid_parasitic_values_are_rejected_instead_of_dropped() {
    for (label, section, record, expected) in [
        ("nan-cap", "*CAP", "1 in:1 NaN", "NAN"),
        ("infinite-res", "*RES", "1 in in:1 inf", "INF"),
        ("negative-cap", "*CAP", "1 in:1 -1", "-1"),
        ("negative-res", "*RES", "1 in in:1 -1", "-1"),
        ("negative-induc", "*INDUC", "1 in in:1 -1", "-1"),
    ] {
        let spef = format!(
            "*SPEF \"IEEE 1481-2009\"\n*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*D_NET in 0\n{section}\n{record}\n*END\n"
        );
        assert_path_backed_spef_error(label, &spef, expected);
    }
}

#[test]
fn invalid_inductance_units_and_records_are_rejected() {
    for (label, unit, record, expected) in [
        ("invalid-l-unit", "1 NH", "1 in in:1 1", "NH"),
        (
            "malformed-induc",
            "1 HENRY",
            "1 in in:1",
            "EXPECTED 4 FIELDS",
        ),
    ] {
        let spef = format!(
            "*SPEF \"IEEE 1481-2009\"\n*L_UNIT {unit}\n*D_NET in 0\n*INDUC\n{record}\n*END\n"
        );
        assert_path_backed_spef_error(label, &spef, expected);
    }
}

#[test]
fn missing_spef_file_is_rejected() {
    let fixture = SpefFixture::new("missing", "*SPEF \"unused\"\n");
    let deck = annotated_deck("does-not-exist.spef");
    let error = Netlist::parse_with_path(&deck, &fixture.deck_path)
        .expect_err("missing SPEF dependency must fail closed")
        .to_string();

    assert!(
        error.contains("does-not-exist.spef"),
        "missing dependency error must name the file, got: {error}"
    );
}

#[test]
fn pathless_spef_cannot_execute_as_an_unannotated_circuit() {
    let deck = annotated_deck("unresolved.spef");
    let parsed = match Netlist::parse(&deck) {
        Ok(netlist) => netlist,
        Err(_) => return,
    };

    Engine::new(SimulationConfig::default())
        .run_dc_op(&parsed)
        .expect_err("an unresolved pathless SPEF directive must not reach simulation");
}

#[test]
fn valid_detailed_rc_spef_still_imports_and_executes() {
    let spef = "\
*SPEF \"IEEE 1481-2009\"
*C_UNIT 1 PF
*R_UNIT 1 OHM
*D_NET in 1
*CONN
*P in I
*CAP
1 in:1 2
*RES
1 in in:1 100
*END
";
    let fixture = SpefFixture::new("valid", spef);
    let deck = annotated_deck("parasitics.spef");
    let netlist = Netlist::parse_with_path(&deck, &fixture.deck_path)
        .expect("supported detailed RC SPEF imports");

    assert!(
        netlist.elements.iter().any(|element| {
            element.name.starts_with("RSPEF") && element.nodes.iter().any(|node| node == "IN__1")
        }),
        "supported SPEF resistor was not retained: {:?}",
        netlist.elements
    );
    assert!(
        netlist.elements.iter().any(|element| {
            element.name.starts_with("CSPEF") && element.nodes.iter().any(|node| node == "IN__1")
        }),
        "supported SPEF capacitor was not retained: {:?}",
        netlist.elements
    );
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("supported detailed RC SPEF circuit executes");
    assert!(
        result.node_voltages.iter().all(|value| value.is_finite()),
        "valid SPEF circuit produced non-finite OP voltages: {:?}",
        result.node_voltages
    );
}

#[test]
fn detailed_rl_spef_honors_units_and_matches_analytical_ac_impedance() {
    let spef = "\
*SPEF \"IEEE 1481-2009\"
*R_UNIT 1 OHM
*L_UNIT 1 UH
*D_NET in 0
*CONN
*P in I
*RES
1 in:1 0 3
*INDUC
1 in in:1 2
*END
";
    let fixture = SpefFixture::new("valid-rl", spef);
    let deck = "\
SPEF detailed RL impedance
I1 0 in DC 0 AC 1
.spef_include \"parasitics.spef\"
.ac lin 1 100k 100k
.end
";
    let netlist = Netlist::parse_with_path(deck, &fixture.deck_path)
        .expect("supported detailed RL SPEF imports");

    let inductor = netlist
        .elements
        .iter()
        .find(|element| element.name.starts_with("LSPEF"))
        .expect("SPEF inductor is retained");
    let rspice_core::netlist::ElementKind::Inductor { value, .. } = &inductor.kind else {
        panic!("LSPEF element did not lower to an inductor: {inductor:?}");
    };
    assert_eq!(*value, 2.0e-6);

    let frequency = 100.0e3;
    let point = Engine::default()
        .run_ac(&netlist, &[frequency])
        .expect("annotated RL network executes")
        .pop()
        .expect("one AC point");
    let actual = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("in"))
        .map(|index| point.voltages[index])
        .expect("input node is present");
    let expected =
        rspice_core::Complex64::new(3.0, 2.0 * std::f64::consts::PI * frequency * 2.0e-6);
    let tolerance = 256.0 * f64::EPSILON * expected.norm().max(1.0);
    assert!(
        (actual - expected).norm() <= tolerance,
        "SPEF RL impedance mismatch: actual={actual:?}, expected={expected:?}, tolerance={tolerance:.3e}"
    );
}

#[test]
fn detailed_rl_spef_matches_analytical_transient_step_response() {
    let spef = "\
*SPEF \"IEEE 1481-2009\"
*R_UNIT 1 OHM
*L_UNIT 1 MH
*D_NET in 0
*CONN
*P in I
*RES
1 in in:1 35.19
*INDUC
1 in:1 0 10
*END
";
    let fixture = SpefFixture::new("transient-rl", spef);
    let deck = "\
SPEF detailed RL step
V1 in 0 PULSE(0 1 1u 1u 1u 1 2)
.spef_include \"parasitics.spef\"
.tran 10u 3m
.end
";
    let netlist = Netlist::parse_with_path(deck, &fixture.deck_path)
        .expect("supported detailed RL SPEF imports");
    let result = Engine::default()
        .run_tran(&netlist, 3.0e-3, 10.0e-6)
        .expect("annotated RL transient converges");
    let node = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("in__1"))
        .expect("SPEF internal node is present");
    let voltage = &result.voltages[node];
    let resistance = 35.19;
    let inductance = 10.0e-3;
    let edge_midpoint = 1.5e-6;
    let mut max_error = 0.0_f64;
    for (&time, &actual) in result.time.iter().zip(voltage) {
        assert!(
            actual.is_finite() && actual.abs() < 1.2,
            "SPEF RL response diverged at {time:.3e} s: {actual:.3e} V"
        );
        if time >= 10.0e-6 {
            let expected = (-(time - edge_midpoint) * resistance / inductance).exp();
            max_error = max_error.max((actual - expected).abs());
        }
    }
    assert!(
        max_error < 8.0e-3,
        "SPEF RL transient differs from the analytical response by {max_error:.3e} V"
    );
}
