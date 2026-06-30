//! Two-source `.DC` sweep: `.DC V1 a b s V2 a2 b2 s2` sweeps the first
//! (inner) source fully at every value of the second (outer) source,
//! ngspice-style, with results concatenated in outer order.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{AnalysisCommand, DcSweepMode, Netlist};

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
    let has_plain_dc = netlist
        .analyses
        .iter()
        .any(|analysis| matches!(analysis, AnalysisCommand::Dc { sweep2: None, .. }));
    assert!(has_plain_dc, "single-source .DC must carry no second sweep");
}

#[test]
fn xyce_dc_upgrade_sweep_modes_parse_and_generate_points() {
    let list = single_dc_analysis(".dc vt1 list -17 12 188 5");
    assert!(matches!(list.mode, DcSweepMode::List(_)));
    assert_eq!(
        rspice_core::netlist::DcSweepSpec {
            start: list.start,
            stop: list.stop,
            step: list.step,
            mode: list.mode.clone(),
        }
        .points(),
        vec![-17.0, 12.0, 188.0, 5.0]
    );

    let list_prefix = single_dc_analysis(".dc list vt1 -17 12 188 5");
    assert_eq!(
        rspice_core::netlist::DcSweepSpec {
            start: list_prefix.start,
            stop: list_prefix.stop,
            step: list_prefix.step,
            mode: list_prefix.mode,
        }
        .points(),
        vec![-17.0, 12.0, 188.0, 5.0]
    );

    let dec = single_dc_analysis(".dc dec vt1 0.1 100 4");
    assert!(matches!(
        dec.mode,
        DcSweepMode::Decade {
            points_per_decade: 4
        }
    ));
    let dec_points = rspice_core::netlist::DcSweepSpec {
        start: dec.start,
        stop: dec.stop,
        step: dec.step,
        mode: dec.mode,
    }
    .points();
    assert_eq!(dec_points.len(), 13);
    assert!((dec_points[0] - 0.1).abs() < 1e-12);
    assert!((dec_points[12] - 100.0).abs() < 1e-9);

    let dec_suffix = single_dc_analysis(".dc vt1 dec 0.1 100 4");
    assert_eq!(
        rspice_core::netlist::DcSweepSpec {
            start: dec_suffix.start,
            stop: dec_suffix.stop,
            step: dec_suffix.step,
            mode: dec_suffix.mode,
        }
        .points()
        .len(),
        13
    );

    let oct = single_dc_analysis(".dc oct vt1 0.125 66 3");
    assert!(matches!(
        oct.mode,
        DcSweepMode::Octave {
            points_per_octave: 3
        }
    ));
    let oct_points = rspice_core::netlist::DcSweepSpec {
        start: oct.start,
        stop: oct.stop,
        step: oct.step,
        mode: oct.mode,
    }
    .points();
    assert_eq!(oct_points.len(), 28);
    assert!((oct_points[0] - 0.125).abs() < 1e-12);
    assert!((oct_points[27] - 64.0).abs() < 1e-9);

    let oct_suffix = single_dc_analysis(".dc vt1 oct 0.125 66 3");
    assert_eq!(
        rspice_core::netlist::DcSweepSpec {
            start: oct_suffix.start,
            stop: oct_suffix.stop,
            step: oct_suffix.step,
            mode: oct_suffix.mode,
        }
        .points()
        .len(),
        28
    );
}

struct ParsedDc {
    start: f64,
    stop: f64,
    step: f64,
    mode: DcSweepMode,
}

fn single_dc_analysis(dc_line: &str) -> ParsedDc {
    let deck = format!("dc mode\nvt1 4 0 0\nr1 4 0 1k\n{dc_line}\n.end\n");
    let netlist = Netlist::parse(&deck).expect("deck parses");
    netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Dc {
                start,
                stop,
                step,
                mode,
                ..
            } => Some(ParsedDc {
                start: *start,
                stop: *stop,
                step: *step,
                mode: mode.clone(),
            }),
            _ => None,
        })
        .expect(".dc card present")
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

#[test]
fn current_source_as_outer_sweep_runs_full_grid() {
    let deck = "\
* current source outer sweep
v1 in 0 dc 0
i1 out 0 0
r1 in out 1k
r2 out 0 1k
.dc v1 0 2 1 i1 -1m 1m 1m
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
            2.0,
            1.0,
            Some(&sweep2),
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("current-source outer sweep solves");

    let inner = [0.0, 1.0, 2.0];
    let outer = [-1.0e-3, 0.0, 1.0e-3];
    assert_eq!(results.len(), inner.len() * outer.len());

    for (o, &i1) in outer.iter().enumerate() {
        for (i, &v1) in inner.iter().enumerate() {
            let (sweep_value, point) = &results[o * inner.len() + i];
            assert!(
                (sweep_value - v1).abs() < 1e-12,
                "point {o},{i}: sweep column must carry the inner value"
            );
            let actual = point
                .try_voltage_named("out")
                .expect("out node voltage is present");
            let expected = (v1 - i1 * 1000.0) / 2.0;
            assert!(
                (actual - expected).abs() < 1e-9,
                "V(out) at v1={v1}, i1={i1}: expected {expected}, got {actual}"
            );
        }
    }
}
