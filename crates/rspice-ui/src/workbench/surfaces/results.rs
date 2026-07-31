//! Result-document surface backed by the retained precision waveform viewers.

use egui::Ui;

use crate::workbench::RSpiceApp;
use crate::workbench::chrome::document_bar;
use crate::workbench::state::WorkspaceDocumentId;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
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
