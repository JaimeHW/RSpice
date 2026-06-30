//! Engine-level validation for native ngspice MOS9 (`MOS LEVEL=9`).
//!
//! ngspice uses `LEVEL=9` for MOS9, the modified Berkeley level-3 MOSFET.
//! Xyce uses the same numeric level for BSIM3, so this test pins the
//! ngspice-compatible four-terminal MOS9 deck to a native MOS9 path instead
//! of the Xyce BSIM3 front.

use rspice_core::Value;
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::Netlist;

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn engine_with_dialect(dialect: SpiceDialect) -> Engine {
    Engine::new(SimulationConfig::default().with_spice_dialect(dialect))
}

fn assert_rel(what: &str, got: Value, reference: Value, rel_tol: Value) {
    let denom = reference.abs().max(1.0e-30);
    let rel = (got - reference).abs() / denom;
    assert!(
        rel <= rel_tol,
        "{what}: rspice={got:.9e} ngspice46={reference:.9e} rel={rel:.3e}"
    );
}

fn assert_ac_node(
    what: &str,
    result: &rspice_core::analysis::AcResult,
    node_name: &str,
    re_ref: Value,
    im_ref: Value,
    abs_tol: Value,
) {
    let node = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node_name))
        .unwrap_or_else(|| panic!("missing node {node_name} in {:?}", result.node_names));
    let got = result.voltages[node];
    let re_delta = (got.re - re_ref).abs();
    let im_delta = (got.im - im_ref).abs();
    assert!(
        re_delta <= abs_tol && im_delta <= abs_tol,
        "{what}: rspice=({:.9e},{:.9e}) ngspice46=({re_ref:.9e},{im_ref:.9e}) delta=({re_delta:.3e},{im_delta:.3e})",
        got.re,
        got.im
    );
}

#[test]
fn ngspice_mos9_level9_dc_op_uses_mos9_not_bsim3() {
    let deck = "\
        * MOS9 NMOS DC operating point, from neospice mos9_nmos_dc_op.cir\n\
        Vdd vdd 0 5\n\
        Vgs gate 0 2.0\n\
        Rd vdd drain 1k\n\
        M1 drain gate 0 0 NMOD W=10u L=1u\n\
        .model NMOD NMOS LEVEL=9 VTO=0.7 KP=110U GAMMA=0.4 PHI=0.65\n\
        + ETA=0.1 THETA=0.05 KAPPA=0.5 DELTA=0.5 NFS=1e10\n\
        .op\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("MOS9 deck parses");
    let (op, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("ngspice MOS9 LEVEL=9 deck must run natively");

    let m1 = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 OP entry");
    assert_eq!(
        m1.device_kind, "MOS9",
        "ngspice LEVEL=9 must dispatch to MOS9, not BSIM3"
    );

    assert_rel(
        "V(drain)",
        op.try_voltage_named("drain").expect("drain voltage"),
        3.193006,
        2.0e-6,
    );
    assert_rel(
        "I(Vdd)",
        op.branch_current_named("vdd").expect("Vdd branch current"),
        -1.80699e-3,
        2.0e-5,
    );
}

#[test]
fn ngspice_mos9_level9_common_source_ac_matches_ngspice46() {
    let deck = "\
        * MOS9 NMOS AC response, from neospice mos9_nmos_ac.cir\n\
        Vdd vdd 0 5\n\
        Vin gate 0 DC 2.0 AC 1\n\
        Rd vdd drain 1k\n\
        M1 drain gate 0 0 NMOD W=10u L=1u\n\
        .model NMOD NMOS LEVEL=9 VTO=0.7 KP=110U GAMMA=0.4 PHI=0.65\n\
        + CGSO=0.6e-9 CGDO=0.6e-9 CGBO=1e-10 CBD=10f CBS=10f\n\
        + ETA=0.1 THETA=0.05 KAPPA=0.5 DELTA=0.5 NFS=1e10\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("MOS9 AC deck parses");
    let reference = [
        (1.0e2, -1.208_659_417_287_943, 8.380_756_336_771_718e-9),
        (1.0e4, -1.208_659_417_270_831, 8.380_756_336_586_169e-7),
        (1.0e6, -1.208_659_413_345_128, 8.380_756_318_222_35e-5),
        (1.0e7, -1.208_659_023_006_565, 8.380_754_481_833_678e-4),
    ];
    let freqs: Vec<Value> = reference.iter().map(|&(freq, _, _)| freq).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("MOS9 AC runs");

    for ((freq, re_ref, im_ref), result) in reference.iter().zip(&results) {
        assert_ac_node(
            &format!("MOS9 AC V(drain) at {freq:.3e} Hz"),
            result,
            "drain",
            *re_ref,
            *im_ref,
            2.0e-8,
        );
    }
}

#[test]
fn best_available_level9_vth0_alias_still_uses_mos9() {
    let deck = "\
        * LEVEL=9 with the threshold alias must not be mistaken for Xyce BSIM3\n\
        Vdd vdd 0 5\n\
        Vgs gate 0 2.0\n\
        Rd vdd drain 1k\n\
        M1 drain gate 0 0 NMOD W=10u L=1u\n\
        .model NMOD NMOS LEVEL=9 VTH0=0.7 KP=110U GAMMA=0.4 PHI=0.65\n\
        + ETA=0.1 THETA=0.05 KAPPA=0.5 DELTA=0.5 NFS=1e10\n\
        .op\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("MOS9 alias deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("LEVEL=9 VTH0 alias deck must run as MOS9");
    let m1 = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 OP entry");

    assert_eq!(
        m1.device_kind, "MOS9",
        "BestAvailable must not classify shared classic-MOS aliases as BSIM3"
    );
}

#[test]
fn best_available_level9_version_with_mos9_surface_uses_mos9() {
    let deck = "\
        * VERSION alone is not decisive when the rest of LEVEL=9 is classic MOS9\n\
        Vdd vdd 0 5\n\
        Vgs gate 0 2.0\n\
        Rd vdd drain 1k\n\
        M1 drain gate 0 0 NMOD W=10u L=1u\n\
        .model NMOD NMOS LEVEL=9 VERSION=3.2.2 VTH0=0.7 KP=110U GAMMA=0.4 PHI=0.65\n\
        + ETA=0.1 THETA=0.05 KAPPA=0.5 DELTA=0.5 NFS=1e10\n\
        .op\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("BestAvailable VERSION+MOS9 deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("BestAvailable VERSION+MOS9 deck must run as MOS9");
    let m1 = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 OP entry");

    assert_eq!(
        m1.device_kind, "MOS9",
        "BestAvailable must not classify VERSION alone as decisive BSIM3 evidence"
    );
}

#[test]
fn explicit_ngspice_dialect_routes_level9_version_card_to_mos9() {
    let deck = "\
        * Ngspice dialect owns MOS LEVEL=9 even when VERSION is present\n\
        Vdd vdd 0 5\n\
        Vgs gate 0 2.0\n\
        Rd vdd drain 1k\n\
        M1 drain gate 0 0 NMOD W=10u L=1u\n\
        .model NMOD NMOS LEVEL=9 VERSION=3.2.2 VTH0=0.7 KP=110U GAMMA=0.4 PHI=0.65\n\
        + ETA=0.1 THETA=0.05 KAPPA=0.5 DELTA=0.5 NFS=1e10\n\
        .op\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("ngspice dialect LEVEL=9 deck parses");
    let (_, report) = engine_with_dialect(SpiceDialect::Ngspice)
        .run_dc_op_with_report(&netlist)
        .expect("ngspice dialect LEVEL=9 deck must run as MOS9");
    let m1 = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 OP entry");

    assert_eq!(m1.device_kind, "MOS9");
}

#[test]
fn explicit_xyce_dialect_routes_level9_mos9_shaped_card_to_bsim3() {
    let deck = "\
        * Xyce dialect owns MOS LEVEL=9 as BSIM3\n\
        Vdd vdd 0 5\n\
        Vgs gate 0 2.0\n\
        Rd vdd drain 1k\n\
        M1 drain gate 0 0 NMOD W=10u L=1u\n\
        .model NMOD NMOS LEVEL=9 VTO=0.7 KP=110U GAMMA=0.4 PHI=0.65\n\
        + ETA=0.1 THETA=0.05 KAPPA=0.5 DELTA=0.5 NFS=1e10\n\
        .op\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("xyce dialect LEVEL=9 deck parses");
    let (_, report) = engine_with_dialect(SpiceDialect::Xyce)
        .run_dc_op_with_report(&netlist)
        .expect("xyce dialect LEVEL=9 deck must run as BSIM3");
    let m1 = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 OP entry");

    assert_eq!(m1.device_kind, "BSIM3");
}
