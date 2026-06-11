//! Per-device operating-point report: device bias and small-signal values
//! surfaced after a DC solve.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

#[test]
fn mosfet_report_carries_bias_and_small_signal_values() {
    // NMOS in saturation: VGS = 2V (VTO = 1), VDS = 5V > VGS - VTO.
    let deck = "\
* mosfet op report
vdd d 0 dc 5
vg g 0 dc 2
m1 d g 0 0 nmod w=10u l=1u
.model nmod NMOS (LEVEL=1 VTO=1 KP=100u LAMBDA=0)
.op
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let (_, report) = engine
        .run_dc_op_with_report(&netlist)
        .expect("operating point solves");

    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 present in OP report");
    assert_eq!(entry.device_kind, "MOSFET");
    assert_eq!(entry.region, Some("saturation"));

    let get = |key: &str| {
        entry
            .params
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| *value)
            .unwrap_or_else(|| panic!("{key} missing from MOSFET OP entry"))
    };

    // Level-1 closed form: Id = KP/2 * W/L * (Vgs - Vth)^2 = 50u*10*(1)^2.
    let id = get("id");
    assert!(
        (id - 500e-6).abs() / 500e-6 < 1e-3,
        "saturation current: got {id}"
    );
    let gm = get("gm");
    // gm = KP * W/L * (Vgs - Vth) = 1m
    assert!((gm - 1e-3).abs() / 1e-3 < 1e-3, "transconductance: got {gm}");
    assert!((get("vgs") - 2.0).abs() < 1e-9);
    assert!((get("vds") - 5.0).abs() < 1e-9);
    assert!((get("vth") - 1.0).abs() < 1e-9);
}

#[test]
fn diode_report_is_consistent_with_shockley_equation() {
    let deck = "\
* diode op report
v1 in 0 dc 5
r1 in a 1k
d1 a 0 dmod
.model dmod D IS=1e-14 N=1.5
.op
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let (result, report) = engine
        .run_dc_op_with_report(&netlist)
        .expect("operating point solves");

    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("d1"))
        .expect("d1 present in OP report");
    assert_eq!(entry.device_kind, "DIODE");

    let get = |key: &str| {
        entry
            .params
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| *value)
            .unwrap()
    };

    // The reported junction voltage must equal the solved anode voltage and
    // the reported current must satisfy KCL with the series resistor.
    let anode_idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("a"))
        .expect("anode node");
    let v_a = result.node_voltages[anode_idx];
    assert!((get("vd") - v_a).abs() < 1e-9, "vd vs solved anode voltage");

    let i_r = (5.0 - v_a) / 1000.0;
    assert!(
        (get("id") - i_r).abs() / i_r.abs() < 1e-6,
        "diode current consistent with resistor current: {} vs {}",
        get("id"),
        i_r
    );
    assert!(get("gd") > 0.0, "conductance positive in forward bias");
}

#[test]
fn bjt_report_carries_currents_and_beta() {
    let deck = "\
* bjt op report
vcc c 0 dc 10
vbb bdrv 0 dc 2
rb bdrv b 100k
rc c col 1k
q1 col b 0 qmod
.model qmod NPN (IS=1e-15 BF=100)
.op
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let (_, report) = engine
        .run_dc_op_with_report(&netlist)
        .expect("operating point solves");

    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("q1"))
        .expect("q1 present in OP report");
    assert_eq!(entry.device_kind, "BJT");

    let get = |key: &str| {
        entry
            .params
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| *value)
            .unwrap()
    };

    // Forward-active: vbe near 0.7V, ic = beta*ib with beta near BF.
    let vbe = get("vbe");
    assert!(
        (0.55..0.85).contains(&vbe),
        "forward-active vbe: got {vbe}"
    );
    let beta = get("beta");
    assert!(
        (50.0..150.0).contains(&beta),
        "beta near BF=100: got {beta}"
    );
    assert!(get("ic") > 0.0 && get("ib") > 0.0);
}
