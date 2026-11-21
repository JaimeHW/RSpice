//! Two windows on two runs, each drawn in turn on every frame.

use crate::workbench::RSpiceApp;
use rspice_app_results::workbench::documents::result_document::{
    ResultViewer, prepare_viewer_state, viewer_availability,
};
use rspice_app_state::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRun, SimulationRunLifecycle,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::state::workbench_session::{
    WorkspaceDocumentId, WorkspaceLayoutState,
};
use rspice_results_ui::presentation::PlotView;
use rspice_results_ui::session::PlotPresentationKey;

fn transient(id: u64) -> AnalysisResult {
    AnalysisResult::new(id, AnalysisType::Transient, "Transient").with_waveforms(vec![
        RetainedTrace::new(
            "V(out)",
            vec![0.0, 0.5, 1.0],
            vec![0.0, 1.0, 0.0],
            "#55aaff",
        ),
    ])
}

fn run(id: u64) -> SimulationRun {
    let mut run = SimulationRun::new(id);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses = vec![transient(1), transient(2)];
    run
}

#[derive(Default)]
pub(super) struct MemoryStorage(std::collections::HashMap<String, String>);

impl eframe::Storage for MemoryStorage {
    fn get_string(&self, key: &str) -> Option<String> {
        self.0.get(key).cloned()
    }

    fn set_string(&mut self, key: &str, value: String) {
        self.0.insert(key.to_owned(), value);
    }

    fn remove_string(&mut self, key: &str) {
        self.0.remove(key);
    }

    fn flush(&mut self) {}
}

/// Store the session and launch again from it.
fn relaunch(app: &mut RSpiceApp) -> RSpiceApp {
    let mut storage = MemoryStorage::default();
    app.state.store_session(&mut storage);
    let restored = crate::workbench::app_state::AppState::restore_eframe_session(&storage)
        .expect("the session restores");
    let mut reopened = RSpiceApp::test_instance();
    reopened.state = restored;
    reopened
}

/// One window's place in Results: the analysis it shows, its sheet, its
/// cursors and its zoom.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Place {
    analysis: usize,
    viewer: ResultViewer,
    cursors: (f64, f64),
    zoom: (f64, f64),
}

const ZOOM_KEY: (ResultViewer, PlotPresentationKey, usize) =
    (ResultViewer::Waves, PlotPresentationKey::Global(0), 0);

fn place_of(app: &RSpiceApp) -> Place {
    let session = &app.state.results.results.session;
    Place {
        analysis: app
            .state
            .model
            .simulation
            .view
            .active_analysis_idx
            .expect("an analysis is shown"),
        viewer: session.viewer,
        cursors: (
            session.cursors.a.expect("cursor A"),
            session.cursors.b.expect("cursor B"),
        ),
        zoom: session
            .views
            .get(&ZOOM_KEY)
            .and_then(|view| view.x)
            .expect("the zoom"),
    }
}

/// Two windows on two runs, each on another analysis and sheet with its own
/// cursors and zoom, keep all four frame after frame and across a restart.
#[test]
fn two_windows_keep_their_own_analysis_sheet_cursors_and_zoom_across_a_restart() {
    let mut app = RSpiceApp::test_instance();
    let runs = [run(1), run(2)];
    let datasets = runs.each_ref().map(|run| run.dataset_id);
    app.state.model.simulation.retained.runs = runs.to_vec().into();
    app.state.workbench.activate(Workspace::Results);
    let session = &mut app.state.workbench.window_session;
    let windows = datasets.map(|dataset| {
        let window = session.create_window(
            "Results",
            Workspace::Results,
            WorkspaceLayoutState::default(),
            false,
        );
        let document = WorkspaceDocumentId::ResultDataset(dataset);
        session
            .move_document(document.clone(), session.primary(), window)
            .expect("the run's view moves to its own window");
        session.state_mut(window).unwrap().active_document = Some(document);
        window
    });
    let draw = |app: &mut RSpiceApp, window| {
        assert!(app.project_application_window(window, WorkspaceLayoutState::default()));
        prepare_viewer_state(&mut app.state.results_parts());
    };
    draw(&mut app, windows[0]);
    let second_sheet = ResultViewer::all()
        .find(|viewer| {
            *viewer != ResultViewer::Waves
                && viewer_availability(app.state.results_reads(), *viewer).available
        })
        .expect("a transient run offers a second sheet");
    let places = [
        Place {
            analysis: 0,
            viewer: ResultViewer::Waves,
            cursors: (0.1, 0.2),
            zoom: (0.0, 0.5),
        },
        Place {
            analysis: 1,
            viewer: second_sheet,
            cursors: (0.6, 0.9),
            zoom: (0.25, 1.0),
        },
    ];
    for (window, place) in windows.into_iter().zip(places) {
        draw(&mut app, window);
        assert!(app.state.model.simulation.select_analysis(place.analysis));
        prepare_viewer_state(&mut app.state.results_parts());
        let session = &mut app.state.results.results.session;
        session.viewer = place.viewer;
        session.cursors.a = Some(place.cursors.0);
        session.cursors.b = Some(place.cursors.1);
        session.views.insert(
            ZOOM_KEY,
            PlotView {
                x: Some(place.zoom),
                y: None,
            },
        );
        app.capture_application_window_projection(window);
    }

    let check = |app: &mut RSpiceApp, phase: &str| {
        for ((window, dataset), place) in windows.into_iter().zip(datasets).zip(places) {
            draw(app, window);
            assert_eq!(
                app.state
                    .model
                    .simulation
                    .active_run()
                    .map(|run| run.dataset_id),
                Some(dataset),
                "{phase}: the window reads its own run"
            );
            assert_eq!(place_of(app), place, "{phase}");
            app.capture_application_window_projection(window);
        }
    };
    for _ in 0..20 {
        check(&mut app, "frame after frame");
    }

    let mut reopened = relaunch(&mut app);
    if reopened.state.model.simulation.retained.runs.is_empty() {
        reopened.state.model.simulation.retained.runs = runs.to_vec().into();
    }
    for _ in 0..3 {
        check(&mut reopened, "after a restart");
    }
}

fn key_press(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers,
    }
}

/// A key typed into a second window runs its command in that window: Alt+2
/// opens Design there and leaves the main window on Results.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_shortcut_typed_in_a_second_window_acts_in_that_window() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Results);
    let layout = WorkspaceLayoutState::default();
    let primary = app.state.workbench.window_session.primary();
    let window = app.state.workbench.window_session.create_window(
        "Second",
        Workspace::Results,
        layout,
        false,
    );
    app.capture_application_window_projection(primary);
    assert!(app.project_application_window(window, layout));

    let ctx = egui::Context::default();
    ctx.set_os(egui::os::OperatingSystem::Windows);
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let input = egui::RawInput {
        events: vec![key_press(egui::Key::Num2, egui::Modifiers::ALT)],
        modifiers: egui::Modifiers::ALT,
        ..Default::default()
    };
    let _ = ctx.run_ui(input, |ui| crate::workbench::show_secondary(ui, &mut app));
    app.capture_application_window_projection(window);
    assert!(app.project_application_window(primary, layout));

    assert_eq!(
        app.state.workbench.workspace,
        Workspace::Results,
        "the main window stays where it was"
    );
    assert_eq!(
        app.state
            .workbench
            .window_session
            .state(window)
            .expect("the second window")
            .workspace,
        Workspace::Design
    );
}
