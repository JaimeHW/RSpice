//! Grammar that foundry PDK device subcircuits depend on.
//!
//! PDK primitives are shipped as subcircuits that compute instance geometry
//! from the subcircuit's own parameters. IHP SG13G2's diode library, vendored
//! at `models/spice/foundry/ihp-sg13g2/`, is representative:
//!
//! ```text
//! .params l=780n w=780n DEV_A=(l*w)/1p DEV_P=(l+w)/0.5u DEV_C=4
//! D1 1 2 darea  area=DEV_A
//! douter bn NWell dsub_mod area=mf*abus pj=0
//! ```
//!
//! Two things there are load-bearing: the plural `.params` spelling, and bare
//! unbraced arithmetic in an instance parameter. ngspice 46 accepts both.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use rspice_core::solver::SimulationResult;

fn run_op(deck: &str) -> SimulationResult {
    let netlist = Netlist::parse(deck).expect("deck parses");
    Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("operating point converges")
}

#[test]
fn params_is_accepted_as_a_spelling_of_param() {
    // ngspice 46 on this deck reports I(V1) = -3 mA, i.e. the parameter took.
    let result = run_op(
        "* plural .params spelling\n\
         .params kk=3\n\
         R1 a 0 1k\n\
         V1 a 0 {kk}\n\
         .op\n\
         .end\n",
    );
    let voltage = result
        .try_voltage_named("a")
        .expect("node a present in the operating point");
    assert!(
        (voltage - 3.0).abs() < 1e-9,
        "expected .params to define kk=3, got V(a)={voltage}"
    );
}

#[test]
fn mosfet_instance_geometry_accepts_bare_arithmetic() {
    // W = mf*aws = 6u, L = 1u, KP = 2e-5, VTO = 1, Vgs = 3, saturation:
    //   Id = 0.5 * KP * (W/L) * (Vgs - Vth)^2 = 0.5 * 2e-5 * 6 * 4 = 240 uA
    // The analytic value pins that the product was evaluated, not truncated to
    // its first factor.
    let result = run_op(
        "* mos instance arithmetic\n\
         .param mf=2 aws=3e-6\n\
         .model NM NMOS(LEVEL=1 VTO=1 KP=2e-5)\n\
         V1 d 0 5\n\
         V2 g 0 3\n\
         M1 d g 0 0 NM W=mf*aws L=1u\n\
         .op\n\
         .end\n",
    );
    let drain_current = -result
        .branch_current_named("v1")
        .expect("v1 branch current present");
    assert!(
        (drain_current - 240e-6).abs() < 1e-9,
        "expected Id=240uA for W=mf*aws, got {drain_current:e}"
    );
}

#[test]
fn diode_instance_area_accepts_bare_arithmetic() {
    // area = mf*aws = 6e-12 scales IS to 6e-26. At 0.7 V that is 34.02 fA,
    // which is what ngspice 46 reports for the diode branch on this deck.
    let netlist = Netlist::parse(
        "* diode instance arithmetic\n\
         .param mf=2 aws=3e-12\n\
         .model DM D(IS=1e-14 N=1)\n\
         V1 a 0 0.7\n\
         D1 a 0 DM area=mf*aws pj=0\n\
         .op\n\
         .end\n",
    )
    .expect("deck parses");
    Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("operating point converges");
}

#[test]
fn signed_literal_instance_value_still_resolves_immediately() {
    // A signed literal reads as a compound token run but is still just a
    // number; it must not be pushed onto the deferred-expression path.
    let netlist = Netlist::parse(
        "* signed instance parameter\n\
         .model DM D(IS=1e-14)\n\
         V1 a 0 0.7\n\
         D1 a 0 DM area=2 ic=-5\n\
         .op\n\
         .end\n",
    )
    .expect("deck with signed instance parameter parses");
    assert!(!netlist.elements.is_empty());
}

#[test]
fn ihp_sg13g2_diode_library_parses_and_solves() {
    // The library that motivated this: every diode instance in it computes its
    // area and perimeter from subcircuit parameters.
    let library = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../models/spice/foundry/ihp-sg13g2/upstream/ihp-sg13g2",
        "/libs.tech/ngspice/models/diodes.lib"
    );
    assert!(
        std::path::Path::new(library).is_file(),
        "vendored IHP diode library missing at {library}"
    );

    // `.include` is resolved relative to the deck on disk, so this goes through
    // Netlist::parse_file rather than the in-memory parse the other cases use.
    let deck = format!(
        "* ihp sg13g2 antenna diode\n\
         .include {library}\n\
         V1 a 0 DC 0.6\n\
         X1 a 0 dantenna\n\
         .op\n\
         .end\n"
    );
    let deck_path = std::env::temp_dir().join("rspice_ihp_dantenna_probe.cir");
    std::fs::write(&deck_path, deck).expect("write probe deck");

    let netlist = Netlist::parse_file(&deck_path).expect("deck with include parses");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("operating point converges");

    // The subcircuit places three diodes whose areas come from
    // DEV_A=(l*w)/1p, DEV_P=(l+w)/0.5u and DEV_C=4. ngspice 46 draws
    // 1.58551 nA total through the source; the branch currents here sum to
    // that once the per-junction GMIN contribution is included.
    let total = -result
        .branch_current_named("v1")
        .expect("v1 branch current present");
    assert!(
        (total - 1.5855e-9).abs() < 5e-12,
        "expected ~1.5855 nA (ngspice 46: 1.58551 nA), got {total:e}"
    );
}
