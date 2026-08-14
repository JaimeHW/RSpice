//! Result-document surface backed by the retained precision waveform viewers.

use egui::Ui;

use crate::workbench::RSpiceApp;
use crate::workbench::chrome::document_bar;
use crate::workbench::state::WorkspaceDocumentId;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    // Consume the one-shot route before consulting the active result
    // document. This reconstitutes the editor after viewer/document
    // reconciliation and prevents an authoring request from degrading into
    // an empty waveform viewer between frames.
    crate::workbench::documents::result_document::consume_pending_specification_editor(
        &mut app.state,
    );
    // Specification authoring is a plan transaction, not a retained-result
    // presentation. It must take precedence over an already-active persistent
    // visualization document; otherwise the editor opens in state while that
    // document continues to own the canvas, making the handoff appear dead.
    if specification_editor_open(app) {
        crate::workbench::documents::result_document::show(ui, app);
        return;
    }
    if let Some(WorkspaceDocumentId::VisualizationDocument(document_id)) =
        document_bar::active_document_id(&app.state)
    {
        crate::workbench::documents::result_document::show_persistent_document(
            ui,
            app,
            document_id,
        );
        return;
    }
    // The result viewers are document engines (plotting, cursors, FFT, RF,
    // measurements), deliberately retained while their former application chrome is
    // replaced by the workbench surrounding this surface.
    crate::workbench::documents::result_document::show(ui, app);
}

fn specification_editor_open(app: &RSpiceApp) -> bool {
    app.state.ui.results.spec_drafts.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn specification_editor_preempts_a_persistent_result_presentation() {
        let mut app = RSpiceApp::test_instance();
        assert!(!specification_editor_open(&app));

        crate::workbench::documents::result_document::open_specification_editor(&mut app.state);

        assert!(specification_editor_open(&app));
    }

    #[test]
    fn pending_specs_route_survives_viewer_and_draft_reconciliation() {
        let mut app = RSpiceApp::test_instance();
        crate::workbench::documents::result_document::open_specification_editor(&mut app.state);
        assert!(app.state.workbench.specification_editor_route_pending);

        // Model the destination's persistent-document/viewer reconciliation.
        app.state.ui.results.viewer = crate::workbench::ResultViewer::Waves;
        app.state.ui.results.spec_drafts = None;

        assert!(
            crate::workbench::documents::result_document::consume_pending_specification_editor(
                &mut app.state
            )
        );

        assert!(specification_editor_open(&app));
        assert_eq!(
            app.state.ui.results.viewer,
            crate::workbench::ResultViewer::Specs
        );
        assert!(!app.state.workbench.specification_editor_route_pending);
        assert_eq!(
            app.state.workbench.workspace,
            crate::workbench::state::Workspace::Results
        );
    }
}
