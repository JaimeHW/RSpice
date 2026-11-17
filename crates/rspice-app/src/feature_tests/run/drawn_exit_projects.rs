//! The exit projects drawn as an engineer draws them, through the whole
//! application: New project; each HDL module written into its cell's view in
//! the code editor and compiled, which gives the cell its symbol; every part
//! placed from the shelf and set in the inspector, every pin's net named by
//! a label or a ground symbol; the transient set on the Analyses page; and
//! Run. What the run gives is held to the corpus deck, run through the
//! manual-deck route.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use egui::accesskit::Role;
use egui::{Key, Modifiers};
use rspice_app_runs::simulation::controller::test_execution::run_manual_batch_at_within;
use rspice_app_services::workbench::lifecycle::project_lifecycle::without_audit;
use rspice_app_state::simulation::plan::AnalysisDraft;
use rspice_app_state::state::{AnalysisResultPayload, ComponentType, Point, SimulationRun};
use rspice_results::run::SimulationRunLifecycle;

use crate::feature_tests::replay::{Control, PRIMARY, Replay, controls, only};
use crate::workbench::RSpiceApp;

/// How long one run may take before it is taken to have hung: a corpus
/// transient in an unoptimized build.
const RUN_LIMIT: Duration = Duration::from_secs(20 * 60);

/// A corpus file, by its path under `tests/mixed-corpus`.
fn corpus(relative: &str) -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/mixed-corpus")
        .join(relative)
}

fn corpus_text(relative: &str) -> String {
    std::fs::read_to_string(corpus(relative)).unwrap_or_else(|error| panic!("{relative}: {error}"))
}

/// A placed part's pins in terminal order, each under its port's name.
type Pins = Vec<(String, Point)>;

/// The pin `name` of a placed part.
fn pin(pins: &Pins, name: &str) -> Point {
    pins.iter()
        .find(|(pin, _)| pin.eq_ignore_ascii_case(name))
        .map(|(_, at)| *at)
        .unwrap_or_else(|| panic!("no pin {name} in {pins:?}"))
}

/// The application in a window large enough for the whole drawing, worked
/// with the pointer and the keyboard alone, one frame per event: a press
/// finds its control in the frame on show, as a reader's eye does, and only
/// renders another when that frame does not show it yet.
struct Engineer {
    replay: Replay,
    /// The controls the last frame published.
    shown: Vec<Control>,
    /// Where the last frame drew the schematic canvas.
    canvas: Option<egui::accesskit::Rect>,
}

/// A press or release of the primary button at `pos`.
fn button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    }
}

/// A key pressed and released with `modifiers`.
fn stroke(key: Key, modifiers: Modifiers) -> [egui::Event; 2] {
    [true, false].map(|pressed| egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed,
        repeat: false,
        modifiers,
    })
}

/// `text` as typed, each line break the Enter key.
fn typed(text: &str) -> Vec<egui::Event> {
    let mut events = Vec::new();
    for (index, line) in text.split('\n').enumerate() {
        if index > 0 {
            events.extend(stroke(Key::Enter, Modifiers::NONE));
        }
        if !line.is_empty() {
            events.push(egui::Event::Text(line.to_owned()));
        }
    }
    events
}

impl Engineer {
    /// File > New project, named `name`.
    fn new_project(name: &str) -> Self {
        let mut engineer = Self {
            replay: Replay::new().sized(egui::vec2(2880.0, 1800.0)),
            shown: Vec::new(),
            canvas: None,
        };
        engineer.press("File", Role::MenuItem);
        engineer.press("New project\u{2026}", Role::MenuItem);
        engineer.press("Project name", Role::TextInput);
        engineer.type_text(name);
        engineer.press("Create project", Role::Button);
        engineer
    }

    /// One frame carrying `events`; the controls it published. Held
    /// unsaved-changes answers go unaudited: the lifecycle's own tests
    /// audit them, and here each audit would capture the whole drawing
    /// again on every frame, half of what the drawing costs.
    fn step(&mut self, events: Vec<egui::Event>) -> &[Control] {
        let tree = without_audit(|| self.replay.frame(events))
            .platform_output
            .accesskit_update
            .expect("a frame with AccessKit on publishes its tree");
        if let Some(bounds) = tree.nodes.iter().find_map(|(_, node)| {
            (node.role() == Role::Canvas)
                .then(|| node.bounds())
                .flatten()
        }) {
            self.canvas = Some(bounds);
        }
        self.shown = controls(&tree);
        &self.shown
    }

    /// The one control named `label` with `role` pressed and released at its
    /// centre, once two frames in a row have drawn it there: a popup's or a
    /// window's first frame lays it out unseen and takes no press.
    fn press(&mut self, label: &str, role: Role) -> &[Control] {
        let shown = |controls: &[Control]| {
            let mut found = controls
                .iter()
                .filter(|control| control.label == label && control.role == role);
            match (found.next(), found.next()) {
                (Some(control), None) => Some(control.center),
                _ => None,
            }
        };
        let mut before = shown(&self.shown);
        let mut at = None;
        for _ in 0..4 {
            let now = shown(self.step(Vec::new()));
            if now.is_some() && now == before {
                at = now;
                break;
            }
            before = now;
        }
        let at = at.unwrap_or_else(|| only(&self.shown, label, role).center);
        self.step(vec![egui::Event::PointerMoved(at), button(at, true)]);
        self.step(vec![button(at, false)])
    }

    fn key(&mut self, key: Key, modifiers: Modifiers) -> &[Control] {
        self.step(stroke(key, modifiers).into())
    }

    /// Typed text in one frame, each line break the Enter key.
    fn type_text(&mut self, text: &str) -> &[Control] {
        self.step(typed(text))
    }

    /// The focused field's text selected and replaced by `text` in one
    /// frame, as a typist does it, and committed with Enter when `enter`.
    fn retype(&mut self, text: &str, enter: bool) -> &[Control] {
        let mut events: Vec<_> = stroke(Key::A, PRIMARY).into();
        events.extend(typed(text));
        if enter {
            events.extend(stroke(Key::Enter, Modifiers::NONE));
        }
        self.step(events)
    }

    /// The command `label` from the palette: its key, the search cleared
    /// and given `label`, and the row of that name pressed.
    fn command(&mut self, label: &str) {
        self.key(Key::K, PRIMARY);
        self.press("Command search", Role::ComboBox);
        self.retype(label, false);
        let row = only(&self.shown, label, Role::ListBoxOption);
        assert!(
            !row.description.starts_with("Unavailable:"),
            "{label}: {}",
            row.description
        );
        self.press(label, Role::ListBoxOption);
    }

    /// The cell `cell` with its module `source` as its `view_type` view:
    /// New cell, the view written in the code editor, and Compile, which
    /// gives the cell the symbol its ports draw.
    fn module(&mut self, cell: &str, view_type: &str, source: &str) {
        self.press("File", Role::MenuItem);
        self.press("New cell\u{2026}", Role::MenuItem);
        self.press("Cell name", Role::TextInput);
        self.type_text(cell);
        self.press("Schematic", Role::CheckBox);
        if view_type == "Verilog-A" {
            self.press("Verilog-A", Role::CheckBox);
            self.press("Create", Role::Button);
        } else {
            self.press("Create", Role::Button);
            self.press("File", Role::MenuItem);
            self.press("New cell view\u{2026}", Role::MenuItem);
            self.press("Type", Role::ComboBox);
            self.press(view_type, Role::ListBoxOption);
            self.press("Create", Role::Button);
        }
        self.press("Verilog-A/AMS source editor", Role::MultilineTextInput);
        let mut events: Vec<_> = stroke(Key::A, PRIMARY).into();
        events.push(egui::Event::Paste(source.to_owned()));
        self.step(events);
        self.press("Compile Verilog-A/AMS (F7)", Role::Button);
        let compiled = format!("Compiled {cell}:");
        let started = Instant::now();
        loop {
            let shown = self.step(Vec::new());
            if shown
                .iter()
                .any(|control| control.role == Role::Status && control.label.starts_with(&compiled))
            {
                break;
            }
            let status: Vec<_> = shown
                .iter()
                .filter(|control| control.role == Role::Status)
                .map(|control| control.label.as_str())
                .collect();
            assert!(
                started.elapsed() < Duration::from_secs(120),
                "{cell} did not compile: {status:?}"
            );
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    /// The tab of the top cell's schematic.
    fn top_sheet(&mut self) {
        self.press("top · schematic", Role::Tab);
    }

    /// Where the schematic point `at` is on the screen.
    fn screen(&mut self, at: Point) -> egui::Pos2 {
        let bounds = match self.canvas {
            Some(bounds) => bounds,
            None => {
                self.step(Vec::new());
                self.canvas.expect("the schematic canvas")
            }
        };
        let editor = &self.replay.app.state.model.schematic.session.editor;
        let x = bounds.x0 + editor.pan.0 as f64 + f64::from(at.x) * editor.zoom as f64;
        let y = bounds.y0 + editor.pan.1 as f64 + f64::from(at.y) * editor.zoom as f64;
        assert!(
            x < bounds.x1 && y < bounds.y1,
            "{at:?} is off the canvas {bounds:?}"
        );
        egui::pos2(x as f32, y as f32)
    }

    /// A press and release on the canvas at `at`, the pointer brought there
    /// first: the canvas places and picks where the pointer hovered.
    fn tap(&mut self, at: Point) -> &[Control] {
        let pos = self.screen(at);
        self.step(vec![egui::Event::PointerMoved(pos)]);
        self.step(vec![button(pos, true)]);
        self.step(vec![button(pos, false)])
    }

    fn escape(&mut self) {
        self.key(Key::Escape, Modifiers::NONE);
    }

    /// The inspector field `label` given `text`.
    fn field(&mut self, label: &str, text: &str) {
        self.press(label, Role::TextInput);
        let shown = self.retype(text, true).to_vec();
        // A refused entry is kept, outlined, in its field and never reaches
        // the design: say what refused it rather than draw on without it.
        let inline_edit = &self.replay.app.state.workbench.inline_edit;
        assert!(
            !inline_edit.is_open(),
            "{label} = {text} was not applied: {} {:?}",
            shown
                .iter()
                .find(|control| control.label == label)
                .map_or("", |control| control.description.as_str()),
            inline_edit
                .refused()
                .iter()
                .map(|refused| (&refused.buffer, &refused.error))
                .collect::<Vec<_>>()
        );
    }

    /// The shelf's `row`, found by `query`, placed with its origin at `at`,
    /// then selected and its inspector `fields` set. Its pins.
    fn place(&mut self, query: &str, row: &str, at: (i32, i32), fields: &[(&str, &str)]) -> Pins {
        let origin = Point::new(at.0, at.1);
        self.command("Place instance");
        self.press("Place component or cell\u{2026}", Role::TextInput);
        self.retype(query, false);
        self.press(row, Role::Button);
        self.tap(origin);
        self.escape();
        self.escape();
        let placed = self.last_part();
        assert_eq!(placed.pos, origin, "{row} lands where it was put down");
        self.tap(origin);
        for (label, text) in fields {
            self.field(label, text);
        }
        self.escape();
        self.pins_of(&self.last_part())
    }

    fn last_part(&self) -> rspice_app_state::state::Component {
        self.replay
            .app
            .state
            .model
            .schematic
            .document()
            .components
            .last()
            .cloned()
            .expect("a placed part")
    }

    /// The pins of `component` where the sheet draws them, a numbered pin
    /// named by the binding's terminal order.
    fn pins_of(&self, component: &rspice_app_state::state::Component) -> Pins {
        let model = &self.replay.app.state.model;
        let resolver = rspice_design::symbol_resolver::SymbolResolver::new(
            model.library_manager.catalog(),
            &model.workspace.content.schematic_buffers,
        );
        let order = component
            .library_cell
            .as_ref()
            .map(|binding| binding.terminal_order.clone())
            .unwrap_or_default();
        rspice_design::connectivity::terminal_positions(component, &|binding| {
            resolver.resolve_binding(binding)
        })
        .into_iter()
        .enumerate()
        .map(|(index, (pin, at))| {
            let port =
                if order.is_empty() || order.iter().any(|port| port.eq_ignore_ascii_case(&pin)) {
                    pin
                } else {
                    order.get(index).cloned().unwrap_or(pin)
                };
            (port, at)
        })
        .collect()
    }

    /// A net label `net` on the pin at `at`.
    fn label(&mut self, at: Point, net: &str) {
        self.command("Place net label");
        self.tap(at);
        self.press("Name", Role::TextInput);
        // The label takes its name on the frame after the typing.
        self.type_text(net);
        self.key(Key::Enter, Modifiers::NONE);
        self.escape();
        let labels = &self.replay.app.state.model.schematic.document().net_labels;
        assert!(
            labels
                .iter()
                .any(|label| label.pos == at && label.name == net),
            "{net} is not on {at:?}"
        );
    }

    /// A ground symbol on the pin at `at`, hanging below it.
    fn ground(&mut self, at: Point) {
        self.command("Place ground");
        self.tap(Point::new(at.x, at.y + 10));
        self.escape();
        let ground = self.last_part();
        assert_eq!(ground.kind, ComponentType::Ground);
        let pins = self.pins_of(&ground);
        assert_eq!(
            pins.first().map(|(_, pin)| *pin),
            Some(at),
            "ground on {at:?}"
        );
    }

    /// Each pin given its net: `0` is a ground symbol, an empty net leaves
    /// the pin alone, any other a label.
    fn connect(&mut self, pins: &Pins, nets: &[&str]) {
        assert_eq!(pins.len(), nets.len(), "a net for every pin of {pins:?}");
        for ((_, at), net) in pins.iter().zip(nets) {
            match *net {
                "" => {}
                "0" => self.ground(*at),
                net => self.label(*at, net),
            }
        }
    }

    /// The plan's transient: Simulate > Setup > Analyses, its stop and
    /// step.
    fn transient(&mut self, step: &str, stop: &str) {
        self.press("Simulate", Role::MenuItem);
        self.press("Setup", Role::MenuItem);
        self.press("Analyses", Role::MenuItem);
        self.field("Stop time", stop);
        self.field("Step time", step);
        let plan = self
            .replay
            .app
            .state
            .model
            .sim_setup
            .stable_analysis_plan()
            .expect("the plan resolves");
        let transient = plan
            .instances()
            .iter()
            .find_map(|instance| match instance.draft() {
                AnalysisDraft::Transient(setup) => Some((setup.step.clone(), setup.stop.clone())),
                _ => None,
            })
            .expect("the plan holds a transient");
        assert_eq!(
            transient,
            (step.to_owned(), stop.to_owned()),
            "the transient as typed"
        );
    }

    /// Run, and the run once it has ended.
    fn run(&mut self) -> SimulationRun {
        self.key(Key::F5, Modifiers::NONE);
        let started = Instant::now();
        loop {
            self.step(Vec::new());
            let runs = &self.replay.app.state.model.simulation.retained.runs;
            if let Some(run) = runs.first()
                && !matches!(
                    run.lifecycle,
                    SimulationRunLifecycle::Preparing | SimulationRunLifecycle::Running
                )
            {
                assert_eq!(
                    run.lifecycle,
                    SimulationRunLifecycle::Completed,
                    "{}",
                    format!("{:?}", run.log)
                        .chars()
                        .take(3000)
                        .collect::<String>()
                );
                return run.clone();
            }
            assert!(started.elapsed() < RUN_LIMIT, "the run did not end");
            // The run is the simulator's: a frame only looks in on it.
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    /// Add measurement over the result on show: the command, `expression`
    /// typed into its field, and Add. The value the project keeps.
    fn measure(&mut self, expression: &str) -> f64 {
        self.command("Add measurement");
        self.press("Expression", Role::TextInput);
        self.type_text(expression);
        let shown = self.press("Add", Role::Button).to_vec();
        // A form that stays open says why under its field, or in the status
        // line.
        assert!(
            self.replay
                .app
                .state
                .results
                .result_document_dialog
                .is_none(),
            "Add did not keep {expression}: {:?} {:?}",
            self.replay
                .app
                .state
                .workbench
                .prompt
                .as_ref()
                .map(|prompt| &prompt.text),
            shown
                .iter()
                .skip_while(|control| control.role != Role::Dialog)
                .filter(|control| !control.label.is_empty())
                .map(|control| format!("{:?} {} {}", control.role, control.label, control.value))
                .collect::<Vec<_>>()
        );
        kept(&self.replay.app, expression)
    }

    /// The project saved, and reopened by a fresh application.
    fn reopened(&mut self) -> RSpiceApp {
        let path = std::env::temp_dir().join(format!(
            "rspice-drawn-{}-{}.rspiceproj",
            std::process::id(),
            uuid::Uuid::new_v4().simple()
        ));
        assert!(
            self.replay.app.state.save_project_to_path(&path),
            "the project saves"
        );
        let mut reopened = RSpiceApp::test_instance();
        let loaded = reopened.state.load_project_from_path(&path);
        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
        let mut lock = path.as_os_str().to_os_string();
        lock.push(".rspice.lock");
        let _ = std::fs::remove_file(PathBuf::from(lock));
        assert!(loaded, "the project reopens");
        reopened
    }
}

/// The value the project keeps for the measurement `expression`.
fn kept(app: &RSpiceApp, expression: &str) -> f64 {
    app.state
        .model
        .workspace
        .content
        .visualization_documents
        .iter()
        .flat_map(|document| document.measurements())
        .find(|measurement| measurement.expression.as_deref() == Some(expression))
        .and_then(|measurement| measurement.value)
        .unwrap_or_else(|| panic!("{expression} is kept"))
}

/// The value the run measured as `name`, a deck's `.MEASURE`.
fn measured(run: &SimulationRun, name: &str) -> f64 {
    let result = run
        .analyses
        .iter()
        .flat_map(|analysis| &analysis.measurements)
        .find(|result| result.name.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("no measurement {name}"));
    result
        .value
        .unwrap_or_else(|| panic!("{name} measured nothing: {result:?}"))
}

/// Run a corpus deck through the manual-deck route with its transient cut
/// to `stop`.
fn deck_route(relative: &str, step: &str, stop: &str) -> SimulationRun {
    let mut source = String::new();
    for line in corpus_text(relative).lines() {
        if line.trim_start().to_ascii_lowercase().starts_with(".tran ") {
            source.push_str(&format!(".tran {step} {stop}\n"));
        } else {
            source.push_str(line);
            source.push('\n');
        }
    }
    run_manual_batch_at_within(
        &source,
        &corpus(relative),
        SimulationRunLifecycle::Completed,
        RUN_LIMIT,
    )
}

/// The committed events of the digital net `name`: `(time, value code)`.
fn events(run: &SimulationRun, name: &str) -> Vec<(f64, u8)> {
    run.analyses
        .iter()
        .filter_map(|analysis| match analysis.result_payload.as_ref() {
            Some(AnalysisResultPayload::TransientEvents { digital_traces, .. }) => {
                Some(digital_traces)
            }
            _ => None,
        })
        .flatten()
        .find(|trace| trace.node_name.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("no D({name}) in the run"))
        .points
        .iter()
        .map(|point| (point.time_s, point.value_code))
        .collect()
}

/// A digital net's value at `time`: its last event at or before it.
fn level_at(events: &[(f64, u8)], time: f64) -> u8 {
    events
        .iter()
        .rev()
        .find(|(at, _)| *at <= time)
        .map_or(u8::MAX, |(_, code)| *code)
}

/// An analog net's sample held at `time`.
fn held_at(run: &SimulationRun, name: &str, time: f64) -> f64 {
    let wave = run
        .analyses
        .iter()
        .flat_map(|analysis| &analysis.waveforms)
        .find(|wave| {
            wave.name.eq_ignore_ascii_case(name)
                || wave.name.eq_ignore_ascii_case(&format!("V({name})"))
        })
        .unwrap_or_else(|| panic!("no {name} in the run"));
    let index = wave
        .x
        .iter()
        .rposition(|t| *t <= time)
        .expect("a sample at or before the time");
    wave.y[index]
}

/// C4, the 8-bit SAR ADC (`tests/mixed-corpus/c04-sar8`, variant `ports`):
/// the successive-approximation register as a Verilog view, the comparator
/// as a Verilog-A view, and the capacitor DAC, its switches and their
/// drives drawn on the top sheet.
mod c04 {
    use super::*;

    /// The codes of the first conversions of the corpus staircase.
    const CODES: [u32; 2] = [0, 1];

    /// The deck's `.MEASURE tconv` as the calculator writes it: samp's first
    /// fall to eoc's first rise.
    const TCONV: &str = r#"delay(D(samp), D(eoc), 0.5, "falling", 1, 0.5, "rising", 1)"#;

    /// tconv's tolerance in the corpus (`c04-sar8/tolerance.toml`): each
    /// edge held to the register's 1 ps precision, two roundings.
    const TCONV_ABS: f64 = 2e-12;

    /// The staircase's PWL table, as the corpus states it.
    fn staircase() -> String {
        corpus_text("c04-sar8/net/vin_codes.cir")
            .lines()
            .filter(|line| !line.starts_with('*'))
            .map(|line| {
                line.trim_start_matches("vin vin 0 pwl(")
                    .trim_start_matches('+')
                    .trim_end_matches(')')
                    .trim()
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn codes(run: &SimulationRun) -> Vec<u32> {
        let eoc = events(run, "eoc");
        let bits: Vec<_> = (0..8)
            .rev()
            .map(|k| events(run, &format!("b{k}")))
            .collect();
        eoc.windows(2)
            .filter(|pair| pair[0].1 == 0 && pair[1].1 == 1)
            .map(|pair| {
                bits.iter().fold(0, |code, bit| {
                    code << 1 | u32::from(level_at(bit, pair[1].0) == 1)
                })
            })
            .collect()
    }

    /// The SAR drawn from New project to its results: the codes of the
    /// first two conversions, the code probe's level, and the conversion
    /// time measured over the result, kept through a save and a reopen, are
    /// the deck's.
    #[test]
    fn c4_drawn_through_the_application_converts_as_its_deck() {
        let mut engineer = Engineer::new_project("sar");
        engineer.module("sar8", "Verilog", &corpus_text("c04-sar8/net/sar8.v"));
        engineer.module(
            "cmp_va",
            "Verilog-A",
            &corpus_text("c04-sar8/net/cmp_va.va"),
        );
        engineer.top_sheet();
        let e = &mut engineer;

        // Supplies, references and stimulus along the top.
        let source = |e: &mut Engineer, x, name: &str, value: &str, net: &str| {
            let pins = e.place(
                "voltage source",
                "Voltage Source",
                (x, 100),
                &[("Instance", name), ("Value", value)],
            );
            e.connect(&pins, &[net, "0"]);
        };
        source(e, 100, "vdd", "1.8", "vdd");
        source(e, 250, "vref", "1.0", "vref");
        source(e, 400, "vcref", "0", "cref");
        let pulse = |e: &mut Engineer, x, name: &str, levels: (&str, &str), td, pw, per, net| {
            let pins = e.place(
                "pulse",
                "Pulse Voltage Source",
                (x, 100),
                &[
                    ("Instance", name),
                    ("Value", levels.0),
                    ("Pulsed value (V2), V", levels.1),
                    ("Delay time (TD), s", td),
                    ("Rise time (TR), s", "1n"),
                    ("Fall time (TF), s", "1n"),
                    ("Pulse width (PW), s", pw),
                    ("Period (PER), s", per),
                ],
            );
            e.connect(&pins, &[net, "0"]);
        };
        pulse(e, 550, "vclk", ("0", "1.8"), "0", "49n", "100n", "clk");
        pulse(e, 700, "vstart", ("1.8", "0"), "50n", "898n", "1u", "start");
        let input = e.place(
            "pwl",
            "PWL Voltage Source",
            (850, 100),
            &[
                ("Instance", "vin"),
                ("Value", &format!("PWL({})", staircase())),
            ],
        );
        e.connect(&input, &["vin", "0"]);

        // The sampling switch grounds the top plate while samp is high; the
        // bottom plates' input switches follow samp through an RC delay.
        let switch = |e: &mut Engineer, at, name: &str, nets: &[&str]| {
            let pins = e.place(
                "V-Switch",
                "V-Switch (S)",
                at,
                &[
                    ("Instance", name),
                    ("Threshold voltage, V", "0.9"),
                    ("Hysteresis voltage, V", "0.05"),
                    ("On resistance, Ω", "50"),
                    ("Off resistance, Ω", "1e12"),
                ],
            );
            e.connect(&pins, nets);
        };
        switch(e, (1000, 100), "stop", &["top", "0", "samp", "0"]);
        let rsd = e.place(
            "resistor",
            "Resistor",
            (1150, 100),
            &[("Instance", "rsd"), ("Value", "1k")],
        );
        e.connect(&rsd, &["samp", "samp_d"]);
        let csd = e.place(
            "capacitor",
            "Capacitor",
            (1300, 100),
            &[("Instance", "csd"), ("Value", "1p")],
        );
        e.connect(&csd, &["samp_d", "0"]);

        // The capacitor DAC, a column per bit: its capacitor off the top
        // plate, the bottom plate's three switches, and their two drives.
        let behavioural = |e: &mut Engineer, at, name: &str, expression: &str, net: &str| {
            let pins = e.place(
                "behavioral",
                "Behavioral Source",
                at,
                &[("Instance", name), ("Value", expression)],
            );
            e.connect(&pins, &[net, "0"]);
        };
        let columns = [
            "20f", "40f", "80f", "160f", "320f", "640f", "1.28p", "2.56p",
        ];
        for (k, value) in columns.into_iter().enumerate() {
            let x = 100 + 160 * k as i32;
            let (plate, bit) = (format!("bt{k}"), format!("b{k}"));
            let (reference, ground) = (format!("cr{k}"), format!("cg{k}"));
            let cap = e.place(
                "capacitor",
                "Capacitor",
                (x, 300),
                &[("Instance", &format!("ctop{k}")), ("Value", value)],
            );
            e.connect(&cap, &["top", &plate]);
            switch(
                e,
                (x, 460),
                &format!("sv{k}"),
                &[&plate, "vin", "samp_d", "0"],
            );
            switch(
                e,
                (x, 620),
                &format!("sr{k}"),
                &[&plate, "vref", &reference, "0"],
            );
            switch(e, (x, 780), &format!("sg{k}"), &[&plate, "0", &ground, "0"]);
            behavioural(
                e,
                (x, 940),
                &format!("bcr{k}"),
                &format!("V = v({bit})*(1.8-v(samp_d))/1.8"),
                &reference,
            );
            behavioural(
                e,
                (x, 1100),
                &format!("bcg{k}"),
                &format!("V = (1.8-v({bit}))*(1.8-v(samp_d))/1.8"),
                &ground,
            );
        }
        let x = 100 + 160 * 8;
        let dummy = e.place(
            "capacitor",
            "Capacitor",
            (x, 300),
            &[("Instance", "ctopd"), ("Value", "20f")],
        );
        e.connect(&dummy, &["top", "btd"]);
        switch(e, (x, 460), "svd", &["btd", "vin", "samp_d", "0"]);
        switch(e, (x, 780), "sgd", &["btd", "0", "cgd", "0"]);
        behavioural(e, (x, 1100), "bcgd", "V = 1.8-v(samp_d)", "cgd");

        // The code probe.
        let code = (0..8)
            .map(|index| format!("(v(b{})>0.9 ? {} : 0)", 7 - index, 128 >> index))
            .collect::<Vec<_>>()
            .join(" + ");
        behavioural(e, (1450, 100), "bcode", &format!("V = {code}"), "code");

        // The register and the comparator.
        let register = e.place("sar8", "sar8", (1800, 500), &[("Instance", "xsar")]);
        for (port, at) in register.clone() {
            e.label(at, &port.to_ascii_lowercase());
        }
        let comparator = e.place("cmp_va", "cmp_va", (1800, 1000), &[("Instance", "xcmp")]);
        e.label(pin(&comparator, "inp"), "top");
        e.label(pin(&comparator, "inn"), "cref");
        e.label(pin(&comparator, "out"), "cmp");
        e.label(pin(&comparator, "vdd"), "vdd");
        e.ground(pin(&comparator, "gnd"));

        e.transient("1n", "2u");
        let designed = e.run();
        let deck = deck_route("c04-sar8/deck/ports.tran.cir", "1n", "2u");

        assert_eq!(
            codes(&designed),
            codes(&deck),
            "the codes the deck converts"
        );
        assert_eq!(codes(&designed), CODES, "the staircase's codes");
        for (k, expected) in CODES.iter().enumerate() {
            let time = 0.97e-6 + 1e-6 * k as f64;
            let (app, reference) = (
                held_at(&designed, "code", time),
                held_at(&deck, "code", time),
            );
            assert!(
                (app - reference).abs() <= 0.01,
                "at {time:e}: {app} against {reference}"
            );
            assert!(
                (app - f64::from(*expected)).abs() <= 0.01,
                "at {time:e}: {app}"
            );
        }

        // Nine 100 ns clock periods, measured as the deck measures them.
        let tconv = e.measure(TCONV);
        let reference = measured(&deck, "tconv");
        assert!(
            (tconv - reference).abs() <= TCONV_ABS,
            "tconv {tconv:e} against the deck's {reference:e}"
        );
        assert!((tconv - 9e-7).abs() <= TCONV_ABS, "tconv {tconv:e}");
        assert_eq!(kept(&e.reopened(), TCONV), tconv, "tconv reopens as kept");
    }
}
