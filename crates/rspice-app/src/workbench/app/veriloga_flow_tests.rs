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
use crate::workbench::framework::command_registry::ids;
use crate::workbench::simulate::commands::SimulateCommand;
use crate::workbench::state::Workspace;
use rspice_design::library::metadata::{cell_parameter_contract, decode_source_view_ports};
use rspice_design::symbol::generated::GENERATED_SYMBOL_VIEW;

const SIZE: egui::Vec2 = egui::vec2(1440.0, 900.0);
const CELL: &str = "vcvs_gain";
const PORTS: &str = "inp:in inn:in out:out gnd";
/// The module the user writes: `out` driven `gain` times the input
/// difference above `gnd`.
const MODULE: &str = "`include \"disciplines.vams\"\n\nmodule vcvs_gain(inp, inn, out, gnd);\n  input inp, inn;\n  output out;\n  inout gnd;\n  electrical inp, inn, out, gnd;\n  (* units=\"V/V\", desc=\"Voltage gain\" *) parameter real gain = 10 from (0:inf);\n  analog V(out, gnd) <+ gain * (V(inp) - V(inn));\nendmodule\n";

/// Someone at the application: its egui context, the application, what the
/// last frame named for a screen reader, with where it drew each name, and
/// what its live status regions said.
struct User {
    ctx: egui::Context,
    app: RSpiceApp,
    named: Vec<(String, egui::Rect)>,
    /// What the last frame's named nodes also said of themselves.
    described: Vec<(String, String)>,
    live: Vec<String>,
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
            described: Vec::new(),
            live: Vec::new(),
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
        let Self {
            ctx,
            app,
            named,
            described,
            live,
        } = self;
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
                app.refresh_live_checks(&ctx);
                app.render_frame_dialogs(&ctx);
                app.end_frame(&before);
            },
        );
        named.clear();
        described.clear();
        live.clear();
        for (_, node) in output
            .platform_output
            .accesskit_update
            .into_iter()
            .flat_map(|update| update.nodes)
        {
            if node.role() == egui::accesskit::Role::Status
                && node.live() == Some(egui::accesskit::Live::Polite)
                && let Some(label) = node.label()
            {
                live.push(label.to_owned());
            }
            if let (Some(name), Some(description)) = (node.label(), node.description()) {
                described.push((name.to_owned(), description.to_owned()));
            }
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
            .model
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
        let schematic = &mut self.app.state.model.schematic;
        assert!(
            schematic.with_undo(format!("place {}", kind.display_name()), |schematic| {
                placed = Some(schematic.add_armed_component(kind, terminal - anchor));
            })
        );
        placed.expect("the device is placed")
    }

    /// Draw a wire through `points` with the canvas's wire tool.
    fn wire(&mut self, points: &[Point]) {
        let schematic = &mut self.app.state.model.schematic;
        schematic.start_wire(points[0]);
        for point in &points[1..] {
            schematic.extend_wire(*point);
        }
        assert!(
            schematic.finish_wire(|_| false).is_some(),
            "the wire is drawn"
        );
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

/// What one Run plan consumed and produced: the deck, and the voltage of the
/// node `out` drives at the operating point and every point computed after it.
struct RunOutcome {
    deck: String,
    out_volts: Vec<f64>,
}

/// The device as the flow leaves it: its view.
struct Device {
    view: CellViewRef,
}

/// Steps 1 and 2: a new project, and the device `cell` with `ports` created
/// from its canvas. Returns the top schematic and the device's view, which
/// the Design workspace shows.
fn new_device(cell: &str, ports: &str) -> (User, CellViewRef, CellViewRef) {
    let (mut user, top) = new_project();
    let library = top.library.clone();

    // 2. A right-click on the empty canvas, then New Verilog-A device….
    let canvas = user.expect("Schematic canvas");
    user.press(canvas.center(), egui::PointerButton::Secondary);
    user.settle();
    user.click("New Verilog-A device…");
    let form = &mut user.app.state.code.new_veriloga_device;
    assert!(form.open, "the canvas row opens the form");
    assert_eq!(
        form.library, library,
        "the form offers the project's design library"
    );
    form.name = cell.to_owned();
    form.ports = ports.to_owned();
    user.settle();
    user.click("Create");
    let view = CellViewRef::new(&library, cell, "veriloga");
    assert!(!user.app.state.code.new_veriloga_device.open);
    assert_eq!(user.app.state.model.workspace.content.active_view, view);
    assert_eq!(user.app.state.workbench.workspace, Workspace::Design);
    (user, top, view)
}

/// Step 1: File → New project. Returns the project's top schematic, which
/// the Design workspace shows.
fn new_project() -> (User, CellViewRef) {
    let mut user = User::new(RSpiceApp::test_instance());

    // 1. File → New project, not keeping the session's unsaved scratch work.
    user.app.execute_command_key(ids::NEW_PROJECT);
    user.settle();
    if user.app.state.workbench.confirmation_dialog.visible {
        user.click("Don't save");
    }
    assert!(
        user.app.state.project.new_project.open,
        "New project asks for the project's identity"
    );
    user.app.state.project.new_project.name = "Verilog-A flow".to_owned();
    user.settle();
    user.click("Create project");
    assert!(
        !user.app.state.project.new_project.open,
        "the project is created"
    );
    let project = &user.app.state.model.workspace.content.project;
    let (library, top_cell) = (project.root_library.clone(), project.top_cell.clone());
    let top = CellViewRef::new(&library, &top_cell, "schematic");
    assert_eq!(user.app.state.model.workspace.content.active_view, top);
    assert_eq!(user.app.state.workbench.workspace, Workspace::Design);
    (user, top)
}

/// Compile the open view as the toolbar's Compile does, and frame until the
/// result is published.
fn compile(user: &mut User) {
    assert!(Command::CompileVerilogA.is_enabled(&user.app));
    Command::CompileVerilogA.execute(&mut user.app);
    user.wait_for("the compile result", |app| {
        let veriloga = &app.state.code.code_workspace.veriloga;
        veriloga.compile_request.is_none() && veriloga.pending.is_none()
    });
    user.settle();
}

/// Steps 1 to 7: a new project, the device created from the canvas, written,
/// compiled, placed in the top schematic, wired and set to `gain=5`.
fn wired_device() -> (User, Device) {
    let (mut user, top, view) = new_device(CELL, PORTS);
    let top_cell = top.cell.clone();

    // 3. The module.
    super::veriloga_interface_tests::edit_source(&mut user.app, MODULE);
    user.settle();

    // 4. Compile, which asks nothing: the next frame starts it, and the frame
    // loop publishes it.
    compile(&mut user);
    assert!(!user.app.state.application_modal_open());
    assert!(
        user.app
            .state
            .code
            .code_workspace
            .veriloga
            .last_failure
            .is_empty(),
        "{:?}",
        user.app.state.code.code_workspace.veriloga.last_failure
    );
    assert!(
        user.app
            .state
            .code
            .code_workspace
            .veriloga
            .receipt
            .is_some()
    );

    // 5. The cell's interface, symbol and parameters, and the offer.
    let cell = user
        .app
        .state
        .model
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
    assert_eq!(user.app.state.model.workspace.content.active_view, top);
    assert_eq!(
        user.app.state.model.schematic.session.editor.tool,
        Tool::Place(ComponentType::CellInstance)
    );
    let canvas = user.expect("Schematic canvas");
    user.press(canvas.center(), egui::PointerButton::Primary);
    user.settle();
    user.app.state.model.schematic.cancel_tool();
    let instances = user
        .app
        .state
        .model
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
            .model
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
        .model
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

    (user, Device { view })
}

/// Step 8: Run plan, exactly as the toolbar's run control does, and the
/// result it publishes.
fn run_plan(user: &mut User, device: &Device) -> RunOutcome {
    assert!(
        user.app
            .state
            .model
            .workspace
            .content
            .configuration_sets
            .configurations()
            .is_empty(),
        "no configuration is made by hand"
    );
    let runs_before = user.app.state.model.simulation.retained.runs.len();
    assert!(user.app.command_key_enabled(SimulateCommand::Run.key()));
    user.app.execute_command_key(SimulateCommand::Run.key());
    user.frame(Vec::new());
    if let Some(report) = user.app.state.runs.preflight.report.as_ref() {
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
        if state.model.simulation.retained.runs.len() > runs_before
            && !state.model.simulation.execution.has_active_execution()
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
        .model
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
        .model
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
        state.model.workspace.content.project.id(),
        &state.model.workspace.content.project_sources,
        state.model.library_manager.catalog(),
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
    RunOutcome { deck, out_volts }
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
    let top = user
        .app
        .state
        .model
        .workspace
        .content
        .simulation_root_reference();
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

/// A behavioural op-amp as a reviewer writes it: both divisions are by
/// quantities the parameter ranges keep positive, and the clip reassigns a
/// variable assigned on every path just before it.
const OPAMP: &str = r#"`include "constants.vams"
`include "disciplines.vams"

/* Behavioural op-amp macromodel:
   single pole, slew limited, clipped to the rails. */
module opamp_bhv(inp, inn, out, vdd, vss);
  input inp, inn;
  output out;
  inout vdd, vss;
  electrical inp, inn, out, vdd, vss, mid;

  (* desc="Open-loop gain", units="V/V" *) parameter real a0 = 1e5 from (0:inf);
  (* desc="Gain-bandwidth product", units="Hz" *) parameter real gbw = 10M from (0:inf);
  (* desc="Slew rate", units="V/s" *) parameter real sr = 5M from (0:inf);
  (* desc="Output resistance", units="Ohm" *) parameter real rout = 50 from (0:inf);
  parameter integer clip = 1 from [0:1];

  real vin, vslew, vtarget, tau;

  analog begin
    tau = a0 / (`M_TWO_PI * gbw);
    vin = V(inp, inn);
    (* integrator node *)
    I(mid) <+ -a0 * vin;
    I(mid) <+ V(mid) + tau * ddt(V(mid));
    vslew = slew(V(mid), sr, -sr);
    vtarget = vslew;
    if (clip)
      vtarget = min(max(vslew, V(vss)), V(vdd));
    I(out) <+ (V(out) - vtarget) / rout;
    I(inp, inn) <+ white_noise(4 * `P_K * $temperature * 1k, "input");
  end
endmodule
"#;

impl User {
    /// Press and release `key`, as a keyboard does.
    fn key(&mut self, key: egui::Key, modifiers: egui::Modifiers) {
        for pressed in [true, false] {
            self.frame(vec![egui::Event::Key {
                key,
                physical_key: Some(key),
                pressed,
                repeat: false,
                modifiers,
            }]);
        }
    }

    /// The one-based line and column of the caret the Verilog-A editor last
    /// reported.
    fn caret(&self) -> (usize, usize) {
        let selected =
            crate::workbench::documents::code_workspace::selected_veriloga_source(&self.app)
                .expect("a Verilog-A view is open");
        let path = selected.document().logical_path().to_owned();
        let source = selected
            .bundle()
            .file_content(&path)
            .expect("the open document")
            .to_owned();
        let index = *self
            .app
            .state
            .code
            .code_workspace
            .source_carets
            .get(&(selected.bundle().id(), path))
            .expect("the editor reported its caret");
        source
            .chars()
            .take(index)
            .fold((1, 1), |(line, column), character| {
                if character == '\n' {
                    (line + 1, 1)
                } else {
                    (line, column + 1)
                }
            })
    }

    fn prompt(&self) -> String {
        self.app.state.prompt_text().unwrap_or_default().to_owned()
    }
}

/// The reviewed op-amp compiles with nothing to say about its bounded
/// divisions or its clip, and says so where a screen reader hears it. With two
/// typos it fails on two errors, each listed on the line it is on, and each
/// reached from Problems, from the inspector and with F8 and Shift+F8.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_reviewed_op_amp_compiles_clean_and_each_of_its_errors_is_reached_where_it_was_written() {
    use crate::workbench::app_state::diagnostic_store::listing::listing;
    use crate::workbench::documents::code_workspace::CodeEditorSeverity;

    let (mut user, _, _) = new_device("opamp_bhv", "inp:in inn:in out:out vdd vss");
    super::veriloga_interface_tests::edit_source(&mut user.app, OPAMP);
    user.settle();
    compile(&mut user);

    let veriloga = &user.app.state.code.code_workspace.veriloga;
    assert!(
        veriloga.last_failure.is_empty(),
        "a valid module compiles: {:#?}",
        veriloga.last_failure
    );
    let receipt = veriloga.receipt.as_ref().expect("the module compiled");
    let codes = receipt
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code().to_owned())
        .collect::<Vec<_>>();
    for lint in [
        "VA-CONVERGENCE-DIVISION",
        "VA-HIDDEN-STATE-PARTIAL-ASSIGNMENT",
    ] {
        assert!(
            !codes.iter().any(|code| code == lint),
            "{lint} in {codes:?}"
        );
    }
    let compiled = user.prompt();
    assert!(
        compiled.starts_with("Compiled opamp_bhv: 5 ports, 5 parameters"),
        "{compiled:?}"
    );
    assert!(
        user.live.contains(&compiled),
        "a live status region says {compiled:?}: {:?}",
        user.live
    );

    // A division by a node voltage nothing keeps from zero is one piece of
    // advice, placed on the denominator: an information row the inspector
    // offers as a button, and no warning mark anywhere.
    let advised = OPAMP.replace(
        "I(out) <+ (V(out) - vtarget) / rout;",
        "I(out) <+ (V(out) - vtarget) / V(vdd, vss);",
    );
    super::veriloga_interface_tests::edit_source(&mut user.app, &advised);
    user.settle();
    compile(&mut user);
    assert_eq!(
        user.prompt(),
        "Compiled opamp_bhv: 5 ports, 5 parameters, 1 advisory"
    );
    user.expect("Information at line 30, column 36: Denominator can reach zero");
    let selected = crate::workbench::documents::code_workspace::selected_veriloga_source(&user.app)
        .expect("the module is open");
    assert_eq!(
        crate::workbench::documents::code_workspace::veriloga_compile_status(
            &user.app.state.code.code_workspace.veriloga,
            user.app.state.model.workspace.content.project.id(),
            &selected,
            user.app.state.workbench.messages(),
        ),
        Some(("1 advisory".to_owned(), None)),
        "advice is not a warning"
    );

    // `gbw` misspelt on line 21 and `vslew` on line 26.
    let broken = OPAMP
        .replace("vslew = slew(", "vslw = slew(")
        .replace("* gbw)", "* gbww)");
    super::veriloga_interface_tests::edit_source(&mut user.app, &broken);
    user.settle();
    compile(&mut user);
    let failure = std::sync::Arc::clone(&user.app.state.code.code_workspace.veriloga.last_failure);
    let mut errors = failure
        .iter()
        .filter(|diagnostic| diagnostic.severity == CodeEditorSeverity::Error)
        .map(|diagnostic| {
            (
                diagnostic.line.expect("an error names its line"),
                diagnostic.column.expect("an error names its column"),
                diagnostic.message.to_string(),
            )
        })
        .collect::<Vec<_>>();
    errors.sort();
    let places = errors
        .iter()
        .map(|(line, column, _)| (*line, *column))
        .collect::<Vec<_>>();
    assert_eq!(places, [(21, 29), (26, 5)], "{errors:#?}");
    assert_eq!(user.prompt(), "Compile failed: 2 errors");
    assert!(
        user.live
            .iter()
            .any(|said| said == "Compile failed: 2 errors"),
        "{:?}",
        user.live
    );
    let (gbww, vslw) = (&errors[0].2, &errors[1].2);

    // Problems lists the error on its own line, and opening it puts the
    // caret on its line and column.
    let row_route = {
        let listed = listing(&user.app.state);
        let row = (0..listed.len())
            .filter_map(|index| listed.row(index))
            .find(|row| row.message == gbww.as_str())
            .expect("Problems lists the error");
        assert_eq!(row.location, "line 21 · column 29");
        row.route.clone().expect("the row opens where the error is")
    };
    user.app.state.model.diagnostics.request_route(row_route);
    user.settle();
    assert_eq!(user.caret(), (21, 29));

    // F8 steps on through the document's findings and wraps; Shift+F8 steps
    // back.
    let path = crate::workbench::documents::code_workspace::selected_veriloga_source(&user.app)
        .expect("a Verilog-A view is open")
        .document()
        .logical_path()
        .to_owned();
    let located = failure.view(Some(&path)).located().len();
    user.key(egui::Key::F8, egui::Modifiers::NONE);
    assert_eq!(user.caret(), (26, 5));
    assert_eq!(
        user.prompt(),
        format!("Error 2 of {located}, line 26: {vslw}")
    );
    user.key(egui::Key::F8, egui::Modifiers::SHIFT);
    assert_eq!(user.caret(), (21, 29));
    assert_eq!(
        user.prompt(),
        format!("Error 1 of {located}, line 21: {gbww}")
    );
    user.key(egui::Key::F8, egui::Modifiers::SHIFT);
    assert_eq!(user.caret(), (26, 5), "Shift+F8 wraps to the last");

    // The inspector's row for an error is a button named for where it is,
    // and pressing it puts the caret there.
    user.click(&format!("Error at line 21, column 29: {gbww}"));
    assert_eq!(user.caret(), (21, 29));
}

/// A failed compile's errors stay on the text they are about while it is
/// edited: outdated, moved down with it when a line is added above, and gone
/// with the statement they named when it is deleted. "Show problem at caret"
/// says what the caret is on, and that it was found before the latest edits.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_failed_compiles_errors_follow_their_text_through_edits_until_it_is_deleted() {
    let (mut user, _, _) = new_device("opamp_bhv", "inp:in inn:in out:out vdd vss");
    let broken = OPAMP
        .replace("vslew = slew(", "vslw = slew(")
        .replace("* gbw)", "* gbww)");
    super::veriloga_interface_tests::edit_source(&mut user.app, &broken);
    user.settle();
    compile(&mut user);
    let path = crate::workbench::documents::code_workspace::selected_veriloga_source(&user.app)
        .expect("a Verilog-A view is open")
        .document()
        .logical_path()
        .to_owned();
    let failure = std::sync::Arc::clone(&user.app.state.code.code_workspace.veriloga.last_failure);
    let located = failure.view(Some(&path)).located();
    let count = located.len();
    let (code, gbww) = located
        .iter()
        .find(|diagnostic| diagnostic.line == Some(21))
        .map(|diagnostic| (diagnostic.code().to_owned(), diagnostic.message.to_string()))
        .expect("the misspelt gain-bandwidth product is an error");
    let vslw = located
        .iter()
        .find(|diagnostic| diagnostic.line == Some(26))
        .map(|diagnostic| diagnostic.message.to_string())
        .expect("the misspelt slew variable is an error");
    let step = |user: &mut User, command: Command| {
        command.execute(&mut user.app);
        user.settle();
        user.prompt()
    };
    assert_eq!(
        step(&mut user, Command::NextProblem),
        format!("Error 1 of {count}, line 21: {gbww}")
    );
    assert_eq!(user.caret(), (21, 29));
    assert_eq!(
        step(&mut user, Command::ShowProblemAtCaret),
        format!("Error {code} at line 21, column 29: {gbww}")
    );

    // A line added above: the errors are where their text went.
    let commented = format!("// Reviewed.\n{broken}");
    super::veriloga_interface_tests::edit_source(&mut user.app, &commented);
    user.settle();
    assert_eq!(
        step(&mut user, Command::NextProblem),
        format!("Error 1 of {count}, line 22: {gbww}")
    );
    assert_eq!(user.caret(), (22, 29));
    assert_eq!(
        step(&mut user, Command::ShowProblemAtCaret),
        format!("Error {code} at line 22, column 29, found before your latest edits: {gbww}")
    );

    // The statement with the misspelt variable deleted: its error goes too.
    let statement = commented
        .lines()
        .find(|line| line.contains("vslw = slew("))
        .expect("the statement is in the source");
    let deleted = commented.replace(&format!("{statement}\n"), "");
    super::veriloga_interface_tests::edit_source(&mut user.app, &deleted);
    user.settle();
    for _ in 0..count {
        let said = step(&mut user, Command::NextProblem);
        assert!(said.contains(&format!(" of {}, ", count - 1)), "{said:?}");
        assert!(!said.contains(&vslw), "{said:?}");
    }
}

/// A module about two thousand lines long: a chain of variables, each one
/// more than the one before, driving one branch.
fn long_module(links: usize) -> String {
    let mut source = String::from(
        "`include \"disciplines.vams\"\n\nmodule chain(p, n);\n  inout p, n;\n  electrical p, n;\n",
    );
    for index in 0..links {
        source.push_str(&format!("  real x{index};\n"));
    }
    source.push_str("  analog begin\n    x0 = 1;\n");
    for index in 1..links {
        source.push_str(&format!("    x{index} = x{} + 1;\n", index - 1));
    }
    source.push_str(&format!(
        "    I(p, n) <+ x{} * V(p, n) * 1e-9;\n  end\nendmodule\n",
        links - 1
    ));
    source
}

/// A pause in writing checks the module, with no compile: an error typed
/// into a two-thousand-line module is found half a second after the typing
/// stops, on the line it was typed on, and correcting it clears it the same
/// way.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_pause_in_writing_checks_a_long_module_and_finds_the_error_typed_into_it() {
    let (mut user, _, _) = new_device("chain", "p n");
    let module = long_module(1000);
    assert!(module.lines().count() >= 2000);
    super::veriloga_interface_tests::edit_source(&mut user.app, &module);
    user.settle();
    compile(&mut user);
    assert!(
        user.app
            .state
            .code
            .code_workspace
            .veriloga
            .last_failure
            .is_empty()
    );
    let checking = |app: &RSpiceApp| {
        app.state
            .code
            .code_workspace
            .veriloga
            .live_check
            .running
            .is_some()
    };
    // The pause is timed the way the frame loop sees it.
    let pause_then_check = |user: &mut User, source: &str| {
        user.wait_for("no check on its way", |app| !checking(app));
        super::veriloga_interface_tests::edit_source(&mut user.app, source);
        let edited = user.ctx.input(|input| input.time);
        while !checking(&user.app) {
            user.frame(Vec::new());
            assert!(
                user.ctx.input(|input| input.time) - edited < 1.0,
                "no check started"
            );
        }
        let paused = user.ctx.input(|input| input.time) - edited;
        assert!((0.5..0.55).contains(&paused), "{paused}");
        user.wait_for("the check", |app| !checking(app));
        assert!(
            user.app
                .state
                .code
                .code_workspace
                .veriloga
                .pending
                .is_none(),
            "nothing was compiled"
        );
        Command::NextProblem.execute(&mut user.app);
        user.settle();
        user.prompt()
    };

    let broken = module.replace("    x700 = x699 + 1;\n", "    x700 = x699 + y;\n");
    let line = broken
        .lines()
        .position(|text| text.contains("+ y;"))
        .expect("the typed error")
        + 1;
    let said = pause_then_check(&mut user, &broken);
    assert!(
        said.starts_with(&format!("Error 1 of 1, line {line}: ")) && said.contains("'y'"),
        "{said:?}"
    );
    assert_eq!(
        pause_then_check(&mut user, &module),
        "No problems in this document."
    );
}

/// Four compact models as their authors ship them, each a root with its
/// includes beside it: the folder under `models/veriloga/cmc`, the root, and
/// the module it declares.
const CMC_ROOTS: [(&str, &str, &str); 4] = [
    ("PSP104.1.0_vacode/vacode", "psp104.va", "PSP104VA"),
    ("BSIM-CMG_112.1.0_04282026/code", "bsimcmg.va", "bsimcmg_va"),
    ("505p5p0_va", "bjt505.va", "bjt505_va"),
    ("diode_cmc_3.0_20250714/vacode", "diode_cmc.va", "DIODE_CMC"),
];

/// A compact model imports from its source folder by picking its root: the
/// import reads every include from beside it, compiles, and is placed from
/// the library shelf on the top schematic, each model one cell of its own.
#[test]
fn compact_models_import_from_their_source_folders_compile_and_place() {
    use crate::state::workspace::ImportedInclude;
    use crate::workbench::documents::code_workspace;

    let (mut user, top) = new_project();
    let models = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../models/veriloga/cmc");
    for (index, (folder, root, module)) in CMC_ROOTS.into_iter().enumerate() {
        let folder = models.join(folder);
        let text = std::fs::read_to_string(folder.join(root)).expect("the root is readable");
        Command::ImportVerilogA.execute(&mut user.app);
        code_workspace::complete_veriloga_files_pick(
            &mut user.app,
            &[(root, &text)],
            Some(&folder),
        );
        user.settle();
        let form = user
            .app
            .state
            .code
            .code_workspace
            .veriloga
            .import_form
            .clone()
            .expect("the import form opens");
        assert_eq!(
            form.closure.missing().count(),
            0,
            "{root}: {:?}",
            form.closure.files
        );
        let found = form
            .closure
            .files
            .iter()
            .filter(|file| file.include == ImportedInclude::Found)
            .count();
        assert!(found >= 4, "{root} brings its includes: {found}");
        user.click("Create cells");
        let view = user.app.state.model.workspace.content.active_view.clone();
        assert_eq!(
            (view.cell.as_str(), view.view.as_str()),
            (module, "veriloga")
        );

        compile(&mut user);
        let veriloga = &user.app.state.code.code_workspace.veriloga;
        assert!(
            veriloga.receipt.is_some(),
            "{root} compiles: {:?}",
            veriloga.last_failure
        );

        // The library shelf arms the cell as its row does, then a click on
        // the top schematic places it.
        let candidate = crate::state::library_cell_placement_candidates(
            &user.app.state.model.library_manager,
            &user.app.state.model.workspace,
        )
        .into_iter()
        .find(|candidate| candidate.library == view.library && candidate.cell == view.cell)
        .expect("the cell is on the shelf");
        assert!(candidate.ready, "{root}: {}", candidate.unavailable_reason);
        user.app.state.open_workspace_view(top.clone());
        user.settle();
        crate::workbench::app::schematic::placement::arm_library_cell_placement(
            &mut user.app.state,
            candidate.binding,
        );
        assert_eq!(user.app.state.model.workspace.content.active_view, top);
        let canvas = user.expect("Schematic canvas");
        let at = canvas.center() + egui::vec2(-240.0 + 160.0 * index as f32, 0.0);
        user.press(at, egui::PointerButton::Primary);
        user.settle();
        user.app.state.model.schematic.cancel_tool();
        let placed = user
            .app
            .state
            .model
            .schematic
            .document()
            .components
            .iter()
            .filter(|component| {
                component
                    .library_cell
                    .as_ref()
                    .is_some_and(|binding| binding.cell == module)
            })
            .count();
        assert_eq!(placed, 1, "{module} is placed once");
    }
}

/// The built-in library, as the Library page shows it, has a name, a class
/// heading and a version for each model; a copy of one is a cell the user can
/// edit, which compiles from its own source and places on the sheet, and it
/// carries the branch its generated code was built from.
#[test]
fn a_builtin_model_copied_from_the_library_page_compiles_and_places() {
    use crate::workbench::project::workspace_state::ProjectPage;

    let (mut user, top) = new_project();
    let library = top.library.clone();
    user.app.state.project.project_page = ProjectPage::Library;
    user.app.state.workbench.activate(Workspace::Project);
    user.settle();

    user.click("Verilog-A models");
    assert!(user.find("BSIM-BULK").is_some(), "the models are listed");
    user.app.state.project.project_library_filter = "vbic".to_owned();
    user.settle();
    user.click("VBIC 3-terminal");
    user.click("Copy cell…");
    let project = &user.app.state.project;
    assert!(project.copy_cell_dialog, "Copy cell opens");
    assert_eq!(project.copy_cell_target_library, library);
    assert_eq!(project.copy_cell_name, "vbic13");
    user.click("Copy");

    let view = CellViewRef::new(&library, "vbic13", "veriloga");
    assert!(
        !user.app.state.project.copy_cell_dialog,
        "{:?}",
        user.app.state.project.copy_cell_error
    );
    assert!(
        user.app
            .state
            .model
            .library_manager
            .get_library(&library)
            .and_then(|library| library.get_cell("vbic13"))
            .and_then(|cell| cell.get_view("veriloga"))
            .is_some(),
        "the copy is a Verilog-A view in the design's library"
    );
    assert!(
        !user.app.state.project.builtin_models.open,
        "the page moves to the copy"
    );

    user.app.state.open_workspace_view(view.clone());
    user.app.state.workbench.activate(Workspace::Design);
    user.settle();
    compile(&mut user);
    let veriloga = &user.app.state.code.code_workspace.veriloga;
    assert!(
        veriloga.receipt.is_some(),
        "the copy compiles: {:?}",
        veriloga.last_failure
    );
    let selected = crate::workbench::documents::code_workspace::selected_veriloga_source(&user.app)
        .expect("the copy is selected");
    let profile =
        rspice_simulation::project_veriloga::build_profile::resolve_veriloga_build_profile(
            selected.bundle(),
        )
        .expect("the copy has a build profile");
    assert!(
        profile
            .profile
            .preprocessor
            .defines
            .contains_key("__NGSPICE__"),
        "the copy builds the branch the shipped model was built from"
    );

    let candidate = crate::state::library_cell_placement_candidates(
        &user.app.state.model.library_manager,
        &user.app.state.model.workspace,
    )
    .into_iter()
    .find(|candidate| candidate.library == library && candidate.cell == "vbic13")
    .expect("the copy is on the shelf");
    assert!(candidate.ready, "{}", candidate.unavailable_reason);
    user.app.state.open_workspace_view(top.clone());
    user.settle();
    crate::workbench::app::schematic::placement::arm_library_cell_placement(
        &mut user.app.state,
        candidate.binding,
    );
    let canvas = user.expect("Schematic canvas");
    user.press(canvas.center(), egui::PointerButton::Primary);
    user.settle();
    user.app.state.model.schematic.cancel_tool();
    let placed = user
        .app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| {
            component
                .library_cell
                .as_ref()
                .is_some_and(|binding| binding.cell == "vbic13")
        })
        .count();
    assert_eq!(placed, 1, "the copy places once");
}

/// A save dialog that answers with `self.0` and writes that reach the disk.
struct SavesTo(std::path::PathBuf);

impl crate::workbench::workflows::export_workflow::ExportWorkflowIo for SavesTo {
    fn show_save_dialog(
        &self,
        _config: crate::workbench::workflows::export_workflow::SaveDialogConfig<'_>,
    ) -> Result<Option<std::path::PathBuf>, String> {
        Ok(Some(self.0.clone()))
    }

    fn write_text_file(&self, path: &std::path::Path, contents: &str) -> Result<(), String> {
        let folder = path.parent().expect("the export has a folder");
        std::fs::create_dir_all(folder).map_err(|error| error.to_string())?;
        std::fs::write(path, contents).map_err(|error| error.to_string())
    }

    fn write_waveform_csv(
        &self,
        _dataset: &crate::io::WaveformDataset,
        _path: &std::path::Path,
    ) -> Result<(), String> {
        Ok(())
    }
}

#[test]
fn a_spectre_export_copies_its_sources_beside_the_deck_and_names_them_relatively() {
    let (mut user, device) = wired_device();
    let folder = std::env::temp_dir().join(format!("rspice_spectre_export_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    let deck_path = folder.join("amp.scs");

    crate::workbench::menu_bar::action_export_netlist_with_io(
        &mut user.app.state,
        crate::io::NetlistFormat::Spectre,
        &SavesTo(deck_path.clone()),
    );

    let deck = std::fs::read_to_string(&deck_path).expect("the deck is written");
    assert!(!deck.contains("__rspice_project__"), "{deck}");
    let relative = deck
        .lines()
        .find_map(|line| line.strip_prefix("ahdl_include "))
        .unwrap_or_else(|| panic!("the deck includes its source:\n{deck}"))
        .trim_matches('"');
    assert!(!std::path::Path::new(relative).is_absolute(), "{relative}");
    assert_eq!(
        std::fs::read_to_string(folder.join(relative)).expect("the source is beside the deck"),
        MODULE
    );
    assert!(
        deck.contains(&format!(
            "// {}/{CELL}/veriloga (module {CELL})",
            device.view.library
        )),
        "{deck}"
    );
    assert!(
        deck.lines()
            .any(|line| line.starts_with('X') && line.split(' ').any(|token| token == CELL)),
        "the instance names the module the source declares:\n{deck}"
    );
    let read = rspice_core::netlist::Netlist::parse_with_path(&deck, &deck_path)
        .unwrap_or_else(|error| panic!("the deck reads back: {error}\n{deck}"));
    assert!(
        read.elements
            .iter()
            .any(|element| element.name.eq_ignore_ascii_case("X1")),
        "{deck}"
    );
    let _ = std::fs::remove_dir_all(&folder);
}

mod language_tools;
