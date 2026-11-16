//! Initial conditions and node sets on the Solver & convergence page, on the
//! running application: typed or picked into their grids, refused in place
//! when they name no net of the design, written into the deck as `.IC` and
//! `.NODESET` cards, one step of the setup's Undo each, kept by a save, run
//! by the application and by the exported deck alike.

use super::engine::enable_only;
use super::pipeline::edit_the_transient;
use super::setup_page::{Page, stored_options};
use super::*;
use rspice_app_state::state::{
    Component, ComponentType, NetLabel, Point, Rotation, SimulationPage, Wire,
};
use rspice_app_workbench::workbench::framework::command_registry::ids;
use rspice_simulation_contract::options::NodeVoltage;

/// The RC lowpass driven by 1 V DC: R1 1k from the drive to `out`, C1 159n
/// from `out` to ground, its transient 500 µs in 1 µs steps.
fn rc() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    enable_only(&mut app.state, &[]);
    rspice_app_state::workbench::examples::load_example(
        "RC Lowpass Filter",
        &mut app.state.model.schematic,
    );
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.name == "VIN")
        .expect("the drive")
        .kind = ComponentType::VoltageSource;
    app.state.sync_active_schematic_to_workspace();
    edit_the_transient(&mut app.state, |draft| {
        draft.stop = "500u".to_owned();
        draft.step = "1u".to_owned();
        draft.max_step = "1u".to_owned();
    });
    app
}

/// Two inverters driving each other: `q` is 3.3 V when `qb` is low and 0 V
/// when it is high, and `qb` the same of `q`, each loaded by 1 MΩ. Both
/// states hold, and so does the balance between them at 1.65 V; which one
/// the operating point finds is what a node set decides.
fn latch() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    enable_only(&mut app.state, &[]);
    let doc = app.state.model.schematic.document_mut_for_test();
    let rail = 300;
    let mut taps = Vec::new();
    let parts = [
        (
            ComponentType::BehavioralSource,
            "B1",
            "V=1.65-1.65*tanh(10*(V(qb)-1.65))",
            "q",
        ),
        (ComponentType::Resistor, "RQ", "1meg", "q"),
        (
            ComponentType::BehavioralSource,
            "B2",
            "V=1.65-1.65*tanh(10*(V(q)-1.65))",
            "qb",
        ),
        (ComponentType::Resistor, "RQB", "1meg", "qb"),
    ];
    for ((id, x), (kind, name, value, net)) in (1_u64..).zip((100..).step_by(80)).zip(parts) {
        let mut part = Component::new(id, kind, Point::new(x, 200)).with_name_value(name, value);
        if kind == ComponentType::Resistor {
            part = part.with_rotation(Rotation::R90);
        }
        let mut pins = part
            .terminal_positions()
            .into_iter()
            .map(|(_, at)| at)
            .collect::<Vec<_>>();
        pins.sort_by_key(|at| at.y);
        let (top, bottom) = (pins[0], pins[1]);
        doc.components.push(part);
        let id = doc.wires.len() as u64 + 1;
        doc.wires
            .push(Wire::new(id, vec![top, Point::new(top.x, 120)]));
        let id = doc.net_labels.len() as u64 + 1;
        doc.net_labels
            .push(NetLabel::new(id, Point::new(top.x, 140), net));
        let id = doc.wires.len() as u64 + 1;
        doc.wires
            .push(Wire::new(id, vec![bottom, Point::new(bottom.x, rail)]));
        taps.push(Point::new(bottom.x, rail));
    }
    let id = doc.wires.len() as u64 + 1;
    doc.wires.push(Wire::new(id, taps));
    doc.components.push(Component::new(
        9,
        ComponentType::Ground,
        Point::new(100, rail + 10),
    ));
    app.state.sync_active_schematic_to_workspace();
    edit_the_transient(&mut app.state, |draft| {
        draft.stop = "1u".to_owned();
        draft.step = "10n".to_owned();
        draft.max_step = "10n".to_owned();
    });
    app
}

/// Run the plan and hand back its transient's trace of `net`, saved as
/// `V(net)` or by the net's name: its times and volts.
fn ran(app: &mut RSpiceApp, net: &str) -> (Vec<f64>, Vec<f64>) {
    app.invalidate_simulation_preflight();
    start(&mut app.state, &mut app.simulation_controller, false);
    app.settle_preflight();
    assert!(
        preflight_blockers(app).is_empty(),
        "{:#?}",
        preflight_blockers(app)
    );
    let _ = app.queued_by_its_check();
    let _ = app.dispatch_frame();
    app.finish_the_run();
    let run = app
        .state
        .model
        .simulation
        .retained
        .runs
        .first()
        .expect("the run is retained");
    assert!(run.success, "{:?}", run.analyses);
    trace(run, net)
}

fn trace(run: &rspice_app_state::state::SimulationRun, net: &str) -> (Vec<f64>, Vec<f64>) {
    let name = format!("V({net})");
    let waves = || {
        run.analyses
            .iter()
            .flat_map(|analysis| analysis.waveforms.iter())
    };
    let wave = waves()
        .find(|wave| wave.name.eq_ignore_ascii_case(&name) || wave.name.eq_ignore_ascii_case(net))
        .unwrap_or_else(|| {
            panic!(
                "the run saves {name}: {:?}",
                waves().map(|wave| wave.name.as_str()).collect::<Vec<_>>()
            )
        });
    (wave.x.to_vec(), wave.y.to_vec())
}

fn description(page: &Page, label: &str) -> String {
    page.control(label).description.clone().unwrap_or_default()
}

/// An initial condition typed into the page's grid, its net in another case
/// than the design's, is stored as the design names the net, written into
/// the deck as `.IC V(out)=0.25` and is one step of Undo. The transient then
/// starts with the capacitor at 0.25 V and charges toward the 1 V drive as
/// `1 - 0.75·exp(-t/RC)` with RC = 159 µs.
#[test]
fn an_initial_condition_typed_on_the_solver_page_starts_the_transient_there() {
    let mut page = Page::open_on(rc(), SimulationPage::Solver);
    let before = stored_options(&page.app);
    page.retype("New initial condition net", "OUT");
    assert_eq!(
        stored_options(&page.app),
        before,
        "a net alone is not yet a row"
    );
    page.retype("New initial condition volts", "250m");
    assert_eq!(
        page.app.state.model.sim_setup.options.initial_conditions,
        [NodeVoltage {
            net: "out".to_owned(),
            volts: 0.25
        }]
    );
    assert!(page.app.state.model.sim_setup.options.nodesets.is_empty());
    page.pass(Vec::new());
    assert_eq!(
        page.control("Initial condition 1 net").value.as_deref(),
        Some("out")
    );
    let deck = rspice_app_runs::simulation::controller::SimulationController::prepare_design_netlist_for_inspection(
        &page.app.state.model,
    )
    .expect("the deck");
    assert!(deck.lines().any(|line| line == ".IC V(out)=0.25"), "{deck}");

    let set = stored_options(&page.app);
    page.app.execute_command_key(ids::UNDO);
    assert_eq!(
        stored_options(&page.app),
        before,
        "one Undo takes the row back"
    );
    page.app.execute_command_key(ids::REDO);
    assert_eq!(stored_options(&page.app), set);

    let (times, volts) = ran(&mut page.app, "out");
    let tau = 1e3 * 159e-9;
    let worst = times
        .iter()
        .zip(&volts)
        .map(|(t, v)| (v - (1.0 - 0.75 * (-t / tau).exp())).abs())
        .fold(0.0_f64, f64::max);
    assert!(
        volts[0] == 0.25,
        "the transient starts at the .IC: {}",
        volts[0]
    );
    assert!(
        // 2.9e-6 V measured on this grid: linear interpolation between the
        // solver's points, which alone sit within 9.0e-7 V.
        worst < 1e-5,
        "V(out) against 1 - 0.75 exp(-t/RC): worst {worst:e} V over {} rows",
        times.len()
    );
}

/// A net the design does not have, ground, or a net already set is refused
/// where it was typed: the cell keeps the text, its reason on hover, and the
/// plan is unchanged. A removed row is one step of Undo too.
#[test]
fn a_net_a_row_cannot_name_is_refused_in_place() {
    let mut page = Page::open_on(rc(), SimulationPage::Solver);
    let before = stored_options(&page.app);
    page.retype("New initial condition net", "nosuch");
    assert_eq!(
        description(&page, "New initial condition net"),
        "the design has no net nosuch"
    );
    page.retype("New node set net", "0");
    assert!(
        description(&page, "New node set net").starts_with("0 is ground"),
        "{}",
        description(&page, "New node set net")
    );
    page.retype("New node set volts", "volts");
    assert!(
        !description(&page, "New node set volts").is_empty(),
        "a value that is no number is refused"
    );
    assert_eq!(stored_options(&page.app), before);

    page.retype("New node set net", "out");
    page.retype("New node set volts", "1");
    let set = stored_options(&page.app);
    assert_ne!(set, before);
    page.retype("New node set net", "out");
    assert_eq!(description(&page, "New node set net"), "out is already set");
    page.click("Remove node set 1");
    assert_eq!(stored_options(&page.app), before);
    page.app.execute_command_key(ids::UNDO);
    assert_eq!(stored_options(&page.app), set, "Undo puts the row back");
}

/// Set the latch's node sets on the page, `q` then `qb`.
fn set_latch(page: &mut Page, q: &str, qb: &str) {
    page.retype("New node set net", "q");
    page.retype("New node set volts", q);
    page.retype("New node set net", "qb");
    page.retype("New node set volts", qb);
    assert_eq!(
        page.app.state.model.sim_setup.options.nodesets.len(),
        2,
        "{:?}",
        page.app.state.model.sim_setup.options.nodesets
    );
}

/// Left alone, the latch's operating point is the balance at 1.65 V. Node
/// sets on `q` and `qb` send it to the state they name, either one, and the
/// transient holds it there; the rows survive a save and reopen, and the
/// exported deck run on the deck route settles in the same state.
#[test]
fn node_sets_send_the_latch_to_the_state_they_name() {
    let (_, balanced) = ran(&mut latch(), "q");
    assert!(
        (balanced[0] - 1.65).abs() < 1e-9,
        "left alone the latch balances: {}",
        balanced[0]
    );

    for (q, qb, high) in [("3.3", "0", true), ("0", "3.3", false)] {
        let mut page = Page::open_on(latch(), SimulationPage::Solver);
        set_latch(&mut page, q, qb);
        let (_, volts) = ran(&mut page.app, "q");
        let expected = if high { 3.3 } else { 0.0 };
        assert!(
            volts.iter().all(|v| (v - expected).abs() < 1e-9),
            "node sets q={q} qb={qb}: V(q) from {} to {}",
            volts[0],
            volts[volts.len() - 1]
        );

        let path = std::env::temp_dir().join(format!(
            "rspice-nodeset-{}.rspiceproj",
            uuid::Uuid::new_v4()
        ));
        rspice_app_services::workbench::lifecycle::project_lifecycle::save_native(
            &mut page.app.state,
            rspice_app_services::workbench::lifecycle::project_lifecycle::SaveScope::AllDocuments,
            &path,
            rspice_app_services::workbench::lifecycle::project_lifecycle::DestinationAuthority::UserSelected,
        )
        .expect("the project saves");
        let mut reopened = AppState::default();
        let opened = reopened.load_project_from_path(&path);
        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
        let mut lock = path.as_os_str().to_os_string();
        lock.push(".rspice.lock");
        let _ = std::fs::remove_file(std::path::PathBuf::from(lock));
        assert!(opened, "the saved project reopens");
        assert_eq!(
            reopened.model.sim_setup.options.nodesets,
            page.app.state.model.sim_setup.options.nodesets
        );

        let folder = std::env::temp_dir().join(format!("rspice-nodeset-{}", uuid::Uuid::new_v4()));
        let deck_path = folder.join("latch.spice");
        crate::feature_tests::code::netlist_export::export_spice(&mut page.app, &deck_path);
        let deck = std::fs::read_to_string(&deck_path).expect("the deck is written");
        assert!(
            deck.lines()
                .any(|line| line == format!(".NODESET V(q)={q}")),
            "{deck}"
        );
        let run = rspice_app_runs::simulation::controller::test_execution::run_manual_batch_at(
            &deck,
            &deck_path,
            rspice_results::run::SimulationRunLifecycle::Completed,
        );
        let _ = std::fs::remove_dir_all(&folder);
        // The deck route prepares on this thread; the next run's dispatch
        // checks nothing was.
        let _ = rspice_app_runs::simulation::controller::prepared_run::thread_preparations::take();
        let (_, exported) = trace(&run, "q");
        assert!(
            exported.iter().all(|v| (v - expected).abs() < 1e-9),
            "the exported deck: V(q) from {} to {}",
            exported[0],
            exported[exported.len() - 1]
        );
    }
}

/// A row naming a net the design no longer has refuses the run, naming the
/// row: the engine would ignore the card and start somewhere the plan does
/// not say.
#[test]
fn a_row_naming_a_net_the_design_lost_refuses_the_run() {
    let mut app = rc();
    let mut options = app.state.model.sim_setup.options.clone();
    options.nodesets.push(NodeVoltage {
        net: "gone".to_owned(),
        volts: 1.0,
    });
    app.state.model.sim_setup.commit_options(&options);
    let error = rspice_app_runs::simulation::controller::SimulationController::prepare_design_netlist_for_inspection(
        &app.state.model,
    )
    .expect_err("the deck is refused");
    assert!(
        error
            .to_string()
            .contains("Node sets gone: the design has no net gone"),
        "{error}"
    );
    app.invalidate_simulation_preflight();
    start(&mut app.state, &mut app.simulation_controller, false);
    app.settle_preflight();
    let blockers = format!("{:?}", preflight_blockers(&app));
    assert!(
        blockers.contains("the design has no net gone"),
        "{blockers}"
    );
}
