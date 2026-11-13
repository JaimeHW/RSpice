//! `CircuitData::device_count()` answers "how many devices are in this
//! circuit", so it has to stay a whole inventory. A counter under that name
//! that quietly omits a device family is worse than no counter at all: every
//! caller reads the number as complete. These tests pin the count for the
//! device families that were once missing from the sum.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn build(deck: &str) -> rspice_core::circuit::CircuitData {
    let netlist = Netlist::parse(deck).expect("fixture deck parses");
    Engine::new(SimulationConfig::default())
        .build_circuit(&netlist)
        .expect("fixture deck builds")
}

/// The defect this inventory was written for: BSIM-SOI routed through
/// `b3soi_fd`, reported by `device_op_report()` and invisible to the counter.
#[test]
fn bsimsoi_level55_mosfet_is_counted() {
    let circuit = build(
        r#"* BSIM-SOI LEVEL=55 device count
V1 d 0 1
M1 d 0 0 0 nbsimsoi W=1u L=1u
.model nbsimsoi nmos level=55
.op
.end
"#,
    );
    assert_eq!(
        circuit.device_count(),
        2,
        "one source and one LEVEL=55 MOSFET"
    );
    assert!(
        circuit
            .device_op_report()
            .entries
            .iter()
            .any(|entry| entry.name.eq_ignore_ascii_case("M1") && entry.device_kind == "B3SOIFD"),
        "the LEVEL=55 fixture must still route to the native B3SOIFD family"
    );
}

#[test]
fn compact_mosfet_families_are_counted() {
    for (level, kind) in [
        (8, "BSIM3"),
        (14, "BSIM4"),
        (10, "B3SOIPD"),
        (57, "B3SOIPD"),
    ] {
        let circuit = build(&format!(
            r#"* compact MOSFET device count
V1 d 0 1
M1 d 0 0 0 nmod W=1u L=1u
.model nmod nmos level={level}
.op
.end
"#
        ));
        assert_eq!(
            circuit.device_count(),
            2,
            "one source and one LEVEL={level} MOSFET"
        );
        assert!(
            circuit
                .device_op_report()
                .entries
                .iter()
                .any(|entry| entry.name.eq_ignore_ascii_case("M1") && entry.device_kind == kind),
            "LEVEL={level} must still route to {kind}"
        );
    }
}

#[test]
fn switches_lines_and_behavioral_sources_are_counted() {
    let switched = build(
        r#"* voltage-controlled switch device count
V1 a 0 1
Vc c 0 1
S1 a b c 0 SMOD
R1 b 0 1k
.model SMOD SW vt=0.5
.op
.end
"#,
    );
    assert_eq!(switched.device_count(), 4, "two sources, one switch, one R");

    let line = build(
        r#"* transmission line device count
V1 a 0 1
T1 a 0 b 0 Z0=50 TD=1n
R1 b 0 50
.op
.end
"#,
    );
    assert_eq!(line.device_count(), 3, "one source, one T line, one R");

    let behavioral = build(
        r#"* behavioral source device count
V1 a 0 1
B1 b 0 V=V(a)*2
R1 b 0 1k
.op
.end
"#,
    );
    assert_eq!(
        behavioral.device_count(),
        3,
        "one source, one B element, one R"
    );
}

/// One K card is one device however many mutual pairs realize it, and a
/// magnetic-core model on an L card does not make that inductor two devices.
#[test]
fn a_magnetic_card_is_counted_once_however_it_is_realized() {
    let coupled = build(
        r#"* three-way coupling device count
V1 a 0 1
L1 a 0 1m
L2 b 0 1m
L3 c 0 1m
R2 b 0 1k
R3 c 0 1k
K1 L1 L2 L3 0.5
.op
.end
"#,
    );
    assert_eq!(
        coupled.device_count(),
        7,
        "one source, three inductors, two resistors and one K card -- not its three pairs"
    );

    let hysteretic = build(
        r#"* Jiles-Atherton inductor device count
V1 a 0 1
L1 a 0 1m JAMOD
.model JAMOD CORE ms=1e6 a=100 k=100 c=0.1 alpha=1e-6 area=1e-4 len=0.1 n=10
.op
.end
"#,
    );
    assert_eq!(
        hysteretic.device_count(),
        2,
        "one source and one inductor, whose core model is an overlay and not a second device"
    );

    let single_winding_core = build(
        r#"single winding nonlinear CORE
VS 1 0 1
R1 1 2 1K
L1 2 0 200
K1 L1 1 nlcore
.model nlcore core gap=.1 path=1 area=1e-4
.op
.end
"#,
    );
    assert_eq!(
        single_winding_core.device_count(),
        4,
        "one source, one resistor, one winding, and the K card that gives it a core"
    );

    let multi_winding_core = build(
        r#"generic nonlinear CORE family
VS 1 0 1
R1 1 2 1K
R2 3 0 1K
L1 2 0 200
L2 3 0 100
K1 L1 L2 1 nlcore
.model nlcore core gap=.1 path=1 area=1e-4
.op
.end
"#,
    );
    assert_eq!(
        multi_winding_core.device_count(),
        6,
        "one source, two resistors, two windings, and one shared core -- not one core per winding"
    );
}
