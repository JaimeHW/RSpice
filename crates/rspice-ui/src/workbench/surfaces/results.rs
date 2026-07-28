//! Result-document surface backed by the retained precision waveform viewers.

use egui::Ui;

use crate::workbench::RSpiceApp;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    // The result viewers are document engines (plotting, cursors, FFT, RF,
    // measurements), deliberately retained while their former application chrome is
    // replaced by the workbench surrounding this surface.
    crate::workbench::documents::result_document::show(ui, app);
}
