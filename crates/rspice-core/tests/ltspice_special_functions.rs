//! LTspice special-function `A` devices against LTspice.
//!
//! Reference values are LTspice 26.1.1 batch runs of the same decks.

use rspice_core::Complex64;
use rspice_core::analysis::ac::AcResult;
use rspice_core::engine::Engine;
use rspice_core::netlist::{ElementKind, Netlist, SpecialFunction};

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}\n{deck}"))
}

fn ac_voltage(point: &AcResult, node: &str) -> Complex64 {
    let index = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("missing node {node:?} in {:?}", point.node_names));
    point.voltages[index]
}

fn assert_close(label: &str, actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{label}: {actual:.9e} vs LTspice {expected:.9e} (tolerance {tolerance:e})"
    );
}

fn build_error(deck: &str) -> String {
    match Engine::default().build_circuit(&parse(deck)) {
        Ok(_) => panic!("deck builds:\n{deck}"),
        Err(error) => error.to_string(),
    }
}

#[test]
fn an_eight_terminal_line_naming_a_special_function_is_the_native_device() {
    let netlist = parse("* routing\nA1 0 in 0 0 0 0 out 0 ota g=1m iout=1 linear\n.end\n");
    let element = &netlist.elements[0];
    let ElementKind::SpecialFunction {
        function,
        instance_params,
        ..
    } = &element.kind
    else {
        panic!("not a special function: {:?}", element.kind);
    };
    assert_eq!(*function, SpecialFunction::Ota);
    assert_eq!(element.nodes.len(), 8);
    assert!(instance_params.contains(&("LINEAR".to_string(), 1.0)));

    // The same shape naming an XSPICE `.model` card stays an XSPICE line.
    let xspice = parse("* xspice\nA1 1 2 3 4 5 6 7 8 buf\n.model buf d_buffer\n.end\n");
    assert!(
        matches!(xspice.elements[0].kind, ElementKind::Xspice { .. }),
        "{:?}",
        xspice.elements[0].kind
    );
}

#[test]
fn ota_operating_points_match_ltspice() {
    let deck = "* ota operating points
V1 in 0 10m
Vm m 0 2
A1 0 in 0 0 0 0 o1 0 OTA G=1m Iout=10u Vhigh=5 Vlow=-5
R1 o1 0 1k
A2 in 0 0 0 0 0 o2 0 OTA G=1m Iout=10u
R2 o2 0 1Meg
A3 0 in m 0 0 0 o3 0 OTA G=1m Iout=10u Vhigh=5 Vlow=-5
R3 o3 0 1k
A4 0 in 0 0 0 0 o4 0 OTA G=1m Iout=10u Isink=-2u Vhigh=5 Vlow=-5
R4 o4 0 1k
A5 0 in 0 0 0 0 o5 0 OTA G=1m Iout=10u Vhigh=2 Rclamp=1k epsilon=0.5
R5 o5 0 1Meg
A6 0 in 0 0 0 0 o6 0 OTA G=1m Iout=10u ref=5m linear Vhigh=5 Vlow=-5
R6 o6 0 1k
A7 0 in 0 0 0 0 o7 0 OTA G=1m Iout=10u Vhigh=5 Vlow=-5 Rout=100k
.op
.end
";
    let op = Engine::default().run_dc_op(&parse(deck)).expect("OP solves");
    let v = |node: &str| op.try_voltage_named(node).unwrap();
    // Iout·tanh(G·vd/Iout) into 1k.
    assert_close("tanh drive", v("o1"), 7.615_941_551_9e-3, 1e-9);
    // Negative drive held at Vlow = 0 by the 1-ohm clamp.
    assert_close("low clamp", v("o2"), -7.615_933_943_6e-6, 1e-10);
    // The multiplier scales the argument, not the limit.
    assert_close("multiplier", v("o3"), 9.640_275_791_1e-3, 1e-9);
    // Isink moves the centre of the tanh.
    assert_close("isink", v("o4"), 9.586_657_642_4e-3, 1e-9);
    // The epsilon-softened 1k clamp above Vhigh = 2.
    assert_close("soft clamp", v("o5"), 2.074_441_240_2, 1e-6);
    // Linear transfer with a 5 mV offset is unbounded by Iout.
    assert_close("linear offset", v("o6"), 0.015, 1e-9);
    // Rout alone loads the output.
    assert_close("rout", v("o7"), 0.761_594_155_96, 1e-7);
}

#[test]
fn ota_small_signal_gain_rolls_off_on_rout_and_cout() {
    let deck = "* ota ac
V1 in 0 0 AC 1
A1 0 in 0 0 0 0 out 0 OTA G=1m Iout=20u Vhigh=10 Vlow=-10 Rout=10k Cout=1n
.end
";
    let netlist = parse(deck);
    let engine = Engine::default();
    let frequencies = [1.0, 1e4, 1e6];
    let ac = engine.run_ac(&netlist, &frequencies).expect("AC solves");
    // LTspice 26.1.1 batch: 9.99999998, 8.46733016, 0.159134790.
    for (point, expected) in ac.iter().zip([9.999_999_98, 8.467_330_16, 0.159_134_790]) {
        assert_close("|H|", ac_voltage(point, "out").norm(), expected, 2e-6 * expected);
    }
    let phase = ac_voltage(&ac[1], "out").arg().to_degrees();
    let expected = -(2.0 * std::f64::consts::PI * 1e4 * 1e-5).atan().to_degrees();
    assert_close("phase at the pole", phase, expected, 1e-6);
}

#[test]
fn ota_refuses_what_ltspice_does_not_define() {
    let unknown = build_error("* bad\nA1 0 in 0 0 0 0 out 0 OTA gain=2\nR1 out 0 1k\n.end\n");
    assert!(unknown.contains("GAIN"), "{unknown}");
    let rail = build_error("* bad\nA1 0 in 0 0 vn 0 out 0 OTA\nR1 out 0 1k\n.end\n");
    assert!(rail.contains("terminal 5"), "{rail}");
    let window = build_error("* bad\nA1 0 in 0 0 0 0 out 0 OTA Vhigh=0 Vlow=1\n.end\n");
    assert!(window.contains("Vhigh above Vlow"), "{window}");
}
