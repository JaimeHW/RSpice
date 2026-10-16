//! The refusals a prepared run states are the engine's own, read off the
//! decks it dispatches, and nothing the engine accepts.

use super::*;

fn refusals_of(deck: &str, tasks: Vec<PreparedTask>) -> Vec<EngineRefusal> {
    let mut run = parts();
    run.executable_netlist = deck.to_owned();
    run.tasks = tasks;
    PreparedRunSnapshot::new(run)
        .expect("the deck prepares")
        .engine_refusals(&rspice_core::NoAbort)
}

const FORCED_ISLAND: &str = "forced island\n\
V1 in 0 1\n\
R1 in 0 1k\n\
I1 0 out dc 1m\n\
C1 out 0 1u\n\
.op\n\
.end\n";

/// Current forced into a capacitor island is refused by the operating point,
/// before it solves. A transient is not claimed: its refusal is decided from
/// the `t = 0` waveform after startup recovery, which a build does not see.
#[test]
fn a_forced_capacitor_island_is_refused_by_the_operating_point_only() {
    let refusals = refusals_of(
        FORCED_ISLAND,
        vec![
            prepared("op", "DC Operating Point", task()),
            prepared("tran", "Transient", transient_task()),
        ],
    );

    assert_eq!(refusals.len(), 1, "{refusals:?}");
    let EngineRefusalKind::NoDcPath { nodes } = &refusals[0].kind else {
        panic!("{refusals:?}");
    };
    assert_eq!(nodes.len(), 1);
    assert!(nodes[0].eq_ignore_ascii_case("out"), "{nodes:?}");
    assert_eq!(
        refusals[0].tasks,
        [RefusedTask {
            analysis: instance_id("op"),
            label: "DC Operating Point".to_owned(),
        }]
    );
}

/// A coupling capacitor's island has no DC path either, but nothing forces a
/// current into it and the engine solves it. It is not refused.
#[test]
fn an_unforced_coupling_capacitor_island_is_not_refused() {
    let refusals = refusals_of(
        "coupling island\nV1 in 0 1\nR1 in 0 1k\nC1 in mid 1u\nC2 mid 0 1u\n.op\n.end\n",
        vec![prepared("op", "DC Operating Point", task())],
    );
    assert!(refusals.is_empty(), "{refusals:?}");
}

/// `.OPTIONS RSHUNT` is a real path from every node to ground, so the engine
/// solves the forced island and there is nothing to refuse.
#[test]
fn rshunt_in_the_deck_clears_the_forced_island() {
    let shunted = FORCED_ISLAND.replace(".op\n", ".options rshunt=1e9\n.op\n");
    let refusals = refusals_of(&shunted, vec![prepared("op", "DC Operating Point", task())]);
    assert!(refusals.is_empty(), "{refusals:?}");
}

/// A source across a winding is a loop of fixed branch voltages in the DC
/// solve, which every operating point refuses. A transient steps the winding
/// as an impedance; its startup is left to the engine and is not claimed.
#[test]
fn a_source_across_a_winding_is_refused_where_a_dc_point_is_solved() {
    let refusals = refusals_of(
        "source across winding\nV1 a 0 1\nL1 a 0 1u\nR1 a 0 1k\n.op\n.end\n",
        vec![
            prepared("op", "DC Operating Point", task()),
            prepared("tran", "Transient", transient_task()),
        ],
    );

    assert_eq!(refusals.len(), 1, "{refusals:?}");
    let EngineRefusalKind::BranchVoltageLoop(branch_loop) = &refusals[0].kind else {
        panic!("{refusals:?}");
    };
    assert!(branch_loop.through_inductor);
    assert_eq!(branch_loop.members.len(), 2, "{branch_loop:?}");
    assert_eq!(refusals[0].tasks.len(), 1);
    assert_eq!(refusals[0].tasks[0].analysis, instance_id("op"));
}

/// Two sources across one node pair are the same singular DC solve, stated
/// with the same scope; the loop says it holds no winding.
#[test]
fn a_loop_of_voltage_sources_alone_is_refused_where_a_dc_point_is_solved() {
    let refusals = refusals_of(
        "source across source\nV1 a 0 1\nV2 a 0 2\nR1 a 0 1k\n.op\n.end\n",
        vec![
            prepared("op", "DC Operating Point", task()),
            prepared("tran", "Transient", transient_task()),
        ],
    );

    assert_eq!(refusals.len(), 1, "{refusals:?}");
    let EngineRefusalKind::BranchVoltageLoop(branch_loop) = &refusals[0].kind else {
        panic!("{refusals:?}");
    };
    assert!(!branch_loop.through_inductor);
    assert_eq!(
        refusals[0].tasks,
        [RefusedTask {
            analysis: instance_id("op"),
            label: "DC Operating Point".to_owned(),
        }]
    );
}

/// The run's own deck is covered, so preparation accepts it. One Run Set
/// point shortens the device below every bin, and the build of that point's
/// deck is refused: the refusal names the instance, its evaluated geometry,
/// the nearest bin, and only the task at that point.
///
/// The device sits in a cell, as a drawn transistor does below the root, and
/// its geometry is evaluated when the cell is placed, against the deck's
/// parameters as the point's override card leaves them.
#[test]
fn an_instance_outside_its_bins_at_one_point_is_refused_at_that_point_only() {
    let mut run = parts();
    run.executable_netlist = "binned at one point\n\
.param LVAL=0.5u\n\
.subckt cell d\n\
M1 d d 0 0 NCH W=1u L={LVAL}\n\
.ends cell\n\
V1 d 0 1\n\
X1 d cell\n\
.model NCH.0 NMOS LEVEL=1 LMIN=0.28u LMAX=0.5u WMIN=0.5u WMAX=2u VTO=0.4\n\
.model NCH.1 NMOS LEVEL=1 LMIN=0.5u LMAX=1.2u WMIN=0.5u WMAX=2u VTO=0.9\n\
.op\n\
.end\n"
        .to_owned();
    run.run_set = Some(global_parameter_run_set("LVAL", &["0.5u", "0.1u"]));
    let snapshot = PreparedRunSnapshot::new(run).expect("the reference deck is covered");

    let refusals = snapshot.engine_refusals(&rspice_core::NoAbort);

    assert_eq!(refusals.len(), 1, "{refusals:?}");
    let EngineRefusalKind::UncoveredModelBin(gap) = &refusals[0].kind else {
        panic!("{refusals:?}");
    };
    assert!(gap.element.to_ascii_lowercase().ends_with("m1"), "{gap:?}");
    assert!(
        gap.length
            .is_some_and(|length| (length - 0.1e-6).abs() < 1e-15),
        "{gap:?}"
    );
    assert!(gap.nearest.model.eq_ignore_ascii_case("nch.0"), "{gap:?}");
    assert_eq!(refusals[0].tasks.len(), 1, "{refusals:?}");
    assert!(
        refusals[0].tasks[0].label.contains("param LVAL=0.1u"),
        "{refusals:?}"
    );
}

/// A mixed-discipline connection is bridged by the connect module the builder
/// inserts, so drawing none is normal. Connect rules that cannot settle the
/// boundary are the builder's refusal, and that is what is stated.
#[test]
fn a_boundary_the_connect_rules_cannot_bridge_is_the_builders_refusal() {
    let (sources, deck) = crate::veriloga::tests::standalone_connection_fixture();
    let bridged = || {
        let mut input = parts();
        input.tasks = vec![prepared("tran", "Transient", transient_task())];
        input.executable_netlist = deck.replace(".tran 0.2n 2n", ".tran 1n 1u");
        input.project_veriloga_runtimes = sources.clone();
        input
    };
    let accepted = PreparedRunSnapshot::new(bridged())
        .expect("the fixture prepares")
        .engine_refusals(&rspice_core::NoAbort);
    assert!(accepted.is_empty(), "{accepted:?}");

    let mut unbridged = bridged();
    unbridged.executable_netlist = unbridged
        .executable_netlist
        .replace("connectrules=Low", "connectrules=NotThere");
    let refusals = PreparedRunSnapshot::new(unbridged)
        .expect("selection is the builder's, not preparation's")
        .engine_refusals(&rspice_core::NoAbort);

    assert_eq!(refusals.len(), 1, "{refusals:?}");
    let EngineRefusalKind::Build { message, .. } = &refusals[0].kind else {
        panic!("{refusals:?}");
    };
    assert!(message.contains("NotThere"), "{message}");
}
