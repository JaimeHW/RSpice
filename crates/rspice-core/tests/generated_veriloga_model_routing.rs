#![cfg(all(feature = "veriloga-builtins", rspice_veriloga_builtins_generated))]

use rspice_core::device::veriloga_generated::builtins;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn has_builtin(expected: &str) -> bool {
    builtins::builtin_names()
        .iter()
        .any(|name| name.eq_ignore_ascii_case(expected))
}

fn build(deck: &str) -> rspice_core::CircuitData {
    let netlist = Netlist::parse(deck).expect("deck parses");
    Engine::new(SimulationConfig::default())
        .build_circuit(&netlist)
        .expect("circuit builds")
}

#[test]
fn diode_level_2002_routes_to_generated_diode_cmc() {
    if !has_builtin("DIODE_CMC") {
        eprintln!("CMC diode builtin not present; skipping generated model routing test");
        return;
    }

    let circuit = build(
        r#"
v1 a 0 dc 0
d1 a 0 dcmc area=2
.model dcmc d level=2002
.op
.end
"#,
    );

    assert!(circuit.has_generated_veriloga_devices());
    assert!(
        circuit.diode_terminal_nodes("d1").is_none(),
        "CMC diode model should not also instantiate the native diode evaluator"
    );
    assert_eq!(
        circuit.device_count(),
        2,
        "device count should include the voltage source and routed generated diode"
    );
}

#[test]
fn bjt_hicum_level_230_routes_to_generated_hicum_l0() {
    if !has_builtin("hicumL0va") {
        eprintln!("HICUM/L0 builtin not present; skipping generated model routing test");
        return;
    }

    let circuit = build(
        r#"
v1 c 0 dc 0
q1 c b e qh area=1
.model qh npn level=230
.op
.end
"#,
    );

    assert!(circuit.has_generated_veriloga_devices());
    assert_eq!(circuit.device_count(), 2);
    assert!(
        circuit
            .device_op_report()
            .entries
            .iter()
            .all(|entry| !entry.name.eq_ignore_ascii_case("q1")),
        "HICUM model should not also instantiate the native BJT evaluator"
    );
}

#[test]
fn mos_model_type_bsimbulk_routes_to_generated_bsimbulk() {
    if !has_builtin("bsimbulk") {
        eprintln!("BSIM-BULK builtin not present; skipping generated model routing test");
        return;
    }

    let circuit = build(
        r#"
v1 d 0 dc 0
m1 d g s b nbulk w=1u l=1u
.model nbulk bsimbulk
.op
.end
"#,
    );

    assert!(circuit.has_generated_veriloga_devices());
    assert_eq!(circuit.device_count(), 2);
    assert!(
        circuit
            .device_op_report()
            .entries
            .iter()
            .all(|entry| !entry.name.eq_ignore_ascii_case("m1")),
        "BSIM-BULK model should not also instantiate the native MOS evaluator"
    );
}

#[test]
fn mos_ekv26_level_260_routes_to_generated_ekv26() {
    assert!(
        has_builtin("ekv_va"),
        "commercial-ready EKV2.6 source should be generated as ekv_va"
    );

    let circuit = build(
        r#"
v1 d 0 dc 0
m1 d g s b ekvmod w=1u l=1u
.model ekvmod nmos level=260 vto=0.5 kp=100u
.op
.end
"#,
    );

    assert!(circuit.has_generated_veriloga_devices());
    assert_eq!(circuit.device_count(), 2);
    assert!(
        circuit
            .device_op_report()
            .entries
            .iter()
            .all(|entry| !entry.name.eq_ignore_ascii_case("m1")),
        "EKV2.6 LEVEL=260 should not also instantiate the native EKV26 evaluator"
    );
}

#[test]
fn mos_bsimsoi_legacy_levels_route_to_generated_bsimsoi_461() {
    assert!(
        has_builtin("bsimsoi_va"),
        "commercial-ready BSIM-SOI 4.6.1 source should be generated as bsimsoi_va"
    );

    let circuit = build(
        r#"
v1 d 0 dc 0
m1 d g s e nbsimsoi w=1u l=1u
.model nbsimsoi nmos level=55
.op
.end
"#,
    );

    assert!(circuit.has_generated_veriloga_devices());
    assert_eq!(circuit.device_count(), 2);
    assert!(
        circuit
            .device_op_report()
            .entries
            .iter()
            .all(|entry| !entry.name.eq_ignore_ascii_case("m1")),
        "legacy BSIM-SOI LEVEL=55 should not also instantiate the native BSIM3-SOI evaluator"
    );
}

#[test]
fn mos_bsimcmg_binned_levels_route_by_l_and_nfin_to_generated_bsimcmg() {
    assert!(
        has_builtin("bsimcmg_va"),
        "commercial-ready BSIM-CMG source should be generated as bsimcmg_va"
    );

    let circuit = build(
        r#"
v1 d 0 dc 0
m1 d g s b nmos1 tfin=15n l=30n nfin=10 nrs=1 nrd=1
.model nmos1.1 nmos level=107 lmin=0 lmax=29n nfinmin=1 nfinmax=4 type=1
.model nmos1.2 nmos level=107 lmin=29n lmax=35n nfinmin=5 nfinmax=100 type=1
.op
.end
"#,
    );

    assert!(circuit.has_generated_veriloga_devices());
    assert_eq!(circuit.device_count(), 2);
    assert!(
        circuit
            .device_op_report()
            .entries
            .iter()
            .all(|entry| !entry.name.eq_ignore_ascii_case("m1")),
        "BSIM-CMG binned LEVEL=107 should not also instantiate a native MOS evaluator"
    );
}

#[test]
fn bjt_vbic_level_4_routes_to_generated_vbic13() {
    assert!(
        has_builtin("vbic13_4t"),
        "commercial-ready VBIC source should be generated as vbic13_4t"
    );

    let circuit = build(
        r#"
v1 c 0 dc 0
q1 c b e qv area=1
.model qv npn level=4
.op
.end
"#,
    );

    assert!(circuit.has_generated_veriloga_devices());
    assert_eq!(circuit.device_count(), 2);
    assert!(
        circuit
            .device_op_report()
            .entries
            .iter()
            .all(|entry| !entry.name.eq_ignore_ascii_case("q1")),
        "VBIC LEVEL=4 should not also instantiate the native BJT evaluator"
    );
}

#[test]
fn bjt_explicit_vbic13_4t_model_type_routes_to_generated_vbic13() {
    assert!(
        has_builtin("vbic13_4t"),
        "commercial-ready VBIC source should be generated as vbic13_4t"
    );

    let circuit = build(
        r#"
v1 c 0 dc 0
q1 c b e s qv area=1
.model qv vbic13_4t
.op
.end
"#,
    );

    assert!(circuit.has_generated_veriloga_devices());
    assert_eq!(circuit.device_count(), 2);
    assert!(
        circuit
            .device_op_report()
            .entries
            .iter()
            .all(|entry| !entry.name.eq_ignore_ascii_case("q1")),
        "explicit vbic13_4t model type should not also instantiate the native BJT evaluator"
    );
}

#[test]
fn mos_compact_angelov_routes_to_generated_three_terminal_model() {
    assert!(
        has_builtin("angelov"),
        "commercial-ready Angelov source should be generated as angelov"
    );

    let circuit = build(
        r#"
v1 d 0 dc 0
m1 d g s angelov_mod
.model angelov_mod angelov
.op
.end
"#,
    );

    assert!(circuit.has_generated_veriloga_devices());
    assert_eq!(circuit.device_count(), 2);
    assert!(
        circuit
            .device_op_report()
            .entries
            .iter()
            .all(|entry| !entry.name.eq_ignore_ascii_case("m1")),
        "Angelov three-terminal model should not also instantiate the native MOS evaluator"
    );
}

#[test]
fn mos_extended_angelov_gan_routes_to_generated_five_terminal_model() {
    assert!(
        has_builtin("angelov_gan"),
        "commercial-ready Angelov-GaN source should be generated as angelov_gan"
    );

    let circuit = build(
        r#"
v1 d 0 dc 0
m1 d g s t rf angelov_gan_mod
.model angelov_gan_mod angelov_gan
.op
.end
"#,
    );

    assert!(circuit.has_generated_veriloga_devices());
    assert_eq!(circuit.device_count(), 2);
    assert!(
        circuit
            .device_op_report()
            .entries
            .iter()
            .all(|entry| !entry.name.eq_ignore_ascii_case("m1")),
        "Angelov-GaN five-terminal model should not also instantiate the native MOS evaluator"
    );
}

#[test]
fn mos_compact_angelov_gan_routes_with_optional_terminals_grounded() {
    assert!(
        has_builtin("angelov_gan"),
        "commercial-ready Angelov-GaN source should be generated as angelov_gan"
    );

    let circuit = build(
        r#"
v1 d 0 dc 0
m1 d g s angelov_gan_mod
.model angelov_gan_mod angelov_gan
.op
.end
"#,
    );

    assert!(circuit.has_generated_veriloga_devices());
    assert_eq!(circuit.device_count(), 2);
    assert!(
        circuit
            .device_op_report()
            .entries
            .iter()
            .all(|entry| !entry.name.eq_ignore_ascii_case("m1")),
        "Angelov-GaN compact model should not also instantiate the native MOS evaluator"
    );
}

#[test]
fn mos_extended_epfl_hemt_routes_to_generated_when_source_is_present() {
    if !has_builtin("EPFL_HEMT_10a") {
        eprintln!("EPFL-HEMT builtin not present; skipping generated model routing test");
        return;
    }

    let circuit = build(
        r#"
v1 d 0 dc 0
m1 d g s b dt epfl_mod
.model epfl_mod epfl_hemt_10a
.op
.end
"#,
    );

    assert!(circuit.has_generated_veriloga_devices());
    assert_eq!(circuit.device_count(), 2);
    assert!(
        circuit
            .device_op_report()
            .entries
            .iter()
            .all(|entry| !entry.name.eq_ignore_ascii_case("m1")),
        "EPFL-HEMT five-terminal model should not also instantiate the native MOS evaluator"
    );
}

#[test]
fn mos_compact_epfl_hemt_routes_with_body_tied_to_source() {
    if !has_builtin("EPFL_HEMT_10a") {
        eprintln!("EPFL-HEMT builtin not present; skipping generated model routing test");
        return;
    }

    let circuit = build(
        r#"
v1 d 0 dc 0
m1 d g s epfl_mod
.model epfl_mod epfl_hemt_10a
.op
.end
"#,
    );

    assert!(circuit.has_generated_veriloga_devices());
    assert_eq!(circuit.device_count(), 2);
    assert!(
        circuit
            .device_op_report()
            .entries
            .iter()
            .all(|entry| !entry.name.eq_ignore_ascii_case("m1")),
        "EPFL-HEMT compact model should not also instantiate the native MOS evaluator"
    );
}

#[test]
fn mos_bsim6_level_77_stays_unrouted_without_exact_generated_source() {
    let deck = r#"
v1 d 0 dc 0
m1 d g s b nbulk w=1u l=1u
.model nbulk nmos level=77
.op
.end
"#;
    let netlist = Netlist::parse(deck).expect("deck parses");
    let error = Engine::new(SimulationConfig::default())
        .build_circuit(&netlist)
        .expect_err("BSIM6 LEVEL=77 must not be routed to BSIM-BULK");
    let message = error.to_string();
    assert!(
        message.contains("LEVEL=77"),
        "error should identify the unrouted BSIM6 selector, got {message}"
    );
}

#[test]
fn ordinary_mos_level_1_stays_on_native_mos_evaluator() {
    let circuit = build(
        r#"
v1 d 0 dc 0
m1 d g s b nch w=1u l=1u
.model nch nmos level=1 vto=0.7 kp=100u
.op
.end
"#,
    );

    assert!(!circuit.has_generated_veriloga_devices());
    assert!(
        circuit
            .device_op_report()
            .entries
            .iter()
            .any(|entry| entry.name.eq_ignore_ascii_case("m1")),
        "ordinary LEVEL=1 MOS should continue to use the native evaluator"
    );
}
