//! Each subject of the project's health read from every place that states
//! it: the Overview's row, the chip over the other pages, the inspector and
//! the status bar say it in the same words.

#![cfg(not(target_arch = "wasm32"))]

use rspice_design::drc::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};

use crate::workbench::examples::hierarchy_reference;
use crate::workbench::{RSpiceApp, features};
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_project::workbench::surfaces::project::overview::{OverviewSnapshot, Tone};
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::CellViewRef;
use rspice_app_state::workbench::workspace::Workspace;

use super::recovery_frames::{app_frame, offscreen, painted};

/// Every string the whole application painted, one per line, with the
/// Project workspace on `page` once its layout has settled.
fn painted_on(app: &mut RSpiceApp, page: ProjectPage) -> String {
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = page;
    let ctx = offscreen();
    for _ in 0..3 {
        app_frame(&ctx, app, Vec::new());
    }
    painted(&app_frame(&ctx, app, Vec::new()))
}

fn has_line(painted: &str, line: &str) -> bool {
    painted.lines().any(|painted| painted == line)
}

/// The Overview's row for `area`: its state, its words as painted, its
/// tone.
fn row(app: &mut RSpiceApp, area: &str) -> (String, String, Tone) {
    let snapshot = OverviewSnapshot::capture(
        &egui::Context::default(),
        &features::cx_with_foreign_answers(app, |_| true),
    );
    snapshot
        .statuses
        .iter()
        .find(|status| status.area == area)
        .map(|status| (status.state().to_owned(), status.text(), status.tone()))
        .unwrap_or_else(|| panic!("the Overview has a {area} row"))
}

/// A project whose top schematic is the cell view on screen.
fn schematic_project() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.model.workspace.content.active_view = CellViewRef::new(
        &app.state.model.workspace.content.project.root_library,
        &app.state.model.workspace.content.project.top_cell,
        rspice_app_state::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    );
    app
}

/// That project with its checks run, finding one error and one warning.
fn checked_project() -> RSpiceApp {
    let mut app = schematic_project();
    let mut result = DrcResult::new();
    for (id, severity) in [(1, DrcSeverity::Error), (2, DrcSeverity::Warning)] {
        result.add_violation(DrcViolation {
            severity,
            ..DrcViolation::new(
                id,
                DrcViolationType::MissingGround,
                "Net vbias1 has no DC path to ground",
                DrcLocation::Global,
            )
        });
    }
    result.completed = true;
    app.state
        .publish_active_design_check_result(result)
        .expect("the checks publish");
    app
}

#[test]
fn schematic_checks_read_alike_on_the_overview_the_band_the_inspector_and_the_status_bar() {
    let mut app = checked_project();
    let (state, text, tone) = row(&mut app, "Schematic checks");
    assert_eq!(state, "1 error \u{b7} 1 advisory");
    assert_eq!(tone, Tone::Error);

    let overview = painted_on(&mut app, ProjectPage::Overview);
    assert!(has_line(&overview, &text), "the row:\n{overview}");
    assert!(
        has_line(&overview, &state),
        "the inspector's Problems:\n{overview}"
    );
    assert!(
        has_line(&overview, &format!("Problems \u{b7} {state}")),
        "the status bar:\n{overview}"
    );

    let library = painted_on(&mut app, ProjectPage::Library);
    assert!(
        has_line(&library, &format!("Checks \u{b7} {state}")),
        "the chip:\n{library}"
    );
}

/// Checks that have never run are neither passed nor a caution: every place
/// says "Not run" and the status bar, which counts findings, says nothing.
#[test]
fn checks_that_have_not_run_read_not_run_and_unmarked_everywhere() {
    let mut app = schematic_project();
    let (state, _, tone) = row(&mut app, "Schematic checks");
    assert_eq!((state.as_str(), tone), ("Not run", Tone::Neutral));

    let overview = painted_on(&mut app, ProjectPage::Overview);
    assert!(
        has_line(&overview, "Not run"),
        "the inspector's Problems:\n{overview}"
    );
    assert!(!overview.contains("Problems \u{b7}"), "{overview}");

    let library = painted_on(&mut app, ProjectPage::Library);
    assert!(has_line(&library, "Checks \u{b7} Not run"), "{library}");
}

#[test]
fn the_configuration_reads_alike_on_the_overview_the_band_and_the_inspector() {
    let mut reference = RSpiceApp::test_instance();
    reference.state = hierarchy_reference::build().state;
    for (mut app, name) in [
        (schematic_project(), "Default"),
        (reference, "Reference hierarchy"),
    ] {
        let (state, text, _) = row(&mut app, "Configuration");
        assert_eq!(state, name);

        let overview = painted_on(&mut app, ProjectPage::Overview);
        assert!(has_line(&overview, &text), "the row:\n{overview}");
        assert!(has_line(&overview, &state), "the inspector:\n{overview}");

        let library = painted_on(&mut app, ProjectPage::Library);
        assert!(
            has_line(&library, &format!("Configuration \u{b7} {state}")),
            "the chip:\n{library}"
        );
    }
}

#[test]
fn the_latest_run_reads_alike_on_the_overview_and_the_band() {
    use rspice_app_state::state::{SimulationRunLifecycle, SimulationRunProvenance};

    let mut app = schematic_project();
    let (state, _, _) = row(&mut app, "Latest run");
    assert_eq!(state, "No runs");
    let library = painted_on(&mut app, ProjectPage::Library);
    assert!(has_line(&library, "No runs"), "the chip:\n{library}");

    {
        let run = app.state.model.simulation.start_run();
        run.lifecycle = SimulationRunLifecycle::Failed;
        run.success = false;
        run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .expect("seal run");
    }
    let (state, text, tone) = row(&mut app, "Latest run");
    assert!(state.starts_with("Run "), "{state}");
    assert_eq!(tone, Tone::Error);
    let overview = painted_on(&mut app, ProjectPage::Overview);
    assert!(has_line(&overview, &text), "the row:\n{overview}");
    let library = painted_on(&mut app, ProjectPage::Library);
    assert!(has_line(&library, &state), "the chip:\n{library}");
}

/// A model file deleted since it was attached is the Dependencies row's to
/// say, in the words the Dependencies page says it.
#[test]
fn a_missing_model_file_reads_alike_on_the_overview_and_the_dependencies_page() {
    let folder = unique_temp_dir("overview-dependencies");
    let mut app = schematic_project();
    let path = folder.join("missing.lib");
    std::fs::write(&path, ".model nmissing nmos level=1 vto=0.5\n").expect("the file writes");
    let name = app
        .state
        .model
        .model_library_manager
        .load_library_file(&path, None)
        .expect("the model file imports");
    std::fs::remove_file(&path).expect("the model file is deleted");

    let overview = painted_on(&mut app, ProjectPage::Overview);
    let (state, text, tone) = row(&mut app, "Dependencies");
    assert_eq!(state, format!("{name}: not found"));
    assert_eq!(tone, Tone::Error);
    assert!(has_line(&overview, &text), "the row:\n{overview}");

    let dependencies = painted_on(&mut app, ProjectPage::Dependencies);
    assert!(has_line(&dependencies, "Not found"), "{dependencies}");
    let _ = std::fs::remove_dir_all(folder);
}

/// A project never saved says so on its row and in the inspector's
/// Location, and the header names no file.
#[test]
fn a_project_never_saved_says_so_on_the_row_and_in_the_inspector() {
    let mut app = schematic_project();
    let (state, _, _) = row(&mut app, "Unsaved work");
    assert_eq!(state, "Not saved");
    let overview = painted_on(&mut app, ProjectPage::Overview);
    assert!(has_line(&overview, "Location"), "{overview}");
    assert!(has_line(&overview, "Not saved"), "{overview}");
}

/// "Run checks" checks the cell view its row describes: the root runs use,
/// whatever is on screen.
#[test]
fn run_checks_opens_and_checks_the_root_its_row_describes() {
    let mut app = RSpiceApp::test_instance();
    app.state = hierarchy_reference::build().state;
    let root = app
        .state
        .model
        .workspace
        .content
        .simulation_root_reference();
    let elsewhere = CellViewRef::new(&root.library, &root.cell, "symbol");
    app.state.model.workspace.content.active_view = elsewhere;

    let snapshot = OverviewSnapshot::capture(
        &egui::Context::default(),
        &features::cx_with_foreign_answers(&mut app, |_| true),
    );
    let checks = &snapshot.statuses[0];
    assert_eq!(checks.area, "Schematic checks");
    assert_eq!(checks.action, "Run checks");
    checks
        .intent
        .clone()
        .raise(&egui::Context::default(), &mut features::cx(&mut app));
    app.after_contribution();

    assert_eq!(app.state.model.workspace.content.active_view, root);
}

/// A netlist-first project's inspector states its deck and the deck's
/// validation, never the pristine bootstrap schematic or its checks.
#[test]
fn a_netlist_first_inspector_states_the_deck_as_the_overview_does() {
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.apply_imported_netlist(
        "V1 out 0 1\n.op\n.end\n".to_owned(),
        None,
        "front_end.sp"
    ));
    let (state, _, _) = row(&mut app, "Source deck");
    assert_eq!(state, "Not validated");

    let overview = painted_on(&mut app, ProjectPage::Overview);
    for line in ["Source deck", "front_end.sp", "Validation", "Not validated"] {
        assert!(has_line(&overview, line), "{line:?}:\n{overview}");
    }
    for line in ["Testbench", "Schematic checks"] {
        assert!(!has_line(&overview, line), "{line:?}:\n{overview}");
    }
}
