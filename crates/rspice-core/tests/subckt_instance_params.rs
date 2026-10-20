//! Per-instance subcircuit parameter propagation.
//!
//! Element parameters written inside a subcircuit body (`w=wn`, `c={cv}`)
//! must be evaluated against each instance's parameter scope — instance
//! overrides over `PARAMS:` defaults over caller scope — not against the
//! definition-time defaults. Every PDK device wrapper depends on this; the
//! historical failure mode was silent (overrides parsed, then ignored).

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn op_voltage(deck: &str, node: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let op = engine.run_dc_op(&netlist).expect("operating point solves");
    let idx = op
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from OP result"));
    op.node_voltages[idx]
}

fn capacitance_of(deck: &str, name: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let circuit = Engine::default()
        .build_circuit(&netlist)
        .expect("circuit builds");
    let capacitors = circuit.capacitor_storage();
    let idx = capacitors
        .names
        .iter()
        .position(|n| n.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("capacitor {name} missing; have {:?}", capacitors.names));
    capacitors.capacitances[idx]
}

/// Saturated NMOS behind a 10k load: with LAMBDA=0 the drop across the load
/// is proportional to W, so V(d) measures the geometry the device received.
fn mos_cell_deck(instance_line: &str) -> String {
    format!(
        "* subckt instance param propagation\n\
         vdd vdd 0 dc 5\n\
         vg g 0 dc 2\n\
         rload vdd d 10k\n\
         {instance_line}\n\
         .subckt cell d g PARAMS: wn=1u\n\
         mn d g 0 0 nch w=wn l=1u\n\
         .ends\n\
         .model nch NMOS (LEVEL=1 VTO=1 KP=100u LAMBDA=0)\n\
         .op\n\
         .end\n"
    )
}

#[test]
fn mosfet_width_override_reaches_device_geometry() {
    // W=1u default: Id = 0.5*100u*1*(2-1)^2 = 50uA -> V(d) = 5 - 0.5 = 4.5 V
    // W=4u override: Id = 200uA -> V(d) = 5 - 2.0 = 3.0 V
    let vd_default = op_voltage(&mos_cell_deck("x1 d g cell"), "d");
    let vd_override = op_voltage(&mos_cell_deck("x1 d g cell wn=4u"), "d");
    assert!(
        (vd_default - 4.5).abs() < 1e-6,
        "default width: expected V(d)=4.5, got {vd_default}"
    );
    assert!(
        (vd_override - 3.0).abs() < 1e-6,
        "wn=4u must quadruple the drain current: expected V(d)=3.0, got {vd_override}"
    );
}

#[test]
fn mosfet_braced_expression_override() {
    // w={wn*2} with wn=3u -> W=6u -> Id = 300uA -> V(d) = 5 - 3.0 = 2.0 V.
    let deck = "* braced expression geometry\n\
         vdd vdd 0 dc 5\n\
         vg g 0 dc 2\n\
         rload vdd d 10k\n\
         x1 d g cell wn=3u\n\
         .subckt cell d g PARAMS: wn=1u\n\
         mn d g 0 0 nch w={wn*2} l=1u\n\
         .ends\n\
         .model nch NMOS (LEVEL=1 VTO=1 KP=100u LAMBDA=0)\n\
         .op\n\
         .end\n";
    let vd = op_voltage(deck, "d");
    assert!(
        (vd - 2.0).abs() < 1e-6,
        "w={{wn*2}} with wn=3u must give W=6u: expected V(d)=2.0, got {vd}"
    );
}

#[test]
fn capacitor_value_override_reaches_storage() {
    let deck = |inst: &str| {
        format!(
            "* cap value propagation\n\
             v1 a 0 dc 1\n\
             {inst}\n\
             .subckt blk a PARAMS: cv=1n\n\
             c1 a 0 cv\n\
             .ends\n\
             .op\n\
             .end\n"
        )
    };
    let c_default = capacitance_of(&deck("x1 a blk"), "x1.c1");
    let c_override = capacitance_of(&deck("x1 a blk cv=4n"), "x1.c1");
    assert!(
        (c_default - 1e-9).abs() < 1e-21,
        "default: expected 1n, got {c_default}"
    );
    assert!(
        (c_override - 4e-9).abs() < 1e-21,
        "cv=4n override must reach the capacitor: got {c_override}"
    );
}

#[test]
fn diode_area_override_scales_current() {
    // Forward-biased diode behind a load; larger AREA -> larger current ->
    // lower anode voltage.
    let deck = |inst: &str| {
        format!(
            "* diode area propagation\n\
             v1 in 0 dc 5\n\
             r1 in a 1k\n\
             {inst}\n\
             .subckt dcell a PARAMS: scale=1\n\
             d1 a 0 dmod area=scale\n\
             .ends\n\
             .model dmod D (IS=1e-14 N=1)\n\
             .op\n\
             .end\n"
        )
    };
    let va_default = op_voltage(&deck("x1 a dcell"), "a");
    let va_big = op_voltage(&deck("x1 a dcell scale=100"), "a");
    assert!(
        va_big < va_default - 0.05,
        "area=100 must raise diode current and lower V(a): default {va_default}, scaled {va_big}"
    );
}

#[test]
fn nested_hierarchy_propagates_overrides() {
    // Top overrides mid's parameter; mid forwards it to the leaf cell.
    let deck = "* two-level propagation\n\
         vdd vdd 0 dc 5\n\
         vg g 0 dc 2\n\
         rload vdd d 10k\n\
         x1 d g mid wo=4u\n\
         .subckt mid d g PARAMS: wo=1u\n\
         xc d g cell wn=wo\n\
         .ends\n\
         .subckt cell d g PARAMS: wn=1u\n\
         mn d g 0 0 nch w=wn l=1u\n\
         .ends\n\
         .model nch NMOS (LEVEL=1 VTO=1 KP=100u LAMBDA=0)\n\
         .op\n\
         .end\n";
    let vd = op_voltage(deck, "d");
    assert!(
        (vd - 3.0).abs() < 1e-6,
        "wo=4u must reach the leaf MOSFET through the mid level: expected V(d)=3.0, got {vd}"
    );
}

#[test]
fn definition_defaults_apply_without_override() {
    let vd = op_voltage(&mos_cell_deck("x1 d g cell"), "d");
    assert!(
        (vd - 4.5).abs() < 1e-6,
        "no override: the PARAMS: default must apply, got V(d)={vd}"
    );
}

#[test]
fn top_level_elements_unaffected() {
    // Outside subcircuit bodies parameters still resolve at parse time.
    let deck = "* top level geometry\n\
         .param wtop=4u\n\
         vdd vdd 0 dc 5\n\
         vg g 0 dc 2\n\
         rload vdd d 10k\n\
         mn d g 0 0 nch w=wtop l=1u\n\
         .model nch NMOS (LEVEL=1 VTO=1 KP=100u LAMBDA=0)\n\
         .op\n\
         .end\n";
    let vd = op_voltage(deck, "d");
    assert!(
        (vd - 3.0).abs() < 1e-6,
        "top-level w=wtop must resolve via .param: expected V(d)=3.0, got {vd}"
    );
}

#[test]
fn unresolvable_deferred_parameter_is_a_hard_error() {
    let deck = "* undefined parameter reference\n\
         v1 a 0 dc 1\n\
         x1 a blk\n\
         .subckt blk a\n\
         c1 a 0 q w=mystery\n\
         .ends\n\
         .end\n";
    let netlist = Netlist::parse(deck).expect("parse defers the unknown reference");
    let err = Engine::default()
        .build_circuit(&netlist)
        .expect_err("flattening must reject an unresolvable parameter");
    let message = err.to_string();
    assert!(
        message.to_lowercase().contains("mystery") || message.to_lowercase().contains("undefined"),
        "error should identify the unresolved reference: {message}"
    );
}

fn ac_magnitude(deck: &str, node: &str, frequency: f64) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let point = Engine::default()
        .run_ac(&netlist, &[frequency])
        .expect("AC solves")
        .remove(0);
    let idx = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from AC result"));
    point.voltages[idx].norm()
}

/// ngspice 46 on audit deck k3.cir gives |V(s)| = 0.5303726 at 1 kHz with
/// the instance's kc=0.5. The subcircuit default kc=0.9 gives 0.9487555,
/// which is what RSpice reported while K read only the default.
#[test]
fn coupling_coefficient_override_reaches_the_mutual_inductance() {
    let deck = "mutual inductor in subckt, param kc\n\
         .subckt xfmr p s params: kc=0.9 lv=1m\n\
         L1 p 0 {lv}\n\
         L2 s 0 4m\n\
         K1 L1 L2 {kc}\n\
         .ends\n\
         V1 in 0 AC 1\n\
         R1 in p 10\n\
         X1 p s xfmr kc=0.5\n\
         R2 s 0 1k\n\
         .ac lin 3 1k 100k\n\
         .end\n";
    let magnitude = ac_magnitude(deck, "s", 1.0e3);
    assert!(
        (magnitude - 0.5303726).abs() < 1.0e-6,
        "|V(s)| = {magnitude}, ngspice 0.5303726"
    );
}

/// A T line's Z0 and TD are evaluated in the instance's scope. ngspice 46
/// with the override z=100 d=2n: |V(b)| = 0.9524316, 1.090099 and 1.523388
/// at 1, 50.5 and 100 MHz. With the defaults (a matched 50 ohm line, which
/// is what RSpice used) the magnitude is a flat 0.952381.
#[test]
fn transmission_line_override_reaches_the_line() {
    let deck = "tline params in subckt\n\
         .subckt line a b params: z=50 d=1n\n\
         T1 a 0 b 0 Z0={z} TD={d}\n\
         .ends\n\
         V1 in 0 AC 1\n\
         R1 in a 50\n\
         X1 a b line z=100 d=2n\n\
         R2 b 0 1k\n\
         .ac lin 3 1meg 100meg\n\
         .end\n";
    for (frequency, expected) in [(1.0e6, 0.9524316), (50.5e6, 1.090099), (100.0e6, 1.523388)] {
        let magnitude = ac_magnitude(deck, "b", frequency);
        // ngspice prints seven significant digits.
        assert!(
            (magnitude - expected).abs() < 2.0e-6,
            "|V(b)| at {frequency:e} Hz = {magnitude}, ngspice {expected}"
        );
    }
}

/// POLY coefficients and TABLE entries of E/G/F/H sources are evaluated in
/// the instance's scope. ngspice 46 on this deck: every output is 2 V except
/// the TABLE's 1.5 V. With the subcircuit defaults (g=1, vmax=1), which is
/// what RSpice used, every output was 0.5 V.
#[test]
fn poly_and_table_terms_resolve_in_each_instance_scope() {
    let deck = "poly and table in subckt\n\
         .subckt amp in o1 o2 o3 o4 o5 params: g=1 vmax=1\n\
         E1 o1 0 POLY(1) in 0 0 {g}\n\
         G2 0 o2 POLY(1) in 0 0 {g*1m}\n\
         R2 o2 0 1k\n\
         E3 o3 0 TABLE {v(in)} = (0, 0) (1, {vmax}) (10, {10*vmax})\n\
         Vs in s 0\n\
         Rs s 0 1k\n\
         F4 0 o4 POLY(1) Vs 0 {g}\n\
         R4 o4 0 1k\n\
         H5 o5 0 POLY(1) Vs 0 {g*1k}\n\
         .ends\n\
         V1 in 0 0.5\n\
         X1 in a1 a2 a3 a4 a5 amp g=4 vmax=3\n\
         R1 a1 0 1k\n\
         R3 a3 0 1k\n\
         R5 a5 0 1k\n\
         .op\n\
         .end\n";
    for (node, expected) in [
        ("a1", 2.0),
        ("a2", 2.0),
        ("a3", 1.5),
        ("a4", 2.0),
        ("a5", 2.0),
    ] {
        let v = op_voltage(deck, node);
        assert!(
            (v - expected).abs() < 1e-9,
            "V({node}) = {v}, ngspice {expected}"
        );
    }
}

/// A root POLY coefficient may cite a `.param` declared later in the file;
/// ngspice 46 gives V(p) = 3 V here. It used to fail to parse.
#[test]
fn root_poly_coefficients_read_later_parameters() {
    let deck = "forward poly\n\
         V1 in 0 1\n\
         R1 in 0 1k\n\
         E2 p 0 POLY(1) in 0 0 {g}\n\
         R5 p 0 1k\n\
         .param g=3\n\
         .op\n\
         .end\n";
    let v = op_voltage(deck, "p");
    assert!((v - 3.0).abs() < 1e-9, "V(p) = {v}, ngspice 3");
}
