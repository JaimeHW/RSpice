//! Operating points that only a convergence aid finds, on the Xyce regression
//! decks vendored under `tests/xyce`, against the reference simulators.
//!
//! These decks are too large for the unoptimised `rspice-core` test build, so
//! they run here, in release.

use rspice_core::NoAbort;
use rspice_core::engine::{
    ContinuationAid, Engine, SimulationConfig, SpiceDialect, resolve_simulation_config,
};
use rspice_core::netlist::Netlist;
use rspice_core::notice::{EngineNotice, NoticeCollector};
use std::path::Path;

/// QSPICE's operating point of CircuitSim90's MOS2 chip2 at four nodes: one
/// held only by device GMIN behind an off pass gate, an inverter output low
/// and one high, and the node where RSpice and QSPICE differ most. QSPICE
/// loads a junction conductance of 1.026e-12 S where RSpice loads 1e-12 S,
/// which moves its GMIN dividers by up to 0.14 V.
const QSPICE_CHIP2_VOLTAGES: [(&str, f64); 4] = [
    ("xslice10a0.xstrip10.13", 3.689578),
    ("xslice19e0.xstrip50.16", 0.026572),
    ("100", 5.0),
    ("xmuxouth0.xottop0.14", 3.335292),
];

/// chip2 is a two-phase shift-register array whose phi1 pass gates are off at
/// t = 0, so 2184 of its nodes are held only by the junction and channel
/// GMIN of the devices around them. Its direct Newton fails. Dynamic GMIN
/// stepping walks a conductance from every node to ground down to the device
/// GMIN, where that pull is still a quarter of those nodes' conductance and
/// most of the chip sits in another logic state; its closing solve on the
/// floor cannot flip them all at once. ngspice-46 fails its dynamic GMIN
/// stepping on this deck the same way and finds the operating point with
/// true GMIN stepping (`new_gmin`), which steps the device GMIN over the
/// nodal floor. QSPICE's GMIN stepping finds the same operating point.
/// CircuitSim90 MOS3 gm17 needs GMIN stepping. ngspice-46's dynamic GMIN
/// stepping, which holds the device GMIN, resolves its latch to 0 V and 5 V;
/// stepping the junction GMIN with the nodes leaves it at its 2.4543 V
/// metastable point, as LTspice does. QSPICE resolves it as ngspice does, but
/// the default dialect keeps the junction stepping that finds chip2's and
/// gstep2's operating points.
///
/// The deck's `.IC` line is left out. ngspice-46 finds the same operating
/// point in the same 50 iterations with it and without it: its `.IC` seeds
/// the node vector (CKTic), which every MOSFET's MODEINITJCT load ignores.
/// RSpice's ngspice dialect starts the direct Newton from the seed, which
/// converges there with the latch in the `.IC`'s state.
#[test]
fn the_ngspice_dialect_resolves_the_gm17_latch_as_ngspice_does() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/xyce/Netlists/CircuitSim90/MOS3/gm17.cir");
    let deck: String = std::fs::read_to_string(&path)
        .expect("gm17 reads")
        .lines()
        .filter(|line| !line.trim_start().to_ascii_lowercase().starts_with(".ic"))
        .map(|line| format!("{line}\n"))
        .collect();
    let netlist = Netlist::parse(&deck).expect("gm17 parses");
    let engine = Engine::new(resolve_simulation_config(
        &SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice),
        Some(&netlist.options),
        &Default::default(),
    ));
    let result = engine
        .run_dc_op_with_abort(&netlist, &NoAbort)
        .expect("gm17 has an operating point");
    // ngspice-46's operating point.
    for (node, ngspice) in [("4", 0.0), ("5", 5.0), ("16", 5.0), ("17", 0.0)] {
        let actual = result
            .try_voltage_named(node)
            .unwrap_or_else(|| panic!("missing voltage for {node}"));
        assert!(
            (actual - ngspice).abs() <= 1.0e-3,
            "V({node}) = {actual}, ngspice-46 {ngspice}"
        );
    }
}

#[test]
fn chip2_operating_point_comes_from_true_gmin_stepping() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/xyce/Netlists/CircuitSim90/MOS2_LARGE/chip2.cir");
    let netlist = Netlist::parse_file(&path).expect("chip2 parses");
    let engine = Engine::new(resolve_simulation_config(
        &SimulationConfig::default(),
        Some(&netlist.options),
        &Default::default(),
    ));

    let collector = NoticeCollector::new();
    let _installed = collector.install();
    let result = engine
        .run_dc_op_with_abort(&netlist, &NoAbort)
        .expect("chip2 has an operating point");
    let aids: Vec<ContinuationAid> = collector
        .take()
        .into_iter()
        .filter_map(|recorded| match recorded.notice {
            EngineNotice::ConvergenceAid(aid) => Some(aid),
            _ => None,
        })
        .collect();
    assert_eq!(
        aids,
        [ContinuationAid::GminStepping],
        "GMIN stepping answers, before source stepping or pseudo-transient runs"
    );

    for (node, qspice) in QSPICE_CHIP2_VOLTAGES {
        let actual = result
            .try_voltage_named(node)
            .unwrap_or_else(|| panic!("missing voltage for {node}"));
        assert!(
            (actual - qspice).abs() <= 0.15,
            "V({node}) = {actual}, QSPICE {qspice}"
        );
    }
}
