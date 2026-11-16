//! A Verilog and a Verilog-AMS view placed on a schematic run as the same
//! modules do in a manual deck that loads them with `.hdl`.
//!
//! The design route binds each view's project source with the sealed project
//! directive; the deck route seals each `.hdl` file. Both reach the engine as
//! one compiled runtime per module, so a run of each through the real
//! controller publishes the same traces. A discrete vector port is a bus pin
//! whose conductors are the nets the engine's boundary binds, so a bus drawn
//! between two of them runs as the deck that names those nets.

use crate::simulation::controller::SimulationController;
use crate::simulation::controller::test_execution::{run_generated_batch, run_manual_batch_at};
use crate::simulation::controller::test_host::RunTestHost;
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::state::{
    Cell, CellViewRef, ComponentType, Library, LibraryCellInstance, Point, ProjectSourceBundle,
    ProjectSourceLanguage, ProjectSourceOwner, RetainedTrace, View, ViewType,
};
use rspice_app_state::workbench::app_state::model::{
    apply_compiled_interface, compile_project_bundle_receipt,
};
use rspice_results::analysis_payload::AnalysisResultPayload;
use rspice_results::run::SimulationRunLifecycle;
use rspice_simulation::output_contract::TransientSelectionGrid;
use rspice_simulation::output_contract::materialization::resample_selected_and_final;

const LIBRARY: &str = "hdl";

/// A purely digital module: a register that toggles every 5 ns, IEEE
/// 1364-2005 alone.
const BLINK: &str = "`timescale 1ns/1ps\nmodule blink(q);\n    output q;\n    reg q;\n    \
                     initial q = 1'b0;\n    always #5 q = ~q;\nendmodule\n";

/// A mixed-signal module: its analog block reads its discrete input.
const LEVEL: &str = "`include \"disciplines.vams\"\n`timescale 1ns/1ps\nmodule level(d, out);\n    \
                     input d;\n    output out;\n    logic d;\n    electrical out;\n    \
                     analog V(out) <+ transition(d ? 1.0 : 0.0, 0, 1n, 1n);\nendmodule\n";

/// A purely digital counter: its two-bit output steps 0, 1, 2, 3 every
/// 5 ns.
const COUNT: &str = "`timescale 1ns/1ps\nmodule count(mode);\n    output [1:0] mode;\n    \
                     reg [1:0] mode;\n    initial mode = 2'd0;\n    \
                     always #5 mode = mode + 2'd1;\nendmodule\n";

/// A conductance its two-bit discrete input selects: 1 mS, and 2 mS more for
/// the most significant bit, 1 mS more for the least.
const MODED: &str = "`include \"disciplines.vams\"\n`timescale 1ns/1ps\n\
                     module moded(p, n, mode);\n    inout p, n;\n    input [1:0] mode;\n    \
                     electrical p, n;\n    logic mode;\n    \
                     analog I(p, n) <+ (1m + (mode[1] ? 2m : 0.0) + (mode[0] ? 1m : 0.0)) \
                     * V(p, n);\nendmodule\n";

/// A cell of `LIBRARY` whose `view_type` view owns `source` as its project
/// source, compiled, with the interface and the generated symbol that clean
/// compile gives it: the path Compile takes.
fn add_source_cell(
    state: &mut RunTestHost,
    cell: &str,
    view_type: ViewType,
    root: &str,
    source: &str,
) -> CellViewRef {
    if state.model.library_manager.get_library(LIBRARY).is_none() {
        state
            .model
            .library_manager
            .add_library(Library::new(LIBRARY));
    }
    let reference = CellViewRef::new(LIBRARY, cell, view_type.display_name());
    let mut view = View::new(view_type.display_name(), view_type);
    view.metadata
        .insert("veriloga.module".to_owned(), cell.to_owned());
    let mut library_cell = Cell::new(cell);
    library_cell.add_view(view);
    state
        .model
        .library_manager
        .get_library_mut(LIBRARY)
        .expect("the library")
        .add_cell(library_cell);
    let bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::cell_view(reference.clone()),
        ProjectSourceLanguage::VerilogA,
        root,
        source,
        [],
        [],
    )
    .expect("a cell-view source");
    state
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle)
        .expect("one source per view");
    let bundle = state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::cell_view(reference.clone()))
        .expect("the view's source");
    let receipt = compile_project_bundle_receipt(
        state.model.workspace.content.project.id(),
        bundle,
        Some(cell),
    )
    .unwrap_or_else(|diagnostics| panic!("{cell} compiles: {diagnostics:#?}"));
    let applied = apply_compiled_interface(
        &mut state.model.library_manager,
        &state.model.workspace.content.schematic_buffers,
        &reference,
        receipt.compilation.report(),
    );
    assert!(applied.wrote, "{cell}: {:?}", applied.outcome);
    assert!(
        state
            .model
            .library_manager
            .get_library(LIBRARY)
            .and_then(|library| library.get_cell(cell))
            .and_then(|cell| cell.get_view(rspice_design::symbol::generated::GENERATED_SYMBOL_VIEW))
            .is_some(),
        "{cell}'s compile drew its symbol"
    );
    reference
}

/// The point `length` out from a terminal at `at`, away from the body
/// centred on `origin`.
fn outward(at: Point, origin: Point, length: i32) -> Point {
    if (at.x - origin.x).abs() >= (at.y - origin.y).abs() {
        Point::new(at.x + (at.x - origin.x).signum() * length, at.y)
    } else {
        Point::new(at.x, at.y + (at.y - origin.y).signum() * length)
    }
}

/// Wire each of `terminals` out to a stub carrying the net label `nets`.
fn label_terminals(state: &mut RunTestHost, terminals: Vec<Point>, origin: Point, nets: &[&str]) {
    assert_eq!(terminals.len(), nets.len());
    for (at, net) in terminals.into_iter().zip(nets) {
        let outward = outward(at, origin, 20);
        state
            .model
            .schematic
            .add_wire(vec![at, outward])
            .expect("the stub commits");
        state
            .model
            .schematic
            .add_net_label(outward, (*net).to_owned());
    }
}

/// Place an instance of `reference` named `name` at `origin`, label each
/// scalar terminal with the net `nets` gives it by name, and hand back where
/// each bus pin landed, by name.
fn place(
    state: &mut RunTestHost,
    reference: &CellViewRef,
    name: &str,
    origin: Point,
    nets: &[(&str, &str)],
) -> Vec<(String, Point)> {
    let ports = rspice_design::library::metadata::metadata_ports(
        &state
            .model
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .expect("the view")
            .metadata,
    )
    .expect("the view states its ports");
    let mut binding =
        LibraryCellInstance::new(&reference.library, &reference.cell, &reference.view);
    binding.bind_interface(&ports);
    let id = state
        .model
        .schematic
        .add_library_cell_component(origin, binding);
    let component = state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("the instance");
    component.name = name.to_owned();
    let (mut scalars, mut labels, mut buses) = (Vec::new(), Vec::new(), Vec::new());
    let names = component
        .library_cell
        .as_ref()
        .expect("a cell instance")
        .terminal_order
        .clone();
    for (terminal, (_, at)) in names.into_iter().zip(component.terminal_positions()) {
        if rspice_design::schematic::bus::declared_vector(&terminal).is_some() {
            buses.push((terminal, at));
            continue;
        }
        let net = nets
            .iter()
            .find(|(port, _)| port.eq_ignore_ascii_case(&terminal))
            .unwrap_or_else(|| panic!("{name}: no net for {terminal}"))
            .1;
        scalars.push(at);
        labels.push(net);
    }
    label_terminals(state, scalars, origin, &labels);
    buses
}

/// A two-terminal device of `kind`, its terminals on the nets `nets`.
fn device(
    state: &mut RunTestHost,
    kind: ComponentType,
    name: &str,
    value: &str,
    origin: Point,
    nets: &[&str],
) {
    let id = state.model.schematic.add_component(kind, origin);
    let component = state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("the device");
    component.name = name.to_owned();
    component.value = value.to_owned();
    let terminals = rspice_design::schematic::component_edit::legacy_terminal_points(component);
    label_terminals(state, terminals, origin, nets);
}

/// A run set of one transient to `stop`, sampled every 0.1 ns.
fn transient(state: &mut RunTestHost, stop: &str) {
    state.model.sim_setup.run_set =
        rspice_simulation_contract::run_set::RunSetState::reference_only();
    let plan = state
        .model
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("a plan");
    for id in plan
        .instances()
        .iter()
        .map(|instance| instance.id())
        .collect::<Vec<_>>()
    {
        plan.set_enabled(id, false).expect("disable");
    }
    let (id, _) = plan.insert(AnalysisKind::Transient).expect("a transient");
    plan.edit(id, |draft| {
        let AnalysisDraft::Transient(draft) = draft else {
            unreachable!()
        };
        draft.stop = stop.into();
        draft.step = "0.1n".into();
        draft.max_step = "0.1n".into();
    })
    .expect("the transient");
}

/// `blink` (a Verilog view) drives `level` (a Verilog-AMS view) on the
/// logic net `d`; `level` drives `out`, loaded by 1 kΩ. A 50 ns transient.
fn design() -> RunTestHost {
    let mut state = RunTestHost::default();
    let blink = add_source_cell(&mut state, "blink", ViewType::Verilog, "blink.v", BLINK);
    let level = add_source_cell(
        &mut state,
        "level",
        ViewType::VerilogAms,
        "level.vams",
        LEVEL,
    );
    place(&mut state, &blink, "XB", Point::new(300, 0), &[("q", "d")]);
    place(
        &mut state,
        &level,
        "XL",
        Point::new(300, 300),
        &[("d", "d"), ("out", "out")],
    );
    device(
        &mut state,
        ComponentType::Resistor,
        "R1",
        "1k",
        Point::new(700, 300),
        &["out", "0"],
    );
    device(
        &mut state,
        ComponentType::Ground,
        "GND",
        "",
        Point::new(900, 300),
        &["0"],
    );
    transient(&mut state, "50n");
    state.sync_active_schematic_to_workspace();
    state
}

/// `count` (a Verilog view) drives `moded` (a Verilog-AMS view) through the
/// bus `mode[1:0]` drawn between their bus pins; `moded` loads `p`, fed from
/// 1 V through 1 kΩ. A 25 ns transient.
fn bus_design() -> RunTestHost {
    let mut state = RunTestHost::default();
    let count = add_source_cell(&mut state, "count", ViewType::Verilog, "count.v", COUNT);
    let moded = add_source_cell(
        &mut state,
        "moded",
        ViewType::VerilogAms,
        "moded.vams",
        MODED,
    );
    let (driver_at, load_at) = (Point::new(300, 0), Point::new(300, 300));
    let mut pins = place(&mut state, &count, "XC", driver_at, &[]);
    pins.extend(place(
        &mut state,
        &moded,
        "XM",
        load_at,
        &[("p", "p"), ("n", "0")],
    ));
    let [(driver, from), (load, to)] = pins.as_slice() else {
        panic!("one bus pin on each: {pins:?}");
    };
    assert_eq!((driver.as_str(), load.as_str()), ("mode[1:0]", "mode[1:0]"));
    // Out from each pin, away from its body, and joined halfway between.
    let (from_out, to_out) = (outward(*from, driver_at, 40), outward(*to, load_at, 40));
    let middle = (from.y + to.y) / 2;
    let mut route = vec![
        *from,
        from_out,
        Point::new(from_out.x, middle),
        Point::new(to_out.x, middle),
        to_out,
        *to,
    ];
    route.dedup();
    state.model.schematic.document_mut_for_test().buses.push(
        rspice_design::schematic::bus::Bus::new(
            1_000,
            route,
            Some(
                rspice_design_model::bus::BusDeclaration::parse("mode[1:0]")
                    .expect("a declaration"),
            ),
        )
        .expect("the bus"),
    );
    device(
        &mut state,
        ComponentType::VoltageSource,
        "V1",
        "1",
        Point::new(700, 0),
        &["a", "0"],
    );
    device(
        &mut state,
        ComponentType::Resistor,
        "R1",
        "1k",
        Point::new(700, 300),
        &["a", "p"],
    );
    device(
        &mut state,
        ComponentType::Ground,
        "GND",
        "",
        Point::new(900, 300),
        &["0"],
    );
    transient(&mut state, "25n");
    // The bus's bits are no saved output of their own; every quantity is
    // kept, so the run's events on them are compared too.
    state.model.sim_setup.save_policy.output_selection_mode =
        crate::state::OutputSelectionMode::SaveAll;
    state.sync_active_schematic_to_workspace();
    state
}

/// The deck the design run executes, with each project source line put
/// back as the `.hdl` line of a file holding that module, and each
/// instance naming the module by the name its source declares. `modules`
/// names each module, its file and its text.
fn manual_twin(
    executable: &str,
    directory: &std::path::Path,
    modules: &[(&str, &str, &str)],
) -> String {
    let mut deck = String::with_capacity(executable.len());
    let mut aliases = Vec::new();
    for line in executable.lines() {
        let trimmed = line.trim_start();
        if trimmed.len() > 9 && trimmed[..9].eq_ignore_ascii_case(".veriloga") {
            let alias = trimmed
                .split_whitespace()
                .last()
                .expect("the line names its module")
                .to_owned();
            let (module, file, text) = modules
                .iter()
                .find(|(module, _, _)| alias.to_ascii_lowercase().starts_with(module))
                .unwrap_or_else(|| panic!("no module for {alias}"));
            std::fs::write(directory.join(file), text).expect("write the module");
            deck.push_str(&format!(".hdl \"{file}\"\n"));
            aliases.push((alias, *module));
            continue;
        }
        deck.push_str(line);
        deck.push('\n');
    }
    for (alias, module) in aliases {
        deck = deck
            .split_inclusive(char::is_whitespace)
            .map(|token| {
                let word = token.trim_end();
                if word.eq_ignore_ascii_case(&alias) {
                    format!("{module}{}", &token[word.len()..])
                } else {
                    token.to_owned()
                }
            })
            .collect();
    }
    deck
}

/// One trace a design run published, by its node: `V(OUT)` is `OUT`.
struct Trace {
    node: String,
    x: Vec<f64>,
    y: Vec<f64>,
}

impl Trace {
    /// The sample at the first grid point at or after `seconds`.
    fn at(&self, seconds: f64) -> f64 {
        let index = self
            .x
            .iter()
            .position(|t| *t >= seconds)
            .expect("inside the run");
        self.y[index]
    }
}

/// What a design run gave, once its deck twin agreed with it.
struct Twin {
    traces: Vec<Trace>,
    /// Each digital net's committed events, by its upper-case name.
    events: Vec<(
        String,
        Vec<rspice_results::events::DigitalEventPointEvidence>,
    )>,
    /// The buses the run declared over those nets.
    buses: Vec<rspice_results::events::DigitalBusEvidence>,
    deck: String,
}

/// Run `state` through the controller and the deck the design run executes,
/// rebuilt by [`manual_twin`], through the manual-deck route; assert both
/// runs commit the same events and publish the same traces, and hand back
/// what the design run gave and the twin deck.
fn run_against_twin(state: RunTestHost, modules: &[(&str, &str, &str)], stop: f64) -> Twin {
    let mut controller = SimulationController::new();
    controller
        .prepare_run_set_for_preflight(&state)
        .expect("the design prepares");
    let mut tasks = controller
        .consume_snapshot_for_dispatch(&state)
        .expect("the design dispatches")
        .into_tasks();
    assert_eq!(tasks.len(), 1);
    let executable = tasks
        .pop_front()
        .expect("one task")
        .executable_netlist()
        .to_owned();
    assert_eq!(
        executable
            .lines()
            .filter(|line| line
                .trim_start()
                .to_ascii_lowercase()
                .starts_with(".veriloga"))
            .count(),
        modules.len(),
        "each view binds its own sealed source:\n{executable}"
    );

    let designed = run_generated_batch(state, SimulationRunLifecycle::Completed);

    let directory = std::env::temp_dir().join(format!(
        "rspice-hdl-view-twin-{}-{}",
        std::process::id(),
        modules[0].0
    ));
    std::fs::create_dir_all(&directory).expect("a scratch directory");
    let deck = manual_twin(&executable, &directory, modules);
    let path = directory.join("twin.cir");
    std::fs::write(&path, &deck).expect("write the deck");
    let manual = run_manual_batch_at(&deck, &path, SimulationRunLifecycle::Completed);
    let _ = std::fs::remove_dir_all(&directory);

    // Digital nets are the event kernel's: both runs commit the same events.
    let events = |run: &crate::state::SimulationRun| match run.analyses[0].result_payload.as_ref() {
        Some(AnalysisResultPayload::TransientEvents { digital_traces, .. }) => digital_traces
            .iter()
            .map(|trace| (trace.node_name.to_ascii_uppercase(), trace.points.clone()))
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };
    let committed = events(&designed);
    assert_eq!(committed, events(&manual), "the same committed events");
    let buses = |run: &crate::state::SimulationRun| match run.analyses[0].result_payload.as_ref() {
        Some(AnalysisResultPayload::TransientEvents { digital_buses, .. }) => digital_buses.clone(),
        _ => Vec::new(),
    };
    let declared = buses(&designed);
    assert_eq!(declared, buses(&manual), "the same buses");

    // A design run names a node's trace `V(OUT)` and samples it on the
    // transient's output grid; a manual deck's run names the node itself and
    // keeps every solver point. The same node sampled on the same grid is
    // the same trace.
    fn traces(run: &crate::state::SimulationRun) -> Vec<(String, &RetainedTrace)> {
        run.analyses[0]
            .waveforms
            .iter()
            .map(|wave| {
                let name = wave.name.to_ascii_uppercase();
                let node = name
                    .strip_prefix("V(")
                    .and_then(|inner| inner.strip_suffix(')'))
                    .map_or_else(|| name.clone(), str::to_owned);
                (node, wave)
            })
            .collect()
    }
    let grid = TransientSelectionGrid {
        start: 0.0,
        step: 0.1e-9,
        stop,
    };
    let manual = traces(&manual);
    let designed = traces(&designed)
        .into_iter()
        .map(|(node, wave)| {
            let (_, manual_wave) = manual
                .iter()
                .find(|(manual_node, _)| *manual_node == node)
                .unwrap_or_else(|| panic!("the deck run publishes {node}:\n{deck}"));
            // A run that kept every solver point is compared point for point.
            let sampled = if wave.x == manual_wave.x {
                (*manual_wave).clone()
            } else {
                resample_selected_and_final(*manual_wave, grid).expect("resample")
            };
            assert_eq!(wave.x, sampled.x, "{node}: the same time grid");
            assert_eq!(wave.y, sampled.y, "{node}: the same samples");
            Trace {
                node,
                x: wave.x.to_vec(),
                y: wave.y.to_vec(),
            }
        })
        .collect();
    Twin {
        traces: designed,
        events: committed,
        buses: declared,
        deck,
    }
}

/// A purely digital Verilog module and a Verilog-AMS module, placed from
/// their project views, run on the digital event kernel and the analog
/// solver exactly as the same modules do when a manual deck loads them with
/// `.hdl`: the same events on the logic net, and every published trace
/// equal, sample for sample.
#[test]
fn verilog_and_verilog_ams_views_run_as_their_hdl_deck_twin() {
    let twin = run_against_twin(
        design(),
        &[("blink", "blink.v", BLINK), ("level", "level.vams", LEVEL)],
        50e-9,
    );
    // `blink`'s register toggles on the event kernel: ten edges in 50 ns.
    let edges = twin
        .events
        .iter()
        .find(|(node, _)| node == "D")
        .map_or(0, |(_, points)| points.len());
    assert!(edges > 5, "{:?}", twin.events);
    let out = twin
        .traces
        .iter()
        .find(|trace| trace.node == "OUT")
        .expect("V(OUT)");
    // `blink` is low for its first 5 ns, then high for the next 5 ns.
    assert!(out.at(4e-9).abs() < 1e-6, "{}", out.at(4e-9));
    assert!((out.at(9e-9) - 1.0).abs() < 1e-6, "{}", out.at(9e-9));
}

/// A two-bit discrete port is a bus pin whose two conductors are the nets
/// the engine binds, most significant first: a counter's `mode[1:0]` drawn
/// to a conductance's `mode[1:0]` runs bit for bit as the deck that names
/// `mode#1` and `mode#0` on both instance cards, and the conductance reads
/// each bit at its own weight. Each bit publishes its `D()` history on both
/// routes, and the two ports meeting on those bits declare them as one bus.
#[test]
fn a_bus_between_discrete_vector_pins_runs_as_its_deck_twin() {
    let Twin {
        traces,
        events,
        buses,
        deck,
    } = run_against_twin(
        bus_design(),
        &[("count", "count.v", COUNT), ("moded", "moded.vams", MODED)],
        25e-9,
    );
    let cards = deck
        .lines()
        .filter(|line| line.starts_with("XC ") || line.starts_with("XM "))
        .collect::<Vec<_>>();
    assert_eq!(
        cards,
        ["XC mode#1 mode#0 count", "XM p 0 mode#1 mode#0 moded"],
        "{deck}"
    );
    let p = traces.iter().find(|trace| trace.node == "P").expect("V(P)");
    // V(p) = 1 / (1 + 1 kΩ · G): mode 0 is 1 mS, 1 is 2 mS, 2 is 3 mS, 3 is
    // 4 mS. A swapped bit order would read 3 mS at mode 1.
    for (seconds, mode) in [(3e-9, 0.0), (8e-9, 1.0), (13e-9, 2.0), (18e-9, 3.0)] {
        let expected = 1.0 / (2.0 + mode);
        assert!(
            (p.at(seconds) - expected).abs() < 1e-6,
            "at {seconds}: {} against {expected}",
            p.at(seconds)
        );
    }
    // The counter's bits as its two event nets commit them: bit 0 toggles
    // every 5 ns and bit 1 every 10 ns, from 0.
    let bit = |name: &str| {
        events
            .iter()
            .find(|(node, _)| node == name)
            .unwrap_or_else(|| panic!("no D({name}) in {events:?}"))
            .1
            .iter()
            .map(|point| (point.time_s, point.value_code))
            .collect::<Vec<_>>()
    };
    for (name, expected) in [
        (
            "MODE#0",
            &[
                (0.0, 0),
                (5e-9, 1),
                (10e-9, 0),
                (15e-9, 1),
                (20e-9, 0),
                (25e-9, 1),
            ][..],
        ),
        ("MODE#1", &[(0.0, 0), (10e-9, 1), (20e-9, 0)][..]),
    ] {
        let history = bit(name);
        assert_eq!(history.len(), expected.len(), "{name}: {history:?}");
        for ((time, code), (want_time, want_code)) in history.iter().zip(expected) {
            assert!((time - want_time).abs() < 1e-15, "{name}: {history:?}");
            assert_eq!(code, want_code, "{name}: {history:?}");
        }
    }
    assert_eq!(
        buses
            .iter()
            .map(|bus| (bus.msb, bus.lsb, bus.members.clone()))
            .collect::<Vec<_>>(),
        [(1, 0, vec!["MODE#1".to_owned(), "MODE#0".to_owned()])],
        "one bus over both bits, most significant first"
    );
}

/// Each net's kind as the rail and the net inspector read it: from the
/// configured design, before anything has run.
fn net_kinds(state: &RunTestHost) -> Vec<(String, &'static str)> {
    let model = &state.model;
    let projection = model
        .workspace
        .design_projection(
            &model.library_manager,
            &model.workspace.content.active_view,
            &model.schematic,
        )
        .expect("the design resolves");
    rspice_design::connectivity::summary::projection_nets(
        model.library_manager.catalog(),
        &projection,
        &model.workspace.content.active_view.key(),
    )
    .iter()
    .map(|net| (net.name.to_ascii_lowercase(), net.domain.label()))
    .collect()
}

/// A net's kind is known before a run, by the rule the engine plans its
/// converters with: two module ports meeting with nothing else are a logic
/// net, a bus between two discrete vector pins is a logic net per bit, and an
/// electrical terminal on a logic net makes it a net the engine converts.
#[test]
fn a_net_reads_its_kind_before_a_run() {
    let mut state = design();
    let kinds = net_kinds(&state);
    let kind = |kinds: &[(String, &'static str)], name: &str| {
        kinds
            .iter()
            .find(|(net, _)| net == name)
            .map(|(_, kind)| *kind)
            .unwrap_or_else(|| panic!("no net {name} in {kinds:?}"))
    };
    assert_eq!(kind(&kinds, "d"), "logic", "{kinds:?}");
    assert_eq!(kind(&kinds, "out"), "electrical", "{kinds:?}");

    device(
        &mut state,
        ComponentType::Resistor,
        "R2",
        "1k",
        Point::new(700, 0),
        &["d", "0"],
    );
    state.sync_active_schematic_to_workspace();
    let kinds = net_kinds(&state);
    assert_eq!(kind(&kinds, "d"), "mixed (converter at run)", "{kinds:?}");

    let kinds = net_kinds(&bus_design());
    assert_eq!(kind(&kinds, "mode#1"), "logic", "{kinds:?}");
    assert_eq!(kind(&kinds, "mode#0"), "logic", "{kinds:?}");
    assert_eq!(kind(&kinds, "p"), "electrical", "{kinds:?}");
}
