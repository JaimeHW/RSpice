//! Circuit-build boundaries for physical parameters consumed without
//! approximation by ordinary and exact-HB device laws.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn build(deck: &str) -> Result<(), String> {
    let netlist = Netlist::parse(deck).expect("model-boundary fixture parses");
    Engine::new(SimulationConfig::default())
        .build_circuit(&netlist)
        .map(|_| ())
        .map_err(|error| error.to_string())
}

#[test]
fn voltage_switch_accepts_jointly_representable_extremes() {
    build(
        "\
* extreme representable voltage switch
S1 out 0 ctrl 0 SMOD
VCTRL ctrl 0 DC 0
R1 out 0 1k
.model SMOD VSWITCH (RON=1e-300 ROFF=1e300 SMOOTH=1e-8)
.end
",
    )
    .expect("switch endpoints and transition slope are jointly representable");
}

#[test]
fn voltage_switch_rejects_invalid_or_nonrepresentable_conductances() {
    for (parameters, expected) in [
        ("RON=0", "RON must be finite and positive"),
        ("ROFF=-1", "ROFF must be finite and positive"),
        ("SMOOTH=0", "SMOOTH must be finite and positive"),
        ("RON=1e-320", "1/RON must be finite and positive"),
        (
            "RON=1e-300 ROFF=1e300 SMOOTH=1e-300",
            "maximum |dg/dVctrl| must be finite and representable",
        ),
        (
            "RON=1e-300 ROFF=1e300 SMOOTH=3.5e-9",
            "maximum |dg/dVctrl| must be finite and representable",
        ),
        (
            "RON=1 ROFF=1.0000000000000002 SMOOTH=1e308",
            "maximum |dg/dVctrl| must be finite and representable",
        ),
    ] {
        let deck = format!(
            "* invalid voltage switch\nS1 out 0 ctrl 0 SMOD\nVCTRL ctrl 0 DC 0\nR1 out 0 1k\n.model SMOD VSWITCH ({parameters})\n.end\n"
        );
        let error = match build(&deck) {
            Err(error) => error,
            Ok(()) => panic!("invalid switch parameters must fail at build: {parameters}"),
        };
        assert!(
            error.contains(expected),
            "{parameters} produced the wrong diagnostic: {error}"
        );
    }
}

#[test]
fn level1_mos_rejects_invalid_effective_geometry_and_overlap_capacitance() {
    let cases = [
        (
            "M1 out gate 0 0 MMOD W=1u L=1u\n.model MMOD NMOS LEVEL=1 KP=1m LD=.5u",
            "effective channel length L-2*LD",
        ),
        (
            "M1 out gate 0 0 MMOD W=0 L=1u\n.model MMOD NMOS LEVEL=1 KP=1m",
            "instance parameter W to be finite and positive",
        ),
        (
            "M1 out gate 0 0 MMOD W=1u L=1u\n.model MMOD NMOS LEVEL=1 KP=1m CGSO=-1p",
            "parameter CGSO to be finite and nonnegative",
        ),
    ];

    for (device_and_model, expected) in cases {
        let deck = format!(
            "* invalid Level-1 boundary\nV1 gate 0 DC 1\nR1 out 0 1k\n{device_and_model}\n.end\n"
        );
        let error = build(&deck).expect_err("invalid Level-1 model must fail at build");
        assert!(
            error.contains(expected),
            "wrong Level-1 boundary diagnostic: {error}"
        );
    }
}

#[test]
fn zero_kp_level1_mos_is_preserved_as_an_exact_open_channel() {
    build(
        "\
* zero-KP exact channel
V1 gate 0 DC 1
R1 out 0 1e15
M1 out gate 0 0 MMOD W=7u L=2u
.model MMOD NMOS LEVEL=1 VTO=.5 KP=0 LAMBDA=0 IS=0 JS=0
.end
",
    )
    .expect("KP=0 is a valid exact zero-channel model, not a request for a floor");
}

#[test]
fn unit_junction_grading_coefficients_are_valid_level1_parameters() {
    build(
        "\
* unit grading coefficients
V1 gate 0 DC 1
R1 out 0 1k
M1 out gate 0 0 MMOD W=1u L=1u AD=1p AS=1p
.model MMOD NMOS LEVEL=1 KP=1m CJ=1p MJ=1 MJSW=1
.end
",
    )
    .expect("MJ=1 and MJSW=1 use the representable logarithmic depletion-charge limit");
}
