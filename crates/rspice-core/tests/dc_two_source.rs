//! Two-source `.DC` sweep: `.DC V1 a b s V2 a2 b2 s2` sweeps the first
//! (inner) source fully at every value of the second (outer) source,
//! ngspice-style, with results concatenated in outer order.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{AnalysisCommand, Netlist};

/// Equal 1k resistors from each source into `out`, 1k to ground:
/// v(out) = (v1 + v2) / 3 — every grid point has a closed form.
const DECK: &str = "\
* two source dc sweep
v1 in1 0 dc 0
v2 in2 0 dc 0
r1 in1 out 1k
r2 in2 out 1k
r3 out 0 1k
.dc v1 0 2 1 v2 0 3 1.5
.end
";

#[test]
fn second_source_parses_into_the_analysis() {
    let netlist = Netlist::parse(DECK).expect("deck parses");
    let dc = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Dc { source, sweep2, .. } => Some((source.clone(), sweep2.clone())),
            _ => None,
        })
        .expect(".dc card present");
    assert!(dc.0.eq_ignore_ascii_case("v1"));
    let sweep2 = dc.1.expect("second sweep captured");
    assert!(sweep2.source.eq_ignore_ascii_case("v2"));
    assert_eq!((sweep2.start, sweep2.stop, sweep2.step), (0.0, 3.0, 1.5));
}

#[test]
fn single_source_dc_still_parses_without_sweep2() {
    let deck = "* single\nv1 a 0 dc 1\nr1 a 0 1k\n.dc v1 0 1 0.5\n.end\n";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let has_plain_dc = netlist.analyses.iter().any(
        |analysis| matches!(analysis, AnalysisCommand::Dc { sweep2: None, .. }),
    );
    assert!(has_plain_dc, "single-source .DC must carry no second sweep");
}

#[test]
fn nested_sweep_covers_the_full_grid_in_outer_order() {
    let netlist = Netlist::parse(DECK).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let sweep2 = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Dc { sweep2, .. } => sweep2.clone(),
            _ => None,
        })
        .expect("second sweep present");

    let results = engine
        .run_dc_sweep2_with_abort(
            &netlist,
            "v1",
            0.0,
            2.0,
            1.0,
            Some(&sweep2),
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("two-source sweep solves");

    let inner = [0.0, 1.0, 2.0];
    let outer = [0.0, 1.5, 3.0];
    assert_eq!(results.len(), inner.len() * outer.len());

    for (o, &v2) in outer.iter().enumerate() {
        for (i, &v1) in inner.iter().enumerate() {
            let (sweep_value, point) = &results[o * inner.len() + i];
            assert!(
                (sweep_value - v1).abs() < 1e-12,
                "point {o},{i}: sweep column must carry the inner value"
            );
            let out_idx = point
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .expect("out node present");
            let expected = (v1 + v2) / 3.0;
            let got = point.node_voltages[out_idx];
            assert!(
                (got - expected).abs() < 1e-9,
                "v(out) at v1={v1}, v2={v2}: expected {expected}, got {got}"
            );
        }
    }
}

#[test]
fn temp_as_outer_sweep_runs() {
    let deck = "\
* temp outer sweep
v1 a 0 dc 1
r1 a 0 1k
.dc v1 0 1 1 temp 0 50 50
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let sweep2 = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Dc { sweep2, .. } => sweep2.clone(),
            _ => None,
        })
        .expect("second sweep present");
    let results = engine
        .run_dc_sweep2_with_abort(
            &netlist,
            "v1",
            0.0,
            1.0,
            1.0,
            Some(&sweep2),
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("temp outer sweep solves");
    assert_eq!(results.len(), 4, "2 inner x 2 outer points");
}
