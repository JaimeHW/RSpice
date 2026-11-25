//! A printed sheet reads as the sheet does: a table sheet prints the table
//! it shows, in its own words, and a plot prints the figures its card shows.

use super::availability::{app_showing, draw_sheet_and_tessellate};
use crate::feature_tests::hardcopy::resolve_retained_hardcopy_source;
use crate::workbench::AppState;
use rspice_app_hardcopy::workbench::hardcopy_adapters::sources::quick_result_availability;
use rspice_app_results::workbench::documents::result_document::ResultViewer;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::state::workbench_session::WorkspaceDocumentId;
use rspice_hardcopy::sources::{HardcopySemanticDocument, ResolvedHardcopyDocument, SemanticTable};
use rspice_hardcopy_contract::HardcopyScope;

/// The active run printed as File > Print takes it from the Results
/// workspace.
fn printed(state: &AppState) -> ResolvedHardcopyDocument {
    let dataset_id = state
        .model
        .simulation
        .active_run()
        .expect("an active run")
        .dataset_id;
    let mut shown = state.clone();
    shown.workbench.workspace = Workspace::Results;
    shown
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(dataset_id));
    let source_key = format!(
        "project:{}:result-dataset:{dataset_id}",
        shown.model.workspace.content.project.id().as_uuid()
    );
    resolve_retained_hardcopy_source(&shown, &source_key, HardcopyScope::ActivePlotDocument)
        .expect("the sheet prints")
}

/// Whether File > Print offers the sheet shown.
fn offered(state: &AppState) -> bool {
    let run = state.model.simulation.active_run().expect("an active run");
    quick_result_availability(&state.model, &*state.results, run).is_available()
}

/// The sheet's tables as a page states them.
fn sheet_tables(state: &AppState) -> Vec<SemanticTable> {
    state
        .results_reads()
        .sheet_print()
        .tables
        .into_iter()
        .map(|(title, (columns, rows))| SemanticTable {
            title,
            columns,
            rows,
        })
        .collect()
}

/// The Table sheet prints the samples it shows, every one of them in its
/// words, and Print is offered only once the sheet shows them.
#[test]
fn a_printed_table_is_the_table_the_sheet_shows() {
    let mut app = app_showing(ResultViewer::Table);
    app.state.results.results.session.viewer = ResultViewer::Table;
    assert!(
        !offered(&app.state),
        "Print is offered before the sheet shows a table"
    );
    draw_sheet_and_tessellate(&mut app, ResultViewer::Table);
    assert!(offered(&app.state));
    let resolved = printed(&app.state);
    let HardcopySemanticDocument::ResultSummary(summary) = resolved.semantic_document() else {
        panic!("a table prints as a table")
    };
    let shown = sheet_tables(&app.state);
    assert_eq!(summary.tables, shown);
    // Every sample of the run, not the rows a page has room for.
    assert_eq!(summary.tables[0].rows.len(), 64);
}

/// The Network Matrix sheet prints the matrix it shows at its frequency,
/// each coefficient in the sheet's words rather than at full precision.
#[test]
fn a_printed_network_matrix_is_the_matrix_the_sheet_shows() {
    let mut app = app_showing(ResultViewer::NetworkMatrix);
    app.state.results.results.session.viewer = ResultViewer::NetworkMatrix;
    draw_sheet_and_tessellate(&mut app, ResultViewer::NetworkMatrix);
    assert!(offered(&app.state));
    let resolved = printed(&app.state);
    let HardcopySemanticDocument::ResultSummary(summary) = resolved.semantic_document() else {
        panic!("a matrix prints as a table")
    };
    let shown = sheet_tables(&app.state);
    assert_eq!(summary.tables, shown);
    assert_eq!(summary.tables.len(), 1);
    let table = &summary.tables[0];
    assert!(table.title.contains(" at "), "{}", table.title);
    assert_eq!(table.columns.len(), table.rows.len() + 1);
    assert!(
        table
            .rows
            .iter()
            .flatten()
            .all(|cell| !cell.contains("e-") && !cell.contains("e+")),
        "{:?}",
        table.rows
    );
}
