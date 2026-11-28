//! The toolbar's run chip: what Run runs, and the popover that picks it.
//!
//! The chip raises the plan manager through the simulate feature's command
//! and owns no dialog of its own. Its popover picks the active plan, which of
//! its analyses Run includes, and the reference PVT a plain Run runs at, each
//! through the path the Simulate pages take. The tests drive it as a reader
//! does, frame by frame, and read what it did from the store.

use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_flow_tests::User;
use rspice_app_simulate::workbench::simulate::commands::SimulateCommand;
use rspice_app_simulate::workbench::surfaces::simulate::page_models::{
    ModelBindingAction, apply_model_binding_action,
};
use rspice_app_state::product::{AnalysisInstanceId, ProcessCorner};
use rspice_app_state::state::SimulationPage;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_simulation_contract::analysis_kind::AnalysisKind;

/// The chip's name ends so, whatever it names.
const CHIP_TAIL: &str = "; choose the plan, the analyses and the reference PVT Run runs";
const TRANSIENT: &str = "Transient \u{b7} 1 ms";
const TEMPERATURE_FIELD: &str = "Reference temperature in \u{b0}C";

/// The chip's name in the last frame, and where it was drawn.
fn chip(user: &User) -> (String, egui::Rect) {
    user.named
        .iter()
        .find(|(name, _)| name.starts_with("Analysis ") && name.ends_with(CHIP_TAIL))
        .map(|(name, rect)| (name.clone(), *rect))
        .unwrap_or_else(|| panic!("the toolbar draws the run chip: {:?}", user.named))
}

/// A press on the chip, which opens its popover or closes it.
fn press_chip(user: &mut User) {
    let (_, at) = chip(user);
    user.press(at.center(), egui::PointerButton::Primary);
    user.settle();
}

/// Where the last frame drew `name` in the popover: between its first
/// head's ends and under it. `None` while it is closed. The navigator's own
/// Add analysis… and the Analyses page's stand elsewhere.
fn in_popover(user: &User, name: &str) -> Option<egui::Rect> {
    let head = user
        .named
        .iter()
        .find(|(candidate, _)| candidate == "Table columns: Plan")?
        .1;
    user.named
        .iter()
        .filter(|(candidate, rect)| {
            candidate == name
                && rect.top() >= head.top()
                && rect.left() >= head.left() - 1.0
                && rect.right() <= head.right() + 1.0
        })
        .map(|(_, rect)| *rect)
        .next()
}

/// Where the last frame drew the one thing whose name starts with `prefix`.
fn starting(user: &User, prefix: &str) -> egui::Rect {
    let found: Vec<_> = user
        .named
        .iter()
        .filter(|(name, _)| name.starts_with(prefix))
        .collect();
    assert_eq!(found.len(), 1, "one {prefix:?}: {:?}", user.named);
    found[0].1
}

fn click_at(user: &mut User, rect: egui::Rect) {
    user.press(rect.center(), egui::PointerButton::Primary);
    user.settle();
}

/// A click on what the popover names `name`.
fn click_in_popover(user: &mut User, name: &str) {
    let rect = in_popover(user, name)
        .unwrap_or_else(|| panic!("the popover shows {name:?}: {:?}", user.named));
    click_at(user, rect);
}

fn key(user: &mut User, key: egui::Key, modifiers: egui::Modifiers) {
    user.frame(vec![egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers,
    }]);
    user.settle();
}

/// Each analysis of the active plan: its identity, what it is, and whether
/// Run includes it.
fn analyses(app: &RSpiceApp) -> Vec<(AnalysisInstanceId, String, bool)> {
    app.state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("an active plan")
        .instances()
        .iter()
        .map(|instance| {
            (
                instance.id(),
                format!("{:?}", instance.draft()),
                instance.enabled(),
            )
        })
        .collect()
}

/// A plan of three analyses: the 1 ms transient, then an AC sweep and the
/// operating point it linearises about.
fn three_analyses() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let plan = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("the default plan");
    plan.insert(AnalysisKind::OperatingPoint)
        .expect("an operating point");
    plan.insert(AnalysisKind::Ac).expect("an AC sweep");
    app
}

/// A project whose active plan, Bench, runs no analysis.
fn empty_plan() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let setup = &mut app.state.model.sim_setup;
    setup.create_plan("Bench").expect("a second plan");
    let plan = setup.stable_analysis_plan_mut().expect("the new plan");
    let ids: Vec<AnalysisInstanceId> = plan
        .instances()
        .iter()
        .map(|instance| instance.id())
        .collect();
    for id in ids {
        plan.remove(id, Vec::new()).expect("nothing refers to it");
    }
    app
}

/// The prompt the status line holds.
fn prompt(user: &User) -> String {
    user.app
        .state
        .workbench
        .prompt
        .as_ref()
        .map(|prompt| prompt.text.trim_end_matches('\u{a0}').to_owned())
        .unwrap_or_default()
}

/// Type `text` into the reference temperature's field, then Enter.
fn type_temperature(user: &mut User, text: &str) {
    user.focus_on(TEMPERATURE_FIELD);
    user.frame(vec![
        egui::Event::Key {
            key: egui::Key::A,
            physical_key: Some(egui::Key::A),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND,
        },
        egui::Event::Text(text.to_owned()),
    ]);
    key(user, egui::Key::Enter, egui::Modifiers::NONE);
}

/// The chip's line and the Run control name the reference alike, as
/// `reference`: the process only where the reference offers one.
fn assert_names_reference(user: &User, reference: &str) {
    assert!(
        chip(user).0.contains(&format!(", {reference};")),
        "{}",
        chip(user).0
    );
    let run = format!("Run active simulation plan at {reference} (F5)");
    assert!(
        user.named.iter().any(|(name, _)| *name == run),
        "{run}: {:?}",
        user.named
            .iter()
            .map(|(name, _)| name)
            .filter(|name| name.starts_with("Run "))
            .collect::<Vec<_>>()
    );
}

/// The model libraries the project holds, none left.
fn without_libraries(app: &mut RSpiceApp) {
    let manager = &mut app.state.model.model_library_manager;
    let names: Vec<String> = manager
        .catalog()
        .libraries()
        .map(|library| library.name.clone())
        .collect();
    for name in names {
        manager.remove_library(&name);
    }
}

/// A library whose `.lib` sections are the typical and slow corners and one
/// whose name is none of the five a reference can name, bound to the active
/// plan as the Models page binds it: at the section it pins, typical.
fn bind_corner_library(app: &mut RSpiceApp) {
    let name = app
        .state
        .model
        .model_library_manager
        .load_library_bytes(
            "foundry-corners.lib",
            b".lib tt\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl tt\n\
              .lib ss\n.model nch NMOS (LEVEL=1 KP=8e-4)\n.endl ss\n\
              .lib hot_typical\n.model nch NMOS (LEVEL=1 KP=9e-4)\n.endl hot_typical\n"
                .to_vec(),
            None,
        )
        .expect("a sectioned library imports");
    apply_model_binding_action(
        &mut crate::workbench::features::cx(app),
        ModelBindingAction::Attach(name),
    );
}

/// The active plan's first library left to the reference process, as the
/// Models page's Automatic choice leaves it.
fn follow_the_reference(app: &mut RSpiceApp) {
    apply_model_binding_action(
        &mut crate::workbench::features::cx(app),
        ModelBindingAction::SetCorner {
            index: 0,
            corner: None,
        },
    );
}

/// The chip's whole purpose is the plan manager. Navigating to Simulation
/// Studio instead left the manager reachable only from a button inside it.
#[test]
fn the_plan_chip_owns_no_dialog_construction_of_its_own() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.workspace = Workspace::Design;

    app.execute_command_key(SimulateCommand::ManagePlans.key());

    assert!(matches!(
        app.state.simulate.simulation_workflow,
        Some(rspice_app_simulate::workbench::state::workflow_drafts::SimulationWorkflowDialog::PlanManager(_))
    ));
    assert_ne!(app.state.workbench.workspace, Workspace::Simulate);
}

/// A plan picked in the popover is the one Run runs, and the chip names it.
#[test]
fn picking_another_plan_makes_it_the_one_run_runs() {
    let mut app = RSpiceApp::test_instance();
    let setup = &mut app.state.model.sim_setup;
    let first = setup.stable_analysis_plan().expect("default plan").id();
    let first_name = setup.active_plan_name().to_string();
    setup.create_plan("Corner sweep").expect("a second plan");
    let mut user = User::new(app);
    assert!(
        chip(&user).0.contains(", plan Corner sweep, "),
        "{}",
        chip(&user).0
    );

    press_chip(&mut user);
    user.expect("Plan: Corner sweep, Active");
    click_in_popover(&mut user, &format!("Plan: {first_name}"));

    let setup = &user.app.state.model.sim_setup;
    assert_eq!(
        setup.stable_analysis_plan().map(|plan| plan.id()).ok(),
        Some(first)
    );
    assert!(
        chip(&user).0.contains(&format!(", plan {first_name}, ")),
        "{}",
        chip(&user).0
    );
    user.expect(&format!("Plan: {first_name}, Active"));
    user.expect("Plan: Corner sweep");
}

/// Unticking an analysis leaves it in the plan, disabled, and Run leaves it
/// out; ticking it again puts it back as it was. Its settings never move.
#[test]
fn unticking_an_analysis_keeps_it_in_the_plan_and_out_of_the_run() {
    let mut user = User::new(three_analyses());
    let before = analyses(&user.app);
    let transient = before[0].0;

    press_chip(&mut user);
    assert_eq!(user.ticked(TRANSIENT), Some(true));
    click_in_popover(&mut user, TRANSIENT);

    let after = analyses(&user.app);
    assert_eq!(after.len(), before.len(), "the plan still holds it");
    for ((id, draft, enabled), (was_id, was_draft, was_enabled)) in after.iter().zip(&before) {
        assert_eq!(id, was_id);
        assert_eq!(draft, was_draft, "settings untouched");
        assert_eq!(*enabled, *was_enabled && *id != transient);
    }
    assert!(
        !user
            .app
            .state
            .model
            .sim_setup
            .enabled_analysis_instances()
            .any(|instance| instance.id() == transient),
        "Run leaves it out"
    );
    assert_eq!(user.ticked(TRANSIENT), Some(false));
    assert!(
        chip(&user)
            .0
            .starts_with("Analysis Operating point +1, plan "),
        "{}",
        chip(&user).0
    );

    click_in_popover(&mut user, TRANSIENT);
    assert_eq!(analyses(&user.app), before);
    assert_eq!(user.ticked(TRANSIENT), Some(true));
}

/// A click on an analysis row's words opens it on Simulate › Analyses,
/// selected there, and closes the popover.
#[test]
fn a_rows_words_open_the_analysis_on_the_analyses_page() {
    let mut app = three_analyses();
    app.state.simulate.simulation_page = SimulationPage::Outputs;
    let ac = analyses(&app)
        .into_iter()
        .find(|(_, draft, _)| draft.starts_with("Ac("))
        .expect("the sweep")
        .0;
    let mut user = User::new(app);

    press_chip(&mut user);
    let row = starting(&user, "Analysis: AC \u{b7} 1 Hz to 1 GHz");
    click_at(&mut user, row);

    let app = &user.app;
    assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
    assert_eq!(app.state.simulate.simulation_page, SimulationPage::Analyses);
    assert_eq!(app.state.simulate.active_analysis_instance, Some(ac));
    assert!(
        in_popover(&user, "Add analysis\u{2026}").is_none(),
        "the popover closed"
    );
}

/// A plan with no analysis opens its popover on one line saying so, and
/// Add analysis… there opens the analysis catalogue.
#[test]
fn an_empty_plan_says_so_and_adds_from_the_catalogue() {
    let mut user = User::new(empty_plan());
    assert!(
        chip(&user)
            .0
            .starts_with("Analysis No analysis, plan Bench, "),
        "{}",
        chip(&user).0
    );

    press_chip(&mut user);
    assert!(in_popover(&user, "This plan runs no analysis.").is_some());
    let add = in_popover(&user, "Add analysis\u{2026}").expect("the popover offers Add analysis…");
    click_at(&mut user, add);

    assert!(user.app.state.model.sim_setup.session.palette_open);
    assert!(in_popover(&user, "This plan runs no analysis.").is_none());
}

/// Nothing the popover answers replaces an analysis or removes one: every
/// key its list takes, on every row, and every tick box, twice.
#[test]
fn no_popover_path_replaces_or_removes_an_analysis() {
    let mut user = User::new(three_analyses());
    let before = analyses(&user.app);

    press_chip(&mut user);
    for _ in 0..before.len() {
        for pressed in [
            egui::Key::Space,
            egui::Key::Space,
            egui::Key::Delete,
            egui::Key::Backspace,
            egui::Key::F2,
        ] {
            key(&mut user, pressed, egui::Modifiers::NONE);
        }
        key(&mut user, egui::Key::ArrowDown, egui::Modifiers::NONE);
    }
    for name in [TRANSIENT, "Operating point", "AC \u{b7} 1 Hz to 1 GHz"] {
        click_in_popover(&mut user, name);
        click_in_popover(&mut user, name);
    }
    key(&mut user, egui::Key::Enter, egui::Modifiers::NONE);

    assert_eq!(analyses(&user.app), before);
}

/// A temperature typed into the popover is the reference the next Run runs
/// at: the solver's temperature, and what the run control and the chip say.
#[test]
fn a_temperature_typed_in_the_popover_is_the_one_the_next_run_uses() {
    let mut user = User::new(RSpiceApp::test_instance());
    press_chip(&mut user);
    type_temperature(&mut user, "85");

    let setup = &user.app.state.model.sim_setup;
    assert_eq!(setup.reference_pvt.temperature_celsius, 85.0);
    assert!(
        setup.options_block().contains("+ TEMP=85"),
        "the temperature the run states"
    );
    assert!(prompt(&user).starts_with("Reference PVT changed to "));
    assert!(prompt(&user).ends_with("85 \u{b0}C."), "{}", prompt(&user));
    assert!(
        user.named.iter().any(|(name, _)| {
            name.starts_with("Run active simulation plan at ") && name.contains("85 \u{b0}C")
        }),
        "the run control runs at 85 \u{b0}C"
    );
    assert!(chip(&user).0.contains("85 \u{b0}C;"), "{}", chip(&user).0);
}

/// The process shows only where it changes what the run binds: over a
/// library the plan leaves to the reference process, and then it offers the
/// corners that library defines that a reference can name. A library pinned
/// to a section binds that section at every process.
#[test]
fn the_process_shows_only_where_it_changes_what_the_run_binds() {
    let mut app = RSpiceApp::test_instance();
    without_libraries(&mut app);
    let mut user = User::new(app);
    assert_names_reference(&user, "27 \u{b0}C");
    press_chip(&mut user);
    assert!(in_popover(&user, TEMPERATURE_FIELD).is_some());
    for process in ["FF", "FS", "TT", "SF", "SS"] {
        assert!(in_popover(&user, process).is_none(), "{process}");
    }
    press_chip(&mut user);

    bind_corner_library(&mut user.app);
    user.settle();
    assert_names_reference(&user, "27 \u{b0}C");

    follow_the_reference(&mut user.app);
    user.settle();
    assert_names_reference(&user, "TT \u{b7} 27 \u{b0}C");
    press_chip(&mut user);
    assert!(in_popover(&user, "TT").is_some());
    assert!(in_popover(&user, "SS").is_some());
    for process in ["FF", "FS", "SF"] {
        assert!(in_popover(&user, process).is_none(), "{process}");
    }
    let slow = in_popover(&user, "SS").expect("SS");
    click_at(&mut user, slow);

    let reference = user.app.state.model.sim_setup.reference_pvt;
    assert_eq!(reference.process, ProcessCorner::SS);
    assert_eq!(reference.temperature_celsius, 27.0);
    assert_names_reference(&user, "SS \u{b7} 27 \u{b0}C");
}

/// A PDK whose model sources are contracted by process makes the process a
/// choice with no library corner anywhere: the run binds the PDK's sources
/// for the process the reference names.
#[test]
fn a_pdk_that_binds_by_process_shows_the_process_without_library_corners() {
    let mut app = RSpiceApp::test_instance();
    without_libraries(&mut app);
    app.state.model.provision_test_project_technology_contract();
    assert!(
        app.state
            .model
            .model_library_manager
            .catalog()
            .libraries()
            .all(|library| library.corners.is_empty())
    );
    let mut user = User::new(app);

    assert_names_reference(&user, "TT \u{b7} 27 \u{b0}C");
    press_chip(&mut user);
    for process in ["FF", "FS", "TT", "SF", "SS"] {
        assert!(in_popover(&user, process).is_some(), "{process}");
    }
    let fast = in_popover(&user, "FF").expect("FF");
    click_at(&mut user, fast);
    assert_eq!(
        user.app.state.model.sim_setup.reference_pvt.process,
        ProcessCorner::FF
    );
}

/// A temperature the reference refuses, or words that are none, leave the
/// reference as it was and say why.
#[test]
fn a_refused_temperature_leaves_the_reference_as_it_was() {
    let mut user = User::new(RSpiceApp::test_instance());
    let before = user.app.state.model.sim_setup.reference_pvt;
    press_chip(&mut user);

    type_temperature(&mut user, "-300");
    assert_eq!(user.app.state.model.sim_setup.reference_pvt, before);
    assert!(
        prompt(&user).starts_with("Reference PVT was not changed: ")
            && prompt(&user).contains("absolute zero"),
        "{}",
        prompt(&user)
    );

    type_temperature(&mut user, "warm");
    assert_eq!(user.app.state.model.sim_setup.reference_pvt, before);
    assert!(
        prompt(&user).contains("is not a temperature"),
        "{}",
        prompt(&user)
    );
}
