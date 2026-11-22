//! The design check run through the whole application: a check that finds errors warns
//! in the notices and opens the Problems page.

use crate::workbench::app_state::AppState;
use rspice_app_design::workbench::app::actions::workspace::design_checks::*;
use rspice_app_state::diagnostics::LogSeverity;
use rspice_design::drc::DrcResult;
use rspice_design::drc::{DrcLocation, DrcSeverity, DrcViolation, DrcViolationType};

fn finding(severity: DrcSeverity) -> DrcResult {
    let mut violation = DrcViolation::new(
        1,
        DrcViolationType::UnconnectedPin,
        "R1 pin 2 is not connected",
        DrcLocation::Point { x: 0.0, y: 0.0 },
    );
    violation.severity = severity;
    let mut result = DrcResult::new();
    result.add_violation(violation);
    result
}

/// A check that found errors says so at Warning and takes the reader to
/// the Problems page; a clean one is a record. Neither is toasted.
#[test]
fn a_check_that_finds_errors_warns_and_opens_problems() {
    let ctx = egui::Context::default();
    let mut state = AppState::default();
    report(
        &mut state.model,
        &mut state.workbench,
        "Design checks",
        Ok(finding(DrcSeverity::Error)),
    );
    state.drain_notices(&ctx);
    let line = state
        .model
        .log_buffer
        .entries()
        .last()
        .expect("the check is recorded");
    assert_eq!(line.severity, LogSeverity::Warning);
    assert!(
        line.message
            .starts_with("Design checks: 1 error, 0 advisories.")
    );
    assert!(state.workbench.console_visible);
    assert_eq!(
        state.workbench.console_page,
        rspice_app_workbench::workbench::state::workbench_session::ConsolePage::Problems
    );
    assert!(state.workbench.toasts.activity().is_empty());

    let mut state = AppState::default();
    let page = state.workbench.console_page;
    report(
        &mut state.model,
        &mut state.workbench,
        "Design checks",
        Ok(finding(DrcSeverity::Warning)),
    );
    state.drain_notices(&ctx);
    let line = state
        .model
        .log_buffer
        .entries()
        .last()
        .expect("the check is recorded");
    assert_eq!(line.severity, LogSeverity::Info);
    assert_eq!(state.workbench.console_page, page);
    assert!(state.workbench.toasts.activity().is_empty());
}

/// The finding badges painted on the canvas: three-cornered fills in a
/// severity's colour, inside the drawing.
fn badges(user: &crate::workbench::app::veriloga_flow_tests::User) -> usize {
    let palette = rspice_app_workbench::ui::tokens::Tokens::get(&user.ctx).color;
    let canvas = user.expect("Schematic canvas");
    user.shapes
        .iter()
        .filter(|clipped| match &clipped.shape {
            egui::Shape::Path(path) => {
                path.closed
                    && path.points.len() == 3
                    && (path.fill == palette.err || path.fill == palette.warn)
                    && path.points.iter().all(|point| canvas.contains(*point))
            }
            _ => false,
        })
        .count()
}

/// A check marks what it found on the drawing in the default display, over
/// the operating point, without a trip to the display options; only Hidden
/// hides the marks.
#[test]
fn a_check_marks_its_findings_over_the_operating_point() {
    use crate::workbench::RSpiceApp;
    use crate::workbench::session::project_transactions::create_example_project;
    use rspice_app_design::workbench::design::commands::DesignCommand;
    use rspice_app_state::state::{ComponentType, Point, SchematicAnnotationVisibility};
    use rspice_app_state::workbench::examples::EXAMPLES;
    use rspice_app_state::workbench::workspace::Workspace;
    use rspice_app_workbench::workbench::framework::command_registry::ids;

    let mut app = RSpiceApp::test_instance();
    create_example_project(&mut app.state, EXAMPLES[0]).expect("the example opens as a project");
    let mut user = crate::workbench::app::veriloga_flow_tests::User::new(app);
    user.app.state.workbench.activate(Workspace::Design);
    let r1 = user
        .app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.kind == ComponentType::Resistor)
        .expect("the example draws a resistor")
        .pos;
    // A resistor of its own inside the loop the circuit draws, on the sheet
    // and connected to nothing.
    user.app
        .state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(r1.x, r1.y + 150));
    user.app.state.sync_active_schematic_to_workspace();
    user.app.execute_command_key(ids::ZOOM_FIT);
    user.settle();
    assert_eq!(
        user.app.state.design.schematic_visibility.annotations,
        SchematicAnnotationVisibility::OperatingPoint,
        "the default display"
    );

    user.app
        .execute_command_key(DesignCommand::CheckCellview.key());
    user.settle();
    assert!(badges(&user) > 0, "the check's findings are marked");

    user.app.state.design.schematic_visibility.annotations = SchematicAnnotationVisibility::Hidden;
    user.settle();
    assert_eq!(badges(&user), 0, "Hidden hides them");
}
