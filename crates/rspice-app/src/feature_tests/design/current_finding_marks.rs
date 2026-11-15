//! What the canvas marks for the current finding beyond its own badge, read
//! through the whole application: the loop's finding is published by the
//! project session's design check, and every source in it is marked where it
//! is drawn.

use crate::schematic::view::violations::related_marks;
use crate::workbench::app_state::AppState;
use crate::workbench::diagnostic_routes::tests::{draw_source_loop, loop_finding};
use rspice_app_state::state::Point;
use rspice_design::drc::DrcViolationType;

/// With a loop current, every source in it is marked where it is drawn, in
/// the finding's own severity colour; with nothing current, nothing is.
#[test]
fn every_source_in_the_current_loop_is_marked() {
    let mut state = AppState::default();
    state.open_project_session();
    let (top, _, sources) = draw_source_loop(&mut state);
    assert_eq!(
        related_marks(&state.canvas_facts()),
        None,
        "no finding is current"
    );

    state
        .model
        .diagnostics
        .set_current_finding(Some(loop_finding(&state, &top)));
    let (severity, mut marked) =
        related_marks(&state.canvas_facts()).expect("the loop's sources are marked");

    let mut drawn: Vec<Point> = sources.iter().map(|source| source.position).collect();
    marked.sort_by_key(|point| (point.x, point.y));
    drawn.sort_by_key(|point| (point.x, point.y));
    assert_eq!(
        marked, drawn,
        "each of the three sources, where it is drawn"
    );
    assert_eq!(
        severity,
        DrcViolationType::SourceToSource.default_severity()
    );
}
