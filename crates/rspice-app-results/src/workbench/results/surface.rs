//! The Results stage: the result-document surface backed by the retained
//! precision waveform viewers.

use egui::Ui;

use crate::simulation::SimulationController;
use crate::workbench::chrome::document_bar::results::active_result_document;
use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::state::WorkspaceDocumentId;

pub(super) fn show(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    run_controller: &mut SimulationController,
) {
    if let Some(WorkspaceDocumentId::VisualizationDocument(document_id)) =
        active_result_document(state.model, state.workbench)
    {
        crate::workbench::documents::result_document::show_persistent_document(
            ui,
            state,
            run_controller,
            document_id,
        );
        return;
    }
    // The result viewers are document engines (plotting, cursors, FFT, RF,
    // measurements), deliberately retained while their former application chrome is
    // replaced by the workbench surrounding this surface.
    crate::workbench::documents::result_document::show(ui, state, run_controller);
}
