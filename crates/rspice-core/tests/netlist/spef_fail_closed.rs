//! Fail-closed contracts for path-backed SPEF parasitic annotation.
//!
//! Unsupported or malformed parasitic data must never yield a plausible
//! unannotated circuit.  These tests exercise the public netlist and engine
//! entry points rather than the private SPEF parser implementation.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use rspice_core::abort_signal::CountingAbort;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{Netlist, ParseWithAbortError, spef::SpefFile};

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

fn reduced_pi_spef() -> &'static str {
    "\
*SPEF \"IEEE 1481-2009\"
*C_UNIT 1 NF
*R_UNIT 1 KOHM
*R_NET in 3
*DRIVER I1:N
*CELL CURRENT_SOURCE
*C2_R1_C1 1 2 2
*LOADS
*RC ILOAD:P 4
*END
"
}

fn detailed_pi_spef() -> &'static str {
    "\
*SPEF \"IEEE 1481-2009\"
*C_UNIT 1 NF
*R_UNIT 1 KOHM
*D_NET in 3
*CONN
*P in O
*I ILOAD:P I
*CAP
1 in 1
2 ILOAD:P 2
*RES
1 in ILOAD:P 2
*END
"
}

fn ac_voltage(netlist: &Netlist, frequency: f64, node: &str) -> rspice_core::Complex64 {
    let point = Engine::default()
        .run_ac(netlist, &[frequency])
        .expect("annotated RC network executes")
        .pop()
        .expect("one AC point");
    point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .map(|index| point.voltages[index])
        .unwrap_or_else(|| panic!("node `{node}` is present: {:?}", point.node_names))
}

#[test]
fn reduced_r_net_matches_detailed_spef_and_closed_form_ac_impedance() {
    let reduced_fixture = SpefFixture::new("reduced-r-ac", reduced_pi_spef());
    let detailed_fixture = SpefFixture::new("detailed-r-ac", detailed_pi_spef());
    let deck = "\
Reduced SPEF pi AC oracle
I1 0 in DC 0 AC 1
ILOAD in 0 DC 0
.spef_include \"parasitics.spef\"
.ac lin 1 1k 1k
.end
";
    let reduced =
        Netlist::parse_with_path(deck, &reduced_fixture.deck_path).expect("reduced SPEF imports");
    let detailed = Netlist::parse_with_path(deck, &detailed_fixture.deck_path)
        .expect("equivalent detailed SPEF imports");

    let c2 = 1.0e-9;
    let resistance = 2.0e3;
    let c1 = 2.0e-9;
    for frequency in [1.0e3, 50.0e3, 1.0e6] {
        let reduced_voltage = ac_voltage(&reduced, frequency, "in");
        let detailed_voltage = ac_voltage(&detailed, frequency, "in");
        let jw = rspice_core::Complex64::new(0.0, 2.0 * std::f64::consts::PI * frequency);
        let expected = rspice_core::Complex64::new(1.0, 0.0)
            / (jw * c2
                + rspice_core::Complex64::new(1.0, 0.0)
                    / (rspice_core::Complex64::new(resistance, 0.0)
                        + rspice_core::Complex64::new(1.0, 0.0) / (jw * c1)));
        let tolerance = 2.0e-10 * expected.norm().max(1.0);
        assert!(
            (reduced_voltage - detailed_voltage).norm() <= tolerance,
            "reduced/detailed mismatch at {frequency:.3e} Hz: reduced={reduced_voltage:?}, detailed={detailed_voltage:?}"
        );
        assert!(
            (reduced_voltage - expected).norm() <= tolerance,
            "closed-form mismatch at {frequency:.3e} Hz: actual={reduced_voltage:?}, expected={expected:?}"
        );
    }
}

#[test]
fn reduced_r_net_matches_detailed_spef_and_closed_form_transient_step() {
    let reduced_spef = reduced_pi_spef()
        .replace("I1:N", "V1:P")
        .replace("CURRENT_SOURCE", "VOLTAGE_SOURCE");
    let reduced_fixture = SpefFixture::new("reduced-r-tran", &reduced_spef);
    let detailed_fixture = SpefFixture::new("detailed-r-tran", detailed_pi_spef());
    let deck = "\
Reduced SPEF pi transient oracle
V1 in 0 PULSE(0 1 1u 1n 1n 1 2)
ILOAD in 0 DC 0
.spef_include \"parasitics.spef\"
.tran 100n 20u
.end
";
    let reduced =
        Netlist::parse_with_path(deck, &reduced_fixture.deck_path).expect("reduced SPEF imports");
    let detailed = Netlist::parse_with_path(deck, &detailed_fixture.deck_path)
        .expect("equivalent detailed SPEF imports");
    let reduced_load_node = reduced
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case("ILOAD"))
        .and_then(|element| element.nodes.first())
        .cloned()
        .expect("reduced load was retained");
    let detailed_load_node = detailed
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case("ILOAD"))
        .and_then(|element| element.nodes.first())
        .cloned()
        .expect("detailed load was retained");
    let reduced_result = Engine::default()
        .run_tran(&reduced, 20.0e-6, 100.0e-9)
        .expect("reduced RC transient converges");
    let detailed_result = Engine::default()
        .run_tran(&detailed, 20.0e-6, 100.0e-9)
        .expect("detailed RC transient converges");
    assert_eq!(reduced_result.time, detailed_result.time);
    let reduced_node = reduced_result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(&reduced_load_node))
        .expect("reduced far node is present");
    let detailed_node = detailed_result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(&detailed_load_node))
        .expect("detailed far node is present");
    let edge_midpoint = 1.0005e-6;
    let time_constant = 2.0e3 * 2.0e-9;
    let mut maximum_oracle_error = 0.0_f64;
    for ((&time, &reduced_voltage), &detailed_voltage) in reduced_result
        .time
        .iter()
        .zip(&reduced_result.voltages[reduced_node])
        .zip(&detailed_result.voltages[detailed_node])
    {
        assert!(
            (reduced_voltage - detailed_voltage).abs() <= 2.0e-10,
            "reduced/detailed transient mismatch at {time:.3e} s"
        );
        if time >= 1.2e-6 {
            let expected = 1.0 - (-(time - edge_midpoint) / time_constant).exp();
            maximum_oracle_error = maximum_oracle_error.max((reduced_voltage - expected).abs());
        }
    }
    assert!(
        maximum_oracle_error < 4.0e-3,
        "reduced RC transient differs from the closed form by {maximum_oracle_error:.3e} V"
    );
}

#[test]
fn reduced_lumped_nets_match_closed_form_ac_impedance() {
    let deck = "\
Reduced SPEF lumped capacitance
I1 0 in DC 0 AC 1
.spef_include \"parasitics.spef\"
.ac lin 1 20k 20k
.end
";
    for (label, keyword) in [("reduced-c-ac", "*C_NET"), ("lumped-r-ac", "*R_NET")] {
        let fixture = SpefFixture::new(
            label,
            &format!("*SPEF \"IEEE 1481-2009\"\n*C_UNIT 1 NF\n{keyword} in 2.5\n*END\n"),
        );
        let netlist = Netlist::parse_with_path(deck, &fixture.deck_path)
            .unwrap_or_else(|error| panic!("lumped {keyword} imports: {error}"));
        let frequency = 20.0e3;
        let actual = ac_voltage(&netlist, frequency, "in");
        let expected = rspice_core::Complex64::new(
            0.0,
            -1.0 / (2.0 * std::f64::consts::PI * frequency * 2.5e-9),
        );
        assert!(
            (actual - expected).norm() <= 2.0e-10 * expected.norm().max(1.0),
            "lumped {keyword} mismatch: actual={actual:?}, expected={expected:?}"
        );
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
            "*SPEF \"IEEE 1481-2009\"\n*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*L_UNIT 1 HENRY\n*D_NET in 0\n{section}\n{record}\n*END\n"
        );
        assert_path_backed_spef_error(label, &spef, expected);
    }
}

#[test]
fn reduced_records_fail_closed_on_zero_extreme_malformed_and_nonconserving_values() {
    let cases = [
        (
            "zero-c-net",
            "*C_UNIT 1 PF\n*C_NET in 0\n*END\n",
            "STRICTLY POSITIVE",
        ),
        (
            "zero-c2",
            "*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*R_NET in 2\n*DRIVER V1:1\n*CELL VOLTAGE_SOURCE\n*C2_R1_C1 0 1 2\n*LOADS\n*RC RLOAD:1 2\n*END\n",
            "C2",
        ),
        (
            "zero-r1",
            "*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*R_NET in 2\n*DRIVER V1:1\n*CELL VOLTAGE_SOURCE\n*C2_R1_C1 1 0 1\n*LOADS\n*RC RLOAD:1 1\n*END\n",
            "R1",
        ),
        (
            "zero-rc",
            "*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*R_NET in 2\n*DRIVER V1:1\n*CELL VOLTAGE_SOURCE\n*C2_R1_C1 1 1 1\n*LOADS\n*RC RLOAD:1 0\n*END\n",
            "ELMORE",
        ),
        (
            "extreme-cap",
            "*C_UNIT 1e308 F\n*C_NET in 10\n*END\n",
            "NON-FINITE",
        ),
        (
            "malformed-unit",
            "*C_UNIT 1 PF EXTRA\n*C_NET in 1\n*END\n",
            "EXACTLY",
        ),
        (
            "invalid-routing-confidence",
            "*C_UNIT 1 PF\n*C_NET in 1 *V 0\n*END\n",
            "POSITIVE INTEGER",
        ),
        (
            "c-net-body",
            "*C_UNIT 1 PF\n*C_NET in 1\n*CAP\n*END\n",
            "UNEXPECTED",
        ),
        (
            "unknown-r-record",
            "*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*R_NET in 2\n*FOO value\n*END\n",
            "UNEXPECTED",
        ),
        (
            "nonconserving-pi",
            "*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*R_NET in 3\n*DRIVER V1:1\n*CELL VOLTAGE_SOURCE\n*C2_R1_C1 1 1 1\n*LOADS\n*RC RLOAD:1 1\n*END\n",
            "CONSERVED",
        ),
        (
            "unknown-driver-cell",
            "*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*R_NET in 2\n*DRIVER V1:1\n*CELL UNKNOWN_DRIVER\n*C2_R1_C1 1 1 1\n*LOADS\n*RC RLOAD:1 1\n*END\n",
            "UNKNOWN_DRIVER",
        ),
        (
            "missing-load-record",
            "*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*R_NET in 2\n*DRIVER V1:1\n*CELL VOLTAGE_SOURCE\n*C2_R1_C1 1 1 1\n*LOADS\n*END\n",
            "AT LEAST ONE",
        ),
        (
            "duplicate-load",
            "*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*R_NET in 2\n*DRIVER V1:1\n*CELL VOLTAGE_SOURCE\n*C2_R1_C1 1 1 1\n*LOADS\n*RC RLOAD:1 1\n*RC RLOAD:1 1\n*END\n",
            "DUPLICATE LOAD",
        ),
        (
            "pole-residue",
            "*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*R_NET in 2\n*DRIVER V1:1\n*CELL VOLTAGE_SOURCE\n*C2_R1_C1 1 1 1\n*LOADS\n*RC RLOAD:1 1\n*Q 1 -1\n*END\n",
            "POLE/RESIDUE",
        ),
        (
            "unrealizable-load-delay",
            "*C_UNIT 1 PF\n*R_UNIT 1 KOHM\n*R_NET in 2\n*DRIVER V1:1\n*CELL VOLTAGE_SOURCE\n*C2_R1_C1 1 2 1\n*LOADS\n*RC RLOAD:1 9\n*END\n",
            "CANNOT BE REPRESENTED",
        ),
    ];
    for (label, body, expected) in cases {
        let spef = format!("*SPEF \"IEEE 1481-2009\"\n{body}");
        assert_path_backed_spef_error(label, &spef, expected);
    }
}

#[test]
fn reduced_records_reject_unknown_or_hierarchically_unresolved_nodes() {
    for (label, driver, load, expected) in [
        ("unknown-driver", "MISSING:1", "RLOAD:1", "MISSING"),
        ("unknown-load", "V1:1", "MISSING:1", "MISSING"),
        ("unresolved-hierarchy", "TOP/V1:1", "RLOAD:1", "TOP/V1"),
    ] {
        let spef = format!(
            "*SPEF \"IEEE 1481-2009\"\n*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*R_NET in 2\n*DRIVER {driver}\n*CELL VOLTAGE_SOURCE\n*C2_R1_C1 1 1 1\n*LOADS\n*RC {load} 1\n*END\n"
        );
        assert_path_backed_spef_error(label, &spef, expected);
    }
    assert_path_backed_spef_error(
        "unknown-c-net",
        "*SPEF \"IEEE 1481-2009\"\n*C_UNIT 1 PF\n*C_NET ghost 1\n*END\n",
        "GHOST",
    );
}

#[test]
fn reduced_driver_cell_must_match_the_resolved_subcircuit_type() {
    let spef = "\
*SPEF \"IEEE 1481-2009\"
*C_UNIT 1 PF
*R_UNIT 1 OHM
*R_NET in 2
*DRIVER XDRV:Y
*CELL WRONG_CELL
*C2_R1_C1 1 1 1
*LOADS
*RC RLOAD:1 1
*END
";
    let fixture = SpefFixture::new("cell-mismatch", spef);
    let deck = "\
Reduced cell validation
.subckt REAL_CELL Y
R1 Y 0 1meg
.ends
XDRV in REAL_CELL
RLOAD in 0 1k
.spef_include \"parasitics.spef\"
.op
.end
";

    let error = Netlist::parse_with_path(deck, &fixture.deck_path)
        .expect_err("mismatched reduced driver cell must fail closed")
        .to_string();

    assert!(error.contains("WRONG_CELL"), "unexpected error: {error}");
    assert!(error.contains("REAL_CELL"), "unexpected error: {error}");
}

#[test]
fn reduced_native_driver_cell_must_match_the_exact_pseudo_cell_type() {
    let spef = "\
*SPEF \"IEEE 1481-2009\"
*C_UNIT 1 PF
*R_UNIT 1 OHM
*R_NET in 2
*DRIVER V1:P
*CELL CURRENT_SOURCE
*C2_R1_C1 1 1 1
*LOADS
*RC RLOAD:1 1
*END
";

    assert_path_backed_spef_error("native-cell-mismatch", spef, "VOLTAGE_SOURCE");
}

#[test]
fn reduced_lowering_is_deterministic_collision_safe_and_source_provenanced() {
    let spef = reduced_pi_spef();
    let fixture = SpefFixture::new("reduced-collisions", spef);
    let deck = "\
Reduced SPEF collision guards
I1 0 in DC 0 AC 1
ILOAD in 0 DC 0
RKEEP __SPEF_REDUCED_IN 0 1meg
CSPEF1 spare 0 1p
CSPEF2 spare 0 1p
RSPEF4 spare 0 1
CSPEF6 spare 0 1p
.spef_include \"parasitics.spef\"
.ac lin 1 1k 1k
.end
";
    let netlist =
        Netlist::parse_with_path(deck, &fixture.deck_path).expect("collision-safe import succeeds");
    let names: Vec<_> = netlist
        .elements
        .iter()
        .map(|element| element.name.to_ascii_uppercase())
        .collect();
    let unique: std::collections::HashSet<_> = names.iter().collect();
    assert_eq!(unique.len(), names.len(), "all element names remain unique");
    for generated in ["CSPEF3", "RSPEF5", "CSPEF7"] {
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(generated))
            .unwrap_or_else(|| panic!("expected deterministic element `{generated}`"));
        assert!(matches!(
            &element.provenance,
            rspice_core::netlist::ElementProvenance::ImportedSpef {
                net,
                record_id: None,
                line: 4,
            } if net == "in"
        ));
    }
    let load_node = netlist
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case("ILOAD"))
        .and_then(|element| element.nodes.first())
        .expect("load node exists");
    assert_eq!(load_node, "__SPEF_REDUCED_IN_1");
}

#[test]
fn reduced_parsing_and_lowering_are_cooperatively_cancellable_and_transactional() {
    let mut spef_text = String::from(
        "*SPEF \"IEEE 1481-2009\"\n*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*R_NET in 2\n*DRIVER V1:1\n*CELL VOLTAGE_SOURCE\n*C2_R1_C1 1 1 1\n*LOADS\n",
    );
    let mut deck = String::from("reduced cancellation\nV1 in 0 DC 0\n");
    for index in 0..1_024 {
        spef_text.push_str(&format!("*RC R{index}:1 1\n"));
        deck.push_str(&format!("R{index} in 0 1meg\n"));
    }
    spef_text.push_str("*END\n");
    deck.push_str(".op\n.end\n");

    let parse_abort = CountingAbort::new(80);
    let parse_result = SpefFile::parse_with_abort(&spef_text, &parse_abort);
    assert!(matches!(parse_result, Err(ParseWithAbortError::Aborted)));
    assert!(parse_abort.count() > 80, "parsing must poll during work");

    let spef = SpefFile::parse(&spef_text).expect("large reduced fixture parses");
    let mut netlist = Netlist::parse(&deck).expect("large fixture deck parses");
    let original_nodes: Vec<_> = netlist
        .elements
        .iter()
        .map(|element| element.nodes.clone())
        .collect();
    let apply_abort = CountingAbort::new(24);
    let apply_result = spef.apply_with_abort(&mut netlist, &apply_abort);
    assert!(matches!(apply_result, Err(ParseWithAbortError::Aborted)));
    assert!(apply_abort.count() > 24, "lowering must poll during work");
    assert_eq!(
        netlist
            .elements
            .iter()
            .map(|element| element.nodes.clone())
            .collect::<Vec<_>>(),
        original_nodes,
        "aborted lowering must not publish partial rewiring"
    );
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
    assert!(matches!(
        &inductor.provenance,
        rspice_core::netlist::ElementProvenance::ImportedSpef {
            net,
            record_id: Some(1),
            ..
        } if net == "in"
    ));

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

fn detailed_series_rlc_spef() -> &'static str {
    "\
*SPEF \"IEEE 1481-2009\"
*C_UNIT 1 UF
*R_UNIT 1 OHM
*L_UNIT 1 MH
*D_NET in 1
*CAP
1 in:2 1
*RES
1 in in:1 63.245553203367585
*INDUC
1 in:1 in:2 1
*END
"
}

#[test]
fn detailed_rlc_spef_matches_analytical_impedance_and_admittance_over_frequency() {
    let fixture = SpefFixture::new("rlc-ac", detailed_series_rlc_spef());
    let deck = "\
SPEF detailed RLC impedance
I1 0 in DC 0 AC 1
.spef_include \"parasitics.spef\"
.ac dec 10 100 1meg
.end
";
    let netlist = Netlist::parse_with_path(deck, &fixture.deck_path)
        .expect("supported detailed RLC SPEF imports");
    let resistance = 63.245553203367585;
    let inductance = 1.0e-3;
    let capacitance = 1.0e-6;

    for frequency in [100.0, 1.0e3, 5.0e3, 50.0e3, 1.0e6] {
        let impedance = ac_voltage(&netlist, frequency, "in");
        let omega = 2.0 * std::f64::consts::PI * frequency;
        let expected = rspice_core::Complex64::new(
            resistance,
            omega * inductance - 1.0 / (omega * capacitance),
        );
        let tolerance = 2.0e-10 * expected.norm().max(1.0);
        assert!(
            (impedance - expected).norm() <= tolerance,
            "RLC impedance mismatch at {frequency:.3e} Hz: actual={impedance:?}, expected={expected:?}"
        );
        let admittance = rspice_core::Complex64::new(1.0, 0.0) / impedance;
        let expected_admittance = rspice_core::Complex64::new(1.0, 0.0) / expected;
        assert!(
            (admittance - expected_admittance).norm()
                <= 2.0e-10 * expected_admittance.norm().max(1.0),
            "RLC admittance mismatch at {frequency:.3e} Hz"
        );
    }
}

#[test]
fn detailed_rlc_spef_matches_critical_step_response() {
    let fixture = SpefFixture::new("rlc-tran", detailed_series_rlc_spef());
    let deck = "\
SPEF detailed RLC critical step
V1 in 0 PULSE(0 1 1u 1n 1n 1 2)
.spef_include \"parasitics.spef\"
.tran 250n 300u
.end
";
    let netlist = Netlist::parse_with_path(deck, &fixture.deck_path)
        .expect("supported detailed RLC SPEF imports");
    let result = Engine::default()
        .run_tran(&netlist, 300.0e-6, 250.0e-9)
        .expect("annotated RLC transient converges");
    let node = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("in__2"))
        .expect("RLC capacitor node is present");
    let omega0 = 1.0 / (1.0e-3_f64 * 1.0e-6).sqrt();
    let edge_midpoint = 1.0005e-6;
    let mut maximum_error = 0.0_f64;
    for (&time, &actual) in result.time.iter().zip(&result.voltages[node]) {
        assert!(
            actual.is_finite(),
            "non-finite RLC response at {time:.3e} s"
        );
        if time >= 2.0e-6 {
            let elapsed = time - edge_midpoint;
            let expected = 1.0 - (-omega0 * elapsed).exp() * (1.0 + omega0 * elapsed);
            maximum_error = maximum_error.max((actual - expected).abs());
        }
    }
    assert!(
        maximum_error < 2.0e-3,
        "SPEF RLC transient differs from the critical closed form by {maximum_error:.3e} V"
    );
}

#[test]
fn hierarchical_spef_identity_survives_generated_node_collisions_and_provenance() {
    let spef = "\
*SPEF \"IEEE 1481-2009\"
*L_UNIT 1 UH
*D_NET top/block/net[3] 0
*INDUC
9 top/block/net[3] top/block/net[3]:1 2
*END
";
    let fixture = SpefFixture::new("hierarchical-identity", spef);
    let deck = "\
SPEF hierarchical identity
I1 0 top/block/net[3] DC 0 AC 1
RKEEP TOP_BLOCK_NET_3___1 0 1meg
.spef_include \"parasitics.spef\"
.ac lin 1 1k 1k
.end
";
    let netlist = Netlist::parse_with_path(deck, &fixture.deck_path)
        .expect("hierarchical SPEF identity imports");
    let inductor = netlist
        .elements
        .iter()
        .find(|element| element.name.starts_with("LSPEF"))
        .expect("hierarchical SPEF inductor materialized");
    assert_eq!(inductor.nodes[0], "TOP/BLOCK/NET[3]");
    assert_ne!(
        inductor.nodes[1].to_ascii_uppercase(),
        "TOP_BLOCK_NET_3___1",
        "lossy sanitization must not capture an authored deck node"
    );
    assert!(inductor.nodes[1].starts_with("__SPEF_NODE__"));
    assert!(matches!(
        &inductor.provenance,
        rspice_core::netlist::ElementProvenance::ImportedSpef {
            net,
            record_id: Some(9),
            line: 5,
        } if net == "top/block/net[3]"
    ));
}
