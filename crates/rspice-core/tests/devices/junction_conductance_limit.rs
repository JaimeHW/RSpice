//! LTspice runs a junction law until its conductance reaches a limit set by
//! GMIN, and on its tangent from there. The default dialect follows it; the
//! ngspice and Xyce dialects keep the exponential. Every reference value
//! below is LTspice 17.2.4 (`LTspice -b`) on the deck written here.

use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::Netlist;

/// The current a bare diode draws at `bias` volts, from an operating point.
fn diode_current(dialect: SpiceDialect, bias: f64, instance: &str, model: &str, gmin: f64) -> f64 {
    let deck = format!(
        "* junction limit\nv1 a 0 {bias}\nd1 a 0 dm {instance}\n.model dm d {model}\n\
         .options gmin={gmin:e}\n.op\n.end\n"
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
    let rel = (got - expected).abs() / expected.abs().max(1e-30);
    assert!(
        rel <= rel_tol,
        "{label}: got {got:.12e}, expected {expected:.12e} (rel {rel:.3e} > {rel_tol:.1e})"
    );
}

/// The whole diode law (the IS exponential with its IKF knee plus the ISR
/// recombination term) runs until its conductance reaches AREA*M/GMIN and
/// on its tangent from there: a slope of exactly 1e12 S at the default
/// GMIN, 1e9 S at GMIN=1e-9 and 1e13 S at AREA=10 or M=10. The tangent of
/// IS=1e-14 crosses zero current at 1.428064730 V (its knee is 1.4538 V).
/// The plain junctions agree to the 1.4e-6 the two tools' thermal voltages
/// differ by. With ISR or IKF, LTspice's tangent is 5e-7 to 2.4e-6 under
/// its limit (an iterated knee); the default dialect runs the limit itself.
#[test]
fn a_diode_stops_steepening_at_area_m_over_gmin_as_ltspice_does() {
    // (instance, model, GMIN, LTspice current at 6 V, its slope there)
    let cases = [
        ("", "is=1e-14", 1e-12, 4.571935270e12, 1e12, 1e-6),
        ("", "is=1e-14 n=2", 1e-12, 3.108014101e12, 1e12, 2e-6),
        ("", "is=1e-14", 1e-9, 4.750604035e9, 1e9, 1e-6),
        ("10", "is=1e-14", 1e-12, 4.571935270e13, 1e13, 1e-6),
        ("m=10", "is=1e-14", 1e-12, 4.571935270e13, 1e13, 1e-6),
        (
            "",
            "is=1e-14 isr=1e-10 nr=2",
            1e-12,
            4.571927681e12,
            9.999983e11,
            3e-6,
        ),
        (
            "",
            "is=1e-100 isr=1e-10 nr=2",
            1e-12,
            3.594221990e12,
            9.999976e11,
            4e-6,
        ),
        (
            "10",
            "is=1e-100 isr=1e-10 nr=2 m=0",
            1e-12,
            3.584462497e13,
            9.999995e12,
            3e-6,
        ),
        (
            "",
            "is=1e-14 ikf=1e-3",
            1e-12,
            3.763125442e12,
            9.999980e11,
            4e-6,
        ),
    ];
    for (instance, model, gmin, at_6, slope, tol) in cases {
        let label = format!("d1 {instance} ({model}) gmin={gmin:e}");
        let current = diode_current(SpiceDialect::BestAvailable, 6.0, instance, model, gmin);
        assert_rel_close(&format!("{label} at 6 V"), current, at_6, tol);
        let below = diode_current(SpiceDialect::BestAvailable, 5.995, instance, model, gmin);
        assert_rel_close(
            &format!("{label} slope"),
            (current - below) / 0.005,
            slope,
            3e-6,
        );
    }
    let at_6 = diode_current(SpiceDialect::BestAvailable, 6.0, "", "is=1e-14", 1e-12);
    let zero_crossing = 6.0 - at_6 / 1e12;
    assert!(
        (zero_crossing - 1.428064730).abs() < 1e-5,
        "the tangent crosses zero at {zero_crossing:.9} V, LTspice 1.428064730 V"
    );
    // ngspice and Xyce run the exponential on: 3.8e19 A at 2 V, where the
    // tangent gives 5.7e11 A.
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let current = diode_current(dialect, 2.0, "", "is=1e-14", 1e-12);
        assert!(current > 1e19, "{dialect:?}: {current:e}");
    }
}

/// Reverse breakdown runs on the same limit: BV=5, IBV=1e-10 has its knee
/// at -6.2157 V and a slope of exactly 1e12 S past it. Below the knee the
/// default dialect's breakdown sits 0.48 mV deeper than LTspice's (1.8 %
/// of the current at -5.165 V), which carries to the 8.2e-5 at -12 V.
#[test]
fn diode_breakdown_stops_steepening_at_area_m_over_gmin_as_ltspice_does() {
    let model = "is=1e-14 bv=5 ibv=1e-10";
    let current = diode_current(SpiceDialect::BestAvailable, -12.0, "", model, 1e-12);
    assert_rel_close("BV=5 at -12 V", current, -5.810160290e12, 1e-4);
    let above = diode_current(SpiceDialect::BestAvailable, -11.995, "", model, 1e-12);
    assert_rel_close("BV=5 slope", (above - current) / 0.005, 1e12, 1e-6);
}
