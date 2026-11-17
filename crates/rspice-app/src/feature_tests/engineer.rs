//! An engineer at the whole application: the pointer and the keyboard, one
//! frame per event, on a window large enough for a whole drawing. New
//! project, a cell's HDL view written and compiled, parts placed from the
//! shelf and set in the inspector, pins named by labels and grounds, the
//! plan's pages, Run, a measurement over the result, and the project saved
//! and reopened. The drawn exit projects are its first users.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use egui::accesskit::Role;
use egui::{Key, Modifiers};
use rspice_app_services::workbench::lifecycle::project_lifecycle::without_audit;
use rspice_app_state::simulation::plan::AnalysisDraft;
use rspice_app_state::state::{ComponentType, Point, SimulationRun};
use rspice_results::run::SimulationRunLifecycle;

use crate::feature_tests::replay::{Control, PRIMARY, Replay, controls, only};
use crate::workbench::RSpiceApp;

/// How long one run may take before it is taken to have hung: a corpus
/// transient in an unoptimized build.
pub(super) const RUN_LIMIT: Duration = Duration::from_secs(20 * 60);

/// A placed part's pins in terminal order, each under its port's name.
pub(super) type Pins = Vec<(String, Point)>;

/// The pin `name` of a placed part.
pub(super) fn pin(pins: &Pins, name: &str) -> Point {
    pins.iter()
        .find(|(pin, _)| pin.eq_ignore_ascii_case(name))
        .map(|(_, at)| *at)
        .unwrap_or_else(|| panic!("no pin {name} in {pins:?}"))
}

/// The application in a window large enough for the whole drawing, worked
/// with the pointer and the keyboard alone, one frame per event: a press
/// finds its control in the frame on show, as a reader's eye does, and only
/// renders another when that frame does not show it yet.
pub(super) struct Engineer {
    pub(super) replay: Replay,
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
    pub(super) fn new_project(name: &str) -> Self {
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
    pub(super) fn step(&mut self, events: Vec<egui::Event>) -> &[Control] {
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
    pub(super) fn press(&mut self, label: &str, role: Role) -> &[Control] {
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

    pub(super) fn key(&mut self, key: Key, modifiers: Modifiers) -> &[Control] {
        self.step(stroke(key, modifiers).into())
    }

    /// Typed text in one frame, each line break the Enter key.
    pub(super) fn type_text(&mut self, text: &str) -> &[Control] {
        self.step(typed(text))
    }

    /// The focused field's text selected and replaced by `text` in one
    /// frame, as a typist does it, and committed with Enter when `enter`.
    pub(super) fn retype(&mut self, text: &str, enter: bool) -> &[Control] {
        let mut events: Vec<_> = stroke(Key::A, PRIMARY).into();
        events.extend(typed(text));
        if enter {
            events.extend(stroke(Key::Enter, Modifiers::NONE));
        }
        self.step(events)
    }

    /// The command `label` from the palette: its key, the search cleared
    /// and given `label`, and the row of that name pressed.
    pub(super) fn command(&mut self, label: &str) {
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
    pub(super) fn module(&mut self, cell: &str, view_type: &str, source: &str) {
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

    /// The tab of the top cell's schematic, from the Schematic workspace
    /// when the workspace on show has no document tabs.
    pub(super) fn top_sheet(&mut self) {
        self.cell_sheet("top");
    }

    /// The tab of the cell `cell`'s schematic, as [`Self::top_sheet`].
    pub(super) fn cell_sheet(&mut self, cell: &str) {
        let tab = format!("{cell} \u{b7} schematic");
        if !self
            .step(Vec::new())
            .iter()
            .any(|control| control.role == Role::Tab && control.label == tab)
        {
            self.press("Schematic", Role::Button);
        }
        self.press(&tab, Role::Tab);
    }

    /// Where the schematic point `at` is on the screen.
    pub(super) fn screen(&mut self, at: Point) -> egui::Pos2 {
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
    pub(super) fn tap(&mut self, at: Point) -> &[Control] {
        let pos = self.screen(at);
        self.step(vec![egui::Event::PointerMoved(pos)]);
        self.step(vec![button(pos, true)]);
        self.step(vec![button(pos, false)])
    }

    pub(super) fn escape(&mut self) {
        self.key(Key::Escape, Modifiers::NONE);
    }

    /// The inspector field `label` given `text`.
    pub(super) fn field(&mut self, label: &str, text: &str) {
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
    pub(super) fn place(
        &mut self,
        query: &str,
        row: &str,
        at: (i32, i32),
        fields: &[(&str, &str)],
    ) -> Pins {
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

    pub(super) fn last_part(&self) -> rspice_app_state::state::Component {
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
    pub(super) fn pins_of(&self, component: &rspice_app_state::state::Component) -> Pins {
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
    pub(super) fn label(&mut self, at: Point, net: &str) {
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
    pub(super) fn ground(&mut self, at: Point) {
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
    pub(super) fn connect(&mut self, pins: &Pins, nets: &[&str]) {
        assert_eq!(pins.len(), nets.len(), "a net for every pin of {pins:?}");
        for ((_, at), net) in pins.iter().zip(nets) {
            match *net {
                "" => {}
                "0" => self.ground(*at),
                net => self.label(*at, net),
            }
        }
    }

    /// The plan's page `page`: Simulate > Setup > `page`.
    pub(super) fn setup_page(&mut self, page: &str) {
        self.press("Simulate", Role::MenuItem);
        self.press("Setup", Role::MenuItem);
        self.press(page, Role::MenuItem);
    }

    /// A new cell `cell` with a schematic, from File > New cell…, its sheet
    /// open.
    pub(super) fn schematic_cell(&mut self, cell: &str) {
        self.press("File", Role::MenuItem);
        self.press("New cell\u{2026}", Role::MenuItem);
        self.press("Cell name", Role::TextInput);
        self.type_text(cell);
        self.press("Create", Role::Button);
        self.cell_sheet(cell);
    }

    /// Pins `names` (space-separated) on the open sheet from the Create pins
    /// form, of direction `direction` and analog, each put down at its
    /// point in `at`. A pin names the net it stands for.
    pub(super) fn pins(&mut self, names: &str, direction: &str, at: &[(i32, i32)]) {
        self.command("Create pins");
        // The form offers the next free name after the last pin's.
        self.press("Names", Role::TextInput);
        self.retype(names, false);
        self.press(direction, Role::RadioButton);
        self.press("Analog", Role::RadioButton);
        self.press("Place", Role::Button);
        for point in at {
            self.tap(Point::new(point.0, point.1));
        }
        self.escape();
        self.escape();
        let placed: Vec<_> = self
            .replay
            .app
            .state
            .model
            .schematic
            .document()
            .components
            .iter()
            .filter(|part| part.kind == ComponentType::Port)
            .map(|part| part.value.clone())
            .collect();
        for name in names.split_whitespace() {
            assert!(
                placed.iter().any(|pin| pin == name),
                "pin {name} is placed: {placed:?}"
            );
        }
    }

    /// The symbol of the cell `cell` made from its open schematic.
    pub(super) fn symbol_from_schematic(&mut self, cell: &str) {
        self.command("Create symbol from schematic");
        let opened = format!("/{cell}/symbol.");
        let shown = self.step(Vec::new());
        assert!(
            shown
                .iter()
                .any(|control| control.role == Role::Status && control.label.ends_with(&opened)),
            "the symbol of {cell} opens"
        );
    }

    /// A measurement the plan defines with its own `.MEAS` card, from Verify:
    /// Add specification > New measurement…, named `name`, its source set
    /// to be defined when the plan runs, and `card` written in.
    pub(super) fn plan_measurement(&mut self, name: &str, card: &str) {
        self.press("Verify", Role::Button);
        self.press("Add specification", Role::Button);
        self.press("New measurement\u{2026}", Role::Button);
        self.field("Name", name);
        self.field("Measurement", name);
        self.press("Source", Role::ComboBox);
        self.press("Defined when the plan runs", Role::Button);
        self.field("Card", card);
        let model = &self.replay.app.state.model;
        let plan = model.sim_setup.analysis_plan.as_ref().expect("a plan").id();
        let defined = model.workspace.content.plan_data(plan).is_some_and(|data| {
            data.specification_definitions.iter().any(|definition| {
                definition.measurement == name
                    && definition.define_measurement
                    && definition.expression == card
            })
        });
        assert!(defined, "the plan defines {name} as {card}");
    }

    /// The design variable `name` given `value` on Design variables: Add
    /// variable, its name and expression, a dimensionless quantity, and
    /// Create variable.
    pub(super) fn variable(&mut self, name: &str, value: &str) {
        self.setup_page("Design variables");
        self.press("Add variable\u{2026}", Role::Button);
        self.press("Name", Role::TextInput);
        self.type_text(name);
        self.press("Expression", Role::TextInput);
        self.type_text(value);
        self.press("Quantity", Role::ComboBox);
        self.press("Dimensionless", Role::ListBoxOption);
        self.press("Create variable", Role::Button);
        let model = &self.replay.app.state.model;
        let plan = model.sim_setup.analysis_plan.as_ref().expect("a plan").id();
        let created = model.workspace.content.plan_data(plan).is_some_and(|data| {
            data.design_variables
                .iter()
                .any(|variable| variable.name == name && variable.expression == value)
        });
        assert!(created, "{name} = {value} is a design variable");
    }

    /// The run's own connect rules on the Connect rules page: run_rules
    /// selected, which starts from the shipped default set, and each of
    /// `module`'s `cells` typed into its column.
    pub(super) fn run_connect_rules(&mut self, module: &str, cells: &[(&str, &str)]) {
        self.setup_page("Connect rules");
        self.press("Connect rules \u{b7} CONNECTRULES choices", Role::Button);
        self.press("run_rules", Role::ListBoxOption);
        for (column, text) in cells {
            self.field(&format!("{module} {column}"), text);
        }
    }

    /// Simulate > Setup > Models & sections > Add library > From file…, the
    /// picker answered with `file`. The library the file became, once the
    /// plan binds it, or what the page says instead.
    pub(super) fn add_model_file(&mut self, file: &Path) -> Result<String, String> {
        use rspice_app_services::io::file_exchange::{
            ScriptedChoice, script_next_choice, take_pickers_opened,
        };
        self.setup_page("Models & sections");
        let bound = |engineer: &Self| {
            engineer
                .replay
                .app
                .state
                .model
                .sim_setup
                .model_bindings
                .len()
        };
        let before = bound(self);
        take_pickers_opened();
        script_next_choice(ScriptedChoice::Chose(file.to_owned()));
        self.press("Add library\u{2026}", Role::Button);
        self.press("From file\u{2026}", Role::Button);
        assert_eq!(
            take_pickers_opened(),
            ["SPICE model source"],
            "From file\u{2026} asks for a model file"
        );
        let started = Instant::now();
        loop {
            let shown = self.step(Vec::new());
            // A label announces its words as its value.
            if let Some(refusal) = shown.iter().find(|control| {
                control.role == Role::Label && control.value.contains(" was not added: ")
            }) {
                return Err(refusal.value.clone());
            }
            if bound(self) > before {
                let binding = self
                    .replay
                    .app
                    .state
                    .model
                    .sim_setup
                    .model_bindings
                    .last()
                    .expect("the new binding");
                return Ok(binding.library_name.clone());
            }
            assert!(
                started.elapsed() < Duration::from_secs(60),
                "{} was neither added nor refused: {:?}",
                file.display(),
                self.shown
                    .iter()
                    .filter(|control| control.role == Role::Label)
                    .map(|control| control.value.as_str())
                    .collect::<Vec<_>>()
            );
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    /// The section of the one library the plan binds set to `section`, on
    /// its row of Models & sections.
    pub(super) fn model_section(&mut self, section: &str) {
        self.press("Corner section", Role::ComboBox);
        self.press(section, Role::ListBoxOption);
    }

    /// The plan's transient: Simulate > Setup > Analyses, its stop and
    /// step.
    pub(super) fn transient(&mut self, step: &str, stop: &str) {
        self.setup_page("Analyses");
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

    /// The transient started from the circuit's initial conditions rather
    /// than its operating point: Use initial conditions on Analyses.
    pub(super) fn initial_conditions(&mut self) {
        self.setup_page("Analyses");
        self.press("Use initial conditions", Role::CheckBox);
        let plan = self
            .replay
            .app
            .state
            .model
            .sim_setup
            .stable_analysis_plan()
            .expect("the plan resolves");
        let uic = plan.instances().iter().any(
            |instance| matches!(instance.draft(), AnalysisDraft::Transient(setup) if setup.uic),
        );
        assert!(uic, "the transient starts from its initial conditions");
    }

    /// Run, and the new run once it has ended.
    pub(super) fn run(&mut self) -> SimulationRun {
        let earlier = self
            .replay
            .app
            .state
            .model
            .simulation
            .retained
            .runs
            .first()
            .map(|run| run.run_id);
        self.key(Key::F5, Modifiers::NONE);
        let started = Instant::now();
        loop {
            self.step(Vec::new());
            let runs = &self.replay.app.state.model.simulation.retained.runs;
            if let Some(run) = runs.first()
                && Some(run.run_id) != earlier
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
    pub(super) fn measure(&mut self, expression: &str) -> f64 {
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
    pub(super) fn reopened(&mut self) -> RSpiceApp {
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

/// A file of its own under the system's scratch directory, removed with its
/// directory when dropped: a model file an engineer has on disk.
pub(super) struct Scratch(pub(super) PathBuf);

impl Scratch {
    pub(super) fn new(name: &str, text: &str) -> Self {
        let directory = std::env::temp_dir().join(format!(
            "rspice-engineer-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(&directory).expect("a scratch directory");
        let file = directory.join(name);
        std::fs::write(&file, text).expect("the file is written");
        Self(file)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if let Some(directory) = self.0.parent() {
            let _ = std::fs::remove_dir_all(directory);
        }
    }
}

/// The value the project keeps for the measurement `expression`.
pub(super) fn kept(app: &RSpiceApp, expression: &str) -> f64 {
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
