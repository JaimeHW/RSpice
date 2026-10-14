//! The Verilog-A device flow, driven end to end the way a user drives it.
//!
//! One project: File → New project, a right-click on the empty canvas and
//! New Verilog-A device…, the module written and compiled,
//! Place in the top schematic and a placement click, the wiring, `gain` set in
//! the inspector, Run plan and the operating point; then the project saved,
//! reopened from disk and run again. Each step goes through the entry point
//! the user's gesture reaches, every frame is a whole application frame, and
//! no configuration is ever made by hand.

use std::time::{Duration, Instant};

use super::*;
use crate::state::{CellViewRef, Component, ComponentType, Point, SymbolDocument, Tool};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::Workspace;
use rspice_design::library::metadata::{cell_parameter_contract, decode_source_view_ports};
use rspice_design::symbol::generated::GENERATED_SYMBOL_VIEW;

const SIZE: egui::Vec2 = egui::vec2(1440.0, 900.0);
const CELL: &str = "vcvs_gain";
const PORTS: &str = "inp:in inn:in out:out gnd";
/// The module the user writes: `out` driven `gain` times the input
/// difference above `gnd`.
const MODULE: &str = "`include \"disciplines.vams\"\n\nmodule vcvs_gain(inp, inn, out, gnd);\n  input inp, inn;\n  output out;\n  inout gnd;\n  electrical inp, inn, out, gnd;\n  (* units=\"V/V\", desc=\"Voltage gain\" *) parameter real gain = 10 from (0:inf);\n  analog V(out, gnd) <+ gain * (V(inp) - V(inn));\nendmodule\n";

/// Someone at the application: its egui context, the application, and what
/// the last frame named for a screen reader, with where it drew each name.
struct User {
    ctx: egui::Context,
    app: RSpiceApp,
    named: Vec<(String, egui::Rect)>,
}

impl User {
    fn new(app: RSpiceApp) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut user = Self {
            ctx,
            app,
            named: Vec::new(),
        };
        user.app.symbol_library = crate::schematic::symbols::SymbolLibrary::load_embedded().ok();
        user.app.state.workbench.inspector_visible = true;
        user.settle();
        user
    }

    /// One application frame, in the order `eframe::App::ui` runs one, closed
    /// at its own boundary: every step that drives the design, the compile,
    /// the run and the selection's cross-probe. The account, live session and
    /// autosave have no part in this flow.
    fn frame(&mut self, events: Vec<egui::Event>) {
        let Self { ctx, app, named } = self;
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SIZE)),
                events,
                ..Default::default()
            },
            |ui| {
                let ctx = ui.ctx().clone();
                let before = FrameSimulationInput::capture(&app.state);
                app.prepare_frame(&ctx);
                app.render_frame_chrome(ui);
                app.render_secondary_application_windows(&ctx);
                app.refresh_incremental_connectivity_checks();
                app.render_frame_dialogs(&ctx);
                app.end_frame(&before);
            },
        );
        named.clear();
        for (_, node) in output
            .platform_output
            .accesskit_update
            .into_iter()
            .flat_map(|update| update.nodes)
        {
            let (Some(name), Some(bounds)) = (node.label().or_else(|| node.value()), node.bounds())
            else {
                continue;
            };
            named.push((
                name.to_owned(),
                egui::Rect::from_min_max(
                    egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                    egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                ),
            ));
        }
    }

    fn settle(&mut self) {
        self.frame(Vec::new());
        self.frame(Vec::new());
    }

    /// Where the last frame drew `name`.
    fn find(&self, name: &str) -> Option<egui::Rect> {
        self.named
            .iter()
            .find(|(candidate, _)| candidate == name)
            .map(|(_, rect)| *rect)
    }

    fn expect(&self, name: &str) -> egui::Rect {
        self.find(name).unwrap_or_else(|| {
            panic!(
                "nothing on screen is named {name:?}; named: {:?}",
                self.named.iter().map(|(name, _)| name).collect::<Vec<_>>()
            )
        })
    }

    fn press(&mut self, at: egui::Pos2, button: egui::PointerButton) {
        self.frame(vec![
            egui::Event::PointerMoved(at),
            egui::Event::PointerButton {
                pos: at,
                button,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ]);
        self.frame(vec![egui::Event::PointerButton {
            pos: at,
            button,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }]);
    }

    fn click(&mut self, name: &str) {
        let at = self.expect(name).center();
        self.press(at, egui::PointerButton::Primary);
        self.settle();
    }

    /// Frame until `done` holds, as the application would while it waits.
    fn wait_for(&mut self, what: &str, done: impl Fn(&RSpiceApp) -> bool) {
        let deadline = Instant::now() + Duration::from_secs(300);
        while !done(&self.app) {
            assert!(Instant::now() < deadline, "timed out waiting for {what}");
            std::thread::sleep(Duration::from_millis(5));
            self.frame(Vec::new());
        }
    }

    fn component(&self, id: u64) -> Component {
        self.app
            .state
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == id)
            .cloned()
            .expect("the component is on the sheet")
    }

    /// Place `kind` with its first terminal on `terminal`, as a placement
    /// click on the canvas does.
    fn place(&mut self, kind: ComponentType, terminal: Point) -> u64 {
        let anchor = Component::new(0, kind, Point::origin())
            .terminal_positions_resolved(None)
            .first()
            .map(|(_, offset)| *offset)
            .expect("the device has a terminal");
        let mut placed = None;
        let schematic = &mut self.app.state.schematic;
        assert!(
            schematic.with_undo(format!("place {}", kind.display_name()), |schematic| {
                placed = Some(schematic.add_armed_component(kind, terminal - anchor));
            })
        );
        placed.expect("the device is placed")
    }

    /// Draw a wire through `points` with the canvas's wire tool.
    fn wire(&mut self, points: &[Point]) {
        let schematic = &mut self.app.state.schematic;
        schematic.start_wire(points[0]);
        for point in &points[1..] {
            schematic.extend_wire(*point);
        }
        assert!(schematic.finish_wire().is_some(), "the wire is drawn");
    }

    /// The other terminal of a two-terminal device placed by [`Self::place`].
    fn second_terminal(&self, id: u64) -> Point {
        self.component(id)
            .terminal_positions_resolved(None)
            .get(1)
            .map(|(_, at)| *at)
            .expect("a second terminal")
    }
}

/// What one Run plan consumed and produced: the deck, the node `out` drives,
/// and its voltage at the operating point and every point computed after it.
struct RunOutcome {
    deck: String,
    out_node: String,
    out_volts: Vec<f64>,
}

/// The device as the flow leaves it: its view, and the placed instance.
struct Device {
    view: CellViewRef,
    instance: u64,
}

/// Steps 1 to 7: a new project, the device created from the canvas, written,
/// compiled, placed in the top schematic, wired and set to `gain=5`.
fn wired_device() -> (User, Device) {
    let mut user = User::new(RSpiceApp::test_instance());

    // 1. File → New project, not keeping the session's unsaved scratch work.
    Command::NewProject.execute(&mut user.app);
    user.settle();
    if user.app.state.dialogs.confirmation_dialog.visible {
        user.click("Don't save");
    }
    assert!(
        user.app.state.dialogs.new_project.open,
        "New project asks for the project's identity"
    );
    user.app.state.dialogs.new_project.name = "Verilog-A flow".to_owned();
    user.settle();
    user.click("Create project");
    assert!(
        !user.app.state.dialogs.new_project.open,
        "the project is created"
    );
    let project = &user.app.state.workspace.content.project;
    let (library, top_cell) = (project.root_library.clone(), project.top_cell.clone());
    let top = CellViewRef::new(&library, &top_cell, "schematic");
    assert_eq!(user.app.state.workspace.content.active_view, top);
    assert_eq!(user.app.state.workbench.workspace, Workspace::Design);

    // 2. A right-click on the empty canvas, then New Verilog-A device….
    let canvas = user.expect("Schematic canvas");
    user.press(canvas.center(), egui::PointerButton::Secondary);
    user.settle();
    user.click("New Verilog-A device…");
    let form = &mut user.app.state.dialogs.new_veriloga_device;
    assert!(form.open, "the canvas row opens the form");
    assert_eq!(
        form.library, library,
        "the form offers the project's design library"
    );
    form.name = CELL.to_owned();
    form.ports = PORTS.to_owned();
    user.settle();
    user.click("Create");
    let view = CellViewRef::new(&library, CELL, "veriloga");
    assert!(!user.app.state.dialogs.new_veriloga_device.open);
    assert_eq!(user.app.state.workspace.content.active_view, view);
    assert_eq!(user.app.state.workbench.workspace, Workspace::Design);

    // 3. The module.
    super::veriloga_interface_tests::edit_source(&mut user.app, MODULE);
    user.settle();

    // 4. Compile, which asks nothing: the next frame starts it, and the frame
    // loop publishes it.
    assert!(Command::CompileVerilogA.is_enabled(&user.app));
    Command::CompileVerilogA.execute(&mut user.app);
    user.wait_for("the compile result", |app| {
        let veriloga = &app.state.ui.code_workspace.veriloga;
        veriloga.compile_request.is_none() && veriloga.pending.is_none()
    });
    assert!(!user.app.state.application_modal_open());
    assert!(
        user.app
            .state
            .ui
            .code_workspace
            .veriloga
            .last_failure
            .is_empty(),
        "{:?}",
        user.app.state.ui.code_workspace.veriloga.last_failure
    );
    assert!(user.app.state.ui.code_workspace.veriloga.receipt.is_some());
    user.settle();

    // 5. The cell's interface, symbol and parameters, and the offer.
    let cell = user
        .app
        .state
        .library_manager
        .get_library(&view.library)
        .and_then(|library| library.get_cell(CELL))
        .expect("the cell");
    let module_view = cell.get_view("veriloga").expect("the veriloga view");
    let ports = module_view
        .metadata
        .get("veriloga.ports")
        .and_then(|encoded| decode_source_view_ports(encoded))
        .expect("the compile states the cell's ports");
    assert_eq!(
        ports
            .iter()
            .map(|port| port.name.as_str())
            .collect::<Vec<_>>(),
        ["inp", "inn", "out", "gnd"]
    );
    let symbol = SymbolDocument::load_from_view(
        cell.get_view(GENERATED_SYMBOL_VIEW)
            .expect("the generated symbol view"),
    )
    .expect("the symbol loads");
    assert_eq!(symbol.pins.len(), 4);
    let contract = cell_parameter_contract(cell, Some(module_view)).expect("a valid contract");
    assert!(
        contract.iter().any(|parameter| parameter.name == "gain"),
        "{contract:?}"
    );
    let place = format!("Place in {top_cell}");
    assert!(user.find(&place).is_some(), "the strip offers {place}");

    // 6. Place in …, then a placement click on the schematic.
    user.click(&place);
    assert_eq!(user.app.state.workspace.content.active_view, top);
    assert_eq!(
        user.app.state.schematic.session.editor.tool,
        Tool::Place(ComponentType::CellInstance)
    );
    let canvas = user.expect("Schematic canvas");
    user.press(canvas.center(), egui::PointerButton::Primary);
    user.settle();
    user.app.state.schematic.cancel_tool();
    let instances = user
        .app
        .state
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| {
            component
                .library_cell
                .as_ref()
                .is_some_and(|binding| binding.cell == CELL)
        })
        .map(|component| component.id)
        .collect::<Vec<_>>();
    let &[instance] = instances.as_slice() else {
        panic!("one click places one instance: {instances:?}");
    };
    let symbols = crate::schematic::view::schematic_symbol_context(&user.app.state);
    let placed = user.component(instance);
    assert!(
        symbols.resolved_symbol(&placed).is_some(),
        "the canvas draws the instance's generated symbol, not an unresolved cell"
    );
    let pins = symbols.named_terminal_points(&placed);
    let pin = |name: &str| {
        pins.iter()
            .find(|(pin, _)| pin == name)
            .map(|(_, at)| *at)
            .unwrap_or_else(|| panic!("the instance draws pin {name}: {pins:?}"))
    };
    let (inp, inn, out, gnd) = (pin("inp"), pin("inn"), pin("out"), pin("gnd"));

    // `inp` from a 1 V source over ground, far enough out that the source's
    // value reads clear of the `inn` route.
    let source_plus = inp + Point::new(-100, 0);
    user.wire(&[inp, source_plus]);
    let source = user.place(ComponentType::VoltageSource, source_plus);
    let source_minus = user.second_terminal(source);
    let source_ground = source_minus + Point::new(0, 20);
    user.wire(&[source_minus, source_ground]);
    user.place(ComponentType::Ground, source_ground);
    let expected = user.component(source);
    let mut one_volt = expected.clone();
    one_volt.value = "1".to_owned();
    assert_eq!(
        user.app
            .state
            .edit_component_transaction(&expected, one_volt, "edit value"),
        Ok(true)
    );
    // `inn` and `gnd` to ground.
    for (terminal, step) in [(inn, -20), (gnd, 20)] {
        let ground = terminal + Point::new(step, 40);
        user.wire(&[terminal, terminal + Point::new(step, 0), ground]);
        user.place(ComponentType::Ground, ground);
    }
    // `out` into a 1 kΩ load to ground.
    let load_plus = out + Point::new(40, 0);
    user.wire(&[out, load_plus]);
    let load = user.place(ComponentType::Resistor, load_plus);
    assert_eq!(user.component(load).value, "1k");
    let load_minus = user.second_terminal(load);
    let load_ground = load_minus + Point::new(20, 60);
    user.wire(&[load_minus, load_minus + Point::new(20, 0), load_ground]);
    user.place(ComponentType::Ground, load_ground);
    user.settle();

    // 7. gain=5, typed into the inspector's row for it.
    user.app
        .state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(instance);
    user.settle();
    user.click("gain");
    user.frame(vec![
        egui::Event::Key {
            key: egui::Key::A,
            physical_key: Some(egui::Key::A),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND,
        },
        egui::Event::Text("5".to_owned()),
    ]);
    user.frame(vec![egui::Event::Key {
        key: egui::Key::Enter,
        physical_key: Some(egui::Key::Enter),
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }]);
    user.settle();
    assert_eq!(
        crate::state::parse_params_string(&user.component(instance).params).get("gain"),
        Some(&"5".to_owned())
    );

    (user, Device { view, instance })
}

/// Step 8: Run plan, exactly as the toolbar's run control does, and the
/// result it publishes.
fn run_plan(user: &mut User, device: &Device) -> RunOutcome {
    assert!(
        user.app
            .state
            .workspace
            .content
            .configuration_sets
            .configurations()
            .is_empty(),
        "no configuration is made by hand"
    );
    let runs_before = user.app.state.simulation.retained.runs.len();
    assert!(Command::RunSimulation.is_enabled(&user.app));
    Command::RunSimulation.execute(&mut user.app);
    user.frame(Vec::new());
    if let Some(report) = user.app.state.workbench.preflight.report.as_ref() {
        assert!(
            report.blockers.is_empty(),
            "Run plan is blocked: {:#?}",
            report.blockers
        );
    }
    // Every deck preflight retained until the run took one.
    let mut retained = Vec::<(String, crate::product::ContentDigest)>::new();
    let deadline = Instant::now() + Duration::from_secs(300);
    loop {
        if let Some((deck, digest)) = user.app.simulation_controller.retained_run_deck()
            && retained.last().is_none_or(|(_, last)| *last != digest)
        {
            retained.push((deck.to_owned(), digest));
        }
        let state = &user.app.state;
        if state.simulation.retained.runs.len() > runs_before
            && !state.simulation.execution.has_active_execution()
        {
            break;
        }
        assert!(Instant::now() < deadline, "timed out waiting for the run");
        std::thread::sleep(Duration::from_millis(5));
        user.frame(Vec::new());
    }
    let run = user
        .app
        .state
        .simulation
        .retained
        .runs
        .iter()
        .max_by_key(|run| run.id)
        .expect("the run is retained");
    let consumed = run
        .prepared_receipt()
        .map(rspice_results::run_receipt::PreparedRunReceipt::prepared_snapshot_digest)
        .expect("the run carries its receipt");
    assert_eq!(
        retained.len(),
        1,
        "Run plan prepares one deck: {:?}",
        retained
            .iter()
            .map(|(_, digest)| digest)
            .collect::<Vec<_>>()
    );
    let deck = retained
        .into_iter()
        .find(|(_, digest)| *digest == consumed)
        .map(|(deck, _)| deck)
        .expect("the run consumed the deck preflight retained");
    for analysis in &run.analyses {
        assert!(analysis.success, "{:?}", analysis.error_message);
    }
    let problems = user
        .app
        .state
        .log_buffer
        .entries()
        .filter(|entry| {
            matches!(
                entry.severity,
                crate::diagnostics::LogSeverity::Error | crate::diagnostics::LogSeverity::Warning
            )
        })
        .map(|entry| entry.message.clone())
        .collect::<Vec<_>>();
    // The one advisory a new project's plan raises on every run, about the
    // outputs it saved for want of a selection, is the plan's and not the
    // device's; anything else is a problem the flow made.
    assert!(
        problems
            .iter()
            .all(|problem| problem.starts_with("Automatic output selection retained 2 bounded")),
        "the flow raises no problem of its own: {problems:#?}"
    );

    // The module's instance, with the value the inspector set.
    let state = &user.app.state;
    let alias = rspice_design::hierarchy::project_veriloga_binding_for_view(
        state.workspace.content.project.id(),
        &state.workspace.content.project_sources,
        state.library_manager.catalog(),
        &device.view,
    )
    .expect("the view owns its bundle")
    .netlist_alias()
    .to_owned();
    let line = deck
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .find(|tokens| {
            tokens
                .iter()
                .any(|token| token.eq_ignore_ascii_case(&alias))
                && tokens
                    .first()
                    .is_some_and(|name| name.to_ascii_uppercase().starts_with('X'))
        })
        .unwrap_or_else(|| panic!("the deck instantiates {alias}:\n{deck}"));
    assert_eq!(
        line.len(),
        7,
        "name, four terminals, the module, gain: {line:?}"
    );
    assert!(line[5].eq_ignore_ascii_case(&alias), "{line:?}");
    assert!(line[6].eq_ignore_ascii_case("gain=5"), "{line:?}");
    let out_node = line[3].to_owned();

    // A new project's plan is one transient: the DC operating point is the
    // one it starts from, its first sample, and a static circuit holds it.
    let name = format!("V({out_node})");
    let out_volts = match run
        .analyses
        .iter()
        .find_map(|analysis| analysis.dc_op.as_ref())
    {
        Some(op) => vec![
            op.node_voltages
                .iter()
                .find(|value| value.name.eq_ignore_ascii_case(&name))
                .unwrap_or_else(|| panic!("{name} in {:?}", op.node_voltages))
                .value,
        ],
        None => run
            .analyses
            .iter()
            .flat_map(|analysis| analysis.waveforms.iter())
            .find(|wave| wave.name.eq_ignore_ascii_case(&name))
            .unwrap_or_else(|| panic!("{name} is retained"))
            .y
            .to_vec(),
    };
    assert!(!out_volts.is_empty(), "{name} has samples");
    RunOutcome {
        deck,
        out_node,
        out_volts,
    }
}

fn assert_five_volts(outcome: &RunOutcome) {
    assert!(
        outcome
            .out_volts
            .iter()
            .all(|volts| ((volts - 5.0) / 5.0).abs() <= 1e-9),
        "V(out) = {:?} V from:\n{}",
        &outcome.out_volts[..outcome.out_volts.len().min(8)],
        outcome.deck
    );
}

/// The whole flow, twice: once as built, and once from the project file.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_new_veriloga_device_runs_from_the_canvas_to_its_operating_point_and_back_from_disk() {
    use crate::workbench::lifecycle::project_lifecycle::{
        DestinationAuthority, SaveScope, save_native,
    };

    let (mut user, device) = wired_device();
    // 8 and 9.
    assert_five_volts(&run_plan(&mut user, &device));

    // 10. Saved, reopened from disk by a fresh application, and run again.
    let path = std::env::temp_dir().join(format!(
        "rspice-va-flow-{}-{}.rspiceproj",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    ));
    save_native(
        &mut user.app.state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("the project saves");
    let top = user.app.state.workspace.content.simulation_root_reference();
    drop(user);

    let mut reopened = User::new(RSpiceApp::test_instance());
    assert!(
        crate::workbench::workflows::project_workflow::load_project_from_path(
            &mut reopened.app.state,
            &path,
        ),
        "the project reopens"
    );
    reopened.app.state.open_workspace_view(top);
    reopened.app.state.workbench.activate(Workspace::Design);
    reopened.settle();
    assert_five_volts(&run_plan(&mut reopened, &device));

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
    let mut lock = path.as_os_str().to_os_string();
    lock.push(".rspice.lock");
    let _ = std::fs::remove_file(std::path::PathBuf::from(lock));
}

/// Renders for review: the schematic with the wired device, its instance
/// inspector, and the operating point after Run plan.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_wired_veriloga_device_for_review() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();
    let mut write = |name: &str, canvas: &crate::ui::raster::Canvas| {
        let path = directory.join(format!("{name}.png"));
        std::fs::write(&path, canvas.png(SIZE.y as usize)).expect("write the render");
        writeln!(report, "{}", path.display()).ok();
    };

    let (mut user, device) = wired_device();
    user.app.state.schematic.session.editor.selection.clear();
    user.app.state.schematic.session.editor.needs_fit = true;
    user.settle();
    let canvas = crate::ui::raster::render(SIZE, |ui, _| user.app.render_frame_chrome(ui));
    write("veriloga-e2e-schematic-1440x900", &canvas);

    user.app
        .state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(device.instance);
    user.settle();
    let canvas = crate::ui::raster::render(SIZE, |ui, _| user.app.render_frame_chrome(ui));
    write("veriloga-e2e-inspector-1440x900", &canvas);

    let outcome = run_plan(&mut user, &device);
    assert_five_volts(&outcome);
    // The run's result, with `out` plotted the way a reader picks it.
    Command::OpenWorkspace(Workspace::Results).execute(&mut user.app);
    user.settle();
    let trace = format!("V({})", outcome.out_node);
    let row = user
        .named
        .iter()
        .find(|(name, _)| name.starts_with(&format!("{trace}, ")))
        .map(|(_, rect)| *rect)
        .unwrap_or_else(|| panic!("the data browser lists {trace}"));
    user.frame(vec![egui::Event::PointerMoved(row.center())]);
    user.frame(vec![egui::Event::PointerMoved(row.center())]);
    user.click(&format!("{trace} plotted"));
    let canvas = crate::ui::raster::render(SIZE, |ui, _| user.app.render_frame_chrome(ui));
    write("veriloga-e2e-result-1440x900", &canvas);
}
