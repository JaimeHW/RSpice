//! The Overview through whole frames of the application: each linked value
//! of the band takes its action, a row's Enter and its context menu take
//! the row's, and the inspector beside them states the project.

use egui::accesskit::Role;
use egui::{Event, Key, Modifiers, PointerButton};

use super::surface::populated_project_app;
use crate::feature_tests::replay::{Control, Replay, only};
use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_simulate::workbench::surfaces::verify::raster::seed_corner;
use rspice_app_state::state::{CellViewRef, ComponentType, Point, workspace};
use rspice_app_state::workbench::workspace::Workspace;

/// `app` with its Overview in front.
fn overview(app: RSpiceApp) -> Replay {
    let mut replay = Replay::of(app);
    replay.app.state.workbench.activate(Workspace::Project);
    replay.app.state.project.project_page = ProjectPage::Overview;
    replay
}

/// The project's top schematic.
fn root(app: &RSpiceApp) -> CellViewRef {
    let project = &app.state.model.workspace.content.project;
    CellViewRef::new(
        &project.root_library,
        &project.top_cell,
        workspace::DEFAULT_SCHEMATIC_VIEW,
    )
}

/// The control of `role` whose label begins `head`.
fn starting<'a>(controls: &'a [Control], head: &str, role: Role) -> &'a Control {
    controls
        .iter()
        .find(|control| control.role == role && control.label.starts_with(head))
        .unwrap_or_else(|| {
            let named = controls
                .iter()
                .filter(|control| control.role == role)
                .map(|control| control.label.as_str())
                .collect::<Vec<_>>();
            panic!("a {role:?} begins {head}: {named:#?}")
        })
}

/// A press of `button` at `at`, released.
fn press(at: egui::Pos2, button: PointerButton) -> Vec<Event> {
    let event = |pressed| Event::PointerButton {
        pos: at,
        button,
        pressed,
        modifiers: Modifiers::NONE,
    };
    vec![Event::PointerMoved(at), event(true), event(false)]
}

/// Checks never run read "Not run", and the value runs them on the top
/// schematic.
#[test]
fn not_run_checks_the_top_schematic() {
    let mut replay = overview(RSpiceApp::test_instance());

    replay.click("Not run", Role::Link);

    assert_eq!(replay.app.state.workbench.workspace, Workspace::Design);
    let root = root(&replay.app);
    assert_eq!(replay.app.state.model.workspace.content.active_view, root);
    assert!(
        replay
            .app
            .state
            .active_design_check_status()
            .last_receipt()
            .is_some(),
        "the top schematic was checked"
    );
}

/// The latest run's value opens Results on the newest run.
#[test]
fn the_latest_run_opens_the_newest_run() {
    let mut replay = overview(populated_project_app());
    let controls = replay.frames(Vec::new());
    let latest = starting(&controls, "Run ", Role::Link).center;

    replay.frames(press(latest, PointerButton::Primary));

    assert_eq!(replay.app.state.workbench.workspace, Workspace::Results);
    assert_eq!(
        replay.app.state.model.simulation.view.active_run_idx,
        Some(0)
    );
}

/// A row pressed is stood on, not acted on; Enter then opens Verify on the
/// specification it states.
#[test]
fn enter_on_a_failing_specification_opens_verify_on_it() {
    let mut app = RSpiceApp::test_instance();
    seed_corner(&mut app.state.model);
    let mut replay = overview(app);
    let controls = replay.frames(Vec::new());
    let gain = starting(&controls, "Output: gain,", Role::Row)
        .label
        .clone();
    for stated in ["39.4 dB", "Fail"] {
        assert!(gain.contains(stated), "{gain}");
    }

    replay.click(&gain, Role::Row);
    assert_eq!(replay.app.state.workbench.workspace, Workspace::Project);

    replay.key(Key::Enter, Modifiers::NONE);
    assert_eq!(replay.app.state.workbench.workspace, Workspace::Verify);
    assert_eq!(
        replay.app.state.simulate.selected_specification.as_deref(),
        Some("gain")
    );
}

/// A row's context menu offers the row's one action, and takes it.
#[test]
fn a_rows_menu_offers_its_one_action() {
    let mut app = RSpiceApp::test_instance();
    seed_corner(&mut app.state.model);
    let mut replay = overview(app);
    let controls = replay.frames(Vec::new());
    let at = starting(&controls, "Output: gain,", Role::Row).center;

    let menu = replay.frames(press(at, PointerButton::Secondary));
    let item = only(&menu, "Show in Verify", Role::Button).center;
    replay.frames(press(item, PointerButton::Primary));

    assert_eq!(replay.app.state.workbench.workspace, Workspace::Verify);
    assert_eq!(
        replay.app.state.simulate.selected_specification.as_deref(),
        Some("gain")
    );
}

/// The inspector beside the Overview states the project, whatever row the
/// reader stands on, and counts what a save would write.
#[test]
fn the_inspector_states_the_project_and_its_unsaved_work() {
    let mut replay = overview(populated_project_app());
    let state = &mut replay.app.state;
    state.model.schematic.with_undo("Place", |schematic| {
        schematic.add_component(ComponentType::Resistor, Point::new(0, 0));
    });
    state.model.workspace.content.set_active_dirty(true);
    let controls = replay.frames(Vec::new());
    let measured = starting(&controls, "Output: gain_db,", Role::Row)
        .label
        .clone();

    let inspected = replay.click(&measured, Role::Row);

    assert!(
        inspected.iter().any(|control| control.label == "Location"),
        "the project's Location stands in the inspector"
    );
    assert!(
        inspected
            .iter()
            .any(|control| control.label == "Unsaved" && control.value == "1 document"),
        "the inspector counts the unsaved drawing"
    );
}
