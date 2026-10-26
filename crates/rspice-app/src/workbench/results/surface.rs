//! The Results stage: the result-document surface backed by the retained
//! precision waveform viewers.

use egui::Ui;

use crate::simulation::SimulationController;
use crate::workbench::chrome::document_bar::results::active_result_document;
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use crate::workbench::state::WorkspaceDocumentId;

pub(super) fn show(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    run_controller: &mut SimulationController,
    selected_specification: Option<&str>,
) {
    // Consume the one-shot route before consulting the active result
    // document. This reconstitutes the editor after viewer/document
    // reconciliation and prevents an authoring request from degrading into
    // an empty waveform viewer between frames.
    crate::workbench::documents::result_document::consume_pending_specification_editor(state);
    // Specification authoring is a plan transaction, not a retained-result
    // presentation. It must take precedence over an already-active persistent
    // visualization document; otherwise the editor opens in state while that
    // document continues to own the canvas, making the handoff appear dead.
    if specification_editor_open(state.results_reads()) {
        crate::workbench::documents::result_document::show(
            ui,
            state,
            run_controller,
            selected_specification,
        );
        return;
    }
    if let Some(WorkspaceDocumentId::VisualizationDocument(document_id)) =
        active_result_document(state.model, state.workbench)
    {
        crate::workbench::documents::result_document::show_persistent_document(
            ui,
            state,
            run_controller,
            selected_specification,
            document_id,
        );
        return;
    }
    // The result viewers are document engines (plotting, cursors, FFT, RF,
    // measurements), deliberately retained while their former application chrome is
    // replaced by the workbench surrounding this surface.
    crate::workbench::documents::result_document::show(
        ui,
        state,
        run_controller,
        selected_specification,
    );
}

fn specification_editor_open(state: ResultsReads<'_>) -> bool {
    state.results.session.spec_drafts.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::RSpiceApp;

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
}
