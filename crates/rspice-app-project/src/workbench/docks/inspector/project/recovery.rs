//! The inspector beside the Recovery page: the selected checkpoint, and the
//! recovery store only when it is not in order.

use egui::Ui;
use rspice_ui_kit::panels::{Problem, section_header};

use crate::workbench::design_system::{property_row, property_row_problem};

use super::ProjectInspector;

pub(super) fn project_recovery(ui: &mut Ui, inspector: &ProjectInspector<'_>) {
    let recovery = inspector.checkpoint_recovery;
    let selected = inspector.checkpoint_selection.as_deref();
    let checkpoint = recovery
        .checkpoints
        .iter()
        .find(|item| selected.is_some_and(|id| id == item.checkpoint_id().to_string()));
    let integrity = recovery.error.is_none() && recovery.quarantined.is_empty();

    // Only a selected checkpoint has anything to say. The revision counter,
    // the payload's byte count and the checkpoint's internal id are not
    // facts a reader restores on.
    if let Some(checkpoint) = checkpoint {
        section_header(ui, "Selected checkpoint", None);
        property_row(ui, "Reason", checkpoint.reason().label());
        if let Some(note) = checkpoint.note() {
            property_row(ui, "Note", note);
        }
        // What retention does with it: the one fact that tells a reader
        // whether to pin it.
        property_row(ui, "Kept", checkpoint.retention().label());
    }

    let quarantined = recovery.quarantined.len();
    // The store is in order or it is not: "verified" is success, and the
    // restore mode and what it leaves alone are constants that belong on the
    // Restore control. Damaged checkpoints are the one thing to report.
    if !integrity || quarantined > 0 {
        section_header(ui, "Recovery store", None);
        if !integrity {
            property_row_problem(ui, "Integrity", "Review required", Some(Problem::Warning));
        }
        if quarantined > 0 {
            property_row_problem(
                ui,
                "Quarantined",
                &quarantined.to_string(),
                Some(Problem::Warning),
            );
        }
    }
}
