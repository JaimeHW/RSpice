//! The default dialect's diode follows LTspice's junction law; the ngspice
//! dialect keeps ngspice's. Every LTspice value below is LTspice 17.2.4
//! (`LTspice -b`, `.dc` sweep) and every ngspice value ngspice-46 (`.op`),
//! on a bare diode at the bias given. The two tools' thermal voltages differ
//! by 1.4e-6, which bounds the agreement.

use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::Netlist;

/// The current a bare diode draws at `bias` volts, from an operating point.
fn diode_current(dialect: SpiceDialect, bias: f64, instance: &str, model: &str) -> f64 {
    let deck = format!(
        "* diode law\nv1 a 0 {bias}\nd1 a 0 dm {instance}\n.model dm d {model}\n.op\n.end\n"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = Engine::new(SimulationConfig::default().with_spice_dialect(dialect))
        .run_dc_op(&netlist)
        .unwrap_or_else(|error| panic!("op failed: {error}\n{deck}"));
    -result
        .branch_current_named("v1")
        .unwrap_or_else(|| panic!("missing v1 branch in {:?}", result.branch_names))
}

fn assert_rel_close(label: &str, got: f64, expected: f64, rel_tol: f64) {
    let rel = (got - expected).abs() / expected.abs().max(1e-300);
    assert!(
        rel <= rel_tol,
        "{label}: got {got:.12e}, expected {expected:.12e} (rel {rel:.3e} > {rel_tol:.1e})"
    );
}

/// LTspice sums `IS*(exp(V/(N*Vt))-1)` and `-IBV*exp(-(V+BV)/(NBV*Vt))`
/// at every bias. It does not match BV to IBV, has no cubic reverse
/// continuation, defaults NBV to 1 and IBV to 1e-10, and scales IBV with
/// AREA and M. At zero bias the breakdown term alone flows,
/// -IBV*exp(-BV/Vt); that point carries 193 thermal voltages, so the
/// thermal-voltage difference is 2.7e-4 there.
#[test]
fn the_default_diode_sums_ltspice_breakdown_at_every_bias() {
    let rb = "is=1e-14 bv=5 ibv=1e-10";
    // (instance, model, bias, LTspice current, tolerance)
    let cases = [
        ("", rb, 0.0, -1.1111229988e-94, 5e-4),
        ("", rb, 0.6, 1.1871580608e-04, 1e-4),
        ("", rb, -4.8, -4.8538359563e-12, 1e-4),
        ("", rb, -5.165, -5.8955994866e-08, 1e-4),
        ("", rb, -5.5, -2.4855573397e-02, 1e-4),
        (
            "",
            "is=1e-14 bv=5 ibv=1e-10 nbv=2",
            -5.33,
            -5.8956159866e-08,
            1e-4,
        ),
        ("10", rb, -5.275, -4.1446049255e-05, 1e-4),
        ("m=10", rb, -4.765, -4.9782793688e-12, 1e-4),
        (
            "",
            "is=1e-14 bv=5 ibv=1e-3",
            -4.345,
            -4.3650460385e-12,
            1e-4,
        ),
        (
            "",
            "is=1e-14 n=2 bv=5 ibv=1e-10",
            -4.91,
            -8.0019367282e-12,
            1e-4,
        ),
        ("", "is=1e-14 bv=5", -5.795, -2.2321522845e+03, 1e-4),
        (
            "",
            "is=1e-9 bv=2 ibv=1e-6 nbv=3",
            -2.5,
            -6.2874607636e-04,
            1e-4,
        ),
        ("", "is=1e-14", -0.12, -1.2990337305e-13, 1e-4),
    ];
    for (instance, model, bias, expected, tolerance) in cases {
        let current = diode_current(SpiceDialect::BestAvailable, bias, instance, model);
        assert_rel_close(
            &format!("d1 {instance} ({model}) at {bias} V"),
            current,
            expected,
            tolerance,
        );
    }
}

/// ngspice matches BV to IBV, continues reverse bias with SPICE2's cubic
/// and defaults IBV to 1 mA (diosetup.c:56); its dialect keeps all three.
#[test]
fn the_ngspice_diode_keeps_matched_breakdown_and_its_cubic() {
    // (model, bias, ngspice-46 current)
    let cases = [
        ("is=1e-14 bv=5 ibv=1e-10", -5.165, -5.78770984515e-08),
        ("is=1e-14 bv=5 ibv=1e-10", -5.5, -2.44011187458e-02),
        ("is=1e-14 bv=5", -5.165, -5.89513418847e-01),
        ("is=1e-14 bv=5 ibv=1e-10", -0.12, -1.29865392317e-13),
    ];
    for (model, bias, expected) in cases {
        let current = diode_current(SpiceDialect::Ngspice, bias, "", model);
        assert_rel_close(
            &format!("d1 ({model}) at {bias} V"),
            current,
            expected,
            1e-9,
        );
    }
}
