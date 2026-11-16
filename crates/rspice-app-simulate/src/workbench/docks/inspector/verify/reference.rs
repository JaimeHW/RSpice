//! The row selected in the Reference view: both runs' values, the tolerance
//! rule in full, and how far the change was allowed to go.

use egui::Ui;
use rspice_ui_kit::panels::inspector::muted_inspector_copy;
use rspice_ui_kit::panels::section_header;

use crate::workbench::design_system::property_row;
use crate::workbench::state::VerificationSessionState;
use crate::workbench::surfaces::verify::reference::selected_reference;

pub(super) fn reference(ui: &mut Ui, session: &VerificationSessionState) {
    let Some(facts) = selected_reference(session) else {
        muted_inspector_copy(ui, "No check selected");
        return;
    };
    section_header(ui, &facts.title, Some(facts.result));
    for (label, value) in &facts.rows {
        property_row(ui, label, value);
    }
}
