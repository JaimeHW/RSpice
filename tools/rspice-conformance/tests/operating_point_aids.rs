//! Operating points that only a convergence aid finds, on the Xyce regression
//! decks vendored under `tests/xyce`, against the reference simulators.
//!
//! These decks are too large for the unoptimised `rspice-core` test build, so
//! they run here, in release.

use rspice_core::NoAbort;
use rspice_core::engine::{ContinuationAid, Engine, SimulationConfig, resolve_simulation_config};
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
