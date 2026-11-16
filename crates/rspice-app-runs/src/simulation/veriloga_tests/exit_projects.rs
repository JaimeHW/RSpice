//! The mixed-signal corpus designs drawn in the app as an engineer draws
//! them: HDL views compiled from the corpus sources, schematic sheets,
//! stimulus, analyses and measurements. Each project is saved, reopened and
//! run through the controller, and what it measures and saves is compared
//! with the corpus deck route at the corpus tolerances. Nothing here is a
//! recorded result: both routes run live.

use std::path::{Path, PathBuf};

use crate::simulation::controller::test_execution::{
    run_generated_batch_within, run_manual_batch_at_within,
};
use crate::simulation::controller::test_host::RunTestHost;
use crate::simulation::plan::connect_rules::ConnectRuleColumn;
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::state::{
    Cell, CellViewRef, ComponentType, Library, Point, ProjectSourceBundle, ProjectSourceLanguage,
    ProjectSourceOwner, SimulationRun, View, ViewType,
};
use rspice_app_state::workbench::app_state::model::{
    apply_compiled_interface, compile_project_bundle_receipt,
};
use rspice_design::schematic::net_label::NetLabelSide;
use rspice_design::schematic::rotation::Rotation;
use rspice_results::analysis_payload::AnalysisResultPayload;
use rspice_results::run::SimulationRunLifecycle;

mod c02;
mod c04;
mod c06;
mod c12;

/// A corpus file, by its path under `tests/mixed-corpus`.
fn corpus(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/mixed-corpus")
        .join(relative)
}

fn corpus_text(relative: &str) -> String {
    std::fs::read_to_string(corpus(relative)).unwrap_or_else(|error| panic!("{relative}: {error}"))
}

/// A project being drawn: its library, and where the next part lands.
struct Drawing {
    state: RunTestHost,
    library: &'static str,
    measurements: Vec<rspice_results::specification::SpecificationDefinition>,
}

/// How far a labelled stub runs out of a pin: one 2.5 mm grid step's worth
/// of wire, four sheet units each.
const STUB: i32 = 20;

/// How a part is turned and flipped as it is placed: the Rotate command's
/// quarter turns, clockwise on the sheet, after Flip horizontal and Flip
/// vertical.
#[derive(Debug, Clone, Copy)]
struct Orient(Rotation, bool, bool);

/// As the shelf places it.
const UPRIGHT: Orient = Orient(Rotation::R0, false, false);
/// A quarter turn: a horizontal two-terminal part stood upright, its first
/// pin on top.
const TURNED: Orient = Orient(Rotation::R90, false, false);
/// Half a turn: a source with its `+` at the bottom.
const HALF: Orient = Orient(Rotation::R180, false, false);
/// Three quarters: a source lying with its `+` on the left, a two-terminal
/// part stood with its first pin at the bottom.
const THREE: Orient = Orient(Rotation::R270, false, false);
/// Flipped left to right: a transistor gated from the right.
const MIRRORED: Orient = Orient(Rotation::R0, true, false);
/// Flipped top to bottom: a PMOS with its source up at the rail.
const FLIPPED: Orient = Orient(Rotation::R0, false, true);

/// Shorthand for a point on the sheet.
const fn at(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

/// Which way a pin's stub runs out of its part.
#[derive(Debug, Clone, Copy)]
enum Out {
    Left,
    Right,
    Up,
    Down,
}

impl Out {
    const fn step(self) -> (i32, i32) {
        match self {
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::Up => (0, -1),
            Self::Down => (0, 1),
        }
    }

    /// Where a label at the stub's end reads, clear of the stub.
    const fn side(self) -> NetLabelSide {
        match self {
            Self::Left => NetLabelSide::Left,
            Self::Right => NetLabelSide::Right,
            Self::Up => NetLabelSide::Above,
            Self::Down => NetLabelSide::Below,
        }
    }
}

impl Drawing {
    fn new(library: &'static str) -> Self {
        let mut state = RunTestHost::default();
        state
            .model
            .library_manager
            .add_library(Library::new(library));
        Self {
            state,
            library,
            measurements: Vec::new(),
        }
    }

    /// A cell whose `view_type` view owns the corpus source `file` as its
    /// project source, compiled, with the interface and the symbol Compile
    /// gives it.
    fn source_view(&mut self, cell: &str, view_type: ViewType, file: &str) -> CellViewRef {
        let reference = CellViewRef::new(self.library, cell, view_type.display_name());
        let mut view = View::new(view_type.display_name(), view_type);
        view.metadata
            .insert("veriloga.module".to_owned(), cell.to_owned());
        let mut library_cell = Cell::new(cell);
        library_cell.add_view(view);
        let model = &mut self.state.model;
        model
            .library_manager
            .get_library_mut(self.library)
            .expect("the library")
            .add_cell(library_cell);
        let root = Path::new(file)
            .file_name()
            .and_then(|name| name.to_str())
            .expect("a file name");
        let bundle = ProjectSourceBundle::try_new(
            ProjectSourceOwner::cell_view(reference.clone()),
            ProjectSourceLanguage::VerilogA,
            root,
            corpus_text(file),
            [],
            [],
        )
        .expect("a cell-view source");
        let sources = &mut model.workspace.content.project_sources;
        sources.insert_bundle(bundle).expect("one source per view");
        let bundle = sources
            .bundle_for_owner(&ProjectSourceOwner::cell_view(reference.clone()))
            .expect("the view's source");
        let receipt = compile_project_bundle_receipt(
            model.workspace.content.project.id(),
            bundle,
            Some(cell),
        )
        .unwrap_or_else(|diagnostics| panic!("{cell} compiles: {diagnostics:#?}"));
        let applied = apply_compiled_interface(
            &mut model.library_manager,
            &model.workspace.content.schematic_buffers,
            &reference,
            receipt.compilation.report(),
        );
        assert!(applied.wrote, "{cell}: {:?}", applied.outcome);
        reference
    }

    /// A device of `kind` named `name`, with its value and instance
    /// parameters, its origin at `at`, turned and flipped as `orient` says. Its pins, in terminal order: the
    /// sheet's wires and labels connect them.
    fn part(
        &mut self,
        kind: ComponentType,
        name: &str,
        value: &str,
        params: &str,
        at: (i32, i32),
        orient: Orient,
    ) -> Vec<Point> {
        let id = self
            .state
            .model
            .schematic
            .add_component(kind, Point::new(at.0, at.1));
        let component = self
            .state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("the device");
        component.name = name.to_owned();
        component.value = value.to_owned();
        component.params = params.to_owned();
        let Orient(rotation, mirror_h, mirror_v) = orient;
        component.rotation = rotation;
        component.mirror_h = mirror_h;
        component.mirror_v = mirror_v;
        rspice_design::schematic::component_edit::legacy_terminal_points(component)
    }

    /// A wire through `points`.
    fn wire(&mut self, points: &[Point]) {
        self.state
            .model
            .schematic
            .add_wire(points.to_vec())
            .expect("the wire commits");
    }

    /// The net's label at `at`, which stands on a pin or a wire's end, its
    /// name on `side`.
    fn name(&mut self, at: Point, net: &str, side: NetLabelSide) {
        let schematic = &mut self.state.model.schematic;
        let id = schematic.add_net_label(at, net.to_owned());
        schematic
            .document_mut_for_test()
            .net_labels
            .iter_mut()
            .find(|label| label.id == id)
            .expect("the label")
            .side = side;
    }

    /// A short stub out of `pin` towards `out`, its net's label at its end.
    fn tag(&mut self, pin: Point, out: Out, net: &str) {
        let (dx, dy) = out.step();
        let end = Point::new(pin.x + dx * STUB, pin.y + dy * STUB);
        self.wire(&[pin, end]);
        self.name(end, net, out.side());
    }

    /// A ground symbol on `pin`, drawn below it.
    fn earth(&mut self, pin: Point) {
        self.state
            .model
            .schematic
            .add_component(ComponentType::Ground, Point::new(pin.x, pin.y + 10));
    }

    /// A ground symbol on `pin`, turned to point left, away from a part on
    /// its right.
    fn earth_left(&mut self, pin: Point) {
        let schematic = &mut self.state.model.schematic;
        let id = schematic.add_component(ComponentType::Ground, Point::new(pin.x - 10, pin.y));
        schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("the ground")
            .rotation = Rotation::R90;
    }

    /// An instance `name` of the view `reference`, placed as the library
    /// browser places it with its origin at `at`; each pin `tags` names is
    /// wired out to its net's label. Every pin, by name.
    fn instance(
        &mut self,
        reference: &CellViewRef,
        name: &str,
        at: (i32, i32),
        tags: &[(&str, &str)],
    ) -> Vec<(String, Point)> {
        let model = &self.state.model;
        let candidate = crate::state::library_cell_placement_candidates(
            &model.library_manager,
            &model.workspace,
        )
        .into_iter()
        .find(|candidate| {
            candidate.library == reference.library
                && candidate.cell == reference.cell
                && candidate.view == reference.view
        })
        .unwrap_or_else(|| panic!("the browser offers {}", reference.display_path()));
        assert!(candidate.ready, "{name}: {}", candidate.unavailable_reason);
        self.place(candidate.binding, name, "", at, tags)
    }

    /// The built-in XSPICE code model the shelf lists as `stable_id`
    /// (Digital event-driven), placed from the shelf as `name` with the model
    /// parameters `params` at `at`; each pin `tags` names is wired out to its
    /// net's label. Every pin, by name.
    fn shelf_xspice(
        &mut self,
        stable_id: &str,
        name: &str,
        params: &str,
        at: (i32, i32),
        tags: &[(&str, &str)],
    ) -> Vec<(String, Point)> {
        use rspice_design::schematic::device_catalog::{
            builtin_xspice_library_binding, engine_only_xspice_devices,
        };
        let descriptor = engine_only_xspice_devices()
            .iter()
            .find(|descriptor| descriptor.stable_id == stable_id)
            .unwrap_or_else(|| panic!("the shelf lists {stable_id}"));
        let binding = builtin_xspice_library_binding(descriptor)
            .unwrap_or_else(|error| panic!("{stable_id}: {error}"));
        self.place(binding, name, params, at, tags)
    }

    /// Place `binding` as `name` with `params` at `at`. Each pin `tags`
    /// names is wired out to its net's label, as blocks are connected; the
    /// caller wires the rest. Every pin, by name.
    fn place(
        &mut self,
        binding: crate::state::LibraryCellInstance,
        name: &str,
        params: &str,
        at: (i32, i32),
        tags: &[(&str, &str)],
    ) -> Vec<(String, Point)> {
        let origin = Point::new(at.0, at.1);
        let model = &mut self.state.model;
        let id = model.schematic.add_library_cell_component(origin, binding);
        let component = model
            .schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("the instance");
        component.name = name.to_owned();
        component.params = params.to_owned();
        let component = component.clone();
        let order = &component
            .library_cell
            .as_ref()
            .expect("a cell instance")
            .terminal_order;
        // Where the pins stand on the symbol the sheet draws for the cell,
        // each under its port's name.
        let resolver = rspice_design::symbol_resolver::SymbolResolver::new(
            model.library_manager.catalog(),
            &model.workspace.content.schematic_buffers,
        );
        let pins = rspice_design::connectivity::terminal_positions(&component, &|binding| {
            resolver.resolve_binding(binding)
        })
        .into_iter()
        .enumerate()
        .map(|(index, (pin, at))| {
            // A symbol with numbered pins answers in the binding's terminal
            // order.
            let port = if order.iter().any(|port| port.eq_ignore_ascii_case(&pin)) {
                pin
            } else {
                order.get(index).cloned().unwrap_or(pin)
            };
            (port, at)
        })
        .collect::<Vec<_>>();
        // A pin on a side edge of the symbol is wired out sideways, away from
        // the body, and one on its top or bottom edge out vertically.
        for (pin, at) in &pins {
            let Some((_, net)) = tags.iter().find(|(port, _)| port.eq_ignore_ascii_case(pin))
            else {
                continue;
            };
            let out = if at.x == origin.x {
                if at.y < origin.y { Out::Up } else { Out::Down }
            } else if at.x < origin.x {
                Out::Left
            } else {
                Out::Right
            };
            self.tag(*at, out, net);
        }
        pins
    }

    /// The pin `name` of a placed block.
    fn pin_of(pins: &[(String, Point)], name: &str) -> Point {
        pins.iter()
            .find(|(pin, _)| pin.eq_ignore_ascii_case(name))
            .map(|(_, at)| *at)
            .unwrap_or_else(|| panic!("no pin {name}"))
    }

    /// An interface port `name` of the sheet being drawn, at `at` and turned
    /// as `orient` says. Its terminal: the caller wires it to its net.
    fn port(
        &mut self,
        name: &str,
        direction: crate::state::PortDirectionType,
        at: (i32, i32),
        orient: Orient,
    ) -> Point {
        let schematic = &mut self.state.model.schematic;
        let pending = crate::state::PendingPortPlacement::new(
            name,
            direction,
            crate::state::PortDiscipline::Electrical,
            schematic.topology_version(),
            schematic.next_interface_order(),
        );
        let id = schematic
            .place_pending_port(Point::new(at.0, at.1), pending)
            .unwrap_or_else(|error| panic!("port {name}: {error}"));
        let port = schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("the port");
        let Orient(rotation, mirror_h, mirror_v) = orient;
        port.rotation = rotation;
        port.mirror_h = mirror_h;
        port.mirror_v = mirror_v;
        rspice_design::schematic::component_edit::legacy_terminal_points(port)[0]
    }

    /// Put the sheet drawn so far on ISO A3 landscape paper, as Sheet
    /// format does on a sheet's first edit: the sheet becomes the
    /// schematic's own governed sheet, holding everything drawn on it.
    fn a3_sheet(&mut self) {
        use rspice_design_model::SchematicPageOrientation;
        use rspice_design_model::design_management::{
            AuthoredDrawingSheetSize, DrawingSheetInheritance, DrawingSheetStandard,
            DrawingSheetTitleFieldId,
        };
        let model = &mut self.state.model;
        let document = model.schematic.document();
        let mut objects = std::collections::BTreeSet::new();
        objects.extend(document.components.iter().map(|object| object.id));
        objects.extend(document.wires.iter().map(|object| object.id));
        objects.extend(document.junctions.iter().map(|object| object.id));
        objects.extend(document.net_labels.iter().map(|object| object.id));
        let owner = model.workspace.content.active_key();
        let drawn = model.active_drawing_sheet();
        let name = drawn.sheet_name;
        let format = drawn
            .format
            .try_update(|draft| {
                let title = draft
                    .title_block
                    .fields
                    .entry(DrawingSheetTitleFieldId::SheetTitle)
                    .or_default();
                if title.value.trim().is_empty() {
                    title.value.clone_from(&name);
                }
                draft.authored_size = AuthoredDrawingSheetSize::Standard {
                    standard: DrawingSheetStandard::IsoA3,
                };
                draft.orientation = SchematicPageOrientation::Landscape;
                draft.inheritance = DrawingSheetInheritance::Explicit;
            })
            .expect("A3 landscape");
        let mut candidate = (*model.workspace.content.design_management).clone();
        let sheet = candidate
            .bootstrap_for_cell_view(&owner, name, objects)
            .expect("the sheet is governed");
        let catalog = candidate.sheet_catalog_mut(&owner).expect("its catalog");
        let revision = catalog.find(sheet).expect("the sheet").revision();
        catalog
            .update_sheet_page_format(sheet, revision, format)
            .expect("A3 landscape");
        let schematics = model
            .prepare_design_management_schematic_transaction(&candidate)
            .expect("the sheet's schematic transaction");
        model
            .workspace
            .content
            .replace_design_management(candidate)
            .expect("the catalog commits");
        model.apply_design_management_schematic_transaction(&schematics);
    }

    /// Make the sheet drawn so far the schematic of the cell `cell`, draw
    /// its symbol from its ports as Generate symbol does, and start the
    /// next sheet.
    fn master(&mut self, cell: &str) -> CellViewRef {
        let reference = CellViewRef::new(self.library, cell, "schematic");
        let mut library_cell = Cell::new(cell);
        library_cell.add_view(View::new("schematic", ViewType::Schematic));
        library_cell.add_view(View::new("symbol", ViewType::Symbol));
        let model = &mut self.state.model;
        model
            .library_manager
            .get_library_mut(self.library)
            .expect("the library")
            .add_cell(library_cell);
        let master = std::mem::take(&mut model.schematic);
        let ports = master.document().interface_ports();
        model
            .workspace
            .insert_schematic_editor(reference.key(), master);
        let symbol = CellViewRef::new(self.library, cell, "symbol");
        let document = model.generated_symbol_document(&symbol, &ports);
        model
            .library_manager
            .edit_library(self.library)
            .and_then(|mut library| library.store_symbol_document(cell, "symbol", &document))
            .expect("the symbol view")
            .expect("the symbol stores");
        reference
    }

    /// Import the model file `file` as Models > Add library does, and
    /// attach it to the plan as the Simulate page's model libraries do.
    fn model_library(&mut self, file: &str) {
        let model = &mut self.state.model;
        let library = model
            .model_library_manager
            .load_library_file(corpus(file), None)
            .unwrap_or_else(|error| panic!("{file}: {error}"));
        self.attach_model_library(&library);
    }

    /// Attach the model library `library` to the plan, as the Simulate
    /// page's model libraries do.
    fn attach_model_library(&mut self, library: &str) {
        let model = &mut self.state.model;
        let binding = model
            .model_library_manager
            .simulation_plan_binding(library)
            .expect("an imported library binds to a plan");
        let mut candidate = model.sim_setup.clone();
        candidate.model_bindings.push(binding);
        model
            .model_library_manager
            .validate_simulation_plan_bindings(&candidate.model_bindings)
            .expect("the binding validates");
        candidate
            .commit_active_plan_configuration_change(format!("Attached model library {library}"))
            .expect("the plan takes the binding");
        model.sim_setup = candidate;
    }

    /// A model card made in the Models workspace as a project model of the
    /// library `library`, attached to the plan.
    fn project_model(&mut self, library: &str, name: &str, kind: &str, parameters: &[(&str, f64)]) {
        let definition = rspice_model_library::ProjectModelDefinition {
            name: name.to_owned(),
            spice_type: kind.to_owned(),
            description: String::new(),
            numeric_parameters: parameters
                .iter()
                .map(|(parameter, value)| ((*parameter).to_owned(), *value))
                .collect(),
            string_parameters: Default::default(),
        };
        let model = &mut self.state.model;
        model
            .model_library_manager
            .create_project_model(library, &definition)
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        self.attach_model_library(library);
    }

    /// Type `cells` into the run's own connect rules on the Connect rules
    /// page: select run_rules, which starts from the shipped default set, and
    /// commit each cell of its grid as the page does.
    fn run_connect_rules(&mut self, cells: &[(&str, ConnectRuleColumn, &str)]) {
        use crate::simulation::plan::OptionsDialogState;
        use crate::simulation::plan::connect_rules;
        use rspice_simulation_contract::options::RunConnectRules;
        let model = &mut self.state.model;
        let draft = &mut model.sim_setup.session.options_draft;
        draft.connect_rules = RunConnectRules::DEFAULT_NAME.to_owned();
        let block = draft
            .connect_rule_block
            .get_or_insert_with(|| connect_rules::started_from(connect_rules::DEFAULT_BASE));
        for (module, column, text) in cells {
            connect_rules::set_cell(block, module, *column, text)
                .unwrap_or_else(|error| panic!("{module} {column:?} = {text}: {error}"));
        }
        let options = draft.to_options().expect("the draft reads");
        options.validate().expect("the options validate");
        let mut candidate = model.sim_setup.clone();
        candidate.commit_options(&options);
        candidate.session.options_draft = OptionsDialogState::from_options(&options);
        candidate
            .commit_active_plan_configuration_change("Updated simulation solver options.")
            .expect("the plan takes the options");
        model.sim_setup = candidate;
    }

    /// A design variable the netlist states as `.param name=value`.
    fn variable(&mut self, name: &str, value: &str) {
        use rspice_simulation_contract::design_variable::{
            DesignVariable, DesignVariableOverridePolicy, DesignVariableScope,
            DesignVariableSweepEligibility,
        };
        use rspice_simulation_contract::design_variable_quantity::DesignVariableQuantity;
        let plan = self.plan_id();
        self.state
            .model
            .workspace
            .content
            .ensure_active_plan_data(plan)
            .design_variables
            .push(
                DesignVariable::new(
                    name,
                    value,
                    DesignVariableQuantity::Dimensionless,
                    DesignVariableScope::Testbench,
                    "",
                    None,
                    DesignVariableSweepEligibility::FixedParameter,
                    DesignVariableOverridePolicy::InheritOwnerOnly,
                )
                .expect("a design variable"),
            );
    }

    fn plan_id(&self) -> rspice_app_state::product::SimulationPlanId {
        self.state
            .model
            .sim_setup
            .analysis_plan
            .as_ref()
            .expect("a plan")
            .id()
    }

    /// The plan's one analysis: a transient to `stop` at `step`, every
    /// quantity saved.
    fn transient(&mut self, step: &str, stop: &str) {
        self.transient_from(step, stop, false);
    }

    /// [`Self::transient`], starting from the circuit's initial conditions
    /// instead of its operating point when `uic` is set.
    fn transient_from(&mut self, step: &str, stop: &str, uic: bool) {
        self.state.model.sim_setup.run_set =
            rspice_simulation_contract::run_set::RunSetState::reference_only();
        let plan = self
            .state
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
            draft.step = step.into();
            draft.max_step = String::new();
            draft.uic = uic;
        })
        .expect("the transient");
        self.state.model.sim_setup.save_policy.output_selection_mode =
            crate::state::OutputSelectionMode::SaveAll;
    }

    /// A measurement the plan defines with its own `.MEAS` card.
    fn measure(&mut self, name: &str, card: &str) {
        use rspice_results::specification::{SpecEntry, SpecPointScope, SpecificationDefinition};
        let mut definition = SpecificationDefinition::new_from_projection(&SpecEntry {
            measurement: name.into(),
            expression: card.into(),
            min: None,
            max: None,
            unit: String::new(),
            scope: SpecPointScope::AllPoints,
        });
        definition.define_measurement = true;
        self.measurements.push(definition);
    }

    fn finish(mut self) -> RunTestHost {
        let plan = self.plan_id();
        self.state
            .model
            .workspace
            .content
            .replace_active_specification_definitions(plan, std::mem::take(&mut self.measurements));
        self.state.sync_active_schematic_to_workspace();
        self.state
    }
}

/// Save `state` as a project file, as File > Save writes it, then open that
/// file into a fresh session, as File > Open reads it.
fn save_and_reopen(state: &RunTestHost, name: &str) -> RunTestHost {
    let directory = std::env::temp_dir().join(format!("rspice-exit-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("a scratch directory");
    let path = directory.join(format!("{name}.rspice"));
    save(state, &path);
    // Closed: nothing of the drawing session survives but the file.
    let reopened = reopen(&path);
    let _ = std::fs::remove_dir_all(&directory);
    reopened
}

/// Write `state` to `path` as File > Save writes a project.
fn save(state: &RunTestHost, path: &Path) {
    use rspice_app_services::workbench::lifecycle::project_lifecycle::{
        LifecycleView, ResultPresentationSource, snapshot,
    };
    struct NoPresentation;
    impl ResultPresentationSource for NoPresentation {
        fn project_presentation(
            &self,
            _: &crate::state::SimulationState,
        ) -> crate::state::result_presentation::ResultPresentation {
            crate::state::result_presentation::ResultPresentation {
                marker_id_high_water: Some(0),
                ..Default::default()
            }
        }
    }
    let platform = rspice_app_services::workbench::slots::platform::PlatformState::default();
    let saved = snapshot(&LifecycleView {
        model: &state.model,
        platform: &platform,
        workspace: rspice_app_state::workbench::workspace::Workspace::Design,
        presentation: &NoPresentation,
    })
    .expect("the project saves");
    let text = rspice_app_services::io::project_io::serialize_project_file(&saved)
        .expect("the project serializes");
    std::fs::write(path, &text).expect("the project is written");
}

/// Open the project file at `path` into a fresh session, as File > Open
/// reads one: its libraries, workspace, plan and model catalog.
fn reopen(path: &Path) -> RunTestHost {
    let mut project =
        rspice_app_services::io::project_io::load_project_file(path).expect("the project opens");
    let context = project
        .file
        .execution_context
        .take()
        .expect("the project saved its simulation setup");
    let (sim_setup, model_library_manager, warnings) =
        rspice_app_services::io::restore_execution_context(context)
            .expect("the simulation setup restores");
    assert!(warnings.is_empty(), "{warnings:?}");
    project.ensure_library_model();
    let mut reopened = RunTestHost::default();
    let model = &mut reopened.model;
    model.library_manager = project.file.libraries;
    model.workspace = crate::state::ProjectWorkspace::from_parts(
        project.file.workspace,
        project.workspace_session,
    );
    model.sim_setup = sim_setup;
    model.model_library_manager = model_library_manager;
    let key = model.workspace.content.active_key();
    model.schematic = model
        .workspace
        .remove_schematic_editor(&key)
        .expect("the active sheet reopens");
    reopened
}

/// A run's whole published result, as a digest: every trace's samples, every
/// committed event, every bus and every measurement, bit for bit.
fn digest(run: &SimulationRun) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for analysis in &run.analyses {
        for wave in &analysis.waveforms {
            wave.name.hash(&mut hasher);
            for value in wave.x.iter().chain(wave.y.iter()) {
                value.to_bits().hash(&mut hasher);
            }
        }
        if let Some(AnalysisResultPayload::TransientEvents {
            digital_traces,
            digital_buses,
            ..
        }) = analysis.result_payload.as_ref()
        {
            for trace in digital_traces {
                trace.node_name.hash(&mut hasher);
                for point in &trace.points {
                    point.time_s.to_bits().hash(&mut hasher);
                    point.value_code.hash(&mut hasher);
                }
            }
            format!("{digital_buses:?}").hash(&mut hasher);
        }
        format!("{:?}", analysis.measurements).hash(&mut hasher);
    }
    hasher.finish()
}

/// The committed events of the digital net `name` in a transient run:
/// `(time, value code)` in order.
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

/// An analog trace's sample held at `time`: its last sample at or before it.
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

/// The value the run measured as `name`.
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
/// to `stop`, as the deck route runs it.
fn deck_route(relative: &str, tran: (&str, &str)) -> SimulationRun {
    let path = corpus(relative);
    let mut source = String::new();
    for line in corpus_text(relative).lines() {
        if line.trim_start().to_ascii_lowercase().starts_with(".tran ") {
            source.push_str(&format!(".tran {} {}\n", tran.0, tran.1));
        } else {
            source.push_str(line);
            source.push('\n');
        }
    }
    run_manual_batch_at_within(&source, &path, SimulationRunLifecycle::Completed, RUN_LIMIT)
}

/// Run a project through the controller, as Run does.
fn design_route(state: RunTestHost) -> SimulationRun {
    run_generated_batch_within(state, SimulationRunLifecycle::Completed, RUN_LIMIT)
}

/// How long one exit run may take before it is taken to have hung: a whole
/// corpus transient in an unoptimized build.
const RUN_LIMIT: std::time::Duration = std::time::Duration::from_secs(20 * 60);
