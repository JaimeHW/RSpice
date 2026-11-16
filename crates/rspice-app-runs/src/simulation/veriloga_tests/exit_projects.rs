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
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::state::{
    Cell, CellViewRef, ComponentType, Library, LibraryCellInstance, Point, ProjectSourceBundle,
    ProjectSourceLanguage, ProjectSourceOwner, SimulationRun, View, ViewType,
};
use rspice_app_state::workbench::app_state::model::{
    apply_compiled_interface, compile_project_bundle_receipt,
};
use rspice_results::analysis_payload::AnalysisResultPayload;
use rspice_results::run::SimulationRunLifecycle;

mod c04;

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
    slot: i32,
    measurements: Vec<rspice_results::specification::SpecificationDefinition>,
}

/// Grid pitch between parts on a sheet: clear of every two- and
/// four-terminal body and its labelled stubs.
const PITCH: i32 = 400;

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
            slot: 0,
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

    /// The next free place on the sheet.
    fn next_origin(&mut self) -> Point {
        let slot = self.slot;
        self.slot += 1;
        Point::new((slot % 12) * PITCH, (slot / 12) * PITCH)
    }

    /// Wire each terminal out from its body's edge to a stub carrying its
    /// net's label: left from a pin on the left edge, up from one on the
    /// top, and so on, so no stub meets another pin.
    fn label(&mut self, terminals: Vec<(Point, &str)>) {
        let xs = || terminals.iter().map(|(at, _)| at.x);
        let ys = || terminals.iter().map(|(at, _)| at.y);
        let (left, right) = (xs().min().unwrap_or(0), xs().max().unwrap_or(0));
        let (top, bottom) = (ys().min().unwrap_or(0), ys().max().unwrap_or(0));
        let centre = Point::new((left + right) / 2, (top + bottom) / 2);
        for (at, net) in &terminals {
            let (dx, dy) = if left == right {
                (0, (at.y - centre.y).signum())
            } else if top == bottom || at.x == left || at.x == right {
                ((at.x - centre.x).signum(), 0)
            } else {
                (0, (at.y - centre.y).signum())
            };
            let out = Point::new(at.x + dx * 20, at.y + dy * 20);
            let schematic = &mut self.state.model.schematic;
            schematic
                .add_wire(vec![*at, out])
                .expect("the stub commits");
            schematic.add_net_label(out, (*net).to_owned());
        }
    }

    /// A device of `kind` named `name`, with its value and instance
    /// parameters, its terminals on `nets` in order.
    fn part(&mut self, kind: ComponentType, name: &str, value: &str, params: &str, nets: &[&str]) {
        let origin = self.next_origin();
        let id = self.state.model.schematic.add_component(kind, origin);
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
        let terminals = rspice_design::schematic::component_edit::legacy_terminal_points(component);
        assert_eq!(terminals.len(), nets.len(), "{name}: one net per terminal");
        self.label(terminals.into_iter().zip(nets.iter().copied()).collect());
    }

    /// The sheet's ground symbol, on the net `0`.
    fn ground(&mut self) {
        self.part(ComponentType::Ground, "GND", "", "", &["0"]);
    }

    /// An instance `name` of the view `reference`, each port on the net
    /// `nets` gives it, in a row of its own below the devices.
    fn instance(&mut self, reference: &CellViewRef, name: &str, row: i32, nets: &[(&str, &str)]) {
        let ports = rspice_design::library::metadata::metadata_ports(
            &self
                .state
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
        let origin = Point::new(row * 1200, -1600);
        let id = self
            .state
            .model
            .schematic
            .add_library_cell_component(origin, binding);
        let component = self
            .state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("the instance");
        component.name = name.to_owned();
        let order = component
            .library_cell
            .as_ref()
            .expect("a cell instance")
            .terminal_order
            .clone();
        let terminals = order
            .iter()
            .zip(component.terminal_positions())
            .map(|(port, (_, at))| {
                let net = nets
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case(port))
                    .unwrap_or_else(|| panic!("{}: no net for {port}", component.name))
                    .1;
                (at, net)
            })
            .collect();
        self.label(terminals);
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
