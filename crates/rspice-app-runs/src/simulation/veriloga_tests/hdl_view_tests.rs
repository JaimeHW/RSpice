//! A Verilog and a Verilog-AMS view placed on a schematic run as the same
//! modules do in a manual deck that loads them with `.hdl`.
//!
//! The design route binds each view's project source with the sealed project
//! directive; the deck route seals each `.hdl` file. Both reach the engine as
//! one compiled runtime per module, so a run of each through the real
//! controller publishes the same traces.

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
        &receipt.compilation.report().abi,
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

/// Wire each of `terminals` out to a stub carrying the net label `nets`.
fn label_terminals(state: &mut RunTestHost, terminals: Vec<Point>, origin: Point, nets: &[&str]) {
    assert_eq!(terminals.len(), nets.len());
    for (at, net) in terminals.into_iter().zip(nets) {
        let outward = if (at.x - origin.x).abs() >= (at.y - origin.y).abs() {
            Point::new(at.x + (at.x - origin.x).signum() * 20, at.y)
        } else {
            Point::new(at.x, at.y + (at.y - origin.y).signum() * 20)
        };
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

/// `blink` (a Verilog view) drives `level` (a Verilog-AMS view) on the
/// logic net `d`; `level` drives `out`, loaded by 1 kΩ. A 50 ns transient.
fn design() -> RunTestHost {
    let mut state = RunTestHost::default();
    state.model.sim_setup.run_set =
        rspice_simulation_contract::run_set::RunSetState::reference_only();
    let blink = add_source_cell(&mut state, "blink", ViewType::Verilog, "blink.v", BLINK);
    let level = add_source_cell(
        &mut state,
        "level",
        ViewType::VerilogAms,
        "level.vams",
        LEVEL,
    );
    for (reference, name, origin, nets) in [
        (&blink, "XB", Point::new(300, 0), &["d"][..]),
        (&level, "XL", Point::new(300, 300), &["d", "out"][..]),
    ] {
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
        let terminals = component
            .terminal_positions()
            .into_iter()
            .map(|(_, at)| at)
            .collect();
        label_terminals(&mut state, terminals, origin, nets);
    }
    for (kind, name, value, origin, nets) in [
        (
            ComponentType::Resistor,
            "R1",
            "1k",
            Point::new(700, 300),
            &["out", "0"][..],
        ),
        (
            ComponentType::Ground,
            "GND",
            "",
            Point::new(900, 300),
            &["0"][..],
        ),
    ] {
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
        label_terminals(&mut state, terminals, origin, nets);
    }
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
        draft.stop = "50n".into();
        draft.step = "0.1n".into();
        draft.max_step = "0.1n".into();
    })
    .expect("the transient");
    state.sync_active_schematic_to_workspace();
    state
}

/// The deck the design run executes, with each project source line put
/// back as the `.hdl` line of a file holding that module, and each
/// instance naming the module by the name its source declares.
fn manual_twin(executable: &str, directory: &std::path::Path) -> String {
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
            let (module, file, text) = if alias.to_ascii_lowercase().starts_with("blink") {
                ("blink", "blink.v", BLINK)
            } else {
                ("level", "level.vams", LEVEL)
            };
            std::fs::write(directory.join(file), text).expect("write the module");
            deck.push_str(&format!(".hdl \"{file}\"\n"));
            aliases.push((alias, module));
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

/// A purely digital Verilog module and a Verilog-AMS module, placed from
/// their project views, run on the digital event kernel and the analog
/// solver exactly as the same modules do when a manual deck loads them with
/// `.hdl`: every published trace is equal, sample for sample.
#[test]
fn verilog_and_verilog_ams_views_run_as_their_hdl_deck_twin() {
    let state = design();
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
        2,
        "each view binds its own sealed source:\n{executable}"
    );

    let designed = run_generated_batch(state, SimulationRunLifecycle::Completed);

    let directory =
        std::env::temp_dir().join(format!("rspice-hdl-view-twin-{}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("a scratch directory");
    let deck = manual_twin(&executable, &directory);
    let path = directory.join("twin.cir");
    std::fs::write(&path, &deck).expect("write the deck");
    let manual = run_manual_batch_at(&deck, &path, SimulationRunLifecycle::Completed);
    let _ = std::fs::remove_dir_all(&directory);

    // The digital net `d` is the event kernel's: both runs commit the same
    // events on it, with no converter between `blink` and `level`.
    let events = |run: &crate::state::SimulationRun| {
        let Some(AnalysisResultPayload::TransientEvents { digital_traces, .. }) =
            run.analyses[0].result_payload.as_ref()
        else {
            panic!("a transient event history");
        };
        digital_traces
            .iter()
            .map(|trace| (trace.node_name.to_ascii_uppercase(), trace.points.clone()))
            .collect::<Vec<_>>()
    };
    let designed_events = events(&designed);
    assert!(
        designed_events
            .iter()
            .any(|(name, points)| name == "D" && points.len() > 5),
        "{designed_events:?}"
    );
    assert_eq!(
        designed_events,
        events(&manual),
        "the same committed events"
    );

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
        stop: 50e-9,
    };
    let (designed, manual) = (traces(&designed), traces(&manual));
    assert!(
        designed.iter().any(|(name, _)| name == "OUT"),
        "{:?}",
        designed.iter().map(|trace| &trace.0).collect::<Vec<_>>()
    );
    for (name, wave) in &designed {
        let (_, manual_wave) = manual
            .iter()
            .find(|(manual_name, _)| manual_name == name)
            .unwrap_or_else(|| panic!("the deck run publishes {name}:\n{deck}"));
        let sampled = resample_selected_and_final(*manual_wave, grid).expect("resample");
        assert_eq!(wave.x, sampled.x, "{name}: the same time grid");
        assert_eq!(wave.y, sampled.y, "{name}: the same samples");
    }
    let designed = designed
        .into_iter()
        .map(|(name, wave)| (name, wave.x.clone(), wave.y.clone()))
        .collect::<Vec<_>>();
    let (_, time, out) = designed
        .iter()
        .find(|(name, _, _)| name == "OUT")
        .expect("V(OUT)");
    let at = |seconds: f64| {
        let index = time
            .iter()
            .position(|t| *t >= seconds)
            .expect("inside the run");
        out[index]
    };
    // `blink` is low for its first 5 ns, then high for the next 5 ns.
    assert!(at(4e-9).abs() < 1e-6, "{}", at(4e-9));
    assert!((at(9e-9) - 1.0).abs() < 1e-6, "{}", at(9e-9));
}
