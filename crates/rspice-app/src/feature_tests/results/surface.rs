//! The Results surface's specification editor route through a whole application.

use crate::workbench::RSpiceApp;
use crate::workbench::results::surface::*;

#[test]
fn specification_editor_preempts_a_persistent_result_presentation() {
    let mut app = RSpiceApp::test_instance();
    assert!(!specification_editor_open(app.state.results_reads()));

    crate::workbench::documents::result_document::open_specification_editor(
        &mut app.state.results_parts(),
    );

    assert!(specification_editor_open(app.state.results_reads()));
}

#[test]
fn pending_specs_route_survives_viewer_and_draft_reconciliation() {
    let mut app = RSpiceApp::test_instance();
    crate::workbench::documents::result_document::open_specification_editor(
        &mut app.state.results_parts(),
    );
    assert!(app.state.results.specification_editor_route_pending);

    // Model the destination's persistent-document/viewer reconciliation.
    app.state.results.results.session.viewer = crate::workbench::ResultViewer::Waves;
    app.state.results.results.session.spec_drafts = None;

    assert!(
        crate::workbench::documents::result_document::consume_pending_specification_editor(
            &mut app.state.results_parts()
        )
    );

    assert!(specification_editor_open(app.state.results_reads()));
    assert_eq!(
        app.state.results.results.session.viewer,
        crate::workbench::ResultViewer::Specs
    );
    assert!(!app.state.results.specification_editor_route_pending);
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Results
    );
}
