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

/// The shunt stated for one analysis, as the solver catalogue authors it,
/// rides that analysis's own deck and clears the refusal for it alone.
#[test]
fn a_node_shunt_stated_for_one_analysis_clears_that_analysis_alone() {
    use rspice_simulation_contract::analysis_kind::AnalysisKind;
    use rspice_simulation_contract::numeric_override::{
        AnalysisNumericOverride, NumericOverrideOption, SolverOwnership,
    };

    let mut record = AnalysisNumericOverride::default();
    record
        .set_for_instance(
            AnalysisKind::OperatingPoint,
            SolverOwnership::NONE,
            NumericOverrideOption::Rshunt,
            "1e9",
        )
        .expect("an operating point carries a node shunt");
    let mut shunted = task();
    shunted.numeric_override = Some(record);

    let refusals = refusals_of(
        FORCED_ISLAND,
        vec![
            prepared("op", "DC Operating Point", task()),
            prepared("shunted", "Shunted Operating Point", shunted),
        ],
    );

    assert_eq!(refusals.len(), 1, "{refusals:?}");
    assert_eq!(
        refusals[0].tasks,
        [RefusedTask {
            analysis: instance_id("op"),
            label: "DC Operating Point".to_owned(),
        }]
    );
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

/// A Run Set point's deck is built at the temperature the runner writes into
/// it before parsing, so `.if (TEMP > 50)` selects as the run will. The deck
/// at its own temperature takes a length its bins cover; at 75 °C it takes
/// one below every bin. The pass refuses the hot point alone, and the runner,
/// given each point as it dispatches it, refuses exactly the points the pass
/// refused.
#[test]
fn a_run_set_point_is_built_at_the_temperature_the_runner_writes_into_its_deck() {
    let mut run = parts();
    run.executable_netlist = "temperature selects the length\n\
.if (TEMP > 50)\n\
.param lval=0.1u\n\
.else\n\
.param lval=0.5u\n\
.endif\n\
M1 d d 0 0 NCH W=1u L={lval}\n\
V1 d 0 1\n\
.model NCH.0 NMOS LEVEL=1 LMIN=0.28u LMAX=0.5u WMIN=0.5u WMAX=2u VTO=0.4\n\
.tran 1n 1u\n\
.end\n"
        .to_owned();
    run.tasks = vec![prepared("tran", "Transient", transient_task())];
    run.run_set = Some(global_temperature_run_set(&["27", "75"]));
    let snapshot = PreparedRunSnapshot::new(run).expect("the deck's own length is covered");

    let refusals = snapshot.engine_refusals(&rspice_core::NoAbort);

    assert_eq!(refusals.len(), 1, "{refusals:?}");
    let EngineRefusalKind::UncoveredModelBin(gap) = &refusals[0].kind else {
        panic!("{refusals:?}");
    };
    assert!(
        gap.length
            .is_some_and(|length| (length - 0.1e-6).abs() < 1e-15),
        "{gap:?}"
    );
    assert_eq!(refusals[0].tasks.len(), 1, "{refusals:?}");
    assert!(refusals[0].tasks[0].label.contains("75"), "{refusals:?}");
    for task in &snapshot.tasks {
        let ran = crate::engine_bridge::EngineBridge::new()
            .run_with_abort_and_source_path_and_environment(
                task.task
                    .config
                    .as_ref()
                    .expect("a transient runs through its configuration"),
                task.executable_netlist_override
                    .as_deref()
                    .unwrap_or(&snapshot.executable_netlist),
                None,
                task.execution_environment.clone(),
                &rspice_core::NoAbort,
            );
        let refused = refusals[0].tasks.iter().any(|refused| refused.label == task.label);
        assert_eq!(
            ran.is_err(),
            refused,
            "{}: {:?}",
            task.label,
            ran.err()
        );
    }
}

/// A Stop raised anywhere in the pass, the scan that names the instances no
/// bin covers included, leaves no refusal: the pass states what the engine
/// refuses of a run it finished checking, and a stopped pass finished none.
#[test]
fn a_pass_stopped_at_any_poll_states_no_refusal() {
    use rspice_core::abort_signal::CountingAbort;

    let mut run = parts();
    run.executable_netlist = "binned at one point\n\
.param LVAL=0.5u\n\
M1 d d 0 0 NCH W=1u L={LVAL}\n\
V1 d 0 1\n\
.model NCH.0 NMOS LEVEL=1 LMIN=0.28u LMAX=0.5u WMIN=0.5u WMAX=2u VTO=0.4\n\
.op\n\
.end\n"
        .to_owned();
    run.run_set = Some(global_parameter_run_set("LVAL", &["0.5u", "0.1u"]));
    let snapshot = PreparedRunSnapshot::new(run).expect("the reference deck is covered");
    let unstopped = CountingAbort::new(usize::MAX);
    assert_eq!(snapshot.engine_refusals(&unstopped).len(), 1);

    for threshold in 0..unstopped.count() {
        let refusals = snapshot.engine_refusals(&CountingAbort::new(threshold));
        assert!(
            refusals.is_empty(),
            "a Stop at poll {}: {refusals:?}",
            threshold + 1
        );
    }
}

/// A corner run's points dispatch the decks their preparation composed, each
/// with its own process's binned cards, and the engine builds exactly those;
/// the declaration assembles the points' results and never reaches it, so it
/// is not built. A device every process's bins cover is refused nowhere, as a
/// second copy of a point's cards, which overlaps its bins, would refuse it.
#[test]
fn a_corner_run_is_built_as_its_points_dispatch_it() {
    use crate::netlist_preparation::{REFERENCE_MODEL_BINDING_BEGIN, REFERENCE_MODEL_BINDING_END};

    let cards = |vto: &str| {
        format!(
            ".model nch.1 nmos level=1 lmin=0.1u lmax=1u wmin=0.1u wmax=100u vto={vto}\n\
             .model nch.2 nmos level=1 lmin=1u lmax=10u wmin=0.1u wmax=100u vto={vto}"
        )
    };
    let binding = |process: ProcessCorner, vto: &str| rspice_model_library::CornerModelBinding {
        process,
        source_label: format!("corners.lib [{}]", process.short_name()),
        section: Some(process.short_name().to_owned()),
        materialized_model_cards: cards(vto),
    };
    let mut corners = corner_task(
        vec![ProcessCorner::SS, ProcessCorner::FF],
        vec![1.0],
        vec![27.0],
        true,
    );
    corners
        .spec_options
        .corner
        .as_mut()
        .expect("a corner task carries its contract")
        .model_bindings = vec![
        binding(ProcessCorner::SS, "0.55"),
        binding(ProcessCorner::FF, "0.45"),
    ];
    let mut run = parts();
    run.executable_netlist = format!(
        "corner devices\nVDD d 0 1\nVG g 0 1\nM1 d g 0 0 nch L=0.5u W=10u\n\
         {REFERENCE_MODEL_BINDING_BEGIN} 2\n{}\n{REFERENCE_MODEL_BINDING_END}\n.op\n.end\n",
        cards("0.5")
    );
    run.tasks = vec![prepared("corners", "Corner run", corners)];
    let snapshot = PreparedRunSnapshot::new(run).expect("every corner's deck is covered");
    assert_eq!(
        snapshot
            .tasks
            .iter()
            .filter(|task| task.declared_point.is_some())
            .count(),
        2,
        "one point per process"
    );

    let progress = std::cell::Cell::new(DeckBuildProgress::default());
    let refusals =
        snapshot.engine_refusals_reporting(&rspice_core::NoAbort, &|now| progress.set(now));

    assert!(refusals.is_empty(), "{refusals:#?}");
    assert_eq!(
        progress.get(),
        DeckBuildProgress { built: 2, total: 2 },
        "one deck per point, and none for the declaration"
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
    let EngineRefusalKind::ConnectRules { message, instance } = &refusals[0].kind else {
        panic!("a connect-rule refusal is told apart from other builds: {refusals:?}");
    };
    assert!(message.contains("NotThere"), "{message}");
    assert_eq!(instance, &None, "an unknown block is no instance's");
}
