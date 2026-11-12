//! The Jobs manager's history hop over the whole application: opening a row
//! lands on that run in Results, and a pruned run is refused without moving
//! the reader.

use crate::state::{AnalysisResult, AnalysisType};
use crate::workbench::RSpiceApp;
use crate::workbench::features;
use crate::workbench::tools::jobs_manager::open_run_in_results;

/// The history is where a run is found; opening one is what a reader came
/// to do next. The stable identity is re-resolved at that moment, because
/// history can be pruned while this dialog is open.
#[test]
fn opening_a_history_row_lands_on_that_run_and_refuses_a_pruned_one() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Simulate);
    let first = app.state.model.simulation.start_run().run_id;
    app.state.model.simulation.retained.runs[0]
        .analyses
        .push(AnalysisResult::new(
            1,
            AnalysisType::Transient,
            "retained TRAN",
        ));
    let second = app.state.model.simulation.start_run().run_id;
    assert_eq!(
        app.state.model.simulation.active_run().map(|run| run.id),
        Some(2),
        "the newest run is selected before the hop"
    );

    open_run_in_results(&mut features::cx(&mut app), first);
    app.after_contribution();
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Results
    );
    assert_eq!(
        app.state.model.simulation.active_run().map(|run| run.id),
        Some(1),
        "the row's own run is what the hop carried, not the active one"
    );

    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Simulate);
    app.state
        .model
        .simulation
        .retained
        .runs
        .retain(|run| run.run_id != second);
    let before = app.state.model.log_buffer.revision();
    open_run_in_results(&mut features::cx(&mut app), second);
    app.after_contribution();
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Simulate,
        "a pruned run must not move the reader"
    );
    assert!(
        app.state
            .prompt_text()
            .is_some_and(|text| text.contains("no longer retained")),
        "the refusal answers the gesture"
    );
    assert_eq!(app.state.model.log_buffer.revision(), before);
}
