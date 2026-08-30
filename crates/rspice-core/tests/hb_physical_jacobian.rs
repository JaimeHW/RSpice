//! Regression oracles ensuring HB/PAC linearizes the authored device law,
//! without inserting solver-conditioning conductance into the physical model.

use num_complex::Complex64;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

const F0: f64 = 1.0e6;
const OFFSET: f64 = 1.0e4;

fn pac_transfer(deck: &str, input: &str, output: &str) -> Complex64 {
    let netlist = Netlist::parse(deck).expect("physical-Jacobian deck parses");
    let config = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(OFFSET, OFFSET, 1)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(0, 0)
        .with_tolerances(1.0e-10, 1.0e-15)
        .with_input_source(input)
        .with_output_node(output);
    Engine::new(SimulationConfig::default())
        .run_pac(&netlist, config)
        .expect("static PAC derivative solve completes")
        .result
        .conversion_matrix
        .get(0, 0, 0)
        .expect("one-point PAC conversion transfer is retained")
}

fn ac_voltage(deck: &str, output: &str) -> Complex64 {
    let netlist = Netlist::parse(deck).expect("static AC oracle deck parses");
    let point = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[OFFSET])
        .expect("static AC derivative oracle completes")
        .pop()
        .expect("one AC point is retained");
    let node = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(output))
        .expect("AC output node is retained");
    point.voltages[node]
}

#[test]
fn cutoff_devices_do_not_shunt_a_very_high_impedance_pac_load() {
    const LOAD: f64 = 1.0e15;
    let cases = [
        (
            "diode",
            "\
* reverse-biased zero-IS diode on a high-impedance node
IIN 0 out DC 0
VBIAS bias 0 DC 1
D1 out bias DMOD
RLOAD out 0 1e15
.model DMOD D IS=0 N=1
.end
",
        ),
        (
            "cutoff MOS",
            "\
* cutoff zero-leakage MOS on a high-impedance node
IIN 0 out DC 0
VG gate 0 DC -1
M1 out gate 0 0 NMOD W=1u L=1u
RLOAD out 0 1e15
.model NMOD NMOS LEVEL=1 VTO=0.7 KP=1m LAMBDA=0 IS=0 JS=0
.end
",
        ),
        (
            "cutoff JFET",
            "\
* cutoff zero-leakage JFET on a high-impedance node
IIN 0 out DC 0
VG gate 0 DC -3
J1 out gate 0 JMOD
RLOAD out 0 1e15
.model JMOD NJF VTO=-2 BETA=1m LAMBDA=0 IS=0
.end
",
        ),
        (
            "zero-KP active MOS",
            "\
* zero-KP active MOS on a high-impedance node
IIN 0 out DC 0
VG gate 0 DC 1.5
M1 out gate 0 0 NMOD W=7u L=2u
RLOAD out 0 1e15
.model NMOD NMOS LEVEL=1 VTO=0.7 KP=0 LAMBDA=0 IS=0 JS=0
.end
",
        ),
        (
            "zero-BETA active JFET",
            "\
* zero-BETA active JFET on a high-impedance node
IIN 0 out DC 0
VG gate 0 DC 0
J1 out gate 0 JMOD
RLOAD out 0 1e15
.model JMOD NJF VTO=-2 BETA=0 LAMBDA=0 IS=0
.end
",
        ),
    ];

    for (kind, deck) in cases {
        let transfer = pac_transfer(deck, "IIN", "out");
        let expected = Complex64::new(LOAD, 0.0);
        assert!(
            (transfer - expected).norm() <= 1.0e-9 * LOAD,
            "{kind} introduced a nonphysical cutoff shunt: got {transfer}, want {expected}"
        );
    }
}

#[test]
fn level1_mos_nonunity_geometry_matches_the_ordinary_ac_derivative() {
    let deck = "\
* non-unity Level-1 geometry
VDD vdd 0 DC 5
VIN gate 0 DC 1.5
RD vdd out 200
M1 out gate 0 0 NMOD W=6u L=3u
.model NMOD NMOS LEVEL=1 VTO=0.7 KP=1m LD=.5u LAMBDA=0 IS=0 JS=0
.end
";
    let pac = pac_transfer(deck, "VIN", "out");
    let ac_deck = deck.replace("VIN gate 0 DC 1.5", "VIN gate 0 DC 1.5 AC 1");
    let ordinary = ac_voltage(&ac_deck, "out");
    assert!(
        (pac - ordinary).norm() <= 2.0e-8 * ordinary.norm(),
        "non-unity W/Leff PAC derivative was {pac}, ordinary AC produced {ordinary}"
    );

    // beta=KP*W/(L-2*LD)=3mA/V^2 and gm=beta*(VGS-VTO)=2.4mS.
    let expected = Complex64::new(-0.48, 0.0);
    assert!(
        (pac - expected).norm() <= 2.0e-6,
        "non-unity W/Leff PAC derivative was {pac}, expected {expected}"
    );
}

#[test]
fn level1_subpicometer_effective_length_and_small_phi_match_ordinary_ac() {
    let deck = "\
* exact sub-picometer Leff and small PHI
VDD vdd 0 DC 5
VIN gate 0 DC 1.5
VB bulk 0 DC -3e-15
RD vdd out 200
M1 out gate 0 bulk NMOD W=1p L=1.5p
.model NMOD NMOS LEVEL=1 VTO=0.7 KP=1m LD=.5p GAMMA=1e6 PHI=1e-15 LAMBDA=0 IS=0 JS=0
.end
";
    let pac = pac_transfer(deck, "VIN", "out");
    let ac_deck = deck.replace("VIN gate 0 DC 1.5", "VIN gate 0 DC 1.5 AC 1");
    let ordinary = ac_voltage(&ac_deck, "out");
    assert!(
        (pac - ordinary).norm() <= 2.0e-8 * ordinary.norm(),
        "sub-picometer Leff/small-PHI PAC derivative was {pac}, ordinary AC produced {ordinary}"
    );
    assert!(
        pac.re < -0.25 && pac.re > -0.35,
        "fixture must exercise the intended finite strong-inversion gain, got {pac}"
    );
}

#[test]
fn exact_hb_switch_preserves_small_ron_large_roff_and_small_smooth() {
    let off = pac_transfer(
        "\
* exact off-state voltage switch
IIN 0 out DC 0
VCTRL ctrl 0 DC -1
S1 out 0 ctrl 0 SMOD
RLOAD out 0 1e15
.model SMOD VSWITCH (VT=0 VH=0 RON=1e-12 ROFF=1e15 SMOOTH=1e-12)
.end
",
        "IIN",
        "out",
    );
    let expected_off = Complex64::new(5.0e14, 0.0);
    assert!(
        (off - expected_off).norm() <= 1.0e-9 * expected_off.norm(),
        "exact-HB off-state switch transfer was {off}, expected {expected_off}"
    );

    let on = pac_transfer(
        "\
* exact on-state voltage switch
IIN 0 out DC 0
VCTRL ctrl 0 DC 1
S1 out 0 ctrl 0 SMOD
RLOAD out 0 1e15
.model SMOD VSWITCH (VT=0 VH=0 RON=1e-12 ROFF=1e15 SMOOTH=1e-12)
.end
",
        "IIN",
        "out",
    );
    let expected_on = Complex64::new(1.0e-12, 0.0);
    assert!(
        (on - expected_on).norm() <= 1.0e-9 * expected_on.norm(),
        "exact-HB on-state switch transfer was {on}, expected {expected_on}"
    );
}

#[test]
fn exact_hb_rejects_stateful_and_xyce_curve_voltage_switches() {
    for (parameters, expected) in [
        ("VT=0 VH=.1 RON=1 ROFF=1e6 SMOOTH=.1", "hysteresis"),
        ("ON=1 OFF=0 RON=1 ROFF=1e6", "Xyce ON/OFF curve"),
    ] {
        let deck = format!(
            "* unsupported exact-HB switch semantics\nVCTRL ctrl 0 DC 0\nS1 out 0 ctrl 0 SMOD\nR1 out 0 1k\n.model SMOD VSWITCH ({parameters})\n.end\n"
        );
        let netlist = Netlist::parse(&deck).expect("unsupported switch fixture parses");
        let error = Engine::new(SimulationConfig::default())
            .run_pac(
                &netlist,
                PacConfig::new()
                    .with_fundamental(F0)
                    .with_sweep(OFFSET, OFFSET, 1)
                    .with_input_source("VCTRL")
                    .with_output_node("out"),
            )
            .expect_err("unsupported switch semantics must fail before PAC solving");
        assert!(
            error.to_string().contains(expected),
            "wrong exact-HB switch rejection for {parameters}: {error}"
        );
    }
}

#[test]
fn zero_pump_pac_uses_exact_static_diode_and_zero_lambda_mos_derivatives() {
    let diode_deck = "\
* diode small-signal resistance at an exact DC current
IBIAS 0 out DC 1m
IIN 0 out DC 0
D1 out 0 DMOD
.model DMOD D IS=1p N=1
.end
";
    let diode = pac_transfer(diode_deck, "IIN", "out");
    let diode_ac_deck = diode_deck.replace("IIN 0 out DC 0", "IIN 0 out DC 0 AC 1");
    let expected_diode = ac_voltage(&diode_ac_deck, "out");
    assert!(
        (diode - expected_diode).norm() <= 2.0e-8 * expected_diode.norm(),
        "PAC diode derivative was {diode}, ordinary AC produced {expected_diode}"
    );

    let mos_deck = "\
* common-source Level-1 MOS with exactly zero channel-length modulation
VDD vdd 0 DC 5
VIN gate 0 DC 1.5
RD vdd out 1k
M1 out gate 0 0 NMOD W=1u L=1u
.model NMOD NMOS LEVEL=1 VTO=0.7 KP=1m LAMBDA=0 IS=0 JS=0
.end
";
    let mos = pac_transfer(mos_deck, "VIN", "out");
    let mos_ac_deck = mos_deck.replace("VIN gate 0 DC 1.5", "VIN gate 0 DC 1.5 AC 1");
    let ordinary_mos = ac_voltage(&mos_ac_deck, "out");
    assert!(
        (mos - ordinary_mos).norm() <= 2.0e-8 * ordinary_mos.norm(),
        "PAC zero-pump MOS derivative was {mos}, ordinary AC produced {ordinary_mos}"
    );
    // HB's Level-1 law is Id=0.5*KP*(VGS-VTO)^2 in saturation, so
    // gm=KP*(VGS-VTO)=0.8 mS. With lambda=0 the authored gds is exactly
    // zero and the 1 kohm drain resistor gives Av=-gm*R=-0.8.
    let expected_mos = Complex64::new(-0.8, 0.0);
    assert!(
        (mos - expected_mos).norm() <= 2.0e-6,
        "PAC zero-LAMBDA MOS derivative was {mos}, expected {expected_mos}"
    );
}
