//! Extended controlled-source forms: POLY(n), VALUE={...}, TABLE {...}.
//!
//! These forms lower onto the behavioral-source engine, so the defining
//! contract is exact equivalence with the hand-written B-source carrying the
//! same mathematics, plus closed-form operating-point checks for the
//! voltage-output forms where signs are unambiguous.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{ElementKind, Netlist};

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

#[test]
fn poly1_polynomial_matches_closed_form() {
    // V(out) = 1 + 2*V(in) + 3*V(in)^2 with V(in) = 2 -> 17
    let deck = "\
* poly1
vin in 0 dc 2
e1 out 0 poly(1) in 0 1 2 3
rl out 0 1k
.op
.end
";
    let v = op_voltage(deck, "out");
    assert!((v - 17.0).abs() < 1e-9, "POLY(1) quadratic: got {v}");
}

#[test]
fn poly1_single_coefficient_is_linear_gain() {
    // SPICE2 rule: one coefficient is p1, not a constant.
    let deck = "\
* poly1 single coeff
vin in 0 dc 2
e1 out 0 poly(1) in 0 5
rl out 0 1k
.op
.end
";
    let v = op_voltage(deck, "out");
    assert!((v - 10.0).abs() < 1e-9, "single-coefficient gain: got {v}");
}

#[test]
fn poly2_cross_term_is_a_multiplier() {
    // Coefficient layout for POLY(2): p0 p1*v1 p2*v2 p3*v1^2 p4*v1*v2 ...
    // p4 = 1 alone makes the source an analog multiplier.
    let deck = "\
* poly2 multiplier
va a 0 dc 3
vb b 0 dc 4
e1 out 0 poly(2) a 0 b 0 0 0 0 0 1
rl out 0 1k
.op
.end
";
    let v = op_voltage(deck, "out");
    assert!((v - 12.0).abs() < 1e-9, "POLY(2) cross term: got {v}");
}

#[test]
fn poly3_weighted_summer() {
    // Linear-only POLY(3): 0.5 + 1*v1 + 2*v2 + 3*v3, with 1V, 2V, 3V inputs.
    let deck = "\
* poly3 summer
v1 a 0 dc 1
v2 b 0 dc 2
v3 c 0 dc 3
e1 out 0 poly(3) a 0 b 0 c 0 0.5 1 2 3
rl out 0 1k
.op
.end
";
    let v = op_voltage(deck, "out");
    assert!((v - 14.5).abs() < 1e-9, "POLY(3) summer: got {v}");
}

#[test]
fn value_form_matches_expression() {
    let deck = "\
* value form
vin in 0 dc 2
e1 out 0 value={3*v(in)+1}
rl out 0 1k
.op
.end
";
    let v = op_voltage(deck, "out");
    assert!((v - 7.0).abs() < 1e-9, "VALUE expression: got {v}");
}

#[test]
fn table_form_interpolates_and_clamps() {
    // Inside the table: linear interpolation between (-1,-5) and (1,5).
    let inside = "\
* table inside
vin in 0 dc 0.5
e1 out 0 table {v(in)} = (-1,-5) (1,5)
rl out 0 1k
.op
.end
";
    let v = op_voltage(inside, "out");
    assert!((v - 2.5).abs() < 1e-9, "TABLE interpolation: got {v}");

    // Beyond the last breakpoint: PSpice semantics clamp to the endpoint.
    let clamped = "\
* table clamped
vin in 0 dc 3
e1 out 0 table {v(in)} = (-1,-5) (1,5)
rl out 0 1k
.op
.end
";
    let v = op_voltage(clamped, "out");
    assert!((v - 5.0).abs() < 1e-9, "TABLE endpoint clamp: got {v}");
}

#[test]
fn cccs_poly_matches_equivalent_b_source() {
    // F POLY(1) squared term against the hand-written behavioral source with
    // identical mathematics: results must agree exactly (same engine path).
    let poly = "\
* f poly
v1 p 0 dc 3
vsense p q dc 0
r1 q 0 3k
f1 0 out poly(1) vsense 0 0 1e3
rl out 0 1k
.op
.end
";
    let behavioral = "\
* b equivalent
v1 p 0 dc 3
vsense p q dc 0
r1 q 0 3k
b1 0 out i=(1e3)*(i(vsense))*(i(vsense))
rl out 0 1k
.op
.end
";
    let vp = op_voltage(poly, "out");
    let vb = op_voltage(behavioral, "out");
    assert!(
        (vp - vb).abs() < 1e-12,
        "F POLY must equal equivalent B source: {vp} vs {vb}"
    );
    // And the magnitude is the closed form (1mA)^2 * 1e3 * 1k = 1V.
    assert!((vp.abs() - 1.0).abs() < 1e-9, "F POLY magnitude: got {vp}");
}

#[test]
fn ccvs_poly_matches_closed_form() {
    // H POLY(1): V(out) = 2000 * I(vsense)^2 = 2000 * (1mA)^2 ... use linear
    // term instead for an unambiguous closed form: rm = 2k -> 2V at 1mA.
    let deck = "\
* h poly linear
v1 p 0 dc 3
vsense p q dc 0
r1 q 0 3k
h1 out 0 poly(1) vsense 0 2k
rl out 0 1k
.op
.end
";
    let v = op_voltage(deck, "out");
    assert!((v - 2.0).abs() < 1e-9, "H POLY linear: got {v}");
}

#[test]
fn linear_forms_keep_dedicated_fast_paths() {
    let deck = "\
* linear controlled sources stay linear
vin in 0 dc 1
vs a b dc 0
e1 e_out 0 in 0 2
g1 g_out 0 in 0 1m
f1 f_out 0 vs 2
h1 h_out 0 vs 1k
r1 a 0 1k
r2 b 0 1k
re e_out 0 1k
rg g_out 0 1k
rf f_out 0 1k
rh h_out 0 1k
.op
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let mut counts = (0, 0, 0, 0);
    for element in &netlist.elements {
        match element.kind {
            ElementKind::Vcvs { .. } => counts.0 += 1,
            ElementKind::Vccs { .. } => counts.1 += 1,
            ElementKind::Cccs { .. } => counts.2 += 1,
            ElementKind::Ccvs { .. } => counts.3 += 1,
            _ => {}
        }
    }
    assert_eq!(
        counts,
        (1, 1, 1, 1),
        "plain linear E/G/F/H must keep their dedicated element kinds"
    );
}

#[test]
fn value_form_matches_linear_gain_in_ac() {
    // Behavioral lowering must be exact in AC as well: E VALUE={2*v(in)}
    // and the dedicated linear VCVS with gain 2 see identical small-signal
    // magnitudes across the band.
    let value_form = "\
* e value ac
vin in 0 dc 1 ac 1
e1 out 0 value={2*v(in)}
rl out 0 1k
cl out 0 1n
.end
";
    let linear_form = "\
* e linear ac
vin in 0 dc 1 ac 1
e1 out 0 in 0 2
rl out 0 1k
cl out 0 1n
.end
";
    let engine = Engine::new(SimulationConfig::default());
    let freqs = [1e3, 1e5, 1e7];
    let value_results = engine
        .run_ac(&Netlist::parse(value_form).expect("parse"), &freqs)
        .expect("value-form AC runs");
    let linear_results = engine
        .run_ac(&Netlist::parse(linear_form).expect("parse"), &freqs)
        .expect("linear-form AC runs");

    for (value_result, linear_result) in value_results.iter().zip(&linear_results) {
        let out_v = value_result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .expect("out node (value form)");
        let out_l = linear_result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .expect("out node (linear form)");
        let a = value_result.voltages[out_v];
        let b = linear_result.voltages[out_l];
        assert!(
            (a - b).norm() < 1e-9,
            "VALUE vs linear VCVS in AC at {} Hz: {a} vs {b}",
            value_result.frequency
        );
        assert!(
            a.norm() > 0.1,
            "behavioral source must actually drive the AC solution"
        );
    }
}

#[test]
fn vccs_value_form_matches_linear_equivalent() {
    // G VALUE={gm*V(in)} must match the dedicated linear VCCS at the OP.
    let value_form = "\
* g value
vin in 0 dc 2
g1 0 out value={2m*v(in)}
rl out 0 1k
.op
.end
";
    let linear_form = "\
* g linear
vin in 0 dc 2
g1 0 out in 0 2m
rl out 0 1k
.op
.end
";
    let vv = op_voltage(value_form, "out");
    let vl = op_voltage(linear_form, "out");
    assert!(
        (vv - vl).abs() < 1e-9,
        "G VALUE must match linear VCCS: {vv} vs {vl}"
    );
}
