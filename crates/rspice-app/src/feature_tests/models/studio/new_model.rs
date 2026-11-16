//! Models › New model on the running application: the device type picked
//! first, the card created with that type's parameters at the engine's own
//! defaults and opened in the model editor, bound to a placed transistor
//! from the Library manager, run, and kept by a save.

use super::*;
use crate::feature_tests::replay::Replay;
use crate::workbench::app_state::AppState;
use egui::accesskit::Role;
use rspice_app_models::workbench::models::workspace_state::ModelsWorkbenchDialog;
use rspice_app_state::state::Wire;

/// The CMOS inverter example cut to its pull-down: MN's drain on the 3.3 V
/// supply, its gate held at 1 V by VIN, its source and bulk on ground, with
/// an operating point. MN is selected, as a designer about to bind it has it.
fn pull_down() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    rspice_app_state::workbench::examples::load_example(
        "CMOS Inverter",
        &mut app.state.model.schematic,
    );
    let doc = app.state.model.schematic.document_mut_for_test();
    doc.components.retain(|component| component.name != "MP");
    // MP's source and drain pins, now one wire: the output is the supply.
    let id = doc.wires.iter().map(|wire| wire.id).max().unwrap_or(0) + 1;
    doc.wires.push(Wire::new(
        id,
        vec![Point::new(260, 80), Point::new(260, 160)],
    ));
    let vin = doc
        .components
        .iter_mut()
        .find(|component| component.name == "VIN")
        .expect("the input source");
    vin.kind = ComponentType::VoltageSource;
    vin.value = "1".to_owned();
    vin.params.clear();
    let mn = doc
        .components
        .iter()
        .find(|component| component.name == "MN")
        .expect("the pull-down")
        .id;
    app.state.sync_active_schematic_to_workspace();
    let plan = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("a default state owns a stable plan");
    let position = plan.instances().len();
    plan.insert_draft_with_id(
        rspice_app_state::product::AnalysisInstanceId::new(),
        rspice_app_state::simulation::plan::AnalysisDraft::for_kind(
            rspice_app_state::simulation::plan::AnalysisKind::OperatingPoint,
        ),
        true,
        position,
    )
    .expect("an operating point has no prerequisites");
    app.state.model.model_library_manager.clear();
    app.state.models.models_page = ModelsPage::Models;
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(mn);
    app
}

/// Models › New model, from the command palette as a hand runs it: pick
/// `device` from the Device type list, name the card `name`, press Create.
fn create_model(replay: &mut Replay, device: &str, name: &str) {
    replay.choose_command("New model");
    assert!(
        matches!(
            replay.app.state.models.models_view.dialog,
            Some(ModelsWorkbenchDialog::NewModel { .. })
        ),
        "New model opened no dialog"
    );
    replay.click("Device type", Role::ComboBox);
    replay.click(device, Role::ListBoxOption);
    replay.click("Model name", Role::TextInput);
    replay.type_text(name);
    replay.click("Create", Role::Button);
    assert!(
        replay.app.state.models.models_view.dialog.is_none(),
        "Create left the dialog open"
    );
}

/// Run the active plan as the Run command does, through its checks, to the end.
fn run_the_active_plan(app: &mut RSpiceApp) {
    app.simulation_controller
        .update(&mut app.state, &*app.export_workflow_io);
    rspice_app_runs::workbench::preflight::start(
        &mut app.state,
        &mut app.simulation_controller,
        true,
    );
    app.settle_preflight();
    let blockers = app
        .state
        .runs
        .preflight
        .report
        .as_ref()
        .map(|report| report.blockers.clone())
        .unwrap_or_default();
    assert!(blockers.is_empty(), "the run is blocked: {blockers:#?}");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    loop {
        app.simulation_controller
            .update(&mut app.state, &*app.export_workflow_io);
        let execution = &app.state.model.simulation.execution;
        if !app.simulation_controller.is_running()
            && !execution.has_active_execution()
            && !execution.trigger_simulation
        {
            return;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "the run did not finish"
        );
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

/// The drain current a run's operating point drew from the supply `VDD`.
fn supply_current(run: &rspice_app_state::state::SimulationRun) -> f64 {
    assert!(run.success, "{:?}", run.analyses);
    let op = run
        .analyses
        .iter()
        .find_map(|analysis| analysis.dc_op.as_ref())
        .expect("the run retained its operating point");
    op.branch_currents
        .iter()
        .find(|value| value.name.eq_ignore_ascii_case("I(VDD)"))
        .unwrap_or_else(|| panic!("no VDD current in {:?}", op.branch_currents))
        .value
        .abs()
}

/// The pull-down written by hand as a CLI deck, its transistor on `card`.
fn hand_deck_current(card: &str) -> f64 {
    let folder = std::env::temp_dir().join(format!("rspice-new-model-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&folder).expect("a scratch folder");
    let path = folder.join("pull_down.spice");
    let deck = format!(
        "pull-down written by hand\nVDD vdd 0 DC 3.3\nVIN g 0 DC 1\n\
         MN vdd g 0 0 hand w=1u l=0.18u\n{card}\n.op\n.end\n"
    );
    let run = rspice_app_runs::simulation::controller::test_execution::run_manual_batch_at(
        &deck,
        &path,
        rspice_results::run::SimulationRunLifecycle::Completed,
    );
    let _ = std::fs::remove_dir_all(&folder);
    let _ = rspice_app_runs::simulation::controller::prepared_run::thread_preparations::take();
    supply_current(&run)
}

/// Save the project to a scratch file and open it again in a fresh state.
fn saved_and_reopened(app: &mut RSpiceApp) -> AppState {
    use rspice_app_services::workbench::lifecycle::project_lifecycle::{
        DestinationAuthority, SaveScope, save_native,
    };
    let path = std::env::temp_dir().join(format!(
        "rspice-new-model-{}.rspiceproj",
        uuid::Uuid::new_v4()
    ));
    save_native(
        &mut app.state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("the project saves");
    let mut reopened = AppState::default();
    let opened = reopened.load_project_from_path(&path);
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
    let mut lock = path.as_os_str().to_os_string();
    lock.push(".rspice.lock");
    let _ = std::fs::remove_file(PathBuf::from(lock));
    assert!(opened, "the saved project reopens");
    reopened
}

/// A level-1 NMOS made through New model is the engine's own level-1
/// device: the editor opens on every parameter of the type at its default,
/// the Library manager binds it to the placed transistor, the run draws
/// the square-law current of those defaults, and the same card written by
/// hand in a deck draws the same. A save keeps the card and the binding.
#[test]
fn a_new_level_one_nmos_is_created_bound_run_and_kept() {
    let mut replay = Replay::of(pull_down());
    let revision = replay.app.state.model.workspace.content.project.revision();
    create_model(&mut replay, "NMOS · N-channel MOSFET (level 1)", "nnew");

    let draft = replay
        .app
        .state
        .models
        .model_editor
        .draft
        .as_ref()
        .expect("Create opens the card in the model editor");
    assert_eq!(
        (draft.library_name.as_str(), draft.model_name.as_str()),
        ("nnew project", "nnew")
    );
    assert_eq!(draft.spice_type, "NMOS");
    let defaults = rspice_core::library::device_model_defaults("NMOS").expect("the NMOS type");
    let value = |name: &str| {
        draft
            .parameters
            .iter()
            .find(|parameter| parameter.name == name)
            .unwrap_or_else(|| panic!("the card has no {name}"))
    };
    assert_eq!(value("LEVEL").value, "1");
    for parameter in defaults.parameters {
        let row = value(parameter.name);
        assert_eq!(
            row.value.parse::<f64>().ok(),
            Some(parameter.default),
            "{}",
            parameter.name
        );
        assert_eq!(row.unit, parameter.unit, "{}", parameter.name);
        assert_eq!(row.description, parameter.description, "{}", parameter.name);
    }
    assert_eq!(draft.parameters.len(), defaults.parameters.len() + 1);
    assert!(
        replay.app.state.model.workspace.content.project.revision() > revision,
        "the card is published as a project revision"
    );

    replay.choose_command("Library manager");
    replay.click("Bind to MN…", Role::Button);
    // The binding names the card and the library that holds it.
    let bound = |state: &AppState| {
        let params = &state
            .model
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.name == "MN")
            .expect("the pull-down")
            .params;
        let params = rspice_app_state::state::parse_params_string(params);
        (
            params.get("model").cloned().unwrap_or_default(),
            params.get("model_library").cloned().unwrap_or_default(),
        )
    };
    assert_eq!(
        bound(&replay.app.state),
        ("nnew".to_owned(), "nnew project".to_owned())
    );

    run_the_active_plan(&mut replay.app);
    let run = replay
        .app
        .state
        .model
        .simulation
        .active_run()
        .expect("a run");
    let drawn = supply_current(run);
    // Square law at the type's defaults: KP 20 µA/V², VTO 0, LAMBDA 0, no
    // LD, so ID = KP/2 · W/L · VGS² with W/L = 1u/0.18u and VGS = 1 V. The
    // supply also feeds the reverse-biased drain junction's GMIN, picoamps.
    let square_law = 2e-5 / 2.0 * (1e-6 / 0.18e-6);
    assert!(
        ((drawn - square_law) / square_law).abs() < 1e-6,
        "{drawn} A against the square law's {square_law} A"
    );
    let by_hand = hand_deck_current(".model hand NMOS (LEVEL=1)");
    assert!(
        ((drawn - by_hand) / by_hand).abs() < 1e-12,
        "{drawn} A against the hand-written deck's {by_hand} A"
    );

    let reopened = saved_and_reopened(&mut replay.app);
    assert_eq!(
        bound(&reopened),
        ("nnew".to_owned(), "nnew project".to_owned())
    );
    let library = reopened
        .model
        .model_library_manager
        .get_library("nnew project")
        .expect("the saved project keeps the new model's library");
    let model = library.get_model("nnew").expect("and its card");
    assert_eq!(model.parameters.get("kp").copied(), Some(2e-5));
}
