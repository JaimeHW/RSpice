//! The engine pass: what it costs, and what it refuses where.

use super::*;

use rspice_simulation_contract::run_set::RunSetDimensionKind;

/// A model library whose five process sections each bin `families` NMOS and
/// PMOS families over a 6 x 6 grid of lengths and widths from 0.15 µm and
/// 0.2 µm. The section and family `short_gap` names bins nothing below
/// 0.25 µm.
fn binned_library(families: usize, level: u32, short_gap: Option<(&str, &str)>) -> Vec<u8> {
    const LENGTHS: [f64; 7] = [0.15e-6, 0.25e-6, 0.5e-6, 1e-6, 2e-6, 5e-6, 20e-6];
    const WIDTHS: [f64; 7] = [0.2e-6, 0.5e-6, 1e-6, 2e-6, 5e-6, 10e-6, 100e-6];
    let mut text = String::new();
    for (process, shift) in [
        ("TT", 0.0),
        ("SS", 0.05),
        ("FF", -0.05),
        ("SF", 0.02),
        ("FS", -0.02),
    ] {
        text.push_str(&format!(".lib {process}\n"));
        for family in 0..families {
            for (polarity, stem, threshold) in [("nmos", "nch", 0.42), ("pmos", "pch", -0.42)] {
                let name = if family == 0 {
                    stem.to_owned()
                } else {
                    format!("{stem}_{family}")
                };
                let threshold = threshold + shift;
                let shortest = usize::from(short_gap == Some((process, name.as_str())));
                let mut bin = 0;
                for length in LENGTHS[shortest..].windows(2) {
                    for width in WIDTHS.windows(2) {
                        bin += 1;
                        let parameters = if level == 54 {
                            format!(
                                "version=4.5 toxe=4.1e-9 toxp=3.5e-9 toxm=4.1e-9 epsrox=3.9 \
                                 xj=1.5e-7 ndep=1.7e17 vth0={threshold} k1=0.5 k2=0.01 k3=0 \
                                 dvt0=1 dvt1=2 dvt2=-0.032 u0=0.03 ua=-6e-10 ub=1.6e-18 \
                                 uc=-4.6e-11 vsat=1e5 a0=1 ags=0.2 rdsw=150 prwg=0 prwb=0 \
                                 wr=1 voff=-0.1 nfactor=1 cit=0 cdsc=2.4e-4 eta0=0.08 \
                                 etab=-0.07 dsub=0.56 pclm=1.3 pdiblc1=0.39 pdiblc2=0.0086 \
                                 drout=0.56 pvag=0 delta=0.01 cgso=2.5e-10 cgdo=2.5e-10 \
                                 cj=1e-3 mj=0.5 cjsw=2e-10 mjsw=0.33 pb=1 pbsw=1"
                            )
                        } else {
                            format!("vto={threshold} kp=1e-4 lambda=0.02")
                        };
                        text.push_str(&format!(
                            ".model {name}.{bin} {polarity} level={level} lmin={} lmax={} \
                             wmin={} wmax={} {parameters}\n",
                            length[0], length[1], width[0], width[1]
                        ));
                    }
                }
            }
        }
        text.push_str(&format!(".endl {process}\n"));
    }
    text.into_bytes()
}

/// Leave enabled only the Run Set axes `kinds` names.
fn enable_only(state: &mut AppState, kinds: &[RunSetDimensionKind]) {
    for dimension in &mut state.sim_setup.run_set.dimensions {
        dimension.enabled = kinds.contains(&dimension.kind);
    }
}

fn add_operating_point(state: &mut AppState) {
    use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
    let plan = state
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("a default state owns a stable plan");
    let position = plan.instances().len();
    plan.insert_draft_with_id(
        crate::product::AnalysisInstanceId::new(),
        AnalysisDraft::for_kind(AnalysisKind::OperatingPoint),
        true,
        position,
    )
    .expect("an operating point has no prerequisites");
}

/// One example, run and then timed: the preparation that builds the decks and
/// the engine pass over them, each the median of `repeats`.
fn time_pass(label: &str, mut app: RSpiceApp, repeats: usize) {
    run(&mut app);
    app.settle_preflight();
    let report = app.state.workbench.preflight.report.as_ref().unwrap();
    let points = report.prepared.as_ref().map_or(0, |p| p.pvt_point_count);
    let tasks = report.prepared.as_ref().map_or(0, |p| p.task_count);
    if !report.blockers.is_empty() {
        eprintln!("MEASURE {label}: blocked {:?}", report.blockers[0].observed);
        return;
    }
    let deck_bytes = app
        .simulation_controller
        .retained_run_deck()
        .map_or(0, |(deck, _)| deck.len());
    let mut prepare = Vec::new();
    let mut engine = Vec::new();
    let mut refused = 0;
    for _ in 0..repeats {
        let started = std::time::Instant::now();
        app.simulation_controller
            .prepare_run_set_for_preflight(&app.state)
            .expect("prepares again");
        prepare.push(started.elapsed());
        let started = std::time::Instant::now();
        assert!(app.simulation_controller.start_engine_check());
        refused = loop {
            if let Some(outcome) = app.simulation_controller.poll_engine_check() {
                break outcome.refusals.map_or(usize::MAX, |refusals| refusals.len());
            }
            std::thread::sleep(std::time::Duration::from_micros(100));
        };
        engine.push(started.elapsed());
    }
    prepare.sort();
    engine.sort();
    eprintln!(
        "MEASURE {label}: points={points} tasks={tasks} deck={deck_bytes}B refusals={refused} \
         prepare={:?} engine={:?} (min {:?} max {:?})",
        prepare[repeats / 2],
        engine[repeats / 2],
        engine[0],
        engine[repeats - 1],
    );
}

fn example_app(name: &str) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    enable_only(&mut app.state, &[]);
    crate::workbench::examples::load_example(name, &mut app.state.schematic);
    add_operating_point(&mut app.state);
    app
}

fn inverter_on_binned_pdk(models: Vec<u8>) -> RSpiceApp {
    let mut app = example_app("CMOS Inverter");
    app.state
        .provision_test_project_technology_with_models(models);
    for component in &mut app.state.schematic.document_mut_for_test().components {
        match component.kind {
            crate::state::ComponentType::Nmos => component.value = "nch".to_owned(),
            crate::state::ComponentType::Pmos => component.value = "pch".to_owned(),
            _ => {}
        }
    }
    enable_only(
        &mut app.state,
        &[
            RunSetDimensionKind::ProcessSection,
            RunSetDimensionKind::Temperature,
        ],
    );
    app
}

#[test]
#[ignore = "measurement"]
fn measure_engine_pass() {
    for name in [
        "RC Lowpass Filter",
        "Voltage Divider",
        "Common Emitter Amplifier",
        "CMOS Inverter",
        "Differential Pair",
        "Opamp Inverting Amplifier",
    ] {
        time_pass(name, example_app(name), 5);
    }
    {
        let mut app = RSpiceApp::test_instance();
        app.state = crate::workbench::examples::hierarchy_reference::build().state;
        app.state.project_lifecycle.authority.open_session();
        enable_only(&mut app.state, &[]);
        add_operating_point(&mut app.state);
        time_pass("hierarchy reference", app, 5);
    }
    time_pass(
        "inverter, 3x3, 1 family L1",
        inverter_on_binned_pdk(binned_library(1, 1, None)),
        5,
    );
    time_pass(
        "inverter, 3x3, 1 family BSIM4",
        inverter_on_binned_pdk(binned_library(1, 54, None)),
        5,
    );
    time_pass(
        "inverter, 3x3, 14 families BSIM4",
        inverter_on_binned_pdk(binned_library(14, 54, None)),
        3,
    );
}

/// The inverter on a PDK whose FF section bins its NMOS nothing shorter than
/// 0.25 µm, run at SS, TT and FF: the 0.18 µm pull-down falls in no bin at FF
/// and in one at the other two. The engine refuses the FF point's deck alone,
/// and the finding is shown on the transistor as drawn.
#[test]
fn a_transistor_outside_its_bins_at_one_corner_is_refused_there_on_the_drawn_device() {
    let mut app = inverter_on_binned_pdk(binned_library(1, 1, Some(("FF", "nch"))));
    enable_only(&mut app.state, &[RunSetDimensionKind::ProcessSection]);
    let root = app.state.workspace.content.simulation_root_reference();
    let pull_down = app
        .state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.name == "MN")
        .expect("the inverter draws MN")
        .id;

    run(&mut app);
    app.settle_preflight();

    let blockers = preflight_blockers(&app);
    let uncovered: Vec<&PreflightIssue> = blockers
        .iter()
        .filter(|issue| issue.check == "Model-bin coverage")
        .collect();
    assert_eq!(uncovered.len(), 1, "{blockers:#?}");
    let issue = uncovered[0];
    assert!(
        issue.observed.contains("'MN'") && issue.observed.contains("L=0.18 µm"),
        "{issue:?}"
    );
    let refused_by = issue
        .observed
        .split("Refused by")
        .nth(1)
        .expect("the finding names what it refuses");
    assert!(
        refused_by.contains("FF") && !refused_by.contains("TT") && !refused_by.contains("SS"),
        "only the FF point is refused: {issue:?}"
    );
    assert_eq!(
        issue.remediation,
        PreflightRemediation::Drawing {
            cell_view: root.clone(),
            object: crate::workbench::state::DrawnObject::Component {
                id: pull_down,
                name: "MN".to_owned(),
            },
        }
    );

    app.state.workbench.activate(Workspace::Verify);
    apply_remediation(&mut app, issue.remediation.clone());
    assert_eq!(
        app.state.workspace.content.active_schematic_reference(),
        root
    );
    assert!(
        app.state
            .schematic
            .session
            .editor
            .selection
            .has_component(pull_down),
        "following the finding selects MN"
    );
}

/// A Verilog-A device whose discrete input `mode` is driven by a voltage
/// source, in a design whose one connect-rule block bridges only the other
/// way. The crossing is the builder's to bridge, and it cannot: the finding is
/// its refusal, shown on the device whose port the crossing is.
#[test]
fn a_crossing_the_connect_rules_cannot_bridge_is_shown_on_the_device() {
    use crate::state::{
        Cell, CellViewRef, ComponentType, Library, LibraryCellInstance, Point,
        ProjectSourceBundle, ProjectSourceLanguage, ProjectSourceOwner, View, ViewType,
    };

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    enable_only(&mut app.state, &[]);

    // The design's connect rules: the shipped connect modules, and one block
    // that converts discrete to analog and nothing else.
    let mut rules = rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES
        .iter()
        .map(|(_, source)| *source)
        .collect::<String>();
    rules.push_str("\nconnectrules only_d2a; connect d2a; endconnectrules\n");
    let library = app
        .state
        .model_library_manager
        .load_library_bundle(
            "connections.lib",
            vec![
                (
                    "root.lib".to_owned(),
                    b".va \"rules.vams\" CONNECTIONS\n".to_vec(),
                ),
                ("rules.vams".to_owned(), rules.into_bytes()),
            ],
            None,
        )
        .expect("the connect rules import");
    let binding = app
        .state
        .model_library_manager
        .simulation_plan_binding(&library)
        .expect("the rules bind to the plan");
    app.state.sim_setup.model_bindings.push(binding);

    // A cell whose Verilog-A view reads a discrete input.
    let device = CellViewRef::new("behavioral", "moded", "veriloga");
    app.state
        .library_manager
        .add_library(Library::new(&device.library));
    let mut view = View::new("veriloga", ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), "moded".to_owned());
    view.metadata.insert(
        "veriloga.ports".to_owned(),
        r#"[{"name":"p","direction":"inout","discipline":"electrical","domain":"continuous"},{"name":"n","direction":"inout","discipline":"electrical","domain":"continuous"},{"name":"mode","direction":"in","discipline":"logic","domain":"discrete"}]"#
            .to_owned(),
    );
    let mut cell = Cell::new(&device.cell);
    cell.add_view(view);
    app.state
        .library_manager
        .get_library_mut(&device.library)
        .expect("the device library")
        .add_cell(cell);
    let bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::cell_view(device.clone()),
        ProjectSourceLanguage::VerilogA,
        "moded.va".to_owned(),
        "`include \"disciplines.vams\"\nmodule moded(p, n, mode);\n  inout p, n;\n  input mode;\n  \
         electrical p, n;\n  logic mode;\n  analog I(p, n) <+ (mode ? 1m : 2m) * V(p, n);\nendmodule\n",
        [],
        [],
    )
    .expect("a cell-view bundle");
    app.state
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle)
        .expect("one bundle per cell view");

    // The testbench: `mode` on a driven net, the analog pins on ground.
    let top = CellViewRef::default_top();
    app.state.open_workspace_view(top.clone());
    let ports = rspice_design::library::metadata::metadata_ports(
        &app.state
            .library_manager
            .get_library(&device.library)
            .and_then(|library| library.get_cell(&device.cell))
            .and_then(|cell| cell.get_view(&device.view))
            .expect("the device view")
            .metadata,
    )
    .expect("the view states its ports");
    let schematic = &mut app.state.schematic;
    let origin = Point::new(300, 0);
    let mut instance = LibraryCellInstance::new(&device.library, &device.cell, &device.view);
    instance.bind_interface(&ports);
    let placed = schematic.add_library_cell_component(origin, instance);
    let component = schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == placed)
        .expect("the device is placed");
    component.name = "XS".to_owned();
    let terminals: Vec<Point> = component
        .terminal_positions()
        .into_iter()
        .map(|(_, at)| at)
        .collect();
    assert_eq!(terminals.len(), 3, "p, n and mode");
    for (at, net) in terminals.into_iter().zip(["gnd", "gnd", "drv"]) {
        let outward = if (at.x - origin.x).abs() >= (at.y - origin.y).abs() {
            Point::new(at.x + (at.x - origin.x).signum() * 20, at.y)
        } else {
            Point::new(at.x, at.y + (at.y - origin.y).signum() * 20)
        };
        schematic
            .add_wire(vec![at, outward])
            .expect("the stub commits");
        schematic.add_net_label(outward, net.to_owned());
    }
    // + at (0, -10) onto `drv`, − at (0, 30) onto the ground at (0, 40).
    schematic.add_component(ComponentType::VoltageSource, Point::new(0, 10));
    schematic.add_component(ComponentType::Ground, Point::new(0, 40));
    schematic
        .add_wire(vec![Point::new(0, -10), Point::new(0, -30)])
        .expect("the drive commits");
    schematic.add_net_label(Point::new(0, -30), "drv".to_owned());
    schematic.add_component(ComponentType::Ground, Point::new(100, 70));
    schematic
        .add_wire(vec![Point::new(100, 60), Point::new(100, 40)])
        .expect("the return commits");
    schematic.add_net_label(Point::new(100, 40), "gnd".to_owned());
    app.state.sync_active_schematic_to_workspace();
    // The configuration that executes Verilog-A views, as the library shelf
    // places a device under one.
    app.state
        .workspace
        .content
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Behavioral".to_owned(),
            root: top.clone(),
            dut_path: "/XS".to_owned(),
            executable_view_policy: vec!["veriloga".to_owned()],
            stop_views: vec!["veriloga".to_owned()],
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Behavioral design".to_owned(),
        })
        .expect("the executable configuration");

    run(&mut app);
    app.settle_preflight();

    let blockers = preflight_blockers(&app);
    let crossing = blockers
        .iter()
        .find(|issue| issue.check == "Engine elaboration")
        .unwrap_or_else(|| panic!("the builder refuses the crossing: {blockers:#?}"));
    assert!(
        crossing.observed.contains("connect rules do not settle it"),
        "{crossing:?}"
    );
    let remediation = crossing.remediation.clone();
    let PreflightRemediation::Drawing { cell_view, object } = &remediation else {
        panic!("the finding is located on the drawing: {crossing:?}");
    };
    assert_eq!(cell_view, &top);
    let crate::workbench::state::DrawnObject::Component { id, .. } = object else {
        panic!("the finding names the device: {crossing:?}");
    };
    assert_eq!(*id, placed);

    app.state.workbench.activate(Workspace::Verify);
    apply_remediation(&mut app, remediation);
    assert_eq!(app.state.workspace.content.active_schematic_reference(), top);
    assert!(
        app.state.schematic.session.editor.selection.has_component(placed),
        "following the finding selects the device"
    );
}

/// A cell `amp` behind one `VSS` port, drawn by `draw` with its return at
/// (0, 30), placed once as `XAMP` in the configured root with `VSS` on
/// ground. The root is left open, with an operating point in its plan.
fn app_with_an_amp_instance(
    draw: impl FnOnce(&mut crate::state::SchematicState),
) -> (RSpiceApp, crate::state::CellViewRef) {
    use crate::state::{
        Cell, CellViewRef, ComponentType, LibraryCellInstance, PendingPortPlacement, Point,
        PortDirectionType, PortDiscipline, View, ViewType,
    };

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    enable_only(&mut app.state, &[]);
    add_operating_point(&mut app.state);
    let root = app.state.workspace.content.simulation_root_reference();
    let amp = CellViewRef::new(&root.library, "amp", "schematic");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    app.state
        .library_manager
        .edit_library(&root.library)
        .expect("the root's library is the project's")
        .add_cell(cell);

    app.state.open_workspace_view(amp.clone());
    let schematic = &mut app.state.schematic;
    let pending = PendingPortPlacement::new(
        "VSS",
        PortDirectionType::InOutPower,
        PortDiscipline::Electrical,
        schematic.topology_version(),
        schematic.next_interface_order(),
    );
    let port = schematic
        .place_pending_port(Point::new(0, 100), pending)
        .expect("VSS places");
    let vss = terminal(schematic, port, 0);
    draw(schematic);
    let mut run = vec![Point::new(0, 30), Point::new(0, vss.y)];
    if vss.x != 0 {
        run.push(vss);
    }
    schematic.add_wire(run).expect("the return reaches VSS");
    app.state.sync_active_schematic_to_workspace();
    let ports = app
        .state
        .workspace
        .content
        .schematic_buffers
        .get(&amp.key())
        .expect("amp is saved to the workspace")
        .document()
        .interface_ports();

    app.state.open_workspace_view(root);
    let schematic = &mut app.state.schematic;
    let mut binding = LibraryCellInstance::new(&amp.library, &amp.cell, &amp.view);
    binding.bind_interface(&ports);
    let instance = schematic.add_library_cell_component(Point::new(300, 0), binding);
    schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == instance)
        .expect("XAMP is placed")
        .name = "XAMP".to_owned();
    let pin = terminal(schematic, instance, 0);
    let foot = Point::new(pin.x, pin.y + 40);
    schematic
        .add_wire(vec![pin, foot])
        .expect("VSS's stub commits");
    schematic.add_component(ComponentType::Ground, Point::new(foot.x, foot.y + 10));
    app.state.sync_active_schematic_to_workspace();
    (app, amp)
}

/// Where a placed component's `index`th terminal is.
fn terminal(
    schematic: &crate::state::SchematicState,
    component: u64,
    index: usize,
) -> crate::state::Point {
    schematic
        .document()
        .components
        .iter()
        .find(|candidate| candidate.id == component)
        .expect("the component is placed")
        .terminal_positions()[index]
        .1
}

/// A forced node inside a placed cell is refused on the net as `amp` draws
/// it, and following the finding opens `amp` with the net's wire selected —
/// not the root, which draws no such net.
#[test]
fn a_forced_node_inside_an_instance_is_shown_in_the_cell_that_draws_it() {
    use crate::state::{ComponentType, Point};

    let (mut app, amp) = app_with_an_amp_instance(|schematic| {
        // + at (0, -10) onto `isl`, − at (0, 30) onto the return.
        schematic.add_component(ComponentType::CurrentSource, Point::new(0, 10));
        schematic.add_component(ComponentType::Capacitor, Point::new(60, -10));
        for (from, to) in [
            (Point::new(0, -10), Point::new(40, -10)),
            (Point::new(80, -10), Point::new(80, 30)),
            (Point::new(80, 30), Point::new(0, 30)),
        ] {
            schematic.add_wire(vec![from, to]).expect("the wire commits");
        }
        schematic.add_net_label(Point::new(0, -10), "isl".to_owned());
    });

    run(&mut app);
    app.settle_preflight();

    let blockers = preflight_blockers(&app);
    let island = blockers
        .iter()
        .find(|issue| issue.check == "Nodes without a DC path")
        .unwrap_or_else(|| panic!("the operating point refuses isl: {blockers:#?}"));
    assert!(island.observed.contains("'isl' in /XAMP"), "{island:?}");
    let remediation = island.remediation.clone();
    let PreflightRemediation::Drawing { cell_view, object } = &remediation else {
        panic!("the finding is located on the drawing: {island:?}");
    };
    assert_eq!(cell_view, &amp, "shown in the cell that draws it");
    let crate::workbench::state::DrawnObject::Net { name, wire, .. } = object else {
        panic!("the finding names a net: {island:?}");
    };
    assert_eq!(name, "isl");
    let wire = wire.expect("isl is drawn on a wire");

    app.state.workbench.activate(Workspace::Verify);
    apply_remediation(&mut app, remediation);
    assert_eq!(app.state.workspace.content.active_schematic_reference(), amp);
    assert!(
        app.state.schematic.session.editor.selection.has_wire(wire),
        "following the finding selects the wire that draws isl in amp"
    );
}

/// A source across a winding inside a placed cell is refused on the winding
/// `amp` draws, and following the finding opens `amp` with it selected.
#[test]
fn a_winding_loop_inside_an_instance_is_shown_in_the_cell_that_draws_it() {
    use crate::state::{ComponentType, Point};

    let mut winding = 0;
    let (mut app, amp) = app_with_an_amp_instance(|schematic| {
        schematic.add_component(ComponentType::VoltageSource, Point::new(0, 10));
        winding = schematic.add_component(ComponentType::Inductor, Point::new(60, -10));
        for (from, to) in [
            (Point::new(0, -10), Point::new(40, -10)),
            (Point::new(80, -10), Point::new(80, 30)),
            (Point::new(80, 30), Point::new(0, 30)),
        ] {
            schematic.add_wire(vec![from, to]).expect("the wire commits");
        }
    });

    run(&mut app);
    app.settle_preflight();

    let blockers = preflight_blockers(&app);
    let closed = blockers
        .iter()
        .find(|issue| issue.check == "Voltage-source loops")
        .unwrap_or_else(|| panic!("the operating point refuses the loop: {blockers:#?}"));
    let remediation = closed.remediation.clone();
    let PreflightRemediation::Drawing { cell_view, object } = &remediation else {
        panic!("the finding is located on the drawing: {closed:?}");
    };
    assert_eq!(cell_view, &amp, "shown in the cell that draws it");
    let crate::workbench::state::DrawnObject::Component { id, .. } = object else {
        panic!("the finding names the winding: {closed:?}");
    };
    assert_eq!(*id, winding);
    assert!(closed.observed.contains("in /XAMP"), "{closed:?}");

    app.state.workbench.activate(Workspace::Verify);
    apply_remediation(&mut app, remediation);
    assert_eq!(app.state.workspace.content.active_schematic_reference(), amp);
    assert!(
        app.state
            .schematic
            .session
            .editor
            .selection
            .has_component(winding),
        "following the finding selects the winding in amp"
    );
}
